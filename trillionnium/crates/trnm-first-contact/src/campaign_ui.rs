use super::campaign_flow::{
    CampaignFlow, CampaignMode, CampaignUiIntent, CampaignUiIntents, ShellMode,
};
use bevy::prelude::*;
use bevy::window::RequestRedraw;
use trnm_campaign_core::{
    CampaignGuideStep, CampaignRoom, EncounterAction, InputMode, MasteryChallenge, QuestState,
    SaveSlotId,
};

const CAMPAIGN_ACTION_BUTTON_COUNT: usize = 6;

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
pub(super) struct CampaignPrimaryActions;

#[derive(Component, Debug)]
pub(super) struct CampaignActionButton {
    slot: usize,
    intent: Option<CampaignUiIntent>,
    enabled: bool,
}

#[derive(Component)]
pub(super) struct CampaignActionButtonLabel {
    slot: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CampaignActionSpec {
    pub(super) label: String,
    pub(super) intent: CampaignUiIntent,
    pub(super) enabled: bool,
}

#[derive(Component)]
pub(super) struct CampaignStatus;

fn campaign_action_button(slot: usize) -> impl Bundle {
    (
        Button,
        Node {
            min_width: px(150),
            height: px(46),
            padding: UiRect::axes(px(16), px(8)),
            border: UiRect::all(px(2)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.055, 0.085, 0.082, 0.96)),
        BorderColor::all(Color::srgb(0.22, 0.33, 0.31)),
        CampaignActionButton {
            slot,
            intent: None,
            enabled: false,
        },
        children![(
            Text::new(String::new()),
            CampaignActionButtonLabel { slot },
            TextFont::from_font_size(15.0),
            TextColor(Color::srgb(0.88, 0.92, 0.78)),
        )],
    )
}

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
                Node {
                    width: px(820),
                    flex_wrap: FlexWrap::Wrap,
                    justify_content: JustifyContent::Center,
                    row_gap: px(10),
                    column_gap: px(10),
                    ..default()
                },
                CampaignPrimaryActions,
                children![
                    campaign_action_button(0),
                    campaign_action_button(1),
                    campaign_action_button(2),
                    campaign_action_button(3),
                    campaign_action_button(4),
                    campaign_action_button(5),
                ],
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
        CampaignRoom::CisternWard => "CISTERN WARD",
        CampaignRoom::NightWatchPost => "NIGHT WATCH POST",
        CampaignRoom::WorkshopGate => "IRON WORKSHOP GATE",
        CampaignRoom::MarketWindPavilion => "MARKET WIND PAVILION",
        CampaignRoom::LanternInfirmary => "LANTERN INFIRMARY",
        CampaignRoom::ArchiveSteps => "ARCHIVE STEPS",
        CampaignRoom::CaravanYard => "CARAVAN YARD",
        CampaignRoom::OuterSignalRoad => "OUTER SIGNAL ROAD",
        CampaignRoom::GlassBasinWayhouse => "GLASS BASIN WAYHOUSE",
        CampaignRoom::DeepRelay => "DEEP RELAY",
        CampaignRoom::GlassReedMarsh => "GLASS REED MARSH",
        CampaignRoom::BasinObservatory => "BASIN OBSERVATORY",
        CampaignRoom::MoonBridge => "MOON BRIDGE",
        CampaignRoom::EmberOrchardEdge => "EMBER ORCHARD EDGE",
        CampaignRoom::AshBeaconField => "ASH BEACON FIELD",
        CampaignRoom::CinderRefuge => "CINDER REFUGE",
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

pub(super) fn town_body(flow: &CampaignFlow) -> String {
    let save = &flow.save;
    let guide = save.current_guide_step();
    let route = save.current_task_route_plan();
    let navigation = if let Some(exit) = route.next_exit.as_ref() {
        format!(
            "NEXT EXIT: {} -> {} ({})",
            exit.from, exit.to, exit.direction
        )
    } else if let Some(reason) = route.blocked_reason.as_ref() {
        format!("ROUTE BLOCKED: {reason:?}")
    } else {
        "NEXT EXIT: objective is in this room".to_string()
    };
    if let Some(encounter) = &save.active_encounter {
        return format!(
            "REGIONAL RPG ENCOUNTER\n\nROUND {}  |  HERO HP {}/{}  |  ENEMY HP {}/{}\nMOMENTUM {}  |  TECHNIQUE {:?}  |  COOLDOWN {}\nHERO STATUS {:?}  |  ENEMY STATUS {:?}\nENEMY INTENT: {}\n\nAttack builds momentum, defend answers telegraphed moves, and K releases the equipped sect technique. Bleed/exposure/guard, item use, withdrawal, injury and loot remain authoritative.",
            encounter.round,
            encounter.player_hp.max(0),
            encounter.player_max_hp,
            encounter.enemy_hp.max(0),
            encounter.enemy_max_hp,
            encounter.momentum,
            encounter.technique_style,
            encounter.technique_cooldown,
            encounter.player_status,
            encounter.enemy_status,
            encounter.enemy_intent,
        );
    }
    let body = match save.room {
        CampaignRoom::MirrorSquare => format!(
            "{}\n\n{}  |  ORIGIN {}  |  LV {}  |  XP {}  |  CR {}  |  REP {}\nGUIDE: {}\nTIME {}  |  STAMINA {}  |  RATIONS {}  |  WATER {}\nGROWTH {}  |  PREVIEW {:?}  |  BUILD {:?}  |  TITLE {:?}\n{}\n\nO changes origin before mentor progress; A/S/D choose growth; Q earns the selected path title through its mastery challenge.\n\nSTORY: {:?}  |  MISSION: {}  |  AFTERSHOCK WINS {}  |  SAVE REVISION {}",
            room_label(save.room),
            save.character.display_name.to_ascii_uppercase(),
            save.character_origin.display_name(),
            save.progression.level,
            save.progression.experience,
            save.progression.credits,
            save.character.attributes.reputation,
            guide.prompt(),
            save.world_clock.label(),
            save.expedition_supplies.stamina,
            save.expedition_supplies.rations,
            save.expedition_supplies.water,
            save.progression.growth_points_available,
            save.pending_growth_stat,
            save.build_path,
            save.active_title,
            navigation,
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
                "{}\n\nMENTOR MET: {}  |  TRAINING COMPLETE: {}\nSELECTED PATH: {}  |  PATH RANK: {}\nSESSIONS: {}/{}  |  CREDITS: {}  |  FACTION: {:?}\n{}\n\nL cycles training; K trains; Y spars; Q attempts the typed {:?} mastery. Growth selects a path, but only mastery grants its title.",
                room_label(save.room),
                save.mentor_met,
                save.trained_with_mentor,
                save.selected_training_path.display_name(),
                rank,
                save.progression.mentor_training_sessions,
                trnm_campaign_core::MAX_MENTOR_TRAINING_SESSIONS,
                save.progression.credits,
                save.faction_rank,
                navigation,
                MasteryChallenge::for_path(save.build_path),
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
                        "{} / {} / injury {} / veteran {} / kills {}",
                        member.display_name,
                        member.role,
                        member.injury_level,
                        member.veteran_rank,
                        member.confirmed_kills,
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");
            let skirmish = if save.skirmish_setup.enabled {
                format!(
                    "\nSKIRMISH: {} vs {} | START {} | {:?} / score {} | seed {}",
                    save.skirmish_setup.player_faction.display_name(),
                    save.skirmish_setup.enemy_faction.display_name(),
                    save.skirmish_setup.starting_resources,
                    save.skirmish_setup.victory_mode,
                    save.skirmish_setup.score_target,
                    save.skirmish_setup.simulation_seed,
                )
            } else {
                String::new()
            };
            format!(
                "{}\n\nMISSION: {} / {}  |  DIFFICULTY: {}  |  LOADOUT: {}\nGUIDE: {}\nPREPARATION: {}  |  TIME: {}\nSTAMINA {}  |  RATIONS {}  |  WATER {}{}\nPARTY (hero + freely chosen companions):\n{}\n\nR cycles preparation; F6 changes difficulty; F10 selects campaign/skirmish. In skirmish T changes faction, Y resources and U victory mode. Every choice is bound into the authoritative BattleSeed.",
                room_label(save.room),
                quest_label(save.quest_state),
                save.active_mission.display_name(),
                save.difficulty.display_name(),
                save.selected_loadout.display_name(),
                guide.prompt(),
                save.selected_expedition_preparation.display_name(),
                save.world_clock.label(),
                save.expedition_supplies.stamina,
                save.expedition_supplies.rations,
                save.expedition_supplies.water,
                skirmish,
                roster,
            )
        }
        CampaignRoom::RelayQuarter => format!(
            "{}\n\nThe route opened only after First Contact and Aftershock were both secured. T speaks with Relay Smith Brann; U recruits him once trust reaches 8; J begins the typed Signal Road ambush.\n\nCISTERN RELIEF: {:?}\nB advances its typed nodes; at the final choice N reinforces the cistern, M evacuates families.\n\nBRANN TRUST: {}  |  RECRUITED: {}  |  FACTION: {:?}  |  LAST ENCOUNTER: {:?}\n\nSIGNAL ROAD FLAGS: {:?}",
            room_label(save.room),
            save.quest_chain,
            save.npc_relationships
                .get("relay-smith-brann")
                .map(|relation| relation.trust)
                .unwrap_or(0),
            save.npc_relationships
                .get("relay-smith-brann")
                .is_some_and(|relation| relation.recruited),
            save.faction_rank,
            save.last_encounter_outcome,
            save.progression
                .world_flags
                .iter()
                .filter(|flag| flag.contains("contact") || flag.contains("aftershock") || flag.contains("signal_road"))
                .collect::<Vec<_>>(),
        ),
        room => {
            let npc = save
                .current_regional_npc_summary()
                .unwrap_or_else(|| "No scheduled regional NPC in this room".to_string());
            let dialogue = save
                .last_npc_conversation
                .as_ref()
                .map(|record| format!("LAST WORD ({}): {}", record.npc_id, record.line))
                .unwrap_or_else(|| "LAST WORD: talk with T to learn this NPC's current concern".to_string());
            let quest = save
                .active_regional_quest_objective()
                .unwrap_or_else(|| "No regional quest active".to_string());
            let commerce = if save.current_market_region_id().is_some() {
                format!("REGIONAL SHOP: {}", save.shop_selection_label())
            } else if room == CampaignRoom::WorkshopGate {
                format!("RECIPE: {}", save.recipe_selection_label())
            } else {
                "F11 cycles equipped owned items outside shop/workshop rooms".to_string()
            };
            let story = flow
                .save
                .main_story_ending
                .map(|ending| format!(
                    "ENDING SCENE: {}\nPOST-ENDING WORLD: {}\nEPILOGUE: {}/3 ({})",
                    ending.label(),
                    flow.save.post_ending_world_state.as_deref().unwrap_or("pending"),
                    flow.save.ending_epilogue_progress,
                    if flow.save.ending_epilogue_complete { "complete" } else { "playable" },
                ))
                .unwrap_or_else(|| {
                    let scene_step = flow
                        .save
                        .pending_main_story_chapter
                        .and_then(|pending| trnm_campaign_core::MAIN_STORY_CHAPTERS
                            .iter()
                            .find(|chapter| chapter.chapter == pending))
                        .and_then(|chapter| flow.save.main_story_scene_progress.get(chapter.scene_id))
                        .copied()
                        .unwrap_or_default();
                    format!(
                        "CHAPTER: {:?} | pending scene: {:?} | scene beat: {}/2 | next resolution: {:?}",
                        flow.save.main_story_chapter,
                        flow.save.pending_main_story_chapter,
                        scene_step,
                        flow.save.main_story_choice
                    )
                });
            let caravan = save
                .visible_regional_caravan()
                .map(|caravan| format!(
                    "VISIBLE CARAVAN: {} | {} x{} | {} -> {} | integrity {} | risk {} | {:?}\nCtrl+Shift+C escorts it; repeat after escort to intercept it.",
                    caravan.caravan_id,
                    caravan.item_id,
                    caravan.quantity,
                    caravan.from_region_id,
                    caravan.to_region_id,
                    caravan.integrity,
                    caravan.risk,
                    caravan.incident,
                ))
                .unwrap_or_else(|| "VISIBLE CARAVAN: none in this room".to_string());
            format!(
                "{}\n\nMIRROR CITY REGIONAL DISTRICT\n\nNPC: {}\n{}\n\nQUEST: {}\n{}\n\n{}\n{}\n{}\n\nT talks; Shift+T changes dialogue intent; Ctrl+T joins the sect in a sect hall. F9 accepts, N walks one legal route edge, F10 resolves the graph-ready node, Ctrl+F10 abandons it for a retry, and R changes Direct/Diplomatic/Resourceful resolution. Shift+B selects a chapter outcome; Ctrl+Shift+B advances its authored scene/epilogue. W advances schedules. Shift+K / Ctrl+Shift+K choose techniques. F11 cycles; Shift+F11 buys/crafts; Ctrl+F11 sells; F12 repairs.",
                room_label(room), npc, dialogue, quest, navigation, commerce, story, caravan,
            )
        }
    };
    let body = format!(
        "{body}\n\nECONOMY: {:?} | soft {} | wallet available {} / reserved {} | outbox {} + priority compensation {} | receipts {} | value events {} | dead letters {}\nCtrl+F7 binds/reconciles; Ctrl+Shift+F7 starts an escrow-backed tradeable purchase; Ctrl+Shift+F8 cancels the latest purchase through the priority compensation lane (Ctrl+Alt+F7 remains a compatibility chord). Local-only, wallet-only and explicit capped dual-track rewards are audited separately. RTS resources and bound items remain local authority; public player listings remain release-gated.",
        save.economy_mode,
        save.progression.credits,
        save.wallet_snapshot.available_credits,
        save.wallet_snapshot.reserved_credits,
        save.pending_economic_intents.len(),
        save.pending_economic_compensations.len(),
        save.verified_economic_receipts.len(),
        save.value_events.len(),
        save.economic_dead_letters.len(),
    );
    if flow.settings.subtitles && !save.combat_log.is_empty() {
        let captions = save
            .combat_log
            .iter()
            .map(|beat| format!("[{}] {}", beat.kind.to_ascii_uppercase(), beat.text))
            .collect::<Vec<_>>()
            .join("\n");
        format!("{body}\n\nCOMBAT CAPTIONS\n{captions}")
    } else {
        body
    }
}

fn journal_body(flow: &CampaignFlow) -> String {
    let rows = flow
        .save
        .campaign_journal()
        .into_iter()
        .map(|entry| {
            format!(
                "{:?}  [{:?}]\n  {}\n  NEXT: {}",
                entry.id,
                entry.state,
                entry.objective,
                entry
                    .next_room
                    .map(room_label)
                    .unwrap_or("NO FURTHER ROUTE"),
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");
    format!(
        "CAMPAIGN JOURNAL\n\n{}\n\nCURRENT GUIDE\n{}",
        rows,
        flow.save.current_guide_step().prompt()
    )
}

fn shell_body(flow: &CampaignFlow) -> String {
    match flow.shell_mode {
        ShellMode::Title => {
            let slots = flow
                .slot_metadata()
                .into_iter()
                .map(|meta| {
                    let marker = if meta.slot == flow.selected_slot { ">" } else { " " };
                    if !meta.exists {
                        format!("{marker} SLOT {}  EMPTY", meta.slot.label())
                    } else if meta.valid {
                        let phase = meta
                            .phase
                            .map(|phase| format!("{phase:?}"))
                            .unwrap_or_else(|| "UNKNOWN".to_string());
                        let mission = meta
                            .mission
                            .map(|mission| format!("{mission:?}"))
                            .unwrap_or_else(|| "UNKNOWN".to_string());
                        format!(
                            "{marker} SLOT {}  REV {}  {}  {}",
                            meta.slot.label(),
                            meta.revision.unwrap_or_default(),
                            phase,
                            mission,
                        )
                    } else {
                        format!("{marker} SLOT {}  CORRUPT: {}", meta.slot.label(), meta.error.unwrap_or_default())
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            format!(
                "THREE INDEPENDENT CAMPAIGN SLOTS\n\n{}\n\nSelected: {}  |  Low motion: {}  |  Input: {:?}\nSubtitles: {}  |  Contrast: {}  |  Controls: {:?}  |  Audio: {}%\n\nA corrupt slot is isolated and cannot poison another slot. NEW requires a second explicit N before overwriting an occupied slot.",
                slots,
                flow.selected_slot.label(),
                flow.settings.low_motion,
                flow.settings.input_mode,
                flow.settings.subtitles,
                flow.settings.high_contrast,
                flow.settings.control_scheme,
                flow.settings.master_volume_percent,
            )
        }
        ShellMode::ResumeGuard => format!(
            "SAVE STATE RESTORED\n\nSlot {} passed validation. Campaign phase: {:?}. Battle checkpoints, when present, were matched against the authoritative seed before loading.\n\nPress ENTER to reveal and resume gameplay.",
            flow.active_slot.label(), flow.save.phase,
        ),
        ShellMode::Paused => format!(
            "PAUSED\n\nSlot {}  |  Tick {}\nLow motion: {}  |  Input: {:?}\nSubtitles: {}  |  Controls: {:?}  |  Audio: {}%\n\nAuthoritative RTS ticks, commands, mouse selection and camera input are stopped. Settings are stored in player-settings.json, outside every character slot.",
            flow.active_slot.label(),
            flow.mission.as_ref().map(|mission| mission.tick).unwrap_or_default(),
            flow.settings.low_motion,
            flow.settings.input_mode,
            flow.settings.subtitles,
            flow.settings.control_scheme,
            flow.settings.master_volume_percent,
        ),
        ShellMode::CharacterCreate => format!(
            "CREATE CHARACTER IDENTITY\n\nNAME: {}\nORIGIN: selected later in Mirror Square\nSAVE SLOT: {}\n\nThe name is persisted on both the RPG character and party hero. Origin remains the existing independent gameplay choice.",
            flow.save.character_identity.name.display_name(),
            flow.active_slot.label(),
        ),
        ShellMode::SkirmishSetup => format!(
            "STANDALONE SKIRMISH\n\nMAP: {}\nPLAYER: {:?}  |  OPPONENT: {:?}\nSTARTING RESOURCES: {}\nVICTORY: {:?}  |  SCORE TARGET: {}\nSIMULATION SEED: {}\n\nM map | T factions | Y resources | U victory | I seed | Enter deploy\n\nThis lane is available from the title slot selector. It grants no campaign-completion flags, but its BattleSeed, deterministic simulation and one-time RPG settlement use the same authority as campaign battles.",
            flow.save.active_mission.display_name(),
            flow.save.skirmish_setup.player_faction,
            flow.save.skirmish_setup.enemy_faction,
            flow.save.skirmish_setup.starting_resources,
            flow.save.skirmish_setup.victory_mode,
            flow.save.skirmish_setup.score_target,
            flow.save.skirmish_setup.simulation_seed,
        ),
        ShellMode::Journal => journal_body(flow),
        ShellMode::ReplayBrowser => format!(
            "VERIFIED REPLAY TIMELINE\n\n{}\n\nFREE CAMERA: ({}, {})\n\nW/A/S/D pan | Space play/pause | Left/Right seek | Up speed 1x/2x/4x/8x | Enter full hash verification | Escape title.",
            flow.status,
            flow.replay_camera_x,
            flow.replay_camera_y,
        ),
        ShellMode::Playing => String::new(),
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

fn action_spec(label: impl Into<String>, intent: CampaignUiIntent) -> CampaignActionSpec {
    CampaignActionSpec {
        label: label.into(),
        intent,
        enabled: true,
    }
}

pub(super) fn campaign_action_specs(flow: &CampaignFlow) -> Vec<CampaignActionSpec> {
    let mut specs = match flow.shell_mode {
        ShellMode::Title => {
            let metadata = flow.slot_metadata();
            let selected = metadata.iter().find(|meta| meta.slot == flow.selected_slot);
            let can_continue = selected.is_some_and(|meta| meta.exists && meta.valid);
            let mut slots = SaveSlotId::ALL
                .into_iter()
                .map(|slot| CampaignActionSpec {
                    label: format!(
                        "{} SLOT {}",
                        if slot == flow.selected_slot {
                            "SELECTED"
                        } else {
                            "SELECT"
                        },
                        slot.label()
                    ),
                    intent: CampaignUiIntent::SelectSlot(slot),
                    enabled: slot != flow.selected_slot,
                })
                .collect::<Vec<_>>();
            slots.push(action_spec(
                if flow.overwrite_pending == Some(flow.selected_slot) {
                    format!("CONFIRM OVERWRITE SLOT {}", flow.selected_slot.label())
                } else {
                    format!("NEW CAMPAIGN — SLOT {}", flow.selected_slot.label())
                },
                CampaignUiIntent::CreateCampaign,
            ));
            slots.push(CampaignActionSpec {
                label: format!("CONTINUE SLOT {}", flow.selected_slot.label()),
                intent: CampaignUiIntent::ContinueCampaign,
                enabled: can_continue,
            });
            slots
        }
        ShellMode::CharacterCreate => vec![
            action_spec("CYCLE NAME", CampaignUiIntent::CycleCharacterName),
            action_spec("CONFIRM CHARACTER", CampaignUiIntent::ConfirmCharacter),
            action_spec("BACK TO TITLE", CampaignUiIntent::ReturnToTitle),
        ],
        ShellMode::ResumeGuard => vec![
            action_spec("RESUME CAMPAIGN", CampaignUiIntent::ResumeCampaign),
            action_spec("BACK TO TITLE", CampaignUiIntent::ReturnToTitle),
        ],
        ShellMode::Paused => vec![
            action_spec("RESUME", CampaignUiIntent::ResumeCampaign),
            action_spec("SAVE & RETURN TO TITLE", CampaignUiIntent::ReturnToTitle),
        ],
        ShellMode::Journal => vec![action_spec("CLOSE JOURNAL", CampaignUiIntent::CloseJournal)],
        ShellMode::SkirmishSetup | ShellMode::ReplayBrowser => vec![action_spec(
            "BACK TO TITLE",
            CampaignUiIntent::ReturnToTitle,
        )],
        ShellMode::Playing if flow.mode == CampaignMode::Debrief => vec![action_spec(
            "RETURN TO MIRROR SQUARE",
            CampaignUiIntent::ReturnToTown,
        )],
        ShellMode::Playing if flow.mode == CampaignMode::Battle => Vec::new(),
        ShellMode::Playing if flow.save.active_encounter.is_some() => vec![
            action_spec(
                "ATTACK",
                CampaignUiIntent::Encounter(EncounterAction::Attack),
            ),
            action_spec(
                "DEFEND",
                CampaignUiIntent::Encounter(EncounterAction::Defend),
            ),
            action_spec(
                "PRIMARY TECHNIQUE",
                CampaignUiIntent::Encounter(EncounterAction::PrimaryTechnique),
            ),
            action_spec(
                "USE TONIC",
                CampaignUiIntent::Encounter(EncounterAction::UseItem),
            ),
            action_spec(
                "WITHDRAW",
                CampaignUiIntent::Encounter(EncounterAction::Withdraw),
            ),
        ],
        ShellMode::Playing => match flow.save.current_guide_step() {
            CampaignGuideStep::MeetMentor if flow.save.room != CampaignRoom::MentorHall => {
                vec![action_spec(
                    "TRAVEL TO MENTOR HALL",
                    CampaignUiIntent::Travel(CampaignRoom::MentorHall),
                )]
            }
            CampaignGuideStep::MeetMentor => vec![action_spec(
                "TALK TO STREET COMPASS SIFU",
                CampaignUiIntent::TalkToMentor,
            )],
            CampaignGuideStep::TrainWithMentor if flow.save.room != CampaignRoom::MentorHall => {
                vec![action_spec(
                    "RETURN TO MENTOR HALL",
                    CampaignUiIntent::Travel(CampaignRoom::MentorHall),
                )]
            }
            CampaignGuideStep::TrainWithMentor => vec![action_spec(
                "COMPLETE MENTOR TRAINING",
                CampaignUiIntent::TrainWithMentor,
            )],
            CampaignGuideStep::EquipWeapon => vec![action_spec(
                "EQUIP STARTER LOADOUT",
                CampaignUiIntent::CycleLoadout,
            )],
            CampaignGuideStep::ReachExpeditionGate => vec![action_spec(
                "TRAVEL TO EXPEDITION GATE",
                CampaignUiIntent::Travel(CampaignRoom::ExpeditionGate),
            )],
            CampaignGuideStep::AcceptMission if flow.save.room != CampaignRoom::ExpeditionGate => {
                vec![action_spec(
                    "TRAVEL TO EXPEDITION GATE",
                    CampaignUiIntent::Travel(CampaignRoom::ExpeditionGate),
                )]
            }
            CampaignGuideStep::AcceptMission => vec![
                action_spec("ACCEPT MISSION", CampaignUiIntent::AcceptMission),
                action_spec("CHANGE PREPARATION", CampaignUiIntent::CyclePreparation),
                action_spec("CHANGE LOADOUT", CampaignUiIntent::CycleLoadout),
            ],
            CampaignGuideStep::DeployMission => vec![
                action_spec("DEPLOY TO RTS", CampaignUiIntent::DeployMission),
                action_spec("CHANGE PREPARATION", CampaignUiIntent::CyclePreparation),
                action_spec("CHANGE LOADOUT", CampaignUiIntent::CycleLoadout),
                action_spec("OPEN JOURNAL", CampaignUiIntent::OpenJournal),
            ],
            CampaignGuideStep::ReadJournal => vec![action_spec(
                "OPEN CAMPAIGN JOURNAL",
                CampaignUiIntent::OpenJournal,
            )],
        },
    };
    if flow.settings.input_mode == InputMode::KeyboardOnly {
        for spec in &mut specs {
            spec.enabled = false;
        }
    }
    specs.truncate(CAMPAIGN_ACTION_BUTTON_COUNT);
    specs
}

fn campaign_action_palette(
    interaction: Interaction,
    enabled: bool,
    high_contrast: bool,
) -> (Color, Color, Color) {
    if !enabled {
        return (
            Color::srgba(0.035, 0.045, 0.043, 0.82),
            Color::srgb(0.14, 0.18, 0.17),
            Color::srgb(0.36, 0.40, 0.38),
        );
    }
    match interaction {
        Interaction::Pressed => (
            Color::srgba(0.34, 0.55, 0.22, 1.0),
            Color::srgb(1.0, 0.93, 0.45),
            Color::WHITE,
        ),
        Interaction::Hovered => (
            Color::srgba(0.18, 0.38, 0.30, 1.0),
            Color::srgb(0.72, 1.0, 0.58),
            if high_contrast {
                Color::WHITE
            } else {
                Color::srgb(0.95, 0.98, 0.76)
            },
        ),
        Interaction::None => (
            Color::srgba(0.075, 0.13, 0.11, 0.98),
            if high_contrast {
                Color::WHITE
            } else {
                Color::srgb(0.31, 0.62, 0.47)
            },
            if high_contrast {
                Color::WHITE
            } else {
                Color::srgb(0.88, 0.92, 0.78)
            },
        ),
    }
}

pub(super) fn collect_campaign_ui_intents(
    mut intents: ResMut<CampaignUiIntents>,
    buttons: Query<(&Interaction, &CampaignActionButton), Changed<Interaction>>,
) {
    for (interaction, button) in &buttons {
        if *interaction == Interaction::Pressed && button.enabled {
            if let Some(intent) = button.intent {
                intents.push(intent);
            }
        }
    }
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
    mut action_buttons: Query<(
        &mut Node,
        &mut CampaignActionButton,
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
    )>,
    mut action_labels: Query<(&CampaignActionButtonLabel, &mut Text, &mut TextColor)>,
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
    let shell_visible = flow.shell_mode != ShellMode::Playing;
    let hidden = flow.mode == CampaignMode::Battle && !shell_visible;
    let action_specs = campaign_action_specs(&flow);
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
        title.0 = if flow.shell_mode == ShellMode::Title {
            "TRILLIONNIUM".to_string()
        } else if flow.shell_mode == ShellMode::CharacterCreate {
            "NEW CHARACTER".to_string()
        } else if flow.shell_mode == ShellMode::SkirmishSetup {
            "SKIRMISH SETUP".to_string()
        } else if flow.shell_mode == ShellMode::ReplayBrowser {
            "REPLAY BROWSER".to_string()
        } else if flow.shell_mode == ShellMode::Journal {
            "CAMPAIGN JOURNAL".to_string()
        } else if flow.shell_mode == ShellMode::Paused {
            "CAMPAIGN PAUSED".to_string()
        } else if flow.shell_mode == ShellMode::ResumeGuard {
            "RESUME CHECK".to_string()
        } else if flow.mode == CampaignMode::Debrief {
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
        body.0 = if shell_visible {
            shell_body(&flow)
        } else if flow.mode == CampaignMode::Debrief {
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
        action.0 = if flow.settings.input_mode == InputMode::MouseOnly {
            "MOUSE-ONLY: choose an available action below; disabled actions are visibly muted."
                .to_string()
        } else if flow.shell_mode == ShellMode::Title {
            "1/2/3 SLOT | N NEW | ENTER LOAD | K SKIRMISH | P REPLAY | F2 MOTION | F3 INPUT | F5 SUBTITLE/CONTRAST | F7 CONTROLS | F8 AUDIO".to_string()
        } else if flow.shell_mode == ShellMode::CharacterCreate {
            "C CYCLE NAME | ENTER CONFIRM | ESC TITLE".to_string()
        } else if flow.shell_mode == ShellMode::SkirmishSetup {
            "M MAP | T FACTIONS | Y RESOURCES | U VICTORY | I SEED | ENTER DEPLOY | ESC TITLE"
                .to_string()
        } else if flow.shell_mode == ShellMode::ReplayBrowser {
            "ENTER VERIFY REPLAY | ESC TITLE".to_string()
        } else if flow.shell_mode == ShellMode::Journal {
            "F4 / ESC CLOSE JOURNAL".to_string()
        } else if flow.shell_mode == ShellMode::ResumeGuard {
            "ENTER RESUME | F1 TITLE".to_string()
        } else if flow.shell_mode == ShellMode::Paused {
            "ESC RESUME | F1 TITLE | F2 MOTION | F3 INPUT | F5 SUBTITLE/CONTRAST | F7 CONTROLS | F8 AUDIO".to_string()
        } else if flow.mode == CampaignMode::Debrief {
            "ENTER  RETURN TO MIRROR SQUARE".to_string()
        } else if flow.save.active_encounter.is_some() {
            "J ATTACK | R DEFEND | K TECHNIQUE | I USE TONIC | ESC WITHDRAW".to_string()
        } else {
            match flow.save.room {
                CampaignRoom::MirrorSquare => {
                "O ORIGIN | A PREVIEW | S CONFIRM | D CANCEL | Q MASTERY | V TITLE | E LOADOUT | P PARTY | H HEAL | G RELIC | 1-4 TRAVEL".to_string()
                }
                CampaignRoom::MentorHall => {
                    "T TALK | L PATH | K TRAIN | Y SPAR | Q MASTERY | E LOADOUT | H HEAL | 1 SQUARE | 3 GATE".to_string()
                }
                CampaignRoom::ExpeditionGate => {
                    "R PREP | F6 DIFFICULTY | F10 MAP/MODE | T FACTION | Y RESOURCES | U VICTORY | F4 JOURNAL | Z/X/C PARTY | E LOADOUT | F DEPLOY"
                        .to_string()
                }
                CampaignRoom::RelayQuarter => {
                    "B CISTERN RELIEF | N REINFORCE | M EVACUATE | T TALK BRANN | U RECRUIT | J RPG AMBUSH | F1 TITLE | ESC PAUSE".to_string()
                }
                _ => "1-0/-/= TRAVEL | T TALK | SHIFT+T INTENT | R APPROACH | W WAIT | F9 ACCEPT | F10 STEP | F11 CYCLE | SHIFT+F11 BUY/CRAFT | CTRL+F11 SELL | F12 REPAIR".to_string(),
            }
        };
        color.0 = if hidden {
            Color::NONE
        } else {
            if flow.settings.high_contrast {
                Color::WHITE
            } else {
                Color::srgb(0.62, 0.88, 0.70)
            }
        };
    }
    for (mut node, mut button, interaction, mut background, mut border) in &mut action_buttons {
        let Some(spec) = action_specs.get(button.slot).filter(|_| !hidden) else {
            node.display = Display::None;
            button.intent = None;
            button.enabled = false;
            continue;
        };
        node.display = Display::Flex;
        button.intent = Some(spec.intent);
        button.enabled = spec.enabled;
        let (background_color, border_color, _) =
            campaign_action_palette(*interaction, spec.enabled, flow.settings.high_contrast);
        background.0 = background_color;
        *border = BorderColor::all(border_color);
    }
    for (label, mut text, mut color) in &mut action_labels {
        if let Some(spec) = action_specs.get(label.slot).filter(|_| !hidden) {
            text.0 = spec.label.clone();
            let (_, _, text_color) = campaign_action_palette(
                Interaction::None,
                spec.enabled,
                flow.settings.high_contrast,
            );
            color.0 = text_color;
        } else {
            text.0.clear();
            color.0 = Color::NONE;
        }
    }
    for (mut status, mut color) in &mut statuses {
        status.0 = flow.status.clone();
        color.0 = if hidden {
            Color::NONE
        } else {
            if flow.settings.high_contrast {
                Color::srgb(1.0, 0.92, 0.35)
            } else {
                Color::srgb(0.72, 0.80, 0.76)
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn campaign_action_is_a_real_button_and_emits_only_when_enabled() {
        let mut app = App::new();
        app.init_resource::<CampaignUiIntents>()
            .add_systems(Update, collect_campaign_ui_intents);
        let entity = app.world_mut().spawn(campaign_action_button(0)).id();
        assert!(app.world().get::<Button>(entity).is_some());
        {
            let mut button = app
                .world_mut()
                .get_mut::<CampaignActionButton>(entity)
                .unwrap();
            button.intent = Some(CampaignUiIntent::ConfirmCharacter);
            button.enabled = false;
        }
        *app.world_mut().get_mut::<Interaction>(entity).unwrap() = Interaction::Pressed;
        app.update();
        assert_eq!(
            app.world_mut()
                .resource_mut::<CampaignUiIntents>()
                .take_first(),
            None
        );

        *app.world_mut().get_mut::<Interaction>(entity).unwrap() = Interaction::None;
        app.update();
        app.world_mut()
            .get_mut::<CampaignActionButton>(entity)
            .unwrap()
            .enabled = true;
        *app.world_mut().get_mut::<Interaction>(entity).unwrap() = Interaction::Pressed;
        app.update();
        assert_eq!(
            app.world_mut()
                .resource_mut::<CampaignUiIntents>()
                .take_first(),
            Some(CampaignUiIntent::ConfirmCharacter)
        );
    }

    #[test]
    fn campaign_action_palette_distinguishes_disabled_hover_and_press() {
        let disabled = campaign_action_palette(Interaction::None, false, false);
        let normal = campaign_action_palette(Interaction::None, true, false);
        let hovered = campaign_action_palette(Interaction::Hovered, true, false);
        let pressed = campaign_action_palette(Interaction::Pressed, true, false);
        assert_ne!(disabled, normal);
        assert_ne!(normal, hovered);
        assert_ne!(hovered, pressed);
    }
}
