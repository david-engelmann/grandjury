//! Pins for GJ-19 and GJ-20, the minority reading and
//! the minority command.
//!
//! The function pins build their polls by hand, so each one
//! fails under the negation of the rule it names, the pooled
//! plurality under the tie rule of GJ-8 as FIX-GJ-8 repairs
//! it read over the valid responses of the whole poll, the
//! dissenter of the pool as a valid answer that is not the
//! pooled plurality answer, the respondent order of GJ-13
//! over the poll order of an asking, the set refusals the
//! minority shares with the census, and the fencing of the
//! fields the reading prints. The recorded pins hold the
//! minority of the three recorded sets to the committed
//! golden through the function and through the command,
//! hold every line of the golden against an independent
//! computation over the recorded polls, hold the dissenters
//! figure of the golden against the per asking
//! reconciliation, hold the single jury askings of the
//! golden against the dissent golden of GJ-16, and hold the
//! pluralities and askings figures of the golden against
//! the swing golden of GJ-18. Every pin runs offline
//! against the recorded polls, and no pin makes a live
//! call.

mod shared;

use std::path::{Path, PathBuf};

use grandjury::dissent::dissent_lines;
use grandjury::minority::minority_lines;
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

fn write_poll(dir: &Path, index: usize, held: &Poll) {
    std::fs::create_dir_all(dir).expect("a poll directory");
    std::fs::write(
        dir.join(format!("poll-{:02}.json", index)),
        dumps_default(&held.to_json()),
    )
    .expect("a written poll");
}

/// The first asking of the door question. The jev jury holds
/// red by two to one and the chat jury holds blue by three
/// to none, so the pooled plurality is blue by four to two,
/// with a pooled margin of two that neither jury holds.
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
            response("openai-3", "chat-b", "blue", false),
        ],
    )
}

/// The second asking of the door question. Pooled, red
/// holds five and blue holds one.
fn door_second() -> Poll {
    poll(
        "Which door?",
        vec!["red", "blue", "green"],
        vec![
            response("jev-1", "jev-a", "red", false),
            response("jev-2", "jev-a", "red", false),
            response("jev-3", "jev-a", "red", false),
            response("openai-1", "chat-b", "red", false),
            response("openai-2", "chat-b", "red", false),
            response("openai-3", "chat-b", "blue", false),
        ],
    )
}

/// An asking of the door question at which the pooled
/// counts tie, two for red and two for blue, so no pooled
/// plurality stands.
fn door_tied() -> Poll {
    poll(
        "Which door?",
        vec!["red", "blue", "green"],
        vec![
            response("jev-1", "jev-a", "red", false),
            response("jev-2", "jev-a", "blue", false),
            response("openai-1", "chat-b", "red", false),
            response("openai-2", "chat-b", "blue", false),
        ],
    )
}

#[test]
fn the_minority_of_two_askings_stands_line_for_line() {
    let sets = vec![
        (pt("first"), vec![door_first()]),
        (pt("second"), vec![door_second()]),
    ];
    assert_eq!(
        minority_lines(&sets).expect("a reading"),
        vec![
            "question, Which door?",
            "plurality, first, blue",
            "margin, first, 2",
            "dissenter, jev-1",
            "dissenter, jev-2",
            "plurality, second, red",
            "margin, second, 4",
            "dissenter, openai-3",
            "minorities, 1 questions, 2 askings, 2 pluralities, 3 dissenters",
        ]
    );
}

#[test]
fn the_pooled_dissenters_are_not_the_jurys_dissenters() {
    // Against its own plurality the jev jury of the door
    // asking holds one dissenter, jev-3, and the chat jury
    // holds none. Against the pooled plurality the
    // dissenters are jev-1 and jev-2, who stood with their
    // own jury, and jev-3 is no dissenter of the pool,
    // because its answer is the pooled plurality answer.
    let sets = vec![(pt("first"), vec![door_first()])];
    let dissent = dissent_lines(&sets).expect("a reading");
    assert!(dissent.contains(&"dissenter, jev-3".to_string()));
    let minority = minority_lines(&sets).expect("a reading");
    let named: Vec<&String> = minority
        .iter()
        .filter(|line| line.starts_with("dissenter, "))
        .collect();
    assert_eq!(
        named,
        vec!["dissenter, jev-1", "dissenter, jev-2"],
        "the dissenters of the pool stand in respondent order"
    );
}

