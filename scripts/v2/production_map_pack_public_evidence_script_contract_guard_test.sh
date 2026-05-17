#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_production_map_pack_public_evidence.sh"

required_lines=(
  'trillionnium_world_production_map_pack_public_evidence_gate_v1'
  'trillionnium_world_production_map_pack_public_evidence_v1'
  'production_map_pack_public_ready_green'
  'TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH'
  'TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_SUMMARY'
  'check_trillionnium_world_production_map_pack_public_evidence_collection.sh'
  'collection:'
  'live_ingestion_performed: false'
  'live_ingestion_allowed: false'
  'runtime_clients_fetch_public_osm_directly: false'
  'template_requires_real_public_map_pack_evidence'
  'visible_attribution_screenshot_green'
  'artifact_ref_status'
  'production_map_source_artifact'
  'cache_retention_refresh_policy'
  'sensitive_poi_report_artifact'
  'sensitive_poi_filter_green'
  'geofence_policy_green'
  'geofence_policy_artifact'
  'key_custody_rotation_green'
  'key_rotation_runbook_artifact'
  'public_distribution_revocation_green'
  'public_distribution_package_artifact'
  'revocation_probe_artifact'
  'public_map_pack_rollback_green'
  'public_map_pack_rollback_artifact'
  'synthetic_or_template_data_rejected'
  '--require-ready'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] production map-pack public evidence script missing contract line: $line" >&2
    exit 1
  fi
done

echo "[PASS] production map-pack public evidence script keeps controlled evidence contract, no-live-ingestion boundary, attribution/POI/rollback checks, and strict mode"
