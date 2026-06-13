//! Bevy-free RTS runtime adapter math for camera, minimap, path preview, and UI hit tests.
//!
//! This crate is intentionally small: it owns deterministic adapter calculations while
//! `trnm-world-bevy` keeps renderer colors, pixels, assets, and Bevy integration.

use serde::{Deserialize, Serialize};

pub const TRNM_RTS_BEVY_RUNTIME_CONTRACT: &str = "trnm_rts_bevy_runtime_adapter_v1";

pub const TRNM_RTS_RUNTIME_MAP_WIDTH_TILES: i32 = 34;
pub const TRNM_RTS_RUNTIME_MAP_HEIGHT_TILES: i32 = 34;
pub const TRNM_RTS_RUNTIME_MAP_MIN_TILE: i32 = 1;
pub const TRNM_RTS_RUNTIME_MAP_MAX_X: i32 = 32;
pub const TRNM_RTS_RUNTIME_MAP_MAX_Y: i32 = 32;
pub const TRNM_RTS_RUNTIME_CAMERA_ORIGIN_X: i32 = 17;
pub const TRNM_RTS_RUNTIME_CAMERA_ORIGIN_Y: i32 = 17;
pub const TRNM_RTS_RUNTIME_TILE_WORLD_W: f32 = 72.0;
pub const TRNM_RTS_RUNTIME_TILE_WORLD_H: f32 = 48.0;

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct RtsRuntimeVec2 {
    pub x: f32,
    pub y: f32,
}

impl RtsRuntimeVec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RtsScrollableMapCameraState {
    pub center_x: f32,
    pub center_y: f32,
    pub zoom: f32,
}

