//! The juror ledger reading over the recorded poll sets.
//!
//! The reading reads the whole record respondent by
//! respondent. For each respondent, one juror string
//! standing in the responses, it states the counts of the
//! respondent in each set it stands in and over the whole
//! record, its own answers and their movement over the
//! questions it was asked more than once, and how often its
//! answer stood with the plurality of its jury, and it
//! closes with the totals of the whole record and the
//! movement counts. It carries no judgement about any
//! respondent, and its movement words name only that an
//! answer stands or has changed across askings. The
//! obligation this module carries is GJ-13 in CONTRACT.md,
//! in the Round 6 section.

use num_bigint::BigInt;
use num_rational::Ratio;

use crate::agreement::{jury_reading, poll_juries, responses_of};
use crate::polls::{fraction_text, Poll, Response};
use crate::pytext::PText;
use crate::record::{
    collect_questions, collect_respondents, is_valid, movement_of, record_line, validate_sets,
    Counts, MovementCounts, Questions,
};

/// Return the juror ledger reading over a list of recorded
/// poll sets as lines, or the defect the first failed rule
/// names.
///
/// Each set pairs one set label with its polls. The set
/// rules of GJ-11 govern the sets unchanged, each label
/// checked before the questions of its set and the first
/// failure the refusal, and a refusal returns no lines at
/// all. The respondents stand in the order their juror
/// strings first appear across the record. A block stands
/// per respondent, opening with the respondent block line
/// of FIX-GJ-13, whose fixed words separate its form from
/// the totals line form of the same block, then one set
/// part per set the respondent stands recorded in in the
/// fenced counts form of FIX-GJ-12, then the totals line
/// of the respondent, then one question part per question
/// the respondent stands recorded in at two or more
/// askings of that question, each part closing with the
/// movement of its own answers, and the agreement line
/// closing the block. After the blocks the record line and
/// the closing line count the movement lines of the
/// reading.
pub fn ledger_lines(sets: &[(PText, Vec<Poll>)]) -> Result<Vec<String>, String> {
    validate_sets(sets)?;

    // The respondents of the record, in the order their
    // juror strings first appear across it, as the shared
    // collection of the record module gathers them.
    let respondents = collect_respondents(sets);

    // The questions of the reading and their askings, as
    // GJ-11 states them for the census and the shared
    // collection of the record module gathers them.
    let Questions { questions, askings } = collect_questions(sets);

    let mut lines: Vec<String> = Vec::new();
    let mut movement_counts = MovementCounts::default();
    // The totals of the whole record, summed through the
    // standing jury reading over every poll of every set,
    // as the census sums them.
    let mut record = Counts::default();
    for (_, polls) in sets {
        for poll in polls {
            for (_, responses) in &poll_juries(poll) {
                record.add_reading(&jury_reading(poll, responses));
            }
        }
    }

    for respondent in &respondents {
        lines.push(format!("respondent block, {}", respondent.display()));
        let mut totals = Counts::default();
        for (label, polls) in sets {
            let mut counts = Counts::default();
            let mut stands = false;
            for poll in polls {
                let mine: Vec<&Response> = poll
                    .responses
                    .iter()
                    .filter(|response| &response.juror == respondent)
                    .collect();
                if !mine.is_empty() {
                    stands = true;
                    counts.add_reading(&jury_reading(poll, &mine));
                }
            }
            if stands {
                lines.push(format!("set, {}, {}", label.display(), counts.text()));
                totals.add_counts(&counts);
            }
        }
        lines.push(format!(
            "respondent, {}, {}",
            respondent.display(),
            totals.text()
        ));
        for (question, question_askings) in questions.iter().zip(askings.iter()) {
            let mine: Vec<(&PText, &Poll, &Response)> = question_askings
                .iter()
                .filter_map(|(set_index, poll)| {
                    poll.responses
                        .iter()
                        .find(|response| &response.juror == respondent)
                        .map(|response| (&sets[*set_index].0, *poll, response))
                })
                .collect();
            if mine.len() < 2 {
                continue;
            }
            lines.push(format!("question, {}", question.display()));
            let mut valid_answers: Vec<PText> = Vec::new();
            for (label, poll, response) in &mine {
                if is_valid(poll, response) {
                    valid_answers.push(response.answer.clone());
                    lines.push(format!(
                        "  {}, {}",
                        label.display(),
                        response.answer.display()
                    ));
                } else {
                    lines.push(format!("  {}, none", label.display()));
                }
            }
            let movement = movement_of(&valid_answers);
            movement_counts.add(movement);
            lines.push(format!(
                "movement, {}, {}",
                respondent.display(),
                movement.word()
            ));
        }
        let mut matched = 0usize;
        let mut base = 0usize;
        for (_, polls) in sets {
            for poll in polls {
                let mine = match poll
                    .responses
                    .iter()
                    .find(|response| &response.juror == respondent)
                {
                    Some(response) => response,
                    None => continue,
                };
                if !is_valid(poll, mine) {
                    continue;
                }
                let juries = poll_juries(poll);
                let reading = jury_reading(poll, &responses_of(&juries, &mine.model));
                if let Some(plurality) = &reading.plurality {
                    base += 1;
                    if *plurality == mine.answer {
                        matched += 1;
                    }
                }
            }
        }
        let share = if base > 0 {
            Ratio::new(BigInt::from(matched), BigInt::from(base))
        } else {
            Ratio::from(BigInt::from(0))
        };
        lines.push(format!(
            "agreement, {} of {}, {}",
            matched,
            base,
            fraction_text(&share)
        ));
    }

    lines.push(record_line(
        sets.iter().map(|(_, polls)| polls.len()).sum(),
        &record,
    ));
    lines.push(movement_counts.line());
    Ok(lines)
}
