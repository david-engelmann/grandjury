//! Pins for GJ-17 and GJ-18, the swing reading and
//! the swing command.
//!
//! The function pins build their polls by hand, so each one
//! fails under the negation of the rule it names, the pooled
//! plurality under the tie rule of GJ-8 as FIX-GJ-8 repairs
//! it read over the valid responses of the whole poll, the
//! margin as an exact difference of pooled counts, the three
//! courses and the case that divides the course from the
//! movement of GJ-11, the set refusals the swing shares with
//! the census, and the fencing of the fields the reading
//! prints. The recorded pins hold the swing of the three
//! recorded sets to the committed golden through the function
//! and through the command, hold every line of the golden
//! against an independent computation over the recorded
//! polls, hold the single jury askings of the golden against
//! the dissent golden of GJ-16, and hold the pooled valid
//! sum over the askings against the record line of the
//! census golden of GJ-12. Every pin runs offline against
//! the recorded polls, and no pin makes a live call.

mod shared;

use std::path::{Path, PathBuf};

use grandjury::census::census_lines;
use grandjury::dissent::dissent_lines;
use grandjury::polls::{load_poll_file, Poll, Response};
use grandjury::pytext::PText;
use grandjury::swing::swing_lines;
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
fn the_swing_of_two_askings_stands_line_for_line() {
    let sets = vec![
        (pt("first"), vec![door_first()]),
        (pt("second"), vec![door_second()]),
    ];
    assert_eq!(
        swing_lines(&sets).expect("a reading"),
        vec![
            "question, Which door?",
            "asking, first, blue, margin 2",
            "asking, second, red, margin 4",
            "course, swung",
            "swings, 1 questions, 0 held, 1 swung, 0 never decided, 2 askings",
        ]
    );
}

#[test]
fn the_pooled_plurality_pools_across_the_juries() {
    let sets = vec![(pt("first"), vec![door_first()])];
    let dissent = dissent_lines(&sets).expect("a reading");
    assert!(dissent.contains(&"plurality, first, red".to_string()));
    assert!(dissent.contains(&"margin, first, 1".to_string()));
    assert!(dissent.contains(&"margin, first, 3".to_string()));
    let swing = swing_lines(&sets).expect("a reading");
    assert_eq!(
        swing[1], "asking, first, blue, margin 2",
        "the pooled plurality and its margin are the pools"
    );
}

#[test]
fn a_question_asked_once_reads_held_where_its_plurality_stands() {
    let sets = vec![(pt("only"), vec![door_second()])];
    assert_eq!(
        swing_lines(&sets).expect("a reading"),
        vec![
            "question, Which door?",
            "asking, only, red, margin 4",
            "course, held",
            "swings, 1 questions, 1 held, 0 swung, 0 never decided, 1 askings",
        ]
    );
}

#[test]
fn a_question_asked_once_reads_never_decided_where_none_stands() {
    let sets = vec![(pt("only"), vec![door_tied()])];
    assert_eq!(
        swing_lines(&sets).expect("a reading"),
        vec![
            "question, Which door?",
            "asking, only, none, margin not counted",
            "course, never decided",
            "swings, 1 questions, 0 held, 0 swung, 1 never decided, 1 askings",
        ]
    );
}

#[test]
fn a_swing_runs_through_an_asking_that_decided_nothing() {
    let sets = vec![
        (pt("first"), vec![door_second()]),
        (pt("second"), vec![door_tied()]),
        (pt("third"), vec![door_first()]),
    ];
    assert_eq!(
        swing_lines(&sets).expect("a reading"),
        vec![
            "question, Which door?",
            "asking, first, red, margin 4",
            "asking, second, none, margin not counted",
            "asking, third, blue, margin 2",
            "course, swung",
            "swings, 1 questions, 0 held, 1 swung, 0 never decided, 3 askings",
        ]
    );
}

#[test]
fn an_agreement_across_a_none_asking_reads_held() {
    let sets = vec![
        (pt("first"), vec![door_second()]),
        (pt("second"), vec![door_tied()]),
        (pt("third"), vec![door_second()]),
    ];
    let lines = swing_lines(&sets).expect("a reading");
    assert_eq!(lines[4], "course, held");
    assert_eq!(
        lines[5],
        "swings, 1 questions, 1 held, 0 swung, 0 never decided, 3 askings"
    );
}

#[test]
fn a_none_asking_before_the_only_standing_reads_held() {
    // The course reads held where the movement of GJ-11
    // reads not counted for both juries, because each jury
    // holds a plurality at one asking alone while the
    // pooled plurality of the question stands there and
    // never changed.
    let sets = vec![
        (pt("first"), vec![door_tied()]),
        (pt("second"), vec![door_second()]),
    ];
    let lines = swing_lines(&sets).expect("a reading");
    assert_eq!(lines[3], "course, held");
    let census = census_lines(&sets).expect("a reading");
    assert!(census.contains(&"movement, jev-a, not counted".to_string()));
    assert!(census.contains(&"movement, chat-b, not counted".to_string()));
}

