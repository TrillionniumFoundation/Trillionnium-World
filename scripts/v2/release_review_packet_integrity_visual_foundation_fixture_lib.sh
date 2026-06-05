#!/usr/bin/env bash

add_visual_foundation_packet_fixtures() {
  local scene_preview_json="$TMP_DIR/bevy-classic-scene-preview.json"
  jq -n '{
    contract_version: "trillionnium_world_bevy_classic_scene_preview_v1",
    status: "classic_scene_preview_green",
    green: true,
    preview_format: "ppm_p3_rgb",
    preview_width: 1280,
    preview_height: 720,
    preview_bytes: 8704298,
    panel_count: 4,
    frame_count: 43,
    non_background_pixels: 560390,
    unique_color_count: 628,
    overlay_text_pixel_count: 14596,
    overlay_accent_text_pixel_count: 4348,
    overlay_panel_pixel_count: 16988,
    loaded_from_manifest: true,
    atlas_parse_gate: true,
    renderer_manifest_gate: true,
    direction_frame_gate: true,
    dynamic_landmark_animation_gate: true,
    preview_nonblank_gate: true,
    overlay_text_gate: true,
    source_of_truth: "The classic low-spec renderer draws deterministic scene preview panels through the same manifest, atlas, transparency, and directional player frame selection path used by the native playtest window.",
    dynamic_landmark_frame_ids: ["actor_enemy_attack", "prop_market_stall", "prop_arena_gate", "prop_reward", "actor_mentor_talk", "prop_workbench", "marker_objective", "prop_banner", "actor_mentor_idle", "prop_training_dummy", "prop_door", "prop_signpost"],
    panel_summaries: [
      {id: "south_talk_square", direction: "south", scene_id: "mirror_city_square", player_frame_id: "actor_player_idle_south", walk_cycle_frame: 0, non_background_pixels: 143711, unique_color_count: 376, dynamic_landmark_frame_ids: ["actor_mentor_talk", "prop_market_stall", "prop_signpost", "marker_objective"]},
      {id: "north_walk_training", direction: "north", scene_id: "mentor_training_room", player_frame_id: "actor_player_walk_north_1", walk_cycle_frame: 1, non_background_pixels: 137142, unique_color_count: 305, dynamic_landmark_frame_ids: ["prop_training_dummy", "prop_workbench", "prop_door"]},
      {id: "east_walk_arena", direction: "east", scene_id: "league_coliseum", player_frame_id: "actor_player_walk_east_1", walk_cycle_frame: 1, non_background_pixels: 135826, unique_color_count: 380, dynamic_landmark_frame_ids: ["actor_enemy_attack", "prop_arena_gate", "prop_reward", "prop_banner"]},
      {id: "west_walk_square", direction: "west", scene_id: "mirror_city_square", player_frame_id: "actor_player_walk_west_2", walk_cycle_frame: 2, non_background_pixels: 143711, unique_color_count: 375, dynamic_landmark_frame_ids: ["actor_mentor_idle", "prop_market_stall", "prop_signpost", "marker_objective"]}
    ],
    cex_runtime_player_client_allowed: false,
    wgpu_required: false,
    android_s5_real_device_claimed: false,
    external_evidence_ignored_for_current_scene_preview_pass: true,
    public_launch_ready: false,
    production_ready_ui_claimed: false,
    screen_for_screen_openra_ui_claimed: false,
    openra_engine_port_claimed: false,
    warcraft_iii_asset_copied: false,
    openra_asset_copied: false,
    third_party_asset_copied: false
  }' >"$scene_preview_json"
  add_artifact_from_path native_bevy_classic_scene_preview "Native/Bevy classic scene preview" "$scene_preview_json" release_review_input

  local scene_preview_ppm="$TMP_DIR/bevy-classic-scene-preview.ppm"
  printf 'P3\n1280 720\n255\n' >"$scene_preview_ppm"
  truncate -s 8000001 "$scene_preview_ppm"
  add_artifact_from_path native_bevy_classic_scene_preview_ppm "Native/Bevy classic scene preview PPM" "$scene_preview_ppm" release_review_visual_evidence

  local model_catalog_json="$TMP_DIR/bevy-classic-model-catalog.json"
  jq -n '{
    contract_version: "trillionnium_world_bevy_classic_model_catalog_v1",
    status: "classic_model_catalog_green",
    green: true,
    catalog_format: "ppm_p3_rgb",
    catalog_width: 640,
    catalog_height: 1056,
    catalog_bytes: 6213869,
    frame_count: 43,
    rendered_frame_count: 43,
    columns: 4,
    cell_width: 160,
    cell_height: 96,
    non_background_pixels: 121267,
    unique_color_count: 97,
    label_pixel_count: 13391,
    loaded_from_manifest: true,
    atlas_parse_gate: true,
    all_frames_rendered_gate: true,
    catalog_sheet_gate: true,
    role_coverage_gate: true,
    actor_clip_catalog_gate: true,
    player_direction_catalog_gate: true,
    scene_reference_catalog_gate: true,
    source_of_truth: "The classic model catalog renders every project-owned manifest frame through the same PPM atlas blitter used by the native low-spec playtest renderer.",
    role_counts: {player_actor: 13, enemy_actor: 4, npc_actor: 4, scene_prop: 8, terrain_tile: 2, objective_marker: 1, interaction_marker: 1},
    frame_summaries: [
      {id: "actor_player_walk_south_1", role: "player_actor", visible_pixel_count: 74},
      {id: "actor_player_walk_north_1", role: "player_actor", visible_pixel_count: 74},
      {id: "actor_player_walk_east_1", role: "player_actor", visible_pixel_count: 74},
      {id: "actor_player_walk_west_1", role: "player_actor", visible_pixel_count: 74},
      {id: "actor_mentor_talk", role: "npc_actor", visible_pixel_count: 77},
      {id: "actor_enemy_attack", role: "enemy_actor", visible_pixel_count: 74},
      {id: "marker_objective", role: "objective_marker", visible_pixel_count: 113},
      {id: "prop_reward", role: "scene_prop", visible_pixel_count: 72}
    ],
    cex_runtime_player_client_allowed: false,
    wgpu_required: false,
    android_s5_real_device_claimed: false,
    external_evidence_ignored_for_current_model_catalog_pass: true,
    public_launch_ready: false,
    production_ready_ui_claimed: false,
    screen_for_screen_openra_ui_claimed: false,
    openra_engine_port_claimed: false,
    warcraft_iii_asset_copied: false,
    openra_asset_copied: false,
    third_party_asset_copied: false
  }' >"$model_catalog_json"
  add_artifact_from_path native_bevy_classic_model_catalog "Native/Bevy classic model catalog" "$model_catalog_json" release_review_input

  local model_catalog_ppm="$TMP_DIR/bevy-classic-model-catalog.ppm"
  printf 'P3\n640 1056\n255\n' >"$model_catalog_ppm"
  truncate -s 5000001 "$model_catalog_ppm"
  add_artifact_from_path native_bevy_classic_model_catalog_ppm "Native/Bevy classic model catalog PPM" "$model_catalog_ppm" release_review_visual_evidence

  local renderer_probe_json="$TMP_DIR/bevy-classic-renderer-probe.json"
  jq -n '{
    contract_version: "trillionnium_world_bevy_classic_renderer_probe_v1",
    status: "classic_renderer_probe_green",
    green: true,
    frame_format: "ppm_p3_rgb",
    frame_width: 640,
    frame_height: 360,
    frame_bytes: 2179145,
    non_background_pixels: 96544,
    unique_color_count: 380,
    hud_panel_pixels: 4084,
    hud_text_pixels: 4867,
    player_frame_id: "actor_player_walk_east_1",
    loaded_from_manifest: true,
    atlas_parse_gate: true,
    frame_nonblank_gate: true,
    hud_probe_gate: true,
    player_frame_color_gate: true,
    scene_frame_gate: true,
    source_of_truth: "The release/debug CLI probe renders a real classic scene frame through the same low-spec renderer path without opening the minifb playtest window.",
    cex_runtime_player_client_allowed: false,
    wgpu_required: false,
    android_s5_real_device_claimed: false,
    external_evidence_ignored_for_current_renderer_probe_pass: true,
    public_launch_ready: false,
    production_ready_ui_claimed: false,
    screen_for_screen_openra_ui_claimed: false,
    openra_engine_port_claimed: false,
    warcraft_iii_asset_copied: false,
    openra_asset_copied: false,
    third_party_asset_copied: false
  }' >"$renderer_probe_json"
  add_artifact_from_path native_bevy_classic_renderer_probe "Native/Bevy classic renderer probe" "$renderer_probe_json" release_review_input

  local renderer_probe_ppm="$TMP_DIR/bevy-classic-renderer-probe.ppm"
  printf 'P3\n640 360\n255\n' >"$renderer_probe_ppm"
  truncate -s 1000001 "$renderer_probe_ppm"
  add_artifact_from_path native_bevy_classic_renderer_probe_ppm "Native/Bevy classic renderer probe PPM" "$renderer_probe_ppm" release_review_visual_evidence
}

