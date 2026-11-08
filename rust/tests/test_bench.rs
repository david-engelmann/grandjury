//! Pins for GJ-23 and GJ-24, the bench reading and the
//! bench command.
//!
//! The function pins build their polls by hand, so each one
//! fails under the negation of the rule it names, the model
//! order of first appearance across the sets, the pooling
//! of one model over the juror names it answered under, the
//! plurality of GJ-8 read over the pooled responses of the
//! model and over the valid responses of the whole poll,
//! and the set refusals the bench shares with the census.
//! The recorded pins hold the bench of the three recorded
//! sets to the committed golden through the function and
//! through the command, hold every line of the golden
//! against an independent computation over the recorded
//! polls, hold the closing line against an independent
//! count, hold the answers figure against the valid figure
//! of the census golden, and hold the plurality figure
//! against the sum of the pooled plurality counts of the
//! recorded askings. Every pin runs offline against the
//! recorded polls, and no pin makes a live call.

mod shared;

use std::path::{Path, PathBuf};

use grandjury::bench::bench_lines;
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

fn door_first() -> Poll {
    poll(
        "Which door?",
        vec!["red", "blue", "green"],
        vec![
            response("seat-1", "zeta", "red", false),
            response("seat-2", "zeta", "blue", false),
            response("seat-3", "alpha", "red", false),
            response("seat-4", "alpha", "red", false),
            response("seat-5", "alpha", "blue", false),
        ],
    )
}

fn door_second() -> Poll {
    poll(
        "Which door?",
        vec!["red", "blue", "green"],
        vec![
            response("seat-6", "zeta", "blue", false),
            response("seat-7", "zeta", "blue", false),
            response("seat-8", "alpha", "red", false),
        ],
    )
}

fn cup_second() -> Poll {
    poll(
        "Which cup?",
        vec!["big", "small"],
        vec![
            response("seat-9", "zeta", "huge", false),
            response("seat-10", "alpha", "small", false),
            response("seat-11", "alpha", "small", false),
            response("seat-12", "ghost", "small", true),
        ],
    )
}

fn hand_sets() -> Vec<(PText, Vec<Poll>)> {
    vec![
        (pt("first"), vec![door_first()]),
        (pt("second"), vec![door_second(), cup_second()]),
    ]
}

#[test]
fn the_bench_pools_each_model_over_its_juror_names() {
    assert_eq!(
        bench_lines(&hand_sets()).expect("a reading"),
        vec![
            "model, zeta",
            "question, Which door?",
            "asking, first, none, valid 2, with the plurality 1",
            "asking, second, blue, valid 2, with the plurality 2",
            "question, Which cup?",
            "asking, second, none, valid 0, with the plurality 0",
            "model, alpha",
            "question, Which door?",
            "asking, first, red, valid 3, with the plurality 2",
            "asking, second, red, valid 1, with the plurality 0",
            "question, Which cup?",
            "asking, second, small, valid 2, with the plurality 2",
            "model, ghost",
            "question, Which cup?",
            "asking, second, none, valid 0, with the plurality 0",
            "benches, 3 models, 10 answers, 7 with the plurality",
        ]
    );
}

#[test]
fn the_bench_of_no_sets_is_the_empty_closing() {
    assert_eq!(
        bench_lines(&[]).expect("a reading"),
        vec!["benches, 0 models, 0 answers, 0 with the plurality"]
    );
}

#[test]
fn the_bench_refuses_the_set_rules_it_shares_with_the_census() {
    let duplicate_label = vec![
        (pt("first"), vec![door_first()]),
        (pt("first"), vec![door_second()]),
    ];
    assert_eq!(
        bench_lines(&duplicate_label),
        Err("the set label first stands twice".to_string())
    );
    let duplicate_question = vec![(pt("only"), vec![door_first(), door_first()])];
    assert_eq!(
        bench_lines(&duplicate_question),
        Err("the set only holds the question Which door? twice".to_string())
    );
}

/// The plurality of a list of valid answers, standing only
/// where one answer holds the largest count alone.
fn direct_plurality<'a>(answers: &'a [String], valid: &[bool]) -> Option<&'a str> {
    let mut counts: Vec<(&str, usize)> = Vec::new();
    for (answer, stands) in answers.iter().zip(valid.iter()) {
        if !stands {
            continue;
        }
        match counts.iter_mut().find(|(held, _)| *held == answer.as_str()) {
            Some((_, count)) => *count += 1,
            None => counts.push((answer.as_str(), 1)),
        }
    }
    let largest = counts.iter().map(|(_, count)| *count).max().unwrap_or(0);
    let holders: Vec<&str> = counts
        .iter()
        .filter(|(_, count)| *count == largest)
        .map(|(answer, _)| *answer)
        .collect();
    if largest > 0 && holders.len() == 1 {
        Some(holders[0])
    } else {
        None
    }
}