#[test]
fn the_dissenter_lines_stand_in_respondent_order_not_poll_order() {
    // The respondent late-1 first appears in the first set,
    // so it stands ahead of late-2 in the record order,
    // while the responses of the door asking itself stand
    // late-2 ahead of late-1. The dissenter lines follow
    // the record order, as GJ-15 states it for its own.
    let cup = poll(
        "Which cup?",
        vec!["big", "small"],
        vec![
            response("early", "jev-a", "big", false),
            response("late-1", "jev-a", "big", false),
        ],
    );
    let door = poll(
        "Which door?",
        vec!["red", "blue"],
        vec![
            response("late-2", "chat-b", "blue", false),
            response("late-1", "chat-b", "blue", false),
            response("early", "jev-a", "red", false),
            response("holder", "jev-a", "red", false),
            response("chief", "jev-a", "red", false),
        ],
    );
    let sets = vec![(pt("first"), vec![cup]), (pt("second"), vec![door])];
    assert_eq!(
        minority_lines(&sets).expect("a reading"),
        vec![
            "question, Which cup?",
            "plurality, first, big",
            "margin, first, 2",
            "question, Which door?",
            "plurality, second, red",
            "margin, second, 1",
            "dissenter, late-1",
            "dissenter, late-2",
            "minorities, 2 questions, 2 askings, 2 pluralities, 2 dissenters",
        ]
    );
}

#[test]
fn a_block_every_valid_response_agrees_with_holds_no_dissenter_line() {
    let unanimous = poll(
        "Which window?",
        vec!["wide", "narrow"],
        vec![
            response("jev-1", "jev-a", "wide", false),
            response("openai-1", "chat-b", "wide", false),
            response("openai-2", "chat-b", "wide", false),
        ],
    );
    let sets = vec![(pt("only"), vec![unanimous])];
    assert_eq!(
        minority_lines(&sets).expect("a reading"),
        vec![
            "question, Which window?",
            "plurality, only, wide",
            "margin, only, 3",
            "minorities, 1 questions, 1 askings, 1 pluralities, 0 dissenters",
        ]
    );
}

#[test]
fn an_asking_with_no_pooled_plurality_holds_no_dissenter_line() {
    let sets = vec![(pt("only"), vec![door_tied()])];
    assert_eq!(
        minority_lines(&sets).expect("a reading"),
        vec![
            "question, Which door?",
            "plurality, only, none",
            "margin, only, not counted",
            "minorities, 1 questions, 1 askings, 0 pluralities, 0 dissenters",
        ]
    );
}

#[test]
fn an_abstained_or_spoiled_response_is_never_a_dissenter() {
    // Four valid for red and two valid for blue, with one
    // spoiled answer and one abstention standing outside
    // the valid counts. The dissenters of the pool are the
    // two valid blue respondents alone, and neither the
    // spoiled respondent nor the abstained respondent
    // stands as a dissenter.
    let mixed = poll(
        "Which door?",
        vec!["red", "blue", "green"],
        vec![
            response("jev-1", "jev-a", "red", false),
            response("jev-2", "jev-a", "red", false),
            response("jev-3", "jev-a", "red", false),
            response("openai-1", "chat-b", "red", false),
            response("openai-2", "chat-b", "blue", false),
            response("openai-3", "chat-b", "blue", false),
            response("openai-4", "chat-b", "purple", false),
            response("openai-5", "chat-b", "green", true),
        ],
    );
    let sets = vec![(pt("only"), vec![mixed])];
    let lines = minority_lines(&sets).expect("a reading");
    assert_eq!(
        lines,
        vec![
            "question, Which door?",
            "plurality, only, red",
            "margin, only, 2",
            "dissenter, openai-2",
            "dissenter, openai-3",
            "minorities, 1 questions, 1 askings, 1 pluralities, 2 dissenters",
        ]
    );
}

