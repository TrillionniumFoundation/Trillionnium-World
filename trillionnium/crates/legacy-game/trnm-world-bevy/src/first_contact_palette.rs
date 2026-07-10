#![cfg(not(target_os = "android"))]

use crate::{
    classic_darken, classic_lighten, classic_mix_color,
    CLASSIC_FIRST_CONTACT_SECONDARY_TRACK_DARKEN_DENOMINATOR,
    CLASSIC_FIRST_CONTACT_SECONDARY_TRACK_DARKEN_NUMERATOR, CLASSIC_HUD_MUTED_TEXT_COLOR,
    CLASSIC_ISO_GOLD_COLOR, CLASSIC_RTS_CAMERA_SYNC_VIEWPORT_COLOR,
    CLASSIC_RTS_COMMAND_SURFACE_TARGET_COLOR, CLASSIC_RTS_DAMAGE_TICK_COLOR,
    CLASSIC_RTS_DEFENSE_READY_COLOR, CLASSIC_RTS_ENVIRONMENT_RESOURCE_GLINT_COLOR,
    CLASSIC_RTS_FIDELITY_ACTION_TRAIL_COLOR, CLASSIC_RTS_FIDELITY_NPC_ACTION_COLOR,
    CLASSIC_RTS_HARVEST_ANIMATION_CARRY_LOAD_COLOR, CLASSIC_RTS_HARVEST_NODE_COLOR,
    CLASSIC_RTS_MODEL_IDENTITY_BEACON_COLOR, CLASSIC_RTS_MODEL_IDENTITY_CORE_COLOR,
    CLASSIC_RTS_MODEL_IDENTITY_FACTION_COLOR, CLASSIC_RTS_MODEL_IDENTITY_RELAY_COLOR,
    CLASSIC_RTS_MODEL_IDENTITY_UNIT_COLOR, CLASSIC_RTS_OBJECTIVE_COLOR,
    CLASSIC_RTS_PRODUCT_LANE_COLOR, CLASSIC_RTS_PRODUCT_MAP_DENSITY_COLOR,
    CLASSIC_RTS_PRODUCT_MODEL_VOLUME_COLOR, CLASSIC_RTS_PRODUCT_UI_ACCENT_COLOR,
    CLASSIC_RTS_SCOUT_REVEAL_COLOR, CLASSIC_RTS_SELECTION_FEEDBACK_ATTACK_COLOR,
    CLASSIC_RTS_SELECTION_FEEDBACK_CONFIRM_COLOR, CLASSIC_RTS_STATUS_HEALTH_BAR_COLOR,
    CLASSIC_RTS_STATUS_MANA_BAR_COLOR, CLASSIC_RTS_STRUCTURE_FOUNDATION_SHADOW_COLOR,
    CLASSIC_RTS_STRUCTURE_REPAIR_BEAM_COLOR, CLASSIC_RTS_STRUCTURE_SCAFFOLD_COLOR,
    CLASSIC_RTS_TACTICAL_VIEWPORT_TILE_COLOR, CLASSIC_RTS_TECH_BASE_COLOR,
};
use trnm_rts_data::{
    first_contact_opening_loop_profile, RtsActorColorRole, RtsTacticalTrackProfile, RtsTerrainRole,
    RtsTerrainTileProfile, RtsVisualTelemetryColorRole,
};

pub(crate) fn owner_from_rts_data(owner: &str) -> &'static str {
    match owner {
        "Multi0" => "Multi0",
        "Multi1" => "Multi1",
        "Multi2" => "Multi2",
        "Multi3" => "Multi3",
        "Multi4" => "Multi4",
        "Multi5" => "Multi5",
        "Neutral" => "Neutral",
        _ => "Neutral",
    }
}

pub(crate) fn visual_telemetry_color(role: RtsVisualTelemetryColorRole) -> u32 {
    match role {
        RtsVisualTelemetryColorRole::Health => CLASSIC_RTS_STATUS_HEALTH_BAR_COLOR,
        RtsVisualTelemetryColorRole::Mana => CLASSIC_RTS_STATUS_MANA_BAR_COLOR,
        RtsVisualTelemetryColorRole::Attack => CLASSIC_RTS_SELECTION_FEEDBACK_ATTACK_COLOR,
        RtsVisualTelemetryColorRole::Confirm => CLASSIC_RTS_SELECTION_FEEDBACK_CONFIRM_COLOR,
        RtsVisualTelemetryColorRole::ActionTrail => CLASSIC_RTS_FIDELITY_ACTION_TRAIL_COLOR,
        RtsVisualTelemetryColorRole::NpcAction => CLASSIC_RTS_FIDELITY_NPC_ACTION_COLOR,
    }
}

pub(crate) fn primary_tactical_track(track: &RtsTacticalTrackProfile) -> bool {
    let opening = first_contact_opening_loop_profile();
    track.from_tile == opening.active_relay_tile
        && track.to_tile == opening.active_beacon_tile
        && track.color_role == RtsVisualTelemetryColorRole::ActionTrail
}

