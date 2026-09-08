#!/usr/bin/env python3
"""Current World closure checker with exact live CEX Sequence 54 binding."""

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

# Re-export the established checker API so existing hostile fixtures keep
# exercising the same implementation instead of a reduced compatibility shim.
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
    "prospective_merge": "c09cf4783f878e909ca8b47bf83ade4f7607136d",
    "sequence": 54
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
validate = _BASE.validate
main = _BASE.main

if __name__ == "__main__":
    raise SystemExit(main())