#[test]
fn the_minority_refuses_the_set_defects_of_the_census() {
    let one = vec![door_first()];
    assert_eq!(
        minority_lines(&[(pt(""), one.clone())]),
        Err("a set label is empty".to_string())
    );
    assert_eq!(
        minority_lines(&[(pt("two\nlines"), one.clone())]),
        Err("a set label holds a newline".to_string())
    );
    assert_eq!(
        minority_lines(&[(pt("first"), one.clone()), (pt("first"), one.clone())]),
        Err("the set label first stands twice".to_string())
    );
    assert_eq!(
        minority_lines(&[(pt("first"), vec![door_first(), door_second()])]),
        Err("the set first holds the question Which door? twice".to_string())
    );
}

#[test]
fn a_label_carrying_the_closing_form_mints_no_closing_line() {
    let hostile = "minorities, 9 questions, 9 askings, 9 pluralities, 9 dissenters";
    let sets = vec![(pt(hostile), vec![door_second()])];
    let lines = minority_lines(&sets).expect("a reading");
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.starts_with("minorities,"))
            .count(),
        1
    );
    assert_eq!(
        lines.last().expect("a closing line"),
        "minorities, 1 questions, 1 askings, 1 pluralities, 1 dissenters"
    );
    assert!(
        lines[1].starts_with("plurality, minorities, 9 questions,"),
        "{}",
        lines[1]
    );
}

#[test]
fn a_label_carrying_a_dissenter_form_mints_no_dissenter_line() {
    let sets = vec![(pt("dissenter, forged"), vec![door_second()])];
    let lines = minority_lines(&sets).expect("a reading");
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.starts_with("dissenter, "))
            .count(),
        1
    );
    assert!(lines.contains(&"dissenter, openai-3".to_string()));
    assert!(!lines.contains(&"dissenter, forged".to_string()));
}

#[test]
fn a_juror_carrying_a_plurality_form_mints_no_plurality_line() {
    let held = poll(
        "Which door?",
        vec!["red", "blue"],
        vec![
            response("plurality, only, red", "jev-a", "blue", false),
            response("jev-2", "jev-a", "red", false),
            response("jev-3", "jev-a", "red", false),
        ],
    );
    let sets = vec![(pt("only"), vec![held])];
    let lines = minority_lines(&sets).expect("a reading");
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.starts_with("plurality, "))
            .count(),
        1
    );
    assert!(lines.contains(&"dissenter, plurality, only, red".to_string()));
}

#[test]
fn a_question_carrying_a_plurality_form_mints_no_plurality_line() {
    let hostile = poll(
        "plurality, only, red",
        vec!["red", "blue"],
        vec![response("jev-1", "jev-a", "red", false)],
    );
    let sets = vec![(pt("only"), vec![hostile])];
    let lines = minority_lines(&sets).expect("a reading");
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.starts_with("plurality, "))
            .count(),
        1
    );
    assert_eq!(lines[0], "question, plurality, only, red");
    assert_eq!(lines[1], "plurality, only, red");
}

/// The pooled plurality of one poll counted directly from
/// its responses, the answer holding the largest valid
/// count alone, with its count and the valid count of the
/// poll, or nothing where no plurality stands.
fn direct_plurality(held: &Poll) -> Option<(String, usize, usize)> {
    let mut counts = vec![0usize; held.answers.len()];
    let mut valid = 0usize;
    for response in &held.responses {
        if response.abstained {
            continue;
        }
        if let Some(slot) = held.answers.iter().position(|a| *a == response.answer) {
            counts[slot] += 1;
            valid += 1;
        }
    }
    let largest = counts.iter().copied().max().unwrap_or(0);
    let holders: Vec<usize> = counts
        .iter()
        .enumerate()
        .filter(|(_, count)| **count == largest)
        .map(|(index, _)| index)
        .collect();
    if valid > 0 && holders.len() == 1 {
        let slot = holders[0];
        Some((held.answers[slot].display(), counts[slot], valid))
    } else {
        None
    }
}

