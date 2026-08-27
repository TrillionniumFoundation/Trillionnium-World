#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[1]
REGISTRY = ROOT / "docs/status/world-gates-v1.json"
OUTPUT = ROOT / "docs/status/CURRENT.md"


def render() -> str:
    data = json.loads(REGISTRY.read_text(encoding="utf-8"))
    lines = [
        "# Trillionnium World Current Status",
        "",
        "> Generated from `docs/status/world-gates-v1.json`. Do not edit gate claims in this file.",
        "",
        f"- As of: `{data['as_of']}`",
        f"- Source plan: `{data['source_plan']}`",
        "- Public online: **NO-GO**",
        "- Public player market: **disabled**",
        "",
        "| Gate | Status | Authority profile | Primary blockers |",
        "| --- | --- | --- | --- |",
    ]
    for gate in data["gates"]:
        blockers = "<br>".join(gate["blockers"]) if gate["blockers"] else "None registered"
        lines.append(
            f"| `{gate['id']}` | **{gate['status']}** | `{gate['authority_profile']}` | {blockers} |"
        )
    lines.extend(["", "## Explicit limitations", ""])
    for gate in data["gates"]:
        lines.append(f"### `{gate['id']}`")
        lines.append("")
        for limitation in gate["limitations"]:
            lines.append(f"- {limitation}")
        lines.append("")
    lines.extend([
        "## Interpretation",
        "",
        "`implemented` is a source status, not remote verification, deployment, operational evidence or release readiness. A promoted status requires exact-commit remote evidence accepted by the gate schema. Gate schema v1 intentionally prevents public-online, public-market, trusted-settlement and closed-Nakama promotion.",
        "",
    ])
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    value = render()
    if args.check:
        current = OUTPUT.read_text(encoding="utf-8") if OUTPUT.exists() else ""
        if current != value:
            raise SystemExit("docs/status/CURRENT.md is stale; run scripts/render-trnm-world-status.py")
        print("TRNM World generated status is current")
    else:
        OUTPUT.parent.mkdir(parents=True, exist_ok=True)
        OUTPUT.write_text(value, encoding="utf-8")
        print(OUTPUT)


if __name__ == "__main__":
    main()
