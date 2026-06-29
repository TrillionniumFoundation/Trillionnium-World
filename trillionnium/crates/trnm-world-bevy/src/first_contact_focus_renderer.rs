#![cfg(not(target_os = "android"))]

use crate::{
    classic_darken, classic_draw_iso_ellipse, classic_draw_rect, classic_draw_text,
    classic_first_contact_command_feedback, classic_first_contact_tile_screen,
    classic_first_contact_tile_tuple, classic_mix_color, classic_parse_rts_tile,
    first_contact_readouts, first_contact_tiles, NativeFirstPlayableRuntime,
    CLASSIC_FIRST_CONTACT_ROUTE_ACK_TICK_HEIGHT_PX, CLASSIC_FIRST_CONTACT_ROUTE_ACK_TICK_WIDTH_PX,
    CLASSIC_FIRST_CONTACT_ROUTE_DASH_HEIGHT_PX, CLASSIC_FIRST_CONTACT_ROUTE_DASH_WIDTH_PX,
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
    CLASSIC_FIRST_CONTACT_TARGET_LOCK_CROSS_THICKNESS_PX, CLASSIC_HUD_TEXT_COLOR,
    CLASSIC_RTS_SELECTION_FEEDBACK_ACK_COLOR, CLASSIC_RTS_SELECTION_FEEDBACK_ATTACK_COLOR,
    CLASSIC_RTS_SELECTION_FEEDBACK_CONFIRM_COLOR, CLASSIC_RTS_SELECTION_FEEDBACK_ERROR_COLOR,
    CLASSIC_RTS_SELECTION_FEEDBACK_MOVE_COLOR, CLASSIC_RTS_STATUS_HEALTH_BAR_COLOR,
    CLASSIC_RTS_STATUS_MANA_BAR_COLOR, CLASSIC_RTS_STRUCTURE_FOUNDATION_SHADOW_COLOR,
    CLASSIC_RTS_TACTICAL_VIEWPORT_SHADOW_COLOR, CLASSIC_RTS_TACTICAL_VIEWPORT_TILE_COLOR,
};
use trnm_rts_bevy_runtime as rts_bevy_runtime;

fn selection_combat_focus_route_tiles(runtime: &NativeFirstPlayableRuntime) -> Vec<(i32, i32)> {
    first_contact_tiles::selection_combat_focus_route_tiles(runtime)
}

fn route_clearance_tiles(runtime: &NativeFirstPlayableRuntime) -> Vec<(i32, i32)> {
    let feedback = classic_first_contact_command_feedback();
    first_contact_tiles::route_clearance_tiles(runtime, feedback.target_tile, feedback.blocked_tile)
}