#[test]
fn a_question_deciding_nothing_at_any_asking_reads_never_decided() {
    let silent = poll(
        "Which door?",
        vec!["red", "blue", "green"],
        vec![
            response("jev-1", "jev-a", "red", true),
            response("openai-1", "chat-b", "blue", true),
        ],
    );
    let sets = vec![
        (pt("first"), vec![door_tied()]),
        (pt("second"), vec![silent]),
    ];
    assert_eq!(
        swing_lines(&sets).expect("a reading"),
        vec![
            "question, Which door?",
            "asking, first, none, margin not counted",
            "asking, second, none, margin not counted",
            "course, never decided",
            "swings, 1 questions, 0 held, 0 swung, 1 never decided, 2 askings",
        ]
    );
}

#[test]
fn the_margin_counts_the_whole_poll_and_ignores_the_unvalid() {
    // Four valid for red and two valid for blue, with one
    // spoiled answer and one abstention standing outside
    // the valid counts, so the pooled plurality is red
    // with a margin of two. Where every valid response
    // chose the plurality answer the margin is its whole
    // count, the largest other count standing at zero.
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
    let unanimous = poll(
        "Which window?",
        vec!["wide", "narrow"],
        vec![
            response("jev-1", "jev-a", "wide", false),
            response("openai-1", "chat-b", "wide", false),
            response("openai-2", "chat-b", "wide", false),
        ],
    );
    let sets = vec![(pt("only"), vec![mixed, unanimous])];
    assert_eq!(
        swing_lines(&sets).expect("a reading"),
        vec![
            "question, Which door?",
            "asking, only, red, margin 2",
            "course, held",
            "question, Which window?",
            "asking, only, wide, margin 3",
            "course, held",
            "swings, 2 questions, 2 held, 0 swung, 0 never decided, 2 askings",
        ]
    );
}

#[test]
fn the_swing_refuses_the_set_defects_of_the_census() {
    let one = vec![door_first()];
    assert_eq!(
        swing_lines(&[(pt(""), one.clone())]),
        Err("a set label is empty".to_string())
    );
    assert_eq!(
        swing_lines(&[(pt("two\nlines"), one.clone())]),
        Err("a set label holds a newline".to_string())
    );
    assert_eq!(
        swing_lines(&[(pt("first"), one.clone()), (pt("first"), one.clone())]),
        Err("the set label first stands twice".to_string())
    );
    assert_eq!(
        swing_lines(&[(pt("first"), vec![door_first(), door_second()])]),
        Err("the set first holds the question Which door? twice".to_string())
    );
}

#[test]
fn a_label_carrying_the_closing_form_mints_no_closing_line() {
    let hostile = "swings, 9 questions, 9 held, 0 swung, 0 never decided, 9 askings";
    let sets = vec![(pt(hostile), vec![door_second()])];
    let lines = swing_lines(&sets).expect("a reading");
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.starts_with("swings,"))
            .count(),
        1
    );
    assert_eq!(
        lines.last().expect("a closing line"),
        "swings, 1 questions, 1 held, 0 swung, 0 never decided, 1 askings"
    );
    assert!(
        lines[1].starts_with("asking, swings, 9 questions,"),
        "{}",
        lines[1]
    );
}

#[test]
fn a_label_carrying_the_course_form_mints_no_course_line() {
    let sets = vec![(pt("course, swung"), vec![door_second()])];
    let lines = swing_lines(&sets).expect("a reading");
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.starts_with("course,"))
            .count(),
        1
    );
    assert_eq!(lines[2], "course, held");
}

#[test]
fn a_question_carrying_an_asking_form_mints_no_asking_line() {
    let hostile = poll(
        "asking, only, red, margin 4",
        vec!["red", "blue"],
        vec![response("jev-1", "jev-a", "red", false)],
    );
    let sets = vec![(pt("only"), vec![hostile])];
    let lines = swing_lines(&sets).expect("a reading");
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.starts_with("asking,"))
            .count(),
        1
    );
    assert_eq!(lines[0], "question, asking, only, red, margin 4");
    assert_eq!(lines[1], "asking, only, red, margin 1");
}

