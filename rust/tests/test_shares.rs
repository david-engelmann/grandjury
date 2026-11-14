//! The shares reading, GJ-47 and GJ-48, twenty pins
//! restated from first principles.
//!
//! The reading is recomputed from the shipped sets
//! and from the generated family by an independent
//! walk in this file, the direct computation below,
//! which counts the valid responses of each part
//! itself and gathers the record by set without the
//! library's reading machinery, and the library walk
//! and the printed surface are compared against it
//! line for line, so a defect in the library walk
//! fails here even where a figure the closing
//! relations reach would let it pass. The shares of
//! each question stand summed to exactly one in
//! exact arithmetic over the printed grounds. The
//! shapes stand in the forms GJ-48 states them, S1
//! to S6, each on a fixture small enough to read
//! whole, the twice asking shape standing as the
//! refusal FIX-GJ-27 records. Against the Round 22
//! closure tree this target does not compile, the
//! shares module standing absent there, and that
//! failure is the base run the round records.
//!
//! The discrimination of the pins is stated against
//! the mutation set of the Round 23 design, QA to
//! QH, its postures computed by the validated
//! transcription of the design, and no posture is
//! stated the computations do not carry. QA, the
//! part counted from its recorded responses in
//! place of its valid, fails pins 1, 2, 3, 4, 6, 7,
//! 8, 9, 10, 11, and 19. QB, the share printed in
//! its unreduced form, fails pins 1, 3, 7, 9, 10,
//! and 11, and pin 4 stands, reading the printed
//! grounds and comparing the fractions by value.
//! QC, a part standing whole at half its question
//! or more, fails pins 1, 2, 3, 6, 10, 11, and 19.
//! QD, a question whose valid count stands at zero
//! rendered as a share of none in place of not
//! counted, fails pins 1, 2, 3, 4, and 8. QF, the
//! sections stated in reverse input order, fails
//! pins 1, 3, 6, 7, 8, 9, 10, 11, and 13, and pins
//! 2, 12, and 19 stand, pin 2's pilot and wave
//! standings standing identical, pin 12 asserting
//! no place, and pin 19 reading the generated
//! standings as a multiset. QG, the closing's whole
//! and none figures swapped, fails pins 1, 2, 3, 7,
//! and 19, and pin 4 stands, asserting the grounds
//! and values of the share lines and not the
//! closing. QH, the share line printed without its
//! grounds, fails pins 1, 3, 4, 6, 7, 9, 10, and
//! 11. The mutation the design names QE, a part
//! taken from its first asking alone, stands
//! uncharged, uncatchable by construction in the
//! posture of the mutation ME of FIX-GJ-23, and no
//! pin claims it. QI, the validate_sets call of the
//! shares path dropped, fails pins 11 and 14, the
//! set computed in the build's warm-up from the
//! accord record, the dropped call's refusals
//! standing asserted by the refusal pin and by the
//! set rules pin together and by no other pin of
//! the twenty. The dispatch restated to run the
//! tally handler on its first value fails pin 23 in
//! rust/tests/test_cli.rs alone, and pins 21 and 22
//! stand. No charged mutation fells pin 5, which
//! stands against a counting of the accord closing's
//! parts by set in place of by asking, or pin 12,
//! which stands against a build dropping the empty
//! section, or pin 15, which stands against surface
//! mutations of the guard, or pin 16, which stands
//! against a golden swapped for another reading's,
//! or pin 17, which stands against a word outside
//! the vocabulary entering the reading's lines, or
//! pin 18, which stands against a help text naming
//! the wrong count of commands, or pin 20, which
//! stands against a loader divergence between the
//! written and the in-memory record.

mod shared;

use shared::*;

use grandjury::accord::accord_lines;
use grandjury::census::census_lines;
use grandjury::polls::{load_poll_file, Poll, Response};
use grandjury::pytext::PText;
use grandjury::shares::shares_lines;

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

