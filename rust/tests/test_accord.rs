//! Pins for the accord reading and command.
//!
//! The accord pins are stated in GJ-37 and GJ-38 in
//! CONTRACT.md, in the Round 18 section. One section
//! stands per question in the order the questions
//! first appear across the sets: the question line,
//! then one part per set of the question in the order
//! the sets stand in the input, the set line carrying
//! the label of the set and the counts of the set
//! summed over its askings of the question with its
//! own plurality line and margin line, then the
//! accord line, naming the one answer every set of
//! the question that holds a plurality names, or the
//! word none where no set holds a plurality or two
//! sets name two answers. The closing line counts
//! the questions, the parts, the questions at which
//! an accord stands, and the divided questions. The
//! relations of GJ-38 bind the reading to the
//! aggregate reading and the margin reading and the
//! closings of the aggregate and the canvass and the
//! margin, the parts figure and the in accord figure
//! carrying no relation and standing pinned by
//! independent computation alone.
//!
//! Discrimination, pin by pin. Pins 1 to 21 in this
//! file fail against the Round 17 closure tree by
//! compile absence, the grandjury::accord module and
//! the accord command and this target and the golden
//! standing nowhere in that tree. Beyond that
//! absence, pin 1 pins the golden byte for byte and
//! fails under any mutation that moves a shipped
//! line, pin 2 holds the whole reading against an
//! independent computation over the recorded polls
//! and fails under the mutations summing a part over
//! every set's askings, reading a part's margin off
//! the smallest other count, naming the pooled
//! plurality's largest other answer on the accord
//! line, counting disagreement questions alone in
//! the divided figure, and omitting a part whose
//! valid count stands at zero, pin 3 holds the parts
//! of the one set questions equal in plurality and
//! margin to the pooled parts of the margin reading
//! and fails under the margin mutation through the
//! one compared line at the question the full set
//! asks alone, the other one set questions coinciding
//! under it, pin 4 holds the accord line of every
//! question whose sets all hold a plurality and stand
//! in accord equal to the pooled plurality of its
//! aggregate section and fails under the mutation
//! naming the largest other answer of the pooled
//! counts on the accord line, pin 5 reconciles the
//! closing with the closings of the aggregate and
//! the canvass and the margin and fails under the
//! mutation counting disagreement questions alone as
//! divided, pins 6 to 13 state the hand built shapes
//! of GJ-38, each with its own mutation named at the
//! pin and its coincidence cells stated there, pin
//! 12 excepted from that statement, its shape being
//! a shape the set rules refuse, so it stands
//! restated under FIX-GJ-23 as a refusal pin over
//! its fixture and claims no mutation of the
//! reading, the mutation ME the design names
//! against its first form, a part built from its
//! set's first asking alone, standing uncaught by
//! construction on every record the set rules
//! accept, as the pin states at its place and the
//! amendment records, pins 14 and 15 exercise the
//! guard on the model of the
//! second set and the question of the first, pin 16
//! exercises the set rules, pin 17 holds the reading
//! under the vocabulary scan and passes by design on
//! the words accord and accords, checked word by word
//! against the fifteen refused words, failing under
//! the mutation restating a form word of the reading
//! as a listed word, pins 19 to 21 hold the generated
//! family to its stated figures, its recount, and its
//! relations, and pin 18 names the command in the
//! help of the binary and fails at the base by
//! assertion there, the accord standing as no command
//! in the standing binary, so its help run draws the
//! unknown command refusal at status two where the pin
//! requires status zero. The five command pins of the
//! round stand in test_cli.rs and fail there by
//! assertion against the surface's unknown command
//! refusal, the two decode posture pins among them.
//! Pin 19 in this file carries one further duty,
//! stated at the pin: it passes on the generator
//! GJ-27 states and fails by assertion against the
//! generator form the Round 13 reviews found. The
//! two status pins of the round stand in
//! test_setup.rs and are not discrimination pins, as
//! the design of the round states: they fail against
//! the base sources by assertion and pass on the
//! slice by construction of the status commit. No
//! generated pin is a bound: every generated figure
//! below is a computed equality at the seed.

mod shared;

use grandjury::accord::accord_lines;
use grandjury::aggregate::aggregate_lines;
use grandjury::canvass::canvass_lines;
use grandjury::margin::margin_lines;
use grandjury::polls::{load_poll_file, Poll, Response};
use grandjury::pytext::PText;
use shared::*;

