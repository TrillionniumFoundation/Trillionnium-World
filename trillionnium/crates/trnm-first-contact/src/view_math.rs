use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewportSpec {
    pub map_pixels: Vec2,
    pub viewport_pixels: Vec2,
    pub projection_scale: f32,
}

impl ViewportSpec {
    pub fn new(
        map_width: u32,
        map_height: u32,
        tile_size: u32,
        viewport_pixels: Vec2,
        projection_scale: f32,
    ) -> Self {
        Self {
            map_pixels: Vec2::new(
                map_width as f32 * tile_size as f32,
                map_height as f32 * tile_size as f32,
            ),
            viewport_pixels,
            projection_scale,
        }
    }

    pub fn clamp_camera(self, requested: Vec2) -> Vec2 {
        let map_half = self.map_pixels * 0.5;
        let viewport_half = self.viewport_pixels * self.projection_scale * 0.5;
        let limit = (map_half - viewport_half).max(Vec2::ZERO);
        requested.clamp(-limit, limit)
    }
}

pub fn world_to_minimap(world: Vec2, map_pixels: Vec2, panel_pixels: Vec2) -> Vec2 {
    let normalized = (world / map_pixels + Vec2::splat(0.5)).clamp(Vec2::ZERO, Vec2::ONE);
    normalized * panel_pixels
}

pub fn minimap_to_tile(local: Vec2, panel_pixels: Vec2, map_width: u32, map_height: u32) -> IVec2 {
    let normalized = (local / panel_pixels).clamp(Vec2::ZERO, Vec2::ONE);
    IVec2::new(
        (normalized.x * (map_width.saturating_sub(1)) as f32).round() as i32,
        ((1.0 - normalized.y) * (map_height.saturating_sub(1)) as f32).round() as i32,
    )
}

pub fn points_in_drag_rect(start: Vec2, end: Vec2, points: &[Vec2]) -> Vec<usize> {
    let minimum = start.min(end);
    let maximum = start.max(end);
    points
        .iter()
        .enumerate()
        .filter_map(|(index, point)| {
            (point.x >= minimum.x
                && point.x <= maximum.x
                && point.y >= minimum.y
                && point.y <= maximum.y)
                .then_some(index)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_minimap_and_drag_are_map_parameterized() {
        let spec = ViewportSpec::new(40, 24, 32, Vec2::new(1280.0, 720.0), 0.74);
        let clamped = spec.clamp_camera(Vec2::splat(9_999.0));
        assert!(clamped.x < spec.map_pixels.x * 0.5);
        assert!(clamped.y < spec.map_pixels.y * 0.5);
        let mini = world_to_minimap(Vec2::ZERO, spec.map_pixels, Vec2::new(164.0, 98.0));
        assert_eq!(mini, Vec2::new(82.0, 49.0));
        assert_eq!(
            minimap_to_tile(mini, Vec2::new(164.0, 98.0), 40, 24),
            IVec2::new(20, 12)
        );
        assert_eq!(
            points_in_drag_rect(
                Vec2::ZERO,
                Vec2::new(10.0, 10.0),
                &[Vec2::new(4.0, 4.0), Vec2::new(14.0, 4.0)]
            ),
            vec![0]
        );
    }
}
