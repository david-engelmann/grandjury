"""Pins for GJ-4, the price table loader and the cost counter.

The counter arithmetic is exact decimal work, so the pins carry
prices whose binary float forms drift, and the unpriced model
report is pinned line by line.
"""

import json
from decimal import Decimal

from grandjury.polls import (
    cost_lines,
    load_poll_text,
    load_prices_file,
    load_prices_text,
)


def response(juror, answer, model="tiny-a", reason="a reason"):
    return {"juror": juror, "model": model, "answer": answer,
            "reason": reason}


def load_poll(data):
    poll, defects = load_poll_text(json.dumps(data))
    assert defects == []
    return poll


def load_prices(data):
    table, defects = load_prices_text(json.dumps(data))
    assert defects == []
    return table


def test_a_valid_price_table_loads_as_exact_decimals():
    table = load_prices({"prices": {"tiny-a": "0.0002",
                                    "tiny-b": "0.1"}})
    assert table == {"tiny-a": Decimal("0.0002"),
                     "tiny-b": Decimal("0.1")}


def test_price_text_that_is_not_json_draws_the_named_defect():
    table, defects = load_prices_text("nope")
    assert table is None
    assert defects == ["input is not valid JSON"]


def test_a_price_table_that_is_not_an_object_draws_the_named_defect():
    table, defects = load_prices_text("[1]")
    assert defects == ["input is not a JSON object"]


def test_an_unreadable_price_file_draws_the_named_defect(tmp_path):
    table, defects = load_prices_file(tmp_path / "missing.json")
    assert table is None
    assert defects == ["file cannot be read"]


def test_a_missing_prices_field_is_named():
    table, defects = load_prices_text("{}")
    assert defects == ["missing field prices"]


def test_a_prices_value_of_the_wrong_type_is_named():
    table, defects = load_prices_text('{"prices": [1]}')
    assert defects == ["field prices has the wrong type"]


def test_a_price_that_is_not_a_string_is_named():
    table, defects = load_prices_text('{"prices": {"tiny-a": 0.0002}}')
    assert defects == ["price for tiny-a has the wrong type"]


def test_a_price_that_is_not_a_decimal_string_is_named():
    table, defects = load_prices_text('{"prices": {"tiny-a": "cheap"}}')
    assert defects == ["price for tiny-a is not a decimal string"]


def test_a_nan_price_is_not_a_decimal_string():
    table, defects = load_prices_text('{"prices": {"tiny-a": "NaN"}}')
    assert defects == ["price for tiny-a is not a decimal string"]


def test_an_infinite_price_is_not_a_decimal_string():
    table, defects = load_prices_text(
        '{"prices": {"tiny-a": "Infinity"}}'
    )
    assert defects == ["price for tiny-a is not a decimal string"]


def test_a_negative_price_is_named():
    table, defects = load_prices_text('{"prices": {"tiny-a": "-0.5"}}')
    assert defects == ["price for tiny-a is negative"]


def test_a_price_holding_an_unpaired_surrogate_is_named():
    table, defects = load_prices_text(
        '{"prices": {"tiny-a": "\\ud800x"}}'
    )
    assert defects == [
        "price for tiny-a holds an unpaired surrogate"
    ]


def test_a_model_label_holding_an_unpaired_surrogate_is_named():
    table, defects = load_prices_text(
        '{"prices": {"\\ud800x": "0.1"}}'
    )
    assert defects == ["a model label holds an unpaired surrogate"]


FORGED_LABEL = "evil\nunpriced model, forged, 7 responses"


def prices_with_label(label, value):
    return json.dumps({"prices": {label: value}})


def test_a_model_label_holding_a_newline_is_named_not_printed():
    table, defects = load_prices_text(
        prices_with_label(FORGED_LABEL, 7)
    )
    assert table is None
    assert defects == ["a model label holds a newline"]


def test_a_newline_label_is_named_before_a_bad_price_string():
    table, defects = load_prices_text(
        prices_with_label(FORGED_LABEL, "cheap")
    )
    assert table is None
    assert defects == ["a model label holds a newline"]


