#!/usr/bin/env python3
"""Fail closed on Rust include! seams while tracking completed module migrations."""

from __future__ import annotations

import argparse
import importlib.util
import json
import os
from pathlib import Path
import sys
import tempfile
from typing import Any

ENGINE_PATH = Path(__file__).with_name("_trnm_world_include_boundary_engine.py")
SPEC = importlib.util.spec_from_file_location(
    "trnm_world_include_boundary_engine", ENGINE_PATH
)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError(f"cannot load include-boundary engine: {ENGINE_PATH}")
ENGINE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ENGINE)

INVENTORY = ENGINE.INVENTORY
MIGRATIONS = "scripts/contracts/trnm-world-include-migrations-v1.json"
IncludeBoundaryFailure = ENGINE.IncludeBoundaryFailure

LEDGER_ROOT_KEYS = {
    "schema",
    "as_of",
    "inventory_baseline_commit",
    "completed",
}
LEDGER_ENTRY_KEYS = {
    "path",
    "expression",
    "replacement_module",
    "required_fragments",
    "completed_commit",
}


def _strict_object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise IncludeBoundaryFailure(f"duplicate migration-ledger JSON key: {key}")
        result[key] = value
    return result


def _reject_constant(value: str) -> None:
    raise IncludeBoundaryFailure(
        f"non-finite migration-ledger JSON constant is forbidden: {value}"
    )


def _load_ledger(root: Path, relative: str) -> dict[str, Any]:
    path = ENGINE.checked_file(root, relative, "migration ledger path")
    data = path.read_bytes()
    if len(data) > ENGINE.MAX_INVENTORY_BYTES:
        raise IncludeBoundaryFailure("migration ledger exceeds the byte budget")
    try:
        value = json.loads(
            data.decode("utf-8", errors="strict"),
            object_pairs_hook=_strict_object,
            parse_constant=_reject_constant,
        )
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise IncludeBoundaryFailure(
            f"invalid strict migration-ledger JSON: {error}"
        ) from error
    if not isinstance(value, dict) or set(value) != LEDGER_ROOT_KEYS:
        raise IncludeBoundaryFailure("migration ledger root key set drift")
    if value.get("schema") != "trnm_world_include_migration_ledger_v1":
        raise IncludeBoundaryFailure("migration ledger schema drift")
    if not isinstance(value.get("as_of"), str) or not value["as_of"].strip():
        raise IncludeBoundaryFailure("migration ledger as_of must be nonempty")
    if not ENGINE.HEX40.fullmatch(
        str(value.get("inventory_baseline_commit", ""))
    ):
        raise IncludeBoundaryFailure(
            "migration ledger inventory_baseline_commit is not 40 lowercase hex"
        )
    completed = value.get("completed")
    if not isinstance(completed, list):
        raise IncludeBoundaryFailure("migration ledger completed must be a list")
    return value


