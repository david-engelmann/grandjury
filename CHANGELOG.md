# Changelog

All notable product and packaging changes to `grandjury` stand recorded in this file.

## Unreleased

### Added

- Prebuilt five-target release workflow (`release.yml`) bundling shell completions, JSON Schemas, and manual pages into Linux, macOS, and Windows archives alongside a `SHA256SUMS` verification installer (`install.sh`).
- Crate packaging and `cargo-binstall` metadata in `Cargo.toml`, excluding internal gate binaries and handbook build sources so published `.crate` archives stay compact, plus `deny.toml` license and advisory checks.
- Draft 2020-12 JSON Schemas under `schemas/`, static tab-completion scripts under `completions/` (`bash`, `zsh`, `fish`, `PowerShell`), offline `man/grandjury.1` manual page, and root composite GitHub Action (`action.yml`).
- Searchable `mdBook` operator handbook under `book/` paired with GitHub Pages CI (`docs.yml`), clean-room installer verification (`clean-install.yml`), and three-OS matrix validation (`cross-platform.yml`).
- Documentation and visual media suite under `docs/` and `docs/media/` covering the operator guide, five-tool pipeline cookbook, pipeline architecture, Spectacle Line comparison, measured Apple Silicon release benchmarks, social preview cards, animated CLI showcase, and pinned-compatible storefront README draft.

## 0.1.0

### Added

- Poll loader validating version 1 multi model juror responses, offered answer sets, confidence abstentions, spoiled ballots, and duplicate juror guards.
- Exact rational vote tallies, ASCII bar chart and wall of reasons stadium views, flat per response decimal price accounting, and within model agreement readings.
- Juror seat envelope parsers handling structured JSON decision payloads and chat completion responses with automatic sub 0.5 confidence abstention.
- Eighteen multi poll and multi wave directory readings covering wave comparisons, census, respondent ledgers, dissent, swing, minority, bench, pool, aggregate, canvass, roll, muster, margin, accord, standing, answers, pairs, and shares.
- Offline pilot, full, and second wave makers reproducing committed poll artifacts byte for byte from recorded envelopes.
