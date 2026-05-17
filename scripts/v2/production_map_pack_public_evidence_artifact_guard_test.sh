#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
VALIDATOR="$ROOT/scripts/check_trillionnium_world_production_map_pack_public_evidence.sh"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

bash "$ROOT/scripts/check_trillionnium_world_production_map_pack_route.sh" >/dev/null

EVIDENCE="$TMP_DIR/green-status-missing-artifacts.json"
SUMMARY="$TMP_DIR/green-status-missing-artifacts.summary.json"
MISSING="$TMP_DIR/missing-artifacts"

jq -n --arg missing "$MISSING" '{
  contract_version: "trillionnium_world_production_map_pack_public_evidence_v1",
  status: "production_map_pack_public_ready_green",
  data_source: {
    source_name: "fixture claims a production map source without files",
    source_url_or_archive: ($missing + "/source.osm.pbf"),
    approved: true,
    license: "ODbL-1.0",
    license_confirmed: true,
    odbl_compliance_confirmed: true,
    live_ingestion_disabled: true,
    operator_notes: "negative fixture: statuses are green but artifacts are missing"
  },
  cache_policy: {
    offline_or_cache_pack: true,
    cache_policy_approved: true,
    clients_fetch_public_osm_directly: false,
    retention_and_refresh_policy: "fixture retention policy text"
  },
  attribution: {
    surfaces: {
      web_public: { status: "visible_attribution_screenshot_green", screenshot_or_url: ($missing + "/web-attribution.png") },
      native_bevy_android: { status: "visible_attribution_screenshot_green", screenshot_or_url: ($missing + "/native-attribution.png") },
      matrix_or_readonly_share: { status: "visible_attribution_screenshot_green", screenshot_or_url: ($missing + "/matrix-attribution.png") }
    }
  },
  sensitive_poi_filter: { status: "sensitive_poi_filter_green", report_path: ($missing + "/sensitive-poi.json"), flagged_sensitive_poi_count: 0 },
  geofence_policy: { status: "geofence_policy_green", policy_path: ($missing + "/geofence.md") },
  key_custody: { status: "key_custody_rotation_green", active_key_id: "production-primary-ed25519", next_key_id: "production-next-ed25519", rotation_runbook: ($missing + "/rotation.md") },
  distribution_revocation: { status: "public_distribution_revocation_green", public_package_url_or_path: ($missing + "/map-pack.tar.zst"), revocation_probe: ($missing + "/revocation-probe.json") },
  rollback: { status: "public_map_pack_rollback_green", rollback_evidence: ($missing + "/rollback.json") },
  operator_signoff: {
    signed_by: "negative-fixture",
    signed_at: "2026-05-17T00:00:00Z",
    real_public_map_evidence_confirmed: true,
    synthetic_or_template_data_rejected: true
  }
}' >"$EVIDENCE"

set +e
TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH="$EVIDENCE" \
TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_SUMMARY="$SUMMARY" \
  "$VALIDATOR" --require-ready >"$TMP_DIR/stdout.log" 2>"$TMP_DIR/stderr.log"
EXIT_CODE=$?
set -e

if [[ "$EXIT_CODE" == "0" ]]; then
  echo "[FAIL] production map-pack public evidence accepted missing artifact references" >&2
  exit 1
fi

STATUS="$(jq -r '.status // empty' "$SUMMARY")"
if [[ "$STATUS" != "blocked_missing_production_map_pack_public_evidence" ]]; then
  echo "[FAIL] unexpected validator status: $STATUS" >&2
  exit 1
fi

required_blockers=(
  production_map_source_artifact
  web_public_attribution_screenshot
  native_bevy_android_attribution_screenshot
  matrix_or_readonly_attribution_screenshot
  sensitive_poi_report_artifact
  geofence_policy_artifact
  key_rotation_runbook_artifact
  public_distribution_package_artifact
  revocation_probe_artifact
  public_map_pack_rollback_artifact
)

for blocker in "${required_blockers[@]}"; do
  if ! jq -e --arg blocker "$blocker" '(.blockers // []) | index($blocker)' "$SUMMARY" >/dev/null; then
    echo "[FAIL] missing expected artifact blocker: $blocker" >&2
    exit 1
  fi
done

if ! jq -e '
  .required_checks.production_map_source_artifact_status == "missing" and
  .required_checks.web_public_attribution_artifact_status == "missing" and
  .required_checks.rollback_evidence_artifact_status == "missing"
' "$SUMMARY" >/dev/null; then
  echo "[FAIL] summary did not expose missing artifact statuses" >&2
  exit 1
fi

echo "[PASS] production map-pack public evidence rejects green-status evidence with missing source, attribution, POI, geofence, key, distribution, revocation, and rollback artifacts"