def test_a_newline_label_is_named_before_a_negative_price():
    table, defects = load_prices_text(
        prices_with_label(FORGED_LABEL, "-0.5")
    )
    assert table is None
    assert defects == ["a model label holds a newline"]


def test_a_newline_label_is_named_before_a_surrogate_price():
    table, defects = load_prices_text(
        prices_with_label(FORGED_LABEL, "\ud800x")
    )
    assert table is None
    assert defects == ["a model label holds a newline"]


def test_a_newline_label_with_a_valid_price_loads_no_table():
    table, defects = load_prices_text(
        prices_with_label(FORGED_LABEL, "0.1")
    )
    assert table is None
    assert defects == ["a model label holds a newline"]


def test_a_carriage_return_label_with_a_valid_price_is_named():
    label = "evil\runpriced model, forged, 7 responses"
    table, defects = load_prices_text(
        prices_with_label(label, "0.1")
    )
    assert table is None
    assert defects == ["a model label holds a newline"]


def test_the_cost_of_a_fully_priced_poll():
    poll = load_poll({
        "question": "q",
        "answers": ["red", "blue"],
        "responses": [
            response("juror-1", "red"),
            response("juror-2", "blue"),
            response("juror-3", "red"),
        ],
    })
    table = load_prices({"prices": {"tiny-a": "0.0002"}})
    assert cost_lines(poll, table) == [
        "question, q",
        "responses, 3",
        "priced, 3 responses, 0.000600 dollars",
        "unpriced, 0 responses",
    ]


def test_the_counter_carries_no_float_drift():
    poll = load_poll({
        "question": "q",
        "answers": ["red", "blue"],
        "responses": [response(f"juror-{n}", "red")
                      for n in range(1, 4)],
    })
    table = load_prices({"prices": {"tiny-a": "0.1"}})
    lines = cost_lines(poll, table)
    assert "priced, 3 responses, 0.300000 dollars" in lines


def test_spoiled_responses_are_priced_too():
    poll = load_poll({
        "question": "q",
        "answers": ["red", "blue"],
        "responses": [
            response("juror-1", "red"),
            response("juror-2", "purple"),
        ],
    })
    table = load_prices({"prices": {"tiny-a": "0.0002"}})
    lines = cost_lines(poll, table)
    assert "priced, 2 responses, 0.000400 dollars" in lines


def test_unpriced_models_are_named_with_their_counts():
    poll = load_poll({
        "question": "q",
        "answers": ["red", "blue"],
        "responses": [
            response("juror-1", "red", model="tiny-a"),
            response("juror-2", "blue", model="tiny-b"),
            response("juror-3", "red", model="tiny-c"),
            response("juror-4", "blue", model="tiny-b"),
        ],
    })
    table = load_prices({"prices": {"tiny-a": "0.0002"}})
    assert cost_lines(poll, table) == [
        "question, q",
        "responses, 4",
        "priced, 1 responses, 0.000200 dollars",
        "unpriced, 3 responses",
        "unpriced model, tiny-b, 2 responses",
        "unpriced model, tiny-c, 1 responses",
    ]


def test_an_empty_price_table_leaves_everything_unpriced():
    poll = load_poll({
        "question": "q",
        "answers": ["red", "blue"],
        "responses": [response("juror-1", "red")],
    })
    table = load_prices({"prices": {}})
    lines = cost_lines(poll, table)
    assert "priced, 0 responses, 0.000000 dollars" in lines
    assert "unpriced, 1 responses" in lines
    assert "unpriced model, tiny-a, 1 responses" in lines


def test_a_total_rounds_to_six_places_with_halves_up():
    poll = load_poll({
        "question": "q",
        "answers": ["red", "blue"],
        "responses": [response("juror-1", "red")],
    })
    table = load_prices({"prices": {"tiny-a": "0.0000025"}})
    lines = cost_lines(poll, table)
    assert "priced, 1 responses, 0.000003 dollars" in lines


