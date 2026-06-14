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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsUnitModelDepthMark {
    pub kind: String,
    pub rect: RtsRuntimeRect,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsActionCadenceMark {
    pub kind: String,
    pub rect: RtsRuntimeRect,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsActionSequenceMark {
    pub kind: String,
    pub rect: RtsRuntimeRect,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsCommandStamp {
    pub input_source: String,
    pub kind: String,
    pub tile_id: Option<String>,
    pub target_id: Option<String>,
    pub player_label: String,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsControlGroupSlotSummary {
    pub slot: String,
    pub key_label: String,
    pub member_count: usize,
    pub occupied: bool,
    pub active: bool,
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

pub fn rts_catalog_text_label(text: &str, max_chars: usize) -> String {
    text.replace('_', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_uppercase()
        .chars()
        .take(max_chars)
        .collect()
}

fn rts_string_vec<const N: usize>(values: [&str; N]) -> Vec<String> {
    values.into_iter().map(str::to_string).collect()
}

fn rts_push_unique_string(values: &mut Vec<String>, value: &str) {
    if !values.iter().any(|existing| existing == value) {
        values.push(value.to_string());
    }
}

pub fn rts_line_path_tiles(start: (i32, i32), end: (i32, i32)) -> Vec<String> {
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

pub fn rts_default_group_units() -> Vec<String> {
    rts_string_vec([
        "player",
        "square_guard_patrol",
        "square_worker_carry",
        "square_creep_wander",
    ])
}

pub fn rts_group_two_units() -> Vec<String> {
    rts_string_vec(["square_guard_patrol", "square_creep_wander"])
}

pub fn rts_unit_selection_class(unit_id: &str) -> &'static str {
    if unit_id.contains("guard") || unit_id == "player" {
        "guard"
    } else if unit_id.contains("worker") {
        "worker"
    } else if unit_id.contains("creep") {
        "creep"
    } else {
        "unit"
    }
}

pub fn rts_same_class_units(unit_id: &str) -> Vec<String> {
    match rts_unit_selection_class(unit_id) {
        "guard" => rts_string_vec(["player", "square_guard_front", "square_guard_patrol"]),
        "worker" => rts_string_vec(["square_worker_carry", "square_worker_harvest"]),
        "creep" => rts_string_vec(["square_creep_wander"]),
        _ => vec![unit_id.to_string()],
    }
}

fn rts_selectable_unit_entries() -> [(&'static str, (i32, i32), &'static str, u8); 6] {
    [
        ("player", (5, 4), "player", 0),
        ("square_guard_front", (5, 4), "player", 1),
        ("square_guard_patrol", (7, 5), "player", 2),
        ("square_worker_carry", (4, 5), "player", 3),
        ("square_worker_harvest", (8, 5), "player", 4),
        ("square_creep_wander", (9, 4), "hostile", 20),
    ]
}

pub fn rts_unit_allegiance(unit_id: &str) -> &'static str {
    rts_selectable_unit_entries()
        .into_iter()
        .find_map(|(entry_unit_id, _, allegiance, _)| {
            (entry_unit_id == unit_id).then_some(allegiance)
        })
        .unwrap_or("unknown")
}

pub fn rts_unit_is_player_owned(unit_id: &str) -> bool {
    rts_unit_allegiance(unit_id) == "player"
}

pub fn rts_unit_selection_priority(unit_id: &str) -> u8 {
    rts_selectable_unit_entries()
        .into_iter()
        .find_map(|(entry_unit_id, _, _, priority)| (entry_unit_id == unit_id).then_some(priority))
        .unwrap_or(u8::MAX)
}

pub fn rts_selectable_unit_tile(unit_id: &str) -> Option<(i32, i32)> {
    rts_selectable_unit_entries()
        .into_iter()
        .find_map(|(entry_unit_id, tile, _, _)| (entry_unit_id == unit_id).then_some(tile))
}

pub fn rts_selectable_unit_at_tile(tile: (i32, i32)) -> Option<&'static str> {
    rts_selectable_unit_entries()
        .into_iter()
        .filter(|(_, unit_tile, _, _)| *unit_tile == tile)
        .min_by_key(|(unit_id, _, allegiance, priority)| {
            let allegiance_priority = if *allegiance == "player" { 0 } else { 1 };
            (allegiance_priority, *priority, *unit_id)
        })
        .map(|(unit_id, _, _, _)| unit_id)
}

pub fn rts_selection_clear_parts(group_id: &str) -> Option<(String, Option<String>, String)> {
    let payload = group_id.strip_prefix("clear:")?;
    if let Some(tile_id) = payload.strip_prefix("empty@") {
        return Some(("empty".to_string(), None, tile_id.to_string()));
    }
    if let Some(hostile_payload) = payload.strip_prefix("hostile:") {
        let (unit_id, tile_id) = hostile_payload.split_once('@')?;
        return Some((
            "hostile".to_string(),
            Some(unit_id.to_string()),
            tile_id.to_string(),
        ));
    }
    None
}

pub fn rts_selection_tiles_for_units(unit_ids: &[String]) -> Vec<String> {
    let mut tiles = Vec::new();
    for unit_id in unit_ids {
        if let Some(tile) = rts_selectable_unit_tile(unit_id) {
            rts_push_unique_string(&mut tiles, &rts_runtime_tile_id(tile));
        }
    }
    tiles
}

pub fn rts_selection_box_tiles() -> Vec<String> {
    rts_string_vec(["5,5", "6,5", "5,4", "6,4"])
}

pub fn rts_control_group_hotkey_slot(group_id: &str, prefix: &str) -> Option<String> {
    group_id
        .strip_prefix(prefix)
        .map(str::trim)
        .filter(|slot| !slot.is_empty())
        .map(ToOwned::to_owned)
}

pub fn rts_default_units_for_control_group_slot(slot: &str) -> Vec<String> {
    match slot {
        "2" => rts_group_two_units(),
        "3" => rts_string_vec(["square_worker_carry", "square_worker_harvest"]),
        _ => rts_default_group_units(),
    }
}

pub fn rts_units_from_control_group_assignment(assignments: &[String], slot: &str) -> Vec<String> {
    let prefix = format!("{slot}:");
    for assignment in assignments.iter().rev() {
        let Some(payload) = assignment.strip_prefix(&prefix) else {
            continue;
        };
        let unit_payload = payload.rsplit(':').next().unwrap_or(payload);
        let units = unit_payload
            .split('|')
            .map(str::trim)
            .filter(|unit| !unit.is_empty())
            .filter(|unit| rts_selectable_unit_tile(unit).is_some())
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        if !units.is_empty() {
            return units;
        }
    }
    Vec::new()
}

pub fn rts_control_group_slot_label(slot: &str) -> &str {
    if slot == "10" {
        "0"
    } else {
        slot
    }
}

pub fn rts_control_group_slot_member_count(assignments: &[String], slot: &str) -> usize {
    rts_units_from_control_group_assignment(assignments, slot).len()
}

pub fn rts_control_group_slot_is_active(
    active_group_ids: &[String],
    current_group_id: Option<&str>,
    slot: &str,
) -> bool {
    active_group_ids.iter().any(|group| group == slot) || current_group_id == Some(slot)
}

pub fn rts_control_group_slot_summaries(
    assignments: &[String],
    active_group_ids: &[String],
    current_group_id: Option<&str>,
) -> Vec<RtsControlGroupSlotSummary> {
    (1..=10)
        .map(|slot_index| {
            let slot = slot_index.to_string();
            let member_count = rts_control_group_slot_member_count(assignments, &slot);
            let active =
                rts_control_group_slot_is_active(active_group_ids, current_group_id, &slot);
            RtsControlGroupSlotSummary {
                key_label: rts_control_group_slot_label(&slot).to_string(),
                slot,
                member_count,
                occupied: member_count > 0,
                active,
            }
        })
        .collect()
}

pub fn rts_merged_unit_ids(base_units: &[String], extra_units: &[String]) -> Vec<String> {
    let mut merged = base_units.to_vec();
    for unit_id in extra_units {
        rts_push_unique_string(&mut merged, unit_id);
    }
    merged
}

pub fn rts_drag_selection_parts(group_id: &str) -> Option<((i32, i32), (i32, i32))> {
    let payload = group_id.strip_prefix("drag:")?;
    let (start, end) = payload.split_once("->")?;
    Some((rts_parse_tile_id(start)?, rts_parse_tile_id(end)?))
}

pub fn rts_selection_box_tiles_between(start: (i32, i32), end: (i32, i32)) -> Vec<String> {
    let start = rts_large_map_clamp_tile(start);
    let end = rts_large_map_clamp_tile(end);
    let min_x = start.0.min(end.0);
    let max_x = start.0.max(end.0);
    let min_y = start.1.min(end.1);
    let max_y = start.1.max(end.1);
    let mut tiles = Vec::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            tiles.push(rts_runtime_tile_id((x, y)));
        }
    }
    tiles
}

pub fn rts_drag_selected_units(start: (i32, i32), end: (i32, i32)) -> Vec<String> {
    rts_drag_units_between(start, end, true)
}

pub fn rts_drag_rejected_unit_ids(start: (i32, i32), end: (i32, i32)) -> Vec<String> {
    rts_drag_units_between(start, end, false)
        .into_iter()
        .filter(|unit_id| !rts_unit_is_player_owned(unit_id))
        .collect()
}

fn rts_drag_units_between(start: (i32, i32), end: (i32, i32), owned_only: bool) -> Vec<String> {
    let start = rts_large_map_clamp_tile(start);
    let end = rts_large_map_clamp_tile(end);
    let min_x = start.0.min(end.0);
    let max_x = start.0.max(end.0);
    let min_y = start.1.min(end.1);
    let max_y = start.1.max(end.1);
    let mut selected = Vec::new();
    for (unit_id, tile, _, _) in rts_selectable_unit_entries() {
        if tile.0 >= min_x && tile.0 <= max_x && tile.1 >= min_y && tile.1 <= max_y {
            if !owned_only || rts_unit_is_player_owned(unit_id) {
                rts_push_unique_string(&mut selected, unit_id);
            }
        }
    }
    selected
}

pub fn rts_move_follow_target(formation: &str) -> Option<&str> {
    formation
        .strip_prefix("follow:")
        .map(str::trim)
        .filter(|target_id| !target_id.is_empty())
}

pub fn rts_move_command_parts(command_id: &str) -> (&str, &str) {
    let command_payload = command_id.strip_prefix("minimap:").unwrap_or(command_id);
    let mut parts = command_payload.splitn(2, ':');
    let tile_id = parts
        .next()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("7,4");
    let formation = parts
        .next()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("diamond");
    (tile_id, formation)
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

pub fn rts_focus_fire_units_for_target(target_id: &str) -> Vec<String> {
    if target_id == "enemy_barracks" {
        rts_army_units_for_batch("mixed_vanguard")
    } else if target_id == "forest_creep_camp"
        || target_id == "arena_creep_attack"
        || target_id == "square_creep_wander"
    {
        rts_default_group_units()
    } else {
        rts_string_vec(["player", "square_guard_patrol"])
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

pub fn rts_army_units_for_batch(batch_id: &str) -> Vec<String> {
    match batch_id {
        "guard_pair" => rts_string_vec(["relay_guard_alpha", "relay_guard_beta"]),
        "wayfinder_pair" => rts_string_vec(["wayfinder_scout", "wayfinder_signal"]),
        "mixed_vanguard" => rts_string_vec([
            "relay_guard_alpha",
            "relay_guard_beta",
            "wayfinder_scout",
            "field_mender",
        ]),
        _ => vec![batch_id.to_string()],
    }
}

pub fn rts_army_rally_tiles_for_id(rally_id: &str) -> Vec<String> {
    match rally_id {
        "forward_watch" => rts_string_vec(["5,5", "6,5", "7,4", "8,4", "8,3"]),
        "forest_relay" => rts_string_vec(["5,5", "6,4", "7,4", "8,3", "9,2"]),
        _ => rts_string_vec(["5,5", "6,5", "7,4"]),
    }
}

pub fn rts_player_army_unit_tile_for_id(unit_id: &str, index: usize) -> (i32, i32) {
    match unit_id {
        "relay_guard_alpha" => (6, 5),
        "relay_guard_beta" => (7, 5),
        "wayfinder_scout" => (7, 4),
        "wayfinder_signal" => (8, 4),
        "field_mender" => (6, 4),
        _ => (6 + (index as i32 % 3), 5 - (index as i32 / 3)),
    }
}

pub fn rts_objective_parts(command: &str) -> (String, String, String) {
    let (kind, payload) = command.split_once(':').unwrap_or(("claim", command));
    let (objective_id, tile_id) = payload.split_once('@').unwrap_or((payload, "6,5"));
    (
        kind.to_string(),
        objective_id.to_string(),
        tile_id.to_string(),
    )
}

pub fn rts_creep_camp_parts(kind_hint: &str, command: &str) -> (String, String, String) {
    let (kind, payload) = if kind_hint == "camp" {
        command.split_once(':').unwrap_or(("clear", command))
    } else {
        (kind_hint, command)
    };
    let (camp_id, tile_id) = payload.split_once('@').unwrap_or((payload, "8,3"));
    let normalized_camp_id = if camp_id == "creep_camp" {
        "forest_creep_camp"
    } else {
        camp_id
    };
    (
        kind.to_string(),
        normalized_camp_id.to_string(),
        tile_id.to_string(),
    )
}

pub fn rts_recon_parts(command: &str) -> (String, String, String) {
    let (kind, payload) = command.split_once(':').unwrap_or(("scout", command));
    let (recon_id, tile_id) = payload.split_once('@').unwrap_or((payload, "10,2"));
    let normalized_recon_id = match recon_id {
        "scout_enemy_base" => "enemy_base",
        value => value,
    };
    (
        kind.to_string(),
        normalized_recon_id.to_string(),
        tile_id.to_string(),
    )
}

pub fn rts_enemy_command_parts(
    command: &str,
    fallback_kind: &str,
    fallback_source: &str,
) -> (String, String, String) {
    let (kind, payload) = command.split_once(':').unwrap_or((fallback_kind, command));
    let (id, source_id) = payload
        .split_once('@')
        .unwrap_or((payload, fallback_source));
    (kind.to_string(), id.to_string(), source_id.to_string())
}

pub fn rts_counter_command_parts(command: &str) -> (String, String, String) {
    let (kind, payload) = command.split_once(':').unwrap_or(("research", command));
    let (id, source_id) = payload.split_once('@').unwrap_or((payload, "signal_spire"));
    (kind.to_string(), id.to_string(), source_id.to_string())
}

pub fn rts_army_command_parts(command: &str) -> (String, String, String) {
    let (kind, payload) = command.split_once(':').unwrap_or(("train", command));
    let (id, source_id) = payload
        .split_once('@')
        .unwrap_or((payload, "training_hall"));
    (kind.to_string(), id.to_string(), source_id.to_string())
}

pub fn rts_base_assault_parts(command: &str) -> (String, String, String) {
    let (kind, payload) = command.split_once(':').unwrap_or(("breach", command));
    let (target_id, tile_id) = payload.split_once('@').unwrap_or((payload, "10,3"));
    (kind.to_string(), target_id.to_string(), tile_id.to_string())
}

pub fn rts_aftermath_parts(command: &str) -> (String, String, String) {
    let (kind, payload) = command.split_once(':').unwrap_or(("destroy", command));
    let (id, tile_id) = payload.split_once('@').unwrap_or((payload, "10,3"));
    (kind.to_string(), id.to_string(), tile_id.to_string())
}

pub fn rts_commander_parts(command: &str) -> (String, String, String) {
    let (kind, payload) = command.split_once(':').unwrap_or(("level", command));
    let (id, source_id) = payload
        .split_once('@')
        .unwrap_or((payload, "mirror_captain"));
    (kind.to_string(), id.to_string(), source_id.to_string())
}

pub fn rts_expansion_parts(command: &str) -> (String, String, String) {
    let (kind, payload) = command.split_once(':').unwrap_or(("claim", command));
    let (id, source_id) = payload.split_once('@').unwrap_or((payload, "9,2"));
    (kind.to_string(), id.to_string(), source_id.to_string())
}

pub fn rts_tier_two_parts(command: &str) -> (String, String, String) {
    let (kind, payload) = command.split_once(':').unwrap_or(("tech", command));
    let (id, source_id) = payload
        .split_once('@')
        .unwrap_or((payload, "relay_outpost"));
    (kind.to_string(), id.to_string(), source_id.to_string())
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

pub fn rts_creep_camp_units_for_id(camp_id: &str) -> Vec<String> {
    if camp_id == "forest_creep_camp" {
        rts_string_vec(["forest_alpha_creep", "forest_stalker", "forest_shaman"])
    } else {
        rts_string_vec(["camp_scout"])
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

pub fn rts_enemy_fortification_tile_for_id(fortification_id: &str) -> (i32, i32) {
    match fortification_id {
        "gate_bulwark" => (10, 3),
        "watch_redoubt" => (10, 2),
        _ => (10, 3),
    }
}

pub fn rts_enemy_repair_units_for_target(target_id: &str) -> Vec<String> {
    if target_id == "gate_bulwark" {
        rts_string_vec(["repair_adept_alpha", "repair_adept_beta"])
    } else {
        vec![format!("{target_id}_repair_adept")]
    }
}

pub fn rts_enemy_flank_units_for_id(flank_id: &str) -> Vec<String> {
    if flank_id == "ridge_sentries" {
        rts_string_vec(["ridge_sentry_left", "ridge_sentry_right", "ridge_sapper"])
    } else {
        vec![format!("{flank_id}_flanker")]
    }
}

pub fn rts_enemy_flank_tile_for_index(index: usize) -> (i32, i32) {
    match index % 3 {
        0 => (9, 4),
        1 => (10, 4),
        _ => (8, 4),
    }
}

pub fn rts_player_hold_tiles_for_id(hold_id: &str, tile_id: &str) -> Vec<String> {
    if hold_id == "shield_line" {
        rts_string_vec(["8,3", "9,3", "9,4", "10,3"])
    } else {
        let tile = rts_parse_tile_id(tile_id).unwrap_or((9, 3));
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

pub fn rts_inner_gate_tile_for_id(gate_id: &str) -> (i32, i32) {
    match gate_id {
        "inner_latch" => (11, 3),
        "signal_lock" => (12, 3),
        _ => (11, 3),
    }
}

pub fn rts_inner_defenders_for_id(defender_id: &str) -> Vec<String> {
    if defender_id == "second_line" {
        rts_string_vec(["inner_guard_alpha", "inner_guard_beta", "signal_lancer"])
    } else {
        vec![format!("{defender_id}_guard")]
    }
}

pub fn rts_supply_convoy_for_id(convoy_id: &str) -> Vec<String> {
    if convoy_id == "relay_convoy" {
        rts_string_vec(["convoy_cart", "field_medic", "ammo_runner"])
    } else {
        vec![format!("{convoy_id}_cart")]
    }
}

pub fn rts_split_squad_tiles_for_id(split_id: &str, tile_id: &str) -> Vec<String> {
    if split_id == "flank_team" {
        rts_string_vec(["10,4", "11,4", "12,4", "12,3"])
    } else {
        let tile = rts_parse_tile_id(tile_id).unwrap_or((10, 4));
        vec![
            format!("{},{}", tile.0, tile.1),
            format!("{},{}", tile.0 + 1, tile.1),
            format!("{},{}", tile.0 + 2, tile.1),
        ]
    }
}

pub fn rts_inner_core_tile_for_id(core_id: &str) -> (i32, i32) {
    match core_id {
        "signal_core" => (12, 3),
        "relay_core" => (12, 4),
        _ => (12, 3),
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

pub fn rts_restored_zones_for_id(zone_id: &str) -> Vec<String> {
    if zone_id == "mirror_city" {
        rts_string_vec(["central_keep", "signal_core", "inner_lane", "forest_relay"])
    } else {
        vec![zone_id.to_string()]
    }
}

pub fn rts_rebuild_structures_for_id(structure_id: &str) -> Vec<String> {
    if structure_id == "signal_core" {
        rts_string_vec(["signal_core", "inner_latch", "mirror_ward"])
    } else {
        vec![structure_id.to_string()]
    }
}

pub fn rts_garrison_units_for_id(garrison_id: &str) -> Vec<String> {
    if garrison_id == "central_keep" {
        rts_string_vec(["mirror_guard_alpha", "signal_lancer", "field_engineer"])
    } else {
        vec![format!("{garrison_id}_garrison")]
    }
}

pub fn rts_open_world_route_tiles_for_id(route_id: &str) -> Vec<String> {
    match route_id {
        "after_action" | "league-coliseum" => {
            rts_string_vec(["13,3", "12,3", "11,3", "10,2", "9,2"])
        }
        _ => rts_string_vec(["13,3", "12,3", "11,3"]),
    }
}

pub fn rts_open_world_panels_for_room(room_id: &str) -> Vec<String> {
    if room_id == "league-coliseum" {
        rts_string_vec([
            "room_panel:league-coliseum",
            "task_panel:task-fixture-first-route",
            "combat_panel:league-coliseum",
            "save_panel:post_rts_restore",
        ])
    } else {
        vec![format!("room_panel:{room_id}")]
    }
}

pub fn rts_siege_unit_tile_for_id(unit_id: &str, index: usize) -> (i32, i32) {
    match unit_id {
        "stonebreak_cart" => (9, 3),
        _ => (9 + (index as i32 % 2), 3),
    }
}

pub fn rts_harvest_tile_for_node(node_id: &str) -> (i32, i32) {
    match node_id {
        "gold_vein" => (3, 3),
        "lumber_copse" => (8, 3),
        "forest_relay_gold" => (10, 2),
        _ => (4, 4),
    }
}

pub fn rts_dropoff_tile_for_structure(structure_id: &str) -> (i32, i32) {
    match structure_id {
        "town_hall" => (5, 5),
        "lumber_mill" => (7, 5),
        "relay_outpost" => (9, 2),
        _ => (5, 5),
    }
}

pub fn rts_build_site_tiles(tile_id: &str) -> Vec<String> {
    match tile_id {
        "7,4" => rts_string_vec(["7,4", "7,5", "8,4"]),
        "8,4" => rts_string_vec(["8,4", "8,5", "9,4"]),
        _ => vec![tile_id.to_string()],
    }
}

pub fn rts_structure_tile_for_id(structure_id: &str) -> (i32, i32) {
    match structure_id {
        "watch_tower" => (7, 4),
        "scout_tower" => (8, 4),
        "town_hall" => (5, 5),
        "training_hall" => (4, 3),
        "signal_spire" => (6, 3),
        _ => (7, 4),
    }
}

pub fn rts_unlock_unit_tile_for_id(unit_id: &str) -> (i32, i32) {
    match unit_id {
        "relay_guard" => (7, 5),
        "wayfinder" => (4, 5),
        _ => (6, 5),
    }
}

pub fn rts_queue_gold_cost(queue_id: &str) -> u64 {
    let queue_id = queue_id.strip_prefix("queue:").unwrap_or(queue_id);
    let item_id = queue_id
        .split_once('@')
        .map(|(item_id, _)| item_id)
        .unwrap_or(queue_id);
    match item_id {
        "train:worker" => 80,
        "train:guard" => 140,
        "train:scout" => 110,
        "build:watch_tower" => 210,
        "build:training_hall" => 260,
        "build:signal_spire" => 320,
        "build:power_node" => 160,
        "build:refinery" => 240,
        "build:command_post" => 300,
        "build:radar_spire" => 220,
        "build:wall" => 60,
        "build:relay" | "build:scout_tower" => 180,
        "upgrade:signal_blade" | "upgrade:training_hall" => 210,
        "harvest:gold_vein" | "harvest:lumber_copse" => 0,
        _ if item_id.starts_with("complete:") => 0,
        _ if item_id.starts_with("cancel:") => 0,
        _ if item_id.starts_with("repair:") => 45,
        _ => 120,
    }
}

pub fn rts_queue_cost_label(queue_id: &str) -> String {
    let cost = rts_queue_gold_cost(queue_id);
    if cost == 0 {
        "-".to_string()
    } else {
        cost.to_string()
    }
}

pub fn rts_log_gold_amount(entry: &str) -> u64 {
    entry
        .split(':')
        .filter_map(|part| part.trim().strip_suffix('g'))
        .filter_map(|amount| {
            amount
                .trim_start_matches(|value| value == '+' || value == '-')
                .parse::<u64>()
                .ok()
        })
        .sum()
}

pub fn rts_resource_gold_commitment(resource_spend_log: &[String]) -> u64 {
    resource_spend_log
        .iter()
        .map(|entry| rts_log_gold_amount(entry))
        .sum()
}

pub fn rts_available_gold(coins: u64, resource_spend_log: &[String]) -> u64 {
    let gross_gold = 620_u64.saturating_add(coins);
    let commitment = rts_resource_gold_commitment(resource_spend_log);
    gross_gold.saturating_sub(commitment.min(gross_gold.saturating_sub(40)))
}

pub fn rts_queue_is_affordable(coins: u64, resource_spend_log: &[String], queue_id: &str) -> bool {
    rts_queue_gold_cost(queue_id) <= rts_available_gold(coins, resource_spend_log)
}

pub fn rts_queue_requires_affordability_check(queue_id: &str) -> bool {
    let queue_id = queue_id.strip_prefix("queue:").unwrap_or(queue_id).trim();
    queue_id.starts_with("build:")
        || queue_id.starts_with("train:")
        || queue_id.starts_with("upgrade:")
        || queue_id.starts_with("research:")
        || queue_id.starts_with("repair:")
}

pub fn rts_build_parts(queue_id: &str) -> (String, String) {
    let payload = queue_id.strip_prefix("build:").unwrap_or(queue_id);
    if let Some((structure_id, tile_id)) = payload.split_once('@') {
        (structure_id.to_string(), tile_id.to_string())
    } else {
        (payload.to_string(), "7,4".to_string())
    }
}

pub fn rts_structure_parts(
    queue_id: &str,
    prefix: &str,
    fallback_tile_id: &str,
) -> (String, String) {
    let payload = queue_id.strip_prefix(prefix).unwrap_or(queue_id);
    if let Some((structure_id, tile_id)) = payload.split_once('@') {
        (structure_id.to_string(), tile_id.to_string())
    } else {
        (payload.to_string(), fallback_tile_id.to_string())
    }
}

pub fn rts_tech_parts(queue_id: &str, prefix: &str, fallback_source_id: &str) -> (String, String) {
    let payload = queue_id.strip_prefix(prefix).unwrap_or(queue_id);
    if let Some((tech_id, source_id)) = payload.split_once('@') {
        (tech_id.to_string(), source_id.to_string())
    } else {
        (payload.to_string(), fallback_source_id.to_string())
    }
}

pub fn rts_queue_uses_production_lane(queue_id: &str) -> bool {
    let queue_id = queue_id.strip_prefix("queue:").unwrap_or(queue_id);
    !queue_id.starts_with("build:")
        && !queue_id.starts_with("cancel:")
        && !queue_id.starts_with("complete:")
        && !queue_id.starts_with("harvest:")
        && !queue_id.starts_with("repair:")
}

pub fn rts_queue_feedback_chip(queue_id: &str) -> String {
    let queue_id = queue_id.strip_prefix("queue:").unwrap_or(queue_id);
    if let Some(unit_id) = queue_id.strip_prefix("train:") {
        format!("feedback:train_queued:{unit_id}")
    } else if queue_id.starts_with("build:") {
        let (structure_id, tile_id) = rts_build_parts(queue_id);
        format!("feedback:build_placed:{structure_id}@{tile_id}")
    } else if let Some(node_id) = queue_id.strip_prefix("harvest:") {
        format!("feedback:harvest_assigned:{node_id}")
    } else if queue_id.starts_with("upgrade:") {
        let (upgrade_id, source_id) = rts_tech_parts(queue_id, "upgrade:", "training_hall");
        format!("feedback:upgrade_queued:{upgrade_id}@{source_id}")
    } else if queue_id.starts_with("research:") {
        let (tech_id, source_id) = rts_tech_parts(queue_id, "research:", "town_hall");
        format!("feedback:research_queued:{tech_id}@{source_id}")
    } else {
        format!("feedback:queue_accepted:{queue_id}")
    }
}

pub fn rts_rejection_feedback_chip(action_label: &str, reason: &str) -> String {
    let action_kind = action_label
        .strip_prefix("RTS:")
        .and_then(|label| label.split(':').next())
        .filter(|label| !label.trim().is_empty())
        .unwrap_or("action")
        .to_ascii_lowercase();
    format!("feedback:blocked:{action_kind}:{reason}")
}

pub fn rts_input_source_player_label(input_source: &str, action_label: &str) -> &'static str {
    let normalized = input_source.to_ascii_lowercase();
    if normalized.contains("mouse_sidebar") {
        "SIDEBAR"
    } else if normalized.contains("mouse_command_bar") {
        "COMMAND BAR"
    } else if normalized.contains("mouse_minimap") {
        "MINIMAP"
    } else if normalized.contains("mouse_bottom_panel") {
        "BOTTOM PANEL"
    } else if normalized.contains("mouse_viewport") {
        "MAP"
    } else if normalized.contains("mouse_drag") {
        "DRAG"
    } else if normalized.contains("hotkey") {
        "HOTKEY"
    } else if normalized.contains("keyboard") {
        "KEYBOARD"
    } else if action_label.starts_with("RTS:QUEUE") {
        "SIDEBAR"
    } else if action_label.starts_with("RTS:MOVE") || action_label.starts_with("RTS:ATTACK") {
        "MAP"
    } else {
        "COMMAND"
    }
}

pub fn rts_command_stamp_for_selection(
    input_source: &str,
    group_id: &str,
    selected_unit_count: usize,
) -> RtsCommandStamp {
    let source = rts_input_source_player_label(input_source, "RTS:SELECT");
    if let Some((clear_kind, unit_id, tile_id)) = rts_selection_clear_parts(group_id) {
        let select_label = if unit_id.is_some() {
            format!("SELECTION CLEARED {}", clear_kind.to_ascii_uppercase())
        } else {
            "SELECTION CLEARED".to_string()
        };
        return RtsCommandStamp {
            input_source: input_source.to_string(),
            kind: "select-clear".to_string(),
            tile_id: Some(tile_id),
            target_id: unit_id,
            player_label: format!("{source} {select_label}"),
        };
    }
    let selected_count = selected_unit_count.max(1);
    let unit_word = if selected_count == 1 { "UNIT" } else { "UNITS" };
    let (kind, select_label, target_id) =
        if let Some(slot) = rts_control_group_hotkey_slot(group_id, "assign:") {
            (
                "control-group",
                format!("GROUP {slot} ASSIGNED"),
                Some(slot),
            )
        } else if let Some(slot) = rts_control_group_hotkey_slot(group_id, "append:") {
            (
                "control-group",
                format!("GROUP {slot} APPENDED"),
                Some(slot),
            )
        } else if let Some(slot) = rts_control_group_hotkey_slot(group_id, "recall:") {
            (
                "control-group",
                format!("GROUP {slot} RECALLED"),
                Some(slot),
            )
        } else if let Some(slot) = rts_control_group_hotkey_slot(group_id, "recall_add:") {
            ("control-group", format!("GROUP {slot} ADDED"), Some(slot))
        } else if let Some(slot) = rts_control_group_hotkey_slot(group_id, "camera:") {
            (
                "control-group-camera",
                format!("GROUP {slot} CAMERA SNAP"),
                Some(slot),
            )
        } else if group_id.starts_with("shift:unit:") {
            (
                "select",
                "SHIFT SELECT".to_string(),
                Some(group_id.to_string()),
            )
        } else if group_id.starts_with("double:unit:") {
            (
                "select",
                "DOUBLE SELECT".to_string(),
                Some(group_id.to_string()),
            )
        } else {
            ("select", "SELECT".to_string(), Some(group_id.to_string()))
        };
    let player_label = if group_id.starts_with("camera:") {
        format!("{source} {select_label}")
    } else if kind == "select" {
        format!("{source} {select_label} SENT {selected_count} {unit_word}")
    } else {
        format!("{source} {select_label} {selected_count} {unit_word}")
    };
    RtsCommandStamp {
        input_source: input_source.to_string(),
        kind: kind.to_string(),
        tile_id: None,
        target_id,
        player_label,
    }
}

pub fn rts_command_stamp_for_queue(input_source: &str, queue_id: &str) -> RtsCommandStamp {
    let source = rts_input_source_player_label(input_source, "RTS:QUEUE");
    let (kind, target_id, tile_id, item_label) = if queue_id.starts_with("build:") {
        let (structure_id, tile_id) = rts_build_parts(queue_id);
        (
            "build",
            structure_id.clone(),
            Some(tile_id),
            rts_catalog_text_label(&structure_id, 20),
        )
    } else if let Some(unit_id) = queue_id.strip_prefix("train:") {
        (
            "train",
            unit_id.to_string(),
            None,
            rts_catalog_text_label(unit_id, 20),
        )
    } else if let Some(node_id) = queue_id.strip_prefix("harvest:") {
        (
            "harvest",
            node_id.to_string(),
            Some(rts_runtime_tile_id(rts_harvest_tile_for_node(node_id))),
            rts_catalog_text_label(node_id, 20),
        )
    } else if queue_id.starts_with("upgrade:") {
        let (upgrade_id, source_id) = rts_tech_parts(queue_id, "upgrade:", "training_hall");
        (
            "upgrade",
            upgrade_id.clone(),
            Some(rts_runtime_tile_id(rts_structure_tile_for_id(&source_id))),
            rts_catalog_text_label(&upgrade_id, 20),
        )
    } else {
        (
            "queue",
            queue_id.to_string(),
            None,
            rts_catalog_text_label(queue_id, 20),
        )
    };
    let tile_suffix = tile_id
        .as_deref()
        .map(|tile| format!(" {tile}"))
        .unwrap_or_default();
    RtsCommandStamp {
        input_source: input_source.to_string(),
        kind: kind.to_string(),
        tile_id,
        target_id: Some(target_id),
        player_label: format!(
            "{source} {} SENT {item_label}{tile_suffix}",
            kind.to_ascii_uppercase()
        ),
    }
}

pub fn rts_command_stamp_for_move(input_source: &str, command_id: &str) -> Option<RtsCommandStamp> {
    let source = rts_input_source_player_label(input_source, "RTS:MOVE");
    let (tile_id, formation) = rts_move_command_parts(command_id);
    rts_parse_tile_id(tile_id)?;
    let follow_target_id = rts_move_follow_target(formation);
    let formation_kind = rts_move_formation_kind(formation);
    let kind = if command_id.starts_with("minimap:") || formation_kind == "rally" {
        "rally"
    } else if formation_kind == "shift_waypoint" {
        "waypoint"
    } else if formation_kind == "attack_move" {
        "attack-move"
    } else if formation_kind == "patrol" {
        "patrol"
    } else if formation_kind == "hold" {
        "hold"
    } else if formation_kind == "stop" {
        "stop"
    } else if formation_kind == "follow" {
        "follow"
    } else {
        "move"
    };
    let target_id = follow_target_id.map(ToOwned::to_owned);
    let player_label = if let Some(target_id) = follow_target_id {
        format!(
            "{source} FOLLOW SENT {}",
            rts_catalog_text_label(target_id, 22)
        )
    } else {
        format!(
            "{source} {} SENT {tile_id}",
            kind.replace('-', " ").to_ascii_uppercase()
        )
    };
    Some(RtsCommandStamp {
        input_source: input_source.to_string(),
        kind: kind.to_string(),
        tile_id: Some(tile_id.to_string()),
        target_id,
        player_label,
    })
}

pub fn rts_command_stamp_for_attack(input_source: &str, target_id: &str) -> RtsCommandStamp {
    let source = rts_input_source_player_label(input_source, "RTS:ATTACK");
    RtsCommandStamp {
        input_source: input_source.to_string(),
        kind: "attack".to_string(),
        tile_id: Some(rts_runtime_tile_id(rts_target_tile_for_id(target_id, 0))),
        target_id: Some(target_id.to_string()),
        player_label: format!(
            "{source} ATTACK SENT {}",
            rts_catalog_text_label(target_id, 22)
        ),
    }
}

pub fn rts_command_stamp_for_ability(
    input_source: &str,
    ability_id: &str,
    attack_target_id: Option<&str>,
) -> RtsCommandStamp {
    let source = rts_input_source_player_label(input_source, "RTS:ABILITY");
    let target_id = attack_target_id.map(ToOwned::to_owned);
    let tile_id =
        attack_target_id.map(|target_id| rts_runtime_tile_id(rts_target_tile_for_id(target_id, 0)));
    RtsCommandStamp {
        input_source: input_source.to_string(),
        kind: "ability".to_string(),
        tile_id,
        target_id,
        player_label: format!(
            "{source} ABILITY SENT {}",
            rts_catalog_text_label(ability_id, 22)
        ),
    }
}

pub fn rts_hover_target_preview_kind(affordance: &str) -> Option<&'static str> {
    if affordance.contains("attack") {
        Some("attack")
    } else if affordance.contains("harvest") {
        Some("harvest")
    } else if affordance.contains("follow") {
        Some("follow")
    } else if affordance.contains("move") {
        Some("move")
    } else {
        None
    }
}

pub fn rts_cursor_kind_for_hover_preview(
    accepted: bool,
    affordance: &str,
    action_label: &str,
) -> &'static str {
    if !accepted {
        return "blocked";
    }
    if affordance.contains("attack") {
        "attack"
    } else if affordance.contains("harvest") {
        "harvest"
    } else if affordance.contains("follow") {
        "follow"
    } else if affordance.contains("build") || affordance.contains("queue") {
        "build"
    } else if affordance.contains("command_button") {
        "ability"
    } else if affordance.contains("rally") || affordance.contains("minimap") {
        "rally"
    } else if affordance.contains("selection") || action_label.starts_with("RTS:SELECT:") {
        "select"
    } else {
        "move"
    }
}

pub fn rts_cursor_label_for_hover_preview(
    input_source: &str,
    action_label: &str,
    accepted: bool,
    cursor_kind: &str,
) -> String {
    let source = rts_input_source_player_label(input_source, action_label);
    let state = if accepted { "READY" } else { "LOCK" };
    format!(
        "{source} CURSOR {} {state}",
        cursor_kind.replace('-', " ").to_ascii_uppercase()
    )
}

pub fn rts_hover_player_label(
    input_source: &str,
    action_label: &str,
    tile_id: Option<&str>,
    queue_id: Option<&str>,
    affordance: &str,
    accepted: bool,
    reason: &str,
) -> String {
    let source = rts_input_source_player_label(input_source, action_label);
    if !accepted && action_label.starts_with("RTS:") {
        let chip = rts_rejection_feedback_chip(action_label, reason);
        return format!("{source} {}", rts_blocked_feedback_player_label(&chip));
    }
    if let Some(queue_id) = queue_id {
        let queue_label = rts_catalog_text_label(
            &queue_id
                .replace("build:", "")
                .replace("train:", "")
                .replace("upgrade:", "")
                .replace("research:", "")
                .replace("harvest:", "")
                .replace('@', " "),
            18,
        );
        let gold = rts_queue_gold_cost(queue_id);
        return if gold > 0 {
            format!("{source} QUEUE READY {queue_label} {gold}G")
        } else if affordance == "viewport_harvest" && queue_id.starts_with("harvest:") {
            format!("{source} HARVEST READY {queue_label}")
        } else {
            format!("{source} QUEUE READY {queue_label}")
        };
    }
    if action_label.starts_with("RTS:MOVE:") {
        let tile = tile_id.unwrap_or("-");
        if let Some(target_id) = action_label
            .strip_prefix("RTS:MOVE:")
            .map(rts_move_command_parts)
            .and_then(|(_, formation)| rts_move_follow_target(formation))
        {
            return format!(
                "{source} FOLLOW READY {}",
                rts_catalog_text_label(&target_id.replace('_', " "), 18)
            );
        }
        return if affordance == "minimap_rally" {
            format!("{source} RALLY READY {tile}")
        } else {
            format!("{source} MOVE READY {tile}")
        };
    }
    if let Some(target_id) = action_label.strip_prefix("RTS:ATTACK:") {
        return format!(
            "{source} ATTACK READY {}",
            rts_catalog_text_label(&target_id.replace('_', " "), 22)
        );
    }
    if let Some(ability_id) = action_label.strip_prefix("RTS:ABILITY:") {
        return format!(
            "{source} ABILITY READY {}",
            rts_catalog_text_label(&ability_id.replace('_', " "), 18)
        );
    }
    if let Some(group_id) = action_label.strip_prefix("RTS:SELECT:") {
        return format!(
            "{source} SELECT READY {}",
            rts_catalog_text_label(group_id, 18)
        );
    }
    format!(
        "{source} READY {}",
        rts_catalog_text_label(&action_label.replace("RTS:", "").replace(':', " "), 24)
    )
}

pub fn rts_blocked_feedback_toast(input_source: &str, action_label: &str, reason: &str) -> String {
    let chip = rts_rejection_feedback_chip(action_label, reason);
    format!(
        "Input blocked: {} {}",
        rts_input_source_player_label(input_source, action_label),
        rts_blocked_feedback_player_label(&chip)
    )
}

pub fn rts_should_emit_rejection_feedback_chip(input_source: &str) -> bool {
    !input_source.contains("bot_executor")
}

pub fn rts_executable_command_queue_snapshot(queue: &[String]) -> Vec<String> {
    queue
        .iter()
        .filter(|entry| !entry.starts_with("feedback:blocked:"))
        .cloned()
        .collect()
}

pub fn rts_blocked_feedback_chip_visible(command_queue: &[String]) -> bool {
    command_queue
        .iter()
        .any(|entry| entry.starts_with("feedback:blocked:"))
}

pub fn rts_command_surface_stage(
    combat_turn: u8,
    combat_events: &[String],
    command_queue: &[String],
) -> Option<&'static str> {
    for event in combat_events.iter().rev() {
        if event.contains("surface:target_queue") {
            return Some("target_queue");
        }
        if event.contains("surface:cooldown_disabled") {
            return Some("cooldown_disabled");
        }
        if event.contains("surface:command_grid") {
            return Some("command_grid");
        }
        if event.contains("surface:selection_state") {
            return Some("selection_state");
        }
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("surface:"))
    {
        return None;
    }
    Some(match combat_turn % 4 {
        0 => "selection_state",
        1 => "command_grid",
        2 => "cooldown_disabled",
        _ => "target_queue",
    })
}

pub fn rts_command_feedback_strip_stage(
    combat_turn: u8,
    combat_events: &[String],
    command_queue: &[String],
) -> Option<&'static str> {
    for event in combat_events.iter().rev().chain(command_queue.iter().rev()) {
        if event.contains("control_group_command_feedback_strip:group_28_filtered") {
            return Some("group_28_filtered");
        }
        if event.contains("control_group_command_feedback_strip:group_28_formation") {
            return Some("group_28_formation");
        }
        if event.contains("control_group_command_feedback_strip:group_27_override") {
            return Some("group_27_override");
        }
        if event.contains("control_group_command_feedback_strip:group_26_queued") {
            return Some("group_26_queued");
        }
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("control_group_command_feedback_strip:"))
    {
        return None;
    }
    Some(match combat_turn % 4 {
        0 => "group_26_queued",
        1 => "group_27_override",
        2 => "group_28_formation",
        _ => "group_28_filtered",
    })
}

fn rts_feedback_lifecycle_texts<'a>(
    group_command_state: &'a str,
    combat_events: &'a [String],
    command_queue: &'a [String],
) -> impl Iterator<Item = &'a str> {
    std::iter::once(group_command_state)
        .chain(combat_events.iter().rev().map(String::as_str))
        .chain(command_queue.iter().rev().map(String::as_str))
}

pub fn rts_command_feedback_lifecycle_stage(
    group_command_state: &str,
    combat_events: &[String],
    command_queue: &[String],
) -> Option<&'static str> {
    for text in rts_feedback_lifecycle_texts(group_command_state, combat_events, command_queue) {
        if text.contains("control_group_command_feedback_lifecycle:cleared")
            || text.contains("command_feedback_lifecycle:cleared")
        {
            return Some("cleared");
        }
        if text.contains("control_group_command_feedback_lifecycle:dimmed")
            || text.contains("command_feedback_lifecycle:dimmed")
        {
            return Some("dimmed");
        }
        if text.contains("control_group_command_feedback_lifecycle:fresh")
            || text.contains("command_feedback_lifecycle:fresh")
        {
            return Some("fresh");
        }
    }
    None
}

pub fn rts_command_history_visible(
    group_command_state: &str,
    combat_events: &[String],
    command_queue: &[String],
) -> bool {
    rts_feedback_lifecycle_texts(group_command_state, combat_events, command_queue).any(|text| {
        text.contains("control_group_command_history:")
            || text.contains("command_feedback_history:")
    })
}

pub fn rts_command_history_prune_visible(
    group_command_state: &str,
    combat_events: &[String],
    command_queue: &[String],
) -> bool {
    rts_feedback_lifecycle_texts(group_command_state, combat_events, command_queue).any(|text| {
        text.contains("control_group_command_history_prune:")
            || text.contains("command_history_prune:")
            || text.contains("history_row_pruned:")
    })
}

pub fn rts_command_execution_feedback_kind(
    unit_response_state: &str,
    group_command_state: &str,
    economy_state: &str,
    command_destination_tile_present: bool,
    minimap_command_kind: &str,
    has_path_tiles: bool,
    has_group_route_tiles: bool,
    command_queue: &[String],
) -> Option<&'static str> {
    if let Some(recent_feedback_chip) = command_queue
        .iter()
        .rev()
        .find(|entry| entry.starts_with("feedback:"))
    {
        if recent_feedback_chip.starts_with("feedback:harvest_assigned:") {
            return Some("harvest");
        }
        if recent_feedback_chip.starts_with("feedback:follow@") {
            return Some("follow");
        }
        if recent_feedback_chip.starts_with("feedback:attack_move@") {
            return Some("attack");
        }
        if recent_feedback_chip.starts_with("feedback:line@")
            || recent_feedback_chip.starts_with("feedback:diamond@")
            || recent_feedback_chip.starts_with("feedback:waypoint_queued@")
            || recent_feedback_chip.starts_with("feedback:hold_position@")
            || recent_feedback_chip.starts_with("feedback:patrol_route@")
            || recent_feedback_chip.starts_with("feedback:rally_confirmed@")
            || recent_feedback_chip.starts_with("feedback:stop_hold@")
        {
            return Some("move");
        }
    }
    if unit_response_state.starts_with("following:")
        || group_command_state.starts_with("follow:")
        || minimap_command_kind == "follow"
    {
        Some("follow")
    } else if unit_response_state.starts_with("engaged:")
        || unit_response_state.starts_with("attack_move_advancing:")
        || group_command_state.starts_with("attack_move:")
    {
        Some("attack")
    } else if command_destination_tile_present
        && minimap_command_kind != "harvest"
        && !economy_state.starts_with("harvesting:")
        && (has_path_tiles || has_group_route_tiles)
    {
        Some("move")
    } else if minimap_command_kind == "harvest"
        || economy_state.starts_with("harvesting:")
        || command_queue
            .iter()
            .rev()
            .any(|entry| entry.starts_with("harvest:"))
    {
        Some("harvest")
    } else {
        None
    }
}

fn rts_recent_stage_from_events(
    markers: &[(&str, &'static str)],
    combat_events: &[String],
    command_queue: &[String],
) -> Option<&'static str> {
    for event in combat_events.iter().rev().chain(command_queue.iter().rev()) {
        if let Some((_, stage)) = markers.iter().find(|(marker, _)| event.contains(marker)) {
            return Some(*stage);
        }
    }
    None
}

pub fn rts_unit_status_portrait_stage(
    combat_turn: u8,
    combat_events: &[String],
    command_queue: &[String],
) -> Option<&'static str> {
    for event in combat_events.iter().rev() {
        if event.contains("unit_status_portrait:multi_select") {
            return Some("multi_select");
        }
        if event.contains("unit_status_portrait:structure") {
            return Some("structure");
        }
        if event.contains("unit_status_portrait:creep_target") {
            return Some("creep_target");
        }
        if event.contains("unit_status_portrait:commander") {
            return Some("commander");
        }
        if event.contains("unit_status_portrait:guard") {
            return Some("guard");
        }
        if event.contains("unit_status_portrait:worker") {
            return Some("worker");
        }
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("unit_status_portrait:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "worker",
        1 => "guard",
        2 => "commander",
        3 => "creep_target",
        4 => "structure",
        _ => "multi_select",
    })
}

pub fn rts_unit_status_portrait_unit_id(
    stage: &str,
    selected_unit_ids: &[String],
    commander_unit_id: Option<&str>,
    attack_target_id: Option<&str>,
    completed_structure_ids: &[String],
) -> String {
    match stage {
        "worker" => selected_unit_ids
            .iter()
            .find(|id| id.contains("worker"))
            .cloned()
            .unwrap_or_else(|| "square_worker_carry".to_string()),
        "guard" => selected_unit_ids
            .iter()
            .find(|id| id.contains("guard"))
            .cloned()
            .unwrap_or_else(|| "arena_guard_left".to_string()),
        "commander" => commander_unit_id
            .map(str::to_string)
            .unwrap_or_else(|| "mirror_captain".to_string()),
        "creep_target" => attack_target_id
            .map(str::to_string)
            .unwrap_or_else(|| "arena_creep_attack".to_string()),
        "structure" => completed_structure_ids
            .first()
            .cloned()
            .unwrap_or_else(|| "training_hall".to_string()),
        _ => selected_unit_ids
            .first()
            .cloned()
            .unwrap_or_else(|| "player".to_string()),
    }
}

pub fn rts_unit_status_health_percent(
    stage: &str,
    unit_health_percents: &[u8],
    structure_health_percents: &[u8],
    target_health_percent: u8,
) -> u8 {
    unit_health_percents.first().copied().unwrap_or_else(|| {
        if stage == "structure" {
            structure_health_percents.first().copied().unwrap_or(86)
        } else if stage == "creep_target" {
            target_health_percent.max(1)
        } else {
            88
        }
    })
}

pub fn rts_unit_status_energy_percent(ability_cooldown_percents: &[u8]) -> u8 {
    ability_cooldown_percents
        .first()
        .copied()
        .map(|cooldown| 100_u8.saturating_sub(cooldown))
        .unwrap_or(68)
}

pub fn rts_unit_status_role_badges(stage: &str) -> [&'static str; 3] {
    match stage {
        "worker" => ["HAR", "REP", "RET"],
        "guard" => ["ATK", "HLD", "DEF"],
        "commander" => ["AUR", "LVL", "CMD"],
        "creep_target" => ["THR", "ARM", "FOC"],
        "structure" => ["Q", "BLD", "UP"],
        _ => ["G1", "SEL", "ORD"],
    }
}

pub fn rts_selection_command_feedback_stage(
    combat_turn: u8,
    combat_events: &[String],
    command_queue: &[String],
) -> Option<&'static str> {
    if let Some(stage) = rts_recent_stage_from_events(
        &[
            ("selection_command_feedback:invalid_order", "invalid_order"),
            ("selection_command_feedback:attack_lock", "attack_lock"),
            ("selection_command_feedback:move_line", "move_line"),
            ("selection_command_feedback:rally_preview", "rally_preview"),
            (
                "selection_command_feedback:selection_confirm",
                "selection_confirm",
            ),
            ("selection_command_feedback:marquee_start", "marquee_start"),
        ],
        combat_events,
        command_queue,
    ) {
        return Some(stage);
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("selection_command_feedback:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "marquee_start",
        1 => "selection_confirm",
        2 => "rally_preview",
        3 => "move_line",
        4 => "attack_lock",
        _ => "invalid_order",
    })
}

pub fn rts_ability_tooltip_telegraph_stage(
    combat_turn: u8,
    combat_events: &[String],
    command_queue: &[String],
) -> Option<&'static str> {
    if let Some(stage) = rts_recent_stage_from_events(
        &[
            (
                "ability_tooltip_telegraph:resource_warning",
                "resource_warning",
            ),
            ("ability_tooltip_telegraph:queue_explain", "queue_explain"),
            ("ability_tooltip_telegraph:cooldown_sweep", "cooldown_sweep"),
            ("ability_tooltip_telegraph:cast_windup", "cast_windup"),
            ("ability_tooltip_telegraph:range_preview", "range_preview"),
            ("ability_tooltip_telegraph:hover_tooltip", "hover_tooltip"),
        ],
        combat_events,
        command_queue,
    ) {
        return Some(stage);
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("ability_tooltip_telegraph:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "hover_tooltip",
        1 => "range_preview",
        2 => "cast_windup",
        3 => "cooldown_sweep",
        4 => "queue_explain",
        _ => "resource_warning",
    })
}

pub fn rts_control_group_hotkey_feedback_stage(
    combat_turn: u8,
    combat_events: &[String],
    command_queue: &[String],
) -> Option<&'static str> {
    if let Some(stage) = rts_recent_stage_from_events(
        &[
            (
                "control_group_hotkey_feedback:ability_hotkey_ack",
                "ability_hotkey_ack",
            ),
            (
                "control_group_hotkey_feedback:production_hotkey",
                "production_hotkey",
            ),
            (
                "control_group_hotkey_feedback:idle_worker_ping",
                "idle_worker_ping",
            ),
            (
                "control_group_hotkey_feedback:double_tap_camera",
                "double_tap_camera",
            ),
            ("control_group_hotkey_feedback:recall_group", "recall_group"),
            ("control_group_hotkey_feedback:assign_group", "assign_group"),
        ],
        combat_events,
        command_queue,
    ) {
        return Some(stage);
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("control_group_hotkey_feedback:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "assign_group",
        1 => "recall_group",
        2 => "double_tap_camera",
        3 => "idle_worker_ping",
        4 => "production_hotkey",
        _ => "ability_hotkey_ack",
    })
}