pub(crate) fn tactical_track_render_color(track: &RtsTacticalTrackProfile) -> u32 {
    let color = visual_telemetry_color(track.color_role);
    if primary_tactical_track(track) {
        color
    } else {
        classic_mix_color(
            color,
            CLASSIC_RTS_TACTICAL_VIEWPORT_TILE_COLOR,
            CLASSIC_FIRST_CONTACT_SECONDARY_TRACK_DARKEN_NUMERATOR,
            CLASSIC_FIRST_CONTACT_SECONDARY_TRACK_DARKEN_DENOMINATOR,
        )
    }
}

pub(crate) fn actor_color_role_color(role: RtsActorColorRole) -> u32 {
    match role {
        RtsActorColorRole::Worker => CLASSIC_RTS_HARVEST_ANIMATION_CARRY_LOAD_COLOR,
        RtsActorColorRole::Scout => CLASSIC_RTS_FIDELITY_ACTION_TRAIL_COLOR,
        RtsActorColorRole::Warden => CLASSIC_RTS_FIDELITY_NPC_ACTION_COLOR,
        RtsActorColorRole::Striker => CLASSIC_RTS_DAMAGE_TICK_COLOR,
        RtsActorColorRole::CommandCore => CLASSIC_RTS_TECH_BASE_COLOR,
        RtsActorColorRole::FluxRelay => CLASSIC_RTS_STRUCTURE_SCAFFOLD_COLOR,
        RtsActorColorRole::Objective => CLASSIC_RTS_OBJECTIVE_COLOR,
        RtsActorColorRole::Resource => CLASSIC_RTS_HARVEST_NODE_COLOR,
        RtsActorColorRole::MapDetail => CLASSIC_RTS_PRODUCT_UI_ACCENT_COLOR,
    }
}

pub(crate) fn tile_profile_color(terrain: RtsTerrainTileProfile) -> u32 {
    if !terrain.playable {
        return 0x111812;
    }
    let x = terrain.tile.x;
    let y = terrain.tile.y;
    let mut color = match terrain.role {
        RtsTerrainRole::Border => 0x111812,
        RtsTerrainRole::Lane => classic_darken(CLASSIC_RTS_PRODUCT_LANE_COLOR, 1, 4),
        RtsTerrainRole::CentralBasin => 0x203f39,
        RtsTerrainRole::BasePad => 0x243326,
        RtsTerrainRole::ResourceZone => 0x21392d,
        RtsTerrainRole::Field => {
            if (x + y) % 2 == 0 {
                0x18251d
            } else {
                0x1d2d22
            }
        }
    };
    let tile = (terrain.tile.x, terrain.tile.y);
    let surface_seed = trnm_rts_bevy_runtime::rts_runtime_terrain_seeds(tile).surface_seed;
    if surface_seed == 0 {
        color = classic_lighten(color, 1, 10);
    } else if surface_seed == 1 || surface_seed == 9 {
        color = classic_darken(color, 1, 10);
    }
    color
}

pub(crate) fn silhouette_unit_color(role: &str) -> u32 {
    match role {
        "worker" => CLASSIC_RTS_HARVEST_ANIMATION_CARRY_LOAD_COLOR,
        "scout" => CLASSIC_RTS_SCOUT_REVEAL_COLOR,
        "warden" => CLASSIC_RTS_DEFENSE_READY_COLOR,
        "relay" => CLASSIC_RTS_MODEL_IDENTITY_RELAY_COLOR,
        _ => CLASSIC_RTS_MODEL_IDENTITY_UNIT_COLOR,
    }
}

pub(crate) fn silhouette_structure_color(kind: &str) -> u32 {
    match kind {
        "command_core" => CLASSIC_RTS_MODEL_IDENTITY_CORE_COLOR,
        "relay" => CLASSIC_RTS_MODEL_IDENTITY_RELAY_COLOR,
        "beacon" => CLASSIC_RTS_MODEL_IDENTITY_BEACON_COLOR,
        _ => CLASSIC_RTS_PRODUCT_MODEL_VOLUME_COLOR,
    }
}

pub(crate) fn silhouette_terrain_color(kind: &str) -> u32 {
    match kind {
        "base_pad" => CLASSIC_RTS_STRUCTURE_FOUNDATION_SHADOW_COLOR,
        "resource_zone" => CLASSIC_RTS_ENVIRONMENT_RESOURCE_GLINT_COLOR,
        "objective_lane" => CLASSIC_RTS_OBJECTIVE_COLOR,
        "central_basin" => CLASSIC_RTS_PRODUCT_LANE_COLOR,
        _ => CLASSIC_RTS_PRODUCT_MAP_DENSITY_COLOR,
    }
}

pub(crate) fn art_terrain_color(role: &str) -> u32 {
    match role {
        "base_concrete" => CLASSIC_RTS_STRUCTURE_FOUNDATION_SHADOW_COLOR,
        "resource_crystal" => CLASSIC_RTS_ENVIRONMENT_RESOURCE_GLINT_COLOR,
        "beacon_lane" => CLASSIC_RTS_OBJECTIVE_COLOR,
        "basin_floor" => CLASSIC_RTS_PRODUCT_LANE_COLOR,
        _ => CLASSIC_RTS_PRODUCT_MAP_DENSITY_COLOR,
    }
}

