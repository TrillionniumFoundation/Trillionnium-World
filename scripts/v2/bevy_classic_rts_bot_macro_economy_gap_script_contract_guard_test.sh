#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_macro_economy_gap.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
CORE="$ROOT/trillionnium/crates/trnm-rts-core/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_bot_macro_economy_gap_v1'
  'bevy-classic-rts-bot-macro-economy-gap.json'
  'bevy-classic-rts-bot-macro-economy-gap.ppm'
  'classic-rts-bot-macro-economy-gap'
  'bevy_macro_economy_vocabulary_not_openra_native_economy_ai'
  'bevy_native_macro_economy_ai_claimed == false'
  'bevy_openra_parity_claimed == false'
  'openra_bot_economy_tech_target_commit == "f6c47d9"'
  'openra_bot_beacon_pressure_target_commit == "2b6f25b"'
  'openra_organic_bot_terminal_target_commit == "5f1bf76"'
  'macro_signal_count >= 24'
  'final_macro_state == "deny_rebuild_pressure"'
  'rts_bot_macro_economy_core_frame_order_gate == true'
  'rts_bot_macro_economy_core_headless_replay_gate == true'
  'rts_bot_macro_economy_core_headless_build_order_count == 2'
  'rts_bot_macro_economy_core_headless_train_order_count == 3'
  'macro_economy_gap_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS bot macro economy gap script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BOT_MACRO_ECONOMY_GAP_CONTRACT'
  'native_classic_rts_bot_macro_economy_gap_evidence_json'
  'classic-rts-bot-macro-economy-gap'
  'worker_saturation_open'
  'natural_expand_timing'
  'supply_cap_recovery'
  'production_queue_cycle'
  'tech_ramp_spend'
  'resource_deny_rebuild'
  'bevy_macro_economy_vocabulary_not_openra_native_economy_ai'
  'OPENRA_BOT_ECONOMY_TECH_COMMIT'
  'OPENRA_BOT_BEACON_PRESSURE_COMMIT'
  'OPENRA_ORGANIC_BOT_TERMINAL_COMMIT'
  'RtsFrameOrder::from_live_command_label'
  'first-contact-basin-bot-macro-economy'
  'trnm-rts-core-bot-macro-economy-rules-v1'
  'natural_refinery'
  'supply_cache'
  'enemy_rebuild_node'
  'deny_enemy_node_rebuild_army'
  'macro_economy_gap_gate'
  'rts_bot_macro_economy_core_frame_order_gate'
  'rts_bot_macro_economy_core_headless_replay_gate'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$CORE" "$MAIN"; then
    echo "[FAIL] missing classic RTS bot macro economy gap source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_bot_macro_economy_gap.sh'
  'bevy-classic-rts-bot-macro-economy-gap.json'
  'classic_rts_bot_macro_economy_gap_green'
  'rts_bot_macro_economy_gap_stage_count'
  'rts_bot_macro_economy_gap_openra_gap_not_closed_gate'
  'rts_bot_macro_economy_gap_core_frame_order_gate'
  'rts_bot_macro_economy_gap_core_headless_replay_gate'
  'rts_bot_macro_economy_gap_core_frame_order_stream_sha256'
  'rts_bot_macro_economy_gap_core_headless_checkpoint_sha256'
  'rts_bot_macro_economy_gap_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS bot macro economy gap readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS bot macro economy gap evidence remains bound to OpenRA economy/tech, beacon pressure, and organic terminal targets while keeping Bevy native economy AI parity unclaimed"
