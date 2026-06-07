#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_full_screen_ui_replication.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
RELEASE_CI="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

test -x "$SCRIPT"

required_script_lines=(
  'classic-rts-full-screen-ui-replication'
  'bevy-classic-rts-full-screen-ui-replication.json'
  'bevy-classic-rts-full-screen-ui-replication.ppm'
  'trillionnium_world_bevy_classic_rts_full_screen_ui_replication_v1'
  'trillionnium_world_bevy_classic_rts_campaign_entry_v1'
  'trillionnium_world_bevy_classic_rts_visual_fidelity_v1'
  'trillionnium_world_bevy_classic_rts_map_ui_modeling_readiness_v1'
  'trillionnium_world_bevy_classic_rts_production_ui_skin_v1'
  'trillionnium_world_bevy_classic_rts_production_interaction_polish_v1'
  'trillionnium_world_bevy_classic_rts_build_lifecycle_v1'
  'trillionnium_world_bevy_classic_rts_tech_tree_v1'
  'trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness_v1'
  'trillionnium_world_bevy_classic_rts_combat_readability_pressure_readiness_v1'
  'replication_surface_count == 10'
  'runtime_screen_gate == true'
  'evidence_board_only == false'
  'full_screen_ui_replication_gate == true'
  'external_evidence_ignored_for_current_replication_pass == true'
  'screen_for_screen_openra_ui_claimed == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FULL_SCREEN_UI_REPLICATION_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing full screen/UI replication script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FULL_SCREEN_UI_REPLICATION_CONTRACT'
  'native_classic_rts_full_screen_ui_replication_evidence_json'
  'TRNM RUST/BEVY FULL SCREEN RUNTIME SURFACE'
  'native_classic_rts_campaign_entry_evidence_json'
  'native_classic_rts_visual_fidelity_evidence_json'
  'native_classic_rts_map_ui_modeling_readiness_evidence_json'
  'native_classic_rts_production_ui_skin_evidence_json'
  'native_classic_rts_production_interaction_polish_evidence_json'
  'native_classic_rts_build_lifecycle_evidence_json'
  'native_classic_rts_tech_tree_evidence_json'
  'native_classic_rts_campaign_outcome_ui_readiness_evidence_json'
  'native_classic_rts_combat_readability_pressure_readiness_evidence_json'
  'TITLE/CAMPAIGN ENTRY'
  'TACTICAL VIEWPORT'
  'MAP/MINIMAP CAMERA'
  'PRODUCTION HUD SKIN'
  'COMMAND INTERACTIONS'
  'BUILD + TECH TREE'
  'UNIT STATUS CARD'
  'ABILITY/COMBAT UI'
  'CAMPAIGN OUTCOME'
  'OPEN-WORLD HANDOFF'
  'full_screen_ui_replication_gate'
  'runtime_screen_gate'
  'player_runtime_screen'
  'external_evidence_ignored_for_current_replication_pass'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing full screen/UI replication source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_full_screen_ui_replication.sh'
  'rts_full_screen_ui_replication'
  'classic_rts_full_screen_ui_replication_green'
  'bevy-classic-rts-full-screen-ui-replication.json'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing full screen/UI replication readiness line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'check_trillionnium_world_bevy_classic_rts_full_screen_ui_replication.sh'
  'bevy_classic_rts_full_screen_ui_replication_script_contract_guard_test.sh'
  'bevy_classic_rts_full_screen_ui_replication_gate'
  'trillionnium_world_bevy_classic_rts_full_screen_ui_replication_v1'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE_CI"; then
    echo "[FAIL] missing full screen/UI replication release CI line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS full screen/UI replication gate remains connected to Rust CLI, internal runtime screen sources, playtest readiness, release-review CI, and no-external-evidence boundaries"
