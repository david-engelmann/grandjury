//! The forms the readings across the recorded poll sets
//! share.
//!
//! The census reading of GJ-11 and the juror ledger
//! reading of GJ-13 stand on the same set rules, gather
//! their questions the same way, count through the same
//! counts, and close with the same record line and the
//! same movement counts line, and the movement selection
//! of those readings also stands in the wave comparison
//! reading of GJ-10. The dissent reading of GJ-15 stands
//! on the same set rules, and the single response
//! classification and the respondent collection the
//! ledger and the dissent share stand here as well.
//! Those forms stand here once, in one home, so the
//! readings that share them cannot drift apart.

use crate::agreement::{counts_text, jury_reading, JuryReading};
use crate::polls::{Poll, Response};
use crate::pytext::PText;

/// Check the sets of a reading across the recorded poll
/// sets under the set rules of GJ-11, each label checked
/// before the questions of its set, and return the defect
/// the first failed rule names.
pub(crate) fn validate_sets(sets: &[(PText, Vec<Poll>)]) -> Result<(), String> {
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
    Ok(())
}

/// The questions of a reading across the recorded poll
/// sets, in the order they first appear across the sets,
/// each with its askings as the index of the set and the
/// poll of that set, as GJ-11 states them.
pub(crate) struct Questions<'a> {
    pub(crate) questions: Vec<PText>,
    pub(crate) askings: Vec<Vec<(usize, &'a Poll)>>,
}

/// Gather the questions of a reading across the recorded
/// poll sets in the order and with the askings the shared
/// collection states.
pub(crate) fn collect_questions(sets: &[(PText, Vec<Poll>)]) -> Questions<'_> {
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
    Questions { questions, askings }
}

/// Gather the respondents of a reading across the
/// recorded poll sets, each juror string in the order it
/// first appears across the record, the order the blocks
/// of the ledger stand in and the dissenter lines of the
/// dissent stand in.
pub(crate) fn collect_respondents(sets: &[(PText, Vec<Poll>)]) -> Vec<PText> {
    let mut respondents: Vec<PText> = Vec::new();
    for (_, polls) in sets {
        for poll in polls {
            for response in &poll.responses {
                if !respondents.contains(&response.juror) {
                    respondents.push(response.juror.clone());
                }
            }
        }
    }
    respondents
}

/// The counts of one jury or one respondent over some
/// polls, in the order the jury lines of GJ-8 state them.
#[derive(Default)]
pub(crate) struct Counts {
    recorded: usize,
    valid: usize,
    abstained: usize,
    spoiled: usize,
}

impl Counts {
    pub(crate) fn of(reading: &JuryReading) -> Counts {
        Counts {
            recorded: reading.recorded,
            valid: reading.valid,
            abstained: reading.abstained,
            spoiled: reading.spoiled,
        }
    }

    pub(crate) fn add_reading(&mut self, reading: &JuryReading) {
        self.recorded += reading.recorded;
        self.valid += reading.valid;
        self.abstained += reading.abstained;
        self.spoiled += reading.spoiled;
    }

    pub(crate) fn add_counts(&mut self, other: &Counts) {
        self.recorded += other.recorded;
        self.valid += other.valid;
        self.abstained += other.abstained;
        self.spoiled += other.spoiled;
    }

    pub(crate) fn text(&self) -> String {
        counts_text(self.recorded, self.valid, self.abstained, self.spoiled)
    }
}

/// The movement of a sequence of standing answers, held
/// when two or more stand and every one is the same
/// answer, moved when two or more stand and they differ,
/// and not counted in every other case.
#[derive(Clone, Copy)]
pub(crate) enum Movement {
    Held,
    Moved,
    NotCounted,
}

impl Movement {
    pub(crate) fn word(&self) -> &'static str {
        match self {
            Movement::Held => "held",
            Movement::Moved => "moved",
            Movement::NotCounted => "not counted",
        }
    }
}

