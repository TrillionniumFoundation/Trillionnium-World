#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
SUMMARY_FILE="${TRILLIONNIUM_WORLD_COHORT_COMMERCIAL_EVIDENCE_SUMMARY:-$ACCEPTANCE_DIR/cohort-commercial-evidence.json}"
SCHEMA_LOG="$ACCEPTANCE_DIR/cohort-commercial-evidence-schema-refresh.log"
COHORT_EVIDENCE_PATH="${TRILLIONNIUM_FIRST_BETA_COHORT_EVIDENCE_PATH:-}"
COMMERCIAL_EVIDENCE_PATH="${TRILLIONNIUM_COMMERCIAL_LAUNCH_DRILL_EVIDENCE_PATH:-}"
REQUIRE_READY=0

usage() {
  cat <<'EOF_USAGE'
Usage: scripts/check_trillionnium_world_cohort_commercial_evidence.sh [--require-ready]

Validates real first-beta cohort evidence and real/sanitized commercial launch
drill evidence. Templates and collection checklists do not grant public-launch
credit.

Collection checklist:
  scripts/check_trillionnium_world_cohort_commercial_evidence_collection.sh

Strict validation:
  TRILLIONNIUM_FIRST_BETA_COHORT_EVIDENCE_PATH=<real-cohort.json> TRILLIONNIUM_COMMERCIAL_LAUNCH_DRILL_EVIDENCE_PATH=<real-commercial-drill.json> scripts/check_trillionnium_world_cohort_commercial_evidence.sh --require-ready
EOF_USAGE
}

for arg in "$@"; do
  case "$arg" in
    --help|-h)
      usage
      exit 0
      ;;
    --require-ready)
      REQUIRE_READY=1
      ;;
    *)
      printf 'unknown option: %s\n' "$arg" >&2
      exit 2
      ;;
  esac
done

mkdir -p "$ACCEPTANCE_DIR"

"$ROOT/scripts/check_trillionnium_world_cohort_commercial_schema.sh" >"$SCHEMA_LOG"

read_json_field() {
  local path="$1"
  local expr="$2"
  if [[ -f "$path" ]]; then
    jq -r "$expr // empty" "$path" 2>/dev/null || true
  fi
}

file_status() {
  local path="$1"
  if [[ -n "$path" && -f "$path" ]]; then
    printf 'present'
  else
    printf 'missing'
  fi
}

json_array_from_lines() {
  jq -Rsc 'split("\n") | map(select(length > 0))'
}

COHORT_BLOCKERS=()
COHORT_FILE_STATUS="$(file_status "$COHORT_EVIDENCE_PATH")"
COHORT_CONTRACT="$(read_json_field "$COHORT_EVIDENCE_PATH" '.contract_version')"
COHORT_STATUS_RAW="$(read_json_field "$COHORT_EVIDENCE_PATH" '.status')"
COHORT_PARTICIPANT_COUNT="$(read_json_field "$COHORT_EVIDENCE_PATH" '.participant_count')"
COHORT_PARTICIPANTS_LEN="$(read_json_field "$COHORT_EVIDENCE_PATH" '(.participants // []) | length')"
COHORT_SESSIONS_LEN="$(read_json_field "$COHORT_EVIDENCE_PATH" '(.sessions // []) | length')"
COHORT_REAL_SIGNOFF="$(read_json_field "$COHORT_EVIDENCE_PATH" '.operator_signoff.real_participants_confirmed == true')"
COHORT_REJECTS_SYNTHETIC="$(read_json_field "$COHORT_EVIDENCE_PATH" '.operator_signoff.synthetic_or_template_data_rejected == true')"
COHORT_SIGNED_BY="$(read_json_field "$COHORT_EVIDENCE_PATH" '.operator_signoff.signed_by')"
COHORT_SIGNED_AT="$(read_json_field "$COHORT_EVIDENCE_PATH" '.operator_signoff.signed_at')"

[[ "$COHORT_FILE_STATUS" == "present" ]] || COHORT_BLOCKERS+=("first_beta_evidence_file")
[[ "$COHORT_CONTRACT" == "trillionnium_world_first_beta_cohort_evidence_v1" ]] || COHORT_BLOCKERS+=("first_beta_contract")
[[ "$COHORT_STATUS_RAW" == "first_beta_cohort_evidence_green" ]] || COHORT_BLOCKERS+=("first_beta_status")
if [[ -z "$COHORT_PARTICIPANT_COUNT" || "$COHORT_PARTICIPANT_COUNT" -lt 5 || "$COHORT_PARTICIPANT_COUNT" -gt 10 ]]; then
  COHORT_BLOCKERS+=("participant_count_5_to_10")
