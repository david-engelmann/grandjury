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
    assert!(text.contains("Closed perfection rounds, 26 of 100"));
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
    // consequence, and restored at the Round 14 closure, and
    // altered again at the Round 15 slice for the same
    // consequence, and restored at the Round 15 closure, and
    // altered again at the Round 16 slice for the same
    // consequence, and restored at the Round 16 closure, and
    // altered again at the Round 17 slice for the same
    // consequence, and restored at the Round 17 closure, and
    // altered again at the Round 18 slice for the same
    // consequence, and restored at the Round 18 closure, and
    // altered again at the Round 19 slice for the same
    // consequence, and restored at the Round 19 closure, and
    // altered again at the Round 20 slice for the same
    // consequence, and restored at the Round 20 closure, and
    // altered again at the Round 21 slice for the same
    // consequence, and restored at the Round 21 closure, and
    // altered again at the Round 22 slice for the same
    // consequence, and restored at the Round 22 closure, and
    // altered again at the Round 23 slice for the same
    // consequence, and restored at the Round 23 closure, and
    // altered again at the Round 24 slice for the same
    // consequence, and restored at the Round 24 closure, and
    // altered again at the Round 25 slice for the same
    // consequence, and restored at the Round 25 closure, and
    // altered again at the Round 26 slice for the same
    // consequence, and restored at the Round 26 closure.
    // This pin counts the slices standing in
    // review across the whole README. No slice stands in
    // review while Round 27 is unbuilt, so the count stands
    // at zero. The round pins below hold
    // the standing lines and the slice lines themselves.
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
    assert!(text.contains("Closed perfection rounds, 26 of 100"));
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
    assert!(text.contains("Closed perfection rounds, 26 of 100"));
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
    assert!(text.contains("Closed perfection rounds, 26 of 100"));
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
    assert!(text.contains("Closed perfection rounds, 26 of 100"));
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
    assert!(text.contains("Closed perfection rounds, 26 of 100"));
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
    assert!(text.contains("Closed perfection rounds, 26 of 100"));
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
    assert!(text.contains("Closed perfection rounds, 26 of 100"));
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
    assert!(text.contains("Closed perfection rounds, 26 of 100"));
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
    assert!(text.contains("Closed perfection rounds, 26 of 100"));
}

#[test]
fn contract_carries_the_round_fourteen_obligations() {
    let text = root_text("CONTRACT.md");
    for oid in ["GJ-29", "GJ-30"] {
        assert!(text.contains(oid), "{}", oid);
    }
}

#[test]
fn readme_reports_the_round_fifteen_status() {
    let text = root_text("README.md");
    let lines = lines_of(&text);
    let standing = "- Contract, GJ-31 and GJ-32 met. They stand in full form in CONTRACT.md, and Round 15 closed on the merged slice 16 after three reviews approving with findings and no repair owed, with the standards Minor on the duplicated counting helpers carried to a later round.";
    assert!(lines.contains(&standing));
    let slice_line = "- Slice 16 has landed. It carries the roll reading over the three recorded poll sets, the pilot and the full poll and the second wave read together jury by jury, stating for every jury the block of the jury at every question it stands at, the question line and the jury line and the answer rows and the plurality line of the canvass block whole, and the standing of the jury over the record in the standing line that closes its section, with its command and its golden.";
    assert!(lines.contains(&slice_line));
    assert!(text.contains("Closed perfection rounds, 26 of 100"));
}

#[test]
fn contract_carries_the_round_fifteen_obligations() {
    let text = root_text("CONTRACT.md");
    for oid in ["GJ-31", "GJ-32"] {
        assert!(text.contains(oid), "{}", oid);
    }
}

#[test]
fn readme_reports_the_round_sixteen_status() {
    let text = root_text("README.md");
    let lines = lines_of(&text);
    let standing = "- Contract, GJ-33 and GJ-34 met. They stand in full form in CONTRACT.md, and Round 16 closed on the merged slice 17 after three reviews, with the standards Minor on the muster pin comments repaired in the slice.";
    assert!(lines.contains(&standing));
    let slice_line = "- Slice 17 has landed. It carries the muster reading over the three recorded poll sets, the pilot and the full poll and the second wave read together question by question, stating for every question, under each offered answer of the question, the row of that answer in the block of every jury of the question, the count of the valid responses of the question that chose the answer, the count of the juries whose summed count of the answer stands at one or more, and the count of the juries whose block plurality the answer is, with its command and its golden, and the fold of the pooled counting helpers of the canvass and the roll into the agreement module, with its three text pins, and the naming of the models of GJ-32 under FIX-GJ-21, with its pin.";
    assert!(lines.contains(&slice_line));
    assert!(text.contains("Closed perfection rounds, 26 of 100"));
}

#[test]
fn contract_carries_the_round_sixteen_obligations() {
    let text = root_text("CONTRACT.md");
    for oid in ["GJ-33", "GJ-34"] {
        assert!(text.contains(oid), "{}", oid);
    }
}

#[test]
fn readme_reports_the_round_seventeen_status() {
    let text = root_text("README.md");
    let lines = lines_of(&text);
    let standing = "- Contract, GJ-35 and GJ-36 met. They stand in full form in CONTRACT.md, and Round 17 closed on the merged slice 18 after three reviews, with the standards Minor on the margin pin comment repaired in the slice.";
    assert!(lines.contains(&standing));
    let slice_line = "- Slice 18 has landed. It carries the margin reading over the three recorded poll sets, the pilot and the full poll and the second wave read together question by question, stating for every question the margin by which the pooled plurality of the question stands and the margin by which the summed plurality of each jury at the question stands, with its command and its golden.";
    assert!(lines.contains(&slice_line));
    assert!(text.contains("Closed perfection rounds, 26 of 100"));
}

#[test]
fn contract_carries_the_round_seventeen_obligations() {
    let text = root_text("CONTRACT.md");
    for oid in ["GJ-35", "GJ-36"] {
        assert!(text.contains(oid), "{}", oid);
    }
}

#[test]
fn readme_reports_the_round_eighteen_status() {
    let text = root_text("README.md");
    let lines = lines_of(&text);
    let standing = "- Contract, GJ-37 and GJ-38 met. They stand in full form in CONTRACT.md with the FIX-GJ-23 repair amendment, and Round 18 closed on the merged slice 19 after three reviews, the design's twice asking shape restated as the set rules refusal under FIX-GJ-23 with its pin restated in that form, and the round's eighteen unexecuted mutation runs and its base run carried to the Round 19 design warm-up.";
    assert!(lines.contains(&standing));
    let slice_line = "- Slice 19 has landed. It carries the accord reading over the three recorded poll sets, the pilot and the full poll and the second wave read together question by question, stating for every question the standing of each set that asked it, the counts and the pooled plurality and the margin of that set at the question, and whether the sets that hold a plurality at the question name one answer, with its command and its golden.";
    assert!(lines.contains(&slice_line));
    assert!(text.contains("Closed perfection rounds, 26 of 100"));
}

