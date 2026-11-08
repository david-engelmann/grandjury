//! The offline makers, the library halves of the scripts
//! under examples.
//!
//! The makers are offline assemblies over recorded
//! files. make_pilot and make_full rebuild the recorded
//! polls, their goldens, and their run records from the
//! questions files and the recorded envelopes through the seat
//! readers. The wave maker of GJ-9 rebuilds the second wave
//! polls and their run record from the pilot questions file
//! and the wave envelopes. The live calling half of a seat is
//! a maintainer action through the connector routes and is not
//! reimplemented here, as PORT-6 in CONTRACT.md states. Where
//! the Python maker raises on an input past its readings, the
//! port maker refuses with an error and writes nothing, and
//! never imitates the half rewritten tree a crashed Python
//! maker leaves behind.
//!
//! The file sets stand as contents here, a relative path and
//! its text for every file the Python script writes, so a
//! caller writes them whole or not at all.

use std::path::{Path, PathBuf};

use num_bigint::BigInt;

use crate::json::{
    dumps, has_int_past_digit_limit, has_unpaired_surrogate, parse, parse_exact, JsonValue,
};
use crate::polls::{cost_lines, load_prices_text, read_text_file, render_stadium, Poll};
use crate::pydecimal::{PyDecimal, Rounding};
use crate::pynum::Num;
use crate::pytext::PText;
use crate::seats::{response_from_chat_envelope, response_from_jev_envelope};

pub const SEATS: usize = 5;

/// A maker refusal, naming the input the maker could not read
/// the way the Python maker reads it.
#[derive(Debug)]
pub struct MakerError(pub String);

type MakerResult<T> = Result<T, MakerError>;

/// One seat reader of the routes the makers read envelopes by.
type SeatReader = fn(&PText, &JsonValue) -> Option<crate::polls::Response>;

fn refusal(what: &str) -> MakerError {
    MakerError(format!("the maker cannot read {}", what))
}

fn pt(text: &str) -> PText {
    PText::from_text(text)
}

fn field<'a>(value: &'a JsonValue, name: &str) -> MakerResult<&'a JsonValue> {
    value.get(name).ok_or_else(|| refusal(name))
}

fn envelope_path(dir: &Path, index: usize, prefix: &str, seat: usize) -> PathBuf {
    dir.join(format!("q{:02}-{}-{}.json", index, prefix, seat))
}

/// Read one envelope file as text, refusing where the Python
/// maker raises, on a missing file or on text that is not
/// UTF-8.
fn read_required(path: &Path) -> MakerResult<String> {
    read_text_file(path).ok_or_else(|| refusal(&path.display().to_string()))
}

/// Read one envelope file as text, or None where no file
/// stands, the full maker's attempted and failed seat.
fn read_present(path: &Path) -> MakerResult<Option<String>> {
    if !path.exists() {
        return Ok(None);
    }
    Ok(Some(read_required(path)?))
}

/// Parse recorded JSON text, refusing where the Python maker
/// raises.
fn parse_required(text: &str) -> MakerResult<JsonValue> {
    parse(text).map_err(|_| refusal("a recorded file as JSON"))
}

fn parse_exact_required(text: &str) -> MakerResult<JsonValue> {
    parse_exact(text).map_err(|_| refusal("a recorded file as JSON"))
}

/// The usage figure of an envelope as a number, the Python
/// addition reading a boolean as the integer it subclasses.
fn usage_number(value: &JsonValue) -> MakerResult<Num> {
    match value {
        JsonValue::Num(num) => Ok(num.clone()),
        JsonValue::Bool(true) => Ok(Num::from_i64(1)),
        JsonValue::Bool(false) => Ok(Num::from_i64(0)),
        _ => Err(refusal("a usage figure as a number")),
    }
}

/// Add one usage figure into a token total.
fn add_usage(total: &Num, value: &JsonValue) -> MakerResult<Num> {
    Ok(total.added(&usage_number(value)?))
}

