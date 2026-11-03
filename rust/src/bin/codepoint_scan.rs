//! The codepoint scan command.
//!
//! A thin shell over the library scan in scan.rs, printing
//! the one line the Python scan script prints over the tree
//! at the current directory and leaving by status 1 when
//! either count is nonzero.

use std::io::Write;

fn main() {
    let (em, en) = grandjury::scan::scan_tree(std::path::Path::new("."));
    let line = format!("codepoint scan U+2014={} U+2013={}\n", em, en);
    if std::io::stdout().write_all(line.as_bytes()).is_err() || std::io::stdout().flush().is_err() {
        std::process::exit(120);
    }
    std::process::exit(if em > 0 || en > 0 { 1 } else { 0 });
}
