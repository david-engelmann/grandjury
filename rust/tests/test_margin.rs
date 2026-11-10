//! Pins for the margin reading and command.
//!
//! The margin pins are stated in GJ-35 and GJ-36 in
//! CONTRACT.md, in the Round 17 section. One section
//! stands per question in the order the questions
//! first appear across the sets: the question line,
//! then the pooled part of the section, the counts
//! line of the whole question under the word pool
//! with the plurality line and the margin line of the
//! pooled plurality, then one block per jury of the
//! question in the order the model strings first
//! appear across its askings, the jury line carrying
//! the counts of the jury summed over the askings of
//! the question with its own plurality line and
//! margin line. The margin of a pool stands as the
//! count of its plurality answer minus the largest
//! count among the other offered answers, read over
//! the summed counts of the pool, and stands only
//! where a plurality stands for the pool, else the
//! words not counted. The closing line counts the
//! questions, the blocks, the blocks at which a
//! plurality stands, and the blocks whose plurality
//! stands at a margin of one, and the relations of
//! GJ-36 bind the reading to the dissent reading and
//! the aggregate reading and the closings of the pool
//! and the canvass and the aggregate, the figure at
//! one carrying no relation to any standing reading
//! and standing pinned by independent computation
//! alone. The seventeenth command prints the reading
//! over label and directory pairs in the census
//! command's forms.
//!
//! Discrimination, pin by pin. Pins 1 to 22 in this
//! file fail against the Round 16 closure tree by
//! compile absence, the grandjury::margin module and
//! the margin command and this target and the golden
//! standing nowhere in that tree. Beyond that
//! absence, pin 1 pins the golden byte for byte and
//! fails under any move of a form word or an order or
//! a figure, the blocks of a section standing before
//! its pooled part among the moves it catches, pin 2
//! holds the whole reading against an independent
//! computation over the recorded polls and fails
//! under the mutation computing a block's margin from
//! the margins of its askings summed, or from its
//! first asking alone, pin 3 holds the blocks of the
//! one asking questions equal in plurality and
//! margin to the dissent parts of their askings and
//! fails under the mutation building a one asking
//! question's blocks from its pooled reading, pin 4
//! holds the pooled part of every section equal to
//! the aggregate section of its question and fails
//! under the mutation reading the pooled margin off
//! the plurality count alone, the largest count
//! among the other answers taken as zero, pin 5
//! reconciles the closing with the closings of the
//! pool and the aggregate and the canvass and fails
//! under the mutation counting the closing's
//! pluralities per asking instead of per block, pin
//! 6 states the third relation on a hand built
//! record, one jury at a question of two askings,
//! its block margin equal to the pooled margin of
//! its section, and fails under the mutation stating
//! the pooled margin from the pooled counts of the
//! whole record, pins 7 to 14 state the hand built
//! shapes of GJ-36, each with its own mutation named
//! at the pin, pins 15 and 16 exercise the guard on
//! the model of the second set and the question of
//! the first, pin 17 exercises the set rules, pin 18
//! holds the reading under the vocabulary scan and
//! passes by design on the words margin and margins,
//! checked word by word against the fifteen refused
//! words, failing under the mutation renaming any
//! form word of the reading to a listed word, pins
//! 19 to 21 hold the generated family to its stated
//! figures, its recount, and its relations, and pin
//! 22 names the command in the help of the binary
//! and fails at the base by assertion there, the
//! margin standing as no command in the standing
//! binary, so its help run draws the unknown command
//! refusal at status two where the pin requires
//! status zero. The three command pins of the round
//! stand in test_cli.rs and fail there by assertion
//! against the surface's unknown command refusal.
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

use grandjury::aggregate::aggregate_lines;
use grandjury::canvass::canvass_lines;
use grandjury::dissent::dissent_lines;
use grandjury::margin::margin_lines;
use grandjury::polls::{load_poll_file, Poll, Response};
use grandjury::pool::pool_lines;
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

/// One row line read as (name, count, fraction,
/// percent), the name holding any commas of its own.
fn row_of(line: &str) -> (String, usize, String, String) {
    let body = line.strip_prefix("  ").expect("a row line");
    let mut parts = body.rsplitn(4, ", ");
    let percent = parts.next().expect("a percent").to_string();
    let fraction = parts.next().expect("a fraction").to_string();
    let count: usize = parts.next().expect("a count").parse().expect("a row count");
    let name = parts.next().expect("a name").to_string();
    (name, count, fraction, percent)
}

type Counts4 = (usize, usize, usize, usize);

fn counts_text(counts: Counts4) -> String {
    format!(
        "recorded {}, valid {}, abstained {}, spoiled {}",
        counts.0, counts.1, counts.2, counts.3
    )
}

/// One block of a margin reading as (model, counts,
/// plurality value, margin value), the plurality
/// value standing as the answer's text or the word
/// none and the margin value as a figure or the
/// words not counted.
type MarginBlock = (String, Counts4, String, String);

