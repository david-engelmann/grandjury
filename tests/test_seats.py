"""Pins for GJ-5, the juror seat readers over decision models.

Every failure shape the contract names is pinned to a failed
call, the abstention threshold is pinned at its edges, and the
reason construction is pinned word for word. The recorded pilot
fixtures replay through the readers at the end.
"""

import json
import pathlib

from grandjury.seats import (
    response_from_chat_envelope,
    response_from_jev_envelope,
)

ANSWERS = ["red", "blue"]
EXAMPLES = pathlib.Path(__file__).resolve().parent.parent / "examples"


def decision(choice="red", confidence=0.62, probabilities=None):
    if probabilities is None:
        probabilities = {"red": 0.62, "blue": 0.38}
    return {
        "type": "choice",
        "choice": choice,
        "confidence": confidence,
        "probabilities": probabilities,
    }


def jev(decision_value, model="jev-test"):
    return {
        "model": model,
        "answers": {"decision": decision_value},
        "usage": {"input_tokens": 10, "output_tokens": 5},
    }


def chat(content, model="chat-test"):
    return {
        "model": model,
        "choices": [{"message": {"content": content}}],
        "usage": {
            "prompt_tokens": 10,
            "completion_tokens": 5,
            "cost": 0.0001,
        },
    }


def chat_content(choice="blue", confidence=0.75, probabilities=None):
    if probabilities is None:
        probabilities = {"red": 0.25, "blue": 0.75}
    return json.dumps({
        "choice": choice,
        "confidence": confidence,
        "probabilities": probabilities,
    })


def read_jev(envelope, juror="jev-1"):
    return response_from_jev_envelope(juror, envelope, ANSWERS)


def read_chat(envelope, juror="openai-1"):
    return response_from_chat_envelope(juror, envelope, ANSWERS)


def test_a_jev_decision_at_high_confidence_records_a_plain_response():
    assert read_jev(jev(decision())) == {
        "juror": "jev-1",
        "model": "jev-test",
        "answer": "red",
        "reason": "jev-test chose red with confidence 0.62",
    }


def test_a_jev_decision_below_one_half_records_an_abstention():
    assert read_jev(jev(decision(confidence=0.4))) == {
        "juror": "jev-1",
        "model": "jev-test",
        "answer": "red",
        "reason": "jev-test chose red with confidence 0.4 and abstained",
        "abstained": True,
    }


def test_a_jev_decision_at_exactly_one_half_is_not_an_abstention():
    response = read_jev(jev(decision(confidence=0.5)))
    assert response is not None
    assert "abstained" not in response
    assert response["reason"] == (
        "jev-test chose red with confidence 0.5"
    )


def test_a_jev_decision_just_below_one_half_abstains():
    response = read_jev(jev(decision(confidence=0.499999)))
    assert response is not None
    assert response["abstained"] is True


def test_an_integer_confidence_renders_with_no_decimal_point():
    response = read_jev(jev(decision(confidence=1)))
    assert response is not None
    assert response["reason"] == "jev-test chose red with confidence 1"


def test_a_zero_confidence_abstains_and_renders_as_zero():
    response = read_jev(jev(decision(confidence=0)))
    assert response is not None
    assert response["abstained"] is True
    assert response["reason"] == (
        "jev-test chose red with confidence 0 and abstained"
    )


def test_a_fractional_confidence_renders_its_shortest_text():
    response = read_jev(jev(decision(confidence=0.25)))
    assert response is not None
    assert response["reason"] == (
        "jev-test chose red with confidence 0.25 and abstained"
    )
    response = read_jev(jev(decision(confidence=0.7)))
    assert response is not None
    assert response["reason"] == "jev-test chose red with confidence 0.7"


def test_a_choice_that_was_not_offered_still_records():
    response = read_jev(jev(decision(choice="green", confidence=0.9)))
    assert response is not None
    assert response["answer"] == "green"
    assert "abstained" not in response


def test_an_unoffered_choice_below_one_half_records_as_abstained():
    response = read_jev(jev(decision(choice="green", confidence=0.3)))
    assert response is not None
    assert response["answer"] == "green"
    assert response["abstained"] is True


def test_a_jev_envelope_that_is_not_an_object_fails():
    assert read_jev(["not", "an", "object"]) is None


def test_a_jev_envelope_without_a_model_fails():
    envelope = jev(decision())
    del envelope["model"]
    assert read_jev(envelope) is None


