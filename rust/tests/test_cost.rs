//! Pins for GJ-4, the price table loader and the cost counter.
//!
//! Translated from tests/test_cost.py. The counter arithmetic
//! is exact decimal work, so the pins carry prices whose
//! binary float forms drift, and the unpriced model report is
//! pinned line by line.

mod shared;

use grandjury::json::JsonValue;
use grandjury::polls::{load_poll_text, load_prices_file, load_prices_text, Poll, PriceTable};
use grandjury::pydecimal::PyDecimal;
use shared::*;

fn response(juror: &str, answer: &str) -> JsonValue {
    response_model(juror, answer, "tiny-a")
}

fn response_model(juror: &str, answer: &str, model: &str) -> JsonValue {
    jobj(vec![
        ("juror", jstr(juror)),
        ("model", jstr(model)),
        ("answer", jstr(answer)),
        ("reason", jstr("a reason")),
    ])
}

fn abstained_response(juror: &str, answer: &str, model: &str) -> JsonValue {
    let mut entry = response_model(juror, answer, model);
    if let JsonValue::Obj(pairs) = &mut entry {
        pairs.push((pt("abstained"), JsonValue::Bool(true)));
    }
    entry
}

fn poll_of(responses: Vec<JsonValue>) -> JsonValue {
    jobj(vec![
        ("question", jstr("q")),
        ("answers", jarr(vec![jstr("red"), jstr("blue")])),
        ("responses", jarr(responses)),
    ])
}

fn load_poll(data: &JsonValue) -> Poll {
    let (poll, defects) = load_poll_text(&dumps_default(data));
    assert_eq!(defects, Vec::<String>::new());
    poll.expect("a poll")
}

fn load_prices(data: &JsonValue) -> PriceTable {
    let (table, defects) = load_prices_text(&dumps_default(data));
    assert_eq!(defects, Vec::<String>::new());
    table.expect("a table")
}

fn prices_obj(pairs: Vec<(&str, JsonValue)>) -> JsonValue {
    jobj(vec![("prices", jobj(pairs))])
}

fn decimal(text: &str) -> PyDecimal {
    PyDecimal::parse_literal(text).expect("a decimal literal")
}

/// The cost report lines of a poll and its table, unwrapped
/// from the reading PORT-11 gives the counter and from the
/// refusal channel FIX-GJ-15 gives it, as a named mechanical
/// consequence of the two repairs. The failure shape itself
/// is pinned below against the reading directly.
fn cost_lines(poll: &Poll, table: &PriceTable) -> Vec<String> {
    grandjury::polls::cost_lines(poll, table)
        .expect("no refusal")
        .expect("a cost report")
}

#[test]
fn a_valid_price_table_loads_as_exact_decimals() {
    let table = load_prices(&prices_obj(vec![
        ("tiny-a", jstr("0.0002")),
        ("tiny-b", jstr("0.1")),
    ]));
    assert_eq!(table.get(&pt("tiny-a")), Some(&decimal("0.0002")));
    assert_eq!(table.get(&pt("tiny-b")), Some(&decimal("0.1")));
    assert_eq!(table.models().len(), 2);
}

#[test]
fn price_text_that_is_not_json_draws_the_named_defect() {
    let (table, defects) = load_prices_text("nope");
    assert!(table.is_none());
    assert_eq!(defects, vec!["input is not valid JSON"]);
}

#[test]
fn a_price_table_that_is_not_an_object_draws_the_named_defect() {
    let (table, defects) = load_prices_text("[1]");
    assert!(table.is_none());
    assert_eq!(defects, vec!["input is not a JSON object"]);
}

#[test]
fn an_unreadable_price_file_draws_the_named_defect() {
    let scratch = TempDir::new("cost-missing");
    let (table, defects) = load_prices_file(&scratch.path.join("missing.json"));
    assert!(table.is_none());
    assert_eq!(defects, vec!["file cannot be read"]);
}

#[test]
fn a_missing_prices_field_is_named() {
    let (table, defects) = load_prices_text("{}");
    assert!(table.is_none());
    assert_eq!(defects, vec!["missing field prices"]);
}

#[test]
fn a_prices_value_of_the_wrong_type_is_named() {
    let (table, defects) = load_prices_text("{\"prices\": [1]}");
    assert!(table.is_none());
    assert_eq!(defects, vec!["field prices has the wrong type"]);
}

#[test]
fn a_price_that_is_not_a_string_is_named() {
    let (table, defects) = load_prices_text("{\"prices\": {\"tiny-a\": 0.0002}}");
    assert!(table.is_none());
    assert_eq!(defects, vec!["price for tiny-a has the wrong type"]);
}

#[test]
fn a_price_that_is_not_a_decimal_string_is_named() {
    let (table, defects) = load_prices_text("{\"prices\": {\"tiny-a\": \"cheap\"}}");
    assert!(table.is_none());
    assert_eq!(defects, vec!["price for tiny-a is not a decimal string"]);
}

