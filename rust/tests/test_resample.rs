//! Pins for FIX-GJ-19, the resampled fixture and the
//! example poll.
//!
//! The pins stand as relations over the committed
//! fixture, as the amendment states them, and not as
//! hand counted goldens. Every poll of the resampled set
//! loads clean, every response of the set stands among
//! the responses the GJ-5 seat readers produce from the
//! recorded pilot envelopes of its question and its
//! model, each poll holds one hundred responses with
//! fifty standing under each model string, the census
//! record line over the set recomputes from the fixture
//! polls themselves, the example poll stands byte
//! identical to the first poll of the set, and the cost
//! golden over the example poll recomputes from the
//! price table. Every pin runs offline against the
//! recorded envelopes, and no pin makes a live call.

mod shared;

use std::path::Path;

use grandjury::census::census_lines;
use grandjury::polls::{cost_lines, load_poll_file, load_prices_file, Poll};
use grandjury::pydecimal::PyDecimal;
use grandjury::seats::{response_from_chat_envelope, response_from_jev_envelope};
use shared::*;

const JEV_MODEL: &str = "jev-1.13.0";
const CHAT_MODEL: &str = "openai/gpt-6-luna";

fn load_poll(path: &Path) -> Poll {
    let (poll, defects) = load_poll_file(path);
    assert!(defects.is_empty(), "{} {:?}", path.display(), defects);
    poll.expect("a poll that loads")
}

fn resampled_polls() -> Vec<Poll> {
    (1..=12)
        .map(|index| {
            load_poll(
                &examples()
                    .join("resampled")
                    .join(format!("poll-{:02}.json", index)),
            )
        })
        .collect()
}

/// The responses the GJ-5 seat readers produce from the
/// five recorded pilot envelopes of one question and one
/// route, as triples of the answer, the abstained
/// standing, and the reason.
fn produced_triples(question: usize, route: &str) -> Vec<(String, bool, String)> {
    let juror = pt("seat-check");
    let mut triples = Vec::new();
    for seat in 1..=5 {
        let envelope = read_json(&examples().join(format!(
            "pilot/envelopes/q{:02}-{}-{}.json",
            question, route, seat
        )));
        let response = if route == "jev" {
            response_from_jev_envelope(&juror, &envelope)
        } else {
            response_from_chat_envelope(&juror, &envelope)
        };
        let response = response.expect("a recorded response");
        triples.push((
            response.answer.display(),
            response.abstained,
            response.reason.display(),
        ));
    }
    triples
}

#[test]
fn every_poll_of_the_resampled_set_loads_clean() {
    let polls = resampled_polls();
    assert_eq!(polls.len(), 12);
    let questions = read_json(&examples().join("pilot/questions.json"));
    let items = questions.as_array().expect("a questions array");
    assert_eq!(items.len(), 12);
    for (poll, entry) in polls.iter().zip(items.iter()) {
        assert_eq!(poll.question.display(), as_str(field(entry, "question")));
        let offered: Vec<String> = field(entry, "answers")
            .as_array()
            .expect("an answers array")
            .iter()
            .map(as_str)
            .collect();
        let held: Vec<String> = poll.answers.iter().map(|answer| answer.display()).collect();
        assert_eq!(held, offered);
    }
}

#[test]
fn each_poll_holds_one_hundred_responses_fifty_under_each_model() {
    for poll in resampled_polls() {
        assert_eq!(poll.responses.len(), 100);
        let jev = poll
            .responses
            .iter()
            .filter(|response| response.model.display() == JEV_MODEL)
            .count();
        let chat = poll
            .responses
            .iter()
            .filter(|response| response.model.display() == CHAT_MODEL)
            .count();
        assert_eq!(jev, 50);
        assert_eq!(chat, 50);
        // The responses stand with the Jev seats first in
        // seat order and then the chat seats in seat
        // order, as the makers assemble a poll.
        for (index, response) in poll.responses.iter().enumerate() {
            let expected = if index < 50 {
                format!("jev-{}", index + 1)
            } else {
                format!("openai-{}", index - 49)
            };
            assert_eq!(response.juror.display(), expected);
        }
    }
}

