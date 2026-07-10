use super::campaign_flow::{CampaignFlow, CampaignMode};
use bevy::prelude::*;
use bevy::window::RequestRedraw;
use trnm_campaign_core::{CampaignRoom, QuestState};

#[derive(Component)]
pub(super) struct CampaignOverlayRoot;

#[derive(Component)]
pub(super) struct CampaignPanel;

#[derive(Component)]
pub(super) struct CampaignTitle;

#[derive(Component)]
pub(super) struct CampaignBody;

#[derive(Component)]
pub(super) struct CampaignActions;

#[derive(Component)]
pub(super) struct CampaignStatus;

fn spawn_campaign_ui_root(commands: &mut Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: px(0),
            top: px(0),
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: px(18),
            padding: UiRect::all(px(40)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.012, 0.025, 0.024)),
        GlobalZIndex(100),
        CampaignOverlayRoot,
        children![
            (
                Text::new("TRILLIONNIUM CAMPAIGN"),
                CampaignTitle,
                TextFont::from_font_size(34.0),
                TextColor(Color::srgb(0.95, 0.82, 0.42)),
            ),
            (
                Node {
                    width: px(820),
                    min_height: px(260),
                    padding: UiRect::all(px(28)),
                    border: UiRect::all(px(2)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.035, 0.070, 0.064, 0.98)),
                BorderColor::all(Color::srgb(0.25, 0.52, 0.42)),
                CampaignPanel,
                children![(
                    Text::new("Loading campaign..."),
                    CampaignBody,
                    TextFont::from_font_size(19.0),
                    TextColor(Color::srgb(0.88, 0.92, 0.78)),
                )],
            ),
            (
                Text::new("1 SQUARE  |  2 MENTOR  |  3 GATE  |  4 RELAY QUARTER"),
                CampaignActions,
                TextFont::from_font_size(18.0),
                TextColor(Color::srgb(0.62, 0.88, 0.70)),
            ),
            (
                Text::new("Campaign ready"),
                CampaignStatus,
                TextFont::from_font_size(15.0),
                TextColor(Color::srgb(0.72, 0.80, 0.76)),
            ),
        ],
    ));
}

pub(super) fn spawn_campaign_ui(mut commands: Commands) {
    spawn_campaign_ui_root(&mut commands);
}

fn room_label(room: CampaignRoom) -> &'static str {
    match room {
        CampaignRoom::MirrorSquare => "MIRROR SQUARE",
        CampaignRoom::MentorHall => "STREET COMPASS SIFU HALL",
        CampaignRoom::ExpeditionGate => "FIRST CONTACT EXPEDITION GATE",
        CampaignRoom::RelayQuarter => "RELAY QUARTER / SIGNAL ROAD",
    }
}

fn quest_label(state: QuestState) -> &'static str {
    match state {
        QuestState::Locked => "LOCKED",
        QuestState::Available => "AVAILABLE",
        QuestState::Accepted => "ACCEPTED",
        QuestState::Completed => "COMPLETED",
        QuestState::Failed => "FAILED - RETRY AVAILABLE",
        QuestState::Withdrawn => "WITHDRAWN - RETRY AVAILABLE",
    }
}

