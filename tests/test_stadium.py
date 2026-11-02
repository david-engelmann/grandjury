"""Pins for GJ-3, the stadium view.

The bar scaling is pinned at stated cases, the wall order is pinned
line by line, and the spoiled section is pinned present and absent.
"""

import json

from grandjury.polls import load_poll_text, render_stadium


def response(juror, answer, reason="a reason", model="tiny-a"):
    return {"juror": juror, "model": model, "answer": answer,
            "reason": reason}


def load(data):
    poll, defects = load_poll_text(json.dumps(data))
    assert defects == []
    return poll


def test_the_stadium_view_of_a_small_poll_line_by_line():
    poll = load({
        "question": "Which door should the agent pick?",
        "answers": ["red", "blue"],
        "responses": [
            response("juror-1", "red", "the shortest path"),
            response("juror-2", "blue", "a calmer colour"),
            response("juror-3", "red", "matches the brief"),
        ],
    })
    assert render_stadium(poll) == [
        "question, Which door should the agent pick?",
        "responses, 3",
        "red |" + "#" * 40 + "| 2, 66.7%",
        "blue |" + "#" * 20 + "| 1, 33.3%",
        "wall of reasons",
        "red",
        "  juror-1, the shortest path",
        "  juror-3, matches the brief",
        "blue",
        "  juror-2, a calmer colour",
    ]


def test_the_bar_scales_against_the_largest_count_at_width_40():
    responses = [response("juror-1", "red"), response("juror-2", "red"),
                 response("juror-3", "red"),
                 response("juror-4", "blue")]
    poll = load({
        "question": "q",
        "answers": ["red", "blue", "green"],
        "responses": responses,
    })
    lines = render_stadium(poll)
    assert lines[2] == "red |" + "#" * 40 + "| 3, 75.0%"
    assert lines[3] == "blue |" + "#" * 13 + "| 1, 25.0%"
    assert lines[4] == "green || 0, 0.0%"


def test_every_bar_is_empty_when_no_response_is_valid():
    poll = load({
        "question": "q",
        "answers": ["red", "blue"],
        "responses": [response("juror-1", "purple")],
    })
    lines = render_stadium(poll)
    assert lines[2] == "red || 0, 0.0%"
    assert lines[3] == "blue || 0, 0.0%"


def test_an_answer_nobody_chose_stands_in_the_wall_with_no_reasons():
    poll = load({
        "question": "q",
        "answers": ["red", "blue", "green"],
        "responses": [response("juror-1", "blue", "a calmer colour")],
    })
    lines = render_stadium(poll)
    wall = lines[lines.index("wall of reasons"):]
    assert wall == [
        "wall of reasons",
        "red",
        "blue",
        "  juror-1, a calmer colour",
        "green",
    ]


def test_the_wall_groups_in_offered_order_not_response_order():
    poll = load({
        "question": "q",
        "answers": ["red", "blue"],
        "responses": [
            response("juror-1", "blue", "first cast"),
            response("juror-2", "red", "second cast"),
            response("juror-3", "blue", "third cast"),
        ],
    })
    lines = render_stadium(poll)
    wall = lines[lines.index("wall of reasons"):]
    assert wall == [
        "wall of reasons",
        "red",
        "  juror-2, second cast",
        "blue",
        "  juror-1, first cast",
        "  juror-3, third cast",
    ]


def test_the_spoiled_section_names_each_spoiled_response():
    poll = load({
        "question": "q",
        "answers": ["red", "blue"],
        "responses": [
            response("juror-1", "red"),
            response("juror-2", "purple"),
            response("juror-3", "teal"),
        ],
    })
    lines = render_stadium(poll)
    assert lines[-3:] == [
        "spoiled, 2",
        "  juror-2, purple",
        "  juror-3, teal",
    ]


def test_no_spoiled_section_stands_when_nothing_is_spoiled():
    poll = load({
        "question": "q",
        "answers": ["red", "blue"],
        "responses": [response("juror-1", "red")],
    })
    lines = render_stadium(poll)
    assert not any(line.startswith("spoiled") for line in lines)


def test_the_same_poll_renders_byte_identical_lines_twice():
    poll = load({
        "question": "q",
        "answers": ["red", "blue"],
        "responses": [
            response("juror-1", "red"),
            response("juror-2", "purple"),
        ],
    })
    first = "\n".join(render_stadium(poll))
    second = "\n".join(render_stadium(poll))
    assert first == second
