//! The canvass reading over the recorded poll sets.
//!
//! The reading reads the whole record question by
//! question and states, for each question, the reading
//! of each jury at that question summed over the
//! question's askings. For each jury of a question it
//! states the counts of the jury summed over the
//! askings it stands at, the count and the share of
//! every offered answer of the question among the
//! jury's summed valid responses, and the plurality of
//! the jury at the question, so the counts a jury draws
//! over the whole life of one question stand shown per
//! jury rather than left for a reader to sum from the
//! census parts by hand, the checkability GJ-10
//! states. It carries no judgement about any answer or
//! any jury, and a canvass names only the counts of the
//! record. The obligations this module carries are
//! GJ-29 and GJ-30 in CONTRACT.md, in the Round 14
//! section.

use num_bigint::BigInt;
use num_rational::Ratio;

use crate::agreement::{
    add_reading, jury_reading, plurality_of, poll_juries, responses_of, JuryReading,
};
use crate::polls::{Poll, Response};
use crate::pytext::PText;
use crate::record::{check_poll_text, collect_questions, validate_sets, Questions};

/// Return the canvass reading over a list of recorded
/// poll sets as lines, or the defect the first failed
/// rule names.
///
/// Each set pairs one set label with its polls. The set
/// rules of GJ-11 govern the sets unchanged, each label
/// checked before the questions of its set and the
/// first failure the refusal, and a refusal returns no
/// lines at all. One section stands per question in the
/// order the questions first appear across the sets,
/// opening with the question line of the census form,
/// then one block per jury of the question in the order
/// the model strings first appear across its askings.
/// A block states the counts of the jury summed over
/// the askings of the question, one line per offered
/// answer of the question in the union order of its
/// askings, and the plurality line of the jury at the
/// question. The summed counts of a block are the sums
/// over the askings of the question of the jury
/// readings GJ-8 states for that jury at each asking,
/// summed count by count through those readings, so
/// the reading holds no second counting of a response,
/// and its rows render through the shared row
/// rendering of the agreement reading from the summed
/// counts, so no share of the reading is a sum of
/// shares. The plurality of a jury at a question is
/// the application GJ-29 states in its own words, the
/// offered answer holding the largest count among the
/// valid responses of the jury summed over every
/// asking of the question, standing only when the jury
/// holds at least one valid response across the askings
/// and one answer holds that count alone. No line of
/// the reading names a set label. The closing line
/// counts the questions of the reading, the jury
/// blocks of the reading, the blocks at which a
/// plurality stands, and the blocks whose standing
/// plurality names the plurality of their question.
pub fn canvass_lines(sets: &[(PText, Vec<Poll>)]) -> Result<Vec<String>, String> {
    for (_, polls) in sets {
        check_poll_text(polls)?;
    }
    validate_sets(sets)?;

    // The questions of the reading and their askings, as
    // the shared collection of the record module gathers
    // them.
    let Questions { questions, askings } = collect_questions(sets);

    let mut lines: Vec<String> = Vec::new();
    let mut block_total = 0usize;
    let mut plurality_total = 0usize;
    let mut with_total = 0usize;
    for (question, question_askings) in questions.iter().zip(askings.iter()) {
        lines.push(format!("question, {}", question.display()));
        // The offered answers of the question are the
        // union of the offered answers of its askings, in
        // the order they first appear across the askings
        // in asking order, each asking's answers in
        // offered order.
        let mut union: Vec<PText> = Vec::new();
        for (_, poll) in question_askings {
            for answer in &poll.answers {
                if !union.contains(answer) {
                    union.push(answer.clone());
                }
            }
        }
        // The juries of the question, in the order their
        // model strings first appear across its askings.
        let mut models: Vec<PText> = Vec::new();
        for (_, poll) in question_askings {
            for (model, _) in &poll_juries(poll) {
                if !models.contains(model) {
                    models.push(model.clone());
                }
            }
        }
        // The pooled counts of the whole question, summed
        // over its askings from the reading of each
        // asking over all its responses, and the
        // plurality of the question read from them, the
        // standing every block of the question reads
        // against.
        let mut pooled: Vec<usize> = vec![0; union.len()];
        let mut pooled_counts = (0usize, 0usize, 0usize, 0usize);
        for (_, poll) in question_askings {
            let responses: Vec<&Response> = poll.responses.iter().collect();
            let reading = jury_reading(poll, &responses);
            add_reading(&mut pooled, &mut pooled_counts, &union, &reading);
        }
        let question_plurality = plurality_of(&pooled, pooled_counts.1, &union);
        for model in &models {
            // The counts of this jury at this question,
            // summed over the askings of the question from
            // the jury reading GJ-8 states for the jury at
            // each asking, an asking the jury does not
            // stand at adding nothing to the sums.
            let mut summed: Vec<usize> = vec![0; union.len()];
            let mut counts = (0usize, 0usize, 0usize, 0usize);
            for (_, poll) in question_askings {
                let juries = poll_juries(poll);
                let reading = jury_reading(poll, &responses_of(&juries, model));
                add_reading(&mut summed, &mut counts, &union, &reading);
            }
            let plurality = plurality_of(&summed, counts.1, &union);
            let mut rows = Vec::new();
            for (answer, count) in union.iter().zip(summed.iter()) {
                let share = if counts.1 > 0 {
                    Ratio::new(BigInt::from(*count), BigInt::from(counts.1))
                } else {
                    Ratio::from(BigInt::from(0))
                };
                rows.push((answer.clone(), *count, share));
            }
            let reading = JuryReading {
                recorded: counts.0,
                valid: counts.1,
                abstained: counts.2,
                spoiled: counts.3,
                rows,
                plurality: plurality.clone(),
            };
            lines.push(format!(
                "jury, {}, {}",
                model.display(),
                reading.counts_text()
            ));
            lines.extend(reading.row_lines());
            lines.push(format!(
                "plurality, {}",
                match &plurality {
                    Some(answer) => answer.display(),
                    None => "none".to_string(),
                }
            ));
            block_total += 1;
            if plurality.is_some() {
                plurality_total += 1;
                if plurality == question_plurality {
                    with_total += 1;
                }
            }
        }
    }
    lines.push(format!(
        "canvasses, {} questions, {} juries, {} pluralities, {} with the plurality",
        questions.len(),
        block_total,
        plurality_total,
        with_total
    ));
    Ok(lines)
}
