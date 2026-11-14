//! The grandjury command line surface, as a library function.
//!
//! Twenty two commands work on recorded polls. The dispatch mirrors
//! the Python argparse surface of the same name, the help and
//! usage text wraps at the width the Python surface wraps at
//! under PORT-7, the arguments are carried as the code unit
//! sequences the operating system bytes decode to under the
//! surrogate escape reading, and a usage refusal echoes an
//! argument in the form the Python refusal echoes it. The
//! function returns the stdout bytes, the stderr bytes, and
//! the exit status, and main.rs is a thin shell over it.

use std::path::PathBuf;

use crate::accord::accord_lines;
use crate::aggregate::aggregate_lines;
use crate::agreement::agreement_lines;
use crate::answers::answers_lines;
use crate::bench::bench_lines;
use crate::canvass::canvass_lines;
use crate::census::census_lines;
use crate::compare::compare_lines;
use crate::dissent::dissent_lines;
use crate::ledger::ledger_lines;
use crate::margin::margin_lines;
use crate::minority::minority_lines;
use crate::muster::muster_lines;
use crate::pairs::pairs_lines;
use crate::polls::{
    cost_lines, load_poll_file, load_prices_file, render_stadium, render_tally_lines, Poll,
};
use crate::pool::pool_lines;
use crate::pytext::PText;
use crate::roll::roll_lines;
use crate::shares::shares_lines;
use crate::standing::standing_lines;
use crate::swing::swing_lines;

/// The outcome of one command line run.
pub struct Outcome {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub code: i32,
}

impl Outcome {
    fn plain(lines: Vec<String>, code: i32) -> Outcome {
        let mut stdout = Vec::new();
        for line in lines {
            stdout.extend_from_slice(line.as_bytes());
            stdout.push(b'\n');
        }
        Outcome {
            stdout,
            stderr: Vec::new(),
            code,
        }
    }
}

/// Decode one argument's operating system bytes the way the
/// Python runtime decodes them, valid UTF-8 to the characters
/// it names and every undecodable byte to its surrogate escape
/// code unit.
pub fn decode_arg(bytes: &[u8]) -> PText {
    let mut units: Vec<u16> = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        match std::str::from_utf8(&bytes[index..]) {
            Ok(rest) => {
                units.extend(rest.encode_utf16());
                break;
            }
            Err(error) => {
                let valid = error.valid_up_to();
                let head = std::str::from_utf8(&bytes[index..index + valid])
                    .expect("the valid prefix is valid");
                units.extend(head.encode_utf16());
                let skip = error.error_len().unwrap_or(bytes.len() - (index + valid));
                for offset in 0..skip {
                    units.push(0xDC00 + bytes[index + valid + offset] as u16);
                }
                index += valid + skip;
            }
        }
    }
    PText::from_units(units)
}

/// Encode one argument back to the operating system bytes it
/// names, the inverse of the decode, for opening the file it
/// names.
fn arg_bytes(text: &PText) -> Vec<u8> {
    let units = text.units();
    let mut out = Vec::new();
    let mut index = 0;
    while index < units.len() {
        let unit = units[index];
        if (0xDC80..=0xDCFF).contains(&unit) {
            // The surrogate escape range runs from the escape of
            // byte 0x80 through the escape of byte 0xFF, its end
            // included, as the Python reading spans it.
            out.push((unit - 0xDC00) as u8);
            index += 1;
        } else if (0xD800..0xDC00).contains(&unit)
            && index + 1 < units.len()
            && (0xDC00..0xE000).contains(&units[index + 1])
        {
            let point =
                0x10000 + ((((unit as u32) - 0xD800) << 10) | ((units[index + 1] as u32) - 0xDC00));
            push_utf8(&mut out, char::from_u32(point).unwrap_or('\u{FFFD}'));
            index += 2;
        } else if (0xD800..0xE000).contains(&unit) {
            // A lone surrogate that did not arrive as an escaped
            // byte names no file; the byte stands in so the open
            // simply fails, as an unencodable path fails.
            out.push(b'?');
            index += 1;
        } else {
            push_utf8(&mut out, char::from_u32(unit as u32).unwrap_or('\u{FFFD}'));
            index += 1;
        }
    }
    out
}

fn push_utf8(out: &mut Vec<u8>, c: char) {
    let mut buffer = [0u8; 4];
    out.extend_from_slice(c.encode_utf8(&mut buffer).as_bytes());
}

#[cfg(unix)]
fn path_from_arg(text: &PText) -> PathBuf {
    use std::os::unix::ffi::OsStringExt;
    let bytes = arg_bytes(text);
    if bytes.is_empty() {
        // The Python path reading reads an empty path as the
        // current directory.
        return PathBuf::from(".");
    }
    PathBuf::from(std::ffi::OsString::from_vec(bytes))
}

#[cfg(not(unix))]
fn path_from_arg(text: &PText) -> PathBuf {
    PathBuf::from(text.display())
}

/// The width the help and usage text wraps at, under the rule
/// PORT-7 states. The COLUMNS value when it is set and
/// positive, else the terminal width, else eighty.
pub fn terminal_columns() -> usize {
    if let Ok(value) = std::env::var("COLUMNS") {
        if let Some(columns) = parse_columns(&value) {
            return columns;
        }
    }
    terminal_width().unwrap_or(80)
}

/// Read a COLUMNS value the way the Python surface reads it.
///
/// The Python surface converts the value with its int
/// conversion, whose grammar the port mirrors here. The
/// conversion strips the whitespace set of the Python string
/// reading from both ends, takes one leading sign, and then
/// takes digits with single underscores standing between
/// digits and nowhere else, where a digit is any code point
/// of category Nd at its decimal value. A value the
/// conversion refuses, and a value of zero or below, names no
/// width. A value past the stored bound settles at the bound,
/// 1000000 columns, past which every layout this surface
/// wraps is already at its widest and no printed line can
/// change.
fn parse_columns(value: &str) -> Option<usize> {
    let held: Vec<char> = value.chars().collect();
    let mut start = 0;
    let mut end = held.len();
    while start < end && crate::pyunicode::is_stripped_space(held[start] as u32) {
        start += 1;
    }
    while end > start && crate::pyunicode::is_stripped_space(held[end - 1] as u32) {
        end -= 1;
    }
    let mut rest: &[char] = &held[start..end];
    let mut negative = false;
    if let Some(first) = rest.first() {
        if *first == '+' || *first == '-' {
            negative = *first == '-';
            rest = &rest[1..];
        }
    }
    let mut magnitude: u64 = 0;
    let mut digits = 0usize;
    let mut underscores = 0usize;
    for point in rest {
        if *point == '_' {
            underscores += 1;
            continue;
        }
        let digit = crate::pyunicode::nd_digit_value(*point as u32)?;
        if underscores > 1 || (underscores == 1 && digits == 0) {
            return None;
        }
        underscores = 0;
        digits += 1;
        magnitude = magnitude
            .saturating_mul(10)
            .saturating_add(digit as u64)
            .min(1_000_000);
    }
    if digits == 0 || underscores > 0 || negative || magnitude == 0 {
        return None;
    }
    Some(magnitude as usize)
}

#[cfg(unix)]
fn terminal_width() -> Option<usize> {
    #[repr(C)]
    struct Winsize {
        ws_row: u16,
        ws_col: u16,
        ws_xpixel: u16,
        ws_ypixel: u16,
    }
    extern "C" {
        fn ioctl(fd: i32, request: u64, ...) -> i32;
    }
    let mut size = Winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    // TIOCGWINSZ on Linux.
    let result = unsafe { ioctl(1, 0x5413, &mut size as *mut Winsize) };
    if result == 0 && size.ws_col > 0 {
        Some(size.ws_col as usize)
    } else {
        None
    }
}

#[cfg(not(unix))]
fn terminal_width() -> Option<usize> {
    None
}

/// One action line of a help screen.
struct ActionSpec {
    invocation: &'static str,
    help: Option<&'static str>,
    subactions: Vec<ActionSpec>,
}

/// One parser of the surface, described for the formatter.
struct ParserSpec {
    prog: &'static str,
    description: Option<&'static str>,
    opt_usage: &'static str,
    pos_usage: &'static str,
    opt_parts: Vec<&'static str>,
    pos_parts: Vec<&'static str>,
    positionals: Vec<ActionSpec>,
    options: Vec<ActionSpec>,
    max_invocation: usize,
}

fn action(invocation: &'static str, help: Option<&'static str>) -> ActionSpec {
    ActionSpec {
        invocation,
        help,
        subactions: Vec::new(),
    }
}

fn help_action() -> ActionSpec {
    action("-h, --help", Some("show this help message and exit"))
}

