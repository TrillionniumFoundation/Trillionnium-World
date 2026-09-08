#!/usr/bin/env python3
"""Negative and positive tests for the physical component catalogue gate."""

from __future__ import annotations

import json
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]
CHECKER = ROOT / "scripts/check-trnm-world-component-catalog.py"
CATALOG = ROOT / "docs/component-catalog.json"


def run(path: Path, *, expect_success: bool) -> None:
    completed = subprocess.run(
        [sys.executable, str(CHECKER), str(path)],
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        timeout=30,
        check=False,
    )
    if (completed.returncode == 0) != expect_success:
        raise AssertionError(
            f"unexpected result for {path}: rc={completed.returncode}\n{completed.stdout}"
        )


def write_json(directory: Path, name: str, value: object) -> Path:
    path = directory / name
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    return path


def main() -> None:
    baseline = json.loads(CATALOG.read_text(encoding="utf-8"))
    with tempfile.TemporaryDirectory(prefix="trnm-component-catalog-") as temporary:
        temp = Path(temporary)

        run(CATALOG, expect_success=True)

        duplicate_id = json.loads(json.dumps(baseline))
        duplicate_id["components"][1]["id"] = duplicate_id["components"][0]["id"]
        run(write_json(temp, "duplicate-id.json", duplicate_id), expect_success=False)

        duplicate_manifest = json.loads(json.dumps(baseline))
        duplicate_manifest["components"][1]["manifest"] = duplicate_manifest["components"][0]["manifest"]
        run(write_json(temp, "duplicate-manifest.json", duplicate_manifest), expect_success=False)

        missing_component = json.loads(json.dumps(baseline))
        missing_component["components"] = [
            component
            for component in missing_component["components"]
            if component["id"] != "trnm-rpg-core"
        ]
        run(write_json(temp, "missing-component.json", missing_component), expect_success=False)

        stale_manifest = json.loads(json.dumps(baseline))
        stale_manifest["components"][1]["manifest"] = "trillionnium/crates/trnm-economy-protocol/absent.toml"
        run(write_json(temp, "stale-manifest.json", stale_manifest), expect_success=False)

        authority_overclaim = json.loads(json.dumps(baseline))
        authority_overclaim["production_authorization"] = "granted"
        run(write_json(temp, "authority-overclaim.json", authority_overclaim), expect_success=False)

        legacy_denominator = json.loads(json.dumps(baseline))
        target = next(
            component
            for component in legacy_denominator["components"]
            if component["id"] == "legacy-trnm-node"
        )
        target["release_denominator"] = "game-product"
        run(write_json(temp, "legacy-denominator.json", legacy_denominator), expect_success=False)

        unsupported_lifecycle = json.loads(json.dumps(baseline))
        unsupported_lifecycle["components"][1]["lifecycle"] = "production"
        run(write_json(temp, "unsupported-lifecycle.json", unsupported_lifecycle), expect_success=False)

        duplicate_key = temp / "duplicate-key.json"
        duplicate_key.write_text(
            '{"schema":"trnm_world_component_catalog_v1",'
            '"schema":"forged","components":[]}\n',
            encoding="utf-8",
        )
        run(duplicate_key, expect_success=False)

        nonfinite = temp / "nonfinite.json"
        nonfinite.write_text(
            '{"schema":"trnm_world_component_catalog_v1",'
            '"repository":"TrillionniumFoundation/Trillionnium-World",'
            '"production_authorization":"not_granted",'
            '"components":[],"forged":NaN}\n',
            encoding="utf-8",
        )
        run(nonfinite, expect_success=False)

        invalid_utf8 = temp / "invalid-utf8.json"
        invalid_utf8.write_bytes(b'{"schema":"x","bad":"\xff"}')
        run(invalid_utf8, expect_success=False)

    print("TRNM_WORLD_COMPONENT_CATALOG_TESTS=PASS")


if __name__ == "__main__":
    main()
