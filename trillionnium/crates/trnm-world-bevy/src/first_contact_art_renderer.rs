#![cfg(not(target_os = "android"))]

use crate::{
    classic_darken, classic_draw_iso_ellipse, classic_draw_rect, classic_first_contact_tile_screen,
    classic_lighten, first_contact_palette, first_contact_renderer_readability,
    CLASSIC_RTS_OBJECTIVE_COLOR, CLASSIC_RTS_PRODUCT_LANE_COLOR,
    CLASSIC_RTS_STRUCTURE_FOUNDATION_SHADOW_COLOR, CLASSIC_RTS_TACTICAL_VIEWPORT_SHADOW_COLOR,
};
use trnm_rts_data::first_contact_samples;

pub(super) fn terrain_samples() -> Vec<((i32, i32), &'static str, &'static str)> {
    first_contact_samples::art_terrain_samples()
}

pub(super) fn building_samples() -> Vec<((i32, i32), &'static str, &'static str)> {
    first_contact_samples::art_building_samples()
}

pub(super) fn landmark_samples() -> Vec<((i32, i32), &'static str, &'static str)> {
    first_contact_samples::art_landmark_samples()
}

pub(super) fn terrain_color(role: &str) -> u32 {
    first_contact_palette::art_terrain_color(role)
}

pub(super) fn building_color(role: &str) -> u32 {
    first_contact_palette::art_building_color(role)
}

pub(super) fn landmark_color(role: &str) -> u32 {
    first_contact_palette::art_landmark_color(role)
}

