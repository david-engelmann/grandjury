//! The census reading over the recorded poll sets.
//!
//! The reading reads across the recorded sets. For each
//! question text standing in any set it states, per jury,
//! the reading of the jury at each asking of the question in
//! the form of GJ-8 and whether the plurality of the jury
//! ever moved across its askings, and it closes with the
//! totals of the whole record and the movement counts. It
//! carries no judgement about any jury, and its movement
//! words name only that a plurality answer stands or has
//! changed across askings. The obligation this module
//! carries is GJ-11 in CONTRACT.md, in the Round 5 section.

use crate::agreement::{jury_reading, poll_juries, responses_of, JuryReading};
use crate::polls::Poll;
use crate::pytext::PText;
use crate::record::{
    collect_questions, movement_of, record_line, validate_sets, Counts, MovementCounts, Questions,
};

/// Push the part of one jury block that one asking carries,
/// the counts line in the order GJ-8 states opening with
/// the word set and the set label standing behind it as
/// FIX-GJ-12 states, one line per offered answer of the
/// asking poll in offered order, and the plurality line of
/// the asking.
fn push_asking_part(lines: &mut Vec<String>, label: &PText, reading: &JuryReading) {
    lines.push(format!(
        "set, {}, {}",
        label.display(),
        Counts::of(reading).text()
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

/// Return the census reading over a list of recorded poll
/// sets as lines, or the defect the first failed rule names.
///
/// Each set pairs one set label with its polls. A set label
/// is a non empty text holding no newline and no carriage
/// return character, and it stands once in one census, and
/// one set holds one question string at most once. The
/// reading checks the sets in the order they stand, and for
/// each set its label before its questions, and the first
/// failure it reaches is the refusal it returns, and a
/// refusal returns no lines at all. The questions of the
/// reading stand in the order they first appear across the
/// sets, and the askings of a question are the sets that
/// asked it, in set order. The juries of a question stand in
/// the order their model strings first appear across the
/// askings of the question. The movement of a jury over a
/// question reads moved when two or more of the askings hold
/// a plurality for the jury and the plurality answers are
/// not all the same answer, held when two or more hold one
/// and every plurality answer is the same answer, and not
/// counted in every other case. After the sections the
/// record line and one totals line per jury of the whole
/// record stand, and the closing line counts the movement
/// lines of the reading.
pub fn census_lines(sets: &[(PText, Vec<Poll>)]) -> Result<Vec<String>, String> {
    validate_sets(sets)?;

    // The questions of the reading and their askings, as
    // the shared collection of the record module gathers
    // them.
    let Questions { questions, askings } = collect_questions(sets);

    let mut lines: Vec<String> = Vec::new();
    let mut movement_counts = MovementCounts::default();
    let mut record = Counts::default();
    let mut jury_order: Vec<PText> = Vec::new();
    let mut jury_totals: Vec<Counts> = Vec::new();

    // The totals of the whole record, summed through the
    // standing jury reading over every poll of every set.
    for (_, polls) in sets {
        for poll in polls {
            for (model, responses) in &poll_juries(poll) {
                let reading = jury_reading(poll, responses);
                record.add_reading(&reading);
                match jury_order.iter().position(|held| held == model) {
                    Some(slot) => jury_totals[slot].add_reading(&reading),
                    None => {
                        jury_order.push(model.clone());
                        jury_totals.push(Counts::of(&reading));
                    }
                }
            }
        }
    }

    for (question, question_askings) in questions.iter().zip(askings.iter()) {
        lines.push(format!("question, {}", question.display()));
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
        for model in &models {
            lines.push(format!("jury, {}", model.display()));
            let mut standing: Vec<PText> = Vec::new();
            for (set_index, poll) in question_askings {
                let juries = poll_juries(poll);
                let reading = jury_reading(poll, &responses_of(&juries, model));
                push_asking_part(&mut lines, &sets[*set_index].0, &reading);
                if let Some(answer) = &reading.plurality {
                    standing.push(answer.clone());
                }
            }
            let movement = movement_of(&standing);
            movement_counts.add(movement);
            lines.push(format!(
                "movement, {}, {}",
                model.display(),
                movement.word()
            ));
        }
    }

    lines.push(record_line(
        sets.iter().map(|(_, polls)| polls.len()).sum(),
        &record,
    ));
    for (model, counts) in jury_order.iter().zip(jury_totals.iter()) {
        lines.push(format!("jury, {}, {}", model.display(), counts.text()));
    }
    lines.push(movement_counts.line());
    Ok(lines)
}
