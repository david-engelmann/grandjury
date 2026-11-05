//! Pins for GJ-13 and GJ-14, the juror ledger reading and
//! the ledger command.
//!
//! The function pins build their polls by hand, so each one
//! fails under the negation of the rule it names, held and
//! moved and not counted in each of their stated causes,
//! the agreement base and its matched count and fraction,
//! the label refusals the ledger shares with the census,
//! and the fencing of the fields the reading prints. The
//! recorded pins hold the ledger of the three recorded sets
//! to the committed golden through the function and through
//! the command, hold its record line against the census
//! golden over the same sets, hold its totals against an
//! independent count over the recorded polls grouped by
//! juror, hold its twice asked movement lines against the
//! recorded answers themselves, and hold its agreement
//! lines against an independent computation. Every pin
//! runs offline against the recorded polls, and no pin
//! makes a live call.

mod shared;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use grandjury::ledger::ledger_lines;
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

fn load_poll(path: &Path) -> Poll {
    let (poll, defects) = load_poll_file(path);
    assert!(defects.is_empty(), "{} {:?}", path.display(), defects);
    poll.expect("a poll that loads")
}

fn load_set(dir: &Path, count: usize) -> Vec<Poll> {
    (1..=count)
        .map(|index| load_poll(&dir.join(format!("poll-{:02}.json", index))))
        .collect()
}

fn recorded_sets() -> Vec<(grandjury::pytext::PText, Vec<Poll>)> {
    vec![
        (pt("pilot"), load_set(&examples().join("pilot"), 12)),
        (pt("full"), load_set(&examples().join("full"), 40)),
        (pt("wave"), load_set(&examples().join("wave"), 12)),
    ]
}

fn door_first() -> Poll {
    poll(
        "Which door?",
        vec!["red", "blue", "green"],
        vec![
            response("jev-1", "jev-a", "red", false),
            response("jev-2", "jev-a", "red", false),
            response("jev-3", "jev-a", "blue", false),
            response("openai-1", "chat-b", "blue", false),
            response("openai-2", "chat-b", "blue", false),
            response("openai-3", "chat-b", "red", false),
        ],
    )
}

fn door_second() -> Poll {
    poll(
        "Which door?",
        vec!["red", "blue", "green"],
        vec![
            response("jev-1", "jev-a", "red", false),
            response("jev-2", "jev-a", "blue", false),
            response("jev-3", "jev-a", "red", false),
            response("openai-1", "chat-b", "red", false),
            response("openai-2", "chat-b", "red", false),
            response("openai-3", "chat-b", "blue", false),
        ],
    )
}

#[test]
fn the_ledger_of_two_askings_stands_line_for_line() {
    let sets = vec![
        (pt("first"), vec![door_first()]),
        (pt("second"), vec![door_second()]),
    ];
    assert_eq!(
        ledger_lines(&sets).expect("a reading"),
        vec![
            "respondent block, jev-1",
            "set, first, recorded 1, valid 1, abstained 0, spoiled 0",
            "set, second, recorded 1, valid 1, abstained 0, spoiled 0",
            "respondent, jev-1, recorded 2, valid 2, abstained 0, spoiled 0",
            "question, Which door?",
            "  first, red",
            "  second, red",
            "movement, jev-1, held",
            "agreement, 2 of 2, 1",
            "respondent block, jev-2",
            "set, first, recorded 1, valid 1, abstained 0, spoiled 0",
            "set, second, recorded 1, valid 1, abstained 0, spoiled 0",
            "respondent, jev-2, recorded 2, valid 2, abstained 0, spoiled 0",
            "question, Which door?",
            "  first, red",
            "  second, blue",
            "movement, jev-2, moved",
            "agreement, 1 of 2, 1/2",
            "respondent block, jev-3",
            "set, first, recorded 1, valid 1, abstained 0, spoiled 0",
            "set, second, recorded 1, valid 1, abstained 0, spoiled 0",
            "respondent, jev-3, recorded 2, valid 2, abstained 0, spoiled 0",
            "question, Which door?",
            "  first, blue",
            "  second, red",
            "movement, jev-3, moved",
            "agreement, 1 of 2, 1/2",
            "respondent block, openai-1",
            "set, first, recorded 1, valid 1, abstained 0, spoiled 0",
            "set, second, recorded 1, valid 1, abstained 0, spoiled 0",
            "respondent, openai-1, recorded 2, valid 2, abstained 0, spoiled 0",
            "question, Which door?",
            "  first, blue",
            "  second, red",
            "movement, openai-1, moved",
            "agreement, 2 of 2, 1",
            "respondent block, openai-2",
            "set, first, recorded 1, valid 1, abstained 0, spoiled 0",
            "set, second, recorded 1, valid 1, abstained 0, spoiled 0",
            "respondent, openai-2, recorded 2, valid 2, abstained 0, spoiled 0",
            "question, Which door?",
            "  first, blue",
            "  second, red",
            "movement, openai-2, moved",
            "agreement, 2 of 2, 1",
            "respondent block, openai-3",
            "set, first, recorded 1, valid 1, abstained 0, spoiled 0",
            "set, second, recorded 1, valid 1, abstained 0, spoiled 0",
            "respondent, openai-3, recorded 2, valid 2, abstained 0, spoiled 0",
            "question, Which door?",
            "  first, red",
            "  second, blue",
            "movement, openai-3, moved",
            "agreement, 0 of 2, 0",
            "record, polls 2, recorded 12, valid 12, abstained 0, spoiled 0",
            "movements, 1 held, 5 moved, 0 not counted",
        ]
    );
}

