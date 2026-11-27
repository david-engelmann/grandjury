# grandjury formats

This reference documents the six version 1 JSON file formats and the wave directory layout `grandjury` reads and rebuilds. The specification of record stands in `CONTRACT.md` under `## Appendix, the formats at version 1` (`FIX-GJ-17`, `GJ-1` through `GJ-64`, and `FIX-GJ-1` through `FIX-GJ-31`). Where this reference and `CONTRACT.md` differ, `CONTRACT.md` governs.

All loaders require valid UTF-8 files (`GJ-49`), enforce `PORT-3` JSON container depth (`9997`) and integer digit (`4300`) bounds, reject unpaired UTF-16 surrogates, ignore unnamed extra JSON fields, and accumulate reachable defects in a single pass without panicking. Single-line text rules (`GJ-1`, `FIX-GJ-6`, `FIX-GJ-7`, `FIX-GJ-14` through `FIX-GJ-18`) reject newline (`\n`) and carriage-return (`\r`) characters in `question`, offered `answers`, `juror`, `model`, `responses[].answer`, `reason`, and price-table model keys.

## 1. Recorded poll file (`tally`, `stadium`, `cost`, `agreement`)

One recorded poll over a discrete list of offered answers (`GJ-1`, `GJ-2`, `GJ-5`, `FIX-GJ-6`, `FIX-GJ-17`).

| Field | Type | Required | Rule |
| --- | --- | --- | --- |
| `question` | string | required | Non-empty single-line question text |
| `answers` | array of strings | required | At least two distinct non-empty single-line offered answers |
| `responses` | array of objects | required | Zero or more juror response objects with unique `juror` identifiers |
| `responses[].juror` | string | required | Unique non-empty single-line seat label (for example `jev-1`) |
| `responses[].model` | string | required | Non-empty single-line model identifier |
| `responses[].answer` | string | required | Single-line string (may be empty `""`, which classifies as `spoiled` unless `abstained` is `true`) |
| `responses[].reason` | string | required | Non-empty single-line explanation text |
| `responses[].abstained` | boolean `true` | optional | Must be `true` when present (passing `"abstained": false` is refused as a defect) |

### Response classification rules

Every recorded response falls into exactly one of three disjoint classes (`GJ-2`, `GJ-5`).

- **Abstained.** `abstained` is present with value `true`. Abstained responses never count toward answer shares or pluralities and are never classed as spoiled.
- **Valid.** `abstained` is absent, and `answer` matches an offered string in `answers` byte for byte.
- **Spoiled.** `abstained` is absent, and `answer` does not match any offered string in `answers` (including `""`).

```json
{
  "question": "Should the village library open on Sunday mornings?",
  "answers": [
    "open on Sundays",
    "stay closed on Sundays"
  ],
  "responses": [
    {
      "juror": "openai-1",
      "model": "openai/gpt-6-luna",
      "answer": "open on Sundays",
      "reason": "openai/gpt-6-luna chose open on Sundays with confidence 0.65"
    },
    {
      "juror": "jev-1",
      "model": "jev-1.13.0",
      "answer": "open on Sundays",
      "reason": "jev-1.13.0 chose open on Sundays with confidence 0.02 and abstained",
      "abstained": true
    }
  ]
}
```

## 2. Price table file (`cost`)

`grandjury cost <poll_file> <prices_file>` takes two required positional file paths (`GJ-4`, `FIX-GJ-2`, `FIX-GJ-7`, `FIX-GJ-17`). Each entry maps a single-line model identifier to a non-negative decimal string naming the flat dollar price of **one recorded response** from that model.

| Field | Type | Required | Rule |
| --- | --- | --- | --- |
| `prices` | object | required | Map from non-empty single-line `model` string to non-negative decimal dollar string per response |

```json
{
  "prices": {
    "openai/gpt-6-luna": "0.000052"
  }
}
```

## 3. Poll set directory layout (`compare` and the seventeen set commands)

Eighteen multi-poll directory commands (`compare <first_dir> <second_dir>`, plus `census`, `ledger`, `dissent`, `swing`, `minority`, `bench`, `pool`, `aggregate`, `canvass`, `roll`, `muster`, `margin`, `accord`, `standing`, `answers`, `pairs`, and `shares` taking `<label> <dir> ...` pairs) read numbered poll files from directories (`GJ-10`, `GJ-12`).

- Files must be named sequentially starting at `poll-01.json`, `poll-02.json`, ..., `poll-09.json`, `poll-10.json`, and so on.
- Reading stops at the first missing number after `poll-01.json`.
- If `poll-01.json` itself is absent or unreadable, the command prints `invalid poll` followed by `defect file cannot be read` and exits with status `1`.
- Within one labelled set directory, no two poll files may carry the exact same `question` string.

```text
examples/pilot/
  poll-01.json
  poll-02.json
  ...
  poll-12.json
```

## 4. Questions file (`grandjury::makers`)

Offline campaign makers (`pilot`, `full`, and `wave` in `rust/src/makers.rs`) read a questions list (`examples/pilot/questions.json` and `examples/full/questions.json`, `GJ-6`, `GJ-7`, `FIX-GJ-17`) holding objects with `question` and `answers` so recorded envelopes rebuild into deterministic `poll-NN.json` files.

## 5. Juror decision envelopes (`grandjury::seats`)

`grandjury::seats` normalizes raw API response envelopes from two decision model routes (`GJ-5`, `FIX-GJ-17`, `FIX-GJ-31`).

- **Confidence floor.** Any decision reporting `confidence < 0.5` is recorded with `"abstained": true` automatically.
- **Structured decision route (`answers.decision`).** Requires top-level `model` (string) and `answers.decision` holding `choice` (string), `confidence` (finite number in `[0.0, 1.0]`), and `probabilities` (object of finite numbers).
- **Chat completions route (`choices[0].message.content`).** Reads the **first** entry of `choices` (`FIX-GJ-31`), parses `message.content` as a JSON decision object holding `choice`, `confidence`, and `probabilities`, and pairs it with top-level `model`.

## 6. Run record and incident files (`run_record.json` and `incident.json`)

Offline makers (`rust/src/makers.rs`) also rebuild `run_record.json` (`GJ-6`, `GJ-7`, `GJ-9`, `FIX-GJ-4`, `FIX-GJ-11`, `FIX-GJ-17`), summarizing per-route attempted calls, recorded responses, abstentions, failed calls, token totals extracted from envelope `usage` blocks, decimal spend, and any verbatim `incident.json` accounting object.
