"""The grandjury command line surface.

Three commands read recorded polls from local files. The tally
command prints the tally, the stadium command prints the stadium
view, and the cost command prints what the poll would have cost
live from a price table. Each command prints the defects of a
malformed input and exits with status one. Nothing here touches
the network, and no command makes a live call.
"""

import argparse
import sys

from grandjury.polls import (
    cost_lines,
    load_poll_file,
    load_prices_file,
    render_stadium,
    render_tally_lines,
)


def _load_poll_or_print_defects(path):
    poll, defects = load_poll_file(path)
    if defects:
        print("invalid poll")
        for defect in defects:
            print(f"defect {defect}")
        return None
    return poll


def _command_tally(args):
    poll = _load_poll_or_print_defects(args.poll_file)
    if poll is None:
        return 1
    for line in render_tally_lines(poll):
        print(line)
    return 0


def _command_stadium(args):
    poll = _load_poll_or_print_defects(args.poll_file)
    if poll is None:
        return 1
    for line in render_stadium(poll):
        print(line)
    return 0


def _command_cost(args):
    poll = _load_poll_or_print_defects(args.poll_file)
    prices, price_defects = load_prices_file(args.prices_file)
    if price_defects:
        print("invalid price table")
        for defect in price_defects:
            print(f"defect {defect}")
    if poll is None or prices is None:
        return 1
    for line in cost_lines(poll, prices):
        print(line)
    return 0


def main(argv=None):
    """Run the grandjury command line and return the exit status."""
    parser = argparse.ArgumentParser(
        prog="grandjury",
        description="Poll tiny models and render the stadium vote.",
    )
    commands = parser.add_subparsers(dest="command", required=True)
    tally_parser = commands.add_parser(
        "tally", help="print the tally of a recorded poll"
    )
    tally_parser.add_argument("poll_file")
    tally_parser.set_defaults(handler=_command_tally)
    stadium_parser = commands.add_parser(
        "stadium", help="print the stadium view of a recorded poll"
    )
    stadium_parser.add_argument("poll_file")
    stadium_parser.set_defaults(handler=_command_stadium)
    cost_parser = commands.add_parser(
        "cost", help="print the cost report of a recorded poll"
    )
    cost_parser.add_argument("poll_file")
    cost_parser.add_argument("prices_file")
    cost_parser.set_defaults(handler=_command_cost)
    args = parser.parse_args(argv)
    return args.handler(args)


if __name__ == "__main__":
    sys.exit(main())
