//! Pins for the aggregate reading and command.
//!
//! The aggregate pins are stated in GJ-27 and GJ-28 in
//! CONTRACT.md, in the Round 13 section. One section
//! stands per question pooling all its askings as one
//! pool: the question line, the counts of the whole
//! question summed over its askings, one row per offered
//! answer of the question in the union order of its
//! askings through the shared row rendering of the
//! agreement reading computed from the summed counts,
//! and the plurality line of the question, the plurality
//! the application GJ-27 states in its own words. The
//! closing line counts the questions, the askings, the
//! valid responses, and the questions at which a
//! plurality stands, and the seven closing relations of
//! GJ-28 bind it to the census, the pool reading, and
//! the swing reading. The thirteenth command prints the
//! reading over label and directory pairs in the census
//! command's forms.
//!
//! Discrimination: every pin in this file fails against
//! the Round 12 tree. The function pins fail by compile
//! absence, the grandjury::aggregate module standing
//! nowhere in that tree. The command pins fail by
//! assertion against the surface's unknown-command
//! refusal, which names no aggregate among its choices.
//! The FIX-GJ-20 pins stand in test_binary_surface.rs
//! and test_setup.rs, and their own headers state their
//! split.

mod shared;

use grandjury::aggregate::aggregate_lines;
use grandjury::census::census_lines;
use grandjury::polls::{load_poll_file, Poll, Response};
use grandjury::pool::pool_lines;
use grandjury::pytext::PText;
use grandjury::swing::swing_lines;
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

