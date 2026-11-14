//! Pins for the command line surface.
//!
//! Translated from tests/test_cli.py. Each command prints its
//! report and exits with status zero on a valid input, and
//! prints the defects and exits with status one on a malformed
//! poll or a malformed price table.

mod shared;

use grandjury::json::JsonValue;
use shared::*;

fn poll_data() -> JsonValue {
    jobj(vec![
        ("question", jstr("Which door should the agent pick?")),
        ("answers", jarr(vec![jstr("red"), jstr("blue")])),
        (
            "responses",
            jarr(vec![jobj(vec![
                ("juror", jstr("juror-1")),
                ("model", jstr("tiny-a")),
                ("answer", jstr("red")),
                ("reason", jstr("the shortest path")),
            ])]),
        ),
    ])
}

fn prices_data(price: JsonValue) -> JsonValue {
    jobj(vec![("prices", jobj(vec![("tiny-a", price)]))])
}

#[test]
fn the_tally_command_prints_the_tally() {
    let scratch = TempDir::new("cli-tally");
    let path = scratch.write_json("poll.json", &poll_data());
    let (out, _, code) = run_cli(&["tally", &path_arg(&path)]);
    assert_eq!(code, 0);
    assert!(out.contains("question, Which door should the agent pick?"));
    assert!(out.contains("red, 1, 100.0%"));
    assert!(out.contains("blue, 0, 0.0%"));
}

#[test]
fn the_stadium_command_prints_the_stadium_view() {
    let scratch = TempDir::new("cli-stadium");
    let path = scratch.write_json("poll.json", &poll_data());
    let (out, _, code) = run_cli(&["stadium", &path_arg(&path)]);
    assert_eq!(code, 0);
    assert!(out.contains("wall of reasons"));
    assert!(out.contains("  juror-1, the shortest path"));
}

#[test]
fn the_cost_command_prints_the_cost_report() {
    let scratch = TempDir::new("cli-cost");
    let poll_path = scratch.write_json("poll.json", &poll_data());
    let prices_path = scratch.write_json("prices.json", &prices_data(jstr("0.0002")));
    let (out, _, code) = run_cli(&["cost", &path_arg(&poll_path), &path_arg(&prices_path)]);
    assert_eq!(code, 0);
    assert!(out.contains("priced, 1 responses, 0.000200 dollars"));
}

#[test]
fn a_malformed_poll_exits_one_on_every_command() {
    let scratch = TempDir::new("cli-malformed");
    let mut data = poll_data();
    if let JsonValue::Obj(pairs) = &mut data {
        for (held, slot) in pairs.iter_mut() {
            if held.display() == "question" {
                *slot = jstr("");
            }
        }
    }
    let path = scratch.write_json("poll.json", &data);
    let prices_path = scratch.write_json("prices.json", &prices_data(jstr("0.0002")));
    let (out_tally, _, code) = run_cli(&["tally", &path_arg(&path)]);
    assert_eq!(code, 1);
    let (_, _, code) = run_cli(&["stadium", &path_arg(&path)]);
    assert_eq!(code, 1);
    let (out_cost, _, code) = run_cli(&["cost", &path_arg(&path), &path_arg(&prices_path)]);
    assert_eq!(code, 1);
    let out = format!("{}{}", out_tally, out_cost);
    assert!(out.contains("invalid poll"));
    assert!(out.contains("defect field question is empty"));
}

#[test]
fn a_malformed_price_table_exits_one_on_cost() {
    let scratch = TempDir::new("cli-bad-prices");
    let poll_path = scratch.write_json("poll.json", &poll_data());
    let prices_path = scratch.write_json("prices.json", &prices_data(jstr("cheap")));
    let (out, _, code) = run_cli(&["cost", &path_arg(&poll_path), &path_arg(&prices_path)]);
    assert_eq!(code, 1);
    assert!(out.contains("invalid price table"));
    assert!(out.contains("defect price for tiny-a is not a decimal string"));
}

#[test]
fn a_label_holding_a_newline_forges_no_cost_line() {
    let scratch = TempDir::new("cli-forged");
    let poll_path = scratch.write_json("poll.json", &poll_data());
    let label = "evil\nunpriced model, forged, 7 responses";
    let prices = JsonValue::object(vec![(
        pt("prices"),
        JsonValue::object(vec![(pt(label), jint(7))]),
    )]);
    let prices_path = scratch.write_json("prices.json", &prices);
    let (out, _, code) = run_cli(&["cost", &path_arg(&poll_path), &path_arg(&prices_path)]);
    assert_eq!(code, 1);
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(
        lines,
        vec![
            "invalid price table",
            "defect a model label holds a newline"
        ]
    );
}

