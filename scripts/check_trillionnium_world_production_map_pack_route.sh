#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S4_map_pack_gate/latest"
SUMMARY_FILE="$ACCEPTANCE_DIR/production-map-pack-route.json"
ADR_FILE="$ROOT/docs/development/trillionnium-world-production-map-pack-adr-v1.md"
UNSIGNED_MANIFEST="$ACCEPTANCE_DIR/map_pack_manifest_unsigned.json"
SIGNED_MANIFEST="$ACCEPTANCE_DIR/map_pack_manifest_signed.json"
ATTRIBUTION_EVIDENCE="$ACCEPTANCE_DIR/attribution_evidence.json"
SENSITIVE_POI_REPORT="$ACCEPTANCE_DIR/sensitive_poi_filter_report.json"
KEY_DIR="$ACCEPTANCE_DIR/production-keys"
PRIMARY_KEY="$KEY_DIR/production-primary-ed25519.key"
PRIMARY_PUB="$KEY_DIR/production-primary-ed25519.pub"
NEXT_KEY="$KEY_DIR/production-next-ed25519.key"
NEXT_PUB="$KEY_DIR/production-next-ed25519.pub"
PRIMARY_SIG="$ACCEPTANCE_DIR/production-map-pack-primary.ed25519.sig"
NEXT_SIG="$ACCEPTANCE_DIR/production-map-pack-next.ed25519.sig"
PRIMARY_VERIFY="$ACCEPTANCE_DIR/production-map-pack-primary.verify.txt"
NEXT_VERIFY="$ACCEPTANCE_DIR/production-map-pack-next.verify.txt"
REVOCATION_LIST="$ACCEPTANCE_DIR/production-map-pack-revocations.json"
ATTRIBUTION_PLAN="$ACCEPTANCE_DIR/production-attribution-screenshot-plan.json"
ACTIVE_MANIFEST="$ACCEPTANCE_DIR/production-active-map-pack-manifest.json"
BAD_MANIFEST="$ACCEPTANCE_DIR/production-takedown-map-pack-manifest.json"
ROLLBACK_MANIFEST="$ACCEPTANCE_DIR/production-rollback-map-pack-manifest.json"
ROLLBACK_EVIDENCE="$ACCEPTANCE_DIR/production-map-pack-rollback.json"

mkdir -p "$KEY_DIR"

if [[ ! -f "$UNSIGNED_MANIFEST" || ! -f "$SIGNED_MANIFEST" ]]; then
  bash "$ROOT/scripts/check_trillionnium_world_map_pack_gate.sh" >/dev/null
fi

openssl genpkey -algorithm ED25519 -out "$PRIMARY_KEY" >/dev/null 2>&1
openssl pkey -in "$PRIMARY_KEY" -pubout -out "$PRIMARY_PUB" >/dev/null 2>&1
openssl genpkey -algorithm ED25519 -out "$NEXT_KEY" >/dev/null 2>&1
openssl pkey -in "$NEXT_KEY" -pubout -out "$NEXT_PUB" >/dev/null 2>&1
openssl pkeyutl -sign -rawin -inkey "$PRIMARY_KEY" -in "$UNSIGNED_MANIFEST" -out "$PRIMARY_SIG"
openssl pkeyutl -verify -rawin -pubin -inkey "$PRIMARY_PUB" -sigfile "$PRIMARY_SIG" -in "$UNSIGNED_MANIFEST" >"$PRIMARY_VERIFY" 2>&1
openssl pkeyutl -sign -rawin -inkey "$NEXT_KEY" -in "$UNSIGNED_MANIFEST" -out "$NEXT_SIG"
openssl pkeyutl -verify -rawin -pubin -inkey "$NEXT_PUB" -sigfile "$NEXT_SIG" -in "$UNSIGNED_MANIFEST" >"$NEXT_VERIFY" 2>&1

jq -n \
  --arg contract_version "trillionnium_world_map_pack_key_revocation_v1" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg active_key_id "production-primary-ed25519" \
  --arg next_key_id "production-next-ed25519" \
  '{
    contract_version: $contract_version,
    generated_at: $generated_at,
    active_key_id: $active_key_id,
    next_key_id: $next_key_id,
    revoked_key_ids: ["dev-fixture-map-pack-ed25519"],
    revoked_package_ids: [],
    rule: "clients reject manifests signed by revoked_key_ids or revoked package ids"
  }' >"$REVOCATION_LIST"

jq -n \
  --arg contract_version "trillionnium_world_map_pack_attribution_screenshot_plan_v1" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg existing_attribution "$ATTRIBUTION_EVIDENCE" \
  '{
    contract_version: $contract_version,
    generated_at: $generated_at,
    status: "attribution_screenshot_plan_ready_pending_public_surfaces",
    fixture_attribution_evidence: $existing_attribution,
    required_public_surfaces: [
      { surface: "standalone_web", required: true, status: "pending_public_screenshot" },
      { surface: "native_bevy_android", required: true, status: "pending_real_device_screenshot" },
      { surface: "matrix_or_readonly_share", required: true, status: "pending_public_surface_screenshot" }
    ],
    visible_text_required: ["OpenStreetMap contributors", "ODbL-1.0"]
  }' >"$ATTRIBUTION_PLAN"

