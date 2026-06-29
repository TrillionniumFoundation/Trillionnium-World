use super::*;
use std::collections::HashSet;

#[cfg(not(target_os = "android"))]
pub(super) fn classic_iso_project(
    origin_x: i32,
    origin_y: i32,
    tile_w: i32,
    tile_h: i32,
    tile: (i32, i32),
) -> (i32, i32) {
    (
        origin_x + (tile.0 - tile.1) * tile_w / 2,
        origin_y + (tile.0 + tile.1) * tile_h / 2,
    )
}

#[cfg(not(target_os = "android"))]
pub(super) fn classic_frame_anchor_color(assets: &ClassicRuntimeAssets, frame_id: &str) -> u32 {
    let Some(frame) = assets.frame_by_id.get(frame_id) else {
        return 0x26352a;
    };
    let sample_x = frame.x + frame.w / 2;
    let sample_y = frame.y + frame.h / 2;
    assets
        .atlas_pixels
        .get(sample_y as usize * assets.manifest.atlas_width as usize + sample_x as usize)
        .copied()
        .unwrap_or(0x26352a)
}

#[cfg(not(target_os = "android"))]
#[allow(clippy::too_many_arguments)]
pub(super) fn classic_draw_iso_diamond(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    center_x: i32,
    top_y: i32,
    tile_w: i32,
    tile_h: i32,
    color: u32,
) {
    let half_w = tile_w / 2;
    let half_h = tile_h / 2;
    let edge = classic_darken(color, 2, 5);
    let highlight = classic_lighten(color, 1, 5);
    for dy in 0..tile_h.max(1) {
        let distance_from_mid = (dy - half_h).abs();
        let span = ((half_h - distance_from_mid).max(0) * half_w) / half_h.max(1);
        let row_color = if dy < half_h {
            classic_mix_color(highlight, color, dy as u32, half_h.max(1) as u32)
        } else {
            classic_mix_color(color, edge, (dy - half_h) as u32, half_h.max(1) as u32)
        };
        classic_draw_rect(
            buffer,
            width,
            height,
            center_x - span,
            top_y + dy,
            span * 2 + 1,
            1,
            row_color,
        );
        if span > 0 {
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - span,
                top_y + dy,
                1,
                1,
                edge,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x + span,
                top_y + dy,
                1,
                1,
                edge,
            );
        }
    }
}

#[cfg(not(target_os = "android"))]
#[allow(clippy::too_many_arguments)]
pub(super) fn classic_draw_iso_tile_cliff_face(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    center_x: i32,
    top_y: i32,
    tile_w: i32,
    tile_h: i32,
    drop_px: i32,
    color: u32,
) {
    let half_w = tile_w / 2;
    let half_h = tile_h / 2;
    let left = (center_x - half_w, top_y + half_h);
    let right = (center_x + half_w, top_y + half_h);
    let bottom = (center_x, top_y + tile_h);
    let left_drop = (center_x - half_w, top_y + half_h + drop_px);
    let right_drop = (center_x + half_w, top_y + half_h + drop_px);
    let bottom_drop = (center_x, top_y + tile_h + drop_px);
    classic_draw_iso_quad(
        buffer,
        width,
        height,
        [left, bottom, bottom_drop, left_drop],
        color,
    );
    classic_draw_iso_quad(
        buffer,
        width,
        height,
        [right, bottom, bottom_drop, right_drop],
        classic_darken(color, 2, 5),
    );
}

#[cfg(not(target_os = "android"))]
#[allow(clippy::too_many_arguments)]
pub(super) fn classic_draw_iso_terrain_detail(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    frame_id: &str,
    center_x: i32,
    top_y: i32,
    tile_w: i32,
    tile_h: i32,
) -> bool {
    match frame_id {
        "tile_road" => {
            classic_draw_iso_diamond(
                buffer,
                width,
                height,
                center_x,
                top_y + 4,
                tile_w - 4,
                tile_h - 2,
                CLASSIC_ISO_ROAD_DETAIL_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - tile_w / 5,
                top_y + tile_h / 2 - 2,
                (tile_w * 2) / 5,
                4,
                classic_darken(CLASSIC_ISO_ROAD_DETAIL_COLOR, 1, 4),
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - tile_w / 8,
                top_y + tile_h / 2 + 5,
                tile_w / 4,
                2,
                CLASSIC_ISO_ROAD_DETAIL_COLOR,
            );
            true
        }
        "tile_water" => {
            classic_draw_iso_diamond(
                buffer,
                width,
                height,
                center_x,
                top_y + 3,
                tile_w - 8,
                tile_h - 5,
                CLASSIC_ISO_WATER_DETAIL_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - tile_w / 3,
                top_y + tile_h / 2 - 5,
                (tile_w * 2) / 3,
                8,
                CLASSIC_ISO_WATER_DETAIL_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - tile_w / 4,
                top_y + tile_h / 2 - 3,
                tile_w / 2,
                2,
                CLASSIC_ISO_WATER_HIGHLIGHT_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - tile_w / 6,
                top_y + tile_h / 2 + 4,
                tile_w / 3,
                1,
                CLASSIC_ISO_WATER_HIGHLIGHT_COLOR,
            );
            true
        }
        "tile_wall" | "tile_roof" | "tile_arena" => {
            classic_draw_iso_tile_cliff_face(
                buffer,
                width,
                height,
                center_x,
                top_y,
                tile_w,
                tile_h,
                if frame_id == "tile_wall" { 12 } else { 8 },
                CLASSIC_ISO_CLIFF_FACE_COLOR,
            );
            true
        }
        _ => false,
    }
}

#[cfg(not(target_os = "android"))]
#[allow(clippy::too_many_arguments)]
pub(super) fn classic_draw_iso_quad(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    points: [(i32, i32); 4],
    color: u32,
) {
    let min_y = points.iter().map(|(_, y)| *y).min().unwrap_or_default();
    let max_y = points.iter().map(|(_, y)| *y).max().unwrap_or_default();
    for y in min_y..=max_y {
        let mut xs = Vec::new();
        for index in 0..points.len() {
            let (x1, y1) = points[index];
            let (x2, y2) = points[(index + 1) % points.len()];
            if y1 == y2 {
                continue;
            }
            let y_min = y1.min(y2);
            let y_max = y1.max(y2);
            if y >= y_min && y < y_max {
                let numerator = (y - y1) * (x2 - x1);
                let denominator = y2 - y1;
                xs.push(x1 + numerator / denominator);
            }
        }
        xs.sort_unstable();
        for pair in xs.chunks(2) {
            if let [left, right] = pair {
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    *left,
                    y,
                    (*right - *left).abs().max(1),
                    1,
                    color,
                );
            }
        }
    }
}

#[cfg(not(target_os = "android"))]
#[allow(clippy::too_many_arguments)]
pub(super) fn classic_draw_iso_prism(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    center_x: i32,
    top_y: i32,
    tile_w: i32,
    tile_h: i32,
    height_px: i32,
    color: u32,
) {
    let half_w = tile_w / 2;
    let half_h = tile_h / 2;
    let roof_top = top_y - height_px;
    let roof = [
        (center_x, roof_top),
        (center_x + half_w, roof_top + half_h),
        (center_x, roof_top + tile_h),
        (center_x - half_w, roof_top + half_h),
    ];
    let base = [
        (center_x, top_y),
        (center_x + half_w, top_y + half_h),
        (center_x, top_y + tile_h),
        (center_x - half_w, top_y + half_h),
    ];
    classic_draw_iso_quad(
        buffer,
        width,
        height,
        [roof[3], roof[2], base[2], base[3]],
        classic_darken(color, 2, 5),
    );
    classic_draw_iso_quad(
        buffer,
        width,
        height,
        [roof[1], roof[2], base[2], base[1]],
        classic_darken(color, 1, 3),
    );
    classic_draw_iso_diamond(
        buffer,
        width,
        height,
        center_x,
        roof_top,
        tile_w,
        tile_h,
        classic_lighten(color, 1, 5),
    );
}

#[cfg(not(target_os = "android"))]
#[allow(clippy::too_many_arguments)]
pub(super) fn classic_draw_iso_ellipse(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    center_x: i32,
    center_y: i32,
    radius_x: i32,
    radius_y: i32,
    color: u32,
) {
    let radius_x = radius_x.max(1);
    let radius_y = radius_y.max(1);
    for dy in -radius_y..=radius_y {
        let y_term = (dy * dy * 1024) / (radius_y * radius_y);
        let span = (((1024 - y_term).max(0) as f32).sqrt() * radius_x as f32 / 32.0) as i32;
        classic_draw_rect(
            buffer,
            width,
            height,
            center_x - span,
            center_y + dy,
            span * 2 + 1,
            1,
            color,
        );
    }
}

#[cfg(not(target_os = "android"))]
pub(super) fn classic_draw_iso_shadow(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    center_x: i32,
    center_y: i32,
    radius_x: i32,
    radius_y: i32,
) {
    let radius_x = radius_x.max(1);
    let radius_y = radius_y.max(1);
    for dy in -radius_y..=radius_y {
        let y_term = (dy * dy * 1024) / (radius_y * radius_y);
        let span = (((1024 - y_term).max(0) as f32).sqrt() * radius_x as f32 / 32.0) as i32;
        classic_draw_rect(
            buffer,
            width,
            height,
            center_x - span,
            center_y + dy,
            span * 2 + 1,
            1,
            CLASSIC_ISO_SHADOW_COLOR,
        );
    }
}

#[cfg(not(target_os = "android"))]
pub(super) fn classic_entity_sprite_scale(
    assets: &ClassicRuntimeAssets,
    frame_id: &str,
    base_scale: u32,
) -> u32 {
    let role = assets
        .frame_by_id
        .get(frame_id)
        .map(|frame| frame.role.as_str())
        .unwrap_or_default();
    if role.contains("actor") || role == "objective_marker" || role == "interaction_marker" {
        base_scale.saturating_add(1)
    } else {
        base_scale
    }
}

