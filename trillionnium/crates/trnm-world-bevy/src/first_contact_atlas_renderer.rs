#![cfg(not(target_os = "android"))]

use crate::{
    classic_blit_frame_override_bottom_center, classic_blit_frame_scaled, classic_darken,
    classic_draw_iso_ellipse, classic_draw_rect, classic_first_contact_tile_screen,
    classic_mix_color, first_contact_palette, first_contact_renderer_readability,
    ClassicRuntimeAssets, CLASSIC_RTS_COMMAND_SURFACE_TARGET_COLOR,
    CLASSIC_RTS_STRUCTURE_FOUNDATION_SHADOW_COLOR, CLASSIC_RTS_TACTICAL_VIEWPORT_SHADOW_COLOR,
    CLASSIC_RTS_TACTICAL_VIEWPORT_TILE_COLOR,
};
use trnm_rts_data::first_contact_samples;

pub(super) fn asset_samples() -> Vec<((i32, i32), &'static str, &'static str, &'static str, u32)> {
    first_contact_samples::atlas_asset_samples()
}

pub(super) fn frame_family_samples(
) -> Vec<((i32, i32), &'static str, &'static str, &'static str, u32)> {
    first_contact_samples::atlas_frame_family_samples()
}

pub(super) fn family_gallery_lane(tile: (i32, i32)) -> &'static str {
    first_contact_samples::atlas_family_gallery_lane(tile)
}

pub(super) fn family_lower_lane_tile(tile: (i32, i32)) -> bool {
    first_contact_samples::atlas_family_lower_lane_tile(tile)
}

pub(super) fn family_slot_color(role: &str, tile: (i32, i32)) -> u32 {
    first_contact_palette::atlas_family_slot_color(role, family_lower_lane_tile(tile))
}

#[allow(clippy::too_many_arguments)]
fn mute_gallery_pixels(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    lower_lane: bool,
) -> usize {
    let mut muted_pixels = 0_usize;
    for py in y.max(0)..(y + h).min(height as i32) {
        for px in x.max(0)..(x + w).min(width as i32) {
            let index = py as usize * width + px as usize;
            let color = buffer[index];
            if color == 0x000000 || color == CLASSIC_RTS_TACTICAL_VIEWPORT_TILE_COLOR {
                continue;
            }
            buffer[index] = if lower_lane {
                classic_mix_color(
                    color,
                    0x020604,
                    trnm_rts_evidence::TRNM_RTS_EVIDENCE_FIRST_CONTACT_LOWER_LANE_GALLERY_DARKEN_NUMERATOR,
                    trnm_rts_evidence::TRNM_RTS_EVIDENCE_FIRST_CONTACT_LOWER_LANE_GALLERY_DARKEN_DENOMINATOR,
                )
            } else {
                classic_mix_color(
                    color,
                    0x06100c,
                    trnm_rts_evidence::TRNM_RTS_EVIDENCE_FIRST_CONTACT_GALLERY_DARKEN_NUMERATOR,
                    trnm_rts_evidence::TRNM_RTS_EVIDENCE_FIRST_CONTACT_GALLERY_DARKEN_DENOMINATOR,
                )
            };
            muted_pixels += 1;
        }
    }
    muted_pixels
}

fn asset_offset(role: &str, frame_px: i32, cell_h: i32) -> (i32, i32) {
    match role {
        "terrain_tile" => (-frame_px / 2, -frame_px / 2),
        "unit_sprite" | "worker_unit_family" | "scout_unit_family" | "warden_unit_family"
        | "relay_unit_family" => (-frame_px / 2, -cell_h - frame_px + 6),
        "structure_sprite" | "command_core_structure_family" | "relay_structure_family" => {
            (-frame_px / 2, -cell_h * 2 - frame_px / 2)
        }
        "objective_sprite" | "beacon_objective_family" => {
            (-frame_px / 2, -cell_h * 2 - frame_px / 2)
        }
        _ => (-frame_px / 2, -frame_px / 2),
    }
}

pub(super) fn asset_frame_size(
    assets: &ClassicRuntimeAssets,
    frame_id: &str,
    scale: u32,
) -> (i32, i32) {
    if let Some(frame) = assets.frame_override_pixels.get(frame_id) {
        return (frame.width as i32, frame.height as i32);
    }
    let scale = scale.max(1) as i32;
    (
        assets.manifest.source_tile_size_px as i32 * scale,
        assets.manifest.source_tile_size_px as i32 * scale,
    )
}

fn runtime_depth_role(role: &str) -> bool {
    first_contact_samples::atlas_runtime_depth_role(role)
}