/// The bench of the recorded sets computed from the
/// responses alone, with no reading of the library standing
/// between the polls and the lines.
fn direct_bench(sets: &[(PText, Vec<Poll>)]) -> Vec<String> {
    let mut models: Vec<String> = Vec::new();
    let mut questions: Vec<String> = Vec::new();
    for (_, polls) in sets {
        for held in polls {
            let question = held.question.display();
            if !questions.contains(&question) {
                questions.push(question);
            }
            for response in &held.responses {
                let model = response.model.display();
                if !models.contains(&model) {
                    models.push(model);
                }
            }
        }
    }
    let mut lines: Vec<String> = Vec::new();
    let mut answer_total = 0usize;
    let mut plurality_total = 0usize;
    for model in &models {
        lines.push(format!("model, {}", model));
        for question in &questions {
            let mut question_lines: Vec<String> = Vec::new();
            for (label, polls) in sets {
                for held in polls {
                    if held.question.display() != *question {
                        continue;
                    }
                    let mine: Vec<&Response> = held
                        .responses
                        .iter()
                        .filter(|response| response.model.display() == *model)
                        .collect();
                    if mine.is_empty() {
                        continue;
                    }
                    let offered: Vec<String> =
                        held.answers.iter().map(|answer| answer.display()).collect();
                    let valid_of = |response: &Response| {
                        !response.abstained && offered.contains(&response.answer.display())
                    };
                    let my_answers: Vec<String> = mine
                        .iter()
                        .map(|response| response.answer.display())
                        .collect();
                    let my_valid: Vec<bool> =
                        mine.iter().map(|response| valid_of(response)).collect();
                    let my_plurality = direct_plurality(&my_answers, &my_valid);
                    let all_answers: Vec<String> = held
                        .responses
                        .iter()
                        .map(|response| response.answer.display())
                        .collect();
                    let all_valid: Vec<bool> = held.responses.iter().map(&valid_of).collect();
                    let pooled = direct_plurality(&all_answers, &all_valid);
                    let valid = my_valid.iter().filter(|stands| **stands).count();
                    let with_plurality = match pooled {
                        Some(answer) => mine
                            .iter()
                            .filter(|response| {
                                valid_of(response) && response.answer.display() == answer
                            })
                            .count(),
                        None => 0,
                    };
                    question_lines.push(format!(
                        "asking, {}, {}, valid {}, with the plurality {}",
                        label.display(),
                        my_plurality.unwrap_or("none"),
                        valid,
                        with_plurality
                    ));
                    answer_total += valid;
                    plurality_total += with_plurality;
                }
            }
            if !question_lines.is_empty() {
                lines.push(format!("question, {}", question));
                lines.extend(question_lines);
            }
        }
    }
    lines.push(format!(
        "benches, {} models, {} answers, {} with the plurality",
        models.len(),
        answer_total,
        plurality_total
    ));
    lines
}

#[test]
fn the_bench_of_the_recorded_sets_stands_line_for_line() {
    let golden = read_text(&examples().join("bench-golden.txt"));
    let held: Vec<String> = golden.lines().map(|line| line.to_string()).collect();
    assert_eq!(bench_lines(&recorded_sets()).expect("a reading"), held);
}

#[test]
fn every_line_of_the_golden_matches_an_independent_computation() {
    let golden = read_text(&examples().join("bench-golden.txt"));
    let held: Vec<String> = golden.lines().map(|line| line.to_string()).collect();
    assert_eq!(direct_bench(&recorded_sets()), held);
}

#[test]
fn the_closing_line_of_the_golden_matches_an_independent_count() {
    let sets = recorded_sets();
    let mut answers = 0usize;
    let mut plurality = 0usize;
    for (_, polls) in &sets {
        for held in polls {
            let offered: Vec<String> = held.answers.iter().map(|answer| answer.display()).collect();
            let valid: Vec<&Response> = held
                .responses
                .iter()
                .filter(|response| {
                    !response.abstained && offered.contains(&response.answer.display())
                })
                .collect();
            answers += valid.len();
            let counts: Vec<usize> = held
                .answers
                .iter()
                .map(|answer| {
                    valid
                        .iter()
                        .filter(|response| response.answer == *answer)
                        .count()
                })
                .collect();
            let largest = counts.iter().copied().max().unwrap_or(0);
            if largest > 0 && counts.iter().filter(|count| **count == largest).count() == 1 {
                plurality += largest;
            }
        }
    }
    let golden = read_text(&examples().join("bench-golden.txt"));
    let models = golden
        .lines()
        .filter(|line| line.starts_with("model, "))
        .count();
    assert_eq!(
        golden.lines().last().expect("a closing line"),
        format!(
            "benches, {} models, {} answers, {} with the plurality",
            models, answers, plurality
        )
    );
    assert!(answers > 0, "the record holds valid answers");
    assert!(plurality > 0, "the record holds pooled pluralities");
}

