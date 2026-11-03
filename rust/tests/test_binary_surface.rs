//! Pins for the binary surface of the port repair, driven
//! through the built binary the way the console entry drives
//! the Python surface.
//!
//! PORT-13, the closed stream rule for a single long line,
//! so a pipe whose reader is already gone reads the exit
//! status the writer model states at each side of the line
//! boundary, on the stadium view and on the tally alike. The
//! small output band of PORT-7 stands pinned beside them.

mod shared;

use std::process::{Command, Stdio};

use shared::*;

fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_grandjury")
}

/// A pipe write end whose reader is already gone, the
/// pre-closed surface the pipe pins drive.
#[cfg(unix)]
fn closed_pipe() -> Stdio {
    use std::os::unix::io::FromRawFd;
    extern "C" {
        fn pipe(fds: *mut i32) -> i32;
        fn close(fd: i32) -> i32;
    }
    let mut fds = [0i32; 2];
    unsafe {
        assert_eq!(pipe(fds.as_mut_ptr()), 0);
        close(fds[0]);
        Stdio::from(std::fs::File::from_raw_fd(fds[1]))
    }
}

fn exit_code_on_closed_stdout(args: &[&str], dir: &std::path::Path) -> i32 {
    let mut child = Command::new(binary())
        .args(args)
        .current_dir(dir)
        .stdout(closed_pipe())
        .stderr(Stdio::null())
        .spawn()
        .expect("the binary starts");
    child
        .wait()
        .expect("the binary exits")
        .code()
        .expect("a status")
}

fn write_poll(
    dir: &std::path::Path,
    name: &str,
    question: &str,
    answers: &[&str],
    reason: &str,
) -> String {
    let answers_json: Vec<String> = answers.iter().map(|a| format!("\"{}\"", a)).collect();
    let text = format!(
        "{{\"question\": \"{}\", \"answers\": [{}], \"responses\": [{{\"juror\": \"j1\", \"model\": \"m\", \"answer\": \"{}\", \"reason\": \"{}\"}}]}}",
        question,
        answers_json.join(", "),
        answers[0],
        reason
    );
    let path = dir.join(name);
    std::fs::write(&path, text).expect("a written poll");
    path_arg(&path)
}

#[test]
fn a_small_output_on_a_closed_stdout_exits_120() {
    let scratch = TempDir::new("pipes-small");
    let poll = write_poll(&scratch.path, "poll.json", "Q?", &["a", "b"], "r");
    let code = exit_code_on_closed_stdout(&["tally", &poll], &scratch.path);
    assert_eq!(code, 120);
}

#[test]
fn a_stadium_wall_line_of_8191_bytes_on_a_closed_stdout_exits_1() {
    let scratch = TempDir::new("pipes-stadium-8191");
    // The wall line reads two spaces, the juror, a comma and
    // a space, and then the reason, so a reason of 8185
    // characters carries the line to 8191 bytes before its
    // newline.
    let reason = "x".repeat(8185);
    let poll = write_poll(&scratch.path, "poll.json", "Q?", &["a", "b"], &reason);
    let code = exit_code_on_closed_stdout(&["stadium", &poll], &scratch.path);
    assert_eq!(code, 1);
}

#[test]
fn a_stadium_wall_line_of_8192_bytes_on_a_closed_stdout_exits_120() {
    let scratch = TempDir::new("pipes-stadium-8192");
    let reason = "x".repeat(8186);
    let poll = write_poll(&scratch.path, "poll.json", "Q?", &["a", "b"], &reason);
    let code = exit_code_on_closed_stdout(&["stadium", &poll], &scratch.path);
    assert_eq!(code, 120);
}

#[test]
fn a_tally_line_of_8191_bytes_on_a_closed_stdout_exits_1() {
    let scratch = TempDir::new("pipes-tally-8191");
    // The tally line reads the answer, a comma, the count,
    // and its percentage, eleven bytes past the answer, so an
    // answer of 8180 characters carries the line to 8191
    // bytes before its newline.
    let answer = "a".repeat(8180);
    let poll = write_poll(&scratch.path, "poll.json", "Q?", &[&answer, "b"], "r");
    let code = exit_code_on_closed_stdout(&["tally", &poll], &scratch.path);
    assert_eq!(code, 1);
}

#[test]
fn a_tally_line_of_8192_bytes_on_a_closed_stdout_exits_120() {
    let scratch = TempDir::new("pipes-tally-8192");
    let answer = "a".repeat(8181);
    let poll = write_poll(&scratch.path, "poll.json", "Q?", &[&answer, "b"], "r");
    let code = exit_code_on_closed_stdout(&["tally", &poll], &scratch.path);
    assert_eq!(code, 120);
}
