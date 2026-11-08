//! Pins for GJ-3, the stadium view.
//!
//! Translated from tests/test_stadium.py. The bar scaling is
//! pinned at stated cases, the wall order is pinned line by
//! line, and the spoiled section is pinned present and absent.

mod shared;

use grandjury::json::JsonValue;
use grandjury::polls::{load_poll_text, Poll};
use shared::*;

fn response(juror: &str, answer: &str, reason: &str) -> JsonValue {
    jobj(vec![
        ("juror", jstr(juror)),
        ("model", jstr("tiny-a")),
        ("answer", jstr(answer)),
        ("reason", jstr(reason)),
    ])
}

fn abstained(juror: &str, answer: &str, reason: &str) -> JsonValue {
    let mut entry = response(juror, answer, reason);
    if let JsonValue::Obj(pairs) = &mut entry {
        pairs.push((pt("abstained"), JsonValue::Bool(true)));
    }
    entry
}

fn load(data: &JsonValue) -> Poll {
    let (poll, defects) = load_poll_text(&dumps_default(data));
    assert_eq!(defects, Vec::<String>::new());
    poll.expect("a poll")
}

fn poll_of(question: &str, answers: Vec<&str>, responses: Vec<JsonValue>) -> JsonValue {
    jobj(vec![
        ("question", jstr(question)),
        ("answers", jarr(answers.into_iter().map(jstr).collect())),
        ("responses", jarr(responses)),
    ])
}

fn bar(count: usize) -> String {
    "#".repeat(count)
}

/// The stadium lines of a poll, unwrapped from the refusal
/// channel FIX-GJ-15 gives the reading, as a named mechanical
/// consequence of the repair. The refusal itself is pinned in
/// the fencing suite against the reading directly.
fn render_stadium(poll: &Poll) -> Vec<String> {
    grandjury::polls::render_stadium(poll).expect("a stadium view")
}

#[test]
fn the_stadium_view_of_a_small_poll_line_by_line() {
    let poll = load(&poll_of(
        "Which door should the agent pick?",
        vec!["red", "blue"],
        vec![
            response("juror-1", "red", "the shortest path"),
            response("juror-2", "blue", "a calmer colour"),
            response("juror-3", "red", "matches the brief"),
        ],
    ));
    assert_eq!(
        render_stadium(&poll),
        vec![
            "question, Which door should the agent pick?".to_string(),
            "responses, 3".to_string(),
            format!("red |{}| 2, 66.7%", bar(40)),
            format!("blue |{}| 1, 33.3%", bar(20)),
            "wall of reasons".to_string(),
            "red".to_string(),
            "  juror-1, the shortest path".to_string(),
            "  juror-3, matches the brief".to_string(),
            "blue".to_string(),
            "  juror-2, a calmer colour".to_string(),
        ]
    );
}

#[test]
fn the_bar_scales_against_the_largest_count_at_width_40() {
    let poll = load(&poll_of(
        "q",
        vec!["red", "blue", "green"],
        vec![
            response("juror-1", "red", "a reason"),
            response("juror-2", "red", "a reason"),
            response("juror-3", "red", "a reason"),
            response("juror-4", "blue", "a reason"),
        ],
    ));
    let lines = render_stadium(&poll);
    assert_eq!(lines[2], format!("red |{}| 3, 75.0%", bar(40)));
    assert_eq!(lines[3], format!("blue |{}| 1, 25.0%", bar(13)));
    assert_eq!(lines[4], "green || 0, 0.0%");
}

#[test]
fn every_bar_is_empty_when_no_response_is_valid() {
    let poll = load(&poll_of(
        "q",
        vec!["red", "blue"],
        vec![response("juror-1", "purple", "a reason")],
    ));
    let lines = render_stadium(&poll);
    assert_eq!(lines[2], "red || 0, 0.0%");
    assert_eq!(lines[3], "blue || 0, 0.0%");
}

#[test]
fn an_answer_nobody_chose_stands_in_the_wall_with_no_reasons() {
    let poll = load(&poll_of(
        "q",
        vec!["red", "blue", "green"],
        vec![response("juror-1", "blue", "a calmer colour")],
    ));
    let lines = render_stadium(&poll);
    let start = lines
        .iter()
        .position(|line| line == "wall of reasons")
        .expect("the wall stands");
    assert_eq!(
        lines[start..],
        [
            "wall of reasons",
            "red",
            "blue",
            "  juror-1, a calmer colour",
            "green",
        ]
    );
}

