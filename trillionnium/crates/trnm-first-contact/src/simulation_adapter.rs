use super::{
    asset_loader::{FirstContactAtlasHandles, FirstContactAtlasManifest},
    map_loader::FirstContactMap,
    renderer::{
        atlas_sprite, map_world_position, FirstContactCamera, FirstContactObjectivePulse,
        FirstContactSelectionRing, FirstContactStructureSprite, FirstContactUnitSprite,
    },
};
use bevy::prelude::*;
use trnm_rts_core::{RtsFrameOrder, RtsFrameOrderStream, RtsOrderKind, RtsOrderSource, RtsTile};

const CONTACT_DAMAGE_PER_SECOND: f32 = 0.25;
const BEACON_CAPTURE_PER_SECOND: f32 = 0.5;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FirstContactCommand {
    Move,
    Attack,
    Harvest,
    #[default]
    Hold,
}

impl FirstContactCommand {
    pub fn label(self) -> &'static str {
        match self {
            Self::Move => "MOVE",
            Self::Attack => "ATTACK",
            Self::Harvest => "HARVEST",
            Self::Hold => "HOLD",
        }
    }
}

#[derive(Resource, Default)]
pub struct FirstContactSimulationAdapter {
    pub accepted_orders: Vec<RtsFrameOrder>,
}

fn frame_order_for_command(
    map: &FirstContactMap,
    frame: u32,
    command: FirstContactCommand,
) -> RtsFrameOrder {
    let kind = match command {
        FirstContactCommand::Move => RtsOrderKind::Move,
        FirstContactCommand::Attack => RtsOrderKind::Attack,
        FirstContactCommand::Harvest => RtsOrderKind::Harvest,
        FirstContactCommand::Hold => RtsOrderKind::Hold,
    };
    let mut order = RtsFrameOrder::new(
        frame,
        "player",
        vec![
            "group1_worker".to_string(),
            "group1_scout".to_string(),
            "group1_warden".to_string(),
            "group1_striker".to_string(),
        ],
        kind,
        RtsOrderSource::LocalInput,
    );
    match command {
        FirstContactCommand::Move => {
            let choke = &map.chokepoints[1];
            order.target_tile = Some(RtsTile::new(
                choke.x + choke.width as i32 / 2,
                choke.y + choke.height as i32 / 2,
            ));
            order.formation_id = Some("group1_wedge".to_string());
        }
        FirstContactCommand::Attack => {
            order.target_actor_id = Some(map.objective.id.clone());
            order.target_tile = Some(RtsTile::new(map.objective.x, map.objective.y));
            order.formation_id = Some("group1_assault_box".to_string());
        }
        FirstContactCommand::Harvest => {
            order.target_actor_id = Some(map.resources[0].id.clone());
            order.target_tile = Some(RtsTile::new(map.resources[0].x, map.resources[0].y));
        }
        FirstContactCommand::Hold => {
            order.formation_id = Some("group1_hold".to_string());
        }
    }
    order.raw_command_label = Some(format!("FIRST_CONTACT:{}", command.label()));
    order
}

#[derive(Resource)]
pub struct FirstContactRuntime {
    pub elapsed_seconds: f32,
    pub credits: u32,
    pub power_percent: u8,
    pub supply_used: u8,
    pub supply_cap: u8,
    pub contact_hp: f32,
    pub objective_progress: f32,
    pub command: FirstContactCommand,
    pub command_feedback: String,
    pub victory: bool,
    pub group_world_position: Vec2,
    animation_timer: Timer,
    feedback_timer: Timer,
    pressure_timer: Timer,
    animation_phase: usize,
    pressure_flash_seconds: f32,
    harvest_credit_buffer: f32,
}

impl Default for FirstContactRuntime {
    fn default() -> Self {
        Self {
            elapsed_seconds: 0.0,
            credits: 1160,
            power_percent: 91,
            supply_used: 12,
            supply_cap: 22,
            contact_hp: 100.0,
            objective_progress: 0.0,
            command: FirstContactCommand::Hold,
            command_feedback: "Group 1 ready".to_string(),
            victory: false,
            group_world_position: Vec2::ZERO,
            animation_timer: Timer::from_seconds(0.14, TimerMode::Repeating),
            feedback_timer: Timer::from_seconds(1.8, TimerMode::Repeating),
            pressure_timer: Timer::from_seconds(15.0, TimerMode::Repeating),
            animation_phase: 0,
            pressure_flash_seconds: 0.0,
            harvest_credit_buffer: 0.0,
        }
    }
}

