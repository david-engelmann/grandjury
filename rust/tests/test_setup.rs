//! Setup pins, translated from tests/test_setup.py with the
//! port pins of PORT-9 added.
//!
//! Each translated pin can fail for a real reason against a
//! repo that lost a setup file or let its status drift from
//! its contract. The port pins hold the README port line to
//! exactly one standing, the CI cargo steps and the scan
//! through the codepoint_scan binary, and the contract
//! port items in place.

mod shared;

use shared::*;

fn root_text(relative: &str) -> String {
    read_text(&repo_root().join(relative))
}

fn lines_of(text: &str) -> Vec<&str> {
    text.lines().collect()
}

#[test]
fn agents_carries_the_canonical_marker() {
    let text = root_text("AGENTS.md");
    assert!(text.contains("<!-- repo annex below this line -->"));
    assert!(text.contains("39 repo storefront perfection program"));
}

#[test]
fn readme_states_the_closed_round_count() {
    let text = root_text("README.md");
    assert!(text.contains("Closed perfection rounds, 14 of 100"));
}

#[test]
fn contract_carries_the_setup_obligations() {
    let text = root_text("CONTRACT.md");
    for oid in ["SETUP-1", "SETUP-2", "SETUP-3", "SETUP-4", "GJ-1"] {
        assert!(text.contains(oid), "{}", oid);
    }
}

#[test]
fn readme_reports_the_contract_status() {
    let text = root_text("README.md");
    let expected = "- Contract, SETUP-1 to SETUP-4 and GJ-1 to GJ-4 met. GJ-1 to GJ-4 stand in full form in CONTRACT.md with the Round 1 repair amendments, and Round 1 closed on the merged slice 2 after three reviews, a repair pass, and an approving re-review.";
    assert!(lines_of(&text).contains(&expected));
}

#[test]
fn readme_reports_the_round_two_status() {
    let text = root_text("README.md");
    let lines = lines_of(&text);
    let standing = "- Contract, GJ-5 and GJ-6 met. They stand in full form in CONTRACT.md with the Round 2 repair amendments, and Round 2 closed on the merged slice 3 after three reviews, two repair passes, and two re-reviews, the second approving with no findings.";
    assert!(lines.contains(&standing));
    let slice_line = "- Slice 3 has landed. It carries the juror seat readers over the two decision model routes, the abstained response kind through the tally, the stadium, and the cost counter, and the recorded pilot over both models, twelve synthetic questions with five seats per question per model and the raw envelopes and the run record under examples.";
    assert!(lines.contains(&slice_line));
}

#[test]
fn readme_reports_the_round_three_status() {
    let text = root_text("README.md");
    let lines = lines_of(&text);
    let standing = "- Contract, GJ-7 and GJ-8 met. They stand in full form in CONTRACT.md with the Round 3 repair amendments, and Round 3 closed on the merged slice 4 after three reviews, a repair pass, and an approving re-review.";
    assert!(lines.contains(&standing));
    let slice_line = "- Slice 4 has landed. It carries the full live poll over both decision model routes, forty synthetic questions with five seats per question per model and the raw envelopes and the run record under examples, and the agreement reading over the recorded responses with its command and its golden.";
    assert!(lines.contains(&slice_line));
    // Altered at the Round 4 slice for its named mechanical
    // consequence, restored at the Round 4 closure, and
    // altered again at the Round 5 slice for the same
    // consequence, and restored at the Round 5 closure, and
    // altered again at the Round 6 slice for the same
    // consequence, and restored at the Round 6 closure, and
    // altered again at the Round 7 slice for the same
    // consequence, and restored at the Round 7 closure, and
    // altered again at the Round 8 slice for the same
    // consequence, and restored at the Round 8 closure, and
    // altered again at the Round 9 slice for the same
    // consequence, and restored at the Round 9 closure, and
    // altered again at the Round 10 slice for the same
    // consequence, and restored at the Round 10 closure, and
    // altered again at the Round 11 slice for the same
    // consequence, and restored at the Round 11 closure, and
    // altered again at the Round 12 slice for the same
    // consequence, and restored at the Round 12 closure, and
    // altered again at the Round 13 slice for the same
    // consequence, and restored at the Round 13 closure, and
    // altered again at the Round 14 slice for the same
    // consequence, and restored at the Round 14 closure.
    // This pin counts the slices standing in
    // review across the whole README. No slice stands in
    // review while Round 15 is unbuilt, so the count stands
    // at zero. The round pins below hold the standing lines
    // and the slice lines themselves.
    assert_eq!(text.matches("built and in review").count(), 0);
}

