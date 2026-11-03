//! Polls, their loader, their tally, the stadium view, and the
//! cost counter for grandjury.
//!
//! Every function in this module is a pure function of its
//! inputs, and the loaders never raise on malformed content.
//! The obligations this module carries are GJ-1 to GJ-4 in
//! CONTRACT.md, in the full form section, and the Round 2
//! extension of the recorded poll shape with its abstained
//! response kind in the Round 2 section. The counter runs on
//! recorded polls and makes no live call.

use std::path::Path;

use num_bigint::BigInt;
use num_rational::Ratio;

use crate::json::{parse, JsonValue};
use crate::pydecimal::{PyDecimal, Rounding};
use crate::pytext::PText;

pub const MAX_BAR: usize = 40;

/// One recorded response of a poll, as the loader holds it.
#[derive(Clone, Debug, PartialEq)]
pub struct Response {
    pub juror: PText,
    pub model: PText,
    pub answer: PText,
    pub reason: PText,
    pub abstained: bool,
}

impl Response {
    /// The response as a JSON value, in the field order the
    /// Python record holds, the abstained field standing only
    /// on an abstained response.
    pub fn to_json(&self) -> JsonValue {
        let mut pairs = vec![
            (
                PText::from_text("juror"),
                JsonValue::Str(self.juror.clone()),
            ),
            (
                PText::from_text("model"),
                JsonValue::Str(self.model.clone()),
            ),
            (
                PText::from_text("answer"),
                JsonValue::Str(self.answer.clone()),
            ),
            (
                PText::from_text("reason"),
                JsonValue::Str(self.reason.clone()),
            ),
        ];
        if self.abstained {
            pairs.push((PText::from_text("abstained"), JsonValue::Bool(true)));
        }
        JsonValue::object(pairs)
    }
}

/// A valid poll, as the loader holds it.
#[derive(Clone, Debug, PartialEq)]
pub struct Poll {
    pub question: PText,
    pub answers: Vec<PText>,
    pub responses: Vec<Response>,
}

impl Poll {
    /// The poll as a JSON value, in the field order the Python
    /// record holds.
    pub fn to_json(&self) -> JsonValue {
        JsonValue::object(vec![
            (
                PText::from_text("question"),
                JsonValue::Str(self.question.clone()),
            ),
            (
                PText::from_text("answers"),
                JsonValue::Arr(
                    self.answers
                        .iter()
                        .map(|answer| JsonValue::Str(answer.clone()))
                        .collect(),
                ),
            ),
            (
                PText::from_text("responses"),
                JsonValue::Arr(self.responses.iter().map(|r| r.to_json()).collect()),
            ),
        ])
    }
}

/// Validate one text field of a response and return its value,
/// or None when the field drew a defect.
fn check_response_text(
    entry: &JsonValue,
    index: usize,
    name: &str,
    defects: &mut Vec<String>,
    allow_empty: bool,
) -> Option<PText> {
    let value = match entry.get(name) {
        None => {
            defects.push(format!("response {} is missing field {}", index, name));
            return None;
        }
        Some(value) => value,
    };
    let text = match value {
        JsonValue::Str(text) => text,
        _ => {
            defects.push(format!(
                "response {} field {} has the wrong type",
                index, name
            ));
            return None;
        }
    };
    if text.is_empty() && !allow_empty {
        defects.push(format!("response {} field {} is empty", index, name));
        return None;
    }
    if text.holds_unpaired_surrogate() {
        defects.push(format!(
            "response {} field {} holds an unpaired surrogate",
            index, name
        ));
        return None;
    }
    if text.holds_line_break() {
        defects.push(format!("response {} field {} holds a newline", index, name));
        return None;
    }
    Some(text.clone())
}

