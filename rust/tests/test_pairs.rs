//! The pairing reading, GJ-43 and GJ-44, twenty pins
//! restated from first principles.
//!
//! The reading is recomputed from the shipped sets
//! and from the generated family by an independent
//! walk in this file, the direct computation below,
//! which counts the responses itself and gathers
//! the record by set pair without the library's
//! reading machinery, and the library walk and the
//! printed surface are compared against it line for
//! line, so a defect in the library walk fails here
//! even where a figure the closing relations reach
//! would let it pass. The parts of the reading stand
//! compared against the accord reading part by part,
//! the regrouping relation GJ-44 states. The shapes
//! stand in the forms GJ-44 states them, S1 to S8,
//! each on a fixture small enough to read whole.
//! Against the Round 20 closure tree this target
//! does not compile, the pairs module standing
//! absent there, and that failure is the base run
//! the round records.
//!
//! The discrimination of the pins is stated against
//! the mutation set of the Round 21 design, PA to
//! PT, its postures computed by the validated
//! transcription of the design or imported from the
//! earlier rounds' executions, and no posture is
//! stated the computations do not carry. PA, the
//! pairing of a part read against the pooled
//! plurality of its question, fails pins 4, 7, 18,
//! 19, and 20, pin 7 by construction, and moves no
//! shipped line. PB, the agreed and divided counts
//! swapped in the pairing counting, fails pins 1,
//! 2, 4, 6, 7, 18, 19, and 20, pins 6 and 7 by
//! construction. PC, a tied part counted as holding
//! the first answer at its tied top count, fails
//! pins 4, 8, 18, 19, and 20, pin 8 by construction.
//! PD, a part at which neither part holds omitted
//! from its section, fails pins 1, 2, 4, 9, 18, 19,
//! and 20, pin 9 by construction. PE, the shared
//! questions gathered from the first set's askings
//! alone, fails pins 1, 2, 3, 4, 5, 11, 18, 19, and
//! 20, pin 11 by construction. PF, the second set's
//! part gathered from the first set's askings, fails
//! pins 3, 4, 10, 18, 19, and 20, pin 10 by
//! construction and pin 3 by the shipped margin
//! movement. PG, the pairs stated with the later set
//! first, fails pins 1, 2, and 19, and the figure
//! pins and the relation pins stand. PH, the head
//! order of pairs_lines swapped, fails no pin, and
//! pins 13 and 14 stand. PI, the guard read over the
//! first set's polls alone, fails pin 13 and pin 14
//! stands. PJ, the duplicate label block of
//! validate_sets deleted, fails pin 15 on the
//! duplicate fixture. PK, the closing word pairs
//! restated as a refused word, fails pin 16 with a
//! wide radius over the line comparing pins, pins
//! 1, 2, 4, 18, 19, and 20 among them, and the
//! status pins stand. PL, the help entry deleted
//! with the dispatch wired, fails pin 17 alone. PM,
//! the dispatch restated to run the tally handler
//! on its first value, fails pin 23 in
//! rust/tests/test_cli.rs alone, and pins 21 and 22
//! stand. PN, the generated family of this file
//! restated with the Round 13 fault, fails pin 18,
//! and its radius over pins 4, 19, and 20 stands to
//! be read from the executed set. PO, the closing
//! computed from the shipped record's figures, fails
//! pins 4, 18, 19, and 20 over the generated record,
//! and pin 5 stands, its legs reading the section
//! sums and the accord recount and not the printed
//! closing. PP, the README Round 21 status line
//! restated, fails pin 24 in
//! rust/tests/test_setup.rs alone, and pin 25
//! stands. PQ, a section's standing line computed
//! from the first shared question alone, fails pins
//! 1, 2, 4, 5, 12, 18, 19, and 20, pin 12 by
//! construction. PR, a section's standing line
//! stating every question of the record, fails pins
//! 1, 2, 4, 5, 11, 18, 19, and 20, pin 11 by
//! construction of its share. PS, the closing's
//! agreed figure counting the questions at which an
//! accord stands, fails pins 1, 2, 4, 18, 19, and
//! 20, and pins 3 and 5 stand, pin 5 by the
//! construction its legs state. PT, the per set
//! question loop of validate_sets moved ahead of
//! the label checks, fails pin 15 on the mixed
//! fixture.

mod shared;

use shared::*;

use grandjury::accord::accord_lines;
use grandjury::census::census_lines;
use grandjury::pairs::pairs_lines;
use grandjury::polls::{load_poll_file, Poll, Response};
use grandjury::pytext::PText;

fn response(juror: &str, model: &str, answer: Option<&str>, reason: &str) -> Response {
    match answer {
        Some(answer) => Response {
            juror: pt(juror),
            model: pt(model),
            answer: pt(answer),
            reason: pt(reason),
            abstained: false,
        },
        None => Response {
            juror: pt(juror),
            model: pt(model),
            answer: pt(""),
            reason: pt("no answer recorded"),
            abstained: true,
        },
    }
}

fn poll(question: &str, answers: &[&str], responses: Vec<Response>) -> Poll {
    Poll {
        question: pt(question),
        answers: answers.iter().map(|answer| pt(answer)).collect(),
        responses,
    }
}

fn votes_of(jury: &str, answer: &str, count: usize) -> Vec<Response> {
    (1..=count)
        .map(|seat| {
            response(
                &format!("{} hand {} {}", jury, answer, seat),
                "model a",
                Some(answer),
                "the record states it",
            )
        })
        .collect()
}

