//! The bench reading over the recorded poll sets.
//!
//! The reading reads across the recorded sets by the model
//! that answered. For each model it states, at each asking
//! of each question the model answered, the plurality of
//! its own responses pooled over the juror names it
//! answered under, the count of its valid answers, and how
//! many of those valid answers stood with the pooled
//! plurality of the whole poll at that asking. It carries
//! no judgement about any model, and standing with the
//! pooled plurality names only that an answer was the
//! pooled plurality answer of its asking. The obligations
//! this module carries are GJ-23 and GJ-24 in CONTRACT.md,
//! in the Round 11 section.

use crate::agreement::{jury_reading, poll_juries, responses_of};
use crate::polls::{Poll, Response};
use crate::pytext::PText;
use crate::record::{check_poll_text, collect_questions, validate_sets, Questions};

/// Return the bench reading over a list of recorded poll
/// sets as lines, or the defect the first failed rule names.
///
/// Each set pairs one set label with its polls under the
/// set rules of GJ-11. The poll text guard stands before
/// those rules, as it stands in the set readings of the
/// earlier rounds, and a refusal returns no lines at all.
/// The sections stand one per model in the order the model
/// strings first appear across the sets. Inside a section
/// one question line stands per question the model
/// answered, in the census question order, and one asking
/// line stands per asking of that question at which the
/// model answered, in asking order. The reading of a model
/// at an asking is the jury reading of GJ-8 over every
/// response of the poll holding that model string, pooled
/// over the juror names the model answered under. The
/// pooled plurality of an asking is the plurality of GJ-8
/// read over the valid responses of the whole poll. The
/// closing line counts the sections, the valid figures of
/// the asking lines, and their figures for standing with
/// the pooled plurality.
pub fn bench_lines(sets: &[(PText, Vec<Poll>)]) -> Result<Vec<String>, String> {
    for (_, polls) in sets {
        check_poll_text(polls)?;
    }
    validate_sets(sets)?;

    let Questions { questions, askings } = collect_questions(sets);

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

    let mut lines: Vec<String> = Vec::new();
    let mut answer_total = 0usize;
    let mut plurality_total = 0usize;

    for model in &models {
        lines.push(format!("model, {}", model.display()));
        for (question, question_askings) in questions.iter().zip(askings.iter()) {
            let mut question_lines: Vec<String> = Vec::new();
            for (set_index, poll) in question_askings {
                let juries = poll_juries(poll);
                let responses = responses_of(&juries, model);
                if responses.is_empty() {
                    continue;
                }
                let reading = jury_reading(poll, &responses);
                let whole: Vec<&Response> = poll.responses.iter().collect();
                let pooled = jury_reading(poll, &whole);
                let with_plurality = match &pooled.plurality {
                    Some(answer) => reading
                        .rows
                        .iter()
                        .find(|(held, _, _)| held == answer)
                        .map(|(_, count, _)| *count)
                        .unwrap_or(0),
                    None => 0,
                };
                question_lines.push(format!(
                    "asking, {}, {}, valid {}, with the plurality {}",
                    sets[*set_index].0.display(),
                    match &reading.plurality {
                        Some(answer) => answer.display(),
                        None => "none".to_string(),
                    },
                    reading.valid,
                    with_plurality
                ));
                answer_total += reading.valid;
                plurality_total += with_plurality;
            }
            if !question_lines.is_empty() {
                lines.push(format!("question, {}", question.display()));
                lines.extend(question_lines);
            }
        }
    }

    lines.push(format!(
        "benches, {} models, {} answers, {} with the plurality",
        models.len(),
        answer_total,
        plurality_total
    ));
    Ok(lines)
}
