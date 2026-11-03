"""Pins for the codepoint scan scope stated in SETUP-3 as amended
by the scan scope amendment.

The scan is run as a subprocess against a planted tree, because the
rule is about which files on disk the scan opens, and only a real run
against a real tree can pin that. Each planted tree carries the scan
script itself under scripts, which is where the scan looks for its
own root.
"""

import pathlib
import shutil
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
EM_DASH = "\u2014"
EN_DASH = "\u2013"


def plant_tree(tmp_path):
    scripts = tmp_path / "scripts"
    scripts.mkdir()
    shutil.copy(
        ROOT / "scripts" / "codepoint_scan.py",
        scripts / "codepoint_scan.py",
    )
    return tmp_path


def run_scan(root):
    return subprocess.run(
        [sys.executable, str(root / "scripts" / "codepoint_scan.py")],
        capture_output=True,
        text=True,
    )


def test_a_clean_planted_tree_passes(tmp_path):
    root = plant_tree(tmp_path)
    (root / "notes.md").write_text("plain text\n", encoding="utf-8")
    result = run_scan(root)
    assert result.returncode == 0
    assert "U+2014=0" in result.stdout
    assert "U+2013=0" in result.stdout


def test_a_dash_in_an_extensionless_file_fails_the_scan(tmp_path):
    root = plant_tree(tmp_path)
    (root / "LICENSE").write_text(
        f"licence text {EM_DASH} more text\n", encoding="utf-8"
    )
    result = run_scan(root)
    assert result.returncode == 1
    assert "U+2014=1" in result.stdout


def test_a_dash_in_a_python_file_fails_the_scan(tmp_path):
    root = plant_tree(tmp_path)
    (root / "module.py").write_text(
        f"# a comment {EN_DASH} here\n", encoding="utf-8"
    )
    result = run_scan(root)
    assert result.returncode == 1
    assert "U+2013=1" in result.stdout


def test_dashes_inside_an_ignored_directory_pass_the_scan(tmp_path):
    root = plant_tree(tmp_path)
    venv = root / ".venv"
    venv.mkdir()
    (venv / "planted.md").write_text(
        f"{EM_DASH} and {EN_DASH}\n", encoding="utf-8"
    )
    result = run_scan(root)
    assert result.returncode == 0
    assert "U+2014=0" in result.stdout
    assert "U+2013=0" in result.stdout


def test_dashes_inside_the_recorded_envelopes_pass_the_scan(tmp_path):
    root = plant_tree(tmp_path)
    envelopes = root / "examples" / "pilot" / "envelopes"
    envelopes.mkdir(parents=True)
    (envelopes / "q01-jev-1.json").write_text(
        f"{{\"reading\": \"one {EM_DASH} two {EN_DASH} three\"}}\n",
        encoding="utf-8",
    )
    result = run_scan(root)
    assert result.returncode == 0
    assert "U+2014=0" in result.stdout
    assert "U+2013=0" in result.stdout


def test_a_dash_in_the_pilot_directory_outside_envelopes_fails(tmp_path):
    root = plant_tree(tmp_path)
    pilot = root / "examples" / "pilot"
    pilot.mkdir(parents=True)
    (pilot / "poll-01.json").write_text(
        f"{{\"question\": \"one {EM_DASH} two\"}}\n", encoding="utf-8"
    )
    result = run_scan(root)
    assert result.returncode == 1
    assert "U+2014=1" in result.stdout


def test_dashes_inside_the_full_envelopes_pass_the_scan(tmp_path):
    root = plant_tree(tmp_path)
    envelopes = root / "examples" / "full" / "envelopes"
    envelopes.mkdir(parents=True)
    (envelopes / "q01-openai-1.json").write_text(
        f"{{\"reading\": \"one {EM_DASH} two {EN_DASH} three\"}}\n",
        encoding="utf-8",
    )
    result = run_scan(root)
    assert result.returncode == 0
    assert "U+2014=0" in result.stdout
    assert "U+2013=0" in result.stdout


def test_a_dash_in_the_full_directory_outside_envelopes_fails(tmp_path):
    root = plant_tree(tmp_path)
    full = root / "examples" / "full"
    full.mkdir(parents=True)
    (full / "poll-01.json").write_text(
        f"{{\"question\": \"one {EM_DASH} two\"}}\n", encoding="utf-8"
    )
    result = run_scan(root)
    assert result.returncode == 1
    assert "U+2014=1" in result.stdout


def test_a_dash_elsewhere_in_examples_still_fails_the_scan(tmp_path):
    root = plant_tree(tmp_path)
    examples = root / "examples"
    examples.mkdir()
    (examples / "notes.md").write_text(
        f"one {EN_DASH} two\n", encoding="utf-8"
    )
    result = run_scan(root)
    assert result.returncode == 1
    assert "U+2013=1" in result.stdout


def test_a_dash_in_a_rust_file_fails_the_scan(tmp_path):
    root = plant_tree(tmp_path)
    (root / "module.rs").write_text(
        f"// a comment {EM_DASH} here\n", encoding="utf-8"
    )
    result = run_scan(root)
    assert result.returncode == 1
    assert "U+2014=1" in result.stdout


def test_dashes_inside_the_target_directory_pass_the_scan(tmp_path):
    root = plant_tree(tmp_path)
    target = root / "target"
    target.mkdir()
    (target / "planted.md").write_text(
        f"{EM_DASH} and {EN_DASH}\n", encoding="utf-8"
    )
    result = run_scan(root)
    assert result.returncode == 0
    assert "U+2014=0" in result.stdout
    assert "U+2013=0" in result.stdout