def test_a_jev_envelope_with_an_empty_model_fails():
    assert read_jev(jev(decision(), model="")) is None


def test_a_jev_envelope_with_a_model_of_the_wrong_type_fails():
    assert read_jev(jev(decision(), model=7)) is None


def test_a_jev_envelope_with_a_surrogate_in_the_model_fails():
    assert read_jev(jev(decision(), model="jev\ud800x")) is None


def test_a_jev_envelope_without_an_answers_field_fails():
    envelope = jev(decision())
    del envelope["answers"]
    assert read_jev(envelope) is None


def test_a_jev_envelope_without_a_decision_fails():
    envelope = jev(decision())
    envelope["answers"] = {}
    assert read_jev(envelope) is None


def test_a_jev_decision_that_is_not_an_object_fails():
    assert read_jev(jev("red")) is None


def test_a_jev_decision_without_a_choice_fails():
    value = decision()
    del value["choice"]
    assert read_jev(jev(value)) is None


def test_a_jev_decision_with_a_choice_of_the_wrong_type_fails():
    assert read_jev(jev(decision(choice=7))) is None


def test_a_jev_decision_with_a_surrogate_in_the_choice_fails():
    assert read_jev(jev(decision(choice="re\ud800d"))) is None


def test_a_jev_decision_without_a_confidence_fails():
    value = decision()
    del value["confidence"]
    assert read_jev(jev(value)) is None


def test_a_jev_decision_with_a_boolean_confidence_fails():
    assert read_jev(jev(decision(confidence=True))) is None


def test_a_jev_decision_with_a_text_confidence_fails():
    assert read_jev(jev(decision(confidence="0.7"))) is None


def test_a_jev_decision_with_a_nan_confidence_fails():
    assert read_jev(jev(decision(confidence=float("nan")))) is None


def test_a_jev_decision_with_an_infinite_confidence_fails():
    assert read_jev(jev(decision(confidence=float("inf")))) is None


def test_a_jev_decision_with_a_negative_confidence_fails():
    assert read_jev(jev(decision(confidence=-0.1))) is None


def test_a_jev_decision_with_a_confidence_above_one_fails():
    assert read_jev(jev(decision(confidence=1.1))) is None


def test_a_jev_decision_without_probabilities_fails():
    value = decision()
    del value["probabilities"]
    assert read_jev(jev(value)) is None


def test_a_jev_decision_with_probabilities_of_the_wrong_type_fails():
    assert read_jev(jev(decision(probabilities=[0.62, 0.38]))) is None


def test_a_jev_decision_with_a_text_probability_fails():
    value = decision(probabilities={"red": "high", "blue": 0.38})
    assert read_jev(jev(value)) is None


def test_a_jev_decision_with_a_boolean_probability_fails():
    value = decision(probabilities={"red": True, "blue": 0.38})
    assert read_jev(jev(value)) is None


def test_a_jev_decision_with_a_nan_probability_fails():
    value = decision(
        probabilities={"red": float("nan"), "blue": 0.38}
    )
    assert read_jev(jev(value)) is None


def test_a_jev_decision_with_an_infinite_probability_fails():
    value = decision(
        probabilities={"red": float("inf"), "blue": 0.38}
    )
    assert read_jev(jev(value)) is None


def test_a_chat_decision_with_a_nan_probability_fails():
    content = chat_content(
        probabilities={"red": float("nan"), "blue": 0.75}
    )
    assert read_chat(chat(content)) is None


def test_a_chat_decision_records_a_plain_response():
    assert read_chat(chat(chat_content())) == {
        "juror": "openai-1",
        "model": "chat-test",
        "answer": "blue",
        "reason": "chat-test chose blue with confidence 0.75",
    }


def test_a_chat_decision_below_one_half_records_an_abstention():
    response = read_chat(chat(chat_content(confidence=0.25)))
    assert response is not None
    assert response["abstained"] is True
    assert response["reason"] == (
        "chat-test chose blue with confidence 0.25 and abstained"
    )


def test_a_chat_envelope_without_choices_fails():
    envelope = chat(chat_content())
    del envelope["choices"]
    assert read_chat(envelope) is None


def test_a_chat_envelope_with_empty_choices_fails():
    envelope = chat(chat_content())
    envelope["choices"] = []
    assert read_chat(envelope) is None


