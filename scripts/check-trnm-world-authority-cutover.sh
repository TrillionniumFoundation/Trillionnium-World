#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
AUTHORITY_ROOT="$ROOT/trillionnium/crates/world-authority"
MANIFEST="$AUTHORITY_ROOT/Cargo.toml"
LOCKFILE="$AUTHORITY_ROOT/Cargo.lock"
CONTRACT="$ROOT/docs/contracts/trillionnium-world-authority-cutover-v1.json"
PROVENANCE="$ROOT/docs/contracts/trillionnium-world-authority-provenance-v1.json"
PROVENANCE_CHECKER="$ROOT/scripts/check-trnm-world-authority-provenance.py"

python3 - "$AUTHORITY_ROOT" "$CONTRACT" <<'PY'
import json
import pathlib
import sys
import tomllib

root = pathlib.Path(sys.argv[1]).resolve()
contract_path = pathlib.Path(sys.argv[2]).resolve()
expected = {
    "trnm-world-domain",
    "trnm-world-command",
    "trnm-world-projection",
    "trnm-world-map-provider",
    "trnm-world-ui-fragments",
    "trnm-world-api",
    "trnm-world-server",
}
manifest = tomllib.loads((root / "Cargo.toml").read_text())
members = set(manifest["workspace"]["members"])
if members != expected:
    raise SystemExit(f"world authority workspace drift: expected={sorted(expected)} actual={sorted(members)}")

for member in sorted(expected):
    crate = root / member
    cargo = crate / "Cargo.toml"
    if not cargo.is_file():
        raise SystemExit(f"missing crate manifest: {cargo}")
    parsed = tomllib.loads(cargo.read_text())
    package_name = parsed.get("package", {}).get("name")
    if package_name != member:
        raise SystemExit(f"package identity mismatch: path={member} name={package_name}")
    for section in ("dependencies", "dev-dependencies", "build-dependencies"):
        for dependency, value in parsed.get(section, {}).items():
            if not isinstance(value, dict) or "path" not in value:
                continue
            resolved = (crate / value["path"]).resolve()
            try:
                resolved.relative_to(root)
            except ValueError as exc:
                raise SystemExit(
                    f"external or sibling-repository path dependency forbidden: {member}:{dependency}:{resolved}"
                ) from exc
            if resolved.name not in expected:
                raise SystemExit(
                    f"path dependency escapes seven-crate authority closure: {member}:{dependency}:{resolved.name}"
                )

contract = json.loads(contract_path.read_text())
if contract.get("production_authorization") != "not_granted":
    raise SystemExit("source restoration must not self-grant production authorization")
if set(contract.get("restored_crates", [])) != expected:
    raise SystemExit("contract restored_crates does not match workspace")
if contract.get("authority_rule") != "world_repository_is_the_only_world_state_writer":
    raise SystemExit("single-writer authority rule missing")
production = contract.get("profiles", {}).get("production", {})
if production.get("fail_closed") is not True:
    raise SystemExit("production profile must be fail-closed")
if production.get("fixture_adapters_allowed") is not False:
    raise SystemExit("production profile cannot allow fixture adapters")
if production.get("file_repository_allowed") is not False:
    raise SystemExit("production profile cannot allow the development file repository")
print("world authority dependency and contract boundary: ok")
PY

python3 "$PROVENANCE_CHECKER" \
  --repo-root "$ROOT" \
  --manifest "$PROVENANCE" \
  --self-test

if grep -R --include='Cargo.toml' -nE 'trnm-world-bevy|trnm-world-dev-env|legacy-game|TrillionniumFoundation/CEX|services/consumer-entry-api' "$AUTHORITY_ROOT"; then
  echo "retired client, legacy workspace, or CEX implementation dependency leaked into authority workspace" >&2
  exit 1
fi

if [[ ! -s "$LOCKFILE" ]]; then
  echo "world authority lockfile is missing or empty" >&2
  exit 1