/// The usage cost of an envelope as an exact decimal, the
/// Python sum reading a boolean as the integer it subclasses
/// and an integer as the decimal it names.
fn usage_cost(value: &JsonValue) -> MakerResult<PyDecimal> {
    match value {
        JsonValue::Num(Num::Int(int)) => Ok(PyDecimal::from_bigint(int.clone())),
        JsonValue::Num(Num::Decimal(decimal)) => Ok(decimal.clone()),
        JsonValue::Bool(true) => Ok(PyDecimal::from_bigint(BigInt::from(1))),
        JsonValue::Bool(false) => Ok(PyDecimal::zero()),
        _ => Err(refusal("a usage cost as a decimal")),
    }
}

/// Sum one route of a run record over the recorded envelopes.
///
/// The counts and the token totals cover the recorded run. The
/// dollar cost of the chat route is summed from the usage
/// figure of each envelope as an exact decimal under the
/// default context, precision 28 with halves rounding to even,
/// the discipline the Python sum runs under. A seat whose
/// envelope file is absent counts as attempted and failed when
/// the caller allows absence, and refuses the record where the
/// pilot inventory stands complete.
fn sum_route(
    dir: &Path,
    questions: &[JsonValue],
    prefix: &str,
    route: &str,
    allow_absent: bool,
    reader: fn(&PText, &JsonValue) -> Option<crate::polls::Response>,
) -> MakerResult<(PText, JsonValue)> {
    let mut attempted = 0usize;
    let mut recorded = 0usize;
    let mut failed = 0usize;
    let mut abstained = 0usize;
    let mut input_tokens = Num::from_i64(0);
    let mut output_tokens = Num::from_i64(0);
    let mut cost = PyDecimal::zero();
    let mut model_name: Option<PText> = None;
    for (offset, item) in questions.iter().enumerate() {
        let index = offset + 1;
        for seat in 1..=SEATS {
            attempted += 1;
            let path = envelope_path(dir, index, prefix, seat);
            let text = if allow_absent {
                match read_present(&path)? {
                    Some(text) => text,
                    None => {
                        failed += 1;
                        continue;
                    }
                }
            } else {
                read_required(&path)?
            };
            let envelope = parse_required(&text)?;
            let exact = parse_exact_required(&text)?;
            let model = match field(&envelope, "model")? {
                JsonValue::Str(text) => text.clone(),
                _ => return Err(refusal("an envelope model as a string")),
            };
            model_name = Some(model);
            let usage = field(&envelope, "usage")?;
            if usage.as_object().is_none() {
                return Err(refusal("an envelope usage as an object"));
            }
            if prefix == "jev" {
                input_tokens = add_usage(&input_tokens, field(usage, "input_tokens")?)?;
                output_tokens = add_usage(&output_tokens, field(usage, "output_tokens")?)?;
            } else {
                input_tokens = add_usage(&input_tokens, field(usage, "prompt_tokens")?)?;
                output_tokens = add_usage(&output_tokens, field(usage, "completion_tokens")?)?;
                let exact_usage = field(&exact, "usage")?;
                let price = usage_cost(field(exact_usage, "cost")?)?;
                cost = cost.added_ctx(&price, 28, Rounding::HalfEven);
            }
            let answers = field(item, "answers")?;
            let _ = answers;
            let juror = pt(&format!("{}-{}", prefix, seat));
            match reader(&juror, &envelope) {
                None => failed += 1,
                Some(response) => {
                    recorded += 1;
                    if response.abstained {
                        abstained += 1;
                    }
                }
            }
        }
    }
    let model_name = model_name.ok_or_else(|| refusal("a route with no envelope"))?;
    let mut entry = vec![
        (pt("route"), JsonValue::Str(pt(route))),
        (
            pt("calls_attempted"),
            JsonValue::Num(Num::from_i64(attempted as i64)),
        ),
        (
            pt("calls_recorded"),
            JsonValue::Num(Num::from_i64(recorded as i64)),
        ),
        (
            pt("failed_calls"),
            JsonValue::Num(Num::from_i64(failed as i64)),
        ),
        (
            pt("abstained"),
            JsonValue::Num(Num::from_i64(abstained as i64)),
        ),
        (pt("input_tokens"), JsonValue::Num(input_tokens)),
        (pt("output_tokens"), JsonValue::Num(output_tokens)),
    ];
    if prefix == "openai" {
        entry.push((pt("cost_dollars"), JsonValue::Str(pt(&cost.render()))));
    }
    Ok((model_name, JsonValue::object(entry)))
}