#[test]
fn a_nan_price_is_not_a_decimal_string() {
    let (table, defects) = load_prices_text("{\"prices\": {\"tiny-a\": \"NaN\"}}");
    assert!(table.is_none());
    assert_eq!(defects, vec!["price for tiny-a is not a decimal string"]);
}

#[test]
fn an_infinite_price_is_not_a_decimal_string() {
    let (table, defects) = load_prices_text("{\"prices\": {\"tiny-a\": \"Infinity\"}}");
    assert!(table.is_none());
    assert_eq!(defects, vec!["price for tiny-a is not a decimal string"]);
}

#[test]
fn a_negative_price_is_named() {
    let (table, defects) = load_prices_text("{\"prices\": {\"tiny-a\": \"-0.5\"}}");
    assert!(table.is_none());
    assert_eq!(defects, vec!["price for tiny-a is negative"]);
}

#[test]
fn a_price_holding_an_unpaired_surrogate_is_named() {
    let (table, defects) = load_prices_text("{\"prices\": {\"tiny-a\": \"\\ud800x\"}}");
    assert!(table.is_none());
    assert_eq!(
        defects,
        vec!["price for tiny-a holds an unpaired surrogate"]
    );
}

#[test]
fn a_model_label_holding_an_unpaired_surrogate_is_named() {
    let (table, defects) = load_prices_text("{\"prices\": {\"\\ud800x\": \"0.1\"}}");
    assert!(table.is_none());
    assert_eq!(defects, vec!["a model label holds an unpaired surrogate"]);
}

const FORGED_LABEL: &str = "evil\nunpriced model, forged, 7 responses";

fn prices_with_label(label: &str, value: JsonValue) -> String {
    dumps_default(&jobj(vec![(
        "prices",
        JsonValue::object(vec![(pt(label), value)]),
    )]))
}

#[test]
fn a_model_label_holding_a_newline_is_named_not_printed() {
    let (table, defects) = load_prices_text(&prices_with_label(FORGED_LABEL, jint(7)));
    assert!(table.is_none());
    assert_eq!(defects, vec!["a model label holds a newline"]);
}

#[test]
fn a_newline_label_is_named_before_a_bad_price_string() {
    let (table, defects) = load_prices_text(&prices_with_label(FORGED_LABEL, jstr("cheap")));
    assert!(table.is_none());
    assert_eq!(defects, vec!["a model label holds a newline"]);
}

#[test]
fn a_newline_label_is_named_before_a_negative_price() {
    let (table, defects) = load_prices_text(&prices_with_label(FORGED_LABEL, jstr("-0.5")));
    assert!(table.is_none());
    assert_eq!(defects, vec!["a model label holds a newline"]);
}

#[test]
fn a_newline_label_is_named_before_a_surrogate_price() {
    let value = jstr_units(&[0xD800, 0x78]);
    let (table, defects) = load_prices_text(&prices_with_label(FORGED_LABEL, value));
    assert!(table.is_none());
    assert_eq!(defects, vec!["a model label holds a newline"]);
}

#[test]
fn a_newline_label_with_a_valid_price_loads_no_table() {
    let (table, defects) = load_prices_text(&prices_with_label(FORGED_LABEL, jstr("0.1")));
    assert!(table.is_none());
    assert_eq!(defects, vec!["a model label holds a newline"]);
}

#[test]
fn a_carriage_return_label_with_a_valid_price_is_named() {
    let label = "evil\runpriced model, forged, 7 responses";
    let (table, defects) = load_prices_text(&prices_with_label(label, jstr("0.1")));
    assert!(table.is_none());
    assert_eq!(defects, vec!["a model label holds a newline"]);
}

#[test]
fn the_cost_of_a_fully_priced_poll() {
    let poll = load_poll(&poll_of(vec![
        response("juror-1", "red"),
        response("juror-2", "blue"),
        response("juror-3", "red"),
    ]));
    let table = load_prices(&prices_obj(vec![("tiny-a", jstr("0.0002"))]));
    assert_eq!(
        cost_lines(&poll, &table),
        vec![
            "question, q",
            "responses, 3",
            "priced, 3 responses, 0.000600 dollars",
            "unpriced, 0 responses",
        ]
    );
}

#[test]
fn the_counter_carries_no_float_drift() {
    let poll = load_poll(&poll_of(vec![
        response("juror-1", "red"),
        response("juror-2", "red"),
        response("juror-3", "red"),
    ]));
    let table = load_prices(&prices_obj(vec![("tiny-a", jstr("0.1"))]));
    let lines = cost_lines(&poll, &table);
    assert!(lines.contains(&"priced, 3 responses, 0.300000 dollars".to_string()));
}

