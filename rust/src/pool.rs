//! The pool reading over the recorded poll sets.
//!
//! The reading reads the whole record question by
//! question. For each asking of a question it states the
//! counts of the whole poll, the count and the share of
//! every offered answer among the pooled valid responses
//! of the asking, and the pooled plurality of the asking,
//! so the pooled plurality lines the swing and the
//! minority state stand checkable against the counts
//! they rest on, the checkability GJ-10 states for its
//! own shares. It carries no judgement about any answer
//! or any jury, and a pool names only the counts of the
//! record. The obligations this module carries are
//! GJ-25 and GJ-26 in CONTRACT.md, in the Round 12
//! section.

use crate::agreement::jury_reading;
use crate::polls::{Poll, Response};
use crate::pytext::PText;
use crate::record::{check_poll_text, collect_questions, validate_sets, Questions};

/// Return the pool reading over a list of recorded poll
/// sets as lines, or the defect the first failed rule
/// names.
///
/// Each set pairs one set label with its polls. The set
/// rules of GJ-11 govern the sets unchanged, each label
/// checked before the questions of its set and the first
/// failure the refusal, and a refusal returns no lines
/// at all. One section stands per question in the order
/// the questions first appear across the sets, opening
/// with the question line of the agreement form, then
/// one block per asking of the question in asking
/// order, the counts of the whole poll of the asking,
/// one line per offered answer in offered order, and
/// the plurality line of the asking. The pooled counts
/// of an asking are the counts the jury reading of GJ-8
/// computes read over every response of the asking's
/// poll as one pool, and the plurality of an asking is
/// the pooled plurality of GJ-17. The closing line
/// counts the sections of the reading, the askings of
/// the reading, the valid responses of the record, and
/// the askings at which a pooled plurality stands.
pub fn pool_lines(sets: &[(PText, Vec<Poll>)]) -> Result<Vec<String>, String> {
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
        for (set_index, poll) in question_askings {
            let label = &sets[*set_index].0;
            // The pool of the asking is every response of
            // the poll, read through the standing jury
            // reading as one pool.
            let responses: Vec<&Response> = poll.responses.iter().collect();
            let reading = jury_reading(poll, &responses);
            asking_total += 1;
            valid_total += reading.valid;
            if reading.plurality.is_some() {
                plurality_total += 1;
            }
            lines.push(format!(
                "set, {}, {}",
                label.display(),
                reading.counts_text()
            ));
            lines.extend(reading.row_lines());
            lines.push(format!(
                "plurality, {}, {}",
                label.display(),
                match &reading.plurality {
                    Some(answer) => answer.display(),
                    None => "none".to_string(),
                }
            ));
        }
    }
    lines.push(format!(
        "pools, {} questions, {} askings, {} valid, {} pluralities",
        questions.len(),
        asking_total,
        valid_total,
        plurality_total
    ));
    Ok(lines)
}
