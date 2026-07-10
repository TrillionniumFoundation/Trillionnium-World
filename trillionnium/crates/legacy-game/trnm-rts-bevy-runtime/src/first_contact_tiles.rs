use trnm_rts_core::RtsTile;

pub const TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_TILE_SURFACE_CONTRACT: &str =
    "trnm_rts_bevy_runtime_first_contact_tile_surface_v1";

pub fn rts_first_contact_tile_tuple(tile: RtsTile) -> (i32, i32) {
    (tile.x, tile.y)
}

pub fn rts_first_contact_tile_id(tile: RtsTile) -> String {
    crate::rts_runtime_tile_id(rts_first_contact_tile_tuple(tile))
}

pub fn rts_first_contact_selection_route_tiles(route_tile_ids: &[String]) -> Vec<(i32, i32)> {
    route_tile_ids
        .iter()
        .filter_map(|tile_id| parse_tile_id(tile_id))
        .collect()
}

pub fn rts_first_contact_visual_hierarchy_corridor_tiles(
    route_tile_ids: &[String],
    selection_box_tile_ids: &[String],
    command_destination_tile: Option<&str>,
    fallback_target_tile: RtsTile,
    blocked_tile: RtsTile,
) -> Vec<(i32, i32)> {
    let mut tiles = rts_first_contact_selection_route_tiles(route_tile_ids);
    tiles.extend(
        selection_box_tile_ids
            .iter()
            .filter_map(|tile_id| parse_tile_id(tile_id)),
    );
    if let Some(tile) = command_destination_tile.and_then(parse_tile_id) {
        tiles.push(tile);
    } else {
        tiles.push(rts_first_contact_tile_tuple(fallback_target_tile));
    }
    tiles.push(rts_first_contact_tile_tuple(blocked_tile));
    tiles.sort_unstable();
    tiles.dedup();
    tiles
}

pub fn rts_first_contact_route_clearance_tiles(
    route_tile_ids: &[String],
    selection_box_tile_ids: &[String],
    command_destination_tile: Option<&str>,
    fallback_target_tile: RtsTile,
    blocked_tile: RtsTile,
) -> Vec<(i32, i32)> {
    let focus_tiles = rts_first_contact_visual_hierarchy_corridor_tiles(
        route_tile_ids,
        selection_box_tile_ids,
        command_destination_tile,
        fallback_target_tile,
        blocked_tile,
    );
    let mut tiles = Vec::new();
    for (tile_x, tile_y) in rts_first_contact_selection_route_tiles(route_tile_ids) {
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

pub fn rts_first_contact_central_clarity_quiet_tiles(
    route_tile_ids: &[String],
    selection_box_tile_ids: &[String],
    command_destination_tile: Option<&str>,
    fallback_target_tile: RtsTile,
    blocked_tile: RtsTile,
) -> Vec<(i32, i32)> {
    let focus_tiles = rts_first_contact_visual_hierarchy_corridor_tiles(
        route_tile_ids,
        selection_box_tile_ids,
        command_destination_tile,
        fallback_target_tile,
        blocked_tile,
    );
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

pub fn rts_first_contact_terminal_legibility_target_quiet_tiles() -> Vec<(i32, i32)> {
    vec![(15, 8), (16, 8), (17, 8), (15, 9), (17, 9)]
}

pub fn rts_first_contact_terminal_legibility_blocked_quiet_tiles() -> Vec<(i32, i32)> {
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

pub fn rts_first_contact_terminal_legibility_quiet_tiles() -> Vec<(i32, i32)> {
    let mut tiles = rts_first_contact_terminal_legibility_target_quiet_tiles();
    tiles.extend(rts_first_contact_terminal_legibility_blocked_quiet_tiles());
    tiles
}

pub fn rts_first_contact_target_callout_tile(
    command_destination_tile: Option<&str>,
    fallback_target_tile: RtsTile,
) -> (i32, i32) {
    command_destination_tile
        .and_then(parse_tile_id)
        .unwrap_or_else(|| rts_first_contact_tile_tuple(fallback_target_tile))
}

pub fn rts_first_contact_radar_objective_tiles() -> Vec<(i32, i32)> {
    vec![(16, 9), (16, 24), (9, 16), (24, 16)]
}

pub fn rts_first_contact_radar_structure_tiles() -> Vec<(i32, i32)> {
    vec![(8, 8), (25, 8), (25, 25), (8, 25), (11, 8), (22, 25)]
}

pub fn rts_first_contact_radar_pressure_tiles() -> Vec<(i32, i32)> {
    vec![(25, 25), (25, 8), (24, 16)]
}

pub fn rts_first_contact_radar_lane_sample_tiles() -> Vec<(i32, i32)> {
    let mut tiles = Vec::new();
    for tile_y in (4..=30).step_by(2) {
        tiles.push((16, tile_y));
    }
    for tile_x in (6..=28).step_by(2) {
        tiles.push((tile_x, 16));
    }
    tiles
}

pub fn rts_first_contact_radar_focus_tile(camera_focus_tile_id: Option<&str>) -> (i32, i32) {
    camera_focus_tile_id
        .and_then(parse_tile_id)
        .unwrap_or((16, 16))
}

fn parse_tile_id(value: &str) -> Option<(i32, i32)> {
    let (x, y) = value.split_once(',')?;
    Some((x.parse().ok()?, y.parse().ok()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tile_ids(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn first_contact_tile_surface_preserves_focus_and_radar_contracts() {
        let route_tile_ids = tile_ids(&["14,11", "15,11", "16,10", "16,9"]);
        let selection_box_tile_ids = tile_ids(&["14,11", "15,11", "15,12", "17,12"]);
        let command_destination_tile = Some("16,9");
        let target = RtsTile::new(16, 9);
        let blocked = RtsTile::new(15, 16);

        assert_eq!(
            TRNM_RTS_BEVY_RUNTIME_FIRST_CONTACT_TILE_SURFACE_CONTRACT,
            "trnm_rts_bevy_runtime_first_contact_tile_surface_v1"
        );
        assert_eq!(rts_first_contact_tile_tuple(target), (16, 9));
        assert_eq!(rts_first_contact_tile_id(target), "16,9");
        assert_eq!(
            rts_first_contact_selection_route_tiles(&route_tile_ids),
            vec![(14, 11), (15, 11), (16, 10), (16, 9)]
        );
        assert_eq!(
            rts_first_contact_visual_hierarchy_corridor_tiles(
                &route_tile_ids,
                &selection_box_tile_ids,
                command_destination_tile,
                target,
                blocked,
            ),
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
            rts_first_contact_route_clearance_tiles(
                &route_tile_ids,
                &selection_box_tile_ids,
                command_destination_tile,
                target,
                blocked,
            ),
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
            rts_first_contact_central_clarity_quiet_tiles(
                &route_tile_ids,
                &selection_box_tile_ids,
                command_destination_tile,
                target,
                blocked,
            )
            .len(),
            13
        );
        assert_eq!(
            rts_first_contact_terminal_legibility_quiet_tiles().len(),
            13
        );
        assert_eq!(
            rts_first_contact_target_callout_tile(command_destination_tile, target),
            (16, 9)
        );
        assert_eq!(rts_first_contact_radar_focus_tile(Some("13,3")), (13, 3));
        assert_eq!(rts_first_contact_radar_lane_sample_tiles().len(), 26);
    }
}
