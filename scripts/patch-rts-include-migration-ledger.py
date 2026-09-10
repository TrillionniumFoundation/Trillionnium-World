#!/usr/bin/env python3
"""Infer the ledger's existing completed status and apply it to three RTS seams."""

from __future__ import annotations

from collections import Counter
import json
from pathlib import Path
import sys
from typing import Any

CURRENT = ("rts_tests_01.rs", "rts_tests_02.rs", "rts_tests_03.rs")
COMPLETED_HINTS = (
    "campaign_storage",
    "save_slots",
    "player_settings",
    "rts_mapping",
    "checkpoint_storage",
)
MIGRATE_VALUES = {"migrate", "migration_debt", "pending_migration"}


def walk(value: Any):
    if isinstance(value, dict):
        yield value
        for child in value.values():
            yield from walk(child)
    elif isinstance(value, list):
        for child in value:
            yield from walk(child)


def strings(value: Any) -> list[str]:
    result: list[str] = []
    if isinstance(value, str):
        result.append(value)
    elif isinstance(value, dict):
        for child in value.values():
            result.extend(strings(child))
    elif isinstance(value, list):
        for child in value:
            result.extend(strings(child))
    return result


def contains(item: dict[str, Any], needles: tuple[str, ...]) -> bool:
    haystack = "\n".join(strings(item)).lower()
    return any(needle.lower() in haystack for needle in needles)


def count_status(value: Any, key: str, statuses: set[str]) -> int:
    return sum(
        1
        for item in walk(value)
        if isinstance(item.get(key), str) and item[key].lower() in statuses
    )


def patch_document(path: Path) -> bool:
    document = json.loads(path.read_text(encoding="utf-8"))
    current_items = [item for item in walk(document) if contains(item, CURRENT)]
    if not current_items:
        return False
    changed = False
    for current_name in CURRENT:
        matches = [item for item in current_items if contains(item, (current_name,))]
        if len(matches) != 1:
            raise ValueError(f"{path}: {current_name} matched {len(matches)} ledger objects")
        item = matches[0]
        status_keys = [
            key
            for key, value in item.items()
            if isinstance(value, str) and value.lower() in MIGRATE_VALUES
        ]
        if not status_keys:
            continue
        if len(status_keys) != 1:
            raise ValueError(f"{path}: {current_name} has ambiguous migration status keys {status_keys}")
        key = status_keys[0]
        completed_values: Counter[str] = Counter()
        for candidate in walk(document):
            if not contains(candidate, COMPLETED_HINTS):
                continue
            value = candidate.get(key)
            if isinstance(value, str) and value.lower() not in MIGRATE_VALUES:
                completed_values[value] += 1
        if not completed_values:
            raise ValueError(f"{path}: cannot infer completed value for status key {key!r}")
        ordered = completed_values.most_common()
        if len(ordered) > 1 and ordered[0][1] == ordered[1][1]:
            raise ValueError(f"{path}: completed status value is ambiguous: {ordered}")
        completed = ordered[0][0]
        item[key] = completed
        print(f"INCLUDE_LEDGER_STATUS={current_name}:{key}:{completed}")
        changed = True

    if not changed:
        return False

    # Refresh explicit aggregate counters only when their key names make their
    # meaning unambiguous. Unknown aggregates remain untouched and are left to
    # the repository's own checker.
    all_dicts = list(walk(document))
    for item in all_dicts:
        for key, value in list(item.items()):
            if not isinstance(value, int):
                continue
            lower = key.lower()
            if lower in {"migrate_count", "migration_debt_count", "pending_migration_count"}:
                status_key_candidates = {
                    candidate_key
                    for candidate in all_dicts
                    for candidate_key, candidate_value in candidate.items()
                    if isinstance(candidate_value, str)
                    and candidate_value.lower() in MIGRATE_VALUES
                }
                if len(status_key_candidates) == 1:
                    status_key = next(iter(status_key_candidates))
                    item[key] = count_status(document, status_key, MIGRATE_VALUES)
                    print(f"INCLUDE_LEDGER_COUNTER={key}:{item[key]}")

    path.write_text(
        json.dumps(document, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
        newline="\n",
    )
    return True


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: patch-rts-include-migration-ledger.py REPOSITORY_ROOT", file=sys.stderr)
        return 64
    root = Path(sys.argv[1]).resolve(strict=True)
    candidates = []
    for base in (root / "docs", root / "scripts", root / "trillionnium"):
        if not base.exists():
            continue
        for path in base.rglob("*.json"):
            try:
                text = path.read_text(encoding="utf-8")
            except (OSError, UnicodeDecodeError):
                continue
            if any(name in text for name in CURRENT):
                candidates.append(path)
    changed = []
    for path in sorted(set(candidates)):
        if patch_document(path):
            changed.append(path.relative_to(root).as_posix())
    if len(changed) > 1:
        print(f"multiple migration ledgers changed: {changed}", file=sys.stderr)
        return 1
    if changed:
        print("INCLUDE_MIGRATION_LEDGER_PATCHED=" + changed[0])
    else:
        print("INCLUDE_MIGRATION_LEDGER_PATCHED=not_required")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, ValueError, json.JSONDecodeError) as error:
        print(f"INCLUDE_MIGRATION_LEDGER_PATCH=FAIL reason={error}", file=sys.stderr)
        raise SystemExit(1)
