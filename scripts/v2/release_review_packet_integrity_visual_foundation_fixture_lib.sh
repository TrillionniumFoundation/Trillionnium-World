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

add_live_window_mouse_hit_test_packet_fixtures() {
  local mouse_hit_test_json="$TMP_DIR/bevy-live-window-mouse-hit-test-sequence.json"
  jq -n '{
    contract_version: "trillionnium_world_bevy_live_window_mouse_hit_test_sequence_v1",
    status: "live_window_mouse_hit_test_sequence_green",
    hit_test_map_contract: "trillionnium_world_bevy_visible_button_hit_test_map_v1",
    source_of_truth: "XTest mouse button events click Bevy-exposed client hit centers on the visible X11 window and xwd captures each post-action frame",
    green: true,
    display: ":0",
    window_id: "0xa00004",
    host_pid: 160672,
    slot_dir: "fixture-slots",
    slot_a_path: "fixture-slots/bevy-session-slot-a.snapshot.json",
    slot_a_bytes: 41520,
    hit_map_path: "fixture-hit-map.json",
    contact_sheet_path: "fixture-contact-sheet.png",
    contact_sheet_size: [560, 1228],
    contact_sheet_colors: 11972,
    contact_sheet_mean: [44.22, 49.38, 38.44],
    expected_frame_ids: ["title", "create", "talk", "train", "training_room", "arena", "fight_result", "save_continue", "title_continue", "resume_continue", "complete"],
    actual_frame_ids: ["title", "create", "talk", "train", "training_room", "arena", "fight_result", "save_continue", "title_continue", "resume_continue", "complete"],
    expected_action_labels: ["TITLE:NEW", "CREATE:CONFIRM", "TALK", "TRAIN", "MOVE:north", "FIGHT", "SAVE:SELECTED", "TITLE:OPEN", "TITLE:CONTINUE", "CONTINUE:SESSION"],
    actions: [
      {step_index: 1, step_id: "title_new", action_label: "TITLE:NEW", target_frame_id: "create", client_x: 390, client_y: 473, row_id: "title", source: "native_control_button"},
      {step_index: 2, step_id: "character_confirm", action_label: "CREATE:CONFIRM", target_frame_id: "talk", client_x: 470, client_y: 473, row_id: "character_create", source: "native_control_button"},
      {step_index: 3, step_id: "mentor_talk", action_label: "TALK", target_frame_id: "train", client_x: 332, client_y: 473, row_id: "core", source: "native_control_button"},
      {step_index: 4, step_id: "mentor_train", action_label: "TRAIN", target_frame_id: "training_room", client_x: 444, client_y: 473, row_id: "core", source: "native_control_button"},
      {step_index: 5, step_id: "move_north", action_label: "MOVE:north", target_frame_id: "arena", client_x: 137, client_y: 412, row_id: "movement", source: "text_adventure_key_button"},
      {step_index: 6, step_id: "fight", action_label: "FIGHT", target_frame_id: "fight_result", client_x: 556, client_y: 473, row_id: "core", source: "native_control_button"},
      {step_index: 7, step_id: "save_selected", action_label: "SAVE:SELECTED", target_frame_id: "save_continue", client_x: 513, client_y: 506, row_id: "selected_slot", source: "native_control_button"},
      {step_index: 8, step_id: "title_open", action_label: "TITLE:OPEN", target_frame_id: "title_continue", client_x: 316, client_y: 473, row_id: "title", source: "native_control_button"},
      {step_index: 9, step_id: "title_continue", action_label: "TITLE:CONTINUE", target_frame_id: "resume_continue", client_x: 474, client_y: 473, row_id: "title", source: "native_control_button"},
      {step_index: 10, step_id: "continue_session", action_label: "CONTINUE:SESSION", target_frame_id: "complete", client_x: 390, client_y: 473, row_id: "selected_slot", source: "native_control_button"}
    ],
    focus_event: {method: "XRaiseWindow+XSetInputFocus", window_id: "0xa00004"},
    mouse_events: [
      {action_label: "TITLE:NEW", step_id: "title_new", target_frame_id: "create", attempt: 1, relative: [390, 473], absolute: [390, 473], window_origin: [0, 0], window_size: [960, 540], source: "native_control_button", row_id: "title"},
      {action_label: "CREATE:CONFIRM", step_id: "character_confirm", target_frame_id: "talk", attempt: 1, relative: [470, 473], absolute: [470, 473], window_origin: [0, 0], window_size: [960, 540], source: "native_control_button", row_id: "character_create"},
      {action_label: "TALK", step_id: "mentor_talk", target_frame_id: "train", attempt: 1, relative: [332, 473], absolute: [332, 473], window_origin: [0, 0], window_size: [960, 540], source: "native_control_button", row_id: "core"},
      {action_label: "TRAIN", step_id: "mentor_train", target_frame_id: "training_room", attempt: 1, relative: [444, 473], absolute: [444, 473], window_origin: [0, 0], window_size: [960, 540], source: "native_control_button", row_id: "core"},
      {action_label: "MOVE:north", step_id: "move_north", target_frame_id: "arena", attempt: 1, relative: [137, 412], absolute: [137, 412], window_origin: [0, 0], window_size: [960, 540], source: "text_adventure_key_button", row_id: "movement"},
      {action_label: "FIGHT", step_id: "fight", target_frame_id: "fight_result", attempt: 1, relative: [556, 473], absolute: [556, 473], window_origin: [0, 0], window_size: [960, 540], source: "native_control_button", row_id: "core"},
      {action_label: "SAVE:SELECTED", step_id: "save_selected", target_frame_id: "save_continue", attempt: 1, relative: [513, 506], absolute: [513, 506], window_origin: [0, 0], window_size: [960, 540], source: "native_control_button", row_id: "selected_slot"},
      {action_label: "TITLE:OPEN", step_id: "title_open", target_frame_id: "title_continue", attempt: 1, relative: [316, 473], absolute: [316, 473], window_origin: [0, 0], window_size: [960, 540], source: "native_control_button", row_id: "title"},
      {action_label: "TITLE:CONTINUE", step_id: "title_continue", target_frame_id: "resume_continue", attempt: 1, relative: [474, 473], absolute: [474, 473], window_origin: [0, 0], window_size: [960, 540], source: "native_control_button", row_id: "title"},
      {action_label: "CONTINUE:SESSION", step_id: "continue_session", target_frame_id: "complete", attempt: 1, relative: [390, 473], absolute: [390, 473], window_origin: [0, 0], window_size: [960, 540], source: "native_control_button", row_id: "selected_slot"}
    ],
    frames: [
      {frame_index: 0, frame_id: "title", after_action: null, path: "00-title.png", size: [960, 540], mean: [53.63, 58.71, 43.68], colors_96x54: 3331, nonblank: true, diff_mean_from_previous: null, diff_bbox_from_previous: null},
      {frame_index: 1, frame_id: "create", after_action: "TITLE:NEW", path: "01-create.png", size: [960, 540], mean: [53.24, 58.12, 43.51], colors_96x54: 3279, nonblank: true, diff_mean_from_previous: 2.32, diff_bbox_from_previous: [281, 391, 591, 529]},
      {frame_index: 2, frame_id: "talk", after_action: "CREATE:CONFIRM", path: "02-talk.png", size: [960, 540], mean: [53.52, 58.43, 43.83], colors_96x54: 3318, nonblank: true, diff_mean_from_previous: 4.94, diff_bbox_from_previous: [55, 0, 927, 471]},
      {frame_index: 3, frame_id: "train", after_action: "TALK", path: "03-train.png", size: [960, 540], mean: [53.61, 58.53, 43.92], colors_96x54: 3330, nonblank: true, diff_mean_from_previous: 2.28, diff_bbox_from_previous: [59, 4, 929, 465]},
      {frame_index: 4, frame_id: "training_room", after_action: "TRAIN", path: "04-training_room.png", size: [960, 540], mean: [52.93, 58.53, 43.76], colors_96x54: 3328, nonblank: true, diff_mean_from_previous: 2.61, diff_bbox_from_previous: [69, 18, 855, 529]},
      {frame_index: 5, frame_id: "arena", after_action: "MOVE:north", path: "05-arena.png", size: [960, 540], mean: [53.1, 58.2, 43.7], colors_96x54: 3301, nonblank: true, diff_mean_from_previous: 1.7, diff_bbox_from_previous: [63, 12, 900, 529]},
      {frame_index: 6, frame_id: "fight_result", after_action: "FIGHT", path: "06-fight_result.png", size: [960, 540], mean: [53.9, 59.1, 44.1], colors_96x54: 3312, nonblank: true, diff_mean_from_previous: 3.2, diff_bbox_from_previous: [60, 0, 930, 529]},
      {frame_index: 7, frame_id: "save_continue", after_action: "SAVE:SELECTED", path: "07-save_continue.png", size: [960, 540], mean: [52.8, 58.0, 43.2], colors_96x54: 3290, nonblank: true, diff_mean_from_previous: 2.0, diff_bbox_from_previous: [80, 30, 890, 529]},
      {frame_index: 8, frame_id: "title_continue", after_action: "TITLE:OPEN", path: "08-title_continue.png", size: [960, 540], mean: [53.5, 58.5, 43.6], colors_96x54: 3320, nonblank: true, diff_mean_from_previous: 3.0, diff_bbox_from_previous: [50, 0, 930, 529]},
      {frame_index: 9, frame_id: "resume_continue", after_action: "TITLE:CONTINUE", path: "09-resume_continue.png", size: [960, 540], mean: [53.0, 58.4, 43.4], colors_96x54: 3307, nonblank: true, diff_mean_from_previous: 2.1, diff_bbox_from_previous: [70, 15, 910, 529]},
      {frame_index: 10, frame_id: "complete", after_action: "CONTINUE:SESSION", path: "10-complete.png", size: [960, 540], mean: [53.4, 58.7, 43.8], colors_96x54: 3333, nonblank: true, diff_mean_from_previous: 2.7, diff_bbox_from_previous: [65, 8, 920, 529]}
    ],
    hit_test_map_gate: true,
    host_window_gate: true,
    mouse_event_count_gate: true,
    frame_count_gate: true,
    frame_sequence_gate: true,
    screenshot_nonblank_gate: true,
    frame_change_gate: true,
    slot_write_gate: true,
    contact_sheet_gate: true,
    android_s5_real_device_claimed: false
  }' >"$mouse_hit_test_json"
  add_artifact_from_path native_bevy_live_window_mouse_hit_test_sequence "Native/Bevy live-window mouse hit-test sequence" "$mouse_hit_test_json" release_review_input
}

