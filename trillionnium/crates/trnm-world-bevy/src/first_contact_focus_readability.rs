#![cfg(not(target_os = "android"))]

use serde_json::Value;
use trnm_rts_core::RtsTile;

use crate::{
    classic_parse_rts_tile, classic_rts_tile_id, classic_text_advance_px, first_contact_readouts,
    first_contact_tiles, NativeFirstPlayableRuntime,
    CLASSIC_FIRST_CONTACT_ROUTE_ACK_TICK_HEIGHT_PX, CLASSIC_FIRST_CONTACT_ROUTE_ACK_TICK_WIDTH_PX,
    CLASSIC_FIRST_CONTACT_ROUTE_CLEARANCE_CORNER_CUES_PER_TILE,
    CLASSIC_FIRST_CONTACT_ROUTE_CLEARANCE_CORNER_CUE_H_PX,
    CLASSIC_FIRST_CONTACT_ROUTE_CLEARANCE_CORNER_CUE_W_PX,
    CLASSIC_FIRST_CONTACT_ROUTE_DASH_HEIGHT_PX, CLASSIC_FIRST_CONTACT_ROUTE_DASH_WIDTH_PX,
    CLASSIC_FIRST_CONTACT_SELECTED_FOCUS_BRACKET_PIXELS_PER_TILE,
    CLASSIC_FIRST_CONTACT_SELECTED_ROLE_BADGE_TICK_H_PX,
    CLASSIC_FIRST_CONTACT_SELECTED_ROLE_BADGE_TICK_W_PX,
    CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_CLEARANCE_PAD_PX,
    CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_HEALTH_BAR_H_PX,
    CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_HEALTH_BAR_W_PX,
    CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_H_PX,
    CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_LEADER_CLEARANCE_H_PX,
    CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_LEADER_CLEARANCE_W_PX,
    CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_W_PX, CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_X_OFFSET_PX,
    CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_Y_OFFSET_PX,
    CLASSIC_FIRST_CONTACT_TARGET_LOCK_ACK_TICK_H_PX,
    CLASSIC_FIRST_CONTACT_TARGET_LOCK_ACK_TICK_W_PX,
    CLASSIC_FIRST_CONTACT_TARGET_LOCK_CROSS_LONG_PX,
    CLASSIC_FIRST_CONTACT_TARGET_LOCK_CROSS_THICKNESS_PX,
    CLASSIC_FIRST_CONTACT_TARGET_PREFLIGHT_CROSS_LONG_PX,
    CLASSIC_FIRST_CONTACT_TARGET_PREFLIGHT_CROSS_THICKNESS_PX,
    CLASSIC_FIRST_CONTACT_TARGET_PREFLIGHT_RING_COUNT,
    CLASSIC_FIRST_CONTACT_TARGET_PREFLIGHT_RING_THICKNESS_PX,
};

fn selected_focus_tiles(runtime: &NativeFirstPlayableRuntime) -> Vec<String> {
    runtime
        .rts_selection_box_tile_ids
        .iter()
        .filter_map(|tile_id| classic_parse_rts_tile(tile_id).map(classic_rts_tile_id))
        .collect()
}

fn target_focus_tile_id(
    runtime: &NativeFirstPlayableRuntime,
    fallback_target_tile: RtsTile,
) -> String {
    classic_rts_tile_id(first_contact_tiles::target_callout_tile(
        runtime,
        fallback_target_tile,
    ))
}

