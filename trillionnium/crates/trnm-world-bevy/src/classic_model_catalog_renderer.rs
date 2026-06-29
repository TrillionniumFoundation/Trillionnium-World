use super::*;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::fs;

#[cfg(not(target_os = "android"))]
pub(super) fn native_classic_model_catalog_evidence_json(catalog_path: &str) -> String {
    const CELL_WIDTH: usize = 160;
    const CELL_HEIGHT: usize = 96;
    const COLUMNS: usize = 4;
    let assets = load_classic_runtime_assets();
    let frame_count = assets.manifest.frames.len();
    let rows = frame_count.div_ceil(COLUMNS).max(1);
    let catalog_width = CELL_WIDTH * COLUMNS;
    let catalog_height = CELL_HEIGHT * rows;
    let mut catalog_pixels = vec![0x0b0d0c_u32; catalog_width * catalog_height];
    let mut rendered_frame_count = 0_usize;
    let mut frame_summaries = Vec::new();
    let mut role_counts: HashMap<String, usize> = HashMap::new();
    for (index, frame) in assets.manifest.frames.iter().enumerate() {
        *role_counts.entry(frame.role.clone()).or_default() += 1;
        let cell_x = ((index % COLUMNS) * CELL_WIDTH) as i32;
        let cell_y = ((index / COLUMNS) * CELL_HEIGHT) as i32;
        classic_draw_rect(
            &mut catalog_pixels,
            catalog_width,
            catalog_height,
            cell_x + 2,
            cell_y + 2,
            CELL_WIDTH as i32 - 4,
            CELL_HEIGHT as i32 - 4,
            0x121813,
        );
        classic_draw_rect(
            &mut catalog_pixels,
            catalog_width,
            catalog_height,
            cell_x + 2,
            cell_y + 2,
            CELL_WIDTH as i32 - 4,
            1,
            0x33483b,
        );
        classic_draw_rect(
            &mut catalog_pixels,
            catalog_width,
            catalog_height,
            cell_x + 2,
            cell_y + CELL_HEIGHT as i32 - 4,
            CELL_WIDTH as i32 - 4,
            1,
            0x33483b,
        );
        classic_draw_rect(
            &mut catalog_pixels,
            catalog_width,
            catalog_height,
            cell_x + 2,
            cell_y + 2,
            1,
            CELL_HEIGHT as i32 - 4,
            0x33483b,
        );
        classic_draw_rect(
            &mut catalog_pixels,
            catalog_width,
            catalog_height,
            cell_x + CELL_WIDTH as i32 - 4,
            cell_y + 2,
            1,
            CELL_HEIGHT as i32 - 4,
            0x33483b,
        );
        let sprite_scale = 4_u32;
        let sprite_w = frame.w as i32 * sprite_scale as i32;
        let sprite_x = cell_x + (CELL_WIDTH as i32 - sprite_w) / 2;
        let sprite_y = cell_y + 8;
        if classic_blit_frame_scaled(
            &mut catalog_pixels,
            catalog_width,
            catalog_height,
            &assets,
            &frame.id,
            sprite_x,
            sprite_y,
            sprite_scale,
        ) {
            rendered_frame_count += 1;
        }
        let label = classic_catalog_frame_label(&frame.id);
        let role_label = classic_catalog_role_label(&frame.role);
        classic_draw_text(
            &mut catalog_pixels,
            catalog_width,
            catalog_height,
            cell_x + 8,
            cell_y + 72,
            &label,
            1,
            CLASSIC_HUD_TEXT_COLOR,
        );
        classic_draw_text(
            &mut catalog_pixels,
            catalog_width,
            catalog_height,
            cell_x + 8,
            cell_y + 84,
            &role_label,
            1,
            CLASSIC_HUD_MUTED_TEXT_COLOR,
        );
        frame_summaries.push(json!({
            "id": frame.id,
            "role": frame.role,
            "label": label,
            "visible_pixel_count": classic_frame_visible_pixel_count(&assets, &frame.id),
        }));
    }
    let write_gate =
        write_classic_rgb_buffer_ppm(catalog_path, catalog_width, catalog_height, &catalog_pixels)
            .is_ok();
    let catalog_bytes = fs::metadata(catalog_path)
        .map(|metadata| metadata.len())
        .unwrap_or_default();
    let unique_color_count = catalog_pixels.iter().copied().collect::<HashSet<_>>().len();
    let non_background_pixels = catalog_pixels
        .iter()
        .filter(|color| **color != 0x0b0d0c_u32 && **color != 0x121813_u32)
        .count();
    let label_pixel_count = catalog_pixels
        .iter()
        .filter(|color| {
            **color == CLASSIC_HUD_TEXT_COLOR || **color == CLASSIC_HUD_MUTED_TEXT_COLOR
        })
        .count();
    let frame_ids = assets
        .manifest
        .frames
        .iter()
        .map(|frame| frame.id.as_str())
        .collect::<HashSet<_>>();
    let player_direction_catalog_gate = [
        "actor_player_idle_south",
        "actor_player_idle_north",
        "actor_player_idle_east",
        "actor_player_idle_west",
        "actor_player_walk_south_1",
        "actor_player_walk_south_2",
        "actor_player_walk_north_1",
        "actor_player_walk_north_2",
        "actor_player_walk_east_1",
        "actor_player_walk_east_2",
        "actor_player_walk_west_1",
        "actor_player_walk_west_2",
    ]
    .iter()
    .all(|frame_id| frame_ids.contains(frame_id));
    let actor_clip_catalog_gate = assets.manifest.actors.iter().all(|actor| {
        !actor.animation_clips.is_empty()
            && actor.animation_clips.iter().all(|clip| {
                clip.frame_ids.len() >= 2
                    && clip
                        .frame_ids
                        .iter()
                        .all(|frame_id| frame_ids.contains(frame_id.as_str()))
            })
    });
    let scene_reference_catalog_gate = assets.manifest.scenes.iter().all(|scene| {
        scene
            .tile_palette
            .iter()
            .all(|entry| frame_ids.contains(entry.frame_id.as_str()))
            && scene
                .landmarks
                .iter()
                .all(|landmark| frame_ids.contains(landmark.frame_id.as_str()))
    });
    let role_coverage_gate = [
        "terrain_tile",
        "player_actor",
        "npc_actor",
        "enemy_actor",
        "scene_prop",
        "objective_marker",
        "interaction_marker",
    ]
    .iter()
    .all(|role| role_counts.contains_key(*role));
    let all_frames_rendered_gate = rendered_frame_count == frame_count
        && frame_summaries.iter().all(|summary| {
            summary
                .get("visible_pixel_count")
                .and_then(|value| value.as_u64())
                .unwrap_or_default()
                > 12
        });
    let catalog_sheet_gate =
        catalog_bytes > 100_000 && unique_color_count >= 32 && non_background_pixels > 40_000;
    let label_gate = label_pixel_count > 2_500;
    let green = write_gate
        && assets.loaded_from_manifest
        && assets.atlas_parse_gate
        && frame_count >= 43
        && rendered_frame_count == frame_count
        && catalog_sheet_gate
        && label_gate
        && all_frames_rendered_gate
        && player_direction_catalog_gate
        && actor_clip_catalog_gate
        && scene_reference_catalog_gate
        && role_coverage_gate
        && !assets.manifest.cex_runtime_player_client_allowed
        && !assets.manifest.wgpu_required;
    serde_json::to_string_pretty(&json!({
        "contract_version": TRILLIONNIUM_WORLD_BEVY_CLASSIC_MODEL_CATALOG_CONTRACT,
        "green": green,
        "catalog_path": catalog_path,
        "catalog_format": "ppm_p3_rgb",
        "catalog_width": catalog_width,
        "catalog_height": catalog_height,
        "catalog_bytes": catalog_bytes,
        "cell_width": CELL_WIDTH,
        "cell_height": CELL_HEIGHT,
        "columns": COLUMNS,
        "frame_count": frame_count,
        "rendered_frame_count": rendered_frame_count,
        "unique_color_count": unique_color_count,
        "non_background_pixels": non_background_pixels,
        "label_pixel_count": label_pixel_count,
        "loaded_from_manifest": assets.loaded_from_manifest,
        "atlas_parse_gate": assets.atlas_parse_gate,
        "catalog_sheet_gate": catalog_sheet_gate,
        "label_gate": label_gate,
        "all_frames_rendered_gate": all_frames_rendered_gate,
        "player_direction_catalog_gate": player_direction_catalog_gate,
        "actor_clip_catalog_gate": actor_clip_catalog_gate,
        "scene_reference_catalog_gate": scene_reference_catalog_gate,
        "role_coverage_gate": role_coverage_gate,
        "role_counts": role_counts,
        "frame_summaries": frame_summaries,
        "cex_runtime_player_client_allowed": assets.manifest.cex_runtime_player_client_allowed,
        "wgpu_required": assets.manifest.wgpu_required,
        "source_of_truth": "The classic model catalog renders every project-owned manifest frame through the same PPM atlas blitter used by the native low-spec playtest renderer."
    }))
    .expect("classic model catalog evidence serializes")
}