#[allow(clippy::too_many_arguments)]
pub(super) fn draw_terrain_material_depth_detail(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    map_x: i32,
    map_y: i32,
    cell_w: i32,
    cell_h: i32,
    tile: (i32, i32),
    role: &str,
) {
    let (tile_x, tile_y) = classic_first_contact_tile_screen(map_x, map_y, cell_w, cell_h, tile);
    let cx = tile_x + cell_w / 2;
    let cy = tile_y + cell_h / 2;
    let color = terrain_color(role);
    if first_contact_renderer_readability::lower_secondary_beacon_lane(tile, role) {
        let quiet = first_contact_renderer_readability::lower_secondary_beacon_art_color(color);
        classic_draw_rect(
            buffer,
            width,
            height,
            cx - cell_w / 2,
            cy + cell_h / 2 - 1,
            cell_w,
            1,
            quiet,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            cx - 1,
            cy - 2,
            2,
            (cell_h / 2).max(4),
            classic_darken(quiet, 1, 4),
        );
        return;
    }
    match role {
        "base_concrete" => {
            let bevel = classic_darken(CLASSIC_RTS_STRUCTURE_FOUNDATION_SHADOW_COLOR, 1, 3);
            classic_draw_rect(
                buffer,
                width,
                height,
                cx - cell_w,
                cy + cell_h - 3,
                cell_w * 2,
                3,
                bevel,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                cx + cell_w - 3,
                cy - cell_h,
                3,
                cell_h * 2,
                classic_darken(color, 1, 4),
            );
        }
        "resource_crystal" => {
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                cx + 3,
                cy + cell_h / 3,
                (cell_w / 2 + 3).max(8),
                (cell_h / 5 + 2).max(4),
                classic_darken(color, 1, 6),
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                cx - cell_w / 2,
                cy + cell_h / 3,
                cell_w,
                2,
                classic_darken(CLASSIC_RTS_TACTICAL_VIEWPORT_SHADOW_COLOR, 1, 3),
            );
        }
        "beacon_lane" => {
            let rail = classic_darken(CLASSIC_RTS_OBJECTIVE_COLOR, 1, 5);
            for offset in [-cell_h / 2, cell_h / 2] {
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    cx - cell_w,
                    cy + offset - 1,
                    cell_w * 2,
                    2,
                    rail,
                );
            }
            classic_draw_rect(
                buffer,
                width,
                height,
                cx - cell_w / 2,
                cy - 1,
                cell_w,
                2,
                classic_darken(CLASSIC_RTS_PRODUCT_LANE_COLOR, 1, 4),
            );
        }
        "basin_floor" => {
            let fracture = classic_darken(CLASSIC_RTS_PRODUCT_LANE_COLOR, 1, 4);
            for (dx, dy, w) in [(-18, -10, 12), (-7, -2, 18), (8, 7, 14), (-2, 14, 10)] {
                classic_draw_rect(buffer, width, height, cx + dx, cy + dy, w, 2, fracture);
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    cx + dx + w / 2,
                    cy + dy - 3,
                    2,
                    7,
                    classic_darken(fracture, 1, 4),
                );
            }
        }
        _ => {}
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn draw_terrain_detail(
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
    let color = terrain_color(role);
    match signature {
        "foundation_panel_seams" => {
            classic_draw_rect(
                buffer,
                width,
                height,
                cx - cell_w,
                cy - 1,
                cell_w * 2,
                2,
                color,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                cx - 1,
                cy - cell_h,
                2,
                cell_h * 2,
                color,
            );
            for offset in [-cell_w / 2, cell_w / 2] {
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    cx + offset - 1,
                    cy - cell_h / 2,
                    2,
                    cell_h,
                    classic_darken(color, 1, 3),
                );
            }
        }
        "flux_crystal_shards" => {
            for (dx, dy, size) in [(-9, 1, 5), (0, -6, 7), (9, 2, 4), (3, 7, 3)] {
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    cx + dx - size / 2,
                    cy + dy - size,
                    size,
                    size * 2,
                    color,
                );
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    cx + dx - size,
                    cy + dy - 1,
                    size * 2,
                    2,
                    classic_darken(color, 1, 4),
                );
            }
        }
        "painted_lane_chevrons" => {
            if first_contact_renderer_readability::lower_secondary_beacon_art_detail(
                tile, role, signature,
            ) {
                let quiet =
                    first_contact_renderer_readability::lower_secondary_beacon_art_color(color);
                classic_draw_rect(buffer, width, height, cx - 8, cy - 1, 16, 2, quiet);
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    cx - 1,
                    cy - cell_h / 3,
                    2,
                    (cell_h * 2 / 3).max(4),
                    classic_darken(quiet, 1, 4),
                );
                return;
            }
            for offset in [-cell_w, 0, cell_w] {
                classic_draw_rect(buffer, width, height, cx + offset - 8, cy - 2, 16, 3, color);
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    cx + offset + 4,
                    cy - cell_h / 2,
                    3,
                    cell_h,
                    color,
                );
            }
        }
        "cracked_plaza_cross" => {
            for step in -2..=2 {
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    cx + step * 7 - 1,
                    cy + step * 3,
                    3,
                    12,
                    color,
                );
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    cx - step * 6,
                    cy + step * 5 - 1,
                    14,
                    2,
                    classic_darken(color, 1, 3),
                );
            }
        }
        _ => {}
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn draw_building_detail(
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
    let color = building_color(role);
    match signature {
        "lit_window_rows" => {
            for row in 0..3 {
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    cx - cell_w + 7,
                    cy - cell_h * 2 + 8 + row * 8,
                    cell_w * 2 - 14,
                    2,
                    color,
                );
                for col in 0..4 {
                    classic_draw_rect(
                        buffer,
                        width,
                        height,
                        cx - cell_w + 10 + col * 9,
                        cy - cell_h * 2 + 5 + row * 8,
                        4,
                        5,
                        classic_darken(color, 1, 4),
                    );
                }
            }
        }
        "antenna_band_panels" => {
            for row in 0..4 {
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    cx - cell_w / 2 + 4,
                    cy - cell_h * 2 + row * 9,
                    cell_w - 8,
                    3,
                    color,
                );
            }
            classic_draw_rect(
                buffer,
                width,
                height,
                cx - 2,
                cy - cell_h * 3 - 4,
                4,
                12,
                color,
            );
        }
        "glowing_spire_panels" => {
            for row in 0..5 {
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    cx - 5,
                    cy - cell_h * 3 + row * 8,
                    10,
                    3,
                    color,
                );
            }
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                cx,
                cy - cell_h * 2,
                (cell_w / 2).max(6),
                3,
                color,
            );
        }
        _ => {}
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn draw_landmark_detail(
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
    let color = landmark_color(role);
    match signature {
        "base_gate_lamps" => {
            classic_draw_rect(
                buffer,
                width,
                height,
                cx - cell_w,
                cy - cell_h / 2,
                cell_w * 2,
                3,
                color,
            );
            for dx in [-cell_w / 2, cell_w / 2] {
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    cx + dx - 3,
                    cy - cell_h,
                    6,
                    12,
                    color,
                );
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    cx + dx - 5,
                    cy - cell_h - 4,
                    10,
                    3,
                    classic_lighten(color, 1, 4),
                );
            }
        }
        "crystal_shadow_sparkles" => {
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                cx,
                cy + 5,
                (cell_w / 2).max(7),
                4,
                classic_darken(color, 1, 5),
            );
            for (dx, dy) in [(-11, -8), (-2, -13), (8, -6), (13, 2)] {
                classic_draw_rect(buffer, width, height, cx + dx, cy + dy, 3, 7, color);
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    cx + dx - 2,
                    cy + dy + 2,
                    7,
                    2,
                    classic_lighten(color, 1, 5),
                );
            }
        }
        "lane_power_pylons" => {
            if first_contact_renderer_readability::lower_secondary_beacon_art_detail(
                tile, role, signature,
            ) {
                let quiet =
                    first_contact_renderer_readability::lower_secondary_beacon_art_color(color);
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    cx - cell_w / 2,
                    cy - 1,
                    cell_w,
                    2,
                    quiet,
                );
                for dx in [-cell_w / 3, cell_w / 3] {
                    classic_draw_rect(
                        buffer,
                        width,
                        height,
                        cx + dx - 2,
                        cy - cell_h / 2,
                        4,
                        cell_h,
                        classic_darken(quiet, 1, 5),
                    );
                }
                return;
            }
            for dx in [-cell_w / 2, cell_w / 2] {
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    cx + dx - 3,
                    cy - cell_h,
                    6,
                    18,
                    color,
                );
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    cx + dx - 8,
                    cy - cell_h + 4,
                    16,
                    3,
                    classic_lighten(color, 1, 4),
                );
            }
            classic_draw_rect(
                buffer,
                width,
                height,
                cx - cell_w / 2,
                cy - cell_h + 10,
                cell_w,
                2,
                color,
            );
        }
        "crater_scuff_marks" => {
            for (dx, dy, w) in [(-16, -8, 18), (-6, 1, 22), (10, 9, 15)] {
                classic_draw_rect(buffer, width, height, cx + dx, cy + dy, w, 2, color);
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    cx + dx + w / 2,
                    cy + dy - 4,
                    2,
                    8,
                    classic_darken(color, 1, 4),
                );
            }
        }
        "relay_ground_cables" => {
            classic_draw_rect(
                buffer,
                width,
                height,
                cx - cell_w / 2,
                cy + cell_h / 2,
                cell_w,
                3,
                color,
            );
            for offset in [-cell_w / 3, 0, cell_w / 3] {
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    cx + offset - 2,
                    cy - cell_h,
                    4,
                    cell_h + 16,
                    classic_lighten(color, 1, 4),
                );
            }
        }
        "beacon_capture_rings" => {
            if first_contact_renderer_readability::secondary_beacon_capture_ring_detail(
                tile, role, signature,
            ) {
                let quiet =
                    first_contact_renderer_readability::lower_secondary_beacon_art_color(color);
                let edge = classic_darken(quiet, 1, 4);
                for (x, y, cue_color) in [
                    (cx - 4, cy - cell_h / 2, quiet),
                    (cx - cell_w / 2, cy - cell_h / 5, edge),
                    (cx + cell_w / 2 - 8, cy + cell_h / 5, edge),
                    (cx - 4, cy + cell_h / 2 - 2, quiet),
                ] {
                    classic_draw_rect(buffer, width, height, x, y, 8, 2, cue_color);
                }
                return;
            }
            if first_contact_renderer_readability::lower_secondary_beacon_art_detail(
                tile, role, signature,
            ) {
                let quiet =
                    first_contact_renderer_readability::lower_secondary_beacon_art_color(color);
                classic_draw_iso_ellipse(
                    buffer,
                    width,
                    height,
                    cx,
                    cy,
                    (cell_w / 2).max(8),
                    3,
                    quiet,
                );
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    cx - cell_w / 2,
                    cy - 1,
                    cell_w,
                    2,
                    classic_darken(quiet, 1, 4),
                );
                return;
            }
            for radius in [cell_w / 2, cell_w] {
                classic_draw_iso_ellipse(
                    buffer,
                    width,
                    height,
                    cx,
                    cy,
                    radius.max(8),
                    (radius / 3).max(3),
                    color,
                );
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
) {
    for (tile, role, _) in terrain_samples() {
        draw_terrain_material_depth_detail(
            buffer, width, height, map_x, map_y, cell_w, cell_h, tile, role,
        );
    }
    for (tile, role, signature) in terrain_samples() {
        draw_terrain_detail(
            buffer, width, height, map_x, map_y, cell_w, cell_h, tile, role, signature,
        );
    }
    for (tile, role, signature) in building_samples() {
        draw_building_detail(
            buffer, width, height, map_x, map_y, cell_w, cell_h, tile, role, signature,
        );
    }
    for (tile, role, signature) in landmark_samples() {
        draw_landmark_detail(
            buffer, width, height, map_x, map_y, cell_w, cell_h, tile, role, signature,
        );
    }
}
