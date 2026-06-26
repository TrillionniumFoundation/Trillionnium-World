#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
SUMMARY_FILE="${TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_BLOCKER_CONSISTENCY_SUMMARY:-$ACCEPTANCE_DIR/public-launch-blocker-consistency.json}"
READINESS_SUMMARY="$ACCEPTANCE_DIR/public-launch-readiness.json"
INTAKE_SUMMARY="$ACCEPTANCE_DIR/public-launch-evidence-intake.json"
READINESS_LOG="$ACCEPTANCE_DIR/public-launch-blocker-consistency-readiness.log"
INTAKE_LOG="$ACCEPTANCE_DIR/public-launch-blocker-consistency-intake.log"
CHECKS_FILE="$(mktemp)"
trap 'rm -f "$CHECKS_FILE"' EXIT

# shellcheck source=scripts/release_review_acceptance_lock.sh
source "$ROOT/scripts/release_review_acceptance_lock.sh"
trnm_acquire_release_review_acceptance_lock "$ACCEPTANCE_DIR"

"$ROOT/scripts/check_trillionnium_world_public_launch_readiness.sh" >"$READINESS_LOG"
"$ROOT/scripts/check_trillionnium_world_public_launch_evidence_intake.sh" >"$INTAKE_LOG"

add_check() {
  local name="$1"
  local status="$2"
  local detail="$3"
  local expected="${4:-}"
  local actual="${5:-}"
  jq -nc \
    --arg name "$name" \
    --arg status "$status" \
    --arg detail "$detail" \
    --arg expected "$expected" \
    --arg actual "$actual" \
    '{
      name: $name,
      status: $status,
      detail: $detail,
      expected: (if $expected == "" then null else $expected end),
      actual: (if $actual == "" then null else $actual end)
    }' >>"$CHECKS_FILE"
}

json_field() {
  local path="$1"
  local expr="$2"
  if [[ -f "$path" ]]; then
    jq -r "$expr // empty" "$path" 2>/dev/null || true
  fi
}

json_bool() {
  local path="$1"
  local expr="$2"
  if [[ -f "$path" ]] && jq -e "$expr" "$path" >/dev/null; then
    printf 'true'
  else
    printf 'false'
  fi
}

KNOWN_BLOCKERS_JSON='[
  "s5_real_device_matrix",
  "production_map_pack_public_evidence",
  "first_beta_cohort_evidence",
  "commercial_launch_drill_evidence",
  "multi_node_or_live_traffic_latency_evidence",
  "public_network_live_exposure_evidence"
]'

if [[ -f "$READINESS_SUMMARY" ]]; then
  add_check readiness_summary_present ok "public launch readiness summary exists" "$READINESS_SUMMARY" "$READINESS_SUMMARY"
else
  add_check readiness_summary_present fail "public launch readiness summary missing" "$READINESS_SUMMARY"
fi

if [[ -f "$INTAKE_SUMMARY" ]]; then
  add_check intake_summary_present ok "public launch evidence intake summary exists" "$INTAKE_SUMMARY" "$INTAKE_SUMMARY"
else
  add_check intake_summary_present fail "public launch evidence intake summary missing" "$INTAKE_SUMMARY"
fi

UNKNOWN_BLOCKERS_JSON="$(jq -c --argjson known "$KNOWN_BLOCKERS_JSON" '(.blockers // []) | map(select(($known | index(.)) == null))' "$READINESS_SUMMARY")"
UNKNOWN_BLOCKER_COUNT="$(jq 'length' <<<"$UNKNOWN_BLOCKERS_JSON")"
KNOWN_BLOCKER_COUNT="$(jq 'length' <<<"$KNOWN_BLOCKERS_JSON")"
READINESS_BLOCKER_COUNT="$(jq '(.blockers // []) | length' "$READINESS_SUMMARY")"
if [[ "$UNKNOWN_BLOCKER_COUNT" == "0" ]]; then
  add_check unknown_readiness_blockers ok "all readiness blockers are in the public launch blocker catalog"
else
  add_check unknown_readiness_blockers fail "readiness has blockers outside the catalog" "[]" "$UNKNOWN_BLOCKERS_JSON"
fi

