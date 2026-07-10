#![cfg(not(target_os = "android"))]

use std::collections::HashSet;

use crate::{
    classic_darken, classic_draw_rect, classic_parse_rts_tile, classic_rts_minimap_cell_origin,
    first_contact_tiles, NativeFirstPlayableRuntime, CLASSIC_ISO_UNIT_ENEMY_COLOR,
    CLASSIC_RTS_CAMERA_SYNC_ROUTE_COLOR, CLASSIC_RTS_MINIMAP_ROAD_COLOR,
    CLASSIC_RTS_MINIMAP_VISION_COLOR, CLASSIC_RTS_MODEL_IDENTITY_RELAY_COLOR,
    CLASSIC_RTS_OBJECTIVE_COLOR, CLASSIC_RTS_PRESSURE_WARNING_COLOR,
    CLASSIC_RTS_TACTICAL_VIEWPORT_SHADOW_COLOR,
};

fn lane_sample_tiles() -> Vec<(i32, i32)> {
    first_contact_tiles::radar_lane_sample_tiles()
}

fn structure_tiles() -> Vec<(i32, i32)> {
    first_contact_tiles::radar_structure_tiles()
}

fn pressure_tiles() -> Vec<(i32, i32)> {
    first_contact_tiles::radar_pressure_tiles()
}

fn objective_tiles() -> Vec<(i32, i32)> {
    first_contact_tiles::radar_objective_tiles()
}

#[allow(clippy::too_many_arguments)]
pub(super) fn draw_context(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    runtime: &NativeFirstPlayableRuntime,
    visible_tiles: &HashSet<(i32, i32)>,
    radar_x: i32,
    radar_y: i32,
    cell_w: i32,
    cell_h: i32,
) {
    let origin_x = radar_x + 4;
    let origin_y = radar_y + 4;
    let marker_w = cell_w.max(3);
    let marker_h = cell_h.max(3);

    for tile in lane_sample_tiles() {
        let (x, y) = classic_rts_minimap_cell_origin(origin_x, origin_y, cell_w, cell_h, tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            x,
            y + marker_h / 2,
            marker_w,
            1,
            classic_darken(CLASSIC_RTS_MINIMAP_ROAD_COLOR, 4, 5),
        );
    }

    for tile in visible_tiles {
        let (x, y) = classic_rts_minimap_cell_origin(origin_x, origin_y, cell_w, cell_h, *tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            x + marker_w - 2,
            y,
            2,
            marker_h,
            CLASSIC_RTS_MINIMAP_VISION_COLOR,
        );
    }

    for (index, tile) in structure_tiles().into_iter().enumerate() {
        let (x, y) = classic_rts_minimap_cell_origin(origin_x, origin_y, cell_w, cell_h, tile);
        let color = if index < 2 {
            CLASSIC_RTS_MINIMAP_VISION_COLOR
        } else if index < 4 {
            CLASSIC_ISO_UNIT_ENEMY_COLOR
        } else {
            CLASSIC_RTS_MODEL_IDENTITY_RELAY_COLOR
        };
        classic_draw_rect(
            buffer,
            width,
            height,
            x - 1,
            y - 1,
            marker_w + 2,
            marker_h + 2,
            CLASSIC_RTS_TACTICAL_VIEWPORT_SHADOW_COLOR,
        );
        classic_draw_rect(buffer, width, height, x, y, marker_w, marker_h, color);
    }

    for tile in pressure_tiles() {
        let (x, y) = classic_rts_minimap_cell_origin(origin_x, origin_y, cell_w, cell_h, tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            x + marker_w / 2,
            y - 2,
            2,
            marker_h + 4,
            CLASSIC_RTS_PRESSURE_WARNING_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            x - 1,
            y + marker_h / 2,
            marker_w + 2,
            2,
            CLASSIC_RTS_PRESSURE_WARNING_COLOR,
        );
    }

    for tile in objective_tiles() {
        let (x, y) = classic_rts_minimap_cell_origin(origin_x, origin_y, cell_w, cell_h, tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            x - 2,
            y - 2,
            marker_w + 4,
            2,
            CLASSIC_RTS_OBJECTIVE_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            x - 2,
            y + marker_h,
            marker_w + 4,
            2,
            CLASSIC_RTS_OBJECTIVE_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            x - 2,
            y - 2,
            2,
            marker_h + 4,
            CLASSIC_RTS_OBJECTIVE_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            x + marker_w,
            y - 2,
            2,
            marker_h + 4,
            CLASSIC_RTS_OBJECTIVE_COLOR,
        );
    }

    for tile_id in &runtime.rts_group_route_tile_ids {
        let Some(tile) = classic_parse_rts_tile(tile_id) else {
            continue;
        };
        let (x, y) = classic_rts_minimap_cell_origin(origin_x, origin_y, cell_w, cell_h, tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            x - 1,
            y + marker_h / 2,
            marker_w + 2,
            2,
            CLASSIC_RTS_CAMERA_SYNC_ROUTE_COLOR,
        );
    }
}
