//! Pins for GJ-11 and GJ-12, the census reading and the
//! census command.
//!
//! The function pins build their polls by hand, so each one
//! fails under the negation of the rule it names, held and
//! moved and not counted in each of their stated causes over
//! one, two, and three askings, the label and pairing
//! refusals, and the record totals. The recorded pins hold
//! the census of the three recorded sets to the committed
//! golden through the function and through the command, hold
//! its totals against the three run records, and hold its
//! twelve twice asked questions against the wave comparison
//! reading over the same polls. Every pin runs offline
//! against the recorded polls, and no pin makes a live call.

mod shared;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use grandjury::census::census_lines;
use grandjury::compare::compare_lines;
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
fn the_census_of_two_askings_stands_line_for_line() {
    let sets = vec![
        (pt("first"), vec![door_first()]),
        (pt("second"), vec![door_second()]),
    ];
    // The counts lines of this pin stand in the repaired
    // form of FIX-GJ-12, the set label fenced behind the
    // word set. The alteration is the named mechanical
    // consequence of that repair, and no other line of the
    // pin moves.
    assert_eq!(
        census_lines(&sets).expect("a reading"),
        vec![
            "question, Which door?",
            "jury, jev-a",
            "set, first, recorded 3, valid 3, abstained 0, spoiled 0",
            "  red, 2, 2/3, 66.7%",
            "  blue, 1, 1/3, 33.3%",
            "  green, 0, 0, 0.0%",
            "plurality, first, red",
            "set, second, recorded 3, valid 3, abstained 0, spoiled 0",
            "  red, 2, 2/3, 66.7%",
            "  blue, 1, 1/3, 33.3%",
            "  green, 0, 0, 0.0%",
            "plurality, second, red",
            "movement, jev-a, held",
            "jury, chat-b",
            "set, first, recorded 3, valid 3, abstained 0, spoiled 0",
            "  red, 1, 1/3, 33.3%",
            "  blue, 2, 2/3, 66.7%",
            "  green, 0, 0, 0.0%",
            "plurality, first, blue",
            "set, second, recorded 3, valid 3, abstained 0, spoiled 0",
            "  red, 2, 2/3, 66.7%",
            "  blue, 1, 1/3, 33.3%",
            "  green, 0, 0, 0.0%",
            "plurality, second, red",
            "movement, chat-b, moved",
            "record, polls 2, recorded 12, valid 12, abstained 0, spoiled 0",
            "jury, jev-a, recorded 6, valid 6, abstained 0, spoiled 0",
            "jury, chat-b, recorded 6, valid 6, abstained 0, spoiled 0",
            "movements, 1 held, 1 moved, 0 not counted",
        ]
    );
}

#[test]
fn a_question_asked_once_counts_no_movement() {
    let sets = vec![(pt("only"), vec![door_first()])];
    let lines = census_lines(&sets).expect("a reading");
    assert!(lines.contains(&"movement, jev-a, not counted".to_string()));
    assert!(lines.contains(&"movement, chat-b, not counted".to_string()));
    assert_eq!(
        lines.last().map(String::as_str),
        Some("movements, 0 held, 0 moved, 2 not counted")
    );
}

fn three_askings(middle: Poll, last: Poll) -> Vec<String> {
    let first = poll(
        "Three times?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", false),
            response("jev-2", "jev-a", "a", false),
        ],
    );
    let sets = vec![
        (pt("one"), vec![first]),
        (pt("two"), vec![middle]),
        (pt("three"), vec![last]),
    ];
    census_lines(&sets).expect("a reading")
}

#[test]
fn a_tie_between_two_standing_pluralities_still_reads_held() {
    let tied = poll(
        "Three times?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", false),
            response("jev-2", "jev-a", "b", false),
        ],
    );
    let again = poll(
        "Three times?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", false),
            response("jev-2", "jev-a", "a", false),
        ],
    );
    let lines = three_askings(tied, again);
    assert!(lines.contains(&"plurality, two, none".to_string()));
    assert!(lines.contains(&"movement, jev-a, held".to_string()));
    assert_eq!(
        lines.last().map(String::as_str),
        Some("movements, 1 held, 0 moved, 0 not counted")
    );
}