fi
lock_before="$(sha256sum "$LOCKFILE" | awk '{print $1}')"

cargo fmt --manifest-path "$MANIFEST" --all -- --check
cargo test --manifest-path "$MANIFEST" --workspace --all-targets --locked
cargo clippy --manifest-path "$MANIFEST" --workspace --all-targets --locked -- -D warnings

TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

cargo run --quiet --locked --manifest-path "$MANIFEST" -p trnm-world-server -- adapter-readiness > "$TMP/adapter-readiness.json"
cargo run --quiet --locked --manifest-path "$MANIFEST" -p trnm-world-server -- full-split-json > "$TMP/full-split.json"
cargo run --quiet --locked --manifest-path "$MANIFEST" -p trnm-world-server -- dev-runtime-repository-smoke "$TMP/world-state.json" > "$TMP/repository-smoke.json"

lock_after="$(sha256sum "$LOCKFILE" | awk '{print $1}')"
if [[ "$lock_before" != "$lock_after" ]]; then
  echo "cargo qualification mutated the committed World authority lockfile" >&2
  exit 1
fi

python3 - "$TMP" <<'PY'
import json
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
adapters = json.loads((root / "adapter-readiness.json").read_text())
if adapters.get("adapter_contract") != "trillionnium_world_runtime_adapter_v1":
    raise SystemExit("runtime adapter contract mismatch")
statuses = adapters.get("statuses", [])
roles = {status.get("role") for status in statuses}
expected_roles = {"identity", "session_guard", "ledger", "repository", "evidence_sink", "metrics_sink"}
if roles != expected_roles:
    raise SystemExit(f"adapter role drift: expected={sorted(expected_roles)} actual={sorted(roles)}")
if len(statuses) != len(expected_roles):
    raise SystemExit("runtime adapter status list contains duplicate or extra roles")
if not all(status.get("fixture_adapter_available") is True for status in statuses):
    raise SystemExit("deterministic fixture adapters are required for parity tests")
if not all(status.get("production_adapter_trait_ready") is True for status in statuses):
    raise SystemExit("all production adapter trait seams must be declared")

full_split = json.loads((root / "full-split.json").read_text())
if full_split.get("api_contract") != "trillionnium_world_api_v1":
    raise SystemExit("full-split API contract mismatch")
if full_split.get("response_contract") != "trillionnium_world_full_split_response_v1":
    raise SystemExit("full-split response contract mismatch")
if full_split.get("domain_contract") != "trillionnium_world_domain_v1":
    raise SystemExit("full-split domain contract mismatch")
if len(full_split.get("runtime_adapters", {}).get("statuses", [])) != len(expected_roles):
    raise SystemExit("full-split runtime adapter coverage mismatch")

repository = json.loads((root / "repository-smoke.json").read_text())
expected_repository = {
    "contract_version": "trillionnium_world_dev_file_repository_v1",
    "dev_runtime_contract": "trillionnium_world_dev_runtime_v1",
    "status": "file_repository_persistence_green",
    "state_file_exists": True,
    "source_of_truth": "rust_world_state_json_repository",
    "before_repository_mode": "file_backed_json",
    "command_accepted": True,
    "command_player_node": "starter-studio",
    "reloaded_player_node": "starter-studio",
}
for key, expected in expected_repository.items():
    actual = repository.get(key)
    if actual != expected:
        raise SystemExit(
            f"repository restart/reload structural evidence mismatch: {key} expected={expected!r} actual={actual!r}"
        )
if not isinstance(repository.get("reloaded_node_count"), int) or repository["reloaded_node_count"] <= 0:
    raise SystemExit("repository restart/reload evidence has no restored nodes")
if repository.get("command_player_node") != repository.get("reloaded_player_node"):
    raise SystemExit("repository restart/reload state continuity mismatch")
if not (root / "world-state.json").is_file():
    raise SystemExit("repository restart/reload state file was not materialized")
print("world authority runtime, projection and structured restart/reload smokes: ok")
PY