#[test]
fn spoiled_responses_are_priced_too() {
    let poll = load_poll(&poll_of(vec![
        response("juror-1", "red"),
        response("juror-2", "purple"),
    ]));
    let table = load_prices(&prices_obj(vec![("tiny-a", jstr("0.0002"))]));
    let lines = cost_lines(&poll, &table);
    assert!(lines.contains(&"priced, 2 responses, 0.000400 dollars".to_string()));
}

#[test]
fn unpriced_models_are_named_with_their_counts() {
    let poll = load_poll(&poll_of(vec![
        response_model("juror-1", "red", "tiny-a"),
        response_model("juror-2", "blue", "tiny-b"),
        response_model("juror-3", "red", "tiny-c"),
        response_model("juror-4", "blue", "tiny-b"),
    ]));
    let table = load_prices(&prices_obj(vec![("tiny-a", jstr("0.0002"))]));
    assert_eq!(
        cost_lines(&poll, &table),
        vec![
            "question, q",
            "responses, 4",
            "priced, 1 responses, 0.000200 dollars",
            "unpriced, 3 responses",
            "unpriced model, tiny-b, 2 responses",
            "unpriced model, tiny-c, 1 responses",
        ]
    );
}

#[test]
fn an_empty_price_table_leaves_everything_unpriced() {
    let poll = load_poll(&poll_of(vec![response("juror-1", "red")]));
    let table = load_prices(&prices_obj(vec![]));
    let lines = cost_lines(&poll, &table);
    assert!(lines.contains(&"priced, 0 responses, 0.000000 dollars".to_string()));
    assert!(lines.contains(&"unpriced, 1 responses".to_string()));
    assert!(lines.contains(&"unpriced model, tiny-a, 1 responses".to_string()));
}

#[test]
fn a_total_rounds_to_six_places_with_halves_up() {
    let poll = load_poll(&poll_of(vec![response("juror-1", "red")]));
    let table = load_prices(&prices_obj(vec![("tiny-a", jstr("0.0000025"))]));
    let lines = cost_lines(&poll, &table);
    assert!(lines.contains(&"priced, 1 responses, 0.000003 dollars".to_string()));
}

#[test]
fn a_price_padded_with_whitespace_is_not_a_decimal_string() {
    let (table, defects) = load_prices_text("{\"prices\": {\"tiny-a\": \" 0.1 \"}}");
    assert!(table.is_none());
    assert_eq!(defects, vec!["price for tiny-a is not a decimal string"]);
}

#[test]
fn a_price_holding_an_underscore_is_not_a_decimal_string() {
    let (table, defects) = load_prices_text("{\"prices\": {\"tiny-a\": \"1_0\"}}");
    assert!(table.is_none());
    assert_eq!(defects, vec!["price for tiny-a is not a decimal string"]);
}

#[test]
fn an_empty_price_is_not_a_decimal_string() {
    let (table, defects) = load_prices_text("{\"prices\": {\"tiny-a\": \"\"}}");
    assert!(table.is_none());
    assert_eq!(defects, vec!["price for tiny-a is not a decimal string"]);
}

#[test]
fn a_price_with_a_plus_sign_head_is_not_a_decimal_string() {
    let (table, defects) = load_prices_text("{\"prices\": {\"tiny-a\": \"+0.1\"}}");
    assert!(table.is_none());
    assert_eq!(defects, vec!["price for tiny-a is not a decimal string"]);
}

#[test]
fn unnamed_fields_of_the_price_table_are_ignored() {
    let (table, defects) = load_prices_text(
        "{\"prices\": {\"tiny-a\": \"0.1\"}, \"currency\": \"dollars\", \"version\": 3}",
    );
    assert_eq!(defects, Vec::<String>::new());
    let table = table.expect("a table");
    assert_eq!(table.get(&pt("tiny-a")), Some(&decimal("0.1")));
    assert_eq!(table.models().len(), 1);
}

fn priced_line(price: &str, count: usize) -> String {
    let responses: Vec<JsonValue> = (1..=count)
        .map(|n| response(&format!("juror-{}", n), "red"))
        .collect();
    let poll = load_poll(&poll_of(responses));
    let table = load_prices(&prices_obj(vec![("tiny-a", jstr(price))]));
    cost_lines(&poll, &table)[2].clone()
}

#[test]
fn a_price_at_1e30_renders_its_total_in_full() {
    let line = priced_line("1e30", 1);
    assert_eq!(
        line,
        format!("priced, 1 responses, 1{}.000000 dollars", "0".repeat(30))
    );
}

#[test]
fn a_price_at_1e22_renders_its_total_in_full() {
    let line = priced_line("1e22", 1);
    assert_eq!(
        line,
        format!("priced, 1 responses, 1{}.000000 dollars", "0".repeat(22))
    );
}

