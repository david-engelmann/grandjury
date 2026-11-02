# grandjury Contract

This Contract is the executable spec for grandjury. Every obligation is checked by a named test or by a named check. The Contract is reported as met or not met in the README programme status.

## Setup obligations

- SETUP-1. The repo carries AGENTS.md with the canonical template verbatim
  above its marked line and the repo annex below it. Test,
  tests/test_setup.py.
- SETUP-2. The README programme status states the closed round count out
  of 100 and reports this Contract as met or not met. Test,
  tests/test_setup.py.
- SETUP-3. The codepoint scan at scripts/codepoint_scan.py counts U+2014
  and U+2013 in every Markdown and text file in the repo and requires
  zero of each. The scan runs in the local gate and in CI.
- SETUP-4. Every fixture and example committed to this repo is synthetic
  and written for this repo. Nothing real is ever committed. Check, a
  manual sanitization review of every added fixture and example, with
  its result recorded in the gate report.

## First slice obligations, stub form

The full Contract for the first slice is the first build act of this repo. These stubs state the obligations the first slice will be held to, in the form they will be extended.

- GJ-1. A poll holds one question, the set of answers offered, and one recorded response per juror, each response holding the answer chosen and a one line reason. Test, the first slice tests.
- GJ-2. The tally reports the count and the share for every answer, computed from the responses themselves, and a response naming an answer that was not offered is counted as spoiled and reported. Test, the first slice tests.
- GJ-3. The stadium view renders the tally as a histogram and a wall of one line reasons, and the same poll always renders byte identical output. Test, the first slice tests.
- GJ-4. The cost counter computes what the poll would have cost live from a stated price per response and the response count, and the counter runs on recorded polls without any live call. Test, the first slice tests.

## Scan scope amendment

SETUP-3 says the codepoint scan counts U+2014 and U+2013 in every Markdown and text file in the repo. This amendment states the scope the scan enforces as a rule, in line with the Round 1 repairs in paceghost and corked. The scan counts the two codepoints in files whose suffix is .md, .txt, .py, .json, .toml, .yml, or .yaml, and in extensionless files, and it skips files under the ignored directories named in the scan itself. The original SETUP-3 text stands above as the record of the form the obligation started in.
