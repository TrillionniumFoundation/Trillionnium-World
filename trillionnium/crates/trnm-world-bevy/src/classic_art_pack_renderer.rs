use crate::{
    classic_blit_pixels_scaled, classic_draw_iso_diamond, classic_draw_iso_ellipse,
    classic_draw_iso_prism, classic_draw_iso_procedural_model, classic_draw_iso_shadow,
    classic_draw_rect, classic_draw_rts_action_cadence_marks,
    classic_draw_rts_unit_model_depth_marks, classic_draw_text, classic_lighten,
    ClassicRuntimeAssets, CLASSIC_HUD_ACCENT_TEXT_COLOR, CLASSIC_HUD_TEXT_COLOR,
    CLASSIC_ISO_ATTACK_ARC_COLOR, CLASSIC_ISO_BANNER_COLOR, CLASSIC_ISO_BLUE_ROOF_COLOR,
    CLASSIC_ISO_BRIDGE_PLANK_COLOR, CLASSIC_ISO_CANOPY_COLOR, CLASSIC_ISO_CANOPY_LIGHT_COLOR,
    CLASSIC_ISO_CLIFF_FACE_COLOR, CLASSIC_ISO_COMMAND_MARKER_COLOR,
    CLASSIC_ISO_DOODAD_CRYSTAL_COLOR, CLASSIC_ISO_DOODAD_FIRE_COLOR,
    CLASSIC_ISO_DOODAD_STONE_COLOR, CLASSIC_ISO_DOODAD_WOOD_COLOR, CLASSIC_ISO_FOLIAGE_DARK_COLOR,
    CLASSIC_ISO_FOUNDATION_COLOR, CLASSIC_ISO_GOLD_COLOR, CLASSIC_ISO_GOLD_VEIN_COLOR,
    CLASSIC_ISO_HIT_FLASH_COLOR, CLASSIC_ISO_MAGIC_COLOR, CLASSIC_ISO_OUTLINE_COLOR,
    CLASSIC_ISO_ROAD_DETAIL_COLOR, CLASSIC_ISO_ROOF_COLOR, CLASSIC_ISO_RUIN_COLOR,
    CLASSIC_ISO_SHADOW_COLOR, CLASSIC_ISO_STONE_COLOR, CLASSIC_ISO_UNIT_CREEP_COLOR,
    CLASSIC_ISO_UNIT_DAMAGE_COLOR, CLASSIC_ISO_UNIT_ENEMY_COLOR, CLASSIC_ISO_UNIT_GUARD_COLOR,
    CLASSIC_ISO_UNIT_HEALTH_COLOR, CLASSIC_ISO_UNIT_MENTOR_COLOR, CLASSIC_ISO_UNIT_RING_COLOR,
    CLASSIC_ISO_UNIT_WORKER_COLOR, CLASSIC_ISO_WALL_COLOR, CLASSIC_ISO_WATER_DETAIL_COLOR,
    CLASSIC_ISO_WATER_HIGHLIGHT_COLOR, CLASSIC_RTS_FIDELITY_ACTION_TRAIL_COLOR,
    CLASSIC_RTS_FIDELITY_ANIMATION_GHOST_COLOR, CLASSIC_RTS_FIDELITY_MODEL_EDGE_COLOR,
    CLASSIC_RTS_FIDELITY_MODEL_HIGHLIGHT_COLOR, CLASSIC_RTS_FIDELITY_NPC_ACTION_COLOR,
};

#[cfg(not(target_os = "android"))]
pub(super) fn classic_art_pack_highlight_color(color: u32) -> bool {
    matches!(
        color,
        0xf1c45d
            | 0xf4d06f
            | 0xd8b15a
            | 0xccbc7a
            | 0xc7d3ff
            | 0x77c6cf
            | 0xff9d45
            | 0x85f0ff
            | 0x9a3e4a
            | 0x83c79d
            | 0xfff0a8
            | 0x7fd8e8
            | 0xf2dc73
            | 0xd8f0a0
            | 0xff8c73
            | 0xff5c4d
            | 0xffb199
            | CLASSIC_ISO_GOLD_COLOR
            | CLASSIC_ISO_MAGIC_COLOR
            | CLASSIC_ISO_BANNER_COLOR
            | CLASSIC_ISO_CANOPY_LIGHT_COLOR
            | CLASSIC_ISO_GOLD_VEIN_COLOR
            | CLASSIC_ISO_BRIDGE_PLANK_COLOR
            | 0xa8d8ff
            | 0xf0be70
            | 0xd0a2ff
            | 0xe8e0bd
            | 0xffe2a6
    )
}

#[cfg(not(target_os = "android"))]
pub(super) fn classic_art_pack_doodad_detail_color(color: u32) -> bool {
    matches!(
        color,
        CLASSIC_ISO_DOODAD_STONE_COLOR
            | CLASSIC_ISO_DOODAD_WOOD_COLOR
            | CLASSIC_ISO_DOODAD_FIRE_COLOR
            | CLASSIC_ISO_DOODAD_CRYSTAL_COLOR
            | CLASSIC_ISO_FOLIAGE_DARK_COLOR
            | CLASSIC_ISO_RUIN_COLOR
            | CLASSIC_ISO_GOLD_VEIN_COLOR
            | CLASSIC_ISO_BRIDGE_PLANK_COLOR
            | CLASSIC_ISO_CANOPY_COLOR
            | CLASSIC_ISO_CANOPY_LIGHT_COLOR
            | 0xffd07a
            | 0xb8fbff
    )
}

#[cfg(not(target_os = "android"))]
pub(super) fn classic_art_pack_terrain_frame(frame_id: &str) -> bool {
    matches!(
        frame_id,
        "tile_grass_a"
            | "tile_grass_b"
            | "tile_road"
            | "tile_water"
            | "tile_wall"
            | "tile_roof"
            | "tile_arena"
            | "tile_cliff_edge"
            | "tile_bridge"
            | "tile_forest_floor"
            | "tile_shadow_edge"
    )
}

#[cfg(not(target_os = "android"))]
pub(super) fn classic_art_pack_terrain_detail_color(color: u32) -> bool {
    matches!(
        color,
        CLASSIC_ISO_ROAD_DETAIL_COLOR
            | CLASSIC_ISO_WATER_DETAIL_COLOR
            | CLASSIC_ISO_WATER_HIGHLIGHT_COLOR
            | CLASSIC_ISO_WALL_COLOR
            | CLASSIC_ISO_BLUE_ROOF_COLOR
            | CLASSIC_ISO_FOUNDATION_COLOR
            | CLASSIC_ISO_CLIFF_FACE_COLOR
            | CLASSIC_ISO_BRIDGE_PLANK_COLOR
            | CLASSIC_ISO_CANOPY_COLOR
            | CLASSIC_ISO_CANOPY_LIGHT_COLOR
            | 0x2e6f44
            | 0x407849
            | 0xb19565
            | 0x6c777c
            | 0x9d5a55
            | 0x915548
            | 0xc4745e
    )
}

