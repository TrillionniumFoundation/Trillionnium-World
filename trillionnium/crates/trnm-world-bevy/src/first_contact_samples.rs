#![cfg(not(target_os = "android"))]

pub(crate) type TileRoleSignature = ((i32, i32), &'static str, &'static str);
pub(crate) type AtlasSample = ((i32, i32), &'static str, &'static str, &'static str, u32);

pub(crate) fn silhouette_unit_samples() -> Vec<TileRoleSignature> {
    vec![
        ((14, 11), "worker", "cargo_pack"),
        ((15, 11), "scout", "sensor_mast"),
        ((15, 12), "warden", "shield_plate"),
        ((17, 12), "relay", "relay_courier"),
    ]
}

pub(crate) fn silhouette_structure_samples() -> Vec<TileRoleSignature> {
    vec![
        ((8, 8), "command_core", "stepped_roof_core"),
        ((25, 8), "command_core", "stepped_roof_core"),
        ((25, 25), "command_core", "stepped_roof_core"),
        ((8, 25), "command_core", "stepped_roof_core"),
        ((11, 8), "relay", "tall_signal_mast"),
        ((22, 25), "relay", "tall_signal_mast"),
        ((16, 9), "beacon", "vertical_beacon_spire"),
        ((16, 24), "beacon", "vertical_beacon_spire"),
        ((9, 16), "beacon", "vertical_beacon_spire"),
        ((24, 16), "beacon", "vertical_beacon_spire"),
    ]
}

pub(crate) fn silhouette_terrain_samples() -> Vec<TileRoleSignature> {
    vec![
        ((8, 8), "base_pad", "base_corner_frame"),
        ((25, 8), "base_pad", "base_corner_frame"),
        ((25, 25), "base_pad", "base_corner_frame"),
        ((8, 25), "base_pad", "base_corner_frame"),
        ((12, 16), "resource_zone", "flux_glint_cluster"),
        ((21, 16), "resource_zone", "flux_glint_cluster"),
        ((16, 9), "objective_lane", "beacon_lane_rim"),
        ((16, 24), "objective_lane", "beacon_lane_rim"),
        ((16, 16), "central_basin", "basin_cross_rim"),
    ]
}

pub(crate) fn art_terrain_samples() -> Vec<TileRoleSignature> {
    vec![
        ((8, 8), "base_concrete", "foundation_panel_seams"),
        ((25, 8), "base_concrete", "foundation_panel_seams"),
        ((25, 25), "base_concrete", "foundation_panel_seams"),
        ((8, 25), "base_concrete", "foundation_panel_seams"),
        ((12, 16), "resource_crystal", "flux_crystal_shards"),
        ((21, 16), "resource_crystal", "flux_crystal_shards"),
        ((16, 9), "beacon_lane", "painted_lane_chevrons"),
        ((16, 24), "beacon_lane", "painted_lane_chevrons"),
        ((16, 16), "basin_floor", "cracked_plaza_cross"),
    ]
}

pub(crate) fn art_building_samples() -> Vec<TileRoleSignature> {
    vec![
        ((8, 8), "command_core", "lit_window_rows"),
        ((25, 8), "command_core", "lit_window_rows"),
        ((25, 25), "command_core", "lit_window_rows"),
        ((8, 25), "command_core", "lit_window_rows"),
        ((11, 8), "relay", "antenna_band_panels"),
        ((22, 25), "relay", "antenna_band_panels"),
        ((16, 9), "beacon", "glowing_spire_panels"),
        ((16, 24), "beacon", "glowing_spire_panels"),
        ((9, 16), "beacon", "glowing_spire_panels"),
        ((24, 16), "beacon", "glowing_spire_panels"),
    ]
}

pub(crate) fn art_landmark_samples() -> Vec<TileRoleSignature> {
    vec![
        ((8, 9), "base_gate", "base_gate_lamps"),
        ((25, 9), "base_gate", "base_gate_lamps"),
        ((25, 24), "base_gate", "base_gate_lamps"),
        ((8, 24), "base_gate", "base_gate_lamps"),
        ((12, 16), "resource_cluster", "crystal_shadow_sparkles"),
        ((21, 16), "resource_cluster", "crystal_shadow_sparkles"),
        ((16, 10), "beacon_lane", "lane_power_pylons"),
        ((16, 23), "beacon_lane", "lane_power_pylons"),
        ((14, 14), "basin_scar", "crater_scuff_marks"),
        ((18, 18), "basin_scar", "crater_scuff_marks"),
        ((11, 9), "relay_cable", "relay_ground_cables"),
        ((22, 24), "relay_cable", "relay_ground_cables"),
        ((16, 9), "beacon_ring", "beacon_capture_rings"),
        ((16, 24), "beacon_ring", "beacon_capture_rings"),
        ((9, 16), "beacon_ring", "beacon_capture_rings"),
        ((24, 16), "beacon_ring", "beacon_capture_rings"),
    ]
}

pub(crate) fn terrain_material_depth_signatures() -> Vec<&'static str> {
    vec![
        "terrain_foundation_beveled_edges",
        "terrain_crystal_cast_shadows",
        "terrain_lane_recessed_rails",
        "terrain_basin_fracture_shadows",
    ]
}