add_camera_minimap_sync_packet_fixtures() {
  local camera_minimap_sync_json="$TMP_DIR/bevy-classic-rts-camera-minimap-sync.json"
  jq -n '{
    contract_version: "trillionnium_world_bevy_classic_rts_camera_minimap_sync_v1",
    green: true,
    preview_format: "ppm_p3_rgb",
    preview_width: 1920,
    preview_height: 720,
    renderer_path: "classic_draw_scene+classic_draw_rts_camera_minimap_sync_overlay",
    input_path: "apply_rts_scrollable_map_camera_input(classic_rts_camera_minimap_sync_input)",
    runtime_path: "apply_rts_scrollable_map_camera_input+rts_camera_minimap_viewport_rect+rts_camera_minimap_revealed_tiles",
    selection_follow_path: "rts_camera_minimap_selection_follow_step",
    native_runtime_path: "update_native_rts_scrollable_map_camera+apply_native_rts_scrollable_map_view+rts_camera_minimap_viewport_rect",
    input_action_count: 6,
    stage_summaries: [
      {stage: "viewport_rect", selected_unit_id: "mirror_captain", control_group_id: "1", minimap_tile_id: null},
      {stage: "fog_reveal", selected_unit_id: "mirror_captain", control_group_id: "1", minimap_tile_id: null},
      {stage: "selection_follow", selected_unit_id: "mirror_captain", control_group_id: "1", minimap_tile_id: "mirror_captain"},
      {stage: "control_group_recall", selected_unit_id: "field_engineer", control_group_id: "2", minimap_tile_id: null},
      {stage: "route_projection", selected_unit_id: "signal_lancer", control_group_id: "2", minimap_tile_id: "minimap_route_target"},
      {stage: "zoom_sync", selected_unit_id: "mirror_captain", control_group_id: "1", minimap_tile_id: null}
    ],
    revealed_tile_union_count: 33,
    viewport_pixel_count: 5579,
    fog_pixel_count: 19452,
    reveal_pixel_count: 2491,
    selection_pixel_count: 7994,
    route_pixel_count: 1458,
    viewport_visual_gate: true,
    fog_visual_gate: true,
    reveal_visual_gate: true,
    selection_visual_gate: true,
    route_visual_gate: true,
    stage_gate: true,
    viewport_sync_gate: true,
    fog_reveal_gate: true,
    selection_follow_gate: true,
    control_group_sync_gate: true,
    route_projection_gate: true,
    zoom_rect_sync_gate: true,
    minimap_runtime_gate: true,
    scene_renderer_gate: true,
    original_art_policy_gate: true,
    warcraft_iii_asset_copied: false,
    cex_runtime_player_client_allowed: false,
    wgpu_required: false,
    android_s5_real_device_claimed: false,
    public_launch_ready: false
  }' >"$camera_minimap_sync_json"
  add_artifact_from_path native_bevy_classic_rts_camera_minimap_sync "Native/Bevy classic RTS camera/minimap sync" "$camera_minimap_sync_json" release_review_input

  local camera_minimap_sync_ppm="$TMP_DIR/bevy-classic-rts-camera-minimap-sync.ppm"
  printf 'P3\n1920 720\n255\n' >"$camera_minimap_sync_ppm"
  truncate -s 8000001 "$camera_minimap_sync_ppm"
  add_artifact_from_path native_bevy_classic_rts_camera_minimap_sync_ppm "Native/Bevy classic RTS camera/minimap sync PPM" "$camera_minimap_sync_ppm" release_review_visual_evidence
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

