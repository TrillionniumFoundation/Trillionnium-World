use super::*;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::fs;

pub(super) fn native_classic_asset_slot_map_evidence_json() -> String {
    let assets = load_classic_runtime_assets();
    let frame_ids = assets
        .manifest
        .frames
        .iter()
        .map(|frame| frame.id.clone())
        .collect::<HashSet<_>>();
    let mut slots: Vec<(String, String, String, String, u32)> = assets
        .manifest
        .frames
        .iter()
        .map(|frame| {
            let category = if frame.id.starts_with("tile_") {
                "terrain"
            } else if frame.id.starts_with("actor_") {
                "unit"
            } else if frame.id.starts_with("prop_") {
                "prop"
            } else if frame.id.starts_with("marker_") {
                "marker"
            } else {
                "unknown"
            };
            (
                format!("manifest_{}", frame.id),
                category.to_string(),
                "manifest_frame".to_string(),
                frame.id.clone(),
                assets.manifest.source_tile_size_px,
            )
        })
        .collect();

    let procedural_model_slot_ids = [
        ("model_town_hall", 96_u32),
        ("model_training_hall", 96_u32),
        ("model_waygate", 96_u32),
        ("model_coliseum_stands", 128_u32),
        ("model_tree_cluster_large", 96_u32),
    ];
    let doodad_slot_ids = [
        ("doodad_rock_cluster", 48_u32),
        ("doodad_barrel_stack", 48_u32),
        ("doodad_torch", 48_u32),
        ("doodad_crystal_cluster", 48_u32),
        ("doodad_bush_cluster", 48_u32),
        ("doodad_ruins_column", 56_u32),
        ("doodad_gold_vein", 48_u32),
        ("doodad_signpost", 48_u32),
    ];
    let terrain_detail_slot_ids = [
        ("tile_cliff_edge", 48_u32),
        ("tile_bridge", 48_u32),
        ("tile_forest_floor", 48_u32),
        ("tile_shadow_edge", 48_u32),
    ];
    let vfx_slot_ids = [
        ("rts_command_destination_marker", 48_u32),
        ("combat_attack_arc", 64_u32),
        ("combat_hit_flash", 48_u32),
        ("rts_unit_selection_ring", 48_u32),
        ("unit_health_bar", 32_u32),
        ("rts_foundation_shadow", 96_u32),
    ];
    let neutral_unit_slot_ids = [
        ("actor_guard_idle", 32_u32),
        ("actor_guard_attack", 40_u32),
        ("actor_worker_idle", 32_u32),
        ("actor_worker_carry", 40_u32),
        ("actor_creep_idle", 36_u32),
        ("actor_creep_attack", 44_u32),
    ];
    for (slot_id, target_px) in procedural_model_slot_ids.iter() {
        slots.push((
            (*slot_id).to_string(),
            "building".to_string(),
            "procedural_isometric_model".to_string(),
            (*slot_id).to_string(),
            *target_px,
        ));
    }
    for (slot_id, target_px) in doodad_slot_ids.iter() {
        slots.push((
            (*slot_id).to_string(),
            "doodad".to_string(),
            "procedural_doodad".to_string(),
            (*slot_id).to_string(),
            *target_px,
        ));
    }
    for (slot_id, target_px) in terrain_detail_slot_ids.iter() {
        slots.push((
            (*slot_id).to_string(),
            "terrain_detail".to_string(),
            "procedural_terrain_detail".to_string(),
            (*slot_id).to_string(),
            *target_px,
        ));
    }
    for (slot_id, target_px) in vfx_slot_ids.iter() {
        slots.push((
            (*slot_id).to_string(),
            "vfx_ui".to_string(),
            "procedural_vfx_ui".to_string(),
            (*slot_id).to_string(),
            *target_px,
        ));
    }
    for (slot_id, target_px) in neutral_unit_slot_ids.iter() {
        slots.push((
            (*slot_id).to_string(),
            "unit".to_string(),
            "procedural_neutral_unit".to_string(),
            (*slot_id).to_string(),
            *target_px,
        ));
    }

    let mut category_counts: HashMap<String, usize> = HashMap::new();
    let mut backing_counts: HashMap<String, usize> = HashMap::new();
    for (_, category, backing_kind, _, _) in &slots {
        *category_counts.entry(category.clone()).or_default() += 1;
        *backing_counts.entry(backing_kind.clone()).or_default() += 1;
    }
    let required_categories = [
        "terrain",
        "terrain_detail",
        "unit",
        "prop",
        "marker",
        "building",
        "doodad",
        "vfx_ui",
    ];
    let required_categories_present_gate = required_categories
        .iter()
        .all(|category| category_counts.get(*category).copied().unwrap_or_default() > 0);
    let manifest_frame_slot_count = backing_counts
        .get("manifest_frame")
        .copied()
        .unwrap_or_default();
    let procedural_model_slot_count = backing_counts
        .get("procedural_isometric_model")
        .copied()
        .unwrap_or_default();
    let doodad_slot_count = backing_counts
        .get("procedural_doodad")
        .copied()
        .unwrap_or_default();
    let vfx_slot_count = backing_counts
        .get("procedural_vfx_ui")
        .copied()
        .unwrap_or_default();
    let terrain_detail_slot_count = backing_counts
        .get("procedural_terrain_detail")
        .copied()
        .unwrap_or_default();
    let neutral_unit_slot_count = backing_counts
        .get("procedural_neutral_unit")
        .copied()
        .unwrap_or_default();
    let manifest_frame_slots_gate = manifest_frame_slot_count == assets.manifest.frames.len()
        && manifest_frame_slot_count >= 43
        && slots
            .iter()
            .filter(|(_, _, backing_kind, _, _)| backing_kind == "manifest_frame")
            .all(|(_, _, _, target_id, _)| frame_ids.contains(target_id));
    let procedural_slot_targets = slots
        .iter()
        .filter(|(_, _, backing_kind, _, _)| backing_kind.starts_with("procedural_"))
        .map(|(_, _, _, target_id, _)| target_id.as_str())
        .collect::<HashSet<_>>();
    let procedural_slots_gate = procedural_model_slot_ids
        .iter()
        .all(|(slot_id, _)| procedural_slot_targets.contains(*slot_id))
        && doodad_slot_ids
            .iter()
            .all(|(slot_id, _)| procedural_slot_targets.contains(*slot_id))
        && terrain_detail_slot_ids
            .iter()
            .all(|(slot_id, _)| procedural_slot_targets.contains(*slot_id))
        && vfx_slot_ids
            .iter()
            .all(|(slot_id, _)| procedural_slot_targets.contains(*slot_id))
        && neutral_unit_slot_ids
            .iter()
            .all(|(slot_id, _)| procedural_slot_targets.contains(*slot_id))
        && procedural_model_slot_count >= 5
        && doodad_slot_count >= 8
        && terrain_detail_slot_count >= 4
        && vfx_slot_count >= 6
        && neutral_unit_slot_count >= 6;
    let replacement_boundary_gate = assets.manifest.x230_low_spec_renderer_target
        && assets.manifest.asset_boundary.contains("not_cex_runtime")
        && !assets.manifest.cex_runtime_player_client_allowed
        && !assets.manifest.wgpu_required;
    let slot_count = slots.len();
    let category_count = category_counts.len();
    let green = assets.loaded_from_manifest
        && assets.atlas_parse_gate
        && slot_count >= 66
        && category_count >= 8
        && required_categories_present_gate
        && manifest_frame_slots_gate
        && procedural_slots_gate
        && replacement_boundary_gate;
    let slot_records = slots
        .iter()
        .map(|(slot_id, category, backing_kind, target_id, target_px)| {
            json!({
                "slot_id": slot_id,
                "category": category,
                "backing_kind": backing_kind,
                "target_id": target_id,
                "target_px": target_px,
                "replacement_rule": "replace_this_slot_in_trnm_world_bevy_assets_only",
            })
        })
        .collect::<Vec<_>>();
    serde_json::to_string_pretty(&json!({
        "contract_version": TRILLIONNIUM_WORLD_BEVY_CLASSIC_ASSET_SLOT_MAP_CONTRACT,
        "green": green,
        "slot_count": slot_count,
        "category_count": category_count,
        "category_counts": category_counts,
        "backing_counts": backing_counts,
        "manifest_frame_slot_count": manifest_frame_slot_count,
        "procedural_model_slot_count": procedural_model_slot_count,
        "doodad_slot_count": doodad_slot_count,
        "terrain_detail_slot_count": terrain_detail_slot_count,
        "vfx_slot_count": vfx_slot_count,
        "neutral_unit_slot_count": neutral_unit_slot_count,
        "required_categories_present_gate": required_categories_present_gate,
        "manifest_frame_slots_gate": manifest_frame_slots_gate,
        "procedural_slots_gate": procedural_slots_gate,
        "replacement_boundary_gate": replacement_boundary_gate,
        "loaded_from_manifest": assets.loaded_from_manifest,
        "atlas_parse_gate": assets.atlas_parse_gate,
        "asset_boundary": assets.manifest.asset_boundary,
        "x230_low_spec_renderer_target": assets.manifest.x230_low_spec_renderer_target,
        "cex_runtime_player_client_allowed": assets.manifest.cex_runtime_player_client_allowed,
        "wgpu_required": assets.manifest.wgpu_required,
        "future_real_asset_contract": "Stable slot ids can replace procedural PPM art with real 2.5D sprites/models inside trnm-world-bevy without moving account, renderer, or runtime ownership back to a legacy client or wgpu.",
        "slots": slot_records,
        "source_of_truth": "The classic asset slot map is a Bevy-owned replacement boundary for manifest frames, procedural RTS buildings, doodads, and command/combat VFX used by the native low-spec client."
    }))
    .expect("classic asset slot map evidence serializes")
}