#[test]
fn every_response_stands_among_the_produced_responses_of_its_envelopes() {
    let polls = resampled_polls();
    for (offset, poll) in polls.iter().enumerate() {
        let number = offset + 1;
        let jev_triples = produced_triples(number, "jev");
        let chat_triples = produced_triples(number, "openai");
        for response in &poll.responses {
            let triple = (
                response.answer.display(),
                response.abstained,
                response.reason.display(),
            );
            let produced = if response.model.display() == JEV_MODEL {
                &jev_triples
            } else {
                &chat_triples
            };
            assert!(
                produced.contains(&triple),
                "poll {} response {} stands outside its envelopes",
                number,
                response.juror.display()
            );
        }
    }
}

#[test]
fn the_census_record_line_over_the_set_recomputes_from_the_polls() {
    let polls = resampled_polls();
    let mut recorded = 0usize;
    let mut valid = 0usize;
    let mut abstained = 0usize;
    let mut spoiled = 0usize;
    for poll in &polls {
        let offered: Vec<String> = poll.answers.iter().map(|answer| answer.display()).collect();
        recorded += poll.responses.len();
        for response in &poll.responses {
            if response.abstained {
                abstained += 1;
            } else if offered.contains(&response.answer.display()) {
                valid += 1;
            } else {
                spoiled += 1;
            }
        }
    }
    let sets = vec![(pt("resampled"), polls)];
    let lines = census_lines(&sets).expect("a reading");
    let record = lines
        .iter()
        .find(|line| line.starts_with("record, "))
        .expect("a record line");
    assert_eq!(
        record,
        &format!(
            "record, polls 12, recorded {}, valid {}, abstained {}, spoiled {}",
            recorded, valid, abstained, spoiled
        )
    );
}

#[test]
fn the_example_poll_stands_byte_identical_to_the_first_resampled_poll() {
    let example = std::fs::read(examples().join("recorded_poll.json")).expect("an example poll");
    let first = std::fs::read(examples().join("resampled/poll-01.json")).expect("a first poll");
    assert_eq!(example, first);
}

#[test]
fn the_cost_golden_recomputes_from_the_price_table() {
    let poll = load_poll(&examples().join("recorded_poll.json"));
    let (table, defects) = load_prices_file(&examples().join("prices.json"));
    assert!(defects.is_empty(), "{:?}", defects);
    let table = table.expect("a price table");
    let mut priced = 0usize;
    let mut unpriced = 0usize;
    let mut total = PyDecimal::zero();
    for response in &poll.responses {
        match table.get(&response.model) {
            Some(price) => {
                priced += 1;
                total = total.added_exact(price);
            }
            None => unpriced += 1,
        }
    }
    let mut expected = vec![
        format!("question, {}", poll.question.display()),
        format!("responses, {}", poll.responses.len()),
        format!("priced, {} responses, {} dollars", priced, total.render()),
        format!("unpriced, {} responses", unpriced),
    ];
    let mut unpriced_models: Vec<String> = Vec::new();
    for response in &poll.responses {
        if table.get(&response.model).is_none() {
            let model = response.model.display();
            if !unpriced_models.contains(&model) {
                unpriced_models.push(model);
            }
        }
    }
    for model in &unpriced_models {
        let count = poll
            .responses
            .iter()
            .filter(|response| response.model.display() == *model)
            .count();
        expected.push(format!("unpriced model, {}, {} responses", model, count));
    }
    let golden = read_text(&examples().join("cost-golden.txt"));
    let held: Vec<String> = golden.lines().map(|line| line.to_string()).collect();
    assert_eq!(expected, held);
    let lines = cost_lines(&poll, &table)
        .expect("no refusal")
        .expect("a cost report");
    assert_eq!(lines, held);
}