#[cfg(not(target_os = "android"))]
#[allow(clippy::too_many_arguments)]
pub(super) fn classic_draw_iso_procedural_model(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    frame_id: &str,
    center_x: i32,
    top_y: i32,
    tile_w: i32,
    tile_h: i32,
) -> bool {
    let base_y = top_y + tile_h;
    match frame_id {
        "model_town_hall" => {
            classic_draw_iso_shadow(buffer, width, height, center_x, base_y + 2, tile_w, 8);
            classic_draw_iso_prism(
                buffer,
                width,
                height,
                center_x,
                top_y + 14,
                tile_w * 2,
                tile_h * 2,
                44,
                CLASSIC_ISO_WALL_COLOR,
            );
            classic_draw_iso_prism(
                buffer,
                width,
                height,
                center_x,
                top_y - 28,
                tile_w * 2 + 16,
                tile_h + 8,
                12,
                CLASSIC_ISO_BLUE_ROOF_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 10,
                top_y + 4,
                20,
                30,
                CLASSIC_ISO_OUTLINE_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 8,
                top_y + 6,
                16,
                26,
                classic_darken(CLASSIC_ISO_WALL_COLOR, 2, 5),
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 26,
                top_y - 4,
                10,
                8,
                CLASSIC_ISO_GOLD_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x + 16,
                top_y - 4,
                10,
                8,
                CLASSIC_ISO_GOLD_COLOR,
            );
            true
        }
        "model_training_hall" => {
            classic_draw_iso_shadow(buffer, width, height, center_x, base_y + 2, tile_w, 7);
            classic_draw_iso_prism(
                buffer,
                width,
                height,
                center_x,
                top_y + 14,
                tile_w * 2,
                tile_h + 12,
                36,
                CLASSIC_ISO_STONE_COLOR,
            );
            classic_draw_iso_prism(
                buffer,
                width,
                height,
                center_x,
                top_y - 20,
                tile_w * 2 + 10,
                tile_h,
                10,
                CLASSIC_ISO_ROOF_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x + 18,
                top_y - 8,
                24,
                5,
                CLASSIC_ISO_GOLD_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x + 16,
                top_y - 11,
                28,
                3,
                CLASSIC_ISO_OUTLINE_COLOR,
            );
            true
        }
        "model_waygate" => {
            classic_draw_iso_shadow(buffer, width, height, center_x, base_y + 2, tile_w / 2, 6);
            classic_draw_iso_prism(
                buffer,
                width,
                height,
                center_x - 18,
                top_y + 8,
                tile_w / 2,
                tile_h,
                48,
                CLASSIC_ISO_STONE_COLOR,
            );
            classic_draw_iso_prism(
                buffer,
                width,
                height,
                center_x + 18,
                top_y + 8,
                tile_w / 2,
                tile_h,
                48,
                CLASSIC_ISO_STONE_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 28,
                top_y - 47,
                56,
                3,
                CLASSIC_ISO_OUTLINE_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 28,
                top_y - 44,
                56,
                8,
                CLASSIC_ISO_STONE_COLOR,
            );
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                center_x,
                top_y - 12,
                20,
                28,
                CLASSIC_ISO_MAGIC_COLOR,
            );
            true
        }
        "model_coliseum_stands" => {
            classic_draw_iso_shadow(buffer, width, height, center_x, base_y + 4, tile_w, 8);
            for tier in 0..3 {
                classic_draw_iso_prism(
                    buffer,
                    width,
                    height,
                    center_x,
                    top_y + 18 - tier * 12,
                    tile_w * 2 - tier * 12,
                    tile_h + 8,
                    12,
                    if tier % 2 == 0 {
                        CLASSIC_ISO_STONE_COLOR
                    } else {
                        classic_lighten(CLASSIC_ISO_STONE_COLOR, 1, 5)
                    },
                );
            }
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 44,
                top_y - 22,
                88,
                4,
                CLASSIC_ISO_BANNER_COLOR,
            );
            true
        }
        "model_tree_cluster_large" => {
            classic_draw_iso_shadow(buffer, width, height, center_x, base_y, tile_w, 8);
            for (dx, dy, radius_x, radius_y, color) in [
                (-26, -34, 25, 15, CLASSIC_ISO_CANOPY_COLOR),
                (-7, -45, 28, 17, CLASSIC_ISO_CANOPY_LIGHT_COLOR),
                (
                    20,
                    -36,
                    26,
                    16,
                    classic_darken(CLASSIC_ISO_CANOPY_COLOR, 1, 5),
                ),
                (2, -27, 33, 15, CLASSIC_ISO_CANOPY_COLOR),
            ] {
                classic_draw_iso_ellipse(
                    buffer,
                    width,
                    height,
                    center_x + dx,
                    top_y + dy,
                    radius_x,
                    radius_y,
                    color,
                );
            }
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 4,
                top_y - 26,
                8,
                36,
                classic_darken(CLASSIC_ISO_WALL_COLOR, 1, 3),
            );
            true
        }
        frame if classic_art_pack_neutral_unit_frame(frame) => {
            let (body, accent, trim, dark, skin) = if frame.starts_with("actor_guard") {
                (
                    CLASSIC_ISO_UNIT_GUARD_COLOR,
                    0xa8d8ff,
                    0xe8e0bd,
                    0x2f3950,
                    0xcaa878,
                )
            } else if frame.starts_with("actor_worker") {
                (
                    CLASSIC_ISO_UNIT_WORKER_COLOR,
                    0xf0be70,
                    0xffe2a6,
                    0x4f3726,
                    0xc6925f,
                )
            } else {
                (
                    CLASSIC_ISO_UNIT_CREEP_COLOR,
                    0xd0a2ff,
                    0xffe2a6,
                    0x3a2448,
                    0x9a79bd,
                )
            };
            classic_draw_iso_shadow(buffer, width, height, center_x, base_y, 18, 5);
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                center_x,
                base_y - 1,
                13,
                5,
                CLASSIC_ISO_UNIT_RING_COLOR,
            );
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                center_x,
                base_y - 1,
                8,
                3,
                CLASSIC_ISO_FOUNDATION_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 6,
                base_y - 35,
                12,
                8,
                CLASSIC_ISO_OUTLINE_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 8,
                base_y - 27,
                16,
                21,
                CLASSIC_ISO_OUTLINE_COLOR,
            );
            classic_draw_rect(buffer, width, height, center_x - 4, base_y - 34, 8, 6, skin);
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 5,
                base_y - 28,
                10,
                17,
                body,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 8,
                base_y - 25,
                16,
                4,
                accent,
            );
            classic_draw_rect(buffer, width, height, center_x - 4, base_y - 20, 8, 2, trim);
            classic_draw_rect(buffer, width, height, center_x - 6, base_y - 11, 4, 7, dark);
            classic_draw_rect(buffer, width, height, center_x + 2, base_y - 11, 4, 7, dark);
            if frame.starts_with("actor_guard") {
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    center_x - 12,
                    base_y - 25,
                    5,
                    13,
                    accent,
                );
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    center_x - 11,
                    base_y - 22,
                    3,
                    7,
                    body,
                );
                if frame.ends_with("_attack") {
                    classic_draw_rect(
                        buffer,
                        width,
                        height,
                        center_x + 9,
                        base_y - 31,
                        3,
                        21,
                        trim,
                    );
                    classic_draw_rect(
                        buffer,
                        width,
                        height,
                        center_x + 12,
                        base_y - 33,
                        8,
                        3,
                        trim,
                    );
                }
            } else if frame.starts_with("actor_worker") {
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    center_x - 11,
                    base_y - 23,
                    5,
                    13,
                    dark,
                );
                if frame.ends_with("_carry") {
                    classic_draw_rect(
                        buffer,
                        width,
                        height,
                        center_x + 9,
                        base_y - 30,
                        15,
                        11,
                        CLASSIC_ISO_OUTLINE_COLOR,
                    );
                    classic_draw_rect(
                        buffer,
                        width,
                        height,
                        center_x + 10,
                        base_y - 29,
                        13,
                        9,
                        CLASSIC_ISO_GOLD_COLOR,
                    );
                } else {
                    classic_draw_rect(
                        buffer,
                        width,
                        height,
                        center_x - 13,
                        base_y - 30,
                        5,
                        18,
                        trim,
                    );
                }
            } else {
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    center_x - 8,
                    base_y - 38,
                    5,
                    5,
                    accent,
                );
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    center_x + 3,
                    base_y - 38,
                    5,
                    5,
                    accent,
                );
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    center_x - 10,
                    base_y - 24,
                    5,
                    12,
                    dark,
                );
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    center_x + 5,
                    base_y - 24,
                    5,
                    12,
                    dark,
                );
                if frame.ends_with("_attack") {
                    classic_draw_rect(
                        buffer,
                        width,
                        height,
                        center_x + 10,
                        base_y - 27,
                        10,
                        4,
                        accent,
                    );
                    classic_draw_rect(
                        buffer,
                        width,
                        height,
                        center_x + 16,
                        base_y - 25,
                        5,
                        6,
                        CLASSIC_ISO_UNIT_DAMAGE_COLOR,
                    );
                }
            }
            classic_draw_rts_unit_model_depth_marks(buffer, width, height, frame, center_x, base_y);
            classic_draw_rts_action_cadence_marks(buffer, width, height, frame, center_x, base_y);
            true
        }
        "doodad_rock_cluster" => {
            classic_draw_iso_shadow(buffer, width, height, center_x, base_y, tile_w / 3, 4);
            for (dx, dy, radius_x, radius_y) in [(-10, -4, 9, 5), (3, -8, 12, 7), (14, -2, 7, 4)] {
                classic_draw_iso_ellipse(
                    buffer,
                    width,
                    height,
                    center_x + dx,
                    base_y + dy,
                    radius_x,
                    radius_y,
                    CLASSIC_ISO_DOODAD_STONE_COLOR,
                );
            }
            true
        }
        "doodad_barrel_stack" => {
            classic_draw_iso_shadow(buffer, width, height, center_x, base_y, tile_w / 4, 4);
            for (dx, dy, height_px) in [(-8, -2, 14), (5, -4, 18), (14, 1, 12)] {
                classic_draw_iso_prism(
                    buffer,
                    width,
                    height,
                    center_x + dx,
                    base_y + dy - 8,
                    14,
                    9,
                    height_px,
                    CLASSIC_ISO_DOODAD_WOOD_COLOR,
                );
            }
            true
        }
        "doodad_torch" => {
            classic_draw_iso_shadow(buffer, width, height, center_x, base_y, tile_w / 5, 3);
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 2,
                base_y - 30,
                4,
                28,
                CLASSIC_ISO_DOODAD_WOOD_COLOR,
            );
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                center_x,
                base_y - 34,
                8,
                7,
                CLASSIC_ISO_DOODAD_FIRE_COLOR,
            );
            true
        }
        "doodad_crystal_cluster" => {
            classic_draw_iso_shadow(buffer, width, height, center_x, base_y, tile_w / 4, 4);
            for (dx, height_px) in [(-8, 24), (3, 32), (13, 20)] {
                classic_draw_iso_prism(
                    buffer,
                    width,
                    height,
                    center_x + dx,
                    base_y - 4,
                    10,
                    8,
                    height_px,
                    CLASSIC_ISO_DOODAD_CRYSTAL_COLOR,
                );
            }
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                center_x + 2,
                base_y - 24,
                17,
                8,
                CLASSIC_ISO_DOODAD_CRYSTAL_COLOR,
            );
            true
        }
        "doodad_bush_cluster" => {
            classic_draw_iso_shadow(buffer, width, height, center_x, base_y, tile_w / 3, 4);
            for (dx, dy, w, h, color) in [
                (-16, -17, 20, 9, CLASSIC_ISO_CANOPY_COLOR),
                (-2, -22, 24, 10, CLASSIC_ISO_CANOPY_LIGHT_COLOR),
                (9, -14, 16, 7, CLASSIC_ISO_FOLIAGE_DARK_COLOR),
            ] {
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    center_x + dx,
                    base_y + dy,
                    w,
                    h,
                    color,
                );
            }
            true
        }
        "doodad_ruins_column" => {
            classic_draw_iso_shadow(buffer, width, height, center_x, base_y, tile_w / 4, 4);
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 8,
                base_y - 46,
                16,
                35,
                CLASSIC_ISO_RUIN_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 10,
                base_y - 53,
                20,
                5,
                CLASSIC_ISO_DOODAD_STONE_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 4,
                base_y - 45,
                8,
                26,
                CLASSIC_ISO_RUIN_COLOR,
            );
            true
        }
        "doodad_gold_vein" => {
            classic_draw_iso_shadow(buffer, width, height, center_x, base_y, tile_w / 3, 4);
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 15,
                base_y - 7,
                30,
                3,
                CLASSIC_ISO_SHADOW_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 16,
                base_y - 13,
                32,
                8,
                CLASSIC_ISO_DOODAD_STONE_COLOR,
            );
            for (dx, dy, w) in [(-12, -10, 9), (-2, -15, 13), (10, -8, 8)] {
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    center_x + dx,
                    base_y + dy,
                    w,
                    4,
                    CLASSIC_ISO_GOLD_VEIN_COLOR,
                );
            }
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 5,
                base_y - 15,
                10,
                2,
                0xffe88a,
            );
            true
        }
        "doodad_signpost" => {
            classic_draw_iso_shadow(buffer, width, height, center_x, base_y, tile_w / 5, 3);
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 2,
                base_y - 32,
                4,
                30,
                CLASSIC_ISO_DOODAD_WOOD_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 16,
                base_y - 30,
                32,
                9,
                CLASSIC_ISO_BRIDGE_PLANK_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 11,
                base_y - 27,
                22,
                2,
                CLASSIC_ISO_GOLD_COLOR,
            );
            true
        }
        "tile_cliff_edge" => {
            classic_draw_iso_diamond(
                buffer,
                width,
                height,
                center_x,
                top_y + 6,
                tile_w,
                tile_h,
                classic_darken(CLASSIC_ISO_WALL_COLOR, 1, 3),
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 18,
                top_y + 18,
                36,
                12,
                CLASSIC_ISO_CLIFF_FACE_COLOR,
            );
            true
        }
        "tile_bridge" => {
            classic_draw_iso_diamond(
                buffer,
                width,
                height,
                center_x,
                top_y + 6,
                tile_w,
                tile_h,
                CLASSIC_ISO_WATER_DETAIL_COLOR,
            );
            for dy in [0, 5, 10] {
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    center_x - 18,
                    top_y + 6 + dy,
                    36,
                    3,
                    CLASSIC_ISO_BRIDGE_PLANK_COLOR,
                );
            }
            true
        }
        "tile_forest_floor" => {
            classic_draw_iso_diamond(
                buffer,
                width,
                height,
                center_x,
                top_y + 6,
                tile_w,
                tile_h,
                CLASSIC_ISO_FOLIAGE_DARK_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 18,
                top_y + 8,
                16,
                5,
                CLASSIC_ISO_CANOPY_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x + 2,
                top_y + 6,
                18,
                5,
                CLASSIC_ISO_CANOPY_LIGHT_COLOR,
            );
            true
        }
        "tile_shadow_edge" => {
            classic_draw_iso_diamond(
                buffer,
                width,
                height,
                center_x,
                top_y + 6,
                tile_w,
                tile_h,
                CLASSIC_ISO_FOUNDATION_COLOR,
            );
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                center_x,
                top_y + 12,
                19,
                6,
                CLASSIC_ISO_SHADOW_COLOR,
            );
            true
        }
        "tile_tree" => {
            classic_draw_iso_shadow(buffer, width, height, center_x, base_y - 2, tile_w / 3, 5);
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 3,
                top_y - 24,
                6,
                32,
                classic_darken(CLASSIC_ISO_WALL_COLOR, 1, 4),
            );
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                center_x - 8,
                top_y - 32,
                18,
                11,
                CLASSIC_ISO_CANOPY_COLOR,
            );
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                center_x + 10,
                top_y - 34,
                17,
                12,
                classic_darken(CLASSIC_ISO_CANOPY_COLOR, 1, 5),
            );
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                center_x,
                top_y - 43,
                16,
                10,
                CLASSIC_ISO_CANOPY_LIGHT_COLOR,
            );
            true
        }
        "prop_market_stall" => {
            classic_draw_iso_prism(
                buffer,
                width,
                height,
                center_x,
                top_y + 8,
                tile_w - 8,
                tile_h - 4,
                18,
                0x48688a,
            );
            classic_draw_iso_prism(
                buffer,
                width,
                height,
                center_x,
                top_y - 6,
                tile_w,
                tile_h,
                8,
                CLASSIC_ISO_BANNER_COLOR,
            );
            true
        }
        "prop_arena_gate" => {
            classic_draw_iso_prism(
                buffer,
                width,
                height,
                center_x - tile_w / 4,
                top_y + 8,
                tile_w / 2,
                tile_h,
                38,
                CLASSIC_ISO_STONE_COLOR,
            );
            classic_draw_iso_prism(
                buffer,
                width,
                height,
                center_x + tile_w / 4,
                top_y + 8,
                tile_w / 2,
                tile_h,
                38,
                CLASSIC_ISO_STONE_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - tile_w / 3,
                top_y - 32,
                (tile_w * 2) / 3,
                9,
                CLASSIC_ISO_ROOF_COLOR,
            );
            true
        }
        "prop_reward" | "prop_workbench" | "prop_training_dummy" => {
            classic_draw_iso_prism(
                buffer,
                width,
                height,
                center_x,
                top_y + 10,
                tile_w / 2,
                tile_h / 2,
                12,
                if frame_id == "prop_reward" {
                    0xd1a73f
                } else {
                    CLASSIC_ISO_WALL_COLOR
                },
            );
            true
        }
        "prop_door" => {
            classic_draw_iso_prism(
                buffer,
                width,
                height,
                center_x,
                top_y + 10,
                tile_w / 2,
                tile_h / 2,
                28,
                CLASSIC_ISO_WALL_COLOR,
            );
            true
        }
        "prop_banner" => {
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 2,
                top_y - 34,
                4,
                44,
                classic_darken(CLASSIC_ISO_WALL_COLOR, 1, 4),
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x + 2,
                top_y - 32,
                18,
                14,
                CLASSIC_ISO_BANNER_COLOR,
            );
            true
        }
        "prop_signpost" => {
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 2,
                top_y - 20,
                4,
                28,
                classic_darken(CLASSIC_ISO_WALL_COLOR, 1, 4),
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 12,
                top_y - 22,
                24,
                7,
                CLASSIC_ISO_WALL_COLOR,
            );
            true
        }
        "actor_enemy_attack" | "actor_enemy_idle" | "actor_enemy_hit" => {
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                center_x,
                base_y - 3,
                17,
                6,
                CLASSIC_ISO_UNIT_RING_COLOR,
            );
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                center_x,
                base_y - 26,
                13,
                18,
                CLASSIC_ISO_UNIT_ENEMY_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x + 8,
                base_y - 42,
                22,
                4,
                CLASSIC_ISO_OUTLINE_COLOR,
            );
            if frame_id == "actor_enemy_attack" {
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    center_x + 10,
                    base_y - 38,
                    24,
                    3,
                    CLASSIC_ISO_UNIT_DAMAGE_COLOR,
                );
            }
            if frame_id == "actor_enemy_hit" {
                classic_draw_iso_ellipse(
                    buffer,
                    width,
                    height,
                    center_x - 8,
                    base_y - 44,
                    8,
                    5,
                    CLASSIC_ISO_UNIT_DAMAGE_COLOR,
                );
            }
            true
        }
        frame if frame.starts_with("actor_player") => {
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                center_x,
                base_y - 3,
                16,
                6,
                CLASSIC_ISO_UNIT_RING_COLOR,
            );
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                center_x,
                base_y - 25,
                12,
                17,
                CLASSIC_ISO_UNIT_PLAYER_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 18,
                base_y - 39,
                10,
                4,
                CLASSIC_ISO_GOLD_COLOR,
            );
            true
        }
        frame if frame.starts_with("actor_mentor") || frame.starts_with("actor_vendor") => {
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                center_x,
                base_y - 3,
                15,
                5,
                CLASSIC_ISO_UNIT_RING_COLOR,
            );
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                center_x,
                base_y - 25,
                11,
                16,
                CLASSIC_ISO_UNIT_MENTOR_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x + 12,
                base_y - 46,
                4,
                30,
                CLASSIC_ISO_GOLD_COLOR,
            );
            true
        }
        _ => false,
    }
}

#[cfg(not(target_os = "android"))]
#[allow(clippy::too_many_arguments)]
pub(super) fn classic_draw_iso_unit_overlay(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    frame_id: &str,
    center_x: i32,
    sprite_top_y: i32,
) -> bool {
    let (bar_color, fill_width) = if frame_id.starts_with("actor_enemy") {
        (
            if frame_id == "actor_enemy_hit" {
                CLASSIC_ISO_UNIT_DAMAGE_COLOR
            } else {
                CLASSIC_ISO_UNIT_ENEMY_COLOR
            },
            if frame_id == "actor_enemy_hit" {
                12
            } else {
                20
            },
        )
    } else if frame_id.starts_with("actor_player") {
        (CLASSIC_ISO_UNIT_HEALTH_COLOR, 22)
    } else if frame_id.starts_with("actor_mentor") || frame_id.starts_with("actor_vendor") {
        (CLASSIC_ISO_UNIT_MENTOR_COLOR, 18)
    } else if frame_id.starts_with("actor_guard") {
        (CLASSIC_ISO_UNIT_GUARD_COLOR, 20)
    } else if frame_id.starts_with("actor_worker") {
        (CLASSIC_ISO_UNIT_WORKER_COLOR, 18)
    } else if frame_id.starts_with("actor_creep") {
        (CLASSIC_ISO_UNIT_CREEP_COLOR, 16)
    } else {
        return false;
    };
    classic_draw_rect(
        buffer,
        width,
        height,
        center_x - 14,
        sprite_top_y - 7,
        28,
        5,
        CLASSIC_ISO_OUTLINE_COLOR,
    );
    classic_draw_rect(
        buffer,
        width,
        height,
        center_x - 12,
        sprite_top_y - 6,
        fill_width,
        3,
        bar_color,
    );
    if frame_id.contains("attack") {
        classic_draw_rect(
            buffer,
            width,
            height,
            center_x + 11,
            sprite_top_y + 16,
            16,
            4,
            CLASSIC_RTS_FIDELITY_ACTION_TRAIL_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            center_x + 20,
            sprite_top_y + 12,
            5,
            9,
            CLASSIC_RTS_FIDELITY_NPC_ACTION_COLOR,
        );
    } else if frame_id.contains("carry") {
        classic_draw_rect(
            buffer,
            width,
            height,
            center_x + 10,
            sprite_top_y + 15,
            13,
            10,
            CLASSIC_ISO_GOLD_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            center_x + 12,
            sprite_top_y + 17,
            9,
            2,
            CLASSIC_RTS_FIDELITY_MODEL_HIGHLIGHT_COLOR,
        );
    } else if frame_id.starts_with("actor_guard")
        || frame_id.starts_with("actor_worker")
        || frame_id.starts_with("actor_creep")
    {
        classic_draw_rect(
            buffer,
            width,
            height,
            center_x - 13,
            sprite_top_y + 19,
            8,
            3,
            CLASSIC_RTS_FIDELITY_ANIMATION_GHOST_COLOR,
        );
    }
    classic_draw_rts_unit_model_depth_marks(
        buffer,
        width,
        height,
        frame_id,
        center_x,
        sprite_top_y + 46,
    );
    classic_draw_rts_action_cadence_marks(
        buffer,
        width,
        height,
        frame_id,
        center_x,
        sprite_top_y + 46,
    );
    true
}

