"""Pins for GJ-6, the pilot recording.

The twelve recorded polls load clean, the envelope inventory is
complete, the envelopes replay to the committed polls through
the seat readers, the goldens regenerate byte identical, and
the run record and the pilot price recompute from the recorded
envelopes. The incident section of the run record is stated
data, and it is pinned against its input file and its own
arithmetic. Every pin runs offline.
"""

import json
import pathlib
from decimal import Decimal, ROUND_HALF_UP

from grandjury.polls import (
    cost_lines,
    load_poll_file,
    load_prices_file,
    render_stadium,
)
from grandjury.seats import (
    response_from_chat_envelope,
    response_from_jev_envelope,
)

EXAMPLES = pathlib.Path(__file__).resolve().parent.parent / "examples"
PILOT = EXAMPLES / "pilot"
ROUTES = (
    ("jev", response_from_jev_envelope),
    ("openai", response_from_chat_envelope),
)


def read_json(path):
    return json.loads(path.read_text(encoding="utf-8"))


def envelope_names():
    return {
        f"q{index:02d}-{prefix}-{seat}.json"
        for index in range(1, 13)
        for prefix in ("jev", "openai")
        for seat in range(1, 6)
    }


def rebuild_poll(index, item):
    responses = []
    for prefix, reader in ROUTES:
        for seat in range(1, 6):
            envelope = read_json(
                PILOT / "envelopes"
                / f"q{index:02d}-{prefix}-{seat}.json"
            )
            response = reader(
                f"{prefix}-{seat}", envelope, item["answers"]
            )
            if response is not None:
                responses.append(response)
    return {
        "question": item["question"],
        "answers": item["answers"],
        "responses": responses,
    }


def test_the_pilot_question_set_holds_twelve_questions():
    questions = read_json(PILOT / "questions.json")
    assert len(questions) == 12
    for item in questions:
        assert set(item) == {"question", "answers"}
        assert isinstance(item["question"], str)
        assert item["question"] != ""
        assert 2 <= len(item["answers"]) <= 4
        for answer in item["answers"]:
            assert isinstance(answer, str)
            assert answer != ""


def test_the_envelope_inventory_is_complete():
    on_disk = {path.name for path in (PILOT / "envelopes").glob("*.json")}
    assert on_disk == envelope_names()


def test_every_pilot_poll_loads_clean_with_ten_responses():
    jurors = {f"jev-{seat}" for seat in range(1, 6)} | {
        f"openai-{seat}" for seat in range(1, 6)
    }
    for index in range(1, 13):
        poll, defects = load_poll_file(PILOT / f"poll-{index:02d}.json")
        assert defects == []
        assert len(poll["responses"]) == 10
        assert {r["juror"] for r in poll["responses"]} == jurors


def test_the_recorded_envelopes_replay_to_the_committed_polls():
    questions = read_json(PILOT / "questions.json")
    for index, item in enumerate(questions, start=1):
        committed = read_json(PILOT / f"poll-{index:02d}.json")
        assert rebuild_poll(index, item) == committed


def test_the_example_poll_is_the_first_pilot_poll():
    assert read_json(EXAMPLES / "pilot_poll.json") == read_json(
        PILOT / "poll-01.json"
    )


def test_the_pilot_goldens_regenerate_byte_identical():
    poll, defects = load_poll_file(EXAMPLES / "pilot_poll.json")
    assert defects == []
    stadium = (EXAMPLES / "pilot-stadium-golden.txt").read_text(
        encoding="utf-8"
    )
    assert "\n".join(render_stadium(poll)) + "\n" == stadium
    table, price_defects = load_prices_file(
        EXAMPLES / "pilot-prices.json"
    )
    assert price_defects == []
    cost = (EXAMPLES / "pilot-cost-golden.txt").read_text(
        encoding="utf-8"
    )
    assert "\n".join(cost_lines(poll, table)) + "\n" == cost


