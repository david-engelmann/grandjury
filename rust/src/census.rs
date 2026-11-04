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

use crate::agreement::{counts_text, jury_reading, poll_juries, responses_of, JuryReading};
use crate::polls::Poll;
use crate::pytext::PText;

/// The counts of one jury over some polls, in the order the
/// jury lines of GJ-8 state them.
#[derive(Default)]
struct Counts {
    recorded: usize,
    valid: usize,
    abstained: usize,
    spoiled: usize,
}

impl Counts {
    fn of(reading: &JuryReading) -> Counts {
        Counts {
            recorded: reading.recorded,
            valid: reading.valid,
            abstained: reading.abstained,
            spoiled: reading.spoiled,
        }
    }

    fn add(&mut self, reading: &JuryReading) {
        self.recorded += reading.recorded;
        self.valid += reading.valid;
        self.abstained += reading.abstained;
        self.spoiled += reading.spoiled;
    }

    fn text(&self) -> String {
        counts_text(self.recorded, self.valid, self.abstained, self.spoiled)
    }
}

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
    for (index, (label, polls)) in sets.iter().enumerate() {
        if label.is_empty() {
            return Err("a set label is empty".to_string());
        }
        if label
            .units()
            .iter()
            .any(|unit| *unit == 0x0A || *unit == 0x0D)
        {
            return Err("a set label holds a newline".to_string());
        }
        if sets[..index].iter().any(|(earlier, _)| earlier == label) {
            return Err(format!("the set label {} stands twice", label.display()));
        }
        for (position, poll) in polls.iter().enumerate() {
            if polls[..position]
                .iter()
                .any(|earlier| earlier.question == poll.question)
            {
                return Err(format!(
                    "the set {} holds the question {} twice",
                    label.display(),
                    poll.question.display()
                ));
            }
        }
    }

    // The questions of the reading, in the order they first
    // appear across the sets, each with its askings as the
    // index of the set and the poll of that set.
    let mut questions: Vec<PText> = Vec::new();
    let mut askings: Vec<Vec<(usize, &Poll)>> = Vec::new();
    for (set_index, (_, polls)) in sets.iter().enumerate() {
        for poll in polls {
            match questions.iter().position(|held| *held == poll.question) {
                Some(slot) => askings[slot].push((set_index, poll)),
                None => {
                    questions.push(poll.question.clone());
                    askings.push(vec![(set_index, poll)]);
                }
            }
        }
    }

    let mut lines: Vec<String> = Vec::new();
    let mut held_count = 0usize;
    let mut moved_count = 0usize;
    let mut not_counted = 0usize;
    let mut record = Counts::default();
    let mut jury_order: Vec<PText> = Vec::new();
    let mut jury_totals: Vec<Counts> = Vec::new();

    // The totals of the whole record, summed through the
    // standing jury reading over every poll of every set.
    for (_, polls) in sets {
        for poll in polls {
            for (model, responses) in &poll_juries(poll) {
                let reading = jury_reading(poll, responses);
                record.add(&reading);
                match jury_order.iter().position(|held| held == model) {
                    Some(slot) => jury_totals[slot].add(&reading),
                    None => {
                        jury_order.push(model.clone());
                        let mut counts = Counts::default();
                        counts.add(&reading);
                        jury_totals.push(counts);
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
            let movement =
                if standing.len() >= 2 && standing.iter().all(|answer| answer == &standing[0]) {
                    held_count += 1;
                    "held"
                } else if standing.len() >= 2 {
                    moved_count += 1;
                    "moved"
                } else {
                    not_counted += 1;
                    "not counted"
                };
            lines.push(format!("movement, {}, {}", model.display(), movement));
        }
    }

    lines.push(format!(
        "record, polls {}, {}",
        sets.iter().map(|(_, polls)| polls.len()).sum::<usize>(),
        record.text()
    ));
    for (model, counts) in jury_order.iter().zip(jury_totals.iter()) {
        lines.push(format!("jury, {}, {}", model.display(), counts.text()));
    }
    lines.push(format!(
        "movements, {} held, {} moved, {} not counted",
        held_count, moved_count, not_counted
    ));
    Ok(lines)
}
