#!/usr/bin/env python3
"""Apply and verify World GitHub Actions and main-branch controls.

The script is intentionally limited to repository Actions policy and classic
branch protection. It cannot push source, merge pull requests, write statuses,
create tags/releases/deployments, or bypass review. Verification is mandatory
after every apply; an organization-level override or partial write fails closed.
"""

from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import sys
import urllib.error
import urllib.request

REPOSITORY = "TrillionniumFoundation/Trillionnium-World"
BRANCH = "main"
CONTEXTS = [
    "trnm-world-v5/closure-contract",
    "trnm-world-v4/docs-governance",
    "trnm-world-v4/transition-contract",
    "trnm-world-v4/settlement-postgres",
    "trnm-world-v4/game-workspace-release",
    "trnm-world-v4/supply-chain",
    "trnm-world-v4/qualified-source-exact-head",
    "trnm-world-v4/prospective-merge",
]


class ControlFailure(RuntimeError):
    pass


def api(method: str, path: str, token: str, payload: dict | None = None) -> dict:
    body = None if payload is None else json.dumps(payload, separators=(",", ":")).encode()
    request = urllib.request.Request(
        "https://api.github.com" + path,
        data=body,
        method=method,
        headers={
            "Accept": "application/vnd.github+json",
            "Authorization": f"Bearer {token}",
            "User-Agent": "trnm-world-repository-controls/v1",
            "X-GitHub-Api-Version": "2022-11-28",
        },
    )
    try:
        with urllib.request.urlopen(request, timeout=120) as response:
            raw = response.read()
            return json.loads(raw) if raw else {}
    except urllib.error.HTTPError as error:
        detail = error.read().decode("utf-8", "replace")
        raise ControlFailure(f"GitHub {method} {path} failed {error.code}: {detail[:4000]}") from error


def apply_actions(token: str) -> None:
    api(
        "PUT",
        f"/repos/{REPOSITORY}/actions/permissions",
        token,
        {"enabled": True, "allowed_actions": "selected"},
    )
    api(
        "PUT",
        f"/repos/{REPOSITORY}/actions/permissions/selected-actions",
        token,
        {
            "github_owned_allowed": True,
            "verified_allowed": False,
            "patterns_allowed": [],
        },
    )


def verify_actions(token: str) -> dict:
    policy = api("GET", f"/repos/{REPOSITORY}/actions/permissions", token)
    selected = api("GET", f"/repos/{REPOSITORY}/actions/permissions/selected-actions", token)
    if policy.get("enabled") is not True:
        raise ControlFailure("Actions remains disabled after read-back")
    if policy.get("allowed_actions") != "selected":
        raise ControlFailure(f"unexpected allowed_actions: {policy.get('allowed_actions')}")
    if selected.get("github_owned_allowed") is not True:
        raise ControlFailure("GitHub-owned actions are not allowed")
    if selected.get("verified_allowed") is not False:
        raise ControlFailure("verified third-party actions were unexpectedly enabled")
    if selected.get("patterns_allowed") not in ([], None):
        raise ControlFailure("unexpected third-party action allowlist")
    return {"policy": policy, "selected_actions": selected}


def protection_payload() -> dict:
    return {
        "required_status_checks": {"strict": True, "contexts": CONTEXTS},
        "enforce_admins": True,
        "required_pull_request_reviews": {
            "dismiss_stale_reviews": True,
            "require_code_owner_reviews": True,
            "required_approving_review_count": 1,
            "require_last_push_approval": True,
        },
        "restrictions": None,
        "required_linear_history": True,
        "allow_force_pushes": False,
        "allow_deletions": False,
        "required_conversation_resolution": True,
    }


def apply_protection(token: str) -> None:
    api("PUT", f"/repos/{REPOSITORY}/branches/{BRANCH}/protection", token, protection_payload())


def enabled(value: object) -> bool:
    return isinstance(value, dict) and value.get("enabled") is True


def verify_protection(token: str) -> dict:
    branch = api("GET", f"/repos/{REPOSITORY}/branches/{BRANCH}", token)
    protection = api("GET", f"/repos/{REPOSITORY}/branches/{BRANCH}/protection", token)
    if branch.get("protected") is not True:
        raise ControlFailure("main does not report protected")
    checks = protection.get("required_status_checks") or {}
    if checks.get("strict") is not True:
        raise ControlFailure("required status checks are not strict")
    observed_contexts = set(checks.get("contexts") or [])
    observed_contexts.update(item.get("context") for item in checks.get("checks") or [] if item.get("context"))
    if observed_contexts != set(CONTEXTS):
        raise ControlFailure(f"required context drift: {sorted(observed_contexts)}")
    if not enabled(protection.get("enforce_admins")):
        raise ControlFailure("administrators are not enforced")
    reviews = protection.get("required_pull_request_reviews") or {}
    for field, expected in {
        "dismiss_stale_reviews": True,
        "require_code_owner_reviews": True,
        "required_approving_review_count": 1,
        "require_last_push_approval": True,
    }.items():
        if reviews.get(field) != expected:
            raise ControlFailure(f"review control drift: {field}={reviews.get(field)!r}")
    for field in ("required_linear_history", "required_conversation_resolution"):
        if not enabled(protection.get(field)):
            raise ControlFailure(f"{field} is not enabled")
    for field in ("allow_force_pushes", "allow_deletions"):
        value = protection.get(field)
        if isinstance(value, dict) and value.get("enabled") is not False:
            raise ControlFailure(f"{field} is enabled")
    return {"branch": branch, "protection": protection}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--apply-actions", action="store_true")
    parser.add_argument("--apply-protection", action="store_true")
    parser.add_argument("--verify-only", action="store_true")
    parser.add_argument("--result", type=Path, default=Path("world-repository-controls-result.json"))
    args = parser.parse_args()
    if not (args.apply_actions or args.apply_protection or args.verify_only):
        raise ControlFailure("select --apply-actions, --apply-protection or --verify-only")
    token = os.environ.get("TRNM_WORLD_ADMIN_TOKEN", "")
    if not token:
        raise ControlFailure("TRNM_WORLD_ADMIN_TOKEN is required")

    if args.apply_actions:
        apply_actions(token)
    if args.apply_protection:
        apply_protection(token)

    actions = verify_actions(token)
    protection = verify_protection(token)
    result = {
        "schema": "trnm_world_repository_controls_result_v1",
        "repository": REPOSITORY,
        "branch": BRANCH,
        "actions": "enabled_selected_github_owned_only",
        "main_protection": "verified",
        "required_contexts": CONTEXTS,
        "source_write": False,
        "merge": False,
        "status_write": False,
        "tag_release_deployment": False,
        "readback": {"actions": actions, "main": protection},
    }
    args.result.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print("TRNM_WORLD_REPOSITORY_CONTROLS=PASS")
    return 0


try:
    raise SystemExit(main())
except (ControlFailure, OSError, ValueError, json.JSONDecodeError) as error:
    print(f"repository controls failed closed: {error}", file=sys.stderr)
    raise SystemExit(1)
