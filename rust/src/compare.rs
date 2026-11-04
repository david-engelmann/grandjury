//! The wave comparison reading over two recorded waves.
//!
//! The reading pairs two lists of recorded polls in order
//! and states, for each pair and for each jury of the pair,
//! the reading of the jury in each wave in the form of GJ-8
//! and whether the plurality answer of the jury stands or has
//! changed between the two waves. It carries no judgement
//! about any jury, and its movement words name only that a
//! plurality answer stands or has changed. The obligation
//! this module carries is GJ-10 in CONTRACT.md, in the Round
//! 4 section.

use crate::agreement::{jury_reading, poll_juries, JuryReading};
use crate::polls::{Poll, Response};
use crate::pytext::PText;

/// The responses of one jury of a poll, from the grouped
/// juries of the poll, or no responses where the jury does
/// not stand in the poll.
fn responses_of<'a>(juries: &'a [(PText, Vec<&'a Response>)], model: &PText) -> Vec<&'a Response> {
    juries
        .iter()
        .find(|(held, _)| held == model)
        .map(|(_, responses)| responses.clone())
        .unwrap_or_default()
}

/// Push the part of one jury block that one wave carries,
/// the counts line in the order GJ-8 states, one line per
/// offered answer of the poll in offered order, and the
/// plurality line of the wave.
fn push_wave_part(lines: &mut Vec<String>, wave: &str, reading: &JuryReading) {
    lines.push(format!(
        "{}, recorded {}, valid {}, abstained {}, spoiled {}",
        wave, reading.recorded, reading.valid, reading.abstained, reading.spoiled
    ));
    lines.extend(reading.row_lines());
    lines.push(format!(
        "plurality, {}, {}",
        wave,
        match &reading.plurality {
            Some(answer) => answer.display(),
            None => "none".to_string(),
        }
    ));
}

/// Return the wave comparison reading over two lists of
/// polls as lines, or the defect the pairing refusal names.
///
/// The two lists pair in order, and pair numbers count from
/// one in step with the poll file numbers. The pairing fails
/// when the two sets hold different counts, and it fails when
/// the two polls of a pair hold different question strings.
/// The juries of a pair are the model strings standing in
/// either poll of the pair, in the order they first appear in
/// the first poll and then those standing only in the second
/// poll in the order they first appear there. A jury absent
/// from one wave reads with every count zero and no plurality
/// in that wave. The movement of a jury reads held when both
/// waves hold the same plurality answer for the jury, moved
/// when both hold one and the answers differ, and not counted
/// in every other case. The closing line counts the movement
/// lines over every question by jury pair of the reading.
pub fn compare_lines(first: &[Poll], second: &[Poll]) -> Result<Vec<String>, String> {
    if first.len() != second.len() {
        return Err("the two poll sets hold different counts".to_string());
    }
    for (index, (first_poll, second_poll)) in first.iter().zip(second.iter()).enumerate() {
        if first_poll.question != second_poll.question {
            return Err(format!("pair {} holds two different questions", index + 1));
        }
    }
    let mut lines: Vec<String> = Vec::new();
    let mut held = 0usize;
    let mut moved = 0usize;
    let mut not_counted = 0usize;
    for (first_poll, second_poll) in first.iter().zip(second.iter()) {
        lines.push(format!("question, {}", first_poll.question.display()));
        let first_juries = poll_juries(first_poll);
        let second_juries = poll_juries(second_poll);
        let mut models: Vec<PText> = first_juries
            .iter()
            .map(|(model, _)| model.clone())
            .collect();
        for (model, _) in &second_juries {
            if !models.contains(model) {
                models.push(model.clone());
            }
        }
        for model in &models {
            lines.push(format!("jury, {}", model.display()));
            let first_reading = jury_reading(first_poll, &responses_of(&first_juries, model));
            let second_reading = jury_reading(second_poll, &responses_of(&second_juries, model));
            push_wave_part(&mut lines, "first wave", &first_reading);
            push_wave_part(&mut lines, "second wave", &second_reading);
            let movement = match (&first_reading.plurality, &second_reading.plurality) {
                (Some(first_answer), Some(second_answer)) if first_answer == second_answer => {
                    held += 1;
                    "held"
                }
                (Some(_), Some(_)) => {
                    moved += 1;
                    "moved"
                }
                _ => {
                    not_counted += 1;
                    "not counted"
                }
            };
            lines.push(format!("movement, {}, {}", model.display(), movement));
        }
    }
    lines.push(format!(
        "movements, {} held, {} moved, {} not counted",
        held, moved, not_counted
    ));
    Ok(lines)
}