#[test]
fn a_missing_poll_file_exits_one() {
    let scratch = TempDir::new("cli-missing");
    let missing = scratch.path.join("missing.json");
    let (out, _, code) = run_cli(&["tally", &path_arg(&missing)]);
    assert_eq!(code, 1);
    assert!(out.contains("defect file cannot be read"));
}

fn cost_with_price(price: &str) -> (String, String, i32) {
    let scratch = TempDir::new("cli-price");
    let poll_path = scratch.write_json("poll.json", &poll_data());
    let prices_path = scratch.write_json("prices.json", &prices_data(jstr(price)));
    run_cli(&["cost", &path_arg(&poll_path), &path_arg(&prices_path)])
}

#[test]
fn the_cost_command_renders_a_1e30_price_in_full() {
    let (out, err, code) = cost_with_price("1e30");
    assert_eq!(code, 0);
    assert_eq!(err, "");
    assert!(out.contains(&format!("1{}.000000 dollars", "0".repeat(30))));
}

#[test]
fn the_cost_command_renders_a_1e22_price_in_full() {
    let (out, err, code) = cost_with_price("1e22");
    assert_eq!(code, 0);
    assert_eq!(err, "");
    assert!(out.contains(&format!("1{}.000000 dollars", "0".repeat(22))));
}

#[test]
fn the_cost_command_renders_a_23_integer_digit_price() {
    let (out, err, code) = cost_with_price("99999999999999999999999.99");
    assert_eq!(code, 0);
    assert_eq!(err, "");
    assert!(out.contains("99999999999999999999999.990000 dollars"));
}

#[test]
fn the_cost_command_renders_a_price_past_two_million_places_in_full() {
    let (out, err, code) = cost_with_price("1e2000001");
    assert_eq!(code, 0);
    assert_eq!(err, "");
    assert!(out.contains(&format!(
        "priced, 1 responses, 1{}.000000 dollars",
        "0".repeat(2000001)
    )));
}

#[test]
fn the_cost_command_fails_with_no_lines_past_the_context_bound() {
    let (out, err, code) = cost_with_price("1e999999999999999999");
    assert_eq!(code, 1);
    assert_eq!(out, "");
    assert_eq!(err, "");
}

fn cost_with_pair(tiny_price: &str) -> (String, String, i32) {
    let scratch = TempDir::new("cli-pair-price");
    let poll = jobj(vec![
        ("question", jstr("q")),
        ("answers", jarr(vec![jstr("red"), jstr("blue")])),
        (
            "responses",
            jarr(vec![
                jobj(vec![
                    ("juror", jstr("juror-1")),
                    ("model", jstr("m1")),
                    ("answer", jstr("red")),
                    ("reason", jstr("a reason")),
                ]),
                jobj(vec![
                    ("juror", jstr("juror-2")),
                    ("model", jstr("m2")),
                    ("answer", jstr("red")),
                    ("reason", jstr("a reason")),
                ]),
            ]),
        ),
    ]);
    let prices = jobj(vec![(
        "prices",
        jobj(vec![("m1", jstr("1")), ("m2", jstr(tiny_price))]),
    )]);
    let poll_path = scratch.write_json("poll.json", &poll);
    let prices_path = scratch.write_json("prices.json", &prices);
    run_cli(&["cost", &path_arg(&poll_path), &path_arg(&prices_path)])
}

#[test]
fn the_cost_command_fails_fast_with_no_lines_at_or_inside_the_bound() {
    // The three spans of the PORT-11 amendment at the command
    // surface, where the observable shape is the whole claim,
    // status 1 with no report lines and no error text, reached
    // at once because the coefficient reservation fails before
    // any sum runs.
    for tiny in [
        "1e-999999999999999995",
        "1e-999999999999999994",
        "1e-999999999999999990",
    ] {
        let (out, err, code) = cost_with_pair(tiny);
        assert_eq!(code, 1, "{}", tiny);
        assert_eq!(out, "", "{}", tiny);
        assert_eq!(err, "", "{}", tiny);
    }
}