fn expected(lines: &[&str]) -> Vec<String> {
    lines.iter().map(|line| line.to_string()).collect()
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a.abs();
    let mut b = b.abs();
    while b != 0 {
        let rest = a % b;
        a = b;
        b = rest;
    }
    a
}

/// The valid count of one part counted from the
/// responses themselves, every response of the
/// part's polls that stands valid under its own
/// poll counted once.
fn direct_valid(polls: &[&Poll]) -> i64 {
    let mut valid = 0i64;
    for poll in polls {
        for response in &poll.responses {
            if !response.abstained && poll.answers.contains(&response.answer) {
                valid += 1;
            }
        }
    }
    valid
}

/// The fraction of a share in the form of GJ-8,
/// the part and the question's count divided by
/// their common divisor, a denominator of one
/// rendering as the numerator alone.
fn direct_fraction(part: i64, total: i64) -> String {
    let step = gcd(part, total);
    let (part, total) = (part / step, total / step);
    if total == 1 {
        format!("{}", part)
    } else {
        format!("{}/{}", part, total)
    }
}

/// The shares reading recomputed from first
/// principles, the transcription the library walk
/// and the printed surface are compared against.
fn direct_shares_lines(sets: &[(PText, Vec<Poll>)]) -> Vec<String> {
    let questions = questions_of(sets);
    let mut lines: Vec<String> = Vec::new();
    let mut totals = [0i64; 4];
    for (set_index, (label, _)) in sets.iter().enumerate() {
        lines.push(format!("set, {}", label.display()));
        let mut standing = [0i64; 4];
        for question in &questions {
            let askings = askings_of(sets, question);
            let part_polls: Vec<&Poll> = askings
                .iter()
                .filter(|(index, _)| *index == set_index)
                .map(|(_, poll)| *poll)
                .collect();
            if part_polls.is_empty() {
                continue;
            }
            let part = direct_valid(&part_polls);
            let mut question_valid = 0i64;
            for other in 0..sets.len() {
                let other_polls: Vec<&Poll> = askings
                    .iter()
                    .filter(|(index, _)| *index == other)
                    .map(|(_, poll)| *poll)
                    .collect();
                if !other_polls.is_empty() {
                    question_valid += direct_valid(&other_polls);
                }
            }
            standing[0] += 1;
            lines.push(format!("question, {}", question.display()));
            if question_valid == 0 {
                standing[3] += 1;
                lines.push("share, not counted".to_string());
            } else {
                if part == question_valid {
                    standing[1] += 1;
                }
                if part == 0 {
                    standing[2] += 1;
                }
                lines.push(format!(
                    "share, {} of {}, {}",
                    part,
                    question_valid,
                    direct_fraction(part, question_valid)
                ));
            }
        }
        lines.push(format!(
            "standing, {} parts, {} whole, {} none, {} not counted",
            standing[0], standing[1], standing[2], standing[3]
        ));
        for slot in 0..4 {
            totals[slot] += standing[slot];
        }
    }
    lines.push(format!(
        "shares, {} sets, {} parts, {} whole, {} none, {} not counted",
        sets.len(),
        totals[0],
        totals[1],
        totals[2],
        totals[3]
    ));
    lines
}

#[test]
fn shares_golden_matches_the_reading_over_the_shipped_sets() {
    let sets = recorded_sets();
    let lines = shares_lines(&sets).unwrap();
    assert_eq!(lines.len(), 135);
    let golden = read_text(&examples().join("shares-golden.txt"));
    let golden_lines: Vec<String> = golden.lines().map(|line| line.to_string()).collect();
    assert_eq!(lines, golden_lines);
    assert_eq!(
        lines.last().unwrap(),
        "shares, 3 sets, 64 parts, 33 whole, 0 none, 9 not counted"
    );
}

