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

/// The reading of one jury in a poll as data.
struct JuryReading {
    recorded: usize,
    valid: usize,
    abstained: usize,
    spoiled: usize,
    rows: Vec<(PText, usize, Ratio<BigInt>)>,
    plurality: Option<PText>,
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
fn jury_reading(poll: &Poll, responses: &[&Response]) -> JuryReading {
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

/// Return the juries of a poll as model and responses pairs.
///
/// A jury is one model string standing in the responses, and
/// the juries stand in the order their model strings first
/// appear in the responses.
fn poll_juries(poll: &Poll) -> Vec<(PText, Vec<&Response>)> {
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

/// Return the agreement reading of a list of polls as lines.
///
/// The reading holds one section per poll in the order the
/// polls stand and a closing line. A section names the
/// question, one block per jury, and the agreement line. The
/// agreement line reads agree or differ only when the poll
/// holds exactly two juries and both hold a plurality, and it
/// reads not counted in every other case. The closing line
/// counts the agree sections over the sections that read agree
/// or differ.
pub fn agreement_lines(polls: &[Poll]) -> Vec<String> {
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
                "jury, {}, recorded {}, valid {}, abstained {}, spoiled {}",
                model.display(),
                reading.recorded,
                reading.valid,
                reading.abstained,
                reading.spoiled
            ));
            for (answer, count, share) in &reading.rows {
                lines.push(format!(
                    "  {}, {}, {}, {}%",
                    answer.display(),
                    count,
                    fraction_text(share),
                    percent(share)
                ));
            }
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
    lines
}