#[test]
fn readme_reports_the_round_four_status() {
    let text = root_text("README.md");
    let lines = lines_of(&text);
    let standing = "- Contract, GJ-9 and GJ-10 met. They stand in full form in CONTRACT.md with the Round 4 repair amendment, and Round 4 closed on the merged slice 5 after three reviews, a repair pass, and an approving re-review.";
    assert!(lines.contains(&standing));
    let slice_line = "- Slice 5 has landed. It carries the second wave poll over both decision model routes, the twelve pilot questions asked again with five seats per question per model and the raw envelopes and the run record under examples, and the wave comparison reading over the pilot and wave polls with its command and its golden.";
    assert!(lines.contains(&slice_line));
}

#[test]
fn contract_carries_the_round_four_obligations() {
    let text = root_text("CONTRACT.md");
    for oid in ["GJ-9", "GJ-10"] {
        assert!(text.contains(oid), "{}", oid);
    }
}

#[test]
fn readme_reports_the_round_five_status() {
    let text = root_text("README.md");
    let lines = lines_of(&text);
    let standing = "- Contract, GJ-11 and GJ-12 met. They stand in full form in CONTRACT.md with the Round 5 repair amendment, and Round 5 closed on the merged slice 6 after three reviews, a repair pass, and an approving re-review.";
    assert!(lines.contains(&standing));
    let slice_line = "- Slice 6 has landed. It carries the census reading over the three recorded poll sets, the pilot and the full poll and the second wave read together question by question and jury by jury under their set labels, with its command and its golden.";
    assert!(lines.contains(&slice_line));
}

#[test]
fn contract_carries_the_round_five_obligations() {
    let text = root_text("CONTRACT.md");
    for oid in ["GJ-11", "GJ-12"] {
        assert!(text.contains(oid), "{}", oid);
    }
}

#[test]
fn readme_reports_the_round_six_status() {
    let text = root_text("README.md");
    let lines = lines_of(&text);
    let standing = "- Contract, GJ-13 and GJ-14 met. They stand in full form in CONTRACT.md with the Round 6 repair amendment, and Round 6 closed on the merged slice 7 after three reviews, a repair pass, and an approving re-review.";
    assert!(lines.contains(&standing));
    let slice_line = "- Slice 7 has landed. It carries the juror ledger reading over the three recorded poll sets, the pilot and the full poll and the second wave read together respondent by respondent under their set labels, with its command and its golden.";
    assert!(lines.contains(&slice_line));
    assert!(text.contains("Closed perfection rounds, 14 of 100"));
}

#[test]
fn contract_carries_the_round_six_obligations() {
    let text = root_text("CONTRACT.md");
    for oid in ["GJ-13", "GJ-14"] {
        assert!(text.contains(oid), "{}", oid);
    }
}

#[test]
fn readme_reports_the_round_seven_status() {
    let text = root_text("README.md");
    let lines = lines_of(&text);
    let standing = "- Contract, GJ-15 and GJ-16 met. They stand in full form in CONTRACT.md, and Round 7 closed on the merged slice 8 after three reviews, a repair pass, and an approving re-review.";
    assert!(lines.contains(&standing));
    let slice_line = "- Slice 8 has landed. It carries the dissent reading over the three recorded poll sets, the pilot and the full poll and the second wave read together asking by asking under their set labels, stating each jury plurality, the margin it stood by, and the respondents who stood against it, with its command and its golden.";
    assert!(lines.contains(&slice_line));
    assert!(text.contains("Closed perfection rounds, 14 of 100"));
}

