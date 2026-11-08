//! Pins for FIX-GJ-14, the library fencing closure, and
//! for FIX-GJ-15, its residual closure over the tally,
//! the stadium view, the cost reading, and the wave
//! comparison reading, and for FIX-GJ-16, the answer
//! fencing closure over the offered answers and the
//! answers of the responses.
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
use grandjury::bench::bench_lines;
use grandjury::census::census_lines;
use grandjury::compare::compare_lines;
use grandjury::dissent::dissent_lines;
use grandjury::ledger::ledger_lines;
use grandjury::minority::minority_lines;
use grandjury::polls::{
    cost_lines, render_stadium, render_tally_lines, Poll, PriceTable, Response,
};
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

#[test]
fn the_forged_question_draws_the_loader_defect_at_the_polls_readings() {
    let handed = forged_question_poll();
    let defect = "field question holds a newline".to_string();
    assert_eq!(render_tally_lines(&handed), Err(defect.clone()));
    assert_eq!(render_stadium(&handed), Err(defect.clone()));
    assert_eq!(
        cost_lines(&handed, &PriceTable::new(Vec::new())),
        Err(defect)
    );
}

#[test]
fn the_forged_juror_draws_the_response_defect_at_the_stadium_view() {
    let handed = poll(
        "Plain question",
        vec!["yes", "no"],
        vec![
            response("adam", "m1", "yes"),
            response("eve\nplurality, fake, yes", "m1", "no"),
            response("carol", "m2", "yes"),
        ],
    );
    assert_eq!(
        render_stadium(&handed),
        Err("response 1 field juror holds a newline".to_string())
    );
}

#[test]
fn a_break_bearing_model_in_the_second_set_draws_the_refusal_at_the_comparison() {
    let clean = poll(
        "Plain question",
        vec!["yes", "no"],
        vec![
            response("adam", "m1", "yes"),
            response("carol", "m2", "yes"),
        ],
    );
    let forged = poll(
        "Plain question",
        vec!["yes", "no"],
        vec![
            response("adam", "m1\nmovement, forged, held", "yes"),
            response("carol", "m2", "yes"),
        ],
    );
    let defect = "response 0 field model holds a newline".to_string();
    assert_eq!(
        compare_lines(std::slice::from_ref(&clean), std::slice::from_ref(&forged)),
        Err(defect.clone())
    );
    assert_eq!(compare_lines(&[forged], &[clean]), Err(defect));
}

#[test]
fn the_guard_stands_before_the_pairing_rules_at_the_comparison() {
    let forged = poll(
        "Plain question",
        vec!["yes", "no"],
        vec![response("adam", "m1\nmovement, forged, held", "yes")],
    );
    let clean = poll(
        "Plain question",
        vec!["yes", "no"],
        vec![response("adam", "m1", "yes")],
    );
    // The counts of the two sets differ, so the pairing
    // rules would refuse on their own terms, and the guard
    // standing before them names the model break instead.
    assert_eq!(
        compare_lines(&[forged], &[clean.clone(), clean]),
        Err("response 0 field model holds a newline".to_string())
    );
}

#[test]
fn a_break_bearing_model_draws_the_response_defect_at_the_agreement_reading() {
    let juror_poll = poll(
        "Plain question",
        vec!["yes", "no"],
        vec![
            response("adam", "m1", "yes"),
            response("eve", "m1\npluralities, agree", "no"),
            response("carol", "m2", "yes"),
        ],
    );
    assert_eq!(
        agreement_lines(&[juror_poll]),
        Err("response 1 field model holds a newline".to_string())
    );
}

