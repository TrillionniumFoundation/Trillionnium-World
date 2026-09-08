#!/usr/bin/env python3
"""Bounded strict JSON loader shared by current World machine-truth gates."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

DEFAULT_MAX_BYTES = 512 * 1024
DEFAULT_MAX_DEPTH = 64
DEFAULT_MAX_NODES = 50_000


class StrictJsonError(ValueError):
    pass


def _reject_constant(token: str) -> object:
    raise StrictJsonError(f"non-finite JSON token is forbidden: {token}")


def _reject_duplicate_object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise StrictJsonError(f"duplicate JSON object key: {key}")
        result[key] = value
    return result


def _enforce_budget(value: Any, *, max_depth: int, max_nodes: int) -> None:
    stack: list[tuple[Any, int]] = [(value, 1)]
    nodes = 0
    while stack:
        current, depth = stack.pop()
        nodes += 1
        if nodes > max_nodes:
            raise StrictJsonError(f"JSON node budget exceeded: {nodes} > {max_nodes}")
        if depth > max_depth:
            raise StrictJsonError(f"JSON depth budget exceeded: {depth} > {max_depth}")
        if isinstance(current, dict):
            stack.extend((item, depth + 1) for item in current.values())
        elif isinstance(current, list):
            stack.extend((item, depth + 1) for item in current)


def loads_strict(
    text: str,
    *,
    max_depth: int = DEFAULT_MAX_DEPTH,
    max_nodes: int = DEFAULT_MAX_NODES,
) -> Any:
    try:
        value = json.loads(
            text,
            object_pairs_hook=_reject_duplicate_object,
            parse_constant=_reject_constant,
        )
    except json.JSONDecodeError as error:
        raise StrictJsonError(str(error)) from error
    _enforce_budget(value, max_depth=max_depth, max_nodes=max_nodes)
    return value


def load_strict_json(
    path: Path,
    *,
    max_bytes: int = DEFAULT_MAX_BYTES,
    max_depth: int = DEFAULT_MAX_DEPTH,
    max_nodes: int = DEFAULT_MAX_NODES,
) -> Any:
    if path.is_symlink():
        raise StrictJsonError(f"symlink is not an accepted JSON source: {path}")
    if not path.is_file():
        raise StrictJsonError(f"missing JSON source: {path}")
    size = path.stat().st_size
    if size == 0:
        raise StrictJsonError(f"empty JSON source: {path}")
    if size > max_bytes:
        raise StrictJsonError(f"oversized JSON source: {path} ({size} > {max_bytes})")
    raw = path.read_bytes()
    try:
        text = raw.decode("utf-8", errors="strict")
    except UnicodeDecodeError as error:
        raise StrictJsonError(f"invalid UTF-8 in {path}: {error}") from error
    return loads_strict(text, max_depth=max_depth, max_nodes=max_nodes)