pub(crate) fn runtime_actor_depth_signatures() -> Vec<&'static str> {
    vec![
        "runtime_structure_roof_rim",
        "runtime_structure_side_shadow",
        "runtime_command_window_pips",
        "runtime_relay_mast_braces",
        "runtime_beacon_core_glow_rungs",
    ]
}

pub(crate) fn animation_cycle_samples() -> Vec<TileRoleSignature> {
    vec![
        ((12, 16), "worker", "harvest_tool_swing_frame"),
        ((10, 12), "worker", "carry_bob_frame"),
        ((14, 11), "worker", "locomotion_footfall_pair"),
        ((25, 8), "scout", "sensor_sweep_arc"),
        ((24, 10), "scout", "turn_arc_frame"),
        ((8, 25), "warden", "shield_charge_flash"),
        ((10, 23), "warden", "attack_recoil_ticks"),
        ((11, 8), "relay", "relay_packet_pulse"),
        ((8, 8), "command_core", "training_tick_lane"),
        ((9, 9), "command_core", "spawn_door_open_frame"),
        ((11, 8), "relay_structure", "construction_spark_ladder"),
        ((16, 9), "beacon", "capture_pulse_frame"),
        ((16, 10), "beacon", "rally_flag_flutter"),
    ]
}

pub(crate) fn animation_frame_richness_signatures() -> Vec<&'static str> {
    vec![
        "animation_secondary_pose_offsets",
        "animation_contact_smear_ticks",
        "animation_structure_shutter_frames",
        "animation_objective_afterglow_frames",
    ]
}

pub(crate) fn unit_animation_role(role: &str) -> bool {
    matches!(role, "worker" | "scout" | "warden" | "relay")
}

pub(crate) fn structure_animation_role(role: &str) -> bool {
    matches!(role, "command_core" | "relay_structure")
}

pub(crate) fn atlas_asset_samples() -> Vec<AtlasSample> {
    vec![
        (
            (8, 8),
            "terrain_tile",
            "tile_stone",
            "base_pad_stone_frame",
            1,
        ),
        (
            (25, 8),
            "terrain_tile",
            "tile_stone",
            "base_pad_stone_frame",
            1,
        ),
        (
            (12, 16),
            "terrain_tile",
            "tile_water",
            "flux_pool_ripple_frame",
            1,
        ),
        (
            (21, 16),
            "terrain_tile",
            "tile_water",
            "flux_pool_ripple_frame",
            1,
        ),
        (
            (16, 9),
            "terrain_tile",
            "tile_road",
            "beacon_lane_tile_frame",
            1,
        ),
        (
            (16, 16),
            "terrain_tile",
            "tile_floor",
            "basin_floor_tile_frame",
            1,
        ),
        (
            (14, 11),
            "unit_sprite",
            "actor_player_walk_south_1",
            "worker_walk_atlas_frame",
            2,
        ),
        (
            (15, 11),
            "unit_sprite",
            "actor_player_walk_east_1",
            "scout_stride_atlas_frame",
            2,
        ),
        (
            (15, 12),
            "unit_sprite",
            "actor_enemy_attack",
            "warden_attack_atlas_frame",
            2,
        ),
        (
            (17, 12),
            "unit_sprite",
            "actor_mentor_talk",
            "relay_operator_atlas_frame",
            2,
        ),
        (
            (8, 8),
            "structure_sprite",
            "prop_workbench",
            "command_core_workbench_frame",
            2,
        ),
        (
            (25, 8),
            "structure_sprite",
            "prop_market_stall",
            "command_core_stall_frame",
            2,
        ),
        (
            (11, 8),
            "structure_sprite",
            "prop_signpost",
            "relay_mast_signpost_frame",
            2,
        ),
        (
            (22, 25),
            "structure_sprite",
            "prop_banner",
            "relay_banner_frame",
            2,
        ),
        (
            (16, 9),
            "objective_sprite",
            "marker_objective",
            "beacon_objective_atlas_frame",
            2,
        ),
        (
            (16, 24),
            "objective_sprite",
            "marker_interaction",
            "beacon_interaction_atlas_frame",
            2,
        ),
    ]
}

