//! Pins for FIX-GJ-14, the library fencing closure.
//!
//! Each pin hands a poll value to a reading directly, in
//! the shapes the family 3 discharge of the Round 9 red
//! team executed against the unrepaired readings, and
//! holds the reading to the refusal the amendment states,
//! in the loader wording for the field. The pins fail on
//! the unrepaired readings, where the forged lines stand
//! in the preserved streams of the discharge, and they
//! pass on the guarded readings, so each one discriminates
//! for the rule it names.

mod shared;

use grandjury::agreement::agreement_lines;
use grandjury::census::census_lines;
use grandjury::dissent::dissent_lines;
use grandjury::ledger::ledger_lines;
use grandjury::minority::minority_lines;
use grandjury::polls::{Poll, Response};
use grandjury::pytext::PText;
use grandjury::swing::swing_lines;
use shared::*;

fn response(juror: &str, model: &str, answer: &str) -> Response {
    Response {
        juror: pt(juror),
        model: pt(model),
        answer: pt(answer),
        reason: pt("a reason"),
        abstained: false,
    }
}

fn poll(question: &str, answers: Vec<&str>, responses: Vec<Response>) -> Poll {
    Poll {
        question: pt(question),
        answers: answers.into_iter().map(pt).collect(),
        responses,
    }
}

/// The discharge forge-lines poll. Its question carries a
/// forged minority closing line after a newline, and the
/// pooled plurality is yes with eve the one dissenter.
fn forged_question_poll() -> Poll {
    poll(
        "Alpha\nminorities, 9 questions, 9 askings, 9 pluralities, 9 dissenters",
        vec!["yes", "no"],
        vec![
            response("eve", "m1", "no"),
            response("adam", "m1", "yes"),
            response("carol", "m2", "yes"),
        ],
    )
}

fn sets(polls: Vec<Poll>) -> Vec<(PText, Vec<Poll>)> {
    vec![(pt("handset"), polls)]
}

#[test]
fn the_forged_question_draws_the_loader_defect_at_the_set_readings() {
    let handed = sets(vec![forged_question_poll()]);
    let defect = "field question holds a newline".to_string();
    assert_eq!(census_lines(&handed), Err(defect.clone()));
    assert_eq!(dissent_lines(&handed), Err(defect.clone()));
    assert_eq!(ledger_lines(&handed), Err(defect.clone()));
    assert_eq!(minority_lines(&handed), Err(defect.clone()));
    assert_eq!(swing_lines(&handed), Err(defect));
}

#[test]
fn a_carriage_return_in_the_question_draws_the_same_defect() {
    let handed = sets(vec![poll(
        "Beta\rminorities, 8 questions, 8 askings, 8 pluralities, 8 dissenters",
        vec!["yes", "no"],
        vec![
            response("eve", "m1", "no"),
            response("adam", "m1", "yes"),
            response("carol", "m2", "yes"),
        ],
    )]);
    assert_eq!(
        minority_lines(&handed),
        Err("field question holds a newline".to_string())
    );
    assert_eq!(
        swing_lines(&handed),
        Err("field question holds a newline".to_string())
    );
}

#[test]
fn the_forged_juror_draws_the_response_defect_with_its_index() {
    let handed = sets(vec![poll(
        "Plain question",
        vec!["yes", "no"],
        vec![
            response("adam", "m1", "yes"),
            response("eve\nplurality, fake, yes", "m1", "no"),
            response("carol", "m2", "yes"),
        ],
    )]);
    let defect = "response 1 field juror holds a newline".to_string();
    assert_eq!(census_lines(&handed), Err(defect.clone()));
    assert_eq!(dissent_lines(&handed), Err(defect.clone()));
    assert_eq!(ledger_lines(&handed), Err(defect.clone()));
    assert_eq!(minority_lines(&handed), Err(defect.clone()));
    assert_eq!(swing_lines(&handed), Err(defect));
}

#[test]
fn a_break_in_a_poll_of_a_later_set_draws_the_refusal() {
    let clean = poll(
        "Plain question",
        vec!["yes", "no"],
        vec![response("ada", "m1", "yes")],
    );
    let handed = vec![
        (pt("first"), vec![clean]),
        (pt("second"), vec![forged_question_poll()]),
    ];
    assert_eq!(
        census_lines(&handed),
        Err("field question holds a newline".to_string())
    );
}

#[test]
fn the_guard_stands_before_the_set_rules() {
    let clean = poll(
        "Plain question",
        vec!["yes", "no"],
        vec![response("ada", "m1", "yes")],
    );
    let handed = vec![
        (pt("same"), vec![clean]),
        (pt("same"), vec![forged_question_poll()]),
    ];
    assert_eq!(
        census_lines(&handed),
        Err("field question holds a newline".to_string())
    );
}

#[test]
fn the_ledger_refuses_a_forged_question_asked_twice() {
    let question = "Shared\nminorities, 9 questions, 9 askings, 9 pluralities, 9 dissenters";
    let first = poll(
        question,
        vec!["yes", "no"],
        vec![response("ada", "m1", "yes"), response("bob", "m1", "no")],
    );
    let second = poll(
        question,
        vec!["yes", "no"],
        vec![response("ada", "m1", "yes"), response("cy", "m1", "no")],
    );
    let handed = vec![(pt("first"), vec![first]), (pt("second"), vec![second])];
    assert_eq!(
        ledger_lines(&handed),
        Err("field question holds a newline".to_string())
    );
}

#[test]
fn a_decided_jurys_dissenter_carrying_the_break_draws_the_refusal() {
    let handed = sets(vec![poll(
        "Plain question",
        vec!["yes", "no"],
        vec![
            response("eve\nplurality, fake, yes", "m1", "no"),
            response("adam", "m1", "yes"),
            response("amy", "m1", "yes"),
        ],
    )]);
    let defect = "response 0 field juror holds a newline".to_string();
    assert_eq!(minority_lines(&handed), Err(defect.clone()));
    assert_eq!(dissent_lines(&handed), Err(defect));
}

#[test]
fn the_agreement_reading_draws_the_same_refusals() {
    assert_eq!(
        agreement_lines(&[forged_question_poll()]),
        Err("field question holds a newline".to_string())
    );
    let juror_poll = poll(
        "Plain question",
        vec!["yes", "no"],
        vec![
            response("adam", "m1", "yes"),
            response("eve\nplurality, fake, yes", "m1", "no"),
            response("carol", "m2", "yes"),
        ],
    );
    assert_eq!(
        agreement_lines(&[juror_poll]),
        Err("response 1 field juror holds a newline".to_string())
    );
}

#[test]
fn a_clean_handed_poll_renders_unchanged() {
    let handed = sets(vec![poll(
        "Alpha",
        vec!["yes", "no"],
        vec![
            response("eve", "m1", "no"),
            response("adam", "m1", "yes"),
            response("carol", "m2", "yes"),
        ],
    )]);
    assert_eq!(
        minority_lines(&handed).expect("a reading"),
        vec![
            "question, Alpha",
            "plurality, handset, yes",
            "margin, handset, 1",
            "dissenter, eve",
            "minorities, 1 questions, 1 askings, 1 pluralities, 1 dissenters",
        ]
    );
}
