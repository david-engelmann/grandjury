"""Pins for the recorded example poll and its golden renderings.

The example is a synthetic poll of 124 recorded responses across
three offered answers and three models, one of them unpriced, with
four spoiled responses. The goldens are held byte identical
against the renderings the code produces today.
"""

import pathlib

from grandjury.polls import (
    cost_lines,
    load_poll_file,
    load_prices_file,
    render_stadium,
    tally_poll,
)

EXAMPLES = pathlib.Path(__file__).resolve().parent.parent / "examples"


def test_the_example_poll_loads_clean_at_its_stated_shape():
    poll, defects = load_poll_file(EXAMPLES / "recorded_poll.json")
    assert defects == []
    assert len(poll["answers"]) == 3
    assert len(poll["responses"]) == 124
    models = {response["model"] for response in poll["responses"]}
    assert models == {"tiny-a", "tiny-b", "tiny-c"}
    tally = tally_poll(poll)
    assert tally["valid"] == 120
    assert tally["spoiled"] == 4


def test_the_example_prices_load_clean():
    table, defects = load_prices_file(EXAMPLES / "prices.json")
    assert defects == []
    assert set(table) == {"tiny-a", "tiny-b"}


def test_the_stadium_golden_is_byte_identical():
    poll, defects = load_poll_file(EXAMPLES / "recorded_poll.json")
    assert defects == []
    golden = (EXAMPLES / "stadium-golden.txt").read_text(
        encoding="utf-8"
    )
    assert "\n".join(render_stadium(poll)) + "\n" == golden


def test_the_cost_golden_is_byte_identical():
    poll, defects = load_poll_file(EXAMPLES / "recorded_poll.json")
    assert defects == []
    table, price_defects = load_prices_file(EXAMPLES / "prices.json")
    assert price_defects == []
    golden = (EXAMPLES / "cost-golden.txt").read_text(encoding="utf-8")
    assert "\n".join(cost_lines(poll, table)) + "\n" == golden


def test_the_pilot_example_poll_loads_clean_at_its_stated_shape():
    poll, defects = load_poll_file(EXAMPLES / "pilot_poll.json")
    assert defects == []
    assert len(poll["answers"]) == 2
    assert len(poll["responses"]) == 10
    models = {response["model"] for response in poll["responses"]}
    assert models == {"jev-1.13.0", "openai/gpt-6-luna"}
    tally = tally_poll(poll)
    assert tally["valid"] == 5
    assert tally["spoiled"] == 0
    assert tally["abstained"] == 5


def test_the_pilot_prices_load_clean():
    table, defects = load_prices_file(EXAMPLES / "pilot-prices.json")
    assert defects == []
    assert set(table) == {"openai/gpt-6-luna"}


def test_the_pilot_stadium_golden_is_byte_identical():
    poll, defects = load_poll_file(EXAMPLES / "pilot_poll.json")
    assert defects == []
    golden = (EXAMPLES / "pilot-stadium-golden.txt").read_text(
        encoding="utf-8"
    )
    assert "\n".join(render_stadium(poll)) + "\n" == golden


def test_the_pilot_cost_golden_is_byte_identical():
    poll, defects = load_poll_file(EXAMPLES / "pilot_poll.json")
    assert defects == []
    table, price_defects = load_prices_file(
        EXAMPLES / "pilot-prices.json"
    )
    assert price_defects == []
    golden = (EXAMPLES / "pilot-cost-golden.txt").read_text(
        encoding="utf-8"
    )
    assert "\n".join(cost_lines(poll, table)) + "\n" == golden
