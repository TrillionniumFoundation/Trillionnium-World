#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_public_launch_evidence_intake.sh"

required_lines=(
  'trillionnium_world_public_launch_evidence_intake_v1'
  'check_trillionnium_world_public_launch_readiness.sh'
  'public-launch-evidence-intake.json'
  'public-launch-evidence-intake.md'
  'release_review_acceptance_lock.sh'
  'trnm_acquire_release_review_acceptance_lock "$ACCEPTANCE_DIR"'
  'public-launch-evidence-intake-readiness.log'
  's5-device-evidence.template.json'
  'collection_command'
  'check_trillionnium_world_s5_device_evidence.sh --require-device'
  'check_trillionnium_world_production_map_pack_public_evidence_collection.sh'
  'check_trillionnium_world_cohort_commercial_evidence_collection.sh'
  'check_trillionnium_world_external_ops_evidence_collection.sh'
  'multi-node-latency-evidence.template.json'
  'public_launch_evidence_intake_ready_for_operator_collection'
  'public_launch_evidence_intake_complete_green'
  'TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_EVIDENCE_INTAKE_SUMMARY'
  'TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_EVIDENCE_INTAKE_MD'
  'TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH'
  'TRILLIONNIUM_FIRST_BETA_COHORT_EVIDENCE_PATH'
  'TRILLIONNIUM_COMMERCIAL_LAUNCH_DRILL_EVIDENCE_PATH'
  'TRILLIONNIUM_MULTI_NODE_LATENCY_EVIDENCE_PATH'
  'TRILLIONNIUM_PUBLIC_NETWORK_DEPLOY_EVIDENCE_PATH'
  'ANDROID_SERIAL'
  'weak-network'
  'APK resource/signature'
  'public_launch_claimed: false'
  'android_s5_real_device_claimed: false'
  'live_map_ingestion_performed: false'
  'live_public_exposure_performed: false'
  '.public_launch_ready // (.overall_status == "ready_for_public_launch_review")'
  'collect_real_external_public_launch_evidence_without_claiming_public_launch_ready_or_android_s5_real_device_ready'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] public launch evidence intake script missing contract line: $line" >&2
    exit 1
  fi
done

echo "[PASS] public launch evidence intake script keeps real-evidence checklist, env hooks, and no-claim boundaries"
