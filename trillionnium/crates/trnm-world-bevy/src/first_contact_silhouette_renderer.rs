#![cfg(not(target_os = "android"))]

use crate::{
    classic_darken, classic_draw_iso_ellipse, classic_draw_rect, classic_first_contact_tile_screen,
    first_contact_palette, first_contact_renderer_readability,
    CLASSIC_FIRST_CONTACT_PLAYER_COMMAND_CORE_FACTION_TICK_COUNT,
    CLASSIC_FIRST_CONTACT_PLAYER_COMMAND_CORE_FACTION_TICK_H_PX,
    CLASSIC_FIRST_CONTACT_PLAYER_COMMAND_CORE_FACTION_TICK_W_PX, CLASSIC_ISO_OUTLINE_COLOR,
    CLASSIC_RTS_CAPTURE_BAR_COLOR, CLASSIC_RTS_COMMANDER_AURA_COLOR,
    CLASSIC_RTS_HARVEST_NODE_COLOR, CLASSIC_RTS_MODEL_IDENTITY_FACTION_COLOR,
    CLASSIC_RTS_STRUCTURE_HEALTH_COLOR, CLASSIC_RTS_STRUCTURE_REPAIR_BEAM_COLOR,
    CLASSIC_RTS_TACTICAL_VIEWPORT_SHADOW_COLOR,
};
use trnm_rts_data::first_contact_samples;

fn unit_samples() -> Vec<((i32, i32), &'static str, &'static str)> {
    first_contact_samples::silhouette_unit_samples()
}

fn structure_samples() -> Vec<((i32, i32), &'static str, &'static str)> {
    first_contact_samples::silhouette_structure_samples()
}

fn terrain_samples() -> Vec<((i32, i32), &'static str, &'static str)> {
    first_contact_samples::silhouette_terrain_samples()
}

fn unit_color(role: &str) -> u32 {
    first_contact_palette::silhouette_unit_color(role)
}

fn structure_color(kind: &str) -> u32 {
    first_contact_palette::silhouette_structure_color(kind)
}

fn terrain_color(kind: &str) -> u32 {
    first_contact_palette::silhouette_terrain_color(kind)
}

