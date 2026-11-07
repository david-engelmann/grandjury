//! Pins for GJ-8, the agreement reading.
//!
//! Translated from tests/test_agreement.py. The reading
//! states each jury share as an exact fraction, the plurality
//! under the stated tie rule, and the closing agreement count.
//! Every pin here builds its polls by hand, so each one fails
//! under the negation of the rule it names. The golden pin
//! holds the reading of the forty recorded full polls to the
//! committed golden byte for byte, through the function and
//! through the command.

mod shared;

use grandjury::agreement::agreement_lines;
use grandjury::polls::{load_poll_file, Poll, Response};
use shared::*;

fn response(juror: &str, model: &str, answer: &str, abstained: bool) -> Response {
    Response {
        juror: pt(juror),
        model: pt(model),
        answer: pt(answer),
        reason: pt("a reason"),
        abstained,
    }
}

fn poll(question: &str, answers: Vec<&str>, responses: Vec<Response>) -> Poll {
    Poll {
        question: pt(question),
        answers: answers.into_iter().map(pt).collect(),
        responses,
    }
}

fn door_poll() -> Poll {
    poll(
        "Which door?",
        vec!["red", "blue", "green"],
        vec![
            response("jev-1", "jev-a", "red", false),
            response("jev-2", "jev-a", "red", false),
            response("jev-3", "jev-a", "blue", false),
            response("jev-4", "jev-a", "purple", false),
            response("jev-5", "jev-a", "red", true),
            response("openai-1", "chat-b", "blue", false),
            response("openai-2", "chat-b", "blue", false),
            response("openai-3", "chat-b", "red", false),
            response("openai-4", "chat-b", "green", false),
            response("openai-5", "chat-b", "green", false),
        ],
    )
}

fn door_poll_json() -> grandjury::json::JsonValue {
    door_poll().to_json()
}

#[test]
fn the_reading_of_one_poll_stands_line_for_line() {
    assert_eq!(
        agreement_lines(&[door_poll()]).expect("a reading"),
        vec![
            "question, Which door?",
            "jury, jev-a, recorded 5, valid 3, abstained 1, spoiled 1",
            "  red, 2, 2/3, 66.7%",
            "  blue, 1, 1/3, 33.3%",
            "  green, 0, 0, 0.0%",
            "plurality, jev-a, red",
            "jury, chat-b, recorded 5, valid 5, abstained 0, spoiled 0",
            "  red, 1, 1/5, 20.0%",
            "  blue, 2, 2/5, 40.0%",
            "  green, 2, 2/5, 40.0%",
            "plurality, chat-b, none",
            "pluralities, not counted",
            "agreements, 0 of 0",
        ]
    );
}

#[test]
fn a_tie_at_the_top_leaves_no_plurality() {
    let tied = poll(
        "Tied?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", false),
            response("jev-2", "jev-a", "b", false),
            response("openai-1", "chat-b", "a", false),
            response("openai-2", "chat-b", "a", false),
        ],
    );
    let lines = agreement_lines(&[tied]).expect("a reading");
    assert!(lines.contains(&"plurality, jev-a, none".to_string()));
    assert!(lines.contains(&"plurality, chat-b, a".to_string()));
    assert!(lines.contains(&"pluralities, not counted".to_string()));
    assert_eq!(lines.last().map(String::as_str), Some("agreements, 0 of 0"));
}

#[test]
fn a_tie_below_the_top_leaves_the_plurality_standing() {
    let tied_below = poll(
        "Clear at the top?",
        vec!["a", "b", "c"],
        vec![
            response("jev-1", "jev-a", "a", false),
            response("jev-2", "jev-a", "a", false),
            response("jev-3", "jev-a", "a", false),
            response("jev-4", "jev-a", "b", false),
            response("jev-5", "jev-a", "b", false),
            response("openai-1", "chat-b", "a", false),
        ],
    );
    let lines = agreement_lines(&[tied_below]).expect("a reading");
    assert!(lines.contains(&"plurality, jev-a, a".to_string()));
}

#[test]
fn agree_and_differ_close_the_count() {
    let agree = poll(
        "Same?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", false),
            response("jev-2", "jev-a", "a", false),
            response("openai-1", "chat-b", "a", false),
            response("openai-2", "chat-b", "b", false),
            response("openai-3", "chat-b", "a", false),
        ],
    );
    let differ = poll(
        "Apart?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "b", false),
            response("openai-1", "chat-b", "a", false),
        ],
    );
    let lines = agreement_lines(&[agree, differ, door_poll()]).expect("a reading");
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.as_str() == "pluralities, agree")
            .count(),
        1
    );
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.as_str() == "pluralities, differ")
            .count(),
        1
    );
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.as_str() == "pluralities, not counted")
            .count(),
        1
    );
    assert_eq!(lines.last().map(String::as_str), Some("agreements, 1 of 2"));
}