/// Validate one response entry and return its clean record and
/// its juror. The record is None when the entry drew any
/// defect, and the juror stands whenever the juror field itself
/// read clean, as the duplicate reading needs it either way.
fn load_response(
    entry: &JsonValue,
    index: usize,
    defects: &mut Vec<String>,
) -> (Option<Response>, Option<PText>) {
    let before = defects.len();
    if entry.as_object().is_none() {
        defects.push(format!("response {} is not an object", index));
        return (None, None);
    }
    let juror = check_response_text(entry, index, "juror", defects, false);
    let model = check_response_text(entry, index, "model", defects, false);
    let answer = check_response_text(entry, index, "answer", defects, true);
    let reason = check_response_text(entry, index, "reason", defects, false);
    let mut abstained = false;
    if let Some(flag) = entry.get("abstained") {
        if matches!(flag, JsonValue::Bool(true)) {
            abstained = true;
        } else {
            defects.push(format!("response {} field abstained is not true", index));
        }
    }
    if defects.len() != before {
        return (None, juror);
    }
    (
        Some(Response {
            juror: juror.clone().expect("a clean juror"),
            model: model.expect("a clean model"),
            answer: answer.expect("a clean answer"),
            reason: reason.expect("a clean reason"),
            abstained,
        }),
        juror,
    )
}

/// Parse poll text into a poll, naming every defect found.
///
/// Returns a pair of the poll and the defect list. A malformed
/// input returns no poll, and this function never fails on bad
/// content. A response naming an answer that was not offered is
/// spoiled, which is a tally outcome and not a defect here.
pub fn load_poll_text(text: &str) -> (Option<Poll>, Vec<String>) {
    let data = match parse(text) {
        Ok(data) => data,
        Err(_) => return (None, vec!["input is not valid JSON".to_string()]),
    };
    if data.as_object().is_none() {
        return (None, vec!["input is not a JSON object".to_string()]);
    }
    let mut defects: Vec<String> = Vec::new();
    let mut question: Option<PText> = None;
    match data.get("question") {
        None => defects.push("missing field question".to_string()),
        Some(value) => match value {
            JsonValue::Str(text) => {
                if text.is_empty() {
                    defects.push("field question is empty".to_string());
                } else if text.holds_unpaired_surrogate() {
                    defects.push("field question holds an unpaired surrogate".to_string());
                } else if text.holds_line_break() {
                    defects.push("field question holds a newline".to_string());
                } else {
                    question = Some(text.clone());
                }
            }
            _ => defects.push("field question has the wrong type".to_string()),
        },
    }
    let mut answers: Option<Vec<PText>> = None;
    match data.get("answers") {
        None => defects.push("missing field answers".to_string()),
        Some(value) => match value.as_array() {
            None => defects.push("field answers has the wrong type".to_string()),
            Some(entries) => {
                if entries.len() < 2 {
                    defects.push("field answers holds fewer than two answers".to_string());
                }
                let mut clean_answers: Vec<PText> = Vec::new();
                let mut seen_answers: Vec<PText> = Vec::new();
                for (index, entry) in entries.iter().enumerate() {
                    let text = match entry {
                        JsonValue::Str(text) => text,
                        _ => {
                            defects.push(format!("answer {} has the wrong type", index));
                            continue;
                        }
                    };
                    if text.is_empty() {
                        defects.push(format!("answer {} is empty", index));
                        continue;
                    }
                    if text.holds_unpaired_surrogate() {
                        defects.push(format!("answer {} holds an unpaired surrogate", index));
                        continue;
                    }
                    if text.holds_line_break() {
                        defects.push(format!("answer {} holds a newline", index));
                        continue;
                    }
                    if seen_answers.contains(text) {
                        defects.push(format!("duplicate answer {}", text.display()));
                        continue;
                    }
                    seen_answers.push(text.clone());
                    clean_answers.push(text.clone());
                }
                answers = Some(clean_answers);
            }
        },
    }
    let mut responses: Option<Vec<Response>> = None;
    match data.get("responses") {
        None => defects.push("missing field responses".to_string()),
        Some(value) => match value.as_array() {
            None => defects.push("field responses has the wrong type".to_string()),
            Some(entries) => {
                let mut clean_responses: Vec<Response> = Vec::new();
                let mut seen_jurors: Vec<PText> = Vec::new();
                for (index, entry) in entries.iter().enumerate() {
                    let (clean, juror) = load_response(entry, index, &mut defects);
                    if let Some(juror) = &juror {
                        if seen_jurors.contains(juror) {
                            defects.push(format!("duplicate juror {}", juror.display()));
                        } else {
                            seen_jurors.push(juror.clone());
                        }
                    }
                    if let Some(clean) = clean {
                        clean_responses.push(clean);
                    }
                }
                responses = Some(clean_responses);
            }
        },
    }
    if !defects.is_empty() {
        return (None, defects);
    }
    (
        Some(Poll {
            question: question.expect("a clean question"),
            answers: answers.expect("clean answers"),
            responses: responses.expect("clean responses"),
        }),
        Vec::new(),
    )
}

