#![cfg(not(target_os = "android"))]

use serde_json::{json, Value};
use trnm_rts_core::RtsTile;

use crate::{
    classic_parse_rts_tile, classic_rts_tile_id, classic_text_advance_px, first_contact_readouts,
    first_contact_tiles, NativeFirstPlayableRuntime,
    CLASSIC_FIRST_CONTACT_ROUTE_ACK_TICK_HEIGHT_PX, CLASSIC_FIRST_CONTACT_ROUTE_ACK_TICK_WIDTH_PX,
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
    TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_SELECTION_COMBAT_FOCUS_CONTRACT,
    TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_TARGET_CALLOUT_CONTRACT,
};

fn string_vec<const N: usize>(values: [&str; N]) -> Vec<String> {
    values.into_iter().map(str::to_string).collect()
}

fn tile_ids(tiles: &[(i32, i32)]) -> Vec<String> {
    tiles.iter().copied().map(classic_rts_tile_id).collect()
}

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
    runtime
        .rts_command_destination_tile
        .as_deref()
        .and_then(classic_parse_rts_tile)
        .map(classic_rts_tile_id)
        .unwrap_or_else(|| first_contact_tiles::tile_id(fallback_target_tile))
}

fn focus_layer_draw_order() -> Vec<String> {
    string_vec([
        "terrain",
        "actors",
        "model_identity",
        "silhouette_readability",
        "art_readability",
        "animation_readability",
        "atlas_readability",
        "unit_state",
        "combat_phase",
        "command_feedback",
        "runtime_core",
        "tactical_tracks",
        "opening_actions",
        "readability_overlays",
        "visual_hierarchy_deemphasis",
        "central_clarity_deemphasis",
        "terminal_legibility_deemphasis",
        "selection_combat_focus",
    ])
}