fn spec_max_invocation(spec: &ParserSpec) -> usize {
    let mut max = 0;
    for item in spec.positionals.iter().chain(spec.options.iter()) {
        max = max.max(item.invocation.len());
        for sub in &item.subactions {
            max = max.max(sub.invocation.len());
        }
    }
    max
}

const COMMANDS: [&str; 22] = [
    "tally",
    "stadium",
    "cost",
    "agreement",
    "compare",
    "census",
    "ledger",
    "dissent",
    "swing",
    "minority",
    "bench",
    "pool",
    "aggregate",
    "canvass",
    "roll",
    "muster",
    "margin",
    "accord",
    "standing",
    "answers",
    "pairs",
    "shares",
];

fn top_spec() -> ParserSpec {
    let mut commands = action(
        "{tally,stadium,cost,agreement,compare,census,ledger,dissent,swing,minority,bench,pool,aggregate,canvass,roll,muster,margin,accord,standing,answers,pairs,shares}",
        None,
    );
    commands.subactions = vec![
        action("tally", Some("print the tally of a recorded poll")),
        action("stadium", Some("print the stadium view of a recorded poll")),
        action("cost", Some("print the cost report of a recorded poll")),
        action("agreement", Some("print the agreement reading over polls")),
        action(
            "compare",
            Some("print the wave comparison reading over two poll sets"),
        ),
        action(
            "census",
            Some("print the census reading over recorded poll sets"),
        ),
        action(
            "ledger",
            Some("print the juror ledger reading over recorded poll sets"),
        ),
        action(
            "dissent",
            Some("print the dissent reading over recorded poll sets"),
        ),
        action(
            "swing",
            Some("print the swing reading over recorded poll sets"),
        ),
        action(
            "minority",
            Some("print the minority reading over recorded poll sets"),
        ),
        action(
            "bench",
            Some("print the bench reading over recorded poll sets"),
        ),
        action(
            "pool",
            Some("print the pool reading over recorded poll sets"),
        ),
        action(
            "aggregate",
            Some("print the aggregate reading over recorded poll sets"),
        ),
        action(
            "canvass",
            Some("print the canvass reading over recorded poll sets"),
        ),
        action(
            "roll",
            Some("print the roll reading over recorded poll sets"),
        ),
        action(
            "muster",
            Some("print the muster reading over recorded poll sets"),
        ),
        action(
            "margin",
            Some("print the margin reading over recorded poll sets"),
        ),
        action(
            "accord",
            Some("print the accord reading over recorded poll sets"),
        ),
        action(
            "standing",
            Some("print the standing reading over recorded poll sets"),
        ),
        action(
            "answers",
            Some("print the answers reading over recorded poll sets"),
        ),
        action(
            "pairs",
            Some("print the pairing reading over recorded poll sets"),
        ),
        action(
            "shares",
            Some("print the shares reading over recorded poll sets"),
        ),
    ];
    let mut spec = ParserSpec {
        prog: "grandjury",
        description: Some("Poll tiny models and render the stadium vote."),
        opt_usage: "[-h]",
        pos_usage:
            "{tally,stadium,cost,agreement,compare,census,ledger,dissent,swing,minority,bench,pool,aggregate,canvass,roll,muster,margin,accord,standing,answers,pairs,shares} ...",
        opt_parts: vec!["[-h]"],
        pos_parts: vec![
            "{tally,stadium,cost,agreement,compare,census,ledger,dissent,swing,minority,bench,pool,aggregate,canvass,roll,muster,margin,accord,standing,answers,pairs,shares}",
            "...",
        ],
        positionals: vec![commands],
        options: vec![help_action()],
        max_invocation: 0,
    };
    spec.max_invocation = spec_max_invocation(&spec);
    spec
}

fn sub_spec(command: &str) -> ParserSpec {
    let (prog, positionals, pos_usage, pos_parts): (
        &'static str,
        Vec<ActionSpec>,
        &'static str,
        Vec<&'static str>,
    ) = match command {
        "tally" => (
            "grandjury tally",
            vec![action("poll_file", None)],
            "poll_file",
            vec!["poll_file"],
        ),
        "stadium" => (
            "grandjury stadium",
            vec![action("poll_file", None)],
            "poll_file",
            vec!["poll_file"],
        ),
        "cost" => (
            "grandjury cost",
            vec![action("poll_file", None), action("prices_file", None)],
            "poll_file prices_file",
            vec!["poll_file", "prices_file"],
        ),
        "agreement" => (
            "grandjury agreement",
            vec![action("poll_files", None)],
            "poll_files [poll_files ...]",
            vec!["poll_files", "[poll_files ...]"],
        ),
        "compare" => (
            "grandjury compare",
            vec![action("first_dir", None), action("second_dir", None)],
            "first_dir second_dir",
            vec!["first_dir", "second_dir"],
        ),
        "census" => (
            "grandjury census",
            vec![action("label", None), action("directory", None)],
            "label directory [label directory ...]",
            vec!["label", "directory", "[label", "directory", "...]"],
        ),
        "ledger" => (
            "grandjury ledger",
            vec![action("label", None), action("directory", None)],
            "label directory [label directory ...]",
            vec!["label", "directory", "[label", "directory", "...]"],
        ),
        "dissent" => (
            "grandjury dissent",
            vec![action("label", None), action("directory", None)],
            "label directory [label directory ...]",
            vec!["label", "directory", "[label", "directory", "...]"],
        ),
        "swing" => (
            "grandjury swing",
            vec![action("label", None), action("directory", None)],
            "label directory [label directory ...]",
            vec!["label", "directory", "[label", "directory", "...]"],
        ),
        "minority" => (
            "grandjury minority",
            vec![action("label", None), action("directory", None)],
            "label directory [label directory ...]",
            vec!["label", "directory", "[label", "directory", "...]"],
        ),
        "bench" => (
            "grandjury bench",
            vec![action("label", None), action("directory", None)],
            "label directory [label directory ...]",
            vec!["label", "directory", "[label", "directory", "...]"],
        ),
        "pool" => (
            "grandjury pool",
            vec![action("label", None), action("directory", None)],
            "label directory [label directory ...]",
            vec!["label", "directory", "[label", "directory", "...]"],
        ),
        "aggregate" => (
            "grandjury aggregate",
            vec![action("label", None), action("directory", None)],
            "label directory [label directory ...]",
            vec!["label", "directory", "[label", "directory", "...]"],
        ),
        "canvass" => (
            "grandjury canvass",
            vec![action("label", None), action("directory", None)],
            "label directory [label directory ...]",
            vec!["label", "directory", "[label", "directory", "...]"],
        ),
        "roll" => (
            "grandjury roll",
            vec![action("label", None), action("directory", None)],
            "label directory [label directory ...]",
            vec!["label", "directory", "[label", "directory", "...]"],
        ),
        "muster" => (
            "grandjury muster",
            vec![action("label", None), action("directory", None)],
            "label directory [label directory ...]",
            vec!["label", "directory", "[label", "directory", "...]"],
        ),
        "margin" => (
            "grandjury margin",
            vec![action("label", None), action("directory", None)],
            "label directory [label directory ...]",
            vec!["label", "directory", "[label", "directory", "...]"],
        ),
        "accord" => (
            "grandjury accord",
            vec![action("label", None), action("directory", None)],
            "label directory [label directory ...]",
            vec!["label", "directory", "[label", "directory", "...]"],
        ),
        "standing" => (
            "grandjury standing",
            vec![action("label", None), action("directory", None)],
            "label directory [label directory ...]",
            vec!["label", "directory", "[label", "directory", "...]"],
        ),
        "answers" => (
            "grandjury answers",
            vec![action("label", None), action("directory", None)],
            "label directory [label directory ...]",
            vec!["label", "directory", "[label", "directory", "...]"],
        ),
        "pairs" => (
            "grandjury pairs",
            vec![action("label", None), action("directory", None)],
            "label directory [label directory ...]",
            vec!["label", "directory", "[label", "directory", "...]"],
        ),
        "shares" => (
            "grandjury shares",
            vec![action("label", None), action("directory", None)],
            "label directory [label directory ...]",
            vec!["label", "directory", "[label", "directory", "...]"],
        ),
        _ => unreachable!("a known command"),
    };
    let mut spec = ParserSpec {
        prog,
        description: None,
        opt_usage: "[-h]",
        pos_usage,
        opt_parts: vec!["[-h]"],
        pos_parts,
        positionals,
        options: vec![help_action()],
        max_invocation: 0,
    };
    spec.max_invocation = spec_max_invocation(&spec);
    spec
}

