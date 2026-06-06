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

add_performance_budget_packet_fixtures() {
  local input_frame_budget_json="$TMP_DIR/bevy-classic-input-frame-budget.json"
  jq -n '{
    contract_version: "trillionnium_world_bevy_classic_input_frame_budget_v1",
    green: true,
    loaded_from_manifest: true,
    atlas_parse_gate: true,
    accepted_input_gate: true,
    direction_coverage_gate: true,
    rendered_frame_gate: true,
    selected_frame_manifest_gate: true,
    response_p95_budget_gate: true,
    response_max_budget_gate: true,
    sample_count: 96,
    accepted_input_count: 96,
    accepted_directions: ["west", "east", "north", "south"],
    input_path: "NativeControlAction::Move -> apply_live_native_action -> classic_draw_scene",
    renderer_path: "classic_cpu_ppm_minifb_low_spec",
    frame_width: 640,
    frame_height: 360,
    p50_micros: 6700,
    p95_micros: 9700,
    avg_micros: 7100,
    max_micros: 11200,
    p95_budget_micros: 20000,
    max_budget_micros: 50000,
    nonblank_samples: [104249, 104249, 104249, 104249],
    selected_frame_ids: [
      "actor_player_walk_north_1",
      "actor_player_idle_west",
      "actor_player_walk_east_2",
      "actor_player_walk_south_1"
    ],
    samples: [
      {accepted: true, direction: "north", elapsed_micros: 7100, last_action: "local_move:north", last_result: "local_map_step_before_training", nonblank_pixels: 104249, selected_frame_id: "actor_player_walk_north_1"},
      {accepted: true, direction: "east", elapsed_micros: 7200, last_action: "local_move:east", last_result: "local_map_step_before_training", nonblank_pixels: 104249, selected_frame_id: "actor_player_walk_east_2"},
      {accepted: true, direction: "south", elapsed_micros: 7300, last_action: "local_move:south", last_result: "local_map_step_before_training", nonblank_pixels: 104249, selected_frame_id: "actor_player_walk_south_1"},
      {accepted: true, direction: "west", elapsed_micros: 7400, last_action: "local_move:west", last_result: "local_map_step_before_training", nonblank_pixels: 104249, selected_frame_id: "actor_player_idle_west"}
    ],
    source_of_truth: "Classic input-frame budget measures accepted movement input through apply_live_native_action plus the next low-spec classic_draw_scene frame, protecting keyboard responsiveness on the Bevy client path.",
    cex_runtime_player_client_allowed: false,
    wgpu_required: false
  }' >"$input_frame_budget_json"
  add_artifact_from_path native_bevy_classic_input_frame_budget "Native/Bevy classic input frame budget" "$input_frame_budget_json" release_review_input

  local render_budget_json="$TMP_DIR/bevy-classic-render-budget.json"
  jq -n '{
    contract_version: "trillionnium_world_bevy_classic_render_budget_v1",
    green: true,
    loaded_from_manifest: true,
    atlas_parse_gate: true,
    frame_count_gate: true,
    p95_budget_gate: true,
    max_budget_gate: true,
    nonblank_gate: true,
    renderer_path: "classic_cpu_ppm_minifb_low_spec",
    frame_width: 640,
    frame_height: 360,
    frame_count: 180,
    p50_micros: 6400,
    p95_micros: 7700,
    avg_micros: 6500,
    max_micros: 10100,
    p95_budget_micros: 16000,
    max_budget_micros: 40000,
    nonblank_samples: [104232, 104252, 96505, 98087],
    source_of_truth: "Classic render budget measures repeated low-spec classic_draw_scene CPU frames without Bevy/wgpu, protecting the X230 playtest path from renderer regressions.",
    cex_runtime_player_client_allowed: false,
    wgpu_required: false
  }' >"$render_budget_json"
  add_artifact_from_path native_bevy_classic_render_budget "Native/Bevy classic render budget" "$render_budget_json" release_review_input
}

