//! The dissent reading over the recorded poll sets.
//!
//! The reading reads the whole record asking by asking.
//! For each jury at each asking it states the plurality
//! of the jury under the rule of GJ-8 as FIX-GJ-8 repairs
//! it, the margin the plurality stood by over the largest
//! count among the other offered answers, and the
//! respondents of the jury whose valid answer was not
//! the plurality answer, in the order those respondents
//! first appear across the record. Where no plurality
//! stands the part names no dissenter and its margin
//! reads not counted. It carries no judgement about any
//! respondent or any jury, and dissent names only that
//! an answer differed from the plurality answer. The
//! obligation this module carries is GJ-15 in
//! CONTRACT.md, in the Round 7 section.

use crate::agreement::{jury_reading, poll_juries};
use crate::polls::Poll;
use crate::pytext::PText;
use crate::record::{check_poll_text, collect_respondents, dissenters_of, validate_sets};

/// Return the dissent reading over a list of recorded
/// poll sets as lines, or the defect the first failed
/// rule names.
///
/// Each set pairs one set label with its polls. The set
/// rules of GJ-11 govern the sets unchanged, each label
/// checked before the questions of its set and the first
/// failure the refusal, and a refusal returns no lines
/// at all. A part stands per jury at one asking, in the
/// order of the sets and of their polls and of the juries
/// of a poll, opening with the question line and the jury
/// line of the agreement form, then the plurality line
/// with the set label standing behind the word plurality,
/// then the margin line with the label behind the word
/// margin, then one dissenter line per dissenter of the
/// part in the order the dissenters first appear across
/// the whole record. The closing line counts the parts
/// of the reading, the parts at which a plurality stood,
/// and the dissenter lines of the reading.
pub fn dissent_lines(sets: &[(PText, Vec<Poll>)]) -> Result<Vec<String>, String> {
    for (_, polls) in sets {
        check_poll_text(polls)?;
    }
    validate_sets(sets)?;

    // The respondents of the record, in the order their
    // juror strings first appear across it, as the shared
    // collection of the record module gathers them, the
    // order the dissenter lines of a part stand in.
    let respondents = collect_respondents(sets);

    let mut lines: Vec<String> = Vec::new();
    let mut questions = 0usize;
    let mut pluralities = 0usize;
    let mut counted = 0usize;
    for (label, polls) in sets {
        for poll in polls {
            for (model, responses) in &poll_juries(poll) {
                let reading = jury_reading(poll, responses);
                questions += 1;
                lines.push(format!("question, {}", poll.question.display()));
                lines.push(format!(
                    "jury, {}, {}",
                    model.display(),
                    reading.counts_text()
                ));
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
                        for juror in dissenters_of(poll, responses, plurality, &respondents) {
                            lines.push(format!("dissenter, {}", juror.display()));
                            counted += 1;
                        }
                    }
                    None => {
                        lines.push(format!("margin, {}, not counted", label.display()));
                    }
                }
            }
        }
    }
    lines.push(format!(
        "dissents, {} questions, {} pluralities, {} counted",
        questions, pluralities, counted
    ));
    Ok(lines)
}