/// Read a poll from disk, with the same contract as text.
pub fn load_poll_file(path: &Path) -> (Option<Poll>, Vec<String>) {
    let text = match read_text_file(path) {
        Some(text) => text,
        None => return (None, vec!["file cannot be read".to_string()]),
    };
    load_poll_text(&text)
}

/// Read a file as UTF-8 text the way the Python path reading
/// reads it, or None where the read fails or the bytes are not
/// UTF-8. The Python reading also folds the newline spellings
/// of the file to one, which no reading in this repo can
/// observe, because a literal line break inside a JSON string
/// is refused by the JSON reading under every spelling, and
/// the spellings agree as whitespace outside one.
pub fn read_text_file(path: &Path) -> Option<String> {
    let bytes = std::fs::read(path).ok()?;
    String::from_utf8(bytes).ok()
}

/// The tally of a valid poll as data.
pub struct Tally {
    pub question: PText,
    pub responses: usize,
    pub valid: usize,
    pub spoiled: usize,
    pub abstained: usize,
    pub rows: Vec<(PText, usize, Ratio<BigInt>)>,
}

/// Return the tally of a valid poll as data.
///
/// The tally is a pure function of the poll. Each row holds one
/// offered answer, in offered order, with the count of valid
/// responses choosing it and its share held as an exact fraction
/// of the valid responses. Spoiled responses are counted apart,
/// and abstained responses are counted apart from both, as their
/// own kind under the Round 2 extension.
pub fn tally_poll(poll: &Poll) -> Tally {
    let mut counts: Vec<usize> = vec![0; poll.answers.len()];
    let mut spoiled = 0usize;
    let mut abstained = 0usize;
    for response in &poll.responses {
        if response.abstained {
            abstained += 1;
        } else if let Some(slot) = poll.answers.iter().position(|a| *a == response.answer) {
            counts[slot] += 1;
        } else {
            spoiled += 1;
        }
    }
    let valid = poll.responses.len() - spoiled - abstained;
    let mut rows = Vec::new();
    for (index, answer) in poll.answers.iter().enumerate() {
        let share = if valid > 0 {
            Ratio::new(BigInt::from(counts[index]), BigInt::from(valid))
        } else {
            Ratio::from(BigInt::from(0))
        };
        rows.push((answer.clone(), counts[index], share));
    }
    Tally {
        question: poll.question.clone(),
        responses: poll.responses.len(),
        valid,
        spoiled,
        abstained,
        rows,
    }
}

/// Render an exact fraction as a percentage with one decimal.
///
/// The arithmetic is exact integer work, so no float ever rounds
/// the figure the wrong way. Halves round up.
pub fn percent(share: &Ratio<BigInt>) -> String {
    let numerator = share.numer();
    let denominator = share.denom();
    let tenths = (BigInt::from(2) * numerator * BigInt::from(1000) + denominator)
        / (BigInt::from(2) * denominator);
    format!("{}.{}", &tenths / 10, &tenths % 10)
}

/// Render a fraction the way the Python fraction text renders
/// it, in lowest terms, a denominator of one rendering as the
/// numerator alone and a zero share rendering as zero.
pub fn fraction_text(share: &Ratio<BigInt>) -> String {
    if *share.denom() == BigInt::from(1) {
        share.numer().to_string()
    } else {
        format!("{}/{}", share.numer(), share.denom())
    }
}

/// Return the tally of a valid poll as lines.
pub fn render_tally_lines(poll: &Poll) -> Vec<String> {
    let tally = tally_poll(poll);
    let mut lines = vec![
        format!("question, {}", tally.question.display()),
        format!("responses, {}", tally.responses),
        format!("valid, {}", tally.valid),
        format!("spoiled, {}", tally.spoiled),
    ];
    if tally.abstained > 0 {
        lines.push(format!("abstained, {}", tally.abstained));
    }
    for (answer, count, share) in &tally.rows {
        lines.push(format!(
            "{}, {}, {}%",
            answer.display(),
            count,
            percent(share)
        ));
    }
    lines
}