impl FirstContactRuntime {
    pub fn recommended_command(&self) -> FirstContactCommand {
        if self.victory {
            FirstContactCommand::Hold
        } else {
            match self.command {
                FirstContactCommand::Move
                | FirstContactCommand::Attack
                | FirstContactCommand::Harvest => self.command,
                FirstContactCommand::Hold => FirstContactCommand::Attack,
            }
        }
    }
}

#[derive(Component)]
pub(super) struct FirstContactTransient {
    timer: Timer,
}

pub(super) fn handle_first_contact_commands(
    input: Res<ButtonInput<KeyCode>>,
    map: Res<FirstContactMap>,
    mut runtime: ResMut<FirstContactRuntime>,
    mut adapter: ResMut<FirstContactSimulationAdapter>,
) {
    let command = if input.just_pressed(KeyCode::KeyQ) {
        Some(FirstContactCommand::Move)
    } else if input.just_pressed(KeyCode::KeyW) {
        Some(FirstContactCommand::Attack)
    } else if input.just_pressed(KeyCode::KeyE) {
        Some(FirstContactCommand::Harvest)
    } else if input.just_pressed(KeyCode::KeyR) {
        Some(FirstContactCommand::Hold)
    } else {
        None
    };
    let Some(command) = command else {
        return;
    };
    let order = frame_order_for_command(&map, (runtime.elapsed_seconds * 60.0) as u32, command);
    let mut candidate = adapter.accepted_orders.clone();
    candidate.push(order);
    RtsFrameOrderStream::new(map.id.clone(), "first_contact_rules_v1", candidate.clone())
        .validate()
        .expect("live First Contact commands remain valid RTS core frame orders");
    adapter.accepted_orders = candidate;
    runtime.command = command;
    runtime.command_feedback = match command {
        FirstContactCommand::Move => "Moving through the south pass".to_string(),
        FirstContactCommand::Attack => "Attacking the Relay Beacon".to_string(),
        FirstContactCommand::Harvest => "Harvesting the home crystal route".to_string(),
        FirstContactCommand::Hold => "Holding the current formation".to_string(),
    };
}

fn command_target(map: &FirstContactMap, command: FirstContactCommand) -> Vec3 {
    match command {
        FirstContactCommand::Move => {
            let choke = &map.chokepoints[1];
            map_world_position(
                map,
                choke.x + choke.width as i32 / 2,
                choke.y + choke.height as i32 / 2,
                8.0,
            )
        }
        FirstContactCommand::Attack => {
            map_world_position(map, map.objective.x, map.objective.y, 8.0)
        }
        FirstContactCommand::Harvest => {
            let resource = &map.resources[0];
            map_world_position(map, resource.x, resource.y, 8.0)
        }
        FirstContactCommand::Hold => Vec3::ZERO,
    }
}

fn formation_offset(unit_id: &str) -> Vec2 {
    match unit_id {
        "group1_worker" => Vec2::new(-46.0, 28.0),
        "group1_scout" => Vec2::new(46.0, 28.0),
        "group1_warden" => Vec2::new(-46.0, -28.0),
        "group1_striker" => Vec2::new(46.0, -28.0),
        _ => Vec2::ZERO,
    }
}

