"""Pins for the command line surface.

Each command prints its report and exits with status zero on a
valid input, and prints the defects and exits with status one on a
malformed poll or a malformed price table.
"""

import json

from grandjury.cli import main


def write(tmp_path, name, data):
    path = tmp_path / name
    path.write_text(json.dumps(data), encoding="utf-8")
    return str(path)


def poll_data():
    return {
        "question": "Which door should the agent pick?",
        "answers": ["red", "blue"],
        "responses": [
            {"juror": "juror-1", "model": "tiny-a", "answer": "red",
             "reason": "the shortest path"},
        ],
    }


def test_the_tally_command_prints_the_tally(tmp_path, capsys):
    path = write(tmp_path, "poll.json", poll_data())
    assert main(["tally", path]) == 0
    out = capsys.readouterr().out
    assert "question, Which door should the agent pick?" in out
    assert "red, 1, 100.0%" in out
    assert "blue, 0, 0.0%" in out


def test_the_stadium_command_prints_the_stadium_view(tmp_path, capsys):
    path = write(tmp_path, "poll.json", poll_data())
    assert main(["stadium", path]) == 0
    out = capsys.readouterr().out
    assert "wall of reasons" in out
    assert "  juror-1, the shortest path" in out


def test_the_cost_command_prints_the_cost_report(tmp_path, capsys):
    poll_path = write(tmp_path, "poll.json", poll_data())
    prices_path = write(tmp_path, "prices.json",
                        {"prices": {"tiny-a": "0.0002"}})
    assert main(["cost", poll_path, prices_path]) == 0
    out = capsys.readouterr().out
    assert "priced, 1 responses, 0.000200 dollars" in out


def test_a_malformed_poll_exits_one_on_every_command(tmp_path, capsys):
    data = poll_data()
    data["question"] = ""
    path = write(tmp_path, "poll.json", data)
    prices_path = write(tmp_path, "prices.json",
                        {"prices": {"tiny-a": "0.0002"}})
    assert main(["tally", path]) == 1
    assert main(["stadium", path]) == 1
    assert main(["cost", path, prices_path]) == 1
    out = capsys.readouterr().out
    assert "invalid poll" in out
    assert "defect field question is empty" in out


def test_a_malformed_price_table_exits_one_on_cost(tmp_path, capsys):
    poll_path = write(tmp_path, "poll.json", poll_data())
    prices_path = write(tmp_path, "prices.json",
                        {"prices": {"tiny-a": "cheap"}})
    assert main(["cost", poll_path, prices_path]) == 1
    out = capsys.readouterr().out
    assert "invalid price table" in out
    assert "defect price for tiny-a is not a decimal string" in out


def test_a_missing_poll_file_exits_one(tmp_path, capsys):
    missing = str(tmp_path / "missing.json")
    assert main(["tally", missing]) == 1
    out = capsys.readouterr().out
    assert "defect file cannot be read" in out


def cost_with_price(tmp_path, capsys, price):
    poll_path = write(tmp_path, "poll.json", poll_data())
    prices_path = write(tmp_path, "prices.json",
                        {"prices": {"tiny-a": price}})
    status = main(["cost", poll_path, prices_path])
    captured = capsys.readouterr()
    return status, captured.out, captured.err


def test_the_cost_command_renders_a_1e30_price_in_full(
        tmp_path, capsys):
    status, out, err = cost_with_price(tmp_path, capsys, "1e30")
    assert status == 0
    assert err == ""
    assert "1" + "0" * 30 + ".000000 dollars" in out


def test_the_cost_command_renders_a_1e22_price_in_full(
        tmp_path, capsys):
    status, out, err = cost_with_price(tmp_path, capsys, "1e22")
    assert status == 0
    assert err == ""
    assert "1" + "0" * 22 + ".000000 dollars" in out


def test_the_cost_command_renders_a_23_integer_digit_price(
        tmp_path, capsys):
    status, out, err = cost_with_price(
        tmp_path, capsys, "99999999999999999999999.99"
    )
    assert status == 0
    assert err == ""
    assert "99999999999999999999999.990000 dollars" in out