#[test]
fn a_question_asked_once_stands_in_no_question_part() {
    let sets = vec![(pt("only"), vec![door_first()])];
    let lines = ledger_lines(&sets).expect("a reading");
    assert!(!lines.iter().any(|line| line.starts_with("question, ")));
    assert!(!lines.iter().any(|line| line.starts_with("movement, ")));
    assert_eq!(
        lines.last().map(String::as_str),
        Some("movements, 0 held, 0 moved, 0 not counted")
    );
    assert!(lines
        .contains(&"respondent, jev-1, recorded 1, valid 1, abstained 0, spoiled 0".to_string()));
}

#[test]
fn an_abstention_at_one_asking_counts_no_movement_and_no_base() {
    let first = poll(
        "Twice?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", false),
            response("jev-2", "jev-a", "a", false),
        ],
    );
    let second = poll(
        "Twice?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", true),
            response("jev-2", "jev-a", "b", false),
        ],
    );
    let sets = vec![(pt("one"), vec![first]), (pt("two"), vec![second])];
    let lines = ledger_lines(&sets).expect("a reading");
    let start = lines
        .iter()
        .position(|line| line == "respondent block, jev-1")
        .expect("the abstaining block");
    assert_eq!(
        lines[start..start + 8].join("\n"),
        [
            "respondent block, jev-1",
            "set, one, recorded 1, valid 1, abstained 0, spoiled 0",
            "set, two, recorded 1, valid 0, abstained 1, spoiled 0",
            "respondent, jev-1, recorded 2, valid 1, abstained 1, spoiled 0",
            "question, Twice?",
            "  one, a",
            "  two, none",
            "movement, jev-1, not counted",
        ]
        .join("\n")
    );
    // The one asking jev-1 answered validly held a plurality
    // of a for its jury and its answer matched it, so the
    // base stands at one and the abstained asking adds
    // nothing to it.
    assert_eq!(lines[start + 8], "agreement, 1 of 1, 1");
}

#[test]
fn a_respondent_abstained_at_every_asking_has_a_zero_base() {
    let first = poll(
        "Silent?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", true),
            response("jev-2", "jev-a", "a", false),
            response("jev-3", "jev-a", "a", false),
        ],
    );
    let second = poll(
        "Silent?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "b", true),
            response("jev-2", "jev-a", "b", false),
            response("jev-3", "jev-a", "b", false),
        ],
    );
    let sets = vec![(pt("one"), vec![first]), (pt("two"), vec![second])];
    let lines = ledger_lines(&sets).expect("a reading");
    let start = lines
        .iter()
        .position(|line| line == "respondent block, jev-1")
        .expect("the silent block");
    assert_eq!(
        lines[start..start + 9].join("\n"),
        [
            "respondent block, jev-1",
            "set, one, recorded 1, valid 0, abstained 1, spoiled 0",
            "set, two, recorded 1, valid 0, abstained 1, spoiled 0",
            "respondent, jev-1, recorded 2, valid 0, abstained 2, spoiled 0",
            "question, Silent?",
            "  one, none",
            "  two, none",
            "movement, jev-1, not counted",
            "agreement, 0 of 0, 0",
        ]
        .join("\n")
    );
}