pub(crate) fn selection_combat_focus_guard(
    runtime: &NativeFirstPlayableRuntime,
    fallback_target_tile: RtsTile,
    blocked_tile: RtsTile,
) -> Value {
    let selected_focus_tiles = selected_focus_tiles(runtime);
    let route_focus_tile_pairs = first_contact_tiles::selection_combat_focus_route_tiles(runtime);
    let route_focus_tiles = tile_ids(&route_focus_tile_pairs);
    let target_focus_tile = target_focus_tile_id(runtime, fallback_target_tile);
    let blocked_focus_tile = first_contact_tiles::tile_id(blocked_tile);
    let route_clearance_tiles =
        first_contact_tiles::route_clearance_tiles(runtime, fallback_target_tile, blocked_tile);
    let route_clearance_tile_ids = tile_ids(&route_clearance_tiles);
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
    let focus_signatures = string_vec([
        "selected_corner_brackets",
        "selected_role_badge_ticks",
        "compact_selected_role_badge_ticks",
        "wide_route_dashes",
        "route_ack_step_ticks",
        "compact_route_ack_ticks",
        "route_clearance_gutters",
        "attack_target_lock_brackets",
        "compact_target_lock_cross",
        "blocked_warning_cross",
    ]);
    let route_dash_count = route_focus_tiles.len();
    let route_ack_tick_count = route_focus_tile_pairs
        .windows(2)
        .map(|pair| trnm_rts_bevy_runtime::rts_runtime_tile_line(pair[0], pair[1]).len())
        .sum::<usize>();
    let route_line_step_count = route_dash_count + route_ack_tick_count;
    let selected_role_badge_tick_pixel_budget = selected_focus_tiles.len()
        * (CLASSIC_FIRST_CONTACT_SELECTED_ROLE_BADGE_TICK_W_PX as usize)
        * (CLASSIC_FIRST_CONTACT_SELECTED_ROLE_BADGE_TICK_H_PX as usize);
    let selected_focus_pixel_budget = selected_focus_tiles.len()
        * CLASSIC_FIRST_CONTACT_SELECTED_FOCUS_BRACKET_PIXELS_PER_TILE
        + selected_role_badge_tick_pixel_budget;
    let route_dash_pixel_budget = route_focus_tiles.len()
        * (CLASSIC_FIRST_CONTACT_ROUTE_DASH_WIDTH_PX as usize)
        * (CLASSIC_FIRST_CONTACT_ROUTE_DASH_HEIGHT_PX as usize);
    let route_ack_tick_pixel_budget = route_ack_tick_count
        * (CLASSIC_FIRST_CONTACT_ROUTE_ACK_TICK_WIDTH_PX as usize)
        * (CLASSIC_FIRST_CONTACT_ROUTE_ACK_TICK_HEIGHT_PX as usize);
    let route_focus_pixel_budget = route_dash_pixel_budget + route_ack_tick_pixel_budget;
    let route_clearance_pixel_budget = route_clearance_tiles.len() * 88;
    let route_clearance_edge_pixel_budget = route_clearance_tiles.len() * 16;
    let combat_target_cross_pixel_budget = (CLASSIC_FIRST_CONTACT_TARGET_LOCK_CROSS_LONG_PX
        * CLASSIC_FIRST_CONTACT_TARGET_LOCK_CROSS_THICKNESS_PX
        * 2) as usize;
    let combat_target_ack_tick_pixel_budget = (CLASSIC_FIRST_CONTACT_TARGET_LOCK_ACK_TICK_W_PX
        * CLASSIC_FIRST_CONTACT_TARGET_LOCK_ACK_TICK_H_PX)
        as usize;
    let combat_target_pixel_budget =
        combat_target_cross_pixel_budget + combat_target_ack_tick_pixel_budget;
    let blocked_warning_pixel_budget = 84;
    let selected_focus_gate = selected_focus_tiles
        == string_vec(["14,11", "15,11", "15,12", "17,12"])
        && CLASSIC_FIRST_CONTACT_SELECTED_ROLE_BADGE_TICK_W_PX == 6
        && CLASSIC_FIRST_CONTACT_SELECTED_ROLE_BADGE_TICK_H_PX == 3
        && selected_role_badge_tick_pixel_budget <= 72
        && (320..=328).contains(&selected_focus_pixel_budget);
    let route_focus_gate = route_focus_tiles == string_vec(["14,11", "15,11", "16,10", "16,9"])
        && route_dash_count >= 4
        && route_ack_tick_count >= 6
        && route_line_step_count >= 10
        && CLASSIC_FIRST_CONTACT_ROUTE_DASH_WIDTH_PX == 16
        && CLASSIC_FIRST_CONTACT_ROUTE_DASH_HEIGHT_PX == 3
        && CLASSIC_FIRST_CONTACT_ROUTE_ACK_TICK_WIDTH_PX == 8
        && CLASSIC_FIRST_CONTACT_ROUTE_ACK_TICK_HEIGHT_PX == 2
        && route_ack_tick_pixel_budget <= 96
        && route_focus_pixel_budget <= 288;
    let route_clearance_gate = route_clearance_tile_ids
        == string_vec([
            "13,11", "14,10", "14,12", "15,9", "15,10", "16,8", "16,11", "17,9", "17,10",
        ])
        && route_clearance_overlap_tiles.is_empty()
        && route_clearance_pixel_budget >= 792
        && route_clearance_edge_pixel_budget >= 144;
    let combat_target_focus_gate = target_focus_tile == "16,9"
        && CLASSIC_FIRST_CONTACT_TARGET_LOCK_CROSS_LONG_PX == 28
        && CLASSIC_FIRST_CONTACT_TARGET_LOCK_CROSS_THICKNESS_PX == 3
        && CLASSIC_FIRST_CONTACT_TARGET_LOCK_ACK_TICK_W_PX == 18
        && CLASSIC_FIRST_CONTACT_TARGET_LOCK_ACK_TICK_H_PX == 4
        && (160..=168).contains(&combat_target_cross_pixel_budget)
        && combat_target_ack_tick_pixel_budget <= 72
        && combat_target_pixel_budget <= 240;
    let blocked_warning_focus_gate =
        blocked_focus_tile == "15,16" && blocked_warning_pixel_budget >= 72;
    let focus_signature_gate = focus_signatures.len() == 10
        && focus_signatures
            .iter()
            .any(|signature| signature.as_str() == "attack_target_lock_brackets")
        && focus_signatures
            .iter()
            .any(|signature| signature.as_str() == "compact_selected_role_badge_ticks")
        && focus_signatures
            .iter()
            .any(|signature| signature.as_str() == "route_clearance_gutters")
        && focus_signatures
            .iter()
            .any(|signature| signature.as_str() == "compact_route_ack_ticks")
        && focus_signatures
            .iter()
            .any(|signature| signature.as_str() == "compact_target_lock_cross")
        && focus_signatures
            .iter()
            .any(|signature| signature.as_str() == "blocked_warning_cross");
    let focus_layer_draw_order = focus_layer_draw_order();
    let focus_layer_order_gate = focus_layer_draw_order
        .iter()
        .position(|layer| layer == "atlas_readability")
        < focus_layer_draw_order
            .iter()
            .position(|layer| layer == "selection_combat_focus")
        && focus_layer_draw_order
            .iter()
            .position(|layer| layer == "readability_overlays")
            < focus_layer_draw_order
                .iter()
                .position(|layer| layer == "visual_hierarchy_deemphasis")
        && focus_layer_draw_order
            .iter()
            .position(|layer| layer == "visual_hierarchy_deemphasis")
            < focus_layer_draw_order
                .iter()
                .position(|layer| layer == "central_clarity_deemphasis")
        && focus_layer_draw_order
            .iter()
            .position(|layer| layer == "central_clarity_deemphasis")
            < focus_layer_draw_order
                .iter()
                .position(|layer| layer == "terminal_legibility_deemphasis")
        && focus_layer_draw_order
            .iter()
            .position(|layer| layer == "terminal_legibility_deemphasis")
            < focus_layer_draw_order
                .iter()
                .position(|layer| layer == "selection_combat_focus")
        && focus_layer_draw_order.last().map(String::as_str) == Some("selection_combat_focus");
    let selection_combat_focus_readability_gate = selected_focus_gate
        && route_focus_gate
        && route_clearance_gate
        && combat_target_focus_gate
        && blocked_warning_focus_gate
        && focus_signature_gate
        && focus_layer_order_gate;

    json!({
        "contract_version": TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_SELECTION_COMBAT_FOCUS_CONTRACT,
        "green": selection_combat_focus_readability_gate,
        "source_path": "trnm-world-bevy classic_draw_first_contact_selection_combat_focus_layer after atlas/motion/readability overlays",
        "selected_focus_tiles": selected_focus_tiles,
        "route_focus_tiles": route_focus_tiles,
        "target_focus_tile": target_focus_tile,
        "blocked_focus_tile": blocked_focus_tile,
        "route_clearance_tiles": route_clearance_tile_ids,
        "route_clearance_overlap_tiles": route_clearance_overlap_tiles,
        "focus_signatures": focus_signatures,
        "route_dash_count": route_dash_count,
        "route_ack_tick_count": route_ack_tick_count,
        "route_line_step_count": route_line_step_count,
        "route_dash_width_px": CLASSIC_FIRST_CONTACT_ROUTE_DASH_WIDTH_PX,
        "route_dash_height_px": CLASSIC_FIRST_CONTACT_ROUTE_DASH_HEIGHT_PX,
        "route_ack_tick_width_px": CLASSIC_FIRST_CONTACT_ROUTE_ACK_TICK_WIDTH_PX,
        "route_ack_tick_height_px": CLASSIC_FIRST_CONTACT_ROUTE_ACK_TICK_HEIGHT_PX,
        "selected_role_badge_tick_width_px": CLASSIC_FIRST_CONTACT_SELECTED_ROLE_BADGE_TICK_W_PX,
        "selected_role_badge_tick_height_px": CLASSIC_FIRST_CONTACT_SELECTED_ROLE_BADGE_TICK_H_PX,
        "selected_role_badge_tick_pixel_budget": selected_role_badge_tick_pixel_budget,
        "selected_focus_pixel_budget": selected_focus_pixel_budget,
        "route_dash_pixel_budget": route_dash_pixel_budget,
        "route_ack_tick_pixel_budget": route_ack_tick_pixel_budget,
        "route_focus_pixel_budget": route_focus_pixel_budget,
        "route_clearance_pixel_budget": route_clearance_pixel_budget,
        "route_clearance_edge_pixel_budget": route_clearance_edge_pixel_budget,
        "combat_target_cross_long_px": CLASSIC_FIRST_CONTACT_TARGET_LOCK_CROSS_LONG_PX,
        "combat_target_cross_thickness_px": CLASSIC_FIRST_CONTACT_TARGET_LOCK_CROSS_THICKNESS_PX,
        "combat_target_ack_tick_width_px": CLASSIC_FIRST_CONTACT_TARGET_LOCK_ACK_TICK_W_PX,
        "combat_target_ack_tick_height_px": CLASSIC_FIRST_CONTACT_TARGET_LOCK_ACK_TICK_H_PX,
        "combat_target_cross_pixel_budget": combat_target_cross_pixel_budget,
        "combat_target_ack_tick_pixel_budget": combat_target_ack_tick_pixel_budget,
        "combat_target_pixel_budget": combat_target_pixel_budget,
        "blocked_warning_pixel_budget": blocked_warning_pixel_budget,
        "selected_focus_gate": selected_focus_gate,
        "route_focus_gate": route_focus_gate,
        "route_clearance_gate": route_clearance_gate,
        "combat_target_focus_gate": combat_target_focus_gate,
        "blocked_warning_focus_gate": blocked_warning_focus_gate,
        "focus_signature_gate": focus_signature_gate,
        "focus_layer_draw_order": focus_layer_draw_order,
        "focus_layer_order_gate": focus_layer_order_gate,
        "selection_combat_focus_readability_gate": selection_combat_focus_readability_gate,
    })
}