pub(crate) fn atlas_frame_family_samples() -> Vec<AtlasSample> {
    vec![
        (
            (4, 14),
            "worker_unit_family",
            "actor_worker_idle",
            "worker_idle_frame_family",
            1,
        ),
        (
            (4, 16),
            "worker_unit_family",
            "actor_worker_carry",
            "worker_carry_frame_family",
            1,
        ),
        (
            (4, 18),
            "scout_unit_family",
            "actor_player_walk_east_1",
            "scout_stride_east_frame_family",
            2,
        ),
        (
            (4, 20),
            "scout_unit_family",
            "actor_player_walk_east_2",
            "scout_stride_east_alt_frame_family",
            2,
        ),
        (
            (29, 14),
            "warden_unit_family",
            "actor_guard_idle",
            "warden_guard_idle_frame_family",
            1,
        ),
        (
            (29, 16),
            "warden_unit_family",
            "actor_guard_attack",
            "warden_guard_attack_frame_family",
            1,
        ),
        (
            (29, 18),
            "relay_unit_family",
            "actor_mentor_talk",
            "relay_operator_talk_frame_family",
            1,
        ),
        (
            (4, 4),
            "command_core_structure_family",
            "model_town_hall",
            "command_core_town_hall_frame_family",
            1,
        ),
        (
            (6, 4),
            "command_core_structure_family",
            "model_training_hall",
            "command_core_training_hall_frame_family",
            1,
        ),
        (
            (27, 4),
            "relay_structure_family",
            "model_waygate",
            "relay_waygate_frame_family",
            1,
        ),
        (
            (29, 4),
            "relay_structure_family",
            "prop_banner",
            "relay_banner_frame_family",
            1,
        ),
        (
            (29, 22),
            "beacon_objective_family",
            "marker_objective",
            "beacon_objective_frame_family",
            1,
        ),
        (
            (29, 24),
            "beacon_objective_family",
            "rts_command_destination_marker",
            "beacon_destination_marker_frame_family",
            1,
        ),
        (
            (29, 26),
            "beacon_objective_family",
            "marker_interaction",
            "beacon_interaction_frame_family",
            1,
        ),
    ]
}

pub(crate) fn atlas_family_gallery_lane(tile: (i32, i32)) -> &'static str {
    if tile.1 <= 6 {
        "north_gallery"
    } else if tile.0 <= 6 {
        "west_gallery"
    } else if tile.0 >= 27 {
        "east_gallery"
    } else {
        "field_gallery"
    }
}

pub(crate) fn atlas_family_busy_core_tile(tile: (i32, i32)) -> bool {
    (12..=19).contains(&tile.0) && (9..=13).contains(&tile.1)
}

pub(crate) fn atlas_family_lower_lane_tile(tile: (i32, i32)) -> bool {
    tile.0 >= 27 && tile.1 >= 22
}

pub(crate) fn atlas_runtime_depth_role(role: &str) -> bool {
    matches!(
        role,
        "unit_sprite"
            | "structure_sprite"
            | "objective_sprite"
            | "worker_unit_family"
            | "scout_unit_family"
            | "warden_unit_family"
            | "relay_unit_family"
            | "command_core_structure_family"
            | "relay_structure_family"
            | "beacon_objective_family"
    )
}

pub(crate) fn atlas_runtime_depth_signatures() -> Vec<&'static str> {
    vec![
        "atlas_unit_grounding_shadow",
        "atlas_structure_footprint_rim",
        "atlas_objective_capture_underlay",
        "atlas_lower_lane_depth_suppressed",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_contact_samples_preserve_art_motion_and_atlas_contracts() {
        assert_eq!(silhouette_unit_samples().len(), 4);
        assert_eq!(silhouette_structure_samples().len(), 10);
        assert_eq!(silhouette_terrain_samples().len(), 9);
        assert_eq!(art_terrain_samples().len(), 9);
        assert_eq!(art_building_samples().len(), 10);
        assert_eq!(art_landmark_samples().len(), 16);

        assert_eq!(
            terrain_material_depth_signatures(),
            vec![
                "terrain_foundation_beveled_edges",
                "terrain_crystal_cast_shadows",
                "terrain_lane_recessed_rails",
                "terrain_basin_fracture_shadows"
            ]
        );
        assert_eq!(
            runtime_actor_depth_signatures(),
            vec![
                "runtime_structure_roof_rim",
                "runtime_structure_side_shadow",
                "runtime_command_window_pips",
                "runtime_relay_mast_braces",
                "runtime_beacon_core_glow_rungs"
            ]
        );
        assert_eq!(animation_cycle_samples().len(), 13);
        assert!(unit_animation_role("worker"));
        assert!(structure_animation_role("relay_structure"));
        assert!(!unit_animation_role("beacon"));
        assert!(!structure_animation_role("beacon"));
        assert_eq!(
            animation_frame_richness_signatures(),
            vec![
                "animation_secondary_pose_offsets",
                "animation_contact_smear_ticks",
                "animation_structure_shutter_frames",
                "animation_objective_afterglow_frames"
            ]
        );

        assert_eq!(atlas_asset_samples().len(), 16);
        assert_eq!(atlas_frame_family_samples().len(), 14);
        assert_eq!(atlas_family_gallery_lane((4, 14)), "west_gallery");
        assert_eq!(atlas_family_gallery_lane((29, 24)), "east_gallery");
        assert!(atlas_family_busy_core_tile((16, 10)));
        assert!(atlas_family_lower_lane_tile((29, 24)));
        assert!(atlas_runtime_depth_role("beacon_objective_family"));
        assert!(!atlas_runtime_depth_role("terrain_tile"));
        assert_eq!(
            atlas_runtime_depth_signatures(),
            vec![
                "atlas_unit_grounding_shadow",
                "atlas_structure_footprint_rim",
                "atlas_objective_capture_underlay",
                "atlas_lower_lane_depth_suppressed"
            ]
        );
    }
}