fn response(juror: &str, model: &str, answer: Option<&str>, reason: &str) -> Response {
    match answer {
        Some(text) => Response {
            juror: pt(juror),
            model: pt(model),
            answer: pt(text),
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

/// n standing responses of one jury naming one answer,
/// the jurors labelled by hand under the jury's model
/// string, so a hand-built shape states the counts of
/// one jury without a fixture file.
fn votes_of(answer: &str, count: usize, model: &str) -> Vec<Response> {
    (0..count)
        .map(|i| {
            response(
                &format!("{model} hand {answer} {}", i + 1),
                model,
                Some(answer),
                "the record states it",
            )
        })
        .collect()
}

fn load_poll(path: &std::path::Path) -> Poll {
    let (held, defects) = load_poll_file(path);
    assert!(defects.is_empty(), "{} {:?}", path.display(), defects);
    held.expect("a poll that loads")
}

fn load_set(dir: &std::path::Path, count: usize) -> Vec<Poll> {
    (1..=count)
        .map(|index| load_poll(&dir.join(format!("poll-{:02}.json", index))))
        .collect()
}

fn recorded_sets() -> Vec<(PText, Vec<Poll>)> {
    vec![
        (pt("pilot"), load_set(&examples().join("pilot"), 12)),
        (pt("full"), load_set(&examples().join("full"), 40)),
        (pt("wave"), load_set(&examples().join("wave"), 12)),
    ]
}

fn figure_before(line: &str, word: &str) -> usize {
    for part in line.split(", ") {
        if let Some((figure, _)) = part.split_once(' ') {
            if part.ends_with(word) {
                return figure.parse().expect("a closing figure");
            }
        }
    }
    panic!("no {word} figure in {line}");
}

fn figure_after(line: &str, key: &str) -> usize {
    let start = line.find(key).expect("a count key") + key.len();
    let digits: String = line[start..]
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    digits.parse().expect("a count figure")
}

fn counts_of(line: &str) -> (usize, usize, usize, usize) {
    (
        figure_after(line, "recorded "),
        figure_after(line, "valid "),
        figure_after(line, "abstained "),
        figure_after(line, "spoiled "),
    )
}

type Counts4 = (usize, usize, usize, usize);

fn counts_text(counts: Counts4) -> String {
    format!(
        "recorded {}, valid {}, abstained {}, spoiled {}",
        counts.0, counts.1, counts.2, counts.3
    )
}

/// One part of an accord reading as (set label,
/// counts, plurality value, margin value), the
/// plurality value standing as the answer's text or
/// the word none and the margin value as a figure or
/// the words not counted.
type AccordPart = (String, Counts4, String, String);

/// One section of an accord reading as (question,
/// parts, accord value).
type AccordSection = (String, Vec<AccordPart>, String);

/// The sections of an accord reading and its closing
/// line, parsed from rendered lines, so the relation
/// pins read the accord as text.
fn parse_accord(lines: &[String]) -> (Vec<AccordSection>, String) {
    let mut sections: Vec<AccordSection> = Vec::new();
    let mut closing = String::new();
    for line in lines {
        if let Some(question) = line.strip_prefix("question, ") {
            sections.push((question.to_string(), Vec::new(), String::new()));
        } else if let Some(rest) = line.strip_prefix("set, ") {
            let split = rest.find(", recorded ").expect("a counts tail");
            let label = rest[..split].to_string();
            sections
                .last_mut()
                .expect("a section before its parts")
                .1
                .push((label, counts_of(line), String::new(), String::new()));
        } else if let Some(value) = line.strip_prefix("plurality, ") {
            sections
                .last_mut()
                .expect("a section before its pluralities")
                .1
                .last_mut()
                .expect("a part before its plurality")
                .2 = value.to_string();
        } else if let Some(value) = line.strip_prefix("margin, ") {
            sections
                .last_mut()
                .expect("a section before its margins")
                .1
                .last_mut()
                .expect("a part before its margin")
                .3 = value.to_string();
        } else if let Some(value) = line.strip_prefix("accord, ") {
            sections.last_mut().expect("a section before its accord").2 = value.to_string();
        } else if line.starts_with("accords, ") {
            closing = line.clone();
        }
    }
    (sections, closing)
}

/// One block of a margin reading as (model, counts,
/// plurality value, margin value).
type MarginBlock = (String, Counts4, String, String);

/// One section of a margin reading as (question,
/// pooled counts, pooled plurality value, pooled
/// margin value, blocks).
type MarginSection = (String, Counts4, String, String, Vec<MarginBlock>);

/// The sections of a margin reading and its closing
/// line, parsed from rendered lines, so the one set
/// relation reads the pooled parts of the margin.
fn parse_margin(lines: &[String]) -> (Vec<MarginSection>, String) {
    let mut sections: Vec<MarginSection> = Vec::new();
    let mut closing = String::new();
    // The standing the next plurality or margin line
    // belongs to: 0 the pooled part of the section,
    // 1 the section's last block.
    let mut standing = 0usize;
    for line in lines {
        if let Some(question) = line.strip_prefix("question, ") {
            sections.push((
                question.to_string(),
                (0, 0, 0, 0),
                String::new(),
                String::new(),
                Vec::new(),
            ));
            standing = 0;
        } else if line.starts_with("pool, ") {
            sections.last_mut().expect("a section before its pool").1 = counts_of(line);
            standing = 0;
        } else if let Some(rest) = line.strip_prefix("jury, ") {
            let split = rest.find(", recorded ").expect("a counts tail");
            let model = rest[..split].to_string();
            sections
                .last_mut()
                .expect("a section before its blocks")
                .4
                .push((model, counts_of(line), String::new(), String::new()));
            standing = 1;
        } else if let Some(value) = line.strip_prefix("plurality, ") {
            let section = sections
                .last_mut()
                .expect("a section before its pluralities");
            if standing == 0 {
                section.2 = value.to_string();
            } else {
                section
                    .4
                    .last_mut()
                    .expect("a block before its plurality")
                    .2 = value.to_string();
            }
        } else if let Some(value) = line.strip_prefix("margin, ") {
            let section = sections.last_mut().expect("a section before its margins");
            if standing == 0 {
                section.3 = value.to_string();
            } else {
                section.4.last_mut().expect("a block before its margin").3 = value.to_string();
            }
        } else if line.starts_with("margins, ") {
            closing = line.clone();
        }
    }
    (sections, closing)
}

/// The sections of an aggregate reading as
/// (question, row lines, plurality line) triples and
/// its closing line, parsed from rendered lines.
fn parse_aggregate_full(lines: &[String]) -> (Vec<(String, Vec<String>, String)>, String) {
    let mut sections: Vec<(String, Vec<String>, String)> = Vec::new();
    let mut closing = String::new();
    for line in lines {
        if let Some(question) = line.strip_prefix("question, ") {
            sections.push((question.to_string(), Vec::new(), String::new()));
        } else if line.starts_with("  ") {
            sections
                .last_mut()
                .expect("a section before its rows")
                .1
                .push(line.clone());
        } else if line.starts_with("plurality, ") {
            sections
                .last_mut()
                .expect("a section before its plurality")
                .2 = line.clone();
        } else if line.starts_with("aggregates, ") {
            closing = line.clone();
        }
    }
    (sections, closing)
}

fn askings_of<'p>(sets: &'p [(PText, Vec<Poll>)], question: &str) -> Vec<&'p Poll> {
    let mut askings = Vec::new();
    for (_, polls) in sets {
        for held in polls {
            if held.question.display() == question {
                askings.push(held);
            }
        }
    }
    askings
}

/// The polls of one set at one question, in the order
/// they stand in the set.
fn part_polls_of<'p>(
    sets: &'p [(PText, Vec<Poll>)],
    set_index: usize,
    question: &str,
) -> Vec<&'p Poll> {
    sets[set_index]
        .1
        .iter()
        .filter(|held| held.question.display() == question)
        .collect()
}

/// The indices of the sets holding a poll of the
/// question, in input order.
fn sets_of(sets: &[(PText, Vec<Poll>)], question: &str) -> Vec<usize> {
    let mut indices = Vec::new();
    for (index, (_, polls)) in sets.iter().enumerate() {
        if polls.iter().any(|held| held.question.display() == question) {
            indices.push(index);
        }
    }
    indices
}

/// The questions of the record in the order they
/// first appear across the sets, read directly.
fn questions_of(sets: &[(PText, Vec<Poll>)]) -> Vec<String> {
    let mut questions: Vec<String> = Vec::new();
    for (_, polls) in sets {
        for held in polls {
            let text = held.question.display();
            if !questions.contains(&text) {
                questions.push(text);
            }
        }
    }
    questions
}

