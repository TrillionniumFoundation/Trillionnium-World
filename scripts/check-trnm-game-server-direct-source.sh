#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
crate="${repo_root}/trillionnium/crates/trnm-game-server"
manifest="${crate}/Cargo.toml"
lib="${crate}/src/lib.rs"
parts_manifest="${crate}/src/lib_parts/manifest.json"

fail() {
  printf 'direct-source gate failed: %s\n' "$*" >&2
  exit 1
}

[[ -f "${manifest}" ]] || fail "missing ${manifest}"
[[ -f "${lib}" ]] || fail "missing ${lib}"
[[ -f "${parts_manifest}" ]] || fail "missing semantic ownership manifest"
[[ ! -e "${crate}/build.rs" ]] || fail "semantic or implicit build.rs remains"
[[ ! -e "${crate}/src/lib.rs.in" ]] || fail "template source src/lib.rs.in remains"
[[ ! -e "${crate}/src/settlement_worker.rs.in" ]] || fail "settlement worker template remains"
[[ ! -e "${crate}/src/cex.rs.in" ]] || fail "CEX template remains"

if grep -Eq '^\s*build\s*=\s*"build\.rs"\s*$' "${manifest}"; then
  fail "Cargo manifest still declares build.rs"
fi
if grep -Eq 'OUT_DIR|trnm_game_server_lib_generated|include!\s*\(\s*concat!' "${lib}"; then
  fail "crate root still depends on generated OUT_DIR source"
fi
if ! grep -Fq 'include!("lib_parts/' "${lib}"; then
  fail "crate root does not expose ordinary ownership-partitioned source"
fi

legacy_references="$(
  grep -R --exclude-dir=tests \
    --include='*.rs' --include='*.toml' \
    -nE 'src/lib\.rs\.in|trnm_game_server_lib_generated\.rs|semantic rewrite' \
    "${crate}/src" "${manifest}" 2>/dev/null || true
)"
[[ -z "${legacy_references}" ]] || {
  printf '%s\n' "${legacy_references}" >&2
  fail "legacy generated-source authority references remain in production source"
}

if grep -R --exclude-dir=tests --include='*.rs' \
    -n 'reconcile_economy(&state\.cex' "${crate}/src"; then
  fail "legacy synchronous campaign settlement call remains"
fi
if grep -R --exclude-dir=tests --include='*.rs' \
    -n 'settle_pending_matches(&settlement_state' "${crate}/src"; then
  fail "legacy in-server terminal settlement loop remains"
fi

python3 - "${crate}" "${parts_manifest}" <<'PY'
from __future__ import annotations

import hashlib
import json
import pathlib
import sys

crate = pathlib.Path(sys.argv[1]).resolve()
manifest_path = pathlib.Path(sys.argv[2]).resolve()
try:
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
except (OSError, json.JSONDecodeError) as error:
    raise SystemExit(f"direct-source gate failed: invalid ownership manifest: {error}")

if manifest.get("schema") != "trnm_semantic_direct_source_partition_v1":
    raise SystemExit("direct-source gate failed: unexpected ownership manifest schema")
if manifest.get("crate") != "trnm-game-server":
    raise SystemExit("direct-source gate failed: ownership manifest targets the wrong crate")
if manifest.get("semantic_generation") is not False:
    raise SystemExit("direct-source gate failed: semantic_generation must be false")

records = manifest.get("parts")
if not isinstance(records, list) or not records:
    raise SystemExit("direct-source gate failed: ownership manifest has no parts")
seen: set[str] = set()
for record in records:
    if not isinstance(record, dict):
        raise SystemExit("direct-source gate failed: malformed ownership part record")
    relative = record.get("path")
    if not isinstance(relative, str) or not relative.startswith("lib_parts/"):
        raise SystemExit("direct-source gate failed: invalid ownership part path")
    if relative in seen:
        raise SystemExit(f"direct-source gate failed: duplicate ownership part {relative}")
    seen.add(relative)
    path = crate / "src" / relative
    try:
        payload = path.read_bytes()
    except OSError as error:
        raise SystemExit(f"direct-source gate failed: cannot read {relative}: {error}")
    if len(payload) != record.get("bytes"):
        raise SystemExit(f"direct-source gate failed: byte count drift for {relative}")
    if hashlib.sha256(payload).hexdigest() != record.get("sha256"):
        raise SystemExit(f"direct-source gate failed: SHA-256 drift for {relative}")
PY

total_bytes="$(
  find "${crate}/src" -type f -name '*.rs' ! -path '*/lib_parts/tests/*' -print0 \
    | xargs -0 cat | wc -c | tr -d ' '
)"
[[ "${total_bytes}" -gt 500000 ]] \
  || fail "direct production source bundle is unexpectedly small (${total_bytes} bytes)"

printf 'game_server_direct_source=passed production_source_bytes=%s\n' "${total_bytes}"