/// Build a run record over one recorded tree, the pilot shape
/// or the full shape by the absence rule.
fn build_run_record(
    tree: &Path,
    questions: &[JsonValue],
    allow_absent: bool,
) -> MakerResult<JsonValue> {
    let envelopes = tree.join("envelopes");
    let mut models: Vec<(PText, JsonValue)> = Vec::new();
    let (jev_name, jev_entry) = sum_route(
        &envelopes,
        questions,
        "jev",
        "jev",
        allow_absent,
        response_from_jev_envelope,
    )?;
    models.push((jev_name, jev_entry));
    let (chat_name, chat_entry) = sum_route(
        &envelopes,
        questions,
        "openai",
        "chat",
        allow_absent,
        response_from_chat_envelope,
    )?;
    models.push((chat_name, chat_entry));
    let incident_text = read_required(&tree.join("incident.json"))?;
    let incident = parse_required(&incident_text)?;
    let record = JsonValue::object(vec![
        (
            pt("questions"),
            JsonValue::Num(Num::from_i64(questions.len() as i64)),
        ),
        (
            pt("seats_per_question_per_model"),
            JsonValue::Num(Num::from_i64(SEATS as i64)),
        ),
        (pt("models"), JsonValue::object(models)),
        (pt("incident"), incident),
    ]);
    check_writable(&record)?;
    Ok(record)
}

/// Refuse a value the maker cannot carry, a summed integer
/// past the digit limit under PORT-6 or a string holding an
/// unpaired surrogate under the stated refusal of PORT-12,
/// which amends PORT-6 for that shape.
fn check_writable(value: &JsonValue) -> MakerResult<()> {
    if has_int_past_digit_limit(value) {
        return Err(refusal("a summed figure past the digit limit"));
    }
    if has_unpaired_surrogate(value) {
        return Err(refusal("a string holding an unpaired surrogate"));
    }
    Ok(())
}

/// Build one recorded poll from its recorded envelopes, the
/// seats in order, the Jev seats first and then the chat
/// seats. A seat whose call failed records nothing and leaves
/// no response.
fn build_poll(
    tree: &Path,
    index: usize,
    item: &JsonValue,
    allow_absent: bool,
) -> MakerResult<JsonValue> {
    let question = field(item, "question")?.clone();
    let answers = field(item, "answers")?.clone();
    let mut responses: Vec<JsonValue> = Vec::new();
    let routes: [(&str, SeatReader); 2] = [
        ("jev", response_from_jev_envelope),
        ("openai", response_from_chat_envelope),
    ];
    for (prefix, reader) in routes {
        for seat in 1..=SEATS {
            let path = envelope_path(&tree.join("envelopes"), index, prefix, seat);
            let text = if allow_absent {
                match read_present(&path)? {
                    Some(text) => text,
                    None => continue,
                }
            } else {
                read_required(&path)?
            };
            let envelope = parse_required(&text)?;
            let juror = pt(&format!("{}-{}", prefix, seat));
            if let Some(response) = reader(&juror, &envelope) {
                responses.push(response.to_json());
            }
        }
    }
    let poll = JsonValue::object(vec![
        (pt("question"), question),
        (pt("answers"), answers),
        (pt("responses"), JsonValue::Arr(responses)),
    ]);
    check_writable(&poll)?;
    Ok(poll)
}

/// Load the questions of one recorded tree.
fn load_questions(tree: &Path) -> MakerResult<Vec<JsonValue>> {
    let text = read_required(&tree.join("questions.json"))?;
    let value = parse_required(&text)?;
    match value.as_array() {
        Some(items) => Ok(items.clone()),
        None => Err(refusal("a questions file as an array")),
    }
}

