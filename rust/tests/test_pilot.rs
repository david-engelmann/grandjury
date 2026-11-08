//! Pins for GJ-6, the pilot recording.
//!
//! Translated from tests/test_pilot.py. The twelve recorded
//! polls load clean, the envelope inventory is complete, the
//! envelopes replay to the committed polls through the seat
//! readers, the goldens regenerate byte identical, and the
//! run record and the pilot price recompute from the recorded
//! envelopes. The incident section of the run record is stated
//! data, and it is pinned against its input file and its own
//! arithmetic. Every pin runs offline.

mod shared;

use std::collections::BTreeSet;
use std::path::PathBuf;

use grandjury::json::JsonValue;
use grandjury::polls::{load_poll_file, load_prices_file, Poll};
use grandjury::pydecimal::{PyDecimal, Rounding};
use grandjury::seats::{response_from_chat_envelope, response_from_jev_envelope};
use shared::*;

fn pilot() -> PathBuf {
    examples().join("pilot")
}

type Reader = fn(&grandjury::pytext::PText, &JsonValue) -> Option<grandjury::polls::Response>;

fn routes() -> Vec<(&'static str, Reader)> {
    vec![
        ("jev", response_from_jev_envelope),
        ("openai", response_from_chat_envelope),
    ]
}

fn envelope_names() -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for index in 1..=12 {
        for prefix in ["jev", "openai"] {
            for seat in 1..=5 {
                names.insert(format!("q{:02}-{}-{}.json", index, prefix, seat));
            }
        }
    }
    names
}

fn rebuild_poll(index: usize, item: &JsonValue) -> JsonValue {
    let mut responses: Vec<JsonValue> = Vec::new();
    for (prefix, reader) in routes() {
        for seat in 1..=5 {
            let envelope = read_json(
                &pilot()
                    .join("envelopes")
                    .join(format!("q{:02}-{}-{}.json", index, prefix, seat)),
            );
            let juror = pt(&format!("{}-{}", prefix, seat));
            if let Some(response) = reader(&juror, &envelope) {
                responses.push(response.to_json());
            }
        }
    }
    jobj(vec![
        ("question", field(item, "question").clone()),
        ("answers", field(item, "answers").clone()),
        ("responses", jarr(responses)),
    ])
}

fn questions() -> Vec<JsonValue> {
    match read_json(&pilot().join("questions.json")).as_array() {
        Some(items) => items.clone(),
        None => panic!("the questions stand as an array"),
    }
}

#[test]
fn the_pilot_question_set_holds_twelve_questions() {
    let items = questions();
    assert_eq!(items.len(), 12);
    for item in &items {
        let obj = item.as_object().expect("an object");
        let mut keys: Vec<String> = obj.iter().map(|(key, _)| key.display()).collect();
        keys.sort();
        assert_eq!(keys, vec!["answers", "question"]);
        let question = as_str(field(item, "question"));
        assert!(!question.is_empty());
        let answers = match field(item, "answers") {
            JsonValue::Arr(held) => held,
            _ => panic!("answers stand as an array"),
        };
        assert!((2..=4).contains(&answers.len()));
        for answer in answers {
            assert!(!as_str(answer).is_empty());
        }
    }
}

#[test]
fn the_envelope_inventory_is_complete() {
    let mut on_disk = BTreeSet::new();
    for entry in std::fs::read_dir(pilot().join("envelopes")).expect("the envelopes stand") {
        let name = entry
            .expect("an entry")
            .file_name()
            .to_string_lossy()
            .into_owned();
        if name.ends_with(".json") {
            on_disk.insert(name);
        }
    }
    assert_eq!(on_disk, envelope_names());
}

#[test]
fn every_pilot_poll_loads_clean_with_ten_responses() {
    let mut jurors = BTreeSet::new();
    for seat in 1..=5 {
        jurors.insert(format!("jev-{}", seat));
        jurors.insert(format!("openai-{}", seat));
    }
    for index in 1..=12 {
        let (poll, defects) = load_poll_file(&pilot().join(format!("poll-{:02}.json", index)));
        assert_eq!(defects, Vec::<String>::new());
        let poll: Poll = poll.expect("a poll");
        assert_eq!(poll.responses.len(), 10);
        let held: BTreeSet<String> = poll
            .responses
            .iter()
            .map(|response| response.juror.display())
            .collect();
        assert_eq!(held, jurors);
    }
}

