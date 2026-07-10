#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_adaptive_build_order_gap.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
CORE="$ROOT/trillionnium/crates/trnm-rts-core/src/lib.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_bot_adaptive_build_order_gap_v1'
  'bevy-classic-rts-bot-adaptive-build-order-gap.json'
  'bevy-classic-rts-bot-adaptive-build-order-gap.ppm'
  'classic-rts-bot-adaptive-build-order-gap'
  'bevy_adaptive_build_order_vocabulary_not_openra_native_ai_planner'
  'bevy_native_adaptive_ai_claimed == false'
  'bevy_openra_parity_claimed == false'
  'openra_bot_economy_tech_target_commit == "f6c47d9"'
  'openra_bot_beacon_pressure_target_commit == "2b6f25b"'
  'openra_organic_bot_terminal_target_commit == "5f1bf76"'
  'rts_core_contract == "trnm_rts_core_frame_order_v1"'
  'rts_bot_adaptive_core_frame_order_gate == true'
  'rts_bot_adaptive_core_headless_replay_gate == true'
  'rts_bot_adaptive_core_headless_applied_order_count == 9'
  'rts_bot_adaptive_core_headless_build_order_count == 2'
  'rts_bot_adaptive_core_headless_train_order_count == 2'
  'adaptive_signal_count >= 24'
  'final_adaptive_state == "pressure_window_rebuild_reattack"'
  'adaptive_build_order_gap_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS bot adaptive build-order gap script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BOT_ADAPTIVE_BUILD_ORDER_GAP_CONTRACT'
  'native_classic_rts_bot_adaptive_build_order_gap_evidence_json'
  'classic-rts-bot-adaptive-build-order-gap'
  'opening_worker_split'
  'scout_trigger_response'
  'expand_or_defend_branch'
  'tech_counter_switch'
  'pressure_window_commit'
  'retreat_rebuild_reattack'
  'RTS:QUEUE:build:relay_refinery@5,5'
  'RTS:QUEUE:recon:scout:enemy_fast_beacon@6,5'
  'RTS:QUEUE:research:signal_array@town_hall'
  'RTS:MOVE:9,5:pullback_rebuild_then_reattack'
  'trnm-rts-core-bot-adaptive-build-rules-v1'
  'headless_replay_tracks_bot_adaptive_build_order_stream'
  'bevy_adaptive_build_order_vocabulary_not_openra_native_ai_planner'
  'OPENRA_BOT_ECONOMY_TECH_COMMIT'
  'OPENRA_BOT_BEACON_PRESSURE_COMMIT'
  'OPENRA_ORGANIC_BOT_TERMINAL_COMMIT'
  'adaptive_build_order_gap_gate'
  'rts_bot_adaptive_core_frame_order_gate'
  'rts_bot_adaptive_core_headless_replay_gate'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN" "$CORE"; then
    echo "[FAIL] missing classic RTS bot adaptive build-order gap source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_bot_adaptive_build_order_gap.sh'
  'bevy-classic-rts-bot-adaptive-build-order-gap.json'
  'classic_rts_bot_adaptive_build_order_gap_green'
  'rts_bot_adaptive_build_order_gap_stage_count'
  'rts_bot_adaptive_build_order_gap_core_frame_order_stream_sha256'
  'rts_bot_adaptive_build_order_gap_core_headless_checkpoint_sha256'
  'rts_bot_adaptive_build_order_gap_openra_gap_not_closed_gate'
  'rts_bot_adaptive_build_order_gap_core_frame_order_gate'
  'rts_bot_adaptive_build_order_gap_core_headless_replay_gate'
  'rts_bot_adaptive_build_order_gap_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS bot adaptive build-order gap readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS bot adaptive build-order gap evidence remains bound to OpenRA economy/tech, beacon pressure, and organic terminal targets while keeping Bevy native AI planner parity unclaimed"
