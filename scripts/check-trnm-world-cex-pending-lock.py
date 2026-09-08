#!/usr/bin/env python3
"""Current World-to-CEX coordination lock checker for CEX Sequence 54."""

from __future__ import annotations

import importlib.util
from pathlib import Path

_BASE_PATH = (
    Path(__file__).with_name("archive")
    / "check-trnm-world-cex-pending-lock-v2-20260908-base.py"
)
_SPEC = importlib.util.spec_from_file_location(
    "trnm_world_cex_pending_lock_v2_base",
    _BASE_PATH,
)
if _SPEC is None or _SPEC.loader is None:
    raise RuntimeError(f"cannot load archived pending-lock checker: {_BASE_PATH}")
_BASE = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(_BASE)

for _name in dir(_BASE):
    if not _name.startswith("__"):
        globals()[_name] = getattr(_BASE, _name)

EXPECTED_WORLD = {
    "repository": "TrillionniumFoundation/Trillionnium-World",
    "pull_request": 104,
    "branch": "fix/world-convergence-v6-20260908",
    "base": "0f6117a56263bcb5bf89e34b8bcda557a7da2e6d",
    "observed_commit_before_update": "1c69b9a2c5ffdf00ef803ca182a0a75712f6536c",
    "observed_tree_before_update": "f9052a18dfd9f8011b69377cd7984fc20f7482ae",
    "observed_prospective_merge_before_update": "4b8021602cb444cc60520e5b7ab09d64262c7735"
}
EXPECTED_CEX = {
    "repository": "TrillionniumFoundation/CEX",
    "pull_request": 53,
    "branch": "integration/cex-v12-sequence54-20260908",
    "commit": "882014451c22026dfe4848248f2b26a9b46ac049",
    "tree": "cb5b69f79eeb4e6d2032b93115d3e25c88957903",
    "base": "db75c74094748a1139fd64b0360e122d8ec797a0",
    "prospective_merge": "c09cf4783f878e909ca8b47bf83ade4f7607136d",
    "sequence": 54
}
_BASE.EXPECTED_WORLD = EXPECTED_WORLD
_BASE.EXPECTED_CEX = EXPECTED_CEX

validate = _BASE.validate
main = _BASE.main

if __name__ == "__main__":
    raise SystemExit(main())
