#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-first-contact-basin-spec.json"
mkdir -p "$(dirname "$OUT")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-first-contact-basin-spec >"$OUT"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_first_contact_basin_spec_v1"
  and .green == true
  and .map_id == "first_contact_basin"
  and .map_size.width == 34
  and .map_size.height == 34
  and .actor_count == 39
  and .spawn_count == 4
  and .flux_bloom_count == 11
  and .beacon_count == 4
  and .expansion_count == 4
  and .unit_rule_count >= 4
  and .building_rule_count >= 2
  and .map_actor_gate == true
  and .map_topology_gate == true
  and .rules_gate == true
  and .rts_data_contract == "trnm_rts_data_map_model_v1"
  and .rts_data_map_model.contract_version == "trnm_rts_data_map_model_v1"
  and .rts_data_map_model.map_id == "first_contact_basin"
  and (.rts_data_map_model.actors | length) == 39
  and .rts_data_map_summary.actor_count == 39
  and .rts_data_map_summary.source_integration_mode == "gpl_internal_component"
  and .rts_data_source_manifest.integration_mode == "gpl_internal_component"
  and .rts_data_source_manifest.copied_or_derived == true
  and (.rts_data_source_manifest.source_paths | index("mods/trnm/maps/first-contact-basin/map.yaml") != null)
  and (.rts_data_canonical_sha256 | type == "string" and length == 64)
  and .rts_data_validation_error == null
  and .rts_data_consumer_gate == true
  and .rts_data_terrain_profile_count == 1156
  and .rts_data_terrain_profile_samples.border.role == "border"
  and .rts_data_terrain_profile_samples.lane.role == "lane"
  and .rts_data_terrain_profile_samples.center.height == 2
  and .rts_data_terrain_profile_samples.base_pad.base_pad == true
  and .rts_data_terrain_profile_samples.resource_zone.resource_zone == true
  and .rts_data_terrain_profile_gate == true
  and .rts_data_opening_profile.contract_version == "trnm_rts_data_first_contact_opening_profile_v1"
  and .rts_data_opening_profile.map_id == "first_contact_basin"
  and .rts_data_opening_profile.active_beacon_tile.x == 16
  and .rts_data_opening_profile.active_beacon_tile.y == 9
  and .rts_data_opening_profile.active_relay_tile.x == 11
  and .rts_data_opening_profile.active_relay_tile.y == 8
  and .rts_data_command_feedback_profile.contract_version == "trnm_rts_data_first_contact_command_feedback_v1"
  and .rts_data_command_feedback_profile.target_tile.x == 16
  and .rts_data_command_feedback_profile.target_tile.y == 9
  and .rts_data_command_feedback_profile.blocked_tile.x == 15
  and .rts_data_command_feedback_profile.blocked_tile.y == 16
  and .rts_data_opening_profile_gate == true
  and .rts_data_command_feedback_gate == true
  and (.rts_data_player_startup_profiles | length) == 4
  and (.rts_data_player_startup_profiles[] | select(.player_id == "Multi0" and .faction == "horizon" and .spawn_tile.x == 8 and .spawn_tile.y == 8 and .faction_unit_rule_id == "trnm.horizon.scout"))
  and (.rts_data_player_startup_profiles[] | select(.player_id == "Multi1" and .faction == "forge" and .spawn_tile.x == 25 and .spawn_tile.y == 25 and .faction_unit_rule_id == "trnm.forge.warden"))
  and (.rts_data_player_startup_profiles[] | select(.player_id == "Multi2" and .faction == "horizon" and .spawn_tile.x == 25 and .spawn_tile.y == 8 and .faction_unit_rule_id == "trnm.horizon.scout"))
  and (.rts_data_player_startup_profiles[] | select(.player_id == "Multi3" and .faction == "forge" and .spawn_tile.x == 8 and .spawn_tile.y == 25 and .faction_unit_rule_id == "trnm.forge.warden"))
  and .rts_data_player_startup_gate == true
  and .rts_data_actor_presentation_contract == "trnm_rts_data_first_contact_actor_presentation_v1"
  and .rts_data_actor_glyph_contract == "trnm_rts_data_first_contact_actor_glyph_v1"
  and (.rts_data_actor_presentation_profiles | length) >= 13
  and (.rts_data_actor_presentation_profiles[] | select(.rule_id == "mpspawn" and .glyph.body == "spawn_pad" and .glyph.accent == "owner_stripe" and .glyph.footprint_width_cells == 3))
  and (.rts_data_actor_presentation_profiles[] | select(.rule_id == "trnm.worker" and .color_role == "worker" and .glyph_role == "worker" and .structure == false and .selectable == true and .glyph.body == "unit" and .glyph.accent == "worker_cargo" and .glyph.selection_ring == true))
  and (.rts_data_actor_presentation_profiles[] | select(.rule_id == "trnm.command.core" and .color_role == "command_core" and .glyph_role == "command_core" and .structure == true and .health_bar_width >= 32 and .glyph.body == "structure" and .glyph.accent == "command_spire" and .glyph.footprint_width_cells == 2))
  and (.rts_data_actor_presentation_profiles[] | select(.rule_id == "trnm.flux.beacon" and .color_role == "objective" and .glyph_role == "beacon" and .structure == true and .glyph.body == "objective_beacon" and .glyph.accent == "beacon_core"))
  and .rts_data_actor_presentation_gate == true
  and .bevy_data_actor_parity_gate == true
  and .bevy_map_model_adapter_gate == true
  and .ui_runtime_gate == true
  and (.rules[] | select(.id == "trnm.worker" and .cost == 200 and .hp == 8000))
  and (.rules[] | select(.id == "trnm.horizon.scout" and .speed == 92))
  and (.rules[] | select(.id == "trnm.forge.warden" and .hp == 18000))
  and (.rules[] | select(.id == "trnm.command.core" and .cost == 1600))
  and (.rules[] | select(.id == "trnm.flux.relay" and .cost == 500))
' "$OUT" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_BASIN_SPEC_GREEN %s\n' "$OUT"
