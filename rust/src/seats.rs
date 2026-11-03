//! Juror seats over decision models, and their envelope
//! readers.
//!
//! A seat names a juror label and a decision model route. The
//! readers in this module take one recorded envelope, as parsed
//! JSON, and return the recorded response the seat produced, or
//! None when the call failed. The readers are pure functions of
//! their inputs and make no call themselves. The obligations
//! this module carries are GJ-5 and GJ-6 in CONTRACT.md, in the
//! Round 2 section.

use num_bigint::BigInt;
use num_rational::Ratio;

use crate::json::{parse_units, JsonValue};
use crate::polls::Response;
use crate::pynum::Num;
use crate::pytext::PText;

/// Render a confidence figure as its shortest round trip text.
///
/// An integer confidence renders with no decimal point, and a
/// fractional one renders as the shortest decimal text that
/// parses back to the same value.
fn render_confidence(confidence: &Num) -> String {
    confidence.plain_text()
}

/// Return the envelope model string, or None when unreadable.
fn read_model(envelope: &JsonValue) -> Option<PText> {
    envelope.as_object()?;
    let model = match envelope.get("model") {
        Some(JsonValue::Str(text)) => text,
        _ => return None,
    };
    if model.is_empty() {
        return None;
    }
    if model.holds_unpaired_surrogate() {
        return None;
    }
    if model.holds_line_break() {
        return None;
    }
    Some(model.clone())
}

/// Build the recorded response from one decision, or None.
///
/// The decision must hold a choice string, a finite confidence
/// inside the range from zero to one, and a probabilities object
/// whose values are numbers. Anything else is a call the reader
/// cannot read, and the seat records nothing. A choice or a
/// model string holding a line break also fails the call, so a
/// response the reader produces always loads clean under GJ-1.
/// Whether the choice was offered is the tally business under
/// GJ-2, so the offered answers are not consulted here.
fn response_from_decision(
    juror: &PText,
    model: &PText,
    decision: Option<&JsonValue>,
) -> Option<Response> {
    let decision = decision?;
    decision.as_object()?;
    let choice = match decision.get("choice") {
        Some(JsonValue::Str(text)) => text.clone(),
        _ => return None,
    };
    if choice.holds_unpaired_surrogate() {
        return None;
    }
    if choice.holds_line_break() {
        return None;
    }
    let confidence = match decision.get("confidence") {
        Some(JsonValue::Num(num)) => num,
        _ => return None,
    };
    // The Python number test of GJ-5 takes an int or a float
    // and nothing else, so a figure held as an exact decimal,
    // the shape the makers exact reading produces, is not a
    // number to this reader and fails the call, as PORT-11
    // states. A float literal reaches this reader as a double
    // already, so a literal that underflows to 0.0 in the
    // Python reading stands here as the same double.
    if matches!(confidence, Num::Decimal(_)) {
        return None;
    }
    if !confidence.is_finite_number() {
        return None;
    }
    if confidence.is_below_int(0) || confidence.is_above_int(1) {
        return None;
    }
    let probabilities = decision.get("probabilities")?;
    let values = probabilities.as_object()?;
    for (_, value) in values {
        match value {
            JsonValue::Num(num) => {
                if matches!(num, Num::Decimal(_)) {
                    return None;
                }
                if !num.is_finite_number() {
                    return None;
                }
            }
            _ => return None,
        }
    }
    let base = format!(
        "{} chose {} with confidence {}",
        model.display(),
        choice.display(),
        render_confidence(confidence)
    );
    let half = Ratio::new(BigInt::from(1), BigInt::from(2));
    let abstained = confidence.to_fraction() < half;
    Some(Response {
        juror: juror.clone(),
        model: model.clone(),
        answer: choice,
        reason: if abstained {
            PText::from_text(&format!("{} and abstained", base))
        } else {
            PText::from_text(&base)
        },
        abstained,
    })
}

/// Return the recorded response from one Jev envelope, or None.
///
/// The envelope is the Typesafe System One response as parsed
/// JSON, holding the model string and an answers field whose
/// decision field holds the decision. The offered answers stand
/// in the Python signature because a seat is filled for a
/// question, and the reader does not consult them, for the
/// reason stated in response_from_decision.
pub fn response_from_jev_envelope(juror: &PText, envelope: &JsonValue) -> Option<Response> {
    let model = read_model(envelope)?;
    let answers_field = envelope.get("answers")?;
    answers_field.as_object()?;
    response_from_decision(juror, &model, answers_field.get("decision"))
}

/// Return the recorded response from one chat envelope, or
/// None.
///
/// The envelope is an OpenAI compatible chat completion as
/// parsed JSON, holding the model string and a choices field
/// whose first entry holds a message whose content is one JSON
/// object holding the decision. Content that does not parse as
/// one JSON object is a failed call.
pub fn response_from_chat_envelope(juror: &PText, envelope: &JsonValue) -> Option<Response> {
    let model = read_model(envelope)?;
    let choices = envelope.get("choices")?;
    let items = choices.as_array()?;
    if items.is_empty() {
        return None;
    }
    let first = &items[0];
    first.as_object()?;
    let message = first.get("message")?;
    message.as_object()?;
    let content = match message.get("content") {
        Some(JsonValue::Str(text)) => text,
        _ => return None,
    };
    let decision = parse_units(content.units()).ok()?;
    response_from_decision(juror, &model, Some(&decision))
}
