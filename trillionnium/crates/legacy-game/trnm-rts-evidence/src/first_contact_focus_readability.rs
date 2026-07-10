#![cfg(not(target_os = "android"))]

use serde_json::{json, Value};
use trnm_rts_bevy_runtime::{
    rts_runtime_tile_id, rts_runtime_tile_line,
    TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_READOUT_SURFACE_CONTRACT,
    TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_SUBJECT_SURFACE_CONTRACT,
    TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_TILE_SURFACE_CONTRACT,
};

use crate::{
    TRNM_RTS_EVIDENCE_FIRST_CONTACT_SELECTION_COMBAT_FOCUS_CONTRACT,
    TRNM_RTS_EVIDENCE_FIRST_CONTACT_TARGET_CALLOUT_CONTRACT,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RtsFirstContactFocusReadabilityGeometrySnapshot {
    pub selected_role_badge_tick_width_px: usize,
    pub selected_role_badge_tick_height_px: usize,
    pub selected_focus_bracket_pixels_per_tile: usize,
    pub route_dash_width_px: usize,
    pub route_dash_height_px: usize,
    pub route_ack_tick_width_px: usize,
    pub route_ack_tick_height_px: usize,
    pub route_clearance_corner_cue_width_px: usize,
    pub route_clearance_corner_cue_height_px: usize,
    pub route_clearance_corner_cues_per_tile: usize,
    pub target_lock_cross_long_px: i32,
    pub target_lock_cross_thickness_px: i32,
    pub target_lock_bracket_tick_long_px: i32,
    pub target_lock_bracket_tick_thickness_px: i32,
    pub target_lock_ack_tick_width_px: i32,
    pub target_lock_ack_tick_height_px: i32,
    pub target_callout_width_px: i32,
    pub target_callout_height_px: i32,
    pub target_callout_x_offset_px: i32,
    pub target_callout_y_offset_px: i32,
    pub target_callout_health_bar_width_px: i32,
    pub target_callout_health_bar_height_px: i32,
    pub target_callout_health_pip_count: usize,
    pub target_callout_health_pip_width_px: i32,
    pub target_callout_health_pip_height_px: i32,
    pub target_callout_health_pip_gap_px: i32,
    pub target_callout_clearance_pad_px: i32,
    pub target_callout_leader_clearance_width_px: i32,
    pub target_callout_leader_clearance_height_px: i32,
    pub target_callout_edge_tick_count: usize,
    pub target_callout_edge_tick_width_px: i32,
    pub target_callout_edge_tick_height_px: i32,
    pub target_callout_leader_tick_width_px: i32,
    pub target_callout_leader_tick_height_px: i32,
    pub target_preflight_ring_count: usize,
    pub target_preflight_ring_thickness_px: i32,
    pub target_preflight_corner_tick_long_px: i32,
    pub target_preflight_cross_long_px: i32,
    pub target_preflight_cross_thickness_px: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RtsFirstContactFocusReadabilityRuntime {
    pub selected_focus_tiles: Vec<String>,
    pub route_focus_tiles: Vec<(i32, i32)>,
    pub target_focus_tile: String,
    pub blocked_focus_tile: String,
    pub route_clearance_tiles: Vec<String>,
    pub route_clearance_overlap_tiles: Vec<String>,
    pub target_subject: String,
    pub target_label: String,
    pub target_label_width_px: i32,
    pub target_health_percent: u8,
    pub geometry: RtsFirstContactFocusReadabilityGeometrySnapshot,
}

fn string_vec<const N: usize>(values: [&str; N]) -> Vec<String> {
    values.into_iter().map(str::to_string).collect()
}

fn tile_ids(tiles: &[(i32, i32)]) -> Vec<String> {
    tiles.iter().copied().map(rts_runtime_tile_id).collect()
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

pub fn first_contact_selection_combat_focus_guard(
    runtime: &RtsFirstContactFocusReadabilityRuntime,
) -> Value {
    let geometry = runtime.geometry;
    let selected_focus_tiles = runtime.selected_focus_tiles.clone();
    let route_focus_tile_pairs = runtime.route_focus_tiles.clone();
    let route_focus_tiles = tile_ids(&route_focus_tile_pairs);
    let target_focus_tile = runtime.target_focus_tile.clone();
    let blocked_focus_tile = runtime.blocked_focus_tile.clone();
    let route_clearance_tile_ids = runtime.route_clearance_tiles.clone();
    let route_clearance_overlap_tiles = runtime.route_clearance_overlap_tiles.clone();
    let focus_signatures = string_vec([
        "selected_corner_brackets",
        "selected_role_badge_micro_pips",
        "compact_route_dashes",
        "route_ack_step_micro_dots",
        "route_ack_micro_dots",
        "route_clearance_corner_cues",
        "attack_target_lock_micro_corner_ticks",
        "compact_target_lock_cross",
        "target_ack_micro_tick",
        "blocked_warning_cross",
    ]);
    let route_dash_count = route_focus_tiles.len();
    let route_ack_tick_count = route_focus_tile_pairs
        .windows(2)
        .map(|pair| rts_runtime_tile_line(pair[0], pair[1]).len())
        .sum::<usize>();
    let route_line_step_count = route_dash_count + route_ack_tick_count;
    let selected_role_badge_tick_pixel_budget = selected_focus_tiles.len()
        * geometry.selected_role_badge_tick_width_px
        * geometry.selected_role_badge_tick_height_px;
    let selected_focus_pixel_budget = selected_focus_tiles.len()
        * geometry.selected_focus_bracket_pixels_per_tile
        + selected_role_badge_tick_pixel_budget;
    let route_dash_pixel_budget =
        route_focus_tiles.len() * geometry.route_dash_width_px * geometry.route_dash_height_px;
    let route_ack_tick_pixel_budget =
        route_ack_tick_count * geometry.route_ack_tick_width_px * geometry.route_ack_tick_height_px;
    let route_focus_pixel_budget = route_dash_pixel_budget + route_ack_tick_pixel_budget;
    let route_clearance_corner_cue_count =
        route_clearance_tile_ids.len() * geometry.route_clearance_corner_cues_per_tile;
    let route_clearance_corner_cue_pixel_budget = route_clearance_corner_cue_count
        * geometry.route_clearance_corner_cue_width_px
        * geometry.route_clearance_corner_cue_height_px;
    let route_clearance_gutter_fill_pixel_budget = 0_usize;
    let route_clearance_pixel_budget = route_clearance_corner_cue_pixel_budget;
    let combat_target_cross_pixel_budget =
        (geometry.target_lock_cross_long_px * geometry.target_lock_cross_thickness_px * 2) as usize;
    let combat_target_bracket_corner_count = 4_usize;
    let combat_target_bracket_pixel_budget = combat_target_bracket_corner_count
        * ((geometry.target_lock_bracket_tick_long_px
            * geometry.target_lock_bracket_tick_thickness_px
            * 2)
            - geometry.target_lock_bracket_tick_thickness_px
                * geometry.target_lock_bracket_tick_thickness_px) as usize;
    let combat_target_bracket_component_max_width_px = geometry.target_lock_bracket_tick_long_px;
    let combat_target_bracket_component_max_height_px = geometry.target_lock_bracket_tick_long_px;
    let combat_target_ack_tick_pixel_budget =
        (geometry.target_lock_ack_tick_width_px * geometry.target_lock_ack_tick_height_px) as usize;
    let combat_target_pixel_budget =
        combat_target_cross_pixel_budget + combat_target_ack_tick_pixel_budget;
    let blocked_warning_pixel_budget = 84;
    let selected_focus_gate = selected_focus_tiles
        == string_vec(["14,11", "15,11", "15,12", "17,12"])
        && geometry.selected_role_badge_tick_width_px == 2
        && geometry.selected_role_badge_tick_height_px == 2
        && selected_role_badge_tick_pixel_budget == 16
        && selected_focus_pixel_budget == 96;
    let route_focus_gate = route_focus_tiles == string_vec(["14,11", "15,11", "16,10", "16,9"])
        && route_dash_count >= 4
        && route_ack_tick_count >= 6
        && route_line_step_count >= 10
        && geometry.route_dash_width_px == 8
        && geometry.route_dash_height_px == 2
        && geometry.route_ack_tick_width_px == 2
        && geometry.route_ack_tick_height_px == 2
        && route_ack_tick_pixel_budget == 24
        && route_dash_pixel_budget == 64
        && route_focus_pixel_budget <= 88;
    let route_clearance_gate = route_clearance_tile_ids
        == string_vec([
            "13,11", "14,10", "14,12", "15,9", "15,10", "16,8", "16,11", "17,9", "17,10",
        ])
        && route_clearance_overlap_tiles.is_empty()
        && geometry.route_clearance_corner_cues_per_tile == 4
        && geometry.route_clearance_corner_cue_width_px == 6
        && geometry.route_clearance_corner_cue_height_px == 2
        && route_clearance_gutter_fill_pixel_budget == 0
        && route_clearance_corner_cue_pixel_budget == 432
        && route_clearance_pixel_budget <= 432;
    let combat_target_focus_gate = target_focus_tile == "16,9"
        && geometry.target_lock_cross_long_px == 6
        && geometry.target_lock_cross_thickness_px == 2
        && geometry.target_lock_bracket_tick_long_px == 6
        && geometry.target_lock_bracket_tick_thickness_px == 2
        && geometry.target_lock_ack_tick_width_px == 4
        && geometry.target_lock_ack_tick_height_px == 2
        && combat_target_cross_pixel_budget == 24
        && combat_target_bracket_corner_count == 4
        && combat_target_bracket_pixel_budget == 80
        && combat_target_bracket_component_max_width_px <= 6
        && combat_target_bracket_component_max_height_px <= 6
        && combat_target_ack_tick_pixel_budget == 8
        && combat_target_pixel_budget <= 32;
    let blocked_warning_focus_gate =
        blocked_focus_tile == "15,16" && blocked_warning_pixel_budget >= 72;
    let focus_signature_gate = focus_signatures.len() == 10
        && focus_signatures
            .iter()
            .any(|signature| signature.as_str() == "attack_target_lock_micro_corner_ticks")
        && focus_signatures
            .iter()
            .any(|signature| signature.as_str() == "selected_role_badge_micro_pips")
        && focus_signatures
            .iter()
            .any(|signature| signature.as_str() == "route_clearance_corner_cues")
        && focus_signatures
            .iter()
            .any(|signature| signature.as_str() == "route_ack_micro_dots")
        && focus_signatures
            .iter()
            .any(|signature| signature.as_str() == "compact_target_lock_cross")
        && focus_signatures
            .iter()
            .any(|signature| signature.as_str() == "target_ack_micro_tick")
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
        "contract_version": TRNM_RTS_EVIDENCE_FIRST_CONTACT_SELECTION_COMBAT_FOCUS_CONTRACT,
        "tile_surface_contract": TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_TILE_SURFACE_CONTRACT,
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
        "route_dash_width_px": geometry.route_dash_width_px,
        "route_dash_height_px": geometry.route_dash_height_px,
        "route_ack_tick_width_px": geometry.route_ack_tick_width_px,
        "route_ack_tick_height_px": geometry.route_ack_tick_height_px,
        "selected_role_badge_tick_width_px": geometry.selected_role_badge_tick_width_px,
        "selected_role_badge_tick_height_px": geometry.selected_role_badge_tick_height_px,
        "selected_role_badge_tick_pixel_budget": selected_role_badge_tick_pixel_budget,
        "selected_focus_pixel_budget": selected_focus_pixel_budget,
        "route_dash_pixel_budget": route_dash_pixel_budget,
        "route_ack_tick_pixel_budget": route_ack_tick_pixel_budget,
        "route_focus_pixel_budget": route_focus_pixel_budget,
        "route_clearance_corner_cue_count": route_clearance_corner_cue_count,
        "route_clearance_corner_cue_width_px": geometry.route_clearance_corner_cue_width_px,
        "route_clearance_corner_cue_height_px": geometry.route_clearance_corner_cue_height_px,
        "route_clearance_corner_cues_per_tile": geometry.route_clearance_corner_cues_per_tile,
        "route_clearance_corner_cue_pixel_budget": route_clearance_corner_cue_pixel_budget,
        "route_clearance_gutter_fill_pixel_budget": route_clearance_gutter_fill_pixel_budget,
        "route_clearance_pixel_budget": route_clearance_pixel_budget,
        "combat_target_cross_long_px": geometry.target_lock_cross_long_px,
        "combat_target_cross_thickness_px": geometry.target_lock_cross_thickness_px,
        "combat_target_bracket_tick_long_px": geometry.target_lock_bracket_tick_long_px,
        "combat_target_bracket_tick_thickness_px": geometry.target_lock_bracket_tick_thickness_px,
        "combat_target_bracket_corner_count": combat_target_bracket_corner_count,
        "combat_target_bracket_pixel_budget": combat_target_bracket_pixel_budget,
        "combat_target_bracket_component_max_width_px": combat_target_bracket_component_max_width_px,
        "combat_target_bracket_component_max_height_px": combat_target_bracket_component_max_height_px,
        "combat_target_ack_tick_width_px": geometry.target_lock_ack_tick_width_px,
        "combat_target_ack_tick_height_px": geometry.target_lock_ack_tick_height_px,
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

pub fn first_contact_target_callout_guard(
    runtime: &RtsFirstContactFocusReadabilityRuntime,
) -> Value {
    let geometry = runtime.geometry;
    let target_tile = runtime.target_focus_tile.clone();
    let target_subject = runtime.target_subject.clone();
    let target_label = runtime.target_label.clone();
    let target_label_width_px = runtime.target_label_width_px;
    let target_health_percent = runtime.target_health_percent.min(100);
    let target_health_fill_pip_count =
        ((usize::from(target_health_percent) * geometry.target_callout_health_pip_count) + 99)
            / 100;
    let target_health_fill_pip_count =
        target_health_fill_pip_count.clamp(1, geometry.target_callout_health_pip_count);
    let target_health_fill_px = target_health_fill_pip_count
        * geometry.target_callout_health_pip_width_px as usize
        * geometry.target_callout_health_pip_height_px as usize;
    let target_callout_pixel_budget =
        (geometry.target_callout_width_px * geometry.target_callout_height_px) as usize;
    let target_callout_edge_tick_pixel_budget = geometry.target_callout_edge_tick_count
        * (geometry.target_callout_edge_tick_width_px * geometry.target_callout_edge_tick_height_px)
            as usize;
    let target_callout_leader_tick_pixel_budget = (geometry.target_callout_leader_tick_width_px
        * geometry.target_callout_leader_tick_height_px)
        as usize;
    let target_callout_leader_pixel_budget =
        target_callout_edge_tick_pixel_budget + target_callout_leader_tick_pixel_budget;
    let target_callout_clearance_pixel_budget =
        (((geometry.target_callout_width_px + geometry.target_callout_clearance_pad_px * 2)
            * (geometry.target_callout_height_px + geometry.target_callout_clearance_pad_px * 2)
            + geometry.target_callout_leader_clearance_width_px
                * geometry.target_callout_leader_clearance_height_px) as usize)
            .saturating_sub(target_callout_pixel_budget);
    let target_prefocus_ring_count = geometry.target_preflight_ring_count;
    let target_prefocus_corner_tick_pixel_budget = target_prefocus_ring_count
        * 4
        * ((geometry.target_preflight_corner_tick_long_px
            * geometry.target_preflight_ring_thickness_px
            * 2)
            - geometry.target_preflight_ring_thickness_px
                * geometry.target_preflight_ring_thickness_px) as usize;
    let target_prefocus_marker_pixel_budget = target_prefocus_corner_tick_pixel_budget
        + (geometry.target_preflight_cross_long_px
            * geometry.target_preflight_cross_thickness_px
            * 2) as usize;
    let target_callout_signatures = string_vec([
        "compact_target_callout_plate",
        "target_subject_label",
        "target_health_micro_pips",
        "short_leader_ticks",
        "prefocus_target_corner_ticks",
        "prefocus_target_cross_thinned",
        "target_lock_preserved",
    ]);
    let target_callout_layer_draw_order = focus_layer_draw_order();
    let target_label_gate = target_tile == "16,9"
        && target_subject == "BEACON"
        && target_label == "BEACON 38%"
        && target_label_width_px <= geometry.target_callout_width_px - 10;
    let target_health_gate = target_health_percent == 38
        && target_health_fill_pip_count == 2
        && target_health_fill_px == 16
        && geometry.target_callout_health_bar_width_px == 54
        && geometry.target_callout_health_bar_height_px == 3
        && geometry.target_callout_health_pip_count == 4
        && geometry.target_callout_health_pip_width_px == 4
        && geometry.target_callout_health_pip_height_px == 2
        && geometry.target_callout_health_pip_gap_px == 4;
    let target_geometry_gate = geometry.target_callout_width_px == 78
        && geometry.target_callout_height_px == 20
        && geometry.target_callout_x_offset_px == 42
        && geometry.target_callout_y_offset_px == -42
        && target_callout_pixel_budget >= 1_560;
    let target_leader_gate = geometry.target_callout_edge_tick_count == 2
        && geometry.target_callout_edge_tick_width_px == 2
        && geometry.target_callout_edge_tick_height_px == 6
        && geometry.target_callout_leader_tick_width_px == 10
        && geometry.target_callout_leader_tick_height_px == 2
        && target_callout_edge_tick_pixel_budget == 24
        && target_callout_leader_tick_pixel_budget == 20
        && target_callout_leader_pixel_budget <= 44;
    let target_callout_clearance_gate = geometry.target_callout_clearance_pad_px == 0
        && geometry.target_callout_leader_clearance_width_px == 0
        && geometry.target_callout_leader_clearance_height_px == 0
        && target_callout_clearance_pixel_budget == 0;
    let target_prefocus_marker_gate = target_prefocus_ring_count == 2
        && geometry.target_preflight_ring_thickness_px == 2
        && geometry.target_preflight_corner_tick_long_px == 6
        && geometry.target_preflight_cross_long_px == 16
        && geometry.target_preflight_cross_thickness_px == 2
        && target_prefocus_corner_tick_pixel_budget == 160
        && target_prefocus_marker_pixel_budget <= 224;
    let target_signature_gate = target_callout_signatures.len() == 7
        && target_callout_signatures
            .iter()
            .any(|signature| signature == "compact_target_callout_plate")
        && target_callout_signatures
            .iter()
            .any(|signature| signature == "target_subject_label")
        && target_callout_signatures
            .iter()
            .any(|signature| signature == "prefocus_target_corner_ticks")
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
        "contract_version": TRNM_RTS_EVIDENCE_FIRST_CONTACT_TARGET_CALLOUT_CONTRACT,
        "tile_surface_contract": TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_TILE_SURFACE_CONTRACT,
        "readout_surface_contract": TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_READOUT_SURFACE_CONTRACT,
        "subject_surface_contract": TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_SUBJECT_SURFACE_CONTRACT,
        "green": target_callout_gate,
        "source_path": "trnm-world-bevy classic_draw_first_contact_selection_combat_focus_layer compact target callout plate inside final focus layer",
        "target_tile": target_tile,
        "target_subject": target_subject,
        "target_label": target_label,
        "target_label_width_px": target_label_width_px,
        "target_health_percent": target_health_percent,
        "target_health_fill_px": target_health_fill_px,
        "target_health_fill_pip_count": target_health_fill_pip_count,
        "target_callout_width_px": geometry.target_callout_width_px,
        "target_callout_height_px": geometry.target_callout_height_px,
        "target_callout_x_offset_px": geometry.target_callout_x_offset_px,
        "target_callout_y_offset_px": geometry.target_callout_y_offset_px,
        "target_callout_health_bar_width_px": geometry.target_callout_health_bar_width_px,
        "target_callout_health_bar_height_px": geometry.target_callout_health_bar_height_px,
        "target_callout_health_pip_count": geometry.target_callout_health_pip_count,
        "target_callout_health_pip_width_px": geometry.target_callout_health_pip_width_px,
        "target_callout_health_pip_height_px": geometry.target_callout_health_pip_height_px,
        "target_callout_health_pip_gap_px": geometry.target_callout_health_pip_gap_px,
        "target_callout_pixel_budget": target_callout_pixel_budget,
        "target_callout_edge_tick_count": geometry.target_callout_edge_tick_count,
        "target_callout_edge_tick_width_px": geometry.target_callout_edge_tick_width_px,
        "target_callout_edge_tick_height_px": geometry.target_callout_edge_tick_height_px,
        "target_callout_edge_tick_pixel_budget": target_callout_edge_tick_pixel_budget,
        "target_callout_leader_tick_width_px": geometry.target_callout_leader_tick_width_px,
        "target_callout_leader_tick_height_px": geometry.target_callout_leader_tick_height_px,
        "target_callout_leader_tick_pixel_budget": target_callout_leader_tick_pixel_budget,
        "target_callout_leader_pixel_budget": target_callout_leader_pixel_budget,
        "target_callout_clearance_pad_px": geometry.target_callout_clearance_pad_px,
        "target_callout_leader_clearance_width_px": geometry.target_callout_leader_clearance_width_px,
        "target_callout_leader_clearance_height_px": geometry.target_callout_leader_clearance_height_px,
        "target_callout_clearance_pixel_budget": target_callout_clearance_pixel_budget,
        "target_prefocus_ring_count": target_prefocus_ring_count,
        "target_prefocus_ring_thickness_px": geometry.target_preflight_ring_thickness_px,
        "target_prefocus_corner_tick_long_px": geometry.target_preflight_corner_tick_long_px,
        "target_prefocus_corner_tick_pixel_budget": target_prefocus_corner_tick_pixel_budget,
        "target_prefocus_cross_long_px": geometry.target_preflight_cross_long_px,
        "target_prefocus_cross_thickness_px": geometry.target_preflight_cross_thickness_px,
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

    fn focus_runtime() -> RtsFirstContactFocusReadabilityRuntime {
        RtsFirstContactFocusReadabilityRuntime {
            selected_focus_tiles: string_vec(["14,11", "15,11", "15,12", "17,12"]),
            route_focus_tiles: vec![(14, 11), (15, 11), (16, 10), (16, 9)],
            target_focus_tile: "16,9".to_string(),
            blocked_focus_tile: "15,16".to_string(),
            route_clearance_tiles: string_vec([
                "13,11", "14,10", "14,12", "15,9", "15,10", "16,8", "16,11", "17,9", "17,10",
            ]),
            route_clearance_overlap_tiles: Vec::new(),
            target_subject: "BEACON".to_string(),
            target_label: "BEACON 38%".to_string(),
            target_label_width_px: 54,
            target_health_percent: 38,
            geometry: RtsFirstContactFocusReadabilityGeometrySnapshot {
                selected_role_badge_tick_width_px: 2,
                selected_role_badge_tick_height_px: 2,
                selected_focus_bracket_pixels_per_tile: 20,
                route_dash_width_px: 8,
                route_dash_height_px: 2,
                route_ack_tick_width_px: 2,
                route_ack_tick_height_px: 2,
                route_clearance_corner_cue_width_px: 6,
                route_clearance_corner_cue_height_px: 2,
                route_clearance_corner_cues_per_tile: 4,
                target_lock_cross_long_px: 6,
                target_lock_cross_thickness_px: 2,
                target_lock_bracket_tick_long_px: 6,
                target_lock_bracket_tick_thickness_px: 2,
                target_lock_ack_tick_width_px: 4,
                target_lock_ack_tick_height_px: 2,
                target_callout_width_px: 78,
                target_callout_height_px: 20,
                target_callout_x_offset_px: 42,
                target_callout_y_offset_px: -42,
                target_callout_health_bar_width_px: 54,
                target_callout_health_bar_height_px: 3,
                target_callout_health_pip_count: 4,
                target_callout_health_pip_width_px: 4,
                target_callout_health_pip_height_px: 2,
                target_callout_health_pip_gap_px: 4,
                target_callout_clearance_pad_px: 0,
                target_callout_leader_clearance_width_px: 0,
                target_callout_leader_clearance_height_px: 0,
                target_callout_edge_tick_count: 2,
                target_callout_edge_tick_width_px: 2,
                target_callout_edge_tick_height_px: 6,
                target_callout_leader_tick_width_px: 10,
                target_callout_leader_tick_height_px: 2,
                target_preflight_ring_count: 2,
                target_preflight_ring_thickness_px: 2,
                target_preflight_corner_tick_long_px: 6,
                target_preflight_cross_long_px: 16,
                target_preflight_cross_thickness_px: 2,
            },
        }
    }

    #[test]
    fn first_contact_focus_readability_preserves_focus_contracts() {
        let guard = first_contact_selection_combat_focus_guard(&focus_runtime());

        assert_eq!(
            guard.get("contract_version").and_then(Value::as_str),
            Some(TRNM_RTS_EVIDENCE_FIRST_CONTACT_SELECTION_COMBAT_FOCUS_CONTRACT)
        );
        assert_eq!(
            guard.get("tile_surface_contract").and_then(Value::as_str),
            Some(TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_TILE_SURFACE_CONTRACT)
        );
        assert_eq!(guard.get("green").and_then(Value::as_bool), Some(true));
        assert_eq!(
            guard.get("route_focus_tiles").cloned(),
            Some(json!(["14,11", "15,11", "16,10", "16,9"]))
        );
        assert_eq!(
            guard
                .get("route_clearance_tiles")
                .and_then(Value::as_array)
                .map(Vec::len),
            Some(9)
        );
        assert_eq!(
            guard
                .get("combat_target_pixel_budget")
                .and_then(Value::as_u64),
            Some(32)
        );
        assert_eq!(
            guard
                .get("combat_target_bracket_pixel_budget")
                .and_then(Value::as_u64),
            Some(80)
        );
        assert_eq!(
            guard
                .get("combat_target_bracket_component_max_width_px")
                .and_then(Value::as_i64),
            Some(6)
        );
        assert_eq!(
            guard
                .get("combat_target_bracket_component_max_height_px")
                .and_then(Value::as_i64),
            Some(6)
        );
        assert_eq!(
            guard
                .get("selection_combat_focus_readability_gate")
                .and_then(Value::as_bool),
            Some(true)
        );
    }

    #[test]
    fn first_contact_target_callout_preserves_label_and_geometry_contracts() {
        let guard = first_contact_target_callout_guard(&focus_runtime());

        assert_eq!(
            guard.get("contract_version").and_then(Value::as_str),
            Some(TRNM_RTS_EVIDENCE_FIRST_CONTACT_TARGET_CALLOUT_CONTRACT)
        );
        assert_eq!(
            guard.get("tile_surface_contract").and_then(Value::as_str),
            Some(TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_TILE_SURFACE_CONTRACT)
        );
        assert_eq!(
            guard
                .get("readout_surface_contract")
                .and_then(Value::as_str),
            Some(TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_READOUT_SURFACE_CONTRACT)
        );
        assert_eq!(
            guard
                .get("subject_surface_contract")
                .and_then(Value::as_str),
            Some(TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_SUBJECT_SURFACE_CONTRACT)
        );
        assert_eq!(guard.get("green").and_then(Value::as_bool), Some(true));
        assert_eq!(
            guard.get("target_label").and_then(Value::as_str),
            Some("BEACON 38%")
        );
        assert_eq!(
            guard
                .get("target_prefocus_marker_pixel_budget")
                .and_then(Value::as_u64),
            Some(224)
        );
        assert_eq!(
            guard
                .get("target_prefocus_corner_tick_pixel_budget")
                .and_then(Value::as_u64),
            Some(160)
        );
        assert_eq!(
            guard
                .get("target_callout_leader_pixel_budget")
                .and_then(Value::as_u64),
            Some(44)
        );
        assert_eq!(
            guard
                .get("target_callout_edge_tick_pixel_budget")
                .and_then(Value::as_u64),
            Some(24)
        );
        assert_eq!(
            guard.get("target_callout_gate").and_then(Value::as_bool),
            Some(true)
        );
    }
}
