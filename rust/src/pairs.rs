//! The pairing reading over the recorded poll sets.
//!
//! The reading reads the whole record set pair by
//! set pair. For each pair of sets it states one
//! part per question the pair shares, the part of
//! each set at the question in the form of the
//! accord reading, its counts, its plurality and
//! its margin, and the pairing of the pair at the
//! question, agreed where both parts hold and name
//! one answer, divided where both hold and name
//! two, one holds where exactly one part holds and
//! neither holds where no part holds, and it closes
//! the section with the standing line of the pair
//! and the reading with the counts of its pairs and
//! of its shared questions and pairings summed over
//! its sections. It carries no judgement about any
//! answer or any set, and pairs names only how the
//! sets of the record stand against each other pair
//! by pair. The obligations this module carries are
//! GJ-43 and GJ-44 in CONTRACT.md, in the Round 21
//! section.

use num_bigint::BigInt;
use num_rational::Ratio;

use crate::agreement::{add_reading, jury_reading, plurality_of, poll_juries, JuryReading};
use crate::polls::Poll;
use crate::pytext::PText;
use crate::record::{check_poll_text, collect_questions, validate_sets};

/// The part of one set at one question, its set
/// line, its plurality line and its margin line in
/// the form of the accord, and the plurality the
/// part holds, counted as GJ-37 states the part is
/// counted, every response of the set's askings of
/// the question summed as one pool.
fn part_lines(label: &PText, part_polls: &[&Poll]) -> (Vec<String>, Option<PText>) {
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
    let plurality = plurality_of(&counts, totals.1, &union);
    let reading = JuryReading {
        recorded: totals.0,
        valid: totals.1,
        abstained: totals.2,
        spoiled: totals.3,
        rows: union
            .iter()
            .zip(counts.iter())
            .map(|(answer, count)| {
                let share = if totals.1 > 0 {
                    Ratio::new(BigInt::from(*count), BigInt::from(totals.1))
                } else {
                    Ratio::new(BigInt::from(0), BigInt::from(1))
                };
                (answer.clone(), *count, share)
            })
            .collect(),
        plurality: plurality.clone(),
    };
    let lines = vec![
        format!("set, {}, {}", label.display(), reading.counts_text()),
        format!(
            "plurality, {}",
            match &plurality {
                Some(answer) => answer.display(),
                None => "none".to_string(),
            }
        ),
        match reading.margin() {
            Some(figure) => format!("margin, {}", figure),
            None => "margin, not counted".to_string(),
        },
    ];
    (lines, plurality)
}

/// Return the pairing reading over a list of
/// recorded poll sets as lines, or the defect the
/// first failed rule names.
///
/// Each set pairs one set label with its polls. The
/// set rules of GJ-2 govern the sets unchanged,
/// each label checked before the questions of its
/// set and the first failure the refusal, and a
/// refusal returns no lines at all. One section
/// stands per pair of sets in the order of their
/// sets in the input. A section opens with the two
/// pair lines of its sets, then one part per
/// question the pair shares in the order the
/// questions first appear across the sets, the
/// question line, the two parts in the form of the
/// accord and the pairing line, then the standing
/// line of the pair. The closing line counts the
/// pairs, the shared questions summed over the
/// sections, and the agreed and divided and one
/// holds and neither holds pairings summed over
/// the sections.
pub fn pairs_lines(sets: &[(PText, Vec<Poll>)]) -> Result<Vec<String>, String> {
    for (_, polls) in sets {
        check_poll_text(polls)?;
    }
    validate_sets(sets)?;
    let gathered = collect_questions(sets);

    let mut lines: Vec<String> = Vec::new();
    let mut total_pairs = 0usize;
    let mut total_shared = 0usize;
    let mut total_agreed = 0usize;
    let mut total_divided = 0usize;
    let mut total_one_holds = 0usize;
    let mut total_neither_holds = 0usize;
    for first in 0..sets.len() {
        for second in first + 1..sets.len() {
            total_pairs += 1;
            lines.push(format!("pair, {}", sets[first].0.display()));
            lines.push(format!("pair, {}", sets[second].0.display()));
            let mut shared = 0usize;
            let mut agreed = 0usize;
            let mut divided = 0usize;
            let mut one_holds = 0usize;
            let mut neither_holds = 0usize;
            for (question, question_askings) in
                gathered.questions.iter().zip(gathered.askings.iter())
            {
                let first_polls: Vec<&Poll> = question_askings
                    .iter()
                    .filter(|(held, _)| *held == first)
                    .map(|(_, poll)| *poll)
                    .collect();
                if first_polls.is_empty() {
                    continue;
                }
                let second_polls: Vec<&Poll> = question_askings
                    .iter()
                    .filter(|(held, _)| *held == second)
                    .map(|(_, poll)| *poll)
                    .collect();
                if second_polls.is_empty() {
                    continue;
                }
                shared += 1;
                lines.push(format!("question, {}", question.display()));
                let (first_lines, first_plurality) = part_lines(&sets[first].0, &first_polls);
                lines.extend(first_lines);
                let (second_lines, second_plurality) = part_lines(&sets[second].0, &second_polls);
                lines.extend(second_lines);
                let pairing = match (&first_plurality, &second_plurality) {
                    (Some(one), Some(two)) => {
                        if one == two {
                            agreed += 1;
                            "agreed"
                        } else {
                            divided += 1;
                            "divided"
                        }
                    }
                    (Some(_), None) | (None, Some(_)) => {
                        one_holds += 1;
                        "one holds"
                    }
                    (None, None) => {
                        neither_holds += 1;
                        "neither holds"
                    }
                };
                lines.push(format!("pairing, {}", pairing));
            }
            lines.push(format!(
                "standing, {} shared, {} agreed, {} divided, {} one holds, {} neither holds",
                shared, agreed, divided, one_holds, neither_holds
            ));
            total_shared += shared;
            total_agreed += agreed;
            total_divided += divided;
            total_one_holds += one_holds;
            total_neither_holds += neither_holds;
        }
    }
    lines.push(format!(
        "pairs, {} pairs, {} shared, {} agreed, {} divided, {} one holds, {} neither holds",
        total_pairs,
        total_shared,
        total_agreed,
        total_divided,
        total_one_holds,
        total_neither_holds
    ));
    Ok(lines)
}
