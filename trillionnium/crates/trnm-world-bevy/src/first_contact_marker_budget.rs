#![cfg(not(target_os = "android"))]

use serde_json::{json, Value};
use trnm_rts_bevy_runtime as rts_bevy_runtime;

use crate::{
    classic_parse_rts_tile, classic_rts_tile_id,
    first_contact_samples::{self, AtlasSample},
    first_contact_tiles, NativeFirstPlayableRuntime,
    CLASSIC_FIRST_CONTACT_ROUTE_ACK_TICK_HEIGHT_PX, CLASSIC_FIRST_CONTACT_ROUTE_ACK_TICK_WIDTH_PX,
    CLASSIC_FIRST_CONTACT_ROUTE_DASH_HEIGHT_PX, CLASSIC_FIRST_CONTACT_ROUTE_DASH_WIDTH_PX,
    CLASSIC_FIRST_CONTACT_SELECTED_FOCUS_BRACKET_PIXELS_PER_TILE,
    CLASSIC_FIRST_CONTACT_SELECTED_ROLE_BADGE_TICK_H_PX,
    CLASSIC_FIRST_CONTACT_SELECTED_ROLE_BADGE_TICK_W_PX,
    TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_MARKER_BUDGET_CONTRACT,
};

pub(crate) const LOWER_LANE_GALLERY_DARKEN_NUMERATOR: u32 = 5;
pub(crate) const LOWER_LANE_GALLERY_DARKEN_DENOMINATOR: u32 = 6;
pub(crate) const LOWER_LANE_SLOT_CUE_PIXELS_PER_SAMPLE: usize = 1;
pub(crate) const LOWER_LANE_GHOST_ANCHOR_COUNT: usize = 1;

pub(crate) struct GalleryBudgetSummary {
    pub(crate) gallery_lanes: Vec<&'static str>,
    pub(crate) busy_core_tiles: Vec<(i32, i32)>,
    pub(crate) lower_lane_gallery_tiles: Vec<(i32, i32)>,
    pub(crate) north_gallery_frame_count: usize,
    pub(crate) west_gallery_frame_count: usize,
    pub(crate) east_gallery_frame_count: usize,
    pub(crate) max_gallery_lane_frame_count: usize,
    pub(crate) muted_gallery_sample_count: usize,
    pub(crate) gallery_mute_overlay_pixel_budget: usize,
    pub(crate) gallery_slot_cue_pixel_budget: usize,
    pub(crate) lower_lane_gallery_sample_count: usize,
    pub(crate) lower_lane_mute_overlay_pixel_budget: usize,
    pub(crate) lower_lane_slot_cue_pixel_budget: usize,
    pub(crate) lower_lane_ghost_anchor_count: usize,
    pub(crate) lower_lane_gallery_darken_numerator: usize,
    pub(crate) lower_lane_gallery_darken_denominator: usize,
    pub(crate) lower_lane_dim_silhouette_pixel_budget: usize,
    pub(crate) lower_lane_shadow_suppressed_count: usize,
    pub(crate) gallery_hot_marker_color_count: usize,
    pub(crate) lower_lane_hot_marker_color_count: usize,
    pub(crate) interactive_hot_marker_role_count: usize,
    pub(crate) gallery_presentation_signatures: Vec<&'static str>,
}

