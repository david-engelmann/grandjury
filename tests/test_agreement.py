"""Pins for GJ-8, the agreement reading.

The reading states each jury share as an exact fraction, the
plurality under the stated tie rule, and the closing agreement
count. Every pin here builds its polls by hand, so each one
fails under the negation of the rule it names. The golden pin
holds the reading of the forty recorded full polls to the
committed golden byte for byte, through the function and
through the command.
"""

import json
import pathlib
import re

from grandjury.agreement import agreement_lines
from grandjury.cli import main
from grandjury.polls import load_poll_file

EXAMPLES = pathlib.Path(__file__).resolve().parent.parent / "examples"
FULL = EXAMPLES / "full"

VERDICT_WORDS = (
    "verdict", "winner", "wins", "correct", "incorrect", "wrong",
    "best", "worst", "rank", "ranking", "ranked", "beats",
    "champion", "loser", "loses",
)


def response(juror, model, answer, abstained=False):
    entry = {
        "juror": juror,
        "model": model,
        "answer": answer,
        "reason": "a reason",
    }
    if abstained:
        entry["abstained"] = True
    return entry


def poll(question, answers, responses):
    return {
        "question": question,
        "answers": answers,
        "responses": responses,
    }


def door_poll():
    return poll(
        "Which door?",
        ["red", "blue", "green"],
        [
            response("jev-1", "jev-a", "red"),
            response("jev-2", "jev-a", "red"),
            response("jev-3", "jev-a", "blue"),
            response("jev-4", "jev-a", "purple"),
            response("jev-5", "jev-a", "red", abstained=True),
            response("openai-1", "chat-b", "blue"),
            response("openai-2", "chat-b", "blue"),
            response("openai-3", "chat-b", "red"),
            response("openai-4", "chat-b", "green"),
            response("openai-5", "chat-b", "green"),
        ],
    )


def test_the_reading_of_one_poll_stands_line_for_line():
    assert agreement_lines([door_poll()]) == [
        "question, Which door?",
        "jury, jev-a, recorded 5, valid 3, abstained 1, spoiled 1",
        "  red, 2, 2/3, 66.7%",
        "  blue, 1, 1/3, 33.3%",
        "  green, 0, 0, 0.0%",
        "plurality, jev-a, red",
        "jury, chat-b, recorded 5, valid 5, abstained 0, spoiled 0",
        "  red, 1, 1/5, 20.0%",
        "  blue, 2, 2/5, 40.0%",
        "  green, 2, 2/5, 40.0%",
        "plurality, chat-b, none",
        "pluralities, not counted",
        "agreements, 0 of 0",
    ]


def test_a_tie_at_the_top_leaves_no_plurality():
    tied = poll(
        "Tied?",
        ["a", "b"],
        [
            response("jev-1", "jev-a", "a"),
            response("jev-2", "jev-a", "b"),
            response("openai-1", "chat-b", "a"),
            response("openai-2", "chat-b", "a"),
        ],
    )
    lines = agreement_lines([tied])
    assert "plurality, jev-a, none" in lines
    assert "plurality, chat-b, a" in lines
    assert "pluralities, not counted" in lines
    assert lines[-1] == "agreements, 0 of 0"


def test_a_tie_below_the_top_leaves_the_plurality_standing():
    tied_below = poll(
        "Clear at the top?",
        ["a", "b", "c"],
        [
            response("jev-1", "jev-a", "a"),
            response("jev-2", "jev-a", "a"),
            response("jev-3", "jev-a", "a"),
            response("jev-4", "jev-a", "b"),
            response("jev-5", "jev-a", "b"),
            response("openai-1", "chat-b", "a"),
        ],
    )
    lines = agreement_lines([tied_below])
    assert "plurality, jev-a, a" in lines


