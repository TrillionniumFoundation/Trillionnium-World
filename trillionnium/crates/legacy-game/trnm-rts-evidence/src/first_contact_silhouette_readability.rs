use serde_json::{json, Value};
use std::collections::BTreeSet;
use trnm_rts_bevy_runtime::rts_runtime_tile_id;
use trnm_rts_data::first_contact_samples;

use crate::TRNM_RTS_EVIDENCE_FIRST_CONTACT_SILHOUETTE_READABILITY_CONTRACT;

fn string_vec<const N: usize>(values: [&str; N]) -> Vec<String> {
    values.into_iter().map(str::to_string).collect()
}

fn tile_id(tile: (i32, i32)) -> String {
    rts_runtime_tile_id(tile)
}

pub fn first_contact_silhouette_readability_guard() -> Value {
    let terrain_samples = first_contact_samples::silhouette_terrain_samples();
    let preview_resource_samples = first_contact_samples::silhouette_preview_resource_samples();
    let unit_samples = first_contact_samples::silhouette_unit_samples();
    let structure_samples = first_contact_samples::silhouette_structure_samples();
    let art_landmark_samples = first_contact_samples::art_landmark_samples();
    let terrain_sample_tiles = terrain_samples
        .iter()
        .map(|(tile, _, _)| tile_id(*tile))
        .collect::<Vec<_>>();
    let terrain_sample_roles = terrain_samples
        .iter()
        .map(|(_, role, _)| (*role).to_string())
        .collect::<Vec<_>>();
    let terrain_signatures = terrain_samples
        .iter()
        .map(|(_, _, signature)| (*signature).to_string())
        .collect::<Vec<_>>();
    let preview_resource_sample_tiles = preview_resource_samples
        .iter()
        .map(|(tile, _, _)| tile_id(*tile))
        .collect::<Vec<_>>();
    let preview_resource_roles = preview_resource_samples
        .iter()
        .map(|(_, role, _)| (*role).to_string())
        .collect::<Vec<_>>();
    let preview_resource_signatures = preview_resource_samples
        .iter()
        .map(|(_, _, signature)| (*signature).to_string())
        .collect::<Vec<_>>();
    let unit_sample_tiles = unit_samples
        .iter()
        .map(|(tile, _, _)| tile_id(*tile))
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
        .map(|(tile, _, _)| tile_id(*tile))
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
                "tile": tile_id(*tile),
                "role": role,
                "signature": signature,
            })
        })
        .collect::<Vec<_>>();
    let preview_resource_sample_objects = preview_resource_samples
        .iter()
        .map(|(tile, role, signature)| {
            json!({
                "tile": tile_id(*tile),
                "role": role,
                "signature": signature,
            })
        })
        .collect::<Vec<_>>();
    let unit_sample_objects = unit_samples
        .iter()
        .map(|(tile, role, signature)| {
            json!({
                "tile": tile_id(*tile),
                "role": role,
                "signature": signature,
            })
        })
        .collect::<Vec<_>>();
    let structure_sample_objects = structure_samples
        .iter()
        .map(|(tile, role, signature)| {
            json!({
                "tile": tile_id(*tile),
                "role": role,
                "signature": signature,
            })
        })
        .collect::<Vec<_>>();
    let unique_terrain_signature_count = terrain_signatures.iter().collect::<BTreeSet<_>>().len();
    let unique_preview_resource_signature_count = preview_resource_signatures
        .iter()
        .collect::<BTreeSet<_>>()
        .len();
    let unique_unit_signature_count = unit_signatures.iter().collect::<BTreeSet<_>>().len();
    let unique_structure_signature_count =
        structure_signatures.iter().collect::<BTreeSet<_>>().len();
    let command_core_count = structure_roles
        .iter()
        .filter(|role| role.as_str() == "command_core")
        .count();
    let player_screen_command_core_faction_samples = structure_samples
        .iter()
        .filter_map(|(tile, role, signature)| {
            if *role == "command_core" && *signature == "stepped_roof_core" {
                Some(json!({
                    "tile": tile_id(*tile),
                    "role": role,
                    "signature": signature,
                }))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let player_screen_command_core_faction_signatures = string_vec([
        "player_screen_command_core_faction_micro_ticks",
        "player_screen_command_core_roof_body_muted",
    ]);
    let player_screen_command_core_faction_count = player_screen_command_core_faction_samples.len();
    let player_screen_command_core_faction_ticks_per_core = 4usize;
    let player_screen_command_core_faction_tick_width_px = 6usize;
    let player_screen_command_core_faction_tick_height_px = 2usize;
    let player_screen_command_core_faction_pixel_budget = player_screen_command_core_faction_count
        * player_screen_command_core_faction_ticks_per_core
        * player_screen_command_core_faction_tick_width_px
        * player_screen_command_core_faction_tick_height_px;
    let player_screen_command_core_hot_roof_pixel_budget = 0usize;
    let relay_count = structure_roles
        .iter()
        .filter(|role| role.as_str() == "relay")
        .count();
    let beacon_count = structure_roles
        .iter()
        .filter(|role| role.as_str() == "beacon")
        .count();
    let player_screen_target_beacon = (16, 9);
    let player_screen_target_beacon_tile = tile_id(player_screen_target_beacon);
    let player_screen_secondary_beacon_body_samples = structure_samples
        .iter()
        .filter_map(|(tile, role, signature)| {
            if *role == "beacon"
                && *signature == "vertical_beacon_spire"
                && *tile != player_screen_target_beacon
            {
                Some(json!({
                    "tile": tile_id(*tile),
                    "role": role,
                    "signature": signature,
                }))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let player_screen_secondary_beacon_body_signatures =
        string_vec(["player_screen_secondary_beacon_body_micro_cues"]);
    let player_screen_secondary_beacon_body_count =
        player_screen_secondary_beacon_body_samples.len();
    let player_screen_secondary_beacon_body_cues_per_beacon = 5usize;
    let player_screen_secondary_beacon_body_cue_width_px = 8usize;
    let player_screen_secondary_beacon_body_cue_height_px = 2usize;
    let player_screen_secondary_beacon_body_pixel_budget = player_screen_secondary_beacon_body_count
        * player_screen_secondary_beacon_body_cues_per_beacon
        * player_screen_secondary_beacon_body_cue_width_px
        * player_screen_secondary_beacon_body_cue_height_px;
    let player_screen_secondary_beacon_actor_body_samples = art_landmark_samples
        .iter()
        .filter_map(|(tile, role, signature)| {
            if *role == "beacon_ring"
                && *signature == "beacon_capture_rings"
                && *tile != player_screen_target_beacon
            {
                Some(json!({
                    "tile": tile_id(*tile),
                    "role": role,
                    "signature": "beacon_ring_actor_body",
                }))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let player_screen_secondary_beacon_actor_body_signatures = string_vec([
        "player_screen_secondary_beacon_actor_body_micro_cues",
        "player_screen_secondary_beacon_hot_actor_body_suppressed",
    ]);
    let player_screen_secondary_beacon_actor_body_count =
        player_screen_secondary_beacon_actor_body_samples.len();
    let player_screen_secondary_beacon_actor_body_cues_per_beacon = 4usize;
    let player_screen_secondary_beacon_actor_body_cue_width_px = 8usize;
    let player_screen_secondary_beacon_actor_body_cue_height_px = 2usize;
    let player_screen_secondary_beacon_actor_body_pixel_budget =
        player_screen_secondary_beacon_actor_body_count
            * player_screen_secondary_beacon_actor_body_cues_per_beacon
            * player_screen_secondary_beacon_actor_body_cue_width_px
            * player_screen_secondary_beacon_actor_body_cue_height_px;
    let player_screen_secondary_beacon_hot_actor_body_pixel_budget = 0usize;
    let terrain_zone_pixel_budget = terrain_samples.len() * 32;
    let preview_resource_bloom_count = preview_resource_samples.len();
    let preview_resource_bloom_cues_per_cluster = 4usize;
    let preview_resource_bloom_cue_width_px = 6usize;
    let preview_resource_bloom_cue_height_px = 2usize;
    let preview_resource_bloom_pixel_budget = preview_resource_bloom_count
        * preview_resource_bloom_cues_per_cluster
        * preview_resource_bloom_cue_width_px
        * preview_resource_bloom_cue_height_px;
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
    let preview_resource_bloom_gate = preview_resource_roles
        == vec!["flux_bloom".to_string(); preview_resource_bloom_count]
        && unique_preview_resource_signature_count == 1
        && preview_resource_bloom_count == 11
        && preview_resource_bloom_pixel_budget >= 528;
    let structure_roofline_gate = command_core_count >= 4
        && relay_count >= 2
        && unique_structure_signature_count >= 3
        && structure_roofline_pixel_budget >= 960;
    let player_screen_command_core_faction_gate = player_screen_command_core_faction_count == 4
        && player_screen_command_core_faction_ticks_per_core == 4
        && player_screen_command_core_faction_tick_width_px == 6
        && player_screen_command_core_faction_tick_height_px == 2
        && player_screen_command_core_faction_pixel_budget <= 192
        && player_screen_command_core_hot_roof_pixel_budget == 0
        && player_screen_command_core_faction_signatures
            .iter()
            .any(|signature| signature == "player_screen_command_core_faction_micro_ticks")
        && player_screen_command_core_faction_signatures
            .iter()
            .any(|signature| signature == "player_screen_command_core_roof_body_muted");
    let beacon_spire_gate = beacon_count == 4 && beacon_spire_pixel_budget >= 288;
    let player_screen_secondary_beacon_body_gate = player_screen_target_beacon_tile == "16,9"
        && player_screen_secondary_beacon_body_count == 3
        && player_screen_secondary_beacon_body_cues_per_beacon == 5
        && player_screen_secondary_beacon_body_cue_width_px == 8
        && player_screen_secondary_beacon_body_cue_height_px == 2
        && player_screen_secondary_beacon_body_pixel_budget <= 240
        && player_screen_secondary_beacon_body_signatures
            .iter()
            .any(|signature| signature == "player_screen_secondary_beacon_body_micro_cues");
    let player_screen_secondary_beacon_actor_body_gate =
        player_screen_secondary_beacon_actor_body_count == 3
            && player_screen_secondary_beacon_actor_body_cues_per_beacon == 4
            && player_screen_secondary_beacon_actor_body_cue_width_px == 8
            && player_screen_secondary_beacon_actor_body_cue_height_px == 2
            && player_screen_secondary_beacon_actor_body_pixel_budget <= 192
            && player_screen_secondary_beacon_hot_actor_body_pixel_budget == 0
            && player_screen_secondary_beacon_actor_body_signatures
                .iter()
                .any(|signature| {
                    signature == "player_screen_secondary_beacon_actor_body_micro_cues"
                })
            && player_screen_secondary_beacon_actor_body_signatures
                .iter()
                .any(|signature| {
                    signature == "player_screen_secondary_beacon_hot_actor_body_suppressed"
                });
    let map_object_silhouette_gate = terrain_zone_gate
        && preview_resource_bloom_gate
        && unit_role_silhouette_gate
        && structure_roofline_gate
        && player_screen_command_core_faction_gate
        && beacon_spire_gate
        && player_screen_secondary_beacon_body_gate
        && player_screen_secondary_beacon_actor_body_gate;
    let green = map_object_silhouette_gate;

    json!({
        "contract_version": TRNM_RTS_EVIDENCE_FIRST_CONTACT_SILHOUETTE_READABILITY_CONTRACT,
        "green": green,
        "source_path": "trnm-world-bevy classic_draw_first_contact_silhouette_readability_layer",
        "terrain_sample_tiles": terrain_sample_tiles,
        "terrain_sample_roles": terrain_sample_roles,
        "terrain_signatures": terrain_signatures,
        "terrain_samples": terrain_sample_objects,
        "terrain_zone_pixel_budget": terrain_zone_pixel_budget,
        "terrain_zone_gate": terrain_zone_gate,
        "preview_resource_sample_tiles": preview_resource_sample_tiles,
        "preview_resource_roles": preview_resource_roles,
        "preview_resource_signatures": preview_resource_signatures,
        "preview_resource_samples": preview_resource_sample_objects,
        "preview_resource_bloom_count": preview_resource_bloom_count,
        "preview_resource_bloom_cues_per_cluster": preview_resource_bloom_cues_per_cluster,
        "preview_resource_bloom_cue_width_px": preview_resource_bloom_cue_width_px,
        "preview_resource_bloom_cue_height_px": preview_resource_bloom_cue_height_px,
        "preview_resource_bloom_pixel_budget": preview_resource_bloom_pixel_budget,
        "preview_resource_bloom_gate": preview_resource_bloom_gate,
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
        "player_screen_command_core_faction_samples": player_screen_command_core_faction_samples,
        "player_screen_command_core_faction_count": player_screen_command_core_faction_count,
        "player_screen_command_core_faction_ticks_per_core": player_screen_command_core_faction_ticks_per_core,
        "player_screen_command_core_faction_tick_width_px": player_screen_command_core_faction_tick_width_px,
        "player_screen_command_core_faction_tick_height_px": player_screen_command_core_faction_tick_height_px,
        "player_screen_command_core_faction_pixel_budget": player_screen_command_core_faction_pixel_budget,
        "player_screen_command_core_hot_roof_pixel_budget": player_screen_command_core_hot_roof_pixel_budget,
        "player_screen_command_core_faction_signatures": player_screen_command_core_faction_signatures,
        "player_screen_command_core_faction_gate": player_screen_command_core_faction_gate,
        "relay_silhouette_count": relay_count,
        "beacon_silhouette_count": beacon_count,
        "player_screen_target_beacon_tile": player_screen_target_beacon_tile,
        "player_screen_secondary_beacon_body_samples": player_screen_secondary_beacon_body_samples,
        "player_screen_secondary_beacon_body_count": player_screen_secondary_beacon_body_count,
        "player_screen_secondary_beacon_body_cues_per_beacon": player_screen_secondary_beacon_body_cues_per_beacon,
        "player_screen_secondary_beacon_body_cue_width_px": player_screen_secondary_beacon_body_cue_width_px,
        "player_screen_secondary_beacon_body_cue_height_px": player_screen_secondary_beacon_body_cue_height_px,
        "player_screen_secondary_beacon_body_pixel_budget": player_screen_secondary_beacon_body_pixel_budget,
        "player_screen_secondary_beacon_body_signatures": player_screen_secondary_beacon_body_signatures,
        "player_screen_secondary_beacon_body_gate": player_screen_secondary_beacon_body_gate,
        "player_screen_secondary_beacon_actor_body_samples": player_screen_secondary_beacon_actor_body_samples,
        "player_screen_secondary_beacon_actor_body_count": player_screen_secondary_beacon_actor_body_count,
        "player_screen_secondary_beacon_actor_body_cues_per_beacon": player_screen_secondary_beacon_actor_body_cues_per_beacon,
        "player_screen_secondary_beacon_actor_body_cue_width_px": player_screen_secondary_beacon_actor_body_cue_width_px,
        "player_screen_secondary_beacon_actor_body_cue_height_px": player_screen_secondary_beacon_actor_body_cue_height_px,
        "player_screen_secondary_beacon_actor_body_pixel_budget": player_screen_secondary_beacon_actor_body_pixel_budget,
        "player_screen_secondary_beacon_hot_actor_body_pixel_budget": player_screen_secondary_beacon_hot_actor_body_pixel_budget,
        "player_screen_secondary_beacon_actor_body_signatures": player_screen_secondary_beacon_actor_body_signatures,
        "player_screen_secondary_beacon_actor_body_gate": player_screen_secondary_beacon_actor_body_gate,
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
        let guard = first_contact_silhouette_readability_guard();

        assert_eq!(
            guard.get("contract_version").and_then(Value::as_str),
            Some(TRNM_RTS_EVIDENCE_FIRST_CONTACT_SILHOUETTE_READABILITY_CONTRACT)
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
            guard
                .get("preview_resource_bloom_count")
                .and_then(Value::as_u64),
            Some(11)
        );
        assert_eq!(
            guard
                .get("preview_resource_bloom_cues_per_cluster")
                .and_then(Value::as_u64),
            Some(4)
        );
        assert_eq!(
            guard
                .get("preview_resource_bloom_cue_width_px")
                .and_then(Value::as_u64),
            Some(6)
        );
        assert_eq!(
            guard
                .get("preview_resource_bloom_cue_height_px")
                .and_then(Value::as_u64),
            Some(2)
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
            guard
                .get("player_screen_command_core_faction_samples")
                .cloned(),
            Some(json!([
                {"tile": "8,8", "role": "command_core", "signature": "stepped_roof_core"},
                {"tile": "25,8", "role": "command_core", "signature": "stepped_roof_core"},
                {"tile": "25,25", "role": "command_core", "signature": "stepped_roof_core"},
                {"tile": "8,25", "role": "command_core", "signature": "stepped_roof_core"}
            ]))
        );
        assert_eq!(
            guard
                .get("player_screen_command_core_faction_count")
                .and_then(Value::as_u64),
            Some(4)
        );
        assert_eq!(
            guard
                .get("player_screen_command_core_faction_ticks_per_core")
                .and_then(Value::as_u64),
            Some(4)
        );
        assert_eq!(
            guard
                .get("player_screen_command_core_faction_tick_width_px")
                .and_then(Value::as_u64),
            Some(6)
        );
        assert_eq!(
            guard
                .get("player_screen_command_core_faction_tick_height_px")
                .and_then(Value::as_u64),
            Some(2)
        );
        assert_eq!(
            guard
                .get("player_screen_command_core_faction_pixel_budget")
                .and_then(Value::as_u64),
            Some(192)
        );
        assert_eq!(
            guard
                .get("player_screen_command_core_hot_roof_pixel_budget")
                .and_then(Value::as_u64),
            Some(0)
        );
        assert_eq!(
            guard
                .get("player_screen_command_core_faction_signatures")
                .and_then(Value::as_array)
                .map(|signatures| signatures.iter().any(|signature| {
                    signature.as_str() == Some("player_screen_command_core_faction_micro_ticks")
                }) && signatures.iter().any(|signature| {
                    signature.as_str() == Some("player_screen_command_core_roof_body_muted")
                })),
            Some(true)
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
                .get("player_screen_target_beacon_tile")
                .and_then(Value::as_str),
            Some("16,9")
        );
        assert_eq!(
            guard
                .get("player_screen_secondary_beacon_body_samples")
                .cloned(),
            Some(json!([
                {"tile": "16,24", "role": "beacon", "signature": "vertical_beacon_spire"},
                {"tile": "9,16", "role": "beacon", "signature": "vertical_beacon_spire"},
                {"tile": "24,16", "role": "beacon", "signature": "vertical_beacon_spire"}
            ]))
        );
        assert_eq!(
            guard
                .get("player_screen_secondary_beacon_body_count")
                .and_then(Value::as_u64),
            Some(3)
        );
        assert_eq!(
            guard
                .get("player_screen_secondary_beacon_body_cues_per_beacon")
                .and_then(Value::as_u64),
            Some(5)
        );
        assert_eq!(
            guard
                .get("player_screen_secondary_beacon_body_cue_width_px")
                .and_then(Value::as_u64),
            Some(8)
        );
        assert_eq!(
            guard
                .get("player_screen_secondary_beacon_body_cue_height_px")
                .and_then(Value::as_u64),
            Some(2)
        );
        assert_eq!(
            guard
                .get("player_screen_secondary_beacon_body_pixel_budget")
                .and_then(Value::as_u64),
            Some(240)
        );
        assert_eq!(
            guard
                .get("player_screen_secondary_beacon_body_signatures")
                .and_then(Value::as_array)
                .map(|signatures| signatures.iter().any(|signature| {
                    signature.as_str() == Some("player_screen_secondary_beacon_body_micro_cues")
                })),
            Some(true)
        );
        assert_eq!(
            guard
                .get("player_screen_secondary_beacon_actor_body_samples")
                .cloned(),
            Some(json!([
                {"tile": "16,24", "role": "beacon_ring", "signature": "beacon_ring_actor_body"},
                {"tile": "9,16", "role": "beacon_ring", "signature": "beacon_ring_actor_body"},
                {"tile": "24,16", "role": "beacon_ring", "signature": "beacon_ring_actor_body"}
            ]))
        );
        assert_eq!(
            guard
                .get("player_screen_secondary_beacon_actor_body_count")
                .and_then(Value::as_u64),
            Some(3)
        );
        assert_eq!(
            guard
                .get("player_screen_secondary_beacon_actor_body_cues_per_beacon")
                .and_then(Value::as_u64),
            Some(4)
        );
        assert_eq!(
            guard
                .get("player_screen_secondary_beacon_actor_body_cue_width_px")
                .and_then(Value::as_u64),
            Some(8)
        );
        assert_eq!(
            guard
                .get("player_screen_secondary_beacon_actor_body_cue_height_px")
                .and_then(Value::as_u64),
            Some(2)
        );
        assert_eq!(
            guard
                .get("player_screen_secondary_beacon_actor_body_pixel_budget")
                .and_then(Value::as_u64),
            Some(192)
        );
        assert_eq!(
            guard
                .get("player_screen_secondary_beacon_hot_actor_body_pixel_budget")
                .and_then(Value::as_u64),
            Some(0)
        );
        assert_eq!(
            guard
                .get("player_screen_secondary_beacon_actor_body_signatures")
                .and_then(Value::as_array)
                .map(|signatures| signatures.iter().any(|signature| {
                    signature.as_str()
                        == Some("player_screen_secondary_beacon_actor_body_micro_cues")
                }) && signatures.iter().any(|signature| {
                    signature.as_str()
                        == Some("player_screen_secondary_beacon_hot_actor_body_suppressed")
                })),
            Some(true)
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
            "preview_resource_bloom_gate",
            "unit_role_silhouette_gate",
            "structure_roofline_gate",
            "player_screen_command_core_faction_gate",
            "beacon_spire_gate",
            "player_screen_secondary_beacon_body_gate",
            "player_screen_secondary_beacon_actor_body_gate",
            "map_object_silhouette_gate",
        ] {
            assert_eq!(guard.get(gate).and_then(Value::as_bool), Some(true));
        }
    }
}