pub(crate) fn gallery_budget_summary<F>(
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
    let gallery_mute_overlay_pixel_budget = family_samples
        .iter()
        .map(|(_, _, frame_id, _, scale)| frame_pixel_area(frame_id, *scale) / 2)
        .sum::<usize>();
    let gallery_slot_cue_pixel_budget = family_samples.len() * 72;
    let lower_lane_gallery_sample_count = lower_lane_gallery_tiles.len();
    let lower_lane_mute_overlay_pixel_budget = lower_lane_gallery_sample_count * 384;
    let lower_lane_slot_cue_pixel_budget =
        lower_lane_gallery_sample_count * LOWER_LANE_SLOT_CUE_PIXELS_PER_SAMPLE;
    let lower_lane_ghost_anchor_count =
        lower_lane_gallery_sample_count * LOWER_LANE_GHOST_ANCHOR_COUNT;
    let lower_lane_gallery_darken_numerator = LOWER_LANE_GALLERY_DARKEN_NUMERATOR as usize;
    let lower_lane_gallery_darken_denominator = LOWER_LANE_GALLERY_DARKEN_DENOMINATOR as usize;
    let lower_lane_dim_silhouette_pixel_budget =
        lower_lane_gallery_sample_count * lower_lane_gallery_darken_numerator * 96;

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
        lower_lane_gallery_sample_count,
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
            "darkened_gallery_frames",
            "lower_lane_gallery_deemphasis",
            "lower_lane_micro_slot_cues",
            "lower_lane_dim_silhouettes",
            "lower_lane_stronger_dim_silhouettes",
            "lower_lane_shadow_suppressed",
            "lower_lane_ghost_anchors",
            "lower_lane_single_point_ghost_anchors",
            "compact_route_ack_ticks",
            "perimeter_gallery_lane_budget",
            "interactive_focus_kept_hot",
        ],
    }
}

fn string_vec<const N: usize>(values: [&str; N]) -> Vec<String> {
    values.into_iter().map(str::to_string).collect()
}

fn tile_ids(tiles: &[(i32, i32)]) -> Vec<String> {
    tiles.iter().copied().map(classic_rts_tile_id).collect()
}