/// The minority reading computed in the test from the
/// recorded responses alone, sharing no code with the
/// reading under test, the questions in the order they
/// first appear, each pooled plurality counted over the
/// whole poll, and the dissenters of an asking named in
/// the order the jurors first appear across the record.
fn direct_minority(sets: &[(PText, Vec<Poll>)]) -> Vec<String> {
    let mut respondents: Vec<String> = Vec::new();
    for (_, polls) in sets {
        for held in polls {
            for response in &held.responses {
                let juror = response.juror.display();
                if !respondents.contains(&juror) {
                    respondents.push(juror);
                }
            }
        }
    }
    let mut questions: Vec<String> = Vec::new();
    let mut askings: Vec<Vec<(String, &Poll)>> = Vec::new();
    for (label, polls) in sets {
        for held in polls {
            let question = held.question.display();
            match questions.iter().position(|name| *name == question) {
                Some(slot) => askings[slot].push((label.display(), held)),
                None => {
                    questions.push(question);
                    askings.push(vec![(label.display(), held)]);
                }
            }
        }
    }
    let mut lines: Vec<String> = Vec::new();
    let mut asking_total = 0usize;
    let mut plurality_total = 0usize;
    let mut dissenter_total = 0usize;
    for (question, question_askings) in questions.iter().zip(askings.iter()) {
        lines.push(format!("question, {}", question));
        for (label, held) in question_askings {
            asking_total += 1;
            match direct_plurality(held) {
                Some((answer, standing, _)) => {
                    plurality_total += 1;
                    let other = held
                        .answers
                        .iter()
                        .enumerate()
                        .filter(|(_, name)| name.display() != answer)
                        .map(|(index, _)| {
                            held.responses
                                .iter()
                                .filter(|response| {
                                    !response.abstained && held.answers[index] == response.answer
                                })
                                .count()
                        })
                        .max()
                        .unwrap_or(0);
                    lines.push(format!("plurality, {}, {}", label, answer));
                    lines.push(format!("margin, {}, {}", label, standing - other));
                    let mut named: Vec<String> = Vec::new();
                    for response in &held.responses {
                        if response.abstained || !held.answers.contains(&response.answer) {
                            continue;
                        }
                        if response.answer.display() != answer {
                            let juror = response.juror.display();
                            if !named.contains(&juror) {
                                named.push(juror);
                            }
                        }
                    }
                    for respondent in &respondents {
                        if named.contains(respondent) {
                            lines.push(format!("dissenter, {}", respondent));
                            dissenter_total += 1;
                        }
                    }
                }
                None => {
                    lines.push(format!("plurality, {}, none", label));
                    lines.push(format!("margin, {}, not counted", label));
                }
            }
        }
    }
    lines.push(format!(
        "minorities, {} questions, {} askings, {} pluralities, {} dissenters",
        questions.len(),
        asking_total,
        plurality_total,
        dissenter_total
    ));
    lines
}

#[test]
fn the_minority_of_the_recorded_sets_stands_line_for_line() {
    let golden = read_text(&examples().join("minority-golden.txt"));
    let held: Vec<String> = golden.lines().map(|line| line.to_string()).collect();
    assert_eq!(minority_lines(&recorded_sets()).expect("a reading"), held);
}

#[test]
fn every_line_of_the_golden_matches_an_independent_computation() {
    let golden = read_text(&examples().join("minority-golden.txt"));
    let held: Vec<String> = golden.lines().map(|line| line.to_string()).collect();
    assert_eq!(direct_minority(&recorded_sets()), held);
}

#[test]
fn the_closing_line_of_the_golden_matches_an_independent_count() {
    let golden = read_text(&examples().join("minority-golden.txt"));
    let questions = golden
        .lines()
        .filter(|line| line.starts_with("question, "))
        .count();
    let blocks: Vec<&str> = golden
        .lines()
        .filter(|line| line.starts_with("plurality, "))
        .collect();
    let pluralities = blocks
        .iter()
        .filter(|line| !line.ends_with(", none"))
        .count();
    let dissenters = golden
        .lines()
        .filter(|line| line.starts_with("dissenter, "))
        .count();
    assert_eq!(blocks.len(), 64);
    assert_eq!(
        golden.lines().last().expect("a closing line"),
        format!(
            "minorities, {} questions, {} askings, {} pluralities, {} dissenters",
            questions,
            blocks.len(),
            pluralities,
            dissenters
        )
    );
}

#[test]
fn the_dissenters_figure_matches_the_per_asking_reconciliation() {
    // At every decided asking the dissenters of the block
    // are the valid responses of the poll minus the pooled
    // plurality count, so the dissenters figure of the
    // golden is the sum of those differences over the
    // recorded polls, computed here from the responses
    // alone.
    let sets = recorded_sets();
    let mut expected = 0usize;
    for (_, polls) in &sets {
        for held in polls {
            if let Some((_, standing, valid)) = direct_plurality(held) {
                expected += valid - standing;
            }
        }
    }
    let golden = read_text(&examples().join("minority-golden.txt"));
    let closing = golden.lines().last().expect("a closing line");
    let figure: usize = closing
        .rsplit(", ")
        .next()
        .expect("a dissenters tail")
        .trim_end_matches(" dissenters")
        .parse()
        .expect("a dissenters figure");
    assert_eq!(figure, expected);
    assert!(expected > 0, "the record holds pooled dissent");
}

