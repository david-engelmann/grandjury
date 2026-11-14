//! The shares reading over the recorded poll sets.
//!
//! The reading reads the whole record set by set.
//! For each set it states one part per question the
//! set asks, the share of the question's valid
//! responses the set holds at it as an exact
//! fraction in lowest terms, and it closes the
//! section with the standing line of the set and
//! the reading with the counts of its sets and of
//! its parts and of its parts standing whole and
//! none and not counted summed over its sections.
//! It carries no judgement about any answer or any
//! set, and shares names only how the valid
//! responses of each question stand divided among
//! the sets that ask it. The obligations this
//! module carries are GJ-47 and GJ-48 in
//! CONTRACT.md, in the Round 23 section.

use num_bigint::BigInt;
use num_rational::Ratio;

use crate::agreement::{add_reading, jury_reading, poll_juries};
use crate::polls::{fraction_text, Poll};
use crate::pytext::PText;
use crate::record::{check_poll_text, collect_questions, validate_sets};

/// The valid count of the part of one set at one
/// question, counted as GJ-37 states the part is
/// counted, every response of the set's askings of
/// the question summed as one pool.
fn part_valid(part_polls: &[&Poll]) -> usize {
    // The offered answers of the part, the union in
    // asking order over the askings of the part
    // alone.
    let mut union: Vec<PText> = Vec::new();
    for poll in part_polls {
        for answer in &poll.answers {
            if !union.contains(answer) {
                union.push(answer.clone());
            }
        }
    }

    // The part of the set at the question, every
    // response of the set's askings of the question
    // summed as one pool.
    let mut counts: Vec<usize> = vec![0; union.len()];
    let mut totals: (usize, usize, usize, usize) = (0, 0, 0, 0);
    for poll in part_polls {
        let juries = poll_juries(poll);
        let all: Vec<_> = juries
            .iter()
            .flat_map(|(_, rs)| rs.iter().copied())
            .collect();
        add_reading(&mut counts, &mut totals, &union, &jury_reading(poll, &all));
    }
    totals.1
}

/// Return the shares reading over a list of
/// recorded poll sets as lines, or the defect the
/// first failed rule names.
///
/// Each set pairs one set label with its polls. The
/// set rules of GJ-2 govern the sets unchanged,
/// each label checked before the questions of its
/// set and the first failure the refusal, and a
/// refusal returns no lines at all. One section
/// stands per set in the order of their sets in
/// the input. A section opens with the set line,
/// then one part per question the set asks in the
/// order the questions first appear across the
/// sets, the question line and the share line,
/// then the standing line of the set. The closing
/// line counts the sets, the parts summed over the
/// sections, and the whole and none and not
/// counted parts summed over the sections.
pub fn shares_lines(sets: &[(PText, Vec<Poll>)]) -> Result<Vec<String>, String> {
    for (_, polls) in sets {
        check_poll_text(polls)?;
    }
    validate_sets(sets)?;
    let gathered = collect_questions(sets);

    // The valid count of each part, by question and
    // by set, standing empty where the set does not
    // ask the question, and the valid count of each
    // question, the sum of the valid counts of its
    // parts over every set that asks it.
    let mut parts: Vec<Vec<Option<usize>>> = Vec::new();
    let mut question_valids: Vec<usize> = Vec::new();
    for (_, question_askings) in gathered.questions.iter().zip(gathered.askings.iter()) {
        let mut by_set: Vec<Option<usize>> = Vec::new();
        let mut question_valid = 0usize;
        for set_index in 0..sets.len() {
            let part_polls: Vec<&Poll> = question_askings
                .iter()
                .filter(|(held, _)| *held == set_index)
                .map(|(_, poll)| *poll)
                .collect();
            if part_polls.is_empty() {
                by_set.push(None);
            } else {
                let valid = part_valid(&part_polls);
                question_valid += valid;
                by_set.push(Some(valid));
            }
        }
        parts.push(by_set);
        question_valids.push(question_valid);
    }

    let mut lines: Vec<String> = Vec::new();
    let mut total_parts = 0usize;
    let mut total_whole = 0usize;
    let mut total_none = 0usize;
    let mut total_not_counted = 0usize;
    for (set_index, (label, _)) in sets.iter().enumerate() {
        lines.push(format!("set, {}", label.display()));
        let mut set_parts = 0usize;
        let mut set_whole = 0usize;
        let mut set_none = 0usize;
        let mut set_not_counted = 0usize;
        for (index, question) in gathered.questions.iter().enumerate() {
            let Some(part) = parts[index][set_index] else {
                continue;
            };
            set_parts += 1;
            lines.push(format!("question, {}", question.display()));
            let question_valid = question_valids[index];
            if question_valid == 0 {
                set_not_counted += 1;
                lines.push("share, not counted".to_string());
            } else {
                if part == 0 {
                    set_none += 1;
                }
                if part == question_valid {
                    set_whole += 1;
                }
                let share = Ratio::new(BigInt::from(part), BigInt::from(question_valid));
                lines.push(format!(
                    "share, {} of {}, {}",
                    part,
                    question_valid,
                    fraction_text(&share)
                ));
            }
        }
        lines.push(format!(
            "standing, {} parts, {} whole, {} none, {} not counted",
            set_parts, set_whole, set_none, set_not_counted
        ));
        total_parts += set_parts;
        total_whole += set_whole;
        total_none += set_none;
        total_not_counted += set_not_counted;
    }
    lines.push(format!(
        "shares, {} sets, {} parts, {} whole, {} none, {} not counted",
        sets.len(),
        total_parts,
        total_whole,
        total_none,
        total_not_counted
    ));
    Ok(lines)
}