fn load_poll(path: &std::path::Path) -> Poll {
    let (poll, defects) = load_poll_file(path);
    assert!(defects.is_empty(), "defects in {:?}: {:?}", path, defects);
    poll.unwrap()
}

fn load_set(dir: &std::path::Path, count: usize) -> Vec<Poll> {
    (1..=count)
        .map(|index| load_poll(&dir.join(format!("poll-{:02}.json", index))))
        .collect()
}

fn recorded_sets() -> Vec<(PText, Vec<Poll>)> {
    let root = examples();
    vec![
        (pt("pilot"), load_set(&root.join("pilot"), 12)),
        (pt("full"), load_set(&root.join("full"), 40)),
        (pt("wave"), load_set(&root.join("wave"), 12)),
    ]
}

fn figure_before(line: &str, word: &str) -> i64 {
    line.split(", ")
        .find_map(|part| {
            let (figure, rest) = part.split_once(' ')?;
            if rest == word {
                figure.parse::<i64>().ok()
            } else {
                None
            }
        })
        .unwrap_or_else(|| panic!("no figure before {} in {}", word, line))
}

fn questions_of(sets: &[(PText, Vec<Poll>)]) -> Vec<PText> {
    let mut questions: Vec<PText> = Vec::new();
    for (_, polls) in sets {
        for poll in polls {
            if !questions.contains(&poll.question) {
                questions.push(poll.question.clone());
            }
        }
    }
    questions
}

fn askings_of<'a>(sets: &'a [(PText, Vec<Poll>)], question: &PText) -> Vec<(usize, &'a Poll)> {
    let mut askings: Vec<(usize, &Poll)> = Vec::new();
    for (set_index, (_, polls)) in sets.iter().enumerate() {
        for poll in polls {
            if &poll.question == question {
                askings.push((set_index, poll));
            }
        }
    }
    askings
}

fn direct_plurality(counts: &[i64], valid: i64, union: &[PText]) -> Option<PText> {
    if valid <= 0 {
        return None;
    }
    let largest = counts.iter().copied().max().unwrap_or(0);
    if counts.iter().filter(|count| **count == largest).count() != 1 {
        return None;
    }
    let index = counts.iter().position(|count| *count == largest).unwrap();
    Some(union[index].clone())
}

/// The part of one set at one question counted
/// from the responses themselves, its set line, its
/// plurality line and its margin line in the form
/// of the accord, and the plurality the part holds,
/// the direct form of the part of GJ-37.
fn direct_part_lines(label: &PText, polls: &[&Poll]) -> (Vec<String>, Option<PText>) {
    let mut union: Vec<PText> = Vec::new();
    for poll in polls {
        for answer in &poll.answers {
            if !union.contains(answer) {
                union.push(answer.clone());
            }
        }
    }
    let mut counts = vec![0i64; union.len()];
    let mut recorded = 0i64;
    let mut valid = 0i64;
    let mut abstained = 0i64;
    let mut spoiled = 0i64;
    for poll in polls {
        for response in &poll.responses {
            recorded += 1;
            if response.abstained {
                abstained += 1;
            } else if poll.answers.contains(&response.answer) {
                let slot = union
                    .iter()
                    .position(|answer| answer == &response.answer)
                    .unwrap();
                counts[slot] += 1;
                valid += 1;
            } else {
                spoiled += 1;
            }
        }
    }
    let plurality = direct_plurality(&counts, valid, &union);
    let margin_line = match &plurality {
        Some(answer) => {
            let standing = counts[union.iter().position(|held| held == answer).unwrap()];
            let largest_other = counts
                .iter()
                .zip(union.iter())
                .filter(|(_, held)| *held != answer)
                .map(|(count, _)| *count)
                .max()
                .unwrap_or(0);
            format!("margin, {}", standing - largest_other)
        }
        None => "margin, not counted".to_string(),
    };
    let lines = vec![
        format!(
            "set, {}, recorded {}, valid {}, abstained {}, spoiled {}",
            label.display(),
            recorded,
            valid,
            abstained,
            spoiled
        ),
        format!(
            "plurality, {}",
            match &plurality {
                Some(answer) => answer.display(),
                None => "none".to_string(),
            }
        ),
        margin_line,
    ];
    (lines, plurality)
}