#[test]
fn a_plurality_changing_at_the_third_asking_reads_moved() {
    let same = poll(
        "Three times?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", false),
            response("jev-2", "jev-a", "a", false),
        ],
    );
    let changed = poll(
        "Three times?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "b", false),
            response("jev-2", "jev-a", "b", false),
        ],
    );
    let lines = three_askings(same, changed);
    assert!(lines.contains(&"movement, jev-a, moved".to_string()));
}

#[test]
fn a_plurality_standing_at_one_of_three_askings_counts_no_movement() {
    let tied = poll(
        "Three times?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", false),
            response("jev-2", "jev-a", "b", false),
        ],
    );
    let silent = poll(
        "Three times?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", true),
            response("jev-2", "jev-a", "b", true),
        ],
    );
    let lines = three_askings(tied, silent);
    // The counts line stands in the repaired form of
    // FIX-GJ-12, the named mechanical consequence of that
    // repair.
    assert!(lines.contains(&"set, three, recorded 2, valid 0, abstained 2, spoiled 0".to_string()));
    assert!(lines.contains(&"plurality, three, none".to_string()));
    assert!(lines.contains(&"movement, jev-a, not counted".to_string()));
}

#[test]
fn a_jury_absent_from_the_middle_asking_reads_from_the_rest() {
    let first = poll(
        "Absent middle?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", false),
            response("jev-2", "jev-a", "a", false),
            response("openai-1", "chat-b", "b", false),
        ],
    );
    let middle = poll(
        "Absent middle?",
        vec!["a", "b"],
        vec![response("openai-1", "chat-b", "b", false)],
    );
    let last = poll(
        "Absent middle?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", false),
            response("openai-1", "chat-b", "b", false),
        ],
    );
    let sets = vec![
        (pt("one"), vec![first]),
        (pt("two"), vec![middle]),
        (pt("three"), vec![last]),
    ];
    let lines = census_lines(&sets).expect("a reading");
    let start = lines
        .iter()
        .position(|line| line == "jury, jev-a")
        .expect("the absent jury block");
    // The counts lines stand in the repaired form of
    // FIX-GJ-12, the named mechanical consequence of that
    // repair.
    assert_eq!(
        lines[start..start + 6].join("\n"),
        [
            "jury, jev-a",
            "set, one, recorded 2, valid 2, abstained 0, spoiled 0",
            "  a, 2, 1, 100.0%",
            "  b, 0, 0, 0.0%",
            "plurality, one, a",
            "set, two, recorded 0, valid 0, abstained 0, spoiled 0",
        ]
        .join("\n")
    );
    assert!(lines.contains(&"movement, jev-a, held".to_string()));
    assert!(lines.contains(&"movement, chat-b, held".to_string()));
}

#[test]
fn a_question_standing_in_one_set_only_shows_that_set() {
    let other = poll(
        "Another question?",
        vec!["x", "y"],
        vec![response("jev-1", "jev-a", "x", false)],
    );
    let sets = vec![
        (pt("first"), vec![door_first()]),
        (pt("second"), vec![other]),
    ];
    let lines = census_lines(&sets).expect("a reading");
    let door_at = lines
        .iter()
        .position(|line| line == "question, Which door?")
        .expect("the door section");
    let other_at = lines
        .iter()
        .position(|line| line == "question, Another question?")
        .expect("the other section");
    assert!(door_at < other_at);
    // The counts line prefixes stand in the repaired form
    // of FIX-GJ-12, the named mechanical consequence of
    // that repair.
    let door_section = &lines[door_at..other_at];
    assert!(door_section
        .iter()
        .any(|line| line.starts_with("set, first, recorded")));
    assert!(!door_section
        .iter()
        .any(|line| line.starts_with("set, second, recorded")));
    let other_section = &lines[other_at..lines.len() - 4];
    assert!(other_section
        .iter()
        .any(|line| line.starts_with("set, second, recorded")));
    assert!(!other_section
        .iter()
        .any(|line| line.starts_with("set, first, recorded")));
}

