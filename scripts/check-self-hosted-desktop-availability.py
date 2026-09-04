#!/usr/bin/env python3
"""Fail closed on unsafe self-hosted desktop availability workflows."""
from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
WORKFLOW = ROOT / ".github/workflows/self-hosted-desktop-availability.yml"

FORBIDDEN = (
    "${{ inputs.reason }}'",
    "'${{ inputs.reason }}",
    '"${{ inputs.reason }}',
    "uname -a",
    "adb devices",
    "nvidia-smi",
    "actions/checkout",
    "curl ",
    "wget ",
    "ssh ",
    "env |",
    "printenv",
)


def failures(source: str) -> list[str]:
    errors: list[str] = []
    if "workflow_dispatch:" not in source:
        errors.append("probe must be manual workflow_dispatch only")
    for trigger in ("pull_request:", "pull_request_target:", "push:", "issue_comment:"):
        if trigger in source:
            errors.append(f"unapproved trigger: {trigger}")
    if not re.search(r"(?m)^permissions:\s*\{\}\s*$", source):
        errors.append("permissions must be exactly empty")
    for label in ("self-hosted", "linux", "x64", "desktop"):
        if not re.search(rf"(?m)^\s*-\s*{re.escape(label)}\s*$", source):
            errors.append(f"dedicated scheduling label missing: {label}")
    if "REASON: ${{ inputs.reason }}" not in source:
        errors.append("dispatch reason must cross the expression boundary through env")
    if "${{ inputs.reason }}" in source.replace("REASON: ${{ inputs.reason }}", ""):
        errors.append("dispatch reason is interpolated into executable source")
    if "${#REASON}" not in source or "32" not in source:
        errors.append("reason length is not bounded")
    if not any(marker in source for marker in ("[A-Za-z0-9._-]", "[:alnum:]")):
        errors.append("reason allowlist is absent")
    if 'printf \'runner=%s\\nos=%s\\narch=%s\\nref=%s\\nreason=%s\\n\'' not in source:
        errors.append("constant-format receipt is absent")
    for marker in FORBIDDEN:
        if marker in source:
            errors.append(f"forbidden probe surface: {marker}")
    if re.search(r"(?m)^\s*uses:\s*", source):
        errors.append("availability probe may not invoke actions or reusable workflows")
    return errors


def hostile_self_test() -> list[str]:
    base = """name: x
on:
  workflow_dispatch:
permissions: {}
jobs:
  probe:
    runs-on:
      - self-hosted
      - linux
      - x64
      - desktop
    steps:
      - env:
          REASON: ${{ inputs.reason }}
        run: |
          test ${#REASON} -le 32
          [[ $REASON =~ ^[A-Za-z0-9._-]*$ ]]
          printf 'runner=%s\\nos=%s\\narch=%s\\nref=%s\\nreason=%s\\n' "$RUNNER_NAME" "$RUNNER_OS" "$RUNNER_ARCH" "$GITHUB_REF" "$REASON"
"""
    mutations = {
        "shell interpolation": base.replace('"$REASON"', "'${{ inputs.reason }}'"),
        "generic runner": base.replace("      - desktop\n", ""),
        "token permission": base.replace("permissions: {}", "permissions:\n  contents: read"),
        "inventory": base.replace("          printf", "          uname -a\n          printf"),
        "pull request": base.replace("  workflow_dispatch:", "  pull_request:"),
    }
    missed = [name for name, source in mutations.items() if not failures(source)]
    return missed


def main() -> int:
    if not WORKFLOW.is_file() or WORKFLOW.is_symlink():
        print("desktop availability contract failed: workflow missing or unsafe", file=sys.stderr)
        return 1
    source = WORKFLOW.read_text(encoding="utf-8")
    errors = failures(source)
    missed = hostile_self_test()
    errors.extend(f"hostile self-test missed {name}" for name in missed)
    if errors:
        print("desktop availability contract failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1
    print("desktop availability contract: OK (manual, empty permissions, dedicated route, data-only bounded input, no inventory)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
