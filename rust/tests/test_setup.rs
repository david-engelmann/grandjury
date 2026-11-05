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
    assert!(text.contains("Closed perfection rounds, 6 of 100"));
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
    // consequence, and restored at the Round 6 closure.
    // This pin counts the slices standing in review across
    // the whole README. Slice 7 stood in review while
    // Round 6 was built, and the Round 6 closure landed
    // slice 7, so the count stands at zero. The round pins
    // below hold the standing lines and the slice lines
    // themselves.
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
    assert!(text.contains("Closed perfection rounds, 6 of 100"));
}

#[test]
fn contract_carries_the_round_six_obligations() {
    let text = root_text("CONTRACT.md");
    for oid in ["GJ-13", "GJ-14"] {
        assert!(text.contains(oid), "{}", oid);
    }
}

#[test]
fn readme_states_the_local_gate_with_the_cargo_gate() {
    let text = root_text("README.md");
    let line = "- Local gate, cargo test, cargo clippy --all-targets -- -D warnings, and cargo fmt --check, and the codepoint scan, cargo run --bin codepoint_scan. CI runs the same commands.";
    assert!(lines_of(&text).contains(&line));
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
