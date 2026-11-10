# grandjury

Grandjury is a Rust library and command line tool that aggregates recorded answers to a question and prints readings over them.

A poll holds one question, the set of answers offered, and one recorded response per juror, and each response holds the answer the juror chose and a one line reason. Grandjury reads poll files and poll set directories from disk and prints readings over them, the tally of one poll, the agreement of the juries in a poll, and readings over sets of polls read together under labels. Every command runs offline and makes no live call, and the same file always prints the same lines.

[![ci](https://github.com/david-engelmann/grandjury/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/david-engelmann/grandjury/actions/workflows/ci.yml)

```text
question, Should the village library open on Sunday mornings?
responses, 100
valid, 50
spoiled, 0
abstained, 50
open on Sundays, 50, 100.0%
stay closed on Sundays, 0, 0.0%
```

## Install

A Rust toolchain is the only prerequisite, and the crate builds on edition 2021. The suite stands tested at Rust 1.91.1, the toolchain the CI file pins.

```sh
git clone https://github.com/david-engelmann/grandjury.git
cd grandjury
cargo install --path .
```

## Quickstart

Run the tally command over the recorded poll example from the root of the clone.

```sh
grandjury tally examples/recorded_poll.json
```

```text
question, Should the village library open on Sunday mornings?
responses, 100
valid, 50
spoiled, 0
abstained, 50
open on Sundays, 50, 100.0%
stay closed on Sundays, 0, 0.0%
```

## Usage

Each file loads whole or not at all. A malformed poll yields no reading, and its loader names every malformed field it finds. A response naming an answer that was not offered is counted as spoiled and reported apart from the tally of the offered answers. The polls under examples were recorded in earlier rounds from two decision model routes under stated caps, and they stand committed as fixtures with their raw envelopes and their run records.

The tally command prints the count and the share for every answer in one recorded poll.

```sh
grandjury tally examples/recorded_poll.json
```

```text
question, Should the village library open on Sunday mornings?
responses, 100
valid, 50
spoiled, 0
abstained, 50
open on Sundays, 50, 100.0%
stay closed on Sundays, 0, 0.0%
```

The stadium command prints the tally of one poll as a histogram together with a wall of the jurors' one line reasons. The reading over the recorded poll runs to 108 lines and its full output stands at examples/stadium-golden.txt, so the first lines and the last lines stand below with the middle elided.

```sh
grandjury stadium examples/recorded_poll.json
```

```text
question, Should the village library open on Sunday mornings?
responses, 100
open on Sundays |########################################| 50, 100.0%
stay closed on Sundays || 0, 0.0%
wall of reasons
open on Sundays
  openai-1, openai/gpt-6-luna chose open on Sundays with confidence 0.65
...
  jev-48, jev-1.13.0 chose open on Sundays with confidence 0.08 and abstained
  jev-49, jev-1.13.0 chose open on Sundays with confidence 0.02 and abstained
  jev-50, jev-1.13.0 chose stay closed on Sundays with confidence 0.02 and abstained
```

The cost command prints what a recorded poll would have cost live, priced from a stated price table and the response count.

```sh
grandjury cost examples/recorded_poll.json examples/prices.json
```

```text
question, Should the village library open on Sunday mornings?
responses, 100
priced, 50 responses, 0.002600 dollars
unpriced, 50 responses
unpriced model, jev-1.13.0, 50 responses
```

The agreement command prints the agreement reading over recorded polls, stating for each poll and each jury the share every answer drew, the jury's plurality answer, and whether the pluralities of the two juries agree.

```sh
grandjury agreement examples/pilot_poll.json
```

```text
question, Should the village library open on Sunday mornings?
jury, jev-1.13.0, recorded 5, valid 0, abstained 5, spoiled 0
  open on Sundays, 0, 0, 0.0%
  stay closed on Sundays, 0, 0, 0.0%
plurality, jev-1.13.0, none
jury, openai/gpt-6-luna, recorded 5, valid 5, abstained 0, spoiled 0
  open on Sundays, 5, 1, 100.0%
  stay closed on Sundays, 0, 0, 0.0%
plurality, openai/gpt-6-luna, open on Sundays
pluralities, not counted
agreements, 0 of 0
```

The compare command prints the wave comparison reading over two poll sets, stating for each question and each jury what each wave drew and whether the plurality held or moved. The reading over the pilot and the wave runs to 297 lines and its full output stands at examples/wave-comparison-golden.txt, so the first section and the closing lines stand below with the middle elided.

```sh
grandjury compare examples/pilot examples/wave
```

```text
question, Should the village library open on Sunday mornings?
jury, jev-1.13.0
first wave, recorded 5, valid 0, abstained 5, spoiled 0
  open on Sundays, 0, 0, 0.0%
  stay closed on Sundays, 0, 0, 0.0%
plurality, first wave, none
second wave, recorded 5, valid 0, abstained 5, spoiled 0
  open on Sundays, 0, 0, 0.0%
...
  cancel outright, 0, 0, 0.0%
plurality, second wave, move into the hall
movement, openai/gpt-6-luna, held
movements, 16 held, 0 moved, 8 not counted
```

The census command prints the census reading over one or more recorded poll sets named with labels, question by question and jury by jury. The reading over the three recorded sets runs to 856 lines and its full output stands at examples/census-golden.txt, so the first lines and the closing lines stand below with the middle elided.

```sh
grandjury census pilot examples/pilot full examples/full wave examples/wave
```

```text
question, Should the village library open on Sunday mornings?
jury, jev-1.13.0
set, pilot, recorded 5, valid 0, abstained 5, spoiled 0
  open on Sundays, 0, 0, 0.0%
  stay closed on Sundays, 0, 0, 0.0%
plurality, pilot, none
set, wave, recorded 5, valid 0, abstained 5, spoiled 0
  open on Sundays, 0, 0, 0.0%
...
record, polls 64, recorded 567, valid 346, abstained 221, spoiled 0
jury, jev-1.13.0, recorded 285, valid 165, abstained 120, spoiled 0
jury, openai/gpt-6-luna, recorded 282, valid 181, abstained 101, spoiled 0
movements, 16 held, 0 moved, 85 not counted
```

The ledger command prints the juror ledger reading over one or more recorded poll sets named with labels, respondent by respondent.

```sh
grandjury ledger pilot examples/pilot
```

```text
respondent block, jev-1
set, pilot, recorded 12, valid 7, abstained 5, spoiled 0
respondent, jev-1, recorded 12, valid 7, abstained 5, spoiled 0
agreement, 7 of 7, 1
respondent block, jev-2
set, pilot, recorded 12, valid 8, abstained 4, spoiled 0
respondent, jev-2, recorded 12, valid 8, abstained 4, spoiled 0
agreement, 8 of 8, 1
respondent block, jev-3
set, pilot, recorded 12, valid 7, abstained 5, spoiled 0
respondent, jev-3, recorded 12, valid 7, abstained 5, spoiled 0
agreement, 7 of 7, 1
respondent block, jev-4
set, pilot, recorded 12, valid 7, abstained 5, spoiled 0
respondent, jev-4, recorded 12, valid 7, abstained 5, spoiled 0
agreement, 7 of 7, 1
respondent block, jev-5
set, pilot, recorded 12, valid 6, abstained 6, spoiled 0
respondent, jev-5, recorded 12, valid 6, abstained 6, spoiled 0
agreement, 6 of 6, 1
respondent block, openai-1
set, pilot, recorded 12, valid 8, abstained 4, spoiled 0
respondent, openai-1, recorded 12, valid 8, abstained 4, spoiled 0
agreement, 8 of 8, 1
respondent block, openai-2
set, pilot, recorded 12, valid 6, abstained 6, spoiled 0
respondent, openai-2, recorded 12, valid 6, abstained 6, spoiled 0
agreement, 6 of 6, 1
respondent block, openai-3
set, pilot, recorded 12, valid 8, abstained 4, spoiled 0
respondent, openai-3, recorded 12, valid 8, abstained 4, spoiled 0
agreement, 8 of 8, 1
respondent block, openai-4
set, pilot, recorded 12, valid 8, abstained 4, spoiled 0
respondent, openai-4, recorded 12, valid 8, abstained 4, spoiled 0
agreement, 8 of 8, 1
respondent block, openai-5
set, pilot, recorded 12, valid 8, abstained 4, spoiled 0
respondent, openai-5, recorded 12, valid 8, abstained 4, spoiled 0
agreement, 8 of 8, 1
record, polls 12, recorded 120, valid 73, abstained 47, spoiled 0
movements, 0 held, 0 moved, 0 not counted
```

The dissent command prints the dissent reading over one or more recorded poll sets named with labels, each jury's plurality, the margin it stood by, and the respondents who stood against it. The reading over the pilot set alone runs to 97 lines, so its first blocks and its closing line stand below with the middle elided, and the full reading over the three recorded sets stands at examples/dissent-golden.txt.

```sh
grandjury dissent pilot examples/pilot
```

```text
question, Should the village library open on Sunday mornings?
jury, jev-1.13.0, recorded 5, valid 0, abstained 5, spoiled 0
plurality, pilot, none
margin, pilot, not counted
question, Should the village library open on Sunday mornings?
jury, openai/gpt-6-luna, recorded 5, valid 5, abstained 0, spoiled 0
plurality, pilot, open on Sundays
margin, pilot, 5
...
margin, pilot, 4
dissents, 24 questions, 17 pluralities, 0 counted
```

The swing command prints the swing reading over one or more recorded poll sets named with labels, the plurality of each asking pooled over the whole poll, the margin it stood by, and the course of each question across its askings.

```sh
grandjury swing pilot examples/pilot
```

```text
question, Should the village library open on Sunday mornings?
asking, pilot, open on Sundays, margin 5
course, held
question, When should the community hall hold its spring clean?
asking, pilot, the first Saturday of April, margin 5
course, held
question, Should the school bake sale take card payments?
asking, pilot, both cards and cash, margin 10
course, held
question, Which night should the book club meet?
asking, pilot, Wednesday, margin 4
course, held
question, Should the allotment gate be locked at dusk?
asking, pilot, lock it at dusk, margin 7
course, held
question, What should the pub quiz do with tied scores?
asking, pilot, a tiebreak question, margin 10
course, held
question, Should the parish newsletter go out by email instead of print?
asking, pilot, both email and print, margin 10
course, held
question, When should the farmers market start in summer?
asking, pilot, eight in the morning, margin 3
course, held
question, Should the scout troop camp by the river or in the woods?
asking, pilot, in the woods, margin 5
course, held
question, What should the choir sing at the winter concert?
asking, pilot, a mix of carols and folk songs, margin 5
course, held
question, Should the bus stop move closer to the shops?
asking, pilot, none, margin not counted
course, never decided
question, How should the village fete handle rain?
asking, pilot, move into the hall, margin 9
course, held
swings, 12 questions, 11 held, 0 swung, 1 never decided, 12 askings
```

The minority command prints the minority reading over one or more recorded poll sets named with labels, the plurality of each asking pooled over the whole poll, the margin it stood by, and the respondents whose valid answer was not the pooled plurality answer.

```sh
grandjury minority pilot examples/pilot
```

```text
question, Should the village library open on Sunday mornings?
plurality, pilot, open on Sundays
margin, pilot, 5
question, When should the community hall hold its spring clean?
plurality, pilot, the first Saturday of April
margin, pilot, 5
question, Should the school bake sale take card payments?
plurality, pilot, both cards and cash
margin, pilot, 10
question, Which night should the book club meet?
plurality, pilot, Wednesday
margin, pilot, 4
question, Should the allotment gate be locked at dusk?
plurality, pilot, lock it at dusk
margin, pilot, 7
question, What should the pub quiz do with tied scores?
plurality, pilot, a tiebreak question
margin, pilot, 10
question, Should the parish newsletter go out by email instead of print?
plurality, pilot, both email and print
margin, pilot, 10
question, When should the farmers market start in summer?
plurality, pilot, eight in the morning
margin, pilot, 3
question, Should the scout troop camp by the river or in the woods?
plurality, pilot, in the woods
margin, pilot, 5
question, What should the choir sing at the winter concert?
plurality, pilot, a mix of carols and folk songs
margin, pilot, 5
question, Should the bus stop move closer to the shops?
plurality, pilot, none
margin, pilot, not counted
question, How should the village fete handle rain?
plurality, pilot, move into the hall
margin, pilot, 9
minorities, 12 questions, 12 askings, 11 pluralities, 0 dissenters
```

The bench command prints the bench reading over one or more recorded poll sets named with labels, model by model, stating the model's answer at each asking pooled over the juror names it answered under, the count of its valid answers, and how many of them stood with the pooled plurality of the asking.

```sh
grandjury bench pilot examples/pilot
```

```text
model, jev-1.13.0
question, Should the village library open on Sunday mornings?
asking, pilot, none, valid 0, with the plurality 0
question, When should the community hall hold its spring clean?
asking, pilot, the first Saturday of April, valid 5, with the plurality 5
question, Should the school bake sale take card payments?
asking, pilot, both cards and cash, valid 5, with the plurality 5
question, Which night should the book club meet?
asking, pilot, Wednesday, valid 4, with the plurality 4
question, Should the allotment gate be locked at dusk?
asking, pilot, lock it at dusk, valid 5, with the plurality 5
question, What should the pub quiz do with tied scores?
asking, pilot, a tiebreak question, valid 5, with the plurality 5
question, Should the parish newsletter go out by email instead of print?
asking, pilot, both email and print, valid 5, with the plurality 5
question, When should the farmers market start in summer?
asking, pilot, eight in the morning, valid 1, with the plurality 1
question, Should the scout troop camp by the river or in the woods?
asking, pilot, none, valid 0, with the plurality 0
question, What should the choir sing at the winter concert?
asking, pilot, none, valid 0, with the plurality 0
question, Should the bus stop move closer to the shops?
asking, pilot, none, valid 0, with the plurality 0
question, How should the village fete handle rain?
asking, pilot, move into the hall, valid 5, with the plurality 5
model, openai/gpt-6-luna
question, Should the village library open on Sunday mornings?
asking, pilot, open on Sundays, valid 5, with the plurality 5
question, When should the community hall hold its spring clean?
asking, pilot, none, valid 0, with the plurality 0
question, Should the school bake sale take card payments?
asking, pilot, both cards and cash, valid 5, with the plurality 5
question, Which night should the book club meet?
asking, pilot, none, valid 0, with the plurality 0
question, Should the allotment gate be locked at dusk?
asking, pilot, lock it at dusk, valid 2, with the plurality 2
question, What should the pub quiz do with tied scores?
asking, pilot, a tiebreak question, valid 5, with the plurality 5
question, Should the parish newsletter go out by email instead of print?
asking, pilot, both email and print, valid 5, with the plurality 5
question, When should the farmers market start in summer?
asking, pilot, eight in the morning, valid 2, with the plurality 2
question, Should the scout troop camp by the river or in the woods?
asking, pilot, in the woods, valid 5, with the plurality 5
question, What should the choir sing at the winter concert?
asking, pilot, a mix of carols and folk songs, valid 5, with the plurality 5
question, Should the bus stop move closer to the shops?
asking, pilot, none, valid 0, with the plurality 0
question, How should the village fete handle rain?
asking, pilot, move into the hall, valid 4, with the plurality 4
benches, 2 models, 73 answers, 73 with the plurality
```

The pool command prints the pool reading over one or more recorded poll sets named with labels, question by question in the order the questions first appear across the sets, stating for every asking the counts of its whole poll pooled as one pool, the count standing behind every offered answer with its share of the valid answers, and the pooled plurality of the asking.

```sh
grandjury pool pilot examples/pilot
```

```text
question, Should the village library open on Sunday mornings?
set, pilot, recorded 10, valid 5, abstained 5, spoiled 0
  open on Sundays, 5, 1, 100.0%
  stay closed on Sundays, 0, 0, 0.0%
plurality, pilot, open on Sundays
question, When should the community hall hold its spring clean?
set, pilot, recorded 10, valid 5, abstained 5, spoiled 0
  the first Saturday of April, 5, 1, 100.0%
  the last Saturday of April, 0, 0, 0.0%
  a weekday evening, 0, 0, 0.0%
plurality, pilot, the first Saturday of April
question, Should the school bake sale take card payments?
set, pilot, recorded 10, valid 10, abstained 0, spoiled 0
  cards only, 0, 0, 0.0%
  cash only, 0, 0, 0.0%
  both cards and cash, 10, 1, 100.0%
plurality, pilot, both cards and cash
question, Which night should the book club meet?
set, pilot, recorded 10, valid 4, abstained 6, spoiled 0
  Tuesday, 0, 0, 0.0%
  Wednesday, 4, 1, 100.0%
  Thursday, 0, 0, 0.0%
plurality, pilot, Wednesday
question, Should the allotment gate be locked at dusk?
set, pilot, recorded 10, valid 7, abstained 3, spoiled 0
  lock it at dusk, 7, 1, 100.0%
  leave it unlocked, 0, 0, 0.0%
  lock it at nine, 0, 0, 0.0%
plurality, pilot, lock it at dusk
question, What should the pub quiz do with tied scores?
set, pilot, recorded 10, valid 10, abstained 0, spoiled 0
  a tiebreak question, 10, 1, 100.0%
  share the prize, 0, 0, 0.0%
  replay the final round, 0, 0, 0.0%
plurality, pilot, a tiebreak question
question, Should the parish newsletter go out by email instead of print?
set, pilot, recorded 10, valid 10, abstained 0, spoiled 0
  email only, 0, 0, 0.0%
  print only, 0, 0, 0.0%
  both email and print, 10, 1, 100.0%
plurality, pilot, both email and print
question, When should the farmers market start in summer?
set, pilot, recorded 10, valid 3, abstained 7, spoiled 0
  seven in the morning, 0, 0, 0.0%
  eight in the morning, 3, 1, 100.0%
  nine in the morning, 0, 0, 0.0%
plurality, pilot, eight in the morning
question, Should the scout troop camp by the river or in the woods?
set, pilot, recorded 10, valid 5, abstained 5, spoiled 0
  by the river, 0, 0, 0.0%
  in the woods, 5, 1, 100.0%
plurality, pilot, in the woods
question, What should the choir sing at the winter concert?
set, pilot, recorded 10, valid 5, abstained 5, spoiled 0
  carols only, 0, 0, 0.0%
  a mix of carols and folk songs, 5, 1, 100.0%
  folk songs only, 0, 0, 0.0%
plurality, pilot, a mix of carols and folk songs
question, Should the bus stop move closer to the shops?
set, pilot, recorded 10, valid 0, abstained 10, spoiled 0
  move it closer, 0, 0, 0.0%
  keep it where it is, 0, 0, 0.0%
  add a second stop, 0, 0, 0.0%
plurality, pilot, none
question, How should the village fete handle rain?
set, pilot, recorded 10, valid 9, abstained 1, spoiled 0
  carry on in the rain, 0, 0, 0.0%
  move into the hall, 9, 1, 100.0%
  postpone by a week, 0, 0, 0.0%
  cancel outright, 0, 0, 0.0%
plurality, pilot, move into the hall
pools, 12 questions, 12 askings, 73 valid, 11 pluralities
```

The aggregate command prints the aggregate reading over one or more recorded poll sets named with labels, question by question in the order the questions first appear across the sets, stating for every question the counts of all its askings summed as one pool, the count standing behind every offered answer of the question with its share of the pooled valid answers, and the pooled plurality of the question.

```sh
grandjury aggregate pilot examples/pilot
```

```text
question, Should the village library open on Sunday mornings?
pool, recorded 10, valid 5, abstained 5, spoiled 0
  open on Sundays, 5, 1, 100.0%
  stay closed on Sundays, 0, 0, 0.0%
plurality, open on Sundays
question, When should the community hall hold its spring clean?
pool, recorded 10, valid 5, abstained 5, spoiled 0
  the first Saturday of April, 5, 1, 100.0%
  the last Saturday of April, 0, 0, 0.0%
  a weekday evening, 0, 0, 0.0%
plurality, the first Saturday of April
question, Should the school bake sale take card payments?
pool, recorded 10, valid 10, abstained 0, spoiled 0
  cards only, 0, 0, 0.0%
  cash only, 0, 0, 0.0%
  both cards and cash, 10, 1, 100.0%
plurality, both cards and cash
question, Which night should the book club meet?
pool, recorded 10, valid 4, abstained 6, spoiled 0
  Tuesday, 0, 0, 0.0%
  Wednesday, 4, 1, 100.0%
  Thursday, 0, 0, 0.0%
plurality, Wednesday
question, Should the allotment gate be locked at dusk?
pool, recorded 10, valid 7, abstained 3, spoiled 0
  lock it at dusk, 7, 1, 100.0%
  leave it unlocked, 0, 0, 0.0%
  lock it at nine, 0, 0, 0.0%
plurality, lock it at dusk
question, What should the pub quiz do with tied scores?
pool, recorded 10, valid 10, abstained 0, spoiled 0
  a tiebreak question, 10, 1, 100.0%
  share the prize, 0, 0, 0.0%
  replay the final round, 0, 0, 0.0%
plurality, a tiebreak question
question, Should the parish newsletter go out by email instead of print?
pool, recorded 10, valid 10, abstained 0, spoiled 0
  email only, 0, 0, 0.0%
  print only, 0, 0, 0.0%
  both email and print, 10, 1, 100.0%
plurality, both email and print
question, When should the farmers market start in summer?
pool, recorded 10, valid 3, abstained 7, spoiled 0
  seven in the morning, 0, 0, 0.0%
  eight in the morning, 3, 1, 100.0%
  nine in the morning, 0, 0, 0.0%
plurality, eight in the morning
question, Should the scout troop camp by the river or in the woods?
pool, recorded 10, valid 5, abstained 5, spoiled 0
  by the river, 0, 0, 0.0%
  in the woods, 5, 1, 100.0%
plurality, in the woods
question, What should the choir sing at the winter concert?
pool, recorded 10, valid 5, abstained 5, spoiled 0
  carols only, 0, 0, 0.0%
  a mix of carols and folk songs, 5, 1, 100.0%
  folk songs only, 0, 0, 0.0%
plurality, a mix of carols and folk songs
question, Should the bus stop move closer to the shops?
pool, recorded 10, valid 0, abstained 10, spoiled 0
  move it closer, 0, 0, 0.0%
  keep it where it is, 0, 0, 0.0%
  add a second stop, 0, 0, 0.0%
plurality, none
question, How should the village fete handle rain?
pool, recorded 10, valid 9, abstained 1, spoiled 0
  carry on in the rain, 0, 0, 0.0%
  move into the hall, 9, 1, 100.0%
  postpone by a week, 0, 0, 0.0%
  cancel outright, 0, 0, 0.0%
plurality, move into the hall
aggregates, 12 questions, 12 askings, 73 valid, 11 pluralities
```

The canvass command prints the canvass reading over one or more recorded poll sets named with labels, question by question in the order the questions first appear across the sets, stating for every question, per jury, the counts of the jury summed over the askings of the question, the count standing behind every offered answer of the question with its share of the summed valid answers of the jury, and the plurality of the jury at the question.

```sh
grandjury canvass pilot examples/pilot
```

```text
question, Should the village library open on Sunday mornings?
jury, jev-1.13.0, recorded 5, valid 0, abstained 5, spoiled 0
  open on Sundays, 0, 0, 0.0%
  stay closed on Sundays, 0, 0, 0.0%
plurality, none
jury, openai/gpt-6-luna, recorded 5, valid 5, abstained 0, spoiled 0
  open on Sundays, 5, 1, 100.0%
  stay closed on Sundays, 0, 0, 0.0%
plurality, open on Sundays
question, When should the community hall hold its spring clean?
jury, jev-1.13.0, recorded 5, valid 5, abstained 0, spoiled 0
  the first Saturday of April, 5, 1, 100.0%
  the last Saturday of April, 0, 0, 0.0%
  a weekday evening, 0, 0, 0.0%
plurality, the first Saturday of April
jury, openai/gpt-6-luna, recorded 5, valid 0, abstained 5, spoiled 0
  the first Saturday of April, 0, 0, 0.0%
  the last Saturday of April, 0, 0, 0.0%
  a weekday evening, 0, 0, 0.0%
plurality, none
question, Should the school bake sale take card payments?
jury, jev-1.13.0, recorded 5, valid 5, abstained 0, spoiled 0
  cards only, 0, 0, 0.0%
  cash only, 0, 0, 0.0%
  both cards and cash, 5, 1, 100.0%
plurality, both cards and cash
jury, openai/gpt-6-luna, recorded 5, valid 5, abstained 0, spoiled 0
  cards only, 0, 0, 0.0%
  cash only, 0, 0, 0.0%
  both cards and cash, 5, 1, 100.0%
plurality, both cards and cash
question, Which night should the book club meet?
jury, jev-1.13.0, recorded 5, valid 4, abstained 1, spoiled 0
  Tuesday, 0, 0, 0.0%
  Wednesday, 4, 1, 100.0%
  Thursday, 0, 0, 0.0%
plurality, Wednesday
jury, openai/gpt-6-luna, recorded 5, valid 0, abstained 5, spoiled 0
  Tuesday, 0, 0, 0.0%
  Wednesday, 0, 0, 0.0%
  Thursday, 0, 0, 0.0%
plurality, none
question, Should the allotment gate be locked at dusk?
jury, jev-1.13.0, recorded 5, valid 5, abstained 0, spoiled 0
  lock it at dusk, 5, 1, 100.0%
  leave it unlocked, 0, 0, 0.0%
  lock it at nine, 0, 0, 0.0%
plurality, lock it at dusk
jury, openai/gpt-6-luna, recorded 5, valid 2, abstained 3, spoiled 0
  lock it at dusk, 2, 1, 100.0%
  leave it unlocked, 0, 0, 0.0%
  lock it at nine, 0, 0, 0.0%
plurality, lock it at dusk
question, What should the pub quiz do with tied scores?
jury, jev-1.13.0, recorded 5, valid 5, abstained 0, spoiled 0
  a tiebreak question, 5, 1, 100.0%
  share the prize, 0, 0, 0.0%
  replay the final round, 0, 0, 0.0%
plurality, a tiebreak question
jury, openai/gpt-6-luna, recorded 5, valid 5, abstained 0, spoiled 0
  a tiebreak question, 5, 1, 100.0%
  share the prize, 0, 0, 0.0%
  replay the final round, 0, 0, 0.0%
plurality, a tiebreak question
question, Should the parish newsletter go out by email instead of print?
jury, jev-1.13.0, recorded 5, valid 5, abstained 0, spoiled 0
  email only, 0, 0, 0.0%
  print only, 0, 0, 0.0%
  both email and print, 5, 1, 100.0%
plurality, both email and print
jury, openai/gpt-6-luna, recorded 5, valid 5, abstained 0, spoiled 0
  email only, 0, 0, 0.0%
  print only, 0, 0, 0.0%
  both email and print, 5, 1, 100.0%
plurality, both email and print
question, When should the farmers market start in summer?
jury, jev-1.13.0, recorded 5, valid 1, abstained 4, spoiled 0
  seven in the morning, 0, 0, 0.0%
  eight in the morning, 1, 1, 100.0%
  nine in the morning, 0, 0, 0.0%
plurality, eight in the morning
jury, openai/gpt-6-luna, recorded 5, valid 2, abstained 3, spoiled 0
  seven in the morning, 0, 0, 0.0%
  eight in the morning, 2, 1, 100.0%
  nine in the morning, 0, 0, 0.0%
plurality, eight in the morning
question, Should the scout troop camp by the river or in the woods?
jury, jev-1.13.0, recorded 5, valid 0, abstained 5, spoiled 0
  by the river, 0, 0, 0.0%
  in the woods, 0, 0, 0.0%
plurality, none
jury, openai/gpt-6-luna, recorded 5, valid 5, abstained 0, spoiled 0
  by the river, 0, 0, 0.0%
  in the woods, 5, 1, 100.0%
plurality, in the woods
question, What should the choir sing at the winter concert?
jury, jev-1.13.0, recorded 5, valid 0, abstained 5, spoiled 0
  carols only, 0, 0, 0.0%
  a mix of carols and folk songs, 0, 0, 0.0%
  folk songs only, 0, 0, 0.0%
plurality, none
jury, openai/gpt-6-luna, recorded 5, valid 5, abstained 0, spoiled 0
  carols only, 0, 0, 0.0%
  a mix of carols and folk songs, 5, 1, 100.0%
  folk songs only, 0, 0, 0.0%
plurality, a mix of carols and folk songs
question, Should the bus stop move closer to the shops?
jury, jev-1.13.0, recorded 5, valid 0, abstained 5, spoiled 0
  move it closer, 0, 0, 0.0%
  keep it where it is, 0, 0, 0.0%
  add a second stop, 0, 0, 0.0%
plurality, none
jury, openai/gpt-6-luna, recorded 5, valid 0, abstained 5, spoiled 0
  move it closer, 0, 0, 0.0%
  keep it where it is, 0, 0, 0.0%
  add a second stop, 0, 0, 0.0%
plurality, none
question, How should the village fete handle rain?
jury, jev-1.13.0, recorded 5, valid 5, abstained 0, spoiled 0
  carry on in the rain, 0, 0, 0.0%
  move into the hall, 5, 1, 100.0%
  postpone by a week, 0, 0, 0.0%
  cancel outright, 0, 0, 0.0%
plurality, move into the hall
jury, openai/gpt-6-luna, recorded 5, valid 4, abstained 1, spoiled 0
  carry on in the rain, 0, 0, 0.0%
  move into the hall, 4, 1, 100.0%
  postpone by a week, 0, 0, 0.0%
  cancel outright, 0, 0, 0.0%
plurality, move into the hall
canvasses, 12 questions, 24 juries, 17 pluralities, 17 with the plurality
```

The roll command prints the roll reading over one or more recorded poll sets named with labels, jury by jury in the order the juries first appear across the sets, stating for every jury the block of the jury at every question it stands at, the question line and the jury line and the answer rows and the plurality line of the canvass block whole, and the standing of the jury over the record in the standing line that closes its section.

```sh
grandjury roll pilot examples/pilot
```

```text
model, jev-1.13.0
question, Should the village library open on Sunday mornings?
jury, jev-1.13.0, recorded 5, valid 0, abstained 5, spoiled 0
  open on Sundays, 0, 0, 0.0%
  stay closed on Sundays, 0, 0, 0.0%
plurality, none
question, When should the community hall hold its spring clean?
jury, jev-1.13.0, recorded 5, valid 5, abstained 0, spoiled 0
  the first Saturday of April, 5, 1, 100.0%
  the last Saturday of April, 0, 0, 0.0%
  a weekday evening, 0, 0, 0.0%
plurality, the first Saturday of April
question, Should the school bake sale take card payments?
jury, jev-1.13.0, recorded 5, valid 5, abstained 0, spoiled 0
  cards only, 0, 0, 0.0%
  cash only, 0, 0, 0.0%
  both cards and cash, 5, 1, 100.0%
plurality, both cards and cash
question, Which night should the book club meet?
jury, jev-1.13.0, recorded 5, valid 4, abstained 1, spoiled 0
  Tuesday, 0, 0, 0.0%
  Wednesday, 4, 1, 100.0%
  Thursday, 0, 0, 0.0%
plurality, Wednesday
question, Should the allotment gate be locked at dusk?
jury, jev-1.13.0, recorded 5, valid 5, abstained 0, spoiled 0
  lock it at dusk, 5, 1, 100.0%
  leave it unlocked, 0, 0, 0.0%
  lock it at nine, 0, 0, 0.0%
plurality, lock it at dusk
question, What should the pub quiz do with tied scores?
jury, jev-1.13.0, recorded 5, valid 5, abstained 0, spoiled 0
  a tiebreak question, 5, 1, 100.0%
  share the prize, 0, 0, 0.0%
  replay the final round, 0, 0, 0.0%
plurality, a tiebreak question
question, Should the parish newsletter go out by email instead of print?
jury, jev-1.13.0, recorded 5, valid 5, abstained 0, spoiled 0
  email only, 0, 0, 0.0%
  print only, 0, 0, 0.0%
  both email and print, 5, 1, 100.0%
plurality, both email and print
question, When should the farmers market start in summer?
jury, jev-1.13.0, recorded 5, valid 1, abstained 4, spoiled 0
  seven in the morning, 0, 0, 0.0%
  eight in the morning, 1, 1, 100.0%
  nine in the morning, 0, 0, 0.0%
plurality, eight in the morning
question, Should the scout troop camp by the river or in the woods?
jury, jev-1.13.0, recorded 5, valid 0, abstained 5, spoiled 0
  by the river, 0, 0, 0.0%
  in the woods, 0, 0, 0.0%
plurality, none
question, What should the choir sing at the winter concert?
jury, jev-1.13.0, recorded 5, valid 0, abstained 5, spoiled 0
  carols only, 0, 0, 0.0%
  a mix of carols and folk songs, 0, 0, 0.0%
  folk songs only, 0, 0, 0.0%
plurality, none
question, Should the bus stop move closer to the shops?
jury, jev-1.13.0, recorded 5, valid 0, abstained 5, spoiled 0
  move it closer, 0, 0, 0.0%
  keep it where it is, 0, 0, 0.0%
  add a second stop, 0, 0, 0.0%
plurality, none
question, How should the village fete handle rain?
jury, jev-1.13.0, recorded 5, valid 5, abstained 0, spoiled 0
  carry on in the rain, 0, 0, 0.0%
  move into the hall, 5, 1, 100.0%
  postpone by a week, 0, 0, 0.0%
  cancel outright, 0, 0, 0.0%
plurality, move into the hall
standing, 12 questions, 8 pluralities, 8 with the plurality
model, openai/gpt-6-luna
question, Should the village library open on Sunday mornings?
jury, openai/gpt-6-luna, recorded 5, valid 5, abstained 0, spoiled 0
  open on Sundays, 5, 1, 100.0%
  stay closed on Sundays, 0, 0, 0.0%
plurality, open on Sundays
question, When should the community hall hold its spring clean?
jury, openai/gpt-6-luna, recorded 5, valid 0, abstained 5, spoiled 0
  the first Saturday of April, 0, 0, 0.0%
  the last Saturday of April, 0, 0, 0.0%
  a weekday evening, 0, 0, 0.0%
plurality, none
question, Should the school bake sale take card payments?
jury, openai/gpt-6-luna, recorded 5, valid 5, abstained 0, spoiled 0
  cards only, 0, 0, 0.0%
  cash only, 0, 0, 0.0%
  both cards and cash, 5, 1, 100.0%
plurality, both cards and cash
question, Which night should the book club meet?
jury, openai/gpt-6-luna, recorded 5, valid 0, abstained 5, spoiled 0
  Tuesday, 0, 0, 0.0%
  Wednesday, 0, 0, 0.0%
  Thursday, 0, 0, 0.0%
plurality, none
question, Should the allotment gate be locked at dusk?
jury, openai/gpt-6-luna, recorded 5, valid 2, abstained 3, spoiled 0
  lock it at dusk, 2, 1, 100.0%
  leave it unlocked, 0, 0, 0.0%
  lock it at nine, 0, 0, 0.0%
plurality, lock it at dusk
question, What should the pub quiz do with tied scores?
jury, openai/gpt-6-luna, recorded 5, valid 5, abstained 0, spoiled 0
  a tiebreak question, 5, 1, 100.0%
  share the prize, 0, 0, 0.0%
  replay the final round, 0, 0, 0.0%
plurality, a tiebreak question
question, Should the parish newsletter go out by email instead of print?
jury, openai/gpt-6-luna, recorded 5, valid 5, abstained 0, spoiled 0
  email only, 0, 0, 0.0%
  print only, 0, 0, 0.0%
  both email and print, 5, 1, 100.0%
plurality, both email and print
question, When should the farmers market start in summer?
jury, openai/gpt-6-luna, recorded 5, valid 2, abstained 3, spoiled 0
  seven in the morning, 0, 0, 0.0%
  eight in the morning, 2, 1, 100.0%
  nine in the morning, 0, 0, 0.0%
plurality, eight in the morning
question, Should the scout troop camp by the river or in the woods?
jury, openai/gpt-6-luna, recorded 5, valid 5, abstained 0, spoiled 0
  by the river, 0, 0, 0.0%
  in the woods, 5, 1, 100.0%
plurality, in the woods
question, What should the choir sing at the winter concert?
jury, openai/gpt-6-luna, recorded 5, valid 5, abstained 0, spoiled 0
  carols only, 0, 0, 0.0%
  a mix of carols and folk songs, 5, 1, 100.0%
  folk songs only, 0, 0, 0.0%
plurality, a mix of carols and folk songs
question, Should the bus stop move closer to the shops?
jury, openai/gpt-6-luna, recorded 5, valid 0, abstained 5, spoiled 0
  move it closer, 0, 0, 0.0%
  keep it where it is, 0, 0, 0.0%
  add a second stop, 0, 0, 0.0%
plurality, none
question, How should the village fete handle rain?
jury, openai/gpt-6-luna, recorded 5, valid 4, abstained 1, spoiled 0
  carry on in the rain, 0, 0, 0.0%
  move into the hall, 4, 1, 100.0%
  postpone by a week, 0, 0, 0.0%
  cancel outright, 0, 0, 0.0%
plurality, move into the hall
standing, 12 questions, 9 pluralities, 9 with the plurality
rolls, 2 juries, 24 blocks, 17 pluralities, 17 with the plurality
```

The muster command prints the muster reading over one or more recorded poll sets named with labels, question by question in the order the questions first appear across the sets, stating for every question, under each offered answer of the question, the row of that answer in the block of every jury of the question, the count of the valid responses of the question that chose the answer, the count of the juries whose summed count of the answer stands at one or more, and the count of the juries whose block plurality the answer is.

```sh
grandjury muster pilot examples/pilot full examples/full wave examples/wave
```

```text
question, Should the village library open on Sunday mornings?
answer, open on Sundays
  jev-1.13.0, 0, 0, 0.0%
  openai/gpt-6-luna, 10, 1, 100.0%
muster, 10 responses, 1 juries, 1 pluralities
answer, stay closed on Sundays
  jev-1.13.0, 0, 0, 0.0%
  openai/gpt-6-luna, 0, 0, 0.0%
muster, 0 responses, 0 juries, 0 pluralities
musters, 52 questions, 143 answers, 278 juries, 65 pluralities
```

## Status

Seeded in October 2026. Setup has landed and Rounds 1 to 15 have closed, with all slices merged on main. Part of the Spectacle line in the 39 repo storefront perfection programme.

## Programme status

- Closed perfection rounds, 15 of 100. A round counts only when a full cycle of work, audit, review, red team, and approval has closed. Seeding and setup are not rounds.
- Contract, SETUP-1 to SETUP-4 and GJ-1 to GJ-4 met. GJ-1 to GJ-4 stand in full form in CONTRACT.md with the Round 1 repair amendments, and Round 1 closed on the merged slice 2 after three reviews, a repair pass, and an approving re-review.
- Contract, GJ-5 and GJ-6 met. They stand in full form in CONTRACT.md with the Round 2 repair amendments, and Round 2 closed on the merged slice 3 after three reviews, two repair passes, and two re-reviews, the second approving with no findings.
- Contract, GJ-7 and GJ-8 met. They stand in full form in CONTRACT.md with the Round 3 repair amendments, and Round 3 closed on the merged slice 4 after three reviews, a repair pass, and an approving re-review.
- Contract, GJ-9 and GJ-10 met. They stand in full form in CONTRACT.md with the Round 4 repair amendment, and Round 4 closed on the merged slice 5 after three reviews, a repair pass, and an approving re-review.
- Contract, GJ-11 and GJ-12 met. They stand in full form in CONTRACT.md with the Round 5 repair amendment, and Round 5 closed on the merged slice 6 after three reviews, a repair pass, and an approving re-review.
- Contract, GJ-13 and GJ-14 met. They stand in full form in CONTRACT.md with the Round 6 repair amendment, and Round 6 closed on the merged slice 7 after three reviews, a repair pass, and an approving re-review.
- Contract, GJ-15 and GJ-16 met. They stand in full form in CONTRACT.md, and Round 7 closed on the merged slice 8 after three reviews, a repair pass, and an approving re-review.
- Contract, GJ-17 and GJ-18 met. They stand in full form in CONTRACT.md, and Round 8 closed on the merged slice 9 after three reviews, all approving, with no repair pass.
- Contract, GJ-19 and GJ-20 met. They stand in full form in CONTRACT.md, and Round 9 closed on the merged slice 10 after three reviews, a repair pass, and a discharging re-review.
- Contract, GJ-21 and GJ-22 met. They stand in full form in CONTRACT.md with the FIX-GJ-14 repair amendment, and Round 10 closed on the merged slice 11 after three reviews approving, the spec review's two conditions discharged on executed checks.
- Contract, GJ-23 and GJ-24 met. They stand in full form in CONTRACT.md with the FIX-GJ-15 and FIX-GJ-16 repair amendments, and Round 11 closed on the merged slice 12 after three reviews, the red team's rejection repaired under FIX-GJ-16, and a discharging re-review.
- Contract, GJ-25 and GJ-26 met. They stand in full form in CONTRACT.md with the FIX-GJ-17, FIX-GJ-18, and FIX-GJ-19 repair amendments, and Round 12 closed on the merged slice 13 after three reviews, one standards Minor on a test comment repaired.
- Contract, GJ-27 and GJ-28 met. They stand in full form in CONTRACT.md with the FIX-GJ-20 repair amendment, and Round 13 closed on the merged slice 14 after three reviews, a repair pass over the aggregate tests, three approving delta reviews, and a hosted lint repair.
- Contract, GJ-29 and GJ-30 met. They stand in full form in CONTRACT.md, and Round 14 closed on the merged slice 15 after three reviews, a repair pass over the canvass test header, and three approving delta reviews.
- Contract, GJ-31 and GJ-32 met. They stand in full form in CONTRACT.md, and Round 15 closed on the merged slice 16 after three reviews approving with findings and no repair owed, with the standards Minor on the duplicated counting helpers carried to a later round.
- Contract, GJ-33 and GJ-34 stand in full form in CONTRACT.md. Their standing is not claimed here, as Round 16 has not closed.
- The Rust port has landed. It carries the same behavior as the Python package it replaces, checked against the same goldens, and the Python tree stands removed from the repo.
- Slice 17 is built and in review. It carries the muster reading over the three recorded poll sets, the pilot and the full poll and the second wave read together question by question, stating for every question, under each offered answer of the question, the row of that answer in the block of every jury of the question, the count of the valid responses of the question that chose the answer, the count of the juries whose summed count of the answer stands at one or more, and the count of the juries whose block plurality the answer is, with its command and its golden, and the fold of the pooled counting helpers of the canvass and the roll into the agreement module, with its three text pins, and the naming of the models of GJ-32 under FIX-GJ-21, with its pin.
- Slice 16 has landed. It carries the roll reading over the three recorded poll sets, the pilot and the full poll and the second wave read together jury by jury, stating for every jury the block of the jury at every question it stands at, the question line and the jury line and the answer rows and the plurality line of the canvass block whole, and the standing of the jury over the record in the standing line that closes its section, with its command and its golden.
- Slice 15 has landed. It carries the canvass reading over the three recorded poll sets, the pilot and the full poll and the second wave read together question by question, stating for every question, per jury, the counts of the jury summed over the askings of the question, the count standing behind every offered answer of the question with its share of the summed valid answers of the jury, and the plurality of the jury at the question, with its command and its golden.
- Slice 14 has landed. It carries the aggregate reading over the three recorded poll sets, the pilot and the full poll and the second wave read together question by question, stating for every question the counts of all its askings summed as one pool, the count standing behind every offered answer of the question with its share of the pooled valid answers, and the pooled plurality of the question, with its command and its golden, and the glob import gate of FIX-GJ-20 over the binary surface tests, with its attribute line and its pin.
- Slice 13 has landed. It carries the pool reading over the three recorded poll sets, the pilot and the full poll and the second wave read together question by question under their set labels, stating for every asking the counts of its whole poll pooled as one pool, the count standing behind every offered answer with its share of the valid answers, and the pooled plurality of the asking, with its command and its golden, and the formats appendix of FIX-GJ-17 stating the six formats of the repo at version 1 with its compatibility rule, and the reason closure of FIX-GJ-18 over the poll text guard, with its widened guard and its pins, and the resampled fixture of FIX-GJ-19, twelve polls drawn from the recorded pilot envelopes at fifty seats per model with the example poll standing as its first poll.
- Slice 12 has landed. It carries the bench reading over the three recorded poll sets, the pilot and the full poll and the second wave read together model by model under their set labels, stating each model's answer at each asking pooled over the juror names it answered under, the count of its valid answers, and how many of them stood with the pooled plurality of the asking, with its command and its golden, and the residual library fencing closure of FIX-GJ-15 over the tally, the stadium view, the cost reading, and the wave comparison reading, with its widened guard and its pins.
- Slice 11 has landed. It carries the README rebuilt to the fleet storefront standard with the one line description standing in the lede and in the manifest and the tally reading over the recorded poll standing on the first screen, and the library fencing closure of FIX-GJ-14 over the agreement, census, dissent, ledger, minority, and swing readings, with its guard and its pins.
- Slice 10 has landed. It carries the minority reading over the three recorded poll sets, the pilot and the full poll and the second wave read together question by question under their set labels, stating the plurality of each asking pooled over the whole poll, the margin it stood by, and the respondents whose valid answer was not the pooled plurality answer, with its command and its golden.
- Slice 9 has landed. It carries the swing reading over the three recorded poll sets, the pilot and the full poll and the second wave read together question by question under their set labels, stating the plurality of each asking pooled over the whole poll, the margin it stood by, and the course of each question across its askings, with its command and its golden.
- Slice 8 has landed. It carries the dissent reading over the three recorded poll sets, the pilot and the full poll and the second wave read together asking by asking under their set labels, stating each jury plurality, the margin it stood by, and the respondents who stood against it, with its command and its golden.
- Slice 7 has landed. It carries the juror ledger reading over the three recorded poll sets, the pilot and the full poll and the second wave read together respondent by respondent under their set labels, with its command and its golden.
- Slice 6 has landed. It carries the census reading over the three recorded poll sets, the pilot and the full poll and the second wave read together question by question and jury by jury under their set labels, with its command and its golden.
- Slice 5 has landed. It carries the second wave poll over both decision model routes, the twelve pilot questions asked again with five seats per question per model and the raw envelopes and the run record under examples, and the wave comparison reading over the pilot and wave polls with its command and its golden.
- Slice 4 has landed. It carries the full live poll over both decision model routes, forty synthetic questions with five seats per question per model and the raw envelopes and the run record under examples, and the agreement reading over the recorded responses with its command and its golden.
- Slice 3 has landed. It carries the juror seat readers over the two decision model routes, the abstained response kind through the tally, the stadium, and the cost counter, and the recorded pilot over both models, twelve synthetic questions with five seats per question per model and the raw envelopes and the run record under examples.
- Slice 2 has landed. It carries the recorded poll loader that names every malformed field, the tally computed from the responses themselves with spoiled responses counted apart, the stadium view with its histogram and wall of reasons, and the cost counter pricing recorded responses from a stated price table, with the tally, stadium, and cost commands, one synthetic example poll of 124 recorded responses and its golden renderings under examples.
- Local gate, cargo test, cargo clippy --all-targets -- -D warnings, and cargo fmt --check, and the codepoint scan, cargo run --bin codepoint_scan, standing tested on Linux. CI runs the same commands.

## Licence

MIT. See LICENSE.
