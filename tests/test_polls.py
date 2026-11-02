"""Pins for GJ-1 and GJ-2, the poll loader and the tally.

Each malformed shape is pinned at its exact defect phrase from the
contract vocabulary, and the tally figures are computed by hand on
small polls so a wrong count or a rounded share fails the test.
"""

import json
from fractions import Fraction

from grandjury.polls import (
    load_poll_file,
    load_poll_text,
    render_tally_lines,
    tally_poll,
)


def response(juror, answer, model="tiny-a", reason="a reason"):
    return {"juror": juror, "model": model, "answer": answer,
            "reason": reason}


def valid_data():
    return {
        "question": "Which door should the agent pick?",
        "answers": ["red", "blue", "green"],
        "responses": [
            response("juror-1", "red"),
            response("juror-2", "blue"),
        ],
    }


def load(data):
    return load_poll_text(json.dumps(data))


def test_a_valid_poll_loads_clean():
    poll, defects = load(valid_data())
    assert defects == []
    assert poll["question"] == "Which door should the agent pick?"
    assert poll["answers"] == ["red", "blue", "green"]
    assert len(poll["responses"]) == 2


def test_an_empty_responses_list_loads_clean():
    data = valid_data()
    data["responses"] = []
    poll, defects = load(data)
    assert defects == []
    assert poll["responses"] == []


def test_text_that_is_not_json_draws_the_named_defect():
    poll, defects = load_poll_text("this is not json")
    assert poll is None
    assert defects == ["input is not valid JSON"]


def test_deeply_nested_input_draws_the_not_valid_json_defect():
    poll, defects = load_poll_text("[" * 100000 + "]" * 100000)
    assert poll is None
    assert defects == ["input is not valid JSON"]


def test_json_that_is_not_an_object_draws_the_named_defect():
    poll, defects = load_poll_text("[1, 2, 3]")
    assert poll is None
    assert defects == ["input is not a JSON object"]


def test_an_unreadable_file_draws_the_named_defect(tmp_path):
    poll, defects = load_poll_file(tmp_path / "missing.json")
    assert poll is None
    assert defects == ["file cannot be read"]


def test_a_missing_question_is_named():
    data = valid_data()
    del data["question"]
    poll, defects = load(data)
    assert poll is None
    assert defects == ["missing field question"]


def test_a_question_of_the_wrong_type_is_named():
    data = valid_data()
    data["question"] = 7
    poll, defects = load(data)
    assert defects == ["field question has the wrong type"]


def test_an_empty_question_is_named():
    data = valid_data()
    data["question"] = ""
    poll, defects = load(data)
    assert defects == ["field question is empty"]


def test_a_question_holding_an_unpaired_surrogate_is_named():
    text = (
        '{"question": "\\ud800x", "answers": ["red", "blue"],'
        ' "responses": []}'
    )
    poll, defects = load_poll_text(text)
    assert poll is None
    assert defects == ["field question holds an unpaired surrogate"]


def test_a_missing_answers_field_is_named():
    data = valid_data()
    del data["answers"]
    poll, defects = load(data)
    assert defects == ["missing field answers"]


def test_answers_of_the_wrong_type_are_named():
    data = valid_data()
    data["answers"] = "red"
    poll, defects = load(data)
    assert defects == ["field answers has the wrong type"]


def test_fewer_than_two_answers_are_named():
    data = valid_data()
    data["answers"] = ["red"]
    poll, defects = load(data)
    assert defects == ["field answers holds fewer than two answers"]


def test_an_answer_of_the_wrong_type_is_named():
    data = valid_data()
    data["answers"] = ["red", 4, "green"]
    poll, defects = load(data)
    assert defects == ["answer 1 has the wrong type"]


def test_an_empty_answer_is_named():
    data = valid_data()
    data["answers"] = ["red", "", "green"]
    poll, defects = load(data)
    assert defects == ["answer 1 is empty"]


def test_an_answer_holding_an_unpaired_surrogate_is_named():
    text = (
        '{"question": "q", "answers": ["red", "\\ud800x"],'
        ' "responses": []}'
    )
    poll, defects = load_poll_text(text)
    assert defects == ["answer 1 holds an unpaired surrogate"]


