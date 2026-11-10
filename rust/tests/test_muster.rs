//! Pins for the muster reading and command.
//!
//! The muster pins are stated in GJ-33 and GJ-34 in
//! CONTRACT.md, in the Round 16 section. One section
//! stands per question in the order the questions
//! first appear across the sets: the question line,
//! then one part per offered answer of the question in
//! the union order of its askings. A part opens with
//! the answer line, then states one row per jury of
//! the question in the order the model strings first
//! appear across its askings, the row of the answer in
//! the block of that jury at that question with the
//! model string standing where the block's row names
//! the answer, through the shared row rendering of the
//! agreement reading computed from the summed counts
//! of the blocks, and the part closes with the muster
//! line, the count of the valid responses of the
//! question that chose the answer, the count of the
//! juries whose summed count of the answer stands at
//! one or more, and the count of the juries whose block
//! plurality the answer is. The closing line counts
//! the questions, the parts, the jury rows, and the
//! pluralities of the parts, and the relations of
//! GJ-34 bind the reading to the canvass reading and
//! the aggregate reading and the census reading, the
//! juries figure of a part carrying no relation and
//! standing pinned by independent computation alone.
//! The sixteenth command prints the reading over label
//! and directory pairs in the census command's forms.
//!
//! Discrimination, pin by pin. Pins 1 to 22 in this
//! file fail against the Round 15 closure tree by
//! compile absence, the grandjury::muster module and
//! the muster command and this target and the golden
//! standing nowhere in that tree. Beyond that absence,
//! pin 1 pins the golden byte for byte, pin 2 holds
//! the whole reading against an independent
//! computation over the recorded polls, pin 3 holds
//! every part row equal to the canvass block row it
//! transposes, pin 4 holds the responses figure of
//! every part equal to the aggregate count of its
//! answer, pin 5 holds one jury's counts across the
//! parts of a question summing to its block's valid
//! count, pin 6 reconciles the closing with the
//! canvass and aggregate closings and row counts, pin
//! 7 holds the parts of a one asking question equal
//! to the transposed census parts of that asking, pins
//! 8 to 14 state the hand built shapes, pins 15 and
//! 16 exercise the guard on the model and the
//! question, pin 17 exercises the set rules, pin 18
//! holds the reading under the vocabulary scan, pins
//! 19 to 21 hold the generated family to its stated
//! figures, its recount, and its relations, and pin
//! 22 names the command in the help of the binary and
//! fails at the base by assertion there, the muster
//! standing as no command in the standing binary, so
//! its help run draws the unknown command refusal at
//! status two where the pin requires status zero. The three
//! command pins of the round stand in test_cli.rs and
//! fail there by assertion against the surface's
//! unknown command refusal. Pin 19 in this file
//! carries one further duty, stated at the pin: it
//! passes on the generator GJ-27 states and fails by
//! assertion against the generator form the Round 13
//! reviews found. The two status pins of the round
//! stand in test_setup.rs and are not discrimination
//! pins, as the design of the round states: they fail
//! against the base sources by assertion and pass on
//! the slice by construction of the status commit.
//! No generated pin is a bound: every generated
//! figure below is a computed equality at the seed.

mod shared;

use grandjury::aggregate::aggregate_lines;
use grandjury::canvass::canvass_lines;
use grandjury::census::census_lines;
use grandjury::muster::muster_lines;
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