impl Default for RtsScrollableMapCameraState {
    fn default() -> Self {
        Self {
            center_x: 0.0,
            center_y: 0.0,
            zoom: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RtsScrollableMapCameraConfig {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub keyboard_speed: f32,
    pub edge_speed: f32,
    pub drag_world_units_per_pixel: f32,
    pub wheel_zoom_step: f32,
    pub edge_band_pixels: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RtsScrollableMapCameraStep {
    pub source: String,
    pub before: RtsScrollableMapCameraState,
    pub after: RtsScrollableMapCameraState,
    pub pan_delta_x: f32,
    pub pan_delta_y: f32,
    pub zoom_delta: f32,
    pub clamped: bool,
    pub minimap_tile_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsCameraMinimapViewportRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsRuntimeRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsRuntimeGridSpec {
    pub origin_x: i32,
    pub origin_y: i32,
    pub columns: usize,
    pub count: usize,
    pub stride_x: i32,
    pub stride_y: i32,
    pub slot_width: i32,
    pub slot_height: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsRuntimeMapLayoutInput {
    pub viewport_width: i32,
    pub viewport_height: i32,
    pub map_width_tiles: i32,
    pub map_height_tiles: i32,
    pub map_origin_x: i32,
    pub map_origin_y: i32,
    pub right_reserved_px: i32,
    pub bottom_reserved_px: i32,
    pub min_map_width_px: i32,
    pub min_map_height_px: i32,
    pub cell_width_min: i32,
    pub cell_width_max: i32,
    pub cell_height_min: i32,
    pub cell_height_max: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsRuntimeMapProjection {
    pub map_x: i32,
    pub map_y: i32,
    pub cell_w: i32,
    pub cell_h: i32,
    pub map_w: i32,
    pub map_h: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsRuntimeTerrainSeeds {
    pub surface_seed: i32,
    pub detail_seed: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsRuntimeTileLineStep {
    pub step_index: i32,
    pub step_count: i32,
    pub tile_x: i32,
    pub tile_y: i32,
}

pub fn rts_large_map_clamp_tile(tile: (i32, i32)) -> (i32, i32) {
    (
        tile.0
            .clamp(TRNM_RTS_RUNTIME_MAP_MIN_TILE, TRNM_RTS_RUNTIME_MAP_MAX_X),
        tile.1
            .clamp(TRNM_RTS_RUNTIME_MAP_MIN_TILE, TRNM_RTS_RUNTIME_MAP_MAX_Y),
    )
}

pub fn rts_runtime_map_projection(input: RtsRuntimeMapLayoutInput) -> RtsRuntimeMapProjection {
    let map_width_tiles = input.map_width_tiles.max(1);
    let map_height_tiles = input.map_height_tiles.max(1);
    let available_w = (input.viewport_width - input.right_reserved_px).max(input.min_map_width_px);
    let available_h = (input.viewport_height - input.bottom_reserved_px - input.map_origin_y)
        .max(input.min_map_height_px);
    let cell_w = (available_w / map_width_tiles)
        .clamp(input.cell_width_min, input.cell_width_max)
        .max(1);
    let cell_h = (available_h / map_height_tiles)
        .clamp(input.cell_height_min, input.cell_height_max)
        .max(1);
    RtsRuntimeMapProjection {
        map_x: input.map_origin_x,
        map_y: input.map_origin_y,
        cell_w,
        cell_h,
        map_w: cell_w * map_width_tiles,
        map_h: cell_h * map_height_tiles,
    }
}

pub fn rts_runtime_tile_screen_origin(
    origin_x: i32,
    origin_y: i32,
    cell_w: i32,
    cell_h: i32,
    tile: (i32, i32),
) -> (i32, i32) {
    (origin_x + tile.0 * cell_w, origin_y + tile.1 * cell_h)
}

pub fn rts_runtime_tile_screen_rect(
    projection: RtsRuntimeMapProjection,
    tile: (i32, i32),
) -> RtsRuntimeRect {
    let (x, y) = rts_runtime_tile_screen_origin(
        projection.map_x,
        projection.map_y,
        projection.cell_w,
        projection.cell_h,
        tile,
    );
    RtsRuntimeRect {
        x,
        y,
        width: projection.cell_w,
        height: projection.cell_h,
    }
}

pub fn rts_runtime_terrain_seeds(tile: (i32, i32)) -> RtsRuntimeTerrainSeeds {
    RtsRuntimeTerrainSeeds {
        surface_seed: (tile.0 * 37 + tile.1 * 19 + (tile.0 - tile.1).abs() * 11) % 17,
        detail_seed: (tile.0 * 13 + tile.1 * 17 + (tile.0 - tile.1).abs() * 7) % 23,
    }
}

pub fn rts_runtime_tile_line(from: (i32, i32), to: (i32, i32)) -> Vec<RtsRuntimeTileLineStep> {
    let dx = to.0 - from.0;
    let dy = to.1 - from.1;
    let steps = dx.abs().max(dy.abs());
    if steps == 0 {
        return vec![RtsRuntimeTileLineStep {
            step_index: 0,
            step_count: 0,
            tile_x: from.0,
            tile_y: from.1,
        }];
    }

    (0..=steps)
        .map(|step| RtsRuntimeTileLineStep {
            step_index: step,
            step_count: steps,
            tile_x: from.0 + (dx * step) / steps,
            tile_y: from.1 + (dy * step) / steps,
        })
        .collect()
}

pub fn rts_large_map_tile_to_camera_center(tile: (i32, i32)) -> RtsRuntimeVec2 {
    let tile = rts_large_map_clamp_tile(tile);
    RtsRuntimeVec2::new(
        (tile.0 - TRNM_RTS_RUNTIME_CAMERA_ORIGIN_X) as f32 * TRNM_RTS_RUNTIME_TILE_WORLD_W,
        -((tile.1 - TRNM_RTS_RUNTIME_CAMERA_ORIGIN_Y) as f32) * TRNM_RTS_RUNTIME_TILE_WORLD_H,
    )
}

pub fn rts_minimap_cell_origin(
    origin_x: i32,
    origin_y: i32,
    cell_w: i32,
    cell_h: i32,
    tile: (i32, i32),
) -> (i32, i32) {
    let tile = rts_large_map_clamp_tile(tile);
    (
        origin_x + (tile.0 - TRNM_RTS_RUNTIME_MAP_MIN_TILE) * cell_w,
        origin_y + (tile.1 - TRNM_RTS_RUNTIME_MAP_MIN_TILE) * cell_h,
    )
}

pub fn rts_large_map_cell_col(tile: (i32, i32)) -> i32 {
    rts_large_map_clamp_tile(tile).0 - TRNM_RTS_RUNTIME_MAP_MIN_TILE
}

pub fn rts_large_map_cell_row(tile: (i32, i32)) -> i32 {
    rts_large_map_clamp_tile(tile).1 - TRNM_RTS_RUNTIME_MAP_MIN_TILE
}

pub fn rts_scrollable_map_camera_config() -> RtsScrollableMapCameraConfig {
    let min_camera = rts_large_map_tile_to_camera_center((
        TRNM_RTS_RUNTIME_MAP_MIN_TILE,
        TRNM_RTS_RUNTIME_MAP_MAX_Y,
    ));
    let max_camera = rts_large_map_tile_to_camera_center((
        TRNM_RTS_RUNTIME_MAP_MAX_X,
        TRNM_RTS_RUNTIME_MAP_MIN_TILE,
    ));
    RtsScrollableMapCameraConfig {
        min_x: min_camera.x,
        max_x: max_camera.x,
        min_y: min_camera.y,
        max_y: max_camera.y,
        min_zoom: 0.66,
        max_zoom: 1.85,
        keyboard_speed: 280.0,
        edge_speed: 360.0,
        drag_world_units_per_pixel: 1.15,
        wheel_zoom_step: 0.12,
        edge_band_pixels: 24.0,
    }
}

pub fn clamp_rts_scrollable_map_camera_state(
    state: RtsScrollableMapCameraState,
    config: RtsScrollableMapCameraConfig,
) -> RtsScrollableMapCameraState {
    RtsScrollableMapCameraState {
        center_x: state.center_x.clamp(config.min_x, config.max_x),
        center_y: state.center_y.clamp(config.min_y, config.max_y),
        zoom: state.zoom.clamp(config.min_zoom, config.max_zoom),
    }
}

pub fn apply_rts_scrollable_map_camera_input(
    source: &str,
    state: RtsScrollableMapCameraState,
    config: RtsScrollableMapCameraConfig,
    pan_delta: RtsRuntimeVec2,
    zoom_delta: f32,
    minimap_jump: Option<(&str, RtsRuntimeVec2)>,
) -> RtsScrollableMapCameraStep {
    let mut next = state;
    if let Some((_tile_id, center)) = minimap_jump {
        next.center_x = center.x;
        next.center_y = center.y;
    } else {
        next.center_x += pan_delta.x;
        next.center_y += pan_delta.y;
    }
    next.zoom += zoom_delta;
    let clamped_next = clamp_rts_scrollable_map_camera_state(next, config);
    RtsScrollableMapCameraStep {
        source: source.to_string(),
        before: state,
        after: clamped_next,
        pan_delta_x: pan_delta.x,
        pan_delta_y: pan_delta.y,
        zoom_delta,
        clamped: (clamped_next.center_x - next.center_x).abs() > f32::EPSILON
            || (clamped_next.center_y - next.center_y).abs() > f32::EPSILON
            || (clamped_next.zoom - next.zoom).abs() > f32::EPSILON,
        minimap_tile_id: minimap_jump.map(|(tile_id, _)| tile_id.to_string()),
    }
}

pub fn rts_scrollable_map_camera_focus_tile(state: RtsScrollableMapCameraState) -> (i32, i32) {
    rts_large_map_clamp_tile((
        (state.center_x / TRNM_RTS_RUNTIME_TILE_WORLD_W).round() as i32
            + TRNM_RTS_RUNTIME_CAMERA_ORIGIN_X,
        (-state.center_y / TRNM_RTS_RUNTIME_TILE_WORLD_H).round() as i32
            + TRNM_RTS_RUNTIME_CAMERA_ORIGIN_Y,
    ))
}

pub fn rts_camera_minimap_viewport_rect(
    state: RtsScrollableMapCameraState,
    minimap_width: i32,
    minimap_height: i32,
) -> RtsCameraMinimapViewportRect {
    let config = rts_scrollable_map_camera_config();
    let normalized_x =
        ((state.center_x - config.min_x) / (config.max_x - config.min_x)).clamp(0.0, 1.0);
    let normalized_y =
        ((state.center_y - config.min_y) / (config.max_y - config.min_y)).clamp(0.0, 1.0);
    let width = ((minimap_width as f32 * 0.28) / state.zoom).round() as i32;
    let height = ((minimap_height as f32 * 0.34) / state.zoom).round() as i32;
    let width = width.clamp(18, (minimap_width - 8).max(18));
    let height = height.clamp(14, (minimap_height - 8).max(14));
    let max_x = (minimap_width - width).max(0);
    let max_y = (minimap_height - height).max(0);
    RtsCameraMinimapViewportRect {
        x: ((normalized_x * max_x as f32).round() as i32).clamp(0, max_x),
        y: (((1.0 - normalized_y) * max_y as f32).round() as i32).clamp(0, max_y),
        width,
        height,
    }
}

pub fn rts_camera_minimap_revealed_tiles(focus_tile: (i32, i32)) -> Vec<String> {
    let mut tile_ids = Vec::new();
    for y_delta in -1..=1 {
        for x_delta in -1..=1 {
            let (tile_x, tile_y) =
                rts_large_map_clamp_tile((focus_tile.0 + x_delta, focus_tile.1 + y_delta));
            let tile_id = rts_runtime_tile_id((tile_x, tile_y));
            if !tile_ids.contains(&tile_id) {
                tile_ids.push(tile_id);
            }
        }
    }
    tile_ids
}

pub fn rts_camera_minimap_selection_follow_step(
    source: &str,
    state: RtsScrollableMapCameraState,
    selected_unit_id: &str,
    selected_unit_center: RtsRuntimeVec2,
) -> RtsScrollableMapCameraStep {
    apply_rts_scrollable_map_camera_input(
        source,
        state,
        rts_scrollable_map_camera_config(),
        RtsRuntimeVec2::ZERO,
        0.0,
        Some((selected_unit_id, selected_unit_center)),
    )
}

pub fn rts_scrollable_map_viewport_center() -> RtsRuntimeVec2 {
    rts_large_map_tile_to_camera_center((8, 8))
}

pub fn rts_runtime_tile_id(tile: (i32, i32)) -> String {
    format!("{},{}", tile.0, tile.1)
}

fn rts_string_vec<const N: usize>(values: [&str; N]) -> Vec<String> {
    values.into_iter().map(str::to_string).collect()
}

fn rts_line_path_tiles(start: (i32, i32), end: (i32, i32)) -> Vec<String> {
    let dx = end.0 - start.0;
    let dy = end.1 - start.1;
    let steps = dx.abs().max(dy.abs()).max(1);
    let mut tiles = Vec::new();
    for step in 1..=steps {
        let tile = (start.0 + (dx * step) / steps, start.1 + (dy * step) / steps);
        let tile_id = rts_runtime_tile_id(tile);
        if tiles.last() != Some(&tile_id) {
            tiles.push(tile_id);
        }
    }
    tiles
}

fn rts_parse_tile_id(value: &str) -> Option<(i32, i32)> {
    let (x, y) = value.split_once(',')?;
    Some((x.parse().ok()?, y.parse().ok()?))
}

pub fn rts_move_follow_target(formation: &str) -> Option<&str> {
    formation
        .strip_prefix("follow:")
        .map(str::trim)
        .filter(|target_id| !target_id.is_empty())
}

pub fn rts_move_formation_kind(formation: &str) -> &str {
    if rts_move_follow_target(formation).is_some() {
        "follow"
    } else {
        formation
    }
}

pub fn rts_path_tiles_for_destination(destination_tile: (i32, i32)) -> Vec<String> {
    if destination_tile == (8, 4) {
        vec!["6,5".to_string(), "7,5".to_string(), "8,4".to_string()]
    } else if destination_tile == (9, 2) {
        vec![
            "6,5".to_string(),
            "7,4".to_string(),
            "8,3".to_string(),
            "9,2".to_string(),
        ]
    } else {
        rts_line_path_tiles((5, 5), destination_tile)
    }
}

pub fn rts_blocked_tiles_for_destination(destination_tile: (i32, i32)) -> Vec<String> {
    if destination_tile == (8, 4) {
        vec!["7,4".to_string()]
    } else {
        Vec::new()
    }
}

pub fn rts_formation_slots_for_destination(
    destination_tile: (i32, i32),
    formation: &str,
) -> Vec<String> {
    let (x, y) = destination_tile;
    let slots = match formation {
        "line" => [(x - 1, y), (x, y), (x + 1, y), (x + 2, y)],
        "rally" => [(x - 1, y + 1), (x, y), (x + 1, y), (x, y + 1)],
        "split" => [(x - 1, y), (x + 1, y), (x - 1, y + 1), (x + 1, y + 1)],
        "wedge" => [(x, y), (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)],
        _ => [(x, y), (x - 1, y), (x, y + 1), (x + 1, y)],
    };
    slots.into_iter().map(rts_runtime_tile_id).collect()
}

pub fn rts_disperse_slots_for_destination(destination_tile: (i32, i32)) -> Vec<String> {
    if destination_tile == (8, 4) {
        vec![
            "6,5".to_string(),
            "7,5".to_string(),
            "8,4".to_string(),
            "8,5".to_string(),
        ]
    } else if destination_tile == (6, 5) {
        vec![
            "5,5".to_string(),
            "6,4".to_string(),
            "6,6".to_string(),
            "7,5".to_string(),
        ]
    } else {
        Vec::new()
    }
}

pub fn rts_engagement_tiles_for_target(target_id: &str) -> Vec<String> {
    if target_id == "enemy_barracks" {
        rts_string_vec(["9,3", "10,3", "10,2", "11,2"])
    } else if target_id == "forest_creep_camp" {
        rts_string_vec(["8,3", "8,2", "9,3", "7,3"])
    } else if target_id == "square_creep_wander" {
        rts_string_vec(["8,4", "9,4", "9,3", "10,4"])
    } else if target_id == "arena_creep_attack" {
        rts_string_vec(["6,5", "6,4", "7,5", "5,5"])
    } else {
        rts_string_vec(["6,5", "6,4"])
    }
}

pub fn rts_contact_flash_tiles_for_target(target_id: &str) -> Vec<String> {
    if target_id == "enemy_barracks" {
        rts_string_vec(["10,3", "10,2", "11,2"])
    } else if target_id == "forest_creep_camp" {
        rts_string_vec(["8,3", "9,3"])
    } else if target_id == "square_creep_wander" {
        rts_string_vec(["9,4", "10,4"])
    } else if target_id == "arena_creep_attack" {
        rts_string_vec(["6,5", "6,4"])
    } else {
        rts_string_vec(["6,5"])
    }
}

pub fn rts_target_tile_for_id(target_id: &str, fallback_index: usize) -> (i32, i32) {
    match target_id {
        "arena_creep_attack" => (6, 5),
        "arena_guard_support" => (6, 4),
        "arena_worker_support" => (7, 5),
        "forest_creep_camp" => (8, 3),
        "forest_stalker_support" => (8, 2),
        "forest_shaman_support" => (9, 3),
        "square_creep_wander" => (9, 4),
        "enemy_watch_post" => (10, 2),
        "enemy_barracks" => (10, 3),
        "enemy_resource_vault" => (11, 2),
        _ => (6 + fallback_index as i32, 5),
    }
}

pub fn rts_target_priority_ids_for_target(target_id: &str) -> Vec<String> {
    if target_id == "enemy_barracks" {
        rts_string_vec(["enemy_barracks", "enemy_watch_post", "enemy_resource_vault"])
    } else if target_id == "forest_creep_camp" {
        rts_string_vec([
            "forest_creep_camp",
            "forest_stalker_support",
            "forest_shaman_support",
        ])
    } else if target_id == "arena_creep_attack" {
        rts_string_vec([
            "arena_creep_attack",
            "arena_guard_support",
            "arena_worker_support",
        ])
    } else if target_id == "square_creep_wander" {
        rts_string_vec([
            "square_creep_wander",
            "forest_creep_camp",
            "arena_creep_attack",
        ])
    } else {
        vec![target_id.to_string()]
    }
}

pub fn rts_threat_levels_for_target(target_id: &str) -> Vec<u8> {
    if target_id == "enemy_barracks" {
        vec![88, 66, 41]
    } else if target_id == "forest_creep_camp" {
        vec![92, 70, 46]
    } else if target_id == "square_creep_wander" {
        vec![86, 54, 28]
    } else if target_id == "arena_creep_attack" {
        vec![100, 64, 32]
    } else {
        vec![72]
    }
}

pub fn rts_projectile_trail_tiles_for_target(target_id: &str) -> Vec<String> {
    if target_id == "enemy_barracks" {
        rts_string_vec(["7,4", "8,4", "9,3", "10,3"])
    } else if target_id == "forest_creep_camp" {
        rts_string_vec(["5,5", "6,5", "7,4", "8,3"])
    } else if target_id == "square_creep_wander" {
        rts_string_vec(["5,5", "6,5", "8,4", "9,4"])
    } else if target_id == "arena_creep_attack" {
        rts_string_vec(["5,5", "5,4", "6,4", "6,5"])
    } else {
        rts_string_vec(["5,5", "6,5"])
    }
}

pub fn rts_ability_effect_tiles_for_target(target_id: &str, ability_id: &str) -> Vec<String> {
    if target_id == "enemy_barracks" && ability_id == "guard_break" {
        rts_string_vec(["10,3", "10,2", "11,2", "9,3"])
    } else if target_id == "forest_creep_camp" && ability_id == "guard_break" {
        rts_string_vec(["8,3", "8,2", "9,3", "7,3"])
    } else if target_id == "forest_creep_camp" {
        rts_string_vec(["8,3", "8,2", "9,3"])
    } else if target_id == "arena_creep_attack" && ability_id == "guard_break" {
        rts_string_vec(["6,5", "6,4", "7,5", "5,5"])
    } else if target_id == "arena_creep_attack" {
        rts_string_vec(["6,5", "6,4", "7,5"])
    } else {
        vec![target_id.to_string()]
    }
}

pub fn rts_damage_ticks_for_ability(ability_id: &str) -> Vec<u8> {
    match ability_id {
        "guard_break" => vec![16, 21, 35],
        "focus_fire" => vec![28],
        _ => vec![18],
    }
}

pub fn rts_projectile_id_for_ability(ability_id: &str) -> &'static str {
    match ability_id {
        "guard_break" => "guard_break_bolt",
        "focus_fire" => "focus_fire_volley",
        _ => "guard_volley",
    }
}

pub fn rts_ai_wave_unit_ids_for_pressure(pressure_id: &str) -> Vec<String> {
    if pressure_id == "skirmish_wave" {
        rts_string_vec(["lane_scout", "mirror_raider", "siege_runner"])
    } else {
        rts_string_vec(["lane_scout"])
    }
}

pub fn rts_ai_pressure_tiles_for_pressure(pressure_id: &str) -> Vec<String> {
    if pressure_id == "skirmish_wave" {
        rts_string_vec(["9,3", "8,4", "7,4", "6,5"])
    } else {
        rts_string_vec(["8,4", "7,4"])
    }
}

pub fn rts_ai_counter_tiles_for_pressure(pressure_id: &str) -> Vec<String> {
    if pressure_id == "skirmish_wave" {
        rts_string_vec(["5,5", "6,5", "6,4", "7,5"])
    } else {
        rts_string_vec(["5,5", "6,5"])
    }
}

pub fn rts_enemy_pressure_wave_units_for_id(wave_id: &str) -> Vec<String> {
    if wave_id == "raider_wave" {
        rts_string_vec(["enemy_raider", "enemy_signal_guard", "enemy_sapper"])
    } else {
        rts_string_vec(["enemy_raider"])
    }
}

pub fn rts_enemy_pressure_lane_tiles_for_wave(wave_id: &str) -> Vec<String> {
    if wave_id == "raider_wave" {
        rts_string_vec(["10,2", "9,3", "8,4", "7,4", "6,5"])
    } else {
        rts_string_vec(["9,3", "8,4"])
    }
}

pub fn rts_scout_route_tiles_for_recon(recon_id: &str) -> Vec<String> {
    if recon_id == "enemy_base" {
        rts_string_vec(["5,5", "6,4", "7,4", "8,3", "9,2", "10,2"])
    } else if recon_id == "watchtower_scan" {
        rts_string_vec(["5,5", "6,5", "7,4"])
    } else {
        rts_string_vec(["5,5", "6,5", "7,5"])
    }
}

pub fn rts_fog_reveal_tiles_for_recon(recon_id: &str, kind: &str) -> Vec<String> {
    if recon_id == "enemy_base" && kind == "mark" {
        rts_string_vec([
            "7,4", "8,3", "8,2", "9,2", "9,3", "10,2", "10,3", "11,1", "11,2",
        ])
    } else if recon_id == "enemy_base" && kind == "sweep" {
        rts_string_vec(["7,4", "8,3", "9,2", "9,3", "10,2", "10,3", "11,2"])
    } else if recon_id == "enemy_base" {
        rts_string_vec(["7,4", "8,3", "9,2", "10,2"])
    } else if recon_id == "watchtower_scan" {
        rts_string_vec(["6,4", "7,4", "7,3", "8,3", "8,2"])
    } else {
        rts_string_vec(["5,5", "6,5", "7,5"])
    }
}

pub fn rts_enemy_structures_for_recon(recon_id: &str, kind: &str) -> Vec<String> {
    if recon_id == "enemy_base" && kind == "mark" {
        rts_string_vec(["enemy_watch_post", "enemy_barracks", "enemy_resource_vault"])
    } else if recon_id == "enemy_base" && kind == "sweep" {
        rts_string_vec(["enemy_watch_post", "enemy_barracks"])
    } else if recon_id == "enemy_base" || recon_id == "watchtower_scan" {
        rts_string_vec(["enemy_watch_post"])
    } else {
        Vec::new()
    }
}

pub fn rts_enemy_units_for_recon(recon_id: &str, kind: &str) -> Vec<String> {
    if recon_id == "enemy_base" && kind == "mark" {
        rts_string_vec(["enemy_scout", "enemy_worker", "enemy_guard"])
    } else if recon_id == "enemy_base" && kind == "sweep" {
        rts_string_vec(["enemy_scout", "enemy_worker"])
    } else if recon_id == "enemy_base" || recon_id == "watchtower_scan" {
        rts_string_vec(["enemy_scout"])
    } else {
        Vec::new()
    }
}

pub fn rts_enemy_structure_tile_for_id(structure_id: &str, index: usize) -> (i32, i32) {
    match structure_id {
        "enemy_watch_post" => (10, 2),
        "enemy_barracks" => (10, 3),
        "enemy_resource_vault" => (11, 2),
        _ => (10 + (index as i32 % 2), 2 + (index as i32 % 2)),
    }
}

pub fn rts_enemy_unit_tile_for_id(unit_id: &str, index: usize) -> (i32, i32) {
    match unit_id {
        "enemy_scout" => (9, 2),
        "enemy_worker" => (10, 3),
        "enemy_guard" => (11, 2),
        "enemy_raider" => (9, 3),
        "enemy_signal_guard" => (10, 3),
        "enemy_sapper" => (11, 2),
        _ => (9 + (index as i32 % 3), 2),
    }
}

pub fn rts_base_assault_path_tiles_for_target(target_id: &str, tile_id: &str) -> Vec<String> {
    if target_id == "enemy_barracks" {
        rts_string_vec(["5,5", "6,5", "7,4", "8,4", "9,3", tile_id])
    } else {
        let target_tile = rts_parse_tile_id(tile_id).unwrap_or((10, 3));
        rts_line_path_tiles((5, 5), target_tile)
    }
}

pub fn rts_base_assault_targets_for_id(target_id: &str) -> Vec<String> {
    if target_id == "enemy_barracks" {
        rts_string_vec(["enemy_watch_post", "enemy_barracks", "enemy_resource_vault"])
    } else {
        vec![target_id.to_string()]
    }
}

pub fn rts_aftermath_debris_tiles_for_id(structure_id: &str, tile_id: &str) -> Vec<String> {
    if structure_id == "enemy_barracks" {
        rts_string_vec(["9,3", "10,3", "10,4", "11,3"])
    } else {
        let tile = rts_parse_tile_id(tile_id).unwrap_or((10, 3));
        vec![
            format!("{},{}", tile.0.saturating_sub(1), tile.1),
            format!("{},{}", tile.0, tile.1),
            format!("{},{}", tile.0, tile.1 + 1),
        ]
    }
}

pub fn rts_aftermath_smoke_tiles_for_id(structure_id: &str, tile_id: &str) -> Vec<String> {
    if structure_id == "enemy_barracks" {
        rts_string_vec(["10,2", "10,3", "11,3"])
    } else {
        let tile = rts_parse_tile_id(tile_id).unwrap_or((10, 3));
        vec![
            format!("{},{}", tile.0, tile.1.saturating_sub(1)),
            format!("{},{}", tile.0, tile.1),
        ]
    }
}

pub fn rts_commander_aura_tiles_for_id(commander_id: &str) -> Vec<String> {
    if commander_id == "mirror_captain" {
        rts_string_vec(["6,5", "7,4", "8,4", "9,3", "10,3"])
    } else {
        rts_string_vec(["5,5", "6,5", "7,4"])
    }
}

pub fn rts_loot_items_for_id(source_id: &str) -> Vec<String> {
    if source_id == "enemy_barracks" {
        rts_string_vec([
            "barracks_map_cache",
            "field_banner_relic",
            "repair_kit_crate",
        ])
    } else {
        vec![format!("{source_id}_field_cache")]
    }
}

pub fn rts_expansion_tiles_for_id(expansion_id: &str, tile_id: &str) -> Vec<String> {
    if expansion_id == "forest_relay" {
        rts_string_vec(["8,2", "9,2", "10,2", "9,3", "10,3"])
    } else {
        let tile = rts_parse_tile_id(tile_id).unwrap_or((9, 2));
        vec![
            format!("{},{}", tile.0.saturating_sub(1), tile.1),
            format!("{},{}", tile.0, tile.1),
            format!("{},{}", tile.0 + 1, tile.1),
        ]
    }
}

pub fn rts_expansion_structure_tile_for_id(structure_id: &str) -> (i32, i32) {
    match structure_id {
        "relay_outpost" => (9, 2),
        "relay_foundry" => (9, 2),
        "relay_storehouse" => (10, 2),
        "watch_lantern" => (8, 3),
        _ => (9, 2),
    }
}

pub fn rts_expansion_workers_for_line(line_id: &str) -> Vec<String> {
    if line_id == "gold_line" {
        rts_string_vec([
            "expansion_worker_alpha",
            "expansion_worker_beta",
            "expansion_worker_gamma",
        ])
    } else {
        vec![format!("{line_id}_worker")]
    }
}

pub fn rts_counterattack_units_for_wave(wave_id: &str) -> Vec<String> {
    if wave_id == "counter_wave" {
        rts_string_vec([
            "counter_raider_alpha",
            "counter_raider_beta",
            "counter_sapper",
        ])
    } else {
        vec![format!("{wave_id}_raider")]
    }
}

pub fn rts_counterattack_route_tiles_for_wave(wave_id: &str, tile_id: &str) -> Vec<String> {
    if wave_id == "counter_wave" {
        rts_string_vec(["11,2", "10,2", "9,3", tile_id, "7,4", "9,2"])
    } else {
        let tile = rts_parse_tile_id(tile_id).unwrap_or((8, 3));
        vec![
            format!("{},{}", tile.0 + 2, tile.1.saturating_sub(1)),
            format!("{},{}", tile.0 + 1, tile.1),
            format!("{},{}", tile.0, tile.1),
        ]
    }
}

pub fn rts_objective_tiles_for_id(objective_id: &str, tile_id: &str) -> Vec<String> {
    if objective_id == "relay_beacon" {
        rts_string_vec(["6,5", "6,4", "7,5", "9,2"])
    } else if objective_id == "forest_relay" {
        rts_string_vec(["8,3", "9,2", "9,3"])
    } else {
        vec![tile_id.to_string()]
    }
}

pub fn rts_creep_camp_tiles_for_id(camp_id: &str, tile_id: &str) -> Vec<String> {
    if camp_id == "forest_creep_camp" {
        rts_string_vec(["8,3", "8,2", "9,3", "9,2"])
    } else {
        vec![tile_id.to_string()]
    }
}

pub fn rts_terrain_route_tiles_for_camp(camp_id: &str) -> Vec<String> {
    if camp_id == "forest_creep_camp" {
        rts_string_vec(["5,5", "6,5", "7,4", "8,3"])
    } else {
        rts_string_vec(["5,5", "6,5"])
    }
}

pub fn rts_terrain_choke_tiles_for_camp(camp_id: &str) -> Vec<String> {
    if camp_id == "forest_creep_camp" {
        rts_string_vec(["7,4", "7,3", "8,4"])
    } else {
        rts_string_vec(["6,5"])
    }
}

pub fn rts_expansion_tiles_for_camp(camp_id: &str) -> Vec<String> {
    if camp_id == "forest_creep_camp" {
        rts_string_vec(["9,2", "10,2", "10,3"])
    } else {
        rts_string_vec(["8,3"])
    }
}

pub fn rts_siege_units_for_id(unit_id: &str) -> Vec<String> {
    if unit_id == "stonebreak_cart" {
        rts_string_vec(["stonebreak_cart"])
    } else {
        vec![unit_id.to_string()]
    }
}

pub fn rts_siege_push_route_tiles_for_target(target_id: &str, tile_id: &str) -> Vec<String> {
    if target_id == "stonebreak_cart" || tile_id == "10,3" {
        rts_string_vec(["9,2", "9,3", "10,3", "10,2", "11,2", "10,3"])
    } else {
        let tile = rts_parse_tile_id(tile_id).unwrap_or((10, 3));
        vec![
            "9,2".to_string(),
            format!("{},{}", tile.0.saturating_sub(1), tile.1),
            format!("{},{}", tile.0, tile.1),
        ]
    }
}

pub fn rts_siege_breach_tiles_for_target(target_id: &str, tile_id: &str) -> Vec<String> {
    if target_id == "gate_bulwark" {
        rts_string_vec(["9,3", "10,3", "10,2", "11,2", "10,3"])
    } else {
        let tile = rts_parse_tile_id(tile_id).unwrap_or((10, 3));
        vec![
            format!("{},{}", tile.0.saturating_sub(1), tile.1),
            format!("{},{}", tile.0, tile.1),
            format!("{},{}", tile.0 + 1, tile.1),
        ]
    }
}

pub fn rts_inner_lane_tiles_for_id(lane_id: &str, tile_id: &str) -> Vec<String> {
    if lane_id == "inner_lane" {
        rts_string_vec(["10,3", "11,2", "11,3", "12,3", "12,4"])
    } else {
        let tile = rts_parse_tile_id(tile_id).unwrap_or((11, 2));
        vec![
            format!("{},{}", tile.0.saturating_sub(1), tile.1),
            format!("{},{}", tile.0, tile.1),
            format!("{},{}", tile.0 + 1, tile.1),
        ]
    }
}

pub fn rts_central_keep_route_tiles_for_id(target_id: &str, tile_id: &str) -> Vec<String> {
    if target_id == "central_keep" {
        rts_string_vec(["12,3", "12,4", "13,4", "13,3", "14,3"])
    } else {
        let tile = rts_parse_tile_id(tile_id).unwrap_or((13, 3));
        vec![
            format!("{},{}", tile.0.saturating_sub(1), tile.1),
            format!("{},{}", tile.0, tile.1),
            format!("{},{}", tile.0 + 1, tile.1),
        ]
    }
}

pub fn rts_central_keep_tile_for_id(target_id: &str) -> (i32, i32) {
    match target_id {
        "central_keep" => (13, 3),
        "mirror_ward" => (13, 3),
        _ => (13, 3),
    }
}

pub fn rts_boss_guard_units_for_id(guard_id: &str) -> Vec<String> {
    if guard_id == "warden_line" {
        rts_string_vec(["keep_warden_alpha", "keep_warden_beta", "ward_sentinel"])
    } else {
        vec![format!("{guard_id}_warden")]
    }
}

pub fn rts_player_siege_line_tiles_for_id(line_id: &str, tile_id: &str) -> Vec<String> {
    if line_id == "final_line" {
        rts_string_vec(["11,4", "12,4", "13,4", "12,3"])
    } else {
        let tile = rts_parse_tile_id(tile_id).unwrap_or((12, 4));
        vec![
            format!("{},{}", tile.0.saturating_sub(1), tile.1),
            format!("{},{}", tile.0, tile.1),
            format!("{},{}", tile.0 + 1, tile.1),
        ]
    }
}

pub fn rts_keep_breach_tiles_for_id(target_id: &str, tile_id: &str) -> Vec<String> {
    if target_id == "central_keep" {
        rts_string_vec(["13,3", "13,4", "14,3", "14,4"])
    } else {
        let tile = rts_parse_tile_id(tile_id).unwrap_or((13, 3));
        vec![
            format!("{},{}", tile.0, tile.1),
            format!("{},{}", tile.0 + 1, tile.1),
            format!("{},{}", tile.0, tile.1 + 1),
        ]
    }
}

pub fn rts_guardian_counter_units_for_id(counter_id: &str) -> Vec<String> {
    if counter_id == "high_warden" {
        rts_string_vec(["high_warden", "ward_lancer", "last_mirror_guard"])
    } else {
        vec![format!("{counter_id}_counter_guard")]
    }
}

pub fn rts_keep_claim_tiles_for_id(target_id: &str, tile_id: &str) -> Vec<String> {
    if target_id == "central_keep" {
        rts_string_vec(["12,3", "13,3", "14,3", "13,4"])
    } else {
        let tile = rts_parse_tile_id(tile_id).unwrap_or((13, 3));
        vec![
            format!("{},{}", tile.0.saturating_sub(1), tile.1),
            format!("{},{}", tile.0, tile.1),
            format!("{},{}", tile.0 + 1, tile.1),
        ]
    }
}

pub fn rts_command_queue_path_preview_stage(
    combat_events: &[String],
    command_queue: &[String],
    combat_turn: u8,
) -> Option<&'static str> {
    for event in combat_events.iter().rev().chain(command_queue.iter().rev()) {
        if event.contains("command_queue_path_preview:cancel_repath") {
            return Some("cancel_repath");
        }
        if event.contains("command_queue_path_preview:build_reservation") {
            return Some("build_reservation");
        }
        if event.contains("command_queue_path_preview:attack_focus") {
            return Some("attack_focus");
        }
        if event.contains("command_queue_path_preview:rally_chain") {
            return Some("rally_chain");
        }
        if event.contains("command_queue_path_preview:shift_waypoints") {
            return Some("shift_waypoints");
        }
        if event.contains("command_queue_path_preview:queue_stack") {
            return Some("queue_stack");
        }
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("command_queue_path_preview:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "queue_stack",
        1 => "shift_waypoints",
        2 => "rally_chain",
        3 => "attack_focus",
        4 => "build_reservation",
        _ => "cancel_repath",
    })
}

pub fn rts_runtime_point_in_rect(mouse_x: i32, mouse_y: i32, rect: RtsRuntimeRect) -> bool {
    mouse_x >= rect.x
        && mouse_x < rect.x + rect.width
        && mouse_y >= rect.y
        && mouse_y < rect.y + rect.height
}

pub fn rts_runtime_grid_slot_rect(
    spec: RtsRuntimeGridSpec,
    index: usize,
) -> Option<RtsRuntimeRect> {
    if spec.count == 0 || spec.columns == 0 || index >= spec.count {
        return None;
    }
    Some(RtsRuntimeRect {
        x: spec.origin_x + (index % spec.columns) as i32 * spec.stride_x,
        y: spec.origin_y + (index / spec.columns) as i32 * spec.stride_y,
        width: spec.slot_width,
        height: spec.slot_height,
    })
}

pub fn rts_runtime_hit_test_grid(
    spec: RtsRuntimeGridSpec,
    mouse_x: i32,
    mouse_y: i32,
) -> Option<usize> {
    (0..spec.count).find(|index| {
        rts_runtime_grid_slot_rect(spec, *index)
            .is_some_and(|rect| rts_runtime_point_in_rect(mouse_x, mouse_y, rect))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_adapter_clamps_focus_and_projects_minimap() {
        let config = rts_scrollable_map_camera_config();
        let start = RtsScrollableMapCameraState::default();
        let step = apply_rts_scrollable_map_camera_input(
            "shift_keyboard_pan",
            start,
            config,
            RtsRuntimeVec2::new(200.0, -120.0),
            0.35,
            None,
        );

        assert_eq!(step.source, "shift_keyboard_pan");
        assert!(step.after.zoom > start.zoom);
        let focus = rts_scrollable_map_camera_focus_tile(step.after);
        assert!(focus.0 >= TRNM_RTS_RUNTIME_MAP_MIN_TILE);
        assert!(focus.1 >= TRNM_RTS_RUNTIME_MAP_MIN_TILE);

        let viewport = rts_camera_minimap_viewport_rect(step.after, 150, 106);
        assert!(viewport.width >= 18);
        assert!(viewport.height >= 14);
        assert!(viewport.x >= 0);
        assert!(viewport.y >= 0);
    }

    #[test]
    fn minimap_grid_and_hit_tests_are_deterministic() {
        assert_eq!(rts_minimap_cell_origin(10, 20, 4, 5, (1, 1)), (10, 20));
        assert_eq!(rts_minimap_cell_origin(10, 20, 4, 5, (32, 32)), (134, 175));
        assert_eq!(rts_large_map_cell_col((32, 10)), 31);
        assert_eq!(rts_large_map_cell_row((8, 32)), 31);

        let spec = RtsRuntimeGridSpec {
            origin_x: 360,
            origin_y: 572,
            columns: 6,
            count: 12,
            stride_x: 58,
            stride_y: 46,
            slot_width: 48,
            slot_height: 38,
        };
        assert_eq!(rts_runtime_hit_test_grid(spec, 363, 575), Some(0));
        assert_eq!(
            rts_runtime_hit_test_grid(spec, 360 + 58 * 5 + 8, 575),
            Some(5)
        );
        assert_eq!(rts_runtime_hit_test_grid(spec, 999, 575), None);
    }

    #[test]
    fn map_projection_and_terrain_seeds_match_first_contact_layout() {
        let projection = rts_runtime_map_projection(RtsRuntimeMapLayoutInput {
            viewport_width: 1280,
            viewport_height: 720,
            map_width_tiles: 34,
            map_height_tiles: 34,
            map_origin_x: 16,
            map_origin_y: 54,
            right_reserved_px: 292,
            bottom_reserved_px: 158,
            min_map_width_px: 374,
            min_map_height_px: 238,
            cell_width_min: 12,
            cell_width_max: 28,
            cell_height_min: 8,
            cell_height_max: 15,
        });

        assert_eq!(
            projection,
            RtsRuntimeMapProjection {
                map_x: 16,
                map_y: 54,
                cell_w: 28,
                cell_h: 14,
                map_w: 952,
                map_h: 476,
            }
        );
        assert_eq!(
            rts_runtime_tile_screen_rect(projection, (16, 16)),
            RtsRuntimeRect {
                x: 464,
                y: 278,
                width: 28,
                height: 14,
            }
        );
        assert_eq!(
            rts_runtime_terrain_seeds((16, 16)),
            RtsRuntimeTerrainSeeds {
                surface_seed: 12,
                detail_seed: 20,
            }
        );
    }

    #[test]
    fn tile_line_adapter_matches_first_contact_track_steps() {
        let line = rts_runtime_tile_line((8, 8), (12, 16));

        assert_eq!(line.len(), 9);
        assert_eq!(
            line[0],
            RtsRuntimeTileLineStep {
                step_index: 0,
                step_count: 8,
                tile_x: 8,
                tile_y: 8,
            }
        );
        assert_eq!(
            line[4],
            RtsRuntimeTileLineStep {
                step_index: 4,
                step_count: 8,
                tile_x: 10,
                tile_y: 12,
            }
        );
        assert_eq!(
            line[8],
            RtsRuntimeTileLineStep {
                step_index: 8,
                step_count: 8,
                tile_x: 12,
                tile_y: 16,
            }
        );
        assert_eq!(
            rts_runtime_tile_line((5, 5), (5, 5)),
            vec![RtsRuntimeTileLineStep {
                step_index: 0,
                step_count: 0,
                tile_x: 5,
                tile_y: 5,
            }]
        );
    }

    #[test]
    fn path_preview_adapter_preserves_command_semantics() {
        assert_eq!(
            rts_move_follow_target("follow:worker_alpha"),
            Some("worker_alpha")
        );
        assert_eq!(rts_move_formation_kind("follow:worker_alpha"), "follow");
        assert_eq!(
            rts_path_tiles_for_destination((8, 4)),
            vec!["6,5", "7,5", "8,4"]
        );
        assert_eq!(rts_blocked_tiles_for_destination((8, 4)), vec!["7,4"]);
        assert_eq!(
            rts_formation_slots_for_destination((8, 4), "rally"),
            vec!["7,5", "8,4", "9,4", "8,5"]
        );
        assert_eq!(
            rts_disperse_slots_for_destination((6, 5)),
            vec!["5,5", "6,4", "6,6", "7,5"]
        );

        let command_queue = vec!["command_queue_path_preview:shift_waypoints".to_string()];
        assert_eq!(
            rts_command_queue_path_preview_stage(&[], &command_queue, 5),
            Some("shift_waypoints")
        );
        assert_eq!(
            rts_command_queue_path_preview_stage(&[], &["other".to_string()], 0),
            None
        );
    }

    #[test]
    fn combat_target_adapter_preserves_first_contact_semantics() {
        assert_eq!(
            rts_engagement_tiles_for_target("enemy_barracks"),
            vec!["9,3", "10,3", "10,2", "11,2"]
        );
        assert_eq!(
            rts_contact_flash_tiles_for_target("arena_creep_attack"),
            vec!["6,5", "6,4"]
        );
        assert_eq!(rts_target_tile_for_id("forest_shaman_support", 0), (9, 3));
        assert_eq!(
            rts_target_priority_ids_for_target("arena_creep_attack"),
            vec![
                "arena_creep_attack",
                "arena_guard_support",
                "arena_worker_support"
            ]
        );
        assert_eq!(
            rts_projectile_trail_tiles_for_target("forest_creep_camp"),
            vec!["5,5", "6,5", "7,4", "8,3"]
        );
        assert_eq!(
            rts_ability_effect_tiles_for_target("enemy_barracks", "guard_break"),
            vec!["10,3", "10,2", "11,2", "9,3"]
        );
        assert_eq!(
            rts_threat_levels_for_target("enemy_barracks"),
            vec![88, 66, 41]
        );
        assert_eq!(
            rts_damage_ticks_for_ability("guard_break"),
            vec![16, 21, 35]
        );
        assert_eq!(
            rts_projectile_id_for_ability("guard_break"),
            "guard_break_bolt"
        );
    }

    #[test]
    fn ai_pressure_adapter_preserves_first_contact_routes() {
        assert_eq!(
            rts_ai_wave_unit_ids_for_pressure("skirmish_wave"),
            vec!["lane_scout", "mirror_raider", "siege_runner"]
        );
        assert_eq!(
            rts_ai_pressure_tiles_for_pressure("skirmish_wave"),
            vec!["9,3", "8,4", "7,4", "6,5"]
        );
        assert_eq!(
            rts_ai_counter_tiles_for_pressure("skirmish_wave"),
            vec!["5,5", "6,5", "6,4", "7,5"]
        );
        assert_eq!(
            rts_enemy_pressure_wave_units_for_id("raider_wave"),
            vec!["enemy_raider", "enemy_signal_guard", "enemy_sapper"]
        );
        assert_eq!(
            rts_enemy_pressure_lane_tiles_for_wave("raider_wave"),
            vec!["10,2", "9,3", "8,4", "7,4", "6,5"]
        );
    }

    #[test]
    fn recon_intel_adapter_preserves_first_contact_routes() {
        assert_eq!(
            rts_scout_route_tiles_for_recon("enemy_base"),
            vec!["5,5", "6,4", "7,4", "8,3", "9,2", "10,2"]
        );
        assert_eq!(
            rts_fog_reveal_tiles_for_recon("enemy_base", "mark"),
            vec!["7,4", "8,3", "8,2", "9,2", "9,3", "10,2", "10,3", "11,1", "11,2"]
        );
        assert_eq!(
            rts_enemy_structures_for_recon("enemy_base", "mark"),
            vec!["enemy_watch_post", "enemy_barracks", "enemy_resource_vault"]
        );
        assert_eq!(
            rts_enemy_units_for_recon("enemy_base", "mark"),
            vec!["enemy_scout", "enemy_worker", "enemy_guard"]
        );
        assert_eq!(
            rts_enemy_structure_tile_for_id("enemy_resource_vault", 2),
            (11, 2)
        );
        assert_eq!(rts_enemy_unit_tile_for_id("enemy_guard", 2), (11, 2));
    }

    #[test]
    fn base_assault_and_aftermath_adapter_preserves_first_contact_routes() {
        assert_eq!(
            rts_base_assault_path_tiles_for_target("enemy_barracks", "10,3"),
            vec!["5,5", "6,5", "7,4", "8,4", "9,3", "10,3"]
        );
        assert_eq!(
            rts_base_assault_targets_for_id("enemy_barracks"),
            vec!["enemy_watch_post", "enemy_barracks", "enemy_resource_vault"]
        );
        assert_eq!(
            rts_aftermath_debris_tiles_for_id("enemy_barracks", "10,3"),
            vec!["9,3", "10,3", "10,4", "11,3"]
        );
        assert_eq!(
            rts_aftermath_smoke_tiles_for_id("enemy_barracks", "10,3"),
            vec!["10,2", "10,3", "11,3"]
        );
    }

    #[test]
    fn commander_and_expansion_counterattack_adapter_preserves_first_contact_routes() {
        assert_eq!(
            rts_commander_aura_tiles_for_id("mirror_captain"),
            vec!["6,5", "7,4", "8,4", "9,3", "10,3"]
        );
        assert_eq!(
            rts_loot_items_for_id("enemy_barracks"),
            vec![
                "barracks_map_cache",
                "field_banner_relic",
                "repair_kit_crate"
            ]
        );
        assert_eq!(
            rts_expansion_tiles_for_id("forest_relay", "9,2"),
            vec!["8,2", "9,2", "10,2", "9,3", "10,3"]
        );
        assert_eq!(rts_expansion_structure_tile_for_id("watch_lantern"), (8, 3));
        assert_eq!(
            rts_expansion_workers_for_line("gold_line"),
            vec![
                "expansion_worker_alpha",
                "expansion_worker_beta",
                "expansion_worker_gamma"
            ]
        );
        assert_eq!(
            rts_counterattack_units_for_wave("counter_wave"),
            vec![
                "counter_raider_alpha",
                "counter_raider_beta",
                "counter_sapper"
            ]
        );
        assert_eq!(
            rts_counterattack_route_tiles_for_wave("counter_wave", "8,3"),
            vec!["11,2", "10,2", "9,3", "8,3", "7,4", "9,2"]
        );
    }

    #[test]
    fn objective_and_terrain_route_adapter_preserves_first_contact_tiles() {
        assert_eq!(
            rts_objective_tiles_for_id("relay_beacon", "6,5"),
            vec!["6,5", "6,4", "7,5", "9,2"]
        );
        assert_eq!(
            rts_creep_camp_tiles_for_id("forest_creep_camp", "8,3"),
            vec!["8,3", "8,2", "9,3", "9,2"]
        );
        assert_eq!(
            rts_terrain_route_tiles_for_camp("forest_creep_camp"),
            vec!["5,5", "6,5", "7,4", "8,3"]
        );
        assert_eq!(
            rts_terrain_choke_tiles_for_camp("forest_creep_camp"),
            vec!["7,4", "7,3", "8,4"]
        );
        assert_eq!(
            rts_expansion_tiles_for_camp("forest_creep_camp"),
            vec!["9,2", "10,2", "10,3"]
        );
    }

    #[test]
    fn siege_and_inner_lane_adapter_preserves_first_contact_routes() {
        assert_eq!(
            rts_siege_units_for_id("stonebreak_cart"),
            vec!["stonebreak_cart"]
        );
        assert_eq!(
            rts_siege_push_route_tiles_for_target("gate_bulwark", "10,3"),
            vec!["9,2", "9,3", "10,3", "10,2", "11,2", "10,3"]
        );
        assert_eq!(
            rts_siege_breach_tiles_for_target("gate_bulwark", "10,3"),
            vec!["9,3", "10,3", "10,2", "11,2", "10,3"]
        );
        assert_eq!(
            rts_inner_lane_tiles_for_id("inner_lane", "11,2"),
            vec!["10,3", "11,2", "11,3", "12,3", "12,4"]
        );
    }

    #[test]
    fn central_keep_adapter_preserves_first_contact_routes() {
        assert_eq!(
            rts_central_keep_route_tiles_for_id("central_keep", "13,3"),
            vec!["12,3", "12,4", "13,4", "13,3", "14,3"]
        );
        assert_eq!(rts_central_keep_tile_for_id("central_keep"), (13, 3));
        assert_eq!(
            rts_boss_guard_units_for_id("warden_line"),
            vec!["keep_warden_alpha", "keep_warden_beta", "ward_sentinel"]
        );
        assert_eq!(
            rts_player_siege_line_tiles_for_id("final_line", "12,4"),
            vec!["11,4", "12,4", "13,4", "12,3"]
        );
        assert_eq!(
            rts_keep_breach_tiles_for_id("central_keep", "13,3"),
            vec!["13,3", "13,4", "14,3", "14,4"]
        );
        assert_eq!(
            rts_guardian_counter_units_for_id("high_warden"),
            vec!["high_warden", "ward_lancer", "last_mirror_guard"]
        );
        assert_eq!(
            rts_keep_claim_tiles_for_id("central_keep", "13,3"),
            vec!["12,3", "13,3", "14,3", "13,4"]
        );
    }
}