#[test]
fn a_spoiled_answer_counts_apart_and_answers_none() {
    let first = poll(
        "Spoiled?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "off the list", false),
            response("jev-2", "jev-a", "a", false),
        ],
    );
    let second = poll(
        "Spoiled?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", false),
            response("jev-2", "jev-a", "a", false),
        ],
    );
    let sets = vec![(pt("one"), vec![first]), (pt("two"), vec![second])];
    let lines = ledger_lines(&sets).expect("a reading");
    let start = lines
        .iter()
        .position(|line| line == "respondent block, jev-1")
        .expect("the spoiling block");
    assert_eq!(
        lines[start..start + 9].join("\n"),
        [
            "respondent block, jev-1",
            "set, one, recorded 1, valid 0, abstained 0, spoiled 1",
            "set, two, recorded 1, valid 1, abstained 0, spoiled 0",
            "respondent, jev-1, recorded 2, valid 1, abstained 0, spoiled 1",
            "question, Spoiled?",
            "  one, none",
            "  two, a",
            "movement, jev-1, not counted",
            // At the first asking the jury plurality stands
            // at a on the one valid answer beside the spoiled
            // one, and the respondent gave no valid answer
            // there, so that asking stands outside its base.
            // At the second asking its answer matches the
            // plurality. The base is one and the match is one.
            "agreement, 1 of 1, 1",
        ]
        .join("\n")
    );
}

#[test]
fn a_tied_jury_holds_no_plurality_and_adds_no_base() {
    let first = poll(
        "Tied?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", false),
            response("jev-2", "jev-a", "b", false),
        ],
    );
    let second = poll(
        "Tied?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", false),
            response("jev-2", "jev-a", "a", false),
        ],
    );
    let sets = vec![(pt("one"), vec![first]), (pt("two"), vec![second])];
    let lines = ledger_lines(&sets).expect("a reading");
    assert!(lines.contains(&"agreement, 1 of 1, 1".to_string()));
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.starts_with("agreement, "))
            .count(),
        2
    );
    assert!(lines.contains(&"movement, jev-1, held".to_string()));
    assert!(lines.contains(&"movement, jev-2, moved".to_string()));
    assert_eq!(
        lines.last().map(String::as_str),
        Some("movements, 1 held, 1 moved, 0 not counted")
    );
}

#[test]
fn the_agreement_fraction_renders_in_lowest_terms() {
    fn agree_poll(question: &str, plurality_x: bool) -> Poll {
        // r-1 answers x in every poll of this fixture. With
        // plurality_x the jury of the poll holds two x
        // answers against one y and its plurality stands x,
        // and without it the jury holds one x against two y
        // and its plurality stands y.
        let (second, third) = if plurality_x { ("x", "y") } else { ("y", "y") };
        poll(
            question,
            vec!["x", "y"],
            vec![
                response("r-1", "m", "x", false),
                response("r-2", "m", second, false),
                response("r-3", "m", third, false),
            ],
        )
    }
    // Two of the four askings of r-1 hold a plurality of x
    // and two hold a plurality of y, so two of its four base
    // askings match its answer and the fraction renders
    // halved at one half.
    let sets = vec![
        (
            pt("one"),
            vec![agree_poll("First?", true), agree_poll("Second?", false)],
        ),
        (
            pt("two"),
            vec![agree_poll("First?", true), agree_poll("Second?", false)],
        ),
    ];
    let lines = ledger_lines(&sets).expect("a reading");
    let start = lines
        .iter()
        .position(|line| line == "respondent block, r-1")
        .expect("the first block");
    let block = &lines[start..start + 13];
    assert_eq!(
        block.last().map(String::as_str),
        Some("agreement, 2 of 4, 1/2")
    );
}

#[test]
fn a_respondent_absent_from_a_set_holds_no_part_for_it() {
    let first = poll(
        "Pair?",
        vec!["a", "b"],
        vec![
            response("ra", "m", "a", false),
            response("rb", "m", "b", false),
        ],
    );
    let second = poll(
        "Pair?",
        vec!["a", "b"],
        vec![response("rb", "m", "a", false)],
    );
    let sets = vec![(pt("one"), vec![first]), (pt("two"), vec![second])];
    let lines = ledger_lines(&sets).expect("a reading");
    let ra_at = lines
        .iter()
        .position(|line| line == "respondent block, ra")
        .expect("the absent respondent block");
    let rb_at = lines
        .iter()
        .position(|line| line == "respondent block, rb")
        .expect("the standing respondent block");
    assert_eq!(
        lines[ra_at..rb_at].join("\n"),
        [
            "respondent block, ra",
            "set, one, recorded 1, valid 1, abstained 0, spoiled 0",
            "respondent, ra, recorded 1, valid 1, abstained 0, spoiled 0",
            "agreement, 0 of 0, 0",
        ]
        .join("\n")
    );
    assert!(lines[rb_at..].contains(&"movement, rb, moved".to_string()));
}

