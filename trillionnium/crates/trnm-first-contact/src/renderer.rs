use super::{
    asset_loader::{register_first_contact_atlases, FirstContactAtlasManifest},
    map_loader::FirstContactMap,
    simulation_adapter::FirstContactTransient,
};
use crate::view_math::ViewportSpec;
use bevy::prelude::*;

pub(super) const FIRST_CONTACT_CAMERA_SCALE: f32 = 0.74;

#[derive(Component)]
pub(super) struct FirstContactCamera;

/// Marks every top-level visual whose lifetime is tied to the authored map.
///
/// Runtime units and structures predate this marker, so the map replacement
/// system also removes those component types explicitly. New authored visuals
/// should always carry this marker so a map change cannot leave decorations or
/// effects from the previous battlefield behind.
#[derive(Component)]
pub(super) struct FirstContactMapScoped;

#[derive(Component)]
pub(super) struct FirstContactUnitSprite {
    pub id: String,
    pub family: String,
    pub owner: String,
}

#[derive(Component)]
pub(super) struct FirstContactSelectionRing {
    pub unit_id: String,
}

#[derive(Component)]
pub(super) struct FirstContactStructureSprite {
    pub id: String,
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

fn authored_camera_translation(map: &FirstContactMap, viewport_size: Vec2) -> Vec3 {
    let authored = map_world_position(map, map.camera_start.x, map.camera_start.y, 0.0);
    let clamped = ViewportSpec::new(
        map.width,
        map.height,
        map.tile_size,
        viewport_size,
        FIRST_CONTACT_CAMERA_SCALE,
    )
    .clamp_camera(authored.truncate());
    clamped.extend(authored.z)
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

#[derive(Component, Debug, Clone, Copy)]
pub(super) struct IdentityMotion {
    base_translation: Vec3,
    base_angle: f32,
    phase: f32,
    frequency: f32,
    amplitude: f32,
}

pub(super) fn attach_identity_geometry(
    commands: &mut Commands,
    parent: Entity,
    family_id: &str,
    tile_size: f32,
    hostile: bool,
) {
    let signature = family_id.bytes().fold(2_166_136_261_u32, |hash, byte| {
        (hash ^ u32::from(byte)).wrapping_mul(16_777_619)
    });
    let width = tile_size * (0.18 + (signature & 7) as f32 * 0.025);
    let height = tile_size * (0.08 + ((signature >> 3) & 3) as f32 * 0.025);
    let offset_x = (((signature >> 5) & 7) as f32 - 3.5) * tile_size * 0.035;
    let offset_y = tile_size * (0.26 + ((signature >> 8) & 3) as f32 * 0.04);
    let angle = (((signature >> 10) & 15) as f32 - 7.5) * 0.075;
    let accent = if hostile {
        Color::srgb(1.0, 0.35 + ((signature >> 14) & 3) as f32 * 0.08, 0.22)
    } else {
        Color::srgb(0.25, 0.72 + ((signature >> 14) & 3) as f32 * 0.07, 0.92)
    };
    commands.entity(parent).with_children(|children| {
        let first_translation = Vec3::new(offset_x, offset_y, 0.4);
        children.spawn((
            Sprite::from_color(accent, Vec2::new(width, height)),
            Transform::from_translation(first_translation)
                .with_rotation(Quat::from_rotation_z(angle)),
            IdentityMotion {
                base_translation: first_translation,
                base_angle: angle,
                phase: (signature & 255) as f32 * 0.024,
                frequency: 1.6 + ((signature >> 16) & 7) as f32 * 0.18,
                amplitude: tile_size * (0.012 + ((signature >> 20) & 3) as f32 * 0.004),
            },
        ));
        let second_translation = Vec3::new(-offset_x * 0.6, -offset_y * 0.42, 0.35);
        children.spawn((
            Sprite::from_color(
                Color::srgba(0.04, 0.07, 0.08, 0.95),
                Vec2::new(height * 0.65, width * 0.72),
            ),
            Transform::from_translation(second_translation)
                .with_rotation(Quat::from_rotation_z(-angle * 0.7)),
            IdentityMotion {
                base_translation: second_translation,
                base_angle: -angle * 0.7,
                phase: (signature & 127) as f32 * 0.031 + 1.4,
                frequency: 1.2 + ((signature >> 23) & 7) as f32 * 0.16,
                amplitude: tile_size * (0.009 + ((signature >> 26) & 3) as f32 * 0.003),
            },
        ));
    });
}

pub(super) fn animate_identity_geometry(
    time: Res<Time>,
    mut identities: Query<(&IdentityMotion, &mut Transform)>,
) {
    let seconds = time.elapsed_secs();
    for (motion, mut transform) in &mut identities {
        let wave = (seconds * motion.frequency + motion.phase).sin();
        let counter = (seconds * (motion.frequency * 0.53) + motion.phase).cos();
        transform.translation = motion.base_translation
            + Vec3::new(
                counter * motion.amplitude * 0.45,
                wave * motion.amplitude,
                0.0,
            );
        transform.rotation = Quat::from_rotation_z(motion.base_angle + wave * 0.045);
        transform.scale = Vec3::new(1.0 + counter * 0.025, 1.0 + wave.abs() * 0.035, 1.0);
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

pub(super) fn prepare_first_contact_live_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    windows: Query<&Window>,
    map: Res<FirstContactMap>,
    manifest: Res<FirstContactAtlasManifest>,
) {
    let handles = register_first_contact_atlases(&asset_server, &mut layouts, &manifest);
    commands.insert_resource(handles.clone());

    let viewport_size = windows
        .single()
        .map(|window| Vec2::new(window.width(), window.height()))
        .unwrap_or(Vec2::new(1280.0, 720.0));

    commands.spawn((
        Camera2d,
        // This client is authored as nearest-neighbour pixel art. Four-sample
        // MSAA adds no useful detail, but multiplies the render-target cost and
        // pushed software/X11 fallback rendering above 250 ms per frame.
        Msaa::Off,
        Projection::Orthographic(OrthographicProjection {
            scale: FIRST_CONTACT_CAMERA_SCALE,
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_translation(authored_camera_translation(&map, viewport_size)),
        FirstContactCamera,
    ));
}

pub(super) fn spawn_first_contact_authored_map(
    mut commands: Commands,
    map: Res<FirstContactMap>,
    manifest: Res<FirstContactAtlasManifest>,
    handles: Res<super::asset_loader::FirstContactAtlasHandles>,
) {
    if !map.is_changed() {
        return;
    }
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
                FirstContactMapScoped,
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
                    FirstContactMapScoped,
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
                    FirstContactMapScoped,
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
            FirstContactMapScoped,
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
            FirstContactMapScoped,
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
            FirstContactMapScoped,
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
        let (image, layout, frame) = family.identity_frame.map_or_else(
            || {
                (
                    handles.world_image.clone(),
                    handles.world_layout.clone(),
                    frame,
                )
            },
            |identity| {
                (
                    handles.identities_image.clone(),
                    handles.identities_layout.clone(),
                    identity,
                )
            },
        );
        let mut sprite = atlas_sprite(
            image,
            layout,
            frame,
            Vec2::new(
                tile_size * 2.75 * family.silhouette_scale[0],
                tile_size * 2.75 * family.silhouette_scale[1],
            ),
        );
        sprite.color = Color::srgb(family.tint[0], family.tint[1], family.tint[2]);
        if structure.owner == "contact" && family.tint == [1.0, 1.0, 1.0] {
            sprite.color = Color::srgb(1.0, 0.76, 0.72);
        }
        let entity = commands
            .spawn((
                sprite,
                Transform::from_translation(map_world_position(
                    &map,
                    structure.x,
                    structure.y,
                    5.0,
                ))
                .with_rotation(Quat::from_rotation_z(
                    family.silhouette_rotation_degrees.to_radians(),
                )),
                FirstContactStructureSprite {
                    id: structure.id.clone(),
                    family: structure.family.clone(),
                    active: structure.active,
                },
                FirstContactMapScoped,
            ))
            .id();
        attach_identity_geometry(
            &mut commands,
            entity,
            &structure.family,
            tile_size,
            structure.owner == "contact",
        );
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
                FirstContactMapScoped,
            ));
        }
        let (image, layout, frame) = family.identity_frame.map_or_else(
            || {
                (
                    handles.units_image.clone(),
                    handles.units_layout.clone(),
                    family.atlas_index(family.idle[0]),
                )
            },
            |identity| {
                (
                    handles.identities_image.clone(),
                    handles.identities_layout.clone(),
                    identity,
                )
            },
        );
        let mut sprite = atlas_sprite(
            image,
            layout,
            frame,
            Vec2::new(
                tile_size * 2.1 * family.silhouette_scale[0],
                tile_size * 2.1 * family.silhouette_scale[1],
            ),
        );
        sprite.color = Color::srgb(family.tint[0], family.tint[1], family.tint[2]);
        if unit.owner == "contact" && family.tint == [1.0, 1.0, 1.0] {
            sprite.color = Color::srgb(1.0, 0.74, 0.70);
        }
        let entity = commands
            .spawn((
                sprite,
                Transform::from_translation(position).with_rotation(Quat::from_rotation_z(
                    family.silhouette_rotation_degrees.to_radians(),
                )),
                FirstContactUnitSprite {
                    id: unit.id.clone(),
                    family: unit.family.clone(),
                    owner: unit.owner.clone(),
                },
                FirstContactMapScoped,
            ))
            .id();
        attach_identity_geometry(
            &mut commands,
            entity,
            &unit.family,
            tile_size,
            unit.owner == "contact",
        );
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
        FirstContactMapScoped,
    ));
}