#[test]
fn the_shares_closing_and_standings_stand_as_stated() {
    let sets = recorded_sets();
    let lines = shares_lines(&sets).unwrap();
    let standings: Vec<&str> = lines
        .iter()
        .filter(|line| line.starts_with("standing,"))
        .map(|line| line.as_str())
        .collect();
    assert_eq!(
        standings,
        vec![
            "standing, 12 parts, 0 whole, 0 none, 1 not counted",
            "standing, 40 parts, 33 whole, 0 none, 7 not counted",
            "standing, 12 parts, 0 whole, 0 none, 1 not counted",
        ]
    );
    assert_eq!(
        lines.last().unwrap(),
        "shares, 3 sets, 64 parts, 33 whole, 0 none, 9 not counted"
    );
}

#[test]
fn direct_computation_matches_the_shares_reading() {
    let sets = recorded_sets();
    assert_eq!(shares_lines(&sets).unwrap(), direct_shares_lines(&sets));
    let generated = generated_sets_in_memory();
    assert_eq!(
        shares_lines(&generated).unwrap(),
        direct_shares_lines(&generated)
    );
}

#[test]
fn shares_sum_to_one_at_every_counted_question() {
    let sets = recorded_sets();
    let lines = shares_lines(&sets).unwrap();
    let mut counted: Vec<(String, i64, i64, i64, i64)> = Vec::new();
    let mut not_counted: Vec<String> = Vec::new();
    let mut question: Option<String> = None;
    for line in &lines {
        if let Some(rest) = line.strip_prefix("question, ") {
            question = Some(rest.to_string());
        } else if let Some(rest) = line.strip_prefix("share, ") {
            let held = question.clone().unwrap();
            if rest == "not counted" {
                not_counted.push(held);
            } else {
                let (grounds, fraction) = rest.split_once(", ").unwrap();
                let (part, total) = grounds.split_once(" of ").unwrap();
                let (num, den) = match fraction.split_once('/') {
                    Some((top, bottom)) => {
                        (top.parse::<i64>().unwrap(), bottom.parse::<i64>().unwrap())
                    }
                    None => (fraction.parse::<i64>().unwrap(), 1),
                };
                counted.push((
                    held,
                    part.parse::<i64>().unwrap(),
                    total.parse::<i64>().unwrap(),
                    num,
                    den,
                ));
            }
        }
    }
    assert_eq!(counted.len() + not_counted.len(), 64);
    for held in &questions_of(&sets) {
        let text = held.display();
        let askings = askings_of(&sets, held);
        let mut parts: Vec<i64> = Vec::new();
        let mut total = 0i64;
        for set_index in 0..sets.len() {
            let polls: Vec<&Poll> = askings
                .iter()
                .filter(|(index, _)| *index == set_index)
                .map(|(_, poll)| *poll)
                .collect();
            if polls.is_empty() {
                continue;
            }
            let valid = direct_valid(&polls);
            parts.push(valid);
            total += valid;
        }
        let printed: Vec<&(String, i64, i64, i64, i64)> =
            counted.iter().filter(|(name, ..)| name == &text).collect();
        let printed_not_counted: Vec<&String> =
            not_counted.iter().filter(|name| *name == &text).collect();
        if total == 0 {
            assert!(printed.is_empty(), "a counted share at {}", text);
            assert_eq!(printed_not_counted.len(), parts.len());
        } else {
            assert!(printed_not_counted.is_empty(), "a not counted at {}", text);
            assert_eq!(printed.len(), parts.len());
            let (mut sum_num, mut sum_den) = (0i64, 1i64);
            for (index, (_, part, printed_total, num, den)) in printed.iter().enumerate() {
                assert_eq!(*part, parts[index]);
                assert_eq!(*printed_total, total);
                assert_eq!(num * total, part * den, "a share value at {}", text);
                sum_num = sum_num * den + num * sum_den;
                sum_den *= den;
                let step = gcd(sum_num, sum_den);
                sum_num /= step;
                sum_den /= step;
            }
            assert_eq!(
                (sum_num, sum_den),
                (1, 1),
                "the shares at {} sum to one",
                text
            );
        }
    }
}