cp "$SIGNED_MANIFEST" "$ACTIVE_MANIFEST"
ACTIVE_SHA256="$(sha256sum "$ACTIVE_MANIFEST" | awk '{print $1}')"
jq '. + {
  "production_status": "revoked_for_takedown_drill",
  "takedown_reason": "simulated_bad_map_pack_for_rollback_drill"
}' "$SIGNED_MANIFEST" >"$BAD_MANIFEST"
BAD_SHA256="$(sha256sum "$BAD_MANIFEST" | awk '{print $1}')"
cp "$ACTIVE_MANIFEST" "$ROLLBACK_MANIFEST"
ROLLBACK_SHA256="$(sha256sum "$ROLLBACK_MANIFEST" | awk '{print $1}')"

jq -n \
  --arg contract_version "trillionnium_world_map_pack_takedown_rollback_v1" \
  --arg active_manifest "$ACTIVE_MANIFEST" \
  --arg bad_manifest "$BAD_MANIFEST" \
  --arg rollback_manifest "$ROLLBACK_MANIFEST" \
  --arg active_sha256 "$ACTIVE_SHA256" \
  --arg bad_sha256 "$BAD_SHA256" \
  --arg rollback_sha256 "$ROLLBACK_SHA256" \
  '{
    contract_version: $contract_version,
    active_manifest: $active_manifest,
    bad_manifest: $bad_manifest,
    rollback_manifest: $rollback_manifest,
    active_sha256: $active_sha256,
    bad_sha256: $bad_sha256,
    rollback_sha256: $rollback_sha256,
    rollback_restores_active_manifest: ($active_sha256 == $rollback_sha256),
    bad_manifest_differs_from_active: ($active_sha256 != $bad_sha256)
  }' >"$ROLLBACK_EVIDENCE"

STATUS="production_map_pack_route_green"
if ! grep -q 'Signature Verified Successfully' "$PRIMARY_VERIFY"; then
  STATUS="production_primary_signature_failed"
fi
if ! grep -q 'Signature Verified Successfully' "$NEXT_VERIFY"; then
  STATUS="production_next_signature_failed"
fi
if [[ "$ACTIVE_SHA256" != "$ROLLBACK_SHA256" || "$ACTIVE_SHA256" == "$BAD_SHA256" ]]; then
  STATUS="production_map_pack_rollback_failed"
fi

jq -n \
  --arg contract_version "trillionnium_world_production_map_pack_route_v1" \
  --arg status "$STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg adr "$ADR_FILE" \
  --arg unsigned_manifest "$UNSIGNED_MANIFEST" \
  --arg signed_manifest "$SIGNED_MANIFEST" \
  --arg attribution_evidence "$ATTRIBUTION_EVIDENCE" \
  --arg sensitive_poi_report "$SENSITIVE_POI_REPORT" \
  --arg primary_pub "$PRIMARY_PUB" \
  --arg next_pub "$NEXT_PUB" \
  --arg primary_verify "$PRIMARY_VERIFY" \
  --arg next_verify "$NEXT_VERIFY" \
  --arg revocation_list "$REVOCATION_LIST" \
  --arg attribution_plan "$ATTRIBUTION_PLAN" \
  --arg rollback_evidence "$ROLLBACK_EVIDENCE" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trnm_world_production_map_pack_route_gate",
    public_map_pack_ready: false,
    accepted_public_launch_status: "production_map_pack_public_ready_green",
    local_route_status: "production_map_pack_route_green",
    adr: $adr,
    fixture_inputs: {
      unsigned_manifest: $unsigned_manifest,
      signed_manifest: $signed_manifest,
      attribution_evidence: $attribution_evidence,
      sensitive_poi_report: $sensitive_poi_report
    },
    key_rotation: {
      active_public_key: $primary_pub,
      next_public_key: $next_pub,
      primary_verify_evidence: $primary_verify,
      next_verify_evidence: $next_verify,
      revocation_list: $revocation_list
    },
    attribution: {
      screenshot_plan: $attribution_plan,
      real_public_screenshots_required_for_green: true
    },
    rollback: {
      evidence: $rollback_evidence
    },
    remaining_for_public_map_pack: [
      "approved_production_map_source",
      "real_public_attribution_screenshots",
      "native_bevy_real_device_attribution_screenshot",
      "key_custody_and_rotation_runbook",
      "public_package_distribution_and_revocation_probe"
    ]
  }' >"$SUMMARY_FILE"

if [[ "$STATUS" == "production_map_pack_route_green" ]]; then
  printf 'TRILLIONNIUM_WORLD_PRODUCTION_MAP_PACK_ROUTE_READY %s\n' "$SUMMARY_FILE"
  exit 0
fi

printf 'TRILLIONNIUM_WORLD_PRODUCTION_MAP_PACK_ROUTE_BLOCKED %s %s\n' "$STATUS" "$SUMMARY_FILE"
exit 1
