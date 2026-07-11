use super::{
    asset_loader::{FirstContactAtlasHandles, FirstContactAtlasManifest},
    campaign_flow::CampaignFlow,
    map_loader::FirstContactMap,
    renderer::{
        atlas_sprite, map_world_position, FirstContactCamera, FirstContactObjectivePulse,
        FirstContactSelectionRing, FirstContactStructureSprite, FirstContactUnitSprite,
        FIRST_CONTACT_CAMERA_SCALE,
    },
    view_math::{minimap_to_tile, points_in_drag_rect, ViewportSpec},
};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::collections::BTreeSet;
use trnm_campaign_core::{BattleGridPoint, BattleOutcome, ControlScheme};
use trnm_rts_protocol::{
    RtsFrameOrder, RtsFrameOrderStream, RtsOrderKind, RtsOrderSource, RtsTile, RtsUnitStance,
};
use trnm_rts_sim::{BattlePhase, SimStructureKind, TICKS_PER_SECOND};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FirstContactCommand {
    Move,
    Attack,
    Harvest,
    Ability,
    FieldAid,
    Fortify,
    Recon,
    Train,
    Research,
    Upgrade,
    Patrol,
    Stop,
    Retreat,
    #[default]
    Hold,
}

fn command_for_keyboard(
    input: &ButtonInput<KeyCode>,
    scheme: ControlScheme,
) -> Option<FirstContactCommand> {
    let keys = match scheme {
        ControlScheme::Classic => [
            (KeyCode::KeyQ, FirstContactCommand::Move),
            (KeyCode::KeyW, FirstContactCommand::Attack),
            (KeyCode::KeyE, FirstContactCommand::Harvest),
            (KeyCode::KeyR, FirstContactCommand::Hold),
            (KeyCode::KeyA, FirstContactCommand::Ability),
            (KeyCode::KeyS, FirstContactCommand::FieldAid),
            (KeyCode::KeyD, FirstContactCommand::Fortify),
            (KeyCode::KeyC, FirstContactCommand::Recon),
            (KeyCode::KeyV, FirstContactCommand::Train),
            (KeyCode::KeyB, FirstContactCommand::Research),
            (KeyCode::KeyN, FirstContactCommand::Upgrade),
            (KeyCode::KeyP, FirstContactCommand::Patrol),
            (KeyCode::Space, FirstContactCommand::Stop),
            (KeyCode::KeyX, FirstContactCommand::Retreat),
        ],
        ControlScheme::LeftHanded => [
            (KeyCode::KeyA, FirstContactCommand::Move),
            (KeyCode::KeyS, FirstContactCommand::Attack),
            (KeyCode::KeyD, FirstContactCommand::Harvest),
            (KeyCode::KeyF, FirstContactCommand::Hold),
            (KeyCode::KeyQ, FirstContactCommand::Ability),
            (KeyCode::KeyW, FirstContactCommand::FieldAid),
            (KeyCode::KeyE, FirstContactCommand::Fortify),
            (KeyCode::KeyR, FirstContactCommand::Recon),
            (KeyCode::KeyZ, FirstContactCommand::Train),
            (KeyCode::KeyX, FirstContactCommand::Research),
            (KeyCode::KeyC, FirstContactCommand::Upgrade),
            (KeyCode::KeyV, FirstContactCommand::Patrol),
            (KeyCode::Space, FirstContactCommand::Stop),
            (KeyCode::KeyG, FirstContactCommand::Retreat),
        ],
        ControlScheme::ArrowGrid => [
            (KeyCode::ArrowUp, FirstContactCommand::Move),
            (KeyCode::ArrowRight, FirstContactCommand::Attack),
            (KeyCode::ArrowLeft, FirstContactCommand::Harvest),
            (KeyCode::ArrowDown, FirstContactCommand::Hold),
            (KeyCode::KeyU, FirstContactCommand::Ability),
            (KeyCode::KeyI, FirstContactCommand::FieldAid),
            (KeyCode::KeyO, FirstContactCommand::Fortify),
            (KeyCode::KeyJ, FirstContactCommand::Recon),
            (KeyCode::KeyK, FirstContactCommand::Train),
            (KeyCode::KeyL, FirstContactCommand::Research),
            (KeyCode::KeyM, FirstContactCommand::Upgrade),
            (KeyCode::KeyP, FirstContactCommand::Patrol),
            (KeyCode::Space, FirstContactCommand::Stop),
            (KeyCode::Backspace, FirstContactCommand::Retreat),
        ],
    };
    keys.into_iter()
        .find_map(|(key, command)| input.just_pressed(key).then_some(command))
}

