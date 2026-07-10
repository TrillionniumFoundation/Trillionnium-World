use super::{
    map_loader::FirstContactMap,
    simulation_adapter::{FirstContactCommand, FirstContactRuntime},
    view_math::world_to_minimap,
};
use bevy::prelude::*;

#[derive(Component)]
pub(super) struct FirstContactResourceText;

#[derive(Component)]
pub(super) struct FirstContactObjectiveText;

#[derive(Component)]
pub(super) struct FirstContactFeedbackText;

#[derive(Component)]
pub(super) struct FirstContactRosterText;

#[derive(Component)]
pub(super) struct FirstContactMatchClockText;

#[derive(Component)]
pub(super) struct FirstContactCommandCard {
    command: FirstContactCommand,
}

#[derive(Component)]
pub(super) struct FirstContactRadarPlayerMarker;

fn command_card(command: FirstContactCommand, key: &str, label: &str) -> impl Bundle {
    (
        Node {
            width: px(86),
            height: px(70),
            border: UiRect::all(px(2)),
            padding: UiRect::all(px(8)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        BackgroundColor(Color::srgba(0.055, 0.085, 0.082, 0.96)),
        BorderColor::all(Color::srgb(0.22, 0.33, 0.31)),
        FirstContactCommandCard { command },
        children![
            (
                Text::new(key.to_string()),
                TextFont::from_font_size(12.0),
                TextColor(Color::srgb(0.62, 0.78, 0.74)),
            ),
            (
                Text::new(label.to_string()),
                TextFont::from_font_size(13.0),
                TextColor(Color::srgb(0.94, 0.94, 0.80)),
            ),
        ],
    )
}

fn radar_panel() -> impl Bundle {
    (
        Node {
            position_type: PositionType::Absolute,
            top: px(60),
            right: px(18),
            width: px(190),
            height: px(132),
            border: UiRect::all(px(2)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.025, 0.045, 0.042, 0.92)),
        BorderColor::all(Color::srgb(0.22, 0.48, 0.43)),
        children![
            (
                Text::new("RADAR"),
                Node {
                    position_type: PositionType::Absolute,
                    top: px(8),
                    left: px(10),
                    ..default()
                },
                TextFont::from_font_size(11.0),
                TextColor(Color::srgb(0.74, 0.86, 0.66)),
            ),
            (
                Node {
                    position_type: PositionType::Absolute,
                    left: px(34),
                    bottom: px(24),
                    width: px(12),
                    height: px(12),
                    border: UiRect::all(px(2)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.36, 0.90, 0.50)),
                BorderColor::all(Color::srgb(0.82, 1.0, 0.78)),
                FirstContactRadarPlayerMarker,
            ),
            (
                Node {
                    position_type: PositionType::Absolute,
                    right: px(26),
                    top: px(28),
                    width: px(16),
                    height: px(16),
                    border: UiRect::all(px(3)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.94, 0.66, 0.24)),
                BorderColor::all(Color::srgb(1.0, 0.93, 0.48)),
            ),
            (
                Node {
                    position_type: PositionType::Absolute,
                    left: px(88),
                    top: px(50),
                    width: px(8),
                    height: px(48),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.22, 0.29, 0.28)),
            ),
            (
                Text::new("YOU"),
                Node {
                    position_type: PositionType::Absolute,
                    left: px(18),
                    bottom: px(6),
                    ..default()
                },
                TextFont::from_font_size(9.0),
                TextColor(Color::srgb(0.55, 0.94, 0.62)),
            ),
            (
                Text::new("BEACON"),
                Node {
                    position_type: PositionType::Absolute,
                    right: px(8),
                    top: px(8),
                    ..default()
                },
                TextFont::from_font_size(9.0),
                TextColor(Color::srgb(1.0, 0.82, 0.38)),
            ),
        ],
    )
}

pub(super) fn spawn_first_contact_hud(mut commands: Commands, map: Res<FirstContactMap>) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: px(0),
            left: px(0),
            width: percent(100),
            height: percent(100),
            ..default()
        },
        children![
            (
                Node {
                    position_type: PositionType::Absolute,
                    top: px(0),
                    left: px(0),
                    width: percent(100),
                    height: px(48),
                    padding: UiRect::axes(px(20), px(0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    border: UiRect::bottom(px(2)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.018, 0.034, 0.031, 0.97)),
                BorderColor::all(Color::srgb(0.22, 0.43, 0.38)),
                children![
                    (
                        Text::new("CREDITS 1160  |  POWER 91%  |  SUPPLY 12/22"),
                        FirstContactResourceText,
                        TextFont::from_font_size(15.0),
                        TextColor(Color::srgb(0.88, 0.93, 0.72)),
                    ),
                    (
                        Text::new(map.objective.label.clone()),
                        FirstContactObjectiveText,
                        TextFont::from_font_size(16.0),
                        TextColor(Color::srgb(1.0, 0.78, 0.31)),
                    ),
                    (
                        Text::new("00:00"),
                        FirstContactMatchClockText,
                        TextFont::from_font_size(14.0),
                        TextColor(Color::srgb(0.68, 0.84, 0.80)),
                    ),
                ],
            ),
            radar_panel(),
            (
                Node {
                    position_type: PositionType::Absolute,
                    left: px(0),
                    bottom: px(0),
                    width: percent(100),
                    height: px(198),
                    padding: UiRect::axes(px(18), px(14)),
                    flex_direction: FlexDirection::Row,
                    column_gap: px(24),
                    border: UiRect::top(px(2)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.018, 0.032, 0.030, 0.98)),
                BorderColor::all(Color::srgb(0.20, 0.40, 0.35)),
                children![
                    (
                        Node {
                            width: px(260),
                            height: percent(100),
                            flex_direction: FlexDirection::Column,
                            row_gap: px(8),
                            ..default()
                        },
                        children![
                            (
                                Text::new("YOU  /  GROUP 1 SELECTED"),
                                TextFont::from_font_size(13.0),
                                TextColor(Color::srgb(0.60, 0.92, 0.62)),
                            ),
                            (
                                Text::new("HERO   SCOUT\nWARDEN   STRIKER"),
                                FirstContactRosterText,
                                TextFont::from_font_size(16.0),
                                TextColor(Color::srgb(0.92, 0.92, 0.78)),
                            ),
                            (
                                Text::new("4 UNITS  |  READY"),
                                TextFont::from_font_size(11.0),
                                TextColor(Color::srgb(0.60, 0.76, 0.72)),
                            ),
                        ],
                    ),
                    (
                        Node {
                            width: px(760),
                            height: percent(100),
                            flex_direction: FlexDirection::Column,
                            row_gap: px(8),
                            ..default()
                        },
                        children![
                            (
                                Text::new("COMMANDS"),
                                TextFont::from_font_size(13.0),
                                TextColor(Color::srgb(0.72, 0.88, 0.68)),
                            ),
                            (
                                Node {
                                    flex_direction: FlexDirection::Row,
                                    column_gap: px(6),
                                    ..default()
                                },
                                children![
                                    command_card(FirstContactCommand::Move, "Q", "MOVE"),
                                    command_card(FirstContactCommand::Attack, "W", "ATTACK"),
                                    command_card(FirstContactCommand::Harvest, "E", "HARVEST"),
                                    command_card(FirstContactCommand::Hold, "R", "HOLD"),
                                    command_card(FirstContactCommand::Ability, "A", "ABILITY"),
                                    command_card(FirstContactCommand::FieldAid, "S", "AID"),
                                    command_card(FirstContactCommand::Fortify, "D", "FORTIFY"),
                                    command_card(FirstContactCommand::Retreat, "X", "RETREAT"),
                                ],
                            ),
                            (
                                Node {
                                    flex_direction: FlexDirection::Row,
                                    column_gap: px(6),
                                    ..default()
                                },
                                children![
                                    command_card(FirstContactCommand::Recon, "C", "RECON"),
                                    command_card(FirstContactCommand::Train, "V", "TRAIN"),
                                    command_card(FirstContactCommand::Research, "B", "RESEARCH"),
                                    command_card(FirstContactCommand::Upgrade, "N", "UPGRADE"),
                                ],
                            ),
                        ],
                    ),
                    (
                        Node {
                            flex_grow: 1.0,
                            height: percent(100),
                            flex_direction: FlexDirection::Column,
                            row_gap: px(8),
                            ..default()
                        },
                        children![
                            (
                                Text::new("CURRENT OBJECTIVE"),
                                TextFont::from_font_size(13.0),
                                TextColor(Color::srgb(0.72, 0.88, 0.68)),
                            ),
                            (
                                Text::new(map.objective.label.clone()),
                                TextFont::from_font_size(17.0),
                                TextColor(Color::srgb(1.0, 0.80, 0.36)),
                            ),
                            (
                                Text::new("Group 1 ready"),
                                FirstContactFeedbackText,
                                TextFont::from_font_size(12.0),
                                TextColor(Color::srgb(0.72, 0.88, 0.82)),
                            ),
                        ],
                    ),
                ],
            ),
        ],
    ));
}

