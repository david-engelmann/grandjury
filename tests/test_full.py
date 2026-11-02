"""Pins for GJ-7, the full poll, and its seat discipline.

The forty full polls were recorded live in Round 3 over the
two decision model routes, five seats per question per model.
Everything here replays offline against the recorded
envelopes. A pin fails under the negation of the rule it
names. The pins hold the question set against the pilot set,
the envelope inventory against the run record, the polls
against a rebuild from the envelopes, the goldens against
the real commands, the run record against a recomputation
from the fixtures with a missing envelope file counted as
attempted and failed, the price against its recomputation
from the recorded envelopes, and the tally of a stated
sample against an independent recount written in this file.
"""

import importlib.util
import json
import pathlib
from decimal import ROUND_HALF_UP, Decimal

from grandjury.polls import (
    cost_lines,
    load_poll_file,
    load_prices_text,
    render_stadium,
    tally_poll,
)
from grandjury.seats import (
    response_from_chat_envelope,
    response_from_jev_envelope,
)

EXAMPLES = pathlib.Path(__file__).resolve().parent.parent / "examples"
FULL = EXAMPLES / "full"
SEATS = 5
SAMPLE = (1, 13, 27, 40)
READERS = {
    "jev": response_from_jev_envelope,
    "openai": response_from_chat_envelope,
}


def load_module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def questions():
    return json.loads((FULL / "questions.json").read_text())


def envelopes(index, prefix):
    found = {}
    for seat in range(1, SEATS + 1):
        path = FULL / "envelopes" / f"q{index:02d}-{prefix}-{seat}.json"
        if path.exists():
            found[seat] = json.loads(path.read_text())
    return found


def recorded_chat_costs():
    """The usage cost of each recorded chat response."""
    costs = []
    for index, item in enumerate(questions(), start=1):
        for seat, envelope in envelopes(index, "openai").items():
            response = response_from_chat_envelope(
                f"openai-{seat}", envelope, item["answers"]
            )
            if response is not None:
                exact = json.loads(
                    (
                        FULL
                        / "envelopes"
                        / f"q{index:02d}-openai-{seat}.json"
                    ).read_text(),
                    parse_float=Decimal,
                )
                costs.append(exact["usage"]["cost"])
    return costs


def test_the_full_question_set_stands_in_shape():
    items = questions()
    assert len(items) == 40
    for item in items:
        assert set(item) == {"question", "answers"}
        assert isinstance(item["question"], str)
        assert item["question"]
        assert 2 <= len(item["answers"]) <= 4
        assert len(set(item["answers"])) == len(item["answers"])
        for answer in item["answers"]:
            assert isinstance(answer, str)
            assert answer
    texts = [item["question"] for item in items]
    assert len(set(texts)) == 40
    counts = sorted(len(item["answers"]) for item in items)
    assert counts[0] == 2
    assert counts[-1] == 4


def test_the_full_questions_are_disjoint_from_the_pilot():
    pilot = json.loads(
        (EXAMPLES / "pilot" / "questions.json").read_text()
    )
    pilot_texts = {item["question"] for item in pilot}
    assert len(pilot_texts) == 12
    for item in questions():
        assert item["question"] not in pilot_texts


def test_the_envelope_inventory_matches_the_run_record():
    record = json.loads((FULL / "run_record.json").read_text())
    names = {
        path.name
        for path in (FULL / "envelopes").glob("*.json")
    }
    expected = {
        f"q{index:02d}-{prefix}-{seat}.json"
        for index in range(1, 41)
        for prefix in READERS
        for seat in range(1, SEATS + 1)
    }
    assert names <= expected
    recorded = sum(
        entry["calls_recorded"]
        for entry in record["models"].values()
    )
    failed = sum(
        entry["failed_calls"] for entry in record["models"].values()
    )
    assert recorded + failed == 400
    # Every envelope file holds exactly one call, and the model
    # string inside each file is the model its seat names.
    for name in names:
        envelope = json.loads((FULL / "envelopes" / name).read_text())
        if "-jev-" in name:
            assert envelope["model"] == "jev-1.13.0"
        else:
            assert envelope["model"] == "openai/gpt-6-luna"
    assert recorded <= len(names) <= 400


def test_every_full_poll_rebuilds_from_its_envelopes():
    make_full = load_module("make_full", EXAMPLES / "make_full.py")
    for index, item in enumerate(questions(), start=1):
        committed = json.loads(
            (FULL / f"poll-{index:02d}.json").read_text()
        )
        assert make_full.build_poll(index, item) == committed


def test_the_full_poll_files_load_clean_and_tally():
    for index in range(1, 41):
        poll, defects = load_poll_file(
            str(FULL / f"poll-{index:02d}.json")
        )
        assert defects == []
        tally_poll(poll)


