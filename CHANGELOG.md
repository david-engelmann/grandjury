# Changelog

All notable product and packaging changes to `grandjury` stand recorded in this file.

## Unreleased

### Added

- Prebuilt release workflow covering Linux, macOS, and Windows archives alongside a `SHA256SUMS` verification installer (`install.sh`) for Linux and macOS.
- Crate packaging and `cargo-binstall` metadata in `Cargo.toml`, excluding the internal `codepoint_scan` gate binary from published crates, plus `deny.toml` license and advisory checks.
- Documentation suite under `docs/` covering the operator guide, pipeline architecture, multi-agent orchestration recipes, and frequently asked questions, paired with dark and light vector diagrams under `docs/media/`.

## 0.1.0

### Added

- Poll loader validating version 1 multi model juror responses, offered answer sets, confidence abstentions, spoiled ballots, and duplicate juror guards.
- Exact rational vote tallies, ASCII bar chart and wall of reasons stadium views, flat per response decimal price accounting, and within model agreement readings.
- Juror seat envelope parsers handling structured JSON decision payloads and chat completion responses with automatic sub 0.5 confidence abstention.
- Eighteen multi poll and multi wave directory readings covering wave comparisons, census, respondent ledgers, dissent, swing, minority, bench, pool, aggregate, canvass, roll, muster, margin, accord, standing, answers, pairs, and shares.
- Offline pilot, full, and second wave makers reproducing committed poll artifacts byte for byte from recorded envelopes.
