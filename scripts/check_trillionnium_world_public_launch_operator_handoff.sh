#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
S5_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
MAP_DIR="$ROOT/acceptance/S4_map_pack_gate/latest"
SUMMARY_FILE="$ACCEPTANCE_DIR/public-launch-operator-handoff.json"
MARKDOWN_FILE="$ACCEPTANCE_DIR/public-launch-operator-handoff.md"
if [[ -v TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_OPERATOR_HANDOFF_SUMMARY && -n "$TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_OPERATOR_HANDOFF_SUMMARY" ]]; then
  SUMMARY_FILE="$TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_OPERATOR_HANDOFF_SUMMARY"
fi
if [[ -v TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_OPERATOR_HANDOFF_MD && -n "$TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_OPERATOR_HANDOFF_MD" ]]; then
  MARKDOWN_FILE="$TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_OPERATOR_HANDOFF_MD"
fi
REFRESH_INPUTS="${TRNM_PUBLIC_LAUNCH_OPERATOR_HANDOFF_REFRESH_INPUTS:-1}"

ARTIFACTS_FILE="$(mktemp)"
FAILURES_FILE="$(mktemp)"
trap 'rm -f "$ARTIFACTS_FILE" "$FAILURES_FILE"' EXIT

# shellcheck source=scripts/release_review_acceptance_lock.sh
source "$ROOT/scripts/release_review_acceptance_lock.sh"
trnm_acquire_release_review_acceptance_lock "$ACCEPTANCE_DIR"

mkdir -p "$S5_DIR" "$MAP_DIR"

STATUS_LOG="$ACCEPTANCE_DIR/public-launch-operator-handoff-status.log"
INTAKE_LOG="$ACCEPTANCE_DIR/public-launch-operator-handoff-intake.log"
KIT_LOG="$ACCEPTANCE_DIR/public-launch-operator-handoff-kit.log"
MAP_COLLECTION_LOG="$ACCEPTANCE_DIR/public-launch-operator-handoff-map-collection.log"
COHORT_COLLECTION_LOG="$ACCEPTANCE_DIR/public-launch-operator-handoff-cohort-commercial-collection.log"
EXTERNAL_OPS_COLLECTION_LOG="$ACCEPTANCE_DIR/public-launch-operator-handoff-external-ops-collection.log"
BLOCKER_CONSISTENCY_LOG="$ACCEPTANCE_DIR/public-launch-operator-handoff-blocker-consistency.log"
TEMPLATE_NEGATIVE_LOG="$ACCEPTANCE_DIR/public-launch-operator-handoff-template-negative-fixtures.log"
EVIDENCE_BUNDLE_LOG="$ACCEPTANCE_DIR/public-launch-operator-handoff-evidence-bundle.log"
BUNDLE_NEGATIVE_LOG="$ACCEPTANCE_DIR/public-launch-operator-handoff-bundle-negative-fixtures.log"

if [[ "$REFRESH_INPUTS" != "0" ]]; then
  "$ROOT/scripts/check_trillionnium_world_release_review_status.sh" >"$STATUS_LOG"
  "$ROOT/scripts/check_trillionnium_world_public_launch_evidence_intake.sh" >"$INTAKE_LOG"
  "$ROOT/scripts/check_trillionnium_world_public_launch_evidence_kit.sh" >"$KIT_LOG"
  "$ROOT/scripts/check_trillionnium_world_production_map_pack_public_evidence_collection.sh" >"$MAP_COLLECTION_LOG"
  "$ROOT/scripts/check_trillionnium_world_cohort_commercial_evidence_collection.sh" >"$COHORT_COLLECTION_LOG"
  "$ROOT/scripts/check_trillionnium_world_external_ops_evidence_collection.sh" >"$EXTERNAL_OPS_COLLECTION_LOG"
  "$ROOT/scripts/check_trillionnium_world_public_launch_blocker_consistency.sh" >"$BLOCKER_CONSISTENCY_LOG"
  "$ROOT/scripts/check_trillionnium_world_public_launch_template_negative_fixtures.sh" >"$TEMPLATE_NEGATIVE_LOG"
  "$ROOT/scripts/check_trillionnium_world_public_launch_evidence_bundle.sh" >"$EVIDENCE_BUNDLE_LOG"
  "$ROOT/scripts/check_trillionnium_world_public_launch_bundle_negative_fixtures.sh" >"$BUNDLE_NEGATIVE_LOG"