/// Return the stadium view of a valid poll as lines.
///
/// The view is a pure function of the poll, so the same poll
/// always renders byte identical lines. The wall of reasons
/// groups the valid responses by the answer they chose, in
/// offered order, and a spoiled section closes the view when
/// spoiled responses exist.
pub fn render_stadium(poll: &Poll) -> Vec<String> {
    let tally = tally_poll(poll);
    let mut lines = vec![
        format!("question, {}", tally.question.display()),
        format!("responses, {}", tally.responses),
    ];
    let largest = tally.rows.iter().map(|row| row.1).max().unwrap_or(0);
    for (answer, count, share) in &tally.rows {
        let bar = match (count * MAX_BAR).checked_div(largest) {
            Some(width) => "#".repeat(width),
            None => String::new(),
        };
        lines.push(format!(
            "{} |{}| {}, {}%",
            answer.display(),
            bar,
            count,
            percent(share)
        ));
    }
    let mut by_answer: Vec<Vec<&Response>> = vec![Vec::new(); poll.answers.len()];
    let mut spoiled: Vec<&Response> = Vec::new();
    let mut abstained: Vec<&Response> = Vec::new();
    for response in &poll.responses {
        if response.abstained {
            abstained.push(response);
        } else if let Some(slot) = poll.answers.iter().position(|a| *a == response.answer) {
            by_answer[slot].push(response);
        } else {
            spoiled.push(response);
        }
    }
    lines.push("wall of reasons".to_string());
    for (index, answer) in poll.answers.iter().enumerate() {
        lines.push(answer.display());
        for response in &by_answer[index] {
            lines.push(format!(
                "  {}, {}",
                response.juror.display(),
                response.reason.display()
            ));
        }
    }
    if !abstained.is_empty() {
        lines.push(format!("abstained, {}", abstained.len()));
        for response in &abstained {
            lines.push(format!(
                "  {}, {}",
                response.juror.display(),
                response.reason.display()
            ));
        }
    }
    if !spoiled.is_empty() {
        lines.push(format!("spoiled, {}", spoiled.len()));
        for response in &spoiled {
            lines.push(format!(
                "  {}, {}",
                response.juror.display(),
                response.answer.display()
            ));
        }
    }
    lines
}

/// A price table, mapping each model label to its price as an
/// exact decimal, in the order the table file names them.
#[derive(Clone, Debug)]
pub struct PriceTable {
    entries: Vec<(PText, PyDecimal)>,
}

impl PriceTable {
    pub fn new(entries: Vec<(PText, PyDecimal)>) -> PriceTable {
        PriceTable { entries }
    }

    pub fn get(&self, model: &PText) -> Option<&PyDecimal> {
        self.entries
            .iter()
            .find(|(held, _)| held == model)
            .map(|(_, price)| price)
    }

    pub fn models(&self) -> Vec<PText> {
        self.entries
            .iter()
            .map(|(model, _)| model.clone())
            .collect()
    }
}

/// Return whether a text matches the price grammar of GJ-4,
/// the whole text under the one exact spelling, an optional
/// minus sign, digits with an optional fraction or a bare
/// fraction, and an optional exponent, ASCII digits only, with
/// no whitespace, no underscores, and no leading plus.
fn price_grammar_matches(text: &PText) -> bool {
    let units = text.units();
    let mut index = 0;
    if index < units.len() && units[index] == 0x2D {
        index += 1;
    }
    let digit = |unit: u16| (0x30..=0x39).contains(&unit);
    let whole_start = index;
    while index < units.len() && digit(units[index]) {
        index += 1;
    }
    let whole = index - whole_start;
    let mut fraction = 0usize;
    if index < units.len() && units[index] == 0x2E {
        index += 1;
        let fraction_start = index;
        while index < units.len() && digit(units[index]) {
            index += 1;
        }
        fraction = index - fraction_start;
        if fraction == 0 {
            return false;
        }
    }
    if whole == 0 && fraction == 0 {
        return false;
    }
    if index < units.len() && (units[index] == 0x65 || units[index] == 0x45) {
        index += 1;
        if index < units.len() && (units[index] == 0x2B || units[index] == 0x2D) {
            index += 1;
        }
        let exponent_start = index;
        while index < units.len() && digit(units[index]) {
            index += 1;
        }
        if index == exponent_start {
            return false;
        }
    }
    index == units.len()
}

