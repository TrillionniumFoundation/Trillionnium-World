#!/usr/bin/env python3
from __future__ import annotations

import copy
import json
import pathlib
import subprocess
import sys
import tempfile

ROOT = pathlib.Path(__file__).resolve().parents[1]
CHECKER = ROOT / "scripts/check-trnm-world-gates.py"
SOURCE = ROOT / "docs/status/world-gates-v1.json"
BASE = json.loads(SOURCE.read_text(encoding="utf-8"))


def run_fixture(label: str, value: dict, expect_success: bool = False) -> None:
    with tempfile.TemporaryDirectory(prefix="trnm-world-gate-") as directory:
        path = pathlib.Path(directory) / "gates.json"
        path.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")
        result = subprocess.run(
            [sys.executable, str(CHECKER), str(path)],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            check=False,
        )
        if (result.returncode == 0) != expect_success:
            print(f"fixture {label!r} had unexpected result {result.returncode}", file=sys.stderr)
            print(result.stdout, file=sys.stderr)
            print(result.stderr, file=sys.stderr)
            raise SystemExit(1)


run_fixture("untampered registry", copy.deepcopy(BASE), expect_success=True)

public_online = copy.deepcopy(BASE)
next(gate for gate in public_online["gates"] if gate["id"] == "public_online")["status"] = "release_ready"
run_fixture("unsupported public-online promotion", public_online)

public_market = copy.deepcopy(BASE)
next(gate for gate in public_market["gates"] if gate["id"] == "public_player_market")["status"] = "implemented"
run_fixture("unsupported public-market enablement", public_market)

missing_limitations = copy.deepcopy(BASE)
next(gate for gate in missing_limitations["gates"] if gate["id"] == "native_software_alpha")["limitations"] = []
run_fixture("missing limitations", missing_limitations)

duplicate = copy.deepcopy(BASE)
duplicate["gates"].append(copy.deepcopy(duplicate["gates"][0]))
run_fixture("duplicate gate", duplicate)

fake_remote = copy.deepcopy(BASE)
gate = next(gate for gate in fake_remote["gates"] if gate["id"] == "deterministic_runtime_alpha")
gate["status"] = "verified_remote"
gate["evidence"] = [
    {
        "scope": "fake",
        "commit_sha": "abc",
        "tree_sha": "def",
        "workflow_url": "file:///tmp/passed.txt",
        "result": "passed",
        "limitations": ["fake"],
        "reviewed_at": "2026-08-27"
    }
]
run_fixture("fake remote evidence", fake_remote)

print("TRNM World gate-registry negative fixtures passed")