#[cfg(not(target_os = "android"))]
pub(super) fn classic_art_pack_world_prop_frame(frame_id: &str) -> bool {
    matches!(
        frame_id,
        "actor_mentor_talk"
            | "actor_vendor_idle"
            | "prop_training_dummy"
            | "prop_reward"
            | "prop_arena_gate"
            | "prop_market_stall"
            | "prop_banner"
            | "marker_objective"
            | "marker_interaction"
    )
}

#[cfg(not(target_os = "android"))]
pub(super) fn classic_art_pack_world_prop_detail_color(color: u32) -> bool {
    matches!(
        color,
        CLASSIC_ISO_UNIT_MENTOR_COLOR
            | CLASSIC_ISO_UNIT_ENEMY_COLOR
            | CLASSIC_ISO_GOLD_COLOR
            | CLASSIC_ISO_MAGIC_COLOR
            | CLASSIC_ISO_BANNER_COLOR
            | CLASSIC_ISO_DOODAD_WOOD_COLOR
            | CLASSIC_ISO_UNIT_RING_COLOR
            | CLASSIC_ISO_COMMAND_MARKER_COLOR
            | CLASSIC_ISO_OUTLINE_COLOR
            | 0xffffff
            | 0xfff0a8
            | 0x3f9b58
    )
}

#[cfg(not(target_os = "android"))]
pub(super) fn classic_art_pack_neutral_unit_frame(frame_id: &str) -> bool {
    matches!(
        frame_id,
        "actor_guard_idle"
            | "actor_guard_attack"
            | "actor_worker_idle"
            | "actor_worker_carry"
            | "actor_creep_idle"
            | "actor_creep_attack"
    )
}

#[cfg(not(target_os = "android"))]
pub(super) fn classic_art_pack_neutral_unit_detail_color(color: u32) -> bool {
    matches!(
        color,
        CLASSIC_ISO_UNIT_GUARD_COLOR
            | CLASSIC_ISO_UNIT_WORKER_COLOR
            | CLASSIC_ISO_UNIT_CREEP_COLOR
            | CLASSIC_ISO_UNIT_RING_COLOR
            | CLASSIC_ISO_UNIT_HEALTH_COLOR
            | CLASSIC_ISO_UNIT_DAMAGE_COLOR
            | CLASSIC_ISO_OUTLINE_COLOR
            | 0xa8d8ff
            | 0xf0be70
            | 0xd0a2ff
            | 0xe8e0bd
            | 0xffe2a6
            | 0x2f3950
            | 0x4f3726
            | 0x3a2448
    )
}

#[cfg(not(target_os = "android"))]
pub(super) fn classic_art_pack_vfx_frame(frame_id: &str) -> bool {
    matches!(
        frame_id,
        "rts_command_destination_marker"
            | "combat_attack_arc"
            | "combat_hit_flash"
            | "rts_unit_selection_ring"
            | "unit_health_bar"
            | "rts_foundation_shadow"
    )
}

#[cfg(not(target_os = "android"))]
pub(super) fn classic_art_pack_vfx_detail_color(color: u32) -> bool {
    matches!(
        color,
        CLASSIC_ISO_COMMAND_MARKER_COLOR
            | CLASSIC_ISO_ATTACK_ARC_COLOR
            | CLASSIC_ISO_HIT_FLASH_COLOR
            | CLASSIC_ISO_UNIT_RING_COLOR
            | CLASSIC_ISO_UNIT_HEALTH_COLOR
            | CLASSIC_ISO_UNIT_DAMAGE_COLOR
            | CLASSIC_ISO_FOUNDATION_COLOR
            | 0xffffff
            | 0xfff3a4
    )
}

#[cfg(not(target_os = "android"))]
pub(super) fn classic_art_pack_override_specs() -> Vec<(&'static str, u32, u32, &'static str)> {
    let mut specs = classic_art_pack_synthetic_override_specs()
        .into_iter()
        .map(|(frame_id, width, height)| {
            let group = match frame_id {
                "model_town_hall" => "town_hall",
                "model_waygate" => "waygate",
                "model_training_hall" => "training_hall",
                "model_coliseum_stands" => "coliseum",
                "model_tree_cluster_large" => "tree_cluster",
                frame if frame.starts_with("doodad_") => "doodad",
                frame if classic_art_pack_terrain_frame(frame) => "terrain",
                frame if classic_art_pack_world_prop_frame(frame) => "world_prop",
                frame if classic_art_pack_neutral_unit_frame(frame) => "neutral_unit",
                frame if classic_art_pack_vfx_frame(frame) => "vfx",
                _ => "model",
            };
            (frame_id, width, height, group)
        })
        .collect::<Vec<_>>();
    specs.extend([
        ("actor_player_idle_south", 16, 16, "player"),
        ("actor_player_idle_north", 16, 16, "player"),
        ("actor_player_idle_east", 16, 16, "player"),
        ("actor_player_idle_west", 16, 16, "player"),
        ("actor_player_walk_1", 16, 16, "player"),
        ("actor_player_walk_south_1", 16, 16, "player"),
        ("actor_player_walk_south_2", 16, 16, "player"),
        ("actor_player_walk_north_1", 16, 16, "player"),
        ("actor_player_walk_north_2", 16, 16, "player"),
        ("actor_player_walk_east_1", 16, 16, "player"),
        ("actor_player_walk_east_2", 16, 16, "player"),
        ("actor_player_walk_west_1", 16, 16, "player"),
        ("actor_player_walk_west_2", 16, 16, "player"),
        ("actor_enemy", 16, 16, "enemy"),
        ("actor_enemy_idle", 16, 16, "enemy"),
        ("actor_enemy_attack", 16, 16, "enemy"),
        ("actor_enemy_hit", 16, 16, "enemy"),
    ]);
    specs
}