add_classic_playtest_handoff_packet_fixtures() {
  local playtest_handoff_readiness_json="$TMP_DIR/bevy-classic-playtest-handoff-readiness.json"
  jq -n \
    --arg root "$ROOT" \
    '{
      contract_version: "trillionnium_world_bevy_classic_playtest_handoff_readiness_v1",
      status: "classic_playtest_handoff_readiness_green",
      green: true,
      source_contracts: {
        playtest_readiness: "trillionnium_world_bevy_classic_playtest_readiness_v1",
        playtest_launcher: "trillionnium_world_bevy_classic_playtest_launcher_v1",
        playtest_runner_status: "trillionnium_world_bevy_classic_playtest_runner_status_v1",
        playtest_observability_readiness: "trillionnium_world_bevy_classic_rts_playtest_observability_readiness_v1"
      },
      handoff_summary: {
        playtest_readiness_green: true,
        launcher_green: true,
        runner_green: true,
        observability_green: true,
        runner_service: "trillionnium-bevy-playtest.service",
        runner_main_pid: 160672,
        runner_binary: ($root + "/target/release/trnm-world-bevy"),
        runner_process_cwd: ($root + "/trillionnium"),
        campaign_slot_bytes: 71913,
        title_actions: ["CAMPAIGN:START", "CAMPAIGN:CONTINUE", "CAMPAIGN:REPLAY"],
        resume_room_id: "league-coliseum",
        resume_map_scene: "arena_outdoor",
        resume_handoff_state: "resumed:league-coliseum",
        resume_primary_action: "COMBAT:attack",
        observability_preview_count: 4,
        replay_elapsed_seconds: 61,
        endurance_elapsed_seconds: 128,
        endurance_peak_active_units: 32
      },
      gates: {
        playtest_readiness_gate: true,
        launcher_gate: true,
        runner_gate: true,
        observability_gate: true,
        first_minute_gate: true,
        map_ui_modeling_gate: true,
        campaign_outcome_ui_gate: true,
        combat_readability_pressure_gate: true,
        playtest_observability_gate: true,
        client_boundary_gate: true,
        campaign_handoff_resume_gate: true,
        campaign_handoff_snapshot_gate: true,
        runner_service_process_gate: true,
        runner_release_binary_gate: true,
        runner_classic_env_gate: true,
        runner_manifest_gate: true,
        runner_override_dir_gate: true,
        runner_workdir_gate: true,
        runner_cex_path_gate: true,
        launcher_player_launch_ready_gate: true,
        launcher_campaign_slot_gate: true,
        launcher_open_world_resume_gate: true,
        launcher_player_command_gate: true,
        launcher_cex_path_gate: true,
        public_launch_not_claimed_gate: true,
        android_s5_real_device_not_claimed_gate: true
      },
      evidence_paths: {
        playtest_readiness: "acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-readiness.json",
        playtest_launcher: "acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-launcher.json",
        playtest_runner_status: "acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-runner-status.json",
        playtest_observability_readiness: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-playtest-observability-readiness.json"
      },
      android_s5_real_device_claimed: false,
      public_launch_ready_claimed: false,
      source_of_truth: "Classic playtest handoff readiness is the local human-playtest handoff layer for trnm-world-bevy. It requires the full Bevy classic playtest readiness chain, a live release runner, a campaign launcher that resumes into the Bevy-owned open-world RTS handoff, and observability evidence. It does not claim S5 real-device evidence, public launch readiness, or OpenRA natural replay/headless parity."
    }' >"$playtest_handoff_readiness_json"
  add_artifact_from_path native_bevy_classic_playtest_handoff_readiness "Native/Bevy classic playtest handoff readiness" "$playtest_handoff_readiness_json" release_review_input

  local playtest_handoff_packet_json="$TMP_DIR/bevy-classic-playtest-handoff-packet.json"
  jq -n \
    --argjson artifacts '[
      {"label":"playtest_handoff_readiness","path":"acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-handoff-readiness.json","sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","bytes":4096},
      {"label":"playtest_readiness","path":"acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-readiness.json","sha256":"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb","bytes":8192},
      {"label":"playtest_launcher","path":"acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-launcher.json","sha256":"cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc","bytes":4096},
      {"label":"playtest_runner_status","path":"acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-runner-status.json","sha256":"dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd","bytes":2048},
      {"label":"playtest_observability_readiness","path":"acceptance/S5_native_bevy_device/latest/bevy-classic-rts-playtest-observability-readiness.json","sha256":"eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee","bytes":4096}
    ]' \
    '{
      contract_version: "trillionnium_world_bevy_classic_playtest_handoff_packet_v1",
      status: "classic_playtest_handoff_packet_green",
      green: true,
      source_contracts: {
        playtest_handoff_readiness: "trillionnium_world_bevy_classic_playtest_handoff_readiness_v1",
        playtest_readiness: "trillionnium_world_bevy_classic_playtest_readiness_v1",
        playtest_launcher: "trillionnium_world_bevy_classic_playtest_launcher_v1",
        playtest_runner_status: "trillionnium_world_bevy_classic_playtest_runner_status_v1",
        playtest_observability_readiness: "trillionnium_world_bevy_classic_rts_playtest_observability_readiness_v1"
      },
      handoff_summary: {
        playtest_readiness_green: true,
        launcher_green: true,
        runner_green: true,
        observability_green: true,
        runner_service: "trillionnium-bevy-playtest.service",
        runner_main_pid: 160672,
        campaign_slot_bytes: 71913,
        title_actions: ["CAMPAIGN:START", "CAMPAIGN:CONTINUE", "CAMPAIGN:REPLAY"],
        resume_room_id: "league-coliseum",
        resume_map_scene: "arena_outdoor",
        resume_handoff_state: "resumed:league-coliseum",
        resume_primary_action: "COMBAT:attack",
        observability_preview_count: 4,
        replay_elapsed_seconds: 61,
        endurance_elapsed_seconds: 128,
        endurance_peak_active_units: 32
      },
      gates: {
        handoff_readiness_green: true,
        playtest_readiness_green: true,
        launcher_green: true,
        runner_green: true,
        observability_green: true,
        public_launch_not_claimed_gate: true,
        android_s5_real_device_not_claimed_gate: true,
        artifact_count_gate: true,
        artifact_sha_gate: true
      },
      run_commands: {
        refresh_handoff: "./scripts/check_trillionnium_world_bevy_classic_playtest_handoff_readiness.sh",
        refresh_packet: "./scripts/check_trillionnium_world_bevy_classic_playtest_handoff_packet.sh",
        inspect_runner: "systemctl --user status trillionnium-bevy-playtest.service",
        launch_client: "./scripts/run_trillionnium_world_bevy_client.sh"
      },
      artifact_manifest: $artifacts,
      no_credit_boundaries: {
        public_launch_ready_claimed: false,
        android_s5_real_device_claimed: false,
        openra_natural_replay_or_headless_parity_claimed: false
      },
      markdown_path: "acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-handoff-packet.md",
      source_of_truth: "Classic playtest handoff packet binds the local Bevy human-playtest handoff to checksummed evidence artifacts and replayable commands. It is a local host-side playtest packet only, not public launch, S5 real-device, or OpenRA natural replay/headless parity credit."
    }' >"$playtest_handoff_packet_json"
  add_artifact_from_path native_bevy_classic_playtest_handoff_packet "Native/Bevy classic playtest handoff packet" "$playtest_handoff_packet_json" release_review_input

  local playtest_handoff_packet_md="$TMP_DIR/bevy-classic-playtest-handoff-packet.md"
  {
    printf '# Bevy Classic Playtest Handoff Packet\n\n'
    printf -- '- Status: `true`\n'
    printf -- '- Contract: `trillionnium_world_bevy_classic_playtest_handoff_packet_v1`\n'
    printf -- '- Runner: `trillionnium-bevy-playtest.service` PID `160672`\n'
    printf -- '- Resume: `league-coliseum` / `arena_outdoor` / `resumed:league-coliseum`\n'
    printf -- '- Campaign slot bytes: `71913`\n'
    printf -- '- Title actions: `CAMPAIGN:START, CAMPAIGN:CONTINUE, CAMPAIGN:REPLAY`\n\n'
    printf '## Commands\n\n'
    printf -- '- `refresh_handoff`: `./scripts/check_trillionnium_world_bevy_classic_playtest_handoff_readiness.sh`\n'
    printf -- '- `refresh_packet`: `./scripts/check_trillionnium_world_bevy_classic_playtest_handoff_packet.sh`\n'
    printf -- '- `inspect_runner`: `systemctl --user status trillionnium-bevy-playtest.service`\n'
    printf -- '- `launch_client`: `./scripts/run_trillionnium_world_bevy_client.sh`\n\n'
    printf '## Evidence\n\n'
    printf -- '- `playtest_handoff_readiness`: `acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-handoff-readiness.json` sha256 `aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa` bytes `4096`\n'
    printf -- '- `playtest_readiness`: `acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-readiness.json` sha256 `bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb` bytes `8192`\n'
    printf -- '- `playtest_launcher`: `acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-launcher.json` sha256 `cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc` bytes `4096`\n'
    printf -- '- `playtest_runner_status`: `acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-runner-status.json` sha256 `dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd` bytes `2048`\n'
    printf -- '- `playtest_observability_readiness`: `acceptance/S5_native_bevy_device/latest/bevy-classic-rts-playtest-observability-readiness.json` sha256 `eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee` bytes `4096`\n\n'
    printf '## Boundaries\n\n'
    printf -- '- Public launch ready: `false`\n'
    printf -- '- Android S5 real device ready: `false`\n'
    printf -- '- OpenRA natural replay/headless parity: `false`\n'
  } >"$playtest_handoff_packet_md"
  add_artifact_from_path native_bevy_classic_playtest_handoff_packet_markdown "Native/Bevy classic playtest handoff packet Markdown" "$playtest_handoff_packet_md" release_review_input
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

