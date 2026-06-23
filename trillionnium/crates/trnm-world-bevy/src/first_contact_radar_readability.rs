#![cfg(not(target_os = "android"))]

use serde_json::{json, Value};

use crate::first_contact_tiles;
use crate::{
    classic_parse_rts_tile, classic_rts_camera_state_for_focus_tile, classic_rts_tile_id,
    rts_camera_minimap_viewport_rect, NativeFirstPlayableRuntime, RtsCameraMinimapViewportRect,
    CLASSIC_RTS_LARGE_MAP_MAX_X, CLASSIC_RTS_LARGE_MAP_MAX_Y, CLASSIC_RTS_LARGE_MAP_MIN_TILE,
    TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_RADAR_READABILITY_CONTRACT,
};

fn tile_ids(tiles: &[(i32, i32)]) -> Vec<String> {
    tiles.iter().copied().map(classic_rts_tile_id).collect()
}

fn runtime_tile_ids(values: &[String]) -> Vec<String> {
    values
        .iter()
        .filter_map(|tile_id| classic_parse_rts_tile(tile_id).map(classic_rts_tile_id))
        .collect()
}

pub(crate) fn radar_zoom(runtime: &NativeFirstPlayableRuntime) -> f32 {
    (runtime.rts_camera_zoom_percent.max(1) as f32 / 100.0).clamp(0.25, 3.0)
}

pub(crate) fn radar_viewport_rect(
    runtime: &NativeFirstPlayableRuntime,
) -> RtsCameraMinimapViewportRect {
    runtime.rts_camera_viewport_rect.unwrap_or_else(|| {
        let camera_state = classic_rts_camera_state_for_focus_tile(
            first_contact_tiles::radar_focus_tile(runtime),
            radar_zoom(runtime),
        );
        rts_camera_minimap_viewport_rect(camera_state, 117, 56)
    })
}