#[test]
fn contract_carries_the_round_seven_obligations() {
    let text = root_text("CONTRACT.md");
    for oid in ["GJ-15", "GJ-16"] {
        assert!(text.contains(oid), "{}", oid);
    }
}

#[test]
fn readme_reports_the_round_eight_status() {
    let text = root_text("README.md");
    let lines = lines_of(&text);
    let standing = "- Contract, GJ-17 and GJ-18 met. They stand in full form in CONTRACT.md, and Round 8 closed on the merged slice 9 after three reviews, all approving, with no repair pass.";
    assert!(lines.contains(&standing));
    let slice_line = "- Slice 9 has landed. It carries the swing reading over the three recorded poll sets, the pilot and the full poll and the second wave read together question by question under their set labels, stating the plurality of each asking pooled over the whole poll, the margin it stood by, and the course of each question across its askings, with its command and its golden.";
    assert!(lines.contains(&slice_line));
    assert!(text.contains("Closed perfection rounds, 14 of 100"));
}

#[test]
fn contract_carries_the_round_eight_obligations() {
    let text = root_text("CONTRACT.md");
    for oid in ["GJ-17", "GJ-18"] {
        assert!(text.contains(oid), "{}", oid);
    }
}

#[test]
fn readme_reports_the_round_nine_status() {
    let text = root_text("README.md");
    let lines = lines_of(&text);
    let standing = "- Contract, GJ-19 and GJ-20 met. They stand in full form in CONTRACT.md, and Round 9 closed on the merged slice 10 after three reviews, a repair pass, and a discharging re-review.";
    assert!(lines.contains(&standing));
    let slice_line = "- Slice 10 has landed. It carries the minority reading over the three recorded poll sets, the pilot and the full poll and the second wave read together question by question under their set labels, stating the plurality of each asking pooled over the whole poll, the margin it stood by, and the respondents whose valid answer was not the pooled plurality answer, with its command and its golden.";
    assert!(lines.contains(&slice_line));
    assert!(text.contains("Closed perfection rounds, 14 of 100"));
}

#[test]
fn contract_carries_the_round_nine_obligations() {
    let text = root_text("CONTRACT.md");
    for oid in ["GJ-19", "GJ-20"] {
        assert!(text.contains(oid), "{}", oid);
    }
}

#[test]
fn readme_carries_the_one_line_description_in_both_carriers() {
    let description = "Grandjury is a Rust library and command line tool that aggregates recorded answers to a question and prints readings over them.";
    let text = root_text("README.md");
    let lines = lines_of(&text);
    assert_eq!(lines.first(), Some(&"# grandjury"));
    assert_eq!(lines.get(2), Some(&description));
    let manifest = root_text("Cargo.toml");
    let field = format!("description = \"{}\"", description);
    assert!(lines_of(&manifest).contains(&field.as_str()));
}

#[test]
fn readme_sections_stand_in_the_standard_order() {
    let text = root_text("README.md");
    let headings: Vec<&str> = lines_of(&text)
        .into_iter()
        .filter(|line| line.starts_with("## "))
        .collect();
    assert_eq!(
        headings,
        vec![
            "## Install",
            "## Quickstart",
            "## Usage",
            "## Status",
            "## Programme status",
            "## Licence",
        ]
    );
}

#[test]
fn readme_first_screen_carries_the_badge_and_the_tally_reading() {
    let text = root_text("README.md");
    assert!(text.contains("[![ci](https://github.com/david-engelmann/grandjury/actions/workflows/ci.yml/badge.svg?branch=main)]"));
    let block = "question, Should the village library open on Sunday mornings?\nresponses, 100\nvalid, 50\nspoiled, 0\nabstained, 50\nopen on Sundays, 50, 100.0%\nstay closed on Sundays, 0, 0.0%";
    assert!(text.contains(block));
}