#[test]
fn an_explicit_argument_on_the_short_help_flag_is_refused() {
    let (out, err, code) = run_cli(&["-h=x"]);
    assert_eq!(code, 2);
    assert_eq!(out, "");
    assert!(
        err.contains("grandjury: error: argument -h/--help: ignored explicit argument 'x'"),
        "{}",
        err
    );
    let (out, err, code) = run_cli(&["tally", "-h=x"]);
    assert_eq!(code, 2);
    assert_eq!(out, "");
    assert!(
        err.contains("argument -h/--help: ignored explicit argument 'x'"),
        "{}",
        err
    );
}

#[test]
fn an_explicit_argument_on_an_empty_or_partial_long_name_is_refused() {
    for arg in ["--=x", "--h=x", "--he=x"] {
        let (out, err, code) = run_cli(&[arg]);
        assert_eq!(code, 2, "{}", arg);
        assert_eq!(out, "", "{}", arg);
        assert!(
            err.contains("argument -h/--help: ignored explicit argument 'x'"),
            "{} on {}",
            err,
            arg
        );
    }
}

#[test]
fn an_unrecognized_option_after_the_agreement_run_strands_the_rest() {
    let scratch = TempDir::new("cli-broken-run");
    let path = scratch.write_json("poll.json", &poll_data());
    let held = path_arg(&path);
    let (out, err, code) = run_cli(&["agreement", &held, "--bogus", &held, &held]);
    assert_eq!(code, 2);
    assert_eq!(out, "");
    assert!(
        err.contains(&format!(
            "unrecognized arguments: --bogus {} {}",
            held, held
        )),
        "{}",
        err
    );
}

// The canvass command pins of GJ-30. Each fails
// against the Round 13 closure tree by assertion:
// canvass stands as no command in that tree, so every
// run below draws the surface's unknown command
// refusal at status two, where the pins require the
// usage refusals and the reading refusal of the
// fourteenth command.

#[test]
fn the_canvass_command_refuses_no_arguments() {
    let (_, stderr, code) = run_cli(&["canvass"]);
    assert_eq!(code, 2);
    assert!(stderr.contains("the following arguments are required: label, directory"));
}

#[test]
fn the_canvass_command_refuses_an_unpaired_label() {
    let (_, stderr, code) = run_cli(&["canvass", "pilot"]);
    assert_eq!(code, 2);
    assert!(stderr.contains("the following arguments are required: directory"));
}

#[test]
fn the_canvass_command_refuses_a_defective_poll_and_a_duplicate_label() {
    let scratch = TempDir::new("cli-canvass-defective");
    let bad = jobj(vec![
        ("question", jstr("A defective poll?")),
        ("answers", jarr(vec![jstr("only one")])),
        ("responses", jarr(vec![])),
    ]);
    scratch.write_json("poll-01.json", &bad);
    let (stdout, _, code) = run_cli(&[
        "canvass",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "marsh",
        &path_arg(&scratch.path),
    ]);
    assert_eq!(code, 1);
    assert_eq!(
        stdout,
        "invalid poll\ndefect field answers holds fewer than two answers\n"
    );
    assert!(!stdout.contains("canvasses"));
    let (stdout, _, code) = run_cli(&[
        "canvass",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "pilot",
        &path_arg(&examples().join("wave")),
    ]);
    assert_eq!(code, 1);
    assert_eq!(
        stdout,
        "invalid canvass\ndefect the set label pilot stands twice\n"
    );
}

// The roll command pins of GJ-32. Each fails
// against the Round 14 closure tree by assertion:
// roll stands as no command in that tree, so every
// run below draws the surface's unknown command
// refusal at status two, where the pins require the
// usage refusals and the reading refusal of the
// fifteenth command.

#[test]
fn the_roll_command_refuses_no_arguments() {
    let (_, stderr, code) = run_cli(&["roll"]);
    assert_eq!(code, 2);
    assert!(stderr.contains("the following arguments are required: label, directory"));
}

#[test]
fn the_roll_command_refuses_an_unpaired_label() {
    let (_, stderr, code) = run_cli(&["roll", "pilot"]);
    assert_eq!(code, 2);
    assert!(stderr.contains("the following arguments are required: directory"));
}

