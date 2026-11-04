//! Pins for GJ-9 and GJ-10, the second wave poll and the
//! wave comparison reading.
//!
//! The comparison pins build their polls by hand, so each one
//! fails under the negation of the rule it names, held and
//! moved and not counted in each of its stated causes, the
//! pairing refusals, and the jury union order. The recorded
//! pins hold the wave maker to the recorded wave tree byte
//! for byte and the comparison of the recorded pilot and wave
//! polls to the committed golden, through the function and
//! through the command. Every pin runs offline against the
//! recorded envelopes and polls, and no pin makes a live call.

mod shared;

use std::path::{Path, PathBuf};

use grandjury::compare::compare_lines;
use grandjury::makers::wave_files;
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
fn the_comparison_of_one_pair_stands_line_for_line() {
    assert_eq!(
        compare_lines(&[door_first()], &[door_second()]).expect("a reading"),
        vec![
            "question, Which door?",
            "jury, jev-a",
            "first wave, recorded 3, valid 3, abstained 0, spoiled 0",
            "  red, 2, 2/3, 66.7%",
            "  blue, 1, 1/3, 33.3%",
            "  green, 0, 0, 0.0%",
            "plurality, first wave, red",
            "second wave, recorded 3, valid 3, abstained 0, spoiled 0",
            "  red, 2, 2/3, 66.7%",
            "  blue, 1, 1/3, 33.3%",
            "  green, 0, 0, 0.0%",
            "plurality, second wave, red",
            "movement, jev-a, held",
            "jury, chat-b",
            "first wave, recorded 3, valid 3, abstained 0, spoiled 0",
            "  red, 1, 1/3, 33.3%",
            "  blue, 2, 2/3, 66.7%",
            "  green, 0, 0, 0.0%",
            "plurality, first wave, blue",
            "second wave, recorded 3, valid 3, abstained 0, spoiled 0",
            "  red, 2, 2/3, 66.7%",
            "  blue, 1, 1/3, 33.3%",
            "  green, 0, 0, 0.0%",
            "plurality, second wave, red",
            "movement, chat-b, moved",
            "movements, 1 held, 1 moved, 0 not counted",
        ]
    );
}

#[test]
fn a_tie_in_the_first_wave_counts_no_movement() {
    let first = poll(
        "Tied first?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", false),
            response("jev-2", "jev-a", "b", false),
        ],
    );
    let second = poll(
        "Tied first?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", false),
            response("jev-2", "jev-a", "a", false),
        ],
    );
    let lines = compare_lines(&[first], &[second]).expect("a reading");
    assert!(lines.contains(&"plurality, first wave, none".to_string()));
    assert!(lines.contains(&"plurality, second wave, a".to_string()));
    assert!(lines.contains(&"movement, jev-a, not counted".to_string()));
    assert_eq!(
        lines.last().map(String::as_str),
        Some("movements, 0 held, 0 moved, 1 not counted")
    );
}

#[test]
fn a_tie_in_the_second_wave_counts_no_movement() {
    let first = poll(
        "Tied second?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", false),
            response("jev-2", "jev-a", "a", false),
        ],
    );
    let second = poll(
        "Tied second?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", false),
            response("jev-2", "jev-a", "b", false),
        ],
    );
    let lines = compare_lines(&[first], &[second]).expect("a reading");
    assert!(lines.contains(&"plurality, first wave, a".to_string()));
    assert!(lines.contains(&"plurality, second wave, none".to_string()));
    assert!(lines.contains(&"movement, jev-a, not counted".to_string()));
}

#[test]
fn a_zero_valid_first_wave_counts_no_movement() {
    let first = poll(
        "Silent first?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", true),
            response("jev-2", "jev-a", "b", true),
        ],
    );
    let second = poll(
        "Silent first?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", false),
            response("jev-2", "jev-a", "a", false),
        ],
    );
    let lines = compare_lines(&[first], &[second]).expect("a reading");
    assert!(lines.contains(&"first wave, recorded 2, valid 0, abstained 2, spoiled 0".to_string()));
    assert!(lines.contains(&"  a, 0, 0, 0.0%".to_string()));
    assert!(lines.contains(&"plurality, first wave, none".to_string()));
    assert!(lines.contains(&"movement, jev-a, not counted".to_string()));
}

