# Contributing to Grandjury

Thank you for your interest in `grandjury`. Every change to this repository follows strict contract-first verification, deterministic output rules, and sanitization standards.

## Local Gate

Before opening a pull request, run the four local gate checks from the repository root and confirm that every command exits with status zero.

```sh
cargo run --bin codepoint_scan
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## Core Discipline

- **Contract first.** Behavioural obligations live in `CONTRACT.md` before implementation code changes. Where code and `CONTRACT.md` disagree, `CONTRACT.md` is the specification.
- **Deterministic and offline.** Every library function and test runs deterministically offline with zero network calls and zero floating-point drift.
- **Synthetic fixtures only.** Never include real session logs, private file paths, company or personal names, or production traces in code, tests, fixtures, examples, or issue descriptions. Every fixture is synthetic and authored specifically for this repository.
- **Typography.** Public prose and documentation carry zero em dashes (`U+2014`), zero en dashes (`U+2013`), and zero colons or semicolons inside prose sentences. Run `cargo run --bin codepoint_scan` to verify zero em or en dashes in scanned extensions, check non-scanned prose files such as `CITATION.cff` separately for dashes, and inspect prose sentences to confirm zero colons and semicolons.
- **Developer Certificate of Origin.** Sign off every commit with `git commit -s` so each commit message carries a `Signed-off-by` trailer.
