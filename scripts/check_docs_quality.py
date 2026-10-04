#!/usr/bin/env python3
"""Fail-closed checks for canonical documentation and module inventory."""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path
from urllib.parse import unquote, urlsplit

try:
    import tomllib
except ModuleNotFoundError as exc:
    raise SystemExit("Python 3.11+ is required") from exc

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "docs" / "modules.json"
ALLOWED_STATUS = {"experimental", "mvp", "alpha", "pre-alpha", "migration-only", "stable", "historical"}
PERSONAL_PATH_PATTERNS = (
    re.compile(r"/Users/[^/\s`]+/"),
    re.compile(r"/home/[^/\s`]+/"),
    re.compile(r"[A-Za-z]:\\\\Users\\\\[^\\\\\s`]+\\\\"),
)
LINK_RE = re.compile(r"(?<!!)\[[^\]]+\]\(([^)]+)\)")
FENCE_RE = re.compile(r"```.*?```", re.DOTALL)


def workspace_members(manifest_path: Path) -> set[str]:
    data = tomllib.loads(manifest_path.read_text(encoding="utf-8"))
    return {(manifest_path.parent / member).relative_to(ROOT).as_posix()
            for member in data.get("workspace", {}).get("members", [])}


def link_target(source: Path, raw: str) -> Path | None:
    value = raw.strip().strip("<>")
    if not value or value.startswith("#"):
        return None
    parts = urlsplit(value)
    if parts.scheme or parts.netloc or not parts.path:
        return None
    return (source.parent / unquote(parts.path)).resolve()


def main() -> int:
    failures: list[str] = []
    if not MANIFEST.is_file():
        failures.append("docs/modules.json is missing")
        data = {"modules": []}
    else:
        try:
            data = json.loads(MANIFEST.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError) as exc:
            failures.append(f"cannot load docs/modules.json: {exc}")
            data = {"modules": []}
    if data.get("schema_version") != 1:
        failures.append("docs/modules.json schema_version must be 1")
    modules = data.get("modules")
    if not isinstance(modules, list) or not modules:
        failures.append("docs/modules.json modules must be non-empty")
        modules = []

    ids: set[str] = set()
    paths: set[str] = set()
    by_path: dict[str, dict] = {}
    entry_docs: list[Path] = []
    for index, module in enumerate(modules):
        prefix = f"modules[{index}]"
        if not isinstance(module, dict):
            failures.append(f"{prefix} is not an object")
            continue
        required = [module.get(key) for key in ("id", "path", "owner", "status", "entry_document")]
        if not all(isinstance(value, str) and value.strip() for value in required):
            failures.append(f"{prefix} lacks required id/path/owner/status/entry_document")
            continue
        module_id, module_path, _, status, entry_document = required
        if module_id in ids:
            failures.append(f"duplicate module id: {module_id}")
        if module_path in paths:
            failures.append(f"duplicate module path: {module_path}")
        ids.add(module_id); paths.add(module_path); by_path[module_path] = module
        if status not in ALLOWED_STATUS:
            failures.append(f"{module_id}: unsupported status {status}")
        absolute_module = (ROOT / module_path).resolve()
        absolute_doc = (ROOT / entry_document).resolve()
        try:
            absolute_module.relative_to(ROOT); absolute_doc.relative_to(ROOT)
        except ValueError:
            failures.append(f"{module_id}: path escapes repository")
        if not absolute_module.exists():
            failures.append(f"{module_id}: missing module path {module_path}")
        if not absolute_doc.is_file():
            failures.append(f"{module_id}: missing entry document {entry_document}")
        else:
            entry_docs.append(absolute_doc)

    workspaces = {
        "game": ROOT / "trillionnium" / "Cargo.toml",
        "platform": ROOT / "trillionnium" / "crates" / "platform" / "Cargo.toml",
        "contracts": ROOT / "contracts" / "Cargo.toml",
    }
    discovered: set[str] = set()
    for lane, manifest_path in workspaces.items():
        try:
            members = workspace_members(manifest_path)
        except (OSError, tomllib.TOMLDecodeError, ValueError) as exc:
            failures.append(f"cannot parse {manifest_path.relative_to(ROOT)}: {exc}")
            continue
        discovered |= members
        for member in sorted(members):
            if not (ROOT / member / "README.md").is_file():
                failures.append(f"{lane} workspace member lacks README.md: {member}")
            module = by_path.get(member)
            if module is None:
                failures.append(f"{lane} workspace member absent from docs/modules.json: {member}")
            elif module.get("workspace") != lane:
                failures.append(f"{member}: workspace mismatch")

    declared_rust = {m["path"] for m in modules if isinstance(m, dict) and m.get("kind") == "rust-crate" and "path" in m}
    for extra in sorted(declared_rust - discovered):
        failures.append(f"declared rust crate absent from workspaces: {extra}")

    root_readme = ROOT / "README.md"
    if not root_readme.is_file() or len(root_readme.read_text(encoding="utf-8").strip()) < 500:
        failures.append("README.md must be a substantive repository entry point")

    canonical = {
        ROOT / "README.md", ROOT / "PROJECT_BOUNDARY.md", ROOT / "SECURITY.md", ROOT / "CONTRIBUTING.md",
        ROOT / "docs" / "README.md", ROOT / "docs" / "DOCUMENTATION_POLICY.md",
        ROOT / "docs" / "MODULE_DOCUMENTATION_TEMPLATE.md", ROOT / "docs" / "architecture" / "README.md",
        ROOT / "docs" / "architecture" / "repository-boundaries.md", ROOT / "docs" / "development" / "README.md",
        ROOT / "docs" / "protocol" / "README.md", ROOT / "docs" / "runbooks" / "README.md",
        ROOT / "docs" / "release" / "README.md", ROOT / "docs" / "release" / "GAP_CLOSURE_BOARD_2026-09-11.md",
        ROOT / "trillionnium" / "docs" / "README.md", *entry_docs,
    }
    checked_links = 0
    for path in sorted(canonical):
        if not path.is_file():
            failures.append(f"missing canonical document: {path.relative_to(ROOT)}")
            continue
        text = path.read_text(encoding="utf-8")
        for pattern in PERSONAL_PATH_PATTERNS:
            match = pattern.search(text)
            if match:
                failures.append(f"{path.relative_to(ROOT)}: personal absolute path: {match.group(0)}")
        for match in LINK_RE.finditer(FENCE_RE.sub("", text)):
            target = link_target(path, match.group(1))
            if target is None:
                continue
            checked_links += 1
            try:
                target.relative_to(ROOT)
            except ValueError:
                failures.append(f"{path.relative_to(ROOT)}: link escapes repository: {match.group(1)}")
                continue
            if not target.exists():
                failures.append(f"{path.relative_to(ROOT)}: broken link: {match.group(1)}")

    if failures:
        print("documentation quality gate: FAIL", file=sys.stderr)
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        return 1
    print(f"documentation quality gate: PASS (modules={len(modules)}, rust_crates={len(discovered)}, files={len(canonical)}, local_links={checked_links})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
