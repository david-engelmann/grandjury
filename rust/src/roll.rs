//! The roll reading over the recorded poll sets.
//!
//! The reading reads the whole record jury by jury and
//! states, for each jury, the block of the canvass at
//! every question the jury stands at, gathered under
//! the jury's own head. For each question a jury stands
//! at it states the counts of the jury summed over the
//! askings of the question, the count and the share of
//! every offered answer of the question among the jury's
//! summed valid responses, and the plurality of the jury
//! at the question, and the section of the jury closes
//! with the standing of the jury over the record, the
//! questions it stands at, the blocks at which a
//! plurality stands, and the blocks whose standing
//! plurality names the plurality of their question, so
//! the summed account of one jury over the whole record
//! stands shown per jury rather than left for a reader
//! to regroup from the canvass sections by hand. It
//! carries no judgement about any answer or any jury,
//! and a roll names only the counts of the record. The
//! obligations this module carries are GJ-31 and GJ-32
//! in CONTRACT.md, in the Round 15 section.

use num_bigint::BigInt;
use num_rational::Ratio;

use crate::agreement::{
    add_reading, jury_reading, plurality_of, poll_juries, responses_of, JuryReading,
};
use crate::polls::{Poll, Response};
use crate::pytext::PText;
use crate::record::{check_poll_text, collect_questions, validate_sets, Questions};

/// Return the roll reading over a list of recorded
/// poll sets as lines, or the defect the first failed
/// rule names.
///
/// Each set pairs one set label with its polls. The set
/// rules of GJ-11 govern the sets unchanged, each label
/// checked before the questions of its set and the
/// first failure the refusal, and a refusal returns no
/// lines at all. One section stands per jury in the
/// order the model strings first appear across the
/// sets, opening with the model line of the bench form,
/// then one block per question the jury stands at, in
/// question order, a jury standing at a question where
/// the poll of at least one asking of the question
/// holds at least one response of that jury. A block is
/// the block of the canvass whole, the question line of
/// the census form, the counts of the jury summed over
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
/// and one answer holds that count alone. A section
/// closes with the standing line of the jury, and no
/// totals line stands inside a section, the totals of
/// a jury binding to its blocks by relation to the
/// census instead. No line of the reading names a set
/// label. The closing line counts the juries of the
/// reading, the blocks of the reading, the blocks at
/// which a plurality stands, and the blocks whose
/// standing plurality names the plurality of their
/// question.
pub fn roll_lines(sets: &[(PText, Vec<Poll>)]) -> Result<Vec<String>, String> {
    for (_, polls) in sets {
        check_poll_text(polls)?;
    }
    validate_sets(sets)?;

    // The questions of the reading and their askings, as
    // the shared collection of the record module gathers
    // them.
    let Questions { questions, askings } = collect_questions(sets);

    // The juries of the reading, in the order their
    // model strings first appear across the sets, as
    // GJ-23 gathers the models of the bench.
    let mut models: Vec<PText> = Vec::new();
    for (_, polls) in sets {
        for poll in polls {
            for response in &poll.responses {
                if !models.contains(&response.model) {
                    models.push(response.model.clone());
                }
            }
        }
    }

    // The offered answers of each question, the union of
    // the offered answers of its askings in asking
    // order, and the plurality of each question read
    // from the pooled counts of the whole question
    // summed over its askings from the reading of each
    // asking over all its responses, the standing every
    // block of the question reads against.
    let mut unions: Vec<Vec<PText>> = Vec::new();
    let mut question_pluralities: Vec<Option<PText>> = Vec::new();
    for question_askings in askings.iter() {
        let mut union: Vec<PText> = Vec::new();
        for (_, poll) in question_askings {
            for answer in &poll.answers {
                if !union.contains(answer) {
                    union.push(answer.clone());
                }
            }
        }
        let mut pooled: Vec<usize> = vec![0; union.len()];
        let mut pooled_counts = (0usize, 0usize, 0usize, 0usize);
        for (_, poll) in question_askings {
            let responses: Vec<&Response> = poll.responses.iter().collect();
            let reading = jury_reading(poll, &responses);
            add_reading(&mut pooled, &mut pooled_counts, &union, &reading);
        }
        question_pluralities.push(plurality_of(&pooled, pooled_counts.1, &union));
        unions.push(union);
    }

    let mut lines: Vec<String> = Vec::new();
    let mut block_total = 0usize;
    let mut plurality_total = 0usize;
    let mut with_total = 0usize;
    for model in &models {
        lines.push(format!("model, {}", model.display()));
        let mut standing_blocks = 0usize;
        let mut standing_pluralities = 0usize;
        let mut standing_with = 0usize;
        for ((question, question_askings), (union, question_plurality)) in questions
            .iter()
            .zip(askings.iter())
            .zip(unions.iter().zip(question_pluralities.iter()))
        {
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
                add_reading(&mut summed, &mut counts, union, &reading);
            }
            // A jury stands at a question only where at
            // least one asking of the question holds at
            // least one response of it, so a jury whose
            // summed recorded count stands at zero holds
            // no block at the question.
            if counts.0 == 0 {
                continue;
            }
            let plurality = plurality_of(&summed, counts.1, union);
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
            lines.push(format!("question, {}", question.display()));
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
            standing_blocks += 1;
            block_total += 1;
            if plurality.is_some() {
                standing_pluralities += 1;
                plurality_total += 1;
                if plurality == *question_plurality {
                    standing_with += 1;
                    with_total += 1;
                }
            }
        }
        lines.push(format!(
            "standing, {} questions, {} pluralities, {} with the plurality",
            standing_blocks, standing_pluralities, standing_with
        ));
    }
    lines.push(format!(
        "rolls, {} juries, {} blocks, {} pluralities, {} with the plurality",
        models.len(),
        block_total,
        plurality_total,
        with_total
    ));
    Ok(lines)
}