#[test]
fn contract_carries_the_round_eighteen_obligations() {
    let text = root_text("CONTRACT.md");
    for oid in ["GJ-37", "GJ-38"] {
        assert!(text.contains(oid), "{}", oid);
    }
}

#[test]
fn readme_reports_the_round_nineteen_status() {
    let text = root_text("README.md");
    let lines = lines_of(&text);
    let standing = "- Contract, GJ-39 and GJ-40 met. They stand in full form in CONTRACT.md, and Round 19 closed on the merged slice 20 after three reviews approving with findings, with the standing header's mutation postures restated to the executed record by two repair commits in the slice.";
    assert!(lines.contains(&standing));
    let slice_line = "- Slice 20 has landed. It carries the standing reading over the three recorded poll sets, the pilot and the full poll and the second wave read together set by set, stating for every set the standing of the set over the whole record, the questions it asked and at how many of them it held a plurality and at how many its top counts tied and at how many it held no valid response and at how many its plurality was the accord of the question and at how many it was the pooled plurality of the question, with its command and its golden.";
    assert!(lines.contains(&slice_line));
    assert!(text.contains("Closed perfection rounds, 26 of 100"));
}

#[test]
fn contract_carries_the_round_nineteen_obligations() {
    let text = root_text("CONTRACT.md");
    for oid in ["GJ-39", "GJ-40"] {
        assert!(text.contains(oid), "{}", oid);
    }
}

#[test]
fn readme_reports_the_round_twenty_status() {
    let text = root_text("README.md");
    let lines = lines_of(&text);
    let standing = "- Contract, GJ-41 and GJ-42 met. They stand in full form in CONTRACT.md, and Round 20 closed on the merged slice 21 after three reviews, with the red team approving on recomputation of the answers golden byte for byte, its two Major findings in the shape and defective poll pins repaired in the slice, and the defective poll genus completed whole across the file's seven pins.";
    assert!(lines.contains(&standing));
    let slice_line = "- Slice 21 has landed. It carries the answers reading over the three recorded poll sets, the pilot and the full poll and the second wave read together answer by answer, stating for every answer each question at which it was offered, the count of parts holding it at that question and of those the counts standing in accord and with the pool, and the standing of the answer over the record, with its command and its golden.";
    assert!(lines.contains(&slice_line));
    assert!(text.contains("Closed perfection rounds, 26 of 100"));
}

#[test]
fn contract_carries_the_round_twenty_obligations() {
    let text = root_text("CONTRACT.md");
    for oid in ["GJ-41", "GJ-42"] {
        assert!(text.contains(oid), "{}", oid);
    }
}

#[test]
fn readme_reports_the_round_twenty_one_status() {
    let text = root_text("README.md");
    let lines = lines_of(&text);
    let standing = "- Contract, GJ-43 and GJ-44 met. They stand in full form in CONTRACT.md with the FIX-GJ-25 repair amendment stating the all-empty form of the answers reading, and Round 21 closed on the merged slice 22 after three reviews, with the red team approving with findings on executed recomputation, the findings standing against the design record's discrimination lists and carried to the records pass, the slice's one doc comment Minor in the pairs module repaired in the slice, and the answers module's doc comment instance of the same genus left standing as a named item for a later round.";
    assert!(lines.contains(&standing));
    let slice_line = "- Slice 22 has landed. It carries the pairing reading over the three recorded poll sets, the pilot and the full poll and the second wave read together set pair by set pair, stating for every pair of sets each question the pair shares, the part of each set at the question in the form of the accord and the pairing of the pair at the question, and the standing of the pair over the record, with its command and its golden.";
    assert!(lines.contains(&slice_line));
    assert!(text.contains("Closed perfection rounds, 26 of 100"));
}

#[test]
fn contract_carries_the_round_twenty_one_obligations() {
    let text = root_text("CONTRACT.md");
    for oid in ["GJ-43", "GJ-44", "FIX-GJ-25"] {
        assert!(text.contains(oid), "{}", oid);
    }
}

#[test]
fn readme_reports_the_round_twenty_two_status() {
    let text = root_text("README.md");
    let lines = lines_of(&text);
    let standing = "- Contract, GJ-45 and GJ-46 met. They stand in full form in CONTRACT.md with the FIX-GJ-26 repair amendment restating the census sentence of GJ-46 in the comma form, and Round 22 closed on the merged slice 23 after three reviews, with the execution record of GJ-45 standing executed whole on its charged trees, the mutation sets NH to NT on the Round 19 slice tree and OA to OT on the Round 20 and Round 21 slice trees, the base runs of Rounds 19 and 20, and the cargo leg of NG failing pin 12 alone, the closure bar of GJ-45 standing satisfied on that record, with the spec review approving with findings at one Minor standing against the build record and three Observations, the standards review approving with findings at one Major cured in the slice under FIX-GJ-26, two Minors cured in the same repair or standing recorded, and two Observations, the red team approving with findings at two Observations, and CodeRabbit's one Minor dismissed with reason on the record, the slice standing at six paths against main and the five path sentence of the slice record standing corrected here.";
    assert!(lines.contains(&standing));
    let slice_line = "- Slice 23 has landed. It carries the execution record of the two rounds before the pairing round, the mutation sets of Rounds 19 and 20 executed on the trees that carry their pins, the base runs of both rounds, and the cargo leg of NG, and it restates the doc comment of the answers module to the set rules of GJ-2, adding no reading and no command.";
    assert!(lines.contains(&slice_line));
    assert!(text.contains("Closed perfection rounds, 26 of 100"));
}

#[test]
fn contract_carries_the_round_twenty_two_obligations() {
    let text = root_text("CONTRACT.md");
    for oid in ["GJ-45", "GJ-46"] {
        assert!(text.contains(oid), "{}", oid);
    }
}