def test_the_sample_tallies_match_an_independent_recount():
    for index in SAMPLE:
        poll, defects = load_poll_file(
            str(FULL / f"poll-{index:02d}.json")
        )
        assert defects == []
        tally = tally_poll(poll)
        responses = poll["responses"]
        valid = [
            entry for entry in responses
            if entry.get("abstained") is not True
            and entry["answer"] in poll["answers"]
        ]
        assert tally["responses"] == len(responses)
        assert tally["valid"] == len(valid)
        assert tally["abstained"] == sum(
            1 for entry in responses
            if entry.get("abstained") is True
        )
        assert tally["spoiled"] == sum(
            1
            for entry in responses
            if entry.get("abstained") is not True
            and entry["answer"] not in poll["answers"]
        )
        rows = {row["answer"]: row["count"] for row in tally["rows"]}
        for answer in poll["answers"]:
            count = sum(
                1 for entry in valid if entry["answer"] == answer
            )
            assert rows[answer] == count


def test_the_full_example_poll_is_the_first_poll():
    example = json.loads((EXAMPLES / "full_poll.json").read_text())
    first = json.loads((FULL / "poll-01.json").read_text())
    assert example == first


def test_the_full_goldens_match_the_renderings():
    example = json.loads((EXAMPLES / "full_poll.json").read_text())
    prices_text = (EXAMPLES / "full-prices.json").read_text()
    table, defects = load_prices_text(prices_text)
    assert defects == []
    stadium = (EXAMPLES / "full-stadium-golden.txt").read_text()
    assert "\n".join(render_stadium(example)) + "\n" == stadium
    cost = (EXAMPLES / "full-cost-golden.txt").read_text()
    assert "\n".join(cost_lines(example, table)) + "\n" == cost


def test_the_full_price_is_the_measured_mean_chat_cost():
    costs = recorded_chat_costs()
    assert costs
    mean = (sum(costs) / len(costs)).quantize(
        Decimal("0.000001"), rounding=ROUND_HALF_UP
    )
    table = json.loads((EXAMPLES / "full-prices.json").read_text())
    assert table == {"prices": {"openai/gpt-6-luna": str(mean)}}


def test_the_full_run_record_recomputes_from_the_fixtures():
    make_full = load_module(
        "make_full_record", EXAMPLES / "make_full.py"
    )
    record = json.loads((FULL / "run_record.json").read_text())
    assert make_full.build_run_record(questions()) == record
    assert record["questions"] == 40
    assert record["seats_per_question_per_model"] == 5


def test_the_full_run_record_counts_agree_with_the_envelopes():
    record = json.loads((FULL / "run_record.json").read_text())
    for prefix, entry in (
        ("jev", record["models"]["jev-1.13.0"]),
        ("openai", record["models"]["openai/gpt-6-luna"]),
    ):
        files = 0
        recorded = 0
        abstained = 0
        input_tokens = 0
        output_tokens = 0
        cost = Decimal(0)
        for index, item in enumerate(questions(), start=1):
            for seat in range(1, SEATS + 1):
                path = (
                    FULL
                    / "envelopes"
                    / f"q{index:02d}-{prefix}-{seat}.json"
                )
                if not path.exists():
                    continue
                files += 1
                envelope = json.loads(path.read_text())
                exact = json.loads(
                    path.read_text(), parse_float=Decimal
                )
                usage = envelope["usage"]
                if prefix == "jev":
                    input_tokens += usage["input_tokens"]
                    output_tokens += usage["output_tokens"]
                else:
                    input_tokens += usage["prompt_tokens"]
                    output_tokens += usage["completion_tokens"]
                    cost += exact["usage"]["cost"]
                response = READERS[prefix](
                    f"{prefix}-{seat}", envelope, item["answers"]
                )
                if response is not None:
                    recorded += 1
                    if response.get("abstained") is True:
                        abstained += 1
        assert entry["route"] == (
            "jev" if prefix == "jev" else "chat"
        )
        assert entry["calls_attempted"] == 200
        assert entry["calls_recorded"] == recorded
        # A failed call is a seat with no envelope file or a
        # seat whose envelope the reader set aside, and the two
        # kinds together are every seat that recorded nothing.
        assert entry["failed_calls"] == 200 - recorded
        assert entry["failed_calls"] == (200 - files) + (
            files - recorded
        )
        assert entry["abstained"] == abstained
        assert entry["input_tokens"] == input_tokens
        assert entry["output_tokens"] == output_tokens
        if prefix == "openai":
            assert entry["cost_dollars"] == str(cost)


def test_the_incident_section_equals_its_input_file():
    record = json.loads((FULL / "run_record.json").read_text())
    incident = json.loads((FULL / "incident.json").read_text())
    assert record["incident"] == incident


def test_the_incident_arithmetic_holds():
    incident = json.loads((FULL / "incident.json").read_text())
    record = json.loads((FULL / "run_record.json").read_text())
    for model, entry in incident.items():
        if model == "lost_token_usage":
            continue
        parts = (
            entry["calls_in_the_recorded_run"]
            + entry["attempts_that_never_left_the_machine"]
            + entry["envelopes_lost_and_seats_rerun"]
            + entry["calls_discarded"]
            + entry["route_verification_calls"]
            + entry["diagnostic_calls"]
        )
        assert parts == entry["attempts_in_round"]
        assert (
            entry["calls_in_the_recorded_run"]
            == record["models"][model]["calls_attempted"]
        )
    total = sum(
        entry["attempts_in_round"]
        for model, entry in incident.items()
        if model != "lost_token_usage"
    )
    assert total <= 1000