#[test]
fn the_recorded_envelopes_replay_to_the_committed_polls() {
    let items = questions();
    for (offset, item) in items.iter().enumerate() {
        let index = offset + 1;
        let committed = read_json(&pilot().join(format!("poll-{:02}.json", index)));
        assert_json_eq(&rebuild_poll(index, item), &committed);
    }
}

#[test]
fn the_example_poll_is_the_first_pilot_poll() {
    assert_json_eq(
        &read_json(&examples().join("pilot_poll.json")),
        &read_json(&pilot().join("poll-01.json")),
    );
}

#[test]
fn the_pilot_goldens_regenerate_byte_identical() {
    let (poll, defects) = load_poll_file(&examples().join("pilot_poll.json"));
    assert_eq!(defects, Vec::<String>::new());
    let poll = poll.expect("a poll");
    let stadium = read_text(&examples().join("pilot-stadium-golden.txt"));
    assert_eq!(
        grandjury::polls::render_stadium(&poll)
            .expect("a stadium view")
            .join("\n")
            + "\n",
        stadium
    );
    let (table, price_defects) = load_prices_file(&examples().join("pilot-prices.json"));
    assert_eq!(price_defects, Vec::<String>::new());
    let table = table.expect("a table");
    let cost = read_text(&examples().join("pilot-cost-golden.txt"));
    assert_eq!(
        grandjury::polls::cost_lines(&poll, &table)
            .expect("no refusal")
            .expect("a cost report")
            .join("\n")
            + "\n",
        cost
    );
}

struct Recomputed {
    recorded: i64,
    abstained: i64,
    failed: i64,
    input_tokens: i64,
    output_tokens: i64,
    cost: PyDecimal,
}

fn exact_cost(envelope_exact: &JsonValue) -> PyDecimal {
    match field(field(envelope_exact, "usage"), "cost") {
        JsonValue::Num(grandjury::pynum::Num::Decimal(value)) => value.clone(),
        other => panic!("the usage cost is a decimal, got {:?}", other),
    }
}

fn recompute_model_record(prefix: &str, reader: Reader, items: &[JsonValue]) -> Recomputed {
    let mut got = Recomputed {
        recorded: 0,
        abstained: 0,
        failed: 0,
        input_tokens: 0,
        output_tokens: 0,
        cost: PyDecimal::zero(),
    };
    for (offset, _item) in items.iter().enumerate() {
        let index = offset + 1;
        for seat in 1..=5 {
            let path = pilot()
                .join("envelopes")
                .join(format!("q{:02}-{}-{}.json", index, prefix, seat));
            let envelope = read_json(&path);
            let exact = read_json_exact(&path);
            let usage = field(&envelope, "usage");
            if prefix == "jev" {
                got.input_tokens += as_int(field(usage, "input_tokens"));
                got.output_tokens += as_int(field(usage, "output_tokens"));
            } else {
                got.input_tokens += as_int(field(usage, "prompt_tokens"));
                got.output_tokens += as_int(field(usage, "completion_tokens"));
                got.cost = got
                    .cost
                    .added_ctx(&exact_cost(&exact), 28, Rounding::HalfEven);
            }
            let juror = pt(&format!("{}-{}", prefix, seat));
            match reader(&juror, &envelope) {
                None => got.failed += 1,
                Some(response) => {
                    got.recorded += 1;
                    if response.abstained {
                        got.abstained += 1;
                    }
                }
            }
        }
    }
    got
}