pub(super) fn classic_art_pack_synthetic_override_specs() -> Vec<(&'static str, u32, u32)> {
    vec![
        ("model_town_hall", 96, 96),
        ("model_waygate", 96, 96),
        ("model_training_hall", 96, 96),
        ("model_coliseum_stands", 128, 96),
        ("model_tree_cluster_large", 96, 96),
        ("doodad_rock_cluster", 48, 48),
        ("doodad_barrel_stack", 48, 48),
        ("doodad_torch", 48, 48),
        ("doodad_crystal_cluster", 48, 48),
        ("doodad_bush_cluster", 48, 48),
        ("doodad_ruins_column", 48, 56),
        ("doodad_gold_vein", 48, 48),
        ("doodad_signpost", 48, 48),
        ("tile_grass_a", 48, 24),
        ("tile_grass_b", 48, 24),
        ("tile_road", 48, 24),
        ("tile_water", 48, 24),
        ("tile_wall", 48, 36),
        ("tile_roof", 48, 36),
        ("tile_arena", 48, 24),
        ("tile_cliff_edge", 48, 36),
        ("tile_bridge", 48, 24),
        ("tile_forest_floor", 48, 24),
        ("tile_shadow_edge", 48, 24),
        ("actor_mentor_talk", 32, 48),
        ("actor_vendor_idle", 32, 48),
        ("prop_training_dummy", 32, 48),
        ("prop_reward", 32, 32),
        ("prop_arena_gate", 64, 48),
        ("prop_market_stall", 64, 48),
        ("prop_banner", 32, 48),
        ("marker_objective", 32, 32),
        ("marker_interaction", 32, 32),
        ("rts_command_destination_marker", 48, 48),
        ("combat_attack_arc", 64, 48),
        ("combat_hit_flash", 48, 48),
        ("rts_unit_selection_ring", 48, 48),
        ("unit_health_bar", 32, 16),
        ("rts_foundation_shadow", 96, 48),
        ("actor_guard_idle", 32, 48),
        ("actor_guard_attack", 40, 48),
        ("actor_worker_idle", 32, 48),
        ("actor_worker_carry", 40, 48),
        ("actor_creep_idle", 36, 48),
        ("actor_creep_attack", 44, 48),
    ]
}

