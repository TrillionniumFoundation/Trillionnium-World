#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
BASIN_SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_first_contact_basin_spec.sh"
READINESS_SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
INTEGRITY_SCRIPT="$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity.sh"
FIXTURE_LIB="$ROOT/scripts/v2/release_review_packet_integrity_visual_foundation_fixture_lib.sh"

require_line() {
  local file="$1"
  local line="$2"
  if ! grep -Fq -- "$line" "$file"; then
    echo "[FAIL] $(basename "$file") missing contract line: $line" >&2
    exit 1
  fi
}

basin_lines=(
  'ART_RENDERER_SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/first_contact_art_renderer.rs"'
  'SILHOUETTE_RENDERER_SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/first_contact_silhouette_renderer.rs"'
  'FOCUS_RENDERER_SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/first_contact_focus_renderer.rs"'
  'RADAR_RENDERER_SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/first_contact_radar_renderer.rs"'
  'RAW_OUT="$OUT.raw.$$"'
  'TMP_OUT="$OUT.tmp.$$"'
  'trap '\''rm -f "$RAW_OUT" "$TMP_OUT" "$JQ_FILTER"'\'' EXIT'
  'classic-rts-first-contact-basin-spec >"$RAW_OUT"'
  'mv "$TMP_OUT" "$OUT"'
  'required_art_renderer_source_lines=('
  'fn terrain_samples'
  'fn building_samples'
  'fn landmark_samples'
  'fn draw_terrain_material_depth_detail'
  'fn draw_terrain_detail'
  'fn draw_building_detail'
  'fn draw_landmark_detail'
  'pub(super) fn draw_readability_layer'
  'required_silhouette_renderer_source_lines=('
  'fn unit_samples'
  'fn structure_samples'
  'fn terrain_samples'
  'fn draw_unit'
  'fn draw_structure'
  'fn draw_terrain_marker'
  'pub(super) fn draw_readability_layer'
  'required_focus_renderer_source_lines=('
  'fn selection_combat_focus_route_tiles'
  'fn route_clearance_tiles'
  'fn draw_route_clearance_gutters'
  'fn draw_focus_corner_brackets'
  'fn draw_target_callout'
  'pub(super) fn draw_selection_combat_focus_layer'
  'required_radar_renderer_source_lines=('
  'fn lane_sample_tiles'
  'fn structure_tiles'
  'fn pressure_tiles'
  'fn objective_tiles'
  'pub(super) fn draw_context'
  'fn classic_first_contact_non_focus_owner_identity_color'
  'fn classic_first_contact_owner_identity_color'
  'fn classic_first_contact_base_owner_identity_tiles'
  'CLASSIC_FIRST_CONTACT_RUNTIME_CORE_VISIBLE_TILE_Y_MAX'
  'fn classic_first_contact_runtime_core_actor_candidate'
  'fn classic_first_contact_runtime_core_actor_player_visible'
  'fn classic_first_contact_runtime_core_visibility'
  'let first_contact_runtime_core_visibility_gate'
  'contract_field_count = ([keys[] | select(endswith("_contract"))] | length)'
  'guard_object_count = ([to_entries[] | select((.key | test("^first_contact_.*_guard$")) and (.value | type == "object"))] | length)'
  'top_level_gate_count = ([to_entries[] | select((.key | endswith("_gate")) and (.value | type == "boolean"))] | length)'
  'rts_data_map_model_actor_count = ((.rts_data_map_model.actors // []) | length)'
  'runtime_player_screen_command_queue_count = ((.rts_bevy_runtime_player_screen_application.command_queue // []) | length)'
  'offline_lobby_ready_label_count = ((.rts_online_offline_adapter_lobby_ready.ready_state_labels // []) | length)'
  '.guard_object_count == 16'
  '.top_level_gate_count == 46'
  'first_contact_runtime_core_visibility.runtime_core_hidden_fixture_actor_count == 49'
  'first_contact_runtime_core_visibility.runtime_core_hidden_control_fixture_actor_count == 48'
  'first_contact_runtime_core_visibility.runtime_core_required_visible_actor_ids == ["multi0.command.core","multi0.worker.0","multi0.flux.relay","map.actor15"]'
  'first_contact_runtime_core_visibility.runtime_core_hidden_control_fixture_actor_ids | index("multi0.append.seed")'
  'first_contact_runtime_core_visibility.runtime_core_control_fixture_gate == true'
  'first_contact_runtime_core_visibility.runtime_core_bottom_fixture_gate == true'
  'first_contact_runtime_core_visibility_gate == true'
  '.rts_data_map_model_rule_count == 19'
  '.runtime_player_screen_visible_tile_count == 64'
  '.offline_lobby_ready_label_count == 4'
)

readiness_lines=(
  'rts_first_contact_basin_spec_contract_field_count: $rts_first_contact_basin_spec[0].contract_field_count'
  'rts_first_contact_basin_spec_guard_object_count: $rts_first_contact_basin_spec[0].guard_object_count'
  'rts_first_contact_basin_spec_map_model_rule_count: $rts_first_contact_basin_spec[0].rts_data_map_model_rule_count'
  'rts_first_contact_basin_spec_runtime_command_queue_count: $rts_first_contact_basin_spec[0].runtime_player_screen_command_queue_count'
  'rts_first_contact_basin_spec_offline_ready_label_count: $rts_first_contact_basin_spec[0].offline_lobby_ready_label_count'
  '.headline.rts_first_contact_basin_spec_contract_field_count == 32'
  '.headline.rts_first_contact_basin_spec_top_level_gate_count == 46'
  '.headline.rts_first_contact_basin_spec_runtime_visible_tile_count == 64'
  '.headline.rts_first_contact_basin_spec_offline_ready_label_count == 4'
)

integrity_lines=(
  'first_contact_basin_spec_count_semantics'
  'runtime_player_screen_command_queue_count == ((.rts_bevy_runtime_player_screen_application.command_queue // []) | length)'
  'offline_lobby_ready_label_count == ((.rts_online_offline_adapter_lobby_ready.ready_state_labels // []) | length)'
  'first_contact_marker_budget_guard.non_focus_owner_identity_colors == ["457953","6a5e4b"]'
  'first_contact_marker_budget_guard.non_focus_owner_identity_gate == true'
  'First Contact Basin packet artifact exposes top-level contract, guard, map-model, runtime, online/offline adapter counts bound to nested structures'
  'first_contact_basin_spec_semantics_first_contact_basin_spec_count_semantics'
)

fixture_lines=(
  'contract_field_count = ([keys[] | select(endswith("_contract"))] | length)'
  'runtime_player_screen_command_queue_count = ((.rts_bevy_runtime_player_screen_application.command_queue // []) | length)'
  'offline_lobby_ready_label_count = ((.rts_online_offline_adapter_lobby_ready.ready_state_labels // []) | length)'
  'non_focus_owner_identity_colors: ["457953", "6a5e4b"]'
  'non_focus_owner_identity_gate: true'
)

for line in "${basin_lines[@]}"; do
  require_line "$BASIN_SCRIPT" "$line"
done

for line in "${readiness_lines[@]}"; do
  require_line "$READINESS_SCRIPT" "$line"
done

for line in "${integrity_lines[@]}"; do
  require_line "$INTEGRITY_SCRIPT" "$line"
done

for line in "${fixture_lines[@]}"; do
  require_line "$FIXTURE_LIB" "$line"
done

echo "[PASS] classic RTS First Contact Basin spec script keeps atomic refresh, top-level basin counts, readiness headline, packet-integrity semantics, and semantic fixtures"