add_playtest_runner_packet_fixtures() {
  local playtest_runner_json="$TMP_DIR/bevy-classic-playtest-runner-status.json"
  jq -n \
    --arg root "$ROOT" \
    '{
      contract_version: "trillionnium_world_bevy_classic_playtest_runner_status_v1",
      status: "green",
      green: true,
      service: {
        unit: "trillionnium-bevy-playtest.service",
        active_state: "active",
        sub_state: "running",
        main_pid: 160672,
        exec_main_status: "0"
      },
      runtime: {
        expected_binary: ($root + "/target/release/trnm-world-bevy"),
        expected_repo_root: $root,
        expected_cwd: ($root + "/trillionnium"),
        process_cwd: ($root + "/trillionnium"),
        expected_manifest: ($root + "/assets/trnm-world/classic/manifest.json"),
        expected_override_dir: ($root + "/assets/trnm-world/classic/art-pack-v1"),
        manifest_sha256: "c628e35e2e44883be53d1de0d99a3cacb88d59bf02aabb3f1e24f165af5ede1f",
        cmdline: [
          ($root + "/target/release/trnm-world-bevy"),
          "run"
        ],
        selected_environment: {
          TRNM_WORLD_BEVY_LOW_SPEC: "1",
          TRNM_WORLD_BEVY_CLASSIC_RENDERER: "1",
          TRNM_WORLD_BEVY_CLASSIC_FPS: "30",
          TRNM_WORLD_BEVY_CLASSIC_ASSET_MANIFEST: ($root + "/assets/trnm-world/classic/manifest.json"),
          TRNM_WORLD_BEVY_CLASSIC_ASSET_OVERRIDE_DIR: ($root + "/assets/trnm-world/classic/art-pack-v1")
        }
      },
      gates: {
        service_process_gate: true,
        release_binary_gate: true,
        classic_env_gate: true,
        manifest_gate: true,
        override_dir_gate: true,
        workdir_gate: true,
        cex_path_gate: true
      },
      source_of_truth: "The live playtest runner must be the release trnm-world-bevy binary with the low-spec classic renderer manifest; CEX paths are explicitly rejected."
  }' >"$playtest_runner_json"
  add_artifact_from_path native_bevy_classic_playtest_runner_status "Native/Bevy classic playtest runner status" "$playtest_runner_json" release_review_input
}

