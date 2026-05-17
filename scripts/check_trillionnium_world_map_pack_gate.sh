#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S4_map_pack_gate/latest"
KEY_DIR="$ACCEPTANCE_DIR/keys"
UNSIGNED_MANIFEST="$ACCEPTANCE_DIR/map_pack_manifest_unsigned.json"
SIGNED_MANIFEST="$ACCEPTANCE_DIR/map_pack_manifest_signed.json"
ATTRIBUTION_EVIDENCE="$ACCEPTANCE_DIR/attribution_evidence.json"
SENSITIVE_POI_REPORT="$ACCEPTANCE_DIR/sensitive_poi_filter_report.json"
SIGNATURE_FILE="$ACCEPTANCE_DIR/map_pack_manifest_unsigned.ed25519.sig"
VERIFY_LOG="$ACCEPTANCE_DIR/map_pack_signature_verify.txt"
SUMMARY_FILE="$ACCEPTANCE_DIR/map-pack-gate-summary.json"
PRIVATE_KEY="$KEY_DIR/dev-fixture-map-pack-ed25519.key"
PUBLIC_KEY="$KEY_DIR/dev-fixture-map-pack-ed25519.pub"
KEY_ID="dev-fixture-ed25519-20260515"

mkdir -p "$ACCEPTANCE_DIR" "$KEY_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-server -- map-pack-manifest >"$UNSIGNED_MANIFEST"
  cargo run -p trnm-world-server -- map-pack-attribution-evidence >"$ATTRIBUTION_EVIDENCE"
  cargo run -p trnm-world-server -- map-pack-sensitive-poi-report >"$SENSITIVE_POI_REPORT"
)

if [[ ! -f "$PRIVATE_KEY" ]]; then
  openssl genpkey -algorithm ED25519 -out "$PRIVATE_KEY" >/dev/null 2>&1
  chmod 600 "$PRIVATE_KEY"
fi
openssl pkey -in "$PRIVATE_KEY" -pubout -out "$PUBLIC_KEY" >/dev/null 2>&1

openssl pkeyutl -sign -rawin -inkey "$PRIVATE_KEY" -in "$UNSIGNED_MANIFEST" -out "$SIGNATURE_FILE"
openssl pkeyutl -verify -rawin -pubin -inkey "$PUBLIC_KEY" -in "$UNSIGNED_MANIFEST" -sigfile "$SIGNATURE_FILE" >"$VERIFY_LOG"

SIGNATURE_BASE64="$(base64 -w0 "$SIGNATURE_FILE")"
SIGNED_PAYLOAD_SHA256="$(sha256sum "$UNSIGNED_MANIFEST" | awk '{print $1}')"
PUBLIC_KEY_SHA256="$(sha256sum "$PUBLIC_KEY" | awk '{print $1}')"

jq \
  --arg status "fixture_signed_map_pack_manifest_green" \
  --arg key_id "$KEY_ID" \
  --arg public_key_path "$PUBLIC_KEY" \
  --arg public_key_sha256 "$PUBLIC_KEY_SHA256" \
  --arg signed_payload_sha256 "$SIGNED_PAYLOAD_SHA256" \
  --arg signature_base64 "$SIGNATURE_BASE64" \
  '.status = $status
   | .signature = {
      required: true,
      status: "verified_dev_fixture_signature",
      algorithm: "Ed25519",
      key_id: $key_id,
      public_key_path: $public_key_path,
      public_key_sha256: $public_key_sha256,
      signed_payload_sha256: $signed_payload_sha256,
      signature_base64: $signature_base64
    }' "$UNSIGNED_MANIFEST" >"$SIGNED_MANIFEST"

ATTRIBUTION_STATUS="$(jq -r '.status' "$ATTRIBUTION_EVIDENCE")"
SENSITIVE_STATUS="$(jq -r '.status' "$SENSITIVE_POI_REPORT")"
FLAGGED_COUNT="$(jq -r '.flagged_node_count' "$SENSITIVE_POI_REPORT")"
MANIFEST_STATUS="$(jq -r '.status' "$SIGNED_MANIFEST")"

OVERALL_STATUS="fixture_signed_map_pack_gate_green"
if [[ "$ATTRIBUTION_STATUS" != "fixture_attribution_evidence_green" || "$SENSITIVE_STATUS" != "fixture_sensitive_poi_filter_green" || "$MANIFEST_STATUS" != "fixture_signed_map_pack_manifest_green" ]]; then
  OVERALL_STATUS="fixture_map_pack_gate_failed"
fi

jq -n \
  --arg contract_version "trillionnium_world_map_pack_gate_v1" \
  --arg status "$OVERALL_STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg signed_manifest "$SIGNED_MANIFEST" \
  --arg unsigned_manifest "$UNSIGNED_MANIFEST" \
  --arg attribution_evidence "$ATTRIBUTION_EVIDENCE" \
  --arg sensitive_poi_report "$SENSITIVE_POI_REPORT" \
  --arg signature_verify_log "$VERIFY_LOG" \
  --arg manifest_status "$MANIFEST_STATUS" \
  --arg attribution_status "$ATTRIBUTION_STATUS" \
  --arg sensitive_status "$SENSITIVE_STATUS" \
  --arg flagged_count "$FLAGGED_COUNT" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trnm_world_map_provider_fixture_map_pack_gate",
    provider_mode: "fixture",
    public_network_ready: false,
    public_network_blocking_reason: "fixture map_pack is signed for development evidence; production/public launch still requires real map source policy, public tile/cache policy, jurisdiction/geofence review, and launch signoff",
    evidence_paths: {
      signed_manifest: $signed_manifest,
      unsigned_manifest: $unsigned_manifest,
      attribution_evidence: $attribution_evidence,
      sensitive_poi_report: $sensitive_poi_report,
      signature_verify_log: $signature_verify_log
    },
    checks: {
      manifest_status: $manifest_status,
      attribution_status: $attribution_status,
      sensitive_poi_status: $sensitive_status,
      flagged_sensitive_poi_count: ($flagged_count | tonumber)
    }
  }' >"$SUMMARY_FILE"

if [[ "$OVERALL_STATUS" != "fixture_signed_map_pack_gate_green" ]]; then
  printf 'TRILLIONNIUM_WORLD_MAP_PACK_GATE_FAILED %s\n' "$SUMMARY_FILE"
  exit 1
fi

printf 'TRILLIONNIUM_WORLD_MAP_PACK_GATE_READY %s\n' "$SUMMARY_FILE"
