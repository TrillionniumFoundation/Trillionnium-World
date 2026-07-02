#![cfg(not(target_os = "android"))]

use crate::{
    classic_darken, classic_draw_rect, classic_mix_color, CLASSIC_RTS_COMMAND_SURFACE_TARGET_COLOR,
    CLASSIC_RTS_TACTICAL_VIEWPORT_TILE_COLOR,
};

pub(crate) fn lower_secondary_beacon_lane(tile: (i32, i32), role: &str) -> bool {
    tile == (16, 24) && role == "beacon_lane"
}

pub(crate) fn lower_secondary_beacon_art_detail(
    tile: (i32, i32),
    role: &str,
    signature: &str,
) -> bool {
    matches!(
        (tile, role, signature),
        ((16, 24), "beacon_lane", "painted_lane_chevrons")
            | ((16, 23), "beacon_lane", "lane_power_pylons")
    )
}

pub(crate) fn lower_secondary_beacon_art_color(color: u32) -> u32 {
    classic_mix_color(
        classic_darken(color, 1, 4),
        CLASSIC_RTS_TACTICAL_VIEWPORT_TILE_COLOR,
        2,
        3,
    )
}

pub(crate) fn secondary_beacon_capture_ring_detail(
    tile: (i32, i32),
    role: &str,
    signature: &str,
) -> bool {
    matches!(
        (tile, role, signature),
        ((16, 24), "beacon_ring", "beacon_capture_rings")
            | ((9, 16), "beacon_ring", "beacon_capture_rings")
            | ((24, 16), "beacon_ring", "beacon_capture_rings")
    )
}

pub(crate) fn player_screen_secondary_beacon_body(
    tile: (i32, i32),
    role: &str,
    signature: &str,
    player_screen: bool,
    target_tile: (i32, i32),
) -> bool {
    player_screen && tile != target_tile && role == "beacon" && signature == "vertical_beacon_spire"
}

pub(crate) fn secondary_objective_atlas_asset(
    tile: (i32, i32),
    role: &str,
    frame_id: &str,
) -> bool {
    tile == (16, 24) && role == "objective_sprite" && frame_id == "marker_interaction"
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn draw_secondary_objective_atlas_anchor(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    cx: i32,
    cy: i32,
    _cell_w: i32,
    cell_h: i32,
) {
    let anchor = classic_mix_color(
        classic_darken(CLASSIC_RTS_COMMAND_SURFACE_TARGET_COLOR, 1, 4),
        CLASSIC_RTS_TACTICAL_VIEWPORT_TILE_COLOR,
        1,
        5,
    );
    classic_draw_rect(
        buffer,
        width,
        height,
        cx - 1,
        cy + cell_h / 2 + 2,
        3,
        1,
        anchor,
    );
    classic_draw_rect(
        buffer,
        width,
        height,
        cx,
        cy + cell_h / 2 + 1,
        1,
        1,
        classic_darken(anchor, 1, 4),
    );
    classic_draw_rect(
        buffer,
        width,
        height,
        cx,
        cy + cell_h / 2 + 4,
        1,
        1,
        classic_darken(anchor, 1, 5),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_contact_renderer_readability_selectors_stay_narrow() {
        assert!(lower_secondary_beacon_lane((16, 24), "beacon_lane"));
        assert!(!lower_secondary_beacon_lane((16, 23), "beacon_lane"));
        assert!(!lower_secondary_beacon_lane((16, 24), "beacon_ring"));

        assert!(lower_secondary_beacon_art_detail(
            (16, 24),
            "beacon_lane",
            "painted_lane_chevrons"
        ));
        assert!(lower_secondary_beacon_art_detail(
            (16, 23),
            "beacon_lane",
            "lane_power_pylons"
        ));
        assert!(!lower_secondary_beacon_art_detail(
            (16, 24),
            "beacon_ring",
            "beacon_capture_rings"
        ));
        assert!(secondary_beacon_capture_ring_detail(
            (16, 24),
            "beacon_ring",
            "beacon_capture_rings"
        ));
        assert!(secondary_beacon_capture_ring_detail(
            (9, 16),
            "beacon_ring",
            "beacon_capture_rings"
        ));
        assert!(secondary_beacon_capture_ring_detail(
            (24, 16),
            "beacon_ring",
            "beacon_capture_rings"
        ));
        assert!(!secondary_beacon_capture_ring_detail(
            (16, 9),
            "beacon_ring",
            "beacon_capture_rings"
        ));
        assert!(!lower_secondary_beacon_art_detail(
            (16, 9),
            "beacon_ring",
            "beacon_capture_rings"
        ));
        assert!(player_screen_secondary_beacon_body(
            (16, 24),
            "beacon",
            "vertical_beacon_spire",
            true,
            (16, 9)
        ));
        assert!(player_screen_secondary_beacon_body(
            (9, 16),
            "beacon",
            "vertical_beacon_spire",
            true,
            (16, 9)
        ));
        assert!(!player_screen_secondary_beacon_body(
            (16, 9),
            "beacon",
            "vertical_beacon_spire",
            true,
            (16, 9)
        ));
        assert!(!player_screen_secondary_beacon_body(
            (16, 24),
            "beacon",
            "vertical_beacon_spire",
            false,
            (16, 9)
        ));

        assert!(secondary_objective_atlas_asset(
            (16, 24),
            "objective_sprite",
            "marker_interaction"
        ));
        assert!(!secondary_objective_atlas_asset(
            (16, 9),
            "objective_sprite",
            "marker_objective"
        ));
        assert!(!secondary_objective_atlas_asset(
            (16, 24),
            "beacon_objective_family",
            "marker_interaction"
        ));
        assert_ne!(lower_secondary_beacon_art_color(0x8090a0), 0x8090a0);
    }
}