#[test]
fn readme_reports_the_round_twenty_three_status() {
    let text = root_text("README.md");
    let lines = lines_of(&text);
    let standing = "- Contract, GJ-47 and GJ-48 met. They stand in full form in CONTRACT.md with the FIX-GJ-27 repair amendment restating the twice asking shape of GJ-48 in the refusal form and the FIX-GJ-28 repair amendment restating the standings clause of GJ-48 and the first clause of its S8, and Round 23 closed on the merged slice 24 after three reviews, with the first build of the round stopping on the design's twice asking premise, the set rules refusing a set that holds one question twice, and the owner's ruling spending FIX-GJ-27 on the precedent of FIX-GJ-23, the shape standing in the refusal form from the section's landing, the mutation QE standing uncharged as uncatchable by construction, and the guard mutation QI charged in its place and executed at pins 11 and 14, the command target's duplicate label pin falling beside the set, with the spec review approving with findings at one Major, two Minors, and four Observations, its Major and its first Minor standing cured in the slice, the tenth pin restated whole to the S5 fixture its charge states and renamed lowest_terms_shape_states_each_share_reduced under the owner's ruling, and the seventh pin's abstained response restored, its posture under QA standing as computed, with the standards review approving with findings at three Observations, the red team approving with findings at one Major and two Minors, all three of its findings standing against text and none moving a figure, and CodeRabbit returning two inline Minors on the slice, both read in full and judged valid on the code and both cured in the slice under FIX-GJ-28, no figure and no behaviour moving under either restatement of that amendment, the reseated build report's accounts of the tenth pin under QA and QC and QG and of the seventh pin under QA standing named here as written against the substituted and leaner fixtures without naming them, the repair report carrying the true mechanisms, and the shares target's header standing restated to the final executed sets in the same repair, the leanness of the sixth and eighth and ninth pins standing posture neutral as a named item on the record, the thirteenth pin's first form standing against the design record as the pairs form transplanted to a reading stating one section for each set, the shipped pin asserting the empty list at its closing line alone at zero sets, and the design record's two wordings cured in the pins standing recorded with them, the sets figure in the unsingularized form of the family and the fifth pin counting the accord's part lines at 64, and the design record's own errata standing named, its twice asking shape sentence itself, its surface paragraph sentence standing FIX-GJ-27 free and unspent in the round, superseded by the ruling's spend and answered by the amendment entry in the posture the Round 22 closure set, and its count of 33 test files against the 32 the tree counts.";
    assert!(lines.contains(&standing));
    let slice_line = "- Slice 24 has landed. It carries the shares reading over the three recorded poll sets, the pilot and the full poll and the second wave read together set by set, stating for every set each question it asks and the share of the question's valid responses the set holds at it, with its command and its golden.";
    assert!(lines.contains(&slice_line));
    assert!(text.contains("Closed perfection rounds, 26 of 100"));
}

#[test]
fn contract_carries_the_round_twenty_three_obligations() {
    let text = root_text("CONTRACT.md");
    for oid in ["GJ-47", "GJ-48", "FIX-GJ-27"] {
        assert!(text.contains(oid), "{}", oid);
    }
}

#[test]
fn readme_reports_the_round_twenty_four_status() {
    let text = root_text("README.md");
    let lines = lines_of(&text);
    let standing = "- Contract, GJ-49 and GJ-50 met. They stand in full form in CONTRACT.md with the FIX-GJ-24 repair amendment amending GJ-1 and GJ-4 to part the two failures the loaders' text reading folds, a file that stands and reads as bytes that are not valid UTF-8 drawing the single defect named file is not valid UTF-8 at the poll loader and at the price table loader and the read defect standing unchanged for a file that does not stand or cannot be read, the amendment standing reserved to the non UTF-8 refusal since Round 21 and spent in this round, and Round 24 closed on the merged slice 25 after three reviews, with the first build of the round stopping at the mutation QD, its executed set standing at seventeen falls against the five pins the design's computation charged in the set and the two falls it named beside it, and the owner's ruling restating the mutation's account to the executed falls, five in the set, a_file_that_is_not_valid_utf8_draws_the_decode_defect and a_directory_standing_as_the_poll_file_draws_the_read_defect of the polls target, a_price_file_that_is_not_valid_utf8_draws_the_decode_defect of the cost target, and a_tally_over_a_poll_file_that_is_not_valid_utf8_exits_one and a_census_over_a_set_whose_first_poll_file_is_not_valid_utf8_exits_one of the command target, and twelve beside it, an_unreadable_file_draws_the_named_defect and an_unreadable_price_file_draws_the_named_defect at the two loader seats, a_missing_poll_file_exits_one, the_census_command_refuses_a_missing_first_directory, the_agreement_command_names_a_missing_file, the_compare_command_refuses_a_missing_first_directory, and the missing first directory pins the_bench_command_refuses_a_missing_first_directory, the_dissent_command_refuses_a_missing_first_directory, the_ledger_command_refuses_a_missing_first_directory, the_minority_command_refuses_a_missing_first_directory, the_pool_command_refuses_a_missing_first_directory, and the_swing_command_refuses_a_missing_first_directory, every one of the twelve asserting the read defect text over a missing file or a missing first directory through the poll loader or the poll set loader, the design's computation standing corrected by the execution and the ruling in the form the spec review's Observation charges this closure to carry, the reseated build adopting the stopped lane's slice whole after verifying it blob for blob against the remote and reproducing the seventeen falls exactly on the repaired head, the warm up and QA and QB and QC standing as the stopped lane's executions on that identical tree, and GJ-50's execution discharging the empty section drop the shares target's header names uncharged, the mutation failing the shares pins an_empty_set_stands_beside_the_record_with_no_part and the_empty_forms_of_the_reading_stand_at_zero_figures and no other pin, the fifty fifth pin of the polls target standing as the parting's witness, passing on the base tree by design and falling under QD alone, and the slice's fifth commit repairing the one formatting site inside the forty seventh pin under the design's repair clause, its one trailing comma insertion standing enumerated in the repair's proof, with the spec review approving with findings at one Observation, the standards review approving with findings at four Observations, its priced items standing for later rounds, the duplicated reading and defect mapping of the polls module priced at a later opening of that module, the obligations pin's presence form priced at the next contract opening, and the one redundant word of GJ-49's wording priced under a later repair amendment, the red team approving with no findings, its eighty nine drives over the twenty two commands standing at the parting's seats exactly as GJ-49 states them, and CodeRabbit returning no inline comments on the slice, and the design record of Round 23 standing with its errata as records items, its surface paragraph sentence standing FIX-GJ-27 free and unspent in that round, superseded by the ruling's spend and answered by the amendment entry in the posture the Round 22 closure set, its thirteenth pin's closing alone form standing cured in substance by FIX-GJ-28, and its count of 33 test files standing true at this round's head by the shares target's landing, the figure standing as an erratum against the 32 its own tree counted.";
    assert!(lines.contains(&standing));
    let slice_line = "- Slice 25 has landed. It carries the decode refusal at the loader seats, the two failures of the loaders' text reading parted so that a file whose bytes are not valid UTF-8 draws its own defect at the poll loader and at the price table loader, under the repair amendment FIX-GJ-24, and the execution of the empty section drop against the shares reading, adding no reading and no command.";
    assert!(lines.contains(&slice_line));
    assert!(text.contains("Closed perfection rounds, 26 of 100"));
}