add_cex_adapter_readiness_packet_fixtures() {
  local mode="${1:-valid}"
  local cex_adapter_json="$TMP_DIR/cex-production-adapter-readiness.json"
  local protocol_contract="trillionnium_world_runtime_adapter_v1"
  local route_record_total=7236
  local adapters_ok=true
  local counts_ok=true

  if [[ "$mode" == "semantic_invalid" ]]; then
    protocol_contract="cex_runtime_adapter_protocol_drift"
    route_record_total=0
    adapters_ok=false
    counts_ok=false
  fi

  jq -n \
    --arg protocol_contract "$protocol_contract" \
    --argjson route_record_total "$route_record_total" \
    --argjson adapters_ok "$adapters_ok" \
    --argjson counts_ok "$counts_ok" \
    '{
      contract_version: "trillionnium_world_cex_adapter_readiness_gate_v1",
      status: "cex_adapter_readiness_green",
      green: true,
      source_of_truth: "trillionnium_world_cex_adapter_readiness_gate",
      cex_import_rule: "trillionnium_world_crates_do_not_import_cex_service_internals; cex_runtime_exports_json_evidence_for_trillionnium_release_review",
      raw_evidence_path: "/fixture/cex-production-adapter-readiness.raw.json",
      input: {
        evidence_path: "",
        url: "",
        fetch_status: "cached_raw_evidence",
        fetch_detail: "/fixture/cex-production-adapter-readiness.raw.json"
      },
      observed: {
        contract_version: "cex_trillionnium_world_production_adapter_v1",
        protocol_contract: $protocol_contract,
        domain_contract: "trillionnium_world_domain_v1",
        status: "cex_production_adapter_bridge_ready",
        cutover_status: "cex_production_impls_connected_to_standalone_traits",
        cex_dependency_status: "consumer_entry_api_depends_on_trnm_world_api_without_trnm_world_importing_cex",
        status_count: 6,
        route_record_total: $route_record_total,
        world_node_count: 24,
        repository_source: "cex_league_repository_normalized_world_tables",
        ledger_reserve_source: "cex_world_contract_and_tactics_ledger_settlement",
        metric_source: "cex_consumer_entry_metrics_projection"
      },
      checks: {
        contract_ok: true,
        protocol_ok: ($protocol_contract == "trillionnium_world_runtime_adapter_v1"),
        domain_ok: true,
        status_ok: true,
        cutover_ok: true,
        dependency_ok: true,
        adapters_ok: $adapters_ok,
        roles_ok: true,
        repository_ok: true,
        ledger_ok: true,
        evidence_ok: true,
        metric_ok: true,
        counts_ok: $counts_ok
      }
    }' >"$cex_adapter_json"
  add_artifact_from_path cex_adapter_readiness "CEX production world adapter readiness" "$cex_adapter_json" release_review_input
}

