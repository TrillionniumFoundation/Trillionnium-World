use super::{
    asset_loader::{register_first_contact_atlases, FirstContactAtlasManifest},
    map_loader::FirstContactMap,
};
use bevy::prelude::*;

#[derive(Component)]
pub(super) struct FirstContactCamera;

#[derive(Component)]
pub(super) struct FirstContactTerrainTile;

#[derive(Component)]
pub(super) struct FirstContactUnitSprite {
    pub id: String,
    pub family: String,
    pub owner: String,
    pub selected: bool,
}

#[derive(Component)]
pub(super) struct FirstContactSelectionRing {
    pub unit_id: String,
}

#[derive(Component)]
pub(super) struct FirstContactStructureSprite {
    pub family: String,
    pub active: bool,
}

#[derive(Component)]
pub(super) struct FirstContactObjectivePulse;

pub(super) fn map_world_position(map: &FirstContactMap, x: i32, y: i32, z: f32) -> Vec3 {
    let tile = map.tile_size as f32;
    let left = -(map.width as f32 * tile) * 0.5 + tile * 0.5;
    let top = (map.height as f32 * tile) * 0.5 - tile * 0.5;
    let height = map.height_at(x.max(0) as usize, y.max(0) as usize) as f32;
    Vec3::new(
        left + x as f32 * tile,
        top - y as f32 * tile + height * 4.0,
        z,
    )
}

pub(super) fn atlas_sprite(
    image: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
    index: usize,
    size: Vec2,
) -> Sprite {
    Sprite {
        image,
        texture_atlas: Some(TextureAtlas { layout, index }),
        custom_size: Some(size),
        ..default()
    }
}

fn terrain_frame_name(key: char, alternate: bool) -> &'static str {
    match (key, alternate) {
        ('g', false) => "moss_a",
        ('g', true) => "moss_b",
        ('r', false) => "route_a",
        ('r', true) => "route_b",
        ('b', false) => "basalt_a",
        ('b', true) => "basalt_b",
        ('w', false) => "water_a",
        ('w', true) => "water_b",
        _ => "moss_a",
    }
}

fn transition_frame_name(map: &FirstContactMap, x: usize, y: usize) -> Option<&'static str> {
    if !matches!(map.terrain_at(x, y), Some('g' | 'r')) {
        return None;
    }
    let basalt = |x: isize, y: isize| {
        if x < 0 || y < 0 {
            return false;
        }
        map.terrain_at(x as usize, y as usize) == Some('b')
    };
    let x = x as isize;
    let y = y as isize;
    let west = basalt(x - 1, y);
    let east = basalt(x + 1, y);
    let north = basalt(x, y - 1);
    let south = basalt(x, y + 1);
    if west && north {
        Some("moss_basalt_nw")
    } else if east && north {
        Some("moss_basalt_ne")
    } else if west && south {
        Some("moss_basalt_sw")
    } else if east && south {
        Some("moss_basalt_se")
    } else if west {
        Some("moss_basalt_west")
    } else if east {
        Some("moss_basalt_east")
    } else if north {
        Some("moss_basalt_north")
    } else if south {
        Some("moss_basalt_south")
    } else {
        None
    }
}