pub fn rts_blocked_feedback_player_label(chip: &str) -> String {
    let blocked = chip.strip_prefix("feedback:blocked:").unwrap_or(chip);
    if let Some(queue_id) = blocked.strip_prefix("queue:rts_queue_unaffordable:") {
        return format!("QUEUE LOCK NEED {}G", rts_queue_gold_cost(queue_id));
    }
    if blocked == "queue:rts_queue_id_required" {
        return "QUEUE LOCK PICK ITEM".to_string();
    }
    if blocked == "select:rts_group_id_required" {
        return "SELECT LOCK GROUP ID".to_string();
    }
    if blocked == "attack:rts_attack_target_required" {
        return "ATTACK LOCK PICK TARGET".to_string();
    }
    if blocked == "ability:rts_attack_required_before_ability" {
        return "ABILITY LOCK NEED TARGET".to_string();
    }
    if blocked == "move:rts_group_selection_required" || blocked == "move:select_units" {
        return "MOVE LOCK SELECT UNITS".to_string();
    }
    if blocked.starts_with("move:rts_invalid_tile:") {
        return "MOVE LOCK INVALID TILE".to_string();
    }
    blocked
        .replace("rts_", "")
        .replace(':', " ")
        .replace('_', " ")
        .to_ascii_uppercase()
}