add_release_signoff_summary_packet_fixtures() {
  local mode="${1:-valid}"
  local signoff_json="$TMP_DIR/release-signoff-summary.json"
  local render_asset_ready=true
  local render_asset_usage_gate=true
  local render_asset_reference_count=32
  local public_launch_consumes_render_asset=true
  local summary_blockers_json='[]'

  if [[ "$mode" == "semantic_invalid" ]]; then
    render_asset_ready=false
    render_asset_usage_gate=false
    render_asset_reference_count=3
    public_launch_consumes_render_asset=false
    summary_blockers_json='["native_bevy_render_asset_eligibility_contract"]'
  fi

  jq -n \
    --argjson render_asset_ready "$render_asset_ready" \
    --argjson render_asset_usage_gate "$render_asset_usage_gate" \
    --arg render_asset_reference_count "$render_asset_reference_count" \
    --argjson public_launch_consumes_render_asset "$public_launch_consumes_render_asset" \
    --argjson summary_blockers "$summary_blockers_json" \
    '{
      contract_version: "trillionnium_world_release_signoff_summary_v1",
      status: "release_signoff_summary_ready_with_public_launch_blockers",
      source_of_truth: "trillionnium_world_release_signoff_summary",
      signoff_rule: "native_bevy_keyboard_replay_action_coach_player_hud_live_screenshot_texture_sampling_correlation_render_asset_eligibility_and_cex_adapter_readiness_must_be_green_and_public_launch_readiness_must_consume_local_playability_before_release_review",
      public_launch_ready: false,
      android_s5_real_device_claimed: false,
      summary_blockers: $summary_blockers,
      public_launch_blockers: [
        "s5_real_device_matrix",
        "production_map_pack_public_evidence",
        "first_beta_cohort_evidence",
        "commercial_launch_drill_evidence",
        "multi_node_or_live_traffic_latency_evidence",
        "public_network_live_exposure_evidence"
      ],
      gates: {
        native_bevy_keyboard_replay: {
          evidence_path: "/fixture/bevy-build-branch-title-route-all-branch-keyboard-replay.json",
          file_status: "present",
          contract_version: "trillionnium_world_bevy_build_branch_title_route_all_branch_keyboard_replay_v1",
          green: true,
          branch_count: 3,
          ready_for_release_review: true,
          proof_scope: "host_side_bevy_runtime_replay_not_android_real_device",
          branches: {
            force: {recorded_sequence_count: 10, final_objective_status: "build_mastery_challenge_completed:force:task-force-mastery-guard-trial", combat_result_state: "victory"},
            agility: {recorded_sequence_count: 8, final_objective_status: "build_mastery_challenge_completed:agility:task-agility-mastery-shortcut-run"},
            craft: {recorded_sequence_count: 7, final_objective_status: "build_mastery_challenge_completed:craft:task-craft-mastery-client-order"}
          }
        },
        public_launch_consumes_replay: {
          evidence_path: "/fixture/public-launch-readiness.json",
          file_status: "present",
          public_launch_status: "blocked_missing_public_launch_evidence",
          ready: true
        },
        native_bevy_action_coach: {
          evidence_path: "/fixture/bevy-action-coach.json",
          file_status: "present",
          contract_version: "trillionnium_world_bevy_action_coach_v1",
          green: true,
          coach_stage_gate: true,
          enter_execution_gate: true,
          final_next_gate: true,
          ready_for_release_review: true,
          proof_scope: "host_side_bevy_runtime_guidance_not_android_real_device"
        },
        native_bevy_player_hud_debug_layer: {
          evidence_path: "/fixture/bevy-player-hud-debug-layer.json",
          file_status: "present",
          contract_version: "trillionnium_world_bevy_player_hud_debug_layer_v1",
          green: true,
          player_hud_gate: true,
          debug_layer_gate: true,
          ready_for_release_review: true,
          proof_scope: "host_side_bevy_hud_layer_not_android_real_device"
        },
        native_bevy_live_window_screenshot_sequence: {
          evidence_path: "/fixture/bevy-live-window-screenshot-sequence.json",
          file_status: "present",
          contract_version: "trillionnium_world_bevy_live_window_screenshot_sequence_v1",
          green: true,
          frame_sequence_gate: true,
          contact_sheet_gate: true,
          actual_frame_count: 11,
          ready_for_release_review: true,
          proof_scope: "host_side_live_window_screenshot_sequence_not_android_real_device"
        },
        native_bevy_sprite_texture_sampling: {
          evidence_path: "/fixture/bevy-sprite-texture-sampling.json",
          file_status: "present",
          contract_version: "trillionnium_world_bevy_sprite_texture_sampling_v1",
          green: true,
          four_layer_texture_sampling_gate: true,
          texture_sample_nonblank_gate: true,
          sampled_surface_count: 32,
          texture_unique_rgba_color_count: 10,
          ready_for_release_review: true,
          proof_scope: "host_side_cpu_texture_sampling_not_gpu_upload_or_android_real_device"
        },
        native_bevy_live_window_sampled_texture_correlation: {
          evidence_path: "/fixture/bevy-live-window-sampled-texture-correlation.json",
          file_status: "present",
          contract_version: "trillionnium_world_bevy_live_window_sampled_texture_correlation_v1",
          green: true,
          four_layer_sampled_live_correlation_gate: true,
          live_frame_count: 11,
          live_final_frame_colors_96x54: 3376,
          ready_for_release_review: true,
          proof_scope: "host_side_sampled_texture_to_live_window_correlation_not_android_real_device"
        },
        native_bevy_render_asset_eligibility: {
          evidence_path: "/fixture/bevy-render-asset-eligibility.json",
          file_status: "present",
          contract_version: "trillionnium_world_bevy_render_asset_eligibility_v1",
          green: $render_asset_ready,
          render_asset_usage_gate: $render_asset_usage_gate,
          image_descriptor_render_eligibility_gate: true,
          atlas_layout_render_eligibility_gate: true,
          sprite_render_reference_gate: true,
          image_asset_usage_debug: "RenderAssetUsages(MAIN_WORLD | RENDER_WORLD)",
          sprite_render_reference_count: ($render_asset_reference_count | tonumber),
          ready_for_release_review: $render_asset_ready,
          proof_scope: "host_side_render_asset_eligibility_not_render_world_extraction_or_gpu_upload"
        },
        cex_adapter_readiness: {
          evidence_path: "/fixture/cex-production-adapter-readiness.json",
          file_status: "present",
          contract_version: "trillionnium_world_cex_adapter_readiness_gate_v1",
          green: true,
          status: "cex_adapter_readiness_green",
          source_contract_version: "cex_trillionnium_world_production_adapter_v1",
          protocol_contract: "trillionnium_world_runtime_adapter_v1",
          domain_contract: "trillionnium_world_domain_v1",
          route_record_total: 7236,
          world_node_count: 24,
          ready_for_release_review: true,
          proof_scope: "cex_incubator_runtime_adapter_json_evidence_not_public_launch_external_evidence"
        },
        public_launch_consumes_local_playability: {
          evidence_path: "/fixture/public-launch-readiness.json",
          file_status: "present",
          public_launch_status: "blocked_missing_public_launch_evidence",
          action_coach: true,
          player_hud_debug_layer: true,
          live_window_screenshot_sequence: true,
          sprite_texture_sampling: true,
          live_window_sampled_texture_correlation: true,
          render_asset_eligibility: $public_launch_consumes_render_asset,
          ready: $public_launch_consumes_render_asset
        },
        s5_real_device_matrix: {
          evidence_path: "/fixture/s5-device-evidence.json",
          file_status: "present",
          status: "blocked_no_connected_android_device",
          ready: false,
          required_before_public_launch_ready: true
        },
        release_latency: {
          evidence_path: "/fixture/release-latency-drill.json",
          file_status: "present",
          status: "local_release_latency_drill_green",
          ready: true,
          local_drill_is_not_multi_node_or_live_traffic: true
        },
        release_rollback_backup: {
          evidence_path: "/fixture/release-rollback-backup-drill.json",
          file_status: "present",
          status: "release_rollback_backup_drill_green",
          ready: true
        },
        public_deploy: {
          evidence_path: "/fixture/public-network-deploy-evidence.json",
          file_status: "present",
          status: "local_public_deploy_drill_green",
          ready: true,
          local_drill_is_not_public_network_exposure: true
        }
      },
      reviewer_shortlist: [
        "native_bevy_keyboard_replay",
        "native_bevy_action_coach",
        "native_bevy_player_hud_debug_layer",
        "native_bevy_live_window_screenshot_sequence",
        "native_bevy_sprite_texture_sampling",
        "native_bevy_live_window_sampled_texture_correlation",
        "native_bevy_render_asset_eligibility",
        "cex_adapter_readiness",
        "public_launch_consumes_replay",
        "public_launch_consumes_local_playability",
        "s5_real_device_matrix",
        "release_latency",
        "release_rollback_backup",
        "public_deploy"
      ]
    }' >"$signoff_json"
  add_artifact_from_path release_signoff_summary "Release signoff summary" "$signoff_json" release_review_input
}