// The whole obligation assertion cure of Round 26 stands
// at this pin and at the Round 25 pin beside it. Each
// asserts its obligations whole inside the sections their
// headings open, where the pins as landed asserted the
// identifier and the opening words inside the section.
// A repair inside the slice strengthens the same three
// pins against the two constructions the slice's red
// team executed beside the cure. Each pin asserts beside
// the whole text that each heading it guards stands once
// in the contract and that the section the heading opens
// carries the heading and the whole text and nothing
// else, so a paragraph cut in its place falls at its pin
// though its whole text stands duplicated behind the cut
// inside its own section, and though its whole text
// stands under a second standing of its heading planted
// ahead of its own.
// The body cut stands executed on the Round 26 slice tree
// in its own fresh target directory at each of the eight
// seats the cure reaches, the obligation's heading and
// opening words standing and its body standing cut away
// at the second sentence. At the cuts of GJ-49 and GJ-50
// and FIX-GJ-24 this pin falls alone, at the cuts of
// GJ-51 and GJ-52 and FIX-GJ-29 the Round 25 pin falls
// alone, and at the cuts of GJ-53 and GJ-54 the Round 26
// pin falls alone, every set standing read from assertion
// failures alone. On the Round 25 closure tree the same
// six cuts of Rounds 24 and 25 pass whole at 816 and 0,
// the pins as landed passing over the cut contracts,
// which is the gap the cure closes. At the body cut of
// GJ-48, a section no pin asserts whole, the slice suite
// passes whole, and at the relocation of GJ-49's opening
// sentence out of its section this pin falls alone. This
// block asserts nothing.
#[test]
fn contract_carries_the_round_twenty_four_obligations() {
    let text = root_text("CONTRACT.md");
    let lines = lines_of(&text);
    let gj49 = contract_section(
        &text,
        "### GJ-49 full form, the decode refusal at the loader seats",
    );
    assert!(gj49.contains("GJ-49. The decode refusal at the loader seats. The file loaders of GJ-1 and GJ-4, load_poll_file and load_prices_file in the polls module, part the two failures their text reading folds today. A file that does not stand, or that stands and cannot be read, draws the single defect named file cannot be read, as GJ-1 and GJ-4 state it today. A file that stands and reads as bytes that are not valid UTF-8 draws the single defect named file is not valid UTF-8, and no other defect stands beside it, and no poll and no price table stands from such a file. The parting stands at the reading of the file and before any parsing, so a file whose bytes are not valid UTF-8 draws the decode defect whatever its bytes would have parsed to, and a file whose bytes are valid UTF-8 draws no decode defect, its defects standing as the standing loaders state them. The polls module parts the two failures in its text reading for the two file loaders, and the standing folded reading read_text_file stands unchanged for its other callers, the makers module's required reading among them, whose refusal form is not a defect of GJ-1 and does not move in this round. Every command surface that loads a poll or a price table through the file loaders states the decode defect in the standing form of its surface, the line invalid poll or invalid price table standing over the line defect and the defect, and a set directory whose poll file holds bytes that are not valid UTF-8 draws the same two lines at the first such file, the set loading standing whole or nothing as GJ-12 states. The defect vocabulary of GJ-1 gains the one phrase, file is not valid UTF-8, and loses none. Test, rust/tests/test_polls.rs and rust/tests/test_cost.rs and rust/tests/test_cli.rs."));
    assert_eq!(
        lines
            .iter()
            .filter(|line| **line == "### GJ-49 full form, the decode refusal at the loader seats")
            .count(),
        1
    );
    assert_eq!(
        gj49,
        "### GJ-49 full form, the decode refusal at the loader seats\n\nGJ-49. The decode refusal at the loader seats. The file loaders of GJ-1 and GJ-4, load_poll_file and load_prices_file in the polls module, part the two failures their text reading folds today. A file that does not stand, or that stands and cannot be read, draws the single defect named file cannot be read, as GJ-1 and GJ-4 state it today. A file that stands and reads as bytes that are not valid UTF-8 draws the single defect named file is not valid UTF-8, and no other defect stands beside it, and no poll and no price table stands from such a file. The parting stands at the reading of the file and before any parsing, so a file whose bytes are not valid UTF-8 draws the decode defect whatever its bytes would have parsed to, and a file whose bytes are valid UTF-8 draws no decode defect, its defects standing as the standing loaders state them. The polls module parts the two failures in its text reading for the two file loaders, and the standing folded reading read_text_file stands unchanged for its other callers, the makers module's required reading among them, whose refusal form is not a defect of GJ-1 and does not move in this round. Every command surface that loads a poll or a price table through the file loaders states the decode defect in the standing form of its surface, the line invalid poll or invalid price table standing over the line defect and the defect, and a set directory whose poll file holds bytes that are not valid UTF-8 draws the same two lines at the first such file, the set loading standing whole or nothing as GJ-12 states. The defect vocabulary of GJ-1 gains the one phrase, file is not valid UTF-8, and loses none. Test, rust/tests/test_polls.rs and rust/tests/test_cost.rs and rust/tests/test_cli.rs.\n"
    );
    let gj50 = contract_section(&text, "### GJ-50 full form, the empty section execution");
    assert!(gj50.contains("GJ-50. The empty section execution. The mutation the Round 23 record names against the twelfth pin of the shares target, a build of the shares reading that drops the section of a set holding no polls, stands executed on the Round 24 slice tree in its own fresh target directory, and its failing set stands read from assertion failures alone, a failure of the environment standing beside the set and never inside it. The set as computed by the Round 24 design stands at the twelfth and thirteenth pins of rust/tests/test_shares.rs, and every parting between the executed set and the computed set stands named in the execution record the slice carries in that target's header. The record states no posture it did not execute, and the round that carries this obligation does not close while the execution it charges stands unexecuted. Test, rust/tests/test_shares.rs."));
    assert_eq!(
        lines
            .iter()
            .filter(|line| **line == "### GJ-50 full form, the empty section execution")
            .count(),
        1
    );
    assert_eq!(
        gj50,
        "### GJ-50 full form, the empty section execution\n\nGJ-50. The empty section execution. The mutation the Round 23 record names against the twelfth pin of the shares target, a build of the shares reading that drops the section of a set holding no polls, stands executed on the Round 24 slice tree in its own fresh target directory, and its failing set stands read from assertion failures alone, a failure of the environment standing beside the set and never inside it. The set as computed by the Round 24 design stands at the twelfth and thirteenth pins of rust/tests/test_shares.rs, and every parting between the executed set and the computed set stands named in the execution record the slice carries in that target's header. The record states no posture it did not execute, and the round that carries this obligation does not close while the execution it charges stands unexecuted. Test, rust/tests/test_shares.rs.\n"
    );
    let fix24 = contract_section(
        &text,
        "### FIX-GJ-24, amending GJ-1 and GJ-4, the decode refusal",
    );
    assert!(fix24.contains("FIX-GJ-24, amending GJ-1 and GJ-4, the decode refusal. GJ-1 states the file defect in these words, A file that cannot be read returns the single defect named file cannot be read, and GJ-4 states the same defect for a file that cannot be read at the price table loader. The loader's text reading folds a failed read and a failed decode into one absence, so a poll file whose bytes are not valid UTF-8 draws the read defect today, a file that stands and cannot be read as text standing refused in the words of a file that does not stand, the folding standing in the doc comment of the text reading itself, which states it returns no text where the read fails or the bytes are not UTF-8. Under this amendment the two file loaders part the failures as GJ-49 states, a file that stands and reads as bytes that are not valid UTF-8 drawing the single defect named file is not valid UTF-8 at both seats, and the read defect standing unchanged for a file that does not stand or cannot be read. No other word of GJ-1 or GJ-4 moves under this amendment, and no figure and no golden moves, the shipped records standing valid UTF-8 whole. Its number is 24, standing reserved to this refusal in the words FIX-GJ-25 carries since Round 21, and it stands spent in this round. Test, rust/tests/test_polls.rs."));
    assert_eq!(
        lines
            .iter()
            .filter(|line| **line == "### FIX-GJ-24, amending GJ-1 and GJ-4, the decode refusal")
            .count(),
        1
    );
    assert_eq!(
        fix24,
        "### FIX-GJ-24, amending GJ-1 and GJ-4, the decode refusal\n\nFIX-GJ-24, amending GJ-1 and GJ-4, the decode refusal. GJ-1 states the file defect in these words, A file that cannot be read returns the single defect named file cannot be read, and GJ-4 states the same defect for a file that cannot be read at the price table loader. The loader's text reading folds a failed read and a failed decode into one absence, so a poll file whose bytes are not valid UTF-8 draws the read defect today, a file that stands and cannot be read as text standing refused in the words of a file that does not stand, the folding standing in the doc comment of the text reading itself, which states it returns no text where the read fails or the bytes are not UTF-8. Under this amendment the two file loaders part the failures as GJ-49 states, a file that stands and reads as bytes that are not valid UTF-8 drawing the single defect named file is not valid UTF-8 at both seats, and the read defect standing unchanged for a file that does not stand or cannot be read. No other word of GJ-1 or GJ-4 moves under this amendment, and no figure and no golden moves, the shipped records standing valid UTF-8 whole. Its number is 24, standing reserved to this refusal in the words FIX-GJ-25 carries since Round 21, and it stands spent in this round. Test, rust/tests/test_polls.rs.\n"
    );
}
/// The contract section a heading opens, standing from
/// its heading line to the next heading line of either
/// level, as the anchored contract pins read it. A
/// heading that does not stand in the contract reads
/// as no section, and the pin asserting on it fails.
fn contract_section(text: &str, heading: &str) -> String {
    let lines = lines_of(text);
    let Some(start) = lines.iter().position(|line| *line == heading) else {
        return String::new();
    };
    let mut section = Vec::new();
    for line in &lines[start..] {
        if !section.is_empty() && (line.starts_with("## ") || line.starts_with("### ")) {
            break;
        }
        section.push(*line);
    }
    section.join("\n")
}

