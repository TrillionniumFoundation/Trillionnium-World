#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S4_map_pack_gate/latest"
SUMMARY_FILE="$ACCEPTANCE_DIR/production-map-pack-public-evidence-collection.json"
MARKDOWN_FILE="$ACCEPTANCE_DIR/production-map-pack-public-evidence-collection.md"
ROUTE_LOG="$ACCEPTANCE_DIR/production-map-pack-public-evidence-collection-route.log"
VALIDATOR_LOG="$ACCEPTANCE_DIR/production-map-pack-public-evidence-collection-validator.log"
ROUTE_SUMMARY="$ACCEPTANCE_DIR/production-map-pack-route.json"
VALIDATOR_SUMMARY="$ACCEPTANCE_DIR/production-map-pack-public-evidence.json"
TEMPLATE_FILE="$ACCEPTANCE_DIR/production-map-pack-public-evidence.template.json"
SCHEMA_FILE="$ACCEPTANCE_DIR/production-map-pack-public-evidence.schema.json"

mkdir -p "$ACCEPTANCE_DIR"

require_cmd() {
  local name="$1"
  if ! command -v "$name" >/dev/null 2>&1; then
    printf 'TRILLIONNIUM_WORLD_PRODUCTION_MAP_PACK_PUBLIC_COLLECTION_FAILED missing command: %s\n' "$name" >&2
    exit 1
  fi
}

read_json_field() {
  local path="$1"
  local expr="$2"
  if [[ -f "$path" ]]; then
    jq -r "$expr // empty" "$path" 2>/dev/null || true
  fi
}

file_sha256() {
  local path="$1"
  if [[ -f "$path" ]]; then
    sha256sum "$path" | awk '{print $1}'
  fi
}

require_cmd jq
require_cmd sha256sum

bash "$ROOT/scripts/check_trillionnium_world_production_map_pack_route.sh" >"$ROUTE_LOG" 2>&1
bash "$ROOT/scripts/check_trillionnium_world_production_map_pack_public_evidence.sh" >"$VALIDATOR_LOG" 2>&1 || true

ROUTE_STATUS="$(read_json_field "$ROUTE_SUMMARY" '.status')"
VALIDATOR_STATUS="$(read_json_field "$VALIDATOR_SUMMARY" '.status')"
STATUS="production_map_pack_public_evidence_collection_ready"
if [[ "$ROUTE_STATUS" != "production_map_pack_route_green" || ! -f "$TEMPLATE_FILE" || ! -f "$SCHEMA_FILE" ]]; then
  STATUS="production_map_pack_public_evidence_collection_blocked"
fi
COLLECTION_GREEN=false
if [[ "$STATUS" == "production_map_pack_public_evidence_collection_ready" ]]; then
  COLLECTION_GREEN=true
fi
BLOCKED_VALIDATOR_STATUS_COUNT=0
if [[ "$VALIDATOR_STATUS" != "production_map_pack_public_ready_green" ]]; then
  BLOCKED_VALIDATOR_STATUS_COUNT=1
fi

