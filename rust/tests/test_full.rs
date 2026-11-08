//! Pins for GJ-7, the full poll, and its seat discipline.
//!
//! Translated from tests/test_full.py. The forty full polls
//! were recorded live in Round 3 over the two decision model
//! routes, five seats per question per model. Everything here
//! replays offline against the recorded envelopes. A pin
//! fails under the negation of the rule it names. The pins
//! hold the question set against the pilot set, the envelope
//! inventory against the run record, the polls against a
//! rebuild from the envelopes through the ported maker, the
//! goldens against the real renderings, the run record
//! against the ported maker and against a recomputation from
//! the fixtures with a missing envelope file counted as
//! attempted and failed, the price against its recomputation
//! from the recorded envelopes, and the tally of a stated
//! sample against an independent recount written in this
//! file.

mod shared;

use std::collections::BTreeSet;
use std::path::PathBuf;

use grandjury::json::JsonValue;
use grandjury::makers::{full_build_poll, full_build_run_record};
use grandjury::polls::{cost_lines, load_poll_file, load_prices_text, render_stadium, tally_poll};
use grandjury::pydecimal::{PyDecimal, Rounding};
use grandjury::seats::{response_from_chat_envelope, response_from_jev_envelope};
use shared::*;

const SEATS: usize = 5;
const SAMPLE: [usize; 4] = [1, 13, 27, 40];

type Reader = fn(&grandjury::pytext::PText, &JsonValue) -> Option<grandjury::polls::Response>;

fn full() -> PathBuf {
    examples().join("full")
}

fn reader_for(prefix: &str) -> Reader {
    match prefix {
        "jev" => response_from_jev_envelope,
        _ => response_from_chat_envelope,
    }
}

fn questions() -> Vec<JsonValue> {
    match read_json(&full().join("questions.json")).as_array() {
        Some(items) => items.clone(),
        None => panic!("the questions stand as an array"),
    }
}

fn envelope_path(index: usize, prefix: &str, seat: usize) -> PathBuf {
    full()
        .join("envelopes")
        .join(format!("q{:02}-{}-{}.json", index, prefix, seat))
}

fn envelopes(index: usize, prefix: &str) -> Vec<(usize, JsonValue)> {
    let mut found = Vec::new();
    for seat in 1..=SEATS {
        let path = envelope_path(index, prefix, seat);
        if path.exists() {
            found.push((seat, read_json(&path)));
        }
    }
    found
}

fn exact_cost(envelope_exact: &JsonValue) -> PyDecimal {
    match field(field(envelope_exact, "usage"), "cost") {
        JsonValue::Num(grandjury::pynum::Num::Decimal(value)) => value.clone(),
        other => panic!("the usage cost is a decimal, got {:?}", other),
    }
}

fn recorded_chat_costs() -> Vec<PyDecimal> {
    let mut costs = Vec::new();
    for (offset, item) in questions().iter().enumerate() {
        let index = offset + 1;
        let _ = item;
        for (seat, envelope) in envelopes(index, "openai") {
            let juror = pt(&format!("openai-{}", seat));
            if response_from_chat_envelope(&juror, &envelope).is_some() {
                let exact = read_json_exact(&envelope_path(index, "openai", seat));
                costs.push(exact_cost(&exact));
            }
        }
    }
    costs
}

#[test]
fn the_full_question_set_stands_in_shape() {
    let items = questions();
    assert_eq!(items.len(), 40);
    let mut counts: Vec<usize> = Vec::new();
    for item in &items {
        let obj = item.as_object().expect("an object");
        let mut keys: Vec<String> = obj.iter().map(|(key, _)| key.display()).collect();
        keys.sort();
        assert_eq!(keys, vec!["answers", "question"]);
        assert!(!as_str(field(item, "question")).is_empty());
        let answers = match field(item, "answers") {
            JsonValue::Arr(held) => held,
            _ => panic!("answers stand as an array"),
        };
        assert!((2..=4).contains(&answers.len()));
        let distinct: BTreeSet<String> = answers.iter().map(as_str).collect();
        assert_eq!(distinct.len(), answers.len());
        for answer in answers {
            assert!(!as_str(answer).is_empty());
        }
        counts.push(answers.len());
    }
    let texts: BTreeSet<String> = items
        .iter()
        .map(|item| as_str(field(item, "question")))
        .collect();
    assert_eq!(texts.len(), 40);
    counts.sort_unstable();
    assert_eq!(counts[0], 2);
    assert_eq!(counts[counts.len() - 1], 4);
}

#[test]
fn the_full_questions_are_disjoint_from_the_pilot() {
    let pilot = read_json(&examples().join("pilot").join("questions.json"));
    let pilot_items = pilot.as_array().expect("an array");
    let pilot_texts: BTreeSet<String> = pilot_items
        .iter()
        .map(|item| as_str(field(item, "question")))
        .collect();
    assert_eq!(pilot_texts.len(), 12);
    for item in &questions() {
        assert!(!pilot_texts.contains(&as_str(field(item, "question"))));
    }
}