fn town_body(flow: &CampaignFlow) -> String {
    let save = &flow.save;
    match save.room {
        CampaignRoom::MirrorSquare => format!(
            "{}\n\n{}  |  LV {}  |  XP {}  |  CR {}  |  REP {}\n\nChoose a mentor path, equipment loadout and four-person party. H treats one injury level using a Field Tonic or {} credits; G equips recovered Relay Core loot.\n\nSTORY: {:?}  |  MISSION: {}  |  AFTERSHOCK WINS {}  |  SAVE REVISION {}",
            room_label(save.room),
            "MIRROR RANGER",
            save.progression.level,
            save.progression.experience,
            save.progression.credits,
            save.character.attributes.reputation,
            trnm_campaign_core::FIELD_CLINIC_CREDIT_COST,
            save.story.current_step,
            quest_label(save.quest_state),
            save.progression.aftershock_completions,
            save.revision,
        ),
        CampaignRoom::MentorHall => {
            let rank = save
                .progression
                .skill_progress
                .get(save.selected_training_path.skill_id())
                .map(|progress| progress.rank)
                .unwrap_or(0);
            format!(
                "{}\n\nMENTOR MET: {}  |  TRAINING COMPLETE: {}\nSELECTED PATH: {}  |  PATH RANK: {}\nSESSIONS: {}/{}  |  CREDITS: {}  |  FACTION: {:?}\n\nL cycles paths; K trains; Y runs the deterministic guard/inner-power/strike sparring bout. Sparring raises trust and can grant Disciple rank once.",
                room_label(save.room),
                save.mentor_met,
                save.trained_with_mentor,
                save.selected_training_path.display_name(),
                rank,
                save.progression.mentor_training_sessions,
                trnm_campaign_core::MAX_MENTOR_TRAINING_SESSIONS,
                save.progression.credits,
                save.faction_rank,
            )
        }
        CampaignRoom::ExpeditionGate => {
            let roster = save
                .active_party_ids
                .iter()
                .filter_map(|unit_id| {
                    save.party
                        .iter()
                        .find(|member| &member.unit_id == unit_id)
                })
                .map(|member| {
                    format!(
                        "{} / {} / injury {}",
                        member.display_name, member.role, member.injury_level
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");
            format!(
                "{}\n\nMISSION: {} / {}  |  LOADOUT: {}\nPARTY (hero + freely chosen companions):\n{}\n\nZ/X/C independently cycle companion slots 1/2/3; P still cycles valid presets. E cycles equipment. Brann is locked until Relay Quarter trust recruitment.",
                room_label(save.room),
                quest_label(save.quest_state),
                save.active_mission.display_name(),
                save.selected_loadout.display_name(),
                roster,
            )
        }
        CampaignRoom::RelayQuarter => format!(
            "{}\n\nThe route opened only after First Contact and Aftershock were both secured. T speaks with Relay Smith Brann; U recruits him once trust reaches 8.\n\nBRANN TRUST: {}  |  RECRUITED: {}  |  FACTION: {:?}\n\nSIGNAL ROAD FLAGS: {:?}",
            room_label(save.room),
            save.npc_relationships
                .get("relay-smith-brann")
                .map(|relation| relation.trust)
                .unwrap_or(0),
            save.npc_relationships
                .get("relay-smith-brann")
                .is_some_and(|relation| relation.recruited),
            save.faction_rank,
            save.progression
                .world_flags
                .iter()
                .filter(|flag| flag.contains("contact") || flag.contains("aftershock") || flag.contains("signal_road"))
                .collect::<Vec<_>>(),
        ),
    }
}

fn debrief_body(flow: &CampaignFlow) -> String {
    let Some(receipt) = flow.last_receipt.as_ref() else {
        return "Battle settlement complete. Press ENTER to return to Mirror Square.".to_string();
    };
    let loot = if receipt.loot_delta.is_empty() {
        "none".to_string()
    } else {
        receipt
            .loot_delta
            .iter()
            .map(|item| format!("{} x{}", item.item_id, item.quantity))
            .collect::<Vec<_>>()
            .join(", ")
    };
    format!(
        "BATTLE SETTLEMENT\n\nOUTCOME: {:?}\nBATTLE ID: {}\nXP: +{}  |  CREDITS: {:+}  |  REPUTATION: {:+}\nLOOT: {}\nINJURIES: {:?}\n\nCampaign revision {} is atomically saved. Withdrawal pays zero XP/resources; this battle id cannot award progression twice.",
        receipt.outcome,
        receipt.battle_id,
        receipt.experience_delta,
        receipt.credit_delta,
        receipt.reputation_delta,
        loot,
        receipt.injury_delta_by_unit,
        receipt.campaign_revision_after,
    )
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(super) fn update_campaign_ui(
    flow: Res<CampaignFlow>,
    mut redraw: MessageWriter<RequestRedraw>,
    mut roots: Query<&mut BackgroundColor, With<CampaignOverlayRoot>>,
    mut panels: Query<
        (&mut BackgroundColor, &mut BorderColor),
        (With<CampaignPanel>, Without<CampaignOverlayRoot>),
    >,
    mut titles: Query<
        (&mut Text, &mut TextColor),
        (
            With<CampaignTitle>,
            Without<CampaignBody>,
            Without<CampaignActions>,
            Without<CampaignStatus>,
        ),
    >,
    mut bodies: Query<
        (&mut Text, &mut TextColor),
        (
            With<CampaignBody>,
            Without<CampaignTitle>,
            Without<CampaignActions>,
            Without<CampaignStatus>,
        ),
    >,
    mut actions: Query<
        (&mut Text, &mut TextColor),
        (
            With<CampaignActions>,
            Without<CampaignTitle>,
            Without<CampaignBody>,
            Without<CampaignStatus>,
        ),
    >,
    mut statuses: Query<
        (&mut Text, &mut TextColor),
        (
            With<CampaignStatus>,
            Without<CampaignTitle>,
            Without<CampaignBody>,
            Without<CampaignActions>,
        ),
    >,
) {
    // Keep the live game surface repainting even when the window manager marks
    // the X11 software-rendered window unfocused. Without an explicit redraw,
    // a campaign -> battle or battle -> debrief transition can remain one
    // presentation frame behind its already-persisted authoritative state.
    redraw.write(RequestRedraw);
    let hidden = flow.mode == CampaignMode::Battle;
    for mut background in &mut roots {
        background.0 = if hidden {
            Color::NONE
        } else {
            Color::srgb(0.012, 0.025, 0.024)
        };
    }
    for (mut background, mut border) in &mut panels {
        background.0 = if hidden {
            Color::NONE
        } else {
            Color::srgba(0.035, 0.070, 0.064, 0.98)
        };
        *border = BorderColor::all(if hidden {
            Color::NONE
        } else {
            Color::srgb(0.25, 0.52, 0.42)
        });
    }
    for (mut title, mut color) in &mut titles {
        title.0 = if flow.mode == CampaignMode::Debrief {
            "RETURN TO THE OPEN WORLD".to_string()
        } else {
            room_label(flow.save.room).to_string()
        };
        color.0 = if hidden {
            Color::NONE
        } else {
            Color::srgb(0.95, 0.82, 0.42)
        };
    }
    for (mut body, mut color) in &mut bodies {
        body.0 = if flow.mode == CampaignMode::Debrief {
            debrief_body(&flow)
        } else {
            town_body(&flow)
        };
        color.0 = if hidden {
            Color::NONE
        } else {
            Color::srgb(0.88, 0.92, 0.78)
        };
    }
    for (mut action, mut color) in &mut actions {
        action.0 = if flow.mode == CampaignMode::Debrief {
            "ENTER  RETURN TO MIRROR SQUARE".to_string()
        } else {
            match flow.save.room {
                CampaignRoom::MirrorSquare => {
                    "1 SQUARE | 2 MENTOR | 3 GATE | 4 RELAY | E LOADOUT | P PARTY | H HEAL | G RELIC".to_string()
                }
                CampaignRoom::MentorHall => {
                    "T TALK | L PATH | K TRAIN | Y SPAR | E LOADOUT | H HEAL | 1 SQUARE | 3 GATE".to_string()
                }
                CampaignRoom::ExpeditionGate => {
                    "Z/X/C FREE PARTY | P PRESET | E LOADOUT | F ACCEPT/DEPLOY | 1 SQUARE | 2 MENTOR | 4 RELAY"
                        .to_string()
                }
                CampaignRoom::RelayQuarter => {
                    "T TALK BRANN | U RECRUIT | Z/X/C FREE PARTY | 1 SQUARE | 2 MENTOR | 3 GATE".to_string()
                }
            }
        };
        color.0 = if hidden {
            Color::NONE
        } else {
            Color::srgb(0.62, 0.88, 0.70)
        };
    }
    for (mut status, mut color) in &mut statuses {
        status.0 = flow.status.clone();
        color.0 = if hidden {
            Color::NONE
        } else {
            Color::srgb(0.72, 0.80, 0.76)
        };
    }
}
