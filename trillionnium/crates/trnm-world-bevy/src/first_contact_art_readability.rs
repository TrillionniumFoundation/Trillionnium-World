#![cfg(not(target_os = "android"))]

use serde_json::{json, Value};
use std::collections::BTreeSet;

use crate::{
    classic_rts_tile_id, first_contact_samples,
    TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_ART_READABILITY_CONTRACT,
};

fn string_vec<const N: usize>(values: [&str; N]) -> Vec<String> {
    values.into_iter().map(str::to_string).collect()
}

pub(crate) fn art_readability_guard() -> Value {
    let terrain_samples = first_contact_samples::art_terrain_samples();
    let building_samples = first_contact_samples::art_building_samples();
    let landmark_samples = first_contact_samples::art_landmark_samples();
    let runtime_actor_depth_signatures = first_contact_samples::runtime_actor_depth_signatures()
        .iter()
        .map(|signature| (*signature).to_string())
        .collect::<Vec<_>>();
    let terrain_material_depth_signatures =
        first_contact_samples::terrain_material_depth_signatures()
            .iter()
            .map(|signature| (*signature).to_string())
            .collect::<Vec<_>>();
    let terrain_sample_tiles = terrain_samples
        .iter()
        .map(|(tile, _, _)| classic_rts_tile_id(*tile))
        .collect::<Vec<_>>();
    let terrain_material_roles = terrain_samples
        .iter()
        .map(|(_, role, _)| (*role).to_string())
        .collect::<Vec<_>>();
    let terrain_material_signatures = terrain_samples
        .iter()
        .map(|(_, _, signature)| (*signature).to_string())
        .collect::<Vec<_>>();
    let building_sample_tiles = building_samples
        .iter()
        .map(|(tile, _, _)| classic_rts_tile_id(*tile))
        .collect::<Vec<_>>();
    let building_roles = building_samples
        .iter()
        .map(|(_, role, _)| (*role).to_string())
        .collect::<Vec<_>>();
    let building_facade_signatures = building_samples
        .iter()
        .map(|(_, _, signature)| (*signature).to_string())
        .collect::<Vec<_>>();
    let map_landmark_sample_tiles = landmark_samples
        .iter()
        .map(|(tile, _, _)| classic_rts_tile_id(*tile))
        .collect::<Vec<_>>();
    let map_landmark_roles = landmark_samples
        .iter()
        .map(|(_, role, _)| (*role).to_string())
        .collect::<Vec<_>>();
    let map_landmark_signatures = landmark_samples
        .iter()
        .map(|(_, _, signature)| (*signature).to_string())
        .collect::<Vec<_>>();
    let terrain_sample_objects = terrain_samples
        .iter()
        .map(|(tile, role, signature)| {
            json!({
                "tile": classic_rts_tile_id(*tile),
                "role": role,
                "signature": signature,
            })
        })
        .collect::<Vec<_>>();
    let building_sample_objects = building_samples
        .iter()
        .map(|(tile, role, signature)| {
            json!({
                "tile": classic_rts_tile_id(*tile),
                "role": role,
                "signature": signature,
            })
        })
        .collect::<Vec<_>>();
    let map_landmark_sample_objects = landmark_samples
        .iter()
        .map(|(tile, role, signature)| {
            json!({
                "tile": classic_rts_tile_id(*tile),
                "role": role,
                "signature": signature,
            })
        })
        .collect::<Vec<_>>();
    let unique_terrain_signature_count = terrain_material_signatures
        .iter()
        .collect::<BTreeSet<_>>()
        .len();
    let unique_building_signature_count = building_facade_signatures
        .iter()
        .collect::<BTreeSet<_>>()
        .len();
    let unique_landmark_signature_count = map_landmark_signatures
        .iter()
        .collect::<BTreeSet<_>>()
        .len();
    let command_core_count = building_roles
        .iter()
        .filter(|role| role.as_str() == "command_core")
        .count();
    let relay_count = building_roles
        .iter()
        .filter(|role| role.as_str() == "relay")
        .count();
    let beacon_count = building_roles
        .iter()
        .filter(|role| role.as_str() == "beacon")
        .count();
    let base_landmark_count = map_landmark_roles
        .iter()
        .filter(|role| role.as_str() == "base_gate")
        .count();
    let resource_landmark_count = map_landmark_roles
        .iter()
        .filter(|role| role.as_str() == "resource_cluster")
        .count();
    let lane_landmark_count = map_landmark_roles
        .iter()
        .filter(|role| role.as_str() == "beacon_lane")
        .count();
    let basin_landmark_count = map_landmark_roles
        .iter()
        .filter(|role| role.as_str() == "basin_scar")
        .count();
    let relay_landmark_count = map_landmark_roles
        .iter()
        .filter(|role| role.as_str() == "relay_cable")
        .count();
    let beacon_landmark_count = map_landmark_roles
        .iter()
        .filter(|role| role.as_str() == "beacon_ring")
        .count();
    let terrain_material_pixel_budget = terrain_samples.len() * 48;
    let terrain_material_depth_sample_count = terrain_samples.len();
    let terrain_material_depth_pixel_budget = terrain_material_depth_sample_count * 64;
    let building_facade_pixel_budget = building_samples.len() * 86;
    let map_landmark_pixel_budget = landmark_samples.len() * 72;
    let runtime_actor_depth_pixel_budget = runtime_actor_depth_signatures.len() * 96;
    let terrain_material_gate = terrain_material_roles
        == string_vec([
            "base_concrete",
            "base_concrete",
            "base_concrete",
            "base_concrete",
            "resource_crystal",
            "resource_crystal",
            "beacon_lane",
            "beacon_lane",
            "basin_floor",
        ])
        && unique_terrain_signature_count >= 4
        && terrain_material_pixel_budget >= 432;
    let terrain_material_depth_gate = terrain_material_depth_signatures
        == string_vec([
            "terrain_foundation_beveled_edges",
            "terrain_crystal_cast_shadows",
            "terrain_lane_recessed_rails",
            "terrain_basin_fracture_shadows",
        ])
        && terrain_material_depth_sample_count >= 9
        && terrain_material_depth_pixel_budget >= 576;
    let building_facade_gate = command_core_count == 4
        && relay_count == 2
        && beacon_count == 4
        && unique_building_signature_count >= 3
        && building_facade_pixel_budget >= 860;
    let map_landmark_detail_gate = base_landmark_count == 4
        && resource_landmark_count == 2
        && lane_landmark_count == 2
        && basin_landmark_count == 2
        && relay_landmark_count == 2
        && beacon_landmark_count == 4
        && unique_landmark_signature_count >= 6
        && map_landmark_pixel_budget >= 1152;
    let runtime_actor_depth_gate = runtime_actor_depth_signatures
        == string_vec([
            "runtime_structure_roof_rim",
            "runtime_structure_side_shadow",
            "runtime_command_window_pips",
            "runtime_relay_mast_braces",
            "runtime_beacon_core_glow_rungs",
        ])
        && runtime_actor_depth_pixel_budget >= 480;
    let authored_map_art_gate = terrain_material_gate
        && terrain_material_depth_gate
        && building_facade_gate
        && map_landmark_detail_gate
        && runtime_actor_depth_gate;
    let green = authored_map_art_gate;

    json!({
        "contract_version": TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_ART_READABILITY_CONTRACT,
        "green": green,
        "source_path": "trnm-world-bevy classic_draw_first_contact_art_readability_layer",
        "terrain_sample_tiles": terrain_sample_tiles,
        "terrain_material_roles": terrain_material_roles,
        "terrain_material_signatures": terrain_material_signatures,
        "terrain_material_samples": terrain_sample_objects,
        "terrain_material_pixel_budget": terrain_material_pixel_budget,
        "terrain_material_gate": terrain_material_gate,
        "terrain_material_depth_source_path": "trnm-world-bevy classic_draw_first_contact_terrain_material_depth_detail",
        "terrain_material_depth_signatures": terrain_material_depth_signatures,
        "terrain_material_depth_sample_count": terrain_material_depth_sample_count,
        "terrain_material_depth_pixel_budget": terrain_material_depth_pixel_budget,
        "terrain_material_depth_gate": terrain_material_depth_gate,
        "building_sample_tiles": building_sample_tiles,
        "building_roles": building_roles,
        "building_facade_signatures": building_facade_signatures,
        "building_facade_samples": building_sample_objects,
        "command_core_facade_count": command_core_count,
        "relay_facade_count": relay_count,
        "beacon_facade_count": beacon_count,
        "building_facade_pixel_budget": building_facade_pixel_budget,
        "building_facade_gate": building_facade_gate,
        "map_landmark_sample_tiles": map_landmark_sample_tiles,
        "map_landmark_roles": map_landmark_roles,
        "map_landmark_signatures": map_landmark_signatures,
        "map_landmark_samples": map_landmark_sample_objects,
        "base_landmark_count": base_landmark_count,
        "resource_landmark_count": resource_landmark_count,
        "lane_landmark_count": lane_landmark_count,
        "basin_landmark_count": basin_landmark_count,
        "relay_landmark_count": relay_landmark_count,
        "beacon_landmark_count": beacon_landmark_count,
        "map_landmark_pixel_budget": map_landmark_pixel_budget,
        "map_landmark_detail_gate": map_landmark_detail_gate,
        "runtime_actor_depth_source_path": "trnm-world-bevy classic_draw_first_contact_actor_glyph",
        "runtime_actor_depth_signatures": runtime_actor_depth_signatures,
        "runtime_actor_depth_pixel_budget": runtime_actor_depth_pixel_budget,
        "runtime_actor_depth_gate": runtime_actor_depth_gate,
        "authored_map_art_gate": authored_map_art_gate,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_contact_art_readability_helpers_preserve_authored_art_contracts() {
        let guard = art_readability_guard();

        assert_eq!(
            guard.get("contract_version").and_then(Value::as_str),
            Some(TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_ART_READABILITY_CONTRACT)
        );
        assert_eq!(guard.get("green").and_then(Value::as_bool), Some(true));
        assert_eq!(
            guard.get("terrain_material_roles").cloned(),
            Some(json!([
                "base_concrete",
                "base_concrete",
                "base_concrete",
                "base_concrete",
                "resource_crystal",
                "resource_crystal",
                "beacon_lane",
                "beacon_lane",
                "basin_floor"
            ]))
        );
        assert_eq!(
            guard.get("building_roles").cloned(),
            Some(json!([
                "command_core",
                "command_core",
                "command_core",
                "command_core",
                "relay",
                "relay",
                "beacon",
                "beacon",
                "beacon",
                "beacon"
            ]))
        );
        assert_eq!(
            guard.get("map_landmark_roles").cloned(),
            Some(json!([
                "base_gate",
                "base_gate",
                "base_gate",
                "base_gate",
                "resource_cluster",
                "resource_cluster",
                "beacon_lane",
                "beacon_lane",
                "basin_scar",
                "basin_scar",
                "relay_cable",
                "relay_cable",
                "beacon_ring",
                "beacon_ring",
                "beacon_ring",
                "beacon_ring"
            ]))
        );
        assert_eq!(
            guard
                .get("terrain_material_depth_signatures")
                .and_then(Value::as_array)
                .map(|signatures| signatures.len()),
            Some(4)
        );
        assert_eq!(
            guard
                .get("runtime_actor_depth_signatures")
                .and_then(Value::as_array)
                .map(|signatures| signatures.len()),
            Some(5)
        );
        assert_eq!(
            guard
                .get("terrain_material_depth_pixel_budget")
                .and_then(Value::as_u64),
            Some(576)
        );
        assert_eq!(
            guard
                .get("runtime_actor_depth_pixel_budget")
                .and_then(Value::as_u64),
            Some(480)
        );
        assert_eq!(
            guard
                .get("command_core_facade_count")
                .and_then(Value::as_u64),
            Some(4)
        );
        assert_eq!(
            guard.get("relay_facade_count").and_then(Value::as_u64),
            Some(2)
        );
        assert_eq!(
            guard.get("beacon_facade_count").and_then(Value::as_u64),
            Some(4)
        );
        for gate in [
            "terrain_material_gate",
            "terrain_material_depth_gate",
            "building_facade_gate",
            "map_landmark_detail_gate",
            "runtime_actor_depth_gate",
            "authored_map_art_gate",
        ] {
            assert_eq!(guard.get(gate).and_then(Value::as_bool), Some(true));
        }
    }
}