/// The movement of a sequence of standing answers under
/// the rule the readings share.
pub(crate) fn movement_of(answers: &[PText]) -> Movement {
    if answers.len() >= 2 && answers.iter().all(|answer| answer == &answers[0]) {
        Movement::Held
    } else if answers.len() >= 2 {
        Movement::Moved
    } else {
        Movement::NotCounted
    }
}

/// The counts of the movement lines of a reading, with
/// the closing line those counts stand in.
#[derive(Default)]
pub(crate) struct MovementCounts {
    held: usize,
    moved: usize,
    not_counted: usize,
}

impl MovementCounts {
    pub(crate) fn add(&mut self, movement: Movement) {
        match movement {
            Movement::Held => self.held += 1,
            Movement::Moved => self.moved += 1,
            Movement::NotCounted => self.not_counted += 1,
        }
    }

    pub(crate) fn line(&self) -> String {
        format!(
            "movements, {} held, {} moved, {} not counted",
            self.held, self.moved, self.not_counted
        )
    }
}

/// The record line of a reading, the count of its polls
/// and the counts of the whole record.
pub(crate) fn record_line(polls: usize, counts: &Counts) -> String {
    format!("record, polls {}, {}", polls, counts.text())
}

/// The refusal a reading over handed polls draws when a
/// field it prints holds a line break, or nothing when no
/// field it reads holds one, as FIX-GJ-14 states.
///
/// The guard reads the polls in the order they stand, the
/// question of a poll first and then the juror of each
/// response in response order, and the first field holding
/// a break is the refusal it returns, named in the wording
/// the loader names for that field, field question holds a
/// newline for the question under FIX-GJ-6 and response
/// and its index and field juror holds a newline for a
/// juror under GJ-1. The readings across the recorded poll
/// sets and the agreement reading consume the guard before
/// they render, so a handed poll stands at the library
/// surface under the same one line rule the loader states,
/// and no defect of the set rules can name a value this
/// guard would have refused. The guard reads no other
/// field and holds no second copy of the break predicate,
/// reading every field through holds_line_break.
pub(crate) fn check_poll_text(polls: &[Poll]) -> Result<(), String> {
    for poll in polls {
        if poll.question.holds_line_break() {
            return Err("field question holds a newline".to_string());
        }
        for (index, response) in poll.responses.iter().enumerate() {
            if response.juror.holds_line_break() {
                return Err(format!("response {} field juror holds a newline", index));
            }
        }
    }
    Ok(())
}

/// Whether a response is a valid answer of its poll,
/// neither abstained nor spoiled, read through the jury
/// reading of GJ-8 over the single response, the one home
/// of the classification the readings across the sets
/// share.
pub(crate) fn is_valid(poll: &Poll, response: &Response) -> bool {
    jury_reading(poll, &[response]).valid == 1
}

/// The dissenters of one asking or one jury, the jurors
/// whose valid answer was not the plurality answer, in
/// the order the respondents first appear across the
/// record.
///
/// The responses are the responses of the asking or of
/// the jury, and the plurality is the standing plurality
/// answer those responses are read against, the
/// plurality of the jury under GJ-8 in the dissent
/// reading and the pooled plurality of GJ-17 in the
/// minority reading, and the helper is called only where
/// a plurality stands, so where no plurality stands no
/// dissenter stands. The selection the dissent reading
/// of GJ-15 and the minority reading of GJ-19 share
/// stands here once, so the two readings cannot drift
/// apart in who a dissenter is or in the order the
/// dissenter lines stand in.
pub(crate) fn dissenters_of<'a>(
    poll: &Poll,
    responses: &[&Response],
    plurality: &PText,
    respondents: &'a [PText],
) -> Vec<&'a PText> {
    let dissenting: Vec<&PText> = responses
        .iter()
        .filter(|response| is_valid(poll, response) && response.answer != *plurality)
        .map(|response| &response.juror)
        .collect();
    respondents
        .iter()
        .filter(|respondent| dissenting.contains(respondent))
        .collect()
}