#[test]
fn the_respondent_order_follows_first_appearance_across_the_sets() {
    let first = poll(
        "Order?",
        vec!["a", "b"],
        vec![response("openai-1", "chat-b", "a", false)],
    );
    let second = poll(
        "Order?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "b", false),
            response("openai-1", "chat-b", "a", false),
        ],
    );
    let sets = vec![(pt("one"), vec![first]), (pt("two"), vec![second])];
    let lines = ledger_lines(&sets).expect("a reading");
    // The block openers stand in the respondent block form
    // of FIX-GJ-13, which separates them from the totals
    // lines by their fixed words, so the filter names that
    // form alone.
    let openers: Vec<&str> = lines
        .iter()
        .filter(|line| line.starts_with("respondent block, "))
        .map(|line| line.as_str())
        .collect();
    assert_eq!(
        openers,
        vec!["respondent block, openai-1", "respondent block, jev-1"]
    );
}

#[test]
fn a_question_twice_in_one_set_refuses_the_ledger() {
    let sets = vec![(pt("first"), vec![door_first(), door_first()])];
    assert_eq!(
        ledger_lines(&sets),
        Err("the set first holds the question Which door? twice".to_string())
    );
}

#[test]
fn a_duplicate_set_label_refuses_the_ledger() {
    let sets = vec![
        (pt("first"), vec![door_first()]),
        (pt("first"), vec![door_second()]),
    ];
    assert_eq!(
        ledger_lines(&sets),
        Err("the set label first stands twice".to_string())
    );
}

#[test]
fn an_empty_set_label_refuses_the_ledger() {
    let sets = vec![(pt(""), vec![door_first()])];
    assert_eq!(ledger_lines(&sets), Err("a set label is empty".to_string()));
}

#[test]
fn a_set_label_holding_a_newline_refuses_the_ledger() {
    let sets = vec![(pt("two\nlines"), vec![door_first()])];
    assert_eq!(
        ledger_lines(&sets),
        Err("a set label holds a newline".to_string())
    );
    let sets = vec![(pt("two\rlines"), vec![door_first()])];
    assert_eq!(
        ledger_lines(&sets),
        Err("a set label holds a newline".to_string())
    );
}

#[test]
fn the_label_refusal_stands_before_the_question_refusal() {
    let sets = vec![
        (pt("first"), vec![door_first()]),
        (pt("first"), vec![door_second(), door_second()]),
    ];
    assert_eq!(
        ledger_lines(&sets),
        Err("the set label first stands twice".to_string())
    );
}

#[test]
fn a_hostile_set_label_mints_no_structural_line() {
    // The pin of FIX-GJ-12 carried to the ledger. The set
    // label stands behind the word set in a set part and
    // behind two spaces in an answer line, so a label naming
    // a structural line of the reading prints that form
    // nowhere at the start of a line. The reading over the
    // two door polls holds six respondent blocks, one record
    // line, and one closing line.
    let labels = [
        "record, polls 999",
        "respondent, forged",
        "movements, 9 held",
        "respondent, jev-1, recorded 9, valid 9, abstained 9, spoiled 9",
        "agreement, 9 of 9, 1",
    ];
    for label in labels {
        let sets = vec![
            (pt(label), vec![door_first()]),
            (pt("second"), vec![door_second()]),
        ];
        let lines = ledger_lines(&sets).expect("a reading");
        assert_eq!(
            lines
                .iter()
                .filter(|line| line.starts_with("record, polls "))
                .count(),
            1,
            "{}",
            label
        );
        assert!(lines.contains(
            &"record, polls 2, recorded 12, valid 12, abstained 0, spoiled 0".to_string()
        ));
        assert_eq!(
            lines
                .iter()
                .filter(|line| line.starts_with("movements, "))
                .count(),
            1,
            "{}",
            label
        );
        let answer_lines: Vec<&String> =
            lines.iter().filter(|line| line.starts_with("  ")).collect();
        assert_eq!(answer_lines.len(), 12, "{}", label);
        for line in &answer_lines {
            assert!(
                line.starts_with(&format!("  {}, ", label)) || line.starts_with("  second, "),
                "{} in {}",
                line,
                label
            );
        }
        let counts: Vec<&String> = lines
            .iter()
            .filter(|line| line.starts_with("set, "))
            .collect();
        assert_eq!(counts.len(), 12, "{}", label);
        for line in counts {
            assert!(
                line.starts_with(&format!("set, {}, recorded", label))
                    || line.starts_with("set, second, recorded"),
                "{} in {}",
                line,
                label
            );
        }
    }
}

