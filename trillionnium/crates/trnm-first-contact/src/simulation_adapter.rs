use super::{
    asset_loader::{FirstContactAtlasHandles, FirstContactAtlasManifest},
    campaign_flow::CampaignFlow,
    map_loader::FirstContactMap,
    renderer::{
        atlas_sprite, map_world_position, FirstContactCamera, FirstContactObjectivePulse,
        FirstContactSelectionRing, FirstContactStructureSprite, FirstContactUnitSprite,
    },
};
use bevy::prelude::*;
use trnm_campaign_core::BattleOutcome;
use trnm_rts_core::{RtsFrameOrder, RtsFrameOrderStream, RtsOrderKind, RtsOrderSource, RtsTile};
use trnm_rts_sim::{SimCommand, TICKS_PER_SECOND};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FirstContactCommand {
    Move,
    Attack,
    Harvest,
    Retreat,
    #[default]
    Hold,
}

impl FirstContactCommand {
    pub fn label(self) -> &'static str {
        match self {
            Self::Move => "MOVE",
            Self::Attack => "ATTACK",
            Self::Harvest => "HARVEST",
            Self::Retreat => "RETREAT",
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
    actor_ids: Vec<String>,
) -> RtsFrameOrder {
    let kind = match command {
        FirstContactCommand::Move => RtsOrderKind::Move,
        FirstContactCommand::Attack => RtsOrderKind::Attack,
        FirstContactCommand::Harvest => RtsOrderKind::Harvest,
        FirstContactCommand::Retreat => RtsOrderKind::Move,
        FirstContactCommand::Hold => RtsOrderKind::Hold,
    };
    let mut order =
        RtsFrameOrder::new(frame, "player", actor_ids, kind, RtsOrderSource::LocalInput);
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
        FirstContactCommand::Retreat => {
            order.target_tile = Some(RtsTile::new(map.player_start.x, map.player_start.y));
            order.formation_id = Some("party_withdraw".to_string());
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
    pub defeat: bool,
    pub withdrawal: bool,
    pub party_hp_percent: u8,
    pub enemy_hp_percent: u8,
    pub group_world_position: Vec2,
    animation_timer: Timer,
    feedback_timer: Timer,
    pressure_timer: Timer,
    animation_phase: usize,
    pressure_flash_seconds: f32,
    harvest_credit_buffer: f32,
    sim_tick_accumulator: f32,
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
            defeat: false,
            withdrawal: false,
            party_hp_percent: 100,
            enemy_hp_percent: 100,
            group_world_position: Vec2::ZERO,
            animation_timer: Timer::from_seconds(0.14, TimerMode::Repeating),
            feedback_timer: Timer::from_seconds(1.8, TimerMode::Repeating),
            pressure_timer: Timer::from_seconds(15.0, TimerMode::Repeating),
            animation_phase: 0,
            pressure_flash_seconds: 0.0,
            harvest_credit_buffer: 0.0,
            sim_tick_accumulator: 0.0,
        }
    }
}

impl FirstContactRuntime {
    pub(super) fn reset_for_battle(&mut self) {
        *self = Self::default();
        self.command_feedback = "Campaign party deployed from BattleSeed".to_string();
    }