#[test]
fn a_zero_valid_second_wave_counts_no_movement() {
    let first = poll(
        "Silent second?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "b", false),
            response("jev-2", "jev-a", "b", false),
        ],
    );
    let second = poll(
        "Silent second?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", true),
            response("jev-2", "jev-a", "b", true),
        ],
    );
    let lines = compare_lines(&[first], &[second]).expect("a reading");
    assert!(lines.contains(&"second wave, recorded 2, valid 0, abstained 2, spoiled 0".to_string()));
    assert!(lines.contains(&"plurality, second wave, none".to_string()));
    assert!(lines.contains(&"movement, jev-a, not counted".to_string()));
}

#[test]
fn a_jury_absent_from_the_second_wave_counts_no_movement() {
    let first = poll(
        "Absent later?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", false),
            response("jev-2", "jev-a", "a", false),
            response("openai-1", "chat-b", "b", false),
        ],
    );
    let second = poll(
        "Absent later?",
        vec!["a", "b"],
        vec![response("openai-1", "chat-b", "b", false)],
    );
    let lines = compare_lines(&[first], &[second]).expect("a reading");
    let start = lines
        .iter()
        .position(|line| line == "jury, jev-a")
        .expect("the absent jury block");
    assert_eq!(
        lines[start..start + 8].join("\n"),
        [
            "jury, jev-a",
            "first wave, recorded 2, valid 2, abstained 0, spoiled 0",
            "  a, 2, 1, 100.0%",
            "  b, 0, 0, 0.0%",
            "plurality, first wave, a",
            "second wave, recorded 0, valid 0, abstained 0, spoiled 0",
            "  a, 0, 0, 0.0%",
            "  b, 0, 0, 0.0%",
        ]
        .join("\n")
    );
    assert!(lines.contains(&"plurality, second wave, none".to_string()));
    assert!(lines.contains(&"movement, jev-a, not counted".to_string()));
    assert!(lines.contains(&"movement, chat-b, held".to_string()));
}

#[test]
fn a_jury_absent_from_the_first_wave_counts_no_movement() {
    let first = poll(
        "Absent earlier?",
        vec!["a", "b"],
        vec![response("openai-1", "chat-b", "a", false)],
    );
    let second = poll(
        "Absent earlier?",
        vec!["a", "b", "c"],
        vec![
            response("openai-1", "chat-b", "a", false),
            response("jev-1", "jev-a", "c", false),
            response("jev-2", "jev-a", "c", false),
        ],
    );
    let lines = compare_lines(&[first], &[second]).expect("a reading");
    let jury_lines: Vec<&str> = lines
        .iter()
        .filter(|line| line.starts_with("jury, "))
        .map(|line| line.as_str())
        .collect();
    assert_eq!(jury_lines, vec!["jury, chat-b", "jury, jev-a"]);
    let start = lines
        .iter()
        .position(|line| line == "jury, jev-a")
        .expect("the late jury block");
    assert_eq!(
        lines[start..start + 10].join("\n"),
        [
            "jury, jev-a",
            "first wave, recorded 0, valid 0, abstained 0, spoiled 0",
            "  a, 0, 0, 0.0%",
            "  b, 0, 0, 0.0%",
            "plurality, first wave, none",
            "second wave, recorded 2, valid 2, abstained 0, spoiled 0",
            "  a, 0, 0, 0.0%",
            "  b, 0, 0, 0.0%",
            "  c, 2, 1, 100.0%",
            "plurality, second wave, c",
        ]
        .join("\n")
    );
    assert!(lines.contains(&"movement, jev-a, not counted".to_string()));
}