#[test]
fn a_hostile_respondent_name_mints_no_structural_line() {
    // A juror string is a value a loaded poll supplies, and
    // it stands behind the word respondent in its block
    // opener and its totals line and behind the word
    // movement in its movement lines, so a name carrying a
    // structural form mints no second line of that form.
    let forged = poll(
        "Forged?",
        vec!["a", "b"],
        vec![
            response(
                "record, polls 9, recorded 9, valid 9, abstained 9, spoiled 9",
                "m",
                "a",
                false,
            ),
            response("plain", "m", "a", false),
        ],
    );
    let again = poll(
        "Forged?",
        vec!["a", "b"],
        vec![
            response(
                "record, polls 9, recorded 9, valid 9, abstained 9, spoiled 9",
                "m",
                "a",
                false,
            ),
            response("plain", "m", "b", false),
        ],
    );
    let sets = vec![(pt("one"), vec![forged]), (pt("two"), vec![again])];
    let lines = ledger_lines(&sets).expect("a reading");
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.starts_with("record, polls "))
            .count(),
        1
    );
    assert!(
        lines.contains(&"record, polls 2, recorded 4, valid 4, abstained 0, spoiled 0".to_string())
    );
}

#[test]
fn a_respondent_named_in_counts_form_mints_no_totals_line() {
    // FIX-GJ-13. A respondent whose juror string carries a
    // full counts form once made the opening line of its
    // block satisfy the totals form exactly, ahead of the
    // true totals line of the block. The block opens in the
    // respondent block form, the fabricated figures stand
    // on no line of the reading, and the totals form of the
    // reading is answered only by the true totals lines,
    // carrying the counts the record holds.
    let hostile = "x, recorded 99, valid 99, abstained 0, spoiled 0";
    let the_poll = poll(
        "Pick?",
        vec!["a", "b"],
        vec![
            response(hostile, "m-1", "a", false),
            response("y", "m-1", "a", false),
        ],
    );
    let sets = vec![(pt("one"), vec![the_poll])];
    let lines = ledger_lines(&sets).expect("a reading");
    let start = lines
        .iter()
        .position(|line| line == &format!("respondent block, {hostile}"))
        .expect("the hostile block");
    assert_eq!(
        lines[start..start + 4].join("\n"),
        [
            "respondent block, x, recorded 99, valid 99, abstained 0, spoiled 0",
            "set, one, recorded 1, valid 1, abstained 0, spoiled 0",
            "respondent, x, recorded 99, valid 99, abstained 0, spoiled 0, recorded 1, valid 1, abstained 0, spoiled 0",
            "agreement, 1 of 1, 1",
        ]
        .join("\n")
    );
    assert!(
        !lines
            .contains(&"respondent, x, recorded 99, valid 99, abstained 0, spoiled 0".to_string()),
        "the fabricated totals line stands nowhere in {lines:?}"
    );
    let totals_lines: Vec<&str> = lines
        .iter()
        .filter(|line| line.starts_with("respondent, ") && line.contains(", recorded "))
        .map(|line| line.as_str())
        .collect();
    assert_eq!(
        totals_lines,
        vec![
            "respondent, x, recorded 99, valid 99, abstained 0, spoiled 0, recorded 1, valid 1, abstained 0, spoiled 0",
            "respondent, y, recorded 1, valid 1, abstained 0, spoiled 0",
        ]
    );
}

#[test]
fn the_ledger_of_no_sets_stands_at_zero() {
    assert_eq!(
        ledger_lines(&[]).expect("a reading"),
        vec![
            "record, polls 0, recorded 0, valid 0, abstained 0, spoiled 0",
            "movements, 0 held, 0 moved, 0 not counted",
        ]
    );
}

fn write_poll(dir: &Path, index: usize, held: &Poll) {
    std::fs::create_dir_all(dir).expect("a poll directory");
    std::fs::write(
        dir.join(format!("poll-{:02}.json", index)),
        dumps_default(&held.to_json()),
    )
    .expect("a written poll");
}

#[test]
fn the_ledger_command_prints_the_recorded_golden() {
    let (out, err, code) = run_cli(&[
        "ledger",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "full",
        &path_arg(&examples().join("full")),
        "wave",
        &path_arg(&examples().join("wave")),
    ]);
    assert_eq!(code, 0, "{}", err);
    let golden = read_text(&examples().join("ledger-golden.txt"));
    assert_eq!(out, golden);
}

