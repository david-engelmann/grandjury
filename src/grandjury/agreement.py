"""The agreement reading over recorded polls.

The reading is arithmetic over recorded responses and nothing
more. For each poll it states, per jury, the share of the jury
valid responses that each offered answer drew as an exact
fraction, the plurality answer of the jury under a stated tie
rule, and whether the pluralities of the two juries agree. It
carries no verdict about any jury. The obligation this module
carries is GJ-8 in CONTRACT.md, in the Round 3 section.
"""

from fractions import Fraction

from grandjury.polls import _percent


def _jury_reading(poll, responses):
    """Return the reading of one jury in a poll as data.

    The jury line counts stand in the order recorded, valid,
    abstained, and spoiled. Spoiled responses stand outside the
    shares, as they stand outside the shares of the tally, and
    abstained responses stand outside them as their own kind.
    The plurality is the offered answer holding the largest
    valid count, and it stands only when the jury holds at
    least one valid response and one answer holds that count
    alone, under FIX-GJ-8.
    """
    counts = {answer: 0 for answer in poll["answers"]}
    abstained = 0
    spoiled = 0
    for response in responses:
        if response.get("abstained"):
            abstained += 1
        elif response["answer"] in counts:
            counts[response["answer"]] += 1
        else:
            spoiled += 1
    valid = len(responses) - abstained - spoiled
    rows = []
    for answer in poll["answers"]:
        if valid:
            share = Fraction(counts[answer], valid)
        else:
            share = Fraction(0)
        rows.append(
            {
                "answer": answer,
                "count": counts[answer],
                "share": share,
            }
        )
    plurality = None
    largest = max(counts.values(), default=0)
    holders = [
        answer for answer in poll["answers"] if counts[answer] == largest
    ]
    if valid and len(holders) == 1:
        plurality = holders[0]
    return {
        "recorded": len(responses),
        "valid": valid,
        "abstained": abstained,
        "spoiled": spoiled,
        "rows": rows,
        "plurality": plurality,
    }


def _poll_juries(poll):
    """Return the juries of a poll as model and responses pairs.

    A jury is one model string standing in the responses, and
    the juries stand in the order their model strings first
    appear in the responses.
    """
    order = []
    grouped = {}
    for response in poll["responses"]:
        model = response["model"]
        if model not in grouped:
            grouped[model] = []
            order.append(model)
        grouped[model].append(response)
    return [(model, grouped[model]) for model in order]


def agreement_lines(polls):
    """Return the agreement reading of a list of polls as lines.

    The reading holds one section per poll in the order the
    polls stand and a closing line. A section names the
    question, one block per jury, and the agreement line. The
    agreement line reads agree or differ only when the poll
    holds exactly two juries and both hold a plurality, and it
    reads not counted in every other case. The closing line
    counts the agree sections over the sections that read agree
    or differ.
    """
    lines = []
    agreed = 0
    counted = 0
    for poll in polls:
        lines.append(f"question, {poll['question']}")
        pluralities = []
        juries = _poll_juries(poll)
        for model, responses in juries:
            reading = _jury_reading(poll, responses)
            lines.append(
                f"jury, {model}, recorded {reading['recorded']}, "
                f"valid {reading['valid']}, "
                f"abstained {reading['abstained']}, "
                f"spoiled {reading['spoiled']}"
            )
            for row in reading["rows"]:
                lines.append(
                    f"  {row['answer']}, {row['count']}, "
                    f"{row['share']}, {_percent(row['share'])}%"
                )
            plurality = reading["plurality"]
            pluralities.append(plurality)
            lines.append(
                f"plurality, {model}, "
                f"{plurality if plurality is not None else 'none'}"
            )
        if (
            len(juries) == 2
            and pluralities[0] is not None
            and pluralities[1] is not None
        ):
            counted += 1
            if pluralities[0] == pluralities[1]:
                agreed += 1
                lines.append("pluralities, agree")
            else:
                lines.append("pluralities, differ")
        else:
            lines.append("pluralities, not counted")
    lines.append(f"agreements, {agreed} of {counted}")
    return lines