def _validated_completed_entries(
    root: Path,
    inventory: dict[str, Any],
    ledger: dict[str, Any],
) -> dict[tuple[str, str], dict[str, Any]]:
    if ledger["inventory_baseline_commit"] != inventory["baseline_commit"]:
        raise IncludeBoundaryFailure(
            "migration ledger is not bound to the inventory baseline_commit"
        )

    inventory_entries: dict[tuple[str, str], dict[str, str]] = {}
    for raw in inventory["entries"]:
        if not isinstance(raw, dict):
            raise IncludeBoundaryFailure("inventory entry is not an object")
        key = (
            raw.get("path", ""),
            ENGINE.normalize_expression(raw.get("expression", "")),
        )
        if key in inventory_entries:
            raise IncludeBoundaryFailure(f"duplicate include inventory entry: {key}")
        inventory_entries[key] = raw

    completed: dict[tuple[str, str], dict[str, Any]] = {}
    for raw in ledger["completed"]:
        if not isinstance(raw, dict) or set(raw) != LEDGER_ENTRY_KEYS:
            raise IncludeBoundaryFailure("migration ledger entry key set drift")
        for field in (
            "path",
            "expression",
            "replacement_module",
            "completed_commit",
        ):
            if not isinstance(raw[field], str) or not raw[field].strip():
                raise IncludeBoundaryFailure(
                    f"migration ledger entry has invalid {field}"
                )
        source_relative = raw["path"]
        ENGINE.safe_relative(source_relative, "completed include source path")
        expression = ENGINE.normalize_expression(raw["expression"])
        module = raw["replacement_module"]
        if not module.replace("_", "").isalnum() or not module[0].isalpha():
            raise IncludeBoundaryFailure(
                f"{source_relative}: invalid replacement module name {module!r}"
            )
        if not ENGINE.HEX40.fullmatch(raw["completed_commit"]):
            raise IncludeBoundaryFailure(
                f"{source_relative}: completed_commit is not 40 lowercase hex"
            )
        fragments = raw["required_fragments"]
        if (
            not isinstance(fragments, list)
            or not fragments
            or any(
                not isinstance(fragment, str)
                or not fragment.strip()
                or fragment != fragment.strip()
                for fragment in fragments
            )
        ):
            raise IncludeBoundaryFailure(
                f"{source_relative}: required_fragments must be nonempty strings"
            )
        key = (source_relative, expression)
        if key in completed:
            raise IncludeBoundaryFailure(
                f"duplicate completed include migration entry: {key}"
            )
        baseline = inventory_entries.get(key)
        if baseline is None:
            raise IncludeBoundaryFailure(
                f"completed migration is absent from baseline inventory: {key}"
            )
        if baseline["class"] not in {"handwritten-runtime", "test-only"}:
            raise IncludeBoundaryFailure(
                f"{source_relative}: only active handwritten seams may be completed"
            )
        if baseline["migration_state"] != "migrate":
            raise IncludeBoundaryFailure(
                f"{source_relative}: completed seam was not baseline-marked migrate"
            )

        ENGINE.validate_target(root, source_relative, expression)
        source = ENGINE.checked_file(root, source_relative, "completed include source")
        data = source.read_bytes()
        if len(data) > ENGINE.MAX_SOURCE_BYTES:
            raise IncludeBoundaryFailure(
                f"completed include source exceeds byte budget: {source_relative}"
            )
        try:
            text = data.decode("utf-8", errors="strict")
        except UnicodeDecodeError as error:
            raise IncludeBoundaryFailure(
                f"completed include source is not strict UTF-8: {source_relative}"
            ) from error
        for fragment in fragments:
            if fragment not in text:
                raise IncludeBoundaryFailure(
                    f"{source_relative}: replacement fragment missing for "
                    f"{module}: {fragment!r}"
                )
        completed[key] = raw
    return completed


def _validate_with_progress(
    root: Path,
    inventory_relative: str = INVENTORY,
    migrations_relative: str = MIGRATIONS,
) -> tuple[int, int, int]:
    root = root.resolve(strict=True)
    ledger_path = root / migrations_relative
    if not ledger_path.exists():
        active, frozen = ENGINE.validate(root, inventory_relative)
        return active, frozen, 0

    inventory = ENGINE.load_inventory(root, inventory_relative)
    ledger = _load_ledger(root, migrations_relative)
    completed = _validated_completed_entries(root, inventory, ledger)
    observed = ENGINE.scan_includes(root)
    reappeared = sorted(set(completed) & set(observed))
    if reappeared:
        raise IncludeBoundaryFailure(
            f"completed include seam reappeared in source: {reappeared}"
        )

    effective = dict(inventory)
    effective["entries"] = [
        raw
        for raw in inventory["entries"]
        if (
            raw["path"],
            ENGINE.normalize_expression(raw["expression"]),
        )
        not in completed
    ]

    runtime_dir = root / "run" / "world-include-boundaries"
    runtime_dir.mkdir(parents=True, exist_ok=True)
    descriptor, temporary_name = tempfile.mkstemp(
        prefix="effective-inventory-",
        suffix=".json",
        dir=runtime_dir,
        text=True,
    )
    temporary = Path(temporary_name)
    try:
        with os.fdopen(descriptor, "w", encoding="utf-8", newline="\n") as handle:
            json.dump(
                effective,
                handle,
                ensure_ascii=False,
                sort_keys=True,
                separators=(",", ":"),
            )
            handle.write("\n")
        relative = temporary.relative_to(root).as_posix()
        active, frozen = ENGINE.validate(root, relative)
    finally:
        temporary.unlink(missing_ok=True)
    return active, frozen, len(completed)


def validate(
    root: Path,
    inventory_relative: str = INVENTORY,
    migrations_relative: str = MIGRATIONS,
) -> tuple[int, int]:
    active, frozen, _completed = _validate_with_progress(
        root, inventory_relative, migrations_relative
    )
    return active, frozen


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", default=str(Path(__file__).resolve().parents[1]))
    parser.add_argument("--inventory", default=INVENTORY)
    parser.add_argument("--migrations", default=MIGRATIONS)
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(sys.argv[1:] if argv is None else argv)
    try:
        active, frozen, completed = _validate_with_progress(
            Path(args.root), args.inventory, args.migrations
        )
    except (IncludeBoundaryFailure, OSError, ValueError) as error:
        print(f"TRNM World include boundary: FAIL: {error}", file=sys.stderr)
        return 1
    print(
        "TRNM World include boundary: PASS "
        f"(active_migration={active}, completed_migration={completed}, "
        f"frozen_legacy={frozen})"
    )
    print("Classification and migration progress do not grant production authorization.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