/// The pairing reading recomputed from first
/// principles, the transcription the library walk
/// and the printed surface are compared against.
fn direct_pairs_lines(sets: &[(PText, Vec<Poll>)]) -> Vec<String> {
    let questions = questions_of(sets);
    let mut lines: Vec<String> = Vec::new();
    let mut totals = [0i64; 5];
    let mut pair_count = 0i64;
    for first in 0..sets.len() {
        for second in first + 1..sets.len() {
            pair_count += 1;
            lines.push(format!("pair, {}", sets[first].0.display()));
            lines.push(format!("pair, {}", sets[second].0.display()));
            let mut standing = [0i64; 5];
            for question in &questions {
                let askings = askings_of(sets, question);
                let first_polls: Vec<&Poll> = askings
                    .iter()
                    .filter(|(index, _)| *index == first)
                    .map(|(_, poll)| *poll)
                    .collect();
                if first_polls.is_empty() {
                    continue;
                }
                let second_polls: Vec<&Poll> = askings
                    .iter()
                    .filter(|(index, _)| *index == second)
                    .map(|(_, poll)| *poll)
                    .collect();
                if second_polls.is_empty() {
                    continue;
                }
                standing[0] += 1;
                lines.push(format!("question, {}", question.display()));
                let (first_lines, first_plurality) =
                    direct_part_lines(&sets[first].0, &first_polls);
                lines.extend(first_lines);
                let (second_lines, second_plurality) =
                    direct_part_lines(&sets[second].0, &second_polls);
                lines.extend(second_lines);
                let pairing = match (&first_plurality, &second_plurality) {
                    (Some(one), Some(two)) => {
                        if one == two {
                            standing[1] += 1;
                            "agreed"
                        } else {
                            standing[2] += 1;
                            "divided"
                        }
                    }
                    (Some(_), None) | (None, Some(_)) => {
                        standing[3] += 1;
                        "one holds"
                    }
                    (None, None) => {
                        standing[4] += 1;
                        "neither holds"
                    }
                };
                lines.push(format!("pairing, {}", pairing));
            }
            lines.push(format!(
                "standing, {} shared, {} agreed, {} divided, {} one holds, {} neither holds",
                standing[0], standing[1], standing[2], standing[3], standing[4]
            ));
            for (slot, figure) in standing.iter().enumerate() {
                totals[slot] += figure;
            }
        }
    }
    lines.push(format!(
        "pairs, {} pairs, {} shared, {} agreed, {} divided, {} one holds, {} neither holds",
        pair_count, totals[0], totals[1], totals[2], totals[3], totals[4]
    ));
    lines
}

type ParsedSection = (Vec<String>, usize, [i64; 5]);

/// The sections of a pairing reading and its
/// closing figures, parsed off the lines. A section
/// carries its two pair labels, the count of its
/// parts, and its standing figures as shared and
/// agreed and divided and one holds and neither
/// holds.
fn parse_pairs(lines: &[String]) -> (Vec<ParsedSection>, [i64; 6]) {
    let mut sections: Vec<ParsedSection> = Vec::new();
    let mut closing = [0i64; 6];
    let mut labels: Vec<String> = Vec::new();
    let mut parts = 0usize;
    for line in lines {
        if let Some(label) = line.strip_prefix("pair, ") {
            labels.push(label.to_string());
        } else if line.starts_with("question, ") {
            parts += 1;
        } else if line.starts_with("standing, ") {
            let standing = [
                figure_before(line, "shared"),
                figure_before(line, "agreed"),
                figure_before(line, "divided"),
                figure_before(line, "one holds"),
                figure_before(line, "neither holds"),
            ];
            sections.push((std::mem::take(&mut labels), parts, standing));
            parts = 0;
        } else if line.starts_with("pairs, ") {
            closing = [
                figure_before(line, "pairs"),
                figure_before(line, "shared"),
                figure_before(line, "agreed"),
                figure_before(line, "divided"),
                figure_before(line, "one holds"),
                figure_before(line, "neither holds"),
            ];
        }
    }
    (sections, closing)
}

/// The sums GJ-44 states. The closing's figures
/// stand at the sums of the sections on every
/// figure, the closing's pairs figure stands at the
/// count of the sections, and each section's shared
/// figure stands at the count of its parts.
fn assert_pairs_sums(lines: &[String]) {
    let (sections, closing) = parse_pairs(lines);
    assert_eq!(closing[0], sections.len() as i64);
    let mut totals = [0i64; 5];
    for (_, parts, standing) in &sections {
        assert_eq!(standing[0], *parts as i64);
        for (slot, figure) in standing.iter().enumerate() {
            totals[slot] += figure;
        }
    }
    for slot in 0..5 {
        assert_eq!(closing[slot + 1], totals[slot]);
    }
}

/// The shared and agreed figures of a pairing
/// reading recomputed from the accord reading over
/// the same sets, the shared figure as the count,
/// over the questions of the accord, of the pairs
/// among the sets asking each question, and the
/// agreed figure as the count of the pair parts
/// naming one answer.
fn accord_recount(sets: &[(PText, Vec<Poll>)]) -> (i64, i64) {
    let accord = accord_lines(sets).unwrap();
    let mut shared = 0i64;
    let mut agreed = 0i64;
    let mut pluralities: Vec<Option<String>> = Vec::new();
    for line in &accord {
        if line.starts_with("question, ") {
            pluralities = Vec::new();
        } else if let Some(word) = line.strip_prefix("plurality, ") {
            pluralities.push(if word == "none" {
                None
            } else {
                Some(word.to_string())
            });
        } else if line.starts_with("accord, ") {
            let asking = pluralities.len() as i64;
            shared += asking * (asking - 1) / 2;
            for first in 0..pluralities.len() {
                for second in first + 1..pluralities.len() {
                    if let (Some(one), Some(two)) = (&pluralities[first], &pluralities[second]) {
                        if one == two {
                            agreed += 1;
                        }
                    }
                }
            }
        }
    }
    (shared, agreed)
}

fn expected(lines: &[&str]) -> Vec<String> {
    lines.iter().map(|line| line.to_string()).collect()
}

#[test]
fn pairs_golden_matches_the_reading_over_the_shipped_sets() {
    let golden = read_text(&examples().join("pairs-golden.txt"));
    let golden_lines: Vec<&str> = golden.lines().collect();
    assert_eq!(golden_lines.len(), 106);
    assert_eq!(
        golden_lines[105],
        "pairs, 3 pairs, 12 shared, 11 agreed, 0 divided, 0 one holds, 1 neither holds"
    );
    let pilot = path_arg(&examples().join("pilot"));
    let full = path_arg(&examples().join("full"));
    let wave = path_arg(&examples().join("wave"));
    let (stdout, _, code) = run_cli(&["pairs", "pilot", &pilot, "full", &full, "wave", &wave]);
    assert_eq!(code, 0);
    assert_eq!(stdout, golden);
}

