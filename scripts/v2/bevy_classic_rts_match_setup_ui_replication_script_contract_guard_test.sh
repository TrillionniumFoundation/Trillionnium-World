#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_match_setup_ui_replication.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
RELEASE_CI="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

test -x "$SCRIPT"

required_script_lines=(
  'classic-rts-match-setup-ui-replication'
  'bevy-classic-rts-match-setup-ui-replication.json'
  'bevy-classic-rts-match-setup-ui-replication.ppm'
  'trillionnium_world_bevy_classic_rts_match_setup_ui_replication_v1'
  'trillionnium_world_bevy_classic_rts_shell_meta_ui_replication_v1'
  'trillionnium_world_bevy_classic_rts_campaign_entry_v1'
  'trillionnium_world_bevy_classic_rts_first_contact_basin_spec_v1'
  'trillionnium_world_bevy_classic_rts_map_ui_modeling_readiness_v1'
  'trillionnium_world_bevy_classic_rts_tech_tree_v1'
  'setup_surface_count == 10'
  'source_contract_count == (.source_contracts | keys | length)'
  'source_path_count == (.source_paths | keys | length)'
  'source_headline_field_count == (.source_headline | keys | length)'
  'runtime_screen_layout_count == (.runtime_screen_layout | keys | length)'
  'setup_pixel_count_field_count == (.setup_pixel_counts | keys | length)'
  'match_setup_player_first_pixel_count_field_count == (.match_setup_player_first_pixel_counts | keys | length)'
  'setup_surface_name_count == (.setup_surface_names | length)'
  'setup_slot_id_count == (.setup_slot_ids | length)'
  'setup_source_surface_count == (.setup_source_surfaces | length)'
  'gate_count == 11'
  'passed_gate_count == 11'
  'failed_gate_count == 0'
  'runtime_screen_gate == true'
  'player_first_match_setup_screen_gate == true'
  'evidence_board_only == false'
  'match_setup_ui_replication_gate == true'
  'external_evidence_ignored_for_current_replication_pass == true'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_MATCH_SETUP_UI_REPLICATION_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing match setup UI replication script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_MATCH_SETUP_UI_REPLICATION_CONTRACT'
  'native_classic_rts_match_setup_ui_replication_evidence_json'
  'TRNM RUST/BEVY MATCH SETUP RUNTIME SURFACE'
  'native_classic_rts_shell_meta_ui_replication_evidence_json'
  'native_classic_rts_campaign_entry_evidence_json'
  'native_classic_rts_first_contact_basin_spec_evidence_json'
  'native_classic_rts_map_ui_modeling_readiness_evidence_json'
  'native_classic_rts_tech_tree_evidence_json'
  'CAMPAIGN ACTIONS'
  'MAP SELECT'
  'FACTION SELECT'
  'SPAWN SLOTS'
  'RESOURCE RULES'
  'BOT / DIFFICULTY'
  'VICTORY CONDITIONS'
  'MINIMAP PREVIEW'
  'START READY'
  'NO-EXTERNAL BOUNDARY'
  'match_setup_player_first_pixel_counts'
  'player_first_match_setup_map_non_background'
  'player_first_match_setup_screen_gate'
  'large First Contact Basin tactical setup viewport'
  'in-map camera fog and spawn-lane preview'
  'faction, resources, bot, victory, and boundary confirmation rail'
  'bottom player launch strip with local Rust/Bevy ready state'
  'match_setup_ui_replication_gate'
  'runtime_screen_gate'
  'player_runtime_match_setup_screen'
  'external_evidence_ignored_for_current_replication_pass'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing match setup UI replication source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_match_setup_ui_replication.sh'
  'rts_match_setup_ui_replication'
  'classic_rts_match_setup_ui_replication_green'
  'bevy-classic-rts-match-setup-ui-replication.json'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing match setup UI replication readiness line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'check_trillionnium_world_bevy_classic_rts_match_setup_ui_replication.sh'
  'bevy_classic_rts_match_setup_ui_replication_script_contract_guard_test.sh'
  'bevy_classic_rts_match_setup_ui_replication_gate'
  'trillionnium_world_bevy_classic_rts_match_setup_ui_replication_v1'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE_CI"; then
    echo "[FAIL] missing match setup UI replication release CI line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS match setup UI replication gate remains connected to Rust CLI, pre-match runtime screen sources, playtest readiness, release-review CI, and no-external-evidence boundaries"