#[test]
fn a_jury_with_no_valid_responses_holds_no_plurality() {
    let silent = poll(
        "Silent?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", true),
            response("jev-2", "jev-a", "b", true),
            response("openai-1", "chat-b", "a", false),
        ],
    );
    let lines = agreement_lines(&[silent]).expect("a reading");
    assert!(lines.contains(&"jury, jev-a, recorded 2, valid 0, abstained 2, spoiled 0".to_string()));
    assert!(lines.contains(&"  a, 0, 0, 0.0%".to_string()));
    assert!(lines.contains(&"  b, 0, 0, 0.0%".to_string()));
    assert!(lines.contains(&"plurality, jev-a, none".to_string()));
    assert!(lines.contains(&"pluralities, not counted".to_string()));
}

#[test]
fn a_zero_valid_jury_in_a_one_answer_poll_holds_no_plurality() {
    let one_answer = poll(
        "Only one way?",
        vec!["a"],
        vec![
            response("jev-1", "jev-a", "a", true),
            response("jev-2", "jev-a", "a", true),
            response("openai-1", "chat-b", "a", true),
            response("openai-2", "chat-b", "b", false),
        ],
    );
    assert_eq!(
        agreement_lines(&[one_answer]).expect("a reading"),
        vec![
            "question, Only one way?",
            "jury, jev-a, recorded 2, valid 0, abstained 2, spoiled 0",
            "  a, 0, 0, 0.0%",
            "plurality, jev-a, none",
            "jury, chat-b, recorded 2, valid 0, abstained 1, spoiled 1",
            "  a, 0, 0, 0.0%",
            "plurality, chat-b, none",
            "pluralities, not counted",
            "agreements, 0 of 0",
        ]
    );
}

#[test]
fn two_zero_valid_juries_in_a_two_answer_poll_hold_no_plurality() {
    let hushed = poll(
        "Hushed?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", true),
            response("openai-1", "chat-b", "not offered", false),
        ],
    );
    let lines = agreement_lines(&[hushed]).expect("a reading");
    assert!(lines.contains(&"plurality, jev-a, none".to_string()));
    assert!(lines.contains(&"plurality, chat-b, none".to_string()));
    assert!(lines.contains(&"pluralities, not counted".to_string()));
    assert_eq!(lines.last().map(String::as_str), Some("agreements, 0 of 0"));
}

#[test]
fn spoiled_and_abstained_stand_outside_the_shares() {
    let mixed = poll(
        "Mixed?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", false),
            response("jev-2", "jev-a", "a", false),
            response("jev-3", "jev-a", "not offered", false),
            response("jev-4", "jev-a", "b", true),
            response("openai-1", "chat-b", "b", false),
        ],
    );
    let lines = agreement_lines(&[mixed]).expect("a reading");
    assert!(lines.contains(&"jury, jev-a, recorded 4, valid 2, abstained 1, spoiled 1".to_string()));
    assert!(lines.contains(&"  a, 2, 1, 100.0%".to_string()));
    assert!(lines.contains(&"  b, 0, 0, 0.0%".to_string()));
}

#[test]
fn the_share_renders_lowest_terms_and_halves_up() {
    let mut responses = vec![response("jev-1", "jev-a", "a", false)];
    for i in 2..17 {
        responses.push(response(&format!("jev-{}", i), "jev-a", "b", false));
    }
    responses.push(response("openai-1", "chat-b", "a", false));
    let sixteen = poll("Sixteenths?", vec!["a", "b"], responses);
    let lines = agreement_lines(&[sixteen]).expect("a reading");
    assert!(lines.contains(&"  a, 1, 1/16, 6.3%".to_string()));
    assert!(lines.contains(&"  b, 15, 15/16, 93.8%".to_string()));
}

#[test]
fn juries_stand_in_first_appearance_order() {
    let reordered = poll(
        "Order?",
        vec!["a", "b"],
        vec![
            response("openai-1", "chat-b", "a", false),
            response("jev-1", "jev-a", "a", false),
        ],
    );
    let lines = agreement_lines(&[reordered]).expect("a reading");
    let chat_at = lines
        .iter()
        .position(|line| line.starts_with("jury, chat-b"))
        .expect("the chat jury stands");
    let jev_at = lines
        .iter()
        .position(|line| line.starts_with("jury, jev-a"))
        .expect("the jev jury stands");
    assert!(chat_at < jev_at);
    assert!(lines.contains(&"pluralities, agree".to_string()));
}