#[test]
fn readme_reports_the_round_twenty_five_status() {
    let text = root_text("README.md");
    let lines = lines_of(&text);
    let standing = "- Contract, GJ-51 and GJ-52 met. They stand in full form in CONTRACT.md with the FIX-GJ-29 repair amendment amending GJ-49, restating the second sentence of GJ-49 without its redundant word, the phrase whose bytes replaced by the word that, a net removal of one word, and Round 25 closed on the merged slice 26 after three reviews, with GJ-51's consolidation standing as charged, the folded reading standing as the checked reading's Ok arm and the defect mapping standing in the one helper read_text_or_defect the two file loaders share, no behaviour, defect text, figure, or golden moving, and GJ-52's execution discharging the directory misreport the Round 24 design record names uncharged against the fifty fifth pin of the polls target, the mutation QC failing that pin alone, and the anchoring cure riding at this contract opening, pin 67 restated from the whole file contains form to the anchored form and pin 69 standing anchored from birth, and the first build of the round stopping at the mutation QB, its executed set standing at eight falls against the five pins the design's computation charged in the set, and the owner's ruling restating the mutation's account to the executed falls, eight in the set, the five pins of test_makers.rs, the_pilot_files_match_the_recorded_tree_byte_for_byte and the_full_files_match_the_recorded_tree_byte_for_byte and the_full_golden_follows_the_rebuild_when_an_envelope_drops and the_full_maker_runs_when_a_standing_poll_file_is_missing and the_full_golden_ignores_a_response_dropped_from_a_standing_poll, and three beside the charged five in the same form at the same seat, every_full_poll_rebuilds_from_its_envelopes and the_full_run_record_recomputes_from_the_fixtures of test_full.rs and the_wave_files_match_the_recorded_tree_byte_for_byte of test_wave.rs, each an expect panic over the maker call, the pins of the full and wave targets also calling the maker functions the design's witness enumeration did not count, the design's computation standing corrected by the execution and the ruling, the reseated build adopting the stopped lane's slice whole after verifying it blob for blob against the remote and executing QC and QD and QE on the verified trees, QD standing at the Round 24 seventeen name for name at the one helper seat, five in the set and twelve beside it, and QE standing at pin 55 and the twelve missing file seats, thirteen in all, with the spec review approving with findings at one Observation, the pin names 67 and 68 and 69 standing by the design's count convention while the pins stand at file positions 55 and 56 and 57 among their target's sixty nine pins, the standards review approving with findings at three Observations, its whole obligation assertion cure priced at a later contract opening, the red team approving with no findings, its one hundred ninety nine drives over the slice and base binaries standing at no parting at any seat and its maker probe standing identical at twenty eight lines on both trees, its spot executions reproducing QB at the adopted eight name for name and QC at pin 55 alone, and the anchoring standing proven in the stickler form, the slice's restated pin 67 failing on the GJ-49 opening sentence moved out of its section into the Round 25 preamble while the base tree's whole file pin passes over the identical contract and the slice's pin passes over its own contract as the control, and CodeRabbit returning no inline comments on the slice, its completed review standing read by the owner at the merge gate at zero inline comments and zero submitted reviews, and the red team's one limitation standing as a record, the permission unreadable input class standing not executed as a permission failure in this sandbox, its processes running as root and root reading through a file of mode 000 and no unprivileged reader standing constructible on the overlay, the bounding argument standing that a permission failure enters through the same standard read error arm the checked reading maps to the read defect, the identical arm the missing file and directory drives execute at every seat, and FIX-GJ-30 standing free and unspent.";
    assert!(lines.contains(&standing));
    let slice_line = "- Slice 26 has landed. It carries the polls module consolidation, the folded reading standing as the checked reading's Ok arm and the defect mapping standing in one helper the two file loaders share, and the directory misreport execution, and the repair amendment FIX-GJ-29 restating the second sentence of GJ-49 without its redundant word, and the anchoring cure at the contract carrying pins, adding no reading and no command.";
    assert!(lines.contains(&slice_line));
    assert!(text.contains("Closed perfection rounds, 26 of 100"));
}
#[test]
fn contract_carries_the_round_twenty_five_obligations() {
    let text = root_text("CONTRACT.md");
    let lines = lines_of(&text);
    let gj51 = contract_section(&text, "### GJ-51 full form, the polls module consolidation");
    assert!(gj51.contains("GJ-51. The polls module consolidation. The polls module stands consolidated in the shape GJ-49's landing left duplicated. The folded reading read_text_file stands as the Ok arm of the checked reading read_text_file_checked, its result at every path standing identical to the result its own body returned, no text where the read fails and no text where the bytes are not valid UTF-8 and the text itself where both succeed, so its other callers, the makers module's required reading among them, stand served exactly as GJ-49 states them, their refusal form standing unmoved. The defect mapping of the two file loaders stands in one helper of the module, read_text_or_defect, which states the defect of the failure the checked reading reports, the read defect for a file that does not stand or cannot be read and the decode defect for a file whose bytes are not valid UTF-8, and load_poll_file and load_prices_file each state the defect the helper returns and no other, so the two defect texts of GJ-49 stand in one place in the crate and a restatement of either stands or falls at both loader seats together. No behaviour of any loader, reading, or command moves under the consolidation, no defect text moves, no figure and no golden moves, and the checked reading and its two failures stand as GJ-49 landed them. Test, rust/tests/test_polls.rs and rust/tests/test_cost.rs and rust/tests/test_cli.rs and rust/tests/test_makers.rs."));
    assert_eq!(
        lines
            .iter()
            .filter(|line| **line == "### GJ-51 full form, the polls module consolidation")
            .count(),
        1
    );
    assert_eq!(
        gj51,
        "### GJ-51 full form, the polls module consolidation\n\nGJ-51. The polls module consolidation. The polls module stands consolidated in the shape GJ-49's landing left duplicated. The folded reading read_text_file stands as the Ok arm of the checked reading read_text_file_checked, its result at every path standing identical to the result its own body returned, no text where the read fails and no text where the bytes are not valid UTF-8 and the text itself where both succeed, so its other callers, the makers module's required reading among them, stand served exactly as GJ-49 states them, their refusal form standing unmoved. The defect mapping of the two file loaders stands in one helper of the module, read_text_or_defect, which states the defect of the failure the checked reading reports, the read defect for a file that does not stand or cannot be read and the decode defect for a file whose bytes are not valid UTF-8, and load_poll_file and load_prices_file each state the defect the helper returns and no other, so the two defect texts of GJ-49 stand in one place in the crate and a restatement of either stands or falls at both loader seats together. No behaviour of any loader, reading, or command moves under the consolidation, no defect text moves, no figure and no golden moves, and the checked reading and its two failures stand as GJ-49 landed them. Test, rust/tests/test_polls.rs and rust/tests/test_cost.rs and rust/tests/test_cli.rs and rust/tests/test_makers.rs.\n"
    );
    let gj52 = contract_section(
        &text,
        "### GJ-52 full form, the directory misreport execution",
    );
    assert!(gj52.contains("GJ-52. The directory misreport execution. The failure mode the Round 24 design record names against the fifty fifth pin of the polls target beyond its charged mutations, a checked reading that reports the failed read of a path that stands as the decode failure, stands executed on the Round 25 slice tree in its own fresh target directory, and its failing set stands read from assertion failures alone, a failure of the environment standing beside the set and never inside it. The set as computed by the Round 25 design stands at the fifty fifth pin of rust/tests/test_polls.rs alone, and every parting between the executed set and the computed set stands named in the execution record the slice carries in that target over the pins of GJ-49. The record states no posture it did not execute, and the round that carries this obligation does not close while the execution it charges stands unexecuted. Test, rust/tests/test_polls.rs."));
    assert_eq!(
        lines
            .iter()
            .filter(|line| **line == "### GJ-52 full form, the directory misreport execution")
            .count(),
        1
    );
    assert_eq!(
        gj52,
        "### GJ-52 full form, the directory misreport execution\n\nGJ-52. The directory misreport execution. The failure mode the Round 24 design record names against the fifty fifth pin of the polls target beyond its charged mutations, a checked reading that reports the failed read of a path that stands as the decode failure, stands executed on the Round 25 slice tree in its own fresh target directory, and its failing set stands read from assertion failures alone, a failure of the environment standing beside the set and never inside it. The set as computed by the Round 25 design stands at the fifty fifth pin of rust/tests/test_polls.rs alone, and every parting between the executed set and the computed set stands named in the execution record the slice carries in that target over the pins of GJ-49. The record states no posture it did not execute, and the round that carries this obligation does not close while the execution it charges stands unexecuted. Test, rust/tests/test_polls.rs.\n"
    );
    let fix29 = contract_section(
        &text,
        "### FIX-GJ-29, amending GJ-49, the one redundant word",
    );
    assert!(fix29.contains("FIX-GJ-29, amending GJ-49, the one redundant word. The second sentence of the decode refusal's parting in GJ-49 states the standing file in these words, A file that stands and reads as bytes whose bytes are not valid UTF-8 draws the single defect named file is not valid UTF-8, the word bytes standing twice in one breath where FIX-GJ-24's own statement of the same file carries the clean form, a file that stands and reads as bytes that are not valid UTF-8 drawing the same defect. Under this amendment that sentence stands restated in GJ-49's own section in these words, A file that stands and reads as bytes that are not valid UTF-8 draws the single defect named file is not valid UTF-8, the phrase whose bytes standing replaced by that, a net removal of one word, and no other word of GJ-49 moves under this amendment, and no figure and no behaviour and no defect text moves, the sentence naming the decode defect in the words it named it. Its number is 29, standing free since the Round 24 section landed, and it stands spent in this round. Test, rust/tests/test_setup.rs."));
    assert_eq!(
        lines
            .iter()
            .filter(|line| **line == "### FIX-GJ-29, amending GJ-49, the one redundant word")
            .count(),
        1
    );
    assert_eq!(
        fix29,
        "### FIX-GJ-29, amending GJ-49, the one redundant word\n\nFIX-GJ-29, amending GJ-49, the one redundant word. The second sentence of the decode refusal's parting in GJ-49 states the standing file in these words, A file that stands and reads as bytes whose bytes are not valid UTF-8 draws the single defect named file is not valid UTF-8, the word bytes standing twice in one breath where FIX-GJ-24's own statement of the same file carries the clean form, a file that stands and reads as bytes that are not valid UTF-8 drawing the same defect. Under this amendment that sentence stands restated in GJ-49's own section in these words, A file that stands and reads as bytes that are not valid UTF-8 draws the single defect named file is not valid UTF-8, the phrase whose bytes standing replaced by that, a net removal of one word, and no other word of GJ-49 moves under this amendment, and no figure and no behaviour and no defect text moves, the sentence naming the decode defect in the words it named it. Its number is 29, standing free since the Round 24 section landed, and it stands spent in this round. Test, rust/tests/test_setup.rs.\n"
    );
}

