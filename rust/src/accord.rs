//! The accord reading over the recorded poll sets.
//!
//! The reading reads the whole record question by
//! question. For each question it states one part per
//! set that asked it, the counts of the set summed
//! over its askings of the question, the plurality of
//! the set at the question, and the margin of that
//! plurality, and it closes the section with the
//! accord line, naming the one answer every set of
//! the question that holds a plurality names, or the
//! word none where no set of the question holds a
//! plurality or two sets of the question name two
//! answers. It carries no judgement about any answer
//! or any set, and accord names only whether the sets
//! that hold a plurality at a question name one
//! answer between them. The obligations this module
//! carries are GJ-37 and GJ-38 in CONTRACT.md, in the
//! Round 18 section.

use num_bigint::BigInt;
use num_rational::Ratio;

use crate::agreement::{add_reading, jury_reading, plurality_of, poll_juries, JuryReading};
use crate::polls::Poll;
use crate::pytext::PText;
use crate::record::{check_poll_text, collect_questions, validate_sets};

/// Return the accord reading over a list of recorded
/// poll sets as lines, or the defect the first failed
/// rule names.
///
/// Each set pairs one set label with its polls. The
/// set rules of GJ-11 govern the sets unchanged, each
/// label checked before the questions of its set and
/// the first failure the refusal, and a refusal
/// returns no lines at all. One section stands per
/// question in the order the questions first appear
/// across the sets. A section opens with the question
/// line, then one part per set of the question in the
/// order the sets stand in the input, the set line
/// carrying the label of the set and the counts of
/// the set summed over its askings of the question
/// with its own plurality line and margin line, then
/// the accord line of the question. The closing line
/// counts the questions, the parts, the questions at
/// which an accord stands, and the divided questions.
pub fn accord_lines(sets: &[(PText, Vec<Poll>)]) -> Result<Vec<String>, String> {
    for (_, polls) in sets {
        check_poll_text(polls)?;
    }
    validate_sets(sets)?;
    let gathered = collect_questions(sets);

    let mut lines: Vec<String> = Vec::new();
    let mut parts = 0usize;
    let mut in_accord = 0usize;
    for (question, question_askings) in gathered.questions.iter().zip(gathered.askings.iter()) {
        lines.push(format!("question, {}", question.display()));

        // The sets of the question, in the order the
        // sets stand in the input.
        let mut set_order: Vec<usize> = Vec::new();
        for (set_index, _) in question_askings {
            if !set_order.contains(set_index) {
                set_order.push(*set_index);
            }
        }

        // The answers the sets holding a plurality at
        // the question name, in the order the parts
        // name them, each answer standing once.
        let mut named: Vec<PText> = Vec::new();
        for set_index in &set_order {
            let part_polls: Vec<&Poll> = question_askings
                .iter()
                .filter(|(held, _)| held == set_index)
                .map(|(_, poll)| *poll)
                .collect();

            // The offered answers of the part, the
            // union in asking order over the askings
            // of the part alone.
            let mut union: Vec<PText> = Vec::new();
            for poll in &part_polls {
                for answer in &poll.answers {
                    if !union.contains(answer) {
                        union.push(answer.clone());
                    }
                }
            }

            // The part of the set at the question,
            // every response of the set's askings of
            // the question summed as one pool.
            let mut counts: Vec<usize> = vec![0; union.len()];
            let mut totals: (usize, usize, usize, usize) = (0, 0, 0, 0);
            for poll in &part_polls {
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
            parts += 1;
            if let Some(answer) = &plurality {
                if !named.contains(answer) {
                    named.push(answer.clone());
                }
            }
            lines.push(format!(
                "set, {}, {}",
                sets[*set_index].0.display(),
                reading.counts_text()
            ));
            lines.push(format!(
                "plurality, {}",
                match &plurality {
                    Some(answer) => answer.display(),
                    None => "none".to_string(),
                }
            ));
            lines.push(match reading.margin() {
                Some(figure) => format!("margin, {}", figure),
                None => "margin, not counted".to_string(),
            });
        }

        // The accord of the question, the one answer
        // every set holding a plurality names, standing
        // only where at least one set holds one and the
        // holding sets name one answer between them.
        let accord = if named.len() == 1 {
            Some(named[0].clone())
        } else {
            None
        };
        if accord.is_some() {
            in_accord += 1;
        }
        lines.push(format!(
            "accord, {}",
            match &accord {
                Some(answer) => answer.display(),
                None => "none".to_string(),
            }
        ));
    }
    lines.push(format!(
        "accords, {} questions, {} parts, {} in accord, {} divided",
        gathered.questions.len(),
        parts,
        in_accord,
        gathered.questions.len() - in_accord
    ));
    Ok(lines)
}
