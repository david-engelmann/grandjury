//! Pins for GJ-1 and GJ-2, the poll loader and the tally.
//!
//! Translated from tests/test_polls.py. Each malformed shape
//! is pinned at its exact defect phrase from the contract
//! vocabulary, and the tally figures are computed by hand on
//! small polls so a wrong count or a rounded share fails the
//! test.

mod shared;

use grandjury::json::JsonValue;
use grandjury::polls::{load_poll_file, load_poll_text, render_tally_lines, tally_poll};
use num_bigint::BigInt;
use num_rational::Ratio;
use shared::*;

fn response(juror: &str, answer: &str) -> JsonValue {
    response_full(juror, answer, "tiny-a", "a reason")
}

fn response_full(juror: &str, answer: &str, model: &str, reason: &str) -> JsonValue {
    jobj(vec![
        ("juror", jstr(juror)),
        ("model", jstr(model)),
        ("answer", jstr(answer)),
        ("reason", jstr(reason)),
    ])
}

fn abstained_response(juror: &str, answer: &str) -> JsonValue {
    abstained_response_full(juror, answer, "tiny-a", "a reason")
}

fn abstained_response_full(juror: &str, answer: &str, model: &str, reason: &str) -> JsonValue {
    let mut entry = response_full(juror, answer, model, reason);
    if let JsonValue::Obj(pairs) = &mut entry {
        pairs.push((pt("abstained"), JsonValue::Bool(true)));
    }
    entry
}

fn valid_data() -> JsonValue {
    jobj(vec![
        ("question", jstr("Which door should the agent pick?")),
        (
            "answers",
            jarr(vec![jstr("red"), jstr("blue"), jstr("green")]),
        ),
        (
            "responses",
            jarr(vec![
                response("juror-1", "red"),
                response("juror-2", "blue"),
            ]),
        ),
    ])
}