#[test]
fn three_juries_are_not_counted() {
    let crowd = poll(
        "Crowd?",
        vec!["a", "b"],
        vec![
            response("one-1", "model-one", "a", false),
            response("two-1", "model-two", "a", false),
            response("three-1", "model-three", "a", false),
        ],
    );
    let lines = agreement_lines(&[crowd]).expect("a reading");
    assert!(lines.contains(&"pluralities, not counted".to_string()));
    assert_eq!(lines.last().map(String::as_str), Some("agreements, 0 of 0"));
}

#[test]
fn one_jury_is_not_counted() {
    let solo = poll(
        "Solo?",
        vec!["a", "b"],
        vec![response("jev-1", "jev-a", "a", false)],
    );
    let lines = agreement_lines(&[solo]).expect("a reading");
    assert!(lines.contains(&"pluralities, not counted".to_string()));
    assert_eq!(lines.last().map(String::as_str), Some("agreements, 0 of 0"));
}

#[test]
fn a_poll_with_no_responses_reads_not_counted() {
    let empty = poll("Nobody?", vec!["a", "b"], vec![]);
    assert_eq!(
        agreement_lines(&[empty]).expect("a reading"),
        vec![
            "question, Nobody?",
            "pluralities, not counted",
            "agreements, 0 of 0",
        ]
    );
}

#[test]
fn the_agreement_command_prints_the_reading() {
    let scratch = TempDir::new("agreement-command");
    let first = scratch.write_json("one.json", &door_poll_json());
    let second_data = poll(
        "Apart?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "b", false),
            response("openai-1", "chat-b", "a", false),
        ],
    );
    let second = scratch.write_json("two.json", &second_data.to_json());
    let (out, _, code) = run_cli(&["agreement", &path_arg(&first), &path_arg(&second)]);
    assert_eq!(code, 0);
    assert!(out.contains("question, Which door?"));
    assert!(out.trim_end().ends_with("agreements, 0 of 1"));
}

#[test]
fn the_agreement_command_names_a_malformed_poll() {
    let scratch = TempDir::new("agreement-bad");
    let good = scratch.write_json("good.json", &door_poll_json());
    let mut bad = door_poll();
    bad.question = pt("");
    let bad_path = scratch.write_json("bad.json", &bad.to_json());
    let (out, _, code) = run_cli(&["agreement", &path_arg(&good), &path_arg(&bad_path)]);
    assert_eq!(code, 1);
    assert!(out.contains("invalid poll"));
    assert!(out.contains("defect field question is empty"));
    assert!(!out.contains("question, Which door?"));
    assert!(!out.contains("agreements,"));
}

#[test]
fn the_agreement_command_names_a_missing_file() {
    let scratch = TempDir::new("agreement-missing");
    let missing = scratch.path.join("missing.json");
    let (out, _, code) = run_cli(&["agreement", &path_arg(&missing)]);
    assert_eq!(code, 1);
    assert!(out.contains("defect file cannot be read"));
}

fn full_poll_paths() -> Vec<String> {
    (1..=40)
        .map(|index| {
            path_arg(
                &examples()
                    .join("full")
                    .join(format!("poll-{:02}.json", index)),
            )
        })
        .collect()
}

#[test]
fn the_agreement_golden_matches_the_function() {
    let mut polls = Vec::new();
    for path in full_poll_paths() {
        let (loaded, defects) = load_poll_file(std::path::Path::new(&path));
        assert_eq!(defects, Vec::<String>::new());
        polls.push(loaded.expect("a poll"));
    }
    let golden = read_text(&examples().join("full-agreement-golden.txt"));
    assert_eq!(
        agreement_lines(&polls).expect("a reading").join("\n") + "\n",
        golden
    );
}

#[test]
fn the_agreement_golden_matches_the_command() {
    let mut args: Vec<&str> = vec!["agreement"];
    let paths = full_poll_paths();
    args.extend(paths.iter().map(String::as_str));
    let (out, _, code) = run_cli(&args);
    assert_eq!(code, 0);
    let golden = read_text(&examples().join("full-agreement-golden.txt"));
    assert_eq!(out, golden);
}

#[test]
fn the_agreement_golden_carries_no_verdict_vocabulary() {
    const VERDICT_WORDS: [&str; 15] = [
        "verdict",
        "winner",
        "wins",
        "correct",
        "incorrect",
        "wrong",
        "best",
        "worst",
        "rank",
        "ranking",
        "ranked",
        "beats",
        "champion",
        "loser",
        "loses",
    ];
    let golden = read_text(&examples().join("full-agreement-golden.txt"));
    let words: Vec<String> = golden
        .to_lowercase()
        .split(|c: char| !c.is_ascii_lowercase())
        .filter(|word| !word.is_empty())
        .map(|word| word.to_string())
        .collect();
    for word in VERDICT_WORDS {
        assert!(!words.contains(&word.to_string()), "{}", word);
    }
}