#[test]
fn closing_parts_reconcile_with_the_accord_reading() {
    let sets = recorded_sets();
    let accord = accord_lines(&sets).unwrap();
    let parts = accord
        .iter()
        .filter(|line| line.starts_with("set, "))
        .count();
    assert_eq!(parts, 64);
    assert_eq!(figure_before(accord.last().unwrap(), "parts"), 64);
    let lines = shares_lines(&sets).unwrap();
    assert_eq!(figure_before(lines.last().unwrap(), "parts"), 64);
    assert_eq!(
        figure_before(lines.last().unwrap(), "parts") as usize,
        parts
    );
}

#[test]
fn divided_shape_states_each_part_of_the_question() {
    let question = "Which gate should the fair open first?";
    let answers = &["the red gate", "the blue gate"];
    let mut first = votes_of("alpha", "the red gate", 3);
    first.push(response(
        "alpha hand the red gate 4",
        "model a",
        None,
        "no answer recorded",
    ));
    let mut second = votes_of("beta", "the red gate", 1);
    second.push(response(
        "beta hand the red gate 2",
        "model a",
        None,
        "no answer recorded",
    ));
    let sets = vec![
        (pt("alpha"), vec![poll(question, answers, first)]),
        (pt("beta"), vec![poll(question, answers, second)]),
    ];
    assert_eq!(
        shares_lines(&sets).unwrap(),
        expected(&[
            "set, alpha",
            "question, Which gate should the fair open first?",
            "share, 3 of 4, 3/4",
            "standing, 1 parts, 0 whole, 0 none, 0 not counted",
            "set, beta",
            "question, Which gate should the fair open first?",
            "share, 1 of 4, 1/4",
            "standing, 1 parts, 0 whole, 0 none, 0 not counted",
            "shares, 2 sets, 2 parts, 0 whole, 0 none, 0 not counted",
        ])
    );
}

#[test]
fn whole_shape_states_the_part_standing_whole() {
    let question = "Which gate should the fair open first?";
    let answers = &["the red gate", "the blue gate"];
    let mut only = votes_of("alpha", "the red gate", 3);
    only.push(response(
        "alpha hand the red gate 4",
        "model a",
        None,
        "no answer recorded",
    ));
    let sets = vec![(pt("alpha"), vec![poll(question, answers, only)])];
    assert_eq!(
        shares_lines(&sets).unwrap(),
        expected(&[
            "set, alpha",
            "question, Which gate should the fair open first?",
            "share, 3 of 3, 1",
            "standing, 1 parts, 1 whole, 0 none, 0 not counted",
            "shares, 1 sets, 1 parts, 1 whole, 0 none, 0 not counted",
        ])
    );
}

#[test]
fn uncounted_shape_states_no_part_as_counted() {
    let question = "Which gate should the fair open first?";
    let answers = &["the red gate", "the blue gate"];
    let idle = |jury: &str| {
        vec![
            response(
                &format!("{} hand one", jury),
                "model a",
                None,
                "no answer recorded",
            ),
            response(
                &format!("{} hand two", jury),
                "model a",
                None,
                "no answer recorded",
            ),
        ]
    };
    let sets = vec![
        (pt("alpha"), vec![poll(question, answers, idle("alpha"))]),
        (pt("beta"), vec![poll(question, answers, idle("beta"))]),
    ];
    assert_eq!(
        shares_lines(&sets).unwrap(),
        expected(&[
            "set, alpha",
            "question, Which gate should the fair open first?",
            "share, not counted",
            "standing, 1 parts, 0 whole, 0 none, 1 not counted",
            "set, beta",
            "question, Which gate should the fair open first?",
            "share, not counted",
            "standing, 1 parts, 0 whole, 0 none, 1 not counted",
            "shares, 2 sets, 2 parts, 0 whole, 0 none, 2 not counted",
        ])
    );
}

