//! Bevy-free RTS runtime adapter math for camera, minimap, path preview, and UI hit tests.
//!
//! This crate is intentionally small: it owns deterministic adapter calculations while
//! `trnm-world-bevy` keeps renderer colors, pixels, assets, and Bevy integration.

use serde::{Deserialize, Serialize};
use trnm_rts_core::RtsTile;
use trnm_rts_data::RtsFirstContactPlayerScreenProfile;

pub const TRNM_RTS_BEVY_RUNTIME_CONTRACT: &str = "trnm_rts_bevy_runtime_adapter_v1";

pub const TRNM_RTS_RUNTIME_MAP_WIDTH_TILES: i32 = 34;
pub const TRNM_RTS_RUNTIME_MAP_HEIGHT_TILES: i32 = 34;
pub const TRNM_RTS_RUNTIME_MAP_MIN_TILE: i32 = 1;
pub const TRNM_RTS_RUNTIME_MAP_MAX_X: i32 = 32;
pub const TRNM_RTS_RUNTIME_MAP_MAX_Y: i32 = 32;
pub const TRNM_RTS_RUNTIME_CAMERA_ORIGIN_X: i32 = 17;
pub const TRNM_RTS_RUNTIME_CAMERA_ORIGIN_Y: i32 = 17;
pub const TRNM_RTS_RUNTIME_TILE_WORLD_W: f32 = 72.0;
pub const TRNM_RTS_RUNTIME_TILE_WORLD_H: f32 = 48.0;
pub const TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_CONSUMPTION_CONTRACT: &str =
    "trnm_rts_bevy_runtime_first_contact_offline_adapter_consumption_v1";
pub const TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_APPLICATION_CONTRACT: &str =
    "trnm_rts_bevy_runtime_first_contact_offline_adapter_runtime_application_v1";
pub const TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_SESSION_TRANSITION_CONTRACT: &str =
    "trnm_rts_bevy_runtime_first_contact_offline_adapter_session_transition_v1";
pub const TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_LOBBY_READY_CONTRACT: &str =
    "trnm_rts_bevy_runtime_first_contact_offline_adapter_lobby_ready_v1";
pub const TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_PLAYER_SCREEN_APPLICATION_CONTRACT: &str =
    "trnm_rts_bevy_runtime_first_contact_player_screen_application_v1";

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct RtsRuntimeVec2 {
    pub x: f32,
    pub y: f32,
}

impl RtsRuntimeVec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RtsScrollableMapCameraState {
    pub center_x: f32,
    pub center_y: f32,
    pub zoom: f32,
}

impl Default for RtsScrollableMapCameraState {
    fn default() -> Self {
        Self {
            center_x: 0.0,
            center_y: 0.0,
            zoom: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RtsScrollableMapCameraConfig {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub keyboard_speed: f32,
    pub edge_speed: f32,
    pub drag_world_units_per_pixel: f32,
    pub wheel_zoom_step: f32,
    pub edge_band_pixels: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RtsScrollableMapCameraStep {
    pub source: String,
    pub before: RtsScrollableMapCameraState,
    pub after: RtsScrollableMapCameraState,
    pub pan_delta_x: f32,
    pub pan_delta_y: f32,
    pub zoom_delta: f32,
    pub clamped: bool,
    pub minimap_tile_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RtsScrollableMapCameraStageSummary {
    pub stage: String,
    pub source: String,
    pub step: RtsScrollableMapCameraStep,
    pub focus_tile: (i32, i32),
    pub command_destination_tile: Option<String>,
    pub command_queue: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsCameraMinimapViewportRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RtsCameraMinimapSyncStageSummary {
    pub stage: String,
    pub source: String,
    pub step: RtsScrollableMapCameraStep,
    pub focus_tile: (i32, i32),
    pub viewport_rect: RtsCameraMinimapViewportRect,
    pub viewport_rect_area: i32,
    pub revealed_tile_ids: Vec<String>,
    pub selected_unit_id: String,
    pub control_group_id: String,
    pub command_destination_tile: Option<String>,
    pub minimap_command_tile_id: Option<String>,
    pub command_queue: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsRuntimeRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsUnitModelDepthMark {
    pub kind: String,
    pub rect: RtsRuntimeRect,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsActionCadenceMark {
    pub kind: String,
    pub rect: RtsRuntimeRect,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsActionSequenceMark {
    pub kind: String,
    pub rect: RtsRuntimeRect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsRuntimeGridSpec {
    pub origin_x: i32,
    pub origin_y: i32,
    pub columns: usize,
    pub count: usize,
    pub stride_x: i32,
    pub stride_y: i32,
    pub slot_width: i32,
    pub slot_height: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsCommandStamp {
    pub input_source: String,
    pub kind: String,
    pub tile_id: Option<String>,
    pub target_id: Option<String>,
    pub player_label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsOrderQueueReplayAction {
    pub kind: String,
    pub payload: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsCommandQueuePathPreviewStageFixture {
    pub stage: String,
    pub action: RtsOrderQueueReplayAction,
    pub history_entry: String,
    pub input_source: String,
    pub renderer_path: String,
    pub preview_surface: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsFormationMovePreviewStageFixture {
    pub stage: String,
    pub action: RtsOrderQueueReplayAction,
    pub history_entry: String,
    pub input_source: String,
    pub renderer_path: String,
    pub preview_surface: String,
    pub command_destination_tile: Option<String>,
    pub path_tile_ids: Vec<String>,
    pub blocked_tile_ids: Vec<String>,
    pub formation_slot_tile_ids: Vec<String>,
    pub disperse_tile_ids: Vec<String>,
    pub pathing_status: Option<String>,
    pub unit_response_state: Option<String>,
    pub group_route_tile_ids_if_empty: Vec<String>,
    pub group_command_state: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsControlGroupRecallFormationPreviewStageFixture {
    pub stage: String,
    pub action: RtsOrderQueueReplayAction,
    pub history_entry: String,
    pub input_source: String,
    pub renderer_path: String,
    pub preview_surface: String,
    pub control_group_id: String,
    pub active_control_group_ids: Vec<String>,
    pub selected_unit_ids: Vec<String>,
    pub stance: String,
    pub recall_focus_tile: String,
    pub formation_anchor_tile: String,
    pub path_tile_ids: Vec<String>,
    pub formation_slot_tile_ids: Vec<String>,
    pub queued_member_ids: Vec<String>,
    pub filtered_member_ids: Vec<String>,
    pub cleared_old_member_ids: Vec<String>,
    pub group_command_state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsControlGroupRecallOverridePreviewStageFixture {
    pub stage: String,
    pub action: RtsOrderQueueReplayAction,
    pub history_entry: String,
    pub input_source: String,
    pub renderer_path: String,
    pub preview_surface: String,
    pub control_group_id: String,
    pub active_control_group_ids: Vec<String>,
    pub selected_unit_ids: Vec<String>,
    pub stance: String,
    pub recall_focus_tile: String,
    pub queued_target_tile: String,
    pub canceled_target_tile: Option<String>,
    pub path_tile_ids: Vec<String>,
    pub group_route_tile_ids: Vec<String>,
    pub override_final_tile_ids: Vec<String>,
    pub queued_member_ids: Vec<String>,
    pub canceled_member_ids: Vec<String>,
    pub filtered_member_ids: Vec<String>,
    pub cleared_old_member_ids: Vec<String>,
    pub group_command_state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsFormationMoveExecutionStageFixture {
    pub stage: String,
    pub action: RtsOrderQueueReplayAction,
    pub history_entry: String,
    pub input_source: String,
    pub renderer_path: String,
    pub preview_surface: String,
    pub selected_unit_ids: Vec<String>,
    pub command_destination_tile: Option<String>,
    pub path_tile_ids: Vec<String>,
    pub blocked_tile_ids: Vec<String>,
    pub formation_slot_tile_ids: Vec<String>,
    pub disperse_tile_ids: Vec<String>,
    pub group_route_tile_ids: Vec<String>,
    pub pathing_status: String,
    pub unit_response_state: String,
    pub group_command_state: String,
    pub slot_claims: Vec<String>,
    pub path_reservations: Vec<String>,
    pub movement_offsets: Vec<String>,
    pub arrival_locked_unit_ids: Vec<String>,
    pub lagging_unit_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsFormationMoveExecutionFixtures {
    pub selected_unit_ids: Vec<String>,
    pub stages: Vec<RtsFormationMoveExecutionStageFixture>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsLocalObstructionRecoveryStageFixture {
    pub stage: String,
    pub action: RtsOrderQueueReplayAction,
    pub history_entry: String,
    pub input_source: String,
    pub renderer_path: String,
    pub preview_surface: String,
    pub selected_unit_ids: Vec<String>,
    pub command_destination_tile: Option<String>,
    pub path_tile_ids: Vec<String>,
    pub blocked_tile_ids: Vec<String>,
    pub disperse_tile_ids: Vec<String>,
    pub formation_slot_tile_ids: Vec<String>,
    pub group_route_tile_ids: Vec<String>,
    pub queued_unit_ids: Vec<String>,
    pub side_step_unit_ids: Vec<String>,
    pub gap_claims: Vec<String>,
    pub pathing_status: String,
    pub unit_response_state: String,
    pub group_command_state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsLocalObstructionRecoveryFixtures {
    pub selected_unit_ids: Vec<String>,
    pub stages: Vec<RtsLocalObstructionRecoveryStageFixture>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsControlGroupCommandFeedbackStripStageFixture {
    pub stage: String,
    pub action: RtsOrderQueueReplayAction,
    pub input_source: String,
    pub renderer_path: String,
    pub preview_surface: String,
    pub control_group_id: String,
    pub active_control_group_ids: Vec<String>,
    pub selected_unit_ids: Vec<String>,
    pub stance: String,
    pub recall_focus_tile: String,
    pub queued_target_tile: Option<String>,
    pub canceled_target_tile: Option<String>,
    pub override_final_tile_ids: Vec<String>,
    pub formation_anchor_tile: Option<String>,
    pub formation_slot_tile_ids: Vec<String>,
    pub queued_member_ids: Vec<String>,
    pub canceled_member_ids: Vec<String>,
    pub filtered_member_ids: Vec<String>,
    pub cleared_old_member_ids: Vec<String>,
    pub path_tile_ids: Vec<String>,
    pub group_route_tile_ids: Vec<String>,
    pub command_queue_entries: Vec<String>,
    pub combat_event: String,
    pub group_command_state: String,
    pub player_tile_x: i32,
    pub player_tile_y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsControlGroupCommandFeedbackStripFixtures {
    pub active_control_group_ids: Vec<String>,
    pub control_group_assignments: Vec<String>,
    pub ability_command_ids: Vec<String>,
    pub stages: Vec<RtsControlGroupCommandFeedbackStripStageFixture>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsControlGroupCommandFeedbackLifecycleStageFixture {
    pub stage: String,
    pub age_ticks: u8,
    pub action: RtsOrderQueueReplayAction,
    pub input_source: String,
    pub renderer_path: String,
    pub preview_surface: String,
    pub control_group_id: String,
    pub active_control_group_ids: Vec<String>,
    pub covered_group_ids: Vec<String>,
    pub selected_unit_ids: Vec<String>,
    pub stance: String,
    pub minimap_command_tile_id: String,
    pub command_destination_tile: String,
    pub path_tile_ids: Vec<String>,
    pub group_route_tile_ids: Vec<String>,
    pub formation_slot_tile_ids: Vec<String>,
    pub command_queue_entries: Vec<String>,
    pub lifecycle_event: String,
    pub group_command_state: String,
    pub player_tile_x: i32,
    pub player_tile_y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsControlGroupCommandFeedbackLifecycleFixtures {
    pub active_control_group_ids: Vec<String>,
    pub covered_group_ids: Vec<String>,
    pub control_group_assignments: Vec<String>,
    pub ability_command_ids: Vec<String>,
    pub group_26_member_ids: Vec<String>,
    pub group_27_member_ids: Vec<String>,
    pub group_28_member_ids: Vec<String>,
    pub all_member_ids: Vec<String>,
    pub group_26_queued_target_tile: String,
    pub group_27_canceled_target_tile: String,
    pub group_27_override_final_tile_ids: Vec<String>,
    pub group_28_formation_anchor_tile: String,
    pub group_28_formation_slot_tile_ids: Vec<String>,
    pub stages: Vec<RtsControlGroupCommandFeedbackLifecycleStageFixture>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsControlGroupCommandFeedbackHistoryEntry {
    pub group_id: String,
    pub badge: String,
    pub target_tile: Option<String>,
    pub canceled_target_tile: Option<String>,
    pub override_final_tile_ids: Vec<String>,
    pub formation_anchor_tile: Option<String>,
    pub formation_slot_tile_ids: Vec<String>,
    pub age_ticks: u32,
    pub bounded_history_index: Option<u32>,
    pub member_ids: Vec<String>,
    pub prune_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsControlGroupCommandHistoryStageFixture {
    pub stage: String,
    pub lifecycle_stage: String,
    pub age_ticks: u8,
    pub action: RtsOrderQueueReplayAction,
    pub input_source: String,
    pub renderer_path: String,
    pub preview_surface: String,
    pub control_group_id: String,
    pub active_control_group_ids: Vec<String>,
    pub selected_unit_ids: Vec<String>,
    pub control_group_assignments: Vec<String>,
    pub minimap_command_tile_id: String,
    pub command_destination_tile: String,
    pub path_tile_ids: Vec<String>,
    pub group_route_tile_ids: Vec<String>,
    pub formation_slot_tile_ids: Vec<String>,
    pub group_command_state: String,
    pub command_queue_entries: Vec<String>,
    pub combat_event_entries: Vec<String>,
    pub active_strip_cleared: bool,
    pub history_retained: bool,
    pub history_overflow_row_count: u32,
    pub stale_pruned_group_visible: bool,
    pub player_tile_x: i32,
    pub player_tile_y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsControlGroupCommandHistoryFixtures {
    pub retained_history_group_ids: Vec<String>,
    pub pruned_history_group_ids: Vec<String>,
    pub control_group_assignments: Vec<String>,
    pub ability_command_ids: Vec<String>,
    pub group_26_member_ids: Vec<String>,
    pub group_27_member_ids: Vec<String>,
    pub group_28_member_ids: Vec<String>,
    pub all_member_ids: Vec<String>,
    pub history_entries: Vec<RtsControlGroupCommandFeedbackHistoryEntry>,
    pub pruned_history_entries: Vec<RtsControlGroupCommandFeedbackHistoryEntry>,
    pub stages: Vec<RtsControlGroupCommandHistoryStageFixture>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsControlGroupCommandFeedbackStepFixture {
    pub step_index: u32,
    pub step_name: String,
    pub action_label: String,
    pub preview_stage: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsControlGroupCommandFeedbackReplayFixtures {
    pub retained_history_group_ids: Vec<String>,
    pub pruned_history_group_ids: Vec<String>,
    pub group_26_member_ids: Vec<String>,
    pub group_27_member_ids: Vec<String>,
    pub group_28_member_ids: Vec<String>,
    pub all_member_ids: Vec<String>,
    pub history_entries: Vec<RtsControlGroupCommandFeedbackHistoryEntry>,
    pub pruned_history_entries: Vec<RtsControlGroupCommandFeedbackHistoryEntry>,
    pub command_steps: Vec<RtsControlGroupCommandFeedbackStepFixture>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsControlGroupCommandFeedbackRejectionStepFixture {
    pub step_index: u32,
    pub step_name: String,
    pub input_source: String,
    pub action_label: String,
    pub expected_accepted: bool,
    pub expected_reason: String,
    pub preview_stage: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsControlGroupCommandFeedbackRejectionVisualStageFixture {
    pub stage: String,
    pub tile_id: String,
    pub reason: String,
    pub last_feedback: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsControlGroupCommandFeedbackRejectionReplayFixtures {
    pub retained_history_group_ids: Vec<String>,
    pub pruned_history_group_ids: Vec<String>,
    pub group_26_member_ids: Vec<String>,
    pub all_member_ids: Vec<String>,
    pub control_group_assignments: Vec<String>,
    pub ability_command_ids: Vec<String>,
    pub resource_spend_log: Vec<String>,
    pub preserved_command_history_events: Vec<String>,
    pub preserved_group_command_state: String,
    pub history_entries: Vec<RtsControlGroupCommandFeedbackHistoryEntry>,
    pub pruned_history_entries: Vec<RtsControlGroupCommandFeedbackHistoryEntry>,
    pub rejection_steps: Vec<RtsControlGroupCommandFeedbackRejectionStepFixture>,
    pub expected_input_sources: Vec<String>,
    pub expected_blocked_reasons: Vec<String>,
    pub visual_stages: Vec<RtsControlGroupCommandFeedbackRejectionVisualStageFixture>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsRuntimeMapLayoutInput {
    pub viewport_width: i32,
    pub viewport_height: i32,
    pub map_width_tiles: i32,
    pub map_height_tiles: i32,
    pub map_origin_x: i32,
    pub map_origin_y: i32,
    pub right_reserved_px: i32,
    pub bottom_reserved_px: i32,
    pub min_map_width_px: i32,
    pub min_map_height_px: i32,
    pub cell_width_min: i32,
    pub cell_width_max: i32,
    pub cell_height_min: i32,
    pub cell_height_max: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsRuntimeMapProjection {
    pub map_x: i32,
    pub map_y: i32,
    pub cell_w: i32,
    pub cell_h: i32,
    pub map_w: i32,
    pub map_h: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsRuntimeTerrainSeeds {
    pub surface_seed: i32,
    pub detail_seed: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsRuntimeTileLineStep {
    pub step_index: i32,
    pub step_count: i32,
    pub tile_x: i32,
    pub tile_y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsControlGroupSlotSummary {
    pub slot: String,
    pub key_label: String,
    pub member_count: usize,
    pub occupied: bool,
    pub active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsFirstContactPlayerScreenRuntimeApplication {
    pub contract_version: String,
    pub green: bool,
    pub profile_contract: String,
    pub map_scene: String,
    pub current_room_id: String,
    pub coins: u64,
    pub xp: u64,
    pub camera_focus_tile_id: Option<String>,
    pub camera_zoom_percent: u8,
    pub group_command_state: String,
    pub command_queue: Vec<String>,
    pub production_queue: Vec<String>,
    pub build_queue: Vec<String>,
    pub unit_health_percents: Vec<u8>,
    pub active_ability_id: Option<String>,
    pub ability_command_ids: Vec<String>,
    pub ability_cooldown_percents: Vec<u8>,
    pub visible_tile_ids: Vec<String>,
    pub fogged_tile_ids: Vec<String>,
    pub selection_box_tile_ids: Vec<String>,
    pub group_route_tile_ids: Vec<String>,
    pub terrain_route_tile_ids: Vec<String>,
    pub command_destination_tile_id: Option<String>,
    pub attack_target_id: Option<String>,
    pub training_progress_percent: u8,
    pub build_progress_percent: u8,
    pub ai_pressure_percent: u8,
    pub visibility_percent: u8,
    pub enemy_pressure_warning_percent: u8,
    pub army_supply_used: u8,
    pub army_supply_cap: u8,
    pub last_feedback: String,
    pub objective_status: String,
    pub profile_application_gate: bool,
    pub command_surface_seed_gate: bool,
    pub route_surface_seed_gate: bool,
    pub runtime_application_path: String,
    pub source_of_truth: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsFirstContactPlayerScreenReview {
    pub map_scene: String,
    pub current_room_id: String,
    pub coins: u64,
    pub xp: u64,
    pub camera_focus_tile_id: Option<String>,
    pub visibility_percent: u8,
    pub army_supply_used: u8,
    pub army_supply_cap: u8,
    pub objective_status: String,
    pub production_queue: Vec<String>,
    pub build_queue: Vec<String>,
    pub selected_unit_ids: Vec<String>,
    pub command_queue: Vec<String>,
    pub command_destination_tile_id: Option<String>,
    pub group_route_tile_ids: Vec<String>,
    pub visible_tile_count: usize,
    pub fogged_tile_count: usize,
    pub selection_box_tile_count: usize,
    pub unit_health_percents: Vec<u8>,
    pub ability_command_ids: Vec<String>,
    pub ability_cooldown_percents: Vec<u8>,
    pub active_ability_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsOfflineAdapterRuntimeHandoffReviewInput {
    pub contract_version: String,
    pub handoff_mode: String,
    pub accepted_runtime_command_labels: Vec<String>,
    pub accepted_runtime_destination_tile_ids: Vec<String>,
    pub accepted_runtime_subject_actor_ids: Vec<String>,
    pub rejected_runtime_command_labels: Vec<String>,
    pub scoped_update_actor_ids: Vec<String>,
    pub runtime_control_group_id: String,
    pub runtime_group_command_state: String,
    pub runtime_pathing_status: String,
    pub runtime_unit_response_state: String,
    pub runtime_command_stamp_source: String,
    pub runtime_command_stamp_kind: String,
    pub runtime_command_stamp_tile_id: Option<String>,
    pub runtime_command_stamp_player_label: String,
    pub runtime_last_feedback: String,
    pub accepted_order_runtime_ready: bool,
    pub rejected_order_runtime_ready: bool,
    pub scoped_update_runtime_ready: bool,
    pub no_socket_boundary_ready: bool,
    pub green: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsOfflineAdapterRuntimeApplication {
    pub contract_version: String,
    pub green: bool,
    pub handoff_contract: String,
    pub handoff_mode: String,
    pub runtime_control_group_id: Option<String>,
    pub selected_unit_ids: Vec<String>,
    pub command_queue: Vec<String>,
    pub command_destination_tile_id: Option<String>,
    pub group_route_tile_ids: Vec<String>,
    pub rejected_runtime_command_labels: Vec<String>,
    pub scoped_update_actor_ids: Vec<String>,
    pub runtime_group_command_state: String,
    pub runtime_pathing_status: String,
    pub runtime_unit_response_state: String,
    pub runtime_command_stamp_source: String,
    pub runtime_command_stamp_kind: String,
    pub runtime_command_stamp_tile_id: Option<String>,
    pub runtime_command_stamp_player_label: String,
    pub runtime_last_feedback: String,
    pub accepted_order_runtime_gate: bool,
    pub rejected_order_runtime_gate: bool,
    pub scoped_update_runtime_gate: bool,
    pub no_socket_boundary_gate: bool,
    pub runtime_application_path: String,
    pub source_of_truth: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsFirstContactOfflineAdapterConsumptionReviewInput {
    pub adapter_green: bool,
    pub adapter_contract: String,
    pub adapter_id: String,
    pub adapter_mode: String,
    pub adapter_runtime_handoff: RtsOfflineAdapterRuntimeHandoffReviewInput,
    pub input_queue_labels: Vec<String>,
    pub accepted_server_order_labels: Vec<String>,
    pub rejected_client_order_reasons: Vec<String>,
    pub runtime_player_screen_review: RtsFirstContactPlayerScreenReview,
    pub server_authoritative: bool,
    pub visibility_scoped_response: bool,
    pub client_prediction_claimed: bool,
    pub rollback_netcode_claimed: bool,
    pub socket_opened: bool,
    pub hosted_service_claimed: bool,
    pub public_launch_ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsFirstContactOfflineAdapterConsumptionReview {
    pub contract_version: String,
    pub green: bool,
    pub adapter_contract: String,
    pub adapter_runtime_handoff_contract: String,
    pub adapter_id: String,
    pub adapter_mode: String,
    pub adapter_runtime_handoff: RtsOfflineAdapterRuntimeHandoffReviewInput,
    pub runtime_application_contract: String,
    pub runtime_application: RtsOfflineAdapterRuntimeApplication,
    pub input_queue_labels: Vec<String>,
    pub accepted_server_order_labels: Vec<String>,
    pub accepted_runtime_command_labels: Vec<String>,
    pub accepted_runtime_destination_tile_ids: Vec<String>,
    pub accepted_runtime_subject_actor_ids: Vec<String>,
    pub rejected_client_order_reasons: Vec<String>,
    pub rejected_runtime_command_labels: Vec<String>,
    pub rejected_commands_suppressed: bool,
    pub scoped_update_actor_ids: Vec<String>,
    pub runtime_control_group_id: Option<String>,
    pub runtime_group_command_state: String,
    pub runtime_pathing_status: String,
    pub runtime_unit_response_state: String,
    pub runtime_command_stamp_source: String,
    pub runtime_command_stamp_kind: String,
    pub runtime_command_stamp_tile_id: Option<String>,
    pub runtime_command_stamp_player_label: String,
    pub runtime_last_feedback: String,
    pub runtime_player_screen_review: RtsFirstContactPlayerScreenReview,
    pub local_session_handoff_gate: bool,
    pub runtime_application_gate: bool,
    pub player_screen_review_gate: bool,
    pub accepted_order_runtime_gate: bool,
    pub rejected_order_runtime_gate: bool,
    pub scoped_update_runtime_gate: bool,
    pub no_network_claim_gate: bool,
    pub server_authoritative: bool,
    pub visibility_scoped_response: bool,
    pub client_prediction_claimed: bool,
    pub rollback_netcode_claimed: bool,
    pub socket_opened: bool,
    pub hosted_service_claimed: bool,
    pub public_launch_ready: bool,
    pub input_path: String,
    pub runtime_path: String,
    pub source_of_truth: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsOfflineAdapterLobbyReadyReviewInput {
    pub adapter_green: bool,
    pub adapter_contract: String,
    pub adapter_id: String,
    pub handoff_id: String,
    pub arena_id: String,
    pub map_id: String,
    pub adapter_mode: String,
    pub bevy_client_role: String,
    pub authority_role: String,
    pub connected_player_ids: Vec<String>,
    pub bot_player_ids: Vec<String>,
    pub frame_sha256s: Vec<String>,
    pub local_multiplayer_ready: bool,
    pub offline_bot_ready: bool,
    pub bevy_adapter_ready: bool,
    pub server_authoritative: bool,
    pub visibility_scoped_response: bool,
    pub client_prediction_claimed: bool,
    pub rollback_netcode_claimed: bool,
    pub socket_opened: bool,
    pub hosted_service_claimed: bool,
    pub public_launch_ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsFirstContactOfflineAdapterLobbyReadyReview {
    pub contract_version: String,
    pub green: bool,
    pub adapter_contract: String,
    pub adapter_id: String,
    pub handoff_id: String,
    pub arena_id: String,
    pub map_id: String,
    pub adapter_mode: String,
    pub bevy_client_role: String,
    pub authority_role: String,
    pub connected_player_ids: Vec<String>,
    pub bot_player_ids: Vec<String>,
    pub ready_state_labels: Vec<String>,
    pub blocked_network_claim_labels: Vec<String>,
    pub local_multiplayer_ready_gate: bool,
    pub offline_bot_ready_gate: bool,
    pub bevy_adapter_ready_gate: bool,
    pub authority_ready_gate: bool,
    pub frame_identity_gate: bool,
    pub no_network_claim_gate: bool,
    pub input_path: String,
    pub runtime_path: String,
    pub source_of_truth: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsFirstContactOfflineAdapterSessionTransitionReview {
    pub contract_version: String,
    pub green: bool,
    pub initial_application_contract: String,
    pub runtime_application_contract: String,
    pub handoff_contract: String,
    pub map_scene: String,
    pub current_room_id: String,
    pub camera_focus_tile_id: Option<String>,
    pub before_command_queue: Vec<String>,
    pub after_command_queue: Vec<String>,
    pub before_route_tile_ids: Vec<String>,
    pub after_route_tile_ids: Vec<String>,
    pub before_command_destination_tile_id: Option<String>,
    pub after_command_destination_tile_id: Option<String>,
    pub selected_unit_ids: Vec<String>,
    pub scoped_update_actor_ids: Vec<String>,
    pub accepted_runtime_command_labels: Vec<String>,
    pub rejected_runtime_command_labels: Vec<String>,
    pub runtime_control_group_id: Option<String>,
    pub runtime_group_command_state: String,
    pub runtime_command_stamp_source: String,
    pub runtime_command_stamp_kind: String,
    pub runtime_command_stamp_tile_id: Option<String>,
    pub runtime_last_feedback: String,
    pub command_surface_replaced_gate: bool,
    pub route_overlay_replaced_gate: bool,
    pub session_context_preserved_gate: bool,
    pub rejected_order_suppressed_gate: bool,
    pub no_socket_boundary_gate: bool,
    pub input_path: String,
    pub runtime_path: String,
    pub source_of_truth: String,
}

pub fn rts_first_contact_player_screen_runtime_application(
    profile: &RtsFirstContactPlayerScreenProfile,
) -> RtsFirstContactPlayerScreenRuntimeApplication {
    let tile_id = |tile: RtsTile| rts_runtime_tile_id((tile.x, tile.y));
    let visible_tile_ids = profile
        .visible_tiles
        .iter()
        .copied()
        .map(tile_id)
        .collect::<Vec<_>>();
    let fogged_tile_ids = profile
        .fogged_tiles
        .iter()
        .copied()
        .map(tile_id)
        .collect::<Vec<_>>();
    let selection_box_tile_ids = profile
        .selection_box_tiles
        .iter()
        .copied()
        .map(tile_id)
        .collect::<Vec<_>>();
    let group_route_tile_ids = profile
        .group_route_tiles
        .iter()
        .copied()
        .map(tile_id)
        .collect::<Vec<_>>();
    let terrain_route_tile_ids = profile
        .terrain_route_tiles
        .iter()
        .copied()
        .map(tile_id)
        .collect::<Vec<_>>();
    let camera_focus_tile_id = Some(tile_id(profile.camera_focus_tile));
    let command_destination_tile_id = Some(tile_id(profile.command_destination_tile));
    let ability_command_ids = profile.chrome.command_grid_slot_ids.clone();

    let profile_application_gate = profile.contract_version
        == trnm_rts_data::TRNM_RTS_DATA_FIRST_CONTACT_PLAYER_SCREEN_CONTRACT
        && profile.map_id == "first_contact_basin"
        && profile.room_id == "first-contact-basin"
        && profile.camera_zoom_percent > 0
        && profile.army_supply_used <= profile.army_supply_cap
        && !profile.last_feedback.is_empty()
        && !profile.objective_status.is_empty();
    let command_surface_seed_gate = profile.command_queue.len() == 4
        && profile
            .command_queue
            .iter()
            .any(|command| command == "build:trnm.flux.relay")
        && profile
            .command_queue
            .iter()
            .any(|command| command == "train:trnm.worker")
        && profile
            .command_queue
            .iter()
            .any(|command| command == "attack:trnm.flux.beacon")
        && ability_command_ids
            .iter()
            .any(|ability| ability == &profile.active_ability_id)
        && profile.ability_cooldown_percents.len() == ability_command_ids.len()
        && profile
            .ability_cooldown_percents
            .iter()
            .all(|percent| *percent <= 100)
        && profile
            .unit_health_percents
            .iter()
            .all(|percent| *percent <= 100);
    let route_surface_seed_gate = visible_tile_ids.len() == 64
        && fogged_tile_ids.len() == 6
        && selection_box_tile_ids.len() == 4
        && group_route_tile_ids.iter().any(|tile| tile == "16,9")
        && command_destination_tile_id.as_deref() == Some("16,9")
        && profile.training_progress_percent <= 100
        && profile.build_progress_percent <= 100
        && profile.ai_pressure_percent <= 100
        && profile.visibility_percent <= 100
        && profile.enemy_pressure_warning_percent <= 100;
    let green = profile_application_gate && command_surface_seed_gate && route_surface_seed_gate;

    RtsFirstContactPlayerScreenRuntimeApplication {
        contract_version:
            TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_PLAYER_SCREEN_APPLICATION_CONTRACT.to_string(),
        green,
        profile_contract: profile.contract_version.clone(),
        map_scene: profile.map_id.clone(),
        current_room_id: profile.room_id.clone(),
        coins: profile.coins,
        xp: profile.xp,
        camera_focus_tile_id,
        camera_zoom_percent: profile.camera_zoom_percent,
        group_command_state: profile.group_command_state.clone(),
        command_queue: profile.command_queue.clone(),
        production_queue: profile.production_queue.clone(),
        build_queue: profile.build_queue.clone(),
        unit_health_percents: profile.unit_health_percents.clone(),
        active_ability_id: Some(profile.active_ability_id.clone()),
        ability_command_ids,
        ability_cooldown_percents: profile.ability_cooldown_percents.clone(),
        visible_tile_ids,
        fogged_tile_ids,
        selection_box_tile_ids,
        group_route_tile_ids,
        terrain_route_tile_ids,
        command_destination_tile_id,
        attack_target_id: Some(profile.attack_target_rule_id.clone()),
        training_progress_percent: profile.training_progress_percent,
        build_progress_percent: profile.build_progress_percent,
        ai_pressure_percent: profile.ai_pressure_percent,
        visibility_percent: profile.visibility_percent,
        enemy_pressure_warning_percent: profile.enemy_pressure_warning_percent,
        army_supply_used: profile.army_supply_used,
        army_supply_cap: profile.army_supply_cap,
        last_feedback: profile.last_feedback.clone(),
        objective_status: profile.objective_status.clone(),
        profile_application_gate,
        command_surface_seed_gate,
        route_surface_seed_gate,
        runtime_application_path:
            "trnm-rts-data first_contact_player_screen_profile -> trnm-rts-bevy-runtime player_screen_runtime_application -> NativeFirstPlayableRuntime mutation"
                .to_string(),
        source_of_truth:
            "This Bevy-free runtime application translates the trnm-rts-data First Contact player-screen profile into room, camera, command queue, production/build queues, visibility, selection, route, ability, supply, and objective runtime fields before the Bevy adapter mutates NativeFirstPlayableRuntime."
                .to_string(),
    }
}

pub fn rts_first_contact_offline_adapter_runtime_application(
    handoff: &RtsOfflineAdapterRuntimeHandoffReviewInput,
) -> RtsOfflineAdapterRuntimeApplication {
    let selected_unit_ids = handoff.accepted_runtime_subject_actor_ids.clone();
    let command_queue = handoff.accepted_runtime_command_labels.clone();
    let group_route_tile_ids = handoff.accepted_runtime_destination_tile_ids.clone();
    let rejected_runtime_command_labels = handoff.rejected_runtime_command_labels.clone();
    let scoped_update_actor_ids = handoff.scoped_update_actor_ids.clone();
    let accepted_order_runtime_gate = handoff.green
        && handoff.accepted_order_runtime_ready
        && command_queue == rts_string_vec(["move:8,4"])
        && group_route_tile_ids == rts_string_vec(["8,4"])
        && selected_unit_ids == rts_string_vec(["trnm.worker.alpha"])
        && handoff.runtime_control_group_id == "1"
        && handoff.runtime_group_command_state == "offline_adapter_authority_applied"
        && handoff.runtime_pathing_status == "offline_adapter_replay_consumed"
        && handoff.runtime_unit_response_state == "server_authoritative_move_applied"
        && handoff.runtime_command_stamp_source == "trnm-rts-online:offline_loopback_authority"
        && handoff.runtime_command_stamp_kind == "server_accepted_move"
        && handoff.runtime_command_stamp_tile_id.as_deref() == Some("8,4");
    let rejected_order_runtime_gate = handoff.rejected_order_runtime_ready
        && rejected_runtime_command_labels == rts_string_vec(["client:attack_fogged_keep"])
        && handoff
            .runtime_last_feedback
            .contains("rejected target_actor_not_visible");
    let scoped_update_runtime_gate = handoff.scoped_update_runtime_ready
        && scoped_update_actor_ids.len() == 4
        && scoped_update_actor_ids
            .iter()
            .any(|actor_id| actor_id == "trnm.worker.alpha")
        && scoped_update_actor_ids
            .iter()
            .all(|actor_id| actor_id != "trnm.enemy.keep.fogged");
    let no_socket_boundary_gate = handoff.no_socket_boundary_ready;
    let green = accepted_order_runtime_gate
        && rejected_order_runtime_gate
        && scoped_update_runtime_gate
        && no_socket_boundary_gate;

    RtsOfflineAdapterRuntimeApplication {
        contract_version:
            TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_APPLICATION_CONTRACT.to_string(),
        green,
        handoff_contract: handoff.contract_version.clone(),
        handoff_mode: handoff.handoff_mode.clone(),
        runtime_control_group_id: Some(handoff.runtime_control_group_id.clone()),
        selected_unit_ids,
        command_queue,
        command_destination_tile_id: handoff.runtime_command_stamp_tile_id.clone(),
        group_route_tile_ids,
        rejected_runtime_command_labels,
        scoped_update_actor_ids,
        runtime_group_command_state: handoff.runtime_group_command_state.clone(),
        runtime_pathing_status: handoff.runtime_pathing_status.clone(),
        runtime_unit_response_state: handoff.runtime_unit_response_state.clone(),
        runtime_command_stamp_source: handoff.runtime_command_stamp_source.clone(),
        runtime_command_stamp_kind: handoff.runtime_command_stamp_kind.clone(),
        runtime_command_stamp_tile_id: handoff.runtime_command_stamp_tile_id.clone(),
        runtime_command_stamp_player_label: handoff.runtime_command_stamp_player_label.clone(),
        runtime_last_feedback: handoff.runtime_last_feedback.clone(),
        accepted_order_runtime_gate,
        rejected_order_runtime_gate,
        scoped_update_runtime_gate,
        no_socket_boundary_gate,
        runtime_application_path:
            "trnm-rts-bevy-runtime offline_adapter_runtime_application -> NativeFirstPlayableRuntime mutation"
                .to_string(),
        source_of_truth:
            "This Bevy-free runtime application translates the trnm-rts-online offline adapter handoff into the command queue, selected actors, route tile, command stamp, pathing/group response state, and feedback that the Bevy adapter mutates onto NativeFirstPlayableRuntime."
                .to_string(),
    }
}

pub fn rts_first_contact_offline_adapter_session_transition_review(
    initial_application: &RtsFirstContactPlayerScreenRuntimeApplication,
    runtime_application: &RtsOfflineAdapterRuntimeApplication,
    handoff: &RtsOfflineAdapterRuntimeHandoffReviewInput,
) -> RtsFirstContactOfflineAdapterSessionTransitionReview {
    let before_command_queue = initial_application.command_queue.clone();
    let after_command_queue = runtime_application.command_queue.clone();
    let before_route_tile_ids = initial_application.group_route_tile_ids.clone();
    let after_route_tile_ids = runtime_application.group_route_tile_ids.clone();
    let command_surface_replaced_gate = initial_application.green
        && runtime_application.green
        && before_command_queue != after_command_queue
        && before_command_queue
            .iter()
            .any(|command| command == "build:trnm.flux.relay")
        && after_command_queue == rts_string_vec(["move:8,4"])
        && runtime_application.selected_unit_ids == rts_string_vec(["trnm.worker.alpha"])
        && runtime_application.runtime_control_group_id.as_deref() == Some("1")
        && runtime_application.runtime_group_command_state == "offline_adapter_authority_applied";
    let route_overlay_replaced_gate = before_route_tile_ids.iter().any(|tile| tile == "16,9")
        && initial_application.command_destination_tile_id.as_deref() == Some("16,9")
        && after_route_tile_ids == rts_string_vec(["8,4"])
        && runtime_application.command_destination_tile_id.as_deref() == Some("8,4")
        && handoff.accepted_runtime_destination_tile_ids == after_route_tile_ids
        && handoff.accepted_runtime_command_labels == after_command_queue;
    let session_context_preserved_gate = initial_application.map_scene == "first_contact_basin"
        && initial_application.current_room_id == "first-contact-basin"
        && initial_application.camera_focus_tile_id.as_deref() == Some("16,16")
        && initial_application.visibility_percent <= 100
        && initial_application.army_supply_used <= initial_application.army_supply_cap
        && !initial_application.objective_status.is_empty();
    let rejected_order_suppressed_gate = runtime_application.rejected_order_runtime_gate
        && runtime_application.rejected_runtime_command_labels
            == rts_string_vec(["client:attack_fogged_keep"])
        && after_command_queue
            .iter()
            .all(|command| !command.contains("fogged_keep"))
        && runtime_application
            .runtime_last_feedback
            .contains("rejected target_actor_not_visible");
    let no_socket_boundary_gate = runtime_application.no_socket_boundary_gate
        && handoff.no_socket_boundary_ready
        && handoff.handoff_mode == "server_authoritative_runtime_command_handoff";
    let green = command_surface_replaced_gate
        && route_overlay_replaced_gate
        && session_context_preserved_gate
        && rejected_order_suppressed_gate
        && no_socket_boundary_gate;

    RtsFirstContactOfflineAdapterSessionTransitionReview {
        contract_version:
            TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_SESSION_TRANSITION_CONTRACT
                .to_string(),
        green,
        initial_application_contract: initial_application.contract_version.clone(),
        runtime_application_contract: runtime_application.contract_version.clone(),
        handoff_contract: handoff.contract_version.clone(),
        map_scene: initial_application.map_scene.clone(),
        current_room_id: initial_application.current_room_id.clone(),
        camera_focus_tile_id: initial_application.camera_focus_tile_id.clone(),
        before_command_queue,
        after_command_queue,
        before_route_tile_ids,
        after_route_tile_ids,
        before_command_destination_tile_id: initial_application.command_destination_tile_id.clone(),
        after_command_destination_tile_id: runtime_application.command_destination_tile_id.clone(),
        selected_unit_ids: runtime_application.selected_unit_ids.clone(),
        scoped_update_actor_ids: runtime_application.scoped_update_actor_ids.clone(),
        accepted_runtime_command_labels: handoff.accepted_runtime_command_labels.clone(),
        rejected_runtime_command_labels: runtime_application.rejected_runtime_command_labels.clone(),
        runtime_control_group_id: runtime_application.runtime_control_group_id.clone(),
        runtime_group_command_state: runtime_application.runtime_group_command_state.clone(),
        runtime_command_stamp_source: runtime_application.runtime_command_stamp_source.clone(),
        runtime_command_stamp_kind: runtime_application.runtime_command_stamp_kind.clone(),
        runtime_command_stamp_tile_id: runtime_application.runtime_command_stamp_tile_id.clone(),
        runtime_last_feedback: runtime_application.runtime_last_feedback.clone(),
        command_surface_replaced_gate,
        route_overlay_replaced_gate,
        session_context_preserved_gate,
        rejected_order_suppressed_gate,
        no_socket_boundary_gate,
        input_path:
            "trnm-rts-data player-screen application + trnm-rts-online offline adapter handoff -> trnm-rts-bevy-runtime session transition review"
                .to_string(),
        runtime_path:
            "trnm-rts-bevy-runtime first_contact_offline_adapter_session_transition -> Bevy local session UI transition evidence"
                .to_string(),
        source_of_truth:
            "This Bevy-free transition review proves the First Contact player-screen command surface and route overlay move from data-seeded local UI state to the server-authoritative offline adapter handoff while room, camera, visibility, supply, objective context, scoped updates, and no-socket claims stay coherent."
                .to_string(),
    }
}

pub fn rts_first_contact_offline_adapter_lobby_ready_review(
    input: RtsOfflineAdapterLobbyReadyReviewInput,
) -> RtsFirstContactOfflineAdapterLobbyReadyReview {
    let ready_state_labels = input
        .connected_player_ids
        .iter()
        .map(|player_id| format!("player:{player_id}:ready"))
        .chain(
            input
                .bot_player_ids
                .iter()
                .map(|bot_id| format!("bot:{bot_id}:ready")),
        )
        .chain(std::iter::once(
            "authority:offline_loopback:no_socket".to_string(),
        ))
        .collect::<Vec<_>>();
    let blocked_network_claim_labels = [
        ("client_prediction", input.client_prediction_claimed),
        ("rollback_netcode", input.rollback_netcode_claimed),
        ("socket", input.socket_opened),
        ("hosted_service", input.hosted_service_claimed),
        ("public_launch", input.public_launch_ready),
    ]
    .into_iter()
    .filter_map(|(label, claimed)| (!claimed).then(|| format!("{label}:not_claimed")))
    .collect::<Vec<_>>();
    let local_multiplayer_ready_gate = input.adapter_green
        && input.local_multiplayer_ready
        && input.connected_player_ids == rts_string_vec(["local-player", "mirror_guard"])
        && input.adapter_mode == "offline_loopback_authority";
    let offline_bot_ready_gate =
        input.offline_bot_ready && input.bot_player_ids == rts_string_vec(["mirror_guard"]);
    let bevy_adapter_ready_gate = input.bevy_adapter_ready
        && input.bevy_client_role == "visualization_and_local_input_submitter"
        && input.authority_role == "trnm_rts_online_fixture_authority_no_socket";
    let authority_ready_gate = input.server_authoritative && input.visibility_scoped_response;
    let frame_identity_gate =
        input.frame_sha256s.len() == 3 && input.frame_sha256s.iter().all(|sha| sha.len() == 64);
    let no_network_claim_gate = !input.client_prediction_claimed
        && !input.rollback_netcode_claimed
        && !input.socket_opened
        && !input.hosted_service_claimed
        && !input.public_launch_ready
        && blocked_network_claim_labels.len() == 5;
    let green = local_multiplayer_ready_gate
        && offline_bot_ready_gate
        && bevy_adapter_ready_gate
        && authority_ready_gate
        && frame_identity_gate
        && no_network_claim_gate
        && input.adapter_contract == "trnm_rts_online_offline_adapter_v1"
        && input.adapter_id == "first-contact-offline-loopback-adapter"
        && input.handoff_id == "first-contact-local-loopback-handoff"
        && input.arena_id == "first-contact-local-arena"
        && input.map_id == "first_contact_basin";

    RtsFirstContactOfflineAdapterLobbyReadyReview {
        contract_version:
            TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_LOBBY_READY_CONTRACT.to_string(),
        green,
        adapter_contract: input.adapter_contract,
        adapter_id: input.adapter_id,
        handoff_id: input.handoff_id,
        arena_id: input.arena_id,
        map_id: input.map_id,
        adapter_mode: input.adapter_mode,
        bevy_client_role: input.bevy_client_role,
        authority_role: input.authority_role,
        connected_player_ids: input.connected_player_ids,
        bot_player_ids: input.bot_player_ids,
        ready_state_labels,
        blocked_network_claim_labels,
        local_multiplayer_ready_gate,
        offline_bot_ready_gate,
        bevy_adapter_ready_gate,
        authority_ready_gate,
        frame_identity_gate,
        no_network_claim_gate,
        input_path:
            "trnm-rts-online offline adapter lobby ready input -> trnm-rts-bevy-runtime lobby ready review"
                .to_string(),
        runtime_path:
            "trnm-rts-bevy-runtime first_contact_offline_adapter_lobby_ready -> Bevy local lobby/ready-state evidence"
                .to_string(),
        source_of_truth:
            "This Bevy-free lobby ready review proves the local First Contact offline adapter has two connected players, one offline bot, stable frame identities, Bevy acting only as visualization/input submitter, and no socket, hosted-service, client-prediction, rollback, or public-launch credit before the Bevy adapter renders any lobby or ready-state UI."
                .to_string(),
    }
}

pub fn rts_first_contact_offline_adapter_consumption_review(
    input: RtsFirstContactOfflineAdapterConsumptionReviewInput,
) -> RtsFirstContactOfflineAdapterConsumptionReview {
    let runtime_handoff = input.adapter_runtime_handoff;
    let runtime = input.runtime_player_screen_review;
    let runtime_application =
        rts_first_contact_offline_adapter_runtime_application(&runtime_handoff);
    let accepted_runtime_destination_tile_ids = runtime_application.group_route_tile_ids.clone();
    let accepted_runtime_subject_actor_ids = runtime_application.selected_unit_ids.clone();
    let rejected_runtime_command_labels =
        runtime_application.rejected_runtime_command_labels.clone();
    let scoped_update_actor_ids = runtime_application.scoped_update_actor_ids.clone();
    let rejected_commands_suppressed = rejected_runtime_command_labels.iter().all(|rejected| {
        runtime
            .command_queue
            .iter()
            .all(|command| !command.contains(rejected) && !command.contains("fogged_keep"))
    }) && runtime
        .command_queue
        .iter()
        .all(|command| !command.contains("trnm.enemy.keep.fogged"));
    let accepted_order_runtime_gate = input.adapter_green
        && runtime_application.green
        && runtime_application.accepted_order_runtime_gate
        && runtime.command_queue == runtime_application.command_queue
        && runtime.command_destination_tile_id.as_deref() == Some("8,4")
        && runtime.selected_unit_ids == accepted_runtime_subject_actor_ids
        && runtime_application.runtime_group_command_state == "offline_adapter_authority_applied"
        && runtime_application.runtime_command_stamp_source
            == "trnm-rts-online:offline_loopback_authority"
        && runtime_application.runtime_command_stamp_kind == "server_accepted_move"
        && runtime_application.runtime_command_stamp_tile_id.as_deref() == Some("8,4")
        && runtime_application.runtime_unit_response_state == "server_authoritative_move_applied";
    let local_session_handoff_gate = runtime.map_scene == "first_contact_basin"
        && runtime.current_room_id == "first-contact-basin"
        && runtime.coins == 890
        && runtime.xp == 92
        && runtime.camera_focus_tile_id.as_deref() == Some("16,16")
        && runtime.visibility_percent == 76
        && runtime.army_supply_used == 12
        && runtime.army_supply_cap == 22
        && runtime.objective_status == "secure first relay beacon and hold the center lane"
        && runtime.production_queue
            == rts_string_vec(["train:guard", "train:worker", "upgrade:signal_blade"])
        && runtime.build_queue == rts_string_vec(["build:watch_tower", "upgrade:training_hall"])
        && runtime.active_ability_id.as_deref() == Some("worker")
        && runtime.ability_command_ids
            == rts_string_vec(["worker", "scout", "warden", "relay", "core", "signal"])
        && runtime.visible_tile_count == 64
        && runtime.fogged_tile_count == 6
        && runtime.selection_box_tile_count == 4
        && runtime.unit_health_percents == vec![96, 78, 71, 34]
        && runtime.ability_cooldown_percents == vec![0, 0, 16, 0, 42, 25];
    let player_screen_review_gate = local_session_handoff_gate
        && runtime.selected_unit_ids == accepted_runtime_subject_actor_ids
        && runtime.command_queue == runtime_application.command_queue
        && runtime.command_destination_tile_id.as_deref() == Some("8,4")
        && runtime.group_route_tile_ids == accepted_runtime_destination_tile_ids
        && runtime_application.runtime_control_group_id.as_deref() == Some("1")
        && runtime_application.runtime_group_command_state == "offline_adapter_authority_applied";
    let rejected_order_runtime_gate = runtime_application.rejected_order_runtime_gate
        && rejected_runtime_command_labels == rts_string_vec(["client:attack_fogged_keep"])
        && input.rejected_client_order_reasons == rts_string_vec(["target_actor_not_visible"])
        && rejected_commands_suppressed
        && runtime_application
            .runtime_last_feedback
            .contains("rejected target_actor_not_visible");
    let scoped_update_runtime_gate = runtime_application.scoped_update_runtime_gate
        && input.visibility_scoped_response
        && input.server_authoritative;
    let no_network_claim_gate = runtime_application.no_socket_boundary_gate
        && !input.client_prediction_claimed
        && !input.rollback_netcode_claimed
        && !input.socket_opened
        && !input.hosted_service_claimed
        && !input.public_launch_ready;
    let green = accepted_order_runtime_gate
        && local_session_handoff_gate
        && player_screen_review_gate
        && rejected_order_runtime_gate
        && scoped_update_runtime_gate
        && no_network_claim_gate;

    RtsFirstContactOfflineAdapterConsumptionReview {
        contract_version:
            TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_CONSUMPTION_CONTRACT.to_string(),
        green,
        adapter_contract: input.adapter_contract,
        adapter_runtime_handoff_contract: runtime_handoff.contract_version.clone(),
        adapter_id: input.adapter_id,
        adapter_mode: input.adapter_mode,
        adapter_runtime_handoff: runtime_handoff.clone(),
        runtime_application_contract: runtime_application.contract_version.clone(),
        runtime_application: runtime_application.clone(),
        input_queue_labels: input.input_queue_labels,
        accepted_server_order_labels: input.accepted_server_order_labels,
        accepted_runtime_command_labels: runtime_application.command_queue.clone(),
        accepted_runtime_destination_tile_ids,
        accepted_runtime_subject_actor_ids: runtime_application.selected_unit_ids.clone(),
        rejected_client_order_reasons: input.rejected_client_order_reasons,
        rejected_runtime_command_labels,
        rejected_commands_suppressed,
        scoped_update_actor_ids,
        runtime_control_group_id: runtime_application.runtime_control_group_id.clone(),
        runtime_group_command_state: runtime_application.runtime_group_command_state.clone(),
        runtime_pathing_status: runtime_application.runtime_pathing_status.clone(),
        runtime_unit_response_state: runtime_application.runtime_unit_response_state.clone(),
        runtime_command_stamp_source: runtime_application.runtime_command_stamp_source.clone(),
        runtime_command_stamp_kind: runtime_application.runtime_command_stamp_kind.clone(),
        runtime_command_stamp_tile_id: runtime_application.runtime_command_stamp_tile_id.clone(),
        runtime_command_stamp_player_label: runtime_application.runtime_command_stamp_player_label.clone(),
        runtime_last_feedback: runtime_application.runtime_last_feedback.clone(),
        runtime_player_screen_review: runtime,
        local_session_handoff_gate,
        runtime_application_gate: runtime_application.green,
        player_screen_review_gate,
        accepted_order_runtime_gate,
        rejected_order_runtime_gate,
        scoped_update_runtime_gate,
        no_network_claim_gate,
        server_authoritative: input.server_authoritative,
        visibility_scoped_response: input.visibility_scoped_response,
        client_prediction_claimed: input.client_prediction_claimed,
        rollback_netcode_claimed: input.rollback_netcode_claimed,
        socket_opened: input.socket_opened,
        hosted_service_claimed: input.hosted_service_claimed,
        public_launch_ready: input.public_launch_ready,
        input_path:
            "trnm-rts-online offline adapter review input -> trnm-rts-bevy-runtime runtime application -> Bevy local player-screen snapshot"
                .to_string(),
        runtime_path:
            "trnm-rts-bevy-runtime offline_adapter_runtime_application + first_contact_offline_adapter_consumption_review -> NativeFirstPlayableRuntime consumer"
                .to_string(),
        source_of_truth:
            "This Bevy-free runtime review consumes the no-socket offline adapter through a trnm-rts-online-owned review input, a Bevy-free runtime application, and a local player-screen/session surface snapshot: the server-authoritative move reaches the visible command queue, route overlay, and command stamp while room, camera, visibility, queues, supply, and objective state stay coherent and the fogged attack rejection is suppressed from UI/action replay state."
                .to_string(),
    }
}

pub fn rts_large_map_clamp_tile(tile: (i32, i32)) -> (i32, i32) {
    (
        tile.0
            .clamp(TRNM_RTS_RUNTIME_MAP_MIN_TILE, TRNM_RTS_RUNTIME_MAP_MAX_X),
        tile.1
            .clamp(TRNM_RTS_RUNTIME_MAP_MIN_TILE, TRNM_RTS_RUNTIME_MAP_MAX_Y),
    )
}

pub fn rts_runtime_map_projection(input: RtsRuntimeMapLayoutInput) -> RtsRuntimeMapProjection {
    let map_width_tiles = input.map_width_tiles.max(1);
    let map_height_tiles = input.map_height_tiles.max(1);
    let available_w = (input.viewport_width - input.right_reserved_px).max(input.min_map_width_px);
    let available_h = (input.viewport_height - input.bottom_reserved_px - input.map_origin_y)
        .max(input.min_map_height_px);
    let cell_w = (available_w / map_width_tiles)
        .clamp(input.cell_width_min, input.cell_width_max)
        .max(1);
    let cell_h = (available_h / map_height_tiles)
        .clamp(input.cell_height_min, input.cell_height_max)
        .max(1);
    RtsRuntimeMapProjection {
        map_x: input.map_origin_x,
        map_y: input.map_origin_y,
        cell_w,
        cell_h,
        map_w: cell_w * map_width_tiles,
        map_h: cell_h * map_height_tiles,
    }
}

pub fn rts_runtime_tile_screen_origin(
    origin_x: i32,
    origin_y: i32,
    cell_w: i32,
    cell_h: i32,
    tile: (i32, i32),
) -> (i32, i32) {
    (origin_x + tile.0 * cell_w, origin_y + tile.1 * cell_h)
}

pub fn rts_runtime_tile_screen_rect(
    projection: RtsRuntimeMapProjection,
    tile: (i32, i32),
) -> RtsRuntimeRect {
    let (x, y) = rts_runtime_tile_screen_origin(
        projection.map_x,
        projection.map_y,
        projection.cell_w,
        projection.cell_h,
        tile,
    );
    RtsRuntimeRect {
        x,
        y,
        width: projection.cell_w,
        height: projection.cell_h,
    }
}

pub fn rts_runtime_terrain_seeds(tile: (i32, i32)) -> RtsRuntimeTerrainSeeds {
    RtsRuntimeTerrainSeeds {
        surface_seed: (tile.0 * 37 + tile.1 * 19 + (tile.0 - tile.1).abs() * 11) % 17,
        detail_seed: (tile.0 * 13 + tile.1 * 17 + (tile.0 - tile.1).abs() * 7) % 23,
    }
}

pub fn rts_runtime_tile_line(from: (i32, i32), to: (i32, i32)) -> Vec<RtsRuntimeTileLineStep> {
    let dx = to.0 - from.0;
    let dy = to.1 - from.1;
    let steps = dx.abs().max(dy.abs());
    if steps == 0 {
        return vec![RtsRuntimeTileLineStep {
            step_index: 0,
            step_count: 0,
            tile_x: from.0,
            tile_y: from.1,
        }];
    }

    (0..=steps)
        .map(|step| RtsRuntimeTileLineStep {
            step_index: step,
            step_count: steps,
            tile_x: from.0 + (dx * step) / steps,
            tile_y: from.1 + (dy * step) / steps,
        })
        .collect()
}

pub fn rts_large_map_tile_to_camera_center(tile: (i32, i32)) -> RtsRuntimeVec2 {
    let tile = rts_large_map_clamp_tile(tile);
    RtsRuntimeVec2::new(
        (tile.0 - TRNM_RTS_RUNTIME_CAMERA_ORIGIN_X) as f32 * TRNM_RTS_RUNTIME_TILE_WORLD_W,
        -((tile.1 - TRNM_RTS_RUNTIME_CAMERA_ORIGIN_Y) as f32) * TRNM_RTS_RUNTIME_TILE_WORLD_H,
    )
}

pub fn rts_minimap_cell_origin(
    origin_x: i32,
    origin_y: i32,
    cell_w: i32,
    cell_h: i32,
    tile: (i32, i32),
) -> (i32, i32) {
    let tile = rts_large_map_clamp_tile(tile);
    (
        origin_x + (tile.0 - TRNM_RTS_RUNTIME_MAP_MIN_TILE) * cell_w,
        origin_y + (tile.1 - TRNM_RTS_RUNTIME_MAP_MIN_TILE) * cell_h,
    )
}

pub fn rts_large_map_cell_col(tile: (i32, i32)) -> i32 {
    rts_large_map_clamp_tile(tile).0 - TRNM_RTS_RUNTIME_MAP_MIN_TILE
}

pub fn rts_large_map_cell_row(tile: (i32, i32)) -> i32 {
    rts_large_map_clamp_tile(tile).1 - TRNM_RTS_RUNTIME_MAP_MIN_TILE
}

pub fn rts_scrollable_map_camera_config() -> RtsScrollableMapCameraConfig {
    let min_camera = rts_large_map_tile_to_camera_center((
        TRNM_RTS_RUNTIME_MAP_MIN_TILE,
        TRNM_RTS_RUNTIME_MAP_MAX_Y,
    ));
    let max_camera = rts_large_map_tile_to_camera_center((
        TRNM_RTS_RUNTIME_MAP_MAX_X,
        TRNM_RTS_RUNTIME_MAP_MIN_TILE,
    ));
    RtsScrollableMapCameraConfig {
        min_x: min_camera.x,
        max_x: max_camera.x,
        min_y: min_camera.y,
        max_y: max_camera.y,
        min_zoom: 0.66,
        max_zoom: 1.85,
        keyboard_speed: 280.0,
        edge_speed: 360.0,
        drag_world_units_per_pixel: 1.15,
        wheel_zoom_step: 0.12,
        edge_band_pixels: 24.0,
    }
}

pub fn clamp_rts_scrollable_map_camera_state(
    state: RtsScrollableMapCameraState,
    config: RtsScrollableMapCameraConfig,
) -> RtsScrollableMapCameraState {
    RtsScrollableMapCameraState {
        center_x: state.center_x.clamp(config.min_x, config.max_x),
        center_y: state.center_y.clamp(config.min_y, config.max_y),
        zoom: state.zoom.clamp(config.min_zoom, config.max_zoom),
    }
}

pub fn apply_rts_scrollable_map_camera_input(
    source: &str,
    state: RtsScrollableMapCameraState,
    config: RtsScrollableMapCameraConfig,
    pan_delta: RtsRuntimeVec2,
    zoom_delta: f32,
    minimap_jump: Option<(&str, RtsRuntimeVec2)>,
) -> RtsScrollableMapCameraStep {
    let mut next = state;
    if let Some((_tile_id, center)) = minimap_jump {
        next.center_x = center.x;
        next.center_y = center.y;
    } else {
        next.center_x += pan_delta.x;
        next.center_y += pan_delta.y;
    }
    next.zoom += zoom_delta;
    let clamped_next = clamp_rts_scrollable_map_camera_state(next, config);
    RtsScrollableMapCameraStep {
        source: source.to_string(),
        before: state,
        after: clamped_next,
        pan_delta_x: pan_delta.x,
        pan_delta_y: pan_delta.y,
        zoom_delta,
        clamped: (clamped_next.center_x - next.center_x).abs() > f32::EPSILON
            || (clamped_next.center_y - next.center_y).abs() > f32::EPSILON
            || (clamped_next.zoom - next.zoom).abs() > f32::EPSILON,
        minimap_tile_id: minimap_jump.map(|(tile_id, _)| tile_id.to_string()),
    }
}

pub fn rts_scrollable_map_camera_focus_tile(state: RtsScrollableMapCameraState) -> (i32, i32) {
    rts_large_map_clamp_tile((
        (state.center_x / TRNM_RTS_RUNTIME_TILE_WORLD_W).round() as i32
            + TRNM_RTS_RUNTIME_CAMERA_ORIGIN_X,
        (-state.center_y / TRNM_RTS_RUNTIME_TILE_WORLD_H).round() as i32
            + TRNM_RTS_RUNTIME_CAMERA_ORIGIN_Y,
    ))
}

pub fn rts_camera_minimap_viewport_rect(
    state: RtsScrollableMapCameraState,
    minimap_width: i32,
    minimap_height: i32,
) -> RtsCameraMinimapViewportRect {
    let config = rts_scrollable_map_camera_config();
    let normalized_x =
        ((state.center_x - config.min_x) / (config.max_x - config.min_x)).clamp(0.0, 1.0);
    let normalized_y =
        ((state.center_y - config.min_y) / (config.max_y - config.min_y)).clamp(0.0, 1.0);
    let width = ((minimap_width as f32 * 0.28) / state.zoom).round() as i32;
    let height = ((minimap_height as f32 * 0.34) / state.zoom).round() as i32;
    let width = width.clamp(18, (minimap_width - 8).max(18));
    let height = height.clamp(14, (minimap_height - 8).max(14));
    let max_x = (minimap_width - width).max(0);
    let max_y = (minimap_height - height).max(0);
    RtsCameraMinimapViewportRect {
        x: ((normalized_x * max_x as f32).round() as i32).clamp(0, max_x),
        y: (((1.0 - normalized_y) * max_y as f32).round() as i32).clamp(0, max_y),
        width,
        height,
    }
}

pub fn rts_camera_minimap_revealed_tiles(focus_tile: (i32, i32)) -> Vec<String> {
    let mut tile_ids = Vec::new();
    for y_delta in -1..=1 {
        for x_delta in -1..=1 {
            let (tile_x, tile_y) =
                rts_large_map_clamp_tile((focus_tile.0 + x_delta, focus_tile.1 + y_delta));
            let tile_id = rts_runtime_tile_id((tile_x, tile_y));
            if !tile_ids.contains(&tile_id) {
                tile_ids.push(tile_id);
            }
        }
    }
    tile_ids
}

pub fn rts_camera_minimap_selection_follow_step(
    source: &str,
    state: RtsScrollableMapCameraState,
    selected_unit_id: &str,
    selected_unit_center: RtsRuntimeVec2,
) -> RtsScrollableMapCameraStep {
    apply_rts_scrollable_map_camera_input(
        source,
        state,
        rts_scrollable_map_camera_config(),
        RtsRuntimeVec2::ZERO,
        0.0,
        Some((selected_unit_id, selected_unit_center)),
    )
}

pub fn rts_scrollable_map_viewport_center() -> RtsRuntimeVec2 {
    rts_large_map_tile_to_camera_center((8, 8))
}

pub fn rts_scrollable_map_default_camera_state() -> RtsScrollableMapCameraState {
    let center = rts_scrollable_map_viewport_center();
    RtsScrollableMapCameraState {
        center_x: center.x,
        center_y: center.y,
        zoom: 1.0,
    }
}

pub fn rts_scrollable_map_camera_stage_summaries() -> Vec<RtsScrollableMapCameraStageSummary> {
    let config = rts_scrollable_map_camera_config();
    let stages: Vec<(
        &str,
        &str,
        RtsRuntimeVec2,
        f32,
        Option<(&str, RtsRuntimeVec2)>,
    )> = vec![
        (
            "keyboard_pan",
            "shift_keyboard_pan",
            RtsRuntimeVec2::new(84.0, 42.0),
            0.0,
            None,
        ),
        (
            "edge_scroll",
            "edge_scroll",
            RtsRuntimeVec2::new(config.edge_speed * 0.20, -config.edge_speed * 0.12),
            0.0,
            None,
        ),
        (
            "middle_mouse_drag",
            "middle_mouse_drag",
            RtsRuntimeVec2::new(-128.0, 68.0),
            0.0,
            None,
        ),
        ("wheel_zoom", "wheel_zoom", RtsRuntimeVec2::ZERO, 0.28, None),
        (
            "minimap_jump",
            "minimap_jump",
            RtsRuntimeVec2::ZERO,
            0.0,
            Some(("minimap_cursor_jump", RtsRuntimeVec2::new(260.0, -140.0))),
        ),
        (
            "bounds_clamp",
            "shift_keyboard_pan+wheel_zoom",
            RtsRuntimeVec2::new(980.0, 720.0),
            2.6,
            None,
        ),
    ];
    let mut camera_state = rts_scrollable_map_default_camera_state();
    stages
        .into_iter()
        .map(|(stage, source, pan_delta, zoom_delta, minimap_jump)| {
            let step = apply_rts_scrollable_map_camera_input(
                source,
                camera_state,
                config,
                pan_delta,
                zoom_delta,
                minimap_jump,
            );
            camera_state = step.after;
            let focus_tile = rts_scrollable_map_camera_focus_tile(step.after);
            RtsScrollableMapCameraStageSummary {
                stage: stage.to_string(),
                source: source.to_string(),
                step,
                focus_tile,
                command_destination_tile: Some(rts_runtime_tile_id(focus_tile)),
                command_queue: vec![
                    source.to_string(),
                    "scrollable_map_camera:viewport_update".to_string(),
                ],
            }
        })
        .collect()
}

pub fn rts_camera_minimap_sync_stage_summaries() -> Vec<RtsCameraMinimapSyncStageSummary> {
    let config = rts_scrollable_map_camera_config();
    let stages: Vec<(
        &str,
        &str,
        RtsRuntimeVec2,
        f32,
        Option<(&str, RtsRuntimeVec2)>,
        &str,
        &str,
    )> = vec![
        (
            "viewport_rect",
            "camera_viewport_rect",
            RtsRuntimeVec2::ZERO,
            0.0,
            None,
            "mirror_captain",
            "1",
        ),
        (
            "fog_reveal",
            "edge_scroll",
            RtsRuntimeVec2::new(92.0, -54.0),
            0.0,
            None,
            "mirror_captain",
            "1",
        ),
        (
            "selection_follow",
            "selection_follow",
            RtsRuntimeVec2::ZERO,
            0.0,
            Some(("mirror_captain", RtsRuntimeVec2::new(210.0, -96.0))),
            "mirror_captain",
            "1",
        ),
        (
            "control_group_recall",
            "control_group_recall_camera",
            RtsRuntimeVec2::new(-68.0, 58.0),
            0.0,
            None,
            "field_engineer",
            "2",
        ),
        (
            "route_projection",
            "minimap_route_projection",
            RtsRuntimeVec2::ZERO,
            0.0,
            Some(("minimap_route_target", RtsRuntimeVec2::new(340.0, -128.0))),
            "signal_lancer",
            "2",
        ),
        (
            "zoom_sync",
            "wheel_zoom",
            RtsRuntimeVec2::ZERO,
            0.52,
            None,
            "mirror_captain",
            "1",
        ),
    ];
    let mut camera_state = rts_scrollable_map_default_camera_state();
    stages
        .into_iter()
        .map(
            |(
                stage,
                source,
                pan_delta,
                zoom_delta,
                minimap_jump,
                selected_unit_id,
                control_group_id,
            )| {
                let step = if source == "selection_follow" {
                    let (_, unit_center) =
                        minimap_jump.expect("selection follow stage carries selected unit center");
                    rts_camera_minimap_selection_follow_step(
                        source,
                        camera_state,
                        selected_unit_id,
                        unit_center,
                    )
                } else {
                    apply_rts_scrollable_map_camera_input(
                        source,
                        camera_state,
                        config,
                        pan_delta,
                        zoom_delta,
                        minimap_jump,
                    )
                };
                camera_state = step.after;
                let focus_tile = rts_scrollable_map_camera_focus_tile(step.after);
                let viewport_rect = rts_camera_minimap_viewport_rect(step.after, 117, 56);
                let revealed_tile_ids = rts_camera_minimap_revealed_tiles(focus_tile);
                let command_destination_tile = Some(rts_runtime_tile_id(focus_tile));
                let minimap_command_tile_id = step
                    .minimap_tile_id
                    .clone()
                    .or_else(|| command_destination_tile.clone());
                RtsCameraMinimapSyncStageSummary {
                    stage: stage.to_string(),
                    source: source.to_string(),
                    step,
                    focus_tile,
                    viewport_rect,
                    viewport_rect_area: viewport_rect.width * viewport_rect.height,
                    revealed_tile_ids: revealed_tile_ids.clone(),
                    selected_unit_id: selected_unit_id.to_string(),
                    control_group_id: control_group_id.to_string(),
                    command_destination_tile,
                    minimap_command_tile_id,
                    command_queue: vec![
                        source.to_string(),
                        format!("camera_minimap_sync:{stage}"),
                        format!("reveal_tiles:{}", revealed_tile_ids.len()),
                    ],
                }
            },
        )
        .collect()
}

pub fn rts_runtime_tile_id(tile: (i32, i32)) -> String {
    format!("{},{}", tile.0, tile.1)
}

pub fn rts_catalog_text_label(text: &str, max_chars: usize) -> String {
    text.replace('_', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_uppercase()
        .chars()
        .take(max_chars)
        .collect()
}

fn rts_string_vec<const N: usize>(values: [&str; N]) -> Vec<String> {
    values.into_iter().map(str::to_string).collect()
}

fn rts_push_unique_string(values: &mut Vec<String>, value: &str) {
    if !values.iter().any(|existing| existing == value) {
        values.push(value.to_string());
    }
}

pub fn rts_line_path_tiles(start: (i32, i32), end: (i32, i32)) -> Vec<String> {
    let dx = end.0 - start.0;
    let dy = end.1 - start.1;
    let steps = dx.abs().max(dy.abs()).max(1);
    let mut tiles = Vec::new();
    for step in 1..=steps {
        let tile = (start.0 + (dx * step) / steps, start.1 + (dy * step) / steps);
        let tile_id = rts_runtime_tile_id(tile);
        if tiles.last() != Some(&tile_id) {
            tiles.push(tile_id);
        }
    }
    tiles
}

fn rts_parse_tile_id(value: &str) -> Option<(i32, i32)> {
    let (x, y) = value.split_once(',')?;
    Some((x.parse().ok()?, y.parse().ok()?))
}

pub fn rts_default_group_units() -> Vec<String> {
    rts_string_vec([
        "player",
        "square_guard_patrol",
        "square_worker_carry",
        "square_creep_wander",
    ])
}

pub fn rts_group_two_units() -> Vec<String> {
    rts_string_vec(["square_guard_patrol", "square_creep_wander"])
}

pub fn rts_unit_selection_class(unit_id: &str) -> &'static str {
    if unit_id.contains("guard") || unit_id == "player" {
        "guard"
    } else if unit_id.contains("worker") {
        "worker"
    } else if unit_id.contains("creep") {
        "creep"
    } else {
        "unit"
    }
}

pub fn rts_same_class_units(unit_id: &str) -> Vec<String> {
    match rts_unit_selection_class(unit_id) {
        "guard" => rts_string_vec(["player", "square_guard_front", "square_guard_patrol"]),
        "worker" => rts_string_vec(["square_worker_carry", "square_worker_harvest"]),
        "creep" => rts_string_vec(["square_creep_wander"]),
        _ => vec![unit_id.to_string()],
    }
}

fn rts_selectable_unit_entries() -> [(&'static str, (i32, i32), &'static str, u8); 6] {
    [
        ("player", (5, 4), "player", 0),
        ("square_guard_front", (5, 4), "player", 1),
        ("square_guard_patrol", (7, 5), "player", 2),
        ("square_worker_carry", (4, 5), "player", 3),
        ("square_worker_harvest", (8, 5), "player", 4),
        ("square_creep_wander", (9, 4), "hostile", 20),
    ]
}

pub fn rts_unit_allegiance(unit_id: &str) -> &'static str {
    rts_selectable_unit_entries()
        .into_iter()
        .find_map(|(entry_unit_id, _, allegiance, _)| {
            (entry_unit_id == unit_id).then_some(allegiance)
        })
        .unwrap_or("unknown")
}

pub fn rts_unit_is_player_owned(unit_id: &str) -> bool {
    rts_unit_allegiance(unit_id) == "player"
}

pub fn rts_unit_selection_priority(unit_id: &str) -> u8 {
    rts_selectable_unit_entries()
        .into_iter()
        .find_map(|(entry_unit_id, _, _, priority)| (entry_unit_id == unit_id).then_some(priority))
        .unwrap_or(u8::MAX)
}

pub fn rts_selectable_unit_tile(unit_id: &str) -> Option<(i32, i32)> {
    rts_selectable_unit_entries()
        .into_iter()
        .find_map(|(entry_unit_id, tile, _, _)| (entry_unit_id == unit_id).then_some(tile))
}

pub fn rts_selectable_unit_at_tile(tile: (i32, i32)) -> Option<&'static str> {
    rts_selectable_unit_entries()
        .into_iter()
        .filter(|(_, unit_tile, _, _)| *unit_tile == tile)
        .min_by_key(|(unit_id, _, allegiance, priority)| {
            let allegiance_priority = if *allegiance == "player" { 0 } else { 1 };
            (allegiance_priority, *priority, *unit_id)
        })
        .map(|(unit_id, _, _, _)| unit_id)
}

pub fn rts_selection_clear_parts(group_id: &str) -> Option<(String, Option<String>, String)> {
    let payload = group_id.strip_prefix("clear:")?;
    if let Some(tile_id) = payload.strip_prefix("empty@") {
        return Some(("empty".to_string(), None, tile_id.to_string()));
    }
    if let Some(hostile_payload) = payload.strip_prefix("hostile:") {
        let (unit_id, tile_id) = hostile_payload.split_once('@')?;
        return Some((
            "hostile".to_string(),
            Some(unit_id.to_string()),
            tile_id.to_string(),
        ));
    }
    None
}

pub fn rts_selection_tiles_for_units(unit_ids: &[String]) -> Vec<String> {
    let mut tiles = Vec::new();
    for unit_id in unit_ids {
        if let Some(tile) = rts_selectable_unit_tile(unit_id) {
            rts_push_unique_string(&mut tiles, &rts_runtime_tile_id(tile));
        }
    }
    tiles
}

pub fn rts_selection_box_tiles() -> Vec<String> {
    rts_string_vec(["5,5", "6,5", "5,4", "6,4"])
}

pub fn rts_control_group_hotkey_slot(group_id: &str, prefix: &str) -> Option<String> {
    group_id
        .strip_prefix(prefix)
        .map(str::trim)
        .filter(|slot| !slot.is_empty())
        .map(ToOwned::to_owned)
}

pub fn rts_default_units_for_control_group_slot(slot: &str) -> Vec<String> {
    match slot {
        "2" => rts_group_two_units(),
        "3" => rts_string_vec(["square_worker_carry", "square_worker_harvest"]),
        _ => rts_default_group_units(),
    }
}

pub fn rts_units_from_control_group_assignment(assignments: &[String], slot: &str) -> Vec<String> {
    let prefix = format!("{slot}:");
    for assignment in assignments.iter().rev() {
        let Some(payload) = assignment.strip_prefix(&prefix) else {
            continue;
        };
        let unit_payload = payload.rsplit(':').next().unwrap_or(payload);
        let units = unit_payload
            .split('|')
            .map(str::trim)
            .filter(|unit| !unit.is_empty())
            .filter(|unit| rts_selectable_unit_tile(unit).is_some())
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        if !units.is_empty() {
            return units;
        }
    }
    Vec::new()
}

pub fn rts_control_group_slot_label(slot: &str) -> &str {
    if slot == "10" {
        "0"
    } else {
        slot
    }
}

pub fn rts_control_group_slot_member_count(assignments: &[String], slot: &str) -> usize {
    rts_units_from_control_group_assignment(assignments, slot).len()
}

pub fn rts_control_group_slot_is_active(
    active_group_ids: &[String],
    current_group_id: Option<&str>,
    slot: &str,
) -> bool {
    active_group_ids.iter().any(|group| group == slot) || current_group_id == Some(slot)
}

pub fn rts_control_group_slot_summaries(
    assignments: &[String],
    active_group_ids: &[String],
    current_group_id: Option<&str>,
) -> Vec<RtsControlGroupSlotSummary> {
    (1..=10)
        .map(|slot_index| {
            let slot = slot_index.to_string();
            let member_count = rts_control_group_slot_member_count(assignments, &slot);
            let active =
                rts_control_group_slot_is_active(active_group_ids, current_group_id, &slot);
            RtsControlGroupSlotSummary {
                key_label: rts_control_group_slot_label(&slot).to_string(),
                slot,
                member_count,
                occupied: member_count > 0,
                active,
            }
        })
        .collect()
}

pub fn rts_merged_unit_ids(base_units: &[String], extra_units: &[String]) -> Vec<String> {
    let mut merged = base_units.to_vec();
    for unit_id in extra_units {
        rts_push_unique_string(&mut merged, unit_id);
    }
    merged
}

pub fn rts_drag_selection_parts(group_id: &str) -> Option<((i32, i32), (i32, i32))> {
    let payload = group_id.strip_prefix("drag:")?;
    let (start, end) = payload.split_once("->")?;
    Some((rts_parse_tile_id(start)?, rts_parse_tile_id(end)?))
}

pub fn rts_drag_distance_sq(start: (i32, i32), end: (i32, i32)) -> i32 {
    let dx = end.0 - start.0;
    let dy = end.1 - start.1;
    dx * dx + dy * dy
}

pub fn rts_drag_select_ready(start: (i32, i32), end: (i32, i32)) -> bool {
    rts_drag_distance_sq(start, end) >= 36
}

pub fn rts_drag_group_id(start_tile: (i32, i32), end_tile: (i32, i32)) -> String {
    format!(
        "drag:{}->{}",
        rts_runtime_tile_id(start_tile),
        rts_runtime_tile_id(end_tile)
    )
}

pub fn rts_drag_select_player_label(
    start_tile_id: &str,
    current_tile_id: &str,
    unit_count: usize,
) -> String {
    format!(
        "DRAG SELECT {} {} {}->{}",
        unit_count,
        if unit_count == 1 { "UNIT" } else { "UNITS" },
        start_tile_id,
        current_tile_id
    )
}

pub fn rts_selection_box_tiles_between(start: (i32, i32), end: (i32, i32)) -> Vec<String> {
    let start = rts_large_map_clamp_tile(start);
    let end = rts_large_map_clamp_tile(end);
    let min_x = start.0.min(end.0);
    let max_x = start.0.max(end.0);
    let min_y = start.1.min(end.1);
    let max_y = start.1.max(end.1);
    let mut tiles = Vec::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            tiles.push(rts_runtime_tile_id((x, y)));
        }
    }
    tiles
}

pub fn rts_drag_selected_units(start: (i32, i32), end: (i32, i32)) -> Vec<String> {
    rts_drag_units_between(start, end, true)
}

pub fn rts_drag_rejected_unit_ids(start: (i32, i32), end: (i32, i32)) -> Vec<String> {
    rts_drag_units_between(start, end, false)
        .into_iter()
        .filter(|unit_id| !rts_unit_is_player_owned(unit_id))
        .collect()
}

fn rts_drag_units_between(start: (i32, i32), end: (i32, i32), owned_only: bool) -> Vec<String> {
    let start = rts_large_map_clamp_tile(start);
    let end = rts_large_map_clamp_tile(end);
    let min_x = start.0.min(end.0);
    let max_x = start.0.max(end.0);
    let min_y = start.1.min(end.1);
    let max_y = start.1.max(end.1);
    let mut selected = Vec::new();
    for (unit_id, tile, _, _) in rts_selectable_unit_entries() {
        if tile.0 >= min_x && tile.0 <= max_x && tile.1 >= min_y && tile.1 <= max_y {
            if !owned_only || rts_unit_is_player_owned(unit_id) {
                rts_push_unique_string(&mut selected, unit_id);
            }
        }
    }
    selected
}

pub fn rts_move_follow_target(formation: &str) -> Option<&str> {
    formation
        .strip_prefix("follow:")
        .map(str::trim)
        .filter(|target_id| !target_id.is_empty())
}

pub fn rts_move_command_parts(command_id: &str) -> (&str, &str) {
    let command_payload = command_id.strip_prefix("minimap:").unwrap_or(command_id);
    let mut parts = command_payload.splitn(2, ':');
    let tile_id = parts
        .next()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("7,4");
    let formation = parts
        .next()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("diamond");
    (tile_id, formation)
}

pub fn rts_move_formation_kind(formation: &str) -> &str {
    if rts_move_follow_target(formation).is_some() {
        "follow"
    } else {
        formation
    }
}

pub fn rts_path_tiles_for_destination(destination_tile: (i32, i32)) -> Vec<String> {
    if destination_tile == (8, 4) {
        vec!["6,5".to_string(), "7,5".to_string(), "8,4".to_string()]
    } else if destination_tile == (9, 2) {
        vec![
            "6,5".to_string(),
            "7,4".to_string(),
            "8,3".to_string(),
            "9,2".to_string(),
        ]
    } else {
        rts_line_path_tiles((5, 5), destination_tile)
    }
}

pub fn rts_blocked_tiles_for_destination(destination_tile: (i32, i32)) -> Vec<String> {
    if destination_tile == (8, 4) {
        vec!["7,4".to_string()]
    } else {
        Vec::new()
    }
}

pub fn rts_formation_slots_for_destination(
    destination_tile: (i32, i32),
    formation: &str,
) -> Vec<String> {
    let (x, y) = destination_tile;
    let slots = match formation {
        "line" => [(x - 1, y), (x, y), (x + 1, y), (x + 2, y)],
        "rally" => [(x - 1, y + 1), (x, y), (x + 1, y), (x, y + 1)],
        "split" => [(x - 1, y), (x + 1, y), (x - 1, y + 1), (x + 1, y + 1)],
        "wedge" => [(x, y), (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)],
        _ => [(x, y), (x - 1, y), (x, y + 1), (x + 1, y)],
    };
    slots.into_iter().map(rts_runtime_tile_id).collect()
}

pub fn rts_disperse_slots_for_destination(destination_tile: (i32, i32)) -> Vec<String> {
    if destination_tile == (8, 4) {
        vec![
            "6,5".to_string(),
            "7,5".to_string(),
            "8,4".to_string(),
            "8,5".to_string(),
        ]
    } else if destination_tile == (6, 5) {
        vec![
            "5,5".to_string(),
            "6,4".to_string(),
            "6,6".to_string(),
            "7,5".to_string(),
        ]
    } else {
        Vec::new()
    }
}

pub fn rts_engagement_tiles_for_target(target_id: &str) -> Vec<String> {
    if target_id == "enemy_barracks" {
        rts_string_vec(["9,3", "10,3", "10,2", "11,2"])
    } else if target_id == "forest_creep_camp" {
        rts_string_vec(["8,3", "8,2", "9,3", "7,3"])
    } else if target_id == "square_creep_wander" {
        rts_string_vec(["8,4", "9,4", "9,3", "10,4"])
    } else if target_id == "arena_creep_attack" {
        rts_string_vec(["6,5", "6,4", "7,5", "5,5"])
    } else {
        rts_string_vec(["6,5", "6,4"])
    }
}

pub fn rts_contact_flash_tiles_for_target(target_id: &str) -> Vec<String> {
    if target_id == "enemy_barracks" {
        rts_string_vec(["10,3", "10,2", "11,2"])
    } else if target_id == "forest_creep_camp" {
        rts_string_vec(["8,3", "9,3"])
    } else if target_id == "square_creep_wander" {
        rts_string_vec(["9,4", "10,4"])
    } else if target_id == "arena_creep_attack" {
        rts_string_vec(["6,5", "6,4"])
    } else {
        rts_string_vec(["6,5"])
    }
}

pub fn rts_target_tile_for_id(target_id: &str, fallback_index: usize) -> (i32, i32) {
    match target_id {
        "arena_creep_attack" => (6, 5),
        "arena_guard_support" => (6, 4),
        "arena_worker_support" => (7, 5),
        "forest_creep_camp" => (8, 3),
        "forest_stalker_support" => (8, 2),
        "forest_shaman_support" => (9, 3),
        "square_creep_wander" => (9, 4),
        "enemy_watch_post" => (10, 2),
        "enemy_barracks" => (10, 3),
        "enemy_resource_vault" => (11, 2),
        _ => (6 + fallback_index as i32, 5),
    }
}

pub fn rts_target_priority_ids_for_target(target_id: &str) -> Vec<String> {
    if target_id == "enemy_barracks" {
        rts_string_vec(["enemy_barracks", "enemy_watch_post", "enemy_resource_vault"])
    } else if target_id == "forest_creep_camp" {
        rts_string_vec([
            "forest_creep_camp",
            "forest_stalker_support",
            "forest_shaman_support",
        ])
    } else if target_id == "arena_creep_attack" {
        rts_string_vec([
            "arena_creep_attack",
            "arena_guard_support",
            "arena_worker_support",
        ])
    } else if target_id == "square_creep_wander" {
        rts_string_vec([
            "square_creep_wander",
            "forest_creep_camp",
            "arena_creep_attack",
        ])
    } else {
        vec![target_id.to_string()]
    }
}

pub fn rts_focus_fire_units_for_target(target_id: &str) -> Vec<String> {
    if target_id == "enemy_barracks" {
        rts_army_units_for_batch("mixed_vanguard")
    } else if target_id == "forest_creep_camp"
        || target_id == "arena_creep_attack"
        || target_id == "square_creep_wander"
    {
        rts_default_group_units()
    } else {
        rts_string_vec(["player", "square_guard_patrol"])
    }
}

pub fn rts_threat_levels_for_target(target_id: &str) -> Vec<u8> {
    if target_id == "enemy_barracks" {
        vec![88, 66, 41]
    } else if target_id == "forest_creep_camp" {
        vec![92, 70, 46]
    } else if target_id == "square_creep_wander" {
        vec![86, 54, 28]
    } else if target_id == "arena_creep_attack" {
        vec![100, 64, 32]
    } else {
        vec![72]
    }
}

pub fn rts_projectile_trail_tiles_for_target(target_id: &str) -> Vec<String> {
    if target_id == "enemy_barracks" {
        rts_string_vec(["7,4", "8,4", "9,3", "10,3"])
    } else if target_id == "forest_creep_camp" {
        rts_string_vec(["5,5", "6,5", "7,4", "8,3"])
    } else if target_id == "square_creep_wander" {
        rts_string_vec(["5,5", "6,5", "8,4", "9,4"])
    } else if target_id == "arena_creep_attack" {
        rts_string_vec(["5,5", "5,4", "6,4", "6,5"])
    } else {
        rts_string_vec(["5,5", "6,5"])
    }
}

pub fn rts_ability_effect_tiles_for_target(target_id: &str, ability_id: &str) -> Vec<String> {
    if target_id == "enemy_barracks" && ability_id == "guard_break" {
        rts_string_vec(["10,3", "10,2", "11,2", "9,3"])
    } else if target_id == "forest_creep_camp" && ability_id == "guard_break" {
        rts_string_vec(["8,3", "8,2", "9,3", "7,3"])
    } else if target_id == "forest_creep_camp" {
        rts_string_vec(["8,3", "8,2", "9,3"])
    } else if target_id == "arena_creep_attack" && ability_id == "guard_break" {
        rts_string_vec(["6,5", "6,4", "7,5", "5,5"])
    } else if target_id == "arena_creep_attack" {
        rts_string_vec(["6,5", "6,4", "7,5"])
    } else {
        vec![target_id.to_string()]
    }
}

pub fn rts_damage_ticks_for_ability(ability_id: &str) -> Vec<u8> {
    match ability_id {
        "guard_break" => vec![16, 21, 35],
        "focus_fire" => vec![28],
        _ => vec![18],
    }
}

pub fn rts_projectile_id_for_ability(ability_id: &str) -> &'static str {
    match ability_id {
        "guard_break" => "guard_break_bolt",
        "focus_fire" => "focus_fire_volley",
        _ => "guard_volley",
    }
}

pub fn rts_ai_wave_unit_ids_for_pressure(pressure_id: &str) -> Vec<String> {
    if pressure_id == "skirmish_wave" {
        rts_string_vec(["lane_scout", "mirror_raider", "siege_runner"])
    } else {
        rts_string_vec(["lane_scout"])
    }
}

pub fn rts_ai_pressure_tiles_for_pressure(pressure_id: &str) -> Vec<String> {
    if pressure_id == "skirmish_wave" {
        rts_string_vec(["9,3", "8,4", "7,4", "6,5"])
    } else {
        rts_string_vec(["8,4", "7,4"])
    }
}

pub fn rts_ai_counter_tiles_for_pressure(pressure_id: &str) -> Vec<String> {
    if pressure_id == "skirmish_wave" {
        rts_string_vec(["5,5", "6,5", "6,4", "7,5"])
    } else {
        rts_string_vec(["5,5", "6,5"])
    }
}

pub fn rts_enemy_pressure_wave_units_for_id(wave_id: &str) -> Vec<String> {
    if wave_id == "raider_wave" {
        rts_string_vec(["enemy_raider", "enemy_signal_guard", "enemy_sapper"])
    } else {
        rts_string_vec(["enemy_raider"])
    }
}

pub fn rts_enemy_pressure_lane_tiles_for_wave(wave_id: &str) -> Vec<String> {
    if wave_id == "raider_wave" {
        rts_string_vec(["10,2", "9,3", "8,4", "7,4", "6,5"])
    } else {
        rts_string_vec(["9,3", "8,4"])
    }
}

pub fn rts_scout_route_tiles_for_recon(recon_id: &str) -> Vec<String> {
    if recon_id == "enemy_base" {
        rts_string_vec(["5,5", "6,4", "7,4", "8,3", "9,2", "10,2"])
    } else if recon_id == "watchtower_scan" {
        rts_string_vec(["5,5", "6,5", "7,4"])
    } else {
        rts_string_vec(["5,5", "6,5", "7,5"])
    }
}

pub fn rts_fog_reveal_tiles_for_recon(recon_id: &str, kind: &str) -> Vec<String> {
    if recon_id == "enemy_base" && kind == "mark" {
        rts_string_vec([
            "7,4", "8,3", "8,2", "9,2", "9,3", "10,2", "10,3", "11,1", "11,2",
        ])
    } else if recon_id == "enemy_base" && kind == "sweep" {
        rts_string_vec(["7,4", "8,3", "9,2", "9,3", "10,2", "10,3", "11,2"])
    } else if recon_id == "enemy_base" {
        rts_string_vec(["7,4", "8,3", "9,2", "10,2"])
    } else if recon_id == "watchtower_scan" {
        rts_string_vec(["6,4", "7,4", "7,3", "8,3", "8,2"])
    } else {
        rts_string_vec(["5,5", "6,5", "7,5"])
    }
}

pub fn rts_enemy_structures_for_recon(recon_id: &str, kind: &str) -> Vec<String> {
    if recon_id == "enemy_base" && kind == "mark" {
        rts_string_vec(["enemy_watch_post", "enemy_barracks", "enemy_resource_vault"])
    } else if recon_id == "enemy_base" && kind == "sweep" {
        rts_string_vec(["enemy_watch_post", "enemy_barracks"])
    } else if recon_id == "enemy_base" || recon_id == "watchtower_scan" {
        rts_string_vec(["enemy_watch_post"])
    } else {
        Vec::new()
    }
}

pub fn rts_enemy_units_for_recon(recon_id: &str, kind: &str) -> Vec<String> {
    if recon_id == "enemy_base" && kind == "mark" {
        rts_string_vec(["enemy_scout", "enemy_worker", "enemy_guard"])
    } else if recon_id == "enemy_base" && kind == "sweep" {
        rts_string_vec(["enemy_scout", "enemy_worker"])
    } else if recon_id == "enemy_base" || recon_id == "watchtower_scan" {
        rts_string_vec(["enemy_scout"])
    } else {
        Vec::new()
    }
}

pub fn rts_enemy_structure_tile_for_id(structure_id: &str, index: usize) -> (i32, i32) {
    match structure_id {
        "enemy_watch_post" => (10, 2),
        "enemy_barracks" => (10, 3),
        "enemy_resource_vault" => (11, 2),
        _ => (10 + (index as i32 % 2), 2 + (index as i32 % 2)),
    }
}

pub fn rts_enemy_unit_tile_for_id(unit_id: &str, index: usize) -> (i32, i32) {
    match unit_id {
        "enemy_scout" => (9, 2),
        "enemy_worker" => (10, 3),
        "enemy_guard" => (11, 2),
        "enemy_raider" => (9, 3),
        "enemy_signal_guard" => (10, 3),
        "enemy_sapper" => (11, 2),
        _ => (9 + (index as i32 % 3), 2),
    }
}

pub fn rts_base_assault_path_tiles_for_target(target_id: &str, tile_id: &str) -> Vec<String> {
    if target_id == "enemy_barracks" {
        rts_string_vec(["5,5", "6,5", "7,4", "8,4", "9,3", tile_id])
    } else {
        let target_tile = rts_parse_tile_id(tile_id).unwrap_or((10, 3));
        rts_line_path_tiles((5, 5), target_tile)
    }
}

pub fn rts_base_assault_targets_for_id(target_id: &str) -> Vec<String> {
    if target_id == "enemy_barracks" {
        rts_string_vec(["enemy_watch_post", "enemy_barracks", "enemy_resource_vault"])
    } else {
        vec![target_id.to_string()]
    }
}

pub fn rts_aftermath_debris_tiles_for_id(structure_id: &str, tile_id: &str) -> Vec<String> {
    if structure_id == "enemy_barracks" {
        rts_string_vec(["9,3", "10,3", "10,4", "11,3"])
    } else {
        let tile = rts_parse_tile_id(tile_id).unwrap_or((10, 3));
        vec![
            format!("{},{}", tile.0.saturating_sub(1), tile.1),
            format!("{},{}", tile.0, tile.1),
            format!("{},{}", tile.0, tile.1 + 1),
        ]
    }
}

pub fn rts_aftermath_smoke_tiles_for_id(structure_id: &str, tile_id: &str) -> Vec<String> {
    if structure_id == "enemy_barracks" {
        rts_string_vec(["10,2", "10,3", "11,3"])
    } else {
        let tile = rts_parse_tile_id(tile_id).unwrap_or((10, 3));
        vec![
            format!("{},{}", tile.0, tile.1.saturating_sub(1)),
            format!("{},{}", tile.0, tile.1),
        ]
    }
}

pub fn rts_commander_aura_tiles_for_id(commander_id: &str) -> Vec<String> {
    if commander_id == "mirror_captain" {
        rts_string_vec(["6,5", "7,4", "8,4", "9,3", "10,3"])
    } else {
        rts_string_vec(["5,5", "6,5", "7,4"])
    }
}

pub fn rts_loot_items_for_id(source_id: &str) -> Vec<String> {
    if source_id == "enemy_barracks" {
        rts_string_vec([
            "barracks_map_cache",
            "field_banner_relic",
            "repair_kit_crate",
        ])
    } else {
        vec![format!("{source_id}_field_cache")]
    }
}

pub fn rts_expansion_tiles_for_id(expansion_id: &str, tile_id: &str) -> Vec<String> {
    if expansion_id == "forest_relay" {
        rts_string_vec(["8,2", "9,2", "10,2", "9,3", "10,3"])
    } else {
        let tile = rts_parse_tile_id(tile_id).unwrap_or((9, 2));
        vec![
            format!("{},{}", tile.0.saturating_sub(1), tile.1),
            format!("{},{}", tile.0, tile.1),
            format!("{},{}", tile.0 + 1, tile.1),
        ]
    }
}

pub fn rts_expansion_structure_tile_for_id(structure_id: &str) -> (i32, i32) {
    match structure_id {
        "relay_outpost" => (9, 2),
        "relay_foundry" => (9, 2),
        "relay_storehouse" => (10, 2),
        "watch_lantern" => (8, 3),
        _ => (9, 2),
    }
}

pub fn rts_expansion_workers_for_line(line_id: &str) -> Vec<String> {
    if line_id == "gold_line" {
        rts_string_vec([
            "expansion_worker_alpha",
            "expansion_worker_beta",
            "expansion_worker_gamma",
        ])
    } else {
        vec![format!("{line_id}_worker")]
    }
}

pub fn rts_counterattack_units_for_wave(wave_id: &str) -> Vec<String> {
    if wave_id == "counter_wave" {
        rts_string_vec([
            "counter_raider_alpha",
            "counter_raider_beta",
            "counter_sapper",
        ])
    } else {
        vec![format!("{wave_id}_raider")]
    }
}

pub fn rts_counterattack_route_tiles_for_wave(wave_id: &str, tile_id: &str) -> Vec<String> {
    if wave_id == "counter_wave" {
        rts_string_vec(["11,2", "10,2", "9,3", tile_id, "7,4", "9,2"])
    } else {
        let tile = rts_parse_tile_id(tile_id).unwrap_or((8, 3));
        vec![
            format!("{},{}", tile.0 + 2, tile.1.saturating_sub(1)),
            format!("{},{}", tile.0 + 1, tile.1),
            format!("{},{}", tile.0, tile.1),
        ]
    }
}

pub fn rts_army_units_for_batch(batch_id: &str) -> Vec<String> {
    match batch_id {
        "guard_pair" => rts_string_vec(["relay_guard_alpha", "relay_guard_beta"]),
        "wayfinder_pair" => rts_string_vec(["wayfinder_scout", "wayfinder_signal"]),
        "mixed_vanguard" => rts_string_vec([
            "relay_guard_alpha",
            "relay_guard_beta",
            "wayfinder_scout",
            "field_mender",
        ]),
        _ => vec![batch_id.to_string()],
    }
}

pub fn rts_army_rally_tiles_for_id(rally_id: &str) -> Vec<String> {
    match rally_id {
        "forward_watch" => rts_string_vec(["5,5", "6,5", "7,4", "8,4", "8,3"]),
        "forest_relay" => rts_string_vec(["5,5", "6,4", "7,4", "8,3", "9,2"]),
        _ => rts_string_vec(["5,5", "6,5", "7,4"]),
    }
}

pub fn rts_player_army_unit_tile_for_id(unit_id: &str, index: usize) -> (i32, i32) {
    match unit_id {
        "relay_guard_alpha" => (6, 5),
        "relay_guard_beta" => (7, 5),
        "wayfinder_scout" => (7, 4),
        "wayfinder_signal" => (8, 4),
        "field_mender" => (6, 4),
        _ => (6 + (index as i32 % 3), 5 - (index as i32 / 3)),
    }
}

pub fn rts_objective_parts(command: &str) -> (String, String, String) {
    let (kind, payload) = command.split_once(':').unwrap_or(("claim", command));
    let (objective_id, tile_id) = payload.split_once('@').unwrap_or((payload, "6,5"));
    (
        kind.to_string(),
        objective_id.to_string(),
        tile_id.to_string(),
    )
}

pub fn rts_creep_camp_parts(kind_hint: &str, command: &str) -> (String, String, String) {
    let (kind, payload) = if kind_hint == "camp" {
        command.split_once(':').unwrap_or(("clear", command))
    } else {
        (kind_hint, command)
    };
    let (camp_id, tile_id) = payload.split_once('@').unwrap_or((payload, "8,3"));
    let normalized_camp_id = if camp_id == "creep_camp" {
        "forest_creep_camp"
    } else {
        camp_id
    };
    (
        kind.to_string(),
        normalized_camp_id.to_string(),
        tile_id.to_string(),
    )
}

pub fn rts_recon_parts(command: &str) -> (String, String, String) {
    let (kind, payload) = command.split_once(':').unwrap_or(("scout", command));
    let (recon_id, tile_id) = payload.split_once('@').unwrap_or((payload, "10,2"));
    let normalized_recon_id = match recon_id {
        "scout_enemy_base" => "enemy_base",
        value => value,
    };
    (
        kind.to_string(),
        normalized_recon_id.to_string(),
        tile_id.to_string(),
    )
}

pub fn rts_enemy_command_parts(
    command: &str,
    fallback_kind: &str,
    fallback_source: &str,
) -> (String, String, String) {
    let (kind, payload) = command.split_once(':').unwrap_or((fallback_kind, command));
    let (id, source_id) = payload
        .split_once('@')
        .unwrap_or((payload, fallback_source));
    (kind.to_string(), id.to_string(), source_id.to_string())
}

pub fn rts_counter_command_parts(command: &str) -> (String, String, String) {
    let (kind, payload) = command.split_once(':').unwrap_or(("research", command));
    let (id, source_id) = payload.split_once('@').unwrap_or((payload, "signal_spire"));
    (kind.to_string(), id.to_string(), source_id.to_string())
}

pub fn rts_army_command_parts(command: &str) -> (String, String, String) {
    let (kind, payload) = command.split_once(':').unwrap_or(("train", command));
    let (id, source_id) = payload
        .split_once('@')
        .unwrap_or((payload, "training_hall"));
    (kind.to_string(), id.to_string(), source_id.to_string())
}

pub fn rts_base_assault_parts(command: &str) -> (String, String, String) {
    let (kind, payload) = command.split_once(':').unwrap_or(("breach", command));
    let (target_id, tile_id) = payload.split_once('@').unwrap_or((payload, "10,3"));
    (kind.to_string(), target_id.to_string(), tile_id.to_string())
}

pub fn rts_aftermath_parts(command: &str) -> (String, String, String) {
    let (kind, payload) = command.split_once(':').unwrap_or(("destroy", command));
    let (id, tile_id) = payload.split_once('@').unwrap_or((payload, "10,3"));
    (kind.to_string(), id.to_string(), tile_id.to_string())
}

pub fn rts_commander_parts(command: &str) -> (String, String, String) {
    let (kind, payload) = command.split_once(':').unwrap_or(("level", command));
    let (id, source_id) = payload
        .split_once('@')
        .unwrap_or((payload, "mirror_captain"));
    (kind.to_string(), id.to_string(), source_id.to_string())
}

pub fn rts_expansion_parts(command: &str) -> (String, String, String) {
    let (kind, payload) = command.split_once(':').unwrap_or(("claim", command));
    let (id, source_id) = payload.split_once('@').unwrap_or((payload, "9,2"));
    (kind.to_string(), id.to_string(), source_id.to_string())
}

pub fn rts_tier_two_parts(command: &str) -> (String, String, String) {
    let (kind, payload) = command.split_once(':').unwrap_or(("tech", command));
    let (id, source_id) = payload
        .split_once('@')
        .unwrap_or((payload, "relay_outpost"));
    (kind.to_string(), id.to_string(), source_id.to_string())
}

pub fn rts_objective_tiles_for_id(objective_id: &str, tile_id: &str) -> Vec<String> {
    if objective_id == "relay_beacon" {
        rts_string_vec(["6,5", "6,4", "7,5", "9,2"])
    } else if objective_id == "forest_relay" {
        rts_string_vec(["8,3", "9,2", "9,3"])
    } else {
        vec![tile_id.to_string()]
    }
}

pub fn rts_creep_camp_tiles_for_id(camp_id: &str, tile_id: &str) -> Vec<String> {
    if camp_id == "forest_creep_camp" {
        rts_string_vec(["8,3", "8,2", "9,3", "9,2"])
    } else {
        vec![tile_id.to_string()]
    }
}

pub fn rts_creep_camp_units_for_id(camp_id: &str) -> Vec<String> {
    if camp_id == "forest_creep_camp" {
        rts_string_vec(["forest_alpha_creep", "forest_stalker", "forest_shaman"])
    } else {
        rts_string_vec(["camp_scout"])
    }
}

pub fn rts_terrain_route_tiles_for_camp(camp_id: &str) -> Vec<String> {
    if camp_id == "forest_creep_camp" {
        rts_string_vec(["5,5", "6,5", "7,4", "8,3"])
    } else {
        rts_string_vec(["5,5", "6,5"])
    }
}

pub fn rts_terrain_choke_tiles_for_camp(camp_id: &str) -> Vec<String> {
    if camp_id == "forest_creep_camp" {
        rts_string_vec(["7,4", "7,3", "8,4"])
    } else {
        rts_string_vec(["6,5"])
    }
}

pub fn rts_expansion_tiles_for_camp(camp_id: &str) -> Vec<String> {
    if camp_id == "forest_creep_camp" {
        rts_string_vec(["9,2", "10,2", "10,3"])
    } else {
        rts_string_vec(["8,3"])
    }
}

pub fn rts_siege_units_for_id(unit_id: &str) -> Vec<String> {
    if unit_id == "stonebreak_cart" {
        rts_string_vec(["stonebreak_cart"])
    } else {
        vec![unit_id.to_string()]
    }
}

pub fn rts_siege_push_route_tiles_for_target(target_id: &str, tile_id: &str) -> Vec<String> {
    if target_id == "stonebreak_cart" || tile_id == "10,3" {
        rts_string_vec(["9,2", "9,3", "10,3", "10,2", "11,2", "10,3"])
    } else {
        let tile = rts_parse_tile_id(tile_id).unwrap_or((10, 3));
        vec![
            "9,2".to_string(),
            format!("{},{}", tile.0.saturating_sub(1), tile.1),
            format!("{},{}", tile.0, tile.1),
        ]
    }
}

pub fn rts_siege_breach_tiles_for_target(target_id: &str, tile_id: &str) -> Vec<String> {
    if target_id == "gate_bulwark" {
        rts_string_vec(["9,3", "10,3", "10,2", "11,2", "10,3"])
    } else {
        let tile = rts_parse_tile_id(tile_id).unwrap_or((10, 3));
        vec![
            format!("{},{}", tile.0.saturating_sub(1), tile.1),
            format!("{},{}", tile.0, tile.1),
            format!("{},{}", tile.0 + 1, tile.1),
        ]
    }
}

pub fn rts_enemy_fortification_tile_for_id(fortification_id: &str) -> (i32, i32) {
    match fortification_id {
        "gate_bulwark" => (10, 3),
        "watch_redoubt" => (10, 2),
        _ => (10, 3),
    }
}

pub fn rts_enemy_repair_units_for_target(target_id: &str) -> Vec<String> {
    if target_id == "gate_bulwark" {
        rts_string_vec(["repair_adept_alpha", "repair_adept_beta"])
    } else {
        vec![format!("{target_id}_repair_adept")]
    }
}

pub fn rts_enemy_flank_units_for_id(flank_id: &str) -> Vec<String> {
    if flank_id == "ridge_sentries" {
        rts_string_vec(["ridge_sentry_left", "ridge_sentry_right", "ridge_sapper"])
    } else {
        vec![format!("{flank_id}_flanker")]
    }
}

pub fn rts_enemy_flank_tile_for_index(index: usize) -> (i32, i32) {
    match index % 3 {
        0 => (9, 4),
        1 => (10, 4),
        _ => (8, 4),
    }
}

pub fn rts_player_hold_tiles_for_id(hold_id: &str, tile_id: &str) -> Vec<String> {
    if hold_id == "shield_line" {
        rts_string_vec(["8,3", "9,3", "9,4", "10,3"])
    } else {
        let tile = rts_parse_tile_id(tile_id).unwrap_or((9, 3));
        vec![
            format!("{},{}", tile.0.saturating_sub(1), tile.1),
            format!("{},{}", tile.0, tile.1),
            format!("{},{}", tile.0 + 1, tile.1),
        ]
    }
}

pub fn rts_inner_lane_tiles_for_id(lane_id: &str, tile_id: &str) -> Vec<String> {
    if lane_id == "inner_lane" {
        rts_string_vec(["10,3", "11,2", "11,3", "12,3", "12,4"])
    } else {
        let tile = rts_parse_tile_id(tile_id).unwrap_or((11, 2));
        vec![
            format!("{},{}", tile.0.saturating_sub(1), tile.1),
            format!("{},{}", tile.0, tile.1),
            format!("{},{}", tile.0 + 1, tile.1),
        ]
    }
}

pub fn rts_inner_gate_tile_for_id(gate_id: &str) -> (i32, i32) {
    match gate_id {
        "inner_latch" => (11, 3),
        "signal_lock" => (12, 3),
        _ => (11, 3),
    }
}

pub fn rts_inner_defenders_for_id(defender_id: &str) -> Vec<String> {
    if defender_id == "second_line" {
        rts_string_vec(["inner_guard_alpha", "inner_guard_beta", "signal_lancer"])
    } else {
        vec![format!("{defender_id}_guard")]
    }
}

pub fn rts_supply_convoy_for_id(convoy_id: &str) -> Vec<String> {
    if convoy_id == "relay_convoy" {
        rts_string_vec(["convoy_cart", "field_medic", "ammo_runner"])
    } else {
        vec![format!("{convoy_id}_cart")]
    }
}

pub fn rts_split_squad_tiles_for_id(split_id: &str, tile_id: &str) -> Vec<String> {
    if split_id == "flank_team" {
        rts_string_vec(["10,4", "11,4", "12,4", "12,3"])
    } else {
        let tile = rts_parse_tile_id(tile_id).unwrap_or((10, 4));
        vec![
            format!("{},{}", tile.0, tile.1),
            format!("{},{}", tile.0 + 1, tile.1),
            format!("{},{}", tile.0 + 2, tile.1),
        ]
    }
}

pub fn rts_inner_core_tile_for_id(core_id: &str) -> (i32, i32) {
    match core_id {
        "signal_core" => (12, 3),
        "relay_core" => (12, 4),
        _ => (12, 3),
    }
}

pub fn rts_central_keep_route_tiles_for_id(target_id: &str, tile_id: &str) -> Vec<String> {
    if target_id == "central_keep" {
        rts_string_vec(["12,3", "12,4", "13,4", "13,3", "14,3"])
    } else {
        let tile = rts_parse_tile_id(tile_id).unwrap_or((13, 3));
        vec![
            format!("{},{}", tile.0.saturating_sub(1), tile.1),
            format!("{},{}", tile.0, tile.1),
            format!("{},{}", tile.0 + 1, tile.1),
        ]
    }
}

pub fn rts_central_keep_tile_for_id(target_id: &str) -> (i32, i32) {
    match target_id {
        "central_keep" => (13, 3),
        "mirror_ward" => (13, 3),
        _ => (13, 3),
    }
}

pub fn rts_boss_guard_units_for_id(guard_id: &str) -> Vec<String> {
    if guard_id == "warden_line" {
        rts_string_vec(["keep_warden_alpha", "keep_warden_beta", "ward_sentinel"])
    } else {
        vec![format!("{guard_id}_warden")]
    }
}

pub fn rts_player_siege_line_tiles_for_id(line_id: &str, tile_id: &str) -> Vec<String> {
    if line_id == "final_line" {
        rts_string_vec(["11,4", "12,4", "13,4", "12,3"])
    } else {
        let tile = rts_parse_tile_id(tile_id).unwrap_or((12, 4));
        vec![
            format!("{},{}", tile.0.saturating_sub(1), tile.1),
            format!("{},{}", tile.0, tile.1),
            format!("{},{}", tile.0 + 1, tile.1),
        ]
    }
}

pub fn rts_keep_breach_tiles_for_id(target_id: &str, tile_id: &str) -> Vec<String> {
    if target_id == "central_keep" {
        rts_string_vec(["13,3", "13,4", "14,3", "14,4"])
    } else {
        let tile = rts_parse_tile_id(tile_id).unwrap_or((13, 3));
        vec![
            format!("{},{}", tile.0, tile.1),
            format!("{},{}", tile.0 + 1, tile.1),
            format!("{},{}", tile.0, tile.1 + 1),
        ]
    }
}

pub fn rts_guardian_counter_units_for_id(counter_id: &str) -> Vec<String> {
    if counter_id == "high_warden" {
        rts_string_vec(["high_warden", "ward_lancer", "last_mirror_guard"])
    } else {
        vec![format!("{counter_id}_counter_guard")]
    }
}

pub fn rts_keep_claim_tiles_for_id(target_id: &str, tile_id: &str) -> Vec<String> {
    if target_id == "central_keep" {
        rts_string_vec(["12,3", "13,3", "14,3", "13,4"])
    } else {
        let tile = rts_parse_tile_id(tile_id).unwrap_or((13, 3));
        vec![
            format!("{},{}", tile.0.saturating_sub(1), tile.1),
            format!("{},{}", tile.0, tile.1),
            format!("{},{}", tile.0 + 1, tile.1),
        ]
    }
}

pub fn rts_restored_zones_for_id(zone_id: &str) -> Vec<String> {
    if zone_id == "mirror_city" {
        rts_string_vec(["central_keep", "signal_core", "inner_lane", "forest_relay"])
    } else {
        vec![zone_id.to_string()]
    }
}

pub fn rts_rebuild_structures_for_id(structure_id: &str) -> Vec<String> {
    if structure_id == "signal_core" {
        rts_string_vec(["signal_core", "inner_latch", "mirror_ward"])
    } else {
        vec![structure_id.to_string()]
    }
}

pub fn rts_garrison_units_for_id(garrison_id: &str) -> Vec<String> {
    if garrison_id == "central_keep" {
        rts_string_vec(["mirror_guard_alpha", "signal_lancer", "field_engineer"])
    } else {
        vec![format!("{garrison_id}_garrison")]
    }
}

pub fn rts_open_world_route_tiles_for_id(route_id: &str) -> Vec<String> {
    match route_id {
        "after_action" | "league-coliseum" => {
            rts_string_vec(["13,3", "12,3", "11,3", "10,2", "9,2"])
        }
        _ => rts_string_vec(["13,3", "12,3", "11,3"]),
    }
}

pub fn rts_open_world_panels_for_room(room_id: &str) -> Vec<String> {
    if room_id == "league-coliseum" {
        rts_string_vec([
            "room_panel:league-coliseum",
            "task_panel:task-fixture-first-route",
            "combat_panel:league-coliseum",
            "save_panel:post_rts_restore",
        ])
    } else {
        vec![format!("room_panel:{room_id}")]
    }
}

pub fn rts_siege_unit_tile_for_id(unit_id: &str, index: usize) -> (i32, i32) {
    match unit_id {
        "stonebreak_cart" => (9, 3),
        _ => (9 + (index as i32 % 2), 3),
    }
}

pub fn rts_harvest_tile_for_node(node_id: &str) -> (i32, i32) {
    match node_id {
        "gold_vein" => (3, 3),
        "lumber_copse" => (8, 3),
        "forest_relay_gold" => (10, 2),
        _ => (4, 4),
    }
}

pub fn rts_dropoff_tile_for_structure(structure_id: &str) -> (i32, i32) {
    match structure_id {
        "town_hall" => (5, 5),
        "lumber_mill" => (7, 5),
        "relay_outpost" => (9, 2),
        _ => (5, 5),
    }
}

pub fn rts_build_site_tiles(tile_id: &str) -> Vec<String> {
    match tile_id {
        "7,4" => rts_string_vec(["7,4", "7,5", "8,4"]),
        "8,4" => rts_string_vec(["8,4", "8,5", "9,4"]),
        _ => vec![tile_id.to_string()],
    }
}

pub fn rts_structure_tile_for_id(structure_id: &str) -> (i32, i32) {
    match structure_id {
        "watch_tower" => (7, 4),
        "scout_tower" => (8, 4),
        "town_hall" => (5, 5),
        "training_hall" => (4, 3),
        "signal_spire" => (6, 3),
        _ => (7, 4),
    }
}

pub fn rts_unlock_unit_tile_for_id(unit_id: &str) -> (i32, i32) {
    match unit_id {
        "relay_guard" => (7, 5),
        "wayfinder" => (4, 5),
        _ => (6, 5),
    }
}

pub fn rts_queue_gold_cost(queue_id: &str) -> u64 {
    let queue_id = queue_id.strip_prefix("queue:").unwrap_or(queue_id);
    let item_id = queue_id
        .split_once('@')
        .map(|(item_id, _)| item_id)
        .unwrap_or(queue_id);
    match item_id {
        "train:worker" => 80,
        "train:guard" => 140,
        "train:scout" => 110,
        "build:watch_tower" => 210,
        "build:training_hall" => 260,
        "build:signal_spire" => 320,
        "build:power_node" => 160,
        "build:refinery" => 240,
        "build:command_post" => 300,
        "build:radar_spire" => 220,
        "build:wall" => 60,
        "build:relay" | "build:scout_tower" => 180,
        "upgrade:signal_blade" | "upgrade:training_hall" => 210,
        "harvest:gold_vein" | "harvest:lumber_copse" => 0,
        _ if item_id.starts_with("complete:") => 0,
        _ if item_id.starts_with("cancel:") => 0,
        _ if item_id.starts_with("repair:") => 45,
        _ => 120,
    }
}

pub fn rts_queue_cost_label(queue_id: &str) -> String {
    let cost = rts_queue_gold_cost(queue_id);
    if cost == 0 {
        "-".to_string()
    } else {
        cost.to_string()
    }
}

pub fn rts_log_gold_amount(entry: &str) -> u64 {
    entry
        .split(':')
        .filter_map(|part| part.trim().strip_suffix('g'))
        .filter_map(|amount| {
            amount
                .trim_start_matches(|value| value == '+' || value == '-')
                .parse::<u64>()
                .ok()
        })
        .sum()
}

pub fn rts_resource_gold_commitment(resource_spend_log: &[String]) -> u64 {
    resource_spend_log
        .iter()
        .map(|entry| rts_log_gold_amount(entry))
        .sum()
}

pub fn rts_available_gold(coins: u64, resource_spend_log: &[String]) -> u64 {
    let gross_gold = 620_u64.saturating_add(coins);
    let commitment = rts_resource_gold_commitment(resource_spend_log);
    gross_gold.saturating_sub(commitment.min(gross_gold.saturating_sub(40)))
}

pub fn rts_queue_is_affordable(coins: u64, resource_spend_log: &[String], queue_id: &str) -> bool {
    rts_queue_gold_cost(queue_id) <= rts_available_gold(coins, resource_spend_log)
}

pub fn rts_queue_requires_affordability_check(queue_id: &str) -> bool {
    let queue_id = queue_id.strip_prefix("queue:").unwrap_or(queue_id).trim();
    queue_id.starts_with("build:")
        || queue_id.starts_with("train:")
        || queue_id.starts_with("upgrade:")
        || queue_id.starts_with("research:")
        || queue_id.starts_with("repair:")
}

pub fn rts_command_slot_id_for_index(
    runtime_ability_ids: &[String],
    chrome_slot_ids: Option<&[String]>,
    fallback_id: &str,
    index: usize,
) -> String {
    chrome_slot_ids
        .and_then(|slot_ids| slot_ids.get(index % slot_ids.len().max(1)))
        .cloned()
        .or_else(|| {
            runtime_ability_ids
                .get(index % runtime_ability_ids.len().max(1))
                .cloned()
        })
        .unwrap_or_else(|| fallback_id.to_string())
}

pub fn rts_build_palette_queue_id(index: usize) -> String {
    [
        "build:power_node@5,3",
        "build:training_hall@4,3",
        "build:refinery@6,4",
        "build:watch_tower@7,4",
        "build:command_post@5,2",
        "build:radar_spire@6,2",
        "build:wall@8,4",
        "upgrade:signal_blade",
    ]
    .get(index)
    .copied()
    .unwrap_or("build:watch_tower@7,4")
    .to_string()
}

pub fn rts_build_palette_queue_id_for_slot(chrome_queue_id: Option<&str>, index: usize) -> String {
    chrome_queue_id
        .filter(|queue_id| !queue_id.trim().is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| rts_build_palette_queue_id(index))
}

pub fn rts_production_slot_queue_id(
    production_queue: &[String],
    build_queue: &[String],
    train_fallback_queue_id: &str,
    build_fallback_queue_id: &str,
    index: usize,
) -> String {
    production_queue
        .get(index)
        .or_else(|| build_queue.get(index.saturating_sub(2)))
        .cloned()
        .unwrap_or_else(|| {
            if index % 2 == 0 {
                train_fallback_queue_id.to_string()
            } else {
                build_fallback_queue_id.to_string()
            }
        })
}

pub fn rts_sidebar_cancel_queue_id(
    production_queue: &[String],
    build_queue: &[String],
    index: usize,
) -> Option<String> {
    if production_queue.get(index).is_some() {
        Some(format!("cancel:production:{index}"))
    } else if index >= 2 && build_queue.get(index - 2).is_some() {
        Some(format!("cancel:build:{}", index - 2))
    } else {
        None
    }
}

pub fn rts_palette_cancel_queue_id(
    build_queue: &[String],
    production_queue: &[String],
    active_blueprint_id: Option<&str>,
    queue_id: &str,
) -> Option<String> {
    if let Some(index) = build_queue.iter().position(|entry| entry == queue_id) {
        Some(format!("cancel:build:{index}"))
    } else if let Some(index) = production_queue.iter().position(|entry| entry == queue_id) {
        Some(format!("cancel:production:{index}"))
    } else if queue_id.starts_with("build:")
        && active_blueprint_id.is_some_and(|id| queue_id.contains(id))
    {
        Some("cancel:active_build".to_string())
    } else {
        None
    }
}

pub fn rts_sidebar_slot_status_label(
    production_queue: &[String],
    build_queue: &[String],
    queue_affordable: bool,
    index: usize,
    progress: u8,
) -> String {
    if production_queue.get(index).is_some() {
        format!("Q{} {} R", index + 1, progress.min(100))
    } else if index >= 2 && build_queue.get(index - 2).is_some() {
        format!("B{} {} R", index - 1, progress.min(100))
    } else if queue_affordable {
        if index >= 2 {
            "ADD BUILD".to_string()
        } else {
            "ADD UNIT".to_string()
        }
    } else {
        "LOCK".to_string()
    }
}

pub fn rts_queue_item_player_label(queue_id: &str) -> String {
    let item = queue_id
        .split_once('@')
        .map(|(item, _)| item)
        .unwrap_or(queue_id);
    let item = item
        .strip_prefix("train:")
        .or_else(|| item.strip_prefix("build:"))
        .or_else(|| item.strip_prefix("upgrade:"))
        .unwrap_or(item);
    let item = item.strip_prefix("trnm.").unwrap_or(item);
    match item {
        "guard" => "GUARD".to_string(),
        "worker" => "WORKER".to_string(),
        "relay_guard" => "RELAY".to_string(),
        "signal_blade" | "signal_beacon" => "SIGNAL".to_string(),
        "watch_tower" | "scout_tower" => "TOWER".to_string(),
        "training_hall" => "TRAINING".to_string(),
        "power_node" => "POWER".to_string(),
        "refinery" => "REFINE".to_string(),
        "command_post" => "COMMAND".to_string(),
        "radar_spire" => "RADAR".to_string(),
        "wall" => "WALL".to_string(),
        _ => rts_catalog_text_label(&item.replace(['_', '.', ':', '-'], " "), 10),
    }
}

pub fn rts_palette_state_label(
    active_blueprint_id: Option<&str>,
    build_queue: &[String],
    production_queue: &[String],
    queue_affordable: bool,
    queue_id: &str,
) -> String {
    if active_blueprint_id.is_some_and(|id| queue_id.contains(id)) {
        "ACT".to_string()
    } else if let Some(index) = build_queue.iter().position(|entry| entry == queue_id) {
        format!("B Q{}", index + 1)
    } else if let Some(index) = production_queue.iter().position(|entry| entry == queue_id) {
        format!("P Q{}", index + 1)
    } else if queue_affordable {
        "RDY".to_string()
    } else {
        "LOCK".to_string()
    }
}

pub fn rts_spawned_unit_id_from_queue(queue_id: &str, existing_count: usize) -> String {
    let item_id = queue_id
        .split_once('@')
        .map(|(item_id, _)| item_id)
        .unwrap_or(queue_id);
    let unit_kind = item_id
        .strip_prefix("train:")
        .or_else(|| item_id.strip_prefix("upgrade:"))
        .unwrap_or("unit");
    format!("{}_{}", unit_kind.replace(':', "_"), existing_count + 1)
}

pub fn rts_structure_id_from_queue(queue_id: &str) -> String {
    queue_id
        .strip_prefix("build:")
        .or_else(|| queue_id.strip_prefix("complete:"))
        .unwrap_or(queue_id)
        .split_once('@')
        .map(|(structure_id, _)| structure_id)
        .unwrap_or("watch_tower")
        .to_string()
}

pub fn rts_sidebar_queue_summary(
    production_queue: &[String],
    build_queue: &[String],
    training_progress_percent: u8,
    build_progress_percent: u8,
) -> String {
    let production = production_queue.first().map(|queue| {
        format!(
            "{} {}%",
            rts_queue_item_player_label(queue),
            training_progress_percent.min(100)
        )
    });
    let build = build_queue.first().map(|queue| {
        format!(
            "{} {}%",
            rts_queue_item_player_label(queue),
            build_progress_percent.min(100)
        )
    });
    let summary = match (production, build) {
        (Some(production), Some(build)) => format!("{production} {build}"),
        (Some(production), None) => format!("TRAIN {production}"),
        (None, Some(build)) => format!("BUILD {build}"),
        (None, None) => "READY".to_string(),
    };
    rts_catalog_text_label(&summary, 20)
}

pub fn rts_build_parts(queue_id: &str) -> (String, String) {
    let payload = queue_id.strip_prefix("build:").unwrap_or(queue_id);
    if let Some((structure_id, tile_id)) = payload.split_once('@') {
        (structure_id.to_string(), tile_id.to_string())
    } else {
        (payload.to_string(), "7,4".to_string())
    }
}

pub fn rts_structure_parts(
    queue_id: &str,
    prefix: &str,
    fallback_tile_id: &str,
) -> (String, String) {
    let payload = queue_id.strip_prefix(prefix).unwrap_or(queue_id);
    if let Some((structure_id, tile_id)) = payload.split_once('@') {
        (structure_id.to_string(), tile_id.to_string())
    } else {
        (payload.to_string(), fallback_tile_id.to_string())
    }
}

pub fn rts_tech_parts(queue_id: &str, prefix: &str, fallback_source_id: &str) -> (String, String) {
    let payload = queue_id.strip_prefix(prefix).unwrap_or(queue_id);
    if let Some((tech_id, source_id)) = payload.split_once('@') {
        (tech_id.to_string(), source_id.to_string())
    } else {
        (payload.to_string(), fallback_source_id.to_string())
    }
}

pub fn rts_queue_uses_production_lane(queue_id: &str) -> bool {
    let queue_id = queue_id.strip_prefix("queue:").unwrap_or(queue_id);
    !queue_id.starts_with("build:")
        && !queue_id.starts_with("cancel:")
        && !queue_id.starts_with("complete:")
        && !queue_id.starts_with("harvest:")
        && !queue_id.starts_with("repair:")
}

pub fn rts_queue_feedback_chip(queue_id: &str) -> String {
    let queue_id = queue_id.strip_prefix("queue:").unwrap_or(queue_id);
    if let Some(unit_id) = queue_id.strip_prefix("train:") {
        format!("feedback:train_queued:{unit_id}")
    } else if queue_id.starts_with("build:") {
        let (structure_id, tile_id) = rts_build_parts(queue_id);
        format!("feedback:build_placed:{structure_id}@{tile_id}")
    } else if let Some(node_id) = queue_id.strip_prefix("harvest:") {
        format!("feedback:harvest_assigned:{node_id}")
    } else if queue_id.starts_with("upgrade:") {
        let (upgrade_id, source_id) = rts_tech_parts(queue_id, "upgrade:", "training_hall");
        format!("feedback:upgrade_queued:{upgrade_id}@{source_id}")
    } else if queue_id.starts_with("research:") {
        let (tech_id, source_id) = rts_tech_parts(queue_id, "research:", "town_hall");
        format!("feedback:research_queued:{tech_id}@{source_id}")
    } else {
        format!("feedback:queue_accepted:{queue_id}")
    }
}

pub fn rts_rejection_feedback_chip(action_label: &str, reason: &str) -> String {
    let action_kind = action_label
        .strip_prefix("RTS:")
        .and_then(|label| label.split(':').next())
        .filter(|label| !label.trim().is_empty())
        .unwrap_or("action")
        .to_ascii_lowercase();
    format!("feedback:blocked:{action_kind}:{reason}")
}

pub fn rts_input_source_player_label(input_source: &str, action_label: &str) -> &'static str {
    let normalized = input_source.to_ascii_lowercase();
    if normalized.contains("mouse_sidebar") {
        "SIDEBAR"
    } else if normalized.contains("mouse_command_bar") {
        "COMMAND BAR"
    } else if normalized.contains("mouse_minimap") {
        "MINIMAP"
    } else if normalized.contains("mouse_bottom_panel") {
        "BOTTOM PANEL"
    } else if normalized.contains("mouse_viewport") {
        "MAP"
    } else if normalized.contains("mouse_drag") {
        "DRAG"
    } else if normalized.contains("hotkey") {
        "HOTKEY"
    } else if normalized.contains("keyboard") {
        "KEYBOARD"
    } else if action_label.starts_with("RTS:QUEUE") {
        "SIDEBAR"
    } else if action_label.starts_with("RTS:MOVE") || action_label.starts_with("RTS:ATTACK") {
        "MAP"
    } else {
        "COMMAND"
    }
}

pub fn rts_command_stamp_for_selection(
    input_source: &str,
    group_id: &str,
    selected_unit_count: usize,
) -> RtsCommandStamp {
    let source = rts_input_source_player_label(input_source, "RTS:SELECT");
    if let Some((clear_kind, unit_id, tile_id)) = rts_selection_clear_parts(group_id) {
        let select_label = if unit_id.is_some() {
            format!("SELECTION CLEARED {}", clear_kind.to_ascii_uppercase())
        } else {
            "SELECTION CLEARED".to_string()
        };
        return RtsCommandStamp {
            input_source: input_source.to_string(),
            kind: "select-clear".to_string(),
            tile_id: Some(tile_id),
            target_id: unit_id,
            player_label: format!("{source} {select_label}"),
        };
    }
    let selected_count = selected_unit_count.max(1);
    let unit_word = if selected_count == 1 { "UNIT" } else { "UNITS" };
    let (kind, select_label, target_id) =
        if let Some(slot) = rts_control_group_hotkey_slot(group_id, "assign:") {
            (
                "control-group",
                format!("GROUP {slot} ASSIGNED"),
                Some(slot),
            )
        } else if let Some(slot) = rts_control_group_hotkey_slot(group_id, "append:") {
            (
                "control-group",
                format!("GROUP {slot} APPENDED"),
                Some(slot),
            )
        } else if let Some(slot) = rts_control_group_hotkey_slot(group_id, "recall:") {
            (
                "control-group",
                format!("GROUP {slot} RECALLED"),
                Some(slot),
            )
        } else if let Some(slot) = rts_control_group_hotkey_slot(group_id, "recall_add:") {
            ("control-group", format!("GROUP {slot} ADDED"), Some(slot))
        } else if let Some(slot) = rts_control_group_hotkey_slot(group_id, "camera:") {
            (
                "control-group-camera",
                format!("GROUP {slot} CAMERA SNAP"),
                Some(slot),
            )
        } else if group_id.starts_with("shift:unit:") {
            (
                "select",
                "SHIFT SELECT".to_string(),
                Some(group_id.to_string()),
            )
        } else if group_id.starts_with("double:unit:") {
            (
                "select",
                "DOUBLE SELECT".to_string(),
                Some(group_id.to_string()),
            )
        } else {
            ("select", "SELECT".to_string(), Some(group_id.to_string()))
        };
    let player_label = if group_id.starts_with("camera:") {
        format!("{source} {select_label}")
    } else if kind == "select" {
        format!("{source} {select_label} SENT {selected_count} {unit_word}")
    } else {
        format!("{source} {select_label} {selected_count} {unit_word}")
    };
    RtsCommandStamp {
        input_source: input_source.to_string(),
        kind: kind.to_string(),
        tile_id: None,
        target_id,
        player_label,
    }
}

pub fn rts_command_stamp_for_queue(input_source: &str, queue_id: &str) -> RtsCommandStamp {
    let source = rts_input_source_player_label(input_source, "RTS:QUEUE");
    let (kind, target_id, tile_id, item_label) = if queue_id.starts_with("build:") {
        let (structure_id, tile_id) = rts_build_parts(queue_id);
        (
            "build",
            structure_id.clone(),
            Some(tile_id),
            rts_catalog_text_label(&structure_id, 20),
        )
    } else if let Some(unit_id) = queue_id.strip_prefix("train:") {
        (
            "train",
            unit_id.to_string(),
            None,
            rts_catalog_text_label(unit_id, 20),
        )
    } else if let Some(node_id) = queue_id.strip_prefix("harvest:") {
        (
            "harvest",
            node_id.to_string(),
            Some(rts_runtime_tile_id(rts_harvest_tile_for_node(node_id))),
            rts_catalog_text_label(node_id, 20),
        )
    } else if queue_id.starts_with("upgrade:") {
        let (upgrade_id, source_id) = rts_tech_parts(queue_id, "upgrade:", "training_hall");
        (
            "upgrade",
            upgrade_id.clone(),
            Some(rts_runtime_tile_id(rts_structure_tile_for_id(&source_id))),
            rts_catalog_text_label(&upgrade_id, 20),
        )
    } else {
        (
            "queue",
            queue_id.to_string(),
            None,
            rts_catalog_text_label(queue_id, 20),
        )
    };
    let tile_suffix = tile_id
        .as_deref()
        .map(|tile| format!(" {tile}"))
        .unwrap_or_default();
    RtsCommandStamp {
        input_source: input_source.to_string(),
        kind: kind.to_string(),
        tile_id,
        target_id: Some(target_id),
        player_label: format!(
            "{source} {} SENT {item_label}{tile_suffix}",
            kind.to_ascii_uppercase()
        ),
    }
}

pub fn rts_command_stamp_for_move(input_source: &str, command_id: &str) -> Option<RtsCommandStamp> {
    let source = rts_input_source_player_label(input_source, "RTS:MOVE");
    let (tile_id, formation) = rts_move_command_parts(command_id);
    rts_parse_tile_id(tile_id)?;
    let follow_target_id = rts_move_follow_target(formation);
    let formation_kind = rts_move_formation_kind(formation);
    let kind = if command_id.starts_with("minimap:") || formation_kind == "rally" {
        "rally"
    } else if formation_kind == "shift_waypoint" {
        "waypoint"
    } else if formation_kind == "attack_move" {
        "attack-move"
    } else if formation_kind == "patrol" {
        "patrol"
    } else if formation_kind == "hold" {
        "hold"
    } else if formation_kind == "stop" {
        "stop"
    } else if formation_kind == "follow" {
        "follow"
    } else {
        "move"
    };
    let target_id = follow_target_id.map(ToOwned::to_owned);
    let player_label = if let Some(target_id) = follow_target_id {
        format!(
            "{source} FOLLOW SENT {}",
            rts_catalog_text_label(target_id, 22)
        )
    } else {
        format!(
            "{source} {} SENT {tile_id}",
            kind.replace('-', " ").to_ascii_uppercase()
        )
    };
    Some(RtsCommandStamp {
        input_source: input_source.to_string(),
        kind: kind.to_string(),
        tile_id: Some(tile_id.to_string()),
        target_id,
        player_label,
    })
}

pub fn rts_command_stamp_for_attack(input_source: &str, target_id: &str) -> RtsCommandStamp {
    let source = rts_input_source_player_label(input_source, "RTS:ATTACK");
    RtsCommandStamp {
        input_source: input_source.to_string(),
        kind: "attack".to_string(),
        tile_id: Some(rts_runtime_tile_id(rts_target_tile_for_id(target_id, 0))),
        target_id: Some(target_id.to_string()),
        player_label: format!(
            "{source} ATTACK SENT {}",
            rts_catalog_text_label(target_id, 22)
        ),
    }
}

pub fn rts_command_stamp_for_ability(
    input_source: &str,
    ability_id: &str,
    attack_target_id: Option<&str>,
) -> RtsCommandStamp {
    let source = rts_input_source_player_label(input_source, "RTS:ABILITY");
    let target_id = attack_target_id.map(ToOwned::to_owned);
    let tile_id =
        attack_target_id.map(|target_id| rts_runtime_tile_id(rts_target_tile_for_id(target_id, 0)));
    RtsCommandStamp {
        input_source: input_source.to_string(),
        kind: "ability".to_string(),
        tile_id,
        target_id,
        player_label: format!(
            "{source} ABILITY SENT {}",
            rts_catalog_text_label(ability_id, 22)
        ),
    }
}

pub fn rts_order_queue_replay_action(
    order: &str,
    fallback_ability_id: &str,
) -> RtsOrderQueueReplayAction {
    let order = order.strip_prefix("queue:").unwrap_or(order).trim();
    let fallback_ability_id = fallback_ability_id.trim();
    let fallback_ability_id = if fallback_ability_id.is_empty() {
        "focus_fire"
    } else {
        fallback_ability_id
    };
    if let Some(target_id) = order.strip_prefix("attack:") {
        RtsOrderQueueReplayAction {
            kind: "attack".to_string(),
            payload: target_id.to_string(),
        }
    } else if let Some(tile_id) = order.strip_prefix("move:") {
        RtsOrderQueueReplayAction {
            kind: "move".to_string(),
            payload: format!("{tile_id}:line"),
        }
    } else if order.starts_with("minimap:") {
        RtsOrderQueueReplayAction {
            kind: "move".to_string(),
            payload: order.to_string(),
        }
    } else if let Some(ability_id) = order.strip_prefix("ability:") {
        RtsOrderQueueReplayAction {
            kind: "ability".to_string(),
            payload: ability_id.to_string(),
        }
    } else if order.starts_with("build:")
        || order.starts_with("train:")
        || order.starts_with("upgrade:")
        || order.starts_with("harvest:")
        || order.starts_with("complete:")
    {
        RtsOrderQueueReplayAction {
            kind: "queue".to_string(),
            payload: order.to_string(),
        }
    } else if let Some(group_id) = order.strip_prefix("select_group_") {
        RtsOrderQueueReplayAction {
            kind: "select-control-group".to_string(),
            payload: group_id.to_string(),
        }
    } else {
        RtsOrderQueueReplayAction {
            kind: "ability".to_string(),
            payload: fallback_ability_id.to_string(),
        }
    }
}

pub fn rts_hover_target_preview_kind(affordance: &str) -> Option<&'static str> {
    if affordance.contains("attack") {
        Some("attack")
    } else if affordance.contains("harvest") {
        Some("harvest")
    } else if affordance.contains("follow") {
        Some("follow")
    } else if affordance.contains("move") {
        Some("move")
    } else {
        None
    }
}

pub fn rts_cursor_kind_for_hover_preview(
    accepted: bool,
    affordance: &str,
    action_label: &str,
) -> &'static str {
    if !accepted {
        return "blocked";
    }
    if affordance.contains("attack") {
        "attack"
    } else if affordance.contains("harvest") {
        "harvest"
    } else if affordance.contains("follow") {
        "follow"
    } else if affordance.contains("build") || affordance.contains("queue") {
        "build"
    } else if affordance.contains("command_button") {
        "ability"
    } else if affordance.contains("rally") || affordance.contains("minimap") {
        "rally"
    } else if affordance.contains("selection") || action_label.starts_with("RTS:SELECT:") {
        "select"
    } else {
        "move"
    }
}

pub fn rts_cursor_label_for_hover_preview(
    input_source: &str,
    action_label: &str,
    accepted: bool,
    cursor_kind: &str,
) -> String {
    let source = rts_input_source_player_label(input_source, action_label);
    let state = if accepted { "READY" } else { "LOCK" };
    format!(
        "{source} CURSOR {} {state}",
        cursor_kind.replace('-', " ").to_ascii_uppercase()
    )
}

pub fn rts_hover_player_label(
    input_source: &str,
    action_label: &str,
    tile_id: Option<&str>,
    queue_id: Option<&str>,
    affordance: &str,
    accepted: bool,
    reason: &str,
) -> String {
    let source = rts_input_source_player_label(input_source, action_label);
    if !accepted && action_label.starts_with("RTS:") {
        let chip = rts_rejection_feedback_chip(action_label, reason);
        return format!("{source} {}", rts_blocked_feedback_player_label(&chip));
    }
    if let Some(queue_id) = queue_id {
        let queue_label = rts_catalog_text_label(
            &queue_id
                .replace("build:", "")
                .replace("train:", "")
                .replace("upgrade:", "")
                .replace("research:", "")
                .replace("harvest:", "")
                .replace('@', " "),
            18,
        );
        let gold = rts_queue_gold_cost(queue_id);
        return if gold > 0 {
            format!("{source} QUEUE READY {queue_label} {gold}G")
        } else if affordance == "viewport_harvest" && queue_id.starts_with("harvest:") {
            format!("{source} HARVEST READY {queue_label}")
        } else {
            format!("{source} QUEUE READY {queue_label}")
        };
    }
    if action_label.starts_with("RTS:MOVE:") {
        let tile = tile_id.unwrap_or("-");
        if let Some(target_id) = action_label
            .strip_prefix("RTS:MOVE:")
            .map(rts_move_command_parts)
            .and_then(|(_, formation)| rts_move_follow_target(formation))
        {
            return format!(
                "{source} FOLLOW READY {}",
                rts_catalog_text_label(&target_id.replace('_', " "), 18)
            );
        }
        return if affordance == "minimap_rally" {
            format!("{source} RALLY READY {tile}")
        } else {
            format!("{source} MOVE READY {tile}")
        };
    }
    if let Some(target_id) = action_label.strip_prefix("RTS:ATTACK:") {
        return format!(
            "{source} ATTACK READY {}",
            rts_catalog_text_label(&target_id.replace('_', " "), 22)
        );
    }
    if let Some(ability_id) = action_label.strip_prefix("RTS:ABILITY:") {
        return format!(
            "{source} ABILITY READY {}",
            rts_catalog_text_label(&ability_id.replace('_', " "), 18)
        );
    }
    if let Some(group_id) = action_label.strip_prefix("RTS:SELECT:") {
        return format!(
            "{source} SELECT READY {}",
            rts_catalog_text_label(group_id, 18)
        );
    }
    format!(
        "{source} READY {}",
        rts_catalog_text_label(&action_label.replace("RTS:", "").replace(':', " "), 24)
    )
}

pub fn rts_blocked_feedback_toast(input_source: &str, action_label: &str, reason: &str) -> String {
    let chip = rts_rejection_feedback_chip(action_label, reason);
    format!(
        "Input blocked: {} {}",
        rts_input_source_player_label(input_source, action_label),
        rts_blocked_feedback_player_label(&chip)
    )
}

pub fn rts_should_emit_rejection_feedback_chip(input_source: &str) -> bool {
    !input_source.contains("bot_executor")
}

pub fn rts_executable_command_queue_snapshot(queue: &[String]) -> Vec<String> {
    queue
        .iter()
        .filter(|entry| !entry.starts_with("feedback:blocked:"))
        .cloned()
        .collect()
}

pub fn rts_blocked_feedback_chip_visible(command_queue: &[String]) -> bool {
    command_queue
        .iter()
        .any(|entry| entry.starts_with("feedback:blocked:"))
}

pub fn rts_command_surface_stage(
    combat_turn: u8,
    combat_events: &[String],
    command_queue: &[String],
) -> Option<&'static str> {
    for event in combat_events.iter().rev() {
        if event.contains("surface:target_queue") {
            return Some("target_queue");
        }
        if event.contains("surface:cooldown_disabled") {
            return Some("cooldown_disabled");
        }
        if event.contains("surface:command_grid") {
            return Some("command_grid");
        }
        if event.contains("surface:selection_state") {
            return Some("selection_state");
        }
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("surface:"))
    {
        return None;
    }
    Some(match combat_turn % 4 {
        0 => "selection_state",
        1 => "command_grid",
        2 => "cooldown_disabled",
        _ => "target_queue",
    })
}

pub fn rts_command_feedback_strip_stage(
    combat_turn: u8,
    combat_events: &[String],
    command_queue: &[String],
) -> Option<&'static str> {
    for event in combat_events.iter().rev().chain(command_queue.iter().rev()) {
        if event.contains("control_group_command_feedback_strip:group_28_filtered") {
            return Some("group_28_filtered");
        }
        if event.contains("control_group_command_feedback_strip:group_28_formation") {
            return Some("group_28_formation");
        }
        if event.contains("control_group_command_feedback_strip:group_27_override") {
            return Some("group_27_override");
        }
        if event.contains("control_group_command_feedback_strip:group_26_queued") {
            return Some("group_26_queued");
        }
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("control_group_command_feedback_strip:"))
    {
        return None;
    }
    Some(match combat_turn % 4 {
        0 => "group_26_queued",
        1 => "group_27_override",
        2 => "group_28_formation",
        _ => "group_28_filtered",
    })
}

fn rts_feedback_lifecycle_texts<'a>(
    group_command_state: &'a str,
    combat_events: &'a [String],
    command_queue: &'a [String],
) -> impl Iterator<Item = &'a str> {
    std::iter::once(group_command_state)
        .chain(combat_events.iter().rev().map(String::as_str))
        .chain(command_queue.iter().rev().map(String::as_str))
}

pub fn rts_command_feedback_lifecycle_stage(
    group_command_state: &str,
    combat_events: &[String],
    command_queue: &[String],
) -> Option<&'static str> {
    for text in rts_feedback_lifecycle_texts(group_command_state, combat_events, command_queue) {
        if text.contains("control_group_command_feedback_lifecycle:cleared")
            || text.contains("command_feedback_lifecycle:cleared")
        {
            return Some("cleared");
        }
        if text.contains("control_group_command_feedback_lifecycle:dimmed")
            || text.contains("command_feedback_lifecycle:dimmed")
        {
            return Some("dimmed");
        }
        if text.contains("control_group_command_feedback_lifecycle:fresh")
            || text.contains("command_feedback_lifecycle:fresh")
        {
            return Some("fresh");
        }
    }
    None
}

pub fn rts_command_history_visible(
    group_command_state: &str,
    combat_events: &[String],
    command_queue: &[String],
) -> bool {
    rts_feedback_lifecycle_texts(group_command_state, combat_events, command_queue).any(|text| {
        text.contains("control_group_command_history:")
            || text.contains("command_feedback_history:")
    })
}

pub fn rts_command_history_prune_visible(
    group_command_state: &str,
    combat_events: &[String],
    command_queue: &[String],
) -> bool {
    rts_feedback_lifecycle_texts(group_command_state, combat_events, command_queue).any(|text| {
        text.contains("control_group_command_history_prune:")
            || text.contains("command_history_prune:")
            || text.contains("history_row_pruned:")
    })
}

pub fn rts_command_execution_feedback_kind(
    unit_response_state: &str,
    group_command_state: &str,
    economy_state: &str,
    command_destination_tile_present: bool,
    minimap_command_kind: &str,
    has_path_tiles: bool,
    has_group_route_tiles: bool,
    command_queue: &[String],
) -> Option<&'static str> {
    if let Some(recent_feedback_chip) = command_queue
        .iter()
        .rev()
        .find(|entry| entry.starts_with("feedback:"))
    {
        if recent_feedback_chip.starts_with("feedback:harvest_assigned:") {
            return Some("harvest");
        }
        if recent_feedback_chip.starts_with("feedback:follow@") {
            return Some("follow");
        }
        if recent_feedback_chip.starts_with("feedback:attack_move@") {
            return Some("attack");
        }
        if recent_feedback_chip.starts_with("feedback:line@")
            || recent_feedback_chip.starts_with("feedback:diamond@")
            || recent_feedback_chip.starts_with("feedback:waypoint_queued@")
            || recent_feedback_chip.starts_with("feedback:hold_position@")
            || recent_feedback_chip.starts_with("feedback:patrol_route@")
            || recent_feedback_chip.starts_with("feedback:rally_confirmed@")
            || recent_feedback_chip.starts_with("feedback:stop_hold@")
        {
            return Some("move");
        }
    }
    if unit_response_state.starts_with("following:")
        || group_command_state.starts_with("follow:")
        || minimap_command_kind == "follow"
    {
        Some("follow")
    } else if unit_response_state.starts_with("engaged:")
        || unit_response_state.starts_with("attack_move_advancing:")
        || group_command_state.starts_with("attack_move:")
    {
        Some("attack")
    } else if command_destination_tile_present
        && minimap_command_kind != "harvest"
        && !economy_state.starts_with("harvesting:")
        && (has_path_tiles || has_group_route_tiles)
    {
        Some("move")
    } else if minimap_command_kind == "harvest"
        || economy_state.starts_with("harvesting:")
        || command_queue
            .iter()
            .rev()
            .any(|entry| entry.starts_with("harvest:"))
    {
        Some("harvest")
    } else {
        None
    }
}

pub fn rts_command_execution_target_label(
    kind: &str,
    attack_target_id: Option<&str>,
    unit_response_state: &str,
    group_command_state: &str,
    harvest_node_ids: &[String],
    command_queue: &[String],
    command_destination_tile: Option<&str>,
) -> String {
    match kind {
        "attack" => attack_target_id
            .map(str::to_string)
            .unwrap_or_else(|| "target".to_string()),
        "follow" => unit_response_state
            .strip_prefix("following:")
            .map(str::to_string)
            .or_else(|| {
                group_command_state
                    .strip_prefix("follow:")
                    .and_then(|target| target.split('@').next())
                    .map(str::to_string)
            })
            .unwrap_or_else(|| "unit".to_string()),
        "harvest" => harvest_node_ids
            .first()
            .cloned()
            .or_else(|| {
                command_queue.iter().rev().find_map(|entry| {
                    entry
                        .strip_prefix("harvest:")
                        .and_then(|value| value.split("->").next())
                        .filter(|value| !value.is_empty())
                        .map(str::to_string)
                })
            })
            .unwrap_or_else(|| "resource".to_string()),
        _ => command_destination_tile
            .map(str::to_string)
            .unwrap_or_else(|| "destination".to_string()),
    }
}

pub fn rts_command_execution_player_label(
    kind: &str,
    target_label: &str,
    dropoff_structure_id: Option<&str>,
) -> String {
    let target_label = target_label.replace('_', " ").to_ascii_uppercase();
    match kind {
        "attack" => format!("ATTACK FOCUS {target_label}"),
        "follow" => format!("FOLLOWING {target_label}"),
        "harvest" => {
            let dropoff = dropoff_structure_id
                .unwrap_or("dropoff")
                .replace('_', " ")
                .to_ascii_uppercase();
            format!("HARVEST {target_label} TO {dropoff}")
        }
        _ => format!("MOVE EXECUTING {target_label}"),
    }
}

pub fn rts_command_execution_target_tile(
    kind: &str,
    attack_target_id: Option<&str>,
    unit_response_state: &str,
    group_command_state: &str,
    harvest_node_ids: &[String],
    command_queue: &[String],
    command_destination_tile: Option<&str>,
) -> Option<(i32, i32)> {
    let destination_tile = command_destination_tile.and_then(rts_parse_tile_id);
    match kind {
        "attack" => attack_target_id
            .map(|target_id| rts_target_tile_for_id(target_id, 0))
            .or(destination_tile),
        "follow" => {
            let target_label = rts_command_execution_target_label(
                kind,
                attack_target_id,
                unit_response_state,
                group_command_state,
                harvest_node_ids,
                command_queue,
                command_destination_tile,
            );
            rts_selectable_unit_tile(&target_label).or(destination_tile)
        }
        "harvest" => harvest_node_ids
            .first()
            .map(|node_id| rts_harvest_tile_for_node(node_id))
            .or(destination_tile),
        _ => destination_tile,
    }
}

fn rts_recent_stage_from_events(
    markers: &[(&str, &'static str)],
    combat_events: &[String],
    command_queue: &[String],
) -> Option<&'static str> {
    for event in combat_events.iter().rev().chain(command_queue.iter().rev()) {
        if let Some((_, stage)) = markers.iter().find(|(marker, _)| event.contains(marker)) {
            return Some(*stage);
        }
    }
    None
}

pub fn rts_unit_status_portrait_stage(
    combat_turn: u8,
    combat_events: &[String],
    command_queue: &[String],
) -> Option<&'static str> {
    for event in combat_events.iter().rev() {
        if event.contains("unit_status_portrait:multi_select") {
            return Some("multi_select");
        }
        if event.contains("unit_status_portrait:structure") {
            return Some("structure");
        }
        if event.contains("unit_status_portrait:creep_target") {
            return Some("creep_target");
        }
        if event.contains("unit_status_portrait:commander") {
            return Some("commander");
        }
        if event.contains("unit_status_portrait:guard") {
            return Some("guard");
        }
        if event.contains("unit_status_portrait:worker") {
            return Some("worker");
        }
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("unit_status_portrait:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "worker",
        1 => "guard",
        2 => "commander",
        3 => "creep_target",
        4 => "structure",
        _ => "multi_select",
    })
}

pub fn rts_unit_status_portrait_unit_id(
    stage: &str,
    selected_unit_ids: &[String],
    commander_unit_id: Option<&str>,
    attack_target_id: Option<&str>,
    completed_structure_ids: &[String],
) -> String {
    match stage {
        "worker" => selected_unit_ids
            .iter()
            .find(|id| id.contains("worker"))
            .cloned()
            .unwrap_or_else(|| "square_worker_carry".to_string()),
        "guard" => selected_unit_ids
            .iter()
            .find(|id| id.contains("guard"))
            .cloned()
            .unwrap_or_else(|| "arena_guard_left".to_string()),
        "commander" => commander_unit_id
            .map(str::to_string)
            .unwrap_or_else(|| "mirror_captain".to_string()),
        "creep_target" => attack_target_id
            .map(str::to_string)
            .unwrap_or_else(|| "arena_creep_attack".to_string()),
        "structure" => completed_structure_ids
            .first()
            .cloned()
            .unwrap_or_else(|| "training_hall".to_string()),
        _ => selected_unit_ids
            .first()
            .cloned()
            .unwrap_or_else(|| "player".to_string()),
    }
}

pub fn rts_unit_status_health_percent(
    stage: &str,
    unit_health_percents: &[u8],
    structure_health_percents: &[u8],
    target_health_percent: u8,
) -> u8 {
    unit_health_percents.first().copied().unwrap_or_else(|| {
        if stage == "structure" {
            structure_health_percents.first().copied().unwrap_or(86)
        } else if stage == "creep_target" {
            target_health_percent.max(1)
        } else {
            88
        }
    })
}

pub fn rts_unit_status_energy_percent(ability_cooldown_percents: &[u8]) -> u8 {
    ability_cooldown_percents
        .first()
        .copied()
        .map(|cooldown| 100_u8.saturating_sub(cooldown))
        .unwrap_or(68)
}

pub fn rts_unit_status_role_badges(stage: &str) -> [&'static str; 3] {
    match stage {
        "worker" => ["HAR", "REP", "RET"],
        "guard" => ["ATK", "HLD", "DEF"],
        "commander" => ["AUR", "LVL", "CMD"],
        "creep_target" => ["THR", "ARM", "FOC"],
        "structure" => ["Q", "BLD", "UP"],
        _ => ["G1", "SEL", "ORD"],
    }
}

pub fn rts_selection_command_feedback_stage(
    combat_turn: u8,
    combat_events: &[String],
    command_queue: &[String],
) -> Option<&'static str> {
    if let Some(stage) = rts_recent_stage_from_events(
        &[
            ("selection_command_feedback:invalid_order", "invalid_order"),
            ("selection_command_feedback:attack_lock", "attack_lock"),
            ("selection_command_feedback:move_line", "move_line"),
            ("selection_command_feedback:rally_preview", "rally_preview"),
            (
                "selection_command_feedback:selection_confirm",
                "selection_confirm",
            ),
            ("selection_command_feedback:marquee_start", "marquee_start"),
        ],
        combat_events,
        command_queue,
    ) {
        return Some(stage);
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("selection_command_feedback:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "marquee_start",
        1 => "selection_confirm",
        2 => "rally_preview",
        3 => "move_line",
        4 => "attack_lock",
        _ => "invalid_order",
    })
}

pub fn rts_ability_tooltip_telegraph_stage(
    combat_turn: u8,
    combat_events: &[String],
    command_queue: &[String],
) -> Option<&'static str> {
    if let Some(stage) = rts_recent_stage_from_events(
        &[
            (
                "ability_tooltip_telegraph:resource_warning",
                "resource_warning",
            ),
            ("ability_tooltip_telegraph:queue_explain", "queue_explain"),
            ("ability_tooltip_telegraph:cooldown_sweep", "cooldown_sweep"),
            ("ability_tooltip_telegraph:cast_windup", "cast_windup"),
            ("ability_tooltip_telegraph:range_preview", "range_preview"),
            ("ability_tooltip_telegraph:hover_tooltip", "hover_tooltip"),
        ],
        combat_events,
        command_queue,
    ) {
        return Some(stage);
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("ability_tooltip_telegraph:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "hover_tooltip",
        1 => "range_preview",
        2 => "cast_windup",
        3 => "cooldown_sweep",
        4 => "queue_explain",
        _ => "resource_warning",
    })
}

pub fn rts_control_group_hotkey_feedback_stage(
    combat_turn: u8,
    combat_events: &[String],
    command_queue: &[String],
) -> Option<&'static str> {
    if let Some(stage) = rts_recent_stage_from_events(
        &[
            (
                "control_group_hotkey_feedback:ability_hotkey_ack",
                "ability_hotkey_ack",
            ),
            (
                "control_group_hotkey_feedback:production_hotkey",
                "production_hotkey",
            ),
            (
                "control_group_hotkey_feedback:idle_worker_ping",
                "idle_worker_ping",
            ),
            (
                "control_group_hotkey_feedback:double_tap_camera",
                "double_tap_camera",
            ),
            ("control_group_hotkey_feedback:recall_group", "recall_group"),
            ("control_group_hotkey_feedback:assign_group", "assign_group"),
        ],
        combat_events,
        command_queue,
    ) {
        return Some(stage);
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("control_group_hotkey_feedback:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "assign_group",
        1 => "recall_group",
        2 => "double_tap_camera",
        3 => "idle_worker_ping",
        4 => "production_hotkey",
        _ => "ability_hotkey_ack",
    })
}

pub fn rts_blocked_feedback_player_label(chip: &str) -> String {
    let blocked = chip.strip_prefix("feedback:blocked:").unwrap_or(chip);
    if let Some(queue_id) = blocked.strip_prefix("queue:rts_queue_unaffordable:") {
        return format!("QUEUE LOCK NEED {}G", rts_queue_gold_cost(queue_id));
    }
    if blocked == "queue:rts_queue_id_required" {
        return "QUEUE LOCK PICK ITEM".to_string();
    }
    if blocked == "select:rts_group_id_required" {
        return "SELECT LOCK GROUP ID".to_string();
    }
    if blocked == "attack:rts_attack_target_required" {
        return "ATTACK LOCK PICK TARGET".to_string();
    }
    if blocked == "ability:rts_attack_required_before_ability" {
        return "ABILITY LOCK NEED TARGET".to_string();
    }
    if blocked == "move:rts_group_selection_required" || blocked == "move:select_units" {
        return "MOVE LOCK SELECT UNITS".to_string();
    }
    if blocked.starts_with("move:rts_invalid_tile:") {
        return "MOVE LOCK INVALID TILE".to_string();
    }
    blocked
        .replace("rts_", "")
        .replace(':', " ")
        .replace('_', " ")
        .to_ascii_uppercase()
}

pub fn rts_scripted_demo_pauses_queue_tick(demo_id: &str) -> bool {
    matches!(
        demo_id,
        "queue_cancel_refund" | "queue_cancel_refund_sequence"
    )
}

pub fn rts_scripted_demo_stage_from_frame(demo_id: &str, frame_tick: u64) -> Option<usize> {
    match demo_id {
        "queue_cancel_refund_sequence" => Some(((frame_tick / 60) % 5) as usize),
        _ => None,
    }
}

pub fn rts_scripted_demo_stage_id(stage: usize) -> &'static str {
    match stage {
        0 => "drag_select_frontline",
        1 => "rally_path_minimap",
        2 => "watch_tower_footprint",
        3 => "cancel_refund",
        4 => "queued_worker_ready",
        _ => "unknown",
    }
}

pub fn rts_scripted_demo_stage_title(stage: usize) -> &'static str {
    match stage {
        0 => "DRAG SELECT",
        1 => "RALLY / MINIMAP",
        2 => "BUILD FOOTPRINT",
        3 => "CANCEL / REFUND",
        4 => "WORKER QUEUED",
        _ => "UNKNOWN",
    }
}

pub fn rts_command_queue_path_preview_stage(
    combat_events: &[String],
    command_queue: &[String],
    combat_turn: u8,
) -> Option<&'static str> {
    for event in combat_events.iter().rev().chain(command_queue.iter().rev()) {
        if event.contains("command_queue_path_preview:cancel_repath") {
            return Some("cancel_repath");
        }
        if event.contains("command_queue_path_preview:build_reservation") {
            return Some("build_reservation");
        }
        if event.contains("command_queue_path_preview:attack_focus") {
            return Some("attack_focus");
        }
        if event.contains("command_queue_path_preview:rally_chain") {
            return Some("rally_chain");
        }
        if event.contains("command_queue_path_preview:shift_waypoints") {
            return Some("shift_waypoints");
        }
        if event.contains("command_queue_path_preview:queue_stack") {
            return Some("queue_stack");
        }
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("command_queue_path_preview:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "queue_stack",
        1 => "shift_waypoints",
        2 => "rally_chain",
        3 => "attack_focus",
        4 => "build_reservation",
        _ => "cancel_repath",
    })
}

pub fn rts_command_queue_path_preview_input_source() -> &'static str {
    "classic_rts_command_queue_path_preview_input"
}

pub fn rts_command_queue_path_preview_renderer_path() -> &'static str {
    "classic_draw_scene+classic_draw_rts_command_queue_path_preview_overlay"
}

pub fn rts_command_queue_path_preview_preview_surface() -> &'static str {
    "queue_stack+shift_waypoints+rally_chain+attack_focus+build_reservation+cancel_repath"
}

pub fn rts_command_queue_path_preview_stage_fixtures() -> Vec<RtsCommandQueuePathPreviewStageFixture>
{
    [
        ("queue_stack", "select-control-group", "box:frontline"),
        ("shift_waypoints", "move", "8,4:line"),
        ("rally_chain", "move", "9,2:rally"),
        ("attack_focus", "attack", "arena_creep_attack"),
        ("build_reservation", "queue", "build:watch_tower@7,4"),
        ("cancel_repath", "queue", "cancel:build:0"),
    ]
    .into_iter()
    .map(
        |(stage, kind, payload)| RtsCommandQueuePathPreviewStageFixture {
            stage: stage.to_string(),
            action: RtsOrderQueueReplayAction {
                kind: kind.to_string(),
                payload: payload.to_string(),
            },
            history_entry: format!("command_queue_path_preview:{stage}"),
            input_source: rts_command_queue_path_preview_input_source().to_string(),
            renderer_path: rts_command_queue_path_preview_renderer_path().to_string(),
            preview_surface: rts_command_queue_path_preview_preview_surface().to_string(),
        },
    )
    .collect()
}

pub fn rts_command_queue_path_preview_stage_ids() -> Vec<String> {
    rts_command_queue_path_preview_stage_fixtures()
        .into_iter()
        .map(|fixture| fixture.stage)
        .collect()
}

pub fn rts_formation_move_preview_stage(
    combat_events: &[String],
    command_queue: &[String],
    combat_turn: u8,
) -> Option<&'static str> {
    for event in combat_events.iter().rev().chain(command_queue.iter().rev()) {
        if event.contains("formation_move_preview:commit_spacing") {
            return Some("commit_spacing");
        }
        if event.contains("formation_move_preview:split_avoidance") {
            return Some("split_avoidance");
        }
        if event.contains("formation_move_preview:collision_avoidance") {
            return Some("collision_avoidance");
        }
        if event.contains("formation_move_preview:line_reflow") {
            return Some("line_reflow");
        }
        if event.contains("formation_move_preview:wedge_spacing") {
            return Some("wedge_spacing");
        }
        if event.contains("formation_move_preview:destination_ghost") {
            return Some("destination_ghost");
        }
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("formation_move_preview:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "destination_ghost",
        1 => "wedge_spacing",
        2 => "line_reflow",
        3 => "collision_avoidance",
        4 => "split_avoidance",
        _ => "commit_spacing",
    })
}

pub fn rts_formation_move_preview_input_source() -> &'static str {
    "classic_rts_formation_move_preview_input"
}

pub fn rts_formation_move_preview_renderer_path() -> &'static str {
    "classic_draw_scene+classic_draw_rts_formation_move_preview_overlay"
}

pub fn rts_formation_move_preview_preview_surface() -> &'static str {
    "destination_ghost+wedge_spacing+line_reflow+collision_avoidance+split_avoidance+commit_spacing"
}

fn rts_formation_move_preview_fixture(
    stage: &str,
    kind: &str,
    payload: &str,
) -> RtsFormationMovePreviewStageFixture {
    RtsFormationMovePreviewStageFixture {
        stage: stage.to_string(),
        action: RtsOrderQueueReplayAction {
            kind: kind.to_string(),
            payload: payload.to_string(),
        },
        history_entry: format!("formation_move_preview:{stage}"),
        input_source: rts_formation_move_preview_input_source().to_string(),
        renderer_path: rts_formation_move_preview_renderer_path().to_string(),
        preview_surface: rts_formation_move_preview_preview_surface().to_string(),
        command_destination_tile: None,
        path_tile_ids: Vec::new(),
        blocked_tile_ids: Vec::new(),
        formation_slot_tile_ids: Vec::new(),
        disperse_tile_ids: Vec::new(),
        pathing_status: None,
        unit_response_state: None,
        group_route_tile_ids_if_empty: Vec::new(),
        group_command_state: None,
    }
}

pub fn rts_formation_move_preview_stage_fixtures() -> Vec<RtsFormationMovePreviewStageFixture> {
    let mut fixtures = vec![
        rts_formation_move_preview_fixture(
            "destination_ghost",
            "select-control-group",
            "box:frontline",
        ),
        rts_formation_move_preview_fixture("wedge_spacing", "move", "8,4:wedge"),
        rts_formation_move_preview_fixture("line_reflow", "move", "8,4:line"),
        rts_formation_move_preview_fixture("collision_avoidance", "move", "8,4:wedge"),
        rts_formation_move_preview_fixture("split_avoidance", "move", "6,5:split"),
        rts_formation_move_preview_fixture("commit_spacing", "move", "9,2:rally"),
    ];

    if let Some(fixture) = fixtures.get_mut(0) {
        fixture.command_destination_tile = Some("8,4".to_string());
        fixture.path_tile_ids = rts_string_vec(["6,5", "7,5", "8,4"]);
        fixture.blocked_tile_ids = rts_string_vec(["7,4"]);
        fixture.formation_slot_tile_ids = rts_string_vec(["8,4", "7,4", "8,5", "9,4"]);
        fixture.disperse_tile_ids = rts_string_vec(["6,5", "7,5", "8,4", "8,5"]);
        fixture.pathing_status = Some("hover_preview:8,4".to_string());
        fixture.unit_response_state = Some("ghost_before_commit".to_string());
    }
    if let Some(fixture) = fixtures.get_mut(3) {
        fixture.pathing_status = Some("detour:7,4".to_string());
    }
    if let Some(fixture) = fixtures.get_mut(4) {
        fixture.group_route_tile_ids_if_empty = rts_string_vec(["5,5", "6,4", "6,5", "7,5", "6,6"]);
        fixture.group_command_state = Some("split_route:group_2".to_string());
    }

    fixtures
}

pub fn rts_formation_move_preview_stage_ids() -> Vec<String> {
    rts_formation_move_preview_stage_fixtures()
        .into_iter()
        .map(|fixture| fixture.stage)
        .collect()
}

pub fn rts_control_group_recall_formation_preview_input_source() -> &'static str {
    "classic_rts_control_group_recall_formation_preview_input"
}

pub fn rts_control_group_recall_formation_preview_renderer_path() -> &'static str {
    "classic_draw_scene+classic_draw_rts_control_group_recall_formation_preview_overlay"
}

pub fn rts_control_group_recall_formation_preview_surface() -> &'static str {
    "control_group_28_recall_focus+formation_anchor+slot_markers+member_filter_states"
}

fn rts_control_group_recall_formation_preview_fixture(
    stage: &str,
    kind: &str,
    payload: &str,
) -> RtsControlGroupRecallFormationPreviewStageFixture {
    let filtered_member_ids = if stage == "filtered_invalid" {
        rts_string_vec([
            "missing:multi0.recall.formation.missing",
            "foreign:map.actor1",
        ])
    } else {
        Vec::new()
    };
    let cleared_old_member_ids = if stage == "filtered_invalid" {
        rts_string_vec([
            "old:multi0.recall.formation.old.seed",
            "old:multi0.recall.formation.old.wing",
        ])
    } else {
        Vec::new()
    };

    RtsControlGroupRecallFormationPreviewStageFixture {
        stage: stage.to_string(),
        action: RtsOrderQueueReplayAction {
            kind: kind.to_string(),
            payload: payload.to_string(),
        },
        history_entry: format!("control_group_recall_formation_preview:{stage}"),
        input_source: rts_control_group_recall_formation_preview_input_source().to_string(),
        renderer_path: rts_control_group_recall_formation_preview_renderer_path().to_string(),
        preview_surface: rts_control_group_recall_formation_preview_surface().to_string(),
        control_group_id: "28".to_string(),
        active_control_group_ids: rts_string_vec(["26", "27", "28"]),
        selected_unit_ids: rts_string_vec([
            "multi0.recall.formation.runner",
            "multi0.recall.formation.wing",
        ]),
        stance: "guard".to_string(),
        recall_focus_tile: "1,30".to_string(),
        formation_anchor_tile: "1,31".to_string(),
        path_tile_ids: rts_string_vec(["1,30", "1,31", "2,31"]),
        formation_slot_tile_ids: rts_string_vec(["1,31", "2,31"]),
        queued_member_ids: rts_string_vec([
            "multi0.recall.formation.runner",
            "multi0.recall.formation.wing",
        ]),
        filtered_member_ids,
        cleared_old_member_ids,
        group_command_state: format!("recall_formation_preview:{stage}:group_28"),
    }
}

pub fn rts_control_group_recall_formation_preview_stage_fixtures(
) -> Vec<RtsControlGroupRecallFormationPreviewStageFixture> {
    [
        ("recall_focus_hud", "select-control-group", "28"),
        ("formation_anchor_slots", "move", "1,31:line"),
        ("queued_valid_members", "move", "1,31:line"),
        ("filtered_invalid", "move", "1,31:line"),
    ]
    .into_iter()
    .map(|(stage, kind, payload)| {
        rts_control_group_recall_formation_preview_fixture(stage, kind, payload)
    })
    .collect()
}

pub fn rts_control_group_recall_formation_preview_stage_ids() -> Vec<String> {
    rts_control_group_recall_formation_preview_stage_fixtures()
        .into_iter()
        .map(|fixture| fixture.stage)
        .collect()
}

pub fn rts_control_group_recall_override_preview_input_source() -> &'static str {
    "classic_rts_control_group_recall_override_preview_input"
}

pub fn rts_control_group_recall_override_preview_renderer_path() -> &'static str {
    "classic_draw_scene+classic_draw_rts_control_group_recall_override_preview_overlay"
}

pub fn rts_control_group_recall_override_preview_surface() -> &'static str {
    "group_26_recall_order_queue+group_27_recall_override_cancel_final_filtered"
}

fn rts_control_group_recall_override_preview_fixture(
    stage: &str,
    kind: &str,
    payload: &str,
) -> RtsControlGroupRecallOverridePreviewStageFixture {
    let is_group_26 = stage.starts_with("group_26");
    let selected_unit_ids = if is_group_26 {
        rts_string_vec(["multi0.recall.order.runner", "multi0.recall.order.wing"])
    } else {
        rts_string_vec([
            "multi0.recall.override.runner",
            "multi0.recall.override.wing",
        ])
    };
    let filtered_member_ids = if stage == "group_27_final_filtered" {
        rts_string_vec([
            "missing:multi0.recall.override.missing",
            "foreign:map.actor1",
        ])
    } else {
        Vec::new()
    };
    let cleared_old_member_ids = if stage == "group_27_final_filtered" {
        rts_string_vec([
            "old:multi0.recall.override.old.seed",
            "old:multi0.recall.override.old.wing",
        ])
    } else {
        Vec::new()
    };

    RtsControlGroupRecallOverridePreviewStageFixture {
        stage: stage.to_string(),
        action: RtsOrderQueueReplayAction {
            kind: kind.to_string(),
            payload: payload.to_string(),
        },
        history_entry: format!("control_group_recall_override_preview:{stage}"),
        input_source: rts_control_group_recall_override_preview_input_source().to_string(),
        renderer_path: rts_control_group_recall_override_preview_renderer_path().to_string(),
        preview_surface: rts_control_group_recall_override_preview_surface().to_string(),
        control_group_id: if is_group_26 { "26" } else { "27" }.to_string(),
        active_control_group_ids: rts_string_vec(["26", "27", "28"]),
        selected_unit_ids: selected_unit_ids.clone(),
        stance: "guard".to_string(),
        recall_focus_tile: if is_group_26 { "18,30" } else { "21,30" }.to_string(),
        queued_target_tile: if is_group_26 { "18,31" } else { "21,25" }.to_string(),
        canceled_target_tile: if is_group_26 {
            None
        } else {
            Some("21,25".to_string())
        },
        path_tile_ids: if is_group_26 {
            rts_string_vec(["18,30", "18,31"])
        } else {
            rts_string_vec(["21,30", "21,29", "21,27", "21,25"])
        },
        group_route_tile_ids: if is_group_26 {
            rts_string_vec(["18,30", "18,31"])
        } else {
            rts_string_vec(["21,25", "20,30", "22,30"])
        },
        override_final_tile_ids: if is_group_26 {
            Vec::new()
        } else {
            rts_string_vec(["20,30", "22,30"])
        },
        queued_member_ids: selected_unit_ids,
        canceled_member_ids: if is_group_26 {
            Vec::new()
        } else {
            rts_string_vec([
                "multi0.recall.override.runner",
                "multi0.recall.override.wing",
            ])
        },
        filtered_member_ids,
        cleared_old_member_ids,
        group_command_state: format!("recall_override_preview:{stage}"),
    }
}

pub fn rts_control_group_recall_override_preview_stage_fixtures(
) -> Vec<RtsControlGroupRecallOverridePreviewStageFixture> {
    [
        ("group_26_recall_focus", "select-control-group", "26"),
        ("group_26_queued_order", "move", "18,31:line"),
        ("group_27_override_cancel", "select-control-group", "27"),
        ("group_27_final_filtered", "move", "20,30:line"),
    ]
    .into_iter()
    .map(|(stage, kind, payload)| {
        rts_control_group_recall_override_preview_fixture(stage, kind, payload)
    })
    .collect()
}

pub fn rts_control_group_recall_override_preview_stage_ids() -> Vec<String> {
    rts_control_group_recall_override_preview_stage_fixtures()
        .into_iter()
        .map(|fixture| fixture.stage)
        .collect()
}

fn rts_default_formation_execution_units() -> Vec<String> {
    rts_string_vec([
        "player",
        "square_guard_patrol",
        "square_worker_carry",
        "square_creep_wander",
    ])
}

fn rts_formation_slot_claims(selected_unit_ids: &[String], slots: &[String]) -> Vec<String> {
    selected_unit_ids
        .iter()
        .enumerate()
        .map(|(slot_index, unit_id)| {
            let slot = slots
                .get(slot_index % slots.len().max(1))
                .cloned()
                .unwrap_or_else(|| "8,4".to_string());
            format!("{unit_id}@{slot}")
        })
        .collect()
}

fn rts_formation_path_reservations(path_tile_ids: &[String]) -> Vec<String> {
    path_tile_ids
        .iter()
        .enumerate()
        .map(|(step_index, tile_id)| format!("step{}:{tile_id}", step_index + 1))
        .collect()
}

fn rts_formation_movement_offsets(selected_unit_ids: &[String], slots: &[String]) -> Vec<String> {
    selected_unit_ids
        .iter()
        .enumerate()
        .map(|(unit_index, unit_id)| {
            let slot = slots
                .get(unit_index % slots.len().max(1))
                .cloned()
                .unwrap_or_else(|| "8,4".to_string());
            format!("{unit_id}:offset:{}@{slot}", unit_index * 2)
        })
        .collect()
}

pub fn rts_formation_move_execution_fixtures() -> RtsFormationMoveExecutionFixtures {
    let selected_unit_ids = rts_default_formation_execution_units();
    let slot_tiles = rts_string_vec(["8,4", "7,4", "8,5", "9,4"]);
    let disperse_tiles = rts_string_vec(["6,5", "7,5", "8,4", "8,5"]);
    let slot_path = rts_string_vec(["6,5", "7,5", "8,4"]);
    let reroute_path = rts_string_vec(["6,5", "7,5", "8,5", "8,4"]);
    let arrival_path = rts_string_vec(["6,5", "7,5", "8,5", "9,4", "9,2"]);
    let slot_claims = rts_formation_slot_claims(&selected_unit_ids, &slot_tiles);
    let path_reservations = rts_formation_path_reservations(&slot_path);
    let movement_offsets = rts_formation_movement_offsets(&selected_unit_ids, &slot_tiles);
    let renderer_path = "classic_draw_scene+classic_draw_rts_formation_move_execution_overlay";
    let input_source = "classic_rts_formation_move_execution_input";
    let preview_surface =
        "slot_claim+path_reservation+stagger_step+crowd_avoidance+blocked_reroute+arrival_lock";

    let stage_fixture = |stage: &str,
                         kind: &str,
                         payload: &str,
                         command_destination_tile: Option<&str>,
                         path_tile_ids: Vec<String>,
                         blocked_tile_ids: Vec<String>,
                         formation_slot_tile_ids: Vec<String>,
                         disperse_tile_ids: Vec<String>,
                         group_route_tile_ids: Vec<String>,
                         pathing_status: &str,
                         unit_response_state: &str,
                         group_command_state: &str,
                         slot_claims: Vec<String>,
                         path_reservations: Vec<String>,
                         movement_offsets: Vec<String>,
                         arrival_locked_unit_ids: Vec<String>,
                         lagging_unit_ids: Vec<String>|
     -> RtsFormationMoveExecutionStageFixture {
        RtsFormationMoveExecutionStageFixture {
            stage: stage.to_string(),
            action: RtsOrderQueueReplayAction {
                kind: kind.to_string(),
                payload: payload.to_string(),
            },
            history_entry: format!("formation_move_execution:{stage}"),
            input_source: input_source.to_string(),
            renderer_path: renderer_path.to_string(),
            preview_surface: preview_surface.to_string(),
            selected_unit_ids: selected_unit_ids.clone(),
            command_destination_tile: command_destination_tile.map(str::to_string),
            path_tile_ids,
            blocked_tile_ids,
            formation_slot_tile_ids,
            disperse_tile_ids,
            group_route_tile_ids,
            pathing_status: pathing_status.to_string(),
            unit_response_state: unit_response_state.to_string(),
            group_command_state: group_command_state.to_string(),
            slot_claims,
            path_reservations,
            movement_offsets,
            arrival_locked_unit_ids,
            lagging_unit_ids,
        }
    };

    RtsFormationMoveExecutionFixtures {
        selected_unit_ids: selected_unit_ids.clone(),
        stages: vec![
            stage_fixture(
                "slot_claim",
                "select-control-group",
                "box:frontline",
                Some("8,4"),
                slot_path.clone(),
                rts_string_vec(["7,4"]),
                slot_tiles.clone(),
                disperse_tiles.clone(),
                Vec::new(),
                "slot_claim_preview:8,4",
                "slot_claimed:frontline",
                "",
                slot_claims.clone(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ),
            stage_fixture(
                "path_reservation",
                "move",
                "8,4:wedge",
                Some("8,4"),
                slot_path.clone(),
                rts_string_vec(["7,4"]),
                slot_tiles.clone(),
                disperse_tiles.clone(),
                slot_path.clone(),
                "slot_claim_preview:8,4",
                "path_reserved:frontline",
                "",
                slot_claims.clone(),
                path_reservations.clone(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ),
            stage_fixture(
                "stagger_step",
                "move",
                "8,4:line",
                Some("8,4"),
                slot_path.clone(),
                rts_string_vec(["7,4"]),
                slot_tiles.clone(),
                disperse_tiles.clone(),
                slot_path.clone(),
                "slot_claim_preview:8,4",
                "stagger_step:line_reflow",
                "",
                slot_claims.clone(),
                path_reservations.clone(),
                movement_offsets.clone(),
                Vec::new(),
                Vec::new(),
            ),
            stage_fixture(
                "crowd_avoidance",
                "move",
                "6,5:split",
                Some("8,4"),
                slot_path.clone(),
                rts_string_vec(["7,4"]),
                slot_tiles.clone(),
                rts_string_vec(["5,5", "6,4", "6,5", "7,5", "6,6"]),
                slot_path.clone(),
                "slot_claim_preview:8,4",
                "crowd_avoidance:split_lane",
                "split_route:group_2",
                slot_claims.clone(),
                path_reservations.clone(),
                movement_offsets.clone(),
                Vec::new(),
                Vec::new(),
            ),
            stage_fixture(
                "blocked_reroute",
                "move",
                "8,4:wedge",
                Some("8,4"),
                reroute_path.clone(),
                rts_string_vec(["7,4"]),
                slot_tiles.clone(),
                disperse_tiles.clone(),
                reroute_path.clone(),
                "reroute:7,4",
                "blocked_reroute:active",
                "split_route:group_2",
                slot_claims.clone(),
                rts_formation_path_reservations(&reroute_path),
                movement_offsets.clone(),
                Vec::new(),
                selected_unit_ids.iter().take(2).cloned().collect(),
            ),
            stage_fixture(
                "arrival_lock",
                "move",
                "9,2:rally",
                Some("9,2"),
                arrival_path.clone(),
                rts_string_vec(["7,4"]),
                slot_tiles,
                disperse_tiles,
                arrival_path.clone(),
                "arrival_brake:slot_lock",
                "arrival_locked:9,2",
                "split_route:group_2",
                slot_claims,
                rts_formation_path_reservations(&arrival_path),
                movement_offsets,
                selected_unit_ids.clone(),
                Vec::new(),
            ),
        ],
    }
}

pub fn rts_local_obstruction_recovery_fixtures() -> RtsLocalObstructionRecoveryFixtures {
    let selected_unit_ids = rts_default_formation_execution_units();
    let slot_tiles = rts_string_vec(["8,4", "8,5", "9,4", "9,5"]);
    let renderer_path = "classic_draw_scene+classic_draw_rts_local_obstruction_recovery_overlay";
    let input_source = "classic_rts_local_obstruction_recovery_input";
    let preview_surface = "detect_block+hold_queue+side_step+gap_claim+flow_resume";
    let gap_claims = slot_tiles
        .iter()
        .enumerate()
        .map(|(slot_index, tile)| format!("unit_{}@{tile}", slot_index + 1))
        .collect::<Vec<_>>();

    let stage_fixture = |stage: &str,
                         kind: &str,
                         payload: &str,
                         command_destination_tile: Option<&str>,
                         path_tile_ids: Vec<String>,
                         blocked_tile_ids: Vec<String>,
                         disperse_tile_ids: Vec<String>,
                         formation_slot_tile_ids: Vec<String>,
                         group_route_tile_ids: Vec<String>,
                         queued_unit_ids: Vec<String>,
                         side_step_unit_ids: Vec<String>,
                         gap_claims: Vec<String>,
                         pathing_status: &str,
                         unit_response_state: &str,
                         group_command_state: &str|
     -> RtsLocalObstructionRecoveryStageFixture {
        RtsLocalObstructionRecoveryStageFixture {
            stage: stage.to_string(),
            action: RtsOrderQueueReplayAction {
                kind: kind.to_string(),
                payload: payload.to_string(),
            },
            history_entry: format!("local_obstruction_recovery:{stage}"),
            input_source: input_source.to_string(),
            renderer_path: renderer_path.to_string(),
            preview_surface: preview_surface.to_string(),
            selected_unit_ids: selected_unit_ids.clone(),
            command_destination_tile: command_destination_tile.map(str::to_string),
            path_tile_ids,
            blocked_tile_ids,
            disperse_tile_ids,
            formation_slot_tile_ids,
            group_route_tile_ids,
            queued_unit_ids,
            side_step_unit_ids,
            gap_claims,
            pathing_status: pathing_status.to_string(),
            unit_response_state: unit_response_state.to_string(),
            group_command_state: group_command_state.to_string(),
        }
    };

    RtsLocalObstructionRecoveryFixtures {
        selected_unit_ids: selected_unit_ids.clone(),
        stages: vec![
            stage_fixture(
                "detect_block",
                "move",
                "8,4:wedge",
                Some("8,4"),
                rts_string_vec(["5,5", "6,5", "7,4", "8,4"]),
                rts_string_vec(["7,4", "7,5"]),
                rts_string_vec(["6,4", "6,6"]),
                slot_tiles.clone(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                "blocked:7,4",
                "blocked:leader_hold",
                "formation_hold:frontline",
            ),
            stage_fixture(
                "hold_queue",
                "move",
                "8,4:line",
                Some("8,4"),
                rts_string_vec(["5,5", "6,5", "6,6", "7,6"]),
                rts_string_vec(["7,4", "7,5"]),
                rts_string_vec(["6,4", "6,6"]),
                slot_tiles.clone(),
                Vec::new(),
                selected_unit_ids.iter().skip(1).cloned().collect(),
                Vec::new(),
                Vec::new(),
                "blocked:7,4",
                "queue_wait:followers",
                "queued:outside_block",
            ),
            stage_fixture(
                "side_step",
                "move",
                "6,5:split",
                Some("8,4"),
                rts_string_vec(["5,5", "6,5", "6,6", "7,6"]),
                rts_string_vec(["7,4", "7,5"]),
                rts_string_vec(["6,4", "6,6", "7,6", "8,5"]),
                slot_tiles.clone(),
                Vec::new(),
                Vec::new(),
                selected_unit_ids.iter().take(2).cloned().collect(),
                Vec::new(),
                "blocked:7,4",
                "side_step:gap_opening",
                "split_lane:local",
            ),
            stage_fixture(
                "gap_claim",
                "select-control-group",
                "box:frontline",
                Some("8,4"),
                rts_string_vec(["5,5", "6,5", "6,6", "7,6"]),
                rts_string_vec(["7,4"]),
                rts_string_vec(["6,4", "6,6", "7,6", "8,5"]),
                slot_tiles.clone(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                gap_claims,
                "blocked:7,4",
                "gap_claimed:unit_2",
                "slot_reassign:unit_2",
            ),
            stage_fixture(
                "flow_resume",
                "move",
                "9,2:rally",
                Some("9,2"),
                rts_string_vec(["6,5", "7,5", "8,5", "9,4", "9,2"]),
                rts_string_vec(["7,4"]),
                rts_string_vec(["6,4", "6,6", "7,6", "8,5"]),
                slot_tiles,
                rts_string_vec(["6,5", "7,5", "8,5", "9,4", "9,2"]),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                "resume:detour_committed",
                "flow_resumed:order_intact",
                "resume_route:group",
            ),
        ],
    }
}

pub fn rts_control_group_command_feedback_strip_fixtures(
) -> RtsControlGroupCommandFeedbackStripFixtures {
    let active_control_group_ids = rts_string_vec(["26", "27", "28"]);
    let control_group_assignments = rts_string_vec([
        "26:multi0.recall.order.runner|multi0.recall.order.wing",
        "27:multi0.recall.override.runner|multi0.recall.override.wing",
        "28:multi0.recall.formation.runner|multi0.recall.formation.wing",
    ]);
    let group_26_member_ids =
        rts_string_vec(["multi0.recall.order.runner", "multi0.recall.order.wing"]);
    let group_27_member_ids = rts_string_vec([
        "multi0.recall.override.runner",
        "multi0.recall.override.wing",
    ]);
    let group_28_member_ids = rts_string_vec([
        "multi0.recall.formation.runner",
        "multi0.recall.formation.wing",
    ]);
    let group_28_formation_slot_tile_ids = rts_string_vec(["1,31", "2,31"]);
    let filtered_member_ids = rts_string_vec([
        "missing:multi0.recall.formation.missing",
        "foreign:map.actor1",
    ]);
    let cleared_old_member_ids = rts_string_vec([
        "old:multi0.recall.formation.old.seed",
        "old:multi0.recall.formation.old.wing",
    ]);

    RtsControlGroupCommandFeedbackStripFixtures {
        active_control_group_ids: active_control_group_ids.clone(),
        control_group_assignments,
        ability_command_ids: rts_string_vec(["move", "stop", "hold", "patrol"]),
        stages: vec![
            RtsControlGroupCommandFeedbackStripStageFixture {
                stage: "group_26_queued".to_string(),
                action: RtsOrderQueueReplayAction {
                    kind: "move".to_string(),
                    payload: "18,31:line".to_string(),
                },
                input_source: "classic_rts_control_group_command_feedback_strip_input".to_string(),
                renderer_path: "classic_draw_scene".to_string(),
                preview_surface: "classic_draw_scene_command_feedback_strip".to_string(),
                control_group_id: "26".to_string(),
                active_control_group_ids: active_control_group_ids.clone(),
                selected_unit_ids: group_26_member_ids.clone(),
                stance: "guard".to_string(),
                recall_focus_tile: "18,30".to_string(),
                queued_target_tile: Some("18,31".to_string()),
                canceled_target_tile: None,
                override_final_tile_ids: Vec::new(),
                formation_anchor_tile: None,
                formation_slot_tile_ids: Vec::new(),
                queued_member_ids: group_26_member_ids,
                canceled_member_ids: Vec::new(),
                filtered_member_ids: Vec::new(),
                cleared_old_member_ids: Vec::new(),
                path_tile_ids: rts_string_vec(["18,30", "18,31"]),
                group_route_tile_ids: rts_string_vec(["18,30", "18,31"]),
                command_queue_entries: rts_string_vec([
                    "queued_group_order:Multi0:26:move:2actors",
                    "queued_order_reached:26:multi0.recall.order.runner:chain0:18,31",
                    "queued_order_reached:26:multi0.recall.order.wing:chain0:18,31",
                ]),
                combat_event: "control_group_command_feedback_strip:group_26_queued".to_string(),
                group_command_state: "command_feedback_strip:group_26_queued".to_string(),
                player_tile_x: 18,
                player_tile_y: 30,
            },
            RtsControlGroupCommandFeedbackStripStageFixture {
                stage: "group_27_override".to_string(),
                action: RtsOrderQueueReplayAction {
                    kind: "select-control-group".to_string(),
                    payload: "27".to_string(),
                },
                input_source: "classic_rts_control_group_command_feedback_strip_input".to_string(),
                renderer_path: "classic_draw_scene".to_string(),
                preview_surface: "classic_draw_scene_command_feedback_strip".to_string(),
                control_group_id: "27".to_string(),
                active_control_group_ids: active_control_group_ids.clone(),
                selected_unit_ids: group_27_member_ids.clone(),
                stance: "guard".to_string(),
                recall_focus_tile: "21,30".to_string(),
                queued_target_tile: None,
                canceled_target_tile: Some("21,25".to_string()),
                override_final_tile_ids: rts_string_vec(["20,30", "22,30"]),
                formation_anchor_tile: Some("21,25".to_string()),
                formation_slot_tile_ids: Vec::new(),
                queued_member_ids: Vec::new(),
                canceled_member_ids: group_27_member_ids,
                filtered_member_ids: Vec::new(),
                cleared_old_member_ids: Vec::new(),
                path_tile_ids: rts_string_vec(["21,30", "21,29", "21,27", "21,25"]),
                group_route_tile_ids: rts_string_vec(["21,25", "20,30", "22,30"]),
                command_queue_entries: rts_string_vec([
                    "queued_order_execute:27:multi0.recall.override.runner:move:chain0",
                    "queued_order_execute:27:multi0.recall.override.wing:move:chain0",
                    "queued_order_override:Multi0:multi0.recall.override.runner:move:cleared1",
                    "queued_order_override:Multi0:multi0.recall.override.wing:move:cleared1",
                ]),
                combat_event: "control_group_command_feedback_strip:group_27_override".to_string(),
                group_command_state: "command_feedback_strip:group_27_override".to_string(),
                player_tile_x: 21,
                player_tile_y: 30,
            },
            RtsControlGroupCommandFeedbackStripStageFixture {
                stage: "group_28_formation".to_string(),
                action: RtsOrderQueueReplayAction {
                    kind: "move".to_string(),
                    payload: "1,31:line".to_string(),
                },
                input_source: "classic_rts_control_group_command_feedback_strip_input".to_string(),
                renderer_path: "classic_draw_scene".to_string(),
                preview_surface: "classic_draw_scene_command_feedback_strip".to_string(),
                control_group_id: "28".to_string(),
                active_control_group_ids: active_control_group_ids.clone(),
                selected_unit_ids: group_28_member_ids.clone(),
                stance: "guard".to_string(),
                recall_focus_tile: "1,30".to_string(),
                queued_target_tile: Some("1,31".to_string()),
                canceled_target_tile: None,
                override_final_tile_ids: Vec::new(),
                formation_anchor_tile: Some("1,31".to_string()),
                formation_slot_tile_ids: group_28_formation_slot_tile_ids.clone(),
                queued_member_ids: group_28_member_ids.clone(),
                canceled_member_ids: Vec::new(),
                filtered_member_ids: Vec::new(),
                cleared_old_member_ids: Vec::new(),
                path_tile_ids: rts_string_vec(["1,30", "1,31", "2,31"]),
                group_route_tile_ids: rts_string_vec(["1,30", "1,31", "2,31"]),
                command_queue_entries: rts_string_vec([
                    "formation_group_order:Multi0:28:1,31:2slots:0reassigned",
                    "formation_move_slot:Multi0:28:multi0.recall.formation.runner:slot0:1,30->1,31",
                    "formation_move_slot:Multi0:28:multi0.recall.formation.wing:slot1:1,30->2,31",
                ]),
                combat_event: "control_group_command_feedback_strip:group_28_formation".to_string(),
                group_command_state: "command_feedback_strip:group_28_formation".to_string(),
                player_tile_x: 1,
                player_tile_y: 30,
            },
            RtsControlGroupCommandFeedbackStripStageFixture {
                stage: "group_28_filtered".to_string(),
                action: RtsOrderQueueReplayAction {
                    kind: "move".to_string(),
                    payload: "1,31:line".to_string(),
                },
                input_source: "classic_rts_control_group_command_feedback_strip_input".to_string(),
                renderer_path: "classic_draw_scene".to_string(),
                preview_surface: "classic_draw_scene_command_feedback_strip".to_string(),
                control_group_id: "28".to_string(),
                active_control_group_ids,
                selected_unit_ids: group_28_member_ids.clone(),
                stance: "guard".to_string(),
                recall_focus_tile: "1,30".to_string(),
                queued_target_tile: Some("1,31".to_string()),
                canceled_target_tile: None,
                override_final_tile_ids: Vec::new(),
                formation_anchor_tile: Some("1,31".to_string()),
                formation_slot_tile_ids: group_28_formation_slot_tile_ids,
                queued_member_ids: group_28_member_ids,
                canceled_member_ids: Vec::new(),
                filtered_member_ids: filtered_member_ids.clone(),
                cleared_old_member_ids: cleared_old_member_ids.clone(),
                path_tile_ids: rts_string_vec(["1,30", "1,31", "2,31"]),
                group_route_tile_ids: rts_string_vec(["1,30", "1,31", "2,31"]),
                command_queue_entries: rts_string_vec([
                    "formation_group_order:Multi0:28:1,31:2slots:0reassigned",
                    "formation_move_slot:Multi0:28:multi0.recall.formation.runner:slot0:1,30->1,31",
                    "formation_move_slot:Multi0:28:multi0.recall.formation.wing:slot1:1,30->2,31",
                    "filtered_member:missing:multi0.recall.formation.missing",
                    "filtered_member:foreign:map.actor1",
                    "filtered_member:old:multi0.recall.formation.old.seed",
                    "filtered_member:old:multi0.recall.formation.old.wing",
                ]),
                combat_event: "control_group_command_feedback_strip:group_28_filtered".to_string(),
                group_command_state: "command_feedback_strip:group_28_filtered".to_string(),
                player_tile_x: 1,
                player_tile_y: 30,
            },
        ],
    }
}

pub fn rts_control_group_command_feedback_lifecycle_fixtures(
) -> RtsControlGroupCommandFeedbackLifecycleFixtures {
    let active_control_group_ids = rts_string_vec(["26", "27", "28"]);
    let covered_group_ids = active_control_group_ids.clone();
    let control_group_assignments = rts_string_vec([
        "26:multi0.recall.order.runner|multi0.recall.order.wing",
        "27:multi0.recall.override.runner|multi0.recall.override.wing",
        "28:multi0.recall.formation.runner|multi0.recall.formation.wing",
    ]);
    let group_26_member_ids =
        rts_string_vec(["multi0.recall.order.runner", "multi0.recall.order.wing"]);
    let group_27_member_ids = rts_string_vec([
        "multi0.recall.override.runner",
        "multi0.recall.override.wing",
    ]);
    let group_28_member_ids = rts_string_vec([
        "multi0.recall.formation.runner",
        "multi0.recall.formation.wing",
    ]);
    let all_member_ids = rts_string_vec([
        "multi0.recall.order.runner",
        "multi0.recall.order.wing",
        "multi0.recall.override.runner",
        "multi0.recall.override.wing",
        "multi0.recall.formation.runner",
        "multi0.recall.formation.wing",
    ]);
    let path_tile_ids = rts_string_vec(["18,30", "18,31", "21,25", "20,30", "22,30", "1,31"]);
    let group_route_tile_ids = rts_string_vec(["18,31", "20,30", "22,30", "1,31", "2,31"]);
    let formation_slot_tile_ids = rts_string_vec(["1,31", "2,31"]);
    let command_queue_entries = rts_string_vec([
        "queued_group_order:Multi0:26:move:2actors",
        "queued_order_cancelled:27:multi0.recall.override.runner:21,25",
        "queued_order_override_final:27:20,30|22,30",
        "formation_group_order:Multi0:28:1,31:2slots:0reassigned",
        "control_group_member_filtered:Multi0:28:missing:multi0.recall.formation.missing,foreign:map.actor1",
        "control_group_old_members_cleared:Multi0:28:multi0.recall.formation.old.seed|multi0.recall.formation.old.wing",
    ]);

    let stage_fixture = |stage: &str,
                         age_ticks: u8,
                         kind: &str,
                         payload: &str,
                         control_group_id: &str|
     -> RtsControlGroupCommandFeedbackLifecycleStageFixture {
        RtsControlGroupCommandFeedbackLifecycleStageFixture {
            stage: stage.to_string(),
            age_ticks,
            action: RtsOrderQueueReplayAction {
                kind: kind.to_string(),
                payload: payload.to_string(),
            },
            input_source: "classic_rts_control_group_command_feedback_lifecycle_input".to_string(),
            renderer_path: "classic_draw_scene".to_string(),
            preview_surface: "classic_draw_scene_command_feedback_lifecycle".to_string(),
            control_group_id: control_group_id.to_string(),
            active_control_group_ids: active_control_group_ids.clone(),
            covered_group_ids: covered_group_ids.clone(),
            selected_unit_ids: all_member_ids.clone(),
            stance: "guard".to_string(),
            minimap_command_tile_id: "18,30".to_string(),
            command_destination_tile: "18,31".to_string(),
            path_tile_ids: path_tile_ids.clone(),
            group_route_tile_ids: group_route_tile_ids.clone(),
            formation_slot_tile_ids: formation_slot_tile_ids.clone(),
            command_queue_entries: command_queue_entries.clone(),
            lifecycle_event: format!("control_group_command_feedback_lifecycle:{stage}"),
            group_command_state: format!("command_feedback_lifecycle:{stage}"),
            player_tile_x: 18,
            player_tile_y: 30,
        }
    };
    let stages = vec![
        stage_fixture("fresh", 0, "move", "18,31:line", "26"),
        stage_fixture("dimmed", 4, "move", "1,31:line", "28"),
        stage_fixture("cleared", 8, "select-control-group", "28", "28"),
    ];

    RtsControlGroupCommandFeedbackLifecycleFixtures {
        active_control_group_ids,
        covered_group_ids,
        control_group_assignments,
        ability_command_ids: rts_string_vec(["move", "stop", "hold", "patrol"]),
        group_26_member_ids,
        group_27_member_ids,
        group_28_member_ids,
        all_member_ids,
        group_26_queued_target_tile: "18,31".to_string(),
        group_27_canceled_target_tile: "21,25".to_string(),
        group_27_override_final_tile_ids: rts_string_vec(["20,30", "22,30"]),
        group_28_formation_anchor_tile: "1,31".to_string(),
        group_28_formation_slot_tile_ids: formation_slot_tile_ids,
        stages,
    }
}

pub fn rts_control_group_command_feedback_replay_fixtures(
) -> RtsControlGroupCommandFeedbackReplayFixtures {
    let group_26_member_ids =
        rts_string_vec(["multi0.recall.order.runner", "multi0.recall.order.wing"]);
    let group_27_member_ids = rts_string_vec([
        "multi0.recall.override.runner",
        "multi0.recall.override.wing",
    ]);
    let group_28_member_ids = rts_string_vec([
        "multi0.recall.formation.runner",
        "multi0.recall.formation.wing",
    ]);
    let all_member_ids = rts_string_vec([
        "multi0.recall.order.runner",
        "multi0.recall.order.wing",
        "multi0.recall.override.runner",
        "multi0.recall.override.wing",
        "multi0.recall.formation.runner",
        "multi0.recall.formation.wing",
    ]);

    RtsControlGroupCommandFeedbackReplayFixtures {
        retained_history_group_ids: rts_string_vec(["26", "27", "28"]),
        pruned_history_group_ids: rts_string_vec(["25", "24"]),
        group_26_member_ids: group_26_member_ids.clone(),
        group_27_member_ids: group_27_member_ids.clone(),
        group_28_member_ids: group_28_member_ids.clone(),
        all_member_ids,
        history_entries: vec![
            RtsControlGroupCommandFeedbackHistoryEntry {
                group_id: "26".to_string(),
                badge: "QUEUE".to_string(),
                target_tile: Some("18,31".to_string()),
                canceled_target_tile: None,
                override_final_tile_ids: Vec::new(),
                formation_anchor_tile: None,
                formation_slot_tile_ids: Vec::new(),
                age_ticks: 0,
                bounded_history_index: Some(0),
                member_ids: group_26_member_ids,
                prune_reason: None,
            },
            RtsControlGroupCommandFeedbackHistoryEntry {
                group_id: "27".to_string(),
                badge: "CANCEL_FINAL".to_string(),
                target_tile: None,
                canceled_target_tile: Some("21,25".to_string()),
                override_final_tile_ids: rts_string_vec(["20,30", "22,30"]),
                formation_anchor_tile: None,
                formation_slot_tile_ids: Vec::new(),
                age_ticks: 4,
                bounded_history_index: Some(1),
                member_ids: group_27_member_ids,
                prune_reason: None,
            },
            RtsControlGroupCommandFeedbackHistoryEntry {
                group_id: "28".to_string(),
                badge: "FORMATION_FILTER_CLEAR".to_string(),
                target_tile: None,
                canceled_target_tile: None,
                override_final_tile_ids: Vec::new(),
                formation_anchor_tile: Some("1,31".to_string()),
                formation_slot_tile_ids: rts_string_vec(["1,31", "2,31"]),
                age_ticks: 8,
                bounded_history_index: Some(2),
                member_ids: group_28_member_ids,
                prune_reason: None,
            },
        ],
        pruned_history_entries: vec![
            RtsControlGroupCommandFeedbackHistoryEntry {
                group_id: "25".to_string(),
                badge: "OLD_QUEUE".to_string(),
                target_tile: Some("17,30".to_string()),
                canceled_target_tile: None,
                override_final_tile_ids: Vec::new(),
                formation_anchor_tile: None,
                formation_slot_tile_ids: Vec::new(),
                age_ticks: 16,
                bounded_history_index: None,
                member_ids: Vec::new(),
                prune_reason: Some("recent_three_capacity".to_string()),
            },
            RtsControlGroupCommandFeedbackHistoryEntry {
                group_id: "24".to_string(),
                badge: "OLD_CANCEL".to_string(),
                target_tile: Some("16,29".to_string()),
                canceled_target_tile: None,
                override_final_tile_ids: Vec::new(),
                formation_anchor_tile: None,
                formation_slot_tile_ids: Vec::new(),
                age_ticks: 20,
                bounded_history_index: None,
                member_ids: Vec::new(),
                prune_reason: Some("recent_three_capacity".to_string()),
            },
        ],
        command_steps: vec![
            RtsControlGroupCommandFeedbackStepFixture {
                step_index: 0,
                step_name: "select_group_26".to_string(),
                action_label: "RTS:SELECT:26".to_string(),
                preview_stage: None,
            },
            RtsControlGroupCommandFeedbackStepFixture {
                step_index: 1,
                step_name: "queue_group_26".to_string(),
                action_label: "RTS:MOVE:18,31:line".to_string(),
                preview_stage: Some("group_26_queued".to_string()),
            },
            RtsControlGroupCommandFeedbackStepFixture {
                step_index: 2,
                step_name: "select_group_27".to_string(),
                action_label: "RTS:SELECT:27".to_string(),
                preview_stage: None,
            },
            RtsControlGroupCommandFeedbackStepFixture {
                step_index: 3,
                step_name: "override_group_27".to_string(),
                action_label: "RTS:MOVE:21,25:line".to_string(),
                preview_stage: Some("group_27_override".to_string()),
            },
            RtsControlGroupCommandFeedbackStepFixture {
                step_index: 4,
                step_name: "select_group_28".to_string(),
                action_label: "RTS:SELECT:28".to_string(),
                preview_stage: None,
            },
            RtsControlGroupCommandFeedbackStepFixture {
                step_index: 5,
                step_name: "formation_group_28".to_string(),
                action_label: "RTS:MOVE:1,31:line".to_string(),
                preview_stage: Some("group_28_formation".to_string()),
            },
            RtsControlGroupCommandFeedbackStepFixture {
                step_index: 6,
                step_name: "bounded_history_after_clear".to_string(),
                action_label: "RTS:SELECT:26".to_string(),
                preview_stage: Some("cleared_history_bounded".to_string()),
            },
        ],
    }
}

pub fn rts_control_group_command_history_fixtures() -> RtsControlGroupCommandHistoryFixtures {
    let replay_fixtures = rts_control_group_command_feedback_replay_fixtures();
    let retained_history_group_ids = replay_fixtures.retained_history_group_ids.clone();
    let control_group_assignments = rts_string_vec([
        "26:multi0.recall.order.runner|multi0.recall.order.wing",
        "27:multi0.recall.override.runner|multi0.recall.override.wing",
        "28:multi0.recall.formation.runner|multi0.recall.formation.wing",
    ]);
    let ability_command_ids = rts_string_vec(["move", "stop", "hold", "patrol"]);
    let path_tile_ids = rts_string_vec(["18,30", "18,31", "21,25", "20,30", "22,30", "1,31"]);
    let group_route_tile_ids = rts_string_vec(["18,31", "20,30", "22,30", "1,31", "2,31"]);
    let formation_slot_tile_ids = rts_string_vec(["1,31", "2,31"]);
    let command_queue_entries = rts_string_vec([
        "history_row:26:queue:18,31:age0",
        "history_row:27:cancel_final:21,25:20,30|22,30:age4",
        "history_row:28:formation_filter_clear:1,31:1,31|2,31:age8",
    ]);
    let stage_fixture = |stage: &str,
                         lifecycle_stage: &str,
                         age_ticks: u8,
                         kind: &str,
                         payload: &str|
     -> RtsControlGroupCommandHistoryStageFixture {
        let lifecycle_event = format!("control_group_command_feedback_lifecycle:{lifecycle_stage}");
        let history_event = format!("control_group_command_history:{stage}");
        RtsControlGroupCommandHistoryStageFixture {
            stage: stage.to_string(),
            lifecycle_stage: lifecycle_stage.to_string(),
            age_ticks,
            action: RtsOrderQueueReplayAction {
                kind: kind.to_string(),
                payload: payload.to_string(),
            },
            input_source: "classic_rts_control_group_command_history_input".to_string(),
            renderer_path: "classic_draw_scene".to_string(),
            preview_surface: "classic_draw_scene_command_history".to_string(),
            control_group_id: "28".to_string(),
            active_control_group_ids: retained_history_group_ids.clone(),
            selected_unit_ids: replay_fixtures.all_member_ids.clone(),
            control_group_assignments: control_group_assignments.clone(),
            minimap_command_tile_id: "1,30".to_string(),
            command_destination_tile: "1,31".to_string(),
            path_tile_ids: path_tile_ids.clone(),
            group_route_tile_ids: group_route_tile_ids.clone(),
            formation_slot_tile_ids: formation_slot_tile_ids.clone(),
            group_command_state: format!("command_feedback_lifecycle:{lifecycle_stage}"),
            command_queue_entries: command_queue_entries.clone(),
            combat_event_entries: vec![lifecycle_event, history_event],
            active_strip_cleared: lifecycle_stage == "cleared",
            history_retained: true,
            history_overflow_row_count: 0,
            stale_pruned_group_visible: false,
            player_tile_x: 18,
            player_tile_y: 30,
        }
    };
    let stages = vec![
        stage_fixture("fresh_history_appended", "fresh", 0, "move", "18,31:line"),
        stage_fixture("dimmed_history_retained", "dimmed", 4, "move", "1,31:line"),
        stage_fixture(
            "cleared_history_retained",
            "cleared",
            8,
            "select-control-group",
            "28",
        ),
    ];

    RtsControlGroupCommandHistoryFixtures {
        retained_history_group_ids,
        pruned_history_group_ids: Vec::new(),
        control_group_assignments,
        ability_command_ids,
        group_26_member_ids: replay_fixtures.group_26_member_ids,
        group_27_member_ids: replay_fixtures.group_27_member_ids,
        group_28_member_ids: replay_fixtures.group_28_member_ids,
        all_member_ids: replay_fixtures.all_member_ids,
        history_entries: replay_fixtures.history_entries,
        pruned_history_entries: Vec::new(),
        stages,
    }
}

pub fn rts_control_group_command_history_prune_fixtures() -> RtsControlGroupCommandHistoryFixtures {
    let replay_fixtures = rts_control_group_command_feedback_replay_fixtures();
    let retained_history_group_ids = replay_fixtures.retained_history_group_ids.clone();
    let pruned_history_group_ids = replay_fixtures.pruned_history_group_ids.clone();
    let control_group_assignments = rts_string_vec([
        "26:multi0.recall.order.runner|multi0.recall.order.wing",
        "27:multi0.recall.override.runner|multi0.recall.override.wing",
        "28:multi0.recall.formation.runner|multi0.recall.formation.wing",
    ]);
    let ability_command_ids = rts_string_vec(["move", "stop", "hold", "patrol"]);
    let path_tile_ids = rts_string_vec([
        "17,30", "16,29", "18,30", "18,31", "21,25", "20,30", "22,30", "1,31",
    ]);
    let group_route_tile_ids = rts_string_vec(["18,31", "20,30", "22,30", "1,31", "2,31"]);
    let formation_slot_tile_ids = rts_string_vec(["1,31", "2,31"]);
    let command_queue_entries = rts_string_vec([
        "history_row_pruned:25:old_queue:17,30:age16",
        "history_row_pruned:24:old_cancel:16,29:age20",
        "history_row:26:queue:18,31:age0",
        "history_row:27:cancel_final:21,25:20,30|22,30:age4",
        "history_row:28:formation_filter_clear:1,31:1,31|2,31:age8",
    ]);
    let stage_fixture = |stage: &str,
                         age_ticks: u8,
                         kind: &str,
                         payload: &str|
     -> RtsControlGroupCommandHistoryStageFixture {
        let lifecycle_event = "control_group_command_feedback_lifecycle:cleared".to_string();
        let history_event = format!("control_group_command_history:{stage}");
        let prune_event = format!("control_group_command_history_prune:{stage}");
        RtsControlGroupCommandHistoryStageFixture {
            stage: stage.to_string(),
            lifecycle_stage: "cleared".to_string(),
            age_ticks,
            action: RtsOrderQueueReplayAction {
                kind: kind.to_string(),
                payload: payload.to_string(),
            },
            input_source: "classic_rts_control_group_command_history_prune_input".to_string(),
            renderer_path: "classic_draw_scene".to_string(),
            preview_surface: "classic_draw_scene_command_history_prune".to_string(),
            control_group_id: "28".to_string(),
            active_control_group_ids: retained_history_group_ids.clone(),
            selected_unit_ids: replay_fixtures.all_member_ids.clone(),
            control_group_assignments: control_group_assignments.clone(),
            minimap_command_tile_id: "1,30".to_string(),
            command_destination_tile: "1,31".to_string(),
            path_tile_ids: path_tile_ids.clone(),
            group_route_tile_ids: group_route_tile_ids.clone(),
            formation_slot_tile_ids: formation_slot_tile_ids.clone(),
            group_command_state:
                "command_feedback_lifecycle:cleared|control_group_command_history_prune:bounded"
                    .to_string(),
            command_queue_entries: command_queue_entries.clone(),
            combat_event_entries: vec![lifecycle_event, history_event, prune_event],
            active_strip_cleared: true,
            history_retained: true,
            history_overflow_row_count: 0,
            stale_pruned_group_visible: false,
            player_tile_x: 18,
            player_tile_y: 30,
        }
    };
    let stages = vec![
        stage_fixture("overflow_input_pruned", 12, "move", "18,31:line"),
        stage_fixture("recent_three_retained", 13, "select-control-group", "28"),
        stage_fixture("cleared_history_bounded", 14, "select-control-group", "26"),
    ];

    RtsControlGroupCommandHistoryFixtures {
        retained_history_group_ids,
        pruned_history_group_ids,
        control_group_assignments,
        ability_command_ids,
        group_26_member_ids: replay_fixtures.group_26_member_ids,
        group_27_member_ids: replay_fixtures.group_27_member_ids,
        group_28_member_ids: replay_fixtures.group_28_member_ids,
        all_member_ids: replay_fixtures.all_member_ids,
        history_entries: replay_fixtures.history_entries,
        pruned_history_entries: replay_fixtures.pruned_history_entries,
        stages,
    }
}

pub fn rts_control_group_command_feedback_rejection_replay_fixtures(
) -> RtsControlGroupCommandFeedbackRejectionReplayFixtures {
    let command_feedback_fixtures = rts_control_group_command_feedback_replay_fixtures();

    RtsControlGroupCommandFeedbackRejectionReplayFixtures {
        retained_history_group_ids: command_feedback_fixtures.retained_history_group_ids.clone(),
        pruned_history_group_ids: command_feedback_fixtures.pruned_history_group_ids.clone(),
        group_26_member_ids: command_feedback_fixtures.group_26_member_ids.clone(),
        all_member_ids: command_feedback_fixtures.all_member_ids.clone(),
        control_group_assignments: rts_string_vec([
            "26:multi0.recall.order.runner|multi0.recall.order.wing",
            "27:multi0.recall.override.runner|multi0.recall.override.wing",
            "28:multi0.recall.formation.runner|multi0.recall.formation.wing",
        ]),
        ability_command_ids: rts_string_vec(["move", "stop", "hold", "patrol"]),
        resource_spend_log: rts_string_vec([
            "commit:1570g:first_minute_command_rejection_resource_pressure",
        ]),
        preserved_command_history_events: rts_string_vec([
            "history_row_pruned:25:old_queue:17,30:age16",
            "history_row_pruned:24:old_cancel:16,29:age20",
            "history_row:26:queue:18,31:age0",
            "history_row:27:cancel_final:21,25:20,30|22,30:age4",
            "history_row:28:formation_filter_clear:1,31:1,31|2,31:age8",
            "control_group_command_feedback_lifecycle:cleared",
            "control_group_command_history:rejection_replay_preserved",
            "control_group_command_history_prune:bounded",
        ]),
        preserved_group_command_state:
            "control_group_command_history:rejection_replay_preserved|control_group_command_history_prune:bounded"
                .to_string(),
        history_entries: command_feedback_fixtures.history_entries,
        pruned_history_entries: command_feedback_fixtures.pruned_history_entries,
        rejection_steps: vec![
            RtsControlGroupCommandFeedbackRejectionStepFixture {
                step_index: 0,
                step_name: "move_without_group_selection".to_string(),
                input_source: "classic_rts_mouse_viewport".to_string(),
                action_label: "RTS:MOVE:18,31:line".to_string(),
                expected_accepted: false,
                expected_reason: "rts_group_selection_required".to_string(),
                preview_stage: Some("group_selection_required".to_string()),
            },
            RtsControlGroupCommandFeedbackRejectionStepFixture {
                step_index: 1,
                step_name: "select_group_26_setup".to_string(),
                input_source: "classic_rts_hotkey".to_string(),
                action_label: "RTS:SELECT:26".to_string(),
                expected_accepted: true,
                expected_reason: "enabled_rts_select_group:26".to_string(),
                preview_stage: None,
            },
            RtsControlGroupCommandFeedbackRejectionStepFixture {
                step_index: 2,
                step_name: "move_invalid_tile_after_selection".to_string(),
                input_source: "classic_rts_mouse_viewport".to_string(),
                action_label: "RTS:MOVE:bad-tile:line".to_string(),
                expected_accepted: false,
                expected_reason: "rts_invalid_tile:bad-tile".to_string(),
                preview_stage: Some("invalid_tile".to_string()),
            },
            RtsControlGroupCommandFeedbackRejectionStepFixture {
                step_index: 3,
                step_name: "attack_without_target".to_string(),
                input_source: "classic_rts_mouse_viewport".to_string(),
                action_label: "RTS:ATTACK:".to_string(),
                expected_accepted: false,
                expected_reason: "rts_attack_target_required".to_string(),
                preview_stage: Some("attack_target_required".to_string()),
            },
            RtsControlGroupCommandFeedbackRejectionStepFixture {
                step_index: 4,
                step_name: "ability_before_attack_target".to_string(),
                input_source: "classic_rts_hotkey".to_string(),
                action_label: "RTS:ABILITY:guard_break".to_string(),
                expected_accepted: false,
                expected_reason: "rts_attack_required_before_ability".to_string(),
                preview_stage: None,
            },
            RtsControlGroupCommandFeedbackRejectionStepFixture {
                step_index: 5,
                step_name: "queue_without_queue_id".to_string(),
                input_source: "classic_rts_mouse_sidebar".to_string(),
                action_label: "RTS:QUEUE:".to_string(),
                expected_accepted: false,
                expected_reason: "rts_queue_id_required".to_string(),
                preview_stage: None,
            },
            RtsControlGroupCommandFeedbackRejectionStepFixture {
                step_index: 6,
                step_name: "queue_unaffordable_build_after_selection".to_string(),
                input_source: "classic_rts_mouse_sidebar".to_string(),
                action_label: "RTS:QUEUE:build:watch_tower@7,4".to_string(),
                expected_accepted: false,
                expected_reason: "rts_queue_unaffordable:build:watch_tower@7,4".to_string(),
                preview_stage: None,
            },
            RtsControlGroupCommandFeedbackRejectionStepFixture {
                step_index: 7,
                step_name: "select_without_group_id".to_string(),
                input_source: "classic_rts_hotkey".to_string(),
                action_label: "RTS:SELECT:".to_string(),
                expected_accepted: false,
                expected_reason: "rts_group_id_required".to_string(),
                preview_stage: Some("history_preserved_after_rejections".to_string()),
            },
        ],
        expected_input_sources: rts_string_vec([
            "classic_rts_mouse_viewport",
            "classic_rts_hotkey",
            "classic_rts_mouse_viewport",
            "classic_rts_mouse_viewport",
            "classic_rts_hotkey",
            "classic_rts_mouse_sidebar",
            "classic_rts_mouse_sidebar",
            "classic_rts_hotkey",
        ]),
        expected_blocked_reasons: rts_string_vec([
            "rts_group_selection_required",
            "rts_invalid_tile:bad-tile",
            "rts_attack_target_required",
            "rts_attack_required_before_ability",
            "rts_queue_id_required",
            "rts_queue_unaffordable:build:watch_tower@7,4",
            "rts_group_id_required",
        ]),
        visual_stages: vec![
            RtsControlGroupCommandFeedbackRejectionVisualStageFixture {
                stage: "group_selection_required".to_string(),
                tile_id: "18,31".to_string(),
                reason: "rts_group_selection_required".to_string(),
                last_feedback: "Input blocked: MAP MOVE LOCK SELECT UNITS".to_string(),
            },
            RtsControlGroupCommandFeedbackRejectionVisualStageFixture {
                stage: "invalid_tile".to_string(),
                tile_id: "3,1".to_string(),
                reason: "rts_invalid_tile:bad-tile".to_string(),
                last_feedback: "Input blocked: MAP MOVE LOCK INVALID TILE".to_string(),
            },
            RtsControlGroupCommandFeedbackRejectionVisualStageFixture {
                stage: "attack_target_required".to_string(),
                tile_id: "21,25".to_string(),
                reason: "rts_attack_target_required".to_string(),
                last_feedback: "Input blocked: MAP ATTACK LOCK PICK TARGET".to_string(),
            },
            RtsControlGroupCommandFeedbackRejectionVisualStageFixture {
                stage: "history_preserved_after_rejections".to_string(),
                tile_id: "1,31".to_string(),
                reason: "recent_three_history_preserved".to_string(),
                last_feedback: "Input blocked: HOTKEY SELECT LOCK GROUP ID".to_string(),
            },
        ],
    }
}

pub fn rts_formation_move_execution_stage(
    combat_events: &[String],
    command_queue: &[String],
    combat_turn: u8,
) -> Option<&'static str> {
    for event in combat_events.iter().rev().chain(command_queue.iter().rev()) {
        if event.contains("formation_move_execution:arrival_lock") {
            return Some("arrival_lock");
        }
        if event.contains("formation_move_execution:blocked_reroute") {
            return Some("blocked_reroute");
        }
        if event.contains("formation_move_execution:crowd_avoidance") {
            return Some("crowd_avoidance");
        }
        if event.contains("formation_move_execution:stagger_step") {
            return Some("stagger_step");
        }
        if event.contains("formation_move_execution:path_reservation") {
            return Some("path_reservation");
        }
        if event.contains("formation_move_execution:slot_claim") {
            return Some("slot_claim");
        }
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("formation_move_execution:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "slot_claim",
        1 => "path_reservation",
        2 => "stagger_step",
        3 => "crowd_avoidance",
        4 => "blocked_reroute",
        _ => "arrival_lock",
    })
}

pub fn rts_local_obstruction_recovery_stage(
    combat_events: &[String],
    command_queue: &[String],
    combat_turn: u8,
) -> Option<&'static str> {
    for event in combat_events.iter().rev().chain(command_queue.iter().rev()) {
        if event.contains("local_obstruction_recovery:flow_resume") {
            return Some("flow_resume");
        }
        if event.contains("local_obstruction_recovery:gap_claim") {
            return Some("gap_claim");
        }
        if event.contains("local_obstruction_recovery:side_step") {
            return Some("side_step");
        }
        if event.contains("local_obstruction_recovery:hold_queue") {
            return Some("hold_queue");
        }
        if event.contains("local_obstruction_recovery:detect_block") {
            return Some("detect_block");
        }
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("local_obstruction_recovery:"))
    {
        return None;
    }
    Some(match combat_turn % 5 {
        0 => "detect_block",
        1 => "hold_queue",
        2 => "side_step",
        3 => "gap_claim",
        _ => "flow_resume",
    })
}

fn rts_unit_model_depth_mark(
    kind: &str,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> RtsUnitModelDepthMark {
    RtsUnitModelDepthMark {
        kind: kind.to_string(),
        rect: RtsRuntimeRect {
            x,
            y,
            width,
            height,
        },
    }
}

pub fn rts_unit_model_depth_marks(frame_id: &str) -> Vec<RtsUnitModelDepthMark> {
    if !(frame_id.starts_with("actor_guard")
        || frame_id.starts_with("actor_worker")
        || frame_id.starts_with("actor_creep"))
    {
        return Vec::new();
    }

    let mut marks = vec![
        rts_unit_model_depth_mark("ground_contact", -14, -3, 28, 2),
        rts_unit_model_depth_mark("rim", -9, -30, 2, 21),
        rts_unit_model_depth_mark("rim", 7, -30, 2, 21),
        rts_unit_model_depth_mark("layer_shadow", -5, -23, 10, 3),
        rts_unit_model_depth_mark("face_shade", -3, -32, 6, 2),
    ];

    if frame_id.starts_with("actor_guard") {
        marks.push(rts_unit_model_depth_mark("armor", -11, -27, 5, 4));
        marks.push(rts_unit_model_depth_mark("armor", 6, -27, 5, 4));
        marks.push(rts_unit_model_depth_mark("role_prop", -4, -38, 8, 3));
    } else if frame_id.starts_with("actor_worker") {
        marks.push(rts_unit_model_depth_mark("role_prop", -15, -26, 5, 13));
        marks.push(rts_unit_model_depth_mark("armor", 9, -25, 5, 12));
        marks.push(rts_unit_model_depth_mark("layer_shadow", -13, -14, 22, 3));
    } else {
        marks.push(rts_unit_model_depth_mark("role_prop", -10, -40, 6, 5));
        marks.push(rts_unit_model_depth_mark("role_prop", 4, -40, 6, 5));
        marks.push(rts_unit_model_depth_mark("armor", -11, -21, 22, 3));
    }

    marks
}

fn rts_action_cadence_mark(
    kind: &str,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> RtsActionCadenceMark {
    RtsActionCadenceMark {
        kind: kind.to_string(),
        rect: RtsRuntimeRect {
            x,
            y,
            width,
            height,
        },
    }
}

pub fn rts_action_cadence_marks(frame_id: &str) -> Vec<RtsActionCadenceMark> {
    let attack_frame = frame_id.ends_with("_attack");
    let carry_frame = frame_id.ends_with("_carry");
    let idle_frame = frame_id.ends_with("_idle");
    let mut marks = Vec::new();

    if attack_frame {
        let windup_left = if frame_id.starts_with("actor_creep") {
            -24
        } else {
            -22
        };
        for step in 0..5 {
            marks.push(rts_action_cadence_mark(
                "windup",
                windup_left + step * 3,
                -36 + step,
                7,
                3,
            ));
        }
        for step in 0..9 {
            marks.push(rts_action_cadence_mark(
                "strike",
                12 + step * 3,
                -34 + step,
                7,
                3,
            ));
        }
        for step in 0..6 {
            marks.push(rts_action_cadence_mark(
                "recovery",
                4 + step * 4,
                -18 + step,
                6,
                3,
            ));
        }
        marks.push(rts_action_cadence_mark("shadow_smear", -15, -7, 32, 3));
        marks.push(rts_action_cadence_mark("shadow_smear", -10, -4, 24, 2));
    } else if carry_frame {
        for step in 0..4 {
            marks.push(rts_action_cadence_mark(
                "carry_bob",
                14 + step * 2,
                -34 - (step % 2),
                4,
                6,
            ));
            marks.push(rts_action_cadence_mark(
                "shadow_smear",
                12 + step * 3,
                -17 + step,
                4,
                3,
            ));
        }
    } else if idle_frame
        && (frame_id.starts_with("actor_guard")
            || frame_id.starts_with("actor_worker")
            || frame_id.starts_with("actor_creep"))
    {
        for step in 0..4 {
            marks.push(rts_action_cadence_mark(
                "idle_breath",
                -10 + step * 6,
                -31 + (step % 2),
                4,
                2,
            ));
        }
    }

    marks
}

pub fn rts_action_sequence_phase(
    frame_id: &str,
    combat_events: &[String],
    command_queue: &[String],
    walk_cycle_frame: u8,
    combat_turn: u8,
    runtime_available: bool,
) -> Option<&'static str> {
    if runtime_available {
        for event in combat_events.iter().rev() {
            if event.contains("sequence:carry_down") {
                return Some("carry_down");
            }
            if event.contains("sequence:carry_up") {
                return Some("carry_up");
            }
            if event.contains("sequence:recovery") {
                return Some("recovery");
            }
            if event.contains("sequence:strike") {
                return Some("strike");
            }
            if event.contains("sequence:windup") {
                return Some("windup");
            }
            if event.contains("sequence:idle") {
                return Some("idle");
            }
        }
        if !command_queue
            .iter()
            .any(|command| command.contains("sequence:"))
        {
            return None;
        }
        if frame_id.contains("carry") {
            return Some(if walk_cycle_frame % 2 == 0 {
                "carry_up"
            } else {
                "carry_down"
            });
        }
        if frame_id.contains("attack") {
            return Some(match combat_turn % 4 {
                1 => "windup",
                2 => "strike",
                3 => "recovery",
                _ => "idle",
            });
        }
    } else if frame_id.contains("carry") {
        return Some("carry_up");
    } else if frame_id.contains("attack") {
        return Some("strike");
    }
    None
}

fn rts_action_sequence_mark(
    kind: &str,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> RtsActionSequenceMark {
    RtsActionSequenceMark {
        kind: kind.to_string(),
        rect: RtsRuntimeRect {
            x,
            y,
            width,
            height,
        },
    }
}

pub fn rts_action_sequence_marks(frame_id: &str, phase: &str) -> Vec<RtsActionSequenceMark> {
    if !(frame_id.starts_with("actor_guard")
        || frame_id.starts_with("actor_worker")
        || frame_id.starts_with("actor_creep"))
    {
        return Vec::new();
    }

    let mut marks = vec![rts_action_sequence_mark("frame_ghost", -16, -7, 32, 2)];

    match phase {
        "windup" => {
            for step in 0..7 {
                marks.push(rts_action_sequence_mark(
                    "windup",
                    -28 + step * 3,
                    -39 + step,
                    7,
                    3,
                ));
            }
            marks.push(rts_action_sequence_mark("windup", -13, -29, 9, 12));
        }
        "strike" => {
            for step in 0..10 {
                marks.push(rts_action_sequence_mark(
                    "strike",
                    8 + step * 3,
                    -38 + step,
                    8,
                    3,
                ));
            }
            marks.push(rts_action_sequence_mark("strike", 24, -28, 12, 10));
        }
        "recovery" => {
            for step in 0..7 {
                marks.push(rts_action_sequence_mark(
                    "recovery",
                    -4 + step * 4,
                    -20 + step,
                    7,
                    3,
                ));
            }
            marks.push(rts_action_sequence_mark("recovery", 6, -32, 8, 16));
        }
        "carry_up" => {
            if frame_id.contains("carry") || frame_id.starts_with("actor_worker") {
                marks.push(rts_action_sequence_mark("carry_up", 11, -39, 16, 6));
                marks.push(rts_action_sequence_mark("carry_up", 15, -31, 10, 6));
            }
        }
        "carry_down" => {
            marks.push(rts_action_sequence_mark("carry_down", -14, -11, 28, 3));
            marks.push(rts_action_sequence_mark("carry_down", -5, -25, 10, 6));
            if frame_id.contains("carry") || frame_id.starts_with("actor_worker") {
                marks.push(rts_action_sequence_mark("carry_down", 10, -28, 18, 7));
                marks.push(rts_action_sequence_mark("carry_down", 7, -18, 20, 4));
            }
        }
        _ => {
            for step in 0..4 {
                marks.push(rts_action_sequence_mark(
                    "idle",
                    -11 + step * 7,
                    -32 + (step % 2),
                    5,
                    2,
                ));
            }
            marks.push(rts_action_sequence_mark("idle", -8, -18, 16, 3));
        }
    }

    marks
}

pub fn rts_npc_behavior_stage(
    combat_events: &[String],
    command_queue: &[String],
    combat_turn: u8,
) -> Option<&'static str> {
    if let Some(stage) = rts_recent_stage_from_events(
        &[
            ("behavior:guard_patrol", "guard_patrol"),
            ("behavior:guard_engage", "guard_engage"),
            ("behavior:worker_work", "worker_work"),
            ("behavior:worker_carry", "worker_carry"),
            ("behavior:creep_stalk", "creep_stalk"),
            ("behavior:creep_retreat", "creep_retreat"),
        ],
        combat_events,
        command_queue,
    ) {
        return Some(stage);
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("behavior:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "guard_patrol",
        1 => "guard_engage",
        2 => "worker_work",
        3 => "worker_carry",
        4 => "creep_stalk",
        _ => "creep_retreat",
    })
}

pub fn rts_combat_impact_stage(
    combat_events: &[String],
    command_queue: &[String],
    combat_turn: u8,
) -> Option<&'static str> {
    if let Some(stage) = rts_recent_stage_from_events(
        &[
            ("impact:victory_settle", "victory_settle"),
            ("impact:corpse_dissolve", "corpse_dissolve"),
            ("impact:death_fall", "death_fall"),
            ("impact:damage_tick", "damage_tick"),
            ("impact:stagger", "stagger"),
            ("impact:hit_flash", "hit_flash"),
        ],
        combat_events,
        command_queue,
    ) {
        return Some(stage);
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("impact:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "hit_flash",
        1 => "stagger",
        2 => "damage_tick",
        3 => "death_fall",
        4 => "corpse_dissolve",
        _ => "victory_settle",
    })
}

pub fn rts_locomotion_blend_stage(
    combat_events: &[String],
    command_queue: &[String],
    walk_cycle_frame: u8,
) -> Option<&'static str> {
    if let Some(stage) = rts_recent_stage_from_events(
        &[
            ("locomotion:arrival_brake", "arrival_brake"),
            ("locomotion:formation_slide", "formation_slide"),
            ("locomotion:turn_arc", "turn_arc"),
            ("locomotion:footstep_right", "footstep_right"),
            ("locomotion:footstep_left", "footstep_left"),
            ("locomotion:path_commit", "path_commit"),
        ],
        combat_events,
        command_queue,
    ) {
        return Some(stage);
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("locomotion:"))
    {
        return None;
    }
    Some(match walk_cycle_frame % 6 {
        0 => "path_commit",
        1 => "footstep_left",
        2 => "footstep_right",
        3 => "turn_arc",
        4 => "formation_slide",
        _ => "arrival_brake",
    })
}

pub fn rts_npc_transition_stage(
    combat_events: &[String],
    command_queue: &[String],
    combat_turn: u8,
) -> Option<&'static str> {
    if let Some(stage) = rts_recent_stage_from_events(
        &[
            ("transition:retreat_resume", "retreat_resume"),
            ("transition:hit_recover", "hit_recover"),
            ("transition:stalk_pounce", "stalk_pounce"),
            ("transition:work_carry", "work_carry"),
            ("transition:patrol_engage", "patrol_engage"),
            ("transition:alert_turn", "alert_turn"),
        ],
        combat_events,
        command_queue,
    ) {
        return Some(stage);
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("transition:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "alert_turn",
        1 => "patrol_engage",
        2 => "work_carry",
        3 => "stalk_pounce",
        4 => "hit_recover",
        _ => "retreat_resume",
    })
}

pub fn rts_depth_readability_stage(
    combat_events: &[String],
    command_queue: &[String],
    combat_turn: u8,
) -> Option<&'static str> {
    if let Some(stage) = rts_recent_stage_from_events(
        &[
            ("depth:terrain_cutaway", "terrain_cutaway"),
            ("depth:path_occlusion", "path_occlusion"),
            ("depth:target_priority", "target_priority"),
            ("depth:building_mask", "building_mask"),
            ("depth:behind_silhouette", "behind_silhouette"),
            ("depth:foreground_canopy", "foreground_canopy"),
        ],
        combat_events,
        command_queue,
    ) {
        return Some(stage);
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("depth:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "foreground_canopy",
        1 => "behind_silhouette",
        2 => "building_mask",
        3 => "target_priority",
        4 => "path_occlusion",
        _ => "terrain_cutaway",
    })
}

pub fn rts_structure_modeling_stage(
    combat_events: &[String],
    command_queue: &[String],
    combat_turn: u8,
) -> Option<&'static str> {
    if let Some(stage) = rts_recent_stage_from_events(
        &[
            ("structure:repair_beam", "repair_beam"),
            ("structure:damage_crack", "damage_crack"),
            ("structure:production_glow", "production_glow"),
            ("structure:construction_spark", "construction_spark"),
            ("structure:scaffold", "scaffold"),
            ("structure:foundation_shadow", "foundation_shadow"),
        ],
        combat_events,
        command_queue,
    ) {
        return Some(stage);
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("structure:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "foundation_shadow",
        1 => "scaffold",
        2 => "construction_spark",
        3 => "production_glow",
        4 => "damage_crack",
        _ => "repair_beam",
    })
}

pub fn rts_environment_life_stage(
    combat_events: &[String],
    command_queue: &[String],
    combat_turn: u8,
) -> Option<&'static str> {
    if let Some(stage) = rts_recent_stage_from_events(
        &[
            ("environment:ambient_dust", "ambient_dust"),
            ("environment:resource_glint", "resource_glint"),
            ("environment:banner_flutter", "banner_flutter"),
            ("environment:water_shimmer", "water_shimmer"),
            ("environment:torch_flicker", "torch_flicker"),
            ("environment:tree_sway", "tree_sway"),
        ],
        combat_events,
        command_queue,
    ) {
        return Some(stage);
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("environment:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "tree_sway",
        1 => "torch_flicker",
        2 => "water_shimmer",
        3 => "banner_flutter",
        4 => "resource_glint",
        _ => "ambient_dust",
    })
}

pub fn rts_worker_harvest_animation_stage(
    combat_events: &[String],
    command_queue: &[String],
    combat_turn: u8,
) -> Option<&'static str> {
    if let Some(stage) = rts_recent_stage_from_events(
        &[
            ("harvest_anim:return_path", "return_path"),
            ("harvest_anim:dropoff_burst", "dropoff_burst"),
            ("harvest_anim:carry_load", "carry_load"),
            ("harvest_anim:resource_pop", "resource_pop"),
            ("harvest_anim:tool_swing", "tool_swing"),
            ("harvest_anim:approach", "approach"),
        ],
        combat_events,
        command_queue,
    ) {
        return Some(stage);
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("harvest_anim:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "approach",
        1 => "tool_swing",
        2 => "resource_pop",
        3 => "carry_load",
        4 => "dropoff_burst",
        _ => "return_path",
    })
}

pub fn rts_production_spawn_animation_stage(
    combat_events: &[String],
    command_queue: &[String],
    combat_turn: u8,
) -> Option<&'static str> {
    if let Some(stage) = rts_recent_stage_from_events(
        &[
            ("production_spawn_anim:supply_flash", "supply_flash"),
            ("production_spawn_anim:formation_join", "formation_join"),
            ("production_spawn_anim:rally_flag", "rally_flag"),
            ("production_spawn_anim:spawn_door", "spawn_door"),
            ("production_spawn_anim:training_tick", "training_tick"),
            ("production_spawn_anim:queue_pulse", "queue_pulse"),
        ],
        combat_events,
        command_queue,
    ) {
        return Some(stage);
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("production_spawn_anim:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "queue_pulse",
        1 => "training_tick",
        2 => "spawn_door",
        3 => "rally_flag",
        4 => "formation_join",
        _ => "supply_flash",
    })
}

pub fn rts_runtime_point_in_rect(mouse_x: i32, mouse_y: i32, rect: RtsRuntimeRect) -> bool {
    mouse_x >= rect.x
        && mouse_x < rect.x + rect.width
        && mouse_y >= rect.y
        && mouse_y < rect.y + rect.height
}

pub fn rts_runtime_grid_slot_rect(
    spec: RtsRuntimeGridSpec,
    index: usize,
) -> Option<RtsRuntimeRect> {
    if spec.count == 0 || spec.columns == 0 || index >= spec.count {
        return None;
    }
    Some(RtsRuntimeRect {
        x: spec.origin_x + (index % spec.columns) as i32 * spec.stride_x,
        y: spec.origin_y + (index / spec.columns) as i32 * spec.stride_y,
        width: spec.slot_width,
        height: spec.slot_height,
    })
}

pub fn rts_runtime_hit_test_grid(
    spec: RtsRuntimeGridSpec,
    mouse_x: i32,
    mouse_y: i32,
) -> Option<usize> {
    (0..spec.count).find(|index| {
        rts_runtime_grid_slot_rect(spec, *index)
            .is_some_and(|rect| rts_runtime_point_in_rect(mouse_x, mouse_y, rect))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_adapter_clamps_focus_and_projects_minimap() {
        let config = rts_scrollable_map_camera_config();
        let start = RtsScrollableMapCameraState::default();
        let step = apply_rts_scrollable_map_camera_input(
            "shift_keyboard_pan",
            start,
            config,
            RtsRuntimeVec2::new(200.0, -120.0),
            0.35,
            None,
        );

        assert_eq!(step.source, "shift_keyboard_pan");
        assert!(step.after.zoom > start.zoom);
        let focus = rts_scrollable_map_camera_focus_tile(step.after);
        assert!(focus.0 >= TRNM_RTS_RUNTIME_MAP_MIN_TILE);
        assert!(focus.1 >= TRNM_RTS_RUNTIME_MAP_MIN_TILE);

        let viewport = rts_camera_minimap_viewport_rect(step.after, 150, 106);
        assert!(viewport.width >= 18);
        assert!(viewport.height >= 14);
        assert!(viewport.x >= 0);
        assert!(viewport.y >= 0);
    }

    #[test]
    fn minimap_grid_and_hit_tests_are_deterministic() {
        assert_eq!(rts_minimap_cell_origin(10, 20, 4, 5, (1, 1)), (10, 20));
        assert_eq!(rts_minimap_cell_origin(10, 20, 4, 5, (32, 32)), (134, 175));
        assert_eq!(rts_large_map_cell_col((32, 10)), 31);
        assert_eq!(rts_large_map_cell_row((8, 32)), 31);

        let spec = RtsRuntimeGridSpec {
            origin_x: 360,
            origin_y: 572,
            columns: 6,
            count: 12,
            stride_x: 58,
            stride_y: 46,
            slot_width: 48,
            slot_height: 38,
        };
        assert_eq!(rts_runtime_hit_test_grid(spec, 363, 575), Some(0));
        assert_eq!(
            rts_runtime_hit_test_grid(spec, 360 + 58 * 5 + 8, 575),
            Some(5)
        );
        assert_eq!(rts_runtime_hit_test_grid(spec, 999, 575), None);
    }

    #[test]
    fn map_projection_and_terrain_seeds_match_first_contact_layout() {
        let projection = rts_runtime_map_projection(RtsRuntimeMapLayoutInput {
            viewport_width: 1280,
            viewport_height: 720,
            map_width_tiles: 34,
            map_height_tiles: 34,
            map_origin_x: 16,
            map_origin_y: 54,
            right_reserved_px: 292,
            bottom_reserved_px: 158,
            min_map_width_px: 374,
            min_map_height_px: 238,
            cell_width_min: 12,
            cell_width_max: 28,
            cell_height_min: 8,
            cell_height_max: 15,
        });

        assert_eq!(
            projection,
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
            rts_runtime_tile_screen_rect(projection, (16, 16)),
            RtsRuntimeRect {
                x: 464,
                y: 278,
                width: 28,
                height: 14,
            }
        );
        assert_eq!(
            rts_runtime_terrain_seeds((16, 16)),
            RtsRuntimeTerrainSeeds {
                surface_seed: 12,
                detail_seed: 20,
            }
        );
    }

    #[test]
    fn tile_line_adapter_matches_first_contact_track_steps() {
        let line = rts_runtime_tile_line((8, 8), (12, 16));

        assert_eq!(line.len(), 9);
        assert_eq!(
            line[0],
            RtsRuntimeTileLineStep {
                step_index: 0,
                step_count: 8,
                tile_x: 8,
                tile_y: 8,
            }
        );
        assert_eq!(
            line[4],
            RtsRuntimeTileLineStep {
                step_index: 4,
                step_count: 8,
                tile_x: 10,
                tile_y: 12,
            }
        );
        assert_eq!(
            line[8],
            RtsRuntimeTileLineStep {
                step_index: 8,
                step_count: 8,
                tile_x: 12,
                tile_y: 16,
            }
        );
        assert_eq!(
            rts_runtime_tile_line((5, 5), (5, 5)),
            vec![RtsRuntimeTileLineStep {
                step_index: 0,
                step_count: 0,
                tile_x: 5,
                tile_y: 5,
            }]
        );
    }

    #[test]
    fn path_preview_adapter_preserves_command_semantics() {
        assert_eq!(
            rts_move_follow_target("follow:worker_alpha"),
            Some("worker_alpha")
        );
        assert_eq!(rts_move_formation_kind("follow:worker_alpha"), "follow");
        assert_eq!(
            rts_path_tiles_for_destination((8, 4)),
            vec!["6,5", "7,5", "8,4"]
        );
        assert_eq!(rts_blocked_tiles_for_destination((8, 4)), vec!["7,4"]);
        assert_eq!(
            rts_formation_slots_for_destination((8, 4), "rally"),
            vec!["7,5", "8,4", "9,4", "8,5"]
        );
        assert_eq!(
            rts_disperse_slots_for_destination((6, 5)),
            vec!["5,5", "6,4", "6,6", "7,5"]
        );

        let command_queue = vec!["command_queue_path_preview:shift_waypoints".to_string()];
        assert_eq!(
            rts_command_queue_path_preview_stage(&[], &command_queue, 5),
            Some("shift_waypoints")
        );
        assert_eq!(
            rts_command_queue_path_preview_stage(&[], &["other".to_string()], 0),
            None
        );

        let fixtures = rts_command_queue_path_preview_stage_fixtures();
        assert_eq!(
            fixtures
                .iter()
                .map(|fixture| fixture.stage.as_str())
                .collect::<Vec<_>>(),
            vec![
                "queue_stack",
                "shift_waypoints",
                "rally_chain",
                "attack_focus",
                "build_reservation",
                "cancel_repath"
            ]
        );
        assert_eq!(
            fixtures
                .iter()
                .map(|fixture| fixture.action.kind.as_str())
                .collect::<Vec<_>>(),
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
            fixtures
                .last()
                .map(|fixture| fixture.history_entry.as_str()),
            Some("command_queue_path_preview:cancel_repath")
        );
    }

    #[test]
    fn combat_target_adapter_preserves_first_contact_semantics() {
        assert_eq!(
            rts_engagement_tiles_for_target("enemy_barracks"),
            vec!["9,3", "10,3", "10,2", "11,2"]
        );
        assert_eq!(
            rts_contact_flash_tiles_for_target("arena_creep_attack"),
            vec!["6,5", "6,4"]
        );
        assert_eq!(rts_target_tile_for_id("forest_shaman_support", 0), (9, 3));
        assert_eq!(
            rts_target_priority_ids_for_target("arena_creep_attack"),
            vec![
                "arena_creep_attack",
                "arena_guard_support",
                "arena_worker_support"
            ]
        );
        assert_eq!(
            rts_projectile_trail_tiles_for_target("forest_creep_camp"),
            vec!["5,5", "6,5", "7,4", "8,3"]
        );
        assert_eq!(
            rts_ability_effect_tiles_for_target("enemy_barracks", "guard_break"),
            vec!["10,3", "10,2", "11,2", "9,3"]
        );
        assert_eq!(
            rts_threat_levels_for_target("enemy_barracks"),
            vec![88, 66, 41]
        );
        assert_eq!(
            rts_damage_ticks_for_ability("guard_break"),
            vec![16, 21, 35]
        );
        assert_eq!(
            rts_projectile_id_for_ability("guard_break"),
            "guard_break_bolt"
        );
    }

    #[test]
    fn ai_pressure_adapter_preserves_first_contact_routes() {
        assert_eq!(
            rts_ai_wave_unit_ids_for_pressure("skirmish_wave"),
            vec!["lane_scout", "mirror_raider", "siege_runner"]
        );
        assert_eq!(
            rts_ai_pressure_tiles_for_pressure("skirmish_wave"),
            vec!["9,3", "8,4", "7,4", "6,5"]
        );
        assert_eq!(
            rts_ai_counter_tiles_for_pressure("skirmish_wave"),
            vec!["5,5", "6,5", "6,4", "7,5"]
        );
        assert_eq!(
            rts_enemy_pressure_wave_units_for_id("raider_wave"),
            vec!["enemy_raider", "enemy_signal_guard", "enemy_sapper"]
        );
        assert_eq!(
            rts_enemy_pressure_lane_tiles_for_wave("raider_wave"),
            vec!["10,2", "9,3", "8,4", "7,4", "6,5"]
        );
    }

    #[test]
    fn recon_intel_adapter_preserves_first_contact_routes() {
        assert_eq!(
            rts_scout_route_tiles_for_recon("enemy_base"),
            vec!["5,5", "6,4", "7,4", "8,3", "9,2", "10,2"]
        );
        assert_eq!(
            rts_fog_reveal_tiles_for_recon("enemy_base", "mark"),
            vec!["7,4", "8,3", "8,2", "9,2", "9,3", "10,2", "10,3", "11,1", "11,2"]
        );
        assert_eq!(
            rts_enemy_structures_for_recon("enemy_base", "mark"),
            vec!["enemy_watch_post", "enemy_barracks", "enemy_resource_vault"]
        );
        assert_eq!(
            rts_enemy_units_for_recon("enemy_base", "mark"),
            vec!["enemy_scout", "enemy_worker", "enemy_guard"]
        );
        assert_eq!(
            rts_enemy_structure_tile_for_id("enemy_resource_vault", 2),
            (11, 2)
        );
        assert_eq!(rts_enemy_unit_tile_for_id("enemy_guard", 2), (11, 2));
    }

    #[test]
    fn base_assault_and_aftermath_adapter_preserves_first_contact_routes() {
        assert_eq!(
            rts_base_assault_path_tiles_for_target("enemy_barracks", "10,3"),
            vec!["5,5", "6,5", "7,4", "8,4", "9,3", "10,3"]
        );
        assert_eq!(
            rts_base_assault_targets_for_id("enemy_barracks"),
            vec!["enemy_watch_post", "enemy_barracks", "enemy_resource_vault"]
        );
        assert_eq!(
            rts_aftermath_debris_tiles_for_id("enemy_barracks", "10,3"),
            vec!["9,3", "10,3", "10,4", "11,3"]
        );
        assert_eq!(
            rts_aftermath_smoke_tiles_for_id("enemy_barracks", "10,3"),
            vec!["10,2", "10,3", "11,3"]
        );
    }

    #[test]
    fn commander_and_expansion_counterattack_adapter_preserves_first_contact_routes() {
        assert_eq!(
            rts_commander_aura_tiles_for_id("mirror_captain"),
            vec!["6,5", "7,4", "8,4", "9,3", "10,3"]
        );
        assert_eq!(
            rts_loot_items_for_id("enemy_barracks"),
            vec![
                "barracks_map_cache",
                "field_banner_relic",
                "repair_kit_crate"
            ]
        );
        assert_eq!(
            rts_expansion_tiles_for_id("forest_relay", "9,2"),
            vec!["8,2", "9,2", "10,2", "9,3", "10,3"]
        );
        assert_eq!(rts_expansion_structure_tile_for_id("watch_lantern"), (8, 3));
        assert_eq!(
            rts_expansion_workers_for_line("gold_line"),
            vec![
                "expansion_worker_alpha",
                "expansion_worker_beta",
                "expansion_worker_gamma"
            ]
        );
        assert_eq!(
            rts_counterattack_units_for_wave("counter_wave"),
            vec![
                "counter_raider_alpha",
                "counter_raider_beta",
                "counter_sapper"
            ]
        );
        assert_eq!(
            rts_counterattack_route_tiles_for_wave("counter_wave", "8,3"),
            vec!["11,2", "10,2", "9,3", "8,3", "7,4", "9,2"]
        );
    }

    #[test]
    fn army_production_rally_adapter_preserves_first_contact_routes() {
        assert_eq!(
            rts_army_units_for_batch("mixed_vanguard"),
            vec![
                "relay_guard_alpha",
                "relay_guard_beta",
                "wayfinder_scout",
                "field_mender"
            ]
        );
        assert_eq!(
            rts_army_rally_tiles_for_id("forward_watch"),
            vec!["5,5", "6,5", "7,4", "8,4", "8,3"]
        );
        assert_eq!(rts_player_army_unit_tile_for_id("field_mender", 3), (6, 4));
        assert_eq!(rts_player_army_unit_tile_for_id("custom_guard", 4), (7, 4));
    }

    #[test]
    fn objective_and_terrain_route_adapter_preserves_first_contact_tiles() {
        assert_eq!(
            rts_objective_tiles_for_id("relay_beacon", "6,5"),
            vec!["6,5", "6,4", "7,5", "9,2"]
        );
        assert_eq!(
            rts_creep_camp_tiles_for_id("forest_creep_camp", "8,3"),
            vec!["8,3", "8,2", "9,3", "9,2"]
        );
        assert_eq!(
            rts_terrain_route_tiles_for_camp("forest_creep_camp"),
            vec!["5,5", "6,5", "7,4", "8,3"]
        );
        assert_eq!(
            rts_terrain_choke_tiles_for_camp("forest_creep_camp"),
            vec!["7,4", "7,3", "8,4"]
        );
        assert_eq!(
            rts_expansion_tiles_for_camp("forest_creep_camp"),
            vec!["9,2", "10,2", "10,3"]
        );
    }

    #[test]
    fn siege_and_inner_lane_adapter_preserves_first_contact_routes() {
        assert_eq!(
            rts_siege_units_for_id("stonebreak_cart"),
            vec!["stonebreak_cart"]
        );
        assert_eq!(
            rts_siege_push_route_tiles_for_target("gate_bulwark", "10,3"),
            vec!["9,2", "9,3", "10,3", "10,2", "11,2", "10,3"]
        );
        assert_eq!(
            rts_siege_breach_tiles_for_target("gate_bulwark", "10,3"),
            vec!["9,3", "10,3", "10,2", "11,2", "10,3"]
        );
        assert_eq!(rts_enemy_fortification_tile_for_id("gate_bulwark"), (10, 3));
        assert_eq!(
            rts_enemy_repair_units_for_target("gate_bulwark"),
            vec!["repair_adept_alpha", "repair_adept_beta"]
        );
        assert_eq!(
            rts_enemy_flank_units_for_id("ridge_sentries"),
            vec!["ridge_sentry_left", "ridge_sentry_right", "ridge_sapper"]
        );
        assert_eq!(rts_enemy_flank_tile_for_index(2), (8, 4));
        assert_eq!(
            rts_player_hold_tiles_for_id("shield_line", "9,3"),
            vec!["8,3", "9,3", "9,4", "10,3"]
        );
        assert_eq!(
            rts_inner_lane_tiles_for_id("inner_lane", "11,2"),
            vec!["10,3", "11,2", "11,3", "12,3", "12,4"]
        );
        assert_eq!(rts_inner_gate_tile_for_id("inner_latch"), (11, 3));
        assert_eq!(rts_inner_gate_tile_for_id("signal_lock"), (12, 3));
        assert_eq!(
            rts_inner_defenders_for_id("second_line"),
            vec!["inner_guard_alpha", "inner_guard_beta", "signal_lancer"]
        );
        assert_eq!(
            rts_supply_convoy_for_id("relay_convoy"),
            vec!["convoy_cart", "field_medic", "ammo_runner"]
        );
        assert_eq!(
            rts_split_squad_tiles_for_id("flank_team", "10,4"),
            vec!["10,4", "11,4", "12,4", "12,3"]
        );
        assert_eq!(rts_inner_core_tile_for_id("signal_core"), (12, 3));
    }

    #[test]
    fn central_keep_adapter_preserves_first_contact_routes() {
        assert_eq!(
            rts_central_keep_route_tiles_for_id("central_keep", "13,3"),
            vec!["12,3", "12,4", "13,4", "13,3", "14,3"]
        );
        assert_eq!(rts_central_keep_tile_for_id("central_keep"), (13, 3));
        assert_eq!(
            rts_boss_guard_units_for_id("warden_line"),
            vec!["keep_warden_alpha", "keep_warden_beta", "ward_sentinel"]
        );
        assert_eq!(
            rts_player_siege_line_tiles_for_id("final_line", "12,4"),
            vec!["11,4", "12,4", "13,4", "12,3"]
        );
        assert_eq!(
            rts_keep_breach_tiles_for_id("central_keep", "13,3"),
            vec!["13,3", "13,4", "14,3", "14,4"]
        );
        assert_eq!(
            rts_guardian_counter_units_for_id("high_warden"),
            vec!["high_warden", "ward_lancer", "last_mirror_guard"]
        );
        assert_eq!(
            rts_keep_claim_tiles_for_id("central_keep", "13,3"),
            vec!["12,3", "13,3", "14,3", "13,4"]
        );
    }

    #[test]
    fn restoration_open_world_adapter_preserves_first_contact_routes() {
        assert_eq!(
            rts_restored_zones_for_id("mirror_city"),
            vec!["central_keep", "signal_core", "inner_lane", "forest_relay"]
        );
        assert_eq!(
            rts_rebuild_structures_for_id("signal_core"),
            vec!["signal_core", "inner_latch", "mirror_ward"]
        );
        assert_eq!(
            rts_garrison_units_for_id("central_keep"),
            vec!["mirror_guard_alpha", "signal_lancer", "field_engineer"]
        );
        assert_eq!(
            rts_open_world_route_tiles_for_id("league-coliseum"),
            vec!["13,3", "12,3", "11,3", "10,2", "9,2"]
        );
        assert_eq!(
            rts_open_world_panels_for_room("league-coliseum"),
            vec![
                "room_panel:league-coliseum",
                "task_panel:task-fixture-first-route",
                "combat_panel:league-coliseum",
                "save_panel:post_rts_restore"
            ]
        );
    }

    #[test]
    fn economy_tech_placement_adapter_preserves_first_contact_tiles() {
        assert_eq!(rts_siege_unit_tile_for_id("stonebreak_cart", 0), (9, 3));
        assert_eq!(rts_harvest_tile_for_node("gold_vein"), (3, 3));
        assert_eq!(rts_dropoff_tile_for_structure("town_hall"), (5, 5));
        assert_eq!(rts_build_site_tiles("7,4"), vec!["7,4", "7,5", "8,4"]);
        assert_eq!(rts_structure_tile_for_id("training_hall"), (4, 3));
        assert_eq!(rts_unlock_unit_tile_for_id("relay_guard"), (7, 5));
    }

    #[test]
    fn queue_economy_adapter_preserves_first_contact_rules() {
        let resource_spend_log = vec!["commit:1200g:prior_queue_pressure".to_string()];
        assert_eq!(rts_queue_gold_cost("build:watch_tower@7,4"), 210);
        assert_eq!(rts_queue_cost_label("harvest:gold_vein"), "-");
        assert_eq!(
            rts_log_gold_amount("commit:210g:build:watch_tower@7,4"),
            210
        );
        assert_eq!(rts_resource_gold_commitment(&resource_spend_log), 1200);
        assert_eq!(rts_available_gold(0, &resource_spend_log), 40);
        assert!(!rts_queue_is_affordable(
            0,
            &resource_spend_log,
            "build:watch_tower@7,4"
        ));
        assert!(rts_queue_requires_affordability_check(
            "build:watch_tower@7,4"
        ));
        assert!(!rts_queue_requires_affordability_check(
            "objective:claim_relay"
        ));
        let command_slot_ids = vec!["move".to_string(), "stop".to_string(), "attack".to_string()];
        let production_queue = vec![
            "train:worker".to_string(),
            "upgrade:signal_blade".to_string(),
        ];
        let build_queue = vec!["build:watch_tower@7,4".to_string()];
        assert_eq!(
            rts_command_slot_id_for_index(&[], Some(&command_slot_ids), "hold", 2),
            "attack"
        );
        assert_eq!(
            rts_build_palette_queue_id_for_slot(None, 3),
            "build:watch_tower@7,4"
        );
        assert_eq!(
            rts_production_slot_queue_id(
                &production_queue,
                &build_queue,
                "train:guard",
                "build:training_hall@4,3",
                2,
            ),
            "build:watch_tower@7,4"
        );
        assert_eq!(
            rts_sidebar_cancel_queue_id(&production_queue, &build_queue, 2).as_deref(),
            Some("cancel:build:0")
        );
        assert_eq!(
            rts_palette_cancel_queue_id(&[], &[], Some("refinery"), "build:refinery@6,4")
                .as_deref(),
            Some("cancel:active_build")
        );
        assert_eq!(
            rts_sidebar_slot_status_label(&production_queue, &build_queue, true, 2, 66),
            "B1 66 R"
        );
        assert_eq!(
            rts_sidebar_slot_status_label(&[], &[], true, 0, 0),
            "ADD UNIT"
        );
        assert_eq!(
            rts_sidebar_slot_status_label(&[], &[], true, 2, 0),
            "ADD BUILD"
        );
        assert_eq!(rts_sidebar_slot_status_label(&[], &[], false, 2, 0), "LOCK");
        assert_eq!(
            rts_palette_state_label(Some("refinery"), &[], &[], true, "build:refinery@6,4"),
            "ACT"
        );
        assert_eq!(rts_queue_item_player_label("train:worker"), "WORKER");
        assert_eq!(
            rts_queue_item_player_label("upgrade:signal_blade"),
            "SIGNAL"
        );
        assert_eq!(
            rts_queue_item_player_label("build:watch_tower@7,4"),
            "TOWER"
        );
        assert_eq!(
            rts_sidebar_queue_summary(&production_queue, &build_queue, 42, 66),
            "WORKER 42% TOWER 66%"
        );
        assert_eq!(
            rts_spawned_unit_id_from_queue("train:worker", 2),
            "worker_3"
        );
        assert_eq!(
            rts_structure_id_from_queue("build:watch_tower@7,4"),
            "watch_tower"
        );
        assert_eq!(
            rts_build_parts("build:watch_tower@7,4"),
            ("watch_tower".to_string(), "7,4".to_string())
        );
        assert_eq!(
            rts_structure_parts("repair:watch_tower@7,4", "repair:", "7,4"),
            ("watch_tower".to_string(), "7,4".to_string())
        );
        assert_eq!(
            rts_tech_parts("upgrade:signal_blade", "upgrade:", "training_hall"),
            ("signal_blade".to_string(), "training_hall".to_string())
        );
        assert!(rts_queue_uses_production_lane("train:worker"));
        assert!(!rts_queue_uses_production_lane("build:watch_tower@7,4"));
        assert_eq!(
            rts_queue_feedback_chip("build:watch_tower@7,4"),
            "feedback:build_placed:watch_tower@7,4"
        );
        assert_eq!(
            rts_rejection_feedback_chip("RTS:QUEUE:build:watch_tower@7,4", "low_gold"),
            "feedback:blocked:queue:low_gold"
        );
        assert_eq!(
            rts_input_source_player_label("classic_rts_mouse_sidebar", "RTS:QUEUE:train:worker"),
            "SIDEBAR"
        );
        assert_eq!(
            rts_blocked_feedback_toast(
                "classic_rts_mouse_sidebar",
                "RTS:QUEUE:build:watch_tower@7,4",
                "rts_queue_unaffordable:build:watch_tower@7,4"
            ),
            "Input blocked: SIDEBAR QUEUE LOCK NEED 210G"
        );
        assert!(rts_should_emit_rejection_feedback_chip(
            "classic_rts_mouse_sidebar"
        ));
        assert!(!rts_should_emit_rejection_feedback_chip(
            "classic_rts_bot_executor"
        ));
        assert_eq!(
            rts_executable_command_queue_snapshot(&[
                "queue:train:worker".to_string(),
                "feedback:blocked:queue:rts_queue_unaffordable:build:watch_tower@7,4".to_string(),
            ]),
            vec!["queue:train:worker"]
        );
        assert!(rts_blocked_feedback_chip_visible(&[
            "queue:train:worker".to_string(),
            "feedback:blocked:queue:rts_queue_unaffordable:build:watch_tower@7,4".to_string(),
        ]));
        assert!(!rts_blocked_feedback_chip_visible(&[
            "queue:train:worker".to_string()
        ]));
        assert_eq!(
            rts_blocked_feedback_player_label(
                "feedback:blocked:queue:rts_queue_unaffordable:build:watch_tower@7,4"
            ),
            "QUEUE LOCK NEED 210G"
        );
    }

    #[test]
    fn command_feedback_adapter_preserves_first_contact_lifecycle() {
        let strip_queue = vec![
            "queued_group_order:Multi0:26:move:2actors".to_string(),
            "control_group_command_feedback_strip:group_27_override".to_string(),
        ];
        let strip_events =
            vec!["control_group_command_feedback_strip:group_28_filtered".to_string()];
        assert_eq!(
            rts_command_feedback_strip_stage(0, &strip_events, &strip_queue),
            Some("group_28_filtered")
        );
        assert_eq!(
            rts_command_surface_stage(
                0,
                &[
                    "surface:selection_state".to_string(),
                    "surface:target_queue".to_string(),
                ],
                &["surface:command_grid".to_string()]
            ),
            Some("target_queue")
        );
        assert_eq!(
            rts_command_surface_stage(2, &[], &["surface:command_grid".to_string()]),
            Some("cooldown_disabled")
        );
        assert_eq!(rts_command_surface_stage(1, &[], &[]), None);
        assert_eq!(
            rts_command_feedback_strip_stage(
                2,
                &[],
                &["control_group_command_feedback_strip:".into()]
            ),
            Some("group_28_formation")
        );
        assert_eq!(rts_command_feedback_strip_stage(1, &[], &[]), None);

        let lifecycle_events = vec!["control_group_command_feedback_lifecycle:dimmed".to_string()];
        let lifecycle_queue = vec![
            "control_group_command_history:dimmed_history_retained".to_string(),
            "history_row_pruned:25:old_queue:17,30:age16".to_string(),
        ];
        assert_eq!(
            rts_command_feedback_lifecycle_stage(
                "command_feedback_lifecycle:fresh",
                &lifecycle_events,
                &lifecycle_queue,
            ),
            Some("fresh")
        );
        assert_eq!(
            rts_command_feedback_lifecycle_stage("", &lifecycle_events, &lifecycle_queue),
            Some("dimmed")
        );
        assert!(rts_command_history_visible(
            "",
            &lifecycle_events,
            &lifecycle_queue,
        ));
        assert!(rts_command_history_prune_visible(
            "",
            &lifecycle_events,
            &lifecycle_queue,
        ));
        let strip_fixtures = rts_control_group_command_feedback_strip_fixtures();
        assert_eq!(strip_fixtures.stages.len(), 4);
        assert_eq!(
            strip_fixtures
                .stages
                .iter()
                .map(|fixture| fixture.stage.as_str())
                .collect::<Vec<_>>(),
            vec![
                "group_26_queued",
                "group_27_override",
                "group_28_formation",
                "group_28_filtered"
            ]
        );
        assert_eq!(
            strip_fixtures.stages[1].override_final_tile_ids,
            vec!["20,30", "22,30"]
        );
        assert_eq!(
            strip_fixtures.stages[3].filtered_member_ids,
            vec![
                "missing:multi0.recall.formation.missing",
                "foreign:map.actor1"
            ]
        );
        assert!(strip_fixtures.stages[3]
            .command_queue_entries
            .iter()
            .any(|entry| entry == "filtered_member:old:multi0.recall.formation.old.wing"));
        let lifecycle_fixtures = rts_control_group_command_feedback_lifecycle_fixtures();
        assert_eq!(
            lifecycle_fixtures
                .stages
                .iter()
                .map(|fixture| fixture.stage.as_str())
                .collect::<Vec<_>>(),
            vec!["fresh", "dimmed", "cleared"]
        );
        assert_eq!(
            lifecycle_fixtures
                .stages
                .iter()
                .map(|fixture| fixture.action.payload.as_str())
                .collect::<Vec<_>>(),
            vec!["18,31:line", "1,31:line", "28"]
        );
        assert_eq!(
            lifecycle_fixtures
                .stages
                .iter()
                .map(|fixture| fixture.age_ticks)
                .collect::<Vec<_>>(),
            vec![0, 4, 8]
        );
        assert_eq!(
            lifecycle_fixtures
                .stages
                .iter()
                .map(|fixture| fixture.lifecycle_event.as_str())
                .collect::<Vec<_>>(),
            vec![
                "control_group_command_feedback_lifecycle:fresh",
                "control_group_command_feedback_lifecycle:dimmed",
                "control_group_command_feedback_lifecycle:cleared"
            ]
        );
        assert_eq!(
            lifecycle_fixtures.group_28_formation_slot_tile_ids,
            vec!["1,31", "2,31"]
        );
        assert!(lifecycle_fixtures
            .all_member_ids
            .iter()
            .any(|member| member == "multi0.recall.override.wing"));
        let replay_fixtures = rts_control_group_command_feedback_replay_fixtures();
        assert_eq!(
            replay_fixtures.retained_history_group_ids,
            vec!["26", "27", "28"]
        );
        assert_eq!(replay_fixtures.pruned_history_group_ids, vec!["25", "24"]);
        assert_eq!(replay_fixtures.command_steps.len(), 7);
        assert_eq!(
            replay_fixtures.command_steps[1].preview_stage.as_deref(),
            Some("group_26_queued")
        );
        assert_eq!(
            replay_fixtures.history_entries[2].formation_slot_tile_ids,
            vec!["1,31", "2,31"]
        );
        assert_eq!(
            replay_fixtures.pruned_history_entries[0]
                .prune_reason
                .as_deref(),
            Some("recent_three_capacity")
        );
        let history_fixtures = rts_control_group_command_history_fixtures();
        assert_eq!(
            history_fixtures
                .stages
                .iter()
                .map(|stage| stage.stage.as_str())
                .collect::<Vec<_>>(),
            vec![
                "fresh_history_appended",
                "dimmed_history_retained",
                "cleared_history_retained"
            ]
        );
        assert_eq!(
            history_fixtures
                .stages
                .iter()
                .map(|stage| stage.lifecycle_stage.as_str())
                .collect::<Vec<_>>(),
            vec!["fresh", "dimmed", "cleared"]
        );
        assert_eq!(
            history_fixtures.retained_history_group_ids,
            vec!["26", "27", "28"]
        );
        assert!(history_fixtures
            .stages
            .iter()
            .all(
                |stage| stage.input_source == "classic_rts_control_group_command_history_input"
                    && stage.renderer_path == "classic_draw_scene"
            ));
        let history_prune_fixtures = rts_control_group_command_history_prune_fixtures();
        assert_eq!(
            history_prune_fixtures
                .stages
                .iter()
                .map(|stage| stage.stage.as_str())
                .collect::<Vec<_>>(),
            vec![
                "overflow_input_pruned",
                "recent_three_retained",
                "cleared_history_bounded"
            ]
        );
        assert_eq!(
            history_prune_fixtures.pruned_history_group_ids,
            vec!["25", "24"]
        );
        assert_eq!(history_prune_fixtures.pruned_history_entries.len(), 2);
        assert!(history_prune_fixtures
            .stages
            .iter()
            .all(|stage| stage.active_strip_cleared
                && stage.input_source == "classic_rts_control_group_command_history_prune_input"
                && stage
                    .command_queue_entries
                    .iter()
                    .any(|entry| entry == "history_row_pruned:25:old_queue:17,30:age16")));
        let rejection_fixtures = rts_control_group_command_feedback_rejection_replay_fixtures();
        assert_eq!(rejection_fixtures.rejection_steps.len(), 8);
        assert_eq!(
            rejection_fixtures
                .rejection_steps
                .iter()
                .map(|step| step.step_name.as_str())
                .collect::<Vec<_>>(),
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
            rejection_fixtures.expected_blocked_reasons,
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
            rejection_fixtures
                .visual_stages
                .iter()
                .map(|stage| stage.stage.as_str())
                .collect::<Vec<_>>(),
            vec![
                "group_selection_required",
                "invalid_tile",
                "attack_target_required",
                "history_preserved_after_rejections"
            ]
        );
        assert_eq!(
            rejection_fixtures.preserved_command_history_events.last(),
            Some(&"control_group_command_history_prune:bounded".to_string())
        );
        assert_eq!(
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
            Some("move")
        );
        assert_eq!(
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
            Some("follow")
        );
        assert_eq!(
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
            Some("attack")
        );
        assert_eq!(
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
            Some("harvest")
        );
        let command_queue = vec!["harvest:gold_vein->town_hall".to_string()];
        let harvest_nodes = vec!["gold_vein".to_string()];
        assert_eq!(
            rts_command_execution_target_label(
                "attack",
                Some("arena_creep_attack"),
                "idle",
                "",
                &[],
                &[],
                Some("8,4"),
            ),
            "arena_creep_attack"
        );
        assert_eq!(
            rts_command_execution_target_label(
                "follow",
                None,
                "following:player",
                "follow:square_guard_patrol@5,5",
                &[],
                &[],
                None,
            ),
            "player"
        );
        assert_eq!(
            rts_command_execution_target_label(
                "harvest",
                None,
                "idle",
                "",
                &[],
                &command_queue,
                None,
            ),
            "gold_vein"
        );
        assert_eq!(
            rts_command_execution_player_label("harvest", "gold_vein", Some("town_hall")),
            "HARVEST GOLD VEIN TO TOWN HALL"
        );
        assert_eq!(
            rts_command_execution_target_tile(
                "attack",
                Some("arena_creep_attack"),
                "idle",
                "",
                &[],
                &[],
                Some("8,4"),
            ),
            Some((6, 5))
        );
        assert_eq!(
            rts_command_execution_target_tile(
                "follow",
                None,
                "following:player",
                "",
                &[],
                &[],
                None,
            ),
            Some((5, 4))
        );
        assert_eq!(
            rts_command_execution_target_tile(
                "harvest",
                None,
                "idle",
                "",
                &harvest_nodes,
                &command_queue,
                None,
            ),
            Some((3, 3))
        );
    }

    #[test]
    fn overlay_stage_adapter_preserves_first_contact_feedback_states() {
        let unit_events = vec!["unit_status_portrait:commander".to_string()];
        assert_eq!(
            rts_unit_status_portrait_stage(4, &unit_events, &["unit_status_portrait:".to_string()],),
            Some("commander")
        );
        assert_eq!(
            rts_unit_status_portrait_stage(5, &[], &["unit_status_portrait:".to_string()]),
            Some("multi_select")
        );
        assert_eq!(rts_unit_status_portrait_stage(0, &[], &[]), None);
        assert_eq!(
            rts_unit_status_portrait_unit_id(
                "worker",
                &[
                    "player".to_string(),
                    "square_worker_carry".to_string(),
                    "square_guard_patrol".to_string(),
                ],
                Some("mirror_captain"),
                Some("arena_creep_attack"),
                &["training_hall".to_string()],
            ),
            "square_worker_carry"
        );
        assert_eq!(
            rts_unit_status_portrait_unit_id(
                "structure",
                &[],
                None,
                None,
                &["relay_outpost".to_string()],
            ),
            "relay_outpost"
        );
        assert_eq!(
            rts_unit_status_health_percent("structure", &[], &[76], 41),
            76
        );
        assert_eq!(
            rts_unit_status_health_percent("creep_target", &[], &[], 0),
            1
        );
        assert_eq!(rts_unit_status_energy_percent(&[32]), 68);
        assert_eq!(
            rts_unit_status_role_badges("commander"),
            ["AUR", "LVL", "CMD"]
        );

        assert_eq!(
            rts_selection_command_feedback_stage(
                0,
                &[],
                &["selection_command_feedback:attack_lock".to_string()],
            ),
            Some("attack_lock")
        );
        assert_eq!(
            rts_selection_command_feedback_stage(
                3,
                &[],
                &["selection_command_feedback:".to_string()],
            ),
            Some("move_line")
        );
        assert_eq!(rts_selection_command_feedback_stage(0, &[], &[]), None);

        assert_eq!(
            rts_ability_tooltip_telegraph_stage(
                0,
                &["ability_tooltip_telegraph:range_preview".to_string()],
                &[],
            ),
            Some("range_preview")
        );
        assert_eq!(
            rts_ability_tooltip_telegraph_stage(
                4,
                &[],
                &["ability_tooltip_telegraph:".to_string()],
            ),
            Some("queue_explain")
        );
        assert_eq!(rts_ability_tooltip_telegraph_stage(0, &[], &[]), None);

        assert_eq!(
            rts_control_group_hotkey_feedback_stage(
                0,
                &[],
                &["control_group_hotkey_feedback:double_tap_camera".to_string()],
            ),
            Some("double_tap_camera")
        );
        assert_eq!(
            rts_control_group_hotkey_feedback_stage(
                5,
                &[],
                &["control_group_hotkey_feedback:".to_string()],
            ),
            Some("ability_hotkey_ack")
        );
        assert_eq!(rts_control_group_hotkey_feedback_stage(0, &[], &[]), None);

        assert_eq!(
            rts_formation_move_preview_stage(
                &["formation_move_preview:commit_spacing".to_string()],
                &["formation_move_preview:destination_ghost".to_string()],
                0,
            ),
            Some("commit_spacing")
        );
        assert_eq!(
            rts_formation_move_preview_stage(&[], &["formation_move_preview:".to_string()], 3,),
            Some("collision_avoidance")
        );
        assert_eq!(rts_formation_move_preview_stage(&[], &[], 0), None);

        let formation_preview_fixtures = rts_formation_move_preview_stage_fixtures();
        assert_eq!(
            formation_preview_fixtures
                .iter()
                .map(|fixture| fixture.stage.as_str())
                .collect::<Vec<_>>(),
            vec![
                "destination_ghost",
                "wedge_spacing",
                "line_reflow",
                "collision_avoidance",
                "split_avoidance",
                "commit_spacing"
            ]
        );
        assert_eq!(
            formation_preview_fixtures
                .iter()
                .map(|fixture| fixture.action.payload.as_str())
                .collect::<Vec<_>>(),
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
            formation_preview_fixtures
                .first()
                .and_then(|fixture| fixture.command_destination_tile.as_deref()),
            Some("8,4")
        );
        assert_eq!(
            formation_preview_fixtures[0].formation_slot_tile_ids,
            vec!["8,4", "7,4", "8,5", "9,4"]
        );
        assert_eq!(
            formation_preview_fixtures[3].pathing_status.as_deref(),
            Some("detour:7,4")
        );
        assert_eq!(
            formation_preview_fixtures[4].group_route_tile_ids_if_empty,
            vec!["5,5", "6,4", "6,5", "7,5", "6,6"]
        );
        assert_eq!(
            formation_preview_fixtures[4].group_command_state.as_deref(),
            Some("split_route:group_2")
        );

        let recall_formation_fixtures = rts_control_group_recall_formation_preview_stage_fixtures();
        assert_eq!(
            recall_formation_fixtures
                .iter()
                .map(|fixture| fixture.stage.as_str())
                .collect::<Vec<_>>(),
            vec![
                "recall_focus_hud",
                "formation_anchor_slots",
                "queued_valid_members",
                "filtered_invalid"
            ]
        );
        assert_eq!(
            recall_formation_fixtures
                .iter()
                .map(|fixture| fixture.action.payload.as_str())
                .collect::<Vec<_>>(),
            vec!["28", "1,31:line", "1,31:line", "1,31:line"]
        );
        assert_eq!(
            recall_formation_fixtures[1].formation_slot_tile_ids,
            vec!["1,31", "2,31"]
        );
        assert_eq!(
            recall_formation_fixtures[3].filtered_member_ids,
            vec![
                "missing:multi0.recall.formation.missing",
                "foreign:map.actor1"
            ]
        );

        let recall_override_fixtures = rts_control_group_recall_override_preview_stage_fixtures();
        assert_eq!(
            recall_override_fixtures
                .iter()
                .map(|fixture| fixture.stage.as_str())
                .collect::<Vec<_>>(),
            vec![
                "group_26_recall_focus",
                "group_26_queued_order",
                "group_27_override_cancel",
                "group_27_final_filtered"
            ]
        );
        assert_eq!(
            recall_override_fixtures
                .iter()
                .map(|fixture| fixture.action.payload.as_str())
                .collect::<Vec<_>>(),
            vec!["26", "18,31:line", "27", "20,30:line"]
        );
        assert_eq!(
            recall_override_fixtures[2].canceled_member_ids,
            vec![
                "multi0.recall.override.runner",
                "multi0.recall.override.wing"
            ]
        );
        assert_eq!(
            recall_override_fixtures[2].override_final_tile_ids,
            vec!["20,30", "22,30"]
        );

        assert_eq!(
            rts_formation_move_execution_stage(
                &["formation_move_execution:arrival_lock".to_string()],
                &["formation_move_execution:slot_claim".to_string()],
                0,
            ),
            Some("arrival_lock")
        );
        assert_eq!(
            rts_formation_move_execution_stage(&[], &["formation_move_execution:".to_string()], 4,),
            Some("blocked_reroute")
        );
        assert_eq!(rts_formation_move_execution_stage(&[], &[], 0), None);
        let formation_execution_fixtures = rts_formation_move_execution_fixtures();
        assert_eq!(
            formation_execution_fixtures
                .stages
                .iter()
                .map(|fixture| fixture.stage.as_str())
                .collect::<Vec<_>>(),
            vec![
                "slot_claim",
                "path_reservation",
                "stagger_step",
                "crowd_avoidance",
                "blocked_reroute",
                "arrival_lock"
            ]
        );
        assert_eq!(
            formation_execution_fixtures
                .stages
                .iter()
                .map(|fixture| fixture.action.payload.as_str())
                .collect::<Vec<_>>(),
            vec![
                "box:frontline",
                "8,4:wedge",
                "8,4:line",
                "6,5:split",
                "8,4:wedge",
                "9,2:rally"
            ]
        );
        assert_eq!(
            formation_execution_fixtures.stages[5].group_route_tile_ids,
            vec!["6,5", "7,5", "8,5", "9,4", "9,2"]
        );
        assert_eq!(
            formation_execution_fixtures.stages[4].lagging_unit_ids,
            vec!["player", "square_guard_patrol"]
        );

        assert_eq!(
            rts_local_obstruction_recovery_stage(
                &["local_obstruction_recovery:flow_resume".to_string()],
                &["local_obstruction_recovery:detect_block".to_string()],
                0,
            ),
            Some("flow_resume")
        );
        assert_eq!(
            rts_local_obstruction_recovery_stage(
                &[],
                &["local_obstruction_recovery:".to_string()],
                2,
            ),
            Some("side_step")
        );
        assert_eq!(rts_local_obstruction_recovery_stage(&[], &[], 0), None);
        let obstruction_fixtures = rts_local_obstruction_recovery_fixtures();
        assert_eq!(
            obstruction_fixtures
                .stages
                .iter()
                .map(|fixture| fixture.stage.as_str())
                .collect::<Vec<_>>(),
            vec![
                "detect_block",
                "hold_queue",
                "side_step",
                "gap_claim",
                "flow_resume"
            ]
        );
        assert_eq!(
            obstruction_fixtures
                .stages
                .iter()
                .map(|fixture| fixture.action.payload.as_str())
                .collect::<Vec<_>>(),
            vec![
                "8,4:wedge",
                "8,4:line",
                "6,5:split",
                "box:frontline",
                "9,2:rally"
            ]
        );
        assert_eq!(
            obstruction_fixtures.stages[0].blocked_tile_ids,
            vec!["7,4", "7,5"]
        );
        assert_eq!(
            obstruction_fixtures.stages[4].group_route_tile_ids,
            vec!["6,5", "7,5", "8,5", "9,4", "9,2"]
        );
    }

    #[test]
    fn selection_roster_adapter_preserves_first_contact_rules() {
        assert_eq!(
            rts_default_group_units(),
            vec![
                "player",
                "square_guard_patrol",
                "square_worker_carry",
                "square_creep_wander"
            ]
        );
        assert_eq!(
            rts_group_two_units(),
            vec!["square_guard_patrol", "square_creep_wander"]
        );
        assert_eq!(rts_unit_selection_class("square_worker_carry"), "worker");
        assert_eq!(
            rts_same_class_units("player"),
            vec!["player", "square_guard_front", "square_guard_patrol"]
        );
        assert_eq!(rts_unit_allegiance("square_creep_wander"), "hostile");
        assert!(rts_unit_is_player_owned("square_worker_harvest"));
        assert_eq!(rts_unit_selection_priority("square_creep_wander"), 20);
        assert_eq!(
            rts_selectable_unit_tile("square_guard_patrol"),
            Some((7, 5))
        );
        assert_eq!(rts_selectable_unit_at_tile((5, 4)), Some("player"));
        assert_eq!(
            rts_selection_tiles_for_units(&[
                "player".to_string(),
                "square_guard_front".to_string(),
                "square_worker_carry".to_string()
            ]),
            vec!["5,4", "4,5"]
        );
        assert_eq!(rts_selection_box_tiles(), vec!["5,5", "6,5", "5,4", "6,4"]);
        assert_eq!(
            rts_drag_selection_parts("drag:5,4->9,5"),
            Some(((5, 4), (9, 5)))
        );
        assert_eq!(rts_drag_distance_sq((4, 4), (8, 5)), 17);
        assert!(rts_drag_select_ready((240, 180), (520, 350)));
        assert_eq!(rts_drag_group_id((4, 4), (8, 5)), "drag:4,4->8,5");
        assert_eq!(
            rts_drag_select_player_label("4,4", "8,5", 5),
            "DRAG SELECT 5 UNITS 4,4->8,5"
        );
        assert_eq!(
            rts_drag_select_player_label("4,4", "4,4", 1),
            "DRAG SELECT 1 UNIT 4,4->4,4"
        );
        assert_eq!(
            rts_selection_box_tiles_between((5, 4), (6, 5)),
            vec!["5,4", "6,4", "5,5", "6,5"]
        );
        assert_eq!(
            rts_drag_selected_units((4, 4), (8, 5)),
            vec![
                "player",
                "square_guard_front",
                "square_guard_patrol",
                "square_worker_carry",
                "square_worker_harvest"
            ]
        );
        assert_eq!(
            rts_drag_rejected_unit_ids((5, 4), (9, 5)),
            vec!["square_creep_wander"]
        );
    }

    #[test]
    fn control_group_roster_adapter_preserves_first_contact_slots() {
        let assignments = vec![
            "2:player|square_guard_patrol".to_string(),
            "10:camera:square_worker_carry|square_worker_harvest".to_string(),
        ];
        let active_group_ids = vec!["10".to_string()];

        assert_eq!(
            rts_control_group_hotkey_slot("assign:10", "assign:").as_deref(),
            Some("10")
        );
        assert_eq!(
            rts_default_units_for_control_group_slot("3"),
            vec!["square_worker_carry", "square_worker_harvest"]
        );
        assert_eq!(
            rts_units_from_control_group_assignment(&assignments, "10"),
            vec!["square_worker_carry", "square_worker_harvest"]
        );
        assert_eq!(rts_control_group_slot_label("10"), "0");
        assert_eq!(rts_control_group_slot_member_count(&assignments, "10"), 2);
        assert!(rts_control_group_slot_is_active(
            &active_group_ids,
            Some("2"),
            "10"
        ));

        let slot_ten = rts_control_group_slot_summaries(&assignments, &active_group_ids, Some("2"))
            .into_iter()
            .find(|summary| summary.slot == "10")
            .expect("slot 10 summary");
        assert_eq!(slot_ten.key_label, "0");
        assert_eq!(slot_ten.member_count, 2);
        assert!(slot_ten.occupied);
        assert!(slot_ten.active);
        assert_eq!(
            rts_merged_unit_ids(
                &["player".to_string()],
                &["player".to_string(), "square_worker_carry".to_string()],
            ),
            vec!["player", "square_worker_carry"]
        );
    }

    #[test]
    fn command_parts_adapter_preserves_first_contact_parsing() {
        assert_eq!(
            rts_selection_clear_parts("clear:hostile:square_creep_wander@9,4"),
            Some((
                "hostile".to_string(),
                Some("square_creep_wander".to_string()),
                "9,4".to_string()
            ))
        );
        assert_eq!(
            rts_move_command_parts("minimap:9,2:attack_move"),
            ("9,2", "attack_move")
        );
        assert_eq!(
            rts_line_path_tiles((5, 5), (8, 3)),
            vec!["6,5", "7,4", "8,3"]
        );
        assert_eq!(
            rts_focus_fire_units_for_target("enemy_barracks"),
            vec![
                "relay_guard_alpha",
                "relay_guard_beta",
                "wayfinder_scout",
                "field_mender"
            ]
        );
        assert_eq!(
            rts_creep_camp_units_for_id("forest_creep_camp"),
            vec!["forest_alpha_creep", "forest_stalker", "forest_shaman"]
        );
        assert_eq!(
            rts_objective_parts("claim:relay_beacon@9,2"),
            (
                "claim".to_string(),
                "relay_beacon".to_string(),
                "9,2".to_string()
            )
        );
        assert_eq!(
            rts_creep_camp_parts("camp", "clear:creep_camp@8,3"),
            (
                "clear".to_string(),
                "forest_creep_camp".to_string(),
                "8,3".to_string()
            )
        );
        assert_eq!(
            rts_recon_parts("mark:scout_enemy_base@10,2"),
            (
                "mark".to_string(),
                "enemy_base".to_string(),
                "10,2".to_string()
            )
        );
        assert_eq!(
            rts_enemy_command_parts("pressure:counter_wave@enemy_gate", "pressure", "enemy_base"),
            (
                "pressure".to_string(),
                "counter_wave".to_string(),
                "enemy_gate".to_string()
            )
        );
        assert_eq!(
            rts_counter_command_parts("upgrade:signal_blade@training_hall"),
            (
                "upgrade".to_string(),
                "signal_blade".to_string(),
                "training_hall".to_string()
            )
        );
        assert_eq!(
            rts_army_command_parts("train:mixed_vanguard@training_hall"),
            (
                "train".to_string(),
                "mixed_vanguard".to_string(),
                "training_hall".to_string()
            )
        );
        assert_eq!(
            rts_base_assault_parts("breach:enemy_barracks@10,3"),
            (
                "breach".to_string(),
                "enemy_barracks".to_string(),
                "10,3".to_string()
            )
        );
        assert_eq!(
            rts_aftermath_parts("destroy:enemy_barracks@10,3"),
            (
                "destroy".to_string(),
                "enemy_barracks".to_string(),
                "10,3".to_string()
            )
        );
        assert_eq!(
            rts_commander_parts("level:mirror_captain@forest_relay"),
            (
                "level".to_string(),
                "mirror_captain".to_string(),
                "forest_relay".to_string()
            )
        );
        assert_eq!(
            rts_expansion_parts("claim:forest_relay@9,2"),
            (
                "claim".to_string(),
                "forest_relay".to_string(),
                "9,2".to_string()
            )
        );
        assert_eq!(
            rts_tier_two_parts("tech:stonebreak_cart@relay_outpost"),
            (
                "tech".to_string(),
                "stonebreak_cart".to_string(),
                "relay_outpost".to_string()
            )
        );
    }

    #[test]
    fn hover_cursor_adapter_preserves_first_contact_affordances() {
        assert_eq!(
            rts_hover_target_preview_kind("viewport_attack_target"),
            Some("attack")
        );
        assert_eq!(
            rts_hover_target_preview_kind("viewport_harvest"),
            Some("harvest")
        );
        assert_eq!(
            rts_hover_target_preview_kind("viewport_follow"),
            Some("follow")
        );
        assert_eq!(
            rts_cursor_kind_for_hover_preview(true, "command_button", "RTS:ABILITY:focus_fire"),
            "ability"
        );
        assert_eq!(
            rts_cursor_label_for_hover_preview(
                "classic_rts_mouse_command_bar",
                "RTS:ABILITY:focus_fire",
                true,
                "ability"
            ),
            "COMMAND BAR CURSOR ABILITY READY"
        );
        assert_eq!(
            rts_cursor_kind_for_hover_preview(false, "viewport_move", "RTS:MOVE:4,3:line"),
            "blocked"
        );
        assert_eq!(
            rts_cursor_label_for_hover_preview(
                "classic_rts_mouse_viewport",
                "RTS:MOVE:4,3:line",
                false,
                "blocked"
            ),
            "MAP CURSOR BLOCKED LOCK"
        );
        assert_eq!(
            rts_hover_player_label(
                "classic_rts_mouse_sidebar",
                "RTS:QUEUE:build:watch_tower@7,4",
                None,
                Some("build:watch_tower@7,4"),
                "sidebar_build_queue",
                true,
                "ok",
            ),
            "SIDEBAR QUEUE READY WATCH TOWER 7,4 210G"
        );
        assert_eq!(
            rts_hover_player_label(
                "classic_rts_mouse_viewport",
                "RTS:MOVE:4,3:line",
                Some("4,3"),
                None,
                "viewport_move",
                true,
                "ok",
            ),
            "MAP MOVE READY 4,3"
        );
        assert_eq!(
            rts_hover_player_label(
                "classic_rts_mouse_viewport",
                "RTS:MOVE:6,5:follow:square_guard_patrol",
                Some("6,5"),
                None,
                "viewport_follow",
                true,
                "ok",
            ),
            "MAP FOLLOW READY SQUARE GUARD PATRO"
        );
        assert_eq!(
            rts_hover_player_label(
                "classic_rts_mouse_viewport",
                "RTS:MOVE:4,3:line",
                Some("4,3"),
                None,
                "viewport_move",
                false,
                "rts_group_selection_required",
            ),
            "MAP MOVE LOCK SELECT UNITS"
        );
    }

    #[test]
    fn command_stamp_adapter_preserves_first_contact_feedback_labels() {
        let selection_stamp = rts_command_stamp_for_selection("classic_rts_hotkey", "assign:5", 2);
        assert_eq!(selection_stamp.kind, "control-group");
        assert_eq!(selection_stamp.target_id.as_deref(), Some("5"));
        assert_eq!(
            selection_stamp.player_label,
            "HOTKEY GROUP 5 ASSIGNED 2 UNITS"
        );

        let move_stamp = rts_command_stamp_for_move("classic_rts_mouse_viewport", "7,4:line")
            .expect("valid move tile stamp");
        assert_eq!(move_stamp.kind, "move");
        assert_eq!(move_stamp.tile_id.as_deref(), Some("7,4"));
        assert_eq!(move_stamp.player_label, "MAP MOVE SENT 7,4");

        let ability_stamp = rts_command_stamp_for_ability(
            "classic_rts_mouse_command_bar",
            "focus_fire",
            Some("arena_creep_attack"),
        );
        assert_eq!(ability_stamp.kind, "ability");
        assert_eq!(ability_stamp.tile_id.as_deref(), Some("6,5"));
        assert_eq!(
            ability_stamp.target_id.as_deref(),
            Some("arena_creep_attack")
        );
        assert_eq!(
            ability_stamp.player_label,
            "COMMAND BAR ABILITY SENT FOCUS FIRE"
        );
    }

    #[test]
    fn order_queue_replay_adapter_preserves_command_surface_actions() {
        assert_eq!(
            rts_order_queue_replay_action("queue:attack:arena_creep_attack", "fallback"),
            RtsOrderQueueReplayAction {
                kind: "attack".to_string(),
                payload: "arena_creep_attack".to_string(),
            }
        );
        assert_eq!(
            rts_order_queue_replay_action("queue:move:9,2", "fallback"),
            RtsOrderQueueReplayAction {
                kind: "move".to_string(),
                payload: "9,2:line".to_string(),
            }
        );
        assert_eq!(
            rts_order_queue_replay_action("minimap:rally:5,2", "fallback"),
            RtsOrderQueueReplayAction {
                kind: "move".to_string(),
                payload: "minimap:rally:5,2".to_string(),
            }
        );
        assert_eq!(
            rts_order_queue_replay_action("queue:train:worker", "fallback"),
            RtsOrderQueueReplayAction {
                kind: "queue".to_string(),
                payload: "train:worker".to_string(),
            }
        );
        assert_eq!(
            rts_order_queue_replay_action("queue:select_group_3", "fallback"),
            RtsOrderQueueReplayAction {
                kind: "select-control-group".to_string(),
                payload: "3".to_string(),
            }
        );
        assert_eq!(
            rts_order_queue_replay_action("feedback:build_placed:watch_tower@7,4", "focus_fire"),
            RtsOrderQueueReplayAction {
                kind: "ability".to_string(),
                payload: "focus_fire".to_string(),
            }
        );
    }

    #[test]
    fn scripted_demo_timeline_adapter_preserves_queue_cancel_sequence() {
        assert!(rts_scripted_demo_pauses_queue_tick("queue_cancel_refund"));
        assert!(rts_scripted_demo_pauses_queue_tick(
            "queue_cancel_refund_sequence"
        ));
        assert!(!rts_scripted_demo_pauses_queue_tick("live_player_flow"));
        assert_eq!(
            rts_scripted_demo_stage_from_frame("queue_cancel_refund_sequence", 0),
            Some(0)
        );
        assert_eq!(
            rts_scripted_demo_stage_from_frame("queue_cancel_refund_sequence", 60),
            Some(1)
        );
        assert_eq!(
            rts_scripted_demo_stage_from_frame("queue_cancel_refund_sequence", 240),
            Some(4)
        );
        assert_eq!(
            rts_scripted_demo_stage_from_frame("queue_cancel_refund_sequence", 300),
            Some(0)
        );
        assert_eq!(
            rts_scripted_demo_stage_from_frame("queue_cancel_refund", 60),
            None
        );
        assert_eq!(rts_scripted_demo_stage_id(3), "cancel_refund");
        assert_eq!(rts_scripted_demo_stage_title(4), "WORKER QUEUED");
    }

    #[test]
    fn scene_stage_adapter_preserves_first_contact_event_precedence() {
        assert_eq!(
            rts_npc_behavior_stage(
                &["behavior:creep_retreat".to_string()],
                &["behavior:guard_patrol".to_string()],
                0,
            ),
            Some("creep_retreat")
        );
        assert_eq!(
            rts_combat_impact_stage(&[], &["impact:damage_tick".to_string()], 1),
            Some("damage_tick")
        );
        assert_eq!(
            rts_locomotion_blend_stage(&[], &["locomotion:cycle".to_string()], 5),
            Some("arrival_brake")
        );
        assert_eq!(
            rts_npc_transition_stage(
                &["transition:hit_recover".to_string()],
                &["transition:alert_turn".to_string()],
                0,
            ),
            Some("hit_recover")
        );
        assert_eq!(
            rts_depth_readability_stage(&[], &["depth:cycle".to_string()], 4),
            Some("path_occlusion")
        );
        assert_eq!(
            rts_structure_modeling_stage(
                &["structure:repair_beam".to_string()],
                &["structure:foundation_shadow".to_string()],
                0,
            ),
            Some("repair_beam")
        );
        assert_eq!(
            rts_environment_life_stage(&[], &["environment:cycle".to_string()], 4),
            Some("resource_glint")
        );
        assert_eq!(
            rts_worker_harvest_animation_stage(
                &["harvest_anim:return_path".to_string()],
                &["harvest_anim:approach".to_string()],
                0,
            ),
            Some("return_path")
        );
        assert_eq!(
            rts_production_spawn_animation_stage(
                &["production_spawn_anim:supply_flash".to_string()],
                &["production_spawn_anim:queue_pulse".to_string()],
                0,
            ),
            Some("supply_flash")
        );
        assert_eq!(rts_npc_behavior_stage(&[], &[], 0), None);
        assert_eq!(rts_structure_modeling_stage(&[], &[], 0), None);
        assert_eq!(rts_environment_life_stage(&[], &[], 0), None);
        assert_eq!(rts_worker_harvest_animation_stage(&[], &[], 0), None);
        assert_eq!(rts_production_spawn_animation_stage(&[], &[], 0), None);
    }

    #[test]
    fn action_cadence_adapter_preserves_first_contact_marks() {
        let guard_attack = rts_action_cadence_marks("actor_guard_attack");
        assert_eq!(guard_attack.len(), 22);
        assert_eq!(
            guard_attack
                .iter()
                .filter(|mark| mark.kind == "windup")
                .count(),
            5
        );
        assert_eq!(
            guard_attack
                .iter()
                .filter(|mark| mark.kind == "strike")
                .count(),
            9
        );
        assert_eq!(
            guard_attack
                .iter()
                .filter(|mark| mark.kind == "recovery")
                .count(),
            6
        );
        assert_eq!(
            guard_attack
                .iter()
                .filter(|mark| mark.kind == "shadow_smear")
                .count(),
            2
        );

        let creep_attack = rts_action_cadence_marks("actor_creep_attack");
        assert_eq!(creep_attack.first().map(|mark| mark.rect.x), Some(-24));
        assert_eq!(guard_attack.first().map(|mark| mark.rect.x), Some(-22));

        let worker_carry = rts_action_cadence_marks("actor_worker_carry");
        assert_eq!(worker_carry.len(), 8);
        assert_eq!(
            worker_carry
                .iter()
                .filter(|mark| mark.kind == "carry_bob")
                .count(),
            4
        );

        let guard_idle = rts_action_cadence_marks("actor_guard_idle");
        assert_eq!(guard_idle.len(), 4);
        assert!(guard_idle.iter().all(|mark| mark.kind == "idle_breath"));
        assert!(rts_action_cadence_marks("actor_player_idle_south").is_empty());
    }

    #[test]
    fn action_sequence_adapter_preserves_phase_and_marks() {
        assert_eq!(
            rts_action_sequence_phase(
                "actor_guard_attack",
                &["sequence:recovery".to_string()],
                &["sequence:windup".to_string()],
                2,
                2,
                true,
            ),
            Some("recovery")
        );
        assert_eq!(
            rts_action_sequence_phase(
                "actor_guard_attack",
                &[],
                &["sequence:cycle".to_string()],
                1,
                1,
                true,
            ),
            Some("windup")
        );
        assert_eq!(
            rts_action_sequence_phase(
                "actor_guard_attack",
                &[],
                &["sequence:cycle".to_string()],
                1,
                2,
                true,
            ),
            Some("strike")
        );
        assert_eq!(
            rts_action_sequence_phase(
                "actor_worker_carry",
                &[],
                &["sequence:cycle".to_string()],
                2,
                0,
                true,
            ),
            Some("carry_up")
        );
        assert_eq!(
            rts_action_sequence_phase(
                "actor_worker_carry",
                &[],
                &["sequence:cycle".to_string()],
                1,
                0,
                true,
            ),
            Some("carry_down")
        );
        assert_eq!(
            rts_action_sequence_phase("actor_guard_attack", &[], &[], 1, 2, true),
            None
        );
        assert_eq!(
            rts_action_sequence_phase("actor_guard_attack", &[], &[], 1, 2, false),
            Some("strike")
        );

        let windup = rts_action_sequence_marks("actor_guard_attack", "windup");
        assert_eq!(windup.len(), 9);
        assert_eq!(
            windup.first().map(|mark| mark.kind.as_str()),
            Some("frame_ghost")
        );
        assert_eq!(
            windup.iter().filter(|mark| mark.kind == "windup").count(),
            8
        );

        let strike = rts_action_sequence_marks("actor_guard_attack", "strike");
        assert_eq!(strike.len(), 12);
        assert_eq!(
            strike.iter().filter(|mark| mark.kind == "strike").count(),
            11
        );

        let carry_down = rts_action_sequence_marks("actor_worker_carry", "carry_down");
        assert_eq!(carry_down.len(), 5);
        assert_eq!(
            carry_down
                .iter()
                .filter(|mark| mark.kind == "carry_down")
                .count(),
            4
        );

        let idle = rts_action_sequence_marks("actor_guard_idle", "idle");
        assert_eq!(idle.len(), 6);
        assert_eq!(idle.iter().filter(|mark| mark.kind == "idle").count(), 5);

        assert!(rts_action_sequence_marks("actor_player_idle_south", "idle").is_empty());
    }

    #[test]
    fn unit_model_depth_adapter_preserves_role_marks() {
        let guard = rts_unit_model_depth_marks("actor_guard_attack");
        assert_eq!(guard.len(), 8);
        assert_eq!(guard.iter().filter(|mark| mark.kind == "rim").count(), 2);
        assert_eq!(guard.iter().filter(|mark| mark.kind == "armor").count(), 2);
        assert_eq!(
            guard
                .iter()
                .find(|mark| mark.kind == "face_shade")
                .map(|mark| mark.rect.y),
            Some(-32)
        );

        let worker = rts_unit_model_depth_marks("actor_worker_carry");
        assert_eq!(worker.len(), 8);
        assert_eq!(
            worker
                .iter()
                .filter(|mark| mark.kind == "layer_shadow")
                .count(),
            2
        );
        assert_eq!(
            worker
                .iter()
                .find(|mark| mark.kind == "role_prop")
                .map(|mark| mark.rect.x),
            Some(-15)
        );

        let creep = rts_unit_model_depth_marks("actor_creep_attack");
        assert_eq!(creep.len(), 8);
        assert_eq!(
            creep.iter().filter(|mark| mark.kind == "role_prop").count(),
            2
        );
        assert_eq!(
            creep
                .iter()
                .filter(|mark| mark.kind == "armor")
                .map(|mark| mark.rect.width)
                .next(),
            Some(22)
        );

        assert!(rts_unit_model_depth_marks("actor_player_idle_south").is_empty());
    }

    #[test]
    fn scrollable_map_stage_summaries_preserve_camera_contract() {
        let summaries = rts_scrollable_map_camera_stage_summaries();

        assert_eq!(summaries.len(), 6);
        assert_eq!(summaries[0].stage, "keyboard_pan");
        assert_eq!(summaries[0].source, "shift_keyboard_pan");
        assert_eq!(summaries[0].focus_tile, (9, 7));
        assert_eq!(
            summaries[0].command_destination_tile.as_deref(),
            Some("9,7")
        );
        assert_eq!(
            summaries[4].step.minimap_tile_id.as_deref(),
            Some("minimap_cursor_jump")
        );
        assert_eq!(summaries[4].focus_tile, (21, 20));
        assert!(summaries[5].step.clamped);
        assert_eq!(summaries[5].focus_tile.0, TRNM_RTS_RUNTIME_MAP_MAX_X);
        assert!(summaries
            .iter()
            .all(|summary| summary.command_queue.len() == 2));
    }

    #[test]
    fn camera_minimap_stage_summaries_preserve_sync_contract() {
        let summaries = rts_camera_minimap_sync_stage_summaries();

        assert_eq!(summaries.len(), 6);
        assert_eq!(summaries[0].stage, "viewport_rect");
        assert_eq!(summaries[0].focus_tile, (8, 8));
        assert_eq!(summaries[0].viewport_rect.width, 33);
        assert_eq!(summaries[0].viewport_rect.height, 19);
        assert!(summaries
            .iter()
            .all(|summary| summary.revealed_tile_ids.len() >= 4));
        assert_eq!(
            summaries[2].step.minimap_tile_id.as_deref(),
            Some("mirror_captain")
        );
        assert_eq!(summaries[2].focus_tile, (20, 19));
        assert_eq!(summaries[3].control_group_id, "2");
        assert_eq!(
            summaries[4].minimap_command_tile_id.as_deref(),
            Some("minimap_route_target")
        );
        assert!(
            summaries[5].viewport_rect_area < summaries[0].viewport_rect_area,
            "zoom stage should shrink the minimap viewport rect"
        );
    }

    #[test]
    fn offline_adapter_consumption_review_preserves_player_screen_handoff_contract() {
        let player_screen_application = rts_first_contact_player_screen_runtime_application(
            &trnm_rts_data::first_contact_player_screen_profile(),
        );
        assert_eq!(
            player_screen_application.contract_version,
            TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_PLAYER_SCREEN_APPLICATION_CONTRACT
        );
        assert!(player_screen_application.green);
        assert!(player_screen_application.profile_application_gate);
        assert!(player_screen_application.command_surface_seed_gate);
        assert!(player_screen_application.route_surface_seed_gate);
        assert_eq!(
            player_screen_application.profile_contract,
            trnm_rts_data::TRNM_RTS_DATA_FIRST_CONTACT_PLAYER_SCREEN_CONTRACT
        );
        assert_eq!(player_screen_application.map_scene, "first_contact_basin");
        assert_eq!(
            player_screen_application.current_room_id,
            "first-contact-basin"
        );
        assert_eq!(
            player_screen_application.camera_focus_tile_id.as_deref(),
            Some("16,16")
        );
        assert_eq!(
            player_screen_application
                .command_destination_tile_id
                .as_deref(),
            Some("16,9")
        );
        assert_eq!(player_screen_application.command_queue.len(), 4);
        assert_eq!(player_screen_application.visible_tile_ids.len(), 64);
        assert_eq!(
            player_screen_application.ability_command_ids,
            rts_string_vec(["worker", "scout", "warden", "relay", "core", "signal"])
        );
        assert!(player_screen_application
            .source_of_truth
            .contains("trnm-rts-data First Contact player-screen profile"));

        let handoff = RtsOfflineAdapterRuntimeHandoffReviewInput {
            contract_version: "trnm_rts_online_offline_adapter_runtime_handoff_v1".to_string(),
            handoff_mode: "server_authoritative_runtime_command_handoff".to_string(),
            accepted_runtime_command_labels: rts_string_vec(["move:8,4"]),
            accepted_runtime_destination_tile_ids: rts_string_vec(["8,4"]),
            accepted_runtime_subject_actor_ids: rts_string_vec(["trnm.worker.alpha"]),
            rejected_runtime_command_labels: rts_string_vec(["client:attack_fogged_keep"]),
            scoped_update_actor_ids: rts_string_vec([
                "trnm.worker.alpha",
                "trnm.horizon.scout.alpha",
                "trnm.command.core.alpha",
                "trnm.flux.beacon.center",
            ]),
            runtime_control_group_id: "1".to_string(),
            runtime_group_command_state: "offline_adapter_authority_applied".to_string(),
            runtime_pathing_status: "offline_adapter_replay_consumed".to_string(),
            runtime_unit_response_state: "server_authoritative_move_applied".to_string(),
            runtime_command_stamp_source: "trnm-rts-online:offline_loopback_authority".to_string(),
            runtime_command_stamp_kind: "server_accepted_move".to_string(),
            runtime_command_stamp_tile_id: Some("8,4".to_string()),
            runtime_command_stamp_player_label: "SERVER ACCEPTED MOVE 8,4".to_string(),
            runtime_last_feedback:
                "Offline adapter applied server move 8,4; rejected target_actor_not_visible"
                    .to_string(),
            accepted_order_runtime_ready: true,
            rejected_order_runtime_ready: true,
            scoped_update_runtime_ready: true,
            no_socket_boundary_ready: true,
            green: true,
        };
        let runtime_application = rts_first_contact_offline_adapter_runtime_application(&handoff);
        assert_eq!(
            runtime_application.contract_version,
            TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_APPLICATION_CONTRACT
        );
        assert!(runtime_application.green);
        assert_eq!(
            runtime_application.command_queue,
            rts_string_vec(["move:8,4"])
        );
        assert_eq!(
            runtime_application.runtime_application_path,
            "trnm-rts-bevy-runtime offline_adapter_runtime_application -> NativeFirstPlayableRuntime mutation"
        );

        let session_transition = rts_first_contact_offline_adapter_session_transition_review(
            &player_screen_application,
            &runtime_application,
            &handoff,
        );
        assert_eq!(
            session_transition.contract_version,
            TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_SESSION_TRANSITION_CONTRACT
        );
        assert!(session_transition.green);
        assert!(session_transition.command_surface_replaced_gate);
        assert!(session_transition.route_overlay_replaced_gate);
        assert!(session_transition.session_context_preserved_gate);
        assert!(session_transition.rejected_order_suppressed_gate);
        assert!(session_transition.no_socket_boundary_gate);
        assert!(session_transition
            .before_command_queue
            .iter()
            .any(|command| command == "build:trnm.flux.relay"));
        assert_eq!(
            session_transition.after_command_queue,
            rts_string_vec(["move:8,4"])
        );
        assert_eq!(
            session_transition
                .before_command_destination_tile_id
                .as_deref(),
            Some("16,9")
        );
        assert_eq!(
            session_transition
                .after_command_destination_tile_id
                .as_deref(),
            Some("8,4")
        );
        assert!(session_transition
            .source_of_truth
            .contains("server-authoritative offline adapter handoff"));

        let lobby_ready_review = rts_first_contact_offline_adapter_lobby_ready_review(
            RtsOfflineAdapterLobbyReadyReviewInput {
                adapter_green: true,
                adapter_contract: "trnm_rts_online_offline_adapter_v1".to_string(),
                adapter_id: "first-contact-offline-loopback-adapter".to_string(),
                handoff_id: "first-contact-local-loopback-handoff".to_string(),
                arena_id: "first-contact-local-arena".to_string(),
                map_id: "first_contact_basin".to_string(),
                adapter_mode: "offline_loopback_authority".to_string(),
                bevy_client_role: "visualization_and_local_input_submitter".to_string(),
                authority_role: "trnm_rts_online_fixture_authority_no_socket".to_string(),
                connected_player_ids: rts_string_vec(["local-player", "mirror_guard"]),
                bot_player_ids: rts_string_vec(["mirror_guard"]),
                frame_sha256s: rts_string_vec([
                    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                    "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
                    "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
                ]),
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
            },
        );
        assert_eq!(
            lobby_ready_review.contract_version,
            TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_LOBBY_READY_CONTRACT
        );
        assert!(lobby_ready_review.green);
        assert!(lobby_ready_review.local_multiplayer_ready_gate);
        assert!(lobby_ready_review.offline_bot_ready_gate);
        assert!(lobby_ready_review.bevy_adapter_ready_gate);
        assert!(lobby_ready_review.authority_ready_gate);
        assert!(lobby_ready_review.frame_identity_gate);
        assert!(lobby_ready_review.no_network_claim_gate);
        assert_eq!(
            lobby_ready_review.ready_state_labels,
            rts_string_vec([
                "player:local-player:ready",
                "player:mirror_guard:ready",
                "bot:mirror_guard:ready",
                "authority:offline_loopback:no_socket",
            ])
        );
        assert_eq!(lobby_ready_review.blocked_network_claim_labels.len(), 5);
        assert!(lobby_ready_review
            .source_of_truth
            .contains("lobby ready review"));

        let runtime_player_screen_review = RtsFirstContactPlayerScreenReview {
            map_scene: "first_contact_basin".to_string(),
            current_room_id: "first-contact-basin".to_string(),
            coins: 890,
            xp: 92,
            camera_focus_tile_id: Some("16,16".to_string()),
            visibility_percent: 76,
            army_supply_used: 12,
            army_supply_cap: 22,
            objective_status: "secure first relay beacon and hold the center lane".to_string(),
            production_queue: rts_string_vec([
                "train:guard",
                "train:worker",
                "upgrade:signal_blade",
            ]),
            build_queue: rts_string_vec(["build:watch_tower", "upgrade:training_hall"]),
            selected_unit_ids: rts_string_vec(["trnm.worker.alpha"]),
            command_queue: rts_string_vec(["move:8,4"]),
            command_destination_tile_id: Some("8,4".to_string()),
            group_route_tile_ids: rts_string_vec(["8,4"]),
            visible_tile_count: 64,
            fogged_tile_count: 6,
            selection_box_tile_count: 4,
            unit_health_percents: vec![96, 78, 71, 34],
            ability_command_ids: rts_string_vec([
                "worker", "scout", "warden", "relay", "core", "signal",
            ]),
            ability_cooldown_percents: vec![0, 0, 16, 0, 42, 25],
            active_ability_id: Some("worker".to_string()),
        };
        let review = rts_first_contact_offline_adapter_consumption_review(
            RtsFirstContactOfflineAdapterConsumptionReviewInput {
                adapter_green: true,
                adapter_contract: "trnm_rts_online_offline_adapter_v1".to_string(),
                adapter_id: "first-contact-offline-loopback-adapter".to_string(),
                adapter_mode: "offline_loopback_authority".to_string(),
                adapter_runtime_handoff: handoff,
                input_queue_labels: rts_string_vec([
                    "client:move_worker@8,4",
                    "client:attack_fogged_keep",
                ]),
                accepted_server_order_labels: rts_string_vec(["client:move_worker@8,4"]),
                rejected_client_order_reasons: rts_string_vec(["target_actor_not_visible"]),
                runtime_player_screen_review,
                server_authoritative: true,
                visibility_scoped_response: true,
                client_prediction_claimed: false,
                rollback_netcode_claimed: false,
                socket_opened: false,
                hosted_service_claimed: false,
                public_launch_ready: false,
            },
        );

        assert_eq!(
            review.contract_version,
            TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_CONSUMPTION_CONTRACT
        );
        assert!(review.green);
        assert_eq!(
            review.runtime_application_contract,
            TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_OFFLINE_ADAPTER_APPLICATION_CONTRACT
        );
        assert!(review.runtime_application_gate);
        assert_eq!(review.runtime_application, runtime_application);
        assert!(review.accepted_order_runtime_gate);
        assert!(review.local_session_handoff_gate);
        assert!(review.player_screen_review_gate);
        assert!(review.rejected_order_runtime_gate);
        assert!(review.scoped_update_runtime_gate);
        assert!(review.no_network_claim_gate);
        assert!(review.rejected_commands_suppressed);
        assert_eq!(
            review.runtime_player_screen_review.command_queue,
            rts_string_vec(["move:8,4"])
        );
        assert_eq!(
            review.runtime_path,
            "trnm-rts-bevy-runtime offline_adapter_runtime_application + first_contact_offline_adapter_consumption_review -> NativeFirstPlayableRuntime consumer"
        );
        assert_eq!(
            review.input_path,
            "trnm-rts-online offline adapter review input -> trnm-rts-bevy-runtime runtime application -> Bevy local player-screen snapshot"
        );
        assert!(review
            .source_of_truth
            .contains("trnm-rts-online-owned review input"));
    }
}
