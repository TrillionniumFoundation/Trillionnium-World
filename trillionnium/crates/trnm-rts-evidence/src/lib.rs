#![recursion_limit = "256"]

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
    rts_expansion_tiles_for_id, rts_expansion_workers_for_line,
    rts_first_contact_offline_adapter_consumption_review,
    rts_first_contact_offline_adapter_lobby_ready_review,
    rts_first_contact_offline_adapter_runtime_application,
    rts_first_contact_offline_adapter_session_transition_review,
    rts_first_contact_player_screen_runtime_application, rts_focus_fire_units_for_target,
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
    rts_recon_parts, rts_restored_zones_for_id, rts_runtime_hit_test_grid,
    rts_runtime_map_projection, rts_runtime_terrain_seeds, rts_runtime_tile_line,
    rts_runtime_tile_screen_rect, rts_same_class_units, rts_scout_route_tiles_for_recon,
    rts_scripted_demo_pauses_queue_tick, rts_scripted_demo_stage_from_frame,
    rts_scripted_demo_stage_id, rts_scripted_demo_stage_title,
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
    RtsControlGroupSlotSummary, RtsFirstContactOfflineAdapterConsumptionReview,
    RtsFirstContactOfflineAdapterLobbyReadyReview,
    RtsFirstContactOfflineAdapterSessionTransitionReview, RtsFirstContactPlayerScreenReview,
    RtsFirstContactPlayerScreenRuntimeApplication, RtsOfflineAdapterRuntimeApplication,
    RtsOrderQueueReplayAction, RtsRuntimeGridSpec, RtsRuntimeMapLayoutInput,
    RtsRuntimeMapProjection, RtsRuntimeRect, RtsRuntimeTerrainSeeds, RtsRuntimeTileLineStep,
    TRNM_RTS_BEVY_RUNTIME_CONTRACT,
    TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_APPLICATION_CONTRACT,
    TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_CONSUMPTION_CONTRACT,
    TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_LOBBY_READY_CONTRACT,
    TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_SESSION_TRANSITION_CONTRACT,
    TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_PLAYER_SCREEN_APPLICATION_CONTRACT,
};

#[cfg(not(target_os = "android"))]
mod first_contact_art_readability;
#[cfg(not(target_os = "android"))]
mod first_contact_atlas_readability;
#[cfg(not(target_os = "android"))]
mod first_contact_focus_readability;
#[cfg(not(target_os = "android"))]
mod first_contact_marker_budget;
#[cfg(not(target_os = "android"))]
mod first_contact_motion_readability;
#[cfg(not(target_os = "android"))]
mod first_contact_radar_readability;
#[cfg(not(target_os = "android"))]
mod first_contact_silhouette_readability;
#[cfg(not(target_os = "android"))]
mod first_contact_spatial_readability;
#[cfg(not(target_os = "android"))]
mod first_contact_visual_readability;

#[cfg(not(target_os = "android"))]
pub use first_contact_art_readability::first_contact_art_readability_guard;
#[cfg(not(target_os = "android"))]
pub use first_contact_atlas_readability::{
    first_contact_atlas_readability_guard, RtsFirstContactAtlasReadabilityRuntime,
};
#[cfg(not(target_os = "android"))]
pub use first_contact_focus_readability::{
    first_contact_selection_combat_focus_guard, first_contact_target_callout_guard,
    RtsFirstContactFocusReadabilityGeometrySnapshot, RtsFirstContactFocusReadabilityRuntime,
};
#[cfg(not(target_os = "android"))]
pub use first_contact_marker_budget::{
    first_contact_marker_budget_guard, RtsFirstContactFocusGeometrySnapshot,
    RtsFirstContactMarkerBudgetRuntime,
};
#[cfg(not(target_os = "android"))]
pub use first_contact_motion_readability::{
    first_contact_motion_readability_guard, RtsFirstContactMotionReadabilityRuntime,
};
#[cfg(not(target_os = "android"))]
pub use first_contact_radar_readability::{
    first_contact_radar_readability_guard, RtsFirstContactRadarReadabilityRuntime,
};
#[cfg(not(target_os = "android"))]
pub use first_contact_silhouette_readability::first_contact_silhouette_readability_guard;
#[cfg(not(target_os = "android"))]
pub use first_contact_spatial_readability::{
    first_contact_central_clarity_guard, first_contact_terminal_legibility_guard,
    first_contact_visual_hierarchy_guard, RtsFirstContactSpatialReadabilityRuntime,
};
#[cfg(not(target_os = "android"))]
pub use first_contact_visual_readability::{
    first_contact_visual_readability_guard, RtsFirstContactVisualReadabilityRuntime,
};

pub const TRNM_RTS_EVIDENCE_CONTRACT: &str = "trnm_rts_evidence_v1";
pub const TRNM_RTS_EVIDENCE_FIRST_CONTACT_ART_READABILITY_CONTRACT: &str =
    "trillionnium_world_bevy_classic_rts_first_contact_art_readability_v1";
pub const TRNM_RTS_EVIDENCE_FIRST_CONTACT_ATLAS_READABILITY_CONTRACT: &str =
    "trillionnium_world_bevy_classic_rts_first_contact_atlas_readability_v1";
pub const TRNM_RTS_EVIDENCE_FIRST_CONTACT_CENTRAL_CLARITY_CONTRACT: &str =
    "trillionnium_world_bevy_classic_rts_first_contact_central_clarity_v1";
pub const TRNM_RTS_EVIDENCE_FIRST_CONTACT_MOTION_READABILITY_CONTRACT: &str =
    "trillionnium_world_bevy_classic_rts_first_contact_motion_readability_v1";
pub const TRNM_RTS_EVIDENCE_FIRST_CONTACT_MARKER_BUDGET_CONTRACT: &str =
    "trillionnium_world_bevy_classic_rts_first_contact_marker_budget_v1";
pub const TRNM_RTS_EVIDENCE_FIRST_CONTACT_RADAR_READABILITY_CONTRACT: &str =
    "trillionnium_world_bevy_classic_rts_first_contact_radar_readability_v1";
pub const TRNM_RTS_EVIDENCE_FIRST_CONTACT_SELECTION_COMBAT_FOCUS_CONTRACT: &str =
    "trillionnium_world_bevy_classic_rts_first_contact_selection_combat_focus_v1";
pub const TRNM_RTS_EVIDENCE_FIRST_CONTACT_SILHOUETTE_READABILITY_CONTRACT: &str =
    "trillionnium_world_bevy_classic_rts_first_contact_silhouette_readability_v1";
pub const TRNM_RTS_EVIDENCE_FIRST_CONTACT_TARGET_CALLOUT_CONTRACT: &str =
    "trillionnium_world_bevy_classic_rts_first_contact_target_callout_v1";
pub const TRNM_RTS_EVIDENCE_FIRST_CONTACT_TERMINAL_LEGIBILITY_CONTRACT: &str =
    "trillionnium_world_bevy_classic_rts_first_contact_terminal_legibility_v1";
pub const TRNM_RTS_EVIDENCE_FIRST_CONTACT_VISUAL_READABILITY_CONTRACT: &str =
    "trillionnium_world_bevy_classic_rts_first_contact_visual_readability_v1";
pub const TRNM_RTS_EVIDENCE_FIRST_CONTACT_VISUAL_HIERARCHY_CONTRACT: &str =
    "trillionnium_world_bevy_classic_rts_first_contact_visual_hierarchy_v1";
pub const TRNM_RTS_EVIDENCE_FIRST_CONTACT_LOWER_LANE_GALLERY_DARKEN_NUMERATOR: u32 = 5;
pub const TRNM_RTS_EVIDENCE_FIRST_CONTACT_LOWER_LANE_GALLERY_DARKEN_DENOMINATOR: u32 = 6;
pub const TRNM_RTS_EVIDENCE_FIRST_CONTACT_SECONDARY_TRACK_DARKEN_NUMERATOR: usize = 3;
pub const TRNM_RTS_EVIDENCE_FIRST_CONTACT_SECONDARY_TRACK_DARKEN_DENOMINATOR: usize = 4;
pub const TRNM_RTS_EVIDENCE_BEVY_RUNTIME_ADAPTER_CONTRACT: &str =
    "trnm_rts_evidence_bevy_runtime_adapter_v1";
pub const TRNM_RTS_EVIDENCE_CAMPAIGN_UI_CONTINUITY_REVIEW_CONTRACT: &str =
    "trnm_rts_evidence_campaign_ui_continuity_review_v1";
pub const TRNM_RTS_EVIDENCE_SESSION_STATE_CONTINUITY_REVIEW_CONTRACT: &str =
    "trnm_rts_evidence_session_state_continuity_review_v1";
pub const TRNM_RTS_EVIDENCE_CONTINUOUS_PLAYER_FLOW_REVIEW_CONTRACT: &str =
    "trnm_rts_evidence_continuous_player_flow_review_v1";
pub const TRNM_RTS_EVIDENCE_LIVE_SESSION_PLAYTHROUGH_REVIEW_CONTRACT: &str =
    "trnm_rts_evidence_live_session_playthrough_review_v1";
pub const TRNM_RTS_EVIDENCE_FULL_GAME_VISUAL_UI_REPLICATION_REVIEW_CONTRACT: &str =
    "trnm_rts_evidence_full_game_visual_ui_replication_review_v1";
pub const TRNM_RTS_EVIDENCE_OPENRA_STYLE_SCREEN_SET_REVIEW_CONTRACT: &str =
    "trnm_rts_evidence_openra_style_screen_set_review_v1";