pub fn rts_scripted_demo_pauses_queue_tick(demo_id: &str) -> bool {
    matches!(
        demo_id,
        "queue_cancel_refund" | "queue_cancel_refund_sequence"
    )
}

pub fn rts_scripted_demo_stage_from_frame(demo_id: &str, frame_tick: u64) -> Option<usize> {
    match demo_id {
        "queue_cancel_refund_sequence" => Some(((frame_tick / 60) % 5) as usize),
        _ => None,
    }
}

pub fn rts_scripted_demo_stage_id(stage: usize) -> &'static str {
    match stage {
        0 => "drag_select_frontline",
        1 => "rally_path_minimap",
        2 => "watch_tower_footprint",
        3 => "cancel_refund",
        4 => "queued_worker_ready",
        _ => "unknown",
    }
}

pub fn rts_scripted_demo_stage_title(stage: usize) -> &'static str {
    match stage {
        0 => "DRAG SELECT",
        1 => "RALLY / MINIMAP",
        2 => "BUILD FOOTPRINT",
        3 => "CANCEL / REFUND",
        4 => "WORKER QUEUED",
        _ => "UNKNOWN",
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

pub fn rts_formation_move_preview_stage(
    combat_events: &[String],
    command_queue: &[String],
    combat_turn: u8,
) -> Option<&'static str> {
    for event in combat_events.iter().rev().chain(command_queue.iter().rev()) {
        if event.contains("formation_move_preview:commit_spacing") {
            return Some("commit_spacing");
        }
        if event.contains("formation_move_preview:split_avoidance") {
            return Some("split_avoidance");
        }
        if event.contains("formation_move_preview:collision_avoidance") {
            return Some("collision_avoidance");
        }
        if event.contains("formation_move_preview:line_reflow") {
            return Some("line_reflow");
        }
        if event.contains("formation_move_preview:wedge_spacing") {
            return Some("wedge_spacing");
        }
        if event.contains("formation_move_preview:destination_ghost") {
            return Some("destination_ghost");
        }
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("formation_move_preview:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "destination_ghost",
        1 => "wedge_spacing",
        2 => "line_reflow",
        3 => "collision_avoidance",
        4 => "split_avoidance",
        _ => "commit_spacing",
    })
}