/// n standing responses naming one answer, the jurors
/// labelled by hand and the models alternating, so a
/// hand-built shape states its counts without a fixture
/// file.
fn votes(answer: &str, count: usize) -> Vec<Response> {
    (0..count)
        .map(|i| {
            response(
                &format!("{} hand {}", answer, i + 1),
                if i % 2 == 0 { "model a" } else { "model b" },
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

/// The sections of a reading as (question, counts line,
/// row lines, plurality line) tuples, the closing line
/// left to the caller.
fn sections_of(lines: &[String]) -> Vec<(String, String, Vec<String>, String)> {
    let mut sections = Vec::new();
    let mut current: Vec<String> = Vec::new();
    for line in lines {
        if line.starts_with("question, ") && !current.is_empty() {
            sections.push(take_section(&current));
            current = Vec::new();
        }
        if line.starts_with("aggregates, ") {
            break;
        }
        current.push(line.clone());
    }
    if !current.is_empty() {
        sections.push(take_section(&current));
    }
    sections
}

fn take_section(lines: &[String]) -> (String, String, Vec<String>, String) {
    let question = lines[0]
        .strip_prefix("question, ")
        .expect("a question line")
        .to_string();
    let counts = lines[1].clone();
    let plurality = lines[lines.len() - 1].clone();
    let rows = lines[2..lines.len() - 1].to_vec();
    (question, counts, rows, plurality)
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

/// The aggregate reading computed directly from the
/// loaded polls with this pin's own arithmetic, each
/// question's responses counted response by response
/// under the validity rule of GJ-8 read over the whole
/// question, so the pin holds no crate counting.
fn direct_aggregate_lines(sets: &[(PText, Vec<Poll>)]) -> Vec<String> {
    let mut questions: Vec<String> = Vec::new();
    for (_, polls) in sets {
        for poll in polls {
            let text = poll.question.display();
            if !questions.contains(&text) {
                questions.push(text);
            }
        }
    }
    let mut lines = Vec::new();
    let mut asking_total = 0usize;
    let mut valid_total = 0usize;
    let mut plurality_total = 0usize;
    for question in &questions {
        let mut union: Vec<String> = Vec::new();
        let mut askings: Vec<&Poll> = Vec::new();
        for (_, polls) in sets {
            for poll in polls {
                if &poll.question.display() == question {
                    askings.push(poll);
                    for answer in &poll.answers {
                        let text = answer.display();
                        if !union.contains(&text) {
                            union.push(text);
                        }
                    }
                }
            }
        }
        let mut counts = vec![0usize; union.len()];
        let (mut recorded, mut valid, mut abstained, mut spoiled) = (0, 0, 0, 0);
        for poll in &askings {
            asking_total += 1;
            for held in &poll.responses {
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
        }
        lines.push(format!("question, {question}"));
        lines.push(format!(
            "pool, recorded {recorded}, valid {valid}, abstained {abstained}, spoiled {spoiled}"
        ));
        for (answer, count) in union.iter().zip(counts.iter()) {
            lines.push(format!(
                "  {answer}, {count}, {}, {}%",
                direct_share(*count, valid),
                direct_percent(*count, valid)
            ));
        }
        let largest = counts.iter().copied().max().unwrap_or(0);
        let holders = counts.iter().filter(|count| **count == largest).count();
        if valid > 0 && holders == 1 {
            let slot = counts
                .iter()
                .position(|count| *count == largest)
                .expect("a largest count");
            lines.push(format!("plurality, {}", union[slot]));
            plurality_total += 1;
        } else {
            lines.push("plurality, none".to_string());
        }
        valid_total += valid;
    }
    lines.push(format!(
        "aggregates, {} questions, {} askings, {} valid, {} pluralities",
        questions.len(),
        asking_total,
        valid_total,
        plurality_total
    ));
    lines
}

/// The closing relations of GJ-28 that bind the
/// aggregate reading to the census, the pool reading,
/// and the swing reading, checked over any sets, so the
/// golden pin and the generated pin share one statement
/// of them. The pluralities figure of the closing holds
/// no relation here, as GJ-28 states: it stands pinned
/// by the direct computation and the generated recount
/// alone.
fn assert_closing_relations(sets: &[(PText, Vec<Poll>)]) {
    let lines = aggregate_lines(sets).expect("the aggregate reading computes");
    let sections = sections_of(&lines);
    let closing = lines.last().expect("a closing line").clone();

    // The census record line over the same sets.
    let census = census_lines(sets).expect("the census computes");
    let record_line = census
        .iter()
        .find(|line| line.starts_with("record, "))
        .expect("a census record line")
        .clone();
    let census_counts = (
        figure_after(&record_line, "recorded "),
        figure_after(&record_line, "valid "),
        figure_after(&record_line, "abstained "),
        figure_after(&record_line, "spoiled "),
    );

    // Relation 2: the closing valid equals the census
    // record line's valid. Relation 3: the summed
    // section counts equal the census record line's four
    // counts.
    assert_eq!(figure_before(&closing, "valid"), census_counts.1);
    let mut summed = (0usize, 0usize, 0usize, 0usize);
    for (_, counts_line, _, _) in &sections {
        let counts = counts_of(counts_line);
        summed.0 += counts.0;
        summed.1 += counts.1;
        summed.2 += counts.2;
        summed.3 += counts.3;
    }
    assert_eq!(summed, census_counts);

    // Relation 1: in each section the answer counts sum
    // to the section's valid.
    for (_, counts_line, rows, _) in &sections {
        let row_sum: usize = rows.iter().map(|row| row_of(row).1).sum();
        assert_eq!(row_sum, counts_of(counts_line).1);
    }

    // Relation 4 and relation 7 read against the pool
    // reading over the same sets. A pool block stands
    // per question, and inside it one part stands per
    // asking: the set counts line of the asking, its
    // answer rows, and its plurality line, which carries
    // the set label where the aggregate plurality line
    // carries none, as GJ-28 states.
    let pool = pool_lines(sets).expect("the pool reading computes");
    let mut pool_counts: std::collections::HashMap<(String, String), usize> =
        std::collections::HashMap::new();
    let mut pool_askings: std::collections::HashMap<
        String,
        Vec<((usize, usize, usize, usize), Vec<String>, String)>,
    > = std::collections::HashMap::new();
    let mut current_question: Option<String> = None;
    let mut current_label: Option<String> = None;
    for line in &pool {
        if let Some(question) = line.strip_prefix("question, ") {
            current_question = Some(question.to_string());
        } else if line.starts_with("set, ") {
            let label = line
                .strip_prefix("set, ")
                .expect("a set counts line")
                .split(", ")
                .next()
                .expect("a set label")
                .to_string();
            current_label = Some(label);
            let question = current_question
                .clone()
                .expect("a question before its sets");
            pool_askings.entry(question).or_default().push((
                counts_of(line),
                Vec::new(),
                String::new(),
            ));
        } else if line.starts_with("  ") {
            let (answer, count, _, _) = row_of(line);
            let question = current_question
                .clone()
                .expect("a question before its rows");
            *pool_counts.entry((question.clone(), answer)).or_insert(0) += count;
            pool_askings
                .get_mut(&question)
                .expect("an asking in progress")
                .last_mut()
                .expect("an asking in progress")
                .1
                .push(line.clone());
        } else if let Some(rest) = line.strip_prefix("plurality, ") {
            let label = current_label
                .clone()
                .expect("a set label before its plurality");
            let value = rest
                .strip_prefix(&format!("{label}, "))
                .expect("the plurality of the asking")
                .to_string();
            let question = current_question
                .clone()
                .expect("a question before its plurality");
            pool_askings
                .get_mut(&question)
                .expect("an asking in progress")
                .last_mut()
                .expect("an asking in progress")
                .2 = value;
        }
    }

    // Relation 4: the per-answer counts of each section
    // equal the sums over the pool blocks of that
    // question.
    for (question, _, rows, _) in &sections {
        for row in rows {
            let (answer, count, _, _) = row_of(row);
            assert_eq!(
                pool_counts.get(&(question.clone(), answer)),
                Some(&count),
                "the pooled count of an answer"
            );
        }
    }

    // Relation 6: the closing questions equal the pool
    // closing's questions, and the closing askings equal
    // the swing closing's askings and the census record
    // line's polls.
    let pool_closing = pool.last().expect("a pool closing").clone();
    assert_eq!(
        figure_before(&closing, "questions"),
        figure_before(&pool_closing, "questions")
    );
    let swing = swing_lines(sets).expect("the swing reading computes");
    let swing_closing = swing.last().expect("a swing closing").clone();
    assert_eq!(
        figure_before(&closing, "askings"),
        figure_before(&swing_closing, "askings")
    );
    assert_eq!(
        figure_before(&closing, "askings"),
        figure_after(&record_line, "polls ")
    );

    // Relation 7: a section over a question at one
    // asking states that asking's pooled counts, its
    // answer rows, and its plurality, as the pool block
    // of that asking states them.
    for (question, counts_line, rows, plurality_line) in &sections {
        let askings = pool_askings
            .get(question)
            .expect("the pool blocks of the question");
        if askings.len() == 1 {
            let (pool_counts_tuple, pool_rows, pool_plurality) = &askings[0];
            assert_eq!(counts_of(counts_line), *pool_counts_tuple);
            assert_eq!(rows, pool_rows);
            assert_eq!(plurality_line, &format!("plurality, {pool_plurality}"));
        }
    }
}

#[test]
fn the_aggregate_golden_stands_byte_for_byte() {
    let golden = read_text(&examples().join("aggregate-golden.txt"));
    let (stdout, _, code) = run_cli(&[
        "aggregate",
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
fn the_aggregate_reading_computes_directly_from_the_record() {
    let sets = recorded_sets();
    let expected = direct_aggregate_lines(&sets);
    assert_eq!(
        aggregate_lines(&sets).expect("the aggregate reading computes"),
        expected
    );
    let (stdout, _, code) = run_cli(&[
        "aggregate",
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
fn the_aggregate_closing_relations_hold_over_the_record() {
    assert_closing_relations(&recorded_sets());
}

#[test]
fn an_aggregate_tie_closes_at_none() {
    // GJ-28 states this shape in its own words, a
    // question at two askings whose pooled top ties,
    // each asking holding a plurality for a different
    // answer and the summed counts standing equal,
    // closing on the word none. The hand computation
    // stands as follows. The first asking holds three
    // votes for the north gate and two for the south
    // gate, a plurality for the north gate at three to
    // two. The second asking holds two for the north
    // gate and three for the south gate, a plurality
    // for the south gate at three to two. The summed
    // counts stand at five and five, equal, so no
    // answer holds the largest summed count alone and
    // the section closes on the word none. A reading
    // that closed on either asking's own plurality, or
    // that broke the pooled tie by the union order,
    // would print an answer's name on the plurality
    // line, so the pin fails under those readings and
    // passes only under the pooled tie rule.
    let question = "Which gate should the night watch use?";
    let mut first = votes("the north gate", 3);
    first.extend(votes("the south gate", 2));
    let mut second = votes("the north gate", 2);
    second.extend(votes("the south gate", 3));
    let sets = vec![
        (
            pt("alpha"),
            vec![poll(question, &["the north gate", "the south gate"], first)],
        ),
        (
            pt("beta"),
            vec![poll(
                question,
                &["the north gate", "the south gate"],
                second,
            )],
        ),
    ];
    assert_eq!(
        aggregate_lines(&sets).expect("the aggregate reading computes"),
        vec![
            "question, Which gate should the night watch use?",
            "pool, recorded 10, valid 10, abstained 0, spoiled 0",
            "  the north gate, 5, 1/2, 50.0%",
            "  the south gate, 5, 1/2, 50.0%",
            "plurality, none",
            "aggregates, 1 questions, 2 askings, 10 valid, 0 pluralities",
        ]
    );
}

#[test]
fn an_aggregate_plurality_can_win_neither_asking() {
    let question = "Which mill should grind the harvest flour?";
    let answers = &["the water mill", "the wind mill", "the steam mill"];
    let mut first = votes("the water mill", 5);
    first.extend(votes("the wind mill", 1));
    first.extend(votes("the steam mill", 4));
    let mut second = votes("the water mill", 1);
    second.extend(votes("the wind mill", 5));
    second.extend(votes("the steam mill", 4));
    let sets = vec![
        (pt("alpha"), vec![poll(question, answers, first)]),
        (pt("beta"), vec![poll(question, answers, second)]),
    ];
    assert_eq!(
        aggregate_lines(&sets).expect("the aggregate reading computes"),
        vec![
            "question, Which mill should grind the harvest flour?",
            "pool, recorded 20, valid 20, abstained 0, spoiled 0",
            "  the water mill, 6, 3/10, 30.0%",
            "  the wind mill, 6, 3/10, 30.0%",
            "  the steam mill, 8, 2/5, 40.0%",
            "plurality, the steam mill",
            "aggregates, 1 questions, 2 askings, 20 valid, 1 pluralities",
        ]
    );
}

#[test]
fn a_question_with_no_valid_response_closes_at_none() {
    let question = "Should the ferry run on Christmas Day?";
    let sets = vec![
        (
            pt("alpha"),
            vec![poll(
                question,
                &["run it", "rest the crew"],
                vec![
                    response("alpha hand 1", "model a", None, "unused"),
                    response("alpha hand 2", "model b", None, "unused"),
                    response("alpha hand 3", "model a", None, "unused"),
                ],
            )],
        ),
        (
            pt("beta"),
            vec![poll(
                question,
                &["run it", "rest the crew"],
                vec![
                    response("beta hand 1", "model a", None, "unused"),
                    response("beta hand 2", "model b", None, "unused"),
                ],
            )],
        ),
    ];
    assert_eq!(
        aggregate_lines(&sets).expect("the aggregate reading computes"),
        vec![
            "question, Should the ferry run on Christmas Day?",
            "pool, recorded 5, valid 0, abstained 5, spoiled 0",
            "  run it, 0, 0, 0.0%",
            "  rest the crew, 0, 0, 0.0%",
            "plurality, none",
            "aggregates, 1 questions, 2 askings, 0 valid, 0 pluralities",
        ]
    );
}

#[test]
fn an_answer_offered_at_the_second_asking_enters_in_union_order() {
    let question = "Which field should host the maypole?";
    let mut first = votes("the north field", 2);
    first.extend(votes("the south field", 1));
    let mut second = votes("the south field", 1);
    second.extend(votes("the east field", 3));
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
                &["the south field", "the east field"],
                second,
            )],
        ),
    ];
    assert_eq!(
        aggregate_lines(&sets).expect("the aggregate reading computes"),
        vec![
            "question, Which field should host the maypole?",
            "pool, recorded 7, valid 7, abstained 0, spoiled 0",
            "  the north field, 2, 2/7, 28.6%",
            "  the south field, 2, 2/7, 28.6%",
            "  the east field, 3, 3/7, 42.9%",
            "plurality, the east field",
            "aggregates, 1 questions, 2 askings, 7 valid, 1 pluralities",
        ]
    );
}

#[test]
fn the_aggregate_guard_reaches_the_second_set() {
    let handed = Response {
        juror: pt("beta hand 1"),
        model: pt("model a"),
        answer: pt("run it"),
        reason: pt("a handed\nreason"),
        abstained: false,
    };
    let sets = vec![
        (
            pt("alpha"),
            vec![poll(
                "Should the ferry run on Christmas Day?",
                &["run it", "rest the crew"],
                votes("run it", 1),
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
        aggregate_lines(&sets),
        Err("response 0 field reason holds a newline".to_string())
    );
}

#[test]
fn the_aggregate_set_rules_refuse_as_the_pool_refuses() {
    let first = vec![poll(
        "Should the ferry run on Christmas Day?",
        &["run it", "rest the crew"],
        votes("run it", 1),
    )];
    let second = vec![poll(
        "Which gate should the night watch use?",
        &["the north gate", "the south gate"],
        votes("the north gate", 1),
    )];
    let duplicated_label = vec![(pt("alpha"), first.clone()), (pt("alpha"), second)];
    assert_eq!(
        aggregate_lines(&duplicated_label),
        Err("the set label alpha stands twice".to_string())
    );
    let question = "Should the ferry run on Christmas Day?";
    let doubled = vec![(pt("alpha"), vec![first[0].clone(), first[0].clone()])];
    assert_eq!(
        aggregate_lines(&doubled),
        Err(format!("the set alpha holds the question {question} twice"))
    );
}

#[test]
fn no_verdict_word_stands_in_the_aggregate_reading() {
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
    let golden = read_text(&examples().join("aggregate-golden.txt"));
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
        "aggregate",
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
    // set, and the closing word of the reading is
    // aggregates.
    for text in [&golden, &out] {
        for line in text.lines() {
            assert!(
                !line.starts_with("set, "),
                "no line of the reading names a set: {line}"
            );
        }
        let closing = text.lines().last().expect("a closing line");
        assert!(
            closing.starts_with("aggregates, "),
            "the closing word of the reading is aggregates: {closing}"
        );
    }
}

#[test]
fn the_aggregate_command_refuses_a_defective_poll_in_the_second_set() {
    let tmp = TempDir::new("aggregate-defective");
    let bad = jobj(vec![
        ("question", jstr("A defective poll?")),
        ("answers", jarr(vec![jstr("only one")])),
        ("responses", jarr(vec![])),
    ]);
    tmp.write_json("bad.json", &bad);
    let (stdout, _, code) = run_cli(&[
        "aggregate",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "marsh",
        &path_arg(&tmp.path),
    ]);
    assert_eq!(code, 1);
    assert!(stdout.starts_with("invalid poll\ndefect "));
    assert!(!stdout.contains("aggregates"));
}

#[test]
fn the_aggregate_command_draws_the_reading_refusal() {
    let (stdout, _, code) = run_cli(&[
        "aggregate",
        "pilot",
        &path_arg(&examples().join("pilot")),
        "pilot",
        &path_arg(&examples().join("wave")),
    ]);
    assert_eq!(code, 1);
    assert_eq!(
        stdout,
        "invalid aggregate\ndefect the set label pilot stands twice\n"
    );
}

#[test]
fn the_aggregate_command_refuses_an_unpaired_label() {
    let (_, stderr, code) = run_cli(&["aggregate", "pilot"]);
    assert_eq!(code, 2);
    assert!(stderr.contains("the following arguments are required: directory"));
}

#[test]
fn the_aggregate_command_refuses_no_arguments() {
    let (_, stderr, code) = run_cli(&["aggregate"]);
    assert_eq!(code, 2);
    assert!(stderr.contains("the following arguments are required: label, directory"));
}

#[test]
fn the_aggregate_help_names_the_command() {
    let (stdout, _, code) = run_cli(&["aggregate", "--help"]);
    assert_eq!(code, 0);
    assert!(
        stdout.contains("usage: grandjury aggregate [-h] label directory [label directory ...]")
    );
    let (top, _, top_code) = run_cli(&["--help"]);
    assert_eq!(top_code, 0);
    assert!(top.contains("print the aggregate reading over recorded poll sets"));
}

// The generated fixture family of GJ-27: polls drawn in
// the test from the stated distribution at the stated
// seed, loaded through the crate's own loader, and
// pinned by the closing relations of GJ-28 and by an
// independent recount inside the test. No golden ships
// for the family, as GJ-27 states: its sections are
// pinned by the relations and the recount, never by a
// stored rendering.
//
// The two span pins at the end of this file state
// their split against the generator this round's
// reviews found, which drew the asking count over one
// to four and the model count over one to three, ran
// the stated phases interleaved in one per question
// pass, and let an abstained response consume neither
// the answer draw nor the reason draw. The model span
// pin fails against that generator by assertion at the
// stated seed, thirteen of its thirty eight polls
// holding one model string. The asking span pin
// passes against that generator's fixture at the
// stated seed, its asking counts reading one to three
// there with the span divergence latent, as the
// reviews record, and it stands as the bound the
// stated span and any later seed must hold. Both pins
// pass on the repaired generator.

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
        for question_idx in 0..36 {
            if !asking_sets_of[question_idx].contains(&set_idx) {
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
        let tmp = TempDir::new("aggregate-generated");
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
fn the_generated_family_closes_under_its_own_relations() {
    assert_closing_relations(&generated_sets_loaded());
}

#[test]
fn the_generated_family_recounts_independently() {
    let in_memory = generated_sets_in_memory();
    let loaded = generated_sets_loaded();
    let lines = aggregate_lines(&loaded).expect("the aggregate reading computes");
    assert_eq!(lines, direct_aggregate_lines(&in_memory));
    let sections = sections_of(&lines);
    assert_eq!(sections.len(), 36);
    assert!(sections
        .iter()
        .all(|(_, counts, _, _)| counts_of(counts).0 > 0));
}

#[test]
fn the_generated_family_holds_its_stated_shapes() {
    let in_memory = generated_sets_in_memory();
    let mut askings_of: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for (_, polls) in &in_memory {
        for held in polls {
            *askings_of.entry(held.question.display()).or_insert(0) += 1;
        }
    }
    assert_eq!(askings_of.len(), 36);
    assert!(
        askings_of.values().any(|count| *count >= 3),
        "at least one generated question stands at three askings"
    );
    let lines = aggregate_lines(&generated_sets_loaded()).expect("the aggregate reading computes");
    let sections = sections_of(&lines);
    assert!(
        sections
            .iter()
            .any(|(_, _, _, plurality)| plurality != "plurality, none"),
        "at least one generated section holds a standing plurality"
    );
}

#[test]
fn the_generated_family_holds_its_asking_span() {
    // GJ-27 states each question stands asked in one
    // to three of the four generated sets. Against the
    // generator the reviews found, which drew the
    // asking count over one to four, this pin passes
    // at the stated seed, that fixture's asking counts
    // reading one to three with the span divergence
    // latent, and it stands as the bound the stated
    // span and any later seed must hold.
    let in_memory = generated_sets_in_memory();
    let mut askings_of: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for (_, polls) in &in_memory {
        for held in polls {
            *askings_of.entry(held.question.display()).or_insert(0) += 1;
        }
    }
    assert_eq!(askings_of.len(), 36);
    assert!(
        askings_of.values().all(|count| (1..=3).contains(count)),
        "every generated question stands at one to three askings"
    );
}

#[test]
fn the_generated_family_holds_its_model_span() {
    // GJ-27 states each poll of the record holds two
    // or three model strings. Against the generator
    // the reviews found, which drew the model count
    // over one to three, this pin fails by assertion
    // at the stated seed, thirteen of its thirty eight
    // polls holding one model string.
    let in_memory = generated_sets_in_memory();
    let mut polls = 0usize;
    for (_, set_polls) in &in_memory {
        for held in set_polls {
            polls += 1;
            let mut models: Vec<String> = Vec::new();
            for entry in &held.responses {
                let model = entry.model.display();
                if !models.contains(&model) {
                    models.push(model);
                }
            }
            assert!(
                (2..=3).contains(&models.len()),
                "every generated poll holds two or three model strings"
            );
        }
    }
    assert!(polls > 0);
}