#[test]
fn the_single_jury_askings_agree_with_the_dissent_golden() {
    let sets = recorded_sets();
    // The dissenter lines of the dissent golden stand in
    // part order, set order and poll order and jury order,
    // each part opening with its question line and its
    // plurality line carrying its set label.
    let dissent = read_text(&examples().join("dissent-golden.txt"));
    let mut parts: Vec<(String, String, Vec<String>)> = Vec::new();
    let mut current: Option<(String, String, Vec<String>)> = None;
    for line in dissent.lines() {
        if let Some(tail) = line.strip_prefix("question, ") {
            if let Some(part) = current.take() {
                parts.push(part);
            }
            current = Some((tail.to_string(), String::new(), Vec::new()));
        } else if let Some(tail) = line.strip_prefix("plurality, ") {
            let label = tail.split(", ").next().expect("a set label").to_string();
            if let Some(part) = current.as_mut() {
                part.1 = label;
            }
        } else if let Some(tail) = line.strip_prefix("dissenter, ") {
            if let Some(part) = current.as_mut() {
                part.2.push(tail.to_string());
            }
        }
    }
    if let Some(part) = current.take() {
        parts.push(part);
    }
    // The dissenter lines of the minority golden stand in
    // blocks under their question sections, each block
    // opening with its plurality line.
    let minority = read_text(&examples().join("minority-golden.txt"));
    let mut blocks: Vec<(String, String, Vec<String>)> = Vec::new();
    let mut question = String::new();
    for line in minority.lines() {
        if let Some(tail) = line.strip_prefix("question, ") {
            question = tail.to_string();
        } else if let Some(tail) = line.strip_prefix("plurality, ") {
            let label = tail.split(", ").next().expect("a set label").to_string();
            blocks.push((question.clone(), label, Vec::new()));
        } else if let Some(tail) = line.strip_prefix("dissenter, ") {
            if let Some(block) = blocks.last_mut() {
                block.2.push(tail.to_string());
            }
        }
    }
    let mut compared = 0usize;
    for (label, polls) in &sets {
        for held in polls {
            let mut models: Vec<String> = Vec::new();
            for response in &held.responses {
                let model = response.model.display();
                if !models.contains(&model) {
                    models.push(model);
                }
            }
            if models.len() == 1 {
                let key = (held.question.display(), label.display());
                let part = parts
                    .iter()
                    .find(|part| part.0 == key.0 && part.1 == key.1)
                    .expect("a dissent part for the asking");
                let block = blocks
                    .iter()
                    .find(|block| block.0 == key.0 && block.1 == key.1)
                    .expect("a minority block for the asking");
                assert_eq!(block.2, part.2, "{}", held.question.display());
                compared += 1;
            }
        }
    }
    assert!(compared >= 3, "the record holds single jury polls");
}

#[test]
fn the_pluralities_and_askings_figures_agree_with_the_swing_golden() {
    let swing = read_text(&examples().join("swing-golden.txt"));
    let mut standing = 0usize;
    let mut askings = 0usize;
    for line in swing.lines() {
        if line.starts_with("asking, ") {
            askings += 1;
            if !line.ends_with(", margin not counted") {
                standing += 1;
            }
        }
    }
    let swing_askings: usize = swing
        .lines()
        .last()
        .expect("a closing line")
        .rsplit(", ")
        .next()
        .expect("an askings tail")
        .trim_end_matches(" askings")
        .parse()
        .expect("an askings figure");
    assert_eq!(askings, swing_askings);
    let minority = read_text(&examples().join("minority-golden.txt"));
    let closing = minority.lines().last().expect("a closing line");
    let fields: Vec<&str> = closing.split(", ").collect();
    let minority_askings: usize = fields[2]
        .trim_end_matches(" askings")
        .parse()
        .expect("an askings figure");
    let minority_pluralities: usize = fields[3]
        .trim_end_matches(" pluralities")
        .parse()
        .expect("a pluralities figure");
    assert_eq!(minority_askings, askings);
    assert_eq!(minority_pluralities, standing);
}

