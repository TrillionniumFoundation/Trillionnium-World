#!/usr/bin/env python3
"""Exercise the external-contract documentation checker through hostile copies."""

from __future__ import annotations

import json
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]
CHECKER = Path("scripts/check-trnm-world-contract-module-documentation.py")
DESIGN = Path("docs/modules/contracts/audit-events-design.md")
LOCAL_README = Path("contracts/audit-events/README.md")
DOCUMENT_CATALOG = Path("docs/catalog.json")
COMPONENT_CATALOG = Path("docs/component-catalog.json")
WORKSPACE = Path("contracts/Cargo.toml")


def run(root: Path, expect_success: bool) -> None:
    completed = subprocess.run(
        [
            sys.executable,
            str(root / CHECKER),
            "--root",
            str(root),
            "--as-of",
            "2026-09-10",
        ],
        cwd=root,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        timeout=60,
        check=False,
    )
    if (completed.returncode == 0) != expect_success:
        raise AssertionError(
            f"unexpected checker result rc={completed.returncode}\n{completed.stdout}"
        )


def clone_and_mutate(mutator) -> None:
    with tempfile.TemporaryDirectory(prefix="trnm-contract-doc-") as temporary:
        clone = Path(temporary) / "repo"
        shutil.copytree(
            ROOT,
            clone,
            ignore=shutil.ignore_patterns(".git", "target", "node_modules", "run"),
        )
        mutator(clone)
        run(clone, expect_success=False)


def mutate_text(relative: Path, transform):
    def apply(root: Path) -> None:
        target = root / relative
        target.write_text(
            transform(target.read_text(encoding="utf-8")),
            encoding="utf-8",
        )

    return apply


def shallow_public_interfaces(root: Path) -> None:
    path = root / DESIGN
    text = path.read_text(encoding="utf-8")
    text, count = re.subn(
        r"(?ms)(^## Public interfaces\s*$).*?(?=^## State model and invariants\s*$)",
        "## Public interfaces\n\nToo short.\n\n",
        text,
        count=1,
    )
    if count != 1:
        raise AssertionError("public interfaces section was not found")
    path.write_text(text, encoding="utf-8")


def remove_document_catalogue_entry(root: Path) -> None:
    path = root / DOCUMENT_CATALOG
    data = json.loads(path.read_text(encoding="utf-8"))
    data["documents"] = [
        item
        for item in data["documents"]
        if item.get("path") != "docs/modules/contracts/audit-events-design.md"
    ]
    path.write_text(
        json.dumps(data, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )


def drift_component_catalogue(root: Path) -> None:
    path = root / COMPONENT_CATALOG
    data = json.loads(path.read_text(encoding="utf-8"))
    for item in data["components"]:
        if item.get("path") == "contracts/audit-events":
            item["canonical_docs"] = ["contracts/audit-events/README.md"]
            break
    path.write_text(
        json.dumps(data, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )


def add_unknown_workspace_member(root: Path) -> None:
    path = root / WORKSPACE
    text = path.read_text(encoding="utf-8")
    text = text.replace(
        '    "settlement-vault",\n]',
        '    "settlement-vault",\n    "unregistered-contract",\n]',
        1,
    )
    path.write_text(text, encoding="utf-8")


def main() -> None:
    run(ROOT, expect_success=True)

    clone_and_mutate(
        mutate_text(
            DESIGN,
            lambda text: text.replace("## State model and invariants", "## State"),
        )
    )
    clone_and_mutate(shallow_public_interfaces)
    clone_and_mutate(
        mutate_text(
            DESIGN,
            lambda text: text.replace("status: current-candidate", "status: current", 1),
        )
    )
    clone_and_mutate(
        mutate_text(
            DESIGN,
            lambda text: text.replace(
                "owner: trillionnium-world-contracts",
                "owner: nobody",
                1,
            ),
        )
    )
    clone_and_mutate(
        mutate_text(
            DESIGN,
            lambda text: text.replace(
                "implementation_conformance: not-implied",
                "implementation_conformance: complete",
                1,
            ),
        )
    )
    clone_and_mutate(
        mutate_text(
            DESIGN,
            lambda text: text.replace(
                "review_due: 2026-10-08",
                "review_due: 2026-09-01",
                1,
            ),
        )
    )
    clone_and_mutate(
        mutate_text(
            DESIGN,
            lambda text: text.replace(
                "contracts/audit-events/src/",
                "missing/source/",
                1,
            ),
        )
    )
    clone_and_mutate(
        mutate_text(
            DESIGN,
            lambda text: text + "\nproduction_authorization: granted\n",
        )
    )
    clone_and_mutate(
        mutate_text(
            LOCAL_README,
            lambda text: text.replace(
                "../../docs/modules/contracts/audit-events-design.md",
                "../../docs/modules/contracts/missing-design.md",
                1,
            ),
        )
    )
    clone_and_mutate(remove_document_catalogue_entry)
    clone_and_mutate(drift_component_catalogue)
    clone_and_mutate(add_unknown_workspace_member)

    print("TRNM_WORLD_CONTRACT_MODULE_DOCUMENTATION_TESTS=PASS")


if __name__ == "__main__":
    main()