pub fn rts_formation_move_execution_stage(
    combat_events: &[String],
    command_queue: &[String],
    combat_turn: u8,
) -> Option<&'static str> {
    for event in combat_events.iter().rev().chain(command_queue.iter().rev()) {
        if event.contains("formation_move_execution:arrival_lock") {
            return Some("arrival_lock");
        }
        if event.contains("formation_move_execution:blocked_reroute") {
            return Some("blocked_reroute");
        }
        if event.contains("formation_move_execution:crowd_avoidance") {
            return Some("crowd_avoidance");
        }
        if event.contains("formation_move_execution:stagger_step") {
            return Some("stagger_step");
        }
        if event.contains("formation_move_execution:path_reservation") {
            return Some("path_reservation");
        }
        if event.contains("formation_move_execution:slot_claim") {
            return Some("slot_claim");
        }
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("formation_move_execution:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "slot_claim",
        1 => "path_reservation",
        2 => "stagger_step",
        3 => "crowd_avoidance",
        4 => "blocked_reroute",
        _ => "arrival_lock",
    })
}

pub fn rts_local_obstruction_recovery_stage(
    combat_events: &[String],
    command_queue: &[String],
    combat_turn: u8,
) -> Option<&'static str> {
    for event in combat_events.iter().rev().chain(command_queue.iter().rev()) {
        if event.contains("local_obstruction_recovery:flow_resume") {
            return Some("flow_resume");
        }
        if event.contains("local_obstruction_recovery:gap_claim") {
            return Some("gap_claim");
        }
        if event.contains("local_obstruction_recovery:side_step") {
            return Some("side_step");
        }
        if event.contains("local_obstruction_recovery:hold_queue") {
            return Some("hold_queue");
        }
        if event.contains("local_obstruction_recovery:detect_block") {
            return Some("detect_block");
        }
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("local_obstruction_recovery:"))
    {
        return None;
    }
    Some(match combat_turn % 5 {
        0 => "detect_block",
        1 => "hold_queue",
        2 => "side_step",
        3 => "gap_claim",
        _ => "flow_resume",
    })
}