/// One section of a margin reading as (question,
/// pooled counts, pooled plurality value, pooled
/// margin value, blocks).
type MarginSection = (String, Counts4, String, String, Vec<MarginBlock>);

/// The sections of a margin reading and its closing
/// line, parsed from rendered lines, so the relation
/// pins read the margin as text.
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

/// The parts of a dissent reading as (question,
/// model, plurality value, margin value) tuples in
/// reading order, parsed from rendered lines, the
/// set label of each part standing between the form
/// word and the value it qualifies.
fn parse_dissent_parts(lines: &[String]) -> Vec<(String, String, String, String)> {
    let mut parts: Vec<(String, String, String, String)> = Vec::new();
    let mut question = String::new();
    let mut model = String::new();
    let mut plurality = String::new();
    for line in lines {
        if let Some(text) = line.strip_prefix("question, ") {
            question = text.to_string();
        } else if let Some(rest) = line.strip_prefix("jury, ") {
            let split = rest.find(", recorded ").expect("a counts tail");
            model = rest[..split].to_string();
        } else if line.starts_with("plurality, ") {
            plurality = line
                .splitn(3, ", ")
                .nth(2)
                .expect("a plurality value")
                .to_string();
        } else if line.starts_with("margin, ") {
            let margin = line
                .splitn(3, ", ")
                .nth(2)
                .expect("a margin value")
                .to_string();
            parts.push((question.clone(), model.clone(), plurality.clone(), margin));
        }
    }
    parts
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

/// Count one jury at one asking directly from the
/// responses, under the validity rule of GJ-8 read
/// response by response, returning the four counts
/// and the per answer counts over the union of the
/// question, the counting this file's direct
/// computation and shape counts share.
fn direct_jury_counts(poll: &Poll, model: &str, union: &[String]) -> (Counts4, Vec<usize>) {
    let mut counts = vec![0usize; union.len()];
    let (mut recorded, mut valid, mut abstained, mut spoiled) = (0, 0, 0, 0);
    for held in &poll.responses {
        if held.model.display() != model {
            continue;
        }
        recorded += 1;
        if held.abstained {
            abstained += 1;
        } else if !poll.answers.contains(&held.answer) {
            spoiled += 1;
        } else {
            valid += 1;
            let slot = union
                .iter()
                .position(|answer| answer == &held.answer.display())
                .expect("a union slot");
            counts[slot] += 1;
        }
    }
    ((recorded, valid, abstained, spoiled), counts)
}

/// The block of one jury at one question computed
/// directly, as (summed four counts, summed per
/// answer counts, plurality), the counts of the jury
/// summed over the askings of the question response
/// by response in the test.
fn direct_block(
    sets: &[(PText, Vec<Poll>)],
    question: &str,
    model: &str,
    union: &[String],
) -> (Counts4, Vec<usize>, Option<String>) {
    let mut totals: Counts4 = (0, 0, 0, 0);
    let mut summed = vec![0usize; union.len()];
    for held in askings_of(sets, question) {
        let (counts, per_answer) = direct_jury_counts(held, model, union);
        totals.0 += counts.0;
        totals.1 += counts.1;
        totals.2 += counts.2;
        totals.3 += counts.3;
        for (slot, count) in per_answer.iter().enumerate() {
            summed[slot] += count;
        }
    }
    let plurality = direct_plurality(&summed, totals.1, union);
    (totals, summed, plurality)
}

/// The gathering of one question computed directly,
/// as (union, models) in the orders GJ-36 states.
fn direct_gathering(sets: &[(PText, Vec<Poll>)], question: &str) -> (Vec<String>, Vec<String>) {
    let askings = askings_of(sets, question);
    let mut union: Vec<String> = Vec::new();
    for held in &askings {
        for answer in &held.answers {
            let text = answer.display();
            if !union.contains(&text) {
                union.push(text);
            }
        }
    }
    let mut models: Vec<String> = Vec::new();
    for held in &askings {
        for entry in &held.responses {
            let model = entry.model.display();
            if !models.contains(&model) {
                models.push(model);
            }
        }
    }
    (union, models)
}

/// The margin reading computed directly from the
/// loaded polls with this file's own arithmetic, each
/// jury's responses at each asking of a question
/// counted response by response under the validity
/// rule of GJ-8 and summed in the test, the pool of a
/// question counted over every response of its
/// askings the same way, so the pin holds no crate
/// counting.
fn direct_margin_lines(sets: &[(PText, Vec<Poll>)]) -> Vec<String> {
    let mut questions: Vec<String> = Vec::new();
    for (_, polls) in sets {
        for held in polls {
            let text = held.question.display();
            if !questions.contains(&text) {
                questions.push(text);
            }
        }
    }
    let mut lines = Vec::new();
    let mut block_total = 0usize;
    let mut plurality_total = 0usize;
    let mut at_one_total = 0usize;
    for question in &questions {
        let (union, models) = direct_gathering(sets, question);
        let mut pool_totals: Counts4 = (0, 0, 0, 0);
        let mut pooled = vec![0usize; union.len()];
        for held in askings_of(sets, question) {
            for entry in &held.responses {
                pool_totals.0 += 1;
                if entry.abstained {
                    pool_totals.2 += 1;
                } else if !held.answers.contains(&entry.answer) {
                    pool_totals.3 += 1;
                } else {
                    pool_totals.1 += 1;
                    let slot = union
                        .iter()
                        .position(|answer| answer == &entry.answer.display())
                        .expect("a union slot");
                    pooled[slot] += 1;
                }
            }
        }
        let pool_plurality = direct_plurality(&pooled, pool_totals.1, &union);
        let pool_margin = direct_margin(&pooled, pool_totals.1, &union);
        lines.push(format!("question, {question}"));
        lines.push(format!("pool, {}", counts_text(pool_totals)));
        lines.push(format!(
            "plurality, {}",
            pool_plurality.as_deref().unwrap_or("none")
        ));
        lines.push(match pool_margin {
            Some(figure) => format!("margin, {figure}"),
            None => "margin, not counted".to_string(),
        });
        for model in &models {
            let (totals, summed, plurality) = direct_block(sets, question, model, &union);
            let margin = direct_margin(&summed, totals.1, &union);
            block_total += 1;
            if plurality.is_some() {
                plurality_total += 1;
                if margin == Some(1) {
                    at_one_total += 1;
                }
            }
            lines.push(format!("jury, {model}, {}", counts_text(totals)));
            lines.push(format!(
                "plurality, {}",
                plurality.as_deref().unwrap_or("none")
            ));
            lines.push(match margin {
                Some(figure) => format!("margin, {figure}"),
                None => "margin, not counted".to_string(),
            });
        }
    }
    lines.push(format!(
        "margins, {} questions, {} blocks, {} pluralities, {} at one",
        questions.len(),
        block_total,
        plurality_total,
        at_one_total
    ));
    lines
}

/// The first relation of GJ-36, checked over any
/// sets: at a question holding exactly one asking,
/// the plurality and the margin of every block stand
/// equal to the plurality and the margin of the part
/// of that jury at that asking in the dissent
/// reading.
fn assert_dissent_relations(sets: &[(PText, Vec<Poll>)]) {
    let lines = margin_lines(sets).expect("the margin reading computes");
    let (sections, _) = parse_margin(&lines);
    let dissent = dissent_lines(sets).expect("the dissent reading computes");
    let parts = parse_dissent_parts(&dissent);
    for (question, _, _, _, blocks) in &sections {
        if askings_of(sets, question).len() != 1 {
            continue;
        }
        for (model, _, plurality, margin) in blocks {
            let part = parts
                .iter()
                .find(|(held_question, held_model, _, _)| {
                    held_question == question && held_model == model
                })
                .expect("a dissent part for the block");
            assert_eq!(plurality, &part.2, "the plurality of {model} at {question}");
            assert_eq!(margin, &part.3, "the margin of {model} at {question}");
        }
    }
}

/// The second relation of GJ-36, checked over any
/// sets: the plurality line of the pooled part of a
/// section stands equal to the plurality line of the
/// same question's section in the aggregate reading,
/// and the margin of the pooled part stands equal to
/// the margin figure of that section's rows, the
/// count of its plurality answer minus the largest
/// count among its other answers read off the rows.
fn assert_pooled_relations(sets: &[(PText, Vec<Poll>)]) {
    let lines = margin_lines(sets).expect("the margin reading computes");
    let (sections, _) = parse_margin(&lines);
    let aggregate = aggregate_lines(sets).expect("the aggregate reading computes");
    let (held_sections, _) = parse_aggregate_full(&aggregate);
    assert_eq!(sections.len(), held_sections.len());
    for ((question, _, plurality, margin, _), (held_question, rows, held_line)) in
        sections.iter().zip(held_sections.iter())
    {
        assert_eq!(question, held_question);
        assert_eq!(&format!("plurality, {plurality}"), held_line);
        if plurality.as_str() == "none" {
            assert_eq!(margin, "not counted");
            continue;
        }
        let counts: Vec<usize> = rows.iter().map(|line| row_of(line).1).collect();
        let slot = rows
            .iter()
            .position(|line| row_of(line).0 == *plurality)
            .expect("the plurality row");
        let largest_other = counts
            .iter()
            .enumerate()
            .filter(|(held, _)| *held != slot)
            .map(|(_, count)| *count)
            .max()
            .unwrap_or(0);
        assert_eq!(margin, &(counts[slot] - largest_other).to_string());
    }
}

/// The closing relations of GJ-36, checked over any
/// sets: the questions figure of the closing equals
/// the questions figures of the closings of the pool
/// and the aggregate and the canvass over the same
/// sets, the blocks figure equals the juries figure
/// of the canvass closing, the pluralities figure
/// equals the pluralities figure of that closing,
/// and the count of the sections at which a pooled
/// plurality stands equals the pluralities figure of
/// the aggregate closing.
fn assert_closing_relations(sets: &[(PText, Vec<Poll>)]) {
    let lines = margin_lines(sets).expect("the margin reading computes");
    let (sections, closing) = parse_margin(&lines);
    let pool = pool_lines(sets).expect("the pool reading computes");
    let pool_closing = pool.last().expect("a pool closing");
    let aggregate = aggregate_lines(sets).expect("the aggregate reading computes");
    let (_, aggregate_closing) = parse_aggregate_full(&aggregate);
    let canvass = canvass_lines(sets).expect("the canvass reading computes");
    let canvass_closing = canvass.last().expect("a canvass closing");
    assert_eq!(
        figure_before(&closing, "questions"),
        figure_before(pool_closing, "questions")
    );
    assert_eq!(
        figure_before(&closing, "questions"),
        figure_before(&aggregate_closing, "questions")
    );
    assert_eq!(
        figure_before(&closing, "questions"),
        figure_before(canvass_closing, "questions")
    );
    assert_eq!(
        figure_before(&closing, "blocks"),
        figure_before(canvass_closing, "juries")
    );
    assert_eq!(
        figure_before(&closing, "pluralities"),
        figure_before(canvass_closing, "pluralities")
    );
    let pooled_standing = sections
        .iter()
        .filter(|(_, _, plurality, _, _)| plurality.as_str() != "none")
        .count();
    assert_eq!(
        pooled_standing,
        figure_before(&aggregate_closing, "pluralities")
    );
}

#[test]
fn the_margin_golden_stands_byte_for_byte() {
    let golden = read_text(&examples().join("margin-golden.txt"));
    let (stdout, _, code) = run_cli(&[
        "margin",
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
fn the_margin_reading_computes_directly_from_the_record() {
    let sets = recorded_sets();
    let expected = direct_margin_lines(&sets);
    assert_eq!(
        margin_lines(&sets).expect("the margin reading computes"),
        expected
    );
    let (stdout, _, code) = run_cli(&[
        "margin",
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
fn the_margin_blocks_at_one_asking_questions_stand_equal_to_the_dissent_parts() {
    // The first relation of GJ-36 over the recorded
    // sets at the function, quantifying over the
    // shipped questions at one asking, counted here
    // as an equality before the relation runs so a
    // record change that moved a question across the
    // one asking line would fail this pin rather than
    // empty it.
    let sets = recorded_sets();
    let lines = margin_lines(&sets).expect("the margin reading computes");
    let (sections, _) = parse_margin(&lines);
    let single = sections
        .iter()
        .filter(|(question, _, _, _, _)| askings_of(&sets, question).len() == 1)
        .count();
    assert_eq!(single, 40, "the one asking questions of the record");
    assert_dissent_relations(&sets);
}

#[test]
fn the_margin_pooled_parts_stand_equal_to_the_aggregate_sections() {
    assert_pooled_relations(&recorded_sets());
}

#[test]
fn the_margin_closing_reconciles_with_the_pool_and_canvass_and_aggregate_closings() {
    assert_closing_relations(&recorded_sets());
}

#[test]
fn the_block_margin_of_a_one_jury_question_equals_its_pooled_margin() {
    // The third relation of GJ-36 on a hand built
    // record. One jury stands at the first question
    // of the record, a question of two askings, three
    // for the low meadow and one for the high meadow
    // at the first asking and one for the low meadow
    // and two for the high meadow at the second, so
    // the pool of the question and the summed block
    // of its one jury are the same responses, four
    // and three, and the margin of the block stands
    // equal to the margin of the pooled part, one in
    // both standings. The record holds a second
    // question, five for the north gate from the same
    // jury, so the pin fails under the mutation
    // stating the pooled margin from the pooled
    // counts of the whole record, which would carry
    // the gate question's five into the meadow
    // section's margin.
    let question = "Which meadow should host the fair?";
    let answers = &["the low meadow", "the high meadow"];
    let mut first = votes_of("the low meadow", 3, "model a");
    first.extend(votes_of("the high meadow", 1, "model a"));
    let mut second = votes_of("the low meadow", 1, "model a");
    second.extend(votes_of("the high meadow", 2, "model a"));
    let sets = vec![
        (
            pt("alpha"),
            vec![
                poll(question, answers, first),
                poll(
                    "Which gate should the night watch use?",
                    &["the north gate", "the south gate"],
                    votes_of("the north gate", 5, "model a"),
                ),
            ],
        ),
        (pt("beta"), vec![poll(question, answers, second)]),
    ];
    let lines = margin_lines(&sets).expect("the margin reading computes");
    assert_eq!(
        lines,
        vec![
            "question, Which meadow should host the fair?",
            "pool, recorded 7, valid 7, abstained 0, spoiled 0",
            "plurality, the low meadow",
            "margin, 1",
            "jury, model a, recorded 7, valid 7, abstained 0, spoiled 0",
            "plurality, the low meadow",
            "margin, 1",
            "question, Which gate should the night watch use?",
            "pool, recorded 5, valid 5, abstained 0, spoiled 0",
            "plurality, the north gate",
            "margin, 5",
            "jury, model a, recorded 5, valid 5, abstained 0, spoiled 0",
            "plurality, the north gate",
            "margin, 5",
            "margins, 2 questions, 2 blocks, 2 pluralities, 1 at one",
        ]
    );
    let (sections, _) = parse_margin(&lines);
    assert_eq!(sections[0].3, sections[0].4[0].3);
}

#[test]
fn a_jury_whose_summed_top_counts_tie_states_no_margin() {
    // The summed tie shape of GJ-36. The one jury
    // holds a plurality at each asking of the
    // question, two for run it and one for rest the
    // crew at the first and one for run it and two
    // for rest the crew at the second, margins of one
    // standing at both askings, but the summed top
    // counts tie at three and three, so the block
    // states the word none and not counted, and the
    // pooled part, the same responses, states the
    // same. The pin fails under the mutation letting
    // a margin stand where a plurality falls, the tie
    // stating a margin of zero as a counted figure.
    let question = "Should the ferry run on Christmas Day?";
    let answers = &["run it", "rest the crew"];
    let mut first = votes_of("run it", 2, "model a");
    first.extend(votes_of("rest the crew", 1, "model a"));
    let mut second = votes_of("run it", 1, "model a");
    second.extend(votes_of("rest the crew", 2, "model a"));
    let sets = vec![
        (pt("alpha"), vec![poll(question, answers, first)]),
        (pt("beta"), vec![poll(question, answers, second)]),
    ];
    assert_eq!(
        margin_lines(&sets).expect("the margin reading computes"),
        vec![
            "question, Should the ferry run on Christmas Day?",
            "pool, recorded 6, valid 6, abstained 0, spoiled 0",
            "plurality, none",
            "margin, not counted",
            "jury, model a, recorded 6, valid 6, abstained 0, spoiled 0",
            "plurality, none",
            "margin, not counted",
            "margins, 1 questions, 1 blocks, 0 pluralities, 0 at one",
        ]
    );
}

#[test]
fn a_unanimous_block_states_its_whole_valid_count() {
    // The unanimous shape of GJ-36. The first jury's
    // four valid responses at the question all chose
    // the north gate, so the largest count among the
    // other answers of its block stands at zero and
    // the margin of the block equals its whole valid
    // count, four. The second jury names one for the
    // north gate and three for the south gate, so the
    // counts of the whole question stand at five and
    // three and the pooled margin at two, and the pin
    // fails under the mutation taking the largest
    // other count over the whole question's counts in
    // place of the block's, which would state two
    // for the first block where the pin requires
    // four.
    let question = "Which gate should the night watch use?";
    let answers = &["the north gate", "the south gate"];
    let mut responses = votes_of("the north gate", 4, "model a");
    responses.extend(votes_of("the north gate", 1, "model b"));
    responses.extend(votes_of("the south gate", 3, "model b"));
    let sets = vec![(pt("alpha"), vec![poll(question, answers, responses)])];
    assert_eq!(
        margin_lines(&sets).expect("the margin reading computes"),
        vec![
            "question, Which gate should the night watch use?",
            "pool, recorded 8, valid 8, abstained 0, spoiled 0",
            "plurality, the north gate",
            "margin, 2",
            "jury, model a, recorded 4, valid 4, abstained 0, spoiled 0",
            "plurality, the north gate",
            "margin, 4",
            "jury, model b, recorded 4, valid 4, abstained 0, spoiled 0",
            "plurality, the south gate",
            "margin, 2",
            "margins, 1 questions, 2 blocks, 2 pluralities, 0 at one",
        ]
    );
}

#[test]
fn a_jury_with_no_valid_response_states_no_margin() {
    // The empty jury shape of GJ-36. The one jury
    // abstains five times across two askings, so it
    // stands at the question with no valid response,
    // and its block states the word none and not
    // counted, the margin standing uncomputed rather
    // than divided by a valid count of zero. The pin
    // fails under the mutation omitting a zero valid
    // jury from its section, or rendering its margin
    // by dividing by its valid count.
    let question = "Should the ferry run on Christmas Day?";
    let answers = &["run it", "rest the crew"];
    let first = vec![
        response("model a hand 1", "model a", None, "unused"),
        response("model a hand 2", "model a", None, "unused"),
    ];
    let second = vec![
        response("model a hand 3", "model a", None, "unused"),
        response("model a hand 4", "model a", None, "unused"),
        response("model a hand 5", "model a", None, "unused"),
    ];
    let sets = vec![
        (pt("alpha"), vec![poll(question, answers, first)]),
        (pt("beta"), vec![poll(question, answers, second)]),
    ];
    assert_eq!(
        margin_lines(&sets).expect("the margin reading computes"),
        vec![
            "question, Should the ferry run on Christmas Day?",
            "pool, recorded 5, valid 0, abstained 5, spoiled 0",
            "plurality, none",
            "margin, not counted",
            "jury, model a, recorded 5, valid 0, abstained 5, spoiled 0",
            "plurality, none",
            "margin, not counted",
            "margins, 1 questions, 1 blocks, 0 pluralities, 0 at one",
        ]
    );
}

#[test]
fn a_question_whose_pooled_counts_tie_states_no_pooled_margin() {
    // The pooled tie shape of GJ-36. The first jury
    // names two for the red gate and one for the blue
    // gate, so its block plurality is the red gate at
    // a margin of one. The second names two for the
    // blue gate and one for the red gate, so its
    // block plurality is the blue gate at a margin of
    // one. The pooled counts tie at three and three,
    // so the pooled part states not counted while the
    // margins of the blocks stand, and both blocks
    // stand counted at one. The pin fails under the
    // mutation reading a section's pooled plurality
    // from its first asking alone.
    let question = "Which gate should the night watch use?";
    let answers = &["the red gate", "the blue gate"];
    let mut responses = votes_of("the red gate", 2, "model a");
    responses.extend(votes_of("the blue gate", 1, "model a"));
    responses.extend(votes_of("the blue gate", 2, "model b"));
    responses.extend(votes_of("the red gate", 1, "model b"));
    let sets = vec![(pt("alpha"), vec![poll(question, answers, responses)])];
    assert_eq!(
        margin_lines(&sets).expect("the margin reading computes"),
        vec![
            "question, Which gate should the night watch use?",
            "pool, recorded 6, valid 6, abstained 0, spoiled 0",
            "plurality, none",
            "margin, not counted",
            "jury, model a, recorded 3, valid 3, abstained 0, spoiled 0",
            "plurality, the red gate",
            "margin, 1",
            "jury, model b, recorded 3, valid 3, abstained 0, spoiled 0",
            "plurality, the blue gate",
            "margin, 1",
            "margins, 1 questions, 2 blocks, 2 pluralities, 2 at one",
        ]
    );
}

#[test]
fn a_block_at_a_margin_of_one_stands_counted_at_one() {
    // The margin of one shape of GJ-36. The one jury
    // names three for the north gate and two for the
    // south gate, so its block plurality stands at a
    // margin of one, the closest a plurality can
    // stand, and the closing of the reading, asserted
    // whole here, counts that one block at one. The
    // pin passes under the mutation counting pooled
    // margins at one in place of block margins, the
    // pooled margin of this record standing at one as
    // well, so the shape stands beside the pooled tie
    // shape, whose pooled part counts nothing while
    // its two blocks stand at one, and the
    // discrimination against that mutation stands
    // jointly with that shape, whose fixture
    // separates the pooled figure from the block
    // figure.
    let question = "Which gate should the night watch use?";
    let answers = &["the north gate", "the south gate"];
    let mut responses = votes_of("the north gate", 3, "model a");
    responses.extend(votes_of("the south gate", 2, "model a"));
    let sets = vec![(pt("alpha"), vec![poll(question, answers, responses)])];
    assert_eq!(
        margin_lines(&sets).expect("the margin reading computes"),
        vec![
            "question, Which gate should the night watch use?",
            "pool, recorded 5, valid 5, abstained 0, spoiled 0",
            "plurality, the north gate",
            "margin, 1",
            "jury, model a, recorded 5, valid 5, abstained 0, spoiled 0",
            "plurality, the north gate",
            "margin, 1",
            "margins, 1 questions, 1 blocks, 1 pluralities, 1 at one",
        ]
    );
}

#[test]
fn an_answer_offered_only_at_the_second_asking_sets_the_block_margin() {
    // The second asking runner up shape of GJ-36.
    // The question offers two answers at its first
    // asking and three at its second, the east field
    // standing offered only at the second. The one
    // jury answers three for the north field at the
    // first asking and two for the east field at the
    // second, so the block's plurality is the north
    // field at three and the largest count against it
    // is the east field's two, a count no asking but
    // the second holds, and the margin of the block
    // stands at one, set from that asking alone. The
    // pin fails under the mutation gathering the
    // union from the first asking alone, which would
    // state a margin of three.
    let question = "Which field should host the maypole?";
    let first = votes_of("the north field", 3, "model a");
    let second = votes_of("the east field", 2, "model a");
    let sets = vec![
        (
            pt("alpha"),
            vec![poll(
                question,
                &["the north field", "the south field"],
                first,
            )],
        ),
        (
            pt("beta"),
            vec![poll(
                question,
                &["the north field", "the south field", "the east field"],
                second,
            )],
        ),
    ];
    assert_eq!(
        margin_lines(&sets).expect("the margin reading computes"),
        vec![
            "question, Which field should host the maypole?",
            "pool, recorded 5, valid 5, abstained 0, spoiled 0",
            "plurality, the north field",
            "margin, 1",
            "jury, model a, recorded 5, valid 5, abstained 0, spoiled 0",
            "plurality, the north field",
            "margin, 1",
            "margins, 1 questions, 1 blocks, 1 pluralities, 1 at one",
        ]
    );
}

#[test]
fn a_jury_at_one_asking_of_a_question_states_that_askings_margin() {
    // The partial jury shape of GJ-36. The question
    // stands at two askings. The first jury, model a,
    // stands at the first asking alone, two for the
    // low meadow and one for the high meadow, so its
    // block is that asking's reading alone and its
    // margin is that asking's margin, one. The second
    // jury, model b, stands at both askings, one and
    // two at the first and two for the low meadow at
    // the second, pooling to three and two and a
    // margin of one. The pin fails under the mutation
    // summing a jury over the asking it does not
    // stand at out of that asking's pooled reading,
    // which would carry model b's second asking into
    // model a's block.
    let question = "Which meadow should host the fair?";
    let answers = &["the low meadow", "the high meadow"];
    let mut first = votes_of("the low meadow", 2, "model a");
    first.extend(votes_of("the high meadow", 1, "model a"));
    first.extend(votes_of("the low meadow", 1, "model b"));
    first.extend(votes_of("the high meadow", 2, "model b"));
    let second = votes_of("the low meadow", 2, "model b");
    let sets = vec![
        (pt("alpha"), vec![poll(question, answers, first)]),
        (pt("beta"), vec![poll(question, answers, second)]),
    ];
    assert_eq!(
        margin_lines(&sets).expect("the margin reading computes"),
        vec![
            "question, Which meadow should host the fair?",
            "pool, recorded 8, valid 8, abstained 0, spoiled 0",
            "plurality, the low meadow",
            "margin, 2",
            "jury, model a, recorded 3, valid 3, abstained 0, spoiled 0",
            "plurality, the low meadow",
            "margin, 1",
            "jury, model b, recorded 5, valid 5, abstained 0, spoiled 0",
            "plurality, the low meadow",
            "margin, 1",
            "margins, 1 questions, 2 blocks, 2 pluralities, 2 at one",
        ]
    );
}

#[test]
fn a_block_whose_plurality_differs_from_the_pooled_states_its_own_margin() {
    // The block against its question shape of GJ-36.
    // The first jury names three for the red gate and
    // one for the blue gate, so its block plurality
    // is the red gate at a margin of two. The second
    // names four for the blue gate and one for the
    // red gate, so the pooled plurality of the
    // question is the blue gate at a margin of one,
    // and the first block states its own margin, two,
    // read off its own summed counts, not the pooled
    // margin of its question and not the pooled
    // plurality. The pin fails under the mutation
    // stating a block's margin from its question's
    // pooled margin where they differ.
    let question = "Which gate should the night watch use?";
    let answers = &["the red gate", "the blue gate"];
    let mut responses = votes_of("the red gate", 3, "model a");
    responses.extend(votes_of("the blue gate", 1, "model a"));
    responses.extend(votes_of("the blue gate", 4, "model b"));
    responses.extend(votes_of("the red gate", 1, "model b"));
    let sets = vec![(pt("alpha"), vec![poll(question, answers, responses)])];
    assert_eq!(
        margin_lines(&sets).expect("the margin reading computes"),
        vec![
            "question, Which gate should the night watch use?",
            "pool, recorded 9, valid 9, abstained 0, spoiled 0",
            "plurality, the blue gate",
            "margin, 1",
            "jury, model a, recorded 4, valid 4, abstained 0, spoiled 0",
            "plurality, the red gate",
            "margin, 2",
            "jury, model b, recorded 5, valid 5, abstained 0, spoiled 0",
            "plurality, the blue gate",
            "margin, 3",
            "margins, 1 questions, 2 blocks, 2 pluralities, 0 at one",
        ]
    );
}

#[test]
fn the_margin_guard_reads_the_model_of_the_second_set() {
    // The guard exercise of GJ-35. The margin prints
    // the model strings of the record on the jury
    // lines of its blocks, and the guard as FIX-GJ-18
    // left it reads the model of every response
    // before any set rule is read, so a handed poll
    // in the second set whose one response carries a
    // model string holding a closing line after a
    // break draws the guard's model defect at the
    // margin, with no widening of the guard standing
    // in this round. The pin fails under the mutation
    // reading the guard lazily as the reading reaches
    // the sets, which would state the reading of the
    // first set before the refusal.
    let handed = Response {
        juror: pt("beta hand 1"),
        model: pt("model a\nmargins, 9 questions, 9 blocks"),
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
        margin_lines(&sets),
        Err("response 0 field model holds a newline".to_string())
    );
}

#[test]
fn the_margin_guard_reads_the_question_of_the_first_set() {
    // The second guard exercise of GJ-35. The margin
    // prints the question strings of the record on
    // its question lines, and the guard reads the
    // question of a poll before its answers and its
    // responses, so a handed poll in the first set
    // whose question string carries a closing line
    // after a break draws the guard's question defect
    // at the margin, before the clean set behind it
    // is read. The pin fails under the same laziness
    // as its fellow, on its own field.
    let sets = vec![
        (
            pt("alpha"),
            vec![poll(
                "Should the ferry run on Christmas Day?\nmargins, 9 questions, 9 blocks",
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
        margin_lines(&sets),
        Err("field question holds a newline".to_string())
    );
}

#[test]
fn the_margin_set_rules_refuse_as_the_census_refuses() {
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
    let duplicated_label = vec![(pt("alpha"), first.clone()), (pt("alpha"), second)];
    assert_eq!(
        margin_lines(&duplicated_label),
        Err("the set label alpha stands twice".to_string())
    );
    let question = "Should the ferry run on Christmas Day?";
    let doubled = vec![(pt("alpha"), vec![first[0].clone(), first[0].clone()])];
    assert_eq!(
        margin_lines(&doubled),
        Err(format!("the set alpha holds the question {question} twice"))
    );
}

#[test]
fn no_verdict_word_stands_in_the_margin_reading() {
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
    let golden = read_text(&examples().join("margin-golden.txt"));
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
        "margin",
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
    // earlier rounds: no line of the reading names a
    // set, the closing word of the reading is
    // margins, and the words margin and margins stand
    // outside the verdict list word by word, the
    // closing word standing in the golden so the
    // check is not vacuous.
    assert!(!VERDICT_WORDS.contains(&"margin"));
    assert!(!VERDICT_WORDS.contains(&"margins"));
    assert!(golden_words.contains(&"margins".to_string()));
    for text in [&golden, &out] {
        for line in text.lines() {
            assert!(
                !line.starts_with("set, "),
                "no line of the reading names a set: {line}"
            );
        }
        let closing = text.lines().last().expect("a closing line");
        assert!(
            closing.starts_with("margins, "),
            "the closing word of the reading is margins: {closing}"
        );
    }
}

#[test]
fn the_margin_help_names_the_command() {
    let (stdout, _, code) = run_cli(&["margin", "--help"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("usage: grandjury margin [-h] label directory [label directory ...]"));
    let (top, _, top_code) = run_cli(&["--help"]);
    assert_eq!(top_code, 0);
    assert!(top.contains("print the margin reading over recorded poll sets"));
}

// The generated fixture family of GJ-27, restated in
// this target as GJ-35 states: polls drawn in the
// test from the stated distribution at the stated
// seed, loaded through the crate's own loader, and
// pinned by the stated figures of GJ-36, by an
// independent recount inside the test, and by the
// relations of GJ-36. No golden ships for the family,
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
        let tmp = TempDir::new("margin-generated");
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
    // GJ-35 as equalities at the seed and counted here
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
    // The shape figures of GJ-36 over the generated
    // record, each computed here from the direct
    // computation over the record and stated as an
    // equality at the seed: the closing of the
    // reading at 36 questions and 91 blocks and 73
    // pluralities and 30 at one, twelve questions at
    // one asking, the pooled plurality standing at
    // 34 of the sections, and the pooled margin
    // standing at one at 6 of them. The figure at
    // one of the closing carries no relation to any
    // standing reading, as GJ-36 states, and stands
    // pinned here by this computation alone.
    let lines = direct_margin_lines(&in_memory);
    let (sections, closing) = parse_margin(&lines);
    assert_eq!(
        closing, "margins, 36 questions, 91 blocks, 73 pluralities, 30 at one",
        "the closing of the generated margin at the seed"
    );
    let mut one_asking = 0usize;
    let mut pooled_standing = 0usize;
    let mut pooled_at_one = 0usize;
    for (question, _, plurality, margin, _) in &sections {
        if askings_of(&in_memory, question).len() == 1 {
            one_asking += 1;
        }
        if plurality.as_str() != "none" {
            pooled_standing += 1;
        }
        if margin.as_str() == "1" {
            pooled_at_one += 1;
        }
    }
    assert_eq!(one_asking, 12, "the one asking questions of the family");
    assert_eq!(pooled_standing, 34, "the sections at a pooled plurality");
    assert_eq!(pooled_at_one, 6, "the sections at a pooled margin of one");
}

#[test]
fn the_generated_family_recounts_independently() {
    // The recount of GJ-36 over the generated record.
    // The pin fails under the mutation computing a
    // block's margin from the margins of its askings
    // summed, the mutation pin 2 names on the shipped
    // record, standing here on the family's blocks.
    let in_memory = generated_sets_in_memory();
    let loaded = generated_sets_loaded();
    let lines = margin_lines(&loaded).expect("the margin reading computes");
    assert_eq!(lines, direct_margin_lines(&in_memory));
    let closing = lines.last().expect("a closing line");
    assert_eq!(figure_before(closing, "questions"), 36);
    assert_eq!(figure_before(closing, "blocks"), 91);
    assert_eq!(figure_before(closing, "pluralities"), 73);
    assert_eq!(figure_before(closing, "at one"), 30);
}

#[test]
fn the_generated_family_closes_under_its_own_relations() {
    // The relations of GJ-36 over the generated
    // record: the closings reconcile against the
    // pool and the aggregate and the canvass over the
    // generated sets at the function, and the pooled
    // part of every section stands equal to the
    // aggregate section of its question. The pin
    // fails under the mutation computing the
    // generated closing from the shipped record's
    // figures, whose questions stand at 52 where the
    // family's stand at 36.
    let loaded = generated_sets_loaded();
    assert_closing_relations(&loaded);
    assert_pooled_relations(&loaded);
}