#[cfg(not(target_os = "android"))]
pub(super) fn classic_art_pack_pixels(frame_id: &str, width: u32, height: u32) -> Vec<u32> {
    let mut pixels = vec![0x000000_u32; width as usize * height as usize];
    match frame_id {
        "tile_grass_a" => {
            classic_draw_iso_diamond(
                &mut pixels,
                width as usize,
                height as usize,
                24,
                2,
                46,
                22,
                0x2e6f44,
            );
            for (x, y, w) in [(12, 9, 7), (27, 6, 9), (31, 15, 6), (18, 16, 5)] {
                classic_draw_rect(
                    &mut pixels,
                    width as usize,
                    height as usize,
                    x,
                    y,
                    w,
                    2,
                    0x407849,
                );
            }
        }
        "tile_grass_b" => {
            classic_draw_iso_diamond(
                &mut pixels,
                width as usize,
                height as usize,
                24,
                2,
                46,
                22,
                0x255a3d,
            );
            for (x, y, w) in [(10, 13, 8), (23, 8, 6), (30, 17, 9), (17, 6, 4)] {
                classic_draw_rect(
                    &mut pixels,
                    width as usize,
                    height as usize,
                    x,
                    y,
                    w,
                    2,
                    0x2e6f44,
                );
            }
        }
        "tile_road" => {
            classic_draw_iso_diamond(
                &mut pixels,
                width as usize,
                height as usize,
                24,
                2,
                46,
                22,
                0x8a7350,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                8,
                11,
                32,
                3,
                0xb19565,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                15,
                7,
                20,
                2,
                CLASSIC_ISO_ROAD_DETAIL_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                19,
                17,
                16,
                2,
                CLASSIC_ISO_ROAD_DETAIL_COLOR,
            );
        }
        "tile_water" => {
            classic_draw_iso_diamond(
                &mut pixels,
                width as usize,
                height as usize,
                24,
                2,
                46,
                22,
                0x2e5d74,
            );
            for y in [7, 11, 15] {
                classic_draw_rect(
                    &mut pixels,
                    width as usize,
                    height as usize,
                    12,
                    y,
                    24,
                    2,
                    CLASSIC_ISO_WATER_DETAIL_COLOR,
                );
            }
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                18,
                9,
                15,
                1,
                CLASSIC_ISO_WATER_HIGHLIGHT_COLOR,
            );
        }
        "tile_wall" => {
            classic_draw_iso_diamond(
                &mut pixels,
                width as usize,
                height as usize,
                24,
                12,
                46,
                22,
                CLASSIC_ISO_WALL_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                9,
                9,
                30,
                7,
                0x6c777c,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                13,
                5,
                22,
                5,
                CLASSIC_ISO_STONE_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                15,
                18,
                18,
                2,
                CLASSIC_ISO_FOUNDATION_COLOR,
            );
        }
        "tile_roof" => {
            classic_draw_iso_diamond(
                &mut pixels,
                width as usize,
                height as usize,
                24,
                12,
                46,
                22,
                CLASSIC_ISO_ROOF_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                10,
                8,
                28,
                5,
                0x9d5a55,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                15,
                5,
                18,
                3,
                0xc4745e,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                18,
                17,
                13,
                2,
                CLASSIC_ISO_FOUNDATION_COLOR,
            );
        }
        "tile_arena" => {
            classic_draw_iso_diamond(
                &mut pixels,
                width as usize,
                height as usize,
                24,
                2,
                46,
                22,
                0x5b3c34,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                10,
                9,
                28,
                3,
                0x915548,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                14,
                14,
                21,
                3,
                0xc4745e,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                19,
                6,
                13,
                2,
                CLASSIC_ISO_FOUNDATION_COLOR,
            );
        }
        "tile_cliff_edge" => {
            classic_draw_iso_diamond(
                &mut pixels,
                width as usize,
                height as usize,
                24,
                8,
                46,
                20,
                0x5d4f3f,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                6,
                18,
                36,
                10,
                CLASSIC_ISO_CLIFF_FACE_COLOR,
            );
            for (x, y, w) in [(9, 20, 8), (21, 23, 10), (33, 20, 6)] {
                classic_draw_rect(
                    &mut pixels,
                    width as usize,
                    height as usize,
                    x,
                    y,
                    w,
                    2,
                    CLASSIC_ISO_DOODAD_STONE_COLOR,
                );
            }
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                12,
                10,
                24,
                3,
                CLASSIC_ISO_FOUNDATION_COLOR,
            );
        }
        "tile_bridge" => {
            classic_draw_iso_diamond(
                &mut pixels,
                width as usize,
                height as usize,
                24,
                2,
                46,
                22,
                CLASSIC_ISO_WATER_DETAIL_COLOR,
            );
            for y in [8, 12, 16] {
                classic_draw_rect(
                    &mut pixels,
                    width as usize,
                    height as usize,
                    8,
                    y,
                    32,
                    3,
                    CLASSIC_ISO_BRIDGE_PLANK_COLOR,
                );
            }
            for x in [12, 24, 36] {
                classic_draw_rect(
                    &mut pixels,
                    width as usize,
                    height as usize,
                    x,
                    5,
                    2,
                    16,
                    CLASSIC_ISO_DOODAD_WOOD_COLOR,
                );
            }
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                10,
                6,
                28,
                2,
                0xb98a55,
            );
        }
        "tile_forest_floor" => {
            classic_draw_iso_diamond(
                &mut pixels,
                width as usize,
                height as usize,
                24,
                2,
                46,
                22,
                CLASSIC_ISO_FOLIAGE_DARK_COLOR,
            );
            for (x, y, rx, ry, color) in [
                (14, 12, 9, 4, CLASSIC_ISO_CANOPY_COLOR),
                (28, 9, 11, 5, CLASSIC_ISO_CANOPY_LIGHT_COLOR),
                (33, 16, 7, 3, 0x2f8b50),
                (18, 17, 8, 3, 0x347f45),
            ] {
                classic_draw_iso_ellipse(
                    &mut pixels,
                    width as usize,
                    height as usize,
                    x,
                    y,
                    rx,
                    ry,
                    color,
                );
            }
        }
        "tile_shadow_edge" => {
            classic_draw_iso_diamond(
                &mut pixels,
                width as usize,
                height as usize,
                24,
                2,
                46,
                22,
                0x253027,
            );
            classic_draw_iso_ellipse(
                &mut pixels,
                width as usize,
                height as usize,
                24,
                15,
                20,
                6,
                CLASSIC_ISO_SHADOW_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                8,
                11,
                32,
                3,
                CLASSIC_ISO_FOUNDATION_COLOR,
            );
        }
        "actor_mentor_talk" => {
            classic_draw_iso_shadow(&mut pixels, width as usize, height as usize, 16, 44, 10, 3);
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                13,
                17,
                6,
                23,
                CLASSIC_ISO_UNIT_MENTOR_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                10,
                23,
                12,
                6,
                CLASSIC_ISO_BANNER_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                12,
                10,
                8,
                7,
                0xfff0a8,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                21,
                12,
                6,
                3,
                CLASSIC_ISO_MAGIC_COLOR,
            );
        }
        "actor_vendor_idle" => {
            classic_draw_iso_shadow(&mut pixels, width as usize, height as usize, 16, 44, 10, 3);
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                12,
                18,
                8,
                22,
                0x3f9b58,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                9,
                25,
                14,
                7,
                CLASSIC_ISO_GOLD_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                12,
                10,
                8,
                7,
                0xfff0a8,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                21,
                18,
                6,
                5,
                CLASSIC_ISO_DOODAD_WOOD_COLOR,
            );
        }
        "prop_training_dummy" => {
            classic_draw_iso_shadow(&mut pixels, width as usize, height as usize, 16, 44, 12, 4);
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                14,
                10,
                4,
                31,
                CLASSIC_ISO_DOODAD_WOOD_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                8,
                18,
                16,
                5,
                CLASSIC_ISO_DOODAD_WOOD_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                10,
                12,
                12,
                10,
                CLASSIC_ISO_BANNER_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                12,
                14,
                8,
                6,
                CLASSIC_ISO_UNIT_ENEMY_COLOR,
            );
        }
        "prop_reward" => {
            classic_draw_iso_shadow(&mut pixels, width as usize, height as usize, 16, 28, 11, 3);
            classic_draw_iso_diamond(
                &mut pixels,
                width as usize,
                height as usize,
                16,
                13,
                24,
                12,
                CLASSIC_ISO_GOLD_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                9,
                11,
                14,
                5,
                0xfff0a8,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                13,
                7,
                6,
                8,
                CLASSIC_ISO_MAGIC_COLOR,
            );
        }
        "prop_arena_gate" => {
            classic_draw_iso_shadow(&mut pixels, width as usize, height as usize, 32, 44, 24, 5);
            classic_draw_iso_prism(
                &mut pixels,
                width as usize,
                height as usize,
                18,
                34,
                14,
                12,
                29,
                CLASSIC_ISO_WALL_COLOR,
            );
            classic_draw_iso_prism(
                &mut pixels,
                width as usize,
                height as usize,
                46,
                34,
                14,
                12,
                29,
                CLASSIC_ISO_WALL_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                16,
                9,
                32,
                7,
                CLASSIC_ISO_BANNER_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                21,
                18,
                22,
                4,
                CLASSIC_ISO_GOLD_COLOR,
            );
        }
        "prop_market_stall" => {
            classic_draw_iso_shadow(&mut pixels, width as usize, height as usize, 32, 44, 24, 5);
            classic_draw_iso_prism(
                &mut pixels,
                width as usize,
                height as usize,
                32,
                34,
                42,
                16,
                18,
                CLASSIC_ISO_DOODAD_WOOD_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                12,
                12,
                40,
                8,
                CLASSIC_ISO_BANNER_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                17,
                22,
                8,
                5,
                CLASSIC_ISO_GOLD_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                38,
                22,
                9,
                5,
                0x3f9b58,
            );
        }
        "prop_banner" => {
            classic_draw_iso_shadow(&mut pixels, width as usize, height as usize, 16, 44, 8, 3);
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                14,
                9,
                4,
                33,
                CLASSIC_ISO_DOODAD_WOOD_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                9,
                10,
                14,
                20,
                CLASSIC_ISO_BANNER_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                11,
                13,
                10,
                4,
                CLASSIC_ISO_GOLD_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                12,
                19,
                8,
                5,
                0xffffff,
            );
        }
        "marker_objective" => {
            classic_draw_iso_ellipse(
                &mut pixels,
                width as usize,
                height as usize,
                16,
                24,
                12,
                5,
                CLASSIC_ISO_UNIT_RING_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                14,
                7,
                4,
                14,
                CLASSIC_ISO_COMMAND_MARKER_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                12,
                5,
                8,
                4,
                0xffffff,
            );
        }
        "marker_interaction" => {
            classic_draw_iso_ellipse(
                &mut pixels,
                width as usize,
                height as usize,
                16,
                24,
                11,
                5,
                CLASSIC_ISO_MAGIC_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                8,
                11,
                16,
                4,
                CLASSIC_ISO_UNIT_RING_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                14,
                5,
                4,
                14,
                0xffffff,
            );
        }
        "rts_command_destination_marker" => {
            classic_draw_iso_ellipse(
                &mut pixels,
                width as usize,
                height as usize,
                24,
                28,
                20,
                8,
                CLASSIC_ISO_COMMAND_MARKER_COLOR,
            );
            classic_draw_iso_ellipse(
                &mut pixels,
                width as usize,
                height as usize,
                24,
                28,
                11,
                4,
                CLASSIC_ISO_FOUNDATION_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                20,
                12,
                8,
                4,
                0xffffff,
            );
        }
        "combat_attack_arc" => {
            for step in 0..28 {
                classic_draw_rect(
                    &mut pixels,
                    width as usize,
                    height as usize,
                    5 + step * 2,
                    34 - step / 2,
                    6,
                    3,
                    CLASSIC_ISO_ATTACK_ARC_COLOR,
                );
            }
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                44,
                14,
                10,
                3,
                0xfff3a4,
            );
        }
        "combat_hit_flash" => {
            classic_draw_iso_ellipse(
                &mut pixels,
                width as usize,
                height as usize,
                24,
                24,
                17,
                10,
                CLASSIC_ISO_HIT_FLASH_COLOR,
            );
            classic_draw_iso_ellipse(
                &mut pixels,
                width as usize,
                height as usize,
                24,
                24,
                8,
                5,
                0xfff3a4,
            );
        }
        "rts_unit_selection_ring" => {
            classic_draw_iso_ellipse(
                &mut pixels,
                width as usize,
                height as usize,
                24,
                30,
                20,
                8,
                CLASSIC_ISO_UNIT_RING_COLOR,
            );
            classic_draw_iso_ellipse(
                &mut pixels,
                width as usize,
                height as usize,
                24,
                30,
                14,
                5,
                CLASSIC_ISO_FOUNDATION_COLOR,
            );
        }
        "unit_health_bar" => {
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                2,
                5,
                28,
                7,
                CLASSIC_ISO_OUTLINE_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                4,
                7,
                18,
                3,
                CLASSIC_ISO_UNIT_HEALTH_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                22,
                7,
                6,
                3,
                CLASSIC_ISO_UNIT_DAMAGE_COLOR,
            );
        }
        "rts_foundation_shadow" => {
            classic_draw_iso_diamond(
                &mut pixels,
                width as usize,
                height as usize,
                48,
                24,
                86,
                28,
                CLASSIC_ISO_FOUNDATION_COLOR,
            );
            classic_draw_iso_ellipse(
                &mut pixels,
                width as usize,
                height as usize,
                48,
                30,
                38,
                8,
                CLASSIC_ISO_SHADOW_COLOR,
            );
        }
        "model_town_hall" => {
            classic_draw_iso_shadow(&mut pixels, width as usize, height as usize, 48, 84, 42, 8);
            classic_draw_iso_prism(
                &mut pixels,
                width as usize,
                height as usize,
                48,
                70,
                66,
                34,
                34,
                0x6f5c43,
            );
            classic_draw_iso_prism(
                &mut pixels,
                width as usize,
                height as usize,
                48,
                42,
                78,
                28,
                16,
                0x274a74,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                41,
                57,
                14,
                26,
                0x201812,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                25,
                50,
                10,
                8,
                0xf1c45d,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                61,
                50,
                10,
                8,
                0xf1c45d,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                32,
                32,
                32,
                4,
                0xf4d06f,
            );
            classic_draw_iso_prism(
                &mut pixels,
                width as usize,
                height as usize,
                28,
                58,
                18,
                12,
                28,
                0x4d3d32,
            );
            classic_draw_iso_prism(
                &mut pixels,
                width as usize,
                height as usize,
                68,
                58,
                18,
                12,
                28,
                0x4d3d32,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                30,
                28,
                36,
                3,
                0x111711,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                68,
                22,
                8,
                18,
                0x121006,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                69,
                17,
                6,
                5,
                0xf4d06f,
            );
        }
        "model_waygate" => {
            classic_draw_iso_shadow(&mut pixels, width as usize, height as usize, 48, 84, 36, 7);
            classic_draw_iso_prism(
                &mut pixels,
                width as usize,
                height as usize,
                30,
                66,
                24,
                30,
                54,
                0x5f6870,
            );
            classic_draw_iso_prism(
                &mut pixels,
                width as usize,
                height as usize,
                66,
                66,
                24,
                30,
                54,
                0x5f6870,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                23,
                20,
                50,
                8,
                0x31383d,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                27,
                28,
                42,
                8,
                0x7e8a92,
            );
            classic_draw_iso_ellipse(
                &mut pixels,
                width as usize,
                height as usize,
                48,
                57,
                20,
                30,
                0x6f7cff,
            );
            classic_draw_iso_ellipse(
                &mut pixels,
                width as usize,
                height as usize,
                48,
                57,
                11,
                20,
                0xc7d3ff,
            );
            classic_draw_iso_ellipse(
                &mut pixels,
                width as usize,
                height as usize,
                48,
                57,
                5,
                10,
                0x77c6cf,
            );
            for y in [34, 44, 54, 64] {
                classic_draw_rect(
                    &mut pixels,
                    width as usize,
                    height as usize,
                    23,
                    y,
                    14,
                    2,
                    0x31383d,
                );
                classic_draw_rect(
                    &mut pixels,
                    width as usize,
                    height as usize,
                    59,
                    y,
                    14,
                    2,
                    0x31383d,
                );
            }
        }
        "model_training_hall" => {
            classic_draw_iso_shadow(&mut pixels, width as usize, height as usize, 48, 84, 40, 8);
            classic_draw_iso_prism(
                &mut pixels,
                width as usize,
                height as usize,
                48,
                70,
                62,
                34,
                30,
                0x5d6365,
            );
            classic_draw_iso_prism(
                &mut pixels,
                width as usize,
                height as usize,
                48,
                45,
                74,
                25,
                14,
                0x8b3438,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                57,
                48,
                22,
                5,
                0xd8b15a,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                58,
                53,
                18,
                3,
                0x2b1b17,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                20,
                54,
                12,
                24,
                0x3d2720,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                17,
                50,
                18,
                6,
                0xccbc7a,
            );
            classic_draw_iso_prism(
                &mut pixels,
                width as usize,
                height as usize,
                28,
                64,
                18,
                12,
                22,
                0x4f3a2c,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                22,
                40,
                12,
                3,
                0xd8b15a,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                64,
                36,
                4,
                18,
                0x121006,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                58,
                34,
                16,
                3,
                0xccbc7a,
            );
        }
        "model_coliseum_stands" => {
            classic_draw_iso_shadow(&mut pixels, width as usize, height as usize, 64, 88, 58, 8);
            classic_draw_iso_prism(
                &mut pixels,
                width as usize,
                height as usize,
                64,
                76,
                96,
                28,
                14,
                0x525a60,
            );
            classic_draw_iso_prism(
                &mut pixels,
                width as usize,
                height as usize,
                64,
                62,
                82,
                24,
                12,
                0x69727a,
            );
            classic_draw_iso_prism(
                &mut pixels,
                width as usize,
                height as usize,
                64,
                49,
                66,
                20,
                10,
                0x808a92,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                22,
                34,
                84,
                5,
                0x9a3e4a,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                34,
                42,
                60,
                4,
                0xd8b15a,
            );
            for (x, y, w) in [(26, 48, 76), (34, 58, 60), (42, 68, 44)] {
                classic_draw_rect(
                    &mut pixels,
                    width as usize,
                    height as usize,
                    x,
                    y,
                    w,
                    2,
                    0x31383d,
                );
            }
            for x in [30, 48, 66, 84] {
                classic_draw_rect(
                    &mut pixels,
                    width as usize,
                    height as usize,
                    x,
                    30,
                    6,
                    11,
                    0x9a3e4a,
                );
            }
        }
        "model_tree_cluster_large" => {
            classic_draw_iso_shadow(&mut pixels, width as usize, height as usize, 48, 82, 42, 8);
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                44,
                52,
                8,
                34,
                0x5b3b22,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                33,
                60,
                7,
                24,
                0x4f341f,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                57,
                58,
                7,
                26,
                0x4f341f,
            );
            classic_draw_iso_ellipse(
                &mut pixels,
                width as usize,
                height as usize,
                35,
                43,
                24,
                15,
                0x1f6f3e,
            );
            classic_draw_iso_ellipse(
                &mut pixels,
                width as usize,
                height as usize,
                55,
                34,
                28,
                18,
                0x2f8b50,
            );
            classic_draw_iso_ellipse(
                &mut pixels,
                width as usize,
                height as usize,
                67,
                48,
                24,
                16,
                0x235f39,
            );
            classic_draw_iso_ellipse(
                &mut pixels,
                width as usize,
                height as usize,
                48,
                53,
                36,
                17,
                0x347f45,
            );
            for (x, y, color) in [
                (33, 34, 0x78bd62),
                (54, 25, 0x78bd62),
                (67, 41, 0x78bd62),
                (48, 46, 0x1b4e30),
            ] {
                classic_draw_iso_ellipse(
                    &mut pixels,
                    width as usize,
                    height as usize,
                    x,
                    y,
                    8,
                    4,
                    color,
                );
            }
        }
        frame if frame.starts_with("doodad_") => {
            let center_x = (width / 2) as i32;
            let top_y = height as i32 - 14;
            classic_draw_iso_procedural_model(
                &mut pixels,
                width as usize,
                height as usize,
                frame,
                center_x,
                top_y,
                24,
                12,
            );
            match frame {
                "doodad_rock_cluster" => {
                    classic_draw_iso_ellipse(
                        &mut pixels,
                        width as usize,
                        height as usize,
                        center_x + 4,
                        height as i32 - 24,
                        7,
                        3,
                        classic_lighten(CLASSIC_ISO_DOODAD_STONE_COLOR, 1, 4),
                    );
                }
                "doodad_barrel_stack" => {
                    classic_draw_rect(
                        &mut pixels,
                        width as usize,
                        height as usize,
                        center_x - 6,
                        height as i32 - 31,
                        12,
                        2,
                        0xffd07a,
                    );
                }
                "doodad_torch" => {
                    classic_draw_iso_ellipse(
                        &mut pixels,
                        width as usize,
                        height as usize,
                        center_x,
                        height as i32 - 36,
                        4,
                        3,
                        0xffd07a,
                    );
                }
                "doodad_crystal_cluster" => {
                    classic_draw_iso_ellipse(
                        &mut pixels,
                        width as usize,
                        height as usize,
                        center_x + 2,
                        height as i32 - 25,
                        7,
                        3,
                        0xb8fbff,
                    );
                }
                "doodad_bush_cluster" => {
                    for (dx, dy, rx, ry, color) in [
                        (-9, -17, 10, 5, CLASSIC_ISO_CANOPY_COLOR),
                        (5, -21, 12, 6, CLASSIC_ISO_CANOPY_LIGHT_COLOR),
                        (13, -14, 8, 4, CLASSIC_ISO_FOLIAGE_DARK_COLOR),
                    ] {
                        classic_draw_iso_ellipse(
                            &mut pixels,
                            width as usize,
                            height as usize,
                            center_x + dx,
                            height as i32 + dy,
                            rx,
                            ry,
                            color,
                        );
                    }
                }
                "doodad_ruins_column" => {
                    classic_draw_iso_prism(
                        &mut pixels,
                        width as usize,
                        height as usize,
                        center_x,
                        height as i32 - 18,
                        16,
                        12,
                        34,
                        CLASSIC_ISO_RUIN_COLOR,
                    );
                    classic_draw_rect(
                        &mut pixels,
                        width as usize,
                        height as usize,
                        center_x - 9,
                        height as i32 - 50,
                        18,
                        5,
                        CLASSIC_ISO_DOODAD_STONE_COLOR,
                    );
                    classic_draw_rect(
                        &mut pixels,
                        width as usize,
                        height as usize,
                        center_x - 4,
                        height as i32 - 43,
                        8,
                        24,
                        CLASSIC_ISO_RUIN_COLOR,
                    );
                }
                "doodad_gold_vein" => {
                    classic_draw_rect(
                        &mut pixels,
                        width as usize,
                        height as usize,
                        center_x - 15,
                        height as i32 - 9,
                        30,
                        3,
                        CLASSIC_ISO_SHADOW_COLOR,
                    );
                    classic_draw_iso_ellipse(
                        &mut pixels,
                        width as usize,
                        height as usize,
                        center_x,
                        height as i32 - 18,
                        17,
                        7,
                        CLASSIC_ISO_DOODAD_STONE_COLOR,
                    );
                    for (dx, dy, w) in [(-9, -21, 8), (0, -25, 11), (8, -18, 7)] {
                        classic_draw_rect(
                            &mut pixels,
                            width as usize,
                            height as usize,
                            center_x + dx,
                            height as i32 + dy,
                            w,
                            3,
                            CLASSIC_ISO_GOLD_VEIN_COLOR,
                        );
                    }
                    classic_draw_rect(
                        &mut pixels,
                        width as usize,
                        height as usize,
                        center_x - 5,
                        height as i32 - 16,
                        10,
                        2,
                        0xffe88a,
                    );
                }
                "doodad_signpost" => {
                    classic_draw_rect(
                        &mut pixels,
                        width as usize,
                        height as usize,
                        center_x - 2,
                        height as i32 - 36,
                        4,
                        23,
                        CLASSIC_ISO_DOODAD_WOOD_COLOR,
                    );
                    classic_draw_rect(
                        &mut pixels,
                        width as usize,
                        height as usize,
                        center_x - 14,
                        height as i32 - 34,
                        28,
                        9,
                        CLASSIC_ISO_BRIDGE_PLANK_COLOR,
                    );
                    classic_draw_rect(
                        &mut pixels,
                        width as usize,
                        height as usize,
                        center_x - 10,
                        height as i32 - 31,
                        20,
                        2,
                        CLASSIC_ISO_GOLD_COLOR,
                    );
                }
                _ => {}
            }
        }
        frame if classic_art_pack_neutral_unit_frame(frame) => {
            classic_draw_neutral_unit_sprite(&mut pixels, width as usize, height as usize, frame);
        }
        frame if frame.starts_with("actor_player") => {
            let accent = if frame.contains("north") {
                0x83c79d
            } else if frame.contains("east") {
                0xfff0a8
            } else if frame.contains("west") {
                0x7fd8e8
            } else {
                0xf2dc73
            };
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                3,
                15,
                10,
                1,
                CLASSIC_ISO_SHADOW_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                5,
                1,
                6,
                4,
                CLASSIC_ISO_OUTLINE_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                4,
                4,
                8,
                8,
                CLASSIC_ISO_OUTLINE_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                6,
                2,
                4,
                3,
                0xd8f0a0,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                5,
                5,
                6,
                6,
                0x3f9b58,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                6,
                6,
                4,
                1,
                0xd8f0a0,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                7,
                3,
                2,
                1,
                accent,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                4,
                7,
                2,
                4,
                accent,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                10,
                7,
                2,
                4,
                accent,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                5,
                11,
                2,
                4,
                0x26382e,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                9,
                11,
                2,
                4,
                0x26382e,
            );
            if frame.contains("_2") {
                classic_draw_rect(
                    &mut pixels,
                    width as usize,
                    height as usize,
                    3,
                    12,
                    3,
                    2,
                    0xf2dc73,
                );
            }
        }
        "actor_enemy" | "actor_enemy_idle" | "actor_enemy_attack" | "actor_enemy_hit" => {
            let body = if frame_id == "actor_enemy_hit" {
                0x8f3a3a
            } else {
                0xb94745
            };
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                3,
                15,
                10,
                1,
                CLASSIC_ISO_SHADOW_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                4,
                2,
                8,
                5,
                CLASSIC_ISO_OUTLINE_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                3,
                6,
                10,
                8,
                CLASSIC_ISO_OUTLINE_COLOR,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                5,
                3,
                6,
                4,
                0xff8c73,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                4,
                7,
                8,
                6,
                body,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                6,
                4,
                1,
                1,
                0xfff0a8,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                9,
                4,
                1,
                1,
                0xfff0a8,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                5,
                8,
                6,
                1,
                0xff8c73,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                3,
                8,
                2,
                4,
                0x3a2020,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                11,
                8,
                2,
                4,
                0x3a2020,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                5,
                13,
                2,
                2,
                0x2b1717,
            );
            classic_draw_rect(
                &mut pixels,
                width as usize,
                height as usize,
                9,
                13,
                2,
                2,
                0x2b1717,
            );
            if frame_id == "actor_enemy_attack" {
                classic_draw_rect(
                    &mut pixels,
                    width as usize,
                    height as usize,
                    12,
                    5,
                    3,
                    2,
                    0xff5c4d,
                );
                classic_draw_rect(
                    &mut pixels,
                    width as usize,
                    height as usize,
                    13,
                    7,
                    2,
                    4,
                    0xff5c4d,
                );
            }
            if frame_id == "actor_enemy_hit" {
                classic_draw_rect(
                    &mut pixels,
                    width as usize,
                    height as usize,
                    2,
                    2,
                    4,
                    2,
                    0xffb199,
                );
            }
        }
        _ => {}
    }
    pixels
}