#[test]
fn readme_reports_the_round_ten_status() {
    let text = root_text("README.md");
    let lines = lines_of(&text);
    let standing = "- Contract, GJ-21 and GJ-22 met. They stand in full form in CONTRACT.md with the FIX-GJ-14 repair amendment, and Round 10 closed on the merged slice 11 after three reviews approving, the spec review's two conditions discharged on executed checks.";
    assert!(lines.contains(&standing));
    let slice_line = "- Slice 11 has landed. It carries the README rebuilt to the fleet storefront standard with the one line description standing in the lede and in the manifest and the tally reading over the recorded poll standing on the first screen, and the library fencing closure of FIX-GJ-14 over the agreement, census, dissent, ledger, minority, and swing readings, with its guard and its pins.";
    assert!(lines.contains(&slice_line));
    assert!(text.contains("Closed perfection rounds, 14 of 100"));
}

#[test]
fn contract_carries_the_round_ten_obligations() {
    let text = root_text("CONTRACT.md");
    for oid in ["GJ-21", "GJ-22"] {
        assert!(text.contains(oid), "{}", oid);
    }
}

#[test]
fn contract_carries_the_round_ten_amendment() {
    let text = root_text("CONTRACT.md");
    assert!(text.contains("FIX-GJ-14"));
}

#[test]
fn readme_reports_the_round_eleven_status() {
    let text = root_text("README.md");
    let lines = lines_of(&text);
    let standing = "- Contract, GJ-23 and GJ-24 met. They stand in full form in CONTRACT.md with the FIX-GJ-15 and FIX-GJ-16 repair amendments, and Round 11 closed on the merged slice 12 after three reviews, the red team's rejection repaired under FIX-GJ-16, and a discharging re-review.";
    assert!(lines.contains(&standing));
    let slice_line = "- Slice 12 has landed. It carries the bench reading over the three recorded poll sets, the pilot and the full poll and the second wave read together model by model under their set labels, stating each model's answer at each asking pooled over the juror names it answered under, the count of its valid answers, and how many of them stood with the pooled plurality of the asking, with its command and its golden, and the residual library fencing closure of FIX-GJ-15 over the tally, the stadium view, the cost reading, and the wave comparison reading, with its widened guard and its pins.";
    assert!(lines.contains(&slice_line));
    assert!(text.contains("Closed perfection rounds, 14 of 100"));
}

#[test]
fn contract_carries_the_round_eleven_obligations() {
    let text = root_text("CONTRACT.md");
    for oid in ["GJ-23", "GJ-24"] {
        assert!(text.contains(oid), "{}", oid);
    }
}

#[test]
fn contract_carries_the_round_eleven_amendment() {
    let text = root_text("CONTRACT.md");
    assert!(text.contains("FIX-GJ-15"));
}

#[test]
fn readme_reports_the_round_twelve_status() {
    let text = root_text("README.md");
    let lines = lines_of(&text);
    let standing = "- Contract, GJ-25 and GJ-26 met. They stand in full form in CONTRACT.md with the FIX-GJ-17, FIX-GJ-18, and FIX-GJ-19 repair amendments, and Round 12 closed on the merged slice 13 after three reviews, one standards Minor on a test comment repaired.";
    assert!(lines.contains(&standing));
    let slice_line = "- Slice 13 has landed. It carries the pool reading over the three recorded poll sets, the pilot and the full poll and the second wave read together question by question under their set labels, stating for every asking the counts of its whole poll pooled as one pool, the count standing behind every offered answer with its share of the valid answers, and the pooled plurality of the asking, with its command and its golden, and the formats appendix of FIX-GJ-17 stating the six formats of the repo at version 1 with its compatibility rule, and the reason closure of FIX-GJ-18 over the poll text guard, with its widened guard and its pins, and the resampled fixture of FIX-GJ-19, twelve polls drawn from the recorded pilot envelopes at fifty seats per model with the example poll standing as its first poll.";
    assert!(lines.contains(&slice_line));
    assert!(text.contains("Closed perfection rounds, 14 of 100"));
}

#[test]
fn contract_carries_the_round_twelve_obligations() {
    let text = root_text("CONTRACT.md");
    for oid in ["GJ-25", "GJ-26"] {
        assert!(text.contains(oid), "{}", oid);
    }
}