fi
if [[ -z "$COHORT_PARTICIPANTS_LEN" || -z "$COHORT_PARTICIPANT_COUNT" || "$COHORT_PARTICIPANTS_LEN" != "$COHORT_PARTICIPANT_COUNT" ]]; then
  COHORT_BLOCKERS+=("participants_match_count")
fi
if [[ -z "$COHORT_SESSIONS_LEN" || -z "$COHORT_PARTICIPANT_COUNT" || "$COHORT_SESSIONS_LEN" -lt "$COHORT_PARTICIPANT_COUNT" ]]; then
  COHORT_BLOCKERS+=("session_count_covers_participants")
fi
[[ "$COHORT_REAL_SIGNOFF" == "true" ]] || COHORT_BLOCKERS+=("real_participants_signoff")
[[ "$COHORT_REJECTS_SYNTHETIC" == "true" ]] || COHORT_BLOCKERS+=("synthetic_cohort_rejected")
[[ -n "$COHORT_SIGNED_BY" && -n "$COHORT_SIGNED_AT" ]] || COHORT_BLOCKERS+=("first_beta_operator_signature")

COHORT_BLOCKERS_JSON="$(printf '%s\n' "${COHORT_BLOCKERS[@]}" | json_array_from_lines)"
COHORT_STATUS="first_beta_cohort_evidence_green"
if [[ "$(jq 'length' <<<"$COHORT_BLOCKERS_JSON")" != "0" ]]; then
  COHORT_STATUS="blocked_missing_first_beta_cohort_evidence"
fi

COMMERCIAL_BLOCKERS=()
COMMERCIAL_FILE_STATUS="$(file_status "$COMMERCIAL_EVIDENCE_PATH")"
COMMERCIAL_CONTRACT="$(read_json_field "$COMMERCIAL_EVIDENCE_PATH" '.contract_version')"
COMMERCIAL_STATUS_RAW="$(read_json_field "$COMMERCIAL_EVIDENCE_PATH" '.status')"
COMMERCIAL_SIGNOFF_REAL="$(read_json_field "$COMMERCIAL_EVIDENCE_PATH" '.operator_signoff.real_or_sanitized_drill_confirmed == true')"
COMMERCIAL_REJECTS_SYNTHETIC="$(read_json_field "$COMMERCIAL_EVIDENCE_PATH" '.operator_signoff.synthetic_or_template_data_rejected == true')"
COMMERCIAL_SIGNED_BY="$(read_json_field "$COMMERCIAL_EVIDENCE_PATH" '.operator_signoff.signed_by')"
COMMERCIAL_SIGNED_AT="$(read_json_field "$COMMERCIAL_EVIDENCE_PATH" '.operator_signoff.signed_at')"

[[ "$COMMERCIAL_FILE_STATUS" == "present" ]] || COMMERCIAL_BLOCKERS+=("commercial_evidence_file")
[[ "$COMMERCIAL_CONTRACT" == "trillionnium_world_commercial_launch_drill_evidence_v1" ]] || COMMERCIAL_BLOCKERS+=("commercial_contract")
[[ "$COMMERCIAL_STATUS_RAW" == "commercial_launch_drill_evidence_green" ]] || COMMERCIAL_BLOCKERS+=("commercial_status")

DRILL_RESULTS_FILE="$(mktemp)"
trap 'rm -f "$DRILL_RESULTS_FILE"' EXIT
for drill in payment refund support legal operator traffic; do
  drill_status="$(read_json_field "$COMMERCIAL_EVIDENCE_PATH" ".drills.$drill.status")"
  drill_evidence="$(read_json_field "$COMMERCIAL_EVIDENCE_PATH" ".drills.$drill.evidence")"
  drill_green=false
  if [[ "$drill_status" == "green" || "$drill_status" == "passed" || "$drill_status" == "${drill}_green" || "$drill_status" == "${drill}_drill_green" ]] && [[ -n "$drill_evidence" && "$drill_evidence" != "null" ]]; then
    drill_green=true
  else
    COMMERCIAL_BLOCKERS+=("${drill}_drill_green_evidence")
  fi
  jq -nc \
    --arg drill "$drill" \
    --arg status "$drill_status" \
    --arg evidence "$drill_evidence" \
    --argjson green "$drill_green" \
    '{drill: $drill, status: (if $status == "" then "missing" else $status end), evidence: (if $evidence == "" then null else $evidence end), green: $green}' >>"$DRILL_RESULTS_FILE"
done
DRILL_RESULTS_JSON="$(jq -s '.' "$DRILL_RESULTS_FILE")"