pub(super) fn native_classic_asset_override_probe_evidence_json(preview_path: &str) -> String {
    const WIDTH: usize = 96;
    const HEIGHT: usize = 96;
    const OVERRIDE_FRAME_ID: &str = "actor_player_idle_south";
    const OVERRIDE_PROBE_COLOR: u32 = 0xff00ff;
    let assets = load_classic_runtime_assets();
    let mut preview_pixels = vec![0x0b0d0c_u32; WIDTH * HEIGHT];
    let draw_gate = classic_blit_frame_scaled(
        &mut preview_pixels,
        WIDTH,
        HEIGHT,
        &assets,
        OVERRIDE_FRAME_ID,
        16,
        16,
        4,
    );
    let write_gate =
        write_classic_rgb_buffer_ppm(preview_path, WIDTH, HEIGHT, &preview_pixels).is_ok();
    let preview_bytes = fs::metadata(preview_path)
        .map(|metadata| metadata.len())
        .unwrap_or_default();
    let override_frame_ids = assets
        .frame_override_pixels
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    let override_frame_gate = assets.frame_override_pixels.contains_key(OVERRIDE_FRAME_ID);
    let override_probe_pixel_count = preview_pixels
        .iter()
        .filter(|color| **color == OVERRIDE_PROBE_COLOR)
        .count();
    let non_background_pixels = preview_pixels
        .iter()
        .filter(|color| **color != 0x0b0d0c_u32)
        .count();
    let replacement_boundary_gate = assets.manifest.x230_low_spec_renderer_target
        && assets.manifest.asset_boundary.contains("not_cex_runtime")
        && !assets.manifest.cex_runtime_player_client_allowed
        && !assets.manifest.wgpu_required;
    let green = write_gate
        && draw_gate
        && assets.loaded_from_manifest
        && assets.atlas_parse_gate
        && override_frame_gate
        && override_probe_pixel_count > 300
        && non_background_pixels > 300
        && preview_bytes > 1_000
        && replacement_boundary_gate;
    serde_json::to_string_pretty(&json!({
        "contract_version": TRILLIONNIUM_WORLD_BEVY_CLASSIC_ASSET_OVERRIDE_PROBE_CONTRACT,
        "green": green,
        "preview_path": preview_path,
        "preview_format": "ppm_p3_rgb",
        "preview_width": WIDTH,
        "preview_height": HEIGHT,
        "preview_bytes": preview_bytes,
        "override_dir": assets.frame_override_dir.clone().unwrap_or_default(),
        "override_frame_id": OVERRIDE_FRAME_ID,
        "override_frame_ids": override_frame_ids,
        "override_frame_count": assets.frame_override_pixels.len(),
        "override_frame_gate": override_frame_gate,
        "override_probe_color": "ff00ff",
        "override_probe_pixel_count": override_probe_pixel_count,
        "non_background_pixels": non_background_pixels,
        "draw_gate": draw_gate,
        "write_gate": write_gate,
        "loaded_from_manifest": assets.loaded_from_manifest,
        "atlas_parse_gate": assets.atlas_parse_gate,
        "replacement_boundary_gate": replacement_boundary_gate,
        "asset_boundary": assets.manifest.asset_boundary,
        "x230_low_spec_renderer_target": assets.manifest.x230_low_spec_renderer_target,
        "cex_runtime_player_client_allowed": assets.manifest.cex_runtime_player_client_allowed,
        "wgpu_required": assets.manifest.wgpu_required,
        "source_of_truth": "The classic asset override probe proves project-local PPM frame overrides are consumed by the same trnm-world-bevy low-spec blitter used by the native playtest renderer."
    }))
    .expect("classic asset override probe evidence serializes")
}