add_classic_playtest_launcher_packet_fixtures() {
  local playtest_launcher_json="$TMP_DIR/bevy-classic-playtest-launcher.json"
  jq -n \
    --arg root "$ROOT" \
    '{
      contract_version: "trillionnium_world_bevy_classic_playtest_launcher_v1",
      campaign_entry_contract: "trillionnium_world_bevy_classic_rts_campaign_entry_v1",
      runner_status_contract: "trillionnium_world_bevy_classic_playtest_runner_status_v1",
      title_menu_contract: "trillionnium_world_bevy_title_menu_v1",
      state_snapshot_contract: "trillionnium_world_bevy_state_snapshot_v1",
      status: "green",
      green: true,
      player_entry: {
        title_actions: ["CAMPAIGN:START", "CAMPAIGN:CONTINUE", "CAMPAIGN:REPLAY"],
        primary_start_action: "CAMPAIGN:START",
        resume_action: "CAMPAIGN:CONTINUE",
        replay_action: "CAMPAIGN:REPLAY",
        followup_action_after_resume: "CONTINUE:SESSION",
        input_path: "apply_live_native_action_with_source(classic_rts_campaign_entry_title_input)",
        input_action_count: 73,
        start_input_count: 73,
        replay_input_count: 73,
        campaign_slot_path: "target/trnm-world-bevy-session-slots/bevy-classic-rts-campaign-entry.snapshot.json",
        campaign_slot_bytes: 71913,
        final_current_room_id: "league-coliseum",
        final_map_scene: "arena_outdoor",
        final_open_world_handoff_state: "resumed:league-coliseum",
        final_contextual_primary_action_label: "COMBAT:attack"
      },
      live_runner: {
        service: {
          unit: "trillionnium-bevy-playtest.service",
          active_state: "active",
          sub_state: "running",
          main_pid: 160672,
          exec_main_status: "0"
        },
        runtime: {
          expected_binary: ($root + "/target/release/trnm-world-bevy"),
          expected_repo_root: $root,
          expected_cwd: ($root + "/trillionnium"),
          process_cwd: ($root + "/trillionnium"),
          expected_manifest: ($root + "/assets/trnm-world/classic/manifest.json"),
          expected_override_dir: ($root + "/assets/trnm-world/classic/art-pack-v1"),
          manifest_sha256: "c628e35e2e44883be53d1de0d99a3cacb88d59bf02aabb3f1e24f165af5ede1f",
          cmdline: [($root + "/target/release/trnm-world-bevy"), "run"],
          selected_environment: {
            TRNM_WORLD_BEVY_LOW_SPEC: "1",
            TRNM_WORLD_BEVY_CLASSIC_RENDERER: "1",
            TRNM_WORLD_BEVY_CLASSIC_FPS: "30",
            TRNM_WORLD_BEVY_CLASSIC_ASSET_MANIFEST: ($root + "/assets/trnm-world/classic/manifest.json"),
            TRNM_WORLD_BEVY_CLASSIC_ASSET_OVERRIDE_DIR: ($root + "/assets/trnm-world/classic/art-pack-v1")
          }
        }
      },
      gates: {
        campaign_entry_gate: true,
        runner_status_gate: true,
        title_campaign_start_action_gate: true,
        title_campaign_continue_action_gate: true,
        title_campaign_replay_action_gate: true,
        campaign_start_gate: true,
        campaign_continue_gate: true,
        campaign_continue_unlock_gate: true,
        campaign_replay_gate: true,
        campaign_slot_gate: true,
        open_world_resume_gate: true,
        player_command_gate: true,
        service_process_gate: true,
        release_binary_gate: true,
        classic_env_gate: true,
        manifest_gate: true,
        override_dir_gate: true,
        workdir_gate: true,
        cex_path_gate: true,
        player_launch_ready_gate: true
      },
      android_s5_real_device_claimed: false,
      source_of_truth: "A player-ready classic playtest launcher must expose CAMPAIGN title actions, persist and restore the campaign slot, resume into the Bevy-owned open-world state, and run on the live release trnm-world-bevy service with no CEX runtime path."
    }' >"$playtest_launcher_json"
  add_artifact_from_path native_bevy_classic_playtest_launcher "Native/Bevy classic playtest launcher" "$playtest_launcher_json" release_review_input
}

