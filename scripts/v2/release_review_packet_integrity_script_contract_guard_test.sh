#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity.sh"

required_lines=(
  'trillionnium_world_release_review_packet_integrity_v1'
  'check_trillionnium_world_release_review_packet.sh'
  'release-review-packet-integrity.json'
  'release-review-packet-integrity-packet.log'
  'TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_JSON'
  'TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_MD'
  'TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_LOG'
  '--no-refresh'
  'sha256sum'
  'bytes_match'
  'contract_match'
  'status_match'
  'length == 58'
  'expected fifty-eight artifacts including operator handoff, checkpoint manifest, CEX adapter readiness, Bevy action coach, player HUD/debug layer, player UI rescue, classic RTS control loop, first-minute command feedback replay, first-minute command feedback recordings, first-minute command feedback contact sheet, live-window screenshots, sprite texture sampling, sampled texture live-window correlation, render asset eligibility, map modeling gate, bundle negative fixtures, evidence bundle, template negative fixtures, evidence kit, blocker consistency, status-only fixture guard, S5 real-device validation, public launch evidence intake, production map-pack collection, cohort/commercial collection, external ops collection, production map-pack public evidence, cohort/commercial validation, and external ops validation'
  'release_review_packet_integrity_green_with_public_launch_blockers'
  'android_s5_real_device_claimed: false'
  'host_side_bevy_runtime_replay_not_android_real_device'
  'packet_artifact_paths_must_exist_and_recorded_sha256_bytes_contract_status_must_match_current_files_including_checkpoint_manifest_cex_adapter_and_local_bevy_playability_evidence'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] release review packet integrity script missing contract line: $line" >&2
    exit 1
  fi
done

echo "[PASS] release review packet integrity script keeps packet refresh, checksum/byte/contract/status verification, no-refresh mode, and Android S5 boundary"