#[test]
fn readme_reports_the_round_twenty_six_status() {
    let text = root_text("README.md");
    let lines = lines_of(&text);
    let standing = "- Contract, GJ-53 and GJ-54 met. They stand in full form in CONTRACT.md, GJ-53 standing as the whole obligation assertion cure and GJ-54 as the body cut execution beside it, and Round 26 closed on the merged slice 27 after three reviews, with the cure standing landed as charged, the pins of Rounds 24 and 25 restated to assert each of GJ-49 and GJ-50 and FIX-GJ-24 and GJ-51 and GJ-52 and FIX-GJ-29 whole inside its own section, and pin 71 standing in the whole text form over GJ-53 and GJ-54 from its birth, and the body cut execution of GJ-54 standing executed at design time at the eight seats and re-executed whole by the build name for name, each cut falling at the one contract carrying pin of its own round alone on the form, the cuts at GJ-49 and GJ-50 and FIX-GJ-24 at the Round 24 pin and the cuts at GJ-51 and GJ-52 and FIX-GJ-29 at the Round 25 pin and the cuts at GJ-53 and GJ-54 at the Round 26 pin, all six cuts of Rounds 24 and 25 standing unseen on the base tree at 816 passed and 0 failed, the radius control at GJ-48 standing whole at 818 and 0, and the relocation control at GJ-49 falling at the Round 24 pin alone, and the build's warm up overlay standing at 816 and 2, its falls at pins 70 and 71 alone and the restated pins 67 and 69 passing on the base tree by design, and the redundancy class of GJ-49 standing surveyed whole with no second seat, its doubled phrase surviving only inside FIX-GJ-29's own quotation, with the spec review approving with findings at one Observation, the pin names 67 and 69 and 70 and 71 standing by the design's count convention while the pins stand at file positions 55 and 57 and 58 and 59 among their target's seventy one pins, the standards review approving with findings at two Observations, the wrap of the records standing named at the call pair's grain where the committed text wraps one call of pin 71's pair, and the same pin numbering grain standing as the record leaves it, its own spot execution standing at the setup target green on the uncut head and falling at the Round 24 pin alone over its own GJ-49 body cut while the base tree's landed pin passes over the same cut, the red team approving with findings at one Minor, the whole obligation assertion standing as a containment claim over the span the first standing of a heading opens, and a contract whose GJ-49 paragraph stands cut in place passing the whole suite at 818 and 0 under its two executed constructions, the whole paragraph standing appended inside GJ-49's own section behind the cut paragraph, and the whole paragraph standing planted under a duplicate of GJ-49's heading line in the Round 24 preamble, no pin asserting that the paragraph stands whole in its place or that the whole text stands once or that the heading stands once, the Minor standing cured in the slice by the repair commit 095ad60a, the repair adding inside each of the three section bounding pins the assertion that each guarded heading stands once in the contract and the assertion that the section the heading opens carries the heading and the whole text and nothing else, and the record block over the Round 24 pin standing restated at its assertion description, the cure standing proven in both directions by execution, both constructions passing the unrepaired head whole at 818 and 0 and falling at the Round 24 pin alone on the repaired tree, and the body cuts at GJ-49 and GJ-53 re-executed on the repaired tree standing name for name at their own pins alone, the red team's sentence move and neighbour duplication legs falling at the Round 24 pin and its phrase exchange leg falling at the Round 24 and Round 25 pins and its relocation control in the other direction falling at the Round 24 pin, and CodeRabbit standing read in full and complete on the slice at zero inline comments and zero submitted reviews, its summary stating no actionable comments, and the slice standing at census 818, and no item of this round standing carried as executed that was not executed, and FIX-GJ-30 standing free and unspent.";
    assert!(lines.contains(&standing));
    let slice_line = "- Slice 27 has landed. It carries the whole obligation assertion cure at the contract carrying pins, the pins of Rounds 24 and 25 restated to assert each obligation whole inside its own section and this round's own pin standing in the whole text form from its birth, and the body cut execution against those pins at the eight seats the cure reaches, adding no reading and no command.";
    assert!(lines.contains(&slice_line));
    assert!(text.contains("Closed perfection rounds, 26 of 100"));
}