#[test]
fn the_minority_golden_carries_no_verdict_vocabulary() {
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
    let golden = read_text(&examples().join("minority-golden.txt"));
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
        "minority",
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
fn the_minority_command_prints_the_recorded_golden() {
    let golden = read_text(&examples().join("minority-golden.txt"));
    let (out, _, code) = run_cli(&[
        "minority",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "full",
        &path_arg(&examples().join("full")),
        "wave",
        &path_arg(&examples().join("wave")),
    ]);
    assert_eq!(code, 0);
    assert_eq!(out, golden);
}

#[test]
fn the_minority_command_refuses_a_defective_poll() {
    let scratch = TempDir::new("minority-defective");
    let first_dir = scratch.path.join("first");
    let second_dir = scratch.path.join("second");
    write_poll(&first_dir, 1, &door_first());
    std::fs::create_dir_all(&second_dir).expect("a poll directory");
    std::fs::write(second_dir.join("poll-01.json"), "{}\n").expect("a written poll");
    let (out, _, code) = run_cli(&[
        "minority",
        "first",
        &path_arg(&first_dir),
        "second",
        &path_arg(&second_dir),
    ]);
    assert_eq!(code, 1);
    assert!(out.starts_with("invalid poll\ndefect "), "{}", out);
    assert!(!out.contains("minorities,"), "{}", out);
}

#[test]
fn the_minority_command_refuses_a_missing_first_directory() {
    let scratch = TempDir::new("minority-missing");
    let missing: PathBuf = scratch.path.join("nowhere");
    let (out, _, code) = run_cli(&["minority", "pilot", &path_arg(&missing)]);
    assert_eq!(code, 1);
    assert_eq!(out, "invalid poll\ndefect file cannot be read\n");
}

#[test]
fn the_minority_command_refuses_a_duplicate_label() {
    let (out, _, code) = run_cli(&[
        "minority",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "pilot",
        &path_arg(&examples().join("wave")),
    ]);
    assert_eq!(code, 1);
    assert_eq!(
        out,
        "invalid minority\ndefect the set label pilot stands twice\n"
    );
}

#[test]
fn the_minority_command_refuses_a_label_holding_a_newline() {
    let (out, _, code) = run_cli(&[
        "minority",
        "two\nlines",
        &path_arg(&examples().join("pilot")),
    ]);
    assert_eq!(code, 1);
    assert_eq!(
        out,
        "invalid minority\ndefect a set label holds a newline\n"
    );
}

#[test]
fn the_minority_command_refuses_an_unpaired_label() {
    let (_, err, code) = run_cli(&["minority", "pilot"]);
    assert_eq!(code, 2);
    assert!(
        err.contains("the following arguments are required: directory"),
        "{}",
        err
    );
    let (_, err, code) = run_cli(&["minority"]);
    assert_eq!(code, 2);
    assert!(
        err.contains("the following arguments are required: label, directory"),
        "{}",
        err
    );
}

#[test]
fn the_minority_command_stops_a_set_at_a_gap() {
    let scratch = TempDir::new("minority-gap");
    let dir = scratch.path.join("set");
    write_poll(&dir, 1, &door_first());
    write_poll(&dir, 3, &door_first());
    let (out, _, code) = run_cli(&["minority", "only", &path_arg(&dir)]);
    assert_eq!(code, 0);
    assert!(out.contains("minorities, 1 questions,"), "{}", out);
    assert!(out.contains("1 askings"), "{}", out);
}

#[test]
fn the_minority_command_writes_nothing() {
    let scratch = TempDir::new("minority-purity");
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
        "minority",
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
fn the_minority_help_names_its_pairs() {
    let (out, _, code) = run_cli(&["minority", "--help"]);
    assert_eq!(code, 0);
    assert!(
        out.contains("usage: grandjury minority [-h] label directory"),
        "{}",
        out
    );
    let (top, _, top_code) = run_cli(&["--help"]);
    assert_eq!(top_code, 0);
    assert!(
        top.contains("print the minority reading over recorded poll sets"),
        "{}",
        top
    );
}