#[test]
fn the_recorded_ledger_stands_in_the_golden() {
    let lines = ledger_lines(&recorded_sets()).expect("a reading");
    let golden = read_text(&examples().join("ledger-golden.txt"));
    assert_eq!(lines.join("\n"), golden.trim_end_matches('\n'));
}

#[test]
fn the_ledger_record_line_is_the_census_record_line() {
    let ledger = read_text(&examples().join("ledger-golden.txt"));
    let census = read_text(&examples().join("census-golden.txt"));
    let ledger_record = ledger
        .lines()
        .find(|line| line.starts_with("record, polls "))
        .expect("the ledger record line");
    let census_record = census
        .lines()
        .find(|line| line.starts_with("record, polls "))
        .expect("the census record line");
    assert_eq!(ledger_record, census_record);
}

/// Parse the four counts out of a counts tail in the GJ-8
/// order, recorded, valid, abstained, spoiled.
fn parse_counts(text: &str) -> [i64; 4] {
    let mut found = [0i64; 4];
    for (index, word) in ["recorded", "valid", "abstained", "spoiled"]
        .iter()
        .enumerate()
    {
        let marker = format!("{} ", word);
        let at = text.find(&marker).expect("a counts word") + marker.len();
        let rest = &text[at..];
        let digits: String = rest
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == '-')
            .collect();
        found[index] = digits.parse().expect("a count");
    }
    found
}

#[test]
fn the_ledger_totals_sum_to_the_record_line() {
    let golden = read_text(&examples().join("ledger-golden.txt"));
    let mut sums = [0i64; 4];
    let mut respondents = 0usize;
    let mut record: Option<[i64; 4]> = None;
    let mut polls_figure = 0i64;
    for line in golden.lines() {
        if let Some(rest) = line.strip_prefix("record, polls ") {
            let (figure, tail) = rest.split_once(',').expect("a record line");
            polls_figure = figure.parse().expect("a polls figure");
            record = Some(parse_counts(tail));
        } else if line.starts_with("respondent, ") && line.contains(", recorded ") {
            let counts = parse_counts(line);
            for index in 0..4 {
                sums[index] += counts[index];
            }
            respondents += 1;
        }
    }
    assert_eq!(respondents, 10);
    assert_eq!(polls_figure, 64);
    assert_eq!(Some(sums), record);
}

#[test]
fn the_ledger_totals_agree_with_an_independent_count() {
    // A direct classification of the recorded responses
    // grouped by juror, sharing no reading code with the
    // ledger, against the totals line of each respondent in
    // the golden, in first appearance order.
    let mut order: Vec<String> = Vec::new();
    let mut counts: BTreeMap<String, [i64; 4]> = BTreeMap::new();
    for (_, polls) in recorded_sets() {
        for held in &polls {
            for item in &held.responses {
                let juror = item.juror.display();
                if !order.contains(&juror) {
                    order.push(juror.clone());
                }
                let entry = counts.entry(juror).or_default();
                entry[0] += 1;
                if item.abstained {
                    entry[2] += 1;
                } else if held.answers.contains(&item.answer) {
                    entry[1] += 1;
                } else {
                    entry[3] += 1;
                }
            }
        }
    }
    let golden = read_text(&examples().join("ledger-golden.txt"));
    let totals: Vec<&str> = golden
        .lines()
        .filter(|line| line.starts_with("respondent, ") && line.contains(", recorded "))
        .collect();
    assert_eq!(totals.len(), order.len());
    for (line, juror) in totals.iter().zip(order.iter()) {
        let held = &counts[juror];
        let expected = format!(
            "respondent, {}, recorded {}, valid {}, abstained {}, spoiled {}",
            juror, held[0], held[1], held[2], held[3]
        );
        assert_eq!(*line, expected);
    }
}

/// The plurality of one model in one recorded poll by a
/// direct count of its valid answers, standing only when
/// one answer holds the largest count alone.
fn direct_plurality(held: &Poll, model: &str) -> Option<String> {
    let mut tallies: BTreeMap<String, i64> = BTreeMap::new();
    for item in &held.responses {
        if item.model.display() == model && !item.abstained && held.answers.contains(&item.answer) {
            *tallies.entry(item.answer.display()).or_default() += 1;
        }
    }
    if tallies.is_empty() {
        return None;
    }
    let largest = tallies.values().max().copied().unwrap_or(0);
    let holders: Vec<&String> = tallies
        .iter()
        .filter(|(_, count)| **count == largest)
        .map(|(answer, _)| answer)
        .collect();
    if holders.len() == 1 {
        Some(holders[0].clone())
    } else {
        None
    }
}

