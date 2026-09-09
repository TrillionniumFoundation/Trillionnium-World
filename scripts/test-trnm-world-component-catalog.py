#!/usr/bin/env python3
"""Hostile fixtures for the closed physical component catalogue verifier."""

from __future__ import annotations

import json
from pathlib import Path
import shutil
import subprocess
import tempfile
from typing import Callable

CHECKER = Path(__file__).with_name("check-trnm-world-component-catalog.py")
GAME = [
    "trnm-economy-protocol", "trnm-rpg-core", "trnm-campaign-core",
    "trnm-rts-protocol", "trnm-rts-sim", "trnm-online-protocol",
    "trnm-game-server", "trnm-first-contact",
]
AUTHORITY = [
    "trnm-world-domain", "trnm-world-command", "trnm-world-projection",
    "trnm-world-map-provider", "trnm-world-ui-fragments", "trnm-world-api",
    "trnm-world-server",
]


def write(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def rust_package(root: Path, path: str, name: str, with_tests: bool = True) -> None:
    write(root / path / "Cargo.toml", f'[package]\nname = "{name}"\nversion = "0.1.0"\nedition = "2021"\n')
    test = "\n#[cfg(test)]\nmod tests { #[test] fn smoke() { assert!(true); } }\n" if with_tests else "\n"
    write(root / path / "src/lib.rs", f"pub fn identity() -> &'static str {{ \"{name}\" }}{test}")
    write(root / path / "README.md", f"# {name}\ncomponent path `{path}`\n")


def workspace(root: Path, path: str, members: list[str], extra: str = "") -> None:
    quoted = ", ".join(json.dumps(member) for member in members)
    write(root / path / "Cargo.toml", f"[workspace]\nmembers = [{quoted}]\nresolver = \"2\"\n{extra}")
    write(root / path / "README.md", f"# workspace\n{path}\n")


def entry(component_id: str, path: str, kind: str, lifecycle: str, denominator: str, docs: list[str]) -> dict[str, object]:
    manifest_name = "Cargo.toml" if kind.startswith("rust-") else "package.json"
    return {
        "id": component_id,
        "path": path,
        "manifest": f"{path}/{manifest_name}",
        "kind": kind,
        "lifecycle": lifecycle,
        "release_denominator": denominator,
        "owner": "fixture-owner",
        "canonical_docs": docs,
        "gate": "gates/all-components.py",
    }


def make_fixture(root: Path) -> None:
    components: list[dict[str, object]] = []
    workspace(
        root,
        "trillionnium",
        [f"crates/{name}" for name in GAME],
        '\n[patch.crates-io]\nwayland-scanner = { path = "vendor/wayland-scanner" }\n',
    )
    write(root / "README.md", "game-product-workspace trillionnium docs/modules/README.md\n")
    write(root / "docs/modules/README.md", "game-product-workspace trillionnium/Cargo.toml\n")
    components.append(entry("game-product-workspace", "trillionnium", "rust-workspace", "active", "game-product", ["README.md", "docs/modules/README.md"]))
    for name in GAME:
        path = f"trillionnium/crates/{name}"
        rust_package(root, path, name)
        design = f"docs/modules/{name}-design.md"
        write(root / design, f"# {name}\n{path}\n")
        lifecycle = "active"
        if name == "trnm-online-protocol":
            lifecycle = "active-compatibility"
        elif name == "trnm-game-server":
            lifecycle = "compatibility-laboratory"
        components.append(entry(name, path, "rust-crate", lifecycle, "game-product", [f"{path}/README.md", design]))

    auth_root = "trillionnium/crates/world-authority"
    workspace(root, auth_root, AUTHORITY)
    write(root / "docs/modules/world-authority/README.md", f"world-authority-workspace {auth_root}\n")
    components.append(entry("world-authority-workspace", auth_root, "rust-workspace", "cutover-candidate", "world-authority-candidate", [f"{auth_root}/README.md", "docs/modules/world-authority/README.md"]))
    for name in AUTHORITY:
        path = f"{auth_root}/{name}"
        rust_package(root, path, name)
        design = f"docs/modules/world-authority/{name}-design.md"
        write(root / design, f"# {name}\n{path}\n")
        lifecycle = "fixture-candidate" if name in {"trnm-world-map-provider", "trnm-world-ui-fragments", "trnm-world-server"} else "cutover-candidate"
        components.append(entry(name, path, "rust-crate", lifecycle, "world-authority-candidate", [f"{path}/README.md", design]))

    platform = "trillionnium/crates/platform"
    workspace(root, platform, ["trnm-node"])
    rust_package(root, f"{platform}/trnm-node", "trnm-node", with_tests=False)
    write(root / platform / "README.md", "legacy-platform-workspace legacy-trnm-node excluded-legacy\n")
    components.append(entry("legacy-platform-workspace", platform, "rust-workspace", "excluded-legacy", "none", [f"{platform}/README.md"]))
    components.append(entry("legacy-trnm-node", f"{platform}/trnm-node", "rust-crate", "excluded-legacy", "none", [f"{platform}/README.md"]))

    workspace(root, "contracts", ["audit-events"])
    rust_package(root, "contracts/audit-events", "audit-events")
    write(root / "docs/modules/contracts/README.md", "external-contracts-workspace contracts/Cargo.toml audit-events\n")
    components.append(entry("external-contracts-workspace", "contracts", "rust-workspace", "mvp-perimeter", "scope-dependent", ["contracts/README.md", "docs/modules/contracts/README.md"]))
    write(root / "docs/modules/contracts/audit-events-design.md", "audit-events contracts/audit-events\n")
    components.append(entry("audit-events", "contracts/audit-events", "rust-crate", "mvp-perimeter", "scope-dependent", ["contracts/audit-events/README.md", "docs/modules/contracts/audit-events-design.md"]))

    transition = "trillionnium/contracts/trnm-world-transition-v1"
    rust_package(root, transition, "trnm-world-transition-v1")
    write(root / "docs/protocol/trnm-world-transition-v1.md", f"trnm-world-transition-v1 {transition}\n")
    components.append(entry("trnm-world-transition-v1", transition, "rust-crate", "protocol-contract", "cross-repository-contract", [f"{transition}/README.md", "docs/protocol/trnm-world-transition-v1.md"]))

    vendor = "trillionnium/vendor/wayland-scanner"
    rust_package(root, vendor, "wayland-scanner", with_tests=False)
    write(root / vendor / "LICENSE.txt", "MIT\n")
    components.append(entry("vendor-wayland-scanner", vendor, "rust-crate", "vendored-dependency", "dependency-only", [f"{vendor}/README.md", f"{vendor}/LICENSE.txt"]))

    write(root / "web4-frontend/package.json", json.dumps({"name": "web4-frontend", "private": True, "scripts": {"build": "echo build", "test": "echo test"}}))
    write(root / "web4-frontend/app/page.tsx", "export default function Page(){ return null }\n")
    write(root / "web4-frontend/README.md", "web4-frontend historical-compatible-subproject\n")
    components.append(entry("web4-frontend", "web4-frontend", "node-application", "historical-compatible-subproject", "none", ["web4-frontend/README.md"]))

    write(root / "examples/sdk-js/package.json", json.dumps({"name": "trnm-sdk-js-example", "private": True, "type": "module", "scripts": {"start": "node quickstart.js"}}))
    write(root / "examples/sdk-js/quickstart.js", "console.log('example')\n")
    write(root / "examples/sdk-js/README.md", "trnm-sdk-js-example examples/sdk-js historical example\n")
    components.append(entry("trnm-sdk-js-example", "examples/sdk-js", "node-example", "historical-example", "none", ["examples/sdk-js/README.md"]))

    tokens: list[str] = []
    for item in components:
        tokens.extend([str(item["id"]), str(item["path"]), str(item["manifest"])])
    write(root / "gates/all-components.py", "# " + "\n# ".join(tokens) + "\n")
    catalog = {
        "schema": "trnm_world_component_catalog_v1",
        "as_of": "2026-09-09",
        "repository": "TrillionniumFoundation/Trillionnium-World",
        "production_authorization": "not_granted",
        "components": components,
    }
    write(root / "docs/component-catalog.json", json.dumps(catalog, indent=2) + "\n")


def run(root: Path, expect_success: bool) -> None:
    process = subprocess.run(
        ["python3", str(CHECKER), "--root", str(root)], text=True,
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False,
    )
    if (process.returncode == 0) != expect_success:
        raise AssertionError(f"unexpected result rc={process.returncode}\nstdout={process.stdout}\nstderr={process.stderr}")


def mutate_json(root: Path, mutator: Callable[[dict[str, object]], None]) -> None:
    path = root / "docs/component-catalog.json"
    data = json.loads(path.read_text(encoding="utf-8"))
    mutator(data)
    write(path, json.dumps(data, indent=2) + "\n")


def case(name: str, baseline: Path, action: Callable[[Path], None]) -> None:
    target = baseline.parent / name
    shutil.copytree(baseline, target)
    action(target)
    run(target, False)


def main() -> int:
    with tempfile.TemporaryDirectory(prefix="trnm-world-component-catalog-") as raw:
        base = Path(raw) / "base"
        make_fixture(base)
        run(base, True)
        case("root-extra", base, lambda r: mutate_json(r, lambda d: d.__setitem__("extra", True)))
        case("root-missing", base, lambda r: mutate_json(r, lambda d: d.pop("as_of")))
        case("entry-extra", base, lambda r: mutate_json(r, lambda d: d["components"][1].__setitem__("extra", True)))
        case("entry-missing", base, lambda r: mutate_json(r, lambda d: d["components"][1].pop("owner")))
        case("absolute-path", base, lambda r: mutate_json(r, lambda d: d["components"][1].__setitem__("path", "/tmp/escape")))
        case("parent-path", base, lambda r: mutate_json(r, lambda d: d["components"][1].__setitem__("path", "../escape")))
        case("dot-alias", base, lambda r: mutate_json(r, lambda d: d["components"][1].__setitem__("path", "./trillionnium/crates/trnm-economy-protocol")))
        case("backslash", base, lambda r: mutate_json(r, lambda d: d["components"][1].__setitem__("path", "trillionnium\\crates\\trnm-economy-protocol")))
        case("trailing-slash", base, lambda r: mutate_json(r, lambda d: d["components"][1].__setitem__("path", "trillionnium/crates/trnm-economy-protocol/")))
        case("manifest-outside", base, lambda r: mutate_json(r, lambda d: d["components"][1].__setitem__("manifest", "trillionnium/Cargo.toml")))
        case("duplicate-id", base, lambda r: mutate_json(r, lambda d: d["components"][1].__setitem__("id", d["components"][2]["id"])))
        case("lifecycle-contradiction", base, lambda r: mutate_json(r, lambda d: d["components"][1].__setitem__("release_denominator", "none")))
        case("production-granted", base, lambda r: mutate_json(r, lambda d: d.__setitem__("production_authorization", "granted")))
        case("missing-gate", base, lambda r: mutate_json(r, lambda d: d["components"][1].__setitem__("gate", "gates/missing.py")))
        case("unclassified-rust", base, lambda r: rust_package(r, "unclassified/rust", "unclassified-rust"))
        case("unclassified-node", base, lambda r: write(r / "unclassified/node/package.json", '{"name":"hidden-node"}\n'))

        def generic_docs(root: Path) -> None:
            write(root / "docs/generic-a.md", "generic words only\n")
            write(root / "docs/generic-b.md", "still generic\n")
            mutate_json(root, lambda data: data["components"][1].__setitem__("canonical_docs", ["docs/generic-a.md", "docs/generic-b.md"]))
        case("generic-docs", base, generic_docs)

        def package_mismatch(root: Path) -> None:
            path = root / "trillionnium/crates/trnm-economy-protocol/Cargo.toml"
            write(path, path.read_text(encoding="utf-8").replace('name = "trnm-economy-protocol"', 'name = "wrong-name"'))
        case("package-mismatch", base, package_mismatch)

        def node_without_test(root: Path) -> None:
            path = root / "web4-frontend/package.json"
            data = json.loads(path.read_text(encoding="utf-8")); data["scripts"].pop("test")
            write(path, json.dumps(data))
        case("node-without-test", base, node_without_test)

        def sdk_without_start(root: Path) -> None:
            path = root / "examples/sdk-js/package.json"
            data = json.loads(path.read_text(encoding="utf-8")); data["scripts"].pop("start")
            write(path, json.dumps(data))
        case("sdk-without-start", base, sdk_without_start)

        def symlink_component(root: Path) -> None:
            path = root / "trillionnium/crates/trnm-economy-protocol"
            moved = path.with_name(path.name + "-real"); path.rename(moved)
            path.symlink_to(moved.name, target_is_directory=True)
        case("symlink-component", base, symlink_component)

        def symlink_manifest(root: Path) -> None:
            path = root / "trillionnium/crates/trnm-economy-protocol/Cargo.toml"
            moved = path.with_name("Cargo.real.toml"); path.rename(moved); path.symlink_to(moved.name)
        case("symlink-manifest", base, symlink_manifest)

        def workspace_unclassified(root: Path) -> None:
            rust_package(root, "trillionnium/crates/ghost", "ghost")
            path = root / "trillionnium/Cargo.toml"
            write(path, path.read_text(encoding="utf-8").replace("]\nresolver", ', "crates/ghost"]\nresolver', 1))
        case("workspace-unclassified", base, workspace_unclassified)

        duplicate = base.parent / "duplicate-json"; shutil.copytree(base, duplicate)
        path = duplicate / "docs/component-catalog.json"
        write(path, path.read_text(encoding="utf-8").replace('"schema": "trnm_world_component_catalog_v1",', '"schema": "trnm_world_component_catalog_v1",\n  "schema": "duplicate",', 1))
        run(duplicate, False)

        nonfinite = base.parent / "nonfinite"; shutil.copytree(base, nonfinite)
        path = nonfinite / "docs/component-catalog.json"
        write(path, path.read_text(encoding="utf-8").replace('"as_of": "2026-09-09",', '"as_of": "2026-09-09",\n  "bad": NaN,', 1))
        run(nonfinite, False)

    print("TRNM World component catalogue hostile fixtures: PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