add_modeling_foundation_packet_fixtures() {
  local asset_pack_json="$TMP_DIR/bevy-classic-asset-pack.json"
  jq -n '{
    contract_version: "trillionnium_world_bevy_classic_asset_pack_v1",
    green: true,
    loaded_from_manifest: true,
    renderer_uses_manifest: true,
    atlas_parse_gate: true,
    frame_gate: true,
    scene_gate: true,
    actor_gate: true,
    animation_clip_gate: true,
    directional_player_frame_gate: true,
    player_walk_clip_gate: true,
    mentor_talk_clip_gate: true,
    enemy_attack_clip_gate: true,
    scene_tile_gate: true,
    scene_landmark_gate: true,
    transparent_sprite_gate: true,
    opaque_tile_gate: true,
    procedural_sprite_shape_gate: true,
    frame_count: 43,
    actor_count: 3,
    scene_count: 3,
    atlas_width: 128,
    atlas_height: 96,
    atlas_bytes: 95697,
    manifest_bytes: 12713,
    atlas_format: "ppm_p3_rgb",
    source_tile_size_px: 16,
    render_tile_size_px: 32,
    asset_boundary: "project_owned_manifest_ppm_atlas_for_classic_low_spec_renderer_not_cex_runtime",
    source_of_truth: "Classic renderer reads project-owned manifest plus PPM atlas and keeps Rust world/input authority in trnm-world-bevy.",
    cex_runtime_player_client_allowed: false,
    wgpu_required: false,
    x230_low_spec_renderer_target: true
  }' >"$asset_pack_json"
  add_artifact_from_path native_bevy_classic_asset_pack "Native/Bevy classic asset pack" "$asset_pack_json" release_review_input

  local manifest_lint_json="$TMP_DIR/bevy-classic-manifest-lint.json"
  jq -n '{
    contract_version: "trillionnium_world_bevy_classic_manifest_lint_v1",
    green: true,
    frame_count: 43,
    actor_count: 3,
    scene_count: 3,
    role_counts: {
      arena_tile: 1,
      bridge_tile: 1,
      enemy_actor: 4,
      foliage_tile: 1,
      interaction_marker: 1,
      interior_tile: 1,
      npc_actor: 4,
      objective_marker: 1,
      player_actor: 13,
      road_tile: 1,
      roof_tile: 1,
      scene_prop: 8,
      shadow_tile: 1,
      stone_tile: 1,
      terrain_tile: 2,
      wall_tile: 1,
      water_tile: 1
    },
    duplicate_frame_ids: [],
    out_of_bounds_frame_ids: [],
    frame_overlap_detected: false,
    loaded_from_manifest: true,
    atlas_parse_gate: true,
    source_tile_size_gate: true,
    frame_rect_gate: true,
    frame_id_naming_gate: true,
    frame_role_alignment_gate: true,
    required_role_family_gate: true,
    player_direction_gate: true,
    mentor_enemy_clip_gate: true,
    scene_palette_gate: true,
    scene_shape_gate: true,
    scene_landmark_gate: true,
    catalog_ready_gate: true,
    boundary_gate: true,
    asset_boundary: "project_owned_manifest_ppm_atlas_for_classic_low_spec_renderer_not_cex_runtime",
    source_of_truth: "Classic manifest lint is the modeling production gate for extending project-owned low-spec sprite frames, scenes, actors, and clips inside trnm-world-bevy.",
    cex_runtime_player_client_allowed: false,
    wgpu_required: false,
    x230_low_spec_renderer_target: true
  }' >"$manifest_lint_json"
  add_artifact_from_path native_bevy_classic_manifest_lint "Native/Bevy classic manifest lint" "$manifest_lint_json" release_review_input

  local isometric_json="$TMP_DIR/bevy-classic-isometric-modeling.json"
  jq -n '{
    contract_version: "trillionnium_world_bevy_classic_isometric_modeling_v1",
    green: true,
    preview_format: "ppm_p3_rgb",
    preview_width: 640,
    preview_height: 360,
    preview_bytes: 2186885,
    non_background_pixels: 105346,
    unique_color_count: 378,
    projection: {
      mode: "orthographic_isometric_2_5d",
      tile_width: 48,
      tile_height: 24,
      samples: [
        {id: "origin", screen: {x: 320, y: 48}, tile: {x: 0, y: 0}},
        {id: "east", screen: {x: 344, y: 60}, tile: {x: 1, y: 0}},
        {id: "south", screen: {x: 296, y: 60}, tile: {x: 0, y: 1}},
        {id: "deep", screen: {x: 344, y: 156}, tile: {x: 5, y: 4}}
      ]
    },
    depth_order: [
      {depth_key: 41, frame_id: "model_town_hall", id: "town_hall"},
      {depth_key: 74, frame_id: "actor_mentor_talk", id: "mentor"},
      {depth_key: 95, frame_id: "actor_player_walk_south_1", id: "player"},
      {depth_key: 96, frame_id: "actor_guard_attack", id: "square_guard_front"},
      {depth_key: 136, frame_id: "actor_creep_attack", id: "square_creep_pressure"},
      {depth_key: 154, frame_id: "doodad_crystal_cluster", id: "square_crystal"}
    ],
    modeling_components: [
      "diamond_terrain_tiles",
      "orthographic_isometric_projection",
      "y_depth_sorted_sprite_entities",
      "actor_footprint_shadows",
      "sprite_anchor_bottom_center",
      "procedural_building_volumes",
      "tree_canopy_occlusion",
      "enlarged_actor_billboards",
      "multi_tile_rts_buildings",
      "warcraft_like_silhouette_set",
      "magic_gate_model",
      "terrain_road_overlay",
      "water_highlight_tiles",
      "raised_tile_cliff_faces",
      "rts_foundation_shadows",
      "rts_unit_selection_rings",
      "unit_health_bars",
      "player_enemy_mentor_silhouettes",
      "unit_depth_overlays",
      "rts_command_destination_marker",
      "combat_attack_arc",
      "combat_hit_flash",
      "rts_doodad_density",
      "procedural_rock_clusters",
      "torch_and_crystal_doodads",
      "doodad_depth_sorting",
      "biome_environment_overlays",
      "bridge_and_cliff_detail_tiles",
      "ruins_gold_vein_and_signpost_doodads",
      "neutral_guard_worker_creep_units"
    ],
    projection_gate: true,
    diamond_tile_gate: true,
    depth_sort_gate: true,
    sprite_anchor_gate: true,
    shadow_anchor_gate: true,
    procedural_volume_gate: true,
    rts_model_set_gate: true,
    terrain_detail_gate: true,
    environment_detail_gate: true,
    doodad_detail_gate: true,
    unit_detail_gate: true,
    neutral_unit_detail_gate: true,
    command_feedback_gate: true,
    rts_model_entity_count: 13,
    rts_environment_entity_count: 17,
    rts_doodad_entity_count: 20,
    rts_neutral_unit_entity_count: 15,
    shadow_pixel_count: 965,
    procedural_model_pixel_count: 11208,
    canopy_pixel_count: 4816,
    rts_building_pixel_count: 4929,
    terrain_detail_pixel_count: 19818,
    terrain_road_pixel_count: 3058,
    terrain_water_pixel_count: 1064,
    terrain_cliff_pixel_count: 2198,
    terrain_foundation_pixel_count: 2034,
    unit_detail_pixel_count: 2234,
    unit_ring_pixel_count: 530,
    unit_health_pixel_count: 196,
    unit_silhouette_pixel_count: 1548,
    neutral_unit_detail_pixel_count: 1323,
    neutral_guard_pixel_count: 288,
    neutral_worker_pixel_count: 294,
    neutral_creep_pixel_count: 590,
    command_feedback_pixel_count: 1657,
    command_marker_pixel_count: 1122,
    attack_arc_pixel_count: 345,
    hit_flash_pixel_count: 421,
    doodad_detail_pixel_count: 2986,
    doodad_stone_pixel_count: 516,
    doodad_wood_pixel_count: 276,
    doodad_fire_pixel_count: 98,
    doodad_crystal_pixel_count: 391,
    environment_detail_pixel_count: 5146,
    environment_foliage_pixel_count: 2598,
    environment_ruin_pixel_count: 92,
    environment_gold_pixel_count: 134,
    environment_bridge_pixel_count: 168,
    source_of_truth: "The classic renderer now uses a Warcraft-style 2.5D model: orthographic isometric diamond terrain, road/water/cliff/foundation details, doodad density props, bottom-center sprite anchors, footprint shadows, command feedback, and Y/depth sorted scene entities inside trnm-world-bevy.",
    cex_runtime_player_client_allowed: false,
    wgpu_required: false
  }' >"$isometric_json"
  add_artifact_from_path native_bevy_classic_isometric_modeling "Native/Bevy classic isometric modeling" "$isometric_json" release_review_input

  local isometric_ppm="$TMP_DIR/bevy-classic-isometric-modeling.ppm"
  printf 'P3\n640 360\n255\n' >"$isometric_ppm"
  truncate -s 2000001 "$isometric_ppm"
  add_artifact_from_path native_bevy_classic_isometric_modeling_ppm "Native/Bevy classic isometric modeling PPM" "$isometric_ppm" release_review_visual_evidence
}