fi

STATUS_JSON="$ACCEPTANCE_DIR/release-review-status.json"
INTAKE_JSON="$ACCEPTANCE_DIR/public-launch-evidence-intake.json"
KIT_JSON="$ACCEPTANCE_DIR/public-launch-evidence-kit.json"
MAP_COLLECTION_JSON="$MAP_DIR/production-map-pack-public-evidence-collection.json"
COHORT_COLLECTION_JSON="$ACCEPTANCE_DIR/cohort-commercial-evidence-collection.json"
EXTERNAL_OPS_COLLECTION_JSON="$ACCEPTANCE_DIR/external-ops-evidence-collection.json"
BLOCKER_CONSISTENCY_JSON="$ACCEPTANCE_DIR/public-launch-blocker-consistency.json"
TEMPLATE_NEGATIVE_JSON="$ACCEPTANCE_DIR/public-launch-template-negative-fixtures.json"
EVIDENCE_BUNDLE_JSON="$ACCEPTANCE_DIR/public-launch-evidence-bundle.json"
EVIDENCE_BUNDLE_MD="$ACCEPTANCE_DIR/public-launch-evidence-bundle.md"
EVIDENCE_BUNDLE_TEMPLATE="$ACCEPTANCE_DIR/public-launch-evidence-bundle.template.json"
BUNDLE_NEGATIVE_JSON="$ACCEPTANCE_DIR/public-launch-bundle-negative-fixtures.json"

artifact() {
  local id="$1"
  local label="$2"
  local path="$3"
  local role="$4"
  local file_status="missing"
  local sha256=""
  local bytes=""
  local contract_version=""
  local status=""

  if [[ -f "$path" ]]; then
    file_status="present"
    sha256="$(sha256sum "$path" | awk '{print $1}')"
    bytes="$(wc -c <"$path" | tr -d ' ')"
    if [[ "$path" == *.json ]]; then
      contract_version="$(jq -r '.contract_version // empty' "$path" 2>/dev/null || true)"
      status="$(jq -r '.status // .overall_status // empty' "$path" 2>/dev/null || true)"
    fi
  fi

  jq -nc --arg id "$id" --arg label "$label" --arg path "$path" --arg role "$role" --arg file_status "$file_status" --arg sha256 "$sha256" --arg bytes "$bytes" --arg contract_version "$contract_version" --arg status "$status" '{
      id: $id,
      label: $label,
      path: $path,
      role: $role,
      file_status: $file_status,
      sha256: (if $sha256 == "" then null else $sha256 end),
      bytes: (if $bytes == "" then null else ($bytes | tonumber) end),
      contract_version: (if $contract_version == "" then null else $contract_version end),
      status: (if $status == "" then null else $status end)
    }' >>"$ARTIFACTS_FILE"
}

add_failure() {
  local id="$1"
  local detail="$2"
  local path=""
  if [[ "$#" -ge 3 ]]; then
    path="$3"
  fi
  jq -nc --arg id "$id" --arg detail "$detail" --arg path "$path" '{id: $id, detail: $detail, path: (if $path == "" then null else $path end)}' >>"$FAILURES_FILE"
}

require_json() {
  local id="$1"
  local path="$2"
  local expr="$3"
  local detail="$4"
  if [[ ! -f "$path" ]]; then
    add_failure "$id" "missing $detail" "$path"
  elif ! jq -e "$expr" "$path" >/dev/null; then
    add_failure "$id" "$detail" "$path"
  fi
}