pub(super) fn classic_parse_rts_tile(value: &str) -> Option<(i32, i32)> {
    let (x, y) = value.split_once(',')?;
    Some((x.trim().parse().ok()?, y.trim().parse().ok()?))
}

#[cfg(not(target_os = "android"))]
pub(super) fn classic_rts_control_group_entities(
    scene_id: &str,
    player_tile: (i32, i32),
    runtime: &NativeFirstPlayableRuntime,
) -> Vec<ClassicIsoEntity> {
    if runtime.rts_selected_unit_ids.is_empty() {
        return Vec::new();
    }
    let selected_ids = runtime
        .rts_selected_unit_ids
        .iter()
        .map(String::as_str)
        .collect::<HashSet<_>>();
    let mut candidates = classic_scene_rts_neutral_unit_entities(scene_id);
    candidates.push(ClassicIsoEntity {
        id: "player".to_string(),
        frame_id: "actor_player_idle_south".to_string(),
        tile: player_tile,
        depth_key: (player_tile.0 + player_tile.1) * 10 + 5,
    });
    candidates
        .into_iter()
        .filter(|entity| selected_ids.contains(entity.id.as_str()))
        .collect()
}

#[cfg(not(target_os = "android"))]
#[allow(clippy::too_many_arguments)]
pub(super) fn classic_draw_iso_rts_selection_marker(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    screen_x: i32,
    screen_y: i32,
    _tile_w: i32,
    tile_h: i32,
) {
    classic_draw_iso_ellipse(
        buffer,
        width,
        height,
        screen_x,
        screen_y + tile_h - 1,
        18,
        7,
        CLASSIC_ISO_CONTROL_GROUP_COLOR,
    );
    classic_draw_iso_ellipse(
        buffer,
        width,
        height,
        screen_x,
        screen_y + tile_h - 1,
        12,
        4,
        CLASSIC_ISO_FOUNDATION_COLOR,
    );
    for (dx, dy) in [(-18, -3), (13, -3), (-18, 5), (13, 5)] {
        classic_draw_rect(
            buffer,
            width,
            height,
            screen_x + dx,
            screen_y + tile_h + dy,
            5,
            2,
            CLASSIC_ISO_CONTROL_GROUP_COLOR,
        );
    }
}

#[cfg(not(target_os = "android"))]
#[allow(clippy::too_many_arguments)]
pub(super) fn classic_draw_iso_rts_formation_line(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    start: (i32, i32),
    end: (i32, i32),
) {
    for step in 0..=12 {
        let x = start.0 + ((end.0 - start.0) * step) / 12;
        let y = start.1 + ((end.1 - start.1) * step) / 12;
        classic_draw_rect(
            buffer,
            width,
            height,
            x - 1,
            y - 1,
            3,
            3,
            CLASSIC_ISO_FORMATION_LINE_COLOR,
        );
    }
}

#[cfg(not(target_os = "android"))]
pub(super) fn classic_draw_rts_command_affordance_drag_marquee(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    screen_points: &[(i32, i32)],
) {
    if screen_points.len() < 2 {
        return;
    }
    let min_x = screen_points.iter().map(|point| point.0).min().unwrap_or(0) - 28;
    let max_x = screen_points.iter().map(|point| point.0).max().unwrap_or(0) + 28;
    let min_y = screen_points.iter().map(|point| point.1).min().unwrap_or(0) - 22;
    let max_y = screen_points.iter().map(|point| point.1).max().unwrap_or(0) + 18;
    let dash: usize = 12;
    for x in (min_x..=max_x).step_by(dash) {
        classic_draw_rect(
            buffer,
            width,
            height,
            x,
            min_y,
            ((dash / 2) as i32).min(max_x - x + 1),
            3,
            CLASSIC_RTS_COMMAND_AFFORDANCE_DRAG_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            x,
            max_y,
            ((dash / 2) as i32).min(max_x - x + 1),
            3,
            CLASSIC_RTS_COMMAND_AFFORDANCE_DRAG_COLOR,
        );
    }
    for y in (min_y..=max_y).step_by(dash) {
        classic_draw_rect(
            buffer,
            width,
            height,
            min_x,
            y,
            3,
            ((dash / 2) as i32).min(max_y - y + 1),
            CLASSIC_RTS_COMMAND_AFFORDANCE_DRAG_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            max_x,
            y,
            3,
            ((dash / 2) as i32).min(max_y - y + 1),
            CLASSIC_RTS_COMMAND_AFFORDANCE_DRAG_COLOR,
        );
    }
}

#[cfg(not(target_os = "android"))]
pub(super) fn classic_draw_rts_command_affordance_cursor_arrow(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    x: i32,
    y: i32,
) {
    for step in 0..10 {
        classic_draw_rect(
            buffer,
            width,
            height,
            x + step,
            y + step,
            3,
            3,
            CLASSIC_RTS_COMMAND_AFFORDANCE_CURSOR_ARROW_COLOR,
        );
    }
    classic_draw_rect(
        buffer,
        width,
        height,
        x + 8,
        y + 16,
        8,
        3,
        CLASSIC_RTS_COMMAND_AFFORDANCE_CURSOR_ARROW_COLOR,
    );
    classic_draw_rect(
        buffer,
        width,
        height,
        x + 16,
        y + 8,
        3,
        8,
        CLASSIC_RTS_COMMAND_AFFORDANCE_CURSOR_ARROW_COLOR,
    );
}

#[cfg(not(target_os = "android"))]
pub(super) fn classic_draw_rts_command_affordance_target_marker(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    x: i32,
    y: i32,
    color: u32,
) {
    classic_draw_iso_ellipse(buffer, width, height, x, y, 25, 10, color);
    classic_draw_rect(buffer, width, height, x - 29, y - 1, 18, 3, color);
    classic_draw_rect(buffer, width, height, x + 11, y - 1, 18, 3, color);
    classic_draw_rect(buffer, width, height, x - 2, y - 22, 4, 17, color);
    classic_draw_rect(buffer, width, height, x - 2, y + 8, 4, 15, color);
}