// Bevy injects each disjoint ECS query as a system parameter; keeping the
// filters explicit here prevents overlapping mutable `Text` access at runtime.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(super) fn update_first_contact_hud(
    runtime: Res<FirstContactRuntime>,
    map: Res<FirstContactMap>,
    mut resources: Query<
        &mut Text,
        (
            With<FirstContactResourceText>,
            Without<FirstContactRosterText>,
        ),
    >,
    mut objectives: Query<
        &mut Text,
        (
            With<FirstContactObjectiveText>,
            Without<FirstContactResourceText>,
            Without<FirstContactRosterText>,
        ),
    >,
    mut feedback: Query<
        &mut Text,
        (
            With<FirstContactFeedbackText>,
            Without<FirstContactResourceText>,
            Without<FirstContactObjectiveText>,
            Without<FirstContactRosterText>,
        ),
    >,
    mut rosters: Query<
        &mut Text,
        (
            With<FirstContactRosterText>,
            Without<FirstContactResourceText>,
            Without<FirstContactObjectiveText>,
            Without<FirstContactFeedbackText>,
            Without<FirstContactMatchClockText>,
        ),
    >,
    mut clocks: Query<
        &mut Text,
        (
            With<FirstContactMatchClockText>,
            Without<FirstContactResourceText>,
            Without<FirstContactObjectiveText>,
            Without<FirstContactFeedbackText>,
            Without<FirstContactRosterText>,
        ),
    >,
    mut cards: Query<(
        &FirstContactCommandCard,
        &mut BackgroundColor,
        &mut BorderColor,
    )>,
    mut radar_markers: Query<&mut Node, With<FirstContactRadarPlayerMarker>>,
) {
    for mut text in &mut resources {
        text.0 = format!(
            "FIELD {} | PWR {}% | PARTY {}% | VIS {}% | INTEL {} | JOBS {} | ORDERS {} | SUPPORT {} | TECH {} | VET {} | SUPPLY {}/{}",
            runtime.credits,
            runtime.power_percent,
            runtime.party_hp_percent,
            runtime.visible_percent,
            runtime.intel_level,
            runtime.queued_jobs,
            runtime.queued_orders,
            runtime.support_units,
            runtime.tech_level,
            runtime.veteran_rank,
            runtime.supply_used,
            runtime.supply_cap
        );
    }
    for mut text in &mut objectives {
        text.0 = if runtime.victory {
            "MISSION SECURED  |  RETURN TO MIRROR SQUARE FOR SETTLEMENT".to_string()
        } else if runtime.defeat {
            "MISSION FAILED  |  RETURNING TO MIRROR SQUARE".to_string()
        } else if runtime.withdrawal {
            "WITHDRAWAL COMPLETE".to_string()
        } else if runtime.phase == trnm_rts_sim::BattlePhase::Approach {
            format!(
                "APPROACH SOUTH PASS  |  TARGET {},{}",
                runtime.target_tile.x, runtime.target_tile.y
            )
        } else if runtime.phase == trnm_rts_sim::BattlePhase::Contact
            && runtime.visible_enemy_count == 0
        {
            "CONTACT UNKNOWN  |  MOVE OR USE C RECON".to_string()
        } else if runtime.enemy_hp_percent > 0 {
            format!(
                "BREAK CONTACT FORCE  |  ENEMY {}%",
                runtime.enemy_hp_percent
            )
        } else if runtime.contact_hp > 0.0 {
            format!("SECURE RELAY BEACON  |  GUARD {:.0}%", runtime.contact_hp)
        } else {
            format!("SECURE RELAY BEACON  |  {:.0}%", runtime.objective_progress)
        };
    }
    for mut text in &mut feedback {
        let next = runtime.recommended_command();
        text.0 = format!(
            "{}  |  NEXT: {} {}",
            runtime.command_feedback,
            match next {
                FirstContactCommand::Move => "Q",
                FirstContactCommand::Attack => "W",
                FirstContactCommand::Harvest => "E",
                FirstContactCommand::Hold => "R",
                FirstContactCommand::Ability => "A",
                FirstContactCommand::FieldAid => "S",
                FirstContactCommand::Fortify => "D",
                FirstContactCommand::Recon => "C",
                FirstContactCommand::Train => "V",
                FirstContactCommand::Research => "B",
                FirstContactCommand::Upgrade => "N",
                FirstContactCommand::Patrol => "P",
                FirstContactCommand::Stop => "SPACE",
                FirstContactCommand::Retreat => "X",
            },
            next.label()
        );
    }
    for mut text in &mut rosters {
        let selection = if runtime.selected_slots.is_empty() {
            "ALL 4".to_string()
        } else {
            runtime
                .selected_slots
                .iter()
                .map(|slot| (slot + 1).to_string())
                .collect::<Vec<_>>()
                .join(",")
        };
        text.0 =
            format!(
            "SEL {selection} | {} | G{} | {}\nDRAG | CTRL+1-9 SET | 1-9 GET\nP PATROL | SPACE STOP | HP {}% | {} / {}",
            runtime.formation.id().trim_start_matches("party_").to_uppercase(),
            runtime
                .active_control_group
                .map(|group| group.to_string())
                .unwrap_or_else(|| "-".to_string()),
            runtime.selected_stance.as_str().to_uppercase(),
            runtime.party_hp_percent,
            if runtime.production_variant == 0 {
                "DRONE"
            } else {
                "MEDIC"
            },
            match runtime.structure_variant {
                0 => "BARRICADE",
                1 => "GENERATOR",
                2 => "WORKSHOP",
                _ => "SUPPLY",
            },
        );
    }
    let minutes = runtime.elapsed_seconds as u32 / 60;
    let seconds = runtime.elapsed_seconds as u32 % 60;
    for mut text in &mut clocks {
        text.0 = format!("{minutes:02}:{seconds:02}");
    }
    for (card, mut background, mut border) in &mut cards {
        let selected = card.command == runtime.recommended_command();
        background.0 = if selected {
            Color::srgba(0.14, 0.30, 0.24, 0.98)
        } else {
            Color::srgba(0.055, 0.085, 0.082, 0.96)
        };
        *border = BorderColor::all(if selected {
            Color::srgb(0.58, 0.96, 0.48)
        } else {
            Color::srgb(0.22, 0.33, 0.31)
        });
    }
    let world_width = map.width as f32 * map.tile_size as f32;
    let world_height = map.height as f32 * map.tile_size as f32;
    let minimap = world_to_minimap(
        runtime.group_world_position,
        Vec2::new(world_width, world_height),
        Vec2::new(164.0, 98.0),
    );
    for mut marker in &mut radar_markers {
        marker.left = px(8.0 + minimap.x);
        marker.bottom = px(8.0 + minimap.y);
    }
}
