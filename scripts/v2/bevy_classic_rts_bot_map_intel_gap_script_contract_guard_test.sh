#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_map_intel_gap.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
CORE="$ROOT/trillionnium/crates/trnm-rts-core/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_bot_map_intel_gap_v1'
  'bevy-classic-rts-bot-map-intel-gap.json'
  'bevy-classic-rts-bot-map-intel-gap.ppm'
  'classic-rts-bot-map-intel-gap'
  'bevy_map_intel_vocabulary_not_openra_native_shroud_memory_ai'
  'bevy_native_shroud_memory_ai_claimed == false'
  'bevy_openra_parity_claimed == false'
  'openra_bot_economy_tech_target_commit == "f6c47d9"'
  'openra_bot_beacon_pressure_target_commit == "2b6f25b"'
  'openra_organic_bot_terminal_target_commit == "5f1bf76"'
  'intel_signal_count >= 24'
  'final_intel_state == "rotate_pressure_confirmed_beacon"'
  'rts_bot_map_intel_core_frame_order_gate == true'
  'rts_bot_map_intel_core_headless_replay_gate == true'
  'rts_bot_map_intel_core_headless_recon_order_count == 5'
  'rts_bot_map_intel_core_headless_scan_order_count == 1'
  'map_intel_gap_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS bot map-intel gap script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_BOT_MAP_INTEL_GAP_CONTRACT'
  'native_classic_rts_bot_map_intel_gap_evidence_json'
  'classic-rts-bot-map-intel-gap'
  'initial_scout_sweep'
  'fog_memory_stamp'
  'expansion_threat_inference'
  'enemy_tech_read'
  'hidden_army_prediction'
  'rotate_pressure_reveal'
  'bevy_map_intel_vocabulary_not_openra_native_shroud_memory_ai'
  'OPENRA_BOT_ECONOMY_TECH_COMMIT'
  'OPENRA_BOT_BEACON_PRESSURE_COMMIT'
  'OPENRA_ORGANIC_BOT_TERMINAL_COMMIT'
  'RtsFrameOrder::from_live_command_label'
  'first-contact-basin-bot-map-intel'
  'trnm-rts-core-bot-map-intel-rules-v1'
  'three_lane_scout_sweep'
  'fog_memory_last_seen_grid'
  'enemy_signal_array_tech'
  'rotate_pressure_to_confirmed_beacon'
  'map_intel_gap_gate'
  'rts_bot_map_intel_core_frame_order_gate'
  'rts_bot_map_intel_core_headless_replay_gate'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$CORE" "$MAIN"; then
    echo "[FAIL] missing classic RTS bot map-intel gap source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_bot_map_intel_gap.sh'
  'bevy-classic-rts-bot-map-intel-gap.json'
  'classic_rts_bot_map_intel_gap_green'
  'rts_bot_map_intel_gap_stage_count'
  'rts_bot_map_intel_gap_openra_gap_not_closed_gate'
  'rts_bot_map_intel_gap_core_frame_order_gate'
  'rts_bot_map_intel_gap_core_headless_replay_gate'
  'rts_bot_map_intel_gap_core_frame_order_stream_sha256'
  'rts_bot_map_intel_gap_core_headless_checkpoint_sha256'
  'rts_bot_map_intel_gap_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS bot map-intel gap readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS bot map-intel gap evidence remains bound to OpenRA economy/tech, beacon pressure, and organic terminal targets while keeping Bevy native shroud-memory AI parity unclaimed"