#[test]
fn direct_computation_matches_the_pairs_reading() {
    let sets = recorded_sets();
    let lines = pairs_lines(&sets).unwrap();
    assert_eq!(lines, direct_pairs_lines(&sets));
    let pilot = path_arg(&examples().join("pilot"));
    let full = path_arg(&examples().join("full"));
    let wave = path_arg(&examples().join("wave"));
    let (stdout, _, code) = run_cli(&["pairs", "pilot", &pilot, "full", &full, "wave", &wave]);
    assert_eq!(code, 0);
    let printed: Vec<String> = stdout.lines().map(|line| line.to_string()).collect();
    assert_eq!(printed, lines);
}

#[test]
fn parts_stand_equal_to_the_accord_parts_over_the_shipped_sets() {
    let sets = recorded_sets();
    let lines = pairs_lines(&sets).unwrap();
    let accord = accord_lines(&sets).unwrap();
    // The accord parts keyed by question and set
    // label, each standing as its set line, its
    // plurality line and its margin line.
    let mut accord_parts: Vec<(String, String, Vec<String>)> = Vec::new();
    let mut question = String::new();
    let mut triple: Vec<String> = Vec::new();
    for line in &accord {
        if let Some(name) = line.strip_prefix("question, ") {
            question = name.to_string();
        } else if line.starts_with("set, ") {
            triple = vec![line.clone()];
        } else if line.starts_with("plurality, ") || line.starts_with("margin, ") {
            triple.push(line.clone());
            if triple.len() == 3 {
                let label = triple[0]
                    .split(", ")
                    .nth(1)
                    .expect("a set line label")
                    .to_string();
                accord_parts.push((question.clone(), label, triple.clone()));
            }
        }
    }
    let mut compared = 0usize;
    let mut question = String::new();
    let mut triple: Vec<String> = Vec::new();
    for line in &lines {
        if let Some(name) = line.strip_prefix("question, ") {
            question = name.to_string();
        } else if line.starts_with("set, ") {
            triple = vec![line.clone()];
        } else if line.starts_with("plurality, ") || line.starts_with("margin, ") {
            triple.push(line.clone());
            if triple.len() == 3 {
                let label = triple[0]
                    .split(", ")
                    .nth(1)
                    .expect("a set line label")
                    .to_string();
                let held = accord_parts
                    .iter()
                    .find(|(held_question, held_label, _)| {
                        held_question == &question && held_label == &label
                    })
                    .expect("an accord part for the pairing part");
                assert_eq!(&held.2, &triple, "the part of {} at {}", label, question);
                compared += 1;
            }
        }
    }
    assert_eq!(compared, 24);
}

#[test]
fn section_figures_sum_to_the_closing_and_the_generated_closing_stands() {
    let sets = recorded_sets();
    assert_pairs_sums(&pairs_lines(&sets).unwrap());
    let generated = generated_sets_loaded();
    let lines = pairs_lines(&generated).unwrap();
    assert_pairs_sums(&lines);
    assert_eq!(
        lines.last().unwrap(),
        "pairs, 6 pairs, 48 shared, 24 agreed, 12 divided, 11 one holds, 1 neither holds"
    );
}

#[test]
fn closing_reconciles_with_the_accord_reading_over_both_records() {
    let sets = recorded_sets();
    let lines = pairs_lines(&sets).unwrap();
    let (sections, _) = parse_pairs(&lines);
    let section_shared: i64 = sections.iter().map(|section| section.2[0]).sum();
    let section_agreed: i64 = sections.iter().map(|section| section.2[1]).sum();
    let (recount_shared, recount_agreed) = accord_recount(&sets);
    assert_eq!(section_shared, recount_shared);
    assert_eq!(section_agreed, recount_agreed);
    assert_eq!(recount_shared, 12);
    assert_eq!(recount_agreed, 11);
    let generated = generated_sets_loaded();
    let generated_lines = pairs_lines(&generated).unwrap();
    let (sections, _) = parse_pairs(&generated_lines);
    let section_shared: i64 = sections.iter().map(|section| section.2[0]).sum();
    let section_agreed: i64 = sections.iter().map(|section| section.2[1]).sum();
    let (recount_shared, recount_agreed) = accord_recount(&generated);
    assert_eq!(section_shared, recount_shared);
    assert_eq!(section_agreed, recount_agreed);
    assert_eq!(recount_shared, 48);
    assert_eq!(recount_agreed, 24);
}

#[test]
fn agreement_shape_states_the_part_agreed() {
    let question = "Which gate should the fair open first?";
    let answers = ["the red gate", "the blue gate"];
    let mut responses = votes_of("one", "the red gate", 3);
    responses.extend(votes_of("two", "the blue gate", 1));
    let alpha = poll(question, &answers, responses);
    let mut responses = votes_of("one", "the red gate", 2);
    responses.extend(votes_of("two", "the blue gate", 1));
    let beta = poll(question, &answers, responses);
    let sets = vec![(pt("alpha"), vec![alpha]), (pt("beta"), vec![beta])];
    assert_eq!(
        pairs_lines(&sets).unwrap(),
        expected(&[
            "pair, alpha",
            "pair, beta",
            "question, Which gate should the fair open first?",
            "set, alpha, recorded 4, valid 4, abstained 0, spoiled 0",
            "plurality, the red gate",
            "margin, 2",
            "set, beta, recorded 3, valid 3, abstained 0, spoiled 0",
            "plurality, the red gate",
            "margin, 1",
            "pairing, agreed",
            "standing, 1 shared, 1 agreed, 0 divided, 0 one holds, 0 neither holds",
            "pairs, 1 pairs, 1 shared, 1 agreed, 0 divided, 0 one holds, 0 neither holds",
        ])
    );
}

