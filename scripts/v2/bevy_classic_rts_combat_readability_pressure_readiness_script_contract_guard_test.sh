#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_combat_readability_pressure_readiness.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
RELEASE_CI="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
PACKET="$ROOT/scripts/check_trillionnium_world_release_review_packet.sh"
INTEGRITY="$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity.sh"

test -x "$SCRIPT"

required_script_lines=(
  'classic-rts-combat-readability-pressure-readiness'
  'bevy-classic-rts-combat-readability-pressure-readiness.json'
  'bevy-classic-rts-combat-readability-pressure-readiness'
  'trillionnium_world_bevy_classic_rts_combat_readability_pressure_readiness_v1'
  'classic_rts_combat_readability_pressure_readiness_green'
  'runtime_screen_mode == "player_runtime_combat_pressure_screen"'
  'runtime_screen_gate == true'
  'evidence_board_only == false'
  'source_contract_count == (.source_contracts | keys | length)'
  'preview_path_count == (.preview_paths | keys | length)'
  'runtime_screen_layout_count == (.runtime_screen_layout | keys | length)'
  'combat_pressure_pixel_count_field_count == (.combat_pressure_pixel_counts | keys | length)'
  'unit_status_summary_field_count == (.unit_status_summary | keys | length)'
  'command_feedback_summary_field_count == (.command_feedback_summary | keys | length)'
  'ability_telegraph_summary_field_count == (.ability_telegraph_summary | keys | length)'
  'depth_summary_field_count == (.depth_summary | keys | length)'
  'pressure_summary_field_count == (.pressure_summary | keys | length)'
  'gate_count == 10'
  'passed_gate_count == 10'
  'failed_gate_count == 0'
  'selected unit portrait, bars, role, and queue badges'
  'marquee, attack, error, and acknowledgment feedback'
  'central keep shield, guard, siege line, and defeat-risk feedback'
  'trillionnium_world_bevy_classic_rts_unit_status_portrait_v1'
  'trillionnium_world_bevy_classic_rts_selection_command_feedback_v1'
  'trillionnium_world_bevy_classic_rts_ability_tooltip_telegraph_v1'
  'trillionnium_world_bevy_classic_rts_depth_readability_v1'
  'trillionnium_world_bevy_classic_rts_central_keep_pressure_v1'
  'preview_count == 5'
  'unit_status_gate == true'
  'command_feedback_gate == true'
  'ability_telegraph_gate == true'
  'depth_readability_gate == true'
  'pressure_feedback_gate == true'
  'combat_readability_pressure_readiness_gate == true'
  'external_evidence_ignored_for_current_combat_readability_pass == true'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_COMBAT_READABILITY_PRESSURE_READINESS_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS combat readability/pressure readiness script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_COMBAT_READABILITY_PRESSURE_READINESS_CONTRACT'
  'native_classic_rts_combat_readability_pressure_readiness_evidence_json'
  'classic-rts-combat-readability-pressure-readiness'
  'unit-status-portrait.ppm'
  'selection-command-feedback.ppm'
  'ability-tooltip-telegraph.ppm'
  'depth-readability.ppm'
  'central-keep-pressure.ppm'
  'classic_rts_combat_readability_pressure_readiness_green'
  'player_runtime_combat_pressure_screen'
  'runtime_screen_layout'
  'combat_readability_pressure_readiness_gate'
  'internal_combat_readability_pressure_readiness_claimed'
  'external_evidence_ignored_for_current_combat_readability_pass'
  'android_s5_real_device_claimed'
  'public_launch_ready'
  'screen_for_screen_openra_ui_claimed'
  'openra_engine_port_claimed'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS combat readability/pressure readiness source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_combat_readability_pressure_readiness.sh'
  'rts_combat_readability_pressure_readiness'
  'classic_rts_combat_readability_pressure_readiness_green'
  'bevy-classic-rts-combat-readability-pressure-readiness.json'
  'bevy-classic-rts-combat-readability-pressure-readiness/'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS combat readability/pressure readiness readiness line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'check_trillionnium_world_bevy_classic_rts_combat_readability_pressure_readiness.sh'
  'bevy_classic_rts_combat_readability_pressure_readiness_script_contract_guard_test.sh'
  'bevy_classic_rts_combat_readability_pressure_readiness_gate'
  'trillionnium_world_bevy_classic_rts_combat_readability_pressure_readiness_v1'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE_CI"; then
    echo "[FAIL] missing classic RTS combat readability/pressure readiness release CI line: $line" >&2
    exit 1
  fi
done

required_packet_lines=(
  'COMBAT_READABILITY_PRESSURE_READINESS_LOG'
  'native_bevy_classic_rts_combat_readability_pressure_readiness'
  'Native/Bevy classic RTS combat readability/pressure readiness'
  'bevy-classic-rts-combat-readability-pressure-readiness.json'
)

for line in "${required_packet_lines[@]}"; do
  if ! grep -Fq "$line" "$PACKET"; then
    echo "[FAIL] missing classic RTS combat readability/pressure readiness release packet line: $line" >&2
    exit 1
  fi
done

required_integrity_lines=(
  'combat_readability_pressure_readiness_semantics'
  'native_bevy_classic_rts_combat_readability_pressure_readiness'
  'trillionnium_world_bevy_classic_rts_combat_readability_pressure_readiness_v1'
  'classic_rts_combat_readability_pressure_readiness_green'
  'source_contract_count == (.source_contracts | keys | length)'
  'combat_pressure_pixel_count_field_count == (.combat_pressure_pixel_counts | keys | length)'
  'runtime_screen_mode == "player_runtime_combat_pressure_screen"'
  'runtime_screen_gate == true'
  'evidence_board_only == false'
  'internal_combat_readability_pressure_readiness_claimed == true'
  'external_evidence_ignored_for_current_combat_readability_pass == true'
)

for line in "${required_integrity_lines[@]}"; do
  if ! grep -Fq "$line" "$INTEGRITY"; then
    echo "[FAIL] missing classic RTS combat readability/pressure readiness packet integrity line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS combat readability/pressure readiness stays connected to Rust CLI, playtest readiness, release packet, packet integrity semantics, release-review CI, and no-external-evidence boundaries"