/// The plurality of one pool of counted answers read
/// directly, the offered answer holding the largest
/// count, standing only where the pool holds a valid
/// response and one answer holds that count alone.
fn direct_plurality(counts: &[usize], valid: usize, union: &[String]) -> Option<String> {
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

/// The margin of one pool of counted answers read
/// directly, the count of the plurality answer minus
/// the largest count among the other offered answers,
/// standing only where a plurality stands.
fn direct_margin(counts: &[usize], valid: usize, union: &[String]) -> Option<usize> {
    let plurality = direct_plurality(counts, valid, union)?;
    let slot = union
        .iter()
        .position(|answer| answer == &plurality)
        .expect("the plurality slot");
    let largest_other = counts
        .iter()
        .enumerate()
        .filter(|(held, _)| *held != slot)
        .map(|(_, count)| *count)
        .max()
        .unwrap_or(0);
    Some(counts[slot] - largest_other)
}

/// The part of one set at one question computed
/// directly, as (four counts, per answer counts over
/// the union of the part, the union of the part,
/// plurality), every response of the set's polls of
/// the question counted response by response under
/// the validity rule of GJ-8 in the test.
fn direct_part(
    sets: &[(PText, Vec<Poll>)],
    set_index: usize,
    question: &str,
) -> (Counts4, Vec<usize>, Vec<String>, Option<String>) {
    let polls = part_polls_of(sets, set_index, question);
    let mut union: Vec<String> = Vec::new();
    for held in &polls {
        for answer in &held.answers {
            let text = answer.display();
            if !union.contains(&text) {
                union.push(text);
            }
        }
    }
    let mut counts = vec![0usize; union.len()];
    let (mut recorded, mut valid, mut abstained, mut spoiled) = (0, 0, 0, 0);
    for held in &polls {
        for entry in &held.responses {
            recorded += 1;
            if entry.abstained {
                abstained += 1;
            } else if !held.answers.contains(&entry.answer) {
                spoiled += 1;
            } else {
                valid += 1;
                let slot = union
                    .iter()
                    .position(|answer| answer == &entry.answer.display())
                    .expect("a union slot");
                counts[slot] += 1;
            }
        }
    }
    let plurality = direct_plurality(&counts, valid, &union);
    (
        (recorded, valid, abstained, spoiled),
        counts,
        union,
        plurality,
    )
}

/// The accord reading computed directly from the
/// loaded polls with this file's own arithmetic, each
/// part counted response by response over the set's
/// own polls of the question and the accord of a
/// question read off the parts' pluralities in the
/// test, so the pin holds no crate counting.
fn direct_accord_lines(sets: &[(PText, Vec<Poll>)]) -> Vec<String> {
    let mut lines = Vec::new();
    let mut part_total = 0usize;
    let mut accord_total = 0usize;
    for question in questions_of(sets) {
        lines.push(format!("question, {question}"));
        let mut named: Vec<String> = Vec::new();
        for set_index in sets_of(sets, &question) {
            let (totals, counts, union, plurality) = direct_part(sets, set_index, &question);
            let margin = direct_margin(&counts, totals.1, &union);
            part_total += 1;
            if let Some(answer) = &plurality {
                if !named.contains(answer) {
                    named.push(answer.clone());
                }
            }
            lines.push(format!(
                "set, {}, {}",
                sets[set_index].0.display(),
                counts_text(totals)
            ));
            lines.push(format!(
                "plurality, {}",
                plurality.as_deref().unwrap_or("none")
            ));
            lines.push(match margin {
                Some(figure) => format!("margin, {figure}"),
                None => "margin, not counted".to_string(),
            });
        }
        let accord = if named.len() == 1 {
            accord_total += 1;
            named[0].clone()
        } else {
            "none".to_string()
        };
        lines.push(format!("accord, {accord}"));
    }
    lines.push(format!(
        "accords, {} questions, {} parts, {} in accord, {} divided",
        questions_of(sets).len(),
        part_total,
        accord_total,
        questions_of(sets).len() - accord_total
    ));
    lines
}

/// The questions of a record at which every asking
/// set holds a plurality and an accord stands,
/// computed directly from the parts of the test.
fn qualifying_questions(sets: &[(PText, Vec<Poll>)]) -> Vec<String> {
    let mut qualifying = Vec::new();
    for question in questions_of(sets) {
        let mut named: Vec<String> = Vec::new();
        let mut all_hold = true;
        for set_index in sets_of(sets, &question) {
            let (_, _, _, plurality) = direct_part(sets, set_index, &question);
            match plurality {
                Some(answer) => {
                    if !named.contains(&answer) {
                        named.push(answer);
                    }
                }
                None => all_hold = false,
            }
        }
        if all_hold && named.len() == 1 {
            qualifying.push(question);
        }
    }
    qualifying
}

/// The one set relation of GJ-38, checked over any
/// sets: at a question asked by exactly one set, the
/// plurality line and the margin line of its one part
/// stand equal to the plurality line and the margin
/// line of the pooled part of that question's section
/// in the margin reading, and the accord line of the
/// section names that part's plurality answer or the
/// word none.
fn assert_one_set_relations(sets: &[(PText, Vec<Poll>)]) {
    let lines = accord_lines(sets).expect("the accord reading computes");
    let (sections, _) = parse_accord(&lines);
    let margin = margin_lines(sets).expect("the margin reading computes");
    let (held_sections, _) = parse_margin(&margin);
    assert_eq!(sections.len(), held_sections.len());
    for ((question, parts, accord), (held_question, _, pooled_plurality, pooled_margin, _)) in
        sections.iter().zip(held_sections.iter())
    {
        assert_eq!(question, held_question);
        if sets_of(sets, question).len() != 1 {
            continue;
        }
        assert_eq!(parts.len(), 1, "the one part of {question}");
        assert_eq!(&parts[0].2, pooled_plurality, "the plurality at {question}");
        assert_eq!(&parts[0].3, pooled_margin, "the margin at {question}");
        assert_eq!(accord, &parts[0].2, "the accord at {question}");
    }
}

/// The accord relation of GJ-38, checked over any
/// sets at the qualifying questions the caller
/// counted directly: where every set asking a
/// question holds a plurality and the question stands
/// in accord, the accord line of its section names
/// the plurality of the pooled part of the same
/// question's section in the aggregate reading.
fn assert_accord_pooled_relations(sets: &[(PText, Vec<Poll>)], qualifying: &[String]) {
    let lines = accord_lines(sets).expect("the accord reading computes");
    let (sections, _) = parse_accord(&lines);
    let aggregate = aggregate_lines(sets).expect("the aggregate reading computes");
    let (held_sections, _) = parse_aggregate_full(&aggregate);
    assert_eq!(sections.len(), held_sections.len());
    for ((question, _, accord), (held_question, _, held_line)) in
        sections.iter().zip(held_sections.iter())
    {
        assert_eq!(question, held_question);
        if qualifying.contains(question) {
            assert_eq!(
                &format!("plurality, {accord}"),
                held_line,
                "the pooled plurality at {question}"
            );
        }
    }
}

/// The closing relations of GJ-38, checked over any
/// sets: the questions figure of the closing equals
/// the questions figures of the closings of the
/// aggregate and the canvass and the margin over the
/// same sets, the in accord figure and the divided
/// figure sum to the questions figure, and the parts
/// figure equals the count of the parts the sets of
/// the record state, recounted here by walking the
/// sets question by question.
fn assert_closing_relations(sets: &[(PText, Vec<Poll>)]) {
    let lines = accord_lines(sets).expect("the accord reading computes");
    let (_, closing) = parse_accord(&lines);
    let aggregate = aggregate_lines(sets).expect("the aggregate reading computes");
    let (_, aggregate_closing) = parse_aggregate_full(&aggregate);
    let canvass = canvass_lines(sets).expect("the canvass reading computes");
    let canvass_closing = canvass.last().expect("a canvass closing");
    let margin = margin_lines(sets).expect("the margin reading computes");
    let (_, margin_closing) = parse_margin(&margin);
    assert_eq!(
        figure_before(&closing, "questions"),
        figure_before(&aggregate_closing, "questions")
    );
    assert_eq!(
        figure_before(&closing, "questions"),
        figure_before(canvass_closing, "questions")
    );
    assert_eq!(
        figure_before(&closing, "questions"),
        figure_before(&margin_closing, "questions")
    );
    assert_eq!(
        figure_before(&closing, "in accord") + figure_before(&closing, "divided"),
        figure_before(&closing, "questions")
    );
    let recounted: usize = questions_of(sets)
        .iter()
        .map(|question| sets_of(sets, question).len())
        .sum();
    assert_eq!(figure_before(&closing, "parts"), recounted);
}

#[test]
fn the_accord_golden_stands_byte_for_byte() {
    let golden = read_text(&examples().join("accord-golden.txt"));
    let (stdout, _, code) = run_cli(&[
        "accord",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "full",
        &path_arg(&examples().join("full")),
        "wave",
        &path_arg(&examples().join("wave")),
    ]);
    assert_eq!(code, 0);
    assert_eq!(stdout, golden);
}

#[test]
fn the_accord_reading_computes_directly_from_the_record() {
    let sets = recorded_sets();
    let expected = direct_accord_lines(&sets);
    assert_eq!(
        accord_lines(&sets).expect("the accord reading computes"),
        expected
    );
    let (stdout, _, code) = run_cli(&[
        "accord",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "full",
        &path_arg(&examples().join("full")),
        "wave",
        &path_arg(&examples().join("wave")),
    ]);
    assert_eq!(code, 0);
    let printed: Vec<String> = stdout.lines().map(str::to_string).collect();
    assert_eq!(printed, expected);
}

#[test]
fn the_accord_parts_at_one_set_questions_stand_equal_to_the_margin_pooled_parts() {
    // The one set relation of GJ-38 over the recorded
    // sets at the function, quantifying over the
    // shipped questions at one set, counted here as
    // an equality before the relation runs so a record
    // change that moved a question across the one set
    // line would fail this pin rather than empty it.
    let sets = recorded_sets();
    let single = questions_of(&sets)
        .iter()
        .filter(|question| sets_of(&sets, question).len() == 1)
        .count();
    assert_eq!(single, 40, "the one set questions of the record");
    assert_one_set_relations(&sets);
}

#[test]
fn the_pooled_plurality_of_an_accord_question_stands_equal_to_its_accord() {
    // The accord relation of GJ-38 over the recorded
    // sets, quantifying over the shipped questions at
    // which every asking set holds a plurality and an
    // accord stands, counted here directly before the
    // relation runs.
    let sets = recorded_sets();
    let qualifying = qualifying_questions(&sets);
    assert_eq!(
        qualifying.len(),
        44,
        "the qualifying questions of the record"
    );
    assert_accord_pooled_relations(&sets, &qualifying);
}

#[test]
fn the_accord_closing_reconciles_with_the_aggregate_and_margin_closings() {
    assert_closing_relations(&recorded_sets());
}

#[test]
fn two_sets_naming_different_pluralities_stand_divided() {
    // Shape S1 of GJ-38. The first set names the red
    // gate at three to one, so its part plurality is
    // the red gate at a margin of two. The second
    // names the blue gate at two to one, so its part
    // plurality is the blue gate at a margin of one.
    // The pooled counts stand at four and three, so
    // the pooled plurality of the question stands at
    // the red gate, but the sets name two answers and
    // the question stands divided with the accord line
    // at none. The pin fails under the mutation
    // naming the pooled plurality on the accord line,
    // which would state the red gate where the pin
    // requires none. The mutation standing the accord
    // only where every set holds a plurality stands
    // against this pin, the named answers differing
    // under both forms.
    let question = "Which gate should the night watch use?";
    let answers = &["the red gate", "the blue gate"];
    let mut alpha = votes_of("the red gate", 3, "model a");
    alpha.extend(votes_of("the blue gate", 1, "model a"));
    let mut beta = votes_of("the blue gate", 2, "model a");
    beta.extend(votes_of("the red gate", 1, "model a"));
    let sets = vec![
        (pt("alpha"), vec![poll(question, answers, alpha)]),
        (pt("beta"), vec![poll(question, answers, beta)]),
    ];
    assert_eq!(
        accord_lines(&sets).expect("the accord reading computes"),
        vec![
            "question, Which gate should the night watch use?",
            "set, alpha, recorded 4, valid 4, abstained 0, spoiled 0",
            "plurality, the red gate",
            "margin, 2",
            "set, beta, recorded 3, valid 3, abstained 0, spoiled 0",
            "plurality, the blue gate",
            "margin, 1",
            "accord, none",
            "accords, 1 questions, 2 parts, 0 in accord, 1 divided",
        ]
    );
}

#[test]
fn an_accord_stands_where_one_set_holds_no_plurality() {
    // Shape S2 of GJ-38. The first set names the red
    // gate at two to one, so its part plurality is the
    // red gate at a margin of one. The second set's
    // summed top counts tie at two and two with the
    // red gate among them, so its part states none and
    // not counted. The one holding set names the red
    // gate, so the accord line names it. The pin fails
    // under the mutation standing the accord only
    // where every set holds a plurality, which would
    // state none, and under the mutation standing a
    // plurality at a tied top count, which would move
    // the tied part's plurality line from none to the
    // first answer of its union.
    let question = "Which gate should the night watch use?";
    let answers = &["the red gate", "the blue gate"];
    let mut alpha = votes_of("the red gate", 2, "model a");
    alpha.extend(votes_of("the blue gate", 1, "model a"));
    let mut beta = votes_of("the red gate", 2, "model a");
    beta.extend(votes_of("the blue gate", 2, "model a"));
    let sets = vec![
        (pt("alpha"), vec![poll(question, answers, alpha)]),
        (pt("beta"), vec![poll(question, answers, beta)]),
    ];
    assert_eq!(
        accord_lines(&sets).expect("the accord reading computes"),
        vec![
            "question, Which gate should the night watch use?",
            "set, alpha, recorded 3, valid 3, abstained 0, spoiled 0",
            "plurality, the red gate",
            "margin, 1",
            "set, beta, recorded 4, valid 4, abstained 0, spoiled 0",
            "plurality, none",
            "margin, not counted",
            "accord, the red gate",
            "accords, 1 questions, 2 parts, 1 in accord, 0 divided",
        ]
    );
}

#[test]
fn an_accord_stands_where_the_pooled_plurality_falls() {
    // Shape S3 of GJ-38, the hand built standing of
    // the boundary the section states as a counted
    // absence at both records. The first set names the
    // red gate at two to one over three answers. The
    // second set holds no plurality, its counts tied
    // at two for the blue gate and two for the green
    // gate with the red gate standing strictly below
    // the tied pair at one, so the pooled counts tie
    // the red gate and the blue gate at three and the
    // pooled plurality falls, while the accord line
    // names the red gate the one holding set names.
    // The pin fails under the mutation naming the
    // pooled plurality on the accord line, which would
    // state none, and under the mutation standing the
    // accord only where every set holds a plurality.
    let question = "Which gate should the night watch use?";
    let answers = &["the red gate", "the blue gate", "the green gate"];
    let mut alpha = votes_of("the red gate", 2, "model a");
    alpha.extend(votes_of("the blue gate", 1, "model a"));
    let mut beta = votes_of("the blue gate", 2, "model a");
    beta.extend(votes_of("the green gate", 2, "model a"));
    beta.extend(votes_of("the red gate", 1, "model a"));
    let sets = vec![
        (pt("alpha"), vec![poll(question, answers, alpha)]),
        (pt("beta"), vec![poll(question, answers, beta)]),
    ];
    let lines = accord_lines(&sets).expect("the accord reading computes");
    assert_eq!(
        lines,
        vec![
            "question, Which gate should the night watch use?",
            "set, alpha, recorded 3, valid 3, abstained 0, spoiled 0",
            "plurality, the red gate",
            "margin, 1",
            "set, beta, recorded 5, valid 5, abstained 0, spoiled 0",
            "plurality, none",
            "margin, not counted",
            "accord, the red gate",
            "accords, 1 questions, 2 parts, 1 in accord, 0 divided",
        ]
    );
    let aggregate = aggregate_lines(&sets).expect("the aggregate reading computes");
    let (sections, _) = parse_aggregate_full(&aggregate);
    assert_eq!(
        sections[0].2, "plurality, none",
        "the pooled plurality falls"
    );
}

#[test]
fn a_question_no_set_holds_a_plurality_at_stands_divided() {
    // Shape S4 of GJ-38. Both sets hold valid
    // responses and both stand tied at the top, the
    // first at one and one and the second at two and
    // two, so every part states none and not counted
    // and the accord line states none. The pin fails
    // under the mutation standing a plurality at a
    // tied top count, the first answer of the union,
    // which would move six lines of the fixture. The
    // mutation moves no line of the shipped record,
    // every tied part there holding no valid response,
    // and it is caught over the generated record by
    // the generated pins beside this one.
    let question = "Which gate should the night watch use?";
    let answers = &["the red gate", "the blue gate"];
    let mut alpha = votes_of("the red gate", 1, "model a");
    alpha.extend(votes_of("the blue gate", 1, "model a"));
    let mut beta = votes_of("the red gate", 2, "model a");
    beta.extend(votes_of("the blue gate", 2, "model a"));
    let sets = vec![
        (pt("alpha"), vec![poll(question, answers, alpha)]),
        (pt("beta"), vec![poll(question, answers, beta)]),
    ];
    assert_eq!(
        accord_lines(&sets).expect("the accord reading computes"),
        vec![
            "question, Which gate should the night watch use?",
            "set, alpha, recorded 2, valid 2, abstained 0, spoiled 0",
            "plurality, none",
            "margin, not counted",
            "set, beta, recorded 4, valid 4, abstained 0, spoiled 0",
            "plurality, none",
            "margin, not counted",
            "accord, none",
            "accords, 1 questions, 2 parts, 0 in accord, 1 divided",
        ]
    );
}

#[test]
fn a_set_with_no_valid_response_still_stands_as_a_part() {
    // Shape S5 of GJ-38. The first set names the red
    // gate twice, so its part plurality is the red
    // gate at a margin of two. The second set abstains
    // three times, so its valid responses stand at
    // zero, and it still stands as a part of the
    // section, its counts on its set line and its
    // plurality line at none, beside the part whose
    // plurality the accord line names. The pin fails
    // under the mutation omitting a part whose valid
    // count stands at zero, which would move five
    // lines of the fixture and, over the shipped
    // record, the parts figure from 64 to 55.
    let question = "Which gate should the night watch use?";
    let answers = &["the red gate", "the blue gate"];
    let alpha = votes_of("the red gate", 2, "model a");
    let beta = vec![
        response("model a hand 1", "model a", None, "unused"),
        response("model a hand 2", "model a", None, "unused"),
        response("model a hand 3", "model a", None, "unused"),
    ];
    let sets = vec![
        (pt("alpha"), vec![poll(question, answers, alpha)]),
        (pt("beta"), vec![poll(question, answers, beta)]),
    ];
    assert_eq!(
        accord_lines(&sets).expect("the accord reading computes"),
        vec![
            "question, Which gate should the night watch use?",
            "set, alpha, recorded 2, valid 2, abstained 0, spoiled 0",
            "plurality, the red gate",
            "margin, 2",
            "set, beta, recorded 3, valid 0, abstained 3, spoiled 0",
            "plurality, none",
            "margin, not counted",
            "accord, the red gate",
            "accords, 1 questions, 2 parts, 1 in accord, 0 divided",
        ]
    );
}

#[test]
fn an_answer_offered_by_one_set_alone_sets_its_parts_plurality() {
    // Shape S6 of GJ-38. The first set offers the
    // north field and the south field and names the
    // south field at two to one. The second offers
    // the north field and the east field and names
    // the east field at three to one, the east field
    // standing offered by that set alone, so its part
    // plurality is read off its own union. The pin
    // fails under the mutation gathering the union of
    // a part from the first set's askings of the
    // question, answers outside that union not
    // counted, under which the second set's part
    // plurality falls from the east field to the north
    // field. The mutation moves no line of either
    // record of the round, and this pin is its only
    // catch, stated at design time.
    let question = "Which field should host the maypole?";
    let mut alpha = votes_of("the south field", 2, "model a");
    alpha.extend(votes_of("the north field", 1, "model a"));
    let mut beta = votes_of("the east field", 3, "model a");
    beta.extend(votes_of("the north field", 1, "model a"));
    let sets = vec![
        (
            pt("alpha"),
            vec![poll(
                question,
                &["the north field", "the south field"],
                alpha,
            )],
        ),
        (
            pt("beta"),
            vec![poll(question, &["the north field", "the east field"], beta)],
        ),
    ];
    assert_eq!(
        accord_lines(&sets).expect("the accord reading computes"),
        vec![
            "question, Which field should host the maypole?",
            "set, alpha, recorded 3, valid 3, abstained 0, spoiled 0",
            "plurality, the south field",
            "margin, 1",
            "set, beta, recorded 4, valid 4, abstained 0, spoiled 0",
            "plurality, the east field",
            "margin, 2",
            "accord, none",
            "accords, 1 questions, 2 parts, 0 in accord, 1 divided",
        ]
    );
}

#[test]
fn a_set_asking_a_question_twice_draws_the_set_rules_refusal() {
    // Shape S7 of GJ-38, as FIX-GJ-23 restates it.
    // The first set holds two polls of the question,
    // the first naming the low meadow at two to one
    // and the second naming the high meadow at three
    // to none, and the second set names the high
    // meadow at one to none. As landed, GJ-38 stated
    // one part summed over both askings of the first
    // set, and this pin asserted that part. That
    // statement describes a record the set rules
    // refuse, GJ-37 holding the set rules of GJ-11
    // unchanged and GJ-11 refusing a set that holds
    // one question twice, so the accord reading
    // refuses over the whole record, no summed part
    // stands, and the pin stands restated as a
    // refusal pin over the fixture unchanged,
    // asserting the refusal GJ-11 names.
    // The mutation the design names ME, a part built
    // from its set's first asking alone, stands
    // uncatchable by construction and this pin does
    // not claim it, because on every record the set
    // rules accept the first asking of a set at a
    // question is its only asking, so the mutated
    // form and the standing form state the same part
    // everywhere the reading runs. The pin's
    // discrimination stands against a guard mutation
    // that drops or reorders the validate_sets call
    // in the accord path, under which the reading
    // would state the summed part the removed
    // sentence stated and this pin fails.
    let question = "Which meadow should host the fair?";
    let answers = &["the low meadow", "the high meadow"];
    let mut first = votes_of("the low meadow", 2, "model a");
    first.extend(votes_of("the high meadow", 1, "model a"));
    let second = votes_of("the high meadow", 3, "model a");
    let beta = votes_of("the high meadow", 1, "model a");
    let sets = vec![
        (
            pt("alpha"),
            vec![
                poll(question, answers, first),
                poll(question, answers, second),
            ],
        ),
        (pt("beta"), vec![poll(question, answers, beta)]),
    ];
    assert_eq!(
        accord_lines(&sets),
        Err(format!("the set alpha holds the question {question} twice"))
    );
}

#[test]
fn where_every_set_holds_the_same_plurality_the_accord_names_it() {
    // Shape S8 of GJ-38. Both sets name the red gate,
    // the first at three to one and the second at two
    // to one, so the accord line names it and the
    // pooled plurality of the aggregate section names
    // it also, the relation of GJ-38 standing on a
    // hand built record beside the records. The pin
    // fails under the mutation naming the largest
    // other answer of the pooled counts on the accord
    // line, which would move the accord line from the
    // red gate to the blue gate.
    let question = "Which gate should the night watch use?";
    let answers = &["the red gate", "the blue gate"];
    let mut alpha = votes_of("the red gate", 3, "model a");
    alpha.extend(votes_of("the blue gate", 1, "model a"));
    let mut beta = votes_of("the red gate", 2, "model a");
    beta.extend(votes_of("the blue gate", 1, "model a"));
    let sets = vec![
        (pt("alpha"), vec![poll(question, answers, alpha)]),
        (pt("beta"), vec![poll(question, answers, beta)]),
    ];
    let lines = accord_lines(&sets).expect("the accord reading computes");
    assert_eq!(
        lines,
        vec![
            "question, Which gate should the night watch use?",
            "set, alpha, recorded 4, valid 4, abstained 0, spoiled 0",
            "plurality, the red gate",
            "margin, 2",
            "set, beta, recorded 3, valid 3, abstained 0, spoiled 0",
            "plurality, the red gate",
            "margin, 1",
            "accord, the red gate",
            "accords, 1 questions, 2 parts, 1 in accord, 0 divided",
        ]
    );
    let aggregate = aggregate_lines(&sets).expect("the aggregate reading computes");
    let (sections, _) = parse_aggregate_full(&aggregate);
    assert_eq!(sections[0].2, "plurality, the red gate");
}

#[test]
fn the_accord_guard_reads_the_model_of_the_second_set() {
    // The guard exercise of GJ-37. The accord prints
    // the set labels of the record on the set lines of
    // its parts and the question strings and the
    // offered answers beside them, and the guard as
    // FIX-GJ-18 left it reads the model of every
    // response before any set rule is read, so a
    // handed poll in the second set whose one response
    // carries a model string holding a closing line
    // after a break draws the guard's model defect at
    // the accord, with no widening of the guard
    // standing in this round. The pin fails under the
    // mutation reading the guard over the first part's
    // set of each section only, which would state the
    // reading of the first set before the refusal, and
    // it stands under the mutations reading the guard
    // after the set rules and per set at first touch.
    let handed = Response {
        juror: pt("beta hand 1"),
        model: pt("model a\naccords, 9 questions, 9 parts"),
        answer: pt("run it"),
        reason: pt("the record states it"),
        abstained: false,
    };
    let sets = vec![
        (
            pt("alpha"),
            vec![poll(
                "Should the ferry run on Christmas Day?",
                &["run it", "rest the crew"],
                votes_of("run it", 1, "model a"),
            )],
        ),
        (
            pt("beta"),
            vec![poll(
                "Should the ferry run on Christmas Day?",
                &["run it", "rest the crew"],
                vec![handed],
            )],
        ),
    ];
    assert_eq!(
        accord_lines(&sets),
        Err("response 0 field model holds a newline".to_string())
    );
}

#[test]
fn the_accord_guard_reads_the_question_of_the_first_set() {
    // The second guard exercise of GJ-37. The accord
    // prints the question strings of the record on its
    // question lines, and the guard reads the question
    // of a poll before its answers and its responses,
    // so a handed poll in the first set whose question
    // string carries a closing line after a break
    // draws the guard's question defect at the accord,
    // before the clean set behind it is read. The pin
    // discriminates no order variant of the guard, the
    // discrimination of the variants standing at its
    // fellow, as the design states plainly.
    let sets = vec![
        (
            pt("alpha"),
            vec![poll(
                "Should the ferry run on Christmas Day?\naccords, 9 questions, 9 parts",
                &["run it", "rest the crew"],
                votes_of("run it", 1, "model a"),
            )],
        ),
        (
            pt("beta"),
            vec![poll(
                "Which gate should the night watch use?",
                &["the north gate", "the south gate"],
                votes_of("the north gate", 1, "model a"),
            )],
        ),
    ];
    assert_eq!(
        accord_lines(&sets),
        Err("field question holds a newline".to_string())
    );
}

#[test]
fn the_accord_set_rules_refuse_as_the_census_refuses() {
    let first = vec![poll(
        "Should the ferry run on Christmas Day?",
        &["run it", "rest the crew"],
        votes_of("run it", 1, "model a"),
    )];
    let second = vec![poll(
        "Which gate should the night watch use?",
        &["the north gate", "the south gate"],
        votes_of("the north gate", 1, "model a"),
    )];
    let empty_label = vec![(pt(""), first.clone())];
    assert_eq!(
        accord_lines(&empty_label),
        Err("a set label is empty".to_string())
    );
    let duplicated_label = vec![(pt("alpha"), first.clone()), (pt("alpha"), second.clone())];
    assert_eq!(
        accord_lines(&duplicated_label),
        Err("the set label alpha stands twice".to_string())
    );
    // The mixed fixture: the second set carries a
    // duplicate label and a repeated question in one
    // call, and the standing order names the label
    // before the questions of its set, so the refusal
    // is the duplicate label defect. The pin fails
    // under the mutation moving the per set question
    // loop ahead of the label checks, which would name
    // the question defect in its place.
    let mixed = vec![
        (pt("alpha"), first),
        (pt("alpha"), vec![second[0].clone(), second[0].clone()]),
    ];
    assert_eq!(
        accord_lines(&mixed),
        Err("the set label alpha stands twice".to_string())
    );
}

#[test]
fn no_verdict_word_stands_in_the_accord_reading() {
    const VERDICT_WORDS: [&str; 15] = [
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
    let golden = read_text(&examples().join("accord-golden.txt"));
    let golden_words: Vec<String> = golden
        .to_lowercase()
        .split(|c: char| !c.is_ascii_lowercase())
        .filter(|word| !word.is_empty())
        .map(|word| word.to_string())
        .collect();
    for word in VERDICT_WORDS {
        assert!(!golden_words.contains(&word.to_string()), "{}", word);
    }
    let (out, _, code) = run_cli(&[
        "accord",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "full",
        &path_arg(&examples().join("full")),
        "wave",
        &path_arg(&examples().join("wave")),
    ]);
    assert_eq!(code, 0);
    let out_words: Vec<String> = out
        .to_lowercase()
        .split(|c: char| !c.is_ascii_lowercase())
        .filter(|word| !word.is_empty())
        .map(|word| word.to_string())
        .collect();
    for word in VERDICT_WORDS {
        assert!(!out_words.contains(&word.to_string()), "{}", word);
    }
    // The pin's own additions beside the scan of the
    // earlier rounds: a set label stands in one
    // position alone in the reading, the set line of a
    // part, so no line but a set line carries a label
    // of the record, the closing word of the reading
    // is accords, and the words accord and accords
    // stand outside the verdict list word by word, the
    // closing word standing in the golden so the check
    // is not vacuous.
    assert!(!VERDICT_WORDS.contains(&"accord"));
    assert!(!VERDICT_WORDS.contains(&"accords"));
    assert!(golden_words.contains(&"accords".to_string()));
    for text in [&golden, &out] {
        let mut set_lines = 0usize;
        for line in text.lines() {
            if line.starts_with("set, ") {
                set_lines += 1;
                let label = line
                    .strip_prefix("set, ")
                    .expect("a set line")
                    .split(", recorded ")
                    .next()
                    .expect("a set label");
                assert!(
                    ["pilot", "full", "wave"].contains(&label),
                    "a set line carries a label of the record: {line}"
                );
            } else {
                assert!(
                    !line.contains("pilot") && !line.contains("wave"),
                    "no line but a set line names a set: {line}"
                );
            }
        }
        assert_eq!(set_lines, 64, "the parts of the shipped reading");
        let closing = text.lines().last().expect("a closing line");
        assert!(
            closing.starts_with("accords, "),
            "the closing word of the reading is accords: {closing}"
        );
    }
}

#[test]
fn the_accord_help_names_the_command() {
    let (stdout, _, code) = run_cli(&["accord", "--help"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("usage: grandjury accord [-h] label directory [label directory ...]"));
    let (top, _, top_code) = run_cli(&["--help"]);
    assert_eq!(top_code, 0);
    assert!(top.contains("print the accord reading over recorded poll sets"));
}

// The generated fixture family of GJ-27, restated in
// this target as GJ-37 states: polls drawn in the
// test from the stated distribution at the stated
// seed, loaded through the crate's own loader, and
// pinned by the stated figures of GJ-38, by an
// independent recount inside the test, and by the
// relations of GJ-38. No golden ships for the family,
// as GJ-27 states: its sections are pinned by the
// figures and the relations and the recount, never by
// a stored rendering.

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

/// The generated sets in memory, drawn in the phases
/// GJ-27 states. First, for each question in question
/// order, its offered count and its favoured answer.
/// Then, for each question in question order, its
/// asking count and the sets it stands asked in. Then,
/// for each set in set order, for each question the
/// set asks in question order, the poll's model count
/// and its first model. Then, for each poll in that
/// order, for each model in its drawn order, for each
/// seat in seat order, the kind draw, the spoiled draw
/// where the response stands, the answer draw every
/// response consumes, and the reason draw: a valid or
/// abstained response names the answer drawn, and a
/// spoiled response names the unoffered gate instead.
fn generated_sets_in_memory() -> Vec<(PText, Vec<Poll>)> {
    let mut rng = Lcg { state: 20261113 };
    // First phase: for each question in question order,
    // its offered count over the span two to four and
    // its favoured answer over its offered answers.
    let mut offered_of: Vec<usize> = Vec::new();
    let mut favoured_of: Vec<usize> = Vec::new();
    for _ in 0..36 {
        let offered = 2 + rng.draw(3) as usize;
        let favoured = rng.draw(offered as u64) as usize;
        offered_of.push(offered);
        favoured_of.push(favoured);
    }
    // Second phase: for each question in question
    // order, its asking count over the span one to
    // three, and draws over the four sets until that
    // many distinct sets stand drawn, the sets of a
    // question standing in the order drawn.
    let mut asking_sets_of: Vec<Vec<usize>> = Vec::new();
    for _ in 0..36 {
        let asking_count = 1 + rng.draw(3) as usize;
        let mut set_idxs: Vec<usize> = Vec::new();
        while set_idxs.len() < asking_count {
            let idx = rng.draw(4) as usize;
            if !set_idxs.contains(&idx) {
                set_idxs.push(idx);
            }
        }
        asking_sets_of.push(set_idxs);
    }
    // Third phase: per set in set order, per asked
    // question in question order, the poll's model
    // count over the span two to three and its first
    // model over the three model strings, the models of
    // the poll standing as that many consecutive
    // strings of the list from the string drawn,
    // wrapping at the end of the list.
    let mut plan: Vec<Vec<(usize, Vec<usize>)>> = vec![Vec::new(); 4];
    for (set_idx, slot) in plan.iter_mut().enumerate() {
        for (question_idx, asking_sets) in asking_sets_of.iter().enumerate() {
            if !asking_sets.contains(&set_idx) {
                continue;
            }
            let model_count = 2 + rng.draw(2) as usize;
            let first = rng.draw(3) as usize;
            let models: Vec<usize> = (0..model_count).map(|k| (first + k) % 3).collect();
            slot.push((question_idx, models));
        }
    }
    // Fourth phase: the responses of each poll in that
    // order, per model in its drawn order, per seat in
    // seat order.
    let mut sets: Vec<(PText, Vec<Poll>)> = Vec::new();
    for (set_idx, questions) in plan.iter().enumerate() {
        let mut polls: Vec<Poll> = Vec::new();
        for (question_idx, models) in questions {
            let offered = offered_of[*question_idx];
            let favoured = favoured_of[*question_idx];
            let mut responses: Vec<Response> = Vec::new();
            for model_idx in models {
                for seat in 1..=5usize {
                    let juror = format!("{} seat {}", GEN_MODELS[*model_idx], seat);
                    if rng.draw(100) < 39 {
                        // An abstained response consumes
                        // the answer draw and the reason
                        // draw and names the answer drawn,
                        // as GJ-27 states.
                        let drawn = draw_answer(&mut rng, offered, favoured);
                        let reason = GEN_REASONS[rng.draw(4) as usize];
                        responses.push(Response {
                            juror: pt(&juror),
                            model: pt(GEN_MODELS[*model_idx]),
                            answer: pt(GEN_ANSWERS[drawn]),
                            reason: pt(reason),
                            abstained: true,
                        });
                        continue;
                    }
                    let spoiled = rng.draw(50) == 0;
                    let drawn = draw_answer(&mut rng, offered, favoured);
                    let answer = if spoiled {
                        "the unoffered gate"
                    } else {
                        GEN_ANSWERS[drawn]
                    };
                    let reason = GEN_REASONS[rng.draw(4) as usize];
                    responses.push(Response {
                        juror: pt(&juror),
                        model: pt(GEN_MODELS[*model_idx]),
                        answer: pt(answer),
                        reason: pt(reason),
                        abstained: false,
                    });
                }
            }
            polls.push(Poll {
                question: pt(&format!("generated question {}?", question_idx + 1)),
                answers: GEN_ANSWERS[..offered].iter().map(|a| pt(a)).collect(),
                responses,
            });
        }
        sets.push((pt(GEN_SET_LABELS[set_idx]), polls));
    }
    sets
}

/// One draw for the answer against the weights GJ-27
/// states, the favoured answer at weight five and
/// every other offered answer at weight two.
fn draw_answer(rng: &mut Lcg, offered: usize, favoured: usize) -> usize {
    let total: u64 = (2 * offered + 3) as u64;
    let roll = rng.draw(total) as usize;
    let mut acc = 0usize;
    for slot in 0..offered {
        let width = if slot == favoured { 5 } else { 2 };
        if roll < acc + width {
            return slot;
        }
        acc += width;
    }
    offered - 1
}

/// The generated sets loaded back through the crate's
/// own loader from files written in the poll form, so
/// the family stands as recorded polls and nothing else.
fn generated_sets_loaded() -> Vec<(PText, Vec<Poll>)> {
    let in_memory = generated_sets_in_memory();
    let mut loaded: Vec<(PText, Vec<Poll>)> = Vec::new();
    for (label, polls) in &in_memory {
        let tmp = TempDir::new("accord-generated");
        for (i, held) in polls.iter().enumerate() {
            tmp.write(
                &format!("poll-{:02}.json", i + 1),
                &dumps_default(&held.to_json()),
            );
        }
        let from_dir = load_set(&tmp.path, polls.len());
        loaded.push((label.clone(), from_dir));
    }
    loaded
}

#[test]
fn the_generated_family_holds_its_stated_figures() {
    // The identity figures of the family, stated in
    // GJ-37 as equalities at the seed and counted here
    // by walking the generated record response by
    // response. This pin carries the restatement duty
    // of the round: it passes on the generator GJ-27
    // states, and it fails by assertion against the
    // generator form the Round 13 reviews found, which
    // drew the asking count over one to four and the
    // model count over one to three, ran the stated
    // phases interleaved in one per question pass, and
    // let an abstained response consume neither the
    // answer draw nor the reason draw, so its record
    // holds different counts on every figure below and
    // polls holding one model string where this pin
    // requires exactly two.
    let in_memory = generated_sets_in_memory();
    let mut questions: Vec<String> = Vec::new();
    let mut polls = 0usize;
    let mut responses = 0usize;
    let mut abstained = 0usize;
    let mut valid = 0usize;
    let mut spoiled = 0usize;
    for (_, set_polls) in &in_memory {
        for held in set_polls {
            polls += 1;
            let text = held.question.display();
            if !questions.contains(&text) {
                questions.push(text);
            }
            let mut models: Vec<String> = Vec::new();
            for entry in &held.responses {
                responses += 1;
                let model = entry.model.display();
                if !models.contains(&model) {
                    models.push(model);
                }
                if entry.abstained {
                    abstained += 1;
                } else if held.answers.contains(&entry.answer) {
                    valid += 1;
                } else {
                    spoiled += 1;
                }
            }
            assert_eq!(
                models.len(),
                2,
                "every generated poll holds exactly two model strings at the seed"
            );
        }
    }
    assert_eq!(questions.len(), 36);
    assert_eq!(polls, 72, "the askings of the family");
    assert_eq!(responses, 720);
    assert_eq!(abstained, 285);
    assert_eq!(valid, 423);
    assert_eq!(spoiled, 12);
    // The shape figures of GJ-38 over the generated
    // record, each computed here from the direct
    // computation over the record and stated as an
    // equality at the seed: the closing of the reading
    // at 36 questions and 72 parts and 28 in accord
    // and 8 divided, twelve questions at one set,
    // seven divided questions whose sets name two
    // answers and one no set holds a plurality at,
    // seven questions in accord while one of their
    // sets holds no plurality, and a pooled plurality
    // standing at six of the divided questions. The
    // parts figure and the in accord figure carry no
    // relation to any standing reading, as GJ-38
    // states, and stand pinned here by this
    // computation alone.
    let lines = direct_accord_lines(&in_memory);
    let (sections, closing) = parse_accord(&lines);
    assert_eq!(
        closing, "accords, 36 questions, 72 parts, 28 in accord, 8 divided",
        "the closing of the generated accord at the seed"
    );
    let mut one_set = 0usize;
    let mut divided_disagreement = 0usize;
    let mut divided_no_plurality = 0usize;
    let mut accord_with_a_set_holding_none = 0usize;
    let mut divided_with_pooled_standing = 0usize;
    for (question, parts, accord) in &sections {
        let holders = sets_of(&in_memory, question);
        if holders.len() == 1 {
            one_set += 1;
        }
        // The pooled plurality of the question read
        // directly off every asking pooled together.
        let askings = askings_of(&in_memory, question);
        let mut union: Vec<String> = Vec::new();
        for held in &askings {
            for answer in &held.answers {
                let text = answer.display();
                if !union.contains(&text) {
                    union.push(text);
                }
            }
        }
        let mut pooled = vec![0usize; union.len()];
        let mut pooled_valid = 0usize;
        for held in &askings {
            for entry in &held.responses {
                if entry.abstained || !held.answers.contains(&entry.answer) {
                    continue;
                }
                pooled_valid += 1;
                let slot = union
                    .iter()
                    .position(|answer| answer == &entry.answer.display())
                    .expect("a union slot");
                pooled[slot] += 1;
            }
        }
        let pooled_plurality = direct_plurality(&pooled, pooled_valid, &union);
        if accord.as_str() == "none" {
            if parts.iter().any(|part| part.2.as_str() != "none") {
                divided_disagreement += 1;
            } else {
                divided_no_plurality += 1;
            }
            if pooled_plurality.is_some() {
                divided_with_pooled_standing += 1;
            }
        } else if parts.iter().any(|part| part.2.as_str() == "none") {
            accord_with_a_set_holding_none += 1;
        }
    }
    assert_eq!(one_set, 12, "the one set questions of the family");
    assert_eq!(divided_disagreement, 7, "the divided by disagreement");
    assert_eq!(divided_no_plurality, 1, "the divided with no plurality");
    assert_eq!(
        accord_with_a_set_holding_none, 7,
        "the accords standing while a set holds no plurality"
    );
    assert_eq!(
        divided_with_pooled_standing, 6,
        "the divided questions at a pooled plurality"
    );
}

#[test]
fn the_generated_family_recounts_independently() {
    // The recount of GJ-38 over the generated record.
    // The pin fails under the mutation summing a part
    // over every asking of the question in every set,
    // the mutation pin 2 names on the shipped record,
    // standing here on the family's parts, and under
    // the mutations naming the pooled plurality on the
    // accord line and standing the accord only where
    // every set holds a plurality, both moving the
    // generated closing's in accord figure off 28.
    let in_memory = generated_sets_in_memory();
    let loaded = generated_sets_loaded();
    let lines = accord_lines(&loaded).expect("the accord reading computes");
    assert_eq!(lines, direct_accord_lines(&in_memory));
    let closing = lines.last().expect("a closing line");
    assert_eq!(figure_before(closing, "questions"), 36);
    assert_eq!(figure_before(closing, "parts"), 72);
    assert_eq!(figure_before(closing, "in accord"), 28);
    assert_eq!(figure_before(closing, "divided"), 8);
}

#[test]
fn the_generated_family_closes_under_its_own_relations() {
    // The relations of GJ-38 over the generated
    // record: the closings reconcile against the
    // aggregate and the canvass and the margin over
    // the generated sets at the function, and the
    // accord of every qualifying question stands equal
    // to the pooled plurality of its aggregate
    // section, the qualifying questions counted here
    // directly before the relation runs. The pin
    // fails under the mutation computing the generated
    // closing from the shipped record's figures, whose
    // questions stand at 52 where the family's stand
    // at 36.
    let loaded = generated_sets_loaded();
    assert_closing_relations(&loaded);
    let qualifying = qualifying_questions(&loaded);
    assert_eq!(
        qualifying.len(),
        21,
        "the qualifying questions of the family"
    );
    assert_accord_pooled_relations(&loaded, &qualifying);
    assert_one_set_relations(&loaded);
}