#[test]
fn the_run_record_agrees_with_the_recorded_envelopes() {
    let items = questions();
    let record = read_json(&pilot().join("run_record.json"));
    assert_eq!(as_int(field(&record, "questions")), 12);
    assert_eq!(as_int(field(&record, "seats_per_question_per_model")), 5);
    let models = field(&record, "models");
    let mut model_names: Vec<String> = models
        .as_object()
        .expect("an object")
        .iter()
        .map(|(key, _)| key.display())
        .collect();
    model_names.sort();
    assert_eq!(model_names, vec!["jev-1.13.0", "openai/gpt-6-luna"]);
    for (prefix, reader, model, route) in [
        (
            "jev",
            response_from_jev_envelope as Reader,
            "jev-1.13.0",
            "jev",
        ),
        (
            "openai",
            response_from_chat_envelope as Reader,
            "openai/gpt-6-luna",
            "chat",
        ),
    ] {
        let entry = field(models, model);
        let got = recompute_model_record(prefix, reader, &items);
        assert_eq!(as_str(field(entry, "route")), route);
        assert_eq!(as_int(field(entry, "calls_attempted")), 60);
        assert_eq!(as_int(field(entry, "calls_recorded")), got.recorded);
        assert_eq!(as_int(field(entry, "failed_calls")), got.failed);
        assert_eq!(as_int(field(entry, "abstained")), got.abstained);
        assert_eq!(as_int(field(entry, "input_tokens")), got.input_tokens);
        assert_eq!(as_int(field(entry, "output_tokens")), got.output_tokens);
    }
    let chat_entry = field(models, "openai/gpt-6-luna");
    let chat_got = recompute_model_record("openai", response_from_chat_envelope, &items);
    assert_eq!(
        as_str(field(chat_entry, "cost_dollars")),
        chat_got.cost.render()
    );
    assert!(field(models, "jev-1.13.0").get("cost_dollars").is_none());
}

#[test]
fn the_run_record_incident_section_matches_its_input_file() {
    let record = read_json(&pilot().join("run_record.json"));
    assert_json_eq(
        field(&record, "incident"),
        &read_json(&pilot().join("incident.json")),
    );
}

#[test]
fn the_incident_section_carries_the_named_counts() {
    let record = read_json(&pilot().join("run_record.json"));
    let incident = field(&record, "incident");
    let jev = field(incident, "jev-1.13.0");
    let chat = field(incident, "openai/gpt-6-luna");
    assert_eq!(
        as_int(field(jev, "attempts_that_never_left_the_machine")),
        40
    );
    assert_eq!(as_int(field(jev, "envelopes_lost_and_seats_rerun")), 10);
    assert_eq!(as_int(field(jev, "calls_discarded")), 0);
    assert_eq!(
        as_int(field(chat, "attempts_that_never_left_the_machine")),
        0
    );
    assert_eq!(as_int(field(chat, "envelopes_lost_and_seats_rerun")), 5);
    assert_eq!(as_int(field(chat, "calls_discarded")), 1);
    for model in ["jev-1.13.0", "openai/gpt-6-luna"] {
        let entry = field(incident, model);
        let parts: i64 = entry
            .as_object()
            .expect("an object")
            .iter()
            .filter(|(name, _)| name.display() != "attempts_in_round")
            .map(|(_, value)| as_int(value))
            .sum();
        assert_eq!(parts, as_int(field(entry, "attempts_in_round")));
        assert_eq!(
            as_int(field(entry, "calls_in_the_recorded_run")),
            as_int(field(
                field(field(&record, "models"), model),
                "calls_attempted"
            ))
        );
    }
    let total = as_int(field(jev, "attempts_in_round")) + as_int(field(chat, "attempts_in_round"));
    assert_eq!(total, 180);
    assert_eq!(
        as_str(field(incident, "lost_token_usage")),
        "unrecoverable and estimated nowhere"
    );
}

#[test]
fn the_pilot_price_recomputes_from_the_recorded_envelopes() {
    let items = questions();
    let mut total = PyDecimal::zero();
    let mut count = 0i64;
    for (offset, _item) in items.iter().enumerate() {
        let index = offset + 1;
        for seat in 1..=5 {
            let path = pilot()
                .join("envelopes")
                .join(format!("q{:02}-openai-{}.json", index, seat));
            let envelope = read_json(&path);
            let juror = pt(&format!("openai-{}", seat));
            if response_from_chat_envelope(&juror, &envelope).is_some() {
                let exact = read_json_exact(&path);
                total = total.added_ctx(&exact_cost(&exact), 28, Rounding::HalfEven);
                count += 1;
            }
        }
    }
    let mean = total
        .divided_default(&PyDecimal::parse_literal(&count.to_string()).expect("a count"))
        .expect("a mean");
    let price = mean
        .quantized(-6, 28, Rounding::HalfUp)
        .expect("a price at six places");
    let (table, defects) = load_prices_file(&examples().join("pilot-prices.json"));
    assert_eq!(defects, Vec::<String>::new());
    let table = table.expect("a table");
    assert_eq!(table.get(&pt("openai/gpt-6-luna")), Some(&price));
    assert!(table.get(&pt("jev-1.13.0")).is_none());
    assert_eq!(table.models().len(), 1);
}