pub(crate) fn radar_readability_guard(runtime: &NativeFirstPlayableRuntime) -> Value {
    let selected_tile_ids = runtime_tile_ids(&runtime.rts_selection_box_tile_ids);
    let route_tile_ids = runtime_tile_ids(&runtime.rts_group_route_tile_ids);
    let visible_tile_ids = runtime_tile_ids(&runtime.rts_visible_tile_ids);
    let objective_tiles = tile_ids(&first_contact_tiles::radar_objective_tiles());
    let structure_tiles = tile_ids(&first_contact_tiles::radar_structure_tiles());
    let pressure_tiles = tile_ids(&first_contact_tiles::radar_pressure_tiles());
    let lane_sample_tiles = tile_ids(&first_contact_tiles::radar_lane_sample_tiles());
    let command_destination_tile = runtime
        .rts_command_destination_tile
        .as_deref()
        .and_then(classic_parse_rts_tile)
        .map(classic_rts_tile_id)
        .unwrap_or_else(|| "16,9".to_string());
    let known_terrain_cell_count = (CLASSIC_RTS_LARGE_MAP_MAX_X - CLASSIC_RTS_LARGE_MAP_MIN_TILE
        + 1)
        * (CLASSIC_RTS_LARGE_MAP_MAX_Y - CLASSIC_RTS_LARGE_MAP_MIN_TILE + 1);
    let fog_context_cell_count =
        known_terrain_cell_count.saturating_sub(visible_tile_ids.len() as i32);
    let terrain_context_pixel_budget = known_terrain_cell_count * 2;
    let visible_tile_pixel_budget = visible_tile_ids.len() * 3;
    let selected_blip_pixel_budget = selected_tile_ids.len() * 16;
    let route_trace_pixel_budget = route_tile_ids.len() * 12;
    let objective_blip_pixel_budget = objective_tiles.len() * 32;
    let structure_blip_pixel_budget = structure_tiles.len() * 24;
    let pressure_blip_pixel_budget = pressure_tiles.len() * 18;
    let lane_context_pixel_budget = lane_sample_tiles.len() * 6;
    let known_terrain_gate =
        known_terrain_cell_count >= 1024 && terrain_context_pixel_budget >= 2048;
    let fog_context_gate = fog_context_cell_count >= 900;
    let visible_tile_gate = visible_tile_ids.len() >= 64 && visible_tile_pixel_budget >= 192;
    let selected_blip_gate = selected_tile_ids.len() >= 4 && selected_blip_pixel_budget >= 64;
    let route_trace_gate = route_tile_ids.len() >= 4
        && route_tile_ids
            .iter()
            .any(|tile| tile == &command_destination_tile)
        && route_trace_pixel_budget >= 48;
    let objective_blip_gate = objective_tiles.len() == 4 && objective_blip_pixel_budget >= 128;
    let structure_blip_gate = structure_tiles.len() >= 6 && structure_blip_pixel_budget >= 144;
    let pressure_blip_gate = pressure_tiles.len() >= 3 && pressure_blip_pixel_budget >= 54;
    let lane_context_gate = lane_sample_tiles.len() >= 24 && lane_context_pixel_budget >= 144;
    let viewport_rect = radar_viewport_rect(runtime);
    let viewport_frame_gate = viewport_rect.width >= 18 && viewport_rect.height >= 14;
    let command_destination_gate = command_destination_tile == "16,9";
    let green = known_terrain_gate
        && fog_context_gate
        && visible_tile_gate
        && selected_blip_gate
        && route_trace_gate
        && objective_blip_gate
        && structure_blip_gate
        && pressure_blip_gate
        && lane_context_gate
        && viewport_frame_gate
        && command_destination_gate;

    json!({
        "contract_version": TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_RADAR_READABILITY_CONTRACT,
        "green": green,
        "source_path": "trnm-world-bevy classic_draw_first_contact_radar_context",
        "known_terrain_cell_count": known_terrain_cell_count,
        "fog_context_cell_count": fog_context_cell_count,
        "terrain_context_pixel_budget": terrain_context_pixel_budget,
        "known_terrain_gate": known_terrain_gate,
        "fog_context_gate": fog_context_gate,
        "visible_tile_ids": visible_tile_ids,
        "visible_tile_pixel_budget": visible_tile_pixel_budget,
        "visible_tile_gate": visible_tile_gate,
        "selected_tile_ids": selected_tile_ids,
        "selected_blip_pixel_budget": selected_blip_pixel_budget,
        "selected_blip_gate": selected_blip_gate,
        "route_tile_ids": route_tile_ids,
        "route_trace_pixel_budget": route_trace_pixel_budget,
        "route_trace_gate": route_trace_gate,
        "command_destination_tile": command_destination_tile,
        "command_destination_gate": command_destination_gate,
        "objective_tiles": objective_tiles,
        "objective_blip_pixel_budget": objective_blip_pixel_budget,
        "objective_blip_gate": objective_blip_gate,
        "structure_tiles": structure_tiles,
        "structure_blip_pixel_budget": structure_blip_pixel_budget,
        "structure_blip_gate": structure_blip_gate,
        "pressure_tiles": pressure_tiles,
        "pressure_blip_pixel_budget": pressure_blip_pixel_budget,
        "pressure_blip_gate": pressure_blip_gate,
        "lane_sample_tiles": lane_sample_tiles,
        "lane_context_pixel_budget": lane_context_pixel_budget,
        "lane_context_gate": lane_context_gate,
        "viewport_frame_gate": viewport_frame_gate,
        "viewport_rect": viewport_rect,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn first_contact_radar_runtime() -> NativeFirstPlayableRuntime {
        let mut visible_tiles = Vec::new();
        for y in 1..=8 {
            for x in 1..=8 {
                visible_tiles.push(format!("{x},{y}"));
            }
        }

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
            rts_visible_tile_ids: visible_tiles,
            rts_command_destination_tile: Some("16,9".to_string()),
            rts_camera_focus_tile_id: Some("13,3".to_string()),
            rts_camera_zoom_percent: 100,
            rts_camera_viewport_rect: Some(RtsCameraMinimapViewportRect {
                x: 8,
                y: 5,
                width: 22,
                height: 16,
            }),
            ..Default::default()
        }
    }

    #[test]
    fn first_contact_radar_readability_helpers_preserve_minimap_contracts() {
        let runtime = first_contact_radar_runtime();
        let guard = radar_readability_guard(&runtime);

        assert_eq!(guard.get("green").and_then(Value::as_bool), Some(true));
        assert_eq!(
            guard
                .get("command_destination_tile")
                .and_then(Value::as_str),
            Some("16,9")
        );
        assert_eq!(
            guard
                .get("visible_tile_ids")
                .and_then(Value::as_array)
                .map(Vec::len),
            Some(64)
        );
        assert_eq!(
            guard
                .get("objective_tiles")
                .and_then(Value::as_array)
                .map(Vec::len),
            Some(4)
        );
        assert_eq!(
            guard
                .get("structure_tiles")
                .and_then(Value::as_array)
                .map(Vec::len),
            Some(6)
        );
        assert_eq!(
            guard
                .get("pressure_tiles")
                .and_then(Value::as_array)
                .map(Vec::len),
            Some(3)
        );
        assert_eq!(
            guard
                .get("lane_sample_tiles")
                .and_then(Value::as_array)
                .map(Vec::len),
            Some(26)
        );
        assert_eq!(
            guard.get("viewport_frame_gate").and_then(Value::as_bool),
            Some(true)
        );
    }
}