#[test]
fn the_roll_command_refuses_a_defective_poll_and_a_duplicate_label() {
    let scratch = TempDir::new("cli-roll-defective");
    let bad = jobj(vec![
        ("question", jstr("A defective poll?")),
        ("answers", jarr(vec![jstr("only one")])),
        ("responses", jarr(vec![])),
    ]);
    scratch.write_json("poll-01.json", &bad);
    let (stdout, _, code) = run_cli(&[
        "roll",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "marsh",
        &path_arg(&scratch.path),
    ]);
    assert_eq!(code, 1);
    assert_eq!(
        stdout,
        "invalid poll\ndefect field answers holds fewer than two answers\n"
    );
    assert!(!stdout.contains("rolls"));
    let (stdout, _, code) = run_cli(&[
        "roll",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "pilot",
        &path_arg(&examples().join("wave")),
    ]);
    assert_eq!(code, 1);
    assert_eq!(
        stdout,
        "invalid roll\ndefect the set label pilot stands twice\n"
    );
}

// The muster command pins of GJ-34. Each fails
// against the Round 15 closure tree by assertion:
// muster stands as no command in that tree, so every
// run below draws the surface's unknown command
// refusal at status two, where the pins require the
// usage refusals and the reading refusal of the
// sixteenth command.

#[test]
fn the_muster_command_refuses_no_arguments() {
    let (_, stderr, code) = run_cli(&["muster"]);
    assert_eq!(code, 2);
    assert!(stderr.contains("the following arguments are required: label, directory"));
}

#[test]
fn the_muster_command_refuses_an_unpaired_label() {
    let (_, stderr, code) = run_cli(&["muster", "pilot"]);
    assert_eq!(code, 2);
    assert!(stderr.contains("the following arguments are required: directory"));
}

#[test]
fn the_muster_command_refuses_a_defective_poll_and_a_duplicate_label() {
    let scratch = TempDir::new("cli-muster-defective");
    let bad = jobj(vec![
        ("question", jstr("A defective poll?")),
        ("answers", jarr(vec![jstr("only one")])),
        ("responses", jarr(vec![])),
    ]);
    scratch.write_json("poll-01.json", &bad);
    let (stdout, _, code) = run_cli(&[
        "muster",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "marsh",
        &path_arg(&scratch.path),
    ]);
    assert_eq!(code, 1);
    assert_eq!(
        stdout,
        "invalid poll\ndefect field answers holds fewer than two answers\n"
    );
    assert!(!stdout.contains("musters"));
    let (stdout, _, code) = run_cli(&[
        "muster",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "pilot",
        &path_arg(&examples().join("wave")),
    ]);
    assert_eq!(code, 1);
    assert_eq!(
        stdout,
        "invalid muster\ndefect the set label pilot stands twice\n"
    );
}

// The margin command pins of GJ-36. Each fails
// against the Round 16 closure tree by assertion:
// margin stands as no command in that tree, so every
// run below draws the surface's unknown command
// refusal at status two, where the pins require the
// usage refusals and the reading refusal of the
// seventeenth command.

#[test]
fn the_margin_command_refuses_no_arguments() {
    let (_, stderr, code) = run_cli(&["margin"]);
    assert_eq!(code, 2);
    assert!(stderr.contains("the following arguments are required: label, directory"));
}

#[test]
fn the_margin_command_refuses_an_unpaired_label() {
    let (_, stderr, code) = run_cli(&["margin", "pilot"]);
    assert_eq!(code, 2);
    assert!(stderr.contains("the following arguments are required: directory"));
}

#[test]
fn the_margin_command_refuses_a_defective_poll_and_a_duplicate_label() {
    let scratch = TempDir::new("cli-margin-defective");
    let bad = jobj(vec![
        ("question", jstr("A defective poll?")),
        ("answers", jarr(vec![jstr("only one")])),
        ("responses", jarr(vec![])),
    ]);
    scratch.write_json("poll-01.json", &bad);
    let (stdout, _, code) = run_cli(&[
        "margin",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "marsh",
        &path_arg(&scratch.path),
    ]);
    assert_eq!(code, 1);
    assert_eq!(
        stdout,
        "invalid poll\ndefect field answers holds fewer than two answers\n"
    );
    assert!(!stdout.contains("margins"));
    let (stdout, _, code) = run_cli(&[
        "margin",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "pilot",
        &path_arg(&examples().join("wave")),
    ]);
    assert_eq!(code, 1);
    assert_eq!(
        stdout,
        "invalid margin\ndefect the set label pilot stands twice\n"
    );
}

