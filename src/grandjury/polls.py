"""Polls, their loader, their tally, the stadium view, and the cost
counter for grandjury.

Every function in this module is a pure function of its inputs, and
the loaders never raise on malformed content. The obligations this
module carries are GJ-1 to GJ-4 in CONTRACT.md, in the full form
section, and the Round 2 extension of the recorded poll shape with
its abstained response kind in the Round 2 section. The counter
runs on recorded polls and makes no live call.
"""

import json
import pathlib
import re
from decimal import (
    MAX_EMAX,
    MIN_EMIN,
    Decimal,
    InvalidOperation,
    ROUND_HALF_UP,
    localcontext,
)
from fractions import Fraction

MAX_BAR = 40
SIX_PLACES = Decimal("0.000001")
PRICE_GRAMMAR = re.compile(
    r"-?(?:[0-9]+(?:\.[0-9]+)?|\.[0-9]+)(?:[eE][+-]?[0-9]+)?"
)


def _holds_unpaired_surrogate(text):
    """Return whether a string fails UTF-8 encoding.

    A string holding an unpaired surrogate cannot be encoded, and a
    poll carrying one cannot be printed, so the loaders name it as
    a defect instead of letting it reach the printer.
    """
    try:
        text.encode("utf-8")
    except UnicodeEncodeError:
        return True
    return False


def _holds_line_break(text):
    """Return whether a string holds a newline or a carriage return.

    Every rendering prints one field to a line, so a text value
    holding a line break forges the structure of the printed
    output, and the loaders name it as a defect instead of
    letting it reach the printer. A value a defect message
    names is held to the same rule, because the command line
    prints the defect list. FIX-GJ-3, FIX-GJ-6, and FIX-GJ-7
    in CONTRACT.md state the rule.
    """
    return "\n" in text or "\r" in text


def _check_response_text(entry, index, name, defects, allow_empty):
    """Validate one text field of a response and return its value.

    Returns None when the field drew a defect, so the caller can
    tell a clean value from a defective one by the defect count.
    """
    if name not in entry:
        defects.append(f"response {index} is missing field {name}")
        return None
    value = entry[name]
    if not isinstance(value, str):
        defects.append(
            f"response {index} field {name} has the wrong type"
        )
        return None
    if value == "" and not allow_empty:
        defects.append(f"response {index} field {name} is empty")
        return None
    if _holds_unpaired_surrogate(value):
        defects.append(
            f"response {index} field {name} holds an unpaired surrogate"
        )
        return None
    if _holds_line_break(value):
        defects.append(
            f"response {index} field {name} holds a newline"
        )
        return None
    return value


def _load_response(entry, index, defects):
    """Validate one response entry and return its clean record.

    The returned record is None when the entry drew any defect.
    """
    before = len(defects)
    if not isinstance(entry, dict):
        defects.append(f"response {index} is not an object")
        return None, None
    juror = _check_response_text(entry, index, "juror", defects, False)
    model = _check_response_text(entry, index, "model", defects, False)
    answer = _check_response_text(entry, index, "answer", defects, True)
    reason = _check_response_text(entry, index, "reason", defects, False)
    abstained = False
    if "abstained" in entry:
        if entry["abstained"] is True:
            abstained = True
        else:
            defects.append(
                f"response {index} field abstained is not true"
            )
    if len(defects) != before:
        return None, juror
    clean = {
        "juror": juror,
        "model": model,
        "answer": answer,
        "reason": reason,
    }
    if abstained:
        clean["abstained"] = True
    return clean, juror


