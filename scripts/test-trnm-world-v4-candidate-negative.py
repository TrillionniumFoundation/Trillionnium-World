#!/usr/bin/env python3
"""Read-only negative fixtures for v4 candidate source status."""

from __future__ import annotations

import copy
import json
import pathlib
import subprocess
import sys
import tempfile

ROOT = pathlib.Path(__file__).resolve().parents[1]
CHECKER = ROOT / "scripts/check-trnm-world-v4-candidate.py"
SCHEMA = ROOT / "docs/status/v4-candidate-v1.schema.json"
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


def invoke(status: pathlib.Path) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [
            sys.executable,
            str(CHECKER),
            "--status",
            str(status),
            "--schema",
            str(SCHEMA),
        ],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )


def main() -> int:
    with tempfile.TemporaryDirectory(prefix="trnm-v4-candidate-negative-") as directory:
        root = pathlib.Path(directory)
        baseline_path = root / "baseline.json"
        baseline_path.write_text(json.dumps(BASELINE, indent=2) + "\n", encoding="utf-8")
        baseline_result = invoke(baseline_path)
        if baseline_result.returncode != 0:
            print(baseline_result.stdout, baseline_result.stderr, file=sys.stderr)
            return 1

        for name, mutate in CASES.items():
            value = copy.deepcopy(BASELINE)
            mutate(value)
            path = root / f"{name}.json"
            path.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")
            if invoke(path).returncode == 0:
                print(f"negative fixture unexpectedly passed: {name}", file=sys.stderr)
                return 1

    print(f"TRNM World v4 candidate negative fixtures: PASS ({len(CASES)}/{len(CASES)})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