artifact release_review_status_json "Release review status JSON" "$STATUS_JSON" operator_context
artifact release_review_status_markdown "Release review status Markdown" "$ACCEPTANCE_DIR/release-review-status.md" operator_context
artifact public_launch_evidence_intake_json "Public launch evidence intake JSON" "$INTAKE_JSON" operator_collection
artifact public_launch_evidence_intake_markdown "Public launch evidence intake Markdown" "$ACCEPTANCE_DIR/public-launch-evidence-intake.md" operator_collection
artifact public_launch_evidence_kit_json "Public launch evidence kit JSON" "$KIT_JSON" operator_collection
artifact public_launch_evidence_kit_markdown "Public launch evidence kit Markdown" "$ACCEPTANCE_DIR/public-launch-evidence-kit.md" operator_collection
artifact production_map_pack_public_collection_json "Production map-pack collection JSON" "$MAP_COLLECTION_JSON" operator_collection
artifact production_map_pack_public_collection_markdown "Production map-pack collection Markdown" "$MAP_DIR/production-map-pack-public-evidence-collection.md" operator_collection
artifact cohort_commercial_collection_json "Cohort/commercial collection JSON" "$COHORT_COLLECTION_JSON" operator_collection
artifact cohort_commercial_collection_markdown "Cohort/commercial collection Markdown" "$ACCEPTANCE_DIR/cohort-commercial-evidence-collection.md" operator_collection
artifact external_ops_collection_json "External ops collection JSON" "$EXTERNAL_OPS_COLLECTION_JSON" operator_collection
artifact external_ops_collection_markdown "External ops collection Markdown" "$ACCEPTANCE_DIR/external-ops-evidence-collection.md" operator_collection
artifact public_launch_blocker_consistency_json "Public launch blocker consistency JSON" "$BLOCKER_CONSISTENCY_JSON" operator_gate
artifact public_launch_template_negative_fixtures_json "Public launch template negative fixtures JSON" "$TEMPLATE_NEGATIVE_JSON" operator_gate
artifact public_launch_evidence_bundle_json "Public launch evidence bundle JSON" "$EVIDENCE_BUNDLE_JSON" operator_bundle
artifact public_launch_evidence_bundle_markdown "Public launch evidence bundle Markdown" "$EVIDENCE_BUNDLE_MD" operator_bundle
artifact public_launch_evidence_bundle_template "Public launch evidence bundle template" "$EVIDENCE_BUNDLE_TEMPLATE" operator_bundle
artifact public_launch_bundle_negative_fixtures_json "Public launch bundle negative fixtures JSON" "$BUNDLE_NEGATIVE_JSON" operator_gate
artifact s5_real_device_template "S5 real-device evidence template" "$S5_DIR/s5-device-evidence.template.json" operator_template
artifact production_map_pack_public_template "Production map-pack public evidence template" "$MAP_DIR/production-map-pack-public-evidence.template.json" operator_template
artifact first_beta_cohort_template "First beta cohort evidence template" "$ACCEPTANCE_DIR/first-beta-cohort-evidence.template.json" operator_template
artifact commercial_launch_drill_template "Commercial launch drill evidence template" "$ACCEPTANCE_DIR/commercial-launch-drill-evidence.template.json" operator_template
artifact multi_node_latency_template "Multi-node latency evidence template" "$ACCEPTANCE_DIR/multi-node-latency-evidence.template.json" operator_template
artifact public_network_deploy_template "Public network deploy evidence template" "$ACCEPTANCE_DIR/public-network-deploy-evidence.template.json" operator_template