pub(super) fn command_key_for_scheme(
    command: FirstContactCommand,
    scheme: ControlScheme,
) -> &'static str {
    match (scheme, command) {
        (ControlScheme::Classic, FirstContactCommand::Move) => "Q",
        (ControlScheme::Classic, FirstContactCommand::Attack) => "W",
        (ControlScheme::Classic, FirstContactCommand::Harvest) => "E",
        (ControlScheme::Classic, FirstContactCommand::Hold) => "R",
        (ControlScheme::Classic, FirstContactCommand::Ability) => "A",
        (ControlScheme::Classic, FirstContactCommand::FieldAid) => "S",
        (ControlScheme::Classic, FirstContactCommand::Fortify) => "D",
        (ControlScheme::Classic, FirstContactCommand::Recon) => "C",
        (ControlScheme::Classic, FirstContactCommand::Train) => "V",
        (ControlScheme::Classic, FirstContactCommand::Research) => "B",
        (ControlScheme::Classic, FirstContactCommand::Upgrade) => "N",
        (ControlScheme::Classic, FirstContactCommand::Patrol) => "P",
        (ControlScheme::Classic, FirstContactCommand::Stop) => "SPACE",
        (ControlScheme::Classic, FirstContactCommand::Retreat) => "X",
        (ControlScheme::LeftHanded, FirstContactCommand::Move) => "A",
        (ControlScheme::LeftHanded, FirstContactCommand::Attack) => "S",
        (ControlScheme::LeftHanded, FirstContactCommand::Harvest) => "D",
        (ControlScheme::LeftHanded, FirstContactCommand::Hold) => "F",
        (ControlScheme::LeftHanded, FirstContactCommand::Ability) => "Q",
        (ControlScheme::LeftHanded, FirstContactCommand::FieldAid) => "W",
        (ControlScheme::LeftHanded, FirstContactCommand::Fortify) => "E",
        (ControlScheme::LeftHanded, FirstContactCommand::Recon) => "R",
        (ControlScheme::LeftHanded, FirstContactCommand::Train) => "Z",
        (ControlScheme::LeftHanded, FirstContactCommand::Research) => "X",
        (ControlScheme::LeftHanded, FirstContactCommand::Upgrade) => "C",
        (ControlScheme::LeftHanded, FirstContactCommand::Patrol) => "V",
        (ControlScheme::LeftHanded, FirstContactCommand::Stop) => "SPACE",
        (ControlScheme::LeftHanded, FirstContactCommand::Retreat) => "G",
        (ControlScheme::ArrowGrid, FirstContactCommand::Move) => "UP",
        (ControlScheme::ArrowGrid, FirstContactCommand::Attack) => "RIGHT",
        (ControlScheme::ArrowGrid, FirstContactCommand::Harvest) => "LEFT",
        (ControlScheme::ArrowGrid, FirstContactCommand::Hold) => "DOWN",
        (ControlScheme::ArrowGrid, FirstContactCommand::Ability) => "U",
        (ControlScheme::ArrowGrid, FirstContactCommand::FieldAid) => "I",
        (ControlScheme::ArrowGrid, FirstContactCommand::Fortify) => "O",
        (ControlScheme::ArrowGrid, FirstContactCommand::Recon) => "J",
        (ControlScheme::ArrowGrid, FirstContactCommand::Train) => "K",
        (ControlScheme::ArrowGrid, FirstContactCommand::Research) => "L",
        (ControlScheme::ArrowGrid, FirstContactCommand::Upgrade) => "M",
        (ControlScheme::ArrowGrid, FirstContactCommand::Patrol) => "P",
        (ControlScheme::ArrowGrid, FirstContactCommand::Stop) => "SPACE",
        (ControlScheme::ArrowGrid, FirstContactCommand::Retreat) => "BACK",
    }
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
            Self::Recon => "RECON",
            Self::Train => "TRAIN",
            Self::Research => "RESEARCH",
            Self::Upgrade => "UPGRADE",
            Self::Patrol => "PATROL",
            Self::Stop => "STOP",
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
        FirstContactCommand::Recon => RtsOrderKind::Recon,
        FirstContactCommand::Train => RtsOrderKind::Train,
        FirstContactCommand::Research => RtsOrderKind::Research,
        FirstContactCommand::Upgrade => RtsOrderKind::Upgrade,
        FirstContactCommand::Patrol => RtsOrderKind::Patrol,
        FirstContactCommand::Stop => RtsOrderKind::Stop,
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
            order.target_actor_id = target_actor_id
                .filter(|target| {
                    target.contains("workshop")
                        || target.contains("post")
                        || target.contains("generator")
                        || target.contains("cache")
                        || target.contains("barricade")
                })
                .or_else(|| Some("party_field_aid".to_string()));
            order.target_tile = Some(RtsTile::new(target_tile.x, target_tile.y));
        }
        FirstContactCommand::Fortify => {
            order.target_rule_id = Some("field_barricade".to_string());
            order.target_tile = Some(RtsTile::new(target_tile.x, target_tile.y));
        }
        FirstContactCommand::Recon => {
            order.target_tile = Some(RtsTile::new(target_tile.x, target_tile.y));
        }
        FirstContactCommand::Train => {
            order.target_rule_id = Some("field_support_drone".to_string());
            order.queue_id = Some("expedition_production".to_string());
            order.target_tile = Some(RtsTile::new(target_tile.x, target_tile.y));
        }
        FirstContactCommand::Research => {
            order.target_rule_id = Some("field_logistics".to_string());
            order.queue_id = Some("expedition_research".to_string());
        }
        FirstContactCommand::Upgrade => {
            order.target_rule_id = Some("relay_arms".to_string());
            order.queue_id = Some("expedition_upgrade".to_string());
        }
        FirstContactCommand::Patrol => {
            order.target_tile = Some(RtsTile::new(target_tile.x, target_tile.y));
            order.formation_id = Some("party_patrol".to_string());
        }
        FirstContactCommand::Stop => {}
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FormationStyle {
    #[default]
    Wedge,
    Line,
    Column,
}