#[cfg(not(target_os = "android"))]
#[allow(clippy::too_many_arguments)]
pub(super) fn classic_draw_iso_command_feedback(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    assets: &ClassicRuntimeAssets,
    runtime: &NativeFirstPlayableRuntime,
    scene_id: &str,
    origin_x: i32,
    origin_y: i32,
    tile_w: i32,
    tile_h: i32,
    player_tile: (i32, i32),
) -> bool {
    let default_destination_tile =
        if runtime.combat_overlay_visible || scene_id == "league_coliseum" {
            (9, 2)
        } else if runtime.dialogue_overlay_visible || scene_id == "mirror_city_square" {
            (4, 3)
        } else {
            player_tile
        };
    let destination_tile = runtime
        .rts_command_destination_tile
        .as_deref()
        .and_then(classic_parse_rts_tile)
        .unwrap_or(default_destination_tile);
    let (dest_x, dest_y) =
        classic_iso_project(origin_x, origin_y, tile_w, tile_h, destination_tile);
    for tile_id in &runtime.rts_path_tile_ids {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (tile_x, tile_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            classic_draw_iso_diamond(
                buffer,
                width,
                height,
                tile_x,
                tile_y + tile_h - 8,
                tile_w / 2,
                tile_h / 2,
                CLASSIC_RTS_PATH_TILE_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                tile_x - 14,
                tile_y + tile_h - 7,
                28,
                4,
                CLASSIC_RTS_PATH_TILE_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                tile_x - 2,
                tile_y + tile_h - 17,
                4,
                18,
                CLASSIC_RTS_PATH_TILE_COLOR,
            );
        }
    }
    for tile_id in &runtime.rts_blocked_tile_ids {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (tile_x, tile_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            classic_draw_rect(
                buffer,
                width,
                height,
                tile_x - 14,
                tile_y + tile_h - 8,
                28,
                4,
                CLASSIC_RTS_BLOCKED_TILE_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                tile_x - 2,
                tile_y + tile_h - 18,
                4,
                24,
                CLASSIC_RTS_BLOCKED_TILE_COLOR,
            );
        }
    }
    for tile_id in &runtime.rts_formation_slot_tile_ids {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (slot_x, slot_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                slot_x,
                slot_y + tile_h + 3,
                10,
                4,
                CLASSIC_RTS_FORMATION_SLOT_COLOR,
            );
        }
    }
    for tile_id in &runtime.rts_disperse_tile_ids {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (slot_x, slot_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            classic_draw_rect(
                buffer,
                width,
                height,
                slot_x - 11,
                slot_y + tile_h + 6,
                22,
                4,
                CLASSIC_RTS_DISPERSION_SLOT_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                slot_x - 2,
                slot_y + tile_h - 2,
                4,
                12,
                CLASSIC_RTS_DISPERSION_SLOT_COLOR,
            );
        }
    }
    for tile_id in &runtime.rts_engagement_tile_ids {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (range_x, range_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                range_x,
                range_y + tile_h + 2,
                15,
                6,
                CLASSIC_RTS_ENGAGEMENT_RANGE_COLOR,
            );
        }
    }
    for tile_id in &runtime.rts_contact_flash_tile_ids {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (flash_x, flash_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            classic_draw_rect(
                buffer,
                width,
                height,
                flash_x - 10,
                flash_y + tile_h - 12,
                20,
                4,
                CLASSIC_RTS_CONTACT_FLASH_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                flash_x - 2,
                flash_y + tile_h - 22,
                4,
                24,
                CLASSIC_RTS_CONTACT_FLASH_COLOR,
            );
        }
    }
    let mut selection_box_screen_points = Vec::new();
    for tile_id in &runtime.rts_selection_box_tile_ids {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (box_x, box_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            selection_box_screen_points.push((box_x, box_y + tile_h));
            classic_draw_rect(
                buffer,
                width,
                height,
                box_x - 23,
                box_y + tile_h - 9,
                46,
                3,
                CLASSIC_RTS_SELECTION_BOX_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                box_x - 23,
                box_y + tile_h + 8,
                46,
                3,
                CLASSIC_RTS_SELECTION_BOX_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                box_x - 24,
                box_y + tile_h - 8,
                3,
                18,
                CLASSIC_RTS_SELECTION_BOX_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                box_x + 21,
                box_y + tile_h - 8,
                3,
                18,
                CLASSIC_RTS_SELECTION_BOX_COLOR,
            );
        }
    }
    if width >= 640 && height >= 320 {
        classic_draw_rts_command_affordance_drag_marquee(
            buffer,
            width,
            height,
            &selection_box_screen_points,
        );
        classic_draw_rts_command_affordance_cursor_arrow(
            buffer,
            width,
            height,
            dest_x + 22,
            dest_y + tile_h - 38,
        );
        classic_draw_rts_command_affordance_target_marker(
            buffer,
            width,
            height,
            dest_x,
            dest_y + tile_h - 2,
            CLASSIC_RTS_COMMAND_AFFORDANCE_RIGHT_CLICK_COLOR,
        );
        if let Some(target_id) = runtime.rts_attack_target_id.as_deref() {
            let target_tile = classic_rts_target_tile_for_id(target_id, 0);
            let (target_x, target_y) =
                classic_iso_project(origin_x, origin_y, tile_w, tile_h, target_tile);
            classic_draw_rts_command_affordance_target_marker(
                buffer,
                width,
                height,
                target_x,
                target_y + tile_h - 2,
                CLASSIC_RTS_COMMAND_AFFORDANCE_ATTACK_CURSOR_COLOR,
            );
            classic_draw_rts_command_affordance_cursor_arrow(
                buffer,
                width,
                height,
                target_x + 24,
                target_y + tile_h - 44,
            );
        }
    }
    if !runtime.rts_group_route_tile_ids.is_empty() {
        let mut previous_screen: Option<(i32, i32)> = None;
        for tile_id in &runtime.rts_group_route_tile_ids {
            if let Some(tile) = classic_parse_rts_tile(tile_id) {
                let (route_x, route_y) =
                    classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
                classic_draw_iso_ellipse(
                    buffer,
                    width,
                    height,
                    route_x,
                    route_y + tile_h + 2,
                    12,
                    5,
                    CLASSIC_RTS_SPLIT_ROUTE_COLOR,
                );
                if let Some((prev_x, prev_y)) = previous_screen {
                    for step in 0..=6 {
                        let line_x = prev_x + ((route_x - prev_x) * step) / 6;
                        let line_y = prev_y + tile_h + ((route_y - prev_y) * step) / 6;
                        classic_draw_rect(
                            buffer,
                            width,
                            height,
                            line_x - 2,
                            line_y - 1,
                            5,
                            3,
                            CLASSIC_RTS_SPLIT_ROUTE_COLOR,
                        );
                    }
                }
                previous_screen = Some((route_x, route_y));
            }
        }
    }
    if let Some(tile) = runtime
        .rts_minimap_command_tile_id
        .as_deref()
        .and_then(classic_parse_rts_tile)
    {
        let (mini_x, mini_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            mini_x - 17,
            mini_y + tile_h - 24,
            34,
            5,
            CLASSIC_RTS_MINIMAP_COMMAND_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            mini_x - 3,
            mini_y + tile_h - 38,
            6,
            33,
            CLASSIC_RTS_MINIMAP_COMMAND_COLOR,
        );
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            mini_x,
            mini_y + tile_h - 6,
            21,
            9,
            CLASSIC_RTS_MINIMAP_COMMAND_COLOR,
        );
    }
    let selected_units = classic_rts_control_group_entities(scene_id, player_tile, runtime);
    for entity in &selected_units {
        let (unit_x, unit_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, entity.tile);
        classic_draw_iso_rts_selection_marker(
            buffer, width, height, unit_x, unit_y, tile_w, tile_h,
        );
        if runtime
            .rts_active_control_group_ids
            .iter()
            .any(|group_id| group_id == "2")
            && runtime
                .rts_control_group_assignments
                .iter()
                .any(|assignment| assignment.starts_with("2:") && assignment.contains(&entity.id))
        {
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                unit_x,
                unit_y + tile_h + 7,
                12,
                5,
                CLASSIC_RTS_GROUP_TWO_COLOR,
            );
        }
        classic_draw_iso_rts_formation_line(
            buffer,
            width,
            height,
            (unit_x, unit_y + tile_h - 1),
            (dest_x, dest_y + tile_h - 1),
        );
    }
    for (index, target_id) in runtime.rts_target_priority_ids.iter().enumerate() {
        let tile = classic_rts_target_tile_for_id(target_id, index);
        let (target_x, target_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            target_x - 13,
            target_y + tile_h - 27 - index as i32 * 3,
            26,
            4,
            CLASSIC_RTS_TARGET_PRIORITY_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            target_x + 14,
            target_y + tile_h - 27 - index as i32 * 3,
            5 + index as i32 * 3,
            4,
            CLASSIC_RTS_TARGET_PRIORITY_COLOR,
        );
    }
    if let Some(aggro_target_id) = runtime.rts_aggro_target_id.as_deref() {
        let tile = classic_rts_target_tile_for_id(aggro_target_id, 0);
        let (aggro_x, aggro_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            aggro_x,
            aggro_y + tile_h - 4,
            26,
            11,
            CLASSIC_RTS_AGGRO_RING_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            aggro_x - 21,
            aggro_y + tile_h - 21,
            42,
            3,
            CLASSIC_RTS_AGGRO_RING_COLOR,
        );
    }
    if !runtime.rts_focus_fire_unit_ids.is_empty() {
        let target_tile = runtime
            .rts_aggro_target_id
            .as_deref()
            .map(|target_id| classic_rts_target_tile_for_id(target_id, 0))
            .unwrap_or(destination_tile);
        let (focus_x, focus_y) =
            classic_iso_project(origin_x, origin_y, tile_w, tile_h, target_tile);
        for (index, entity) in selected_units.iter().enumerate() {
            if runtime
                .rts_focus_fire_unit_ids
                .iter()
                .any(|unit_id| unit_id == &entity.id)
            {
                let (unit_x, unit_y) =
                    classic_iso_project(origin_x, origin_y, tile_w, tile_h, entity.tile);
                for step in 0..=8 {
                    let beam_x = unit_x + ((focus_x - unit_x) * step) / 8;
                    let beam_y = unit_y + tile_h - 14 + ((focus_y - unit_y) * step) / 8;
                    classic_draw_rect(
                        buffer,
                        width,
                        height,
                        beam_x - 1,
                        beam_y - 1 + index as i32 % 2,
                        5,
                        3,
                        CLASSIC_RTS_FOCUS_FIRE_COLOR,
                    );
                }
            }
        }
    }
    if !runtime.rts_projectile_trail_tile_ids.is_empty() {
        let mut previous_screen: Option<(i32, i32)> = None;
        for tile_id in &runtime.rts_projectile_trail_tile_ids {
            if let Some(tile) = classic_parse_rts_tile(tile_id) {
                let (trail_x, trail_y) =
                    classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    trail_x - 5,
                    trail_y + tile_h - 31,
                    10,
                    4,
                    CLASSIC_RTS_PROJECTILE_TRAIL_COLOR,
                );
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    trail_x - 2,
                    trail_y + tile_h - 37,
                    4,
                    12,
                    CLASSIC_RTS_PROJECTILE_TRAIL_COLOR,
                );
                if let Some((prev_x, prev_y)) = previous_screen {
                    for step in 0..=7 {
                        let line_x = prev_x + ((trail_x - prev_x) * step) / 7;
                        let line_y = prev_y + tile_h - 31 + ((trail_y - prev_y) * step) / 7;
                        classic_draw_rect(
                            buffer,
                            width,
                            height,
                            line_x - 2,
                            line_y - 1,
                            5,
                            3,
                            CLASSIC_RTS_PROJECTILE_TRAIL_COLOR,
                        );
                    }
                }
                previous_screen = Some((trail_x, trail_y));
            }
        }
    }
    for tile_id in &runtime.rts_ability_effect_tile_ids {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (effect_x, effect_y) =
                classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                effect_x,
                effect_y + tile_h - 7,
                24,
                10,
                CLASSIC_RTS_ABILITY_RADIUS_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                effect_x - 17,
                effect_y + tile_h - 42,
                34,
                4,
                CLASSIC_RTS_ABILITY_RADIUS_COLOR,
            );
        }
    }
    if let Some(impact_tile) = runtime
        .rts_projectile_impact_tile_id
        .as_deref()
        .and_then(classic_parse_rts_tile)
    {
        let (impact_x, impact_y) =
            classic_iso_project(origin_x, origin_y, tile_w, tile_h, impact_tile);
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            impact_x,
            impact_y + tile_h - 12,
            18,
            10,
            CLASSIC_RTS_PROJECTILE_IMPACT_COLOR,
        );
        for step in -12..=12 {
            classic_draw_rect(
                buffer,
                width,
                height,
                impact_x + step,
                impact_y + tile_h - 30 + step / 2,
                4,
                4,
                CLASSIC_RTS_PROJECTILE_IMPACT_COLOR,
            );
        }
        for (index, tick) in runtime.rts_ability_damage_ticks.iter().take(4).enumerate() {
            classic_draw_rect(
                buffer,
                width,
                height,
                impact_x + 24,
                impact_y + tile_h - 42 + index as i32 * 7,
                ((*tick).min(40) as i32 * 2).max(8),
                4,
                CLASSIC_RTS_DAMAGE_TICK_COLOR,
            );
        }
        if runtime.rts_target_armor_percent > 0 || runtime.rts_target_shield_percent > 0 {
            classic_draw_rect(
                buffer,
                width,
                height,
                impact_x - 24,
                impact_y + tile_h - 55,
                (runtime.rts_target_armor_percent.min(100) as i32 * 42) / 100,
                4,
                CLASSIC_RTS_ARMOR_SHIELD_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                impact_x - 24,
                impact_y + tile_h - 61,
                (runtime.rts_target_shield_percent.min(100) as i32 * 42) / 100,
                4,
                CLASSIC_RTS_ARMOR_SHIELD_COLOR,
            );
        }
    }
    for tile_id in &runtime.rts_contact_flash_tile_ids {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (flash_x, flash_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            classic_draw_rect(
                buffer,
                width,
                height,
                flash_x - 12,
                flash_y + tile_h - 18,
                24,
                3,
                CLASSIC_RTS_CONTACT_FLASH_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                flash_x - 3,
                flash_y + tile_h - 26,
                6,
                12,
                CLASSIC_RTS_CONTACT_FLASH_COLOR,
            );
        }
    }
    for (index, tile_id) in runtime.rts_ai_pressure_tile_ids.iter().enumerate() {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (pressure_x, pressure_y) =
                classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                pressure_x,
                pressure_y + tile_h - 8,
                20,
                8,
                CLASSIC_RTS_AI_PRESSURE_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                pressure_x - 14,
                pressure_y + tile_h - 34,
                28,
                4,
                CLASSIC_RTS_AI_PRESSURE_COLOR,
            );
            if runtime.rts_ai_wave_unit_ids.get(index).is_some() {
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    pressure_x - 9,
                    pressure_y + tile_h - 27,
                    18,
                    13,
                    CLASSIC_RTS_AI_WAVE_COLOR,
                );
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    pressure_x - 5,
                    pressure_y + tile_h - 38,
                    10,
                    10,
                    CLASSIC_RTS_AI_WAVE_COLOR,
                );
            }
        }
    }
    for tile_id in &runtime.rts_ai_counter_tile_ids {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (counter_x, counter_y) =
                classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            classic_draw_rect(
                buffer,
                width,
                height,
                counter_x - 18,
                counter_y + tile_h - 12,
                36,
                4,
                CLASSIC_RTS_AI_COUNTER_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                counter_x + 10,
                counter_y + tile_h - 22,
                7,
                16,
                CLASSIC_RTS_AI_COUNTER_COLOR,
            );
        }
    }
    if let Some(retreat_tile) = runtime
        .rts_ai_retreat_tile_id
        .as_deref()
        .and_then(classic_parse_rts_tile)
    {
        let (retreat_x, retreat_y) =
            classic_iso_project(origin_x, origin_y, tile_w, tile_h, retreat_tile);
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            retreat_x,
            retreat_y + tile_h - 5,
            17,
            7,
            CLASSIC_RTS_AI_RETREAT_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            retreat_x - 15,
            retreat_y + tile_h - 29,
            30,
            5,
            CLASSIC_RTS_AI_RETREAT_COLOR,
        );
    }
    for tile_id in &runtime.rts_objective_tile_ids {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (objective_x, objective_y) =
                classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                objective_x,
                objective_y + tile_h - 9,
                24,
                9,
                CLASSIC_RTS_OBJECTIVE_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                objective_x - 12,
                objective_y + tile_h - 42,
                24,
                22,
                CLASSIC_RTS_OBJECTIVE_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                objective_x - 16,
                objective_y + tile_h - 47,
                (runtime.rts_objective_capture_percent.min(100) as i32 * 32) / 100,
                4,
                CLASSIC_RTS_CAPTURE_BAR_COLOR,
            );
        }
    }
    if runtime.rts_objective_result_state.starts_with("victory:") {
        for tile_id in &runtime.rts_objective_tile_ids {
            if let Some(tile) = classic_parse_rts_tile(tile_id) {
                let (victory_x, victory_y) =
                    classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
                classic_draw_iso_ellipse(
                    buffer,
                    width,
                    height,
                    victory_x,
                    victory_y + tile_h - 18,
                    30,
                    11,
                    CLASSIC_RTS_VICTORY_COLOR,
                );
            }
        }
    }
    if let Some(extraction_tile) = runtime
        .rts_objective_extraction_tile_id
        .as_deref()
        .and_then(classic_parse_rts_tile)
    {
        let (extract_x, extract_y) =
            classic_iso_project(origin_x, origin_y, tile_w, tile_h, extraction_tile);
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            extract_x,
            extract_y + tile_h - 7,
            19,
            7,
            CLASSIC_RTS_EXTRACTION_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            extract_x - 10,
            extract_y + tile_h - 36,
            20,
            18,
            CLASSIC_RTS_EXTRACTION_COLOR,
        );
    }
    for tile_id in &runtime.rts_terrain_route_tile_ids {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (route_x, route_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                route_x,
                route_y + tile_h - 6,
                18,
                6,
                CLASSIC_RTS_TERRAIN_ROUTE_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                route_x - 14,
                route_y + tile_h - 28,
                28,
                4,
                CLASSIC_RTS_TERRAIN_ROUTE_COLOR,
            );
        }
    }
    for tile_id in &runtime.rts_terrain_choke_tile_ids {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (choke_x, choke_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            classic_draw_rect(
                buffer,
                width,
                height,
                choke_x - 18,
                choke_y + tile_h - 10,
                36,
                4,
                CLASSIC_RTS_CHOKE_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                choke_x - 3,
                choke_y + tile_h - 32,
                6,
                25,
                CLASSIC_RTS_CHOKE_COLOR,
            );
        }
    }
    for (index, tile_id) in runtime.rts_creep_camp_tile_ids.iter().enumerate() {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (camp_x, camp_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                camp_x,
                camp_y + tile_h - 8,
                22,
                8,
                CLASSIC_RTS_CREEP_CAMP_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                camp_x - 12,
                camp_y + tile_h - 35,
                24,
                16,
                CLASSIC_RTS_CREEP_CAMP_COLOR,
            );
            if runtime.rts_creep_camp_unit_ids.get(index).is_some() {
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    camp_x - 8,
                    camp_y + tile_h - 28,
                    16,
                    12,
                    CLASSIC_ISO_UNIT_CREEP_COLOR,
                );
            }
        }
    }
    for tile_id in &runtime.rts_expansion_tile_ids {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (expand_x, expand_y) =
                classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                expand_x,
                expand_y + tile_h - 9,
                25,
                8,
                CLASSIC_RTS_EXPANSION_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                expand_x - 11,
                expand_y + tile_h - 39,
                22,
                18,
                CLASSIC_RTS_EXPANSION_COLOR,
            );
        }
    }
    if !runtime.rts_scout_route_tile_ids.is_empty() {
        let mut previous_screen: Option<(i32, i32)> = None;
        for tile_id in &runtime.rts_scout_route_tile_ids {
            if let Some(tile) = classic_parse_rts_tile(tile_id) {
                let (route_x, route_y) =
                    classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
                classic_draw_iso_ellipse(
                    buffer,
                    width,
                    height,
                    route_x,
                    route_y + tile_h - 3,
                    13,
                    5,
                    CLASSIC_RTS_SCOUT_ROUTE_COLOR,
                );
                if let Some((prev_x, prev_y)) = previous_screen {
                    for step in 0..=7 {
                        let line_x = prev_x + ((route_x - prev_x) * step) / 7;
                        let line_y = prev_y + tile_h - 3 + ((route_y - prev_y) * step) / 7;
                        classic_draw_rect(
                            buffer,
                            width,
                            height,
                            line_x - 2,
                            line_y - 1,
                            5,
                            3,
                            CLASSIC_RTS_SCOUT_ROUTE_COLOR,
                        );
                    }
                }
                previous_screen = Some((route_x, route_y));
            }
        }
    }
    for tile_id in &runtime.rts_fog_reveal_tile_ids {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (reveal_x, reveal_y) =
                classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            classic_draw_iso_diamond(
                buffer,
                width,
                height,
                reveal_x,
                reveal_y + tile_h - 13,
                tile_w / 3,
                tile_h / 3,
                CLASSIC_RTS_FOG_REVEAL_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                reveal_x - 8,
                reveal_y + tile_h - 32,
                16,
                4,
                CLASSIC_RTS_FOG_REVEAL_COLOR,
            );
        }
    }
    for (index, structure_id) in runtime.rts_revealed_enemy_structure_ids.iter().enumerate() {
        let tile = classic_rts_enemy_structure_tile_for_id(structure_id, index);
        let (structure_x, structure_y) =
            classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            structure_x - 15,
            structure_y + tile_h - 42,
            30,
            24,
            CLASSIC_RTS_ENEMY_STRUCTURE_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            structure_x - 20,
            structure_y + tile_h - 47,
            40,
            4,
            CLASSIC_RTS_ENEMY_STRUCTURE_COLOR,
        );
    }
    for (index, unit_id) in runtime.rts_revealed_enemy_unit_ids.iter().enumerate() {
        let tile = classic_rts_enemy_unit_tile_for_id(unit_id, index);
        let (enemy_x, enemy_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            enemy_x,
            enemy_y + tile_h - 7,
            18,
            7,
            CLASSIC_RTS_ENEMY_INTEL_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            enemy_x - 8,
            enemy_y + tile_h - 30,
            16,
            14,
            CLASSIC_RTS_ENEMY_INTEL_COLOR,
        );
    }
    for (index, tech_id) in runtime.rts_enemy_base_tech_ids.iter().enumerate() {
        let tile = classic_rts_enemy_structure_tile_for_id(
            if tech_id.contains("forge") {
                "enemy_resource_vault"
            } else {
                "enemy_barracks"
            },
            index,
        );
        let (tech_x, tech_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            tech_x,
            tech_y + tile_h - 18 - index as i32 * 4,
            24,
            9,
            CLASSIC_RTS_ENEMY_TECH_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            tech_x - 11,
            tech_y + tile_h - 56 - index as i32 * 3,
            22,
            5,
            CLASSIC_RTS_ENEMY_TECH_COLOR,
        );
    }
    for (index, unit_id) in runtime.rts_enemy_pressure_wave_unit_ids.iter().enumerate() {
        let tile = classic_rts_enemy_unit_tile_for_id(unit_id, index);
        let (wave_x, wave_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            wave_x - 10,
            wave_y + tile_h - 36,
            20,
            18,
            CLASSIC_RTS_ENEMY_PRODUCTION_COLOR,
        );
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            wave_x,
            wave_y + tile_h - 8,
            18,
            6,
            CLASSIC_RTS_ENEMY_PRODUCTION_COLOR,
        );
    }
    for tile_id in &runtime.rts_ai_pressure_tile_ids {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (lane_x, lane_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            classic_draw_rect(
                buffer,
                width,
                height,
                lane_x - 13,
                lane_y + tile_h - 5,
                26,
                3,
                CLASSIC_RTS_PRESSURE_WARNING_COLOR,
            );
        }
    }
    for (index, tech_id) in runtime.rts_player_counter_tech_ids.iter().enumerate() {
        let tile = classic_rts_structure_tile_for_id(if tech_id.contains("lantern") {
            "signal_spire"
        } else {
            "training_hall"
        });
        let (counter_x, counter_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            counter_x,
            counter_y + tile_h - 18 - index as i32 * 4,
            20,
            8,
            CLASSIC_RTS_PLAYER_COUNTER_TECH_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            counter_x - 8,
            counter_y + tile_h - 50 - index as i32 * 3,
            16,
            5,
            CLASSIC_RTS_PLAYER_COUNTER_TECH_COLOR,
        );
    }
    for structure_id in &runtime.rts_player_defense_structure_ids {
        let tile = classic_rts_structure_tile_for_id(structure_id);
        let (defense_x, defense_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            defense_x - 14,
            defense_y + tile_h - 44,
            28,
            6,
            CLASSIC_RTS_DEFENSE_READY_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            defense_x - 4,
            defense_y + tile_h - 56,
            8,
            16,
            CLASSIC_RTS_DEFENSE_READY_COLOR,
        );
    }
    if !runtime.rts_army_rally_tile_ids.is_empty() {
        let mut previous_screen: Option<(i32, i32)> = None;
        for tile_id in &runtime.rts_army_rally_tile_ids {
            if let Some(tile) = classic_parse_rts_tile(tile_id) {
                let (rally_x, rally_y) =
                    classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
                classic_draw_iso_ellipse(
                    buffer,
                    width,
                    height,
                    rally_x,
                    rally_y + tile_h - 4,
                    16,
                    5,
                    CLASSIC_RTS_RALLY_LINE_COLOR,
                );
                if let Some((prev_x, prev_y)) = previous_screen {
                    for step in 0..=7 {
                        let line_x = prev_x + ((rally_x - prev_x) * step) / 7;
                        let line_y = prev_y + tile_h - 4 + ((rally_y - prev_y) * step) / 7;
                        classic_draw_rect(
                            buffer,
                            width,
                            height,
                            line_x - 2,
                            line_y - 1,
                            5,
                            3,
                            CLASSIC_RTS_RALLY_LINE_COLOR,
                        );
                    }
                }
                previous_screen = Some((rally_x, rally_y));
            }
        }
    }
    for (index, unit_id) in runtime.rts_army_spawned_unit_ids.iter().enumerate() {
        let tile = classic_rts_player_army_unit_tile_for_id(unit_id, index);
        let (unit_x, unit_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            unit_x,
            unit_y + tile_h - 7,
            18,
            6,
            CLASSIC_RTS_ARMY_SPAWN_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            unit_x - 8,
            unit_y + tile_h - 34,
            16,
            15,
            CLASSIC_RTS_ARMY_SPAWN_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            unit_x - 12,
            unit_y + tile_h - 40,
            24,
            3,
            CLASSIC_RTS_COMPOSITION_COLOR,
        );
    }
    if !runtime.rts_base_assault_path_tile_ids.is_empty() {
        let mut previous_screen: Option<(i32, i32)> = None;
        for tile_id in &runtime.rts_base_assault_path_tile_ids {
            if let Some(tile) = classic_parse_rts_tile(tile_id) {
                let (path_x, path_y) =
                    classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
                classic_draw_iso_ellipse(
                    buffer,
                    width,
                    height,
                    path_x,
                    path_y + tile_h - 3,
                    15,
                    5,
                    CLASSIC_RTS_BASE_ASSAULT_PATH_COLOR,
                );
                if let Some((prev_x, prev_y)) = previous_screen {
                    for step in 0..=7 {
                        let line_x = prev_x + ((path_x - prev_x) * step) / 7;
                        let line_y = prev_y + tile_h - 3 + ((path_y - prev_y) * step) / 7;
                        classic_draw_rect(
                            buffer,
                            width,
                            height,
                            line_x - 2,
                            line_y - 1,
                            5,
                            3,
                            CLASSIC_RTS_BASE_ASSAULT_PATH_COLOR,
                        );
                    }
                }
                previous_screen = Some((path_x, path_y));
            }
        }
    }
    for (index, target_id) in runtime.rts_base_assault_target_ids.iter().enumerate() {
        let tile = classic_rts_enemy_structure_tile_for_id(target_id, index);
        let (target_x, target_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            target_x - 19,
            target_y + tile_h - 52,
            38,
            5,
            CLASSIC_RTS_PRODUCTION_SLOT_COLOR,
        );
        let health = runtime
            .rts_enemy_structure_health_percents
            .get(index)
            .copied()
            .unwrap_or(68);
        classic_draw_rect(
            buffer,
            width,
            height,
            target_x - 18,
            target_y + tile_h - 51,
            (health.min(100) as i32 * 36) / 100,
            3,
            CLASSIC_RTS_ENEMY_BASE_HEALTH_COLOR,
        );
        if runtime.rts_base_breach_percent > 0 {
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                target_x,
                target_y + tile_h - 20,
                28,
                10,
                CLASSIC_RTS_BASE_BREACH_COLOR,
            );
        }
    }
    for tile_id in &runtime.rts_aftermath_debris_tile_ids {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (debris_x, debris_y) =
                classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            classic_draw_rect(
                buffer,
                width,
                height,
                debris_x - 18,
                debris_y + tile_h - 33,
                36,
                13,
                CLASSIC_RTS_DEBRIS_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                debris_x - 10,
                debris_y + tile_h - 42,
                20,
                8,
                CLASSIC_RTS_BASE_BREACH_COLOR,
            );
        }
    }
    for tile_id in &runtime.rts_aftermath_smoke_tile_ids {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (smoke_x, smoke_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                smoke_x,
                smoke_y + tile_h - 54,
                17,
                10,
                CLASSIC_RTS_SMOKE_COLOR,
            );
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                smoke_x + 8,
                smoke_y + tile_h - 67,
                11,
                7,
                CLASSIC_RTS_SMOKE_COLOR,
            );
        }
    }
    for (index, unit_id) in runtime.rts_veteran_unit_ids.iter().enumerate() {
        let tile = classic_rts_player_army_unit_tile_for_id(unit_id, index);
        let (unit_x, unit_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            unit_x - 11,
            unit_y + tile_h - 52,
            22,
            4,
            CLASSIC_RTS_VETERAN_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            unit_x - 3,
            unit_y + tile_h - 60,
            6,
            6,
            CLASSIC_RTS_VETERAN_COLOR,
        );
    }
    if let Some(commander_id) = runtime.rts_commander_unit_id.as_deref() {
        let tile = classic_rts_player_army_unit_tile_for_id(commander_id, 0);
        let (commander_x, commander_y) =
            classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            commander_x,
            commander_y + tile_h - 10,
            26,
            10,
            CLASSIC_RTS_COMMANDER_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            commander_x - 9,
            commander_y + tile_h - 58,
            18,
            8,
            CLASSIC_RTS_ABILITY_POINT_COLOR,
        );
    }
    for tile_id in &runtime.rts_commander_aura_tile_ids {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (aura_x, aura_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                aura_x,
                aura_y + tile_h - 15,
                20,
                7,
                CLASSIC_RTS_COMMANDER_AURA_COLOR,
            );
        }
    }
    for (index, _item_id) in runtime.rts_loot_item_ids.iter().enumerate() {
        let tile = (9 + index as i32 % 3, 3 + index as i32 / 3);
        let (loot_x, loot_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            loot_x - 7,
            loot_y + tile_h - 38,
            14,
            10,
            CLASSIC_RTS_LOOT_COLOR,
        );
    }
    if !runtime.rts_next_action_ids.is_empty() {
        let tile = runtime
            .rts_objective_extraction_tile_id
            .as_deref()
            .and_then(classic_parse_rts_tile)
            .unwrap_or((9, 2));
        let (next_x, next_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            next_x,
            next_y + tile_h - 8,
            24,
            9,
            CLASSIC_RTS_NEXT_ACTION_COLOR,
        );
    }
    for structure_id in &runtime.rts_expansion_structure_ids {
        let tile = classic_rts_expansion_structure_tile_for_id(structure_id);
        let (base_x, base_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            base_x - 22,
            base_y + tile_h - 42,
            44,
            10,
            CLASSIC_RTS_EXPANSION_BASE_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            base_x - 7,
            base_y + tile_h - 66,
            14,
            28,
            CLASSIC_RTS_EXPANSION_BASE_COLOR,
        );
    }
    for (index, _worker_id) in runtime.rts_expansion_worker_unit_ids.iter().enumerate() {
        let tile = match index % 3 {
            0 => (9, 2),
            1 => (10, 2),
            _ => (9, 3),
        };
        let (worker_x, worker_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            worker_x,
            worker_y + tile_h - 6,
            13,
            5,
            CLASSIC_RTS_EXPANSION_WORKER_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            worker_x - 6,
            worker_y + tile_h - 32,
            12,
            13,
            CLASSIC_RTS_EXPANSION_WORKER_COLOR,
        );
    }
    if runtime.rts_expansion_income_per_minute > 0 {
        for tile_id in &runtime.rts_expansion_tile_ids {
            if let Some(tile) = classic_parse_rts_tile(tile_id) {
                let (income_x, income_y) =
                    classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    income_x - 9,
                    income_y + tile_h - 50,
                    18,
                    4,
                    CLASSIC_RTS_EXPANSION_INCOME_COLOR,
                );
            }
        }
    }
    if !runtime.rts_enemy_counterattack_route_tile_ids.is_empty() {
        let mut previous_screen: Option<(i32, i32)> = None;
        for tile_id in &runtime.rts_enemy_counterattack_route_tile_ids {
            if let Some(tile) = classic_parse_rts_tile(tile_id) {
                let (route_x, route_y) =
                    classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
                classic_draw_iso_ellipse(
                    buffer,
                    width,
                    height,
                    route_x,
                    route_y + tile_h - 4,
                    14,
                    5,
                    CLASSIC_RTS_COUNTERATTACK_COLOR,
                );
                if let Some((prev_x, prev_y)) = previous_screen {
                    for step in 0..=7 {
                        let line_x = prev_x + ((route_x - prev_x) * step) / 7;
                        let line_y = prev_y + tile_h - 4 + ((route_y - prev_y) * step) / 7;
                        classic_draw_rect(
                            buffer,
                            width,
                            height,
                            line_x - 2,
                            line_y - 1,
                            5,
                            3,
                            CLASSIC_RTS_COUNTERATTACK_COLOR,
                        );
                    }
                }
                previous_screen = Some((route_x, route_y));
            }
        }
    }
    for (index, _unit_id) in runtime.rts_enemy_counterattack_unit_ids.iter().enumerate() {
        let tile = match index % 3 {
            0 => (8, 3),
            1 => (9, 3),
            _ => (10, 2),
        };
        let (enemy_x, enemy_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            enemy_x - 9,
            enemy_y + tile_h - 36,
            18,
            16,
            CLASSIC_RTS_COUNTERATTACK_COLOR,
        );
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            enemy_x,
            enemy_y + tile_h - 7,
            17,
            6,
            CLASSIC_RTS_COUNTERATTACK_COLOR,
        );
    }
    if runtime.rts_expansion_defense_state.starts_with("defended:") {
        for tile_id in ["8,3", "9,2", "10,2"] {
            if let Some(tile) = classic_parse_rts_tile(tile_id) {
                let (defense_x, defense_y) =
                    classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
                classic_draw_iso_ellipse(
                    buffer,
                    width,
                    height,
                    defense_x,
                    defense_y + tile_h - 18,
                    22,
                    8,
                    CLASSIC_RTS_EXPANSION_DEFENSE_COLOR,
                );
            }
        }
    }
    for tech_id in &runtime.rts_tier_two_tech_ids {
        let tile = classic_rts_expansion_structure_tile_for_id(tech_id);
        let (tech_x, tech_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            tech_x - 26,
            tech_y + tile_h - 74,
            52,
            7,
            CLASSIC_RTS_TIER_TWO_TECH_COLOR,
        );
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            tech_x,
            tech_y + tile_h - 22,
            26,
            8,
            CLASSIC_RTS_TIER_TWO_TECH_COLOR,
        );
    }
    for (index, _upgrade_id) in runtime.rts_tier_two_upgrade_ids.iter().enumerate() {
        let tile = (9 + index as i32 % 2, 2);
        let (upgrade_x, upgrade_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            upgrade_x - 16,
            upgrade_y + tile_h - 84,
            32,
            5,
            CLASSIC_RTS_TIER_TWO_TECH_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            upgrade_x - 5,
            upgrade_y + tile_h - 97,
            10,
            10,
            CLASSIC_RTS_TIER_TWO_TECH_COLOR,
        );
    }
    if !runtime.rts_siege_push_route_tile_ids.is_empty() {
        let mut previous_screen: Option<(i32, i32)> = None;
        for tile_id in &runtime.rts_siege_push_route_tile_ids {
            if let Some(tile) = classic_parse_rts_tile(tile_id) {
                let (route_x, route_y) =
                    classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
                classic_draw_iso_ellipse(
                    buffer,
                    width,
                    height,
                    route_x,
                    route_y + tile_h - 2,
                    16,
                    5,
                    CLASSIC_RTS_SIEGE_ROUTE_COLOR,
                );
                if let Some((prev_x, prev_y)) = previous_screen {
                    for step in 0..=7 {
                        let line_x = prev_x + ((route_x - prev_x) * step) / 7;
                        let line_y = prev_y + tile_h - 2 + ((route_y - prev_y) * step) / 7;
                        classic_draw_rect(
                            buffer,
                            width,
                            height,
                            line_x - 2,
                            line_y - 1,
                            5,
                            3,
                            CLASSIC_RTS_SIEGE_ROUTE_COLOR,
                        );
                    }
                }
                previous_screen = Some((route_x, route_y));
            }
        }
    }
    for (index, unit_id) in runtime.rts_siege_unit_ids.iter().enumerate() {
        let tile = classic_rts_siege_unit_tile_for_id(unit_id, index);
        let (siege_x, siege_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            siege_x,
            siege_y + tile_h - 7,
            20,
            7,
            CLASSIC_RTS_SIEGE_UNIT_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            siege_x - 15,
            siege_y + tile_h - 36,
            30,
            14,
            CLASSIC_RTS_SIEGE_UNIT_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            siege_x + 9,
            siege_y + tile_h - 32,
            18,
            4,
            CLASSIC_RTS_SIEGE_UNIT_COLOR,
        );
    }
    for fortification_id in &runtime.rts_enemy_fortification_ids {
        let tile = classic_rts_enemy_fortification_tile_for_id(fortification_id);
        let (fort_x, fort_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            fort_x - 25,
            fort_y + tile_h - 54,
            50,
            26,
            CLASSIC_RTS_ENEMY_FORTIFY_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            fort_x - 30,
            fort_y + tile_h - 62,
            60,
            6,
            CLASSIC_RTS_ENEMY_FORTIFY_COLOR,
        );
        if !runtime.rts_siege_damage_log.is_empty() {
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                fort_x,
                fort_y + tile_h - 28,
                30,
                9,
                CLASSIC_RTS_SIEGE_DAMAGE_COLOR,
            );
        }
    }
    for tile_id in &runtime.rts_siege_breach_tile_ids {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (breach_x, breach_y) =
                classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                breach_x,
                breach_y + tile_h - 10,
                24,
                7,
                CLASSIC_RTS_SIEGE_BREACH_COLOR,
            );
        }
    }
    for (index, _repair_id) in runtime.rts_enemy_repair_unit_ids.iter().enumerate() {
        let tile = if index % 2 == 0 { (10, 2) } else { (11, 3) };
        let (repair_x, repair_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            repair_x - 8,
            repair_y + tile_h - 37,
            16,
            14,
            CLASSIC_RTS_ENEMY_REPAIR_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            repair_x - 16,
            repair_y + tile_h - 21,
            32,
            4,
            CLASSIC_RTS_ENEMY_REPAIR_COLOR,
        );
    }
    for (index, _flank_id) in runtime.rts_enemy_flank_unit_ids.iter().enumerate() {
        let tile = classic_rts_enemy_flank_tile_for_index(index);
        let (flank_x, flank_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            flank_x,
            flank_y + tile_h - 5,
            15,
            5,
            CLASSIC_RTS_ENEMY_FLANK_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            flank_x - 7,
            flank_y + tile_h - 34,
            14,
            13,
            CLASSIC_RTS_ENEMY_FLANK_COLOR,
        );
    }
    for tile_id in &runtime.rts_player_hold_tile_ids {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (hold_x, hold_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                hold_x,
                hold_y + tile_h - 17,
                20,
                7,
                CLASSIC_RTS_PLAYER_HOLD_COLOR,
            );
        }
    }
    if runtime
        .rts_siege_breach_state
        .starts_with("counterplay_won:")
    {
        let tile = classic_rts_enemy_fortification_tile_for_id(
            runtime
                .rts_siege_breach_target_id
                .as_deref()
                .unwrap_or("gate_bulwark"),
        );
        let (win_x, win_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            win_x,
            win_y + tile_h - 32,
            36,
            10,
            CLASSIC_RTS_COUNTERPLAY_RESOLUTION_COLOR,
        );
    }
    if !runtime.rts_inner_lane_tile_ids.is_empty() {
        let mut previous_screen: Option<(i32, i32)> = None;
        for tile_id in &runtime.rts_inner_lane_tile_ids {
            if let Some(tile) = classic_parse_rts_tile(tile_id) {
                let (route_x, route_y) =
                    classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
                classic_draw_iso_ellipse(
                    buffer,
                    width,
                    height,
                    route_x,
                    route_y + tile_h - 3,
                    18,
                    5,
                    CLASSIC_RTS_INNER_ROUTE_COLOR,
                );
                if let Some((prev_x, prev_y)) = previous_screen {
                    for step in 0..=7 {
                        let line_x = prev_x + ((route_x - prev_x) * step) / 7;
                        let line_y = prev_y + tile_h - 3 + ((route_y - prev_y) * step) / 7;
                        classic_draw_rect(
                            buffer,
                            width,
                            height,
                            line_x - 2,
                            line_y - 1,
                            5,
                            3,
                            CLASSIC_RTS_INNER_ROUTE_COLOR,
                        );
                    }
                }
                previous_screen = Some((route_x, route_y));
            }
        }
    }
    for gate_id in &runtime.rts_inner_gate_ids {
        let tile = classic_rts_inner_gate_tile_for_id(gate_id);
        let (gate_x, gate_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            gate_x - 20,
            gate_y + tile_h - 58,
            40,
            24,
            CLASSIC_RTS_INNER_GATE_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            gate_x - 24,
            gate_y + tile_h - 66,
            48,
            5,
            CLASSIC_RTS_INNER_GATE_COLOR,
        );
    }
    for (index, _defender_id) in runtime.rts_inner_defender_unit_ids.iter().enumerate() {
        let tile = match index % 3 {
            0 => (11, 3),
            1 => (12, 3),
            _ => (12, 4),
        };
        let (def_x, def_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            def_x,
            def_y + tile_h - 6,
            16,
            5,
            CLASSIC_RTS_INNER_DEFENDER_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            def_x - 8,
            def_y + tile_h - 34,
            16,
            14,
            CLASSIC_RTS_INNER_DEFENDER_COLOR,
        );
    }
    for (index, _convoy_id) in runtime.rts_supply_convoy_ids.iter().enumerate() {
        let tile = match index % 3 {
            0 => (9, 3),
            1 => (10, 3),
            _ => (10, 4),
        };
        let (convoy_x, convoy_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            convoy_x - 14,
            convoy_y + tile_h - 31,
            28,
            12,
            CLASSIC_RTS_INNER_SUPPLY_COLOR,
        );
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            convoy_x,
            convoy_y + tile_h - 5,
            18,
            5,
            CLASSIC_RTS_INNER_SUPPLY_COLOR,
        );
    }
    for tile_id in &runtime.rts_split_squad_tile_ids {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (split_x, split_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                split_x,
                split_y + tile_h - 14,
                18,
                6,
                CLASSIC_RTS_INNER_SPLIT_COLOR,
            );
        }
    }
    if runtime
        .rts_inner_objective_state
        .starts_with("inner_core_secured:")
    {
        let core_id = runtime
            .rts_inner_objective_state
            .strip_prefix("inner_core_secured:")
            .unwrap_or("signal_core");
        let tile = classic_rts_inner_core_tile_for_id(core_id);
        let (core_x, core_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            core_x,
            core_y + tile_h - 32,
            34,
            11,
            CLASSIC_RTS_INNER_CORE_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            core_x - 8,
            core_y + tile_h - 70,
            16,
            35,
            CLASSIC_RTS_INNER_CORE_COLOR,
        );
    }
    if !runtime.rts_central_keep_route_tile_ids.is_empty() {
        let mut previous_screen: Option<(i32, i32)> = None;
        for tile_id in &runtime.rts_central_keep_route_tile_ids {
            if let Some(tile) = classic_parse_rts_tile(tile_id) {
                let (route_x, route_y) =
                    classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
                classic_draw_iso_ellipse(
                    buffer,
                    width,
                    height,
                    route_x,
                    route_y + tile_h - 2,
                    18,
                    5,
                    CLASSIC_RTS_KEEP_ROUTE_COLOR,
                );
                if let Some((prev_x, prev_y)) = previous_screen {
                    for step in 0..=7 {
                        let line_x = prev_x + ((route_x - prev_x) * step) / 7;
                        let line_y = prev_y + tile_h - 2 + ((route_y - prev_y) * step) / 7;
                        classic_draw_rect(
                            buffer,
                            width,
                            height,
                            line_x - 2,
                            line_y - 1,
                            5,
                            3,
                            CLASSIC_RTS_KEEP_ROUTE_COLOR,
                        );
                    }
                }
                previous_screen = Some((route_x, route_y));
            }
        }
    }
    for target_id in &runtime.rts_central_keep_target_ids {
        let tile = classic_rts_central_keep_tile_for_id(target_id);
        let (keep_x, keep_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            keep_x - 28,
            keep_y + tile_h - 74,
            56,
            34,
            CLASSIC_RTS_KEEP_SHIELD_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            keep_x - 10,
            keep_y + tile_h - 102,
            20,
            28,
            CLASSIC_RTS_KEEP_SHIELD_COLOR,
        );
        if runtime.rts_keep_shield_percent <= 24 && runtime.rts_keep_shield_percent > 0 {
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                keep_x,
                keep_y + tile_h - 45,
                38,
                11,
                CLASSIC_RTS_KEEP_PRESSURE_COLOR,
            );
        }
    }
    for (index, _guard_id) in runtime.rts_boss_guard_unit_ids.iter().enumerate() {
        let tile = match index % 3 {
            0 => (12, 3),
            1 => (13, 4),
            _ => (12, 4),
        };
        let (guard_x, guard_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            guard_x,
            guard_y + tile_h - 5,
            18,
            6,
            CLASSIC_RTS_KEEP_GUARD_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            guard_x - 9,
            guard_y + tile_h - 37,
            18,
            16,
            CLASSIC_RTS_KEEP_GUARD_COLOR,
        );
    }
    for tile_id in &runtime.rts_player_siege_line_tile_ids {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (line_x, line_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                line_x,
                line_y + tile_h - 16,
                21,
                7,
                CLASSIC_RTS_KEEP_SIEGE_LINE_COLOR,
            );
        }
    }
    for tile_id in &runtime.rts_keep_breach_tile_ids {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (breach_x, breach_y) =
                classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                breach_x,
                breach_y + tile_h - 24,
                24,
                8,
                CLASSIC_RTS_KEEP_BREACH_COLOR,
            );
        }
    }
    for (index, _unit_id) in runtime.rts_guardian_counter_unit_ids.iter().enumerate() {
        let tile = match index % 3 {
            0 => (13, 4),
            1 => (14, 3),
            _ => (14, 4),
        };
        let (counter_x, counter_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            counter_x - 10,
            counter_y + tile_h - 42,
            20,
            18,
            CLASSIC_RTS_KEEP_COUNTER_COLOR,
        );
    }
    for tile_id in &runtime.rts_keep_claim_tile_ids {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (claim_x, claim_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                claim_x,
                claim_y + tile_h - 7,
                22,
                6,
                CLASSIC_RTS_KEEP_CLAIM_COLOR,
            );
        }
    }
    if runtime.rts_victory_banner_state.starts_with("victory:") {
        let (banner_x, banner_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, (13, 3));
        classic_draw_rect(
            buffer,
            width,
            height,
            banner_x - 3,
            banner_y + tile_h - 124,
            6,
            58,
            CLASSIC_RTS_KEEP_VICTORY_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            banner_x + 3,
            banner_y + tile_h - 124,
            30,
            18,
            CLASSIC_RTS_KEEP_CLAIM_COLOR,
        );
    }
    for (index, _zone_id) in runtime.rts_restored_zone_ids.iter().enumerate() {
        let tile = match index % 4 {
            0 => (13, 3),
            1 => (12, 3),
            2 => (11, 3),
            _ => (9, 2),
        };
        let (zone_x, zone_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            zone_x,
            zone_y + tile_h - 8,
            24,
            7,
            CLASSIC_RTS_RESTORE_ZONE_COLOR,
        );
    }
    for (index, _structure_id) in runtime.rts_rebuild_structure_ids.iter().enumerate() {
        let tile = match index % 3 {
            0 => (12, 3),
            1 => (11, 3),
            _ => (13, 3),
        };
        let (rebuild_x, rebuild_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            rebuild_x - 12,
            rebuild_y + tile_h - 54,
            24,
            24,
            CLASSIC_RTS_REBUILD_CORE_COLOR,
        );
    }
    for (index, _unit_id) in runtime.rts_garrison_unit_ids.iter().enumerate() {
        let tile = match index % 3 {
            0 => (13, 3),
            1 => (13, 4),
            _ => (12, 3),
        };
        let (garrison_x, garrison_y) =
            classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            garrison_x - 8,
            garrison_y + tile_h - 38,
            16,
            16,
            CLASSIC_RTS_GARRISON_COLOR,
        );
    }
    if runtime
        .rts_victory_handoff_state
        .starts_with("handoff_ready:")
    {
        let (handoff_x, handoff_y) =
            classic_iso_project(origin_x, origin_y, tile_w, tile_h, (13, 3));
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            handoff_x,
            handoff_y + tile_h - 62,
            42,
            12,
            CLASSIC_RTS_HANDOFF_COLOR,
        );
    }
    for tile_id in &runtime.rts_open_world_route_tile_ids {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (route_x, route_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                route_x,
                route_y + tile_h - 4,
                18,
                5,
                CLASSIC_RTS_OPEN_WORLD_ROUTE_COLOR,
            );
        }
    }
    for (index, _panel_id) in runtime.rts_open_world_panel_ids.iter().enumerate() {
        let tile = match index % 4 {
            0 => (12, 2),
            1 => (13, 2),
            2 => (14, 2),
            _ => (13, 1),
        };
        let (panel_x, panel_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            panel_x - 12,
            panel_y + tile_h - 70,
            24,
            14,
            CLASSIC_RTS_OPEN_WORLD_PANEL_COLOR,
        );
    }
    if runtime.rts_open_world_handoff_state.starts_with("resumed:") {
        let (resume_x, resume_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, (12, 3));
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            resume_x,
            resume_y + tile_h - 82,
            44,
            10,
            CLASSIC_RTS_OPEN_WORLD_RESUME_COLOR,
        );
    }
    for node_id in &runtime.rts_harvest_node_ids {
        let node_tile = classic_rts_harvest_tile_for_node(node_id);
        let (node_x, node_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, node_tile);
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            node_x,
            node_y + tile_h - 4,
            18,
            8,
            CLASSIC_RTS_HARVEST_NODE_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            node_x - 10,
            node_y + tile_h - 24,
            20,
            5,
            CLASSIC_RTS_HARVEST_NODE_COLOR,
        );
        for entity in &selected_units {
            if runtime
                .rts_worker_assignment_ids
                .iter()
                .any(|assignment| assignment.starts_with(&entity.id))
            {
                let (unit_x, unit_y) =
                    classic_iso_project(origin_x, origin_y, tile_w, tile_h, entity.tile);
                for step in 0..=8 {
                    let route_x = unit_x + ((node_x - unit_x) * step) / 8;
                    let route_y = unit_y + tile_h - 8 + ((node_y - unit_y) * step) / 8;
                    classic_draw_rect(
                        buffer,
                        width,
                        height,
                        route_x - 1,
                        route_y - 1,
                        4,
                        3,
                        CLASSIC_RTS_WORKER_ROUTE_COLOR,
                    );
                }
            }
        }
    }
    if let Some(dropoff_id) = runtime.rts_dropoff_structure_id.as_deref() {
        let dropoff_tile = classic_rts_dropoff_tile_for_structure(dropoff_id);
        let (dropoff_x, dropoff_y) =
            classic_iso_project(origin_x, origin_y, tile_w, tile_h, dropoff_tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            dropoff_x - 18,
            dropoff_y + tile_h - 28,
            36,
            5,
            CLASSIC_RTS_DROPOFF_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            dropoff_x - 4,
            dropoff_y + tile_h - 38,
            8,
            18,
            CLASSIC_RTS_DROPOFF_COLOR,
        );
    }
    for tile_id in &runtime.rts_build_site_tile_ids {
        if let Some(tile) = classic_parse_rts_tile(tile_id) {
            let (site_x, site_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
            classic_draw_iso_diamond(
                buffer,
                width,
                height,
                site_x,
                site_y + tile_h - 5,
                tile_w / 2,
                tile_h / 2,
                CLASSIC_RTS_BUILD_BLUEPRINT_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                site_x - 14,
                site_y + tile_h - 17,
                (runtime.rts_building_progress_percent.max(1) as i32 * 28) / 100,
                4,
                CLASSIC_RTS_BLUEPRINT_PROGRESS_COLOR,
            );
        }
    }
    for (index, structure_id) in runtime.rts_completed_structure_ids.iter().enumerate() {
        let structure_tile = classic_rts_structure_tile_for_id(structure_id);
        let (structure_x, structure_y) =
            classic_iso_project(origin_x, origin_y, tile_w, tile_h, structure_tile);
        classic_draw_iso_diamond(
            buffer,
            width,
            height,
            structure_x,
            structure_y + tile_h - 11,
            tile_w / 2,
            tile_h / 2,
            CLASSIC_RTS_STRUCTURE_COMPLETE_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            structure_x - 16,
            structure_y + tile_h - 37,
            32,
            5,
            CLASSIC_RTS_STRUCTURE_COMPLETE_COLOR,
        );
        let health = runtime
            .rts_structure_health_percents
            .get(index)
            .copied()
            .unwrap_or(100);
        classic_draw_rect(
            buffer,
            width,
            height,
            structure_x - 18,
            structure_y + tile_h - 30,
            36,
            4,
            CLASSIC_RTS_PRODUCTION_SLOT_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            structure_x - 18,
            structure_y + tile_h - 30,
            (health.min(100) as i32 * 36) / 100,
            4,
            CLASSIC_RTS_STRUCTURE_HEALTH_COLOR,
        );
    }
    if let Some(repair_target_id) = runtime.rts_repair_target_id.as_deref() {
        let repair_tile = classic_rts_structure_tile_for_id(repair_target_id);
        let (repair_x, repair_y) =
            classic_iso_project(origin_x, origin_y, tile_w, tile_h, repair_tile);
        for entity in &selected_units {
            let (unit_x, unit_y) =
                classic_iso_project(origin_x, origin_y, tile_w, tile_h, entity.tile);
            for step in 0..=8 {
                let beam_x = unit_x + ((repair_x - unit_x) * step) / 8;
                let beam_y = unit_y + tile_h - 8 + ((repair_y - unit_y) * step) / 8;
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    beam_x - 1,
                    beam_y - 1,
                    4,
                    3,
                    CLASSIC_RTS_STRUCTURE_REPAIR_COLOR,
                );
            }
        }
        classic_draw_rect(
            buffer,
            width,
            height,
            repair_x - 20,
            repair_y + tile_h - 44,
            (runtime.rts_repair_progress_percent.min(100) as i32 * 40) / 100,
            5,
            CLASSIC_RTS_STRUCTURE_REPAIR_COLOR,
        );
    }
    for structure_id in &runtime.rts_cancelled_structure_ids {
        let cancel_tile = classic_rts_structure_tile_for_id(structure_id);
        let (cancel_x, cancel_y) =
            classic_iso_project(origin_x, origin_y, tile_w, tile_h, cancel_tile);
        for step in -10..=10 {
            classic_draw_rect(
                buffer,
                width,
                height,
                cancel_x + step,
                cancel_y + tile_h - 25 + step,
                3,
                3,
                CLASSIC_RTS_STRUCTURE_CANCEL_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                cancel_x + step,
                cancel_y + tile_h - 5 - step,
                3,
                3,
                CLASSIC_RTS_STRUCTURE_CANCEL_COLOR,
            );
        }
        if !runtime.rts_refund_delta_log.is_empty() {
            classic_draw_text(
                buffer,
                width,
                height,
                cancel_x - 22,
                cancel_y + tile_h - 42,
                "REFUND",
                1,
                CLASSIC_RTS_STRUCTURE_CANCEL_COLOR,
            );
        }
    }
    for structure_id in &runtime.rts_base_structure_ids {
        let base_tile = classic_rts_structure_tile_for_id(structure_id);
        let (base_x, base_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, base_tile);
        classic_draw_iso_diamond(
            buffer,
            width,
            height,
            base_x,
            base_y + tile_h - 13,
            tile_w / 2,
            tile_h / 2,
            CLASSIC_RTS_TECH_BASE_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            base_x - 18,
            base_y + tile_h - 41,
            36,
            4,
            CLASSIC_RTS_TECH_BASE_COLOR,
        );
    }
    for (index, tech_id) in runtime.rts_tech_research_ids.iter().enumerate() {
        let tech_tile = if tech_id.contains("wayfinder") {
            (5, 4)
        } else {
            (6, 4)
        };
        let (tech_x, tech_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tech_tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            tech_x - 20,
            tech_y + tile_h - 50 - index as i32 * 7,
            (runtime.rts_tech_progress_percent.min(100) as i32 * 40) / 100,
            5,
            CLASSIC_RTS_TECH_RESEARCH_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            tech_x + 23,
            tech_y + tile_h - 50 - index as i32 * 7,
            8,
            5,
            CLASSIC_RTS_TECH_RESEARCH_COLOR,
        );
    }
    for (index, upgrade_id) in runtime.rts_completed_upgrade_ids.iter().enumerate() {
        let upgrade_tile = if upgrade_id.contains("iron") {
            (4, 3)
        } else {
            (6, 3)
        };
        let (upgrade_x, upgrade_y) =
            classic_iso_project(origin_x, origin_y, tile_w, tile_h, upgrade_tile);
        classic_draw_rect(
            buffer,
            width,
            height,
            upgrade_x - 18,
            upgrade_y + tile_h - 57 - index as i32 * 6,
            36,
            4,
            CLASSIC_RTS_TECH_UPGRADE_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            upgrade_x - 3,
            upgrade_y + tile_h - 64 - index as i32 * 6,
            6,
            10,
            CLASSIC_RTS_TECH_UPGRADE_COLOR,
        );
    }
    for unit_id in &runtime.rts_unlocked_unit_ids {
        let unit_tile = classic_rts_unlock_unit_tile_for_id(unit_id);
        let (unlock_x, unlock_y) =
            classic_iso_project(origin_x, origin_y, tile_w, tile_h, unit_tile);
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            unlock_x,
            unlock_y + tile_h - 2,
            16,
            6,
            CLASSIC_RTS_TECH_UNLOCK_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            unlock_x - 12,
            unlock_y + tile_h - 27,
            24,
            4,
            CLASSIC_RTS_TECH_UNLOCK_COLOR,
        );
    }
    for (index, _entry) in runtime.rts_tech_requirements_log.iter().take(4).enumerate() {
        classic_draw_rect(
            buffer,
            width,
            height,
            118 + index as i32 * 18,
            154,
            12,
            5,
            CLASSIC_RTS_TECH_REQUIREMENT_COLOR,
        );
    }
    let command_marker_drawn = classic_blit_frame_override_bottom_center(
        buffer,
        width,
        height,
        assets,
        "rts_command_destination_marker",
        dest_x,
        dest_y + tile_h + 5,
    );
    if !command_marker_drawn {
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            dest_x,
            dest_y + tile_h - 2,
            18,
            7,
            CLASSIC_ISO_COMMAND_MARKER_COLOR,
        );
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            dest_x,
            dest_y + tile_h - 2,
            10,
            4,
            CLASSIC_ISO_FOUNDATION_COLOR,
        );
    }
    let explicit_rts_command = !selected_units.is_empty()
        || runtime.rts_command_destination_tile.is_some()
        || !runtime.rts_command_queue.is_empty();
    if explicit_rts_command {
        for (dx, dy, rect_w, rect_h) in [
            (-22, -3, 12, 2),
            (10, -3, 12, 2),
            (-22, 8, 12, 2),
            (10, 8, 12, 2),
            (-22, -3, 2, 12),
            (20, -3, 2, 12),
        ] {
            classic_draw_rect(
                buffer,
                width,
                height,
                dest_x + dx,
                dest_y + tile_h + dy,
                rect_w,
                rect_h,
                CLASSIC_ISO_COMMAND_MARKER_COLOR,
            );
        }
    }

    let queued_attack_command = runtime
        .rts_command_queue
        .iter()
        .any(|entry| entry.contains("attack"));
    if runtime.combat_overlay_visible || runtime.combat_overlay_was_visible || queued_attack_command
    {
        let attack_arc_drawn = classic_blit_frame_override_bottom_center(
            buffer,
            width,
            height,
            assets,
            "combat_attack_arc",
            dest_x + 8,
            dest_y + 4,
        );
        if !attack_arc_drawn {
            for step in 0..28 {
                classic_draw_rect(
                    buffer,
                    width,
                    height,
                    dest_x - 32 + step * 2,
                    dest_y - 28 + step,
                    6,
                    3,
                    CLASSIC_ISO_ATTACK_ARC_COLOR,
                );
            }
        }
        let hit_flash_drawn = classic_blit_frame_override_bottom_center(
            buffer,
            width,
            height,
            assets,
            "combat_hit_flash",
            dest_x + 12,
            dest_y,
        );
        if !hit_flash_drawn {
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                dest_x + 12,
                dest_y - 10,
                15,
                9,
                CLASSIC_ISO_HIT_FLASH_COLOR,
            );
        }
    }
    true
}