// The accord command pins of GJ-38. Each fails
// against the Round 17 closure tree by assertion:
// accord stands as no command in that tree, so every
// run below draws the surface's unknown command
// refusal at status two, where the pins require the
// usage refusals and the reading refusal of the
// eighteenth command, and the two decode posture pins
// fail with the command's absence in the same
// invocation, reported as part of this target's
// failure set at the base, not as posture
// discrimination.

#[test]
fn the_accord_command_refuses_no_arguments() {
    let (_, stderr, code) = run_cli(&["accord"]);
    assert_eq!(code, 2);
    assert!(stderr.contains("the following arguments are required: label, directory"));
}

#[test]
fn the_accord_command_refuses_an_unpaired_label() {
    let (_, stderr, code) = run_cli(&["accord", "pilot"]);
    assert_eq!(code, 2);
    assert!(stderr.contains("the following arguments are required: directory"));
}

#[test]
fn the_accord_command_refuses_a_defective_poll_and_a_duplicate_label() {
    let scratch = TempDir::new("cli-accord-defective");
    let bad = jobj(vec![
        ("question", jstr("A defective poll?")),
        ("answers", jarr(vec![jstr("only one")])),
        ("responses", jarr(vec![])),
    ]);
    scratch.write_json("poll-01.json", &bad);
    let (stdout, _, code) = run_cli(&[
        "accord",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "marsh",
        &path_arg(&scratch.path),
    ]);
    assert_eq!(code, 1);
    assert_eq!(
        stdout,
        "invalid poll\ndefect field answers holds fewer than two answers\n"
    );
    assert!(!stdout.contains("accords"));
    let (stdout, _, code) = run_cli(&[
        "accord",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "pilot",
        &path_arg(&examples().join("wave")),
    ]);
    assert_eq!(code, 1);
    assert_eq!(
        stdout,
        "invalid accord\ndefect the set label pilot stands twice\n"
    );
}

// The standing command pins of GJ-40. Each fails
// against the Round 18 closure tree by assertion:
// standing stands as no command in that tree, so every
// run below draws the surface's unknown command
// refusal at status two, where the pins require the
// usage refusals and the reading refusal of the
// nineteenth command. The defective poll and duplicate
// label pin fails under the mutation running the tally
// handler on the standing command's first value, the
// tally handler stating the pilot set alone where the
// pin requires the duplicate label refusal; the two
// usage pins stand under that mutation by design, the
// tally handler drawing the same usage refusals at the
// same statuses.

#[test]
fn the_standing_command_refuses_no_arguments() {
    let (_, stderr, code) = run_cli(&["standing"]);
    assert_eq!(code, 2);
    assert!(stderr.contains("the following arguments are required: label, directory"));
}

#[test]
fn the_standing_command_refuses_an_unpaired_label() {
    let (_, stderr, code) = run_cli(&["standing", "pilot"]);
    assert_eq!(code, 2);
    assert!(stderr.contains("the following arguments are required: directory"));
}

#[test]
fn the_standing_command_refuses_a_defective_poll_and_a_duplicate_label() {
    let scratch = TempDir::new("cli-standing-defective");
    let bad = jobj(vec![
        ("question", jstr("A defective poll?")),
        ("answers", jarr(vec![jstr("only one")])),
        ("responses", jarr(vec![])),
    ]);
    scratch.write_json("poll-01.json", &bad);
    let (stdout, _, code) = run_cli(&[
        "standing",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "marsh",
        &path_arg(&scratch.path),
    ]);
    assert_eq!(code, 1);
    assert_eq!(
        stdout,
        "invalid poll\ndefect field answers holds fewer than two answers\n"
    );
    assert!(!stdout.contains("standings"));
    let (stdout, _, code) = run_cli(&[
        "standing",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "pilot",
        &path_arg(&examples().join("wave")),
    ]);
    assert_eq!(code, 1);
    assert_eq!(
        stdout,
        "invalid standing\ndefect the set label pilot stands twice\n"
    );
}

// The answers command's surface, GJ-42. The three
// answers pins fail against the Round 19 closure
// tree by assertion, the command standing absent
// there and its invalid choice refusal carrying
// neither text the two usage pins assert. The
// defective poll pin fails under OM alone, the
// dispatch restated to run the tally handler on
// its first value, and the two usage pins stand
// under it.

