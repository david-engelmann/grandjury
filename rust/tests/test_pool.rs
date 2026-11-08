//! Pins for GJ-25 and GJ-26, the pool reading and the
//! pool command.
//!
//! The function pins build their polls by hand, so each
//! one fails under the negation of the rule it names,
//! the pooling of every response of a poll as one pool
//! through the jury reading of GJ-8, the question order
//! of first appearance across the sets, the pooled
//! plurality of GJ-17, and the set refusals the pool
//! shares with the census. The recorded pins hold the
//! pool of the three recorded sets to the committed
//! golden through the function and through the command,
//! hold every block of the golden against an independent
//! computation of the pooled counts over the recorded
//! polls, and hold the closing line against the census,
//! swing, and minority goldens under the relations GJ-26
//! states. The constructed example pin holds the tied
//! asking the shipped sets do not hold. Every pin runs
//! offline against the recorded polls, and no pin makes
//! a live call.

mod shared;

use std::path::{Path, PathBuf};

use grandjury::polls::{load_poll_file, Poll, Response};
use grandjury::pool::pool_lines;
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
fn the_pool_states_the_pooled_counts_of_each_asking() {
    assert_eq!(
        pool_lines(&hand_sets()).expect("a reading"),
        vec![
            "question, Which door?",
            "set, first, recorded 5, valid 5, abstained 0, spoiled 0",
            "  red, 3, 3/5, 60.0%",
            "  blue, 2, 2/5, 40.0%",
            "  green, 0, 0, 0.0%",
            "plurality, first, red",
            "set, second, recorded 3, valid 3, abstained 0, spoiled 0",
            "  red, 1, 1/3, 33.3%",
            "  blue, 2, 2/3, 66.7%",
            "  green, 0, 0, 0.0%",
            "plurality, second, blue",
            "question, Which cup?",
            "set, second, recorded 4, valid 2, abstained 1, spoiled 1",
            "  big, 0, 0, 0.0%",
            "  small, 2, 1, 100.0%",
            "plurality, second, small",
            "pools, 2 questions, 3 askings, 10 valid, 3 pluralities",
        ]
    );
}

#[test]
fn the_pool_of_no_sets_is_the_empty_closing() {
    assert_eq!(
        pool_lines(&[]).expect("a reading"),
        vec!["pools, 0 questions, 0 askings, 0 valid, 0 pluralities"]
    );
}

#[test]
fn the_pool_of_an_asking_with_no_valid_response_states_zero_shares() {
    let silent = poll(
        "Which bell?",
        vec!["iron", "brass"],
        vec![
            response("seat-1", "zeta", "iron", true),
            response("seat-2", "alpha", "brass", true),
        ],
    );
    let sets = vec![(pt("only"), vec![silent])];
    assert_eq!(
        pool_lines(&sets).expect("a reading"),
        vec![
            "question, Which bell?",
            "set, only, recorded 2, valid 0, abstained 2, spoiled 0",
            "  iron, 0, 0, 0.0%",
            "  brass, 0, 0, 0.0%",
            "plurality, only, none",
            "pools, 1 questions, 1 askings, 0 valid, 0 pluralities",
        ]
    );
}

#[test]
fn the_pool_refuses_the_set_rules_it_shares_with_the_census() {
    let duplicate_label = vec![
        (pt("first"), vec![door_first()]),
        (pt("first"), vec![door_second()]),
    ];
    assert_eq!(
        pool_lines(&duplicate_label),
        Err("the set label first stands twice".to_string())
    );
    let duplicate_question = vec![(pt("only"), vec![door_first(), door_first()])];
    assert_eq!(
        pool_lines(&duplicate_question),
        Err("the set only holds the question Which door? twice".to_string())
    );
}

/// The greatest common divisor of two counts, for the
/// direct fraction rendering below.
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// The share of a count among a valid base in the GJ-8
/// forms, the exact fraction in lowest terms and the
/// percentage with one decimal place, halves rounding up
/// on the exact value, computed from the counts alone.
fn direct_share(count: usize, valid: usize) -> (String, String) {
    if valid == 0 {
        return ("0".to_string(), "0.0".to_string());
    }
    let divisor = gcd(count, valid);
    let (numerator, denominator) = (count / divisor, valid / divisor);
    let fraction = if denominator == 1 {
        format!("{}", numerator)
    } else {
        format!("{}/{}", numerator, denominator)
    };
    let tenths = (2000 * count + valid) / (2 * valid);
    (fraction, format!("{}.{}", tenths / 10, tenths % 10))
}

