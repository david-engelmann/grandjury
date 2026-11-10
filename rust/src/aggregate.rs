//! The aggregate reading over the recorded poll sets.
//!
//! The reading reads the whole record question by
//! question and pools each question across its askings.
//! For each question it states the counts of all its
//! askings summed as one pool, the count and the share
//! of every offered answer of the question among its
//! pooled valid responses, and the pooled plurality of
//! the question, so the counts a question draws over its
//! whole life in the record stand shown rather than left
//! for a reader to sum from the pool blocks by hand, the
//! checkability GJ-10 states. It carries no judgement
//! about any answer or any jury, and an aggregate names
//! only the counts of the record. The obligations this
//! module carries are GJ-27 and GJ-28 in CONTRACT.md, in
//! the Round 13 section.

use num_bigint::BigInt;
use num_rational::Ratio;

use crate::agreement::{jury_reading, plurality_of, JuryReading};
use crate::polls::{Poll, Response};
use crate::pytext::PText;
use crate::record::{check_poll_text, collect_questions, validate_sets, Questions};

/// Return the aggregate reading over a list of recorded
/// poll sets as lines, or the defect the first failed
/// rule names.
///
/// Each set pairs one set label with its polls. The set
/// rules of GJ-11 govern the sets unchanged, each label
/// checked before the questions of its set and the first
/// failure the refusal, and a refusal returns no lines
/// at all. One section stands per question in the order
/// the questions first appear across the sets, opening
/// with the question line of the agreement form, then
/// the counts of the whole question summed over its
/// askings, one line per offered answer of the question
/// in the union order of its askings, and the plurality
/// line of the question. The pooled counts of a question
/// are the sums over its askings of the pooled counts
/// GJ-25 states for an asking, summed count by count
/// through the jury reading of GJ-8, so the reading
/// holds no second counting of a response, and its rows
/// render through the shared row rendering of the
/// agreement reading from the summed counts, so no share
/// of the reading is a sum of shares. The plurality of
/// a question is the application GJ-27 states in its own
/// words, the offered answer holding the largest count
/// among the pooled valid responses of every asking of
/// the question, standing only when the question holds
/// at least one valid response across its askings and
/// one answer holds that count alone. No line of the
/// reading names a set label. The closing line counts
/// the questions of the reading, the askings of the
/// reading, the valid responses of the record, and the
/// questions at which a plurality stands.
pub fn aggregate_lines(sets: &[(PText, Vec<Poll>)]) -> Result<Vec<String>, String> {
    for (_, polls) in sets {
        check_poll_text(polls)?;
    }
    validate_sets(sets)?;

    // The questions of the reading and their askings, as
    // the shared collection of the record module gathers
    // them.
    let Questions { questions, askings } = collect_questions(sets);

    let mut lines: Vec<String> = Vec::new();
    let mut asking_total = 0usize;
    let mut valid_total = 0usize;
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
        // The pooled counts of the question, summed over
        // its askings count by count from the pooled
        // reading GJ-25 states for each asking.
        let mut summed: Vec<usize> = vec![0; union.len()];
        let mut recorded = 0usize;
        let mut valid = 0usize;
        let mut abstained = 0usize;
        let mut spoiled = 0usize;
        for (_, poll) in question_askings {
            let responses: Vec<&Response> = poll.responses.iter().collect();
            let reading = jury_reading(poll, &responses);
            recorded += reading.recorded;
            valid += reading.valid;
            abstained += reading.abstained;
            spoiled += reading.spoiled;
            for (answer, count, _) in &reading.rows {
                let slot = union
                    .iter()
                    .position(|held| held == answer)
                    .expect("an offered answer of the union");
                summed[slot] += count;
            }
            asking_total += 1;
        }
        let mut rows = Vec::new();
        for (answer, count) in union.iter().zip(summed.iter()) {
            let share = if valid > 0 {
                Ratio::new(BigInt::from(*count), BigInt::from(valid))
            } else {
                Ratio::from(BigInt::from(0))
            };
            rows.push((answer.clone(), *count, share));
        }
        let plurality = plurality_of(&summed, valid, &union);
        let reading = JuryReading {
            recorded,
            valid,
            abstained,
            spoiled,
            rows,
            plurality: plurality.clone(),
        };
        lines.push(format!("pool, {}", reading.counts_text()));
        lines.extend(reading.row_lines());
        lines.push(format!(
            "plurality, {}",
            match &plurality {
                Some(answer) => answer.display(),
                None => "none".to_string(),
            }
        ));
        valid_total += valid;
        if plurality.is_some() {
            plurality_total += 1;
        }
    }
    lines.push(format!(
        "aggregates, {} questions, {} askings, {} valid, {} pluralities",
        questions.len(),
        asking_total,
        valid_total,
        plurality_total
    ));
    Ok(lines)
}
