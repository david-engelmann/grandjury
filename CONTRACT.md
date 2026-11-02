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

## First slice obligations, full form

This section extends the stub section above, which stays standing as the record of the form these obligations started in. The full form below is the text slice 2 is held to. Where the two disagree, the full form governs.

### The recorded poll shape

A recorded poll is one JSON object with the fields named here. The field question is a non empty string. The field answers is a list of at least two strings, each one non empty, and no string offered twice. The field responses is a list that may be empty. Each entry in responses is one JSON object holding a juror field with a non empty string, a model field with a non empty string, an answer field with a string, and a reason field with a non empty string that holds one line and so holds no newline character. One recorded response stands per juror, so no juror string stands twice in one poll. A response whose answer string is not one of the offered answers is spoiled, and spoiled is a tally outcome and never a loader defect. Fields the contract does not name are ignored. Test, tests/test_polls.py.

### The price table shape

A price table is one JSON object holding a prices field whose value is an object. Each key of that object is a model label, and each value is a string naming the price of one response from that model in dollars as a decimal, such as the string 0.0002. Prices are read as exact decimals, never as binary floats, so the arithmetic of the cost counter carries no float error. A price must be finite and must not be negative. Test, tests/test_cost.py.

### GJ-1 full form, the loader names every malformed field and never raises

The loader entry points are load_poll_text and load_poll_file in the grandjury.polls module. Each returns a pair of the poll and the list of defects it found. A clean input returns the poll and an empty defect list. A malformed input returns no poll and a defect list naming every malformed field the loader could reach, and neither entry point ever raises on malformed content. A file that cannot be read returns the single defect named file cannot be read. The defect vocabulary is fixed, and a test pins each phrase. Text that is not JSON at all draws the defect named input is not valid JSON, and JSON nested too deeply to parse draws the same defect. JSON that parses to something other than an object draws the defect named input is not a JSON object. An absent field draws missing field and the field name. A field of the wrong kind draws field and the field name and has the wrong type. An empty question draws field question is empty. A text field holding an unpaired surrogate draws field and the field name and holds an unpaired surrogate, and the same naming applies inside answers and inside responses with their own prefixes below. An answers list holding fewer than two entries draws field answers holds fewer than two answers. An offered answer of the wrong kind draws answer and its index and has the wrong type, an empty one draws answer and its index and is empty, and one holding an unpaired surrogate draws answer and its index and holds an unpaired surrogate. An offered answer that stands twice draws duplicate answer and the answer itself. A responses entry that is not an object draws response and its index and is not an object, and response indexes count from zero. A response missing a field draws response and its index and is missing field and the field name. A response field of the wrong kind draws response and its index and field and the field name and has the wrong type. An empty response field draws response and its index and field and the field name and is empty. A response text field holding an unpaired surrogate draws response and its index and field and the field name and holds an unpaired surrogate. A reason holding a newline character draws response and its index and field reason holds a newline. A juror string that stands twice draws duplicate juror and the juror string itself. One malformed input can draw several defects in one pass, and the loader keeps naming fields after the first defect it finds. Test, tests/test_polls.py.

### GJ-2 full form, the tally is computed from the responses themselves

The tally_poll function in the grandjury.polls module takes a valid poll and returns the tally as data, and the tally is a pure function of the poll. A response is valid when its answer is one of the offered answers, and spoiled otherwise. The tally carries one row per offered answer, in the order the answers are offered, and each row holds the answer, the count of valid responses choosing it, and its share. The share is the exact fraction of valid responses that chose the answer, held as an exact fraction and never as a rounded float. The tally also carries the count of recorded responses, the count of valid responses, and the count of spoiled responses. When there are no valid responses, every share is zero. The render_tally_lines function returns the tally as lines. The first lines name the question, the recorded response count, the valid count, and the spoiled count, and then one line stands per offered answer in offered order, naming the answer, its count, and its share rendered as a percentage with one decimal place, with halves rounding up. Test, tests/test_polls.py.

### GJ-3 full form, the stadium view renders the tally and the wall

The render_stadium function in the grandjury.polls module takes a valid poll and returns the stadium view as lines, and the same poll always renders byte identical lines. The view opens with a header naming the question and then the recorded response count. One histogram line stands per offered answer in offered order. A histogram line names the answer, then a bar of hash characters standing between two pipe characters, then the count, and then the share rendered as a percentage with one decimal place, with halves rounding up. The bar length is proportional to the count against the largest count at a stated maximum width of 40, computed as the count multiplied by 40 and divided by the largest count with the whole part kept. A zero count gets an empty bar, and when the largest count is zero every bar is empty. After the histogram stands the wall of reasons. The wall opens with the line wall of reasons, and then one group stands per offered answer in offered order, even when no reason stands under it. A group opens with the answer itself on its own line, and under it one line stands per valid response that chose that answer, in response order, naming the juror and then the reason, indented by two spaces. When spoiled responses exist, a spoiled section closes the view. The section opens with a line naming the spoiled count, and under it one line stands per spoiled response in response order, naming the juror and then the answer it named, indented by two spaces. When no spoiled responses exist, no spoiled section stands. Test, tests/test_stadium.py, and the golden stadium view held in tests/test_examples.py.