/// Parse price table text into a price table, naming defects.
///
/// Returns a pair of the table and the defect list. A malformed
/// input returns no table, and this function never fails on bad
/// content.
pub fn load_prices_text(text: &str) -> (Option<PriceTable>, Vec<String>) {
    let data = match parse(text) {
        Ok(data) => data,
        Err(_) => return (None, vec!["input is not valid JSON".to_string()]),
    };
    if data.as_object().is_none() {
        return (None, vec!["input is not a JSON object".to_string()]);
    }
    let mut defects: Vec<String> = Vec::new();
    let raw = match data.get("prices") {
        None => {
            defects.push("missing field prices".to_string());
            return (None, defects);
        }
        Some(raw) => raw,
    };
    let raw_pairs = match raw.as_object() {
        Some(raw_pairs) => raw_pairs,
        None => {
            defects.push("field prices has the wrong type".to_string());
            return (None, defects);
        }
    };
    let mut entries: Vec<(PText, PyDecimal)> = Vec::new();
    for (model, value) in raw_pairs {
        if model.holds_unpaired_surrogate() {
            defects.push("a model label holds an unpaired surrogate".to_string());
            continue;
        }
        if model.holds_line_break() {
            defects.push("a model label holds a newline".to_string());
            continue;
        }
        let text = match value {
            JsonValue::Str(text) => text,
            _ => {
                defects.push(format!("price for {} has the wrong type", model.display()));
                continue;
            }
        };
        if text.holds_unpaired_surrogate() {
            defects.push(format!(
                "price for {} holds an unpaired surrogate",
                model.display()
            ));
            continue;
        }
        if !price_grammar_matches(text) {
            defects.push(format!(
                "price for {} is not a decimal string",
                model.display()
            ));
            continue;
        }
        let literal: String = text
            .units()
            .iter()
            .map(|unit| *unit as u8 as char)
            .collect();
        let price = match PyDecimal::parse_literal(&literal) {
            Some(price) => price,
            None => {
                defects.push(format!(
                    "price for {} is not a decimal string",
                    model.display()
                ));
                continue;
            }
        };
        if price.is_negative() {
            defects.push(format!("price for {} is negative", model.display()));
            continue;
        }
        entries.push((model.clone(), price));
    }
    if !defects.is_empty() {
        return (None, defects);
    }
    (Some(PriceTable::new(entries)), Vec::new())
}

/// Read a price table from disk, with the same contract as text.
pub fn load_prices_file(path: &Path) -> (Option<PriceTable>, Vec<String>) {
    let text = match read_text_file(path) {
        Some(text) => text,
        None => return (None, vec!["file cannot be read".to_string()]),
    };
    load_prices_text(&text)
}

/// The largest working precision the Python decimal context
/// accepts, its MAX_PREC bound. PORT-11 in CONTRACT.md
/// states the failure that stands past it.
pub const MAX_WORKING_PRECISION: i64 = 999999999999999999;

/// Size a decimal precision that covers the given prices
/// exactly.
///
/// The precision takes in the largest price magnitude, the
/// least significant place among the prices, the count of
/// prices being summed (a sum can carry that many extra
/// digits), and the six decimal places of the rendered total,
/// so a sum computed at this precision never rounds and its
/// six place rendering never overflows it. FIX-GJ-1 in
/// CONTRACT.md states the rule.
fn working_precision(priced_prices: &[PyDecimal]) -> i64 {
    let largest = priced_prices
        .iter()
        .map(|price| price.adjusted())
        .max()
        .expect("a priced list");
    let least_place = priced_prices
        .iter()
        .map(|price| price.exponent())
        .min()
        .expect("a priced list");
    let carry = priced_prices.len().to_string().len() as i64;
    let span = largest - least_place + 1 + carry;
    let rendering = largest + carry + 7;
    std::cmp::max(28, std::cmp::max(span, rendering)) + 2
}