#[cfg(not(target_os = "android"))]
#[allow(clippy::too_many_arguments)]
pub(super) fn classic_draw_isometric_frame_at_tile(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    assets: &ClassicRuntimeAssets,
    runtime: Option<&NativeFirstPlayableRuntime>,
    frame_id: &str,
    origin_x: i32,
    origin_y: i32,
    tile_w: i32,
    tile_h: i32,
    tile: (i32, i32),
    scale: u32,
) {
    let (screen_x, screen_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, tile);
    let sprite_scale = classic_entity_sprite_scale(assets, frame_id, scale);
    let sprite_px = assets.manifest.source_tile_size_px as i32 * sprite_scale.max(1) as i32;
    if frame_id.starts_with("model_") {
        classic_draw_iso_diamond(
            buffer,
            width,
            height,
            screen_x,
            screen_y + tile_h - 6,
            tile_w * 3,
            tile_h + 14,
            CLASSIC_ISO_FOUNDATION_COLOR,
        );
    }
    classic_draw_iso_shadow(
        buffer,
        width,
        height,
        screen_x,
        screen_y + tile_h - 2,
        sprite_px / 3,
        4,
    );
    if assets
        .frame_override_pixels
        .get(frame_id)
        .is_some_and(|frame| {
            frame.width > assets.manifest.source_tile_size_px
                || frame.height > assets.manifest.source_tile_size_px
        })
        && classic_blit_frame_override_bottom_center(
            buffer,
            width,
            height,
            assets,
            frame_id,
            screen_x,
            screen_y + tile_h,
        )
    {
        if let Some(phase) = classic_rts_action_sequence_phase(frame_id, runtime) {
            classic_draw_rts_action_sequence_marks(
                buffer,
                width,
                height,
                frame_id,
                screen_x,
                screen_y + tile_h,
                phase,
            );
        }
        if let Some(behavior) = classic_rts_npc_behavior_stage(runtime) {
            classic_draw_rts_npc_behavior_marks(
                buffer,
                width,
                height,
                frame_id,
                screen_x,
                screen_y + tile_h,
                behavior,
            );
        }
        if let Some(impact) = classic_rts_combat_impact_stage(runtime) {
            classic_draw_rts_combat_impact_marks(
                buffer,
                width,
                height,
                frame_id,
                screen_x,
                screen_y + tile_h,
                impact,
            );
        }
        if let Some(locomotion) = classic_rts_locomotion_blend_stage(runtime) {
            classic_draw_rts_locomotion_blend_marks(
                buffer,
                width,
                height,
                frame_id,
                screen_x,
                screen_y + tile_h,
                locomotion,
            );
        }
        if let Some(transition) = classic_rts_npc_transition_stage(runtime) {
            classic_draw_rts_npc_transition_marks(
                buffer,
                width,
                height,
                frame_id,
                screen_x,
                screen_y + tile_h,
                transition,
            );
        }
        if let Some(depth_stage) = classic_rts_depth_readability_stage(runtime) {
            classic_draw_rts_depth_readability_marks(
                buffer,
                width,
                height,
                frame_id,
                screen_x,
                screen_y + tile_h,
                depth_stage,
            );
        }
        return;
    }
    if !assets.frame_by_id.contains_key(frame_id)
        && classic_blit_frame_override_bottom_center(
            buffer,
            width,
            height,
            assets,
            frame_id,
            screen_x,
            screen_y + tile_h,
        )
    {
        if let Some(phase) = classic_rts_action_sequence_phase(frame_id, runtime) {
            classic_draw_rts_action_sequence_marks(
                buffer,
                width,
                height,
                frame_id,
                screen_x,
                screen_y + tile_h,
                phase,
            );
        }
        if let Some(behavior) = classic_rts_npc_behavior_stage(runtime) {
            classic_draw_rts_npc_behavior_marks(
                buffer,
                width,
                height,
                frame_id,
                screen_x,
                screen_y + tile_h,
                behavior,
            );
        }
        if let Some(impact) = classic_rts_combat_impact_stage(runtime) {
            classic_draw_rts_combat_impact_marks(
                buffer,
                width,
                height,
                frame_id,
                screen_x,
                screen_y + tile_h,
                impact,
            );
        }
        if let Some(locomotion) = classic_rts_locomotion_blend_stage(runtime) {
            classic_draw_rts_locomotion_blend_marks(
                buffer,
                width,
                height,
                frame_id,
                screen_x,
                screen_y + tile_h,
                locomotion,
            );
        }
        if let Some(transition) = classic_rts_npc_transition_stage(runtime) {
            classic_draw_rts_npc_transition_marks(
                buffer,
                width,
                height,
                frame_id,
                screen_x,
                screen_y + tile_h,
                transition,
            );
        }
        if let Some(depth_stage) = classic_rts_depth_readability_stage(runtime) {
            classic_draw_rts_depth_readability_marks(
                buffer,
                width,
                height,
                frame_id,
                screen_x,
                screen_y + tile_h,
                depth_stage,
            );
        }
        return;
    }
    classic_draw_iso_procedural_model(
        buffer, width, height, frame_id, screen_x, screen_y, tile_w, tile_h,
    );
    classic_blit_frame_scaled(
        buffer,
        width,
        height,
        assets,
        frame_id,
        screen_x - sprite_px / 2,
        screen_y + tile_h - sprite_px,
        sprite_scale,
    );
    classic_draw_iso_unit_overlay(
        buffer,
        width,
        height,
        frame_id,
        screen_x,
        screen_y + tile_h - sprite_px,
    );
    if let Some(phase) = classic_rts_action_sequence_phase(frame_id, runtime) {
        classic_draw_rts_action_sequence_marks(
            buffer,
            width,
            height,
            frame_id,
            screen_x,
            screen_y + tile_h,
            phase,
        );
    }
    if let Some(behavior) = classic_rts_npc_behavior_stage(runtime) {
        classic_draw_rts_npc_behavior_marks(
            buffer,
            width,
            height,
            frame_id,
            screen_x,
            screen_y + tile_h,
            behavior,
        );
    }
    if let Some(impact) = classic_rts_combat_impact_stage(runtime) {
        classic_draw_rts_combat_impact_marks(
            buffer,
            width,
            height,
            frame_id,
            screen_x,
            screen_y + tile_h,
            impact,
        );
    }
    if let Some(locomotion) = classic_rts_locomotion_blend_stage(runtime) {
        classic_draw_rts_locomotion_blend_marks(
            buffer,
            width,
            height,
            frame_id,
            screen_x,
            screen_y + tile_h,
            locomotion,
        );
    }
    if let Some(transition) = classic_rts_npc_transition_stage(runtime) {
        classic_draw_rts_npc_transition_marks(
            buffer,
            width,
            height,
            frame_id,
            screen_x,
            screen_y + tile_h,
            transition,
        );
    }
    if let Some(depth_stage) = classic_rts_depth_readability_stage(runtime) {
        classic_draw_rts_depth_readability_marks(
            buffer,
            width,
            height,
            frame_id,
            screen_x,
            screen_y + tile_h,
            depth_stage,
        );
    }
}

