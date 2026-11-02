"""Juror seats over decision models, and their envelope readers.

A seat names a juror label and a decision model route. The readers
in this module take one recorded envelope, as parsed JSON, and
return the recorded response the seat produced, or None when the
call failed. The readers are pure functions of their inputs and
make no call themselves. The obligations this module carries are
GJ-5 and GJ-6 in CONTRACT.md, in the Round 2 section.
"""

import json
import math
from fractions import Fraction

from grandjury.polls import (
    _holds_line_break,
    _holds_unpaired_surrogate,
)

ABSTAIN_THRESHOLD = Fraction(1, 2)


def _render_confidence(confidence):
    """Render a confidence figure as its shortest round trip text.

    An integer confidence renders with no decimal point, and a
    fractional one renders as the shortest decimal text that
    parses back to the same value.
    """
    if isinstance(confidence, int):
        return str(confidence)
    return repr(confidence)


def _read_model(envelope):
    """Return the envelope model string, or None when unreadable."""
    if not isinstance(envelope, dict):
        return None
    model = envelope.get("model")
    if not isinstance(model, str) or model == "":
        return None
    if _holds_unpaired_surrogate(model):
        return None
    if _holds_line_break(model):
        return None
    return model


def _is_number(value):
    """Return whether a JSON value is a number and not a boolean."""
    return isinstance(value, (int, float)) and not isinstance(
        value, bool
    )


def _response_from_decision(juror, model, decision):
    """Build the recorded response from one decision, or None.

    The decision must hold a choice string, a finite confidence
    inside the range from zero to one, and a probabilities object
    whose values are numbers. Anything else is a call the reader
    cannot read, and the seat records nothing. A choice or a
    model string holding a line break also fails the call, so a
    response the reader produces always loads clean under GJ-1.
    Whether the choice was offered is the tally business under
    GJ-2, so the offered answers are not consulted here.
    """
    if not isinstance(decision, dict):
        return None
    choice = decision.get("choice")
    if not isinstance(choice, str):
        return None
    if _holds_unpaired_surrogate(choice):
        return None
    if _holds_line_break(choice):
        return None
    confidence = decision.get("confidence")
    if not _is_number(confidence):
        return None
    if not math.isfinite(confidence):
        return None
    if confidence < 0 or confidence > 1:
        return None
    probabilities = decision.get("probabilities")
    if not isinstance(probabilities, dict):
        return None
    for value in probabilities.values():
        if not _is_number(value) or not math.isfinite(value):
            return None
    reason = (
        f"{model} chose {choice} with confidence "
        f"{_render_confidence(confidence)}"
    )
    response = {
        "juror": juror,
        "model": model,
        "answer": choice,
        "reason": reason,
    }
    if Fraction(confidence) < ABSTAIN_THRESHOLD:
        response["reason"] = reason + " and abstained"
        response["abstained"] = True
    return response


def response_from_jev_envelope(juror, envelope, answers):
    """Return the recorded response from one Jev envelope, or None.

    The envelope is the Typesafe System One response as parsed
    JSON, holding the model string and an answers field whose
    decision field holds the decision. The offered answers stand
    in the signature because a seat is filled for a question, and
    the reader does not consult them, for the reason stated in
    _response_from_decision.
    """
    model = _read_model(envelope)
    if model is None:
        return None
    answers_field = envelope.get("answers")
    if not isinstance(answers_field, dict):
        return None
    return _response_from_decision(
        juror, model, answers_field.get("decision")
    )


def response_from_chat_envelope(juror, envelope, answers):
    """Return the recorded response from one chat envelope, or None.

    The envelope is an OpenAI compatible chat completion as parsed
    JSON, holding the model string and a choices field whose first
    entry holds a message whose content is one JSON object holding
    the decision. Content that does not parse as one JSON object
    is a failed call.
    """
    model = _read_model(envelope)
    if model is None:
        return None
    choices = envelope.get("choices")
    if not isinstance(choices, list) or not choices:
        return None
    first = choices[0]
    if not isinstance(first, dict):
        return None
    message = first.get("message")
    if not isinstance(message, dict):
        return None
    content = message.get("content")
    if not isinstance(content, str):
        return None
    try:
        decision = json.loads(content)
    except (ValueError, RecursionError):
        return None
    return _response_from_decision(juror, model, decision)
