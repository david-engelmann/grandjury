# grandjury

Ten thousand tiny judges empanelled on any question, for pocket change.

## What this is

Ask one big model a question and you get one confident answer. Grandjury asks the crowd instead. It polls ten thousand tiny cheap models on the same question and renders the verdict as a stadium filling in, a live histogram of the vote plus a wall of one-line reasons from individual jurors. The cost counter stays embarrassingly small the whole time. The point is the absurd scale itself, made visible. Majority, dissent, and the price of finding out, all on one screen.

## Status

Seeded in October 2026. Setup has landed and Rounds 1 and 2 have closed, with both slices merged on main. Part of the Spectacle line in the 39 repo storefront perfection programme.

## Programme status

- Closed perfection rounds, 2 of 100. A round counts only when a full cycle of work, audit, review, red team, and approval has closed. Seeding and setup are not rounds.
- Contract, SETUP-1 to SETUP-4 and GJ-1 to GJ-4 met. GJ-1 to GJ-4 stand in full form in CONTRACT.md with the Round 1 repair amendments, and Round 1 closed on the merged slice 2 after three reviews, a repair pass, and an approving re-review.
- Contract, GJ-5 and GJ-6 met. They stand in full form in CONTRACT.md with the Round 2 repair amendments, and Round 2 closed on the merged slice 3 after three reviews, two repair passes, and two re-reviews, the second approving with no findings.
- Contract, GJ-7 and GJ-8 now stand in full form in CONTRACT.md under the Round 3 obligations, and their standing is not claimed here, as Round 3 has not closed.
- Slice 4 is built and in review. It carries the full live poll over both decision model routes, forty synthetic questions with five seats per question per model and the raw envelopes and the run record under examples, and the agreement reading over the recorded responses with its command and its golden.
- Slice 3 has landed. It carries the juror seat readers over the two decision model routes, the abstained response kind through the tally, the stadium, and the cost counter, and the recorded pilot over both models, twelve synthetic questions with five seats per question per model and the raw envelopes and the run record under examples.
- Slice 2 has landed. It carries the recorded poll loader that names every malformed field, the tally computed from the responses themselves with spoiled responses counted apart, the stadium view with its histogram and wall of reasons, and the cost counter pricing recorded responses from a stated price table, with the tally, stadium, and cost commands, one synthetic example poll of 124 recorded responses and its golden renderings under examples.
- Local gate, python3 scripts/codepoint_scan.py and python3 -m pytest tests -q. CI runs the same commands.

## Licence

MIT. See LICENSE.