#[cfg(not(target_os = "android"))]
#[allow(clippy::too_many_arguments)]
pub(super) fn classic_draw_rts_product_map_density_layer(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    origin_x: i32,
    origin_y: i32,
    tile_w: i32,
    tile_h: i32,
    scene_id: &str,
) {
    let (lane_tiles, expansion_tiles, resource_tiles, base_tiles): (
        &[(i32, i32)],
        &[(i32, i32)],
        &[(i32, i32)],
        &[(i32, i32)],
    ) = match scene_id {
        "mentor_training_room" => (
            &[(3, 3), (4, 3), (5, 3), (6, 4), (7, 4), (8, 4)],
            &[(3, 5), (8, 5)],
            &[(2, 2), (9, 2), (9, 5)],
            &[(4, 1), (7, 1)],
        ),
        "league_coliseum" => (
            &[(3, 4), (4, 4), (5, 4), (6, 4), (7, 4), (8, 4), (9, 4)],
            &[(2, 5), (9, 5)],
            &[(5, 5), (6, 5), (9, 5)],
            &[(3, 2), (8, 5)],
        ),
        _ => (
            &[
                (2, 4),
                (3, 4),
                (4, 4),
                (5, 4),
                (6, 4),
                (7, 4),
                (8, 4),
                (9, 4),
            ],
            &[(2, 6), (9, 2), (10, 5)],
            &[(4, 6), (8, 5), (10, 4)],
            &[(2, 2), (4, 5), (9, 3)],
        ),
    };

    for (index, tile) in lane_tiles.iter().enumerate() {
        let (center_x, screen_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, *tile);
        classic_draw_iso_diamond(
            buffer,
            width,
            height,
            center_x,
            screen_y + tile_h - 11,
            tile_w / 2,
            tile_h / 2,
            CLASSIC_RTS_PRODUCT_LANE_COLOR,
        );
        if index % 2 == 0 {
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 15,
                screen_y + tile_h - 7,
                30,
                3,
                classic_darken(CLASSIC_RTS_PRODUCT_LANE_COLOR, 1, 4),
            );
        }
        classic_draw_rect(
            buffer,
            width,
            height,
            center_x - 20,
            screen_y + tile_h - 3,
            40,
            3,
            CLASSIC_RTS_PRODUCT_LANE_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            center_x - 3,
            screen_y + tile_h - 13,
            6,
            20,
            CLASSIC_RTS_PRODUCT_LANE_COLOR,
        );
    }

    for tile in expansion_tiles {
        let (center_x, screen_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, *tile);
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            center_x,
            screen_y + tile_h + 1,
            26,
            9,
            CLASSIC_RTS_PRODUCT_MAP_DENSITY_COLOR,
        );
        classic_draw_iso_ellipse(
            buffer,
            width,
            height,
            center_x,
            screen_y + tile_h - 2,
            15,
            5,
            CLASSIC_ISO_FOUNDATION_COLOR,
        );
    }

    for tile in resource_tiles {
        let (center_x, screen_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, *tile);
        for (dx, dy, w) in [(-20, -7, 13), (-7, -13, 18), (10, -8, 12)] {
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x + dx,
                screen_y + tile_h + dy,
                w,
                4,
                CLASSIC_RTS_PRODUCT_RESOURCE_COLOR,
            );
        }
        classic_draw_rect(
            buffer,
            width,
            height,
            center_x - 24,
            screen_y + tile_h - 14,
            48,
            5,
            CLASSIC_RTS_PRODUCT_RESOURCE_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            center_x - 10,
            screen_y + tile_h - 20,
            20,
            6,
            CLASSIC_RTS_PRODUCT_RESOURCE_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            center_x - 18,
            screen_y + tile_h - 2,
            36,
            3,
            classic_darken(CLASSIC_RTS_PRODUCT_RESOURCE_COLOR, 1, 3),
        );
    }

    for tile in base_tiles {
        let (center_x, screen_y) = classic_iso_project(origin_x, origin_y, tile_w, tile_h, *tile);
        classic_draw_iso_diamond(
            buffer,
            width,
            height,
            center_x,
            screen_y + tile_h - 9,
            tile_w,
            tile_h,
            CLASSIC_ISO_FOUNDATION_COLOR,
        );
        for step in 0..4 {
            classic_draw_rect(
                buffer,
                width,
                height,
                center_x - 28 + step * 18,
                screen_y + tile_h - 2 + (step % 2),
                12,
                3,
                CLASSIC_RTS_PRODUCT_MODEL_VOLUME_COLOR,
            );
        }
        classic_draw_rect(
            buffer,
            width,
            height,
            center_x - 32,
            screen_y + tile_h - 16,
            64,
            5,
            CLASSIC_RTS_PRODUCT_MODEL_VOLUME_COLOR,
        );
        classic_draw_rect(
            buffer,
            width,
            height,
            center_x - 22,
            screen_y + tile_h - 25,
            44,
            5,
            CLASSIC_RTS_PRODUCT_MODEL_VOLUME_COLOR,
        );
    }
}

