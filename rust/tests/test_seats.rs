//! Pins for GJ-5, the juror seat readers over decision models.
//!
//! Translated from tests/test_seats.py. Every failure shape
//! the contract names is pinned to a failed call, the
//! abstention threshold is pinned at its edges, and the reason
//! construction is pinned word for word. The recorded pilot
//! fixtures replay through the readers at the end. The Python
//! readers take the offered answers as a third argument the
//! readings never consult, so the port readers take none, and
//! the replay pins pass none.

mod shared;

use grandjury::json::JsonValue;
use grandjury::polls::Response;
use grandjury::pytext::PText;
use grandjury::seats::{response_from_chat_envelope, response_from_jev_envelope};
use shared::*;

fn decision(choice: &str, confidence: JsonValue, probabilities: JsonValue) -> JsonValue {
    jobj(vec![
        ("type", jstr("choice")),
        ("choice", jstr(choice)),
        ("confidence", confidence),
        ("probabilities", probabilities),
    ])
}

fn default_decision() -> JsonValue {
    decision(
        "red",
        jfloat(0.62),
        jobj(vec![("red", jfloat(0.62)), ("blue", jfloat(0.38))]),
    )
}

fn jev(decision_value: JsonValue) -> JsonValue {
    jev_model(decision_value, jstr("jev-test"))
}

fn jev_model(decision_value: JsonValue, model: JsonValue) -> JsonValue {
    jobj(vec![
        ("model", model),
        ("answers", jobj(vec![("decision", decision_value)])),
        (
            "usage",
            jobj(vec![("input_tokens", jint(10)), ("output_tokens", jint(5))]),
        ),
    ])
}

fn chat(content: &str) -> JsonValue {
    chat_model(content, "chat-test")
}

fn chat_model(content: &str, model: &str) -> JsonValue {
    jobj(vec![
        ("model", jstr(model)),
        (
            "choices",
            jarr(vec![jobj(vec![(
                "message",
                jobj(vec![("content", jstr(content))]),
            )])]),
        ),
        (
            "usage",
            jobj(vec![
                ("prompt_tokens", jint(10)),
                ("completion_tokens", jint(5)),
                ("cost", jfloat(0.0001)),
            ]),
        ),
    ])
}

fn chat_content(choice: &str, confidence: f64) -> String {
    chat_content_probs(
        choice,
        confidence,
        jobj(vec![("red", jfloat(0.25)), ("blue", jfloat(0.75))]),
    )
}

fn chat_content_probs(choice: &str, confidence: f64, probabilities: JsonValue) -> String {
    dumps_default(&jobj(vec![
        ("choice", jstr(choice)),
        ("confidence", jfloat(confidence)),
        ("probabilities", probabilities),
    ]))
}

fn read_jev(envelope: &JsonValue) -> Option<Response> {
    response_from_jev_envelope(&pt("jev-1"), envelope)
}

fn read_chat(envelope: &JsonValue) -> Option<Response> {
    response_from_chat_envelope(&pt("openai-1"), envelope)
}

fn expected(juror: &str, model: &str, answer: &str, reason: &str, abstained: bool) -> JsonValue {
    let mut pairs = vec![
        ("juror", jstr(juror)),
        ("model", jstr(model)),
        ("answer", jstr(answer)),
        ("reason", jstr(reason)),
    ];
    if abstained {
        pairs.push(("abstained", JsonValue::Bool(true)));
    }
    jobj(pairs)
}

fn assert_response(response: &Option<Response>, want: JsonValue) {
    let response = response.as_ref().expect("a recorded response");
    assert_json_eq(&response.to_json(), &want);
}

fn set_field(data: &mut JsonValue, key: &str, value: JsonValue) {
    if let JsonValue::Obj(pairs) = data {
        for (held, slot) in pairs.iter_mut() {
            if held.display() == key {
                *slot = value;
                return;
            }
        }
        pairs.push((pt(key), value));
    }
}

fn drop_field(data: &mut JsonValue, key: &str) {
    if let JsonValue::Obj(pairs) = data {
        pairs.retain(|(held, _)| held.display() != key);
    }
}

#[test]
fn a_jev_decision_at_high_confidence_records_a_plain_response() {
    assert_response(
        &read_jev(&jev(default_decision())),
        expected(
            "jev-1",
            "jev-test",
            "red",
            "jev-test chose red with confidence 0.62",
            false,
        ),
    );
}

