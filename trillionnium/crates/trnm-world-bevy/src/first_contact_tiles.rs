#![cfg(not(target_os = "android"))]

use crate::{classic_parse_rts_tile, classic_rts_tile_id, NativeFirstPlayableRuntime};
use trnm_rts_core::RtsTile;

pub(crate) fn tile_tuple(tile: RtsTile) -> (i32, i32) {
    (tile.x, tile.y)
}

pub(crate) fn tile_id(tile: RtsTile) -> String {
    classic_rts_tile_id(tile_tuple(tile))
}

pub(crate) fn selection_combat_focus_route_tiles(
    runtime: &NativeFirstPlayableRuntime,
) -> Vec<(i32, i32)> {
    runtime
        .rts_group_route_tile_ids
        .iter()
        .filter_map(|tile_id| classic_parse_rts_tile(tile_id))
        .collect()
}

pub(crate) fn visual_hierarchy_corridor_tiles(
    runtime: &NativeFirstPlayableRuntime,
    target_tile: RtsTile,
    blocked_tile: RtsTile,
) -> Vec<(i32, i32)> {
    let mut tiles = selection_combat_focus_route_tiles(runtime);
    tiles.extend(
        runtime
            .rts_selection_box_tile_ids
            .iter()
            .filter_map(|tile_id| classic_parse_rts_tile(tile_id)),
    );
    if let Some(tile) = runtime
        .rts_command_destination_tile
        .as_deref()
        .and_then(classic_parse_rts_tile)
    {
        tiles.push(tile);
    } else {
        tiles.push(tile_tuple(target_tile));
    }
    tiles.push(tile_tuple(blocked_tile));
    tiles.sort_unstable();
    tiles.dedup();
    tiles
}

pub(crate) fn route_clearance_tiles(
    runtime: &NativeFirstPlayableRuntime,
    target_tile: RtsTile,
    blocked_tile: RtsTile,
) -> Vec<(i32, i32)> {
    let focus_tiles = visual_hierarchy_corridor_tiles(runtime, target_tile, blocked_tile);
    let mut tiles = Vec::new();
    for (tile_x, tile_y) in selection_combat_focus_route_tiles(runtime) {
        for candidate in [
            (tile_x - 1, tile_y),
            (tile_x + 1, tile_y),
            (tile_x, tile_y - 1),
            (tile_x, tile_y + 1),
        ] {
            if !focus_tiles.contains(&candidate) && !tiles.contains(&candidate) {
                tiles.push(candidate);
            }
        }
    }
    tiles.sort_unstable();
    tiles
}

pub(crate) fn central_clarity_quiet_tiles(
    runtime: &NativeFirstPlayableRuntime,
    target_tile: RtsTile,
    blocked_tile: RtsTile,
) -> Vec<(i32, i32)> {
    let focus_tiles = visual_hierarchy_corridor_tiles(runtime, target_tile, blocked_tile);
    let mut tiles = Vec::new();
    for y in 10..=12 {
        for x in 13..=18 {
            let tile = (x, y);
            if !focus_tiles.contains(&tile) {
                tiles.push(tile);
            }
        }
    }
    tiles
}

pub(crate) fn terminal_legibility_target_quiet_tiles() -> Vec<(i32, i32)> {
    vec![(15, 8), (16, 8), (17, 8), (15, 9), (17, 9)]
}

pub(crate) fn terminal_legibility_blocked_quiet_tiles() -> Vec<(i32, i32)> {
    vec![
        (14, 15),
        (15, 15),
        (16, 15),
        (14, 16),
        (16, 16),
        (14, 17),
        (15, 17),
        (16, 17),
    ]
}

#[cfg(test)]
pub(crate) fn terminal_legibility_quiet_tiles() -> Vec<(i32, i32)> {
    let mut tiles = terminal_legibility_target_quiet_tiles();
    tiles.extend(terminal_legibility_blocked_quiet_tiles());
    tiles
}

pub(crate) fn target_callout_tile(
    runtime: &NativeFirstPlayableRuntime,
    fallback_target_tile: RtsTile,
) -> (i32, i32) {
    runtime
        .rts_command_destination_tile
        .as_deref()
        .and_then(classic_parse_rts_tile)
        .unwrap_or_else(|| tile_tuple(fallback_target_tile))
}

pub(crate) fn radar_objective_tiles() -> Vec<(i32, i32)> {
    vec![(16, 9), (16, 24), (9, 16), (24, 16)]
}

pub(crate) fn radar_structure_tiles() -> Vec<(i32, i32)> {
    vec![(8, 8), (25, 8), (25, 25), (8, 25), (11, 8), (22, 25)]
}

pub(crate) fn radar_pressure_tiles() -> Vec<(i32, i32)> {
    vec![(25, 25), (25, 8), (24, 16)]
}

pub(crate) fn radar_lane_sample_tiles() -> Vec<(i32, i32)> {
    let mut tiles = Vec::new();
    for tile_y in (4..=30).step_by(2) {
        tiles.push((16, tile_y));
    }
    for tile_x in (6..=28).step_by(2) {
        tiles.push((tile_x, 16));
    }
    tiles
}

pub(crate) fn radar_focus_tile(runtime: &NativeFirstPlayableRuntime) -> (i32, i32) {
    runtime
        .rts_camera_focus_tile_id
        .as_deref()
        .and_then(classic_parse_rts_tile)
        .unwrap_or((16, 16))
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
