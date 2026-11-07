//! The swing reading over the recorded poll sets.
//!
//! The reading reads the whole record question by
//! question. For each asking of a question it states the
//! plurality of the whole poll, the rule of GJ-8 as
//! FIX-GJ-8 repairs it read over the valid responses of
//! every jury of the poll together, and the margin that
//! plurality stood by over the largest count among the
//! other offered answers, and it closes each question
//! with its course, held where the pooled plurality
//! never changed across the askings at which one stood,
//! swung where two askings hold different standing
//! pluralities, and never decided where no plurality
//! stood at any asking. It carries no judgement about
//! any answer or any jury, and a swing names only that
//! a pooled plurality changed. The obligations this
//! module carries are GJ-17 and GJ-18 in CONTRACT.md,
//! in the Round 8 section.

use crate::agreement::jury_reading;
use crate::polls::{Poll, Response};
use crate::pytext::PText;
use crate::record::{check_poll_text, collect_questions, validate_sets, Questions};

/// Return the swing reading over a list of recorded
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
/// one asking line per asking of the question in asking
/// order, the set label standing behind the word asking
/// and the margin of the asking standing behind the word
/// margin inside the same line, then the course line of
/// the question. The closing line counts the sections
/// of the reading, the course lines by their course, and
/// the asking lines of the reading.
pub fn swing_lines(sets: &[(PText, Vec<Poll>)]) -> Result<Vec<String>, String> {
    for (_, polls) in sets {
        check_poll_text(polls)?;
    }
    validate_sets(sets)?;

    // The questions of the reading and their askings, as
    // the shared collection of the record module gathers
    // them.
    let Questions { questions, askings } = collect_questions(sets);

    let mut lines: Vec<String> = Vec::new();
    let mut held = 0usize;
    let mut swung = 0usize;
    let mut never_decided = 0usize;
    let mut asking_total = 0usize;
    for (question, question_askings) in questions.iter().zip(askings.iter()) {
        lines.push(format!("question, {}", question.display()));
        // The standing pooled pluralities of the question,
        // in asking order, an asking at which no plurality
        // stands standing in no place among them.
        let mut standing: Vec<PText> = Vec::new();
        for (set_index, poll) in question_askings {
            let label = &sets[*set_index].0;
            // The pool of the asking is every response of
            // the poll, read through the standing jury
            // reading as one pool.
            let responses: Vec<&Response> = poll.responses.iter().collect();
            let reading = jury_reading(poll, &responses);
            asking_total += 1;
            match (&reading.plurality, reading.margin()) {
                (Some(answer), Some(margin)) => {
                    lines.push(format!(
                        "asking, {}, {}, margin {}",
                        label.display(),
                        answer.display(),
                        margin
                    ));
                    standing.push(answer.clone());
                }
                _ => {
                    lines.push(format!(
                        "asking, {}, none, margin not counted",
                        label.display()
                    ));
                }
            }
        }
        let course = if standing.is_empty() {
            never_decided += 1;
            "never decided"
        } else if standing.iter().all(|answer| answer == &standing[0]) {
            held += 1;
            "held"
        } else {
            swung += 1;
            "swung"
        };
        lines.push(format!("course, {}", course));
    }
    lines.push(format!(
        "swings, {} questions, {} held, {} swung, {} never decided, {} askings",
        questions.len(),
        held,
        swung,
        never_decided,
        asking_total
    ));
    Ok(lines)
}