require_json status_ready "$STATUS_JSON" '.ready_for_release_review == true and .public_launch_ready == false and .android_s5_real_device_claimed == false' "release review status must be review-ready while keeping public launch blocked and Android S5 unclaimed"
require_json intake_contract "$INTAKE_JSON" '.contract_version == "trillionnium_world_public_launch_evidence_intake_v1" and (.evidence_items // [] | length) == 6 and .public_launch_claimed == false and .android_s5_real_device_claimed == false and .live_map_ingestion_performed == false and .live_public_exposure_performed == false' "intake must expose six no-claim external evidence items"
require_json kit_contract "$KIT_JSON" '.contract_version == "trillionnium_world_public_launch_evidence_kit_v1" and .green == true and (.evidence_items // [] | length) == 6 and .public_launch_claimed == false and .android_s5_real_device_claimed == false' "kit must expose six valid no-credit templates and validator commands"
require_json blocker_consistency "$BLOCKER_CONSISTENCY_JSON" '.contract_version == "trillionnium_world_public_launch_blocker_consistency_v1" and (.failures // [] | length) == 0 and (.unknown_intake_blockers // [] | length) == 0 and (.unknown_readiness_blockers // [] | length) == 0 and .public_launch_ready == false' "readiness blockers, intake items, and validators must stay consistent"
require_json template_negative "$TEMPLATE_NEGATIVE_JSON" '.contract_version == "trillionnium_world_public_launch_template_negative_fixtures_v1" and .green == true and .status == "public_launch_template_negative_fixtures_green"' "no-credit templates must fail strict validators"
require_json bundle_gate "$EVIDENCE_BUNDLE_JSON" '.contract_version == "trillionnium_world_public_launch_evidence_bundle_gate_v1" and (.status == "public_launch_evidence_bundle_ready_for_real_evidence" or .status == "public_launch_evidence_bundle_green") and .public_launch_claimed == false and .android_s5_real_device_claimed == false' "bundle gate must be ready for real evidence without claiming public launch"
require_json bundle_negative "$BUNDLE_NEGATIVE_JSON" '.contract_version == "trillionnium_world_public_launch_bundle_negative_fixtures_v1" and .green == true and .status == "public_launch_bundle_negative_fixtures_green"' "fake-green bundle must be rejected"

ARTIFACTS_JSON="$(jq -s '.' "$ARTIFACTS_FILE")"
MISSING_ARTIFACTS_JSON="$(jq -c '[.[] | select(.file_status != "present") | .id]' <<<"$ARTIFACTS_JSON")"
MISSING_ARTIFACT_COUNT="$(jq 'length' <<<"$MISSING_ARTIFACTS_JSON")"
if [[ "$MISSING_ARTIFACT_COUNT" != "0" ]]; then
  add_failure missing_handoff_artifacts "operator handoff artifact paths must exist" "$SUMMARY_FILE"
fi

FAILURES_JSON="$(jq -s '.' "$FAILURES_FILE")"
FAILURE_COUNT="$(jq 'length' <<<"$FAILURES_JSON")"
READY_FOR_RELEASE_REVIEW="$(jq -r '.ready_for_release_review // false' "$STATUS_JSON")"
PUBLIC_LAUNCH_READY="$(jq -r '.public_launch_ready // false' "$STATUS_JSON")"
NEEDS_COLLECTION_COUNT="$(jq -r '(.needs_collection // []) | length' "$INTAKE_JSON")"
OPERATOR_ACTIONS_JSON="$(jq -c '[.evidence_items[] | {
  id,
  blocker_id,
  label,
  evidence_env_var,
  template_path,
  template_sha256,
  collection_command,
  validator_command,
  validator_summary,
  accepted_status,
  current_status,
  collection_requirement,
  template_public_launch_credit
}]' "$KIT_JSON")"
BLOCKED_ITEMS_JSON="$(jq -c '.blocked_items // []' "$STATUS_JSON")"
KNOWN_BLOCKERS_JSON="$(jq -c '.known_blockers // []' "$BLOCKER_CONSISTENCY_JSON")"

GREEN=false
STATUS=public_launch_operator_handoff_blocked
if [[ "$FAILURE_COUNT" == "0" && "$READY_FOR_RELEASE_REVIEW" == "true" ]]; then
  GREEN=true
  if [[ "$PUBLIC_LAUNCH_READY" == "true" ]]; then
    STATUS=public_launch_operator_handoff_complete_green
  else
    STATUS=public_launch_operator_handoff_ready_with_external_blockers
  fi
fi

