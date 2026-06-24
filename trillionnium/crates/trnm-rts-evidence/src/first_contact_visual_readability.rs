#![cfg(not(target_os = "android"))]

use serde_json::{json, Value};
use trnm_rts_bevy_runtime::rts_runtime_tile_id;
use trnm_rts_data::{first_contact_basin_map, first_contact_map_renderer_model};

use crate::TRNM_RTS_EVIDENCE_FIRST_CONTACT_VISUAL_READABILITY_CONTRACT;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RtsFirstContactVisualReadabilityRuntime {
    pub selected_tile_ids: Vec<String>,
    pub route_tile_ids: Vec<String>,
    pub command_destination_tile: Option<String>,
}

fn string_vec<const N: usize>(values: [&str; N]) -> Vec<String> {
    values.into_iter().map(str::to_string).collect()
}

pub fn first_contact_visual_readability_guard(
    runtime: &RtsFirstContactVisualReadabilityRuntime,
) -> Value {
    let selected_tile_ids = runtime.selected_tile_ids.clone();
    let route_tile_ids = runtime.route_tile_ids.clone();
    let command_destination_tile = runtime
        .command_destination_tile
        .clone()
        .unwrap_or_else(|| "16,9".to_string());
    let structure_anchor_tiles = string_vec(["8,8", "25,8", "25,25", "8,25", "11,8", "22,25"]);
    let objective_focus_tiles = string_vec(["16,9", "16,24", "9,16", "24,16"]);
    let lane_edge_sample_tiles = first_contact_map_renderer_model(&first_contact_basin_map())
        .lane_tiles
        .iter()
        .take(16)
        .map(|tile| rts_runtime_tile_id((tile.x, tile.y)))
        .collect::<Vec<_>>();
    let selected_marker_pixel_budget = selected_tile_ids.len() * 56;
    let route_marker_pixel_budget = route_tile_ids.len() * 18;
    let structure_outline_pixel_budget = structure_anchor_tiles.len() * 92;
    let objective_focus_pixel_budget = objective_focus_tiles.len() * 72;
    let lane_edge_pixel_budget = lane_edge_sample_tiles.len() * 16;
    let selected_marker_gate = selected_tile_ids.len() >= 4 && selected_marker_pixel_budget >= 224;
    let route_marker_gate = route_tile_ids.len() >= 4 && route_marker_pixel_budget >= 72;
    let command_target_gate = command_destination_tile == "16,9";
    let structure_outline_gate =
        structure_anchor_tiles.len() >= 6 && structure_outline_pixel_budget >= 552;
    let objective_focus_gate =
        objective_focus_tiles.len() >= 4 && objective_focus_pixel_budget >= 288;
    let terrain_lane_edge_gate =
        lane_edge_sample_tiles.len() >= 12 && lane_edge_pixel_budget >= 192;
    let green = selected_marker_gate
        && route_marker_gate
        && command_target_gate
        && structure_outline_gate
        && objective_focus_gate
        && terrain_lane_edge_gate;

    json!({
        "contract_version": TRNM_RTS_EVIDENCE_FIRST_CONTACT_VISUAL_READABILITY_CONTRACT,
        "green": green,
        "source_path": "trnm-world-bevy classic_draw_first_contact_readability_overlays",
        "selected_tile_ids": selected_tile_ids,
        "selected_marker_pixel_budget": selected_marker_pixel_budget,
        "selected_marker_gate": selected_marker_gate,
        "route_tile_ids": route_tile_ids,
        "route_marker_pixel_budget": route_marker_pixel_budget,
        "route_marker_gate": route_marker_gate,
        "command_destination_tile": command_destination_tile,
        "command_target_gate": command_target_gate,
        "structure_anchor_tiles": structure_anchor_tiles,
        "structure_outline_pixel_budget": structure_outline_pixel_budget,
        "structure_outline_gate": structure_outline_gate,
        "objective_focus_tiles": objective_focus_tiles,
        "objective_focus_pixel_budget": objective_focus_pixel_budget,
        "objective_focus_gate": objective_focus_gate,
        "lane_edge_sample_tiles": lane_edge_sample_tiles,
        "lane_edge_pixel_budget": lane_edge_pixel_budget,
        "terrain_lane_edge_gate": terrain_lane_edge_gate,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn first_contact_focus_runtime() -> RtsFirstContactVisualReadabilityRuntime {
        RtsFirstContactVisualReadabilityRuntime {
            selected_tile_ids: string_vec(["14,11", "15,11", "15,12", "17,12"]),
            route_tile_ids: string_vec(["14,11", "15,11", "16,10", "16,9"]),
            command_destination_tile: Some("16,9".to_string()),
        }
    }

    #[test]
    fn first_contact_visual_readability_helpers_preserve_overlay_contracts() {
        let guard = first_contact_visual_readability_guard(&first_contact_focus_runtime());

        assert_eq!(
            guard.get("contract_version").and_then(Value::as_str),
            Some(TRNM_RTS_EVIDENCE_FIRST_CONTACT_VISUAL_READABILITY_CONTRACT)
        );
        assert_eq!(guard.get("green").and_then(Value::as_bool), Some(true));
        assert_eq!(
            guard.get("selected_tile_ids").cloned(),
            Some(json!(["14,11", "15,11", "15,12", "17,12"]))
        );
        assert_eq!(
            guard.get("route_tile_ids").cloned(),
            Some(json!(["14,11", "15,11", "16,10", "16,9"]))
        );
        assert_eq!(
            guard
                .get("command_destination_tile")
                .and_then(Value::as_str),
            Some("16,9")
        );
        assert_eq!(
            guard
                .get("structure_anchor_tiles")
                .and_then(Value::as_array)
                .map(Vec::len),
            Some(6)
        );
        assert_eq!(
            guard
                .get("objective_focus_tiles")
                .and_then(Value::as_array)
                .map(Vec::len),
            Some(4)
        );
        assert_eq!(
            guard
                .get("lane_edge_sample_tiles")
                .and_then(Value::as_array)
                .map(|tiles| tiles.len() >= 12),
            Some(true)
        );
        for gate in [
            "selected_marker_gate",
            "route_marker_gate",
            "command_target_gate",
            "structure_outline_gate",
            "objective_focus_gate",
            "terrain_lane_edge_gate",
        ] {
            assert_eq!(guard.get(gate).and_then(Value::as_bool), Some(true));
        }
    }
}