def test_a_duplicate_answer_is_named():
    data = valid_data()
    data["answers"] = ["red", "blue", "red"]
    poll, defects = load(data)
    assert defects == ["duplicate answer red"]


def test_a_missing_responses_field_is_named():
    data = valid_data()
    del data["responses"]
    poll, defects = load(data)
    assert defects == ["missing field responses"]


def test_responses_of_the_wrong_type_are_named():
    data = valid_data()
    data["responses"] = "many"
    poll, defects = load(data)
    assert defects == ["field responses has the wrong type"]


def test_a_response_that_is_not_an_object_is_named():
    data = valid_data()
    data["responses"] = ["red"]
    poll, defects = load(data)
    assert defects == ["response 0 is not an object"]


def test_a_response_missing_a_field_is_named():
    data = valid_data()
    del data["responses"][0]["reason"]
    poll, defects = load(data)
    assert defects == ["response 0 is missing field reason"]


def test_a_response_field_of_the_wrong_type_is_named():
    data = valid_data()
    data["responses"][1]["model"] = 3
    poll, defects = load(data)
    assert defects == ["response 1 field model has the wrong type"]


def test_an_empty_juror_is_named():
    data = valid_data()
    data["responses"][0]["juror"] = ""
    poll, defects = load(data)
    assert defects == ["response 0 field juror is empty"]


def test_an_empty_reason_is_named():
    data = valid_data()
    data["responses"][0]["reason"] = ""
    poll, defects = load(data)
    assert defects == ["response 0 field reason is empty"]


def test_a_response_field_holding_an_unpaired_surrogate_is_named():
    text = (
        '{"question": "q", "answers": ["red", "blue"], "responses":'
        ' [{"juror": "j1", "model": "\\ud800x", "answer": "red",'
        ' "reason": "fine"}]}'
    )
    poll, defects = load_poll_text(text)
    assert defects == [
        "response 0 field model holds an unpaired surrogate"
    ]


def test_a_reason_holding_a_newline_is_named():
    data = valid_data()
    data["responses"][0]["reason"] = "two\nlines"
    poll, defects = load(data)
    assert defects == ["response 0 field reason holds a newline"]


def test_a_reason_holding_a_carriage_return_is_named():
    data = valid_data()
    data["responses"][0]["reason"] = "two\rlines"
    poll, defects = load(data)
    assert defects == ["response 0 field reason holds a newline"]


def test_a_question_holding_a_newline_is_named():
    data = valid_data()
    data["question"] = "Which door\nshould the agent pick?"
    poll, defects = load(data)
    assert poll is None
    assert defects == ["field question holds a newline"]


def test_a_question_holding_a_carriage_return_is_named():
    data = valid_data()
    data["question"] = "Which door\rshould the agent pick?"
    poll, defects = load(data)
    assert poll is None
    assert defects == ["field question holds a newline"]


def test_an_answer_holding_a_newline_is_named():
    data = valid_data()
    data["answers"] = ["red", "blue\nspoofed", "green"]
    poll, defects = load(data)
    assert poll is None
    assert defects == ["answer 1 holds a newline"]


def test_a_juror_holding_a_newline_is_named():
    data = valid_data()
    data["responses"][0]["juror"] = "juror-1\nspoofed, 99, 100.0%"
    poll, defects = load(data)
    assert poll is None
    assert defects == ["response 0 field juror holds a newline"]


def test_a_model_holding_a_newline_is_named():
    data = valid_data()
    data["responses"][0]["model"] = (
        "tiny-a\nunpriced model, spoof, 9 responses"
    )
    poll, defects = load(data)
    assert poll is None
    assert defects == ["response 0 field model holds a newline"]


def test_a_response_answer_holding_a_carriage_return_is_named():
    data = valid_data()
    data["responses"][0]["answer"] = "red\rspoofed"
    poll, defects = load(data)
    assert poll is None
    assert defects == ["response 0 field answer holds a newline"]


