//! The answers reading over the recorded poll sets.
//!
//! The reading reads the whole record answer by
//! answer. For each answer it states one part per
//! question at which the answer was offered, the
//! count of the parts at that question holding the
//! answer and of those the counts standing in accord
//! and with the pool, and it closes the section with
//! the standing line, counting the questions at
//! which the answer was offered, the parts holding
//! it summed over those questions, the questions at
//! which the accord of the question names it, and
//! the questions at which the pooled plurality of
//! the question names it. It carries no judgement
//! about any answer or any question, and answers
//! names only how each answer of the record stands
//! over the whole record. The obligations this
//! module carries are GJ-41 and GJ-42 in
//! CONTRACT.md, in the Round 20 section.

use crate::agreement::{add_reading, jury_reading, plurality_of, poll_juries};
use crate::polls::Poll;
use crate::pytext::PText;
use crate::record::{check_poll_text, collect_questions, validate_sets};

/// One question of the answers reading, the pooled
/// union of the question, the plurality of each part
/// of the question in set order, the accord of the
/// question, and the pooled plurality of the
/// question.
struct Question {
    union: Vec<PText>,
    pluralities: Vec<Option<PText>>,
    accord: Option<PText>,
    pooled: Option<PText>,
}

/// The pooled union and the plurality over a list of
/// askings of one question, the counts of every
/// asking summed as GJ-37 states the parts are
/// summed, the plurality read off the summed counts
/// over the union in asking order. Over the askings
/// of one set the pair is the part of that set at
/// the question, and over every asking of the
/// question it is the pooled plurality of GJ-28.
fn pooled_plurality(askings: &[&Poll]) -> (Vec<PText>, Option<PText>) {
    let mut union: Vec<PText> = Vec::new();
    for poll in askings {
        for answer in &poll.answers {
            if !union.contains(answer) {
                union.push(answer.clone());
            }
        }
    }
    let mut counts: Vec<usize> = vec![0; union.len()];
    let mut totals: (usize, usize, usize, usize) = (0, 0, 0, 0);
    for poll in askings {
        let juries = poll_juries(poll);
        let all: Vec<_> = juries
            .iter()
            .flat_map(|(_, rs)| rs.iter().copied())
            .collect();
        add_reading(&mut counts, &mut totals, &union, &jury_reading(poll, &all));
    }
    let plurality = plurality_of(&counts, totals.1, &union);
    (union, plurality)
}

/// Return the answers reading over a list of
/// recorded poll sets as lines, or the defect the
/// first failed rule names.
///
/// Each set pairs one set label with its polls. The
/// set rules of GJ-11 govern the sets unchanged, each
/// label checked before the questions of its set and
/// the first failure the refusal, and a refusal
/// returns no lines at all. One section stands per
/// answer in the order of its first offering. A
/// section opens with the answer line, then one part
/// per question at which the answer was offered, the
/// question line and the held line of the part, then
/// the standing line of the answer. The closing line
/// counts the answers, the questions and the parts
/// summed over the sections, and the questions in
/// accord and the questions with the pool summed
/// over the sections.
pub fn answers_lines(sets: &[(PText, Vec<Poll>)]) -> Result<Vec<String>, String> {
    for (_, polls) in sets {
        check_poll_text(polls)?;
    }
    validate_sets(sets)?;
    let gathered = collect_questions(sets);
    let mut questions: Vec<Question> = Vec::new();
    for askings in &gathered.askings {
        let mut set_order: Vec<usize> = Vec::new();
        for (set_index, _) in askings {
            if !set_order.contains(set_index) {
                set_order.push(*set_index);
            }
        }
        let mut pluralities: Vec<Option<PText>> = Vec::new();
        let mut named: Vec<PText> = Vec::new();
        for set_index in &set_order {
            let part_askings: Vec<&Poll> = askings
                .iter()
                .filter(|(index, _)| index == set_index)
                .map(|(_, poll)| *poll)
                .collect();
            let (_, plurality) = pooled_plurality(&part_askings);
            if let Some(answer) = &plurality {
                if !named.contains(answer) {
                    named.push(answer.clone());
                }
            }
            pluralities.push(plurality);
        }
        let accord = if named.len() == 1 {
            Some(named[0].clone())
        } else {
            None
        };
        let all_askings: Vec<&Poll> = askings.iter().map(|(_, poll)| *poll).collect();
        let (union, pooled) = pooled_plurality(&all_askings);
        questions.push(Question {
            union,
            pluralities,
            accord,
            pooled,
        });
    }
    let mut answers: Vec<PText> = Vec::new();
    for question in &questions {
        for answer in &question.union {
            if !answers.contains(answer) {
                answers.push(answer.clone());
            }
        }
    }
    let mut lines: Vec<String> = Vec::new();
    let mut total_questions = 0;
    let mut total_parts = 0;
    let mut total_accord = 0;
    let mut total_pool = 0;
    for answer in &answers {
        lines.push(format!("answer, {}", answer.display()));
        let mut offered = 0;
        let mut parts_held = 0;
        let mut in_accord = 0;
        let mut with_pool = 0;
        for (index, question) in questions.iter().enumerate() {
            if !question.union.contains(answer) {
                continue;
            }
            let held = question
                .pluralities
                .iter()
                .filter(|plurality| plurality.as_ref() == Some(answer))
                .count();
            let part_accord = if question.accord.as_ref() == Some(answer) {
                held
            } else {
                0
            };
            let part_pool = if question.pooled.as_ref() == Some(answer) {
                held
            } else {
                0
            };
            lines.push(format!("question, {}", gathered.questions[index].display()));
            lines.push(format!(
                "held, {}, in accord, {}, with the pool, {}",
                held, part_accord, part_pool
            ));
            offered += 1;
            parts_held += held;
            if question.accord.as_ref() == Some(answer) {
                in_accord += 1;
            }
            if question.pooled.as_ref() == Some(answer) {
                with_pool += 1;
            }
        }
        lines.push(format!(
            "standing, {} questions, {} parts held, {} in accord, {} with the pool",
            offered, parts_held, in_accord, with_pool
        ));
        total_questions += offered;
        total_parts += parts_held;
        total_accord += in_accord;
        total_pool += with_pool;
    }
    lines.push(format!(
        "answers, {} answers, {} questions, {} parts, {} in accord, {} with the pool",
        answers.len(),
        total_questions,
        total_parts,
        total_accord,
        total_pool
    ));
    Ok(lines)
}
