//! Bevy-free RTS evidence summaries and gates.
//!
//! This crate keeps proof contracts separate from `trnm-world-bevy` rendering code.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use trnm_rts_bevy_runtime::{
    rts_ability_effect_tiles_for_target, rts_ability_tooltip_telegraph_stage,
    rts_action_cadence_marks, rts_action_sequence_marks, rts_action_sequence_phase,
    rts_aftermath_debris_tiles_for_id, rts_aftermath_parts, rts_aftermath_smoke_tiles_for_id,
    rts_ai_counter_tiles_for_pressure, rts_ai_pressure_tiles_for_pressure,
    rts_ai_wave_unit_ids_for_pressure, rts_army_command_parts, rts_army_rally_tiles_for_id,
    rts_army_units_for_batch, rts_available_gold, rts_base_assault_parts,
    rts_base_assault_path_tiles_for_target, rts_base_assault_targets_for_id,
    rts_blocked_feedback_chip_visible, rts_blocked_feedback_player_label,
    rts_boss_guard_units_for_id, rts_build_palette_queue_id_for_slot, rts_build_parts,
    rts_build_site_tiles, rts_camera_minimap_sync_stage_summaries,
    rts_central_keep_route_tiles_for_id, rts_central_keep_tile_for_id, rts_combat_impact_stage,
    rts_command_execution_feedback_kind, rts_command_execution_player_label,
    rts_command_execution_target_label, rts_command_execution_target_tile,
    rts_command_feedback_lifecycle_stage, rts_command_feedback_strip_stage,
    rts_command_history_prune_visible, rts_command_history_visible,
    rts_command_queue_path_preview_stage, rts_command_queue_path_preview_stage_fixtures,
    rts_command_slot_id_for_index, rts_command_stamp_for_ability, rts_command_stamp_for_move,
    rts_command_stamp_for_selection, rts_command_surface_stage, rts_commander_aura_tiles_for_id,
    rts_commander_parts, rts_contact_flash_tiles_for_target,
    rts_control_group_command_feedback_lifecycle_fixtures,
    rts_control_group_command_feedback_rejection_replay_fixtures,
    rts_control_group_command_feedback_replay_fixtures,
    rts_control_group_command_feedback_strip_fixtures, rts_control_group_command_history_fixtures,
    rts_control_group_command_history_prune_fixtures, rts_control_group_hotkey_feedback_stage,
    rts_control_group_hotkey_slot, rts_control_group_recall_formation_preview_stage_fixtures,
    rts_control_group_recall_override_preview_stage_fixtures, rts_control_group_slot_summaries,
    rts_counter_command_parts, rts_counterattack_route_tiles_for_wave,
    rts_counterattack_units_for_wave, rts_creep_camp_parts, rts_creep_camp_tiles_for_id,
    rts_creep_camp_units_for_id, rts_cursor_kind_for_hover_preview,
    rts_cursor_label_for_hover_preview, rts_damage_ticks_for_ability, rts_default_group_units,
    rts_default_units_for_control_group_slot, rts_depth_readability_stage, rts_drag_distance_sq,
    rts_drag_group_id, rts_drag_rejected_unit_ids, rts_drag_select_player_label,
    rts_drag_select_ready, rts_drag_selected_units, rts_dropoff_tile_for_structure,
    rts_enemy_command_parts, rts_enemy_flank_tile_for_index, rts_enemy_flank_units_for_id,
    rts_enemy_fortification_tile_for_id, rts_enemy_pressure_lane_tiles_for_wave,
    rts_enemy_pressure_wave_units_for_id, rts_enemy_repair_units_for_target,
    rts_enemy_structure_tile_for_id, rts_enemy_structures_for_recon, rts_enemy_unit_tile_for_id,
    rts_enemy_units_for_recon, rts_engagement_tiles_for_target, rts_environment_life_stage,
    rts_expansion_parts, rts_expansion_structure_tile_for_id, rts_expansion_tiles_for_camp,
    rts_expansion_tiles_for_id, rts_expansion_workers_for_line, rts_focus_fire_units_for_target,
    rts_fog_reveal_tiles_for_recon, rts_formation_move_execution_fixtures,
    rts_formation_move_execution_stage, rts_formation_move_preview_stage,
    rts_formation_move_preview_stage_fixtures, rts_garrison_units_for_id,
    rts_guardian_counter_units_for_id, rts_harvest_tile_for_node, rts_hover_player_label,
    rts_hover_target_preview_kind, rts_inner_core_tile_for_id, rts_inner_defenders_for_id,
    rts_inner_gate_tile_for_id, rts_inner_lane_tiles_for_id, rts_keep_breach_tiles_for_id,
    rts_keep_claim_tiles_for_id, rts_line_path_tiles, rts_local_obstruction_recovery_fixtures,
    rts_local_obstruction_recovery_stage, rts_locomotion_blend_stage, rts_loot_items_for_id,
    rts_merged_unit_ids, rts_minimap_cell_origin, rts_move_command_parts, rts_npc_behavior_stage,
    rts_npc_transition_stage, rts_objective_parts, rts_objective_tiles_for_id,
    rts_open_world_panels_for_room, rts_open_world_route_tiles_for_id,
    rts_order_queue_replay_action, rts_palette_cancel_queue_id, rts_palette_state_label,
    rts_player_army_unit_tile_for_id, rts_player_hold_tiles_for_id,
    rts_player_siege_line_tiles_for_id, rts_production_slot_queue_id,
    rts_production_spawn_animation_stage, rts_projectile_id_for_ability,
    rts_projectile_trail_tiles_for_target, rts_queue_feedback_chip, rts_queue_gold_cost,
    rts_queue_is_affordable, rts_queue_uses_production_lane, rts_rebuild_structures_for_id,
    rts_recon_parts, rts_restored_zones_for_id, rts_runtime_hit_test_grid, rts_runtime_tile_line,
    rts_same_class_units, rts_scout_route_tiles_for_recon, rts_scripted_demo_pauses_queue_tick,
    rts_scripted_demo_stage_from_frame, rts_scripted_demo_stage_id, rts_scripted_demo_stage_title,
    rts_scrollable_map_camera_stage_summaries, rts_selectable_unit_tile, rts_selection_clear_parts,
    rts_selection_command_feedback_stage, rts_selection_tiles_for_units,
    rts_sidebar_cancel_queue_id, rts_sidebar_queue_summary, rts_sidebar_slot_status_label,
    rts_siege_breach_tiles_for_target, rts_siege_push_route_tiles_for_target,
    rts_siege_unit_tile_for_id, rts_siege_units_for_id, rts_spawned_unit_id_from_queue,
    rts_split_squad_tiles_for_id, rts_structure_id_from_queue, rts_structure_modeling_stage,
    rts_structure_tile_for_id, rts_supply_convoy_for_id, rts_target_priority_ids_for_target,
    rts_target_tile_for_id, rts_terrain_choke_tiles_for_camp, rts_terrain_route_tiles_for_camp,
    rts_threat_levels_for_target, rts_tier_two_parts, rts_unit_model_depth_marks,
    rts_unit_status_energy_percent, rts_unit_status_health_percent, rts_unit_status_portrait_stage,
    rts_unit_status_portrait_unit_id, rts_unit_status_role_badges,
    rts_units_from_control_group_assignment, rts_unlock_unit_tile_for_id,
    rts_worker_harvest_animation_stage, RtsCameraMinimapViewportRect, RtsCommandStamp,
    RtsControlGroupSlotSummary, RtsOrderQueueReplayAction, RtsRuntimeGridSpec,
    RtsRuntimeTileLineStep, TRNM_RTS_BEVY_RUNTIME_CONTRACT,
};

pub const TRNM_RTS_EVIDENCE_CONTRACT: &str = "trnm_rts_evidence_v1";
pub const TRNM_RTS_EVIDENCE_BEVY_RUNTIME_ADAPTER_CONTRACT: &str =
    "trnm_rts_evidence_bevy_runtime_adapter_v1";
pub const TRNM_RTS_EVIDENCE_CAMPAIGN_UI_CONTINUITY_REVIEW_CONTRACT: &str =
    "trnm_rts_evidence_campaign_ui_continuity_review_v1";
pub const TRNM_RTS_EVIDENCE_SESSION_STATE_CONTINUITY_REVIEW_CONTRACT: &str =
    "trnm_rts_evidence_session_state_continuity_review_v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsEvidencePoint {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsCampaignUiContinuityReview {
    pub contract_version: String,
    pub green: bool,
    pub campaign_handoff_contract: String,
    pub campaign_handoff_green: bool,
    pub preview_width: u64,
    pub preview_height: u64,
    pub capture_frame_count: u64,
    pub final_current_room_id: Option<String>,
    pub restored_current_room_id: Option<String>,
    pub final_route_director_task_id: Option<String>,
    pub restored_route_director_task_id: Option<String>,
    pub final_open_world_handoff_state: Option<String>,
    pub restored_open_world_handoff_state: Option<String>,
    pub handoff_green_gate: bool,
    pub preview_resolution_gate: bool,
    pub live_input_gate: bool,
    pub milestone_gate: bool,
    pub map_ui_state_gate: bool,
    pub restored_ui_state_gate: bool,
    pub persistence_gate: bool,
    pub render_readability_gate: bool,
    pub native_client_boundary_gate: bool,
    pub player_first_campaign_continuity_screen_gate: bool,
    pub input_path: String,
    pub evidence_path: String,
    pub source_of_truth: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsSessionStateContinuityReview {
    pub contract_version: String,
    pub green: bool,
    pub shell_meta_contract: String,
    pub session_slot_confirm_contract: String,
    pub session_load_resume_contract: String,
    pub session_recovery_contract: String,
    pub match_setup_contract: String,
    pub hud_contract: String,
    pub campaign_outcome_contract: String,
    pub campaign_continuity_contract: String,
    pub preview_width: u64,
    pub preview_height: u64,
    pub state_continuity_surface_count: u64,
    pub shell_meta_surface_count: u64,
    pub confirmed_slot_a_bytes: u64,
    pub load_resume_slot_a_bytes: u64,
    pub load_resume_final_objective_status: Option<String>,
    pub match_setup_map_id: Option<String>,
    pub hud_surface_count: u64,
    pub hud_army_supply_used: u64,
    pub campaign_outcome_open_world_state: Option<String>,
    pub campaign_continuity_restored_room_id: Option<String>,
    pub shell_meta_gate: bool,
    pub session_slot_confirm_gate: bool,
    pub session_load_resume_gate: bool,
    pub session_recovery_gate: bool,
    pub match_setup_gate: bool,
    pub hud_restore_gate: bool,
    pub campaign_outcome_gate: bool,
    pub campaign_continuity_gate: bool,
    pub surface_chain_gate: bool,
    pub state_continuity_chain_gate: bool,
    pub native_client_boundary_gate: bool,
    pub preview_gate: bool,
    pub player_first_session_resume_screen_gate: bool,
    pub source_preview_gate: bool,
    pub runtime_screen_gate: bool,
    pub session_state_continuity_gate: bool,
    pub input_path: String,
    pub evidence_path: String,
    pub source_of_truth: String,
}

fn json_bool_at(value: &Value, key: &str) -> bool {
    value.get(key).and_then(Value::as_bool) == Some(true)
}

fn json_bool_pointer(value: &Value, pointer: &str) -> bool {
    value.pointer(pointer).and_then(Value::as_bool) == Some(true)
}

fn json_string_at(value: &Value, key: &str) -> Option<String> {
    value.get(key).and_then(Value::as_str).map(str::to_string)
}

fn json_string_pointer(value: &Value, pointer: &str) -> Option<String> {
    value
        .pointer(pointer)
        .and_then(Value::as_str)
        .map(str::to_string)
}

fn json_string_equals(value: &Value, key: &str, expected: &str) -> bool {
    value.get(key).and_then(Value::as_str) == Some(expected)
}

fn json_u64_at(value: &Value, key: &str) -> u64 {
    value.get(key).and_then(Value::as_u64).unwrap_or_default()
}

fn json_u64_pointer(value: &Value, pointer: &str) -> u64 {
    value
        .pointer(pointer)
        .and_then(Value::as_u64)
        .unwrap_or_default()
}

fn json_array_len_pointer(value: &Value, pointer: &str) -> u64 {
    value
        .pointer(pointer)
        .and_then(Value::as_array)
        .map(|items| items.len() as u64)
        .unwrap_or_default()
}

fn json_array_contains(value: &Value, pointer: &str, expected: &str) -> bool {
    value
        .pointer(pointer)
        .and_then(Value::as_array)
        .is_some_and(|items| items.iter().any(|item| item.as_str() == Some(expected)))
}

fn json_contract_is(value: &Value, expected: &str) -> bool {
    value.get("contract_version").and_then(Value::as_str) == Some(expected)
}

pub fn rts_campaign_ui_continuity_review(handoff: &Value) -> RtsCampaignUiContinuityReview {
    let campaign_handoff_contract = json_string_at(handoff, "contract_version").unwrap_or_default();
    let campaign_handoff_green = json_bool_at(handoff, "green");
    let preview_width = json_u64_at(handoff, "preview_width");
    let preview_height = json_u64_at(handoff, "preview_height");
    let capture_frame_count = json_u64_at(handoff, "capture_frame_count");
    let final_current_room_id = json_string_at(handoff, "final_current_room_id");
    let restored_current_room_id = json_string_at(handoff, "restored_current_room_id");
    let final_route_director_task_id = json_string_at(handoff, "final_route_director_task_id");
    let restored_route_director_task_id =
        json_string_at(handoff, "restored_route_director_task_id");
    let final_open_world_handoff_state = json_string_at(handoff, "final_open_world_handoff_state");
    let restored_open_world_handoff_state =
        json_string_at(handoff, "restored_open_world_handoff_state");
    let milestone_gate = handoff
        .get("milestones")
        .and_then(Value::as_object)
        .is_some_and(|milestones| {
            milestones
                .values()
                .all(|milestone| milestone.as_bool() == Some(true))
        });
    let handoff_green_gate = campaign_handoff_green
        && campaign_handoff_contract == "trillionnium_world_bevy_classic_rts_campaign_handoff_v1";
    let preview_resolution_gate = json_bool_at(handoff, "write_gate")
        && preview_width == 1920
        && preview_height == 1080
        && capture_frame_count == 16;
    let map_ui_state_gate = json_string_equals(handoff, "final_current_room_id", "league-coliseum")
        && json_string_equals(handoff, "final_map_scene", "arena_outdoor")
        && json_string_equals(
            handoff,
            "final_route_director_task_id",
            "task-fixture-first-route",
        )
        && handoff
            .get("final_route_director_next_room_id")
            .is_some_and(Value::is_null)
        && json_string_equals(
            handoff,
            "final_open_world_handoff_state",
            "resumed:league-coliseum",
        )
        && json_string_equals(
            handoff,
            "final_contextual_primary_action_label",
            "COMBAT:attack",
        )
        && json_string_equals(
            handoff,
            "final_objective_status",
            "open_world_after_action_ready",
        )
        && json_array_contains(handoff, "/final_contextual_action_labels", "COMBAT:attack")
        && json_array_contains(
            handoff,
            "/final_active_task_ids",
            "task-fixture-first-route",
        )
        && json_array_contains(
            handoff,
            "/final_route_director_history",
            "rts_open_world_after_action:league-coliseum:arrived",
        );
    let restored_ui_state_gate =
        json_string_equals(handoff, "restored_current_room_id", "league-coliseum")
            && json_string_equals(handoff, "restored_map_scene", "arena_outdoor")
            && json_string_equals(
                handoff,
                "restored_open_world_handoff_state",
                "resumed:league-coliseum",
            )
            && json_string_equals(
                handoff,
                "restored_route_director_task_id",
                "task-fixture-first-route",
            )
            && handoff
                .get("restored_route_director_next_room_id")
                .is_some_and(Value::is_null)
            && json_array_contains(
                handoff,
                "/restored_contextual_action_labels",
                "COMBAT:attack",
            )
            && json_array_contains(
                handoff,
                "/restored_active_task_ids",
                "task-fixture-first-route",
            );
    let render_readability_gate = json_u64_at(handoff, "non_background_pixels") > 500_000
        && json_u64_at(handoff, "victory_pixel_count") > 20
        && json_u64_at(handoff, "expansion_pixel_count") > 60
        && json_u64_at(handoff, "breach_pixel_count") > 40
        && json_u64_at(handoff, "keep_pixel_count") > 40
        && json_u64_at(handoff, "restoration_pixel_count") > 20
        && json_u64_at(handoff, "open_world_pixel_count") > 60;
    let live_input_gate = json_bool_at(handoff, "live_campaign_input_gate")
        && json_bool_at(handoff, "early_campaign_gate")
        && json_bool_at(handoff, "mid_campaign_gate")
        && json_bool_at(handoff, "end_campaign_gate")
        && json_bool_at(handoff, "open_world_resume_gate");
    let persistence_gate = json_bool_at(handoff, "snapshot_round_trip_gate");
    let native_client_boundary_gate = handoff
        .get("cex_runtime_player_client_allowed")
        .and_then(Value::as_bool)
        == Some(false)
        && handoff.get("wgpu_required").and_then(Value::as_bool) == Some(false);
    let player_first_campaign_continuity_screen_gate =
        json_bool_at(handoff, "player_first_campaign_handoff_screen_gate")
            && json_string_equals(
                handoff,
                "runtime_screen_mode",
                "player_runtime_campaign_handoff_screen",
            )
            && handoff.get("evidence_board_only").and_then(Value::as_bool) == Some(false)
            && json_u64_pointer(
                handoff,
                "/campaign_handoff_pixel_counts/player_first_campaign_view_non_background",
            ) > 600_000
            && json_u64_pointer(
                handoff,
                "/campaign_handoff_pixel_counts/player_first_campaign_view_frame",
            ) > 10_000
            && json_u64_pointer(
                handoff,
                "/campaign_handoff_pixel_counts/player_first_campaign_status_strip",
            ) > 8_000
            && json_u64_pointer(
                handoff,
                "/campaign_handoff_pixel_counts/player_first_campaign_route_rail",
            ) > 100_000;
    let green = handoff_green_gate
        && preview_resolution_gate
        && live_input_gate
        && milestone_gate
        && map_ui_state_gate
        && restored_ui_state_gate
        && persistence_gate
        && render_readability_gate
        && native_client_boundary_gate
        && player_first_campaign_continuity_screen_gate;

    RtsCampaignUiContinuityReview {
        contract_version: TRNM_RTS_EVIDENCE_CAMPAIGN_UI_CONTINUITY_REVIEW_CONTRACT.to_string(),
        green,
        campaign_handoff_contract,
        campaign_handoff_green,
        preview_width,
        preview_height,
        capture_frame_count,
        final_current_room_id,
        restored_current_room_id,
        final_route_director_task_id,
        restored_route_director_task_id,
        final_open_world_handoff_state,
        restored_open_world_handoff_state,
        handoff_green_gate,
        preview_resolution_gate,
        live_input_gate,
        milestone_gate,
        map_ui_state_gate,
        restored_ui_state_gate,
        persistence_gate,
        render_readability_gate,
        native_client_boundary_gate,
        player_first_campaign_continuity_screen_gate,
        input_path: "trnm-world-bevy campaign handoff evidence JSON -> trnm-rts-evidence campaign UI continuity review".to_string(),
        evidence_path: "trnm-rts-evidence campaign_ui_continuity_review -> Bevy campaign UI continuity packet artifact".to_string(),
        source_of_truth: "The RTS evidence crate reviews campaign handoff final/restored route state, save-restore persistence, milestone pixels, native-client no-credit boundary, and player-first route-resume screen gates before trnm-world-bevy includes the campaign UI continuity artifact in release-review evidence.".to_string(),
    }
}