    pub fn recommended_command(&self) -> FirstContactCommand {
        if self.victory {
            FirstContactCommand::Hold
        } else {
            match self.command {
                FirstContactCommand::Move
                | FirstContactCommand::Attack
                | FirstContactCommand::Harvest => self.command,
                FirstContactCommand::Retreat => FirstContactCommand::Hold,
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
    flow: Res<CampaignFlow>,
) {
    if !flow.in_battle() {
        return;
    }
    let command = if input.just_pressed(KeyCode::KeyQ) {
        Some(FirstContactCommand::Move)
    } else if input.just_pressed(KeyCode::KeyW) {
        Some(FirstContactCommand::Attack)
    } else if input.just_pressed(KeyCode::KeyE) {
        Some(FirstContactCommand::Harvest)
    } else if input.just_pressed(KeyCode::KeyR) {
        Some(FirstContactCommand::Hold)
    } else if input.just_pressed(KeyCode::KeyX) {
        Some(FirstContactCommand::Retreat)
    } else {
        None
    };
    let Some(command) = command else {
        return;
    };
    let actor_ids = flow
        .mission
        .as_ref()
        .map(|mission| {
            mission
                .seed
                .party
                .iter()
                .map(|unit| unit.unit_id.clone())
                .collect()
        })
        .unwrap_or_default();
    let order = frame_order_for_command(
        &map,
        (runtime.elapsed_seconds * 60.0) as u32,
        command,
        actor_ids,
    );
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
        FirstContactCommand::Retreat => "Withdrawing to Mirror Square".to_string(),
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
        FirstContactCommand::Retreat => {
            map_world_position(map, map.player_start.x, map.player_start.y, 8.0)
        }
        FirstContactCommand::Hold => Vec3::ZERO,
    }
}

fn formation_offset(unit_id: &str) -> Vec2 {
    match unit_id {
        "party_0" => Vec2::new(-46.0, 28.0),
        "party_1" => Vec2::new(46.0, 28.0),
        "party_2" => Vec2::new(-46.0, -28.0),
        "party_3" => Vec2::new(46.0, -28.0),
        _ => Vec2::ZERO,
    }
}

fn simulated_world_position(map: &FirstContactMap, progress_milli: i32, visual_id: &str) -> Vec2 {
    let start = map_world_position(map, map.player_start.x, map.player_start.y, 8.0).truncate();
    let objective = map_world_position(map, map.objective.x, map.objective.y, 8.0).truncate();
    let progress = (progress_milli as f32 / 100_000.0).clamp(0.0, 1.0);
    start.lerp(objective, progress) + formation_offset(visual_id)
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
    mut flow: ResMut<CampaignFlow>,
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
    if !flow.in_battle() {
        return;
    }
    let delta = time.delta_secs();
    runtime.elapsed_seconds += delta;
    runtime.sim_tick_accumulator += delta;
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

    let sim_command = match runtime.command {
        FirstContactCommand::Move => SimCommand::Advance,
        FirstContactCommand::Attack => SimCommand::Assault,
        FirstContactCommand::Harvest => SimCommand::Harvest,
        FirstContactCommand::Hold => SimCommand::Hold,
        FirstContactCommand::Retreat => SimCommand::Retreat,
    };
    let tick_seconds = 1.0 / TICKS_PER_SECOND as f32;
    while runtime.sim_tick_accumulator >= tick_seconds {
        let Some(mission) = flow.mission.as_mut() else {
            flow.status = "Battle mode lost its authoritative simulation".to_string();
            return;
        };
        if mission.terminal() {
            break;
        }
        if let Err(error) = mission.step(sim_command) {
            flow.status = error.to_string();
            break;
        }
        runtime.sim_tick_accumulator -= tick_seconds;
    }
    if let Err(error) = flow.checkpoint_if_due() {
        flow.status = error;
    }
    if let Some(mission) = flow.mission.as_ref() {
        runtime.party_hp_percent = mission.party_hp_percent();
        runtime.enemy_hp_percent = mission.enemy_hp_percent();
        runtime.contact_hp = mission.relay_guard_percent() as f32;
        runtime.objective_progress = mission.capture_percent() as f32;
        runtime.victory = mission.outcome == Some(BattleOutcome::Victory);
        runtime.defeat = mission.outcome == Some(BattleOutcome::Defeat);
        runtime.withdrawal = mission.outcome == Some(BattleOutcome::Withdrawal);
        runtime.credits = 1_160_u32.saturating_add(mission.resources_gathered);
        runtime.command_feedback = if runtime.victory {
            "FIRST CONTACT SECURED".to_string()
        } else if runtime.defeat {
            "PARTY INCAPACITATED".to_string()
        } else if runtime.withdrawal {
            "WITHDRAWAL COMPLETE".to_string()
        } else if mission.enemy_hp_percent() > 0 {
            format!(
                "Party {}% | Contact force {}%",
                mission.party_hp_percent(),
                mission.enemy_hp_percent()
            )
        } else if mission.relay_guard_hp > 0 {
            format!("Relay guard {}%", mission.relay_guard_percent())
        } else {
            format!("Securing relay {}%", mission.capture_percent())
        };
    }

    let mut simulated_visuals = std::collections::HashMap::new();
    if let Some(mission) = flow.mission.as_ref() {
        for seeded in &mission.seed.party {
            if let Some(unit) = mission
                .party
                .iter()
                .find(|unit| unit.unit_id == seeded.unit_id)
            {
                simulated_visuals.insert(
                    seeded.spawn_slot.clone(),
                    (unit.position_milli, unit.alive(), true),
                );
            }
        }
        for unit in &mission.enemies {
            simulated_visuals.insert(
                unit.unit_id.clone(),
                (unit.position_milli, unit.alive(), false),
            );
        }
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
        if let Some((progress_milli, _, _)) = simulated_visuals.get(&unit.id) {
            let position = simulated_world_position(&map, *progress_milli, &unit.id);
            transform.translation.x = position.x;
            transform.translation.y = position.y;
        }
        let distance = transform.translation.truncate().distance(unit_target);
        if selected_player
            && !simulated_visuals.contains_key(&unit.id)
            && runtime.command != FirstContactCommand::Hold
            && distance > 84.0
        {
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
        let simulated_alive = simulated_visuals
            .get(&unit.id)
            .map(|(_, alive, _)| *alive)
            .unwrap_or(true);
        let column = if !simulated_alive || (runtime.victory && unit.owner == "contact") {
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
        && !runtime.defeat
        && runtime.feedback_timer.just_finished()
    {
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
    flow: Res<CampaignFlow>,
    mut cameras: Query<&mut Transform, With<FirstContactCamera>>,
) {
    if !flow.in_battle() {
        return;
    }
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
            FirstContactCommand::Retreat,
        ]
        .into_iter()
        .enumerate()
        .map(|(frame, command)| {
            frame_order_for_command(
                &map,
                frame as u32,
                command,
                vec![
                    "hero".to_string(),
                    "aya".to_string(),
                    "mako".to_string(),
                    "tess".to_string(),
                ],
            )
        })
        .collect();
        let stream = RtsFrameOrderStream::new(map.id.clone(), "first_contact_rules_v1", orders);
        stream.validate().expect("frame orders validate");
    }
}