#[test]
fn empty_and_whole_shape_states_the_none_and_the_whole() {
    let question = "Which gate should the fair open first?";
    let answers = &["the red gate", "the blue gate"];
    let idle = vec![
        response("alpha hand one", "model a", None, "no answer recorded"),
        response("alpha hand two", "model a", None, "no answer recorded"),
    ];
    let sets = vec![
        (pt("alpha"), vec![poll(question, answers, idle)]),
        (
            pt("beta"),
            vec![poll(question, answers, votes_of("beta", "the red gate", 4))],
        ),
    ];
    assert_eq!(
        shares_lines(&sets).unwrap(),
        expected(&[
            "set, alpha",
            "question, Which gate should the fair open first?",
            "share, 0 of 4, 0",
            "standing, 1 parts, 0 whole, 1 none, 0 not counted",
            "set, beta",
            "question, Which gate should the fair open first?",
            "share, 4 of 4, 1",
            "standing, 1 parts, 1 whole, 0 none, 0 not counted",
            "shares, 2 sets, 2 parts, 1 whole, 1 none, 0 not counted",
        ])
    );
}

#[test]
fn lowest_terms_shape_states_each_share_reduced() {
    let question = "Which gate should the fair open first?";
    let answers = &["the red gate", "the blue gate"];
    let mut first = votes_of("alpha", "the red gate", 6);
    first.push(response(
        "alpha hand the red gate 7",
        "model a",
        None,
        "no answer recorded",
    ));
    first.push(response(
        "alpha hand the red gate 8",
        "model a",
        None,
        "no answer recorded",
    ));
    let sets = vec![
        (pt("alpha"), vec![poll(question, answers, first)]),
        (
            pt("beta"),
            vec![poll(question, answers, votes_of("beta", "the red gate", 4))],
        ),
    ];
    assert_eq!(
        shares_lines(&sets).unwrap(),
        expected(&[
            "set, alpha",
            "question, Which gate should the fair open first?",
            "share, 6 of 10, 3/5",
            "standing, 1 parts, 0 whole, 0 none, 0 not counted",
            "set, beta",
            "question, Which gate should the fair open first?",
            "share, 4 of 10, 2/5",
            "standing, 1 parts, 0 whole, 0 none, 0 not counted",
            "shares, 2 sets, 2 parts, 0 whole, 0 none, 0 not counted",
        ])
    );
}

#[test]
fn a_set_asking_a_question_twice_draws_the_set_rules_refusal() {
    // Shape S6 of GJ-48, as FIX-GJ-27 restates it.
    // The first set holds two polls of the question,
    // the first naming the low meadow at two valid of
    // two and the second naming the high meadow at
    // three valid of four with one abstained, and the
    // second set names the high meadow at five valid
    // of five. As designed, GJ-48 stated one part
    // summed over both askings of the first set, and
    // this pin asserted that part. That statement
    // describes a record the set rules refuse, GJ-47
    // holding the set rules of GJ-2 unchanged and
    // GJ-2 refusing a set that holds one question
    // twice, so the shares reading refuses over the
    // whole record, no summed part stands, and the
    // pin stands restated as a refusal pin over the
    // fixture unchanged, asserting the refusal GJ-2
    // names. The mutation the design names QE, a
    // part built from its set's first asking alone,
    // stands uncatchable by construction and this pin
    // does not claim it, because on every record the
    // set rules accept the first asking of a set at a
    // question is its only asking, so the mutated
    // form and the standing form state the same part
    // everywhere the reading runs. The pin's
    // discrimination stands against a guard mutation
    // that drops the validate_sets call in the shares
    // path, the mutation the round names QI, under
    // which the reading would state the summed part
    // the removed sentence stated and this pin fails.
    let question = "Which meadow should host the fair?";
    let answers = &["the low meadow", "the high meadow"];
    let first = votes_of("alpha", "the low meadow", 2);
    let mut second = votes_of("alpha", "the high meadow", 3);
    second.push(response(
        "alpha hand the high meadow 4",
        "model a",
        None,
        "no answer recorded",
    ));
    let sets = vec![
        (
            pt("alpha"),
            vec![
                poll(question, answers, first),
                poll(question, answers, second),
            ],
        ),
        (
            pt("beta"),
            vec![poll(
                question,
                answers,
                votes_of("beta", "the high meadow", 5),
            )],
        ),
    ];
    assert_eq!(
        shares_lines(&sets),
        Err(format!(
            "the set alpha holds the question {} twice",
            question
        ))
    );
}