#[cfg(not(target_os = "android"))]
#[allow(clippy::too_many_arguments)]
pub(super) fn classic_draw_isometric_scene(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    scene: Option<&ClassicSceneMap>,
    assets: &ClassicRuntimeAssets,
    runtime: &NativeFirstPlayableRuntime,
    player_tile: (i32, i32),
    player_frame: &str,
) {
    let origin_x = if width >= 1100 {
        ((width as i32 - 290) / 2).clamp(360, 560)
    } else {
        (width as i32 / 2).clamp(260, (width as i32 - 280).max(340))
    };
    let origin_y = if width >= 1100 {
        74
    } else if width >= 900 {
        54
    } else {
        48
    };
    let tile_w = if width >= 1100 {
        72
    } else if width >= 900 {
        56
    } else {
        50
    };
    let tile_h = if width >= 1100 {
        36
    } else if width >= 900 {
        28
    } else {
        25
    };
    let base_scale =
        (assets.manifest.render_tile_size_px / assets.manifest.source_tile_size_px).max(1);
    let scale = if width >= 1100 {
        base_scale.max(2)
    } else {
        base_scale
    };

    if let Some(scene) = scene {
        let mut entities = Vec::new();
        let mut terrain_overlays: Vec<(&str, i32, i32)> = Vec::new();
        for (row_idx, row) in scene.tile_rows.iter().enumerate() {
            for (col_idx, key) in row.chars().enumerate() {
                let frame_id = classic_scene_tile_frame_id(scene, key);
                let color = classic_frame_anchor_color(assets, frame_id);
                let (screen_x, screen_y) = classic_iso_project(
                    origin_x,
                    origin_y,
                    tile_w,
                    tile_h,
                    (col_idx as i32, row_idx as i32),
                );
                let terrain_override_drawn = classic_blit_frame_override_bottom_center(
                    buffer,
                    width,
                    height,
                    assets,
                    frame_id,
                    screen_x,
                    screen_y + tile_h,
                );
                if !terrain_override_drawn
                    && matches!(frame_id, "tile_wall" | "tile_roof" | "tile_arena")
                {
                    classic_draw_iso_terrain_detail(
                        buffer, width, height, frame_id, screen_x, screen_y, tile_w, tile_h,
                    );
                }
                if !terrain_override_drawn {
                    classic_draw_iso_diamond(
                        buffer, width, height, screen_x, screen_y, tile_w, tile_h, color,
                    );
                }
                if !terrain_override_drawn && matches!(frame_id, "tile_road" | "tile_water") {
                    terrain_overlays.push((frame_id, screen_x, screen_y));
                }
                if frame_id == "tile_tree" {
                    let tile = (col_idx as i32, row_idx as i32);
                    entities.push(ClassicIsoEntity {
                        id: format!("tree_{}_{}", tile.0, tile.1),
                        frame_id: "tile_tree".to_string(),
                        tile,
                        depth_key: (tile.0 + tile.1) * 10 + 3,
                    });
                }
            }
        }
        for (frame_id, screen_x, screen_y) in terrain_overlays {
            classic_draw_iso_terrain_detail(
                buffer, width, height, frame_id, screen_x, screen_y, tile_w, tile_h,
            );
        }
        classic_draw_rts_product_map_density_layer(
            buffer,
            width,
            height,
            origin_x,
            origin_y,
            tile_w,
            tile_h,
            scene.id.as_str(),
        );

        entities.extend(scene.landmarks.iter().map(|landmark| ClassicIsoEntity {
            id: landmark.id.clone(),
            frame_id: classic_dynamic_landmark_frame_id(landmark, runtime).to_string(),
            tile: (landmark.tile_x, landmark.tile_y),
            depth_key: (landmark.tile_x + landmark.tile_y) * 10 + 4,
        }));
        entities.extend(classic_scene_rts_model_entities(scene.id.as_str()));
        entities.extend(classic_scene_rts_environment_entities(scene.id.as_str()));
        entities.extend(classic_scene_rts_neutral_unit_entities(scene.id.as_str()));
        entities.extend(classic_scene_rts_doodad_entities(scene.id.as_str()));
        if runtime.dialogue_overlay_visible {
            entities.push(ClassicIsoEntity {
                id: "dialogue_objective_marker".to_string(),
                frame_id: "marker_objective".to_string(),
                tile: (4, 4),
                depth_key: 84,
            });
        }
        if runtime.combat_overlay_visible || runtime.combat_overlay_was_visible {
            entities.push(ClassicIsoEntity {
                id: "combat_objective_marker".to_string(),
                frame_id: "marker_objective".to_string(),
                tile: (9, 2),
                depth_key: 114,
            });
        }
        entities.push(ClassicIsoEntity {
            id: "player".to_string(),
            frame_id: player_frame.to_string(),
            tile: player_tile,
            depth_key: (player_tile.0 + player_tile.1) * 10 + 5,
        });
        entities.sort_by(|left, right| {
            left.depth_key
                .cmp(&right.depth_key)
                .then(left.tile.1.cmp(&right.tile.1))
                .then(left.tile.0.cmp(&right.tile.0))
                .then(left.id.cmp(&right.id))
        });
        for entity in entities {
            classic_draw_isometric_frame_at_tile(
                buffer,
                width,
                height,
                assets,
                Some(runtime),
                &entity.frame_id,
                origin_x,
                origin_y,
                tile_w,
                tile_h,
                entity.tile,
                scale,
            );
        }
        if let Some(structure_stage) = classic_rts_structure_modeling_stage(Some(runtime)) {
            classic_draw_rts_structure_modeling_scene_overlay(
                buffer,
                width,
                height,
                origin_x,
                origin_y,
                tile_w,
                tile_h,
                scene.id.as_str(),
                structure_stage,
            );
        }
        if let Some(environment_stage) = classic_rts_environment_life_stage(Some(runtime)) {
            classic_draw_rts_environment_life_scene_overlay(
                buffer,
                width,
                height,
                origin_x,
                origin_y,
                tile_w,
                tile_h,
                scene.id.as_str(),
                environment_stage,
            );
        }
        if let Some(harvest_stage) = classic_rts_worker_harvest_animation_stage(Some(runtime)) {
            classic_draw_rts_worker_harvest_animation_scene_overlay(
                buffer,
                width,
                height,
                origin_x,
                origin_y,
                tile_w,
                tile_h,
                scene.id.as_str(),
                harvest_stage,
            );
        }
        if let Some(production_stage) = classic_rts_production_spawn_animation_stage(Some(runtime))
        {
            classic_draw_rts_production_spawn_animation_scene_overlay(
                buffer,
                width,
                height,
                origin_x,
                origin_y,
                tile_w,
                tile_h,
                scene.id.as_str(),
                production_stage,
                runtime,
            );
        }
        classic_draw_rts_product_map_density_layer(
            buffer,
            width,
            height,
            origin_x,
            origin_y,
            tile_w,
            tile_h,
            scene.id.as_str(),
        );
        classic_draw_iso_command_feedback(
            buffer,
            width,
            height,
            assets,
            runtime,
            scene.id.as_str(),
            origin_x,
            origin_y,
            tile_w,
            tile_h,
            player_tile,
        );
    } else {
        for row in 0..8 {
            for col in 0..12 {
                let color = if (row + col) % 2 == 0 {
                    0x26352a
                } else {
                    0x314333
                };
                let (screen_x, screen_y) =
                    classic_iso_project(origin_x, origin_y, tile_w, tile_h, (col, row));
                classic_draw_iso_diamond(
                    buffer, width, height, screen_x, screen_y, tile_w, tile_h, color,
                );
            }
        }
        classic_draw_isometric_frame_at_tile(
            buffer,
            width,
            height,
            assets,
            Some(runtime),
            player_frame,
            origin_x,
            origin_y,
            tile_w,
            tile_h,
            player_tile,
            scale,
        );
    }
}

