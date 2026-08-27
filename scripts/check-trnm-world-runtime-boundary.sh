#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
RUNTIME_ROOT="$ROOT_DIR/contracts/world-runtime"
V1_ROOT="$RUNTIME_ROOT/v1"
RUST_ROOT="$RUNTIME_ROOT/rust"
HOST_ROOT="$RUNTIME_ROOT/host"

fail() {
  printf 'TRNM World runtime boundary failed: %s\n' "$*" >&2
  exit 1
}

required_files=(
  "$V1_ROOT/README.md"
  "$V1_ROOT/trnm-world-runtime-v1.schema.json"
  "$V1_ROOT/trnm-world-shadow-v1.schema.json"
  "$V1_ROOT/golden-vectors.json"
  "$V1_ROOT/shadow-vectors.json"
  "$V1_ROOT/error-catalog.json"
  "$V1_ROOT/compatibility-matrix.json"
  "$RUST_ROOT/Cargo.toml"
  "$RUST_ROOT/src/lib.rs"
  "$HOST_ROOT/Cargo.toml"
  "$HOST_ROOT/README.md"
  "$HOST_ROOT/src/lib.rs"
  "$HOST_ROOT/src/bin/trnm-world-runtime-exec.rs"
  "$HOST_ROOT/src/bin/trnm-world-runtime-shadow-diff.rs"
  "$ROOT_DIR/docs/protocol/trnm-world-runtime-v1.md"
  "$ROOT_DIR/docs/development/trnm-world-nakama-shadow-v1.md"
  "$ROOT_DIR/docs/runbooks/trnm-world-authority-cutover-v1.md"
  "$ROOT_DIR/scripts/verify-trnm-world-runtime-v1.py"
  "$ROOT_DIR/scripts/verify-trnm-world-shadow-v1.py"
)
for file in "${required_files[@]}"; do
  [[ -f "$file" ]] || fail "required artifact is missing: ${file#$ROOT_DIR/}"
done

for manifest in "$RUST_ROOT/Cargo.toml" "$HOST_ROOT/Cargo.toml"; do
  if grep -niE '^[[:space:]]*(bevy|axum|reqwest|hyper|tokio|sqlx|postgres|diesel|tonic|openssl|ring|ed25519|secp256k1)[[:space:]]*=' "$manifest"; then
    fail "forbidden authority, network, persistence, UI or signing dependency in ${manifest#$ROOT_DIR/}"
  fi
done

forbidden_use='(^|[[:space:]])(use|extern[[:space:]]+crate)[[:space:]]+(std::net|tokio::net|reqwest|axum|hyper|sqlx|postgres|diesel|tonic|openssl|ring|ed25519|secp256k1)(::|;|[[:space:]])'
if grep -RniE --include='*.rs' "$forbidden_use" "$RUST_ROOT/src" "$HOST_ROOT/src"; then
  fail 'runtime source imports a forbidden UI, network, persistence or signing capability'
fi

forbidden_authority_type='(^|[^A-Za-z0-9_])(MatchCompletedV1|NakamaAuthorityPrivateKey|CompletionSigningKey|CanonicalArchiveRootWriter)([^A-Za-z0-9_]|$)'
if grep -RniE --include='*.rs' "$forbidden_authority_type" "$RUST_ROOT/src" "$HOST_ROOT/src"; then
  fail 'runtime source declares or consumes a canonical online-authority type'
fi

grep -q 'pub trait WorldRulesetExecutor' "$RUST_ROOT/src/lib.rs" \
  || fail 'WorldRulesetExecutor boundary is missing'
grep -q 'pub struct RtsMissionExecutor' "$RUST_ROOT/src/lib.rs" \
  || fail 'the production deterministic RTS adapter is missing'
grep -q 'SHADOW_INPUT_VERSION' "$HOST_ROOT/src/lib.rs" \
  || fail 'shadow input contract is not implemented'
grep -q 'request_hash_mismatch' "$HOST_ROOT/src/lib.rs" \
  || fail 'shadow comparison does not bind the exact request'
grep -q 'output_contract_violation' "$HOST_ROOT/src/lib.rs" \
  || fail 'shadow comparison does not reject self-inconsistent output'
grep -q 'RtsMissionExecutor::new' "$HOST_ROOT/src/bin/trnm-world-runtime-exec.rs" \
  || fail 'execution binary is not wired to the Bevy-free RTS executor'
grep -q 'compare_shadow_value' "$HOST_ROOT/src/bin/trnm-world-runtime-shadow-diff.rs" \
  || fail 'shadow-diff binary is not wired to the strict comparator'

