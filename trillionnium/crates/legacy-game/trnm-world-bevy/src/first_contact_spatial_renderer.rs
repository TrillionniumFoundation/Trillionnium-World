#![cfg(not(target_os = "android"))]

use crate::{
    classic_darken, classic_draw_iso_ellipse, classic_draw_rect,
    classic_first_contact_command_feedback, classic_first_contact_tile_screen,
    classic_first_contact_tile_tuple, classic_mix_color, classic_parse_rts_tile,
    first_contact_tiles, NativeFirstPlayableRuntime,
    CLASSIC_FIRST_CONTACT_CENTRAL_QUIET_CUES_PER_TILE,
    CLASSIC_FIRST_CONTACT_CENTRAL_QUIET_CUE_H_PX, CLASSIC_FIRST_CONTACT_CENTRAL_QUIET_CUE_W_PX,
    CLASSIC_FIRST_CONTACT_CORRIDOR_DEEMPHASIS_CUE_H_PX,
    CLASSIC_FIRST_CONTACT_CORRIDOR_DEEMPHASIS_CUE_W_PX,
    CLASSIC_FIRST_CONTACT_ROUTE_SPINE_SHADOW_CUE_H_PX,
    CLASSIC_FIRST_CONTACT_ROUTE_SPINE_SHADOW_CUE_W_PX,
    CLASSIC_FIRST_CONTACT_TERMINAL_QUIET_CUES_PER_TILE,
    CLASSIC_FIRST_CONTACT_TERMINAL_QUIET_CUE_H_PX, CLASSIC_FIRST_CONTACT_TERMINAL_QUIET_CUE_W_PX,
    CLASSIC_RTS_COMMAND_SURFACE_TARGET_COLOR, CLASSIC_RTS_PRODUCT_LANE_COLOR,
    CLASSIC_RTS_SELECTION_FEEDBACK_ERROR_COLOR, CLASSIC_RTS_TACTICAL_VIEWPORT_SHADOW_COLOR,
    CLASSIC_RTS_TACTICAL_VIEWPORT_TILE_COLOR,
};
use trnm_rts_bevy_runtime as rts_bevy_runtime;

fn selection_combat_focus_route_tiles(runtime: &NativeFirstPlayableRuntime) -> Vec<(i32, i32)> {
    first_contact_tiles::selection_combat_focus_route_tiles(runtime)
}

fn visual_hierarchy_corridor_tiles(runtime: &NativeFirstPlayableRuntime) -> Vec<(i32, i32)> {
    let feedback = classic_first_contact_command_feedback();
    first_contact_tiles::visual_hierarchy_corridor_tiles(
        runtime,
        feedback.target_tile,
        feedback.blocked_tile,
    )
}

fn central_clarity_quiet_tiles(runtime: &NativeFirstPlayableRuntime) -> Vec<(i32, i32)> {
    let feedback = classic_first_contact_command_feedback();
    first_contact_tiles::central_clarity_quiet_tiles(
        runtime,
        feedback.target_tile,
        feedback.blocked_tile,
    )
}

fn terminal_legibility_target_quiet_tiles() -> Vec<(i32, i32)> {
    first_contact_tiles::terminal_legibility_target_quiet_tiles()
}

fn terminal_legibility_blocked_quiet_tiles() -> Vec<(i32, i32)> {
    first_contact_tiles::terminal_legibility_blocked_quiet_tiles()
}