#[test]
fn an_empty_set_stands_beside_the_record_with_no_part() {
    let question = "Which gate should the fair open first?";
    let answers = &["the red gate", "the blue gate"];
    let mut first = votes_of("alpha", "the red gate", 3);
    first.push(response(
        "alpha hand the red gate 4",
        "model a",
        None,
        "no answer recorded",
    ));
    let mut second = votes_of("beta", "the red gate", 1);
    second.push(response(
        "beta hand the red gate 2",
        "model a",
        None,
        "no answer recorded",
    ));
    let held = vec![
        (pt("alpha"), vec![poll(question, answers, first)]),
        (pt("beta"), vec![poll(question, answers, second)]),
    ];
    let base_lines = shares_lines(&held).unwrap();
    let mut sets = held.clone();
    sets.push((pt("empty"), Vec::new()));
    let lines = shares_lines(&sets).unwrap();
    assert_eq!(
        lines.last().unwrap(),
        "shares, 3 sets, 2 parts, 0 whole, 0 none, 0 not counted"
    );
    let tail = lines.len();
    assert_eq!(lines[tail - 3], "set, empty");
    assert_eq!(
        lines[tail - 2],
        "standing, 0 parts, 0 whole, 0 none, 0 not counted"
    );
    let mut stripped = lines.clone();
    stripped.drain(tail - 3..tail - 1);
    assert_eq!(
        stripped[..stripped.len() - 1],
        base_lines[..base_lines.len() - 1]
    );
}

#[test]
fn the_empty_forms_of_the_reading_stand_at_zero_figures() {
    let lines = shares_lines(&[]).unwrap();
    assert_eq!(
        lines,
        expected(&["shares, 0 sets, 0 parts, 0 whole, 0 none, 0 not counted"])
    );
    let lines = shares_lines(&[(pt("alpha"), Vec::new())]).unwrap();
    assert_eq!(
        lines,
        expected(&[
            "set, alpha",
            "standing, 0 parts, 0 whole, 0 none, 0 not counted",
            "shares, 1 sets, 0 parts, 0 whole, 0 none, 0 not counted",
        ])
    );
    let lines = shares_lines(&[(pt("alpha"), Vec::new()), (pt("beta"), Vec::new())]).unwrap();
    assert_eq!(
        lines,
        expected(&[
            "set, alpha",
            "standing, 0 parts, 0 whole, 0 none, 0 not counted",
            "set, beta",
            "standing, 0 parts, 0 whole, 0 none, 0 not counted",
            "shares, 2 sets, 0 parts, 0 whole, 0 none, 0 not counted",
        ])
    );
}

#[test]
fn set_rules_refuse_the_empty_the_duplicate_the_newline_and_the_mixed_fixtures() {
    let question = "Which gate should the fair open first?";
    let answers = ["the red gate", "the blue gate"];
    let one = || poll(question, &answers, votes_of("one", "the red gate", 1));
    let sets = vec![(pt(""), vec![one()])];
    assert_eq!(shares_lines(&sets).unwrap_err(), "a set label is empty");
    let sets = vec![(pt("alpha"), vec![one()]), (pt("alpha"), vec![one()])];
    assert_eq!(
        shares_lines(&sets).unwrap_err(),
        "the set label alpha stands twice"
    );
    let sets = vec![(pt("alpha\nbeta"), vec![one()])];
    assert_eq!(
        shares_lines(&sets).unwrap_err(),
        "a set label holds a newline"
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
        shares_lines(&sets).unwrap_err(),
        "the set label beta stands twice"
    );
}