/// The swing reading computed in the test from the recorded
/// responses alone, sharing no code with the reading under
/// test, the questions in the order they first appear and
/// each pooled plurality counted over the whole poll.
fn direct_swing(sets: &[(PText, Vec<Poll>)]) -> Vec<String> {
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
    let (mut held_count, mut swung_count, mut never_count) = (0usize, 0usize, 0usize);
    let mut asking_total = 0usize;
    for (question, question_askings) in questions.iter().zip(askings.iter()) {
        lines.push(format!("question, {}", question));
        let mut standing: Vec<String> = Vec::new();
        for (label, held) in question_askings {
            asking_total += 1;
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
                let answer = held.answers[slot].display();
                let other = counts
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| *index != slot)
                    .map(|(_, count)| *count)
                    .max()
                    .unwrap_or(0);
                lines.push(format!(
                    "asking, {}, {}, margin {}",
                    label,
                    answer,
                    counts[slot] - other
                ));
                standing.push(answer);
            } else {
                lines.push(format!("asking, {}, none, margin not counted", label));
            }
        }
        let course = if standing.is_empty() {
            never_count += 1;
            "never decided"
        } else if standing.iter().all(|answer| *answer == standing[0]) {
            held_count += 1;
            "held"
        } else {
            swung_count += 1;
            "swung"
        };
        lines.push(format!("course, {}", course));
    }
    lines.push(format!(
        "swings, {} questions, {} held, {} swung, {} never decided, {} askings",
        questions.len(),
        held_count,
        swung_count,
        never_count,
        asking_total
    ));
    lines
}

#[test]
fn the_swing_of_the_recorded_sets_stands_line_for_line() {
    let golden = read_text(&examples().join("swing-golden.txt"));
    let held: Vec<String> = golden.lines().map(|line| line.to_string()).collect();
    assert_eq!(swing_lines(&recorded_sets()).expect("a reading"), held);
}

#[test]
fn every_line_of_the_golden_matches_an_independent_computation() {
    let golden = read_text(&examples().join("swing-golden.txt"));
    let held: Vec<String> = golden.lines().map(|line| line.to_string()).collect();
    assert_eq!(direct_swing(&recorded_sets()), held);
}

#[test]
fn the_closing_line_of_the_golden_matches_an_independent_count() {
    let golden = read_text(&examples().join("swing-golden.txt"));
    let questions = golden
        .lines()
        .filter(|line| line.starts_with("question,"))
        .count();
    let askings = golden
        .lines()
        .filter(|line| line.starts_with("asking,"))
        .count();
    let held = golden
        .lines()
        .filter(|line| *line == "course, held")
        .count();
    let swung = golden
        .lines()
        .filter(|line| *line == "course, swung")
        .count();
    let never = golden
        .lines()
        .filter(|line| *line == "course, never decided")
        .count();
    assert_eq!(questions, held + swung + never);
    assert_eq!(askings, 64);
    assert_eq!(
        golden.lines().last().expect("a closing line"),
        format!(
            "swings, {} questions, {} held, {} swung, {} never decided, {} askings",
            questions, held, swung, never, askings
        )
    );
}

#[test]
fn the_single_jury_askings_agree_with_the_dissent_golden() {
    let sets = recorded_sets();
    let dissent = read_text(&examples().join("dissent-golden.txt"));
    // The plurality and margin lines of the dissent golden
    // stand in part order, set order and poll order and jury
    // order, each plurality line followed by its margin line.
    let dissent_lines: Vec<&str> = dissent.lines().collect();
    let mut pairs: Vec<(String, String)> = Vec::new();
    let mut index = 0usize;
    while index < dissent_lines.len() {
        if dissent_lines[index].starts_with("plurality, ") {
            pairs.push((
                dissent_lines[index].to_string(),
                dissent_lines[index + 1].to_string(),
            ));
            index += 2;
        } else {
            index += 1;
        }
    }
    let swing = read_text(&examples().join("swing-golden.txt"));
    let swing_lines: Vec<&str> = swing.lines().collect();
    let mut part = 0usize;
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
                let (plurality_line, margin_line) = &pairs[part];
                let answer = plurality_line
                    .strip_prefix(&format!("plurality, {}, ", label.display()))
                    .expect("a plurality tail");
                let margin = margin_line
                    .strip_prefix(&format!("margin, {}, ", label.display()))
                    .expect("a margin tail");
                let section = swing_lines
                    .iter()
                    .position(|line| *line == format!("question, {}", held.question.display()))
                    .expect("a section");
                let asking = swing_lines[section + 1..]
                    .iter()
                    .find(|line| line.starts_with(&format!("asking, {}, ", label.display())))
                    .expect("an asking line");
                assert_eq!(
                    *asking,
                    format!("asking, {}, {}, margin {}", label.display(), answer, margin),
                    "{}",
                    held.question.display()
                );
                compared += 1;
            }
            part += models.len();
        }
    }
    assert!(compared >= 3, "the record holds single jury polls");
}