#[test]
fn the_answers_figure_of_the_golden_matches_the_census_record_line() {
    let census = read_text(&examples().join("census-golden.txt"));
    let record = census
        .lines()
        .find(|line| line.starts_with("record, polls "))
        .expect("a census record line");
    let census_valid: usize = record
        .split(", ")
        .find_map(|part| part.strip_prefix("valid "))
        .expect("a valid figure")
        .parse()
        .expect("a valid count");
    let golden = read_text(&examples().join("bench-golden.txt"));
    let closing = golden.lines().last().expect("a closing line");
    let bench_answers: usize = closing
        .split(", ")
        .find_map(|part| part.strip_suffix(" answers"))
        .expect("an answers figure")
        .parse()
        .expect("an answers count");
    assert_eq!(bench_answers, census_valid);
}

#[test]
fn the_plurality_figure_of_the_golden_matches_the_pooled_plurality_sum() {
    let sets = recorded_sets();
    let mut expected = 0usize;
    for (_, polls) in &sets {
        for held in polls {
            let offered: Vec<String> = held.answers.iter().map(|answer| answer.display()).collect();
            let valid: Vec<&Response> = held
                .responses
                .iter()
                .filter(|response| {
                    !response.abstained && offered.contains(&response.answer.display())
                })
                .collect();
            let counts: Vec<usize> = held
                .answers
                .iter()
                .map(|answer| {
                    valid
                        .iter()
                        .filter(|response| response.answer == *answer)
                        .count()
                })
                .collect();
            let largest = counts.iter().copied().max().unwrap_or(0);
            if largest > 0 && counts.iter().filter(|count| **count == largest).count() == 1 {
                expected += largest;
            }
        }
    }
    let golden = read_text(&examples().join("bench-golden.txt"));
    let closing = golden.lines().last().expect("a closing line");
    let figure: usize = closing
        .rsplit(", ")
        .next()
        .expect("a plurality tail")
        .trim_end_matches(" with the plurality")
        .parse()
        .expect("a plurality figure");
    assert_eq!(figure, expected);
}

#[test]
fn the_bench_golden_carries_no_verdict_vocabulary() {
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
    let golden = read_text(&examples().join("bench-golden.txt"));
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
        "bench",
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
fn the_bench_command_prints_the_recorded_golden() {
    let golden = read_text(&examples().join("bench-golden.txt"));
    let (out, _, code) = run_cli(&[
        "bench",
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
fn the_bench_command_refuses_a_defective_poll() {
    let scratch = TempDir::new("bench-defective");
    let first_dir = scratch.path.join("first");
    let second_dir = scratch.path.join("second");
    write_poll(&first_dir, 1, &door_first());
    std::fs::create_dir_all(&second_dir).expect("a poll directory");
    std::fs::write(second_dir.join("poll-01.json"), "{}\n").expect("a written poll");
    let (out, _, code) = run_cli(&[
        "bench",
        "first",
        &path_arg(&first_dir),
        "second",
        &path_arg(&second_dir),
    ]);
    assert_eq!(code, 1);
    assert!(out.starts_with("invalid poll\ndefect "), "{}", out);
    assert!(!out.contains("benches,"), "{}", out);
}

#[test]
fn the_bench_command_refuses_a_missing_first_directory() {
    let scratch = TempDir::new("bench-missing");
    let missing: PathBuf = scratch.path.join("nowhere");
    let (out, _, code) = run_cli(&["bench", "pilot", &path_arg(&missing)]);
    assert_eq!(code, 1);
    assert_eq!(out, "invalid poll\ndefect file cannot be read\n");
}

#[test]
fn the_bench_command_refuses_a_duplicate_label() {
    let (out, _, code) = run_cli(&[
        "bench",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "pilot",
        &path_arg(&examples().join("wave")),
    ]);
    assert_eq!(code, 1);
    assert_eq!(
        out,
        "invalid bench\ndefect the set label pilot stands twice\n"
    );
}

#[test]
fn the_bench_command_refuses_an_unpaired_label() {
    let (_, err, code) = run_cli(&["bench", "pilot"]);
    assert_eq!(code, 2);
    assert!(
        err.contains("the following arguments are required: directory"),
        "{}",
        err
    );
    let (_, err, code) = run_cli(&["bench"]);
    assert_eq!(code, 2);
    assert!(
        err.contains("the following arguments are required: label, directory"),
        "{}",
        err
    );
}

#[test]
fn the_bench_help_names_its_pairs() {
    let (out, _, code) = run_cli(&["bench", "--help"]);
    assert_eq!(code, 0);
    assert!(
        out.contains("usage: grandjury bench [-h] label directory"),
        "{}",
        out
    );
    let (top, _, top_code) = run_cli(&["--help"]);
    assert_eq!(top_code, 0);
    assert!(
        top.contains("print the bench reading over recorded poll sets"),
        "{}",
        top
    );
}
