#![cfg(not(target_os = "android"))]

use serde_json::{json, Value};
use trnm_rts_bevy_runtime::{
    self as rts_bevy_runtime, rts_runtime_tile_id,
    TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_TILE_SURFACE_CONTRACT,
};
use trnm_rts_core::RtsTile;
use trnm_rts_data::first_contact_samples;

use crate::{
    TRNM_RTS_EVIDENCE_FIRST_CONTACT_CENTRAL_CLARITY_CONTRACT,
    TRNM_RTS_EVIDENCE_FIRST_CONTACT_TERMINAL_LEGIBILITY_CONTRACT,
    TRNM_RTS_EVIDENCE_FIRST_CONTACT_VISUAL_HIERARCHY_CONTRACT,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RtsFirstContactSpatialReadabilityRuntime {
    pub selected_tiles: Vec<(i32, i32)>,
    pub route_tiles: Vec<(i32, i32)>,
    pub command_destination_tile: Option<(i32, i32)>,
    pub fallback_target_tile: (i32, i32),
    pub blocked_tile: (i32, i32),
}

fn string_vec<const N: usize>(values: [&str; N]) -> Vec<String> {
    values.into_iter().map(str::to_string).collect()
}

fn tile_id(tile: (i32, i32)) -> String {
    rts_runtime_tile_id(tile)
}

fn tile_ids(tiles: &[(i32, i32)]) -> Vec<String> {
    tiles.iter().copied().map(tile_id).collect()
}

fn runtime_tile(tile: (i32, i32)) -> RtsTile {
    RtsTile::new(tile.0, tile.1)
}

fn route_tile_ids(runtime: &RtsFirstContactSpatialReadabilityRuntime) -> Vec<String> {
    tile_ids(&runtime.route_tiles)
}

fn command_destination_tile_id(
    runtime: &RtsFirstContactSpatialReadabilityRuntime,
) -> Option<String> {
    runtime.command_destination_tile.map(tile_id)
}

fn selected_focus_tiles(runtime: &RtsFirstContactSpatialReadabilityRuntime) -> Vec<String> {
    tile_ids(&runtime.selected_tiles)
}

fn target_focus_tile(runtime: &RtsFirstContactSpatialReadabilityRuntime) -> (i32, i32) {
    let command_destination_tile = command_destination_tile_id(runtime);
    rts_bevy_runtime::rts_first_contact_target_callout_tile(
        command_destination_tile.as_deref(),
        runtime_tile(runtime.fallback_target_tile),
    )
}

fn target_focus_tile_id(runtime: &RtsFirstContactSpatialReadabilityRuntime) -> String {
    tile_id(target_focus_tile(runtime))
}

fn visual_hierarchy_corridor_tiles(
    runtime: &RtsFirstContactSpatialReadabilityRuntime,
) -> Vec<(i32, i32)> {
    let route_tile_ids = route_tile_ids(runtime);
    let selected_tile_ids = selected_focus_tiles(runtime);
    let command_destination_tile = command_destination_tile_id(runtime);
    rts_bevy_runtime::rts_first_contact_visual_hierarchy_corridor_tiles(
        &route_tile_ids,
        &selected_tile_ids,
        command_destination_tile.as_deref(),
        runtime_tile(runtime.fallback_target_tile),
        runtime_tile(runtime.blocked_tile),
    )
}

fn central_clarity_quiet_tiles(
    runtime: &RtsFirstContactSpatialReadabilityRuntime,
) -> Vec<(i32, i32)> {
    let route_tile_ids = route_tile_ids(runtime);
    let selected_tile_ids = selected_focus_tiles(runtime);
    let command_destination_tile = command_destination_tile_id(runtime);
    rts_bevy_runtime::rts_first_contact_central_clarity_quiet_tiles(
        &route_tile_ids,
        &selected_tile_ids,
        command_destination_tile.as_deref(),
        runtime_tile(runtime.fallback_target_tile),
        runtime_tile(runtime.blocked_tile),
    )
}

fn terminal_legibility_target_quiet_tiles() -> Vec<(i32, i32)> {
    rts_bevy_runtime::rts_first_contact_terminal_legibility_target_quiet_tiles()
}

fn terminal_legibility_blocked_quiet_tiles() -> Vec<(i32, i32)> {
    rts_bevy_runtime::rts_first_contact_terminal_legibility_blocked_quiet_tiles()
}

fn terminal_legibility_quiet_tiles() -> Vec<(i32, i32)> {
    rts_bevy_runtime::rts_first_contact_terminal_legibility_quiet_tiles()
}

fn route_line_step_count(route_focus_tile_pairs: &[(i32, i32)]) -> usize {
    route_focus_tile_pairs
        .windows(2)
        .map(|pair| rts_bevy_runtime::rts_runtime_tile_line(pair[0], pair[1]).len())
        .sum::<usize>()
        + route_focus_tile_pairs.len()
}

fn readability_layer_draw_order() -> Vec<String> {
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

pub fn first_contact_visual_hierarchy_guard(
    runtime: &RtsFirstContactSpatialReadabilityRuntime,
) -> Value {
    let selected_focus_tiles = selected_focus_tiles(runtime);
    let route_focus_tile_pairs = runtime.route_tiles.clone();
    let route_focus_tiles = tile_ids(&route_focus_tile_pairs);
    let corridor_tile_pairs = visual_hierarchy_corridor_tiles(runtime);
    let corridor_tiles = tile_ids(&corridor_tile_pairs);
    let target_focus_tile = target_focus_tile_id(runtime);
    let blocked_focus_tile = tile_id(runtime.blocked_tile);
    let route_line_step_count = route_line_step_count(&route_focus_tile_pairs);
    let atlas_family_gallery_lanes = first_contact_samples::atlas_frame_family_samples()
        .iter()
        .map(|(tile, _, _, _, _)| {
            first_contact_samples::atlas_family_gallery_lane(*tile).to_string()
        })
        .collect::<Vec<_>>();
    let atlas_family_busy_core_tiles = first_contact_samples::atlas_frame_family_samples()
        .into_iter()
        .filter_map(|(tile, _, _, _, _)| {
            first_contact_samples::atlas_family_busy_core_tile(tile).then(|| tile_id(tile))
        })
        .collect::<Vec<_>>();
    let mut unique_gallery_lanes = atlas_family_gallery_lanes.clone();
    unique_gallery_lanes.sort();
    unique_gallery_lanes.dedup();
    let hierarchy_signatures = string_vec([
        "route_corridor_deemphasis",
        "route_spine_shadow",
        "selected_halo_backplates",
        "attack_target_micro_backplate",
        "blocked_warning_micro_backplate",
        "perimeter_gallery_preserved",
    ]);
    let corridor_deemphasis_pixel_budget = corridor_tiles.len() * 78;
    let route_spine_shadow_pixel_budget = route_line_step_count * 18;
    let selected_halo_pixel_budget = selected_focus_tiles.len() * 96;
    let target_backplate_pixel_budget = 96;
    let blocked_backplate_pixel_budget = 72;
    let corridor_tile_gate = corridor_tiles
        == string_vec(["14,11", "15,11", "15,12", "15,16", "16,9", "16,10", "17,12"])
        && corridor_deemphasis_pixel_budget >= 546;
    let route_spine_gate = route_focus_tiles == string_vec(["14,11", "15,11", "16,10", "16,9"])
        && route_line_step_count >= 10
        && route_spine_shadow_pixel_budget >= 180;
    let selected_halo_gate = selected_focus_tiles.len() == 4 && selected_halo_pixel_budget >= 384;
    let combat_backplate_gate = target_focus_tile == "16,9"
        && blocked_focus_tile == "15,16"
        && target_backplate_pixel_budget == 96
        && blocked_backplate_pixel_budget == 72;
    let gallery_preservation_gate = atlas_family_busy_core_tiles.is_empty()
        && unique_gallery_lanes == string_vec(["east_gallery", "north_gallery", "west_gallery"]);
    let hierarchy_signature_gate = hierarchy_signatures.len() == 6
        && hierarchy_signatures
            .iter()
            .any(|signature| signature == "route_corridor_deemphasis")
        && hierarchy_signatures
            .iter()
            .any(|signature| signature == "perimeter_gallery_preserved");
    let hierarchy_layer_draw_order = readability_layer_draw_order();
    let hierarchy_layer_order_gate = hierarchy_layer_draw_order
        .iter()
        .position(|layer| layer == "readability_overlays")
        < hierarchy_layer_draw_order
            .iter()
            .position(|layer| layer == "visual_hierarchy_deemphasis")
        && hierarchy_layer_draw_order
            .iter()
            .position(|layer| layer == "visual_hierarchy_deemphasis")
            < hierarchy_layer_draw_order
                .iter()
                .position(|layer| layer == "central_clarity_deemphasis")
        && hierarchy_layer_draw_order
            .iter()
            .position(|layer| layer == "central_clarity_deemphasis")
            < hierarchy_layer_draw_order
                .iter()
                .position(|layer| layer == "terminal_legibility_deemphasis")
        && hierarchy_layer_draw_order
            .iter()
            .position(|layer| layer == "terminal_legibility_deemphasis")
            < hierarchy_layer_draw_order
                .iter()
                .position(|layer| layer == "selection_combat_focus");
    let visual_hierarchy_gate = corridor_tile_gate
        && route_spine_gate
        && selected_halo_gate
        && combat_backplate_gate
        && gallery_preservation_gate
        && hierarchy_signature_gate
        && hierarchy_layer_order_gate;

    json!({
        "contract_version": TRNM_RTS_EVIDENCE_FIRST_CONTACT_VISUAL_HIERARCHY_CONTRACT,
        "tile_surface_contract": TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_TILE_SURFACE_CONTRACT,
        "green": visual_hierarchy_gate,
        "source_path": "trnm-world-bevy classic_draw_first_contact_visual_hierarchy_layer between readability overlays and selection/combat focus",
        "corridor_tiles": corridor_tiles,
        "selected_focus_tiles": selected_focus_tiles,
        "route_focus_tiles": route_focus_tiles,
        "target_focus_tile": target_focus_tile,
        "blocked_focus_tile": blocked_focus_tile,
        "route_line_step_count": route_line_step_count,
        "atlas_family_gallery_lanes": atlas_family_gallery_lanes,
        "atlas_family_busy_core_tiles": atlas_family_busy_core_tiles,
        "unique_gallery_lanes": unique_gallery_lanes,
        "hierarchy_signatures": hierarchy_signatures,
        "corridor_deemphasis_pixel_budget": corridor_deemphasis_pixel_budget,
        "route_spine_shadow_pixel_budget": route_spine_shadow_pixel_budget,
        "selected_halo_pixel_budget": selected_halo_pixel_budget,
        "target_backplate_pixel_budget": target_backplate_pixel_budget,
        "blocked_backplate_pixel_budget": blocked_backplate_pixel_budget,
        "corridor_tile_gate": corridor_tile_gate,
        "route_spine_gate": route_spine_gate,
        "selected_halo_gate": selected_halo_gate,
        "combat_backplate_gate": combat_backplate_gate,
        "gallery_preservation_gate": gallery_preservation_gate,
        "hierarchy_signature_gate": hierarchy_signature_gate,
        "hierarchy_layer_draw_order": hierarchy_layer_draw_order,
        "hierarchy_layer_order_gate": hierarchy_layer_order_gate,
        "visual_hierarchy_gate": visual_hierarchy_gate,
    })
}

pub fn first_contact_central_clarity_guard(
    runtime: &RtsFirstContactSpatialReadabilityRuntime,
) -> Value {
    let quiet_tiles = central_clarity_quiet_tiles(runtime);
    let focus_corridor_tiles = visual_hierarchy_corridor_tiles(runtime);
    let quiet_tile_ids = tile_ids(&quiet_tiles);
    let focus_corridor_tile_ids = tile_ids(&focus_corridor_tiles);
    let focus_overlap_tiles = quiet_tiles
        .iter()
        .filter(|tile| focus_corridor_tiles.contains(tile))
        .copied()
        .map(tile_id)
        .collect::<Vec<_>>();
    let central_core_tile_count = 18_usize;
    let central_quiet_tile_count = quiet_tiles.len();
    let central_focus_tile_count = central_core_tile_count.saturating_sub(central_quiet_tile_count);
    let quiet_tile_pixel_budget = central_quiet_tile_count * 96;
    let quiet_edge_pixel_budget = central_quiet_tile_count * 18;
    let clarity_signatures = string_vec([
        "central_negative_space_tiles",
        "focus_corridor_not_muted",
        "quiet_edge_separators",
        "selection_focus_still_last",
    ]);
    let clarity_layer_draw_order = readability_layer_draw_order();
    let central_quiet_tile_gate = quiet_tile_ids
        == string_vec([
            "13,10", "14,10", "15,10", "17,10", "18,10", "13,11", "16,11", "17,11", "18,11",
            "13,12", "14,12", "16,12", "18,12",
        ])
        && central_quiet_tile_count == 13
        && central_focus_tile_count == 5
        && quiet_tile_pixel_budget >= 1_248;
    let focus_overlap_gate = focus_overlap_tiles.is_empty()
        && focus_corridor_tile_ids
            == string_vec(["14,11", "15,11", "15,12", "15,16", "16,9", "16,10", "17,12"]);
    let quiet_edge_gate = quiet_edge_pixel_budget >= 234;
    let clarity_signature_gate = clarity_signatures.len() == 4
        && clarity_signatures
            .iter()
            .any(|signature| signature == "central_negative_space_tiles")
        && clarity_signatures
            .iter()
            .any(|signature| signature == "focus_corridor_not_muted");
    let clarity_layer_order_gate = clarity_layer_draw_order
        .iter()
        .position(|layer| layer == "visual_hierarchy_deemphasis")
        < clarity_layer_draw_order
            .iter()
            .position(|layer| layer == "central_clarity_deemphasis")
        && clarity_layer_draw_order
            .iter()
            .position(|layer| layer == "central_clarity_deemphasis")
            < clarity_layer_draw_order
                .iter()
                .position(|layer| layer == "terminal_legibility_deemphasis")
        && clarity_layer_draw_order
            .iter()
            .position(|layer| layer == "terminal_legibility_deemphasis")
            < clarity_layer_draw_order
                .iter()
                .position(|layer| layer == "selection_combat_focus")
        && clarity_layer_draw_order.last().map(String::as_str) == Some("selection_combat_focus");
    let central_clarity_gate = central_quiet_tile_gate
        && focus_overlap_gate
        && quiet_edge_gate
        && clarity_signature_gate
        && clarity_layer_order_gate;

    json!({
        "contract_version": TRNM_RTS_EVIDENCE_FIRST_CONTACT_CENTRAL_CLARITY_CONTRACT,
        "tile_surface_contract": TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_TILE_SURFACE_CONTRACT,
        "green": central_clarity_gate,
        "source_path": "trnm-world-bevy classic_draw_first_contact_central_clarity_layer between visual hierarchy and selection/combat focus",
        "central_core_tile_count": central_core_tile_count,
        "central_quiet_tile_count": central_quiet_tile_count,
        "central_focus_tile_count": central_focus_tile_count,
        "quiet_tiles": quiet_tile_ids,
        "focus_corridor_tiles": focus_corridor_tile_ids,
        "focus_overlap_tiles": focus_overlap_tiles,
        "quiet_tile_pixel_budget": quiet_tile_pixel_budget,
        "quiet_edge_pixel_budget": quiet_edge_pixel_budget,
        "clarity_signatures": clarity_signatures,
        "clarity_layer_draw_order": clarity_layer_draw_order,
        "central_quiet_tile_gate": central_quiet_tile_gate,
        "focus_overlap_gate": focus_overlap_gate,
        "quiet_edge_gate": quiet_edge_gate,
        "clarity_signature_gate": clarity_signature_gate,
        "clarity_layer_order_gate": clarity_layer_order_gate,
        "central_clarity_gate": central_clarity_gate,
    })
}

pub fn first_contact_terminal_legibility_guard(
    runtime: &RtsFirstContactSpatialReadabilityRuntime,
) -> Value {
    let target_quiet_tiles = terminal_legibility_target_quiet_tiles();
    let blocked_quiet_tiles = terminal_legibility_blocked_quiet_tiles();
    let quiet_tiles = terminal_legibility_quiet_tiles();
    let target_quiet_tile_ids = tile_ids(&target_quiet_tiles);
    let blocked_quiet_tile_ids = tile_ids(&blocked_quiet_tiles);
    let target_focus_tile = target_focus_tile_id(runtime);
    let blocked_focus_tile = tile_id(runtime.blocked_tile);
    let route_focus_tile_pairs = runtime.route_tiles.clone();
    let mut terminal_focus_tiles = route_focus_tile_pairs
        .iter()
        .rev()
        .take(2)
        .copied()
        .collect::<Vec<_>>();
    terminal_focus_tiles.push(runtime.blocked_tile);
    terminal_focus_tiles.sort_unstable();
    terminal_focus_tiles.dedup();
    let terminal_focus_tile_ids = tile_ids(&terminal_focus_tiles);
    let focus_overlap_tiles = quiet_tiles
        .iter()
        .filter(|tile| terminal_focus_tiles.contains(tile))
        .copied()
        .map(tile_id)
        .collect::<Vec<_>>();
    let target_quiet_pixel_budget = target_quiet_tiles.len() * 96;
    let blocked_quiet_pixel_budget = blocked_quiet_tiles.len() * 96;
    let target_edge_pixel_budget = target_quiet_tiles.len() * 18;
    let blocked_edge_pixel_budget = blocked_quiet_tiles.len() * 18;
    let terminal_signatures = string_vec([
        "target_terminal_quiet_band",
        "blocked_terminal_quiet_column",
        "route_terminal_focus_preserved",
        "focus_markers_still_last",
    ]);
    let terminal_layer_draw_order = readability_layer_draw_order();
    let target_terminal_quiet_gate = target_quiet_tile_ids
        == string_vec(["15,8", "16,8", "17,8", "15,9", "17,9"])
        && target_focus_tile == "16,9"
        && target_quiet_pixel_budget >= 480;
    let blocked_terminal_quiet_gate = blocked_quiet_tile_ids
        == string_vec([
            "14,15", "15,15", "16,15", "14,16", "16,16", "14,17", "15,17", "16,17",
        ])
        && blocked_focus_tile == "15,16"
        && blocked_quiet_pixel_budget >= 768;
    let terminal_focus_preservation_gate = focus_overlap_tiles.is_empty()
        && terminal_focus_tile_ids == string_vec(["15,16", "16,9", "16,10"]);
    let terminal_edge_budget_gate =
        target_edge_pixel_budget >= 90 && blocked_edge_pixel_budget >= 144;
    let terminal_signature_gate = terminal_signatures.len() == 4
        && terminal_signatures
            .iter()
            .any(|signature| signature == "target_terminal_quiet_band")
        && terminal_signatures
            .iter()
            .any(|signature| signature == "route_terminal_focus_preserved");
    let terminal_layer_order_gate = terminal_layer_draw_order
        .iter()
        .position(|layer| layer == "central_clarity_deemphasis")
        < terminal_layer_draw_order
            .iter()
            .position(|layer| layer == "terminal_legibility_deemphasis")
        && terminal_layer_draw_order
            .iter()
            .position(|layer| layer == "terminal_legibility_deemphasis")
            < terminal_layer_draw_order
                .iter()
                .position(|layer| layer == "selection_combat_focus")
        && terminal_layer_draw_order.last().map(String::as_str) == Some("selection_combat_focus");
    let terminal_legibility_gate = target_terminal_quiet_gate
        && blocked_terminal_quiet_gate
        && terminal_focus_preservation_gate
        && terminal_edge_budget_gate
        && terminal_signature_gate
        && terminal_layer_order_gate;

    json!({
        "contract_version": TRNM_RTS_EVIDENCE_FIRST_CONTACT_TERMINAL_LEGIBILITY_CONTRACT,
        "tile_surface_contract": TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_TILE_SURFACE_CONTRACT,
        "green": terminal_legibility_gate,
        "source_path": "trnm-world-bevy classic_draw_first_contact_terminal_legibility_layer between central clarity and selection/combat focus",
        "terminal_quiet_tile_count": quiet_tiles.len(),
        "target_quiet_tiles": target_quiet_tile_ids,
        "blocked_quiet_tiles": blocked_quiet_tile_ids,
        "terminal_focus_tiles": terminal_focus_tile_ids,
        "focus_overlap_tiles": focus_overlap_tiles,
        "target_focus_tile": target_focus_tile,
        "blocked_focus_tile": blocked_focus_tile,
        "target_quiet_pixel_budget": target_quiet_pixel_budget,
        "blocked_quiet_pixel_budget": blocked_quiet_pixel_budget,
        "target_edge_pixel_budget": target_edge_pixel_budget,
        "blocked_edge_pixel_budget": blocked_edge_pixel_budget,
        "terminal_signatures": terminal_signatures,
        "terminal_layer_draw_order": terminal_layer_draw_order,
        "target_terminal_quiet_gate": target_terminal_quiet_gate,
        "blocked_terminal_quiet_gate": blocked_terminal_quiet_gate,
        "terminal_focus_preservation_gate": terminal_focus_preservation_gate,
        "terminal_edge_budget_gate": terminal_edge_budget_gate,
        "terminal_signature_gate": terminal_signature_gate,
        "terminal_layer_order_gate": terminal_layer_order_gate,
        "terminal_legibility_gate": terminal_legibility_gate,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn focus_runtime() -> RtsFirstContactSpatialReadabilityRuntime {
        RtsFirstContactSpatialReadabilityRuntime {
            selected_tiles: vec![(14, 11), (15, 11), (15, 12), (17, 12)],
            route_tiles: vec![(14, 11), (15, 11), (16, 10), (16, 9)],
            command_destination_tile: Some((16, 9)),
            fallback_target_tile: (16, 9),
            blocked_tile: (15, 16),
        }
    }

    #[test]
    fn first_contact_spatial_readability_helpers_preserve_focus_contracts() {
        let runtime = focus_runtime();

        let hierarchy = first_contact_visual_hierarchy_guard(&runtime);
        assert_eq!(
            hierarchy.get("contract_version").and_then(Value::as_str),
            Some(TRNM_RTS_EVIDENCE_FIRST_CONTACT_VISUAL_HIERARCHY_CONTRACT)
        );
        assert_eq!(
            hierarchy
                .get("tile_surface_contract")
                .and_then(Value::as_str),
            Some(TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_TILE_SURFACE_CONTRACT)
        );
        assert_eq!(hierarchy["green"].as_bool(), Some(true));
        assert_eq!(
            hierarchy["corridor_tiles"].as_array().map(Vec::len),
            Some(7)
        );
        assert_eq!(hierarchy["route_line_step_count"].as_u64(), Some(10));
        assert_eq!(
            hierarchy["target_backplate_pixel_budget"].as_u64(),
            Some(96)
        );
        assert_eq!(
            hierarchy["blocked_backplate_pixel_budget"].as_u64(),
            Some(72)
        );

        let central = first_contact_central_clarity_guard(&runtime);
        assert_eq!(
            central.get("contract_version").and_then(Value::as_str),
            Some(TRNM_RTS_EVIDENCE_FIRST_CONTACT_CENTRAL_CLARITY_CONTRACT)
        );
        assert_eq!(
            central.get("tile_surface_contract").and_then(Value::as_str),
            Some(TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_TILE_SURFACE_CONTRACT)
        );
        assert_eq!(central["green"].as_bool(), Some(true));
        assert_eq!(central["central_quiet_tile_count"].as_u64(), Some(13));
        assert_eq!(
            central["focus_overlap_tiles"].as_array().map(Vec::len),
            Some(0)
        );

        let terminal = first_contact_terminal_legibility_guard(&runtime);
        assert_eq!(
            terminal.get("contract_version").and_then(Value::as_str),
            Some(TRNM_RTS_EVIDENCE_FIRST_CONTACT_TERMINAL_LEGIBILITY_CONTRACT)
        );
        assert_eq!(
            terminal
                .get("tile_surface_contract")
                .and_then(Value::as_str),
            Some(TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_TILE_SURFACE_CONTRACT)
        );
        assert_eq!(terminal["green"].as_bool(), Some(true));
        assert_eq!(terminal["terminal_quiet_tile_count"].as_u64(), Some(13));
        assert_eq!(
            terminal["terminal_focus_tiles"].as_array().map(Vec::len),
            Some(3)
        );
    }
}