/// The storage a binary coefficient of the given digit width
/// can need, at four bits for each decimal digit. Four bits
/// hold a decimal digit with room to spare, so no binary
/// coefficient of that width needs more storage than this
/// figure names, and the figure grows in proportion to the
/// width and against no stated cap.
fn coefficient_storage_bytes(digits: i64) -> Option<usize> {
    if digits <= 0 {
        return Some(0);
    }
    let bytes = ((digits as u128) * 4).div_ceil(8);
    usize::try_from(bytes).ok()
}

/// Attempt to reserve the storage a coefficient of the given
/// digit width needs, in one fallible reservation that either
/// stands whole or fails whole. The reservation the caller
/// holds is the boundary the PORT-11 amendment in CONTRACT.md
/// states, so a failed reservation returns None here and the
/// cost reading takes the failure path PORT-11 names, before
/// any addition has had the chance to grind on a coefficient
/// the machine cannot materialize.
fn reserve_coefficient_storage(digits: i64) -> Option<Vec<u8>> {
    let bytes = coefficient_storage_bytes(digits)?;
    let mut storage: Vec<u8> = Vec::new();
    storage.try_reserve_exact(bytes).ok()?;
    Some(storage)
}

/// Return the cost report for a valid poll as lines, or None
/// where the Python reading fails inside itself.
///
/// Every recorded response is priced, spoiled responses
/// included. A response whose model stands in the table adds
/// its price to the priced total once. A model missing from
/// the table is named with its response count, in the order
/// such models first appear in the responses, and is never
/// silently priced at zero. Sums are exact decimals computed
/// at a precision sized to the prices themselves, and the
/// total renders in dollars with six decimal places, halves
/// rounding up. The sized precision past the Python context
/// bound MAX_PREC is the failure PORT-11 states, because the
/// Python counter raises when it sets that precision, before
/// any report line prints, so the reading returns None here
/// and the command surface reads that as its exit status 1
/// with no report lines. Inside that bound the counter first
/// attempts the fallible coefficient reservation the PORT-11
/// amendment states, sized from that same precision, and a
/// failed reservation returns None the same way, because the
/// Python library refuses the allocation its sum demands in
/// that region and fails the command before any line prints.
pub fn cost_lines(poll: &Poll, prices: &PriceTable) -> Option<Vec<String>> {
    let mut priced_prices: Vec<PyDecimal> = Vec::new();
    let mut unpriced_counts: Vec<(PText, usize)> = Vec::new();
    for response in &poll.responses {
        match prices.get(&response.model) {
            Some(price) => priced_prices.push(price.clone()),
            None => {
                if let Some(slot) = unpriced_counts
                    .iter()
                    .position(|(held, _)| *held == response.model)
                {
                    unpriced_counts[slot].1 += 1;
                } else {
                    unpriced_counts.push((response.model.clone(), 1));
                }
            }
        }
    }
    let priced = priced_prices.len();
    let unpriced: usize = unpriced_counts.iter().map(|(_, count)| count).sum();
    let precision = if priced_prices.is_empty() {
        28
    } else {
        working_precision(&priced_prices)
    };
    if precision > MAX_WORKING_PRECISION {
        return None;
    }
    // The reservation stands while the sum runs, and its
    // failure is the materialization failure the PORT-11
    // amendment names, taken here before any addition runs.
    let _coefficient_storage = reserve_coefficient_storage(precision)?;
    let prec = usize::try_from(precision).unwrap_or(usize::MAX);
    let mut total = PyDecimal::zero();
    for price in &priced_prices {
        total = total.added_ctx(price, prec, Rounding::HalfEven);
    }
    let rendered = total.quantized_six(prec)?;
    let mut lines = vec![
        format!("question, {}", poll.question.display()),
        format!("responses, {}", poll.responses.len()),
        format!(
            "priced, {} responses, {} dollars",
            priced,
            rendered.render()
        ),
        format!("unpriced, {} responses", unpriced),
    ];
    for (model, count) in &unpriced_counts {
        lines.push(format!(
            "unpriced model, {}, {} responses",
            model.display(),
            count
        ));
    }
    Some(lines)
}
