//! The answers reading, GJ-41 and GJ-42, twenty pins
//! restated from first principles.
//!
//! The reading is recomputed from the shipped sets
//! and from the generated family by an independent
//! walk in this file, the direct computation below,
//! which counts the responses itself and gathers
//! the record by answer without the library's
//! reading machinery, and the library walk and the
//! printed surface are compared against it line for
//! line, so a defect in the library walk fails here
//! even where a figure the closing relations reach
//! would let it pass. The closing relations stand
//! as GJ-42 states them, each one failable on its
//! own ground. The shapes stand in the forms GJ-42
//! states them, S1 to S7, each on a fixture small
//! enough to read whole. Against the Round 19
//! closure tree this target does not compile, the
//! answers module standing absent there, and that
//! failure is the base run the round records.
//!
//! The discrimination of the pins is stated against
//! the mutation set of the Round 20 design, OA to
//! OT, its postures computed by the validated
//! transcription of the design or imported from the
//! Round 19 executions, and no posture is stated
//! the computations do not carry. OA, an answer's
//! held figure summed over every question in place
//! of its own, fails pins 4, 12, 18, 19, and 20,
//! pin 12 by construction, and moves no shipped
//! line. OB, the in accord and with the pool
//! standings swapped, fails pins 4, 10, 18, 19,
//! and 20, pin 10 by construction. OC, a tied
//! part's answers at the top count counted as
//! held, fails pins 4, 7, 18, 19, and 20, pin 7 by
//! construction. OD, an answer offered only at
//! questions whose parts all stand empty omitted
//! from the reading, fails pins 1, 2, and 8, pin 8
//! by construction, and pins 3, 4, and 5 stand. OE,
//! the held figure read off the first asking set's
//! part alone, fails pins 1, 2, 4, 6, 18, 19, and
//! 20, pin 6 by construction. OF, the accord read
//! as the first named plurality in set order, fails
//! pins 4, 9, 18, 19, and 20, pin 9 by
//! construction. OG, the offerings gathered from
//! the first asking set's poll alone, fails pin 11
//! alone and moves no line of either record. OH,
//! the head order of answers_lines swapped, fails
//! no pin, and pins 13 and 14 stand. OI, the guard
//! read over the first set's polls alone, fails pin
//! 13 and pin 14 stands. OJ, the duplicate label
//! block of validate_sets deleted, fails pin 15 on
//! the duplicate fixture. OK, the closing word
//! restated as a refused word, fails pin 16 with a
//! wide radius over the line comparing pins, pins
//! 1, 2, 4, 18, 19, and 20 among them, and the
//! status pins stand. OL, the help entry deleted
//! with the dispatch wired, fails pin 17 alone. OM,
//! the dispatch restated to run the tally handler
//! on its first value, fails pin 23 in
//! rust/tests/test_cli.rs alone, and pins 21 and 22
//! stand. ON, the generated family of this file
//! restated with the Round 13 fault, fails pin 18,
//! and its radius over pins 4, 19, and 20 stands to
//! be read from the executed set. OO, the closing
//! computed from the shipped record's figures,
//! fails pins 4 and 20 over the generated record,
//! and pin 5 stands, its generated leg reading the
//! reconciliation off the section sums and not the
//! closing line. Pin 18 asserts the generated
//! closing as GJ-42 states it, so pin 18 fails
//! under OO with pins 4 and 20, where the design's
//! OO entry states it stands on the sections alone,
//! and the difference stands flagged here for the
//! reviews. OP, the README Round 20 status line
//! restated, fails pin 24 in
//! rust/tests/test_setup.rs alone, and pin 25
//! stands. OQ, a section's standing line computed
//! from the first offered question alone, fails
//! pins 4, 12, 18, 19, and 20, pin 12 by
//! construction. OR, a section's standing line
//! stating every question of the record, fails pins
//! 1, 2, 4, 11, 18, and 19, and pin 20 stands. OS,
//! the closing's in accord figure counting the
//! parts standing in accord, fails pins 3, 4, 5,
//! 18, 19, and 20. OT, the per set question loop of
//! validate_sets moved ahead of the label checks,
//! fails pin 15 on the mixed fixture.

mod shared;

use shared::*;

use grandjury::accord::accord_lines;
use grandjury::aggregate::aggregate_lines;
use grandjury::answers::answers_lines;
use grandjury::census::census_lines;
use grandjury::polls::{load_poll_file, Poll, Response};
use grandjury::pytext::PText;
use grandjury::standing::standing_lines;

fn response(juror: &str, model: &str, answer: Option<&str>, reason: &str) -> Response {
    match answer {
        Some(answer) => Response {
            juror: pt(juror),
            model: pt(model),
            answer: pt(answer),
            reason: pt(reason),
            abstained: false,
        },
        None => Response {
            juror: pt(juror),
            model: pt(model),
            answer: pt(""),
            reason: pt("no answer recorded"),
            abstained: true,
        },
    }
}

fn poll(question: &str, answers: &[&str], responses: Vec<Response>) -> Poll {
    Poll {
        question: pt(question),
        answers: answers.iter().map(|answer| pt(answer)).collect(),
        responses,
    }
}