#[test]
fn the_envelope_inventory_matches_the_run_record() {
    let record = read_json(&full().join("run_record.json"));
    let mut names = BTreeSet::new();
    for entry in std::fs::read_dir(full().join("envelopes")).expect("the envelopes stand") {
        let name = entry
            .expect("an entry")
            .file_name()
            .to_string_lossy()
            .into_owned();
        if name.ends_with(".json") {
            names.insert(name);
        }
    }
    let mut expected = BTreeSet::new();
    for index in 1..=40 {
        for prefix in ["jev", "openai"] {
            for seat in 1..=SEATS {
                expected.insert(format!("q{:02}-{}-{}.json", index, prefix, seat));
            }
        }
    }
    assert!(names.iter().all(|name| expected.contains(name)));
    let models = field(&record, "models");
    let mut recorded = 0i64;
    let mut failed = 0i64;
    for (_, entry) in models.as_object().expect("an object") {
        recorded += as_int(field(entry, "calls_recorded"));
        failed += as_int(field(entry, "failed_calls"));
    }
    assert_eq!(recorded + failed, 400);
    for name in &names {
        let envelope = read_json(&full().join("envelopes").join(name));
        let model = as_str(field(&envelope, "model"));
        if name.contains("-jev-") {
            assert_eq!(model, "jev-1.13.0");
        } else {
            assert_eq!(model, "openai/gpt-6-luna");
        }
    }
    assert!(recorded <= names.len() as i64 && names.len() <= 400);
}

#[test]
fn every_full_poll_rebuilds_from_its_envelopes() {
    for (offset, item) in questions().iter().enumerate() {
        let index = offset + 1;
        let committed = read_json(&full().join(format!("poll-{:02}.json", index)));
        let built = full_build_poll(&examples(), index, item).expect("a built poll");
        assert_json_eq(&built, &committed);
    }
}

#[test]
fn the_full_poll_files_load_clean_and_tally() {
    for index in 1..=40 {
        let (poll, defects) = load_poll_file(&full().join(format!("poll-{:02}.json", index)));
        assert_eq!(defects, Vec::<String>::new());
        let poll = poll.expect("a poll");
        let _ = tally_poll(&poll);
    }
}

#[test]
fn the_sample_tallies_match_an_independent_recount() {
    for index in SAMPLE {
        let (poll, defects) = load_poll_file(&full().join(format!("poll-{:02}.json", index)));
        assert_eq!(defects, Vec::<String>::new());
        let poll = poll.expect("a poll");
        let tally = tally_poll(&poll);
        let valid: Vec<&grandjury::polls::Response> = poll
            .responses
            .iter()
            .filter(|entry| !entry.abstained && poll.answers.contains(&entry.answer))
            .collect();
        assert_eq!(tally.responses, poll.responses.len());
        assert_eq!(tally.valid, valid.len());
        assert_eq!(
            tally.abstained,
            poll.responses
                .iter()
                .filter(|entry| entry.abstained)
                .count()
        );
        assert_eq!(
            tally.spoiled,
            poll.responses
                .iter()
                .filter(|entry| !entry.abstained && !poll.answers.contains(&entry.answer))
                .count()
        );
        for answer in &poll.answers {
            let count = valid.iter().filter(|entry| entry.answer == *answer).count();
            let row = tally
                .rows
                .iter()
                .find(|(held, _, _)| held == answer)
                .expect("a row");
            assert_eq!(row.1, count);
        }
    }
}

#[test]
fn the_full_example_poll_is_the_first_poll() {
    assert_json_eq(
        &read_json(&examples().join("full_poll.json")),
        &read_json(&full().join("poll-01.json")),
    );
}

#[test]
fn the_full_goldens_match_the_renderings() {
    let (example, defects) = load_poll_file(&examples().join("full_poll.json"));
    assert_eq!(defects, Vec::<String>::new());
    let example = example.expect("a poll");
    let prices_text = read_text(&examples().join("full-prices.json"));
    let (table, price_defects) = load_prices_text(&prices_text);
    assert_eq!(price_defects, Vec::<String>::new());
    let table = table.expect("a table");
    let stadium = read_text(&examples().join("full-stadium-golden.txt"));
    assert_eq!(
        render_stadium(&example).expect("a stadium view").join("\n") + "\n",
        stadium
    );
    let cost = read_text(&examples().join("full-cost-golden.txt"));
    assert_eq!(
        cost_lines(&example, &table)
            .expect("no refusal")
            .expect("a cost report")
            .join("\n")
            + "\n",
        cost
    );
}

