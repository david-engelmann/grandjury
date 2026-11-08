//! Pins for the recorded example poll and its golden
//! renderings.
//!
//! Translated from tests/test_examples.py. The example is
//! the first poll of the resampled set of FIX-GJ-19, one
//! hundred recorded responses across two offered answers
//! and two models, one of them unpriced, with fifty
//! abstained responses and no spoiled responses. The
//! goldens are held byte identical against the renderings
//! the code produces today.

mod shared;

use grandjury::polls::{cost_lines, load_poll_file, load_prices_file, render_stadium, tally_poll};
use shared::*;

fn model_set(poll: &grandjury::polls::Poll) -> Vec<String> {
    let mut models: Vec<String> = poll.responses.iter().map(|r| r.model.display()).collect();
    models.sort();
    models.dedup();
    models
}

#[test]
fn the_example_poll_loads_clean_at_its_stated_shape() {
    let (poll, defects) = load_poll_file(&examples().join("recorded_poll.json"));
    assert_eq!(defects, Vec::<String>::new());
    let poll = poll.expect("a poll");
    assert_eq!(poll.answers.len(), 2);
    assert_eq!(poll.responses.len(), 100);
    assert_eq!(model_set(&poll), vec!["jev-1.13.0", "openai/gpt-6-luna"]);
    let tally = tally_poll(&poll);
    assert_eq!(tally.valid, 50);
    assert_eq!(tally.spoiled, 0);
    assert_eq!(tally.abstained, 50);
}

#[test]
fn the_example_prices_load_clean() {
    let (table, defects) = load_prices_file(&examples().join("prices.json"));
    assert_eq!(defects, Vec::<String>::new());
    let table = table.expect("a table");
    let mut models: Vec<String> = table.models().iter().map(|m| m.display()).collect();
    models.sort();
    assert_eq!(models, vec!["openai/gpt-6-luna"]);
}

#[test]
fn the_stadium_golden_is_byte_identical() {
    let (poll, defects) = load_poll_file(&examples().join("recorded_poll.json"));
    assert_eq!(defects, Vec::<String>::new());
    let poll = poll.expect("a poll");
    let golden = read_text(&examples().join("stadium-golden.txt"));
    assert_eq!(
        render_stadium(&poll).expect("a stadium view").join("\n") + "\n",
        golden
    );
}

#[test]
fn the_cost_golden_is_byte_identical() {
    let (poll, defects) = load_poll_file(&examples().join("recorded_poll.json"));
    assert_eq!(defects, Vec::<String>::new());
    let poll = poll.expect("a poll");
    let (table, price_defects) = load_prices_file(&examples().join("prices.json"));
    assert_eq!(price_defects, Vec::<String>::new());
    let table = table.expect("a table");
    let golden = read_text(&examples().join("cost-golden.txt"));
    assert_eq!(
        cost_lines(&poll, &table)
            .expect("no refusal")
            .expect("a cost report")
            .join("\n")
            + "\n",
        golden
    );
}

#[test]
fn the_pilot_example_poll_loads_clean_at_its_stated_shape() {
    let (poll, defects) = load_poll_file(&examples().join("pilot_poll.json"));
    assert_eq!(defects, Vec::<String>::new());
    let poll = poll.expect("a poll");
    assert_eq!(poll.answers.len(), 2);
    assert_eq!(poll.responses.len(), 10);
    assert_eq!(model_set(&poll), vec!["jev-1.13.0", "openai/gpt-6-luna"]);
    let tally = tally_poll(&poll);
    assert_eq!(tally.valid, 5);
    assert_eq!(tally.spoiled, 0);
    assert_eq!(tally.abstained, 5);
}

#[test]
fn the_pilot_prices_load_clean() {
    let (table, defects) = load_prices_file(&examples().join("pilot-prices.json"));
    assert_eq!(defects, Vec::<String>::new());
    let table = table.expect("a table");
    let models: Vec<String> = table.models().iter().map(|m| m.display()).collect();
    assert_eq!(models, vec!["openai/gpt-6-luna"]);
}

#[test]
fn the_pilot_stadium_golden_is_byte_identical() {
    let (poll, defects) = load_poll_file(&examples().join("pilot_poll.json"));
    assert_eq!(defects, Vec::<String>::new());
    let poll = poll.expect("a poll");
    let golden = read_text(&examples().join("pilot-stadium-golden.txt"));
    assert_eq!(
        render_stadium(&poll).expect("a stadium view").join("\n") + "\n",
        golden
    );
}

#[test]
fn the_pilot_cost_golden_is_byte_identical() {
    let (poll, defects) = load_poll_file(&examples().join("pilot_poll.json"));
    assert_eq!(defects, Vec::<String>::new());
    let poll = poll.expect("a poll");
    let (table, price_defects) = load_prices_file(&examples().join("pilot-prices.json"));
    assert_eq!(price_defects, Vec::<String>::new());
    let table = table.expect("a table");
    let golden = read_text(&examples().join("pilot-cost-golden.txt"));
    assert_eq!(
        cost_lines(&poll, &table)
            .expect("no refusal")
            .expect("a cost report")
            .join("\n")
            + "\n",
        golden
    );
}
