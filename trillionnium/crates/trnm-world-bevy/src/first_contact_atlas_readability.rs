#![cfg(not(target_os = "android"))]

use serde_json::{json, Value};
use std::collections::BTreeSet;
use trnm_rts_data::first_contact_samples;

use crate::{
    ClassicRuntimeAssets, TRILLIONNIUM_WORLD_BEVY_CLASSIC_ASSET_PACK_CONTRACT,
    TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_ATLAS_READABILITY_CONTRACT,
};

fn string_vec<const N: usize>(items: [&str; N]) -> Vec<String> {
    items.into_iter().map(str::to_string).collect()
}

pub(crate) fn atlas_readability_guard(assets: &ClassicRuntimeAssets) -> Value {
    let samples = first_contact_samples::atlas_asset_samples();
    let family_samples = first_contact_samples::atlas_frame_family_samples();
    let atlas_runtime_depth_signatures = first_contact_samples::atlas_runtime_depth_signatures()
        .iter()
        .map(|signature| (*signature).to_string())
        .collect::<Vec<_>>();
    let sample_tiles = samples
        .iter()
        .map(|(tile, _, _, _, _)| crate::classic_rts_tile_id(*tile))
        .collect::<Vec<_>>();
    let atlas_roles = samples
        .iter()
        .map(|(_, role, _, _, _)| (*role).to_string())
        .collect::<Vec<_>>();
    let atlas_frame_ids = samples
        .iter()
        .map(|(_, _, frame_id, _, _)| (*frame_id).to_string())
        .collect::<Vec<_>>();
    let atlas_signatures = samples
        .iter()
        .map(|(_, _, _, signature, _)| (*signature).to_string())
        .collect::<Vec<_>>();
    let atlas_samples = samples
        .iter()
        .map(|(tile, role, frame_id, signature, scale)| {
            json!({
                "tile": crate::classic_rts_tile_id(*tile),
                "role": role,
                "frame_id": frame_id,
                "signature": signature,
                "scale": scale,
            })
        })
        .collect::<Vec<_>>();
    let atlas_manifest_roles = samples
        .iter()
        .map(|(_, _, frame_id, _, _)| {
            assets
                .frame_by_id
                .get(*frame_id)
                .map(|frame| frame.role.clone())
                .unwrap_or_else(|| "missing".to_string())
        })
        .collect::<Vec<_>>();
    let atlas_family_sample_tiles = family_samples
        .iter()
        .map(|(tile, _, _, _, _)| crate::classic_rts_tile_id(*tile))
        .collect::<Vec<_>>();
    let atlas_family_roles = family_samples
        .iter()
        .map(|(_, role, _, _, _)| (*role).to_string())
        .collect::<Vec<_>>();
    let atlas_family_frame_ids = family_samples
        .iter()
        .map(|(_, _, frame_id, _, _)| (*frame_id).to_string())
        .collect::<Vec<_>>();
    let atlas_family_signatures = family_samples
        .iter()
        .map(|(_, _, _, signature, _)| (*signature).to_string())
        .collect::<Vec<_>>();
    let atlas_family_samples = family_samples
        .iter()
        .map(|(tile, role, frame_id, signature, scale)| {
            json!({
                "tile": crate::classic_rts_tile_id(*tile),
                "role": role,
                "frame_id": frame_id,
                "signature": signature,
                "scale": scale,
            })
        })
        .collect::<Vec<_>>();
    let atlas_family_gallery_lanes = family_samples
        .iter()
        .map(|(tile, _, _, _, _)| {
            first_contact_samples::atlas_family_gallery_lane(*tile).to_string()
        })
        .collect::<Vec<_>>();
    let atlas_family_busy_core_tiles = family_samples
        .iter()
        .filter(|(tile, _, _, _, _)| first_contact_samples::atlas_family_busy_core_tile(*tile))
        .map(|(tile, _, _, _, _)| crate::classic_rts_tile_id(*tile))
        .collect::<Vec<_>>();
    let atlas_family_manifest_roles = family_samples
        .iter()
        .map(|(_, _, frame_id, _, _)| {
            assets
                .frame_by_id
                .get(*frame_id)
                .map(|frame| frame.role.clone())
                .unwrap_or_else(|| {
                    if assets.frame_override_pixels.contains_key(*frame_id) {
                        "override_frame".to_string()
                    } else {
                        "missing".to_string()
                    }
                })
        })
        .collect::<Vec<_>>();
    let atlas_family_override_frame_ids = atlas_family_frame_ids
        .iter()
        .filter(|frame_id| assets.frame_override_pixels.contains_key(frame_id.as_str()))
        .cloned()
        .collect::<Vec<_>>();
    let manifest_frame_gate = atlas_frame_ids
        .iter()
        .all(|frame_id| assets.frame_by_id.contains_key(frame_id));
    let family_frame_available_gate = atlas_family_frame_ids.iter().all(|frame_id| {
        assets.frame_by_id.contains_key(frame_id)
            || assets.frame_override_pixels.contains_key(frame_id)
    });
    let unique_frame_count = atlas_frame_ids.iter().collect::<BTreeSet<_>>().len();
    let unique_signature_count = atlas_signatures.iter().collect::<BTreeSet<_>>().len();
    let atlas_family_unique_frame_count =
        atlas_family_frame_ids.iter().collect::<BTreeSet<_>>().len();
    let atlas_family_unique_signature_count = atlas_family_signatures
        .iter()
        .collect::<BTreeSet<_>>()
        .len();
    let atlas_family_unique_gallery_lane_count = atlas_family_gallery_lanes
        .iter()
        .collect::<BTreeSet<_>>()
        .len();
    let north_gallery_frame_count = atlas_family_gallery_lanes
        .iter()
        .filter(|lane| lane.as_str() == "north_gallery")
        .count();
    let west_gallery_frame_count = atlas_family_gallery_lanes
        .iter()
        .filter(|lane| lane.as_str() == "west_gallery")
        .count();
    let east_gallery_frame_count = atlas_family_gallery_lanes
        .iter()
        .filter(|lane| lane.as_str() == "east_gallery")
        .count();
    let terrain_frame_count = atlas_roles
        .iter()
        .filter(|role| role.as_str() == "terrain_tile")
        .count();
    let unit_frame_count = atlas_roles
        .iter()
        .filter(|role| role.as_str() == "unit_sprite")
        .count();
    let structure_frame_count = atlas_roles
        .iter()
        .filter(|role| role.as_str() == "structure_sprite")
        .count();
    let objective_frame_count = atlas_roles
        .iter()
        .filter(|role| role.as_str() == "objective_sprite")
        .count();
    let worker_family_frame_count = atlas_family_roles
        .iter()
        .filter(|role| role.as_str() == "worker_unit_family")
        .count();
    let scout_family_frame_count = atlas_family_roles
        .iter()
        .filter(|role| role.as_str() == "scout_unit_family")
        .count();
    let warden_family_frame_count = atlas_family_roles
        .iter()
        .filter(|role| role.as_str() == "warden_unit_family")
        .count();
    let relay_unit_family_frame_count = atlas_family_roles
        .iter()
        .filter(|role| role.as_str() == "relay_unit_family")
        .count();
    let command_core_family_frame_count = atlas_family_roles
        .iter()
        .filter(|role| role.as_str() == "command_core_structure_family")
        .count();
    let relay_structure_family_frame_count = atlas_family_roles
        .iter()
        .filter(|role| role.as_str() == "relay_structure_family")
        .count();
    let beacon_family_frame_count = atlas_family_roles
        .iter()
        .filter(|role| role.as_str() == "beacon_objective_family")
        .count();
    let atlas_runtime_depth_sample_count = samples
        .iter()
        .filter(|(_, role, _, _, _)| first_contact_samples::atlas_runtime_depth_role(role))
        .count()
        + family_samples
            .iter()
            .filter(|(tile, role, _, _, _)| {
                first_contact_samples::atlas_runtime_depth_role(role)
                    && !first_contact_samples::atlas_family_lower_lane_tile(*tile)
            })
            .count();
    let atlas_lower_lane_depth_suppressed_count = family_samples
        .iter()
        .filter(|(tile, role, _, _, _)| {
            first_contact_samples::atlas_runtime_depth_role(role)
                && first_contact_samples::atlas_family_lower_lane_tile(*tile)
        })
        .count();
    let atlas_frame_pixel_budget = samples
        .iter()
        .map(|(_, _, _, _, scale)| 16_usize * 16_usize * (*scale as usize).pow(2))
        .sum::<usize>();
    let atlas_family_frame_pixel_budget = family_samples
        .iter()
        .map(|(_, _, frame_id, _, scale)| {
            if let Some(frame) = assets.frame_override_pixels.get(*frame_id) {
                (frame.width as usize) * (frame.height as usize)
            } else {
                16_usize * 16_usize * (*scale as usize).pow(2)
            }
        })
        .sum::<usize>();
    let atlas_runtime_depth_pixel_budget = atlas_runtime_depth_sample_count * 64;
    let atlas_manifest_gate = assets.manifest.contract_version
        == TRILLIONNIUM_WORLD_BEVY_CLASSIC_ASSET_PACK_CONTRACT
        && assets.atlas_parse_gate
        && assets.manifest.asset_boundary.contains("project_owned")
        && !assets.manifest.cex_runtime_player_client_allowed
        && !assets.manifest.wgpu_required;
    let terrain_atlas_frame_gate = terrain_frame_count >= 6
        && atlas_frame_ids
            .iter()
            .any(|frame_id| frame_id == "tile_stone")
        && atlas_frame_ids
            .iter()
            .any(|frame_id| frame_id == "tile_water")
        && atlas_frame_ids
            .iter()
            .any(|frame_id| frame_id == "tile_road")
        && atlas_frame_ids
            .iter()
            .any(|frame_id| frame_id == "tile_floor");
    let unit_atlas_sprite_gate = unit_frame_count >= 4
        && atlas_frame_ids
            .iter()
            .any(|frame_id| frame_id == "actor_player_walk_south_1")
        && atlas_frame_ids
            .iter()
            .any(|frame_id| frame_id == "actor_player_walk_east_1")
        && atlas_frame_ids
            .iter()
            .any(|frame_id| frame_id == "actor_enemy_attack")
        && atlas_frame_ids
            .iter()
            .any(|frame_id| frame_id == "actor_mentor_talk");
    let structure_atlas_sprite_gate = structure_frame_count >= 4
        && atlas_frame_ids
            .iter()
            .any(|frame_id| frame_id == "prop_workbench")
        && atlas_frame_ids
            .iter()
            .any(|frame_id| frame_id == "prop_market_stall")
        && atlas_frame_ids
            .iter()
            .any(|frame_id| frame_id == "prop_signpost")
        && atlas_frame_ids
            .iter()
            .any(|frame_id| frame_id == "prop_banner");
    let objective_atlas_sprite_gate = objective_frame_count >= 2
        && atlas_frame_ids
            .iter()
            .any(|frame_id| frame_id == "marker_objective")
        && atlas_frame_ids
            .iter()
            .any(|frame_id| frame_id == "marker_interaction");
    let worker_frame_family_gate = worker_family_frame_count >= 2
        && atlas_family_frame_ids
            .iter()
            .any(|frame_id| frame_id == "actor_worker_idle")
        && atlas_family_frame_ids
            .iter()
            .any(|frame_id| frame_id == "actor_worker_carry");
    let scout_frame_family_gate = scout_family_frame_count >= 2
        && atlas_family_frame_ids
            .iter()
            .any(|frame_id| frame_id == "actor_player_walk_east_1")
        && atlas_family_frame_ids
            .iter()
            .any(|frame_id| frame_id == "actor_player_walk_east_2");
    let warden_frame_family_gate = warden_family_frame_count >= 2
        && atlas_family_frame_ids
            .iter()
            .any(|frame_id| frame_id == "actor_guard_idle")
        && atlas_family_frame_ids
            .iter()
            .any(|frame_id| frame_id == "actor_guard_attack");
    let relay_unit_frame_family_gate = relay_unit_family_frame_count >= 1
        && atlas_family_frame_ids
            .iter()
            .any(|frame_id| frame_id == "actor_mentor_talk");
    let command_core_frame_family_gate = command_core_family_frame_count >= 2
        && atlas_family_frame_ids
            .iter()
            .any(|frame_id| frame_id == "model_town_hall")
        && atlas_family_frame_ids
            .iter()
            .any(|frame_id| frame_id == "model_training_hall");
    let relay_structure_frame_family_gate = relay_structure_family_frame_count >= 2
        && atlas_family_frame_ids
            .iter()
            .any(|frame_id| frame_id == "model_waygate")
        && atlas_family_frame_ids
            .iter()
            .any(|frame_id| frame_id == "prop_banner");
    let beacon_frame_family_gate = beacon_family_frame_count >= 3
        && atlas_family_frame_ids
            .iter()
            .any(|frame_id| frame_id == "marker_objective")
        && atlas_family_frame_ids
            .iter()
            .any(|frame_id| frame_id == "marker_interaction")
        && atlas_family_frame_ids
            .iter()
            .any(|frame_id| frame_id == "rts_command_destination_marker");
    let override_frame_family_gate = atlas_family_override_frame_ids.len() >= 10
        && atlas_family_override_frame_ids
            .iter()
            .any(|frame_id| frame_id == "actor_worker_carry")
        && atlas_family_override_frame_ids
            .iter()
            .any(|frame_id| frame_id == "actor_guard_attack")
        && atlas_family_override_frame_ids
            .iter()
            .any(|frame_id| frame_id == "model_town_hall")
        && atlas_family_override_frame_ids
            .iter()
            .any(|frame_id| frame_id == "model_waygate")
        && atlas_family_override_frame_ids
            .iter()
            .any(|frame_id| frame_id == "rts_command_destination_marker");
    let atlas_family_perimeter_placement_gate = atlas_family_unique_gallery_lane_count >= 3
        && atlas_family_busy_core_tiles.is_empty()
        && north_gallery_frame_count >= 4
        && west_gallery_frame_count >= 4
        && east_gallery_frame_count >= 6;
    let atlas_composition_gate = manifest_frame_gate
        && unique_frame_count >= 14
        && unique_signature_count >= 12
        && atlas_frame_pixel_budget >= 8_704;
    let atlas_frame_family_gate = family_frame_available_gate
        && atlas_family_unique_frame_count >= 14
        && atlas_family_unique_signature_count >= 14
        && atlas_family_frame_pixel_budget >= 34_000
        && worker_frame_family_gate
        && scout_frame_family_gate
        && warden_frame_family_gate
        && relay_unit_frame_family_gate
        && command_core_frame_family_gate
        && relay_structure_frame_family_gate
        && beacon_frame_family_gate
        && override_frame_family_gate
        && atlas_family_perimeter_placement_gate;
    let atlas_runtime_depth_gate = atlas_runtime_depth_signatures
        == string_vec([
            "atlas_unit_grounding_shadow",
            "atlas_structure_footprint_rim",
            "atlas_objective_capture_underlay",
            "atlas_lower_lane_depth_suppressed",
        ])
        && atlas_runtime_depth_sample_count >= 21
        && atlas_lower_lane_depth_suppressed_count == 3
        && atlas_runtime_depth_pixel_budget >= 1_344;
    let no_copy_boundary_gate =
        !assets.manifest.cex_runtime_player_client_allowed && !assets.manifest.wgpu_required;
    let first_contact_atlas_readability_gate = atlas_manifest_gate
        && terrain_atlas_frame_gate
        && unit_atlas_sprite_gate
        && structure_atlas_sprite_gate
        && objective_atlas_sprite_gate
        && atlas_composition_gate
        && atlas_frame_family_gate
        && atlas_runtime_depth_gate
        && no_copy_boundary_gate;
    let green = first_contact_atlas_readability_gate;

    json!({
        "contract_version": TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_ATLAS_READABILITY_CONTRACT,
        "green": green,
        "source_path": "trnm-world-bevy classic_draw_first_contact_atlas_readability_layer",
        "asset_pack_contract": assets.manifest.contract_version,
        "asset_boundary": assets.manifest.asset_boundary,
        "atlas_parse_gate": assets.atlas_parse_gate,
        "sample_tiles": sample_tiles,
        "atlas_roles": atlas_roles,
        "atlas_frame_ids": atlas_frame_ids,
        "atlas_manifest_roles": atlas_manifest_roles,
        "atlas_signatures": atlas_signatures,
        "atlas_samples": atlas_samples,
        "atlas_family_sample_tiles": atlas_family_sample_tiles,
        "atlas_family_roles": atlas_family_roles,
        "atlas_family_frame_ids": atlas_family_frame_ids,
        "atlas_family_manifest_roles": atlas_family_manifest_roles,
        "atlas_family_override_frame_ids": atlas_family_override_frame_ids,
        "atlas_family_signatures": atlas_family_signatures,
        "atlas_family_samples": atlas_family_samples,
        "atlas_family_gallery_lanes": atlas_family_gallery_lanes,
        "atlas_family_busy_core_tiles": atlas_family_busy_core_tiles,
        "terrain_frame_count": terrain_frame_count,
        "unit_frame_count": unit_frame_count,
        "structure_frame_count": structure_frame_count,
        "objective_frame_count": objective_frame_count,
        "worker_family_frame_count": worker_family_frame_count,
        "scout_family_frame_count": scout_family_frame_count,
        "warden_family_frame_count": warden_family_frame_count,
        "relay_unit_family_frame_count": relay_unit_family_frame_count,
        "command_core_family_frame_count": command_core_family_frame_count,
        "relay_structure_family_frame_count": relay_structure_family_frame_count,
        "beacon_family_frame_count": beacon_family_frame_count,
        "unique_frame_count": unique_frame_count,
        "unique_signature_count": unique_signature_count,
        "atlas_family_unique_frame_count": atlas_family_unique_frame_count,
        "atlas_family_unique_signature_count": atlas_family_unique_signature_count,
        "atlas_family_unique_gallery_lane_count": atlas_family_unique_gallery_lane_count,
        "north_gallery_frame_count": north_gallery_frame_count,
        "west_gallery_frame_count": west_gallery_frame_count,
        "east_gallery_frame_count": east_gallery_frame_count,
        "atlas_frame_pixel_budget": atlas_frame_pixel_budget,
        "atlas_family_frame_pixel_budget": atlas_family_frame_pixel_budget,
        "atlas_runtime_depth_source_path": "trnm-world-bevy classic_draw_first_contact_atlas_asset_sample",
        "atlas_runtime_depth_signatures": atlas_runtime_depth_signatures,
        "atlas_runtime_depth_sample_count": atlas_runtime_depth_sample_count,
        "atlas_lower_lane_depth_suppressed_count": atlas_lower_lane_depth_suppressed_count,
        "atlas_runtime_depth_pixel_budget": atlas_runtime_depth_pixel_budget,
        "manifest_frame_gate": manifest_frame_gate,
        "family_frame_available_gate": family_frame_available_gate,
        "atlas_manifest_gate": atlas_manifest_gate,
        "terrain_atlas_frame_gate": terrain_atlas_frame_gate,
        "unit_atlas_sprite_gate": unit_atlas_sprite_gate,
        "structure_atlas_sprite_gate": structure_atlas_sprite_gate,
        "objective_atlas_sprite_gate": objective_atlas_sprite_gate,
        "worker_frame_family_gate": worker_frame_family_gate,
        "scout_frame_family_gate": scout_frame_family_gate,
        "warden_frame_family_gate": warden_frame_family_gate,
        "relay_unit_frame_family_gate": relay_unit_frame_family_gate,
        "command_core_frame_family_gate": command_core_frame_family_gate,
        "relay_structure_frame_family_gate": relay_structure_frame_family_gate,
        "beacon_frame_family_gate": beacon_frame_family_gate,
        "override_frame_family_gate": override_frame_family_gate,
        "atlas_family_perimeter_placement_gate": atlas_family_perimeter_placement_gate,
        "atlas_composition_gate": atlas_composition_gate,
        "atlas_frame_family_gate": atlas_frame_family_gate,
        "atlas_runtime_depth_gate": atlas_runtime_depth_gate,
        "no_copy_boundary_gate": no_copy_boundary_gate,
        "first_contact_atlas_readability_gate": first_contact_atlas_readability_gate,
        "warcraft_iii_asset_copied": false,
        "openra_asset_copied": false,
        "third_party_asset_copied": false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_contact_atlas_readability_helpers_preserve_frame_family_contracts() {
        let assets = crate::classic_first_contact_atlas_readability_assets();
        let guard = atlas_readability_guard(&assets);

        assert_eq!(guard.get("green").and_then(Value::as_bool), Some(true));
        assert_eq!(
            guard
                .get("atlas_family_unique_gallery_lane_count")
                .and_then(Value::as_u64),
            Some(3)
        );
        assert_eq!(
            guard
                .get("west_gallery_frame_count")
                .and_then(Value::as_u64),
            Some(4)
        );
        assert_eq!(
            guard
                .get("east_gallery_frame_count")
                .and_then(Value::as_u64),
            Some(6)
        );
        assert_eq!(
            guard
                .get("north_gallery_frame_count")
                .and_then(Value::as_u64),
            Some(4)
        );
        assert_eq!(
            guard
                .get("atlas_family_frame_pixel_budget")
                .and_then(Value::as_u64),
            Some(42_496)
        );
        assert_eq!(
            guard
                .get("atlas_runtime_depth_sample_count")
                .and_then(Value::as_u64),
            Some(21)
        );
        assert_eq!(
            guard
                .get("atlas_lower_lane_depth_suppressed_count")
                .and_then(Value::as_u64),
            Some(3)
        );
        assert_eq!(
            guard
                .get("first_contact_atlas_readability_gate")
                .and_then(Value::as_bool),
            Some(true)
        );
    }
}