def recompute_model_record(prefix, reader, questions):
    recorded = 0
    abstained = 0
    failed = 0
    input_tokens = 0
    output_tokens = 0
    cost = Decimal(0)
    for index, item in enumerate(questions, start=1):
        for seat in range(1, 6):
            path = (
                PILOT / "envelopes"
                / f"q{index:02d}-{prefix}-{seat}.json"
            )
            envelope = read_json(path)
            exact = json.loads(
                path.read_text(encoding="utf-8"), parse_float=Decimal
            )
            usage = envelope["usage"]
            if prefix == "jev":
                input_tokens += usage["input_tokens"]
                output_tokens += usage["output_tokens"]
            else:
                input_tokens += usage["prompt_tokens"]
                output_tokens += usage["completion_tokens"]
                cost += exact["usage"]["cost"]
            response = reader(
                f"{prefix}-{seat}", envelope, item["answers"]
            )
            if response is None:
                failed += 1
            else:
                recorded += 1
                if response.get("abstained") is True:
                    abstained += 1
    return {
        "recorded": recorded,
        "abstained": abstained,
        "failed": failed,
        "input_tokens": input_tokens,
        "output_tokens": output_tokens,
        "cost": cost,
    }


def test_the_run_record_agrees_with_the_recorded_envelopes():
    questions = read_json(PILOT / "questions.json")
    record = read_json(PILOT / "run_record.json")
    assert record["questions"] == 12
    assert record["seats_per_question_per_model"] == 5
    assert set(record["models"]) == {"jev-1.13.0", "openai/gpt-6-luna"}
    for prefix, reader, model, route in (
        ("jev", response_from_jev_envelope, "jev-1.13.0", "jev"),
        (
            "openai",
            response_from_chat_envelope,
            "openai/gpt-6-luna",
            "chat",
        ),
    ):
        entry = record["models"][model]
        got = recompute_model_record(prefix, reader, questions)
        assert entry["route"] == route
        assert entry["calls_attempted"] == 60
        assert entry["calls_recorded"] == got["recorded"]
        assert entry["failed_calls"] == got["failed"]
        assert entry["abstained"] == got["abstained"]
        assert entry["input_tokens"] == got["input_tokens"]
        assert entry["output_tokens"] == got["output_tokens"]
    chat_entry = record["models"]["openai/gpt-6-luna"]
    chat_got = recompute_model_record(
        "openai", response_from_chat_envelope, questions
    )
    assert chat_entry["cost_dollars"] == str(chat_got["cost"])
    assert "cost_dollars" not in record["models"]["jev-1.13.0"]


def test_the_run_record_incident_section_matches_its_input_file():
    record = read_json(PILOT / "run_record.json")
    assert record["incident"] == read_json(PILOT / "incident.json")


def test_the_incident_section_carries_the_named_counts():
    record = read_json(PILOT / "run_record.json")
    incident = record["incident"]
    jev = incident["jev-1.13.0"]
    chat = incident["openai/gpt-6-luna"]
    assert jev["attempts_that_never_left_the_machine"] == 40
    assert jev["envelopes_lost_and_seats_rerun"] == 10
    assert jev["calls_discarded"] == 0
    assert chat["attempts_that_never_left_the_machine"] == 0
    assert chat["envelopes_lost_and_seats_rerun"] == 5
    assert chat["calls_discarded"] == 1
    for model, entry in (("jev-1.13.0", jev), ("openai/gpt-6-luna", chat)):
        parts = [
            value
            for name, value in entry.items()
            if name != "attempts_in_round"
        ]
        assert sum(parts) == entry["attempts_in_round"]
        assert entry["calls_in_the_recorded_run"] == (
            record["models"][model]["calls_attempted"]
        )
    total = jev["attempts_in_round"] + chat["attempts_in_round"]
    assert total == 180
    assert incident["lost_token_usage"] == (
        "unrecoverable and estimated nowhere"
    )


def test_the_pilot_price_recomputes_from_the_recorded_envelopes():
    questions = read_json(PILOT / "questions.json")
    total = Decimal(0)
    count = 0
    for index, item in enumerate(questions, start=1):
        for seat in range(1, 6):
            path = (
                PILOT / "envelopes" / f"q{index:02d}-openai-{seat}.json"
            )
            envelope = read_json(path)
            response = response_from_chat_envelope(
                f"openai-{seat}", envelope, item["answers"]
            )
            if response is not None:
                exact = json.loads(
                    path.read_text(encoding="utf-8"),
                    parse_float=Decimal,
                )
                total += exact["usage"]["cost"]
                count += 1
    price = (total / count).quantize(
        Decimal("0.000001"), rounding=ROUND_HALF_UP
    )
    table, defects = load_prices_file(EXAMPLES / "pilot-prices.json")
    assert defects == []
    assert table == {"openai/gpt-6-luna": price}
    assert "jev-1.13.0" not in table
