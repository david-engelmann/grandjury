//! The agreement reading over recorded polls.
//!
//! The reading is arithmetic over recorded responses and
//! nothing more. For each poll it states, per jury, the share
//! of the jury valid responses that each offered answer drew
//! as an exact fraction, the plurality answer of the jury
//! under a stated tie rule, and whether the pluralities of the
//! two juries agree. It carries no verdict about any jury. The
//! obligation this module carries is GJ-8 in CONTRACT.md, in
//! the Round 3 section.

use num_bigint::BigInt;
use num_rational::Ratio;

use crate::polls::{fraction_text, percent, Poll, Response};
use crate::pytext::PText;
use crate::record::check_poll_text;

/// The reading of one jury in a poll as data.
pub(crate) struct JuryReading {
    pub(crate) recorded: usize,
    pub(crate) valid: usize,
    pub(crate) abstained: usize,
    pub(crate) spoiled: usize,
    pub(crate) rows: Vec<(PText, usize, Ratio<BigInt>)>,
    pub(crate) plurality: Option<PText>,
}

/// The counts tail of the GJ-8 lines as text, the word
/// recorded and its count, then valid, abstained, and
/// spoiled in the order GJ-8 states. The agreement jury
/// line carries it, the wave parts of GJ-10 carry it, and
/// the asking parts and totals lines of the census of
/// GJ-11 carry it, and the form stands here in one place
/// for every reading.
pub(crate) fn counts_text(
    recorded: usize,
    valid: usize,
    abstained: usize,
    spoiled: usize,
) -> String {
    format!(
        "recorded {}, valid {}, abstained {}, spoiled {}",
        recorded, valid, abstained, spoiled
    )
}

impl JuryReading {
    /// The counts of the reading in the GJ-8 form, the tail
    /// the shared counts form states from the four counts
    /// of the reading.
    pub(crate) fn counts_text(&self) -> String {
        counts_text(self.recorded, self.valid, self.abstained, self.spoiled)
    }

    /// The per answer rows of the reading in the GJ-8 form,
    /// one line per offered answer, two spaces, the answer,
    /// its count, its share as the exact fraction in lowest
    /// terms, and its percentage. The agreement reading and
    /// the wave comparison reading both print these rows,
    /// and the form stands here in one place for both.
    pub(crate) fn row_lines(&self) -> Vec<String> {
        self.rows
            .iter()
            .map(|(answer, count, share)| {
                format!(
                    "  {}, {}, {}, {}%",
                    answer.display(),
                    count,
                    fraction_text(share),
                    percent(share)
                )
            })
            .collect()
    }

    /// The margin of the plurality of the reading, the
    /// count of the plurality answer minus the largest
    /// count among the other offered answers, or no margin
    /// where no plurality stands. The dissent reading of
    /// GJ-15 states this figure for one jury at one asking
    /// and the swing reading of GJ-18 states it for the
    /// whole poll at one asking, and the figure stands
    /// here in one place for both readings.
    pub(crate) fn margin(&self) -> Option<usize> {
        let plurality = self.plurality.as_ref()?;
        let standing = self
            .rows
            .iter()
            .find(|(answer, _, _)| answer == plurality)
            .map(|(_, count, _)| *count)
            .unwrap_or(0);
        let largest_other = self
            .rows
            .iter()
            .filter(|(answer, _, _)| answer != plurality)
            .map(|(_, count, _)| *count)
            .max()
            .unwrap_or(0);
        Some(standing - largest_other)
    }
}

/// Return the reading of one jury in a poll as data.
///
/// The jury line counts stand in the order recorded, valid,
/// abstained, and spoiled. Spoiled responses stand outside the
/// shares, as they stand outside the shares of the tally, and
/// abstained responses stand outside them as their own kind.
/// The plurality is the offered answer holding the largest
/// valid count, and it stands only when the jury holds at
/// least one valid response and one answer holds that count
/// alone, under FIX-GJ-8.
pub(crate) fn jury_reading(poll: &Poll, responses: &[&Response]) -> JuryReading {
    let mut counts: Vec<usize> = vec![0; poll.answers.len()];
    let mut abstained = 0usize;
    let mut spoiled = 0usize;
    for response in responses {
        if response.abstained {
            abstained += 1;
        } else if let Some(slot) = poll.answers.iter().position(|a| *a == response.answer) {
            counts[slot] += 1;
        } else {
            spoiled += 1;
        }
    }
    let valid = responses.len() - abstained - spoiled;
    let mut rows = Vec::new();
    for (index, answer) in poll.answers.iter().enumerate() {
        let share = if valid > 0 {
            Ratio::new(BigInt::from(counts[index]), BigInt::from(valid))
        } else {
            Ratio::from(BigInt::from(0))
        };
        rows.push((answer.clone(), counts[index], share));
    }
    let largest = counts.iter().copied().max().unwrap_or(0);
    let holders: Vec<&PText> = poll
        .answers
        .iter()
        .enumerate()
        .filter(|(index, _)| counts[*index] == largest)
        .map(|(_, answer)| answer)
        .collect();
    let plurality = if valid > 0 && holders.len() == 1 {
        Some(holders[0].clone())
    } else {
        None
    };
    JuryReading {
        recorded: responses.len(),
        valid,
        abstained,
        spoiled,
        rows,
        plurality,
    }
}

