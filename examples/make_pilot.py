"""Rebuild the pilot polls and their golden renderings.

The twelve polls under examples/pilot were recorded live in
Round 2, one call per juror seat, and the raw envelopes stand
beside them under examples/pilot/envelopes. This script rebuilds
the twelve polls, the round example poll at
examples/pilot_poll.json, the two goldens, and the run record
from the questions file and the recorded envelopes through the
seat readers in grandjury.seats, entirely offline. The incident
section of the run record is not derivable from the envelopes,
so it is carried in unchanged from the stated input file
examples/pilot/incident.json. Running it against the recorded
fixtures reproduces the committed files byte for byte.

Run from the repo root with the package importable, for example
PYTHONPATH=src python3 examples/make_pilot.py
"""

import json
import pathlib
import sys
from decimal import Decimal

sys.path.insert(
    0, str(pathlib.Path(__file__).resolve().parent.parent / "src")
)

from grandjury.polls import cost_lines, load_prices_text, render_stadium
from grandjury.seats import (
    response_from_chat_envelope,
    response_from_jev_envelope,
)

HERE = pathlib.Path(__file__).resolve().parent
PILOT = HERE / "pilot"
SEATS = 5


def build_poll(index, item):
    """Build one recorded poll from its recorded envelopes.

    The seats stand in order, the Jev seats first and then the
    chat seats. A seat whose call failed records nothing and
    leaves no response, exactly as GJ-5 and GJ-6 state.
    """
    question = item["question"]
    answers = item["answers"]
    responses = []
    routes = (
        ("jev", response_from_jev_envelope),
        ("openai", response_from_chat_envelope),
    )
    for prefix, reader in routes:
        for seat in range(1, SEATS + 1):
            name = f"q{index:02d}-{prefix}-{seat}.json"
            envelope = json.loads(
                (PILOT / "envelopes" / name).read_text(encoding="utf-8")
            )
            response = reader(f"{prefix}-{seat}", envelope, answers)
            if response is not None:
                responses.append(response)
    return {
        "question": question,
        "answers": answers,
        "responses": responses,
    }


def build_run_record(questions):
    """Rebuild the run record from the recorded envelopes.

    The counts and the token totals cover the recorded run, the
    envelopes standing under examples/pilot/envelopes, as
    FIX-GJ-4 in CONTRACT.md states. The dollar cost of the chat
    route is summed from the usage figure of each envelope as an
    exact decimal. The incident section is carried in unchanged
    from its stated input file.
    """
    models = {}
    routes = (
        ("jev", "jev", response_from_jev_envelope),
        ("openai", "chat", response_from_chat_envelope),
    )
    for prefix, route, reader in routes:
        attempted = 0
        recorded = 0
        failed = 0
        abstained = 0
        input_tokens = 0
        output_tokens = 0
        cost = Decimal(0)
        model_name = None
        for index, item in enumerate(questions, start=1):
            for seat in range(1, SEATS + 1):
                path = (
                    PILOT / "envelopes"
                    / f"q{index:02d}-{prefix}-{seat}.json"
                )
                text = path.read_text(encoding="utf-8")
                envelope = json.loads(text)
                exact = json.loads(text, parse_float=Decimal)
                model_name = envelope["model"]
                attempted += 1
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
        entry = {
            "route": route,
            "calls_attempted": attempted,
            "calls_recorded": recorded,
            "failed_calls": failed,
            "abstained": abstained,
            "input_tokens": input_tokens,
            "output_tokens": output_tokens,
        }
        if prefix == "openai":
            entry["cost_dollars"] = str(cost)
        models[model_name] = entry
    incident = json.loads(
        (PILOT / "incident.json").read_text(encoding="utf-8")
    )
    return {
        "questions": len(questions),
        "seats_per_question_per_model": SEATS,
        "models": models,
        "incident": incident,
    }


def main():
    questions = json.loads(
        (PILOT / "questions.json").read_text(encoding="utf-8")
    )
    polls = [
        build_poll(index, item)
        for index, item in enumerate(questions, start=1)
    ]
    for index, poll in enumerate(polls, start=1):
        (PILOT / f"poll-{index:02d}.json").write_text(
            json.dumps(poll, indent=2, ensure_ascii=False) + "\n",
            encoding="utf-8",
        )
    example = polls[0]
    (HERE / "pilot_poll.json").write_text(
        json.dumps(example, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )
    prices_text = (HERE / "pilot-prices.json").read_text(
        encoding="utf-8"
    )
    table, defects = load_prices_text(prices_text)
    assert defects == []
    (HERE / "pilot-stadium-golden.txt").write_text(
        "\n".join(render_stadium(example)) + "\n", encoding="utf-8"
    )
    (HERE / "pilot-cost-golden.txt").write_text(
        "\n".join(cost_lines(example, table)) + "\n", encoding="utf-8"
    )
    record = build_run_record(questions)
    (PILOT / "run_record.json").write_text(
        json.dumps(record, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )
    recorded = sum(len(poll["responses"]) for poll in polls)
    print(f"polls {len(polls)} responses {recorded}")


if __name__ == "__main__":
    main()
