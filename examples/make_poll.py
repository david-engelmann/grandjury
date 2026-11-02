"""Generate the recorded example poll and its golden renderings.

This script was run once to author the synthetic fixture standing
beside it. The poll is invented for this repo. Jurors, models,
answers, and reasons are all synthetic, and the pattern that lays
the responses out is fixed arithmetic, so running the script again
reproduces the same files byte for byte.

Run from the repo root with the package importable, for example
PYTHONPATH=src python3 examples/make_poll.py
"""

import json
import pathlib
import sys

sys.path.insert(
    0, str(pathlib.Path(__file__).resolve().parent.parent / "src")
)

from grandjury.polls import cost_lines, load_prices_text, render_stadium

HERE = pathlib.Path(__file__).resolve().parent

QUESTION = "Should the night shift agent deploy on Fridays?"
ANSWERS = ["deploy", "hold", "deploy with a watcher"]
SPOILED_ANSWERS = ["deploy at dawn", "ask a human first", "never deploy"]

REASONS = {
    "deploy": [
        "the queue is clear and the tests are green",
        "Friday traffic is the lightest of the week",
        "the change is small and well fenced",
        "waiting until Monday helps nobody",
        "the rollback path was rehearsed twice",
        "the diff touches nothing shared",
    ],
    "hold": [
        "nobody is awake to watch the graphs",
        "a Friday fault festers until Monday",
        "the on call rotation is thin this week",
        "the last Friday deploy cost a weekend",
        "the change can wait two days safely",
        "the review was rushed and it shows",
    ],
    "deploy with a watcher": [
        "the watcher catches what the graphs miss",
        "a named human beats a paging rule",
        "the risk is real but watchable",
        "deploy, but keep eyes on the error rate",
        "the watcher can roll back in one step",
        "attention is cheaper than a weekend",
    ],
}
SPOILED_REASONS = [
    "the question felt too narrow",
    "answering from habit, not evidence",
    "the offered answers missed the point",
]


def build_poll():
    responses = []
    reason_counts = {answer: 0 for answer in ANSWERS}
    spoiled_count = 0
    for index in range(124):
        juror = f"juror-{index + 1:03d}"
        if index % 31 == 13:
            model = "tiny-c"
        elif index % 2 == 0:
            model = "tiny-a"
        else:
            model = "tiny-b"
        if index % 29 == 11:
            answer = SPOILED_ANSWERS[spoiled_count % len(SPOILED_ANSWERS)]
            reason = SPOILED_REASONS[spoiled_count % len(SPOILED_REASONS)]
            spoiled_count += 1
        else:
            slot = index % 10
            if slot < 5:
                answer = "deploy"
            elif slot < 8:
                answer = "hold"
            else:
                answer = "deploy with a watcher"
            pool = REASONS[answer]
            reason = pool[reason_counts[answer] % len(pool)]
            reason_counts[answer] += 1
        responses.append(
            {"juror": juror, "model": model, "answer": answer,
             "reason": reason}
        )
    return {"question": QUESTION, "answers": ANSWERS,
            "responses": responses}


def main():
    poll = build_poll()
    (HERE / "recorded_poll.json").write_text(
        json.dumps(poll, indent=2) + "\n", encoding="utf-8"
    )
    prices = {"prices": {"tiny-a": "0.0002", "tiny-b": "0.00035"}}
    (HERE / "prices.json").write_text(
        json.dumps(prices, indent=2) + "\n", encoding="utf-8"
    )
    table, defects = load_prices_text(json.dumps(prices))
    assert defects == []
    (HERE / "stadium-golden.txt").write_text(
        "\n".join(render_stadium(poll)) + "\n", encoding="utf-8"
    )
    (HERE / "cost-golden.txt").write_text(
        "\n".join(cost_lines(poll, table)) + "\n", encoding="utf-8"
    )
    print(f"responses {len(poll['responses'])}")


if __name__ == "__main__":
    main()