#[test]
fn the_answers_command_refuses_no_arguments() {
    let (_, stderr, code) = run_cli(&["answers"]);
    assert_eq!(code, 2);
    assert!(stderr.contains("the following arguments are required: label, directory"));
}

#[test]
fn the_answers_command_refuses_an_unpaired_label() {
    let (_, stderr, code) = run_cli(&["answers", "pilot"]);
    assert_eq!(code, 2);
    assert!(stderr.contains("the following arguments are required: directory"));
}

#[test]
fn the_answers_command_refuses_a_defective_poll_and_a_duplicate_label() {
    let scratch = TempDir::new("cli-answers-defective");
    let bad = jobj(vec![
        ("question", jstr("A defective poll?")),
        ("answers", jarr(vec![jstr("only one")])),
        ("responses", jarr(vec![])),
    ]);
    scratch.write_json("poll-01.json", &bad);
    let pilot = path_arg(&examples().join("pilot"));
    let wave = path_arg(&examples().join("wave"));
    let marsh = path_arg(&scratch.path);
    let (stdout, _, code) = run_cli(&["answers", "pilot", &pilot, "marsh", &marsh]);
    assert_eq!(code, 1);
    assert_eq!(
        stdout,
        "invalid poll\ndefect field answers holds fewer than two answers\n"
    );
    assert!(!stdout.contains("answers, 143 answers"));
    let (stdout, _, code) = run_cli(&["answers", "pilot", &pilot, "pilot", &wave]);
    assert_eq!(code, 1);
    assert_eq!(
        stdout,
        "invalid answers\ndefect the set label pilot stands twice\n"
    );
}

#[test]
fn the_pairs_command_refuses_no_arguments() {
    let (_, stderr, code) = run_cli(&["pairs"]);
    assert_eq!(code, 2);
    assert!(stderr.contains("the following arguments are required: label, directory"));
}

#[test]
fn the_pairs_command_refuses_an_unpaired_label() {
    let (_, stderr, code) = run_cli(&["pairs", "pilot"]);
    assert_eq!(code, 2);
    assert!(stderr.contains("the following arguments are required: directory"));
}

#[test]
fn the_pairs_command_refuses_a_defective_poll_and_a_duplicate_label() {
    let scratch = TempDir::new("cli-pairs-defective");
    let bad = jobj(vec![
        ("question", jstr("A defective poll?")),
        ("answers", jarr(vec![jstr("only one")])),
        ("responses", jarr(vec![])),
    ]);
    scratch.write_json("poll-01.json", &bad);
    let pilot = path_arg(&examples().join("pilot"));
    let wave = path_arg(&examples().join("wave"));
    let marsh = path_arg(&scratch.path);
    let (stdout, _, code) = run_cli(&["pairs", "pilot", &pilot, "marsh", &marsh]);
    assert_eq!(code, 1);
    assert_eq!(
        stdout,
        "invalid poll\ndefect field answers holds fewer than two answers\n"
    );
    assert!(!stdout.contains("pairs, 3 pairs"));
    let (stdout, _, code) = run_cli(&["pairs", "pilot", &pilot, "pilot", &wave]);
    assert_eq!(code, 1);
    assert_eq!(
        stdout,
        "invalid pairs\ndefect the set label pilot stands twice\n"
    );
}

#[test]
fn a_non_utf8_label_renders_in_replacement_form() {
    // The label posture pin of GJ-38. The real surface
    // over the pilot directory under a label whose
    // bytes are the word bad, the byte 0xFF, and the
    // word label, handed to the surface as the
    // operating system hands it bytes, through the
    // surface's own decode. The run exits at zero, the
    // set lines carry the label in the replacement
    // form the display renders for a surrogate, and
    // the reading stands whole, byte for byte the
    // reading the same label spelled with the
    // replacement character itself prints. The pin
    // fails under the mutation restating the decode to
    // refuse an undecodable byte, under which the run
    // refuses, and under the mutation restating the
    // display form to drop surrogate units, under
    // which the rendered label loses the replacement
    // character.
    use grandjury::cli::{decode_arg, run};
    let directory = path_arg(&examples().join("pilot"));
    let handed = vec![pt("accord"), decode_arg(b"bad\xfflabel"), pt(&directory)];
    let outcome = run(&handed, 80);
    assert_eq!(outcome.code, 0);
    let stdout = String::from_utf8(outcome.stdout).expect("stdout text");
    assert!(stdout.contains("set, bad\u{fffd}label, recorded "));
    let spelled = vec![pt("accord"), pt("bad\u{fffd}label"), pt(&directory)];
    let expected = run(&spelled, 80);
    assert_eq!(expected.code, 0);
    assert_eq!(
        stdout,
        String::from_utf8(expected.stdout).expect("stdout text")
    );
}

