#!/usr/bin/env bash

add_visual_foundation_packet_fixtures() {
  local scene_preview_json="$TMP_DIR/bevy-classic-scene-preview.json"
  jq -n '{
    contract_version: "trillionnium_world_bevy_classic_scene_preview_v1",
    status: "classic_scene_preview_green",
    green: true,
    ready_for_release_review: true,
    preview_format: "ppm_p3_rgb",
    preview_width: 1280,
    preview_height: 720,
    preview_bytes: 8704298,
    panel_count: 4,
    panel_summary_count: 4,
    dynamic_landmark_frame_id_count: 12,
    unique_panel_scene_count: 3,
    unique_panel_player_frame_count: 4,
    gate_count: 7,
    passed_gate_count: 7,
    failed_gate_count: 0,
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
    ready_for_release_review: true,
    catalog_format: "ppm_p3_rgb",
    catalog_width: 640,
    catalog_height: 1056,
    catalog_bytes: 6213869,
    frame_count: 43,
    rendered_frame_count: 43,
    frame_summary_count: 8,
    role_family_count: 7,
    gate_count: 8,
    passed_gate_count: 8,
    failed_gate_count: 0,
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
    label_gate: true,
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
    ready_for_release_review: true,
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
    gate_count: 6,
    passed_gate_count: 6,
    failed_gate_count: 0,
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
    runtime_probe_contract: "trillionnium_world_bevy_runtime_probe_v1",
    source_of_truth: "XTest mouse button events click Bevy-exposed client hit centers on the visible X11 window and xwd captures each post-action frame",
    green: true,
    ready_for_release_review: true,
    display: ":0",
    window_id: "0xa00004",
    host_pid: 160672,
    slot_dir: "fixture-slots",
    slot_a_path: "fixture-slots/bevy-session-slot-a.snapshot.json",
    slot_a_bytes: 41520,
    hit_map_path: "fixture-hit-map.json",
    probe_path: "fixture-runtime-probe.json",
    contact_sheet_path: "fixture-contact-sheet.png",
    contact_sheet_size: [560, 1228],
    contact_sheet_colors: 11972,
    contact_sheet_mean: [44.22, 49.38, 38.44],
    expected_frame_ids: ["title", "create", "talk", "train", "training_room", "arena", "fight_result", "save_continue", "title_continue", "resume_continue", "complete"],
    actual_frame_ids: ["title", "create", "talk", "train", "training_room", "arena", "fight_result", "save_continue", "title_continue", "resume_continue", "complete"],
    expected_frame_count: 11,
    actual_frame_count: 11,
    expected_action_labels: ["TITLE:NEW", "CREATE:CONFIRM", "TALK", "TRAIN", "MOVE:north", "FIGHT", "SAVE:SELECTED", "TITLE:OPEN", "TITLE:CONTINUE", "CONTINUE:SESSION"],
    expected_action_label_count: 10,
    actions: [
      {step_index: 1, step_id: "title_new", action_label: "TITLE:NEW", target_frame_id: "create", client_x: 390, client_y: 473, row_id: "title", source: "native_control_button"},
      {step_index: 2, step_id: "character_confirm", action_label: "CREATE:CONFIRM", target_frame_id: "talk", client_x: 470, client_y: 473, row_id: "character_create", source: "native_control_button"},
      {step_index: 3, step_id: "mentor_talk", action_label: "TALK", target_frame_id: "train", client_x: 332, client_y: 473, row_id: "core", source: "native_control_button"},
      {step_index: 4, step_id: "mentor_train", action_label: "TRAIN", target_frame_id: "training_room", client_x: 332, client_y: 473, row_id: "core_route_actions_reflowed", source: "state_specific_visible_button"},
      {step_index: 5, step_id: "move_north", action_label: "MOVE:north", target_frame_id: "arena", client_x: 137, client_y: 412, row_id: "movement", source: "text_adventure_key_button"},
      {step_index: 6, step_id: "fight", action_label: "FIGHT", target_frame_id: "fight_result", client_x: 332, client_y: 473, row_id: "core_route_actions_reflowed", source: "state_specific_visible_button"},
      {step_index: 7, step_id: "save_selected", action_label: "SAVE:SELECTED", target_frame_id: "save_continue", client_x: 513, client_y: 506, row_id: "selected_slot", source: "native_control_button"},
      {step_index: 8, step_id: "title_open", action_label: "TITLE:OPEN", target_frame_id: "title_continue", client_x: 316, client_y: 473, row_id: "title", source: "native_control_button"},
      {step_index: 9, step_id: "title_continue", action_label: "TITLE:CONTINUE", target_frame_id: "resume_continue", client_x: 474, client_y: 473, row_id: "title", source: "native_control_button"},
      {step_index: 10, step_id: "continue_session", action_label: "CONTINUE:SESSION", target_frame_id: "complete", client_x: 390, client_y: 473, row_id: "selected_slot", source: "native_control_button"}
    ],
    action_count: 10,
    focus_event: {method: "XRaiseWindow+XSetInputFocus", window_id: "0xa00004"},
    mouse_events: [
      {action_label: "TITLE:NEW", step_id: "title_new", target_frame_id: "create", attempt: 1, relative: [390, 473], absolute: [390, 473], window_origin: [0, 0], window_size: [960, 540], source: "native_control_button", row_id: "title"},
      {action_label: "CREATE:CONFIRM", step_id: "character_confirm", target_frame_id: "talk", attempt: 1, relative: [470, 473], absolute: [470, 473], window_origin: [0, 0], window_size: [960, 540], source: "native_control_button", row_id: "character_create"},
      {action_label: "TALK", step_id: "mentor_talk", target_frame_id: "train", attempt: 1, relative: [332, 473], absolute: [332, 473], window_origin: [0, 0], window_size: [960, 540], source: "native_control_button", row_id: "core"},
      {action_label: "TRAIN", step_id: "mentor_train", target_frame_id: "training_room", attempt: 1, relative: [332, 473], absolute: [332, 473], window_origin: [0, 0], window_size: [960, 540], source: "state_specific_visible_button", row_id: "core_route_actions_reflowed"},
      {action_label: "MOVE:north", step_id: "move_north", target_frame_id: "arena", attempt: 1, relative: [137, 412], absolute: [137, 412], window_origin: [0, 0], window_size: [960, 540], source: "text_adventure_key_button", row_id: "movement"},
      {action_label: "FIGHT", step_id: "fight", target_frame_id: "fight_result", attempt: 1, relative: [332, 473], absolute: [332, 473], window_origin: [0, 0], window_size: [960, 540], source: "state_specific_visible_button", row_id: "core_route_actions_reflowed"},
      {action_label: "SAVE:SELECTED", step_id: "save_selected", target_frame_id: "save_continue", attempt: 1, relative: [513, 506], absolute: [513, 506], window_origin: [0, 0], window_size: [960, 540], source: "native_control_button", row_id: "selected_slot"},
      {action_label: "TITLE:OPEN", step_id: "title_open", target_frame_id: "title_continue", attempt: 1, relative: [316, 473], absolute: [316, 473], window_origin: [0, 0], window_size: [960, 540], source: "native_control_button", row_id: "title"},
      {action_label: "TITLE:CONTINUE", step_id: "title_continue", target_frame_id: "resume_continue", attempt: 1, relative: [474, 473], absolute: [474, 473], window_origin: [0, 0], window_size: [960, 540], source: "native_control_button", row_id: "title"},
      {action_label: "CONTINUE:SESSION", step_id: "continue_session", target_frame_id: "complete", attempt: 1, relative: [390, 473], absolute: [390, 473], window_origin: [0, 0], window_size: [960, 540], source: "native_control_button", row_id: "selected_slot"}
    ],
    mouse_event_count: 10,
    step_results: [
      {step_index: 1, step_id: "title_new", action_label: "TITLE:NEW", actual_accepted: true, reason: "enabled_title_new_game", input_feedback_toast: "TOAST OK | TITLE:NEW | enabled_title_new_game | NEXT CREATE:CONFIRM", state_check: {ok: true}},
      {step_index: 2, step_id: "character_confirm", action_label: "CREATE:CONFIRM", actual_accepted: true, reason: "enabled_character_create_confirm", input_feedback_toast: "TOAST OK | CREATE:CONFIRM | enabled_character_create_confirm | NEXT TALK", state_check: {ok: true}},
      {step_index: 3, step_id: "mentor_talk", action_label: "TALK", actual_accepted: true, reason: "enabled_at_mentor_tile", input_feedback_toast: "TOAST OK | TALK | enabled_at_mentor_tile | NEXT TRAIN", state_check: {ok: true}},
      {step_index: 4, step_id: "mentor_train", action_label: "TRAIN", actual_accepted: true, reason: "enabled_after_dialogue_choice", input_feedback_toast: "TOAST OK | TRAIN | enabled_after_dialogue_choice | NEXT MOVE:north", state_check: {ok: true}},
      {step_index: 5, step_id: "move_north", action_label: "MOVE:north", actual_accepted: true, reason: "enabled_route_step_north", input_feedback_toast: "TOAST OK | MOVE:north | enabled_route_step_north | NEXT FIGHT", state_check: {ok: true}},
      {step_index: 6, step_id: "fight", action_label: "FIGHT", actual_accepted: true, reason: "enabled_enemy_adjacent", input_feedback_toast: "TOAST OK | FIGHT | enabled_enemy_adjacent | NEXT SAVE:SELECTED", state_check: {ok: true}},
      {step_index: 7, step_id: "save_selected", action_label: "SAVE:SELECTED", actual_accepted: true, reason: "enabled_save_selected_slot:A", input_feedback_toast: "TOAST OK | SAVE:SELECTED | enabled_save_selected_slot:A | NEXT TITLE:OPEN", state_check: {ok: true}},
      {step_index: 8, step_id: "title_open", action_label: "TITLE:OPEN", actual_accepted: true, reason: "enabled_open_title_menu", input_feedback_toast: "TOAST OK | TITLE:OPEN | enabled_open_title_menu | NEXT TITLE:CONTINUE", state_check: {ok: true}},
      {step_index: 9, step_id: "title_continue", action_label: "TITLE:CONTINUE", actual_accepted: true, reason: "enabled_title_continue_slot:A", input_feedback_toast: "TOAST OK | TITLE:CONTINUE | enabled_title_continue_slot:A | NEXT CONTINUE:SESSION", state_check: {ok: true}},
      {step_index: 10, step_id: "continue_session", action_label: "CONTINUE:SESSION", actual_accepted: true, reason: "enabled_session_resume_continue", input_feedback_toast: "TOAST OK | CONTINUE:SESSION | enabled_session_resume_continue | NEXT FIRST MINUTE COMPLETE", state_check: {ok: true}}
    ],
    step_result_count: 10,
    changed_frame_count: 10,
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
    runtime_probe_gate: true,
    runtime_feedback_gate: true,
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
    input_sources: ["selection_follow", "camera_viewport_rect", "edge_scroll", "control_group_recall_camera", "minimap_route_projection", "wheel_zoom"],
    input_source_count: 6,
    input_action_count: 6,
    large_map: {map_width_tiles: 34, map_height_tiles: 34, playable_min_tile: 1, playable_max_x: 32, playable_max_y: 32},
    large_map_field_count: 5,
    stage_summaries: [
      {stage: "viewport_rect", selected_unit_id: "mirror_captain", control_group_id: "1", minimap_tile_id: null, focus_tile: {x: 8, y: 8}},
      {stage: "fog_reveal", selected_unit_id: "mirror_captain", control_group_id: "1", minimap_tile_id: null, focus_tile: {x: 9, y: 9}},
      {stage: "selection_follow", selected_unit_id: "mirror_captain", control_group_id: "1", minimap_tile_id: "mirror_captain", focus_tile: {x: 20, y: 19}},
      {stage: "control_group_recall", selected_unit_id: "field_engineer", control_group_id: "2", minimap_tile_id: null, focus_tile: {x: 19, y: 18}},
      {stage: "route_projection", selected_unit_id: "signal_lancer", control_group_id: "2", minimap_tile_id: "minimap_route_target", focus_tile: {x: 22, y: 20}},
      {stage: "zoom_sync", selected_unit_id: "mirror_captain", control_group_id: "1", minimap_tile_id: null, focus_tile: {x: 22, y: 20}}
    ],
    stage_summary_count: 6,
    stage_name_count: 6,
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
    large_map_minimap_gate: true,
    selection_follow_gate: true,
    control_group_sync_gate: true,
    route_projection_gate: true,
    zoom_rect_sync_gate: true,
    minimap_runtime_gate: true,
    scene_renderer_gate: true,
    original_art_policy_gate: true,
    gate_count: 16,
    passed_gate_count: 16,
    failed_gate_count: 0,
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

add_first_contact_basin_source_manifest_packet_fixtures() {
  local first_contact_basin_spec_json="$TMP_DIR/bevy-classic-rts-first-contact-basin-spec.json"
  jq -n '{
    contract_version: "trillionnium_world_bevy_classic_rts_first_contact_basin_spec_v1",
    status: "classic_rts_first_contact_basin_spec_green",
    green: true,
    map_id: "first_contact_basin",
    actor_count: 39,
    spawn_count: 4,
    flux_bloom_count: 11,
    beacon_count: 4,
    expansion_count: 4,
    unit_rule_count: 6,
    building_rule_count: 5,
    map_actor_gate: true,
    map_topology_gate: true,
    rules_gate: true,
    rts_data_source_manifest: {
      integration_mode: "gpl_internal_component",
      copied_or_derived: true,
      release_constraint: "internal_only_until_gpl_component_review_or_replacement",
      license: "GPL-3.0-or-later OpenRA Mod SDK prototype boundary"
    },
    rts_data_map_summary: {
      actor_count: 39,
      spawn_count: 4,
      flux_bloom_count: 11,
      beacon_count: 4,
      expansion_count: 4,
      source_integration_mode: "gpl_internal_component"
    },
    rts_data_validation_error: null,
    rts_data_consumer_gate: true,
    bevy_map_model_adapter_gate: true,
    rts_data_map_model_review: {
      map_summary: {
        actor_count: 39,
        spawn_count: 4,
        flux_bloom_count: 11,
        beacon_count: 4,
        expansion_count: 4,
        source_integration_mode: "gpl_internal_component"
      },
      unit_rule_count: 6,
      building_rule_count: 5,
      data_validation_error: null,
      map_actor_gate: true,
      map_topology_gate: true,
      rules_gate: true,
      data_consumer_gate: true,
      map_model_adapter_gate: true
    },
    rts_data_map_model_gate: true,
    rts_data_terrain_profile_count: 1156,
    rts_data_terrain_profile_samples: {
      border: {role: "border", height: 0, base_pad: false, resource_zone: false},
      lane: {role: "lane", height: 1, base_pad: false, resource_zone: false},
      center: {role: "central_basin", height: 2, base_pad: false, resource_zone: false},
      base_pad: {role: "base_pad", height: 1, base_pad: true, resource_zone: false},
      resource_zone: {role: "resource_zone", height: 1, base_pad: false, resource_zone: true}
    },
    rts_data_terrain_profile_gate: true,
    rts_data_renderer_projection_gate: true,
    rts_data_renderer_projection: {
      renderable_tile_count: 1024,
      lane_tile_count: 240,
      resource_zone_tile_count: 79,
      base_pad_tile_count: 144,
      minimap_anchor_actor_count: 39,
      resource_actor_tile_count: 11,
      objective_actor_tile_count: 4,
      spawn_actor_tile_count: 4,
      lane_tile_samples: [{x: 16, y: 1}],
      resource_actor_tile_samples: [{x: 12, y: 16}],
      objective_actor_tile_samples: [{x: 16, y: 9}],
      spawn_actor_tile_samples: [{x: 8, y: 8}],
      minimap_anchor_actor_samples: ["Actor0"],
      source: "RtsMapModel bounds, terrain profiles, actor rules, and runtime projection math"
    },
    rts_data_preview_actor_contract: "trnm_rts_data_first_contact_preview_actor_v1",
    rts_data_preview_actor_projection: {
      actor_count: 39,
      spawn_count: 4,
      flux_bloom_count: 11,
      beacon_count: 4,
      expansion_count: 4,
      actor_samples: [
        {
          contract_version: "trnm_rts_data_first_contact_preview_actor_v1",
          source_actor_id: "Actor0",
          kind: "spawn",
          owner: "Multi0",
          tile: {x: 8, y: 8},
          source_rule_id: "mpspawn",
          openra_preview_rule_id: "trnm.map.detail"
        },
        {
          contract_version: "trnm_rts_data_first_contact_preview_actor_v1",
          source_actor_id: "Actor2",
          kind: "flux_bloom",
          owner: "Neutral",
          tile: {x: 12, y: 16},
          source_rule_id: "trnm.flux.bloom",
          openra_preview_rule_id: "trnm.flux.bloom"
        },
        {
          contract_version: "trnm_rts_data_first_contact_preview_actor_v1",
          source_actor_id: "Actor5",
          kind: "spawn",
          owner: "Multi2",
          tile: {x: 25, y: 8},
          source_rule_id: "mpspawn",
          openra_preview_rule_id: "trnm.map.detail"
        }
      ],
      source: "trnm-rts-data first_contact_preview_actors projection from RtsMapModel actors"
    },
    rts_data_preview_actor_projection_gate: true,
    rts_data_preview_actors: ([
      {
        contract_version: "trnm_rts_data_first_contact_preview_actor_v1",
        source_actor_id: "Actor0",
        kind: "spawn",
        owner: "Multi0",
        tile: {x: 8, y: 8},
        source_rule_id: "mpspawn",
        openra_preview_rule_id: "trnm.map.detail"
      },
      {
        contract_version: "trnm_rts_data_first_contact_preview_actor_v1",
        source_actor_id: "Actor15",
        kind: "beacon",
        owner: "Neutral",
        tile: {x: 16, y: 9},
        source_rule_id: "trnm.flux.beacon",
        openra_preview_rule_id: "trnm.flux.beacon"
      },
      {
        contract_version: "trnm_rts_data_first_contact_preview_actor_v1",
        source_actor_id: "Actor35",
        kind: "expansion_marker",
        owner: "Neutral",
        tile: {x: 11, y: 8},
        source_rule_id: "trnm.expansion.marker",
        openra_preview_rule_id: "trnm.map.detail"
      }
    ] + [range(0;36) | {
      contract_version: "trnm_rts_data_first_contact_preview_actor_v1",
      source_actor_id: ("FixtureActor" + tostring),
      kind: "lane_marker",
      owner: "Neutral",
      tile: {x: 12, y: 12},
      source_rule_id: "trnm.lane.marker",
      openra_preview_rule_id: "trnm.map.detail"
    }]),
    rts_data_opening_profile: {
      contract_version: "trnm_rts_data_first_contact_opening_profile_v1",
      map_id: "first_contact_basin",
      active_beacon_tile: {x: 16, y: 9},
      active_relay_tile: {x: 11, y: 8}
    },
    rts_data_opening_profile_gate: true,
    rts_data_command_feedback_profile: {
      contract_version: "trnm_rts_data_first_contact_command_feedback_v1",
      map_id: "first_contact_basin",
      target_tile: {x: 16, y: 9},
      blocked_tile: {x: 15, y: 16}
    },
    rts_data_command_feedback_gate: true,
    rts_data_player_startup_profiles: [
      {contract_version: "trnm_rts_data_first_contact_player_startup_v1", map_id: "first_contact_basin", player_id: "Multi0", faction: "horizon", spawn_tile: {x: 8, y: 8}, faction_unit_rule_id: "trnm.horizon.scout"},
      {contract_version: "trnm_rts_data_first_contact_player_startup_v1", map_id: "first_contact_basin", player_id: "Multi1", faction: "forge", spawn_tile: {x: 25, y: 25}, faction_unit_rule_id: "trnm.forge.warden"},
      {contract_version: "trnm_rts_data_first_contact_player_startup_v1", map_id: "first_contact_basin", player_id: "Multi2", faction: "horizon", spawn_tile: {x: 25, y: 8}, faction_unit_rule_id: "trnm.horizon.scout"},
      {contract_version: "trnm_rts_data_first_contact_player_startup_v1", map_id: "first_contact_basin", player_id: "Multi3", faction: "forge", spawn_tile: {x: 8, y: 25}, faction_unit_rule_id: "trnm.forge.warden"}
    ],
    rts_data_player_startup_gate: true,
    rts_data_actor_presentation_profiles: [
      {contract_version: "trnm_rts_data_first_contact_actor_presentation_v1", map_id: "first_contact_basin", rule_id: "mpspawn", color_role: "map_detail", glyph_role: "map_detail", structure: false, selectable: false, glyph: {contract_version: "trnm_rts_data_first_contact_actor_glyph_v1", body: "spawn_pad", accent: "owner_stripe", footprint_width_cells: 3}},
      {contract_version: "trnm_rts_data_first_contact_actor_presentation_v1", map_id: "first_contact_basin", rule_id: "trnm.worker", color_role: "worker", glyph_role: "worker", structure: false, selectable: true, glyph: {contract_version: "trnm_rts_data_first_contact_actor_glyph_v1", body: "unit", accent: "worker_cargo", selection_ring: true}},
      {contract_version: "trnm_rts_data_first_contact_actor_presentation_v1", map_id: "first_contact_basin", rule_id: "trnm.command.core", color_role: "command_core", glyph_role: "command_core", structure: true, health_bar_width: 36, glyph: {contract_version: "trnm_rts_data_first_contact_actor_glyph_v1", body: "structure", accent: "command_spire", footprint_width_cells: 2}},
      {contract_version: "trnm_rts_data_first_contact_actor_presentation_v1", map_id: "first_contact_basin", rule_id: "trnm.flux.beacon", color_role: "objective", glyph_role: "beacon", structure: true, glyph: {contract_version: "trnm_rts_data_first_contact_actor_glyph_v1", body: "objective_beacon", accent: "beacon_core"}}
    ],
    rts_data_actor_presentation_gate: true,
    rts_data_visual_telemetry_profile: {
      contract_version: "trnm_rts_data_first_contact_visual_telemetry_v1",
      map_id: "first_contact_basin",
      unit_statuses: [{tile: {x: 8, y: 8}, role_badge: "W", role_color: "health", health_percent: 82, shield_percent: 44}],
      tactical_tracks: [{from_tile: {x: 11, y: 8}, to_tile: {x: 16, y: 9}, color_role: "action_trail"}]
    },
    rts_data_visual_telemetry_gate: true,
    rts_data_player_screen_profile: {
      contract_version: "trnm_rts_data_first_contact_player_screen_v1",
      map_id: "first_contact_basin",
      room_id: "first-contact-basin",
      layout: {
        player_map: {map_origin_x: 16},
        spec_map: {map_origin_x: 24}
      },
      chrome: {
        command_grid_slot_ids: ["worker", "scout", "warden", "relay", "core", "signal"]
      },
      command_queue: ["move:16,9", "build:trnm.flux.relay", "train:trnm.worker", "attack:trnm.flux.beacon"],
      production_queue: ["train:guard", "train:worker", "upgrade:signal_blade"],
      build_queue: ["build:watch_tower", "upgrade:training_hall"]
    },
    rts_data_player_screen_layout_profile: {
      player_map: {map_origin_x: 16},
      spec_map: {map_origin_x: 24}
    },
    rts_data_player_screen_chrome_profile: {
      command_grid_slot_ids: ["worker", "scout", "warden", "relay", "core", "signal"]
    },
    first_contact_player_screen_label_guard_contract: "trillionnium_world_bevy_classic_rts_first_contact_label_guard_v1",
    first_contact_player_screen_label_guard: {
      contract_version: "trillionnium_world_bevy_classic_rts_first_contact_label_guard_v1",
      green: true,
      source_path: "trnm-world-bevy classic_draw_openra_style_rts_shell display-label helpers",
      resource_labels: ["CREDITS", "POWER", "SUPPLY", "VISION"],
      resource_spacing_samples: [
        {label: "CREDITS", text_width_px: 42, value_x_delta_px: 66, value_spacing_gate: true},
        {label: "POWER", text_width_px: 30, value_x_delta_px: 54, value_spacing_gate: true},
        {label: "SUPPLY", text_width_px: 36, value_x_delta_px: 60, value_spacing_gate: true},
        {label: "VISION", text_width_px: 36, value_x_delta_px: 60, value_spacing_gate: true}
      ],
      production_slot_labels: ["GUARD", "WORKER", "SIGNAL", "TRAINING"],
      production_slot_badge_labels: ["GRD", "WRK", "SIG", "TRN"],
      production_slot_badge_widths: [18, 18, 18, 18],
      production_slot_status_labels: ["Q1 64 R", "Q2 42 R", "Q3 64 R", "B2 42 R"],
      production_empty_slot_status_labels: ["ADD UNIT", "ADD UNIT", "ADD BUILD", "ADD BUILD"],
      build_palette_labels: ["POWER", "TRAIN", "REFINE", "TOWER", "COMMAND", "RADAR", "WALL", "SIGNAL"],
      build_palette_badge_labels: ["PWR", "TRN", "REF", "TWR", "CMD", "RAD", "WAL", "SIG"],
      build_palette_badge_widths: [18, 18, 18, 18, 18, 18, 18, 18],
      build_palette_state_labels: ["READY", "QUEUE", "READY", "QUEUE", "READY", "READY", "READY", "QUEUE"],
      build_palette_fit_samples: [
        {label: "POWER", label_x: 108, right_x: 138, fits_tile_gate: true},
        {label: "TRAIN", label_x: 108, right_x: 138, fits_tile_gate: true},
        {label: "REFINE", label_x: 105, right_x: 141, fits_tile_gate: true},
        {label: "TOWER", label_x: 108, right_x: 138, fits_tile_gate: true},
        {label: "COMMAND", label_x: 102, right_x: 144, fits_tile_gate: true},
        {label: "RADAR", label_x: 108, right_x: 138, fits_tile_gate: true},
        {label: "WALL", label_x: 111, right_x: 135, fits_tile_gate: true},
        {label: "SIGNAL", label_x: 105, right_x: 141, fits_tile_gate: true}
      ],
      build_palette_badge_fit_samples: [
        {label: "PWR", label_x: 114, right_x: 132, fits_tile_gate: true},
        {label: "TRN", label_x: 114, right_x: 132, fits_tile_gate: true},
        {label: "REF", label_x: 114, right_x: 132, fits_tile_gate: true},
        {label: "TWR", label_x: 114, right_x: 132, fits_tile_gate: true},
        {label: "CMD", label_x: 114, right_x: 132, fits_tile_gate: true},
        {label: "RAD", label_x: 114, right_x: 132, fits_tile_gate: true},
        {label: "WAL", label_x: 114, right_x: 132, fits_tile_gate: true},
        {label: "SIG", label_x: 114, right_x: 132, fits_tile_gate: true}
      ],
      order_queue_labels: ["ATTACK BEACON", "TRAIN WORKER", "BUILD RELAY", "MOVE 16/9"],
      completion_event_labels: ["WORKER READY", "SIGNAL READY", "TOWER READY", "TRAINING READY"],
      tactics_queue_summary: "GUARD 64% TOWER 42%",
      tactics_target_label: "RELAY BEACON",
      tactics_build_label: "IDLE",
      tactics_detail_labels: [
        "SECURE RELAY BEACON",
        "RELAY BEACON",
        "16/16",
        "GUARD 64% TOWER 42%",
        "IDLE"
      ],
      tactics_compact_badge_labels: ["SECURE", "BEACON", "16/16", "G64/T42", "IDLE"],
      tactics_compact_badge_widths: [36, 36, 30, 42, 24],
      tactics_queue_fallback_values: ["TRAIN SIGNAL", "TRAIN SI", "BUILD RELAY", "ATTACK BEACON", "READY"],
      tactics_queue_fallback_badge_labels: ["TRN SIG", "TRN SIG", "BLD RLY", "ATK BCN", "RDY"],
      tactics_queue_fallback_badge_widths: [42, 42, 42, 42, 18],
      field_status_title: "FIELD STATUS",
      live_status_labels: [
        "SQUAD READY",
        "RALLY 16/9",
        "QUEUE GUARD 64%",
        "SCOUTING 76%",
        "CAMERA 16/16",
        "SUPPLY 12/22",
        "SAVE ROUTE READY"
      ],
      live_state_labels: [
        "ORDER SECURE RELAY BEACON",
        "TARGET RELAY BEACON",
        "BUILD IDLE",
        "QUEUE GUARD 64%",
        "CAM 16/16",
        "DRAG NONE",
        "HOVER NONE",
        "RES UPGRADE 210 CREDITS"
      ],
      tactical_header_title: "TACTICAL VIEW",
      tactical_header_order_label: "SECURE RELAY BEACON",
      tactical_header_camera_label: "CAM 16/16 Z100",
      tactical_header_title_width_px: 76,
      tactical_header_order_width_px: 110,
      tactical_header_camera_width_px: 80,
      tactical_header_title_order_width_px: 200,
      build_placement_status_label: "PLACE TOWER 210C READY",
      upgrade_placement_status_label: "PLACE TRAINING 120C READY",
      build_placement_status_width_px: 126,
      upgrade_placement_status_width_px: 144,
      forbidden_display_fragments: ["TRNM", "PRODUCTION COMPLETE", "BUILD COMPLETE", "UPGRADE COMPLETE", "LIVE INPUT", "LMB", "WASD", "CTRL", "SHIFT", "PROD ", ":", ".", "@", "_", "->"],
      expected_label_gate: true,
      resource_spacing_gate: true,
      production_slot_width_gate: true,
      production_slot_status_width_gate: true,
      build_palette_width_gate: true,
      build_palette_state_width_gate: true,
      order_queue_width_gate: true,
      tactics_summary_width_gate: true,
      tactics_detail_width_gate: true,
      tactics_compact_badge_width_gate: true,
      tactics_queue_fallback_badge_width_gate: true,
      live_status_width_gate: true,
      live_state_width_gate: true,
      tactical_header_title_order_width_gate: true,
      tactical_header_camera_width_gate: true,
      build_placement_status_width_gate: true,
      tactical_header_vertical_separation_gate: true,
      raw_marker_gate: true
    },
    first_contact_player_screen_label_guard_gate: true,
    first_contact_visual_readability_contract: "trillionnium_world_bevy_classic_rts_first_contact_visual_readability_v1",
    first_contact_visual_readability_guard: {
      contract_version: "trillionnium_world_bevy_classic_rts_first_contact_visual_readability_v1",
      green: true,
      source_path: "trnm-world-bevy classic_draw_first_contact_readability_overlays",
      selected_tile_ids: ["14,11", "15,11", "15,12", "17,12"],
      selected_marker_pixel_budget: 224,
      selected_marker_gate: true,
      route_tile_ids: ["14,11", "15,11", "16,10", "16,9"],
      route_marker_pixel_budget: 72,
      route_marker_gate: true,
      command_destination_tile: "16,9",
      command_target_gate: true,
      structure_anchor_tiles: ["8,8", "25,8", "25,25", "8,25", "11,8", "22,25"],
      structure_outline_pixel_budget: 552,
      structure_outline_gate: true,
      objective_focus_tiles: ["16,9", "16,24", "9,16", "24,16"],
      objective_focus_pixel_budget: 288,
      objective_focus_gate: true,
      lane_edge_sample_tiles: ["16,1", "16,2", "16,3", "16,4", "16,5", "16,6", "16,7", "16,8", "16,9", "16,10", "16,11", "16,12"],
      lane_edge_pixel_budget: 192,
      terrain_lane_edge_gate: true
    },
    first_contact_visual_readability_guard_gate: true,
    first_contact_radar_readability_contract: "trillionnium_world_bevy_classic_rts_first_contact_radar_readability_v1",
    first_contact_radar_readability_guard: {
      contract_version: "trillionnium_world_bevy_classic_rts_first_contact_radar_readability_v1",
      green: true,
      source_path: "trnm-world-bevy classic_draw_first_contact_radar_context",
      known_terrain_cell_count: 1024,
      fog_context_cell_count: 960,
      terrain_context_pixel_budget: 2048,
      known_terrain_gate: true,
      fog_context_gate: true,
      visible_tile_ids: ["12,12", "13,12", "14,12", "15,12", "16,12", "17,12", "18,12", "19,12"],
      visible_tile_pixel_budget: 192,
      visible_tile_gate: true,
      selected_tile_ids: ["14,11", "15,11", "15,12", "17,12"],
      selected_blip_pixel_budget: 64,
      selected_blip_gate: true,
      route_tile_ids: ["14,11", "15,11", "16,10", "16,9"],
      route_trace_pixel_budget: 48,
      route_trace_gate: true,
      command_destination_tile: "16,9",
      command_destination_gate: true,
      objective_tiles: ["16,9", "16,24", "9,16", "24,16"],
      objective_blip_pixel_budget: 128,
      objective_blip_gate: true,
      structure_tiles: ["8,8", "25,8", "25,25", "8,25", "11,8", "22,25"],
      structure_blip_pixel_budget: 144,
      structure_blip_gate: true,
      pressure_tiles: ["25,25", "25,8", "24,16"],
      pressure_blip_pixel_budget: 54,
      pressure_blip_gate: true,
      lane_sample_tiles: ["16,4", "16,6", "16,8", "16,10", "16,12", "16,14", "16,16", "16,18", "16,20", "16,22", "16,24", "16,26", "16,28", "16,30", "6,16", "8,16", "10,16", "12,16", "14,16", "16,16", "18,16", "20,16", "22,16", "24,16", "26,16", "28,16"],
      lane_context_pixel_budget: 156,
      lane_context_gate: true,
      viewport_frame_gate: true
    },
    first_contact_radar_readability_guard_gate: true,
    first_contact_command_grid_readability_contract: "trillionnium_world_bevy_classic_rts_first_contact_command_grid_readability_v1",
    first_contact_command_grid_readability_guard: {
      contract_version: "trillionnium_world_bevy_classic_rts_first_contact_command_grid_readability_v1",
      green: true,
      source_path: "trnm-world-bevy classic_draw_rts_command_glyph role-specific First Contact command buttons",
      command_slot_ids: ["worker", "scout", "warden", "relay", "core", "signal", "worker", "scout", "warden", "relay", "core", "signal"],
      command_icon_roles: ["worker", "scout", "warden", "relay", "core", "signal", "worker", "scout", "warden", "relay", "core", "signal"],
      command_icon_signatures: ["unit_pickaxe_ore", "diamond_eye_crosshair", "shield_barrier", "mast_broadcast", "stepped_base", "pulse_spire", "unit_pickaxe_ore", "diamond_eye_crosshair", "shield_barrier", "mast_broadcast", "stepped_base", "pulse_spire"],
      unique_icon_role_count: 6,
      unique_icon_signature_count: 6,
      top_row_roles: ["worker", "scout", "warden", "relay", "core", "signal"],
      bottom_row_roles: ["worker", "scout", "warden", "relay", "core", "signal"],
      active_slot_role: "worker",
      cooldown_badge_samples: [
        {slot: "worker", role: "worker", cooldown_percent: 0},
        {slot: "scout", role: "scout", cooldown_percent: 0},
        {slot: "warden", role: "warden", cooldown_percent: 16},
        {slot: "relay", role: "relay", cooldown_percent: 0},
        {slot: "core", role: "core", cooldown_percent: 42},
        {slot: "signal", role: "signal", cooldown_percent: 25}
      ],
      slot_badge_pixel_budget: 144,
      glyph_shape_pixel_budget: 1152,
      role_sequence_gate: true,
      repeated_rows_gate: true,
      unique_icon_gate: true,
      active_slot_gate: true,
      cooldown_badge_gate: true,
      player_screen_symbol_gate: true
    },
    first_contact_command_grid_readability_guard_gate: true,
    first_contact_sidebar_density_contract: "trillionnium_world_bevy_classic_rts_first_contact_sidebar_density_v1",
    first_contact_sidebar_density_guard: {
      contract_version: "trillionnium_world_bevy_classic_rts_first_contact_sidebar_density_v1",
      green: true,
      source_path: "trnm-world-bevy classic_draw_openra_style_rts_shell right sidebar production/build palette/tactics density",
      production_slot_visible_count: 4,
      production_slot_column_count: 2,
      production_row_count: 2,
      production_slot_labels: ["GUARD", "WORKER", "SIGNAL", "TRAINING"],
      production_slot_badge_labels: ["GRD", "WRK", "SIG", "TRN"],
      production_slot_badge_widths: [18, 18, 18, 18],
      production_slot_status_labels: ["Q1 64 R", "Q2 42 R", "Q3 64 R", "B2 42 R"],
      production_slot_status_badge_labels: ["Q1", "Q2", "Q3", "B2"],
      production_slot_status_badge_widths: [12, 12, 12, 12],
      production_empty_slot_status_badge_labels: ["ADD", "ADD", "ADD", "ADD"],
      production_empty_slot_status_badge_widths: [18, 18, 18, 18],
      production_empty_slot_badge_label: "READY",
      production_empty_slot_badge_width_px: 30,
      production_to_palette_gap_px: 16,
      build_palette_visible_count: 8,
      build_palette_column_count: 4,
      build_palette_row_count: 2,
      build_palette_slot_width_px: 46,
      build_palette_slot_height_px: 40,
      build_palette_row_gap_px: 48,
      build_palette_inter_row_gap_px: 8,
      build_palette_to_tactics_gap_px: 24,
      build_palette_labels: ["POWER", "TRAIN", "REFINE", "TOWER", "COMMAND", "RADAR", "WALL", "SIGNAL"],
      build_palette_badge_labels: ["PWR", "TRN", "REF", "TWR", "CMD", "RAD", "WAL", "SIG"],
      build_palette_badge_widths: [18, 18, 18, 18, 18, 18, 18, 18],
      build_palette_state_labels: ["READY", "QUEUE", "READY", "QUEUE", "READY", "READY", "READY", "QUEUE"],
      build_palette_state_badge_labels: ["RDY", "QUE", "RDY", "QUE", "RDY", "RDY", "RDY", "QUE"],
      build_palette_state_badge_widths: [18, 18, 18, 18, 18, 18, 18, 18],
      build_palette_state_badge_width_px: 24,
      build_palette_state_badge_height_px: 9,
      tactics_row_count: 5,
      tactics_row_gap_px: 4,
      tactics_detail_labels: ["SECURE RELAY BEACON", "RELAY BEACON", "16/16", "GUARD 64% TOWER 42%", "IDLE"],
      tactics_compact_badge_labels: ["SECURE", "BEACON", "16/16", "G64/T42", "IDLE"],
      tactics_compact_badge_widths: [36, 36, 30, 42, 24],
      tactics_queue_fallback_values: ["TRAIN SIGNAL", "TRAIN SI", "BUILD RELAY", "ATTACK BEACON", "READY"],
      tactics_queue_fallback_badge_labels: ["TRN SIG", "TRN SIG", "BLD RLY", "ATK BCN", "RDY"],
      tactics_queue_fallback_badge_widths: [42, 42, 42, 42, 18],
      production_density_gate: true,
      palette_geometry_gate: true,
      palette_state_badge_gate: true,
      palette_label_gate: true,
      tactics_density_gate: true,
      right_sidebar_density_gate: true
    },
    first_contact_sidebar_density_guard_gate: true,
    first_contact_bottom_panel_readability_contract: "trillionnium_world_bevy_classic_rts_first_contact_bottom_panel_readability_v1",
    first_contact_bottom_panel_readability_guard: {
      contract_version: "trillionnium_world_bevy_classic_rts_first_contact_bottom_panel_readability_v1",
      green: true,
      source_path: "trnm-world-bevy classic_draw_openra_style_rts_shell bottom selection/status panel",
      group_summary: "GROUP 1  4 UNITS SELECTED",
      selected_unit_display_count: 4,
      feedback_labels: ["GROUP 1 SECURING RELAY", "SIGNAL BLADE READY", "WATCH TOWER READY"],
      runtime_feedback_label: "GROUP 1 SECURING RELAY",
      upgrade_feedback_label: "SIGNAL BLADE READY",
      build_feedback_label: "WATCH TOWER READY",
      squad_role_labels: ["WORKER", "SCOUT", "GUARD", "RELAY"],
      order_queue_badge_labels: ["ATK BCN", "TRN WRK", "BLD RLY", "MOV 16/9"],
      completion_event_badge_labels: ["WRK READY", "SIG READY", "TWR READY", "TRN READY"],
      raw_marker_gate: true,
      feedback_expected_gate: true,
      feedback_width_gate: true,
      squad_strip_gate: true,
      squad_chip_width_gate: true,
      squad_chip_bottom_margin_px: 17,
      squad_chip_edge_clearance_gate: true,
      order_queue_badge_gate: true,
      selection_density_gate: true
    },
    first_contact_bottom_panel_readability_guard_gate: true,
    first_contact_silhouette_readability_contract: "trillionnium_world_bevy_classic_rts_first_contact_silhouette_readability_v1",
    first_contact_silhouette_readability_guard: {
      contract_version: "trillionnium_world_bevy_classic_rts_first_contact_silhouette_readability_v1",
      green: true,
      source_path: "trnm-world-bevy classic_draw_first_contact_silhouette_readability_layer",
      terrain_sample_tiles: ["8,8", "25,8", "25,25", "8,25", "12,16", "21,16", "16,9", "16,24", "16,16"],
      terrain_sample_roles: ["base_pad", "base_pad", "base_pad", "base_pad", "resource_zone", "resource_zone", "objective_lane", "objective_lane", "central_basin"],
      terrain_signatures: ["base_corner_frame", "base_corner_frame", "base_corner_frame", "base_corner_frame", "flux_glint_cluster", "flux_glint_cluster", "beacon_lane_rim", "beacon_lane_rim", "basin_cross_rim"],
      terrain_samples: [
        {tile: "8,8", role: "base_pad", signature: "base_corner_frame"},
        {tile: "25,8", role: "base_pad", signature: "base_corner_frame"},
        {tile: "25,25", role: "base_pad", signature: "base_corner_frame"},
        {tile: "8,25", role: "base_pad", signature: "base_corner_frame"},
        {tile: "12,16", role: "resource_zone", signature: "flux_glint_cluster"},
        {tile: "21,16", role: "resource_zone", signature: "flux_glint_cluster"},
        {tile: "16,9", role: "objective_lane", signature: "beacon_lane_rim"},
        {tile: "16,24", role: "objective_lane", signature: "beacon_lane_rim"},
        {tile: "16,16", role: "central_basin", signature: "basin_cross_rim"}
      ],
      terrain_zone_pixel_budget: 288,
      terrain_zone_gate: true,
      unit_sample_tiles: ["14,11", "15,11", "15,12", "17,12"],
      unit_roles: ["worker", "scout", "warden", "relay"],
      unit_signatures: ["cargo_pack", "sensor_mast", "shield_plate", "relay_courier"],
      unit_samples: [
        {tile: "14,11", role: "worker", signature: "cargo_pack"},
        {tile: "15,11", role: "scout", signature: "sensor_mast"},
        {tile: "15,12", role: "warden", signature: "shield_plate"},
        {tile: "17,12", role: "relay", signature: "relay_courier"}
      ],
      unit_silhouette_pixel_budget: 344,
      unit_role_silhouette_gate: true,
      structure_sample_tiles: ["8,8", "25,8", "25,25", "8,25", "11,8", "22,25", "16,9", "16,24", "9,16", "24,16"],
      structure_roles: ["command_core", "command_core", "command_core", "command_core", "relay", "relay", "beacon", "beacon", "beacon", "beacon"],
      structure_signatures: ["stepped_roof_core", "stepped_roof_core", "stepped_roof_core", "stepped_roof_core", "tall_signal_mast", "tall_signal_mast", "vertical_beacon_spire", "vertical_beacon_spire", "vertical_beacon_spire", "vertical_beacon_spire"],
      structure_samples: [
        {tile: "8,8", role: "command_core", signature: "stepped_roof_core"},
        {tile: "25,8", role: "command_core", signature: "stepped_roof_core"},
        {tile: "25,25", role: "command_core", signature: "stepped_roof_core"},
        {tile: "8,25", role: "command_core", signature: "stepped_roof_core"},
        {tile: "11,8", role: "relay", signature: "tall_signal_mast"},
        {tile: "22,25", role: "relay", signature: "tall_signal_mast"},
        {tile: "16,9", role: "beacon", signature: "vertical_beacon_spire"},
        {tile: "16,24", role: "beacon", signature: "vertical_beacon_spire"},
        {tile: "9,16", role: "beacon", signature: "vertical_beacon_spire"},
        {tile: "24,16", role: "beacon", signature: "vertical_beacon_spire"}
      ],
      command_core_silhouette_count: 4,
      relay_silhouette_count: 2,
      beacon_silhouette_count: 4,
      structure_roofline_pixel_budget: 960,
      beacon_spire_pixel_budget: 288,
      structure_roofline_gate: true,
      beacon_spire_gate: true,
      map_object_silhouette_gate: true
    },
    first_contact_silhouette_readability_guard_gate: true,
    first_contact_art_readability_contract: "trillionnium_world_bevy_classic_rts_first_contact_art_readability_v1",
    first_contact_art_readability_guard: {
      contract_version: "trillionnium_world_bevy_classic_rts_first_contact_art_readability_v1",
      green: true,
      source_path: "trnm-world-bevy classic_draw_first_contact_art_readability_layer",
      terrain_sample_tiles: ["8,8", "25,8", "25,25", "8,25", "12,16", "21,16", "16,9", "16,24", "16,16"],
      terrain_material_roles: ["base_concrete", "base_concrete", "base_concrete", "base_concrete", "resource_crystal", "resource_crystal", "beacon_lane", "beacon_lane", "basin_floor"],
      terrain_material_signatures: ["foundation_panel_seams", "foundation_panel_seams", "foundation_panel_seams", "foundation_panel_seams", "flux_crystal_shards", "flux_crystal_shards", "painted_lane_chevrons", "painted_lane_chevrons", "cracked_plaza_cross"],
      terrain_material_samples: [
        {tile: "8,8", role: "base_concrete", signature: "foundation_panel_seams"},
        {tile: "25,8", role: "base_concrete", signature: "foundation_panel_seams"},
        {tile: "25,25", role: "base_concrete", signature: "foundation_panel_seams"},
        {tile: "8,25", role: "base_concrete", signature: "foundation_panel_seams"},
        {tile: "12,16", role: "resource_crystal", signature: "flux_crystal_shards"},
        {tile: "21,16", role: "resource_crystal", signature: "flux_crystal_shards"},
        {tile: "16,9", role: "beacon_lane", signature: "painted_lane_chevrons"},
        {tile: "16,24", role: "beacon_lane", signature: "painted_lane_chevrons"},
        {tile: "16,16", role: "basin_floor", signature: "cracked_plaza_cross"}
      ],
      terrain_material_pixel_budget: 432,
      terrain_material_gate: true,
      terrain_material_depth_source_path: "trnm-world-bevy classic_draw_first_contact_terrain_material_depth_detail",
      terrain_material_depth_signatures: [
        "terrain_foundation_beveled_edges",
        "terrain_crystal_cast_shadows",
        "terrain_lane_recessed_rails",
        "terrain_basin_fracture_shadows"
      ],
      terrain_material_depth_sample_count: 9,
      terrain_material_depth_pixel_budget: 576,
      terrain_material_depth_gate: true,
      building_sample_tiles: ["8,8", "25,8", "25,25", "8,25", "11,8", "22,25", "16,9", "16,24", "9,16", "24,16"],
      building_roles: ["command_core", "command_core", "command_core", "command_core", "relay", "relay", "beacon", "beacon", "beacon", "beacon"],
      building_facade_signatures: ["lit_window_rows", "lit_window_rows", "lit_window_rows", "lit_window_rows", "antenna_band_panels", "antenna_band_panels", "glowing_spire_panels", "glowing_spire_panels", "glowing_spire_panels", "glowing_spire_panels"],
      building_facade_samples: [
        {tile: "8,8", role: "command_core", signature: "lit_window_rows"},
        {tile: "25,8", role: "command_core", signature: "lit_window_rows"},
        {tile: "25,25", role: "command_core", signature: "lit_window_rows"},
        {tile: "8,25", role: "command_core", signature: "lit_window_rows"},
        {tile: "11,8", role: "relay", signature: "antenna_band_panels"},
        {tile: "22,25", role: "relay", signature: "antenna_band_panels"},
        {tile: "16,9", role: "beacon", signature: "glowing_spire_panels"},
        {tile: "16,24", role: "beacon", signature: "glowing_spire_panels"},
        {tile: "9,16", role: "beacon", signature: "glowing_spire_panels"},
        {tile: "24,16", role: "beacon", signature: "glowing_spire_panels"}
      ],
      command_core_facade_count: 4,
      relay_facade_count: 2,
      beacon_facade_count: 4,
      building_facade_pixel_budget: 860,
      building_facade_gate: true,
      map_landmark_sample_tiles: ["8,9", "25,9", "25,24", "8,24", "12,16", "21,16", "16,10", "16,23", "14,14", "18,18", "11,9", "22,24", "16,9", "16,24", "9,16", "24,16"],
      map_landmark_roles: ["base_gate", "base_gate", "base_gate", "base_gate", "resource_cluster", "resource_cluster", "beacon_lane", "beacon_lane", "basin_scar", "basin_scar", "relay_cable", "relay_cable", "beacon_ring", "beacon_ring", "beacon_ring", "beacon_ring"],
      map_landmark_signatures: ["base_gate_lamps", "base_gate_lamps", "base_gate_lamps", "base_gate_lamps", "crystal_shadow_sparkles", "crystal_shadow_sparkles", "lane_power_pylons", "lane_power_pylons", "crater_scuff_marks", "crater_scuff_marks", "relay_ground_cables", "relay_ground_cables", "beacon_capture_rings", "beacon_capture_rings", "beacon_capture_rings", "beacon_capture_rings"],
      map_landmark_samples: [
        {tile: "8,9", role: "base_gate", signature: "base_gate_lamps"},
        {tile: "25,9", role: "base_gate", signature: "base_gate_lamps"},
        {tile: "25,24", role: "base_gate", signature: "base_gate_lamps"},
        {tile: "8,24", role: "base_gate", signature: "base_gate_lamps"},
        {tile: "12,16", role: "resource_cluster", signature: "crystal_shadow_sparkles"},
        {tile: "21,16", role: "resource_cluster", signature: "crystal_shadow_sparkles"},
        {tile: "16,10", role: "beacon_lane", signature: "lane_power_pylons"},
        {tile: "16,23", role: "beacon_lane", signature: "lane_power_pylons"},
        {tile: "14,14", role: "basin_scar", signature: "crater_scuff_marks"},
        {tile: "18,18", role: "basin_scar", signature: "crater_scuff_marks"},
        {tile: "11,9", role: "relay_cable", signature: "relay_ground_cables"},
        {tile: "22,24", role: "relay_cable", signature: "relay_ground_cables"},
        {tile: "16,9", role: "beacon_ring", signature: "beacon_capture_rings"},
        {tile: "16,24", role: "beacon_ring", signature: "beacon_capture_rings"},
        {tile: "9,16", role: "beacon_ring", signature: "beacon_capture_rings"},
        {tile: "24,16", role: "beacon_ring", signature: "beacon_capture_rings"}
      ],
      base_landmark_count: 4,
      resource_landmark_count: 2,
      lane_landmark_count: 2,
      basin_landmark_count: 2,
      relay_landmark_count: 2,
      beacon_landmark_count: 4,
      map_landmark_pixel_budget: 1152,
      map_landmark_detail_gate: true,
      runtime_actor_depth_source_path: "trnm-world-bevy classic_draw_first_contact_actor_glyph",
      runtime_actor_depth_signatures: [
        "runtime_structure_roof_rim",
        "runtime_structure_side_shadow",
        "runtime_command_window_pips",
        "runtime_relay_mast_braces",
        "runtime_beacon_core_glow_rungs"
      ],
      runtime_actor_depth_pixel_budget: 480,
      runtime_actor_depth_gate: true,
      lower_secondary_beacon_art_samples: [
        {tile: "16,24", role: "beacon_lane", signature: "painted_lane_chevrons"},
        {tile: "16,23", role: "beacon_lane", signature: "lane_power_pylons"},
        {tile: "16,24", role: "beacon_ring", signature: "beacon_capture_rings"}
      ],
      lower_secondary_beacon_art_sample_count: 3,
      lower_secondary_beacon_art_pixel_budget: 96,
      lower_secondary_beacon_art_signatures: [
        "lower_secondary_beacon_micro_chevrons",
        "lower_secondary_beacon_power_pylons_capped",
        "lower_secondary_beacon_capture_ring_reduced"
      ],
      lower_secondary_beacon_art_deemphasis_gate: true,
      authored_map_art_gate: true
    },
    first_contact_art_readability_guard_gate: true,
    first_contact_motion_readability_contract: "trillionnium_world_bevy_classic_rts_first_contact_motion_readability_v1",
    first_contact_motion_readability_guard: {
      contract_version: "trillionnium_world_bevy_classic_rts_first_contact_motion_readability_v1",
      green: true,
      source_path: "trnm-world-bevy classic_draw_first_contact_opening_actions + classic_draw_first_contact_unit_state_layers + classic_draw_first_contact_command_feedback_layers + classic_draw_first_contact_animation_readability_layer",
      active_relay_tile: "11,8",
      active_beacon_tile: "16,9",
      opening_action_ids: ["worker_harvest_flux", "build_flux_relay", "train_worker", "train_horizon_scout", "secure_flux_beacon"],
      action_verbs: ["worker", "build", "train", "train", "secure"],
      progress_meter_pixel_budget: 210,
      opening_action_gate: true,
      unit_status_badges: ["W", "S", "R", "G"],
      unit_status_color_roles: ["health", "mana", "attack", "confirm"],
      unit_status_pixel_budget: 256,
      unit_state_motion_signatures: [
        "unit_status_badges",
        "player_screen_production_training_micro_sparks",
        "player_screen_warden_attack_micro_sparks"
      ],
      production_training_spark_count: 3,
      production_training_spark_width_px: 4,
      production_training_spark_height_px: 2,
      production_training_spark_pixel_budget: 24,
      production_training_spark_gate: true,
      warden_attack_arm_count: 3,
      warden_attack_arm_width_px: 14,
      warden_attack_arm_height_px: 2,
      warden_attack_arm_pixel_budget: 84,
      warden_attack_arm_gate: true,
      player_screen_animation_signatures: [
        "player_screen_shield_charge_micro_arcs"
      ],
      shield_charge_arc_count: 3,
      shield_charge_arc_width_px: 10,
      shield_charge_arc_height_px: 2,
      shield_charge_arc_pixel_budget: 60,
      shield_charge_arc_gate: true,
      unit_state_motion_gate: true,
      track_roles: ["action_trail", "npc_action", "action_trail", "npc_action", "action_trail", "npc_action"],
      animation_sample_tiles: ["12,16", "10,12", "14,11", "25,8", "24,10", "8,25", "10,23", "11,8", "8,8", "9,9", "11,8", "16,9", "16,10"],
      animation_roles: ["worker", "worker", "worker", "scout", "scout", "warden", "warden", "relay", "command_core", "command_core", "relay_structure", "beacon", "beacon"],
      animation_signatures: ["harvest_tool_swing_frame", "carry_bob_frame", "locomotion_footfall_pair", "sensor_sweep_arc", "turn_arc_frame", "shield_charge_flash", "attack_recoil_ticks", "relay_packet_pulse", "training_tick_lane", "spawn_door_open_frame", "construction_spark_ladder", "capture_pulse_frame", "rally_flag_flutter"],
      animation_frame_richness_source_path: "trnm-world-bevy classic_draw_first_contact_animation_frame_richness_detail",
      animation_frame_richness_signatures: [
        "animation_secondary_pose_offsets",
        "animation_contact_smear_ticks",
        "animation_structure_shutter_frames",
        "animation_objective_afterglow_frames"
      ],
      animation_frame_richness_sample_count: 13,
      animation_frame_richness_pixel_budget: 520,
      animation_samples: [
        {tile: "12,16", role: "worker", signature: "harvest_tool_swing_frame"},
        {tile: "10,12", role: "worker", signature: "carry_bob_frame"},
        {tile: "14,11", role: "worker", signature: "locomotion_footfall_pair"},
        {tile: "25,8", role: "scout", signature: "sensor_sweep_arc"},
        {tile: "24,10", role: "scout", signature: "turn_arc_frame"},
        {tile: "8,25", role: "warden", signature: "shield_charge_flash"},
        {tile: "10,23", role: "warden", signature: "attack_recoil_ticks"},
        {tile: "11,8", role: "relay", signature: "relay_packet_pulse"},
        {tile: "8,8", role: "command_core", signature: "training_tick_lane"},
        {tile: "9,9", role: "command_core", signature: "spawn_door_open_frame"},
        {tile: "11,8", role: "relay_structure", signature: "construction_spark_ladder"},
        {tile: "16,9", role: "beacon", signature: "capture_pulse_frame"},
        {tile: "16,10", role: "beacon", signature: "rally_flag_flutter"}
      ],
      unit_animation_frame_count: 8,
      building_animation_frame_count: 3,
      objective_animation_frame_count: 2,
      unique_animation_signature_count: 13,
      animation_frame_pixel_budget: 1144,
      unit_animation_frame_gate: true,
      building_animation_frame_gate: true,
      objective_animation_frame_gate: true,
      animation_cycle_detail_gate: true,
      animation_frame_richness_gate: true,
      track_samples: [
        {from_tile: "8,8", to_tile: "12,16", role: "action_trail"},
        {from_tile: "25,25", to_tile: "21,16", role: "npc_action"},
        {from_tile: "25,8", to_tile: "16,12", role: "action_trail"},
        {from_tile: "8,25", to_tile: "16,21", role: "npc_action"},
        {from_tile: "11,8", to_tile: "16,9", role: "action_trail"},
        {from_tile: "22,25", to_tile: "16,24", role: "npc_action"}
      ],
      action_trail_count: 3,
      npc_action_count: 3,
      primary_tactical_track_count: 1,
      secondary_tactical_track_count: 5,
      primary_tactical_track_pixel_budget: 48,
      secondary_tactical_track_pixel_budget: 80,
      secondary_tactical_track_height_px: 1,
      secondary_tactical_track_darken_numerator: 3,
      secondary_tactical_track_darken_denominator: 4,
      tactical_track_density_signatures: [
        "primary_relay_beacon_track_kept_hot",
        "secondary_tracks_dimmed",
        "secondary_tracks_faded_into_terrain",
        "secondary_tracks_one_pixel"
      ],
      tactical_track_pixel_budget: 128,
      tactical_track_motion_gate: true,
      feedback_selected_group: "GROUP 1",
      feedback_active_order: "SECURE BEACON",
      feedback_player_labels: [
        "GROUP 1 SECURING BEACON",
        "QUEUE ADDED  ORDER READY",
        "ROUTE BLOCKED MID VENT"
      ],
      feedback_target_tile: "16,9",
      feedback_queued_before: 2,
      feedback_queued_after: 3,
      feedback_command_ack_progress: 86,
      feedback_cooldown_progress: 32,
      feedback_raw_marker_gate: true,
      feedback_player_label_gate: true,
      feedback_pixel_budget: 190,
      command_feedback_motion_signatures: [
        "feedback_player_labels",
        "battlefield_command_feedback_micro_trail"
      ],
      feedback_move_trail_origin_count: 2,
      feedback_move_trail_step_count_per_origin: 10,
      feedback_move_trail_tick_count: 20,
      feedback_move_trail_tick_width_px: 2,
      feedback_move_trail_tick_height_px: 2,
      feedback_move_trail_pixel_budget: 80,
      feedback_battlefield_trail_gate: true,
      command_feedback_motion_gate: true,
      walk_cycle_frame: 2,
      combat_turn: 3,
      route_tile_ids: ["14,11", "15,11", "16,10", "16,9"],
      command_destination_tile: "16,9",
      attack_target_id: "trnm.flux.beacon",
      combat_event_log: ["guard_attack_windup", "worker_carry_supply", "creep_counter_swing", "secure_beacon:16,9"],
      training_progress_percent: 64,
      build_progress_percent: 42,
      runtime_motion_gate: true
    },
    first_contact_motion_readability_guard_gate: true,
    first_contact_atlas_readability_contract: "trillionnium_world_bevy_classic_rts_first_contact_atlas_readability_v1",
    first_contact_atlas_readability_guard: {
      contract_version: "trillionnium_world_bevy_classic_rts_first_contact_atlas_readability_v1",
      green: true,
      source_path: "trnm-world-bevy classic_draw_first_contact_atlas_readability_layer",
      asset_pack_contract: "trillionnium_world_bevy_classic_asset_pack_v1",
      asset_boundary: "project_owned_manifest_ppm_atlas_for_classic_low_spec_renderer_not_cex_runtime",
      atlas_parse_gate: true,
      sample_tiles: ["8,8", "25,8", "12,16", "21,16", "16,9", "16,16", "14,11", "15,11", "15,12", "17,12", "8,8", "25,8", "11,8", "22,25", "16,9", "16,24"],
      atlas_roles: ["terrain_tile", "terrain_tile", "terrain_tile", "terrain_tile", "terrain_tile", "terrain_tile", "unit_sprite", "unit_sprite", "unit_sprite", "unit_sprite", "structure_sprite", "structure_sprite", "structure_sprite", "structure_sprite", "objective_sprite", "objective_sprite"],
      atlas_frame_ids: ["tile_stone", "tile_stone", "tile_water", "tile_water", "tile_road", "tile_floor", "actor_player_walk_south_1", "actor_player_walk_east_1", "actor_enemy_attack", "actor_mentor_talk", "prop_workbench", "prop_market_stall", "prop_signpost", "prop_banner", "marker_objective", "marker_interaction"],
      atlas_manifest_roles: ["stone_tile", "stone_tile", "water_tile", "water_tile", "road_tile", "interior_tile", "player_actor", "player_actor", "enemy_actor", "npc_actor", "scene_prop", "scene_prop", "scene_prop", "scene_prop", "objective_marker", "interaction_marker"],
      atlas_signatures: ["base_pad_stone_frame", "base_pad_stone_frame", "flux_pool_ripple_frame", "flux_pool_ripple_frame", "beacon_lane_tile_frame", "basin_floor_tile_frame", "worker_walk_atlas_frame", "scout_stride_atlas_frame", "warden_attack_atlas_frame", "relay_operator_atlas_frame", "command_core_workbench_frame", "command_core_stall_frame", "relay_mast_signpost_frame", "relay_banner_frame", "beacon_objective_atlas_frame", "beacon_interaction_atlas_frame"],
      atlas_samples: [
        {tile: "8,8", role: "terrain_tile", frame_id: "tile_stone", signature: "base_pad_stone_frame", scale: 1},
        {tile: "25,8", role: "terrain_tile", frame_id: "tile_stone", signature: "base_pad_stone_frame", scale: 1},
        {tile: "12,16", role: "terrain_tile", frame_id: "tile_water", signature: "flux_pool_ripple_frame", scale: 1},
        {tile: "21,16", role: "terrain_tile", frame_id: "tile_water", signature: "flux_pool_ripple_frame", scale: 1},
        {tile: "16,9", role: "terrain_tile", frame_id: "tile_road", signature: "beacon_lane_tile_frame", scale: 1},
        {tile: "16,16", role: "terrain_tile", frame_id: "tile_floor", signature: "basin_floor_tile_frame", scale: 1},
        {tile: "14,11", role: "unit_sprite", frame_id: "actor_player_walk_south_1", signature: "worker_walk_atlas_frame", scale: 2},
        {tile: "15,11", role: "unit_sprite", frame_id: "actor_player_walk_east_1", signature: "scout_stride_atlas_frame", scale: 2},
        {tile: "15,12", role: "unit_sprite", frame_id: "actor_enemy_attack", signature: "warden_attack_atlas_frame", scale: 2},
        {tile: "17,12", role: "unit_sprite", frame_id: "actor_mentor_talk", signature: "relay_operator_atlas_frame", scale: 2},
        {tile: "8,8", role: "structure_sprite", frame_id: "prop_workbench", signature: "command_core_workbench_frame", scale: 2},
        {tile: "25,8", role: "structure_sprite", frame_id: "prop_market_stall", signature: "command_core_stall_frame", scale: 2},
        {tile: "11,8", role: "structure_sprite", frame_id: "prop_signpost", signature: "relay_mast_signpost_frame", scale: 2},
        {tile: "22,25", role: "structure_sprite", frame_id: "prop_banner", signature: "relay_banner_frame", scale: 2},
        {tile: "16,9", role: "objective_sprite", frame_id: "marker_objective", signature: "beacon_objective_atlas_frame", scale: 2},
        {tile: "16,24", role: "objective_sprite", frame_id: "marker_interaction", signature: "beacon_interaction_atlas_frame", scale: 2}
      ],
      secondary_objective_atlas_samples: [
        {tile: "16,24", role: "objective_sprite", frame_id: "marker_interaction", signature: "beacon_interaction_atlas_frame", scale: 2}
      ],
      secondary_objective_atlas_sample_count: 1,
      secondary_objective_atlas_source_frame_pixel_budget: 1024,
      secondary_objective_atlas_rendered_frame_pixel_budget: 0,
      secondary_objective_atlas_anchor_pixel_budget: 24,
      secondary_objective_atlas_signatures: [
        "secondary_objective_atlas_frame_suppressed",
        "secondary_objective_atlas_micro_anchor_only"
      ],
      atlas_family_sample_tiles: ["4,14", "4,16", "4,18", "4,20", "29,14", "29,16", "29,18", "4,4", "6,4", "27,4", "29,4", "29,22", "29,24", "29,26"],
      atlas_family_roles: ["worker_unit_family", "worker_unit_family", "scout_unit_family", "scout_unit_family", "warden_unit_family", "warden_unit_family", "relay_unit_family", "command_core_structure_family", "command_core_structure_family", "relay_structure_family", "relay_structure_family", "beacon_objective_family", "beacon_objective_family", "beacon_objective_family"],
      atlas_family_frame_ids: ["actor_worker_idle", "actor_worker_carry", "actor_player_walk_east_1", "actor_player_walk_east_2", "actor_guard_idle", "actor_guard_attack", "actor_mentor_talk", "model_town_hall", "model_training_hall", "model_waygate", "prop_banner", "marker_objective", "rts_command_destination_marker", "marker_interaction"],
      atlas_family_manifest_roles: ["override_frame", "override_frame", "player_actor", "player_actor", "override_frame", "override_frame", "npc_actor", "override_frame", "override_frame", "override_frame", "scene_prop", "objective_marker", "override_frame", "interaction_marker"],
      atlas_family_override_frame_ids: ["actor_worker_idle", "actor_worker_carry", "actor_player_walk_east_1", "actor_player_walk_east_2", "actor_guard_idle", "actor_guard_attack", "actor_mentor_talk", "model_town_hall", "model_training_hall", "model_waygate", "prop_banner", "marker_objective", "rts_command_destination_marker", "marker_interaction"],
      atlas_family_signatures: ["worker_idle_frame_family", "worker_carry_frame_family", "scout_stride_east_frame_family", "scout_stride_east_alt_frame_family", "warden_guard_idle_frame_family", "warden_guard_attack_frame_family", "relay_operator_talk_frame_family", "command_core_town_hall_frame_family", "command_core_training_hall_frame_family", "relay_waygate_frame_family", "relay_banner_frame_family", "beacon_objective_frame_family", "beacon_destination_marker_frame_family", "beacon_interaction_frame_family"],
      atlas_family_samples: [
        {tile: "4,14", role: "worker_unit_family", frame_id: "actor_worker_idle", signature: "worker_idle_frame_family", scale: 1},
        {tile: "4,16", role: "worker_unit_family", frame_id: "actor_worker_carry", signature: "worker_carry_frame_family", scale: 1},
        {tile: "4,18", role: "scout_unit_family", frame_id: "actor_player_walk_east_1", signature: "scout_stride_east_frame_family", scale: 2},
        {tile: "4,20", role: "scout_unit_family", frame_id: "actor_player_walk_east_2", signature: "scout_stride_east_alt_frame_family", scale: 2},
        {tile: "29,14", role: "warden_unit_family", frame_id: "actor_guard_idle", signature: "warden_guard_idle_frame_family", scale: 1},
        {tile: "29,16", role: "warden_unit_family", frame_id: "actor_guard_attack", signature: "warden_guard_attack_frame_family", scale: 1},
        {tile: "29,18", role: "relay_unit_family", frame_id: "actor_mentor_talk", signature: "relay_operator_talk_frame_family", scale: 1},
        {tile: "4,4", role: "command_core_structure_family", frame_id: "model_town_hall", signature: "command_core_town_hall_frame_family", scale: 1},
        {tile: "6,4", role: "command_core_structure_family", frame_id: "model_training_hall", signature: "command_core_training_hall_frame_family", scale: 1},
        {tile: "27,4", role: "relay_structure_family", frame_id: "model_waygate", signature: "relay_waygate_frame_family", scale: 1},
        {tile: "29,4", role: "relay_structure_family", frame_id: "prop_banner", signature: "relay_banner_frame_family", scale: 1},
        {tile: "29,22", role: "beacon_objective_family", frame_id: "marker_objective", signature: "beacon_objective_frame_family", scale: 1},
        {tile: "29,24", role: "beacon_objective_family", frame_id: "rts_command_destination_marker", signature: "beacon_destination_marker_frame_family", scale: 1},
        {tile: "29,26", role: "beacon_objective_family", frame_id: "marker_interaction", signature: "beacon_interaction_frame_family", scale: 1}
      ],
      atlas_family_gallery_lanes: ["west_gallery", "west_gallery", "west_gallery", "west_gallery", "east_gallery", "east_gallery", "east_gallery", "north_gallery", "north_gallery", "north_gallery", "north_gallery", "east_gallery", "east_gallery", "east_gallery"],
      atlas_family_busy_core_tiles: [],
      terrain_frame_count: 6,
      unit_frame_count: 4,
      structure_frame_count: 4,
      objective_frame_count: 2,
      worker_family_frame_count: 2,
      scout_family_frame_count: 2,
      warden_family_frame_count: 2,
      relay_unit_family_frame_count: 1,
      command_core_family_frame_count: 2,
      relay_structure_family_frame_count: 2,
      beacon_family_frame_count: 3,
      unique_frame_count: 14,
      unique_signature_count: 14,
      atlas_family_unique_frame_count: 14,
      atlas_family_unique_signature_count: 14,
      atlas_family_unique_gallery_lane_count: 3,
      north_gallery_frame_count: 4,
      west_gallery_frame_count: 4,
      east_gallery_frame_count: 6,
      atlas_frame_pixel_budget: 11776,
      atlas_family_frame_pixel_budget: 42496,
      atlas_runtime_depth_source_path: "trnm-world-bevy classic_draw_first_contact_atlas_asset_sample",
      atlas_runtime_depth_signatures: [
        "atlas_unit_grounding_shadow",
        "atlas_structure_footprint_rim",
        "atlas_objective_capture_underlay",
        "atlas_lower_lane_depth_suppressed"
      ],
      atlas_runtime_depth_sample_count: 21,
      atlas_lower_lane_depth_suppressed_count: 3,
      atlas_runtime_depth_pixel_budget: 1344,
      manifest_frame_gate: true,
      family_frame_available_gate: true,
      atlas_manifest_gate: true,
      terrain_atlas_frame_gate: true,
      unit_atlas_sprite_gate: true,
      structure_atlas_sprite_gate: true,
      objective_atlas_sprite_gate: true,
      worker_frame_family_gate: true,
      scout_frame_family_gate: true,
      warden_frame_family_gate: true,
      relay_unit_frame_family_gate: true,
      command_core_frame_family_gate: true,
      relay_structure_frame_family_gate: true,
      beacon_frame_family_gate: true,
      override_frame_family_gate: true,
      atlas_family_perimeter_placement_gate: true,
      atlas_composition_gate: true,
      atlas_frame_family_gate: true,
      atlas_runtime_depth_gate: true,
      secondary_objective_atlas_deemphasis_gate: true,
      no_copy_boundary_gate: true,
      first_contact_atlas_readability_gate: true,
      warcraft_iii_asset_copied: false,
      openra_asset_copied: false,
      third_party_asset_copied: false
    },
    first_contact_atlas_readability_guard_gate: true,
    first_contact_visual_hierarchy_contract: "trillionnium_world_bevy_classic_rts_first_contact_visual_hierarchy_v1",
    first_contact_visual_hierarchy_guard: {
      contract_version: "trillionnium_world_bevy_classic_rts_first_contact_visual_hierarchy_v1",
      green: true,
      source_path: "trnm-world-bevy classic_draw_first_contact_visual_hierarchy_layer between readability overlays and selection/combat focus",
      corridor_tiles: ["14,11", "15,11", "15,12", "15,16", "16,9", "16,10", "17,12"],
      selected_focus_tiles: ["14,11", "15,11", "15,12", "17,12"],
      route_focus_tiles: ["14,11", "15,11", "16,10", "16,9"],
      target_focus_tile: "16,9",
      blocked_focus_tile: "15,16",
      route_line_step_count: 10,
      atlas_family_gallery_lanes: ["west_gallery", "west_gallery", "west_gallery", "west_gallery", "east_gallery", "east_gallery", "east_gallery", "north_gallery", "north_gallery", "north_gallery", "north_gallery", "east_gallery", "east_gallery", "east_gallery"],
      atlas_family_busy_core_tiles: [],
      unique_gallery_lanes: ["east_gallery", "north_gallery", "west_gallery"],
      hierarchy_signatures: ["route_corridor_micro_edge_cues", "route_spine_micro_shadow_cues", "selected_halo_backplates", "attack_target_micro_backplate", "blocked_warning_micro_backplate", "perimeter_gallery_preserved"],
      corridor_deemphasis_cue_width_px: 8,
      corridor_deemphasis_cue_height_px: 2,
      corridor_deemphasis_cues_per_tile: 2,
      corridor_deemphasis_pixel_budget: 224,
      route_spine_shadow_cue_width_px: 6,
      route_spine_shadow_cue_height_px: 2,
      route_spine_shadow_pixel_budget: 120,
      selected_halo_pixel_budget: 384,
      target_backplate_pixel_budget: 96,
      blocked_backplate_pixel_budget: 72,
      corridor_tile_gate: true,
      route_spine_gate: true,
      selected_halo_gate: true,
      combat_backplate_gate: true,
      gallery_preservation_gate: true,
      hierarchy_signature_gate: true,
      hierarchy_layer_draw_order: ["terrain", "actors", "model_identity", "silhouette_readability", "art_readability", "animation_readability", "atlas_readability", "unit_state", "combat_phase", "command_feedback", "runtime_core", "tactical_tracks", "opening_actions", "readability_overlays", "visual_hierarchy_deemphasis", "central_clarity_deemphasis", "terminal_legibility_deemphasis", "selection_combat_focus"],
      hierarchy_layer_order_gate: true,
      visual_hierarchy_gate: true
    },
    first_contact_visual_hierarchy_guard_gate: true,
    first_contact_central_clarity_contract: "trillionnium_world_bevy_classic_rts_first_contact_central_clarity_v1",
    first_contact_central_clarity_guard: {
      contract_version: "trillionnium_world_bevy_classic_rts_first_contact_central_clarity_v1",
      green: true,
      source_path: "trnm-world-bevy classic_draw_first_contact_central_clarity_layer between visual hierarchy and selection/combat focus",
      central_core_tile_count: 18,
      central_quiet_tile_count: 13,
      central_focus_tile_count: 5,
      quiet_tiles: ["13,10", "14,10", "15,10", "17,10", "18,10", "13,11", "16,11", "17,11", "18,11", "13,12", "14,12", "16,12", "18,12"],
      focus_corridor_tiles: ["14,11", "15,11", "15,12", "15,16", "16,9", "16,10", "17,12"],
      focus_overlap_tiles: [],
      central_quiet_cue_width_px: 6,
      central_quiet_cue_height_px: 2,
      central_quiet_cues_per_tile: 2,
      quiet_tile_fill_pixel_budget: 0,
      quiet_tile_pixel_budget: 312,
      quiet_edge_pixel_budget: 156,
      clarity_signatures: ["central_negative_space_micro_edge_cues", "focus_corridor_not_muted", "quiet_edge_micro_cues", "selection_focus_still_last"],
      clarity_layer_draw_order: ["terrain", "actors", "model_identity", "silhouette_readability", "art_readability", "animation_readability", "atlas_readability", "unit_state", "combat_phase", "command_feedback", "runtime_core", "tactical_tracks", "opening_actions", "readability_overlays", "visual_hierarchy_deemphasis", "central_clarity_deemphasis", "terminal_legibility_deemphasis", "selection_combat_focus"],
      central_quiet_tile_gate: true,
      focus_overlap_gate: true,
      quiet_edge_gate: true,
      clarity_signature_gate: true,
      clarity_layer_order_gate: true,
      central_clarity_gate: true
    },
    first_contact_central_clarity_guard_gate: true,
    first_contact_terminal_legibility_contract: "trillionnium_world_bevy_classic_rts_first_contact_terminal_legibility_v1",
    first_contact_terminal_legibility_guard: {
      contract_version: "trillionnium_world_bevy_classic_rts_first_contact_terminal_legibility_v1",
      green: true,
      source_path: "trnm-world-bevy classic_draw_first_contact_terminal_legibility_layer between central clarity and selection/combat focus",
      terminal_quiet_tile_count: 13,
      target_quiet_tiles: ["15,8", "16,8", "17,8", "15,9", "17,9"],
      blocked_quiet_tiles: ["14,15", "15,15", "16,15", "14,16", "16,16", "14,17", "15,17", "16,17"],
      terminal_focus_tiles: ["15,16", "16,9", "16,10"],
      focus_overlap_tiles: [],
      target_focus_tile: "16,9",
      blocked_focus_tile: "15,16",
      terminal_quiet_cue_width_px: 6,
      terminal_quiet_cue_height_px: 2,
      terminal_quiet_cues_per_tile: 2,
      target_quiet_fill_pixel_budget: 0,
      blocked_quiet_fill_pixel_budget: 0,
      target_quiet_pixel_budget: 120,
      blocked_quiet_pixel_budget: 192,
      target_edge_pixel_budget: 60,
      blocked_edge_pixel_budget: 96,
      terminal_signatures: ["target_terminal_micro_edge_cues", "blocked_terminal_micro_edge_cues", "route_terminal_focus_preserved", "focus_markers_still_last"],
      terminal_layer_draw_order: ["terrain", "actors", "model_identity", "silhouette_readability", "art_readability", "animation_readability", "atlas_readability", "unit_state", "combat_phase", "command_feedback", "runtime_core", "tactical_tracks", "opening_actions", "readability_overlays", "visual_hierarchy_deemphasis", "central_clarity_deemphasis", "terminal_legibility_deemphasis", "selection_combat_focus"],
      target_terminal_quiet_gate: true,
      blocked_terminal_quiet_gate: true,
      terminal_focus_preservation_gate: true,
      terminal_edge_budget_gate: true,
      terminal_signature_gate: true,
      terminal_layer_order_gate: true,
      terminal_legibility_gate: true
    },
    first_contact_terminal_legibility_guard_gate: true,
    first_contact_selection_combat_focus_contract: "trillionnium_world_bevy_classic_rts_first_contact_selection_combat_focus_v1",
    first_contact_selection_combat_focus_guard: {
      contract_version: "trillionnium_world_bevy_classic_rts_first_contact_selection_combat_focus_v1",
      green: true,
      source_path: "trnm-world-bevy classic_draw_first_contact_selection_combat_focus_layer after atlas/motion/readability overlays",
      selected_focus_tiles: ["14,11", "15,11", "15,12", "17,12"],
      route_focus_tiles: ["14,11", "15,11", "16,10", "16,9"],
      route_clearance_tiles: ["13,11", "14,10", "14,12", "15,9", "15,10", "16,8", "16,11", "17,9", "17,10"],
      route_clearance_overlap_tiles: [],
      target_focus_tile: "16,9",
      blocked_focus_tile: "15,16",
      focus_signatures: ["selected_corner_brackets", "selected_role_badge_micro_pips", "compact_route_dashes", "route_ack_step_micro_dots", "route_ack_micro_dots", "route_clearance_corner_cues", "attack_target_lock_brackets", "compact_target_lock_cross", "target_ack_micro_tick", "blocked_warning_cross"],
      route_dash_count: 4,
      route_ack_tick_count: 6,
      route_line_step_count: 10,
      route_dash_width_px: 8,
      route_dash_height_px: 2,
      route_ack_tick_width_px: 2,
      route_ack_tick_height_px: 2,
      selected_role_badge_tick_width_px: 2,
      selected_role_badge_tick_height_px: 2,
      selected_role_badge_tick_pixel_budget: 16,
      selected_focus_pixel_budget: 272,
      route_dash_pixel_budget: 64,
      route_ack_tick_pixel_budget: 24,
      route_focus_pixel_budget: 88,
      route_clearance_corner_cue_count: 36,
      route_clearance_corner_cue_width_px: 6,
      route_clearance_corner_cue_height_px: 2,
      route_clearance_corner_cues_per_tile: 4,
      route_clearance_corner_cue_pixel_budget: 432,
      route_clearance_gutter_fill_pixel_budget: 0,
      route_clearance_pixel_budget: 432,
      combat_target_cross_long_px: 28,
      combat_target_cross_thickness_px: 3,
      combat_target_ack_tick_width_px: 4,
      combat_target_ack_tick_height_px: 2,
      combat_target_cross_pixel_budget: 168,
      combat_target_ack_tick_pixel_budget: 8,
      combat_target_pixel_budget: 176,
      blocked_warning_pixel_budget: 84,
      selected_focus_gate: true,
      route_focus_gate: true,
      route_clearance_gate: true,
      combat_target_focus_gate: true,
      blocked_warning_focus_gate: true,
      focus_signature_gate: true,
      focus_layer_draw_order: ["terrain", "actors", "model_identity", "silhouette_readability", "art_readability", "animation_readability", "atlas_readability", "unit_state", "combat_phase", "command_feedback", "runtime_core", "tactical_tracks", "opening_actions", "readability_overlays", "visual_hierarchy_deemphasis", "central_clarity_deemphasis", "terminal_legibility_deemphasis", "selection_combat_focus"],
      focus_layer_order_gate: true,
      selection_combat_focus_readability_gate: true
    },
    first_contact_selection_combat_focus_guard_gate: true,
    first_contact_target_callout_contract: "trillionnium_world_bevy_classic_rts_first_contact_target_callout_v1",
    first_contact_target_callout_guard: {
      contract_version: "trillionnium_world_bevy_classic_rts_first_contact_target_callout_v1",
      green: true,
      source_path: "trnm-world-bevy classic_draw_first_contact_selection_combat_focus_layer compact target callout plate inside final focus layer",
      target_tile: "16,9",
      target_subject: "BEACON",
      target_label: "BEACON 38%",
      target_label_width_px: 60,
      target_health_percent: 38,
      target_health_fill_px: 20,
      target_callout_width_px: 78,
      target_callout_height_px: 20,
      target_callout_x_offset_px: 42,
      target_callout_y_offset_px: -42,
      target_callout_health_bar_width_px: 54,
      target_callout_health_bar_height_px: 3,
      target_callout_pixel_budget: 1560,
      target_callout_leader_pixel_budget: 64,
      target_callout_clearance_pad_px: 0,
      target_callout_leader_clearance_width_px: 0,
      target_callout_leader_clearance_height_px: 0,
      target_callout_clearance_pixel_budget: 0,
      target_prefocus_ring_count: 2,
      target_prefocus_ring_thickness_px: 2,
      target_prefocus_cross_long_px: 16,
      target_prefocus_cross_thickness_px: 2,
      target_prefocus_marker_pixel_budget: 256,
      target_callout_signatures: ["compact_target_callout_plate", "target_subject_label", "target_health_strip", "short_leader_ticks", "prefocus_target_rings_capped", "prefocus_target_cross_thinned", "target_lock_preserved"],
      target_callout_layer_draw_order: ["terrain", "actors", "model_identity", "silhouette_readability", "art_readability", "animation_readability", "atlas_readability", "unit_state", "combat_phase", "command_feedback", "runtime_core", "tactical_tracks", "opening_actions", "readability_overlays", "visual_hierarchy_deemphasis", "central_clarity_deemphasis", "terminal_legibility_deemphasis", "selection_combat_focus"],
      target_label_gate: true,
      target_health_gate: true,
      target_geometry_gate: true,
      target_leader_gate: true,
      target_callout_clearance_gate: true,
      target_prefocus_marker_gate: true,
      target_signature_gate: true,
      target_layer_order_gate: true,
      target_callout_gate: true
    },
    first_contact_target_callout_guard_gate: true,
    first_contact_marker_budget_contract: "trillionnium_world_bevy_classic_rts_first_contact_marker_budget_v1",
    first_contact_marker_budget_guard: {
      contract_version: "trillionnium_world_bevy_classic_rts_first_contact_marker_budget_v1",
      green: true,
      source_path: "trnm-world-bevy muted First Contact atlas gallery presentation plus player-screen tactical/hover marker budgets and final selection/combat focus layer",
      gallery_sample_count: 14,
      muted_gallery_sample_count: 14,
      gallery_lanes: ["west_gallery", "west_gallery", "west_gallery", "west_gallery", "east_gallery", "east_gallery", "east_gallery", "north_gallery", "north_gallery", "north_gallery", "north_gallery", "east_gallery", "east_gallery", "east_gallery"],
      busy_core_tiles: [],
      lower_lane_gallery_tiles: ["29,22", "29,24", "29,26"],
      north_gallery_frame_count: 4,
      west_gallery_frame_count: 4,
      east_gallery_frame_count: 6,
      max_gallery_lane_frame_count: 6,
      gallery_mute_overlay_pixel_budget: 21248,
      gallery_slot_cue_pixel_budget: 14,
      lower_lane_gallery_sample_count: 3,
      lower_lane_rendered_frame_pixel_budget: 0,
      lower_lane_frame_suppressed_count: 3,
      lower_lane_mute_overlay_pixel_budget: 0,
      lower_lane_slot_cue_pixel_budget: 3,
      lower_lane_ghost_anchor_count: 3,
      lower_lane_gallery_darken_numerator: 5,
      lower_lane_gallery_darken_denominator: 6,
      lower_lane_dim_silhouette_pixel_budget: 0,
      lower_lane_shadow_suppressed_count: 3,
      gallery_hot_marker_color_count: 0,
      lower_lane_hot_marker_color_count: 0,
      interactive_hot_marker_role_count: 5,
      tactical_track_count: 6,
      player_screen_tactical_track_count: 1,
      player_screen_primary_tactical_track_count: 1,
      player_screen_secondary_tactical_track_count: 0,
      hover_route_path_marker_count: 8,
      player_screen_hover_route_path_marker_count: 0,
      non_focus_owner_identity_colors: ["457953", "6a5e4b"],
      non_focus_owner_identity_hot_color_count: 0,
      non_focus_owner_identity_pixel_budget: 192,
      player_screen_relay_identity_rail_count: 6,
      player_screen_relay_identity_rail_width_px: 16,
      player_screen_relay_identity_rail_height_px: 2,
      player_screen_relay_identity_rail_pixel_budget: 192,
      player_screen_legacy_midfield_status_bar_pixel_budget: 0,
      selected_focus_tiles: ["14,11", "15,11", "15,12", "17,12"],
      route_focus_tiles: ["14,11", "15,11", "16,10", "16,9"],
      selected_role_badge_tick_pixel_budget: 16,
      interactive_focus_pixel_budget: 636,
      gallery_presentation_signatures: ["muted_gallery_slot_cues", "perimeter_gallery_edge_anchors", "perimeter_gallery_single_pixel_anchors", "darkened_gallery_frames", "lower_lane_gallery_deemphasis", "lower_lane_micro_slot_cues", "lower_lane_frame_suppressed", "lower_lane_anchor_only", "lower_lane_shadow_suppressed", "lower_lane_ghost_anchors", "lower_lane_single_point_ghost_anchors", "compact_route_dashes", "route_ack_micro_dots", "perimeter_gallery_lane_budget", "interactive_focus_kept_hot", "non_focus_owner_identity_muted", "player_screen_primary_tactical_track_only", "player_screen_hover_route_path_suppressed", "player_screen_relay_identity_micro_rails", "player_screen_legacy_midfield_status_bars_suppressed"],
      marker_budget_layer_draw_order: ["atlas_readability", "atlas_gallery_muted", "visual_hierarchy_deemphasis", "central_clarity_deemphasis", "terminal_legibility_deemphasis", "selection_combat_focus"],
      gallery_lane_budget_gate: true,
      gallery_mute_gate: true,
      lower_lane_gallery_deemphasis_gate: true,
      interactive_focus_preservation_gate: true,
      non_focus_owner_identity_gate: true,
      player_screen_tactical_track_gate: true,
      player_screen_hover_route_path_gate: true,
      player_screen_relay_identity_rail_gate: true,
      player_screen_legacy_midfield_status_bar_gate: true,
      marker_budget_layer_order_gate: true,
      first_contact_marker_budget_gate: true
    },
    first_contact_marker_budget_guard_gate: true,
    first_contact_runtime_core_visibility: {
      green: true,
      source_path: "trnm-world-bevy classic_draw_first_contact_runtime_core_layer player-visible actor subset with control and semantic fixtures hidden, inline health bars decluttered, default selection rings suppressed, and unit role accents muted",
      runtime_core_visible_tile_y_max: 28,
      runtime_core_actor_count: 83,
      runtime_core_visible_actor_count: 14,
      runtime_core_inline_health_bar_actor_count: 4,
      runtime_core_suppressed_inline_health_bar_actor_count: 10,
      runtime_core_progress_bar_actor_count: 2,
      runtime_core_selection_ring_candidate_actor_count: 14,
      runtime_core_selection_ring_visible_actor_count: 0,
      runtime_core_suppressed_selection_ring_actor_count: 14,
      runtime_core_unit_role_accent_candidate_actor_count: 9,
      runtime_core_unit_role_accent_visible_actor_count: 0,
      runtime_core_suppressed_unit_role_accent_actor_count: 9,
      runtime_core_hidden_fixture_actor_count: 69,
      runtime_core_hidden_control_fixture_actor_count: 48,
      runtime_core_hidden_semantic_fixture_actor_count: 20,
      runtime_core_visible_actor_ids: ["map.actor0", "map.actor15", "multi0.command.core", "multi0.damaged.relay", "multi0.flux.relay", "multi0.guard.sentinel", "multi0.line.0", "multi0.scout.intel", "multi0.scout.spotter", "multi0.striker.0", "multi0.warden.capture", "multi0.worker.0", "multi0.worker.1", "multi0.worker.repair"],
      runtime_core_inline_health_bar_actor_ids: ["multi0.damaged.relay", "multi0.line.0", "multi0.scout.intel", "multi0.warden.capture"],
      runtime_core_progress_bar_actor_ids: ["map.actor15", "multi0.worker.1"],
      runtime_core_selection_ring_candidate_actor_ids: ["map.actor0", "map.actor15", "multi0.command.core", "multi0.damaged.relay", "multi0.flux.relay", "multi0.guard.sentinel", "multi0.line.0", "multi0.scout.intel", "multi0.scout.spotter", "multi0.striker.0", "multi0.warden.capture", "multi0.worker.0", "multi0.worker.1", "multi0.worker.repair"],
      runtime_core_selection_ring_visible_actor_ids: [],
      runtime_core_unit_role_accent_candidate_actor_ids: ["multi0.guard.sentinel", "multi0.line.0", "multi0.scout.intel", "multi0.scout.spotter", "multi0.striker.0", "multi0.warden.capture", "multi0.worker.0", "multi0.worker.1", "multi0.worker.repair"],
      runtime_core_unit_role_accent_visible_actor_ids: [],
      runtime_core_hidden_fixture_actor_ids: ["multi0.append.runner", "multi0.append.seed", "multi0.append.wing", "multi0.assignment.runner", "multi0.assignment.wing", "multi0.attackmove.warden", "multi0.capture.contested.warden", "multi0.chain.runner", "multi0.clear.seed", "multi0.clear.wing", "multi0.focus.warden.a", "multi0.focus.warden.b", "multi0.formation.blocked.lead", "multi0.formation.blocked.left", "multi0.formation.blocked.right", "multi0.formation.lead", "multi0.formation.left", "multi0.formation.prune.runner", "multi0.formation.right", "multi0.formation.slot.blocker", "multi0.formation.validation.runner", "multi0.group.validation.runner", "multi0.obstruction.blocker", "multi0.obstruction.follower", "multi0.obstruction.leader", "multi0.override.runner", "multi0.patrol.warden", "multi0.priority.guard", "multi0.queue.prune.runner", "multi0.queue.reject.runner", "multi0.reassignment.runner", "multi0.reassignment.stale", "multi0.reassignment.wing", "multi0.rebuild.old.seed", "multi0.rebuild.old.wing", "multi0.rebuild.runner", "multi0.rebuild.wing", "multi0.recall.formation.old.seed", "multi0.recall.formation.old.wing", "multi0.recall.formation.runner", "multi0.recall.formation.wing", "multi0.recall.order.old.seed", "multi0.recall.order.old.wing", "multi0.recall.order.runner", "multi0.recall.order.wing", "multi0.recall.override.old.seed", "multi0.recall.override.old.wing", "multi0.recall.override.runner", "multi0.recall.override.wing", "multi0.recall.rebuild.old.seed", "multi0.recall.rebuild.old.wing", "multi0.recall.rebuild.runner", "multi0.recall.rebuild.wing", "multi0.remove.runner", "multi0.remove.seed", "multi0.remove.wing", "multi0.reservation.lead", "multi0.reservation.wing", "multi0.stance.aggressive", "multi0.stance.guard", "multi0.stance.holdfire", "multi0.stance.prune.runner", "multi0.stance.spotter", "multi0.stop.warden", "multi0.traffic.lead", "multi0.traffic.stuck.blocker", "multi0.traffic.stuck.runner", "multi0.traffic.yield", "multi0.veteran.warden"],
      runtime_core_hidden_control_fixture_actor_ids: ["multi0.append.runner", "multi0.append.seed", "multi0.append.wing", "multi0.assignment.runner", "multi0.assignment.wing", "multi0.chain.runner", "multi0.clear.seed", "multi0.clear.wing", "multi0.formation.blocked.lead", "multi0.formation.blocked.left", "multi0.formation.blocked.right", "multi0.formation.lead", "multi0.formation.left", "multi0.formation.prune.runner", "multi0.formation.right", "multi0.formation.slot.blocker", "multi0.formation.validation.runner", "multi0.group.validation.runner", "multi0.override.runner", "multi0.queue.prune.runner", "multi0.queue.reject.runner", "multi0.reassignment.runner", "multi0.reassignment.stale", "multi0.reassignment.wing", "multi0.rebuild.old.seed", "multi0.rebuild.old.wing", "multi0.rebuild.runner", "multi0.rebuild.wing", "multi0.recall.formation.old.seed", "multi0.recall.formation.old.wing", "multi0.recall.formation.runner", "multi0.recall.formation.wing", "multi0.recall.order.old.seed", "multi0.recall.order.old.wing", "multi0.recall.order.runner", "multi0.recall.order.wing", "multi0.recall.override.old.seed", "multi0.recall.override.old.wing", "multi0.recall.override.runner", "multi0.recall.override.wing", "multi0.recall.rebuild.old.seed", "multi0.recall.rebuild.old.wing", "multi0.recall.rebuild.runner", "multi0.recall.rebuild.wing", "multi0.remove.runner", "multi0.remove.seed", "multi0.remove.wing", "multi0.stance.prune.runner"],
      runtime_core_hidden_semantic_fixture_actor_ids: ["multi0.attackmove.warden", "multi0.capture.contested.warden", "multi0.focus.warden.a", "multi0.focus.warden.b", "multi0.obstruction.blocker", "multi0.obstruction.follower", "multi0.obstruction.leader", "multi0.patrol.warden", "multi0.priority.guard", "multi0.reservation.lead", "multi0.reservation.wing", "multi0.stance.aggressive", "multi0.stance.guard", "multi0.stance.spotter", "multi0.stop.warden", "multi0.traffic.lead", "multi0.traffic.stuck.blocker", "multi0.traffic.stuck.runner", "multi0.traffic.yield", "multi0.veteran.warden"],
      runtime_core_hidden_fixture_tile_ids: ["1,2", "1,27", "1,30", "1,4", "11,20", "11,31", "13,31", "17,25", "17,29", "17,30", "18,25", "18,29", "18,30", "18,31", "19,25", "19,29", "19,30", "2,2", "2,27", "2,3", "2,30", "2,31", "20,25", "20,30", "21,16", "21,29", "22,30", "23,18", "23,27", "24,27", "24,29", "25,27", "26,27", "26,29", "27,16", "27,27", "28,25", "28,27", "28,29", "28,30", "29,12", "29,25", "29,27", "3,14", "3,18", "3,2", "3,29", "3,30", "3,31", "30,2", "30,27", "30,29", "30,30", "31,27", "31,28", "31,29", "31,31", "32,29", "4,4", "4,6", "5,20", "5,4", "6,6", "8,2", "9,2"],
      runtime_core_required_visible_actor_ids: ["multi0.command.core", "multi0.worker.0", "multi0.flux.relay", "map.actor15"],
      runtime_core_required_visible_gate: true,
      runtime_core_hidden_fixture_gate: true,
      runtime_core_control_fixture_gate: true,
      runtime_core_semantic_fixture_gate: true,
      runtime_core_visible_subset_gate: true,
      runtime_core_bottom_fixture_gate: true,
      runtime_core_inline_health_bar_gate: true,
      runtime_core_selection_ring_gate: true,
      runtime_core_unit_role_accent_gate: true
    },
    first_contact_runtime_core_visibility_gate: true,
    rts_data_player_screen_layout_gate: true,
    rts_data_player_screen_chrome_gate: true,
    rts_data_player_screen_gate: true,
    rts_bevy_runtime_player_screen_application_contract: "trnm_rts_bevy_runtime_first_contact_player_screen_application_v1",
    rts_bevy_runtime_player_screen_application: {
      contract_version: "trnm_rts_bevy_runtime_first_contact_player_screen_application_v1",
      green: true,
      profile_contract: "trnm_rts_data_first_contact_player_screen_v1",
      map_scene: "first_contact_basin",
      current_room_id: "first-contact-basin",
      camera_focus_tile_id: "16,16",
      command_destination_tile_id: "16,9",
      command_queue: ["move:16,9", "build:trnm.flux.relay", "train:trnm.worker", "attack:trnm.flux.beacon"],
      production_queue: ["train:guard", "train:worker", "upgrade:signal_blade"],
      build_queue: ["build:watch_tower", "upgrade:training_hall"],
      ability_command_ids: ["worker", "scout", "warden", "relay", "core", "signal"],
      visible_tile_ids: [range(0;64) | "visible_fixture_tile"],
      group_route_tile_ids: ["11,8", "13,8", "16,9"],
      profile_application_gate: true,
      command_surface_seed_gate: true,
      route_surface_seed_gate: true,
      runtime_application_path: "trnm-rts-data first_contact_player_screen_profile -> trnm-rts-bevy-runtime player_screen_runtime_application -> NativeFirstPlayableRuntime mutation",
      source_of_truth: "This Bevy-free runtime application translates the trnm-rts-data First Contact player-screen profile into room, camera, command queue, production/build queues, visibility, selection, route, ability, supply, and objective runtime fields before the Bevy adapter mutates NativeFirstPlayableRuntime."
    },
    rts_bevy_runtime_player_screen_application_gate: true,
    rts_evidence_bevy_runtime_adapter: {
      contract_version: "trnm_rts_evidence_bevy_runtime_adapter_v1",
      runtime_contract: "trnm_rts_bevy_runtime_adapter_v1",
      green: true,
      first_contact_online_protocol_fixture: {
        contract_version: "trnm_rts_online_first_contact_fixture_v1",
        green: true,
        envelope: {
          map_id: "first_contact_basin"
        }
      },
      first_contact_online_local_handoff: {
        contract_version: "trnm_rts_online_local_handoff_v1",
        green: true,
        accepted_order_count: 1
      },
      first_contact_online_offline_adapter: {
        contract_version: "trnm_rts_online_offline_adapter_v1",
        green: true,
        local_runtime_handoff: {
          accepted_runtime_command_labels: ["move:8,4"]
        }
      },
      first_contact_online_protocol_gate: true,
      first_contact_online_local_handoff_gate: true,
      first_contact_online_offline_adapter_gate: true,
      first_contact_map_model_review: {
        map_summary: {
          actor_count: 39,
          spawn_count: 4,
          flux_bloom_count: 11,
          beacon_count: 4,
          expansion_count: 4,
          source_integration_mode: "gpl_internal_component"
        },
        unit_rule_count: 6,
        building_rule_count: 5,
        data_validation_error: null,
        map_actor_gate: true,
        map_topology_gate: true,
        rules_gate: true,
        data_consumer_gate: true,
        map_model_adapter_gate: true
      },
      first_contact_map_model_gate: true,
      first_contact_opening_profile: {
        contract_version: "trnm_rts_data_first_contact_opening_profile_v1",
        map_id: "first_contact_basin",
        active_beacon_tile: {x: 16, y: 9},
        active_relay_tile: {x: 11, y: 8}
      },
      first_contact_opening_profile_gate: true,
      first_contact_command_feedback_profile: {
        contract_version: "trnm_rts_data_first_contact_command_feedback_v1",
        map_id: "first_contact_basin",
        target_tile: {x: 16, y: 9},
        blocked_tile: {x: 15, y: 16}
      },
      first_contact_command_feedback_gate: true,
      first_contact_player_startup_profiles: [
        {contract_version: "trnm_rts_data_first_contact_player_startup_v1", map_id: "first_contact_basin", player_id: "Multi0", faction: "horizon", spawn_tile: {x: 8, y: 8}, faction_unit_rule_id: "trnm.horizon.scout"},
        {contract_version: "trnm_rts_data_first_contact_player_startup_v1", map_id: "first_contact_basin", player_id: "Multi1", faction: "forge", spawn_tile: {x: 25, y: 25}, faction_unit_rule_id: "trnm.forge.warden"},
        {contract_version: "trnm_rts_data_first_contact_player_startup_v1", map_id: "first_contact_basin", player_id: "Multi2", faction: "horizon", spawn_tile: {x: 25, y: 8}, faction_unit_rule_id: "trnm.horizon.scout"},
        {contract_version: "trnm_rts_data_first_contact_player_startup_v1", map_id: "first_contact_basin", player_id: "Multi3", faction: "forge", spawn_tile: {x: 8, y: 25}, faction_unit_rule_id: "trnm.forge.warden"}
      ],
      first_contact_player_startup_gate: true,
      first_contact_actor_presentation_profiles: [
        {contract_version: "trnm_rts_data_first_contact_actor_presentation_v1", map_id: "first_contact_basin", rule_id: "mpspawn", color_role: "map_detail", glyph_role: "map_detail", structure: false, selectable: false, glyph: {contract_version: "trnm_rts_data_first_contact_actor_glyph_v1", body: "spawn_pad", accent: "owner_stripe", footprint_width_cells: 3}},
        {contract_version: "trnm_rts_data_first_contact_actor_presentation_v1", map_id: "first_contact_basin", rule_id: "trnm.worker", color_role: "worker", glyph_role: "worker", structure: false, selectable: true, glyph: {contract_version: "trnm_rts_data_first_contact_actor_glyph_v1", body: "unit", accent: "worker_cargo", selection_ring: true}},
        {contract_version: "trnm_rts_data_first_contact_actor_presentation_v1", map_id: "first_contact_basin", rule_id: "trnm.command.core", color_role: "command_core", glyph_role: "command_core", structure: true, health_bar_width: 36, glyph: {contract_version: "trnm_rts_data_first_contact_actor_glyph_v1", body: "structure", accent: "command_spire", footprint_width_cells: 2}},
        {contract_version: "trnm_rts_data_first_contact_actor_presentation_v1", map_id: "first_contact_basin", rule_id: "trnm.flux.beacon", color_role: "objective", glyph_role: "beacon", structure: true, glyph: {contract_version: "trnm_rts_data_first_contact_actor_glyph_v1", body: "objective_beacon", accent: "beacon_core"}}
      ],
      first_contact_actor_presentation_gate: true,
      first_contact_visual_telemetry_profile: {
        contract_version: "trnm_rts_data_first_contact_visual_telemetry_v1",
        map_id: "first_contact_basin",
        unit_statuses: [{tile: {x: 8, y: 8}, role_badge: "W", role_color: "health", health_percent: 82, shield_percent: 44}],
        tactical_tracks: [{from_tile: {x: 11, y: 8}, to_tile: {x: 16, y: 9}, color_role: "action_trail"}]
      },
      first_contact_visual_telemetry_gate: true,
      first_contact_preview_actor_projection: {
        actor_count: 39,
        spawn_count: 4,
        flux_bloom_count: 11,
        beacon_count: 4,
        expansion_count: 4,
        actor_samples: [
          {
            contract_version: "trnm_rts_data_first_contact_preview_actor_v1",
            source_actor_id: "Actor0",
            kind: "spawn",
            owner: "Multi0",
            tile: {x: 8, y: 8},
            source_rule_id: "mpspawn",
            openra_preview_rule_id: "trnm.map.detail"
          },
          {
            contract_version: "trnm_rts_data_first_contact_preview_actor_v1",
            source_actor_id: "Actor2",
            kind: "flux_bloom",
            owner: "Neutral",
            tile: {x: 12, y: 16},
            source_rule_id: "trnm.flux.bloom",
            openra_preview_rule_id: "trnm.flux.bloom"
          },
          {
            contract_version: "trnm_rts_data_first_contact_preview_actor_v1",
            source_actor_id: "Actor5",
            kind: "spawn",
            owner: "Multi2",
            tile: {x: 25, y: 8},
            source_rule_id: "mpspawn",
            openra_preview_rule_id: "trnm.map.detail"
          }
        ],
        source: "trnm-rts-data first_contact_preview_actors projection from RtsMapModel actors"
      },
      first_contact_preview_actor_projection_gate: true,
      first_contact_player_screen_profile: {
        contract_version: "trnm_rts_data_first_contact_player_screen_v1",
        map_id: "first_contact_basin",
        room_id: "first-contact-basin",
        layout: {
          player_map: {map_origin_x: 16},
          spec_map: {map_origin_x: 24}
        },
        chrome: {
          command_grid_slot_ids: ["worker", "scout", "warden", "relay", "core", "signal"]
        },
        command_queue: ["move:16,9", "build:trnm.flux.relay", "train:trnm.worker", "attack:trnm.flux.beacon"],
        production_queue: ["train:guard", "train:worker", "upgrade:signal_blade"],
        build_queue: ["build:watch_tower", "upgrade:training_hall"]
      },
      first_contact_player_screen_layout_gate: true,
      first_contact_player_screen_chrome_gate: true,
      first_contact_player_screen_profile_gate: true,
      first_contact_terrain_profile_count: 1156,
      first_contact_terrain_profile_samples: {
        border: {role: "border", height: 0, base_pad: false, resource_zone: false},
        lane: {role: "lane", height: 1, base_pad: false, resource_zone: false},
        center: {role: "central_basin", height: 2, base_pad: false, resource_zone: false},
        base_pad: {role: "base_pad", height: 1, base_pad: true, resource_zone: false},
        resource_zone: {role: "resource_zone", height: 1, base_pad: false, resource_zone: true}
      },
      first_contact_terrain_profile_gate: true,
      first_contact_renderer_projection: {
        renderable_tile_count: 1024,
        lane_tile_count: 240,
        resource_zone_tile_count: 79,
        base_pad_tile_count: 144,
        minimap_anchor_actor_count: 39,
        resource_actor_tile_count: 11,
        objective_actor_tile_count: 4,
        spawn_actor_tile_count: 4,
        lane_tile_samples: [{x: 16, y: 1}],
        resource_actor_tile_samples: [{x: 12, y: 16}],
        objective_actor_tile_samples: [{x: 16, y: 9}],
        spawn_actor_tile_samples: [{x: 8, y: 8}],
        minimap_anchor_actor_samples: ["Actor0"],
        source: "RtsMapModel bounds, terrain profiles, actor rules, and runtime projection math"
      },
      first_contact_renderer_projection_gate: true,
      first_contact_runtime_map_projection: {
        map_x: 16,
        map_y: 54,
        cell_w: 28,
        cell_h: 14,
        map_w: 952,
        map_h: 476
      },
      first_contact_runtime_tile_rect_sample: {
        x: 464,
        y: 278,
        width: 28,
        height: 14
      },
      first_contact_runtime_terrain_seed_sample: {
        surface_seed: 12,
        detail_seed: 20
      },
      first_contact_runtime_map_projection_gate: true,
      first_contact_player_screen_application: {
        contract_version: "trnm_rts_bevy_runtime_first_contact_player_screen_application_v1",
        green: true
      },
      first_contact_offline_adapter_application: {
        contract_version: "trnm_rts_bevy_runtime_first_contact_offline_adapter_runtime_application_v1",
        green: true,
        command_queue: ["move:8,4"]
      },
      first_contact_offline_adapter_consumption_review: {
        contract_version: "trnm_rts_bevy_runtime_first_contact_offline_adapter_consumption_v1",
        green: true,
        runtime_command_stamp_tile_id: "8,4",
        runtime_application: {
          command_queue: ["move:8,4"]
        }
      },
      first_contact_offline_adapter_session_transition_review: {
        contract_version: "trnm_rts_bevy_runtime_first_contact_offline_adapter_session_transition_v1",
        green: true,
        after_command_queue: ["move:8,4"]
      },
      first_contact_offline_adapter_lobby_ready_review: {
        contract_version: "trnm_rts_bevy_runtime_first_contact_offline_adapter_lobby_ready_v1",
        green: true,
        ready_state_labels: ["player:local-player:ready", "player:mirror_guard:ready", "bot:mirror_guard:ready", "authority:offline_loopback:no_socket"]
      },
      first_contact_player_screen_application_contract: "trnm_rts_bevy_runtime_first_contact_player_screen_application_v1",
      first_contact_player_screen_application_green: true,
      first_contact_offline_adapter_application_contract: "trnm_rts_bevy_runtime_first_contact_offline_adapter_runtime_application_v1",
      first_contact_offline_adapter_application_green: true,
      first_contact_offline_adapter_consumption_contract: "trnm_rts_bevy_runtime_first_contact_offline_adapter_consumption_v1",
      first_contact_offline_adapter_consumption_green: true,
      first_contact_offline_adapter_session_transition_contract: "trnm_rts_bevy_runtime_first_contact_offline_adapter_session_transition_v1",
      first_contact_offline_adapter_session_transition_green: true,
      first_contact_offline_adapter_lobby_ready_contract: "trnm_rts_bevy_runtime_first_contact_offline_adapter_lobby_ready_v1",
      first_contact_offline_adapter_lobby_ready_green: true,
      first_contact_runtime_review_contracts: [
        "trnm_rts_bevy_runtime_first_contact_player_screen_application_v1",
        "trnm_rts_bevy_runtime_first_contact_offline_adapter_runtime_application_v1",
        "trnm_rts_bevy_runtime_first_contact_offline_adapter_consumption_v1",
        "trnm_rts_bevy_runtime_first_contact_offline_adapter_session_transition_v1",
        "trnm_rts_bevy_runtime_first_contact_offline_adapter_lobby_ready_v1"
      ],
      first_contact_runtime_review_before_command_queue_sample: ["move:16,9", "build:trnm.flux.relay", "train:trnm.worker", "attack:trnm.flux.beacon"],
      first_contact_runtime_review_after_command_queue_sample: ["move:8,4"],
      first_contact_runtime_review_ready_state_labels_sample: ["player:local-player:ready", "player:mirror_guard:ready", "bot:mirror_guard:ready", "authority:offline_loopback:no_socket"],
      first_contact_runtime_review_command_stamp_tile_sample: "8,4",
      first_contact_runtime_review_gate: true
    },
    rts_evidence_bevy_runtime_adapter_gate: true,
    rts_bevy_runtime_map_projection: {
      map_x: 16,
      map_y: 54,
      cell_w: 28,
      cell_h: 14,
      map_w: 952,
      map_h: 476
    },
    rts_bevy_runtime_tile_rect_sample: {
      x: 464,
      y: 278,
      width: 28,
      height: 14
    },
    rts_bevy_runtime_terrain_seed_sample: {
      surface_seed: 12,
      detail_seed: 20
    },
    rts_bevy_runtime_map_projection_gate: true,
    rts_online_protocol_fixture: {
      transport: {
        contract_version: "trnm_rts_online_loopback_transport_v1",
        green: true,
        socket_opened: false,
        hosted_service_claimed: false,
        public_launch_ready: false
      }
    },
    rts_online_protocol_gate: true,
    rts_online_local_handoff_contract: "trnm_rts_online_local_handoff_v1",
    rts_online_local_handoff: {
      contract_version: "trnm_rts_online_local_handoff_v1",
      handoff_id: "first-contact-local-loopback-handoff",
      map_id: "first_contact_basin",
      player_id: "mirror_guard",
      green: true,
      handoff_ready: true,
      accepted_order_count: 1,
      rejected_order_count: 1,
      scoped_update_count: 1,
      bot_count: 1,
      visible_chunk_count: 3,
      visible_actor_count: 4,
      server_authoritative: true,
      visibility_scoped_response: true,
      socket_opened: false,
      hosted_service_claimed: false,
      public_launch_ready: false
    },
    rts_online_local_handoff_gate: true,
    rts_online_offline_adapter_contract: "trnm_rts_online_offline_adapter_v1",
    rts_online_offline_adapter_local_replay_contract: "trnm_rts_online_offline_adapter_local_replay_v1",
    rts_online_offline_adapter_runtime_handoff_contract: "trnm_rts_online_offline_adapter_runtime_handoff_v1",
    rts_online_offline_adapter: {
      contract_version: "trnm_rts_online_offline_adapter_v1",
      adapter_id: "first-contact-offline-loopback-adapter",
      handoff_id: "first-contact-local-loopback-handoff",
      map_id: "first_contact_basin",
      adapter_mode: "offline_loopback_authority",
      connected_player_ids: ["local-player", "mirror_guard"],
      bot_player_ids: ["mirror_guard"],
      input_queue_labels: ["client:move_worker@8,4", "client:attack_fogged_keep"],
      accepted_server_order_labels: ["client:move_worker@8,4"],
      rejected_client_order_reasons: ["target_actor_not_visible"],
      scoped_update_actor_ids: ["trnm.worker.alpha", "trnm.horizon.scout.alpha", "trnm.command.core.alpha", "trnm.flux.beacon.center"],
      scoped_update_order_count: 1,
      frame_sha256s: ["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb", "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"],
      local_action_replay: {
        contract_version: "trnm_rts_online_offline_adapter_local_replay_v1",
        replay_mode: "bevy_local_ui_action_replay",
        accepted_action_labels: ["RTS:SELECT:26", "RTS:MOVE:18,31:line", "RTS:SELECT:27", "RTS:MOVE:21,25:line", "RTS:SELECT:28", "RTS:MOVE:1,31:line", "RTS:SELECT:26"],
        accepted_preview_stages: ["group_26_queued", "group_27_override", "group_28_formation", "cleared_history_bounded"],
        blocked_action_labels: ["RTS:MOVE:18,31:line", "RTS:MOVE:bad-tile:line", "RTS:ATTACK:", "RTS:ABILITY:guard_break", "RTS:QUEUE:", "RTS:QUEUE:build:watch_tower@7,4", "RTS:SELECT:"],
        blocked_input_sources: ["classic_rts_mouse_viewport", "classic_rts_mouse_viewport", "classic_rts_mouse_viewport", "classic_rts_hotkey", "classic_rts_mouse_sidebar", "classic_rts_mouse_sidebar", "classic_rts_hotkey"],
        blocked_reasons: ["rts_group_selection_required", "rts_invalid_tile:bad-tile", "rts_attack_target_required", "rts_attack_required_before_ability", "rts_queue_id_required", "rts_queue_unaffordable:build:watch_tower@7,4", "rts_group_id_required"],
        blocked_preview_stages: ["group_selection_required", "invalid_tile", "attack_target_required", "history_preserved_after_rejections"],
        retained_history_group_ids: ["26", "27", "28"],
        pruned_history_group_ids: ["25", "24"],
        command_history_capacity: 3,
        local_input_sources_ready: true,
        command_history_ready: true,
        green: true
      },
      local_runtime_handoff: {
        contract_version: "trnm_rts_online_offline_adapter_runtime_handoff_v1",
        handoff_mode: "server_authoritative_runtime_command_handoff",
        accepted_runtime_command_labels: ["move:8,4"],
        accepted_runtime_destination_tile_ids: ["8,4"],
        accepted_runtime_subject_actor_ids: ["trnm.worker.alpha"],
        rejected_runtime_command_labels: ["client:attack_fogged_keep"],
        scoped_update_actor_ids: ["trnm.worker.alpha", "trnm.horizon.scout.alpha", "trnm.command.core.alpha", "trnm.flux.beacon.center"],
        runtime_control_group_id: "1",
        runtime_group_command_state: "offline_adapter_authority_applied",
        runtime_pathing_status: "offline_adapter_replay_consumed",
        runtime_unit_response_state: "server_authoritative_move_applied",
        runtime_command_stamp_source: "trnm-rts-online:offline_loopback_authority",
        runtime_command_stamp_kind: "server_accepted_move",
        runtime_command_stamp_tile_id: "8,4",
        runtime_command_stamp_player_label: "SERVER ACCEPTED MOVE 8,4",
        runtime_last_feedback: "Offline adapter applied server move 8,4; rejected target_actor_not_visible",
        accepted_order_runtime_ready: true,
        rejected_order_runtime_ready: true,
        scoped_update_runtime_ready: true,
        no_socket_boundary_ready: true,
        green: true
      },
      local_multiplayer_ready: true,
      offline_bot_ready: true,
      bevy_adapter_ready: true,
      server_authoritative: true,
      visibility_scoped_response: true,
      client_prediction_claimed: false,
      rollback_netcode_claimed: false,
      socket_opened: false,
      hosted_service_claimed: false,
      public_launch_ready: false,
      green: true
    },
    rts_online_offline_adapter_gate: true,
    rts_online_offline_adapter_consumption: {
      contract_version: "trnm_rts_bevy_runtime_first_contact_offline_adapter_consumption_v1",
      green: true,
      adapter_contract: "trnm_rts_online_offline_adapter_v1",
      adapter_runtime_handoff_contract: "trnm_rts_online_offline_adapter_runtime_handoff_v1",
      adapter_id: "first-contact-offline-loopback-adapter",
      adapter_mode: "offline_loopback_authority",
      adapter_runtime_handoff: {
        contract_version: "trnm_rts_online_offline_adapter_runtime_handoff_v1",
        handoff_mode: "server_authoritative_runtime_command_handoff",
        accepted_runtime_command_labels: ["move:8,4"],
        accepted_runtime_destination_tile_ids: ["8,4"],
        accepted_runtime_subject_actor_ids: ["trnm.worker.alpha"],
        rejected_runtime_command_labels: ["client:attack_fogged_keep"],
        scoped_update_actor_ids: ["trnm.worker.alpha", "trnm.horizon.scout.alpha", "trnm.command.core.alpha", "trnm.flux.beacon.center"],
        runtime_control_group_id: "1",
        runtime_group_command_state: "offline_adapter_authority_applied",
        runtime_pathing_status: "offline_adapter_replay_consumed",
        runtime_unit_response_state: "server_authoritative_move_applied",
        runtime_command_stamp_source: "trnm-rts-online:offline_loopback_authority",
        runtime_command_stamp_kind: "server_accepted_move",
        runtime_command_stamp_tile_id: "8,4",
        runtime_command_stamp_player_label: "SERVER ACCEPTED MOVE 8,4",
        runtime_last_feedback: "Offline adapter applied server move 8,4; rejected target_actor_not_visible",
        accepted_order_runtime_ready: true,
        rejected_order_runtime_ready: true,
        scoped_update_runtime_ready: true,
        no_socket_boundary_ready: true,
        green: true
      },
      runtime_application_contract: "trnm_rts_bevy_runtime_first_contact_offline_adapter_runtime_application_v1",
      runtime_application: {
        contract_version: "trnm_rts_bevy_runtime_first_contact_offline_adapter_runtime_application_v1",
        green: true,
        handoff_contract: "trnm_rts_online_offline_adapter_runtime_handoff_v1",
        handoff_mode: "server_authoritative_runtime_command_handoff",
        runtime_control_group_id: "1",
        selected_unit_ids: ["trnm.worker.alpha"],
        command_queue: ["move:8,4"],
        command_destination_tile_id: "8,4",
        group_route_tile_ids: ["8,4"],
        rejected_runtime_command_labels: ["client:attack_fogged_keep"],
        scoped_update_actor_ids: ["trnm.worker.alpha", "trnm.horizon.scout.alpha", "trnm.command.core.alpha", "trnm.flux.beacon.center"],
        runtime_group_command_state: "offline_adapter_authority_applied",
        runtime_pathing_status: "offline_adapter_replay_consumed",
        runtime_unit_response_state: "server_authoritative_move_applied",
        runtime_command_stamp_source: "trnm-rts-online:offline_loopback_authority",
        runtime_command_stamp_kind: "server_accepted_move",
        runtime_command_stamp_tile_id: "8,4",
        runtime_command_stamp_player_label: "SERVER ACCEPTED MOVE 8,4",
        runtime_last_feedback: "Offline adapter applied server move 8,4; rejected target_actor_not_visible",
        accepted_order_runtime_gate: true,
        rejected_order_runtime_gate: true,
        scoped_update_runtime_gate: true,
        no_socket_boundary_gate: true,
        runtime_application_path: "trnm-rts-bevy-runtime offline_adapter_runtime_application -> NativeFirstPlayableRuntime mutation",
        source_of_truth: "This Bevy-free runtime application translates the trnm-rts-online offline adapter handoff into the command queue, selected actors, route tile, command stamp, pathing/group response state, and feedback that the Bevy adapter mutates onto NativeFirstPlayableRuntime."
      },
      input_queue_labels: ["client:move_worker@8,4", "client:attack_fogged_keep"],
      accepted_server_order_labels: ["client:move_worker@8,4"],
      accepted_runtime_command_labels: ["move:8,4"],
      accepted_runtime_destination_tile_ids: ["8,4"],
      accepted_runtime_subject_actor_ids: ["trnm.worker.alpha"],
      rejected_client_order_reasons: ["target_actor_not_visible"],
      rejected_runtime_command_labels: ["client:attack_fogged_keep"],
      rejected_commands_suppressed: true,
      scoped_update_actor_ids: ["trnm.worker.alpha", "trnm.horizon.scout.alpha", "trnm.command.core.alpha", "trnm.flux.beacon.center"],
      runtime_control_group_id: "1",
      runtime_group_command_state: "offline_adapter_authority_applied",
      runtime_pathing_status: "offline_adapter_replay_consumed",
      runtime_unit_response_state: "server_authoritative_move_applied",
      runtime_command_stamp_source: "trnm-rts-online:offline_loopback_authority",
      runtime_command_stamp_kind: "server_accepted_move",
      runtime_command_stamp_tile_id: "8,4",
      runtime_command_stamp_player_label: "SERVER ACCEPTED MOVE 8,4",
      runtime_last_feedback: "Offline adapter applied server move 8,4; rejected target_actor_not_visible",
      runtime_player_screen_review: {
        map_scene: "first_contact_basin",
        current_room_id: "first-contact-basin",
        coins: 890,
        xp: 92,
        camera_focus_tile_id: "16,16",
        visibility_percent: 76,
        army_supply_used: 12,
        army_supply_cap: 22,
        objective_status: "secure first relay beacon and hold the center lane",
        production_queue: ["train:guard", "train:worker", "upgrade:signal_blade"],
        build_queue: ["build:watch_tower", "upgrade:training_hall"],
        selected_unit_ids: ["trnm.worker.alpha"],
        command_queue: ["move:8,4"],
        command_destination_tile_id: "8,4",
        group_route_tile_ids: ["8,4"],
        visible_tile_count: 64,
        fogged_tile_count: 6,
        selection_box_tile_count: 4,
        unit_health_percents: [96, 78, 71, 34],
        ability_command_ids: ["worker", "scout", "warden", "relay", "core", "signal"],
        ability_cooldown_percents: [0, 0, 16, 0, 42, 25],
        active_ability_id: "worker"
      },
      local_session_handoff_gate: true,
      runtime_application_gate: true,
      player_screen_review_gate: true,
      accepted_order_runtime_gate: true,
      rejected_order_runtime_gate: true,
      scoped_update_runtime_gate: true,
      no_network_claim_gate: true,
      server_authoritative: true,
      visibility_scoped_response: true,
      client_prediction_claimed: false,
      rollback_netcode_claimed: false,
      socket_opened: false,
      hosted_service_claimed: false,
      public_launch_ready: false,
      input_path: "trnm-rts-online offline adapter review input -> trnm-rts-bevy-runtime runtime application -> Bevy local player-screen snapshot",
      runtime_path: "trnm-rts-bevy-runtime offline_adapter_runtime_application + first_contact_offline_adapter_consumption_review -> NativeFirstPlayableRuntime consumer",
      source_of_truth: "This Bevy-free runtime review consumes the no-socket offline adapter through a trnm-rts-online-owned review input, a Bevy-free runtime application, and a local player-screen/session surface snapshot: the server-authoritative move reaches the visible command queue, route overlay, and command stamp while room, camera, visibility, queues, supply, and objective state stay coherent and the fogged attack rejection is suppressed from UI/action replay state."
    },
    rts_online_offline_adapter_consumption_gate: true,
    rts_online_offline_adapter_session_transition_contract: "trnm_rts_bevy_runtime_first_contact_offline_adapter_session_transition_v1",
    rts_online_offline_adapter_session_transition: {
      contract_version: "trnm_rts_bevy_runtime_first_contact_offline_adapter_session_transition_v1",
      green: true,
      initial_application_contract: "trnm_rts_bevy_runtime_first_contact_player_screen_application_v1",
      runtime_application_contract: "trnm_rts_bevy_runtime_first_contact_offline_adapter_runtime_application_v1",
      handoff_contract: "trnm_rts_online_offline_adapter_runtime_handoff_v1",
      map_scene: "first_contact_basin",
      current_room_id: "first-contact-basin",
      camera_focus_tile_id: "16,16",
      before_command_queue: ["move:16,9", "build:trnm.flux.relay", "train:trnm.worker", "attack:trnm.flux.beacon"],
      after_command_queue: ["move:8,4"],
      before_route_tile_ids: ["11,8", "13,8", "16,9"],
      after_route_tile_ids: ["8,4"],
      before_command_destination_tile_id: "16,9",
      after_command_destination_tile_id: "8,4",
      selected_unit_ids: ["trnm.worker.alpha"],
      scoped_update_actor_ids: ["trnm.worker.alpha", "trnm.horizon.scout.alpha", "trnm.command.core.alpha", "trnm.flux.beacon.center"],
      accepted_runtime_command_labels: ["move:8,4"],
      rejected_runtime_command_labels: ["client:attack_fogged_keep"],
      runtime_control_group_id: "1",
      runtime_group_command_state: "offline_adapter_authority_applied",
      runtime_command_stamp_source: "trnm-rts-online:offline_loopback_authority",
      runtime_command_stamp_kind: "server_accepted_move",
      runtime_command_stamp_tile_id: "8,4",
      runtime_last_feedback: "Offline adapter applied server move 8,4; rejected target_actor_not_visible",
      command_surface_replaced_gate: true,
      route_overlay_replaced_gate: true,
      session_context_preserved_gate: true,
      rejected_order_suppressed_gate: true,
      no_socket_boundary_gate: true,
      input_path: "trnm-rts-data player-screen application + trnm-rts-online offline adapter handoff -> trnm-rts-bevy-runtime session transition review",
      runtime_path: "trnm-rts-bevy-runtime first_contact_offline_adapter_session_transition -> Bevy local session UI transition evidence",
      source_of_truth: "This Bevy-free transition review proves the First Contact player-screen command surface and route overlay move from data-seeded local UI state to the server-authoritative offline adapter handoff while room, camera, visibility, supply, objective context, scoped updates, and no-socket claims stay coherent."
    },
    rts_online_offline_adapter_session_transition_gate: true,
    rts_online_offline_adapter_lobby_ready_contract: "trnm_rts_bevy_runtime_first_contact_offline_adapter_lobby_ready_v1",
    rts_online_offline_adapter_lobby_ready: {
      contract_version: "trnm_rts_bevy_runtime_first_contact_offline_adapter_lobby_ready_v1",
      green: true,
      adapter_contract: "trnm_rts_online_offline_adapter_v1",
      adapter_id: "first-contact-offline-loopback-adapter",
      handoff_id: "first-contact-local-loopback-handoff",
      arena_id: "first-contact-local-arena",
      map_id: "first_contact_basin",
      adapter_mode: "offline_loopback_authority",
      bevy_client_role: "visualization_and_local_input_submitter",
      authority_role: "trnm_rts_online_fixture_authority_no_socket",
      connected_player_ids: ["local-player", "mirror_guard"],
      bot_player_ids: ["mirror_guard"],
      ready_state_labels: ["player:local-player:ready", "player:mirror_guard:ready", "bot:mirror_guard:ready", "authority:offline_loopback:no_socket"],
      blocked_network_claim_labels: ["client_prediction:not_claimed", "rollback_netcode:not_claimed", "socket:not_claimed", "hosted_service:not_claimed", "public_launch:not_claimed"],
      local_multiplayer_ready_gate: true,
      offline_bot_ready_gate: true,
      bevy_adapter_ready_gate: true,
      authority_ready_gate: true,
      frame_identity_gate: true,
      no_network_claim_gate: true,
      input_path: "trnm-rts-online offline adapter lobby ready input -> trnm-rts-bevy-runtime lobby ready review",
      runtime_path: "trnm-rts-bevy-runtime first_contact_offline_adapter_lobby_ready -> Bevy local lobby/ready-state evidence",
      source_of_truth: "This Bevy-free lobby ready review proves the local First Contact offline adapter has two connected players, one offline bot, stable frame identities, Bevy acting only as visualization/input submitter, and no socket, hosted-service, client-prediction, rollback, or public-launch credit before the Bevy adapter renders any lobby or ready-state UI."
    },
    rts_online_offline_adapter_lobby_ready_gate: true,
    source_policy: "OpenRA engine code and third-party/proprietary RTS assets are not copied; First Contact Basin remains internal-only until GPL component review or replacement.",
    android_s5_real_device_claimed: false,
    public_launch_ready: false
  }
  | .contract_field_count = ([keys[] | select(endswith("_contract"))] | length)
  | .guard_object_count = ([to_entries[] | select((.key | test("^first_contact_.*_guard$")) and (.value | type == "object"))] | length)
  | .guard_gate_count = ([to_entries[] | select((.key | test("^first_contact_.*_guard_gate$")) and (.value | type == "boolean"))] | length)
  | .passed_guard_gate_count = ([to_entries[] | select((.key | test("^first_contact_.*_guard_gate$")) and .value == true)] | length)
  | .failed_guard_gate_count = (.guard_gate_count - .passed_guard_gate_count)
  | .top_level_gate_count = ([to_entries[] | select((.key | endswith("_gate")) and (.value | type == "boolean"))] | length)
  | .passed_top_level_gate_count = ([to_entries[] | select((.key | endswith("_gate")) and .value == true)] | length)
  | .failed_top_level_gate_count = (.top_level_gate_count - .passed_top_level_gate_count)
  | .rts_data_map_model_field_count = ((.rts_data_map_model // {}) | keys | length)
  | .rts_data_map_summary_field_count = ((.rts_data_map_summary // {}) | keys | length)
  | .rts_data_map_model_actor_count = ((.rts_data_map_model.actors // []) | length)
  | .rts_data_map_model_player_count = ((.rts_data_map_model.players // []) | length)
  | .rts_data_map_model_rule_count = ((.rts_data_map_model.rules // []) | length)
  | .rts_data_preview_actor_count = ((.rts_data_preview_actors // []) | length)
  | .rts_data_player_startup_profile_count = ((.rts_data_player_startup_profiles // []) | length)
  | .rts_data_actor_presentation_profile_count = ((.rts_data_actor_presentation_profiles // []) | length)
  | .online_protocol_fixture_field_count = ((.rts_online_protocol_fixture // {}) | keys | length)
  | .online_protocol_transport_field_count = ((.rts_online_protocol_fixture.transport // {}) | keys | length)
  | .online_protocol_authority_field_count = ((.rts_online_protocol_fixture.authority // {}) | keys | length)
  | .online_local_handoff_field_count = ((.rts_online_local_handoff // {}) | keys | length)
  | .offline_adapter_field_count = ((.rts_online_offline_adapter // {}) | keys | length)
  | .offline_consumption_field_count = ((.rts_online_offline_adapter_consumption // {}) | keys | length)
  | .offline_session_transition_field_count = ((.rts_online_offline_adapter_session_transition // {}) | keys | length)
  | .offline_lobby_ready_field_count = ((.rts_online_offline_adapter_lobby_ready // {}) | keys | length)
  | .runtime_player_screen_command_queue_count = ((.rts_bevy_runtime_player_screen_application.command_queue // []) | length)
  | .runtime_player_screen_production_queue_count = ((.rts_bevy_runtime_player_screen_application.production_queue // []) | length)
  | .runtime_player_screen_build_queue_count = ((.rts_bevy_runtime_player_screen_application.build_queue // []) | length)
  | .runtime_player_screen_visible_tile_count = ((.rts_bevy_runtime_player_screen_application.visible_tile_ids // []) | length)
  | .runtime_player_screen_fogged_tile_count = ((.rts_bevy_runtime_player_screen_application.fogged_tile_ids // []) | length)
  | .runtime_player_screen_ability_command_count = ((.rts_bevy_runtime_player_screen_application.ability_command_ids // []) | length)
  | .offline_consumption_command_queue_count = ((.rts_online_offline_adapter_consumption.runtime_player_screen_review.command_queue // []) | length)
  | .offline_consumption_production_queue_count = ((.rts_online_offline_adapter_consumption.runtime_player_screen_review.production_queue // []) | length)
  | .offline_consumption_build_queue_count = ((.rts_online_offline_adapter_consumption.runtime_player_screen_review.build_queue // []) | length)
  | .offline_consumption_visible_tile_count = ((.rts_online_offline_adapter_consumption.runtime_player_screen_review.visible_tile_ids // []) | length)
  | .offline_consumption_fogged_tile_count = ((.rts_online_offline_adapter_consumption.runtime_player_screen_review.fogged_tile_ids // []) | length)
  | .offline_consumption_ability_command_count = ((.rts_online_offline_adapter_consumption.runtime_player_screen_review.ability_command_ids // []) | length)
  | .offline_lobby_ready_label_count = ((.rts_online_offline_adapter_lobby_ready.ready_state_labels // []) | length)
  ' >"$first_contact_basin_spec_json"
  add_artifact_from_path native_bevy_classic_rts_first_contact_basin_spec "Native/Bevy classic RTS First Contact Basin spec" "$first_contact_basin_spec_json" release_review_input
}

add_modeling_foundation_packet_fixtures() {
  local asset_pack_json="$TMP_DIR/bevy-classic-asset-pack.json"
  jq -n '{
    contract_version: "trillionnium_world_bevy_classic_asset_pack_v1",
    status: "classic_asset_pack_green",
    green: true,
    ready_for_release_review: true,
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
    gate_count: 14,
    passed_gate_count: 14,
    failed_gate_count: 0,
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
    status: "classic_manifest_lint_green",
    green: true,
    ready_for_release_review: true,
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
    role_family_count: 17,
    duplicate_frame_ids: [],
    duplicate_frame_id_count: 0,
    out_of_bounds_frame_ids: [],
    out_of_bounds_frame_id_count: 0,
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
    gate_count: 16,
    passed_gate_count: 16,
    failed_gate_count: 0,
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
    status: "classic_isometric_modeling_green",
    green: true,
    ready_for_release_review: true,
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
    projection_sample_count: 4,
    depth_order: [
      {depth_key: 41, frame_id: "model_town_hall", id: "town_hall"},
      {depth_key: 74, frame_id: "actor_mentor_talk", id: "mentor"},
      {depth_key: 95, frame_id: "actor_player_walk_south_1", id: "player"},
      {depth_key: 96, frame_id: "actor_guard_attack", id: "square_guard_front"},
      {depth_key: 136, frame_id: "actor_creep_attack", id: "square_creep_pressure"},
      {depth_key: 154, frame_id: "doodad_crystal_cluster", id: "square_crystal"}
    ],
    depth_order_count: 6,
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
    modeling_component_count: 30,
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
    gate_count: 13,
    passed_gate_count: 13,
    failed_gate_count: 0,
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
    wgpu_required: false,
    android_s5_real_device_claimed: false,
    external_evidence_ignored_for_current_isometric_modeling_pass: true,
    public_launch_ready: false,
    production_ready_ui_claimed: false,
    screen_for_screen_openra_ui_claimed: false,
    openra_engine_port_claimed: false,
    warcraft_iii_asset_copied: false,
    openra_asset_copied: false,
    third_party_asset_copied: false
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
    status: "classic_input_frame_budget_green",
    green: true,
    ready_for_release_review: true,
    loaded_from_manifest: true,
    atlas_parse_gate: true,
    accepted_input_gate: true,
    direction_coverage_gate: true,
    rendered_frame_gate: true,
    selected_frame_manifest_gate: true,
    response_p95_budget_gate: true,
    response_max_budget_gate: true,
    gate_count: 7,
    passed_gate_count: 7,
    failed_gate_count: 0,
    sample_count: 96,
    accepted_input_count: 96,
    sample_detail_count: 4,
    accepted_direction_count: 4,
    selected_frame_id_count: 4,
    nonblank_sample_count: 4,
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
    status: "classic_render_budget_green",
    green: true,
    ready_for_release_review: true,
    loaded_from_manifest: true,
    atlas_parse_gate: true,
    frame_count_gate: true,
    p95_budget_gate: true,
    max_budget_gate: true,
    nonblank_gate: true,
    gate_count: 5,
    passed_gate_count: 5,
    failed_gate_count: 0,
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
    nonblank_sample_count: 4,
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
        exec_main_status: "0",
        cpu_weight: "50",
        cpu_quota_per_sec_usec: "500ms",
        expected_cpu_weight: "50",
        expected_cpu_quota_per_sec_usec: "500ms"
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
          TRNM_WORLD_BEVY_CLASSIC_PLAYER_SCREEN: "1",
          WINIT_UNIX_BACKEND: "x11",
          WAYLAND_DISPLAY: "",
          TRNM_WORLD_BEVY_CLASSIC_ASSET_MANIFEST: ($root + "/assets/trnm-world/classic/manifest.json"),
          TRNM_WORLD_BEVY_CLASSIC_ASSET_OVERRIDE_DIR: ($root + "/assets/trnm-world/classic/art-pack-v1")
        }
      },
      live_player_screen: {
        window_id: "0x800021",
        window_title: "Trillionnium RTS | room=first-contact-basin | tile=5,4 | cam=5,4 | owned-assets-v1",
        window_title_chars: 82,
        window_width: 1280,
        window_height: 720,
        screenshot_path: ($root + "/acceptance/S5_native_bevy_device/latest/manual_bevy/bevy-classic-player-screen-runner-status.png"),
        probe_path: ($root + "/acceptance/S5_native_bevy_device/latest/manual_bevy/bevy-classic-player-screen-runner-status-probe.json"),
        screenshot_bytes: 49585,
        contract_version: "trillionnium_world_bevy_classic_player_screen_runner_visual_v1",
        title_rule: "concise player-facing title keeps room and owned-art-pack identity while excluding debug/proof strings and shortcut manuals"
      },
      gates: {
        service_process_gate: true,
        release_binary_gate: true,
        classic_env_gate: true,
        player_screen_env_gate: true,
        x11_backend_gate: true,
        manifest_gate: true,
        override_dir_gate: true,
        workdir_gate: true,
        cpu_budget_gate: true,
        cex_path_gate: true,
        player_screen_window_gate: true,
        player_screen_title_gate: true,
        player_screen_proof_debug_absent_gate: true,
        player_screen_title_concise_gate: true,
        player_screen_screenshot_gate: true,
        player_screen_region_gate: true,
        player_screen_dead_panel_gate: true,
        player_screen_clipped_label_gate: true,
        player_screen_gameplay_scene_gate: true,
        player_screen_debug_title_absent_gate: true,
        player_screen_visual_gate: true
      },
      ready_for_release_review: true,
      public_launch_ready: false,
      android_s5_real_device_claimed: false,
      source_of_truth: "The live playtest runner must be the release trnm-world-bevy binary with the low-spec classic player screen, X11 backend, classic renderer manifest, a concise player-facing First Contact Basin window title, a bounded CPUQuota/CPUWeight budget, and a visible First Contact Basin player screen with real map/HUD/command pixels, non-dead HUD panels, clipped-label edge safety, and gameplay-scene balance; CEX paths are explicitly rejected, and proof/debug/shortcut-manual default title strings are explicitly rejected."
  }
  | .runtime_cmdline_arg_count = (.runtime.cmdline | length)
  | .selected_environment_count = (.runtime.selected_environment | keys | length)
  | .live_player_screen_evidence_path_count = ([.live_player_screen.screenshot_path, .live_player_screen.probe_path] | map(select(. != null and . != "")) | length)
  | .live_player_screen_pixel_count = (.live_player_screen.window_width * .live_player_screen.window_height)
  | .gate_count = (.gates | keys | length)
  | .passed_gate_count = ([.gates[] | select(. == true)] | length)
  | .failed_gate_count = ([.gates[] | select(. != true)] | length)' >"$playtest_runner_json"
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
          exec_main_status: "0",
          cpu_weight: "50",
          cpu_quota_per_sec_usec: "500ms",
          expected_cpu_weight: "50",
          expected_cpu_quota_per_sec_usec: "500ms"
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
      public_launch_ready: false,
      public_launch_ready_claimed: false,
      source_of_truth: "A player-ready classic playtest launcher must expose CAMPAIGN title actions, persist and restore the campaign slot, resume into the Bevy-owned open-world state, and run on the live release trnm-world-bevy service with no CEX runtime path."
    }
    | .source_contract_count = ([.campaign_entry_contract, .runner_status_contract, .title_menu_contract, .state_snapshot_contract] | length)
    | .title_action_count = (.player_entry.title_actions | length)
    | .runner_cmdline_arg_count = (.live_runner.runtime.cmdline | length)
    | .runner_selected_environment_count = (.live_runner.runtime.selected_environment | keys | length)
    | .gate_count = (.gates | keys | length)
    | .passed_gate_count = ([.gates[] | select(. == true)] | length)
    | .failed_gate_count = ([.gates[] | select(. != true)] | length)' >"$playtest_launcher_json"
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
        playtest_observability_readiness: "trillionnium_world_bevy_classic_rts_playtest_observability_readiness_v1",
        first_contact_runtime_review: "trnm_rts_evidence_bevy_runtime_adapter_v1"
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
        endurance_peak_active_units: 32,
        first_contact_basin_map_id: "first_contact_basin",
        first_contact_basin_actor_count: 39,
        first_contact_runtime_review_contract: "trnm_rts_evidence_bevy_runtime_adapter_v1",
        first_contact_runtime_review_contract_count: 5,
        first_contact_runtime_review_contracts: ["trnm_rts_bevy_runtime_first_contact_player_screen_application_v1", "trnm_rts_bevy_runtime_first_contact_offline_adapter_runtime_application_v1", "trnm_rts_bevy_runtime_first_contact_offline_adapter_consumption_v1", "trnm_rts_bevy_runtime_first_contact_offline_adapter_session_transition_v1", "trnm_rts_bevy_runtime_first_contact_offline_adapter_lobby_ready_v1"],
        first_contact_runtime_review_before_command_queue: ["build:trnm.flux.relay", "train:trnm.worker", "attack:trnm.flux.beacon"],
        first_contact_runtime_review_after_command_queue: ["move:8,4"],
        first_contact_runtime_review_ready_state_labels: ["authority:offline_loopback:no_socket", "player:local-player:ready", "bot:mirror_guard:ready", "player:mirror_guard:ready"],
        first_contact_runtime_review_command_stamp_tile: "8,4"
      },
      gates: {
        playtest_readiness_gate: true,
        launcher_gate: true,
        runner_gate: true,
        observability_gate: true,
        first_minute_gate: true,
        map_ui_modeling_gate: true,
        first_contact_basin_spec_green: true,
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
        first_contact_basin_spec_gate: true,
        first_contact_runtime_review_gate: true,
        first_contact_runtime_adapter_evidence_gate: true,
        first_contact_offline_adapter_consumption_gate: true,
        first_contact_offline_adapter_session_transition_gate: true,
        first_contact_offline_adapter_lobby_ready_gate: true,
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
      public_launch_ready: false,
      source_of_truth: "Classic playtest handoff readiness is the local human-playtest handoff layer for trnm-world-bevy. It requires the full Bevy classic playtest readiness chain, a live release runner, a campaign launcher that resumes into the Bevy-owned open-world RTS handoff, and observability evidence. It does not claim S5 real-device evidence, public launch readiness, or OpenRA natural replay/headless parity."
    }
    | .source_contract_count = (.source_contracts | keys | length)
    | .evidence_path_count = (.evidence_paths | keys | length)
    | .handoff_summary_field_count = (.handoff_summary | keys | length)
    | .title_action_count = (.handoff_summary.title_actions | length)
    | .first_contact_runtime_review_contract_count = (.handoff_summary.first_contact_runtime_review_contracts | length)
    | .first_contact_runtime_review_before_command_count = (.handoff_summary.first_contact_runtime_review_before_command_queue | length)
    | .first_contact_runtime_review_after_command_count = (.handoff_summary.first_contact_runtime_review_after_command_queue | length)
    | .first_contact_runtime_review_ready_state_label_count = (.handoff_summary.first_contact_runtime_review_ready_state_labels | length)
    | .gate_count = (.gates | keys | length)
    | .passed_gate_count = ([.gates[] | select(. == true)] | length)
    | .failed_gate_count = ([.gates[] | select(. != true)] | length)' >"$playtest_handoff_readiness_json"
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
        playtest_observability_readiness: "trillionnium_world_bevy_classic_rts_playtest_observability_readiness_v1",
        first_contact_runtime_review: "trnm_rts_evidence_bevy_runtime_adapter_v1"
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
        endurance_peak_active_units: 32,
        first_contact_basin_map_id: "first_contact_basin",
        first_contact_basin_actor_count: 39,
        first_contact_runtime_review_contract: "trnm_rts_evidence_bevy_runtime_adapter_v1",
        first_contact_runtime_review_contract_count: 5,
        first_contact_runtime_review_contracts: ["trnm_rts_bevy_runtime_first_contact_player_screen_application_v1", "trnm_rts_bevy_runtime_first_contact_offline_adapter_runtime_application_v1", "trnm_rts_bevy_runtime_first_contact_offline_adapter_consumption_v1", "trnm_rts_bevy_runtime_first_contact_offline_adapter_session_transition_v1", "trnm_rts_bevy_runtime_first_contact_offline_adapter_lobby_ready_v1"],
        first_contact_runtime_review_before_command_queue: ["build:trnm.flux.relay", "train:trnm.worker", "attack:trnm.flux.beacon"],
        first_contact_runtime_review_after_command_queue: ["move:8,4"],
        first_contact_runtime_review_ready_state_labels: ["authority:offline_loopback:no_socket", "player:local-player:ready", "bot:mirror_guard:ready", "player:mirror_guard:ready"],
        first_contact_runtime_review_command_stamp_tile: "8,4"
      },
      gates: {
        handoff_readiness_green: true,
        playtest_readiness_green: true,
        launcher_green: true,
        runner_green: true,
        observability_green: true,
        public_launch_not_claimed_gate: true,
        android_s5_real_device_not_claimed_gate: true,
        first_contact_basin_spec_gate: true,
        first_contact_runtime_review_gate: true,
        first_contact_runtime_adapter_evidence_gate: true,
        first_contact_offline_adapter_consumption_gate: true,
        first_contact_offline_adapter_session_transition_gate: true,
        first_contact_offline_adapter_lobby_ready_gate: true,
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
      public_launch_ready: false,
      public_launch_ready_claimed: false,
      android_s5_real_device_claimed: false,
      openra_natural_replay_or_headless_parity_claimed: false,
      markdown_path: "acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-handoff-packet.md",
      source_of_truth: "Classic playtest handoff packet binds the local Bevy human-playtest handoff to checksummed evidence artifacts and replayable commands. It is a local host-side playtest packet only, not public launch, S5 real-device, or OpenRA natural replay/headless parity credit."
    }
    | .source_contract_count = (.source_contracts | keys | length)
    | .handoff_summary_field_count = (.handoff_summary | keys | length)
    | .title_action_count = (.handoff_summary.title_actions | length)
    | .first_contact_runtime_review_contract_count = (.handoff_summary.first_contact_runtime_review_contracts | length)
    | .first_contact_runtime_review_before_command_count = (.handoff_summary.first_contact_runtime_review_before_command_queue | length)
    | .first_contact_runtime_review_after_command_count = (.handoff_summary.first_contact_runtime_review_after_command_queue | length)
    | .first_contact_runtime_review_ready_state_label_count = (.handoff_summary.first_contact_runtime_review_ready_state_labels | length)
    | .artifact_count = (.artifact_manifest | length)
    | .artifact_label_count = ([.artifact_manifest[].label] | length)
    | .artifact_path_count = ([.artifact_manifest[].path] | length)
    | .run_command_count = (.run_commands | keys | length)
    | .no_credit_boundary_count = (.no_credit_boundaries | keys | length)
    | .artifact_bytes_total = ([.artifact_manifest[].bytes] | add)
    | .gate_count = (.gates | keys | length)
    | .passed_gate_count = ([.gates[] | select(. == true)] | length)
    | .failed_gate_count = ([.gates[] | select(. != true)] | length)' >"$playtest_handoff_packet_json"
  add_artifact_from_path native_bevy_classic_playtest_handoff_packet "Native/Bevy classic playtest handoff packet" "$playtest_handoff_packet_json" release_review_input

  local playtest_handoff_packet_md="$TMP_DIR/bevy-classic-playtest-handoff-packet.md"
  {
    printf '# Bevy Classic Playtest Handoff Packet\n\n'
    printf -- '- Status: `true`\n'
    printf -- '- Contract: `trillionnium_world_bevy_classic_playtest_handoff_packet_v1`\n'
    printf -- '- Gate count: `15` / `15` passed\n'
    printf -- '- Artifact count: `5`, bytes `22528`\n'
    printf -- '- Runner: `trillionnium-bevy-playtest.service` PID `160672`\n'
    printf -- '- Resume: `league-coliseum` / `arena_outdoor` / `resumed:league-coliseum`\n'
    printf -- '- Campaign slot bytes: `71913`\n'
    printf -- '- Title actions: `CAMPAIGN:START, CAMPAIGN:CONTINUE, CAMPAIGN:REPLAY`\n\n'
    printf -- '- First Contact runtime review: `trnm_rts_evidence_bevy_runtime_adapter_v1` after `move:8,4` tile `8,4`\n\n'
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
    runtime_screen_mode: "player_runtime_campaign_ui_continuity_screen",
    runtime_screen_gate: true,
    evidence_board_only: false,
    runtime_screen_layout: {
      primary_tactical_viewport: "large restored route tactical state with open-world resume status",
      campaign_route_rail: "sixteen accepted campaign handoff stages shown as a player-side route rail",
      resume_timeline: "bottom player timeline binds objective, expansion, siege, keep, restoration, and open-world resume"
    },
    runtime_screen_layout_count: 3,
    rts_evidence_campaign_ui_continuity_review_contract: "trnm_rts_evidence_campaign_ui_continuity_review_v1",
    rts_evidence_campaign_ui_continuity_review: {
      contract_version: "trnm_rts_evidence_campaign_ui_continuity_review_v1",
      green: true,
      campaign_handoff_contract: "trillionnium_world_bevy_classic_rts_campaign_handoff_v1",
      campaign_handoff_green: true,
      preview_width: 1920,
      preview_height: 1080,
      capture_frame_count: 16,
      final_current_room_id: "league-coliseum",
      restored_current_room_id: "league-coliseum",
      final_route_director_task_id: "task-fixture-first-route",
      restored_route_director_task_id: "task-fixture-first-route",
      final_open_world_handoff_state: "resumed:league-coliseum",
      restored_open_world_handoff_state: "resumed:league-coliseum",
      handoff_green_gate: true,
      preview_resolution_gate: true,
      live_input_gate: true,
      milestone_gate: true,
      map_ui_state_gate: true,
      restored_ui_state_gate: true,
      persistence_gate: true,
      render_readability_gate: true,
      native_client_boundary_gate: true,
      player_first_campaign_continuity_screen_gate: true,
      input_path: "trnm-world-bevy campaign handoff evidence JSON -> trnm-rts-evidence campaign UI continuity review",
      evidence_path: "trnm-rts-evidence campaign_ui_continuity_review -> Bevy campaign UI continuity packet artifact",
      source_of_truth: "The RTS evidence crate reviews campaign handoff final/restored route state, save-restore persistence, milestone pixels, native-client no-credit boundary, and player-first route-resume screen gates before trnm-world-bevy includes the campaign UI continuity artifact in release-review evidence."
    },
    rts_evidence_campaign_ui_continuity_review_field_count: 26,
    rts_evidence_campaign_ui_continuity_review_gate: true,
    capture_frame_count: 16,
    final_current_room_id: "league-coliseum",
    final_map_scene: "arena_outdoor",
    final_route_director_task_id: "task-fixture-first-route",
    final_route_director_next_room_id: null,
    final_open_world_handoff_state: "resumed:league-coliseum",
    final_contextual_primary_action_label: "COMBAT:attack",
    final_contextual_action_labels: ["TITLE:OPEN", "ACCOUNT:REGISTER", "ACCOUNT:LOGIN", "ACCOUNT:CONTINUE", "ROOM:mirror-city-square", "ROOM:delivery-dock", "NPC:enemy-market-bandit", "COMBAT:attack", "COMBAT:defend", "COMBAT:potion", "COMBAT:escape", "BAG:open", "STAT:force", "STAT:agility", "STAT:craft", "SAVE:SLOT", "SLOT:A", "SAVE:A", "SLOT:B", "SAVE:B", "SLOT:C", "SAVE:C", "SAVE:SELECTED", "PAUSE:MENU"],
    final_contextual_action_label_count: 24,
    final_active_task_ids: ["task-fixture-first-route"],
    final_active_task_count: 1,
    final_objective_status: "open_world_after_action_ready",
    restored_current_room_id: "league-coliseum",
    restored_map_scene: "arena_outdoor",
    restored_open_world_handoff_state: "resumed:league-coliseum",
    restored_route_director_task_id: "task-fixture-first-route",
    restored_route_director_next_room_id: null,
    restored_contextual_action_labels: ["TITLE:OPEN", "ACCOUNT:REGISTER", "ACCOUNT:LOGIN", "ACCOUNT:CONTINUE", "ROOM:mirror-city-square", "ROOM:delivery-dock", "NPC:enemy-market-bandit", "COMBAT:attack", "COMBAT:defend", "COMBAT:potion", "COMBAT:escape", "BAG:open", "STAT:force", "STAT:agility", "STAT:craft", "SAVE:SLOT", "SLOT:A", "SAVE:A", "SLOT:B", "SAVE:B", "SLOT:C", "SAVE:C", "SAVE:SELECTED", "PAUSE:MENU"],
    restored_contextual_action_label_count: 24,
    restored_active_task_ids: ["task-fixture-first-route"],
    restored_active_task_count: 1,
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
    milestone_count: 16,
    non_background_pixels: 2073600,
    victory_pixel_count: 1702,
    expansion_pixel_count: 20311,
    breach_pixel_count: 4574,
    keep_pixel_count: 729,
    restoration_pixel_count: 1593,
    open_world_pixel_count: 1088,
    campaign_continuity_pixel_counts: {
      player_first_campaign_view_non_background: 820000,
      player_first_campaign_view_frame: 14000,
      player_first_campaign_status_strip: 28000,
      player_first_campaign_route_rail: 160000
    },
    campaign_continuity_pixel_count_field_count: 4,
    handoff_green_gate: true,
    preview_resolution_gate: true,
    live_input_gate: true,
    milestone_gate: true,
    map_ui_state_gate: true,
    restored_ui_state_gate: true,
    persistence_gate: true,
    render_readability_gate: true,
    native_client_boundary_gate: true,
    player_first_campaign_continuity_screen_gate: true,
    gate_count: 12,
    passed_gate_count: 12,
    failed_gate_count: 0,
    android_s5_real_device_claimed: false,
    public_launch_ready: false,
    screen_for_screen_openra_ui_claimed: false,
    openra_engine_port_claimed: false,
    source_of_truth: "Classic RTS campaign UI continuity evidence binds the Bevy-owned campaign handoff preview to final and restored map scene, route director, objective panel, contextual action labels, milestone pixels, and native-client boundary gates through trnm-rts-evidence review so the RTS-to-open-world map/UI handoff cannot regress silently. The preview is a player-first route-resume screen, not a contact sheet."
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
    green: true,
    fixture_only: true,
    provider_mode: "fixture",
    source_of_truth: "trnm_world_map_provider_fixture_modeling",
    public_map_pack_ready: false,
    public_launch_ready: false,
    public_launch_credit: false,
    live_ingestion_performed: false,
    live_ingestion_enabled: false,
    runtime_clients_fetch_public_osm_directly: false,
    public_network_ready: false,
    building_model_count: 24,
    road_model_count: 62,
    greenery_model_count: 8,
    terrain_model_count: 4,
    modeling_layer_count: 4,
    required_next_evidence_count: 9,
    gate_count: 6,
    passed_gate_count: 6,
    failed_gate_count: 0,
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

add_public_launch_readiness_packet_fixtures() {
  local readiness_json="$TMP_DIR/public-launch-readiness.json"
  jq -n '{
    contract_version: "trillionnium_world_public_launch_readiness_v1",
    overall_status: "blocked_missing_public_launch_evidence",
    source_of_truth: "trillionnium_world_public_launch_readiness_gate",
    green: true,
    public_launch_ready: false,
    public_launch_claimed: false,
    android_s5_real_device_claimed: false,
    launch_rule: "do_not_claim_public_launch_ready_without_native_bevy_local_playability_texture_sampling_render_asset_eligibility_real_device_map_pack_cohort_commercial_multi_node_and_public_deploy_evidence",
    blockers: ["s5_real_device_matrix", "production_map_pack_public_evidence", "first_beta_cohort_evidence", "commercial_launch_drill_evidence", "multi_node_or_live_traffic_latency_evidence", "public_network_live_exposure_evidence"],
    blocker_count: 6,
    unknown_blockers: [],
    unknown_blocker_count: 0,
    known_public_launch_blockers: ["s5_real_device_matrix", "production_map_pack_public_evidence", "first_beta_cohort_evidence", "commercial_launch_drill_evidence", "multi_node_or_live_traffic_latency_evidence", "public_network_live_exposure_evidence"],
    known_public_launch_blocker_count: 6,
    gates: {
      dev_runtime_repository: {evidence_path: "/fixture/dev-runtime-repository-smoke.json", status: "file_repository_persistence_green", required_status: "file_repository_persistence_green"},
      standalone_browser_parity: {evidence_path: "/fixture/browser-parity.json", status: "standalone_browser_parity_green", file_status: "present", accepted_status: "standalone_browser_parity_green"},
      repository_adapter_boundary: {evidence_path: "/fixture/repository-adapter-boundary.json", status: "repository_adapter_boundary_green", file_status: "present", accepted_status: "repository_adapter_boundary_green"},
      release_rollback_backup: {evidence_path: "/fixture/release-rollback-backup-drill.json", status: "release_rollback_backup_drill_green", file_status: "present", accepted_status: "release_rollback_backup_drill_green"},
      cohort_commercial_schema: {evidence_path: "/fixture/cohort-commercial-evidence-schema.json", status: "cohort_commercial_evidence_schema_green", file_status: "present", accepted_status: "cohort_commercial_evidence_schema_green"},
      cohort_commercial_evidence: {evidence_path: "/fixture/cohort-commercial-evidence.json", status: "blocked_missing_cohort_commercial_real_evidence", file_status: "present", accepted_status: "cohort_commercial_evidence_green"},
      external_ops_evidence: {evidence_path: "/fixture/external-ops-evidence.json", status: "blocked_missing_external_ops_real_evidence", file_status: "present", accepted_status: "external_ops_evidence_green"},
      s5_real_device_matrix: {evidence_path: "/fixture/s5-device-evidence.json", status: "blocked_missing_s5_real_device_evidence", file_status: "present", validator_summary: "/fixture/s5-real-device-evidence-validation.json", validator_file_status: "present", required_status: "s5_real_device_evidence_green"},
      native_bevy_keyboard_replay: {evidence_path: "/fixture/bevy-build-branch-title-route-all-branch-keyboard-replay.json", file_status: "present", contract_version: "trillionnium_world_bevy_build_branch_title_route_all_branch_keyboard_replay_v1", green: true, required_contract: "trillionnium_world_bevy_build_branch_title_route_all_branch_keyboard_replay_v1", required_green: true, proof_scope: "host_side_bevy_runtime_replay_not_android_real_device"},
      native_bevy_action_coach: {evidence_path: "/fixture/bevy-action-coach.json", file_status: "present", contract_version: "trillionnium_world_bevy_action_coach_v1", green: true, required_contract: "trillionnium_world_bevy_action_coach_v1", required_green: true, proof_scope: "host_side_bevy_runtime_guidance_not_android_real_device"},
      native_bevy_player_hud_debug_layer: {evidence_path: "/fixture/bevy-player-hud-debug-layer.json", file_status: "present", contract_version: "trillionnium_world_bevy_player_hud_debug_layer_v1", green: true, required_contract: "trillionnium_world_bevy_player_hud_debug_layer_v1", required_green: true, proof_scope: "host_side_bevy_hud_layer_not_android_real_device"},
      native_bevy_live_window_screenshot_sequence: {evidence_path: "/fixture/bevy-live-window-screenshot-sequence.json", file_status: "present", contract_version: "trillionnium_world_bevy_live_window_screenshot_sequence_v1", green: true, frame_sequence_gate: true, contact_sheet_gate: true, required_contract: "trillionnium_world_bevy_live_window_screenshot_sequence_v1", required_green: true, proof_scope: "host_side_live_window_screenshot_sequence_not_android_real_device"},
      native_bevy_sprite_texture_sampling: {evidence_path: "/fixture/bevy-sprite-texture-sampling.json", file_status: "present", contract_version: "trillionnium_world_bevy_sprite_texture_sampling_v1", green: true, four_layer_texture_sampling_gate: true, texture_sample_nonblank_gate: true, required_contract: "trillionnium_world_bevy_sprite_texture_sampling_v1", required_green: true, proof_scope: "host_side_cpu_texture_sampling_not_gpu_upload_or_android_real_device"},
      native_bevy_live_window_sampled_texture_correlation: {evidence_path: "/fixture/bevy-live-window-sampled-texture-correlation.json", file_status: "present", contract_version: "trillionnium_world_bevy_live_window_sampled_texture_correlation_v1", green: true, four_layer_sampled_live_correlation_gate: true, required_contract: "trillionnium_world_bevy_live_window_sampled_texture_correlation_v1", required_green: true, proof_scope: "host_side_sampled_texture_to_live_window_correlation_not_android_real_device"},
      native_bevy_render_asset_eligibility: {evidence_path: "/fixture/bevy-render-asset-eligibility.json", file_status: "present", contract_version: "trillionnium_world_bevy_render_asset_eligibility_v1", green: true, render_asset_usage_gate: true, sprite_render_reference_gate: true, required_contract: "trillionnium_world_bevy_render_asset_eligibility_v1", required_green: true, proof_scope: "host_side_render_asset_eligibility_not_render_world_extraction_or_gpu_upload"},
      signed_map_pack: {evidence_path: "/fixture/map_pack_manifest_signed.json", summary_path: "/fixture/map-pack-gate-summary.json", status: "fixture_signed_map_pack_gate_green", manifest_status: "present", required_status: "fixture_signed_map_pack_gate_green"},
      production_map_pack: {evidence_path: "/fixture/production-map-pack-public-evidence.json", status: "blocked_missing_production_map_pack_public_evidence", file_status: "present", accepted_status: "production_map_pack_public_ready_green", local_route_status: "production_map_pack_route_green", evidence_contract: "trillionnium_world_production_map_pack_public_evidence_gate_v1", live_ingestion_allowed: false, runtime_clients_fetch_public_osm_directly: false},
      first_beta_cohort: {evidence_path: "", status: "blocked_missing_first_beta_cohort_evidence", file_status: "missing", accepted_status: "first_beta_cohort_evidence_green", validator_summary: "/fixture/cohort-commercial-evidence.json"},
      commercial_launch_drill: {evidence_path: "", status: "blocked_missing_commercial_launch_drill_evidence", file_status: "missing", accepted_status: "commercial_launch_drill_evidence_green", validator_summary: "/fixture/cohort-commercial-evidence.json"},
      multi_node_or_live_traffic_latency: {evidence_path: "", status: "blocked_missing_multi_node_or_live_traffic_latency_evidence", file_status: "missing", accepted_status: "multi_node_or_live_traffic_latency_green", local_drill_status: "local_release_latency_drill_green", validator_summary: "/fixture/external-ops-evidence.json"},
      public_network_deploy: {evidence_path: "", status: "blocked_missing_public_network_live_exposure_evidence", file_status: "missing", accepted_status: "public_network_deploy_green", local_drill_status: "local_public_deploy_drill_green", validator_summary: "/fixture/external-ops-evidence.json"}
    }
  }' >"$readiness_json"
  add_artifact_from_path public_launch_readiness "Public launch readiness" "$readiness_json" release_review_input
}

add_public_launch_collection_packet_fixtures() {
  local production_collection_json="$TMP_DIR/production-map-pack-public-evidence-collection.json"
  local cohort_collection_json="$TMP_DIR/cohort-commercial-evidence-collection.json"
  local external_collection_json="$TMP_DIR/external-ops-evidence-collection.json"

  jq -n '{
    contract_version: "trillionnium_world_production_map_pack_public_evidence_collection_v1",
    status: "production_map_pack_public_evidence_collection_ready",
    source_of_truth: "trillionnium_world_production_map_pack_public_evidence_collection",
    green: true,
    public_map_pack_ready: false,
    public_launch_credit: false,
    live_ingestion_performed: false,
    live_ingestion_allowed: false,
    runtime_clients_fetch_public_osm_directly: false,
    route_prerequisite_count: 1,
    template_count: 1,
    template_schema_count: 1,
    required_evidence_count: 11,
    validator_status_count: 1,
    blocked_validator_status_count: 1,
    collection_command: "scripts/check_trillionnium_world_production_map_pack_public_evidence_collection.sh",
    validation_command: "TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH=<real-map-pack-evidence.json> scripts/check_trillionnium_world_production_map_pack_public_evidence.sh --require-ready",
    route_prerequisite: {summary: "/fixture/production-map-pack-route.json", status: "production_map_pack_route_green", log: "/fixture/production-map-pack-public-evidence-collection-route.log", accepted_status: "production_map_pack_route_green"},
    validator: {summary: "/fixture/production-map-pack-public-evidence.json", status: "blocked_missing_production_map_pack_public_evidence", log: "/fixture/production-map-pack-public-evidence-collection-validator.log", accepted_status: "production_map_pack_public_ready_green"},
    template: {path: "/fixture/production-map-pack-public-evidence.template.json", sha256: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", schema_path: "/fixture/production-map-pack-public-evidence.schema.json", schema_sha256: "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb", public_launch_credit: false},
    required_evidence: [
      {id: "approved_production_map_source"}, {id: "offline_cache_policy"}, {id: "web_public_attribution_screenshot"}, {id: "native_bevy_android_attribution_screenshot"}, {id: "matrix_or_readonly_attribution_screenshot"}, {id: "sensitive_poi_filter"}, {id: "geofence_policy"}, {id: "key_custody_rotation"}, {id: "public_distribution_revocation"}, {id: "public_map_pack_rollback"}, {id: "operator_signoff"}
    ],
    boundary: [
      "This script creates a collection checklist only.",
      "It does not perform live Overpass or Geofabrik ingestion.",
      "It does not claim production_map_pack_public_ready_green.",
      "Only the validator can grant public launch credit after real external artifacts are attached."
    ],
    reviewer_next_action: "fill_template_with_real_public_map_pack_evidence_then_run_validator"
  }' >"$production_collection_json"
  add_artifact_from_path production_map_pack_public_evidence_collection "Production map-pack public evidence collection" "$production_collection_json" release_review_collection

  jq -n '{
    contract_version: "trillionnium_world_cohort_commercial_evidence_collection_v1",
    status: "cohort_commercial_evidence_collection_ready",
    source_of_truth: "trillionnium_world_cohort_commercial_evidence_collection",
    green: true,
    public_launch_credit: false,
    first_beta_ready: false,
    commercial_launch_drill_ready: false,
    template_count: 2,
    template_schema_count: 2,
    required_evidence_count: 11,
    validator_status_count: 2,
    blocked_validator_status_count: 2,
    collection_command: "scripts/check_trillionnium_world_cohort_commercial_evidence_collection.sh",
    validation_command: "TRILLIONNIUM_FIRST_BETA_COHORT_EVIDENCE_PATH=<real-cohort.json> TRILLIONNIUM_COMMERCIAL_LAUNCH_DRILL_EVIDENCE_PATH=<real-commercial-drill.json> scripts/check_trillionnium_world_cohort_commercial_evidence.sh --require-ready",
    schema: {summary: "/fixture/cohort-commercial-evidence-schema.json", status: "cohort_commercial_evidence_schema_green", log: "/fixture/cohort-commercial-evidence-collection-schema.log"},
    validator: {summary: "/fixture/cohort-commercial-evidence.json", log: "/fixture/cohort-commercial-evidence-collection-validator.log", first_beta_status: "blocked_missing_first_beta_cohort_evidence", commercial_launch_drill_status: "blocked_missing_commercial_launch_drill_evidence"},
    templates: {
      first_beta: {path: "/fixture/first-beta-cohort-evidence.template.json", sha256: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", schema_path: "/fixture/first-beta-cohort-evidence.schema.json", schema_sha256: "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb", accepted_status: "first_beta_cohort_evidence_green", public_launch_credit: false},
      commercial_launch_drill: {path: "/fixture/commercial-launch-drill-evidence.template.json", sha256: "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc", schema_path: "/fixture/commercial-launch-drill-evidence.schema.json", schema_sha256: "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd", accepted_status: "commercial_launch_drill_evidence_green", public_launch_credit: false}
    },
    required_evidence: [
      {id: "first_beta_participants"}, {id: "first_beta_sessions"}, {id: "first_beta_feedback_summary"}, {id: "first_beta_operator_signoff"}, {id: "commercial_payment_drill"}, {id: "commercial_refund_drill"}, {id: "commercial_support_drill"}, {id: "commercial_legal_drill"}, {id: "commercial_operator_drill"}, {id: "commercial_traffic_drill"}, {id: "commercial_operator_signoff"}
    ],
    privacy_boundary: [
      "Use sanitized participant ids and evidence references.",
      "Do not store private personal data in templates.",
      "Templates and collection checklists carry no public-launch credit."
    ],
    reviewer_next_action: "collect_real_first_beta_and_commercial_drill_evidence_then_run_validator"
  }' >"$cohort_collection_json"
  add_artifact_from_path cohort_commercial_evidence_collection "Cohort/commercial evidence collection" "$cohort_collection_json" release_review_collection

  jq -n '{
    contract_version: "trillionnium_world_external_ops_evidence_collection_v1",
    status: "external_ops_evidence_collection_ready",
    source_of_truth: "trillionnium_world_external_ops_evidence_collection",
    green: true,
    public_launch_credit: false,
    multi_node_or_live_traffic_latency_ready: false,
    public_network_deploy_ready: false,
    live_public_exposure_performed: false,
    template_count: 2,
    local_drill_count: 2,
    required_evidence_count: 12,
    validator_status_count: 2,
    blocked_validator_status_count: 2,
    collection_command: "scripts/check_trillionnium_world_external_ops_evidence_collection.sh",
    validation_command: "TRILLIONNIUM_MULTI_NODE_LATENCY_EVIDENCE_PATH=<real-latency.json> TRILLIONNIUM_PUBLIC_NETWORK_DEPLOY_EVIDENCE_PATH=<real-public-deploy.json> scripts/check_trillionnium_world_external_ops_evidence.sh --require-ready",
    validator: {summary: "/fixture/external-ops-evidence.json", log: "/fixture/external-ops-evidence-collection-validator.log", multi_node_or_live_traffic_latency_status: "blocked_missing_multi_node_or_live_traffic_latency_evidence", public_network_deploy_status: "blocked_missing_public_network_live_exposure_evidence"},
    templates: {
      multi_node_or_live_traffic_latency: {path: "/fixture/multi-node-latency-evidence.template.json", sha256: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", accepted_status: "multi_node_or_live_traffic_latency_green", public_launch_credit: false},
      public_network_deploy: {path: "/fixture/public-network-deploy-evidence.template.json", sha256: "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb", accepted_status: "public_network_deploy_green", public_launch_credit: false}
    },
    local_drills: {
      release_latency: {path: "/fixture/release-latency-drill.json", file_status: "present", status: "local_release_latency_drill_green", public_launch_credit: false},
      public_deploy: {path: "/fixture/public-network-deploy-evidence.json", file_status: "present", status: "local_public_deploy_drill_green", public_launch_credit: false}
    },
    required_evidence: [
      {id: "multi_node_or_live_traffic_scope"}, {id: "latency_endpoints"}, {id: "latency_public_url_probes"}, {id: "latency_p95_budget"}, {id: "monitoring_timeseries"}, {id: "rollback_under_load"}, {id: "latency_operator_signoff"}, {id: "public_exposure_approval"}, {id: "public_host_domain_tls"}, {id: "public_url_health_probes"}, {id: "public_monitoring_backup_rollback"}, {id: "public_exposure_operator_signoff"}
    ],
    boundary: [
      "This script creates a collection checklist only.",
      "It does not open a public network route.",
      "It does not create live public traffic.",
      "Local latency/deploy drills are useful but have no public-launch credit."
    ],
    reviewer_next_action: "collect_real_external_ops_evidence_then_run_validator"
  }' >"$external_collection_json"
  add_artifact_from_path external_ops_evidence_collection "External ops evidence collection" "$external_collection_json" release_review_collection
}

add_public_launch_validator_packet_fixtures() {
  local production_evidence_json="$TMP_DIR/production-map-pack-public-evidence.json"
  local cohort_evidence_json="$TMP_DIR/cohort-commercial-evidence.json"
  local external_ops_evidence_json="$TMP_DIR/external-ops-evidence.json"

  jq -n '{
    contract_version: "trillionnium_world_production_map_pack_public_evidence_gate_v1",
    status: "blocked_missing_production_map_pack_public_evidence",
    generated_at: "2026-06-07T00:00:00Z",
    source_of_truth: "trillionnium_world_production_map_pack_public_evidence_gate",
    green: false,
    public_map_pack_ready: false,
    accepted_status: "production_map_pack_public_ready_green",
    live_ingestion_performed: false,
    live_ingestion_allowed: false,
    runtime_clients_fetch_public_osm_directly: false,
    public_launch_credit: "only_when_status_is_production_map_pack_public_ready_green",
    blocker_count: 24,
    required_check_count: 24,
    schema_artifact_count: 2,
    blockers: [
      "production_map_pack_public_evidence_file",
      "production_map_pack_public_contract",
      "production_map_pack_public_status",
      "approved_production_map_source",
      "production_map_source_artifact",
      "license_and_odbl_compliance",
      "live_ingestion_must_remain_disabled",
      "offline_cache_policy",
      "cache_retention_refresh_policy",
      "web_public_attribution_screenshot",
      "native_bevy_android_attribution_screenshot",
      "matrix_or_readonly_attribution_screenshot",
      "sensitive_poi_filter",
      "sensitive_poi_report_artifact",
      "geofence_policy",
      "geofence_policy_artifact",
      "key_custody_rotation",
      "key_rotation_runbook_artifact",
      "public_distribution_revocation",
      "public_distribution_package_artifact",
      "revocation_probe_artifact",
      "public_map_pack_rollback",
      "public_map_pack_rollback_artifact",
      "operator_signoff"
    ],
    operator_evidence: {path: "", file_status: "missing", contract_version: "", status: ""},
    route_prerequisite: {evidence_path: "/fixture/production-map-pack-route.json", file_status: "present", status: "production_map_pack_route_green", accepted_status: "production_map_pack_route_green"},
    schema: {
      path: "/fixture/production-map-pack-public-evidence.schema.json",
      sha256: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
      template_path: "/fixture/production-map-pack-public-evidence.template.json",
      template_sha256: "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
    },
    required_checks: {
      approved_production_map_source: true,
      production_map_source_artifact_status: "missing",
      license_and_odbl_compliance: true,
      live_ingestion_disabled: true,
      offline_cache_policy: true,
      cache_retention_refresh_policy: null,
      direct_public_osm_client_fetch_forbidden: true,
      web_public_attribution_status: "",
      web_public_attribution_artifact_status: "missing",
      native_bevy_android_attribution_status: "",
      native_bevy_android_attribution_artifact_status: "missing",
      matrix_or_readonly_attribution_status: "",
      matrix_or_readonly_attribution_artifact_status: "missing",
      sensitive_poi_filter_status: "",
      sensitive_poi_report_artifact_status: "missing",
      geofence_policy_status: "",
      geofence_policy_artifact_status: "missing",
      key_custody_status: "",
      key_rotation_runbook_artifact_status: "missing",
      distribution_revocation_status: "",
      public_distribution_package_artifact_status: "missing",
      revocation_probe_artifact_status: "missing",
      rollback_status: "",
      rollback_evidence_artifact_status: "missing"
    }
  }' >"$production_evidence_json"
  add_artifact_from_path production_map_pack_public_evidence "Production map-pack public evidence" "$production_evidence_json" release_review_input

  jq -n '{
    contract_version: "trillionnium_world_cohort_commercial_evidence_gate_v1",
    status: "blocked_missing_cohort_commercial_real_evidence",
    generated_at: "2026-06-07T00:00:00Z",
    source_of_truth: "trillionnium_world_cohort_commercial_evidence_gate",
    green: false,
    cohort_commercial_ready: false,
    first_beta_ready: false,
    commercial_launch_drill_ready: false,
    blocker_count: 21,
    first_beta_blocker_count: 9,
    commercial_launch_drill_blocker_count: 12,
    required_drill_count: 6,
    failed_drill_count: 6,
    public_launch_credit: "only_when_first_beta_and_commercial_statuses_are_green_after_field_validation",
    schema: {
      summary_path: "/fixture/cohort-commercial-evidence-schema.json",
      refresh_log_path: "/fixture/cohort-commercial-evidence-schema-refresh.log"
    },
    first_beta: {
      status: "blocked_missing_first_beta_cohort_evidence",
      accepted_status: "first_beta_cohort_evidence_green",
      operator_evidence: {path: null, file_status: "missing", contract_version: "", status: ""},
      participant_count: 0,
      participants_len: 0,
      sessions_len: 0,
      blockers: [
        "first_beta_evidence_file",
        "first_beta_contract",
        "first_beta_status",
        "participant_count_5_to_10",
        "participants_match_count",
        "session_count_covers_participants",
        "real_participants_signoff",
        "synthetic_cohort_rejected",
        "first_beta_operator_signature"
      ]
    },
    commercial_launch_drill: {
      status: "blocked_missing_commercial_launch_drill_evidence",
      accepted_status: "commercial_launch_drill_evidence_green",
      operator_evidence: {path: null, file_status: "missing", contract_version: "", status: ""},
      required_drills: [
        {drill: "payment", status: "missing", evidence: null, green: false},
        {drill: "refund", status: "missing", evidence: null, green: false},
        {drill: "support", status: "missing", evidence: null, green: false},
        {drill: "legal", status: "missing", evidence: null, green: false},
        {drill: "operator", status: "missing", evidence: null, green: false},
        {drill: "traffic", status: "missing", evidence: null, green: false}
      ],
      blockers: [
        "commercial_evidence_file",
        "commercial_contract",
        "commercial_status",
        "payment_drill_green_evidence",
        "refund_drill_green_evidence",
        "support_drill_green_evidence",
        "legal_drill_green_evidence",
        "operator_drill_green_evidence",
        "traffic_drill_green_evidence",
        "real_or_sanitized_commercial_signoff",
        "synthetic_commercial_rejected",
        "commercial_operator_signature"
      ]
    }
  }' >"$cohort_evidence_json"
  add_artifact_from_path cohort_commercial_evidence "Cohort/commercial evidence validation" "$cohort_evidence_json" release_review_input

  jq -n '{
    contract_version: "trillionnium_world_external_ops_evidence_gate_v1",
    status: "blocked_missing_external_ops_real_evidence",
    generated_at: "2026-06-07T00:00:00Z",
    source_of_truth: "trillionnium_world_external_ops_evidence_gate",
    green: false,
    external_ops_ready: false,
    multi_node_or_live_traffic_latency_ready: false,
    public_network_deploy_ready: false,
    public_launch_credit: "only_when_multi_node_or_live_traffic_and_public_network_deploy_statuses_are_green_after_field_validation",
    local_drill_rule: "local_release_load_drill_only_not_multi_node_or_live_traffic",
    live_public_exposure_performed_by_this_script: false,
    blocker_count: 27,
    multi_node_or_live_traffic_latency_blocker_count: 13,
    public_network_deploy_blocker_count: 14,
    template_count: 2,
    local_drill_count: 2,
    multi_node_or_live_traffic_latency: {
      status: "blocked_missing_multi_node_or_live_traffic_latency_evidence",
      accepted_status: "multi_node_or_live_traffic_latency_green",
      operator_evidence: {path: null, file_status: "missing", contract_version: "", status: ""},
      local_drill: {path: "/fixture/release-latency-drill.json", file_status: "present", status: "local_release_latency_drill_green", public_launch_credit: false},
      node_count: 0,
      endpoint_count: 0,
      public_url_probe_sample_count: 0,
      blockers: [
        "multi_node_latency_evidence_file",
        "multi_node_latency_contract",
        "multi_node_latency_status",
        "multi_node_or_live_traffic_scope_confirmed",
        "multi_node_count_or_live_traffic",
        "latency_endpoint_count",
        "public_url_probe_samples",
        "p95_latency_budget",
        "monitoring_timeseries_evidence",
        "rollback_under_load",
        "real_multi_node_or_live_traffic_signoff",
        "synthetic_latency_rejected",
        "latency_operator_signature"
      ]
    },
    public_network_deploy: {
      status: "blocked_missing_public_network_live_exposure_evidence",
      accepted_status: "public_network_deploy_green",
      operator_evidence: {path: null, file_status: "missing", contract_version: "", status: ""},
      local_drill: {path: "/fixture/public-network-deploy-evidence.json", file_status: "present", status: "local_public_deploy_drill_green", public_launch_credit: false},
      public_url_probe_sample_count: 0,
      blockers: [
        "public_network_deploy_evidence_file",
        "public_network_deploy_contract",
        "public_network_deploy_status",
        "public_network_exposure_approved",
        "host_domain_public_url",
        "tls_certificate",
        "public_url_probe_samples",
        "public_url_health_probe",
        "monitoring_alerts",
        "backup_restore",
        "public_deploy_rollback",
        "public_exposure_signoff",
        "synthetic_deploy_rejected",
        "public_deploy_operator_signature"
      ]
    },
    templates: {
      multi_node_or_live_traffic_latency: "/fixture/multi-node-latency-evidence.template.json",
      public_network_deploy: "/fixture/public-network-deploy-evidence.template.json"
    }
  }' >"$external_ops_evidence_json"
  add_artifact_from_path external_ops_evidence "External ops evidence validation" "$external_ops_evidence_json" release_review_input
}

add_public_launch_s5_real_device_validator_packet_fixtures() {
  local s5_real_device_evidence_json="$TMP_DIR/s5-real-device-evidence-validation.json"

  jq -n '{
    contract_version: "trillionnium_world_s5_real_device_evidence_gate_v1",
    status: "blocked_missing_s5_real_device_evidence",
    generated_at: "2026-06-07T00:00:00Z",
    source_of_truth: "trillionnium_world_s5_real_device_evidence_gate",
    green: false,
    accepted_status: "s5_real_device_evidence_green",
    android_s5_real_device_claimed: false,
    host_side_replay_credit: false,
    blocker_count: 13,
    template_count: 1,
    host_artifact_count: 2,
    real_device_evidence_field_count: 9,
    present_real_device_evidence_count: 2,
    missing_real_device_evidence_count: 7,
    go_condition_count: 4,
    blocked_go_condition_count: 3,
    template: {
      path: "/fixture/s5-device-evidence.template.json",
      sha256: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
      status: "template_requires_real_s5_device_evidence",
      public_launch_credit: false
    },
    operator_evidence: {
      path: "/fixture/s5-device-evidence.json",
      file_status: "present",
      contract_version: "trillionnium_world_s5_native_bevy_device_evidence_v1",
      overall_status: "blocked_no_connected_android_device"
    },
    native_lib: {
      status: "android_native_cdylib_ready",
      path: "/fixture/libtrnm_world_bevy.so",
      symbols_evidence: "/fixture/native-lib-symbols.txt"
    },
    apk: {
      status: "signed_debug_apk_ready",
      path: "/fixture/trillionnium-world-bevy-debug.apk"
    },
    real_device_matrix: {
      status: "blocked_no_connected_android_device",
      device_serial: null,
      adb_devices_evidence: "/fixture/adb-devices.txt",
      screenshot_evidence: "",
      gfxinfo_evidence: "",
      logcat_evidence: "",
      lifecycle_evidence: "",
      locale_evidence: "",
      input_method_evidence: "",
      weak_network_evidence: "",
      resource_pack_evidence: "/fixture/apk-package-evidence.txt",
      cjk_display_input_gate: "requires_real_device_cjk_locale_input_evidence",
      weak_network_gate: "requires_real_device_weak_network_run",
      resource_pack_gate: "apk_signature_resource_pack_evidence_collected",
      crash_free_gate: "not_run_no_device"
    },
    go_condition_matrix: {
      cjk_display_input_gate: "requires_real_device_cjk_locale_input_evidence",
      weak_network_gate: "requires_real_device_weak_network_run",
      resource_pack_gate: "apk_signature_resource_pack_evidence_collected",
      crash_free_gate: "not_run_no_device",
      accepted_cjk_display_input_gate: "cjk_locale_input_snapshot_collected",
      accepted_weak_network_gate: "real_device_weak_network_run",
      accepted_resource_pack_gate: "apk_signature_resource_pack_evidence_collected"
    },
    blockers: [
      "s5_overall_ready",
      "real_device_evidence_collected",
      "real_device_serial",
      "real_device_screenshot",
      "real_device_gfxinfo_or_frame_stats",
      "real_device_logcat",
      "real_device_lifecycle",
      "real_device_cjk_locale",
      "real_device_input_method",
      "real_device_cjk_display_input",
      "real_device_weak_network_evidence",
      "real_device_weak_network",
      "crash_free_logcat_window"
    ]
  }' >"$s5_real_device_evidence_json"
  add_artifact_from_path s5_real_device_evidence "S5 real-device evidence validation" "$s5_real_device_evidence_json" release_review_input
}

add_public_launch_evidence_intake_packet_fixtures() {
  local evidence_intake_json="$TMP_DIR/public-launch-evidence-intake.json"
  jq -n '{
    contract_version: "trillionnium_world_public_launch_evidence_intake_v1",
    status: "public_launch_evidence_intake_ready_for_operator_collection",
    green: true,
    source_of_truth: "trillionnium_world_public_launch_evidence_intake",
    public_launch_readiness_summary: "/fixture/public-launch-readiness.json",
    public_launch_readiness_log: "/fixture/public-launch-evidence-intake-readiness.log",
    public_launch_readiness_status: "blocked_missing_public_launch_evidence",
    markdown_path: "/fixture/public-launch-evidence-intake.md",
    complete: false,
    public_launch_ready: false,
    public_launch_claimed: false,
    android_s5_real_device_claimed: false,
    live_map_ingestion_performed: false,
    live_public_exposure_performed: false,
    intake_rule: "collect_real_external_public_launch_evidence_without_claiming_public_launch_ready_or_android_s5_real_device_ready",
    blocker_count: 6,
    blockers: [
      "s5_real_device_matrix",
      "production_map_pack_public_evidence",
      "first_beta_cohort_evidence",
      "commercial_launch_drill_evidence",
      "multi_node_or_live_traffic_latency_evidence",
      "public_network_live_exposure_evidence"
    ],
    unknown_blocker_count: 0,
    unknown_blockers: [],
    evidence_item_count: 6,
    evidence_items: [
      {id: "s5_android_real_device_matrix", label: "S5 Android real-device matrix", blocker_id: "s5_real_device_matrix", evidence_env_var: "ANDROID_SERIAL", accepted_status: "real_device_evidence_green", current_status: "blocked_missing_s5_real_device_evidence", file_status: "present", green: false, blocked_by_public_launch_gate: true, evidence_path: "/fixture/s5-device-evidence.json", collection_command: "ANDROID_SERIAL=<device-serial> scripts/check_trillionnium_world_s5_device_evidence.sh --require-device", template_path: "/fixture/s5-device-evidence.template.json"},
      {id: "production_map_pack_public_evidence", label: "Production map-pack public evidence", blocker_id: "production_map_pack_public_evidence", evidence_env_var: "TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH", accepted_status: "production_map_pack_public_ready_green", current_status: "blocked_missing_production_map_pack_public_evidence", file_status: "present", green: false, blocked_by_public_launch_gate: true, evidence_path: "/fixture/production-map-pack-public-evidence.json", collection_command: "scripts/check_trillionnium_world_production_map_pack_public_evidence_collection.sh", template_path: "/fixture/production-map-pack-public-evidence.template.json"},
      {id: "first_beta_cohort_evidence", label: "First beta cohort evidence", blocker_id: "first_beta_cohort_evidence", evidence_env_var: "TRILLIONNIUM_FIRST_BETA_COHORT_EVIDENCE_PATH", accepted_status: "first_beta_cohort_evidence_green", current_status: "blocked_missing_first_beta_cohort_evidence", file_status: "missing", green: false, blocked_by_public_launch_gate: true, evidence_path: null, collection_command: "scripts/check_trillionnium_world_cohort_commercial_evidence_collection.sh", template_path: "/fixture/first-beta-cohort-evidence.template.json"},
      {id: "commercial_launch_drill_evidence", label: "Commercial launch drill evidence", blocker_id: "commercial_launch_drill_evidence", evidence_env_var: "TRILLIONNIUM_COMMERCIAL_LAUNCH_DRILL_EVIDENCE_PATH", accepted_status: "commercial_launch_drill_evidence_green", current_status: "blocked_missing_commercial_launch_drill_evidence", file_status: "missing", green: false, blocked_by_public_launch_gate: true, evidence_path: null, collection_command: "scripts/check_trillionnium_world_cohort_commercial_evidence_collection.sh", template_path: "/fixture/commercial-launch-drill-evidence.template.json"},
      {id: "multi_node_or_live_traffic_latency_evidence", label: "Multi-node or live-traffic latency evidence", blocker_id: "multi_node_or_live_traffic_latency_evidence", evidence_env_var: "TRILLIONNIUM_MULTI_NODE_LATENCY_EVIDENCE_PATH", accepted_status: "multi_node_or_live_traffic_latency_green", current_status: "blocked_missing_multi_node_or_live_traffic_latency_evidence", file_status: "missing", green: false, blocked_by_public_launch_gate: true, evidence_path: null, collection_command: "scripts/check_trillionnium_world_external_ops_evidence_collection.sh", template_path: "/fixture/multi-node-latency-evidence.template.json"},
      {id: "public_network_live_exposure_evidence", label: "Public network live exposure evidence", blocker_id: "public_network_live_exposure_evidence", evidence_env_var: "TRILLIONNIUM_PUBLIC_NETWORK_DEPLOY_EVIDENCE_PATH", accepted_status: "public_network_deploy_green", current_status: "blocked_missing_public_network_live_exposure_evidence", file_status: "missing", green: false, blocked_by_public_launch_gate: true, evidence_path: null, collection_command: "scripts/check_trillionnium_world_external_ops_evidence_collection.sh", template_path: "/fixture/public-deploy-runbook.md"}
    ],
    needs_collection_count: 6,
    needs_collection: [
      {id: "s5_android_real_device_matrix", current_status: "blocked_missing_s5_real_device_evidence", accepted_status: "real_device_evidence_green", file_status: "present", green: false, evidence_env_var: "ANDROID_SERIAL", collection_command: "ANDROID_SERIAL=<device-serial> scripts/check_trillionnium_world_s5_device_evidence.sh --require-device", template_path: "/fixture/s5-device-evidence.template.json"},
      {id: "production_map_pack_public_evidence", current_status: "blocked_missing_production_map_pack_public_evidence", accepted_status: "production_map_pack_public_ready_green", file_status: "present", green: false, evidence_env_var: "TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH", collection_command: "scripts/check_trillionnium_world_production_map_pack_public_evidence_collection.sh", template_path: "/fixture/production-map-pack-public-evidence.template.json"},
      {id: "first_beta_cohort_evidence", current_status: "blocked_missing_first_beta_cohort_evidence", accepted_status: "first_beta_cohort_evidence_green", file_status: "missing", green: false, evidence_env_var: "TRILLIONNIUM_FIRST_BETA_COHORT_EVIDENCE_PATH", collection_command: "scripts/check_trillionnium_world_cohort_commercial_evidence_collection.sh", template_path: "/fixture/first-beta-cohort-evidence.template.json"},
      {id: "commercial_launch_drill_evidence", current_status: "blocked_missing_commercial_launch_drill_evidence", accepted_status: "commercial_launch_drill_evidence_green", file_status: "missing", green: false, evidence_env_var: "TRILLIONNIUM_COMMERCIAL_LAUNCH_DRILL_EVIDENCE_PATH", collection_command: "scripts/check_trillionnium_world_cohort_commercial_evidence_collection.sh", template_path: "/fixture/commercial-launch-drill-evidence.template.json"},
      {id: "multi_node_or_live_traffic_latency_evidence", current_status: "blocked_missing_multi_node_or_live_traffic_latency_evidence", accepted_status: "multi_node_or_live_traffic_latency_green", file_status: "missing", green: false, evidence_env_var: "TRILLIONNIUM_MULTI_NODE_LATENCY_EVIDENCE_PATH", collection_command: "scripts/check_trillionnium_world_external_ops_evidence_collection.sh", template_path: "/fixture/multi-node-latency-evidence.template.json"},
      {id: "public_network_live_exposure_evidence", current_status: "blocked_missing_public_network_live_exposure_evidence", accepted_status: "public_network_deploy_green", file_status: "missing", green: false, evidence_env_var: "TRILLIONNIUM_PUBLIC_NETWORK_DEPLOY_EVIDENCE_PATH", collection_command: "scripts/check_trillionnium_world_external_ops_evidence_collection.sh", template_path: "/fixture/public-deploy-runbook.md"}
    ],
    green_evidence_item_count: 0,
    blocked_evidence_item_count: 6,
    present_evidence_item_count: 2,
    missing_evidence_item_count: 4,
    reviewer_next_action: "collect_evidence_items_in_needs_collection"
  }' >"$evidence_intake_json"
  add_artifact_from_path public_launch_evidence_intake "Public launch evidence intake" "$evidence_intake_json" release_review_input

  local evidence_intake_md="$TMP_DIR/public-launch-evidence-intake.md"
  {
    printf '# Trillionnium World Public Launch Evidence Intake\n\n'
    printf -- '- status: public_launch_evidence_intake_ready_for_operator_collection\n'
    printf -- '- green: true\n'
    printf -- '- public_launch_ready: false\n'
    printf -- '- public_launch_claimed: false\n'
    printf -- '- android_s5_real_device_claimed: false\n'
    printf -- '- live_map_ingestion_performed: false\n'
    printf -- '- live_public_exposure_performed: false\n\n'
    printf -- '- evidence_item_count: 6\n'
    printf -- '- needs_collection_count: 6\n'
    printf -- '- blocker_count: 6\n\n'
    printf '## Evidence To Collect\n\n'
    printf -- '- [ ] S5 Android real-device matrix (real_device_evidence_green)\n'
    printf -- '  - env: ANDROID_SERIAL\n'
    printf -- '  - current_status: blocked_missing_s5_real_device_evidence\n'
    printf -- '  - collect: ANDROID_SERIAL=<device-serial> scripts/check_trillionnium_world_s5_device_evidence.sh --require-device\n'
    printf -- '- [ ] Production map-pack public evidence (production_map_pack_public_ready_green)\n'
    printf -- '  - env: TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH\n'
    printf -- '  - current_status: blocked_missing_production_map_pack_public_evidence\n'
    printf -- '- [ ] First beta cohort evidence (first_beta_cohort_evidence_green)\n'
    printf -- '- [ ] Commercial launch drill evidence (commercial_launch_drill_evidence_green)\n'
    printf -- '- [ ] Multi-node or live-traffic latency evidence (multi_node_or_live_traffic_latency_green)\n'
    printf -- '- [ ] Public network live exposure evidence (public_network_deploy_green)\n'
    printf -- '  - env: TRILLIONNIUM_PUBLIC_NETWORK_DEPLOY_EVIDENCE_PATH\n'
    printf -- '  - current_status: blocked_missing_public_network_live_exposure_evidence\n\n'
    printf '## Evidence Already Green\n\n'
    printf -- '- [ ] No external public-launch evidence item is green yet.\n\n'
    printf '## Boundary\n\n'
    printf -- '- This is an intake/checklist artifact, not a public-launch approval.\n'
    printf -- '- Live map ingestion and live public exposure are not performed by this script.\n'
  } >"$evidence_intake_md"
  add_artifact_from_path public_launch_evidence_intake_markdown "Public launch evidence intake Markdown" "$evidence_intake_md" release_review_input
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

add_public_launch_evidence_kit_packet_fixtures() {
  local evidence_kit_json="$TMP_DIR/public-launch-evidence-kit.json"
  jq -n '{
    contract_version: "trillionnium_world_public_launch_evidence_kit_v1",
    status: "public_launch_evidence_kit_ready_for_operator_collection",
    source_of_truth: "trillionnium_world_public_launch_evidence_kit",
    green: true,
    public_launch_ready: false,
    public_launch_claimed: false,
    android_s5_real_device_claimed: false,
    live_map_ingestion_performed: false,
    live_public_exposure_performed: false,
    kit_rule: "operator_templates_must_exist_and_must_not_claim_green_until_real_external_evidence_passes_field_validators",
    markdown_path: "/fixture/public-launch-evidence-kit.md",
    intake_summary: "/fixture/public-launch-evidence-intake.json",
    refresh_logs: {
      s5_real_device: "/fixture/public-launch-evidence-kit-s5.log",
      production_map_pack: "/fixture/public-launch-evidence-kit-map-pack.log",
      cohort_schema: "/fixture/public-launch-evidence-kit-cohort-schema.log",
      external_ops: "/fixture/public-launch-evidence-kit-external-ops.log",
      evidence_intake: "/fixture/public-launch-evidence-kit-intake.log"
    },
    needs_collection_count: 6,
    evidence_item_count: 6,
    ready_template_count: 6,
    template_failure_count: 0,
    refresh_log_count: 5,
    evidence_items: [
      {id: "s5_android_real_device_matrix", blocker_id: "s5_real_device_matrix", evidence_env_var: "ANDROID_SERIAL", accepted_status: "s5_real_device_evidence_green", current_status: "blocked_missing_s5_real_device_evidence", template_status: "template_requires_real_s5_device_evidence", template_ok: true, collection_command: "ANDROID_SERIAL=<device-serial> scripts/check_trillionnium_world_s5_device_evidence.sh --require-device", validator_command: "TRILLIONNIUM_WORLD_S5_REAL_DEVICE_EVIDENCE_PATH=<real-s5-evidence.json> scripts/check_trillionnium_world_s5_real_device_evidence.sh --require-ready", template_public_launch_credit: false},
      {id: "production_map_pack_public_evidence", blocker_id: "production_map_pack_public_evidence", evidence_env_var: "TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH", accepted_status: "production_map_pack_public_ready_green", current_status: "blocked_missing_production_map_pack_public_evidence", template_status: "template_requires_real_public_map_pack_evidence", template_ok: true, collection_command: "scripts/check_trillionnium_world_production_map_pack_public_evidence_collection.sh", validator_command: "TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH=<real-map-pack-evidence.json> scripts/check_trillionnium_world_production_map_pack_public_evidence.sh --require-ready", template_public_launch_credit: false},
      {id: "first_beta_cohort_evidence", blocker_id: "first_beta_cohort_evidence", evidence_env_var: "TRILLIONNIUM_FIRST_BETA_COHORT_EVIDENCE_PATH", accepted_status: "first_beta_cohort_evidence_green", current_status: "blocked_missing_first_beta_cohort_evidence", template_status: "template_requires_real_participants", template_ok: true, collection_command: "scripts/check_trillionnium_world_cohort_commercial_evidence_collection.sh", validator_command: "TRILLIONNIUM_FIRST_BETA_COHORT_EVIDENCE_PATH=<real-cohort.json> scripts/check_trillionnium_world_cohort_commercial_evidence.sh --require-ready", template_public_launch_credit: false},
      {id: "commercial_launch_drill_evidence", blocker_id: "commercial_launch_drill_evidence", evidence_env_var: "TRILLIONNIUM_COMMERCIAL_LAUNCH_DRILL_EVIDENCE_PATH", accepted_status: "commercial_launch_drill_evidence_green", current_status: "blocked_missing_commercial_launch_drill_evidence", template_status: "template_requires_real_drill", template_ok: true, collection_command: "scripts/check_trillionnium_world_cohort_commercial_evidence_collection.sh", validator_command: "TRILLIONNIUM_COMMERCIAL_LAUNCH_DRILL_EVIDENCE_PATH=<real-commercial-drill.json> scripts/check_trillionnium_world_cohort_commercial_evidence.sh --require-ready", template_public_launch_credit: false},
      {id: "multi_node_or_live_traffic_latency_evidence", blocker_id: "multi_node_or_live_traffic_latency_evidence", evidence_env_var: "TRILLIONNIUM_MULTI_NODE_LATENCY_EVIDENCE_PATH", accepted_status: "multi_node_or_live_traffic_latency_green", current_status: "blocked_missing_multi_node_or_live_traffic_latency_evidence", template_status: "template_requires_multi_node_or_live_traffic_latency", template_ok: true, collection_command: "scripts/check_trillionnium_world_external_ops_evidence_collection.sh", validator_command: "TRILLIONNIUM_MULTI_NODE_LATENCY_EVIDENCE_PATH=<real-latency.json> scripts/check_trillionnium_world_external_ops_evidence.sh --require-ready", template_public_launch_credit: false},
      {id: "public_network_live_exposure_evidence", blocker_id: "public_network_live_exposure_evidence", evidence_env_var: "TRILLIONNIUM_PUBLIC_NETWORK_DEPLOY_EVIDENCE_PATH", accepted_status: "public_network_deploy_green", current_status: "blocked_missing_public_network_live_exposure_evidence", template_status: "template_requires_public_network_deploy", template_ok: true, collection_command: "scripts/check_trillionnium_world_external_ops_evidence_collection.sh", validator_command: "TRILLIONNIUM_PUBLIC_NETWORK_DEPLOY_EVIDENCE_PATH=<real-public-deploy.json> scripts/check_trillionnium_world_external_ops_evidence.sh --require-ready", template_public_launch_credit: false}
    ],
    template_failures: [],
    reviewer_next_action: "collect_real_external_public_launch_evidence_using_templates"
  }' >"$evidence_kit_json"
  add_artifact_from_path public_launch_evidence_kit "Public launch evidence kit" "$evidence_kit_json" release_review_gate

  local evidence_kit_md="$TMP_DIR/public-launch-evidence-kit.md"
  {
    printf '# Trillionnium World Public Launch Evidence Kit\n\n'
    printf -- '- status: public_launch_evidence_kit_ready_for_operator_collection\n'
    printf -- '- public_launch_ready: false\n'
    printf -- '- public_launch_claimed: false\n'
    printf -- '- android_s5_real_device_claimed: false\n\n'
    printf -- '- evidence_item_count: 6\n'
    printf -- '- ready_template_count: 6\n'
    printf -- '- template_failure_count: 0\n'
    printf -- '- refresh_log_count: 5\n\n'
    printf '## Evidence Templates\n\n'
    printf -- '- s5_android_real_device_matrix: /fixture/s5-device-evidence.template.json\n'
    printf -- '  - env: ANDROID_SERIAL\n'
    printf -- '  - accepted_status: s5_real_device_evidence_green\n'
    printf -- '  - current_status: blocked_missing_s5_real_device_evidence\n'
    printf -- '  - validator: TRILLIONNIUM_WORLD_S5_REAL_DEVICE_EVIDENCE_PATH=<real-s5-evidence.json> scripts/check_trillionnium_world_s5_real_device_evidence.sh --require-ready\n'
    printf -- '- production_map_pack_public_evidence: /fixture/production-map-pack-public-evidence.template.json\n'
    printf -- '  - env: TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH\n'
    printf -- '  - accepted_status: production_map_pack_public_ready_green\n'
    printf -- '  - current_status: blocked_missing_production_map_pack_public_evidence\n'
    printf -- '- public_network_live_exposure_evidence: /fixture/public-network-deploy-evidence.template.json\n'
    printf -- '  - accepted_status: public_network_deploy_green\n\n'
    printf '## Boundary\n\n'
    printf -- '- Templates are collection scaffolding only and carry no public-launch credit.\n'
    printf -- '- Public launch stays blocked until each real evidence file passes its field-level validator.\n'
  } >"$evidence_kit_md"
  add_artifact_from_path public_launch_evidence_kit_markdown "Public launch evidence kit Markdown" "$evidence_kit_md" release_review_gate
}

add_public_launch_evidence_bundle_packet_fixtures() {
  local evidence_bundle_json="$TMP_DIR/public-launch-evidence-bundle.json"
  jq -n '{
    contract_version: "trillionnium_world_public_launch_evidence_bundle_gate_v1",
    status: "public_launch_evidence_bundle_ready_for_real_evidence",
    source_of_truth: "trillionnium_world_public_launch_evidence_bundle_gate",
    green: false,
    public_launch_ready: false,
    public_launch_claimed: false,
    android_s5_real_device_claimed: false,
    live_map_ingestion_performed_by_this_script: false,
    live_public_exposure_performed_by_this_script: false,
    bundle_rule: "single_manifest_must_point_to_real_external_evidence_that_passes_all_field_validators_before_public_launch_credit",
    evidence_item_count: 6,
    item_failure_count: 6,
    green_evidence_item_count: 0,
    present_evidence_item_count: 0,
    missing_evidence_item_count: 6,
    validator_count: 4,
    failed_validator_count: 4,
    evidence_bundle: {
      path: null,
      file_status: "missing",
      contract_version: "",
      status: "",
      metadata_ok: false,
      signoff_ok: false
    },
    template: {
      path: "/fixture/public-launch-evidence-bundle.template.json",
      sha256: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
      public_launch_credit: false
    },
    markdown_path: "/fixture/public-launch-evidence-bundle.md",
    evidence_kit_log: "/fixture/public-launch-evidence-bundle-kit.log",
    evidence_items: [
      {id: "s5_real_device", label: "S5 Android real-device evidence", green: false, evidence_path: null, file_status: "missing", evidence_env_var: "TRILLIONNIUM_WORLD_S5_REAL_DEVICE_EVIDENCE_PATH", validator_name: "s5_real_device", validator_summary: "/fixture/public-launch-evidence-bundle-s5-real-device.json", actual_status: "blocked_missing_s5_real_device_evidence", accepted_status: "s5_real_device_evidence_green"},
      {id: "production_map_pack_public", label: "Production map-pack public evidence", green: false, evidence_path: null, file_status: "missing", evidence_env_var: "TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH", validator_name: "production_map_pack", validator_summary: "/fixture/public-launch-evidence-bundle-production-map-pack.json", actual_status: "blocked_missing_production_map_pack_public_evidence", accepted_status: "production_map_pack_public_ready_green"},
      {id: "first_beta_cohort", label: "First beta cohort evidence", green: false, evidence_path: null, file_status: "missing", evidence_env_var: "TRILLIONNIUM_FIRST_BETA_COHORT_EVIDENCE_PATH", validator_name: "cohort_commercial", validator_summary: "/fixture/public-launch-evidence-bundle-cohort-commercial.json", actual_status: "blocked_missing_first_beta_cohort_evidence", accepted_status: "first_beta_cohort_evidence_green"},
      {id: "commercial_launch_drill", label: "Commercial launch drill evidence", green: false, evidence_path: null, file_status: "missing", evidence_env_var: "TRILLIONNIUM_COMMERCIAL_LAUNCH_DRILL_EVIDENCE_PATH", validator_name: "cohort_commercial", validator_summary: "/fixture/public-launch-evidence-bundle-cohort-commercial.json", actual_status: "blocked_missing_commercial_launch_drill_evidence", accepted_status: "commercial_launch_drill_evidence_green"},
      {id: "multi_node_or_live_traffic_latency", label: "Multi-node or live-traffic latency evidence", green: false, evidence_path: null, file_status: "missing", evidence_env_var: "TRILLIONNIUM_MULTI_NODE_LATENCY_EVIDENCE_PATH", validator_name: "external_ops", validator_summary: "/fixture/public-launch-evidence-bundle-external-ops.json", actual_status: "blocked_missing_multi_node_or_live_traffic_latency_evidence", accepted_status: "multi_node_or_live_traffic_latency_green"},
      {id: "public_network_deploy", label: "Public network deploy evidence", green: false, evidence_path: null, file_status: "missing", evidence_env_var: "TRILLIONNIUM_PUBLIC_NETWORK_DEPLOY_EVIDENCE_PATH", validator_name: "external_ops", validator_summary: "/fixture/public-launch-evidence-bundle-external-ops.json", actual_status: "blocked_missing_public_network_live_exposure_evidence", accepted_status: "public_network_deploy_green"}
    ],
    item_failures: [
      {id: "s5_real_device", actual_status: "blocked_missing_s5_real_device_evidence"},
      {id: "production_map_pack_public", actual_status: "blocked_missing_production_map_pack_public_evidence"},
      {id: "first_beta_cohort", actual_status: "blocked_missing_first_beta_cohort_evidence"},
      {id: "commercial_launch_drill", actual_status: "blocked_missing_commercial_launch_drill_evidence"},
      {id: "multi_node_or_live_traffic_latency", actual_status: "blocked_missing_multi_node_or_live_traffic_latency_evidence"},
      {id: "public_network_deploy", actual_status: "blocked_missing_public_network_live_exposure_evidence"}
    ],
    validators: [
      {name: "s5_real_device", log_path: "/fixture/public-launch-evidence-bundle-s5-real-device.log", exit_status: 1},
      {name: "production_map_pack", log_path: "/fixture/public-launch-evidence-bundle-production-map-pack.log", exit_status: 1},
      {name: "cohort_commercial", log_path: "/fixture/public-launch-evidence-bundle-cohort-commercial.log", exit_status: 1},
      {name: "external_ops", log_path: "/fixture/public-launch-evidence-bundle-external-ops.log", exit_status: 1}
    ]
  }' >"$evidence_bundle_json"
  add_artifact_from_path public_launch_evidence_bundle "Public launch evidence bundle" "$evidence_bundle_json" release_review_gate

  local evidence_bundle_md="$TMP_DIR/public-launch-evidence-bundle.md"
  {
    printf '# Trillionnium World Public Launch Evidence Bundle\n\n'
    printf -- '- status: public_launch_evidence_bundle_ready_for_real_evidence\n'
    printf -- '- public_launch_ready: false\n'
    printf -- '- evidence_item_count: 6\n'
    printf -- '- item_failure_count: 6\n'
    printf -- '- validator_count: 4\n'
    printf -- '- failed_validator_count: 4\n'
    printf -- '- bundle_path: missing\n'
    printf -- '- template: /fixture/public-launch-evidence-bundle.template.json\n\n'
    printf '## Evidence Items\n\n'
    printf -- '- s5_real_device: blocked_missing_s5_real_device_evidence (accepted: s5_real_device_evidence_green)\n'
    printf -- '- production_map_pack_public: blocked_missing_production_map_pack_public_evidence (accepted: production_map_pack_public_ready_green)\n'
    printf -- '- first_beta_cohort: blocked_missing_first_beta_cohort_evidence (accepted: first_beta_cohort_evidence_green)\n'
    printf -- '- commercial_launch_drill: blocked_missing_commercial_launch_drill_evidence (accepted: commercial_launch_drill_evidence_green)\n'
    printf -- '- multi_node_or_live_traffic_latency: blocked_missing_multi_node_or_live_traffic_latency_evidence (accepted: multi_node_or_live_traffic_latency_green)\n'
    printf -- '- public_network_deploy: blocked_missing_public_network_live_exposure_evidence (accepted: public_network_deploy_green)\n\n'
    printf '## Boundary\n\n'
    printf -- '- This script validates a manifest only; it does not collect real external evidence.\n'
    printf -- '- Public launch credit requires the bundle status and all six field validators to be green.\n'
  } >"$evidence_bundle_md"
  add_artifact_from_path public_launch_evidence_bundle_markdown "Public launch evidence bundle Markdown" "$evidence_bundle_md" release_review_gate
}

add_public_launch_bundle_negative_fixtures_packet_fixtures() {
  local bundle_negative_json="$TMP_DIR/public-launch-bundle-negative-fixtures.json"
  jq -n '{
    contract_version: "trillionnium_world_public_launch_bundle_negative_fixtures_v1",
    status: "public_launch_bundle_negative_fixtures_green",
    source_of_truth: "trillionnium_world_public_launch_bundle_negative_fixtures",
    green: true,
    public_launch_claimed: false,
    android_s5_real_device_claimed: false,
    live_map_ingestion_performed: false,
    live_public_exposure_performed: false,
    bundle_negative_rule: "fake_green_bundle_manifest_pointing_to_no_credit_templates_must_fail_require_ready",
    fake_bundle_path: "/fixture/fake-green-template-bundle.json",
    evidence_kit_log: "/fixture/public-launch-bundle-negative-fixtures-kit.log",
    bundle_validation_summary: "/fixture/fake-green-template-bundle-summary.json",
    bundle_validation_log: "/fixture/public-launch-bundle-negative-fixtures-bundle.log",
    expected_status: "public_launch_evidence_bundle_blocked_invalid_real_evidence",
    actual_status: "public_launch_evidence_bundle_blocked_invalid_real_evidence",
    validator_exit_status: 1,
    expected_item_failure_count: 6,
    actual_item_failure_count: 6
  }' >"$bundle_negative_json"
  add_artifact_from_path public_launch_bundle_negative_fixtures "Public launch bundle negative fixtures" "$bundle_negative_json" release_review_gate
}

add_public_launch_template_negative_fixtures_packet_fixtures() {
  local template_negative_json="$TMP_DIR/public-launch-template-negative-fixtures.json"
  jq -n '{
    contract_version: "trillionnium_world_public_launch_template_negative_fixtures_v1",
    status: "public_launch_template_negative_fixtures_green",
    source_of_truth: "trillionnium_world_public_launch_template_negative_fixtures",
    green: true,
    public_launch_claimed: false,
    android_s5_real_device_claimed: false,
    live_map_ingestion_performed: false,
    live_public_exposure_performed: false,
    template_negative_rule: "no_credit_templates_must_fail_strict_field_validators_before_public_launch_handoff",
    evidence_kit_log: "/fixture/public-launch-template-negative-fixtures-kit.log",
    result_count: 4,
    template_count: 6,
    results: [
      {name: "s5_real_device_template", validator: "check_trillionnium_world_s5_real_device_evidence", summary_path: "/fixture/s5-real-device-template-summary.json", expected_status: "blocked_missing_s5_real_device_evidence", actual_status: "blocked_missing_s5_real_device_evidence", template_paths: ["/fixture/s5-device-evidence.template.json"], exit_status: 1, rejected: true},
      {name: "production_map_pack_template", validator: "check_trillionnium_world_production_map_pack_public_evidence", summary_path: "/fixture/production-map-pack-template-summary.json", expected_status: "blocked_missing_production_map_pack_public_evidence", actual_status: "blocked_missing_production_map_pack_public_evidence", template_paths: ["/fixture/production-map-pack-public-evidence.template.json"], exit_status: 1, rejected: true},
      {name: "cohort_commercial_templates", validator: "check_trillionnium_world_cohort_commercial_evidence", summary_path: "/fixture/cohort-commercial-template-summary.json", expected_status: "blocked_missing_cohort_commercial_real_evidence", actual_status: "blocked_missing_cohort_commercial_real_evidence", template_paths: ["/fixture/first-beta-cohort-evidence.template.json", "/fixture/commercial-launch-drill-evidence.template.json"], exit_status: 1, rejected: true},
      {name: "external_ops_templates", validator: "check_trillionnium_world_external_ops_evidence", summary_path: "/fixture/external-ops-template-summary.json", expected_status: "blocked_missing_external_ops_real_evidence", actual_status: "blocked_missing_external_ops_real_evidence", template_paths: ["/fixture/multi-node-latency-evidence.template.json", "/fixture/public-network-deploy-evidence.template.json"], exit_status: 1, rejected: true}
    ],
    failures: []
  }' >"$template_negative_json"
  add_artifact_from_path public_launch_template_negative_fixtures "Public launch template negative fixtures" "$template_negative_json" release_review_gate
}

add_public_launch_status_only_fixture_guard_packet_fixtures() {
  local status_only_json="$TMP_DIR/public-launch-status-only-fixtures.json"
  jq -n '{
    contract_version: "trillionnium_world_public_launch_status_only_fixture_guard_v1",
    status: "public_launch_status_only_fixture_guard_green",
    source_of_truth: "trillionnium_world_public_launch_status_only_fixture_guard",
    green: true,
    public_launch_claimed: false,
    android_s5_real_device_claimed: false,
    live_map_ingestion_performed: false,
    live_public_exposure_performed: false,
    guard_rule: "status_only_green_fixtures_must_be_rejected_by_field_level_public_launch_evidence_validators",
    fixture_dir: "/fixture/status-only",
    result_count: 4,
    failure_count: 0,
    results: [
      {name: "s5_status_only_fixture", exit_code: 1, summary_path: "/fixture/s5-summary.json", expected_status: "blocked_missing_s5_real_device_evidence", summary_status: "blocked_missing_s5_real_device_evidence", blocked_as_expected: true, blocker_present: true, stdout_path: "/fixture/s5.out", stderr_path: "/fixture/s5.err"},
      {name: "production_map_pack_status_only_fixture", exit_code: 1, summary_path: "/fixture/map-summary.json", expected_status: "blocked_missing_production_map_pack_public_evidence", summary_status: "blocked_missing_production_map_pack_public_evidence", blocked_as_expected: true, blocker_present: true, stdout_path: "/fixture/map.out", stderr_path: "/fixture/map.err"},
      {name: "cohort_commercial_status_only_fixture", exit_code: 1, summary_path: "/fixture/cohort-commercial-summary.json", expected_status: "blocked_missing_cohort_commercial_real_evidence", summary_status: "blocked_missing_cohort_commercial_real_evidence", blocked_as_expected: true, blocker_present: true, stdout_path: "/fixture/cohort-commercial.out", stderr_path: "/fixture/cohort-commercial.err"},
      {name: "external_ops_status_only_fixture", exit_code: 1, summary_path: "/fixture/external-ops-summary.json", expected_status: "blocked_missing_external_ops_real_evidence", summary_status: "blocked_missing_external_ops_real_evidence", blocked_as_expected: true, blocker_present: true, stdout_path: "/fixture/external-ops.out", stderr_path: "/fixture/external-ops.err"}
    ],
    failures: []
  }' >"$status_only_json"
  add_artifact_from_path public_launch_status_only_fixture_guard "Public launch status-only fixture guard" "$status_only_json" release_review_gate
}

add_public_launch_operator_handoff_packet_fixtures() {
  local operator_handoff_json="$TMP_DIR/public-launch-operator-handoff.json"
  jq -n '
    [
      {id: "release_review_status_json"},
      {id: "release_review_status_markdown"},
      {id: "public_launch_evidence_intake_json"},
      {id: "public_launch_evidence_intake_markdown"},
      {id: "public_launch_evidence_kit_json"},
      {id: "public_launch_evidence_kit_markdown"},
      {id: "production_map_pack_public_collection_json"},
      {id: "production_map_pack_public_collection_markdown"},
      {id: "cohort_commercial_collection_json"},
      {id: "cohort_commercial_collection_markdown"},
      {id: "external_ops_collection_json"},
      {id: "external_ops_collection_markdown"},
      {id: "public_launch_blocker_consistency_json"},
      {id: "public_launch_template_negative_fixtures_json"},
      {id: "public_launch_evidence_bundle_json"},
      {id: "public_launch_evidence_bundle_markdown"},
      {id: "public_launch_evidence_bundle_template"},
      {id: "public_launch_bundle_negative_fixtures_json"},
      {id: "s5_real_device_template"},
      {id: "production_map_pack_public_template"},
      {id: "first_beta_cohort_template"},
      {id: "commercial_launch_drill_template"},
      {id: "multi_node_latency_template"},
      {id: "public_network_deploy_template"}
    ] as $artifact_ids |
    {
      contract_version: "trillionnium_world_public_launch_operator_handoff_v1",
      status: "public_launch_operator_handoff_ready_with_external_blockers",
      source_of_truth: "trillionnium_world_public_launch_operator_handoff",
      green: true,
      ready_for_release_review: true,
      public_launch_ready: false,
      public_launch_claimed: false,
      android_s5_real_device_claimed: false,
      live_map_ingestion_performed: false,
      live_public_exposure_performed: false,
      handoff_rule: "operator_handoff_collects_real_external_public_launch_evidence_without_claiming_public_launch_ready_or_android_s5_real_device_ready",
      markdown_path: "/fixture/public-launch-operator-handoff.md",
      evidence_bundle_template: "/fixture/public-launch-evidence-bundle.template.json",
      needs_collection_count: 6,
      known_blockers: [
        "s5_real_device_matrix",
        "production_map_pack_public_evidence",
        "first_beta_cohort_evidence",
        "commercial_launch_drill_evidence",
        "multi_node_or_live_traffic_latency_evidence",
        "public_network_live_exposure_evidence"
      ],
      blocked_items: [
        {id: "s5_real_device_matrix"},
        {id: "production_map_pack_public_evidence"},
        {id: "first_beta_cohort_evidence"},
        {id: "commercial_launch_drill_evidence"},
        {id: "multi_node_or_live_traffic_latency_evidence"},
        {id: "public_network_live_exposure_evidence"}
      ],
      operator_actions: [
        {id: "s5_android_real_device_matrix", blocker_id: "s5_real_device_matrix", label: "S5 Android real-device matrix", evidence_env_var: "ANDROID_SERIAL", template_path: "/fixture/s5-device-evidence.template.json", template_sha256: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", collection_command: "ANDROID_SERIAL=<device-serial> scripts/check_trillionnium_world_s5_device_evidence.sh --require-device", validator_command: "TRILLIONNIUM_WORLD_S5_REAL_DEVICE_EVIDENCE_PATH=<real-s5-evidence.json> scripts/check_trillionnium_world_s5_real_device_evidence.sh --require-ready", accepted_status: "s5_real_device_evidence_green", current_status: "blocked_missing_s5_real_device_evidence", collection_requirement: "Attach a USB-debugging Android device and collect real screenshot/gfxinfo/logcat/lifecycle evidence.", template_public_launch_credit: false},
        {id: "production_map_pack_public_evidence", blocker_id: "production_map_pack_public_evidence", label: "Production map-pack public evidence", evidence_env_var: "TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH", template_path: "/fixture/production-map-pack-public-evidence.template.json", template_sha256: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", collection_command: "scripts/check_trillionnium_world_production_map_pack_public_evidence_collection.sh", validator_command: "TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_EVIDENCE_PATH=<real-map-pack-evidence.json> scripts/check_trillionnium_world_production_map_pack_public_evidence.sh --require-ready", accepted_status: "production_map_pack_public_ready_green", current_status: "blocked_missing_production_map_pack_public_evidence", collection_requirement: "Collect approved source, ODbL/license, attribution, POI/geofence, distribution, rollback, and signoff evidence.", template_public_launch_credit: false},
        {id: "first_beta_cohort_evidence", blocker_id: "first_beta_cohort_evidence", label: "First beta cohort evidence", evidence_env_var: "TRILLIONNIUM_FIRST_BETA_COHORT_EVIDENCE_PATH", template_path: "/fixture/first-beta-cohort-evidence.template.json", template_sha256: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", collection_command: "scripts/check_trillionnium_world_cohort_commercial_evidence_collection.sh", validator_command: "TRILLIONNIUM_FIRST_BETA_COHORT_EVIDENCE_PATH=<real-cohort.json> scripts/check_trillionnium_world_cohort_commercial_evidence.sh --require-ready", accepted_status: "first_beta_cohort_evidence_green", current_status: "blocked_missing_first_beta_cohort_evidence", collection_requirement: "Collect real 5-10 participant/session/feedback/signoff evidence.", template_public_launch_credit: false},
        {id: "commercial_launch_drill_evidence", blocker_id: "commercial_launch_drill_evidence", label: "Commercial launch drill evidence", evidence_env_var: "TRILLIONNIUM_COMMERCIAL_LAUNCH_DRILL_EVIDENCE_PATH", template_path: "/fixture/commercial-launch-drill-evidence.template.json", template_sha256: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", collection_command: "scripts/check_trillionnium_world_cohort_commercial_evidence_collection.sh", validator_command: "TRILLIONNIUM_COMMERCIAL_LAUNCH_DRILL_EVIDENCE_PATH=<real-commercial-drill.json> scripts/check_trillionnium_world_cohort_commercial_evidence.sh --require-ready", accepted_status: "commercial_launch_drill_evidence_green", current_status: "blocked_missing_commercial_launch_drill_evidence", collection_requirement: "Collect payment, refund, support, legal, operator, traffic, and signoff evidence.", template_public_launch_credit: false},
        {id: "multi_node_or_live_traffic_latency_evidence", blocker_id: "multi_node_or_live_traffic_latency_evidence", label: "Multi-node or live-traffic latency evidence", evidence_env_var: "TRILLIONNIUM_MULTI_NODE_LATENCY_EVIDENCE_PATH", template_path: "/fixture/multi-node-latency-evidence.template.json", template_sha256: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", collection_command: "scripts/check_trillionnium_world_external_ops_evidence_collection.sh", validator_command: "TRILLIONNIUM_MULTI_NODE_LATENCY_EVIDENCE_PATH=<real-latency.json> scripts/check_trillionnium_world_external_ops_evidence.sh --require-ready", accepted_status: "multi_node_or_live_traffic_latency_green", current_status: "blocked_missing_multi_node_or_live_traffic_latency_evidence", collection_requirement: "Collect multi-node or live public traffic latency, monitoring, and rollback evidence.", template_public_launch_credit: false},
        {id: "public_network_live_exposure_evidence", blocker_id: "public_network_live_exposure_evidence", label: "Public network live exposure evidence", evidence_env_var: "TRILLIONNIUM_PUBLIC_NETWORK_DEPLOY_EVIDENCE_PATH", template_path: "/fixture/public-network-deploy-evidence.template.json", template_sha256: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", collection_command: "scripts/check_trillionnium_world_external_ops_evidence_collection.sh", validator_command: "TRILLIONNIUM_PUBLIC_NETWORK_DEPLOY_EVIDENCE_PATH=<real-public-deploy.json> scripts/check_trillionnium_world_external_ops_evidence.sh --require-ready", accepted_status: "public_network_deploy_green", current_status: "blocked_missing_public_network_live_exposure_evidence", collection_requirement: "Collect approved public exposure, host, domain, TLS, probes, monitoring, backup, rollback, and signoff evidence.", template_public_launch_credit: false}
      ],
      handoff_artifacts: ($artifact_ids | map({
        id: .id,
        label: .id,
        path: ("/fixture/" + .id + ".json"),
        role: "operator_handoff_fixture",
        file_status: "present",
        sha256: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        bytes: 128
      })),
      missing_artifacts: [],
      failures: [],
      reviewer_next_action: "collect_real_external_public_launch_evidence_using_operator_handoff"
    }
  ' >"$operator_handoff_json"
  add_artifact_from_path public_launch_operator_handoff "Public launch operator handoff" "$operator_handoff_json" release_review_operator_handoff

  local operator_handoff_md="$TMP_DIR/public-launch-operator-handoff.md"
  {
    printf '# Trillionnium World Public Launch Operator Handoff\n\n'
    printf -- '- status: public_launch_operator_handoff_ready_with_external_blockers\n'
    printf -- '- ready_for_release_review: true\n'
    printf -- '- public_launch_ready: false\n'
    printf -- '- public_launch_claimed: false\n'
    printf -- '- android_s5_real_device_claimed: false\n\n'
    printf '## Operator Collection Actions\n\n'
    printf -- '- [ ] S5 Android real-device matrix (s5_real_device_evidence_green)\n'
    printf -- '- [ ] Production map-pack public evidence (production_map_pack_public_ready_green)\n'
    printf -- '- [ ] First beta cohort evidence (first_beta_cohort_evidence_green)\n'
    printf -- '- [ ] Commercial launch drill evidence (commercial_launch_drill_evidence_green)\n'
    printf -- '- [ ] Multi-node or live-traffic latency evidence (multi_node_or_live_traffic_latency_green)\n'
    printf -- '- [ ] Public network live exposure evidence (public_network_deploy_green)\n\n'
    printf '## Bundle Flow\n\n'
    printf -- '- Run TRILLIONNIUM_PUBLIC_LAUNCH_EVIDENCE_BUNDLE_PATH=<real-bundle.json> scripts/check_trillionnium_world_public_launch_evidence_bundle.sh --require-ready.\n\n'
    printf '## Handoff Artifacts\n\n'
    printf -- '- public_launch_evidence_bundle_template: /fixture/public-launch-evidence-bundle.template.json (present, aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa)\n\n'
    printf '## Boundary\n\n'
    printf -- '- This handoff is an operator checklist and checksum manifest, not public-launch approval.\n'
    printf -- '- No live map ingestion, public exposure, Android S5 claim, or public-launch readiness claim is made here.\n'
  } >"$operator_handoff_md"
  add_artifact_from_path public_launch_operator_handoff_markdown "Public launch operator handoff Markdown" "$operator_handoff_md" release_review_operator_handoff
}

add_release_review_checkpoint_manifest_packet_fixtures() {
  local mode="${1:-valid}"
  local checkpoint_manifest_json="$TMP_DIR/release-review-checkpoint-manifest.json"
  local working_tree_path_count=3

  if [[ "$mode" == "semantic_invalid" ]]; then
    working_tree_path_count=4
  fi

  jq -n \
    --argjson working_tree_path_count "$working_tree_path_count" \
    '{
      contract_version: "trillionnium_world_release_review_checkpoint_manifest_v1",
      status: "checkpoint_manifest_ready_with_dirty_wip",
      generated_at: "2026-06-27T00:00:00Z",
      source_of_truth: "git_status_porcelain_plus_release_review_evidence",
      green: true,
      dirty_tree: true,
      ready_for_release_review: true,
      public_launch_ready: false,
      cex_adapter_ready: true,
      working_tree_path_count: $working_tree_path_count,
      tracked_modified_count: 2,
      untracked_path_count: 1,
      uncategorized_path_count: 0,
      working_tree_group_count: 2,
      working_tree_entry_count: 3,
      release_review_artifact_count: 128,
      release_review_failure_count: 0,
      repository_ahead_count: 763,
      repository_behind_count: 0,
      proof_scope: "checkpoint_manifest_only_not_public_launch_evidence",
      repository: {
        branch: "main",
        head_sha: "fixture123",
        head_subject: "fixture checkpoint manifest",
        upstream: "origin/main",
        ahead_count: 763,
        behind_count: 0
      },
      release_review_snapshot: {
        ci_gate_path: "/fixture/release-review-ci-gate.json",
        status_path: "/fixture/release-review-status.json",
        status: "release_review_ci_gate_green_with_public_launch_blockers",
        green: true,
        ready_for_release_review: true,
        public_launch_ready: false,
        failure_count: 0,
        artifact_count: 128
      },
      cex_adapter_readiness_snapshot: {
        evidence_path: "/fixture/cex-production-adapter-readiness.json",
        green: true,
        status: "cex_adapter_readiness_green",
        protocol_contract: "trillionnium_world_runtime_adapter_v1",
        source_contract: "cex_trillionnium_world_production_adapter_v1",
        route_record_total: 7236,
        world_node_count: 24
      },
      working_tree: {
        total_paths: 3,
        tracked_modified_count: 2,
        untracked_count: 1,
        uncategorized_count: 0,
        groups: [
          {category: "release_review_surface", label: "Release-review status, packet, signoff, and docs surface", review_order: 20, path_count: 2, tracked_modified_count: 2, untracked_count: 0, paths: ["scripts/check_trillionnium_world_release_review_checkpoint_manifest.sh", "scripts/check_trillionnium_world_release_review_packet_integrity.sh"]},
          {category: "generated_acceptance_evidence", label: "Generated acceptance evidence", review_order: 80, path_count: 1, tracked_modified_count: 0, untracked_count: 1, paths: ["acceptance/S6_public_launch/latest/release-review-checkpoint-manifest.json"]}
        ],
        entries: [
          {status_code: " M", tracking_state: "tracked", category: "release_review_surface", path: "scripts/check_trillionnium_world_release_review_checkpoint_manifest.sh"},
          {status_code: " M", tracking_state: "tracked", category: "release_review_surface", path: "scripts/check_trillionnium_world_release_review_packet_integrity.sh"},
          {status_code: "??", tracking_state: "untracked", category: "generated_acceptance_evidence", path: "acceptance/S6_public_launch/latest/release-review-checkpoint-manifest.json"}
        ]
      },
      recommended_review_order: [
        "cex_adapter_readiness",
        "release_review_surface",
        "external_evidence_validators",
        "native_bevy_host_playability",
        "map_pack_repository_boundary",
        "code_surface",
        "repo_infra_dev_environment",
        "generated_acceptance_evidence",
        "docs_planning",
        "uncategorized"
      ],
      reviewer_next_action: "review_grouped_wip_then_commit_by_slice",
      boundary: {
        checkpoint_manifest_claim: "groups_current_working_tree_only",
        public_launch_claim: "does_not_replace_real_external_evidence",
        commit_claim: "does_not_commit_or_stage_files"
      }
    }' >"$checkpoint_manifest_json"
  add_artifact_from_path release_review_checkpoint_manifest "Release review checkpoint manifest" "$checkpoint_manifest_json" release_review_checkpoint
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
      green: true,
      public_launch_ready: false,
      android_s5_real_device_claimed: false,
      gate_count: 14,
      ready_gate_count: (11 + (if $render_asset_ready then 1 else 0 end) + (if $public_launch_consumes_render_asset then 1 else 0 end)),
      blocked_gate_count: (3 - (if $render_asset_ready then 1 else 0 end) - (if $public_launch_consumes_render_asset then 1 else 0 end)),
      summary_blocker_count: ($summary_blockers | length),
      public_launch_blocker_count: 6,
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
      green: true,
      ready_for_release_review: true,
      public_launch_ready: false,
      android_s5_real_device_claimed: false,
      gate_count: 14,
      ready_gate_count: (11 + (if $render_asset_ready then 1 else 0 end) + (if $consumes_local_playability then 1 else 0 end)),
      blocked_gate_count: (3 - (if $render_asset_ready then 1 else 0 end) - (if $consumes_local_playability then 1 else 0 end)),
      signoff_summary_blocker_count: ($signoff_summary_blockers | length),
      public_launch_blocker_count: 6,
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
  local status_md="$TMP_DIR/release-review-status.md"
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
      green: true,
      ready_for_release_review: true,
      public_launch_ready: $public_launch_ready,
      android_s5_real_device_claimed: $android_s5_real_device_claimed,
      boundary: {
        native_bevy_replay_scope: "host_side_bevy_runtime_replay_not_android_real_device",
        native_bevy_texture_render_scope: "host_side_texture_sampling_correlation_and_render_asset_eligibility_not_gpu_upload_or_android_real_device",
        public_launch_claim: "blocked_until_real_external_evidence_is_attached"
      },
      ready_item_count: ($ready_items | length),
      blocked_item_count: 6,
      public_launch_blocker_count: 6,
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

  if [[ "$mode" == "semantic_invalid" ]]; then
    printf '# Broken Release Review Status\n\nThis fixture intentionally omits the required review checklist and public-launch boundary.\n' >"$status_md"
  else
    cat >"$status_md" <<'MARKDOWN'
# Trillionnium World Release Review Status

- status: `release_review_ready_public_launch_blocked`
- green: `true`
- ready_for_release_review: `true`
- public_launch_ready: `false`
- android_s5_real_device_claimed: `false`

## Counts

- ready_item_count: `13`
- blocked_item_count: `6`
- public_launch_blocker_count: `6`

## Green For Review

- [x] Native/Bevy keyboard replay: force=10, agility=8, craft=7; force combat=victory
- [x] Native/Bevy action coach: coach_stage=true, enter_execution=true, final_next=true
- [x] Native/Bevy player HUD/debug layer: player_hud=true, debug_layer=true
- [x] Native/Bevy live-window screenshot sequence: frames=11, sequence=true, contact_sheet=true
- [x] Native/Bevy sprite texture sampling: sampled_surfaces=32, unique_rgba=10, four_layer=true
- [x] Native/Bevy sampled texture live-window correlation: live_frames=11, final_frame_colors=3376, four_layer=true
- [x] Native/Bevy render asset eligibility: usage=RenderAssetUsages(MAIN_WORLD | RENDER_WORLD), sprite_refs=32, render_usage=true
- [x] CEX production world adapter readiness: routes=7236, nodes=24, protocol=trillionnium_world_runtime_adapter_v1
- [x] Public launch consumes replay gate: blocked_missing_public_launch_evidence
- [x] Public launch consumes local playability gates: blocked_missing_public_launch_evidence
- [x] Release latency local drill: local_release_latency_drill_green
- [x] Release rollback/backup drill: release_rollback_backup_drill_green
- [x] Public deploy local drill: local_public_deploy_drill_green

## Still Requires Real External Evidence

- [ ] S5 Android real-device matrix: Connect an Android device and collect launch, screenshot, gfxinfo/frame, CJK/input, lifecycle, weak-network, APK resource/signature, and crash-free logcat evidence.
- [ ] Production map-pack public evidence: Provide production/public map-pack ready evidence, not only the local route or fixture-signed manifest.
- [ ] First beta cohort evidence: Attach real 5-10 participant cohort evidence with status first_beta_cohort_evidence_green.
- [ ] Commercial launch drill evidence: Attach real or sanitized payment, refund, support, legal, operator, and traffic drill evidence.
- [ ] Multi-node or live-traffic latency evidence: Provide multi-node release latency or live public traffic latency evidence; local latency drill is not enough.
- [ ] Public network live exposure evidence: Provide approved host, domain/TLS, monitoring, backup, rollback, and public URL probe evidence.

## Boundary

- Native/Bevy keyboard replay, classic animation preview/selector, classic player motion, action coach, HUD/debug layer, player UI rescue, live screenshots, sprite texture sampling, sampled texture live-window correlation, and render asset eligibility are host-side proof, not Android real-device proof.
- CEX adapter readiness is incubator runtime adapter evidence, not real external public-launch evidence.
- Public launch remains blocked until the external evidence above is attached.
MARKDOWN
  fi

  add_artifact_from_path release_review_status_json "Release review status JSON" "$status_json" release_review_checklist
  add_artifact_from_path release_review_status_markdown "Release review status Markdown" "$status_md" release_review_checklist
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
      check_count: ($checks | length),
      failed_check_count: 0,
      checks_total: ($checks | length),
      checks_failed: 0,
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

add_release_review_packet_convergence_log_packet_fixtures() {
  local convergence_log="$TMP_DIR/release-review-packet-convergence.log"
  printf 'TRILLIONNIUM_WORLD_RELEASE_REVIEW_CONVERGENCE_GREEN_WITH_PUBLIC_LAUNCH_BLOCKERS /fixture/release-review-convergence.json\n' >"$convergence_log"
  add_artifact_from_path release_review_packet_convergence_log "Release review packet convergence log" "$convergence_log" release_review_log
}