pub fn rts_session_state_continuity_review(input: &Value) -> RtsSessionStateContinuityReview {
    let null = Value::Null;
    let shell_meta = input
        .pointer("/sources/shell_meta_ui_replication")
        .unwrap_or(&null);
    let session_slot_confirm = input
        .pointer("/sources/session_slot_confirm")
        .unwrap_or(&null);
    let session_load_resume = input
        .pointer("/sources/session_load_resume")
        .unwrap_or(&null);
    let session_recovery = input
        .pointer("/sources/session_recovery_ui")
        .unwrap_or(&null);
    let match_setup = input
        .pointer("/sources/match_setup_ui_replication")
        .unwrap_or(&null);
    let hud = input
        .pointer("/sources/in_match_hud_state_replication")
        .unwrap_or(&null);
    let campaign_outcome = input
        .pointer("/sources/campaign_outcome_ui_readiness")
        .unwrap_or(&null);
    let campaign_continuity = input
        .pointer("/sources/campaign_ui_continuity")
        .unwrap_or(&null);

    let shell_meta_contract = json_string_at(shell_meta, "contract_version").unwrap_or_default();
    let session_slot_confirm_contract =
        json_string_at(session_slot_confirm, "contract_version").unwrap_or_default();
    let session_load_resume_contract =
        json_string_at(session_load_resume, "contract_version").unwrap_or_default();
    let session_recovery_contract =
        json_string_at(session_recovery, "contract_version").unwrap_or_default();
    let match_setup_contract = json_string_at(match_setup, "contract_version").unwrap_or_default();
    let hud_contract = json_string_at(hud, "contract_version").unwrap_or_default();
    let campaign_outcome_contract =
        json_string_at(campaign_outcome, "contract_version").unwrap_or_default();
    let campaign_continuity_contract =
        json_string_at(campaign_continuity, "contract_version").unwrap_or_default();

    let preview_width = json_u64_at(input, "preview_width");
    let preview_height = json_u64_at(input, "preview_height");
    let state_continuity_surface_count =
        json_array_len_pointer(input, "/state_continuity_surface_names");
    let shell_meta_surface_count = json_u64_at(shell_meta, "shell_meta_surface_count");
    let confirmed_slot_a_bytes = json_u64_at(session_slot_confirm, "confirmed_slot_a_bytes");
    let load_resume_slot_a_bytes = json_u64_at(session_load_resume, "slot_a_bytes");
    let load_resume_final_objective_status =
        json_string_pointer(session_load_resume, "/final_runtime/objective_status");
    let match_setup_map_id = json_string_pointer(match_setup, "/source_headline/map_id");
    let hud_surface_count = json_u64_at(hud, "hud_surface_count");
    let hud_army_supply_used = json_u64_at(hud, "army_supply_used");
    let campaign_outcome_open_world_state = json_string_pointer(
        campaign_outcome,
        "/open_world_summary/final_open_world_handoff_state",
    );
    let campaign_continuity_restored_room_id =
        json_string_at(campaign_continuity, "restored_current_room_id");

    let shell_meta_gate = json_contract_is(
        shell_meta,
        "trillionnium_world_bevy_classic_rts_shell_meta_ui_replication_v1",
    ) && json_bool_at(shell_meta, "green")
        && json_bool_at(shell_meta, "runtime_screen_gate")
        && shell_meta
            .get("evidence_board_only")
            .and_then(Value::as_bool)
            == Some(false)
        && json_bool_at(shell_meta, "session_slot_confirm_gate")
        && json_bool_at(shell_meta, "session_load_resume_gate")
        && json_bool_at(shell_meta, "session_recovery_gate")
        && json_bool_at(shell_meta, "no_external_boundary_gate");
    let session_slot_confirm_gate = json_contract_is(
        session_slot_confirm,
        "trillionnium_world_bevy_session_slot_confirm_v1",
    ) && json_bool_at(session_slot_confirm, "green")
        && json_bool_at(session_slot_confirm, "save_selected_gate")
        && json_bool_at(session_slot_confirm, "confirm_overwrite_gate")
        && json_bool_at(session_slot_confirm, "load_selected_restore_gate")
        && json_bool_at(session_slot_confirm, "continue_after_load_gate")
        && json_bool_at(session_slot_confirm, "slot_file_gate")
        && confirmed_slot_a_bytes > 512;
    let session_load_resume_gate = json_contract_is(
        session_load_resume,
        "trillionnium_world_bevy_session_load_resume_v1",
    ) && json_bool_at(session_load_resume, "green")
        && json_bool_at(session_load_resume, "save_selected_gate")
        && json_bool_at(session_load_resume, "load_resume_gate")
        && json_bool_at(session_load_resume, "locked_input_gate")
        && json_bool_at(session_load_resume, "continue_gate")
        && json_bool_at(session_load_resume, "final_hud_gate")
        && load_resume_final_objective_status.as_deref() == Some("first_playable_loop_complete");
    let session_recovery_gate = json_contract_is(
        session_recovery,
        "trillionnium_world_bevy_session_recovery_ui_v1",
    ) && json_bool_at(session_recovery, "green")
        && json_bool_at(session_recovery, "recovered_status_gate")
        && json_bool_at(session_recovery, "continued_summary_gate")
        && json_bool_at(session_recovery, "guard_status_gate");
    let match_setup_gate = json_contract_is(
        match_setup,
        "trillionnium_world_bevy_classic_rts_match_setup_ui_replication_v1",
    ) && json_bool_at(match_setup, "green")
        && json_bool_at(match_setup, "match_setup_ui_replication_gate")
        && json_bool_at(match_setup, "runtime_screen_gate")
        && match_setup
            .get("evidence_board_only")
            .and_then(Value::as_bool)
            == Some(false)
        && json_bool_at(match_setup, "shell_meta_gate")
        && json_bool_at(match_setup, "faction_gate")
        && json_bool_at(match_setup, "no_external_boundary_gate");
    let hud_restore_gate = json_contract_is(
        hud,
        "trillionnium_world_bevy_classic_rts_in_match_hud_state_replication_v1",
    ) && json_bool_at(hud, "green")
        && json_bool_at(hud, "in_match_hud_state_replication_gate")
        && json_bool_at(hud, "runtime_screen_gate")
        && hud.get("evidence_board_only").and_then(Value::as_bool) == Some(false)
        && json_bool_at(hud, "selection_gate")
        && json_bool_at(hud, "command_gate")
        && json_bool_at(hud, "production_gate")
        && json_bool_at(hud, "native_client_boundary_gate")
        && hud_surface_count == 8
        && json_array_contains(hud, "/command_queue", "move:16,9")
        && json_array_contains(hud, "/command_queue", "train:trnm.worker")
        && json_array_contains(hud, "/command_queue", "build:trnm.flux.relay")
        && json_array_contains(hud, "/command_queue", "attack:trnm.flux.beacon");
    let campaign_outcome_gate = json_contract_is(
        campaign_outcome,
        "trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness_v1",
    ) && json_bool_at(campaign_outcome, "green")
        && json_bool_at(campaign_outcome, "first_minute_gate")
        && json_bool_at(campaign_outcome, "objective_victory_gate")
        && json_bool_at(campaign_outcome, "base_assault_gate")
        && json_bool_at(campaign_outcome, "battle_aftermath_gate")
        && json_bool_at(campaign_outcome, "open_world_return_gate")
        && json_bool_at(campaign_outcome, "runtime_screen_gate")
        && campaign_outcome
            .get("evidence_board_only")
            .and_then(Value::as_bool)
            == Some(false)
        && campaign_outcome_open_world_state.as_deref() == Some("resumed:league-coliseum");
    let campaign_continuity_gate = json_contract_is(
        campaign_continuity,
        "trillionnium_world_bevy_classic_rts_campaign_ui_continuity_v1",
    ) && json_bool_at(campaign_continuity, "green")
        && json_bool_at(campaign_continuity, "persistence_gate")
        && json_bool_at(campaign_continuity, "restored_ui_state_gate")
        && json_bool_at(campaign_continuity, "map_ui_state_gate")
        && json_bool_at(campaign_continuity, "native_client_boundary_gate")
        && campaign_continuity_restored_room_id.as_deref() == Some("league-coliseum")
        && json_string_equals(
            campaign_continuity,
            "restored_open_world_handoff_state",
            "resumed:league-coliseum",
        );
    let surface_chain_gate = state_continuity_surface_count == 8
        && json_array_contains(
            input,
            "/state_continuity_surface_names",
            "MATCH SETUP SNAPSHOT",
        )
        && json_array_contains(
            input,
            "/state_continuity_surface_names",
            "SESSION SLOT WRITE",
        )
        && json_array_contains(input, "/state_continuity_surface_names", "LOAD RESUME LOCK")
        && json_array_contains(input, "/state_continuity_surface_names", "CONTINUE UNLOCK")
        && json_array_contains(
            input,
            "/state_continuity_surface_names",
            "IN-MATCH HUD RESTORE",
        )
        && json_array_contains(
            input,
            "/state_continuity_surface_names",
            "OUTCOME REWARD STATE",
        )
        && json_array_contains(
            input,
            "/state_continuity_surface_names",
            "OPEN-WORLD RESUME",
        )
        && json_array_contains(
            input,
            "/state_continuity_surface_names",
            "RECOVERY UI GUARD",
        )
        && json_array_contains(input, "/resume_chain", "match_setup_saved")
        && json_array_contains(input, "/resume_chain", "slot_a_written")
        && json_array_contains(input, "/resume_chain", "load_resume_locked")
        && json_array_contains(input, "/resume_chain", "continue_unlocked")
        && json_array_contains(input, "/resume_chain", "in_match_hud_restored")
        && json_array_contains(input, "/resume_chain", "campaign_outcome_saved")
        && json_array_contains(input, "/resume_chain", "open_world_resumed");
    let native_client_boundary_gate = !json_bool_at(shell_meta, "android_s5_real_device_claimed")
        && !json_bool_at(session_slot_confirm, "android_s5_real_device_claimed")
        && !json_bool_at(session_load_resume, "android_s5_real_device_claimed")
        && !json_bool_at(session_recovery, "android_s5_real_device_claimed")
        && !json_bool_at(match_setup, "android_s5_real_device_claimed")
        && !json_bool_at(hud, "android_s5_real_device_claimed")
        && !json_bool_at(campaign_outcome, "android_s5_real_device_claimed")
        && !json_bool_at(shell_meta, "public_launch_ready")
        && !json_bool_at(match_setup, "public_launch_ready")
        && !json_bool_at(hud, "public_launch_ready")
        && !json_bool_at(campaign_outcome, "public_launch_ready")
        && !json_bool_at(hud, "screen_for_screen_openra_ui_claimed")
        && !json_bool_at(hud, "openra_engine_port_claimed")
        && !json_bool_at(hud, "warcraft_iii_asset_copied")
        && !json_bool_at(hud, "openra_asset_copied")
        && !json_bool_at(hud, "third_party_asset_copied")
        && !json_bool_pointer(
            input,
            "/native_client_boundary/cex_runtime_player_client_allowed",
        )
        && !json_bool_pointer(input, "/native_client_boundary/wgpu_required");
    let player_first_session_resume_screen_gate = json_u64_pointer(
        input,
        "/state_continuity_pixel_counts/player_first_resume_view_non_background",
    ) > 250_000
        && json_u64_pointer(
            input,
            "/state_continuity_pixel_counts/player_first_resume_view_frame",
        ) > 8_000
        && json_u64_pointer(
            input,
            "/state_continuity_pixel_counts/player_first_resume_status_strip",
        ) > 10_000
        && json_u64_pointer(
            input,
            "/state_continuity_pixel_counts/player_first_resume_stage_rail",
        ) > 70_000;
    let preview_gate = json_bool_at(input, "write_gate")
        && json_bool_at(input, "preview_file_ready")
        && preview_width == 1600
        && preview_height == 900
        && json_string_equals(input, "preview_format", "ppm_p3_rgb")
        && json_u64_pointer(input, "/state_continuity_pixel_counts/non_background") > 300_000
        && json_u64_pointer(input, "/state_continuity_pixel_counts/board") > 100_000
        && json_u64_pointer(input, "/state_continuity_pixel_counts/match_setup_snapshot") > 2_000
        && json_u64_pointer(input, "/state_continuity_pixel_counts/session_slot_write") > 2_000
        && json_u64_pointer(input, "/state_continuity_pixel_counts/load_resume_lock") > 2_000
        && json_u64_pointer(input, "/state_continuity_pixel_counts/continue_unlock") > 2_000
        && json_u64_pointer(input, "/state_continuity_pixel_counts/in_match_hud_restore") > 2_000
        && json_u64_pointer(input, "/state_continuity_pixel_counts/outcome_reward_state") > 2_000
        && json_u64_pointer(input, "/state_continuity_pixel_counts/open_world_resume") > 2_000
        && json_u64_pointer(input, "/state_continuity_pixel_counts/recovery_ui_guard") > 2_000
        && json_u64_pointer(input, "/state_continuity_pixel_counts/highlight") > 1_000
        && player_first_session_resume_screen_gate;
    let source_preview_gate =
        json_bool_pointer(input, "/source_preview_ready/shell_meta_ui_replication")
            && json_bool_pointer(input, "/source_preview_ready/match_setup_ui_replication")
            && json_bool_pointer(
                input,
                "/source_preview_ready/in_match_hud_state_replication",
            )
            && json_bool_pointer(input, "/source_preview_ready/campaign_ui_continuity")
            && json_bool_at(campaign_outcome, "preview_gate");
    let state_continuity_chain_gate = shell_meta_gate
        && session_slot_confirm_gate
        && session_load_resume_gate
        && session_recovery_gate
        && match_setup_gate
        && hud_restore_gate
        && campaign_outcome_gate
        && campaign_continuity_gate
        && surface_chain_gate;
    let runtime_screen_gate = state_continuity_chain_gate && preview_gate && source_preview_gate;
    let session_state_continuity_gate = runtime_screen_gate && native_client_boundary_gate;
    let green = session_state_continuity_gate;

    RtsSessionStateContinuityReview {
        contract_version: TRNM_RTS_EVIDENCE_SESSION_STATE_CONTINUITY_REVIEW_CONTRACT.to_string(),
        green,
        shell_meta_contract,
        session_slot_confirm_contract,
        session_load_resume_contract,
        session_recovery_contract,
        match_setup_contract,
        hud_contract,
        campaign_outcome_contract,
        campaign_continuity_contract,
        preview_width,
        preview_height,
        state_continuity_surface_count,
        shell_meta_surface_count,
        confirmed_slot_a_bytes,
        load_resume_slot_a_bytes,
        load_resume_final_objective_status,
        match_setup_map_id,
        hud_surface_count,
        hud_army_supply_used,
        campaign_outcome_open_world_state,
        campaign_continuity_restored_room_id,
        shell_meta_gate,
        session_slot_confirm_gate,
        session_load_resume_gate,
        session_recovery_gate,
        match_setup_gate,
        hud_restore_gate,
        campaign_outcome_gate,
        campaign_continuity_gate,
        surface_chain_gate,
        state_continuity_chain_gate,
        native_client_boundary_gate,
        preview_gate,
        player_first_session_resume_screen_gate,
        source_preview_gate,
        runtime_screen_gate,
        session_state_continuity_gate,
        input_path: "trnm-world-bevy session-state continuity source JSON and pixel counts -> trnm-rts-evidence session-state continuity review".to_string(),
        evidence_path: "trnm-rts-evidence session_state_continuity_review -> Bevy session-state continuity packet artifact".to_string(),
        source_of_truth: "The RTS evidence crate reviews save-slot confirmation, load-resume lock/continue, recovery guard, match setup, restored HUD, campaign outcome, campaign continuity, source preview readiness, native-client no-credit boundaries, and the player-first session resume screen before trnm-world-bevy includes the session-state continuity artifact in release-review evidence.".to_string(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsBevyRuntimeAdapterEvidence {
    pub contract_version: String,
    pub runtime_contract: String,
    pub green: bool,
    pub minimap_cell_sample: RtsEvidencePoint,
    pub scroll_camera_stage_count_sample: usize,
    pub scroll_camera_first_focus_tile_sample: RtsEvidencePoint,
    pub scroll_camera_minimap_jump_tile_sample: Option<String>,
    pub scroll_camera_bounds_clamped_sample: bool,
    pub camera_minimap_stage_count_sample: usize,
    pub camera_minimap_viewport_rect_sample: RtsCameraMinimapViewportRect,
    pub camera_minimap_selection_follow_tile_sample: Option<String>,
    pub camera_minimap_revealed_union_count_sample: usize,
    pub camera_minimap_zoom_rect_area_sample: i32,
    pub path_preview_sample: Option<String>,
    pub command_queue_path_preview_stage_count_sample: usize,
    pub command_queue_path_preview_action_kinds_sample: Vec<String>,
    pub command_queue_path_preview_action_payloads_sample: Vec<String>,
    pub command_queue_path_preview_history_entries_sample: Vec<String>,
    pub formation_move_preview_stage_sample: Option<String>,
    pub formation_move_preview_stage_count_sample: usize,
    pub formation_move_preview_action_payloads_sample: Vec<String>,
    pub formation_move_preview_history_entries_sample: Vec<String>,
    pub formation_move_preview_destination_slots_sample: Vec<String>,
    pub formation_move_preview_split_route_sample: Vec<String>,
    pub control_group_recall_formation_preview_stage_count_sample: usize,
    pub control_group_recall_formation_preview_action_payloads_sample: Vec<String>,
    pub control_group_recall_formation_preview_history_entries_sample: Vec<String>,
    pub control_group_recall_formation_preview_slot_tiles_sample: Vec<String>,
    pub control_group_recall_formation_preview_filtered_members_sample: Vec<String>,
    pub control_group_recall_override_preview_stage_count_sample: usize,
    pub control_group_recall_override_preview_action_payloads_sample: Vec<String>,
    pub control_group_recall_override_preview_history_entries_sample: Vec<String>,
    pub control_group_recall_override_preview_final_tiles_sample: Vec<String>,
    pub control_group_recall_override_preview_canceled_members_sample: Vec<String>,
    pub formation_move_execution_stage_sample: Option<String>,
    pub formation_move_execution_stage_names_sample: Vec<String>,
    pub formation_move_execution_action_payloads_sample: Vec<String>,
    pub formation_move_execution_arrival_route_sample: Vec<String>,
    pub local_obstruction_recovery_stage_sample: Option<String>,
    pub local_obstruction_recovery_stage_names_sample: Vec<String>,
    pub local_obstruction_recovery_action_payloads_sample: Vec<String>,
    pub local_obstruction_recovery_blocked_tiles_sample: Vec<String>,
    pub local_obstruction_recovery_resume_route_sample: Vec<String>,
    pub npc_behavior_stage_sample: Option<String>,
    pub combat_impact_stage_sample: Option<String>,
    pub locomotion_blend_stage_sample: Option<String>,
    pub npc_transition_stage_sample: Option<String>,
    pub depth_readability_stage_sample: Option<String>,
    pub structure_modeling_stage_sample: Option<String>,
    pub environment_life_stage_sample: Option<String>,
    pub worker_harvest_animation_stage_sample: Option<String>,
    pub production_spawn_animation_stage_sample: Option<String>,
    pub action_cadence_attack_mark_count_sample: usize,
    pub action_cadence_carry_mark_count_sample: usize,
    pub action_cadence_idle_mark_count_sample: usize,
    pub action_cadence_creep_windup_offset_sample: i32,
    pub action_sequence_phase_sample: Option<String>,
    pub action_sequence_windup_mark_count_sample: usize,
    pub action_sequence_strike_mark_count_sample: usize,
    pub action_sequence_carry_down_mark_count_sample: usize,
    pub action_sequence_idle_mark_count_sample: usize,
    pub unit_model_depth_guard_mark_count_sample: usize,
    pub unit_model_depth_worker_mark_count_sample: usize,
    pub unit_model_depth_creep_mark_count_sample: usize,
    pub unit_model_depth_creep_role_prop_count_sample: usize,
    pub unit_model_depth_face_shade_offset_sample: i32,
    pub command_surface_stage_sample: Option<String>,
    pub command_grid_hit_sample: Option<usize>,
    pub tile_line_sample: Vec<RtsRuntimeTileLineStep>,
    pub combat_engagement_tiles_sample: Vec<String>,
    pub combat_flash_tiles_sample: Vec<String>,
    pub combat_target_tile_sample: RtsEvidencePoint,
    pub combat_target_priority_sample: Vec<String>,
    pub combat_projectile_trail_sample: Vec<String>,
    pub combat_ability_effect_tiles_sample: Vec<String>,
    pub combat_threat_levels_sample: Vec<u8>,
    pub combat_damage_ticks_sample: Vec<u8>,
    pub combat_projectile_id_sample: String,
    pub ai_pressure_wave_units_sample: Vec<String>,
    pub ai_pressure_tiles_sample: Vec<String>,
    pub ai_pressure_counter_tiles_sample: Vec<String>,
    pub enemy_pressure_wave_units_sample: Vec<String>,
    pub enemy_pressure_lane_tiles_sample: Vec<String>,
    pub recon_scout_route_tiles_sample: Vec<String>,
    pub recon_fog_reveal_tiles_sample: Vec<String>,
    pub recon_enemy_structures_sample: Vec<String>,
    pub recon_enemy_units_sample: Vec<String>,
    pub recon_enemy_structure_tile_sample: RtsEvidencePoint,
    pub recon_enemy_unit_tile_sample: RtsEvidencePoint,
    pub base_assault_path_tiles_sample: Vec<String>,
    pub base_assault_targets_sample: Vec<String>,
    pub aftermath_debris_tiles_sample: Vec<String>,
    pub aftermath_smoke_tiles_sample: Vec<String>,
    pub commander_aura_tiles_sample: Vec<String>,
    pub commander_loot_items_sample: Vec<String>,
    pub expansion_claim_tiles_sample: Vec<String>,
    pub expansion_structure_tile_sample: RtsEvidencePoint,
    pub expansion_workers_sample: Vec<String>,
    pub counterattack_units_sample: Vec<String>,
    pub counterattack_route_tiles_sample: Vec<String>,
    pub army_units_sample: Vec<String>,
    pub army_rally_tiles_sample: Vec<String>,
    pub player_army_unit_tile_sample: RtsEvidencePoint,
    pub central_keep_route_tiles_sample: Vec<String>,
    pub central_keep_tile_sample: RtsEvidencePoint,
    pub boss_guard_units_sample: Vec<String>,
    pub player_siege_line_tiles_sample: Vec<String>,
    pub keep_breach_tiles_sample: Vec<String>,
    pub guardian_counter_units_sample: Vec<String>,
    pub keep_claim_tiles_sample: Vec<String>,
    pub objective_tiles_sample: Vec<String>,
    pub creep_camp_tiles_sample: Vec<String>,
    pub terrain_route_tiles_sample: Vec<String>,
    pub terrain_choke_tiles_sample: Vec<String>,
    pub expansion_tiles_sample: Vec<String>,
    pub siege_units_sample: Vec<String>,
    pub siege_push_route_tiles_sample: Vec<String>,
    pub siege_breach_tiles_sample: Vec<String>,
    pub enemy_fortification_tile_sample: RtsEvidencePoint,
    pub enemy_repair_units_sample: Vec<String>,
    pub enemy_flank_units_sample: Vec<String>,
    pub enemy_flank_tile_sample: RtsEvidencePoint,
    pub player_hold_tiles_sample: Vec<String>,
    pub inner_lane_tiles_sample: Vec<String>,
    pub inner_gate_tile_sample: RtsEvidencePoint,
    pub signal_lock_tile_sample: RtsEvidencePoint,
    pub inner_defenders_sample: Vec<String>,
    pub supply_convoy_sample: Vec<String>,
    pub split_squad_tiles_sample: Vec<String>,
    pub inner_core_tile_sample: RtsEvidencePoint,
    pub restored_zones_sample: Vec<String>,
    pub rebuild_structures_sample: Vec<String>,
    pub garrison_units_sample: Vec<String>,
    pub open_world_route_tiles_sample: Vec<String>,
    pub open_world_panels_sample: Vec<String>,
    pub siege_unit_tile_sample: RtsEvidencePoint,
    pub harvest_tile_sample: RtsEvidencePoint,
    pub dropoff_tile_sample: RtsEvidencePoint,
    pub build_site_tiles_sample: Vec<String>,
    pub structure_tile_sample: RtsEvidencePoint,
    pub unlock_unit_tile_sample: RtsEvidencePoint,
    pub queue_gold_cost_sample: u64,
    pub queue_available_gold_sample: u64,
    pub queue_affordable_sample: bool,
    pub queue_build_parts_sample: Vec<String>,
    pub queue_production_lane_sample: bool,
    pub queue_feedback_chip_sample: String,
    pub blocked_feedback_chip_visible_sample: bool,
    pub queue_blocked_feedback_label_sample: String,
    pub command_panel_slot_id_sample: String,
    pub command_panel_build_palette_queue_id_sample: String,
    pub command_panel_production_slot_queue_id_sample: String,
    pub command_panel_sidebar_cancel_queue_id_sample: Option<String>,
    pub command_panel_palette_cancel_queue_id_sample: Option<String>,
    pub command_panel_sidebar_slot_status_label_sample: String,
    pub command_panel_palette_state_label_sample: String,
    pub command_panel_sidebar_queue_summary_sample: String,
    pub command_panel_spawned_unit_id_sample: String,
    pub command_panel_structure_id_sample: String,
    pub scripted_demo_pauses_queue_tick_sample: bool,
    pub scripted_demo_stage_from_frame_sample: Option<usize>,
    pub scripted_demo_stage_id_sample: String,
    pub scripted_demo_stage_title_sample: String,
    pub selection_default_units_sample: Vec<String>,
    pub selection_same_class_units_sample: Vec<String>,
    pub selection_guard_tile_sample: Option<RtsEvidencePoint>,
    pub selection_drag_units_sample: Vec<String>,
    pub selection_drag_rejected_units_sample: Vec<String>,
    pub selection_drag_distance_sq_sample: i32,
    pub selection_drag_ready_sample: bool,
    pub selection_drag_group_id_sample: String,
    pub selection_drag_player_label_sample: String,
    pub selection_tiles_for_units_sample: Vec<String>,
    pub control_group_hotkey_slot_sample: Option<String>,
    pub control_group_default_slot_three_units_sample: Vec<String>,
    pub control_group_assignment_units_sample: Vec<String>,
    pub control_group_summary_slot_ten_sample: RtsControlGroupSlotSummary,
    pub control_group_merged_units_sample: Vec<String>,
    pub selection_clear_parts_sample: Option<(String, Option<String>, String)>,
    pub move_command_parts_sample: Vec<String>,
    pub line_path_tiles_sample: Vec<String>,
    pub focus_fire_units_sample: Vec<String>,
    pub creep_camp_units_sample: Vec<String>,
    pub command_parts_samples: Vec<Vec<String>>,
    pub selection_command_stamp_sample: RtsCommandStamp,
    pub move_command_stamp_sample: RtsCommandStamp,
    pub ability_command_stamp_sample: RtsCommandStamp,
    pub order_queue_replay_action_samples: Vec<RtsOrderQueueReplayAction>,
    pub command_feedback_strip_stage_sample: Option<String>,
    pub command_feedback_strip_fixture_stage_names_sample: Vec<String>,
    pub command_feedback_strip_fixture_action_payloads_sample: Vec<String>,
    pub command_feedback_strip_fixture_focus_tiles_sample: Vec<String>,
    pub command_feedback_strip_fixture_filtered_members_sample: Vec<String>,
    pub command_feedback_lifecycle_stage_sample: Option<String>,
    pub command_feedback_lifecycle_fixture_stage_names_sample: Vec<String>,
    pub command_feedback_lifecycle_fixture_action_payloads_sample: Vec<String>,
    pub command_feedback_lifecycle_fixture_age_ticks_sample: Vec<u8>,
    pub command_feedback_lifecycle_fixture_events_sample: Vec<String>,
    pub command_feedback_replay_step_names_sample: Vec<String>,
    pub command_feedback_replay_preview_stages_sample: Vec<String>,
    pub command_feedback_replay_retained_group_ids_sample: Vec<String>,
    pub command_feedback_replay_pruned_group_ids_sample: Vec<String>,
    pub command_feedback_replay_history_badges_sample: Vec<String>,
    pub command_feedback_rejection_replay_step_names_sample: Vec<String>,
    pub command_feedback_rejection_replay_preview_stages_sample: Vec<String>,
    pub command_feedback_rejection_replay_input_sources_sample: Vec<String>,
    pub command_feedback_rejection_replay_blocked_reasons_sample: Vec<String>,
    pub command_feedback_rejection_replay_visual_stages_sample: Vec<String>,
    pub command_feedback_rejection_replay_retained_group_ids_sample: Vec<String>,
    pub command_feedback_rejection_replay_pruned_group_ids_sample: Vec<String>,
    pub command_history_visible_sample: bool,
    pub command_history_prune_visible_sample: bool,
    pub command_history_fixture_stage_names_sample: Vec<String>,
    pub command_history_fixture_lifecycle_stages_sample: Vec<String>,
    pub command_history_fixture_group_ids_sample: Vec<String>,
    pub command_history_prune_fixture_stage_names_sample: Vec<String>,
    pub command_history_prune_fixture_pruned_group_ids_sample: Vec<String>,
    pub command_history_prune_fixture_prune_reasons_sample: Vec<String>,
    pub command_execution_feedback_kind_samples: Vec<String>,
    pub command_execution_target_label_samples: Vec<String>,
    pub command_execution_player_label_samples: Vec<String>,
    pub command_execution_target_tile_samples: Vec<RtsEvidencePoint>,
    pub hover_target_preview_kind_sample: Option<String>,
    pub hover_cursor_kind_sample: String,
    pub hover_cursor_label_sample: String,
    pub blocked_cursor_kind_sample: String,
    pub blocked_cursor_label_sample: String,
    pub hover_player_label_sample: String,
    pub hover_queue_player_label_sample: String,
    pub blocked_hover_player_label_sample: String,
    pub unit_status_stage_sample: Option<String>,
    pub unit_status_unit_id_sample: String,
    pub unit_status_health_sample: u8,
    pub unit_status_energy_sample: u8,
    pub unit_status_role_badges_sample: Vec<String>,
    pub selection_feedback_stage_sample: Option<String>,
    pub ability_tooltip_stage_sample: Option<String>,
    pub control_group_hotkey_feedback_stage_sample: Option<String>,
    pub source_of_truth: String,
}

pub fn first_contact_bevy_runtime_adapter_evidence() -> RtsBevyRuntimeAdapterEvidence {
    let minimap_cell = rts_minimap_cell_origin(10, 20, 4, 5, (32, 32));
    let scroll_camera_stage_summaries = rts_scrollable_map_camera_stage_summaries();
    let scroll_camera_first_focus_tile = scroll_camera_stage_summaries
        .first()
        .map(|summary| summary.focus_tile)
        .unwrap_or_default();
    let scroll_camera_minimap_jump_tile = scroll_camera_stage_summaries
        .iter()
        .find(|summary| summary.stage == "minimap_jump")
        .and_then(|summary| summary.step.minimap_tile_id.clone());
    let scroll_camera_bounds_clamped = scroll_camera_stage_summaries
        .iter()
        .any(|summary| summary.stage == "bounds_clamp" && summary.step.clamped);
    let camera_minimap_stage_summaries = rts_camera_minimap_sync_stage_summaries();
    let camera_minimap_viewport_rect = camera_minimap_stage_summaries
        .first()
        .map(|summary| summary.viewport_rect)
        .unwrap_or(RtsCameraMinimapViewportRect {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        });
    let camera_minimap_selection_follow_tile = camera_minimap_stage_summaries
        .iter()
        .find(|summary| summary.stage == "selection_follow")
        .and_then(|summary| summary.step.minimap_tile_id.clone());
    let mut camera_minimap_revealed_union = Vec::new();
    for summary in &camera_minimap_stage_summaries {
        for tile_id in &summary.revealed_tile_ids {
            if !camera_minimap_revealed_union.contains(tile_id) {
                camera_minimap_revealed_union.push(tile_id.clone());
            }
        }
    }
    let camera_minimap_zoom_rect_area = camera_minimap_stage_summaries
        .iter()
        .find(|summary| summary.stage == "zoom_sync")
        .map(|summary| summary.viewport_rect_area)
        .unwrap_or_default();
    let preview_queue = vec!["command_queue_path_preview:queue_stack".to_string()];
    let path_preview =
        rts_command_queue_path_preview_stage(&[], &preview_queue, 0).map(str::to_string);
    let command_queue_path_preview_fixtures = rts_command_queue_path_preview_stage_fixtures();
    let command_queue_path_preview_action_kinds = command_queue_path_preview_fixtures
        .iter()
        .map(|fixture| fixture.action.kind.clone())
        .collect::<Vec<_>>();
    let command_queue_path_preview_action_payloads = command_queue_path_preview_fixtures
        .iter()
        .map(|fixture| fixture.action.payload.clone())
        .collect::<Vec<_>>();
    let command_queue_path_preview_history_entries = command_queue_path_preview_fixtures
        .iter()
        .map(|fixture| fixture.history_entry.clone())
        .collect::<Vec<_>>();
    let formation_preview_stage = rts_formation_move_preview_stage(
        &["formation_move_preview:commit_spacing".to_string()],
        &["formation_move_preview:destination_ghost".to_string()],
        0,
    )
    .map(str::to_string);
    let formation_preview_fixtures = rts_formation_move_preview_stage_fixtures();
    let formation_preview_action_payloads = formation_preview_fixtures
        .iter()
        .map(|fixture| fixture.action.payload.clone())
        .collect::<Vec<_>>();
    let formation_preview_history_entries = formation_preview_fixtures
        .iter()
        .map(|fixture| fixture.history_entry.clone())
        .collect::<Vec<_>>();
    let formation_preview_destination_slots = formation_preview_fixtures
        .iter()
        .find(|fixture| fixture.stage == "destination_ghost")
        .map(|fixture| fixture.formation_slot_tile_ids.clone())
        .unwrap_or_default();
    let formation_preview_split_route = formation_preview_fixtures
        .iter()
        .find(|fixture| fixture.stage == "split_avoidance")
        .map(|fixture| fixture.group_route_tile_ids_if_empty.clone())
        .unwrap_or_default();
    let recall_formation_fixtures = rts_control_group_recall_formation_preview_stage_fixtures();
    let recall_formation_action_payloads = recall_formation_fixtures
        .iter()
        .map(|fixture| fixture.action.payload.clone())
        .collect::<Vec<_>>();
    let recall_formation_history_entries = recall_formation_fixtures
        .iter()
        .map(|fixture| fixture.history_entry.clone())
        .collect::<Vec<_>>();
    let recall_formation_slot_tiles = recall_formation_fixtures
        .iter()
        .find(|fixture| fixture.stage == "formation_anchor_slots")
        .map(|fixture| fixture.formation_slot_tile_ids.clone())
        .unwrap_or_default();
    let recall_formation_filtered_members = recall_formation_fixtures
        .iter()
        .find(|fixture| fixture.stage == "filtered_invalid")
        .map(|fixture| fixture.filtered_member_ids.clone())
        .unwrap_or_default();
    let recall_override_fixtures = rts_control_group_recall_override_preview_stage_fixtures();
    let recall_override_action_payloads = recall_override_fixtures
        .iter()
        .map(|fixture| fixture.action.payload.clone())
        .collect::<Vec<_>>();
    let recall_override_history_entries = recall_override_fixtures
        .iter()
        .map(|fixture| fixture.history_entry.clone())
        .collect::<Vec<_>>();
    let recall_override_final_tiles = recall_override_fixtures
        .iter()
        .find(|fixture| fixture.stage == "group_27_override_cancel")
        .map(|fixture| fixture.override_final_tile_ids.clone())
        .unwrap_or_default();
    let recall_override_canceled_members = recall_override_fixtures
        .iter()
        .find(|fixture| fixture.stage == "group_27_override_cancel")
        .map(|fixture| fixture.canceled_member_ids.clone())
        .unwrap_or_default();
    let formation_execution_stage = rts_formation_move_execution_stage(
        &["formation_move_execution:arrival_lock".to_string()],
        &["formation_move_execution:slot_claim".to_string()],
        0,
    )
    .map(str::to_string);
    let formation_execution_fixtures = rts_formation_move_execution_fixtures();
    let formation_execution_stage_names = formation_execution_fixtures
        .stages
        .iter()
        .map(|fixture| fixture.stage.clone())
        .collect::<Vec<_>>();
    let formation_execution_action_payloads = formation_execution_fixtures
        .stages
        .iter()
        .map(|fixture| fixture.action.payload.clone())
        .collect::<Vec<_>>();
    let formation_execution_arrival_route = formation_execution_fixtures
        .stages
        .iter()
        .find(|fixture| fixture.stage == "arrival_lock")
        .map(|fixture| fixture.group_route_tile_ids.clone())
        .unwrap_or_default();
    let local_obstruction_stage = rts_local_obstruction_recovery_stage(
        &["local_obstruction_recovery:flow_resume".to_string()],
        &["local_obstruction_recovery:detect_block".to_string()],
        0,
    )
    .map(str::to_string);
    let local_obstruction_fixtures = rts_local_obstruction_recovery_fixtures();
    let local_obstruction_stage_names = local_obstruction_fixtures
        .stages
        .iter()
        .map(|fixture| fixture.stage.clone())
        .collect::<Vec<_>>();
    let local_obstruction_action_payloads = local_obstruction_fixtures
        .stages
        .iter()
        .map(|fixture| fixture.action.payload.clone())
        .collect::<Vec<_>>();
    let local_obstruction_blocked_tiles = local_obstruction_fixtures
        .stages
        .iter()
        .find(|fixture| fixture.stage == "detect_block")
        .map(|fixture| fixture.blocked_tile_ids.clone())
        .unwrap_or_default();
    let local_obstruction_resume_route = local_obstruction_fixtures
        .stages
        .iter()
        .find(|fixture| fixture.stage == "flow_resume")
        .map(|fixture| fixture.group_route_tile_ids.clone())
        .unwrap_or_default();
    let npc_behavior_stage = rts_npc_behavior_stage(
        &["behavior:creep_retreat".to_string()],
        &["behavior:guard_patrol".to_string()],
        0,
    )
    .map(str::to_string);
    let combat_impact_stage =
        rts_combat_impact_stage(&[], &["impact:damage_tick".to_string()], 1).map(str::to_string);
    let locomotion_blend_stage =
        rts_locomotion_blend_stage(&["locomotion:formation_slide".to_string()], &[], 0)
            .map(str::to_string);
    let npc_transition_stage = rts_npc_transition_stage(
        &["transition:hit_recover".to_string()],
        &["transition:alert_turn".to_string()],
        0,
    )
    .map(str::to_string);
    let depth_readability_stage =
        rts_depth_readability_stage(&["depth:target_priority".to_string()], &[], 0)
            .map(str::to_string);
    let structure_modeling_stage = rts_structure_modeling_stage(
        &["structure:repair_beam".to_string()],
        &["structure:foundation_shadow".to_string()],
        0,
    )
    .map(str::to_string);
    let environment_life_stage =
        rts_environment_life_stage(&[], &["environment:resource_glint".to_string()], 0)
            .map(str::to_string);
    let worker_harvest_animation_stage = rts_worker_harvest_animation_stage(
        &["harvest_anim:return_path".to_string()],
        &["harvest_anim:approach".to_string()],
        0,
    )
    .map(str::to_string);
    let production_spawn_animation_stage = rts_production_spawn_animation_stage(
        &["production_spawn_anim:supply_flash".to_string()],
        &["production_spawn_anim:queue_pulse".to_string()],
        0,
    )
    .map(str::to_string);
    let action_cadence_attack_marks = rts_action_cadence_marks("actor_guard_attack");
    let action_cadence_carry_marks = rts_action_cadence_marks("actor_worker_carry");
    let action_cadence_idle_marks = rts_action_cadence_marks("actor_guard_idle");
    let action_cadence_creep_windup_offset = rts_action_cadence_marks("actor_creep_attack")
        .first()
        .map(|mark| mark.rect.x)
        .unwrap_or_default();
    let action_sequence_phase = rts_action_sequence_phase(
        "actor_guard_attack",
        &["sequence:recovery".to_string()],
        &["sequence:windup".to_string()],
        2,
        2,
        true,
    )
    .map(str::to_string);
    let action_sequence_windup_marks = rts_action_sequence_marks("actor_guard_attack", "windup");
    let action_sequence_strike_marks = rts_action_sequence_marks("actor_guard_attack", "strike");
    let action_sequence_carry_down_marks =
        rts_action_sequence_marks("actor_worker_carry", "carry_down");
    let action_sequence_idle_marks = rts_action_sequence_marks("actor_guard_idle", "idle");
    let unit_model_depth_guard_marks = rts_unit_model_depth_marks("actor_guard_attack");
    let unit_model_depth_worker_marks = rts_unit_model_depth_marks("actor_worker_carry");
    let unit_model_depth_creep_marks = rts_unit_model_depth_marks("actor_creep_attack");
    let unit_model_depth_creep_role_prop_count = unit_model_depth_creep_marks
        .iter()
        .filter(|mark| mark.kind == "role_prop")
        .count();
    let unit_model_depth_face_shade_offset = unit_model_depth_guard_marks
        .iter()
        .find(|mark| mark.kind == "face_shade")
        .map(|mark| mark.rect.y)
        .unwrap_or_default();
    let command_surface_stage = rts_command_surface_stage(
        7,
        &["surface:target_queue".to_string()],
        &["surface:command_grid".to_string()],
    )
    .map(str::to_string);
    let command_grid_hit = rts_runtime_hit_test_grid(
        RtsRuntimeGridSpec {
            origin_x: 360,
            origin_y: 572,
            columns: 6,
            count: 12,
            stride_x: 58,
            stride_y: 46,
            slot_width: 48,
            slot_height: 38,
        },
        363,
        575,
    );
    let tile_line = rts_runtime_tile_line((8, 8), (12, 16));
    let combat_engagement_tiles = rts_engagement_tiles_for_target("enemy_barracks");
    let combat_flash_tiles = rts_contact_flash_tiles_for_target("arena_creep_attack");
    let combat_target_tile = rts_target_tile_for_id("forest_shaman_support", 0);
    let combat_target_priority = rts_target_priority_ids_for_target("arena_creep_attack");
    let combat_projectile_trail = rts_projectile_trail_tiles_for_target("forest_creep_camp");
    let combat_ability_effect_tiles =
        rts_ability_effect_tiles_for_target("enemy_barracks", "guard_break");
    let combat_threat_levels = rts_threat_levels_for_target("enemy_barracks");
    let combat_damage_ticks = rts_damage_ticks_for_ability("guard_break");
    let combat_projectile_id = rts_projectile_id_for_ability("guard_break");
    let ai_pressure_wave_units = rts_ai_wave_unit_ids_for_pressure("skirmish_wave");
    let ai_pressure_tiles = rts_ai_pressure_tiles_for_pressure("skirmish_wave");
    let ai_pressure_counter_tiles = rts_ai_counter_tiles_for_pressure("skirmish_wave");
    let enemy_pressure_wave_units = rts_enemy_pressure_wave_units_for_id("raider_wave");
    let enemy_pressure_lane_tiles = rts_enemy_pressure_lane_tiles_for_wave("raider_wave");
    let recon_scout_route_tiles = rts_scout_route_tiles_for_recon("enemy_base");
    let recon_fog_reveal_tiles = rts_fog_reveal_tiles_for_recon("enemy_base", "mark");
    let recon_enemy_structures = rts_enemy_structures_for_recon("enemy_base", "mark");
    let recon_enemy_units = rts_enemy_units_for_recon("enemy_base", "mark");
    let recon_enemy_structure_tile = rts_enemy_structure_tile_for_id("enemy_resource_vault", 2);
    let recon_enemy_unit_tile = rts_enemy_unit_tile_for_id("enemy_guard", 2);
    let base_assault_path_tiles = rts_base_assault_path_tiles_for_target("enemy_barracks", "10,3");
    let base_assault_targets = rts_base_assault_targets_for_id("enemy_barracks");
    let aftermath_debris_tiles = rts_aftermath_debris_tiles_for_id("enemy_barracks", "10,3");
    let aftermath_smoke_tiles = rts_aftermath_smoke_tiles_for_id("enemy_barracks", "10,3");
    let commander_aura_tiles = rts_commander_aura_tiles_for_id("mirror_captain");
    let commander_loot_items = rts_loot_items_for_id("enemy_barracks");
    let expansion_claim_tiles = rts_expansion_tiles_for_id("forest_relay", "9,2");
    let expansion_structure_tile = rts_expansion_structure_tile_for_id("watch_lantern");
    let expansion_workers = rts_expansion_workers_for_line("gold_line");
    let counterattack_units = rts_counterattack_units_for_wave("counter_wave");
    let counterattack_route_tiles = rts_counterattack_route_tiles_for_wave("counter_wave", "8,3");
    let army_units = rts_army_units_for_batch("mixed_vanguard");
    let army_rally_tiles = rts_army_rally_tiles_for_id("forward_watch");
    let player_army_unit_tile = rts_player_army_unit_tile_for_id("field_mender", 3);
    let central_keep_route_tiles = rts_central_keep_route_tiles_for_id("central_keep", "13,3");
    let central_keep_tile = rts_central_keep_tile_for_id("central_keep");
    let boss_guard_units = rts_boss_guard_units_for_id("warden_line");
    let player_siege_line_tiles = rts_player_siege_line_tiles_for_id("final_line", "12,4");
    let keep_breach_tiles = rts_keep_breach_tiles_for_id("central_keep", "13,3");
    let guardian_counter_units = rts_guardian_counter_units_for_id("high_warden");
    let keep_claim_tiles = rts_keep_claim_tiles_for_id("central_keep", "13,3");
    let objective_tiles = rts_objective_tiles_for_id("relay_beacon", "6,5");
    let creep_camp_tiles = rts_creep_camp_tiles_for_id("forest_creep_camp", "8,3");
    let terrain_route_tiles = rts_terrain_route_tiles_for_camp("forest_creep_camp");
    let terrain_choke_tiles = rts_terrain_choke_tiles_for_camp("forest_creep_camp");
    let expansion_tiles = rts_expansion_tiles_for_camp("forest_creep_camp");
    let siege_units = rts_siege_units_for_id("stonebreak_cart");
    let siege_push_route_tiles = rts_siege_push_route_tiles_for_target("gate_bulwark", "10,3");
    let siege_breach_tiles = rts_siege_breach_tiles_for_target("gate_bulwark", "10,3");
    let enemy_fortification_tile = rts_enemy_fortification_tile_for_id("gate_bulwark");
    let enemy_repair_units = rts_enemy_repair_units_for_target("gate_bulwark");
    let enemy_flank_units = rts_enemy_flank_units_for_id("ridge_sentries");
    let enemy_flank_tile = rts_enemy_flank_tile_for_index(2);
    let player_hold_tiles = rts_player_hold_tiles_for_id("shield_line", "9,3");
    let inner_lane_tiles = rts_inner_lane_tiles_for_id("inner_lane", "11,2");
    let inner_gate_tile = rts_inner_gate_tile_for_id("inner_latch");
    let signal_lock_tile = rts_inner_gate_tile_for_id("signal_lock");
    let inner_defenders = rts_inner_defenders_for_id("second_line");
    let supply_convoy = rts_supply_convoy_for_id("relay_convoy");
    let split_squad_tiles = rts_split_squad_tiles_for_id("flank_team", "10,4");
    let inner_core_tile = rts_inner_core_tile_for_id("signal_core");
    let restored_zones = rts_restored_zones_for_id("mirror_city");
    let rebuild_structures = rts_rebuild_structures_for_id("signal_core");
    let garrison_units = rts_garrison_units_for_id("central_keep");
    let open_world_route_tiles = rts_open_world_route_tiles_for_id("league-coliseum");
    let open_world_panels = rts_open_world_panels_for_room("league-coliseum");
    let siege_unit_tile = rts_siege_unit_tile_for_id("stonebreak_cart", 0);
    let harvest_tile = rts_harvest_tile_for_node("gold_vein");
    let dropoff_tile = rts_dropoff_tile_for_structure("town_hall");
    let build_site_tiles = rts_build_site_tiles("7,4");
    let structure_tile = rts_structure_tile_for_id("training_hall");
    let unlock_unit_tile = rts_unlock_unit_tile_for_id("relay_guard");
    let queue_resource_spend_log = vec!["commit:1200g:prior_queue_pressure".to_string()];
    let queue_gold_cost = rts_queue_gold_cost("build:watch_tower@7,4");
    let queue_available_gold = rts_available_gold(0, &queue_resource_spend_log);
    let queue_affordable =
        rts_queue_is_affordable(0, &queue_resource_spend_log, "build:watch_tower@7,4");
    let queue_build_parts = rts_build_parts("build:watch_tower@7,4");
    let queue_production_lane = rts_queue_uses_production_lane("train:worker");
    let queue_feedback_chip = rts_queue_feedback_chip("build:watch_tower@7,4");
    let blocked_feedback_chip_visible = rts_blocked_feedback_chip_visible(&[
        "queue:train:worker".to_string(),
        "feedback:blocked:queue:rts_queue_unaffordable:build:watch_tower@7,4".to_string(),
    ]);
    let queue_blocked_feedback_label = rts_blocked_feedback_player_label(
        "feedback:blocked:queue:rts_queue_unaffordable:build:watch_tower@7,4",
    );
    let command_panel_slot_ids = vec!["move".to_string(), "stop".to_string(), "attack".to_string()];
    let command_panel_production_queue = vec![
        "train:worker".to_string(),
        "upgrade:signal_blade".to_string(),
    ];
    let command_panel_build_queue = vec!["build:watch_tower@7,4".to_string()];
    let command_panel_slot_id =
        rts_command_slot_id_for_index(&[], Some(&command_panel_slot_ids), "hold", 2);
    let command_panel_build_palette_queue_id = rts_build_palette_queue_id_for_slot(None, 3);
    let command_panel_production_slot_queue_id = rts_production_slot_queue_id(
        &command_panel_production_queue,
        &command_panel_build_queue,
        "train:guard",
        "build:training_hall@4,3",
        2,
    );
    let command_panel_sidebar_cancel_queue_id = rts_sidebar_cancel_queue_id(
        &command_panel_production_queue,
        &command_panel_build_queue,
        2,
    );
    let command_panel_palette_cancel_queue_id =
        rts_palette_cancel_queue_id(&[], &[], Some("refinery"), "build:refinery@6,4");
    let command_panel_sidebar_slot_status_label = rts_sidebar_slot_status_label(
        &command_panel_production_queue,
        &command_panel_build_queue,
        true,
        2,
        66,
    );
    let command_panel_palette_state_label =
        rts_palette_state_label(Some("refinery"), &[], &[], true, "build:refinery@6,4");
    let command_panel_sidebar_queue_summary = rts_sidebar_queue_summary(
        &command_panel_production_queue,
        &command_panel_build_queue,
        42,
        66,
    );
    let command_panel_spawned_unit_id = rts_spawned_unit_id_from_queue("train:worker", 2);
    let command_panel_structure_id = rts_structure_id_from_queue("build:watch_tower@7,4");
    let scripted_demo_pauses_queue_tick =
        rts_scripted_demo_pauses_queue_tick("queue_cancel_refund_sequence");
    let scripted_demo_stage_from_frame =
        rts_scripted_demo_stage_from_frame("queue_cancel_refund_sequence", 240);
    let scripted_demo_stage_id = rts_scripted_demo_stage_id(3);
    let scripted_demo_stage_title = rts_scripted_demo_stage_title(4);
    let selection_default_units = rts_default_group_units();
    let selection_same_class_units = rts_same_class_units("player");
    let selection_guard_tile = rts_selectable_unit_tile("square_guard_patrol");
    let selection_drag_units = rts_drag_selected_units((4, 4), (8, 5));
    let selection_drag_rejected_units = rts_drag_rejected_unit_ids((5, 4), (9, 5));
    let selection_drag_distance_sq = rts_drag_distance_sq((240, 180), (520, 350));
    let selection_drag_ready = rts_drag_select_ready((240, 180), (520, 350));
    let selection_drag_group_id = rts_drag_group_id((4, 4), (8, 5));
    let selection_drag_player_label =
        rts_drag_select_player_label("4,4", "8,5", selection_drag_units.len());
    let selection_tiles_for_units = rts_selection_tiles_for_units(&[
        "player".to_string(),
        "square_guard_front".to_string(),
        "square_worker_carry".to_string(),
    ]);
    let control_group_assignments = vec![
        "2:player|square_guard_patrol".to_string(),
        "10:camera:square_worker_carry|square_worker_harvest".to_string(),
    ];
    let control_group_active_ids = vec!["10".to_string()];
    let control_group_hotkey_slot = rts_control_group_hotkey_slot("assign:10", "assign:");
    let control_group_default_slot_three_units = rts_default_units_for_control_group_slot("3");
    let control_group_assignment_units =
        rts_units_from_control_group_assignment(&control_group_assignments, "10");
    let control_group_summary_slot_ten = rts_control_group_slot_summaries(
        &control_group_assignments,
        &control_group_active_ids,
        Some("2"),
    )
    .into_iter()
    .find(|summary| summary.slot == "10")
    .expect("slot 10 summary");
    let control_group_merged_units = rts_merged_unit_ids(
        &["player".to_string()],
        &["player".to_string(), "square_worker_carry".to_string()],
    );
    let selection_clear_parts = rts_selection_clear_parts("clear:hostile:square_creep_wander@9,4");
    let move_command_parts = rts_move_command_parts("minimap:9,2:attack_move");
    let move_command_parts_sample = vec![
        move_command_parts.0.to_string(),
        move_command_parts.1.to_string(),
    ];
    let line_path_tiles = rts_line_path_tiles((5, 5), (8, 3));
    let focus_fire_units = rts_focus_fire_units_for_target("enemy_barracks");
    let creep_camp_units = rts_creep_camp_units_for_id("forest_creep_camp");
    let command_parts_samples = vec![
        rts_objective_parts("claim:relay_beacon@9,2"),
        rts_creep_camp_parts("camp", "clear:creep_camp@8,3"),
        rts_recon_parts("mark:scout_enemy_base@10,2"),
        rts_enemy_command_parts("pressure:counter_wave@enemy_gate", "pressure", "enemy_base"),
        rts_counter_command_parts("upgrade:signal_blade@training_hall"),
        rts_army_command_parts("train:mixed_vanguard@training_hall"),
        rts_base_assault_parts("breach:enemy_barracks@10,3"),
        rts_aftermath_parts("destroy:enemy_barracks@10,3"),
        rts_commander_parts("level:mirror_captain@forest_relay"),
        rts_expansion_parts("claim:forest_relay@9,2"),
        rts_tier_two_parts("tech:stonebreak_cart@relay_outpost"),
    ]
    .into_iter()
    .map(|(kind, id, source_id)| vec![kind, id, source_id])
    .collect::<Vec<_>>();
    let selection_command_stamp =
        rts_command_stamp_for_selection("classic_rts_hotkey", "assign:5", 2);
    let move_command_stamp = rts_command_stamp_for_move("classic_rts_mouse_viewport", "7,4:line")
        .expect("move command stamp sample");
    let ability_command_stamp = rts_command_stamp_for_ability(
        "classic_rts_mouse_command_bar",
        "focus_fire",
        Some("arena_creep_attack"),
    );
    let order_queue_replay_action_samples = vec![
        rts_order_queue_replay_action("queue:attack:arena_creep_attack", "focus_fire"),
        rts_order_queue_replay_action("queue:move:9,2", "focus_fire"),
        rts_order_queue_replay_action("minimap:rally:5,2", "focus_fire"),
        rts_order_queue_replay_action("queue:train:worker", "focus_fire"),
        rts_order_queue_replay_action("queue:select_group_3", "focus_fire"),
        rts_order_queue_replay_action("feedback:build_placed:watch_tower@7,4", "focus_fire"),
    ];
    let command_feedback_queue = vec![
        "queued_group_order:Multi0:26:move:2actors".to_string(),
        "control_group_command_feedback_strip:group_27_override".to_string(),
        "control_group_command_history:dimmed_history_retained".to_string(),
        "history_row_pruned:25:old_queue:17,30:age16".to_string(),
    ];
    let command_feedback_events =
        vec!["control_group_command_feedback_lifecycle:dimmed".to_string()];
    let command_feedback_strip_stage =
        rts_command_feedback_strip_stage(1, &[], &command_feedback_queue).map(str::to_string);
    let command_feedback_strip_fixtures = rts_control_group_command_feedback_strip_fixtures();
    let command_feedback_strip_fixture_stage_names = command_feedback_strip_fixtures
        .stages
        .iter()
        .map(|fixture| fixture.stage.clone())
        .collect::<Vec<_>>();
    let command_feedback_strip_fixture_action_payloads = command_feedback_strip_fixtures
        .stages
        .iter()
        .map(|fixture| fixture.action.payload.clone())
        .collect::<Vec<_>>();
    let command_feedback_strip_fixture_focus_tiles = command_feedback_strip_fixtures
        .stages
        .iter()
        .map(|fixture| fixture.recall_focus_tile.clone())
        .collect::<Vec<_>>();
    let command_feedback_strip_fixture_filtered_members = command_feedback_strip_fixtures
        .stages
        .iter()
        .find(|fixture| fixture.stage == "group_28_filtered")
        .map(|fixture| fixture.filtered_member_ids.clone())
        .unwrap_or_default();
    let command_feedback_lifecycle_stage =
        rts_command_feedback_lifecycle_stage("", &command_feedback_events, &command_feedback_queue)
            .map(str::to_string);
    let command_feedback_lifecycle_fixtures =
        rts_control_group_command_feedback_lifecycle_fixtures();
    let command_feedback_lifecycle_fixture_stage_names = command_feedback_lifecycle_fixtures
        .stages
        .iter()
        .map(|fixture| fixture.stage.clone())
        .collect::<Vec<_>>();
    let command_feedback_lifecycle_fixture_action_payloads = command_feedback_lifecycle_fixtures
        .stages
        .iter()
        .map(|fixture| fixture.action.payload.clone())
        .collect::<Vec<_>>();
    let command_feedback_lifecycle_fixture_age_ticks = command_feedback_lifecycle_fixtures
        .stages
        .iter()
        .map(|fixture| fixture.age_ticks)
        .collect::<Vec<_>>();
    let command_feedback_lifecycle_fixture_events = command_feedback_lifecycle_fixtures
        .stages
        .iter()
        .map(|fixture| fixture.lifecycle_event.clone())
        .collect::<Vec<_>>();
    let command_history_visible =
        rts_command_history_visible("", &command_feedback_events, &command_feedback_queue);
    let command_history_prune_visible =
        rts_command_history_prune_visible("", &command_feedback_events, &command_feedback_queue);
    let command_history_fixtures = rts_control_group_command_history_fixtures();
    let command_history_fixture_stage_names = command_history_fixtures
        .stages
        .iter()
        .map(|fixture| fixture.stage.clone())
        .collect::<Vec<_>>();
    let command_history_fixture_lifecycle_stages = command_history_fixtures
        .stages
        .iter()
        .map(|fixture| fixture.lifecycle_stage.clone())
        .collect::<Vec<_>>();
    let command_history_prune_fixtures = rts_control_group_command_history_prune_fixtures();
    let command_history_prune_fixture_stage_names = command_history_prune_fixtures
        .stages
        .iter()
        .map(|fixture| fixture.stage.clone())
        .collect::<Vec<_>>();
    let command_history_prune_fixture_prune_reasons = command_history_prune_fixtures
        .pruned_history_entries
        .iter()
        .filter_map(|entry| entry.prune_reason.clone())
        .collect::<Vec<_>>();
    let command_feedback_replay_fixtures = rts_control_group_command_feedback_replay_fixtures();
    let command_feedback_replay_step_names = command_feedback_replay_fixtures
        .command_steps
        .iter()
        .map(|step| step.step_name.clone())
        .collect::<Vec<_>>();
    let command_feedback_replay_preview_stages = command_feedback_replay_fixtures
        .command_steps
        .iter()
        .filter_map(|step| step.preview_stage.clone())
        .collect::<Vec<_>>();
    let command_feedback_replay_history_badges = command_feedback_replay_fixtures
        .history_entries
        .iter()
        .map(|entry| entry.badge.clone())
        .collect::<Vec<_>>();
    let command_feedback_rejection_replay_fixtures =
        rts_control_group_command_feedback_rejection_replay_fixtures();
    let command_feedback_rejection_replay_step_names = command_feedback_rejection_replay_fixtures
        .rejection_steps
        .iter()
        .map(|step| step.step_name.clone())
        .collect::<Vec<_>>();
    let command_feedback_rejection_replay_preview_stages =
        command_feedback_rejection_replay_fixtures
            .rejection_steps
            .iter()
            .filter_map(|step| step.preview_stage.clone())
            .collect::<Vec<_>>();
    let command_feedback_rejection_replay_input_sources =
        command_feedback_rejection_replay_fixtures
            .rejection_steps
            .iter()
            .map(|step| step.input_source.clone())
            .collect::<Vec<_>>();
    let command_feedback_rejection_replay_visual_stages =
        command_feedback_rejection_replay_fixtures
            .visual_stages
            .iter()
            .map(|stage| stage.stage.clone())
            .collect::<Vec<_>>();
    let command_execution_feedback_kind_samples = vec![
        rts_command_execution_feedback_kind(
            "idle",
            "move:line",
            "stable",
            true,
            "rally",
            true,
            false,
            &["feedback:rally_confirmed@8,4".to_string()],
        ),
        rts_command_execution_feedback_kind(
            "following:player",
            "follow:player",
            "stable",
            false,
            "follow",
            false,
            false,
            &[],
        ),
        rts_command_execution_feedback_kind(
            "attack_move_advancing:forest_creep_camp",
            "attack_move:10,3",
            "stable",
            false,
            "attack_move",
            false,
            false,
            &[],
        ),
        rts_command_execution_feedback_kind(
            "idle",
            "queue",
            "harvesting:gold_vein",
            false,
            "harvest",
            false,
            false,
            &["harvest:gold_vein".to_string()],
        ),
    ]
    .into_iter()
    .map(|kind| kind.unwrap_or("none").to_string())
    .collect::<Vec<_>>();
    let command_execution_harvest_nodes = vec!["gold_vein".to_string()];
    let command_execution_harvest_queue = vec!["harvest:gold_vein->town_hall".to_string()];
    let command_execution_target_label_samples = vec![
        rts_command_execution_target_label(
            "move",
            None,
            "idle",
            "move:line",
            &[],
            &[],
            Some("8,4"),
        ),
        rts_command_execution_target_label(
            "follow",
            None,
            "following:player",
            "follow:player",
            &[],
            &[],
            None,
        ),
        rts_command_execution_target_label(
            "attack",
            Some("arena_creep_attack"),
            "attack_move_advancing:forest_creep_camp",
            "attack_move:10,3",
            &[],
            &[],
            Some("8,4"),
        ),
        rts_command_execution_target_label(
            "harvest",
            None,
            "idle",
            "queue",
            &command_execution_harvest_nodes,
            &command_execution_harvest_queue,
            None,
        ),
    ];
    let command_execution_player_label_samples = vec![
        rts_command_execution_player_label(
            "move",
            &command_execution_target_label_samples[0],
            None,
        ),
        rts_command_execution_player_label(
            "follow",
            &command_execution_target_label_samples[1],
            None,
        ),
        rts_command_execution_player_label(
            "attack",
            &command_execution_target_label_samples[2],
            None,
        ),
        rts_command_execution_player_label(
            "harvest",
            &command_execution_target_label_samples[3],
            Some("town_hall"),
        ),
    ];
    let command_execution_target_tile_samples = vec![
        rts_command_execution_target_tile("move", None, "idle", "move:line", &[], &[], Some("8,4"))
            .expect("move target tile sample"),
        rts_command_execution_target_tile(
            "follow",
            None,
            "following:player",
            "follow:player",
            &[],
            &[],
            None,
        )
        .expect("follow target tile sample"),
        rts_command_execution_target_tile(
            "attack",
            Some("arena_creep_attack"),
            "attack_move_advancing:forest_creep_camp",
            "attack_move:10,3",
            &[],
            &[],
            Some("8,4"),
        )
        .expect("attack target tile sample"),
        rts_command_execution_target_tile(
            "harvest",
            None,
            "idle",
            "queue",
            &command_execution_harvest_nodes,
            &command_execution_harvest_queue,
            None,
        )
        .expect("harvest target tile sample"),
    ];
    let hover_target_preview_kind =
        rts_hover_target_preview_kind("viewport_attack_target").map(str::to_string);
    let hover_cursor_kind =
        rts_cursor_kind_for_hover_preview(true, "command_button", "RTS:ABILITY:focus_fire");
    let hover_cursor_label = rts_cursor_label_for_hover_preview(
        "classic_rts_mouse_command_bar",
        "RTS:ABILITY:focus_fire",
        true,
        hover_cursor_kind,
    );
    let blocked_cursor_kind =
        rts_cursor_kind_for_hover_preview(false, "viewport_move", "RTS:MOVE:4,3:line");
    let blocked_cursor_label = rts_cursor_label_for_hover_preview(
        "classic_rts_mouse_viewport",
        "RTS:MOVE:4,3:line",
        false,
        blocked_cursor_kind,
    );
    let hover_player_label = rts_hover_player_label(
        "classic_rts_mouse_viewport",
        "RTS:ATTACK:square_creep_wander",
        Some("5,4"),
        None,
        "viewport_attack_target",
        true,
        "ok",
    );
    let hover_queue_player_label = rts_hover_player_label(
        "classic_rts_mouse_sidebar",
        "RTS:QUEUE:build:watch_tower@7,4",
        None,
        Some("build:watch_tower@7,4"),
        "sidebar_build_queue",
        true,
        "ok",
    );
    let blocked_hover_player_label = rts_hover_player_label(
        "classic_rts_mouse_viewport",
        "RTS:MOVE:4,3:line",
        Some("4,3"),
        None,
        "viewport_move",
        false,
        "rts_group_selection_required",
    );
    let unit_status_events = vec!["unit_status_portrait:commander".to_string()];
    let unit_status_queue = vec!["unit_status_portrait:".to_string()];
    let unit_status_stage =
        rts_unit_status_portrait_stage(2, &unit_status_events, &unit_status_queue)
            .map(str::to_string);
    let unit_status_unit_id = rts_unit_status_portrait_unit_id(
        unit_status_stage.as_deref().unwrap_or("commander"),
        &["square_worker_carry".to_string()],
        Some("mirror_captain"),
        Some("arena_creep_attack"),
        &["training_hall".to_string()],
    );
    let unit_status_health = rts_unit_status_health_percent("structure", &[], &[76], 41);
    let unit_status_energy = rts_unit_status_energy_percent(&[32]);
    let unit_status_role_badges = rts_unit_status_role_badges("commander")
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>();
    let selection_feedback_stage = rts_selection_command_feedback_stage(
        0,
        &[],
        &["selection_command_feedback:attack_lock".to_string()],
    )
    .map(str::to_string);
    let ability_tooltip_stage = rts_ability_tooltip_telegraph_stage(
        0,
        &["ability_tooltip_telegraph:range_preview".to_string()],
        &[],
    )
    .map(str::to_string);
    let control_group_hotkey_feedback_stage = rts_control_group_hotkey_feedback_stage(
        0,
        &[],
        &["control_group_hotkey_feedback:double_tap_camera".to_string()],
    )
    .map(str::to_string);
    let green = TRNM_RTS_BEVY_RUNTIME_CONTRACT == "trnm_rts_bevy_runtime_adapter_v1"
        && minimap_cell == (134, 175)
        && scroll_camera_stage_summaries.len() == 6
        && scroll_camera_first_focus_tile == (9, 7)
        && scroll_camera_minimap_jump_tile.as_deref() == Some("minimap_cursor_jump")
        && scroll_camera_bounds_clamped
        && camera_minimap_stage_summaries.len() == 6
        && camera_minimap_viewport_rect.x == 19
        && camera_minimap_viewport_rect.y == 8
        && camera_minimap_viewport_rect.width == 33
        && camera_minimap_viewport_rect.height == 19
        && camera_minimap_selection_follow_tile.as_deref() == Some("mirror_captain")
        && camera_minimap_revealed_union.len() == 35
        && camera_minimap_zoom_rect_area == 308
        && path_preview.as_deref() == Some("queue_stack")
        && command_queue_path_preview_fixtures.len() == 6
        && command_queue_path_preview_action_kinds
            == [
                "select-control-group",
                "move",
                "move",
                "attack",
                "queue",
                "queue",
            ]
        && command_queue_path_preview_action_payloads
            == [
                "box:frontline",
                "8,4:line",
                "9,2:rally",
                "arena_creep_attack",
                "build:watch_tower@7,4",
                "cancel:build:0",
            ]
        && command_queue_path_preview_history_entries
            == [
                "command_queue_path_preview:queue_stack",
                "command_queue_path_preview:shift_waypoints",
                "command_queue_path_preview:rally_chain",
                "command_queue_path_preview:attack_focus",
                "command_queue_path_preview:build_reservation",
                "command_queue_path_preview:cancel_repath",
            ]
        && formation_preview_stage.as_deref() == Some("commit_spacing")
        && formation_preview_fixtures.len() == 6
        && formation_preview_action_payloads
            == [
                "box:frontline",
                "8,4:wedge",
                "8,4:line",
                "8,4:wedge",
                "6,5:split",
                "9,2:rally",
            ]
        && formation_preview_history_entries
            == [
                "formation_move_preview:destination_ghost",
                "formation_move_preview:wedge_spacing",
                "formation_move_preview:line_reflow",
                "formation_move_preview:collision_avoidance",
                "formation_move_preview:split_avoidance",
                "formation_move_preview:commit_spacing",
            ]
        && formation_preview_destination_slots == ["8,4", "7,4", "8,5", "9,4"]
        && formation_preview_split_route == ["5,5", "6,4", "6,5", "7,5", "6,6"]
        && recall_formation_fixtures.len() == 4
        && recall_formation_action_payloads == ["28", "1,31:line", "1,31:line", "1,31:line"]
        && recall_formation_history_entries
            == [
                "control_group_recall_formation_preview:recall_focus_hud",
                "control_group_recall_formation_preview:formation_anchor_slots",
                "control_group_recall_formation_preview:queued_valid_members",
                "control_group_recall_formation_preview:filtered_invalid",
            ]
        && recall_formation_slot_tiles == ["1,31", "2,31"]
        && recall_formation_filtered_members
            == [
                "missing:multi0.recall.formation.missing",
                "foreign:map.actor1",
            ]
        && recall_override_fixtures.len() == 4
        && recall_override_action_payloads == ["26", "18,31:line", "27", "20,30:line"]
        && recall_override_history_entries
            == [
                "control_group_recall_override_preview:group_26_recall_focus",
                "control_group_recall_override_preview:group_26_queued_order",
                "control_group_recall_override_preview:group_27_override_cancel",
                "control_group_recall_override_preview:group_27_final_filtered",
            ]
        && recall_override_final_tiles == ["20,30", "22,30"]
        && recall_override_canceled_members
            == [
                "multi0.recall.override.runner",
                "multi0.recall.override.wing",
            ]
        && formation_execution_stage.as_deref() == Some("arrival_lock")
        && formation_execution_fixtures.stages.len() == 6
        && formation_execution_stage_names
            == [
                "slot_claim",
                "path_reservation",
                "stagger_step",
                "crowd_avoidance",
                "blocked_reroute",
                "arrival_lock",
            ]
        && formation_execution_action_payloads
            == [
                "box:frontline",
                "8,4:wedge",
                "8,4:line",
                "6,5:split",
                "8,4:wedge",
                "9,2:rally",
            ]
        && formation_execution_arrival_route == ["6,5", "7,5", "8,5", "9,4", "9,2"]
        && local_obstruction_stage.as_deref() == Some("flow_resume")
        && local_obstruction_fixtures.stages.len() == 5
        && local_obstruction_stage_names
            == [
                "detect_block",
                "hold_queue",
                "side_step",
                "gap_claim",
                "flow_resume",
            ]
        && local_obstruction_action_payloads
            == [
                "8,4:wedge",
                "8,4:line",
                "6,5:split",
                "box:frontline",
                "9,2:rally",
            ]
        && local_obstruction_blocked_tiles == ["7,4", "7,5"]
        && local_obstruction_resume_route == ["6,5", "7,5", "8,5", "9,4", "9,2"]
        && npc_behavior_stage.as_deref() == Some("creep_retreat")
        && combat_impact_stage.as_deref() == Some("damage_tick")
        && locomotion_blend_stage.as_deref() == Some("formation_slide")
        && npc_transition_stage.as_deref() == Some("hit_recover")
        && depth_readability_stage.as_deref() == Some("target_priority")
        && structure_modeling_stage.as_deref() == Some("repair_beam")
        && environment_life_stage.as_deref() == Some("resource_glint")
        && worker_harvest_animation_stage.as_deref() == Some("return_path")
        && production_spawn_animation_stage.as_deref() == Some("supply_flash")
        && action_cadence_attack_marks.len() == 22
        && action_cadence_carry_marks.len() == 8
        && action_cadence_idle_marks.len() == 4
        && action_cadence_creep_windup_offset == -24
        && action_sequence_phase.as_deref() == Some("recovery")
        && action_sequence_windup_marks.len() == 9
        && action_sequence_strike_marks.len() == 12
        && action_sequence_carry_down_marks.len() == 5
        && action_sequence_idle_marks.len() == 6
        && unit_model_depth_guard_marks.len() == 8
        && unit_model_depth_worker_marks.len() == 8
        && unit_model_depth_creep_marks.len() == 8
        && unit_model_depth_creep_role_prop_count == 2
        && unit_model_depth_face_shade_offset == -32
        && command_surface_stage.as_deref() == Some("target_queue")
        && command_grid_hit == Some(0)
        && tile_line.len() == 9
        && tile_line.first().is_some_and(|step| {
            step.step_index == 0 && step.step_count == 8 && step.tile_x == 8 && step.tile_y == 8
        })
        && tile_line.get(4).is_some_and(|step| {
            step.step_index == 4 && step.step_count == 8 && step.tile_x == 10 && step.tile_y == 12
        })
        && tile_line.last().is_some_and(|step| {
            step.step_index == 8 && step.step_count == 8 && step.tile_x == 12 && step.tile_y == 16
        })
        && combat_engagement_tiles == vec!["9,3", "10,3", "10,2", "11,2"]
        && combat_flash_tiles == vec!["6,5", "6,4"]
        && combat_target_tile == (9, 3)
        && combat_target_priority
            == vec![
                "arena_creep_attack",
                "arena_guard_support",
                "arena_worker_support",
            ]
        && combat_projectile_trail == vec!["5,5", "6,5", "7,4", "8,3"]
        && combat_ability_effect_tiles == vec!["10,3", "10,2", "11,2", "9,3"]
        && combat_threat_levels == vec![88, 66, 41]
        && combat_damage_ticks == vec![16, 21, 35]
        && combat_projectile_id == "guard_break_bolt"
        && ai_pressure_wave_units == vec!["lane_scout", "mirror_raider", "siege_runner"]
        && ai_pressure_tiles == vec!["9,3", "8,4", "7,4", "6,5"]
        && ai_pressure_counter_tiles == vec!["5,5", "6,5", "6,4", "7,5"]
        && enemy_pressure_wave_units == vec!["enemy_raider", "enemy_signal_guard", "enemy_sapper"]
        && enemy_pressure_lane_tiles == vec!["10,2", "9,3", "8,4", "7,4", "6,5"]
        && recon_scout_route_tiles == vec!["5,5", "6,4", "7,4", "8,3", "9,2", "10,2"]
        && recon_fog_reveal_tiles
            == vec![
                "7,4", "8,3", "8,2", "9,2", "9,3", "10,2", "10,3", "11,1", "11,2",
            ]
        && recon_enemy_structures
            == vec!["enemy_watch_post", "enemy_barracks", "enemy_resource_vault"]
        && recon_enemy_units == vec!["enemy_scout", "enemy_worker", "enemy_guard"]
        && recon_enemy_structure_tile == (11, 2)
        && recon_enemy_unit_tile == (11, 2)
        && base_assault_path_tiles == vec!["5,5", "6,5", "7,4", "8,4", "9,3", "10,3"]
        && base_assault_targets
            == vec!["enemy_watch_post", "enemy_barracks", "enemy_resource_vault"]
        && aftermath_debris_tiles == vec!["9,3", "10,3", "10,4", "11,3"]
        && aftermath_smoke_tiles == vec!["10,2", "10,3", "11,3"]
        && commander_aura_tiles == vec!["6,5", "7,4", "8,4", "9,3", "10,3"]
        && commander_loot_items
            == vec![
                "barracks_map_cache",
                "field_banner_relic",
                "repair_kit_crate",
            ]
        && expansion_claim_tiles == vec!["8,2", "9,2", "10,2", "9,3", "10,3"]
        && expansion_structure_tile == (8, 3)
        && expansion_workers
            == vec![
                "expansion_worker_alpha",
                "expansion_worker_beta",
                "expansion_worker_gamma",
            ]
        && counterattack_units
            == vec![
                "counter_raider_alpha",
                "counter_raider_beta",
                "counter_sapper",
            ]
        && counterattack_route_tiles == vec!["11,2", "10,2", "9,3", "8,3", "7,4", "9,2"]
        && army_units
            == vec![
                "relay_guard_alpha",
                "relay_guard_beta",
                "wayfinder_scout",
                "field_mender",
            ]
        && army_rally_tiles == vec!["5,5", "6,5", "7,4", "8,4", "8,3"]
        && player_army_unit_tile == (6, 4)
        && central_keep_route_tiles == vec!["12,3", "12,4", "13,4", "13,3", "14,3"]
        && central_keep_tile == (13, 3)
        && boss_guard_units == vec!["keep_warden_alpha", "keep_warden_beta", "ward_sentinel"]
        && player_siege_line_tiles == vec!["11,4", "12,4", "13,4", "12,3"]
        && keep_breach_tiles == vec!["13,3", "13,4", "14,3", "14,4"]
        && guardian_counter_units == vec!["high_warden", "ward_lancer", "last_mirror_guard"]
        && keep_claim_tiles == vec!["12,3", "13,3", "14,3", "13,4"]
        && objective_tiles == vec!["6,5", "6,4", "7,5", "9,2"]
        && creep_camp_tiles == vec!["8,3", "8,2", "9,3", "9,2"]
        && terrain_route_tiles == vec!["5,5", "6,5", "7,4", "8,3"]
        && terrain_choke_tiles == vec!["7,4", "7,3", "8,4"]
        && expansion_tiles == vec!["9,2", "10,2", "10,3"]
        && siege_units == vec!["stonebreak_cart"]
        && siege_push_route_tiles == vec!["9,2", "9,3", "10,3", "10,2", "11,2", "10,3"]
        && siege_breach_tiles == vec!["9,3", "10,3", "10,2", "11,2", "10,3"]
        && enemy_fortification_tile == (10, 3)
        && enemy_repair_units == vec!["repair_adept_alpha", "repair_adept_beta"]
        && enemy_flank_units == vec!["ridge_sentry_left", "ridge_sentry_right", "ridge_sapper"]
        && enemy_flank_tile == (8, 4)
        && player_hold_tiles == vec!["8,3", "9,3", "9,4", "10,3"]
        && inner_lane_tiles == vec!["10,3", "11,2", "11,3", "12,3", "12,4"]
        && inner_gate_tile == (11, 3)
        && signal_lock_tile == (12, 3)
        && inner_defenders == vec!["inner_guard_alpha", "inner_guard_beta", "signal_lancer"]
        && supply_convoy == vec!["convoy_cart", "field_medic", "ammo_runner"]
        && split_squad_tiles == vec!["10,4", "11,4", "12,4", "12,3"]
        && inner_core_tile == (12, 3)
        && restored_zones == vec!["central_keep", "signal_core", "inner_lane", "forest_relay"]
        && rebuild_structures == vec!["signal_core", "inner_latch", "mirror_ward"]
        && garrison_units == vec!["mirror_guard_alpha", "signal_lancer", "field_engineer"]
        && open_world_route_tiles == vec!["13,3", "12,3", "11,3", "10,2", "9,2"]
        && open_world_panels
            == vec![
                "room_panel:league-coliseum",
                "task_panel:task-fixture-first-route",
                "combat_panel:league-coliseum",
                "save_panel:post_rts_restore",
            ]
        && siege_unit_tile == (9, 3)
        && harvest_tile == (3, 3)
        && dropoff_tile == (5, 5)
        && build_site_tiles == vec!["7,4", "7,5", "8,4"]
        && structure_tile == (4, 3)
        && unlock_unit_tile == (7, 5)
        && queue_gold_cost == 210
        && queue_available_gold == 40
        && !queue_affordable
        && queue_build_parts == ("watch_tower".to_string(), "7,4".to_string())
        && queue_production_lane
        && queue_feedback_chip == "feedback:build_placed:watch_tower@7,4"
        && blocked_feedback_chip_visible
        && queue_blocked_feedback_label == "QUEUE LOCK NEED 210G"
        && command_panel_slot_id == "attack"
        && command_panel_build_palette_queue_id == "build:watch_tower@7,4"
        && command_panel_production_slot_queue_id == "build:watch_tower@7,4"
        && command_panel_sidebar_cancel_queue_id.as_deref() == Some("cancel:build:0")
        && command_panel_palette_cancel_queue_id.as_deref() == Some("cancel:active_build")
        && command_panel_sidebar_slot_status_label == "B1 66 R"
        && command_panel_palette_state_label == "ACT"
        && command_panel_sidebar_queue_summary == "P:worker@42% B:watch_tower@66%"
        && command_panel_spawned_unit_id == "worker_3"
        && command_panel_structure_id == "watch_tower"
        && scripted_demo_pauses_queue_tick
        && scripted_demo_stage_from_frame == Some(4)
        && scripted_demo_stage_id == "cancel_refund"
        && scripted_demo_stage_title == "WORKER QUEUED"
        && selection_default_units
            == vec![
                "player",
                "square_guard_patrol",
                "square_worker_carry",
                "square_creep_wander",
            ]
        && selection_same_class_units
            == vec!["player", "square_guard_front", "square_guard_patrol"]
        && selection_guard_tile == Some((7, 5))
        && selection_drag_units
            == vec![
                "player",
                "square_guard_front",
                "square_guard_patrol",
                "square_worker_carry",
                "square_worker_harvest",
            ]
        && selection_drag_rejected_units == vec!["square_creep_wander"]
        && selection_drag_distance_sq == 107_300
        && selection_drag_ready
        && selection_drag_group_id == "drag:4,4->8,5"
        && selection_drag_player_label == "DRAG SELECT 5 UNITS 4,4->8,5"
        && selection_tiles_for_units == vec!["5,4", "4,5"]
        && control_group_hotkey_slot.as_deref() == Some("10")
        && control_group_default_slot_three_units
            == vec!["square_worker_carry", "square_worker_harvest"]
        && control_group_assignment_units == vec!["square_worker_carry", "square_worker_harvest"]
        && control_group_summary_slot_ten.slot == "10"
        && control_group_summary_slot_ten.key_label == "0"
        && control_group_summary_slot_ten.member_count == 2
        && control_group_summary_slot_ten.occupied
        && control_group_summary_slot_ten.active
        && control_group_merged_units == vec!["player", "square_worker_carry"]
        && selection_clear_parts
            == Some((
                "hostile".to_string(),
                Some("square_creep_wander".to_string()),
                "9,4".to_string(),
            ))
        && move_command_parts_sample == vec!["9,2", "attack_move"]
        && line_path_tiles == vec!["6,5", "7,4", "8,3"]
        && focus_fire_units
            == vec![
                "relay_guard_alpha",
                "relay_guard_beta",
                "wayfinder_scout",
                "field_mender",
            ]
        && creep_camp_units == vec!["forest_alpha_creep", "forest_stalker", "forest_shaman"]
        && command_parts_samples
            == vec![
                vec!["claim", "relay_beacon", "9,2"],
                vec!["clear", "forest_creep_camp", "8,3"],
                vec!["mark", "enemy_base", "10,2"],
                vec!["pressure", "counter_wave", "enemy_gate"],
                vec!["upgrade", "signal_blade", "training_hall"],
                vec!["train", "mixed_vanguard", "training_hall"],
                vec!["breach", "enemy_barracks", "10,3"],
                vec!["destroy", "enemy_barracks", "10,3"],
                vec!["level", "mirror_captain", "forest_relay"],
                vec!["claim", "forest_relay", "9,2"],
                vec!["tech", "stonebreak_cart", "relay_outpost"],
            ]
        && selection_command_stamp.kind == "control-group"
        && selection_command_stamp.target_id.as_deref() == Some("5")
        && selection_command_stamp.player_label == "HOTKEY GROUP 5 ASSIGNED 2 UNITS"
        && move_command_stamp.kind == "move"
        && move_command_stamp.tile_id.as_deref() == Some("7,4")
        && move_command_stamp.player_label == "MAP MOVE SENT 7,4"
        && ability_command_stamp.kind == "ability"
        && ability_command_stamp.tile_id.as_deref() == Some("6,5")
        && ability_command_stamp.target_id.as_deref() == Some("arena_creep_attack")
        && ability_command_stamp.player_label == "COMMAND BAR ABILITY SENT FOCUS FIRE"
        && order_queue_replay_action_samples
            .iter()
            .map(|action| vec![action.kind.as_str(), action.payload.as_str()])
            .collect::<Vec<_>>()
            == vec![
                vec!["attack", "arena_creep_attack"],
                vec!["move", "9,2:line"],
                vec!["move", "minimap:rally:5,2"],
                vec!["queue", "train:worker"],
                vec!["select-control-group", "3"],
                vec!["ability", "focus_fire"],
            ]
        && command_feedback_strip_stage.as_deref() == Some("group_27_override")
        && command_feedback_strip_fixture_stage_names
            == vec![
                "group_26_queued",
                "group_27_override",
                "group_28_formation",
                "group_28_filtered",
            ]
        && command_feedback_strip_fixture_action_payloads
            == vec!["18,31:line", "27", "1,31:line", "1,31:line"]
        && command_feedback_strip_fixture_focus_tiles == vec!["18,30", "21,30", "1,30", "1,30"]
        && command_feedback_strip_fixture_filtered_members
            == vec![
                "missing:multi0.recall.formation.missing",
                "foreign:map.actor1",
            ]
        && command_feedback_lifecycle_stage.as_deref() == Some("dimmed")
        && command_feedback_lifecycle_fixture_stage_names == vec!["fresh", "dimmed", "cleared"]
        && command_feedback_lifecycle_fixture_action_payloads
            == vec!["18,31:line", "1,31:line", "28"]
        && command_feedback_lifecycle_fixture_age_ticks == vec![0, 4, 8]
        && command_feedback_lifecycle_fixture_events
            == vec![
                "control_group_command_feedback_lifecycle:fresh",
                "control_group_command_feedback_lifecycle:dimmed",
                "control_group_command_feedback_lifecycle:cleared",
            ]
        && command_feedback_replay_step_names
            == vec![
                "select_group_26",
                "queue_group_26",
                "select_group_27",
                "override_group_27",
                "select_group_28",
                "formation_group_28",
                "bounded_history_after_clear",
            ]
        && command_feedback_replay_preview_stages
            == vec![
                "group_26_queued",
                "group_27_override",
                "group_28_formation",
                "cleared_history_bounded",
            ]
        && command_feedback_replay_fixtures.retained_history_group_ids == vec!["26", "27", "28"]
        && command_feedback_replay_fixtures.pruned_history_group_ids == vec!["25", "24"]
        && command_feedback_replay_history_badges
            == vec!["QUEUE", "CANCEL_FINAL", "FORMATION_FILTER_CLEAR"]
        && command_feedback_rejection_replay_step_names
            == vec![
                "move_without_group_selection",
                "select_group_26_setup",
                "move_invalid_tile_after_selection",
                "attack_without_target",
                "ability_before_attack_target",
                "queue_without_queue_id",
                "queue_unaffordable_build_after_selection",
                "select_without_group_id",
            ]
        && command_feedback_rejection_replay_preview_stages
            == vec![
                "group_selection_required",
                "invalid_tile",
                "attack_target_required",
                "history_preserved_after_rejections",
            ]
        && command_feedback_rejection_replay_fixtures.expected_blocked_reasons
            == vec![
                "rts_group_selection_required",
                "rts_invalid_tile:bad-tile",
                "rts_attack_target_required",
                "rts_attack_required_before_ability",
                "rts_queue_id_required",
                "rts_queue_unaffordable:build:watch_tower@7,4",
                "rts_group_id_required",
            ]
        && command_feedback_rejection_replay_fixtures.expected_input_sources
            == command_feedback_rejection_replay_input_sources
        && command_feedback_rejection_replay_visual_stages
            == vec![
                "group_selection_required",
                "invalid_tile",
                "attack_target_required",
                "history_preserved_after_rejections",
            ]
        && command_feedback_rejection_replay_fixtures.retained_history_group_ids
            == vec!["26", "27", "28"]
        && command_feedback_rejection_replay_fixtures.pruned_history_group_ids == vec!["25", "24"]
        && command_history_visible
        && command_history_prune_visible
        && command_history_fixture_stage_names
            == vec![
                "fresh_history_appended",
                "dimmed_history_retained",
                "cleared_history_retained",
            ]
        && command_history_fixture_lifecycle_stages == vec!["fresh", "dimmed", "cleared"]
        && command_history_fixtures.retained_history_group_ids == vec!["26", "27", "28"]
        && command_history_prune_fixture_stage_names
            == vec![
                "overflow_input_pruned",
                "recent_three_retained",
                "cleared_history_bounded",
            ]
        && command_history_prune_fixtures.pruned_history_group_ids == vec!["25", "24"]
        && command_history_prune_fixture_prune_reasons
            == vec!["recent_three_capacity", "recent_three_capacity"]
        && command_execution_feedback_kind_samples == vec!["move", "follow", "attack", "harvest"]
        && command_execution_target_label_samples
            == vec!["8,4", "player", "arena_creep_attack", "gold_vein"]
        && command_execution_player_label_samples
            == vec![
                "MOVE EXECUTING 8,4",
                "FOLLOWING PLAYER",
                "ATTACK FOCUS ARENA CREEP ATTACK",
                "HARVEST GOLD VEIN TO TOWN HALL",
            ]
        && command_execution_target_tile_samples == vec![(8, 4), (5, 4), (6, 5), (3, 3)]
        && hover_target_preview_kind.as_deref() == Some("attack")
        && hover_cursor_kind == "ability"
        && hover_cursor_label == "COMMAND BAR CURSOR ABILITY READY"
        && blocked_cursor_kind == "blocked"
        && blocked_cursor_label == "MAP CURSOR BLOCKED LOCK"
        && hover_player_label == "MAP ATTACK READY SQUARE CREEP WANDER"
        && hover_queue_player_label == "SIDEBAR QUEUE READY WATCH TOWER 7,4 210G"
        && blocked_hover_player_label == "MAP MOVE LOCK SELECT UNITS"
        && unit_status_stage.as_deref() == Some("commander")
        && unit_status_unit_id == "mirror_captain"
        && unit_status_health == 76
        && unit_status_energy == 68
        && unit_status_role_badges == vec!["AUR", "LVL", "CMD"]
        && selection_feedback_stage.as_deref() == Some("attack_lock")
        && ability_tooltip_stage.as_deref() == Some("range_preview")
        && control_group_hotkey_feedback_stage.as_deref() == Some("double_tap_camera");

    RtsBevyRuntimeAdapterEvidence {
        contract_version: TRNM_RTS_EVIDENCE_BEVY_RUNTIME_ADAPTER_CONTRACT.to_string(),
        runtime_contract: TRNM_RTS_BEVY_RUNTIME_CONTRACT.to_string(),
        green,
        minimap_cell_sample: RtsEvidencePoint {
            x: minimap_cell.0,
            y: minimap_cell.1,
        },
        scroll_camera_stage_count_sample: scroll_camera_stage_summaries.len(),
        scroll_camera_first_focus_tile_sample: RtsEvidencePoint {
            x: scroll_camera_first_focus_tile.0,
            y: scroll_camera_first_focus_tile.1,
        },
        scroll_camera_minimap_jump_tile_sample: scroll_camera_minimap_jump_tile,
        scroll_camera_bounds_clamped_sample: scroll_camera_bounds_clamped,
        camera_minimap_stage_count_sample: camera_minimap_stage_summaries.len(),
        camera_minimap_viewport_rect_sample: camera_minimap_viewport_rect,
        camera_minimap_selection_follow_tile_sample: camera_minimap_selection_follow_tile,
        camera_minimap_revealed_union_count_sample: camera_minimap_revealed_union.len(),
        camera_minimap_zoom_rect_area_sample: camera_minimap_zoom_rect_area,
        path_preview_sample: path_preview,
        command_queue_path_preview_stage_count_sample: command_queue_path_preview_fixtures.len(),
        command_queue_path_preview_action_kinds_sample: command_queue_path_preview_action_kinds,
        command_queue_path_preview_action_payloads_sample: command_queue_path_preview_action_payloads,
        command_queue_path_preview_history_entries_sample:
            command_queue_path_preview_history_entries,
        formation_move_preview_stage_sample: formation_preview_stage,
        formation_move_preview_stage_count_sample: formation_preview_fixtures.len(),
        formation_move_preview_action_payloads_sample: formation_preview_action_payloads,
        formation_move_preview_history_entries_sample: formation_preview_history_entries,
        formation_move_preview_destination_slots_sample: formation_preview_destination_slots,
        formation_move_preview_split_route_sample: formation_preview_split_route,
        control_group_recall_formation_preview_stage_count_sample: recall_formation_fixtures.len(),
        control_group_recall_formation_preview_action_payloads_sample:
            recall_formation_action_payloads,
        control_group_recall_formation_preview_history_entries_sample:
            recall_formation_history_entries,
        control_group_recall_formation_preview_slot_tiles_sample: recall_formation_slot_tiles,
        control_group_recall_formation_preview_filtered_members_sample:
            recall_formation_filtered_members,
        control_group_recall_override_preview_stage_count_sample: recall_override_fixtures.len(),
        control_group_recall_override_preview_action_payloads_sample:
            recall_override_action_payloads,
        control_group_recall_override_preview_history_entries_sample:
            recall_override_history_entries,
        control_group_recall_override_preview_final_tiles_sample: recall_override_final_tiles,
        control_group_recall_override_preview_canceled_members_sample:
            recall_override_canceled_members,
        formation_move_execution_stage_sample: formation_execution_stage,
        formation_move_execution_stage_names_sample: formation_execution_stage_names,
        formation_move_execution_action_payloads_sample: formation_execution_action_payloads,
        formation_move_execution_arrival_route_sample: formation_execution_arrival_route,
        local_obstruction_recovery_stage_sample: local_obstruction_stage,
        local_obstruction_recovery_stage_names_sample: local_obstruction_stage_names,
        local_obstruction_recovery_action_payloads_sample: local_obstruction_action_payloads,
        local_obstruction_recovery_blocked_tiles_sample: local_obstruction_blocked_tiles,
        local_obstruction_recovery_resume_route_sample: local_obstruction_resume_route,
        npc_behavior_stage_sample: npc_behavior_stage,
        combat_impact_stage_sample: combat_impact_stage,
        locomotion_blend_stage_sample: locomotion_blend_stage,
        npc_transition_stage_sample: npc_transition_stage,
        depth_readability_stage_sample: depth_readability_stage,
        structure_modeling_stage_sample: structure_modeling_stage,
        environment_life_stage_sample: environment_life_stage,
        worker_harvest_animation_stage_sample: worker_harvest_animation_stage,
        production_spawn_animation_stage_sample: production_spawn_animation_stage,
        action_cadence_attack_mark_count_sample: action_cadence_attack_marks.len(),
        action_cadence_carry_mark_count_sample: action_cadence_carry_marks.len(),
        action_cadence_idle_mark_count_sample: action_cadence_idle_marks.len(),
        action_cadence_creep_windup_offset_sample: action_cadence_creep_windup_offset,
        action_sequence_phase_sample: action_sequence_phase,
        action_sequence_windup_mark_count_sample: action_sequence_windup_marks.len(),
        action_sequence_strike_mark_count_sample: action_sequence_strike_marks.len(),
        action_sequence_carry_down_mark_count_sample: action_sequence_carry_down_marks.len(),
        action_sequence_idle_mark_count_sample: action_sequence_idle_marks.len(),
        unit_model_depth_guard_mark_count_sample: unit_model_depth_guard_marks.len(),
        unit_model_depth_worker_mark_count_sample: unit_model_depth_worker_marks.len(),
        unit_model_depth_creep_mark_count_sample: unit_model_depth_creep_marks.len(),
        unit_model_depth_creep_role_prop_count_sample: unit_model_depth_creep_role_prop_count,
        unit_model_depth_face_shade_offset_sample: unit_model_depth_face_shade_offset,
        command_surface_stage_sample: command_surface_stage,
        command_grid_hit_sample: command_grid_hit,
        tile_line_sample: tile_line,
        combat_engagement_tiles_sample: combat_engagement_tiles,
        combat_flash_tiles_sample: combat_flash_tiles,
        combat_target_tile_sample: RtsEvidencePoint {
            x: combat_target_tile.0,
            y: combat_target_tile.1,
        },
        combat_target_priority_sample: combat_target_priority,
        combat_projectile_trail_sample: combat_projectile_trail,
        combat_ability_effect_tiles_sample: combat_ability_effect_tiles,
        combat_threat_levels_sample: combat_threat_levels,
        combat_damage_ticks_sample: combat_damage_ticks,
        combat_projectile_id_sample: combat_projectile_id.to_string(),
        ai_pressure_wave_units_sample: ai_pressure_wave_units,
        ai_pressure_tiles_sample: ai_pressure_tiles,
        ai_pressure_counter_tiles_sample: ai_pressure_counter_tiles,
        enemy_pressure_wave_units_sample: enemy_pressure_wave_units,
        enemy_pressure_lane_tiles_sample: enemy_pressure_lane_tiles,
        recon_scout_route_tiles_sample: recon_scout_route_tiles,
        recon_fog_reveal_tiles_sample: recon_fog_reveal_tiles,
        recon_enemy_structures_sample: recon_enemy_structures,
        recon_enemy_units_sample: recon_enemy_units,
        recon_enemy_structure_tile_sample: RtsEvidencePoint {
            x: recon_enemy_structure_tile.0,
            y: recon_enemy_structure_tile.1,
        },
        recon_enemy_unit_tile_sample: RtsEvidencePoint {
            x: recon_enemy_unit_tile.0,
            y: recon_enemy_unit_tile.1,
        },
        base_assault_path_tiles_sample: base_assault_path_tiles,
        base_assault_targets_sample: base_assault_targets,
        aftermath_debris_tiles_sample: aftermath_debris_tiles,
        aftermath_smoke_tiles_sample: aftermath_smoke_tiles,
        commander_aura_tiles_sample: commander_aura_tiles,
        commander_loot_items_sample: commander_loot_items,
        expansion_claim_tiles_sample: expansion_claim_tiles,
        expansion_structure_tile_sample: RtsEvidencePoint {
            x: expansion_structure_tile.0,
            y: expansion_structure_tile.1,
        },
        expansion_workers_sample: expansion_workers,
        counterattack_units_sample: counterattack_units,
        counterattack_route_tiles_sample: counterattack_route_tiles,
        army_units_sample: army_units,
        army_rally_tiles_sample: army_rally_tiles,
        player_army_unit_tile_sample: RtsEvidencePoint {
            x: player_army_unit_tile.0,
            y: player_army_unit_tile.1,
        },
        central_keep_route_tiles_sample: central_keep_route_tiles,
        central_keep_tile_sample: RtsEvidencePoint {
            x: central_keep_tile.0,
            y: central_keep_tile.1,
        },
        boss_guard_units_sample: boss_guard_units,
        player_siege_line_tiles_sample: player_siege_line_tiles,
        keep_breach_tiles_sample: keep_breach_tiles,
        guardian_counter_units_sample: guardian_counter_units,
        keep_claim_tiles_sample: keep_claim_tiles,
        objective_tiles_sample: objective_tiles,
        creep_camp_tiles_sample: creep_camp_tiles,
        terrain_route_tiles_sample: terrain_route_tiles,
        terrain_choke_tiles_sample: terrain_choke_tiles,
        expansion_tiles_sample: expansion_tiles,
        siege_units_sample: siege_units,
        siege_push_route_tiles_sample: siege_push_route_tiles,
        siege_breach_tiles_sample: siege_breach_tiles,
        enemy_fortification_tile_sample: RtsEvidencePoint {
            x: enemy_fortification_tile.0,
            y: enemy_fortification_tile.1,
        },
        enemy_repair_units_sample: enemy_repair_units,
        enemy_flank_units_sample: enemy_flank_units,
        enemy_flank_tile_sample: RtsEvidencePoint {
            x: enemy_flank_tile.0,
            y: enemy_flank_tile.1,
        },
        player_hold_tiles_sample: player_hold_tiles,
        inner_lane_tiles_sample: inner_lane_tiles,
        inner_gate_tile_sample: RtsEvidencePoint {
            x: inner_gate_tile.0,
            y: inner_gate_tile.1,
        },
        signal_lock_tile_sample: RtsEvidencePoint {
            x: signal_lock_tile.0,
            y: signal_lock_tile.1,
        },
        inner_defenders_sample: inner_defenders,
        supply_convoy_sample: supply_convoy,
        split_squad_tiles_sample: split_squad_tiles,
        inner_core_tile_sample: RtsEvidencePoint {
            x: inner_core_tile.0,
            y: inner_core_tile.1,
        },
        restored_zones_sample: restored_zones,
        rebuild_structures_sample: rebuild_structures,
        garrison_units_sample: garrison_units,
        open_world_route_tiles_sample: open_world_route_tiles,
        open_world_panels_sample: open_world_panels,
        siege_unit_tile_sample: RtsEvidencePoint {
            x: siege_unit_tile.0,
            y: siege_unit_tile.1,
        },
        harvest_tile_sample: RtsEvidencePoint {
            x: harvest_tile.0,
            y: harvest_tile.1,
        },
        dropoff_tile_sample: RtsEvidencePoint {
            x: dropoff_tile.0,
            y: dropoff_tile.1,
        },
        build_site_tiles_sample: build_site_tiles,
        structure_tile_sample: RtsEvidencePoint {
            x: structure_tile.0,
            y: structure_tile.1,
        },
        unlock_unit_tile_sample: RtsEvidencePoint {
            x: unlock_unit_tile.0,
            y: unlock_unit_tile.1,
        },
        queue_gold_cost_sample: queue_gold_cost,
        queue_available_gold_sample: queue_available_gold,
        queue_affordable_sample: queue_affordable,
        queue_build_parts_sample: vec![queue_build_parts.0, queue_build_parts.1],
        queue_production_lane_sample: queue_production_lane,
        queue_feedback_chip_sample: queue_feedback_chip,
        blocked_feedback_chip_visible_sample: blocked_feedback_chip_visible,
        queue_blocked_feedback_label_sample: queue_blocked_feedback_label,
        command_panel_slot_id_sample: command_panel_slot_id,
        command_panel_build_palette_queue_id_sample: command_panel_build_palette_queue_id,
        command_panel_production_slot_queue_id_sample: command_panel_production_slot_queue_id,
        command_panel_sidebar_cancel_queue_id_sample: command_panel_sidebar_cancel_queue_id,
        command_panel_palette_cancel_queue_id_sample: command_panel_palette_cancel_queue_id,
        command_panel_sidebar_slot_status_label_sample: command_panel_sidebar_slot_status_label,
        command_panel_palette_state_label_sample: command_panel_palette_state_label,
        command_panel_sidebar_queue_summary_sample: command_panel_sidebar_queue_summary,
        command_panel_spawned_unit_id_sample: command_panel_spawned_unit_id,
        command_panel_structure_id_sample: command_panel_structure_id,
        scripted_demo_pauses_queue_tick_sample: scripted_demo_pauses_queue_tick,
        scripted_demo_stage_from_frame_sample: scripted_demo_stage_from_frame,
        scripted_demo_stage_id_sample: scripted_demo_stage_id.to_string(),
        scripted_demo_stage_title_sample: scripted_demo_stage_title.to_string(),
        selection_default_units_sample: selection_default_units,
        selection_same_class_units_sample: selection_same_class_units,
        selection_guard_tile_sample: selection_guard_tile.map(|tile| RtsEvidencePoint {
            x: tile.0,
            y: tile.1,
        }),
        selection_drag_units_sample: selection_drag_units,
        selection_drag_rejected_units_sample: selection_drag_rejected_units,
        selection_drag_distance_sq_sample: selection_drag_distance_sq,
        selection_drag_ready_sample: selection_drag_ready,
        selection_drag_group_id_sample: selection_drag_group_id,
        selection_drag_player_label_sample: selection_drag_player_label,
        selection_tiles_for_units_sample: selection_tiles_for_units,
        control_group_hotkey_slot_sample: control_group_hotkey_slot,
        control_group_default_slot_three_units_sample: control_group_default_slot_three_units,
        control_group_assignment_units_sample: control_group_assignment_units,
        control_group_summary_slot_ten_sample: control_group_summary_slot_ten,
        control_group_merged_units_sample: control_group_merged_units,
        selection_clear_parts_sample: selection_clear_parts,
        move_command_parts_sample,
        line_path_tiles_sample: line_path_tiles,
        focus_fire_units_sample: focus_fire_units,
        creep_camp_units_sample: creep_camp_units,
        command_parts_samples,
        selection_command_stamp_sample: selection_command_stamp,
        move_command_stamp_sample: move_command_stamp,
        ability_command_stamp_sample: ability_command_stamp,
        order_queue_replay_action_samples,
        command_feedback_strip_stage_sample: command_feedback_strip_stage,
        command_feedback_strip_fixture_stage_names_sample: command_feedback_strip_fixture_stage_names,
        command_feedback_strip_fixture_action_payloads_sample:
            command_feedback_strip_fixture_action_payloads,
        command_feedback_strip_fixture_focus_tiles_sample: command_feedback_strip_fixture_focus_tiles,
        command_feedback_strip_fixture_filtered_members_sample:
            command_feedback_strip_fixture_filtered_members,
        command_feedback_lifecycle_stage_sample: command_feedback_lifecycle_stage,
        command_feedback_lifecycle_fixture_stage_names_sample:
            command_feedback_lifecycle_fixture_stage_names,
        command_feedback_lifecycle_fixture_action_payloads_sample:
            command_feedback_lifecycle_fixture_action_payloads,
        command_feedback_lifecycle_fixture_age_ticks_sample:
            command_feedback_lifecycle_fixture_age_ticks,
        command_feedback_lifecycle_fixture_events_sample:
            command_feedback_lifecycle_fixture_events,
        command_feedback_replay_step_names_sample: command_feedback_replay_step_names,
        command_feedback_replay_preview_stages_sample: command_feedback_replay_preview_stages,
        command_feedback_replay_retained_group_ids_sample: command_feedback_replay_fixtures
            .retained_history_group_ids,
        command_feedback_replay_pruned_group_ids_sample: command_feedback_replay_fixtures
            .pruned_history_group_ids,
        command_feedback_replay_history_badges_sample: command_feedback_replay_history_badges,
        command_feedback_rejection_replay_step_names_sample:
            command_feedback_rejection_replay_step_names,
        command_feedback_rejection_replay_preview_stages_sample:
            command_feedback_rejection_replay_preview_stages,
        command_feedback_rejection_replay_input_sources_sample:
            command_feedback_rejection_replay_input_sources,
        command_feedback_rejection_replay_blocked_reasons_sample:
            command_feedback_rejection_replay_fixtures.expected_blocked_reasons,
        command_feedback_rejection_replay_visual_stages_sample:
            command_feedback_rejection_replay_visual_stages,
        command_feedback_rejection_replay_retained_group_ids_sample:
            command_feedback_rejection_replay_fixtures.retained_history_group_ids,
        command_feedback_rejection_replay_pruned_group_ids_sample:
            command_feedback_rejection_replay_fixtures.pruned_history_group_ids,
        command_history_visible_sample: command_history_visible,
        command_history_prune_visible_sample: command_history_prune_visible,
        command_history_fixture_stage_names_sample: command_history_fixture_stage_names,
        command_history_fixture_lifecycle_stages_sample: command_history_fixture_lifecycle_stages,
        command_history_fixture_group_ids_sample: command_history_fixtures.retained_history_group_ids,
        command_history_prune_fixture_stage_names_sample: command_history_prune_fixture_stage_names,
        command_history_prune_fixture_pruned_group_ids_sample:
            command_history_prune_fixtures.pruned_history_group_ids,
        command_history_prune_fixture_prune_reasons_sample:
            command_history_prune_fixture_prune_reasons,
        command_execution_feedback_kind_samples,
        command_execution_target_label_samples,
        command_execution_player_label_samples,
        command_execution_target_tile_samples: command_execution_target_tile_samples
            .into_iter()
            .map(|tile| RtsEvidencePoint {
                x: tile.0,
                y: tile.1,
            })
            .collect(),
        hover_target_preview_kind_sample: hover_target_preview_kind,
        hover_cursor_kind_sample: hover_cursor_kind.to_string(),
        hover_cursor_label_sample: hover_cursor_label,
        blocked_cursor_kind_sample: blocked_cursor_kind.to_string(),
        blocked_cursor_label_sample: blocked_cursor_label,
        hover_player_label_sample: hover_player_label,
        hover_queue_player_label_sample: hover_queue_player_label,
        blocked_hover_player_label_sample: blocked_hover_player_label,
        unit_status_stage_sample: unit_status_stage,
        unit_status_unit_id_sample: unit_status_unit_id,
        unit_status_health_sample: unit_status_health,
        unit_status_energy_sample: unit_status_energy,
        unit_status_role_badges_sample: unit_status_role_badges,
        selection_feedback_stage_sample: selection_feedback_stage,
        ability_tooltip_stage_sample: ability_tooltip_stage,
        control_group_hotkey_feedback_stage_sample: control_group_hotkey_feedback_stage,
        source_of_truth: "The RTS evidence crate verifies the Bevy-free runtime adapter contract using deterministic First Contact minimap, scrollable camera/minimap sync, path preview, formation move preview/execution, local obstruction recovery, scene stage semantics, structure/environment stage semantics, harvest/production animation stage semantics, action cadence marks, action sequence phase/marks, unit-model depth marks, command-surface stage, command-grid, tile-line raster, combat-target, ability-effect, AI-pressure, recon-intel, base-assault, aftermath, commander-progression, expansion-counterattack, army-production/rally, siege breach counterplay, inner-lane breakthrough, central-keep, restoration/open-world handoff, economy/tech placement, queue economy, blocked-feedback chip visibility, scripted-demo timeline, selection roster, control-group roster, command parsing, command stamp semantics, order-queue replay actions, command feedback lifecycle/history/execution target labels and tiles, hover/cursor affordance, overlay stage/portrait semantics, objective, terrain-route, and siege-route samples before trnm-world-bevy includes the proof in release-review evidence.".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn campaign_ui_continuity_review_preserves_handoff_gates() {
        let handoff: Value = serde_json::from_str(
            r#"{
                "contract_version": "trillionnium_world_bevy_classic_rts_campaign_handoff_v1",
                "green": true,
                "write_gate": true,
                "preview_width": 1920,
                "preview_height": 1080,
                "capture_frame_count": 16,
                "final_current_room_id": "league-coliseum",
                "final_map_scene": "arena_outdoor",
                "final_route_director_task_id": "task-fixture-first-route",
                "final_route_director_next_room_id": null,
                "final_open_world_handoff_state": "resumed:league-coliseum",
                "final_contextual_primary_action_label": "COMBAT:attack",
                "final_contextual_action_labels": ["COMBAT:attack"],
                "final_active_task_ids": ["task-fixture-first-route"],
                "final_objective_status": "open_world_after_action_ready",
                "final_route_director_history": [
                    "rts_open_world_after_action:league-coliseum:arrived"
                ],
                "restored_current_room_id": "league-coliseum",
                "restored_map_scene": "arena_outdoor",
                "restored_open_world_handoff_state": "resumed:league-coliseum",
                "restored_route_director_task_id": "task-fixture-first-route",
                "restored_route_director_next_room_id": null,
                "restored_contextual_action_labels": ["COMBAT:attack"],
                "restored_active_task_ids": ["task-fixture-first-route"],
                "milestones": {
                    "victory": true,
                    "expansion": true,
                    "open_world": true
                },
                "non_background_pixels": 500001,
                "victory_pixel_count": 21,
                "expansion_pixel_count": 61,
                "breach_pixel_count": 41,
                "keep_pixel_count": 41,
                "restoration_pixel_count": 21,
                "open_world_pixel_count": 61,
                "live_campaign_input_gate": true,
                "early_campaign_gate": true,
                "mid_campaign_gate": true,
                "end_campaign_gate": true,
                "open_world_resume_gate": true,
                "snapshot_round_trip_gate": true,
                "cex_runtime_player_client_allowed": false,
                "wgpu_required": false,
                "player_first_campaign_handoff_screen_gate": true,
                "runtime_screen_mode": "player_runtime_campaign_handoff_screen",
                "evidence_board_only": false,
                "campaign_handoff_pixel_counts": {
                    "player_first_campaign_view_non_background": 600001,
                    "player_first_campaign_view_frame": 10001,
                    "player_first_campaign_status_strip": 8001,
                    "player_first_campaign_route_rail": 100001
                }
            }"#,
        )
        .expect("campaign UI continuity handoff fixture parses");

        let review = rts_campaign_ui_continuity_review(&handoff);

        assert_eq!(
            review.contract_version,
            TRNM_RTS_EVIDENCE_CAMPAIGN_UI_CONTINUITY_REVIEW_CONTRACT
        );
        assert!(review.green);
        assert!(review.handoff_green_gate);
        assert!(review.preview_resolution_gate);
        assert!(review.live_input_gate);
        assert!(review.milestone_gate);
        assert!(review.map_ui_state_gate);
        assert!(review.restored_ui_state_gate);
        assert!(review.persistence_gate);
        assert!(review.render_readability_gate);
        assert!(review.native_client_boundary_gate);
        assert!(review.player_first_campaign_continuity_screen_gate);
        assert_eq!(
            review.final_current_room_id.as_deref(),
            Some("league-coliseum")
        );
        assert_eq!(
            review.restored_open_world_handoff_state.as_deref(),
            Some("resumed:league-coliseum")
        );
        assert!(review
            .source_of_truth
            .contains("player-first route-resume screen"));
    }

    #[test]
    fn session_state_continuity_review_preserves_resume_chain_gates() {
        let input: Value = serde_json::from_str(
            r#"{
                "preview_width": 1600,
                "preview_height": 900,
                "preview_format": "ppm_p3_rgb",
                "write_gate": true,
                "preview_file_ready": true,
                "state_continuity_surface_names": [
                    "MATCH SETUP SNAPSHOT",
                    "SESSION SLOT WRITE",
                    "LOAD RESUME LOCK",
                    "CONTINUE UNLOCK",
                    "IN-MATCH HUD RESTORE",
                    "OUTCOME REWARD STATE",
                    "OPEN-WORLD RESUME",
                    "RECOVERY UI GUARD"
                ],
                "resume_chain": [
                    "match_setup_saved",
                    "slot_a_written",
                    "load_resume_locked",
                    "continue_unlocked",
                    "in_match_hud_restored",
                    "campaign_outcome_saved",
                    "open_world_resumed"
                ],
                "state_continuity_pixel_counts": {
                    "non_background": 300001,
                    "board": 100001,
                    "match_setup_snapshot": 2001,
                    "session_slot_write": 2001,
                    "load_resume_lock": 2001,
                    "continue_unlock": 2001,
                    "in_match_hud_restore": 2001,
                    "outcome_reward_state": 2001,
                    "open_world_resume": 2001,
                    "recovery_ui_guard": 2001,
                    "highlight": 1001,
                    "player_first_resume_view_non_background": 250001,
                    "player_first_resume_view_frame": 8001,
                    "player_first_resume_status_strip": 10001,
                    "player_first_resume_stage_rail": 70001
                },
                "source_preview_ready": {
                    "shell_meta_ui_replication": true,
                    "match_setup_ui_replication": true,
                    "in_match_hud_state_replication": true,
                    "campaign_ui_continuity": true
                },
                "native_client_boundary": {
                    "cex_runtime_player_client_allowed": false,
                    "wgpu_required": false
                },
                "sources": {
                    "shell_meta_ui_replication": {
                        "contract_version": "trillionnium_world_bevy_classic_rts_shell_meta_ui_replication_v1",
                        "green": true,
                        "runtime_screen_gate": true,
                        "evidence_board_only": false,
                        "session_slot_confirm_gate": true,
                        "session_load_resume_gate": true,
                        "session_recovery_gate": true,
                        "no_external_boundary_gate": true,
                        "shell_meta_surface_count": 12,
                        "android_s5_real_device_claimed": false,
                        "public_launch_ready": false
                    },
                    "session_slot_confirm": {
                        "contract_version": "trillionnium_world_bevy_session_slot_confirm_v1",
                        "green": true,
                        "save_selected_gate": true,
                        "confirm_overwrite_gate": true,
                        "load_selected_restore_gate": true,
                        "continue_after_load_gate": true,
                        "slot_file_gate": true,
                        "confirmed_slot_a_bytes": 513,
                        "android_s5_real_device_claimed": false
                    },
                    "session_load_resume": {
                        "contract_version": "trillionnium_world_bevy_session_load_resume_v1",
                        "green": true,
                        "save_selected_gate": true,
                        "load_resume_gate": true,
                        "locked_input_gate": true,
                        "continue_gate": true,
                        "final_hud_gate": true,
                        "slot_a_bytes": 1024,
                        "final_runtime": {
                            "objective_status": "first_playable_loop_complete"
                        },
                        "android_s5_real_device_claimed": false
                    },
                    "session_recovery_ui": {
                        "contract_version": "trillionnium_world_bevy_session_recovery_ui_v1",
                        "green": true,
                        "recovered_status_gate": true,
                        "continued_summary_gate": true,
                        "guard_status_gate": true,
                        "android_s5_real_device_claimed": false
                    },
                    "match_setup_ui_replication": {
                        "contract_version": "trillionnium_world_bevy_classic_rts_match_setup_ui_replication_v1",
                        "green": true,
                        "match_setup_ui_replication_gate": true,
                        "runtime_screen_gate": true,
                        "evidence_board_only": false,
                        "shell_meta_gate": true,
                        "faction_gate": true,
                        "no_external_boundary_gate": true,
                        "runtime_screen_mode": "player_runtime_match_setup_screen",
                        "source_headline": {
                            "map_id": "first_contact_basin"
                        },
                        "android_s5_real_device_claimed": false,
                        "public_launch_ready": false
                    },
                    "in_match_hud_state_replication": {
                        "contract_version": "trillionnium_world_bevy_classic_rts_in_match_hud_state_replication_v1",
                        "green": true,
                        "in_match_hud_state_replication_gate": true,
                        "runtime_screen_gate": true,
                        "evidence_board_only": false,
                        "selection_gate": true,
                        "command_gate": true,
                        "production_gate": true,
                        "native_client_boundary_gate": true,
                        "hud_surface_count": 8,
                        "army_supply_used": 9,
                        "command_queue": [
                            "move:16,9",
                            "train:trnm.worker",
                            "build:trnm.flux.relay",
                            "attack:trnm.flux.beacon"
                        ],
                        "android_s5_real_device_claimed": false,
                        "public_launch_ready": false,
                        "screen_for_screen_openra_ui_claimed": false,
                        "openra_engine_port_claimed": false,
                        "warcraft_iii_asset_copied": false,
                        "openra_asset_copied": false,
                        "third_party_asset_copied": false
                    },
                    "campaign_outcome_ui_readiness": {
                        "contract_version": "trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness_v1",
                        "green": true,
                        "first_minute_gate": true,
                        "objective_victory_gate": true,
                        "base_assault_gate": true,
                        "battle_aftermath_gate": true,
                        "open_world_return_gate": true,
                        "runtime_screen_gate": true,
                        "evidence_board_only": false,
                        "preview_gate": true,
                        "open_world_summary": {
                            "final_open_world_handoff_state": "resumed:league-coliseum"
                        },
                        "android_s5_real_device_claimed": false,
                        "public_launch_ready": false
                    },
                    "campaign_ui_continuity": {
                        "contract_version": "trillionnium_world_bevy_classic_rts_campaign_ui_continuity_v1",
                        "green": true,
                        "persistence_gate": true,
                        "restored_ui_state_gate": true,
                        "map_ui_state_gate": true,
                        "native_client_boundary_gate": true,
                        "restored_current_room_id": "league-coliseum",
                        "restored_open_world_handoff_state": "resumed:league-coliseum"
                    }
                }
            }"#,
        )
        .expect("session-state continuity review fixture parses");

        let review = rts_session_state_continuity_review(&input);

        assert_eq!(
            review.contract_version,
            TRNM_RTS_EVIDENCE_SESSION_STATE_CONTINUITY_REVIEW_CONTRACT
        );
        assert!(review.green);
        assert!(review.shell_meta_gate);
        assert!(review.session_slot_confirm_gate);
        assert!(review.session_load_resume_gate);
        assert!(review.session_recovery_gate);
        assert!(review.match_setup_gate);
        assert!(review.hud_restore_gate);
        assert!(review.campaign_outcome_gate);
        assert!(review.campaign_continuity_gate);
        assert!(review.surface_chain_gate);
        assert!(review.state_continuity_chain_gate);
        assert!(review.native_client_boundary_gate);
        assert!(review.preview_gate);
        assert!(review.player_first_session_resume_screen_gate);
        assert!(review.source_preview_gate);
        assert!(review.runtime_screen_gate);
        assert!(review.session_state_continuity_gate);
        assert_eq!(review.load_resume_slot_a_bytes, 1024);
        assert_eq!(
            review.load_resume_final_objective_status.as_deref(),
            Some("first_playable_loop_complete")
        );
        assert_eq!(
            review.campaign_outcome_open_world_state.as_deref(),
            Some("resumed:league-coliseum")
        );
        assert!(review
            .source_of_truth
            .contains("player-first session resume screen"));
    }

    #[test]
    fn first_contact_runtime_adapter_evidence_is_green() {
        let evidence = first_contact_bevy_runtime_adapter_evidence();

        assert_eq!(
            evidence.contract_version,
            TRNM_RTS_EVIDENCE_BEVY_RUNTIME_ADAPTER_CONTRACT
        );
        assert_eq!(evidence.runtime_contract, TRNM_RTS_BEVY_RUNTIME_CONTRACT);
        assert!(evidence.green);
        assert_eq!(
            evidence.minimap_cell_sample,
            RtsEvidencePoint { x: 134, y: 175 }
        );
        assert_eq!(evidence.scroll_camera_stage_count_sample, 6);
        assert_eq!(
            evidence.scroll_camera_first_focus_tile_sample,
            RtsEvidencePoint { x: 9, y: 7 }
        );
        assert_eq!(
            evidence.scroll_camera_minimap_jump_tile_sample.as_deref(),
            Some("minimap_cursor_jump")
        );
        assert!(evidence.scroll_camera_bounds_clamped_sample);
        assert_eq!(evidence.camera_minimap_stage_count_sample, 6);
        assert_eq!(
            evidence.camera_minimap_viewport_rect_sample,
            RtsCameraMinimapViewportRect {
                x: 19,
                y: 8,
                width: 33,
                height: 19,
            }
        );
        assert_eq!(
            evidence
                .camera_minimap_selection_follow_tile_sample
                .as_deref(),
            Some("mirror_captain")
        );
        assert_eq!(evidence.camera_minimap_revealed_union_count_sample, 35);
        assert_eq!(evidence.camera_minimap_zoom_rect_area_sample, 308);
        assert_eq!(evidence.path_preview_sample.as_deref(), Some("queue_stack"));
        assert_eq!(evidence.command_queue_path_preview_stage_count_sample, 6);
        assert_eq!(
            evidence.command_queue_path_preview_action_kinds_sample,
            vec![
                "select-control-group",
                "move",
                "move",
                "attack",
                "queue",
                "queue"
            ]
        );
        assert_eq!(
            evidence.command_queue_path_preview_action_payloads_sample,
            vec![
                "box:frontline",
                "8,4:line",
                "9,2:rally",
                "arena_creep_attack",
                "build:watch_tower@7,4",
                "cancel:build:0"
            ]
        );
        assert_eq!(
            evidence.command_queue_path_preview_history_entries_sample,
            vec![
                "command_queue_path_preview:queue_stack",
                "command_queue_path_preview:shift_waypoints",
                "command_queue_path_preview:rally_chain",
                "command_queue_path_preview:attack_focus",
                "command_queue_path_preview:build_reservation",
                "command_queue_path_preview:cancel_repath"
            ]
        );
        assert_eq!(
            evidence.formation_move_preview_stage_sample.as_deref(),
            Some("commit_spacing")
        );
        assert_eq!(evidence.formation_move_preview_stage_count_sample, 6);
        assert_eq!(
            evidence.formation_move_preview_action_payloads_sample,
            vec![
                "box:frontline",
                "8,4:wedge",
                "8,4:line",
                "8,4:wedge",
                "6,5:split",
                "9,2:rally"
            ]
        );
        assert_eq!(
            evidence.formation_move_preview_history_entries_sample,
            vec![
                "formation_move_preview:destination_ghost",
                "formation_move_preview:wedge_spacing",
                "formation_move_preview:line_reflow",
                "formation_move_preview:collision_avoidance",
                "formation_move_preview:split_avoidance",
                "formation_move_preview:commit_spacing"
            ]
        );
        assert_eq!(
            evidence.formation_move_preview_destination_slots_sample,
            vec!["8,4", "7,4", "8,5", "9,4"]
        );
        assert_eq!(
            evidence.formation_move_preview_split_route_sample,
            vec!["5,5", "6,4", "6,5", "7,5", "6,6"]
        );
        assert_eq!(
            evidence.control_group_recall_formation_preview_stage_count_sample,
            4
        );
        assert_eq!(
            evidence.control_group_recall_formation_preview_action_payloads_sample,
            vec!["28", "1,31:line", "1,31:line", "1,31:line"]
        );
        assert_eq!(
            evidence.control_group_recall_formation_preview_history_entries_sample,
            vec![
                "control_group_recall_formation_preview:recall_focus_hud",
                "control_group_recall_formation_preview:formation_anchor_slots",
                "control_group_recall_formation_preview:queued_valid_members",
                "control_group_recall_formation_preview:filtered_invalid"
            ]
        );
        assert_eq!(
            evidence.control_group_recall_formation_preview_slot_tiles_sample,
            vec!["1,31", "2,31"]
        );
        assert_eq!(
            evidence.control_group_recall_formation_preview_filtered_members_sample,
            vec![
                "missing:multi0.recall.formation.missing",
                "foreign:map.actor1"
            ]
        );
        assert_eq!(
            evidence.control_group_recall_override_preview_stage_count_sample,
            4
        );
        assert_eq!(
            evidence.control_group_recall_override_preview_action_payloads_sample,
            vec!["26", "18,31:line", "27", "20,30:line"]
        );
        assert_eq!(
            evidence.control_group_recall_override_preview_history_entries_sample,
            vec![
                "control_group_recall_override_preview:group_26_recall_focus",
                "control_group_recall_override_preview:group_26_queued_order",
                "control_group_recall_override_preview:group_27_override_cancel",
                "control_group_recall_override_preview:group_27_final_filtered"
            ]
        );
        assert_eq!(
            evidence.control_group_recall_override_preview_final_tiles_sample,
            vec!["20,30", "22,30"]
        );
        assert_eq!(
            evidence.control_group_recall_override_preview_canceled_members_sample,
            vec![
                "multi0.recall.override.runner",
                "multi0.recall.override.wing"
            ]
        );
        assert_eq!(
            evidence.formation_move_execution_stage_sample.as_deref(),
            Some("arrival_lock")
        );
        assert_eq!(
            evidence.local_obstruction_recovery_stage_sample.as_deref(),
            Some("flow_resume")
        );
        assert_eq!(
            evidence.npc_behavior_stage_sample.as_deref(),
            Some("creep_retreat")
        );
        assert_eq!(
            evidence.combat_impact_stage_sample.as_deref(),
            Some("damage_tick")
        );
        assert_eq!(
            evidence.locomotion_blend_stage_sample.as_deref(),
            Some("formation_slide")
        );
        assert_eq!(
            evidence.npc_transition_stage_sample.as_deref(),
            Some("hit_recover")
        );
        assert_eq!(
            evidence.depth_readability_stage_sample.as_deref(),
            Some("target_priority")
        );
        assert_eq!(
            evidence.structure_modeling_stage_sample.as_deref(),
            Some("repair_beam")
        );
        assert_eq!(
            evidence.environment_life_stage_sample.as_deref(),
            Some("resource_glint")
        );
        assert_eq!(
            evidence.worker_harvest_animation_stage_sample.as_deref(),
            Some("return_path")
        );
        assert_eq!(
            evidence.production_spawn_animation_stage_sample.as_deref(),
            Some("supply_flash")
        );
        assert_eq!(evidence.action_cadence_attack_mark_count_sample, 22);
        assert_eq!(evidence.action_cadence_carry_mark_count_sample, 8);
        assert_eq!(evidence.action_cadence_idle_mark_count_sample, 4);
        assert_eq!(evidence.action_cadence_creep_windup_offset_sample, -24);
        assert_eq!(
            evidence.action_sequence_phase_sample.as_deref(),
            Some("recovery")
        );
        assert_eq!(evidence.action_sequence_windup_mark_count_sample, 9);
        assert_eq!(evidence.action_sequence_strike_mark_count_sample, 12);
        assert_eq!(evidence.action_sequence_carry_down_mark_count_sample, 5);
        assert_eq!(evidence.action_sequence_idle_mark_count_sample, 6);
        assert_eq!(evidence.unit_model_depth_guard_mark_count_sample, 8);
        assert_eq!(evidence.unit_model_depth_worker_mark_count_sample, 8);
        assert_eq!(evidence.unit_model_depth_creep_mark_count_sample, 8);
        assert_eq!(evidence.unit_model_depth_creep_role_prop_count_sample, 2);
        assert_eq!(evidence.unit_model_depth_face_shade_offset_sample, -32);
        assert_eq!(
            evidence.command_surface_stage_sample.as_deref(),
            Some("target_queue")
        );
        assert_eq!(evidence.command_grid_hit_sample, Some(0));
        assert_eq!(evidence.tile_line_sample.len(), 9);
        assert_eq!(evidence.tile_line_sample[4].tile_x, 10);
        assert_eq!(evidence.tile_line_sample[4].tile_y, 12);
        assert_eq!(evidence.tile_line_sample[8].tile_x, 12);
        assert_eq!(evidence.tile_line_sample[8].tile_y, 16);
        assert_eq!(
            evidence.combat_engagement_tiles_sample,
            vec!["9,3", "10,3", "10,2", "11,2"]
        );
        assert_eq!(evidence.combat_flash_tiles_sample, vec!["6,5", "6,4"]);
        assert_eq!(
            evidence.combat_target_tile_sample,
            RtsEvidencePoint { x: 9, y: 3 }
        );
        assert_eq!(
            evidence.combat_target_priority_sample,
            vec![
                "arena_creep_attack",
                "arena_guard_support",
                "arena_worker_support"
            ]
        );
        assert_eq!(
            evidence.combat_projectile_trail_sample,
            vec!["5,5", "6,5", "7,4", "8,3"]
        );
        assert_eq!(
            evidence.combat_ability_effect_tiles_sample,
            vec!["10,3", "10,2", "11,2", "9,3"]
        );
        assert_eq!(evidence.combat_threat_levels_sample, vec![88, 66, 41]);
        assert_eq!(evidence.combat_damage_ticks_sample, vec![16, 21, 35]);
        assert_eq!(evidence.combat_projectile_id_sample, "guard_break_bolt");
        assert_eq!(
            evidence.ai_pressure_wave_units_sample,
            vec!["lane_scout", "mirror_raider", "siege_runner"]
        );
        assert_eq!(
            evidence.ai_pressure_tiles_sample,
            vec!["9,3", "8,4", "7,4", "6,5"]
        );
        assert_eq!(
            evidence.ai_pressure_counter_tiles_sample,
            vec!["5,5", "6,5", "6,4", "7,5"]
        );
        assert_eq!(
            evidence.enemy_pressure_wave_units_sample,
            vec!["enemy_raider", "enemy_signal_guard", "enemy_sapper"]
        );
        assert_eq!(
            evidence.enemy_pressure_lane_tiles_sample,
            vec!["10,2", "9,3", "8,4", "7,4", "6,5"]
        );
        assert_eq!(
            evidence.recon_scout_route_tiles_sample,
            vec!["5,5", "6,4", "7,4", "8,3", "9,2", "10,2"]
        );
        assert_eq!(
            evidence.recon_fog_reveal_tiles_sample,
            vec!["7,4", "8,3", "8,2", "9,2", "9,3", "10,2", "10,3", "11,1", "11,2"]
        );
        assert_eq!(
            evidence.recon_enemy_structures_sample,
            vec!["enemy_watch_post", "enemy_barracks", "enemy_resource_vault"]
        );
        assert_eq!(
            evidence.recon_enemy_units_sample,
            vec!["enemy_scout", "enemy_worker", "enemy_guard"]
        );
        assert_eq!(
            evidence.recon_enemy_structure_tile_sample,
            RtsEvidencePoint { x: 11, y: 2 }
        );
        assert_eq!(
            evidence.recon_enemy_unit_tile_sample,
            RtsEvidencePoint { x: 11, y: 2 }
        );
        assert_eq!(
            evidence.base_assault_path_tiles_sample,
            vec!["5,5", "6,5", "7,4", "8,4", "9,3", "10,3"]
        );
        assert_eq!(
            evidence.base_assault_targets_sample,
            vec!["enemy_watch_post", "enemy_barracks", "enemy_resource_vault"]
        );
        assert_eq!(
            evidence.aftermath_debris_tiles_sample,
            vec!["9,3", "10,3", "10,4", "11,3"]
        );
        assert_eq!(
            evidence.aftermath_smoke_tiles_sample,
            vec!["10,2", "10,3", "11,3"]
        );
        assert_eq!(
            evidence.commander_aura_tiles_sample,
            vec!["6,5", "7,4", "8,4", "9,3", "10,3"]
        );
        assert_eq!(
            evidence.commander_loot_items_sample,
            vec![
                "barracks_map_cache",
                "field_banner_relic",
                "repair_kit_crate"
            ]
        );
        assert_eq!(
            evidence.expansion_claim_tiles_sample,
            vec!["8,2", "9,2", "10,2", "9,3", "10,3"]
        );
        assert_eq!(
            evidence.expansion_structure_tile_sample,
            RtsEvidencePoint { x: 8, y: 3 }
        );
        assert_eq!(
            evidence.expansion_workers_sample,
            vec![
                "expansion_worker_alpha",
                "expansion_worker_beta",
                "expansion_worker_gamma"
            ]
        );
        assert_eq!(
            evidence.counterattack_units_sample,
            vec![
                "counter_raider_alpha",
                "counter_raider_beta",
                "counter_sapper"
            ]
        );
        assert_eq!(
            evidence.counterattack_route_tiles_sample,
            vec!["11,2", "10,2", "9,3", "8,3", "7,4", "9,2"]
        );
        assert_eq!(
            evidence.army_units_sample,
            vec![
                "relay_guard_alpha",
                "relay_guard_beta",
                "wayfinder_scout",
                "field_mender"
            ]
        );
        assert_eq!(
            evidence.army_rally_tiles_sample,
            vec!["5,5", "6,5", "7,4", "8,4", "8,3"]
        );
        assert_eq!(
            evidence.player_army_unit_tile_sample,
            RtsEvidencePoint { x: 6, y: 4 }
        );
        assert_eq!(
            evidence.central_keep_route_tiles_sample,
            vec!["12,3", "12,4", "13,4", "13,3", "14,3"]
        );
        assert_eq!(
            evidence.central_keep_tile_sample,
            RtsEvidencePoint { x: 13, y: 3 }
        );
        assert_eq!(
            evidence.boss_guard_units_sample,
            vec!["keep_warden_alpha", "keep_warden_beta", "ward_sentinel"]
        );
        assert_eq!(
            evidence.player_siege_line_tiles_sample,
            vec!["11,4", "12,4", "13,4", "12,3"]
        );
        assert_eq!(
            evidence.keep_breach_tiles_sample,
            vec!["13,3", "13,4", "14,3", "14,4"]
        );
        assert_eq!(
            evidence.guardian_counter_units_sample,
            vec!["high_warden", "ward_lancer", "last_mirror_guard"]
        );
        assert_eq!(
            evidence.keep_claim_tiles_sample,
            vec!["12,3", "13,3", "14,3", "13,4"]
        );
        assert_eq!(
            evidence.objective_tiles_sample,
            vec!["6,5", "6,4", "7,5", "9,2"]
        );
        assert_eq!(
            evidence.creep_camp_tiles_sample,
            vec!["8,3", "8,2", "9,3", "9,2"]
        );
        assert_eq!(
            evidence.terrain_route_tiles_sample,
            vec!["5,5", "6,5", "7,4", "8,3"]
        );
        assert_eq!(
            evidence.terrain_choke_tiles_sample,
            vec!["7,4", "7,3", "8,4"]
        );
        assert_eq!(evidence.expansion_tiles_sample, vec!["9,2", "10,2", "10,3"]);
        assert_eq!(evidence.siege_units_sample, vec!["stonebreak_cart"]);
        assert_eq!(
            evidence.siege_push_route_tiles_sample,
            vec!["9,2", "9,3", "10,3", "10,2", "11,2", "10,3"]
        );
        assert_eq!(
            evidence.siege_breach_tiles_sample,
            vec!["9,3", "10,3", "10,2", "11,2", "10,3"]
        );
        assert_eq!(
            evidence.enemy_fortification_tile_sample,
            RtsEvidencePoint { x: 10, y: 3 }
        );
        assert_eq!(
            evidence.enemy_repair_units_sample,
            vec!["repair_adept_alpha", "repair_adept_beta"]
        );
        assert_eq!(
            evidence.enemy_flank_units_sample,
            vec!["ridge_sentry_left", "ridge_sentry_right", "ridge_sapper"]
        );
        assert_eq!(
            evidence.enemy_flank_tile_sample,
            RtsEvidencePoint { x: 8, y: 4 }
        );
        assert_eq!(
            evidence.player_hold_tiles_sample,
            vec!["8,3", "9,3", "9,4", "10,3"]
        );
        assert_eq!(
            evidence.inner_lane_tiles_sample,
            vec!["10,3", "11,2", "11,3", "12,3", "12,4"]
        );
        assert_eq!(
            evidence.inner_gate_tile_sample,
            RtsEvidencePoint { x: 11, y: 3 }
        );
        assert_eq!(
            evidence.signal_lock_tile_sample,
            RtsEvidencePoint { x: 12, y: 3 }
        );
        assert_eq!(
            evidence.inner_defenders_sample,
            vec!["inner_guard_alpha", "inner_guard_beta", "signal_lancer"]
        );
        assert_eq!(
            evidence.supply_convoy_sample,
            vec!["convoy_cart", "field_medic", "ammo_runner"]
        );
        assert_eq!(
            evidence.split_squad_tiles_sample,
            vec!["10,4", "11,4", "12,4", "12,3"]
        );
        assert_eq!(
            evidence.inner_core_tile_sample,
            RtsEvidencePoint { x: 12, y: 3 }
        );
        assert_eq!(
            evidence.restored_zones_sample,
            vec!["central_keep", "signal_core", "inner_lane", "forest_relay"]
        );
        assert_eq!(
            evidence.rebuild_structures_sample,
            vec!["signal_core", "inner_latch", "mirror_ward"]
        );
        assert_eq!(
            evidence.garrison_units_sample,
            vec!["mirror_guard_alpha", "signal_lancer", "field_engineer"]
        );
        assert_eq!(
            evidence.open_world_route_tiles_sample,
            vec!["13,3", "12,3", "11,3", "10,2", "9,2"]
        );
        assert_eq!(
            evidence.open_world_panels_sample,
            vec![
                "room_panel:league-coliseum",
                "task_panel:task-fixture-first-route",
                "combat_panel:league-coliseum",
                "save_panel:post_rts_restore"
            ]
        );
        assert_eq!(
            evidence.siege_unit_tile_sample,
            RtsEvidencePoint { x: 9, y: 3 }
        );
        assert_eq!(
            evidence.harvest_tile_sample,
            RtsEvidencePoint { x: 3, y: 3 }
        );
        assert_eq!(
            evidence.dropoff_tile_sample,
            RtsEvidencePoint { x: 5, y: 5 }
        );
        assert_eq!(evidence.build_site_tiles_sample, vec!["7,4", "7,5", "8,4"]);
        assert_eq!(
            evidence.structure_tile_sample,
            RtsEvidencePoint { x: 4, y: 3 }
        );
        assert_eq!(
            evidence.unlock_unit_tile_sample,
            RtsEvidencePoint { x: 7, y: 5 }
        );
        assert_eq!(evidence.queue_gold_cost_sample, 210);
        assert_eq!(evidence.queue_available_gold_sample, 40);
        assert!(!evidence.queue_affordable_sample);
        assert_eq!(
            evidence.queue_build_parts_sample,
            vec!["watch_tower", "7,4"]
        );
        assert!(evidence.queue_production_lane_sample);
        assert_eq!(
            evidence.queue_feedback_chip_sample,
            "feedback:build_placed:watch_tower@7,4"
        );
        assert!(evidence.blocked_feedback_chip_visible_sample);
        assert_eq!(
            evidence.queue_blocked_feedback_label_sample,
            "QUEUE LOCK NEED 210G"
        );
        assert_eq!(evidence.command_panel_slot_id_sample, "attack");
        assert_eq!(
            evidence.command_panel_build_palette_queue_id_sample,
            "build:watch_tower@7,4"
        );
        assert_eq!(
            evidence.command_panel_production_slot_queue_id_sample,
            "build:watch_tower@7,4"
        );
        assert_eq!(
            evidence
                .command_panel_sidebar_cancel_queue_id_sample
                .as_deref(),
            Some("cancel:build:0")
        );
        assert_eq!(
            evidence
                .command_panel_palette_cancel_queue_id_sample
                .as_deref(),
            Some("cancel:active_build")
        );
        assert_eq!(
            evidence.command_panel_sidebar_slot_status_label_sample,
            "B1 66 R"
        );
        assert_eq!(evidence.command_panel_palette_state_label_sample, "ACT");
        assert_eq!(
            evidence.command_panel_sidebar_queue_summary_sample,
            "P:worker@42% B:watch_tower@66%"
        );
        assert_eq!(evidence.command_panel_spawned_unit_id_sample, "worker_3");
        assert_eq!(evidence.command_panel_structure_id_sample, "watch_tower");
        assert!(evidence.scripted_demo_pauses_queue_tick_sample);
        assert_eq!(evidence.scripted_demo_stage_from_frame_sample, Some(4));
        assert_eq!(evidence.scripted_demo_stage_id_sample, "cancel_refund");
        assert_eq!(evidence.scripted_demo_stage_title_sample, "WORKER QUEUED");
        assert_eq!(
            evidence.selection_default_units_sample,
            vec![
                "player",
                "square_guard_patrol",
                "square_worker_carry",
                "square_creep_wander"
            ]
        );
        assert_eq!(
            evidence.selection_same_class_units_sample,
            vec!["player", "square_guard_front", "square_guard_patrol"]
        );
        assert_eq!(
            evidence.selection_guard_tile_sample,
            Some(RtsEvidencePoint { x: 7, y: 5 })
        );
        assert_eq!(
            evidence.selection_drag_units_sample,
            vec![
                "player",
                "square_guard_front",
                "square_guard_patrol",
                "square_worker_carry",
                "square_worker_harvest"
            ]
        );
        assert_eq!(
            evidence.selection_drag_rejected_units_sample,
            vec!["square_creep_wander"]
        );
        assert_eq!(evidence.selection_drag_distance_sq_sample, 107_300);
        assert!(evidence.selection_drag_ready_sample);
        assert_eq!(evidence.selection_drag_group_id_sample, "drag:4,4->8,5");
        assert_eq!(
            evidence.selection_drag_player_label_sample,
            "DRAG SELECT 5 UNITS 4,4->8,5"
        );
        assert_eq!(
            evidence.selection_tiles_for_units_sample,
            vec!["5,4", "4,5"]
        );
        assert_eq!(
            evidence.control_group_hotkey_slot_sample.as_deref(),
            Some("10")
        );
        assert_eq!(
            evidence.control_group_default_slot_three_units_sample,
            vec!["square_worker_carry", "square_worker_harvest"]
        );
        assert_eq!(
            evidence.control_group_assignment_units_sample,
            vec!["square_worker_carry", "square_worker_harvest"]
        );
        assert_eq!(evidence.control_group_summary_slot_ten_sample.slot, "10");
        assert_eq!(
            evidence.control_group_summary_slot_ten_sample.key_label,
            "0"
        );
        assert_eq!(
            evidence.control_group_summary_slot_ten_sample.member_count,
            2
        );
        assert!(evidence.control_group_summary_slot_ten_sample.occupied);
        assert!(evidence.control_group_summary_slot_ten_sample.active);
        assert_eq!(
            evidence.control_group_merged_units_sample,
            vec!["player", "square_worker_carry"]
        );
        assert_eq!(
            evidence.selection_clear_parts_sample,
            Some((
                "hostile".to_string(),
                Some("square_creep_wander".to_string()),
                "9,4".to_string()
            ))
        );
        assert_eq!(
            evidence.move_command_parts_sample,
            vec!["9,2", "attack_move"]
        );
        assert_eq!(evidence.line_path_tiles_sample, vec!["6,5", "7,4", "8,3"]);
        assert_eq!(
            evidence.focus_fire_units_sample,
            vec![
                "relay_guard_alpha",
                "relay_guard_beta",
                "wayfinder_scout",
                "field_mender"
            ]
        );
        assert_eq!(
            evidence.creep_camp_units_sample,
            vec!["forest_alpha_creep", "forest_stalker", "forest_shaman"]
        );
        assert_eq!(
            evidence.command_parts_samples,
            vec![
                vec!["claim", "relay_beacon", "9,2"],
                vec!["clear", "forest_creep_camp", "8,3"],
                vec!["mark", "enemy_base", "10,2"],
                vec!["pressure", "counter_wave", "enemy_gate"],
                vec!["upgrade", "signal_blade", "training_hall"],
                vec!["train", "mixed_vanguard", "training_hall"],
                vec!["breach", "enemy_barracks", "10,3"],
                vec!["destroy", "enemy_barracks", "10,3"],
                vec!["level", "mirror_captain", "forest_relay"],
                vec!["claim", "forest_relay", "9,2"],
                vec!["tech", "stonebreak_cart", "relay_outpost"],
            ]
        );
        assert_eq!(
            evidence.selection_command_stamp_sample.kind,
            "control-group"
        );
        assert_eq!(
            evidence.selection_command_stamp_sample.target_id.as_deref(),
            Some("5")
        );
        assert_eq!(
            evidence.selection_command_stamp_sample.player_label,
            "HOTKEY GROUP 5 ASSIGNED 2 UNITS"
        );
        assert_eq!(evidence.move_command_stamp_sample.kind, "move");
        assert_eq!(
            evidence.move_command_stamp_sample.tile_id.as_deref(),
            Some("7,4")
        );
        assert_eq!(
            evidence.move_command_stamp_sample.player_label,
            "MAP MOVE SENT 7,4"
        );
        assert_eq!(evidence.ability_command_stamp_sample.kind, "ability");
        assert_eq!(
            evidence.ability_command_stamp_sample.tile_id.as_deref(),
            Some("6,5")
        );
        assert_eq!(
            evidence.ability_command_stamp_sample.target_id.as_deref(),
            Some("arena_creep_attack")
        );
        assert_eq!(
            evidence.ability_command_stamp_sample.player_label,
            "COMMAND BAR ABILITY SENT FOCUS FIRE"
        );
        assert_eq!(
            evidence.command_feedback_strip_stage_sample.as_deref(),
            Some("group_27_override")
        );
        assert_eq!(
            evidence.command_feedback_strip_fixture_stage_names_sample,
            vec![
                "group_26_queued",
                "group_27_override",
                "group_28_formation",
                "group_28_filtered"
            ]
        );
        assert_eq!(
            evidence.command_feedback_strip_fixture_action_payloads_sample,
            vec!["18,31:line", "27", "1,31:line", "1,31:line"]
        );
        assert_eq!(
            evidence.command_feedback_strip_fixture_focus_tiles_sample,
            vec!["18,30", "21,30", "1,30", "1,30"]
        );
        assert_eq!(
            evidence.command_feedback_strip_fixture_filtered_members_sample,
            vec![
                "missing:multi0.recall.formation.missing",
                "foreign:map.actor1"
            ]
        );
        assert_eq!(
            evidence.command_feedback_lifecycle_stage_sample.as_deref(),
            Some("dimmed")
        );
        assert_eq!(
            evidence.command_feedback_replay_step_names_sample,
            vec![
                "select_group_26",
                "queue_group_26",
                "select_group_27",
                "override_group_27",
                "select_group_28",
                "formation_group_28",
                "bounded_history_after_clear"
            ]
        );
        assert_eq!(
            evidence.command_feedback_replay_preview_stages_sample,
            vec![
                "group_26_queued",
                "group_27_override",
                "group_28_formation",
                "cleared_history_bounded"
            ]
        );
        assert_eq!(
            evidence.command_feedback_replay_retained_group_ids_sample,
            vec!["26", "27", "28"]
        );
        assert_eq!(
            evidence.command_feedback_replay_pruned_group_ids_sample,
            vec!["25", "24"]
        );
        assert_eq!(
            evidence.command_feedback_replay_history_badges_sample,
            vec!["QUEUE", "CANCEL_FINAL", "FORMATION_FILTER_CLEAR"]
        );
        assert_eq!(
            evidence.command_feedback_rejection_replay_step_names_sample,
            vec![
                "move_without_group_selection",
                "select_group_26_setup",
                "move_invalid_tile_after_selection",
                "attack_without_target",
                "ability_before_attack_target",
                "queue_without_queue_id",
                "queue_unaffordable_build_after_selection",
                "select_without_group_id"
            ]
        );
        assert_eq!(
            evidence.command_feedback_rejection_replay_preview_stages_sample,
            vec![
                "group_selection_required",
                "invalid_tile",
                "attack_target_required",
                "history_preserved_after_rejections"
            ]
        );
        assert_eq!(
            evidence.command_feedback_rejection_replay_input_sources_sample,
            vec![
                "classic_rts_mouse_viewport",
                "classic_rts_hotkey",
                "classic_rts_mouse_viewport",
                "classic_rts_mouse_viewport",
                "classic_rts_hotkey",
                "classic_rts_mouse_sidebar",
                "classic_rts_mouse_sidebar",
                "classic_rts_hotkey"
            ]
        );
        assert_eq!(
            evidence.command_feedback_rejection_replay_blocked_reasons_sample,
            vec![
                "rts_group_selection_required",
                "rts_invalid_tile:bad-tile",
                "rts_attack_target_required",
                "rts_attack_required_before_ability",
                "rts_queue_id_required",
                "rts_queue_unaffordable:build:watch_tower@7,4",
                "rts_group_id_required"
            ]
        );
        assert_eq!(
            evidence.command_feedback_rejection_replay_visual_stages_sample,
            vec![
                "group_selection_required",
                "invalid_tile",
                "attack_target_required",
                "history_preserved_after_rejections"
            ]
        );
        assert_eq!(
            evidence.command_feedback_rejection_replay_retained_group_ids_sample,
            vec!["26", "27", "28"]
        );
        assert_eq!(
            evidence.command_feedback_rejection_replay_pruned_group_ids_sample,
            vec!["25", "24"]
        );
        assert!(evidence.command_history_visible_sample);
        assert!(evidence.command_history_prune_visible_sample);
        assert_eq!(
            evidence.command_history_fixture_stage_names_sample,
            vec![
                "fresh_history_appended",
                "dimmed_history_retained",
                "cleared_history_retained"
            ]
        );
        assert_eq!(
            evidence.command_history_fixture_lifecycle_stages_sample,
            vec!["fresh", "dimmed", "cleared"]
        );
        assert_eq!(
            evidence.command_history_fixture_group_ids_sample,
            vec!["26", "27", "28"]
        );
        assert_eq!(
            evidence.command_history_prune_fixture_stage_names_sample,
            vec![
                "overflow_input_pruned",
                "recent_three_retained",
                "cleared_history_bounded"
            ]
        );
        assert_eq!(
            evidence.command_history_prune_fixture_pruned_group_ids_sample,
            vec!["25", "24"]
        );
        assert_eq!(
            evidence.command_history_prune_fixture_prune_reasons_sample,
            vec!["recent_three_capacity", "recent_three_capacity"]
        );
        assert_eq!(
            evidence.command_execution_feedback_kind_samples,
            vec!["move", "follow", "attack", "harvest"]
        );
        assert_eq!(
            evidence.command_execution_target_label_samples,
            vec!["8,4", "player", "arena_creep_attack", "gold_vein"]
        );
        assert_eq!(
            evidence.command_execution_player_label_samples,
            vec![
                "MOVE EXECUTING 8,4",
                "FOLLOWING PLAYER",
                "ATTACK FOCUS ARENA CREEP ATTACK",
                "HARVEST GOLD VEIN TO TOWN HALL"
            ]
        );
        assert_eq!(
            evidence.command_execution_target_tile_samples,
            vec![
                RtsEvidencePoint { x: 8, y: 4 },
                RtsEvidencePoint { x: 5, y: 4 },
                RtsEvidencePoint { x: 6, y: 5 },
                RtsEvidencePoint { x: 3, y: 3 }
            ]
        );
        assert_eq!(
            evidence.hover_target_preview_kind_sample.as_deref(),
            Some("attack")
        );
        assert_eq!(evidence.hover_cursor_kind_sample, "ability");
        assert_eq!(
            evidence.hover_cursor_label_sample,
            "COMMAND BAR CURSOR ABILITY READY"
        );
        assert_eq!(evidence.blocked_cursor_kind_sample, "blocked");
        assert_eq!(
            evidence.blocked_cursor_label_sample,
            "MAP CURSOR BLOCKED LOCK"
        );
        assert_eq!(
            evidence.hover_player_label_sample,
            "MAP ATTACK READY SQUARE CREEP WANDER"
        );
        assert_eq!(
            evidence.hover_queue_player_label_sample,
            "SIDEBAR QUEUE READY WATCH TOWER 7,4 210G"
        );
        assert_eq!(
            evidence.blocked_hover_player_label_sample,
            "MAP MOVE LOCK SELECT UNITS"
        );
        assert_eq!(
            evidence.unit_status_stage_sample.as_deref(),
            Some("commander")
        );
        assert_eq!(evidence.unit_status_unit_id_sample, "mirror_captain");
        assert_eq!(evidence.unit_status_health_sample, 76);
        assert_eq!(evidence.unit_status_energy_sample, 68);
        assert_eq!(
            evidence.unit_status_role_badges_sample,
            vec!["AUR", "LVL", "CMD"]
        );
        assert_eq!(
            evidence.selection_feedback_stage_sample.as_deref(),
            Some("attack_lock")
        );
        assert_eq!(
            evidence.ability_tooltip_stage_sample.as_deref(),
            Some("range_preview")
        );
        assert_eq!(
            evidence
                .control_group_hotkey_feedback_stage_sample
                .as_deref(),
            Some("double_tap_camera")
        );
    }
}
