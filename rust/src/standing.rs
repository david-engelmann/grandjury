//! The standing reading over the recorded poll sets.
//!
//! The reading reads the whole record set by set. For
//! each set it states the counts of the set over the
//! whole record, then one part per question the set
//! asked, the plurality of the set at the question and
//! the margin of that plurality in the forms the
//! accord states for its parts, and it closes the
//! section with the standing line, counting the set's
//! questions, its parts with a plurality, its tied
//! parts, its parts with no valid response, its parts
//! in accord, and its parts with the pool. It carries
//! no judgement about any answer or any set, and
//! standing names only how one set stands over the
//! whole record. The obligations this module carries
//! are GJ-39 and GJ-40 in CONTRACT.md, in the Round 19
//! section.

use num_bigint::BigInt;
use num_rational::Ratio;

use crate::agreement::{
    add_reading, counts_text, jury_reading, plurality_of, poll_juries, JuryReading,
};
use crate::polls::Poll;
use crate::pytext::PText;
use crate::record::{check_poll_text, collect_questions, validate_sets};

/// One part of the standing reading, the standing of
/// one set at one question, its plurality line and
/// margin line in the forms the accord states, the
/// plurality itself, and the valid count of the part,
/// by which the part stands held or tied or empty.
struct Part {
    plurality_line: String,
    margin_line: String,
    plurality: Option<PText>,
    valid: usize,
}