/// The pool of the recorded sets computed from the
/// responses alone, with no reading of the library
/// standing between the polls and the lines.
fn direct_pool(sets: &[(PText, Vec<Poll>)]) -> Vec<String> {
    let mut questions: Vec<String> = Vec::new();
    for (_, polls) in sets {
        for held in polls {
            let question = held.question.display();
            if !questions.contains(&question) {
                questions.push(question);
            }
        }
    }
    let mut lines: Vec<String> = Vec::new();
    let mut asking_total = 0usize;
    let mut valid_total = 0usize;
    let mut plurality_total = 0usize;
    for question in &questions {
        lines.push(format!("question, {}", question));
        for (label, polls) in sets {
            for held in polls {
                if held.question.display() != *question {
                    continue;
                }
                let offered: Vec<String> =
                    held.answers.iter().map(|answer| answer.display()).collect();
                let mut abstained = 0usize;
                let mut spoiled = 0usize;
                let mut counts: Vec<usize> = vec![0; held.answers.len()];
                for response in &held.responses {
                    if response.abstained {
                        abstained += 1;
                    } else if let Some(slot) = offered
                        .iter()
                        .position(|answer| *answer == response.answer.display())
                    {
                        counts[slot] += 1;
                    } else {
                        spoiled += 1;
                    }
                }
                let valid: usize = counts.iter().sum();
                let largest = counts.iter().copied().max().unwrap_or(0);
                let plurality =
                    if valid > 0 && counts.iter().filter(|count| **count == largest).count() == 1 {
                        let slot = counts
                            .iter()
                            .position(|count| *count == largest)
                            .expect("a largest count");
                        Some(offered[slot].clone())
                    } else {
                        None
                    };
                asking_total += 1;
                valid_total += valid;
                if plurality.is_some() {
                    plurality_total += 1;
                }
                lines.push(format!(
                    "set, {}, recorded {}, valid {}, abstained {}, spoiled {}",
                    label.display(),
                    held.responses.len(),
                    valid,
                    abstained,
                    spoiled
                ));
                for (answer, count) in held.answers.iter().zip(counts.iter()) {
                    let (fraction, percent) = direct_share(*count, valid);
                    lines.push(format!(
                        "  {}, {}, {}, {}%",
                        answer.display(),
                        count,
                        fraction,
                        percent
                    ));
                }
                lines.push(format!(
                    "plurality, {}, {}",
                    label.display(),
                    plurality.as_deref().unwrap_or("none")
                ));
            }
        }
    }
    lines.push(format!(
        "pools, {} questions, {} askings, {} valid, {} pluralities",
        questions.len(),
        asking_total,
        valid_total,
        plurality_total
    ));
    lines
}

#[test]
fn the_pool_of_the_recorded_sets_stands_line_for_line() {
    let golden = read_text(&examples().join("pool-golden.txt"));
    let held: Vec<String> = golden.lines().map(|line| line.to_string()).collect();
    assert_eq!(pool_lines(&recorded_sets()).expect("a reading"), held);
}

#[test]
fn every_block_of_the_golden_matches_a_direct_computation() {
    let golden = read_text(&examples().join("pool-golden.txt"));
    let held: Vec<String> = golden.lines().map(|line| line.to_string()).collect();
    assert_eq!(direct_pool(&recorded_sets()), held);
}

/// Read one figure out of a closing or record line, the
/// count standing beside its word in either order, as
/// the forms of the readings state them.
fn figure_before(line: &str, word: &str) -> usize {
    for part in line.split(", ") {
        let tokens: Vec<&str> = part.split(' ').collect();
        if tokens.len() == 2 {
            if tokens[1] == word {
                return tokens[0].parse().expect("a figure");
            }
            if tokens[0] == word {
                return tokens[1].parse().expect("a figure");
            }
        }
    }
    panic!("no {} figure in {}", word, line);
}