/// Load a prices file beside the examples root into a table,
/// refusing where the Python maker asserts a clean load.
fn load_table(examples: &Path, name: &str) -> MakerResult<crate::polls::PriceTable> {
    let text = read_required(&examples.join(name))?;
    let (table, defects) = load_prices_text(&text);
    if !defects.is_empty() {
        return Err(refusal("a prices file that loads clean"));
    }
    table.ok_or_else(|| refusal("a prices file that loads clean"))
}

/// Render one golden text from a built poll value, the poll
/// read back through the loader so the golden is the rendering
/// of the poll the loader holds, as the Python maker renders
/// the poll it built.
fn golden_poll(poll_value: &JsonValue) -> MakerResult<Poll> {
    let text = dumps(poll_value, Some(2), false, false);
    let (poll, defects) = crate::polls::load_poll_text(&text);
    if !defects.is_empty() {
        return Err(refusal("a built poll that loads clean"));
    }
    poll.ok_or_else(|| refusal("a built poll that loads clean"))
}

fn join_lines(lines: &[String]) -> String {
    let mut out = lines.join("\n");
    out.push('\n');
    out
}

/// The files of the pilot maker, as relative paths and
/// contents, reproducing examples/make_pilot.py whole.
pub fn pilot_files(examples: &Path) -> MakerResult<Vec<(String, String)>> {
    let tree = examples.join("pilot");
    let questions = load_questions(&tree)?;
    let mut polls: Vec<JsonValue> = Vec::new();
    for (offset, item) in questions.iter().enumerate() {
        polls.push(build_poll(&tree, offset + 1, item, false)?);
    }
    let mut files: Vec<(String, String)> = Vec::new();
    for (offset, poll) in polls.iter().enumerate() {
        files.push((
            format!("pilot/poll-{:02}.json", offset + 1),
            format!("{}\n", dumps(poll, Some(2), false, false)),
        ));
    }
    let example = &polls[0];
    files.push((
        "pilot_poll.json".to_string(),
        format!("{}\n", dumps(example, Some(2), false, false)),
    ));
    let table = load_table(examples, "pilot-prices.json")?;
    let example_poll = golden_poll(example)?;
    let stadium =
        render_stadium(&example_poll).map_err(|_| refusal("a stadium view the reading renders"))?;
    files.push(("pilot-stadium-golden.txt".to_string(), join_lines(&stadium)));
    let example_cost = cost_lines(&example_poll, &table)
        .map_err(|_| refusal("a cost report the counter renders"))?
        .ok_or_else(|| refusal("a cost report the counter renders"))?;
    files.push((
        "pilot-cost-golden.txt".to_string(),
        join_lines(&example_cost),
    ));
    let record = build_run_record(&tree, &questions, false)?;
    files.push((
        "pilot/run_record.json".to_string(),
        format!("{}\n", dumps(&record, Some(2), false, false)),
    ));
    Ok(files)
}

/// The files of the full maker, as relative paths and
/// contents, reproducing examples/make_full.py whole. The
/// agreement golden is rendered through the agreement reading
/// over the polls this maker built, as the Python maker
/// renders it through the agreement command over the files it
/// wrote, and never over the poll files standing in the tree,
/// as PORT-12 states.
pub fn full_files(examples: &Path) -> MakerResult<Vec<(String, String)>> {
    let tree = examples.join("full");
    let questions = load_questions(&tree)?;
    let mut polls: Vec<JsonValue> = Vec::new();
    for (offset, item) in questions.iter().enumerate() {
        polls.push(build_poll(&tree, offset + 1, item, true)?);
    }
    let mut files: Vec<(String, String)> = Vec::new();
    for (offset, poll) in polls.iter().enumerate() {
        files.push((
            format!("full/poll-{:02}.json", offset + 1),
            format!("{}\n", dumps(poll, Some(2), false, false)),
        ));
    }
    let example = &polls[0];
    files.push((
        "full_poll.json".to_string(),
        format!("{}\n", dumps(example, Some(2), false, false)),
    ));
    let table = load_table(examples, "full-prices.json")?;
    let example_poll = golden_poll(example)?;
    let stadium =
        render_stadium(&example_poll).map_err(|_| refusal("a stadium view the reading renders"))?;
    files.push(("full-stadium-golden.txt".to_string(), join_lines(&stadium)));
    let example_cost = cost_lines(&example_poll, &table)
        .map_err(|_| refusal("a cost report the counter renders"))?
        .ok_or_else(|| refusal("a cost report the counter renders"))?;
    files.push((
        "full-cost-golden.txt".to_string(),
        join_lines(&example_cost),
    ));
    files.push((
        "full-agreement-golden.txt".to_string(),
        full_agreement_golden(&polls)?,
    ));
    let record = build_run_record(&tree, &questions, true)?;
    files.push((
        "full/run_record.json".to_string(),
        format!("{}\n", dumps(&record, Some(2), false, false)),
    ));
    Ok(files)
}