#[test]
fn a_jev_decision_below_one_half_records_an_abstention() {
    let value = decision(
        "red",
        jfloat(0.4),
        jobj(vec![("red", jfloat(0.62)), ("blue", jfloat(0.38))]),
    );
    assert_response(
        &read_jev(&jev(value)),
        expected(
            "jev-1",
            "jev-test",
            "red",
            "jev-test chose red with confidence 0.4 and abstained",
            true,
        ),
    );
}

#[test]
fn a_jev_decision_at_exactly_one_half_is_not_an_abstention() {
    let value = decision(
        "red",
        jfloat(0.5),
        jobj(vec![("red", jfloat(0.62)), ("blue", jfloat(0.38))]),
    );
    let response = read_jev(&jev(value)).expect("a response");
    assert!(!response.abstained);
    assert_eq!(
        response.reason,
        pt("jev-test chose red with confidence 0.5")
    );
}

#[test]
fn a_jev_decision_just_below_one_half_abstains() {
    let value = decision(
        "red",
        jfloat(0.499999),
        jobj(vec![("red", jfloat(0.62)), ("blue", jfloat(0.38))]),
    );
    let response = read_jev(&jev(value)).expect("a response");
    assert!(response.abstained);
}

#[test]
fn an_integer_confidence_renders_with_no_decimal_point() {
    let value = decision(
        "red",
        jint(1),
        jobj(vec![("red", jfloat(0.62)), ("blue", jfloat(0.38))]),
    );
    let response = read_jev(&jev(value)).expect("a response");
    assert_eq!(response.reason, pt("jev-test chose red with confidence 1"));
}

#[test]
fn a_zero_confidence_abstains_and_renders_as_zero() {
    let value = decision(
        "red",
        jint(0),
        jobj(vec![("red", jfloat(0.62)), ("blue", jfloat(0.38))]),
    );
    let response = read_jev(&jev(value)).expect("a response");
    assert!(response.abstained);
    assert_eq!(
        response.reason,
        pt("jev-test chose red with confidence 0 and abstained")
    );
}

#[test]
fn a_fractional_confidence_renders_its_shortest_text() {
    let quarter = decision(
        "red",
        jfloat(0.25),
        jobj(vec![("red", jfloat(0.62)), ("blue", jfloat(0.38))]),
    );
    let response = read_jev(&jev(quarter)).expect("a response");
    assert_eq!(
        response.reason,
        pt("jev-test chose red with confidence 0.25 and abstained")
    );
    let seven = decision(
        "red",
        jfloat(0.7),
        jobj(vec![("red", jfloat(0.62)), ("blue", jfloat(0.38))]),
    );
    let response = read_jev(&jev(seven)).expect("a response");
    assert_eq!(
        response.reason,
        pt("jev-test chose red with confidence 0.7")
    );
}

#[test]
fn a_choice_that_was_not_offered_still_records() {
    let value = decision(
        "green",
        jfloat(0.9),
        jobj(vec![("red", jfloat(0.62)), ("blue", jfloat(0.38))]),
    );
    let response = read_jev(&jev(value)).expect("a response");
    assert_eq!(response.answer, pt("green"));
    assert!(!response.abstained);
}

#[test]
fn an_unoffered_choice_below_one_half_records_as_abstained() {
    let value = decision(
        "green",
        jfloat(0.3),
        jobj(vec![("red", jfloat(0.62)), ("blue", jfloat(0.38))]),
    );
    let response = read_jev(&jev(value)).expect("a response");
    assert_eq!(response.answer, pt("green"));
    assert!(response.abstained);
}

#[test]
fn a_jev_envelope_that_is_not_an_object_fails() {
    let envelope = jarr(vec![jstr("not"), jstr("an"), jstr("object")]);
    assert!(read_jev(&envelope).is_none());
}

#[test]
fn a_jev_envelope_without_a_model_fails() {
    let mut envelope = jev(default_decision());
    drop_field(&mut envelope, "model");
    assert!(read_jev(&envelope).is_none());
}

#[test]
fn a_jev_envelope_with_an_empty_model_fails() {
    assert!(read_jev(&jev_model(default_decision(), jstr(""))).is_none());
}

#[test]
fn a_jev_envelope_with_a_model_of_the_wrong_type_fails() {
    assert!(read_jev(&jev_model(default_decision(), jint(7))).is_none());
}