def test_a_price_padded_with_whitespace_is_not_a_decimal_string():
    table, defects = load_prices_text(
        '{"prices": {"tiny-a": " 0.1 "}}'
    )
    assert table is None
    assert defects == ["price for tiny-a is not a decimal string"]


def test_a_price_holding_an_underscore_is_not_a_decimal_string():
    table, defects = load_prices_text('{"prices": {"tiny-a": "1_0"}}')
    assert table is None
    assert defects == ["price for tiny-a is not a decimal string"]


def test_an_empty_price_is_not_a_decimal_string():
    table, defects = load_prices_text('{"prices": {"tiny-a": ""}}')
    assert table is None
    assert defects == ["price for tiny-a is not a decimal string"]


def test_a_price_with_a_plus_sign_head_is_not_a_decimal_string():
    table, defects = load_prices_text(
        '{"prices": {"tiny-a": "+0.1"}}'
    )
    assert table is None
    assert defects == ["price for tiny-a is not a decimal string"]


def test_unnamed_fields_of_the_price_table_are_ignored():
    table, defects = load_prices_text(
        '{"prices": {"tiny-a": "0.1"}, "currency": "dollars",'
        ' "version": 3}'
    )
    assert defects == []
    assert table == {"tiny-a": Decimal("0.1")}


def priced_line(price, count=1):
    poll = load_poll({
        "question": "q",
        "answers": ["red", "blue"],
        "responses": [response(f"juror-{n}", "red")
                      for n in range(1, count + 1)],
    })
    table = load_prices({"prices": {"tiny-a": price}})
    lines = cost_lines(poll, table)
    return lines[2]


def test_a_price_at_1e30_renders_its_total_in_full():
    line = priced_line("1e30")
    assert line == (
        "priced, 1 responses, " + "1" + "0" * 30 + ".000000 dollars"
    )


def test_a_price_at_1e22_renders_its_total_in_full():
    line = priced_line("1e22")
    assert line == (
        "priced, 1 responses, " + "1" + "0" * 22 + ".000000 dollars"
    )


def test_a_price_with_23_integer_digits_renders_in_full():
    line = priced_line("99999999999999999999999.99")
    assert line == (
        "priced, 1 responses, 99999999999999999999999.990000 dollars"
    )


def test_an_accumulation_of_large_prices_renders_in_full():
    line = priced_line("4e21", count=3)
    assert line == (
        "priced, 3 responses, " + "12" + "0" * 21 + ".000000 dollars"
    )


def test_an_exact_half_past_the_28th_digit_rounds_up():
    line = priced_line("1000000000000000000000.0000005")
    assert line == (
        "priced, 1 responses, 1000000000000000000000.000001 dollars"
    )


def abstained_response(juror, answer, model="tiny-a", reason="a reason"):
    entry = response(juror, answer, model=model, reason=reason)
    entry["abstained"] = True
    return entry


def test_an_abstained_response_is_priced_like_any_other_response():
    poll = load_poll({
        "question": "q",
        "answers": ["red", "blue"],
        "responses": [
            response("juror-1", "red"),
            abstained_response("juror-2", "blue"),
        ],
    })
    table = load_prices({"prices": {"tiny-a": "0.0002"}})
    assert cost_lines(poll, table) == [
        "question, q",
        "responses, 2",
        "priced, 2 responses, 0.000400 dollars",
        "unpriced, 0 responses",
    ]


def test_an_abstained_response_from_an_unpriced_model_is_named():
    poll = load_poll({
        "question": "q",
        "answers": ["red", "blue"],
        "responses": [
            response("juror-1", "red"),
            abstained_response("juror-2", "blue", model="tiny-c"),
            abstained_response("juror-3", "red", model="tiny-c"),
        ],
    })
    table = load_prices({"prices": {"tiny-a": "0.0002"}})
    assert cost_lines(poll, table) == [
        "question, q",
        "responses, 3",
        "priced, 1 responses, 0.000200 dollars",
        "unpriced, 2 responses",
        "unpriced model, tiny-c, 2 responses",
    ]