def load_poll_text(text):
    """Parse poll text into a poll, naming every defect found.

    Returns a pair of the poll and the defect list. A malformed
    input returns no poll, and this function never raises on bad
    content. A response naming an answer that was not offered is
    spoiled, which is a tally outcome and not a defect here.
    """
    try:
        data = json.loads(text)
    except (ValueError, RecursionError):
        return None, ["input is not valid JSON"]
    if not isinstance(data, dict):
        return None, ["input is not a JSON object"]
    defects = []
    question = None
    if "question" not in data:
        defects.append("missing field question")
    elif not isinstance(data["question"], str):
        defects.append("field question has the wrong type")
    elif data["question"] == "":
        defects.append("field question is empty")
    elif _holds_unpaired_surrogate(data["question"]):
        defects.append("field question holds an unpaired surrogate")
    elif _holds_line_break(data["question"]):
        defects.append("field question holds a newline")
    else:
        question = data["question"]
    answers = None
    if "answers" not in data:
        defects.append("missing field answers")
    elif not isinstance(data["answers"], list):
        defects.append("field answers has the wrong type")
    else:
        if len(data["answers"]) < 2:
            defects.append("field answers holds fewer than two answers")
        clean_answers = []
        seen_answers = set()
        for index, entry in enumerate(data["answers"]):
            if not isinstance(entry, str):
                defects.append(f"answer {index} has the wrong type")
                continue
            if entry == "":
                defects.append(f"answer {index} is empty")
                continue
            if _holds_unpaired_surrogate(entry):
                defects.append(
                    f"answer {index} holds an unpaired surrogate"
                )
                continue
            if _holds_line_break(entry):
                defects.append(f"answer {index} holds a newline")
                continue
            if entry in seen_answers:
                defects.append(f"duplicate answer {entry}")
                continue
            seen_answers.add(entry)
            clean_answers.append(entry)
        answers = clean_answers
    responses = None
    if "responses" not in data:
        defects.append("missing field responses")
    elif not isinstance(data["responses"], list):
        defects.append("field responses has the wrong type")
    else:
        clean_responses = []
        seen_jurors = set()
        for index, entry in enumerate(data["responses"]):
            clean, juror = _load_response(entry, index, defects)
            if juror is not None:
                if juror in seen_jurors:
                    defects.append(f"duplicate juror {juror}")
                else:
                    seen_jurors.add(juror)
            if clean is not None:
                clean_responses.append(clean)
        responses = clean_responses
    if defects:
        return None, defects
    poll = {
        "question": question,
        "answers": answers,
        "responses": responses,
    }
    return poll, []


def load_poll_file(path):
    """Read a poll from disk, with the same contract as text."""
    try:
        text = pathlib.Path(path).read_text(encoding="utf-8")
    except (OSError, UnicodeError):
        return None, ["file cannot be read"]
    return load_poll_text(text)


def tally_poll(poll):
    """Return the tally of a valid poll as data.

    The tally is a pure function of the poll. Each row holds one
    offered answer, in offered order, with the count of valid
    responses choosing it and its share held as an exact fraction
    of the valid responses. Spoiled responses are counted apart,
    and abstained responses are counted apart from both, as their
    own kind under the Round 2 extension.
    """
    counts = {answer: 0 for answer in poll["answers"]}
    spoiled = 0
    abstained = 0
    for response in poll["responses"]:
        if response.get("abstained"):
            abstained += 1
        elif response["answer"] in counts:
            counts[response["answer"]] += 1
        else:
            spoiled += 1
    valid = len(poll["responses"]) - spoiled - abstained
    rows = []
    for answer in poll["answers"]:
        if valid:
            share = Fraction(counts[answer], valid)
        else:
            share = Fraction(0)
        rows.append(
            {"answer": answer, "count": counts[answer], "share": share}
        )
    return {
        "question": poll["question"],
        "responses": len(poll["responses"]),
        "valid": valid,
        "spoiled": spoiled,
        "abstained": abstained,
        "rows": rows,
    }


def _percent(share):
    """Render an exact fraction as a percentage with one decimal.

    The arithmetic is exact integer work, so no float ever rounds
    the figure the wrong way. Halves round up.
    """
    tenths = (2 * share.numerator * 1000 + share.denominator) // (
        2 * share.denominator
    )
    return f"{tenths // 10}.{tenths % 10}"


def render_tally_lines(poll):
    """Return the tally of a valid poll as lines."""
    tally = tally_poll(poll)
    lines = [
        f"question, {tally['question']}",
        f"responses, {tally['responses']}",
        f"valid, {tally['valid']}",
        f"spoiled, {tally['spoiled']}",
    ]
    if tally["abstained"]:
        lines.append(f"abstained, {tally['abstained']}")
    for row in tally["rows"]:
        lines.append(
            f"{row['answer']}, {row['count']}, {_percent(row['share'])}%"
        )
    return lines