pub(crate) fn art_building_color(role: &str) -> u32 {
    match role {
        "command_core" => CLASSIC_RTS_MODEL_IDENTITY_FACTION_COLOR,
        "relay" => CLASSIC_RTS_STRUCTURE_REPAIR_BEAM_COLOR,
        "beacon" => CLASSIC_RTS_ENVIRONMENT_RESOURCE_GLINT_COLOR,
        _ => CLASSIC_RTS_PRODUCT_UI_ACCENT_COLOR,
    }
}

pub(crate) fn art_landmark_color(role: &str) -> u32 {
    match role {
        "base_gate" => CLASSIC_RTS_MODEL_IDENTITY_FACTION_COLOR,
        "resource_cluster" => CLASSIC_RTS_ENVIRONMENT_RESOURCE_GLINT_COLOR,
        "beacon_lane" => CLASSIC_RTS_OBJECTIVE_COLOR,
        "basin_scar" => CLASSIC_RTS_PRODUCT_LANE_COLOR,
        "relay_cable" => CLASSIC_RTS_STRUCTURE_REPAIR_BEAM_COLOR,
        "beacon_ring" => CLASSIC_RTS_COMMAND_SURFACE_TARGET_COLOR,
        _ => CLASSIC_RTS_PRODUCT_UI_ACCENT_COLOR,
    }
}

pub(crate) fn gallery_muted_color(color: u32) -> u32 {
    classic_mix_color(color, CLASSIC_RTS_TACTICAL_VIEWPORT_TILE_COLOR, 4, 5)
}

pub(crate) fn atlas_family_slot_color(role: &str, lower_lane: bool) -> u32 {
    let source_color = match role {
        "worker_unit_family" | "command_core_structure_family" => CLASSIC_ISO_GOLD_COLOR,
        "scout_unit_family" | "relay_structure_family" => CLASSIC_RTS_CAMERA_SYNC_VIEWPORT_COLOR,
        "warden_unit_family" => CLASSIC_RTS_SELECTION_FEEDBACK_ATTACK_COLOR,
        "relay_unit_family" => CLASSIC_RTS_FIDELITY_NPC_ACTION_COLOR,
        "beacon_objective_family" => CLASSIC_RTS_COMMAND_SURFACE_TARGET_COLOR,
        _ => CLASSIC_HUD_MUTED_TEXT_COLOR,
    };
    let muted_color = gallery_muted_color(source_color);
    if lower_lane {
        classic_mix_color(muted_color, CLASSIC_RTS_TACTICAL_VIEWPORT_TILE_COLOR, 2, 3)
    } else {
        muted_color
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        CLASSIC_HUD_MUTED_TEXT_COLOR, CLASSIC_ISO_GOLD_COLOR, CLASSIC_RTS_TECH_BASE_COLOR,
    };
    use trnm_rts_core::RtsTile;
    use trnm_rts_data::first_contact_terrain_profile;

    #[test]
    fn first_contact_palette_helpers_preserve_color_roles() {
        assert_eq!(owner_from_rts_data("Multi0"), "Multi0");
        assert_eq!(owner_from_rts_data("SomethingElse"), "Neutral");
        assert_eq!(
            visual_telemetry_color(RtsVisualTelemetryColorRole::Health),
            CLASSIC_RTS_STATUS_HEALTH_BAR_COLOR
        );
        assert_eq!(
            actor_color_role_color(RtsActorColorRole::CommandCore),
            CLASSIC_RTS_TECH_BASE_COLOR
        );
        assert_eq!(
            silhouette_unit_color("worker"),
            CLASSIC_RTS_HARVEST_ANIMATION_CARRY_LOAD_COLOR
        );
        assert_eq!(
            silhouette_structure_color("beacon"),
            CLASSIC_RTS_MODEL_IDENTITY_BEACON_COLOR
        );
        assert_eq!(
            silhouette_terrain_color("unknown"),
            CLASSIC_RTS_PRODUCT_MAP_DENSITY_COLOR
        );
        assert_eq!(
            art_landmark_color("beacon_ring"),
            CLASSIC_RTS_COMMAND_SURFACE_TARGET_COLOR
        );
        assert_ne!(
            tile_profile_color(first_contact_terrain_profile(RtsTile::new(16, 9))),
            0x111812
        );

        let standard_slot = atlas_family_slot_color("worker_unit_family", false);
        let lower_lane_slot = atlas_family_slot_color("worker_unit_family", true);
        assert_eq!(gallery_muted_color(CLASSIC_ISO_GOLD_COLOR), standard_slot);
        assert_ne!(standard_slot, lower_lane_slot);
        assert_eq!(
            atlas_family_slot_color("unknown", false),
            gallery_muted_color(CLASSIC_HUD_MUTED_TEXT_COLOR)
        );
    }
}