fn rts_unit_model_depth_mark(
    kind: &str,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> RtsUnitModelDepthMark {
    RtsUnitModelDepthMark {
        kind: kind.to_string(),
        rect: RtsRuntimeRect {
            x,
            y,
            width,
            height,
        },
    }
}

pub fn rts_unit_model_depth_marks(frame_id: &str) -> Vec<RtsUnitModelDepthMark> {
    if !(frame_id.starts_with("actor_guard")
        || frame_id.starts_with("actor_worker")
        || frame_id.starts_with("actor_creep"))
    {
        return Vec::new();
    }

    let mut marks = vec![
        rts_unit_model_depth_mark("ground_contact", -14, -3, 28, 2),
        rts_unit_model_depth_mark("rim", -9, -30, 2, 21),
        rts_unit_model_depth_mark("rim", 7, -30, 2, 21),
        rts_unit_model_depth_mark("layer_shadow", -5, -23, 10, 3),
        rts_unit_model_depth_mark("face_shade", -3, -32, 6, 2),
    ];

    if frame_id.starts_with("actor_guard") {
        marks.push(rts_unit_model_depth_mark("armor", -11, -27, 5, 4));
        marks.push(rts_unit_model_depth_mark("armor", 6, -27, 5, 4));
        marks.push(rts_unit_model_depth_mark("role_prop", -4, -38, 8, 3));
    } else if frame_id.starts_with("actor_worker") {
        marks.push(rts_unit_model_depth_mark("role_prop", -15, -26, 5, 13));
        marks.push(rts_unit_model_depth_mark("armor", 9, -25, 5, 12));
        marks.push(rts_unit_model_depth_mark("layer_shadow", -13, -14, 22, 3));
    } else {
        marks.push(rts_unit_model_depth_mark("role_prop", -10, -40, 6, 5));
        marks.push(rts_unit_model_depth_mark("role_prop", 4, -40, 6, 5));
        marks.push(rts_unit_model_depth_mark("armor", -11, -21, 22, 3));
    }

    marks
}

fn rts_action_cadence_mark(
    kind: &str,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> RtsActionCadenceMark {
    RtsActionCadenceMark {
        kind: kind.to_string(),
        rect: RtsRuntimeRect {
            x,
            y,
            width,
            height,
        },
    }
}

pub fn rts_action_cadence_marks(frame_id: &str) -> Vec<RtsActionCadenceMark> {
    let attack_frame = frame_id.ends_with("_attack");
    let carry_frame = frame_id.ends_with("_carry");
    let idle_frame = frame_id.ends_with("_idle");
    let mut marks = Vec::new();

    if attack_frame {
        let windup_left = if frame_id.starts_with("actor_creep") {
            -24
        } else {
            -22
        };
        for step in 0..5 {
            marks.push(rts_action_cadence_mark(
                "windup",
                windup_left + step * 3,
                -36 + step,
                7,
                3,
            ));
        }
        for step in 0..9 {
            marks.push(rts_action_cadence_mark(
                "strike",
                12 + step * 3,
                -34 + step,
                7,
                3,
            ));
        }
        for step in 0..6 {
            marks.push(rts_action_cadence_mark(
                "recovery",
                4 + step * 4,
                -18 + step,
                6,
                3,
            ));
        }
        marks.push(rts_action_cadence_mark("shadow_smear", -15, -7, 32, 3));
        marks.push(rts_action_cadence_mark("shadow_smear", -10, -4, 24, 2));
    } else if carry_frame {
        for step in 0..4 {
            marks.push(rts_action_cadence_mark(
                "carry_bob",
                14 + step * 2,
                -34 - (step % 2),
                4,
                6,
            ));
            marks.push(rts_action_cadence_mark(
                "shadow_smear",
                12 + step * 3,
                -17 + step,
                4,
                3,
            ));
        }
    } else if idle_frame
        && (frame_id.starts_with("actor_guard")
            || frame_id.starts_with("actor_worker")
            || frame_id.starts_with("actor_creep"))
    {
        for step in 0..4 {
            marks.push(rts_action_cadence_mark(
                "idle_breath",
                -10 + step * 6,
                -31 + (step % 2),
                4,
                2,
            ));
        }
    }

    marks
}

pub fn rts_action_sequence_phase(
    frame_id: &str,
    combat_events: &[String],
    command_queue: &[String],
    walk_cycle_frame: u8,
    combat_turn: u8,
    runtime_available: bool,
) -> Option<&'static str> {
    if runtime_available {
        for event in combat_events.iter().rev() {
            if event.contains("sequence:carry_down") {
                return Some("carry_down");
            }
            if event.contains("sequence:carry_up") {
                return Some("carry_up");
            }
            if event.contains("sequence:recovery") {
                return Some("recovery");
            }
            if event.contains("sequence:strike") {
                return Some("strike");
            }
            if event.contains("sequence:windup") {
                return Some("windup");
            }
            if event.contains("sequence:idle") {
                return Some("idle");
            }
        }
        if !command_queue
            .iter()
            .any(|command| command.contains("sequence:"))
        {
            return None;
        }
        if frame_id.contains("carry") {
            return Some(if walk_cycle_frame % 2 == 0 {
                "carry_up"
            } else {
                "carry_down"
            });
        }
        if frame_id.contains("attack") {
            return Some(match combat_turn % 4 {
                1 => "windup",
                2 => "strike",
                3 => "recovery",
                _ => "idle",
            });
        }
    } else if frame_id.contains("carry") {
        return Some("carry_up");
    } else if frame_id.contains("attack") {
        return Some("strike");
    }
    None
}

fn rts_action_sequence_mark(
    kind: &str,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> RtsActionSequenceMark {
    RtsActionSequenceMark {
        kind: kind.to_string(),
        rect: RtsRuntimeRect {
            x,
            y,
            width,
            height,
        },
    }
}

pub fn rts_action_sequence_marks(frame_id: &str, phase: &str) -> Vec<RtsActionSequenceMark> {
    if !(frame_id.starts_with("actor_guard")
        || frame_id.starts_with("actor_worker")
        || frame_id.starts_with("actor_creep"))
    {
        return Vec::new();
    }

    let mut marks = vec![rts_action_sequence_mark("frame_ghost", -16, -7, 32, 2)];

    match phase {
        "windup" => {
            for step in 0..7 {
                marks.push(rts_action_sequence_mark(
                    "windup",
                    -28 + step * 3,
                    -39 + step,
                    7,
                    3,
                ));
            }
            marks.push(rts_action_sequence_mark("windup", -13, -29, 9, 12));
        }
        "strike" => {
            for step in 0..10 {
                marks.push(rts_action_sequence_mark(
                    "strike",
                    8 + step * 3,
                    -38 + step,
                    8,
                    3,
                ));
            }
            marks.push(rts_action_sequence_mark("strike", 24, -28, 12, 10));
        }
        "recovery" => {
            for step in 0..7 {
                marks.push(rts_action_sequence_mark(
                    "recovery",
                    -4 + step * 4,
                    -20 + step,
                    7,
                    3,
                ));
            }
            marks.push(rts_action_sequence_mark("recovery", 6, -32, 8, 16));
        }
        "carry_up" => {
            if frame_id.contains("carry") || frame_id.starts_with("actor_worker") {
                marks.push(rts_action_sequence_mark("carry_up", 11, -39, 16, 6));
                marks.push(rts_action_sequence_mark("carry_up", 15, -31, 10, 6));
            }
        }
        "carry_down" => {
            marks.push(rts_action_sequence_mark("carry_down", -14, -11, 28, 3));
            marks.push(rts_action_sequence_mark("carry_down", -5, -25, 10, 6));
            if frame_id.contains("carry") || frame_id.starts_with("actor_worker") {
                marks.push(rts_action_sequence_mark("carry_down", 10, -28, 18, 7));
                marks.push(rts_action_sequence_mark("carry_down", 7, -18, 20, 4));
            }
        }
        _ => {
            for step in 0..4 {
                marks.push(rts_action_sequence_mark(
                    "idle",
                    -11 + step * 7,
                    -32 + (step % 2),
                    5,
                    2,
                ));
            }
            marks.push(rts_action_sequence_mark("idle", -8, -18, 16, 3));
        }
    }

    marks
}

pub fn rts_npc_behavior_stage(
    combat_events: &[String],
    command_queue: &[String],
    combat_turn: u8,
) -> Option<&'static str> {
    if let Some(stage) = rts_recent_stage_from_events(
        &[
            ("behavior:guard_patrol", "guard_patrol"),
            ("behavior:guard_engage", "guard_engage"),
            ("behavior:worker_work", "worker_work"),
            ("behavior:worker_carry", "worker_carry"),
            ("behavior:creep_stalk", "creep_stalk"),
            ("behavior:creep_retreat", "creep_retreat"),
        ],
        combat_events,
        command_queue,
    ) {
        return Some(stage);
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("behavior:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "guard_patrol",
        1 => "guard_engage",
        2 => "worker_work",
        3 => "worker_carry",
        4 => "creep_stalk",
        _ => "creep_retreat",
    })
}

pub fn rts_combat_impact_stage(
    combat_events: &[String],
    command_queue: &[String],
    combat_turn: u8,
) -> Option<&'static str> {
    if let Some(stage) = rts_recent_stage_from_events(
        &[
            ("impact:victory_settle", "victory_settle"),
            ("impact:corpse_dissolve", "corpse_dissolve"),
            ("impact:death_fall", "death_fall"),
            ("impact:damage_tick", "damage_tick"),
            ("impact:stagger", "stagger"),
            ("impact:hit_flash", "hit_flash"),
        ],
        combat_events,
        command_queue,
    ) {
        return Some(stage);
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("impact:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "hit_flash",
        1 => "stagger",
        2 => "damage_tick",
        3 => "death_fall",
        4 => "corpse_dissolve",
        _ => "victory_settle",
    })
}