impl FormationStyle {
    fn next(self) -> Self {
        match self {
            Self::Wedge => Self::Line,
            Self::Line => Self::Column,
            Self::Column => Self::Wedge,
        }
    }

    pub(super) fn id(self) -> &'static str {
        match self {
            Self::Wedge => "party_wedge",
            Self::Line => "party_line",
            Self::Column => "party_column",
        }
    }
}

#[derive(Resource, Default)]
pub(super) struct MouseSelectionState {
    drag_start_world: Option<Vec2>,
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
    pub visible_enemy_count: usize,
    pub intel_level: u8,
    pub visible_percent: u8,
    pub queued_jobs: usize,
    pub queued_orders: usize,
    pub support_units: usize,
    pub tech_level: u8,
    pub veteran_rank: u8,
    pub production_variant: u8,
    pub structure_variant: u8,
    pub selected_stance: RtsUnitStance,
    pub active_control_group: Option<u8>,
    pub last_group_recall: Option<(u8, f32)>,
    pub camera_focus_request: Option<Vec2>,
    pub group_world_position: Vec2,
    pub phase: BattlePhase,
    pub selected_slots: BTreeSet<usize>,
    pub formation: FormationStyle,
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
            visible_enemy_count: 0,
            intel_level: 0,
            visible_percent: 0,
            queued_jobs: 0,
            queued_orders: 0,
            support_units: 0,
            tech_level: 0,
            veteran_rank: 0,
            production_variant: 0,
            structure_variant: 0,
            selected_stance: RtsUnitStance::Guard,
            active_control_group: Some(1),
            last_group_recall: None,
            camera_focus_request: None,
            group_world_position: Vec2::ZERO,
            phase: BattlePhase::Approach,
            selected_slots: BTreeSet::new(),
            formation: FormationStyle::default(),
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
        if matches!(self.phase, BattlePhase::Contact | BattlePhase::Relay)
            && self.visible_enemy_count == 0
            && self.credits >= 10
        {
            return FirstContactCommand::Recon;
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
            BattlePhase::ConvoyEscort | BattlePhase::Extraction => FirstContactCommand::Move,
            BattlePhase::GeneratorDefense if self.enemy_hp_percent > 0 => {
                FirstContactCommand::Attack
            }
            BattlePhase::GeneratorDefense => FirstContactCommand::Hold,
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
        .filter(|unit| {
            flow.mission
                .as_ref()
                .is_some_and(|mission| mission.is_enemy_visible(&unit.unit_id))
        })
        .map(|unit| {
            (
                unit.unit_id.clone(),
                IVec2::new(unit.position.x as i32, unit.position.y as i32),
            )
        })
        .collect::<Vec<_>>();
    targets.extend(
        flow.mission
            .as_ref()
            .into_iter()
            .flat_map(|mission| mission.structures.iter())
            .filter(|structure| structure.hp > 0)
            .map(|structure| {
                (
                    structure.structure_id.clone(),
                    IVec2::new(structure.position.x as i32, structure.position.y as i32),
                )
            }),
    );
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

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(super) fn handle_first_contact_mouse_selection(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut cameras: Query<
        (&Camera, &GlobalTransform, &mut Transform),
        (With<FirstContactCamera>, Without<FirstContactUnitSprite>),
    >,
    units: Query<
        (&Transform, &FirstContactUnitSprite),
        (With<FirstContactUnitSprite>, Without<FirstContactCamera>),
    >,
    map: Res<FirstContactMap>,
    flow: Res<CampaignFlow>,
    mut mouse: ResMut<MouseSelectionState>,
    mut runtime: ResMut<FirstContactRuntime>,
) {
    if !flow.in_battle()
        || !flow.mouse_gameplay_enabled()
        || (!buttons.just_pressed(MouseButton::Left) && !buttons.just_released(MouseButton::Left))
    {
        return;
    }
    let Ok(window) = windows.single() else {
        return;
    };
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let Ok((camera, camera_global, mut camera_transform)) = cameras.single_mut() else {
        return;
    };
    if buttons.just_pressed(MouseButton::Left) {
        if let Ok(world) = camera.viewport_to_world_2d(camera_global, cursor) {
            mouse.drag_start_world = Some(world);
        }
        return;
    }

    let radar_left = window.width() - 208.0;
    let radar_top = 60.0;
    if cursor.x >= radar_left
        && cursor.x <= radar_left + 190.0
        && cursor.y >= radar_top
        && cursor.y <= radar_top + 132.0
    {
        let local = Vec2::new(
            (cursor.x - radar_left - 13.0).clamp(0.0, 164.0),
            (132.0 - (cursor.y - radar_top) - 17.0).clamp(0.0, 98.0),
        );
        let tile = minimap_to_tile(local, Vec2::new(164.0, 98.0), map.width, map.height);
        runtime.target_tile = tile;
        runtime.target_actor_id = None;
        let desired = map_world_position(&map, tile.x, tile.y, camera_transform.translation.z);
        camera_transform.translation = ViewportSpec::new(
            map.width,
            map.height,
            map.tile_size,
            Vec2::new(window.width(), window.height()),
            FIRST_CONTACT_CAMERA_SCALE,
        )
        .clamp_camera(desired.truncate())
        .extend(desired.z);
        runtime.command_feedback = format!("Minimap target/camera: {},{}", tile.x, tile.y);
        mouse.drag_start_world = None;
        return;
    }

    let Some(start) = mouse.drag_start_world.take() else {
        return;
    };
    let Ok(mut end) = camera.viewport_to_world_2d(camera_global, cursor) else {
        return;
    };
    let mut start = start;
    if start.distance(end) < 6.0 {
        let radius = map.tile_size as f32 * 0.6;
        start -= Vec2::splat(radius);
        end += Vec2::splat(radius);
    }
    let mut slots = Vec::new();
    let mut positions = Vec::new();
    for (transform, unit) in &units {
        if unit.owner != "player" {
            continue;
        }
        let Some(slot) = unit
            .id
            .strip_prefix("party_")
            .and_then(|value| value.parse().ok())
        else {
            continue;
        };
        slots.push(slot);
        positions.push(transform.translation.truncate());
    }
    let selected = points_in_drag_rect(start, end, &positions)
        .into_iter()
        .filter_map(|index| slots.get(index).copied())
        .collect::<BTreeSet<_>>();
    if !selected.is_empty() {
        runtime.selected_slots = selected;
        runtime.command_feedback = format!(
            "Mouse selected {} unit(s); F cycles formation",
            runtime.selected_slots.len()
        );
    }
}

pub(super) fn handle_first_contact_commands(
    input: Res<ButtonInput<KeyCode>>,
    map: Res<FirstContactMap>,
    mut runtime: ResMut<FirstContactRuntime>,
    mut adapter: ResMut<FirstContactSimulationAdapter>,
    mut flow: ResMut<CampaignFlow>,
) {
    if !flow.in_battle() || !flow.keyboard_gameplay_enabled() {
        return;
    }
    if input.just_pressed(KeyCode::Digit0) {
        runtime.selected_slots.clear();
        runtime.active_control_group = None;
        runtime.command_feedback = "Selected the full four-person party".to_string();
    }
    let control = input.pressed(KeyCode::ControlLeft) || input.pressed(KeyCode::ControlRight);
    for (key, group) in [
        (KeyCode::Digit1, 1_u8),
        (KeyCode::Digit2, 2),
        (KeyCode::Digit3, 3),
        (KeyCode::Digit4, 4),
        (KeyCode::Digit5, 5),
        (KeyCode::Digit6, 6),
        (KeyCode::Digit7, 7),
        (KeyCode::Digit8, 8),
        (KeyCode::Digit9, 9),
    ] {
        if input.just_pressed(key) {
            let Some(mission) = flow.mission.as_mut() else {
                return;
            };
            let selected_ids = mission
                .party
                .iter()
                .enumerate()
                .filter(|(index, unit)| {
                    unit.alive()
                        && (runtime.selected_slots.is_empty()
                            || runtime.selected_slots.contains(index))
                })
                .map(|(_, unit)| unit.unit_id.clone())
                .collect::<Vec<_>>();
            let mut order = RtsFrameOrder::new(
                mission.tick as u32,
                "player",
                selected_ids,
                if control {
                    RtsOrderKind::AssignGroup
                } else {
                    RtsOrderKind::RecallGroup
                },
                RtsOrderSource::LocalInput,
            );
            order.target_rule_id = Some(group.to_string());
            order.raw_command_label = Some(if control {
                format!("CONTROL_GROUP_ASSIGN:{group}")
            } else {
                format!("CONTROL_GROUP_RECALL:{group}")
            });
            if let Err(error) = mission.issue_order(order.clone()) {
                runtime.command_feedback = error.to_string();
                continue;
            }
            if control {
                runtime.active_control_group = Some(group);
                runtime.command_feedback = format!("Assigned selection to control group {group}");
            } else {
                let members = mission.control_group_members(&group.to_string());
                runtime.selected_slots = mission
                    .party
                    .iter()
                    .enumerate()
                    .filter(|(_, unit)| members.contains(&unit.unit_id))
                    .map(|(index, _)| index)
                    .collect();
                runtime.active_control_group = Some(group);
                let double_tap = runtime.last_group_recall.is_some_and(|(previous, at)| {
                    previous == group && runtime.elapsed_seconds - at <= 0.65
                });
                if double_tap {
                    let positions = mission
                        .party
                        .iter()
                        .filter(|unit| members.contains(&unit.unit_id) && unit.alive())
                        .map(|unit| {
                            map_world_position(
                                &map,
                                unit.position.x as i32,
                                unit.position.y as i32,
                                0.0,
                            )
                            .truncate()
                        })
                        .collect::<Vec<_>>();
                    if !positions.is_empty() {
                        runtime.camera_focus_request =
                            Some(positions.iter().copied().sum::<Vec2>() / positions.len() as f32);
                    }
                    runtime.command_feedback =
                        format!("Recalled and focused control group {group}");
                } else {
                    runtime.command_feedback = format!("Recalled control group {group}");
                }
                runtime.last_group_recall = Some((group, runtime.elapsed_seconds));
            }
            adapter.accepted_orders.push(order);
        }
    }
    if input.just_pressed(KeyCode::KeyF) {
        runtime.formation = runtime.formation.next();
        runtime.command_feedback = format!("Formation selected: {}", runtime.formation.id());
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

    if input.just_pressed(KeyCode::KeyZ) {
        runtime.production_variant = (runtime.production_variant + 1) % 2;
        runtime.command_feedback = if runtime.production_variant == 0 {
            "Production selected: support drone".to_string()
        } else {
            "Production selected: field medic".to_string()
        };
        return;
    }
    if input.just_pressed(KeyCode::KeyH) {
        runtime.structure_variant = (runtime.structure_variant + 1) % 4;
        let label = match runtime.structure_variant {
            0 => "field barricade",
            1 => "relay generator",
            2 => "field workshop",
            _ => "supply cache",
        };
        runtime.command_feedback = format!("Structure selected: {label}");
        return;
    }
    if input.just_pressed(KeyCode::KeyG) {
        runtime.selected_stance = match runtime.selected_stance {
            RtsUnitStance::HoldFire => RtsUnitStance::Guard,
            RtsUnitStance::Guard => RtsUnitStance::Aggressive,
            RtsUnitStance::Aggressive => RtsUnitStance::HoldFire,
        };
        let Some(mission) = flow.mission.as_ref() else {
            return;
        };
        let subjects = mission
            .party
            .iter()
            .enumerate()
            .filter(|(index, unit)| {
                unit.alive()
                    && (runtime.selected_slots.is_empty() || runtime.selected_slots.contains(index))
            })
            .map(|(_, unit)| unit.unit_id.clone())
            .collect::<Vec<_>>();
        let mut order = RtsFrameOrder::new(
            mission.tick as u32,
            "player",
            subjects,
            RtsOrderKind::SetStance,
            RtsOrderSource::LocalInput,
        );
        order.target_rule_id = Some(runtime.selected_stance.as_str().to_string());
        match flow
            .mission
            .as_mut()
            .expect("stance mission exists")
            .issue_order(order.clone())
        {
            Ok(()) => {
                adapter.accepted_orders.push(order);
                runtime.command_feedback =
                    format!("Unit stance: {}", runtime.selected_stance.as_str());
            }
            Err(error) => runtime.command_feedback = error.to_string(),
        }
        return;
    }
    let lifecycle = flow.mission.as_ref().and_then(|mission| {
        if input.just_pressed(KeyCode::Delete) {
            mission
                .queued_orders
                .back()
                .and_then(|queued| queued.queue_id.clone())
                .map(|id| (RtsOrderKind::CancelQueuedOrder, id))
        } else if input.just_pressed(KeyCode::KeyU) {
            mission
                .jobs
                .first()
                .map(|job| (RtsOrderKind::CancelJob, job.job_id.clone()))
        } else if input.just_pressed(KeyCode::KeyY) {
            mission.jobs.first().map(|job| {
                (
                    if job.paused {
                        RtsOrderKind::ResumeJob
                    } else {
                        RtsOrderKind::PauseJob
                    },
                    job.job_id.clone(),
                )
            })
        } else if input.just_pressed(KeyCode::KeyO) {
            mission
                .jobs
                .last()
                .map(|job| (RtsOrderKind::PromoteJob, job.job_id.clone()))
        } else if input.just_pressed(KeyCode::KeyM) {
            mission
                .jobs
                .first()
                .map(|job| (RtsOrderKind::SetRally, job.job_id.clone()))
        } else {
            None
        }
    });
    if let Some((kind, queue_id)) = lifecycle {
        let mission = flow.mission.as_ref().expect("lifecycle mission exists");
        let subjects = mission
            .party
            .iter()
            .filter(|unit| unit.alive())
            .map(|unit| unit.unit_id.clone())
            .collect::<Vec<_>>();
        let mut order = RtsFrameOrder::new(
            mission.tick as u32,
            "player",
            subjects,
            kind,
            RtsOrderSource::LocalInput,
        );
        order.queue_id = Some(queue_id);
        if kind == RtsOrderKind::SetRally {
            order.target_tile = Some(RtsTile::new(runtime.target_tile.x, runtime.target_tile.y));
        }
        match flow
            .mission
            .as_mut()
            .expect("lifecycle mission exists")
            .issue_order(order.clone())
        {
            Ok(()) => {
                adapter.accepted_orders.push(order);
                runtime.command_feedback = format!("{} accepted", kind.as_str());
            }
            Err(error) => runtime.command_feedback = error.to_string(),
        }
        return;
    }

    let command = command_for_keyboard(&input, flow.settings.control_scheme);
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
            unit.alive()
                && (runtime.selected_slots.is_empty() || runtime.selected_slots.contains(index))
        })
        .map(|(_, unit)| unit.unit_id.clone())
        .collect::<Vec<_>>();
    let mut order = frame_order_for_command(
        &map,
        mission.tick as u32,
        command,
        actor_ids,
        runtime.target_tile,
        runtime.target_actor_id.clone(),
    );
    let shift = input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight);
    if shift
        && matches!(
            command,
            FirstContactCommand::Move
                | FirstContactCommand::Attack
                | FirstContactCommand::Harvest
                | FirstContactCommand::Patrol
                | FirstContactCommand::Hold
        )
    {
        order.queued = true;
        order.queue_id = Some(format!("player-order-{}", mission.tick));
    }
    if command == FirstContactCommand::Train && runtime.production_variant == 1 {
        order.target_rule_id = Some("field_medic".to_string());
    }
    if command == FirstContactCommand::Fortify {
        order.target_rule_id = Some(
            match runtime.structure_variant {
                0 => "field_barricade",
                1 => "relay_generator",
                2 => "field_workshop",
                _ => "supply_cache",
            }
            .to_string(),
        );
    }
    if command == FirstContactCommand::Research
        && mission.researched_techs.contains("field_logistics")
    {
        order.target_rule_id = Some("signal_optics".to_string());
    }
    if command == FirstContactCommand::Upgrade && mission.upgrade_level > 0 {
        order.target_rule_id = Some("field_armor".to_string());
    }
    if matches!(
        command,
        FirstContactCommand::Move
            | FirstContactCommand::Attack
            | FirstContactCommand::Patrol
            | FirstContactCommand::Hold
    ) {
        order.formation_id = Some(runtime.formation.id().to_string());
    }
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
            "Spent field resources to heal units or repair the targeted structure".to_string()
        }
        FirstContactCommand::Fortify => {
            "Spent 30 field resources to fortify selected units".to_string()
        }
        FirstContactCommand::Recon => {
            "Spent 10 field resources: recon now boosts mapped contact damage".to_string()
        }
        FirstContactCommand::Train => {
            "Queued a persistent-in-battle support drone for 40 field resources".to_string()
        }
        FirstContactCommand::Research => {
            "Queued Field Logistics research for 35 field resources".to_string()
        }
        FirstContactCommand::Upgrade => {
            "Queued Relay Arms upgrade after research for 45 field resources".to_string()
        }
        FirstContactCommand::Patrol => {
            "Patrolling between the current position and free target".to_string()
        }
        FirstContactCommand::Stop => "Stopped selected units and cleared their queue".to_string(),
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
        (
            &mut Sprite,
            &mut Transform,
            &mut Visibility,
            &FirstContactUnitSprite,
        ),
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
        (
            &mut Sprite,
            &mut Transform,
            &mut Visibility,
            &FirstContactStructureSprite,
        ),
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
    if !flow.in_battle() || !flow.gameplay_running() {
        return;
    }
    let delta = time.delta_secs();
    runtime.elapsed_seconds += delta;
    runtime.sim_tick_accumulator += delta;
    runtime.feedback_timer.tick(time.delta());
    if flow.settings.low_motion {
        runtime.pressure_flash_seconds = 0.0;
    } else {
        runtime.animation_timer.tick(time.delta());
        runtime.pressure_timer.tick(time.delta());
        runtime.pressure_flash_seconds = (runtime.pressure_flash_seconds - delta).max(0.0);
        if runtime.animation_timer.just_finished() {
            runtime.animation_phase = runtime.animation_phase.wrapping_add(1);
        }
        if runtime.pressure_timer.just_finished() && !runtime.victory {
            runtime.pressure_flash_seconds = 0.42;
        }
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
        runtime.enemy_hp_percent = mission.visible_enemy_hp_percent();
        runtime.visible_enemy_count = mission.visible_enemy_count();
        runtime.contact_hp = mission.relay_guard_percent() as f32;
        runtime.objective_progress = mission.capture_percent() as f32;
        runtime.victory = mission.outcome == Some(BattleOutcome::Victory);
        runtime.defeat = mission.outcome == Some(BattleOutcome::Defeat);
        runtime.withdrawal = mission.outcome == Some(BattleOutcome::Withdrawal);
        runtime.credits = mission.resources_available;
        runtime.intel_level = mission.intel_level;
        runtime.visible_percent = mission.visible_percent();
        runtime.queued_jobs = mission.jobs.len();
        runtime.queued_orders = mission.queued_orders.len();
        runtime.support_units = mission.support_units.len();
        runtime.tech_level = mission.upgrade_level;
        runtime.veteran_rank = mission
            .party
            .iter()
            .map(|unit| unit.veteran_rank)
            .max()
            .unwrap_or(0);
        runtime.supply_used = mission.supply_used();
        runtime.supply_cap = mission.supply_cap();
        runtime.power_percent = if mission.power_draw() == 0 {
            100
        } else {
            (u32::from(mission.power_provided()) * 100 / u32::from(mission.power_draw())).min(100)
                as u8
        };
        runtime.phase = mission.phase;
        runtime.command_feedback = if runtime.victory {
            let mission_name = mission.seed.mission.title.to_ascii_uppercase();
            format!("{mission_name} SECURED: rewards and resources will return to town")
        } else if runtime.defeat {
            "PARTY DEFEATED: injuries applied, no harvested credits retained".to_string()
        } else if runtime.withdrawal {
            "WITHDRAWAL COMPLETE: no XP or resource payout".to_string()
        } else {
            match mission.phase {
                BattlePhase::Approach => "APPROACH: move through the south pass".to_string(),
                BattlePhase::Contact if mission.visible_enemy_count() == 0 => {
                    "CONTACT: no hostile is currently visible; move or use C recon".to_string()
                }
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
                BattlePhase::ConvoyEscort => format!(
                    "ESCORT: stay near the convoy and move toward {}",
                    mission.current_objective_id().unwrap_or("the generator")
                ),
                BattlePhase::GeneratorDefense => format!(
                    "DEFEND: hold the generator {} ticks; clear incoming raiders",
                    mission.objective_progress_ticks
                ),
                BattlePhase::Extraction => format!(
                    "EXTRACT: escort the convoy to the north gate ({}/80)",
                    mission.objective_progress_ticks
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
                let selected =
                    runtime.selected_slots.is_empty() || runtime.selected_slots.contains(&index);
                simulated_visuals.insert(
                    seeded.spawn_slot.clone(),
                    (unit.position, unit.alive(), true, selected, true),
                );
            }
        }
        for unit in &mission.enemies {
            simulated_visuals.insert(
                unit.unit_id.clone(),
                (
                    unit.position,
                    unit.alive(),
                    false,
                    false,
                    mission.is_enemy_visible(&unit.unit_id),
                ),
            );
        }
        for support in &mission.support_units {
            simulated_visuals.insert(
                support.unit_id.clone(),
                (support.position, support.hp > 0, true, false, true),
            );
        }
        if let Some(position) = mission.convoy_position {
            simulated_visuals.insert(
                "supply_convoy".to_string(),
                (position, mission.convoy_hp > 0, true, false, true),
            );
        }
    }

    let existing_visual_ids = units
        .iter()
        .map(|(_, _, _, unit)| unit.id.clone())
        .collect::<BTreeSet<_>>();
    let existing_structure_ids = structures
        .iter()
        .map(|(_, _, _, structure)| structure.id.clone())
        .collect::<BTreeSet<_>>();
    if let Some(handles) = handles.as_deref() {
        let family = manifest
            .unit("sentinel")
            .expect("support sentinel family is authored");
        for (id, (position, alive, player, _, _)) in &simulated_visuals {
            if (!id.starts_with("field_support_") && !id.starts_with("field_medic_"))
                || existing_visual_ids.contains(id)
            {
                continue;
            }
            commands.spawn((
                atlas_sprite(
                    handles.world_image.clone(),
                    handles.world_layout.clone(),
                    if *alive {
                        family.idle[0]
                    } else {
                        family.disabled
                    },
                    Vec2::splat(map.tile_size as f32 * 1.55),
                ),
                Transform::from_translation(map_world_position(
                    &map,
                    position.x as i32,
                    position.y as i32,
                    8.0,
                )),
                FirstContactUnitSprite {
                    id: id.clone(),
                    family: "sentinel".to_string(),
                    owner: if *player { "player" } else { "contact" }.to_string(),
                },
            ));
        }
        if let Some(mission) = flow.mission.as_ref() {
            for structure in &mission.structures {
                if existing_structure_ids.contains(&structure.structure_id) {
                    continue;
                }
                let family_id = match structure.kind {
                    SimStructureKind::CommandPost => "command_core",
                    SimStructureKind::FieldWorkshop => "foundry",
                    SimStructureKind::RelayGenerator => "shield_relay",
                    SimStructureKind::SupplyCache => "refinery",
                    SimStructureKind::FieldBarricade => "defense_turret",
                    SimStructureKind::SensorTower => "shield_relay",
                    SimStructureKind::FieldHospital => "refinery",
                    SimStructureKind::SiegeFoundry => "foundry",
                };
                let family = manifest
                    .structure(family_id)
                    .expect("sim structure family is authored");
                commands.spawn((
                    atlas_sprite(
                        handles.world_image.clone(),
                        handles.world_layout.clone(),
                        family.active,
                        Vec2::splat(map.tile_size as f32 * 2.35),
                    ),
                    Transform::from_translation(map_world_position(
                        &map,
                        structure.position.x as i32,
                        structure.position.y as i32,
                        6.0,
                    )),
                    FirstContactStructureSprite {
                        id: structure.structure_id.clone(),
                        family: family_id.to_string(),
                        active: true,
                    },
                ));
            }
        }
    }

    let mut selected_position_sum = Vec2::ZERO;
    let mut selected_position_count = 0usize;
    let mut unit_positions = std::collections::HashMap::new();
    for (mut sprite, mut transform, mut visibility, unit) in &mut units {
        let family = manifest
            .unit(&unit.family)
            .expect("rendered unit family remains in atlas");
        let Some((position, alive, player, selected, visible)) =
            simulated_visuals.get(&unit.id).copied()
        else {
            continue;
        };
        *visibility = if visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
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
        if visible {
            unit_positions.insert(unit.id.clone(), (transform.translation, selected));
        }
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

    for (mut sprite, mut transform, mut visibility, structure) in &mut structures {
        if let Some(simulated) = flow.mission.as_ref().and_then(|mission| {
            mission
                .structures
                .iter()
                .find(|candidate| candidate.structure_id == structure.id)
        }) {
            transform.translation = map_world_position(
                &map,
                simulated.position.x as i32,
                simulated.position.y as i32,
                6.0,
            );
            *visibility = if simulated.hp > 0 {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
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
    windows: Query<&Window, With<PrimaryWindow>>,
    flow: Res<CampaignFlow>,
    mut runtime: ResMut<FirstContactRuntime>,
    mut cameras: Query<&mut Transform, With<FirstContactCamera>>,
) {
    if !flow.in_battle() || !flow.keyboard_gameplay_enabled() {
        return;
    }
    let Ok(mut camera) = cameras.single_mut() else {
        return;
    };
    if let Some(request) = runtime.camera_focus_request.take() {
        camera.translation.x = request.x;
        camera.translation.y = request.y;
    }
    let mut direction = Vec2::ZERO;
    let arrow_grid = flow.settings.control_scheme == ControlScheme::ArrowGrid;
    if input.pressed(if arrow_grid {
        KeyCode::KeyA
    } else {
        KeyCode::ArrowLeft
    }) {
        direction.x -= 1.0;
    }
    if input.pressed(if arrow_grid {
        KeyCode::KeyD
    } else {
        KeyCode::ArrowRight
    }) {
        direction.x += 1.0;
    }
    if input.pressed(if arrow_grid {
        KeyCode::KeyW
    } else {
        KeyCode::ArrowUp
    }) {
        direction.y += 1.0;
    }
    if input.pressed(if arrow_grid {
        KeyCode::KeyS
    } else {
        KeyCode::ArrowDown
    }) {
        direction.y -= 1.0;
    }
    camera.translation += direction.normalize_or_zero().extend(0.0) * 360.0 * time.delta_secs();
    let viewport = windows
        .single()
        .map(|window| Vec2::new(window.width(), window.height()))
        .unwrap_or(Vec2::new(1280.0, 720.0));
    let clamped = ViewportSpec::new(
        map.width,
        map.height,
        map.tile_size,
        viewport,
        FIRST_CONTACT_CAMERA_SCALE,
    )
    .clamp_camera(camera.translation.truncate());
    camera.translation.x = clamped.x;
    camera.translation.y = clamped.y;
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
            FirstContactCommand::Recon,
            FirstContactCommand::Train,
            FirstContactCommand::Research,
            FirstContactCommand::Upgrade,
            FirstContactCommand::Patrol,
            FirstContactCommand::Stop,
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
        RtsFrameOrderStream::new(map.id.clone(), "first_contact_campaign_rules_v3", orders)
            .validate()
            .expect("frame orders validate");

        let repair = frame_order_for_command(
            &map,
            20,
            FirstContactCommand::FieldAid,
            vec!["hero".to_string()],
            IVec2::new(map.player_start.x, map.player_start.y),
            Some("relay_generator-20".to_string()),
        );
        assert_eq!(
            repair.target_actor_id.as_deref(),
            Some("relay_generator-20")
        );
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
            "authored route must reach the exposed relay: tick {} phase {:?} outcome {:?} guard {} enemies {}",
            sim.tick,
            sim.phase,
            sim.outcome,
            sim.relay_guard_hp,
            sim.enemies.iter().filter(|enemy| enemy.alive()).count(),
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
#[test]
fn control_profiles_remap_authoritative_commands_instead_of_only_changing_a_label() {
    let mut classic = ButtonInput::default();
    classic.press(KeyCode::KeyQ);
    assert_eq!(
        command_for_keyboard(&classic, ControlScheme::Classic),
        Some(FirstContactCommand::Move)
    );

    let mut left_handed = ButtonInput::default();
    left_handed.press(KeyCode::KeyA);
    assert_eq!(
        command_for_keyboard(&left_handed, ControlScheme::LeftHanded),
        Some(FirstContactCommand::Move)
    );

    let mut arrow_grid = ButtonInput::default();
    arrow_grid.press(KeyCode::ArrowRight);
    assert_eq!(
        command_for_keyboard(&arrow_grid, ControlScheme::ArrowGrid),
        Some(FirstContactCommand::Attack)
    );
    assert_eq!(
        command_key_for_scheme(FirstContactCommand::Attack, ControlScheme::ArrowGrid),
        "RIGHT"
    );
}
