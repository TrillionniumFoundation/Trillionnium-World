from __future__ import annotations

import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OPENAPI = ROOT / "docs/protocol/openapi/trnm-world-legacy-local-alpha-v1.openapi.json"
TEMPLATES = ROOT / "docs/protocol/openapi/path-templates.json"
GAME_ROOT = ROOT / "trillionnium/crates/trnm-game-server/src"
PROTOCOL_SOURCE = ROOT / "trillionnium/crates/trnm-online-protocol/src/lib.rs"
STREAM_SOURCE = GAME_ROOT / "stream.rs"


def load(path: Path):
    if not path.is_file():
        raise SystemExit(f"missing required file: {path.relative_to(ROOT)}")
    with path.open("rb") as handle:
        return json.load(handle)


def read(path: Path) -> str:
    if not path.is_file():
        raise SystemExit(f"missing required file: {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def resolve_pointer(document, pointer: str):
    if pointer in ("", "/"):
        return document
    node = document
    for raw in pointer.lstrip("/").split("/"):
        token = raw.replace("~1", "/").replace("~0", "~")
        node = node[token]
    return node


def router_source() -> tuple[Path, str]:
    source_path = GAME_ROOT / "lib.rs"
    if not source_path.is_file() or source_path.stat().st_size < 100_000:
        source_path = GAME_ROOT / "lib.rs.in"
    return source_path, read(source_path)


def source_routes(source: str) -> dict[tuple[str, str], str]:
    route_re = re.compile(
        r'\.route\(\s*"(?P<path>[^"]+)"\s*,\s*'
        r'(?P<method>get|post|put|delete|patch)\s*\(\s*'
        r'(?P<handler>(?:[A-Za-z_][A-Za-z0-9_]*::)*[A-Za-z_][A-Za-z0-9_]*)\s*\)',
        re.S,
    )
    routes: dict[tuple[str, str], str] = {}
    for match in route_re.finditer(source):
        path = re.sub(r":([A-Za-z_][A-Za-z0-9_]*)", r"{\1}", match.group("path"))
        key = (path, match.group("method"))
        if key in routes:
            raise SystemExit(f"duplicate source route: {key}")
        routes[key] = match.group("handler").split("::")[-1]
    if not routes:
        raise SystemExit("no game-server routes parsed")
    return routes
