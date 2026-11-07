//! The minority reading over the recorded poll sets.
//!
//! The reading reads the whole record question by
//! question. For each asking of a question it states the
//! plurality of the whole poll, the rule of GJ-8 as
//! FIX-GJ-8 repairs it read over the valid responses of
//! every jury of the poll together, and the margin that
//! plurality stood by over the largest count among the
//! other offered answers, and it names the respondents
//! of the poll whose valid answer was not the pooled
//! plurality answer, in the order those respondents
//! first appear across the record. Where no plurality
//! stands the block names no dissenter and its margin
//! reads not counted. It carries no judgement about any
//! respondent or any jury, and a dissenter of the pool
//! names only that an answer differed from the
//! plurality of the whole poll. The obligations this
//! module carries are GJ-19 and GJ-20 in CONTRACT.md,
//! in the Round 9 section.

use crate::agreement::jury_reading;
use crate::polls::{Poll, Response};
use crate::pytext::PText;
use crate::record::{
    check_poll_text, collect_questions, collect_respondents, dissenters_of, validate_sets,
    Questions,
};

/// Return the minority reading over a list of recorded
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
/// one asking block per asking of the question in
/// asking order, the plurality line with the set label
/// standing behind the word plurality, then the margin
/// line with the label behind the word margin, then one
/// dissenter line per dissenter of the asking in the
/// order the dissenters first appear across the whole
/// record. The closing line counts the sections of the
/// reading, the asking blocks, the blocks at which a
/// pooled plurality stood, and the dissenter lines.
pub fn minority_lines(sets: &[(PText, Vec<Poll>)]) -> Result<Vec<String>, String> {
    for (_, polls) in sets {
        check_poll_text(polls)?;
    }
    validate_sets(sets)?;

    // The questions of the reading and their askings, as
    // the shared collection of the record module gathers
    // them, and the respondents of the record in the
    // order their juror strings first appear across it,
    // the order the dissenter lines of a block stand in.
    let Questions { questions, askings } = collect_questions(sets);
    let respondents = collect_respondents(sets);

    let mut lines: Vec<String> = Vec::new();
    let mut asking_total = 0usize;
    let mut pluralities = 0usize;
    let mut dissenters_total = 0usize;
    for (question, question_askings) in questions.iter().zip(askings.iter()) {
        lines.push(format!("question, {}", question.display()));
        for (set_index, poll) in question_askings {
            let label = &sets[*set_index].0;
            // The pool of the asking is every response of
            // the poll, read through the standing jury
            // reading as one pool, as the swing reads it.
            let responses: Vec<&Response> = poll.responses.iter().collect();
            let reading = jury_reading(poll, &responses);
            asking_total += 1;
            lines.push(format!(
                "plurality, {}, {}",
                label.display(),
                match &reading.plurality {
                    Some(answer) => answer.display(),
                    None => "none".to_string(),
                }
            ));
            match &reading.plurality {
                Some(plurality) => {
                    pluralities += 1;
                    lines.push(format!(
                        "margin, {}, {}",
                        label.display(),
                        reading
                            .margin()
                            .expect("the margin of a standing plurality")
                    ));
                    for juror in dissenters_of(poll, &responses, plurality, &respondents) {
                        lines.push(format!("dissenter, {}", juror.display()));
                        dissenters_total += 1;
                    }
                }
                None => {
                    lines.push(format!("margin, {}, not counted", label.display()));
                }
            }
        }
    }
    lines.push(format!(
        "minorities, {} questions, {} askings, {} pluralities, {} dissenters",
        questions.len(),
        asking_total,
        pluralities,
        dissenters_total
    ));
    Ok(lines)
}