fn direct_fraction(matched: i64, base: i64) -> String {
    if base == 0 || matched == 0 {
        return "0".to_string();
    }
    let mut a = matched;
    let mut b = base;
    while b != 0 {
        let rest = a % b;
        a = b;
        b = rest;
    }
    let (numerator, denominator) = (matched / a, base / a);
    if denominator == 1 {
        numerator.to_string()
    } else {
        format!("{}/{}", numerator, denominator)
    }
}

#[test]
fn the_ledger_agreement_lines_agree_with_an_independent_computation() {
    let sets = recorded_sets();
    let mut order: Vec<String> = Vec::new();
    for (_, polls) in &sets {
        for held in polls {
            for item in &held.responses {
                let juror = item.juror.display();
                if !order.contains(&juror) {
                    order.push(juror);
                }
            }
        }
    }
    let mut expected: Vec<String> = Vec::new();
    for juror in &order {
        let mut matched = 0i64;
        let mut base = 0i64;
        for (_, polls) in &sets {
            for held in polls {
                for item in &held.responses {
                    if item.juror.display() != *juror
                        || item.abstained
                        || !held.answers.contains(&item.answer)
                    {
                        continue;
                    }
                    if let Some(plurality) = direct_plurality(held, &item.model.display()) {
                        base += 1;
                        if plurality == item.answer.display() {
                            matched += 1;
                        }
                    }
                }
            }
        }
        expected.push(format!(
            "agreement, {} of {}, {}",
            matched,
            base,
            direct_fraction(matched, base)
        ));
    }
    let golden = read_text(&examples().join("ledger-golden.txt"));
    let found: Vec<&str> = golden
        .lines()
        .filter(|line| line.starts_with("agreement, "))
        .collect();
    assert_eq!(
        found,
        expected.iter().map(String::as_str).collect::<Vec<&str>>()
    );
}

#[test]
fn the_ledger_movements_agree_with_the_recorded_answers() {
    // The twelve pilot questions stand in the pilot and the
    // wave. For every respondent standing in both polls of a
    // question, the golden holds a question part whose
    // answer lines and movement word a direct reading of the
    // two recorded polls reproduces, in the order the parts
    // stand, respondent block by respondent block and
    // question by question in the order the pilot asks them.
    let pilot = load_set(&examples().join("pilot"), 12);
    let wave = load_set(&examples().join("wave"), 12);
    let mut order: Vec<String> = Vec::new();
    for (_, polls) in recorded_sets() {
        for held in &polls {
            for item in &held.responses {
                let juror = item.juror.display();
                if !order.contains(&juror) {
                    order.push(juror);
                }
            }
        }
    }
    let answer_of = |held: &Poll, juror: &str| -> Option<Option<String>> {
        held.responses
            .iter()
            .find(|item| item.juror.display() == juror)
            .map(|item| {
                if item.abstained || !held.answers.contains(&item.answer) {
                    None
                } else {
                    Some(item.answer.display())
                }
            })
    };
    let mut expected: Vec<String> = Vec::new();
    for juror in &order {
        for (first, second) in pilot.iter().zip(wave.iter()) {
            let (a, b) = match (answer_of(first, juror), answer_of(second, juror)) {
                (Some(a), Some(b)) => (a, b),
                _ => continue,
            };
            expected.push(format!("question, {}", first.question.display()));
            expected.push(format!(
                "  pilot, {}",
                a.clone().unwrap_or_else(|| "none".to_string())
            ));
            expected.push(format!(
                "  wave, {}",
                b.clone().unwrap_or_else(|| "none".to_string())
            ));
            let valid: Vec<&String> = [a.as_ref(), b.as_ref()].into_iter().flatten().collect();
            let movement = if valid.len() >= 2 && valid.iter().all(|held| *held == valid[0]) {
                "held"
            } else if valid.len() >= 2 {
                "moved"
            } else {
                "not counted"
            };
            expected.push(format!("movement, {}, {}", juror, movement));
        }
    }
    let golden = read_text(&examples().join("ledger-golden.txt"));
    let mut found: Vec<String> = Vec::new();
    let mut lines = golden.lines().peekable();
    while let Some(line) = lines.next() {
        if line.starts_with("question, ") {
            found.push(line.to_string());
            for _ in 0..2 {
                found.push(lines.next().expect("an answer line").to_string());
            }
            found.push(lines.next().expect("a movement line").to_string());
        }
    }
    assert_eq!(found, expected);
}