pub fn rts_locomotion_blend_stage(
    combat_events: &[String],
    command_queue: &[String],
    walk_cycle_frame: u8,
) -> Option<&'static str> {
    if let Some(stage) = rts_recent_stage_from_events(
        &[
            ("locomotion:arrival_brake", "arrival_brake"),
            ("locomotion:formation_slide", "formation_slide"),
            ("locomotion:turn_arc", "turn_arc"),
            ("locomotion:footstep_right", "footstep_right"),
            ("locomotion:footstep_left", "footstep_left"),
            ("locomotion:path_commit", "path_commit"),
        ],
        combat_events,
        command_queue,
    ) {
        return Some(stage);
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("locomotion:"))
    {
        return None;
    }
    Some(match walk_cycle_frame % 6 {
        0 => "path_commit",
        1 => "footstep_left",
        2 => "footstep_right",
        3 => "turn_arc",
        4 => "formation_slide",
        _ => "arrival_brake",
    })
}

pub fn rts_npc_transition_stage(
    combat_events: &[String],
    command_queue: &[String],
    combat_turn: u8,
) -> Option<&'static str> {
    if let Some(stage) = rts_recent_stage_from_events(
        &[
            ("transition:retreat_resume", "retreat_resume"),
            ("transition:hit_recover", "hit_recover"),
            ("transition:stalk_pounce", "stalk_pounce"),
            ("transition:work_carry", "work_carry"),
            ("transition:patrol_engage", "patrol_engage"),
            ("transition:alert_turn", "alert_turn"),
        ],
        combat_events,
        command_queue,
    ) {
        return Some(stage);
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("transition:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "alert_turn",
        1 => "patrol_engage",
        2 => "work_carry",
        3 => "stalk_pounce",
        4 => "hit_recover",
        _ => "retreat_resume",
    })
}

pub fn rts_depth_readability_stage(
    combat_events: &[String],
    command_queue: &[String],
    combat_turn: u8,
) -> Option<&'static str> {
    if let Some(stage) = rts_recent_stage_from_events(
        &[
            ("depth:terrain_cutaway", "terrain_cutaway"),
            ("depth:path_occlusion", "path_occlusion"),
            ("depth:target_priority", "target_priority"),
            ("depth:building_mask", "building_mask"),
            ("depth:behind_silhouette", "behind_silhouette"),
            ("depth:foreground_canopy", "foreground_canopy"),
        ],
        combat_events,
        command_queue,
    ) {
        return Some(stage);
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("depth:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "foreground_canopy",
        1 => "behind_silhouette",
        2 => "building_mask",
        3 => "target_priority",
        4 => "path_occlusion",
        _ => "terrain_cutaway",
    })
}

pub fn rts_structure_modeling_stage(
    combat_events: &[String],
    command_queue: &[String],
    combat_turn: u8,
) -> Option<&'static str> {
    if let Some(stage) = rts_recent_stage_from_events(
        &[
            ("structure:repair_beam", "repair_beam"),
            ("structure:damage_crack", "damage_crack"),
            ("structure:production_glow", "production_glow"),
            ("structure:construction_spark", "construction_spark"),
            ("structure:scaffold", "scaffold"),
            ("structure:foundation_shadow", "foundation_shadow"),
        ],
        combat_events,
        command_queue,
    ) {
        return Some(stage);
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("structure:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "foundation_shadow",
        1 => "scaffold",
        2 => "construction_spark",
        3 => "production_glow",
        4 => "damage_crack",
        _ => "repair_beam",
    })
}

pub fn rts_environment_life_stage(
    combat_events: &[String],
    command_queue: &[String],
    combat_turn: u8,
) -> Option<&'static str> {
    if let Some(stage) = rts_recent_stage_from_events(
        &[
            ("environment:ambient_dust", "ambient_dust"),
            ("environment:resource_glint", "resource_glint"),
            ("environment:banner_flutter", "banner_flutter"),
            ("environment:water_shimmer", "water_shimmer"),
            ("environment:torch_flicker", "torch_flicker"),
            ("environment:tree_sway", "tree_sway"),
        ],
        combat_events,
        command_queue,
    ) {
        return Some(stage);
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("environment:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "tree_sway",
        1 => "torch_flicker",
        2 => "water_shimmer",
        3 => "banner_flutter",
        4 => "resource_glint",
        _ => "ambient_dust",
    })
}

pub fn rts_worker_harvest_animation_stage(
    combat_events: &[String],
    command_queue: &[String],
    combat_turn: u8,
) -> Option<&'static str> {
    if let Some(stage) = rts_recent_stage_from_events(
        &[
            ("harvest_anim:return_path", "return_path"),
            ("harvest_anim:dropoff_burst", "dropoff_burst"),
            ("harvest_anim:carry_load", "carry_load"),
            ("harvest_anim:resource_pop", "resource_pop"),
            ("harvest_anim:tool_swing", "tool_swing"),
            ("harvest_anim:approach", "approach"),
        ],
        combat_events,
        command_queue,
    ) {
        return Some(stage);
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("harvest_anim:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "approach",
        1 => "tool_swing",
        2 => "resource_pop",
        3 => "carry_load",
        4 => "dropoff_burst",
        _ => "return_path",
    })
}