jq -n \
  --arg contract_version "trillionnium_world_production_map_pack_public_evidence_collection_v1" \
  --arg status "$STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --argjson collection_green "$COLLECTION_GREEN" \
  --argjson blocked_validator_status_count "$BLOCKED_VALIDATOR_STATUS_COUNT" \
  --arg route_summary "$ROUTE_SUMMARY" \
  --arg route_status "$ROUTE_STATUS" \
  --arg route_log "$ROUTE_LOG" \
  --arg validator_summary "$VALIDATOR_SUMMARY" \
  --arg validator_status "$VALIDATOR_STATUS" \
  --arg validator_log "$VALIDATOR_LOG" \
  --arg template_path "$TEMPLATE_FILE" \
  --arg schema_path "$SCHEMA_FILE" \
  --arg template_sha256 "$(file_sha256 "$TEMPLATE_FILE")" \
  --arg schema_sha256 "$(file_sha256 "$SCHEMA_FILE")" \
  --arg markdown_path "$MARKDOWN_FILE" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_production_map_pack_public_evidence_collection",
    green: $collection_green,
    public_map_pack_ready: false,
    public_launch_credit: false,
    live_ingestion_performed: false,
    live_ingestion_allowed: false,
    runtime_clients_fetch_public_osm_directly: false,
    route_prerequisite_count: 1,
    template_count: 1,
    template_schema_count: 1,
    required_evidence_count: 11,
    validator_status_count: 1,
    blocked_validator_status_count: $blocked_validator_status_count,
    collection_command: "scripts/check_trillionnium_world_production_map_pack_public_evidence_collection.sh",
    validation_command: "TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH=<real-map-pack-evidence.json> scripts/check_trillionnium_world_production_map_pack_public_evidence.sh --require-ready",
    route_prerequisite: {
      summary: $route_summary,
      status: $route_status,
      log: $route_log,
      accepted_status: "production_map_pack_route_green"
    },
    validator: {
      summary: $validator_summary,
      status: $validator_status,
      log: $validator_log,
      accepted_status: "production_map_pack_public_ready_green"
    },
    template: {
      path: $template_path,
      sha256: $template_sha256,
      schema_path: $schema_path,
      schema_sha256: $schema_sha256,
      public_launch_credit: false
    },
    required_evidence: [
      { id: "approved_production_map_source", field: "data_source.source_url_or_archive", evidence: "approved public source archive or URL, source name, ODbL/license approval, and operator notes" },
      { id: "offline_cache_policy", field: "cache_policy", evidence: "offline/cache-pack policy, retention and refresh policy, and proof clients do not fetch public OSM directly" },
      { id: "web_public_attribution_screenshot", field: "attribution.surfaces.web_public", evidence: "public web screenshot or URL showing OpenStreetMap contributors and ODbL-1.0 attribution" },
      { id: "native_bevy_android_attribution_screenshot", field: "attribution.surfaces.native_bevy_android", evidence: "real Android Native/Bevy screenshot showing OpenStreetMap contributors and ODbL-1.0 attribution" },
      { id: "matrix_or_readonly_attribution_screenshot", field: "attribution.surfaces.matrix_or_readonly_share", evidence: "readonly/share surface screenshot or URL showing attribution" },
      { id: "sensitive_poi_filter", field: "sensitive_poi_filter.report_path", evidence: "sensitive POI filter report with flagged count and reviewer disposition" },
      { id: "geofence_policy", field: "geofence_policy.policy_path", evidence: "geofence/downstream takedown policy artifact" },
      { id: "key_custody_rotation", field: "key_custody.rotation_runbook", evidence: "active and next key ids plus rotation/custody runbook" },
      { id: "public_distribution_revocation", field: "distribution_revocation", evidence: "public package URL/path and revocation probe artifact" },
      { id: "public_map_pack_rollback", field: "rollback.rollback_evidence", evidence: "rollback drill artifact for the public map pack" },
      { id: "operator_signoff", field: "operator_signoff", evidence: "signed_by, signed_at, real_public_map_evidence_confirmed=true, synthetic_or_template_data_rejected=true" }
    ],
    boundary: [
      "This script creates a collection checklist only.",
      "It does not perform live Overpass or Geofabrik ingestion.",
      "It does not claim production_map_pack_public_ready_green.",
      "Only the validator can grant public launch credit after real external artifacts are attached."
    ],
    reviewer_next_action: "fill_template_with_real_public_map_pack_evidence_then_run_validator"
  }' >"$SUMMARY_FILE"

{
  printf '# Trillionnium World Production Map-Pack Public Evidence Collection\n\n'
  printf -- '- status: %s\n' "$STATUS"
  printf -- '- green: %s\n' "$COLLECTION_GREEN"
  printf -- '- public_map_pack_ready: false\n'
  printf -- '- live_ingestion_performed: false\n'
  printf -- '- required_evidence_count: 11\n'
  printf -- '- template_count: 1\n'
  printf -- '- template_schema_count: 1\n'
  printf -- '- blocked_validator_status_count: %s\n' "$BLOCKED_VALIDATOR_STATUS_COUNT"
  printf -- '- route_status: %s\n' "$ROUTE_STATUS"
  printf -- '- validator_status: %s\n\n' "$VALIDATOR_STATUS"
  printf '## Commands\n\n'
  printf -- '- collect: scripts/check_trillionnium_world_production_map_pack_public_evidence_collection.sh\n'
  printf -- '- validate: TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH=<real-map-pack-evidence.json> scripts/check_trillionnium_world_production_map_pack_public_evidence.sh --require-ready\n\n'
  printf '## Required Evidence\n\n'
  jq -r '.required_evidence[] | "- [ ] " + .id + ": " + .evidence + "\n  - field: " + .field' "$SUMMARY_FILE"
  printf '\n## Boundary\n\n'
  printf -- '- No live map ingestion is performed here.\n'
  printf -- '- This collection artifact has no public-launch credit.\n'
  printf -- '- Production map-pack public readiness requires the validator accepted status.\n'
} >"$MARKDOWN_FILE"

if [[ "$STATUS" == "production_map_pack_public_evidence_collection_ready" ]]; then
  printf 'TRILLIONNIUM_WORLD_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_COLLECTION_READY %s %s\n' "$SUMMARY_FILE" "$MARKDOWN_FILE"
  exit 0
fi

printf 'TRILLIONNIUM_WORLD_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_COLLECTION_BLOCKED %s %s\n' "$STATUS" "$SUMMARY_FILE" >&2
exit 1