add_release_review_quickcheck_packet_fixtures() {
  local mode="${1:-valid}"
  local quickcheck_json="$TMP_DIR/release-review-quickcheck.json"
  local render_asset_ready=true
  local consumes_local_playability=true
  local signoff_summary_blockers_json='[]'

  if [[ "$mode" == "semantic_invalid" ]]; then
    render_asset_ready=false
    consumes_local_playability=false
    signoff_summary_blockers_json='["native_bevy_render_asset_eligibility_contract"]'
  fi

  jq -n \
    --argjson render_asset_ready "$render_asset_ready" \
    --argjson consumes_local_playability "$consumes_local_playability" \
    --argjson signoff_summary_blockers "$signoff_summary_blockers_json" \
    '{
      contract_version: "trillionnium_world_release_review_quickcheck_v1",
      status: "release_review_quickcheck_green_with_public_launch_blockers",
      source_of_truth: "trillionnium_world_release_review_quickcheck",
      quickcheck_rule: "refresh_public_launch_readiness_then_release_signoff_summary_and_fail_only_when_native_bevy_local_playability_texture_sampling_render_asset_eligibility_cex_adapter_readiness_or_consumption_is_broken_unless_require_ready_is_set",
      ready_for_release_review: true,
      public_launch_ready: false,
      android_s5_real_device_claimed: false,
      refreshed_evidence: {
        public_launch_readiness: {
          summary_path: "/fixture/public-launch-readiness.json",
          log_path: "/fixture/release-review-public-launch-readiness.log",
          status: "blocked_missing_public_launch_evidence"
        },
        release_signoff_summary: {
          summary_path: "/fixture/release-signoff-summary.json",
          log_path: "/fixture/release-review-signoff-summary.log",
          status: "release_signoff_summary_ready_with_public_launch_blockers"
        }
      },
      gates: {
        native_bevy_keyboard_replay_ready: true,
        public_launch_consumes_replay: true,
        native_bevy_action_coach_ready: true,
        native_bevy_player_hud_debug_layer_ready: true,
        native_bevy_live_window_screenshot_sequence_ready: true,
        native_bevy_sprite_texture_sampling_ready: true,
        native_bevy_live_window_sampled_texture_correlation_ready: true,
        native_bevy_render_asset_eligibility_ready: $render_asset_ready,
        cex_adapter_readiness_ready: true,
        public_launch_consumes_local_playability: $consumes_local_playability,
        s5_real_device_ready: false,
        release_latency_ready: true,
        release_rollback_backup_ready: true,
        public_deploy_ready: true
      },
      signoff_summary_blockers: $signoff_summary_blockers,
      public_launch_blockers: [
        "s5_real_device_matrix",
        "production_map_pack_public_evidence",
        "first_beta_cohort_evidence",
        "commercial_launch_drill_evidence",
        "multi_node_or_live_traffic_latency_evidence",
        "public_network_live_exposure_evidence"
      ]
    }' >"$quickcheck_json"
  add_artifact_from_path release_review_quickcheck "Release review quickcheck" "$quickcheck_json" release_review_input
}