def render_stadium(poll):
    """Return the stadium view of a valid poll as lines.

    The view is a pure function of the poll, so the same poll
    always renders byte identical lines. The wall of reasons
    groups the valid responses by the answer they chose, in
    offered order, and a spoiled section closes the view when
    spoiled responses exist.
    """
    tally = tally_poll(poll)
    lines = [
        f"question, {tally['question']}",
        f"responses, {tally['responses']}",
    ]
    largest = max((row["count"] for row in tally["rows"]), default=0)
    for row in tally["rows"]:
        if largest:
            bar = "#" * ((row["count"] * MAX_BAR) // largest)
        else:
            bar = ""
        lines.append(
            f"{row['answer']} |{bar}| {row['count']}, "
            f"{_percent(row['share'])}%"
        )
    by_answer = {answer: [] for answer in poll["answers"]}
    spoiled = []
    abstained = []
    for response in poll["responses"]:
        if response.get("abstained"):
            abstained.append(response)
        elif response["answer"] in by_answer:
            by_answer[response["answer"]].append(response)
        else:
            spoiled.append(response)
    lines.append("wall of reasons")
    for answer in poll["answers"]:
        lines.append(answer)
        for response in by_answer[answer]:
            lines.append(f"  {response['juror']}, {response['reason']}")
    if abstained:
        lines.append(f"abstained, {len(abstained)}")
        for response in abstained:
            lines.append(f"  {response['juror']}, {response['reason']}")
    if spoiled:
        lines.append(f"spoiled, {len(spoiled)}")
        for response in spoiled:
            lines.append(f"  {response['juror']}, {response['answer']}")
    return lines


def load_prices_text(text):
    """Parse price table text into a price table, naming defects.

    Returns a pair of the table and the defect list. The table maps
    each model label to its price as an exact decimal. A malformed
    input returns no table, and this function never raises on bad
    content.
    """
    try:
        data = json.loads(text)
    except (ValueError, RecursionError):
        return None, ["input is not valid JSON"]
    if not isinstance(data, dict):
        return None, ["input is not a JSON object"]
    defects = []
    if "prices" not in data:
        defects.append("missing field prices")
        return None, defects
    raw = data["prices"]
    if not isinstance(raw, dict):
        defects.append("field prices has the wrong type")
        return None, defects
    table = {}
    for model, value in raw.items():
        if _holds_unpaired_surrogate(model):
            defects.append("a model label holds an unpaired surrogate")
            continue
        if _holds_line_break(model):
            defects.append("a model label holds a newline")
            continue
        if not isinstance(value, str):
            defects.append(f"price for {model} has the wrong type")
            continue
        if _holds_unpaired_surrogate(value):
            defects.append(
                f"price for {model} holds an unpaired surrogate"
            )
            continue
        if PRICE_GRAMMAR.fullmatch(value) is None:
            defects.append(f"price for {model} is not a decimal string")
            continue
        try:
            price = Decimal(value)
        except InvalidOperation:
            defects.append(f"price for {model} is not a decimal string")
            continue
        if not price.is_finite():
            defects.append(f"price for {model} is not a decimal string")
            continue
        if price < 0:
            defects.append(f"price for {model} is negative")
            continue
        table[model] = price
    if defects:
        return None, defects
    return table, []


def load_prices_file(path):
    """Read a price table from disk, with the same contract as text."""
    try:
        text = pathlib.Path(path).read_text(encoding="utf-8")
    except (OSError, UnicodeError):
        return None, ["file cannot be read"]
    return load_prices_text(text)


def _working_precision(priced_prices):
    """Size a decimal precision that covers the given prices exactly.

    The precision takes in the largest price magnitude, the least
    significant place among the prices, the count of prices being
    summed (a sum can carry that many extra digits), and the six
    decimal places of the rendered total, so a sum computed at
    this precision never rounds and its six place rendering never
    overflows it. FIX-GJ-1 in CONTRACT.md states the rule.
    """
    largest = max(price.adjusted() for price in priced_prices)
    least_place = min(
        price.as_tuple().exponent for price in priced_prices
    )
    carry = len(str(len(priced_prices)))
    span = largest - least_place + 1 + carry
    rendering = largest + carry + 7
    return max(28, span, rendering) + 2


def cost_lines(poll, prices):
    """Return the cost report for a valid poll as lines.

    Every recorded response is priced, spoiled responses included.
    A response whose model stands in the table adds its price to
    the priced total once. A model missing from the table is named
    with its response count, in the order such models first appear
    in the responses, and is never silently priced at zero. Sums
    are exact decimals computed at a precision sized to the prices
    themselves, and the total renders in dollars with six decimal
    places, halves rounding up.
    """
    priced_prices = []
    unpriced_counts = {}
    for response in poll["responses"]:
        price = prices.get(response["model"])
        if price is None:
            unpriced_counts[response["model"]] = (
                unpriced_counts.get(response["model"], 0) + 1
            )
        else:
            priced_prices.append(price)
    priced = len(priced_prices)
    unpriced = sum(unpriced_counts.values())
    if priced_prices:
        precision = _working_precision(priced_prices)
    else:
        precision = 28
    with localcontext() as context:
        context.prec = precision
        context.Emax = MAX_EMAX
        context.Emin = MIN_EMIN
        total = Decimal(0)
        for price in priced_prices:
            total += price
        rendered = total.quantize(SIX_PLACES, rounding=ROUND_HALF_UP)
    lines = [
        f"question, {poll['question']}",
        f"responses, {len(poll['responses'])}",
        f"priced, {priced} responses, {rendered} dollars",
        f"unpriced, {unpriced} responses",
    ]
    for model, count in unpriced_counts.items():
        lines.append(f"unpriced model, {model}, {count} responses")
    return lines
