#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_scripted_demo_replay.sh"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
LIB="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"

for line in \
  'trillionnium_world_bevy_classic_rts_scripted_demo_replay_v1' \
  'bevy-classic-rts-scripted-demo-replay.json' \
  'bevy-classic-rts-scripted-demo-replay.ppm' \
  'classic-rts-scripted-demo-replay' \
  'stage_ids == ["drag_select_frontline", "rally_path_minimap", "watch_tower_footprint", "cancel_refund", "queued_worker_ready"]' \
  'sequence_frame_gate == true' \
  'scripted_runtime_gate == true' \
  'tactical_status_gate == true' \
  'visual_feedback_gate == true' \
  'queue_tick_paused_for_screenshot_stability == true'
do
  if ! grep -F "$line" "$SCRIPT" >/dev/null; then
    echo "[FAIL] missing scripted demo replay script line: $line" >&2
    exit 1
  fi
done

for line in \
  'native_classic_rts_scripted_demo_replay_evidence_json' \
  'classic-rts-scripted-demo-replay' \
  'classic-rts-demo-replay'
do
  if ! grep -F "$line" "$MAIN" >/dev/null; then
    echo "[FAIL] missing scripted demo replay CLI line: $line" >&2
    exit 1
  fi
done

for line in \
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_SCRIPTED_DEMO_REPLAY_CONTRACT' \
  'native_classic_rts_scripted_demo_replay_evidence_json' \
  'apply_classic_rts_scripted_demo_stage_runtime(queue_cancel_refund_sequence)' \
  'classic_rts_scripted_demo_stage_id' \
  'classic_rts_scripted_demo_stage_title' \
  'classic_tactical_status_label' \
  'queue_cancel_refund_sequence' \
  'queue_tick_paused_for_screenshot_stability'
do
  if ! grep -F "$line" "$LIB" >/dev/null; then
    echo "[FAIL] missing scripted demo replay source line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS scripted demo replay remains bound to CLI, runtime stage IDs, tactical HUD status, visual frame gates, and no public/OpenRA credit"