pub const TRNM_RTS_EVIDENCE_RELEASE_REVIEW_PACKET_ASSEMBLY_REVIEW_CONTRACT: &str =
    "trnm_rts_evidence_release_review_packet_assembly_review_v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsEvidencePoint {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsFirstContactTerrainProfileSamples {
    pub border: trnm_rts_data::RtsTerrainTileProfile,
    pub lane: trnm_rts_data::RtsTerrainTileProfile,
    pub center: trnm_rts_data::RtsTerrainTileProfile,
    pub base_pad: trnm_rts_data::RtsTerrainTileProfile,
    pub resource_zone: trnm_rts_data::RtsTerrainTileProfile,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsFirstContactPreviewActorProjectionEvidence {
    pub actor_count: usize,
    pub spawn_count: usize,
    pub flux_bloom_count: usize,
    pub beacon_count: usize,
    pub expansion_count: usize,
    pub actor_samples: Vec<trnm_rts_data::RtsFirstContactPreviewActor>,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsFirstContactMapModelReview {
    pub map_summary: trnm_rts_data::RtsMapSummary,
    pub unit_rule_count: usize,
    pub building_rule_count: usize,
    pub data_validation_error: Option<String>,
    pub map_actor_gate: bool,
    pub map_topology_gate: bool,
    pub rules_gate: bool,
    pub data_consumer_gate: bool,
    pub map_model_adapter_gate: bool,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsFirstContactRendererProjectionEvidence {
    pub renderable_tile_count: usize,
    pub lane_tile_count: usize,
    pub resource_zone_tile_count: usize,
    pub base_pad_tile_count: usize,
    pub minimap_anchor_actor_count: usize,
    pub resource_actor_tile_count: usize,
    pub objective_actor_tile_count: usize,
    pub spawn_actor_tile_count: usize,
    pub lane_tile_samples: Vec<trnm_rts_core::RtsTile>,
    pub resource_actor_tile_samples: Vec<trnm_rts_core::RtsTile>,
    pub objective_actor_tile_samples: Vec<trnm_rts_core::RtsTile>,
    pub spawn_actor_tile_samples: Vec<trnm_rts_core::RtsTile>,
    pub minimap_anchor_actor_samples: Vec<String>,
    pub source: String,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsContinuousPlayerFlowReview {
    pub contract_version: String,
    pub green: bool,
    pub continuous_player_flow_contract: String,
    pub continuous_player_flow_green: bool,
    pub preview_width: u64,
    pub preview_height: u64,
    pub continuous_player_flow_step_count: u64,
    pub shell_meta_surface_count: u64,
    pub match_setup_map_id: Option<String>,
    pub match_setup_faction_id: Option<String>,
    pub hud_surface_count: u64,
    pub hud_army_supply_used: u64,
    pub interaction_surface_count: u64,
    pub session_final_objective_status: Option<String>,
    pub session_open_world_state: Option<String>,
    pub campaign_outcome_open_world_state: Option<String>,
    pub campaign_continuity_restored_room_id: Option<String>,
    pub source_contract_gate: bool,
    pub transition_sequence_gate: bool,
    pub source_headline_gate: bool,
    pub pixel_gate: bool,
    pub title_account_gate: bool,
    pub match_setup_gate: bool,
    pub in_match_hud_gate: bool,
    pub command_feedback_gate: bool,
    pub save_resume_gate: bool,
    pub outcome_open_world_gate: bool,
    pub continuous_player_flow_chain_gate: bool,
    pub source_preview_gate: bool,
    pub preview_gate: bool,
    pub player_first_continuous_flow_screen_gate: bool,
    pub native_client_boundary_gate: bool,
    pub runtime_screen_gate: bool,
    pub no_credit_boundary_gate: bool,
    pub continuous_player_flow_gate: bool,
    pub input_path: String,
    pub evidence_path: String,
    pub source_of_truth: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsLiveSessionPlaythroughReview {
    pub contract_version: String,
    pub green: bool,
    pub live_session_playthrough_contract: String,
    pub live_session_playthrough_green: bool,
    pub preview_width: u64,
    pub preview_height: u64,
    pub stage_count: u64,
    pub top_level_action_count: u64,
    pub top_level_accepted_action_count: u64,
    pub accepted_input_count: u64,
    pub campaign_handoff_input_count: u64,
    pub live_command_input_count: u64,
    pub slot_a_bytes: u64,
    pub final_objective_status: Option<String>,
    pub final_open_world_handoff_state: Option<String>,
    pub final_open_world_resume_room_id: Option<String>,
    pub source_contract_gate: bool,
    pub stage_sequence_gate: bool,
    pub same_process_trace_gate: bool,
    pub title_account_gate: bool,
    pub match_setup_gate: bool,
    pub in_match_hud_gate: bool,
    pub command_feedback_gate: bool,
    pub save_resume_gate: bool,
    pub outcome_open_world_gate: bool,
    pub live_command_gate: bool,
    pub final_state_gate: bool,
    pub pixel_gate: bool,
    pub trace_sidecar_gate: bool,
    pub player_first_live_session_screen_gate: bool,
    pub runtime_screen_gate: bool,
    pub native_client_boundary_gate: bool,
    pub no_credit_boundary_gate: bool,
    pub live_session_playthrough_gate: bool,
    pub input_path: String,
    pub evidence_path: String,
    pub source_of_truth: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsFullGameVisualUiReplicationReview {
    pub contract_version: String,
    pub green: bool,
    pub full_game_visual_ui_replication_contract: String,
    pub full_game_visual_ui_replication_green: bool,
    pub preview_width: u64,
    pub preview_height: u64,
    pub coverage_surface_count: u64,
    pub runtime_screen_mode: Option<String>,
    pub session_state_review_contract: Option<String>,
    pub continuous_player_flow_review_contract: Option<String>,
    pub live_session_playthrough_review_contract: Option<String>,
    pub full_screen_surface_count: u64,
    pub shell_meta_surface_count: u64,
    pub match_setup_surface_count: u64,
    pub hud_surface_count: u64,
    pub continuous_step_count: u64,
    pub live_session_stage_count: u64,
    pub live_session_accepted_input_count: u64,
    pub live_session_final_objective_status: Option<String>,
    pub live_session_open_world_state: Option<String>,
    pub source_contract_gate: bool,
    pub source_green_gate: bool,
    pub source_review_gate: bool,
    pub coverage_surface_gate: bool,
    pub source_headline_gate: bool,
    pub player_flow_gate: bool,
    pub pixel_gate: bool,
    pub preview_gate: bool,
    pub runtime_screen_chain_gate: bool,
    pub runtime_screen_gate: bool,
    pub player_first_tactical_composition_gate: bool,
    pub player_first_full_game_visual_ui_screen_gate: bool,
    pub no_copy_boundary_gate: bool,
    pub full_game_visual_ui_replication_gate: bool,
    pub input_path: String,
    pub evidence_path: String,
    pub source_of_truth: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsOpenraStyleScreenSetReview {
    pub contract_version: String,
    pub green: bool,
    pub openra_screen_for_screen_ui_replication_contract: String,
    pub openra_screen_for_screen_ui_replication_green: bool,
    pub preview_width: u64,
    pub preview_height: u64,
    pub screen_for_screen_mode: Option<String>,
    pub runtime_screen_mode: Option<String>,
    pub openra_widget_root_count: u64,
    pub openra_reference_screen_count: u64,
    pub replicated_interaction_surface_count: u64,
    pub full_game_surface_count: u64,
    pub full_screen_surface_count: u64,
    pub shell_meta_surface_count: u64,
    pub match_setup_surface_count: u64,
    pub hud_surface_count: u64,
    pub session_surface_count: u64,
    pub openra_parity_lane_axis_count: u64,
    pub source_contract_gate: bool,
    pub source_green_gate: bool,
    pub openra_runtime_vocabulary_gate: bool,
    pub widget_root_reference_gate: bool,
    pub screen_set_gate: bool,
    pub source_screen_chain_gate: bool,
    pub pixel_gate: bool,
    pub preview_gate: bool,
    pub runtime_screen_gate: bool,
    pub player_first_openra_style_ingame_screen_gate: bool,
    pub no_asset_copy_boundary_gate: bool,
    pub no_credit_boundary_gate: bool,
    pub openra_style_ui_screen_set_replication_gate: bool,
    pub openra_screen_for_screen_ui_replication_gate: bool,
    pub openra_style_widget_root_screen_set_claimed: bool,
    pub input_path: String,
    pub evidence_path: String,
    pub source_of_truth: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsReleaseReviewPacketAssemblyReview {
    pub contract_version: String,
    pub green: bool,
    pub packet_contract: String,
    pub packet_status: String,
    pub artifact_count: u64,
    pub release_review_input_count: u64,
    pub release_review_visual_evidence_count: u64,
    pub release_review_recording_count: u64,
    pub release_review_collection_count: u64,
    pub release_review_gate_count: u64,
    pub release_review_operator_handoff_count: u64,
    pub release_review_checkpoint_count: u64,
    pub release_review_checklist_count: u64,
    pub release_review_log_count: u64,
    pub missing_artifact_count: u64,
    pub packet_integrity_fixture_count: u64,
    pub reviewed_runtime_artifact_count: u64,
    pub reviewed_packet_fixture_count: u64,
    pub ready_item_count: u64,
    pub blocked_item_count: u64,
    pub inventory_summary_gate: bool,
    pub artifact_manifest_gate: bool,
    pub missing_artifacts_gate: bool,
    pub release_review_readiness_gate: bool,
    pub status_handoff_gate: bool,
    pub key_runtime_artifacts_gate: bool,
    pub full_game_visual_ui_handoff_gate: bool,
    pub packet_integrity_fixture_gate: bool,
    pub public_launch_boundary_gate: bool,
    pub external_blocker_gate: bool,
    pub reviewed_runtime_artifact_ids: Vec<String>,
    pub reviewed_packet_fixture_ids: Vec<String>,
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

fn json_array_len_at(value: &Value, key: &str) -> u64 {
    value
        .get(key)
        .and_then(Value::as_array)
        .map(|items| items.len() as u64)
        .unwrap_or_default()
}

fn json_contract_is(value: &Value, expected: &str) -> bool {
    value.get("contract_version").and_then(Value::as_str) == Some(expected)
}

fn artifact_id_is(artifact: &Value, expected: &str) -> bool {
    artifact.get("id").and_then(Value::as_str) == Some(expected)
}

fn artifact_role_is(artifact: &Value, expected: &str) -> bool {
    artifact.get("role").and_then(Value::as_str) == Some(expected)
}

fn artifact_present_with_manifest_metadata(artifact: &Value) -> bool {
    artifact
        .get("id")
        .and_then(Value::as_str)
        .is_some_and(|id| !id.is_empty())
        && artifact
            .get("path")
            .and_then(Value::as_str)
            .is_some_and(|path| !path.is_empty())
        && artifact.get("file_status").and_then(Value::as_str) == Some("present")
        && artifact
            .get("sha256")
            .and_then(Value::as_str)
            .is_some_and(|sha| sha.len() == 64)
        && artifact
            .get("bytes")
            .and_then(Value::as_u64)
            .unwrap_or_default()
            > 0
}

fn artifacts_have_id(artifacts: &[Value], expected: &str) -> bool {
    artifacts.iter().any(|artifact| {
        artifact_id_is(artifact, expected) && artifact_present_with_manifest_metadata(artifact)
    })
}

fn artifacts_have_id_with_role(artifacts: &[Value], expected: &str, role: &str) -> bool {
    artifacts.iter().any(|artifact| {
        artifact_id_is(artifact, expected)
            && artifact_role_is(artifact, role)
            && artifact_present_with_manifest_metadata(artifact)
    })
}

fn artifacts_have_id_contract_status(
    artifacts: &[Value],
    expected: &str,
    contract_version: &str,
    status: &str,
) -> bool {
    artifacts.iter().any(|artifact| {
        artifact_id_is(artifact, expected)
            && artifact.get("contract_version").and_then(Value::as_str) == Some(contract_version)
            && artifact.get("status").and_then(Value::as_str) == Some(status)
            && artifact_present_with_manifest_metadata(artifact)
    })
}

fn artifact_role_count(artifacts: &[Value], role: &str) -> u64 {
    artifacts
        .iter()
        .filter(|artifact| artifact_role_is(artifact, role))
        .count() as u64
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

pub fn rts_continuous_player_flow_review(input: &Value) -> RtsContinuousPlayerFlowReview {
    let continuous_player_flow_contract =
        json_string_at(input, "contract_version").unwrap_or_default();
    let continuous_player_flow_green = json_bool_at(input, "green");
    let preview_width = json_u64_at(input, "preview_width");
    let preview_height = json_u64_at(input, "preview_height");
    let continuous_player_flow_step_count = json_u64_at(input, "continuous_player_flow_step_count");
    let shell_meta_surface_count =
        json_u64_pointer(input, "/source_headline/shell_meta_surface_count");
    let match_setup_map_id = json_string_pointer(input, "/source_headline/match_setup_map_id");
    let match_setup_faction_id =
        json_string_pointer(input, "/source_headline/match_setup_faction_id");
    let hud_surface_count = json_u64_pointer(input, "/source_headline/hud_surface_count");
    let hud_army_supply_used = json_u64_pointer(input, "/source_headline/hud_army_supply_used");
    let interaction_surface_count =
        json_u64_pointer(input, "/source_headline/interaction_surface_count");
    let session_final_objective_status =
        json_string_pointer(input, "/source_headline/session_final_objective_status");
    let session_open_world_state =
        json_string_pointer(input, "/source_headline/session_open_world_state");
    let campaign_outcome_open_world_state =
        json_string_pointer(input, "/source_headline/campaign_outcome_open_world_state");
    let campaign_continuity_restored_room_id = json_string_pointer(
        input,
        "/source_headline/campaign_continuity_restored_room_id",
    );

    let source_contract_gate =
        json_string_pointer(input, "/source_contracts/shell_meta_ui_replication").as_deref()
            == Some("trillionnium_world_bevy_classic_rts_shell_meta_ui_replication_v1")
            && json_string_pointer(input, "/source_contracts/match_setup_ui_replication")
                .as_deref()
                == Some("trillionnium_world_bevy_classic_rts_match_setup_ui_replication_v1")
            && json_string_pointer(input, "/source_contracts/in_match_hud_state_replication")
                .as_deref()
                == Some("trillionnium_world_bevy_classic_rts_in_match_hud_state_replication_v1")
            && json_string_pointer(input, "/source_contracts/production_interaction_polish")
                .as_deref()
                == Some("trillionnium_world_bevy_classic_rts_production_interaction_polish_v1")
            && json_string_pointer(input, "/source_contracts/session_state_continuity").as_deref()
                == Some("trillionnium_world_bevy_classic_rts_session_state_continuity_v1")
            && json_string_pointer(input, "/source_contracts/campaign_outcome_ui_readiness")
                .as_deref()
                == Some("trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness_v1")
            && json_string_pointer(input, "/source_contracts/campaign_ui_continuity").as_deref()
                == Some("trillionnium_world_bevy_classic_rts_campaign_ui_continuity_v1");
    let expected_steps = [
        "title_account",
        "match_setup",
        "in_match_hud",
        "command_feedback",
        "save_load_resume",
        "outcome_open_world",
    ];
    let step_ids_gate = input
        .pointer("/continuous_player_flow_steps")
        .and_then(Value::as_array)
        .is_some_and(|steps| {
            expected_steps.iter().all(|expected| {
                steps.iter().any(|step| {
                    step.get("step_id").and_then(Value::as_str) == Some(*expected)
                        && step
                            .get("runtime_screen_mode")
                            .and_then(Value::as_str)
                            .is_some_and(|mode| mode.starts_with("player_runtime_"))
                })
            })
        });
    let transition_sequence_gate = continuous_player_flow_step_count == 6
        && step_ids_gate
        && expected_steps
            .iter()
            .all(|expected| json_array_contains(input, "/transition_sequence", expected));
    let source_headline_gate = shell_meta_surface_count == 12
        && match_setup_map_id.as_deref() == Some("first_contact_basin")
        && match_setup_faction_id.as_deref() == Some("mirror_guard")
        && hud_surface_count == 8
        && hud_army_supply_used == 9
        && interaction_surface_count == 6
        && session_final_objective_status.as_deref() == Some("first_playable_loop_complete")
        && session_open_world_state.as_deref() == Some("resumed:league-coliseum")
        && campaign_outcome_open_world_state.as_deref() == Some("resumed:league-coliseum")
        && campaign_continuity_restored_room_id.as_deref() == Some("league-coliseum");
    let pixel_gate = json_u64_pointer(input, "/flow_pixel_counts/non_background") > 250_000
        && json_u64_pointer(input, "/flow_pixel_counts/board") > 100_000
        && json_u64_pointer(input, "/flow_pixel_counts/title_account") > 2_000
        && json_u64_pointer(input, "/flow_pixel_counts/match_setup") > 2_000
        && json_u64_pointer(input, "/flow_pixel_counts/in_match_hud") > 2_000
        && json_u64_pointer(input, "/flow_pixel_counts/command_feedback") > 2_000
        && json_u64_pointer(input, "/flow_pixel_counts/save_load_resume") > 2_000
        && json_u64_pointer(input, "/flow_pixel_counts/outcome_open_world") > 2_000
        && json_u64_pointer(input, "/flow_pixel_counts/lane") > 500
        && json_u64_pointer(input, "/flow_pixel_counts/highlight") > 1_000
        && json_u64_pointer(
            input,
            "/flow_pixel_counts/player_first_flow_view_non_background",
        ) > 300_000
        && json_u64_pointer(input, "/flow_pixel_counts/player_first_flow_view_frame") > 8_000
        && json_u64_pointer(input, "/flow_pixel_counts/player_first_flow_status_strip") > 10_000
        && json_u64_pointer(input, "/flow_pixel_counts/player_first_flow_stage_rail") > 50_000;
    let title_account_gate = json_bool_at(input, "title_account_gate");
    let match_setup_gate = json_bool_at(input, "match_setup_gate");
    let in_match_hud_gate = json_bool_at(input, "in_match_hud_gate");
    let command_feedback_gate = json_bool_at(input, "command_feedback_gate");
    let save_resume_gate = json_bool_at(input, "save_resume_gate");
    let outcome_open_world_gate = json_bool_at(input, "outcome_open_world_gate");
    let continuous_player_flow_chain_gate = title_account_gate
        && match_setup_gate
        && in_match_hud_gate
        && command_feedback_gate
        && save_resume_gate
        && outcome_open_world_gate
        && json_bool_at(input, "continuous_player_flow_chain_gate");
    let source_preview_gate = json_bool_at(input, "source_preview_gate");
    let preview_gate = json_bool_at(input, "preview_gate")
        && preview_width == 1600
        && preview_height == 900
        && json_string_equals(input, "preview_format", "ppm_p3_rgb")
        && pixel_gate;
    let player_first_continuous_flow_screen_gate =
        json_bool_at(input, "player_first_continuous_flow_screen_gate");
    let native_client_boundary_gate = json_bool_at(input, "native_client_boundary_gate");
    let runtime_screen_gate = json_bool_at(input, "runtime_screen_gate")
        && json_string_equals(
            input,
            "runtime_screen_mode",
            "player_runtime_continuous_player_flow_screen",
        )
        && input.get("evidence_board_only").and_then(Value::as_bool) == Some(false);
    let no_credit_boundary_gate = json_bool_at(
        input,
        "external_evidence_ignored_for_current_replication_pass",
    ) && !json_bool_at(input, "android_s5_real_device_claimed")
        && !json_bool_at(input, "public_launch_ready")
        && !json_bool_at(input, "production_ready_ui_claimed")
        && !json_bool_at(input, "screen_for_screen_openra_ui_claimed")
        && !json_bool_at(input, "openra_engine_port_claimed")
        && !json_bool_at(input, "warcraft_iii_asset_copied")
        && !json_bool_at(input, "openra_asset_copied")
        && !json_bool_at(input, "third_party_asset_copied");
    let continuous_player_flow_gate = json_bool_at(input, "continuous_player_flow_gate")
        && source_contract_gate
        && transition_sequence_gate
        && source_headline_gate
        && continuous_player_flow_chain_gate
        && source_preview_gate
        && preview_gate
        && player_first_continuous_flow_screen_gate
        && native_client_boundary_gate
        && runtime_screen_gate
        && no_credit_boundary_gate;
    let green = json_contract_is(
        input,
        "trillionnium_world_bevy_classic_rts_continuous_player_flow_v1",
    ) && continuous_player_flow_green
        && continuous_player_flow_gate;

    RtsContinuousPlayerFlowReview {
        contract_version: TRNM_RTS_EVIDENCE_CONTINUOUS_PLAYER_FLOW_REVIEW_CONTRACT.to_string(),
        green,
        continuous_player_flow_contract,
        continuous_player_flow_green,
        preview_width,
        preview_height,
        continuous_player_flow_step_count,
        shell_meta_surface_count,
        match_setup_map_id,
        match_setup_faction_id,
        hud_surface_count,
        hud_army_supply_used,
        interaction_surface_count,
        session_final_objective_status,
        session_open_world_state,
        campaign_outcome_open_world_state,
        campaign_continuity_restored_room_id,
        source_contract_gate,
        transition_sequence_gate,
        source_headline_gate,
        pixel_gate,
        title_account_gate,
        match_setup_gate,
        in_match_hud_gate,
        command_feedback_gate,
        save_resume_gate,
        outcome_open_world_gate,
        continuous_player_flow_chain_gate,
        source_preview_gate,
        preview_gate,
        player_first_continuous_flow_screen_gate,
        native_client_boundary_gate,
        runtime_screen_gate,
        no_credit_boundary_gate,
        continuous_player_flow_gate,
        input_path: "trnm-world-bevy continuous player-flow source JSON and pixel counts -> trnm-rts-evidence continuous player-flow review".to_string(),
        evidence_path: "trnm-rts-evidence continuous_player_flow_review -> Bevy continuous player-flow packet/readiness artifact".to_string(),
        source_of_truth: "The RTS evidence crate reviews the six-step continuous player flow from title/account through match setup, in-match HUD, command feedback, save/resume, and outcome/open-world return, while preserving player-first screen gates and S5/public/OpenRA/third-party no-credit boundaries before trnm-world-bevy includes the flow in playtest readiness.".to_string(),
    }
}

pub fn rts_live_session_playthrough_review(input: &Value) -> RtsLiveSessionPlaythroughReview {
    let live_session_playthrough_contract =
        json_string_at(input, "contract_version").unwrap_or_default();
    let live_session_playthrough_green = json_bool_at(input, "green");
    let preview_width = json_u64_at(input, "preview_width");
    let preview_height = json_u64_at(input, "preview_height");
    let stage_count = json_u64_at(input, "stage_count");
    let top_level_action_count = json_u64_at(input, "top_level_action_count");
    let top_level_accepted_action_count = json_u64_at(input, "top_level_accepted_action_count");
    let accepted_input_count = json_u64_at(input, "accepted_input_count");
    let campaign_handoff_input_count = json_u64_at(input, "campaign_handoff_input_count");
    let live_command_input_count = json_u64_at(input, "live_command_input_count");
    let slot_a_bytes = json_u64_at(input, "slot_a_bytes");
    let final_objective_status = json_string_pointer(input, "/final_state/objective_status");
    let final_open_world_handoff_state =
        json_string_pointer(input, "/final_state/open_world_handoff_state");
    let final_open_world_resume_room_id =
        json_string_pointer(input, "/final_state/open_world_resume_room_id");
    let expected_steps = [
        "title_account",
        "match_setup",
        "in_match_hud",
        "command_feedback",
        "save_load_resume",
        "outcome_open_world",
    ];
    let stage_ids_gate = input
        .pointer("/stage_ids")
        .and_then(Value::as_array)
        .is_some_and(|steps| {
            steps.len() == expected_steps.len()
                && steps
                    .iter()
                    .zip(expected_steps.iter())
                    .all(|(actual, expected)| actual.as_str() == Some(*expected))
        });
    let stage_summaries_gate = input
        .pointer("/stage_summaries")
        .and_then(Value::as_array)
        .is_some_and(|summaries| {
            summaries.len() == expected_steps.len()
                && summaries
                    .iter()
                    .zip(expected_steps.iter())
                    .all(|(summary, expected)| {
                        summary.get("step_id").and_then(Value::as_str) == Some(*expected)
                            && summary.get("turn").and_then(Value::as_u64).is_some()
                            && summary
                                .get("input_feedback_event_count")
                                .and_then(Value::as_u64)
                                .is_some()
                    })
        });
    let source_contract_gate = json_contract_is(
        input,
        "trillionnium_world_bevy_classic_rts_live_session_playthrough_v1",
    ) && live_session_playthrough_green;
    let stage_sequence_gate = stage_count == 6 && stage_ids_gate && stage_summaries_gate;
    let trace_sidecar_gate = json_bool_at(input, "trace_write_gate")
        && json_string_equals(input, "trace_seed", "classic_rts_live_session_seed_v1")
        && input
            .get("trace_path")
            .and_then(Value::as_str)
            .is_some_and(|path| {
                path.ends_with("bevy-classic-rts-live-session-playthrough.trace.json")
            });
    let same_process_trace_gate = json_bool_at(input, "same_process_trace_gate")
        && json_bool_at(input, "same_process_session_playthrough")
        && top_level_action_count >= 12
        && top_level_action_count == top_level_accepted_action_count
        && accepted_input_count >= 78
        && campaign_handoff_input_count >= 70
        && trace_sidecar_gate;
    let title_account_gate = json_bool_at(input, "title_account_gate");
    let match_setup_gate = json_bool_at(input, "match_setup_gate");
    let in_match_hud_gate = json_bool_at(input, "in_match_hud_gate");
    let command_feedback_gate = json_bool_at(input, "command_feedback_gate");
    let save_resume_gate = json_bool_at(input, "save_resume_gate") && slot_a_bytes > 10_000;
    let outcome_open_world_gate = json_bool_at(input, "outcome_open_world_gate");
    let live_command_gate = command_feedback_gate && live_command_input_count == 5;
    let final_state_gate = final_objective_status.as_deref()
        == Some("open_world_after_action_ready")
        && final_open_world_handoff_state.as_deref() == Some("resumed:league-coliseum")
        && final_open_world_resume_room_id.as_deref() == Some("league-coliseum")
        && json_string_pointer(input, "/final_state/current_room_id").as_deref()
            == Some("league-coliseum")
        && json_string_pointer(input, "/final_state/map_scene").as_deref()
            == Some("arena_league_coliseum")
        && json_string_pointer(input, "/final_state/contextual_primary_action_label").as_deref()
            == Some("COMBAT:attack");
    let player_first_live_session_screen_gate =
        json_bool_at(input, "player_first_live_session_screen_gate")
            && json_u64_pointer(input, "/pixel_counts/player_first_live_view_non_background")
                > 250_000
            && json_u64_pointer(input, "/pixel_counts/player_first_live_view_frame") > 8_000
            && json_u64_pointer(input, "/pixel_counts/player_first_live_status_strip") > 10_000
            && json_u64_pointer(input, "/pixel_counts/player_first_live_stage_rail") > 25_000;
    let pixel_gate = json_bool_at(input, "preview_gate")
        && preview_width == 1600
        && preview_height == 900
        && json_string_equals(input, "preview_format", "ppm_p3_rgb")
        && json_u64_pointer(input, "/pixel_counts/non_background") > 300_000
        && json_u64_pointer(input, "/pixel_counts/title_account") > 1_000
        && json_u64_pointer(input, "/pixel_counts/match_setup") > 1_000
        && json_u64_pointer(input, "/pixel_counts/in_match_hud") > 1_000
        && json_u64_pointer(input, "/pixel_counts/command_feedback") > 1_000
        && json_u64_pointer(input, "/pixel_counts/save_load_resume") > 1_000
        && json_u64_pointer(input, "/pixel_counts/outcome_open_world") > 1_000
        && player_first_live_session_screen_gate;
    let runtime_screen_gate = json_bool_at(input, "runtime_screen_gate")
        && json_string_equals(
            input,
            "runtime_screen_mode",
            "player_runtime_live_session_playthrough_screen",
        )
        && input.get("evidence_board_only").and_then(Value::as_bool) == Some(false)
        && same_process_trace_gate
        && pixel_gate;
    let native_client_boundary_gate = json_bool_at(input, "native_client_boundary_gate")
        && input
            .get("cex_runtime_player_client_allowed")
            .and_then(Value::as_bool)
            == Some(false)
        && input.get("wgpu_required").and_then(Value::as_bool) == Some(false);
    let no_credit_boundary_gate =
        json_bool_at(input, "external_evidence_ignored_for_current_playtest_pass")
            && !json_bool_at(input, "android_s5_real_device_claimed")
            && !json_bool_at(input, "public_launch_ready")
            && !json_bool_at(input, "production_ready_ui_claimed")
            && !json_bool_at(input, "screen_for_screen_openra_ui_claimed")
            && !json_bool_at(input, "openra_engine_port_claimed")
            && !json_bool_at(input, "warcraft_iii_asset_copied")
            && !json_bool_at(input, "openra_asset_copied")
            && !json_bool_at(input, "third_party_asset_copied");
    let live_session_playthrough_gate = json_bool_at(input, "live_session_playthrough_gate")
        && source_contract_gate
        && stage_sequence_gate
        && same_process_trace_gate
        && title_account_gate
        && match_setup_gate
        && in_match_hud_gate
        && live_command_gate
        && save_resume_gate
        && outcome_open_world_gate
        && final_state_gate
        && runtime_screen_gate
        && native_client_boundary_gate
        && no_credit_boundary_gate;
    let green = live_session_playthrough_gate;

    RtsLiveSessionPlaythroughReview {
        contract_version: TRNM_RTS_EVIDENCE_LIVE_SESSION_PLAYTHROUGH_REVIEW_CONTRACT.to_string(),
        green,
        live_session_playthrough_contract,
        live_session_playthrough_green,
        preview_width,
        preview_height,
        stage_count,
        top_level_action_count,
        top_level_accepted_action_count,
        accepted_input_count,
        campaign_handoff_input_count,
        live_command_input_count,
        slot_a_bytes,
        final_objective_status,
        final_open_world_handoff_state,
        final_open_world_resume_room_id,
        source_contract_gate,
        stage_sequence_gate,
        same_process_trace_gate,
        title_account_gate,
        match_setup_gate,
        in_match_hud_gate,
        command_feedback_gate,
        save_resume_gate,
        outcome_open_world_gate,
        live_command_gate,
        final_state_gate,
        pixel_gate,
        trace_sidecar_gate,
        player_first_live_session_screen_gate,
        runtime_screen_gate,
        native_client_boundary_gate,
        no_credit_boundary_gate,
        live_session_playthrough_gate,
        input_path: "trnm-world-bevy live-session playthrough trace/source JSON and player-first pixels -> trnm-rts-evidence live-session playthrough review".to_string(),
        evidence_path: "trnm-rts-evidence live_session_playthrough_review -> Bevy live-session playthrough packet/readiness artifact".to_string(),
        source_of_truth: "The RTS evidence crate reviews the same-process local live session playthrough from title/account through campaign start, in-match HUD, live command feedback, slot A save/load/resume, and open-world outcome, including trace sidecar, player-first tactical screen, native-client boundary, and S5/public/OpenRA/third-party no-credit boundaries.".to_string(),
    }
}

pub fn rts_full_game_visual_ui_replication_review(
    input: &Value,
) -> RtsFullGameVisualUiReplicationReview {
    let full_game_visual_ui_replication_contract =
        json_string_at(input, "contract_version").unwrap_or_default();
    let full_game_visual_ui_replication_green = json_bool_at(input, "green");
    let preview_width = json_u64_at(input, "preview_width");
    let preview_height = json_u64_at(input, "preview_height");
    let coverage_surface_count = json_u64_at(input, "coverage_surface_count");
    let runtime_screen_mode = json_string_at(input, "runtime_screen_mode");
    let session_state_review_contract =
        json_string_pointer(input, "/source_review_contracts/session_state_continuity");
    let continuous_player_flow_review_contract =
        json_string_pointer(input, "/source_review_contracts/continuous_player_flow");
    let live_session_playthrough_review_contract =
        json_string_pointer(input, "/source_review_contracts/live_session_playthrough");
    let full_screen_surface_count =
        json_u64_pointer(input, "/source_headline/full_screen_surface_count");
    let shell_meta_surface_count =
        json_u64_pointer(input, "/source_headline/shell_meta_surface_count");
    let match_setup_surface_count =
        json_u64_pointer(input, "/source_headline/match_setup_surface_count");
    let hud_surface_count = json_u64_pointer(input, "/source_headline/hud_surface_count");
    let continuous_step_count = json_u64_pointer(input, "/source_headline/continuous_step_count");
    let live_session_stage_count =
        json_u64_pointer(input, "/source_headline/live_session_stage_count");
    let live_session_accepted_input_count =
        json_u64_pointer(input, "/source_headline/live_session_accepted_input_count");
    let live_session_final_objective_status = json_string_pointer(
        input,
        "/source_headline/live_session_final_objective_status",
    );
    let live_session_open_world_state =
        json_string_pointer(input, "/source_headline/live_session_open_world_state");

    let source_contract_gate = json_contract_is(
        input,
        "trillionnium_world_bevy_classic_rts_full_game_visual_ui_replication_v1",
    ) && full_game_visual_ui_replication_green
        && json_string_pointer(input, "/source_contracts/visual_fidelity").as_deref()
            == Some("trillionnium_world_bevy_classic_rts_visual_fidelity_v1")
        && json_string_pointer(input, "/source_contracts/production_art_replication").as_deref()
            == Some("trillionnium_world_bevy_classic_rts_production_art_replication_v1")
        && json_string_pointer(input, "/source_contracts/production_asset_atlas").as_deref()
            == Some("trillionnium_world_bevy_classic_rts_production_asset_atlas_v1")
        && json_string_pointer(input, "/source_contracts/production_ui_skin").as_deref()
            == Some("trillionnium_world_bevy_classic_rts_production_ui_skin_v1")
        && json_string_pointer(input, "/source_contracts/production_interaction_polish").as_deref()
            == Some("trillionnium_world_bevy_classic_rts_production_interaction_polish_v1")
        && json_string_pointer(input, "/source_contracts/full_screen_ui_replication").as_deref()
            == Some("trillionnium_world_bevy_classic_rts_full_screen_ui_replication_v1")
        && json_string_pointer(input, "/source_contracts/shell_meta_ui_replication").as_deref()
            == Some("trillionnium_world_bevy_classic_rts_shell_meta_ui_replication_v1")
        && json_string_pointer(input, "/source_contracts/match_setup_ui_replication").as_deref()
            == Some("trillionnium_world_bevy_classic_rts_match_setup_ui_replication_v1")
        && json_string_pointer(input, "/source_contracts/in_match_hud_state_replication")
            .as_deref()
            == Some("trillionnium_world_bevy_classic_rts_in_match_hud_state_replication_v1")
        && json_string_pointer(input, "/source_contracts/session_state_continuity").as_deref()
            == Some("trillionnium_world_bevy_classic_rts_session_state_continuity_v1")
        && json_string_pointer(input, "/source_contracts/continuous_player_flow").as_deref()
            == Some("trillionnium_world_bevy_classic_rts_continuous_player_flow_v1")
        && json_string_pointer(input, "/source_contracts/live_session_playthrough").as_deref()
            == Some("trillionnium_world_bevy_classic_rts_live_session_playthrough_v1")
        && json_string_pointer(input, "/source_contracts/command_surface").as_deref()
            == Some("trillionnium_world_bevy_classic_rts_command_surface_v1")
        && json_string_pointer(input, "/source_contracts/command_affordance").as_deref()
            == Some("trillionnium_world_bevy_classic_rts_command_affordance_v1");
    let source_green_gate = json_bool_at(input, "source_green_gate");
    let source_review_gate = session_state_review_contract.as_deref()
        == Some(TRNM_RTS_EVIDENCE_SESSION_STATE_CONTINUITY_REVIEW_CONTRACT)
        && continuous_player_flow_review_contract.as_deref()
            == Some(TRNM_RTS_EVIDENCE_CONTINUOUS_PLAYER_FLOW_REVIEW_CONTRACT)
        && live_session_playthrough_review_contract.as_deref()
            == Some(TRNM_RTS_EVIDENCE_LIVE_SESSION_PLAYTHROUGH_REVIEW_CONTRACT)
        && json_bool_pointer(input, "/source_review_gates/session_state_continuity")
        && json_bool_pointer(input, "/source_review_gates/continuous_player_flow")
        && json_bool_pointer(input, "/source_review_gates/live_session_playthrough")
        && json_string_pointer(input, "/source_review_sources/session_state_continuity")
            .is_some_and(|source| source.contains("save-slot confirmation"))
        && json_string_pointer(input, "/source_review_sources/continuous_player_flow")
            .is_some_and(|source| source.contains("six-step continuous player flow"))
        && json_string_pointer(input, "/source_review_sources/live_session_playthrough")
            .is_some_and(|source| source.contains("same-process local live session playthrough"));
    let coverage_surface_gate = coverage_surface_count == 18
        && json_array_contains(input, "/coverage_surface_names", "title_account_shell")
        && json_array_contains(input, "/coverage_surface_names", "match_setup_start")
        && json_array_contains(input, "/coverage_surface_names", "tactical_viewport")
        && json_array_contains(input, "/coverage_surface_names", "map_minimap_camera")
        && json_array_contains(input, "/coverage_surface_names", "command_grid")
        && json_array_contains(input, "/coverage_surface_names", "session_slot_save_load")
        && json_array_contains(input, "/coverage_surface_names", "open_world_handoff")
        && json_bool_at(input, "coverage_surface_gate");
    let source_headline_gate = full_screen_surface_count == 10
        && shell_meta_surface_count == 12
        && match_setup_surface_count == 10
        && hud_surface_count == 8
        && continuous_step_count == 6
        && live_session_stage_count == 6
        && live_session_accepted_input_count >= 78
        && live_session_final_objective_status.as_deref() == Some("open_world_after_action_ready")
        && live_session_open_world_state.as_deref() == Some("resumed:league-coliseum")
        && json_string_pointer(input, "/source_headline/live_session_runtime_screen_mode")
            .as_deref()
            == Some("player_runtime_live_session_playthrough_screen")
        && json_bool_pointer(input, "/source_headline/live_session_runtime_screen_gate")
        && json_string_pointer(input, "/source_headline/production_ui_runtime_screen_mode")
            .as_deref()
            == Some("player_runtime_production_hud_skin_screen")
        && json_string_pointer(input, "/source_headline/interaction_runtime_screen_mode")
            .as_deref()
            == Some("player_runtime_command_interaction_screen")
        && json_u64_pointer(input, "/source_headline/command_surface_ready_pixel_count") > 1_000
        && json_u64_pointer(
            input,
            "/source_headline/command_affordance_hotkey_pixel_count",
        ) > 3_000;
    let player_flow_gate = json_bool_at(input, "player_flow_gate")
        && continuous_step_count == 6
        && live_session_stage_count == 6
        && live_session_accepted_input_count >= 78
        && live_session_final_objective_status.as_deref() == Some("open_world_after_action_ready")
        && live_session_open_world_state.as_deref() == Some("resumed:league-coliseum");
    let player_first_tactical_composition_gate =
        json_bool_at(input, "player_first_tactical_composition_gate")
            && json_u64_pointer(
                input,
                "/pixel_counts/player_first_tactical_preview_non_background",
            ) > 350_000
            && json_u64_pointer(input, "/pixel_counts/player_first_tactical_viewport_frame")
                > 8_000
            && json_u64_pointer(input, "/pixel_counts/player_first_tactical_status_strip") > 10_000;
    let pixel_gate = json_u64_pointer(input, "/pixel_counts/non_background") > 900_000
        && json_u64_pointer(input, "/pixel_counts/hud_chrome") > 120_000
        && json_u64_pointer(input, "/pixel_counts/shell_session") > 8_000
        && json_u64_pointer(input, "/pixel_counts/match_setup") > 5_000
        && json_u64_pointer(input, "/pixel_counts/hud") > 500
        && json_u64_pointer(input, "/pixel_counts/command") > 20_000
        && json_u64_pointer(input, "/pixel_counts/session") > 10_000
        && json_u64_pointer(input, "/pixel_counts/outcome") > 10_000
        && json_u64_pointer(input, "/pixel_counts/tech") > 1_000
        && json_u64_pointer(input, "/pixel_counts/minimap") > 1_000
        && json_u64_pointer(input, "/pixel_counts/highlight") > 4_000
        && player_first_tactical_composition_gate;
    let preview_gate = json_bool_at(input, "preview_gate")
        && preview_width == 1920
        && preview_height == 1080
        && json_string_equals(input, "preview_format", "ppm_p3_rgb")
        && pixel_gate;
    let runtime_screen_chain_gate = json_bool_at(input, "runtime_screen_chain_gate");
    let runtime_screen_gate = json_bool_at(input, "runtime_screen_gate")
        && runtime_screen_mode.as_deref() == Some("player_runtime_full_game_visual_ui_screen")
        && input.get("evidence_board_only").and_then(Value::as_bool) == Some(false)
        && runtime_screen_chain_gate
        && preview_gate;
    let player_first_full_game_visual_ui_screen_gate =
        json_bool_at(input, "player_first_full_game_visual_ui_screen_gate")
            && runtime_screen_gate
            && player_flow_gate
            && coverage_surface_gate
            && source_review_gate;
    let no_copy_boundary_gate = json_bool_at(
        input,
        "external_evidence_ignored_for_current_replication_pass",
    ) && !json_bool_at(input, "android_s5_real_device_claimed")
        && !json_bool_at(input, "public_launch_ready")
        && !json_bool_at(input, "production_ready_ui_claimed")
        && !json_bool_at(input, "screen_for_screen_openra_ui_claimed")
        && !json_bool_at(input, "openra_engine_port_claimed")
        && !json_bool_at(input, "warcraft_iii_asset_copied")
        && !json_bool_at(input, "openra_asset_copied")
        && !json_bool_at(input, "third_party_asset_copied");
    let full_game_visual_ui_replication_gate =
        json_bool_at(input, "full_game_visual_ui_replication_gate")
            && source_contract_gate
            && source_green_gate
            && source_review_gate
            && coverage_surface_gate
            && source_headline_gate
            && player_flow_gate
            && preview_gate
            && runtime_screen_gate
            && player_first_full_game_visual_ui_screen_gate
            && no_copy_boundary_gate;
    let green = full_game_visual_ui_replication_gate;

    RtsFullGameVisualUiReplicationReview {
        contract_version: TRNM_RTS_EVIDENCE_FULL_GAME_VISUAL_UI_REPLICATION_REVIEW_CONTRACT
            .to_string(),
        green,
        full_game_visual_ui_replication_contract,
        full_game_visual_ui_replication_green,
        preview_width,
        preview_height,
        coverage_surface_count,
        runtime_screen_mode,
        session_state_review_contract,
        continuous_player_flow_review_contract,
        live_session_playthrough_review_contract,
        full_screen_surface_count,
        shell_meta_surface_count,
        match_setup_surface_count,
        hud_surface_count,
        continuous_step_count,
        live_session_stage_count,
        live_session_accepted_input_count,
        live_session_final_objective_status,
        live_session_open_world_state,
        source_contract_gate,
        source_green_gate,
        source_review_gate,
        coverage_surface_gate,
        source_headline_gate,
        player_flow_gate,
        pixel_gate,
        preview_gate,
        runtime_screen_chain_gate,
        runtime_screen_gate,
        player_first_tactical_composition_gate,
        player_first_full_game_visual_ui_screen_gate,
        no_copy_boundary_gate,
        full_game_visual_ui_replication_gate,
        input_path: "trnm-world-bevy full-game visual/UI aggregate JSON, source review contracts, and player-first pixels -> trnm-rts-evidence full-game visual/UI replication review".to_string(),
        evidence_path: "trnm-rts-evidence full_game_visual_ui_replication_review -> Bevy full-game visual/UI packet/readiness artifact".to_string(),
        source_of_truth: "The RTS evidence crate reviews the local Rust/Bevy full-game visual/UI replication aggregate across source contracts, nested evidence reviews, 18 coverage surfaces, live-session/open-world handoff, player-first tactical pixels, runtime screen chain, and S5/public/OpenRA/third-party no-credit boundaries.".to_string(),
    }
}

pub fn rts_openra_style_screen_set_review(input: &Value) -> RtsOpenraStyleScreenSetReview {
    let openra_screen_for_screen_ui_replication_contract =
        json_string_at(input, "contract_version").unwrap_or_default();
    let openra_screen_for_screen_ui_replication_green = json_bool_at(input, "green");
    let preview_width = json_u64_at(input, "preview_width");
    let preview_height = json_u64_at(input, "preview_height");
    let screen_for_screen_mode = json_string_at(input, "screen_for_screen_mode");
    let runtime_screen_mode = json_string_at(input, "runtime_screen_mode");
    let openra_widget_root_count = json_u64_at(input, "openra_widget_root_count");
    let openra_reference_screen_count = json_u64_at(input, "openra_reference_screen_count");
    let replicated_interaction_surface_count =
        json_u64_at(input, "replicated_interaction_surface_count");
    let full_game_surface_count =
        json_u64_pointer(input, "/source_headline/full_game_surface_count");
    let full_screen_surface_count =
        json_u64_pointer(input, "/source_headline/full_screen_surface_count");
    let shell_meta_surface_count =
        json_u64_pointer(input, "/source_headline/shell_meta_surface_count");
    let match_setup_surface_count =
        json_u64_pointer(input, "/source_headline/match_setup_surface_count");
    let hud_surface_count = json_u64_pointer(input, "/source_headline/hud_surface_count");
    let session_surface_count = json_u64_pointer(input, "/source_headline/session_surface_count");
    let openra_parity_lane_axis_count =
        json_u64_pointer(input, "/source_headline/openra_parity_lane_axis_count");

    let source_contract_gate = json_contract_is(
        input,
        "trillionnium_world_bevy_classic_rts_openra_screen_for_screen_ui_replication_v1",
    ) && openra_screen_for_screen_ui_replication_green
        && json_string_pointer(input, "/source_contracts/full_game_visual_ui_replication")
            .as_deref()
            == Some("trillionnium_world_bevy_classic_rts_full_game_visual_ui_replication_v1")
        && json_string_pointer(input, "/source_contracts/full_screen_ui_replication").as_deref()
            == Some("trillionnium_world_bevy_classic_rts_full_screen_ui_replication_v1")
        && json_string_pointer(input, "/source_contracts/shell_meta_ui_replication").as_deref()
            == Some("trillionnium_world_bevy_classic_rts_shell_meta_ui_replication_v1")
        && json_string_pointer(input, "/source_contracts/match_setup_ui_replication").as_deref()
            == Some("trillionnium_world_bevy_classic_rts_match_setup_ui_replication_v1")
        && json_string_pointer(input, "/source_contracts/in_match_hud_state_replication")
            .as_deref()
            == Some("trillionnium_world_bevy_classic_rts_in_match_hud_state_replication_v1")
        && json_string_pointer(input, "/source_contracts/session_state_continuity").as_deref()
            == Some("trillionnium_world_bevy_classic_rts_session_state_continuity_v1")
        && json_string_pointer(input, "/source_contracts/openra_like_core").as_deref()
            == Some("trillionnium_world_bevy_classic_rts_openra_like_core_v1")
        && json_string_pointer(input, "/source_contracts/openra_parity_lane").as_deref()
            == Some("trillionnium_world_bevy_classic_rts_openra_parity_lane_v1")
        && json_bool_at(input, "source_contract_gate");
    let source_green_gate = json_bool_at(input, "source_green_gate");
    let openra_runtime_vocabulary_gate = json_bool_at(input, "openra_runtime_vocabulary_gate")
        && json_string_pointer(input, "/source_headline/openra_like_runtime_model").as_deref()
            == Some("rust_bevy_owned_openra_like_rts_core")
        && openra_parity_lane_axis_count == 6;
    let widget_root_reference_gate = json_bool_at(input, "widget_root_reference_gate")
        && openra_widget_root_count == 4
        && json_array_contains(input, "/openra_widget_roots", "ShellmapRoot=MAINMENU")
        && json_array_contains(input, "/openra_widget_roots", "IngameRoot=INGAME_ROOT")
        && json_array_contains(
            input,
            "/openra_widget_roots",
            "GameSaveLoadingRoot=GAMESAVE_LOADING_SCREEN",
        )
        && json_array_contains(input, "/openra_widget_roots", "EditorRoot=EDITOR_ROOT");
    let screen_set_gate = json_bool_at(input, "screen_set_gate")
        && openra_reference_screen_count == 8
        && replicated_interaction_surface_count == 8
        && json_array_contains(input, "/openra_reference_screens", "MAINMENU_shellmap_root")
        && json_array_contains(
            input,
            "/openra_reference_screens",
            "SKIRMISH_mission_browser",
        )
        && json_array_contains(
            input,
            "/openra_reference_screens",
            "MULTIPLAYER_server_browser",
        )
        && json_array_contains(input, "/openra_reference_screens", "LOBBY_setup_room")
        && json_array_contains(
            input,
            "/openra_reference_screens",
            "LOADING_briefing_progress",
        )
        && json_array_contains(
            input,
            "/openra_reference_screens",
            "INGAME_ROOT_sidebar_hud",
        )
        && json_array_contains(input, "/openra_reference_screens", "PAUSE_options_overlay")
        && json_array_contains(input, "/openra_reference_screens", "POSTGAME_statistics")
        && json_array_contains(
            input,
            "/replicated_interaction_surfaces",
            "shellmap_menu_stack",
        )
        && json_array_contains(
            input,
            "/replicated_interaction_surfaces",
            "mission_map_list",
        )
        && json_array_contains(
            input,
            "/replicated_interaction_surfaces",
            "server_filter_table",
        )
        && json_array_contains(
            input,
            "/replicated_interaction_surfaces",
            "lobby_player_slots",
        )
        && json_array_contains(
            input,
            "/replicated_interaction_surfaces",
            "loading_briefing_progress",
        )
        && json_array_contains(
            input,
            "/replicated_interaction_surfaces",
            "ingame_viewport_sidebar_minimap",
        )
        && json_array_contains(
            input,
            "/replicated_interaction_surfaces",
            "pause_settings_overlay",
        )
        && json_array_contains(
            input,
            "/replicated_interaction_surfaces",
            "postgame_score_tabs",
        );
    let source_screen_chain_gate = json_bool_at(input, "source_screen_chain_gate")
        && full_game_surface_count == 18
        && json_bool_pointer(input, "/source_headline/full_game_internal_claimed")
        && full_screen_surface_count == 10
        && shell_meta_surface_count == 12
        && match_setup_surface_count == 10
        && hud_surface_count == 8
        && session_surface_count == 8;
    let pixel_gate = json_u64_pointer(input, "/pixel_counts/non_background") > 1_200_000
        && json_u64_pointer(input, "/pixel_counts/mainmenu") > 8_000
        && json_u64_pointer(input, "/pixel_counts/skirmish") > 8_000
        && json_u64_pointer(input, "/pixel_counts/server_browser") > 8_000
        && json_u64_pointer(input, "/pixel_counts/lobby") > 8_000
        && json_u64_pointer(input, "/pixel_counts/loading") > 8_000
        && json_u64_pointer(input, "/pixel_counts/ingame") > 8_000
        && json_u64_pointer(input, "/pixel_counts/pause") > 8_000
        && json_u64_pointer(input, "/pixel_counts/postgame_stats") > 8_000
        && json_u64_pointer(input, "/pixel_counts/active_highlight") > 6_000
        && json_u64_pointer(
            input,
            "/openra_style_ingame_pixel_counts/player_first_openra_style_ingame_view_non_background",
        ) > 70_000
        && json_u64_pointer(
            input,
            "/openra_style_ingame_pixel_counts/player_first_openra_style_ingame_sidebar_non_background",
        ) > 30_000
        && json_u64_pointer(
            input,
            "/openra_style_ingame_pixel_counts/player_first_openra_style_ingame_command_lane_non_background",
        ) > 5_000
        && json_u64_pointer(
            input,
            "/openra_style_ingame_pixel_counts/player_first_openra_style_ingame_control_color",
        ) > 30_000
        && json_u64_pointer(
            input,
            "/openra_style_ingame_pixel_counts/player_first_openra_style_active_highlight",
        ) > 6_000;
    let preview_gate = json_bool_at(input, "preview_gate")
        && preview_width == 1920
        && preview_height == 1080
        && json_string_equals(input, "preview_format", "ppm_p3_rgb")
        && pixel_gate;
    let runtime_screen_gate = json_bool_at(input, "runtime_screen_gate")
        && screen_for_screen_mode.as_deref()
            == Some(
                "openra_style_widget_root_screen_set_and_interaction_surface_replication_original_trillionnium_art",
            )
        && runtime_screen_mode.as_deref() == Some("player_runtime_openra_style_ingame_screen_set")
        && input.get("evidence_board_only").and_then(Value::as_bool) == Some(false)
        && preview_gate;
    let player_first_openra_style_ingame_screen_gate =
        json_bool_at(input, "player_first_openra_style_ingame_screen_gate")
            && runtime_screen_gate
            && pixel_gate;
    let no_asset_copy_boundary_gate = json_bool_at(input, "no_asset_copy_boundary_gate")
        && !json_bool_at(input, "openra_asset_copied")
        && !json_bool_at(input, "warcraft_iii_asset_copied")
        && !json_bool_at(input, "third_party_asset_copied")
        && !json_bool_at(input, "openra_engine_port_claimed")
        && !json_bool_at(input, "openra_pixel_perfect_asset_parity_claimed");
    let openra_style_widget_root_screen_set_claimed =
        json_bool_at(input, "openra_style_widget_root_screen_set_claimed");
    let no_credit_boundary_gate = openra_style_widget_root_screen_set_claimed
        && !json_bool_at(input, "screen_for_screen_openra_ui_claimed")
        && !json_bool_at(input, "openra_screen_for_screen_ui_replication_claimed")
        && !json_bool_at(input, "openra_pixel_perfect_asset_parity_claimed")
        && !json_bool_at(input, "openra_engine_port_claimed")
        && !json_bool_at(input, "openra_asset_copied")
        && !json_bool_at(input, "warcraft_iii_asset_copied")
        && !json_bool_at(input, "third_party_asset_copied")
        && !json_bool_at(input, "bevy_openra_runtime_parity_claimed")
        && !json_bool_at(input, "bevy_openra_replay_file_claimed")
        && !json_bool_at(input, "android_s5_real_device_claimed")
        && !json_bool_at(input, "public_launch_ready");
    let openra_style_ui_screen_set_replication_gate =
        json_bool_at(input, "openra_style_ui_screen_set_replication_gate")
            && source_contract_gate
            && source_green_gate
            && openra_runtime_vocabulary_gate
            && widget_root_reference_gate
            && screen_set_gate
            && source_screen_chain_gate
            && preview_gate
            && runtime_screen_gate
            && player_first_openra_style_ingame_screen_gate
            && no_asset_copy_boundary_gate
            && no_credit_boundary_gate;
    let openra_screen_for_screen_ui_replication_gate =
        json_bool_at(input, "openra_screen_for_screen_ui_replication_gate")
            && openra_style_ui_screen_set_replication_gate;
    let green = openra_screen_for_screen_ui_replication_gate;

    RtsOpenraStyleScreenSetReview {
        contract_version: TRNM_RTS_EVIDENCE_OPENRA_STYLE_SCREEN_SET_REVIEW_CONTRACT.to_string(),
        green,
        openra_screen_for_screen_ui_replication_contract,
        openra_screen_for_screen_ui_replication_green,
        preview_width,
        preview_height,
        screen_for_screen_mode,
        runtime_screen_mode,
        openra_widget_root_count,
        openra_reference_screen_count,
        replicated_interaction_surface_count,
        full_game_surface_count,
        full_screen_surface_count,
        shell_meta_surface_count,
        match_setup_surface_count,
        hud_surface_count,
        session_surface_count,
        openra_parity_lane_axis_count,
        source_contract_gate,
        source_green_gate,
        openra_runtime_vocabulary_gate,
        widget_root_reference_gate,
        screen_set_gate,
        source_screen_chain_gate,
        pixel_gate,
        preview_gate,
        runtime_screen_gate,
        player_first_openra_style_ingame_screen_gate,
        no_asset_copy_boundary_gate,
        no_credit_boundary_gate,
        openra_style_ui_screen_set_replication_gate,
        openra_screen_for_screen_ui_replication_gate,
        openra_style_widget_root_screen_set_claimed,
        input_path: "trnm-world-bevy OpenRA-style widget root/screen-set JSON and player-first pixels -> trnm-rts-evidence OpenRA-style screen-set review".to_string(),
        evidence_path: "trnm-rts-evidence openra_style_screen_set_review -> Bevy OpenRA-style screen-set packet/readiness artifact".to_string(),
        source_of_truth: "The RTS evidence crate reviews the OpenRA-style screen-set UI replication boundary: widget roots, eight reference screens, interaction surfaces, player-first ingame HUD pixels, source artifact chain, and no-copy/no-overclaim boundaries while keeping OpenRA screen-for-screen parity, engine-port, pixel-perfect asset parity, S5, public-launch, and third-party asset-copy claims false.".to_string(),
    }
}

pub fn rts_release_review_packet_assembly_review(
    packet: &Value,
) -> RtsReleaseReviewPacketAssemblyReview {
    let artifacts = packet
        .get("artifacts")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let packet_contract = json_string_at(packet, "contract_version").unwrap_or_default();
    let packet_status = json_string_at(packet, "status").unwrap_or_default();
    let artifact_count = artifacts.len() as u64;
    let release_review_input_count = artifact_role_count(&artifacts, "release_review_input");
    let release_review_visual_evidence_count =
        artifact_role_count(&artifacts, "release_review_visual_evidence");
    let release_review_recording_count =
        artifact_role_count(&artifacts, "release_review_recording");
    let release_review_collection_count =
        artifact_role_count(&artifacts, "release_review_collection");
    let release_review_gate_count = artifact_role_count(&artifacts, "release_review_gate");
    let release_review_operator_handoff_count =
        artifact_role_count(&artifacts, "release_review_operator_handoff");
    let release_review_checkpoint_count =
        artifact_role_count(&artifacts, "release_review_checkpoint");
    let release_review_checklist_count =
        artifact_role_count(&artifacts, "release_review_checklist");
    let release_review_log_count = artifact_role_count(&artifacts, "release_review_log");
    let missing_artifact_count = json_array_len_at(packet, "missing_artifacts");
    let ready_item_count = json_array_len_at(packet, "ready_items");
    let blocked_item_count = json_array_len_at(packet, "blocked_items");

    let reviewed_runtime_artifact_ids = [
        "native_bevy_classic_rts_first_contact_basin_spec",
        "native_bevy_classic_rts_campaign_ui_continuity",
        "native_bevy_classic_rts_campaign_ui_continuity_ppm",
        "native_bevy_classic_rts_in_match_hud_state_replication",
        "native_bevy_classic_rts_session_state_continuity",
        "native_bevy_classic_rts_combat_readability_pressure_readiness",
        "native_bevy_classic_rts_full_game_visual_ui_replication",
        "native_bevy_classic_rts_full_game_visual_ui_replication_ppm",
        "native_bevy_classic_playtest_readiness",
        "native_bevy_classic_playtest_runner_status",
        "native_bevy_classic_playtest_launcher",
        "native_bevy_classic_playtest_handoff_packet",
        "release_review_convergence",
        "release_review_checkpoint_manifest",
        "release_review_status_json",
        "release_review_quickcheck",
    ]
    .iter()
    .map(|id| (*id).to_string())
    .collect::<Vec<_>>();
    let reviewed_packet_fixture_ids = [
        "release_review_packet_integrity_semantic_fixture",
        "release_review_packet_integrity_bot_executor_semantic_fixture",
        "release_review_packet_integrity_bot_executor_matrix_semantic_fixture",
        "release_review_packet_integrity_bot_gap_semantic_fixture",
        "release_review_packet_integrity_control_loop_semantic_fixture",
        "release_review_packet_integrity_selection_minimap_semantic_fixture",
        "release_review_packet_integrity_build_lifecycle_semantic_fixture",
        "release_review_packet_integrity_tech_tree_semantic_fixture",
        "release_review_packet_integrity_projectile_ability_semantic_fixture",
    ]
    .iter()
    .map(|id| (*id).to_string())
    .collect::<Vec<_>>();

    let inventory_summary_gate = json_u64_at(packet, "artifact_count") == artifact_count
        && json_u64_at(packet, "release_review_input_count") == release_review_input_count
        && json_u64_at(packet, "release_review_visual_evidence_count")
            == release_review_visual_evidence_count
        && json_u64_at(packet, "release_review_recording_count") == release_review_recording_count
        && json_u64_at(packet, "release_review_collection_count")
            == release_review_collection_count
        && json_u64_at(packet, "release_review_gate_count") == release_review_gate_count
        && json_u64_at(packet, "release_review_operator_handoff_count")
            == release_review_operator_handoff_count
        && json_u64_at(packet, "release_review_checkpoint_count")
            == release_review_checkpoint_count
        && json_u64_at(packet, "release_review_checklist_count") == release_review_checklist_count
        && json_u64_at(packet, "release_review_log_count") == release_review_log_count
        && json_u64_at(packet, "missing_artifact_count") == missing_artifact_count
        && json_u64_at(packet, "reviewed_runtime_artifact_count")
            == reviewed_runtime_artifact_ids.len() as u64
        && json_u64_at(packet, "reviewed_packet_fixture_count")
            == reviewed_packet_fixture_ids.len() as u64;
    let artifact_manifest_gate = artifact_count >= 120
        && artifacts
            .iter()
            .all(artifact_present_with_manifest_metadata);
    let missing_artifacts_gate = missing_artifact_count == 0
        && packet
            .get("missing_artifacts")
            .and_then(Value::as_array)
            .is_some_and(Vec::is_empty);
    let release_review_readiness_gate = packet_contract
        == "trillionnium_world_release_review_packet_v1"
        && json_bool_at(packet, "ready_for_release_review")
        && matches!(
            packet_status.as_str(),
            "release_review_packet_ready_with_public_launch_blockers"
                | "release_review_packet_green"
        );
    let status_handoff_gate = json_string_at(packet, "convergence_status")
        .is_some_and(|status| status.contains("release_review_convergence_green"))
        && matches!(
            json_string_at(packet, "status_checklist_status").as_deref(),
            Some("release_review_ready_public_launch_blocked" | "release_review_ready")
        )
        && ready_item_count >= 13;
    let key_runtime_artifacts_gate = reviewed_runtime_artifact_ids
        .iter()
        .all(|id| artifacts_have_id(&artifacts, id));
    let packet_integrity_fixture_count = reviewed_packet_fixture_ids
        .iter()
        .filter(|id| artifacts_have_id_with_role(&artifacts, id, "release_review_gate"))
        .count() as u64;
    let reviewed_runtime_artifact_count = reviewed_runtime_artifact_ids
        .iter()
        .filter(|id| artifacts_have_id(&artifacts, id))
        .count() as u64;
    let reviewed_packet_fixture_count = packet_integrity_fixture_count;
    let full_game_visual_ui_handoff_gate = artifacts_have_id_contract_status(
        &artifacts,
        "native_bevy_classic_rts_full_game_visual_ui_replication",
        "trillionnium_world_bevy_classic_rts_full_game_visual_ui_replication_v1",
        "classic_rts_full_game_visual_ui_replication_green",
    ) && artifacts_have_id_with_role(
        &artifacts,
        "native_bevy_classic_rts_full_game_visual_ui_replication_ppm",
        "release_review_visual_evidence",
    );
    let packet_integrity_fixture_gate =
        packet_integrity_fixture_count == reviewed_packet_fixture_ids.len() as u64;
    let public_launch_boundary_gate = !json_bool_at(packet, "public_launch_ready")
        && !json_bool_at(packet, "android_s5_real_device_claimed")
        && json_string_equals(
            packet,
            "proof_scope",
            "host_side_bevy_runtime_replay_not_android_real_device",
        );
    let external_blocker_gate = blocked_item_count == 6
        && matches!(
            json_string_at(packet, "reviewer_next_action").as_deref(),
            Some("collect_real_external_public_launch_evidence")
        );
    let green = inventory_summary_gate
        && artifact_manifest_gate
        && missing_artifacts_gate
        && release_review_readiness_gate
        && status_handoff_gate
        && key_runtime_artifacts_gate
        && full_game_visual_ui_handoff_gate
        && packet_integrity_fixture_gate
        && public_launch_boundary_gate
        && external_blocker_gate;

    RtsReleaseReviewPacketAssemblyReview {
        contract_version: TRNM_RTS_EVIDENCE_RELEASE_REVIEW_PACKET_ASSEMBLY_REVIEW_CONTRACT
            .to_string(),
        green,
        packet_contract,
        packet_status,
        artifact_count,
        release_review_input_count,
        release_review_visual_evidence_count,
        release_review_recording_count,
        release_review_collection_count,
        release_review_gate_count,
        release_review_operator_handoff_count,
        release_review_checkpoint_count,
        release_review_checklist_count,
        release_review_log_count,
        missing_artifact_count,
        packet_integrity_fixture_count,
        reviewed_runtime_artifact_count,
        reviewed_packet_fixture_count,
        ready_item_count,
        blocked_item_count,
        inventory_summary_gate,
        artifact_manifest_gate,
        missing_artifacts_gate,
        release_review_readiness_gate,
        status_handoff_gate,
        key_runtime_artifacts_gate,
        full_game_visual_ui_handoff_gate,
        packet_integrity_fixture_gate,
        public_launch_boundary_gate,
        external_blocker_gate,
        reviewed_runtime_artifact_ids,
        reviewed_packet_fixture_ids,
        input_path: "release-review packet manifest artifacts/status/checklist/blockers -> trnm-rts-evidence release review packet assembly review".to_string(),
        evidence_path: "trnm-rts-evidence release_review_packet_assembly_review -> release-review packet handoff artifact".to_string(),
        source_of_truth: "The RTS evidence crate reviews release-review packet assembly semantics after the shell manifest has gathered checksummed artifacts: top-level inventory summary, manifest completeness, missing-artifact state, status handoff, key Bevy RTS runtime artifacts including full-game visual/UI handoff, packet semantic fixtures, public-launch no-credit boundary, and six external evidence blockers.".to_string(),
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
    pub first_contact_player_screen_application: RtsFirstContactPlayerScreenRuntimeApplication,
    pub first_contact_offline_adapter_application: RtsOfflineAdapterRuntimeApplication,
    pub first_contact_offline_adapter_consumption_review:
        RtsFirstContactOfflineAdapterConsumptionReview,
    pub first_contact_offline_adapter_session_transition_review:
        RtsFirstContactOfflineAdapterSessionTransitionReview,
    pub first_contact_offline_adapter_lobby_ready_review:
        RtsFirstContactOfflineAdapterLobbyReadyReview,
    pub first_contact_online_protocol_fixture: trnm_rts_online::RtsOnlineProtocolFixture,
    pub first_contact_online_local_handoff: trnm_rts_online::RtsOnlineLocalHandoff,
    pub first_contact_online_offline_adapter: trnm_rts_online::RtsOnlineOfflineAdapterSummary,
    pub first_contact_online_protocol_gate: bool,
    pub first_contact_online_local_handoff_gate: bool,
    pub first_contact_online_offline_adapter_gate: bool,
    pub first_contact_map_model_review: RtsFirstContactMapModelReview,
    pub first_contact_map_model_gate: bool,
    pub first_contact_opening_profile: trnm_rts_data::RtsOpeningLoopProfile,
    pub first_contact_opening_profile_gate: bool,
    pub first_contact_command_feedback_profile: trnm_rts_data::RtsCommandFeedbackProfile,
    pub first_contact_command_feedback_gate: bool,
    pub first_contact_player_startup_profiles: Vec<trnm_rts_data::RtsPlayerStartupProfile>,
    pub first_contact_player_startup_gate: bool,
    pub first_contact_actor_presentation_profiles: Vec<trnm_rts_data::RtsActorPresentationProfile>,
    pub first_contact_actor_presentation_gate: bool,
    pub first_contact_visual_telemetry_profile:
        trnm_rts_data::RtsFirstContactVisualTelemetryProfile,
    pub first_contact_visual_telemetry_gate: bool,
    pub first_contact_preview_actor_projection: RtsFirstContactPreviewActorProjectionEvidence,
    pub first_contact_preview_actor_projection_gate: bool,
    pub first_contact_player_screen_profile: trnm_rts_data::RtsFirstContactPlayerScreenProfile,
    pub first_contact_player_screen_layout_gate: bool,
    pub first_contact_player_screen_chrome_gate: bool,
    pub first_contact_player_screen_profile_gate: bool,
    pub first_contact_terrain_profile_count: usize,
    pub first_contact_terrain_profile_samples: RtsFirstContactTerrainProfileSamples,
    pub first_contact_terrain_profile_gate: bool,
    pub first_contact_renderer_projection: RtsFirstContactRendererProjectionEvidence,
    pub first_contact_renderer_projection_gate: bool,
    pub first_contact_runtime_map_projection: RtsRuntimeMapProjection,
    pub first_contact_runtime_tile_rect_sample: RtsRuntimeRect,
    pub first_contact_runtime_terrain_seed_sample: RtsRuntimeTerrainSeeds,
    pub first_contact_runtime_map_projection_gate: bool,
    pub first_contact_player_screen_application_contract: String,
    pub first_contact_player_screen_application_green: bool,
    pub first_contact_offline_adapter_application_contract: String,
    pub first_contact_offline_adapter_application_green: bool,
    pub first_contact_offline_adapter_consumption_contract: String,
    pub first_contact_offline_adapter_consumption_green: bool,
    pub first_contact_offline_adapter_session_transition_contract: String,
    pub first_contact_offline_adapter_session_transition_green: bool,
    pub first_contact_offline_adapter_lobby_ready_contract: String,
    pub first_contact_offline_adapter_lobby_ready_green: bool,
    pub first_contact_runtime_review_contracts: Vec<String>,
    pub first_contact_runtime_review_before_command_queue_sample: Vec<String>,
    pub first_contact_runtime_review_after_command_queue_sample: Vec<String>,
    pub first_contact_runtime_review_ready_state_labels_sample: Vec<String>,
    pub first_contact_runtime_review_command_stamp_tile_sample: Option<String>,
    pub first_contact_runtime_review_gate: bool,
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
    let first_contact_profile = trnm_rts_data::first_contact_player_screen_profile();
    let first_contact_map_model = trnm_rts_data::first_contact_basin_map();
    let first_contact_data_validation_error = first_contact_map_model.validate().err();
    let first_contact_map_summary = first_contact_map_model.summary();
    let first_contact_unit_rule_count = first_contact_map_model
        .rules
        .iter()
        .filter(|rule| rule.kind == trnm_rts_data::RtsRuleKind::Unit)
        .count();
    let first_contact_building_rule_count = first_contact_map_model
        .rules
        .iter()
        .filter(|rule| rule.kind == trnm_rts_data::RtsRuleKind::Structure)
        .count();
    let first_contact_terrain_profiles = trnm_rts_data::first_contact_terrain_profiles();
    let first_contact_renderer_model =
        trnm_rts_data::first_contact_map_renderer_model(&first_contact_map_model);
    let first_contact_preview_actors =
        trnm_rts_data::first_contact_preview_actors(&first_contact_map_model);
    let first_contact_opening_profile = trnm_rts_data::first_contact_opening_loop_profile();
    let first_contact_command_feedback_profile =
        trnm_rts_data::first_contact_command_feedback_profile();
    let first_contact_player_startup_profiles =
        trnm_rts_data::first_contact_player_startup_profiles();
    let first_contact_actor_presentation_profiles =
        trnm_rts_data::first_contact_actor_presentation_profiles();
    let first_contact_visual_telemetry_profile =
        trnm_rts_data::first_contact_visual_telemetry_profile();
    let first_contact_player_screen_application =
        rts_first_contact_player_screen_runtime_application(&first_contact_profile);
    let first_contact_online_protocol_fixture =
        trnm_rts_online::first_contact_online_protocol_fixture();
    let first_contact_online_local_handoff = trnm_rts_online::rts_online_local_handoff_from_fixture(
        &first_contact_online_protocol_fixture,
    );
    let first_contact_adapter = trnm_rts_online::rts_online_offline_adapter_from_fixture(
        &first_contact_online_protocol_fixture,
    );
    let first_contact_runtime_handoff =
        trnm_rts_online::rts_online_offline_adapter_runtime_handoff_review_input(
            &first_contact_adapter,
        );
    let first_contact_offline_adapter_application =
        rts_first_contact_offline_adapter_runtime_application(&first_contact_runtime_handoff);
    let first_contact_session_transition_review =
        rts_first_contact_offline_adapter_session_transition_review(
            &first_contact_player_screen_application,
            &first_contact_offline_adapter_application,
            &first_contact_runtime_handoff,
        );
    let first_contact_lobby_ready_review = rts_first_contact_offline_adapter_lobby_ready_review(
        trnm_rts_online::rts_online_offline_adapter_lobby_ready_review_input(
            &first_contact_adapter,
        ),
    );
    let first_contact_runtime_player_screen_review = RtsFirstContactPlayerScreenReview {
        map_scene: first_contact_player_screen_application.map_scene.clone(),
        current_room_id: first_contact_player_screen_application
            .current_room_id
            .clone(),
        coins: first_contact_player_screen_application.coins,
        xp: first_contact_player_screen_application.xp,
        camera_focus_tile_id: first_contact_player_screen_application
            .camera_focus_tile_id
            .clone(),
        visibility_percent: first_contact_player_screen_application.visibility_percent,
        army_supply_used: first_contact_player_screen_application.army_supply_used,
        army_supply_cap: first_contact_player_screen_application.army_supply_cap,
        objective_status: first_contact_player_screen_application
            .objective_status
            .clone(),
        production_queue: first_contact_player_screen_application
            .production_queue
            .clone(),
        build_queue: first_contact_player_screen_application.build_queue.clone(),
        selected_unit_ids: first_contact_offline_adapter_application
            .selected_unit_ids
            .clone(),
        command_queue: first_contact_offline_adapter_application
            .command_queue
            .clone(),
        command_destination_tile_id: first_contact_offline_adapter_application
            .command_destination_tile_id
            .clone(),
        group_route_tile_ids: first_contact_offline_adapter_application
            .group_route_tile_ids
            .clone(),
        visible_tile_count: first_contact_player_screen_application
            .visible_tile_ids
            .len(),
        fogged_tile_count: first_contact_player_screen_application
            .fogged_tile_ids
            .len(),
        selection_box_tile_count: first_contact_player_screen_application
            .selection_box_tile_ids
            .len(),
        unit_health_percents: first_contact_player_screen_application
            .unit_health_percents
            .clone(),
        ability_command_ids: first_contact_player_screen_application
            .ability_command_ids
            .clone(),
        ability_cooldown_percents: first_contact_player_screen_application
            .ability_cooldown_percents
            .clone(),
        active_ability_id: first_contact_player_screen_application
            .active_ability_id
            .clone(),
    };
    let first_contact_consumption_review = rts_first_contact_offline_adapter_consumption_review(
        trnm_rts_online::rts_online_offline_adapter_consumption_review_input(
            &first_contact_adapter,
            first_contact_runtime_player_screen_review,
        ),
    );
    let first_contact_runtime_review_contracts = vec![
        TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_PLAYER_SCREEN_APPLICATION_CONTRACT.to_string(),
        TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_APPLICATION_CONTRACT.to_string(),
        TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_CONSUMPTION_CONTRACT.to_string(),
        TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_SESSION_TRANSITION_CONTRACT.to_string(),
        TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_LOBBY_READY_CONTRACT.to_string(),
    ];
    let first_contact_runtime_review_gate = first_contact_player_screen_application.green
        && first_contact_offline_adapter_application.green
        && first_contact_consumption_review.green
        && first_contact_session_transition_review.green
        && first_contact_lobby_ready_review.green
        && first_contact_player_screen_application.contract_version
            == TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_PLAYER_SCREEN_APPLICATION_CONTRACT
        && first_contact_offline_adapter_application.contract_version
            == TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_APPLICATION_CONTRACT
        && first_contact_consumption_review.contract_version
            == TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_CONSUMPTION_CONTRACT
        && first_contact_session_transition_review.contract_version
            == TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_SESSION_TRANSITION_CONTRACT
        && first_contact_lobby_ready_review.contract_version
            == TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_LOBBY_READY_CONTRACT
        && first_contact_session_transition_review
            .before_command_queue
            .iter()
            .any(|command| command == "build:trnm.flux.relay")
        && first_contact_session_transition_review.after_command_queue == vec!["move:8,4"]
        && first_contact_session_transition_review.after_route_tile_ids == vec!["8,4"]
        && first_contact_session_transition_review
            .after_command_destination_tile_id
            .as_deref()
            == Some("8,4")
        && first_contact_lobby_ready_review
            .ready_state_labels
            .iter()
            .any(|label| label == "authority:offline_loopback:no_socket")
        && first_contact_consumption_review
            .runtime_command_stamp_tile_id
            .as_deref()
            == Some("8,4")
        && !first_contact_consumption_review.socket_opened
        && !first_contact_consumption_review.hosted_service_claimed
        && !first_contact_consumption_review.public_launch_ready;
    let first_contact_online_protocol_gate = first_contact_online_protocol_fixture.green
        && first_contact_online_protocol_fixture.envelope.map_id == "first_contact_basin"
        && first_contact_online_protocol_fixture.lifecycle.map_id == "first_contact_basin"
        && first_contact_online_protocol_fixture
            .envelope
            .update_sha256
            .len()
            == 64
        && first_contact_online_protocol_fixture
            .envelope
            .scope
            .visible_chunks
            .len()
            == 3
        && first_contact_online_protocol_fixture
            .envelope
            .scope
            .visible_actor_ids
            .iter()
            .any(|actor_id| actor_id == "trnm.flux.beacon.center")
        && first_contact_online_protocol_fixture.lifecycle.bot_count == 1;
    let first_contact_online_local_handoff_gate = first_contact_online_local_handoff.green
        && first_contact_online_local_handoff.handoff_ready
        && first_contact_online_local_handoff.map_id == "first_contact_basin"
        && first_contact_online_local_handoff.accepted_order_count == 1
        && first_contact_online_local_handoff.rejected_order_count == 1
        && first_contact_online_local_handoff.scoped_update_count == 1
        && first_contact_online_local_handoff.bot_count == 1
        && first_contact_online_local_handoff.visible_chunk_count == 3
        && first_contact_online_local_handoff.visible_actor_count == 4
        && first_contact_online_local_handoff.server_authoritative
        && first_contact_online_local_handoff.visibility_scoped_response
        && !first_contact_online_local_handoff.socket_opened
        && !first_contact_online_local_handoff.hosted_service_claimed
        && !first_contact_online_local_handoff.public_launch_ready;
    let first_contact_online_offline_adapter_gate = first_contact_adapter.green
        && first_contact_adapter.adapter_mode == "offline_loopback_authority"
        && first_contact_adapter.map_id == "first_contact_basin"
        && first_contact_adapter.local_multiplayer_ready
        && first_contact_adapter.offline_bot_ready
        && first_contact_adapter.bevy_adapter_ready
        && first_contact_adapter.local_action_replay.green
        && first_contact_adapter.local_runtime_handoff.green
        && first_contact_adapter.local_runtime_handoff.contract_version
            == trnm_rts_online::TRNM_RTS_ONLINE_OFFLINE_ADAPTER_RUNTIME_HANDOFF_CONTRACT
        && first_contact_adapter
            .local_runtime_handoff
            .accepted_runtime_command_labels
            == vec!["move:8,4"]
        && first_contact_adapter
            .local_runtime_handoff
            .accepted_runtime_destination_tile_ids
            == vec!["8,4"]
        && first_contact_adapter
            .local_runtime_handoff
            .accepted_runtime_subject_actor_ids
            == vec!["trnm.worker.alpha"]
        && first_contact_adapter
            .local_runtime_handoff
            .rejected_runtime_command_labels
            == vec!["client:attack_fogged_keep"]
        && first_contact_adapter
            .local_runtime_handoff
            .runtime_command_stamp_tile_id
            .as_deref()
            == Some("8,4")
        && first_contact_adapter
            .local_runtime_handoff
            .accepted_order_runtime_ready
        && first_contact_adapter
            .local_runtime_handoff
            .rejected_order_runtime_ready
        && first_contact_adapter
            .local_runtime_handoff
            .scoped_update_runtime_ready
        && first_contact_adapter
            .local_runtime_handoff
            .no_socket_boundary_ready
        && first_contact_adapter
            .local_action_replay
            .accepted_action_labels
            == vec![
                "RTS:SELECT:26",
                "RTS:MOVE:18,31:line",
                "RTS:SELECT:27",
                "RTS:MOVE:21,25:line",
                "RTS:SELECT:28",
                "RTS:MOVE:1,31:line",
                "RTS:SELECT:26",
            ]
        && first_contact_adapter
            .local_action_replay
            .accepted_preview_stages
            == vec![
                "group_26_queued",
                "group_27_override",
                "group_28_formation",
                "cleared_history_bounded",
            ]
        && first_contact_adapter.local_action_replay.blocked_reasons
            == vec![
                "rts_group_selection_required",
                "rts_invalid_tile:bad-tile",
                "rts_attack_target_required",
                "rts_attack_required_before_ability",
                "rts_queue_id_required",
                "rts_queue_unaffordable:build:watch_tower@7,4",
                "rts_group_id_required",
            ]
        && first_contact_adapter
            .local_action_replay
            .retained_history_group_ids
            == vec!["26", "27", "28"]
        && first_contact_adapter
            .local_action_replay
            .pruned_history_group_ids
            == vec!["25", "24"]
        && first_contact_adapter
            .local_action_replay
            .command_history_capacity
            == 3
        && first_contact_adapter
            .local_action_replay
            .local_input_sources_ready
        && first_contact_adapter
            .local_action_replay
            .command_history_ready
        && first_contact_adapter.connected_player_ids.len() == 2
        && first_contact_adapter.bot_player_ids.len() == 1
        && first_contact_adapter.input_queue_labels.len() == 2
        && first_contact_adapter.accepted_server_order_labels.len() == 1
        && first_contact_adapter.rejected_client_order_reasons.len() == 1
        && first_contact_adapter.scoped_update_actor_ids.len() == 4
        && first_contact_adapter.scoped_update_order_count == 1
        && first_contact_adapter
            .frame_sha256s
            .iter()
            .all(|sha| sha.len() == 64)
        && first_contact_adapter.server_authoritative
        && first_contact_adapter.visibility_scoped_response
        && !first_contact_adapter.client_prediction_claimed
        && !first_contact_adapter.rollback_netcode_claimed
        && !first_contact_adapter.socket_opened
        && !first_contact_adapter.hosted_service_claimed
        && !first_contact_adapter.public_launch_ready;
    let first_contact_opening_profile_gate = first_contact_opening_profile.contract_version
        == trnm_rts_data::TRNM_RTS_DATA_FIRST_CONTACT_OPENING_PROFILE_CONTRACT
        && first_contact_opening_profile.map_id == first_contact_map_model.map_id
        && first_contact_map_model
            .bounds
            .contains(first_contact_opening_profile.active_beacon_tile)
        && first_contact_map_model.actors.iter().any(|actor| {
            actor.rule_id == "trnm.flux.beacon"
                && actor.tile == first_contact_opening_profile.active_beacon_tile
        })
        && first_contact_map_model.actors.iter().any(|actor| {
            actor.rule_id == "trnm.expansion.marker"
                && actor.tile == first_contact_opening_profile.active_relay_tile
        });
    let first_contact_command_feedback_gate = first_contact_command_feedback_profile
        .contract_version
        == trnm_rts_data::TRNM_RTS_DATA_FIRST_CONTACT_COMMAND_FEEDBACK_CONTRACT
        && first_contact_command_feedback_profile.map_id == first_contact_map_model.map_id
        && first_contact_command_feedback_profile.target_tile
            == first_contact_opening_profile.active_beacon_tile
        && first_contact_map_model
            .bounds
            .contains(first_contact_command_feedback_profile.blocked_tile);
    let first_contact_player_startup_gate = first_contact_player_startup_profiles.len() == 4
        && first_contact_player_startup_profiles.iter().all(|startup| {
            startup.contract_version
                == trnm_rts_data::TRNM_RTS_DATA_FIRST_CONTACT_PLAYER_STARTUP_CONTRACT
                && startup.map_id == first_contact_map_model.map_id
                && first_contact_map_model.bounds.contains(startup.spawn_tile)
                && first_contact_map_model.players.iter().any(|player| {
                    player.id == startup.player_id
                        && player.playable
                        && player.faction == startup.faction
                })
                && first_contact_map_model.actors.iter().any(|actor| {
                    actor.rule_id == "mpspawn"
                        && actor.owner == startup.player_id
                        && actor.tile == startup.spawn_tile
                })
                && first_contact_map_model
                    .rules
                    .iter()
                    .any(|rule| rule.id == startup.command_core_rule_id)
                && first_contact_map_model
                    .rules
                    .iter()
                    .any(|rule| rule.id == startup.worker_rule_id)
                && first_contact_map_model
                    .rules
                    .iter()
                    .any(|rule| rule.id == startup.faction_unit_rule_id)
                && startup.opening_beacon_tile == first_contact_opening_profile.active_beacon_tile
                && startup.opening_relay_tile == first_contact_opening_profile.active_relay_tile
        });
    let first_contact_actor_presentation_by_rule = |rule_id: &str| {
        first_contact_actor_presentation_profiles
            .iter()
            .find(|profile| profile.rule_id == rule_id)
    };
    let first_contact_actor_presentation_gate = first_contact_actor_presentation_profiles.len()
        >= 13
        && first_contact_actor_presentation_profiles
            .iter()
            .all(|profile| {
                profile.contract_version
                    == trnm_rts_data::TRNM_RTS_DATA_FIRST_CONTACT_ACTOR_PRESENTATION_CONTRACT
                    && profile.map_id == first_contact_map_model.map_id
                    && profile.health_bar_width >= 10
                    && profile.glyph.contract_version
                        == trnm_rts_data::TRNM_RTS_DATA_FIRST_CONTACT_ACTOR_GLYPH_CONTRACT
                    && profile.glyph.footprint_width_cells > 0
                    && profile.glyph.footprint_height_cells > 0
                    && first_contact_map_model
                        .rules
                        .iter()
                        .any(|rule| rule.id == profile.rule_id)
            })
        && first_contact_actor_presentation_by_rule("trnm.worker").is_some_and(|profile| {
            profile.color_role == trnm_rts_data::RtsActorColorRole::Worker
                && !profile.structure
                && profile.selectable
        })
        && first_contact_actor_presentation_by_rule("trnm.command.core").is_some_and(|profile| {
            profile.color_role == trnm_rts_data::RtsActorColorRole::CommandCore
                && profile.structure
                && profile.health_bar_width >= 32
                && profile.glyph.body == trnm_rts_data::RtsActorGlyphBody::Structure
                && profile.glyph.accent == trnm_rts_data::RtsActorGlyphAccent::CommandSpire
                && profile.glyph.footprint_width_cells == 2
        })
        && first_contact_actor_presentation_by_rule("trnm.flux.beacon").is_some_and(|profile| {
            profile.color_role == trnm_rts_data::RtsActorColorRole::Objective
                && profile.structure
                && profile.glyph.body == trnm_rts_data::RtsActorGlyphBody::ObjectiveBeacon
                && profile.glyph.accent == trnm_rts_data::RtsActorGlyphAccent::BeaconCore
        })
        && first_contact_actor_presentation_by_rule("mpspawn").is_some_and(|profile| {
            profile.glyph.body == trnm_rts_data::RtsActorGlyphBody::SpawnPad
                && profile.glyph.accent == trnm_rts_data::RtsActorGlyphAccent::OwnerStripe
        });
    let first_contact_visual_telemetry_gate = first_contact_visual_telemetry_profile
        .contract_version
        == trnm_rts_data::TRNM_RTS_DATA_FIRST_CONTACT_VISUAL_TELEMETRY_CONTRACT
        && first_contact_visual_telemetry_profile.map_id == first_contact_map_model.map_id
        && first_contact_visual_telemetry_profile.unit_statuses.len() == 4
        && first_contact_visual_telemetry_profile.tactical_tracks.len() == 6
        && first_contact_visual_telemetry_profile
            .unit_statuses
            .iter()
            .all(|status| {
                first_contact_map_model.bounds.contains(status.tile)
                    && status.health_percent <= 100
                    && status.shield_percent <= 100
                    && !status.role_badge.is_empty()
            })
        && first_contact_visual_telemetry_profile
            .tactical_tracks
            .iter()
            .all(|track| {
                first_contact_map_model.bounds.contains(track.from_tile)
                    && first_contact_map_model.bounds.contains(track.to_tile)
            })
        && first_contact_visual_telemetry_profile
            .unit_statuses
            .iter()
            .any(|status| {
                status.tile == trnm_rts_core::RtsTile::new(8, 8)
                    && status.role_color == trnm_rts_data::RtsVisualTelemetryColorRole::Health
            })
        && first_contact_visual_telemetry_profile
            .tactical_tracks
            .iter()
            .any(|track| {
                track.from_tile == first_contact_opening_profile.active_relay_tile
                    && track.to_tile == first_contact_opening_profile.active_beacon_tile
            });
    let first_contact_preview_actor_projection = RtsFirstContactPreviewActorProjectionEvidence {
        actor_count: first_contact_preview_actors.len(),
        spawn_count: first_contact_preview_actors
            .iter()
            .filter(|actor| actor.kind == trnm_rts_data::RtsFirstContactPreviewActorKind::Spawn)
            .count(),
        flux_bloom_count: first_contact_preview_actors
            .iter()
            .filter(|actor| actor.kind == trnm_rts_data::RtsFirstContactPreviewActorKind::FluxBloom)
            .count(),
        beacon_count: first_contact_preview_actors
            .iter()
            .filter(|actor| actor.kind == trnm_rts_data::RtsFirstContactPreviewActorKind::Beacon)
            .count(),
        expansion_count: first_contact_preview_actors
            .iter()
            .filter(|actor| {
                actor.kind == trnm_rts_data::RtsFirstContactPreviewActorKind::ExpansionMarker
            })
            .count(),
        actor_samples: first_contact_preview_actors
            .iter()
            .take(6)
            .cloned()
            .collect(),
        source: "trnm-rts-data first_contact_preview_actors projection from RtsMapModel actors"
            .to_string(),
    };
    let first_contact_preview_actor_projection_gate =
        first_contact_preview_actor_projection.actor_count == first_contact_map_summary.actor_count
            && first_contact_preview_actor_projection.spawn_count
                == first_contact_map_summary.spawn_count
            && first_contact_preview_actor_projection.flux_bloom_count
                == first_contact_map_summary.flux_bloom_count
            && first_contact_preview_actor_projection.beacon_count
                == first_contact_map_summary.beacon_count
            && first_contact_preview_actor_projection.expansion_count
                == first_contact_map_summary.expansion_count
            && first_contact_preview_actors.iter().all(|actor| {
                actor.contract_version
                    == trnm_rts_data::TRNM_RTS_DATA_FIRST_CONTACT_PREVIEW_ACTOR_CONTRACT
                    && first_contact_map_model.bounds.contains(actor.tile)
                    && actor.source_rule_id == actor.kind.source_rule_id()
                    && actor.openra_preview_rule_id == actor.kind.openra_preview_rule_id()
                    && first_contact_map_model
                        .actors
                        .iter()
                        .any(|source| source.id == actor.source_actor_id)
            });
    let first_contact_map_actor_gate = first_contact_map_summary.actor_count == 39;
    let first_contact_map_topology_gate = first_contact_map_summary.spawn_count == 4
        && first_contact_map_summary.flux_bloom_count == 11
        && first_contact_map_summary.beacon_count == 4
        && first_contact_map_summary.expansion_count == 4;
    let first_contact_rules_gate = first_contact_unit_rule_count >= 4
        && first_contact_building_rule_count >= 2
        && first_contact_map_model
            .rules
            .iter()
            .any(|rule| rule.id == "trnm.worker" && rule.cost == 200 && rule.hp == 8000)
        && first_contact_map_model
            .rules
            .iter()
            .any(|rule| rule.id == "trnm.horizon.scout" && rule.speed == Some(92))
        && first_contact_map_model
            .rules
            .iter()
            .any(|rule| rule.id == "trnm.forge.warden" && rule.hp == 18000)
        && first_contact_map_model
            .rules
            .iter()
            .any(|rule| rule.id == "trnm.command.core" && rule.cost == 1600)
        && first_contact_map_model
            .rules
            .iter()
            .any(|rule| rule.id == "trnm.flux.relay" && rule.cost == 500);
    let first_contact_data_consumer_gate = first_contact_data_validation_error.is_none()
        && first_contact_map_model.contract_version == trnm_rts_data::TRNM_RTS_DATA_CONTRACT
        && first_contact_map_summary.contract_version == trnm_rts_data::TRNM_RTS_DATA_CONTRACT
        && first_contact_map_summary.canonical_sha256.len() == 64
        && first_contact_map_summary.source_integration_mode == "gpl_internal_component";
    let first_contact_map_model_adapter_gate = first_contact_data_consumer_gate
        && first_contact_preview_actor_projection_gate
        && first_contact_map_summary.actor_count
            == first_contact_preview_actor_projection.actor_count
        && first_contact_preview_actors.len() == first_contact_preview_actor_projection.actor_count;
    let first_contact_map_model_gate = first_contact_map_actor_gate
        && first_contact_map_topology_gate
        && first_contact_rules_gate
        && first_contact_data_consumer_gate
        && first_contact_map_model_adapter_gate;
    let first_contact_map_model_review = RtsFirstContactMapModelReview {
        map_summary: first_contact_map_summary.clone(),
        unit_rule_count: first_contact_unit_rule_count,
        building_rule_count: first_contact_building_rule_count,
        data_validation_error: first_contact_data_validation_error.clone(),
        map_actor_gate: first_contact_map_actor_gate,
        map_topology_gate: first_contact_map_topology_gate,
        rules_gate: first_contact_rules_gate,
        data_consumer_gate: first_contact_data_consumer_gate,
        map_model_adapter_gate: first_contact_map_model_adapter_gate,
        source:
            "trnm-rts-data First Contact map model and rule summary reviewed by trnm-rts-evidence"
                .to_string(),
    };
    let first_contact_player_screen_layout = first_contact_profile.layout;
    let first_contact_player_screen_layout_gate =
        first_contact_player_screen_layout.player_map.map_origin_x == 16
            && first_contact_player_screen_layout.player_map.map_origin_y == 54
            && first_contact_player_screen_layout
                .player_map
                .right_reserved_px
                == 292
            && first_contact_player_screen_layout
                .player_map
                .bottom_reserved_px
                == 158
            && first_contact_player_screen_layout
                .player_map
                .min_map_width_px
                == 374
            && first_contact_player_screen_layout
                .player_map
                .min_map_height_px
                == 238
            && first_contact_player_screen_layout.player_map.cell_width.min == 12
            && first_contact_player_screen_layout.player_map.cell_width.max == 28
            && first_contact_player_screen_layout
                .player_map
                .cell_height
                .min
                == 8
            && first_contact_player_screen_layout
                .player_map
                .cell_height
                .max
                == 15
            && first_contact_player_screen_layout.spec_map.map_origin_x == 24
            && first_contact_player_screen_layout.spec_map.map_origin_y == 110
            && first_contact_player_screen_layout
                .spec_map
                .right_reserved_px
                == 266
            && first_contact_player_screen_layout
                .spec_map
                .bottom_reserved_px
                == 158
            && first_contact_player_screen_layout.spec_map.min_map_width_px == 374
            && first_contact_player_screen_layout
                .spec_map
                .min_map_height_px
                == 238
            && first_contact_player_screen_layout.spec_map.cell_width.min == 10
            && first_contact_player_screen_layout.spec_map.cell_width.max == 22
            && first_contact_player_screen_layout.spec_map.cell_height.min == 7
            && first_contact_player_screen_layout.spec_map.cell_height.max == 14
            && first_contact_player_screen_layout.map_outer_padding_px == 8
            && first_contact_player_screen_layout.map_inner_padding_px == 4;
    let first_contact_player_screen_chrome = &first_contact_profile.chrome;
    let first_contact_player_screen_chrome_gate = first_contact_player_screen_chrome.top_title
        == "TRNM RTS"
        && first_contact_player_screen_chrome.skirmish_status_label
            == "LOCAL SKIRMISH  OWNED ASSETS"
        && first_contact_player_screen_chrome.tactical_view_title == "TACTICAL VIEW"
        && first_contact_player_screen_chrome.tactical_view_camera_prefix == "CAM"
        && first_contact_player_screen_chrome.tactical_view_zoom_prefix == "Z"
        && first_contact_player_screen_chrome.tactical_view_default_camera_tile
            == first_contact_profile.camera_focus_tile
        && first_contact_player_screen_chrome.tactical_view_status_fallback
            == "GROUP 1  ATTACK QUEUED"
        && first_contact_player_screen_chrome.tactical_view_status_max_chars == 40
        && first_contact_player_screen_chrome.resource_readouts.len() == 4
        && first_contact_player_screen_chrome
            .resource_readouts
            .iter()
            .any(|readout| {
                readout.kind == trnm_rts_data::RtsPlayerScreenResourceReadoutKind::Credits
                    && readout.label == "CREDITS"
            })
        && first_contact_player_screen_chrome
            .resource_readouts
            .iter()
            .any(|readout| {
                readout.kind == trnm_rts_data::RtsPlayerScreenResourceReadoutKind::Power
                    && readout.label == "POWER"
            })
        && first_contact_player_screen_chrome
            .resource_readouts
            .iter()
            .any(|readout| {
                readout.kind == trnm_rts_data::RtsPlayerScreenResourceReadoutKind::Supply
                    && readout.label == "SUPPLY"
            })
        && first_contact_player_screen_chrome
            .resource_readouts
            .iter()
            .any(|readout| {
                readout.kind == trnm_rts_data::RtsPlayerScreenResourceReadoutKind::Visibility
                    && readout.label == "VISION"
            })
        && first_contact_player_screen_chrome.radar_title == "RADAR"
        && first_contact_player_screen_chrome.production_title == "PRODUCTION"
        && first_contact_player_screen_chrome.build_palette_title == "BUILD PALETTE"
        && first_contact_player_screen_chrome.production_empty_label == "ready"
        && first_contact_player_screen_chrome.production_slot_visible_count == 4
        && first_contact_player_screen_chrome.production_slot_column_count == 2
        && first_contact_player_screen_chrome.build_palette_slots.len() == 8
        && first_contact_player_screen_chrome
            .build_palette_slots
            .iter()
            .any(|slot| slot.label == "POWER" && slot.queue_id == "build:power_node@5,3")
        && first_contact_player_screen_chrome
            .build_palette_slots
            .iter()
            .any(|slot| slot.label == "TRAIN" && slot.queue_id == "build:training_hall@4,3")
        && first_contact_player_screen_chrome
            .build_palette_slots
            .iter()
            .any(|slot| slot.label == "SIGNAL" && slot.queue_id == "upgrade:signal_blade")
        && first_contact_player_screen_chrome.build_palette_visible_count == 8
        && first_contact_player_screen_chrome.build_palette_column_count == 4
        && first_contact_player_screen_chrome.tactics_title == "TACTICS"
        && first_contact_player_screen_chrome.tactics_rows.len() == 5
        && first_contact_player_screen_chrome
            .tactics_rows
            .iter()
            .any(|row| {
                row.kind == trnm_rts_data::RtsPlayerScreenTacticsRowKind::Order
                    && row.label == "ORDER"
                    && row.max_value_chars == 20
            })
        && first_contact_player_screen_chrome
            .tactics_rows
            .iter()
            .any(|row| {
                row.kind == trnm_rts_data::RtsPlayerScreenTacticsRowKind::Target
                    && row.label == "TARGET"
                    && row.empty_label == "NONE"
            })
        && first_contact_player_screen_chrome
            .tactics_rows
            .iter()
            .any(|row| {
                row.kind == trnm_rts_data::RtsPlayerScreenTacticsRowKind::Camera
                    && row.label == "CAM"
                    && row.empty_label == "-"
            })
        && first_contact_player_screen_chrome
            .tactics_rows
            .iter()
            .any(|row| {
                row.kind == trnm_rts_data::RtsPlayerScreenTacticsRowKind::Queue
                    && row.label == "QUEUE"
            })
        && first_contact_player_screen_chrome
            .tactics_rows
            .iter()
            .any(|row| {
                row.kind == trnm_rts_data::RtsPlayerScreenTacticsRowKind::Build
                    && row.label == "BUILD"
                    && row.empty_label == "NONE"
            })
        && first_contact_player_screen_chrome.selection_panel_title == "SELECTION"
        && first_contact_player_screen_chrome.selection_card_visible_count == 5
        && first_contact_player_screen_chrome
            .selection_card_frame_ids
            .len()
            == 5
        && first_contact_player_screen_chrome
            .selection_card_frame_ids
            .iter()
            .any(|frame| frame == "actor_player_idle_south")
        && first_contact_player_screen_chrome
            .selection_card_frame_ids
            .iter()
            .any(|frame| frame == "prop_banner")
        && first_contact_player_screen_chrome.selection_card_health_fallback_percent == 80
        && first_contact_player_screen_chrome.selection_feedback_label_max_chars == 62
        && first_contact_player_screen_chrome.command_panel_title == "COMMANDS"
        && first_contact_player_screen_chrome.command_grid_slot_count == 12
        && first_contact_player_screen_chrome.command_grid_column_count == 6
        && first_contact_player_screen_chrome
            .command_grid_slot_ids
            .len()
            == 6
        && first_contact_player_screen_chrome
            .command_grid_slot_ids
            .iter()
            .any(|ability| ability == "relay")
        && first_contact_player_screen_chrome
            .command_grid_slot_ids
            .iter()
            .any(|ability| ability == "signal")
        && first_contact_player_screen_chrome.command_slot_fallback_id == "hold"
        && first_contact_player_screen_chrome.order_queue_title == "ORDER QUEUE"
        && first_contact_player_screen_chrome.order_queue_empty_label == "NO ORDERS"
        && first_contact_player_screen_chrome.order_queue_visible_count == 5
        && first_contact_player_screen_chrome.order_queue_label_max_chars == 32
        && first_contact_player_screen_chrome.group_summary_prefix == "GROUP"
        && first_contact_player_screen_chrome.group_summary_suffix == "UNITS SELECTED";
    let first_contact_player_screen_profile_gate = first_contact_profile.contract_version
        == trnm_rts_data::TRNM_RTS_DATA_FIRST_CONTACT_PLAYER_SCREEN_CONTRACT
        && first_contact_profile.map_id == first_contact_map_model.map_id
        && first_contact_profile.room_id == "first-contact-basin"
        && first_contact_player_screen_layout_gate
        && first_contact_player_screen_chrome_gate
        && first_contact_profile.camera_zoom_percent > 0
        && first_contact_map_model
            .bounds
            .contains(first_contact_profile.camera_focus_tile)
        && first_contact_profile.command_destination_tile
            == first_contact_opening_profile.active_beacon_tile
        && first_contact_map_model
            .rules
            .iter()
            .any(|rule| rule.id == first_contact_profile.attack_target_rule_id)
        && first_contact_profile
            .command_queue
            .iter()
            .any(|command| command == "build:trnm.flux.relay")
        && first_contact_profile
            .command_queue
            .iter()
            .any(|command| command == "train:trnm.worker")
        && first_contact_profile
            .command_queue
            .iter()
            .any(|command| command == "attack:trnm.flux.beacon")
        && first_contact_profile
            .production_queue
            .iter()
            .any(|queue| queue == "train:guard")
        && first_contact_profile
            .production_queue
            .iter()
            .any(|queue| queue == "upgrade:signal_blade")
        && first_contact_profile
            .build_queue
            .iter()
            .any(|queue| queue == "build:watch_tower")
        && first_contact_profile
            .build_queue
            .iter()
            .any(|queue| queue == "upgrade:training_hall")
        && first_contact_profile.unit_health_percents == vec![96, 78, 71, 34]
        && first_contact_profile
            .unit_health_percents
            .iter()
            .all(|percent| *percent <= 100)
        && first_contact_profile.active_ability_id == "worker"
        && first_contact_player_screen_chrome
            .command_grid_slot_ids
            .iter()
            .any(|ability| ability == &first_contact_profile.active_ability_id)
        && first_contact_profile.ability_cooldown_percents == vec![0, 0, 16, 0, 42, 25]
        && first_contact_profile.ability_cooldown_percents.len()
            == first_contact_player_screen_chrome
                .command_grid_slot_ids
                .len()
        && first_contact_profile
            .ability_cooldown_percents
            .iter()
            .all(|percent| *percent <= 100)
        && first_contact_profile.visible_tiles.len() == 64
        && first_contact_profile
            .visible_tiles
            .iter()
            .all(|tile| first_contact_map_model.bounds.contains(*tile))
        && first_contact_profile
            .fogged_tiles
            .iter()
            .all(|tile| first_contact_map_model.bounds.contains(*tile))
        && first_contact_profile
            .selection_box_tiles
            .iter()
            .all(|tile| first_contact_map_model.bounds.contains(*tile))
        && first_contact_profile
            .group_route_tiles
            .iter()
            .any(|tile| *tile == first_contact_opening_profile.active_beacon_tile)
        && first_contact_profile
            .terrain_route_tiles
            .iter()
            .all(|tile| first_contact_map_model.bounds.contains(*tile))
        && first_contact_profile.training_progress_percent <= 100
        && first_contact_profile.build_progress_percent <= 100
        && first_contact_profile.ai_pressure_percent <= 100
        && first_contact_profile.visibility_percent <= 100
        && first_contact_profile.enemy_pressure_warning_percent <= 100
        && first_contact_profile.army_supply_used <= first_contact_profile.army_supply_cap
        && !first_contact_profile.last_feedback.is_empty()
        && !first_contact_profile.objective_status.is_empty();
    let first_contact_terrain_profile_samples = RtsFirstContactTerrainProfileSamples {
        border: trnm_rts_data::first_contact_terrain_profile(trnm_rts_core::RtsTile::new(0, 0)),
        lane: trnm_rts_data::first_contact_terrain_profile(trnm_rts_core::RtsTile::new(16, 9)),
        center: trnm_rts_data::first_contact_terrain_profile(trnm_rts_core::RtsTile::new(16, 16)),
        base_pad: trnm_rts_data::first_contact_terrain_profile(trnm_rts_core::RtsTile::new(10, 10)),
        resource_zone: trnm_rts_data::first_contact_terrain_profile(trnm_rts_core::RtsTile::new(
            12, 16,
        )),
    };
    let first_contact_terrain_profile_count = first_contact_terrain_profiles.len();
    let first_contact_terrain_profile_gate = first_contact_terrain_profile_count
        == (first_contact_map_model.width * first_contact_map_model.height) as usize
        && first_contact_terrain_profiles
            .iter()
            .any(|profile| profile.role == trnm_rts_data::RtsTerrainRole::Lane)
        && first_contact_terrain_profiles
            .iter()
            .any(|profile| profile.role == trnm_rts_data::RtsTerrainRole::BasePad)
        && first_contact_terrain_profiles
            .iter()
            .any(|profile| profile.role == trnm_rts_data::RtsTerrainRole::CentralBasin)
        && first_contact_terrain_profiles
            .iter()
            .filter(|profile| profile.resource_zone)
            .count()
            >= 76
        && first_contact_terrain_profile_samples.center.height == 2;
    let first_contact_renderer_projection = RtsFirstContactRendererProjectionEvidence {
        renderable_tile_count: first_contact_renderer_model.renderable_tiles.len(),
        lane_tile_count: first_contact_renderer_model.lane_tiles.len(),
        resource_zone_tile_count: first_contact_renderer_model.resource_zone_tiles.len(),
        base_pad_tile_count: first_contact_renderer_model.base_pad_tiles.len(),
        minimap_anchor_actor_count: first_contact_renderer_model.minimap_anchor_actor_ids.len(),
        resource_actor_tile_count: first_contact_renderer_model.resource_actor_tiles.len(),
        objective_actor_tile_count: first_contact_renderer_model.objective_actor_tiles.len(),
        spawn_actor_tile_count: first_contact_renderer_model.spawn_actor_tiles.len(),
        lane_tile_samples: first_contact_renderer_model
            .lane_tiles
            .iter()
            .take(6)
            .copied()
            .collect(),
        resource_actor_tile_samples: first_contact_renderer_model
            .resource_actor_tiles
            .iter()
            .take(4)
            .copied()
            .collect(),
        objective_actor_tile_samples: first_contact_renderer_model
            .objective_actor_tiles
            .iter()
            .take(4)
            .copied()
            .collect(),
        spawn_actor_tile_samples: first_contact_renderer_model
            .spawn_actor_tiles
            .iter()
            .take(4)
            .copied()
            .collect(),
        minimap_anchor_actor_samples: first_contact_renderer_model
            .minimap_anchor_actor_ids
            .iter()
            .take(6)
            .cloned()
            .collect(),
        source: "RtsMapModel bounds, terrain profiles, actor rules, and runtime projection math"
            .to_string(),
    };
    let first_contact_renderer_projection_gate = first_contact_renderer_projection
        .renderable_tile_count
        == (first_contact_map_model.bounds.width * first_contact_map_model.bounds.height) as usize
        && first_contact_renderer_projection.lane_tile_count >= 120
        && first_contact_renderer_projection.resource_zone_tile_count >= 76
        && first_contact_renderer_projection.base_pad_tile_count >= 120
        && first_contact_renderer_projection.minimap_anchor_actor_count
            == first_contact_map_summary.actor_count
        && first_contact_renderer_projection.resource_actor_tile_count
            == first_contact_map_summary.flux_bloom_count
        && first_contact_renderer_projection.objective_actor_tile_count
            == first_contact_map_summary.beacon_count
        && first_contact_renderer_projection.spawn_actor_tile_count
            == first_contact_map_summary.spawn_count
        && first_contact_renderer_model
            .resource_actor_tiles
            .iter()
            .all(|tile| first_contact_map_model.bounds.contains(*tile))
        && first_contact_renderer_model
            .objective_actor_tiles
            .iter()
            .all(|tile| first_contact_map_model.bounds.contains(*tile))
        && first_contact_renderer_model
            .spawn_actor_tiles
            .iter()
            .all(|tile| first_contact_map_model.bounds.contains(*tile));
    let first_contact_runtime_map_projection =
        rts_runtime_map_projection(RtsRuntimeMapLayoutInput {
            viewport_width: 1280,
            viewport_height: 720,
            map_width_tiles: first_contact_map_model.width as i32,
            map_height_tiles: first_contact_map_model.height as i32,
            map_origin_x: first_contact_profile.layout.player_map.map_origin_x,
            map_origin_y: first_contact_profile.layout.player_map.map_origin_y,
            right_reserved_px: first_contact_profile.layout.player_map.right_reserved_px,
            bottom_reserved_px: first_contact_profile.layout.player_map.bottom_reserved_px,
            min_map_width_px: first_contact_profile.layout.player_map.min_map_width_px,
            min_map_height_px: first_contact_profile.layout.player_map.min_map_height_px,
            cell_width_min: first_contact_profile.layout.player_map.cell_width.min,
            cell_width_max: first_contact_profile.layout.player_map.cell_width.max,
            cell_height_min: first_contact_profile.layout.player_map.cell_height.min,
            cell_height_max: first_contact_profile.layout.player_map.cell_height.max,
        });
    let first_contact_runtime_tile_rect_sample =
        rts_runtime_tile_screen_rect(first_contact_runtime_map_projection, (16, 16));
    let first_contact_runtime_terrain_seed_sample = rts_runtime_terrain_seeds((16, 16));
    let first_contact_runtime_map_projection_gate = first_contact_runtime_map_projection.map_x
        == 16
        && first_contact_runtime_map_projection.map_y == 54
        && first_contact_runtime_map_projection.cell_w == 28
        && first_contact_runtime_map_projection.cell_h == 14
        && first_contact_runtime_map_projection.map_w == 952
        && first_contact_runtime_map_projection.map_h == 476
        && first_contact_runtime_tile_rect_sample.x == 464
        && first_contact_runtime_tile_rect_sample.y == 278
        && first_contact_runtime_tile_rect_sample.width == 28
        && first_contact_runtime_tile_rect_sample.height == 14
        && first_contact_runtime_terrain_seed_sample.surface_seed == 12
        && first_contact_runtime_terrain_seed_sample.detail_seed == 20;
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
        && command_panel_palette_state_label == "ACTIVE"
        && command_panel_sidebar_queue_summary == "WORKER 42% TOWER 66%"
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
        && control_group_hotkey_feedback_stage.as_deref() == Some("double_tap_camera")
        && first_contact_runtime_review_gate
        && first_contact_online_protocol_gate
        && first_contact_online_local_handoff_gate
        && first_contact_online_offline_adapter_gate
        && first_contact_map_model_gate
        && first_contact_opening_profile_gate
        && first_contact_command_feedback_gate
        && first_contact_player_startup_gate
        && first_contact_actor_presentation_gate
        && first_contact_visual_telemetry_gate
        && first_contact_preview_actor_projection_gate
        && first_contact_player_screen_profile_gate
        && first_contact_terrain_profile_gate
        && first_contact_renderer_projection_gate
        && first_contact_runtime_map_projection_gate;

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
        first_contact_player_screen_application: first_contact_player_screen_application.clone(),
        first_contact_offline_adapter_application: first_contact_offline_adapter_application
            .clone(),
        first_contact_offline_adapter_consumption_review: first_contact_consumption_review
            .clone(),
        first_contact_offline_adapter_session_transition_review:
            first_contact_session_transition_review.clone(),
        first_contact_offline_adapter_lobby_ready_review: first_contact_lobby_ready_review
            .clone(),
        first_contact_online_protocol_fixture: first_contact_online_protocol_fixture.clone(),
        first_contact_online_local_handoff: first_contact_online_local_handoff.clone(),
        first_contact_online_offline_adapter: first_contact_adapter.clone(),
        first_contact_online_protocol_gate,
        first_contact_online_local_handoff_gate,
        first_contact_online_offline_adapter_gate,
        first_contact_map_model_review,
        first_contact_map_model_gate,
        first_contact_opening_profile: first_contact_opening_profile.clone(),
        first_contact_opening_profile_gate,
        first_contact_command_feedback_profile: first_contact_command_feedback_profile.clone(),
        first_contact_command_feedback_gate,
        first_contact_player_startup_profiles: first_contact_player_startup_profiles.clone(),
        first_contact_player_startup_gate,
        first_contact_actor_presentation_profiles: first_contact_actor_presentation_profiles
            .clone(),
        first_contact_actor_presentation_gate,
        first_contact_visual_telemetry_profile: first_contact_visual_telemetry_profile.clone(),
        first_contact_visual_telemetry_gate,
        first_contact_preview_actor_projection,
        first_contact_preview_actor_projection_gate,
        first_contact_player_screen_profile: first_contact_profile.clone(),
        first_contact_player_screen_layout_gate,
        first_contact_player_screen_chrome_gate,
        first_contact_player_screen_profile_gate,
        first_contact_terrain_profile_count,
        first_contact_terrain_profile_samples,
        first_contact_terrain_profile_gate,
        first_contact_renderer_projection,
        first_contact_renderer_projection_gate,
        first_contact_runtime_map_projection,
        first_contact_runtime_tile_rect_sample,
        first_contact_runtime_terrain_seed_sample,
        first_contact_runtime_map_projection_gate,
        first_contact_player_screen_application_contract: first_contact_player_screen_application
            .contract_version,
        first_contact_player_screen_application_green: first_contact_player_screen_application
            .green,
        first_contact_offline_adapter_application_contract:
            first_contact_offline_adapter_application.contract_version,
        first_contact_offline_adapter_application_green:
            first_contact_offline_adapter_application.green,
        first_contact_offline_adapter_consumption_contract: first_contact_consumption_review
            .contract_version,
        first_contact_offline_adapter_consumption_green: first_contact_consumption_review.green,
        first_contact_offline_adapter_session_transition_contract:
            first_contact_session_transition_review.contract_version,
        first_contact_offline_adapter_session_transition_green:
            first_contact_session_transition_review.green,
        first_contact_offline_adapter_lobby_ready_contract: first_contact_lobby_ready_review
            .contract_version,
        first_contact_offline_adapter_lobby_ready_green: first_contact_lobby_ready_review.green,
        first_contact_runtime_review_contracts,
        first_contact_runtime_review_before_command_queue_sample:
            first_contact_session_transition_review.before_command_queue,
        first_contact_runtime_review_after_command_queue_sample:
            first_contact_session_transition_review.after_command_queue,
        first_contact_runtime_review_ready_state_labels_sample: first_contact_lobby_ready_review
            .ready_state_labels,
        first_contact_runtime_review_command_stamp_tile_sample: first_contact_consumption_review
            .runtime_command_stamp_tile_id,
        first_contact_runtime_review_gate,
        source_of_truth: "The RTS evidence crate verifies the Bevy-free runtime adapter contract using deterministic First Contact minimap, scrollable camera/minimap sync, path preview, formation move preview/execution, local obstruction recovery, scene stage semantics, structure/environment stage semantics, harvest/production animation stage semantics, action cadence marks, action sequence phase/marks, unit-model depth marks, command-surface stage, command-grid, tile-line raster, combat-target, ability-effect, AI-pressure, recon-intel, base-assault, aftermath, commander-progression, expansion-counterattack, army-production/rally, siege breach counterplay, inner-lane breakthrough, central-keep, restoration/open-world handoff, economy/tech placement, queue economy, blocked-feedback chip visibility, scripted-demo timeline, selection roster, control-group roster, command parsing, command stamp semantics, order-queue replay actions, command feedback lifecycle/history/execution target labels and tiles, hover/cursor affordance, overlay stage/portrait semantics, objective, terrain-route, siege-route, and First Contact player-screen/offline-adapter application, consumption, session-transition, and lobby-ready review samples before trnm-world-bevy includes the proof in release-review evidence.".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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
    fn continuous_player_flow_review_preserves_six_step_screen_gates() {
        let input = json!({
            "contract_version": "trillionnium_world_bevy_classic_rts_continuous_player_flow_v1",
            "green": true,
            "preview_format": "ppm_p3_rgb",
            "preview_width": 1600,
            "preview_height": 900,
            "source_contracts": {
                "shell_meta_ui_replication": "trillionnium_world_bevy_classic_rts_shell_meta_ui_replication_v1",
                "match_setup_ui_replication": "trillionnium_world_bevy_classic_rts_match_setup_ui_replication_v1",
                "in_match_hud_state_replication": "trillionnium_world_bevy_classic_rts_in_match_hud_state_replication_v1",
                "production_interaction_polish": "trillionnium_world_bevy_classic_rts_production_interaction_polish_v1",
                "session_state_continuity": "trillionnium_world_bevy_classic_rts_session_state_continuity_v1",
                "campaign_outcome_ui_readiness": "trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness_v1",
                "campaign_ui_continuity": "trillionnium_world_bevy_classic_rts_campaign_ui_continuity_v1"
            },
            "runtime_screen_mode": "player_runtime_continuous_player_flow_screen",
            "runtime_screen_gate": true,
            "evidence_board_only": false,
            "continuous_player_flow_steps": [
                {"step_id": "title_account", "runtime_screen_mode": "player_runtime_shell_meta_screen"},
                {"step_id": "match_setup", "runtime_screen_mode": "player_runtime_match_setup_screen"},
                {"step_id": "in_match_hud", "runtime_screen_mode": "player_runtime_in_match_hud_screen"},
                {"step_id": "command_feedback", "runtime_screen_mode": "player_runtime_command_interaction_screen"},
                {"step_id": "save_load_resume", "runtime_screen_mode": "player_runtime_session_resume_screen"},
                {"step_id": "outcome_open_world", "runtime_screen_mode": "player_runtime_campaign_outcome_screen"}
            ],
            "continuous_player_flow_step_count": 6,
            "transition_sequence": [
                "title_account",
                "match_setup",
                "in_match_hud",
                "command_feedback",
                "save_load_resume",
                "outcome_open_world"
            ],
            "flow_pixel_counts": {
                "non_background": 250001,
                "board": 100001,
                "title_account": 2001,
                "match_setup": 2001,
                "in_match_hud": 2001,
                "command_feedback": 2001,
                "save_load_resume": 2001,
                "outcome_open_world": 2001,
                "lane": 501,
                "highlight": 1001,
                "player_first_flow_view_non_background": 300001,
                "player_first_flow_view_frame": 8001,
                "player_first_flow_status_strip": 10001,
                "player_first_flow_stage_rail": 50001
            },
            "source_headline": {
                "shell_meta_surface_count": 12,
                "match_setup_map_id": "first_contact_basin",
                "match_setup_faction_id": "mirror_guard",
                "hud_surface_count": 8,
                "hud_army_supply_used": 9,
                "interaction_surface_count": 6,
                "session_final_objective_status": "first_playable_loop_complete",
                "session_open_world_state": "resumed:league-coliseum",
                "campaign_outcome_open_world_state": "resumed:league-coliseum",
                "campaign_continuity_restored_room_id": "league-coliseum"
            },
            "title_account_gate": true,
            "match_setup_gate": true,
            "in_match_hud_gate": true,
            "command_feedback_gate": true,
            "save_resume_gate": true,
            "outcome_open_world_gate": true,
            "continuous_player_flow_chain_gate": true,
            "source_preview_gate": true,
            "preview_gate": true,
            "player_first_continuous_flow_screen_gate": true,
            "native_client_boundary_gate": true,
            "continuous_player_flow_gate": true,
            "external_evidence_ignored_for_current_replication_pass": true,
            "android_s5_real_device_claimed": false,
            "public_launch_ready": false,
            "production_ready_ui_claimed": false,
            "screen_for_screen_openra_ui_claimed": false,
            "openra_engine_port_claimed": false,
            "warcraft_iii_asset_copied": false,
            "openra_asset_copied": false,
            "third_party_asset_copied": false
        });

        let review = rts_continuous_player_flow_review(&input);

        assert_eq!(
            review.contract_version,
            TRNM_RTS_EVIDENCE_CONTINUOUS_PLAYER_FLOW_REVIEW_CONTRACT
        );
        assert!(review.green);
        assert!(review.source_contract_gate);
        assert!(review.transition_sequence_gate);
        assert!(review.source_headline_gate);
        assert!(review.pixel_gate);
        assert!(review.continuous_player_flow_chain_gate);
        assert!(review.player_first_continuous_flow_screen_gate);
        assert!(review.no_credit_boundary_gate);
        assert!(review.continuous_player_flow_gate);
        assert_eq!(
            review.session_open_world_state.as_deref(),
            Some("resumed:league-coliseum")
        );
        assert!(review
            .source_of_truth
            .contains("six-step continuous player flow"));
    }

    #[test]
    fn live_session_playthrough_review_preserves_trace_and_screen_gates() {
        let input = json!({
            "contract_version": "trillionnium_world_bevy_classic_rts_live_session_playthrough_v1",
            "status": "classic_rts_live_session_playthrough_green",
            "green": true,
            "preview_format": "ppm_p3_rgb",
            "preview_width": 1600,
            "preview_height": 900,
            "trace_path": "/tmp/bevy-classic-rts-live-session-playthrough.trace.json",
            "trace_write_gate": true,
            "trace_seed": "classic_rts_live_session_seed_v1",
            "same_process_session_playthrough": true,
            "runtime_screen_mode": "player_runtime_live_session_playthrough_screen",
            "runtime_screen_gate": true,
            "evidence_board_only": false,
            "stage_count": 6,
            "stage_ids": [
                "title_account",
                "match_setup",
                "in_match_hud",
                "command_feedback",
                "save_load_resume",
                "outcome_open_world"
            ],
            "stage_summaries": [
                {"step_id": "title_account", "turn": 2, "input_feedback_event_count": 2},
                {"step_id": "match_setup", "turn": 3, "input_feedback_event_count": 73},
                {"step_id": "in_match_hud", "turn": 3, "input_feedback_event_count": 73},
                {"step_id": "command_feedback", "turn": 8, "input_feedback_event_count": 78},
                {"step_id": "save_load_resume", "turn": 13, "input_feedback_event_count": 83},
                {"step_id": "outcome_open_world", "turn": 13, "input_feedback_event_count": 83}
            ],
            "top_level_action_count": 12,
            "top_level_accepted_action_count": 12,
            "accepted_input_count": 91,
            "campaign_handoff_input_count": 70,
            "live_command_input_count": 5,
            "slot_a_bytes": 10001,
            "pixel_counts": {
                "non_background": 300001,
                "title_account": 1001,
                "match_setup": 1001,
                "in_match_hud": 1001,
                "command_feedback": 1001,
                "save_load_resume": 1001,
                "outcome_open_world": 1001,
                "player_first_live_view_non_background": 250001,
                "player_first_live_view_frame": 8001,
                "player_first_live_status_strip": 10001,
                "player_first_live_stage_rail": 25001
            },
            "final_state": {
                "current_room_id": "league-coliseum",
                "map_scene": "arena_league_coliseum",
                "objective_status": "open_world_after_action_ready",
                "open_world_handoff_state": "resumed:league-coliseum",
                "open_world_resume_room_id": "league-coliseum",
                "contextual_primary_action_label": "COMBAT:attack"
            },
            "title_account_gate": true,
            "match_setup_gate": true,
            "in_match_hud_gate": true,
            "command_feedback_gate": true,
            "save_resume_gate": true,
            "outcome_open_world_gate": true,
            "same_process_trace_gate": true,
            "player_first_live_session_screen_gate": true,
            "preview_gate": true,
            "native_client_boundary_gate": true,
            "live_session_playthrough_gate": true,
            "external_evidence_ignored_for_current_playtest_pass": true,
            "android_s5_real_device_claimed": false,
            "public_launch_ready": false,
            "production_ready_ui_claimed": false,
            "screen_for_screen_openra_ui_claimed": false,
            "openra_engine_port_claimed": false,
            "warcraft_iii_asset_copied": false,
            "openra_asset_copied": false,
            "third_party_asset_copied": false,
            "cex_runtime_player_client_allowed": false,
            "wgpu_required": false
        });

        let review = rts_live_session_playthrough_review(&input);

        assert_eq!(
            review.contract_version,
            TRNM_RTS_EVIDENCE_LIVE_SESSION_PLAYTHROUGH_REVIEW_CONTRACT
        );
        assert!(review.green);
        assert!(review.source_contract_gate);
        assert!(review.stage_sequence_gate);
        assert!(review.trace_sidecar_gate);
        assert!(review.same_process_trace_gate);
        assert!(review.live_command_gate);
        assert!(review.save_resume_gate);
        assert!(review.final_state_gate);
        assert!(review.pixel_gate);
        assert!(review.player_first_live_session_screen_gate);
        assert!(review.native_client_boundary_gate);
        assert!(review.no_credit_boundary_gate);
        assert!(review.live_session_playthrough_gate);
        assert_eq!(review.accepted_input_count, 91);
        assert_eq!(
            review.final_open_world_handoff_state.as_deref(),
            Some("resumed:league-coliseum")
        );
        assert!(review
            .source_of_truth
            .contains("same-process local live session playthrough"));
    }

    #[test]
    fn full_game_visual_ui_replication_review_preserves_aggregate_gates() {
        let input = json!({
            "contract_version": "trillionnium_world_bevy_classic_rts_full_game_visual_ui_replication_v1",
            "status": "classic_rts_full_game_visual_ui_replication_green",
            "green": true,
            "preview_width": 1920,
            "preview_height": 1080,
            "preview_format": "ppm_p3_rgb",
            "runtime_screen_mode": "player_runtime_full_game_visual_ui_screen",
            "runtime_screen_gate": true,
            "evidence_board_only": false,
            "coverage_surface_count": 18,
            "coverage_surface_names": [
                "title_account_shell",
                "character_create",
                "session_slot_save_load",
                "match_setup_start",
                "tactical_viewport",
                "map_minimap_camera",
                "resource_topbar",
                "selection_unit_status",
                "command_grid",
                "ability_tooltip_telegraph",
                "production_queue",
                "build_tech_tree",
                "command_feedback",
                "formation_path_preview",
                "combat_alerts",
                "campaign_outcome",
                "open_world_handoff",
                "observability_boundary"
            ],
            "source_contracts": {
                "visual_fidelity": "trillionnium_world_bevy_classic_rts_visual_fidelity_v1",
                "production_art_replication": "trillionnium_world_bevy_classic_rts_production_art_replication_v1",
                "production_asset_atlas": "trillionnium_world_bevy_classic_rts_production_asset_atlas_v1",
                "production_ui_skin": "trillionnium_world_bevy_classic_rts_production_ui_skin_v1",
                "production_interaction_polish": "trillionnium_world_bevy_classic_rts_production_interaction_polish_v1",
                "full_screen_ui_replication": "trillionnium_world_bevy_classic_rts_full_screen_ui_replication_v1",
                "shell_meta_ui_replication": "trillionnium_world_bevy_classic_rts_shell_meta_ui_replication_v1",
                "match_setup_ui_replication": "trillionnium_world_bevy_classic_rts_match_setup_ui_replication_v1",
                "in_match_hud_state_replication": "trillionnium_world_bevy_classic_rts_in_match_hud_state_replication_v1",
                "session_state_continuity": "trillionnium_world_bevy_classic_rts_session_state_continuity_v1",
                "continuous_player_flow": "trillionnium_world_bevy_classic_rts_continuous_player_flow_v1",
                "live_session_playthrough": "trillionnium_world_bevy_classic_rts_live_session_playthrough_v1",
                "command_surface": "trillionnium_world_bevy_classic_rts_command_surface_v1",
                "command_affordance": "trillionnium_world_bevy_classic_rts_command_affordance_v1"
            },
            "source_review_contracts": {
                "session_state_continuity": "trnm_rts_evidence_session_state_continuity_review_v1",
                "continuous_player_flow": "trnm_rts_evidence_continuous_player_flow_review_v1",
                "live_session_playthrough": "trnm_rts_evidence_live_session_playthrough_review_v1"
            },
            "source_review_gates": {
                "session_state_continuity": true,
                "continuous_player_flow": true,
                "live_session_playthrough": true
            },
            "source_review_sources": {
                "session_state_continuity": "The RTS evidence crate reviews save-slot confirmation and resume gates.",
                "continuous_player_flow": "The RTS evidence crate reviews the six-step continuous player flow.",
                "live_session_playthrough": "The RTS evidence crate reviews the same-process local live session playthrough."
            },
            "pixel_counts": {
                "non_background": 2073600,
                "hud_chrome": 276317,
                "shell_session": 10018,
                "match_setup": 6581,
                "hud": 3414,
                "command": 42590,
                "session": 39312,
                "outcome": 26546,
                "tech": 2824,
                "minimap": 2242,
                "highlight": 6752,
                "player_first_tactical_preview_non_background": 570458,
                "player_first_tactical_viewport_frame": 16704,
                "player_first_tactical_status_strip": 21375
            },
            "source_headline": {
                "full_screen_surface_count": 10,
                "shell_meta_surface_count": 12,
                "match_setup_surface_count": 10,
                "hud_surface_count": 8,
                "continuous_step_count": 6,
                "live_session_stage_count": 6,
                "live_session_accepted_input_count": 91,
                "live_session_final_objective_status": "open_world_after_action_ready",
                "live_session_open_world_state": "resumed:league-coliseum",
                "live_session_runtime_screen_mode": "player_runtime_live_session_playthrough_screen",
                "live_session_runtime_screen_gate": true,
                "production_ui_runtime_screen_mode": "player_runtime_production_hud_skin_screen",
                "interaction_runtime_screen_mode": "player_runtime_command_interaction_screen",
                "command_surface_ready_pixel_count": 1890,
                "command_affordance_hotkey_pixel_count": 3600
            },
            "source_green_gate": true,
            "runtime_screen_chain_gate": true,
            "player_flow_gate": true,
            "coverage_surface_gate": true,
            "preview_gate": true,
            "player_first_tactical_composition_gate": true,
            "player_first_full_game_visual_ui_screen_gate": true,
            "no_copy_boundary_gate": true,
            "full_game_visual_ui_replication_gate": true,
            "internal_rust_full_game_visual_ui_replication_claimed": true,
            "external_evidence_ignored_for_current_replication_pass": true,
            "android_s5_real_device_claimed": false,
            "public_launch_ready": false,
            "production_ready_ui_claimed": false,
            "screen_for_screen_openra_ui_claimed": false,
            "openra_engine_port_claimed": false,
            "warcraft_iii_asset_copied": false,
            "openra_asset_copied": false,
            "third_party_asset_copied": false
        });

        let review = rts_full_game_visual_ui_replication_review(&input);

        assert!(review.green);
        assert!(review.source_contract_gate);
        assert!(review.source_review_gate);
        assert!(review.source_headline_gate);
        assert!(review.player_first_full_game_visual_ui_screen_gate);
        assert_eq!(review.coverage_surface_count, 18);
        assert_eq!(
            review.live_session_final_objective_status.as_deref(),
            Some("open_world_after_action_ready")
        );
        assert_eq!(
            review.live_session_playthrough_review_contract.as_deref(),
            Some(TRNM_RTS_EVIDENCE_LIVE_SESSION_PLAYTHROUGH_REVIEW_CONTRACT)
        );
        assert!(review
            .source_of_truth
            .contains("full-game visual/UI replication aggregate"));
    }

    #[test]
    fn openra_style_screen_set_review_preserves_no_overclaim_gates() {
        let input = json!({
            "contract_version": "trillionnium_world_bevy_classic_rts_openra_screen_for_screen_ui_replication_v1",
            "status": "classic_rts_openra_screen_for_screen_ui_replication_green",
            "green": true,
            "preview_width": 1920,
            "preview_height": 1080,
            "preview_format": "ppm_p3_rgb",
            "screen_for_screen_mode": "openra_style_widget_root_screen_set_and_interaction_surface_replication_original_trillionnium_art",
            "runtime_screen_mode": "player_runtime_openra_style_ingame_screen_set",
            "runtime_screen_gate": true,
            "evidence_board_only": false,
            "openra_widget_roots": [
                "ShellmapRoot=MAINMENU",
                "IngameRoot=INGAME_ROOT",
                "GameSaveLoadingRoot=GAMESAVE_LOADING_SCREEN",
                "EditorRoot=EDITOR_ROOT"
            ],
            "openra_widget_root_count": 4,
            "openra_reference_screens": [
                "MAINMENU_shellmap_root",
                "SKIRMISH_mission_browser",
                "MULTIPLAYER_server_browser",
                "LOBBY_setup_room",
                "LOADING_briefing_progress",
                "INGAME_ROOT_sidebar_hud",
                "PAUSE_options_overlay",
                "POSTGAME_statistics"
            ],
            "openra_reference_screen_count": 8,
            "replicated_interaction_surfaces": [
                "shellmap_menu_stack",
                "mission_map_list",
                "server_filter_table",
                "lobby_player_slots",
                "loading_briefing_progress",
                "ingame_viewport_sidebar_minimap",
                "pause_settings_overlay",
                "postgame_score_tabs"
            ],
            "replicated_interaction_surface_count": 8,
            "source_contracts": {
                "full_game_visual_ui_replication": "trillionnium_world_bevy_classic_rts_full_game_visual_ui_replication_v1",
                "full_screen_ui_replication": "trillionnium_world_bevy_classic_rts_full_screen_ui_replication_v1",
                "shell_meta_ui_replication": "trillionnium_world_bevy_classic_rts_shell_meta_ui_replication_v1",
                "match_setup_ui_replication": "trillionnium_world_bevy_classic_rts_match_setup_ui_replication_v1",
                "in_match_hud_state_replication": "trillionnium_world_bevy_classic_rts_in_match_hud_state_replication_v1",
                "session_state_continuity": "trillionnium_world_bevy_classic_rts_session_state_continuity_v1",
                "openra_like_core": "trillionnium_world_bevy_classic_rts_openra_like_core_v1",
                "openra_parity_lane": "trillionnium_world_bevy_classic_rts_openra_parity_lane_v1"
            },
            "pixel_counts": {
                "non_background": 1400000,
                "mainmenu": 12000,
                "skirmish": 12000,
                "server_browser": 12000,
                "lobby": 12000,
                "loading": 12000,
                "ingame": 45000,
                "pause": 12000,
                "postgame_stats": 12000,
                "active_highlight": 7000
            },
            "openra_style_ingame_pixel_counts": {
                "player_first_openra_style_ingame_view_non_background": 80000,
                "player_first_openra_style_ingame_sidebar_non_background": 35000,
                "player_first_openra_style_ingame_command_lane_non_background": 6000,
                "player_first_openra_style_ingame_control_color": 45000,
                "player_first_openra_style_active_highlight": 7000
            },
            "source_headline": {
                "full_game_surface_count": 18,
                "full_game_internal_claimed": true,
                "full_screen_surface_count": 10,
                "shell_meta_surface_count": 12,
                "match_setup_surface_count": 10,
                "hud_surface_count": 8,
                "session_surface_count": 8,
                "openra_like_runtime_model": "rust_bevy_owned_openra_like_rts_core",
                "openra_parity_lane_axis_count": 6
            },
            "source_contract_gate": true,
            "source_green_gate": true,
            "openra_runtime_vocabulary_gate": true,
            "widget_root_reference_gate": true,
            "screen_set_gate": true,
            "source_screen_chain_gate": true,
            "preview_gate": true,
            "no_asset_copy_boundary_gate": true,
            "player_first_openra_style_ingame_screen_gate": true,
            "openra_style_ui_screen_set_replication_gate": true,
            "openra_screen_for_screen_ui_replication_gate": true,
            "openra_style_widget_root_screen_set_claimed": true,
            "screen_for_screen_openra_ui_claimed": false,
            "openra_screen_for_screen_ui_replication_claimed": false,
            "openra_pixel_perfect_asset_parity_claimed": false,
            "openra_engine_port_claimed": false,
            "openra_asset_copied": false,
            "warcraft_iii_asset_copied": false,
            "third_party_asset_copied": false,
            "bevy_openra_runtime_parity_claimed": false,
            "bevy_openra_replay_file_claimed": false,
            "android_s5_real_device_claimed": false,
            "public_launch_ready": false
        });

        let review = rts_openra_style_screen_set_review(&input);

        assert!(review.green);
        assert!(review.source_contract_gate);
        assert!(review.widget_root_reference_gate);
        assert!(review.screen_set_gate);
        assert!(review.player_first_openra_style_ingame_screen_gate);
        assert!(review.no_credit_boundary_gate);
        assert_eq!(
            review.contract_version,
            TRNM_RTS_EVIDENCE_OPENRA_STYLE_SCREEN_SET_REVIEW_CONTRACT
        );
        assert_eq!(review.openra_reference_screen_count, 8);
        assert!(review.source_of_truth.contains("OpenRA-style screen-set"));
    }

    #[test]
    fn release_review_packet_assembly_review_preserves_manifest_handoff_gates() {
        fn packet_artifact(id: &str, role: &str) -> Value {
            json!({
                "id": id,
                "label": id,
                "path": format!("/tmp/{id}.json"),
                "role": role,
                "file_status": "present",
                "sha256": "a".repeat(64),
                "bytes": 128,
                "contract_version": "fixture_contract_v1",
                "status": "fixture_green"
            })
        }

        let runtime_ids = [
            "native_bevy_classic_rts_first_contact_basin_spec",
            "native_bevy_classic_rts_campaign_ui_continuity",
            "native_bevy_classic_rts_campaign_ui_continuity_ppm",
            "native_bevy_classic_rts_in_match_hud_state_replication",
            "native_bevy_classic_rts_session_state_continuity",
            "native_bevy_classic_rts_combat_readability_pressure_readiness",
            "native_bevy_classic_rts_full_game_visual_ui_replication",
            "native_bevy_classic_rts_full_game_visual_ui_replication_ppm",
            "native_bevy_classic_playtest_readiness",
            "native_bevy_classic_playtest_runner_status",
            "native_bevy_classic_playtest_launcher",
            "native_bevy_classic_playtest_handoff_packet",
            "release_review_convergence",
            "release_review_checkpoint_manifest",
            "release_review_status_json",
            "release_review_quickcheck",
        ];
        let fixture_ids = [
            "release_review_packet_integrity_semantic_fixture",
            "release_review_packet_integrity_bot_executor_semantic_fixture",
            "release_review_packet_integrity_bot_executor_matrix_semantic_fixture",
            "release_review_packet_integrity_bot_gap_semantic_fixture",
            "release_review_packet_integrity_control_loop_semantic_fixture",
            "release_review_packet_integrity_selection_minimap_semantic_fixture",
            "release_review_packet_integrity_build_lifecycle_semantic_fixture",
            "release_review_packet_integrity_tech_tree_semantic_fixture",
            "release_review_packet_integrity_projectile_ability_semantic_fixture",
        ];
        let mut artifacts = Vec::new();
        for id in runtime_ids {
            let role = if id.ends_with("_ppm") {
                "release_review_visual_evidence"
            } else {
                "release_review_input"
            };
            artifacts.push(packet_artifact(id, role));
        }
        for id in fixture_ids {
            artifacts.push(packet_artifact(id, "release_review_gate"));
        }
        while artifacts.len() < 128 {
            artifacts.push(packet_artifact(
                &format!("fixture_release_review_input_{}", artifacts.len()),
                "release_review_input",
            ));
        }
        for artifact in &mut artifacts {
            if artifact.get("id").and_then(Value::as_str)
                == Some("native_bevy_classic_rts_full_game_visual_ui_replication")
            {
                artifact["contract_version"] =
                    json!("trillionnium_world_bevy_classic_rts_full_game_visual_ui_replication_v1");
                artifact["status"] = json!("classic_rts_full_game_visual_ui_replication_green");
            }
        }
        let ready_items = (0..13)
            .map(|index| json!({"label": format!("ready_{index}"), "ready": true}))
            .collect::<Vec<_>>();
        let blocked_items = (0..6)
            .map(|index| json!({"label": format!("blocked_{index}"), "needed": "real evidence"}))
            .collect::<Vec<_>>();
        let release_review_input_count = artifacts
            .iter()
            .filter(|artifact| {
                artifact.get("role").and_then(Value::as_str) == Some("release_review_input")
            })
            .count() as u64;
        let release_review_visual_evidence_count = artifacts
            .iter()
            .filter(|artifact| {
                artifact.get("role").and_then(Value::as_str)
                    == Some("release_review_visual_evidence")
            })
            .count() as u64;
        let release_review_gate_count = artifacts
            .iter()
            .filter(|artifact| {
                artifact.get("role").and_then(Value::as_str) == Some("release_review_gate")
            })
            .count() as u64;
        let packet = json!({
            "contract_version": "trillionnium_world_release_review_packet_v1",
            "status": "release_review_packet_ready_with_public_launch_blockers",
            "artifact_count": artifacts.len() as u64,
            "release_review_input_count": release_review_input_count,
            "release_review_visual_evidence_count": release_review_visual_evidence_count,
            "release_review_recording_count": 0,
            "release_review_collection_count": 0,
            "release_review_gate_count": release_review_gate_count,
            "release_review_operator_handoff_count": 0,
            "release_review_checkpoint_count": 0,
            "release_review_checklist_count": 0,
            "release_review_log_count": 0,
            "missing_artifact_count": 0,
            "reviewed_runtime_artifact_count": runtime_ids.len() as u64,
            "reviewed_packet_fixture_count": fixture_ids.len() as u64,
            "ready_for_release_review": true,
            "public_launch_ready": false,
            "android_s5_real_device_claimed": false,
            "proof_scope": "host_side_bevy_runtime_replay_not_android_real_device",
            "convergence_status": "release_review_convergence_green_with_public_launch_blockers",
            "status_checklist_status": "release_review_ready_public_launch_blocked",
            "missing_artifacts": [],
            "ready_items": ready_items,
            "blocked_items": blocked_items,
            "reviewer_next_action": "collect_real_external_public_launch_evidence",
            "artifacts": artifacts
        });

        let review = rts_release_review_packet_assembly_review(&packet);

        assert_eq!(
            review.contract_version,
            TRNM_RTS_EVIDENCE_RELEASE_REVIEW_PACKET_ASSEMBLY_REVIEW_CONTRACT
        );
        assert!(review.green);
        assert_eq!(review.artifact_count, 128);
        assert_eq!(review.packet_integrity_fixture_count, 9);
        assert_eq!(
            review.reviewed_runtime_artifact_count,
            runtime_ids.len() as u64
        );
        assert_eq!(
            review.reviewed_packet_fixture_count,
            fixture_ids.len() as u64
        );
        assert_eq!(review.missing_artifact_count, 0);
        assert_eq!(review.ready_item_count, 13);
        assert_eq!(review.blocked_item_count, 6);
        assert!(review.inventory_summary_gate);
        assert!(review.artifact_manifest_gate);
        assert!(review.missing_artifacts_gate);
        assert!(review.release_review_readiness_gate);
        assert!(review.status_handoff_gate);
        assert!(review.key_runtime_artifacts_gate);
        assert!(review.full_game_visual_ui_handoff_gate);
        assert!(review.packet_integrity_fixture_gate);
        assert!(review.public_launch_boundary_gate);
        assert!(review.external_blocker_gate);
        assert!(review
            .reviewed_runtime_artifact_ids
            .contains(&"native_bevy_classic_rts_session_state_continuity".to_string()));
        assert!(review
            .reviewed_runtime_artifact_ids
            .contains(&"native_bevy_classic_rts_full_game_visual_ui_replication".to_string()));
        assert!(review
            .source_of_truth
            .contains("top-level inventory summary"));
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
            evidence.first_contact_player_screen_application_contract,
            TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_PLAYER_SCREEN_APPLICATION_CONTRACT
        );
        assert!(evidence.first_contact_player_screen_application_green);
        assert_eq!(
            evidence
                .first_contact_player_screen_application
                .contract_version,
            evidence.first_contact_player_screen_application_contract
        );
        assert!(evidence.first_contact_player_screen_application.green);
        assert_eq!(
            evidence.first_contact_offline_adapter_application_contract,
            TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_APPLICATION_CONTRACT
        );
        assert!(evidence.first_contact_offline_adapter_application_green);
        assert_eq!(
            evidence
                .first_contact_offline_adapter_application
                .contract_version,
            evidence.first_contact_offline_adapter_application_contract
        );
        assert_eq!(
            evidence
                .first_contact_offline_adapter_application
                .command_queue,
            vec!["move:8,4"]
        );
        assert_eq!(
            evidence.first_contact_offline_adapter_consumption_contract,
            TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_CONSUMPTION_CONTRACT
        );
        assert!(evidence.first_contact_offline_adapter_consumption_green);
        assert_eq!(
            evidence
                .first_contact_offline_adapter_consumption_review
                .contract_version,
            evidence.first_contact_offline_adapter_consumption_contract
        );
        assert_eq!(
            evidence
                .first_contact_offline_adapter_consumption_review
                .runtime_command_stamp_tile_id
                .as_deref(),
            Some("8,4")
        );
        assert_eq!(
            evidence.first_contact_offline_adapter_session_transition_contract,
            TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_SESSION_TRANSITION_CONTRACT
        );
        assert!(evidence.first_contact_offline_adapter_session_transition_green);
        assert_eq!(
            evidence
                .first_contact_offline_adapter_session_transition_review
                .contract_version,
            evidence.first_contact_offline_adapter_session_transition_contract
        );
        assert_eq!(
            evidence
                .first_contact_offline_adapter_session_transition_review
                .after_command_queue,
            vec!["move:8,4"]
        );
        assert_eq!(
            evidence.first_contact_offline_adapter_lobby_ready_contract,
            TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_LOBBY_READY_CONTRACT
        );
        assert!(evidence.first_contact_offline_adapter_lobby_ready_green);
        assert_eq!(
            evidence
                .first_contact_offline_adapter_lobby_ready_review
                .contract_version,
            evidence.first_contact_offline_adapter_lobby_ready_contract
        );
        assert!(evidence
            .first_contact_offline_adapter_lobby_ready_review
            .ready_state_labels
            .contains(&"authority:offline_loopback:no_socket".to_string()));
        assert_eq!(
            evidence.first_contact_runtime_review_contracts,
            vec![
                TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_PLAYER_SCREEN_APPLICATION_CONTRACT,
                TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_APPLICATION_CONTRACT,
                TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_CONSUMPTION_CONTRACT,
                TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_SESSION_TRANSITION_CONTRACT,
                TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_LOBBY_READY_CONTRACT,
            ]
        );
        assert!(evidence
            .first_contact_runtime_review_before_command_queue_sample
            .contains(&"build:trnm.flux.relay".to_string()));
        assert_eq!(
            evidence.first_contact_runtime_review_after_command_queue_sample,
            vec!["move:8,4"]
        );
        assert!(evidence
            .first_contact_runtime_review_ready_state_labels_sample
            .contains(&"authority:offline_loopback:no_socket".to_string()));
        assert_eq!(
            evidence
                .first_contact_runtime_review_command_stamp_tile_sample
                .as_deref(),
            Some("8,4")
        );
        assert!(evidence.first_contact_runtime_review_gate);
        assert!(evidence.first_contact_online_protocol_gate);
        assert_eq!(
            evidence
                .first_contact_online_protocol_fixture
                .envelope
                .map_id,
            "first_contact_basin"
        );
        assert_eq!(
            evidence
                .first_contact_online_protocol_fixture
                .envelope
                .scope
                .visible_chunks
                .len(),
            3
        );
        assert!(evidence.first_contact_online_local_handoff_gate);
        assert_eq!(
            evidence.first_contact_online_local_handoff.map_id,
            "first_contact_basin"
        );
        assert_eq!(
            evidence
                .first_contact_online_local_handoff
                .accepted_order_count,
            1
        );
        assert!(evidence.first_contact_online_offline_adapter_gate);
        assert_eq!(
            evidence.first_contact_online_offline_adapter.adapter_mode,
            "offline_loopback_authority"
        );
        assert_eq!(
            evidence
                .first_contact_online_offline_adapter
                .local_runtime_handoff
                .accepted_runtime_command_labels,
            vec!["move:8,4"]
        );
        assert!(evidence.first_contact_map_model_gate);
        assert!(evidence.first_contact_map_model_review.map_actor_gate);
        assert!(evidence.first_contact_map_model_review.map_topology_gate);
        assert!(evidence.first_contact_map_model_review.rules_gate);
        assert!(evidence.first_contact_map_model_review.data_consumer_gate);
        assert!(
            evidence
                .first_contact_map_model_review
                .map_model_adapter_gate
        );
        assert_eq!(
            evidence
                .first_contact_map_model_review
                .map_summary
                .actor_count,
            39
        );
        assert_eq!(
            evidence
                .first_contact_map_model_review
                .map_summary
                .source_integration_mode,
            "gpl_internal_component"
        );
        assert_eq!(evidence.first_contact_map_model_review.unit_rule_count, 6);
        assert_eq!(
            evidence.first_contact_map_model_review.building_rule_count,
            5
        );
        assert_eq!(
            evidence
                .first_contact_map_model_review
                .data_validation_error,
            None
        );
        assert!(evidence.first_contact_opening_profile_gate);
        assert_eq!(
            evidence.first_contact_opening_profile.contract_version,
            trnm_rts_data::TRNM_RTS_DATA_FIRST_CONTACT_OPENING_PROFILE_CONTRACT
        );
        assert_eq!(
            evidence.first_contact_opening_profile.active_beacon_tile,
            trnm_rts_core::RtsTile::new(16, 9)
        );
        assert!(evidence.first_contact_command_feedback_gate);
        assert_eq!(
            evidence
                .first_contact_command_feedback_profile
                .contract_version,
            trnm_rts_data::TRNM_RTS_DATA_FIRST_CONTACT_COMMAND_FEEDBACK_CONTRACT
        );
        assert_eq!(
            evidence.first_contact_command_feedback_profile.target_tile,
            evidence.first_contact_opening_profile.active_beacon_tile
        );
        assert!(evidence.first_contact_player_startup_gate);
        assert_eq!(evidence.first_contact_player_startup_profiles.len(), 4);
        assert!(evidence
            .first_contact_player_startup_profiles
            .iter()
            .any(|startup| startup.player_id == "Multi0"
                && startup.faction == "horizon"
                && startup.spawn_tile == trnm_rts_core::RtsTile::new(8, 8)
                && startup.faction_unit_rule_id == "trnm.horizon.scout"));
        assert!(evidence.first_contact_actor_presentation_gate);
        assert!(evidence
            .first_contact_actor_presentation_profiles
            .iter()
            .any(|profile| profile.rule_id == "trnm.command.core"
                && profile.structure
                && profile.glyph.body == trnm_rts_data::RtsActorGlyphBody::Structure
                && profile.glyph.accent == trnm_rts_data::RtsActorGlyphAccent::CommandSpire));
        assert!(evidence.first_contact_visual_telemetry_gate);
        assert_eq!(
            evidence
                .first_contact_visual_telemetry_profile
                .contract_version,
            trnm_rts_data::TRNM_RTS_DATA_FIRST_CONTACT_VISUAL_TELEMETRY_CONTRACT
        );
        assert_eq!(
            evidence
                .first_contact_visual_telemetry_profile
                .unit_statuses
                .len(),
            4
        );
        assert!(evidence
            .first_contact_visual_telemetry_profile
            .tactical_tracks
            .iter()
            .any(
                |track| track.from_tile == trnm_rts_core::RtsTile::new(11, 8)
                    && track.to_tile == trnm_rts_core::RtsTile::new(16, 9)
            ));
        assert!(evidence.first_contact_preview_actor_projection_gate);
        assert_eq!(
            evidence.first_contact_preview_actor_projection.actor_count,
            39
        );
        assert_eq!(
            evidence.first_contact_preview_actor_projection.spawn_count,
            4
        );
        assert_eq!(
            evidence
                .first_contact_preview_actor_projection
                .flux_bloom_count,
            11
        );
        assert_eq!(
            evidence.first_contact_preview_actor_projection.beacon_count,
            4
        );
        assert_eq!(
            evidence
                .first_contact_preview_actor_projection
                .expansion_count,
            4
        );
        assert!(evidence
            .first_contact_preview_actor_projection
            .actor_samples
            .iter()
            .any(|actor| actor.source_actor_id == "Actor0"
                && actor.kind == trnm_rts_data::RtsFirstContactPreviewActorKind::Spawn
                && actor.openra_preview_rule_id == "trnm.map.detail"));
        assert!(evidence.first_contact_player_screen_layout_gate);
        assert!(evidence.first_contact_player_screen_chrome_gate);
        assert!(evidence.first_contact_player_screen_profile_gate);
        assert_eq!(
            evidence
                .first_contact_player_screen_profile
                .contract_version,
            trnm_rts_data::TRNM_RTS_DATA_FIRST_CONTACT_PLAYER_SCREEN_CONTRACT
        );
        assert_eq!(
            evidence.first_contact_player_screen_profile.map_id,
            "first_contact_basin"
        );
        assert_eq!(
            evidence
                .first_contact_player_screen_profile
                .layout
                .player_map
                .map_origin_x,
            16
        );
        assert_eq!(
            evidence
                .first_contact_player_screen_profile
                .chrome
                .command_grid_slot_ids,
            vec!["worker", "scout", "warden", "relay", "core", "signal"]
        );
        assert_eq!(
            evidence.first_contact_player_screen_profile.command_queue,
            vec![
                "move:16,9",
                "build:trnm.flux.relay",
                "train:trnm.worker",
                "attack:trnm.flux.beacon"
            ]
        );
        assert!(evidence.first_contact_terrain_profile_gate);
        assert_eq!(evidence.first_contact_terrain_profile_count, 1156);
        assert_eq!(
            evidence.first_contact_terrain_profile_samples.center.height,
            2
        );
        assert!(
            evidence
                .first_contact_terrain_profile_samples
                .resource_zone
                .resource_zone
        );
        assert!(evidence.first_contact_renderer_projection_gate);
        assert_eq!(
            evidence
                .first_contact_renderer_projection
                .renderable_tile_count,
            1024
        );
        assert_eq!(
            evidence.first_contact_renderer_projection.lane_tile_count,
            240
        );
        assert_eq!(
            evidence
                .first_contact_renderer_projection
                .minimap_anchor_actor_count,
            39
        );
        assert!(evidence
            .first_contact_renderer_projection
            .minimap_anchor_actor_samples
            .contains(&"Actor0".to_string()));
        assert!(evidence.first_contact_runtime_map_projection_gate);
        assert_eq!(
            evidence.first_contact_runtime_map_projection,
            RtsRuntimeMapProjection {
                map_x: 16,
                map_y: 54,
                cell_w: 28,
                cell_h: 14,
                map_w: 952,
                map_h: 476,
            }
        );
        assert_eq!(
            evidence.first_contact_runtime_tile_rect_sample,
            RtsRuntimeRect {
                x: 464,
                y: 278,
                width: 28,
                height: 14,
            }
        );
        assert_eq!(
            evidence.first_contact_runtime_terrain_seed_sample,
            RtsRuntimeTerrainSeeds {
                surface_seed: 12,
                detail_seed: 20,
            }
        );
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
        assert_eq!(evidence.command_panel_palette_state_label_sample, "ACTIVE");
        assert_eq!(
            evidence.command_panel_sidebar_queue_summary_sample,
            "WORKER 42% TOWER 66%"
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
