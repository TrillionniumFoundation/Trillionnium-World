use serde_json::{json, Value};
use std::collections::BTreeSet;
use trnm_rts_bevy_runtime::rts_runtime_tile_id;
use trnm_rts_data::first_contact_samples;

use crate::TRNM_RTS_EVIDENCE_FIRST_CONTACT_ART_READABILITY_CONTRACT;

fn string_vec<const N: usize>(values: [&str; N]) -> Vec<String> {
    values.into_iter().map(str::to_string).collect()
}

fn tile_id(tile: (i32, i32)) -> String {
    rts_runtime_tile_id(tile)
}

fn lower_secondary_beacon_terrain_detail(tile: (i32, i32), role: &str, signature: &str) -> bool {
    tile == (16, 24) && role == "beacon_lane" && signature == "painted_lane_chevrons"
}

fn resource_crystal_shard_detail(_tile: (i32, i32), role: &str, signature: &str) -> bool {
    role == "resource_crystal" && signature == "flux_crystal_shards"
}

fn lower_secondary_beacon_landmark_detail(tile: (i32, i32), role: &str, signature: &str) -> bool {
    tile == (16, 23) && role == "beacon_lane" && signature == "lane_power_pylons"
}

fn secondary_beacon_capture_ring_detail(tile: (i32, i32), role: &str, signature: &str) -> bool {
    matches!(
        (tile, role, signature),
        ((16, 24), "beacon_ring", "beacon_capture_rings")
            | ((9, 16), "beacon_ring", "beacon_capture_rings")
            | ((24, 16), "beacon_ring", "beacon_capture_rings")
    )
}

fn active_beacon_capture_ring_detail(tile: (i32, i32), role: &str, signature: &str) -> bool {
    tile == (16, 9) && role == "beacon_ring" && signature == "beacon_capture_rings"
}

fn resource_cluster_sparkle_detail(_tile: (i32, i32), role: &str, signature: &str) -> bool {
    role == "resource_cluster" && signature == "crystal_shadow_sparkles"
}

fn command_core_art_detail(_tile: (i32, i32), role: &str, signature: &str) -> bool {
    role == "command_core" && signature == "lit_window_rows"
}

fn base_gate_lamp_detail(_tile: (i32, i32), role: &str, signature: &str) -> bool {
    role == "base_gate" && signature == "base_gate_lamps"
}