#[test]
fn divided_shape_states_the_part_divided() {
    let question = "Which gate should the fair open first?";
    let answers = ["the red gate", "the blue gate"];
    let mut responses = votes_of("one", "the red gate", 3);
    responses.extend(votes_of("two", "the blue gate", 1));
    let alpha = poll(question, &answers, responses);
    let mut responses = votes_of("one", "the blue gate", 2);
    responses.extend(votes_of("two", "the red gate", 1));
    let beta = poll(question, &answers, responses);
    let sets = vec![(pt("alpha"), vec![alpha]), (pt("beta"), vec![beta])];
    assert_eq!(
        pairs_lines(&sets).unwrap(),
        expected(&[
            "pair, alpha",
            "pair, beta",
            "question, Which gate should the fair open first?",
            "set, alpha, recorded 4, valid 4, abstained 0, spoiled 0",
            "plurality, the red gate",
            "margin, 2",
            "set, beta, recorded 3, valid 3, abstained 0, spoiled 0",
            "plurality, the blue gate",
            "margin, 1",
            "pairing, divided",
            "standing, 1 shared, 0 agreed, 1 divided, 0 one holds, 0 neither holds",
            "pairs, 1 pairs, 1 shared, 0 agreed, 1 divided, 0 one holds, 0 neither holds",
        ])
    );
}

#[test]
fn one_holds_shape_states_the_part_one_holds() {
    let question = "Which gate should the fair open first?";
    let answers = ["the red gate", "the blue gate"];
    let mut responses = votes_of("one", "the red gate", 3);
    responses.extend(votes_of("two", "the blue gate", 1));
    let alpha = poll(question, &answers, responses);
    let mut responses = votes_of("one", "the red gate", 2);
    responses.extend(votes_of("two", "the blue gate", 2));
    let beta = poll(question, &answers, responses);
    let sets = vec![(pt("alpha"), vec![alpha]), (pt("beta"), vec![beta])];
    assert_eq!(
        pairs_lines(&sets).unwrap(),
        expected(&[
            "pair, alpha",
            "pair, beta",
            "question, Which gate should the fair open first?",
            "set, alpha, recorded 4, valid 4, abstained 0, spoiled 0",
            "plurality, the red gate",
            "margin, 2",
            "set, beta, recorded 4, valid 4, abstained 0, spoiled 0",
            "plurality, none",
            "margin, not counted",
            "pairing, one holds",
            "standing, 1 shared, 0 agreed, 0 divided, 1 one holds, 0 neither holds",
            "pairs, 1 pairs, 1 shared, 0 agreed, 0 divided, 1 one holds, 0 neither holds",
        ])
    );
}

#[test]
fn neither_holds_shape_states_the_part_neither_holds_and_the_section_stands() {
    let question = "Which gate should the fair open first?";
    let answers = ["the red gate", "the blue gate"];
    let abstained = |jury: &str| -> Vec<Response> {
        (1..=3)
            .map(|seat| {
                response(
                    &format!("{} seat {}", jury, seat),
                    "model a",
                    None,
                    "no answer recorded",
                )
            })
            .collect()
    };
    let alpha = poll(question, &answers, abstained("one"));
    let beta = poll(question, &answers, abstained("two"));
    let sets = vec![(pt("alpha"), vec![alpha]), (pt("beta"), vec![beta])];
    assert_eq!(
        pairs_lines(&sets).unwrap(),
        expected(&[
            "pair, alpha",
            "pair, beta",
            "question, Which gate should the fair open first?",
            "set, alpha, recorded 3, valid 0, abstained 3, spoiled 0",
            "plurality, none",
            "margin, not counted",
            "set, beta, recorded 3, valid 0, abstained 3, spoiled 0",
            "plurality, none",
            "margin, not counted",
            "pairing, neither holds",
            "standing, 1 shared, 0 agreed, 0 divided, 0 one holds, 1 neither holds",
            "pairs, 1 pairs, 1 shared, 0 agreed, 0 divided, 0 one holds, 1 neither holds",
        ])
    );
}

#[test]
fn pair_agreed_where_no_accord_stands_states_agreed_and_divided() {
    let question = "Which gate should the fair open first?";
    let answers = ["the red gate", "the blue gate"];
    let build = |plurality: &str, other: &str| {
        let mut responses = votes_of("one", plurality, 2);
        responses.extend(votes_of("two", other, 1));
        poll(question, &answers, responses)
    };
    let sets = vec![
        (pt("alpha"), vec![build("the red gate", "the blue gate")]),
        (pt("beta"), vec![build("the red gate", "the blue gate")]),
        (pt("gamma"), vec![build("the blue gate", "the red gate")]),
    ];
    let lines = pairs_lines(&sets).unwrap();
    let pairings: Vec<&str> = lines
        .iter()
        .filter(|line| line.starts_with("pairing, "))
        .map(|line| line.as_str())
        .collect();
    assert_eq!(
        pairings,
        vec!["pairing, agreed", "pairing, divided", "pairing, divided"]
    );
    assert_eq!(
        lines.last().unwrap(),
        "pairs, 3 pairs, 3 shared, 1 agreed, 2 divided, 0 one holds, 0 neither holds"
    );
    let accord = accord_lines(&sets).unwrap();
    assert!(accord.contains(&"accord, none".to_string()));
}