add_campaign_ui_continuity_packet_fixtures() {
  local campaign_ui_continuity_json="$TMP_DIR/bevy-classic-rts-campaign-ui-continuity.json"
  jq -n '{
    contract_version: "trillionnium_world_bevy_classic_rts_campaign_ui_continuity_v1",
    green: true,
    campaign_handoff_contract: "trillionnium_world_bevy_classic_rts_campaign_handoff_v1",
    campaign_handoff_green: true,
    preview_width: 1920,
    preview_height: 1080,
    preview_format: "ppm_p3_rgb",
    capture_frame_count: 16,
    final_current_room_id: "league-coliseum",
    final_map_scene: "arena_outdoor",
    final_route_director_task_id: "task-fixture-first-route",
    final_route_director_next_room_id: null,
    final_open_world_handoff_state: "resumed:league-coliseum",
    final_contextual_primary_action_label: "COMBAT:attack",
    final_contextual_action_labels: ["TITLE:OPEN", "ACCOUNT:REGISTER", "ACCOUNT:LOGIN", "ACCOUNT:CONTINUE", "ROOM:mirror-city-square", "ROOM:delivery-dock", "NPC:enemy-market-bandit", "COMBAT:attack", "COMBAT:defend", "COMBAT:potion", "COMBAT:escape", "BAG:open", "STAT:force", "STAT:agility", "STAT:craft", "SAVE:SLOT", "SLOT:A", "SAVE:A", "SLOT:B", "SAVE:B", "SLOT:C", "SAVE:C", "SAVE:SELECTED", "PAUSE:MENU"],
    final_active_task_ids: ["task-fixture-first-route"],
    final_objective_status: "open_world_after_action_ready",
    restored_current_room_id: "league-coliseum",
    restored_map_scene: "arena_outdoor",
    restored_open_world_handoff_state: "resumed:league-coliseum",
    restored_route_director_task_id: "task-fixture-first-route",
    restored_route_director_next_room_id: null,
    restored_contextual_action_labels: ["TITLE:OPEN", "ACCOUNT:REGISTER", "ACCOUNT:LOGIN", "ACCOUNT:CONTINUE", "ROOM:mirror-city-square", "ROOM:delivery-dock", "NPC:enemy-market-bandit", "COMBAT:attack", "COMBAT:defend", "COMBAT:potion", "COMBAT:escape", "BAG:open", "STAT:force", "STAT:agility", "STAT:craft", "SAVE:SLOT", "SLOT:A", "SAVE:A", "SLOT:B", "SAVE:B", "SLOT:C", "SAVE:C", "SAVE:SELECTED", "PAUSE:MENU"],
    restored_active_task_ids: ["task-fixture-first-route"],
    milestones: {
      aftermath_seen: true,
      army_rally_seen: true,
      base_assault_seen: true,
      breach_seen: true,
      commander_seen: true,
      creep_camp_seen: true,
      enemy_pressure_seen: true,
      expansion_seen: true,
      inner_seen: true,
      keep_pressure_seen: true,
      keep_victory_seen: true,
      objective_victory_seen: true,
      open_world_seen: true,
      recon_seen: true,
      restoration_seen: true,
      tier_two_seen: true
    },
    non_background_pixels: 2073600,
    victory_pixel_count: 1702,
    expansion_pixel_count: 20311,
    breach_pixel_count: 4574,
    keep_pixel_count: 729,
    restoration_pixel_count: 1593,
    open_world_pixel_count: 1088,
    handoff_green_gate: true,
    preview_resolution_gate: true,
    live_input_gate: true,
    milestone_gate: true,
    map_ui_state_gate: true,
    restored_ui_state_gate: true,
    persistence_gate: true,
    render_readability_gate: true,
    native_client_boundary_gate: true,
    android_s5_real_device_claimed: false,
    public_launch_ready: false,
    screen_for_screen_openra_ui_claimed: false,
    openra_engine_port_claimed: false,
    source_of_truth: "Classic RTS campaign UI continuity evidence binds the Bevy-owned campaign handoff preview to final and restored map scene, route director, objective panel, contextual action labels, milestone pixels, and native-client boundary gates so the RTS-to-open-world map/UI handoff cannot regress silently."
  }' >"$campaign_ui_continuity_json"
  add_artifact_from_path native_bevy_classic_rts_campaign_ui_continuity "Native/Bevy classic RTS campaign UI continuity" "$campaign_ui_continuity_json" release_review_input

  local campaign_ui_continuity_ppm="$TMP_DIR/bevy-classic-rts-campaign-ui-continuity.ppm"
  printf 'P3\n1920 1080\n255\n' >"$campaign_ui_continuity_ppm"
  truncate -s 20000001 "$campaign_ui_continuity_ppm"
  add_artifact_from_path native_bevy_classic_rts_campaign_ui_continuity_ppm "Native/Bevy classic RTS campaign UI continuity PPM" "$campaign_ui_continuity_ppm" release_review_visual_evidence
}

