<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="media/grandjury-lockup-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="media/grandjury-lockup-light.svg">
    <img alt="grandjury" src="media/grandjury-lockup-light.svg" width="380">
  </picture>
</p>

# Grandjury Documentation

Multi-model jury polling, sub-0.5 confidence abstention, flat per-response spend, and twenty-two consensus readings.

## Documentation Map

| Document | What it covers |
| --- | --- |
| [GUIDE.md](GUIDE.md) | Installation, 60-second walkthrough, categorized command matrix, and multi-agent pipeline recipes |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Four-stage processing pipeline, module map, exact arithmetic guarantees, and input fencing |
| [FAQ.md](FAQ.md) | Scope boundaries against sibling Spectacle tools, exit codes, determinism, and common troubleshooting |
| [../FORMATS.md](../FORMATS.md) | Complete version 1 JSON and text input schemas, field rules, and minimal examples |
| [../CONTRACT.md](../CONTRACT.md) | Normative behavioural specification and numbered round obligations |
| [../CHANGELOG.md](../CHANGELOG.md) | Release history and packaging milestones |

## Design Guarantees

- **Pure offline execution.** `grandjury` makes zero network calls, spawns no background daemons, and mutates none of the poll files or wave directories handed to it.
- **Exact rational shares and decimal spend.** Vote shares, lowest-terms fractions, half-up percentages, and flat per-response dollar totals compute over `BigInt`, `BigRational`, and exact decimal arithmetic.
- **Whole-input defect reporting.** Malformed polls and price tables accumulate every reachable schema violation in a single pass before printing any reading.
- **Terminal injection fencing.** Questions, offered answers, juror names, model labels, and reasons reject embedded newlines across both CLI loaders and library guards.