#[allow(clippy::too_many_arguments)]
fn draw_unit(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    map_x: i32,
    map_y: i32,
    cell_w: i32,
    cell_h: i32,
    tile: (i32, i32),
    role: &str,
    signature: &str,
) {
    let (tile_x, tile_y) = classic_first_contact_tile_screen(map_x, map_y, cell_w, cell_h, tile);
    let cx = tile_x + cell_w / 2;
    let cy = tile_y + cell_h / 2;
    let color = unit_color(role);
    classic_draw_iso_ellipse(
        buffer,
        width,
        height,
        cx,
        cy + cell_h / 2 + 2,
        (cell_w / 2 + 4).max(8),
        (cell_h / 4 + 2).max(4),
        CLASSIC_RTS_TACTICAL_VIEWPORT_SHADOW_COLOR,
    );
    classic_draw_rect(
        buffer,
        width,
        height,
        cx - 5,
        cy - cell_h / 2 - 2,
        10,
        cell_h + 7,
        CLASSIC_ISO_OUTLINE_COLOR,
    );
    classic_draw_rect(
        buffer,
        width,
        height,
        cx - 3,
        cy - cell_h / 2,
        6,
        cell_h + 3,
        color,
    );
    match signature {
        "cargo_pack" => {
            classic_draw_rect(
                buffer,
                width,
                height,
                cx + 5,
                cy - 2,
                8,
                7,
                CLASSIC_RTS_HARVEST_NODE_COLOR,
            );
            classic_draw_rect(buffer, width, height, cx - 12, cy + 4, 24, 3, color);
        }
        "sensor_mast" => {
            classic_draw_rect(
                buffer,
                width,
                height,
                cx,
                cy - cell_h - 6,
                2,
                cell_h + 8,
                color,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                cx - 10,
                cy - cell_h - 3,
                20,
                3,
                color,
            );
        }
        "shield_plate" => {
            classic_draw_rect(buffer, width, height, cx - 12, cy - 2, 24, 5, color);
            classic_draw_rect(
                buffer,
                width,
                height,
                cx - 8,
                cy + 3,
                16,
                4,
                CLASSIC_RTS_STRUCTURE_HEALTH_COLOR,
            );
        }
        "relay_courier" => {
            classic_draw_rect(
                buffer,
                width,
                height,
                cx - 9,
                cy - cell_h / 2 - 5,
                18,
                4,
                CLASSIC_RTS_STRUCTURE_REPAIR_BEAM_COLOR,
            );
            classic_draw_rect(buffer, width, height, cx + 7, cy - 6, 4, cell_h + 9, color);
        }
        _ => {}
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_structure(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    map_x: i32,
    map_y: i32,
    cell_w: i32,
    cell_h: i32,
    tile: (i32, i32),
    kind: &str,
    signature: &str,
    player_screen: bool,
    target_tile: (i32, i32),
) {
    let (tile_x, tile_y) = classic_first_contact_tile_screen(map_x, map_y, cell_w, cell_h, tile);
    let cx = tile_x + cell_w / 2;
    let cy = tile_y + cell_h / 2;
    let color = if player_screen && kind == "command_core" && signature == "stepped_roof_core" {
        classic_darken(CLASSIC_RTS_MODEL_IDENTITY_FACTION_COLOR, 3, 5)
    } else {
        structure_color(kind)
    };
    classic_draw_rect(
        buffer,
        width,
        height,
        cx - cell_w - 5,
        cy + cell_h + 6,
        cell_w * 2 + 10,
        5,
        CLASSIC_RTS_TACTICAL_VIEWPORT_SHADOW_COLOR,
    );
    match signature {
        "stepped_roof_core" => {
            for step in 0..3 {
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    cx - cell_w + step * 6,
                    cy - cell_h * 2 - 10 + step * 7,
                    cell_w * 2 - step * 12,
                    5,
                    color,
                );
            }
            classic_draw_rect(
                buffer,
                width,
                height,
                cx - 4,
                cy - cell_h * 2 - 16,
                8,
                14,
                color,
            );
            if player_screen {
                let tick_color = classic_darken(CLASSIC_RTS_MODEL_IDENTITY_FACTION_COLOR, 1, 3);
                for tick in 0..CLASSIC_FIRST_CONTACT_PLAYER_COMMAND_CORE_FACTION_TICK_COUNT {
                    classic_draw_rect(
                        buffer,
                        width,
                        height,
                        cx - cell_w + 5 + tick as i32 * 12,
                        cy - cell_h + 2 + (tick as i32 % 2),
                        CLASSIC_FIRST_CONTACT_PLAYER_COMMAND_CORE_FACTION_TICK_W_PX,
                        CLASSIC_FIRST_CONTACT_PLAYER_COMMAND_CORE_FACTION_TICK_H_PX,
                        tick_color,
                    );
                }
            } else {
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    cx - cell_w,
                    cy - cell_h + 2,
                    cell_w * 2,
                    3,
                    CLASSIC_RTS_MODEL_IDENTITY_FACTION_COLOR,
                );
            }
        }
        "tall_signal_mast" => {
            classic_draw_rect(
                buffer,
                width,
                height,
                cx - 4,
                cy - cell_h * 3,
                8,
                cell_h * 3,
                color,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                cx - cell_w,
                cy - cell_h * 2,
                cell_w * 2,
                4,
                CLASSIC_RTS_STRUCTURE_REPAIR_BEAM_COLOR,
            );
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                cx,
                cy - cell_h * 2 - 2,
                (cell_w / 2 + 2).max(7),
                (cell_h / 4 + 1).max(3),
                CLASSIC_RTS_COMMANDER_AURA_COLOR,
            );
        }
        "vertical_beacon_spire" => {
            if first_contact_renderer_readability::player_screen_secondary_beacon_body(
                tile,
                kind,
                signature,
                player_screen,
                target_tile,
            ) {
                let quiet =
                    first_contact_renderer_readability::lower_secondary_beacon_art_color(color);
                let edge = classic_darken(quiet, 1, 4);
                for (dx, dy, cue_width, cue_color) in [
                    (-4, -cell_h * 2 - 6, 8, quiet),
                    (-3, -cell_h * 2 + 4, 6, edge),
                    (-4, -cell_h + 2, 8, quiet),
                ] {
                    classic_draw_rect(
                        buffer,
                        width,
                        height,
                        cx + dx,
                        cy + dy,
                        cue_width,
                        2,
                        cue_color,
                    );
                }
                for (dx, dy, cue_height) in [(-2, -cell_h * 2 - 2, 6), (0, -cell_h - 6, 6)] {
                    classic_draw_rect(buffer, width, height, cx + dx, cy + dy, 2, cue_height, edge);
                }
                return;
            }
            classic_draw_rect(
                buffer,
                width,
                height,
                cx - cell_w / 2,
                cy - cell_h * 2 - 8,
                cell_w,
                cell_h * 3,
                classic_darken(color, 1, 5),
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                cx - 3,
                cy - cell_h * 3,
                6,
                cell_h * 4,
                color,
            );
            if !(player_screen && tile != target_tile) {
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    cx - cell_w,
                    cy - cell_h - 5,
                    cell_w * 2,
                    4,
                    CLASSIC_RTS_CAPTURE_BAR_COLOR,
                );
            }
        }
        _ => {}
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_terrain_marker(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    map_x: i32,
    map_y: i32,
    cell_w: i32,
    cell_h: i32,
    tile: (i32, i32),
    kind: &str,
    signature: &str,
    player_screen: bool,
    target_tile: (i32, i32),
) {
    let (tile_x, tile_y) = classic_first_contact_tile_screen(map_x, map_y, cell_w, cell_h, tile);
    let cx = tile_x + cell_w / 2;
    let cy = tile_y + cell_h / 2;
    let color = terrain_color(kind);
    match signature {
        "base_corner_frame" => {
            for (dx, dy) in [(-1, -1), (1, -1), (-1, 1), (1, 1)] {
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    cx + dx * cell_w - 5,
                    cy + dy * cell_h - 2,
                    10,
                    4,
                    color,
                );
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    cx + dx * cell_w - 2,
                    cy + dy * cell_h - 5,
                    4,
                    10,
                    color,
                );
            }
        }
        "flux_glint_cluster" => {
            for offset in [(-8, 0), (0, -4), (8, 2)] {
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    cx + offset.0 - 2,
                    cy + offset.1 - 2,
                    4,
                    4,
                    color,
                );
            }
        }
        "beacon_lane_rim" => {
            if player_screen && tile != target_tile {
                let quiet =
                    first_contact_renderer_readability::lower_secondary_beacon_art_color(color);
                let edge = classic_darken(quiet, 1, 4);
                for (x, y, w, h, cue_color) in [
                    (cx - cell_w / 2, cy - cell_h / 2, 8, 2, quiet),
                    (cx + cell_w / 2 - 8, cy + cell_h / 2 - 2, 8, 2, quiet),
                    (cx - 1, cy - cell_h / 2 + 2, 2, 6, edge),
                    (cx - 1, cy + cell_h / 2 - 8, 2, 6, edge),
                ] {
                    classic_draw_rect(buffer, width, height, x, y, w, h, cue_color);
                }
                return;
            }
            classic_draw_rect(
                buffer,
                width,
                height,
                cx - cell_w,
                cy - 2,
                cell_w * 2,
                4,
                color,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                cx - 2,
                cy - cell_h,
                4,
                cell_h * 2,
                color,
            );
        }
        "basin_cross_rim" => {
            classic_draw_rect(
                buffer,
                width,
                height,
                cx - cell_w * 2,
                cy - 1,
                cell_w * 4,
                2,
                color,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                cx - 1,
                cy - cell_h * 2,
                2,
                cell_h * 4,
                color,
            );
        }
        _ => {}
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn draw_readability_layer(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    map_x: i32,
    map_y: i32,
    cell_w: i32,
    cell_h: i32,
    player_screen: bool,
    target_tile: (i32, i32),
) {
    for (tile, kind, signature) in terrain_samples() {
        draw_terrain_marker(
            buffer,
            width,
            height,
            map_x,
            map_y,
            cell_w,
            cell_h,
            tile,
            kind,
            signature,
            player_screen,
            target_tile,
        );
    }
    for (tile, kind, signature) in structure_samples() {
        draw_structure(
            buffer,
            width,
            height,
            map_x,
            map_y,
            cell_w,
            cell_h,
            tile,
            kind,
            signature,
            player_screen,
            target_tile,
        );
    }
    for (tile, role, signature) in unit_samples() {
        draw_unit(
            buffer, width, height, map_x, map_y, cell_w, cell_h, tile, role, signature,
        );
    }
}