jq -n --arg contract_version "trillionnium_world_public_launch_operator_handoff_v1" --arg status "$STATUS" --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" --arg markdown_path "$MARKDOWN_FILE" --arg evidence_bundle_template "$EVIDENCE_BUNDLE_TEMPLATE" --argjson green "$GREEN" --argjson ready_for_release_review "$READY_FOR_RELEASE_REVIEW" --argjson public_launch_ready "$PUBLIC_LAUNCH_READY" --argjson needs_collection_count "$NEEDS_COLLECTION_COUNT" --argjson operator_actions "$OPERATOR_ACTIONS_JSON" --argjson blocked_items "$BLOCKED_ITEMS_JSON" --argjson known_blockers "$KNOWN_BLOCKERS_JSON" --argjson handoff_artifacts "$ARTIFACTS_JSON" --argjson missing_artifacts "$MISSING_ARTIFACTS_JSON" --argjson failures "$FAILURES_JSON" '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_public_launch_operator_handoff",
    green: $green,
    ready_for_release_review: $ready_for_release_review,
    public_launch_ready: $public_launch_ready,
    public_launch_claimed: false,
    android_s5_real_device_claimed: false,
    live_map_ingestion_performed: false,
    live_public_exposure_performed: false,
    handoff_rule: "operator_handoff_collects_real_external_public_launch_evidence_without_claiming_public_launch_ready_or_android_s5_real_device_ready",
    markdown_path: $markdown_path,
    evidence_bundle_template: $evidence_bundle_template,
    needs_collection_count: $needs_collection_count,
    known_blockers: $known_blockers,
    blocked_items: $blocked_items,
    operator_actions: $operator_actions,
    handoff_artifacts: $handoff_artifacts,
    missing_artifacts: $missing_artifacts,
    failures: $failures,
    reviewer_next_action: (if $green and $public_launch_ready then "review_public_launch_ready_evidence" elif $green then "collect_real_external_public_launch_evidence_using_operator_handoff" else "repair_public_launch_operator_handoff_chain" end)
  }' >"$SUMMARY_FILE"

{
  printf '# Trillionnium World Public Launch Operator Handoff\n\n'
  printf -- '- status: %s\n' "$STATUS"
  printf -- '- ready_for_release_review: %s\n' "$READY_FOR_RELEASE_REVIEW"
  printf -- '- public_launch_ready: %s\n' "$PUBLIC_LAUNCH_READY"
  printf -- '- public_launch_claimed: false\n'
  printf -- '- android_s5_real_device_claimed: false\n'
  printf -- '- live_map_ingestion_performed: false\n'
  printf -- '- live_public_exposure_performed: false\n\n'
  printf '## Operator Collection Actions\n\n'
  jq -r '.operator_actions[] | "- [ ] " + .label + " (" + .accepted_status + "): " + .collection_requirement + "\n  - env: " + .evidence_env_var + "\n  - template: " + .template_path + "\n  - collect: " + .collection_command + "\n  - validate: " + .validator_command' "$SUMMARY_FILE"
  printf '\n## Bundle Flow\n\n'
  printf -- '- Copy %s to an operator-owned evidence manifest and fill all six real evidence paths.\n' "$EVIDENCE_BUNDLE_TEMPLATE"
  printf -- '- Run TRILLIONNIUM_PUBLIC_LAUNCH_EVIDENCE_BUNDLE_PATH=<real-bundle.json> scripts/check_trillionnium_world_public_launch_evidence_bundle.sh --require-ready.\n'
  printf -- '- Then rerun scripts/check_trillionnium_world_release_review_ci_gate.sh.\n\n'
  printf '## Handoff Artifacts\n\n'
  jq -r '.handoff_artifacts[] | "- " + .id + ": " + .path + " (" + .file_status + ", " + (.sha256 // "no-sha") + ")"' "$SUMMARY_FILE"
  printf '\n## Boundary\n\n'
  printf -- '- This handoff is an operator checklist and checksum manifest, not public-launch approval.\n'
  printf -- '- No live map ingestion, public exposure, Android S5 claim, or public-launch readiness claim is made here.\n'
} >"$MARKDOWN_FILE"

case "$STATUS" in
  public_launch_operator_handoff_complete_green)
    printf 'TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_OPERATOR_HANDOFF_COMPLETE %s %s\n' "$SUMMARY_FILE" "$MARKDOWN_FILE"
    ;;
  public_launch_operator_handoff_ready_with_external_blockers)
    printf 'TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_OPERATOR_HANDOFF_READY_WITH_EXTERNAL_BLOCKERS %s %s\n' "$SUMMARY_FILE" "$MARKDOWN_FILE"
    ;;
  *)
    printf 'TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_OPERATOR_HANDOFF_BLOCKED %s %s\n' "$STATUS" "$SUMMARY_FILE" >&2
    exit 1
    ;;
esac
