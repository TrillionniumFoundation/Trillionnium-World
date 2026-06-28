#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_tech_tree.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
CORE="$ROOT/trillionnium/crates/trnm-rts-core/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_tech_tree_v1'
  'bevy-classic-rts-tech-tree.json'
  'bevy-classic-rts-tech-tree.ppm'
  'classic-rts-tech-tree'
  'input_path == "apply_live_native_action_with_source(classic_rts_tech_tree_input)"'
  'action_label_count == (.action_labels | length)'
  'input_source_count == (.input_sources | length)'
  'stage_summary_count == (.stage_summaries | length)'
  'final_command_queue_count == (.final_command_queue | length)'
  'rts_tech_tree_core_frame_order_count == (.rts_tech_tree_core_frame_orders | length)'
  'tech_tree_gate_count == 9'
  'tech_tree_passed_gate_count == 9'
  'tech_tree_failed_gate_count == 0'
  'RTS:QUEUE:faction:mirror_guard'
  'RTS:QUEUE:research:wayfinder_code@town_hall'
  'RTS:QUEUE:upgrade:iron_lacing@training_hall'
  'RTS:QUEUE:unlock:relay_guard'
  'faction_base_gate == true'
  'research_gate == true'
  'upgrade_gate == true'
  'unlock_gate == true'
  'dependency_gate == true'
  'rts_core_contract == "trnm_rts_core_frame_order_v1"'
  'rts_tech_tree_core_frame_order_gate == true'
  'rts_tech_tree_core_headless_replay_gate == true'
  'rts_tech_tree_core_frame_orders | length == 5'
  'rts_tech_tree_core_frame_order_kind_labels | tostring == "[\"queue\",\"build\",\"research\",\"upgrade\",\"unlock\"]"'
  'rts_tech_tree_core_headless_applied_order_count == 5'
  'rts_tech_tree_core_tech_order_count == 3'
  'rts_tech_tree_core_research_order_count == 1'
  'rts_tech_tree_core_upgrade_order_count == 1'
  'rts_tech_tree_core_unlock_order_count == 1'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS tech tree script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_TECH_TREE_CONTRACT'
  'native_classic_rts_tech_tree_evidence_json'
  'classic-rts-tech-tree'
  'classic_rts_tech_tree_input'
  'rts_faction_id'
  'rts_base_structure_ids'
  'rts_tech_research_ids'
  'rts_completed_upgrade_ids'
  'rts_unlocked_unit_ids'
  'rts_unlocked_structure_ids'
  'rts_tech_requirements_log'
  'rts_tech_progress_percent'
  'rts_tech_state'
  'CLASSIC_RTS_TECH_BASE_COLOR'
  'CLASSIC_RTS_TECH_RESEARCH_COLOR'
  'CLASSIC_RTS_TECH_UPGRADE_COLOR'
  'CLASSIC_RTS_TECH_UNLOCK_COLOR'
  'CLASSIC_RTS_TECH_REQUIREMENT_COLOR'
  'RtsFrameOrder::from_live_command_label'
  'RtsFrameOrderStream::new'
  'RtsOrderKind::Research'
  'RtsOrderKind::Upgrade'
  'RtsOrderKind::Unlock'
  'RtsTechTreeCheckpoint'
  'rts_tech_tree_core_frame_order_gate'
  'rts_tech_tree_core_headless_replay_gate'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN" "$CORE"; then
    echo "[FAIL] missing classic RTS tech tree source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_tech_tree.sh'
  'bevy-classic-rts-tech-tree.json'
  'classic_rts_tech_tree_green'
  'rts_tech_tree_faction_base_gate'
  'rts_tech_tree_research_gate'
  'rts_tech_tree_upgrade_gate'
  'rts_tech_tree_unlock_gate'
  'rts_tech_tree_dependency_gate'
  'rts_tech_tree_core_frame_order_gate'
  'rts_tech_tree_core_headless_replay_gate'
  'rts_tech_tree_core_frame_order_count'
  'rts_tech_tree_core_tech_order_count'
  'rts_tech_tree_core_research_order_count'
  'rts_tech_tree_core_upgrade_order_count'
  'rts_tech_tree_core_unlock_order_count'
  'rts_tech_tree_pixel_count'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS tech tree readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS tech tree evidence remains connected to faction/base, research, upgrade, unlock dependency runtime state, renderer overlays, and readiness"
