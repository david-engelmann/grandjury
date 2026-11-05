//! Pins for GJ-15 and GJ-16, the dissent reading and
//! the dissent command.
//!
//! The function pins build their polls by hand, so each one
//! fails under the negation of the rule it names, the
//! plurality under the tie rule of GJ-8 as FIX-GJ-8 repairs
//! it, the margin as an exact difference of counts, the
//! dissenters in the respondent order of the record and
//! never the abstained or the spoiled, the label refusals
//! the dissent shares with the census, and the fencing of
//! the fields the reading prints. The recorded pins hold
//! the dissent of the three recorded sets to the committed
//! golden through the function and through the command,
//! hold its plurality and margin lines and its closing
//! line against an independent computation over the
//! recorded polls, and hold its dissents against the
//! ledger golden of GJ-14, respondent by respondent, the
//! dissents of each respondent standing equal to the base
//! of its ledger agreement line minus its matched count.
//! Every pin runs offline against the recorded polls, and
//! no pin makes a live call.

mod shared;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use grandjury::dissent::dissent_lines;
use grandjury::polls::{load_poll_file, Poll, Response};
use grandjury::pytext::PText;
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

fn recorded_sets() -> Vec<(PText, Vec<Poll>)> {
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
fn the_dissent_of_two_askings_stands_line_for_line() {
    let sets = vec![
        (pt("first"), vec![door_first()]),
        (pt("second"), vec![door_second()]),
    ];
    assert_eq!(
        dissent_lines(&sets).expect("a reading"),
        vec![
            "question, Which door?",
            "jury, jev-a, recorded 3, valid 3, abstained 0, spoiled 0",
            "plurality, first, red",
            "margin, first, 1",
            "dissenter, jev-3",
            "question, Which door?",
            "jury, chat-b, recorded 3, valid 3, abstained 0, spoiled 0",
            "plurality, first, blue",
            "margin, first, 1",
            "dissenter, openai-3",
            "question, Which door?",
            "jury, jev-a, recorded 3, valid 3, abstained 0, spoiled 0",
            "plurality, second, red",
            "margin, second, 1",
            "dissenter, jev-2",
            "question, Which door?",
            "jury, chat-b, recorded 3, valid 3, abstained 0, spoiled 0",
            "plurality, second, red",
            "margin, second, 1",
            "dissenter, openai-3",
            "dissents, 4 questions, 4 pluralities, 4 counted",
        ]
    );
}

#[test]
fn the_dissent_of_no_sets_stands_at_zero() {
    assert_eq!(
        dissent_lines(&[]).expect("a reading"),
        vec!["dissents, 0 questions, 0 pluralities, 0 counted"]
    );
}

#[test]
fn a_poll_holding_no_responses_holds_no_part() {
    let silent = poll("Silent?", vec!["a", "b"], vec![]);
    let sets = vec![(pt("only"), vec![silent])];
    assert_eq!(
        dissent_lines(&sets).expect("a reading"),
        vec!["dissents, 0 questions, 0 pluralities, 0 counted"]
    );
}

#[test]
fn a_top_tie_names_no_dissenters_and_reads_not_counted() {
    let tied = poll(
        "Which window?",
        vec!["left", "right"],
        vec![
            response("ann", "solo", "left", false),
            response("bob", "solo", "right", false),
            response("cy", "solo", "left", false),
            response("dan", "solo", "right", false),
        ],
    );
    let sets = vec![(pt("only"), vec![tied])];
    let lines = dissent_lines(&sets).expect("a reading");
    assert_eq!(
        lines,
        vec![
            "question, Which window?",
            "jury, solo, recorded 4, valid 4, abstained 0, spoiled 0",
            "plurality, only, none",
            "margin, only, not counted",
            "dissents, 1 questions, 0 pluralities, 0 counted",
        ]
    );
    assert!(!lines.iter().any(|line| line.starts_with("dissenter, ")));
}

#[test]
fn a_jury_with_no_valid_responses_holds_no_plurality() {
    let silent = poll(
        "Which door?",
        vec!["red", "blue"],
        vec![
            response("jev-1", "jev-a", "red", true),
            response("jev-2", "jev-a", "blue", true),
            response("jev-3", "jev-a", "purple", false),
        ],
    );
    let sets = vec![(pt("only"), vec![silent])];
    assert_eq!(
        dissent_lines(&sets).expect("a reading"),
        vec![
            "question, Which door?",
            "jury, jev-a, recorded 3, valid 0, abstained 2, spoiled 1",
            "plurality, only, none",
            "margin, only, not counted",
            "dissents, 1 questions, 0 pluralities, 0 counted",
        ]
    );
}

#[test]
fn a_margin_of_one_stands_at_three_to_two() {
    let close = poll(
        "Which letter?",
        vec!["a", "b"],
        vec![
            response("r1", "m", "a", false),
            response("r2", "m", "a", false),
            response("r3", "m", "a", false),
            response("r4", "m", "b", false),
            response("r5", "m", "b", false),
        ],
    );
    let sets = vec![(pt("only"), vec![close])];
    assert_eq!(
        dissent_lines(&sets).expect("a reading"),
        vec![
            "question, Which letter?",
            "jury, m, recorded 5, valid 5, abstained 0, spoiled 0",
            "plurality, only, a",
            "margin, only, 1",
            "dissenter, r4",
            "dissenter, r5",
            "dissents, 1 questions, 1 pluralities, 2 counted",
        ]
    );
}

#[test]
fn a_unanimous_jury_holds_no_dissenter_and_its_full_margin() {
    let agreed = poll(
        "Which door?",
        vec!["red", "blue"],
        vec![
            response("r1", "m", "red", false),
            response("r2", "m", "red", false),
            response("r3", "m", "red", false),
        ],
    );
    let sets = vec![(pt("only"), vec![agreed])];
    assert_eq!(
        dissent_lines(&sets).expect("a reading"),
        vec![
            "question, Which door?",
            "jury, m, recorded 3, valid 3, abstained 0, spoiled 0",
            "plurality, only, red",
            "margin, only, 3",
            "dissents, 1 questions, 1 pluralities, 0 counted",
        ]
    );
}

#[test]
fn the_dissenters_stand_in_record_order_and_not_poll_order() {
    // The first poll establishes zed ahead of amy in the
    // record order. The second poll stands amy ahead of
    // zed in its own responses, and both dissent there, so
    // the dissenter lines discriminate the record order
    // from the poll order.
    let first = poll(
        "Which door?",
        vec!["red", "blue"],
        vec![
            response("zed", "m", "red", false),
            response("amy", "m", "red", false),
        ],
    );
    let second = poll(
        "Which door?",
        vec!["red", "blue"],
        vec![
            response("amy", "m", "blue", false),
            response("kim", "m", "red", false),
            response("jo", "m", "red", false),
            response("sam", "m", "red", false),
            response("zed", "m", "blue", false),
        ],
    );
    let sets = vec![(pt("first"), vec![first]), (pt("second"), vec![second])];
    assert_eq!(
        dissent_lines(&sets).expect("a reading"),
        vec![
            "question, Which door?",
            "jury, m, recorded 2, valid 2, abstained 0, spoiled 0",
            "plurality, first, red",
            "margin, first, 2",
            "question, Which door?",
            "jury, m, recorded 5, valid 5, abstained 0, spoiled 0",
            "plurality, second, red",
            "margin, second, 1",
            "dissenter, zed",
            "dissenter, amy",
            "dissents, 2 questions, 2 pluralities, 2 counted",
        ]
    );
}

#[test]
fn an_abstained_or_spoiled_answer_is_never_a_dissenter() {
    let mixed = poll(
        "Which door?",
        vec!["red", "blue"],
        vec![
            response("a", "m", "red", false),
            response("b", "m", "red", false),
            response("c", "m", "blue", false),
            response("d", "m", "blue", true),
            response("e", "m", "green", false),
        ],
    );
    let sets = vec![(pt("only"), vec![mixed])];
    let lines = dissent_lines(&sets).expect("a reading");
    assert_eq!(
        lines,
        vec![
            "question, Which door?",
            "jury, m, recorded 5, valid 3, abstained 1, spoiled 1",
            "plurality, only, red",
            "margin, only, 1",
            "dissenter, c",
            "dissents, 1 questions, 1 pluralities, 1 counted",
        ]
    );
}

#[test]
fn a_respondent_named_in_counts_form_mints_no_jury_line() {
    // The fencing rule of FIX-GJ-12 carried to the dissent.
    // A juror string stands behind the word dissenter in
    // the one line that carries it, so a name carrying the
    // counts form of the jury line prints that form
    // nowhere, and the one jury line of the part carries
    // the counts the record holds.
    let named = "x, recorded 99, valid 99, abstained 0, spoiled 0";
    let held = poll(
        "Which door?",
        vec!["red", "blue"],
        vec![
            response("p", "m", "red", false),
            response("q", "m", "red", false),
            response(named, "m", "blue", false),
        ],
    );
    let sets = vec![(pt("only"), vec![held])];
    let lines = dissent_lines(&sets).expect("a reading");
    assert!(lines.contains(&format!("dissenter, {}", named)));
    let jury_lines: Vec<&String> = lines
        .iter()
        .filter(|line| line.starts_with("jury, "))
        .collect();
    assert_eq!(
        jury_lines,
        vec!["jury, m, recorded 3, valid 3, abstained 0, spoiled 0"]
    );
}

#[test]
fn a_respondent_named_in_margin_form_mints_no_margin_line() {
    // The name carries the margin form of the reading
    // itself, and it stands behind the word dissenter, so
    // the one margin line of the part is the true one.
    let named = "margin, second, 7";
    let held = poll(
        "Which door?",
        vec!["red", "blue"],
        vec![
            response("p", "m", "red", false),
            response("q", "m", "red", false),
            response(named, "m", "blue", false),
        ],
    );
    let sets = vec![(pt("second"), vec![held])];
    let lines = dissent_lines(&sets).expect("a reading");
    assert!(lines.contains(&format!("dissenter, {}", named)));
    let margin_lines: Vec<&String> = lines
        .iter()
        .filter(|line| line.starts_with("margin, "))
        .collect();
    assert_eq!(margin_lines, vec!["margin, second, 1"]);
}

#[test]
fn a_hostile_set_label_mints_no_structural_line() {
    // The label names the closing line of the reading in
    // full, and it stands behind the word plurality in
    // the plurality line and behind the word margin in
    // the margin line, so the closing form stands once,
    // carrying the figures the record holds, and the
    // label text opens no line.
    let label = "dissents, 99 questions, 99 pluralities, 99 counted";
    let sets = vec![(pt(label), vec![door_first()])];
    let lines = dissent_lines(&sets).expect("a reading");
    let closing: Vec<&String> = lines
        .iter()
        .filter(|line| line.starts_with("dissents, "))
        .collect();
    assert_eq!(
        closing,
        vec!["dissents, 2 questions, 2 pluralities, 2 counted"]
    );
    assert!(!lines.iter().any(|line| line.starts_with(label)));
    assert!(lines.contains(&format!("plurality, {}, red", label)));
    assert!(lines.contains(&format!("margin, {}, 1", label)));
}

#[test]
fn a_question_twice_in_one_set_refuses_the_dissent() {
    let sets = vec![(pt("first"), vec![door_first(), door_first()])];
    assert_eq!(
        dissent_lines(&sets),
        Err("the set first holds the question Which door? twice".to_string())
    );
}

#[test]
fn a_duplicate_set_label_refuses_the_dissent() {
    let sets = vec![
        (pt("first"), vec![door_first()]),
        (pt("first"), vec![door_second()]),
    ];
    assert_eq!(
        dissent_lines(&sets),
        Err("the set label first stands twice".to_string())
    );
}

#[test]
fn an_empty_set_label_refuses_the_dissent() {
    let sets = vec![(pt(""), vec![door_first()])];
    assert_eq!(
        dissent_lines(&sets),
        Err("a set label is empty".to_string())
    );
}

#[test]
fn a_set_label_holding_a_newline_refuses_the_dissent() {
    let sets = vec![(pt("two\nlines"), vec![door_first()])];
    assert_eq!(
        dissent_lines(&sets),
        Err("a set label holds a newline".to_string())
    );
    let sets = vec![(pt("two\rlines"), vec![door_first()])];
    assert_eq!(
        dissent_lines(&sets),
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
        dissent_lines(&sets),
        Err("the set label first stands twice".to_string())
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
fn the_dissent_command_prints_the_recorded_golden() {
    let (out, err, code) = run_cli(&[
        "dissent",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "full",
        &path_arg(&examples().join("full")),
        "wave",
        &path_arg(&examples().join("wave")),
    ]);
    assert_eq!(code, 0, "{}", err);
    let golden = read_text(&examples().join("dissent-golden.txt"));
    assert_eq!(out, golden);
}

#[test]
fn the_recorded_dissent_stands_in_the_golden() {
    let lines = dissent_lines(&recorded_sets()).expect("a reading");
    let golden = read_text(&examples().join("dissent-golden.txt"));
    assert_eq!(lines.join("\n"), golden.trim_end_matches('\n'));
}

/// Compute the plurality and margin lines of the reading
/// in part order, the dissents of each respondent, and
/// the closing line, directly from the recorded polls,
/// without the reading under test.
fn direct_dissent(sets: &[(PText, Vec<Poll>)]) -> (Vec<String>, BTreeMap<String, i64>, String) {
    let mut respondents: Vec<String> = Vec::new();
    for (_, polls) in sets {
        for poll in polls {
            for held in &poll.responses {
                let name = held.juror.display();
                if !respondents.contains(&name) {
                    respondents.push(name);
                }
            }
        }
    }
    let mut paired: Vec<String> = Vec::new();
    let mut per: BTreeMap<String, i64> = BTreeMap::new();
    let mut questions = 0i64;
    let mut pluralities = 0i64;
    let mut counted = 0i64;
    for (label, polls) in sets {
        for poll in polls {
            let mut models: Vec<PText> = Vec::new();
            for held in &poll.responses {
                if !models.contains(&held.model) {
                    models.push(held.model.clone());
                }
            }
            for model in &models {
                questions += 1;
                let mine: Vec<&Response> = poll
                    .responses
                    .iter()
                    .filter(|held| &held.model == model)
                    .collect();
                let mut counts = vec![0i64; poll.answers.len()];
                for held in &mine {
                    if !held.abstained {
                        if let Some(slot) = poll.answers.iter().position(|a| a == &held.answer) {
                            counts[slot] += 1;
                        }
                    }
                }
                let valid: i64 = counts.iter().sum();
                let largest = counts.iter().copied().max().unwrap_or(0);
                let holders = counts.iter().filter(|c| **c == largest).count();
                let plurality = if valid > 0 && holders == 1 {
                    let slot = counts
                        .iter()
                        .position(|c| *c == largest)
                        .expect("a largest count");
                    Some(poll.answers[slot].clone())
                } else {
                    None
                };
                match &plurality {
                    Some(answer) => {
                        pluralities += 1;
                        paired.push(format!(
                            "plurality, {}, {}",
                            label.display(),
                            answer.display()
                        ));
                        let standing = counts[poll
                            .answers
                            .iter()
                            .position(|a| a == answer)
                            .expect("the plurality among the answers")];
                        let other = poll
                            .answers
                            .iter()
                            .zip(counts.iter())
                            .filter(|(a, _)| *a != answer)
                            .map(|(_, c)| *c)
                            .max()
                            .unwrap_or(0);
                        paired.push(format!("margin, {}, {}", label.display(), standing - other));
                        let names: Vec<String> = mine
                            .iter()
                            .filter(|held| {
                                !held.abstained
                                    && poll.answers.contains(&held.answer)
                                    && held.answer != *answer
                            })
                            .map(|held| held.juror.display())
                            .collect();
                        for name in &respondents {
                            if names.contains(name) {
                                counted += 1;
                                *per.entry(name.clone()).or_insert(0) += 1;
                            }
                        }
                    }
                    None => {
                        paired.push(format!("plurality, {}, none", label.display()));
                        paired.push(format!("margin, {}, not counted", label.display()));
                    }
                }
            }
        }
    }
    (
        paired,
        per,
        format!(
            "dissents, {} questions, {} pluralities, {} counted",
            questions, pluralities, counted
        ),
    )
}

#[test]
fn the_golden_plurality_and_margin_lines_agree_with_an_independent_computation() {
    let (paired, _, _) = direct_dissent(&recorded_sets());
    let golden = read_text(&examples().join("dissent-golden.txt"));
    let held: Vec<&str> = golden
        .lines()
        .filter(|line| line.starts_with("plurality, ") || line.starts_with("margin, "))
        .collect();
    assert_eq!(held, paired);
}

#[test]
fn the_golden_closing_line_agrees_with_an_independent_count() {
    let (_, _, closing) = direct_dissent(&recorded_sets());
    let golden = read_text(&examples().join("dissent-golden.txt"));
    let last = golden.lines().last().expect("a closing line");
    assert_eq!(last, closing);
}

#[test]
fn the_golden_dissents_reconcile_with_the_ledger_golden() {
    // The ledger golden states, per respondent block, the
    // matched count and the base of its agreement line.
    // The dissents of a respondent in the dissent golden
    // stand equal to that base minus that matched count,
    // the reconciliation GJ-15 states.
    let ledger = read_text(&examples().join("ledger-golden.txt"));
    let mut expected: BTreeMap<String, i64> = BTreeMap::new();
    let mut current: Option<String> = None;
    for line in ledger.lines() {
        if let Some(name) = line.strip_prefix("respondent block, ") {
            current = Some(name.to_string());
        }
        if let Some(rest) = line.strip_prefix("agreement, ") {
            let figures = rest.split(',').next().expect("the counts of the line");
            let mut words = figures.split(' ');
            let matched: i64 = words
                .next()
                .expect("a matched count")
                .parse()
                .expect("a figure");
            assert_eq!(words.next(), Some("of"));
            let base: i64 = words
                .next()
                .expect("a base count")
                .parse()
                .expect("a figure");
            let name = current.clone().expect("a respondent block");
            expected.insert(name, base - matched);
        }
    }
    assert_eq!(expected.len(), 10);
    let golden = read_text(&examples().join("dissent-golden.txt"));
    let mut counted: BTreeMap<String, i64> = BTreeMap::new();
    for line in golden.lines() {
        if let Some(name) = line.strip_prefix("dissenter, ") {
            *counted.entry(name.to_string()).or_insert(0) += 1;
        }
    }
    for (name, gap) in &expected {
        assert_eq!(counted.get(name).copied().unwrap_or(0), *gap, "{}", name);
    }
    for name in counted.keys() {
        assert!(expected.contains_key(name), "{}", name);
    }
    let (_, per, _) = direct_dissent(&recorded_sets());
    assert_eq!(counted, per);
}

#[test]
fn the_dissent_golden_carries_no_verdict_vocabulary() {
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
    let golden = read_text(&examples().join("dissent-golden.txt"));
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
        "dissent",
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
fn the_dissent_command_refuses_a_defective_poll() {
    let scratch = TempDir::new("dissent-defective");
    let first_dir = scratch.path.join("first");
    let second_dir = scratch.path.join("second");
    write_poll(&first_dir, 1, &door_first());
    std::fs::create_dir_all(&second_dir).expect("a poll directory");
    std::fs::write(second_dir.join("poll-01.json"), "{}\n").expect("a written poll");
    let (out, _, code) = run_cli(&[
        "dissent",
        "first",
        &path_arg(&first_dir),
        "second",
        &path_arg(&second_dir),
    ]);
    assert_eq!(code, 1);
    assert!(out.starts_with("invalid poll\ndefect "), "{}", out);
    assert!(!out.contains("dissents,"), "{}", out);
}

#[test]
fn the_dissent_command_refuses_a_missing_first_directory() {
    let scratch = TempDir::new("dissent-missing");
    let missing: PathBuf = scratch.path.join("nowhere");
    let (out, _, code) = run_cli(&["dissent", "pilot", &path_arg(&missing)]);
    assert_eq!(code, 1);
    assert_eq!(out, "invalid poll\ndefect file cannot be read\n");
}

#[test]
fn the_dissent_command_refuses_a_duplicate_label() {
    let (out, _, code) = run_cli(&[
        "dissent",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "pilot",
        &path_arg(&examples().join("wave")),
    ]);
    assert_eq!(code, 1);
    assert_eq!(
        out,
        "invalid dissent\ndefect the set label pilot stands twice\n"
    );
}

#[test]
fn the_dissent_command_refuses_a_label_holding_a_newline() {
    let (out, _, code) = run_cli(&[
        "dissent",
        "two\nlines",
        &path_arg(&examples().join("pilot")),
    ]);
    assert_eq!(code, 1);
    assert_eq!(out, "invalid dissent\ndefect a set label holds a newline\n");
}

#[test]
fn the_dissent_command_refuses_an_unpaired_label() {
    let (_, err, code) = run_cli(&["dissent", "pilot"]);
    assert_eq!(code, 2);
    assert!(
        err.contains("the following arguments are required: directory"),
        "{}",
        err
    );
    let (_, err, code) = run_cli(&["dissent"]);
    assert_eq!(code, 2);
    assert!(
        err.contains("the following arguments are required: label, directory"),
        "{}",
        err
    );
}

#[test]
fn the_dissent_command_stops_a_set_at_a_gap() {
    let scratch = TempDir::new("dissent-gap");
    let dir = scratch.path.join("set");
    write_poll(&dir, 1, &door_first());
    write_poll(&dir, 3, &door_first());
    let (out, _, code) = run_cli(&["dissent", "only", &path_arg(&dir)]);
    assert_eq!(code, 0);
    assert!(out.contains("dissents, 2 questions,"), "{}", out);
}

#[test]
fn the_dissent_command_writes_nothing() {
    let scratch = TempDir::new("dissent-purity");
    let first_dir = scratch.path.join("first");
    let second_dir = scratch.path.join("second");
    write_poll(&first_dir, 1, &door_first());
    write_poll(&second_dir, 1, &door_second());
    let snapshot = |dir: &Path| -> Vec<(String, Vec<u8>)> {
        let mut entries: Vec<(String, Vec<u8>)> = std::fs::read_dir(dir)
            .expect("a poll directory")
            .map(|entry| {
                let entry = entry.expect("a directory entry");
                (
                    entry.file_name().to_string_lossy().into_owned(),
                    std::fs::read(entry.path()).expect("poll bytes"),
                )
            })
            .collect();
        entries.sort();
        entries
    };
    let before_first = snapshot(&first_dir);
    let before_second = snapshot(&second_dir);
    let (_, _, code) = run_cli(&[
        "dissent",
        "first",
        &path_arg(&first_dir),
        "second",
        &path_arg(&second_dir),
    ]);
    assert_eq!(code, 0);
    assert_eq!(snapshot(&first_dir), before_first);
    assert_eq!(snapshot(&second_dir), before_second);
}

#[test]
fn the_dissent_help_names_its_pairs() {
    let (out, _, code) = run_cli(&["dissent", "--help"]);
    assert_eq!(code, 0);
    assert!(
        out.contains("usage: grandjury dissent [-h] label directory"),
        "{}",
        out
    );
    let (top, _, top_code) = run_cli(&["--help"]);
    assert_eq!(top_code, 0);
    assert!(
        top.contains("print the dissent reading over recorded poll sets"),
        "{}",
        top
    );
}
