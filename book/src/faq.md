# Grandjury FAQ

## How does Grandjury differ from Myriad and Stickler?

- **`grandjury`** aggregates answers to a discrete question whose candidate options already exist (`["yes", "no"]`, `["approve", "escalate", "reject"]`). A poll is a vote, not a search.
- **`myriad`** maps an open-ended question before candidate answers exist by branching into sub-questions, findings, and ruled-out paths.
- **`stickler`** checks Python unified diffs for static code smells and arbitrates line-level diff flags.

## What Rust version is required to build Grandjury from source?

`grandjury` uses `slice::as_chunks` (stabilized in Rust `1.88.0`) inside `rust/src/cli.rs`. Use Rust `1.88.0` or newer (`rustup update stable`). Under `FIX-GJ-17` (`CONTRACT.md`), `Cargo.toml` intentionally omits a `rust-version` field while CI pins `1.91.1`.

## Why did `grandjury compare` or `grandjury census` stop reading after `poll-03.json`?

Set directory loaders read consecutive filenames `poll-01.json`, `poll-02.json`, `poll-03.json`, ... and stop at the first missing integer (`GJ-10`, `GJ-12`). If `poll-04.json` was deleted leaving `poll-05.json`, renumber the files consecutively starting from `poll-01.json`.

## How are ties handled when computing a plurality?

Under `FIX-GJ-8`, a plurality requires a strict single leader (`top_count > second_count`) among valid responses. If two or more answers tie for the highest valid count, or if `valid == 0`, the plurality is reported as `none` rather than picking whichever answer appeared first.

## What do the exit codes mean?

- **`0`** Valid poll(s) loaded and reading printed cleanly (note that under `PORT-13`, if `stdout` closes during the final flush when between `4097` and `8191` bytes sit buffered, `BandWriter` returns the command's own status).
- **`1`** Malformed poll, non-UTF-8 file (`GJ-49`), malformed price table, or pairing/set constraint refused with `invalid <surface>` and defect lines on `stdout`, `cost` precision overflow under `PORT-11`, or a closed `stdout` write when `8192` or more bytes sit buffered (`PORT-13`).
- **`2`** Missing or odd `<label> <dir>` pairs, unrecognized flags, or unknown subcommand on `stderr`.
- **`120`** Failed `stderr` stream write, or a closed `stdout` write when `4096` or fewer bytes sit buffered (`PORT-13`).
