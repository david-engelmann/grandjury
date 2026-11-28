# Grandjury User Guide

Grandjury aggregates recorded multi-model answers to discrete questions, separating valid votes from low-confidence abstentions and spoiled ballots, summing exact per-response API spend, and printing twenty-two consensus, dissent, and wave-stability readings.

## Installation

### Prebuilt binary (Linux and macOS)

```sh
curl -fsSL https://raw.githubusercontent.com/david-engelmann/grandjury/main/install.sh | sh
export PATH="$HOME/.local/bin:$PATH"
```

Pass `--to <DIR>` to install somewhere other than `$HOME/.local/bin`, and ensure the destination directory stands on `PATH`.

### Cargo Binstall

```sh
cargo binstall grandjury
```

### Build from source

Note that `grandjury` uses `slice::as_chunks` from the Rust standard library (`rust/src/cli.rs`) and requires **Rust `1.88.0` or newer** (CI runs on `1.91.1`).

```sh
git clone https://github.com/david-engelmann/grandjury.git
cd grandjury
cargo install --path . --bin grandjury --locked
```

## 60-Second Walkthrough

From the root of a `grandjury` repository checkout (`git clone https://github.com/david-engelmann/grandjury.git && cd grandjury`), run the four poll-file commands on `examples/recorded_poll.json` and `examples/prices.json`, compare two 12-question waves (`examples/pilot` vs `examples/wave`), and run multi-set consensus, dissent, accord, and share readings across all three shipped waves.

```sh
grandjury tally examples/recorded_poll.json
grandjury stadium examples/recorded_poll.json
grandjury cost examples/recorded_poll.json examples/prices.json
grandjury agreement examples/recorded_poll.json
grandjury compare examples/pilot examples/wave
grandjury swing pilot examples/pilot full examples/full wave examples/wave
grandjury dissent pilot examples/pilot full examples/full wave examples/wave
grandjury accord pilot examples/pilot full examples/full wave examples/wave
grandjury shares pilot examples/pilot full examples/full wave examples/wave
```

## Reading Grandjury Output Formats

- **Exact fraction + percentage triples (`GJ-8`).** Share lines print the raw valid count, the exact reduced fraction (`count / valid`, with a denominator of `1` rendered as the integer numerator alone such as `1` or `0`), and the half-up percentage to one decimal place (for example `open on Sundays, 50, 1, 100.0%` or `yes, 3, 3/5, 60.0%`).
- **Automatic sub-`0.5` confidence abstention (`GJ-5`).** When `grandjury::seats` ingests raw LLM envelopes, any decision with `confidence < 0.5` is recorded with `"abstained": true` so uncertain seats abstain cleanly instead of injecting low-confidence guesses into pluralities.
- **Sequential wave directories (`GJ-10`, `GJ-12`).** Multi-poll directory commands read `poll-01.json`, `poll-02.json`, ... sequentially in each directory and stop at the first gap. Always number poll files consecutively starting at `poll-01.json`.

## Command Reference Matrix (22 Commands)

All six version 1 JSON formats (`poll`, `prices`, `questions`, envelopes, `run_record`, and `incident`) and directory layouts stand documented in [../FORMATS.md](../FORMATS.md).

### Poll-file commands (4 commands)

| Command | Arguments | Purpose |
| --- | --- | --- |
| `tally` | `<poll.json>` | Total responses, `valid` / `spoiled` / `abstained` counts, and valid vote percentages per offered answer |
| `stadium` | `<poll.json>` | Horizontal `#` bar chart (`MAX_BAR = 40`), `wall of reasons`, and `abstained` / `spoiled` counts (`GJ-3`) |
| `cost` | `<poll.json> <prices.json>` | Per-model response counts and exact decimal dollar spend from flat per-response prices (`GJ-4`) |
| `agreement` | `<poll.json> [<poll2.json> ...]` | Breakdown by `model` jury across one or more poll files showing per-model shares and pluralities (`GJ-8`) |

### Two-wave directory comparison (1 command)

| Command | Arguments | Purpose |
| --- | --- | --- |
| `compare` | `<first_dir> <second_dir>` | Pairwise wave comparison reporting per-model pluralities and whether each jury `held`, `moved`, or was `not counted` (`GJ-10`) |

### Multi-set campaign commands (17 commands, all take `<label> <dir> [<label2> <dir2> ...]`)

| Category | Commands | What they surface across labelled poll sets |
| --- | --- | --- |
| Cross-wave consensus & courses (6) | `census`, `pool`, `aggregate`, `swing`, `accord`, `margin` | Per-question/per-jury asking histories (`census`), pooled askings (`pool`), grand question totals (`aggregate`), pooled plurality courses `held`/`swung`/`never decided` (`swing`), cross-set `accord, <answer>` vs `accord, none` with closing `in accord` and `divided` counts (`accord`), and winning vote margins (`margin`) |
| Dissent & minority audit (2) | `dissent`, `minority` | Dissenting respondents within each model's own jury (`dissent`) and dissenting respondents against the pooled whole-poll plurality (`minority`) |
| Respondent, jury, & set ledgers (6) | `ledger`, `bench`, `canvass`, `roll`, `muster`, `standing` | Complete per-respondent answer history (`ledger`), model jury presence across sets (`bench`), model-jury aggregations by question, model, and answer (`canvass`, `roll`, `muster`), and per-set question standings (`standing`) |
| Answer & cross-set structure (3) | `answers`, `pairs`, `shares` | Per-answer part counts `held`/`in accord`/`with the pool` (`answers`), pairwise agreement/division between labelled poll sets on shared questions (`pairs`), and each set's fraction of a question's valid responses (`shares`) |

## Multi-Agent Ensemble Triage Recipe

In production multi-agent workflows where discrete classification or triage decisions benefit from multi-model consensus rather than single-model variance, wire `grandjury` as your deterministic ballot box.

1. **Dispatch discrete rubric ballots.** Send the same structured question and offered `answers` (`["approve", "escalate", "reject"]`) to 3 to 5 independent seats across distinct model families.
2. **Normalize envelopes with confidence abstention.** Parse raw route envelopes through `grandjury::seats` into a version 1 `poll.json` file so seats with `confidence < 0.5` record `"abstained": true` automatically (and never emit `"abstained": false`).
3. **Gate automation on consensus, margin, and cost.** Run `grandjury tally poll.json`, `grandjury agreement poll.json`, `grandjury cost poll.json prices.json`, and (across waves) `grandjury accord` and `grandjury minority`. Auto-execute items where sets or model juries agree with a clear margin, and route `divided` items to human review with the dissenting jurors' exact `reason` lines attached.