#[test]
fn contract_carries_the_round_twenty_six_obligations() {
    let text = root_text("CONTRACT.md");
    let lines = lines_of(&text);
    let gj53 = contract_section(
        &text,
        "### GJ-53 full form, the whole obligation assertion cure",
    );
    assert!(gj53.contains("GJ-53. The whole obligation assertion cure. The contract carrying pins of Round 24 and Round 25 in rust/tests/test_setup.rs, contract_carries_the_round_twenty_four_obligations and contract_carries_the_round_twenty_five_obligations, stand restated so that each asserts its obligations whole inside the sections their headings open. Each pin takes its section in the anchored form the Round 25 slice landed and asserts that the section carries the obligation's whole text, the opening words and the body and the Test line standing together inside the section, where the pins as landed assert the identifier and the opening words inside the section and pass on a contract that carries the heading and the opening words with the obligation's body cut away. The cure reaches the six texts the two pins assert, GJ-49 and GJ-50 and FIX-GJ-24 under the Round 24 pin and GJ-51 and GJ-52 and FIX-GJ-29 under the Round 25 pin, and the contract carrying pin of this round asserts GJ-53 and GJ-54 in the same whole text form from its birth. No word of any obligation moves under the cure, and no behaviour and no figure and no golden moves, the cure standing test only in the setup target. The contract carrying pins of Rounds 1 to 23 stand in the presence form as they landed and do not move in this round. Test, rust/tests/test_setup.rs."));
    assert_eq!(
        lines
            .iter()
            .filter(|line| **line == "### GJ-53 full form, the whole obligation assertion cure")
            .count(),
        1
    );
    assert_eq!(
        gj53,
        "### GJ-53 full form, the whole obligation assertion cure\n\nGJ-53. The whole obligation assertion cure. The contract carrying pins of Round 24 and Round 25 in rust/tests/test_setup.rs, contract_carries_the_round_twenty_four_obligations and contract_carries_the_round_twenty_five_obligations, stand restated so that each asserts its obligations whole inside the sections their headings open. Each pin takes its section in the anchored form the Round 25 slice landed and asserts that the section carries the obligation's whole text, the opening words and the body and the Test line standing together inside the section, where the pins as landed assert the identifier and the opening words inside the section and pass on a contract that carries the heading and the opening words with the obligation's body cut away. The cure reaches the six texts the two pins assert, GJ-49 and GJ-50 and FIX-GJ-24 under the Round 24 pin and GJ-51 and GJ-52 and FIX-GJ-29 under the Round 25 pin, and the contract carrying pin of this round asserts GJ-53 and GJ-54 in the same whole text form from its birth. No word of any obligation moves under the cure, and no behaviour and no figure and no golden moves, the cure standing test only in the setup target. The contract carrying pins of Rounds 1 to 23 stand in the presence form as they landed and do not move in this round. Test, rust/tests/test_setup.rs.\n"
    );
    let gj54 = contract_section(&text, "### GJ-54 full form, the body cut execution");
    assert!(gj54.contains("GJ-54. The body cut execution. The failure mode the Round 25 standards review names against the contract carrying pins of Rounds 24 and 25, a contract carrying an obligation's heading and its opening words with the obligation's body cut away and passing both pins as landed, stands executed on the Round 26 slice tree in its own fresh target directory at each seat the cure of GJ-53 reaches, the six texts of Rounds 24 and 25 and the two obligations of this round, and its failing set stands read from assertion failures alone, a failure of the environment standing beside the set and never inside it. The set as computed by the Round 26 design stands at the one contract carrying pin of the cut obligation's own round at each seat, and every parting between the executed set and the computed set stands named in the execution record the slice carries in the setup target over the pins the cure restates. The record states no posture it did not execute, and the round that carries this obligation does not close while the execution it charges stands unexecuted. Test, rust/tests/test_setup.rs."));
    assert_eq!(
        lines
            .iter()
            .filter(|line| **line == "### GJ-54 full form, the body cut execution")
            .count(),
        1
    );
    assert_eq!(
        gj54,
        "### GJ-54 full form, the body cut execution\n\nGJ-54. The body cut execution. The failure mode the Round 25 standards review names against the contract carrying pins of Rounds 24 and 25, a contract carrying an obligation's heading and its opening words with the obligation's body cut away and passing both pins as landed, stands executed on the Round 26 slice tree in its own fresh target directory at each seat the cure of GJ-53 reaches, the six texts of Rounds 24 and 25 and the two obligations of this round, and its failing set stands read from assertion failures alone, a failure of the environment standing beside the set and never inside it. The set as computed by the Round 26 design stands at the one contract carrying pin of the cut obligation's own round at each seat, and every parting between the executed set and the computed set stands named in the execution record the slice carries in the setup target over the pins the cure restates. The record states no posture it did not execute, and the round that carries this obligation does not close while the execution it charges stands unexecuted. Test, rust/tests/test_setup.rs.\n"
    );
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

fn crate_sources() -> Vec<(String, String)> {
    let mut found = Vec::new();
    let mut stack = vec![repo_root().join("rust/src")];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("the source tree") {
            let entry = entry.expect("a source tree entry");
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                let name = path
                    .strip_prefix(repo_root())
                    .expect("a source under the root")
                    .to_string_lossy()
                    .into_owned();
                found.push((name, read_text(&path)));
            }
        }
    }
    found
}

