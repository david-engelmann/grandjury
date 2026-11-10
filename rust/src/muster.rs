//! The muster reading over the recorded poll sets.
//!
//! The reading reads the whole record question by
//! question and restates the canvass blocks of each
//! question answer by answer. For each question it
//! states, under each offered answer of the question,
//! the row of that answer in the block of every jury of
//! the question, the count of the valid responses of
//! the question that chose the answer, the count of the
//! juries whose summed count of the answer stands at
//! one or more, and the count of the juries whose block
//! plurality the answer is, so which juries stand
//! behind each offered answer of a question and how
//! many of them name it as their plurality stands shown
//! in one gathering rather than left for a reader to
//! pick out of every block of the question by hand. It
//! carries no judgement about any answer or any jury,
//! and a muster names only the counts of the record.
//! The obligations this module carries are GJ-33 and
//! GJ-34 in CONTRACT.md, in the Round 16 section.

use num_bigint::BigInt;
use num_rational::Ratio;

use crate::agreement::{
    add_reading, jury_reading, plurality_of, poll_juries, responses_of, JuryReading,
};
use crate::polls::{Poll, Response};
use crate::pytext::PText;
use crate::record::{check_poll_text, collect_questions, validate_sets, Questions};

/// Return the muster reading over a list of recorded
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
/// then one part per offered answer of the question in
/// the union order of its askings. A part opens with
/// the answer line, then states one row per jury of the
/// question in the order the model strings first appear
/// across its askings, the row of the answer in the
/// block of that jury at that question with the model
/// string of the jury standing where the block's row
/// names the answer, and the part closes with the
/// muster line, the count of the valid responses of the
/// question that chose the answer summed over the
/// juries of the question, the count of the juries
/// whose summed count of the answer stands at one or
/// more, and the count of the juries whose block
/// plurality the answer is. The blocks the reading
/// transposes are the blocks GJ-30 states, the sums
/// over the askings of the question of the jury
/// readings GJ-8 states for each jury at each asking,
/// summed count by count through those readings, so
/// the reading holds no second counting of a response
/// and no second summation, and its rows render through
/// the shared row rendering of the agreement reading
/// from the summed counts of the blocks, so no share
/// of the reading is a sum of shares. No line of the
/// reading names a set label. The closing line counts
/// the questions of the reading, the parts of the
/// reading, the jury rows of the reading, and the
/// pluralities of the parts summed over the reading.
pub fn muster_lines(sets: &[(PText, Vec<Poll>)]) -> Result<Vec<String>, String> {
    for (_, polls) in sets {
        check_poll_text(polls)?;
    }
    validate_sets(sets)?;

    // The questions of the reading and their askings, as
    // the shared collection of the record module gathers
    // them.
    let Questions { questions, askings } = collect_questions(sets);

    let mut lines: Vec<String> = Vec::new();
    let mut part_total = 0usize;
    let mut row_total = 0usize;
    let mut plurality_total = 0usize;
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
        // The block of each jury at the question, the
        // counts of the jury summed over the askings of
        // the question from the jury reading GJ-8 states
        // for the jury at each asking, an asking the jury
        // does not stand at adding nothing to the sums,
        // with the block's plurality read from the summed
        // counts under the application GJ-29 states.
        let mut blocks: Vec<(Vec<usize>, usize, Option<PText>)> = Vec::new();
        for model in &models {
            let mut summed: Vec<usize> = vec![0; union.len()];
            let mut counts = (0usize, 0usize, 0usize, 0usize);
            for (_, poll) in question_askings {
                let juries = poll_juries(poll);
                let reading = jury_reading(poll, &responses_of(&juries, model));
                add_reading(&mut summed, &mut counts, &union, &reading);
            }
            let plurality = plurality_of(&summed, counts.1, &union);
            blocks.push((summed, counts.1, plurality));
        }
        // The pooled counts of the whole question, summed
        // over its askings from the reading of each
        // asking over all its responses, so the responses
        // figure of a part is the count of its answer in
        // the pooled reading of the question.
        let mut pooled: Vec<usize> = vec![0; union.len()];
        let mut pooled_counts = (0usize, 0usize, 0usize, 0usize);
        for (_, poll) in question_askings {
            let responses: Vec<&Response> = poll.responses.iter().collect();
            let reading = jury_reading(poll, &responses);
            add_reading(&mut pooled, &mut pooled_counts, &union, &reading);
        }
        for (slot, answer) in union.iter().enumerate() {
            lines.push(format!("answer, {}", answer.display()));
            let mut backers = 0usize;
            let mut carriers = 0usize;
            for (model, (summed, valid, plurality)) in models.iter().zip(blocks.iter()) {
                let count = summed[slot];
                let share = if *valid > 0 {
                    Ratio::new(BigInt::from(count), BigInt::from(*valid))
                } else {
                    Ratio::from(BigInt::from(0))
                };
                let reading = JuryReading {
                    recorded: 0,
                    valid: *valid,
                    abstained: 0,
                    spoiled: 0,
                    rows: vec![(model.clone(), count, share)],
                    plurality: plurality.clone(),
                };
                lines.extend(reading.row_lines());
                row_total += 1;
                if count >= 1 {
                    backers += 1;
                }
                if plurality.as_ref() == Some(answer) {
                    carriers += 1;
                }
            }
            lines.push(format!(
                "muster, {} responses, {} juries, {} pluralities",
                pooled[slot], backers, carriers
            ));
            part_total += 1;
            plurality_total += carriers;
        }
    }
    lines.push(format!(
        "musters, {} questions, {} answers, {} juries, {} pluralities",
        questions.len(),
        part_total,
        row_total,
        plurality_total
    ));
    Ok(lines)
}
