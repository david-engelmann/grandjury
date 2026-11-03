//! Pins for the assembled file lists of the makers, PORT-6
//! as amended by PORT-12.
//!
//! The assembly functions pilot_files, full_files, and
//! synthetic_files are pinned byte for byte against the
//! recorded trees under examples, the form PORT-6 states.
//! The full maker probes then mutate a copied tree the three
//! ways the port reviews probed, an envelope the rebuild
//! drops, a standing poll file missing outright, and a
//! response dropped from a standing poll alone, and pin the
//! agreement golden standing over the built polls in each.

mod shared;

use std::path::{Path, PathBuf};

use grandjury::makers::{full_files, pilot_files, synthetic_files};
use shared::*;

fn assert_files_match_tree(files: &[(String, String)], root: &Path) {
    for (relative, content) in files {
        let committed = read_text(&root.join(relative));
        assert_eq!(content, &committed, "{}", relative);
    }
}

#[test]
fn the_pilot_files_match_the_recorded_tree_byte_for_byte() {
    let files = pilot_files(&examples()).expect("the pilot files");
    let mut names: Vec<&str> = files.iter().map(|(name, _)| name.as_str()).collect();
    names.sort();
    let mut expected: Vec<String> = (1..=12)
        .map(|index| format!("pilot/poll-{:02}.json", index))
        .collect();
    expected.push("pilot/run_record.json".to_string());
    expected.push("pilot_poll.json".to_string());
    expected.push("pilot-stadium-golden.txt".to_string());
    expected.push("pilot-cost-golden.txt".to_string());
    let mut expected_names: Vec<&str> = expected.iter().map(|name| name.as_str()).collect();
    expected_names.sort();
    assert_eq!(names, expected_names);
    assert_files_match_tree(&files, &examples());
}

#[test]
fn the_full_files_match_the_recorded_tree_byte_for_byte() {
    let files = full_files(&examples()).expect("the full files");
    let mut names: Vec<&str> = files.iter().map(|(name, _)| name.as_str()).collect();
    names.sort();
    let mut expected: Vec<String> = (1..=40)
        .map(|index| format!("full/poll-{:02}.json", index))
        .collect();
    expected.push("full/run_record.json".to_string());
    expected.push("full_poll.json".to_string());
    expected.push("full-stadium-golden.txt".to_string());
    expected.push("full-cost-golden.txt".to_string());
    expected.push("full-agreement-golden.txt".to_string());
    let mut expected_names: Vec<&str> = expected.iter().map(|name| name.as_str()).collect();
    expected_names.sort();
    assert_eq!(names, expected_names);
    assert_files_match_tree(&files, &examples());
}

#[test]
fn the_synthetic_files_match_the_recorded_tree_byte_for_byte() {
    let files = synthetic_files().expect("the synthetic files");
    let names: Vec<&str> = files.iter().map(|(name, _)| name.as_str()).collect();
    assert_eq!(
        names,
        vec![
            "recorded_poll.json",
            "prices.json",
            "stadium-golden.txt",
            "cost-golden.txt",
        ]
    );
    assert_files_match_tree(&files, &examples());
}

/// Copy the parts of the examples tree the full maker reads
/// and the standing polls it must not read into a scratch
/// root of the same shape, so a probe can mutate the copy.
fn copy_full_tree(scratch: &Path) -> PathBuf {
    let root = scratch.join("examples");
    let source = examples();
    copy_dir(&source.join("full"), &root.join("full"));
    std::fs::copy(
        source.join("full-prices.json"),
        root.join("full-prices.json"),
    )
    .expect("a copied price table");
    root
}

fn copy_dir(source: &Path, target: &Path) {
    std::fs::create_dir_all(target).expect("a copied directory");
    for entry in std::fs::read_dir(source).expect("a read directory") {
        let entry = entry.expect("a directory entry");
        let held = target.join(entry.file_name());
        if entry.path().is_dir() {
            copy_dir(&entry.path(), &held);
        } else {
            std::fs::copy(entry.path(), &held).expect("a copied file");
        }
    }
}

fn file_named<'a>(files: &'a [(String, String)], name: &str) -> &'a str {
    files
        .iter()
        .find(|(held, _)| held == name)
        .map(|(_, content)| content.as_str())
        .expect("a named file")
}

fn section_of(golden: &str, question: &str) -> String {
    let lines: Vec<&str> = golden.lines().collect();
    let start = lines
        .iter()
        .position(|line| *line == format!("question, {}", question))
        .expect("a section");
    let mut end = start + 1;
    while end < lines.len() && !lines[end].starts_with("question, ") {
        end += 1;
    }
    lines[start..end].join("\n")
}

#[test]
fn the_full_golden_follows_the_rebuild_when_an_envelope_drops() {
    let scratch = TempDir::new("makers-envelope-drops");
    let root = copy_full_tree(&scratch.path);
    std::fs::remove_file(root.join("full/envelopes/q05-jev-3.json")).expect("a dropped envelope");
    let files = full_files(&root).expect("the full files");
    let golden = file_named(&files, "full-agreement-golden.txt");
    let committed = read_text(&examples().join("full-agreement-golden.txt"));
    assert_ne!(golden, committed);
    let section = section_of(
        golden,
        "Should the farm shop sell flowers as well as vegetables?",
    );
    assert!(
        section.contains("jury, jev-1.13.0, recorded 3, valid 0, abstained 3, spoiled 0"),
        "{}",
        section
    );
}

#[test]
fn the_full_maker_runs_when_a_standing_poll_file_is_missing() {
    let scratch = TempDir::new("makers-standing-missing");
    let root = copy_full_tree(&scratch.path);
    std::fs::remove_file(root.join("full/poll-05.json")).expect("a removed poll file");
    let files = full_files(&root).expect("the full files");
    let poll = file_named(&files, "full/poll-05.json");
    assert_eq!(poll, read_text(&examples().join("full/poll-05.json")));
    let golden = file_named(&files, "full-agreement-golden.txt");
    assert_eq!(
        golden,
        read_text(&examples().join("full-agreement-golden.txt"))
    );
}

#[test]
fn the_full_golden_ignores_a_response_dropped_from_a_standing_poll() {
    let scratch = TempDir::new("makers-standing-dropped");
    let root = copy_full_tree(&scratch.path);
    let standing = root.join("full/poll-05.json");
    let held = read_json(&standing);
    let mut pairs: Vec<(grandjury::pytext::PText, grandjury::json::JsonValue)> = Vec::new();
    if let grandjury::json::JsonValue::Obj(entries) = &held {
        for (key, value) in entries {
            if key.display() == "responses" {
                if let grandjury::json::JsonValue::Arr(items) = value {
                    pairs.push((
                        key.clone(),
                        grandjury::json::JsonValue::Arr(items[1..].to_vec()),
                    ));
                    continue;
                }
            }
            pairs.push((key.clone(), value.clone()));
        }
    }
    let poll = grandjury::json::JsonValue::object(pairs);
    std::fs::write(
        &standing,
        format!("{}\n", grandjury::json::dumps(&poll, Some(2), false, false)),
    )
    .expect("a rewritten poll file");
    let files = full_files(&root).expect("the full files");
    let golden = file_named(&files, "full-agreement-golden.txt");
    assert_eq!(
        golden,
        read_text(&examples().join("full-agreement-golden.txt"))
    );
}
