//! Shared helpers for the translated integration tests.
//!
//! The Python corpus builds its inputs as Python values and
//! hands most of them to the surface through its JSON writer,
//! so these helpers build JSON values and render them through
//! the crate writer in the same default layout. Paths resolve
//! against the crate root, which is the repo root. A scratch
//! directory stands in for the Python tmp_path fixture and
//! removes itself when the test ends.

#![allow(dead_code)]

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use grandjury::json::{dumps, JsonValue};
use grandjury::pynum::Num;
use grandjury::pytext::PText;

pub fn pt(text: &str) -> PText {
    PText::from_text(text)
}

pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn examples() -> PathBuf {
    repo_root().join("examples")
}

pub fn read_text(path: &std::path::Path) -> String {
    std::fs::read_to_string(path).expect("a readable text file")
}

pub fn read_json(path: &std::path::Path) -> JsonValue {
    grandjury::json::parse(&read_text(path)).expect("a JSON file")
}

pub fn read_json_exact(path: &std::path::Path) -> JsonValue {
    grandjury::json::parse_exact(&read_text(path)).expect("a JSON file")
}

pub fn jstr(text: &str) -> JsonValue {
    JsonValue::Str(pt(text))
}

/// A JSON string holding the given code units, for the
/// strings a Rust string cannot hold, such as one carrying
/// an unpaired surrogate.
pub fn jstr_units(units: &[u16]) -> JsonValue {
    JsonValue::Str(PText::from_units(units.to_vec()))
}

pub fn jint(value: i64) -> JsonValue {
    JsonValue::Num(Num::from_i64(value))
}

pub fn jfloat(value: f64) -> JsonValue {
    JsonValue::Num(Num::Float(value))
}

pub fn jbool(value: bool) -> JsonValue {
    JsonValue::Bool(value)
}

pub fn jarr(items: Vec<JsonValue>) -> JsonValue {
    JsonValue::Arr(items)
}

pub fn jobj(pairs: Vec<(&str, JsonValue)>) -> JsonValue {
    JsonValue::object(
        pairs
            .into_iter()
            .map(|(key, value)| (pt(key), value))
            .collect(),
    )
}

/// Render a value the way the Python test corpus renders the
/// values it hands over, the default writer layout with the
/// ASCII escaping on.
pub fn dumps_default(value: &JsonValue) -> String {
    dumps(value, None, false, true)
}

/// Fetch one field of a JSON object, failing the test when it
/// is absent.
pub fn field<'a>(value: &'a JsonValue, key: &str) -> &'a JsonValue {
    value
        .get(key)
        .unwrap_or_else(|| panic!("field {} stands in the value", key))
}

pub fn as_int(value: &JsonValue) -> i64 {
    match value {
        JsonValue::Num(Num::Int(int)) => {
            let text = int.to_string();
            text.parse::<i64>().expect("an int that fits")
        }
        other => panic!("an int value, got {:?}", other),
    }
}

pub fn as_str(value: &JsonValue) -> String {
    match value {
        JsonValue::Str(text) => text.display(),
        other => panic!("a string value, got {:?}", other),
    }
}

/// Deep equality in the shape of the Python value equality
/// the corpus pins with, object fields unordered and numbers
/// equal across their shapes.
pub fn json_eq(a: &JsonValue, b: &JsonValue) -> bool {
    match (a, b) {
        (JsonValue::Obj(left), JsonValue::Obj(right)) => {
            if left.len() != right.len() {
                return false;
            }
            left.iter().all(|(key, value)| match b.get(&key.display()) {
                Some(other) => json_eq(value, other),
                None => false,
            })
        }
        (JsonValue::Arr(left), JsonValue::Arr(right)) => {
            left.len() == right.len() && left.iter().zip(right.iter()).all(|(x, y)| json_eq(x, y))
        }
        _ => a == b,
    }
}

pub fn assert_json_eq(a: &JsonValue, b: &JsonValue) {
    assert!(
        json_eq(a, b),
        "values differ\nleft {}\nright {}",
        dumps_default(a),
        dumps_default(b)
    );
}

/// A scratch directory that removes itself on drop, the
/// tmp_path of the Python corpus.
pub struct TempDir {
    pub path: PathBuf,
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);

impl TempDir {
    pub fn new(tag: &str) -> TempDir {
        let serial = COUNTER.fetch_add(1, Ordering::SeqCst);
        let path = std::env::temp_dir().join(format!(
            "grandjury-test-{}-{}-{}",
            std::process::id(),
            tag,
            serial
        ));
        std::fs::create_dir_all(&path).expect("a scratch directory");
        TempDir { path }
    }

    pub fn write(&self, name: &str, text: &str) -> PathBuf {
        let path = self.path.join(name);
        std::fs::write(&path, text).expect("a written file");
        path
    }

    pub fn write_json(&self, name: &str, value: &JsonValue) -> PathBuf {
        self.write(name, &dumps_default(value))
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

/// Run the command line surface over text arguments at the
/// default width, returning stdout, stderr, and the status.
pub fn run_cli(args: &[&str]) -> (String, String, i32) {
    run_cli_at(args, 80)
}

pub fn run_cli_at(args: &[&str], columns: usize) -> (String, String, i32) {
    let held: Vec<PText> = args.iter().map(|arg| pt(arg)).collect();
    let outcome = grandjury::cli::run(&held, columns);
    (
        String::from_utf8(outcome.stdout).expect("stdout text"),
        String::from_utf8(outcome.stderr).expect("stderr text"),
        outcome.code,
    )
}

pub fn path_arg(path: &std::path::Path) -> String {
    path.to_string_lossy().into_owned()
}