python3 - \
  "$V1_ROOT/trnm-world-runtime-v1.schema.json" \
  "$V1_ROOT/trnm-world-shadow-v1.schema.json" \
  "$V1_ROOT/golden-vectors.json" \
  "$V1_ROOT/shadow-vectors.json" \
  "$V1_ROOT/error-catalog.json" \
  "$V1_ROOT/compatibility-matrix.json" <<'PY'
import json
import pathlib
import sys

runtime_schema, shadow_schema, runtime_vectors, shadow_vectors, errors, matrix = [
    json.loads(pathlib.Path(path).read_text()) for path in sys.argv[1:]
]
if runtime_schema.get("$schema") != "https://json-schema.org/draft/2020-12/schema":
    raise SystemExit("runtime schema draft drift")
if "executeRequest" not in runtime_schema.get("$defs", {}) or "executeResult" not in runtime_schema.get("$defs", {}):
    raise SystemExit("runtime schema request/result definitions are incomplete")
if shadow_schema.get("$schema") != "https://json-schema.org/draft/2020-12/schema":
    raise SystemExit("shadow schema draft drift")
required_shadow_defs = {"runtimeObservation", "shadowInput", "shadowReport", "runtimeError"}
if not required_shadow_defs.issubset(shadow_schema.get("$defs", {})):
    raise SystemExit("shadow schema definitions are incomplete")
if runtime_vectors.get("contract_version") != "trnm_world_runtime_golden_vectors_v1":
    raise SystemExit("runtime vector version drift")
if len(runtime_vectors.get("canonicalization_vectors", [])) < 4 or len(runtime_vectors.get("negative_vectors", [])) < 5:
    raise SystemExit("runtime vector set is incomplete")
if shadow_vectors.get("contract_version") != "trnm_world_shadow_golden_vectors_v1":
    raise SystemExit("shadow vector version drift")
if len(shadow_vectors.get("vectors", [])) < 6:
    raise SystemExit("shadow vector set is incomplete")
if errors.get("contract_version") != "trnm_world_runtime_error_catalog_v1":
    raise SystemExit("error catalogue version drift")
codes = [entry.get("code") for entry in errors.get("errors", [])]
required_codes = {
    "unsupported_contract",
    "invalid_contract",
    "invalid_canonical_json",
    "resource_limit_exceeded",
    "ruleset_unavailable",
    "content_unavailable",
    "ordinal_discontinuity",
    "invalid_game_state",
    "invalid_game_command",
    "deterministic_execution_failed",
    "output_contract_violation",
    "authority_boundary_violation",
    "invalid_host_configuration",
}
if len(codes) != len(set(codes)) or not required_codes.issubset(codes):
    raise SystemExit("runtime error catalogue is incomplete or duplicated")
if len(errors.get("shadow_divergence_codes", [])) < 16:
    raise SystemExit("shadow divergence catalogue is incomplete")
if matrix.get("contract_version") != "trnm_world_runtime_compatibility_matrix_v1":
    raise SystemExit("compatibility matrix version drift")
flags = matrix.get("flags", {})
if flags.get("public_online_enabled") is not False:
    raise SystemExit("public online must remain disabled")
if flags.get("public_player_market_enabled") is not False:
    raise SystemExit("public player market must remain disabled")
if flags.get("canonical_cutover_complete") is not False:
    raise SystemExit("canonical cutover cannot be claimed from World source")
if flags.get("active_match_takeover_claimed") is not False:
    raise SystemExit("active-match takeover cannot be inferred")
nakama = matrix.get("consumers", {}).get("nakama", {})
if nakama.get("status") != "pending_independent_consumer":
    raise SystemExit("independent Nakama consumer must remain pending until external evidence exists")
if nakama.get("owns_match_completed_signing") is not True:
    raise SystemExit("Nakama completion-signing ownership drift")
world_enclave = matrix.get("world_compatibility_enclave", {})
if world_enclave.get("new_public_canonical_admission_allowed") is not False:
    raise SystemExit("World compatibility enclave cannot admit new public canonical matches")
if world_enclave.get("completion_signing_allowed") is not False:
    raise SystemExit("World compatibility enclave cannot sign canonical completion")
PY

python3 -m py_compile \
  "$ROOT_DIR/scripts/verify-trnm-world-runtime-v1.py" \
  "$ROOT_DIR/scripts/verify-trnm-world-shadow-v1.py"
python3 "$ROOT_DIR/scripts/verify-trnm-world-runtime-v1.py" >/dev/null
python3 "$ROOT_DIR/scripts/verify-trnm-world-shadow-v1.py" >/dev/null

printf '%s\n' 'TRNM World runtime adapter, shadow and authority boundary passed.'
