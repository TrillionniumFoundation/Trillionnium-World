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
use trnm_campaign_core::{BattleGridPoint, BattleOutcome};
use trnm_rts_protocol::{
    RtsFrameOrder, RtsFrameOrderStream, RtsOrderKind, RtsOrderSource, RtsTile,
};
use trnm_rts_sim::{BattlePhase, TICKS_PER_SECOND};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FirstContactCommand {
    Move,
    Attack,
    Harvest,
    Ability,
    FieldAid,
    Fortify,
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
            Self::Ability => "ABILITY",
            Self::FieldAid => "FIELD_AID",
            Self::Fortify => "FORTIFY",
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
    target_tile: IVec2,
    target_actor_id: Option<String>,
) -> RtsFrameOrder {
    let kind = match command {
        FirstContactCommand::Move => RtsOrderKind::Move,
        FirstContactCommand::Attack => RtsOrderKind::Attack,
        FirstContactCommand::Harvest => RtsOrderKind::Harvest,
        FirstContactCommand::Ability => RtsOrderKind::Ability,
        FirstContactCommand::FieldAid => RtsOrderKind::Repair,
        FirstContactCommand::Fortify => RtsOrderKind::Build,
        FirstContactCommand::Retreat => RtsOrderKind::Extract,
        FirstContactCommand::Hold => RtsOrderKind::Hold,
    };
    let mut order =
        RtsFrameOrder::new(frame, "player", actor_ids, kind, RtsOrderSource::LocalInput);
    match command {
        FirstContactCommand::Move => {
            order.target_tile = Some(RtsTile::new(target_tile.x, target_tile.y));
            order.formation_id = Some("party_adaptive_wedge".to_string());
        }
        FirstContactCommand::Attack => {
            order.target_actor_id = target_actor_id.or_else(|| Some(map.objective.id.clone()));
            order.target_tile = Some(RtsTile::new(target_tile.x, target_tile.y));
            order.formation_id = Some("party_assault_box".to_string());
        }
        FirstContactCommand::Harvest => {
            let resource = map
                .resources
                .iter()
                .find(|resource| Some(resource.id.as_str()) == target_actor_id.as_deref())
                .unwrap_or(&map.resources[0]);
            order.target_actor_id = Some(resource.id.clone());
            order.target_tile = Some(RtsTile::new(resource.x, resource.y));
        }
        FirstContactCommand::Ability => {
            order.target_rule_id = Some("party_signature".to_string());
            order.target_actor_id = target_actor_id;
            order.target_tile = Some(RtsTile::new(target_tile.x, target_tile.y));
        }
        FirstContactCommand::FieldAid => {
            order.target_actor_id = Some("party_field_aid".to_string());
            order.target_tile = Some(RtsTile::new(target_tile.x, target_tile.y));
        }
        FirstContactCommand::Fortify => {
            order.target_rule_id = Some("field_barricade".to_string());
            order.target_tile = Some(RtsTile::new(target_tile.x, target_tile.y));
        }
        FirstContactCommand::Retreat => {
            order.target_actor_id = Some("expedition_gate".to_string());
            order.target_tile = Some(RtsTile::new(map.player_start.x, map.player_start.y));
            order.formation_id = Some("party_withdraw".to_string());
        }
        FirstContactCommand::Hold => {
            order.formation_id = Some("party_guard_capture".to_string());
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
    pub phase: BattlePhase,
    pub selected_slot: Option<usize>,
    pub target_tile: IVec2,
    pub target_actor_id: Option<String>,
    animation_timer: Timer,
    feedback_timer: Timer,
    pressure_timer: Timer,
    animation_phase: usize,
    pressure_flash_seconds: f32,
    sim_tick_accumulator: f32,
}

impl Default for FirstContactRuntime {
    fn default() -> Self {
        Self {
            elapsed_seconds: 0.0,
            credits: 0,
            power_percent: 100,
            supply_used: 4,
            supply_cap: 4,
            contact_hp: 100.0,
            objective_progress: 0.0,
            command: FirstContactCommand::Hold,
            command_feedback: "Select a route and issue MOVE".to_string(),
            victory: false,
            defeat: false,
            withdrawal: false,
            party_hp_percent: 100,
            enemy_hp_percent: 100,
            group_world_position: Vec2::ZERO,
            phase: BattlePhase::Approach,
            selected_slot: None,
            target_tile: IVec2::ZERO,
            target_actor_id: None,
            animation_timer: Timer::from_seconds(0.14, TimerMode::Repeating),
            feedback_timer: Timer::from_seconds(1.8, TimerMode::Repeating),
            pressure_timer: Timer::from_seconds(15.0, TimerMode::Repeating),
            animation_phase: 0,
            pressure_flash_seconds: 0.0,
            sim_tick_accumulator: 0.0,
        }
    }
}

impl FirstContactRuntime {
    pub(super) fn reset_for_battle(&mut self, map: &FirstContactMap) {
        *self = Self::default();
        let choke = &map.chokepoints[1];
        self.target_tile = IVec2::new(
            choke.x + choke.width as i32 / 2,
            choke.y + choke.height as i32 / 2,
        );
        self.command_feedback = "APPROACH: Q moves to the south pass".to_string();
    }

    pub fn recommended_command(&self) -> FirstContactCommand {
        if self.victory || self.defeat || self.withdrawal {
            return FirstContactCommand::Hold;
        }
        if self.party_hp_percent < 60 && self.credits >= 20 {
            return FirstContactCommand::FieldAid;
        }
        if self.phase == BattlePhase::Relay && self.enemy_hp_percent > 0 && self.credits >= 30 {
            return FirstContactCommand::Fortify;
        }
        match self.phase {
            BattlePhase::Approach => FirstContactCommand::Move,
            BattlePhase::Contact if self.credits < 40 => FirstContactCommand::Harvest,
            BattlePhase::Contact => FirstContactCommand::Attack,
            BattlePhase::Relay if self.contact_hp > 0.0 => FirstContactCommand::Attack,
            BattlePhase::Relay | BattlePhase::Complete => FirstContactCommand::Hold,
        }
    }
}

#[derive(Component)]
pub(super) struct FirstContactTransient {
    timer: Timer,
}

fn target_cycle(
    map: &FirstContactMap,
    flow: &CampaignFlow,
    current: Option<&str>,
) -> (String, IVec2) {
    let mut targets = flow
        .mission
        .as_ref()
        .into_iter()
        .flat_map(|mission| mission.enemies.iter())
        .filter(|unit| unit.alive())
        .map(|unit| {
            (
                unit.unit_id.clone(),
                IVec2::new(unit.position.x as i32, unit.position.y as i32),
            )
        })
        .collect::<Vec<_>>();
    targets.extend(
        map.resources
            .iter()
            .map(|resource| (resource.id.clone(), IVec2::new(resource.x, resource.y))),
    );
    targets.push((
        map.objective.id.clone(),
        IVec2::new(map.objective.x, map.objective.y),
    ));
    let next = targets
        .iter()
        .position(|(id, _)| Some(id.as_str()) == current)
        .map(|index| (index + 1) % targets.len())
        .unwrap_or(0);
    targets[next].clone()
}

pub(super) fn handle_first_contact_commands(
    input: Res<ButtonInput<KeyCode>>,
    map: Res<FirstContactMap>,
    mut runtime: ResMut<FirstContactRuntime>,
    mut adapter: ResMut<FirstContactSimulationAdapter>,
    mut flow: ResMut<CampaignFlow>,
) {
    if !flow.in_battle() {
        return;
    }
    if input.just_pressed(KeyCode::Digit0) {
        runtime.selected_slot = None;
        runtime.command_feedback = "Selected the full four-person party".to_string();
    }
    for (key, slot) in [
        (KeyCode::Digit1, 0),
        (KeyCode::Digit2, 1),
        (KeyCode::Digit3, 2),
        (KeyCode::Digit4, 3),
    ] {
        if input.just_pressed(key) {
            runtime.selected_slot = Some(slot);
            runtime.command_feedback = format!("Selected party slot {}", slot + 1);
        }
    }
    if input.just_pressed(KeyCode::Tab) {
        let (target, tile) = target_cycle(&map, &flow, runtime.target_actor_id.as_deref());
        runtime.target_actor_id = Some(target.clone());
        runtime.target_tile = tile;
        runtime.command_feedback = format!("Target locked: {target} at {},{}", tile.x, tile.y);
    }
    let mut target_delta = IVec2::ZERO;
    if input.just_pressed(KeyCode::KeyJ) {
        target_delta.x -= 1;
    }
    if input.just_pressed(KeyCode::KeyL) {
        target_delta.x += 1;
    }
    if input.just_pressed(KeyCode::KeyI) {
        target_delta.y -= 1;
    }
    if input.just_pressed(KeyCode::KeyK) {
        target_delta.y += 1;
    }
    if target_delta != IVec2::ZERO {
        let candidate = runtime.target_tile + target_delta;
        let passable = flow.mission.as_ref().is_some_and(|mission| {
            mission
                .seed
                .map
                .passable(BattleGridPoint::new(candidate.x as i16, candidate.y as i16))
        });
        if passable {
            runtime.target_tile = candidate;
            runtime.target_actor_id = None;
            runtime.command_feedback = format!("Free target: {},{}", candidate.x, candidate.y);
        }
    }

    let command = if input.just_pressed(KeyCode::KeyQ) {
        Some(FirstContactCommand::Move)
    } else if input.just_pressed(KeyCode::KeyW) {
        Some(FirstContactCommand::Attack)
    } else if input.just_pressed(KeyCode::KeyE) {
        Some(FirstContactCommand::Harvest)
    } else if input.just_pressed(KeyCode::KeyR) {
        Some(FirstContactCommand::Hold)
    } else if input.just_pressed(KeyCode::KeyA) {
        Some(FirstContactCommand::Ability)
    } else if input.just_pressed(KeyCode::KeyS) {
        Some(FirstContactCommand::FieldAid)
    } else if input.just_pressed(KeyCode::KeyD) {
        Some(FirstContactCommand::Fortify)
    } else if input.just_pressed(KeyCode::KeyX) {
        Some(FirstContactCommand::Retreat)
    } else {
        None
    };
    let Some(command) = command else {
        return;
    };
    if command == FirstContactCommand::Harvest {
        let resource = map
            .resources
            .iter()
            .find(|resource| Some(resource.id.as_str()) == runtime.target_actor_id.as_deref())
            .unwrap_or(&map.resources[1]);
        runtime.target_actor_id = Some(resource.id.clone());
        runtime.target_tile = IVec2::new(resource.x, resource.y);
    }
    let Some(mission) = flow.mission.as_ref() else {
        flow.status = "Battle mode lost its authoritative simulation".to_string();
        return;
    };
    let actor_ids = mission
        .party
        .iter()
        .enumerate()
        .filter(|(index, unit)| {
            unit.alive() && runtime.selected_slot.is_none_or(|slot| slot == *index)
        })
        .map(|(_, unit)| unit.unit_id.clone())
        .collect::<Vec<_>>();
    let order = frame_order_for_command(
        &map,
        mission.tick as u32,
        command,
        actor_ids,
        runtime.target_tile,
        runtime.target_actor_id.clone(),
    );
    let mut candidate = adapter.accepted_orders.clone();
    candidate.push(order.clone());
    if let Err(error) = RtsFrameOrderStream::new(
        mission.seed.map_id.clone(),
        mission.seed.rules_version.clone(),
        candidate.clone(),
    )
    .validate()
    {
        flow.status = format!("Order stream rejected: {error}");
        return;
    }
    if let Err(error) = flow
        .mission
        .as_mut()
        .expect("mission checked above")
        .issue_order(order)
    {
        runtime.command_feedback = error.to_string();
        return;
    }
    adapter.accepted_orders = candidate;
    runtime.command = command;
    runtime.command_feedback = match command {
        FirstContactCommand::Move => format!(
            "Moving selected units to {},{}",
            runtime.target_tile.x, runtime.target_tile.y
        ),
        FirstContactCommand::Attack => "Map-aware assault order accepted".to_string(),
        FirstContactCommand::Harvest => {
            "Harvest route accepted; resources power abilities and return credits".to_string()
        }
        FirstContactCommand::Ability => "Signature abilities activated".to_string(),
        FirstContactCommand::FieldAid => {
            "Spent 20 field resources to heal selected units".to_string()
        }
        FirstContactCommand::Fortify => {
            "Spent 30 field resources to fortify selected units".to_string()
        }
        FirstContactCommand::Hold => {
            "Guarding position; HOLD captures an exposed relay".to_string()
        }
        FirstContactCommand::Retreat => {
            "Withdrawal accepted with zero progression reward".to_string()
        }
    };
}

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
    }

    let tick_seconds = 1.0 / TICKS_PER_SECOND as f32;
    while runtime.sim_tick_accumulator >= tick_seconds {
        let Some(mission) = flow.mission.as_mut() else {
            flow.status = "Battle mode lost its authoritative simulation".to_string();
            return;
        };
        if mission.terminal() {
            break;
        }
        if let Err(error) = mission.step() {
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
        runtime.credits = mission.resources_available;
        runtime.phase = mission.phase;
        runtime.command_feedback = if runtime.victory {
            let mission_name = if mission.seed.map_id == "first_contact_aftershock" {
                "AFTERSHOCK PATROL"
            } else {
                "FIRST CONTACT"
            };
            format!("{mission_name} SECURED: rewards and resources will return to town")
        } else if runtime.defeat {
            "PARTY DEFEATED: injuries applied, no harvested credits retained".to_string()
        } else if runtime.withdrawal {
            "WITHDRAWAL COMPLETE: no XP or resource payout".to_string()
        } else {
            match mission.phase {
                BattlePhase::Approach => "APPROACH: move through the south pass".to_string(),
                BattlePhase::Contact if mission.resources_gathered < 40 => format!(
                    "CONTACT: Party {}% | Enemy {}% | harvest for ability energy",
                    mission.party_hp_percent(),
                    mission.enemy_hp_percent()
                ),
                BattlePhase::Contact => format!(
                    "CONTACT: Party {}% | Enemy {}% | assault or use A",
                    mission.party_hp_percent(),
                    mission.enemy_hp_percent()
                ),
                BattlePhase::Relay if mission.relay_guard_hp > 0 => {
                    format!(
                        "RELAY: guard {}% | assault the objective",
                        mission.relay_guard_percent()
                    )
                }
                BattlePhase::Relay => format!(
                    "RELAY EXPOSED: press R to secure {}%",
                    mission.capture_percent()
                ),
                BattlePhase::Complete => "Battle complete".to_string(),
            }
        };
    }

    let mut simulated_visuals = std::collections::HashMap::new();
    if let Some(mission) = flow.mission.as_ref() {
        for (index, seeded) in mission.seed.party.iter().enumerate() {
            if let Some(unit) = mission
                .party
                .iter()
                .find(|unit| unit.unit_id == seeded.unit_id)
            {
                let selected = runtime.selected_slot.is_none_or(|slot| slot == index);
                simulated_visuals.insert(
                    seeded.spawn_slot.clone(),
                    (unit.position, unit.alive(), true, selected),
                );
            }
        }
        for unit in &mission.enemies {
            simulated_visuals.insert(
                unit.unit_id.clone(),
                (unit.position, unit.alive(), false, false),
            );
        }
    }

    let mut selected_position_sum = Vec2::ZERO;
    let mut selected_position_count = 0usize;
    let mut unit_positions = std::collections::HashMap::new();
    for (mut sprite, mut transform, unit) in &mut units {
        let family = manifest
            .unit(&unit.family)
            .expect("rendered unit family remains in atlas");
        let Some((position, alive, player, selected)) = simulated_visuals.get(&unit.id).copied()
        else {
            continue;
        };
        let world = map_world_position(&map, position.x as i32, position.y as i32, 8.0);
        let old = transform.translation.truncate();
        transform.translation.x = world.x;
        transform.translation.y = world.y;
        if player && selected {
            selected_position_sum += transform.translation.truncate();
            selected_position_count += 1;
        }
        let moved = old.distance(transform.translation.truncate()) > 0.5;
        let phase = runtime.animation_phase % 2;
        let column = if !alive || (runtime.victory && unit.owner == "contact") {
            family.disabled
        } else if player && selected && runtime.pressure_flash_seconds > 0.0 {
            family.hit
        } else if player && runtime.command == FirstContactCommand::Attack {
            family.attack[phase]
        } else if moved {
            family.r#move[phase]
        } else {
            family.idle[phase]
        };
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            atlas.index = family.atlas_index(column);
        }
        unit_positions.insert(unit.id.clone(), (transform.translation, selected));
    }
    if selected_position_count > 0 {
        runtime.group_world_position = selected_position_sum / selected_position_count as f32;
    }

    for (mut transform, ring) in &mut rings {
        if let Some((position, selected)) = unit_positions.get(&ring.unit_id) {
            transform.translation.x = position.x;
            transform.translation.y = position.y - 3.0;
            transform.scale = if *selected { Vec3::ONE } else { Vec3::ZERO };
        }
    }

    if runtime.command == FirstContactCommand::Attack
        && runtime.feedback_timer.just_finished()
        && !runtime.victory
        && !runtime.defeat
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
                    runtime.target_tile.x,
                    runtime.target_tile.y,
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
    use trnm_campaign_core::{BattleOutcome, CampaignRoom, CampaignSaveV1};
    use trnm_rts_sim::{BattlePhase, MissionSimV1, FIVE_MINUTE_TICKS, THREE_MINUTE_TICKS};

    fn living_subjects(sim: &MissionSimV1) -> Vec<String> {
        sim.party
            .iter()
            .filter(|unit| unit.alive())
            .map(|unit| unit.unit_id.clone())
            .collect()
    }

    #[test]
    fn player_commands_are_the_same_orders_consumed_by_the_sim() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../assets/first_contact/maps/first_contact.yaml");
        let map = load_first_contact_map(&path).expect("authored map loads");
        let commands = [
            FirstContactCommand::Move,
            FirstContactCommand::Attack,
            FirstContactCommand::Harvest,
            FirstContactCommand::Hold,
            FirstContactCommand::Ability,
            FirstContactCommand::FieldAid,
            FirstContactCommand::Fortify,
            FirstContactCommand::Retreat,
        ];
        let orders = commands
            .into_iter()
            .enumerate()
            .map(|(frame, command)| {
                frame_order_for_command(
                    &map,
                    frame as u32,
                    command,
                    vec!["hero".to_string()],
                    IVec2::new(map.objective.x, map.objective.y),
                    Some(map.objective.id.clone()),
                )
            })
            .collect();
        RtsFrameOrderStream::new(map.id.clone(), "first_contact_campaign_rules_v2", orders)
            .validate()
            .expect("frame orders validate");
    }

    #[test]
    fn authored_map_supports_the_three_phase_three_to_five_minute_route() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../assets/first_contact/maps/first_contact.yaml");
        let map = load_first_contact_map(&path).expect("authored map loads");
        let mut campaign = CampaignSaveV1::default();
        campaign.move_to(CampaignRoom::MentorHall).unwrap();
        campaign.talk_to_mentor().unwrap();
        campaign.train_with_mentor().unwrap();
        campaign.equip_starter_weapon().unwrap();
        campaign.move_to(CampaignRoom::ExpeditionGate).unwrap();
        campaign.accept_first_contact_quest().unwrap();
        let seed = campaign
            .start_first_contact_battle(map.battle_seed_map().unwrap())
            .unwrap();
        let mut sim = MissionSimV1::from_seed(seed).unwrap();
        let approach = sim.seed.map.approach_point;
        let move_order = frame_order_for_command(
            &map,
            sim.tick as u32,
            FirstContactCommand::Move,
            living_subjects(&sim),
            IVec2::new(approach.x as i32, approach.y as i32),
            None,
        );
        sim.issue_order(move_order).unwrap();
        while sim.phase == BattlePhase::Approach && !sim.terminal() {
            sim.step().unwrap();
        }
        let resource = &map.resources[1];
        let harvest_order = frame_order_for_command(
            &map,
            sim.tick as u32,
            FirstContactCommand::Harvest,
            living_subjects(&sim),
            IVec2::new(resource.x, resource.y),
            Some(resource.id.clone()),
        );
        sim.issue_order(harvest_order).unwrap();
        while sim.resources_available < 100 && !sim.terminal() {
            sim.step().unwrap();
        }
        let attack_order = frame_order_for_command(
            &map,
            sim.tick as u32,
            FirstContactCommand::Attack,
            living_subjects(&sim),
            IVec2::new(map.objective.x, map.objective.y),
            Some(map.objective.id.clone()),
        );
        sim.issue_order(attack_order).unwrap();
        while !(sim.terminal() || sim.phase == BattlePhase::Relay && sim.relay_guard_hp <= 0) {
            sim.step().unwrap();
        }
        assert!(
            !sim.terminal(),
            "authored route must reach the exposed relay"
        );
        let hold_order = frame_order_for_command(
            &map,
            sim.tick as u32,
            FirstContactCommand::Hold,
            living_subjects(&sim),
            IVec2::new(map.objective.x, map.objective.y),
            Some(map.objective.id.clone()),
        );
        sim.issue_order(hold_order).unwrap();
        for wave in 1..=2 {
            while sim.reinforcement_wave < wave && !sim.terminal() {
                sim.step().unwrap();
            }
            let resource_command = if wave == 1 {
                FirstContactCommand::FieldAid
            } else {
                FirstContactCommand::Fortify
            };
            let resource_order = frame_order_for_command(
                &map,
                sim.tick as u32,
                resource_command,
                living_subjects(&sim),
                IVec2::new(map.objective.x, map.objective.y),
                Some(map.objective.id.clone()),
            );
            sim.issue_order(resource_order).unwrap();
            let attack_order = frame_order_for_command(
                &map,
                sim.tick as u32,
                FirstContactCommand::Attack,
                living_subjects(&sim),
                IVec2::new(map.objective.x, map.objective.y),
                Some(map.objective.id.clone()),
            );
            sim.issue_order(attack_order).unwrap();
            while sim.enemies.iter().any(|enemy| enemy.alive()) && !sim.terminal() {
                sim.step().unwrap();
            }
            let move_order = frame_order_for_command(
                &map,
                sim.tick as u32,
                FirstContactCommand::Move,
                living_subjects(&sim),
                IVec2::new(map.objective.x, map.objective.y),
                None,
            );
            sim.issue_order(move_order).unwrap();
            while !sim.party.iter().any(|unit| {
                unit.alive()
                    && (unit.position.x - sim.seed.map.objective.x).abs()
                        + (unit.position.y - sim.seed.map.objective.y).abs()
                        <= 2
            }) && !sim.terminal()
            {
                sim.step().unwrap();
            }
            let hold_order = frame_order_for_command(
                &map,
                sim.tick as u32,
                FirstContactCommand::Hold,
                living_subjects(&sim),
                IVec2::new(map.objective.x, map.objective.y),
                Some(map.objective.id.clone()),
            );
            sim.issue_order(hold_order).unwrap();
        }
        while !sim.terminal() && sim.tick <= FIVE_MINUTE_TICKS {
            sim.step().unwrap();
        }
        assert_eq!(sim.outcome, Some(BattleOutcome::Victory));
        assert!(
            (THREE_MINUTE_TICKS..=FIVE_MINUTE_TICKS).contains(&sim.tick),
            "authored-map victory tick {} is outside 3-5 minutes",
            sim.tick
        );
        assert!((8..=12).contains(&sim.order_count));
    }
}
