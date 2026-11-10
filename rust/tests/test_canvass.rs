//! Pins for the canvass reading and command.
//!
//! The canvass pins are stated in GJ-29 and GJ-30 in
//! CONTRACT.md, in the Round 14 section. One section
//! stands per question in the order the questions
//! first appear across the sets: the question line,
//! then one block per jury of the question in the
//! order the model strings first appear across its
//! askings. A block states the jury line in the form
//! of the agreement jury line, one row per offered
//! answer of the question in the union order of its
//! askings through the shared row rendering of the
//! agreement reading computed from the summed counts,
//! and the plurality line in the aggregate form, the
//! plurality the application GJ-29 states in its own
//! words. The closing line counts the questions, the
//! jury blocks, the blocks at which a plurality
//! stands, and the blocks whose standing plurality
//! names the plurality of their question, and the six
//! closing relations of GJ-30 bind the reading to the
//! census reading and the aggregate reading, the last
//! of them carrying no relation and standing pinned by
//! independent computation alone. The fourteenth
//! command prints the reading over label and
//! directory pairs in the census command's forms.
//!
//! Discrimination, pin by pin. Pins 1 to 21 in this
//! file fail against the Round 13 closure tree by
//! compile absence, the grandjury::canvass module and
//! the canvass command and this target and the golden
//! standing nowhere in that tree. The three command
//! pins of the round stand in test_cli.rs and fail
//! there by assertion against the surface's unknown
//! command refusal. Pin 18 in this file carries one
//! further duty, stated at the pin: it passes on the
//! generator GJ-27 states and fails by assertion
//! against the generator form the Round 13 reviews
//! found. No pin of this file and no command pin
//! of this round passes against the closure tree
//! in any mode; the one pin of the round that
//! passes there is pin 25, the named wash in
//! test_setup.rs, which passes against the closure
//! tree by design, as its own comments state. No
//! generated pin is a bound: every generated
//! figure below is a computed equality at the
//! seed.

mod shared;

use grandjury::aggregate::aggregate_lines;
use grandjury::canvass::canvass_lines;
use grandjury::census::census_lines;
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