// Explicit disjoint query filters let Bevy validate the mutable component
// access while keeping this frame update as one ordered simulation system.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(super) fn advance_first_contact_simulation(
    mut commands: Commands,
    time: Res<Time>,
    map: Res<FirstContactMap>,
    manifest: Res<FirstContactAtlasManifest>,
    handles: Option<Res<FirstContactAtlasHandles>>,
    mut runtime: ResMut<FirstContactRuntime>,
    mut units: Query<
        (&mut Sprite, &mut Transform, &FirstContactUnitSprite),
        (
            Without<FirstContactSelectionRing>,
            Without<FirstContactStructureSprite>,
            Without<FirstContactObjectivePulse>,
        ),
    >,
    mut rings: Query<
        (&mut Transform, &FirstContactSelectionRing),
        (
            Without<FirstContactUnitSprite>,
            Without<FirstContactStructureSprite>,
            Without<FirstContactObjectivePulse>,
        ),
    >,
    mut structures: Query<
        (&mut Sprite, &FirstContactStructureSprite),
        (
            Without<FirstContactUnitSprite>,
            Without<FirstContactSelectionRing>,
            Without<FirstContactObjectivePulse>,
        ),
    >,
    mut pulses: Query<
        (&mut Transform, &FirstContactObjectivePulse),
        (
            Without<FirstContactUnitSprite>,
            Without<FirstContactSelectionRing>,
            Without<FirstContactStructureSprite>,
        ),
    >,
) {
    let delta = time.delta_secs();
    runtime.elapsed_seconds += delta;
    runtime.animation_timer.tick(time.delta());
    runtime.feedback_timer.tick(time.delta());
    runtime.pressure_timer.tick(time.delta());
    runtime.pressure_flash_seconds = (runtime.pressure_flash_seconds - delta).max(0.0);
    if runtime.animation_timer.just_finished() {
        runtime.animation_phase = runtime.animation_phase.wrapping_add(1);
    }
    if runtime.pressure_timer.just_finished() && !runtime.victory {
        runtime.power_percent = runtime.power_percent.saturating_sub(1).max(45);
        runtime.pressure_flash_seconds = 0.42;
        runtime.command_feedback = "Contact pressure hit the outer relay".to_string();
    }

    let target = command_target(&map, runtime.command);
    let mut selected_near_target = 0usize;
    let mut selected_position_sum = Vec2::ZERO;
    let mut selected_position_count = 0usize;
    let mut unit_positions = std::collections::HashMap::new();
    for (mut sprite, mut transform, unit) in &mut units {
        let family = manifest
            .unit(&unit.family)
            .expect("rendered unit family remains in atlas");
        let selected_player = unit.owner == "player" && unit.selected;
        let unit_target = target.truncate() + formation_offset(&unit.id);
        let distance = transform.translation.truncate().distance(unit_target);
        if selected_player && runtime.command != FirstContactCommand::Hold && distance > 84.0 {
            let direction = (unit_target - transform.translation.truncate()).normalize_or_zero();
            transform.translation += direction.extend(0.0) * 56.0 * delta;
        }
        if selected_player && distance <= 120.0 {
            selected_near_target += 1;
        }
        if selected_player {
            selected_position_sum += transform.translation.truncate();
            selected_position_count += 1;
        }
        let phase = runtime.animation_phase % 2;
        let column = if runtime.victory && unit.owner == "contact" {
            family.disabled
        } else if selected_player && runtime.pressure_flash_seconds > 0.0 {
            family.hit
        } else if selected_player
            && runtime.command == FirstContactCommand::Attack
            && distance <= 132.0
        {
            family.attack[phase]
        } else if selected_player && runtime.command != FirstContactCommand::Hold && distance > 84.0
        {
            family.r#move[phase]
        } else {
            family.idle[phase]
        };
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            atlas.index = family.atlas_index(column);
        }
        unit_positions.insert(unit.id.clone(), transform.translation);
    }
    if selected_position_count > 0 {
        runtime.group_world_position = selected_position_sum / selected_position_count as f32;
    }

    for (mut transform, ring) in &mut rings {
        if let Some(position) = unit_positions.get(&ring.unit_id) {
            transform.translation.x = position.x;
            transform.translation.y = position.y - 3.0;
        }
    }

    if runtime.command == FirstContactCommand::Harvest && selected_near_target >= 3 {
        runtime.harvest_credit_buffer += 5.0 * delta;
        let harvested = runtime.harvest_credit_buffer.floor() as u32;
        if harvested > 0 {
            runtime.credits = runtime.credits.saturating_add(harvested);
            runtime.harvest_credit_buffer -= harvested as f32;
        }
        runtime.command_feedback = "Crystal flow online".to_string();
    }
    if runtime.command == FirstContactCommand::Attack
        && selected_near_target >= 3
        && !runtime.victory
    {
        if runtime.contact_hp > 0.0 {
            runtime.contact_hp = (runtime.contact_hp - CONTACT_DAMAGE_PER_SECOND * delta).max(0.0);
            runtime.command_feedback = format!("Beacon guard {:.0}%", runtime.contact_hp);
        } else {
            runtime.objective_progress =
                (runtime.objective_progress + BEACON_CAPTURE_PER_SECOND * delta).min(100.0);
            runtime.command_feedback =
                format!("Securing beacon {:.0}%", runtime.objective_progress);
            if runtime.objective_progress >= 100.0 {
                runtime.victory = true;
                runtime.command = FirstContactCommand::Hold;
                runtime.command_feedback = "FIRST CONTACT SECURED".to_string();
            }
        }
        if runtime.feedback_timer.just_finished() {
            if let Some(handles) = handles.as_deref() {
                let hit = manifest.effect_frames[if runtime.animation_phase.is_multiple_of(2) {
                    "hit_a"
                } else {
                    "hit_b"
                }];
                commands.spawn((
                    atlas_sprite(
                        handles.world_image.clone(),
                        handles.world_layout.clone(),
                        hit,
                        Vec2::splat(map.tile_size as f32 * 1.7),
                    ),
                    Transform::from_translation(map_world_position(
                        &map,
                        map.objective.x,
                        map.objective.y,
                        12.0,
                    )),
                    FirstContactTransient {
                        timer: Timer::from_seconds(0.32, TimerMode::Once),
                    },
                ));
            }
        }
    }

    for (mut sprite, structure) in &mut structures {
        let family = manifest
            .structure(&structure.family)
            .expect("rendered structure family remains in atlas");
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            atlas.index = if structure.active && runtime.animation_phase % 6 >= 3 {
                family.active
            } else {
                family.idle
            };
        }
    }

    let pulse = 0.96 + (runtime.elapsed_seconds * 3.0).sin().abs() * 0.10;
    for (mut transform, _) in &mut pulses {
        transform.scale = Vec3::splat(pulse);
    }
}