def test_unnamed_fields_of_the_poll_are_ignored():
    data = valid_data()
    data["counts"] = {"red": 99, "blue": 99}
    data["source"] = "a stored figure the tally must not read"
    poll, defects = load(data)
    assert defects == []
    assert set(poll) == {"question", "answers", "responses"}
    tally = tally_poll(poll)
    assert tally["responses"] == 2
    rows = {row["answer"]: row for row in tally["rows"]}
    assert rows["red"]["count"] == 1
    assert rows["blue"]["count"] == 1


def test_unnamed_fields_of_a_response_are_ignored():
    data = valid_data()
    data["responses"][0]["confidence"] = "high"
    data["responses"][0]["seed"] = 42
    poll, defects = load(data)
    assert defects == []
    assert set(poll["responses"][0]) == {
        "juror", "model", "answer", "reason",
    }


def test_a_duplicate_juror_is_named():
    data = valid_data()
    data["responses"][1]["juror"] = "juror-1"
    poll, defects = load(data)
    assert defects == ["duplicate juror juror-1"]


def test_a_spoiled_response_is_not_a_loader_defect():
    data = valid_data()
    data["responses"][0]["answer"] = "purple"
    poll, defects = load(data)
    assert defects == []
    assert poll["responses"][0]["answer"] == "purple"


def test_several_defects_arrive_in_one_pass():
    data = valid_data()
    data["question"] = ""
    data["answers"] = ["red", "red"]
    data["responses"] = [{"juror": "j1"}]
    poll, defects = load(data)
    assert poll is None
    assert "field question is empty" in defects
    assert "duplicate answer red" in defects
    assert "response 0 is missing field model" in defects
    assert len(defects) >= 4


def hand_poll():
    return {
        "question": "Which door should the agent pick?",
        "answers": ["red", "blue", "green"],
        "responses": [
            response("juror-1", "red"),
            response("juror-2", "blue"),
            response("juror-3", "red"),
            response("juror-4", "red"),
            response("juror-5", "blue"),
            response("juror-6", "purple"),
        ],
    }


def test_the_tally_counts_and_shares_are_computed_by_hand():
    poll, defects = load(hand_poll())
    assert defects == []
    tally = tally_poll(poll)
    assert tally["responses"] == 6
    assert tally["valid"] == 5
    assert tally["spoiled"] == 1
    rows = {row["answer"]: row for row in tally["rows"]}
    assert rows["red"]["count"] == 3
    assert rows["red"]["share"] == Fraction(3, 5)
    assert rows["blue"]["count"] == 2
    assert rows["blue"]["share"] == Fraction(2, 5)
    assert rows["green"]["count"] == 0
    assert rows["green"]["share"] == Fraction(0)


def test_the_tally_rows_stand_in_offered_order():
    poll, _ = load(hand_poll())
    tally = tally_poll(poll)
    assert [row["answer"] for row in tally["rows"]] == [
        "red", "blue", "green",
    ]


def test_the_tally_of_an_empty_poll_has_zero_shares():
    data = valid_data()
    data["responses"] = []
    poll, _ = load(data)
    tally = tally_poll(poll)
    assert tally["valid"] == 0
    assert tally["spoiled"] == 0
    assert all(row["share"] == Fraction(0) for row in tally["rows"])
    assert all(row["count"] == 0 for row in tally["rows"])


def test_the_tally_lines_render_the_hand_computed_poll():
    poll, _ = load(hand_poll())
    assert render_tally_lines(poll) == [
        "question, Which door should the agent pick?",
        "responses, 6",
        "valid, 5",
        "spoiled, 1",
        "red, 3, 60.0%",
        "blue, 2, 40.0%",
        "green, 0, 0.0%",
    ]


def test_a_share_renders_as_a_percentage_with_one_decimal():
    data = valid_data()
    data["answers"] = ["red", "blue"]
    data["responses"] = [
        response("juror-1", "red"),
        response("juror-2", "blue"),
        response("juror-3", "blue"),
    ]
    poll, _ = load(data)
    lines = render_tally_lines(poll)
    assert "red, 1, 33.3%" in lines
    assert "blue, 2, 66.7%" in lines


def test_a_percentage_half_rounds_up_not_to_even():
    data = valid_data()
    data["answers"] = ["red", "blue"]
    data["responses"] = [response("juror-1", "red")] + [
        response(f"juror-{n}", "blue") for n in range(2, 17)
    ]
    poll, _ = load(data)
    lines = render_tally_lines(poll)
    assert "red, 1, 6.3%" in lines
    assert "blue, 15, 93.8%" in lines


