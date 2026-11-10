//! The margin reading over the recorded poll sets.
//!
//! The reading reads the whole record question by
//! question. For each question it states the margin by
//! which the pooled plurality of the question stands
//! and, for each jury at the question, the margin by
//! which the plurality of the jury's summed block
//! stands, the margin of a pool standing as the count
//! of its plurality answer minus the largest count
//! among the other offered answers, read over the
//! summed counts of the pool, and standing only where
//! a plurality stands for the pool, else the words not
//! counted. It carries no judgement about any answer
//! or any jury, and margin names only how far a
//! plurality stands from the closest count behind it.
//! The obligations this module carries are GJ-35 and
//! GJ-36 in CONTRACT.md, in the Round 17 section.

use num_bigint::BigInt;
use num_rational::Ratio;

use crate::agreement::{
    add_reading, jury_reading, plurality_of, poll_juries, responses_of, JuryReading,
};
use crate::polls::Poll;
use crate::pytext::PText;
use crate::record::{check_poll_text, collect_questions, validate_sets};

/// Return the margin reading over a list of recorded
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
/// line, then the pooled part of the section, the
/// counts line of the whole question under the word
/// pool with the plurality line and the margin line
/// of the pooled plurality, then one block per jury
/// of the question in the order the juries first
/// appear at it, the jury line carrying the counts of
/// the jury summed over the askings of the question
/// with its own plurality line and margin line. The
/// closing line counts the questions, the blocks, the
/// blocks at which a plurality stands, and the blocks
/// whose plurality stands at a margin of one.
pub fn margin_lines(sets: &[(PText, Vec<Poll>)]) -> Result<Vec<String>, String> {
    for (_, polls) in sets {
        check_poll_text(polls)?;
    }
    validate_sets(sets)?;
    let gathered = collect_questions(sets);

    let mut lines: Vec<String> = Vec::new();
    let mut blocks = 0usize;
    let mut pluralities = 0usize;
    let mut at_one = 0usize;
    for (question, question_askings) in gathered.questions.iter().zip(gathered.askings.iter()) {
        // The offered answers of the question, the union
        // in asking order, as the canvass builds it.
        let mut union: Vec<PText> = Vec::new();
        for (_, poll) in question_askings {
            for answer in &poll.answers {
                if !union.contains(answer) {
                    union.push(answer.clone());
                }
            }
        }

        lines.push(format!("question, {}", question.display()));

        // The pool of the question, every asking's
        // responses summed as the aggregate sums them.
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
        let pool_plurality = plurality_of(&pool_counts, pool_totals.1, &union);
        let pool_reading = JuryReading {
            recorded: pool_totals.0,
            valid: pool_totals.1,
            abstained: pool_totals.2,
            spoiled: pool_totals.3,
            rows: union
                .iter()
                .zip(pool_counts.iter())
                .map(|(answer, count)| {
                    let share = if pool_totals.1 > 0 {
                        Ratio::new(BigInt::from(*count), BigInt::from(pool_totals.1))
                    } else {
                        Ratio::new(BigInt::from(0), BigInt::from(1))
                    };
                    (answer.clone(), *count, share)
                })
                .collect(),
            plurality: pool_plurality.clone(),
        };
        lines.push(format!("pool, {}", pool_reading.counts_text()));
        lines.push(format!(
            "plurality, {}",
            match &pool_plurality {
                Some(answer) => answer.display(),
                None => "none".to_string(),
            }
        ));
        lines.push(match pool_reading.margin() {
            Some(figure) => format!("margin, {}", figure),
            None => "margin, not counted".to_string(),
        });

        // The juries of the question, in the order they
        // first appear at it, each block the jury's
        // readings summed over the askings it stands at.
        let mut models: Vec<PText> = Vec::new();
        for (_, poll) in question_askings {
            for (model, _) in poll_juries(poll) {
                if !models.contains(&model) {
                    models.push(model);
                }
            }
        }
        for model in &models {
            let mut counts: Vec<usize> = vec![0; union.len()];
            let mut totals: (usize, usize, usize, usize) = (0, 0, 0, 0);
            for (_, poll) in question_askings {
                let juries = poll_juries(poll);
                add_reading(
                    &mut counts,
                    &mut totals,
                    &union,
                    &jury_reading(poll, &responses_of(&juries, model)),
                );
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
            blocks += 1;
            if plurality.is_some() {
                pluralities += 1;
                if reading.margin() == Some(1) {
                    at_one += 1;
                }
            }
            lines.push(format!(
                "jury, {}, {}",
                model.display(),
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
    }
    lines.push(format!(
        "margins, {} questions, {} blocks, {} pluralities, {} at one",
        gathered.questions.len(),
        blocks,
        pluralities,
        at_one
    ));
    Ok(lines)
}
