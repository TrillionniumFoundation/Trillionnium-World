#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-match-setup-ui-replication.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-match-setup-ui-replication.ppm"
mkdir -p "$(dirname "$SUMMARY")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-match-setup-ui-replication "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_match_setup_ui_replication_v1"
  and .status == "classic_rts_match_setup_ui_replication_green"
  and .green == true
  and .preview_width == 1280
  and .preview_height == 768
  and .preview_format == "ppm_p3_rgb"
  and .source_contracts.shell_meta_ui_replication == "trillionnium_world_bevy_classic_rts_shell_meta_ui_replication_v1"
  and .source_contracts.campaign_entry == "trillionnium_world_bevy_classic_rts_campaign_entry_v1"
  and .source_contracts.first_contact_basin_spec == "trillionnium_world_bevy_classic_rts_first_contact_basin_spec_v1"
  and .source_contracts.map_ui_modeling_readiness == "trillionnium_world_bevy_classic_rts_map_ui_modeling_readiness_v1"
  and .source_contracts.tech_tree == "trillionnium_world_bevy_classic_rts_tech_tree_v1"
  and .setup_surface_count == 10
  and (.setup_surface_names | index("CAMPAIGN ACTIONS") != null)
  and (.setup_surface_names | index("MAP SELECT") != null)
  and (.setup_surface_names | index("FACTION SELECT") != null)
  and (.setup_surface_names | index("SPAWN SLOTS") != null)
  and (.setup_surface_names | index("RESOURCE RULES") != null)
  and (.setup_surface_names | index("BOT / DIFFICULTY") != null)
  and (.setup_surface_names | index("VICTORY CONDITIONS") != null)
  and (.setup_surface_names | index("MINIMAP PREVIEW") != null)
  and (.setup_surface_names | index("START READY") != null)
  and (.setup_surface_names | index("NO-EXTERNAL BOUNDARY") != null)
  and (.setup_slot_ids | index("campaign_start_continue_replay") != null)
  and (.setup_slot_ids | index("first_contact_basin") != null)
  and (.setup_slot_ids | index("mirror_guard") != null)
  and (.setup_slot_ids | index("four_spawn_lanes") != null)
  and (.setup_slot_ids | index("flux_beacons_expansions") != null)
  and (.setup_slot_ids | index("local_bot_fixture") != null)
  and (.setup_slot_ids | index("beacon_extract") != null)
  and (.setup_slot_ids | index("camera_fog_spawn") != null)
  and (.setup_slot_ids | index("ready_to_start") != null)
  and (.setup_slot_ids | index("no_s5_no_public") != null)
  and .setup_pixel_counts.board > 80000
  and .setup_pixel_counts.campaign_actions > 2000
  and .setup_pixel_counts.map_select > 2000
  and .setup_pixel_counts.faction_select > 2000
  and .setup_pixel_counts.spawn_slots > 2000
  and .setup_pixel_counts.resource_rules > 2000
  and .setup_pixel_counts.bot_difficulty > 2000
  and .setup_pixel_counts.victory_conditions > 2000
  and .setup_pixel_counts.minimap_preview > 2000
  and .setup_pixel_counts.start_ready > 2000
  and .setup_pixel_counts.boundary > 2000
  and .setup_pixel_counts.highlight > 3000
  and .source_headline.shell_meta_surface_count == 12
  and .source_headline.campaign_input_action_count == 73
  and .source_headline.campaign_slot_bytes > 20000
  and .source_headline.map_id == "first_contact_basin"
  and .source_headline.map_spawn_count == 4
  and .source_headline.map_actor_count == 39
  and .source_headline.map_ui_preview_count == 6
  and .source_headline.faction_id == "mirror_guard"
  and .source_headline.tech_state == "unlocked:relay_guard"
  and .shell_meta_gate == true
  and .campaign_entry_gate == true
  and .map_spec_gate == true
  and .map_ui_gate == true
  and .faction_gate == true
  and .no_external_boundary_gate == true
  and .setup_preview_gate == true
  and .source_preview_gate == true
  and .match_setup_ui_replication_gate == true
  and .internal_match_setup_ui_replication_claimed == true
  and .external_evidence_ignored_for_current_replication_pass == true
  and .android_s5_real_device_claimed == false
  and .public_launch_ready == false
  and .screen_for_screen_openra_ui_claimed == false
  and .openra_engine_port_claimed == false
  and .warcraft_iii_asset_copied == false
  and .openra_asset_copied == false
  and .third_party_asset_copied == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_MATCH_SETUP_UI_REPLICATION_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