#[test]
fn the_jury_union_order_follows_the_askings() {
    let first = poll(
        "Union?",
        vec!["a", "b"],
        vec![response("openai-1", "chat-b", "a", false)],
    );
    let second = poll(
        "Union?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "b", false),
            response("openai-1", "chat-b", "a", false),
        ],
    );
    let sets = vec![(pt("one"), vec![first]), (pt("two"), vec![second])];
    let lines = census_lines(&sets).expect("a reading");
    let jury_lines: Vec<&str> = lines
        .iter()
        .filter(|line| line.starts_with("jury, ") && !line.contains("recorded"))
        .map(|line| line.as_str())
        .collect();
    assert_eq!(jury_lines, vec!["jury, chat-b", "jury, jev-a"]);
}

#[test]
fn the_record_totals_count_each_response_kind_apart() {
    let mixed = poll(
        "Mixed?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", false),
            response("jev-2", "jev-a", "off the list", false),
            response("jev-3", "jev-a", "b", true),
            response("openai-1", "chat-b", "b", false),
        ],
    );
    let sets = vec![(pt("only"), vec![mixed])];
    let lines = census_lines(&sets).expect("a reading");
    // The counts line stands in the repaired form of
    // FIX-GJ-12, the named mechanical consequence of that
    // repair.
    assert!(lines.contains(&"set, only, recorded 3, valid 1, abstained 1, spoiled 1".to_string()));
    assert!(
        lines.contains(&"record, polls 1, recorded 4, valid 2, abstained 1, spoiled 1".to_string())
    );
    assert!(lines.contains(&"jury, jev-a, recorded 3, valid 1, abstained 1, spoiled 1".to_string()));
    assert!(
        lines.contains(&"jury, chat-b, recorded 1, valid 1, abstained 0, spoiled 0".to_string())
    );
}

#[test]
fn a_question_twice_in_one_set_refuses_the_census() {
    let sets = vec![(pt("first"), vec![door_first(), door_first()])];
    assert_eq!(
        census_lines(&sets),
        Err("the set first holds the question Which door? twice".to_string())
    );
}

#[test]
fn a_duplicate_set_label_refuses_the_census() {
    let sets = vec![
        (pt("first"), vec![door_first()]),
        (pt("first"), vec![door_second()]),
    ];
    assert_eq!(
        census_lines(&sets),
        Err("the set label first stands twice".to_string())
    );
}

#[test]
fn an_empty_set_label_refuses_the_census() {
    let sets = vec![(pt(""), vec![door_first()])];
    assert_eq!(census_lines(&sets), Err("a set label is empty".to_string()));
}

#[test]
fn a_set_label_holding_a_newline_refuses_the_census() {
    let sets = vec![(pt("two\nlines"), vec![door_first()])];
    assert_eq!(
        census_lines(&sets),
        Err("a set label holds a newline".to_string())
    );
    let sets = vec![(pt("two\rlines"), vec![door_first()])];
    assert_eq!(
        census_lines(&sets),
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
        census_lines(&sets),
        Err("the set label first stands twice".to_string())
    );
}