#[test]
fn the_closing_line_of_the_golden_matches_the_standing_goldens() {
    let golden = read_text(&examples().join("pool-golden.txt"));
    let closing = golden.lines().last().expect("a closing line").to_string();
    // The valid figure equals the valid figure of the
    // record line of the census golden.
    let census = read_text(&examples().join("census-golden.txt"));
    let record = census
        .lines()
        .find(|line| line.starts_with("record, "))
        .expect("a record line");
    assert_eq!(
        figure_before(&closing, "valid"),
        figure_before(record, "valid")
    );
    // The askings figure equals the askings figure of
    // the swing closing and the polls figure of the
    // census record line.
    let swing = read_text(&examples().join("swing-golden.txt"));
    let swing_closing = swing.lines().last().expect("a closing line");
    assert_eq!(
        figure_before(&closing, "askings"),
        figure_before(swing_closing, "askings")
    );
    assert_eq!(
        figure_before(&closing, "askings"),
        figure_before(record, "polls")
    );
    // The pluralities figure equals the pluralities
    // figure of the minority closing and the count of
    // the asking lines of the swing whose plurality is
    // not none.
    let minority = read_text(&examples().join("minority-golden.txt"));
    let minority_closing = minority.lines().last().expect("a closing line");
    assert_eq!(
        figure_before(&closing, "pluralities"),
        figure_before(minority_closing, "pluralities")
    );
    let swing_pluralities = swing
        .lines()
        .filter(|line| line.starts_with("asking, ") && !line.contains(", none,"))
        .count();
    assert_eq!(figure_before(&closing, "pluralities"), swing_pluralities);
    // Every plurality line of the pool equals the
    // plurality of the swing asking line for the same
    // asking, read in asking order across both goldens.
    let pool_pluralities: Vec<&str> = golden
        .lines()
        .filter(|line| line.starts_with("plurality, "))
        .collect();
    let swing_pluralities_in_order: Vec<String> = swing
        .lines()
        .filter(|line| line.starts_with("asking, "))
        .map(|line| {
            let head = line
                .strip_prefix("asking, ")
                .and_then(|rest| rest.rsplit_once(", margin"))
                .map(|(head, _)| head)
                .expect("an asking line");
            let (label, answer) = head.split_once(", ").expect("a label and an answer");
            format!("plurality, {}, {}", label, answer)
        })
        .collect();
    assert_eq!(pool_pluralities.len(), swing_pluralities_in_order.len());
    for (pool_line, swing_line) in pool_pluralities
        .iter()
        .zip(swing_pluralities_in_order.iter())
    {
        assert_eq!(pool_line, swing_line);
    }
}

#[test]
fn the_constructed_example_carries_the_tied_asking() {
    let held = load_poll(&examples().join("constructed-pool.json"));
    let sets = vec![(pt("constructed"), vec![held])];
    assert_eq!(
        pool_lines(&sets).expect("a reading"),
        vec![
            "question, Which gate should the night market use?",
            "set, constructed, recorded 5, valid 4, abstained 1, spoiled 0",
            "  the north gate, 2, 1/2, 50.0%",
            "  the south gate, 2, 1/2, 50.0%",
            "plurality, constructed, none",
            "pools, 1 questions, 1 askings, 4 valid, 0 pluralities",
        ]
    );
}

#[test]
fn the_pool_golden_carries_no_verdict_vocabulary() {
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
    let golden = read_text(&examples().join("pool-golden.txt"));
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
        "pool",
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
fn the_pool_command_prints_the_recorded_golden() {
    let golden = read_text(&examples().join("pool-golden.txt"));
    let (out, _, code) = run_cli(&[
        "pool",
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
fn the_pool_command_refuses_a_defective_poll() {
    let scratch = TempDir::new("pool-defective");
    let first_dir = scratch.path.join("first");
    let second_dir = scratch.path.join("second");
    write_poll(&first_dir, 1, &door_first());
    std::fs::create_dir_all(&second_dir).expect("a poll directory");
    std::fs::write(second_dir.join("poll-01.json"), "{}\n").expect("a written poll");
    let (out, _, code) = run_cli(&[
        "pool",
        "first",
        &path_arg(&first_dir),
        "second",
        &path_arg(&second_dir),
    ]);
    assert_eq!(code, 1);
    assert!(out.starts_with("invalid poll\ndefect "), "{}", out);
    assert!(!out.contains("pools,"), "{}", out);
}

#[test]
fn the_pool_command_refuses_a_missing_first_directory() {
    let scratch = TempDir::new("pool-missing");
    let missing: PathBuf = scratch.path.join("nowhere");
    let (out, _, code) = run_cli(&["pool", "pilot", &path_arg(&missing)]);
    assert_eq!(code, 1);
    assert_eq!(out, "invalid poll\ndefect file cannot be read\n");
}

#[test]
fn the_pool_command_refuses_a_duplicate_label() {
    let (out, _, code) = run_cli(&[
        "pool",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "pilot",
        &path_arg(&examples().join("wave")),
    ]);
    assert_eq!(code, 1);
    assert_eq!(
        out,
        "invalid pool\ndefect the set label pilot stands twice\n"
    );
}

#[test]
fn the_pool_command_refuses_an_unpaired_label() {
    let (_, err, code) = run_cli(&["pool", "pilot"]);
    assert_eq!(code, 2);
    assert!(
        err.contains("the following arguments are required: directory"),
        "{}",
        err
    );
    let (_, err, code) = run_cli(&["pool"]);
    assert_eq!(code, 2);
    assert!(
        err.contains("the following arguments are required: label, directory"),
        "{}",
        err
    );
}

#[test]
fn the_pool_help_names_its_pairs() {
    let (out, _, code) = run_cli(&["pool", "--help"]);
    assert_eq!(code, 0);
    assert!(
        out.contains("usage: grandjury pool [-h] label directory"),
        "{}",
        out
    );
    let (top, _, top_code) = run_cli(&["--help"]);
    assert_eq!(top_code, 0);
    assert!(
        top.contains("print the pool reading over recorded poll sets"),
        "{}",
        top
    );
}