fn gcd(a: usize, b: usize) -> usize {
    let (mut a, mut b) = (a, b);
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn direct_share(count: usize, valid: usize) -> String {
    if valid == 0 || count == 0 {
        return "0".to_string();
    }
    if count == valid {
        return "1".to_string();
    }
    let g = gcd(count, valid);
    let (n, d) = (count / g, valid / g);
    if d == 1 {
        n.to_string()
    } else {
        format!("{n}/{d}")
    }
}

fn direct_percent(count: usize, valid: usize) -> String {
    if valid == 0 {
        return "0.0".to_string();
    }
    let scaled = (2000 * count + valid) / (2 * valid);
    format!("{}.{}", scaled / 10, scaled % 10)
}

type Counts4 = (usize, usize, usize, usize);

/// One block of a canvass reading as (model, counts,
/// row lines, plurality value), the plurality value
/// standing as the answer's text or the word none.
type Block = (String, Counts4, Vec<String>, String);

/// The sections of a canvass reading and its closing
/// line, parsed from rendered lines, so the relation
/// pins read the canvass as text.
fn parse_canvass(lines: &[String]) -> (Vec<(String, Vec<Block>)>, String) {
    let mut sections: Vec<(String, Vec<Block>)> = Vec::new();
    let mut closing = String::new();
    for line in lines {
        if let Some(question) = line.strip_prefix("question, ") {
            sections.push((question.to_string(), Vec::new()));
        } else if let Some(rest) = line.strip_prefix("jury, ") {
            let split = rest.find(", recorded ").expect("a counts tail");
            let model = rest[..split].to_string();
            let block = (model, counts_of(line), Vec::new(), String::new());
            sections
                .last_mut()
                .expect("a section before its blocks")
                .1
                .push(block);
        } else if line.starts_with("  ") {
            sections
                .last_mut()
                .expect("a section before its rows")
                .1
                .last_mut()
                .expect("a block before its rows")
                .2
                .push(line.clone());
        } else if let Some(value) = line.strip_prefix("plurality, ") {
            sections
                .last_mut()
                .expect("a section before its pluralities")
                .1
                .last_mut()
                .expect("a block before its plurality")
                .3 = value.to_string();
        } else if line.starts_with("canvasses, ") {
            closing = line.clone();
        }
    }
    (sections, closing)
}

/// One part of a muster reading as (answer, row lines,
/// muster line).
type MusterPart = (String, Vec<String>, String);

/// The sections of a muster reading and its closing
/// line, parsed from rendered lines, so the golden
/// pins and the function pins read the one shape.
fn parse_muster(lines: &[String]) -> (Vec<(String, Vec<MusterPart>)>, String) {
    let mut sections: Vec<(String, Vec<MusterPart>)> = Vec::new();
    let mut closing = String::new();
    for line in lines {
        if let Some(question) = line.strip_prefix("question, ") {
            sections.push((question.to_string(), Vec::new()));
        } else if let Some(answer) = line.strip_prefix("answer, ") {
            sections
                .last_mut()
                .expect("a section before its parts")
                .1
                .push((answer.to_string(), Vec::new(), String::new()));
        } else if line.starts_with("  ") {
            sections
                .last_mut()
                .expect("a section before its rows")
                .1
                .last_mut()
                .expect("a part before its rows")
                .1
                .push(line.clone());
        } else if line.starts_with("muster, ") {
            sections
                .last_mut()
                .expect("a section before its muster lines")
                .1
                .last_mut()
                .expect("a part before its muster line")
                .2 = line.clone();
        } else if line.starts_with("musters, ") {
            closing = line.clone();
        }
    }
    (sections, closing)
}

/// The sections of an aggregate reading as (question,
/// row lines) pairs with its closing line, parsed
/// from rendered lines.
fn parse_aggregate(lines: &[String]) -> (Vec<(String, Vec<String>)>, String) {
    let mut sections: Vec<(String, Vec<String>)> = Vec::new();
    let mut closing = String::new();
    for line in lines {
        if let Some(question) = line.strip_prefix("question, ") {
            sections.push((question.to_string(), Vec::new()));
        } else if line.starts_with("  ") {
            sections
                .last_mut()
                .expect("a section before its rows")
                .1
                .push(line.clone());
        } else if line.starts_with("aggregates, ") {
            closing = line.clone();
        }
    }
    (sections, closing)
}

/// One asking part of a census jury as (set label,
/// counts, row lines, plurality value).
type Part = (String, Counts4, Vec<String>, String);

/// The asking parts of the census as (question,
/// model, parts) triples in reading order.
type CensusGroups = Vec<(String, String, Vec<Part>)>;

/// The asking parts of the census over rendered lines,
/// as (question, model, parts) triples in reading
/// order, so the relation pins read the census as
/// text and hold no crate counting of their own
/// beyond the parsers here.
fn parse_census(lines: &[String]) -> CensusGroups {
    let mut groups: Vec<(String, String, Vec<Part>)> = Vec::new();
    let mut question = String::new();
    let mut past_record = false;
    for line in lines {
        if let Some(text) = line.strip_prefix("question, ") {
            question = text.to_string();
        } else if line.starts_with("record, ") {
            past_record = true;
        } else if let Some(rest) = line.strip_prefix("jury, ") {
            if !past_record {
                groups.push((question.clone(), rest.to_string(), Vec::new()));
            }
        } else if let Some(rest) = line.strip_prefix("set, ") {
            let label = rest.split(", ").next().expect("a set label").to_string();
            groups.last_mut().expect("a jury before its parts").2.push((
                label,
                counts_of(line),
                Vec::new(),
                String::new(),
            ));
        } else if line.starts_with("  ") {
            groups
                .last_mut()
                .expect("a jury before its rows")
                .2
                .last_mut()
                .expect("a part before its rows")
                .2
                .push(line.clone());
        } else if let Some(rest) = line.strip_prefix("plurality, ") {
            let part = groups
                .last_mut()
                .expect("a jury before its plurality")
                .2
                .last_mut()
                .expect("a part before its plurality");
            let prefix = format!("{}, ", part.0);
            part.3 = rest
                .strip_prefix(&prefix)
                .expect("the plurality of the part")
                .to_string();
        }
    }
    groups
}

/// The askings of one question in the sets, in asking
/// order.
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
/// directly, as (summed per answer counts, valid
/// count, plurality), the counts of the jury summed
/// over the askings of the question response by
/// response in the test.
fn direct_block(
    sets: &[(PText, Vec<Poll>)],
    question: &str,
    model: &str,
    union: &[String],
) -> (Vec<usize>, usize, Option<String>) {
    let mut summed = vec![0usize; union.len()];
    let mut valid = 0usize;
    for held in askings_of(sets, question) {
        let (counts, per_answer) = direct_jury_counts(held, model, union);
        valid += counts.1;
        for (slot, count) in per_answer.iter().enumerate() {
            summed[slot] += count;
        }
    }
    let plurality = direct_plurality(&summed, valid, union);
    (summed, valid, plurality)
}

/// The gathering of one question computed directly,
/// as (union, models) in the orders GJ-34 states.
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

/// The muster reading computed directly from the
/// loaded polls with this file's own arithmetic, each
/// jury's responses at each asking of a question
/// counted response by response under the validity
/// rule of GJ-8 and summed in the test, the parts
/// stated by transposing those blocks in the test, so
/// the pin holds no crate counting.
fn direct_muster_lines(sets: &[(PText, Vec<Poll>)]) -> Vec<String> {
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
    let mut part_total = 0usize;
    let mut row_total = 0usize;
    let mut plurality_total = 0usize;
    for question in &questions {
        let (union, models) = direct_gathering(sets, question);
        let mut blocks: Vec<(Vec<usize>, usize, Option<String>)> = Vec::new();
        for model in &models {
            blocks.push(direct_block(sets, question, model, &union));
        }
        let mut pooled = vec![0usize; union.len()];
        for held in askings_of(sets, question) {
            for entry in &held.responses {
                if entry.abstained || !held.answers.contains(&entry.answer) {
                    continue;
                }
                let slot = union
                    .iter()
                    .position(|answer| answer == &entry.answer.display())
                    .expect("a union slot");
                pooled[slot] += 1;
            }
        }
        lines.push(format!("question, {question}"));
        for (slot, answer) in union.iter().enumerate() {
            lines.push(format!("answer, {answer}"));
            let mut backers = 0usize;
            let mut carriers = 0usize;
            for (model, (summed, valid, plurality)) in models.iter().zip(blocks.iter()) {
                let count = summed[slot];
                lines.push(format!(
                    "  {model}, {count}, {}, {}%",
                    direct_share(count, *valid),
                    direct_percent(count, *valid)
                ));
                row_total += 1;
                if count >= 1 {
                    backers += 1;
                }
                if plurality.as_ref() == Some(answer) {
                    carriers += 1;
                }
            }
            lines.push(format!(
                "muster, {} responses, {} juries, {} pluralities",
                pooled[slot], backers, carriers
            ));
            part_total += 1;
            plurality_total += carriers;
        }
    }
    lines.push(format!(
        "musters, {} questions, {} answers, {} juries, {} pluralities",
        questions.len(),
        part_total,
        row_total,
        plurality_total
    ));
    lines
}

/// The first relation of GJ-34, checked over any
/// sets: every row of a part stands equal in its
/// count and its share and its percentage to the row
/// of the same answer in the block of the same jury
/// at the same question in the canvass reading.
fn assert_row_relations(sets: &[(PText, Vec<Poll>)]) {
    let lines = muster_lines(sets).expect("the muster reading computes");
    let (sections, _) = parse_muster(&lines);
    let canvass = canvass_lines(sets).expect("the canvass reading computes");
    let (canvass_sections, _) = parse_canvass(&canvass);
    assert_eq!(sections.len(), canvass_sections.len());
    for ((question, parts), (held_question, blocks)) in sections.iter().zip(canvass_sections.iter())
    {
        assert_eq!(question, held_question);
        for (answer, rows, _) in parts {
            for block in blocks {
                let block_row = block
                    .2
                    .iter()
                    .find(|row| row_of(row).0 == *answer)
                    .expect("the block row of the answer");
                let (_, block_count, block_share, block_percent) = row_of(block_row);
                let part_row = rows
                    .iter()
                    .find(|row| row_of(row).0 == block.0)
                    .expect("the part row of the jury");
                let (_, count, share, percent) = row_of(part_row);
                assert_eq!(
                    (count, share, percent),
                    (block_count, block_share, block_percent),
                    "the row of an answer in a part and in its block"
                );
            }
        }
    }
}

/// The second relation of GJ-34, checked over any
/// sets: the responses figure of a part equals the
/// count of the same answer in the section of the
/// same question of the aggregate reading.
fn assert_response_relations(sets: &[(PText, Vec<Poll>)]) {
    let lines = muster_lines(sets).expect("the muster reading computes");
    let (sections, _) = parse_muster(&lines);
    let aggregate = aggregate_lines(sets).expect("the aggregate reading computes");
    let (aggregate_sections, _) = parse_aggregate(&aggregate);
    assert_eq!(sections.len(), aggregate_sections.len());
    for ((question, parts), (held_question, rows)) in sections.iter().zip(aggregate_sections.iter())
    {
        assert_eq!(question, held_question);
        for (answer, _, muster_line) in parts {
            let aggregate_row = rows
                .iter()
                .find(|row| row_of(row).0 == *answer)
                .expect("the aggregate row of the answer");
            let (_, count, _, _) = row_of(aggregate_row);
            assert_eq!(
                figure_before(muster_line, "responses"),
                count,
                "the responses of a part against the aggregate"
            );
        }
    }
}

/// The third relation of GJ-34, checked over any
/// sets: the counts in the rows one jury holds across
/// the parts of one question sum to the valid count
/// of that jury's block in the canvass.
fn assert_sum_relations(sets: &[(PText, Vec<Poll>)]) {
    let lines = muster_lines(sets).expect("the muster reading computes");
    let (sections, _) = parse_muster(&lines);
    let canvass = canvass_lines(sets).expect("the canvass reading computes");
    let (canvass_sections, _) = parse_canvass(&canvass);
    for ((question, parts), (_, blocks)) in sections.iter().zip(canvass_sections.iter()) {
        let _ = question;
        for block in blocks {
            let mut summed = 0usize;
            for (_, rows, _) in parts {
                for row in rows {
                    let (name, count, _, _) = row_of(row);
                    if name == block.0 {
                        summed += count;
                    }
                }
            }
            assert_eq!(summed, block.1 .1, "the rows of a jury across the parts");
        }
    }
}

/// The closing relations of GJ-34, checked over any
/// sets: the pluralities figure of the closing equals
/// the pluralities figure of the canvass closing, the
/// answers figure equals the count of the answer rows
/// over the sections of the aggregate, the juries
/// figure equals the count of the answer rows over
/// the blocks of the canvass, and the questions figure
/// equals the questions figures of both closings.
fn assert_closing_relations(sets: &[(PText, Vec<Poll>)]) {
    let lines = muster_lines(sets).expect("the muster reading computes");
    let (_, closing) = parse_muster(&lines);
    let canvass = canvass_lines(sets).expect("the canvass reading computes");
    let (canvass_sections, canvass_closing) = parse_canvass(&canvass);
    let aggregate = aggregate_lines(sets).expect("the aggregate reading computes");
    let (aggregate_sections, aggregate_closing) = parse_aggregate(&aggregate);
    assert_eq!(
        figure_before(&closing, "questions"),
        figure_before(&canvass_closing, "questions")
    );
    assert_eq!(
        figure_before(&closing, "questions"),
        figure_before(&aggregate_closing, "questions")
    );
    assert_eq!(
        figure_before(&closing, "pluralities"),
        figure_before(&canvass_closing, "pluralities")
    );
    let aggregate_rows: usize = aggregate_sections.iter().map(|(_, rows)| rows.len()).sum();
    assert_eq!(figure_before(&closing, "answers"), aggregate_rows);
    let canvass_rows: usize = canvass_sections
        .iter()
        .flat_map(|(_, blocks)| blocks.iter().map(|block| block.2.len()))
        .sum();
    assert_eq!(figure_before(&closing, "juries"), canvass_rows);
}

/// The eighth relation of GJ-34, checked over any
/// sets at the function: at a question holding
/// exactly one asking, the rows of its parts stand
/// equal to the transposed rows of the parts of that
/// asking in the census reading, the count and the
/// share of the row of an answer in the part of a
/// jury standing equal to the count and the share of
/// the row of that answer in the census part of that
/// jury at that asking.
fn assert_one_asking_relations(sets: &[(PText, Vec<Poll>)]) {
    let lines = muster_lines(sets).expect("the muster reading computes");
    let (sections, _) = parse_muster(&lines);
    let census = census_lines(sets).expect("the census computes");
    let groups = parse_census(&census);
    let mut checked = 0usize;
    for (question, parts) in &sections {
        if askings_of(sets, question).len() != 1 {
            continue;
        }
        checked += 1;
        for (answer, rows, _) in parts {
            for row in rows {
                let (model, count, share, _) = row_of(row);
                let group = groups
                    .iter()
                    .find(|(held_question, held_model, _)| {
                        held_question == question && held_model == &model
                    })
                    .expect("the census jury of the row");
                assert_eq!(group.2.len(), 1, "one part at one asking");
                let census_row = group.2[0]
                    .2
                    .iter()
                    .find(|held| row_of(held).0 == *answer)
                    .expect("the census row of the answer");
                let (_, census_count, census_share, _) = row_of(census_row);
                assert_eq!(
                    (count, share),
                    (census_count, census_share),
                    "the row of a part against the census part"
                );
            }
        }
    }
    assert!(
        checked > 0,
        "the relation quantifies over one asking questions"
    );
}

#[test]
fn the_muster_golden_stands_byte_for_byte() {
    let golden = read_text(&examples().join("muster-golden.txt"));
    let (stdout, _, code) = run_cli(&[
        "muster",
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
fn the_muster_reading_computes_directly_from_the_record() {
    let sets = recorded_sets();
    let expected = direct_muster_lines(&sets);
    assert_eq!(
        muster_lines(&sets).expect("the muster reading computes"),
        expected
    );
    let (stdout, _, code) = run_cli(&[
        "muster",
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
fn the_muster_rows_stand_equal_to_the_canvass_block_rows() {
    assert_row_relations(&recorded_sets());
}

#[test]
fn the_muster_part_responses_stand_equal_to_the_aggregate_counts() {
    assert_response_relations(&recorded_sets());
}

#[test]
fn the_muster_rows_of_one_jury_sum_to_its_block_valid() {
    assert_sum_relations(&recorded_sets());
}

#[test]
fn the_muster_closing_reconciles_with_the_canvass_and_aggregate_closings() {
    assert_closing_relations(&recorded_sets());
}

#[test]
fn the_muster_parts_at_one_asking_questions_transpose_the_census_parts() {
    // The eighth relation of GJ-34 over the recorded
    // sets at the function, quantifying over the 40
    // shipped questions at one asking, counted here as
    // an equality so a record change that moved a
    // question across the one asking line would fail
    // on the count before it failed on any row.
    let sets = recorded_sets();
    let mut one_asking = 0usize;
    let mut questions: Vec<String> = Vec::new();
    for (_, polls) in &sets {
        for held in polls {
            let text = held.question.display();
            if !questions.contains(&text) {
                questions.push(text);
            }
        }
    }
    for question in &questions {
        if askings_of(&sets, question).len() == 1 {
            one_asking += 1;
        }
    }
    assert_eq!(one_asking, 40, "the one asking questions of the record");
    assert_one_asking_relations(&sets);
}

#[test]
fn an_answer_chosen_by_no_jury_stands_at_zero() {
    // The unchosen shape of GJ-34. The one jury votes
    // twice for one answer of the question and never
    // for the other, so the part of the other answer
    // opens with its answer line and states a row of
    // zero for the one jury and closes on a muster
    // line of zero responses and zero juries and zero
    // pluralities, all under the ordinary form.
    let question = "Should the ferry run on Christmas Day?";
    let answers = &["run it", "rest the crew"];
    let sets = vec![(
        pt("alpha"),
        vec![poll(question, answers, votes_of("run it", 2, "model a"))],
    )];
    assert_eq!(
        muster_lines(&sets).expect("the muster reading computes"),
        vec![
            "question, Should the ferry run on Christmas Day?",
            "answer, run it",
            "  model a, 2, 1, 100.0%",
            "muster, 2 responses, 1 juries, 1 pluralities",
            "answer, rest the crew",
            "  model a, 0, 0, 0.0%",
            "muster, 0 responses, 0 juries, 0 pluralities",
            "musters, 1 questions, 2 answers, 2 juries, 1 pluralities",
        ]
    );
}

#[test]
fn an_answer_carrying_the_pooled_count_stands_at_zero_pluralities() {
    // The unbacked plurality shape of GJ-34. Two
    // juries stand at one asking of the question. The
    // first names three for the red gate and two for
    // the blue gate, so its plurality is the red gate.
    // The second names three for the green gate and
    // two for the blue gate, so its plurality is the
    // green gate. The pooled counts stand at four for
    // the blue gate against three and three, so the
    // blue gate is the plurality of the question
    // holding the largest responses figure of the
    // section, and its part closes at zero pluralities
    // while its juries figure stands at two, both
    // juries backing it. A reading that computed a
    // part's plurality figure from the pooled counts,
    // or refused to state a figure it did not hold,
    // would fail here.
    let question = "Which gate should the night watch use?";
    let answers = &["the red gate", "the blue gate", "the green gate"];
    let mut responses = votes_of("the red gate", 3, "model a");
    responses.extend(votes_of("the blue gate", 2, "model a"));
    responses.extend(votes_of("the green gate", 3, "model b"));
    responses.extend(votes_of("the blue gate", 2, "model b"));
    let sets = vec![(pt("alpha"), vec![poll(question, answers, responses)])];
    assert_eq!(
        muster_lines(&sets).expect("the muster reading computes"),
        vec![
            "question, Which gate should the night watch use?",
            "answer, the red gate",
            "  model a, 3, 3/5, 60.0%",
            "  model b, 0, 0, 0.0%",
            "muster, 3 responses, 1 juries, 1 pluralities",
            "answer, the blue gate",
            "  model a, 2, 2/5, 40.0%",
            "  model b, 2, 2/5, 40.0%",
            "muster, 4 responses, 2 juries, 0 pluralities",
            "answer, the green gate",
            "  model a, 0, 0, 0.0%",
            "  model b, 3, 3/5, 60.0%",
            "muster, 3 responses, 1 juries, 1 pluralities",
            "musters, 1 questions, 3 answers, 6 juries, 2 pluralities",
        ]
    );
}

#[test]
fn two_juries_holding_different_pluralities_stand_counted_in_their_own_parts() {
    // The divergence shape of GJ-34, the part form of
    // the divergence the census and the canvass state
    // block by block. The two juries of the pin above
    // name different answers as their pluralities, the
    // first the red gate and the second the green
    // gate, and each part counts its own carrier
    // once, neither jury counted in a part not its
    // own. The parts are read from the reading
    // directly here, and each part's pluralities
    // figure is asserted, the whole reading itself
    // standing pinned as text in the pin above over
    // the same fixture.
    let question = "Which gate should the night watch use?";
    let answers = &["the red gate", "the blue gate", "the green gate"];
    let mut responses = votes_of("the red gate", 3, "model a");
    responses.extend(votes_of("the blue gate", 2, "model a"));
    responses.extend(votes_of("the green gate", 3, "model b"));
    responses.extend(votes_of("the blue gate", 2, "model b"));
    let sets = vec![(pt("alpha"), vec![poll(question, answers, responses)])];
    let lines = muster_lines(&sets).expect("the muster reading computes");
    let (sections, _) = parse_muster(&lines);
    let parts = &sections[0].1;
    assert_eq!(
        figure_before(&parts[0].2, "pluralities"),
        1,
        "the red gate's part counts its one carrier"
    );
    assert_eq!(
        figure_before(&parts[2].2, "pluralities"),
        1,
        "the green gate's part counts its one carrier"
    );
    assert_eq!(
        figure_before(&parts[1].2, "pluralities"),
        0,
        "the blue gate's part counts no carrier"
    );
}

#[test]
fn a_jury_with_no_valid_response_states_a_row_of_zero_in_every_part() {
    // The empty jury shape of GJ-34. The one jury
    // abstains five times across two askings, so it
    // stands at the question with no valid response,
    // and its row in every part states a count of zero
    // and a share of zero, the share computed without
    // a division by its valid count, and no part
    // counts the jury among its backers.
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
        muster_lines(&sets).expect("the muster reading computes"),
        vec![
            "question, Should the ferry run on Christmas Day?",
            "answer, run it",
            "  model a, 0, 0, 0.0%",
            "muster, 0 responses, 0 juries, 0 pluralities",
            "answer, rest the crew",
            "  model a, 0, 0, 0.0%",
            "muster, 0 responses, 0 juries, 0 pluralities",
            "musters, 1 questions, 2 answers, 2 juries, 0 pluralities",
        ]
    );
}

#[test]
fn an_answer_offered_only_at_the_second_asking_enters_the_parts_in_union_order() {
    // The union shape of GJ-34. The question offers
    // two answers at its first asking and three at its
    // second, the east field standing offered only at
    // the second. The one jury answers two for the
    // north field and one for the south field at the
    // first asking and one for the north field and
    // two for the east field at the second, so the
    // parts stand in the union order, the north field
    // and the south field and the east field, at three
    // and one and two responses.
    let question = "Which field should host the maypole?";
    let mut first = votes_of("the north field", 2, "model a");
    first.extend(votes_of("the south field", 1, "model a"));
    let mut second = votes_of("the north field", 1, "model a");
    second.extend(votes_of("the east field", 2, "model a"));
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
        muster_lines(&sets).expect("the muster reading computes"),
        vec![
            "question, Which field should host the maypole?",
            "answer, the north field",
            "  model a, 3, 1/2, 50.0%",
            "muster, 3 responses, 1 juries, 1 pluralities",
            "answer, the south field",
            "  model a, 1, 1/6, 16.7%",
            "muster, 1 responses, 1 juries, 0 pluralities",
            "answer, the east field",
            "  model a, 2, 1/3, 33.3%",
            "muster, 2 responses, 1 juries, 0 pluralities",
            "musters, 1 questions, 3 answers, 3 juries, 1 pluralities",
        ]
    );
}

#[test]
fn a_jury_at_one_asking_of_a_question_sums_that_asking_alone() {
    // The partial jury shape of GJ-34, read through
    // the parts. The question stands at two askings.
    // The first jury, model a, stands at the first
    // asking alone, two for the low meadow and one
    // for the high meadow, so its rows across the
    // parts state that asking alone, two and one,
    // summing to its block's valid count of three.
    // The second jury, model b, stands at both
    // askings, one and two at the first and two and
    // none at the second, pooling to three and two
    // over five valid.
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
        muster_lines(&sets).expect("the muster reading computes"),
        vec![
            "question, Which meadow should host the fair?",
            "answer, the low meadow",
            "  model a, 2, 2/3, 66.7%",
            "  model b, 3, 3/5, 60.0%",
            "muster, 5 responses, 2 juries, 2 pluralities",
            "answer, the high meadow",
            "  model a, 1, 1/3, 33.3%",
            "  model b, 2, 2/5, 40.0%",
            "muster, 3 responses, 2 juries, 0 pluralities",
            "musters, 1 questions, 2 answers, 4 juries, 2 pluralities",
        ]
    );
}

#[test]
fn a_question_whose_pooled_counts_tie_at_the_top_claims_no_pooled_plurality() {
    // The pooled tie shape of GJ-34. The first jury
    // names two for the red gate and one for the blue
    // gate, so its plurality is the red gate. The
    // second names two for the blue gate and one for
    // the red gate, so its plurality is the blue gate.
    // The pooled counts tie at three and three, so the
    // question holds no pooled plurality, and the
    // parts stand in their ordinary form, each part
    // counting the one jury whose block plurality it
    // is, no part claiming a plurality the question
    // does not hold.
    let question = "Which gate should the night watch use?";
    let answers = &["the red gate", "the blue gate"];
    let mut responses = votes_of("the red gate", 2, "model a");
    responses.extend(votes_of("the blue gate", 1, "model a"));
    responses.extend(votes_of("the blue gate", 2, "model b"));
    responses.extend(votes_of("the red gate", 1, "model b"));
    let sets = vec![(pt("alpha"), vec![poll(question, answers, responses)])];
    assert_eq!(
        muster_lines(&sets).expect("the muster reading computes"),
        vec![
            "question, Which gate should the night watch use?",
            "answer, the red gate",
            "  model a, 2, 2/3, 66.7%",
            "  model b, 1, 1/3, 33.3%",
            "muster, 3 responses, 2 juries, 1 pluralities",
            "answer, the blue gate",
            "  model a, 1, 1/3, 33.3%",
            "  model b, 2, 2/3, 66.7%",
            "muster, 3 responses, 2 juries, 1 pluralities",
            "musters, 1 questions, 2 answers, 4 juries, 2 pluralities",
        ]
    );
}

#[test]
fn the_muster_guard_reads_the_model_of_the_second_set() {
    // The guard exercise of GJ-33. The muster prints
    // the model strings of the record on the rows of
    // its parts, and the guard as FIX-GJ-18 left it
    // reads the model of every response before any set
    // rule is read, so a handed poll in the second set
    // whose one response carries a model string
    // holding a closing line after a break draws the
    // guard's model defect at the muster, with no
    // widening of the guard standing in this round.
    let handed = Response {
        juror: pt("beta hand 1"),
        model: pt("model a\nmusters, 9 questions, 9 juries"),
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
        muster_lines(&sets),
        Err("response 0 field model holds a newline".to_string())
    );
}

#[test]
fn the_muster_guard_reads_the_question_of_the_first_set() {
    // The second guard exercise of GJ-33. The muster
    // prints the question strings of the record on its
    // question lines, and the guard reads the question
    // of a poll before its answers and its responses,
    // so a handed poll in the first set whose question
    // string carries a closing line after a break
    // draws the guard's question defect at the
    // muster, before the clean set behind it is read.
    let sets = vec![
        (
            pt("alpha"),
            vec![poll(
                "Should the ferry run on Christmas Day?\nmusters, 9 questions, 9 juries",
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
        muster_lines(&sets),
        Err("field question holds a newline".to_string())
    );
}

#[test]
fn the_muster_set_rules_refuse_as_the_census_refuses() {
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
        muster_lines(&duplicated_label),
        Err("the set label alpha stands twice".to_string())
    );
    let question = "Should the ferry run on Christmas Day?";
    let doubled = vec![(pt("alpha"), vec![first[0].clone(), first[0].clone()])];
    assert_eq!(
        muster_lines(&doubled),
        Err(format!("the set alpha holds the question {question} twice"))
    );
}

#[test]
fn no_verdict_word_stands_in_the_muster_reading() {
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
    let golden = read_text(&examples().join("muster-golden.txt"));
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
        "muster",
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
    // set, the closing word of the reading is musters,
    // and the words muster and musters and answer
    // stand outside the verdict list word by word, the
    // closing word standing in the golden so the check
    // is not vacuous.
    assert!(!VERDICT_WORDS.contains(&"muster"));
    assert!(!VERDICT_WORDS.contains(&"musters"));
    assert!(!VERDICT_WORDS.contains(&"answer"));
    assert!(golden_words.contains(&"musters".to_string()));
    for text in [&golden, &out] {
        for line in text.lines() {
            assert!(
                !line.starts_with("set, "),
                "no line of the reading names a set: {line}"
            );
        }
        let closing = text.lines().last().expect("a closing line");
        assert!(
            closing.starts_with("musters, "),
            "the closing word of the reading is musters: {closing}"
        );
    }
}

#[test]
fn the_muster_help_names_the_command() {
    let (stdout, _, code) = run_cli(&["muster", "--help"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("usage: grandjury muster [-h] label directory [label directory ...]"));
    let (top, _, top_code) = run_cli(&["--help"]);
    assert_eq!(top_code, 0);
    assert!(top.contains("print the muster reading over recorded poll sets"));
}

// The generated fixture family of GJ-27, restated in
// this target as GJ-33 states: polls drawn in the
// test from the stated distribution at the stated
// seed, loaded through the crate's own loader, and
// pinned by the stated figures of GJ-34, by an
// independent recount inside the test, and by the
// relations of GJ-34. No golden ships for the family,
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
        let tmp = TempDir::new("muster-generated");
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
    // GJ-33 as equalities at the seed and counted here
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
    // The shape figures of GJ-34 over the generated
    // record, each computed here from the record
    // directly and stated as an equality at the seed:
    // twelve questions at one asking, thirteen parts
    // whose every row count stands at zero, sixty
    // three parts standing at zero pluralities, two
    // parts holding the largest responses figure of
    // their section at a question whose pooled counts
    // tie at the top and standing at zero pluralities,
    // and no unbacked pooled plurality, a question
    // whose pooled plurality no block plurality of
    // the question names. The zero of the last figure
    // is a counted absence, as GJ-34 states: the
    // unbacked shape itself rests on the hand built
    // pin of this file alone.
    let lines = direct_muster_lines(&in_memory);
    let (sections, closing) = parse_muster(&lines);
    assert_eq!(
        closing, "musters, 36 questions, 110 answers, 277 juries, 73 pluralities",
        "the closing of the generated muster at the seed"
    );
    let mut one_asking = 0usize;
    let mut zero_parts = 0usize;
    let mut zero_plurality_parts = 0usize;
    let mut tied_top = 0usize;
    let mut unbacked = 0usize;
    for (question, parts) in &sections {
        let askings = askings_of(&in_memory, question);
        if askings.len() == 1 {
            one_asking += 1;
        }
        let (union, _) = direct_gathering(&in_memory, question);
        let mut pooled = vec![0usize; union.len()];
        for held in &askings {
            for entry in &held.responses {
                if entry.abstained || !held.answers.contains(&entry.answer) {
                    continue;
                }
                let slot = union
                    .iter()
                    .position(|answer| answer == &entry.answer.display())
                    .expect("a union slot");
                pooled[slot] += 1;
            }
        }
        let pooled_valid: usize = pooled.iter().sum();
        let pooled_plurality = direct_plurality(&pooled, pooled_valid, &union);
        let largest = parts
            .iter()
            .map(|(_, _, line)| figure_before(line, "responses"))
            .max()
            .unwrap_or(0);
        for (slot, (_, _, line)) in parts.iter().enumerate() {
            if figure_before(line, "responses") == 0 {
                zero_parts += 1;
            }
            if figure_before(line, "pluralities") == 0 {
                zero_plurality_parts += 1;
            }
            if pooled_plurality.is_none()
                && figure_before(line, "responses") == largest
                && figure_before(line, "pluralities") == 0
                && pooled[slot] > 0
            {
                tied_top += 1;
            }
        }
        if let Some(held) = &pooled_plurality {
            let slot = union
                .iter()
                .position(|answer| answer == held)
                .expect("the pooled plurality slot");
            if figure_before(&parts[slot].2, "pluralities") == 0 {
                unbacked += 1;
            }
        }
    }
    assert_eq!(one_asking, 12, "the one asking questions of the family");
    assert_eq!(zero_parts, 13, "the parts of the family at zero responses");
    assert_eq!(
        zero_plurality_parts, 63,
        "the parts of the family at zero pluralities"
    );
    assert_eq!(tied_top, 2, "the tied top parts of the family");
    assert_eq!(unbacked, 0, "the unbacked pooled pluralities of the family");
}

#[test]
fn the_generated_family_recounts_independently() {
    let in_memory = generated_sets_in_memory();
    let loaded = generated_sets_loaded();
    let lines = muster_lines(&loaded).expect("the muster reading computes");
    assert_eq!(lines, direct_muster_lines(&in_memory));
    let closing = lines.last().expect("a closing line");
    assert_eq!(figure_before(closing, "questions"), 36);
    assert_eq!(figure_before(closing, "answers"), 110);
    assert_eq!(figure_before(closing, "juries"), 277);
    assert_eq!(figure_before(closing, "pluralities"), 73);
}

#[test]
fn the_generated_family_closes_under_its_own_relations() {
    let loaded = generated_sets_loaded();
    assert_row_relations(&loaded);
    assert_response_relations(&loaded);
    assert_sum_relations(&loaded);
    assert_closing_relations(&loaded);
    assert_one_asking_relations(&loaded);
}