#[test]
fn a_hostile_set_label_mints_no_structural_line() {
    // The pin of FIX-GJ-12. The set label is the one field
    // the census itself supplies, and GJ-11 as first stated
    // carried it at the start of the counts line, so a
    // label naming a structural line printed that line's
    // form with the label's own text standing in the place
    // of its figures. Under the repair the counts line
    // opens with the word set and the label stands behind
    // it. Each label below names a structural line of the
    // reading in whole or in its opening, and none of them
    // mints a second line of that form. The reading over
    // the door poll holds one question line, one record
    // line, and one closing line, four jury lines over its
    // two blocks and its two totals lines, and two counts
    // lines in the repaired form.
    let labels = [
        "record, polls 999",
        "question, Planted",
        "movements, 9 held",
        "jury, jev-a, recorded 9, valid 9, abstained 9, spoiled 9",
        "record, polls 1, recorded 6, valid 6, abstained 0, spoiled 0",
        "set, pilot",
    ];
    for label in labels {
        let sets = vec![(pt(label), vec![door_first()])];
        let lines = census_lines(&sets).expect("a reading");
        assert_eq!(
            lines
                .iter()
                .filter(|line| line.starts_with("question, "))
                .count(),
            1,
            "{}",
            label
        );
        assert!(lines.contains(&"question, Which door?".to_string()));
        assert_eq!(
            lines
                .iter()
                .filter(|line| line.starts_with("record, polls "))
                .count(),
            1,
            "{}",
            label
        );
        assert!(lines
            .contains(&"record, polls 1, recorded 6, valid 6, abstained 0, spoiled 0".to_string()));
        assert_eq!(
            lines
                .iter()
                .filter(|line| line.starts_with("movements, "))
                .count(),
            1,
            "{}",
            label
        );
        assert!(lines.contains(&"movements, 0 held, 0 moved, 2 not counted".to_string()));
        assert_eq!(
            lines
                .iter()
                .filter(|line| line.starts_with("jury, "))
                .count(),
            4,
            "{}",
            label
        );
        let counts: Vec<&String> = lines
            .iter()
            .filter(|line| line.starts_with("set, "))
            .collect();
        assert_eq!(counts.len(), 2, "{}", label);
        for line in counts {
            assert_eq!(
                line.as_str(),
                format!(
                    "set, {}, recorded 3, valid 3, abstained 0, spoiled 0",
                    label
                )
                .as_str(),
                "{}",
                label
            );
        }
    }
}