#[test]
fn a_jev_envelope_with_a_surrogate_in_the_model_fails() {
    let model = JsonValue::Str(PText::from_units(vec![0x6A, 0x65, 0x76, 0xD800, 0x78]));
    assert!(read_jev(&jev_model(default_decision(), model)).is_none());
}

#[test]
fn a_jev_envelope_without_an_answers_field_fails() {
    let mut envelope = jev(default_decision());
    drop_field(&mut envelope, "answers");
    assert!(read_jev(&envelope).is_none());
}

#[test]
fn a_jev_envelope_without_a_decision_fails() {
    let mut envelope = jev(default_decision());
    set_field(&mut envelope, "answers", jobj(vec![]));
    assert!(read_jev(&envelope).is_none());
}

#[test]
fn a_jev_decision_that_is_not_an_object_fails() {
    assert!(read_jev(&jev(jstr("red"))).is_none());
}

#[test]
fn a_jev_decision_without_a_choice_fails() {
    let mut value = default_decision();
    drop_field(&mut value, "choice");
    assert!(read_jev(&jev(value)).is_none());
}

#[test]
fn a_jev_decision_with_a_choice_of_the_wrong_type_fails() {
    let mut value = default_decision();
    set_field(&mut value, "choice", jint(7));
    assert!(read_jev(&jev(value)).is_none());
}

#[test]
fn a_jev_decision_with_a_surrogate_in_the_choice_fails() {
    let mut value = default_decision();
    let choice = JsonValue::Str(PText::from_units(vec![0x72, 0x65, 0xD800, 0x64]));
    set_field(&mut value, "choice", choice);
    assert!(read_jev(&jev(value)).is_none());
}

#[test]
fn a_jev_decision_without_a_confidence_fails() {
    let mut value = default_decision();
    drop_field(&mut value, "confidence");
    assert!(read_jev(&jev(value)).is_none());
}

#[test]
fn a_jev_decision_with_a_boolean_confidence_fails() {
    let mut value = default_decision();
    set_field(&mut value, "confidence", JsonValue::Bool(true));
    assert!(read_jev(&jev(value)).is_none());
}

#[test]
fn a_jev_decision_with_a_text_confidence_fails() {
    let mut value = default_decision();
    set_field(&mut value, "confidence", jstr("0.7"));
    assert!(read_jev(&jev(value)).is_none());
}

#[test]
fn a_jev_decision_with_a_nan_confidence_fails() {
    let mut value = default_decision();
    set_field(&mut value, "confidence", jfloat(f64::NAN));
    assert!(read_jev(&jev(value)).is_none());
}

#[test]
fn a_jev_decision_with_an_infinite_confidence_fails() {
    let mut value = default_decision();
    set_field(&mut value, "confidence", jfloat(f64::INFINITY));
    assert!(read_jev(&jev(value)).is_none());
}

#[test]
fn a_jev_decision_with_a_negative_confidence_fails() {
    let mut value = default_decision();
    set_field(&mut value, "confidence", jfloat(-0.1));
    assert!(read_jev(&jev(value)).is_none());
}

#[test]
fn a_jev_decision_with_a_confidence_above_one_fails() {
    let mut value = default_decision();
    set_field(&mut value, "confidence", jfloat(1.1));
    assert!(read_jev(&jev(value)).is_none());
}

#[test]
fn a_jev_decision_without_probabilities_fails() {
    let mut value = default_decision();
    drop_field(&mut value, "probabilities");
    assert!(read_jev(&jev(value)).is_none());
}

#[test]
fn a_jev_decision_with_probabilities_of_the_wrong_type_fails() {
    let value = decision("red", jfloat(0.62), jarr(vec![jfloat(0.62), jfloat(0.38)]));
    assert!(read_jev(&jev(value)).is_none());
}

#[test]
fn a_jev_decision_with_a_text_probability_fails() {
    let value = decision(
        "red",
        jfloat(0.62),
        jobj(vec![("red", jstr("high")), ("blue", jfloat(0.38))]),
    );
    assert!(read_jev(&jev(value)).is_none());
}

#[test]
fn a_jev_decision_with_a_boolean_probability_fails() {
    let value = decision(
        "red",
        jfloat(0.62),
        jobj(vec![("red", JsonValue::Bool(true)), ("blue", jfloat(0.38))]),
    );
    assert!(read_jev(&jev(value)).is_none());
}