/// Sum the counts of one reading into a pool over the
/// union answers of a question, the four counts of the
/// reading added to the pool counts and the count of
/// every answer of the reading added to the slot of
/// that answer in the union, count by count.
pub(crate) fn add_reading(
    pool: &mut [usize],
    counts: &mut (usize, usize, usize, usize),
    union: &[PText],
    reading: &JuryReading,
) {
    counts.0 += reading.recorded;
    counts.1 += reading.valid;
    counts.2 += reading.abstained;
    counts.3 += reading.spoiled;
    for (answer, count, _) in &reading.rows {
        let slot = union
            .iter()
            .position(|held| held == answer)
            .expect("an offered answer of the union");
        pool[slot] += count;
    }
}

/// The plurality of a pool of summed counts, the
/// offered answer holding the largest count among the
/// valid responses the counts sum, standing only when
/// the pool holds at least one valid response and one
/// answer holds that count alone. The rule serves the
/// pool of a whole question under the application
/// GJ-27 states and the pool of one jury over one
/// question under the application GJ-29 states, and
/// the aggregate and the canvass and the roll all read
/// it, and the rule stands here in one place for every
/// reading.
pub(crate) fn plurality_of(counts: &[usize], valid: usize, union: &[PText]) -> Option<PText> {
    let largest = counts.iter().copied().max().unwrap_or(0);
    let holders = counts.iter().filter(|count| **count == largest).count();
    if valid > 0 && holders == 1 {
        let slot = counts
            .iter()
            .position(|count| *count == largest)
            .expect("a largest count");
        Some(union[slot].clone())
    } else {
        None
    }
}

/// The responses of one jury of a poll, from the grouped
/// juries of the poll, or no responses where the jury does
/// not stand in the poll.
pub(crate) fn responses_of<'a>(
    juries: &'a [(PText, Vec<&'a Response>)],
    model: &PText,
) -> Vec<&'a Response> {
    juries
        .iter()
        .find(|(held, _)| held == model)
        .map(|(_, responses)| responses.clone())
        .unwrap_or_default()
}

/// Return the juries of a poll as model and responses pairs.
///
/// A jury is one model string standing in the responses, and
/// the juries stand in the order their model strings first
/// appear in the responses.
pub(crate) fn poll_juries(poll: &Poll) -> Vec<(PText, Vec<&Response>)> {
    let mut order: Vec<PText> = Vec::new();
    let mut grouped: Vec<Vec<&Response>> = Vec::new();
    for response in &poll.responses {
        if let Some(slot) = order.iter().position(|m| *m == response.model) {
            grouped[slot].push(response);
        } else {
            order.push(response.model.clone());
            grouped.push(vec![response]);
        }
    }
    order.into_iter().zip(grouped).collect()
}

/// Return the agreement reading of a list of polls as lines,
/// or the refusal the poll text guard of the record module
/// names for the first handed field holding a line break,
/// as FIX-GJ-14 states.
///
/// The reading holds one section per poll in the order the
/// polls stand and a closing line. A section names the
/// question, one block per jury, and the agreement line. The
/// agreement line reads agree or differ only when the poll
/// holds exactly two juries and both hold a plurality, and it
/// reads not counted in every other case. The closing line
/// counts the agree sections over the sections that read agree
/// or differ.
pub fn agreement_lines(polls: &[Poll]) -> Result<Vec<String>, String> {
    check_poll_text(polls)?;
    let mut lines: Vec<String> = Vec::new();
    let mut agreed = 0usize;
    let mut counted = 0usize;
    for poll in polls {
        lines.push(format!("question, {}", poll.question.display()));
        let mut pluralities: Vec<Option<PText>> = Vec::new();
        let juries = poll_juries(poll);
        for (model, responses) in &juries {
            let reading = jury_reading(poll, responses);
            lines.push(format!(
                "jury, {}, {}",
                model.display(),
                reading.counts_text()
            ));
            lines.extend(reading.row_lines());
            let plurality = reading.plurality.clone();
            lines.push(format!(
                "plurality, {}, {}",
                model.display(),
                match &plurality {
                    Some(answer) => answer.display(),
                    None => "none".to_string(),
                }
            ));
            pluralities.push(plurality);
        }
        if juries.len() == 2 && pluralities[0].is_some() && pluralities[1].is_some() {
            counted += 1;
            if pluralities[0] == pluralities[1] {
                agreed += 1;
                lines.push("pluralities, agree".to_string());
            } else {
                lines.push("pluralities, differ".to_string());
            }
        } else {
            lines.push("pluralities, not counted".to_string());
        }
    }
    lines.push(format!("agreements, {} of {}", agreed, counted));
    Ok(lines)
}