#[cfg(not(target_os = "android"))]
pub(super) fn classic_frame_visible_pixel_count(
    assets: &ClassicRuntimeAssets,
    frame_id: &str,
) -> usize {
    let Some(frame) = assets.frame_by_id.get(frame_id) else {
        return 0;
    };
    let mut count = 0_usize;
    for y in frame.y..frame.y + frame.h {
        for x in frame.x..frame.x + frame.w {
            let sx = x - frame.x;
            let sy = y - frame.y;
            if classic_frame_source_pixel(assets, frame, sx, sy) != 0x000000 {
                count += 1;
            }
        }
    }
    count
}

pub(super) fn classic_frame_source_pixel(
    assets: &ClassicRuntimeAssets,
    frame: &ClassicAtlasFrame,
    sx: u32,
    sy: u32,
) -> u32 {
    if let Some(frame_override) = assets.frame_override_pixels.get(&frame.id) {
        if sx >= frame_override.width || sy >= frame_override.height {
            return 0x000000;
        }
        let index = sy as usize * frame_override.width as usize + sx as usize;
        return frame_override
            .pixels
            .get(index)
            .copied()
            .unwrap_or_default();
    }
    let source_x = frame.x + sx;
    let source_y = frame.y + sy;
    if source_x >= assets.manifest.atlas_width || source_y >= assets.manifest.atlas_height {
        return 0x000000;
    }
    let index = source_y as usize * assets.manifest.atlas_width as usize + source_x as usize;
    assets.atlas_pixels.get(index).copied().unwrap_or_default()
}