def test_agree_and_differ_close_the_count():
    agree = poll(
        "Same?",
        ["a", "b"],
        [
            response("jev-1", "jev-a", "a"),
            response("jev-2", "jev-a", "a"),
            response("openai-1", "chat-b", "a"),
            response("openai-2", "chat-b", "b"),
            response("openai-3", "chat-b", "a"),
        ],
    )
    differ = poll(
        "Apart?",
        ["a", "b"],
        [
            response("jev-1", "jev-a", "b"),
            response("openai-1", "chat-b", "a"),
        ],
    )
    lines = agreement_lines([agree, differ, door_poll()])
    assert lines.count("pluralities, agree") == 1
    assert lines.count("pluralities, differ") == 1
    assert lines.count("pluralities, not counted") == 1
    assert lines[-1] == "agreements, 1 of 2"


def test_a_jury_with_no_valid_responses_holds_no_plurality():
    silent = poll(
        "Silent?",
        ["a", "b"],
        [
            response("jev-1", "jev-a", "a", abstained=True),
            response("jev-2", "jev-a", "b", abstained=True),
            response("openai-1", "chat-b", "a"),
        ],
    )
    lines = agreement_lines([silent])
    assert "jury, jev-a, recorded 2, valid 0, abstained 2, spoiled 0" in lines
    assert "  a, 0, 0, 0.0%" in lines
    assert "  b, 0, 0, 0.0%" in lines
    assert "plurality, jev-a, none" in lines
    assert "pluralities, not counted" in lines


def test_a_zero_valid_jury_in_a_one_answer_poll_holds_no_plurality():
    one_answer = poll(
        "Only one way?",
        ["a"],
        [
            response("jev-1", "jev-a", "a", abstained=True),
            response("jev-2", "jev-a", "a", abstained=True),
            response("openai-1", "chat-b", "a", abstained=True),
            response("openai-2", "chat-b", "b"),
        ],
    )
    assert agreement_lines([one_answer]) == [
        "question, Only one way?",
        "jury, jev-a, recorded 2, valid 0, abstained 2, spoiled 0",
        "  a, 0, 0, 0.0%",
        "plurality, jev-a, none",
        "jury, chat-b, recorded 2, valid 0, abstained 1, spoiled 1",
        "  a, 0, 0, 0.0%",
        "plurality, chat-b, none",
        "pluralities, not counted",
        "agreements, 0 of 0",
    ]


def test_two_zero_valid_juries_in_a_two_answer_poll_hold_no_plurality():
    hushed = poll(
        "Hushed?",
        ["a", "b"],
        [
            response("jev-1", "jev-a", "a", abstained=True),
            response("openai-1", "chat-b", "not offered"),
        ],
    )
    lines = agreement_lines([hushed])
    assert "plurality, jev-a, none" in lines
    assert "plurality, chat-b, none" in lines
    assert "pluralities, not counted" in lines
    assert lines[-1] == "agreements, 0 of 0"


def test_spoiled_and_abstained_stand_outside_the_shares():
    mixed = poll(
        "Mixed?",
        ["a", "b"],
        [
            response("jev-1", "jev-a", "a"),
            response("jev-2", "jev-a", "a"),
            response("jev-3", "jev-a", "not offered"),
            response("jev-4", "jev-a", "b", abstained=True),
            response("openai-1", "chat-b", "b"),
        ],
    )
    lines = agreement_lines([mixed])
    jury_line = (
        "jury, jev-a, recorded 4, valid 2, abstained 1, spoiled 1"
    )
    assert jury_line in lines
    assert "  a, 2, 1, 100.0%" in lines
    assert "  b, 0, 0, 0.0%" in lines


def test_the_share_renders_lowest_terms_and_halves_up():
    sixteen = poll(
        "Sixteenths?",
        ["a", "b"],
        [response(f"jev-{i}", "jev-a", "a") for i in range(1, 2)]
        + [response(f"jev-{i}", "jev-a", "b") for i in range(2, 17)]
        + [response("openai-1", "chat-b", "a")],
    )
    lines = agreement_lines([sixteen])
    assert "  a, 1, 1/16, 6.3%" in lines
    assert "  b, 15, 15/16, 93.8%" in lines