UNKNOWN_INTAKE_JSON="$(jq -c --argjson known "$KNOWN_BLOCKERS_JSON" '(.needs_collection // []) | map(.blocker_id) | map(select(($known | index(.)) == null))' "$INTAKE_SUMMARY")"
UNKNOWN_INTAKE_COUNT="$(jq 'length' <<<"$UNKNOWN_INTAKE_JSON")"
INTAKE_NEEDS_COLLECTION_COUNT="$(jq '(.needs_collection // []) | length' "$INTAKE_SUMMARY")"
if [[ "$UNKNOWN_INTAKE_COUNT" == "0" ]]; then
  add_check unknown_intake_blockers ok "all intake needs_collection items map to known blockers"
else
  add_check unknown_intake_blockers fail "intake needs_collection has unknown blocker ids" "[]" "$UNKNOWN_INTAKE_JSON"
fi

check_item() {
  local blocker_id="$1"
  local intake_id="$2"
  local validator_path="$3"
  local status_expr="$4"
  local accepted_status="$5"

  local blocker_present
  blocker_present="$(jq -r --arg id "$blocker_id" '(.blockers // []) | index($id) != null' "$READINESS_SUMMARY")"
  local intake_json
  intake_json="$(jq -c --arg id "$intake_id" '.evidence_items[]? | select(.id == $id)' "$INTAKE_SUMMARY")"
  local validator_status
  validator_status="$(json_field "$validator_path" "$status_expr")"

  if [[ ! -f "$validator_path" ]]; then
    add_check "${blocker_id}_validator_present" fail "validator summary missing" "$validator_path"
  else
    add_check "${blocker_id}_validator_present" ok "validator summary exists" "$validator_path" "$validator_path"
  fi

  if [[ -z "$intake_json" ]]; then
    add_check "${blocker_id}_intake_item" fail "intake item missing" "$intake_id"
    return
  fi

  local intake_green intake_blocker intake_gate
  intake_green="$(jq -r '.green // false' <<<"$intake_json")"
  intake_blocker="$(jq -r '.blocker_id // empty' <<<"$intake_json")"
  intake_gate="$(jq -r '.blocked_by_public_launch_gate // false' <<<"$intake_json")"

  if [[ "$blocker_present" == "true" ]]; then
    if [[ "$intake_green" == "false" && "$intake_blocker" == "$blocker_id" && "$intake_gate" == "true" && "$validator_status" != "$accepted_status" ]]; then
      add_check "${blocker_id}_blocked_consistency" ok "readiness blocker, intake item, and validator blocked status agree" "$accepted_status" "$validator_status"
    else
      add_check "${blocker_id}_blocked_consistency" fail "readiness blocker does not match intake/validator state" "blocked intake and non-$accepted_status" "intake_green=$intake_green intake_blocker=$intake_blocker intake_gate=$intake_gate validator_status=$validator_status"
    fi
  else
    if [[ "$intake_green" == "true" && "$validator_status" == "$accepted_status" ]]; then
      add_check "${blocker_id}_green_consistency" ok "cleared blocker has green intake and validator status" "$accepted_status" "$validator_status"
    else
      add_check "${blocker_id}_green_consistency" fail "cleared blocker is not backed by green intake/validator state" "$accepted_status" "intake_green=$intake_green validator_status=$validator_status"
    fi
  fi
}

check_item \
  s5_real_device_matrix \
  s5_android_real_device_matrix \
  "$ROOT/acceptance/S5_native_bevy_device/latest/s5-real-device-evidence-validation.json" \
  '.status' \
  s5_real_device_evidence_green

check_item \
  production_map_pack_public_evidence \
  production_map_pack_public_evidence \
  "$ROOT/acceptance/S4_map_pack_gate/latest/production-map-pack-public-evidence.json" \
  '.status' \
  production_map_pack_public_ready_green

check_item \
  first_beta_cohort_evidence \
  first_beta_cohort_evidence \
  "$ACCEPTANCE_DIR/cohort-commercial-evidence.json" \
  '.first_beta.status' \
  first_beta_cohort_evidence_green

check_item \
  commercial_launch_drill_evidence \
  commercial_launch_drill_evidence \
  "$ACCEPTANCE_DIR/cohort-commercial-evidence.json" \
  '.commercial_launch_drill.status' \
  commercial_launch_drill_evidence_green

