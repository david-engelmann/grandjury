<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="media/grandjury-lockup-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="media/grandjury-lockup-light.svg">
    <img alt="grandjury" src="media/grandjury-lockup-light.svg" width="380">
  </picture>
</p>

# Grandjury Documentation

Multi-model jury polling, sub-0.5 confidence abstention, flat per-response spend, and twenty-two consensus readings.

<p align="center">
  <img alt="grandjury terminal demo" src="media/money-shot.gif" width="840">
</p>

## Documentation Map

| Document | What it covers |
| --- | --- |
| [GUIDE.md](guide.md) | Installation, 60-second walkthrough, categorized command matrix, and multi-agent pipeline recipes |
| [ARCHITECTURE.md](architecture.md) | Four-stage processing pipeline, module map, exact arithmetic guarantees, and input fencing |
| [COMPARISON.md](comparison.md) | Spectacle Line 5-tool boundary matrix and comparison against cloud eval SaaS and scripts |
| [PERFORMANCE.md](performance.md) | Measured release binary size, sub-100ms benchmark latencies, and parser memory bounds |
| [FAQ.md](faq.md) | Scope boundaries, exit codes (`0`, `1`, `2`, `120`), determinism, and troubleshooting |
| [FORMATS.md](formats.md) | Normative version 1 JSON and text input schemas, field rules, and minimal examples |
| [schemas/](https://github.com/david-engelmann/grandjury/tree/main/schemas) | Draft 2020-12 JSON Schema definitions (`*.schema.json`) for IDE autocomplete and CI validation |
| [completions/](https://github.com/david-engelmann/grandjury/tree/main/completions) | Ready-to-source `bash`, `zsh`, `fish`, and `PowerShell` tab-completion scripts |
| [CONTRACT.md](https://github.com/david-engelmann/grandjury/blob/main/CONTRACT.md) | Normative behavioural specification and numbered round obligations |
| [CHANGELOG.md](https://github.com/david-engelmann/grandjury/blob/main/CHANGELOG.md) | Release history and packaging milestones |

## Design Guarantees

- **Pure offline execution.** `grandjury` makes zero network calls, spawns no background daemons, and mutates none of the poll files or wave directories handed to it.
- **Exact rational shares and decimal spend.** Vote shares, lowest-terms fractions, half-up percentages, and flat per-response dollar totals compute over `BigInt`, `BigRational`, and exact decimal arithmetic.
- **Whole-input defect reporting.** Malformed polls and price tables accumulate every reachable schema violation in a single pass before printing any reading.
- **Single-line output fencing.** Questions, offered answers, juror names, model labels, chosen answers, and reasons reject unpaired UTF-16 surrogates and line breaks (`\n` and `\r`) across loaders and library guards, though untrusted text displayed in a terminal should still pass through a pager or sanitizer.