fn votes_of(jury: &str, answer: &str, count: usize) -> Vec<Response> {
    (1..=count)
        .map(|seat| {
            response(
                &format!("{} hand {} {}", jury, answer, seat),
                "model a",
                Some(answer),
                "the record states it",
            )
        })
        .collect()
}

fn load_poll(path: &std::path::Path) -> Poll {
    let (poll, defects) = load_poll_file(path);
    assert!(defects.is_empty(), "defects in {:?}: {:?}", path, defects);
    poll.unwrap()
}

fn load_set(dir: &std::path::Path, count: usize) -> Vec<Poll> {
    (1..=count)
        .map(|index| load_poll(&dir.join(format!("poll-{:02}.json", index))))
        .collect()
}

fn recorded_sets() -> Vec<(PText, Vec<Poll>)> {
    let root = examples();
    vec![
        (pt("pilot"), load_set(&root.join("pilot"), 12)),
        (pt("full"), load_set(&root.join("full"), 40)),
        (pt("wave"), load_set(&root.join("wave"), 12)),
    ]
}

fn figure_before(line: &str, word: &str) -> i64 {
    line.split(", ")
        .find_map(|part| {
            let (figure, rest) = part.split_once(' ')?;
            if rest == word {
                figure.parse::<i64>().ok()
            } else {
                None
            }
        })
        .unwrap_or_else(|| panic!("no figure before {} in {}", word, line))
}

fn questions_of(sets: &[(PText, Vec<Poll>)]) -> Vec<PText> {
    let mut questions: Vec<PText> = Vec::new();
    for (_, polls) in sets {
        for poll in polls {
            if !questions.contains(&poll.question) {
                questions.push(poll.question.clone());
            }
        }
    }
    questions
}

fn askings_of<'a>(sets: &'a [(PText, Vec<Poll>)], question: &PText) -> Vec<(usize, &'a Poll)> {
    let mut askings: Vec<(usize, &Poll)> = Vec::new();
    for (set_index, (_, polls)) in sets.iter().enumerate() {
        for poll in polls {
            if &poll.question == question {
                askings.push((set_index, poll));
            }
        }
    }
    askings
}

fn direct_plurality(counts: &[i64], valid: i64, union: &[PText]) -> Option<PText> {
    if valid <= 0 {
        return None;
    }
    let largest = counts.iter().copied().max().unwrap_or(0);
    if counts.iter().filter(|count| **count == largest).count() != 1 {
        return None;
    }
    let index = counts.iter().position(|count| *count == largest).unwrap();
    Some(union[index].clone())
}

/// The pooled union of a list of askings of one
/// question and the plurality over it, counted from
/// the responses themselves, the direct form of the
/// part of GJ-37 and of the pooled plurality of
/// GJ-28.
fn direct_part(polls: &[&Poll]) -> (Vec<PText>, Option<PText>) {
    let mut union: Vec<PText> = Vec::new();
    for poll in polls {
        for answer in &poll.answers {
            if !union.contains(answer) {
                union.push(answer.clone());
            }
        }
    }
    let mut counts = vec![0i64; union.len()];
    let mut valid = 0i64;
    for poll in polls {
        for response in &poll.responses {
            if response.abstained {
                continue;
            }
            if let Some(slot) = poll
                .answers
                .iter()
                .position(|answer| answer == &response.answer)
            {
                let union_slot = union
                    .iter()
                    .position(|answer| answer == &poll.answers[slot])
                    .unwrap();
                counts[union_slot] += 1;
                valid += 1;
            }
        }
    }
    let plurality = direct_plurality(&counts, valid, &union);
    (union, plurality)
}