#[test]
fn the_census_of_no_sets_stands_at_zero() {
    assert_eq!(
        census_lines(&[]).expect("a reading"),
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
fn the_census_command_prints_the_recorded_golden() {
    let (out, err, code) = run_cli(&[
        "census",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "full",
        &path_arg(&examples().join("full")),
        "wave",
        &path_arg(&examples().join("wave")),
    ]);
    assert_eq!(code, 0, "{}", err);
    let golden = read_text(&examples().join("census-golden.txt"));
    assert_eq!(out, golden);
}

#[test]
fn the_recorded_census_stands_in_the_golden() {
    let lines = census_lines(&recorded_sets()).expect("a reading");
    let golden = read_text(&examples().join("census-golden.txt"));
    assert_eq!(lines.join("\n"), golden.trim_end_matches('\n'));
}

#[test]
fn the_census_reads_the_sets_in_the_order_given() {
    let sets = vec![
        (pt("wave"), load_set(&examples().join("wave"), 12)),
        (pt("pilot"), load_set(&examples().join("pilot"), 12)),
    ];
    let lines = census_lines(&sets).expect("a reading");
    let questions = read_json(&examples().join("pilot/questions.json"));
    let first_question = match &questions {
        grandjury::json::JsonValue::Arr(items) => as_str(field(&items[0], "question")),
        other => panic!("an array of questions, got {:?}", other),
    };
    assert_eq!(lines[0], format!("question, {}", first_question));
    // The counts line stands in the repaired form of
    // FIX-GJ-12, the named mechanical consequence of that
    // repair.
    assert!(lines[2].starts_with("set, wave, recorded"), "{}", lines[2]);
}

#[test]
fn the_census_totals_agree_with_the_run_records() {
    // The record line and the jury totals of the golden,
    // against the sums of the three run records for the
    // figures those records state, and against a direct
    // classification of the recorded responses for the valid
    // and spoiled figures no run record states.
    let golden = read_text(&examples().join("census-golden.txt"));
    let lines: Vec<&str> = golden.lines().collect();
    let record_line = lines
        .iter()
        .find(|line| line.starts_with("record, polls "))
        .expect("the record line")
        .to_string();
    let mut figures: BTreeMap<String, i64> = BTreeMap::new();
    let mut per_model: BTreeMap<String, BTreeMap<String, i64>> = BTreeMap::new();
    for label in ["pilot", "full", "wave"] {
        let record = read_json(&examples().join(label).join("run_record.json"));
        *figures.entry("polls".to_string()).or_default() += as_int(field(&record, "questions"));
        let models = field(&record, "models");
        let entries = match models {
            grandjury::json::JsonValue::Obj(entries) => entries,
            other => panic!("a models object, got {:?}", other),
        };
        for (model, entry) in entries {
            let held = per_model.entry(model.display()).or_default();
            for (source, target) in [("calls_recorded", "recorded"), ("abstained", "abstained")] {
                let value = as_int(field(entry, source));
                *figures.entry(target.to_string()).or_default() += value;
                *held.entry(target.to_string()).or_default() += value;
            }
        }
    }
    let mut direct: BTreeMap<String, i64> = BTreeMap::new();
    let mut direct_per_model: BTreeMap<String, BTreeMap<String, i64>> = BTreeMap::new();
    for (_, polls) in recorded_sets() {
        for held in &polls {
            for item in &held.responses {
                let kind = if item.abstained {
                    "abstained"
                } else if held.answers.contains(&item.answer) {
                    "valid"
                } else {
                    "spoiled"
                };
                *direct.entry(kind.to_string()).or_default() += 1;
                *direct_per_model
                    .entry(item.model.display())
                    .or_default()
                    .entry(kind.to_string())
                    .or_default() += 1;
            }
        }
    }
    figures.insert(
        "valid".to_string(),
        direct.get("valid").copied().unwrap_or(0),
    );
    figures.insert(
        "spoiled".to_string(),
        direct.get("spoiled").copied().unwrap_or(0),
    );
    let expected_record = format!(
        "record, polls {}, recorded {}, valid {}, abstained {}, spoiled {}",
        figures["polls"],
        figures["recorded"],
        figures["valid"],
        figures["abstained"],
        figures["spoiled"]
    );
    assert_eq!(record_line, expected_record);
    for (model, held) in &per_model {
        let classified = &direct_per_model[model];
        let line = format!(
            "jury, {}, recorded {}, valid {}, abstained {}, spoiled {}",
            model,
            held["recorded"],
            classified.get("valid").copied().unwrap_or(0),
            held["abstained"],
            classified.get("spoiled").copied().unwrap_or(0)
        );
        assert!(lines.iter().any(|held_line| *held_line == line), "{}", line);
    }
}

/// The pluralities and movements of one reading, parsed in
/// line order into a map from question and jury to the two
/// plurality answers and the movement word, with the two
/// asking words of the reading under the given names. A
/// block holding only one of the two askings contributes no
/// entry, as the full poll blocks of the census do not.
fn movements_of(
    lines: &[String],
    first_word: &str,
    second_word: &str,
) -> BTreeMap<(String, String), (String, String, String)> {
    let first_prefix = format!("plurality, {}, ", first_word);
    let second_prefix = format!("plurality, {}, ", second_word);
    let mut found: BTreeMap<(String, String), (String, String, String)> = BTreeMap::new();
    let mut question = String::new();
    let mut jury = String::new();
    let mut first: Option<String> = None;
    let mut second: Option<String> = None;
    for line in lines {
        if let Some(rest) = line.strip_prefix("question, ") {
            question = rest.to_string();
            first = None;
            second = None;
        } else if let Some(rest) = line.strip_prefix("jury, ") {
            if !rest.contains(',') {
                jury = rest.to_string();
                first = None;
                second = None;
            }
        } else if let Some(rest) = line.strip_prefix(first_prefix.as_str()) {
            first = Some(rest.to_string());
        } else if let Some(rest) = line.strip_prefix(second_prefix.as_str()) {
            second = Some(rest.to_string());
        } else if let Some(rest) = line.strip_prefix("movement, ") {
            if let (Some(held_first), Some(held_second)) = (&first, &second) {
                let (named, movement) = rest.split_once(", ").expect("a movement line");
                assert_eq!(named, jury);
                found.insert(
                    (question.clone(), jury.clone()),
                    (
                        held_first.clone(),
                        held_second.clone(),
                        movement.to_string(),
                    ),
                );
            }
            first = None;
            second = None;
        }
    }
    found
}

#[test]
fn the_twice_asked_questions_agree_with_the_wave_comparison() {
    // The twelve pilot questions stand in the census under
    // the labels pilot and wave, and their plurality and
    // movement lines agree with the GJ-10 reading of the
    // same polls, where the labels stand in place of its
    // wave words. The closing line counts those 24 pairs at
    // the 16 held, 0 moved, 8 not counted of the comparison,
    // and the forty full questions count every one of their
    // 77 jury blocks not counted, three full polls holding
    // one jury only, so the census closes at 85 not counted.
    let pilot = load_set(&examples().join("pilot"), 12);
    let wave = load_set(&examples().join("wave"), 12);
    let comparison = compare_lines(&pilot, &wave).expect("a reading");
    let expected = movements_of(&comparison, "first wave", "second wave");
    assert_eq!(expected.len(), 24);
    let census = census_lines(&recorded_sets()).expect("a reading");
    let found = movements_of(&census, "pilot", "wave");
    for (key, value) in &expected {
        assert_eq!(found.get(key), Some(value), "{:?}", key);
    }
    assert_eq!(
        census.last().map(String::as_str),
        Some("movements, 16 held, 0 moved, 85 not counted")
    );
}

#[test]
fn the_census_golden_carries_no_verdict_vocabulary() {
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
    let golden = read_text(&examples().join("census-golden.txt"));
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

#[test]
fn the_census_command_refuses_a_defective_poll() {
    let scratch = TempDir::new("census-defective");
    let first_dir = scratch.path.join("first");
    let second_dir = scratch.path.join("second");
    write_poll(&first_dir, 1, &door_first());
    std::fs::create_dir_all(&second_dir).expect("a poll directory");
    std::fs::write(second_dir.join("poll-01.json"), "{}\n").expect("a written poll");
    let (out, _, code) = run_cli(&[
        "census",
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
fn the_census_command_refuses_a_missing_first_directory() {
    let scratch = TempDir::new("census-missing");
    let missing: PathBuf = scratch.path.join("nowhere");
    let (out, _, code) = run_cli(&["census", "pilot", &path_arg(&missing)]);
    assert_eq!(code, 1);
    assert_eq!(out, "invalid poll\ndefect file cannot be read\n");
}

#[test]
fn the_census_command_refuses_a_duplicate_label() {
    let (out, _, code) = run_cli(&[
        "census",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "pilot",
        &path_arg(&examples().join("wave")),
    ]);
    assert_eq!(code, 1);
    assert_eq!(
        out,
        "invalid census\ndefect the set label pilot stands twice\n"
    );
}

#[test]
fn the_census_command_refuses_a_label_holding_a_newline() {
    let (out, _, code) = run_cli(&["census", "two\nlines", &path_arg(&examples().join("pilot"))]);
    assert_eq!(code, 1);
    assert_eq!(out, "invalid census\ndefect a set label holds a newline\n");
}

#[test]
fn the_census_command_refuses_an_unpaired_label() {
    let (_, err, code) = run_cli(&["census", "pilot"]);
    assert_eq!(code, 2);
    assert!(
        err.contains("the following arguments are required: directory"),
        "{}",
        err
    );
    let (_, err, code) = run_cli(&["census", "pilot", "somewhere", "wave"]);
    assert_eq!(code, 2);
    assert!(
        err.contains("the following arguments are required: directory"),
        "{}",
        err
    );
    let (_, err, code) = run_cli(&["census"]);
    assert_eq!(code, 2);
    assert!(
        err.contains("the following arguments are required: label, directory"),
        "{}",
        err
    );
}

#[test]
fn the_census_command_stops_a_set_at_a_gap() {
    let scratch = TempDir::new("census-gap");
    let dir = scratch.path.join("set");
    write_poll(&dir, 1, &door_first());
    write_poll(&dir, 3, &door_first());
    let (out, _, code) = run_cli(&["census", "only", &path_arg(&dir)]);
    assert_eq!(code, 0);
    assert!(out.contains("record, polls 1, recorded 6"), "{}", out);
}

#[test]
fn the_census_help_names_its_pairs() {
    let (out, _, code) = run_cli(&["census", "--help"]);
    assert_eq!(code, 0);
    assert!(
        out.contains("usage: grandjury census [-h] label directory"),
        "{}",
        out
    );
    let (top, _, top_code) = run_cli(&["--help"]);
    assert_eq!(top_code, 0);
    assert!(
        top.contains("print the census reading over recorded poll sets"),
        "{}",
        top
    );
}