#[allow(clippy::too_many_arguments)]
fn draw_route_clearance_gutters(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    runtime: &NativeFirstPlayableRuntime,
    map_x: i32,
    map_y: i32,
    cell_w: i32,
    cell_h: i32,
) {
    let gutter_fill = classic_mix_color(CLASSIC_RTS_TACTICAL_VIEWPORT_TILE_COLOR, 0x010302, 1, 3);
    let gutter_edge = classic_darken(CLASSIC_RTS_SELECTION_FEEDBACK_ACK_COLOR, 1, 6);
    for tile in route_clearance_tiles(runtime) {
        let (tile_x, tile_y) =
            classic_first_contact_tile_screen(map_x, map_y, cell_w, cell_h, tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            tile_x + 4,
            tile_y + 4,
            (cell_w - 8).max(8),
            (cell_h - 8).max(6),
            gutter_fill,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            tile_x + cell_w / 2 - 8,
            tile_y + cell_h / 2 - 1,
            16,
            2,
            gutter_edge,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_focus_corner_brackets(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    cx: i32,
    cy: i32,
    half_w: i32,
    half_h: i32,
    color: u32,
) {
    let arm_w = (half_w / 2).max(7);
    let arm_h = (half_h / 2).max(5);
    for (sx, sy) in [(-1, -1), (1, -1), (-1, 1), (1, 1)] {
        let x = cx + sx * half_w;
        let y = cy + sy * half_h;
        let horizontal_x = if sx < 0 { x } else { x - arm_w };
        let vertical_y = if sy < 0 { y } else { y - arm_h };
        classic_draw_rect(buffer, width, height, horizontal_x, y, arm_w, 3, color);
        classic_draw_rect(buffer, width, height, x, vertical_y, 3, arm_h, color);
    }
}

fn target_callout_label(runtime: &NativeFirstPlayableRuntime) -> String {
    first_contact_readouts::target_callout_label(runtime)
}

#[allow(clippy::too_many_arguments)]
fn draw_target_callout(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    runtime: &NativeFirstPlayableRuntime,
    target_cx: i32,
    target_cy: i32,
) {
    let label = target_callout_label(runtime);
    let callout_x = target_cx + CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_X_OFFSET_PX;
    let callout_y = target_cy + CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_Y_OFFSET_PX;
    let clearance_fill =
        classic_mix_color(CLASSIC_RTS_TACTICAL_VIEWPORT_TILE_COLOR, 0x010302, 1, 2);
    classic_draw_rect(
        buffer,
        width,
        height,
        callout_x - CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_CLEARANCE_PAD_PX,
        callout_y - CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_CLEARANCE_PAD_PX,
        CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_W_PX
            + CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_CLEARANCE_PAD_PX * 2,
        CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_H_PX
            + CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_CLEARANCE_PAD_PX * 2,
        clearance_fill,
    );
    classic_draw_rect(
        buffer,
        width,
        height,
        target_cx + 10,
        target_cy - 10,
        CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_LEADER_CLEARANCE_W_PX,
        CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_LEADER_CLEARANCE_H_PX,
        clearance_fill,
    );
    classic_draw_rect(
        buffer,
        width,
        height,
        callout_x,
        callout_y,
        CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_W_PX,
        CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_H_PX,
        classic_mix_color(CLASSIC_RTS_TACTICAL_VIEWPORT_TILE_COLOR, 0x020403, 1, 3),
    );
    classic_draw_rect(
        buffer,
        width,
        height,
        callout_x,
        callout_y,
        3,
        CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_H_PX,
        CLASSIC_RTS_SELECTION_FEEDBACK_ATTACK_COLOR,
    );
    classic_draw_text(
        buffer,
        width,
        height,
        callout_x + 7,
        callout_y + 3,
        &label,
        1,
        CLASSIC_HUD_TEXT_COLOR,
    );
    classic_draw_rect(
        buffer,
        width,
        height,
        callout_x + 7,
        callout_y + CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_H_PX - 5,
        CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_HEALTH_BAR_W_PX,
        CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_HEALTH_BAR_H_PX,
        CLASSIC_RTS_STRUCTURE_FOUNDATION_SHADOW_COLOR,
    );
    classic_draw_rect(
        buffer,
        width,
        height,
        callout_x + 7,
        callout_y + CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_H_PX - 5,
        (i32::from(runtime.rts_target_health_percent.min(100))
            * CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_HEALTH_BAR_W_PX)
            / 100,
        CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_HEALTH_BAR_H_PX,
        CLASSIC_RTS_STATUS_HEALTH_BAR_COLOR,
    );
    classic_draw_rect(
        buffer,
        width,
        height,
        target_cx + 15,
        target_cy - 8,
        24,
        2,
        CLASSIC_RTS_SELECTION_FEEDBACK_ATTACK_COLOR,
    );
    classic_draw_rect(
        buffer,
        width,
        height,
        callout_x - 5,
        callout_y + CLASSIC_FIRST_CONTACT_TARGET_CALLOUT_H_PX / 2,
        8,
        2,
        classic_darken(CLASSIC_RTS_SELECTION_FEEDBACK_ATTACK_COLOR, 1, 3),
    );
}

#[allow(clippy::too_many_arguments)]
pub(super) fn draw_selection_combat_focus_layer(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    runtime: &NativeFirstPlayableRuntime,
    map_x: i32,
    map_y: i32,
    cell_w: i32,
    cell_h: i32,
) {
    let feedback = classic_first_contact_command_feedback();
    let route_tiles = selection_combat_focus_route_tiles(runtime);
    let target_tile = runtime
        .rts_command_destination_tile
        .as_deref()
        .and_then(classic_parse_rts_tile)
        .unwrap_or_else(|| classic_first_contact_tile_tuple(feedback.target_tile));
    let blocked_tile = classic_first_contact_tile_tuple(feedback.blocked_tile);

    draw_route_clearance_gutters(buffer, width, height, runtime, map_x, map_y, cell_w, cell_h);

    for (index, tile) in route_tiles.iter().enumerate() {
        let (tile_x, tile_y) =
            classic_first_contact_tile_screen(map_x, map_y, cell_w, cell_h, *tile);
        let cx = tile_x + cell_w / 2;
        let cy = tile_y + cell_h / 2;
        classic_draw_rect(
            buffer,
            width,
            height,
            cx - cell_w / 2,
            cy + cell_h / 2 + 1,
            cell_w,
            5,
            CLASSIC_RTS_TACTICAL_VIEWPORT_SHADOW_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            cx - CLASSIC_FIRST_CONTACT_ROUTE_DASH_WIDTH_PX / 2,
            cy + cell_h / 2 - CLASSIC_FIRST_CONTACT_ROUTE_DASH_HEIGHT_PX / 2,
            CLASSIC_FIRST_CONTACT_ROUTE_DASH_WIDTH_PX,
            CLASSIC_FIRST_CONTACT_ROUTE_DASH_HEIGHT_PX,
            if index + 1 == route_tiles.len() {
                CLASSIC_RTS_SELECTION_FEEDBACK_ATTACK_COLOR
            } else {
                CLASSIC_RTS_SELECTION_FEEDBACK_MOVE_COLOR
            },
        );
    }

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
                tile_x + cell_w / 2 - CLASSIC_FIRST_CONTACT_ROUTE_ACK_TICK_WIDTH_PX / 2,
                tile_y + cell_h / 2 - CLASSIC_FIRST_CONTACT_ROUTE_ACK_TICK_HEIGHT_PX / 2,
                CLASSIC_FIRST_CONTACT_ROUTE_ACK_TICK_WIDTH_PX,
                CLASSIC_FIRST_CONTACT_ROUTE_ACK_TICK_HEIGHT_PX,
                CLASSIC_RTS_SELECTION_FEEDBACK_ACK_COLOR,
            );
        }
    }

    for (index, tile_id) in runtime
        .rts_selection_box_tile_ids
        .iter()
        .take(4)
        .enumerate()
    {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (tile_x, tile_y) =
                classic_first_contact_tile_screen(map_x, map_y, cell_w, cell_h, tile);
            let cx = tile_x + cell_w / 2;
            let cy = tile_y + cell_h / 2;
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                cx,
                cy + cell_h / 2,
                (cell_w / 2 + 8).max(10),
                (cell_h / 4 + 4).max(5),
                CLASSIC_RTS_TACTICAL_VIEWPORT_SHADOW_COLOR,
            );
            draw_focus_corner_brackets(
                buffer,
                width,
                height,
                cx,
                cy,
                cell_w / 2 + 6,
                cell_h / 2 + 5,
                CLASSIC_RTS_SELECTION_FEEDBACK_CONFIRM_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                cx - CLASSIC_FIRST_CONTACT_SELECTED_ROLE_BADGE_TICK_W_PX / 2,
                cy - cell_h - 8,
                CLASSIC_FIRST_CONTACT_SELECTED_ROLE_BADGE_TICK_W_PX,
                CLASSIC_FIRST_CONTACT_SELECTED_ROLE_BADGE_TICK_H_PX,
                match index {
                    0 => CLASSIC_RTS_STATUS_HEALTH_BAR_COLOR,
                    1 => CLASSIC_RTS_STATUS_MANA_BAR_COLOR,
                    2 => CLASSIC_RTS_SELECTION_FEEDBACK_ATTACK_COLOR,
                    _ => CLASSIC_RTS_SELECTION_FEEDBACK_ACK_COLOR,
                },
            );
        }
    }

    let (target_x, target_y) =
        classic_first_contact_tile_screen(map_x, map_y, cell_w, cell_h, target_tile);
    let target_cx = target_x + cell_w / 2;
    let target_cy = target_y + cell_h / 2;
    draw_focus_corner_brackets(
        buffer,
        width,
        height,
        target_cx,
        target_cy,
        cell_w + 12,
        cell_h + 8,
        CLASSIC_RTS_SELECTION_FEEDBACK_ATTACK_COLOR,
    );
    classic_draw_rect(
        buffer,
        width,
        height,
        target_cx - CLASSIC_FIRST_CONTACT_TARGET_LOCK_CROSS_LONG_PX / 2,
        target_cy - CLASSIC_FIRST_CONTACT_TARGET_LOCK_CROSS_THICKNESS_PX / 2,
        CLASSIC_FIRST_CONTACT_TARGET_LOCK_CROSS_LONG_PX,
        CLASSIC_FIRST_CONTACT_TARGET_LOCK_CROSS_THICKNESS_PX,
        CLASSIC_RTS_SELECTION_FEEDBACK_ATTACK_COLOR,
    );
    classic_draw_rect(
        buffer,
        width,
        height,
        target_cx - CLASSIC_FIRST_CONTACT_TARGET_LOCK_CROSS_THICKNESS_PX / 2,
        target_cy - CLASSIC_FIRST_CONTACT_TARGET_LOCK_CROSS_LONG_PX / 2,
        CLASSIC_FIRST_CONTACT_TARGET_LOCK_CROSS_THICKNESS_PX,
        CLASSIC_FIRST_CONTACT_TARGET_LOCK_CROSS_LONG_PX,
        CLASSIC_RTS_SELECTION_FEEDBACK_ATTACK_COLOR,
    );
    classic_draw_rect(
        buffer,
        width,
        height,
        target_cx - CLASSIC_FIRST_CONTACT_TARGET_LOCK_ACK_TICK_W_PX / 2,
        target_cy + cell_h + 14,
        CLASSIC_FIRST_CONTACT_TARGET_LOCK_ACK_TICK_W_PX,
        CLASSIC_FIRST_CONTACT_TARGET_LOCK_ACK_TICK_H_PX,
        CLASSIC_RTS_SELECTION_FEEDBACK_ACK_COLOR,
    );
    draw_target_callout(buffer, width, height, runtime, target_cx, target_cy);

    let (blocked_x, blocked_y) =
        classic_first_contact_tile_screen(map_x, map_y, cell_w, cell_h, blocked_tile);
    let blocked_cx = blocked_x + cell_w / 2;
    let blocked_cy = blocked_y + cell_h / 2;
    for slash in 0..6 {
        classic_draw_rect(
            buffer,
            width,
            height,
            blocked_cx - 16 + slash * 6,
            blocked_cy - 16 + slash * 5,
            7,
            4,
            CLASSIC_RTS_SELECTION_FEEDBACK_ERROR_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            blocked_cx + 16 - slash * 6,
            blocked_cy - 16 + slash * 5,
            7,
            4,
            CLASSIC_RTS_SELECTION_FEEDBACK_ERROR_COLOR,
        );
    }
}