#[test]
fn spoiled_responses_stand_outside_the_comparison_shares() {
    let first = poll(
        "Spoiled?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", false),
            response("jev-2", "jev-a", "a", false),
            response("jev-3", "jev-a", "off the list", false),
        ],
    );
    let second = poll(
        "Spoiled?",
        vec!["a", "b"],
        vec![
            response("jev-1", "jev-a", "a", false),
            response("jev-2", "jev-a", "b", false),
            response("jev-3", "jev-a", "b", false),
        ],
    );
    let lines = compare_lines(&[first], &[second]).expect("a reading");
    assert!(lines.contains(&"first wave, recorded 3, valid 2, abstained 0, spoiled 1".to_string()));
    assert!(lines.contains(&"  a, 2, 1, 100.0%".to_string()));
    assert!(lines.contains(&"movement, jev-a, moved".to_string()));
}

#[test]
fn different_counts_refuse_the_reading() {
    let one = door_first();
    assert_eq!(
        compare_lines(&[door_first()], &[one.clone(), one]),
        Err("the two poll sets hold different counts".to_string())
    );
}

#[test]
fn a_question_mismatch_refuses_with_its_pair_number() {
    let first = vec![door_first(), door_first()];
    let mut second = vec![door_second(), door_second()];
    second[1].question = pt("Another question?");
    assert_eq!(
        compare_lines(&first, &second),
        Err("pair 2 holds two different questions".to_string())
    );
}