#[cfg(not(target_os = "android"))]
pub(super) fn classic_catalog_frame_label(frame_id: &str) -> String {
    let label = if let Some(rest) = frame_id.strip_prefix("actor_player_") {
        format!("P {}", rest)
    } else if let Some(rest) = frame_id.strip_prefix("actor_mentor") {
        format!("MENTOR {}", rest)
    } else if let Some(rest) = frame_id.strip_prefix("actor_enemy") {
        format!("ENEMY {}", rest)
    } else if let Some(rest) = frame_id.strip_prefix("actor_vendor") {
        format!("VENDOR {}", rest)
    } else if let Some(rest) = frame_id.strip_prefix("prop_") {
        format!("PROP {}", rest)
    } else if let Some(rest) = frame_id.strip_prefix("tile_") {
        format!("TILE {}", rest)
    } else if let Some(rest) = frame_id.strip_prefix("marker_") {
        format!("MARK {}", rest)
    } else {
        frame_id.to_string()
    };
    classic_catalog_text_label(&label, 24)
}

#[cfg(not(target_os = "android"))]
pub(super) fn classic_catalog_role_label(role: &str) -> String {
    classic_catalog_text_label(role, 20)
}

pub(super) fn classic_catalog_text_label(text: &str, max_chars: usize) -> String {
    text.replace('_', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_uppercase()
        .chars()
        .take(max_chars)
        .collect()
}