def test_a_chat_envelope_without_a_message_fails():
    envelope = chat(chat_content())
    envelope["choices"] = [{}]
    assert read_chat(envelope) is None


def test_a_chat_envelope_without_message_content_fails():
    envelope = chat(chat_content())
    envelope["choices"] = [{"message": {}}]
    assert read_chat(envelope) is None


def test_a_chat_envelope_with_content_of_the_wrong_type_fails():
    envelope = chat(chat_content())
    envelope["choices"] = [{"message": {"content": 42}}]
    assert read_chat(envelope) is None


def test_chat_content_that_is_not_json_fails():
    assert read_chat(chat("the answer is blue")) is None


def test_chat_content_that_is_a_json_array_fails():
    assert read_chat(chat("[1, 2]")) is None


def test_chat_content_holding_an_empty_object_fails():
    assert read_chat(chat("{}")) is None


def test_chat_content_with_a_text_confidence_fails():
    content = json.dumps({
        "choice": "blue",
        "confidence": "high",
        "probabilities": {"red": 0.25, "blue": 0.75},
    })
    assert read_chat(chat(content)) is None


def test_a_jev_decision_with_a_newline_in_the_choice_fails():
    assert read_jev(jev(decision(choice="red\nblue"))) is None


def test_a_jev_decision_with_a_carriage_return_in_the_choice_fails():
    assert read_jev(jev(decision(choice="red\rblue"))) is None


def test_a_jev_envelope_with_a_newline_in_the_model_fails():
    assert read_jev(jev(decision(), model="jev\ntest")) is None


def test_a_chat_decision_with_a_newline_in_the_choice_fails():
    assert read_chat(chat(chat_content(choice="blue\ngreen"))) is None


def test_a_chat_decision_with_a_carriage_return_in_the_choice_fails():
    assert read_chat(chat(chat_content(choice="blue\rgreen"))) is None


def test_a_chat_envelope_with_a_newline_in_the_model_fails():
    envelope = chat(chat_content(), model="chat\ntest")
    assert read_chat(envelope) is None


def recorded_envelope(name):
    return json.loads(
        (EXAMPLES / "pilot" / "envelopes" / name).read_text(
            encoding="utf-8"
        )
    )


def pilot_answers(index):
    questions = json.loads(
        (EXAMPLES / "pilot" / "questions.json").read_text(
            encoding="utf-8"
        )
    )
    return questions[index]["answers"]


def test_the_recorded_jev_fixture_replays_to_its_response():
    envelope = recorded_envelope("q03-jev-1.json")
    response = response_from_jev_envelope(
        "jev-1", envelope, pilot_answers(2)
    )
    assert response == {
        "juror": "jev-1",
        "model": "jev-1.13.0",
        "answer": "both cards and cash",
        "reason": (
            "jev-1.13.0 chose both cards and cash with confidence 0.89"
        ),
    }


def test_the_recorded_jev_abstention_fixture_replays():
    envelope = recorded_envelope("q01-jev-1.json")
    response = response_from_jev_envelope(
        "jev-1", envelope, pilot_answers(0)
    )
    assert response == {
        "juror": "jev-1",
        "model": "jev-1.13.0",
        "answer": "open on Sundays",
        "reason": (
            "jev-1.13.0 chose open on Sundays with confidence 0.02 "
            "and abstained"
        ),
        "abstained": True,
    }


def test_the_recorded_chat_fixture_replays_to_its_response():
    envelope = recorded_envelope("q01-openai-1.json")
    response = response_from_chat_envelope(
        "openai-1", envelope, pilot_answers(0)
    )
    assert response == {
        "juror": "openai-1",
        "model": "openai/gpt-6-luna",
        "answer": "open on Sundays",
        "reason": (
            "openai/gpt-6-luna chose open on Sundays with "
            "confidence 0.62"
        ),
    }


def test_the_recorded_chat_abstention_fixture_replays():
    envelope = recorded_envelope("q12-openai-2.json")
    response = response_from_chat_envelope(
        "openai-2", envelope, pilot_answers(11)
    )
    assert response == {
        "juror": "openai-2",
        "model": "openai/gpt-6-luna",
        "answer": "move into the hall",
        "reason": (
            "openai/gpt-6-luna chose move into the hall with "
            "confidence 0.42 and abstained"
        ),
        "abstained": True,
    }
