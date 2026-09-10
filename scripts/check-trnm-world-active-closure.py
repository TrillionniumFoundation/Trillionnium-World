#!/usr/bin/env python3
"""Validate the current World closure without stale Markdown identity duplication."""

from __future__ import annotations

import importlib.util
from pathlib import Path

_BASE_PATH = (
    Path(__file__).with_name("archive")
    / "check-trnm-world-active-closure-v6-20260908-base.py"
)
_SPEC = importlib.util.spec_from_file_location(
    "trnm_world_active_closure_v6_base",
    _BASE_PATH,
)
if _SPEC is None or _SPEC.loader is None:
    raise RuntimeError(f"cannot load archived active-closure checker: {_BASE_PATH}")
_BASE = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(_BASE)

# Re-export the established checker API. The current wrapper deliberately keeps
# the archived validator as the broad negative surface and overlays only facts
# that moved after the archived snapshot.
for _name in dir(_BASE):
    if not _name.startswith("__"):
        globals()[_name] = getattr(_BASE, _name)

CURRENT_CEX = {
    "repository": "TrillionniumFoundation/CEX",
    "pull_request": 53,
    "branch": "integration/cex-v12-sequence54-20260908",
    "commit": "882014451c22026dfe4848248f2b26a9b46ac049",
    "tree": "cb5b69f79eeb4e6d2032b93115d3e25c88957903",
    "base": "db75c74094748a1139fd64b0360e122d8ec797a0",
    "prospective_merge": "7a93ca40eb9278b4da7dc64b9f773acd639b9410",
    "sequence": 54,
}
_BASE.CURRENT_CEX = CURRENT_CEX

_SUPERSEDED_CEX_IDENTITIES = (
    "551b9b85d23393ce22f7f2358e1dfc28b8879f7d",
    "c4a855e9ef85d627306dfeeb746b9ed1a442e5ef",
    "95f08146f751fa2d413d2a0102ebdf9f8ab7434a",
    "6a14d0da4cf1aefd4689708a6967a4ae1c0e84fc",
    "e425423fda89ce5438879518367f4b12ad2ef3f5",
    "53676ad0ea2712b5efa2bf2a49359e00035521c4",
    "0dfbba47c265e70a8880aa6f7d0279473c7d4268",
    "a5642ee4d5d3eaa1d066818a76ed1f27c060a2f8",
    "301314c61c127f9febc48d57358b9ba73640b74e",
    "c09cf4783f878e909ca8b47bf83ade4f7607136d",
)
_BASE_REJECT_CURRENT_STALE_IDENTITY = _BASE.reject_current_stale_identity


def reject_current_stale_identity(relative: str, text: str) -> None:
    _BASE_REJECT_CURRENT_STALE_IDENTITY(relative, text)
    for marker in _SUPERSEDED_CEX_IDENTITIES:
        _BASE.require(
            marker not in text,
            f"{relative} contains superseded CEX identity: {marker}",
        )


_BASE.reject_current_stale_identity = reject_current_stale_identity

# The archived validator required the selected CEX PR and Git SHAs to be copied
# into both CURRENT_PLAN.md and RELEASE_READINESS.md. The current plan explicitly
# removed that self-invalidating duplication: exact external identities now live
# in versioned machine records and are still checked below by the archived
# ledger/lock comparisons. Suppress only those obsolete presentation assertions;
# every structural, exact-object and no-overclaim assertion remains active.
_BASE_REQUIRE = _BASE.require
_OBSOLETE_MARKDOWN_IDENTITY_FAILURES = {
    "release truth missing marker: CEX PR #53",
    f"release truth missing marker: {CURRENT_CEX['commit']}",
    f"release truth missing marker: {CURRENT_CEX['tree']}",
    "CURRENT_PLAN.md missing current marker: CEX PR #53",
    f"CURRENT_PLAN.md missing current marker: {CURRENT_CEX['commit']}",
    f"CURRENT_PLAN.md missing current marker: {CURRENT_CEX['tree']}",
}


def require(condition: bool, message: str) -> None:
    if not condition and message in _OBSOLETE_MARKDOWN_IDENTITY_FAILURES:
        return
    _BASE_REQUIRE(condition, message)


_BASE.require = require
_BASE_VALIDATE = _BASE.validate


def validate(root: Path) -> None:
    root = root.resolve()
    _BASE_VALIDATE(root)

    plan = _BASE.read_text(root, _BASE.PLAN)
    release = _BASE.read_text(root, _BASE.RELEASE)

    for marker in (
        "Tracked source cannot certify its own current Git head",
        "Exact external candidate identities are deliberately not duplicated in this tracked plan.",
        _BASE.CEX_LOCK,
        _BASE.LEDGER,
        _BASE.RELEASE,
        "Production authorization remains **not granted**",
    ):
        _BASE_REQUIRE(marker in plan, f"CURRENT_PLAN.md missing live-object rule marker: {marker}")

    for marker in (
        "CEX trusted settlement/custody",
        "blocked_upstream",
        "all_repository_gaps_closed=false",
        "all_plan_gaps_closed=false",
        "production_ready=false",
        "production_authorization=not_granted",
    ):
        _BASE_REQUIRE(marker in release, f"release truth missing current generic marker: {marker}")

    # Exact external values remain mandatory in the machine lock and ledger, but
    # must not be reintroduced into the tracked plan where they become stale as
    # soon as an accountable external repository moves.
    for marker in (
        "CEX PR #53",
        CURRENT_CEX["commit"],
        CURRENT_CEX["tree"],
        CURRENT_CEX["prospective_merge"],
    ):
        _BASE_REQUIRE(
            marker not in plan,
            f"CURRENT_PLAN.md duplicates external candidate identity: {marker}",
        )


_BASE.validate = validate

# The archived module lives one directory deeper than the current entrypoint.
# Point its CLI default-root calculation at this wrapper so parents[1] resolves
# to the repository root without rewriting the retained archive snapshot.
_BASE.__file__ = str(Path(__file__).resolve())
main = _BASE.main

if __name__ == "__main__":
    raise SystemExit(main())