def abstained_response(juror, answer, model="tiny-a", reason="a reason"):
    entry = response(juror, answer, model=model, reason=reason)
    entry["abstained"] = True
    return entry


def test_an_abstained_response_loads_clean_and_keeps_its_kind():
    data = valid_data()
    data["responses"] = [
        response("juror-1", "red"),
        abstained_response("juror-2", "blue"),
    ]
    poll, defects = load(data)
    assert defects == []
    assert "abstained" not in poll["responses"][0]
    assert poll["responses"][1]["abstained"] is True


def test_an_abstained_field_holding_false_is_named():
    data = valid_data()
    entry = response("juror-1", "red")
    entry["abstained"] = False
    data["responses"] = [entry]
    poll, defects = load(data)
    assert poll is None
    assert defects == ["response 0 field abstained is not true"]


def test_an_abstained_field_holding_a_string_is_named():
    data = valid_data()
    entry = response("juror-1", "red")
    entry["abstained"] = "true"
    data["responses"] = [response("juror-0", "blue"), entry]
    poll, defects = load(data)
    assert poll is None
    assert defects == ["response 1 field abstained is not true"]


def test_an_abstained_field_holding_a_number_is_named():
    data = valid_data()
    entry = response("juror-1", "red")
    entry["abstained"] = 1
    data["responses"] = [entry]
    poll, defects = load(data)
    assert poll is None
    assert defects == ["response 0 field abstained is not true"]


def test_an_abstained_response_with_an_empty_reason_is_still_named():
    data = valid_data()
    entry = abstained_response("juror-1", "red", reason="")
    data["responses"] = [entry]
    poll, defects = load(data)
    assert poll is None
    assert defects == ["response 0 field reason is empty"]


def test_a_duplicate_juror_is_named_when_one_response_abstained():
    data = valid_data()
    data["responses"] = [
        response("juror-1", "red"),
        abstained_response("juror-1", "blue"),
    ]
    poll, defects = load(data)
    assert poll is None
    assert defects == ["duplicate juror juror-1"]


def test_the_tally_counts_abstained_responses_apart():
    data = valid_data()
    data["responses"] = [
        response("juror-1", "red"),
        response("juror-2", "red"),
        response("juror-3", "blue"),
        response("juror-4", "purple"),
        abstained_response("juror-5", "red"),
        abstained_response("juror-6", "purple"),
    ]
    poll, defects = load(data)
    assert defects == []
    tally = tally_poll(poll)
    assert tally["responses"] == 6
    assert tally["valid"] == 3
    assert tally["spoiled"] == 1
    assert tally["abstained"] == 2
    rows = {row["answer"]: row for row in tally["rows"]}
    assert rows["red"]["count"] == 2
    assert rows["red"]["share"] == Fraction(2, 3)
    assert rows["blue"]["count"] == 1
    assert rows["blue"]["share"] == Fraction(1, 3)


def test_the_tally_lines_name_the_abstained_count_after_spoiled():
    data = valid_data()
    data["answers"] = ["red", "blue"]
    data["responses"] = [
        response("juror-1", "red"),
        response("juror-2", "purple"),
        abstained_response("juror-3", "blue"),
    ]
    poll, defects = load(data)
    assert defects == []
    assert render_tally_lines(poll) == [
        "question, Which door should the agent pick?",
        "responses, 3",
        "valid, 1",
        "spoiled, 1",
        "abstained, 1",
        "red, 1, 100.0%",
        "blue, 0, 0.0%",
    ]


def test_the_tally_lines_omit_the_abstained_line_when_none_abstained():
    data = valid_data()
    data["answers"] = ["red", "blue"]
    data["responses"] = [
        response("juror-1", "red"),
        response("juror-2", "purple"),
    ]
    poll, defects = load(data)
    assert defects == []
    assert render_tally_lines(poll) == [
        "question, Which door should the agent pick?",
        "responses, 2",
        "valid, 1",
        "spoiled, 1",
        "red, 1, 100.0%",
        "blue, 0, 0.0%",
    ]