#[cfg(not(target_os = "android"))]
pub(super) fn classic_scene_rts_model_entities(scene_id: &str) -> Vec<ClassicIsoEntity> {
    let specs: &[(&str, &str, (i32, i32), i32)] = match scene_id {
        "mentor_training_room" => &[
            ("training_hall", "model_training_hall", (4, 1), 2),
            (
                "training_tree_cluster",
                "model_tree_cluster_large",
                (9, 2),
                3,
            ),
            ("training_waygate_side", "model_waygate", (8, 4), 2),
        ],
        "league_coliseum" => &[
            ("coliseum_left_stands", "model_coliseum_stands", (3, 2), 2),
            ("coliseum_right_stands", "model_coliseum_stands", (9, 3), 2),
            ("coliseum_rear_stands", "model_coliseum_stands", (6, 2), 1),
            ("coliseum_waygate", "model_waygate", (6, 1), 2),
            ("coliseum_training_hall", "model_training_hall", (8, 5), 2),
        ],
        _ => &[
            ("town_hall", "model_town_hall", (2, 2), 1),
            ("south_training_hall", "model_training_hall", (4, 5), 2),
            ("north_tree_cluster", "model_tree_cluster_large", (8, 1), 3),
            ("west_tree_cluster", "model_tree_cluster_large", (1, 4), 3),
            ("east_waygate", "model_waygate", (9, 3), 2),
        ],
    };
    specs
        .iter()
        .map(|(id, frame_id, tile, depth_offset)| ClassicIsoEntity {
            id: (*id).to_string(),
            frame_id: (*frame_id).to_string(),
            tile: *tile,
            depth_key: (tile.0 + tile.1) * 10 + depth_offset,
        })
        .collect()
}

#[cfg(not(target_os = "android"))]
pub(super) fn classic_scene_rts_doodad_entities(scene_id: &str) -> Vec<ClassicIsoEntity> {
    let specs: &[(&str, &str, (i32, i32), i32)] = match scene_id {
        "mentor_training_room" => &[
            ("training_barrels", "doodad_barrel_stack", (2, 4), 3),
            ("training_supply_barrels", "doodad_barrel_stack", (8, 6), 3),
            ("training_torch_left", "doodad_torch", (1, 2), 4),
            ("training_torch_right", "doodad_torch", (10, 2), 4),
            ("training_rocks", "doodad_rock_cluster", (8, 5), 3),
            ("training_crystal_lane", "doodad_crystal_cluster", (6, 5), 4),
        ],
        "league_coliseum" => &[
            ("arena_rocks_left", "doodad_rock_cluster", (1, 5), 3),
            ("arena_rocks_right", "doodad_rock_cluster", (10, 5), 3),
            ("arena_torch_left", "doodad_torch", (3, 1), 4),
            ("arena_torch_right", "doodad_torch", (8, 1), 4),
            ("arena_crystal", "doodad_crystal_cluster", (6, 4), 4),
            ("arena_center_gold", "doodad_gold_vein", (5, 5), 4),
            ("arena_bush_cover", "doodad_bush_cluster", (7, 6), 4),
        ],
        _ => &[
            ("square_barrels", "doodad_barrel_stack", (3, 6), 3),
            ("square_rocks", "doodad_rock_cluster", (6, 6), 3),
            ("square_torch", "doodad_torch", (5, 2), 4),
            ("square_crystal", "doodad_crystal_cluster", (10, 5), 4),
            ("square_market_gold", "doodad_gold_vein", (2, 6), 4),
            ("square_outer_bush", "doodad_bush_cluster", (11, 3), 4),
            ("square_ruins_column", "doodad_ruins_column", (6, 2), 4),
        ],
    };
    specs
        .iter()
        .map(|(id, frame_id, tile, depth_offset)| ClassicIsoEntity {
            id: (*id).to_string(),
            frame_id: (*frame_id).to_string(),
            tile: *tile,
            depth_key: (tile.0 + tile.1) * 10 + depth_offset,
        })
        .collect()
}

#[cfg(not(target_os = "android"))]
pub(super) fn classic_scene_rts_environment_entities(scene_id: &str) -> Vec<ClassicIsoEntity> {
    let specs: &[(&str, &str, (i32, i32), i32)] = match scene_id {
        "mentor_training_room" => &[
            ("training_forest_floor", "tile_forest_floor", (8, 2), 1),
            ("training_cliff_edge", "tile_cliff_edge", (9, 5), 1),
            ("training_signpost", "doodad_signpost", (3, 3), 4),
            ("training_ruins_column", "doodad_ruins_column", (7, 4), 4),
            ("training_shadow_edge", "tile_shadow_edge", (5, 6), 1),
        ],
        "league_coliseum" => &[
            ("arena_shadow_edge", "tile_shadow_edge", (5, 4), 1),
            ("arena_shadow_edge_right", "tile_shadow_edge", (7, 4), 1),
            ("arena_ruins_column", "doodad_ruins_column", (2, 3), 4),
            ("arena_gold_vein", "doodad_gold_vein", (9, 5), 4),
            ("arena_signpost", "doodad_signpost", (6, 2), 4),
            ("arena_bridge_marker", "tile_bridge", (6, 5), 1),
        ],
        _ => &[
            ("square_bridge", "tile_bridge", (7, 4), 1),
            ("square_forest_floor", "tile_forest_floor", (8, 2), 1),
            ("square_forest_floor_south", "tile_forest_floor", (9, 6), 1),
            ("square_bush_cluster", "doodad_bush_cluster", (4, 6), 4),
            ("square_gold_vein", "doodad_gold_vein", (10, 4), 4),
            ("square_cliff_edge", "tile_cliff_edge", (1, 2), 1),
        ],
    };
    specs
        .iter()
        .map(|(id, frame_id, tile, depth_offset)| ClassicIsoEntity {
            id: (*id).to_string(),
            frame_id: (*frame_id).to_string(),
            tile: *tile,
            depth_key: (tile.0 + tile.1) * 10 + depth_offset,
        })
        .collect()
}

#[cfg(not(target_os = "android"))]
pub(super) fn classic_scene_rts_neutral_unit_entities(scene_id: &str) -> Vec<ClassicIsoEntity> {
    let specs: &[(&str, &str, (i32, i32), i32)] = match scene_id {
        "mentor_training_room" => &[
            ("training_guard", "actor_guard_idle", (5, 3), 6),
            ("training_worker", "actor_worker_idle", (3, 5), 6),
            ("training_worker_carry", "actor_worker_carry", (8, 4), 6),
            ("training_creep_dummy", "actor_creep_idle", (6, 3), 6),
        ],
        "league_coliseum" => &[
            ("arena_guard_left", "actor_guard_attack", (4, 4), 6),
            ("arena_guard_right", "actor_guard_idle", (8, 4), 6),
            ("arena_worker_relay", "actor_worker_carry", (5, 5), 6),
            ("arena_creep_attack", "actor_creep_attack", (6, 5), 6),
            ("arena_creep_flank", "actor_creep_idle", (9, 4), 6),
        ],
        _ => &[
            ("square_guard_front", "actor_guard_attack", (5, 4), 6),
            ("square_guard_patrol", "actor_guard_idle", (7, 5), 6),
            ("square_worker_carry", "actor_worker_carry", (4, 5), 6),
            ("square_worker_harvest", "actor_worker_idle", (8, 5), 6),
            ("square_creep_wander", "actor_creep_idle", (9, 4), 6),
            ("square_creep_pressure", "actor_creep_attack", (10, 3), 6),
        ],
    };
    specs
        .iter()
        .map(|(id, frame_id, tile, depth_offset)| ClassicIsoEntity {
            id: (*id).to_string(),
            frame_id: (*frame_id).to_string(),
            tile: *tile,
            depth_key: (tile.0 + tile.1) * 10 + depth_offset,
        })
        .collect()
}
