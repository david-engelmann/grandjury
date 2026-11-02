"""Setup pins. Each test can fail for a real reason against a repo that
lost a setup file or let its status drift from its contract."""
import pathlib, re
ROOT = pathlib.Path(__file__).resolve().parent.parent

def test_agents_carries_the_canonical_marker():
    text = (ROOT / "AGENTS.md").read_text(encoding="utf-8")
    assert "<!-- repo annex below this line -->" in text
    assert "39 repo storefront perfection program" in text

def test_readme_states_the_closed_round_count():
    text = (ROOT / "README.md").read_text(encoding="utf-8")
    assert re.search(r"Closed perfection rounds, 1 of 100", text)

def test_contract_carries_the_setup_obligations():
    text = (ROOT / "CONTRACT.md").read_text(encoding="utf-8")
    for oid in ("SETUP-1", "SETUP-2", "SETUP-3", "SETUP-4", "GJ-1"):
        assert oid in text, oid

def test_readme_reports_the_contract_status():
    text = (ROOT / "README.md").read_text(encoding="utf-8")
    expected = (
        "- Contract, SETUP-1 to SETUP-4 and GJ-1 to GJ-4 met. "
        "GJ-1 to GJ-4 stand in full form in CONTRACT.md with the Round 1 "
        "repair amendments, and Round 1 closed on the merged slice 2 after "
        "three reviews, a repair pass, and an approving re-review."
    )
    assert expected in text.splitlines()

def test_readme_reports_the_round_two_status():
    text = (ROOT / "README.md").read_text(encoding="utf-8")
    lines = text.splitlines()
    standing = (
        "- Contract, GJ-5 and GJ-6 now stand in full form in CONTRACT.md "
        "under the Round 2 obligations, and their standing is not "
        "claimed here, as Round 2 has not closed."
    )
    assert standing in lines
    slice_line = (
        "- Slice 3 is built and in review. It carries the juror seat "
        "readers over the two decision model routes, the abstained "
        "response kind through the tally, the stadium, and the cost "
        "counter, and the recorded pilot over both models, twelve "
        "synthetic questions with five seats per question per model and "
        "the raw envelopes and the run record under examples."
    )
    assert slice_line in lines
    assert text.count("built and in review") == 1

def test_ci_runs_the_same_gate_as_the_local_gate():
    text = (ROOT / ".github" / "workflows" / "ci.yml").read_text(encoding="utf-8")
    live = "\n".join(line.split("#", 1)[0] for line in text.splitlines())
    assert "python3 scripts/codepoint_scan.py" in live
    assert "python3 -m pytest tests -q" in live
