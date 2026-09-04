#!/usr/bin/env python3
"""Fail-closed source contract for the protected-main World runner probe."""

from __future__ import annotations

import argparse
from pathlib import Path

DEFAULT_PATH = Path(".github/workflows/self-hosted-desktop-availability.yml")

JOB_PREDICATE = """    if: >-
      github.event_name == 'issue_comment' &&
      github.repository == 'TrillionniumFoundation/Trillionnium-World' &&
      github.event.repository.default_branch == 'main' &&
      github.ref == 'refs/heads/main' &&
      github.ref_protected == true &&
      github.event.issue.number == 55 &&
      github.event.issue.pull_request == null &&
      github.event.comment.body == '/world-desktop-runner-probe' &&
      github.actor == 'ProfHepta'
"""

REQUIRED = (
    "on:\n  issue_comment:\n    types: [created]\n",
    "permissions: {}",
    "concurrency:\n  group: trnm-world-desktop-availability\n  cancel-in-progress: false",
    JOB_PREDICATE,
    "runs-on:\n      group: trillionnium-world-desktop\n      labels: trillionnium-world-desktop",
    "timeout-minutes: 5",
    'test "$GITHUB_EVENT_NAME" = issue_comment',
    'test "$GITHUB_REPOSITORY" = TrillionniumFoundation/Trillionnium-World',
    'test "$GITHUB_REF" = refs/heads/main',
    'test "$GITHUB_REF_PROTECTED" = true',
    'test "$GITHUB_ACTOR" = ProfHepta',
    'test "$RUNNER_NAME" = desktop',
    'test "$RUNNER_OS" = Linux',
    'test "$RUNNER_ARCH" = X64',
    "status=world-desktop-runner-acquired\\ntrigger=issue_comment\\nref=refs/heads/main\\nissue=55\\n",
)

FORBIDDEN = (
    "${{",
    "workflow_dispatch:",
    "repository_dispatch:",
    "pull_request:",
    "pull_request_target:",
    "push:",
    "schedule:",
    "workflow_call:",
    "inputs:",
    "env:",
    "uses:",
    "GITHUB_TOKEN",
    "GITHUB_EVENT_PATH",
    "secrets.",
    "curl ",
    "wget ",
    "git ",
    "ssh ",
    "scp ",
    "adb ",
    "nvidia-smi",
    "uname ",
    "printenv",
    "/proc/",
    "RUNNER_TEMP",
    "$HOME",
    "startsWith(",
    "contains(",
    "fromJSON(",
)


def validate(text: str) -> list[str]:
    failures: list[str] = []
    lines = text.splitlines()
    for fragment in REQUIRED:
        if fragment not in text:
            failures.append(f"required fragment missing: {fragment!r}")
    for fragment in FORBIDDEN:
        if fragment in text:
            failures.append(f"forbidden fragment present: {fragment!r}")

    if text.count("issue_comment:") != 1 or text.count("types: [created]") != 1:
        failures.append("the probe must have one created-comment trigger")
    if text.count("permissions:") != 1 or text.count("permissions: {}") != 1:
        failures.append("token permissions must be exactly empty")
    if text.count("runs-on:") != 1 or text.count("run: |") != 1:
        failures.append("the probe must have one runner boundary and one shell step")
    if text.count("if: >-") != 1:
        failures.append("the probe must have one allocation predicate")

    predicate_lines = JOB_PREDICATE.rstrip("\n").splitlines()
    try:
        predicate_index = lines.index("    if: >-")
        allocation_index = lines.index("    runs-on:")
        shell_index = lines.index("        run: |")
    except ValueError:
        failures.append("predicate, allocation or shell boundary is absent")
    else:
        if lines[predicate_index : predicate_index + len(predicate_lines)] != predicate_lines:
            failures.append("job-level allocation predicate must match exactly")
        if not predicate_index < allocation_index < shell_index:
            failures.append("authorization must precede runner allocation and shell execution")

    return failures


def require_rejected(name: str, candidate: str) -> None:
    if not validate(candidate):
        raise AssertionError(f"hostile fixture unexpectedly accepted: {name}")


def self_test(baseline: str) -> None:
    failures = validate(baseline)
    if failures:
        raise AssertionError("baseline failed: " + "; ".join(failures))

    hostile = {
        "selected-ref-dispatch": baseline.replace(
            "  issue_comment:\n    types: [created]\n", "  workflow_dispatch:\n", 1
        ),
        "edited-comment": baseline.replace("types: [created]", "types: [created, edited]", 1),
        "missing-main": baseline.replace(
            "github.ref == 'refs/heads/main'", "github.ref != 'refs/heads/main'", 1
        ),
        "unprotected-ref": baseline.replace(
            "github.ref_protected == true", "github.ref_protected == false", 1
        ),
        "wrong-default": baseline.replace(
            "github.event.repository.default_branch == 'main'",
            "github.event.repository.default_branch != 'main'",
            1,
        ),
        "prefix-command": baseline.replace(
            "github.event.comment.body == '/world-desktop-runner-probe'",
            "startsWith(github.event.comment.body, '/world-desktop-runner-probe')",
            1,
        ),
        "any-actor": baseline.replace("github.actor == 'ProfHepta'", "github.actor != ''", 1),
        "wrong-issue": baseline.replace(
            "github.event.issue.number == 55", "github.event.issue.number > 0", 1
        ),
        "pr-comment": baseline.replace(
            "      github.event.issue.pull_request == null &&\n", "", 1
        ),
        "step-level-if": baseline.replace("    if: >-\n", "      if: >-\n", 1),
        "broad-group": baseline.replace(
            "group: trillionnium-world-desktop", "group: desktop", 1
        ),
        "broad-label": baseline.replace(
            "labels: trillionnium-world-desktop", "labels: desktop", 1
        ),
        "expression-in-shell": baseline.replace(
            "printf 'status=world-desktop-runner-acquired\\ntrigger=issue_comment\\nref=refs/heads/main\\nissue=55\\n'",
            "printf '%s\\n' '${{ github.event.comment.body }}'",
            1,
        ),
        "checkout": baseline.replace(
            "    steps:\n", "    steps:\n      - uses: actions/checkout@v6\n", 1
        ),
        "host-inventory": baseline.replace(
            "          printf 'status=world-desktop-runner-acquired",
            "          uname -a\n          printf 'status=world-desktop-runner-acquired",
            1,
        ),
        "network": baseline.replace(
            "          printf 'status=world-desktop-runner-acquired",
            "          curl https://example.invalid\n          printf 'status=world-desktop-runner-acquired",
            1,
        ),
        "token": baseline.replace("permissions: {}", "permissions:\n  contents: read", 1),
    }
    for name, candidate in hostile.items():
        require_rejected(name, candidate)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--path", type=Path, default=DEFAULT_PATH)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    text = args.path.read_text(encoding="utf-8")
    failures = validate(text)
    if args.self_test:
        self_test(text)
        print("World desktop runner hostile fixtures: PASS")
    if failures:
        for failure in failures:
            print(f"ERROR: {failure}")
        return 1
    print(f"World desktop runner source contract: PASS ({args.path})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