/// Wrap one text the way the Python text wrapper wraps it, a
/// greedy word wrap at the given width with a word longer than
/// the width split at the width.
fn text_wrap(text: &str, width: i64) -> Vec<String> {
    if width <= 0 {
        return Vec::new();
    }
    let width = width as usize;
    let mut lines: Vec<String> = Vec::new();
    let mut current = String::new();
    for word in text.split(' ') {
        if word.is_empty() {
            continue;
        }
        let mut rest = word;
        loop {
            if current.is_empty() {
                if rest.len() <= width {
                    current = rest.to_string();
                    rest = "";
                } else {
                    lines.push(rest[..width].to_string());
                    rest = &rest[width..];
                }
            } else if current.len() + 1 + rest.len() <= width {
                current.push(' ');
                current.push_str(rest);
                rest = "";
            } else {
                lines.push(std::mem::take(&mut current));
            }
            if rest.is_empty() {
                break;
            }
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

/// The usage block of a parser at a width, wrapped when it
/// overflows, in the form both the help screen and the refusal
/// lines carry.
fn format_usage(spec: &ParserSpec, width: i64) -> String {
    let prefix = "usage: ";
    let full = format!("{} {} {}", spec.prog, spec.opt_usage, spec.pos_usage);
    let usage = if (prefix.len() + full.len()) as i64 > width {
        wrap_usage(spec, width)
    } else {
        full
    };
    format!("{}{}", prefix, usage)
}

fn get_lines(parts: &[&str], indent: &str, prefix: Option<&str>, text_width: i64) -> Vec<String> {
    let indent_length = indent.len() as i64;
    let mut lines: Vec<String> = Vec::new();
    let mut line: Vec<&str> = Vec::new();
    let mut line_len: i64 = match prefix {
        Some(prefix) => prefix.len() as i64 - 1,
        None => indent_length - 1,
    };
    for part in parts {
        if line_len + 1 + part.len() as i64 > text_width && !line.is_empty() {
            lines.push(format!("{}{}", indent, line.join(" ")));
            line = Vec::new();
            line_len = indent_length - 1;
        }
        line.push(part);
        line_len += part.len() as i64 + 1;
    }
    if !line.is_empty() {
        lines.push(format!("{}{}", indent, line.join(" ")));
    }
    if prefix.is_some() && !lines.is_empty() {
        lines[0] = lines[0][indent.len()..].to_string();
    }
    lines
}

fn wrap_usage(spec: &ParserSpec, text_width: i64) -> String {
    let prefix = "usage: ";
    let prog = spec.prog;
    let short = (prefix.len() + prog.len()) as f64 <= 0.75 * text_width as f64;
    let lines: Vec<String> = if short {
        let indent = " ".repeat(prefix.len() + prog.len() + 1);
        if !spec.opt_parts.is_empty() {
            let mut first: Vec<&str> = vec![prog];
            first.extend(spec.opt_parts.iter());
            let mut lines = get_lines(&first, &indent, Some(prefix), text_width);
            lines.extend(get_lines(&spec.pos_parts, &indent, None, text_width));
            lines
        } else if !spec.pos_parts.is_empty() {
            let mut first: Vec<&str> = vec![prog];
            first.extend(spec.pos_parts.iter());
            get_lines(&first, &indent, Some(prefix), text_width)
        } else {
            vec![prog.to_string()]
        }
    } else {
        let indent = " ".repeat(prefix.len());
        let mut all: Vec<&str> = Vec::new();
        all.extend(spec.opt_parts.iter());
        all.extend(spec.pos_parts.iter());
        let mut lines = get_lines(&all, &indent, None, text_width);
        if lines.len() > 1 {
            lines = get_lines(&spec.opt_parts, &indent, None, text_width);
            lines.extend(get_lines(&spec.pos_parts, &indent, None, text_width));
        }
        let mut with_prog = vec![prog.to_string()];
        with_prog.extend(lines);
        with_prog
    };
    lines.join("\n")
}

fn format_action(
    spec_action: &ActionSpec,
    indent: usize,
    help_position: i64,
    width: i64,
    out: &mut String,
) {
    let help_width = (width - help_position).max(11);
    let action_width = help_position - indent as i64 - 2;
    match spec_action.help {
        None => {
            out.push_str(&" ".repeat(indent));
            out.push_str(spec_action.invocation);
            out.push('\n');
        }
        Some(help) => {
            let help_lines = text_wrap(help, help_width);
            if (spec_action.invocation.len() as i64) <= action_width {
                out.push_str(&" ".repeat(indent));
                out.push_str(&format!(
                    "{:<width$}",
                    spec_action.invocation,
                    width = action_width as usize
                ));
                out.push_str("  ");
                if let Some(first) = help_lines.first() {
                    out.push_str(first);
                }
                out.push('\n');
                for line in help_lines.iter().skip(1) {
                    out.push_str(&" ".repeat(help_position as usize));
                    out.push_str(line);
                    out.push('\n');
                }
            } else {
                out.push_str(&" ".repeat(indent));
                out.push_str(spec_action.invocation);
                out.push('\n');
                for line in help_lines.iter() {
                    out.push_str(&" ".repeat(help_position as usize));
                    out.push_str(line);
                    out.push('\n');
                }
            }
        }
    }
    for sub in &spec_action.subactions {
        format_action(sub, indent + 2, help_position, width, out);
    }
}

/// The full help text of a parser at the given terminal
/// columns, the text printed to stdout with a trailing newline
/// on its last line.
fn format_help(spec: &ParserSpec, columns: usize) -> String {
    let width = columns as i64 - 2;
    let max_help_position = 24.min((width - 20).max(4));
    let help_position = (spec.max_invocation as i64 + 4).min(max_help_position);
    let mut out = String::new();
    out.push_str(&format_usage(spec, width));
    out.push_str("\n\n");
    if let Some(description) = spec.description {
        // The text formatter floors its width at eleven, as
        // the Python formatter does for the description.
        for (index, line) in text_wrap(description, width.max(11)).iter().enumerate() {
            if index > 0 {
                out.push('\n');
            }
            out.push_str(line);
        }
        out.push_str("\n\n");
    }
    let sections: [(&str, &Vec<ActionSpec>); 2] = [
        ("positional arguments", &spec.positionals),
        ("options", &spec.options),
    ];
    for (index, (heading, actions)) in sections.iter().enumerate() {
        if index > 0 {
            out.push('\n');
        }
        out.push_str(heading);
        out.push_str(":\n");
        for item in actions.iter() {
            format_action(item, 2, help_position, width, &mut out);
        }
    }
    out
}

/// A usage refusal, the usage block and the error line on
/// stderr with exit status 2.
fn usage_error(spec: &ParserSpec, columns: usize, message: &str) -> Outcome {
    let width = columns as i64 - 2;
    let mut stderr = Vec::new();
    stderr.extend_from_slice(format_usage(spec, width).as_bytes());
    stderr.push(b'\n');
    stderr.extend_from_slice(format!("{}: error: {}\n", spec.prog, message).as_bytes());
    Outcome {
        stdout: Vec::new(),
        stderr,
        code: 2,
    }
}

/// The echo form of an argument in a plain, unquoted error
/// message. The text renders as itself, with every unpaired
/// surrogate rendered as its unicode escape, the form the
/// Python writes to its error stream under backslashreplace.
fn echo_text(text: &PText) -> String {
    let units = text.units();
    let mut out = String::new();
    let mut index = 0;
    while index < units.len() {
        let unit = units[index];
        if (0xD800..0xDC00).contains(&unit)
            && index + 1 < units.len()
            && (0xDC00..0xE000).contains(&units[index + 1])
        {
            let point =
                0x10000 + ((((unit as u32) - 0xD800) << 10) | ((units[index + 1] as u32) - 0xDC00));
            out.push(char::from_u32(point).unwrap_or('\u{FFFD}'));
            index += 2;
        } else if (0xD800..0xE000).contains(&unit) {
            out.push_str(&format!("\\u{:04x}", unit));
            index += 1;
        } else {
            out.push(char::from_u32(unit as u32).unwrap_or('\u{FFFD}'));
            index += 1;
        }
    }
    out
}

/// The text form the Python repr gives a string, which the
/// invalid choice refusal and the ignored explicit argument
/// refusal echo the refused argument in. One walk applies the
/// Python rule whole. The quote is a double quote when the
/// text holds a single quote and no double quote, and a
/// single quote otherwise, and the chosen quote is escaped
/// where it stands in the text. The backslash and the three
/// named control characters take their short escapes. Every
/// other code point stands as itself when the Python
/// printable test holds it printable, read through the
/// category table in pyunicode, and takes its hexadecimal
/// escape at its width otherwise. An unpaired surrogate takes
/// its escape at the four digit width, as the repr of the
/// Python string renders it.
fn py_repr(text: &PText) -> String {
    let units = text.units();
    let has_single = units.contains(&0x27);
    let has_double = units.contains(&0x22);
    let quote = if has_single && !has_double { '"' } else { '\'' };
    let mut out = String::new();
    out.push(quote);
    let mut index = 0;
    while index < units.len() {
        let unit = units[index];
        if (0xD800..0xDC00).contains(&unit)
            && index + 1 < units.len()
            && (0xDC00..0xE000).contains(&units[index + 1])
        {
            let point =
                0x10000 + ((((unit as u32) - 0xD800) << 10) | ((units[index + 1] as u32) - 0xDC00));
            push_repr_point(&mut out, point, quote);
            index += 2;
            continue;
        }
        if (0xD800..0xE000).contains(&unit) {
            out.push_str(&format!("\\u{:04x}", unit));
            index += 1;
            continue;
        }
        push_repr_point(&mut out, unit as u32, quote);
        index += 1;
    }
    out.push(quote);
    out
}

fn push_repr_point(out: &mut String, point: u32, quote: char) {
    match point {
        0x5C => out.push_str("\\\\"),
        0x0A => out.push_str("\\n"),
        0x0D => out.push_str("\\r"),
        0x09 => out.push_str("\\t"),
        point if point == quote as u32 => {
            out.push('\\');
            out.push(char::from_u32(point).expect("the quote is a character"));
        }
        point if crate::pyunicode::is_printable(point) => {
            out.push(char::from_u32(point).expect("a printable point is a character"));
        }
        point if point < 0x100 => out.push_str(&format!("\\x{:02x}", point)),
        point if point < 0x10000 => out.push_str(&format!("\\u{:04x}", point)),
        point => out.push_str(&format!("\\U{:08x}", point)),
    }
}

/// Return whether a token reads as an option rather than a
/// value, under the argparse reading, a leading dash on more
/// than one character that does not start a negative number.
fn is_option_like(text: &PText) -> bool {
    let units = text.units();
    if units.len() < 2 || units[0] != 0x2D {
        return false;
    }
    if units[1] >= 0x30 && units[1] <= 0x39 {
        return false;
    }
    if units[1] == 0x2E && units.len() > 2 && units[2] >= 0x30 && units[2] <= 0x39 {
        return false;
    }
    true
}

fn text_is(text: &PText, word: &str) -> bool {
    *text == PText::from_text(word)
}

fn starts_with_dashdash(text: &PText) -> bool {
    let units = text.units();
    units.len() >= 2 && units[0] == 0x2D && units[1] == 0x2D
}

/// The outcome of matching one long option token against the
/// long options of a parser, of which this surface holds one.
enum LongMatch {
    Help,
    Unknown,
    HelpWithValue(PText),
}

/// Match a token that starts with two dashes against the long
/// options in play. An unambiguous prefix matches, as the
/// Python surface matches it, and a value rides after an
/// equals sign. The match works on the code units of the
/// token, so a value carrying a byte no encoding accepts rides
/// through as its surrogate escape units, the way the Python
/// reading holds it, rather than through a rendered text that
/// would trade those units for a replacement character.
fn match_long(text: &PText) -> LongMatch {
    let units = text.units();
    let (name, value) = match units.iter().position(|unit| *unit == 0x3D) {
        Some(split) => (
            &units[..split],
            Some(PText::from_units(units[split + 1..].to_vec())),
        ),
        None => (units, None),
    };
    if value.is_none() && name.len() <= 2 {
        // The bare separator carries no option name, and the
        // Python matcher never reads it as an abbreviation.
        // A value riding after an equals sign changes the
        // reading, as the next check states.
        return LongMatch::Unknown;
    }
    // The match is a prefix match against the one long option,
    // and the empty prefix matches it too, so a token whose
    // name is only the two dashes still names the help option
    // when a value rides after its equals sign, the reading
    // the Python surface gives it.
    let held: Vec<u16> = "--help".encode_utf16().collect();
    if held != name && !held.starts_with(name) {
        return LongMatch::Unknown;
    }
    match value {
        Some(value) => LongMatch::HelpWithValue(value),
        None => LongMatch::Help,
    }
}

/// The value riding on the short help option in its explicit
/// form, the token opening with the help flag and an equals
/// sign, or None when the token carries no such value. A
/// cluster whose second character is anything else, and a
/// cluster that runs on past the flag without the equals sign
/// at that spot, carries no explicit value, as the Python
/// surface reads them.
fn short_help_value(text: &PText) -> Option<PText> {
    let units = text.units();
    if units.len() > 2 && units[0] == 0x2D && units[1] == 0x68 && units[2] == 0x3D {
        Some(PText::from_units(units[3..].to_vec()))
    } else {
        None
    }
}

fn is_help_token(text: &PText) -> bool {
    if text_is(text, "-h") {
        return true;
    }
    let units = text.units();
    if units.len() >= 2 && units[0] == 0x2D && units[1] == 0x68 && !starts_with_dashdash(text) {
        // A short cluster opening with the help flag.
        return true;
    }
    if starts_with_dashdash(text) {
        return matches!(match_long(text), LongMatch::Help);
    }
    false
}

/// Run the command line over the given arguments at the given
/// terminal width, returning the outcome.
pub fn run(args: &[PText], columns: usize) -> Outcome {
    let top = top_spec();
    let mut extras: Vec<PText> = Vec::new();
    let mut index = 0;
    let command: String;
    let sub_args: &[PText];
    loop {
        if index >= args.len() {
            return usage_error(
                &top,
                columns,
                "the following arguments are required: command",
            );
        }
        let token = &args[index];
        if let Some(value) = short_help_value(token) {
            return usage_error(
                &top,
                columns,
                &format!(
                    "argument -h/--help: ignored explicit argument {}",
                    py_repr(&value)
                ),
            );
        }
        if starts_with_dashdash(token) {
            if let LongMatch::HelpWithValue(value) = match_long(token) {
                return usage_error(
                    &top,
                    columns,
                    &format!(
                        "argument -h/--help: ignored explicit argument {}",
                        py_repr(&value)
                    ),
                );
            }
        }
        if is_help_token(token) {
            return Outcome::plain(
                format_help(&top, columns)
                    .lines()
                    .map(|line| line.to_string())
                    .collect(),
                0,
            );
        }
        if text_is(token, "--") {
            // The separator reached while the command is still
            // unsought is itself the refused value when any
            // token follows it, and the required error when
            // none does, as the Python surface reads it.
            if index + 1 >= args.len() {
                return usage_error(
                    &top,
                    columns,
                    "the following arguments are required: command",
                );
            }
            return invalid_choice(&top, columns, token);
        }
        if let Some(found) = COMMANDS.iter().find(|name| text_is(token, name)) {
            command = found.to_string();
            sub_args = &args[index + 1..];
            break;
        }
        if is_option_like(token) {
            extras.push(token.clone());
            index += 1;
            continue;
        }
        return invalid_choice(&top, columns, token);
    }
    run_subcommand(&command, sub_args, extras, columns)
}

fn invalid_choice(spec: &ParserSpec, columns: usize, value: &PText) -> Outcome {
    usage_error(
        spec,
        columns,
        &format!(
            "argument command: invalid choice: {} (choose from 'tally', 'stadium', 'cost', 'agreement', 'compare', 'census', 'ledger', 'dissent', 'swing', 'minority', 'bench', 'pool', 'aggregate', 'canvass', 'roll', 'muster', 'margin', 'accord', 'standing', 'answers', 'pairs', 'shares')",
            py_repr(value)
        ),
    )
}

enum SubParse {
    Done(Outcome),
    Parsed {
        values: Vec<PText>,
        extras: Vec<PText>,
    },
}

fn parse_subcommand(
    command: &str,
    args: &[PText],
    top_extras: Vec<PText>,
    columns: usize,
) -> SubParse {
    let spec = sub_spec(command);
    let positional_names: &[&str] = match command {
        "cost" => &["poll_file", "prices_file"],
        "agreement" => &["poll_files"],
        "compare" => &["first_dir", "second_dir"],
        "census" | "ledger" | "dissent" | "swing" | "minority" | "bench" | "pool" | "aggregate"
        | "canvass" | "roll" | "muster" | "margin" | "accord" | "standing" | "answers"
        | "pairs" | "shares" => &["label", "directory"],
        _ => &["poll_file"],
    };
    let unlimited = command == "agreement"
        || command == "census"
        || command == "ledger"
        || command == "dissent"
        || command == "swing"
        || command == "minority"
        || command == "bench"
        || command == "pool"
        || command == "aggregate"
        || command == "canvass"
        || command == "roll"
        || command == "muster"
        || command == "margin"
        || command == "accord"
        || command == "standing"
        || command == "answers"
        || command == "pairs"
        || command == "shares";
    let mut values: Vec<PText> = Vec::new();
    let mut extras: Vec<PText> = top_extras;
    let mut positional_only = false;
    // The unlimited positional takes its values in one run.
    // An unrecognized option token that lands after the run
    // has started breaks it, and every later token, values and
    // separators alike, stands among the extras, the reading
    // the Python surface gives the broken run. A token that
    // lands before the run starts breaks nothing.
    let mut broken_run = false;
    let mut index = 0;
    while index < args.len() {
        let token = &args[index];
        if !positional_only {
            if text_is(token, "--") {
                if broken_run {
                    extras.push(token.clone());
                } else {
                    positional_only = true;
                }
                index += 1;
                continue;
            }
            if let Some(value) = short_help_value(token) {
                return SubParse::Done(usage_error(
                    &spec,
                    columns,
                    &format!(
                        "argument -h/--help: ignored explicit argument {}",
                        py_repr(&value)
                    ),
                ));
            }
            if is_help_token(token) {
                return SubParse::Done(Outcome::plain(
                    format_help(&spec, columns)
                        .lines()
                        .map(|line| line.to_string())
                        .collect(),
                    0,
                ));
            }
            if starts_with_dashdash(token) {
                match match_long(token) {
                    LongMatch::Help => {
                        return SubParse::Done(Outcome::plain(
                            format_help(&spec, columns)
                                .lines()
                                .map(|line| line.to_string())
                                .collect(),
                            0,
                        ));
                    }
                    LongMatch::HelpWithValue(value) => {
                        return SubParse::Done(usage_error(
                            &spec,
                            columns,
                            &format!(
                                "argument -h/--help: ignored explicit argument {}",
                                py_repr(&value)
                            ),
                        ));
                    }
                    LongMatch::Unknown => {
                        extras.push(token.clone());
                        if unlimited && !values.is_empty() {
                            broken_run = true;
                        }
                        index += 1;
                        continue;
                    }
                }
            }
            if is_option_like(token) {
                extras.push(token.clone());
                if unlimited && !values.is_empty() {
                    broken_run = true;
                }
                index += 1;
                continue;
            }
        }
        if broken_run {
            extras.push(token.clone());
        } else if unlimited || values.len() < positional_names.len() {
            values.push(token.clone());
        } else {
            extras.push(token.clone());
        }
        index += 1;
    }
    if values.len() < positional_names.len() {
        let missing = &positional_names[values.len()..];
        return SubParse::Done(usage_error(
            &spec,
            columns,
            &format!(
                "the following arguments are required: {}",
                missing.join(", ")
            ),
        ));
    }
    SubParse::Parsed { values, extras }
}

fn run_subcommand(
    command: &str,
    args: &[PText],
    top_extras: Vec<PText>,
    columns: usize,
) -> Outcome {
    let (values, extras) = match parse_subcommand(command, args, top_extras, columns) {
        SubParse::Done(outcome) => return outcome,
        SubParse::Parsed { values, extras } => (values, extras),
    };
    if !extras.is_empty() {
        let top = top_spec();
        let joined = extras
            .iter()
            .map(echo_text)
            .collect::<Vec<String>>()
            .join(" ");
        return usage_error(
            &top,
            columns,
            &format!("unrecognized arguments: {}", joined),
        );
    }
    if (command == "census"
        || command == "ledger"
        || command == "dissent"
        || command == "swing"
        || command == "minority"
        || command == "bench"
        || command == "pool"
        || command == "aggregate"
        || command == "canvass"
        || command == "roll"
        || command == "muster"
        || command == "margin"
        || command == "accord"
        || command == "standing"
        || command == "answers"
        || command == "pairs"
        || command == "shares")
        && values.len() % 2 == 1
    {
        // The census arguments stand as label and directory
        // pairs, so an odd count leaves the last label with
        // no directory after it, and the refusal names the
        // directory argument as required, as GJ-12 states.
        // The ledger takes the same pairs and the same
        // refusal, as GJ-14 states, and the dissent takes
        // the same pairs and the same refusal, as GJ-16
        // states, and the swing takes the same pairs and
        // the same refusal, as GJ-18 states, and the
        // minority takes the same pairs and the same
        // refusal, as GJ-20 states, and the bench takes the
        // same pairs and the same refusal, as GJ-24 states,
        // and the pool takes the same pairs and the same
        // refusal, as GJ-26 states, and the aggregate takes
        // the same pairs and the same refusal, as GJ-28
        // states, and the canvass takes the same pairs and
        // the same refusal, as GJ-30 states, and the roll
        // takes the same pairs and the same refusal, as
        // GJ-32 states, and the muster takes the same
        // pairs and the same refusal, as GJ-34 states, and
        // the margin takes the same pairs and the same
        // refusal, as GJ-36 states, and the accord
        // takes the same pairs and the same refusal, as
        // GJ-38 states, and the standing takes the same
        // pairs and the same refusal, as GJ-40 states,
        // and the answers takes the same pairs and the
        // same refusal, as GJ-42 states, and the pairs
        // takes the same pairs and the same refusal, as
        // GJ-44 states, and the shares takes the same
        // pairs and the same refusal, as GJ-48 states.
        return usage_error(
            &sub_spec(command),
            columns,
            "the following arguments are required: directory",
        );
    }
    match command {
        "tally" => command_tally(&values[0]),
        "stadium" => command_stadium(&values[0]),
        "cost" => command_cost(&values[0], &values[1]),
        "agreement" => command_agreement(&values),
        "compare" => command_compare(&values[0], &values[1]),
        "census" => command_census(&values),
        "ledger" => command_ledger(&values),
        "dissent" => command_dissent(&values),
        "swing" => command_swing(&values),
        "minority" => command_minority(&values),
        "bench" => command_bench(&values),
        "pool" => command_pool(&values),
        "aggregate" => command_aggregate(&values),
        "canvass" => command_canvass(&values),
        "roll" => command_roll(&values),
        "muster" => command_muster(&values),
        "margin" => command_margin(&values),
        "accord" => command_accord(&values),
        "standing" => command_standing(&values),
        "answers" => command_answers(&values),
        "pairs" => command_pairs(&values),
        "shares" => command_shares(&values),
        _ => unreachable!("a known command"),
    }
}

/// Load one poll or stand its defect lines in the outcome
/// lines, as the Python surface prints them.
fn load_poll_or_lines(path: &PText, lines: &mut Vec<String>) -> Option<Poll> {
    load_poll_path_or_lines(&path_from_arg(path), lines)
}

/// Load one poll from a path already in hand or stand its
/// defect lines in the outcome lines, the one home of the
/// invalid poll surface the commands print.
fn load_poll_path_or_lines(path: &std::path::Path, lines: &mut Vec<String>) -> Option<Poll> {
    let (poll, defects) = load_poll_file(path);
    if !defects.is_empty() {
        lines.push("invalid poll".to_string());
        for defect in defects {
            lines.push(format!("defect {}", defect));
        }
        return None;
    }
    poll
}

fn command_tally(file: &PText) -> Outcome {
    let mut lines: Vec<String> = Vec::new();
    let poll = match load_poll_or_lines(file, &mut lines) {
        Some(poll) => poll,
        None => return Outcome::plain(lines, 1),
    };
    match render_tally_lines(&poll) {
        Ok(reading) => {
            lines.extend(reading);
            Outcome::plain(lines, 0)
        }
        Err(defect) => {
            lines.push("invalid tally".to_string());
            lines.push(format!("defect {}", defect));
            Outcome::plain(lines, 1)
        }
    }
}

fn command_stadium(file: &PText) -> Outcome {
    let mut lines: Vec<String> = Vec::new();
    let poll = match load_poll_or_lines(file, &mut lines) {
        Some(poll) => poll,
        None => return Outcome::plain(lines, 1),
    };
    match render_stadium(&poll) {
        Ok(reading) => {
            lines.extend(reading);
            Outcome::plain(lines, 0)
        }
        Err(defect) => {
            lines.push("invalid stadium".to_string());
            lines.push(format!("defect {}", defect));
            Outcome::plain(lines, 1)
        }
    }
}

fn command_cost(poll_file: &PText, prices_file: &PText) -> Outcome {
    let mut lines: Vec<String> = Vec::new();
    let poll = load_poll_or_lines(poll_file, &mut lines);
    let (prices, price_defects) = load_prices_file(&path_from_arg(prices_file));
    if !price_defects.is_empty() {
        lines.push("invalid price table".to_string());
        for defect in price_defects {
            lines.push(format!("defect {}", defect));
        }
    }
    let (poll, prices) = match (poll, prices) {
        (Some(poll), Some(prices)) => (poll, prices),
        _ => return Outcome::plain(lines, 1),
    };
    match cost_lines(&poll, &prices) {
        Ok(Some(report)) => lines.extend(report),
        // The working precision past the Python context bound
        // fails the command the way the Python crash reads on
        // the surface, status 1 with no report lines, as
        // PORT-11 states.
        Ok(None) => return Outcome::plain(Vec::new(), 1),
        Err(defect) => {
            lines.push("invalid cost".to_string());
            lines.push(format!("defect {}", defect));
            return Outcome::plain(lines, 1);
        }
    }
    Outcome::plain(lines, 0)
}

fn command_agreement(files: &[PText]) -> Outcome {
    let mut lines: Vec<String> = Vec::new();
    let mut polls: Vec<Poll> = Vec::new();
    for file in files {
        match load_poll_or_lines(file, &mut lines) {
            Some(poll) => polls.push(poll),
            None => return Outcome::plain(lines, 1),
        }
    }
    match agreement_lines(&polls) {
        Ok(reading) => {
            lines.extend(reading);
            Outcome::plain(lines, 0)
        }
        Err(defect) => {
            lines.push("invalid agreement".to_string());
            lines.push(format!("defect {}", defect));
            Outcome::plain(lines, 1)
        }
    }
}

/// Load the poll set of one directory, the files named
/// poll-01.json, poll-02.json, and so on in number order,
/// consecutive from the first, the set ending at the first
/// number whose file does not stand. The first file that
/// draws defects stands its defect lines in the outcome
/// lines, as the loader surface prints them. A directory in
/// which the first poll file does not stand reads that file
/// through the loader, so its defect stands as the defect a
/// missing poll file draws.
fn load_poll_set(dir: &PText, lines: &mut Vec<String>) -> Option<Vec<Poll>> {
    let directory = path_from_arg(dir);
    let mut polls: Vec<Poll> = Vec::new();
    let mut index = 1usize;
    loop {
        let path = directory.join(format!("poll-{:02}.json", index));
        if !path.exists() && !polls.is_empty() {
            break;
        }
        let poll = load_poll_path_or_lines(&path, lines)?;
        polls.push(poll);
        index += 1;
    }
    Some(polls)
}

fn command_compare(first_dir: &PText, second_dir: &PText) -> Outcome {
    let mut lines: Vec<String> = Vec::new();
    let first = match load_poll_set(first_dir, &mut lines) {
        Some(polls) => polls,
        None => return Outcome::plain(lines, 1),
    };
    let second = match load_poll_set(second_dir, &mut lines) {
        Some(polls) => polls,
        None => return Outcome::plain(lines, 1),
    };
    match compare_lines(&first, &second) {
        Ok(reading) => {
            lines.extend(reading);
            Outcome::plain(lines, 0)
        }
        Err(defect) => {
            lines.push("invalid comparison".to_string());
            lines.push(format!("defect {}", defect));
            Outcome::plain(lines, 1)
        }
    }
}

/// The census command. The arguments stand as set label and
/// directory pairs, and the sets load in pair order through
/// the poll set loader the compare command uses, so the
/// first set whose polls draw defects stands its defect
/// lines in the outcome and no census prints, the census
/// whole or nothing as GJ-12 states. A refusal of the
/// census reading itself prints the invalid census line and
/// its defect line, in the form of the invalid comparison
/// surface.
fn command_census(values: &[PText]) -> Outcome {
    let mut lines: Vec<String> = Vec::new();
    let mut sets: Vec<(PText, Vec<Poll>)> = Vec::new();
    // The dispatch refuses an odd argument count before this
    // command runs, so the values stand in whole pairs and
    // the remainder of the pairing is empty.
    let (pairs, _) = values.as_chunks::<2>();
    for pair in pairs {
        let polls = match load_poll_set(&pair[1], &mut lines) {
            Some(polls) => polls,
            None => return Outcome::plain(lines, 1),
        };
        sets.push((pair[0].clone(), polls));
    }
    match census_lines(&sets) {
        Ok(reading) => {
            lines.extend(reading);
            Outcome::plain(lines, 0)
        }
        Err(defect) => {
            lines.push("invalid census".to_string());
            lines.push(format!("defect {}", defect));
            Outcome::plain(lines, 1)
        }
    }
}

/// The ledger command. The arguments stand as set label and
/// directory pairs exactly as the census command takes them,
/// and the sets load in pair order through the same poll set
/// loader, so the first set whose polls draw defects stands
/// its defect lines in the outcome and no ledger prints, the
/// ledger whole or nothing as GJ-14 states. A refusal of the
/// ledger reading itself prints the invalid ledger line and
/// its defect line, in the form of the invalid census
/// surface.
fn command_ledger(values: &[PText]) -> Outcome {
    let mut lines: Vec<String> = Vec::new();
    let mut sets: Vec<(PText, Vec<Poll>)> = Vec::new();
    // The dispatch refuses an odd argument count before this
    // command runs, so the values stand in whole pairs and
    // the remainder of the pairing is empty.
    let (pairs, _) = values.as_chunks::<2>();
    for pair in pairs {
        let polls = match load_poll_set(&pair[1], &mut lines) {
            Some(polls) => polls,
            None => return Outcome::plain(lines, 1),
        };
        sets.push((pair[0].clone(), polls));
    }
    match ledger_lines(&sets) {
        Ok(reading) => {
            lines.extend(reading);
            Outcome::plain(lines, 0)
        }
        Err(defect) => {
            lines.push("invalid ledger".to_string());
            lines.push(format!("defect {}", defect));
            Outcome::plain(lines, 1)
        }
    }
}

/// The dissent command. The arguments stand as set label and
/// directory pairs exactly as the census command takes them,
/// and the sets load in pair order through the same poll set
/// loader, so the first set whose polls draw defects stands
/// its defect lines in the outcome and no dissent prints, the
/// dissent whole or nothing as GJ-16 states. A refusal of
/// the dissent reading itself prints the invalid dissent
/// line and its defect line, in the form of the invalid
/// census surface.
fn command_dissent(values: &[PText]) -> Outcome {
    let mut lines: Vec<String> = Vec::new();
    let mut sets: Vec<(PText, Vec<Poll>)> = Vec::new();
    // The dispatch refuses an odd argument count before this
    // command runs, so the values stand in whole pairs and
    // the remainder of the pairing is empty.
    let (pairs, _) = values.as_chunks::<2>();
    for pair in pairs {
        let polls = match load_poll_set(&pair[1], &mut lines) {
            Some(polls) => polls,
            None => return Outcome::plain(lines, 1),
        };
        sets.push((pair[0].clone(), polls));
    }
    match dissent_lines(&sets) {
        Ok(reading) => {
            lines.extend(reading);
            Outcome::plain(lines, 0)
        }
        Err(defect) => {
            lines.push("invalid dissent".to_string());
            lines.push(format!("defect {}", defect));
            Outcome::plain(lines, 1)
        }
    }
}

/// The swing command. The arguments stand as set label and
/// directory pairs exactly as the census command takes them,
/// and the sets load in pair order through the same poll set
/// loader, so the first set whose polls draw defects stands
/// its defect lines in the outcome and no swing prints, the
/// swing whole or nothing as GJ-18 states. A refusal of the
/// swing reading itself prints the invalid swing line and
/// its defect line, in the form of the invalid census
/// surface.
fn command_swing(values: &[PText]) -> Outcome {
    let mut lines: Vec<String> = Vec::new();
    let mut sets: Vec<(PText, Vec<Poll>)> = Vec::new();
    // The dispatch refuses an odd argument count before this
    // command runs, so the values stand in whole pairs and
    // the remainder of the pairing is empty.
    let (pairs, _) = values.as_chunks::<2>();
    for pair in pairs {
        let polls = match load_poll_set(&pair[1], &mut lines) {
            Some(polls) => polls,
            None => return Outcome::plain(lines, 1),
        };
        sets.push((pair[0].clone(), polls));
    }
    match swing_lines(&sets) {
        Ok(reading) => {
            lines.extend(reading);
            Outcome::plain(lines, 0)
        }
        Err(defect) => {
            lines.push("invalid swing".to_string());
            lines.push(format!("defect {}", defect));
            Outcome::plain(lines, 1)
        }
    }
}

/// The minority command. The arguments stand as set label and
/// directory pairs exactly as the census command takes them,
/// and the sets load in pair order through the same poll set
/// loader, so the first set whose polls draw defects stands
/// its defect lines in the outcome and no minority prints, the
/// minority whole or nothing as GJ-20 states. A refusal of
/// the minority reading itself prints the invalid minority
/// line and its defect line, in the form of the invalid
/// census surface.
fn command_minority(values: &[PText]) -> Outcome {
    let mut lines: Vec<String> = Vec::new();
    let mut sets: Vec<(PText, Vec<Poll>)> = Vec::new();
    // The dispatch refuses an odd argument count before this
    // command runs, so the values stand in whole pairs and
    // the remainder of the pairing is empty.
    let (pairs, _) = values.as_chunks::<2>();
    for pair in pairs {
        let polls = match load_poll_set(&pair[1], &mut lines) {
            Some(polls) => polls,
            None => return Outcome::plain(lines, 1),
        };
        sets.push((pair[0].clone(), polls));
    }
    match minority_lines(&sets) {
        Ok(reading) => {
            lines.extend(reading);
            Outcome::plain(lines, 0)
        }
        Err(defect) => {
            lines.push("invalid minority".to_string());
            lines.push(format!("defect {}", defect));
            Outcome::plain(lines, 1)
        }
    }
}

/// The bench command. The arguments stand as set label and
/// directory pairs exactly as the census command takes them,
/// and the sets load in pair order through the same poll set
/// loader, so the first set whose polls draw defects stands
/// its defect lines in the outcome and no bench prints, the
/// bench whole or nothing as GJ-24 states. A refusal of the
/// bench reading itself prints the invalid bench line and
/// its defect line, in the form of the invalid census
/// surface.
fn command_bench(values: &[PText]) -> Outcome {
    let mut lines: Vec<String> = Vec::new();
    let mut sets: Vec<(PText, Vec<Poll>)> = Vec::new();
    // The dispatch refuses an odd argument count before this
    // command runs, so the values stand in whole pairs and
    // the remainder of the pairing is empty.
    let (pairs, _) = values.as_chunks::<2>();
    for pair in pairs {
        let polls = match load_poll_set(&pair[1], &mut lines) {
            Some(polls) => polls,
            None => return Outcome::plain(lines, 1),
        };
        sets.push((pair[0].clone(), polls));
    }
    match bench_lines(&sets) {
        Ok(reading) => {
            lines.extend(reading);
            Outcome::plain(lines, 0)
        }
        Err(defect) => {
            lines.push("invalid bench".to_string());
            lines.push(format!("defect {}", defect));
            Outcome::plain(lines, 1)
        }
    }
}

/// The pool command. The arguments stand as set label and
/// directory pairs exactly as the census command takes
/// them, and the sets load in pair order through the same
/// poll set loader, so the first set whose polls draw
/// defects stands its defect lines in the outcome and no
/// pool prints, the pool whole or nothing as GJ-26
/// states. A refusal of the pool reading itself prints
/// the invalid pool line and its defect line, in the form
/// of the invalid census surface.
fn command_pool(values: &[PText]) -> Outcome {
    let mut lines: Vec<String> = Vec::new();
    let mut sets: Vec<(PText, Vec<Poll>)> = Vec::new();
    // The dispatch refuses an odd argument count before
    // this command runs, so the values stand in whole
    // pairs and the remainder of the pairing is empty.
    let (pairs, _) = values.as_chunks::<2>();
    for pair in pairs {
        let polls = match load_poll_set(&pair[1], &mut lines) {
            Some(polls) => polls,
            None => return Outcome::plain(lines, 1),
        };
        sets.push((pair[0].clone(), polls));
    }
    match pool_lines(&sets) {
        Ok(reading) => {
            lines.extend(reading);
            Outcome::plain(lines, 0)
        }
        Err(defect) => {
            lines.push("invalid pool".to_string());
            lines.push(format!("defect {}", defect));
            Outcome::plain(lines, 1)
        }
    }
}

/// The aggregate command. The arguments stand as set label
/// and directory pairs exactly as the census command takes
/// them, and the sets load in pair order through the same
/// poll set loader, so the first set whose polls draw
/// defects stands its defect lines in the outcome and no
/// aggregate prints, the aggregate whole or nothing as
/// GJ-28 states. A refusal of the aggregate reading itself
/// prints the invalid aggregate line and its defect line,
/// in the form of the invalid census surface.
fn command_aggregate(values: &[PText]) -> Outcome {
    let mut lines: Vec<String> = Vec::new();
    let mut sets: Vec<(PText, Vec<Poll>)> = Vec::new();
    // The dispatch refuses an odd argument count before
    // this command runs, so the values stand in whole
    // pairs and the remainder of the pairing is empty.
    let (pairs, _) = values.as_chunks::<2>();
    for pair in pairs {
        let polls = match load_poll_set(&pair[1], &mut lines) {
            Some(polls) => polls,
            None => return Outcome::plain(lines, 1),
        };
        sets.push((pair[0].clone(), polls));
    }
    match aggregate_lines(&sets) {
        Ok(reading) => {
            lines.extend(reading);
            Outcome::plain(lines, 0)
        }
        Err(defect) => {
            lines.push("invalid aggregate".to_string());
            lines.push(format!("defect {}", defect));
            Outcome::plain(lines, 1)
        }
    }
}

/// The canvass command. The arguments stand as set label
/// and directory pairs exactly as the census command takes
/// them, and the sets load in pair order through the same
/// poll set loader, so the first set whose polls draw
/// defects stands its defect lines in the outcome and no
/// canvass prints, the canvass whole or nothing as GJ-30
/// states. A refusal of the canvass reading itself prints
/// the invalid canvass line and its defect line, in the
/// form of the invalid census surface.
fn command_canvass(values: &[PText]) -> Outcome {
    let mut lines: Vec<String> = Vec::new();
    let mut sets: Vec<(PText, Vec<Poll>)> = Vec::new();
    // The dispatch refuses an odd argument count before
    // this command runs, so the values stand in whole
    // pairs and the remainder of the pairing is empty.
    let (pairs, _) = values.as_chunks::<2>();
    for pair in pairs {
        let polls = match load_poll_set(&pair[1], &mut lines) {
            Some(polls) => polls,
            None => return Outcome::plain(lines, 1),
        };
        sets.push((pair[0].clone(), polls));
    }
    match canvass_lines(&sets) {
        Ok(reading) => {
            lines.extend(reading);
            Outcome::plain(lines, 0)
        }
        Err(defect) => {
            lines.push("invalid canvass".to_string());
            lines.push(format!("defect {}", defect));
            Outcome::plain(lines, 1)
        }
    }
}

/// The roll command. The arguments stand as set label and
/// directory pairs exactly as the census command takes
/// them, and the sets load in pair order through the same
/// poll set loader, so the first set whose polls draw
/// defects stands its defect lines in the outcome and no
/// roll prints, the roll whole or nothing as GJ-32
/// states. A refusal of the roll reading itself prints
/// the invalid roll line and its defect line, in the form
/// of the invalid census surface.
fn command_roll(values: &[PText]) -> Outcome {
    let mut lines: Vec<String> = Vec::new();
    let mut sets: Vec<(PText, Vec<Poll>)> = Vec::new();
    // The dispatch refuses an odd argument count before
    // this command runs, so the values stand in whole
    // pairs and the remainder of the pairing is empty.
    let (pairs, _) = values.as_chunks::<2>();
    for pair in pairs {
        let polls = match load_poll_set(&pair[1], &mut lines) {
            Some(polls) => polls,
            None => return Outcome::plain(lines, 1),
        };
        sets.push((pair[0].clone(), polls));
    }
    match roll_lines(&sets) {
        Ok(reading) => {
            lines.extend(reading);
            Outcome::plain(lines, 0)
        }
        Err(defect) => {
            lines.push("invalid roll".to_string());
            lines.push(format!("defect {}", defect));
            Outcome::plain(lines, 1)
        }
    }
}

/// The muster command. The arguments stand as set label
/// and directory pairs exactly as the census command
/// takes them, and the sets load in pair order through
/// the same poll set loader, so the first set whose
/// polls draw defects stands its defect lines in the
/// outcome and no muster prints, the muster whole or
/// nothing as GJ-34 states. A refusal of the muster
/// reading itself prints the invalid muster line and
/// its defect line, in the form of the invalid census
/// surface.
fn command_muster(values: &[PText]) -> Outcome {
    let mut lines: Vec<String> = Vec::new();
    let mut sets: Vec<(PText, Vec<Poll>)> = Vec::new();
    // The dispatch refuses an odd argument count before
    // this command runs, so the values stand in whole
    // pairs and the remainder of the pairing is empty.
    let (pairs, _) = values.as_chunks::<2>();
    for pair in pairs {
        let polls = match load_poll_set(&pair[1], &mut lines) {
            Some(polls) => polls,
            None => return Outcome::plain(lines, 1),
        };
        sets.push((pair[0].clone(), polls));
    }
    match muster_lines(&sets) {
        Ok(reading) => {
            lines.extend(reading);
            Outcome::plain(lines, 0)
        }
        Err(defect) => {
            lines.push("invalid muster".to_string());
            lines.push(format!("defect {}", defect));
            Outcome::plain(lines, 1)
        }
    }
}

/// The margin command. The arguments stand as set label
/// and directory pairs exactly as the census command
/// takes them, and the sets load in pair order through
/// the same poll set loader, so the first set whose
/// polls draw defects stands its defect lines in the
/// outcome and no margin prints, the margin whole or
/// nothing as GJ-36 states. A refusal of the margin
/// reading itself prints the invalid margin line and
/// its defect line, in the form of the invalid census
/// surface.
fn command_margin(values: &[PText]) -> Outcome {
    let mut lines: Vec<String> = Vec::new();
    let mut sets: Vec<(PText, Vec<Poll>)> = Vec::new();
    // The dispatch refuses an odd argument count before
    // this command runs, so the values stand in whole
    // pairs and the remainder of the pairing is empty.
    let (pairs, _) = values.as_chunks::<2>();
    for pair in pairs {
        let polls = match load_poll_set(&pair[1], &mut lines) {
            Some(polls) => polls,
            None => return Outcome::plain(lines, 1),
        };
        sets.push((pair[0].clone(), polls));
    }
    match margin_lines(&sets) {
        Ok(reading) => {
            lines.extend(reading);
            Outcome::plain(lines, 0)
        }
        Err(defect) => {
            lines.push("invalid margin".to_string());
            lines.push(format!("defect {}", defect));
            Outcome::plain(lines, 1)
        }
    }
}

/// The accord command. The arguments stand as set label
/// and directory pairs exactly as the census command
/// takes them, and the sets load in pair order through
/// the same poll set loader, so the first set whose
/// polls draw defects stands its defect lines in the
/// outcome and no accord prints, the accord whole or
/// nothing as GJ-38 states. A refusal of the accord
/// reading itself prints the invalid accord line and
/// its defect line, in the form of the invalid census
/// surface.
fn command_accord(values: &[PText]) -> Outcome {
    let mut lines: Vec<String> = Vec::new();
    let mut sets: Vec<(PText, Vec<Poll>)> = Vec::new();
    // The dispatch refuses an odd argument count before
    // this command runs, so the values stand in whole
    // pairs and the remainder of the pairing is empty.
    let (pairs, _) = values.as_chunks::<2>();
    for pair in pairs {
        let polls = match load_poll_set(&pair[1], &mut lines) {
            Some(polls) => polls,
            None => return Outcome::plain(lines, 1),
        };
        sets.push((pair[0].clone(), polls));
    }
    match accord_lines(&sets) {
        Ok(reading) => {
            lines.extend(reading);
            Outcome::plain(lines, 0)
        }
        Err(defect) => {
            lines.push("invalid accord".to_string());
            lines.push(format!("defect {}", defect));
            Outcome::plain(lines, 1)
        }
    }
}

/// The standing command. The arguments stand as set
/// label and directory pairs exactly as the census
/// command takes them, and the sets load in pair order
/// through the same poll set loader, so the first set
/// whose polls draw defects stands its defect lines in
/// the outcome and no standing prints, the standing
/// whole or nothing as GJ-40 states. A refusal of the
/// standing reading itself prints the invalid standing
/// line and its defect line, in the form of the
/// invalid census surface.
fn command_standing(values: &[PText]) -> Outcome {
    let mut lines: Vec<String> = Vec::new();
    let mut sets: Vec<(PText, Vec<Poll>)> = Vec::new();
    // The dispatch refuses an odd argument count before
    // this command runs, so the values stand in whole
    // pairs and the remainder of the pairing is empty.
    let (pairs, _) = values.as_chunks::<2>();
    for pair in pairs {
        let polls = match load_poll_set(&pair[1], &mut lines) {
            Some(polls) => polls,
            None => return Outcome::plain(lines, 1),
        };
        sets.push((pair[0].clone(), polls));
    }
    match standing_lines(&sets) {
        Ok(reading) => {
            lines.extend(reading);
            Outcome::plain(lines, 0)
        }
        Err(defect) => {
            lines.push("invalid standing".to_string());
            lines.push(format!("defect {}", defect));
            Outcome::plain(lines, 1)
        }
    }
}

/// The answers command. The arguments stand as set
/// label and directory pairs exactly as the census
/// command takes them, and the sets load in pair order
/// through the same poll set loader, so the first set
/// whose polls draw defects stands its defect lines in
/// the outcome and no answers print, the answers
/// whole or nothing as GJ-42 states. A refusal of the
/// answers reading itself prints the invalid answers
/// line and its defect line, in the form of the
/// invalid census surface.
fn command_answers(values: &[PText]) -> Outcome {
    let mut lines: Vec<String> = Vec::new();
    let mut sets: Vec<(PText, Vec<Poll>)> = Vec::new();
    // The dispatch refuses an odd argument count before
    // this command runs, so the values stand in whole
    // pairs and the remainder of the pairing is empty.
    let (pairs, _) = values.as_chunks::<2>();
    for pair in pairs {
        let polls = match load_poll_set(&pair[1], &mut lines) {
            Some(polls) => polls,
            None => return Outcome::plain(lines, 1),
        };
        sets.push((pair[0].clone(), polls));
    }
    match answers_lines(&sets) {
        Ok(reading) => {
            lines.extend(reading);
            Outcome::plain(lines, 0)
        }
        Err(defect) => {
            lines.push("invalid answers".to_string());
            lines.push(format!("defect {}", defect));
            Outcome::plain(lines, 1)
        }
    }
}

/// The pairs command. The arguments stand as set
/// label and directory pairs exactly as the census
/// command takes them, and the sets load in pair order
/// through the same poll set loader, so the first set
/// whose polls draw defects stands its defect lines in
/// the outcome and no pairs print, the pairs
/// whole or nothing as GJ-44 states. A refusal of the
/// pairs reading itself prints the invalid pairs
/// line and its defect line, in the form of the
/// invalid census surface.
fn command_pairs(values: &[PText]) -> Outcome {
    let mut lines: Vec<String> = Vec::new();
    let mut sets: Vec<(PText, Vec<Poll>)> = Vec::new();
    // The dispatch refuses an odd argument count before
    // this command runs, so the values stand in whole
    // pairs and the remainder of the pairing is empty.
    let (pairs, _) = values.as_chunks::<2>();
    for pair in pairs {
        let polls = match load_poll_set(&pair[1], &mut lines) {
            Some(polls) => polls,
            None => return Outcome::plain(lines, 1),
        };
        sets.push((pair[0].clone(), polls));
    }
    match pairs_lines(&sets) {
        Ok(reading) => {
            lines.extend(reading);
            Outcome::plain(lines, 0)
        }
        Err(defect) => {
            lines.push("invalid pairs".to_string());
            lines.push(format!("defect {}", defect));
            Outcome::plain(lines, 1)
        }
    }
}

/// The shares command. The arguments stand as set
/// label and directory pairs exactly as the census
/// command takes them, and the sets load in pair order
/// through the same poll set loader, so the first set
/// whose polls draw defects stands its defect lines in
/// the outcome and no shares print, the shares
/// whole or nothing as GJ-48 states. A refusal of the
/// shares reading itself prints the invalid shares
/// line and its defect line, in the form of the
/// invalid census surface.
fn command_shares(values: &[PText]) -> Outcome {
    let mut lines: Vec<String> = Vec::new();
    let mut sets: Vec<(PText, Vec<Poll>)> = Vec::new();
    // The dispatch refuses an odd argument count before
    // this command runs, so the values stand in whole
    // pairs and the remainder of the pairing is empty.
    let (pairs, _) = values.as_chunks::<2>();
    for pair in pairs {
        let polls = match load_poll_set(&pair[1], &mut lines) {
            Some(polls) => polls,
            None => return Outcome::plain(lines, 1),
        };
        sets.push((pair[0].clone(), polls));
    }
    match shares_lines(&sets) {
        Ok(reading) => {
            lines.extend(reading);
            Outcome::plain(lines, 0)
        }
        Err(defect) => {
            lines.push("invalid shares".to_string());
            lines.push(format!("defect {}", defect));
            Outcome::plain(lines, 1)
        }
    }
}