add_map_modeling_packet_fixtures() {
  local map_modeling_json="$TMP_DIR/map-modeling-gate.json"
  jq -n '{
    contract_version: "trillionnium_world_map_modeling_gate_v1",
    status: "fixture_map_modeling_gate_green_with_public_data_blockers",
    fixture_only: true,
    provider_mode: "fixture",
    source_of_truth: "trnm_world_map_provider_fixture_modeling",
    live_ingestion_enabled: false,
    runtime_clients_fetch_public_osm_directly: false,
    public_network_ready: false,
    layer_counts: {
      buildings: 24,
      roads: 62,
      greenery: 8,
      terrain: 4
    },
    gates: {
      building_modeling_gate: true,
      road_modeling_gate: true,
      greenery_modeling_gate: true,
      terrain_modeling_gate: true,
      no_live_ingestion_gate: true,
      all_layers_modeled: true
    },
    modeling_layers: {
      buildings: [
        {
          asset_class: "building_mass_from_map_pack_node",
          model_id: "building:mirror-city-square",
          source_node_id: "mirror-city-square",
          node_kind: "hub_square",
          collision_role: "walkable_boundary_and_occlusion_hint",
          roof_profile: "low_poly_city_roof",
          footprint: {contract: "fixture_grid_footprint_from_world_node_lat_lng_e7", half_extent_tiles: 2},
          gameplay_anchor_tags: ["talk", "notice_board", "hub_square"]
        },
        {
          asset_class: "building_mass_from_map_pack_node",
          model_id: "building:league-coliseum",
          source_node_id: "league-coliseum",
          node_kind: "arena_gate",
          collision_role: "walkable_boundary_and_occlusion_hint",
          roof_profile: "gatehouse_roof",
          footprint: {contract: "fixture_grid_footprint_from_world_node_lat_lng_e7", half_extent_tiles: 1},
          gameplay_anchor_tags: ["arena", "ranking", "arena_gate"]
        },
        {
          asset_class: "building_mass_from_map_pack_node",
          model_id: "building:survey-tower",
          source_node_id: "survey-tower",
          node_kind: "survey_tower",
          collision_role: "walkable_boundary_and_occlusion_hint",
          roof_profile: "watch_tower_roof",
          footprint: {contract: "fixture_grid_footprint_from_world_node_lat_lng_e7", half_extent_tiles: 1},
          gameplay_anchor_tags: ["terrain", "survey", "blocked_path"]
        }
      ],
      roads: [
        {
          asset_class: "road_path_from_map_pack_edge",
          model_id: "road:001:mirror-city-square:agent-dormitory",
          navigation_role: "walkable_route_graph",
          road_class: "street_lane",
          material_hint: "packed_earth_street",
          path_polyline: [{x: 0, y: 0}, {x: -1, y: 0}],
          source_edge: {from: "mirror-city-square", to: "agent-dormitory", direction: "west"}
        },
        {
          asset_class: "road_path_from_map_pack_edge",
          model_id: "road:053:survey-tower:guild-vault",
          navigation_role: "walkable_route_graph",
          road_class: "path_lane",
          material_hint: "stone_path",
          path_polyline: [{x: 2, y: 5}, {x: 2, y: -1}],
          source_edge: {from: "survey-tower", to: "guild-vault", direction: "south"}
        }
      ],
      greenery: [
        {
          asset_class: "greenery_cluster_from_map_pack_tags",
          model_id: "greenery:mirror-city-planter",
          source_tag: "plaza_greenery",
          foliage_role: "readability_breakup"
        }
      ],
      terrain: [
        {
          zone_id: "terrain:mirror-city-paved-lowland",
          terrain_kind: "paved_urban_plaza",
          mesh_role: "ground_surface",
          elevation_band: "lowland",
          walkability: "high"
        },
        {
          zone_id: "terrain:river-cistern-wetland",
          terrain_kind: "water_edge_buffer",
          mesh_role: "water_and_bank_surface",
          elevation_band: "lowland_water",
          walkability: "partial"
        },
        {
          zone_id: "terrain:survey-tower-ridge",
          terrain_kind: "survey_ridge_blocked_path",
          mesh_role: "height_hint_and_blocked_path_surface",
          elevation_band: "raised_ridge",
          walkability: "gated"
        }
      ]
    },
    modeling_policy: {
      building_source: "map_pack_nodes_to_low_poly_footprints",
      road_source: "map_pack_edges_to_walkable_route_graph",
      greenery_source: "map_pack_tags_to_foliage_clusters",
      terrain_source: "authored_zone_meshes_bound_to_map_pack_node_groups",
      renderer_authority: "native_bevy_visualization_only_world_state_remains_rust_authoritative",
      production_data_rule: "real map modeling credit must consume signed production map_pack artifacts, not direct runtime Overpass or Geofabrik calls"
    },
    public_network_blocking_reason: "building/road/greenery/terrain modeling is proven on deterministic fixture map_pack only; production credit still requires approved real map-pack source, cache policy, attribution screenshots, sensitive POI/geofence review, and operator signoff",
    required_next_evidence: [
      "approved_production_map_source",
      "signed_production_map_pack_manifest",
      "building_footprint_derivation_report",
      "road_graph_derivation_report",
      "greenery_landuse_derivation_report",
      "terrain_mesh_derivation_report",
      "visible_attribution_screenshots",
      "sensitive_poi_and_geofence_review",
      "operator_signoff"
    ]
  }' >"$map_modeling_json"
  add_artifact_from_path map_modeling_gate "Map modeling gate" "$map_modeling_json" release_review_input
}