#[cfg(not(target_os = "android"))]
pub(super) fn classic_draw_neutral_unit_sprite(
    pixels: &mut [u32],
    width: usize,
    height: usize,
    frame_id: &str,
) {
    let center_x = (width / 2) as i32;
    let foot_y = height as i32 - 4;
    let (body, accent, trim, dark, skin) = if frame_id.starts_with("actor_guard") {
        (
            CLASSIC_ISO_UNIT_GUARD_COLOR,
            0xa8d8ff,
            0xe8e0bd,
            0x2f3950,
            0xcaa878,
        )
    } else if frame_id.starts_with("actor_worker") {
        (
            CLASSIC_ISO_UNIT_WORKER_COLOR,
            0xf0be70,
            0xffe2a6,
            0x4f3726,
            0xc6925f,
        )
    } else {
        (
            CLASSIC_ISO_UNIT_CREEP_COLOR,
            0xd0a2ff,
            0xffe2a6,
            0x3a2448,
            0x9a79bd,
        )
    };
    classic_draw_iso_shadow(pixels, width, height, center_x, foot_y, 18, 5);
    classic_draw_iso_ellipse(
        pixels,
        width,
        height,
        center_x,
        foot_y - 1,
        13,
        5,
        CLASSIC_ISO_UNIT_RING_COLOR,
    );
    classic_draw_iso_ellipse(
        pixels,
        width,
        height,
        center_x,
        foot_y - 1,
        8,
        3,
        CLASSIC_ISO_FOUNDATION_COLOR,
    );
    classic_draw_rect(
        pixels,
        width,
        height,
        center_x - 6,
        foot_y - 35,
        12,
        8,
        CLASSIC_ISO_OUTLINE_COLOR,
    );
    classic_draw_rect(
        pixels,
        width,
        height,
        center_x - 8,
        foot_y - 27,
        16,
        21,
        CLASSIC_ISO_OUTLINE_COLOR,
    );
    classic_draw_rect(pixels, width, height, center_x - 4, foot_y - 34, 8, 6, skin);
    classic_draw_rect(
        pixels,
        width,
        height,
        center_x - 5,
        foot_y - 28,
        10,
        17,
        body,
    );
    classic_draw_rect(
        pixels,
        width,
        height,
        center_x - 8,
        foot_y - 25,
        16,
        4,
        accent,
    );
    classic_draw_rect(pixels, width, height, center_x - 4, foot_y - 20, 8, 2, trim);
    classic_draw_rect(pixels, width, height, center_x - 6, foot_y - 11, 4, 7, dark);
    classic_draw_rect(pixels, width, height, center_x + 2, foot_y - 11, 4, 7, dark);
    classic_draw_rect(
        pixels,
        width,
        height,
        center_x - 7,
        foot_y - 29,
        14,
        2,
        CLASSIC_RTS_FIDELITY_MODEL_HIGHLIGHT_COLOR,
    );
    classic_draw_rect(
        pixels,
        width,
        height,
        center_x - 9,
        foot_y - 20,
        18,
        2,
        CLASSIC_RTS_FIDELITY_MODEL_EDGE_COLOR,
    );
    classic_draw_rect(
        pixels,
        width,
        height,
        center_x - 12,
        foot_y - 17,
        4,
        6,
        CLASSIC_RTS_FIDELITY_ANIMATION_GHOST_COLOR,
    );
    classic_draw_rect(
        pixels,
        width,
        height,
        center_x + 8,
        foot_y - 17,
        4,
        6,
        CLASSIC_RTS_FIDELITY_ANIMATION_GHOST_COLOR,
    );

    if frame_id.starts_with("actor_guard") {
        classic_draw_rect(
            pixels,
            width,
            height,
            center_x - 12,
            foot_y - 25,
            5,
            13,
            accent,
        );
        classic_draw_rect(
            pixels,
            width,
            height,
            center_x - 11,
            foot_y - 22,
            3,
            7,
            body,
        );
        classic_draw_rect(
            pixels,
            width,
            height,
            center_x - 5,
            foot_y - 38,
            10,
            3,
            trim,
        );
        if frame_id.ends_with("_attack") {
            classic_draw_rect(
                pixels,
                width,
                height,
                center_x + 9,
                foot_y - 31,
                3,
                21,
                trim,
            );
            classic_draw_rect(
                pixels,
                width,
                height,
                center_x + 12,
                foot_y - 33,
                8,
                3,
                trim,
            );
            classic_draw_rect(
                pixels,
                width,
                height,
                center_x + 13,
                foot_y - 35,
                5,
                2,
                0xffffff,
            );
            classic_draw_rect(
                pixels,
                width,
                height,
                center_x + 17,
                foot_y - 30,
                10,
                3,
                CLASSIC_RTS_FIDELITY_ACTION_TRAIL_COLOR,
            );
            classic_draw_rect(
                pixels,
                width,
                height,
                center_x + 22,
                foot_y - 27,
                4,
                8,
                CLASSIC_RTS_FIDELITY_NPC_ACTION_COLOR,
            );
        } else {
            classic_draw_rect(
                pixels,
                width,
                height,
                center_x + 8,
                foot_y - 24,
                3,
                17,
                trim,
            );
        }
    } else if frame_id.starts_with("actor_worker") {
        classic_draw_rect(
            pixels,
            width,
            height,
            center_x - 11,
            foot_y - 23,
            5,
            13,
            dark,
        );
        classic_draw_rect(
            pixels,
            width,
            height,
            center_x + 7,
            foot_y - 24,
            3,
            19,
            trim,
        );
        classic_draw_rect(
            pixels,
            width,
            height,
            center_x + 5,
            foot_y - 26,
            8,
            3,
            CLASSIC_ISO_OUTLINE_COLOR,
        );
        if frame_id.ends_with("_carry") {
            classic_draw_rect(
                pixels,
                width,
                height,
                center_x + 9,
                foot_y - 30,
                15,
                11,
                CLASSIC_ISO_OUTLINE_COLOR,
            );
            classic_draw_rect(
                pixels,
                width,
                height,
                center_x + 10,
                foot_y - 29,
                13,
                9,
                CLASSIC_ISO_GOLD_COLOR,
            );
            classic_draw_rect(
                pixels,
                width,
                height,
                center_x + 12,
                foot_y - 27,
                9,
                2,
                trim,
            );
            classic_draw_rect(
                pixels,
                width,
                height,
                center_x + 11,
                foot_y - 25,
                11,
                2,
                CLASSIC_RTS_FIDELITY_MODEL_HIGHLIGHT_COLOR,
            );
        } else {
            classic_draw_rect(
                pixels,
                width,
                height,
                center_x - 13,
                foot_y - 30,
                5,
                18,
                trim,
            );
            classic_draw_rect(
                pixels,
                width,
                height,
                center_x - 15,
                foot_y - 32,
                9,
                3,
                CLASSIC_ISO_OUTLINE_COLOR,
            );
        }
    } else {
        classic_draw_rect(
            pixels,
            width,
            height,
            center_x - 8,
            foot_y - 38,
            5,
            5,
            accent,
        );
        classic_draw_rect(
            pixels,
            width,
            height,
            center_x + 3,
            foot_y - 38,
            5,
            5,
            accent,
        );
        classic_draw_rect(
            pixels,
            width,
            height,
            center_x - 10,
            foot_y - 24,
            5,
            12,
            dark,
        );
        classic_draw_rect(
            pixels,
            width,
            height,
            center_x + 5,
            foot_y - 24,
            5,
            12,
            dark,
        );
        classic_draw_rect(
            pixels,
            width,
            height,
            center_x - 3,
            foot_y - 31,
            2,
            2,
            0xffe2a6,
        );
        classic_draw_rect(
            pixels,
            width,
            height,
            center_x + 3,
            foot_y - 31,
            2,
            2,
            0xffe2a6,
        );
        if frame_id.ends_with("_attack") {
            classic_draw_rect(
                pixels,
                width,
                height,
                center_x + 10,
                foot_y - 27,
                10,
                4,
                accent,
            );
            classic_draw_rect(
                pixels,
                width,
                height,
                center_x + 16,
                foot_y - 25,
                5,
                6,
                CLASSIC_ISO_UNIT_DAMAGE_COLOR,
            );
            classic_draw_rect(
                pixels,
                width,
                height,
                center_x + 18,
                foot_y - 26,
                11,
                3,
                CLASSIC_RTS_FIDELITY_ACTION_TRAIL_COLOR,
            );
            classic_draw_rect(
                pixels,
                width,
                height,
                center_x - 18,
                foot_y - 25,
                9,
                4,
                accent,
            );
        }
    }

    classic_draw_rts_unit_model_depth_marks(pixels, width, height, frame_id, center_x, foot_y);
    classic_draw_rts_action_cadence_marks(pixels, width, height, frame_id, center_x, foot_y);
}