#[test]
fn the_full_price_is_the_measured_mean_chat_cost() {
    let costs = recorded_chat_costs();
    assert!(!costs.is_empty());
    let mut total = PyDecimal::zero();
    for cost in &costs {
        total = total.added_ctx(cost, 28, Rounding::HalfEven);
    }
    let count = PyDecimal::parse_literal(&costs.len().to_string()).expect("a count");
    let mean = total
        .divided_default(&count)
        .expect("a mean")
        .quantized(-6, 28, Rounding::HalfUp)
        .expect("a price at six places");
    let table = read_json(&examples().join("full-prices.json"));
    assert_json_eq(
        &table,
        &jobj(vec![(
            "prices",
            jobj(vec![("openai/gpt-6-luna", jstr(&mean.render()))]),
        )]),
    );
}

#[test]
fn the_full_run_record_recomputes_from_the_fixtures() {
    let record = read_json(&full().join("run_record.json"));
    let built = full_build_run_record(&examples(), &questions()).expect("a built record");
    assert_json_eq(&built, &record);
    assert_eq!(as_int(field(&record, "questions")), 40);
    assert_eq!(as_int(field(&record, "seats_per_question_per_model")), 5);
}

#[test]
fn the_full_run_record_counts_agree_with_the_envelopes() {
    let record = read_json(&full().join("run_record.json"));
    let models = field(&record, "models");
    for (prefix, model) in [("jev", "jev-1.13.0"), ("openai", "openai/gpt-6-luna")] {
        let entry = field(models, model);
        let reader = reader_for(prefix);
        let mut files = 0i64;
        let mut recorded = 0i64;
        let mut abstained = 0i64;
        let mut input_tokens = 0i64;
        let mut output_tokens = 0i64;
        let mut cost = PyDecimal::zero();
        for (offset, _item) in questions().iter().enumerate() {
            let index = offset + 1;
            for seat in 1..=SEATS {
                let path = envelope_path(index, prefix, seat);
                if !path.exists() {
                    continue;
                }
                files += 1;
                let envelope = read_json(&path);
                let exact = read_json_exact(&path);
                let usage = field(&envelope, "usage");
                if prefix == "jev" {
                    input_tokens += as_int(field(usage, "input_tokens"));
                    output_tokens += as_int(field(usage, "output_tokens"));
                } else {
                    input_tokens += as_int(field(usage, "prompt_tokens"));
                    output_tokens += as_int(field(usage, "completion_tokens"));
                    cost = cost.added_ctx(&exact_cost(&exact), 28, Rounding::HalfEven);
                }
                let juror = pt(&format!("{}-{}", prefix, seat));
                if let Some(response) = reader(&juror, &envelope) {
                    recorded += 1;
                    if response.abstained {
                        abstained += 1;
                    }
                }
            }
        }
        assert_eq!(
            as_str(field(entry, "route")),
            if prefix == "jev" { "jev" } else { "chat" }
        );
        assert_eq!(as_int(field(entry, "calls_attempted")), 200);
        assert_eq!(as_int(field(entry, "calls_recorded")), recorded);
        assert_eq!(as_int(field(entry, "failed_calls")), 200 - recorded);
        assert_eq!(
            as_int(field(entry, "failed_calls")),
            (200 - files) + (files - recorded)
        );
        assert_eq!(as_int(field(entry, "abstained")), abstained);
        assert_eq!(as_int(field(entry, "input_tokens")), input_tokens);
        assert_eq!(as_int(field(entry, "output_tokens")), output_tokens);
        if prefix == "openai" {
            assert_eq!(as_str(field(entry, "cost_dollars")), cost.render());
        }
    }
}

#[test]
fn the_incident_section_equals_its_input_file() {
    let record = read_json(&full().join("run_record.json"));
    let incident = read_json(&full().join("incident.json"));
    assert_json_eq(field(&record, "incident"), &incident);
}

#[test]
fn the_incident_arithmetic_holds() {
    let incident = read_json(&full().join("incident.json"));
    let record = read_json(&full().join("run_record.json"));
    let mut total = 0i64;
    for (model, entry) in incident.as_object().expect("an object") {
        let model = model.display();
        if model == "lost_token_usage" {
            continue;
        }
        let parts = as_int(field(entry, "calls_in_the_recorded_run"))
            + as_int(field(entry, "attempts_that_never_left_the_machine"))
            + as_int(field(entry, "envelopes_lost_and_seats_rerun"))
            + as_int(field(entry, "calls_discarded"))
            + as_int(field(entry, "route_verification_calls"))
            + as_int(field(entry, "diagnostic_calls"));
        assert_eq!(parts, as_int(field(entry, "attempts_in_round")));
        assert_eq!(
            as_int(field(entry, "calls_in_the_recorded_run")),
            as_int(field(
                field(field(&record, "models"), &model),
                "calls_attempted"
            ))
        );
        total += as_int(field(entry, "attempts_in_round"));
    }
    assert!(total <= 1000);
}