pub(super) fn spawn_first_contact_live_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    map: Res<FirstContactMap>,
    manifest: Res<FirstContactAtlasManifest>,
) {
    let handles = register_first_contact_atlases(&asset_server, &mut layouts, &manifest);
    commands.insert_resource(handles.clone());

    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 1.20,
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(0.0, -55.0, 0.0),
        FirstContactCamera,
    ));

    let tile_size = map.tile_size as f32;
    for y in 0..map.height as usize {
        for x in 0..map.width as usize {
            let key = map.terrain_at(x, y).unwrap_or('g');
            let frame_name = terrain_frame_name(key, (x + y) % 2 == 1);
            let frame = *manifest
                .terrain_frames
                .get(frame_name)
                .expect("validated terrain frame exists");
            let height = map.height_at(x, y) as f32;
            commands.spawn((
                atlas_sprite(
                    handles.world_image.clone(),
                    handles.world_layout.clone(),
                    frame,
                    Vec2::splat(tile_size + 0.75),
                ),
                Transform::from_translation(map_world_position(
                    &map,
                    x as i32,
                    y as i32,
                    -20.0 + height * 0.05,
                )),
                FirstContactTerrainTile,
            ));
            if let Some(transition_name) = transition_frame_name(&map, x, y) {
                let transition = *manifest
                    .terrain_frames
                    .get(transition_name)
                    .expect("validated transition frame exists");
                commands.spawn((
                    atlas_sprite(
                        handles.world_image.clone(),
                        handles.world_layout.clone(),
                        transition,
                        Vec2::splat(tile_size + 0.75),
                    ),
                    Transform::from_translation(map_world_position(
                        &map, x as i32, y as i32, -19.8,
                    )),
                ));
            }
            if height >= 2.0
                && y + 1 < map.height as usize
                && map.height_at(x, y + 1) < 2
                && x % 2 == 0
            {
                let detail = *manifest
                    .detail_frames
                    .get(&format!("cliff_high_{}", ['a', 'b', 'c', 'd'][x % 4]))
                    .expect("validated cliff frame exists");
                let mut position = map_world_position(&map, x as i32, y as i32, -10.0);
                position.y -= tile_size * 0.35;
                commands.spawn((
                    atlas_sprite(
                        handles.world_image.clone(),
                        handles.world_layout.clone(),
                        detail,
                        Vec2::splat(tile_size * 1.5),
                    ),
                    Transform::from_translation(position),
                ));
            }
        }
    }

    for (index, chokepoint) in map.chokepoints.iter().enumerate() {
        let frame = manifest.detail_frames[if index % 2 == 0 {
            "choke_gate_a"
        } else {
            "choke_gate_b"
        }];
        let x = chokepoint.x + chokepoint.width as i32 / 2;
        let y = chokepoint.y + chokepoint.height as i32 / 2;
        commands.spawn((
            atlas_sprite(
                handles.world_image.clone(),
                handles.world_layout.clone(),
                frame,
                Vec2::new(tile_size * 2.2, tile_size * 1.25),
            ),
            Transform::from_translation(map_world_position(&map, x, y, 1.0)),
        ));
    }

    for resource in &map.resources {
        let frame = manifest.detail_frames[resource.kind.as_str()];
        commands.spawn((
            atlas_sprite(
                handles.world_image.clone(),
                handles.world_layout.clone(),
                frame,
                Vec2::splat(tile_size * 1.9),
            ),
            Transform::from_translation(map_world_position(&map, resource.x, resource.y, 3.0)),
        ));
    }

    for landmark in &map.landmarks {
        let frames = manifest
            .landmark_frames
            .get(&landmark.frame)
            .expect("validated landmark frame exists");
        let mut entity = commands.spawn((
            atlas_sprite(
                handles.world_image.clone(),
                handles.world_layout.clone(),
                frames[0],
                Vec2::splat(tile_size * 2.35),
            ),
            Transform::from_translation(map_world_position(&map, landmark.x, landmark.y, 4.0)),
        ));
        if landmark.id == map.objective.id {
            entity.insert(FirstContactObjectivePulse);
        }
    }

    for structure in &map.structures {
        let family = manifest
            .structure(&structure.family)
            .expect("map structure family exists in atlas");
        let frame = if structure.active {
            family.active
        } else {
            family.idle
        };
        let mut sprite = atlas_sprite(
            handles.world_image.clone(),
            handles.world_layout.clone(),
            frame,
            Vec2::splat(tile_size * 2.75),
        );
        if structure.owner == "contact" {
            sprite.color = Color::srgb(1.0, 0.76, 0.72);
        }
        commands.spawn((
            sprite,
            Transform::from_translation(map_world_position(&map, structure.x, structure.y, 5.0)),
            FirstContactStructureSprite {
                family: structure.family.clone(),
                active: structure.active,
            },
        ));
    }

    let ring_frame = manifest.effect_frames["selection_ring"];
    for unit in &map.units {
        let family = manifest
            .unit(&unit.family)
            .expect("map unit family exists in atlas");
        let position = map_world_position(&map, unit.x, unit.y, 8.0);
        if unit.selected {
            commands.spawn((
                atlas_sprite(
                    handles.world_image.clone(),
                    handles.world_layout.clone(),
                    ring_frame,
                    Vec2::splat(tile_size * 1.7),
                ),
                Transform::from_translation(position + Vec3::new(0.0, -3.0, -0.5)),
                FirstContactSelectionRing {
                    unit_id: unit.id.clone(),
                },
            ));
        }
        let mut sprite = atlas_sprite(
            handles.units_image.clone(),
            handles.units_layout.clone(),
            family.atlas_index(family.idle[0]),
            Vec2::splat(tile_size * 2.1),
        );
        if unit.owner == "contact" {
            sprite.color = Color::srgb(1.0, 0.74, 0.70);
        }
        commands.spawn((
            sprite,
            Transform::from_translation(position),
            FirstContactUnitSprite {
                id: unit.id.clone(),
                family: unit.family.clone(),
                owner: unit.owner.clone(),
                selected: unit.selected,
            },
        ));
    }

    let capture_frame = manifest.effect_frames["capture_pulse"];
    commands.spawn((
        atlas_sprite(
            handles.world_image.clone(),
            handles.world_layout.clone(),
            capture_frame,
            Vec2::splat(tile_size * 2.25),
        ),
        Transform::from_translation(map_world_position(
            &map,
            map.objective.x,
            map.objective.y,
            7.0,
        )),
        FirstContactObjectivePulse,
    ));
}