add_release_review_status_packet_fixtures() {
  local mode="${1:-valid}"
  local status_json="$TMP_DIR/release-review-status.json"
  local public_launch_ready=false
  local android_s5_real_device_claimed=false
  local ready_items_json='[
    {"id":"native_bevy_keyboard_replay","label":"Native/Bevy keyboard replay","ready":true,"evidence_path":"/fixture/bevy-build-branch-title-route-all-branch-keyboard-replay.json","detail":"force=10, agility=8, craft=7; force combat=victory"},
    {"id":"native_bevy_action_coach","label":"Native/Bevy action coach","ready":true,"evidence_path":"/fixture/bevy-action-coach.json","detail":"coach_stage=true, enter_execution=true, final_next=true"},
    {"id":"native_bevy_player_hud_debug_layer","label":"Native/Bevy player HUD/debug layer","ready":true,"evidence_path":"/fixture/bevy-player-hud-debug-layer.json","detail":"player_hud=true, debug_layer=true"},
    {"id":"native_bevy_live_window_screenshot_sequence","label":"Native/Bevy live-window screenshot sequence","ready":true,"evidence_path":"/fixture/bevy-live-window-screenshot-sequence.json","detail":"frames=11, sequence=true, contact_sheet=true"},
    {"id":"native_bevy_sprite_texture_sampling","label":"Native/Bevy sprite texture sampling","ready":true,"evidence_path":"/fixture/bevy-sprite-texture-sampling.json","detail":"sampled_surfaces=32, unique_rgba=10, four_layer=true"},
    {"id":"native_bevy_live_window_sampled_texture_correlation","label":"Native/Bevy sampled texture live-window correlation","ready":true,"evidence_path":"/fixture/bevy-live-window-sampled-texture-correlation.json","detail":"live_frames=11, final_frame_colors=3376, four_layer=true"},
    {"id":"native_bevy_render_asset_eligibility","label":"Native/Bevy render asset eligibility","ready":true,"evidence_path":"/fixture/bevy-render-asset-eligibility.json","detail":"usage=RenderAssetUsages(MAIN_WORLD | RENDER_WORLD), sprite_refs=32, render_usage=true"},
    {"id":"cex_adapter_readiness","label":"CEX production world adapter readiness","ready":true,"evidence_path":"/fixture/cex-production-adapter-readiness.json","detail":"routes=7236, nodes=24, protocol=trillionnium_world_runtime_adapter_v1"},
    {"id":"public_launch_consumes_replay","label":"Public launch consumes replay gate","ready":true,"evidence_path":"/fixture/public-launch-readiness.json","detail":"blocked_missing_public_launch_evidence"},
    {"id":"public_launch_consumes_local_playability","label":"Public launch consumes local playability gates","ready":true,"evidence_path":"/fixture/public-launch-readiness.json","detail":"blocked_missing_public_launch_evidence"},
    {"id":"release_latency_local_drill","label":"Release latency local drill","ready":true,"evidence_path":"/fixture/release-latency-drill.json","detail":"local_release_latency_drill_green"},
    {"id":"release_rollback_backup_drill","label":"Release rollback/backup drill","ready":true,"evidence_path":"/fixture/release-rollback-backup-drill.json","detail":"release_rollback_backup_drill_green"},
    {"id":"public_deploy_local_drill","label":"Public deploy local drill","ready":true,"evidence_path":"/fixture/public-network-deploy-evidence.json","detail":"local_public_deploy_drill_green"}
  ]'

  if [[ "$mode" == "semantic_invalid" ]]; then
    public_launch_ready=true
    android_s5_real_device_claimed=true
    ready_items_json='[
      {"id":"native_bevy_keyboard_replay","label":"Native/Bevy keyboard replay","ready":true,"evidence_path":"/fixture/bevy-build-branch-title-route-all-branch-keyboard-replay.json","detail":"force=10, agility=8, craft=7; force combat=victory"}
    ]'
  fi

  jq -n \
    --argjson public_launch_ready "$public_launch_ready" \
    --argjson android_s5_real_device_claimed "$android_s5_real_device_claimed" \
    --argjson ready_items "$ready_items_json" \
    '{
      contract_version: "trillionnium_world_release_review_status_v1",
      status: "release_review_ready_public_launch_blocked",
      source_of_truth: "trillionnium_world_release_review_status",
      quickcheck_summary: "/fixture/release-review-quickcheck.json",
      signoff_summary: "/fixture/release-signoff-summary.json",
      quickcheck_log: "/fixture/release-review-status-quickcheck.log",
      markdown_path: "/fixture/release-review-status.md",
      ready_for_release_review: true,
      public_launch_ready: $public_launch_ready,
      android_s5_real_device_claimed: $android_s5_real_device_claimed,
      boundary: {
        native_bevy_replay_scope: "host_side_bevy_runtime_replay_not_android_real_device",
        native_bevy_texture_render_scope: "host_side_texture_sampling_correlation_and_render_asset_eligibility_not_gpu_upload_or_android_real_device",
        public_launch_claim: "blocked_until_real_external_evidence_is_attached"
      },
      ready_items: $ready_items,
      blocked_items: [
        {
          id: "s5_real_device_matrix",
          label: "S5 Android real-device matrix",
          needed: "Connect an Android device and collect launch, screenshot, gfxinfo/frame, CJK/input, lifecycle, weak-network, APK resource/signature, and crash-free logcat evidence."
        },
        {
          id: "production_map_pack_public_evidence",
          label: "Production map-pack public evidence",
          needed: "Provide production/public map-pack ready evidence, not only the local route or fixture-signed manifest."
        },
        {
          id: "first_beta_cohort_evidence",
          label: "First beta cohort evidence",
          needed: "Attach real 5-10 participant cohort evidence with status first_beta_cohort_evidence_green."
        },
        {
          id: "commercial_launch_drill_evidence",
          label: "Commercial launch drill evidence",
          needed: "Attach real or sanitized payment, refund, support, legal, operator, and traffic drill evidence."
        },
        {
          id: "multi_node_or_live_traffic_latency_evidence",
          label: "Multi-node or live-traffic latency evidence",
          needed: "Provide multi-node release latency or live public traffic latency evidence; local latency drill is not enough."
        },
        {
          id: "public_network_live_exposure_evidence",
          label: "Public network live exposure evidence",
          needed: "Provide approved host, domain/TLS, monitoring, backup, rollback, and public URL probe evidence."
        }
      ],
      reviewer_next_action: "collect_real_external_public_launch_evidence"
    }' >"$status_json"
  add_artifact_from_path release_review_status_json "Release review status JSON" "$status_json" release_review_checklist
}