/// Return the standing reading over a list of recorded
/// poll sets as lines, or the defect the first failed
/// rule names.
///
/// Each set pairs one set label with its polls. The
/// set rules of GJ-11 govern the sets unchanged, each
/// label checked before the questions of its set and
/// the first failure the refusal, and a refusal
/// returns no lines at all. One section stands per
/// set in the order the sets stand in the input. A
/// section opens with the set line, carrying the
/// label of the set and the counts of the set summed
/// over every poll of the set, then one part per
/// question the set asked in the order GJ-29 gathers
/// the questions, the part of the set at the question
/// as GJ-37 states it for the accord, then the
/// standing line of the set. The closing line counts
/// the sets, the parts, and the five standings of the
/// parts over the whole record.
pub fn standing_lines(sets: &[(PText, Vec<Poll>)]) -> Result<Vec<String>, String> {
    for (_, polls) in sets {
        check_poll_text(polls)?;
    }
    validate_sets(sets)?;
    let gathered = collect_questions(sets);

    // The parts of every question, by the set that
    // asked it, with the accord of the question and
    // the pooled plurality of the question, gathered
    // once so the sections below state them by set.
    let mut parts_by_question: Vec<Vec<(usize, Part)>> = Vec::new();
    let mut accords: Vec<Option<PText>> = Vec::new();
    let mut pooled_pluralities: Vec<Option<PText>> = Vec::new();
    for (_, question_askings) in gathered.questions.iter().zip(gathered.askings.iter()) {
        // The sets of the question, in the order the
        // sets stand in the input.
        let mut set_order: Vec<usize> = Vec::new();
        for (set_index, _) in question_askings {
            if !set_order.contains(set_index) {
                set_order.push(*set_index);
            }
        }

        let mut parts: Vec<(usize, Part)> = Vec::new();
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
            if let Some(answer) = &plurality {
                if !named.contains(answer) {
                    named.push(answer.clone());
                }
            }
            parts.push((
                *set_index,
                Part {
                    plurality_line: format!(
                        "plurality, {}",
                        match &plurality {
                            Some(answer) => answer.display(),
                            None => "none".to_string(),
                        }
                    ),
                    margin_line: match reading.margin() {
                        Some(figure) => format!("margin, {}", figure),
                        None => "margin, not counted".to_string(),
                    },
                    plurality,
                    valid: totals.1,
                },
            ));
        }

        // The accord of the question, the one answer
        // every set holding a plurality names, as
        // GJ-37 states it.
        let accord = if named.len() == 1 {
            Some(named[0].clone())
        } else {
            None
        };
        accords.push(accord);

        // The pooled plurality of the question, read
        // over the pooled counts of the whole question
        // as the aggregate reading states them.
        let mut union: Vec<PText> = Vec::new();
        for (_, poll) in question_askings {
            for answer in &poll.answers {
                if !union.contains(answer) {
                    union.push(answer.clone());
                }
            }
        }
        let mut pool_counts: Vec<usize> = vec![0; union.len()];
        let mut pool_totals: (usize, usize, usize, usize) = (0, 0, 0, 0);
        for (_, poll) in question_askings {
            let juries = poll_juries(poll);
            let all: Vec<_> = juries
                .iter()
                .flat_map(|(_, rs)| rs.iter().copied())
                .collect();
            add_reading(
                &mut pool_counts,
                &mut pool_totals,
                &union,
                &jury_reading(poll, &all),
            );
        }
        pooled_pluralities.push(plurality_of(&pool_counts, pool_totals.1, &union));

        parts_by_question.push(parts);
    }

    let mut lines: Vec<String> = Vec::new();
    let mut total_parts = 0usize;
    let mut total_with_a_plurality = 0usize;
    let mut total_tied = 0usize;
    let mut total_with_no_valid_response = 0usize;
    let mut total_in_accord = 0usize;
    let mut total_with_the_pool = 0usize;
    for (set_index, (label, polls)) in sets.iter().enumerate() {
        // The counts of the set over the whole record,
        // every poll of the set summed across its
        // juries.
        let mut totals: (usize, usize, usize, usize) = (0, 0, 0, 0);
        for poll in polls {
            let juries = poll_juries(poll);
            let all: Vec<_> = juries
                .iter()
                .flat_map(|(_, rs)| rs.iter().copied())
                .collect();
            let reading = jury_reading(poll, &all);
            totals.0 += reading.recorded;
            totals.1 += reading.valid;
            totals.2 += reading.abstained;
            totals.3 += reading.spoiled;
        }
        lines.push(format!(
            "set, {}, {}",
            label.display(),
            counts_text(totals.0, totals.1, totals.2, totals.3)
        ));

        let mut questions = 0usize;
        let mut with_a_plurality = 0usize;
        let mut tied = 0usize;
        let mut with_no_valid_response = 0usize;
        let mut in_accord = 0usize;
        let mut with_the_pool = 0usize;
        for (question_index, question) in gathered.questions.iter().enumerate() {
            let part = match parts_by_question[question_index]
                .iter()
                .find(|(held, _)| *held == set_index)
            {
                Some((_, part)) => part,
                None => continue,
            };
            lines.push(format!("question, {}", question.display()));
            lines.push(part.plurality_line.clone());
            lines.push(part.margin_line.clone());
            questions += 1;
            match &part.plurality {
                Some(answer) => {
                    with_a_plurality += 1;
                    if accords[question_index].as_ref() == Some(answer) {
                        in_accord += 1;
                    }
                    if pooled_pluralities[question_index].as_ref() == Some(answer) {
                        with_the_pool += 1;
                    }
                }
                None if part.valid > 0 => {
                    tied += 1;
                }
                None => {
                    with_no_valid_response += 1;
                }
            }
        }
        lines.push(format!(
            "standing, {} questions, {} with a plurality, {} tied, {} with no valid response, {} in accord, {} with the pool",
            questions, with_a_plurality, tied, with_no_valid_response, in_accord, with_the_pool
        ));
        total_parts += questions;
        total_with_a_plurality += with_a_plurality;
        total_tied += tied;
        total_with_no_valid_response += with_no_valid_response;
        total_in_accord += in_accord;
        total_with_the_pool += with_the_pool;
    }
    lines.push(format!(
        "standings, {} sets, {} parts, {} with a plurality, {} tied, {} with no valid response, {} in accord, {} with the pool",
        sets.len(),
        total_parts,
        total_with_a_plurality,
        total_tied,
        total_with_no_valid_response,
        total_in_accord,
        total_with_the_pool
    ));
    Ok(lines)
}