#[test]
fn contract_carries_the_round_twelve_amendment() {
    let text = root_text("CONTRACT.md");
    for aid in ["FIX-GJ-17", "FIX-GJ-18", "FIX-GJ-19"] {
        assert!(text.contains(aid), "{}", aid);
    }
}

#[test]
fn readme_reports_the_round_thirteen_status() {
    let text = root_text("README.md");
    let lines = lines_of(&text);
    let standing = "- Contract, GJ-27 and GJ-28 met. They stand in full form in CONTRACT.md with the FIX-GJ-20 repair amendment, and Round 13 closed on the merged slice 14 after three reviews, a repair pass over the aggregate tests, three approving delta reviews, and a hosted lint repair.";
    assert!(lines.contains(&standing));
    let slice_line = "- Slice 14 has landed. It carries the aggregate reading over the three recorded poll sets, the pilot and the full poll and the second wave read together question by question, stating for every question the counts of all its askings summed as one pool, the count standing behind every offered answer of the question with its share of the pooled valid answers, and the pooled plurality of the question, with its command and its golden, and the glob import gate of FIX-GJ-20 over the binary surface tests, with its attribute line and its pin.";
    assert!(lines.contains(&slice_line));
    assert!(text.contains("Closed perfection rounds, 14 of 100"));
}

#[test]
fn contract_carries_the_round_thirteen_obligations() {
    let text = root_text("CONTRACT.md");
    for oid in ["GJ-27", "GJ-28"] {
        assert!(text.contains(oid), "{}", oid);
    }
}

#[test]
fn contract_carries_the_round_thirteen_amendment() {
    let text = root_text("CONTRACT.md");
    assert!(text.contains("FIX-GJ-20"));
}

#[test]
fn readme_reports_the_round_fourteen_status() {
    let text = root_text("README.md");
    let lines = lines_of(&text);
    let standing = "- Contract, GJ-29 and GJ-30 met. They stand in full form in CONTRACT.md, and Round 14 closed on the merged slice 15 after three reviews, a repair pass over the canvass test header, and three approving delta reviews.";
    assert!(lines.contains(&standing));
    let slice_line = "- Slice 15 has landed. It carries the canvass reading over the three recorded poll sets, the pilot and the full poll and the second wave read together question by question, stating for every question, per jury, the counts of the jury summed over the askings of the question, the count standing behind every offered answer of the question with its share of the summed valid answers of the jury, and the plurality of the jury at the question, with its command and its golden.";
    assert!(lines.contains(&slice_line));
    assert!(text.contains("Closed perfection rounds, 14 of 100"));
}

#[test]
fn contract_carries_the_round_fourteen_obligations() {
    let text = root_text("CONTRACT.md");
    for oid in ["GJ-29", "GJ-30"] {
        assert!(text.contains(oid), "{}", oid);
    }
}

#[test]
fn the_binary_surface_glob_import_stands_gated_to_linux() {
    // The text pin FIX-GJ-20 states. The gate attribute
    // line stands immediately above the glob import line
    // in the binary surface tests, so the import draws in
    // the shared module's items only on the target where
    // a gated item uses them. The pin fails against the
    // Round 12 tree by assertion on every platform, the
    // import standing bare there. The five closed pipe
    // pins in that target pass before the repair on this
    // platform by design, the gate standing true here, and
    // stand as the repair's scope guard rather than as
    // discriminators.
    let text = root_text("rust/tests/test_binary_surface.rs");
    let lines = lines_of(&text);
    let import_at = lines
        .iter()
        .position(|line| *line == "use shared::*;")
        .expect("the glob import line");
    assert!(import_at >= 1);
    assert_eq!(lines[import_at - 1], "#[cfg(target_os = \"linux\")]");
}

#[test]
fn readme_states_the_local_gate_with_the_cargo_gate() {
    let text = root_text("README.md");
    // The platform phrase stands in the line under
    // FIX-GJ-17, which names the tested platform in the
    // gate line, so this pin moved with that amendment as
    // its named mechanical consequence.
    let line = "- Local gate, cargo test, cargo clippy --all-targets -- -D warnings, and cargo fmt --check, and the codepoint scan, cargo run --bin codepoint_scan, standing tested on Linux. CI runs the same commands.";
    assert!(lines_of(&text).contains(&line));
}