#[test]
fn guard_refuses_a_newline_in_the_model_of_the_second_set() {
    let mut sets = recorded_sets();
    sets[1].1[1].responses[0].model = pt("model a\nshares, forged");
    let defect = shares_lines(&sets).unwrap_err();
    assert_eq!(defect, "response 0 field model holds a newline");
    assert_eq!(census_lines(&sets).unwrap_err(), defect);
}

#[test]
fn shares_golden_differs_from_the_accord_and_pairs_goldens() {
    let sets = recorded_sets();
    let lines = shares_lines(&sets).unwrap();
    let golden = read_text(&examples().join("shares-golden.txt"));
    let golden_lines: Vec<String> = golden.lines().map(|line| line.to_string()).collect();
    assert_eq!(lines, golden_lines);
    let accord = read_text(&examples().join("accord-golden.txt"));
    let accord_lines_text: Vec<String> = accord.lines().map(|line| line.to_string()).collect();
    assert_ne!(lines, accord_lines_text);
    let pairs = read_text(&examples().join("pairs-golden.txt"));
    let pairs_lines_text: Vec<String> = pairs.lines().map(|line| line.to_string()).collect();
    assert_ne!(lines, pairs_lines_text);
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
    assert!(!verdict_words.contains(&"shares"));
    let golden = read_text(&examples().join("shares-golden.txt"));
    assert!(golden.contains("shares, 3 sets"));
    let openers = golden
        .lines()
        .filter(|line| line.starts_with("set, "))
        .count();
    assert_eq!(openers, 3);
    let mut lines: Vec<String> = golden.lines().map(|line| line.to_string()).collect();
    lines.extend(shares_lines(&generated_sets_in_memory()).unwrap());
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
fn help_text_names_the_shares_command() {
    let (stdout, _, code) = run_cli(&["shares", "--help"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("usage: grandjury shares [-h] label directory [label directory ...]"));
    let (stdout, _, code) = run_cli(&["--help"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("print the shares reading over recorded poll sets"));
    let open = stdout.find('{').unwrap();
    let close = stdout.find('}').unwrap();
    let named = stdout[open + 1..close].split(',').count();
    assert_eq!(named, 22);
}

#[test]
fn generated_family_holds_its_figures() {
    let sets = generated_sets_in_memory();
    let lines = shares_lines(&sets).unwrap();
    assert_eq!(lines.len(), 153);
    let mut standings: Vec<&str> = lines
        .iter()
        .filter(|line| line.starts_with("standing,"))
        .map(|line| line.as_str())
        .collect();
    standings.sort();
    let mut stated = vec![
        "standing, 19 parts, 1 whole, 0 none, 0 not counted",
        "standing, 20 parts, 4 whole, 0 none, 0 not counted",
        "standing, 14 parts, 3 whole, 0 none, 0 not counted",
        "standing, 19 parts, 4 whole, 1 none, 0 not counted",
    ];
    stated.sort();
    assert_eq!(standings, stated);
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.starts_with("set, "))
            .count(),
        4
    );
    assert_eq!(
        lines.last().unwrap(),
        "shares, 4 sets, 72 parts, 12 whole, 1 none, 0 not counted"
    );
}

#[test]
fn the_generated_family_loaded_from_files_stands_the_same() {
    assert_eq!(
        shares_lines(&generated_sets_loaded()).unwrap(),
        shares_lines(&generated_sets_in_memory()).unwrap()
    );
}

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
    let scratch = TempDir::new("shares-generated");
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