add_release_review_convergence_packet_fixtures() {
  local mode="${1:-valid}"
  local convergence_json="$TMP_DIR/release-review-convergence.json"
  local public_launch_ready=false
  local android_s5_real_device_claimed=false
  local checks_json='[
    {"name":"cex_adapter_readiness_refresh","status":"ok","path":"/fixture/release-review-convergence-cex-adapter-readiness.log","detail":"refreshed"},
    {"name":"release_review_status_refresh","status":"ok","path":"/fixture/release-review-convergence-status.log","detail":"refreshed"},
    {"name":"quickcheck_script","status":"ok","path":"/fixture/check_trillionnium_world_release_review_quickcheck.sh","detail":"executable"},
    {"name":"status_script","status":"ok","path":"/fixture/check_trillionnium_world_release_review_status.sh","detail":"executable"},
    {"name":"convergence_script","status":"ok","path":"/fixture/check_trillionnium_world_release_review_convergence.sh","detail":"executable"},
    {"name":"cex_adapter_readiness_script","status":"ok","path":"/fixture/check_trillionnium_world_cex_adapter_readiness.sh","detail":"executable"},
    {"name":"readme_guard","status":"ok","path":"/fixture/root_readme_world_release_review_quickcheck_guard_test.sh","detail":"executable"},
    {"name":"status_guard","status":"ok","path":"/fixture/release_review_status_script_contract_guard_test.sh","detail":"executable"},
    {"name":"doc_README_md_quickcheck","status":"ok","path":"/fixture/README.md","detail":"contains: check_trillionnium_world_release_review_quickcheck.sh"},
    {"name":"doc_README_md_status","status":"ok","path":"/fixture/README.md","detail":"contains: check_trillionnium_world_release_review_status.sh"},
    {"name":"doc_README_md_convergence","status":"ok","path":"/fixture/README.md","detail":"contains: check_trillionnium_world_release_review_convergence.sh"},
    {"name":"doc_trillionnium_world_unified_development_doc_v1_md_quickcheck","status":"ok","path":"/fixture/trillionnium-world-unified-development-doc-v1.md","detail":"contains: check_trillionnium_world_release_review_quickcheck.sh"},
    {"name":"doc_trillionnium_world_unified_development_doc_v1_md_status","status":"ok","path":"/fixture/trillionnium-world-unified-development-doc-v1.md","detail":"contains: check_trillionnium_world_release_review_status.sh"},
    {"name":"doc_trillionnium_world_unified_development_doc_v1_md_convergence","status":"ok","path":"/fixture/trillionnium-world-unified-development-doc-v1.md","detail":"contains: check_trillionnium_world_release_review_convergence.sh"},
    {"name":"doc_trillionnium_world_cex_full_split_plan_v1_md_quickcheck","status":"ok","path":"/fixture/trillionnium-world-cex-full-split-plan-v1.md","detail":"contains: check_trillionnium_world_release_review_quickcheck.sh"},
    {"name":"doc_trillionnium_world_cex_full_split_plan_v1_md_status","status":"ok","path":"/fixture/trillionnium-world-cex-full-split-plan-v1.md","detail":"contains: check_trillionnium_world_release_review_status.sh"},
    {"name":"doc_trillionnium_world_cex_full_split_plan_v1_md_convergence","status":"ok","path":"/fixture/trillionnium-world-cex-full-split-plan-v1.md","detail":"contains: check_trillionnium_world_release_review_convergence.sh"},
    {"name":"doc_trillionnium_world_dev_environment_v1_md_quickcheck","status":"ok","path":"/fixture/trillionnium-world-dev-environment-v1.md","detail":"contains: check_trillionnium_world_release_review_quickcheck.sh"},
    {"name":"doc_trillionnium_world_dev_environment_v1_md_status","status":"ok","path":"/fixture/trillionnium-world-dev-environment-v1.md","detail":"contains: check_trillionnium_world_release_review_status.sh"},
    {"name":"doc_trillionnium_world_dev_environment_v1_md_convergence","status":"ok","path":"/fixture/trillionnium-world-dev-environment-v1.md","detail":"contains: check_trillionnium_world_release_review_convergence.sh"},
    {"name":"workflow_readme_guard","status":"ok","path":"/fixture/trnm-gate-quick-check.yml","detail":"contains: root_readme_world_release_review_quickcheck_guard_test.sh"},
    {"name":"workflow_status_guard","status":"ok","path":"/fixture/trnm-gate-quick-check.yml","detail":"contains: release_review_status_script_contract_guard_test.sh"},
    {"name":"native_bevy_keyboard_replay","status":"ok","path":"/fixture/bevy-build-branch-title-route-all-branch-keyboard-replay.json","detail":"contract green, 3 branches, keyboard replay counts, force combat victory"},
    {"name":"native_bevy_action_coach","status":"ok","path":"/fixture/bevy-action-coach.json","detail":"action coach contract green with Android S5 no-claim boundary"},
    {"name":"native_bevy_player_hud_debug_layer","status":"ok","path":"/fixture/bevy-player-hud-debug-layer.json","detail":"player HUD/debug layer contract green with Android S5 no-claim boundary"},
    {"name":"native_bevy_live_window_screenshot_sequence","status":"ok","path":"/fixture/bevy-live-window-screenshot-sequence.json","detail":"live-window screenshot sequence contract green with Android S5 no-claim boundary"},
    {"name":"native_bevy_sprite_texture_sampling","status":"ok","path":"/fixture/bevy-sprite-texture-sampling.json","detail":"sprite texture sampling contract green with host-side CPU sampling boundary"},
    {"name":"native_bevy_live_window_sampled_texture_correlation","status":"ok","path":"/fixture/bevy-live-window-sampled-texture-correlation.json","detail":"sampled texture live-window correlation contract green with Android S5 no-claim boundary"},
    {"name":"native_bevy_render_asset_eligibility","status":"ok","path":"/fixture/bevy-render-asset-eligibility.json","detail":"render asset eligibility contract green without claiming extraction/GPU/Android"},
    {"name":"cex_adapter_readiness","status":"ok","path":"/fixture/cex-production-adapter-readiness.json","detail":"CEX production adapter readiness evidence green without importing CEX internals"},
    {"name":"public_launch_consumes_replay","status":"ok","path":"/fixture/public-launch-readiness.json","detail":"native replay gate consumed; replay contract is not a blocker"},
    {"name":"public_launch_consumes_local_playability","status":"ok","path":"/fixture/public-launch-readiness.json","detail":"action coach, player HUD, live screenshot, texture sampling, sampled correlation, and render eligibility gates consumed; local playability contracts are not blockers"},
    {"name":"release_signoff_summary","status":"ok","path":"/fixture/release-signoff-summary.json","detail":"signoff summary keeps local Bevy playability, texture sampling, render eligibility, CEX adapter readiness ready; public-launch consumed; Android S5 unclaimed"},
    {"name":"release_review_quickcheck","status":"ok","path":"/fixture/release-review-quickcheck.json","detail":"quickcheck green for review with public-launch blockers, CEX adapter readiness, and local Bevy texture/render playability gates"},
    {"name":"release_review_status_json","status":"ok","path":"/fixture/release-review-status.json","detail":"status checklist has expanded green review items including CEX adapter readiness and six external blockers"},
    {"name":"release_review_status_markdown_green","status":"ok","path":"/fixture/release-review-status.md","detail":"contains: Green For Review"},
    {"name":"release_review_status_markdown_blockers","status":"ok","path":"/fixture/release-review-status.md","detail":"contains: Still Requires Real External Evidence"},
    {"name":"release_review_status_markdown_boundary","status":"ok","path":"/fixture/release-review-status.md","detail":"contains: Native/Bevy keyboard replay, classic animation preview/selector, classic player motion, action coach, HUD/debug layer, player UI rescue, live screenshots, sprite texture sampling, sampled texture live-window correlation, and render asset eligibility are host-side proof, not Android real-device proof."},
    {"name":"release_review_status_markdown_cex_boundary","status":"ok","path":"/fixture/release-review-status.md","detail":"contains: CEX adapter readiness is incubator runtime adapter evidence, not real external public-launch evidence."}
  ]'

  if [[ "$mode" == "semantic_invalid" ]]; then
    public_launch_ready=true
    android_s5_real_device_claimed=true
    checks_json='[
      {"name":"release_review_status_refresh","status":"ok","path":"/fixture/release-review-convergence-status.log","detail":"refreshed"}
    ]'
  fi

  jq -n \
    --argjson public_launch_ready "$public_launch_ready" \
    --argjson android_s5_real_device_claimed "$android_s5_real_device_claimed" \
    --argjson checks "$checks_json" \
    '{
      contract_version: "trillionnium_world_release_review_convergence_v1",
      status: "release_review_convergence_green_with_public_launch_blockers",
      source_of_truth: "trillionnium_world_release_review_convergence",
      green: true,
      ready_for_release_review: true,
      public_launch_ready: $public_launch_ready,
      android_s5_real_device_claimed: $android_s5_real_device_claimed,
      proof_scope: "host_side_bevy_runtime_replay_not_android_real_device",
      refreshed_status: {
        json_path: "/fixture/release-review-status.json",
        markdown_path: "/fixture/release-review-status.md",
        log_path: "/fixture/release-review-convergence-status.log"
      },
      convergence_rule: "release_review_status_must_refresh_and_scripts_docs_workflow_guards_evidence_outputs_must_remain_connected",
      checks: $checks,
      failures: [],
      reviewer_next_action: "collect_real_external_public_launch_evidence"
    }' >"$convergence_json"
  add_artifact_from_path release_review_convergence "Release review convergence" "$convergence_json" release_review_gate
}