#[test]
fn the_wall_groups_in_offered_order_not_response_order() {
    let poll = load(&poll_of(
        "q",
        vec!["red", "blue"],
        vec![
            response("juror-1", "blue", "first cast"),
            response("juror-2", "red", "second cast"),
            response("juror-3", "blue", "third cast"),
        ],
    ));
    let lines = render_stadium(&poll);
    let start = lines
        .iter()
        .position(|line| line == "wall of reasons")
        .expect("the wall stands");
    assert_eq!(
        lines[start..],
        [
            "wall of reasons",
            "red",
            "  juror-2, second cast",
            "blue",
            "  juror-1, first cast",
            "  juror-3, third cast",
        ]
    );
}

#[test]
fn the_spoiled_section_names_each_spoiled_response() {
    let poll = load(&poll_of(
        "q",
        vec!["red", "blue"],
        vec![
            response("juror-1", "red", "a reason"),
            response("juror-2", "purple", "a reason"),
            response("juror-3", "teal", "a reason"),
        ],
    ));
    let lines = render_stadium(&poll);
    assert_eq!(
        lines[lines.len() - 3..],
        ["spoiled, 2", "  juror-2, purple", "  juror-3, teal"]
    );
}

#[test]
fn no_spoiled_section_stands_when_nothing_is_spoiled() {
    let poll = load(&poll_of(
        "q",
        vec!["red", "blue"],
        vec![response("juror-1", "red", "a reason")],
    ));
    let lines = render_stadium(&poll);
    assert!(!lines.iter().any(|line| line.starts_with("spoiled")));
}

#[test]
fn the_same_poll_renders_byte_identical_lines_twice() {
    let poll = load(&poll_of(
        "q",
        vec!["red", "blue"],
        vec![
            response("juror-1", "red", "a reason"),
            response("juror-2", "purple", "a reason"),
        ],
    ));
    let first = render_stadium(&poll).join("\n");
    let second = render_stadium(&poll).join("\n");
    assert_eq!(first, second);
}

#[test]
fn the_abstained_section_stands_between_wall_and_spoiled() {
    let poll = load(&poll_of(
        "q",
        vec!["red", "blue"],
        vec![
            response("juror-1", "red", "the shortest path"),
            abstained("juror-2", "blue", "too close to call"),
            response("juror-3", "purple", "answering from habit"),
        ],
    ));
    assert_eq!(
        render_stadium(&poll),
        vec![
            "question, q".to_string(),
            "responses, 3".to_string(),
            format!("red |{}| 1, 100.0%", bar(40)),
            "blue || 0, 0.0%".to_string(),
            "wall of reasons".to_string(),
            "red".to_string(),
            "  juror-1, the shortest path".to_string(),
            "blue".to_string(),
            "abstained, 1".to_string(),
            "  juror-2, too close to call".to_string(),
            "spoiled, 1".to_string(),
            "  juror-3, purple".to_string(),
        ]
    );
}

#[test]
fn no_abstained_section_stands_when_nobody_abstained() {
    let poll = load(&poll_of(
        "q",
        vec!["red", "blue"],
        vec![response("juror-1", "red", "a reason")],
    ));
    let lines = render_stadium(&poll);
    assert!(!lines.iter().any(|line| line.starts_with("abstained")));
}

#[test]
fn the_abstained_section_lists_each_abstention_in_order() {
    let poll = load(&poll_of(
        "q",
        vec!["red", "blue"],
        vec![
            abstained("juror-1", "red", "first held back"),
            response("juror-2", "red", "a plain answer"),
            abstained("juror-3", "blue", "second held back"),
        ],
    ));
    let lines = render_stadium(&poll);
    let opening = lines
        .iter()
        .position(|line| line == "abstained, 2")
        .expect("the abstained section stands");
    assert_eq!(lines[opening + 1], "  juror-1, first held back");
    assert_eq!(lines[opening + 2], "  juror-3, second held back");
}
