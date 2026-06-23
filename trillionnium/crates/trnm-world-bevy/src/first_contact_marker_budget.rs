#![cfg(not(target_os = "android"))]

use crate::first_contact_samples::{self, AtlasSample};

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
}