#[test]
fn disjoint_shape_and_the_empty_forms_stand_at_zero_figures() {
    let gates = "Which gate should the fair open first?";
    let fields = "Which field should the fair mow first?";
    let mut gate_responses = votes_of("one", "the red gate", 2);
    gate_responses.extend(votes_of("two", "the blue gate", 1));
    let alpha = vec![poll(
        gates,
        &["the red gate", "the blue gate"],
        gate_responses,
    )];
    let mut field_responses = votes_of("one", "the south field", 2);
    field_responses.extend(votes_of("two", "the north field", 1));
    let beta = vec![poll(
        fields,
        &["the north field", "the south field"],
        field_responses,
    )];
    let sets = vec![(pt("alpha"), alpha.clone()), (pt("beta"), beta.clone())];
    let lines = pairs_lines(&sets).unwrap();
    assert_eq!(
        lines,
        expected(&[
            "pair, alpha",
            "pair, beta",
            "standing, 0 shared, 0 agreed, 0 divided, 0 one holds, 0 neither holds",
            "pairs, 1 pairs, 0 shared, 0 agreed, 0 divided, 0 one holds, 0 neither holds",
        ])
    );
    let mut with_empty = sets.clone();
    with_empty.push((pt("empty"), Vec::new()));
    let widened = pairs_lines(&with_empty).unwrap();
    assert_eq!(&widened[..3], &lines[..3]);
    assert_eq!(
        widened.last().unwrap(),
        "pairs, 3 pairs, 0 shared, 0 agreed, 0 divided, 0 one holds, 0 neither holds"
    );
    let all_empty = vec![(pt("alpha"), Vec::new()), (pt("beta"), Vec::new())];
    assert_eq!(
        pairs_lines(&all_empty).unwrap(),
        expected(&[
            "pair, alpha",
            "pair, beta",
            "standing, 0 shared, 0 agreed, 0 divided, 0 one holds, 0 neither holds",
            "pairs, 1 pairs, 0 shared, 0 agreed, 0 divided, 0 one holds, 0 neither holds",
        ])
    );
    let one_set = vec![(pt("alpha"), alpha)];
    assert_eq!(
        pairs_lines(&one_set).unwrap(),
        expected(&["pairs, 0 pairs, 0 shared, 0 agreed, 0 divided, 0 one holds, 0 neither holds",])
    );
}

#[test]
fn pair_at_two_questions_states_each_part_from_its_own_question() {
    let gates = "Which gate should the fair open first?";
    let fields = "Which field should the fair mow first?";
    let build = |first: &str, first_other: &str, second: &str, second_other: &str| {
        let mut gate_responses = votes_of("one", first, 2);
        gate_responses.extend(votes_of("two", first_other, 1));
        let mut field_responses = votes_of("one", second, 2);
        field_responses.extend(votes_of("two", second_other, 1));
        vec![
            poll(gates, &["the red gate", "the blue gate"], gate_responses),
            poll(
                fields,
                &["the north field", "the south field"],
                field_responses,
            ),
        ]
    };
    let sets = vec![
        (
            pt("alpha"),
            build(
                "the red gate",
                "the blue gate",
                "the south field",
                "the north field",
            ),
        ),
        (
            pt("beta"),
            build(
                "the red gate",
                "the blue gate",
                "the north field",
                "the south field",
            ),
        ),
    ];
    assert_eq!(
        pairs_lines(&sets).unwrap(),
        expected(&[
            "pair, alpha",
            "pair, beta",
            "question, Which gate should the fair open first?",
            "set, alpha, recorded 3, valid 3, abstained 0, spoiled 0",
            "plurality, the red gate",
            "margin, 1",
            "set, beta, recorded 3, valid 3, abstained 0, spoiled 0",
            "plurality, the red gate",
            "margin, 1",
            "pairing, agreed",
            "question, Which field should the fair mow first?",
            "set, alpha, recorded 3, valid 3, abstained 0, spoiled 0",
            "plurality, the south field",
            "margin, 1",
            "set, beta, recorded 3, valid 3, abstained 0, spoiled 0",
            "plurality, the north field",
            "margin, 1",
            "pairing, divided",
            "standing, 2 shared, 1 agreed, 1 divided, 0 one holds, 0 neither holds",
            "pairs, 1 pairs, 2 shared, 1 agreed, 1 divided, 0 one holds, 0 neither holds",
        ])
    );
}

#[test]
fn guard_refuses_a_newline_in_the_model_of_the_second_set() {
    let mut sets = recorded_sets();
    sets[1].1[1].responses[0].model = pt("model a\npairs, forged");
    let defect = pairs_lines(&sets).unwrap_err();
    assert_eq!(defect, "response 0 field model holds a newline");
    assert_eq!(census_lines(&sets).unwrap_err(), defect);
}