#[test]
fn a_price_with_23_integer_digits_renders_in_full() {
    let line = priced_line("99999999999999999999999.99", 1);
    assert_eq!(
        line,
        "priced, 1 responses, 99999999999999999999999.990000 dollars"
    );
}

#[test]
fn an_accumulation_of_large_prices_renders_in_full() {
    let line = priced_line("4e21", 3);
    assert_eq!(
        line,
        format!("priced, 3 responses, 12{}.000000 dollars", "0".repeat(21))
    );
}

#[test]
fn an_exact_half_past_the_28th_digit_rounds_up() {
    let line = priced_line("1000000000000000000000.0000005", 1);
    assert_eq!(
        line,
        "priced, 1 responses, 1000000000000000000000.000001 dollars"
    );
}

#[test]
fn an_abstained_response_is_priced_like_any_other_response() {
    let poll = load_poll(&poll_of(vec![
        response("juror-1", "red"),
        abstained_response("juror-2", "blue", "tiny-a"),
    ]));
    let table = load_prices(&prices_obj(vec![("tiny-a", jstr("0.0002"))]));
    assert_eq!(
        cost_lines(&poll, &table),
        vec![
            "question, q",
            "responses, 2",
            "priced, 2 responses, 0.000400 dollars",
            "unpriced, 0 responses",
        ]
    );
}

#[test]
fn an_abstained_response_from_an_unpriced_model_is_named() {
    let poll = load_poll(&poll_of(vec![
        response("juror-1", "red"),
        abstained_response("juror-2", "blue", "tiny-c"),
        abstained_response("juror-3", "red", "tiny-c"),
    ]));
    let table = load_prices(&prices_obj(vec![("tiny-a", jstr("0.0002"))]));
    assert_eq!(
        cost_lines(&poll, &table),
        vec![
            "question, q",
            "responses, 3",
            "priced, 1 responses, 0.000200 dollars",
            "unpriced, 2 responses",
            "unpriced model, tiny-c, 2 responses",
        ]
    );
}

#[test]
fn a_span_past_two_million_places_still_sums() {
    let poll = load_poll(&poll_of(vec![
        response_model("juror-1", "red", "m1"),
        response_model("juror-2", "red", "m1"),
        response_model("juror-3", "red", "m1"),
        response_model("juror-4", "red", "m2"),
        response_model("juror-5", "red", "m2"),
    ]));
    let table = load_prices(&prices_obj(vec![
        ("m1", jstr("1")),
        ("m2", jstr("1e-2000001")),
    ]));
    assert_eq!(
        grandjury::polls::cost_lines(&poll, &table),
        Ok(Some(vec![
            "question, q".to_string(),
            "responses, 5".to_string(),
            "priced, 5 responses, 3.000000 dollars".to_string(),
            "unpriced, 0 responses".to_string(),
        ]))
    );
}

#[test]
fn a_working_precision_past_the_context_bound_fails_the_reading() {
    let single = load_poll(&poll_of(vec![response_model("juror-1", "red", "m1")]));
    let single_table = load_prices(&prices_obj(vec![("m1", jstr("1e999999999999999999"))]));
    assert_eq!(
        grandjury::polls::cost_lines(&single, &single_table),
        Ok(None)
    );
    let pair = load_poll(&poll_of(vec![
        response_model("juror-1", "red", "m1"),
        response_model("juror-2", "red", "m2"),
    ]));
    let pair_table = load_prices(&prices_obj(vec![
        ("m1", jstr("1")),
        ("m2", jstr("1e-999999999999999997")),
    ]));
    assert_eq!(grandjury::polls::cost_lines(&pair, &pair_table), Ok(None));
}

#[test]
fn a_span_at_or_just_inside_the_context_bound_fails_the_reading() {
    // The three spans of the PORT-11 amendment, the pair of 1
    // with a price whose sized precision stands exactly at the
    // context bound, one step inside it, and at the widest span
    // the re-review probed inside it. The exact coefficient
    // each sum needs cannot be materialized, so the reading
    // fails the way the Python MemoryError fails on the
    // surface, and this pin completes at once on the repaired
    // counter because the reservation fails before any sum
    // runs. Against the pre-repair counter each span grinds
    // with no outcome, which the repair report proves in a
    // scratch copy under a timeout rather than in this suite.
    let poll = load_poll(&poll_of(vec![
        response_model("juror-1", "red", "m1"),
        response_model("juror-2", "red", "m2"),
    ]));
    for tiny in [
        "1e-999999999999999995",
        "1e-999999999999999994",
        "1e-999999999999999990",
    ] {
        let table = load_prices(&prices_obj(vec![("m1", jstr("1")), ("m2", jstr(tiny))]));
        assert_eq!(
            grandjury::polls::cost_lines(&poll, &table),
            Ok(None),
            "{}",
            tiny
        );
    }
}