pub fn first_contact_art_readability_guard() -> Value {
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
        .map(|(tile, _, _)| tile_id(*tile))
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
        .map(|(tile, _, _)| tile_id(*tile))
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
        .map(|(tile, _, _)| tile_id(*tile))
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
                "tile": tile_id(*tile),
                "role": role,
                "signature": signature,
            })
        })
        .collect::<Vec<_>>();
    let building_sample_objects = building_samples
        .iter()
        .map(|(tile, role, signature)| {
            json!({
                "tile": tile_id(*tile),
                "role": role,
                "signature": signature,
            })
        })
        .collect::<Vec<_>>();
    let map_landmark_sample_objects = landmark_samples
        .iter()
        .map(|(tile, role, signature)| {
            json!({
                "tile": tile_id(*tile),
                "role": role,
                "signature": signature,
            })
        })
        .collect::<Vec<_>>();
    let mut lower_secondary_beacon_art_samples = terrain_samples
        .iter()
        .filter(|(tile, role, signature)| {
            lower_secondary_beacon_terrain_detail(*tile, role, signature)
        })
        .map(|(tile, role, signature)| {
            json!({
                "tile": tile_id(*tile),
                "role": role,
                "signature": signature,
            })
        })
        .collect::<Vec<_>>();
    lower_secondary_beacon_art_samples.extend(
        landmark_samples
            .iter()
            .filter(|(tile, role, signature)| {
                lower_secondary_beacon_landmark_detail(*tile, role, signature)
            })
            .map(|(tile, role, signature)| {
                json!({
                    "tile": tile_id(*tile),
                    "role": role,
                    "signature": signature,
                })
            }),
    );
    let lower_secondary_beacon_art_signatures = string_vec([
        "lower_secondary_beacon_micro_chevrons",
        "lower_secondary_beacon_power_pylons_capped",
    ]);
    let lower_secondary_beacon_art_sample_count = lower_secondary_beacon_art_samples.len();
    let lower_secondary_beacon_art_pixel_budget = lower_secondary_beacon_art_sample_count * 32;
    let secondary_beacon_capture_ring_samples = landmark_samples
        .iter()
        .filter(|(tile, role, signature)| {
            secondary_beacon_capture_ring_detail(*tile, role, signature)
        })
        .map(|(tile, role, signature)| {
            json!({
                "tile": tile_id(*tile),
                "role": role,
                "signature": signature,
            })
        })
        .collect::<Vec<_>>();
    let secondary_beacon_capture_ring_signatures =
        string_vec(["secondary_beacon_capture_micro_cues"]);
    let secondary_beacon_capture_ring_count = secondary_beacon_capture_ring_samples.len();
    let secondary_beacon_capture_ring_cues_per_ring = 4_usize;
    let secondary_beacon_capture_ring_cue_count =
        secondary_beacon_capture_ring_count * secondary_beacon_capture_ring_cues_per_ring;
    let secondary_beacon_capture_ring_cue_width_px = 8_usize;
    let secondary_beacon_capture_ring_cue_height_px = 2_usize;
    let secondary_beacon_capture_ring_pixel_budget = secondary_beacon_capture_ring_cue_count
        * secondary_beacon_capture_ring_cue_width_px
        * secondary_beacon_capture_ring_cue_height_px;
    let player_screen_active_beacon_capture_ring_samples = landmark_samples
        .iter()
        .filter(|(tile, role, signature)| active_beacon_capture_ring_detail(*tile, role, signature))
        .map(|(tile, role, signature)| {
            json!({
                "tile": tile_id(*tile),
                "role": role,
                "signature": signature,
            })
        })
        .collect::<Vec<_>>();
    let player_screen_active_beacon_capture_ring_signatures =
        string_vec(["player_screen_active_beacon_capture_micro_ticks"]);
    let player_screen_active_beacon_capture_ring_count =
        player_screen_active_beacon_capture_ring_samples.len();
    let player_screen_active_beacon_capture_ring_ticks_per_ring = 4usize;
    let player_screen_active_beacon_capture_ring_tick_width_px = 10usize;
    let player_screen_active_beacon_capture_ring_tick_height_px = 2usize;
    let player_screen_active_beacon_capture_ring_pixel_budget =
        player_screen_active_beacon_capture_ring_count
            * player_screen_active_beacon_capture_ring_ticks_per_ring
            * player_screen_active_beacon_capture_ring_tick_width_px
            * player_screen_active_beacon_capture_ring_tick_height_px;
    let player_screen_resource_crystal_shard_samples = terrain_samples
        .iter()
        .filter(|(tile, role, signature)| resource_crystal_shard_detail(*tile, role, signature))
        .map(|(tile, role, signature)| {
            json!({
                "tile": tile_id(*tile),
                "role": role,
                "signature": signature,
            })
        })
        .collect::<Vec<_>>();
    let player_screen_resource_crystal_shard_signatures = string_vec([
        "player_screen_resource_crystal_micro_shards",
        "player_screen_resource_crystal_hot_shards_suppressed",
    ]);
    let player_screen_resource_crystal_shard_count =
        player_screen_resource_crystal_shard_samples.len();
    let player_screen_resource_crystal_shards_per_cluster = 4usize;
    let player_screen_resource_crystal_shard_width_px = 4usize;
    let player_screen_resource_crystal_shard_height_px = 2usize;
    let player_screen_resource_crystal_shard_pixel_budget =
        player_screen_resource_crystal_shard_count
            * player_screen_resource_crystal_shards_per_cluster
            * player_screen_resource_crystal_shard_width_px
            * player_screen_resource_crystal_shard_height_px;
    let player_screen_resource_crystal_hot_shard_pixel_budget = 0usize;
    let player_screen_resource_cluster_sparkle_samples = landmark_samples
        .iter()
        .filter(|(tile, role, signature)| resource_cluster_sparkle_detail(*tile, role, signature))
        .map(|(tile, role, signature)| {
            json!({
                "tile": tile_id(*tile),
                "role": role,
                "signature": signature,
            })
        })
        .collect::<Vec<_>>();
    let player_screen_resource_cluster_sparkle_signatures = string_vec([
        "player_screen_resource_cluster_micro_sparkles",
        "player_screen_resource_cluster_hot_glints_suppressed",
    ]);
    let player_screen_resource_cluster_sparkle_count =
        player_screen_resource_cluster_sparkle_samples.len();
    let player_screen_resource_cluster_sparkles_per_cluster = 4usize;
    let player_screen_resource_cluster_sparkle_width_px = 4usize;
    let player_screen_resource_cluster_sparkle_height_px = 2usize;
    let player_screen_resource_cluster_sparkle_pixel_budget =
        player_screen_resource_cluster_sparkle_count
            * player_screen_resource_cluster_sparkles_per_cluster
            * player_screen_resource_cluster_sparkle_width_px
            * player_screen_resource_cluster_sparkle_height_px;
    let player_screen_resource_cluster_hot_glint_pixel_budget = 0usize;
    let player_screen_command_core_art_samples = building_samples
        .iter()
        .filter(|(tile, role, signature)| command_core_art_detail(*tile, role, signature))
        .map(|(tile, role, signature)| {
            json!({
                "tile": tile_id(*tile),
                "role": role,
                "signature": signature,
            })
        })
        .collect::<Vec<_>>();
    let player_screen_command_core_art_signatures = string_vec([
        "player_screen_command_core_art_micro_ticks",
        "player_screen_command_core_hot_facade_suppressed",
    ]);
    let player_screen_command_core_art_count = player_screen_command_core_art_samples.len();
    let player_screen_command_core_art_ticks_per_core = 4usize;
    let player_screen_command_core_art_tick_width_px = 6usize;
    let player_screen_command_core_art_tick_height_px = 2usize;
    let player_screen_command_core_art_pixel_budget = player_screen_command_core_art_count
        * player_screen_command_core_art_ticks_per_core
        * player_screen_command_core_art_tick_width_px
        * player_screen_command_core_art_tick_height_px;
    let player_screen_command_core_hot_facade_pixel_budget = 0usize;
    let player_screen_base_gate_lamp_samples = landmark_samples
        .iter()
        .filter(|(tile, role, signature)| base_gate_lamp_detail(*tile, role, signature))
        .map(|(tile, role, signature)| {
            json!({
                "tile": tile_id(*tile),
                "role": role,
                "signature": signature,
            })
        })
        .collect::<Vec<_>>();
    let player_screen_base_gate_lamp_signatures = string_vec([
        "player_screen_base_gate_micro_lamps",
        "player_screen_base_gate_hot_lamps_suppressed",
    ]);
    let player_screen_base_gate_lamp_count = player_screen_base_gate_lamp_samples.len();
    let player_screen_base_gate_lamps_per_gate = 2usize;
    let player_screen_base_gate_lamp_width_px = 6usize;
    let player_screen_base_gate_lamp_height_px = 2usize;
    let player_screen_base_gate_lamp_pixel_budget = player_screen_base_gate_lamp_count
        * player_screen_base_gate_lamps_per_gate
        * player_screen_base_gate_lamp_width_px
        * player_screen_base_gate_lamp_height_px;
    let player_screen_base_gate_hot_lamp_pixel_budget = 0usize;
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
    let lower_secondary_beacon_art_deemphasis_gate = lower_secondary_beacon_art_samples
        == vec![
            json!({
                "tile": "16,24",
                "role": "beacon_lane",
                "signature": "painted_lane_chevrons",
            }),
            json!({
                "tile": "16,23",
                "role": "beacon_lane",
                "signature": "lane_power_pylons",
            }),
        ]
        && lower_secondary_beacon_art_pixel_budget <= 64
        && lower_secondary_beacon_art_signatures
            .iter()
            .any(|signature| signature == "lower_secondary_beacon_micro_chevrons")
        && lower_secondary_beacon_art_signatures
            .iter()
            .any(|signature| signature == "lower_secondary_beacon_power_pylons_capped");
    let secondary_beacon_capture_ring_gate = secondary_beacon_capture_ring_samples
        == vec![
            json!({
                "tile": "16,24",
                "role": "beacon_ring",
                "signature": "beacon_capture_rings",
            }),
            json!({
                "tile": "9,16",
                "role": "beacon_ring",
                "signature": "beacon_capture_rings",
            }),
            json!({
                "tile": "24,16",
                "role": "beacon_ring",
                "signature": "beacon_capture_rings",
            }),
        ]
        && secondary_beacon_capture_ring_count == 3
        && secondary_beacon_capture_ring_cues_per_ring == 4
        && secondary_beacon_capture_ring_cue_width_px == 8
        && secondary_beacon_capture_ring_cue_height_px == 2
        && secondary_beacon_capture_ring_pixel_budget <= 192
        && secondary_beacon_capture_ring_signatures
            .iter()
            .any(|signature| signature == "secondary_beacon_capture_micro_cues");
    let player_screen_active_beacon_capture_ring_gate =
        player_screen_active_beacon_capture_ring_samples
            == vec![json!({
                "tile": "16,9",
                "role": "beacon_ring",
                "signature": "beacon_capture_rings",
            })]
            && player_screen_active_beacon_capture_ring_count == 1
            && player_screen_active_beacon_capture_ring_ticks_per_ring == 4
            && player_screen_active_beacon_capture_ring_tick_width_px == 10
            && player_screen_active_beacon_capture_ring_tick_height_px == 2
            && player_screen_active_beacon_capture_ring_pixel_budget <= 80
            && player_screen_active_beacon_capture_ring_signatures
                .iter()
                .any(|signature| signature == "player_screen_active_beacon_capture_micro_ticks");
    let player_screen_resource_crystal_shard_gate = player_screen_resource_crystal_shard_samples
        == vec![
            json!({
                "tile": "12,16",
                "role": "resource_crystal",
                "signature": "flux_crystal_shards",
            }),
            json!({
                "tile": "21,16",
                "role": "resource_crystal",
                "signature": "flux_crystal_shards",
            }),
        ]
        && player_screen_resource_crystal_shard_count == 2
        && player_screen_resource_crystal_shards_per_cluster == 4
        && player_screen_resource_crystal_shard_width_px == 4
        && player_screen_resource_crystal_shard_height_px == 2
        && player_screen_resource_crystal_shard_pixel_budget <= 64
        && player_screen_resource_crystal_hot_shard_pixel_budget == 0
        && player_screen_resource_crystal_shard_signatures
            .iter()
            .any(|signature| signature == "player_screen_resource_crystal_micro_shards")
        && player_screen_resource_crystal_shard_signatures
            .iter()
            .any(|signature| signature == "player_screen_resource_crystal_hot_shards_suppressed");
    let player_screen_resource_cluster_sparkle_gate = player_screen_resource_cluster_sparkle_samples
        == vec![
            json!({
                "tile": "12,16",
                "role": "resource_cluster",
                "signature": "crystal_shadow_sparkles",
            }),
            json!({
                "tile": "21,16",
                "role": "resource_cluster",
                "signature": "crystal_shadow_sparkles",
            }),
        ]
        && player_screen_resource_cluster_sparkle_count == 2
        && player_screen_resource_cluster_sparkles_per_cluster == 4
        && player_screen_resource_cluster_sparkle_width_px == 4
        && player_screen_resource_cluster_sparkle_height_px == 2
        && player_screen_resource_cluster_sparkle_pixel_budget <= 64
        && player_screen_resource_cluster_hot_glint_pixel_budget == 0
        && player_screen_resource_cluster_sparkle_signatures
            .iter()
            .any(|signature| signature == "player_screen_resource_cluster_micro_sparkles")
        && player_screen_resource_cluster_sparkle_signatures
            .iter()
            .any(|signature| signature == "player_screen_resource_cluster_hot_glints_suppressed");
    let player_screen_command_core_art_gate = player_screen_command_core_art_samples
        == vec![
            json!({
                "tile": "8,8",
                "role": "command_core",
                "signature": "lit_window_rows",
            }),
            json!({
                "tile": "25,8",
                "role": "command_core",
                "signature": "lit_window_rows",
            }),
            json!({
                "tile": "25,25",
                "role": "command_core",
                "signature": "lit_window_rows",
            }),
            json!({
                "tile": "8,25",
                "role": "command_core",
                "signature": "lit_window_rows",
            }),
        ]
        && player_screen_command_core_art_count == 4
        && player_screen_command_core_art_ticks_per_core == 4
        && player_screen_command_core_art_tick_width_px == 6
        && player_screen_command_core_art_tick_height_px == 2
        && player_screen_command_core_art_pixel_budget <= 192
        && player_screen_command_core_hot_facade_pixel_budget == 0
        && player_screen_command_core_art_signatures
            .iter()
            .any(|signature| signature == "player_screen_command_core_art_micro_ticks")
        && player_screen_command_core_art_signatures
            .iter()
            .any(|signature| signature == "player_screen_command_core_hot_facade_suppressed");
    let player_screen_base_gate_lamp_gate = player_screen_base_gate_lamp_samples
        == vec![
            json!({
                "tile": "8,9",
                "role": "base_gate",
                "signature": "base_gate_lamps",
            }),
            json!({
                "tile": "25,9",
                "role": "base_gate",
                "signature": "base_gate_lamps",
            }),
            json!({
                "tile": "25,24",
                "role": "base_gate",
                "signature": "base_gate_lamps",
            }),
            json!({
                "tile": "8,24",
                "role": "base_gate",
                "signature": "base_gate_lamps",
            }),
        ]
        && player_screen_base_gate_lamp_count == 4
        && player_screen_base_gate_lamps_per_gate == 2
        && player_screen_base_gate_lamp_width_px == 6
        && player_screen_base_gate_lamp_height_px == 2
        && player_screen_base_gate_lamp_pixel_budget <= 96
        && player_screen_base_gate_hot_lamp_pixel_budget == 0
        && player_screen_base_gate_lamp_signatures
            .iter()
            .any(|signature| signature == "player_screen_base_gate_micro_lamps")
        && player_screen_base_gate_lamp_signatures
            .iter()
            .any(|signature| signature == "player_screen_base_gate_hot_lamps_suppressed");
    let authored_map_art_gate = terrain_material_gate
        && terrain_material_depth_gate
        && building_facade_gate
        && map_landmark_detail_gate
        && runtime_actor_depth_gate
        && lower_secondary_beacon_art_deemphasis_gate
        && secondary_beacon_capture_ring_gate
        && player_screen_active_beacon_capture_ring_gate
        && player_screen_resource_crystal_shard_gate
        && player_screen_resource_cluster_sparkle_gate
        && player_screen_command_core_art_gate
        && player_screen_base_gate_lamp_gate;
    let green = authored_map_art_gate;

    json!({
        "contract_version": TRNM_RTS_EVIDENCE_FIRST_CONTACT_ART_READABILITY_CONTRACT,
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
        "lower_secondary_beacon_art_samples": lower_secondary_beacon_art_samples,
        "lower_secondary_beacon_art_sample_count": lower_secondary_beacon_art_sample_count,
        "lower_secondary_beacon_art_pixel_budget": lower_secondary_beacon_art_pixel_budget,
        "lower_secondary_beacon_art_signatures": lower_secondary_beacon_art_signatures,
        "lower_secondary_beacon_art_deemphasis_gate": lower_secondary_beacon_art_deemphasis_gate,
        "secondary_beacon_capture_ring_samples": secondary_beacon_capture_ring_samples,
        "secondary_beacon_capture_ring_count": secondary_beacon_capture_ring_count,
        "secondary_beacon_capture_ring_cues_per_ring": secondary_beacon_capture_ring_cues_per_ring,
        "secondary_beacon_capture_ring_cue_count": secondary_beacon_capture_ring_cue_count,
        "secondary_beacon_capture_ring_cue_width_px": secondary_beacon_capture_ring_cue_width_px,
        "secondary_beacon_capture_ring_cue_height_px": secondary_beacon_capture_ring_cue_height_px,
        "secondary_beacon_capture_ring_pixel_budget": secondary_beacon_capture_ring_pixel_budget,
        "secondary_beacon_capture_ring_signatures": secondary_beacon_capture_ring_signatures,
        "secondary_beacon_capture_ring_gate": secondary_beacon_capture_ring_gate,
        "player_screen_active_beacon_capture_ring_samples": player_screen_active_beacon_capture_ring_samples,
        "player_screen_active_beacon_capture_ring_count": player_screen_active_beacon_capture_ring_count,
        "player_screen_active_beacon_capture_ring_ticks_per_ring": player_screen_active_beacon_capture_ring_ticks_per_ring,
        "player_screen_active_beacon_capture_ring_tick_width_px": player_screen_active_beacon_capture_ring_tick_width_px,
        "player_screen_active_beacon_capture_ring_tick_height_px": player_screen_active_beacon_capture_ring_tick_height_px,
        "player_screen_active_beacon_capture_ring_pixel_budget": player_screen_active_beacon_capture_ring_pixel_budget,
        "player_screen_active_beacon_capture_ring_signatures": player_screen_active_beacon_capture_ring_signatures,
        "player_screen_active_beacon_capture_ring_gate": player_screen_active_beacon_capture_ring_gate,
        "player_screen_resource_crystal_shard_samples": player_screen_resource_crystal_shard_samples,
        "player_screen_resource_crystal_shard_count": player_screen_resource_crystal_shard_count,
        "player_screen_resource_crystal_shards_per_cluster": player_screen_resource_crystal_shards_per_cluster,
        "player_screen_resource_crystal_shard_width_px": player_screen_resource_crystal_shard_width_px,
        "player_screen_resource_crystal_shard_height_px": player_screen_resource_crystal_shard_height_px,
        "player_screen_resource_crystal_shard_pixel_budget": player_screen_resource_crystal_shard_pixel_budget,
        "player_screen_resource_crystal_hot_shard_pixel_budget": player_screen_resource_crystal_hot_shard_pixel_budget,
        "player_screen_resource_crystal_shard_signatures": player_screen_resource_crystal_shard_signatures,
        "player_screen_resource_crystal_shard_gate": player_screen_resource_crystal_shard_gate,
        "player_screen_resource_cluster_sparkle_samples": player_screen_resource_cluster_sparkle_samples,
        "player_screen_resource_cluster_sparkle_count": player_screen_resource_cluster_sparkle_count,
        "player_screen_resource_cluster_sparkles_per_cluster": player_screen_resource_cluster_sparkles_per_cluster,
        "player_screen_resource_cluster_sparkle_width_px": player_screen_resource_cluster_sparkle_width_px,
        "player_screen_resource_cluster_sparkle_height_px": player_screen_resource_cluster_sparkle_height_px,
        "player_screen_resource_cluster_sparkle_pixel_budget": player_screen_resource_cluster_sparkle_pixel_budget,
        "player_screen_resource_cluster_hot_glint_pixel_budget": player_screen_resource_cluster_hot_glint_pixel_budget,
        "player_screen_resource_cluster_sparkle_signatures": player_screen_resource_cluster_sparkle_signatures,
        "player_screen_resource_cluster_sparkle_gate": player_screen_resource_cluster_sparkle_gate,
        "player_screen_command_core_art_samples": player_screen_command_core_art_samples,
        "player_screen_command_core_art_count": player_screen_command_core_art_count,
        "player_screen_command_core_art_ticks_per_core": player_screen_command_core_art_ticks_per_core,
        "player_screen_command_core_art_tick_width_px": player_screen_command_core_art_tick_width_px,
        "player_screen_command_core_art_tick_height_px": player_screen_command_core_art_tick_height_px,
        "player_screen_command_core_art_pixel_budget": player_screen_command_core_art_pixel_budget,
        "player_screen_command_core_hot_facade_pixel_budget": player_screen_command_core_hot_facade_pixel_budget,
        "player_screen_command_core_art_signatures": player_screen_command_core_art_signatures,
        "player_screen_command_core_art_gate": player_screen_command_core_art_gate,
        "player_screen_base_gate_lamp_samples": player_screen_base_gate_lamp_samples,
        "player_screen_base_gate_lamp_count": player_screen_base_gate_lamp_count,
        "player_screen_base_gate_lamps_per_gate": player_screen_base_gate_lamps_per_gate,
        "player_screen_base_gate_lamp_width_px": player_screen_base_gate_lamp_width_px,
        "player_screen_base_gate_lamp_height_px": player_screen_base_gate_lamp_height_px,
        "player_screen_base_gate_lamp_pixel_budget": player_screen_base_gate_lamp_pixel_budget,
        "player_screen_base_gate_hot_lamp_pixel_budget": player_screen_base_gate_hot_lamp_pixel_budget,
        "player_screen_base_gate_lamp_signatures": player_screen_base_gate_lamp_signatures,
        "player_screen_base_gate_lamp_gate": player_screen_base_gate_lamp_gate,
        "authored_map_art_gate": authored_map_art_gate,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_contact_art_readability_helpers_preserve_authored_art_contracts() {
        let guard = first_contact_art_readability_guard();

        assert_eq!(
            guard.get("contract_version").and_then(Value::as_str),
            Some(TRNM_RTS_EVIDENCE_FIRST_CONTACT_ART_READABILITY_CONTRACT)
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
            guard.get("lower_secondary_beacon_art_samples").cloned(),
            Some(json!([
                {
                    "tile": "16,24",
                    "role": "beacon_lane",
                    "signature": "painted_lane_chevrons",
                },
                {
                    "tile": "16,23",
                    "role": "beacon_lane",
                    "signature": "lane_power_pylons",
                }
            ]))
        );
        assert_eq!(
            guard
                .get("lower_secondary_beacon_art_pixel_budget")
                .and_then(Value::as_u64),
            Some(64)
        );
        assert_eq!(
            guard.get("secondary_beacon_capture_ring_samples").cloned(),
            Some(json!([
                {
                    "tile": "16,24",
                    "role": "beacon_ring",
                    "signature": "beacon_capture_rings",
                },
                {
                    "tile": "9,16",
                    "role": "beacon_ring",
                    "signature": "beacon_capture_rings",
                },
                {
                    "tile": "24,16",
                    "role": "beacon_ring",
                    "signature": "beacon_capture_rings",
                }
            ]))
        );
        assert_eq!(
            guard
                .get("secondary_beacon_capture_ring_cue_count")
                .and_then(Value::as_u64),
            Some(12)
        );
        assert_eq!(
            guard
                .get("secondary_beacon_capture_ring_cue_width_px")
                .and_then(Value::as_u64),
            Some(8)
        );
        assert_eq!(
            guard
                .get("secondary_beacon_capture_ring_cue_height_px")
                .and_then(Value::as_u64),
            Some(2)
        );
        assert_eq!(
            guard
                .get("secondary_beacon_capture_ring_pixel_budget")
                .and_then(Value::as_u64),
            Some(192)
        );
        assert_eq!(
            guard
                .get("player_screen_active_beacon_capture_ring_samples")
                .cloned(),
            Some(json!([
                {
                    "tile": "16,9",
                    "role": "beacon_ring",
                    "signature": "beacon_capture_rings",
                }
            ]))
        );
        assert_eq!(
            guard
                .get("player_screen_active_beacon_capture_ring_ticks_per_ring")
                .and_then(Value::as_u64),
            Some(4)
        );
        assert_eq!(
            guard
                .get("player_screen_active_beacon_capture_ring_tick_width_px")
                .and_then(Value::as_u64),
            Some(10)
        );
        assert_eq!(
            guard
                .get("player_screen_active_beacon_capture_ring_tick_height_px")
                .and_then(Value::as_u64),
            Some(2)
        );
        assert_eq!(
            guard
                .get("player_screen_active_beacon_capture_ring_pixel_budget")
                .and_then(Value::as_u64),
            Some(80)
        );
        assert_eq!(
            guard
                .get("player_screen_resource_crystal_shard_samples")
                .cloned(),
            Some(json!([
                {"tile": "12,16", "role": "resource_crystal", "signature": "flux_crystal_shards"},
                {"tile": "21,16", "role": "resource_crystal", "signature": "flux_crystal_shards"}
            ]))
        );
        assert_eq!(
            guard
                .get("player_screen_resource_crystal_shard_count")
                .and_then(Value::as_u64),
            Some(2)
        );
        assert_eq!(
            guard
                .get("player_screen_resource_crystal_shards_per_cluster")
                .and_then(Value::as_u64),
            Some(4)
        );
        assert_eq!(
            guard
                .get("player_screen_resource_crystal_shard_width_px")
                .and_then(Value::as_u64),
            Some(4)
        );
        assert_eq!(
            guard
                .get("player_screen_resource_crystal_shard_height_px")
                .and_then(Value::as_u64),
            Some(2)
        );
        assert_eq!(
            guard
                .get("player_screen_resource_crystal_shard_pixel_budget")
                .and_then(Value::as_u64),
            Some(64)
        );
        assert_eq!(
            guard
                .get("player_screen_resource_crystal_hot_shard_pixel_budget")
                .and_then(Value::as_u64),
            Some(0)
        );
        assert_eq!(
            guard
                .get("player_screen_resource_crystal_shard_signatures")
                .and_then(Value::as_array)
                .map(|signatures| signatures.iter().any(|signature| {
                    signature.as_str() == Some("player_screen_resource_crystal_micro_shards")
                }) && signatures.iter().any(|signature| {
                    signature.as_str()
                        == Some("player_screen_resource_crystal_hot_shards_suppressed")
                })),
            Some(true)
        );
        assert_eq!(
            guard
                .get("player_screen_resource_cluster_sparkle_samples")
                .cloned(),
            Some(json!([
                {"tile": "12,16", "role": "resource_cluster", "signature": "crystal_shadow_sparkles"},
                {"tile": "21,16", "role": "resource_cluster", "signature": "crystal_shadow_sparkles"}
            ]))
        );
        assert_eq!(
            guard
                .get("player_screen_resource_cluster_sparkle_count")
                .and_then(Value::as_u64),
            Some(2)
        );
        assert_eq!(
            guard
                .get("player_screen_resource_cluster_sparkles_per_cluster")
                .and_then(Value::as_u64),
            Some(4)
        );
        assert_eq!(
            guard
                .get("player_screen_resource_cluster_sparkle_width_px")
                .and_then(Value::as_u64),
            Some(4)
        );
        assert_eq!(
            guard
                .get("player_screen_resource_cluster_sparkle_height_px")
                .and_then(Value::as_u64),
            Some(2)
        );
        assert_eq!(
            guard
                .get("player_screen_resource_cluster_sparkle_pixel_budget")
                .and_then(Value::as_u64),
            Some(64)
        );
        assert_eq!(
            guard
                .get("player_screen_resource_cluster_hot_glint_pixel_budget")
                .and_then(Value::as_u64),
            Some(0)
        );
        assert_eq!(
            guard
                .get("player_screen_resource_cluster_sparkle_signatures")
                .and_then(Value::as_array)
                .map(|signatures| signatures.iter().any(|signature| {
                    signature.as_str() == Some("player_screen_resource_cluster_micro_sparkles")
                }) && signatures.iter().any(|signature| {
                    signature.as_str()
                        == Some("player_screen_resource_cluster_hot_glints_suppressed")
                })),
            Some(true)
        );
        assert_eq!(
            guard.get("player_screen_command_core_art_samples").cloned(),
            Some(json!([
                {"tile": "8,8", "role": "command_core", "signature": "lit_window_rows"},
                {"tile": "25,8", "role": "command_core", "signature": "lit_window_rows"},
                {"tile": "25,25", "role": "command_core", "signature": "lit_window_rows"},
                {"tile": "8,25", "role": "command_core", "signature": "lit_window_rows"}
            ]))
        );
        assert_eq!(
            guard
                .get("player_screen_command_core_art_count")
                .and_then(Value::as_u64),
            Some(4)
        );
        assert_eq!(
            guard
                .get("player_screen_command_core_art_ticks_per_core")
                .and_then(Value::as_u64),
            Some(4)
        );
        assert_eq!(
            guard
                .get("player_screen_command_core_art_tick_width_px")
                .and_then(Value::as_u64),
            Some(6)
        );
        assert_eq!(
            guard
                .get("player_screen_command_core_art_tick_height_px")
                .and_then(Value::as_u64),
            Some(2)
        );
        assert_eq!(
            guard
                .get("player_screen_command_core_art_pixel_budget")
                .and_then(Value::as_u64),
            Some(192)
        );
        assert_eq!(
            guard
                .get("player_screen_command_core_hot_facade_pixel_budget")
                .and_then(Value::as_u64),
            Some(0)
        );
        assert_eq!(
            guard
                .get("player_screen_command_core_art_signatures")
                .and_then(Value::as_array)
                .map(|signatures| signatures.iter().any(|signature| {
                    signature.as_str() == Some("player_screen_command_core_art_micro_ticks")
                }) && signatures.iter().any(|signature| {
                    signature.as_str() == Some("player_screen_command_core_hot_facade_suppressed")
                })),
            Some(true)
        );
        assert_eq!(
            guard.get("player_screen_base_gate_lamp_samples").cloned(),
            Some(json!([
                {"tile": "8,9", "role": "base_gate", "signature": "base_gate_lamps"},
                {"tile": "25,9", "role": "base_gate", "signature": "base_gate_lamps"},
                {"tile": "25,24", "role": "base_gate", "signature": "base_gate_lamps"},
                {"tile": "8,24", "role": "base_gate", "signature": "base_gate_lamps"}
            ]))
        );
        assert_eq!(
            guard
                .get("player_screen_base_gate_lamp_count")
                .and_then(Value::as_u64),
            Some(4)
        );
        assert_eq!(
            guard
                .get("player_screen_base_gate_lamps_per_gate")
                .and_then(Value::as_u64),
            Some(2)
        );
        assert_eq!(
            guard
                .get("player_screen_base_gate_lamp_width_px")
                .and_then(Value::as_u64),
            Some(6)
        );
        assert_eq!(
            guard
                .get("player_screen_base_gate_lamp_height_px")
                .and_then(Value::as_u64),
            Some(2)
        );
        assert_eq!(
            guard
                .get("player_screen_base_gate_lamp_pixel_budget")
                .and_then(Value::as_u64),
            Some(96)
        );
        assert_eq!(
            guard
                .get("player_screen_base_gate_hot_lamp_pixel_budget")
                .and_then(Value::as_u64),
            Some(0)
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
            "lower_secondary_beacon_art_deemphasis_gate",
            "secondary_beacon_capture_ring_gate",
            "player_screen_resource_crystal_shard_gate",
            "player_screen_resource_cluster_sparkle_gate",
            "player_screen_command_core_art_gate",
            "player_screen_base_gate_lamp_gate",
            "authored_map_art_gate",
        ] {
            assert_eq!(guard.get(gate).and_then(Value::as_bool), Some(true));
        }
    }
}
