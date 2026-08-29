#!/usr/bin/env python3
"""Fail closed until v4 exact-head checks and review evidence are registered."""

from __future__ import annotations

import argparse
import json
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parents[1]
DEFAULT_STATUS = ROOT / "docs/status/v4-candidate-v1.json"
DEFAULT_SCHEMA = ROOT / "docs/status/v4-candidate-v1.schema.json"
SHA = re.compile(r"^[0-9a-f]{40}$")
REQUIRED = {
    "trnm-world-v4/docs-governance",
    "trnm-world-v4/transition-contract",
    "trnm-world-v4/settlement-postgres",
    "trnm-world-v4/game-workspace-release",
    "trnm-world-v4/supply-chain",
}


def fail(message: str) -> None:
    raise SystemExit(f"TRNM World v4 candidate: FAIL: {message}")


def load(path: pathlib.Path) -> dict:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        fail(f"cannot parse {path}: {error}")
    if not isinstance(value, dict):
        fail(f"{path} must contain one object")
    return value


def validate(value: dict, schema: dict) -> None:
    if schema.get("additionalProperties") is not False:
        fail("candidate schema must be closed")
    if value.get("schema") != "trnm_world_v4_candidate_status_v1":
        fail("wrong status schema")
    if value.get("candidate_branch") != "fix/world-plan-gap-closure-v4":
        fail("wrong candidate branch")
    if value.get("base_branch") != "fix/world-settlement-gap-closure-v1":
        fail("wrong base branch")
    if value.get("base_commit") != "1d4dee6d5add45a64f5c138f424e3bdab369ecd4":
        fail("wrong base commit")
    if value.get("status") != "implemented_pending_exact_head_ci":
        fail("source must not self-promote before workflow/review evidence")
    if value.get("candidate_commit") is not None or value.get("candidate_tree") is not None:
        fail("source status invented a future exact candidate identity")
    if set(value.get("required_checks", [])) != REQUIRED:
        fail("required exact check registry drift")
    for field in ("workflow_runs", "artifacts", "reviewers"):
        if value.get(field) != []:
            fail(f"source status invented future {field}")
    if value.get("release_effect") != "none":
        fail("candidate status granted release effect")
    if value.get("public_online") != "no_go":
        fail("candidate status enabled public online")
    if value.get("public_player_market") != "disabled":
        fail("candidate status enabled public market")
    if not isinstance(value.get("limitations"), list) or not value["limitations"]:
        fail("limitations are missing")
    if SHA.fullmatch(value["base_commit"]) is None:
        fail("base commit is not an exact SHA")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--status", type=pathlib.Path, default=DEFAULT_STATUS)
    parser.add_argument("--schema", type=pathlib.Path, default=DEFAULT_SCHEMA)
    args = parser.parse_args()
    validate(load(args.status), load(args.schema))
    print("TRNM World v4 candidate source status: PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
