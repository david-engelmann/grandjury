"""Rebuild the full poll files and their golden renderings.

The forty polls under examples/full were recorded live in
Round 3, one call per juror seat, and the raw envelopes stand
beside them under examples/full/envelopes. This script rebuilds
the forty polls, the round example poll at
examples/full_poll.json, the stadium and cost goldens, the
agreement golden, and the run record from the questions file
and the recorded envelopes through the seat readers, entirely
offline. A seat whose envelope file is absent counts as a call
attempted and failed and leaves no response, exactly as GJ-7
states. The incident section of the run record is not
derivable from the envelopes, so it is carried in unchanged
from the stated input file examples/full/incident.json. Running
it against the recorded fixtures reproduces the committed files
byte for byte.

Run from the repo root with the package importable, for example
PYTHONPATH=src python3 examples/make_full.py
"""

import contextlib
import io
import json
import pathlib
import sys
from decimal import Decimal

sys.path.insert(
    0, str(pathlib.Path(__file__).resolve().parent.parent / "src")
)

from grandjury.cli import main as cli_main
from grandjury.polls import cost_lines, load_prices_text, render_stadium
from grandjury.seats import (
    response_from_chat_envelope,
    response_from_jev_envelope,
)

HERE = pathlib.Path(__file__).resolve().parent
FULL = HERE / "full"
SEATS = 5
ROUTES = (
    ("jev", response_from_jev_envelope),
    ("openai", response_from_chat_envelope),
)


def read_envelope(index, prefix, seat):
    """Return one recorded envelope, or None when no file stands.

    A call that returned no envelope leaves no file, and the
    seat counts as attempted and failed, so the absence is data
    and not an error here.
    """
    path = FULL / "envelopes" / f"q{index:02d}-{prefix}-{seat}.json"
    if not path.exists():
        return None
    return path.read_text(encoding="utf-8")


def build_poll(index, item):
    """Build one recorded poll from its recorded envelopes.

    The seats stand in order, the Jev seats first and then the
    chat seats. A seat whose call failed records nothing and
    leaves no response, exactly as GJ-5 and GJ-7 state.
    """
    responses = []
    for prefix, reader in ROUTES:
        for seat in range(1, SEATS + 1):
            text = read_envelope(index, prefix, seat)
            if text is None:
                continue
            response = reader(
                f"{prefix}-{seat}", json.loads(text), item["answers"]
            )
            if response is not None:
                responses.append(response)
    return {
        "question": item["question"],
        "answers": item["answers"],
        "responses": responses,
    }


def build_run_record(questions):
    """Rebuild the run record from the recorded envelopes.

    The counts and the token totals cover the recorded run, the
    envelopes standing under examples/full/envelopes, with a
    seat whose envelope file is absent counted as a call
    attempted and failed, as GJ-7 states. The dollar cost of
    the chat route is summed from the usage figure of each
    envelope as an exact decimal. The incident section is
    carried in unchanged from its stated input file.
    """
    models = {}
    for prefix, reader in ROUTES:
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
                attempted += 1
                text = read_envelope(index, prefix, seat)
                if text is None:
                    failed += 1
                    continue
                envelope = json.loads(text)
                exact = json.loads(text, parse_float=Decimal)
                model_name = envelope["model"]
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
            "route": "jev" if prefix == "jev" else "chat",
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
        (FULL / "incident.json").read_text(encoding="utf-8")
    )
    return {
        "questions": len(questions),
        "seats_per_question_per_model": SEATS,
        "models": models,
        "incident": incident,
    }


def build_agreement_golden(questions):
    """Render the agreement golden through the agreement command."""
    paths = [
        str(FULL / f"poll-{index:02d}.json")
        for index in range(1, len(questions) + 1)
    ]
    buffer = io.StringIO()
    with contextlib.redirect_stdout(buffer):
        status = cli_main(["agreement"] + paths)
    assert status == 0
    return buffer.getvalue()


def main():
    questions = json.loads(
        (FULL / "questions.json").read_text(encoding="utf-8")
    )
    polls = [
        build_poll(index, item)
        for index, item in enumerate(questions, start=1)
    ]
    for index, poll in enumerate(polls, start=1):
        (FULL / f"poll-{index:02d}.json").write_text(
            json.dumps(poll, indent=2, ensure_ascii=False) + "\n",
            encoding="utf-8",
        )
    example = polls[0]
    (HERE / "full_poll.json").write_text(
        json.dumps(example, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )
    prices_text = (HERE / "full-prices.json").read_text(
        encoding="utf-8"
    )
    table, defects = load_prices_text(prices_text)
    assert defects == []
    (HERE / "full-stadium-golden.txt").write_text(
        "\n".join(render_stadium(example)) + "\n", encoding="utf-8"
    )
    (HERE / "full-cost-golden.txt").write_text(
        "\n".join(cost_lines(example, table)) + "\n", encoding="utf-8"
    )
    (HERE / "full-agreement-golden.txt").write_text(
        build_agreement_golden(questions), encoding="utf-8"
    )
    record = build_run_record(questions)
    (FULL / "run_record.json").write_text(
        json.dumps(record, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )
    recorded = sum(len(poll["responses"]) for poll in polls)
    print(f"polls {len(polls)} responses {recorded}")


if __name__ == "__main__":
    main()