#[test]
fn the_valid_sum_over_the_askings_equals_the_census_record_line() {
    let sets = recorded_sets();
    let census = read_text(&examples().join("census-golden.txt"));
    let record = census
        .lines()
        .find(|line| line.starts_with("record, "))
        .expect("a record line");
    let fields: Vec<&str> = record.split(", ").collect();
    let polls_figure: usize = fields[1]
        .trim_start_matches("polls ")
        .parse()
        .expect("a polls figure");
    let valid_figure: usize = fields[3]
        .trim_start_matches("valid ")
        .parse()
        .expect("a valid figure");
    let mut summed = 0usize;
    for (_, polls) in &sets {
        for held in polls {
            summed += held
                .responses
                .iter()
                .filter(|response| !response.abstained && held.answers.contains(&response.answer))
                .count();
        }
    }
    assert_eq!(summed, valid_figure);
    let swing = swing_lines(&sets).expect("a reading");
    let closing = swing.last().expect("a closing line");
    let askings_figure: usize = closing
        .rsplit(", ")
        .next()
        .expect("an askings tail")
        .trim_end_matches(" askings")
        .parse()
        .expect("an askings figure");
    assert_eq!(askings_figure, polls_figure);
}

#[test]
fn the_swing_golden_carries_no_verdict_vocabulary() {
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
    let golden = read_text(&examples().join("swing-golden.txt"));
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
        "swing",
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
fn the_swing_command_prints_the_recorded_golden() {
    let golden = read_text(&examples().join("swing-golden.txt"));
    let (out, _, code) = run_cli(&[
        "swing",
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
fn the_swing_command_refuses_a_defective_poll() {
    let scratch = TempDir::new("swing-defective");
    let first_dir = scratch.path.join("first");
    let second_dir = scratch.path.join("second");
    write_poll(&first_dir, 1, &door_first());
    std::fs::create_dir_all(&second_dir).expect("a poll directory");
    std::fs::write(second_dir.join("poll-01.json"), "{}\n").expect("a written poll");
    let (out, _, code) = run_cli(&[
        "swing",
        "first",
        &path_arg(&first_dir),
        "second",
        &path_arg(&second_dir),
    ]);
    assert_eq!(code, 1);
    assert!(out.starts_with("invalid poll\ndefect "), "{}", out);
    assert!(!out.contains("swings,"), "{}", out);
}

#[test]
fn the_swing_command_refuses_a_missing_first_directory() {
    let scratch = TempDir::new("swing-missing");
    let missing: PathBuf = scratch.path.join("nowhere");
    let (out, _, code) = run_cli(&["swing", "pilot", &path_arg(&missing)]);
    assert_eq!(code, 1);
    assert_eq!(out, "invalid poll\ndefect file cannot be read\n");
}

#[test]
fn the_swing_command_refuses_a_duplicate_label() {
    let (out, _, code) = run_cli(&[
        "swing",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "pilot",
        &path_arg(&examples().join("wave")),
    ]);
    assert_eq!(code, 1);
    assert_eq!(
        out,
        "invalid swing\ndefect the set label pilot stands twice\n"
    );
}

#[test]
fn the_swing_command_refuses_a_label_holding_a_newline() {
    let (out, _, code) = run_cli(&["swing", "two\nlines", &path_arg(&examples().join("pilot"))]);
    assert_eq!(code, 1);
    assert_eq!(out, "invalid swing\ndefect a set label holds a newline\n");
}

#[test]
fn the_swing_command_refuses_an_unpaired_label() {
    let (_, err, code) = run_cli(&["swing", "pilot"]);
    assert_eq!(code, 2);
    assert!(
        err.contains("the following arguments are required: directory"),
        "{}",
        err
    );
    let (_, err, code) = run_cli(&["swing"]);
    assert_eq!(code, 2);
    assert!(
        err.contains("the following arguments are required: label, directory"),
        "{}",
        err
    );
}

#[test]
fn the_swing_command_stops_a_set_at_a_gap() {
    let scratch = TempDir::new("swing-gap");
    let dir = scratch.path.join("set");
    write_poll(&dir, 1, &door_first());
    write_poll(&dir, 3, &door_first());
    let (out, _, code) = run_cli(&["swing", "only", &path_arg(&dir)]);
    assert_eq!(code, 0);
    assert!(out.contains("swings, 1 questions,"), "{}", out);
    assert!(out.contains("1 askings"), "{}", out);
}

#[test]
fn the_swing_command_writes_nothing() {
    let scratch = TempDir::new("swing-purity");
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
        "swing",
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
fn the_swing_help_names_its_pairs() {
    let (out, _, code) = run_cli(&["swing", "--help"]);
    assert_eq!(code, 0);
    assert!(
        out.contains("usage: grandjury swing [-h] label directory"),
        "{}",
        out
    );
    let (top, _, top_code) = run_cli(&["--help"]);
    assert_eq!(top_code, 0);
    assert!(
        top.contains("print the swing reading over recorded poll sets"),
        "{}",
        top
    );
}
