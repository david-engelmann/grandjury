# grandjury

Ten thousand tiny judges empanelled on any question, for pocket change.

## What this is

Ask one big model a question and you get one confident answer. Grandjury asks the crowd instead. It polls ten thousand tiny cheap models on the same question and renders the verdict as a stadium filling in, a live histogram of the vote plus a wall of one-line reasons from individual jurors. The cost counter stays embarrassingly small the whole time. The point is the absurd scale itself, made visible. Majority, dissent, and the price of finding out, all on one screen.

## Status

Seeded in October 2026. Setup has landed and slice 2 is built and in review. Part of the Spectacle line in the 39 repo storefront perfection programme.

## Programme status

- Closed perfection rounds, 0 of 100. A round counts only when a full cycle of work, audit, review, red team, and approval has closed. Seeding and setup are not rounds.
- Contract, SETUP-1 to SETUP-4 met by the setup slice. The first slice obligations GJ-1 to GJ-4 now stand in full form in CONTRACT.md, and their standing is not claimed here, as slice 2 is in review and no round has closed.
- Slice 2 carries the recorded poll loader that names every malformed field, the tally computed from the responses themselves with spoiled responses counted apart, the stadium view with its histogram and wall of reasons, and the cost counter pricing recorded responses from a stated price table, with the tally, stadium, and cost commands, one synthetic example poll of 124 recorded responses and its golden renderings under examples.
- Local gate, python3 scripts/codepoint_scan.py and python3 -m pytest tests -q. CI runs the same commands.

## Licence

MIT. See LICENSE.