### GJ-4 full form, the cost counter prices recorded responses and makes no live call

The price table loader entry points are load_prices_text and load_prices_file in the grandjury.polls module, and each returns a pair of the price table and the list of defects it found, under the same never raises discipline as GJ-1, with the defect named input is not valid JSON for text that is not JSON, the defect named input is not a JSON object for JSON that is not an object, and the defect named file cannot be read for a file that cannot be read. An absent prices field draws missing field prices, and a prices value that is not an object draws field prices has the wrong type. A price that is not a string draws price for and the model label and has the wrong type. A price string that does not read as a finite decimal draws price for and the model label and is not a decimal string. A price below zero draws price for and the model label and is negative. A model label holding an unpaired surrogate draws a model label holds an unpaired surrogate, and a price string holding an unpaired surrogate draws price for and the model label and holds an unpaired surrogate. The cost_lines function takes a valid poll and a valid price table and returns the cost report as lines, and the counter is a pure function that makes no live call. Every recorded response is priced, spoiled responses included, because a live poll would have paid for those calls too. A response whose model stands in the table adds that price to the priced total once. The report names the question, the recorded response count, the priced response count with the priced total rendered in dollars with six decimal places, and the unpriced response count. When unpriced responses exist, one line stands per unpriced model in the order those models first appear in the responses, naming the model and its response count. A model missing from the table is never silently priced at zero. Sums are computed as exact decimals, and a total that needs rounding to six decimal places rounds with halves up. Test, tests/test_cost.py.

### The command line surface

The grandjury.cli module provides a main function and a console script named grandjury with three commands. The tally command takes one poll file and prints the tally lines. The stadium command takes one poll file and prints the stadium view. The cost command takes one poll file and one price table file and prints the cost report. A malformed poll prints its defects and the command exits with status one, and a malformed price table does the same on the cost command. Every printed report is deterministic, so the same inputs always print the same bytes. Test, tests/test_cli.py.

## Scan scope amendment

SETUP-3 says the codepoint scan counts U+2014 and U+2013 in every Markdown and text file in the repo. This amendment states the scope the scan enforces as a rule, in line with the Round 1 repairs in paceghost and corked. The scan counts the two codepoints in files whose suffix is .md, .txt, .py, .json, .toml, .yml, or .yaml, and in extensionless files, and it skips files under the ignored directories named in the scan itself. The original SETUP-3 text stands above as the record of the form the obligation started in.

## Round 1 repair amendments

This section states the repairs the Round 1 reviews required. Each amendment names the obligation it amends, and every earlier text stays standing. Where an amendment and an earlier text disagree, the amendment governs.

### FIX-GJ-1, amending GJ-4

The sums of GJ-4 are computed at a working precision that covers the inputs exactly. The precision is sized from the prices being summed, taking in the largest price magnitude, the least significant place among those prices, the count of priced responses, and the six decimal places of the rendered total, so no price the price table loader accepts can carry a total past the precision in force. No bound on the magnitude of a price is stated or imposed anywhere in this contract. A total is rounded only at the six place rendering GJ-4 already states, with halves rounding up, and no rounding happens inside the sum itself. Test, tests/test_cost.py and tests/test_cli.py.

### FIX-GJ-2, amending GJ-4

A price string under GJ-4 follows one exact grammar. The string is a run of decimal digits with an optional decimal point and fraction digits after it, or a decimal point followed by fraction digits with no run before it, and either shape may carry an exponent part and an optional leading minus sign. The exponent part is the letter e or the letter E, followed by an optional sign and a run of digits. The string holds nothing else, holding no surrounding whitespace, no underscores, no plus sign at its head, and no NaN or Infinity spelling in any case. A string outside this grammar draws price for and the model label and is not a decimal string, the defect the full form already names for a string that does not read as a finite decimal. A string inside the grammar whose value is below zero draws price for and the model label and is negative, as the full form already states. Test, tests/test_cost.py.

### FIX-GJ-3, amending GJ-1

A reason holding a carriage return character draws response and its index and field reason holds a newline, the same defect a reason holding a newline character draws under GJ-1. A carriage return ends a line in the renderings just as a newline does, so the holds one line rule of the recorded poll shape covers both characters. Test, tests/test_polls.py.