#[test]
fn a_clean_handed_poll_renders_unchanged_at_the_polls_readings() {
    let handed = poll(
        "Alpha",
        vec!["yes", "no"],
        vec![
            response("eve", "m1", "no"),
            response("adam", "m1", "yes"),
            response("carol", "m2", "yes"),
        ],
    );
    assert_eq!(
        render_tally_lines(&handed).expect("a tally"),
        vec![
            "question, Alpha",
            "responses, 3",
            "valid, 3",
            "spoiled, 0",
            "yes, 2, 66.7%",
            "no, 1, 33.3%",
        ]
    );
    let stadium = render_stadium(&handed).expect("a stadium view");
    assert_eq!(stadium[0], "question, Alpha");
    assert_eq!(stadium[1], "responses, 3");
    assert_eq!(stadium[2], format!("yes |{}| 2, 66.7%", "#".repeat(40)));
    assert_eq!(stadium[3], format!("no |{}| 1, 33.3%", "#".repeat(20)));
    assert_eq!(
        stadium[4..],
        [
            "wall of reasons",
            "yes",
            "  adam, a reason",
            "  carol, a reason",
            "no",
            "  eve, a reason",
        ]
    );
    assert_eq!(
        cost_lines(&handed, &PriceTable::new(Vec::new())).expect("no refusal"),
        Some(vec![
            "question, Alpha".to_string(),
            "responses, 3".to_string(),
            "priced, 0 responses, 0.000000 dollars".to_string(),
            "unpriced, 3 responses".to_string(),
            "unpriced model, m1, 2 responses".to_string(),
            "unpriced model, m2, 1 responses".to_string(),
        ])
    );
}

/// The forged offered answer the Round 11 red team named
/// as finding F-1, the forged minority closing line
/// standing after a break inside the first offered
/// answer.
const FORGED_ANSWER: &str = "yes\nminorities, 9 questions, 9 askings, 9 pluralities, 9 dissenters";

fn forged_answer_poll() -> Poll {
    poll(
        "Plain question",
        vec![FORGED_ANSWER, "no"],
        vec![
            response("eve", "m1", "no"),
            response("adam", "m1", FORGED_ANSWER),
            response("carol", "m2", FORGED_ANSWER),
        ],
    )
}

fn clean_poll() -> Poll {
    poll(
        "Plain question",
        vec!["yes", "no"],
        vec![
            response("eve", "m1", "no"),
            response("adam", "m1", "yes"),
            response("carol", "m2", "yes"),
        ],
    )
}

#[test]
fn the_forged_offered_answer_draws_the_answer_defect_at_the_set_readings() {
    let handed = sets(vec![forged_answer_poll()]);
    let defect = "answer 0 holds a newline".to_string();
    assert_eq!(census_lines(&handed), Err(defect.clone()));
    assert_eq!(dissent_lines(&handed), Err(defect.clone()));
    assert_eq!(ledger_lines(&handed), Err(defect.clone()));
    assert_eq!(minority_lines(&handed), Err(defect.clone()));
    assert_eq!(swing_lines(&handed), Err(defect));
}

#[test]
fn the_forged_offered_answer_draws_the_answer_defect_at_the_bench_reading() {
    let handed = sets(vec![forged_answer_poll()]);
    let defect = Err("answer 0 holds a newline".to_string());
    assert_eq!(bench_lines(&handed), defect);
}

#[test]
fn the_forged_offered_answer_draws_the_answer_defect_at_the_polls_readings() {
    let handed_poll = forged_answer_poll();
    let defect = "answer 0 holds a newline".to_string();
    assert_eq!(render_tally_lines(&handed_poll), Err(defect.clone()));
    assert_eq!(render_stadium(&handed_poll), Err(defect.clone()));
    assert_eq!(
        cost_lines(&handed_poll, &PriceTable::new(Vec::new())),
        Err(defect)
    );
}

#[test]
fn the_forged_offered_answer_draws_the_answer_defect_at_the_agreement_and_the_comparison() {
    let clean = clean_poll();
    let forged = forged_answer_poll();
    let defect = "answer 0 holds a newline".to_string();
    assert_eq!(
        agreement_lines(std::slice::from_ref(&forged)),
        Err(defect.clone())
    );
    assert_eq!(
        compare_lines(std::slice::from_ref(&forged), std::slice::from_ref(&clean)),
        Err(defect.clone())
    );
    assert_eq!(
        compare_lines(std::slice::from_ref(&clean), std::slice::from_ref(&forged)),
        Err(defect)
    );
}

