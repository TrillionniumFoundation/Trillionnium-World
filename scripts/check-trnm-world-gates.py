#!/usr/bin/env python3
from __future__ import annotations

import datetime as dt
import json
import pathlib
import re
import sys
from typing import Any

ROOT = pathlib.Path(__file__).resolve().parents[1]
REGISTRY = pathlib.Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else ROOT / "docs/status/world-gates-v1.json"

REQUIRED_GATES = {
    "deterministic_runtime_alpha",
    "native_software_alpha",
    "trusted_cex_settlement",
    "closed_online_nakama",
    "public_online",
    "public_player_market",
    "commercial_single_player",
}
ALLOWED_STATUSES = {
    "planned",
    "implemented",
    "verified_local",
    "verified_remote",
    "deployed",
    "operational",
    "release_ready",
    "blocked",
    "no_go",
    "disabled",
}
PROMOTED_STATUSES = {"verified_remote", "deployed", "operational", "release_ready"}
SHA40 = re.compile(r"^[0-9a-f]{40}$")
SHA256 = re.compile(r"^[0-9a-f]{64}$")
WORKFLOW_URL = re.compile(r"^https://github\.com/.+/actions/runs/[0-9]+(?:/.*)?$")


def fail(errors: list[str]) -> None:
    if errors:
        print("TRNM World gate registry validation failed:", file=sys.stderr)
        for error in errors:
            print(f" - {error}", file=sys.stderr)
        raise SystemExit(1)


def nonempty_strings(value: Any) -> bool:
    return isinstance(value, list) and bool(value) and all(isinstance(item, str) and item.strip() for item in value)


def main() -> None:
    errors: list[str] = []
    try:
        data = json.loads(REGISTRY.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise SystemExit(f"cannot read gate registry {REGISTRY}: {error}")

    if data.get("schema_version") != 1:
        errors.append("schema_version must be 1")
    if data.get("project_id") != "trillionnium-world":
        errors.append("project_id must be trillionnium-world")
    try:
        dt.date.fromisoformat(data.get("as_of", ""))
    except ValueError:
        errors.append("as_of must be an ISO date")

    source_plan = data.get("source_plan")
    if not isinstance(source_plan, str) or not source_plan:
        errors.append("source_plan is required")
    elif REGISTRY == ROOT / "docs/status/world-gates-v1.json" and not (ROOT / source_plan).is_file():
        errors.append(f"source_plan does not exist: {source_plan}")

    gates = data.get("gates")
    if not isinstance(gates, list):
        errors.append("gates must be an array")
        fail(errors)
        return

    by_id: dict[str, dict[str, Any]] = {}
    for index, gate in enumerate(gates):
        if not isinstance(gate, dict):
            errors.append(f"gates[{index}] must be an object")
            continue
        gate_id = gate.get("id")
        if not isinstance(gate_id, str) or not re.fullmatch(r"[a-z][a-z0-9_]*", gate_id):
            errors.append(f"gates[{index}].id is invalid")
            continue
        if gate_id in by_id:
            errors.append(f"duplicate gate id: {gate_id}")
            continue
        by_id[gate_id] = gate

        status = gate.get("status")
        if status not in ALLOWED_STATUSES:
            errors.append(f"{gate_id}: invalid status {status!r}")
        if not isinstance(gate.get("authority_profile"), str) or not gate["authority_profile"].strip():
            errors.append(f"{gate_id}: authority_profile is required")
        if not isinstance(gate.get("blockers"), list) or not all(
            isinstance(item, str) and item.strip() for item in gate.get("blockers", [])
        ):
            errors.append(f"{gate_id}: blockers must be an array of non-empty strings")
        if not nonempty_strings(gate.get("limitations")):
            errors.append(f"{gate_id}: at least one explicit limitation is required")

        evidence = gate.get("evidence")
        if not isinstance(evidence, list):
            errors.append(f"{gate_id}: evidence must be an array")
            evidence = []
        passed_remote = 0
        for evidence_index, item in enumerate(evidence):
            prefix = f"{gate_id}.evidence[{evidence_index}]"
            if not isinstance(item, dict):
                errors.append(f"{prefix}: must be an object")
                continue
            if not isinstance(item.get("scope"), str) or not item["scope"].strip():
                errors.append(f"{prefix}: scope is required")
            if not SHA40.fullmatch(str(item.get("commit_sha", ""))):
                errors.append(f"{prefix}: commit_sha must be 40 lowercase hex characters")
            if not SHA40.fullmatch(str(item.get("tree_sha", ""))):
                errors.append(f"{prefix}: tree_sha must be 40 lowercase hex characters")
            artifact = item.get("artifact_sha256")
            if artifact is not None and not SHA256.fullmatch(str(artifact)):
                errors.append(f"{prefix}: artifact_sha256 must be 64 lowercase hex characters")
            if not WORKFLOW_URL.fullmatch(str(item.get("workflow_url", ""))):
                errors.append(f"{prefix}: workflow_url must be an exact GitHub Actions run URL")
            result = item.get("result")
            if result not in {"passed", "failed", "invalid"}:
                errors.append(f"{prefix}: result must be passed, failed or invalid")
            if not nonempty_strings(item.get("limitations")):
                errors.append(f"{prefix}: evidence limitations are required")
            for date_key in ("reviewed_at", "review_due"):
                if date_key in item:
                    try:
                        dt.date.fromisoformat(str(item[date_key]))
                    except ValueError:
                        errors.append(f"{prefix}: {date_key} must be an ISO date")
            if result == "passed" and WORKFLOW_URL.fullmatch(str(item.get("workflow_url", ""))):
                passed_remote += 1

        if status in PROMOTED_STATUSES and passed_remote == 0:
            errors.append(f"{gate_id}: promoted status {status} requires passed remote evidence")
        if status in {"blocked", "no_go", "disabled"} and not gate.get("blockers"):
            errors.append(f"{gate_id}: blocked/no-go/disabled status requires blockers")

    missing = REQUIRED_GATES - set(by_id)
    extra = set(by_id) - REQUIRED_GATES
    if missing:
        errors.append(f"missing required gates: {', '.join(sorted(missing))}")
    if extra:
        errors.append(f"unexpected gates in schema v1: {', '.join(sorted(extra))}")

    # v1 is intentionally fail-closed. Promotion requires a reviewed schema
    # revision that names the complete denominator-specific evidence set.
    if by_id.get("public_online", {}).get("status") != "no_go":
        errors.append("public_online must remain no_go under gate schema v1")
    if by_id.get("public_player_market", {}).get("status") != "disabled":
        errors.append("public_player_market must remain disabled under gate schema v1")
    if by_id.get("closed_online_nakama", {}).get("status") not in {"planned", "implemented", "blocked"}:
        errors.append("closed_online_nakama cannot be remotely promoted under schema v1")
    if by_id.get("trusted_cex_settlement", {}).get("status") not in {"planned", "implemented", "blocked"}:
        errors.append("trusted_cex_settlement cannot be remotely promoted until schema v2 names fault/custody scopes")

    fail(errors)
    print(f"TRNM World gate registry validation passed: {REGISTRY}")


if __name__ == "__main__":
    main()