pub fn rts_production_spawn_animation_stage(
    combat_events: &[String],
    command_queue: &[String],
    combat_turn: u8,
) -> Option<&'static str> {
    if let Some(stage) = rts_recent_stage_from_events(
        &[
            ("production_spawn_anim:supply_flash", "supply_flash"),
            ("production_spawn_anim:formation_join", "formation_join"),
            ("production_spawn_anim:rally_flag", "rally_flag"),
            ("production_spawn_anim:spawn_door", "spawn_door"),
            ("production_spawn_anim:training_tick", "training_tick"),
            ("production_spawn_anim:queue_pulse", "queue_pulse"),
        ],
        combat_events,
        command_queue,
    ) {
        return Some(stage);
    }
    if !command_queue
        .iter()
        .any(|command| command.contains("production_spawn_anim:"))
    {
        return None;
    }
    Some(match combat_turn % 6 {
        0 => "queue_pulse",
        1 => "training_tick",
        2 => "spawn_door",
        3 => "rally_flag",
        4 => "formation_join",
        _ => "supply_flash",
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
    fn army_production_rally_adapter_preserves_first_contact_routes() {
        assert_eq!(
            rts_army_units_for_batch("mixed_vanguard"),
            vec![
                "relay_guard_alpha",
                "relay_guard_beta",
                "wayfinder_scout",
                "field_mender"
            ]
        );
        assert_eq!(
            rts_army_rally_tiles_for_id("forward_watch"),
            vec!["5,5", "6,5", "7,4", "8,4", "8,3"]
        );
        assert_eq!(rts_player_army_unit_tile_for_id("field_mender", 3), (6, 4));
        assert_eq!(rts_player_army_unit_tile_for_id("custom_guard", 4), (7, 4));
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
        assert_eq!(rts_enemy_fortification_tile_for_id("gate_bulwark"), (10, 3));
        assert_eq!(
            rts_enemy_repair_units_for_target("gate_bulwark"),
            vec!["repair_adept_alpha", "repair_adept_beta"]
        );
        assert_eq!(
            rts_enemy_flank_units_for_id("ridge_sentries"),
            vec!["ridge_sentry_left", "ridge_sentry_right", "ridge_sapper"]
        );
        assert_eq!(rts_enemy_flank_tile_for_index(2), (8, 4));
        assert_eq!(
            rts_player_hold_tiles_for_id("shield_line", "9,3"),
            vec!["8,3", "9,3", "9,4", "10,3"]
        );
        assert_eq!(
            rts_inner_lane_tiles_for_id("inner_lane", "11,2"),
            vec!["10,3", "11,2", "11,3", "12,3", "12,4"]
        );
        assert_eq!(rts_inner_gate_tile_for_id("inner_latch"), (11, 3));
        assert_eq!(rts_inner_gate_tile_for_id("signal_lock"), (12, 3));
        assert_eq!(
            rts_inner_defenders_for_id("second_line"),
            vec!["inner_guard_alpha", "inner_guard_beta", "signal_lancer"]
        );
        assert_eq!(
            rts_supply_convoy_for_id("relay_convoy"),
            vec!["convoy_cart", "field_medic", "ammo_runner"]
        );
        assert_eq!(
            rts_split_squad_tiles_for_id("flank_team", "10,4"),
            vec!["10,4", "11,4", "12,4", "12,3"]
        );
        assert_eq!(rts_inner_core_tile_for_id("signal_core"), (12, 3));
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

    #[test]
    fn restoration_open_world_adapter_preserves_first_contact_routes() {
        assert_eq!(
            rts_restored_zones_for_id("mirror_city"),
            vec!["central_keep", "signal_core", "inner_lane", "forest_relay"]
        );
        assert_eq!(
            rts_rebuild_structures_for_id("signal_core"),
            vec!["signal_core", "inner_latch", "mirror_ward"]
        );
        assert_eq!(
            rts_garrison_units_for_id("central_keep"),
            vec!["mirror_guard_alpha", "signal_lancer", "field_engineer"]
        );
        assert_eq!(
            rts_open_world_route_tiles_for_id("league-coliseum"),
            vec!["13,3", "12,3", "11,3", "10,2", "9,2"]
        );
        assert_eq!(
            rts_open_world_panels_for_room("league-coliseum"),
            vec![
                "room_panel:league-coliseum",
                "task_panel:task-fixture-first-route",
                "combat_panel:league-coliseum",
                "save_panel:post_rts_restore"
            ]
        );
    }

    #[test]
    fn economy_tech_placement_adapter_preserves_first_contact_tiles() {
        assert_eq!(rts_siege_unit_tile_for_id("stonebreak_cart", 0), (9, 3));
        assert_eq!(rts_harvest_tile_for_node("gold_vein"), (3, 3));
        assert_eq!(rts_dropoff_tile_for_structure("town_hall"), (5, 5));
        assert_eq!(rts_build_site_tiles("7,4"), vec!["7,4", "7,5", "8,4"]);
        assert_eq!(rts_structure_tile_for_id("training_hall"), (4, 3));
        assert_eq!(rts_unlock_unit_tile_for_id("relay_guard"), (7, 5));
    }

    #[test]
    fn queue_economy_adapter_preserves_first_contact_rules() {
        let resource_spend_log = vec!["commit:1200g:prior_queue_pressure".to_string()];
        assert_eq!(rts_queue_gold_cost("build:watch_tower@7,4"), 210);
        assert_eq!(rts_queue_cost_label("harvest:gold_vein"), "-");
        assert_eq!(
            rts_log_gold_amount("commit:210g:build:watch_tower@7,4"),
            210
        );
        assert_eq!(rts_resource_gold_commitment(&resource_spend_log), 1200);
        assert_eq!(rts_available_gold(0, &resource_spend_log), 40);
        assert!(!rts_queue_is_affordable(
            0,
            &resource_spend_log,
            "build:watch_tower@7,4"
        ));
        assert!(rts_queue_requires_affordability_check(
            "build:watch_tower@7,4"
        ));
        assert!(!rts_queue_requires_affordability_check(
            "objective:claim_relay"
        ));
        assert_eq!(
            rts_build_parts("build:watch_tower@7,4"),
            ("watch_tower".to_string(), "7,4".to_string())
        );
        assert_eq!(
            rts_structure_parts("repair:watch_tower@7,4", "repair:", "7,4"),
            ("watch_tower".to_string(), "7,4".to_string())
        );
        assert_eq!(
            rts_tech_parts("upgrade:signal_blade", "upgrade:", "training_hall"),
            ("signal_blade".to_string(), "training_hall".to_string())
        );
        assert!(rts_queue_uses_production_lane("train:worker"));
        assert!(!rts_queue_uses_production_lane("build:watch_tower@7,4"));
        assert_eq!(
            rts_queue_feedback_chip("build:watch_tower@7,4"),
            "feedback:build_placed:watch_tower@7,4"
        );
        assert_eq!(
            rts_rejection_feedback_chip("RTS:QUEUE:build:watch_tower@7,4", "low_gold"),
            "feedback:blocked:queue:low_gold"
        );
        assert_eq!(
            rts_input_source_player_label("classic_rts_mouse_sidebar", "RTS:QUEUE:train:worker"),
            "SIDEBAR"
        );
        assert_eq!(
            rts_blocked_feedback_toast(
                "classic_rts_mouse_sidebar",
                "RTS:QUEUE:build:watch_tower@7,4",
                "rts_queue_unaffordable:build:watch_tower@7,4"
            ),
            "Input blocked: SIDEBAR QUEUE LOCK NEED 210G"
        );
        assert!(rts_should_emit_rejection_feedback_chip(
            "classic_rts_mouse_sidebar"
        ));
        assert!(!rts_should_emit_rejection_feedback_chip(
            "classic_rts_bot_executor"
        ));
        assert_eq!(
            rts_executable_command_queue_snapshot(&[
                "queue:train:worker".to_string(),
                "feedback:blocked:queue:rts_queue_unaffordable:build:watch_tower@7,4".to_string(),
            ]),
            vec!["queue:train:worker"]
        );
        assert!(rts_blocked_feedback_chip_visible(&[
            "queue:train:worker".to_string(),
            "feedback:blocked:queue:rts_queue_unaffordable:build:watch_tower@7,4".to_string(),
        ]));
        assert!(!rts_blocked_feedback_chip_visible(&[
            "queue:train:worker".to_string()
        ]));
        assert_eq!(
            rts_blocked_feedback_player_label(
                "feedback:blocked:queue:rts_queue_unaffordable:build:watch_tower@7,4"
            ),
            "QUEUE LOCK NEED 210G"
        );
    }

    #[test]
    fn command_feedback_adapter_preserves_first_contact_lifecycle() {
        let strip_queue = vec![
            "queued_group_order:Multi0:26:move:2actors".to_string(),
            "control_group_command_feedback_strip:group_27_override".to_string(),
        ];
        let strip_events =
            vec!["control_group_command_feedback_strip:group_28_filtered".to_string()];
        assert_eq!(
            rts_command_feedback_strip_stage(0, &strip_events, &strip_queue),
            Some("group_28_filtered")
        );
        assert_eq!(
            rts_command_surface_stage(
                0,
                &[
                    "surface:selection_state".to_string(),
                    "surface:target_queue".to_string(),
                ],
                &["surface:command_grid".to_string()]
            ),
            Some("target_queue")
        );
        assert_eq!(
            rts_command_surface_stage(2, &[], &["surface:command_grid".to_string()]),
            Some("cooldown_disabled")
        );
        assert_eq!(rts_command_surface_stage(1, &[], &[]), None);
        assert_eq!(
            rts_command_feedback_strip_stage(
                2,
                &[],
                &["control_group_command_feedback_strip:".into()]
            ),
            Some("group_28_formation")
        );
        assert_eq!(rts_command_feedback_strip_stage(1, &[], &[]), None);

        let lifecycle_events = vec!["control_group_command_feedback_lifecycle:dimmed".to_string()];
        let lifecycle_queue = vec![
            "control_group_command_history:dimmed_history_retained".to_string(),
            "history_row_pruned:25:old_queue:17,30:age16".to_string(),
        ];
        assert_eq!(
            rts_command_feedback_lifecycle_stage(
                "command_feedback_lifecycle:fresh",
                &lifecycle_events,
                &lifecycle_queue,
            ),
            Some("fresh")
        );
        assert_eq!(
            rts_command_feedback_lifecycle_stage("", &lifecycle_events, &lifecycle_queue),
            Some("dimmed")
        );
        assert!(rts_command_history_visible(
            "",
            &lifecycle_events,
            &lifecycle_queue,
        ));
        assert!(rts_command_history_prune_visible(
            "",
            &lifecycle_events,
            &lifecycle_queue,
        ));
        assert_eq!(
            rts_command_execution_feedback_kind(
                "idle",
                "move:line",
                "stable",
                true,
                "rally",
                true,
                false,
                &["feedback:rally_confirmed@8,4".to_string()],
            ),
            Some("move")
        );
        assert_eq!(
            rts_command_execution_feedback_kind(
                "following:player",
                "follow:player",
                "stable",
                false,
                "follow",
                false,
                false,
                &[],
            ),
            Some("follow")
        );
        assert_eq!(
            rts_command_execution_feedback_kind(
                "attack_move_advancing:forest_creep_camp",
                "attack_move:10,3",
                "stable",
                false,
                "attack_move",
                false,
                false,
                &[],
            ),
            Some("attack")
        );
        assert_eq!(
            rts_command_execution_feedback_kind(
                "idle",
                "queue",
                "harvesting:gold_vein",
                false,
                "harvest",
                false,
                false,
                &["harvest:gold_vein".to_string()],
            ),
            Some("harvest")
        );
    }

    #[test]
    fn overlay_stage_adapter_preserves_first_contact_feedback_states() {
        let unit_events = vec!["unit_status_portrait:commander".to_string()];
        assert_eq!(
            rts_unit_status_portrait_stage(4, &unit_events, &["unit_status_portrait:".to_string()],),
            Some("commander")
        );
        assert_eq!(
            rts_unit_status_portrait_stage(5, &[], &["unit_status_portrait:".to_string()]),
            Some("multi_select")
        );
        assert_eq!(rts_unit_status_portrait_stage(0, &[], &[]), None);
        assert_eq!(
            rts_unit_status_portrait_unit_id(
                "worker",
                &[
                    "player".to_string(),
                    "square_worker_carry".to_string(),
                    "square_guard_patrol".to_string(),
                ],
                Some("mirror_captain"),
                Some("arena_creep_attack"),
                &["training_hall".to_string()],
            ),
            "square_worker_carry"
        );
        assert_eq!(
            rts_unit_status_portrait_unit_id(
                "structure",
                &[],
                None,
                None,
                &["relay_outpost".to_string()],
            ),
            "relay_outpost"
        );
        assert_eq!(
            rts_unit_status_health_percent("structure", &[], &[76], 41),
            76
        );
        assert_eq!(
            rts_unit_status_health_percent("creep_target", &[], &[], 0),
            1
        );
        assert_eq!(rts_unit_status_energy_percent(&[32]), 68);
        assert_eq!(
            rts_unit_status_role_badges("commander"),
            ["AUR", "LVL", "CMD"]
        );

        assert_eq!(
            rts_selection_command_feedback_stage(
                0,
                &[],
                &["selection_command_feedback:attack_lock".to_string()],
            ),
            Some("attack_lock")
        );
        assert_eq!(
            rts_selection_command_feedback_stage(
                3,
                &[],
                &["selection_command_feedback:".to_string()],
            ),
            Some("move_line")
        );
        assert_eq!(rts_selection_command_feedback_stage(0, &[], &[]), None);

        assert_eq!(
            rts_ability_tooltip_telegraph_stage(
                0,
                &["ability_tooltip_telegraph:range_preview".to_string()],
                &[],
            ),
            Some("range_preview")
        );
        assert_eq!(
            rts_ability_tooltip_telegraph_stage(
                4,
                &[],
                &["ability_tooltip_telegraph:".to_string()],
            ),
            Some("queue_explain")
        );
        assert_eq!(rts_ability_tooltip_telegraph_stage(0, &[], &[]), None);

        assert_eq!(
            rts_control_group_hotkey_feedback_stage(
                0,
                &[],
                &["control_group_hotkey_feedback:double_tap_camera".to_string()],
            ),
            Some("double_tap_camera")
        );
        assert_eq!(
            rts_control_group_hotkey_feedback_stage(
                5,
                &[],
                &["control_group_hotkey_feedback:".to_string()],
            ),
            Some("ability_hotkey_ack")
        );
        assert_eq!(rts_control_group_hotkey_feedback_stage(0, &[], &[]), None);

        assert_eq!(
            rts_formation_move_preview_stage(
                &["formation_move_preview:commit_spacing".to_string()],
                &["formation_move_preview:destination_ghost".to_string()],
                0,
            ),
            Some("commit_spacing")
        );
        assert_eq!(
            rts_formation_move_preview_stage(&[], &["formation_move_preview:".to_string()], 3,),
            Some("collision_avoidance")
        );
        assert_eq!(rts_formation_move_preview_stage(&[], &[], 0), None);

        assert_eq!(
            rts_formation_move_execution_stage(
                &["formation_move_execution:arrival_lock".to_string()],
                &["formation_move_execution:slot_claim".to_string()],
                0,
            ),
            Some("arrival_lock")
        );
        assert_eq!(
            rts_formation_move_execution_stage(&[], &["formation_move_execution:".to_string()], 4,),
            Some("blocked_reroute")
        );
        assert_eq!(rts_formation_move_execution_stage(&[], &[], 0), None);

        assert_eq!(
            rts_local_obstruction_recovery_stage(
                &["local_obstruction_recovery:flow_resume".to_string()],
                &["local_obstruction_recovery:detect_block".to_string()],
                0,
            ),
            Some("flow_resume")
        );
        assert_eq!(
            rts_local_obstruction_recovery_stage(
                &[],
                &["local_obstruction_recovery:".to_string()],
                2,
            ),
            Some("side_step")
        );
        assert_eq!(rts_local_obstruction_recovery_stage(&[], &[], 0), None);
    }

    #[test]
    fn selection_roster_adapter_preserves_first_contact_rules() {
        assert_eq!(
            rts_default_group_units(),
            vec![
                "player",
                "square_guard_patrol",
                "square_worker_carry",
                "square_creep_wander"
            ]
        );
        assert_eq!(
            rts_group_two_units(),
            vec!["square_guard_patrol", "square_creep_wander"]
        );
        assert_eq!(rts_unit_selection_class("square_worker_carry"), "worker");
        assert_eq!(
            rts_same_class_units("player"),
            vec!["player", "square_guard_front", "square_guard_patrol"]
        );
        assert_eq!(rts_unit_allegiance("square_creep_wander"), "hostile");
        assert!(rts_unit_is_player_owned("square_worker_harvest"));
        assert_eq!(rts_unit_selection_priority("square_creep_wander"), 20);
        assert_eq!(
            rts_selectable_unit_tile("square_guard_patrol"),
            Some((7, 5))
        );
        assert_eq!(rts_selectable_unit_at_tile((5, 4)), Some("player"));
        assert_eq!(
            rts_selection_tiles_for_units(&[
                "player".to_string(),
                "square_guard_front".to_string(),
                "square_worker_carry".to_string()
            ]),
            vec!["5,4", "4,5"]
        );
        assert_eq!(rts_selection_box_tiles(), vec!["5,5", "6,5", "5,4", "6,4"]);
        assert_eq!(
            rts_drag_selection_parts("drag:5,4->9,5"),
            Some(((5, 4), (9, 5)))
        );
        assert_eq!(
            rts_selection_box_tiles_between((5, 4), (6, 5)),
            vec!["5,4", "6,4", "5,5", "6,5"]
        );
        assert_eq!(
            rts_drag_selected_units((4, 4), (8, 5)),
            vec![
                "player",
                "square_guard_front",
                "square_guard_patrol",
                "square_worker_carry",
                "square_worker_harvest"
            ]
        );
        assert_eq!(
            rts_drag_rejected_unit_ids((5, 4), (9, 5)),
            vec!["square_creep_wander"]
        );
    }

    #[test]
    fn control_group_roster_adapter_preserves_first_contact_slots() {
        let assignments = vec![
            "2:player|square_guard_patrol".to_string(),
            "10:camera:square_worker_carry|square_worker_harvest".to_string(),
        ];
        let active_group_ids = vec!["10".to_string()];

        assert_eq!(
            rts_control_group_hotkey_slot("assign:10", "assign:").as_deref(),
            Some("10")
        );
        assert_eq!(
            rts_default_units_for_control_group_slot("3"),
            vec!["square_worker_carry", "square_worker_harvest"]
        );
        assert_eq!(
            rts_units_from_control_group_assignment(&assignments, "10"),
            vec!["square_worker_carry", "square_worker_harvest"]
        );
        assert_eq!(rts_control_group_slot_label("10"), "0");
        assert_eq!(rts_control_group_slot_member_count(&assignments, "10"), 2);
        assert!(rts_control_group_slot_is_active(
            &active_group_ids,
            Some("2"),
            "10"
        ));

        let slot_ten = rts_control_group_slot_summaries(&assignments, &active_group_ids, Some("2"))
            .into_iter()
            .find(|summary| summary.slot == "10")
            .expect("slot 10 summary");
        assert_eq!(slot_ten.key_label, "0");
        assert_eq!(slot_ten.member_count, 2);
        assert!(slot_ten.occupied);
        assert!(slot_ten.active);
        assert_eq!(
            rts_merged_unit_ids(
                &["player".to_string()],
                &["player".to_string(), "square_worker_carry".to_string()],
            ),
            vec!["player", "square_worker_carry"]
        );
    }

    #[test]
    fn command_parts_adapter_preserves_first_contact_parsing() {
        assert_eq!(
            rts_selection_clear_parts("clear:hostile:square_creep_wander@9,4"),
            Some((
                "hostile".to_string(),
                Some("square_creep_wander".to_string()),
                "9,4".to_string()
            ))
        );
        assert_eq!(
            rts_move_command_parts("minimap:9,2:attack_move"),
            ("9,2", "attack_move")
        );
        assert_eq!(
            rts_line_path_tiles((5, 5), (8, 3)),
            vec!["6,5", "7,4", "8,3"]
        );
        assert_eq!(
            rts_focus_fire_units_for_target("enemy_barracks"),
            vec![
                "relay_guard_alpha",
                "relay_guard_beta",
                "wayfinder_scout",
                "field_mender"
            ]
        );
        assert_eq!(
            rts_creep_camp_units_for_id("forest_creep_camp"),
            vec!["forest_alpha_creep", "forest_stalker", "forest_shaman"]
        );
        assert_eq!(
            rts_objective_parts("claim:relay_beacon@9,2"),
            (
                "claim".to_string(),
                "relay_beacon".to_string(),
                "9,2".to_string()
            )
        );
        assert_eq!(
            rts_creep_camp_parts("camp", "clear:creep_camp@8,3"),
            (
                "clear".to_string(),
                "forest_creep_camp".to_string(),
                "8,3".to_string()
            )
        );
        assert_eq!(
            rts_recon_parts("mark:scout_enemy_base@10,2"),
            (
                "mark".to_string(),
                "enemy_base".to_string(),
                "10,2".to_string()
            )
        );
        assert_eq!(
            rts_enemy_command_parts("pressure:counter_wave@enemy_gate", "pressure", "enemy_base"),
            (
                "pressure".to_string(),
                "counter_wave".to_string(),
                "enemy_gate".to_string()
            )
        );
        assert_eq!(
            rts_counter_command_parts("upgrade:signal_blade@training_hall"),
            (
                "upgrade".to_string(),
                "signal_blade".to_string(),
                "training_hall".to_string()
            )
        );
        assert_eq!(
            rts_army_command_parts("train:mixed_vanguard@training_hall"),
            (
                "train".to_string(),
                "mixed_vanguard".to_string(),
                "training_hall".to_string()
            )
        );
        assert_eq!(
            rts_base_assault_parts("breach:enemy_barracks@10,3"),
            (
                "breach".to_string(),
                "enemy_barracks".to_string(),
                "10,3".to_string()
            )
        );
        assert_eq!(
            rts_aftermath_parts("destroy:enemy_barracks@10,3"),
            (
                "destroy".to_string(),
                "enemy_barracks".to_string(),
                "10,3".to_string()
            )
        );
        assert_eq!(
            rts_commander_parts("level:mirror_captain@forest_relay"),
            (
                "level".to_string(),
                "mirror_captain".to_string(),
                "forest_relay".to_string()
            )
        );
        assert_eq!(
            rts_expansion_parts("claim:forest_relay@9,2"),
            (
                "claim".to_string(),
                "forest_relay".to_string(),
                "9,2".to_string()
            )
        );
        assert_eq!(
            rts_tier_two_parts("tech:stonebreak_cart@relay_outpost"),
            (
                "tech".to_string(),
                "stonebreak_cart".to_string(),
                "relay_outpost".to_string()
            )
        );
    }

    #[test]
    fn hover_cursor_adapter_preserves_first_contact_affordances() {
        assert_eq!(
            rts_hover_target_preview_kind("viewport_attack_target"),
            Some("attack")
        );
        assert_eq!(
            rts_hover_target_preview_kind("viewport_harvest"),
            Some("harvest")
        );
        assert_eq!(
            rts_hover_target_preview_kind("viewport_follow"),
            Some("follow")
        );
        assert_eq!(
            rts_cursor_kind_for_hover_preview(true, "command_button", "RTS:ABILITY:focus_fire"),
            "ability"
        );
        assert_eq!(
            rts_cursor_label_for_hover_preview(
                "classic_rts_mouse_command_bar",
                "RTS:ABILITY:focus_fire",
                true,
                "ability"
            ),
            "COMMAND BAR CURSOR ABILITY READY"
        );
        assert_eq!(
            rts_cursor_kind_for_hover_preview(false, "viewport_move", "RTS:MOVE:4,3:line"),
            "blocked"
        );
        assert_eq!(
            rts_cursor_label_for_hover_preview(
                "classic_rts_mouse_viewport",
                "RTS:MOVE:4,3:line",
                false,
                "blocked"
            ),
            "MAP CURSOR BLOCKED LOCK"
        );
        assert_eq!(
            rts_hover_player_label(
                "classic_rts_mouse_sidebar",
                "RTS:QUEUE:build:watch_tower@7,4",
                None,
                Some("build:watch_tower@7,4"),
                "sidebar_build_queue",
                true,
                "ok",
            ),
            "SIDEBAR QUEUE READY WATCH TOWER 7,4 210G"
        );
        assert_eq!(
            rts_hover_player_label(
                "classic_rts_mouse_viewport",
                "RTS:MOVE:4,3:line",
                Some("4,3"),
                None,
                "viewport_move",
                true,
                "ok",
            ),
            "MAP MOVE READY 4,3"
        );
        assert_eq!(
            rts_hover_player_label(
                "classic_rts_mouse_viewport",
                "RTS:MOVE:6,5:follow:square_guard_patrol",
                Some("6,5"),
                None,
                "viewport_follow",
                true,
                "ok",
            ),
            "MAP FOLLOW READY SQUARE GUARD PATRO"
        );
        assert_eq!(
            rts_hover_player_label(
                "classic_rts_mouse_viewport",
                "RTS:MOVE:4,3:line",
                Some("4,3"),
                None,
                "viewport_move",
                false,
                "rts_group_selection_required",
            ),
            "MAP MOVE LOCK SELECT UNITS"
        );
    }

    #[test]
    fn command_stamp_adapter_preserves_first_contact_feedback_labels() {
        let selection_stamp = rts_command_stamp_for_selection("classic_rts_hotkey", "assign:5", 2);
        assert_eq!(selection_stamp.kind, "control-group");
        assert_eq!(selection_stamp.target_id.as_deref(), Some("5"));
        assert_eq!(
            selection_stamp.player_label,
            "HOTKEY GROUP 5 ASSIGNED 2 UNITS"
        );

        let move_stamp = rts_command_stamp_for_move("classic_rts_mouse_viewport", "7,4:line")
            .expect("valid move tile stamp");
        assert_eq!(move_stamp.kind, "move");
        assert_eq!(move_stamp.tile_id.as_deref(), Some("7,4"));
        assert_eq!(move_stamp.player_label, "MAP MOVE SENT 7,4");

        let ability_stamp = rts_command_stamp_for_ability(
            "classic_rts_mouse_command_bar",
            "focus_fire",
            Some("arena_creep_attack"),
        );
        assert_eq!(ability_stamp.kind, "ability");
        assert_eq!(ability_stamp.tile_id.as_deref(), Some("6,5"));
        assert_eq!(
            ability_stamp.target_id.as_deref(),
            Some("arena_creep_attack")
        );
        assert_eq!(
            ability_stamp.player_label,
            "COMMAND BAR ABILITY SENT FOCUS FIRE"
        );
    }

    #[test]
    fn scripted_demo_timeline_adapter_preserves_queue_cancel_sequence() {
        assert!(rts_scripted_demo_pauses_queue_tick("queue_cancel_refund"));
        assert!(rts_scripted_demo_pauses_queue_tick(
            "queue_cancel_refund_sequence"
        ));
        assert!(!rts_scripted_demo_pauses_queue_tick("live_player_flow"));
        assert_eq!(
            rts_scripted_demo_stage_from_frame("queue_cancel_refund_sequence", 0),
            Some(0)
        );
        assert_eq!(
            rts_scripted_demo_stage_from_frame("queue_cancel_refund_sequence", 60),
            Some(1)
        );
        assert_eq!(
            rts_scripted_demo_stage_from_frame("queue_cancel_refund_sequence", 240),
            Some(4)
        );
        assert_eq!(
            rts_scripted_demo_stage_from_frame("queue_cancel_refund_sequence", 300),
            Some(0)
        );
        assert_eq!(
            rts_scripted_demo_stage_from_frame("queue_cancel_refund", 60),
            None
        );
        assert_eq!(rts_scripted_demo_stage_id(3), "cancel_refund");
        assert_eq!(rts_scripted_demo_stage_title(4), "WORKER QUEUED");
    }

    #[test]
    fn scene_stage_adapter_preserves_first_contact_event_precedence() {
        assert_eq!(
            rts_npc_behavior_stage(
                &["behavior:creep_retreat".to_string()],
                &["behavior:guard_patrol".to_string()],
                0,
            ),
            Some("creep_retreat")
        );
        assert_eq!(
            rts_combat_impact_stage(&[], &["impact:damage_tick".to_string()], 1),
            Some("damage_tick")
        );
        assert_eq!(
            rts_locomotion_blend_stage(&[], &["locomotion:cycle".to_string()], 5),
            Some("arrival_brake")
        );
        assert_eq!(
            rts_npc_transition_stage(
                &["transition:hit_recover".to_string()],
                &["transition:alert_turn".to_string()],
                0,
            ),
            Some("hit_recover")
        );
        assert_eq!(
            rts_depth_readability_stage(&[], &["depth:cycle".to_string()], 4),
            Some("path_occlusion")
        );
        assert_eq!(
            rts_structure_modeling_stage(
                &["structure:repair_beam".to_string()],
                &["structure:foundation_shadow".to_string()],
                0,
            ),
            Some("repair_beam")
        );
        assert_eq!(
            rts_environment_life_stage(&[], &["environment:cycle".to_string()], 4),
            Some("resource_glint")
        );
        assert_eq!(
            rts_worker_harvest_animation_stage(
                &["harvest_anim:return_path".to_string()],
                &["harvest_anim:approach".to_string()],
                0,
            ),
            Some("return_path")
        );
        assert_eq!(
            rts_production_spawn_animation_stage(
                &["production_spawn_anim:supply_flash".to_string()],
                &["production_spawn_anim:queue_pulse".to_string()],
                0,
            ),
            Some("supply_flash")
        );
        assert_eq!(rts_npc_behavior_stage(&[], &[], 0), None);
        assert_eq!(rts_structure_modeling_stage(&[], &[], 0), None);
        assert_eq!(rts_environment_life_stage(&[], &[], 0), None);
        assert_eq!(rts_worker_harvest_animation_stage(&[], &[], 0), None);
        assert_eq!(rts_production_spawn_animation_stage(&[], &[], 0), None);
    }

    #[test]
    fn action_cadence_adapter_preserves_first_contact_marks() {
        let guard_attack = rts_action_cadence_marks("actor_guard_attack");
        assert_eq!(guard_attack.len(), 22);
        assert_eq!(
            guard_attack
                .iter()
                .filter(|mark| mark.kind == "windup")
                .count(),
            5
        );
        assert_eq!(
            guard_attack
                .iter()
                .filter(|mark| mark.kind == "strike")
                .count(),
            9
        );
        assert_eq!(
            guard_attack
                .iter()
                .filter(|mark| mark.kind == "recovery")
                .count(),
            6
        );
        assert_eq!(
            guard_attack
                .iter()
                .filter(|mark| mark.kind == "shadow_smear")
                .count(),
            2
        );

        let creep_attack = rts_action_cadence_marks("actor_creep_attack");
        assert_eq!(creep_attack.first().map(|mark| mark.rect.x), Some(-24));
        assert_eq!(guard_attack.first().map(|mark| mark.rect.x), Some(-22));

        let worker_carry = rts_action_cadence_marks("actor_worker_carry");
        assert_eq!(worker_carry.len(), 8);
        assert_eq!(
            worker_carry
                .iter()
                .filter(|mark| mark.kind == "carry_bob")
                .count(),
            4
        );

        let guard_idle = rts_action_cadence_marks("actor_guard_idle");
        assert_eq!(guard_idle.len(), 4);
        assert!(guard_idle.iter().all(|mark| mark.kind == "idle_breath"));
        assert!(rts_action_cadence_marks("actor_player_idle_south").is_empty());
    }

    #[test]
    fn action_sequence_adapter_preserves_phase_and_marks() {
        assert_eq!(
            rts_action_sequence_phase(
                "actor_guard_attack",
                &["sequence:recovery".to_string()],
                &["sequence:windup".to_string()],
                2,
                2,
                true,
            ),
            Some("recovery")
        );
        assert_eq!(
            rts_action_sequence_phase(
                "actor_guard_attack",
                &[],
                &["sequence:cycle".to_string()],
                1,
                1,
                true,
            ),
            Some("windup")
        );
        assert_eq!(
            rts_action_sequence_phase(
                "actor_guard_attack",
                &[],
                &["sequence:cycle".to_string()],
                1,
                2,
                true,
            ),
            Some("strike")
        );
        assert_eq!(
            rts_action_sequence_phase(
                "actor_worker_carry",
                &[],
                &["sequence:cycle".to_string()],
                2,
                0,
                true,
            ),
            Some("carry_up")
        );
        assert_eq!(
            rts_action_sequence_phase(
                "actor_worker_carry",
                &[],
                &["sequence:cycle".to_string()],
                1,
                0,
                true,
            ),
            Some("carry_down")
        );
        assert_eq!(
            rts_action_sequence_phase("actor_guard_attack", &[], &[], 1, 2, true),
            None
        );
        assert_eq!(
            rts_action_sequence_phase("actor_guard_attack", &[], &[], 1, 2, false),
            Some("strike")
        );

        let windup = rts_action_sequence_marks("actor_guard_attack", "windup");
        assert_eq!(windup.len(), 9);
        assert_eq!(
            windup.first().map(|mark| mark.kind.as_str()),
            Some("frame_ghost")
        );
        assert_eq!(
            windup.iter().filter(|mark| mark.kind == "windup").count(),
            8
        );

        let strike = rts_action_sequence_marks("actor_guard_attack", "strike");
        assert_eq!(strike.len(), 12);
        assert_eq!(
            strike.iter().filter(|mark| mark.kind == "strike").count(),
            11
        );

        let carry_down = rts_action_sequence_marks("actor_worker_carry", "carry_down");
        assert_eq!(carry_down.len(), 5);
        assert_eq!(
            carry_down
                .iter()
                .filter(|mark| mark.kind == "carry_down")
                .count(),
            4
        );

        let idle = rts_action_sequence_marks("actor_guard_idle", "idle");
        assert_eq!(idle.len(), 6);
        assert_eq!(idle.iter().filter(|mark| mark.kind == "idle").count(), 5);

        assert!(rts_action_sequence_marks("actor_player_idle_south", "idle").is_empty());
    }

    #[test]
    fn unit_model_depth_adapter_preserves_role_marks() {
        let guard = rts_unit_model_depth_marks("actor_guard_attack");
        assert_eq!(guard.len(), 8);
        assert_eq!(guard.iter().filter(|mark| mark.kind == "rim").count(), 2);
        assert_eq!(guard.iter().filter(|mark| mark.kind == "armor").count(), 2);
        assert_eq!(
            guard
                .iter()
                .find(|mark| mark.kind == "face_shade")
                .map(|mark| mark.rect.y),
            Some(-32)
        );

        let worker = rts_unit_model_depth_marks("actor_worker_carry");
        assert_eq!(worker.len(), 8);
        assert_eq!(
            worker
                .iter()
                .filter(|mark| mark.kind == "layer_shadow")
                .count(),
            2
        );
        assert_eq!(
            worker
                .iter()
                .find(|mark| mark.kind == "role_prop")
                .map(|mark| mark.rect.x),
            Some(-15)
        );

        let creep = rts_unit_model_depth_marks("actor_creep_attack");
        assert_eq!(creep.len(), 8);
        assert_eq!(
            creep.iter().filter(|mark| mark.kind == "role_prop").count(),
            2
        );
        assert_eq!(
            creep
                .iter()
                .filter(|mark| mark.kind == "armor")
                .map(|mark| mark.rect.width)
                .next(),
            Some(22)
        );

        assert!(rts_unit_model_depth_marks("actor_player_idle_south").is_empty());
    }
}