#[allow(clippy::too_many_arguments)]
fn draw_micro_backplate_corners(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    corner_w: i32,
    corner_h: i32,
    color: u32,
) {
    for (corner_x, corner_y, sx, sy) in [
        (x, y, 1, 1),
        (x + w - corner_w, y, -1, 1),
        (x, y + h - corner_h, 1, -1),
        (x + w - corner_w, y + h - corner_h, -1, -1),
    ] {
        classic_draw_rect(
            buffer, width, height, corner_x, corner_y, corner_w, 2, color,
        );
        let vertical_x = if sx > 0 {
            corner_x
        } else {
            corner_x + corner_w - 2
        };
        let vertical_y = if sy > 0 {
            corner_y
        } else {
            corner_y + corner_h - 2
        };
        classic_draw_rect(
            buffer, width, height, vertical_x, vertical_y, 2, corner_h, color,
        );
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn draw_visual_hierarchy_layer(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    runtime: &NativeFirstPlayableRuntime,
    map_x: i32,
    map_y: i32,
    cell_w: i32,
    cell_h: i32,
) {
    let corridor_color =
        classic_mix_color(CLASSIC_RTS_TACTICAL_VIEWPORT_TILE_COLOR, 0x020705, 3, 4);
    for tile in visual_hierarchy_corridor_tiles(runtime) {
        let (tile_x, tile_y) =
            classic_first_contact_tile_screen(map_x, map_y, cell_w, cell_h, tile);
        let cue_w = CLASSIC_FIRST_CONTACT_CORRIDOR_DEEMPHASIS_CUE_W_PX;
        let cue_h = CLASSIC_FIRST_CONTACT_CORRIDOR_DEEMPHASIS_CUE_H_PX;
        let left_x = tile_x + 4;
        let right_x = tile_x + cell_w - 4 - cue_w;
        let mid_y = tile_y + cell_h / 2;
        classic_draw_rect(
            buffer,
            width,
            height,
            left_x,
            mid_y - 3,
            cue_w,
            cue_h,
            corridor_color,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            right_x,
            mid_y + 3,
            cue_w,
            cue_h,
            CLASSIC_RTS_TACTICAL_VIEWPORT_SHADOW_COLOR,
        );
    }

    let route_tiles = selection_combat_focus_route_tiles(runtime);
    for pair in route_tiles.windows(2) {
        for step in rts_bevy_runtime::rts_runtime_tile_line(pair[0], pair[1]) {
            let (tile_x, tile_y) = classic_first_contact_tile_screen(
                map_x,
                map_y,
                cell_w,
                cell_h,
                (step.tile_x, step.tile_y),
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                tile_x + cell_w / 2 - CLASSIC_FIRST_CONTACT_ROUTE_SPINE_SHADOW_CUE_W_PX / 2,
                tile_y + cell_h / 2 - CLASSIC_FIRST_CONTACT_ROUTE_SPINE_SHADOW_CUE_H_PX / 2,
                CLASSIC_FIRST_CONTACT_ROUTE_SPINE_SHADOW_CUE_W_PX,
                CLASSIC_FIRST_CONTACT_ROUTE_SPINE_SHADOW_CUE_H_PX,
                CLASSIC_RTS_TACTICAL_VIEWPORT_SHADOW_COLOR,
            );
        }
    }

    for tile_id in runtime.rts_selection_box_tile_ids.iter().take(4) {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (tile_x, tile_y) =
                classic_first_contact_tile_screen(map_x, map_y, cell_w, cell_h, tile);
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                tile_x + cell_w / 2,
                tile_y + cell_h,
                (cell_w / 2 + 10).max(12),
                (cell_h / 3 + 5).max(6),
                corridor_color,
            );
        }
    }

    let feedback = classic_first_contact_command_feedback();
    let target_tile = runtime
        .rts_command_destination_tile
        .as_deref()
        .and_then(classic_parse_rts_tile)
        .unwrap_or_else(|| classic_first_contact_tile_tuple(feedback.target_tile));
    let (target_x, target_y) =
        classic_first_contact_tile_screen(map_x, map_y, cell_w, cell_h, target_tile);
    draw_micro_backplate_corners(
        buffer,
        width,
        height,
        target_x - 6,
        target_y + cell_h / 2 - 10,
        cell_w + 12,
        cell_h + 12,
        8,
        6,
        classic_mix_color(CLASSIC_RTS_COMMAND_SURFACE_TARGET_COLOR, 0x050a08, 1, 5),
    );

    let blocked_tile = classic_first_contact_tile_tuple(feedback.blocked_tile);
    let (blocked_x, blocked_y) =
        classic_first_contact_tile_screen(map_x, map_y, cell_w, cell_h, blocked_tile);
    draw_micro_backplate_corners(
        buffer,
        width,
        height,
        blocked_x + 2,
        blocked_y + cell_h / 2 - 6,
        cell_w - 4,
        cell_h + 4,
        6,
        5,
        classic_mix_color(CLASSIC_RTS_SELECTION_FEEDBACK_ERROR_COLOR, 0x050a08, 1, 6),
    );
}