fn focus_geometry_snapshot() -> trnm_rts_evidence::RtsFirstContactFocusReadabilityGeometrySnapshot {
    trnm_rts_evidence::RtsFirstContactFocusReadabilityGeometrySnapshot {
        selected_role_badge_tick_width_px: CLASSIC_FIRST_CONTACT_SELECTED_ROLE_BADGE_TICK_W_PX
            as usize,
        selected_role_badge_tick_height_px: CLASSIC_FIRST_CONTACT_SELECTED_ROLE_BADGE_TICK_H_PX
            as usize,
        selected_focus_bracket_pixels_per_tile:
            CLASSIC_FIRST_CONTACT_SELECTED_FOCUS_BRACKET_PIXELS_PER_TILE,
        route_dash_width_px: CLASSIC_FIRST_CONTACT_ROUTE_DASH_WIDTH_PX as usize,
        route_dash_height_px: CLASSIC_FIRST_CONTACT_ROUTE_DASH_HEIGHT_PX as usize,
        route_ack_tick_width_px: CLASSIC_FIRST_CONTACT_ROUTE_ACK_TICK_WIDTH_PX as usize,
        route_ack_tick_height_px: CLASSIC_FIRST_CONTACT_ROUTE_ACK_TICK_HEIGHT_PX as usize,
        route_clearance_corner_cue_width_px: CLASSIC_FIRST_CONTACT_ROUTE_CLEARANCE_CORNER_CUE_W_PX
            as usize,
        route_clearance_corner_cue_height_px: CLASSIC_FIRST_CONTACT_ROUTE_CLEARANCE_CORNER_CUE_H_PX
            as usize,
        route_clearance_corner_cues_per_tile:
            CLASSIC_FIRST_CONTACT_ROUTE_CLEARANCE_CORNER_CUES_PER_TILE,
        target_lock_cross_long_px: CLASSIC_FIRST_CONTACT_TARGET_LOCK_CROSS_LONG_PX,
        target_lock_cross_thickness_px: CLASSIC_FIRST_CONTACT_TARGET_LOCK_CROSS_THICKNESS_PX,
        target_lock_ack_tick_width_px: CLASSIC_FIRST_CONTACT_TARGET_LOCK_ACK_TICK_W_PX,
        target_lock_ack_tick_height_px: CLASSIC_FIRST_CONTACT_TARGET_LOCK_ACK_TICK_H_PX,
        target_callout_width_px: CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_W_PX,
        target_callout_height_px: CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_H_PX,
        target_callout_x_offset_px: CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_X_OFFSET_PX,
        target_callout_y_offset_px: CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_Y_OFFSET_PX,
        target_callout_health_bar_width_px: CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_HEALTH_BAR_W_PX,
        target_callout_health_bar_height_px: CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_HEALTH_BAR_H_PX,
        target_callout_clearance_pad_px: CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_CLEARANCE_PAD_PX,
        target_callout_leader_clearance_width_px:
            CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_LEADER_CLEARANCE_W_PX,
        target_callout_leader_clearance_height_px:
            CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_LEADER_CLEARANCE_H_PX,
        target_preflight_ring_count: CLASSIC_FIRST_CONTACT_TARGET_PREFLIGHT_RING_COUNT as usize,
        target_preflight_ring_thickness_px:
            CLASSIC_FIRST_CONTACT_TARGET_PREFLIGHT_RING_THICKNESS_PX,
        target_preflight_cross_long_px: CLASSIC_FIRST_CONTACT_TARGET_PREFLIGHT_CROSS_LONG_PX,
        target_preflight_cross_thickness_px:
            CLASSIC_FIRST_CONTACT_TARGET_PREFLIGHT_CROSS_THICKNESS_PX,
    }
}

fn focus_readability_runtime(
    runtime: &NativeFirstPlayableRuntime,
    fallback_target_tile: RtsTile,
    blocked_tile: RtsTile,
) -> trnm_rts_evidence::RtsFirstContactFocusReadabilityRuntime {
    let route_focus_tiles = first_contact_tiles::selection_combat_focus_route_tiles(runtime);
    let route_clearance_tiles =
        first_contact_tiles::route_clearance_tiles(runtime, fallback_target_tile, blocked_tile);
    let focus_corridor_tiles = first_contact_tiles::visual_hierarchy_corridor_tiles(
        runtime,
        fallback_target_tile,
        blocked_tile,
    );
    let route_clearance_overlap_tiles = route_clearance_tiles
        .iter()
        .filter(|tile| focus_corridor_tiles.contains(tile))
        .copied()
        .map(classic_rts_tile_id)
        .collect::<Vec<_>>();
    let target_label = first_contact_readouts::target_callout_label(runtime);

    trnm_rts_evidence::RtsFirstContactFocusReadabilityRuntime {
        selected_focus_tiles: selected_focus_tiles(runtime),
        route_focus_tiles,
        target_focus_tile: target_focus_tile_id(runtime, fallback_target_tile),
        blocked_focus_tile: classic_rts_tile_id((blocked_tile.x, blocked_tile.y)),
        route_clearance_tiles: route_clearance_tiles
            .iter()
            .copied()
            .map(classic_rts_tile_id)
            .collect(),
        route_clearance_overlap_tiles,
        target_subject: first_contact_readouts::target_callout_subject(runtime),
        target_label_width_px: classic_text_advance_px(&target_label, 1),
        target_label,
        target_health_percent: runtime.rts_target_health_percent.min(100),
        geometry: focus_geometry_snapshot(),
    }
}

pub(crate) fn selection_combat_focus_guard(
    runtime: &NativeFirstPlayableRuntime,
    fallback_target_tile: RtsTile,
    blocked_tile: RtsTile,
) -> Value {
    let focus_runtime = focus_readability_runtime(runtime, fallback_target_tile, blocked_tile);
    trnm_rts_evidence::first_contact_selection_combat_focus_guard(&focus_runtime)
}

pub(crate) fn target_callout_guard(
    runtime: &NativeFirstPlayableRuntime,
    fallback_target_tile: RtsTile,
) -> Value {
    let focus_runtime =
        focus_readability_runtime(runtime, fallback_target_tile, RtsTile::new(15, 16));
    trnm_rts_evidence::first_contact_target_callout_guard(&focus_runtime)
}
