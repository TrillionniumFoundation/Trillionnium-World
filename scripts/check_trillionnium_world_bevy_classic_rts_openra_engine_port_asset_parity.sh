#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-engine-port-asset-parity.json"
PREVIEW="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-engine-port-asset-parity.ppm"
PREVIEW_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-engine-port-asset-parity"
mkdir -p "$(dirname "$SUMMARY")" "$PREVIEW_DIR"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-openra-engine-port-asset-parity "$PREVIEW" >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_engine_port_asset_parity_v1"
  and .status == "classic_rts_openra_engine_port_asset_parity_green"
  and .green == true
  and .preview_width == 1920
  and .preview_height == 1080
  and .preview_format == "ppm_p3_rgb"
  and .engine_port_mode == "rust_reimplementation_of_openra_engine_foundation_owned_assets"
  and .openra_engine_port_scope == "moddata_ruleset_actor_world_order_chrome_widget_sprite_palette_asset_loader_replay_foundation"
  and .ported_engine_module_count >= 10
  and ([.ported_engine_modules[]] | index("ModData: owned mod manifest, package load order, rules/chrome source registry") != null)
  and ([.ported_engine_modules[]] | index("Ruleset: actor rules, prerequisites, production, weapons, terrain, and traits") != null)
  and ([.ported_engine_modules[]] | index("OrderManager: deterministic issue-order queue, validation, rejection, and replay hooks") != null)
  and ([.ported_engine_modules[]] | index("ChromeProvider: widget-root lookup, screen id binding, modal overlay routing") != null)
  and ([.ported_engine_modules[]] | index("SpriteSequence: frame id, facing set, fps, loop policy, and texture-atlas rects") != null)
  and .openra_widget_root_count == 4
  and .openra_chrome_screen_count == 8
  and .source_contracts.openra_screen_for_screen_ui_replication == "trillionnium_world_bevy_classic_rts_openra_screen_for_screen_ui_replication_v1"
  and .source_contracts.openra_like_core == "trillionnium_world_bevy_classic_rts_openra_like_core_v1"
  and .source_contracts.classic_asset_pack == "trillionnium_world_bevy_classic_asset_pack_v1"
  and .source_headline.openra_screen_for_screen_claimed == true
  and .source_headline.openra_reference_screen_count == 8
  and .source_headline.openra_like_runtime_model == "rust_bevy_owned_openra_like_rts_core"
  and .source_headline.rules_count >= 10
  and .source_headline.actor_template_count >= 39
  and .source_headline.simulation_tick_count >= 320
  and .asset_manifest.atlas_format == "ppm_p3_rgb"
  and .asset_manifest.frame_count >= 40
  and .asset_manifest.scene_count >= 3
  and .asset_manifest.actor_count >= 3
  and .asset_manifest.loaded_from_manifest == true
  and .asset_manifest.atlas_parse_gate == true
  and (.asset_manifest.manifest_sha256 | type == "string" and length == 64)
  and (.port_manifest_sha256 | type == "string" and length == 64)
  and .pixel_parity.scope == "trillionnium_owned_openra_compatible_asset_pack"
  and .pixel_parity.coverage == "full_classic_asset_manifest_frame_set"
  and .pixel_parity.sample_count == .asset_manifest.frame_count
  and .pixel_parity.manifest_frame_count == .asset_manifest.frame_count
  and .pixel_parity.sample_sha_match_count == .asset_manifest.frame_count
  and .pixel_parity.manifest_frame_match_count == .asset_manifest.frame_count
  and .pixel_parity.sample_pixel_count == .pixel_parity.manifest_frame_pixel_count
  and .pixel_parity.sample_pixel_count >= 11000
  and .pixel_parity.sample_visible_pixel_count > 2500
  and .pixel_parity.sample_pixel_mismatch_count == 0
  and .pixel_parity.reference_render_pixel_mismatch_count == 0
  and .pixel_parity.role_family_count == .pixel_parity.manifest_role_family_count
  and .pixel_parity.role_family_count >= 16
  and (.pixel_parity.manifest_frame_ids | length) == .asset_manifest.frame_count
  and ([.pixel_parity.manifest_frame_ids[]] | index("tile_grass_a") != null)
  and ([.pixel_parity.manifest_frame_ids[]] | index("actor_player_idle_south") != null)
  and ([.pixel_parity.manifest_frame_ids[]] | index("actor_enemy_attack") != null)
  and ([.pixel_parity.manifest_frame_ids[]] | index("marker_interaction") != null)
  and (.pixel_parity.sample_reports | length) == .asset_manifest.frame_count
  and (.pixel_parity.sample_reports | all(.available == true and .sha_match == true and .pixel_mismatch_count == 0 and (.source_rgb_sha256 | length) == 64 and (.rust_port_rgb_sha256 | length) == 64))
  and .source_contract_gate == true
  and .source_green_gate == true
  and .engine_module_gate == true
  and .rules_mod_port_gate == true
  and .chrome_widget_port_gate == true
  and .asset_loader_port_gate == true
  and .pixel_perfect_asset_parity_gate == true
  and .write_gate == true
  and .no_copy_boundary_gate == true
  and .openra_engine_port_asset_parity_gate == true
  and .openra_engine_port_foundation_claimed == true
  and .openra_engine_port_claimed == true
  and .openra_full_engine_port_claimed == false
  and .openra_pixel_perfect_asset_parity_claimed == true
  and .openra_pixel_perfect_asset_parity_scope == "trillionnium_owned_openra_compatible_asset_pack"
  and .openra_westwood_pixel_perfect_asset_parity_claimed == false
  and .openra_asset_copied == false
  and .westwood_asset_copied == false
  and .warcraft_iii_asset_copied == false
  and .third_party_asset_copied == false
  and .openra_csharp_engine_code_copied == false
  and .bevy_openra_binary_replay_compatible == false
  and .bevy_openra_network_order_stream_claimed == false
  and .android_s5_real_device_claimed == false
  and .public_launch_ready == false
' "$SUMMARY" >/dev/null

test -s "$PREVIEW"
test -s "$PREVIEW_DIR/openra-engine-port-manifest.json"
test -s "$PREVIEW_DIR/openra-engine-port-asset-parity-reference.ppm"
test -s "$PREVIEW_DIR/openra-engine-port-asset-parity-rendered.ppm"
test -s "$PREVIEW_DIR/openra-engine-port-asset-parity-diff.ppm"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_ENGINE_PORT_ASSET_PARITY_GREEN %s %s\n' "$SUMMARY" "$PREVIEW"