def test_juries_stand_in_first_appearance_order():
    reordered = poll(
        "Order?",
        ["a", "b"],
        [
            response("openai-1", "chat-b", "a"),
            response("jev-1", "jev-a", "a"),
        ],
    )
    lines = agreement_lines([reordered])
    chat_line = next(
        line for line in lines if line.startswith("jury, chat-b")
    )
    jev_line = next(
        line for line in lines if line.startswith("jury, jev-a")
    )
    assert lines.index(chat_line) < lines.index(jev_line)
    assert "pluralities, agree" in lines


def test_three_juries_are_not_counted():
    crowd = poll(
        "Crowd?",
        ["a", "b"],
        [
            response("one-1", "model-one", "a"),
            response("two-1", "model-two", "a"),
            response("three-1", "model-three", "a"),
        ],
    )
    lines = agreement_lines([crowd])
    assert "pluralities, not counted" in lines
    assert lines[-1] == "agreements, 0 of 0"


def test_one_jury_is_not_counted():
    solo = poll(
        "Solo?",
        ["a", "b"],
        [response("jev-1", "jev-a", "a")],
    )
    lines = agreement_lines([solo])
    assert "pluralities, not counted" in lines
    assert lines[-1] == "agreements, 0 of 0"


def test_a_poll_with_no_responses_reads_not_counted():
    empty = poll("Nobody?", ["a", "b"], [])
    assert agreement_lines([empty]) == [
        "question, Nobody?",
        "pluralities, not counted",
        "agreements, 0 of 0",
    ]


def write(tmp_path, name, data):
    path = tmp_path / name
    path.write_text(json.dumps(data), encoding="utf-8")
    return str(path)


def test_the_agreement_command_prints_the_reading(tmp_path, capsys):
    first = write(tmp_path, "one.json", door_poll())
    second = write(
        tmp_path,
        "two.json",
        poll(
            "Apart?",
            ["a", "b"],
            [
                response("jev-1", "jev-a", "b"),
                response("openai-1", "chat-b", "a"),
            ],
        ),
    )
    assert main(["agreement", first, second]) == 0
    out = capsys.readouterr().out
    assert "question, Which door?" in out
    assert out.rstrip().endswith("agreements, 0 of 1")


def test_the_agreement_command_names_a_malformed_poll(
        tmp_path, capsys):
    good = write(tmp_path, "good.json", door_poll())
    bad_data = door_poll()
    bad_data["question"] = ""
    bad = write(tmp_path, "bad.json", bad_data)
    assert main(["agreement", good, bad]) == 1
    out = capsys.readouterr().out
    assert "invalid poll" in out
    assert "defect field question is empty" in out
    assert "question, Which door?" not in out
    assert "agreements," not in out


def test_the_agreement_command_names_a_missing_file(
        tmp_path, capsys):
    missing = str(tmp_path / "missing.json")
    assert main(["agreement", missing]) == 1
    out = capsys.readouterr().out
    assert "defect file cannot be read" in out


def full_poll_paths():
    return [
        str(FULL / f"poll-{index:02d}.json") for index in range(1, 41)
    ]


def test_the_agreement_golden_matches_the_function():
    polls = []
    for path in full_poll_paths():
        loaded, defects = load_poll_file(path)
        assert defects == []
        polls.append(loaded)
    golden = (EXAMPLES / "full-agreement-golden.txt").read_text(
        encoding="utf-8"
    )
    assert "\n".join(agreement_lines(polls)) + "\n" == golden


def test_the_agreement_golden_matches_the_command(capsys):
    assert main(["agreement"] + full_poll_paths()) == 0
    out = capsys.readouterr().out
    golden = (EXAMPLES / "full-agreement-golden.txt").read_text(
        encoding="utf-8"
    )
    assert out == golden


def test_the_agreement_golden_carries_no_verdict_vocabulary():
    golden = (EXAMPLES / "full-agreement-golden.txt").read_text(
        encoding="utf-8"
    )
    words = set(re.findall(r"[a-z]+", golden.lower()))
    for word in VERDICT_WORDS:
        assert word not in words, word