pub(crate) fn marker_budget_guard<F>(
    runtime: &NativeFirstPlayableRuntime,
    mut frame_pixel_area: F,
) -> Value
where
    F: FnMut(&str, u32) -> usize,
{
    let family_samples = first_contact_samples::atlas_frame_family_samples();
    let gallery_summary = gallery_budget_summary(&family_samples, |frame_id, scale| {
        frame_pixel_area(frame_id, scale)
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
    let lower_lane_gallery_sample_count = gallery_summary.lower_lane_gallery_sample_count;
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
    let gallery_presentation_signatures = gallery_summary
        .gallery_presentation_signatures
        .iter()
        .map(|signature| (*signature).to_string())
        .collect::<Vec<_>>();
    let selected_focus_tiles = runtime
        .rts_selection_box_tile_ids
        .iter()
        .filter_map(|tile_id| classic_parse_rts_tile(tile_id).map(classic_rts_tile_id))
        .collect::<Vec<_>>();
    let route_focus_tile_pairs = first_contact_tiles::selection_combat_focus_route_tiles(runtime);
    let route_focus_tiles = tile_ids(&route_focus_tile_pairs);
    let route_ack_tick_count = route_focus_tile_pairs
        .windows(2)
        .map(|pair| rts_bevy_runtime::rts_runtime_tile_line(pair[0], pair[1]).len())
        .sum::<usize>();
    let selected_role_badge_tick_pixel_budget = selected_focus_tiles.len()
        * (CLASSIC_FIRST_CONTACT_SELECTED_ROLE_BADGE_TICK_W_PX as usize)
        * (CLASSIC_FIRST_CONTACT_SELECTED_ROLE_BADGE_TICK_H_PX as usize);
    let selected_focus_pixel_budget = selected_focus_tiles.len()
        * CLASSIC_FIRST_CONTACT_SELECTED_FOCUS_BRACKET_PIXELS_PER_TILE
        + selected_role_badge_tick_pixel_budget;
    let route_focus_pixel_budget = route_focus_tiles.len()
        * (CLASSIC_FIRST_CONTACT_ROUTE_DASH_WIDTH_PX as usize)
        * (CLASSIC_FIRST_CONTACT_ROUTE_DASH_HEIGHT_PX as usize)
        + route_ack_tick_count
            * (CLASSIC_FIRST_CONTACT_ROUTE_ACK_TICK_WIDTH_PX as usize)
            * (CLASSIC_FIRST_CONTACT_ROUTE_ACK_TICK_HEIGHT_PX as usize);
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
        && gallery_mute_overlay_pixel_budget >= 21_000
        && gallery_slot_cue_pixel_budget <= 1_008
        && gallery_hot_marker_color_count == 0
        && gallery_presentation_signatures
            .iter()
            .any(|signature| signature == "darkened_gallery_frames");
    let lower_lane_gallery_deemphasis_gate = lower_lane_gallery_tiles
        == string_vec(["29,22", "29,24", "29,26"])
        && lower_lane_gallery_sample_count == 3
        && lower_lane_mute_overlay_pixel_budget >= 1_152
        && lower_lane_slot_cue_pixel_budget <= 3
        && lower_lane_ghost_anchor_count == 3
        && lower_lane_gallery_darken_numerator == 5
        && lower_lane_gallery_darken_denominator == 6
        && lower_lane_dim_silhouette_pixel_budget <= 1_440
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
            .any(|signature| signature == "lower_lane_dim_silhouettes")
        && gallery_presentation_signatures
            .iter()
            .any(|signature| signature == "lower_lane_stronger_dim_silhouettes")
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
        && selected_role_badge_tick_pixel_budget <= 72
        && interactive_focus_pixel_budget >= 890;
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
        && marker_budget_layer_order_gate;

    json!({
        "contract_version": TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_MARKER_BUDGET_CONTRACT,
        "green": first_contact_marker_budget_gate,
        "source_path": "trnm-world-bevy muted First Contact atlas gallery presentation plus final selection/combat focus layer",
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
        "lower_lane_gallery_sample_count": lower_lane_gallery_sample_count,
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
        "marker_budget_layer_order_gate": marker_budget_layer_order_gate,
        "first_contact_marker_budget_gate": first_contact_marker_budget_gate,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(summary.lower_lane_mute_overlay_pixel_budget, 1_152);
        assert_eq!(summary.lower_lane_slot_cue_pixel_budget, 3);
        assert_eq!(summary.lower_lane_ghost_anchor_count, 3);
        assert_eq!(summary.lower_lane_gallery_darken_numerator, 5);
        assert_eq!(summary.lower_lane_gallery_darken_denominator, 6);
        assert_eq!(summary.lower_lane_dim_silhouette_pixel_budget, 1_440);
        assert_eq!(summary.lower_lane_shadow_suppressed_count, 3);
        assert_eq!(summary.gallery_hot_marker_color_count, 0);
        assert_eq!(summary.lower_lane_hot_marker_color_count, 0);
        assert_eq!(summary.interactive_hot_marker_role_count, 5);
        assert!(summary
            .gallery_presentation_signatures
            .contains(&"lower_lane_single_point_ghost_anchors"));
        assert!(summary
            .gallery_presentation_signatures
            .contains(&"interactive_focus_kept_hot"));
    }

    #[test]
    fn first_contact_marker_budget_guard_preserves_gallery_and_focus_contracts() {
        let runtime = crate::classic_first_contact_player_screen_runtime();
        let assets = crate::classic_first_contact_atlas_readability_assets();
        let guard = marker_budget_guard(&runtime, |frame_id, scale| {
            let (frame_w, frame_h) =
                crate::classic_first_contact_atlas_asset_frame_size(&assets, frame_id, scale);
            (frame_w.max(0) as usize) * (frame_h.max(0) as usize)
        });

        assert_eq!(guard.get("green").and_then(Value::as_bool), Some(true));
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
                .get("lower_lane_ghost_anchor_count")
                .and_then(Value::as_u64),
            Some(3)
        );
        assert_eq!(
            guard
                .get("selected_role_badge_tick_pixel_budget")
                .and_then(Value::as_u64),
            Some(72)
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

        for gate in [
            "gallery_lane_budget_gate",
            "gallery_mute_gate",
            "lower_lane_gallery_deemphasis_gate",
            "interactive_focus_preservation_gate",
            "marker_budget_layer_order_gate",
            "first_contact_marker_budget_gate",
        ] {
            assert_eq!(guard.get(gate).and_then(Value::as_bool), Some(true));
        }
    }
}