#[test]
fn guard_refuses_a_newline_in_the_question_of_the_first_set() {
    let mut sets = recorded_sets();
    let question = sets[0].1[0].question.display();
    sets[0].1[0].question = pt(&format!("{}\npairs, forged", question));
    let defect = pairs_lines(&sets).unwrap_err();
    assert_eq!(defect, "field question holds a newline");
    assert_eq!(census_lines(&sets).unwrap_err(), defect);
}

#[test]
fn set_rules_refuse_the_empty_the_duplicate_and_the_mixed_fixtures() {
    let question = "Which gate should the fair open first?";
    let answers = ["the red gate", "the blue gate"];
    let one = || poll(question, &answers, votes_of("one", "the red gate", 1));
    let sets = vec![(pt(""), vec![one()])];
    assert_eq!(pairs_lines(&sets).unwrap_err(), "a set label is empty");
    let sets = vec![(pt("alpha"), vec![one()]), (pt("alpha"), vec![one()])];
    assert_eq!(
        pairs_lines(&sets).unwrap_err(),
        "the set label alpha stands twice"
    );
    let other = "Which field should the fair mow first?";
    let sets = vec![
        (
            pt("beta"),
            vec![poll(other, &answers, votes_of("one", "the red gate", 1))],
        ),
        (pt("beta"), vec![one(), one()]),
    ];
    assert_eq!(
        pairs_lines(&sets).unwrap_err(),
        "the set label beta stands twice"
    );
}