#[test]
fn the_ledger_golden_carries_no_verdict_vocabulary() {
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
    let golden = read_text(&examples().join("ledger-golden.txt"));
    let words: Vec<String> = golden
        .to_lowercase()
        .split(|c: char| !c.is_ascii_lowercase())
        .filter(|word| !word.is_empty())
        .map(|word| word.to_string())
        .collect();
    for word in VERDICT_WORDS {
        assert!(!words.contains(&word.to_string()), "{}", word);
    }
    let (out, _, code) = run_cli(&[
        "ledger",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "full",
        &path_arg(&examples().join("full")),
        "wave",
        &path_arg(&examples().join("wave")),
    ]);
    assert_eq!(code, 0);
    let out_words: Vec<String> = out
        .to_lowercase()
        .split(|c: char| !c.is_ascii_lowercase())
        .filter(|word| !word.is_empty())
        .map(|word| word.to_string())
        .collect();
    for word in VERDICT_WORDS {
        assert!(!out_words.contains(&word.to_string()), "{}", word);
    }
}

#[test]
fn the_ledger_command_refuses_a_defective_poll() {
    let scratch = TempDir::new("ledger-defective");
    let first_dir = scratch.path.join("first");
    let second_dir = scratch.path.join("second");
    write_poll(&first_dir, 1, &door_first());
    std::fs::create_dir_all(&second_dir).expect("a poll directory");
    std::fs::write(second_dir.join("poll-01.json"), "{}\n").expect("a written poll");
    let (out, _, code) = run_cli(&[
        "ledger",
        "first",
        &path_arg(&first_dir),
        "second",
        &path_arg(&second_dir),
    ]);
    assert_eq!(code, 1);
    assert!(out.starts_with("invalid poll\ndefect "), "{}", out);
    assert!(!out.contains("record, polls"), "{}", out);
}

#[test]
fn the_ledger_command_refuses_a_missing_first_directory() {
    let scratch = TempDir::new("ledger-missing");
    let missing: PathBuf = scratch.path.join("nowhere");
    let (out, _, code) = run_cli(&["ledger", "pilot", &path_arg(&missing)]);
    assert_eq!(code, 1);
    assert_eq!(out, "invalid poll\ndefect file cannot be read\n");
}

#[test]
fn the_ledger_command_refuses_a_duplicate_label() {
    let (out, _, code) = run_cli(&[
        "ledger",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "pilot",
        &path_arg(&examples().join("wave")),
    ]);
    assert_eq!(code, 1);
    assert_eq!(
        out,
        "invalid ledger\ndefect the set label pilot stands twice\n"
    );
}

#[test]
fn the_ledger_command_refuses_a_label_holding_a_newline() {
    let (out, _, code) = run_cli(&["ledger", "two\nlines", &path_arg(&examples().join("pilot"))]);
    assert_eq!(code, 1);
    assert_eq!(out, "invalid ledger\ndefect a set label holds a newline\n");
}

#[test]
fn the_ledger_command_refuses_an_unpaired_label() {
    let (_, err, code) = run_cli(&["ledger", "pilot"]);
    assert_eq!(code, 2);
    assert!(
        err.contains("the following arguments are required: directory"),
        "{}",
        err
    );
    let (_, err, code) = run_cli(&["ledger"]);
    assert_eq!(code, 2);
    assert!(
        err.contains("the following arguments are required: label, directory"),
        "{}",
        err
    );
}

#[test]
fn the_ledger_command_stops_a_set_at_a_gap() {
    let scratch = TempDir::new("ledger-gap");
    let dir = scratch.path.join("set");
    write_poll(&dir, 1, &door_first());
    write_poll(&dir, 3, &door_first());
    let (out, _, code) = run_cli(&["ledger", "only", &path_arg(&dir)]);
    assert_eq!(code, 0);
    assert!(out.contains("record, polls 1, recorded 6"), "{}", out);
}

#[test]
fn the_ledger_help_names_its_pairs() {
    let (out, _, code) = run_cli(&["ledger", "--help"]);
    assert_eq!(code, 0);
    assert!(
        out.contains("usage: grandjury ledger [-h] label directory"),
        "{}",
        out
    );
    let (top, _, top_code) = run_cli(&["--help"]);
    assert_eq!(top_code, 0);
    assert!(
        top.contains("print the juror ledger reading over recorded poll sets"),
        "{}",
        top
    );
}