#[allow(clippy::too_many_arguments)]
pub(super) fn draw_central_clarity_layer(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    runtime: &NativeFirstPlayableRuntime,
    map_x: i32,
    map_y: i32,
    cell_w: i32,
    cell_h: i32,
) {
    let quiet_fill = classic_mix_color(CLASSIC_RTS_TACTICAL_VIEWPORT_TILE_COLOR, 0x010403, 1, 2);
    let quiet_edge = classic_darken(CLASSIC_RTS_PRODUCT_LANE_COLOR, 1, 6);
    for tile in central_clarity_quiet_tiles(runtime) {
        let (tile_x, tile_y) =
            classic_first_contact_tile_screen(map_x, map_y, cell_w, cell_h, tile);
        let cue_w = CLASSIC_FIRST_CONTACT_CENTRAL_QUIET_CUE_W_PX;
        let cue_h = CLASSIC_FIRST_CONTACT_CENTRAL_QUIET_CUE_H_PX;
        let left_x = tile_x + 5;
        let right_x = tile_x + cell_w - 5 - cue_w;
        let mid_y = tile_y + cell_h / 2;
        let cues = [
            (left_x, mid_y - 3, quiet_fill),
            (right_x, mid_y + 2, quiet_edge),
        ];
        debug_assert_eq!(
            cues.len(),
            CLASSIC_FIRST_CONTACT_CENTRAL_QUIET_CUES_PER_TILE
        );
        for (x, y, color) in cues {
            classic_draw_rect(buffer, width, height, x, y, cue_w, cue_h, color);
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn draw_terminal_legibility_layer(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    map_x: i32,
    map_y: i32,
    cell_w: i32,
    cell_h: i32,
) {
    let target_fill = classic_mix_color(CLASSIC_RTS_TACTICAL_VIEWPORT_TILE_COLOR, 0x010302, 2, 3);
    let target_edge = classic_darken(CLASSIC_RTS_COMMAND_SURFACE_TARGET_COLOR, 1, 6);
    for tile in terminal_legibility_target_quiet_tiles() {
        let (tile_x, tile_y) =
            classic_first_contact_tile_screen(map_x, map_y, cell_w, cell_h, tile);
        let cue_w = CLASSIC_FIRST_CONTACT_TERMINAL_QUIET_CUE_W_PX;
        let cue_h = CLASSIC_FIRST_CONTACT_TERMINAL_QUIET_CUE_H_PX;
        let cues = [
            (tile_x + 5, tile_y + 3, target_fill),
            (tile_x + cell_w - 5 - cue_w, tile_y + 5, target_edge),
        ];
        debug_assert_eq!(
            cues.len(),
            CLASSIC_FIRST_CONTACT_TERMINAL_QUIET_CUES_PER_TILE
        );
        for (x, y, color) in cues {
            classic_draw_rect(buffer, width, height, x, y, cue_w, cue_h, color);
        }
    }

    let blocked_fill = classic_mix_color(CLASSIC_RTS_TACTICAL_VIEWPORT_TILE_COLOR, 0x020102, 1, 2);
    let blocked_edge = classic_darken(CLASSIC_RTS_SELECTION_FEEDBACK_ERROR_COLOR, 1, 7);
    for tile in terminal_legibility_blocked_quiet_tiles() {
        let (tile_x, tile_y) =
            classic_first_contact_tile_screen(map_x, map_y, cell_w, cell_h, tile);
        let cue_w = CLASSIC_FIRST_CONTACT_TERMINAL_QUIET_CUE_W_PX;
        let cue_h = CLASSIC_FIRST_CONTACT_TERMINAL_QUIET_CUE_H_PX;
        let cues = [
            (tile_x + 5, tile_y + cell_h / 2 - 3, blocked_fill),
            (
                tile_x + cell_w - 5 - cue_w,
                tile_y + cell_h / 2 + 2,
                blocked_edge,
            ),
        ];
        debug_assert_eq!(
            cues.len(),
            CLASSIC_FIRST_CONTACT_TERMINAL_QUIET_CUES_PER_TILE
        );
        for (x, y, color) in cues {
            classic_draw_rect(buffer, width, height, x, y, cue_w, cue_h, color);
        }
    }
}
