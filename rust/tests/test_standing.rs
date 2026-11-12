//! The standing reading of GJ-39 and GJ-40.
//!
//! The pins restate the standing from first principles.
//! Direct transcription stands beside the crate's reading
//! over the shipped record and the generated family, and
//! the parts themselves stand regrouped against the
//! accord reading's own parts, so a standing part is the
//! accord part of the same set at the same question by
//! construction and by check.
//!
//! The discrimination split, stated pin by pin. NA sums
//! a part's counts over every asking of the question in
//! every set, and pins 1, 2, 3, 9, 10, 11, 12, and 19 fail
//! under it.
//! NB swaps the in accord and with the pool standings in
//! their counting, and pins 9 and 10 carry its named
//! failure, with pins 12 and 19 failing under it by
//! transcription, the accord figure computed at 43 and
//! the pool figure at 53; pin 1 passes under it by design,
//! the shipped figures standing equal at 55 and 55, and
//! pin 2's transcription claim for it stands at the pins
//! named here. NC counts a tied part with a plurality, and
//! pins 7, 10, and 19 fail under it, pin 4 standing under
//! it, its asserted sums standing invariant under it, the
//! generated sections standing tied at nine and the
//! shipped sections standing tied at zero. ND omits
//! a part whose valid count stands at zero, and pins 1,
//! 2, 3, 5, 8, 19, and 20 fail under it, the shipped parts
//! figure moving from 64 to 55. NE reads a part's margin
//! off the smallest other count, and pins 1, 2, 3, 10, and
//! 19 fail under it.
//! NF stands a plurality at a tied top count, and pins 7,
//! 10, and 19 fail under it, pin 1 standing under it by
//! design. NG gathers a part's union from the first set's
//! askings alone, and pins 4, 12, 19, and 20 fail under it,
//! the union change moving counts on parts whose askings
//! differ from the first set's. NQ sums a set line's
//! counts over the first poll of the set alone, and pin 1
//! fails under
//! it. NR counts every question of the record in a
//! section's questions figure, and pin 11 fails under it.
//! NS counts the closing's in accord figure by questions
//! rather than parts, and pin 6 fails under it, the
//! shipped record standing beside it at 55 parts in
//! accord against the 44 questions of the accord closing.
//! NH reads the guard after the set rules and NI over the
//! first section's set only; pin 13 stands against both
//! by the guard postures carried from the accord warm-up,
//! and pin 14 stands under all three guard variants as
//! the GJ-11 rules refuse the same fixture one step
//! later. NJ removes the duplicate label check and NT
//! moves the per set question loop ahead of the label
//! checks, and pin 15 fails under both. NK restates the
//! closing word as verdict, and pins 1 and 16 fail under
//! it. NL omits the command from the help, and pin 17
//! fails under it. NM runs the tally handler on the
//! standing command's first value, and pin 23 fails under
//! it; pins 21 and 22 stand under it by design, the tally
//! handler drawing the same usage refusals at the same
//! statuses. NN restates the generated family with the
//! Round 13 fault, and pin 18 fails under it. NO computes
//! the closing line from the shipped record's figures,
//! and pins 5 and 20 fail under it. NP restates the
//! round's README status, and the setup pin of the
//! examples and status step fails under it. Against the
//! Round 18 closure tree the twenty pins of this target
//! fail at compile with the standing module absent, and
//! the three command pins of the CLI target and the two
//! status pins of the setup target fail by assertion.

mod shared;

use shared::*;

use grandjury::accord::accord_lines;
use grandjury::aggregate::aggregate_lines;
use grandjury::polls::{load_poll_file, Poll, Response};
use grandjury::pytext::PText;
use grandjury::standing::standing_lines;

/// One recorded poll set, its label and its polls.
type SetPolls = (PText, Vec<Poll>);

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