/// The answers reading recomputed from first
/// principles, the transcription the library walk
/// and the printed surface are compared against.
fn direct_answers_lines(sets: &[(PText, Vec<Poll>)]) -> Vec<String> {
    let questions = questions_of(sets);
    let mut unions: Vec<Vec<PText>> = Vec::new();
    let mut pluralities: Vec<Vec<Option<PText>>> = Vec::new();
    let mut accords: Vec<Option<PText>> = Vec::new();
    let mut pooleds: Vec<Option<PText>> = Vec::new();
    for question in &questions {
        let askings = askings_of(sets, question);
        let mut set_order: Vec<usize> = Vec::new();
        for (set_index, _) in &askings {
            if !set_order.contains(set_index) {
                set_order.push(*set_index);
            }
        }
        let mut parts: Vec<Option<PText>> = Vec::new();
        let mut named: Vec<PText> = Vec::new();
        for set_index in &set_order {
            let part_polls: Vec<&Poll> = askings
                .iter()
                .filter(|(index, _)| index == set_index)
                .map(|(_, poll)| *poll)
                .collect();
            let (_, plurality) = direct_part(&part_polls);
            if let Some(answer) = &plurality {
                if !named.contains(answer) {
                    named.push(answer.clone());
                }
            }
            parts.push(plurality);
        }
        let accord = if named.len() == 1 {
            Some(named[0].clone())
        } else {
            None
        };
        let all_polls: Vec<&Poll> = askings.iter().map(|(_, poll)| *poll).collect();
        let (union, pooled) = direct_part(&all_polls);
        unions.push(union);
        pluralities.push(parts);
        accords.push(accord);
        pooleds.push(pooled);
    }
    let mut answers: Vec<PText> = Vec::new();
    for union in &unions {
        for answer in union {
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
            if !unions[index].contains(answer) {
                continue;
            }
            let held = pluralities[index]
                .iter()
                .filter(|plurality| plurality.as_ref() == Some(answer))
                .count();
            let part_accord = if accords[index].as_ref() == Some(answer) {
                held
            } else {
                0
            };
            let part_pool = if pooleds[index].as_ref() == Some(answer) {
                held
            } else {
                0
            };
            lines.push(format!("question, {}", question.display()));
            lines.push(format!(
                "held, {}, in accord, {}, with the pool, {}",
                held, part_accord, part_pool
            ));
            offered += 1;
            parts_held += held;
            if accords[index].as_ref() == Some(answer) {
                in_accord += 1;
            }
            if pooleds[index].as_ref() == Some(answer) {
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
    lines
}

/// The sections of an answers reading and its
/// closing figures, parsed off the lines. A section
/// carries its answer, its parts as question and
/// held and in accord and with the pool figures,
/// and its standing figures as questions and parts
/// held and in accord and with the pool.
fn parse_answers(lines: &[String]) -> (Vec<(String, Vec<(String, [i64; 3])>, [i64; 4])>, [i64; 5]) {
    let mut sections: Vec<(String, Vec<(String, [i64; 3])>, [i64; 4])> = Vec::new();
    let mut closing = [0i64; 5];
    let mut current: Option<(String, Vec<(String, [i64; 3])>)> = None;
    for line in lines {
        if let Some(answer) = line.strip_prefix("answer, ") {
            current = Some((answer.to_string(), Vec::new()));
        } else if let Some(question) = line.strip_prefix("question, ") {
            current
                .as_mut()
                .unwrap()
                .1
                .push((question.to_string(), [0; 3]));
        } else if line.starts_with("held, ") {
            let fields: Vec<&str> = line.split(", ").collect();
            assert_eq!(fields.len(), 6, "a held line of no form: {}", line);
            assert_eq!(fields[1 + 1], "in accord");
            assert_eq!(fields[3 + 1], "with the pool");
            let part = current.as_mut().unwrap().1.last_mut().unwrap();
            part.1 = [
                fields[1].parse().unwrap(),
                fields[3].parse().unwrap(),
                fields[5].parse().unwrap(),
            ];
        } else if line.starts_with("standing, ") {
            let (answer, parts) = current.take().unwrap();
            let standing = [
                figure_before(line, "questions"),
                figure_before(line, "parts held"),
                figure_before(line, "in accord"),
                figure_before(line, "with the pool"),
            ];
            sections.push((answer, parts, standing));
        } else if line.starts_with("answers, ") {
            closing = [
                figure_before(line, "answers"),
                figure_before(line, "questions"),
                figure_before(line, "parts"),
                figure_before(line, "in accord"),
                figure_before(line, "with the pool"),
            ];
        } else {
            panic!("a line of no answers form: {}", line);
        }
    }
    (sections, closing)
}

/// The sums GJ-42 states. The closing's figures
/// stand at the sums of the sections on every
/// figure, and each section's standing states the
/// count of its parts and the sum of their held
/// figures.
fn assert_answers_sums(lines: &[String]) {
    let (sections, closing) = parse_answers(lines);
    assert_eq!(closing[0], sections.len() as i64);
    let mut totals = [0i64; 4];
    for (_, parts, standing) in &sections {
        assert_eq!(standing[0], parts.len() as i64);
        assert_eq!(standing[1], parts.iter().map(|part| part.1[0]).sum::<i64>());
        for (slot, figure) in standing.iter().enumerate() {
            totals[slot] += figure;
        }
    }
    assert_eq!(closing[1], totals[0]);
    assert_eq!(closing[2], totals[1]);
    assert_eq!(closing[3], totals[2]);
    assert_eq!(closing[4], totals[3]);
}

fn expected(lines: &[&str]) -> Vec<String> {
    lines.iter().map(|line| line.to_string()).collect()
}

#[test]
fn answers_golden_matches_the_reading_over_the_shipped_sets() {
    let golden = read_text(&examples().join("answers-golden.txt"));
    let golden_lines: Vec<&str> = golden.lines().collect();
    assert_eq!(golden_lines.len(), 573);
    assert_eq!(
        golden_lines[572],
        "answers, 143 answers, 143 questions, 55 parts, 44 in accord, 44 with the pool"
    );
    let pilot = path_arg(&examples().join("pilot"));
    let full = path_arg(&examples().join("full"));
    let wave = path_arg(&examples().join("wave"));
    let (stdout, _, code) = run_cli(&["answers", "pilot", &pilot, "full", &full, "wave", &wave]);
    assert_eq!(code, 0);
    assert_eq!(stdout, golden);
}

#[test]
fn direct_computation_matches_the_answers_reading() {
    let sets = recorded_sets();
    let lines = answers_lines(&sets).unwrap();
    assert_eq!(lines, direct_answers_lines(&sets));
    let pilot = path_arg(&examples().join("pilot"));
    let full = path_arg(&examples().join("full"));
    let wave = path_arg(&examples().join("wave"));
    let (stdout, _, code) = run_cli(&["answers", "pilot", &pilot, "full", &full, "wave", &wave]);
    assert_eq!(code, 0);
    let printed: Vec<String> = stdout.lines().map(|line| line.to_string()).collect();
    assert_eq!(printed, lines);
}

#[test]
fn closing_relations_hold_over_the_shipped_sets() {
    let sets = recorded_sets();
    let lines = answers_lines(&sets).unwrap();
    let closing = lines.last().unwrap();
    let standing = standing_lines(&sets).unwrap();
    let accord = accord_lines(&sets).unwrap();
    let aggregate = aggregate_lines(&sets).unwrap();
    assert_eq!(
        figure_before(closing, "parts"),
        figure_before(standing.last().unwrap(), "with a plurality")
    );
    assert_eq!(
        figure_before(closing, "in accord"),
        figure_before(accord.last().unwrap(), "in accord")
    );
    assert_eq!(
        figure_before(closing, "with the pool"),
        figure_before(aggregate.last().unwrap(), "pluralities")
    );
    assert_eq!(figure_before(closing, "parts"), 55);
    assert_eq!(figure_before(closing, "in accord"), 44);
    assert_eq!(figure_before(closing, "with the pool"), 44);
}

#[test]
fn section_figures_sum_to_the_closing_and_the_generated_closing_stands() {
    let sets = recorded_sets();
    assert_answers_sums(&answers_lines(&sets).unwrap());
    let generated = generated_sets_loaded();
    let lines = answers_lines(&generated).unwrap();
    assert_answers_sums(&lines);
    assert_eq!(
        lines.last().unwrap(),
        "answers, 4 answers, 110 questions, 62 parts, 28 in accord, 34 with the pool"
    );
}

#[test]
fn closing_reconciles_with_the_accord_and_aggregate_closings() {
    let sets = recorded_sets();
    let lines = answers_lines(&sets).unwrap();
    let closing = lines.last().unwrap().clone();
    let standing = standing_lines(&sets).unwrap();
    let parts_with_a_plurality: i64 = standing
        .iter()
        .filter(|line| line.starts_with("standing,"))
        .map(|line| figure_before(line, "with a plurality"))
        .sum();
    assert_eq!(figure_before(&closing, "parts"), parts_with_a_plurality);
    let accord = accord_lines(&sets).unwrap();
    let aggregate = aggregate_lines(&sets).unwrap();
    assert_eq!(
        figure_before(&closing, "in accord"),
        figure_before(accord.last().unwrap(), "in accord")
    );
    assert_eq!(
        figure_before(&closing, "with the pool"),
        figure_before(aggregate.last().unwrap(), "pluralities")
    );
    let generated = generated_sets_loaded();
    let generated_lines = answers_lines(&generated).unwrap();
    let (sections, _) = parse_answers(&generated_lines);
    let generated_standing = standing_lines(&generated).unwrap();
    let generated_accord = accord_lines(&generated).unwrap();
    let generated_aggregate = aggregate_lines(&generated).unwrap();
    assert_eq!(
        sections.iter().map(|section| section.2[1]).sum::<i64>(),
        figure_before(generated_standing.last().unwrap(), "with a plurality")
    );
    assert_eq!(
        sections.iter().map(|section| section.2[2]).sum::<i64>(),
        figure_before(generated_accord.last().unwrap(), "in accord")
    );
    assert_eq!(
        sections.iter().map(|section| section.2[3]).sum::<i64>(),
        figure_before(generated_aggregate.last().unwrap(), "pluralities")
    );
}

#[test]
fn agreement_shape_states_the_agreed_answer_held_and_in_accord() {
    let question = "Which gate should the fair open first?";
    let answers = ["the red gate", "the blue gate"];
    let mut responses = votes_of("one", "the red gate", 3);
    responses.extend(votes_of("two", "the blue gate", 1));
    let alpha = poll(question, &answers, responses);
    let mut responses = votes_of("one", "the red gate", 2);
    responses.extend(votes_of("two", "the blue gate", 1));
    let beta = poll(question, &answers, responses);
    let sets = vec![(pt("alpha"), vec![alpha]), (pt("beta"), vec![beta])];
    assert_eq!(
        answers_lines(&sets).unwrap(),
        expected(&[
            "answer, the red gate",
            "question, Which gate should the fair open first?",
            "held, 2, in accord, 2, with the pool, 2",
            "standing, 1 questions, 2 parts held, 1 in accord, 1 with the pool",
            "answer, the blue gate",
            "question, Which gate should the fair open first?",
            "held, 0, in accord, 0, with the pool, 0",
            "standing, 1 questions, 0 parts held, 0 in accord, 0 with the pool",
            "answers, 2 answers, 2 questions, 2 parts, 1 in accord, 1 with the pool",
        ])
    );
}

#[test]
fn tied_part_states_held_at_zero_and_its_sections_stand() {
    let question = "Which gate should the fair open first?";
    let answers = ["the red gate", "the blue gate"];
    let mut responses = votes_of("one", "the red gate", 2);
    responses.extend(votes_of("two", "the blue gate", 2));
    let alpha = poll(question, &answers, responses);
    let sets = vec![(pt("alpha"), vec![alpha])];
    assert_eq!(
        answers_lines(&sets).unwrap(),
        expected(&[
            "answer, the red gate",
            "question, Which gate should the fair open first?",
            "held, 0, in accord, 0, with the pool, 0",
            "standing, 1 questions, 0 parts held, 0 in accord, 0 with the pool",
            "answer, the blue gate",
            "question, Which gate should the fair open first?",
            "held, 0, in accord, 0, with the pool, 0",
            "standing, 1 questions, 0 parts held, 0 in accord, 0 with the pool",
            "answers, 2 answers, 2 questions, 0 parts, 0 in accord, 0 with the pool",
        ])
    );
}

#[test]
fn empty_question_states_held_at_zero_and_its_sections_stand() {
    let question = "Which gate should the fair open first?";
    let answers = ["the red gate", "the blue gate"];
    let responses: Vec<Response> = (1..=3)
        .map(|seat| {
            response(
                &format!("one seat {}", seat),
                "model a",
                None,
                "no answer recorded",
            )
        })
        .collect();
    let alpha = poll(question, &answers, responses);
    let sets = vec![(pt("alpha"), vec![alpha])];
    assert_eq!(
        answers_lines(&sets).unwrap(),
        expected(&[
            "answer, the red gate",
            "question, Which gate should the fair open first?",
            "held, 0, in accord, 0, with the pool, 0",
            "standing, 1 questions, 0 parts held, 0 in accord, 0 with the pool",
            "answer, the blue gate",
            "question, Which gate should the fair open first?",
            "held, 0, in accord, 0, with the pool, 0",
            "standing, 1 questions, 0 parts held, 0 in accord, 0 with the pool",
            "answers, 2 answers, 2 questions, 0 parts, 0 in accord, 0 with the pool",
        ])
    );
}

#[test]
fn divided_sets_state_held_at_one_and_the_pool_at_the_pooled_answer() {
    let question = "Which gate should the fair open first?";
    let answers = ["the red gate", "the blue gate"];
    let mut responses = votes_of("one", "the red gate", 3);
    responses.extend(votes_of("two", "the blue gate", 1));
    let alpha = poll(question, &answers, responses);
    let mut responses = votes_of("one", "the blue gate", 2);
    responses.extend(votes_of("two", "the red gate", 1));
    let beta = poll(question, &answers, responses);
    let sets = vec![(pt("alpha"), vec![alpha]), (pt("beta"), vec![beta])];
    assert_eq!(
        answers_lines(&sets).unwrap(),
        expected(&[
            "answer, the red gate",
            "question, Which gate should the fair open first?",
            "held, 1, in accord, 0, with the pool, 1",
            "standing, 1 questions, 1 parts held, 0 in accord, 1 with the pool",
            "answer, the blue gate",
            "question, Which gate should the fair open first?",
            "held, 1, in accord, 0, with the pool, 0",
            "standing, 1 questions, 1 parts held, 0 in accord, 0 with the pool",
            "answers, 2 answers, 2 questions, 2 parts, 0 in accord, 1 with the pool",
        ])
    );
}

#[test]
fn accord_where_the_pool_falls_elsewhere_states_with_the_pool_at_zero() {
    let question = "Which gate should the fair repaint first?";
    let answers = ["the red gate", "the blue gate", "the green gate"];
    let mut responses = votes_of("one", "the red gate", 2);
    responses.extend(votes_of("two", "the blue gate", 1));
    let alpha = poll(question, &answers, responses);
    let mut responses = votes_of("one", "the blue gate", 2);
    responses.extend(votes_of("two", "the green gate", 2));
    responses.extend(votes_of("three", "the red gate", 1));
    let beta = poll(question, &answers, responses);
    let sets = vec![(pt("alpha"), vec![alpha]), (pt("beta"), vec![beta])];
    assert_eq!(
        answers_lines(&sets).unwrap(),
        expected(&[
            "answer, the red gate",
            "question, Which gate should the fair repaint first?",
            "held, 1, in accord, 1, with the pool, 0",
            "standing, 1 questions, 1 parts held, 1 in accord, 0 with the pool",
            "answer, the blue gate",
            "question, Which gate should the fair repaint first?",
            "held, 0, in accord, 0, with the pool, 0",
            "standing, 1 questions, 0 parts held, 0 in accord, 0 with the pool",
            "answer, the green gate",
            "question, Which gate should the fair repaint first?",
            "held, 0, in accord, 0, with the pool, 0",
            "standing, 1 questions, 0 parts held, 0 in accord, 0 with the pool",
            "answers, 3 answers, 3 questions, 1 parts, 1 in accord, 0 with the pool",
        ])
    );
}

#[test]
fn answer_offered_by_one_set_alone_stands_and_the_empty_set_moves_no_line() {
    let gates = "Which gate should the fair open first?";
    let fields = "Which field should the fair mow first?";
    let build = |with_empty: bool| {
        let mut gate_responses = votes_of("one", "the red gate", 2);
        gate_responses.extend(votes_of("two", "the blue gate", 1));
        let mut south_responses = votes_of("one", "the south field", 2);
        south_responses.extend(votes_of("two", "the north field", 1));
        let alpha = vec![
            poll(gates, &["the red gate", "the blue gate"], gate_responses),
            poll(
                fields,
                &["the north field", "the south field"],
                south_responses,
            ),
        ];
        let mut east_responses = votes_of("one", "the east field", 3);
        east_responses.extend(votes_of("two", "the north field", 1));
        let beta = vec![poll(
            fields,
            &["the north field", "the east field"],
            east_responses,
        )];
        let mut sets = vec![(pt("alpha"), alpha), (pt("beta"), beta)];
        if with_empty {
            sets.push((pt("empty"), Vec::new()));
        }
        sets
    };
    let lines = answers_lines(&build(true)).unwrap();
    assert_eq!(
        lines,
        expected(&[
            "answer, the red gate",
            "question, Which gate should the fair open first?",
            "held, 1, in accord, 1, with the pool, 1",
            "standing, 1 questions, 1 parts held, 1 in accord, 1 with the pool",
            "answer, the blue gate",
            "question, Which gate should the fair open first?",
            "held, 0, in accord, 0, with the pool, 0",
            "standing, 1 questions, 0 parts held, 0 in accord, 0 with the pool",
            "answer, the north field",
            "question, Which field should the fair mow first?",
            "held, 0, in accord, 0, with the pool, 0",
            "standing, 1 questions, 0 parts held, 0 in accord, 0 with the pool",
            "answer, the south field",
            "question, Which field should the fair mow first?",
            "held, 1, in accord, 0, with the pool, 0",
            "standing, 1 questions, 1 parts held, 0 in accord, 0 with the pool",
            "answer, the east field",
            "question, Which field should the fair mow first?",
            "held, 1, in accord, 0, with the pool, 1",
            "standing, 1 questions, 1 parts held, 0 in accord, 1 with the pool",
            "answers, 5 answers, 5 questions, 3 parts, 1 in accord, 2 with the pool",
        ])
    );
    assert_eq!(lines, answers_lines(&build(false)).unwrap());
}

#[test]
fn answer_at_two_questions_states_each_part_from_its_own_question() {
    let gates = "Which gate should the fair open first?";
    let fields = "Which field should the fair mow first?";
    let mut alpha_gates = votes_of("one", "the red gate", 2);
    alpha_gates.extend(votes_of("two", "the blue gate", 1));
    let mut alpha_fields = votes_of("one", "the red gate", 2);
    alpha_fields.extend(votes_of("two", "the green gate", 1));
    let alpha = vec![
        poll(gates, &["the red gate", "the blue gate"], alpha_gates),
        poll(fields, &["the red gate", "the green gate"], alpha_fields),
    ];
    let mut beta_gates = votes_of("one", "the blue gate", 2);
    beta_gates.extend(votes_of("two", "the red gate", 1));
    let mut beta_fields = votes_of("one", "the green gate", 2);
    beta_fields.extend(votes_of("two", "the red gate", 1));
    let beta = vec![
        poll(gates, &["the red gate", "the blue gate"], beta_gates),
        poll(fields, &["the red gate", "the green gate"], beta_fields),
    ];
    let sets = vec![(pt("alpha"), alpha), (pt("beta"), beta)];
    assert_eq!(
        answers_lines(&sets).unwrap(),
        expected(&[
            "answer, the red gate",
            "question, Which gate should the fair open first?",
            "held, 1, in accord, 0, with the pool, 0",
            "question, Which field should the fair mow first?",
            "held, 1, in accord, 0, with the pool, 0",
            "standing, 2 questions, 2 parts held, 0 in accord, 0 with the pool",
            "answer, the blue gate",
            "question, Which gate should the fair open first?",
            "held, 1, in accord, 0, with the pool, 0",
            "standing, 1 questions, 1 parts held, 0 in accord, 0 with the pool",
            "answer, the green gate",
            "question, Which field should the fair mow first?",
            "held, 1, in accord, 0, with the pool, 0",
            "standing, 1 questions, 1 parts held, 0 in accord, 0 with the pool",
            "answers, 3 answers, 4 questions, 4 parts, 0 in accord, 0 with the pool",
        ])
    );
}

#[test]
fn guard_refuses_a_newline_in_the_model_of_the_second_set() {
    let mut sets = recorded_sets();
    sets[1].1[1].responses[0].model = pt("model a\nanswers, forged");
    let defect = answers_lines(&sets).unwrap_err();
    assert_eq!(defect, "response 0 field model holds a newline");
    assert_eq!(census_lines(&sets).unwrap_err(), defect);
}

#[test]
fn guard_refuses_a_newline_in_the_question_of_the_first_set() {
    let mut sets = recorded_sets();
    let question = sets[0].1[0].question.display();
    sets[0].1[0].question = pt(&format!("{}\nanswers, forged", question));
    let defect = answers_lines(&sets).unwrap_err();
    assert_eq!(defect, "field question holds a newline");
    assert_eq!(census_lines(&sets).unwrap_err(), defect);
}

#[test]
fn set_rules_refuse_the_empty_the_duplicate_and_the_mixed_fixtures() {
    let question = "Which gate should the fair open first?";
    let answers = ["the red gate", "the blue gate"];
    let one = || poll(question, &answers, votes_of("one", "the red gate", 1));
    let sets = vec![(pt(""), vec![one()])];
    assert_eq!(answers_lines(&sets).unwrap_err(), "a set label is empty");
    let sets = vec![(pt("alpha"), vec![one()]), (pt("alpha"), vec![one()])];
    assert_eq!(
        answers_lines(&sets).unwrap_err(),
        "the set label alpha stands twice"
    );
    let other = "Which field should the fair mow first?";
    let sets = vec![
        (
            pt("beta"),
            vec![poll(other, &answers, votes_of("one", "the red gate", 1))],
        ),
        (pt("beta"), vec![one(), one()]),
    ];
    assert_eq!(
        answers_lines(&sets).unwrap_err(),
        "the set label beta stands twice"
    );
}

#[test]
fn no_line_of_the_reading_holds_a_refused_word() {
    let verdict_words = [
        "verdict",
        "winner",
        "wins",
        "correct",
        "incorrect",
        "wrong",
        "best",
        "worst",
        "rank",
        "ranking",
        "ranked",
        "beats",
        "champion",
        "loser",
        "loses",
    ];
    assert!(!verdict_words.contains(&"answers"));
    let golden = read_text(&examples().join("answers-golden.txt"));
    assert!(golden.contains("answers, 143 answers"));
    let openers = golden
        .lines()
        .filter(|line| line.starts_with("answer, "))
        .count();
    assert_eq!(openers, 143);
    let mut lines: Vec<String> = golden.lines().map(|line| line.to_string()).collect();
    lines.extend(answers_lines(&generated_sets_in_memory()).unwrap());
    for line in &lines {
        for token in line
            .split(|c: char| !c.is_alphanumeric())
            .filter(|token| !token.is_empty())
        {
            assert!(
                !verdict_words.contains(&token.to_lowercase().as_str()),
                "a refused word in: {}",
                line
            );
        }
    }
}

#[test]
fn help_text_names_the_answers_command() {
    let (stdout, _, code) = run_cli(&["answers", "--help"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("usage: grandjury answers [-h] label directory [label directory...]"));
    let (stdout, _, code) = run_cli(&["--help"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("print the answers reading over recorded poll sets"));
}

// The generated family of GJ-27, copied from the
// standing tests, the family the answers reading
// states its generated figures over. The draws
// stand in the order the standing file states
// them, and the family the two files generate is
// the one family.
struct Lcg {
    state: u64,
}

impl Lcg {
    fn draw(&mut self, span: u64) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state % span
    }
}

const GEN_ANSWERS: [&str; 4] = [
    "the ash gate",
    "the birch gate",
    "the cedar gate",
    "the elm gate",
];
const GEN_MODELS: [&str; 3] = ["model a", "model b", "model c"];
const GEN_REASONS: [&str; 4] = [
    "the record states it",
    "the shorter path",
    "as the record stands",
    "a stated reason",
];
const GEN_SET_LABELS: [&str; 4] = ["gen-a", "gen-b", "gen-c", "gen-d"];

fn draw_answer(rng: &mut Lcg, offered: usize, favoured: usize) -> usize {
    let total = 2 * offered as u64 + 3;
    let roll = rng.draw(total);
    let mut accrued = 0u64;
    for answer in 0..offered {
        accrued += if answer == favoured { 5 } else { 2 };
        if roll < accrued {
            return answer;
        }
    }
    offered - 1
}

fn generated_sets_in_memory() -> Vec<(PText, Vec<Poll>)> {
    let mut rng = Lcg { state: 20261113 };
    let mut questions: Vec<(usize, usize)> = Vec::new();
    for _ in 0..36 {
        let offered = 2 + rng.draw(3) as usize;
        let favoured = rng.draw(offered as u64) as usize;
        questions.push((offered, favoured));
    }
    let mut askings: Vec<Vec<usize>> = Vec::new();
    for _ in 0..36 {
        let count = 1 + rng.draw(3) as usize;
        let mut chosen: Vec<usize> = Vec::new();
        while chosen.len() < count {
            let candidate = rng.draw(4) as usize;
            if !chosen.contains(&candidate) {
                chosen.push(candidate);
            }
        }
        askings.push(chosen);
    }
    let mut plan: Vec<Vec<(usize, Vec<usize>)>> =
        vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    for (set_index, slot) in plan.iter_mut().enumerate() {
        for (question_index, asking_sets) in askings.iter().enumerate() {
            if asking_sets.contains(&set_index) {
                let model_count = 2 + rng.draw(2) as usize;
                let first = rng.draw(3) as usize;
                let models: Vec<usize> = (0..model_count).map(|step| (first + step) % 3).collect();
                slot.push((question_index, models));
            }
        }
    }
    let mut sets: Vec<(PText, Vec<Poll>)> = Vec::new();
    for (set_index, slot) in plan.iter().enumerate() {
        let mut polls: Vec<Poll> = Vec::new();
        for (question_index, models) in slot {
            let (offered, favoured) = questions[*question_index];
            let mut responses: Vec<Response> = Vec::new();
            for model_index in models {
                for seat in 1..=5 {
                    let juror = format!("{} seat {}", GEN_MODELS[*model_index], seat);
                    if rng.draw(100) < 39 {
                        let drawn = draw_answer(&mut rng, offered, favoured);
                        let reason = GEN_REASONS[rng.draw(4) as usize];
                        let mut held = response(&juror, GEN_MODELS[*model_index], None, reason);
                        held.answer = pt(GEN_ANSWERS[drawn]);
                        responses.push(held);
                    } else {
                        let spoiled = rng.draw(50) == 0;
                        let drawn = draw_answer(&mut rng, offered, favoured);
                        let answer = if spoiled {
                            "the unoffered gate"
                        } else {
                            GEN_ANSWERS[drawn]
                        };
                        let reason = GEN_REASONS[rng.draw(4) as usize];
                        responses.push(response(
                            &juror,
                            GEN_MODELS[*model_index],
                            Some(answer),
                            reason,
                        ));
                    }
                }
            }
            polls.push(Poll {
                question: pt(&format!("generated question {}?", question_index + 1)),
                answers: GEN_ANSWERS[..offered]
                    .iter()
                    .map(|answer| pt(answer))
                    .collect(),
                responses,
            });
        }
        sets.push((pt(GEN_SET_LABELS[set_index]), polls));
    }
    sets
}

fn generated_sets_loaded() -> Vec<(PText, Vec<Poll>)> {
    let scratch = TempDir::new("answers-generated");
    let mut sets: Vec<(PText, Vec<Poll>)> = Vec::new();
    for (index, (label, polls)) in generated_sets_in_memory().iter().enumerate() {
        let dir = scratch.path.join(format!("set-{}", index));
        std::fs::create_dir_all(&dir).unwrap();
        for (offset, held) in polls.iter().enumerate() {
            std::fs::write(
                dir.join(format!("poll-{:02}.json", offset + 1)),
                dumps_default(&held.to_json()),
            )
            .unwrap();
        }
        sets.push((label.clone(), load_set(&dir, polls.len())));
    }
    sets
}

#[test]
fn generated_family_holds_its_figures() {
    let sets = generated_sets_in_memory();
    assert_eq!(questions_of(&sets).len(), 36);
    let polls: Vec<&Poll> = sets.iter().flat_map(|(_, polls)| polls.iter()).collect();
    assert_eq!(polls.len(), 72);
    let responses: Vec<&Response> = polls
        .iter()
        .flat_map(|held| held.responses.iter())
        .collect();
    assert_eq!(responses.len(), 720);
    assert_eq!(responses.iter().filter(|held| held.abstained).count(), 285);
    let spoiled = polls
        .iter()
        .flat_map(|held| {
            held.responses.iter().filter(move |response| {
                !response.abstained && !held.answers.contains(&response.answer)
            })
        })
        .count();
    assert_eq!(spoiled, 12);
    assert_eq!(responses.len() - 285 - spoiled, 423);
    for held in &polls {
        let models: std::collections::HashSet<&PText> = held
            .responses
            .iter()
            .map(|response| &response.model)
            .collect();
        assert_eq!(models.len(), 2);
    }
    let lines = answers_lines(&sets).unwrap();
    let standings: Vec<&str> = lines
        .iter()
        .filter(|line| line.starts_with("standing,"))
        .map(|line| line.as_str())
        .collect();
    assert_eq!(
        standings,
        vec![
            "standing, 36 questions, 4 parts held, 2 in accord, 2 with the pool",
            "standing, 36 questions, 40 parts held, 18 in accord, 21 with the pool",
            "standing, 24 questions, 11 parts held, 3 in accord, 6 with the pool",
            "standing, 14 questions, 7 parts held, 5 in accord, 5 with the pool",
        ]
    );
    assert_eq!(
        lines.last().unwrap(),
        "answers, 4 answers, 110 questions, 62 parts, 28 in accord, 34 with the pool"
    );
}

#[test]
fn generated_recount_matches_the_direct_computation() {
    let loaded = generated_sets_loaded();
    let in_memory = generated_sets_in_memory();
    let lines = answers_lines(&loaded).unwrap();
    assert_eq!(lines, direct_answers_lines(&in_memory));
    let closing = lines.last().unwrap();
    assert_eq!(figure_before(closing, "answers"), 4);
    assert_eq!(figure_before(closing, "questions"), 110);
    assert_eq!(figure_before(closing, "parts"), 62);
    assert_eq!(figure_before(closing, "in accord"), 28);
    assert_eq!(figure_before(closing, "with the pool"), 34);
}

#[test]
fn generated_relations_hold_and_the_sections_sum() {
    let loaded = generated_sets_loaded();
    let lines = answers_lines(&loaded).unwrap();
    assert_answers_sums(&lines);
    let closing = lines.last().unwrap();
    let standing = standing_lines(&loaded).unwrap();
    let accord = accord_lines(&loaded).unwrap();
    let aggregate = aggregate_lines(&loaded).unwrap();
    assert_eq!(
        figure_before(closing, "parts"),
        figure_before(standing.last().unwrap(), "with a plurality")
    );
    assert_eq!(
        figure_before(closing, "in accord"),
        figure_before(accord.last().unwrap(), "in accord")
    );
    assert_eq!(
        figure_before(closing, "with the pool"),
        figure_before(aggregate.last().unwrap(), "pluralities")
    );
}