#[test]
#[cfg(target_os = "linux")]
fn a_non_utf8_directory_is_read_by_its_own_bytes() {
    // The directory posture pin of GJ-38. A directory
    // whose name holds the byte 0xFF, holding a copy
    // of a recorded poll, is read by its own bytes
    // under the standing decode, the run exiting at
    // zero with the poll's sections standing whole,
    // byte for byte the reading a plainly named
    // directory holding the same poll prints. The pin
    // fails under the mutation building the path from
    // the display form of the argument in place of its
    // own bytes, under which the named directory does
    // not exist and the run refuses.
    use grandjury::cli::{decode_arg, run};
    use std::os::unix::ffi::OsStrExt;
    let scratch = TempDir::new("cli-accord-nonutf8");
    let poll_bytes =
        std::fs::read(examples().join("pilot").join("poll-01.json")).expect("a recorded poll");
    let plain = scratch.path.join("plaindir");
    std::fs::create_dir(&plain).expect("a plain directory");
    std::fs::write(plain.join("poll-01.json"), &poll_bytes).expect("the poll copied");
    let handed_name = std::ffi::OsStr::from_bytes(b"dir\xffname");
    let handed_dir = scratch.path.join(handed_name);
    std::fs::create_dir(&handed_dir).expect("a handed directory");
    std::fs::write(handed_dir.join("poll-01.json"), &poll_bytes).expect("the poll copied");
    let handed = vec![
        pt("accord"),
        pt("pilot"),
        decode_arg(handed_dir.as_os_str().as_bytes()),
    ];
    let outcome = run(&handed, 80);
    assert_eq!(outcome.code, 0);
    let stdout = String::from_utf8(outcome.stdout).expect("stdout text");
    assert!(stdout.contains("question, "));
    assert!(stdout.contains("accords, "));
    let spelled = vec![pt("accord"), pt("pilot"), pt(&path_arg(&plain))];
    let expected = run(&spelled, 80);
    assert_eq!(expected.code, 0);
    assert_eq!(
        stdout,
        String::from_utf8(expected.stdout).expect("stdout text")
    );
}

// The shares command's surface, GJ-48. The three
// shares pins fail against the Round 22 closure
// tree by assertion, the command standing absent
// there and its invalid choice refusal carrying
// neither text the two usage pins assert. The
// defective poll pin fails under the dispatch
// mutation alone, the dispatch restated to run the
// tally handler on its first value, and the two
// usage pins stand under it.

#[test]
fn the_shares_command_refuses_no_arguments() {
    let (_, stderr, code) = run_cli(&["shares"]);
    assert_eq!(code, 2);
    assert!(stderr.contains("the following arguments are required: label, directory"));
}

#[test]
fn the_shares_command_refuses_an_unpaired_label() {
    let (_, stderr, code) = run_cli(&["shares", "pilot"]);
    assert_eq!(code, 2);
    assert!(stderr.contains("the following arguments are required: directory"));
}

#[test]
fn the_shares_command_refuses_a_defective_poll_and_a_duplicate_label() {
    let scratch = TempDir::new("cli-shares-defective");
    let bad = jobj(vec![
        ("question", jstr("A defective poll?")),
        ("answers", jarr(vec![jstr("only one")])),
        ("responses", jarr(vec![])),
    ]);
    scratch.write_json("poll-01.json", &bad);
    let pilot = path_arg(&examples().join("pilot"));
    let wave = path_arg(&examples().join("wave"));
    let marsh = path_arg(&scratch.path);
    let (stdout, _, code) = run_cli(&["shares", "pilot", &pilot, "marsh", &marsh]);
    assert_eq!(code, 1);
    assert_eq!(
        stdout,
        "invalid poll\ndefect field answers holds fewer than two answers\n"
    );
    assert!(!stdout.contains("shares, 3 sets"));
    let (stdout, _, code) = run_cli(&["shares", "pilot", &pilot, "pilot", &wave]);
    assert_eq!(code, 1);
    assert_eq!(
        stdout,
        "invalid shares\ndefect the set label pilot stands twice\n"
    );
}
