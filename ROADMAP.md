# grandjury roadmap

The contract holds what is built. This file holds what is planned, in
the order the programme intends to build it. Nothing here is claimed
as met anywhere in the README programme status until a round closes
on it.

## Decision models in grandjury

Grandjury is the strongest fit in the Spectacle line for a decision
model, because the jury itself is a decision engine. Every juror
response is a small decision, one choice among the offered answers
with a one line reason, and the poll is an aggregate of decisions.
Typesafe Jev is the primary route, connected and verified live on
2026-11-02 through the programme's own connector, answering a choice
question with a chosen label, a confidence, and a probability per
label. The OpenAI decision model is the comparison point, and the
first live round records both models over the same fixtures before
any adoption widens beyond Jev.

Where judgment enters is the juror seat and nowhere else. In a live
poll a juror call asks Jev to choose among the answers the poll
offers, and the chosen label and the confidence come back as data.
A juror whose confidence falls below a threshold the contract will
state abstains, and the abstention is recorded as its own response
kind, distinct from a spoiled ballot, which stays what Round 1 made
it, a tally outcome for a response naming an answer that was not
offered. Judgment also enters in one smaller seat, the comparison
runs that grade juror models against each other over recorded
questions before a live poll trusts one.

What the model decides is a choice per juror, and a score only where
a comparison run asks for one. What stays deterministic is
everything Round 1 built. The loader still names malformed polls and
never raises. The tally is still arithmetic over recorded responses,
with shares as exact fractions. The stadium still renders the same
poll byte identical. The cost counter still prices recorded
responses from a stated price table, and a live poll's responses
become recorded responses the moment they land, priced by the same
counter from their recorded usage. A poll's verdict is never
decided by a model. It is computed from decisions other models made
one juror at a time, and the computation is checkable by anyone.

Tests stay offline on recorded fixtures. A live juror call returns
an envelope holding the model name, the decision with its choice,
confidence, and probabilities, and the token usage, and that
envelope is what a fixture records. The suite replays fixtures
through the same adapter the live slice uses, so the parsing and
tally paths are exercised with no live call in the gate. Live calls
happen only inside a gated live slice whose recordings then join
the fixtures.

The bounded live budget proposal is a pilot of 200 juror calls to
measure the real per call cost against the trivial verification
call, which used 385 input tokens and 31 output tokens, followed by
live polls capped at 1000 juror calls in any one round. Every live
round logs its estimate before it runs and its actual spend after,
in the Spectacle line ledger, under the standing cap of 50 dollars
per repo per round that the owner set on 2026-11-02. The caps
stated here are the standing budget for this repo under the
owner's ample budget ruling of the same day, and a live round runs
against them without a further approval step. Raising a cap means
amending this section with the reason recorded, in the open, and
the spend ledger still closes every round.