pub(crate) fn target_callout_guard(
    runtime: &NativeFirstPlayableRuntime,
    fallback_target_tile: RtsTile,
) -> Value {
    let target_tile = classic_rts_tile_id(first_contact_tiles::target_callout_tile(
        runtime,
        fallback_target_tile,
    ));
    let target_subject = first_contact_readouts::target_callout_subject(runtime);
    let target_label = first_contact_readouts::target_callout_label(runtime);
    let target_label_width_px = classic_text_advance_px(&target_label, 1);
    let target_health_percent = runtime.rts_target_health_percent.min(100);
    let target_health_fill_px = (i32::from(target_health_percent)
        * CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_HEALTH_BAR_W_PX)
        / 100;
    let target_callout_pixel_budget = (CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_W_PX
        * CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_H_PX) as usize;
    let target_callout_leader_pixel_budget = 64_usize;
    let target_callout_clearance_pixel_budget = ((CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_W_PX
        + CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_CLEARANCE_PAD_PX * 2)
        * (CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_H_PX
            + CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_CLEARANCE_PAD_PX * 2)
        + CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_LEADER_CLEARANCE_W_PX
            * CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_LEADER_CLEARANCE_H_PX)
        as usize;
    let target_prefocus_ring_count = CLASSIC_FIRST_CONTACT_TARGET_PREFLIGHT_RING_COUNT as usize;
    let target_prefocus_marker_pixel_budget = target_prefocus_ring_count * 96
        + (CLASSIC_FIRST_CONTACT_TARGET_PREFLIGHT_CROSS_LONG_PX
            * CLASSIC_FIRST_CONTACT_TARGET_PREFLIGHT_CROSS_THICKNESS_PX
            * 2) as usize;
    let target_callout_signatures = string_vec([
        "target_callout_clearance_gutter",
        "target_subject_label",
        "target_health_strip",
        "short_leader_ticks",
        "prefocus_target_rings_capped",
        "prefocus_target_cross_thinned",
        "target_lock_preserved",
    ]);
    let target_callout_layer_draw_order = focus_layer_draw_order();
    let target_label_gate = target_tile == "16,9"
        && target_subject == "BEACON"
        && target_label == "BEACON 38%"
        && target_label_width_px <= CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_W_PX - 10;
    let target_health_gate = target_health_percent == 38
        && target_health_fill_px == 20
        && CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_HEALTH_BAR_W_PX == 54
        && CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_HEALTH_BAR_H_PX == 3;
    let target_geometry_gate = CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_W_PX == 78
        && CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_H_PX == 20
        && CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_X_OFFSET_PX == 42
        && CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_Y_OFFSET_PX == -42
        && target_callout_pixel_budget >= 1_560;
    let target_leader_gate = target_callout_leader_pixel_budget >= 64;
    let target_callout_clearance_gate = CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_CLEARANCE_PAD_PX == 5
        && CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_LEADER_CLEARANCE_W_PX == 34
        && CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_LEADER_CLEARANCE_H_PX == 6
        && target_callout_clearance_pixel_budget >= 2_844;
    let target_prefocus_marker_gate = target_prefocus_ring_count == 2
        && CLASSIC_FIRST_CONTACT_TARGET_PREFLIGHT_RING_THICKNESS_PX == 2
        && CLASSIC_FIRST_CONTACT_TARGET_PREFLIGHT_CROSS_LONG_PX == 16
        && CLASSIC_FIRST_CONTACT_TARGET_PREFLIGHT_CROSS_THICKNESS_PX == 2
        && target_prefocus_marker_pixel_budget <= 256;
    let target_signature_gate = target_callout_signatures.len() == 7
        && target_callout_signatures
            .iter()
            .any(|signature| signature == "target_callout_clearance_gutter")
        && target_callout_signatures
            .iter()
            .any(|signature| signature == "target_subject_label")
        && target_callout_signatures
            .iter()
            .any(|signature| signature == "prefocus_target_rings_capped")
        && target_callout_signatures
            .iter()
            .any(|signature| signature == "target_lock_preserved");
    let target_layer_order_gate = target_callout_layer_draw_order
        .iter()
        .position(|layer| layer == "terminal_legibility_deemphasis")
        < target_callout_layer_draw_order
            .iter()
            .position(|layer| layer == "selection_combat_focus")
        && target_callout_layer_draw_order.last().map(String::as_str)
            == Some("selection_combat_focus");
    let target_callout_gate = target_label_gate
        && target_health_gate
        && target_geometry_gate
        && target_leader_gate
        && target_callout_clearance_gate
        && target_prefocus_marker_gate
        && target_signature_gate
        && target_layer_order_gate;

    json!({
        "contract_version": TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_TARGET_CALLOUT_CONTRACT,
        "green": target_callout_gate,
        "source_path": "trnm-world-bevy classic_draw_first_contact_selection_combat_focus_layer target callout with clearance gutter inside final focus layer",
        "target_tile": target_tile,
        "target_subject": target_subject,
        "target_label": target_label,
        "target_label_width_px": target_label_width_px,
        "target_health_percent": target_health_percent,
        "target_health_fill_px": target_health_fill_px,
        "target_callout_width_px": CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_W_PX,
        "target_callout_height_px": CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_H_PX,
        "target_callout_x_offset_px": CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_X_OFFSET_PX,
        "target_callout_y_offset_px": CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_Y_OFFSET_PX,
        "target_callout_health_bar_width_px": CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_HEALTH_BAR_W_PX,
        "target_callout_health_bar_height_px": CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_HEALTH_BAR_H_PX,
        "target_callout_pixel_budget": target_callout_pixel_budget,
        "target_callout_leader_pixel_budget": target_callout_leader_pixel_budget,
        "target_callout_clearance_pad_px": CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_CLEARANCE_PAD_PX,
        "target_callout_leader_clearance_width_px": CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_LEADER_CLEARANCE_W_PX,
        "target_callout_leader_clearance_height_px": CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_LEADER_CLEARANCE_H_PX,
        "target_callout_clearance_pixel_budget": target_callout_clearance_pixel_budget,
        "target_prefocus_ring_count": target_prefocus_ring_count,
        "target_prefocus_ring_thickness_px": CLASSIC_FIRST_CONTACT_TARGET_PREFLIGHT_RING_THICKNESS_PX,
        "target_prefocus_cross_long_px": CLASSIC_FIRST_CONTACT_TARGET_PREFLIGHT_CROSS_LONG_PX,
        "target_prefocus_cross_thickness_px": CLASSIC_FIRST_CONTACT_TARGET_PREFLIGHT_CROSS_THICKNESS_PX,
        "target_prefocus_marker_pixel_budget": target_prefocus_marker_pixel_budget,
        "target_callout_signatures": target_callout_signatures,
        "target_callout_layer_draw_order": target_callout_layer_draw_order,
        "target_label_gate": target_label_gate,
        "target_health_gate": target_health_gate,
        "target_geometry_gate": target_geometry_gate,
        "target_leader_gate": target_leader_gate,
        "target_callout_clearance_gate": target_callout_clearance_gate,
        "target_prefocus_marker_gate": target_prefocus_marker_gate,
        "target_signature_gate": target_signature_gate,
        "target_layer_order_gate": target_layer_order_gate,
        "target_callout_gate": target_callout_gate,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn first_contact_focus_runtime() -> NativeFirstPlayableRuntime {
        NativeFirstPlayableRuntime {
            rts_selection_box_tile_ids: vec![
                "14,11".to_string(),
                "15,11".to_string(),
                "15,12".to_string(),
                "17,12".to_string(),
            ],
            rts_group_route_tile_ids: vec![
                "14,11".to_string(),
                "15,11".to_string(),
                "16,10".to_string(),
                "16,9".to_string(),
            ],
            rts_command_destination_tile: Some("16,9".to_string()),
            rts_attack_target_id: Some("trnm.flux.beacon".to_string()),
            rts_target_health_percent: 38,
            ..Default::default()
        }
    }

    #[test]
    fn first_contact_focus_helpers_preserve_focus_and_callout_contracts() {
        let runtime = first_contact_focus_runtime();
        let target = RtsTile::new(16, 9);
        let blocked = RtsTile::new(15, 16);

        let focus_guard = selection_combat_focus_guard(&runtime, target, blocked);
        assert_eq!(
            focus_guard.get("green").and_then(Value::as_bool),
            Some(true)
        );
        assert_eq!(
            focus_guard.get("route_focus_tiles").cloned(),
            Some(json!(["14,11", "15,11", "16,10", "16,9"]))
        );
        assert_eq!(
            focus_guard
                .get("route_clearance_tiles")
                .and_then(Value::as_array)
                .map(Vec::len),
            Some(9)
        );
        assert_eq!(
            focus_guard
                .get("combat_target_pixel_budget")
                .and_then(Value::as_u64),
            Some(240)
        );
        assert_eq!(
            focus_guard
                .get("selection_combat_focus_readability_gate")
                .and_then(Value::as_bool),
            Some(true)
        );

        let callout_guard = target_callout_guard(&runtime, target);
        assert_eq!(
            callout_guard.get("green").and_then(Value::as_bool),
            Some(true)
        );
        assert_eq!(
            callout_guard.get("target_label").and_then(Value::as_str),
            Some("BEACON 38%")
        );
        assert_eq!(
            callout_guard
                .get("target_prefocus_marker_pixel_budget")
                .and_then(Value::as_u64),
            Some(256)
        );
        assert_eq!(
            callout_guard
                .get("target_callout_gate")
                .and_then(Value::as_bool),
            Some(true)
        );
    }
}