#[test]
fn an_unvoted_forged_offered_answer_draws_the_answer_defect() {
    let handed_poll = poll(
        "Plain question",
        vec![FORGED_ANSWER, "no"],
        vec![
            response("eve", "m1", "no"),
            response("adam", "m1", "no"),
            response("carol", "m2", "no"),
        ],
    );
    let handed = sets(vec![handed_poll.clone()]);
    let defect = "answer 0 holds a newline".to_string();
    assert_eq!(render_tally_lines(&handed_poll), Err(defect.clone()));
    assert_eq!(census_lines(&handed), Err(defect));
}

#[test]
fn a_break_in_the_second_offered_answer_draws_its_own_index() {
    let handed = sets(vec![poll(
        "Plain question",
        vec![
            "yes",
            "no\nminorities, 9 questions, 9 askings, 9 pluralities, 9 dissenters",
        ],
        vec![response("adam", "m1", "yes")],
    )]);
    let defect = "answer 1 holds a newline".to_string();
    assert_eq!(census_lines(&handed), Err(defect.clone()));
    assert_eq!(minority_lines(&handed), Err(defect));
}

#[test]
fn a_carriage_return_in_an_offered_answer_draws_the_same_defect() {
    let handed = sets(vec![poll(
        "Plain question",
        vec![
            "yes\rminorities, 9 questions, 9 askings, 9 pluralities, 9 dissenters",
            "no",
        ],
        vec![response("eve", "m1", "no")],
    )]);
    let defect = "answer 0 holds a newline".to_string();
    assert_eq!(census_lines(&handed), Err(defect.clone()));
    assert_eq!(minority_lines(&handed), Err(defect));
}

#[test]
fn the_answers_stand_before_the_responses_in_the_guard_order() {
    let handed = sets(vec![poll(
        "Plain question",
        vec![FORGED_ANSWER, "no"],
        vec![
            response("eve", "m1", "no"),
            response("adam\nplurality, fake, yes", "m1", "no"),
        ],
    )]);
    let defect = Err("answer 0 holds a newline".to_string());
    assert_eq!(census_lines(&handed), defect);
}

#[test]
fn a_spoiled_response_answer_carrying_a_break_draws_the_response_answer_defect() {
    let handed_poll = poll(
        "Plain question",
        vec!["yes", "no"],
        vec![
            response("adam", "m1", "yes"),
            response("carol", "m2", "yes"),
            response(
                "eve",
                "m1",
                "unoffered\nminorities, 9 questions, 9 askings, 9 pluralities, 9 dissenters",
            ),
        ],
    );
    let handed = sets(vec![handed_poll.clone()]);
    let defect = "response 2 field answer holds a newline".to_string();
    assert_eq!(render_stadium(&handed_poll), Err(defect.clone()));
    assert_eq!(census_lines(&handed), Err(defect));
}

#[test]
fn the_responses_stand_in_response_order_across_their_fields() {
    let handed = sets(vec![poll(
        "Plain question",
        vec!["yes", "no"],
        vec![
            response(
                "eve",
                "m1",
                "unoffered\nminorities, 9 questions, 9 askings, 9 pluralities, 9 dissenters",
            ),
            response("adam\nplurality, fake, yes", "m1", "no"),
        ],
    )]);
    let defect = Err("response 0 field answer holds a newline".to_string());
    assert_eq!(census_lines(&handed), defect);
}

#[test]
fn the_model_stands_before_the_answer_in_the_guard_order() {
    let handed = sets(vec![poll(
        "Plain question",
        vec!["yes", "no"],
        vec![response(
            "eve",
            "m1\npluralities, agree",
            "unoffered\nminorities, 9 questions, 9 askings, 9 pluralities, 9 dissenters",
        )],
    )]);
    let defect = Err("response 0 field model holds a newline".to_string());
    assert_eq!(census_lines(&handed), defect);
}
