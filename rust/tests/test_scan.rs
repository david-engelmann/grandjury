//! Pins for the codepoint scan scope stated in SETUP-3 as
//! amended by the scan scope amendment.
//!
//! Translated from tests/test_scan.py. The Python pins run
//! the scan script as a subprocess against a planted tree,
//! because the rule is about which files on disk the scan
//! opens. The port pins plant the same trees against the
//! library scan the scan command runs, which reads a tree by
//! the same scope. The exit status of the scan command is
//! one when either count is nonzero and zero otherwise, so
//! each pin states the counts the Python pin states through
//! its status and its printed line. Two pins are the port
//! additions of PORT-9, a dash in a Rust file failing the
//! scan and a dash inside the cargo target directory passing
//! it, and they stand in the Python suite in the same words.

mod shared;

use grandjury::scan::scan_tree;
use shared::*;

const EM_DASH: &str = "\u{2014}";
const EN_DASH: &str = "\u{2013}";

fn plant(root: &std::path::Path, relative: &str, text: &str) {
    let path = root.join(relative);
    std::fs::create_dir_all(path.parent().expect("a parent")).expect("directories");
    std::fs::write(&path, text).expect("a planted file");
}

#[test]
fn a_clean_planted_tree_passes() {
    let scratch = TempDir::new("scan-clean");
    plant(&scratch.path, "notes.md", "plain text\n");
    assert_eq!(scan_tree(&scratch.path), (0, 0));
}

#[test]
fn a_dash_in_an_extensionless_file_fails_the_scan() {
    let scratch = TempDir::new("scan-extensionless");
    plant(
        &scratch.path,
        "LICENSE",
        &format!("licence text {} more text\n", EM_DASH),
    );
    assert_eq!(scan_tree(&scratch.path), (1, 0));
}

#[test]
fn a_dash_in_a_python_file_fails_the_scan() {
    let scratch = TempDir::new("scan-python");
    plant(
        &scratch.path,
        "module.py",
        &format!("# a comment {} here\n", EN_DASH),
    );
    assert_eq!(scan_tree(&scratch.path), (0, 1));
}

#[test]
fn dashes_inside_an_ignored_directory_pass_the_scan() {
    let scratch = TempDir::new("scan-ignored");
    plant(
        &scratch.path,
        ".venv/planted.md",
        &format!("{} and {}\n", EM_DASH, EN_DASH),
    );
    assert_eq!(scan_tree(&scratch.path), (0, 0));
}

#[test]
fn dashes_inside_the_recorded_envelopes_pass_the_scan() {
    let scratch = TempDir::new("scan-pilot-envelopes");
    plant(
        &scratch.path,
        "examples/pilot/envelopes/q01-jev-1.json",
        &format!(
            "{{\"reading\": \"one {} two {} three\"}}\n",
            EM_DASH, EN_DASH
        ),
    );
    assert_eq!(scan_tree(&scratch.path), (0, 0));
}

#[test]
fn a_dash_in_the_pilot_directory_outside_envelopes_fails() {
    let scratch = TempDir::new("scan-pilot-outside");
    plant(
        &scratch.path,
        "examples/pilot/poll-01.json",
        &format!("{{\"question\": \"one {} two\"}}\n", EM_DASH),
    );
    assert_eq!(scan_tree(&scratch.path), (1, 0));
}

#[test]
fn dashes_inside_the_full_envelopes_pass_the_scan() {
    let scratch = TempDir::new("scan-full-envelopes");
    plant(
        &scratch.path,
        "examples/full/envelopes/q01-openai-1.json",
        &format!(
            "{{\"reading\": \"one {} two {} three\"}}\n",
            EM_DASH, EN_DASH
        ),
    );
    assert_eq!(scan_tree(&scratch.path), (0, 0));
}

#[test]
fn a_dash_in_the_full_directory_outside_envelopes_fails() {
    let scratch = TempDir::new("scan-full-outside");
    plant(
        &scratch.path,
        "examples/full/poll-01.json",
        &format!("{{\"question\": \"one {} two\"}}\n", EM_DASH),
    );
    assert_eq!(scan_tree(&scratch.path), (1, 0));
}

#[test]
fn a_dash_elsewhere_in_examples_still_fails_the_scan() {
    let scratch = TempDir::new("scan-examples");
    plant(
        &scratch.path,
        "examples/notes.md",
        &format!("one {} two\n", EN_DASH),
    );
    assert_eq!(scan_tree(&scratch.path), (0, 1));
}

#[test]
fn a_dash_in_a_rust_file_fails_the_scan() {
    let scratch = TempDir::new("scan-rust");
    plant(
        &scratch.path,
        "module.rs",
        &format!("// a comment {} here\n", EM_DASH),
    );
    assert_eq!(scan_tree(&scratch.path), (1, 0));
}

#[test]
fn dashes_inside_the_target_directory_pass_the_scan() {
    let scratch = TempDir::new("scan-target");
    plant(
        &scratch.path,
        "target/planted.md",
        &format!("{} and {}\n", EM_DASH, EN_DASH),
    );
    assert_eq!(scan_tree(&scratch.path), (0, 0));
}
