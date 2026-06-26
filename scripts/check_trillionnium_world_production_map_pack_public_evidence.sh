#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S4_map_pack_gate/latest"
SUMMARY_FILE="${TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_SUMMARY:-$ACCEPTANCE_DIR/production-map-pack-public-evidence.json}"
EVIDENCE_PATH="${TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH:-}"
ROUTE_EVIDENCE="$ACCEPTANCE_DIR/production-map-pack-route.json"
SCHEMA_FILE="$ACCEPTANCE_DIR/production-map-pack-public-evidence.schema.json"
TEMPLATE_FILE="$ACCEPTANCE_DIR/production-map-pack-public-evidence.template.json"
REQUIRE_READY=0

usage() {
  cat <<'EOF_USAGE'
Usage: scripts/check_trillionnium_world_production_map_pack_public_evidence.sh [--require-ready]

Validates operator-supplied production map-pack public evidence. The validator
does not perform live map ingestion and does not grant public-launch credit unless
the evidence file reaches production_map_pack_public_ready_green.

Collection checklist:
  scripts/check_trillionnium_world_production_map_pack_public_evidence_collection.sh

Strict validation:
  TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH=<real-map-pack-evidence.json> scripts/check_trillionnium_world_production_map_pack_public_evidence.sh --require-ready
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

file_status() {
  local path="$1"
  if [[ -n "$path" && -f "$path" ]]; then
    printf 'present'
  else
    printf 'missing'
  fi
}

read_json_field() {
  local path="$1"
  local expr="$2"
  if [[ -f "$path" ]]; then
    jq -r "$expr // empty" "$path" 2>/dev/null || true
  fi
}

artifact_ref_status() {
  local ref="$1"
  local evidence_dir=""
  if [[ -n "$EVIDENCE_PATH" ]]; then
    evidence_dir="$(cd "$(dirname "$EVIDENCE_PATH")" 2>/dev/null && pwd || true)"
  fi

  if [[ -z "$ref" || "$ref" == "null" ]]; then
    printf 'missing'
  elif [[ "$ref" =~ ^https?:// ]]; then
    printf 'remote_url'
  elif [[ -f "$ref" ]]; then
    printf 'local_file'
  elif [[ "$ref" != /* && -f "$ROOT/$ref" ]]; then
    printf 'workspace_file'
  elif [[ -n "$evidence_dir" && "$ref" != /* && -f "$evidence_dir/$ref" ]]; then
    printf 'evidence_relative_file'
  else
    printf 'missing'
  fi
}

jq -n '{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "Trillionnium World Production Map-Pack Public Evidence",
  "type": "object",
  "required": ["contract_version", "status", "data_source", "cache_policy", "attribution", "sensitive_poi_filter", "geofence_policy", "key_custody", "distribution_revocation", "rollback", "operator_signoff"],
  "properties": {
    "contract_version": { "const": "trillionnium_world_production_map_pack_public_evidence_v1" },
    "status": { "enum": ["production_map_pack_public_ready_green", "template_requires_real_public_map_pack_evidence", "blocked"] },
    "data_source": { "type": "object" },
    "cache_policy": { "type": "object" },
    "attribution": { "type": "object" },
    "sensitive_poi_filter": { "type": "object" },
    "geofence_policy": { "type": "object" },
    "key_custody": { "type": "object" },
    "distribution_revocation": { "type": "object" },
    "rollback": { "type": "object" },
    "operator_signoff": { "type": "object" }
  }
}' >"$SCHEMA_FILE"

jq -n '{
  contract_version: "trillionnium_world_production_map_pack_public_evidence_v1",
  status: "template_requires_real_public_map_pack_evidence",
  acceptance_status: "production_map_pack_public_ready_green",
  data_source: {
    source_name: null,
    source_url_or_archive: null,
    approved: false,
    license: null,
    license_confirmed: false,
    odbl_compliance_confirmed: false,
    live_ingestion_disabled: true,
    operator_notes: null
  },
  cache_policy: {
    offline_or_cache_pack: false,
    cache_policy_approved: false,
    clients_fetch_public_osm_directly: false,
    retention_and_refresh_policy: null
  },
  attribution: {
    visible_text_required: ["OpenStreetMap contributors", "ODbL-1.0"],
    surfaces: {
      web_public: { status: "pending_public_screenshot", screenshot_or_url: null },
      native_bevy_android: { status: "pending_real_device_screenshot", screenshot_or_url: null },
      matrix_or_readonly_share: { status: "pending_public_surface_screenshot", screenshot_or_url: null }
    }
  },
  sensitive_poi_filter: { status: "not_run", report_path: null, flagged_sensitive_poi_count: null },
  geofence_policy: { status: "not_reviewed", policy_path: null },
  key_custody: { status: "not_reviewed", active_key_id: null, next_key_id: null, rotation_runbook: null },
  distribution_revocation: { status: "not_tested", public_package_url_or_path: null, revocation_probe: null },
  rollback: { status: "not_tested", rollback_evidence: null },
  operator_signoff: {
    signed_by: null,
    signed_at: null,
    real_public_map_evidence_confirmed: false,
    synthetic_or_template_data_rejected: true
  },
  collection: {
    command: "scripts/check_trillionnium_world_production_map_pack_public_evidence_collection.sh",
    validation_command: "TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH=<real-map-pack-evidence.json> scripts/check_trillionnium_world_production_map_pack_public_evidence.sh --require-ready",
    output_path: "acceptance/S4_map_pack_gate/latest/production-map-pack-public-evidence.json",
    requires_live_ingestion: false,
    requires_public_attribution_screenshots: true,
    requires_native_bevy_android_attribution_screenshot: true,
    requires_operator_signoff: true
  }
}' >"$TEMPLATE_FILE"

ROUTE_STATUS="$(read_json_field "$ROUTE_EVIDENCE" '.status')"
ROUTE_FILE_STATUS="$(file_status "$ROUTE_EVIDENCE")"
EVIDENCE_FILE_STATUS="$(file_status "$EVIDENCE_PATH")"

CONTRACT="$(read_json_field "$EVIDENCE_PATH" '.contract_version')"
EVIDENCE_STATUS="$(read_json_field "$EVIDENCE_PATH" '.status')"
DATA_SOURCE_APPROVED="$(read_json_field "$EVIDENCE_PATH" '.data_source.approved == true')"
DATA_SOURCE_REF="$(read_json_field "$EVIDENCE_PATH" '.data_source.source_url_or_archive')"
DATA_SOURCE_REF_STATUS="$(artifact_ref_status "$DATA_SOURCE_REF")"
LICENSE_CONFIRMED="$(read_json_field "$EVIDENCE_PATH" '.data_source.license_confirmed == true')"
ODBL_CONFIRMED="$(read_json_field "$EVIDENCE_PATH" '.data_source.odbl_compliance_confirmed == true')"
LIVE_INGESTION_DISABLED="$(read_json_field "$EVIDENCE_PATH" '.data_source.live_ingestion_disabled == true')"
CACHE_PACK_READY="$(read_json_field "$EVIDENCE_PATH" '.cache_policy.offline_or_cache_pack == true')"
CACHE_POLICY_APPROVED="$(read_json_field "$EVIDENCE_PATH" '.cache_policy.cache_policy_approved == true')"
CLIENTS_FETCH_DIRECT="$(read_json_field "$EVIDENCE_PATH" '.cache_policy.clients_fetch_public_osm_directly == true')"
CACHE_RETENTION_POLICY="$(read_json_field "$EVIDENCE_PATH" '.cache_policy.retention_and_refresh_policy')"
WEB_ATTRIBUTION_STATUS="$(read_json_field "$EVIDENCE_PATH" '.attribution.surfaces.web_public.status')"
NATIVE_ATTRIBUTION_STATUS="$(read_json_field "$EVIDENCE_PATH" '.attribution.surfaces.native_bevy_android.status')"
MATRIX_ATTRIBUTION_STATUS="$(read_json_field "$EVIDENCE_PATH" '.attribution.surfaces.matrix_or_readonly_share.status')"
WEB_ATTRIBUTION_PROOF="$(read_json_field "$EVIDENCE_PATH" '.attribution.surfaces.web_public.screenshot_or_url')"
NATIVE_ATTRIBUTION_PROOF="$(read_json_field "$EVIDENCE_PATH" '.attribution.surfaces.native_bevy_android.screenshot_or_url')"
MATRIX_ATTRIBUTION_PROOF="$(read_json_field "$EVIDENCE_PATH" '.attribution.surfaces.matrix_or_readonly_share.screenshot_or_url')"
WEB_ATTRIBUTION_PROOF_STATUS="$(artifact_ref_status "$WEB_ATTRIBUTION_PROOF")"
NATIVE_ATTRIBUTION_PROOF_STATUS="$(artifact_ref_status "$NATIVE_ATTRIBUTION_PROOF")"
MATRIX_ATTRIBUTION_PROOF_STATUS="$(artifact_ref_status "$MATRIX_ATTRIBUTION_PROOF")"
SENSITIVE_STATUS="$(read_json_field "$EVIDENCE_PATH" '.sensitive_poi_filter.status')"
SENSITIVE_REPORT_PATH="$(read_json_field "$EVIDENCE_PATH" '.sensitive_poi_filter.report_path')"
SENSITIVE_REPORT_STATUS="$(artifact_ref_status "$SENSITIVE_REPORT_PATH")"
GEOFENCE_STATUS="$(read_json_field "$EVIDENCE_PATH" '.geofence_policy.status')"
GEOFENCE_POLICY_PATH="$(read_json_field "$EVIDENCE_PATH" '.geofence_policy.policy_path')"
GEOFENCE_POLICY_REF_STATUS="$(artifact_ref_status "$GEOFENCE_POLICY_PATH")"
KEY_CUSTODY_STATUS="$(read_json_field "$EVIDENCE_PATH" '.key_custody.status')"
KEY_ROTATION_RUNBOOK="$(read_json_field "$EVIDENCE_PATH" '.key_custody.rotation_runbook')"
KEY_ROTATION_RUNBOOK_STATUS="$(artifact_ref_status "$KEY_ROTATION_RUNBOOK")"
DISTRIBUTION_STATUS="$(read_json_field "$EVIDENCE_PATH" '.distribution_revocation.status')"
DISTRIBUTION_PACKAGE_REF="$(read_json_field "$EVIDENCE_PATH" '.distribution_revocation.public_package_url_or_path')"
REVOCATION_PROBE_REF="$(read_json_field "$EVIDENCE_PATH" '.distribution_revocation.revocation_probe')"
DISTRIBUTION_PACKAGE_REF_STATUS="$(artifact_ref_status "$DISTRIBUTION_PACKAGE_REF")"
REVOCATION_PROBE_REF_STATUS="$(artifact_ref_status "$REVOCATION_PROBE_REF")"
ROLLBACK_STATUS="$(read_json_field "$EVIDENCE_PATH" '.rollback.status')"
ROLLBACK_EVIDENCE_REF="$(read_json_field "$EVIDENCE_PATH" '.rollback.rollback_evidence')"
ROLLBACK_EVIDENCE_REF_STATUS="$(artifact_ref_status "$ROLLBACK_EVIDENCE_REF")"
SIGNOFF_REAL="$(read_json_field "$EVIDENCE_PATH" '.operator_signoff.real_public_map_evidence_confirmed == true')"
SIGNOFF_REJECTS_TEMPLATE="$(read_json_field "$EVIDENCE_PATH" '.operator_signoff.synthetic_or_template_data_rejected == true')"
SIGNOFF_BY="$(read_json_field "$EVIDENCE_PATH" '.operator_signoff.signed_by')"
SIGNOFF_AT="$(read_json_field "$EVIDENCE_PATH" '.operator_signoff.signed_at')"

BLOCKERS=()
[[ "$ROUTE_STATUS" == "production_map_pack_route_green" ]] || BLOCKERS+=("production_map_pack_route_green")
[[ "$EVIDENCE_FILE_STATUS" == "present" ]] || BLOCKERS+=("production_map_pack_public_evidence_file")
[[ "$CONTRACT" == "trillionnium_world_production_map_pack_public_evidence_v1" ]] || BLOCKERS+=("production_map_pack_public_contract")
[[ "$EVIDENCE_STATUS" == "production_map_pack_public_ready_green" ]] || BLOCKERS+=("production_map_pack_public_status")
[[ "$DATA_SOURCE_APPROVED" == "true" ]] || BLOCKERS+=("approved_production_map_source")
[[ "$DATA_SOURCE_REF_STATUS" != "missing" ]] || BLOCKERS+=("production_map_source_artifact")
[[ "$LICENSE_CONFIRMED" == "true" && "$ODBL_CONFIRMED" == "true" ]] || BLOCKERS+=("license_and_odbl_compliance")
[[ "$LIVE_INGESTION_DISABLED" == "true" ]] || BLOCKERS+=("live_ingestion_must_remain_disabled")
[[ "$CACHE_PACK_READY" == "true" && "$CACHE_POLICY_APPROVED" == "true" && "$CLIENTS_FETCH_DIRECT" != "true" ]] || BLOCKERS+=("offline_cache_policy")
[[ -n "$CACHE_RETENTION_POLICY" && "$CACHE_RETENTION_POLICY" != "null" ]] || BLOCKERS+=("cache_retention_refresh_policy")
[[ "$WEB_ATTRIBUTION_STATUS" == "visible_attribution_screenshot_green" && "$WEB_ATTRIBUTION_PROOF_STATUS" != "missing" ]] || BLOCKERS+=("web_public_attribution_screenshot")
[[ "$NATIVE_ATTRIBUTION_STATUS" == "visible_attribution_screenshot_green" && "$NATIVE_ATTRIBUTION_PROOF_STATUS" != "missing" ]] || BLOCKERS+=("native_bevy_android_attribution_screenshot")
[[ "$MATRIX_ATTRIBUTION_STATUS" == "visible_attribution_screenshot_green" && "$MATRIX_ATTRIBUTION_PROOF_STATUS" != "missing" ]] || BLOCKERS+=("matrix_or_readonly_attribution_screenshot")
[[ "$SENSITIVE_STATUS" == "sensitive_poi_filter_green" ]] || BLOCKERS+=("sensitive_poi_filter")
[[ "$SENSITIVE_REPORT_STATUS" != "missing" ]] || BLOCKERS+=("sensitive_poi_report_artifact")
[[ "$GEOFENCE_STATUS" == "geofence_policy_green" ]] || BLOCKERS+=("geofence_policy")
[[ "$GEOFENCE_POLICY_REF_STATUS" != "missing" ]] || BLOCKERS+=("geofence_policy_artifact")
[[ "$KEY_CUSTODY_STATUS" == "key_custody_rotation_green" ]] || BLOCKERS+=("key_custody_rotation")
[[ "$KEY_ROTATION_RUNBOOK_STATUS" != "missing" ]] || BLOCKERS+=("key_rotation_runbook_artifact")
[[ "$DISTRIBUTION_STATUS" == "public_distribution_revocation_green" ]] || BLOCKERS+=("public_distribution_revocation")
[[ "$DISTRIBUTION_PACKAGE_REF_STATUS" != "missing" ]] || BLOCKERS+=("public_distribution_package_artifact")
[[ "$REVOCATION_PROBE_REF_STATUS" != "missing" ]] || BLOCKERS+=("revocation_probe_artifact")
[[ "$ROLLBACK_STATUS" == "public_map_pack_rollback_green" ]] || BLOCKERS+=("public_map_pack_rollback")
[[ "$ROLLBACK_EVIDENCE_REF_STATUS" != "missing" ]] || BLOCKERS+=("public_map_pack_rollback_artifact")
[[ "$SIGNOFF_REAL" == "true" && "$SIGNOFF_REJECTS_TEMPLATE" == "true" && -n "$SIGNOFF_BY" && -n "$SIGNOFF_AT" ]] || BLOCKERS+=("operator_signoff")

STATUS="production_map_pack_public_ready_green"
if [[ "${#BLOCKERS[@]}" -gt 0 ]]; then
  STATUS="blocked_missing_production_map_pack_public_evidence"
fi

BLOCKERS_JSON="$(printf '%s\n' "${BLOCKERS[@]}" | jq -Rsc 'split("\n") | map(select(length > 0))')"
BLOCKER_COUNT="${#BLOCKERS[@]}"
PRODUCTION_MAP_PACK_GREEN=false
if [[ "$STATUS" == "production_map_pack_public_ready_green" ]]; then
  PRODUCTION_MAP_PACK_GREEN=true
fi

jq -n \
  --arg contract_version "trillionnium_world_production_map_pack_public_evidence_gate_v1" \
  --arg status "$STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --argjson production_map_pack_green "$PRODUCTION_MAP_PACK_GREEN" \
  --argjson blocker_count "$BLOCKER_COUNT" \
  --argjson required_check_count 24 \
  --argjson schema_artifact_count 2 \
  --arg evidence_path "$EVIDENCE_PATH" \
  --arg evidence_file_status "$EVIDENCE_FILE_STATUS" \
  --arg evidence_contract "$CONTRACT" \
  --arg evidence_status "$EVIDENCE_STATUS" \
  --arg route_evidence "$ROUTE_EVIDENCE" \
  --arg route_status "$ROUTE_STATUS" \
  --arg route_file_status "$ROUTE_FILE_STATUS" \
  --arg schema_path "$SCHEMA_FILE" \
  --arg template_path "$TEMPLATE_FILE" \
  --arg schema_sha256 "$(sha256sum "$SCHEMA_FILE" | awk '{print $1}')" \
  --arg template_sha256 "$(sha256sum "$TEMPLATE_FILE" | awk '{print $1}')" \
  --arg web_attribution_status "$WEB_ATTRIBUTION_STATUS" \
  --arg web_attribution_proof_status "$WEB_ATTRIBUTION_PROOF_STATUS" \
  --arg native_attribution_status "$NATIVE_ATTRIBUTION_STATUS" \
  --arg native_attribution_proof_status "$NATIVE_ATTRIBUTION_PROOF_STATUS" \
  --arg matrix_attribution_status "$MATRIX_ATTRIBUTION_STATUS" \
  --arg matrix_attribution_proof_status "$MATRIX_ATTRIBUTION_PROOF_STATUS" \
  --arg data_source_ref_status "$DATA_SOURCE_REF_STATUS" \
  --arg cache_retention_policy "$CACHE_RETENTION_POLICY" \
  --arg sensitive_status "$SENSITIVE_STATUS" \
  --arg sensitive_report_status "$SENSITIVE_REPORT_STATUS" \
  --arg geofence_status "$GEOFENCE_STATUS" \
  --arg geofence_policy_ref_status "$GEOFENCE_POLICY_REF_STATUS" \
  --arg key_custody_status "$KEY_CUSTODY_STATUS" \
  --arg key_rotation_runbook_status "$KEY_ROTATION_RUNBOOK_STATUS" \
  --arg distribution_status "$DISTRIBUTION_STATUS" \
  --arg distribution_package_ref_status "$DISTRIBUTION_PACKAGE_REF_STATUS" \
  --arg revocation_probe_ref_status "$REVOCATION_PROBE_REF_STATUS" \
  --arg rollback_status "$ROLLBACK_STATUS" \
  --arg rollback_evidence_ref_status "$ROLLBACK_EVIDENCE_REF_STATUS" \
  --argjson blockers "$BLOCKERS_JSON" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_production_map_pack_public_evidence_gate",
    green: $production_map_pack_green,
    public_map_pack_ready: ($status == "production_map_pack_public_ready_green"),
    accepted_status: "production_map_pack_public_ready_green",
    live_ingestion_performed: false,
    live_ingestion_allowed: false,
    runtime_clients_fetch_public_osm_directly: false,
    public_launch_credit: "only_when_status_is_production_map_pack_public_ready_green",
    blocker_count: $blocker_count,
    required_check_count: $required_check_count,
    schema_artifact_count: $schema_artifact_count,
    blockers: $blockers,
    operator_evidence: {
      path: $evidence_path,
      file_status: $evidence_file_status,
      contract_version: $evidence_contract,
      status: $evidence_status
    },
    route_prerequisite: {
      evidence_path: $route_evidence,
      file_status: $route_file_status,
      status: $route_status,
      accepted_status: "production_map_pack_route_green"
    },
    schema: {
      path: $schema_path,
      sha256: $schema_sha256,
      template_path: $template_path,
      template_sha256: $template_sha256
    },
    required_checks: {
      approved_production_map_source: true,
      production_map_source_artifact_status: $data_source_ref_status,
      license_and_odbl_compliance: true,
      live_ingestion_disabled: true,
      offline_cache_policy: true,
      cache_retention_refresh_policy: (if $cache_retention_policy == "" then null else $cache_retention_policy end),
      direct_public_osm_client_fetch_forbidden: true,
      web_public_attribution_status: $web_attribution_status,
      web_public_attribution_artifact_status: $web_attribution_proof_status,
      native_bevy_android_attribution_status: $native_attribution_status,
      native_bevy_android_attribution_artifact_status: $native_attribution_proof_status,
      matrix_or_readonly_attribution_status: $matrix_attribution_status,
      matrix_or_readonly_attribution_artifact_status: $matrix_attribution_proof_status,
      sensitive_poi_filter_status: $sensitive_status,
      sensitive_poi_report_artifact_status: $sensitive_report_status,
      geofence_policy_status: $geofence_status,
      geofence_policy_artifact_status: $geofence_policy_ref_status,
      key_custody_status: $key_custody_status,
      key_rotation_runbook_artifact_status: $key_rotation_runbook_status,
      distribution_revocation_status: $distribution_status,
      public_distribution_package_artifact_status: $distribution_package_ref_status,
      revocation_probe_artifact_status: $revocation_probe_ref_status,
      rollback_status: $rollback_status,
      rollback_evidence_artifact_status: $rollback_evidence_ref_status
    }
  }' >"$SUMMARY_FILE"

if [[ "$STATUS" == "production_map_pack_public_ready_green" ]]; then
  printf 'TRILLIONNIUM_WORLD_PRODUCTION_MAP_PACK_PUBLIC_READY %s\n' "$SUMMARY_FILE"
  exit 0
fi

printf 'TRILLIONNIUM_WORLD_PRODUCTION_MAP_PACK_PUBLIC_BLOCKED %s %s\n' "$STATUS" "$SUMMARY_FILE"
if [[ "$REQUIRE_READY" -eq 1 ]]; then
  exit 1
fi