#[test]
fn a_jev_decision_with_a_nan_probability_fails() {
    let value = decision(
        "red",
        jfloat(0.62),
        jobj(vec![("red", jfloat(f64::NAN)), ("blue", jfloat(0.38))]),
    );
    assert!(read_jev(&jev(value)).is_none());
}

#[test]
fn a_jev_decision_with_an_infinite_probability_fails() {
    let value = decision(
        "red",
        jfloat(0.62),
        jobj(vec![("red", jfloat(f64::INFINITY)), ("blue", jfloat(0.38))]),
    );
    assert!(read_jev(&jev(value)).is_none());
}

#[test]
fn a_chat_decision_with_a_nan_probability_fails() {
    let content = chat_content_probs(
        "blue",
        0.75,
        jobj(vec![("red", jfloat(f64::NAN)), ("blue", jfloat(0.75))]),
    );
    assert!(read_chat(&chat(&content)).is_none());
}

#[test]
fn a_chat_decision_records_a_plain_response() {
    assert_response(
        &read_chat(&chat(&chat_content("blue", 0.75))),
        expected(
            "openai-1",
            "chat-test",
            "blue",
            "chat-test chose blue with confidence 0.75",
            false,
        ),
    );
}

#[test]
fn a_chat_decision_below_one_half_records_an_abstention() {
    let response = read_chat(&chat(&chat_content("blue", 0.25))).expect("a response");
    assert!(response.abstained);
    assert_eq!(
        response.reason,
        pt("chat-test chose blue with confidence 0.25 and abstained")
    );
}

#[test]
fn a_chat_envelope_without_choices_fails() {
    let mut envelope = chat(&chat_content("blue", 0.75));
    drop_field(&mut envelope, "choices");
    assert!(read_chat(&envelope).is_none());
}

#[test]
fn a_chat_envelope_with_empty_choices_fails() {
    let mut envelope = chat(&chat_content("blue", 0.75));
    set_field(&mut envelope, "choices", jarr(vec![]));
    assert!(read_chat(&envelope).is_none());
}

#[test]
fn a_chat_envelope_without_a_message_fails() {
    let mut envelope = chat(&chat_content("blue", 0.75));
    set_field(&mut envelope, "choices", jarr(vec![jobj(vec![])]));
    assert!(read_chat(&envelope).is_none());
}

#[test]
fn a_chat_envelope_without_message_content_fails() {
    let mut envelope = chat(&chat_content("blue", 0.75));
    set_field(
        &mut envelope,
        "choices",
        jarr(vec![jobj(vec![("message", jobj(vec![]))])]),
    );
    assert!(read_chat(&envelope).is_none());
}

#[test]
fn a_chat_envelope_with_content_of_the_wrong_type_fails() {
    let mut envelope = chat(&chat_content("blue", 0.75));
    set_field(
        &mut envelope,
        "choices",
        jarr(vec![jobj(vec![(
            "message",
            jobj(vec![("content", jint(42))]),
        )])]),
    );
    assert!(read_chat(&envelope).is_none());
}

#[test]
fn chat_content_that_is_not_json_fails() {
    assert!(read_chat(&chat("the answer is blue")).is_none());
}

#[test]
fn chat_content_that_is_a_json_array_fails() {
    assert!(read_chat(&chat("[1, 2]")).is_none());
}

#[test]
fn chat_content_holding_an_empty_object_fails() {
    assert!(read_chat(&chat("{}")).is_none());
}

#[test]
fn chat_content_with_a_text_confidence_fails() {
    let content = dumps_default(&jobj(vec![
        ("choice", jstr("blue")),
        ("confidence", jstr("high")),
        (
            "probabilities",
            jobj(vec![("red", jfloat(0.25)), ("blue", jfloat(0.75))]),
        ),
    ]));
    assert!(read_chat(&chat(&content)).is_none());
}

#[test]
fn a_jev_decision_with_a_newline_in_the_choice_fails() {
    let value = decision(
        "red\nblue",
        jfloat(0.62),
        jobj(vec![("red", jfloat(0.62)), ("blue", jfloat(0.38))]),
    );
    assert!(read_jev(&jev(value)).is_none());
}

#[test]
fn a_jev_decision_with_a_carriage_return_in_the_choice_fails() {
    let value = decision(
        "red\rblue",
        jfloat(0.62),
        jobj(vec![("red", jfloat(0.62)), ("blue", jfloat(0.38))]),
    );
    assert!(read_jev(&jev(value)).is_none());
}