#[allow(clippy::type_complexity)]
pub(super) fn sync_first_contact_authored_map(
    mut commands: Commands,
    map: Res<FirstContactMap>,
    windows: Query<&Window>,
    map_entities: Query<
        Entity,
        Or<(
            With<FirstContactMapScoped>,
            With<FirstContactUnitSprite>,
            With<FirstContactStructureSprite>,
            With<FirstContactTransient>,
        )>,
    >,
    mut cameras: Query<
        &mut Transform,
        (
            With<FirstContactCamera>,
            Without<FirstContactMapScoped>,
            Without<FirstContactUnitSprite>,
            Without<FirstContactStructureSprite>,
        ),
    >,
) {
    if !map.is_changed() {
        return;
    }
    // The authored battlefield is a single lifecycle unit. Rebuilding it is
    // deliberately simpler and safer than trying to diff decorative entities
    // which do not have simulation identities. Unit and structure filters also
    // catch runtime-spawned visuals from the previous battle.
    for entity in &map_entities {
        commands.entity(entity).despawn();
    }
    let viewport_size = windows
        .single()
        .map(|window| Vec2::new(window.width(), window.height()))
        .unwrap_or(Vec2::new(1280.0, 720.0));
    for mut camera in &mut cameras {
        camera.translation = authored_camera_translation(&map, viewport_size);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        asset_loader::{load_first_contact_atlas, FirstContactAtlasHandles},
        map_loader::{load_first_contact_map, MissionMapCatalog},
    };
    use std::{
        collections::{BTreeMap, BTreeSet},
        path::Path,
    };

    fn authored_visual_count(map: &FirstContactMap) -> usize {
        let mut count = map.width as usize * map.height as usize;
        for y in 0..map.height as usize {
            for x in 0..map.width as usize {
                count += usize::from(transition_frame_name(map, x, y).is_some());
                let height = map.height_at(x, y);
                count += usize::from(
                    height >= 2
                        && y + 1 < map.height as usize
                        && map.height_at(x, y + 1) < 2
                        && x % 2 == 0,
                );
            }
        }
        count
            + map.chokepoints.len()
            + map.resources.len()
            + map.landmarks.len()
            + map.structures.len()
            + map.units.len()
            + map.units.iter().filter(|unit| unit.selected).count()
            + 1 // capture pulse
    }

    fn dummy_atlas_handles() -> FirstContactAtlasHandles {
        FirstContactAtlasHandles {
            units_image: Handle::default(),
            units_layout: Handle::default(),
            world_image: Handle::default(),
            world_layout: Handle::default(),
            identities_image: Handle::default(),
            identities_layout: Handle::default(),
        }
    }

    fn map_scoped_count(world: &mut World) -> usize {
        let mut query = world.query_filtered::<Entity, With<FirstContactMapScoped>>();
        query.iter(world).count()
    }

    fn identity_geometry_count(world: &mut World) -> usize {
        let mut query = world.query_filtered::<Entity, With<IdentityMotion>>();
        query.iter(world).count()
    }

    fn unit_ids(world: &mut World) -> BTreeSet<String> {
        let mut query = world.query::<&FirstContactUnitSprite>();
        query.iter(world).map(|unit| unit.id.clone()).collect()
    }

    fn structure_ids(world: &mut World) -> BTreeSet<String> {
        let mut query = world.query::<&FirstContactStructureSprite>();
        query
            .iter(world)
            .map(|structure| structure.id.clone())
            .collect()
    }

    fn assert_rendered_map(world: &mut World, map: &FirstContactMap, camera: Entity) {
        assert_eq!(map_scoped_count(world), authored_visual_count(map));
        assert_eq!(
            identity_geometry_count(world),
            (map.units.len() + map.structures.len()) * 2
        );

        let rendered_units = {
            let mut query = world.query::<(&FirstContactUnitSprite, &Transform)>();
            query
                .iter(world)
                .map(|(unit, transform)| (unit.id.clone(), transform.translation))
                .collect::<BTreeMap<_, _>>()
        };
        assert_eq!(
            rendered_units.keys().cloned().collect::<BTreeSet<_>>(),
            map.units
                .iter()
                .map(|unit| unit.id.clone())
                .collect::<BTreeSet<_>>()
        );
        for unit in &map.units {
            assert_eq!(
                rendered_units[&unit.id],
                map_world_position(map, unit.x, unit.y, 8.0),
                "unit {} did not move to map {} coordinates",
                unit.id,
                map.id
            );
        }

        let rendered_structures = {
            let mut query = world.query::<(&FirstContactStructureSprite, &Transform)>();
            query
                .iter(world)
                .map(|(structure, transform)| (structure.id.clone(), transform.translation))
                .collect::<BTreeMap<_, _>>()
        };
        assert_eq!(
            rendered_structures.keys().cloned().collect::<BTreeSet<_>>(),
            map.structures
                .iter()
                .map(|structure| structure.id.clone())
                .collect::<BTreeSet<_>>()
        );
        for structure in &map.structures {
            assert_eq!(
                rendered_structures[&structure.id],
                map_world_position(map, structure.x, structure.y, 5.0),
                "structure {} did not move to map {} coordinates",
                structure.id,
                map.id
            );
        }

        let expected_camera = authored_camera_translation(map, Vec2::new(1280.0, 720.0));
        assert_eq!(
            world.get::<Transform>(camera).unwrap().translation,
            expected_camera
        );

        let expected_capture = map_world_position(map, map.objective.x, map.objective.y, 7.0);
        let mut pulses = world.query_filtered::<&Transform, With<FirstContactObjectivePulse>>();
        assert!(pulses
            .iter(world)
            .any(|transform| transform.translation == expected_capture));
    }

    #[test]
    fn initial_camera_consumes_the_authored_map_start() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../assets/first_contact/maps/first_contact.yaml");
        let map = load_first_contact_map(&path).expect("authored map loads");
        let viewport = Vec2::new(1280.0, 699.0);
        let translation = authored_camera_translation(&map, viewport);
        let map_half_size = Vec2::new(
            map.width as f32 * map.tile_size as f32 * 0.5,
            map.height as f32 * map.tile_size as f32 * 0.5,
        );
        let viewport_half_size = viewport * FIRST_CONTACT_CAMERA_SCALE * 0.5;

        assert!(translation.x < 0.0 && translation.y < 0.0);
        assert!(translation.x - viewport_half_size.x >= -map_half_size.x - f32::EPSILON);
        assert!(translation.x + viewport_half_size.x <= map_half_size.x + f32::EPSILON);
        assert!(translation.y - viewport_half_size.y >= -map_half_size.y - f32::EPSILON);
        assert!(translation.y + viewport_half_size.y <= map_half_size.y + f32::EPSILON);
    }

    #[test]
    fn replacing_the_map_rebuilds_every_authored_visual_without_stale_entities() {
        let asset_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../assets");
        let maps = MissionMapCatalog::load(&asset_root).expect("authored maps load");
        let manifest = load_first_contact_atlas(&asset_root.join("first_contact/atlas.yaml"))
            .expect("authored atlas loads");
        let first = maps.first_contact.clone();
        let replacements = vec![
            maps.cinder_crown.clone(),
            maps.aftershock_patrol.clone(),
            maps.convoy_exodus.clone(),
            maps.mirror_siege.clone(),
            maps.iron_delta.clone(),
            maps.night_watch_crossing.clone(),
            maps.glass_basin.clone(),
            maps.ember_orchard.clone(),
            maps.salt_marsh.clone(),
        ];

        let mut app = App::new();
        app.insert_resource(first.clone())
            .insert_resource(manifest)
            .insert_resource(dummy_atlas_handles())
            .add_systems(
                Update,
                (
                    sync_first_contact_authored_map,
                    spawn_first_contact_authored_map,
                )
                    .chain(),
            );
        let camera = app
            .world_mut()
            .spawn((Transform::default(), FirstContactCamera))
            .id();

        app.update();
        assert_rendered_map(app.world_mut(), &first, camera);

        // These emulate visuals created by the simulation during a battle.
        // They are intentionally unmarked to verify the compatibility cleanup
        // for runtime units and structures as well as authored decorations.
        app.world_mut().spawn(FirstContactUnitSprite {
            id: "stale_runtime_unit".into(),
            family: "sentinel".into(),
            owner: "player".into(),
        });
        app.world_mut().spawn(FirstContactStructureSprite {
            id: "stale_runtime_structure".into(),
            family: "command_post".into(),
            active: true,
        });

        for (index, replacement) in replacements.iter().enumerate() {
            app.insert_resource(replacement.clone());
            app.update();
            assert_rendered_map(app.world_mut(), replacement, camera);

            if index == 0 {
                assert!(!unit_ids(app.world_mut()).contains("contact_scout"));
                assert!(!unit_ids(app.world_mut()).contains("stale_runtime_unit"));
                assert!(!structure_ids(app.world_mut()).contains("stale_runtime_structure"));
            }
        }
    }
}