#[test]
fn contract_carries_the_formats_appendix_at_version_one() {
    let text = root_text("CONTRACT.md");
    assert!(text.contains("## Appendix, the formats at version 1"));
    for heading in [
        "### The recorded poll, version 1",
        "### The price table, version 1",
        "### The questions file, version 1",
        "### The decision envelope, version 1",
        "### The run record, version 1",
        "### The incident input, version 1",
    ] {
        assert!(text.contains(heading), "{}", heading);
    }
}

#[test]
fn agents_annex_states_the_local_gate_with_the_cargo_gate() {
    let text = root_text("AGENTS.md");
    assert!(text.contains(
        "cargo test, cargo clippy --all-targets -- -D warnings, and cargo fmt --check, and the codepoint scan, cargo run --bin codepoint_scan, and the suite runs offline"
    ));
}

#[test]
fn ci_runs_the_same_gate_as_the_local_gate() {
    let text = root_text(".github/workflows/ci.yml");
    let live: String = text
        .lines()
        .map(|line| line.split('#').next().unwrap_or(""))
        .collect::<Vec<&str>>()
        .join("\n");
    assert!(live.contains("cargo run --bin codepoint_scan"));
    assert!(!live.contains("pytest"));
    assert!(!live.contains("scripts/codepoint_scan.py"));
    assert!(!live.contains("setup-python"));
}

#[test]
fn readme_states_the_rust_port_line_exactly_once() {
    let text = root_text("README.md");
    let port_line = "- The Rust port has landed. It carries the same behavior as the Python package it replaces, checked against the same goldens, and the Python tree stands removed from the repo.";
    assert_eq!(text.matches(port_line).count(), 1);
}

#[test]
fn ci_carries_the_cargo_steps() {
    let text = root_text(".github/workflows/ci.yml");
    let live: String = text
        .lines()
        .map(|line| line.split('#').next().unwrap_or(""))
        .collect::<Vec<&str>>()
        .join("\n");
    assert!(live.contains("cargo fmt --check"));
    assert!(live.contains("cargo clippy --all-targets -- -D warnings"));
    assert!(live.contains("cargo test"));
}

#[test]
fn contract_carries_the_port_obligations() {
    let text = root_text("CONTRACT.md");
    for oid in [
        "PORT-1", "PORT-2", "PORT-3", "PORT-4", "PORT-5", "PORT-6", "PORT-7", "PORT-8", "PORT-9",
        "PORT-10", "PORT-11", "PORT-12", "PORT-13", "PORT-14", "PORT-15",
    ] {
        assert!(text.contains(oid), "{}", oid);
    }
}

// The manifest pin. The integration tests live beside the
// library under rust/tests, outside the root level tests
// directory cargo discovers, so each one is named in the
// manifest with its path. This pin fails the moment a test
// file stands in the tree without a manifest entry naming
// it, or an entry names a file that does not stand, so no
// test file can stand silently unrun.

#[test]
fn every_rust_test_file_is_named_in_the_cargo_manifest() {
    let manifest = root_text("Cargo.toml");
    let mut named: Vec<String> = Vec::new();
    let mut in_test = false;
    for line in manifest.lines() {
        let trimmed = line.trim();
        if trimmed == "[[test]]" {
            in_test = true;
            continue;
        }
        if trimmed.starts_with('[') {
            in_test = false;
            continue;
        }
        if in_test && trimmed.starts_with("name = ") {
            named.push(
                trimmed
                    .trim_start_matches("name = ")
                    .trim_matches('"')
                    .to_string(),
            );
        }
    }
    let mut on_disk: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(repo_root().join("rust/tests")).expect("the test tree") {
        let entry = entry.expect("a test tree entry");
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.ends_with(".rs") {
            on_disk.push(name.trim_end_matches(".rs").to_string());
        }
    }
    named.sort();
    on_disk.sort();
    assert_eq!(named, on_disk);
}