add_public_launch_blocker_consistency_packet_fixtures() {
  local blocker_consistency_json="$TMP_DIR/public-launch-blocker-consistency.json"
  jq -n '{
    contract_version: "trillionnium_world_public_launch_blocker_consistency_v1",
    status: "public_launch_blocker_consistency_green_with_public_launch_blockers",
    source_of_truth: "trillionnium_world_public_launch_blocker_consistency",
    public_launch_ready: false,
    public_launch_claimed: false,
    consistency_rule: "public_launch_readiness_blockers_must_match_evidence_intake_items_and_field_level_validator_statuses",
    readiness: {
      summary_path: "/fixture/public-launch-readiness.json",
      refresh_log_path: "/fixture/public-launch-blocker-consistency-readiness.log"
    },
    intake: {
      summary_path: "/fixture/public-launch-evidence-intake.json",
      refresh_log_path: "/fixture/public-launch-blocker-consistency-intake.log"
    },
    known_blockers: [
      "s5_real_device_matrix",
      "production_map_pack_public_evidence",
      "first_beta_cohort_evidence",
      "commercial_launch_drill_evidence",
      "multi_node_or_live_traffic_latency_evidence",
      "public_network_live_exposure_evidence"
    ],
    unknown_readiness_blockers: [],
    unknown_intake_blockers: [],
    checks: [
      {name: "readiness_summary_present", status: "ok", actual: "/fixture/public-launch-readiness.json"},
      {name: "intake_summary_present", status: "ok", actual: "/fixture/public-launch-evidence-intake.json"},
      {name: "unknown_readiness_blockers", status: "ok", actual: null},
      {name: "unknown_intake_blockers", status: "ok", actual: null},
      {name: "s5_real_device_matrix_validator_present", status: "ok", actual: "/fixture/s5-real-device-evidence-validation.json"},
      {name: "s5_real_device_matrix_blocked_consistency", status: "ok", expected: "s5_real_device_evidence_green", actual: "blocked_missing_s5_real_device_evidence"},
      {name: "production_map_pack_public_evidence_validator_present", status: "ok", actual: "/fixture/production-map-pack-public-evidence.json"},
      {name: "production_map_pack_public_evidence_blocked_consistency", status: "ok", expected: "production_map_pack_public_ready_green", actual: "blocked_missing_production_map_pack_public_evidence"},
      {name: "first_beta_cohort_evidence_validator_present", status: "ok", actual: "/fixture/cohort-commercial-evidence.json"},
      {name: "first_beta_cohort_evidence_blocked_consistency", status: "ok", expected: "first_beta_cohort_evidence_green", actual: "blocked_missing_first_beta_cohort_evidence"},
      {name: "commercial_launch_drill_evidence_validator_present", status: "ok", actual: "/fixture/cohort-commercial-evidence.json"},
      {name: "commercial_launch_drill_evidence_blocked_consistency", status: "ok", expected: "commercial_launch_drill_evidence_green", actual: "blocked_missing_commercial_launch_drill_evidence"},
      {name: "multi_node_or_live_traffic_latency_evidence_validator_present", status: "ok", actual: "/fixture/external-ops-evidence.json"},
      {name: "multi_node_or_live_traffic_latency_evidence_blocked_consistency", status: "ok", expected: "multi_node_or_live_traffic_latency_green", actual: "blocked_missing_multi_node_or_live_traffic_latency_evidence"},
      {name: "public_network_live_exposure_evidence_validator_present", status: "ok", actual: "/fixture/external-ops-evidence.json"},
      {name: "public_network_live_exposure_evidence_blocked_consistency", status: "ok", expected: "public_network_deploy_green", actual: "blocked_missing_public_network_live_exposure_evidence"}
    ],
    failures: []
  }' >"$blocker_consistency_json"
  add_artifact_from_path public_launch_blocker_consistency "Public launch blocker consistency" "$blocker_consistency_json" release_review_gate
}