#[test]
fn the_closing_line_counts_across_pairs() {
    let held_pair_first = poll(
        "Held?",
        vec!["a", "b"],
        vec![response("jev-1", "jev-a", "a", false)],
    );
    let held_pair_second = poll(
        "Held?",
        vec!["a", "b"],
        vec![response("jev-1", "jev-a", "a", false)],
    );
    let moved_pair_first = poll(
        "Moved?",
        vec!["a", "b"],
        vec![response("jev-1", "jev-a", "a", false)],
    );
    let moved_pair_second = poll(
        "Moved?",
        vec!["a", "b"],
        vec![response("jev-1", "jev-a", "b", false)],
    );
    let silent_pair_first = poll(
        "Silent?",
        vec!["a", "b"],
        vec![response("jev-1", "jev-a", "a", true)],
    );
    let silent_pair_second = poll(
        "Silent?",
        vec!["a", "b"],
        vec![response("jev-1", "jev-a", "a", false)],
    );
    let lines = compare_lines(
        &[held_pair_first, moved_pair_first, silent_pair_first],
        &[held_pair_second, moved_pair_second, silent_pair_second],
    )
    .expect("a reading");
    assert_eq!(
        lines.last().map(String::as_str),
        Some("movements, 1 held, 1 moved, 1 not counted")
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
fn the_compare_command_prints_the_recorded_golden() {
    let (out, err, code) = run_cli(&[
        "compare",
        &path_arg(&examples().join("pilot")),
        &path_arg(&examples().join("wave")),
    ]);
    assert_eq!(code, 0, "{}", err);
    let golden = read_text(&examples().join("wave-comparison-golden.txt"));
    assert_eq!(out, golden);
}

#[test]
fn the_recorded_comparison_stands_in_the_golden() {
    let first = load_set(&examples().join("pilot"), 12);
    let second = load_set(&examples().join("wave"), 12);
    let lines = compare_lines(&first, &second).expect("a reading");
    let golden = read_text(&examples().join("wave-comparison-golden.txt"));
    assert_eq!(lines.join("\n"), golden.trim_end_matches('\n'));
}

#[test]
fn the_wave_polls_carry_the_pilot_questions_in_order() {
    let questions = read_json(&examples().join("pilot/questions.json"));
    let items = match &questions {
        grandjury::json::JsonValue::Arr(items) => items.clone(),
        other => panic!("an array of questions, got {:?}", other),
    };
    assert_eq!(items.len(), 12);
    let wave = load_set(&examples().join("wave"), 12);
    for (item, held) in items.iter().zip(wave.iter()) {
        assert_eq!(as_str(field(item, "question")), held.question.display());
    }
}

#[test]
fn the_wave_files_match_the_recorded_tree_byte_for_byte() {
    let files = wave_files(&examples()).expect("the wave files");
    let mut names: Vec<&str> = files.iter().map(|(name, _)| name.as_str()).collect();
    names.sort();
    let mut expected: Vec<String> = (1..=12)
        .map(|index| format!("wave/poll-{:02}.json", index))
        .collect();
    expected.push("wave/run_record.json".to_string());
    let mut expected_names: Vec<&str> = expected.iter().map(|name| name.as_str()).collect();
    expected_names.sort();
    assert_eq!(names, expected_names);
    for (relative, content) in &files {
        let committed = read_text(&examples().join(relative));
        assert_eq!(content, &committed, "{}", relative);
    }
}

#[test]
fn the_wave_run_record_counts_sixty_attempts_per_model() {
    let record = read_json(&examples().join("wave/run_record.json"));
    assert_eq!(as_int(field(&record, "questions")), 12);
    assert_eq!(as_int(field(&record, "seats_per_question_per_model")), 5);
    let models = field(&record, "models");
    let entries = match models {
        grandjury::json::JsonValue::Obj(entries) => entries,
        other => panic!("a models object, got {:?}", other),
    };
    assert_eq!(entries.len(), 2);
    for (_, entry) in entries {
        assert_eq!(as_int(field(entry, "calls_attempted")), 60);
        assert_eq!(
            as_int(field(entry, "calls_recorded")) + as_int(field(entry, "failed_calls")),
            60
        );
    }
}

#[test]
fn the_comparison_golden_carries_no_verdict_vocabulary() {
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
    let golden = read_text(&examples().join("wave-comparison-golden.txt"));
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
fn the_compare_command_refuses_a_mismatched_pair() {
    let scratch = TempDir::new("wave-mismatch");
    let first_dir = scratch.path.join("first");
    let second_dir = scratch.path.join("second");
    write_poll(&first_dir, 1, &door_first());
    write_poll(&first_dir, 2, &door_first());
    write_poll(&second_dir, 1, &door_second());
    let mut other = door_second();
    other.question = pt("Another question?");
    write_poll(&second_dir, 2, &other);
    let (out, _, code) = run_cli(&["compare", &path_arg(&first_dir), &path_arg(&second_dir)]);
    assert_eq!(code, 1);
    assert_eq!(
        out,
        "invalid comparison\ndefect pair 2 holds two different questions\n"
    );
}

#[test]
fn the_compare_command_refuses_different_counts() {
    let scratch = TempDir::new("wave-counts");
    let first_dir = scratch.path.join("first");
    let second_dir = scratch.path.join("second");
    write_poll(&first_dir, 1, &door_first());
    write_poll(&first_dir, 2, &door_first());
    write_poll(&second_dir, 1, &door_second());
    let (out, _, code) = run_cli(&["compare", &path_arg(&first_dir), &path_arg(&second_dir)]);
    assert_eq!(code, 1);
    assert_eq!(
        out,
        "invalid comparison\ndefect the two poll sets hold different counts\n"
    );
}

#[test]
fn the_compare_command_refuses_a_defective_poll() {
    let scratch = TempDir::new("wave-defective");
    let first_dir = scratch.path.join("first");
    let second_dir = scratch.path.join("second");
    write_poll(&first_dir, 1, &door_first());
    std::fs::create_dir_all(&second_dir).expect("a poll directory");
    std::fs::write(second_dir.join("poll-01.json"), "{}\n").expect("a written poll");
    let (out, _, code) = run_cli(&["compare", &path_arg(&first_dir), &path_arg(&second_dir)]);
    assert_eq!(code, 1);
    assert!(out.starts_with("invalid poll\ndefect "), "{}", out);
}

#[test]
fn the_compare_command_refuses_a_missing_first_directory() {
    let scratch = TempDir::new("wave-missing");
    let missing: PathBuf = scratch.path.join("nowhere");
    let (out, _, code) = run_cli(&[
        "compare",
        &path_arg(&missing),
        &path_arg(&examples().join("wave")),
    ]);
    assert_eq!(code, 1);
    assert_eq!(out, "invalid poll\ndefect file cannot be read\n");
}

#[test]
fn the_compare_command_stops_the_set_at_a_gap() {
    let scratch = TempDir::new("wave-gap");
    let first_dir = scratch.path.join("first");
    let second_dir = scratch.path.join("second");
    write_poll(&first_dir, 1, &door_first());
    write_poll(&first_dir, 3, &door_first());
    write_poll(&second_dir, 1, &door_second());
    let (out, _, code) = run_cli(&["compare", &path_arg(&first_dir), &path_arg(&second_dir)]);
    assert_eq!(code, 0);
    assert!(out.ends_with("movements, 1 held, 1 moved, 0 not counted\n"));
}

#[test]
fn the_compare_help_names_its_two_directories() {
    let (out, _, code) = run_cli(&["compare", "--help"]);
    assert_eq!(code, 0);
    assert!(
        out.contains("usage: grandjury compare [-h] first_dir second_dir"),
        "{}",
        out
    );
    let (top, _, top_code) = run_cli(&["--help"]);
    assert_eq!(top_code, 0);
    assert!(
        top.contains("print the wave comparison reading over two poll sets"),
        "{}",
        top
    );
}

#[test]
fn the_wave_run_record_incident_section_equals_its_input_file() {
    // The repair of FIX-GJ-11 changes the incident file, and
    // the run record the maker rebuilds carries the incident
    // section of that file unchanged, so this pin stands
    // beside the counting pins of the repair.
    let record = read_json(&examples().join("wave/run_record.json"));
    let incident = read_json(&examples().join("wave/incident.json"));
    assert_json_eq(field(&record, "incident"), &incident);
}

#[test]
fn the_wave_incident_counts_the_aborted_first_launch() {
    // The counting FIX-GJ-11 states, a named mechanical
    // consequence of the repair. The recorded run made sixty
    // poll calls and one route verification call per model,
    // and the aborted first launch made one route
    // verification call per model ahead of its poll calls, so
    // the attempts in the round stand at sixty two per model
    // and the arithmetic of the record is the sum of every
    // counted field including the aborted launch count. The
    // pin read sixty one before the repair because the
    // incident numbers described the recorded run alone.
    let incident = read_json(&examples().join("wave/incident.json"));
    for model in ["jev-1.13.0", "openai/gpt-6-luna"] {
        let entry = field(&incident, model);
        assert_eq!(as_int(field(entry, "calls_in_the_recorded_run")), 60);
        assert_eq!(
            as_int(field(
                entry,
                "verification_calls_in_the_aborted_first_launch"
            )),
            1
        );
        let parts = as_int(field(entry, "calls_in_the_recorded_run"))
            + as_int(field(entry, "attempts_that_never_left_the_machine"))
            + as_int(field(entry, "envelopes_lost_and_seats_rerun"))
            + as_int(field(entry, "calls_discarded"))
            + as_int(field(entry, "route_verification_calls"))
            + as_int(field(entry, "diagnostic_calls"))
            + as_int(field(
                entry,
                "verification_calls_in_the_aborted_first_launch",
            ));
        assert_eq!(parts, as_int(field(entry, "attempts_in_round")));
        assert_eq!(as_int(field(entry, "attempts_in_round")), 62);
    }
}

#[test]
fn the_wave_incident_states_the_aborted_launch_accounting() {
    // The aborted launch accounting FIX-GJ-11 names, stated
    // per route in the measure the record holds. The chat
    // route states its spend as the decimal string the key
    // usage delta measured and its call count as the estimate
    // the record states, and the Jev route states its token
    // usage unrecoverable and estimated nowhere.
    let incident = read_json(&examples().join("wave/incident.json"));
    let aborted = field(&incident, "aborted_first_launch");
    let chat = field(aborted, "openai/gpt-6-luna");
    assert_eq!(as_int(field(chat, "verification_calls")), 1);
    assert_eq!(as_str(field(chat, "spend_dollars")), "0.0013174");
    assert!(as_str(field(chat, "poll_calls")).contains("about twenty five"));
    assert!(as_str(field(chat, "spend_measurement"))
        .contains("measured exactly by the key usage delta"));
    let jev = field(aborted, "jev-1.13.0");
    assert_eq!(as_int(field(jev, "verification_calls")), 1);
    assert!(as_str(field(jev, "spend")).contains("unrecoverable and estimated nowhere"));
}
