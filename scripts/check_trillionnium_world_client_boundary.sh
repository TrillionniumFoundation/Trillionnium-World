#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
SUMMARY_FILE="$ACCEPTANCE_DIR/client-boundary-cleanliness.json"
if [[ -v TRILLIONNIUM_WORLD_CLIENT_BOUNDARY_SUMMARY && -n "$TRILLIONNIUM_WORLD_CLIENT_BOUNDARY_SUMMARY" ]]; then
  SUMMARY_FILE="$TRILLIONNIUM_WORLD_CLIENT_BOUNDARY_SUMMARY"
fi

CONTRACT_VERSION="trillionnium_world_client_boundary_v1"
README="$ROOT/README.md"
UNIFIED_DOC="$ROOT/docs/development/trillionnium-world-unified-development-doc-v1.md"
RELEASE_READINESS="$ROOT/RELEASE_READINESS.md"
DEV_ENV="$ROOT/config/trillionnium-world-dev.env.example"
BEVY_RUNNER="$ROOT/scripts/run_trillionnium_world_bevy_client.sh"
BEVY_CRATE="$ROOT/trillionnium/crates/trnm-world-bevy"
CEX_ADAPTER_GATE="$ROOT/scripts/check_trillionnium_world_cex_adapter_readiness.sh"

mkdir -p "$(dirname "$SUMMARY_FILE")"

FAILURES=()
CHECKS=()

record_check() {
  local name="$1"
  local status="$2"
  local detail="$3"
  CHECKS+=("$name|$status|$detail")
  if [[ "$status" != "ok" ]]; then
    FAILURES+=("$name: $detail")
  fi
}

contains() {
  local file="$1"
  local needle="$2"
  local name="$3"
  if grep -Fq -- "$needle" "$file"; then
    record_check "$name" ok "found"
  else
    record_check "$name" fail "missing '$needle' in $file"
  fi
}

not_matches() {
  local path="$1"
  local pattern="$2"
  local name="$3"
  local tmp
  tmp="$(mktemp)"
  if rg -n --hidden --glob '!target/**' --glob '!acceptance/**' --glob '!.git/**' "$pattern" "$path" >"$tmp" 2>/dev/null; then
    record_check "$name" fail "$(tr '\n' ';' <"$tmp" | cut -c 1-500)"
  else
    record_check "$name" ok "absent"
  fi
  rm -f "$tmp"
}

if [[ -x "$BEVY_RUNNER" ]]; then
  record_check bevy_runner_executable ok "$BEVY_RUNNER"
else
  record_check bevy_runner_executable fail "$BEVY_RUNNER is missing or not executable"
fi

contains "$README" "Trillionnium World Client Boundary" readme_boundary_section
contains "$README" "Native playable client:" readme_native_client
contains "$README" "CEX is a legacy incubator/evidence adapter only" readme_cex_evidence_only
contains "$README" "scripts/check_trillionnium_world_client_boundary.sh" readme_boundary_gate

contains "$UNIFIED_DOC" "$CONTRACT_VERSION" unified_doc_contract
contains "$UNIFIED_DOC" "trnm-world-bevy" unified_doc_bevy_client_owner
contains "$UNIFIED_DOC" "CEX 只能作为 legacy evidence adapter / migration reference" unified_doc_cex_reference_only
contains "$UNIFIED_DOC" "账号注册、登录、profile、session、revoke" unified_doc_account_migration_boundary
contains "$UNIFIED_DOC" "scripts/run_trillionnium_world_bevy_client.sh" unified_doc_manual_playtest_entry

contains "$RELEASE_READINESS" "Client boundary gate" release_readiness_boundary_gate
contains "$RELEASE_READINESS" "CEX is legacy adapter evidence only" release_readiness_cex_evidence_only

contains "$DEV_ENV" "Legacy evidence adapter only" dev_env_cex_adapter_comment
contains "$DEV_ENV" "Do not launch CEX as the player client" dev_env_no_cex_client_comment

contains "$BEVY_CRATE/Cargo.toml" "minifb = \"0.27\"" bevy_crate_has_classic_low_spec_renderer
contains "$BEVY_RUNNER" "TRNM_WORLD_BEVY_CLASSIC_RENDERER=1" bevy_runner_defaults_to_classic_low_spec_renderer
contains "$BEVY_RUNNER" "TRNM_WORLD_BEVY_CLASSIC_FPS:-30" bevy_runner_caps_classic_low_spec_fps
contains "$BEVY_RUNNER" "cargo build -p trnm-world-bevy --release" bevy_runner_builds_native_crate
contains "$BEVY_RUNNER" "target/release/trnm-world-bevy" bevy_runner_launches_optimized_native_binary
not_matches "$BEVY_RUNNER" "CEX|consumer-entry|consumer_entry|runtime-manager-linux|/world" bevy_runner_has_no_cex_runtime_refs
not_matches "$BEVY_CRATE" "CEX|consumer-entry|consumer_entry|cex_default|cex_world|cex_consumer|cex_service|cex_incubator" bevy_crate_has_no_cex_internals
contains "$CEX_ADAPTER_GATE" "trillionnium_world_crates_do_not_import_cex_service_internals" cex_adapter_import_rule

CHECKS_JSON="$(printf '%s\n' "${CHECKS[@]}" | jq -R 'split("|") | {name: .[0], status: .[1], detail: .[2]}' | jq -s '.')"
FAILURES_JSON="$(printf '%s\n' "${FAILURES[@]}" | jq -R 'select(length > 0)' | jq -s '.')"
FAILURE_COUNT="$(jq 'length' <<<"$FAILURES_JSON")"

GREEN=false
STATUS=trillionnium_world_client_boundary_blocked
if [[ "$FAILURE_COUNT" == "0" ]]; then
  GREEN=true
  STATUS=trillionnium_world_client_boundary_green
fi

jq -n \
  --arg contract_version "$CONTRACT_VERSION" \
  --arg status "$STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg native_client_crate "trillionnium/crates/trnm-world-bevy" \
  --arg manual_playtest_entry "scripts/run_trillionnium_world_bevy_client.sh" \
  --arg cex_boundary "legacy_adapter_evidence_and_migration_reference_only_not_player_client" \
  --arg account_boundary "account_logic_may_migrate_from_cex_but_product_api_must_be_trillionnium_owned_and_consumed_by_trnm_world_bevy" \
  --argjson green "$GREEN" \
  --argjson checks "$CHECKS_JSON" \
  --argjson failures "$FAILURES_JSON" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_client_boundary_gate",
    green: $green,
    native_client_crate: $native_client_crate,
    manual_playtest_entry: $manual_playtest_entry,
    cex_boundary: $cex_boundary,
    account_boundary: $account_boundary,
    player_client_owner: "trnm-world-bevy",
    cex_runtime_player_client_allowed: false,
    checks: $checks,
    failures: $failures
  }' >"$SUMMARY_FILE"

if [[ "$GREEN" == "true" ]]; then
  printf 'TRILLIONNIUM_WORLD_CLIENT_BOUNDARY_GREEN %s\n' "$SUMMARY_FILE"
else
  printf 'TRILLIONNIUM_WORLD_CLIENT_BOUNDARY_BLOCKED %s\n' "$SUMMARY_FILE" >&2
  exit 1
fi
