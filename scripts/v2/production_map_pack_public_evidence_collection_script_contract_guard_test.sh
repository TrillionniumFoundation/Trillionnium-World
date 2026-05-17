#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_production_map_pack_public_evidence_collection.sh"

required_lines=(
  'trillionnium_world_production_map_pack_public_evidence_collection_v1'
  'production-map-pack-public-evidence-collection.json'
  'production-map-pack-public-evidence-collection.md'
  'check_trillionnium_world_production_map_pack_route.sh'
  'check_trillionnium_world_production_map_pack_public_evidence.sh'
  'production_map_pack_route_green'
  'production_map_pack_public_ready_green'
  'TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH=<real-map-pack-evidence.json>'
  'approved_production_map_source'
  'offline_cache_policy'
  'web_public_attribution_screenshot'
  'native_bevy_android_attribution_screenshot'
  'matrix_or_readonly_attribution_screenshot'
  'sensitive_poi_filter'
  'geofence_policy'
  'key_custody_rotation'
  'public_distribution_revocation'
  'public_map_pack_rollback'
  'operator_signoff'
  'live_ingestion_performed: false'
  'live_ingestion_allowed: false'
  'runtime_clients_fetch_public_osm_directly: false'
  'public_launch_credit: false'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] production map-pack public evidence collection script missing contract line: $line" >&2
    exit 1
  fi
done

echo "[PASS] production map-pack public evidence collection script keeps route prerequisite, artifact checklist, validation command, and no-live-ingestion boundary"
