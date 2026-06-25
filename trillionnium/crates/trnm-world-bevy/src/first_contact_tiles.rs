#![cfg(not(target_os = "android"))]

use crate::NativeFirstPlayableRuntime;
use trnm_rts_core::RtsTile;

pub(crate) fn tile_tuple(tile: RtsTile) -> (i32, i32) {
    trnm_rts_bevy_runtime::rts_first_contact_tile_tuple(tile)
}

pub(crate) fn tile_id(tile: RtsTile) -> String {
    trnm_rts_bevy_runtime::rts_first_contact_tile_id(tile)
}

pub(crate) fn selection_combat_focus_route_tiles(
    runtime: &NativeFirstPlayableRuntime,
) -> Vec<(i32, i32)> {
    trnm_rts_bevy_runtime::rts_first_contact_selection_route_tiles(
        &runtime.rts_group_route_tile_ids,
    )
}

pub(crate) fn visual_hierarchy_corridor_tiles(
    runtime: &NativeFirstPlayableRuntime,
    target_tile: RtsTile,
    blocked_tile: RtsTile,
) -> Vec<(i32, i32)> {
    trnm_rts_bevy_runtime::rts_first_contact_visual_hierarchy_corridor_tiles(
        &runtime.rts_group_route_tile_ids,
        &runtime.rts_selection_box_tile_ids,
        runtime.rts_command_destination_tile.as_deref(),
        target_tile,
        blocked_tile,
    )
}

pub(crate) fn route_clearance_tiles(
    runtime: &NativeFirstPlayableRuntime,
    target_tile: RtsTile,
    blocked_tile: RtsTile,
) -> Vec<(i32, i32)> {
    trnm_rts_bevy_runtime::rts_first_contact_route_clearance_tiles(
        &runtime.rts_group_route_tile_ids,
        &runtime.rts_selection_box_tile_ids,
        runtime.rts_command_destination_tile.as_deref(),
        target_tile,
        blocked_tile,
    )
}

pub(crate) fn central_clarity_quiet_tiles(
    runtime: &NativeFirstPlayableRuntime,
    target_tile: RtsTile,
    blocked_tile: RtsTile,
) -> Vec<(i32, i32)> {
    trnm_rts_bevy_runtime::rts_first_contact_central_clarity_quiet_tiles(
        &runtime.rts_group_route_tile_ids,
        &runtime.rts_selection_box_tile_ids,
        runtime.rts_command_destination_tile.as_deref(),
        target_tile,
        blocked_tile,
    )
}

pub(crate) fn terminal_legibility_target_quiet_tiles() -> Vec<(i32, i32)> {
    trnm_rts_bevy_runtime::rts_first_contact_terminal_legibility_target_quiet_tiles()
}

pub(crate) fn terminal_legibility_blocked_quiet_tiles() -> Vec<(i32, i32)> {
    trnm_rts_bevy_runtime::rts_first_contact_terminal_legibility_blocked_quiet_tiles()
}

#[cfg(test)]
pub(crate) fn terminal_legibility_quiet_tiles() -> Vec<(i32, i32)> {
    trnm_rts_bevy_runtime::rts_first_contact_terminal_legibility_quiet_tiles()
}

pub(crate) fn target_callout_tile(
    runtime: &NativeFirstPlayableRuntime,
    fallback_target_tile: RtsTile,
) -> (i32, i32) {
    trnm_rts_bevy_runtime::rts_first_contact_target_callout_tile(
        runtime.rts_command_destination_tile.as_deref(),
        fallback_target_tile,
    )
}

pub(crate) fn radar_objective_tiles() -> Vec<(i32, i32)> {
    trnm_rts_bevy_runtime::rts_first_contact_radar_objective_tiles()
}

pub(crate) fn radar_structure_tiles() -> Vec<(i32, i32)> {
    trnm_rts_bevy_runtime::rts_first_contact_radar_structure_tiles()
}

pub(crate) fn radar_pressure_tiles() -> Vec<(i32, i32)> {
    trnm_rts_bevy_runtime::rts_first_contact_radar_pressure_tiles()
}

pub(crate) fn radar_lane_sample_tiles() -> Vec<(i32, i32)> {
    trnm_rts_bevy_runtime::rts_first_contact_radar_lane_sample_tiles()
}

pub(crate) fn radar_focus_tile(runtime: &NativeFirstPlayableRuntime) -> (i32, i32) {
    trnm_rts_bevy_runtime::rts_first_contact_radar_focus_tile(
        runtime.rts_camera_focus_tile_id.as_deref(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn first_contact_focus_runtime() -> NativeFirstPlayableRuntime {
        NativeFirstPlayableRuntime {
            rts_group_route_tile_ids: vec![
                "14,11".to_string(),
                "15,11".to_string(),
                "16,10".to_string(),
                "16,9".to_string(),
            ],
            rts_selection_box_tile_ids: vec![
                "14,11".to_string(),
                "15,11".to_string(),
                "15,12".to_string(),
                "17,12".to_string(),
            ],
            rts_command_destination_tile: Some("16,9".to_string()),
            rts_camera_focus_tile_id: Some("13,3".to_string()),
            ..Default::default()
        }
    }

    #[test]
    fn first_contact_tile_sets_preserve_focus_and_radar_contracts() {
        let runtime = first_contact_focus_runtime();
        let target = RtsTile::new(16, 9);
        let blocked = RtsTile::new(15, 16);

        assert_eq!(
            selection_combat_focus_route_tiles(&runtime),
            vec![(14, 11), (15, 11), (16, 10), (16, 9)]
        );
        assert_eq!(
            visual_hierarchy_corridor_tiles(&runtime, target, blocked),
            vec![
                (14, 11),
                (15, 11),
                (15, 12),
                (15, 16),
                (16, 9),
                (16, 10),
                (17, 12)
            ]
        );
        assert_eq!(
            route_clearance_tiles(&runtime, target, blocked),
            vec![
                (13, 11),
                (14, 10),
                (14, 12),
                (15, 9),
                (15, 10),
                (16, 8),
                (16, 11),
                (17, 9),
                (17, 10),
            ]
        );
        assert_eq!(
            central_clarity_quiet_tiles(&runtime, target, blocked).len(),
            13
        );
        assert_eq!(terminal_legibility_quiet_tiles().len(), 13);
        assert_eq!(target_callout_tile(&runtime, target), (16, 9));
        assert_eq!(radar_focus_tile(&runtime), (13, 3));
        assert_eq!(radar_lane_sample_tiles().len(), 26);
    }
}