check_item \
  multi_node_or_live_traffic_latency_evidence \
  multi_node_or_live_traffic_latency_evidence \
  "$ACCEPTANCE_DIR/external-ops-evidence.json" \
  '.multi_node_or_live_traffic_latency.status' \
  multi_node_or_live_traffic_latency_green

check_item \
  public_network_live_exposure_evidence \
  public_network_live_exposure_evidence \
  "$ACCEPTANCE_DIR/external-ops-evidence.json" \
  '.public_network_deploy.status' \
  public_network_deploy_green

CHECKS_JSON="$(jq -s '.' "$CHECKS_FILE")"
FAILURES_JSON="$(jq -c '[.[] | select(.status != "ok")]' <<<"$CHECKS_JSON")"
FAILURE_COUNT="$(jq 'length' <<<"$FAILURES_JSON")"
PUBLIC_LAUNCH_READY="$(jq -r '.public_launch_ready // (.overall_status == "ready_for_public_launch_review")' "$READINESS_SUMMARY" 2>/dev/null || printf 'false')"

STATUS=public_launch_blocker_consistency_blocked
if [[ "$FAILURE_COUNT" == "0" ]]; then
  if [[ "$PUBLIC_LAUNCH_READY" == "true" ]]; then
    STATUS=public_launch_blocker_consistency_green
  else
    STATUS=public_launch_blocker_consistency_green_with_public_launch_blockers
  fi
fi

jq -n \
  --arg contract_version "trillionnium_world_public_launch_blocker_consistency_v1" \
  --arg status "$STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg readiness_summary "$READINESS_SUMMARY" \
  --arg readiness_log "$READINESS_LOG" \
  --arg intake_summary "$INTAKE_SUMMARY" \
  --arg intake_log "$INTAKE_LOG" \
  --argjson public_launch_ready "$PUBLIC_LAUNCH_READY" \
  --argjson known_blockers "$KNOWN_BLOCKERS_JSON" \
  --argjson known_blocker_count "$KNOWN_BLOCKER_COUNT" \
  --argjson readiness_blocker_count "$READINESS_BLOCKER_COUNT" \
  --argjson intake_needs_collection_count "$INTAKE_NEEDS_COLLECTION_COUNT" \
  --argjson unknown_readiness_blocker_count "$UNKNOWN_BLOCKER_COUNT" \
  --argjson unknown_intake_blocker_count "$UNKNOWN_INTAKE_COUNT" \
  --argjson unknown_readiness_blockers "$UNKNOWN_BLOCKERS_JSON" \
  --argjson unknown_intake_blockers "$UNKNOWN_INTAKE_JSON" \
  --argjson checks "$CHECKS_JSON" \
  --argjson failures "$FAILURES_JSON" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_public_launch_blocker_consistency",
    green: ($failures | length == 0),
    public_launch_ready: $public_launch_ready,
    public_launch_claimed: false,
    consistency_rule: "public_launch_readiness_blockers_must_match_evidence_intake_items_and_field_level_validator_statuses",
    readiness: {summary_path: $readiness_summary, refresh_log_path: $readiness_log},
    intake: {summary_path: $intake_summary, refresh_log_path: $intake_log},
    blockers: $known_blockers,
    known_blockers: $known_blockers,
    known_blocker_count: $known_blocker_count,
    readiness_blocker_count: $readiness_blocker_count,
    intake_needs_collection_count: $intake_needs_collection_count,
    unknown_readiness_blocker_count: $unknown_readiness_blocker_count,
    unknown_intake_blocker_count: $unknown_intake_blocker_count,
    unknown_readiness_blockers: $unknown_readiness_blockers,
    unknown_intake_blockers: $unknown_intake_blockers,
    check_count: ($checks | length),
    failed_check_count: ($failures | length),
    checks: $checks,
    failures: $failures
  }' >"$SUMMARY_FILE"

case "$STATUS" in
  public_launch_blocker_consistency_green|public_launch_blocker_consistency_green_with_public_launch_blockers)
    printf 'TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_BLOCKER_CONSISTENCY_GREEN %s\n' "$SUMMARY_FILE"
    ;;
  *)
    printf 'TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_BLOCKER_CONSISTENCY_BLOCKED %s\n' "$SUMMARY_FILE" >&2
    exit 1
    ;;
esac