#[test]
fn the_plurality_rule_stands_once_in_the_agreement_module() {
    // The layout pin of the Round 16 fold, in the
    // posture of the FIX-GJ-20 pin above. It holds
    // layout, not semantics, and this comment states
    // that limit plainly, as FIX-GJ-20 states its own.
    // The pin fails at the Round 15 closure tree by
    // assertion, plurality_of standing defined in
    // canvass.rs and in roll.rs and the aggregate
    // holding no call to it. It fails under the
    // mutation that leaves a consumer copy standing
    // while agreement gains its own, the fold half
    // done, and under the mutation that leaves the
    // aggregate's inline standing, its call assertion
    // failing. No behavioural mutation moves it, and
    // no behavioural pin discriminates the fold: the
    // drift of the shared body's behaviour is held by
    // the behavioural pins, pins 3, 19, and 20 of
    // test_roll.rs first among them.
    let sources = crate_sources();
    let defining: Vec<&str> = sources
        .iter()
        .filter(|(_, text)| text.contains("fn plurality_of("))
        .map(|(name, _)| name.as_str())
        .collect();
    assert_eq!(defining, vec!["rust/src/agreement.rs"]);
    let aggregate = root_text("rust/src/aggregate.rs");
    assert!(aggregate.contains("plurality_of(&summed, valid, &union)"));
}

#[test]
fn add_reading_stands_once_as_a_free_function_in_the_agreement_module() {
    // The second layout pin of the Round 16 fold, with
    // the same stated limit as the pin above. It fails
    // at the Round 15 closure tree by assertion, the
    // free function standing defined in canvass.rs and
    // in roll.rs, and it fails under the mutation that
    // leaves either consumer's copy standing. The pin
    // does not read rust/src/record.rs, whose Counts
    // method of the same name is a different item, the
    // four counts of one reading added into an
    // accumulator and nothing more, and no reader is to
    // mistake that method for a surviving copy.
    assert!(root_text("rust/src/agreement.rs").contains("pub(crate) fn add_reading("));
    for module in ["rust/src/canvass.rs", "rust/src/roll.rs"] {
        assert!(!root_text(module).contains("fn add_reading("), "{}", module);
    }
}

#[test]
fn fix_gj_21_names_the_models_of_the_generated_roll() {
    // The pin FIX-GJ-21 states. It fails at the Round
    // 15 closure tree by assertion, the completed
    // sentences standing nowhere in the contract and
    // the ordinal forms standing in it, and it fails
    // under the mutation that applies the completion
    // to the shipped record paragraph of GJ-32 instead
    // of the generated one, and under the mutation
    // that names the models in the standing lines
    // sentence alone. No behavioural pin can
    // discriminate the repair, the statement it
    // completes being prose over figures that do not
    // move, and the amendment states that plainly
    // rather than dress this pin as one.
    let text = root_text("CONTRACT.md");
    assert!(text.contains("twenty seven questions and twenty four pluralities and twenty with the plurality for model a, thirty four questions and twenty six pluralities and twenty one with the plurality for model b, and thirty questions and twenty three pluralities and nineteen with the plurality for model c"));
    for sentence in [
        "Forty one of the blocks are blocks of a jury that stands at fewer askings than its question holds, fifteen of them under model a and twelve under model b and fourteen under model c.",
        "Eleven blocks hold a standing plurality that differs from the plurality of their question, three under model a and five under model b and three under model c.",
        "Three blocks hold a plurality at some asking of their question and none pooled, their summed top counts tied, one under model b and two under model c.",
        "Two blocks hold a plurality at a question whose pooled plurality stands at none, one under model a and one under model c.",
    ] {
        assert!(text.contains(sentence), "{}", sentence);
    }
    for form in [
        "for the first model",
        "under the first model",
        "under the second model",
    ] {
        assert!(!text.contains(form), "{}", form);
    }
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