[[ "$COMMERCIAL_SIGNOFF_REAL" == "true" ]] || COMMERCIAL_BLOCKERS+=("real_or_sanitized_commercial_signoff")
[[ "$COMMERCIAL_REJECTS_SYNTHETIC" == "true" ]] || COMMERCIAL_BLOCKERS+=("synthetic_commercial_rejected")
[[ -n "$COMMERCIAL_SIGNED_BY" && -n "$COMMERCIAL_SIGNED_AT" ]] || COMMERCIAL_BLOCKERS+=("commercial_operator_signature")

COMMERCIAL_BLOCKERS_JSON="$(printf '%s\n' "${COMMERCIAL_BLOCKERS[@]}" | json_array_from_lines)"
COMMERCIAL_STATUS="commercial_launch_drill_evidence_green"
if [[ "$(jq 'length' <<<"$COMMERCIAL_BLOCKERS_JSON")" != "0" ]]; then
  COMMERCIAL_STATUS="blocked_missing_commercial_launch_drill_evidence"
fi

STATUS="cohort_commercial_evidence_green"
if [[ "$COHORT_STATUS" != "first_beta_cohort_evidence_green" || "$COMMERCIAL_STATUS" != "commercial_launch_drill_evidence_green" ]]; then
  STATUS="blocked_missing_cohort_commercial_real_evidence"
fi

jq -n \
  --arg contract_version "trillionnium_world_cohort_commercial_evidence_gate_v1" \
  --arg status "$STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg schema_summary "$ACCEPTANCE_DIR/cohort-commercial-evidence-schema.json" \
  --arg schema_log "$SCHEMA_LOG" \
  --arg cohort_path "$COHORT_EVIDENCE_PATH" \
  --arg cohort_file_status "$COHORT_FILE_STATUS" \
  --arg cohort_contract "$COHORT_CONTRACT" \
  --arg cohort_raw_status "$COHORT_STATUS_RAW" \
  --arg cohort_status "$COHORT_STATUS" \
  --argjson cohort_participant_count "${COHORT_PARTICIPANT_COUNT:-0}" \
  --argjson cohort_participants_len "${COHORT_PARTICIPANTS_LEN:-0}" \
  --argjson cohort_sessions_len "${COHORT_SESSIONS_LEN:-0}" \
  --argjson cohort_blockers "$COHORT_BLOCKERS_JSON" \
  --arg commercial_path "$COMMERCIAL_EVIDENCE_PATH" \
  --arg commercial_file_status "$COMMERCIAL_FILE_STATUS" \
  --arg commercial_contract "$COMMERCIAL_CONTRACT" \
  --arg commercial_raw_status "$COMMERCIAL_STATUS_RAW" \
  --arg commercial_status "$COMMERCIAL_STATUS" \
  --argjson drill_results "$DRILL_RESULTS_JSON" \
  --argjson commercial_blockers "$COMMERCIAL_BLOCKERS_JSON" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_cohort_commercial_evidence_gate",
    public_launch_credit: "only_when_first_beta_and_commercial_statuses_are_green_after_field_validation",
    schema: {
      summary_path: $schema_summary,
      refresh_log_path: $schema_log
    },
    first_beta: {
      status: $cohort_status,
      accepted_status: "first_beta_cohort_evidence_green",
      operator_evidence: {
        path: (if $cohort_path == "" then null else $cohort_path end),
        file_status: $cohort_file_status,
        contract_version: $cohort_contract,
        status: $cohort_raw_status
      },
      participant_count: $cohort_participant_count,
      participants_len: $cohort_participants_len,
      sessions_len: $cohort_sessions_len,
      blockers: $cohort_blockers
    },
    commercial_launch_drill: {
      status: $commercial_status,
      accepted_status: "commercial_launch_drill_evidence_green",
      operator_evidence: {
        path: (if $commercial_path == "" then null else $commercial_path end),
        file_status: $commercial_file_status,
        contract_version: $commercial_contract,
        status: $commercial_raw_status
      },
      required_drills: $drill_results,
      blockers: $commercial_blockers
    }
  }' >"$SUMMARY_FILE"

if [[ "$STATUS" == "cohort_commercial_evidence_green" ]]; then
  printf 'TRILLIONNIUM_WORLD_COHORT_COMMERCIAL_EVIDENCE_READY %s\n' "$SUMMARY_FILE"
  exit 0
fi

printf 'TRILLIONNIUM_WORLD_COHORT_COMMERCIAL_EVIDENCE_BLOCKED %s %s\n' "$STATUS" "$SUMMARY_FILE"
if [[ "$REQUIRE_READY" -eq 1 ]]; then
  exit 1
fi