#[test]
fn a_jev_envelope_with_a_newline_in_the_model_fails() {
    assert!(read_jev(&jev_model(default_decision(), jstr("jev\ntest"))).is_none());
}

#[test]
fn a_chat_decision_with_a_newline_in_the_choice_fails() {
    assert!(read_chat(&chat(&chat_content("blue\ngreen", 0.75))).is_none());
}

#[test]
fn a_chat_decision_with_a_carriage_return_in_the_choice_fails() {
    assert!(read_chat(&chat(&chat_content("blue\rgreen", 0.75))).is_none());
}

#[test]
fn a_chat_envelope_with_a_newline_in_the_model_fails() {
    let envelope = chat_model(&chat_content("blue", 0.75), "chat\ntest");
    assert!(read_chat(&envelope).is_none());
}

fn recorded_envelope(name: &str) -> JsonValue {
    read_json(&examples().join("pilot").join("envelopes").join(name))
}

#[test]
fn the_recorded_jev_fixture_replays_to_its_response() {
    let envelope = recorded_envelope("q03-jev-1.json");
    let response = response_from_jev_envelope(&pt("jev-1"), &envelope);
    assert_response(
        &response,
        expected(
            "jev-1",
            "jev-1.13.0",
            "both cards and cash",
            "jev-1.13.0 chose both cards and cash with confidence 0.89",
            false,
        ),
    );
}

#[test]
fn the_recorded_jev_abstention_fixture_replays() {
    let envelope = recorded_envelope("q01-jev-1.json");
    let response = response_from_jev_envelope(&pt("jev-1"), &envelope);
    assert_response(
        &response,
        expected(
            "jev-1",
            "jev-1.13.0",
            "open on Sundays",
            "jev-1.13.0 chose open on Sundays with confidence 0.02 and abstained",
            true,
        ),
    );
}

#[test]
fn the_recorded_chat_fixture_replays_to_its_response() {
    let envelope = recorded_envelope("q01-openai-1.json");
    let response = response_from_chat_envelope(&pt("openai-1"), &envelope);
    assert_response(
        &response,
        expected(
            "openai-1",
            "openai/gpt-6-luna",
            "open on Sundays",
            "openai/gpt-6-luna chose open on Sundays with confidence 0.62",
            false,
        ),
    );
}

#[test]
fn the_recorded_chat_abstention_fixture_replays() {
    let envelope = recorded_envelope("q12-openai-2.json");
    let response = response_from_chat_envelope(&pt("openai-2"), &envelope);
    assert_response(
        &response,
        expected(
            "openai-2",
            "openai/gpt-6-luna",
            "move into the hall",
            "openai/gpt-6-luna chose move into the hall with confidence 0.42 and abstained",
            true,
        ),
    );
}

#[test]
fn a_float_literal_that_underflows_reads_as_the_double_zero() {
    let text = "{\"model\": \"jev-test\", \"answers\": {\"decision\": {\"choice\": \"red\", \"confidence\": 1e-3000000, \"probabilities\": {\"red\": 1.0}}}, \"usage\": {\"input_tokens\": 1, \"output_tokens\": 1}}";
    let envelope = grandjury::json::parse(text).expect("an envelope");
    let response = read_jev(&envelope);
    assert_response(
        &response,
        expected(
            "jev-1",
            "jev-test",
            "red",
            "jev-test chose red with confidence 0.0 and abstained",
            true,
        ),
    );
}

#[test]
fn a_figure_held_as_an_exact_decimal_fails_the_call() {
    for literal in ["1e-3000000", "0.25"] {
        let text = format!(
            "{{\"model\": \"jev-test\", \"answers\": {{\"decision\": {{\"choice\": \"red\", \"confidence\": {}, \"probabilities\": {{\"red\": 1}}}}}}, \"usage\": {{\"input_tokens\": 1, \"output_tokens\": 1}}}}",
            literal
        );
        let envelope = grandjury::json::parse_exact(&text).expect("an envelope");
        assert!(read_jev(&envelope).is_none(), "{}", literal);
    }
    let text = "{\"model\": \"jev-test\", \"answers\": {\"decision\": {\"choice\": \"red\", \"confidence\": 1, \"probabilities\": {\"red\": 0.5}}}, \"usage\": {\"input_tokens\": 1, \"output_tokens\": 1}}";
    let envelope = grandjury::json::parse_exact(text).expect("an envelope");
    assert!(read_jev(&envelope).is_none());
}