/// Render the agreement golden through the agreement reading
/// over the polls the maker built, each built value read back
/// through the loader the way the goldens of one poll are
/// read, so the golden stands over the built polls alone and
/// no file of the input tree reaches it, as PORT-12 states.
fn full_agreement_golden(polls: &[JsonValue]) -> MakerResult<String> {
    let mut built: Vec<Poll> = Vec::new();
    for poll in polls {
        built.push(golden_poll(poll)?);
    }
    Ok(join_lines(
        &crate::agreement::agreement_lines(&built).map_err(MakerError)?,
    ))
}

/// Build one recorded poll of the pilot tree, the function
/// the translated pilot pins call by name.
pub fn pilot_build_poll(examples: &Path, index: usize, item: &JsonValue) -> MakerResult<JsonValue> {
    build_poll(&examples.join("pilot"), index, item, false)
}

/// Build the run record of the pilot tree.
pub fn pilot_build_run_record(examples: &Path, questions: &[JsonValue]) -> MakerResult<JsonValue> {
    build_run_record(&examples.join("pilot"), questions, false)
}

/// Build one recorded poll of the full tree, the function the
/// translated full pins call by name.
pub fn full_build_poll(examples: &Path, index: usize, item: &JsonValue) -> MakerResult<JsonValue> {
    build_poll(&examples.join("full"), index, item, true)
}

/// Build the run record of the full tree.
pub fn full_build_run_record(examples: &Path, questions: &[JsonValue]) -> MakerResult<JsonValue> {
    build_run_record(&examples.join("full"), questions, true)
}

/// The files of the wave maker, as relative paths and
/// contents. The wave polls the pilot question set a second
/// time under GJ-9, so its questions load from the pilot tree
/// while its polls and its run record build over the wave
/// tree from the envelopes recorded there, under the absence
/// counting of the full maker. The wave carries no example
/// poll and no price table and no golden of one poll, so the
/// maker writes the twelve polls and the run record and
/// nothing else.
pub fn wave_files(examples: &Path) -> MakerResult<Vec<(String, String)>> {
    let questions = load_questions(&examples.join("pilot"))?;
    let tree = examples.join("wave");
    let mut polls: Vec<JsonValue> = Vec::new();
    for (offset, item) in questions.iter().enumerate() {
        polls.push(build_poll(&tree, offset + 1, item, true)?);
    }
    let mut files: Vec<(String, String)> = Vec::new();
    for (offset, poll) in polls.iter().enumerate() {
        files.push((
            format!("wave/poll-{:02}.json", offset + 1),
            format!("{}\n", dumps(poll, Some(2), false, false)),
        ));
    }
    let record = build_run_record(&tree, &questions, true)?;
    files.push((
        "wave/run_record.json".to_string(),
        format!("{}\n", dumps(&record, Some(2), false, false)),
    ));
    Ok(files)
}

/// Build one recorded poll of the wave tree.
pub fn wave_build_poll(examples: &Path, index: usize, item: &JsonValue) -> MakerResult<JsonValue> {
    build_poll(&examples.join("wave"), index, item, true)
}

/// Build the run record of the wave tree.
pub fn wave_build_run_record(examples: &Path, questions: &[JsonValue]) -> MakerResult<JsonValue> {
    build_run_record(&examples.join("wave"), questions, true)
}

// The synthetic maker of the port, which invented the
// example poll and the example price table by fixed
// arithmetic and assembled the four files they stand in,
// retired under FIX-GJ-19, which replaced every file it
// made with the resampled fixture and its renderings.