#[allow(clippy::too_many_arguments)]
fn draw_runtime_depth(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    role: &str,
    cx: i32,
    cy: i32,
    cell_w: i32,
    cell_h: i32,
    lower_lane_gallery: bool,
) {
    if lower_lane_gallery || !runtime_depth_role(role) {
        return;
    }

    match role {
        "unit_sprite" | "worker_unit_family" | "scout_unit_family" | "warden_unit_family"
        | "relay_unit_family" => {
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                cx,
                cy + cell_h / 2 + 2,
                (cell_w / 2 + 5).max(8),
                (cell_h / 4 + 2).max(4),
                CLASSIC_RTS_TACTICAL_VIEWPORT_SHADOW_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                cx - cell_w / 3,
                cy + cell_h / 2 + 2,
                (cell_w * 2 / 3).max(8),
                2,
                classic_darken(CLASSIC_RTS_TACTICAL_VIEWPORT_SHADOW_COLOR, 1, 4),
            );
        }
        "structure_sprite" | "command_core_structure_family" | "relay_structure_family" => {
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                cx,
                cy + cell_h / 2 + 3,
                (cell_w + 6).max(14),
                (cell_h / 3 + 3).max(5),
                CLASSIC_RTS_TACTICAL_VIEWPORT_SHADOW_COLOR,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                cx - cell_w,
                cy + cell_h / 2 + 2,
                cell_w * 2,
                3,
                classic_darken(CLASSIC_RTS_STRUCTURE_FOUNDATION_SHADOW_COLOR, 1, 4),
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                cx - cell_w + 4,
                cy + cell_h / 2 - 2,
                cell_w * 2 - 8,
                2,
                CLASSIC_RTS_STRUCTURE_FOUNDATION_SHADOW_COLOR,
            );
        }
        "objective_sprite" | "beacon_objective_family" => {
            let underlay = classic_darken(CLASSIC_RTS_COMMAND_SURFACE_TARGET_COLOR, 1, 6);
            classic_draw_iso_ellipse(
                buffer,
                width,
                height,
                cx,
                cy + cell_h / 2 + 2,
                (cell_w + 4).max(12),
                (cell_h / 3 + 2).max(4),
                classic_darken(underlay, 1, 3),
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                cx - cell_w / 2,
                cy + cell_h / 2 + 1,
                cell_w,
                2,
                underlay,
            );
            classic_draw_rect(
                buffer,
                width,
                height,
                cx - 2,
                cy,
                4,
                cell_h / 2,
                classic_darken(underlay, 1, 4),
            );
        }
        _ => {}
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_asset_sample(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    assets: &ClassicRuntimeAssets,
    map_x: i32,
    map_y: i32,
    cell_w: i32,
    cell_h: i32,
    tile: (i32, i32),
    role: &str,
    frame_id: &str,
    scale: u32,
    muted_gallery: bool,
) -> bool {
    let (tile_x, tile_y) = classic_first_contact_tile_screen(map_x, map_y, cell_w, cell_h, tile);
    let cx = tile_x + cell_w / 2;
    let cy = tile_y + cell_h / 2;
    let lower_lane_gallery = muted_gallery && family_lower_lane_tile(tile);
    let (frame_w, frame_h) = asset_frame_size(assets, frame_id, scale);
    let (offset_x, offset_y) = asset_offset(role, frame_h.max(frame_w), cell_h);
    if runtime_depth_role(role) {
        draw_runtime_depth(
            buffer,
            width,
            height,
            role,
            cx,
            cy,
            cell_w,
            cell_h,
            lower_lane_gallery,
        );
    }
    if first_contact_renderer_readability::secondary_objective_atlas_asset(tile, role, frame_id) {
        first_contact_renderer_readability::draw_secondary_objective_atlas_anchor(
            buffer, width, height, cx, cy, cell_w, cell_h,
        );
        return true;
    }
    let has_override_frame = assets.frame_override_pixels.contains_key(frame_id);
    let blit_x = if has_override_frame {
        cx - frame_w / 2
    } else {
        cx + offset_x.min(-frame_w / 2)
    };
    let blit_y = cy + offset_y;
    let drawn = if has_override_frame {
        classic_blit_frame_override_bottom_center(
            buffer,
            width,
            height,
            assets,
            frame_id,
            cx,
            cy + offset_y + frame_h,
        )
    } else {
        classic_blit_frame_scaled(
            buffer, width, height, assets, frame_id, blit_x, blit_y, scale,
        )
    };
    if drawn && muted_gallery {
        mute_gallery_pixels(
            buffer,
            width,
            height,
            blit_x - 1,
            blit_y - 1,
            frame_w + 2,
            frame_h + 2,
            lower_lane_gallery,
        );
    }
    drawn
}

#[allow(clippy::too_many_arguments)]
fn draw_family_slot_cue(
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
    let lower_lane = family_lower_lane_tile(tile);
    let color = family_slot_color(role, tile);
    if lower_lane {
        let anchor_color = classic_darken(color, 1, 5);
        classic_draw_rect(
            buffer,
            width,
            height,
            tile_x + cell_w / 2,
            tile_y + cell_h - 3,
            1,
            1,
            classic_darken(anchor_color, 1, 4),
        );
        return;
    }

    let lane = family_gallery_lane(tile);
    let anchor_color = classic_darken(color, 1, 4);
    match lane {
        "west_gallery" => classic_draw_rect(
            buffer,
            width,
            height,
            tile_x + 1,
            tile_y + cell_h / 2,
            1,
            1,
            anchor_color,
        ),
        "east_gallery" => classic_draw_rect(
            buffer,
            width,
            height,
            tile_x + cell_w - 2,
            tile_y + cell_h / 2,
            1,
            1,
            anchor_color,
        ),
        _ => classic_draw_rect(
            buffer,
            width,
            height,
            tile_x + cell_w / 2,
            tile_y + 1,
            1,
            1,
            anchor_color,
        ),
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn draw_readability_layer(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    assets: &ClassicRuntimeAssets,
    map_x: i32,
    map_y: i32,
    cell_w: i32,
    cell_h: i32,
) {
    for (tile, role, frame_id, _, scale) in asset_samples() {
        draw_asset_sample(
            buffer, width, height, assets, map_x, map_y, cell_w, cell_h, tile, role, frame_id,
            scale, false,
        );
    }
    for (tile, role, frame_id, _, scale) in frame_family_samples() {
        draw_family_slot_cue(
            buffer, width, height, map_x, map_y, cell_w, cell_h, tile, role,
        );
        if family_lower_lane_tile(tile) {
            continue;
        }
        draw_asset_sample(
            buffer, width, height, assets, map_x, map_y, cell_w, cell_h, tile, role, frame_id,
            scale, true,
        );
    }
}