fn load(data: &JsonValue) -> (Option<grandjury::polls::Poll>, Vec<String>) {
    load_poll_text(&dumps_default(data))
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

fn response_mut(data: &mut JsonValue, index: usize) -> &mut JsonValue {
    match data {
        JsonValue::Obj(pairs) => {
            for (held, slot) in pairs.iter_mut() {
                if held.display() == "responses" {
                    return match slot {
                        JsonValue::Arr(items) => &mut items[index],
                        _ => panic!("responses is an array"),
                    };
                }
            }
            panic!("responses stands in the data");
        }
        _ => panic!("the data is an object"),
    }
}

fn defects_of(data: &JsonValue) -> Vec<String> {
    let (poll, defects) = load(data);
    assert!(poll.is_none());
    defects
}

#[test]
fn a_valid_poll_loads_clean() {
    let (poll, defects) = load(&valid_data());
    assert_eq!(defects, Vec::<String>::new());
    let poll = poll.expect("a poll");
    assert_eq!(poll.question, pt("Which door should the agent pick?"));
    assert_eq!(poll.answers, vec![pt("red"), pt("blue"), pt("green")]);
    assert_eq!(poll.responses.len(), 2);
}

#[test]
fn an_empty_responses_list_loads_clean() {
    let mut data = valid_data();
    set_field(&mut data, "responses", jarr(vec![]));
    let (poll, defects) = load(&data);
    assert_eq!(defects, Vec::<String>::new());
    assert_eq!(poll.expect("a poll").responses.len(), 0);
}

#[test]
fn text_that_is_not_json_draws_the_named_defect() {
    let (poll, defects) = load_poll_text("this is not json");
    assert!(poll.is_none());
    assert_eq!(defects, vec!["input is not valid JSON"]);
}

#[test]
fn deeply_nested_input_draws_the_not_valid_json_defect() {
    let text = format!("{}{}", "[".repeat(100000), "]".repeat(100000));
    let (poll, defects) = load_poll_text(&text);
    assert!(poll.is_none());
    assert_eq!(defects, vec!["input is not valid JSON"]);
}

#[test]
fn json_that_is_not_an_object_draws_the_named_defect() {
    let (poll, defects) = load_poll_text("[1, 2, 3]");
    assert!(poll.is_none());
    assert_eq!(defects, vec!["input is not a JSON object"]);
}

#[test]
fn an_unreadable_file_draws_the_named_defect() {
    let scratch = TempDir::new("polls-missing");
    let (poll, defects) = load_poll_file(&scratch.path.join("missing.json"));
    assert!(poll.is_none());
    assert_eq!(defects, vec!["file cannot be read"]);
}

#[test]
fn a_missing_question_is_named() {
    let mut data = valid_data();
    drop_field(&mut data, "question");
    assert_eq!(defects_of(&data), vec!["missing field question"]);
}

#[test]
fn a_question_of_the_wrong_type_is_named() {
    let mut data = valid_data();
    set_field(&mut data, "question", jint(7));
    assert_eq!(defects_of(&data), vec!["field question has the wrong type"]);
}

#[test]
fn an_empty_question_is_named() {
    let mut data = valid_data();
    set_field(&mut data, "question", jstr(""));
    assert_eq!(defects_of(&data), vec!["field question is empty"]);
}

#[test]
fn a_question_holding_an_unpaired_surrogate_is_named() {
    let text = "{\"question\": \"\\ud800x\", \"answers\": [\"red\", \"blue\"], \"responses\": []}";
    let (poll, defects) = load_poll_text(text);
    assert!(poll.is_none());
    assert_eq!(defects, vec!["field question holds an unpaired surrogate"]);
}

#[test]
fn a_missing_answers_field_is_named() {
    let mut data = valid_data();
    drop_field(&mut data, "answers");
    assert_eq!(defects_of(&data), vec!["missing field answers"]);
}

#[test]
fn answers_of_the_wrong_type_are_named() {
    let mut data = valid_data();
    set_field(&mut data, "answers", jstr("red"));
    assert_eq!(defects_of(&data), vec!["field answers has the wrong type"]);
}

#[test]
fn fewer_than_two_answers_are_named() {
    let mut data = valid_data();
    set_field(&mut data, "answers", jarr(vec![jstr("red")]));
    assert_eq!(
        defects_of(&data),
        vec!["field answers holds fewer than two answers"]
    );
}

#[test]
fn an_answer_of_the_wrong_type_is_named() {
    let mut data = valid_data();
    set_field(
        &mut data,
        "answers",
        jarr(vec![jstr("red"), jint(4), jstr("green")]),
    );
    assert_eq!(defects_of(&data), vec!["answer 1 has the wrong type"]);
}

#[test]
fn an_empty_answer_is_named() {
    let mut data = valid_data();
    set_field(
        &mut data,
        "answers",
        jarr(vec![jstr("red"), jstr(""), jstr("green")]),
    );
    assert_eq!(defects_of(&data), vec!["answer 1 is empty"]);
}

#[test]
fn an_answer_holding_an_unpaired_surrogate_is_named() {
    let text = "{\"question\": \"q\", \"answers\": [\"red\", \"\\ud800x\"], \"responses\": []}";
    let (poll, defects) = load_poll_text(text);
    assert!(poll.is_none());
    assert_eq!(defects, vec!["answer 1 holds an unpaired surrogate"]);
}

#[test]
fn a_duplicate_answer_is_named() {
    let mut data = valid_data();
    set_field(
        &mut data,
        "answers",
        jarr(vec![jstr("red"), jstr("blue"), jstr("red")]),
    );
    assert_eq!(defects_of(&data), vec!["duplicate answer red"]);
}

#[test]
fn a_missing_responses_field_is_named() {
    let mut data = valid_data();
    drop_field(&mut data, "responses");
    assert_eq!(defects_of(&data), vec!["missing field responses"]);
}

#[test]
fn responses_of_the_wrong_type_are_named() {
    let mut data = valid_data();
    set_field(&mut data, "responses", jstr("many"));
    assert_eq!(
        defects_of(&data),
        vec!["field responses has the wrong type"]
    );
}

#[test]
fn a_response_that_is_not_an_object_is_named() {
    let mut data = valid_data();
    set_field(&mut data, "responses", jarr(vec![jstr("red")]));
    assert_eq!(defects_of(&data), vec!["response 0 is not an object"]);
}

#[test]
fn a_response_missing_a_field_is_named() {
    let mut data = valid_data();
    let entry = response_mut(&mut data, 0);
    let mut held = response("juror-1", "red");
    drop_field(&mut held, "reason");
    *entry = held;
    assert_eq!(
        defects_of(&data),
        vec!["response 0 is missing field reason"]
    );
}

#[test]
fn a_response_field_of_the_wrong_type_is_named() {
    let mut data = valid_data();
    let entry = response_mut(&mut data, 1);
    set_field(entry, "model", jint(3));
    assert_eq!(
        defects_of(&data),
        vec!["response 1 field model has the wrong type"]
    );
}

#[test]
fn an_empty_juror_is_named() {
    let mut data = valid_data();
    let entry = response_mut(&mut data, 0);
    set_field(entry, "juror", jstr(""));
    assert_eq!(defects_of(&data), vec!["response 0 field juror is empty"]);
}

#[test]
fn an_empty_reason_is_named() {
    let mut data = valid_data();
    let entry = response_mut(&mut data, 0);
    set_field(entry, "reason", jstr(""));
    assert_eq!(defects_of(&data), vec!["response 0 field reason is empty"]);
}

#[test]
fn a_response_field_holding_an_unpaired_surrogate_is_named() {
    let text = "{\"question\": \"q\", \"answers\": [\"red\", \"blue\"], \"responses\": [{\"juror\": \"j1\", \"model\": \"\\ud800x\", \"answer\": \"red\", \"reason\": \"fine\"}]}";
    let (poll, defects) = load_poll_text(text);
    assert!(poll.is_none());
    assert_eq!(
        defects,
        vec!["response 0 field model holds an unpaired surrogate"]
    );
}

#[test]
fn a_reason_holding_a_newline_is_named() {
    let mut data = valid_data();
    let entry = response_mut(&mut data, 0);
    set_field(entry, "reason", jstr("two\nlines"));
    assert_eq!(
        defects_of(&data),
        vec!["response 0 field reason holds a newline"]
    );
}

#[test]
fn a_reason_holding_a_carriage_return_is_named() {
    let mut data = valid_data();
    let entry = response_mut(&mut data, 0);
    set_field(entry, "reason", jstr("two\rlines"));
    assert_eq!(
        defects_of(&data),
        vec!["response 0 field reason holds a newline"]
    );
}

#[test]
fn a_question_holding_a_newline_is_named() {
    let mut data = valid_data();
    set_field(
        &mut data,
        "question",
        jstr("Which door\nshould the agent pick?"),
    );
    let (poll, defects) = load(&data);
    assert!(poll.is_none());
    assert_eq!(defects, vec!["field question holds a newline"]);
}

#[test]
fn a_question_holding_a_carriage_return_is_named() {
    let mut data = valid_data();
    set_field(
        &mut data,
        "question",
        jstr("Which door\rshould the agent pick?"),
    );
    let (poll, defects) = load(&data);
    assert!(poll.is_none());
    assert_eq!(defects, vec!["field question holds a newline"]);
}

#[test]
fn an_answer_holding_a_newline_is_named() {
    let mut data = valid_data();
    set_field(
        &mut data,
        "answers",
        jarr(vec![jstr("red"), jstr("blue\nspoofed"), jstr("green")]),
    );
    let (poll, defects) = load(&data);
    assert!(poll.is_none());
    assert_eq!(defects, vec!["answer 1 holds a newline"]);
}

#[test]
fn a_juror_holding_a_newline_is_named() {
    let mut data = valid_data();
    let entry = response_mut(&mut data, 0);
    set_field(entry, "juror", jstr("juror-1\nspoofed, 99, 100.0%"));
    let (poll, defects) = load(&data);
    assert!(poll.is_none());
    assert_eq!(defects, vec!["response 0 field juror holds a newline"]);
}

#[test]
fn a_model_holding_a_newline_is_named() {
    let mut data = valid_data();
    let entry = response_mut(&mut data, 0);
    set_field(
        entry,
        "model",
        jstr("tiny-a\nunpriced model, spoof, 9 responses"),
    );
    let (poll, defects) = load(&data);
    assert!(poll.is_none());
    assert_eq!(defects, vec!["response 0 field model holds a newline"]);
}

#[test]
fn a_response_answer_holding_a_carriage_return_is_named() {
    let mut data = valid_data();
    let entry = response_mut(&mut data, 0);
    set_field(entry, "answer", jstr("red\rspoofed"));
    let (poll, defects) = load(&data);
    assert!(poll.is_none());
    assert_eq!(defects, vec!["response 0 field answer holds a newline"]);
}

#[test]
fn unnamed_fields_of_the_poll_are_ignored() {
    let mut data = valid_data();
    set_field(
        &mut data,
        "counts",
        jobj(vec![("red", jint(99)), ("blue", jint(99))]),
    );
    set_field(
        &mut data,
        "source",
        jstr("a stored figure the tally must not read"),
    );
    let (poll, defects) = load(&data);
    assert_eq!(defects, Vec::<String>::new());
    let poll = poll.expect("a poll");
    let keys: Vec<String> = poll
        .to_json()
        .as_object()
        .expect("an object")
        .iter()
        .map(|(key, _)| key.display())
        .collect();
    assert_eq!(keys, vec!["question", "answers", "responses"]);
    let tally = tally_poll(&poll);
    assert_eq!(tally.responses, 2);
    let rows = &tally.rows;
    assert_eq!(rows[0].1, 1);
    assert_eq!(rows[1].1, 1);
}

#[test]
fn unnamed_fields_of_a_response_are_ignored() {
    let mut data = valid_data();
    let entry = response_mut(&mut data, 0);
    set_field(entry, "confidence", jstr("high"));
    set_field(entry, "seed", jint(42));
    let (poll, defects) = load(&data);
    assert_eq!(defects, Vec::<String>::new());
    let poll = poll.expect("a poll");
    let keys: Vec<String> = poll.responses[0]
        .to_json()
        .as_object()
        .expect("an object")
        .iter()
        .map(|(key, _)| key.display())
        .collect();
    assert_eq!(keys, vec!["juror", "model", "answer", "reason"]);
}

#[test]
fn a_duplicate_juror_is_named() {
    let mut data = valid_data();
    let entry = response_mut(&mut data, 1);
    set_field(entry, "juror", jstr("juror-1"));
    assert_eq!(defects_of(&data), vec!["duplicate juror juror-1"]);
}

#[test]
fn a_spoiled_response_is_not_a_loader_defect() {
    let mut data = valid_data();
    let entry = response_mut(&mut data, 0);
    set_field(entry, "answer", jstr("purple"));
    let (poll, defects) = load(&data);
    assert_eq!(defects, Vec::<String>::new());
    assert_eq!(poll.expect("a poll").responses[0].answer, pt("purple"));
}

#[test]
fn several_defects_arrive_in_one_pass() {
    let mut data = valid_data();
    set_field(&mut data, "question", jstr(""));
    set_field(&mut data, "answers", jarr(vec![jstr("red"), jstr("red")]));
    set_field(
        &mut data,
        "responses",
        jarr(vec![jobj(vec![("juror", jstr("j1"))])]),
    );
    let (poll, defects) = load(&data);
    assert!(poll.is_none());
    assert!(defects.contains(&"field question is empty".to_string()));
    assert!(defects.contains(&"duplicate answer red".to_string()));
    assert!(defects.contains(&"response 0 is missing field model".to_string()));
    assert!(defects.len() >= 4);
}

fn hand_poll() -> JsonValue {
    jobj(vec![
        ("question", jstr("Which door should the agent pick?")),
        (
            "answers",
            jarr(vec![jstr("red"), jstr("blue"), jstr("green")]),
        ),
        (
            "responses",
            jarr(vec![
                response("juror-1", "red"),
                response("juror-2", "blue"),
                response("juror-3", "red"),
                response("juror-4", "red"),
                response("juror-5", "blue"),
                response("juror-6", "purple"),
            ]),
        ),
    ])
}

fn fraction(numerator: i64, denominator: i64) -> Ratio<BigInt> {
    Ratio::new(BigInt::from(numerator), BigInt::from(denominator))
}

#[test]
fn the_tally_counts_and_shares_are_computed_by_hand() {
    let (poll, defects) = load(&hand_poll());
    assert_eq!(defects, Vec::<String>::new());
    let tally = tally_poll(&poll.expect("a poll"));
    assert_eq!(tally.responses, 6);
    assert_eq!(tally.valid, 5);
    assert_eq!(tally.spoiled, 1);
    let row = |answer: &str| {
        tally
            .rows
            .iter()
            .find(|(held, _, _)| *held == pt(answer))
            .unwrap_or_else(|| panic!("a row for {}", answer))
    };
    assert_eq!(row("red").1, 3);
    assert_eq!(row("red").2, fraction(3, 5));
    assert_eq!(row("blue").1, 2);
    assert_eq!(row("blue").2, fraction(2, 5));
    assert_eq!(row("green").1, 0);
    assert_eq!(row("green").2, fraction(0, 1));
}

#[test]
fn the_tally_rows_stand_in_offered_order() {
    let (poll, _) = load(&hand_poll());
    let tally = tally_poll(&poll.expect("a poll"));
    let answers: Vec<String> = tally.rows.iter().map(|(a, _, _)| a.display()).collect();
    assert_eq!(answers, vec!["red", "blue", "green"]);
}

#[test]
fn the_tally_of_an_empty_poll_has_zero_shares() {
    let mut data = valid_data();
    set_field(&mut data, "responses", jarr(vec![]));
    let (poll, _) = load(&data);
    let tally = tally_poll(&poll.expect("a poll"));
    assert_eq!(tally.valid, 0);
    assert_eq!(tally.spoiled, 0);
    assert!(tally
        .rows
        .iter()
        .all(|(_, _, share)| *share == fraction(0, 1)));
    assert!(tally.rows.iter().all(|(_, count, _)| *count == 0));
}

#[test]
fn the_tally_lines_render_the_hand_computed_poll() {
    let (poll, _) = load(&hand_poll());
    let poll = poll.expect("a poll");
    assert_eq!(
        render_tally_lines(&poll).expect("a tally"),
        vec![
            "question, Which door should the agent pick?",
            "responses, 6",
            "valid, 5",
            "spoiled, 1",
            "red, 3, 60.0%",
            "blue, 2, 40.0%",
            "green, 0, 0.0%",
        ]
    );
}

#[test]
fn a_share_renders_as_a_percentage_with_one_decimal() {
    let mut data = valid_data();
    set_field(&mut data, "answers", jarr(vec![jstr("red"), jstr("blue")]));
    set_field(
        &mut data,
        "responses",
        jarr(vec![
            response("juror-1", "red"),
            response("juror-2", "blue"),
            response("juror-3", "blue"),
        ]),
    );
    let (poll, _) = load(&data);
    let lines = render_tally_lines(&poll.expect("a poll")).expect("a tally");
    assert!(lines.contains(&"red, 1, 33.3%".to_string()));
    assert!(lines.contains(&"blue, 2, 66.7%".to_string()));
}

#[test]
fn a_percentage_half_rounds_up_not_to_even() {
    let mut data = valid_data();
    set_field(&mut data, "answers", jarr(vec![jstr("red"), jstr("blue")]));
    let mut responses = vec![response("juror-1", "red")];
    for n in 2..17 {
        responses.push(response(&format!("juror-{}", n), "blue"));
    }
    set_field(&mut data, "responses", jarr(responses));
    let (poll, _) = load(&data);
    let lines = render_tally_lines(&poll.expect("a poll")).expect("a tally");
    assert!(lines.contains(&"red, 1, 6.3%".to_string()));
    assert!(lines.contains(&"blue, 15, 93.8%".to_string()));
}

#[test]
fn an_abstained_response_loads_clean_and_keeps_its_kind() {
    let mut data = valid_data();
    set_field(
        &mut data,
        "responses",
        jarr(vec![
            response("juror-1", "red"),
            abstained_response("juror-2", "blue"),
        ]),
    );
    let (poll, defects) = load(&data);
    assert_eq!(defects, Vec::<String>::new());
    let poll = poll.expect("a poll");
    assert!(!poll.responses[0].abstained);
    assert!(poll.responses[1].abstained);
}

#[test]
fn an_abstained_field_holding_false_is_named() {
    let mut data = valid_data();
    let mut entry = response("juror-1", "red");
    set_field(&mut entry, "abstained", JsonValue::Bool(false));
    set_field(&mut data, "responses", jarr(vec![entry]));
    assert_eq!(
        defects_of(&data),
        vec!["response 0 field abstained is not true"]
    );
}

#[test]
fn an_abstained_field_holding_a_string_is_named() {
    let mut data = valid_data();
    let mut entry = response("juror-1", "red");
    set_field(&mut entry, "abstained", jstr("true"));
    set_field(
        &mut data,
        "responses",
        jarr(vec![response("juror-0", "blue"), entry]),
    );
    assert_eq!(
        defects_of(&data),
        vec!["response 1 field abstained is not true"]
    );
}

#[test]
fn an_abstained_field_holding_a_number_is_named() {
    let mut data = valid_data();
    let mut entry = response("juror-1", "red");
    set_field(&mut entry, "abstained", jint(1));
    set_field(&mut data, "responses", jarr(vec![entry]));
    assert_eq!(
        defects_of(&data),
        vec!["response 0 field abstained is not true"]
    );
}

#[test]
fn an_abstained_response_with_an_empty_reason_is_still_named() {
    let mut data = valid_data();
    let entry = abstained_response_full("juror-1", "red", "tiny-a", "");
    set_field(&mut data, "responses", jarr(vec![entry]));
    assert_eq!(defects_of(&data), vec!["response 0 field reason is empty"]);
}

#[test]
fn a_duplicate_juror_is_named_when_one_response_abstained() {
    let mut data = valid_data();
    set_field(
        &mut data,
        "responses",
        jarr(vec![
            response("juror-1", "red"),
            abstained_response("juror-1", "blue"),
        ]),
    );
    assert_eq!(defects_of(&data), vec!["duplicate juror juror-1"]);
}

#[test]
fn the_tally_counts_abstained_responses_apart() {
    let mut data = valid_data();
    set_field(
        &mut data,
        "responses",
        jarr(vec![
            response("juror-1", "red"),
            response("juror-2", "red"),
            response("juror-3", "blue"),
            response("juror-4", "purple"),
            abstained_response("juror-5", "red"),
            abstained_response("juror-6", "purple"),
        ]),
    );
    let (poll, defects) = load(&data);
    assert_eq!(defects, Vec::<String>::new());
    let tally = tally_poll(&poll.expect("a poll"));
    assert_eq!(tally.responses, 6);
    assert_eq!(tally.valid, 3);
    assert_eq!(tally.spoiled, 1);
    assert_eq!(tally.abstained, 2);
    let row = |answer: &str| {
        tally
            .rows
            .iter()
            .find(|(held, _, _)| *held == pt(answer))
            .unwrap_or_else(|| panic!("a row for {}", answer))
    };
    assert_eq!(row("red").1, 2);
    assert_eq!(row("red").2, fraction(2, 3));
    assert_eq!(row("blue").1, 1);
    assert_eq!(row("blue").2, fraction(1, 3));
}

#[test]
fn the_tally_lines_name_the_abstained_count_after_spoiled() {
    let mut data = valid_data();
    set_field(&mut data, "answers", jarr(vec![jstr("red"), jstr("blue")]));
    set_field(
        &mut data,
        "responses",
        jarr(vec![
            response("juror-1", "red"),
            response("juror-2", "purple"),
            abstained_response("juror-3", "blue"),
        ]),
    );
    let (poll, defects) = load(&data);
    assert_eq!(defects, Vec::<String>::new());
    let poll = poll.expect("a poll");
    assert_eq!(
        render_tally_lines(&poll).expect("a tally"),
        vec![
            "question, Which door should the agent pick?",
            "responses, 3",
            "valid, 1",
            "spoiled, 1",
            "abstained, 1",
            "red, 1, 100.0%",
            "blue, 0, 0.0%",
        ]
    );
}

#[test]
fn the_tally_lines_omit_the_abstained_line_when_none_abstained() {
    let mut data = valid_data();
    set_field(&mut data, "answers", jarr(vec![jstr("red"), jstr("blue")]));
    set_field(
        &mut data,
        "responses",
        jarr(vec![
            response("juror-1", "red"),
            response("juror-2", "purple"),
        ]),
    );
    let (poll, defects) = load(&data);
    assert_eq!(defects, Vec::<String>::new());
    let poll = poll.expect("a poll");
    assert_eq!(
        render_tally_lines(&poll).expect("a tally"),
        vec![
            "question, Which door should the agent pick?",
            "responses, 2",
            "valid, 1",
            "spoiled, 1",
            "red, 1, 100.0%",
            "blue, 0, 0.0%",
        ]
    );
}