#[test]
fn no_line_of_the_reading_holds_a_refused_word() {
    let verdict_words = [
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
    assert!(!verdict_words.contains(&"pairs"));
    let golden = read_text(&examples().join("pairs-golden.txt"));
    assert!(golden.contains("pairs, 3 pairs"));
    let openers = golden
        .lines()
        .filter(|line| line.starts_with("pair, "))
        .count();
    assert_eq!(openers, 6);
    let mut lines: Vec<String> = golden.lines().map(|line| line.to_string()).collect();
    lines.extend(pairs_lines(&generated_sets_in_memory()).unwrap());
    for line in &lines {
        for token in line
            .split(|c: char| !c.is_alphanumeric())
            .filter(|token| !token.is_empty())
        {
            assert!(
                !verdict_words.contains(&token.to_lowercase().as_str()),
                "a refused word in: {}",
                line
            );
        }
    }
}

#[test]
fn help_text_names_the_pairs_command() {
    let (stdout, _, code) = run_cli(&["pairs", "--help"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("usage: grandjury pairs [-h] label directory [label directory ...]"));
    let (stdout, _, code) = run_cli(&["--help"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("print the pairing reading over recorded poll sets"));
}

#[test]
fn generated_family_holds_its_figures() {
    let sets = generated_sets_in_memory();
    assert_eq!(questions_of(&sets).len(), 36);
    let polls: Vec<&Poll> = sets.iter().flat_map(|(_, polls)| polls.iter()).collect();
    assert_eq!(polls.len(), 72);
    let responses: Vec<&Response> = polls
        .iter()
        .flat_map(|held| held.responses.iter())
        .collect();
    assert_eq!(responses.len(), 720);
    assert_eq!(responses.iter().filter(|held| held.abstained).count(), 285);
    let spoiled = polls
        .iter()
        .flat_map(|held| {
            held.responses.iter().filter(move |response| {
                !response.abstained && !held.answers.contains(&response.answer)
            })
        })
        .count();
    assert_eq!(spoiled, 12);
    assert_eq!(responses.len() - 285 - spoiled, 423);
    for held in &polls {
        let models: std::collections::HashSet<&PText> = held
            .responses
            .iter()
            .map(|response| &response.model)
            .collect();
        assert_eq!(models.len(), 2);
    }
    let lines = pairs_lines(&sets).unwrap();
    let standings: Vec<&str> = lines
        .iter()
        .filter(|line| line.starts_with("standing,"))
        .map(|line| line.as_str())
        .collect();
    assert_eq!(
        standings,
        vec![
            "standing, 12 shared, 5 agreed, 3 divided, 3 one holds, 1 neither holds",
            "standing, 5 shared, 4 agreed, 1 divided, 0 one holds, 0 neither holds",
            "standing, 11 shared, 5 agreed, 2 divided, 4 one holds, 0 neither holds",
            "standing, 7 shared, 4 agreed, 1 divided, 2 one holds, 0 neither holds",
            "standing, 7 shared, 1 agreed, 4 divided, 2 one holds, 0 neither holds",
            "standing, 6 shared, 5 agreed, 1 divided, 0 one holds, 0 neither holds",
        ]
    );
    assert_eq!(
        lines.last().unwrap(),
        "pairs, 6 pairs, 48 shared, 24 agreed, 12 divided, 11 one holds, 1 neither holds"
    );
}

#[test]
fn generated_recount_matches_the_direct_computation() {
    let loaded = generated_sets_loaded();
    let in_memory = generated_sets_in_memory();
    let lines = pairs_lines(&loaded).unwrap();
    assert_eq!(lines, direct_pairs_lines(&in_memory));
    let closing = lines.last().unwrap();
    assert_eq!(figure_before(closing, "pairs"), 6);
    assert_eq!(figure_before(closing, "shared"), 48);
    assert_eq!(figure_before(closing, "agreed"), 24);
    assert_eq!(figure_before(closing, "divided"), 12);
    assert_eq!(figure_before(closing, "one holds"), 11);
    assert_eq!(figure_before(closing, "neither holds"), 1);
}

#[test]
fn generated_relations_hold_and_the_sections_sum() {
    let loaded = generated_sets_loaded();
    let lines = pairs_lines(&loaded).unwrap();
    assert_pairs_sums(&lines);
    let (sections, _) = parse_pairs(&lines);
    let section_shared: i64 = sections.iter().map(|section| section.2[0]).sum();
    let section_agreed: i64 = sections.iter().map(|section| section.2[1]).sum();
    let (recount_shared, recount_agreed) = accord_recount(&loaded);
    assert_eq!(section_shared, recount_shared);
    assert_eq!(section_agreed, recount_agreed);
}

// The generated family of GJ-27, copied from the
// standing tests, the family the pairing reading
// states its generated figures over. The draws
// stand in the order the standing file states
// them, and the family the two files generate is
// the one family.
struct Lcg {
    state: u64,
}

impl Lcg {
    fn draw(&mut self, span: u64) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state % span
    }
}

const GEN_ANSWERS: [&str; 4] = [
    "the ash gate",
    "the birch gate",
    "the cedar gate",
    "the elm gate",
];
const GEN_MODELS: [&str; 3] = ["model a", "model b", "model c"];
const GEN_REASONS: [&str; 4] = [
    "the record states it",
    "the shorter path",
    "as the record stands",
    "a stated reason",
];
const GEN_SET_LABELS: [&str; 4] = ["gen-a", "gen-b", "gen-c", "gen-d"];

fn draw_answer(rng: &mut Lcg, offered: usize, favoured: usize) -> usize {
    let total = 2 * offered as u64 + 3;
    let roll = rng.draw(total);
    let mut accrued = 0u64;
    for answer in 0..offered {
        accrued += if answer == favoured { 5 } else { 2 };
        if roll < accrued {
            return answer;
        }
    }
    offered - 1
}

fn generated_sets_in_memory() -> Vec<(PText, Vec<Poll>)> {
    let mut rng = Lcg { state: 20261113 };
    let mut questions: Vec<(usize, usize)> = Vec::new();
    for _ in 0..36 {
        let offered = 2 + rng.draw(3) as usize;
        let favoured = rng.draw(offered as u64) as usize;
        questions.push((offered, favoured));
    }
    let mut askings: Vec<Vec<usize>> = Vec::new();
    for _ in 0..36 {
        let count = 1 + rng.draw(3) as usize;
        let mut chosen: Vec<usize> = Vec::new();
        while chosen.len() < count {
            let candidate = rng.draw(4) as usize;
            if !chosen.contains(&candidate) {
                chosen.push(candidate);
            }
        }
        askings.push(chosen);
    }
    let mut plan: Vec<Vec<(usize, Vec<usize>)>> =
        vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    for (set_index, slot) in plan.iter_mut().enumerate() {
        for (question_index, asking_sets) in askings.iter().enumerate() {
            if asking_sets.contains(&set_index) {
                let model_count = 2 + rng.draw(2) as usize;
                let first = rng.draw(3) as usize;
                let models: Vec<usize> = (0..model_count).map(|step| (first + step) % 3).collect();
                slot.push((question_index, models));
            }
        }
    }
    let mut sets: Vec<(PText, Vec<Poll>)> = Vec::new();
    for (set_index, slot) in plan.iter().enumerate() {
        let mut polls: Vec<Poll> = Vec::new();
        for (question_index, models) in slot {
            let (offered, favoured) = questions[*question_index];
            let mut responses: Vec<Response> = Vec::new();
            for model_index in models {
                for seat in 1..=5 {
                    let juror = format!("{} seat {}", GEN_MODELS[*model_index], seat);
                    if rng.draw(100) < 39 {
                        let drawn = draw_answer(&mut rng, offered, favoured);
                        let reason = GEN_REASONS[rng.draw(4) as usize];
                        let mut held = response(&juror, GEN_MODELS[*model_index], None, reason);
                        held.answer = pt(GEN_ANSWERS[drawn]);
                        responses.push(held);
                    } else {
                        let spoiled = rng.draw(50) == 0;
                        let drawn = draw_answer(&mut rng, offered, favoured);
                        let answer = if spoiled {
                            "the unoffered gate"
                        } else {
                            GEN_ANSWERS[drawn]
                        };
                        let reason = GEN_REASONS[rng.draw(4) as usize];
                        responses.push(response(
                            &juror,
                            GEN_MODELS[*model_index],
                            Some(answer),
                            reason,
                        ));
                    }
                }
            }
            polls.push(Poll {
                question: pt(&format!("generated question {}?", question_index + 1)),
                answers: GEN_ANSWERS[..offered]
                    .iter()
                    .map(|answer| pt(answer))
                    .collect(),
                responses,
            });
        }
        sets.push((pt(GEN_SET_LABELS[set_index]), polls));
    }
    sets
}

fn generated_sets_loaded() -> Vec<(PText, Vec<Poll>)> {
    let scratch = TempDir::new("pairs-generated");
    let mut sets: Vec<(PText, Vec<Poll>)> = Vec::new();
    for (index, (label, polls)) in generated_sets_in_memory().iter().enumerate() {
        let dir = scratch.path.join(format!("set-{}", index));
        std::fs::create_dir_all(&dir).unwrap();
        for (offset, held) in polls.iter().enumerate() {
            std::fs::write(
                dir.join(format!("poll-{:02}.json", offset + 1)),
                dumps_default(&held.to_json()),
            )
            .unwrap();
        }
        sets.push((label.clone(), load_set(&dir, polls.len())));
    }
    sets
}
