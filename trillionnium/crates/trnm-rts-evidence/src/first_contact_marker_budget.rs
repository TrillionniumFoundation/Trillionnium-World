#![cfg(not(target_os = "android"))]

use serde_json::{json, Value};
use trnm_rts_bevy_runtime::{
    self as rts_bevy_runtime, rts_runtime_tile_id,
    TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_TILE_SURFACE_CONTRACT,
};
use trnm_rts_data::first_contact_samples::{self, AtlasSample};

use crate::{
    TRNM_RTS_EVIDENCE_FIRST_CONTACT_GALLERY_DARKEN_DENOMINATOR,
    TRNM_RTS_EVIDENCE_FIRST_CONTACT_GALLERY_DARKEN_NUMERATOR,
    TRNM_RTS_EVIDENCE_FIRST_CONTACT_LOWER_LANE_GALLERY_DARKEN_DENOMINATOR,
    TRNM_RTS_EVIDENCE_FIRST_CONTACT_LOWER_LANE_GALLERY_DARKEN_NUMERATOR,
    TRNM_RTS_EVIDENCE_FIRST_CONTACT_MARKER_BUDGET_CONTRACT,
};

const GALLERY_SLOT_CUE_PIXELS_PER_SAMPLE: usize = 1;
const LOWER_LANE_SLOT_CUE_PIXELS_PER_SAMPLE: usize = 1;
const LOWER_LANE_GHOST_ANCHOR_COUNT: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RtsFirstContactFocusGeometrySnapshot {
    pub selected_role_badge_tick_width_px: usize,
    pub selected_role_badge_tick_height_px: usize,
    pub selected_focus_bracket_pixels_per_tile: usize,
    pub route_dash_width_px: usize,
    pub route_dash_height_px: usize,
    pub route_ack_tick_width_px: usize,
    pub route_ack_tick_height_px: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RtsFirstContactMarkerBudgetRuntime {
    pub selected_tiles: Vec<(i32, i32)>,
    pub route_tiles: Vec<(i32, i32)>,
    pub frame_pixel_areas: Vec<(String, usize)>,
    pub tactical_track_count: usize,
    pub player_screen_tactical_track_count: usize,
    pub player_screen_primary_tactical_track_count: usize,
    pub player_screen_secondary_tactical_track_count: usize,
    pub hover_route_path_marker_count: usize,
    pub player_screen_hover_route_path_marker_count: usize,
    pub non_focus_owner_identity_colors: Vec<String>,
    pub non_focus_owner_identity_hot_color_count: usize,
    pub non_focus_owner_identity_pixel_budget: usize,
    pub player_screen_relay_identity_rail_count: usize,
    pub player_screen_relay_identity_rail_width_px: usize,
    pub player_screen_relay_identity_rail_height_px: usize,
    pub player_screen_legacy_midfield_status_bar_pixel_budget: usize,
    pub focus_geometry: RtsFirstContactFocusGeometrySnapshot,
}

struct GalleryBudgetSummary {
    gallery_lanes: Vec<&'static str>,
    busy_core_tiles: Vec<(i32, i32)>,
    lower_lane_gallery_tiles: Vec<(i32, i32)>,
    north_gallery_frame_count: usize,
    west_gallery_frame_count: usize,
    east_gallery_frame_count: usize,
    max_gallery_lane_frame_count: usize,
    muted_gallery_sample_count: usize,
    gallery_mute_overlay_pixel_budget: usize,
    gallery_slot_cue_pixel_budget: usize,
    gallery_darken_numerator: usize,
    gallery_darken_denominator: usize,
    lower_lane_gallery_sample_count: usize,
    lower_lane_rendered_frame_pixel_budget: usize,
    lower_lane_frame_suppressed_count: usize,
    lower_lane_mute_overlay_pixel_budget: usize,
    lower_lane_slot_cue_pixel_budget: usize,
    lower_lane_ghost_anchor_count: usize,
    lower_lane_gallery_darken_numerator: usize,
    lower_lane_gallery_darken_denominator: usize,
    lower_lane_dim_silhouette_pixel_budget: usize,
    lower_lane_shadow_suppressed_count: usize,
    gallery_hot_marker_color_count: usize,
    lower_lane_hot_marker_color_count: usize,
    interactive_hot_marker_role_count: usize,
    gallery_presentation_signatures: Vec<&'static str>,
}

fn gallery_budget_summary<F>(
    family_samples: &[AtlasSample],
    mut frame_pixel_area: F,
) -> GalleryBudgetSummary
where
    F: FnMut(&str, u32) -> usize,
{
    let gallery_lanes = family_samples
        .iter()
        .map(|(tile, _, _, _, _)| first_contact_samples::atlas_family_gallery_lane(*tile))
        .collect::<Vec<_>>();
    let busy_core_tiles = family_samples
        .iter()
        .filter(|(tile, _, _, _, _)| first_contact_samples::atlas_family_busy_core_tile(*tile))
        .map(|(tile, _, _, _, _)| *tile)
        .collect::<Vec<_>>();
    let lower_lane_gallery_tiles = family_samples
        .iter()
        .filter(|(tile, _, _, _, _)| first_contact_samples::atlas_family_lower_lane_tile(*tile))
        .map(|(tile, _, _, _, _)| *tile)
        .collect::<Vec<_>>();
    let north_gallery_frame_count = gallery_lanes
        .iter()
        .filter(|lane| **lane == "north_gallery")
        .count();
    let west_gallery_frame_count = gallery_lanes
        .iter()
        .filter(|lane| **lane == "west_gallery")
        .count();
    let east_gallery_frame_count = gallery_lanes
        .iter()
        .filter(|lane| **lane == "east_gallery")
        .count();
    let max_gallery_lane_frame_count = [
        north_gallery_frame_count,
        west_gallery_frame_count,
        east_gallery_frame_count,
    ]
    .into_iter()
    .max()
    .unwrap_or(0);
    let muted_gallery_sample_count = family_samples.len();
    let gallery_darken_numerator =
        TRNM_RTS_EVIDENCE_FIRST_CONTACT_GALLERY_DARKEN_NUMERATOR as usize;
    let gallery_darken_denominator =
        TRNM_RTS_EVIDENCE_FIRST_CONTACT_GALLERY_DARKEN_DENOMINATOR as usize;
    let gallery_mute_overlay_pixel_budget = family_samples
        .iter()
        .filter(|(tile, _, _, _, _)| !first_contact_samples::atlas_family_lower_lane_tile(*tile))
        .map(|(_, _, frame_id, _, scale)| {
            frame_pixel_area(frame_id, *scale) * gallery_darken_numerator
                / gallery_darken_denominator.max(1)
        })
        .sum::<usize>();
    let lower_lane_gallery_sample_count = lower_lane_gallery_tiles.len();
    let perimeter_gallery_sample_count = family_samples
        .len()
        .saturating_sub(lower_lane_gallery_sample_count);
    let gallery_slot_cue_pixel_budget = perimeter_gallery_sample_count
        * GALLERY_SLOT_CUE_PIXELS_PER_SAMPLE
        + lower_lane_gallery_sample_count * LOWER_LANE_SLOT_CUE_PIXELS_PER_SAMPLE;
    let lower_lane_rendered_frame_pixel_budget = 0;
    let lower_lane_frame_suppressed_count = lower_lane_gallery_sample_count;
    let lower_lane_mute_overlay_pixel_budget = 0;
    let lower_lane_slot_cue_pixel_budget =
        lower_lane_gallery_sample_count * LOWER_LANE_SLOT_CUE_PIXELS_PER_SAMPLE;
    let lower_lane_ghost_anchor_count =
        lower_lane_gallery_sample_count * LOWER_LANE_GHOST_ANCHOR_COUNT;
    let lower_lane_gallery_darken_numerator =
        TRNM_RTS_EVIDENCE_FIRST_CONTACT_LOWER_LANE_GALLERY_DARKEN_NUMERATOR as usize;
    let lower_lane_gallery_darken_denominator =
        TRNM_RTS_EVIDENCE_FIRST_CONTACT_LOWER_LANE_GALLERY_DARKEN_DENOMINATOR as usize;
    let lower_lane_dim_silhouette_pixel_budget = 0;

    GalleryBudgetSummary {
        gallery_lanes,
        busy_core_tiles,
        lower_lane_gallery_tiles,
        north_gallery_frame_count,
        west_gallery_frame_count,
        east_gallery_frame_count,
        max_gallery_lane_frame_count,
        muted_gallery_sample_count,
        gallery_mute_overlay_pixel_budget,
        gallery_slot_cue_pixel_budget,
        gallery_darken_numerator,
        gallery_darken_denominator,
        lower_lane_gallery_sample_count,
        lower_lane_rendered_frame_pixel_budget,
        lower_lane_frame_suppressed_count,
        lower_lane_mute_overlay_pixel_budget,
        lower_lane_slot_cue_pixel_budget,
        lower_lane_ghost_anchor_count,
        lower_lane_gallery_darken_numerator,
        lower_lane_gallery_darken_denominator,
        lower_lane_dim_silhouette_pixel_budget,
        lower_lane_shadow_suppressed_count: lower_lane_gallery_sample_count,
        gallery_hot_marker_color_count: 0,
        lower_lane_hot_marker_color_count: 0,
        interactive_hot_marker_role_count: 5,
        gallery_presentation_signatures: vec![
            "muted_gallery_slot_cues",
            "perimeter_gallery_edge_anchors",
            "perimeter_gallery_single_pixel_anchors",
            "darkened_gallery_frames",
            "perimeter_gallery_stronger_deemphasis",
            "lower_lane_gallery_deemphasis",
            "lower_lane_micro_slot_cues",
            "lower_lane_frame_suppressed",
            "lower_lane_anchor_only",
            "lower_lane_shadow_suppressed",
            "lower_lane_ghost_anchors",
            "lower_lane_single_point_ghost_anchors",
            "compact_route_dashes",
            "route_ack_micro_dots",
            "perimeter_gallery_lane_budget",
            "interactive_focus_kept_hot",
            "non_focus_owner_identity_muted",
            "player_screen_primary_tactical_track_only",
            "player_screen_hover_route_path_suppressed",
            "player_screen_relay_identity_micro_rails",
            "player_screen_legacy_midfield_status_bars_suppressed",
        ],
    }
}

fn string_vec<const N: usize>(values: [&str; N]) -> Vec<String> {
    values.into_iter().map(str::to_string).collect()
}

fn tile_ids(tiles: &[(i32, i32)]) -> Vec<String> {
    tiles.iter().copied().map(rts_runtime_tile_id).collect()
}

fn runtime_frame_pixel_area(
    runtime: &RtsFirstContactMarkerBudgetRuntime,
    frame_id: &str,
    scale: u32,
) -> usize {
    runtime
        .frame_pixel_areas
        .iter()
        .find_map(|(id, area)| (id == frame_id).then_some(*area))
        .unwrap_or_else(|| {
            let frame_px = 16_usize * scale.max(1) as usize;
            frame_px * frame_px
        })
}

pub fn first_contact_marker_budget_guard(runtime: &RtsFirstContactMarkerBudgetRuntime) -> Value {
    let family_samples = first_contact_samples::atlas_frame_family_samples();
    let gallery_summary = gallery_budget_summary(&family_samples, |frame_id, scale| {
        runtime_frame_pixel_area(runtime, frame_id, scale)
    });
    let gallery_lanes = gallery_summary
        .gallery_lanes
        .iter()
        .map(|lane| (*lane).to_string())
        .collect::<Vec<_>>();
    let busy_core_tiles = tile_ids(&gallery_summary.busy_core_tiles);
    let lower_lane_gallery_tiles = tile_ids(&gallery_summary.lower_lane_gallery_tiles);
    let north_gallery_frame_count = gallery_summary.north_gallery_frame_count;
    let west_gallery_frame_count = gallery_summary.west_gallery_frame_count;
    let east_gallery_frame_count = gallery_summary.east_gallery_frame_count;
    let max_gallery_lane_frame_count = gallery_summary.max_gallery_lane_frame_count;
    let muted_gallery_sample_count = gallery_summary.muted_gallery_sample_count;
    let gallery_mute_overlay_pixel_budget = gallery_summary.gallery_mute_overlay_pixel_budget;
    let gallery_slot_cue_pixel_budget = gallery_summary.gallery_slot_cue_pixel_budget;
    let gallery_darken_numerator = gallery_summary.gallery_darken_numerator;
    let gallery_darken_denominator = gallery_summary.gallery_darken_denominator;
    let lower_lane_gallery_sample_count = gallery_summary.lower_lane_gallery_sample_count;
    let lower_lane_rendered_frame_pixel_budget =
        gallery_summary.lower_lane_rendered_frame_pixel_budget;
    let lower_lane_frame_suppressed_count = gallery_summary.lower_lane_frame_suppressed_count;
    let lower_lane_mute_overlay_pixel_budget = gallery_summary.lower_lane_mute_overlay_pixel_budget;
    let lower_lane_slot_cue_pixel_budget = gallery_summary.lower_lane_slot_cue_pixel_budget;
    let lower_lane_ghost_anchor_count = gallery_summary.lower_lane_ghost_anchor_count;
    let lower_lane_gallery_darken_numerator = gallery_summary.lower_lane_gallery_darken_numerator;
    let lower_lane_gallery_darken_denominator =
        gallery_summary.lower_lane_gallery_darken_denominator;
    let lower_lane_dim_silhouette_pixel_budget =
        gallery_summary.lower_lane_dim_silhouette_pixel_budget;
    let lower_lane_shadow_suppressed_count = gallery_summary.lower_lane_shadow_suppressed_count;
    let gallery_hot_marker_color_count = gallery_summary.gallery_hot_marker_color_count;
    let lower_lane_hot_marker_color_count = gallery_summary.lower_lane_hot_marker_color_count;
    let interactive_hot_marker_role_count = gallery_summary.interactive_hot_marker_role_count;
    let tactical_track_count = runtime.tactical_track_count;
    let player_screen_tactical_track_count = runtime.player_screen_tactical_track_count;
    let player_screen_primary_tactical_track_count =
        runtime.player_screen_primary_tactical_track_count;
    let player_screen_secondary_tactical_track_count =
        runtime.player_screen_secondary_tactical_track_count;
    let hover_route_path_marker_count = runtime.hover_route_path_marker_count;
    let player_screen_hover_route_path_marker_count =
        runtime.player_screen_hover_route_path_marker_count;
    let non_focus_owner_identity_colors = runtime.non_focus_owner_identity_colors.clone();
    let non_focus_owner_identity_hot_color_count = runtime.non_focus_owner_identity_hot_color_count;
    let non_focus_owner_identity_pixel_budget = runtime.non_focus_owner_identity_pixel_budget;
    let player_screen_relay_identity_rail_count = runtime.player_screen_relay_identity_rail_count;
    let player_screen_relay_identity_rail_width_px =
        runtime.player_screen_relay_identity_rail_width_px;
    let player_screen_relay_identity_rail_height_px =
        runtime.player_screen_relay_identity_rail_height_px;
    let player_screen_relay_identity_rail_pixel_budget = player_screen_relay_identity_rail_count
        * player_screen_relay_identity_rail_width_px
        * player_screen_relay_identity_rail_height_px;
    let player_screen_legacy_midfield_status_bar_pixel_budget =
        runtime.player_screen_legacy_midfield_status_bar_pixel_budget;
    let gallery_presentation_signatures = gallery_summary
        .gallery_presentation_signatures
        .iter()
        .map(|signature| (*signature).to_string())
        .collect::<Vec<_>>();
    let selected_focus_tiles = tile_ids(&runtime.selected_tiles);
    let route_focus_tile_pairs = runtime.route_tiles.clone();
    let route_focus_tiles = tile_ids(&route_focus_tile_pairs);
    let route_ack_tick_count = route_focus_tile_pairs
        .windows(2)
        .map(|pair| rts_bevy_runtime::rts_runtime_tile_line(pair[0], pair[1]).len())
        .sum::<usize>();
    let selected_role_badge_tick_pixel_budget = selected_focus_tiles.len()
        * runtime.focus_geometry.selected_role_badge_tick_width_px
        * runtime.focus_geometry.selected_role_badge_tick_height_px;
    let selected_focus_pixel_budget = selected_focus_tiles.len()
        * runtime
            .focus_geometry
            .selected_focus_bracket_pixels_per_tile
        + selected_role_badge_tick_pixel_budget;
    let route_focus_pixel_budget = route_focus_tiles.len()
        * runtime.focus_geometry.route_dash_width_px
        * runtime.focus_geometry.route_dash_height_px
        + route_ack_tick_count
            * runtime.focus_geometry.route_ack_tick_width_px
            * runtime.focus_geometry.route_ack_tick_height_px;
    let combat_target_pixel_budget = 192_usize;
    let blocked_warning_pixel_budget = 84_usize;
    let interactive_focus_pixel_budget = selected_focus_pixel_budget
        + route_focus_pixel_budget
        + combat_target_pixel_budget
        + blocked_warning_pixel_budget;
    let marker_budget_layer_draw_order = string_vec([
        "atlas_readability",
        "atlas_gallery_muted",
        "visual_hierarchy_deemphasis",
        "central_clarity_deemphasis",
        "terminal_legibility_deemphasis",
        "selection_combat_focus",
    ]);
    let gallery_lane_budget_gate = family_samples.len() == 14
        && busy_core_tiles.is_empty()
        && west_gallery_frame_count <= 4
        && north_gallery_frame_count <= 4
        && east_gallery_frame_count <= 6
        && max_gallery_lane_frame_count <= 6;
    let gallery_mute_gate = muted_gallery_sample_count == family_samples.len()
        && gallery_mute_overlay_pixel_budget >= 22_000
        && gallery_slot_cue_pixel_budget <= 14
        && gallery_darken_numerator == 4
        && gallery_darken_denominator == 5
        && gallery_hot_marker_color_count == 0
        && gallery_presentation_signatures
            .iter()
            .any(|signature| signature == "darkened_gallery_frames")
        && gallery_presentation_signatures
            .iter()
            .any(|signature| signature == "perimeter_gallery_stronger_deemphasis");
    let lower_lane_gallery_deemphasis_gate = lower_lane_gallery_tiles
        == string_vec(["29,22", "29,24", "29,26"])
        && lower_lane_gallery_sample_count == 3
        && lower_lane_rendered_frame_pixel_budget == 0
        && lower_lane_frame_suppressed_count == 3
        && lower_lane_mute_overlay_pixel_budget == 0
        && lower_lane_slot_cue_pixel_budget <= 3
        && lower_lane_ghost_anchor_count == 3
        && lower_lane_gallery_darken_numerator == 5
        && lower_lane_gallery_darken_denominator == 6
        && lower_lane_dim_silhouette_pixel_budget == 0
        && lower_lane_shadow_suppressed_count == 3
        && lower_lane_hot_marker_color_count == 0
        && gallery_presentation_signatures
            .iter()
            .any(|signature| signature == "lower_lane_gallery_deemphasis")
        && gallery_presentation_signatures
            .iter()
            .any(|signature| signature == "lower_lane_micro_slot_cues")
        && gallery_presentation_signatures
            .iter()
            .any(|signature| signature == "lower_lane_frame_suppressed")
        && gallery_presentation_signatures
            .iter()
            .any(|signature| signature == "lower_lane_anchor_only")
        && gallery_presentation_signatures
            .iter()
            .any(|signature| signature == "lower_lane_shadow_suppressed")
        && gallery_presentation_signatures
            .iter()
            .any(|signature| signature == "lower_lane_ghost_anchors")
        && gallery_presentation_signatures
            .iter()
            .any(|signature| signature == "lower_lane_single_point_ghost_anchors");
    let interactive_focus_preservation_gate = selected_focus_tiles
        == string_vec(["14,11", "15,11", "15,12", "17,12"])
        && route_focus_tiles == string_vec(["14,11", "15,11", "16,10", "16,9"])
        && interactive_hot_marker_role_count >= 5
        && selected_role_badge_tick_pixel_budget == 16
        && interactive_focus_pixel_budget == 636
        && gallery_presentation_signatures
            .iter()
            .any(|signature| signature == "compact_route_dashes");
    let non_focus_owner_identity_gate = non_focus_owner_identity_colors.len() == 2
        && non_focus_owner_identity_hot_color_count == 0
        && non_focus_owner_identity_pixel_budget <= 192
        && !non_focus_owner_identity_colors
            .iter()
            .any(|color| color == "67c980" || color == "d47967")
        && gallery_presentation_signatures
            .iter()
            .any(|signature| signature == "non_focus_owner_identity_muted");
    let player_screen_tactical_track_gate = tactical_track_count == 6
        && player_screen_tactical_track_count == 1
        && player_screen_primary_tactical_track_count == 1
        && player_screen_secondary_tactical_track_count == 0
        && gallery_presentation_signatures
            .iter()
            .any(|signature| signature == "player_screen_primary_tactical_track_only");
    let player_screen_hover_route_path_gate = hover_route_path_marker_count > 0
        && player_screen_hover_route_path_marker_count == 0
        && gallery_presentation_signatures
            .iter()
            .any(|signature| signature == "player_screen_hover_route_path_suppressed");
    let player_screen_relay_identity_rail_gate = player_screen_relay_identity_rail_count == 6
        && player_screen_relay_identity_rail_width_px == 16
        && player_screen_relay_identity_rail_height_px == 2
        && player_screen_relay_identity_rail_pixel_budget <= 192
        && gallery_presentation_signatures
            .iter()
            .any(|signature| signature == "player_screen_relay_identity_micro_rails");
    let player_screen_legacy_midfield_status_bar_gate =
        player_screen_legacy_midfield_status_bar_pixel_budget == 0
            && gallery_presentation_signatures.iter().any(|signature| {
                signature == "player_screen_legacy_midfield_status_bars_suppressed"
            });
    let marker_budget_layer_order_gate = marker_budget_layer_draw_order
        .iter()
        .position(|layer| layer == "atlas_gallery_muted")
        < marker_budget_layer_draw_order
            .iter()
            .position(|layer| layer == "visual_hierarchy_deemphasis")
        && marker_budget_layer_draw_order
            .iter()
            .position(|layer| layer == "visual_hierarchy_deemphasis")
            < marker_budget_layer_draw_order
                .iter()
                .position(|layer| layer == "central_clarity_deemphasis")
        && marker_budget_layer_draw_order
            .iter()
            .position(|layer| layer == "central_clarity_deemphasis")
            < marker_budget_layer_draw_order
                .iter()
                .position(|layer| layer == "terminal_legibility_deemphasis")
        && marker_budget_layer_draw_order
            .iter()
            .position(|layer| layer == "terminal_legibility_deemphasis")
            < marker_budget_layer_draw_order
                .iter()
                .position(|layer| layer == "selection_combat_focus")
        && marker_budget_layer_draw_order.last().map(String::as_str)
            == Some("selection_combat_focus");
    let first_contact_marker_budget_gate = gallery_lane_budget_gate
        && gallery_mute_gate
        && lower_lane_gallery_deemphasis_gate
        && interactive_focus_preservation_gate
        && non_focus_owner_identity_gate
        && player_screen_tactical_track_gate
        && player_screen_hover_route_path_gate
        && player_screen_relay_identity_rail_gate
        && player_screen_legacy_midfield_status_bar_gate
        && marker_budget_layer_order_gate;

    json!({
        "contract_version": TRNM_RTS_EVIDENCE_FIRST_CONTACT_MARKER_BUDGET_CONTRACT,
        "tile_surface_contract": TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_TILE_SURFACE_CONTRACT,
        "green": first_contact_marker_budget_gate,
        "source_path": "trnm-world-bevy muted First Contact atlas gallery presentation plus player-screen tactical/hover marker budgets and final selection/combat focus layer",
        "gallery_sample_count": family_samples.len(),
        "muted_gallery_sample_count": muted_gallery_sample_count,
        "gallery_lanes": gallery_lanes,
        "busy_core_tiles": busy_core_tiles,
        "lower_lane_gallery_tiles": lower_lane_gallery_tiles,
        "north_gallery_frame_count": north_gallery_frame_count,
        "west_gallery_frame_count": west_gallery_frame_count,
        "east_gallery_frame_count": east_gallery_frame_count,
        "max_gallery_lane_frame_count": max_gallery_lane_frame_count,
        "gallery_mute_overlay_pixel_budget": gallery_mute_overlay_pixel_budget,
        "gallery_slot_cue_pixel_budget": gallery_slot_cue_pixel_budget,
        "gallery_darken_numerator": gallery_darken_numerator,
        "gallery_darken_denominator": gallery_darken_denominator,
        "lower_lane_gallery_sample_count": lower_lane_gallery_sample_count,
        "lower_lane_rendered_frame_pixel_budget": lower_lane_rendered_frame_pixel_budget,
        "lower_lane_frame_suppressed_count": lower_lane_frame_suppressed_count,
        "lower_lane_mute_overlay_pixel_budget": lower_lane_mute_overlay_pixel_budget,
        "lower_lane_slot_cue_pixel_budget": lower_lane_slot_cue_pixel_budget,
        "lower_lane_ghost_anchor_count": lower_lane_ghost_anchor_count,
        "lower_lane_gallery_darken_numerator": lower_lane_gallery_darken_numerator,
        "lower_lane_gallery_darken_denominator": lower_lane_gallery_darken_denominator,
        "lower_lane_dim_silhouette_pixel_budget": lower_lane_dim_silhouette_pixel_budget,
        "lower_lane_shadow_suppressed_count": lower_lane_shadow_suppressed_count,
        "gallery_hot_marker_color_count": gallery_hot_marker_color_count,
        "lower_lane_hot_marker_color_count": lower_lane_hot_marker_color_count,
        "interactive_hot_marker_role_count": interactive_hot_marker_role_count,
        "tactical_track_count": tactical_track_count,
        "player_screen_tactical_track_count": player_screen_tactical_track_count,
        "player_screen_primary_tactical_track_count": player_screen_primary_tactical_track_count,
        "player_screen_secondary_tactical_track_count": player_screen_secondary_tactical_track_count,
        "hover_route_path_marker_count": hover_route_path_marker_count,
        "player_screen_hover_route_path_marker_count": player_screen_hover_route_path_marker_count,
        "non_focus_owner_identity_colors": non_focus_owner_identity_colors,
        "non_focus_owner_identity_hot_color_count": non_focus_owner_identity_hot_color_count,
        "non_focus_owner_identity_pixel_budget": non_focus_owner_identity_pixel_budget,
        "player_screen_relay_identity_rail_count": player_screen_relay_identity_rail_count,
        "player_screen_relay_identity_rail_width_px": player_screen_relay_identity_rail_width_px,
        "player_screen_relay_identity_rail_height_px": player_screen_relay_identity_rail_height_px,
        "player_screen_relay_identity_rail_pixel_budget": player_screen_relay_identity_rail_pixel_budget,
        "player_screen_legacy_midfield_status_bar_pixel_budget": player_screen_legacy_midfield_status_bar_pixel_budget,
        "selected_role_badge_tick_pixel_budget": selected_role_badge_tick_pixel_budget,
        "selected_focus_tiles": selected_focus_tiles,
        "route_focus_tiles": route_focus_tiles,
        "interactive_focus_pixel_budget": interactive_focus_pixel_budget,
        "gallery_presentation_signatures": gallery_presentation_signatures,
        "marker_budget_layer_draw_order": marker_budget_layer_draw_order,
        "gallery_lane_budget_gate": gallery_lane_budget_gate,
        "gallery_mute_gate": gallery_mute_gate,
        "lower_lane_gallery_deemphasis_gate": lower_lane_gallery_deemphasis_gate,
        "interactive_focus_preservation_gate": interactive_focus_preservation_gate,
        "non_focus_owner_identity_gate": non_focus_owner_identity_gate,
        "player_screen_tactical_track_gate": player_screen_tactical_track_gate,
        "player_screen_hover_route_path_gate": player_screen_hover_route_path_gate,
        "player_screen_relay_identity_rail_gate": player_screen_relay_identity_rail_gate,
        "player_screen_legacy_midfield_status_bar_gate": player_screen_legacy_midfield_status_bar_gate,
        "marker_budget_layer_order_gate": marker_budget_layer_order_gate,
        "first_contact_marker_budget_gate": first_contact_marker_budget_gate,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn focus_geometry() -> RtsFirstContactFocusGeometrySnapshot {
        RtsFirstContactFocusGeometrySnapshot {
            selected_role_badge_tick_width_px: 2,
            selected_role_badge_tick_height_px: 2,
            selected_focus_bracket_pixels_per_tile: 64,
            route_dash_width_px: 8,
            route_dash_height_px: 2,
            route_ack_tick_width_px: 2,
            route_ack_tick_height_px: 2,
        }
    }

    fn marker_budget_runtime() -> RtsFirstContactMarkerBudgetRuntime {
        let mut frame_pixel_areas = first_contact_samples::atlas_frame_family_samples()
            .iter()
            .map(|(_, _, frame_id, _, _)| ((*frame_id).to_string(), 3_000_usize))
            .collect::<Vec<_>>();
        if let Some((_, area)) = frame_pixel_areas.last_mut() {
            *area = 3_496;
        }
        RtsFirstContactMarkerBudgetRuntime {
            selected_tiles: vec![(14, 11), (15, 11), (15, 12), (17, 12)],
            route_tiles: vec![(14, 11), (15, 11), (16, 10), (16, 9)],
            frame_pixel_areas,
            tactical_track_count: 6,
            player_screen_tactical_track_count: 1,
            player_screen_primary_tactical_track_count: 1,
            player_screen_secondary_tactical_track_count: 0,
            hover_route_path_marker_count: 8,
            player_screen_hover_route_path_marker_count: 0,
            non_focus_owner_identity_colors: string_vec(["457953", "6a5e4b"]),
            non_focus_owner_identity_hot_color_count: 0,
            non_focus_owner_identity_pixel_budget: 192,
            player_screen_relay_identity_rail_count: 6,
            player_screen_relay_identity_rail_width_px: 16,
            player_screen_relay_identity_rail_height_px: 2,
            player_screen_legacy_midfield_status_bar_pixel_budget: 0,
            focus_geometry: focus_geometry(),
        }
    }

    #[test]
    fn first_contact_marker_budget_helpers_preserve_gallery_contracts() {
        let samples = first_contact_samples::atlas_frame_family_samples();
        let summary = gallery_budget_summary(&samples, |_, scale| {
            let frame_px = 16_usize * scale.max(1) as usize;
            frame_px * frame_px
        });

        assert_eq!(summary.muted_gallery_sample_count, 14);
        assert_eq!(summary.busy_core_tiles, Vec::<(i32, i32)>::new());
        assert_eq!(
            summary.lower_lane_gallery_tiles,
            vec![(29, 22), (29, 24), (29, 26)]
        );
        assert_eq!(summary.west_gallery_frame_count, 4);
        assert_eq!(summary.north_gallery_frame_count, 4);
        assert_eq!(summary.east_gallery_frame_count, 6);
        assert_eq!(summary.max_gallery_lane_frame_count, 6);
        assert_eq!(summary.lower_lane_gallery_sample_count, 3);
        assert_eq!(summary.gallery_slot_cue_pixel_budget, 14);
        assert_eq!(summary.gallery_darken_numerator, 4);
        assert_eq!(summary.gallery_darken_denominator, 5);
        assert_eq!(summary.lower_lane_rendered_frame_pixel_budget, 0);
        assert_eq!(summary.lower_lane_frame_suppressed_count, 3);
        assert_eq!(summary.lower_lane_mute_overlay_pixel_budget, 0);
        assert_eq!(summary.lower_lane_slot_cue_pixel_budget, 3);
        assert_eq!(summary.lower_lane_ghost_anchor_count, 3);
        assert_eq!(summary.lower_lane_gallery_darken_numerator, 5);
        assert_eq!(summary.lower_lane_gallery_darken_denominator, 6);
        assert_eq!(summary.lower_lane_dim_silhouette_pixel_budget, 0);
        assert_eq!(summary.lower_lane_shadow_suppressed_count, 3);
        assert_eq!(summary.gallery_hot_marker_color_count, 0);
        assert_eq!(summary.lower_lane_hot_marker_color_count, 0);
        assert_eq!(summary.interactive_hot_marker_role_count, 5);
        assert!(summary
            .gallery_presentation_signatures
            .contains(&"player_screen_primary_tactical_track_only"));
        assert!(summary
            .gallery_presentation_signatures
            .contains(&"player_screen_hover_route_path_suppressed"));
        assert!(summary
            .gallery_presentation_signatures
            .contains(&"lower_lane_single_point_ghost_anchors"));
        assert!(summary
            .gallery_presentation_signatures
            .contains(&"lower_lane_frame_suppressed"));
        assert!(summary
            .gallery_presentation_signatures
            .contains(&"lower_lane_anchor_only"));
        assert!(summary
            .gallery_presentation_signatures
            .contains(&"perimeter_gallery_stronger_deemphasis"));
        assert!(summary
            .gallery_presentation_signatures
            .contains(&"perimeter_gallery_edge_anchors"));
        assert!(summary
            .gallery_presentation_signatures
            .contains(&"perimeter_gallery_single_pixel_anchors"));
        assert!(summary
            .gallery_presentation_signatures
            .contains(&"interactive_focus_kept_hot"));
        assert!(summary
            .gallery_presentation_signatures
            .contains(&"non_focus_owner_identity_muted"));
        assert!(summary
            .gallery_presentation_signatures
            .contains(&"player_screen_relay_identity_micro_rails"));
        assert!(summary
            .gallery_presentation_signatures
            .contains(&"player_screen_legacy_midfield_status_bars_suppressed"));
    }

    #[test]
    fn first_contact_marker_budget_guard_preserves_gallery_and_focus_contracts() {
        let guard = first_contact_marker_budget_guard(&marker_budget_runtime());

        assert_eq!(guard.get("green").and_then(Value::as_bool), Some(true));
        assert_eq!(
            guard.get("contract_version").and_then(Value::as_str),
            Some(TRNM_RTS_EVIDENCE_FIRST_CONTACT_MARKER_BUDGET_CONTRACT)
        );
        assert_eq!(
            guard.get("tile_surface_contract").and_then(Value::as_str),
            Some(TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_TILE_SURFACE_CONTRACT)
        );
        assert_eq!(
            guard.get("gallery_sample_count").and_then(Value::as_u64),
            Some(14)
        );
        assert_eq!(guard.get("busy_core_tiles").cloned(), Some(json!([])));
        assert_eq!(
            guard.get("lower_lane_gallery_tiles").cloned(),
            Some(json!(["29,22", "29,24", "29,26"]))
        );
        assert_eq!(
            guard
                .get("lower_lane_slot_cue_pixel_budget")
                .and_then(Value::as_u64),
            Some(3)
        );
        assert_eq!(
            guard
                .get("lower_lane_rendered_frame_pixel_budget")
                .and_then(Value::as_u64),
            Some(0)
        );
        assert_eq!(
            guard
                .get("lower_lane_frame_suppressed_count")
                .and_then(Value::as_u64),
            Some(3)
        );
        assert_eq!(
            guard
                .get("gallery_slot_cue_pixel_budget")
                .and_then(Value::as_u64),
            Some(14)
        );
        assert_eq!(
            guard.get("non_focus_owner_identity_colors").cloned(),
            Some(json!(["457953", "6a5e4b"]))
        );
        assert_eq!(
            guard
                .get("non_focus_owner_identity_hot_color_count")
                .and_then(Value::as_u64),
            Some(0)
        );
        assert_eq!(
            guard
                .get("non_focus_owner_identity_pixel_budget")
                .and_then(Value::as_u64),
            Some(192)
        );
        assert_eq!(
            guard
                .get("player_screen_relay_identity_rail_count")
                .and_then(Value::as_u64),
            Some(6)
        );
        assert_eq!(
            guard
                .get("player_screen_relay_identity_rail_width_px")
                .and_then(Value::as_u64),
            Some(16)
        );
        assert_eq!(
            guard
                .get("player_screen_relay_identity_rail_height_px")
                .and_then(Value::as_u64),
            Some(2)
        );
        assert_eq!(
            guard
                .get("player_screen_relay_identity_rail_pixel_budget")
                .and_then(Value::as_u64),
            Some(192)
        );
        assert_eq!(
            guard
                .get("player_screen_legacy_midfield_status_bar_pixel_budget")
                .and_then(Value::as_u64),
            Some(0)
        );
        assert_eq!(
            guard
                .get("non_focus_owner_identity_gate")
                .and_then(Value::as_bool),
            Some(true)
        );
        assert_eq!(
            guard.get("tactical_track_count").and_then(Value::as_u64),
            Some(6)
        );
        assert_eq!(
            guard
                .get("player_screen_tactical_track_count")
                .and_then(Value::as_u64),
            Some(1)
        );
        assert_eq!(
            guard
                .get("player_screen_primary_tactical_track_count")
                .and_then(Value::as_u64),
            Some(1)
        );
        assert_eq!(
            guard
                .get("player_screen_secondary_tactical_track_count")
                .and_then(Value::as_u64),
            Some(0)
        );
        assert_eq!(
            guard
                .get("player_screen_tactical_track_gate")
                .and_then(Value::as_bool),
            Some(true)
        );
        assert_eq!(
            guard
                .get("hover_route_path_marker_count")
                .and_then(Value::as_u64),
            Some(8)
        );
        assert_eq!(
            guard
                .get("player_screen_hover_route_path_marker_count")
                .and_then(Value::as_u64),
            Some(0)
        );
        assert_eq!(
            guard
                .get("player_screen_hover_route_path_gate")
                .and_then(Value::as_bool),
            Some(true)
        );
        assert_eq!(
            guard
                .get("gallery_darken_numerator")
                .and_then(Value::as_u64),
            Some(4)
        );
        assert_eq!(
            guard
                .get("gallery_darken_denominator")
                .and_then(Value::as_u64),
            Some(5)
        );
        assert_eq!(
            guard
                .get("lower_lane_ghost_anchor_count")
                .and_then(Value::as_u64),
            Some(3)
        );
        assert_eq!(
            guard
                .get("selected_role_badge_tick_pixel_budget")
                .and_then(Value::as_u64),
            Some(16)
        );
        assert_eq!(
            guard.get("selected_focus_tiles").cloned(),
            Some(json!(["14,11", "15,11", "15,12", "17,12"]))
        );
        assert_eq!(
            guard.get("route_focus_tiles").cloned(),
            Some(json!(["14,11", "15,11", "16,10", "16,9"]))
        );
        assert!(guard
            .get("gallery_presentation_signatures")
            .and_then(Value::as_array)
            .is_some_and(|signatures| signatures.iter().any(
                |signature| signature.as_str() == Some("lower_lane_single_point_ghost_anchors")
            )));
        assert!(guard
            .get("gallery_presentation_signatures")
            .and_then(Value::as_array)
            .is_some_and(|signatures| signatures
                .iter()
                .any(|signature| signature.as_str() == Some("lower_lane_frame_suppressed"))));
        assert!(guard
            .get("gallery_presentation_signatures")
            .and_then(Value::as_array)
            .is_some_and(|signatures| signatures
                .iter()
                .any(|signature| signature.as_str() == Some("lower_lane_anchor_only"))));
        assert!(guard
            .get("gallery_presentation_signatures")
            .and_then(Value::as_array)
            .is_some_and(|signatures| signatures.iter().any(
                |signature| signature.as_str() == Some("perimeter_gallery_stronger_deemphasis")
            )));
        assert!(guard
            .get("gallery_presentation_signatures")
            .and_then(Value::as_array)
            .is_some_and(|signatures| signatures
                .iter()
                .any(|signature| signature.as_str()
                    == Some("perimeter_gallery_single_pixel_anchors"))));
        assert!(guard
            .get("gallery_presentation_signatures")
            .and_then(Value::as_array)
            .is_some_and(|signatures| signatures
                .iter()
                .any(|signature| signature.as_str() == Some("non_focus_owner_identity_muted"))));
        assert!(guard
            .get("gallery_presentation_signatures")
            .and_then(Value::as_array)
            .is_some_and(|signatures| signatures
                .iter()
                .any(|signature| signature.as_str()
                    == Some("player_screen_primary_tactical_track_only"))));
        assert!(guard
            .get("gallery_presentation_signatures")
            .and_then(Value::as_array)
            .is_some_and(|signatures| signatures.iter().any(|signature| {
                signature.as_str() == Some("player_screen_hover_route_path_suppressed")
            })));
        assert!(guard
            .get("gallery_presentation_signatures")
            .and_then(Value::as_array)
            .is_some_and(|signatures| signatures.iter().any(|signature| {
                signature.as_str() == Some("player_screen_relay_identity_micro_rails")
            })));
        assert!(guard
            .get("gallery_presentation_signatures")
            .and_then(Value::as_array)
            .is_some_and(|signatures| signatures.iter().any(|signature| {
                signature.as_str() == Some("player_screen_legacy_midfield_status_bars_suppressed")
            })));

        for gate in [
            "gallery_lane_budget_gate",
            "gallery_mute_gate",
            "lower_lane_gallery_deemphasis_gate",
            "interactive_focus_preservation_gate",
            "non_focus_owner_identity_gate",
            "player_screen_tactical_track_gate",
            "player_screen_hover_route_path_gate",
            "player_screen_relay_identity_rail_gate",
            "player_screen_legacy_midfield_status_bar_gate",
            "marker_budget_layer_order_gate",
            "first_contact_marker_budget_gate",
        ] {
            assert_eq!(guard.get(gate).and_then(Value::as_bool), Some(true));
        }
    }
}
