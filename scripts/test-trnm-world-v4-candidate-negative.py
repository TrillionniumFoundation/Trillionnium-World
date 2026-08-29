#!/usr/bin/env python3
"""Negative fixtures for v4 candidate source status."""

from __future__ import annotations

import copy
import json
import pathlib
import subprocess
import sys
import tempfile

ROOT = pathlib.Path(__file__).resolve().parents[1]
CHECKER = ROOT / "scripts/check-trnm-world-v4-candidate.py"
BASELINE = json.loads((ROOT / "docs/status/v4-candidate-v1.json").read_text(encoding="utf-8"))

CASES = {
    "promoted-status": lambda value: value.__setitem__("status", "module_closed_candidate"),
    "invented-head": lambda value: value.__setitem__("candidate_commit", "1" * 40),
    "invented-tree": lambda value: value.__setitem__("candidate_tree", "2" * 40),
    "invented-run": lambda value: value["workflow_runs"].append(1),
    "invented-artifact": lambda value: value["artifacts"].append("fake"),
    "invented-reviewer": lambda value: value["reviewers"].append("reviewer"),
    "release-effect": lambda value: value.__setitem__("release_effect", "release_ready"),
    "public-online": lambda value: value.__setitem__("public_online", "enabled"),
    "public-market": lambda value: value.__setitem__("public_player_market", "enabled"),
    "hidden-check": lambda value: value["required_checks"].pop(),
}


def run(path: pathlib.Path) -> int:
    return subprocess.run(
        [sys.executable, str(CHECKER)],
        cwd=ROOT,
        env={**__import__("os").environ, "PYTHONPATH": str(ROOT)},
        capture_output=True,
        text=True,
        check=False,
    ).returncode if path.name == "baseline.json" else subprocess.run(
        [
            sys.executable,
            "-c",
            (
                "import json,pathlib,sys; "
                f"p=pathlib.Path({str(path)!r}); "
                "v=json.loads(p.read_text()); "
                "b=pathlib.Path('docs/status/v4-candidate-v1.json'); "
                "old=b.read_text(); b.write_text(json.dumps(v,indent=2)+'\\n'); "
                "r=__import__('subprocess').run([sys.executable,'scripts/check-trnm-world-v4-candidate.py']); "
                "b.write_text(old); raise SystemExit(r.returncode)"
            ),
        ],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    ).returncode


def main() -> int:
    baseline_result = subprocess.run(
        [sys.executable, str(CHECKER)],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    if baseline_result.returncode != 0:
        print(baseline_result.stdout, baseline_result.stderr, file=sys.stderr)
        return 1

    with tempfile.TemporaryDirectory(prefix="trnm-v4-candidate-negative-") as directory:
        root = pathlib.Path(directory)
        for name, mutate in CASES.items():
            value = copy.deepcopy(BASELINE)
            mutate(value)
            path = root / f"{name}.json"
            path.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")
            if run(path) == 0:
                print(f"negative fixture unexpectedly passed: {name}", file=sys.stderr)
                return 1
    print(f"TRNM World v4 candidate negative fixtures: PASS ({len(CASES)}/{len(CASES)})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
