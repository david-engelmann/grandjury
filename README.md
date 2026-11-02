# grandjury

Ten thousand tiny judges empanelled on any question, for pocket change.

## What this is

Ask one big model a question and you get one confident answer. Grandjury asks the crowd instead. It polls ten thousand tiny cheap models on the same question and renders the verdict as a stadium filling in, a live histogram of the vote plus a wall of one-line reasons from individual jurors. The cost counter stays embarrassingly small the whole time. The point is the absurd scale itself, made visible. Majority, dissent, and the price of finding out, all on one screen.

## Status

Seeded in October 2026. Setup has landed and the build has not started. Part of the Spectacle line in the 39 repo storefront perfection programme.

## Programme status

- Closed perfection rounds, 0 of 100. A round counts only when a full cycle of work, audit, review, red team, and approval has closed. Seeding and setup are not rounds.
- Contract, SETUP-1 to SETUP-4 met by the setup slice. The first slice obligations GJ-1 to GJ-4 stand in stub form in CONTRACT.md and are not yet met.
- The first slice is the polling harness and the stadium visual fed entirely by recorded responses, with the cost counter computing what the live poll would have cost.
- Local gate, python3 scripts/codepoint_scan.py and python3 -m pytest tests -q. CI runs the same commands.

## Licence

MIT. See LICENSE.
