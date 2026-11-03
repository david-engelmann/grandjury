//! The grandjury command line entry point.
//!
//! A thin shell over the library surface in cli. The
//! arguments arrive as the operating system bytes and decode
//! under the surrogate escape reading, as the Python runtime
//! decodes them. The output stream follows the closed stream
//! bands PORT-7 states and the single long line rule PORT-13
//! states, both measured on this surface. The writer hands
//! its output down line by line. An accumulation of ordinary
//! lines is handed down when it reaches 8192 bytes, and a
//! failure there leaves by status 1. What remains at the end
//! is handed down last, and a failure there stands for status
//! 120 at 4096 bytes or fewer and is absorbed above that, the
//! command keeping its own status. A single printed line
//! holding 8192 bytes or more before its newline first hands
//! down the part that stands before it, and a failure of that
//! part leaves by status 120 when the part holds from 1 to
//! 4096 bytes and by status 1 when it holds more, and a
//! failure on the line itself, with nothing before it, leaves
//! by status 1. A failed write or flush of the error stream
//! leaves by status 120, the status the Python runtime leaves
//! by on a failed write to a closed stream.

use std::io::Write;

use grandjury::cli::{decode_arg, run, terminal_columns};
use grandjury::pytext::PText;

/// The size the writer hands an accumulation of ordinary
/// lines down at, and the size a single printed line must
/// reach before its newline to stand under the long line
/// rule of PORT-13.
const HAND_DOWN_AT: usize = 8192;

/// The top of the band in which a failed final handing down,
/// or a failed handing down of the part before a long line,
/// stands for status 120.
const STANDS_AT: usize = 4096;

struct BandWriter {
    pending: Vec<u8>,
    total: usize,
    failed: Option<i32>,
}

impl BandWriter {
    fn new() -> BandWriter {
        BandWriter {
            pending: Vec::new(),
            total: 0,
            failed: None,
        }
    }

    /// Write one printed line, its newline standing at its
    /// end when it carries one, through the stream model.
    fn write_line(&mut self, line: &[u8]) {
        if self.failed.is_some() {
            return;
        }
        self.total += line.len();
        let content = line.len() - usize::from(line.last() == Some(&b'\n'));
        if content >= HAND_DOWN_AT {
            if !self.pending.is_empty() {
                let before = self.pending.len();
                let held = std::mem::take(&mut self.pending);
                if std::io::stdout().write_all(&held).is_err() {
                    self.failed = Some(if before <= STANDS_AT { 120 } else { 1 });
                    return;
                }
            }
            if std::io::stdout().write_all(line).is_err() {
                self.failed = Some(1);
            }
            return;
        }
        self.pending.extend_from_slice(line);
        if self.pending.len() >= HAND_DOWN_AT {
            let held = std::mem::take(&mut self.pending);
            if std::io::stdout().write_all(&held).is_err() {
                self.failed = Some(1);
            }
        }
    }

    fn finish(&mut self, code: i32) -> i32 {
        if let Some(failed) = self.failed {
            return failed;
        }
        if !self.pending.is_empty() {
            let size = self.pending.len();
            let held = std::mem::take(&mut self.pending);
            if std::io::stdout().write_all(&held).is_err() {
                if size <= STANDS_AT {
                    return 120;
                }
                return code;
            }
            if size <= STANDS_AT && std::io::stdout().flush().is_err() {
                return 120;
            }
            return code;
        }
        if self.total <= STANDS_AT && std::io::stdout().flush().is_err() {
            return 120;
        }
        code
    }
}

fn main() {
    let raw: Vec<Vec<u8>> = std::env::args_os()
        .skip(1)
        .map(|arg| arg_bytes_of(&arg))
        .collect();
    let args: Vec<PText> = raw.iter().map(|bytes| decode_arg(bytes)).collect();
    let outcome = run(&args, terminal_columns());
    let mut writer = BandWriter::new();
    if !outcome.stderr.is_empty()
        && (std::io::stderr().write_all(&outcome.stderr).is_err()
            || std::io::stderr().flush().is_err())
    {
        std::process::exit(120);
    }
    let mut rest: &[u8] = &outcome.stdout;
    while !rest.is_empty() {
        match rest.iter().position(|byte| *byte == b'\n') {
            Some(split) => {
                writer.write_line(&rest[..=split]);
                rest = &rest[split + 1..];
            }
            None => {
                writer.write_line(rest);
                rest = &[];
            }
        }
    }
    std::process::exit(writer.finish(outcome.code));
}

#[cfg(unix)]
fn arg_bytes_of(arg: &std::ffi::OsString) -> Vec<u8> {
    use std::os::unix::ffi::OsStrExt;
    arg.as_os_str().as_bytes().to_vec()
}

#[cfg(not(unix))]
fn arg_bytes_of(arg: &std::ffi::OsString) -> Vec<u8> {
    arg.to_string_lossy().into_owned().into_bytes()
}