pub(super) fn expire_first_contact_feedback(
    mut commands: Commands,
    time: Res<Time>,
    mut transients: Query<(Entity, &mut FirstContactTransient)>,
) {
    for (entity, mut transient) in &mut transients {
        transient.timer.tick(time.delta());
        if transient.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub(super) fn pan_first_contact_camera(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    map: Res<FirstContactMap>,
    mut cameras: Query<&mut Transform, With<FirstContactCamera>>,
) {
    let Ok(mut camera) = cameras.single_mut() else {
        return;
    };
    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    if input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    camera.translation += direction.normalize_or_zero().extend(0.0) * 360.0 * time.delta_secs();
    let half_width = map.width as f32 * map.tile_size as f32 * 0.5;
    let half_height = map.height as f32 * map.tile_size as f32 * 0.5;
    camera.translation.x = camera
        .translation
        .x
        .clamp(-half_width * 0.35, half_width * 0.35);
    camera.translation.y = camera
        .translation
        .y
        .clamp(-half_height * 0.35, half_height * 0.35);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map_loader::load_first_contact_map;
    use std::path::Path;

    #[test]
    fn player_commands_enter_the_existing_rts_frame_order_contract() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../assets/first_contact/maps/first_contact.yaml");
        let map = load_first_contact_map(&path).expect("authored map loads");
        let orders = [
            FirstContactCommand::Move,
            FirstContactCommand::Attack,
            FirstContactCommand::Harvest,
            FirstContactCommand::Hold,
        ]
        .into_iter()
        .enumerate()
        .map(|(frame, command)| frame_order_for_command(&map, frame as u32, command))
        .collect();
        let stream = RtsFrameOrderStream::new(map.id.clone(), "first_contact_rules_v1", orders);
        stream.validate().expect("frame orders validate");
    }

    #[test]
    fn objective_pacing_targets_a_ten_minute_assault_and_capture() {
        let assault_seconds = 100.0 / CONTACT_DAMAGE_PER_SECOND;
        let capture_seconds = 100.0 / BEACON_CAPTURE_PER_SECOND;
        assert_eq!(assault_seconds + capture_seconds, 600.0);
    }
}