/// One row line read as (answer, count, fraction,
/// percent), the answer holding any commas of its own.
fn row_of(line: &str) -> (String, usize, String, String) {
    let body = line.strip_prefix("  ").expect("a row line");
    let mut parts = body.rsplitn(4, ", ");
    let percent = parts.next().expect("a percent").to_string();
    let fraction = parts.next().expect("a fraction").to_string();
    let count: usize = parts.next().expect("a count").parse().expect("a row count");
    let answer = parts.next().expect("an answer").to_string();
    (answer, count, fraction, percent)
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
/// line, parsed from rendered lines, so the golden
/// pins and the function pins read the one shape.
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

/// One asking part of a census jury as (set label,
/// counts, row lines, plurality value).
type Part = (String, Counts4, Vec<String>, String);

/// The asking parts of the census as (question,
/// model, parts) triples in reading order.
type CensusGroups = Vec<(String, String, Vec<Part>)>;

/// The totals lines of the census as (model, counts)
/// pairs.
type CensusTotals = Vec<(String, Counts4)>;

/// The asking parts of the census over rendered lines,
/// as (question, model, parts) triples in reading
/// order, with the totals lines of the census as
/// (model, counts) pairs, so the relation pins read
/// the census as text and hold no crate counting of
/// their own beyond the parsers here.
fn parse_census(lines: &[String]) -> (CensusGroups, CensusTotals) {
    let mut groups: Vec<(String, String, Vec<Part>)> = Vec::new();
    let mut totals: Vec<(String, Counts4)> = Vec::new();
    let mut question = String::new();
    let mut past_record = false;
    for line in lines {
        if let Some(text) = line.strip_prefix("question, ") {
            question = text.to_string();
        } else if line.starts_with("record, ") {
            past_record = true;
        } else if let Some(rest) = line.strip_prefix("jury, ") {
            if past_record {
                let split = rest.find(", recorded ").expect("a counts tail");
                totals.push((rest[..split].to_string(), counts_of(line)));
            } else {
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
    (groups, totals)
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

/// The canvass reading computed directly from the
/// loaded polls with this file's own arithmetic, each
/// jury's responses at each asking of a question
/// counted response by response under the validity
/// rule of GJ-8 and summed in the test, so the pin
/// holds no crate counting.
fn direct_canvass_lines(sets: &[(PText, Vec<Poll>)]) -> Vec<String> {
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
    let mut with_total = 0usize;
    for question in &questions {
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
        // The pooled counts of the whole question,
        // every response of every asking counted once.
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
        let question_plurality = direct_plurality(&pooled, pooled_valid, &union);
        lines.push(format!("question, {question}"));
        for model in &models {
            let mut summed = vec![0usize; union.len()];
            let (mut recorded, mut valid, mut abstained, mut spoiled) = (0, 0, 0, 0);
            for held in &askings {
                let (counts, per_answer) = direct_jury_counts(held, model, &union);
                recorded += counts.0;
                valid += counts.1;
                abstained += counts.2;
                spoiled += counts.3;
                for (slot, count) in per_answer.iter().enumerate() {
                    summed[slot] += count;
                }
            }
            lines.push(format!(
                "jury, {model}, recorded {recorded}, valid {valid}, abstained {abstained}, spoiled {spoiled}"
            ));
            for (answer, count) in union.iter().zip(summed.iter()) {
                lines.push(format!(
                    "  {answer}, {count}, {}, {}%",
                    direct_share(*count, valid),
                    direct_percent(*count, valid)
                ));
            }
            let plurality = direct_plurality(&summed, valid, &union);
            match &plurality {
                Some(answer) => {
                    lines.push(format!("plurality, {answer}"));
                    plurality_total += 1;
                    if question_plurality.as_ref() == Some(answer) {
                        with_total += 1;
                    }
                }
                None => lines.push("plurality, none".to_string()),
            }
            block_total += 1;
        }
    }
    lines.push(format!(
        "canvasses, {} questions, {} juries, {} pluralities, {} with the plurality",
        questions.len(),
        block_total,
        plurality_total,
        with_total
    ));
    lines
}

/// The first two closing relations of GJ-30, checked
/// over any sets: inside one block the answer counts
/// sum to the block's valid count, and the four counts
/// of the blocks of one question summed equal the
/// counts of that question's section in the aggregate
/// reading over the same sets. The sixth relation of
/// GJ-30, the count of blocks standing with the
/// plurality, holds no place here, as GJ-30 states:
/// it stands pinned by the direct computations of
/// this file alone.
fn assert_block_relations(sets: &[(PText, Vec<Poll>)]) {
    let lines = canvass_lines(sets).expect("the canvass reading computes");
    let (sections, _) = parse_canvass(&lines);
    for (_, blocks) in &sections {
        for block in blocks {
            let row_sum: usize = block.2.iter().map(|row| row_of(row).1).sum();
            assert_eq!(row_sum, block.1 .1, "the answer counts of a block");
        }
    }
    let aggregate = aggregate_lines(sets).expect("the aggregate reading computes");
    let mut aggregate_counts: Vec<(String, Counts4)> = Vec::new();
    let mut current = String::new();
    for line in &aggregate {
        if let Some(question) = line.strip_prefix("question, ") {
            current = question.to_string();
        } else if line.starts_with("pool, ") {
            aggregate_counts.push((current.clone(), counts_of(line)));
        }
    }
    assert_eq!(sections.len(), aggregate_counts.len());
    for ((question, blocks), (held_question, held_counts)) in
        sections.iter().zip(aggregate_counts.iter())
    {
        assert_eq!(question, held_question);
        let mut summed: Counts4 = (0, 0, 0, 0);
        for block in blocks {
            summed.0 += block.1 .0;
            summed.1 += block.1 .1;
            summed.2 += block.1 .2;
            summed.3 += block.1 .3;
        }
        assert_eq!(&summed, held_counts, "the blocks of a question summed");
    }
}

/// The third closing relation of GJ-30, checked over
/// any sets: the count of every answer in a block
/// equals the sum of that answer's counts over the
/// parts of that jury at that question in the census
/// reading over the same sets.
fn assert_part_relations(sets: &[(PText, Vec<Poll>)]) {
    let lines = canvass_lines(sets).expect("the canvass reading computes");
    let (sections, _) = parse_canvass(&lines);
    let census = census_lines(sets).expect("the census computes");
    let (groups, _) = parse_census(&census);
    let block_total: usize = sections.iter().map(|(_, blocks)| blocks.len()).sum();
    assert_eq!(groups.len(), block_total, "one census jury per block");
    for (question, blocks) in &sections {
        for block in blocks {
            let group = groups
                .iter()
                .find(|(held_question, model, _)| held_question == question && model == &block.0)
                .expect("the census jury of the block");
            for row in &block.2 {
                let (answer, count, _, _) = row_of(row);
                let summed: usize = group
                    .2
                    .iter()
                    .flat_map(|part| part.2.iter())
                    .map(|part_row| row_of(part_row))
                    .filter(|(held_answer, _, _, _)| held_answer == &answer)
                    .map(|(_, held_count, _, _)| held_count)
                    .sum();
                assert_eq!(summed, count, "the count of an answer over the parts");
            }
        }
    }
}

/// The fourth closing relation of GJ-30, checked over
/// any sets at the function: at a question holding
/// exactly one asking, a block equals the census part
/// of its jury at that asking whole, its counts, its
/// rows, and its plurality.
fn assert_one_asking_relations(sets: &[(PText, Vec<Poll>)]) {
    let lines = canvass_lines(sets).expect("the canvass reading computes");
    let (sections, _) = parse_canvass(&lines);
    let census = census_lines(sets).expect("the census computes");
    let (groups, _) = parse_census(&census);
    for (question, blocks) in &sections {
        if askings_of(sets, question).len() != 1 {
            continue;
        }
        for block in blocks {
            let group = groups
                .iter()
                .find(|(held_question, model, _)| held_question == question && model == &block.0)
                .expect("the census jury of the block");
            assert_eq!(group.2.len(), 1, "one part at one asking");
            let part = &group.2[0];
            assert_eq!(block.1, part.1, "the counts of the block and the part");
            assert_eq!(block.2, part.2, "the rows of the block and the part");
            assert_eq!(block.3, part.3, "the plurality of the block and the part");
        }
    }
}

/// The fifth closing relation of GJ-30, checked over
/// any sets: the four counts of one jury summed over
/// all its blocks across the questions equal the four
/// counts of that jury's totals line in the census
/// over the same sets.
fn assert_total_relations(sets: &[(PText, Vec<Poll>)]) {
    let lines = canvass_lines(sets).expect("the canvass reading computes");
    let (sections, _) = parse_canvass(&lines);
    let census = census_lines(sets).expect("the census computes");
    let (_, totals) = parse_census(&census);
    for (model, total) in &totals {
        let mut summed: Counts4 = (0, 0, 0, 0);
        for (_, blocks) in &sections {
            for block in blocks {
                if &block.0 == model {
                    summed.0 += block.1 .0;
                    summed.1 += block.1 .1;
                    summed.2 += block.1 .2;
                    summed.3 += block.1 .3;
                }
            }
        }
        assert_eq!(&summed, total, "the blocks of a jury summed");
    }
}

#[test]
fn the_canvass_golden_stands_byte_for_byte() {
    let golden = read_text(&examples().join("canvass-golden.txt"));
    let (stdout, _, code) = run_cli(&[
        "canvass",
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
fn the_canvass_reading_computes_directly_from_the_record() {
    let sets = recorded_sets();
    let expected = direct_canvass_lines(&sets);
    assert_eq!(
        canvass_lines(&sets).expect("the canvass reading computes"),
        expected
    );
    let (stdout, _, code) = run_cli(&[
        "canvass",
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
fn the_canvass_blocks_reconcile_with_the_aggregate_reading() {
    assert_block_relations(&recorded_sets());
}

#[test]
fn the_canvass_blocks_recount_from_the_census_parts() {
    assert_part_relations(&recorded_sets());
}

#[test]
fn the_canvass_blocks_at_one_asking_questions_equal_their_census_parts() {
    // The fourth closing relation of GJ-30 read over
    // the two goldens as text: at every question the
    // shipped census holds at one asking, every block
    // of the shipped canvass equals the census part of
    // its jury at that asking whole. The census golden
    // marks a question as one asking where its parts
    // carry one set label between them. The shipped
    // record holds 40 such questions and 77 blocks at
    // them, and the pin counts both as equalities, so
    // a record change that moved a question across the
    // one asking line would fail here on the counts
    // before it failed on any block.
    let golden_lines: Vec<String> = read_text(&examples().join("canvass-golden.txt"))
        .lines()
        .map(str::to_string)
        .collect();
    let (sections, _) = parse_canvass(&golden_lines);
    let census_text: Vec<String> = read_text(&examples().join("census-golden.txt"))
        .lines()
        .map(str::to_string)
        .collect();
    let (groups, _) = parse_census(&census_text);
    let mut one_asking_questions = 0usize;
    let mut one_asking_blocks = 0usize;
    for (question, blocks) in &sections {
        let mut labels: Vec<String> = Vec::new();
        for (held_question, _, parts) in &groups {
            if held_question == question {
                for part in parts {
                    if !labels.contains(&part.0) {
                        labels.push(part.0.clone());
                    }
                }
            }
        }
        if labels.len() != 1 {
            continue;
        }
        one_asking_questions += 1;
        for block in blocks {
            one_asking_blocks += 1;
            let group = groups
                .iter()
                .find(|(held_question, model, _)| held_question == question && model == &block.0)
                .expect("the census jury of the block");
            assert_eq!(group.2.len(), 1, "one part at one asking");
            let part = &group.2[0];
            assert_eq!(block.1, part.1, "the counts of the block and the part");
            assert_eq!(block.2, part.2, "the rows of the block and the part");
            assert_eq!(block.3, part.3, "the plurality of the block and the part");
        }
    }
    assert_eq!(one_asking_questions, 40);
    assert_eq!(one_asking_blocks, 77);
}

#[test]
fn the_canvass_blocks_sum_to_the_census_totals() {
    let sets = recorded_sets();
    let census = census_lines(&sets).expect("the census computes");
    let (_, totals) = parse_census(&census);
    assert_eq!(totals.len(), 2, "the two juries of the record");
    assert_total_relations(&sets);
}

#[test]
fn the_canvass_help_names_the_command() {
    let (stdout, _, code) = run_cli(&["canvass", "--help"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("usage: grandjury canvass [-h] label directory [label directory ...]"));
    let (top, _, top_code) = run_cli(&["--help"]);
    assert_eq!(top_code, 0);
    assert!(top.contains("print the canvass reading over recorded poll sets"));
}

#[test]
fn a_jury_plurality_can_flip_across_askings() {
    // GJ-30 states this shape in its own words, a jury
    // holding the plurality for one answer at one
    // asking of a question and for another answer at
    // another asking, whose summed counts close on a
    // third answer. The hand computation stands as
    // follows. The one jury answers four for the north
    // gate, one for the south gate, and three for the
    // east gate at the first asking, a plurality for
    // the north gate, and one, four, and three at the
    // second asking, a plurality for the south gate.
    // The summed counts stand at five, five, and six,
    // so the block closes on the east gate, an answer
    // that held the plurality at neither asking. A
    // reading that carried either asking's plurality
    // into the block, or that pooled the answers in
    // any order but the union order, would print a
    // different plurality line, so the pin fails under
    // those readings and passes only under the summed
    // plurality GJ-29 states. The shipped record holds
    // no such block and the generated record holds
    // none at the seed, so this shape rests on this
    // pin alone, as GJ-30 states.
    let question = "Which gate should the night watch use?";
    let answers = &["the north gate", "the south gate", "the east gate"];
    let mut first = votes_of("the north gate", 4, "model a");
    first.extend(votes_of("the south gate", 1, "model a"));
    first.extend(votes_of("the east gate", 3, "model a"));
    let mut second = votes_of("the north gate", 1, "model a");
    second.extend(votes_of("the south gate", 4, "model a"));
    second.extend(votes_of("the east gate", 3, "model a"));
    let sets = vec![
        (pt("alpha"), vec![poll(question, answers, first)]),
        (pt("beta"), vec![poll(question, answers, second)]),
    ];
    assert_eq!(
        canvass_lines(&sets).expect("the canvass reading computes"),
        vec![
            "question, Which gate should the night watch use?",
            "jury, model a, recorded 16, valid 16, abstained 0, spoiled 0",
            "  the north gate, 5, 5/16, 31.3%",
            "  the south gate, 5, 5/16, 31.3%",
            "  the east gate, 6, 3/8, 37.5%",
            "plurality, the east gate",
            "canvasses, 1 questions, 1 juries, 1 pluralities, 1 with the plurality",
        ]
    );
}

#[test]
fn a_question_can_hold_one_jury_with_its_plurality_and_one_against_it() {
    // The divergence shape of GJ-30, stated here whole
    // so the closing count of blocks standing with the
    // plurality carries a hand computation. The first
    // jury answers three for the river path and one
    // for the hill path at each asking, summing to six
    // and two, a plurality for the river path. The
    // second jury answers one and three at the first
    // asking and two and three at the second, summing
    // to three and six, a plurality for the hill path.
    // The pooled counts of the question stand at nine
    // for the river path and eight for the hill path,
    // so the plurality of the question is the river
    // path: the first block stands with it and the
    // second stands against it, and the closing counts
    // one with the plurality of the two pluralities
    // standing.
    let question = "Which path should the parish walk take?";
    let answers = &["the river path", "the hill path"];
    let mut first = votes_of("the river path", 3, "model a");
    first.extend(votes_of("the hill path", 1, "model a"));
    first.extend(votes_of("the river path", 1, "model b"));
    first.extend(votes_of("the hill path", 3, "model b"));
    let mut second = votes_of("the river path", 3, "model a");
    second.extend(votes_of("the hill path", 1, "model a"));
    second.extend(votes_of("the river path", 2, "model b"));
    second.extend(votes_of("the hill path", 3, "model b"));
    let sets = vec![
        (pt("alpha"), vec![poll(question, answers, first)]),
        (pt("beta"), vec![poll(question, answers, second)]),
    ];
    assert_eq!(
        canvass_lines(&sets).expect("the canvass reading computes"),
        vec![
            "question, Which path should the parish walk take?",
            "jury, model a, recorded 8, valid 8, abstained 0, spoiled 0",
            "  the river path, 6, 3/4, 75.0%",
            "  the hill path, 2, 1/4, 25.0%",
            "plurality, the river path",
            "jury, model b, recorded 9, valid 9, abstained 0, spoiled 0",
            "  the river path, 3, 1/3, 33.3%",
            "  the hill path, 6, 2/3, 66.7%",
            "plurality, the hill path",
            "canvasses, 1 questions, 2 juries, 2 pluralities, 1 with the plurality",
        ]
    );
}

#[test]
fn a_jury_top_can_tie_across_askings_and_close_at_none() {
    // The pooled tie shape of GJ-30. The jury answers
    // three for the north gate and one for the south
    // gate at the first asking and one and three at
    // the second, a plurality standing at each asking,
    // and the summed counts stand at four and four,
    // equal, so no answer holds the largest summed
    // count alone and the block closes on the word
    // none, and the closing counts no plurality and
    // nothing with the plurality.
    let question = "Which gate should the dawn watch use?";
    let answers = &["the north gate", "the south gate"];
    let mut first = votes_of("the north gate", 3, "model a");
    first.extend(votes_of("the south gate", 1, "model a"));
    let mut second = votes_of("the north gate", 1, "model a");
    second.extend(votes_of("the south gate", 3, "model a"));
    let sets = vec![
        (pt("alpha"), vec![poll(question, answers, first)]),
        (pt("beta"), vec![poll(question, answers, second)]),
    ];
    assert_eq!(
        canvass_lines(&sets).expect("the canvass reading computes"),
        vec![
            "question, Which gate should the dawn watch use?",
            "jury, model a, recorded 8, valid 8, abstained 0, spoiled 0",
            "  the north gate, 4, 1/2, 50.0%",
            "  the south gate, 4, 1/2, 50.0%",
            "plurality, none",
            "canvasses, 1 questions, 1 juries, 0 pluralities, 0 with the plurality",
        ]
    );
}

#[test]
fn a_jury_at_one_asking_of_a_question_sums_that_asking_alone() {
    // The partial jury shape of GJ-30. The first jury
    // stands at the first asking alone, answering two
    // for the low meadow and one for the high meadow,
    // so its block sums that asking alone and stands
    // equal to its census part at that asking whole,
    // its counts and its rows and its plurality. The
    // second jury stands at both askings, one and two
    // at the first and two and none at the second,
    // pooling to three and two over five valid. The
    // pooled counts of the question stand at five and
    // three, so both blocks stand with the plurality
    // of the question.
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
        canvass_lines(&sets).expect("the canvass reading computes"),
        vec![
            "question, Which meadow should host the fair?",
            "jury, model a, recorded 3, valid 3, abstained 0, spoiled 0",
            "  the low meadow, 2, 2/3, 66.7%",
            "  the high meadow, 1, 1/3, 33.3%",
            "plurality, the low meadow",
            "jury, model b, recorded 5, valid 5, abstained 0, spoiled 0",
            "  the low meadow, 3, 3/5, 60.0%",
            "  the high meadow, 2, 2/5, 40.0%",
            "plurality, the low meadow",
            "canvasses, 1 questions, 2 juries, 2 pluralities, 2 with the plurality",
        ]
    );
}

#[test]
fn a_jury_with_no_valid_response_closes_at_none() {
    // The empty jury shape of GJ-30. The one jury
    // abstains twice at the first asking and three
    // times at the second, so its block records five
    // responses and no valid response, every share
    // stands at zero, and the block closes on the word
    // none.
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
        canvass_lines(&sets).expect("the canvass reading computes"),
        vec![
            "question, Should the ferry run on Christmas Day?",
            "jury, model a, recorded 5, valid 0, abstained 5, spoiled 0",
            "  run it, 0, 0, 0.0%",
            "  rest the crew, 0, 0, 0.0%",
            "plurality, none",
            "canvasses, 1 questions, 1 juries, 0 pluralities, 0 with the plurality",
        ]
    );
}

#[test]
fn an_answer_offered_at_the_second_asking_enters_the_block_in_union_order() {
    // The union shape of GJ-30. The question offers
    // two answers at its first asking and three at its
    // second, the east field standing offered only at
    // the second. The one jury answers two for the
    // north field and one for the south field at the
    // first asking and one for the north field and
    // two for the east field at the second, so its
    // block states the three rows in the union order,
    // the north field and the south field and the east
    // field, at three and one and two over six valid.
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
        canvass_lines(&sets).expect("the canvass reading computes"),
        vec![
            "question, Which field should host the maypole?",
            "jury, model a, recorded 6, valid 6, abstained 0, spoiled 0",
            "  the north field, 3, 1/2, 50.0%",
            "  the south field, 1, 1/6, 16.7%",
            "  the east field, 2, 1/3, 33.3%",
            "plurality, the north field",
            "canvasses, 1 questions, 1 juries, 1 pluralities, 1 with the plurality",
        ]
    );
}

#[test]
fn the_canvass_guard_reads_the_model_of_the_second_set() {
    // The guard exercise of GJ-29. The canvass prints
    // the model strings of the record on its jury
    // lines, and the guard as FIX-GJ-18 left it reads
    // the model of every response before any set rule
    // is read, so a handed poll in the second set
    // whose one response carries a model string
    // holding a closing line after a break draws the
    // guard's model defect at the canvass, with no
    // widening of the guard standing in this round.
    let handed = Response {
        juror: pt("beta hand 1"),
        model: pt("model a\ncanvasses, 9 questions, 9 juries"),
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
        canvass_lines(&sets),
        Err("response 0 field model holds a newline".to_string())
    );
}

#[test]
fn the_canvass_guard_reads_the_question_of_the_first_set() {
    // The second guard exercise of GJ-29. The canvass
    // prints the question strings of the record on its
    // question lines, and the guard reads the question
    // of a poll before its answers and its responses,
    // so a handed poll in the first set whose question
    // string carries a closing line after a break
    // draws the guard's question defect at the
    // canvass, before the clean set behind it is read.
    let sets = vec![
        (
            pt("alpha"),
            vec![poll(
                "Should the ferry run on Christmas Day?\ncanvasses, 9 questions, 9 juries",
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
        canvass_lines(&sets),
        Err("field question holds a newline".to_string())
    );
}

#[test]
fn the_canvass_set_rules_refuse_as_the_census_refuses() {
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
        canvass_lines(&duplicated_label),
        Err("the set label alpha stands twice".to_string())
    );
    let question = "Should the ferry run on Christmas Day?";
    let doubled = vec![(pt("alpha"), vec![first[0].clone(), first[0].clone()])];
    assert_eq!(
        canvass_lines(&doubled),
        Err(format!("the set alpha holds the question {question} twice"))
    );
}

#[test]
fn no_verdict_word_stands_in_the_canvass_reading() {
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
    let golden = read_text(&examples().join("canvass-golden.txt"));
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
        "canvass",
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
    // canvasses, and the words canvass and canvasses
    // stand outside the verdict list word by word, the
    // closing word standing in the golden so the check
    // is not vacuous.
    assert!(!VERDICT_WORDS.contains(&"canvass"));
    assert!(!VERDICT_WORDS.contains(&"canvasses"));
    assert!(golden_words.contains(&"canvasses".to_string()));
    for text in [&golden, &out] {
        for line in text.lines() {
            assert!(
                !line.starts_with("set, "),
                "no line of the reading names a set: {line}"
            );
        }
        let closing = text.lines().last().expect("a closing line");
        assert!(
            closing.starts_with("canvasses, "),
            "the closing word of the reading is canvasses: {closing}"
        );
    }
}

// The generated fixture family of GJ-27, restated in
// this target as GJ-29 states: polls drawn in the
// test from the stated distribution at the stated
// seed, loaded through the crate's own loader, and
// pinned by the closing relations of GJ-30, by an
// independent recount inside the test, and by the
// stated figures of the family as equalities at the
// seed. No golden ships for the family, as GJ-27
// states: its sections are pinned by the relations
// and the recount, never by a stored rendering.

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
        let tmp = TempDir::new("canvass-generated");
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
    // GJ-29 as equalities at the seed and counted here
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
}

#[test]
fn the_generated_family_recounts_independently() {
    let in_memory = generated_sets_in_memory();
    let loaded = generated_sets_loaded();
    let lines = canvass_lines(&loaded).expect("the canvass reading computes");
    assert_eq!(lines, direct_canvass_lines(&in_memory));
    let closing = lines.last().expect("a closing line");
    assert_eq!(figure_before(closing, "questions"), 36);
    assert_eq!(figure_before(closing, "juries"), 91);
    assert_eq!(figure_before(closing, "pluralities"), 73);
    assert_eq!(figure_before(closing, "with the plurality"), 60);
}

#[test]
fn the_generated_family_closes_under_its_own_relations() {
    let loaded = generated_sets_loaded();
    assert_block_relations(&loaded);
    assert_part_relations(&loaded);
    assert_one_asking_relations(&loaded);
    assert_total_relations(&loaded);
}

#[test]
fn the_generated_family_holds_its_stated_shapes() {
    // The shape figures of GJ-30 over the generated
    // record, each computed here by walking the record
    // jury by jury and stated as an equality at the
    // seed: twelve questions at three askings, forty
    // one blocks of a jury standing at fewer askings
    // than its question holds, eleven blocks whose
    // standing plurality differs from the plurality of
    // their question, three blocks holding a plurality
    // at some asking of their question and none pooled
    // over a tied summed top, and no flip
    // block, a jury holding one plurality at one
    // asking and another at another whose summed
    // plurality is an answer no asking plurality of
    // its jury named. The zero of the last figure is a
    // counted absence: the flip shape itself rests on
    // the hand-built pin of this file alone, as GJ-30
    // states.
    let in_memory = generated_sets_in_memory();
    let mut askings_of_map: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();
    for (_, set_polls) in &in_memory {
        for held in set_polls {
            *askings_of_map.entry(held.question.display()).or_insert(0) += 1;
        }
    }
    assert_eq!(askings_of_map.len(), 36);
    assert_eq!(
        askings_of_map.values().filter(|count| **count == 3).count(),
        12,
        "twelve generated questions stand at three askings"
    );
    let mut questions: Vec<String> = Vec::new();
    for (_, set_polls) in &in_memory {
        for held in set_polls {
            let text = held.question.display();
            if !questions.contains(&text) {
                questions.push(text);
            }
        }
    }
    let (mut partial, mut divergence, mut ties, mut flips) = (0usize, 0usize, 0usize, 0usize);
    for question in &questions {
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
        let question_plurality = direct_plurality(&pooled, pooled_valid, &union);
        let mut models: Vec<String> = Vec::new();
        for held in &askings {
            for entry in &held.responses {
                let model = entry.model.display();
                if !models.contains(&model) {
                    models.push(model);
                }
            }
        }
        for model in &models {
            let mut summed = vec![0usize; union.len()];
            let mut block_valid = 0usize;
            let mut stood = 0usize;
            let mut standings: Vec<String> = Vec::new();
            for held in &askings {
                let (counts, per_answer) = direct_jury_counts(held, model, &union);
                if counts.0 == 0 {
                    continue;
                }
                stood += 1;
                block_valid += counts.1;
                for (slot, count) in per_answer.iter().enumerate() {
                    summed[slot] += count;
                }
                if let Some(answer) = direct_plurality(&per_answer, counts.1, &union) {
                    if !standings.contains(&answer) {
                        standings.push(answer);
                    }
                }
            }
            if stood < askings.len() {
                partial += 1;
            }
            let block_plurality = direct_plurality(&summed, block_valid, &union);
            match (&block_plurality, &question_plurality) {
                (Some(held), Some(question_held)) if held != question_held => {
                    divergence += 1;
                }
                _ => {}
            }
            if !standings.is_empty() && block_plurality.is_none() {
                ties += 1;
            }
            let flip = standings.len() >= 2
                && block_plurality
                    .as_ref()
                    .is_some_and(|held| !standings.contains(held));
            if flip {
                flips += 1;
            }
        }
    }
    assert_eq!(partial, 41, "the partial jury blocks of the family");
    assert_eq!(divergence, 11, "the divergence blocks of the family");
    assert_eq!(ties, 3, "the pooled tie blocks of the family");
    assert_eq!(flips, 0, "the flip blocks of the family");
}