/// n standing responses of one jury naming one answer.
fn votes_of(jury: &str, answer: &str, n: usize) -> Vec<Response> {
    (0..n)
        .map(|i| {
            response(
                &format!("{jury} hand {answer} {}", i + 1),
                "model a",
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

/// The figure standing before a word in a summary line.
fn figure_before(line: &str, word: &str) -> usize {
    for part in line.split(", ") {
        if let Some(head) = part.strip_suffix(word) {
            return head.trim().parse().expect("a stated figure");
        }
    }
    panic!("{} stands in {}", word, line);
}

/// The four counts of a set line, in the stated order.
#[derive(Clone, Copy, PartialEq, Debug)]
struct Counts4(usize, usize, usize, usize);

fn counts_text(counts: Counts4) -> String {
    format!(
        "recorded {}, valid {}, abstained {}, spoiled {}",
        counts.0, counts.1, counts.2, counts.3
    )
}

/// The plurality of counted answers under GJ-27, read
/// straight off the counts. A plurality stands only
/// where the valid count stands above zero and one
/// answer holds the largest count alone.
fn direct_plurality(counts: &[(String, usize)], valid: usize) -> Option<String> {
    if valid == 0 {
        return None;
    }
    let mut best: Option<&(String, usize)> = None;
    let mut tied = false;
    for entry in counts {
        match best {
            None => best = Some(entry),
            Some(standing) => {
                if entry.1 > standing.1 {
                    best = Some(entry);
                    tied = false;
                } else if entry.1 == standing.1 {
                    tied = true;
                }
            }
        }
    }
    if tied {
        None
    } else {
        best.map(|entry| entry.0.clone())
    }
}

/// The margin of GJ-35 read straight off a part's
/// counts, the plurality count less the largest other
/// count, standing only where a plurality stands.
fn direct_margin(counts: &[(String, usize)], valid: usize, union: &[String]) -> Option<usize> {
    let plurality = direct_plurality(counts, valid)?;
    let own = counts
        .iter()
        .find(|(answer, _)| answer == &plurality)
        .map(|(_, count)| *count)
        .unwrap_or(0);
    let mut largest_other = 0;
    for answer in union {
        if answer != &plurality {
            let count = counts
                .iter()
                .find(|(entry, _)| entry == answer)
                .map(|(_, count)| *count)
                .unwrap_or(0);
            largest_other = largest_other.max(count);
        }
    }
    Some(own - largest_other)
}

/// The questions of the record in first appearance
/// order, as GJ-29 states them, read straight off the
/// polls.
fn questions_of(sets: &[SetPolls]) -> Vec<String> {
    let mut questions: Vec<String> = Vec::new();
    for (_, polls) in sets {
        for poll in polls {
            let question = poll.question.display();
            if !questions.contains(&question) {
                questions.push(question);
            }
        }
    }
    questions
}

/// The askings of a question, as (set index, poll).
fn askings_of<'a>(sets: &'a [SetPolls], question: &str) -> Vec<(usize, &'a Poll)> {
    let mut askings = Vec::new();
    for (index, (_, polls)) in sets.iter().enumerate() {
        for poll in polls {
            if poll.question.display() == question {
                askings.push((index, poll));
            }
        }
    }
    askings
}

/// One part counted directly, as (per answer counts
/// over the union of the part, the four counts of the
/// part, the union of the part, plurality).
type DirectPart = (Vec<(String, usize)>, Counts4, Vec<String>, Option<String>);

/// One part counted straight off the responses, the
/// GJ-37 part. The union gathers the offered answers of
/// the part's askings in first appearance order, and the
/// counts sum the valid responses naming each answer of
/// the union over the part's askings, under the GJ-8
/// rule that an abstention and an answer outside the
/// poll's own offered answers stand outside the valid.
fn direct_part(sets: &[SetPolls], set_index: usize, question: &str) -> DirectPart {
    let polls: Vec<&Poll> = sets[set_index]
        .1
        .iter()
        .filter(|poll| poll.question.display() == question)
        .collect();
    let mut union: Vec<String> = Vec::new();
    for poll in &polls {
        for answer in &poll.answers {
            let text = answer.display();
            if !union.contains(&text) {
                union.push(text);
            }
        }
    }
    let mut counts: Vec<(String, usize)> = union.iter().map(|answer| (answer.clone(), 0)).collect();
    let (mut recorded, mut valid, mut abstained, mut spoiled) = (0, 0, 0, 0);
    for poll in &polls {
        let offered: Vec<String> = poll.answers.iter().map(|answer| answer.display()).collect();
        for response in &poll.responses {
            recorded += 1;
            if response.abstained {
                abstained += 1;
            } else {
                let answer = response.answer.display();
                if offered.contains(&answer) {
                    valid += 1;
                    if let Some(entry) = counts.iter_mut().find(|(name, _)| name == &answer) {
                        entry.1 += 1;
                    }
                } else {
                    spoiled += 1;
                }
            }
        }
    }
    let plurality = direct_plurality(&counts, valid);
    (
        counts,
        Counts4(recorded, valid, abstained, spoiled),
        union,
        plurality,
    )
}

/// The accord of a question under GJ-37 read straight
/// off the parts, the one answer every part plurality
/// names, standing only where the parts that hold a
/// plurality name one answer between them.
fn direct_accord(sets: &[SetPolls], question: &str) -> Option<String> {
    let mut named: Vec<String> = Vec::new();
    for (index, _) in askings_of(sets, question) {
        let (_, _, _, plurality) = direct_part(sets, index, question);
        if let Some(answer) = plurality {
            if !named.contains(&answer) {
                named.push(answer);
            }
        }
    }
    if named.len() == 1 {
        named.into_iter().next()
    } else {
        None
    }
}

/// The pooled plurality of a question under the
/// aggregate reading, GJ-28, read straight off the
/// responses of every asking of the question pooled as
/// one pool.
fn direct_pooled_plurality(sets: &[SetPolls], question: &str) -> Option<String> {
    let askings = askings_of(sets, question);
    let mut union: Vec<String> = Vec::new();
    for (_, poll) in &askings {
        for answer in &poll.answers {
            let text = answer.display();
            if !union.contains(&text) {
                union.push(text);
            }
        }
    }
    let mut counts: Vec<(String, usize)> = union.iter().map(|answer| (answer.clone(), 0)).collect();
    let mut valid = 0;
    for (_, poll) in &askings {
        let offered: Vec<String> = poll.answers.iter().map(|answer| answer.display()).collect();
        for response in &poll.responses {
            if response.abstained {
                continue;
            }
            let answer = response.answer.display();
            if offered.contains(&answer) {
                valid += 1;
                if let Some(entry) = counts.iter_mut().find(|(name, _)| name == &answer) {
                    entry.1 += 1;
                }
            }
        }
    }
    direct_plurality(&counts, valid)
}

/// The counts of one set over the whole record, summed
/// over every poll of the set, read straight off the
/// responses under the GJ-8 rule.
fn direct_set_totals(sets: &[SetPolls], set_index: usize) -> Counts4 {
    let (mut recorded, mut valid, mut abstained, mut spoiled) = (0, 0, 0, 0);
    for poll in &sets[set_index].1 {
        let offered: Vec<String> = poll.answers.iter().map(|answer| answer.display()).collect();
        for response in &poll.responses {
            recorded += 1;
            if response.abstained {
                abstained += 1;
            } else if offered.contains(&response.answer.display()) {
                valid += 1;
            } else {
                spoiled += 1;
            }
        }
    }
    Counts4(recorded, valid, abstained, spoiled)
}

/// The whole standing reading of GJ-40 transcribed
/// directly, section by section, part by part, with its
/// standing lines and its closing.
fn direct_standing_lines(sets: &[SetPolls]) -> Vec<String> {
    let mut lines = Vec::new();
    let (mut parts_total, mut plu_total, mut tied_total, mut empty_total) = (0, 0, 0, 0);
    let (mut accord_total, mut pool_total) = (0, 0);
    for (set_index, (label, _)) in sets.iter().enumerate() {
        lines.push(format!(
            "set, {}, {}",
            label.display(),
            counts_text(direct_set_totals(sets, set_index))
        ));
        let (mut questions, mut held, mut tied, mut empty) = (0, 0, 0, 0);
        let (mut in_accord, mut with_pool) = (0, 0);
        for question in questions_of(sets) {
            let asked = sets[set_index]
                .1
                .iter()
                .any(|poll| poll.question.display() == question);
            if !asked {
                continue;
            }
            let (counts, totals, union, plurality) = direct_part(sets, set_index, &question);
            let margin = direct_margin(&counts, totals.1, &union);
            lines.push(format!("question, {}", question));
            lines.push(format!(
                "plurality, {}",
                plurality.as_deref().unwrap_or("none")
            ));
            lines.push(match margin {
                Some(figure) => format!("margin, {}", figure),
                None => "margin, not counted".to_string(),
            });
            questions += 1;
            match &plurality {
                Some(answer) => {
                    held += 1;
                    if direct_accord(sets, &question).as_deref() == Some(answer.as_str()) {
                        in_accord += 1;
                    }
                    if direct_pooled_plurality(sets, &question).as_deref() == Some(answer.as_str())
                    {
                        with_pool += 1;
                    }
                }
                None => {
                    if totals.1 > 0 {
                        tied += 1;
                    } else {
                        empty += 1;
                    }
                }
            }
        }
        lines.push(format!(
            "standing, {} questions, {} with a plurality, {} tied, {} with no valid response, {} in accord, {} with the pool",
            questions, held, tied, empty, in_accord, with_pool
        ));
        parts_total += questions;
        plu_total += held;
        tied_total += tied;
        empty_total += empty;
        accord_total += in_accord;
        pool_total += with_pool;
    }
    lines.push(format!(
        "standings, {} sets, {} parts, {} with a plurality, {} tied, {} with no valid response, {} in accord, {} with the pool",
        sets.len(),
        parts_total,
        plu_total,
        tied_total,
        empty_total,
        accord_total,
        pool_total
    ));
    lines
}

/// A parsed standing section, its label, its parts,
/// and its standing figures in the stated order,
/// questions, with a plurality, tied, with no valid
/// response, in accord, with the pool.
struct StandingSection {
    label: String,
    parts: Vec<(String, String, String)>,
    figures: (usize, usize, usize, usize, usize, usize),
}

fn parse_standing(lines: &[String]) -> (Vec<StandingSection>, String) {
    let mut sections: Vec<StandingSection> = Vec::new();
    let mut current: Option<StandingSection> = None;
    let mut closing = String::new();
    for line in lines {
        if let Some(rest) = line.strip_prefix("standings, ") {
            closing = format!("standings, {}", rest);
        } else if let Some(rest) = line.strip_prefix("set, ") {
            if let Some(done) = current.take() {
                sections.push(done);
            }
            let label = rest.split(", ").next().expect("a set label").to_string();
            current = Some(StandingSection {
                label,
                parts: Vec::new(),
                figures: (0, 0, 0, 0, 0, 0),
            });
        } else if let Some(rest) = line.strip_prefix("question, ") {
            current.as_mut().expect("a section stands").parts.push((
                rest.to_string(),
                String::new(),
                String::new(),
            ));
        } else if let Some(rest) = line.strip_prefix("plurality, ") {
            let section = current.as_mut().expect("a section stands");
            section.parts.last_mut().expect("a part stands").1 = rest.to_string();
        } else if let Some(rest) = line.strip_prefix("margin, ") {
            let section = current.as_mut().expect("a section stands");
            section.parts.last_mut().expect("a part stands").2 = rest.to_string();
        } else if line.starts_with("standing, ") {
            let section = current.as_mut().expect("a section stands");
            section.figures = (
                figure_before(line, "questions"),
                figure_before(line, "with a plurality"),
                figure_before(line, "tied"),
                figure_before(line, "with no valid response"),
                figure_before(line, "in accord"),
                figure_before(line, "with the pool"),
            );
        }
    }
    if let Some(done) = current.take() {
        sections.push(done);
    }
    (sections, closing)
}

/// One part of an accord reading as (set label,
/// plurality value, margin value), parsed from rendered
/// lines.
type AccordPartLines = (String, String, String);

/// One section of an accord reading as (question,
/// parts), parsed from rendered lines.
type AccordSectionLines = (String, Vec<AccordPartLines>);

/// A parsed accord section, its question, and its parts
/// by set label, each part's plurality and margin lines.
fn parse_accord(lines: &[String]) -> Vec<AccordSectionLines> {
    let mut sections: Vec<AccordSectionLines> = Vec::new();
    for line in lines {
        if let Some(rest) = line.strip_prefix("question, ") {
            sections.push((rest.to_string(), Vec::new()));
        } else if line.starts_with("set, ") {
            let label = line.split(", ").nth(1).expect("a set label").to_string();
            sections.last_mut().expect("a section stands").1.push((
                label,
                String::new(),
                String::new(),
            ));
        } else if let Some(rest) = line.strip_prefix("plurality, ") {
            let section = sections.last_mut().expect("a section stands");
            section.1.last_mut().expect("a part stands").1 = rest.to_string();
        } else if let Some(rest) = line.strip_prefix("margin, ") {
            let section = sections.last_mut().expect("a section stands");
            section.1.last_mut().expect("a part stands").2 = rest.to_string();
        }
    }
    sections
}

/// The closing line of a reading.
fn closing_of(lines: &[String]) -> String {
    lines.last().expect("a closing line").clone()
}

// ------------------------------------------------------------------
// The standing reading
// ------------------------------------------------------------------

#[test]
fn the_standing_golden_stands_byte_for_byte() {
    // The golden is the shipped record's standing
    // reading, 199 lines, closing on three sets, 64
    // parts, 55 with a plurality, none tied, 9 with no
    // valid response, 55 in accord, and 55 with the pool.
    let golden = read_text(&examples().join("standing-golden.txt"));
    let lines: Vec<String> = golden.lines().map(|line| line.to_string()).collect();
    assert_eq!(lines.len(), 199);
    assert_eq!(
        closing_of(&lines),
        "standings, 3 sets, 64 parts, 55 with a plurality, 0 tied, 9 with no valid response, 55 in accord, 55 with the pool"
    );
    let (stdout, _, code) = run_cli(&[
        "standing",
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
fn the_standing_reading_computes_directly_from_the_record() {
    // The library reading over the shipped record equals
    // a direct transcription of GJ-40, section by
    // section, part by part, with its standing lines and
    // its closing, and the command prints the same lines.
    let sets = recorded_sets();
    let lines = standing_lines(&sets).expect("a standing reading");
    assert_eq!(lines, direct_standing_lines(&sets));
    let (stdout, _, code) = run_cli(&[
        "standing",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "full",
        &path_arg(&examples().join("full")),
        "wave",
        &path_arg(&examples().join("wave")),
    ]);
    assert_eq!(code, 0);
    let printed: Vec<String> = stdout.lines().map(|line| line.to_string()).collect();
    assert_eq!(printed, lines);
}

#[test]
fn the_standing_parts_stand_equal_to_the_accord_parts() {
    // The regrouping relation of GJ-40. Every part of the
    // standing reading states the plurality line and the
    // margin line of the same part in the accord reading,
    // the part of the same set at the same question. The
    // parts stand counted at 64 before the loop, so an
    // empty comparison proves nothing.
    let sets = recorded_sets();
    let accord = accord_lines(&sets).expect("an accord reading");
    let accord_sections = parse_accord(&accord);
    let standing = standing_lines(&sets).expect("a standing reading");
    let (sections, _) = parse_standing(&standing);
    let total: usize = sections.iter().map(|section| section.parts.len()).sum();
    assert_eq!(total, 64);
    for section in &sections {
        for (question, plurality, margin) in &section.parts {
            let accord_section = accord_sections
                .iter()
                .find(|(name, _)| name == question)
                .expect("the question stands in the accord reading");
            let accord_part = accord_section
                .1
                .iter()
                .find(|(label, _, _)| label == &section.label)
                .expect("the part stands in the accord reading");
            assert_eq!(plurality, &accord_part.1, "{}", question);
            assert_eq!(margin, &accord_part.2, "{}", question);
        }
    }
}

/// The sums of GJ-40 over one record. In every section
/// the questions figure stands as the count of parts
/// with a plurality, tied, and with no valid response
/// summed, and every figure of the closing stands as
/// the sum of its sections' figures.
fn assert_standing_sums(lines: &[String]) {
    let (sections, closing) = parse_standing(lines);
    let (mut parts, mut held, mut tied, mut empty) = (0, 0, 0, 0);
    let (mut in_accord, mut with_pool) = (0, 0);
    for section in &sections {
        let (q, wp, td, em, ia, wpool) = section.figures;
        assert_eq!(section.parts.len(), q, "{}", section.label);
        assert_eq!(q, wp + td + em, "{}", section.label);
        parts += q;
        held += wp;
        tied += td;
        empty += em;
        in_accord += ia;
        with_pool += wpool;
    }
    assert_eq!(figure_before(&closing, "sets"), sections.len());
    assert_eq!(figure_before(&closing, "parts"), parts);
    assert_eq!(figure_before(&closing, "with a plurality"), held);
    assert_eq!(figure_before(&closing, "tied"), tied);
    assert_eq!(figure_before(&closing, "with no valid response"), empty);
    assert_eq!(figure_before(&closing, "in accord"), in_accord);
    assert_eq!(figure_before(&closing, "with the pool"), with_pool);
}

#[test]
fn the_standing_sections_and_closing_sum_as_stated() {
    // The sums hold over the shipped sections and over
    // the generated sections both. The generated record
    // is where the sums bite, its sections tying at
    // three, three, two, and one, where the shipped
    // sections tie nowhere.
    let sets = recorded_sets();
    let lines = standing_lines(&sets).expect("a standing reading");
    assert_standing_sums(&lines);
    let (loaded, _) = generated_sets_loaded();
    let generated = standing_lines(&loaded).expect("a standing reading");
    assert_standing_sums(&generated);
}

#[test]
fn the_standing_closing_reconciles_with_the_accord_and_aggregate_closings() {
    // The parts figure of the standing closing stands
    // equal to the parts figure of the accord closing
    // and to the askings figure of the aggregate closing
    // over the same record, each part standing over one
    // asking of the aggregate's counting and one part of
    // the accord's.
    let sets = recorded_sets();
    let standing = standing_lines(&sets).expect("a standing reading");
    let accord = accord_lines(&sets).expect("an accord reading");
    let aggregate = aggregate_lines(&sets).expect("an aggregate reading");
    let parts = figure_before(&closing_of(&standing), "parts");
    assert_eq!(parts, 64);
    assert_eq!(parts, figure_before(&closing_of(&accord), "parts"));
    assert_eq!(parts, figure_before(&closing_of(&aggregate), "askings"));
}

#[test]
fn two_sets_naming_the_same_plurality_both_stand_in_accord() {
    // Shape S1 of GJ-40, the same-plurality precedent of
    // GJ-38 restated at the set grain. Two sets ask one
    // question and both name red, alpha at three of four
    // and beta at two of three. The accord of the
    // question is red, and each set's part names it, so
    // the standing closing counts two parts in accord
    // where the accord closing counts one question. The
    // in accord figure counts parts, and this pin is
    // where a figure counted by questions fails, standing
    // at one against the stated two. The shipped record
    // stands beside the shape, its standing closing
    // counting 55 parts in accord against the accord
    // closing's 44 questions in accord.
    let question = "Which gate should the fair open first?";
    let alpha = vec![poll(
        question,
        &["the red gate", "the blue gate"],
        [
            votes_of("ada", "the red gate", 3),
            votes_of("ada", "the blue gate", 1),
        ]
        .concat(),
    )];
    let beta = vec![poll(
        question,
        &["the red gate", "the blue gate"],
        [
            votes_of("ada", "the red gate", 2),
            votes_of("ada", "the blue gate", 1),
        ]
        .concat(),
    )];
    let sets: Vec<SetPolls> = vec![(pt("alpha"), alpha), (pt("beta"), beta)];
    let lines = standing_lines(&sets).expect("a standing reading");
    assert_eq!(
        closing_of(&lines),
        "standings, 2 sets, 2 parts, 2 with a plurality, 0 tied, 0 with no valid response, 2 in accord, 2 with the pool"
    );
    let accord = accord_lines(&sets).expect("an accord reading");
    assert_eq!(
        closing_of(&accord),
        "accords, 1 questions, 2 parts, 1 in accord, 0 divided"
    );
    let recorded = recorded_sets();
    let shipped = standing_lines(&recorded).expect("a standing reading");
    assert_eq!(figure_before(&closing_of(&shipped), "in accord"), 55);
    let shipped_accord = accord_lines(&recorded).expect("an accord reading");
    assert_eq!(figure_before(&closing_of(&shipped_accord), "in accord"), 44);
}

#[test]
fn a_set_tied_at_the_top_counts_tied() {
    // Shape S2 of GJ-40, the tie path of the standing
    // reading, carried as a stated closing figure at the
    // set grain. One set asks one question and its valid
    // responses name red and blue at two each, so the
    // part's top counts tie, its plurality line stands
    // at none and its margin at not counted, and the
    // standing line counts the part as tied, neither with
    // a plurality nor with no valid response. A reading
    // that counted a tied part with a plurality fails
    // here at the stated one tied, and a reading that
    // stood a plurality at the tied top count fails at
    // the plurality line itself.
    let question = "Which gate should the fair open first?";
    let alpha = vec![poll(
        question,
        &["the red gate", "the blue gate"],
        [
            votes_of("ada", "the red gate", 2),
            votes_of("ada", "the blue gate", 2),
        ]
        .concat(),
    )];
    let sets: Vec<SetPolls> = vec![(pt("alpha"), alpha)];
    let lines = standing_lines(&sets).expect("a standing reading");
    assert_eq!(
        lines,
        vec![
            "set, alpha, recorded 4, valid 4, abstained 0, spoiled 0",
            "question, Which gate should the fair open first?",
            "plurality, none",
            "margin, not counted",
            "standing, 1 questions, 0 with a plurality, 1 tied, 0 with no valid response, 0 in accord, 0 with the pool",
            "standings, 1 sets, 1 parts, 0 with a plurality, 1 tied, 0 with no valid response, 0 in accord, 0 with the pool",
        ]
    );
}

#[test]
fn a_set_with_no_valid_response_counts_with_no_valid_response() {
    // Shape S3 of GJ-40, the no-valid-response precedent
    // of GJ-38 restated. One set asks one question and
    // every response abstains, so the part stands with a
    // valid count of zero, its plurality line at none and
    // its margin at not counted, and the standing line
    // counts it with no valid response. The part stands
    // in the section, and a reading that omitted a part
    // at a valid count of zero fails here at the stated
    // figures, the parts figure standing at one.
    let question = "Which gate should the fair open first?";
    let alpha = vec![poll(
        question,
        &["the red gate", "the blue gate"],
        vec![
            response("ada", "model a", None, "no answer recorded"),
            response("ada", "model a", None, "no answer recorded"),
            response("ben", "model a", None, "no answer recorded"),
        ],
    )];
    let sets: Vec<SetPolls> = vec![(pt("alpha"), alpha)];
    let lines = standing_lines(&sets).expect("a standing reading");
    assert_eq!(
        lines,
        vec![
            "set, alpha, recorded 3, valid 0, abstained 3, spoiled 0",
            "question, Which gate should the fair open first?",
            "plurality, none",
            "margin, not counted",
            "standing, 1 questions, 0 with a plurality, 0 tied, 1 with no valid response, 0 in accord, 0 with the pool",
            "standings, 1 sets, 1 parts, 0 with a plurality, 0 tied, 1 with no valid response, 0 in accord, 0 with the pool",
        ]
    );
}

#[test]
fn a_part_can_stand_with_the_pool_and_not_in_accord() {
    // Shape S4 of GJ-40, the divided precedent of GJ-38
    // restated. Alpha names red at three of four and beta
    // names blue at two of three, so the question stands
    // divided and no part stands in accord, while the
    // pooled counts name red at four against three, so
    // alpha's part stands with the pool and beta's does
    // not. The two standings are read off different
    // pluralities, and a reading that swapped them fails
    // here, alpha standing at one with the pool and none
    // in accord.
    let question = "Which gate should the fair open first?";
    let alpha = vec![poll(
        question,
        &["the red gate", "the blue gate"],
        [
            votes_of("ada", "the red gate", 3),
            votes_of("ada", "the blue gate", 1),
        ]
        .concat(),
    )];
    let beta = vec![poll(
        question,
        &["the red gate", "the blue gate"],
        [
            votes_of("ada", "the blue gate", 2),
            votes_of("ada", "the red gate", 1),
        ]
        .concat(),
    )];
    let sets: Vec<SetPolls> = vec![(pt("alpha"), alpha), (pt("beta"), beta)];
    let lines = standing_lines(&sets).expect("a standing reading");
    assert_eq!(
        lines,
        vec![
            "set, alpha, recorded 4, valid 4, abstained 0, spoiled 0",
            "question, Which gate should the fair open first?",
            "plurality, the red gate",
            "margin, 2",
            "standing, 1 questions, 1 with a plurality, 0 tied, 0 with no valid response, 0 in accord, 1 with the pool",
            "set, beta, recorded 3, valid 3, abstained 0, spoiled 0",
            "question, Which gate should the fair open first?",
            "plurality, the blue gate",
            "margin, 1",
            "standing, 1 questions, 1 with a plurality, 0 tied, 0 with no valid response, 0 in accord, 0 with the pool",
            "standings, 2 sets, 2 parts, 2 with a plurality, 0 tied, 0 with no valid response, 0 in accord, 1 with the pool",
        ]
    );
}

#[test]
fn a_part_can_stand_in_accord_and_not_with_the_pool() {
    // Shape S5 of GJ-40, the pooled-falls precedent of
    // GJ-38 restated. Alpha names red at two of three
    // over the three offered answers and beta names none,
    // its blue and green counts tying at two each with
    // one red standing, so the accord of the question is
    // red and alpha's part stands in accord. The pooled
    // counts tie red and blue at three each, so the
    // pooled plurality stands at none and no part stands
    // with the pool. A reading that swapped the two
    // standings fails here from the other side, alpha
    // standing at one in accord and none with the pool,
    // and beta's part counts tied.
    let question = "Which gate should the fair open first?";
    let answers = &["the red gate", "the blue gate", "the green gate"];
    let alpha = vec![poll(
        question,
        answers,
        [
            votes_of("ada", "the red gate", 2),
            votes_of("ada", "the blue gate", 1),
        ]
        .concat(),
    )];
    let beta = vec![poll(
        question,
        answers,
        [
            votes_of("ada", "the blue gate", 2),
            votes_of("ada", "the green gate", 2),
            votes_of("ada", "the red gate", 1),
        ]
        .concat(),
    )];
    let sets: Vec<SetPolls> = vec![(pt("alpha"), alpha), (pt("beta"), beta)];
    let lines = standing_lines(&sets).expect("a standing reading");
    assert_eq!(
        lines,
        vec![
            "set, alpha, recorded 3, valid 3, abstained 0, spoiled 0",
            "question, Which gate should the fair open first?",
            "plurality, the red gate",
            "margin, 1",
            "standing, 1 questions, 1 with a plurality, 0 tied, 0 with no valid response, 1 in accord, 0 with the pool",
            "set, beta, recorded 5, valid 5, abstained 0, spoiled 0",
            "question, Which gate should the fair open first?",
            "plurality, none",
            "margin, not counted",
            "standing, 1 questions, 0 with a plurality, 1 tied, 0 with no valid response, 0 in accord, 0 with the pool",
            "standings, 2 sets, 2 parts, 1 with a plurality, 1 tied, 0 with no valid response, 1 in accord, 0 with the pool",
        ]
    );
}

#[test]
fn a_set_states_parts_for_its_questions_alone() {
    // Shape S6 of GJ-40. Alpha asks two questions and
    // beta asks the first alone, so beta's section states
    // one part and its standing line counts one question,
    // where a reading that counted every question of the
    // record in every section fails at the stated one. A
    // set holding no polls stands as a section at zero
    // counts and zero figures, its questions figure
    // standing at zero against the same fault.
    let gates = "Which gate should the fair open first?";
    let fields = "Which field should the fair mow first?";
    let alpha = vec![
        poll(
            gates,
            &["the red gate", "the blue gate"],
            [
                votes_of("ada", "the red gate", 2),
                votes_of("ada", "the blue gate", 1),
            ]
            .concat(),
        ),
        poll(
            fields,
            &["the north field", "the south field"],
            [
                votes_of("ada", "the north field", 2),
                votes_of("ada", "the south field", 1),
            ]
            .concat(),
        ),
    ];
    let beta = vec![poll(
        gates,
        &["the red gate", "the blue gate"],
        [
            votes_of("ada", "the blue gate", 2),
            votes_of("ada", "the red gate", 1),
        ]
        .concat(),
    )];
    let sets: Vec<SetPolls> = vec![
        (pt("alpha"), alpha),
        (pt("beta"), beta),
        (pt("empty"), vec![]),
    ];
    let lines = standing_lines(&sets).expect("a standing reading");
    assert_eq!(
        lines,
        vec![
            "set, alpha, recorded 6, valid 6, abstained 0, spoiled 0",
            "question, Which gate should the fair open first?",
            "plurality, the red gate",
            "margin, 1",
            "question, Which field should the fair mow first?",
            "plurality, the north field",
            "margin, 1",
            "standing, 2 questions, 2 with a plurality, 0 tied, 0 with no valid response, 1 in accord, 1 with the pool",
            "set, beta, recorded 3, valid 3, abstained 0, spoiled 0",
            "question, Which gate should the fair open first?",
            "plurality, the blue gate",
            "margin, 1",
            "standing, 1 questions, 1 with a plurality, 0 tied, 0 with no valid response, 0 in accord, 0 with the pool",
            "set, empty, recorded 0, valid 0, abstained 0, spoiled 0",
            "standing, 0 questions, 0 with a plurality, 0 tied, 0 with no valid response, 0 in accord, 0 with the pool",
            "standings, 3 sets, 3 parts, 3 with a plurality, 0 tied, 0 with no valid response, 1 in accord, 1 with the pool",
        ]
    );
}

#[test]
fn an_answer_offered_by_one_set_alone_sets_its_parts_plurality() {
    // Shape S7 of GJ-40, the union precedent of GJ-38
    // restated, and the one catch of a union gathered
    // from the first set's askings alone. Alpha asks
    // which field over the north and south fields and
    // names the south field at two against one. Beta asks
    // the same question over the north and east fields
    // and names the east field at three against one. The
    // east field stands in beta's union alone, so beta's
    // part states the east field as its plurality, where
    // a union gathered from alpha's askings drops the
    // east field's count and the part states none.
    let question = "Which field should the fair mow first?";
    let alpha = vec![poll(
        question,
        &["the north field", "the south field"],
        [
            votes_of("ada", "the south field", 2),
            votes_of("ada", "the north field", 1),
        ]
        .concat(),
    )];
    let beta = vec![poll(
        question,
        &["the north field", "the east field"],
        [
            votes_of("ada", "the east field", 3),
            votes_of("ada", "the north field", 1),
        ]
        .concat(),
    )];
    let sets: Vec<SetPolls> = vec![(pt("alpha"), alpha), (pt("beta"), beta)];
    let lines = standing_lines(&sets).expect("a standing reading");
    assert_eq!(
        lines,
        vec![
            "set, alpha, recorded 3, valid 3, abstained 0, spoiled 0",
            "question, Which field should the fair mow first?",
            "plurality, the south field",
            "margin, 1",
            "standing, 1 questions, 1 with a plurality, 0 tied, 0 with no valid response, 0 in accord, 0 with the pool",
            "set, beta, recorded 4, valid 4, abstained 0, spoiled 0",
            "question, Which field should the fair mow first?",
            "plurality, the east field",
            "margin, 2",
            "standing, 1 questions, 1 with a plurality, 0 tied, 0 with no valid response, 0 in accord, 1 with the pool",
            "standings, 2 sets, 2 parts, 2 with a plurality, 0 tied, 0 with no valid response, 0 in accord, 1 with the pool",
        ]
    );
}

#[test]
fn the_standing_guard_reads_the_model_of_the_second_set() {
    // The guard of GJ-39 stands over every set before
    // the set rules. The second set holds a poll whose
    // first response model carries a line break and a
    // forged standings line after it, the model drawn at
    // response zero of the set's second poll, a model
    // string alone, never an answer and never a reason.
    // A census reading over the same record draws the
    // same defect, the position the guard's ground
    // states, and the standing reading draws it in the
    // guard's own form. A guard read after the set rules
    // does not move this fixture, no set rule defect
    // standing in it, and a guard read over the first
    // section's set only draws no defect here at all, the
    // reading standing where this pin requires the
    // refusal.
    let forged = "model a\nstandings, forged";
    let first = vec![poll(
        "Which gate should the fair open first?",
        &["the red gate", "the blue gate"],
        votes_of("ada", "the red gate", 1),
    )];
    let second = vec![
        poll(
            "Which gate should the fair open first?",
            &["the red gate", "the blue gate"],
            votes_of("ada", "the red gate", 1),
        ),
        poll(
            "Which field should the fair mow first?",
            &["the north field", "the south field"],
            vec![response(
                "ada",
                forged,
                Some("the north field"),
                "a stated reason",
            )],
        ),
    ];
    let sets: Vec<SetPolls> = vec![(pt("alpha"), first), (pt("beta"), second)];
    assert_eq!(
        standing_lines(&sets),
        Err("response 0 field model holds a newline".to_string())
    );
    assert_eq!(
        grandjury::census::census_lines(&sets),
        Err("response 0 field model holds a newline".to_string())
    );
}

#[test]
fn the_standing_guard_reads_the_question_of_the_first_set() {
    // The second guard fixture stands at the other edge
    // of the sweep. The first set holds a poll whose
    // question string carries a line break and a forged
    // standings line after it, a question string alone,
    // and a census reading over the same record draws
    // the same defect. The pin stands under every guard
    // order and sweep variant the round names, the set
    // rules refusing the same fixture one step later
    // where the guard is not read first, and it
    // discriminates no variant among them, stated
    // plainly rather than claimed.
    let forged = "Which gate should the fair open first?\nstandings, forged";
    let first = vec![poll(
        forged,
        &["the red gate", "the blue gate"],
        votes_of("ada", "the red gate", 1),
    )];
    let second = vec![poll(
        "Which field should the fair mow first?",
        &["the north field", "the south field"],
        votes_of("ada", "the north field", 1),
    )];
    let sets: Vec<SetPolls> = vec![(pt("alpha"), first), (pt("beta"), second)];
    assert_eq!(
        standing_lines(&sets),
        Err("field question holds a newline".to_string())
    );
    assert_eq!(
        grandjury::census::census_lines(&sets),
        Err("field question holds a newline".to_string())
    );
}

#[test]
fn the_standing_set_rules_refuse_as_the_census_refuses() {
    // The set rules of GJ-11 stand unchanged, and the
    // standing reading draws them in the census form. An
    // empty label and a duplicate label refuse in the
    // stated words, and a record carrying both a
    // duplicate label and a set holding a question twice
    // draws the duplicate label defect, the label rules
    // standing ahead of the per set question loop in the
    // rules as they stand. No order variant moving the
    // duplicate check after the empty check is owed a
    // pin, the two refusals being unobservable apart, no
    // label standing both empty and twice in a record the
    // rules can state.
    let one = vec![poll(
        "Which gate should the fair open first?",
        &["the red gate", "the blue gate"],
        votes_of("ada", "the red gate", 1),
    )];
    let empty: Vec<SetPolls> = vec![(pt(""), one.clone())];
    assert_eq!(
        standing_lines(&empty),
        Err("a set label is empty".to_string())
    );
    assert_eq!(
        grandjury::census::census_lines(&empty),
        Err("a set label is empty".to_string())
    );
    let twice: Vec<SetPolls> = vec![(pt("alpha"), one.clone()), (pt("alpha"), one.clone())];
    assert_eq!(
        standing_lines(&twice),
        Err("the set label alpha stands twice".to_string())
    );
    let repeated = vec![
        poll(
            "Which gate should the fair open first?",
            &["the red gate", "the blue gate"],
            votes_of("ada", "the red gate", 1),
        ),
        poll(
            "Which gate should the fair open first?",
            &["the red gate", "the blue gate"],
            votes_of("ada", "the blue gate", 1),
        ),
    ];
    let mixed: Vec<SetPolls> = vec![(pt("alpha"), one), (pt("alpha"), repeated)];
    assert_eq!(
        standing_lines(&mixed),
        Err("the set label alpha stands twice".to_string())
    );
}

#[test]
fn no_verdict_word_stands_in_the_standing_reading() {
    // The vocabulary of GJ-39. The reading states
    // standings and never verdicts. Over the shipped
    // golden and over the command's own answers, no word
    // of the reading, read word by word, is one of the
    // fifteen refused words, a set label stands in one
    // position alone, and the words standing and
    // standings stand outside the verdict list word by
    // word, the closing word standings standing in the
    // golden so the check is not vacuous.
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
    let golden = read_text(&examples().join("standing-golden.txt"));
    let (stdout, _, code) = run_cli(&[
        "standing",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "full",
        &path_arg(&examples().join("full")),
        "wave",
        &path_arg(&examples().join("wave")),
    ]);
    assert_eq!(code, 0);
    for text in [&golden, &stdout] {
        let words: Vec<String> = text
            .to_lowercase()
            .split(|c: char| !c.is_ascii_lowercase())
            .filter(|word| !word.is_empty())
            .map(|word| word.to_string())
            .collect();
        for word in VERDICT_WORDS {
            assert!(!words.contains(&word.to_string()), "{}", word);
        }
    }
    assert!(!VERDICT_WORDS.contains(&"standing"));
    assert!(!VERDICT_WORDS.contains(&"standings"));
    let golden_words: Vec<String> = golden
        .to_lowercase()
        .split(|c: char| !c.is_ascii_lowercase())
        .filter(|word| !word.is_empty())
        .map(|word| word.to_string())
        .collect();
    assert!(golden_words.contains(&"standings".to_string()));
    let mut set_lines = 0;
    for text in [&golden, &stdout] {
        for line in text.lines() {
            if line.starts_with("set, ") {
                set_lines += 1;
                let label = line.split(", ").nth(1).expect("a set label");
                assert!(["pilot", "full", "wave"].contains(&label));
            } else {
                assert!(!line.contains("pilot"), "{}", line);
                assert!(!line.contains("wave"), "{}", line);
            }
        }
    }
    assert_eq!(set_lines, 6);
}

#[test]
fn the_standing_help_names_the_command() {
    // The usage text of GJ-40. The command stands in the
    // top level help with its own line, and its own help
    // names its argument shape. The top level assertions
    // are surgical, the command's description line and
    // the subcommand usage line alone, so a help text
    // that merely omits the command fails here without
    // the pin claiming the whole help surface.
    let (stdout, _, code) = run_cli(&["standing", "--help"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("usage: grandjury standing [-h] label directory [label directory ...]"));
    let (stdout, _, code) = run_cli(&["--help"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("print the standing reading over recorded poll sets"));
}

// The generated fixture family of GJ-27, restated in
// this target as GJ-40 states: polls drawn in the
// test from the stated distribution at the stated
// seed, loaded through the crate's own loader, and
// pinned by the stated figures of GJ-40, by an
// independent recount inside the test, and by the
// relations of GJ-40. No golden ships for the family,
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
fn generated_sets_loaded() -> (Vec<(PText, Vec<Poll>)>, TempDir) {
    let in_memory = generated_sets_in_memory();
    let scratch = TempDir::new("standing-generated");
    let mut loaded: Vec<(PText, Vec<Poll>)> = Vec::new();
    for (index, (label, polls)) in in_memory.iter().enumerate() {
        let dir = scratch.path.join(format!("set-{}", index));
        std::fs::create_dir_all(&dir).expect("a scratch set directory");
        for (i, held) in polls.iter().enumerate() {
            std::fs::write(
                dir.join(format!("poll-{:02}.json", i + 1)),
                dumps_default(&held.to_json()),
            )
            .expect("a written poll file");
        }
        loaded.push((label.clone(), load_set(&dir, polls.len())));
    }
    (loaded, scratch)
}

#[test]
fn the_generated_family_holds_its_stated_figures() {
    // The identity figures of the family, stated in
    // GJ-27 as equalities at the seed and counted here
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
    // The standing figures of GJ-40 over the generated
    // record, each computed here from the direct
    // transcription over the record and stated as an
    // equality at the seed: the closing at four sets,
    // 72 parts, 62 with a plurality, nine tied, one with
    // no valid response, 43 in accord, and 53 with the
    // pool, and the four standing lines in set order.
    // The three standings stand apart on this record,
    // 62 and 43 and 53, where the shipped record holds
    // them equal by coincidence.
    let lines = direct_standing_lines(&in_memory);
    let (sections, closing) = parse_standing(&lines);
    assert_eq!(
        closing,
        "standings, 4 sets, 72 parts, 62 with a plurality, 9 tied, 1 with no valid response, 43 in accord, 53 with the pool",
        "the closing of the generated standing at the seed"
    );
    let stated: [(usize, usize, usize, usize, usize, usize); 4] = [
        (19, 16, 3, 0, 10, 13),
        (20, 17, 3, 0, 11, 13),
        (14, 12, 2, 0, 10, 12),
        (19, 17, 1, 1, 12, 15),
    ];
    assert_eq!(sections.len(), 4);
    for (section, figures) in sections.iter().zip(stated.iter()) {
        assert_eq!(&section.figures, figures, "{}", section.label);
    }
}

#[test]
fn the_generated_family_recounts_independently() {
    // The recount of GJ-40 over the generated record.
    // The reading over the sets loaded back through the
    // crate's loader equals the direct transcription
    // over the sets in memory, and its closing states
    // the family's figures. The pin fails under the
    // mutation summing a part over every asking of the
    // question in every set, and under the swap of the
    // in accord and with the pool standings, the direct
    // transcription computing the accord figure at 43
    // and the pool figure at 53 where the swap states
    // them exchanged.
    let in_memory = generated_sets_in_memory();
    let (loaded, _scratch) = generated_sets_loaded();
    let lines = standing_lines(&loaded).expect("the standing reading computes");
    assert_eq!(lines, direct_standing_lines(&in_memory));
    let closing = closing_of(&lines);
    assert_eq!(figure_before(&closing, "sets"), 4);
    assert_eq!(figure_before(&closing, "parts"), 72);
    assert_eq!(figure_before(&closing, "with a plurality"), 62);
    assert_eq!(figure_before(&closing, "tied"), 9);
    assert_eq!(figure_before(&closing, "with no valid response"), 1);
    assert_eq!(figure_before(&closing, "in accord"), 43);
    assert_eq!(figure_before(&closing, "with the pool"), 53);
}

#[test]
fn the_generated_family_closes_under_its_own_relations() {
    // The relations of GJ-40 over the generated record:
    // the sums hold over the generated sections, and the
    // parts figure of the standing closing stands equal
    // to the parts figure of the accord closing and the
    // askings figure of the aggregate closing over the
    // same sets. The pin fails under the mutation
    // computing the closing line from the shipped
    // record's figures, whose parts stand at 64 where
    // the family's stand at 72.
    let (loaded, _scratch) = generated_sets_loaded();
    let lines = standing_lines(&loaded).expect("the standing reading computes");
    assert_standing_sums(&lines);
    let accord = accord_lines(&loaded).expect("an accord reading");
    let aggregate = aggregate_lines(&loaded).expect("an aggregate reading");
    let parts = figure_before(&closing_of(&lines), "parts");
    assert_eq!(parts, 72);
    assert_eq!(parts, figure_before(&closing_of(&accord), "parts"));
    assert_eq!(parts, figure_before(&closing_of(&aggregate), "askings"));
}
