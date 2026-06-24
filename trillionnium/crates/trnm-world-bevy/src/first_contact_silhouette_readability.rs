#![cfg(not(target_os = "android"))]

use serde_json::{json, Value};
use std::collections::BTreeSet;

use crate::{
    classic_rts_tile_id, first_contact_samples,
    TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_SILHOUETTE_READABILITY_CONTRACT,
};

fn string_vec<const N: usize>(values: [&str; N]) -> Vec<String> {
    values.into_iter().map(str::to_string).collect()
}

pub(crate) fn silhouette_readability_guard() -> Value {
    let terrain_samples = first_contact_samples::silhouette_terrain_samples();
    let unit_samples = first_contact_samples::silhouette_unit_samples();
    let structure_samples = first_contact_samples::silhouette_structure_samples();
    let terrain_sample_tiles = terrain_samples
        .iter()
        .map(|(tile, _, _)| classic_rts_tile_id(*tile))
        .collect::<Vec<_>>();
    let terrain_sample_roles = terrain_samples
        .iter()
        .map(|(_, role, _)| (*role).to_string())
        .collect::<Vec<_>>();
    let terrain_signatures = terrain_samples
        .iter()
        .map(|(_, _, signature)| (*signature).to_string())
        .collect::<Vec<_>>();
    let unit_sample_tiles = unit_samples
        .iter()
        .map(|(tile, _, _)| classic_rts_tile_id(*tile))
        .collect::<Vec<_>>();
    let unit_roles = unit_samples
        .iter()
        .map(|(_, role, _)| (*role).to_string())
        .collect::<Vec<_>>();
    let unit_signatures = unit_samples
        .iter()
        .map(|(_, _, signature)| (*signature).to_string())
        .collect::<Vec<_>>();
    let structure_sample_tiles = structure_samples
        .iter()
        .map(|(tile, _, _)| classic_rts_tile_id(*tile))
        .collect::<Vec<_>>();
    let structure_roles = structure_samples
        .iter()
        .map(|(_, role, _)| (*role).to_string())
        .collect::<Vec<_>>();
    let structure_signatures = structure_samples
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
    let unit_sample_objects = unit_samples
        .iter()
        .map(|(tile, role, signature)| {
            json!({
                "tile": classic_rts_tile_id(*tile),
                "role": role,
                "signature": signature,
            })
        })
        .collect::<Vec<_>>();
    let structure_sample_objects = structure_samples
        .iter()
        .map(|(tile, role, signature)| {
            json!({
                "tile": classic_rts_tile_id(*tile),
                "role": role,
                "signature": signature,
            })
        })
        .collect::<Vec<_>>();
    let unique_terrain_signature_count = terrain_signatures.iter().collect::<BTreeSet<_>>().len();
    let unique_unit_signature_count = unit_signatures.iter().collect::<BTreeSet<_>>().len();
    let unique_structure_signature_count =
        structure_signatures.iter().collect::<BTreeSet<_>>().len();
    let command_core_count = structure_roles
        .iter()
        .filter(|role| role.as_str() == "command_core")
        .count();
    let relay_count = structure_roles
        .iter()
        .filter(|role| role.as_str() == "relay")
        .count();
    let beacon_count = structure_roles
        .iter()
        .filter(|role| role.as_str() == "beacon")
        .count();
    let terrain_zone_pixel_budget = terrain_samples.len() * 32;
    let unit_silhouette_pixel_budget = unit_samples.len() * 86;
    let structure_roofline_pixel_budget = structure_samples.len() * 96;
    let beacon_spire_pixel_budget = beacon_count * 72;
    let terrain_zone_gate = terrain_sample_roles
        == string_vec([
            "base_pad",
            "base_pad",
            "base_pad",
            "base_pad",
            "resource_zone",
            "resource_zone",
            "objective_lane",
            "objective_lane",
            "central_basin",
        ])
        && unique_terrain_signature_count >= 4
        && terrain_zone_pixel_budget >= 288;
    let unit_role_silhouette_gate = unit_roles
        == string_vec(["worker", "scout", "warden", "relay"])
        && unique_unit_signature_count == 4
        && unit_silhouette_pixel_budget >= 344;
    let structure_roofline_gate = command_core_count >= 4
        && relay_count >= 2
        && unique_structure_signature_count >= 3
        && structure_roofline_pixel_budget >= 960;
    let beacon_spire_gate = beacon_count == 4 && beacon_spire_pixel_budget >= 288;
    let map_object_silhouette_gate = terrain_zone_gate
        && unit_role_silhouette_gate
        && structure_roofline_gate
        && beacon_spire_gate;
    let green = map_object_silhouette_gate;

    json!({
        "contract_version": TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_SILHOUETTE_READABILITY_CONTRACT,
        "green": green,
        "source_path": "trnm-world-bevy classic_draw_first_contact_silhouette_readability_layer",
        "terrain_sample_tiles": terrain_sample_tiles,
        "terrain_sample_roles": terrain_sample_roles,
        "terrain_signatures": terrain_signatures,
        "terrain_samples": terrain_sample_objects,
        "terrain_zone_pixel_budget": terrain_zone_pixel_budget,
        "terrain_zone_gate": terrain_zone_gate,
        "unit_sample_tiles": unit_sample_tiles,
        "unit_roles": unit_roles,
        "unit_signatures": unit_signatures,
        "unit_samples": unit_sample_objects,
        "unit_silhouette_pixel_budget": unit_silhouette_pixel_budget,
        "unit_role_silhouette_gate": unit_role_silhouette_gate,
        "structure_sample_tiles": structure_sample_tiles,
        "structure_roles": structure_roles,
        "structure_signatures": structure_signatures,
        "structure_samples": structure_sample_objects,
        "command_core_silhouette_count": command_core_count,
        "relay_silhouette_count": relay_count,
        "beacon_silhouette_count": beacon_count,
        "structure_roofline_pixel_budget": structure_roofline_pixel_budget,
        "beacon_spire_pixel_budget": beacon_spire_pixel_budget,
        "structure_roofline_gate": structure_roofline_gate,
        "beacon_spire_gate": beacon_spire_gate,
        "map_object_silhouette_gate": map_object_silhouette_gate,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_contact_silhouette_readability_helpers_preserve_shape_contracts() {
        let guard = silhouette_readability_guard();

        assert_eq!(
            guard.get("contract_version").and_then(Value::as_str),
            Some(TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_SILHOUETTE_READABILITY_CONTRACT)
        );
        assert_eq!(guard.get("green").and_then(Value::as_bool), Some(true));
        assert_eq!(
            guard.get("terrain_sample_roles").cloned(),
            Some(json!([
                "base_pad",
                "base_pad",
                "base_pad",
                "base_pad",
                "resource_zone",
                "resource_zone",
                "objective_lane",
                "objective_lane",
                "central_basin"
            ]))
        );
        assert_eq!(
            guard.get("unit_roles").cloned(),
            Some(json!(["worker", "scout", "warden", "relay"]))
        );
        assert_eq!(
            guard.get("unit_signatures").cloned(),
            Some(json!([
                "cargo_pack",
                "sensor_mast",
                "shield_plate",
                "relay_courier"
            ]))
        );
        assert_eq!(
            guard
                .get("command_core_silhouette_count")
                .and_then(Value::as_u64),
            Some(4)
        );
        assert_eq!(
            guard.get("relay_silhouette_count").and_then(Value::as_u64),
            Some(2)
        );
        assert_eq!(
            guard.get("beacon_silhouette_count").and_then(Value::as_u64),
            Some(4)
        );
        assert_eq!(
            guard
                .get("terrain_signatures")
                .and_then(Value::as_array)
                .map(|signatures| signatures.len()),
            Some(9)
        );
        for gate in [
            "terrain_zone_gate",
            "unit_role_silhouette_gate",
            "structure_roofline_gate",
            "beacon_spire_gate",
            "map_object_silhouette_gate",
        ] {
            assert_eq!(guard.get(gate).and_then(Value::as_bool), Some(true));
        }
    }
}