#[cfg(not(target_os = "android"))]

pub(super) fn classic_draw_art_pack_preview(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    specs: &[(&str, u32, u32, &str)],
    assets: &ClassicRuntimeAssets,
) {
    let cell_w = 160_i32;
    let cell_h = 105_i32;
    for (index, (frame_id, source_width, source_height, group)) in specs.iter().enumerate() {
        let x = (index as i32 % 4) * cell_w;
        let y = (index as i32 / 4) * cell_h;
        classic_draw_rect(
            buffer,
            width,
            height,
            x + 3,
            y + 3,
            cell_w - 6,
            cell_h - 6,
            0x121813,
        );
        classic_draw_text(
            buffer,
            width,
            height,
            x + 8,
            y + 8,
            group,
            1,
            CLASSIC_HUD_ACCENT_TEXT_COLOR,
        );
        classic_draw_text(
            buffer,
            width,
            height,
            x + 8,
            y + 20,
            frame_id,
            1,
            CLASSIC_HUD_TEXT_COLOR,
        );
        if let Some(frame_override) = assets.frame_override_pixels.get(*frame_id) {
            let scale = if *source_width <= 16 { 4 } else { 1 };
            let draw_w = frame_override.width as i32 * scale as i32;
            let draw_h = frame_override.height as i32 * scale as i32;
            classic_blit_pixels_scaled(
                buffer,
                width,
                height,
                &frame_override.pixels,
                frame_override.width,
                frame_override.height,
                x + (cell_w - draw_w) / 2,
                y + 96 - draw_h,
                scale,
            );
        } else {
            let pixels = classic_art_pack_pixels(frame_id, *source_width, *source_height);
            let scale = if *source_width <= 16 { 4 } else { 1 };
            let draw_w = *source_width as i32 * scale as i32;
            let draw_h = *source_height as i32 * scale as i32;
            classic_blit_pixels_scaled(
                buffer,
                width,
                height,
                &pixels,
                *source_width,
                *source_height,
                x + (cell_w - draw_w) / 2,
                y + 96 - draw_h,
                scale,
            );
        }
    }
}
