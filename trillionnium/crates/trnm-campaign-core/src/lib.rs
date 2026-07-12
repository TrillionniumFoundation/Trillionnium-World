//! Authoritative RPG -> RTS -> RPG campaign contracts.
//!
//! This crate is deliberately independent from Bevy. Presentation clients may
//! request a [`BattleSeedV1`] and submit a [`BattleResultV1`], but only this
//! aggregate is allowed to mutate persistent RPG progression.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt, fs,
    io::Write,
    path::{Path, PathBuf},
};
pub use trnm_economy_protocol::{
    ActorRef as EconomyActorRef, AssetRef as EconomyAssetRef, EconomicIntent, EconomicIntentKind,
    EconomicReceipt, EconomyAccountBinding, EconomyAssetClass, EconomyAssetSemantic,
    EconomyCurrencyClass, EconomyMode, EconomyTransferability,
    IdempotencyKey as EconomyIdempotencyKey, ReceiptProgressionClass, ReceiptStatus,
    SettlementBackendKind, WalletSnapshot, CEX_SETTLEMENT_BACKEND_ID, OFFLINE_LOCAL_BACKEND_ID,
    TERM_EXCHANGE_PROTOCOL_VERSION,
};
use trnm_rpg_core::{
    inventory_item_for as trillionnium_inventory_item_for, market_price_with_state,
    mirror_city_world_graph, npc_choice_dialogue, npc_dialogue, npc_room_at, npc_schedule,
    npc_social_event, original_combat_log, quest_condition_graph, quest_narrative,
    quest_resolution_text, quest_runtime_rule, resolve_mentor_sparring, skill_unlockable,
    BuildPath, BuildTitle, Character as WorldTrillionniumCharacter, CharacterOrigin, CombatLogBeat,
    DialogueChoice, EncounterOutcome, EquipmentAffixCondition, FactionRank, GrowthStat,
    ItemCondition, NpcRelationship, QuestApproach, RelationshipAction, RelationshipStage,
    RpgEncounterState, SparringAction, SparringOutcome, SparringReport, TechniqueStyle,
    TrillionniumAttributes, WorldRoutePlan, ARCHIVE_STEPS_ROOM, ASH_BEACON_FIELD_ROOM,
    BASIN_OBSERVATORY_ROOM, CARAVAN_YARD_ROOM, CINDER_REFUGE_ROOM, CISTERN_WARD_ROOM,
    CRAFTING_RECIPES, DEEP_RELAY_ROOM, ECONOMY_ITEM_CATALOG, EMBER_ORCHARD_EDGE_ROOM,
    ENCOUNTER_CATALOG, EXPEDITION_GATE_ROOM, GLASS_BASIN_WAYHOUSE_ROOM, GLASS_REED_MARSH_ROOM,
    LANTERN_INFIRMARY_ROOM, MARKET_WIND_PAVILION_ROOM, MENTOR_HALL_ROOM, MIRROR_SQUARE_ROOM,
    MOON_BRIDGE_ROOM, NIGHT_WATCH_POST_ROOM, NPC_CATALOG, OUTER_SIGNAL_ROAD_ROOM,
    REGIONAL_QUEST_CATALOG, RELAY_QUARTER_ROOM, SECT_CATALOG, SKILL_CATALOG, WORKSHOP_GATE_ROOM,
};
pub use trnm_rpg_core::{EncounterAction, MasteryChallenge, SectId};

pub const CAMPAIGN_SAVE_CONTRACT: &str = "trnm_campaign_save_v1";
pub const BATTLE_SEED_CONTRACT: &str = "trnm_battle_seed_v8";
pub const BATTLE_RESULT_CONTRACT: &str = "trnm_battle_result_v2";
pub const SETTLEMENT_RECEIPT_CONTRACT: &str = "trnm_settlement_receipt_v1";
pub const FIRST_CONTACT_RULES_VERSION: &str = "first_contact_campaign_rules_v8";
pub const MAX_MENTOR_TRAINING_SESSIONS: u8 = 2;
pub const FIELD_CLINIC_CREDIT_COST: i64 = 40;

fn default_campaign_credits() -> i64 {
    260
}

fn legacy_campaign_schema_revision() -> u16 {
    1
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrainingPath {
    #[default]
    IronGuard,
    WindStep,
    InnerFlame,
}

impl TrainingPath {
    pub fn next(self) -> Self {
        match self {
            Self::IronGuard => Self::WindStep,
            Self::WindStep => Self::InnerFlame,
            Self::InnerFlame => Self::IronGuard,
        }
    }

    pub fn skill_id(self) -> &'static str {
        match self {
            Self::IronGuard => "iron_guard",
            Self::WindStep => "wind_step",
            Self::InnerFlame => "inner_flame",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::IronGuard => "Iron Guard",
            Self::WindStep => "Wind Step",
            Self::InnerFlame => "Inner Flame",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LoadoutPreset {
    #[default]
    Guard,
    Raider,
    Mystic,
}

impl LoadoutPreset {
    pub fn next(self) -> Self {
        match self {
            Self::Guard => Self::Raider,
            Self::Raider => Self::Mystic,
            Self::Mystic => Self::Guard,
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::Guard => "Route Guard",
            Self::Raider => "Night Raider",
            Self::Mystic => "Relay Mystic",
        }
    }

    fn item_ids(self) -> &'static [&'static str] {
        match self {
            Self::Guard => &["route-guard-staff", "street-compass-bracer"],
            Self::Raider => &["iron-workshop-blade", "night-watch-cloak"],
            Self::Mystic => &["market-wind-sword", "raid-signal-drum"],
        }
    }
}

#[derive(Debug)]
pub enum CampaignError {
    InvalidState(String),
    InvalidContract(String),
    Integrity(String),
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl fmt::Display for CampaignError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidState(message) => write!(formatter, "invalid campaign state: {message}"),
            Self::InvalidContract(message) => write!(formatter, "invalid contract: {message}"),
            Self::Integrity(message) => write!(formatter, "campaign integrity error: {message}"),
            Self::Io(error) => write!(formatter, "campaign storage error: {error}"),
            Self::Json(error) => write!(formatter, "campaign JSON error: {error}"),
        }
    }
}

impl std::error::Error for CampaignError {}

impl From<std::io::Error> for CampaignError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for CampaignError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CampaignRoom {
    MirrorSquare,
    MentorHall,
    ExpeditionGate,
    RelayQuarter,
    CisternWard,
    NightWatchPost,
    WorkshopGate,
    MarketWindPavilion,
    LanternInfirmary,
    ArchiveSteps,
    CaravanYard,
    OuterSignalRoad,
    GlassBasinWayhouse,
    DeepRelay,
    GlassReedMarsh,
    BasinObservatory,
    MoonBridge,
    EmberOrchardEdge,
    AshBeaconField,
    CinderRefuge,
}

impl CampaignRoom {
    pub fn id(self) -> &'static str {
        match self {
            Self::MirrorSquare => MIRROR_SQUARE_ROOM,
            Self::MentorHall => MENTOR_HALL_ROOM,
            Self::ExpeditionGate => EXPEDITION_GATE_ROOM,
            Self::RelayQuarter => RELAY_QUARTER_ROOM,
            Self::CisternWard => CISTERN_WARD_ROOM,
            Self::NightWatchPost => NIGHT_WATCH_POST_ROOM,
            Self::WorkshopGate => WORKSHOP_GATE_ROOM,
            Self::MarketWindPavilion => MARKET_WIND_PAVILION_ROOM,
            Self::LanternInfirmary => LANTERN_INFIRMARY_ROOM,
            Self::ArchiveSteps => ARCHIVE_STEPS_ROOM,
            Self::CaravanYard => CARAVAN_YARD_ROOM,
            Self::OuterSignalRoad => OUTER_SIGNAL_ROAD_ROOM,
            Self::GlassBasinWayhouse => GLASS_BASIN_WAYHOUSE_ROOM,
            Self::DeepRelay => DEEP_RELAY_ROOM,
            Self::GlassReedMarsh => GLASS_REED_MARSH_ROOM,
            Self::BasinObservatory => BASIN_OBSERVATORY_ROOM,
            Self::MoonBridge => MOON_BRIDGE_ROOM,
            Self::EmberOrchardEdge => EMBER_ORCHARD_EDGE_ROOM,
            Self::AshBeaconField => ASH_BEACON_FIELD_ROOM,
            Self::CinderRefuge => CINDER_REFUGE_ROOM,
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::MirrorSquare => "镜城广场",
            Self::MentorHall => "街指南师父居",
            Self::ExpeditionGate => "First Contact 出征口",
            Self::RelayQuarter => "中继新街",
            Self::CisternWard => "蓄水坊",
            Self::NightWatchPost => "夜巡哨所",
            Self::WorkshopGate => "铁工门",
            Self::MarketWindPavilion => "市风阁",
            Self::LanternInfirmary => "灯火医馆",
            Self::ArchiveSteps => "镜档石阶",
            Self::CaravanYard => "行商院",
            Self::OuterSignalRoad => "外信号道",
            Self::GlassBasinWayhouse => "琉璃盆地驿",
            Self::DeepRelay => "深层中继站",
            Self::GlassReedMarsh => "琉璃苇泽",
            Self::BasinObservatory => "盆地观象台",
            Self::MoonBridge => "月镜桥",
            Self::EmberOrchardEdge => "烬果园边地",
            Self::AshBeaconField => "灰烬烽野",
            Self::CinderRefuge => "余烬避难所",
        }
    }

    pub fn from_id(id: &str) -> Option<Self> {
        [
            Self::MirrorSquare,
            Self::MentorHall,
            Self::ExpeditionGate,
            Self::RelayQuarter,
            Self::CisternWard,
            Self::NightWatchPost,
            Self::WorkshopGate,
            Self::MarketWindPavilion,
            Self::LanternInfirmary,
            Self::ArchiveSteps,
            Self::CaravanYard,
            Self::OuterSignalRoad,
            Self::GlassBasinWayhouse,
            Self::DeepRelay,
            Self::GlassReedMarsh,
            Self::BasinObservatory,
            Self::MoonBridge,
            Self::EmberOrchardEdge,
            Self::AshBeaconField,
            Self::CinderRefuge,
        ]
        .into_iter()
        .find(|room| room.id() == id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CampaignPhase {
    Town,
    BattlePending,
    PostBattlePending,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestState {
    Locked,
    Available,
    Accepted,
    Completed,
    Failed,
    Withdrawn,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CampaignMission {
    #[default]
    FirstContact,
    AftershockPatrol,
    ConvoyExodus,
    MirrorSiege,
    IronDeltaSkirmish,
    NightWatchCrossingSkirmish,
    GlassBasinSkirmish,
    EmberOrchardSkirmish,
    SaltMarshSkirmish,
    CinderCrownSkirmish,
}

impl CampaignMission {
    pub fn map_id(self) -> &'static str {
        match self {
            Self::FirstContact => "first_contact",
            Self::AftershockPatrol => "aftershock_patrol",
            Self::ConvoyExodus => "convoy_exodus",
            Self::MirrorSiege => "mirror_siege",
            Self::IronDeltaSkirmish => "iron_delta",
            Self::NightWatchCrossingSkirmish => "night_watch_crossing",
            Self::GlassBasinSkirmish => "glass_basin",
            Self::EmberOrchardSkirmish => "ember_orchard",
            Self::SaltMarshSkirmish => "salt_marsh",
            Self::CinderCrownSkirmish => "cinder_crown",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::FirstContact => "First Contact",
            Self::AftershockPatrol => "Aftershock Patrol",
            Self::ConvoyExodus => "Signal Convoy Exodus",
            Self::MirrorSiege => "Mirror Siege Counterstrike",
            Self::IronDeltaSkirmish => "Iron Delta Skirmish",
            Self::NightWatchCrossingSkirmish => "Night Watch Crossing",
            Self::GlassBasinSkirmish => "Glass Basin Control",
            Self::EmberOrchardSkirmish => "Ember Orchard Annihilation",
            Self::SaltMarshSkirmish => "Salt Marsh Divide",
            Self::CinderCrownSkirmish => "Cinder Crown Siege",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CampaignDifficulty {
    Story,
    #[default]
    Standard,
    Veteran,
}

impl CampaignDifficulty {
    pub fn next(self) -> Self {
        match self {
            Self::Story => Self::Standard,
            Self::Standard => Self::Veteran,
            Self::Veteran => Self::Story,
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::Story => "Story",
            Self::Standard => "Standard",
            Self::Veteran => "Veteran",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CampaignFaction {
    #[default]
    MirrorCoalition,
    AshenCompact,
}

impl CampaignFaction {
    pub fn opponent(self) -> Self {
        match self {
            Self::MirrorCoalition => Self::AshenCompact,
            Self::AshenCompact => Self::MirrorCoalition,
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::MirrorCoalition => "Mirror Coalition",
            Self::AshenCompact => "Ashen Compact",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkirmishVictoryMode {
    #[default]
    Objective,
    Score,
    Annihilation,
}

impl SkirmishVictoryMode {
    pub fn next(self) -> Self {
        match self {
            Self::Objective => Self::Score,
            Self::Score => Self::Annihilation,
            Self::Annihilation => Self::Objective,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkirmishSetup {
    pub enabled: bool,
    pub player_faction: CampaignFaction,
    pub enemy_faction: CampaignFaction,
    pub starting_resources: u32,
    pub victory_mode: SkirmishVictoryMode,
    pub score_target: u32,
    #[serde(default = "default_skirmish_simulation_seed")]
    pub simulation_seed: u64,
}

fn default_skirmish_simulation_seed() -> u64 {
    1
}

impl Default for SkirmishSetup {
    fn default() -> Self {
        Self {
            enabled: false,
            player_faction: CampaignFaction::MirrorCoalition,
            enemy_faction: CampaignFaction::AshenCompact,
            starting_resources: 300,
            victory_mode: SkirmishVictoryMode::Objective,
            score_target: 800,
            simulation_seed: default_skirmish_simulation_seed(),
        }
    }
}

impl SkirmishSetup {
    pub fn validate(&self, map_id: &str) -> Result<(), CampaignError> {
        if !self.enabled {
            return Ok(());
        }
        if !matches!(
            map_id,
            "iron_delta"
                | "night_watch_crossing"
                | "glass_basin"
                | "ember_orchard"
                | "salt_marsh"
                | "cinder_crown"
        ) {
            return Err(CampaignError::InvalidContract(
                "skirmish setup requires an authored skirmish map".to_string(),
            ));
        }
        if self.player_faction == self.enemy_faction
            || !(100..=1_000).contains(&self.starting_resources)
            || !(40..=5_000).contains(&self.score_target)
        {
            return Err(CampaignError::InvalidContract(
                "skirmish factions, resources or score target are invalid".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CharacterNamePreset {
    #[default]
    MirrorRanger,
    SignalRook,
    EmberVale,
}

impl CharacterNamePreset {
    pub fn next(self) -> Self {
        match self {
            Self::MirrorRanger => Self::SignalRook,
            Self::SignalRook => Self::EmberVale,
            Self::EmberVale => Self::MirrorRanger,
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::MirrorRanger => "Mirror Ranger",
            Self::SignalRook => "Signal Rook",
            Self::EmberVale => "Ember Vale",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharacterIdentity {
    pub name: CharacterNamePreset,
    pub confirmed: bool,
}

impl Default for CharacterIdentity {
    fn default() -> Self {
        Self {
            name: CharacterNamePreset::MirrorRanger,
            confirmed: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectiveKind {
    Destroy,
    Capture,
    Defend,
    Escort,
    Extract,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MissionObjectiveDefinition {
    pub id: String,
    pub kind: ObjectiveKind,
    pub target: BattleGridPoint,
    pub duration_ticks: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MissionDefinition {
    pub mission: CampaignMission,
    pub map_id: String,
    pub title: String,
    pub objectives: Vec<MissionObjectiveDefinition>,
}

impl MissionDefinition {
    pub fn for_mission(mission: CampaignMission, map: &BattleMapSeedV1) -> Self {
        let objectives = match mission {
            CampaignMission::FirstContact | CampaignMission::AftershockPatrol => vec![
                MissionObjectiveDefinition {
                    id: "reach_contact_line".to_string(),
                    kind: ObjectiveKind::Escort,
                    target: map.approach_point,
                    duration_ticks: 0,
                },
                MissionObjectiveDefinition {
                    id: "break_relay_guard".to_string(),
                    kind: ObjectiveKind::Destroy,
                    target: map.objective,
                    duration_ticks: 0,
                },
                MissionObjectiveDefinition {
                    id: "secure_relay".to_string(),
                    kind: ObjectiveKind::Capture,
                    target: map.objective,
                    duration_ticks: 602,
                },
            ],
            CampaignMission::ConvoyExodus => vec![
                MissionObjectiveDefinition {
                    id: "escort_supply_convoy".to_string(),
                    kind: ObjectiveKind::Escort,
                    target: map.approach_point,
                    duration_ticks: 0,
                },
                MissionObjectiveDefinition {
                    id: "hold_signal_generator".to_string(),
                    kind: ObjectiveKind::Defend,
                    target: map.approach_point,
                    duration_ticks: 260,
                },
                MissionObjectiveDefinition {
                    id: "extract_at_north_gate".to_string(),
                    kind: ObjectiveKind::Extract,
                    target: map.objective,
                    duration_ticks: 80,
                },
            ],
            CampaignMission::MirrorSiege => vec![
                MissionObjectiveDefinition {
                    id: "breach_siege_perimeter".to_string(),
                    kind: ObjectiveKind::Escort,
                    target: map.approach_point,
                    duration_ticks: 0,
                },
                MissionObjectiveDefinition {
                    id: "destroy_signal_jammer".to_string(),
                    kind: ObjectiveKind::Destroy,
                    target: map.objective,
                    duration_ticks: 0,
                },
                MissionObjectiveDefinition {
                    id: "reclaim_mirror_gate".to_string(),
                    kind: ObjectiveKind::Capture,
                    target: map.objective,
                    duration_ticks: 720,
                },
            ],
            CampaignMission::IronDeltaSkirmish => vec![
                MissionObjectiveDefinition {
                    id: "take_delta_crossing".to_string(),
                    kind: ObjectiveKind::Escort,
                    target: map.approach_point,
                    duration_ticks: 0,
                },
                MissionObjectiveDefinition {
                    id: "destroy_ash_foundry".to_string(),
                    kind: ObjectiveKind::Destroy,
                    target: map.objective,
                    duration_ticks: 0,
                },
                MissionObjectiveDefinition {
                    id: "hold_iron_delta".to_string(),
                    kind: ObjectiveKind::Capture,
                    target: map.objective,
                    duration_ticks: 500,
                },
            ],
            CampaignMission::NightWatchCrossingSkirmish => vec![
                MissionObjectiveDefinition {
                    id: "screen_night_convoy".to_string(),
                    kind: ObjectiveKind::Escort,
                    target: map.approach_point,
                    duration_ticks: 0,
                },
                MissionObjectiveDefinition {
                    id: "break_ash_beacon".to_string(),
                    kind: ObjectiveKind::Destroy,
                    target: map.objective,
                    duration_ticks: 0,
                },
                MissionObjectiveDefinition {
                    id: "extract_watch_patrol".to_string(),
                    kind: ObjectiveKind::Extract,
                    target: map.objective,
                    duration_ticks: 80,
                },
            ],
            CampaignMission::GlassBasinSkirmish => vec![
                MissionObjectiveDefinition {
                    id: "cross_glass_flats".to_string(),
                    kind: ObjectiveKind::Escort,
                    target: map.approach_point,
                    duration_ticks: 0,
                },
                MissionObjectiveDefinition {
                    id: "disable_basin_array".to_string(),
                    kind: ObjectiveKind::Destroy,
                    target: map.objective,
                    duration_ticks: 0,
                },
                MissionObjectiveDefinition {
                    id: "hold_glass_basin".to_string(),
                    kind: ObjectiveKind::Capture,
                    target: map.objective,
                    duration_ticks: 560,
                },
            ],
            CampaignMission::EmberOrchardSkirmish => vec![
                MissionObjectiveDefinition {
                    id: "enter_ember_orchard".to_string(),
                    kind: ObjectiveKind::Escort,
                    target: map.approach_point,
                    duration_ticks: 0,
                },
                MissionObjectiveDefinition {
                    id: "break_orchard_command".to_string(),
                    kind: ObjectiveKind::Destroy,
                    target: map.objective,
                    duration_ticks: 0,
                },
                MissionObjectiveDefinition {
                    id: "clear_orchard_base".to_string(),
                    kind: ObjectiveKind::Capture,
                    target: map.objective,
                    duration_ticks: 620,
                },
            ],
            CampaignMission::SaltMarshSkirmish => vec![
                MissionObjectiveDefinition {
                    id: "cross_salt_causeway".to_string(),
                    kind: ObjectiveKind::Escort,
                    target: map.approach_point,
                    duration_ticks: 0,
                },
                MissionObjectiveDefinition {
                    id: "break_marsh_pump".to_string(),
                    kind: ObjectiveKind::Destroy,
                    target: map.objective,
                    duration_ticks: 0,
                },
                MissionObjectiveDefinition {
                    id: "hold_salt_divide".to_string(),
                    kind: ObjectiveKind::Capture,
                    target: map.objective,
                    duration_ticks: 580,
                },
            ],
            CampaignMission::CinderCrownSkirmish => vec![
                MissionObjectiveDefinition {
                    id: "breach_cinder_ring".to_string(),
                    kind: ObjectiveKind::Escort,
                    target: map.approach_point,
                    duration_ticks: 0,
                },
                MissionObjectiveDefinition {
                    id: "destroy_crown_command".to_string(),
                    kind: ObjectiveKind::Destroy,
                    target: map.objective,
                    duration_ticks: 0,
                },
                MissionObjectiveDefinition {
                    id: "secure_cinder_crown".to_string(),
                    kind: ObjectiveKind::Capture,
                    target: map.objective,
                    duration_ticks: 640,
                },
            ],
        };
        Self {
            mission,
            map_id: mission.map_id().to_string(),
            title: mission.display_name().to_string(),
            objectives,
        }
    }

    pub fn validate(&self, map: &BattleMapSeedV1) -> Result<(), CampaignError> {
        if self.map_id != self.mission.map_id() || self.objectives.len() < 2 {
            return Err(CampaignError::InvalidContract(
                "mission definition identity/objectives are invalid".to_string(),
            ));
        }
        let ids = self
            .objectives
            .iter()
            .map(|objective| objective.id.as_str())
            .collect::<BTreeSet<_>>();
        if ids.len() != self.objectives.len()
            || self.objectives.iter().any(|objective| {
                objective.id.trim().is_empty()
                    || !map.in_bounds(objective.target)
                    || !map.passable(objective.target)
                    || matches!(
                        objective.kind,
                        ObjectiveKind::Defend | ObjectiveKind::Capture | ObjectiveKind::Extract
                    ) && objective.duration_ticks == 0
            })
        {
            return Err(CampaignError::InvalidContract(
                "mission objective is invalid".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StoryQuestId {
    #[default]
    SignalRoad,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StoryStepId {
    #[default]
    MeetMentor,
    SecureFirstContact,
    BreakAftershock,
    EvacuateConvoy,
    SignalRoadComplete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestChainId {
    CisternRelief,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestChainNodeId {
    SurveyDamage,
    GatherSupplies,
    ChooseReliefPlan,
    ReliefComplete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestBranch {
    ReinforceCistern,
    EvacuateFamilies,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum QuestChainCondition {
    WorldFlag { flag: String },
    MinimumCredits { credits: i64 },
    AtRoom { room_id: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum QuestChainReward {
    Credits { amount: i64 },
    Reputation { amount: i32 },
    WorldFlag { flag: String },
    RelationshipTrust { npc_id: String, amount: i16 },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuestChainNodeDefinition {
    pub id: QuestChainNodeId,
    pub room_id: String,
    pub conditions: Vec<QuestChainCondition>,
    pub next: Vec<QuestChainNodeId>,
    pub branch: Option<QuestBranch>,
    pub rewards: Vec<QuestChainReward>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuestChainDefinition {
    pub id: QuestChainId,
    pub title: String,
    pub nodes: Vec<QuestChainNodeDefinition>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuestChainProgress {
    pub id: QuestChainId,
    pub current_node: QuestChainNodeId,
    #[serde(default)]
    pub chosen_branch: Option<QuestBranch>,
    #[serde(default)]
    pub completed_nodes: BTreeSet<QuestChainNodeId>,
    #[serde(default)]
    pub complete: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CampaignJournalId {
    SignalRoad,
    CisternRelief,
    Mastery,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CampaignJournalState {
    Locked,
    Available,
    Active,
    Completed,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CampaignJournalEntry {
    pub id: CampaignJournalId,
    pub title: String,
    pub state: CampaignJournalState,
    pub objective: String,
    pub next_room: Option<CampaignRoom>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CampaignGuideStep {
    MeetMentor,
    TrainWithMentor,
    EquipWeapon,
    ReachExpeditionGate,
    AcceptMission,
    DeployMission,
    ReadJournal,
}

impl CampaignGuideStep {
    pub fn prompt(self) -> &'static str {
        match self {
            Self::MeetMentor => "Press 2, then T to meet Street Compass Sifu",
            Self::TrainWithMentor => "Press K to complete mentor training",
            Self::EquipWeapon => "Press E to equip a typed loadout",
            Self::ReachExpeditionGate => "Press 3 to reach the expedition gate",
            Self::AcceptMission => "Press F to accept the current campaign mission",
            Self::DeployMission => "Review preparation, then press F to deploy",
            Self::ReadJournal => "Press F4 to review active, completed and locked goals",
        }
    }
}

pub fn cistern_relief_quest_chain_definition() -> QuestChainDefinition {
    QuestChainDefinition {
        id: QuestChainId::CisternRelief,
        title: "Signal Cistern Relief".to_string(),
        nodes: vec![
            QuestChainNodeDefinition {
                id: QuestChainNodeId::SurveyDamage,
                room_id: RELAY_QUARTER_ROOM.to_string(),
                conditions: vec![QuestChainCondition::WorldFlag {
                    flag: "outer_signal_road_open".to_string(),
                }],
                next: vec![QuestChainNodeId::GatherSupplies],
                branch: None,
                rewards: Vec::new(),
            },
            QuestChainNodeDefinition {
                id: QuestChainNodeId::GatherSupplies,
                room_id: EXPEDITION_GATE_ROOM.to_string(),
                conditions: vec![QuestChainCondition::MinimumCredits { credits: 40 }],
                next: vec![QuestChainNodeId::ChooseReliefPlan],
                branch: None,
                rewards: Vec::new(),
            },
            QuestChainNodeDefinition {
                id: QuestChainNodeId::ChooseReliefPlan,
                room_id: RELAY_QUARTER_ROOM.to_string(),
                conditions: vec![QuestChainCondition::AtRoom {
                    room_id: RELAY_QUARTER_ROOM.to_string(),
                }],
                next: vec![QuestChainNodeId::ReliefComplete],
                branch: None,
                rewards: Vec::new(),
            },
            QuestChainNodeDefinition {
                id: QuestChainNodeId::ReliefComplete,
                room_id: RELAY_QUARTER_ROOM.to_string(),
                conditions: Vec::new(),
                next: Vec::new(),
                branch: Some(QuestBranch::ReinforceCistern),
                rewards: vec![
                    QuestChainReward::Reputation { amount: 4 },
                    QuestChainReward::RelationshipTrust {
                        npc_id: "relay-smith-brann".to_string(),
                        amount: 3,
                    },
                    QuestChainReward::WorldFlag {
                        flag: "cistern_reinforced".to_string(),
                    },
                ],
            },
            QuestChainNodeDefinition {
                id: QuestChainNodeId::ReliefComplete,
                room_id: RELAY_QUARTER_ROOM.to_string(),
                conditions: Vec::new(),
                next: Vec::new(),
                branch: Some(QuestBranch::EvacuateFamilies),
                rewards: vec![
                    QuestChainReward::Credits { amount: 35 },
                    QuestChainReward::Reputation { amount: 2 },
                    QuestChainReward::WorldFlag {
                        flag: "cistern_families_evacuated".to_string(),
                    },
                ],
            },
        ],
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpeditionPreparation {
    #[default]
    Immediate,
    Rested,
    Supplied,
    Shortcut,
}

impl ExpeditionPreparation {
    pub fn next(self) -> Self {
        match self {
            Self::Immediate => Self::Rested,
            Self::Rested => Self::Supplied,
            Self::Supplied => Self::Shortcut,
            Self::Shortcut => Self::Immediate,
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::Immediate => "Immediate March",
            Self::Rested => "Rest Before Departure",
            Self::Supplied => "Stocked Expedition",
            Self::Shortcut => "Signal-road Shortcut",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldClock {
    pub day: u32,
    pub minute_of_day: u16,
}

impl Default for WorldClock {
    fn default() -> Self {
        Self {
            day: 1,
            minute_of_day: 8 * 60,
        }
    }
}

impl WorldClock {
    pub fn advance(&mut self, minutes: u32) {
        let absolute = u32::from(self.minute_of_day).saturating_add(minutes);
        self.day = self.day.saturating_add(absolute / (24 * 60));
        self.minute_of_day = (absolute % (24 * 60)) as u16;
    }

    pub fn label(&self) -> String {
        format!(
            "Day {} {:02}:{:02}",
            self.day,
            self.minute_of_day / 60,
            self.minute_of_day % 60
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpeditionSupplyState {
    pub stamina: u8,
    pub rations: u8,
    pub water: u8,
}

impl Default for ExpeditionSupplyState {
    fn default() -> Self {
        Self {
            stamina: 100,
            rations: 4,
            water: 6,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpeditionReadiness {
    pub preparation: ExpeditionPreparation,
    pub stamina: u8,
    pub rations: u8,
    pub water: u8,
    pub starting_resources: u32,
    pub travel_minutes: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum UnlockCondition {
    MentorMet,
    WorldFlag {
        flag: String,
    },
    MissionVictories {
        mission: CampaignMission,
        count: u32,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum QuestReward {
    WorldFlag { flag: String },
    UnlockRoom { room_id: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuestStepDefinition {
    pub id: StoryStepId,
    pub mission: Option<CampaignMission>,
    pub conditions: Vec<UnlockCondition>,
    pub rewards: Vec<QuestReward>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuestDefinition {
    pub id: StoryQuestId,
    pub title: String,
    pub steps: Vec<QuestStepDefinition>,
}

pub fn signal_road_quest_definition() -> QuestDefinition {
    QuestDefinition {
        id: StoryQuestId::SignalRoad,
        title: "信号之路".to_string(),
        steps: vec![
            QuestStepDefinition {
                id: StoryStepId::MeetMentor,
                mission: None,
                conditions: vec![UnlockCondition::MentorMet],
                rewards: vec![
                    QuestReward::WorldFlag {
                        flag: "expedition_gate_open".to_string(),
                    },
                    QuestReward::UnlockRoom {
                        room_id: EXPEDITION_GATE_ROOM.to_string(),
                    },
                ],
            },
            QuestStepDefinition {
                id: StoryStepId::SecureFirstContact,
                mission: Some(CampaignMission::FirstContact),
                conditions: vec![UnlockCondition::WorldFlag {
                    flag: "first_contact_secured".to_string(),
                }],
                rewards: Vec::new(),
            },
            QuestStepDefinition {
                id: StoryStepId::BreakAftershock,
                mission: Some(CampaignMission::AftershockPatrol),
                conditions: vec![UnlockCondition::MissionVictories {
                    mission: CampaignMission::AftershockPatrol,
                    count: 1,
                }],
                rewards: vec![
                    QuestReward::WorldFlag {
                        flag: "signal_road_secured".to_string(),
                    },
                    QuestReward::UnlockRoom {
                        room_id: RELAY_QUARTER_ROOM.to_string(),
                    },
                ],
            },
            QuestStepDefinition {
                id: StoryStepId::EvacuateConvoy,
                mission: Some(CampaignMission::ConvoyExodus),
                conditions: vec![UnlockCondition::WorldFlag {
                    flag: "convoy_exodus_secured".to_string(),
                }],
                rewards: vec![QuestReward::WorldFlag {
                    flag: "outer_signal_road_open".to_string(),
                }],
            },
        ],
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoryProgress {
    pub quest_id: StoryQuestId,
    pub current_step: StoryStepId,
    #[serde(default)]
    pub completed_steps: BTreeSet<StoryStepId>,
    #[serde(default)]
    pub unlocked_room_ids: BTreeSet<String>,
}

impl Default for StoryProgress {
    fn default() -> Self {
        Self {
            quest_id: StoryQuestId::SignalRoad,
            current_step: StoryStepId::MeetMentor,
            completed_steps: BTreeSet::new(),
            unlocked_room_ids: BTreeSet::from([
                MIRROR_SQUARE_ROOM.to_string(),
                MENTOR_HALL_ROOM.to_string(),
                CISTERN_WARD_ROOM.to_string(),
                NIGHT_WATCH_POST_ROOM.to_string(),
                WORKSHOP_GATE_ROOM.to_string(),
                MARKET_WIND_PAVILION_ROOM.to_string(),
                LANTERN_INFIRMARY_ROOM.to_string(),
                ARCHIVE_STEPS_ROOM.to_string(),
                CARAVAN_YARD_ROOM.to_string(),
            ]),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BattleOutcome {
    Victory,
    Defeat,
    Withdrawal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnitBattleStatus {
    Healthy,
    Wounded,
    Incapacitated,
    Lost,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillProgress {
    pub rank: u16,
    pub experience: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LootStack {
    pub item_id: String,
    pub quantity: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartyMember {
    pub unit_id: String,
    pub display_name: String,
    pub role: String,
    pub attributes: TrillionniumAttributes,
    pub skill_ids: Vec<String>,
    pub persistent: bool,
    #[serde(default)]
    pub experience: u64,
    #[serde(default)]
    pub veteran_rank: u8,
    #[serde(default)]
    pub confirmed_kills: u32,
    pub injury_level: u8,
    pub available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CampaignProgression {
    pub level: u32,
    pub experience: u64,
    #[serde(default = "default_campaign_credits")]
    pub credits: i64,
    #[serde(default)]
    pub mentor_training_sessions: u8,
    #[serde(default)]
    pub aftershock_completions: u32,
    #[serde(default)]
    pub growth_points_available: u16,
    #[serde(default)]
    pub growth_points_awarded: u16,
    #[serde(default)]
    pub growth_allocations: BTreeMap<GrowthStat, u16>,
    pub skill_progress: BTreeMap<String, SkillProgress>,
    pub inventory: Vec<LootStack>,
    pub world_flags: BTreeSet<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BattleGridPoint {
    pub x: i16,
    pub y: i16,
}

impl BattleGridPoint {
    pub const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BattleMapNodeV1 {
    pub id: String,
    pub position: BattleGridPoint,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BattleMapSeedV1 {
    pub width: u16,
    pub height: u16,
    pub terrain_rows: Vec<String>,
    pub party_start: BattleGridPoint,
    pub approach_point: BattleGridPoint,
    pub objective: BattleGridPoint,
    pub resource_nodes: Vec<BattleMapNodeV1>,
    pub enemy_spawns: Vec<BattleMapNodeV1>,
}

impl BattleMapSeedV1 {
    pub fn validate(&self) -> Result<(), CampaignError> {
        if self.width < 12 || self.height < 8 || self.terrain_rows.len() != self.height as usize {
            return Err(CampaignError::InvalidContract(
                "battle map dimensions/rows are invalid".to_string(),
            ));
        }
        if self
            .terrain_rows
            .iter()
            .any(|row| row.chars().count() != self.width as usize)
        {
            return Err(CampaignError::InvalidContract(
                "battle map row width mismatch".to_string(),
            ));
        }
        if self.resource_nodes.is_empty() || self.enemy_spawns.len() < 3 {
            return Err(CampaignError::InvalidContract(
                "battle map requires resources and enemy spawns".to_string(),
            ));
        }
        for point in std::iter::once(&self.party_start)
            .chain(std::iter::once(&self.approach_point))
            .chain(std::iter::once(&self.objective))
            .chain(self.resource_nodes.iter().map(|node| &node.position))
            .chain(self.enemy_spawns.iter().map(|node| &node.position))
        {
            if !self.in_bounds(*point) {
                return Err(CampaignError::InvalidContract(
                    "battle map anchor is outside map bounds".to_string(),
                ));
            }
        }
        if !self.passable(self.party_start)
            || !self.passable(self.approach_point)
            || !self.passable(self.objective)
        {
            return Err(CampaignError::InvalidContract(
                "battle map route anchors must be passable".to_string(),
            ));
        }
        Ok(())
    }

    pub fn in_bounds(&self, point: BattleGridPoint) -> bool {
        point.x >= 0 && point.y >= 0 && point.x < self.width as i16 && point.y < self.height as i16
    }

    pub fn passable(&self, point: BattleGridPoint) -> bool {
        self.in_bounds(point)
            && self
                .terrain_rows
                .get(point.y as usize)
                .and_then(|row| row.as_bytes().get(point.x as usize))
                .is_some_and(|terrain| matches!(*terrain as char, 'g' | 'r'))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsUnitStats {
    pub max_hp: u32,
    pub damage: u32,
    pub armor: u32,
    pub move_speed_milli: u32,
    pub attack_interval_ticks: u32,
    pub evasion_permille: u16,
    pub energy: u32,
    pub ability_range: u32,
    pub skill_power_permille: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedEquipmentModifier {
    pub item_id: String,
    pub max_hp: i32,
    pub damage: i32,
    pub armor: i32,
    pub move_speed_milli: i32,
    pub attack_interval_ticks: i32,
    pub evasion_permille: i32,
    pub energy: i32,
    pub ability_range: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BattleUnitSeedV1 {
    pub unit_id: String,
    pub display_name: String,
    pub role: String,
    pub spawn_slot: String,
    pub persistent: bool,
    pub injury_level: u8,
    pub skill_ids: Vec<String>,
    pub equipment_ids: Vec<String>,
    #[serde(default)]
    pub veteran_rank: u8,
    pub stats: RtsUnitStats,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BattleSeedV1 {
    pub contract_version: String,
    pub battle_id: String,
    pub campaign_revision: u64,
    pub map_id: String,
    pub rules_version: String,
    pub map: BattleMapSeedV1,
    pub party: Vec<BattleUnitSeedV1>,
    pub mission: MissionDefinition,
    #[serde(default)]
    pub difficulty: CampaignDifficulty,
    #[serde(default)]
    pub character_origin: CharacterOrigin,
    #[serde(default)]
    pub build_path: BuildPath,
    #[serde(default)]
    pub active_title: Option<BuildTitle>,
    #[serde(default)]
    pub sect_id: Option<String>,
    #[serde(default)]
    pub regional_skill_bonus_permille: u16,
    #[serde(default = "default_field_build_cost_permille")]
    pub field_build_cost_permille: u16,
    #[serde(default = "default_expedition_readiness")]
    pub expedition_readiness: ExpeditionReadiness,
    #[serde(default)]
    pub skirmish: SkirmishSetup,
    pub seed_hash: String,
}

fn default_field_build_cost_permille() -> u16 {
    1000
}

fn default_expedition_readiness() -> ExpeditionReadiness {
    let supplies = ExpeditionSupplyState::default();
    ExpeditionReadiness {
        preparation: ExpeditionPreparation::Immediate,
        stamina: supplies.stamina,
        rations: supplies.rations,
        water: supplies.water,
        starting_resources: 0,
        travel_minutes: 20,
    }
}

impl BattleSeedV1 {
    pub fn validate(&self) -> Result<(), CampaignError> {
        if self.contract_version != BATTLE_SEED_CONTRACT {
            return Err(CampaignError::InvalidContract(
                self.contract_version.clone(),
            ));
        }
        if !matches!(
            self.map_id.as_str(),
            "first_contact"
                | "aftershock_patrol"
                | "first_contact_aftershock"
                | "convoy_exodus"
                | "mirror_siege"
                | "iron_delta"
                | "night_watch_crossing"
                | "glass_basin"
                | "ember_orchard"
                | "salt_marsh"
                | "cinder_crown"
        ) || self.rules_version != FIRST_CONTACT_RULES_VERSION
        {
            return Err(CampaignError::InvalidContract(
                "unknown map or rules version".to_string(),
            ));
        }
        self.map.validate()?;
        self.mission.validate(&self.map)?;
        self.skirmish.validate(&self.map_id)?;
        let skirmish_map = matches!(
            self.map_id.as_str(),
            "iron_delta"
                | "night_watch_crossing"
                | "glass_basin"
                | "ember_orchard"
                | "salt_marsh"
                | "cinder_crown"
        );
        if self.skirmish.enabled != skirmish_map {
            return Err(CampaignError::InvalidContract(
                "skirmish setup and selected map disagree".to_string(),
            ));
        }
        if self.mission.map_id != self.map_id {
            return Err(CampaignError::InvalidContract(
                "BattleSeed map and mission definition disagree".to_string(),
            ));
        }
        if self.party.len() != 4 {
            return Err(CampaignError::InvalidContract(
                "First Contact requires exactly four party units".to_string(),
            ));
        }
        if self.expedition_readiness.stamina > 100
            || self.expedition_readiness.rations > 12
            || self.expedition_readiness.water > 16
            || self.expedition_readiness.travel_minutes == 0
        {
            return Err(CampaignError::InvalidContract(
                "expedition readiness is outside authoritative bounds".to_string(),
            ));
        }
        let ids = self
            .party
            .iter()
            .map(|unit| unit.unit_id.as_str())
            .collect::<BTreeSet<_>>();
        let slots = self
            .party
            .iter()
            .map(|unit| unit.spawn_slot.as_str())
            .collect::<BTreeSet<_>>();
        if ids.len() != self.party.len() || slots.len() != self.party.len() {
            return Err(CampaignError::InvalidContract(
                "party unit ids and spawn slots must be unique".to_string(),
            ));
        }
        if self.seed_hash != self.computed_hash()? {
            return Err(CampaignError::Integrity(
                "BattleSeed hash does not match its payload".to_string(),
            ));
        }
        Ok(())
    }

    pub fn computed_hash(&self) -> Result<String, CampaignError> {
        let mut canonical = self.clone();
        canonical.seed_hash.clear();
        canonical_json_hash(&canonical)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnitBattleReportV1 {
    pub unit_id: String,
    pub status: UnitBattleStatus,
    pub remaining_hp: u32,
    pub experience_gained: u64,
    #[serde(default)]
    pub veteran_rank: u8,
    #[serde(default)]
    pub confirmed_kills: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BattleResultV1 {
    pub contract_version: String,
    pub battle_id: String,
    pub seed_hash: String,
    pub outcome: BattleOutcome,
    pub units: Vec<UnitBattleReportV1>,
    pub loot: Vec<LootStack>,
    pub resource_delta: i64,
    pub reputation_delta: i32,
    pub world_flags: Vec<String>,
    pub elapsed_ticks: u64,
    pub final_snapshot_hash: String,
}

impl BattleResultV1 {
    pub fn validate_against(&self, seed: &BattleSeedV1) -> Result<(), CampaignError> {
        seed.validate()?;
        if self.contract_version != BATTLE_RESULT_CONTRACT {
            return Err(CampaignError::InvalidContract(
                self.contract_version.clone(),
            ));
        }
        if self.battle_id != seed.battle_id || self.seed_hash != seed.seed_hash {
            return Err(CampaignError::Integrity(
                "BattleResult does not belong to the pending seed".to_string(),
            ));
        }
        if self.final_snapshot_hash.len() != 64 || self.elapsed_ticks == 0 {
            return Err(CampaignError::InvalidContract(
                "BattleResult requires a terminal snapshot hash and elapsed ticks".to_string(),
            ));
        }
        let expected = seed
            .party
            .iter()
            .map(|unit| unit.unit_id.as_str())
            .collect::<BTreeSet<_>>();
        let actual = self
            .units
            .iter()
            .map(|unit| unit.unit_id.as_str())
            .collect::<BTreeSet<_>>();
        if actual.len() != self.units.len() || actual != expected {
            return Err(CampaignError::InvalidContract(
                "BattleResult must report every seeded party unit exactly once".to_string(),
            ));
        }
        Ok(())
    }

    pub fn computed_hash(&self) -> Result<String, CampaignError> {
        canonical_json_hash(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingBattleV1 {
    pub seed: BattleSeedV1,
    pub result: Option<BattleResultV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettlementReceiptV1 {
    pub contract_version: String,
    pub battle_id: String,
    pub seed_hash: String,
    pub result_hash: String,
    pub campaign_revision_before: u64,
    pub campaign_revision_after: u64,
    pub outcome: BattleOutcome,
    pub experience_delta: u64,
    pub reputation_delta: i32,
    #[serde(default)]
    pub credit_delta: i64,
    pub loot_delta: Vec<LootStack>,
    pub injury_delta_by_unit: BTreeMap<String, u8>,
    #[serde(default)]
    pub economic_intent_id: Option<String>,
    #[serde(default)]
    pub economic_receipt_id: Option<String>,
    pub duplicate: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TradeablePurchaseStage {
    ReservePending,
    Reserved,
    SellerSettlementPending,
    SellerSettled,
    BuyerConsumePending,
    Consumed,
    RefundPending,
    Refunded,
    HardFailed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingTradeablePurchase {
    pub purchase_id: String,
    pub item_id: String,
    pub quantity: u16,
    pub price_wallet_credits: i64,
    pub buyer: EconomyAccountBinding,
    pub seller: EconomyAccountBinding,
    pub stage: TradeablePurchaseStage,
    pub reserve_intent_id: String,
    #[serde(default)]
    pub settle_intent_id: Option<String>,
    #[serde(default)]
    pub consume_intent_id: Option<String>,
    #[serde(default)]
    pub refund_intent_id: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EconomyReconciliationReport {
    pub attempted: u16,
    pub applied: u16,
    pub recoverable_holds: u16,
    pub hard_failures: u16,
    pub remaining: usize,
    pub last_error: Option<String>,
}

struct EconomicIntentDraft {
    kind: EconomicIntentKind,
    term_id: String,
    intent_id: String,
    binding: EconomyAccountBinding,
    asset_id: String,
    quantity: i64,
    amount_credits: i64,
    metadata: Value,
}

pub trait EconomyBackend {
    fn backend_id(&self) -> &str;
    fn execute(&self, intent: &EconomicIntent) -> Result<EconomicReceipt, String>;
    fn wallet_snapshot(
        &self,
        _binding: &EconomyAccountBinding,
        _cursor: u64,
    ) -> Result<Option<WalletSnapshot>, String> {
        Ok(None)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct OfflineLocalEconomyBackend;

impl EconomyBackend for OfflineLocalEconomyBackend {
    fn backend_id(&self) -> &str {
        OFFLINE_LOCAL_BACKEND_ID
    }

    fn execute(&self, intent: &EconomicIntent) -> Result<EconomicReceipt, String> {
        intent.validate()?;
        let status = match intent.kind {
            EconomicIntentKind::Reserve => ReceiptStatus::Reserved,
            EconomicIntentKind::Settle => ReceiptStatus::Settled,
            EconomicIntentKind::Consume => ReceiptStatus::Consumed,
            EconomicIntentKind::Refund => ReceiptStatus::Refunded,
            EconomicIntentKind::Chargeback => ReceiptStatus::SellerChargebackConsumed,
            EconomicIntentKind::ReleaseReward | EconomicIntentKind::CompleteContract => {
                ReceiptStatus::ApprovedRelease
            }
            EconomicIntentKind::Quote => ReceiptStatus::SkippedZeroPrice,
            EconomicIntentKind::VerifyReceipt => ReceiptStatus::Duplicate,
        };
        let mut receipt = EconomicReceipt::from_intent(
            format!("offline-receipt:{}", intent.intent_id),
            intent,
            OFFLINE_LOCAL_BACKEND_ID,
            SettlementBackendKind::LocalTest,
            status,
            intent.created_at_epoch,
        );
        receipt.settlement_reference = Some(intent.idempotency_key.key.clone());
        receipt.evidence = json!({"authority": "trnm-campaign-core", "offline": true});
        Ok(receipt)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NpcConversationRecord {
    pub npc_id: String,
    pub line: String,
    pub activity: String,
    pub day: u32,
    pub minute_of_day: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NpcSocialEventRecord {
    pub event_id: String,
    pub first_npc_id: String,
    pub second_npc_id: String,
    pub room_id: String,
    pub text: String,
    pub day: u32,
    pub minute_of_day: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionalMarketTransfer {
    pub item_id: String,
    pub from_region_id: String,
    pub to_region_id: String,
    pub quantity: u16,
    pub day: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NpcAutonomousGoalKind {
    Produce,
    Migrate,
    FormAlliance,
    ResolveConflict,
    PublishTask,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NpcAutonomousGoal {
    pub kind: NpcAutonomousGoalKind,
    pub target_id: String,
    pub region_id: String,
    pub progress: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionalCaravanState {
    pub caravan_id: String,
    pub item_id: String,
    pub from_region_id: String,
    pub to_region_id: String,
    pub quantity: u16,
    pub progress_legs: u8,
    pub risk: u8,
    #[serde(default)]
    pub route_room_ids: Vec<String>,
    #[serde(default)]
    pub route_index: usize,
    #[serde(default = "default_caravan_integrity")]
    pub integrity: u8,
    #[serde(default)]
    pub guarded_by_player: bool,
    #[serde(default)]
    pub incident: Option<String>,
}

const fn default_caravan_integrity() -> u8 {
    100
}

impl RegionalCaravanState {
    pub fn current_room_id(&self) -> Option<&str> {
        self.route_room_ids
            .get(self.route_index)
            .map(String::as_str)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MainStoryChoice {
    #[default]
    ProtectWayhouses,
    ExposeConspiracy,
    ForgeAccord,
}

const fn default_secondary_technique_slot() -> u8 {
    1
}

const MARKET_REGION_IDS: [&str; 4] = ["mirror_city", "signal_road", "glass_basin", "ashen_fringe"];

fn default_regional_market_stock() -> BTreeMap<String, BTreeMap<String, u16>> {
    MARKET_REGION_IDS
        .into_iter()
        .enumerate()
        .map(|(region_index, region)| {
            let items = ECONOMY_ITEM_CATALOG
                .iter()
                .enumerate()
                .map(|(item_index, item)| {
                    let baseline = if item.material { 12 } else { 4 };
                    let local_bias = if region_index == 0 {
                        0
                    } else {
                        ((region_index + item_index) % 3) as u16
                    };
                    (item.id.to_string(), baseline + local_bias)
                })
                .collect();
            (region.to_string(), items)
        })
        .collect()
}

fn default_regional_market_demand() -> BTreeMap<String, BTreeMap<String, i16>> {
    MARKET_REGION_IDS
        .into_iter()
        .enumerate()
        .map(|(region_index, region)| {
            let items = ECONOMY_ITEM_CATALOG
                .iter()
                .enumerate()
                .map(|(item_index, item)| {
                    (
                        item.id.to_string(),
                        if region_index == 0 {
                            0
                        } else {
                            ((region_index * 2 + item_index) % 5) as i16 - 2
                        },
                    )
                })
                .collect();
            (region.to_string(), items)
        })
        .collect()
}

impl MainStoryChoice {
    pub fn next(self) -> Self {
        match self {
            Self::ProtectWayhouses => Self::ExposeConspiracy,
            Self::ExposeConspiracy => Self::ForgeAccord,
            Self::ForgeAccord => Self::ProtectWayhouses,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MainStoryChapter {
    #[default]
    MirrorCityOaths,
    SignalRoadReckoning,
    AshenFringeCountermarch,
    ChapterComplete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MainStoryEnding {
    WayhouseLeague,
    OpenArchiveRepublic,
    FrontierAccord,
    ThreeRoadCompact,
    ContestedMandate,
}

impl MainStoryEnding {
    pub fn label(self) -> &'static str {
        match self {
            Self::WayhouseLeague => "The Wayhouse League",
            Self::OpenArchiveRepublic => "The Open Archive",
            Self::FrontierAccord => "The Frontier Accord",
            Self::ThreeRoadCompact => "The Three-Road Compact",
            Self::ContestedMandate => "The Contested Mandate",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MainStoryChapterDefinition {
    pub chapter: MainStoryChapter,
    pub title: &'static str,
    pub protagonist_id: &'static str,
    pub scene_id: &'static str,
    pub room_id: &'static str,
    pub quest_ids: [&'static str; 5],
}

pub const MAIN_STORY_CHAPTERS: [MainStoryChapterDefinition; 3] = [
    MainStoryChapterDefinition {
        chapter: MainStoryChapter::MirrorCityOaths,
        title: "Oaths Beneath the Mirror",
        protagonist_id: "street-compass-sifu",
        scene_id: "mirror_square_public_oath",
        room_id: MIRROR_SQUARE_ROOM,
        quest_ids: [
            "wayfinder_oath",
            "broken_milestone",
            "forge_commission",
            "lost_tooling",
            "lantern_watch",
        ],
    },
    MainStoryChapterDefinition {
        chapter: MainStoryChapter::SignalRoadReckoning,
        title: "The Signal Road Reckoning",
        protagonist_id: "captain-veyra",
        scene_id: "archive_steps_reckoning",
        room_id: ARCHIVE_STEPS_ROOM,
        quest_ids: [
            "wanted_raider",
            "relay_salvage",
            "fever_tonic",
            "archive_witness",
            "market_debt",
        ],
    },
    MainStoryChapterDefinition {
        chapter: MainStoryChapter::AshenFringeCountermarch,
        title: "Countermarch at Ashen Fringe",
        protagonist_id: "scout-mako",
        scene_id: "ash_beacon_final_assembly",
        room_id: ASH_BEACON_FIELD_ROOM,
        quest_ids: [
            "missing_crate",
            "night_letter",
            "escort_manifest",
            "bandit_tracks",
            "ration_audit",
        ],
    },
];

fn main_story_chapter_outcome(
    chapter: MainStoryChapter,
    choice: MainStoryChoice,
) -> (&'static str, i64, i32, &'static str) {
    match (chapter, choice) {
        (MainStoryChapter::MirrorCityOaths, MainStoryChoice::ProtectWayhouses) => (
            "chapter_mirror_oaths_wayhouses",
            35,
            2,
            "At the public oath, the first wayhouses become neutral shelters under your seal.",
        ),
        (MainStoryChapter::MirrorCityOaths, MainStoryChoice::ExposeConspiracy) => (
            "chapter_mirror_oaths_archive",
            15,
            5,
            "At the public oath, the archive names the guild officers who sold the road markers.",
        ),
        (MainStoryChapter::MirrorCityOaths, MainStoryChoice::ForgeAccord) => (
            "chapter_mirror_oaths_accord",
            25,
            3,
            "At the public oath, city smiths and road guides sign a shared repair compact.",
        ),
        (MainStoryChapter::SignalRoadReckoning, MainStoryChoice::ProtectWayhouses) => (
            "chapter_signal_reckoning_wayhouses",
            45,
            2,
            "Veyra stations mixed watches at every Signal Road refuge.",
        ),
        (MainStoryChapter::SignalRoadReckoning, MainStoryChoice::ExposeConspiracy) => (
            "chapter_signal_reckoning_archive",
            20,
            6,
            "Sol publishes the convoy ledger and breaks the hidden ration cartel.",
        ),
        (MainStoryChapter::SignalRoadReckoning, MainStoryChoice::ForgeAccord) => (
            "chapter_signal_reckoning_accord",
            35,
            4,
            "The watch, couriers and salvagers accept one road court.",
        ),
        (MainStoryChapter::AshenFringeCountermarch, MainStoryChoice::ProtectWayhouses) => (
            "chapter_ashen_countermarch_wayhouses",
            55,
            3,
            "The countermarch ends with a defended chain of free frontier houses.",
        ),
        (MainStoryChapter::AshenFringeCountermarch, MainStoryChoice::ExposeConspiracy) => (
            "chapter_ashen_countermarch_archive",
            25,
            7,
            "The final assembly hears every witness and dissolves the Ashen command clique.",
        ),
        (MainStoryChapter::AshenFringeCountermarch, MainStoryChoice::ForgeAccord) => (
            "chapter_ashen_countermarch_accord",
            45,
            5,
            "Mirror City and the Ashen Fringe sign a guarded frontier constitution.",
        ),
        _ => unreachable!("terminal chapter outcomes exclude ChapterComplete"),
    }
}

pub fn resolve_main_story_ending(decisions: &[MainStoryDecisionRecord]) -> Option<MainStoryEnding> {
    if decisions.len() != MAIN_STORY_CHAPTERS.len() {
        return None;
    }
    let choices = decisions
        .iter()
        .map(|decision| decision.choice)
        .collect::<Vec<_>>();
    if choices
        .iter()
        .all(|choice| *choice == MainStoryChoice::ProtectWayhouses)
    {
        Some(MainStoryEnding::WayhouseLeague)
    } else if choices
        .iter()
        .all(|choice| *choice == MainStoryChoice::ExposeConspiracy)
    {
        Some(MainStoryEnding::OpenArchiveRepublic)
    } else if choices
        .iter()
        .all(|choice| *choice == MainStoryChoice::ForgeAccord)
    {
        Some(MainStoryEnding::FrontierAccord)
    } else if choices.iter().copied().collect::<BTreeSet<_>>().len() == 3 {
        Some(MainStoryEnding::ThreeRoadCompact)
    } else {
        Some(MainStoryEnding::ContestedMandate)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MainStoryDecisionRecord {
    pub chapter: MainStoryChapter,
    pub choice: MainStoryChoice,
    pub outcome_flag: String,
    pub day: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MainStorySceneAdvance {
    SceneBeat {
        chapter: MainStoryChapter,
        step: u8,
        text: String,
    },
    ChapterResolved(MainStoryDecisionRecord),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionalQuestRuntime {
    pub quest_id: String,
    pub accepted_day: u32,
    pub deadline_day: u32,
    pub approach: QuestApproach,
    pub evidence_count: u8,
    pub failure_count: u8,
    #[serde(default)]
    pub completed_condition_node_ids: BTreeSet<String>,
}

impl RegionalQuestRuntime {
    fn new(quest_id: &str, accepted_day: u32, deadline_days: u32, failures: u8) -> Self {
        let completed_condition_node_ids = BTreeSet::from([format!("{quest_id}_giver")]);
        Self {
            quest_id: quest_id.to_string(),
            accepted_day,
            deadline_day: accepted_day.saturating_add(deadline_days),
            approach: QuestApproach::Direct,
            evidence_count: 0,
            failure_count: failures,
            completed_condition_node_ids,
        }
    }
}

fn quest_graph_node_ready(
    graph: &trnm_rpg_core::QuestConditionGraph,
    node_id: &str,
    completed: &BTreeSet<String>,
) -> bool {
    let incoming = graph
        .edges
        .iter()
        .filter(|edge| edge.to == node_id)
        .collect::<Vec<_>>();
    let mut exclusive_groups = BTreeSet::<String>::new();
    for edge in incoming {
        let Some(source) = graph.nodes.iter().find(|node| node.id == edge.from) else {
            return false;
        };
        if source.optional {
            continue;
        }
        if let Some(group) = &source.exclusive_group {
            exclusive_groups.insert(group.clone());
        } else if !completed.contains(&source.id) {
            return false;
        }
    }
    exclusive_groups.iter().all(|group| {
        graph.nodes.iter().any(|candidate| {
            candidate.exclusive_group.as_ref() == Some(group) && completed.contains(&candidate.id)
        })
    })
}

fn quest_route_node_satisfied(
    graph: &trnm_rpg_core::QuestConditionGraph,
    node: &trnm_rpg_core::QuestConditionNode,
    completed: &BTreeSet<String>,
) -> bool {
    if completed.contains(&node.id) || node.optional {
        return true;
    }
    node.exclusive_group.as_ref().is_some_and(|group| {
        graph.nodes.iter().any(|candidate| {
            candidate.exclusive_group.as_ref() == Some(group) && completed.contains(&candidate.id)
        })
    })
}

impl SettlementReceiptV1 {
    pub fn duplicate_from(existing: &Self, revision: u64) -> Self {
        Self {
            contract_version: existing.contract_version.clone(),
            battle_id: existing.battle_id.clone(),
            seed_hash: existing.seed_hash.clone(),
            result_hash: existing.result_hash.clone(),
            campaign_revision_before: revision,
            campaign_revision_after: revision,
            outcome: existing.outcome,
            experience_delta: 0,
            reputation_delta: 0,
            credit_delta: 0,
            loot_delta: Vec::new(),
            injury_delta_by_unit: BTreeMap::new(),
            economic_intent_id: existing.economic_intent_id.clone(),
            economic_receipt_id: existing.economic_receipt_id.clone(),
            duplicate: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CampaignSaveV1 {
    pub contract_version: String,
    #[serde(default = "legacy_campaign_schema_revision")]
    pub schema_revision: u16,
    pub campaign_id: String,
    pub revision: u64,
    pub room: CampaignRoom,
    pub phase: CampaignPhase,
    pub character: WorldTrillionniumCharacter,
    #[serde(default)]
    pub character_identity: CharacterIdentity,
    #[serde(default)]
    pub character_origin: CharacterOrigin,
    #[serde(default)]
    pub difficulty: CampaignDifficulty,
    pub progression: CampaignProgression,
    pub party: Vec<PartyMember>,
    pub active_party_ids: Vec<String>,
    #[serde(default)]
    pub selected_training_path: TrainingPath,
    #[serde(default)]
    pub selected_loadout: LoadoutPreset,
    #[serde(default)]
    pub active_mission: CampaignMission,
    #[serde(default)]
    pub skirmish_setup: SkirmishSetup,
    #[serde(default)]
    pub story: StoryProgress,
    #[serde(default)]
    pub npc_relationships: BTreeMap<String, NpcRelationship>,
    #[serde(default)]
    pub faction_rank: FactionRank,
    #[serde(default)]
    pub last_sparring: Option<SparringReport>,
    #[serde(default)]
    pub pending_growth_stat: Option<GrowthStat>,
    #[serde(default)]
    pub build_path: BuildPath,
    #[serde(default)]
    pub unlocked_titles: BTreeSet<BuildTitle>,
    #[serde(default)]
    pub active_title: Option<BuildTitle>,
    #[serde(default)]
    pub active_encounter: Option<RpgEncounterState>,
    #[serde(default)]
    pub last_encounter_outcome: Option<EncounterOutcome>,
    #[serde(default)]
    pub combat_log: Vec<CombatLogBeat>,
    #[serde(default)]
    pub regional_quest_states: BTreeMap<String, QuestState>,
    #[serde(default)]
    pub active_regional_quest_id: Option<String>,
    #[serde(default)]
    pub active_regional_quest_step: usize,
    #[serde(default)]
    pub active_regional_quest_runtime: Option<RegionalQuestRuntime>,
    #[serde(default)]
    pub regional_quest_failure_counts: BTreeMap<String, u8>,
    #[serde(default)]
    pub dialogue_choice: DialogueChoice,
    #[serde(default)]
    pub equipped_technique_slot: u8,
    #[serde(default = "default_secondary_technique_slot")]
    pub secondary_technique_slot: u8,
    #[serde(default)]
    pub technique_mastery: BTreeMap<String, u16>,
    #[serde(default)]
    pub main_story_chapter: MainStoryChapter,
    #[serde(default)]
    pub main_story_choice: MainStoryChoice,
    #[serde(default)]
    pub main_story_decisions: Vec<MainStoryDecisionRecord>,
    #[serde(default)]
    pub main_story_ending: Option<MainStoryEnding>,
    #[serde(default)]
    pub pending_main_story_chapter: Option<MainStoryChapter>,
    #[serde(default)]
    pub main_story_scene_progress: BTreeMap<String, u8>,
    #[serde(default)]
    pub post_ending_world_state: Option<String>,
    #[serde(default)]
    pub ending_epilogue_progress: u8,
    #[serde(default)]
    pub ending_epilogue_complete: bool,
    #[serde(default)]
    pub last_npc_conversation: Option<NpcConversationRecord>,
    #[serde(default)]
    pub conversation_history: Vec<NpcConversationRecord>,
    #[serde(default)]
    pub social_event_history: Vec<NpcSocialEventRecord>,
    #[serde(default)]
    pub npc_memory: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    pub npc_bonds: BTreeMap<String, i16>,
    #[serde(default)]
    pub npc_work_output: BTreeMap<String, u32>,
    #[serde(default)]
    pub npc_autonomous_goals: BTreeMap<String, NpcAutonomousGoal>,
    #[serde(default)]
    pub npc_goal_rooms: BTreeMap<String, String>,
    #[serde(default)]
    pub selected_shop_item_index: usize,
    #[serde(default)]
    pub selected_recipe_index: usize,
    #[serde(default)]
    pub selected_inventory_index: usize,
    #[serde(default)]
    pub item_conditions: BTreeMap<String, ItemCondition>,
    #[serde(default)]
    pub market_stock: BTreeMap<String, u16>,
    #[serde(default)]
    pub market_demand: BTreeMap<String, i16>,
    #[serde(default = "default_regional_market_stock")]
    pub regional_market_stock: BTreeMap<String, BTreeMap<String, u16>>,
    #[serde(default = "default_regional_market_demand")]
    pub regional_market_demand: BTreeMap<String, BTreeMap<String, i16>>,
    #[serde(default)]
    pub regional_logistics: Vec<RegionalMarketTransfer>,
    #[serde(default)]
    pub active_regional_caravans: Vec<RegionalCaravanState>,
    #[serde(default)]
    pub economy_mode: EconomyMode,
    #[serde(default)]
    pub economy_account_binding: Option<EconomyAccountBinding>,
    #[serde(default)]
    pub wallet_snapshot: WalletSnapshot,
    #[serde(default)]
    pub pending_economic_intents: Vec<EconomicIntent>,
    #[serde(default)]
    pub verified_economic_receipts: Vec<EconomicReceipt>,
    #[serde(default)]
    pub economic_idempotency_keys: BTreeSet<String>,
    #[serde(default)]
    pub economic_dead_letters: Vec<EconomicIntent>,
    #[serde(default)]
    pub pending_tradeable_purchases: Vec<PendingTradeablePurchase>,
    #[serde(default)]
    pub reconciliation_cursor: u64,
    #[serde(default)]
    pub quest_chain: Option<QuestChainProgress>,
    #[serde(default)]
    pub world_clock: WorldClock,
    #[serde(default)]
    pub expedition_supplies: ExpeditionSupplyState,
    #[serde(default)]
    pub selected_expedition_preparation: ExpeditionPreparation,
    pub mentor_met: bool,
    pub trained_with_mentor: bool,
    pub quest_state: QuestState,
    pub pending_battle: Option<PendingBattleV1>,
    pub settled_battle_ids: BTreeSet<String>,
    pub settlement_receipts: Vec<SettlementReceiptV1>,
}

impl Default for CampaignSaveV1 {
    fn default() -> Self {
        let mut character = WorldTrillionniumCharacter::default_for("local-player");
        CharacterOrigin::Balanced.apply(&mut character.attributes);
        character
            .skill_ids
            .push(CharacterOrigin::Balanced.starter_skill().to_string());
        for item_id in [
            "iron-workshop-blade",
            "market-wind-sword",
            "night-watch-cloak",
            "raid-signal-drum",
        ] {
            if let Some(item) = trillionnium_inventory_item_for(
                "local-player",
                item_id,
                "first_contact_loadout_choice",
                None,
                0,
            ) {
                character.inventory_items.push(item);
            }
        }
        character.equipment_slots.clear();
        for item in &mut character.inventory_items {
            item.equipped_slot = None;
        }
        let mut skill_progress = BTreeMap::new();
        for skill_id in &character.skill_ids {
            skill_progress.insert(
                skill_id.clone(),
                SkillProgress {
                    rank: 1,
                    experience: 0,
                },
            );
        }
        let base = character.attributes.clone();
        let mut scout = base.clone();
        scout.agility += 4;
        scout.insight += 2;
        let mut warden = base.clone();
        warden.physique += 5;
        warden.resolve += 4;
        let mut striker = base.clone();
        striker.force += 5;
        striker.agility += 2;
        let item_conditions = character_item_conditions(&character);
        Self {
            contract_version: CAMPAIGN_SAVE_CONTRACT.to_string(),
            schema_revision: 10,
            campaign_id: "local-campaign".to_string(),
            revision: 0,
            room: CampaignRoom::MirrorSquare,
            phase: CampaignPhase::Town,
            character,
            character_identity: CharacterIdentity::default(),
            character_origin: CharacterOrigin::Balanced,
            difficulty: CampaignDifficulty::Standard,
            progression: CampaignProgression {
                level: 1,
                experience: 0,
                credits: default_campaign_credits(),
                mentor_training_sessions: 0,
                aftershock_completions: 0,
                growth_points_available: 1,
                growth_points_awarded: 1,
                growth_allocations: BTreeMap::new(),
                skill_progress,
                inventory: Vec::new(),
                world_flags: BTreeSet::new(),
            },
            party: vec![
                PartyMember {
                    unit_id: "hero".to_string(),
                    display_name: "Mirror Ranger".to_string(),
                    role: "worker".to_string(),
                    attributes: base,
                    skill_ids: vec!["basic_inner_power".to_string()],
                    persistent: true,
                    experience: 0,
                    veteran_rank: 0,
                    confirmed_kills: 0,
                    injury_level: 0,
                    available: true,
                },
                PartyMember {
                    unit_id: "aya".to_string(),
                    display_name: "Aya".to_string(),
                    role: "scout".to_string(),
                    attributes: scout,
                    skill_ids: vec![
                        "basic_lightness".to_string(),
                        "route_scouting".to_string(),
                        "wind_step".to_string(),
                    ],
                    persistent: true,
                    experience: 0,
                    veteran_rank: 0,
                    confirmed_kills: 0,
                    injury_level: 0,
                    available: true,
                },
                PartyMember {
                    unit_id: "mako".to_string(),
                    display_name: "Mako".to_string(),
                    role: "warden".to_string(),
                    attributes: warden,
                    skill_ids: vec!["basic_unarmed".to_string(), "iron_guard".to_string()],
                    persistent: true,
                    experience: 0,
                    veteran_rank: 0,
                    confirmed_kills: 0,
                    injury_level: 0,
                    available: true,
                },
                PartyMember {
                    unit_id: "tess".to_string(),
                    display_name: "Tess".to_string(),
                    role: "striker".to_string(),
                    attributes: striker,
                    skill_ids: vec!["basic_blade".to_string(), "inner_flame".to_string()],
                    persistent: true,
                    experience: 0,
                    veteran_rank: 0,
                    confirmed_kills: 0,
                    injury_level: 0,
                    available: true,
                },
                PartyMember {
                    unit_id: "nia".to_string(),
                    display_name: "Nia".to_string(),
                    role: "medic".to_string(),
                    attributes: {
                        let mut attributes = TrillionniumAttributes::default();
                        attributes.resolve += 4;
                        attributes.insight += 5;
                        attributes
                    },
                    skill_ids: vec!["field_mend".to_string()],
                    persistent: true,
                    experience: 0,
                    veteran_rank: 0,
                    confirmed_kills: 0,
                    injury_level: 0,
                    available: true,
                },
                PartyMember {
                    unit_id: "brann".to_string(),
                    display_name: "Brann".to_string(),
                    role: "engineer".to_string(),
                    attributes: {
                        let mut attributes = TrillionniumAttributes::default();
                        attributes.craft += 6;
                        attributes.physique += 3;
                        attributes
                    },
                    skill_ids: vec!["relay_overcharge".to_string()],
                    persistent: true,
                    experience: 0,
                    veteran_rank: 0,
                    confirmed_kills: 0,
                    injury_level: 0,
                    available: false,
                },
                PartyMember {
                    unit_id: "sol".to_string(),
                    display_name: "Sol".to_string(),
                    role: "mystic".to_string(),
                    attributes: {
                        let mut attributes = TrillionniumAttributes::default();
                        attributes.insight += 6;
                        attributes.resolve += 2;
                        attributes
                    },
                    skill_ids: vec!["inner_flame".to_string()],
                    persistent: true,
                    experience: 0,
                    veteran_rank: 0,
                    confirmed_kills: 0,
                    injury_level: 0,
                    available: true,
                },
            ],
            active_party_ids: vec![
                "hero".to_string(),
                "aya".to_string(),
                "mako".to_string(),
                "tess".to_string(),
            ],
            selected_training_path: TrainingPath::default(),
            selected_loadout: LoadoutPreset::default(),
            active_mission: CampaignMission::default(),
            skirmish_setup: SkirmishSetup::default(),
            story: StoryProgress::default(),
            npc_relationships: NPC_CATALOG
                .iter()
                .map(|npc| {
                    (
                        npc.id.to_string(),
                        NpcRelationship::new(npc.id, npc.faction_id),
                    )
                })
                .collect(),
            faction_rank: FactionRank::Outsider,
            last_sparring: None,
            pending_growth_stat: None,
            build_path: BuildPath::Unformed,
            unlocked_titles: BTreeSet::new(),
            active_title: None,
            active_encounter: None,
            last_encounter_outcome: None,
            combat_log: Vec::new(),
            regional_quest_states: REGIONAL_QUEST_CATALOG
                .iter()
                .map(|quest| (quest.id.to_string(), QuestState::Available))
                .collect(),
            active_regional_quest_id: None,
            active_regional_quest_step: 0,
            active_regional_quest_runtime: None,
            regional_quest_failure_counts: BTreeMap::new(),
            dialogue_choice: DialogueChoice::AskForWork,
            equipped_technique_slot: 0,
            secondary_technique_slot: default_secondary_technique_slot(),
            technique_mastery: BTreeMap::new(),
            main_story_chapter: MainStoryChapter::MirrorCityOaths,
            main_story_choice: MainStoryChoice::ProtectWayhouses,
            main_story_decisions: Vec::new(),
            main_story_ending: None,
            pending_main_story_chapter: None,
            main_story_scene_progress: BTreeMap::new(),
            post_ending_world_state: None,
            ending_epilogue_progress: 0,
            ending_epilogue_complete: false,
            last_npc_conversation: None,
            conversation_history: Vec::new(),
            social_event_history: Vec::new(),
            npc_memory: BTreeMap::new(),
            npc_bonds: BTreeMap::new(),
            npc_work_output: BTreeMap::new(),
            npc_autonomous_goals: BTreeMap::new(),
            npc_goal_rooms: BTreeMap::new(),
            selected_shop_item_index: 0,
            selected_recipe_index: 0,
            selected_inventory_index: 0,
            item_conditions,
            market_stock: ECONOMY_ITEM_CATALOG
                .iter()
                .map(|item| (item.id.to_string(), if item.material { 12 } else { 4 }))
                .collect(),
            market_demand: ECONOMY_ITEM_CATALOG
                .iter()
                .map(|item| (item.id.to_string(), 0))
                .collect(),
            regional_market_stock: default_regional_market_stock(),
            regional_market_demand: default_regional_market_demand(),
            regional_logistics: Vec::new(),
            active_regional_caravans: Vec::new(),
            economy_mode: EconomyMode::OfflineLocal,
            economy_account_binding: None,
            wallet_snapshot: WalletSnapshot::default(),
            pending_economic_intents: Vec::new(),
            verified_economic_receipts: Vec::new(),
            economic_idempotency_keys: BTreeSet::new(),
            economic_dead_letters: Vec::new(),
            pending_tradeable_purchases: Vec::new(),
            reconciliation_cursor: 0,
            quest_chain: None,
            world_clock: WorldClock::default(),
            expedition_supplies: ExpeditionSupplyState::default(),
            selected_expedition_preparation: ExpeditionPreparation::Immediate,
            mentor_met: false,
            trained_with_mentor: false,
            quest_state: QuestState::Locked,
            pending_battle: None,
            settled_battle_ids: BTreeSet::new(),
            settlement_receipts: Vec::new(),
        }
    }
}

impl CampaignSaveV1 {
    pub fn ensure_gameplay_defaults(&mut self) {
        self.schema_revision = 10;
        if self.active_regional_quest_id.is_none() {
            self.active_regional_quest_step = 0;
            self.active_regional_quest_runtime = None;
        }
        if self.conversation_history.len() > 24 {
            self.conversation_history = self
                .conversation_history
                .split_off(self.conversation_history.len() - 24);
        }
        if self.social_event_history.len() > 16 {
            self.social_event_history = self
                .social_event_history
                .split_off(self.social_event_history.len() - 16);
        }
        for memory in self.npc_memory.values_mut() {
            if memory.len() > 8 {
                *memory = memory.split_off(memory.len() - 8);
            }
        }
        self.main_story_decisions
            .sort_by_key(|decision| decision.chapter);
        self.main_story_decisions
            .dedup_by_key(|decision| decision.chapter);
        self.main_story_ending = resolve_main_story_ending(&self.main_story_decisions);
        self.main_story_chapter = MAIN_STORY_CHAPTERS
            .iter()
            .find(|chapter| {
                !self
                    .main_story_decisions
                    .iter()
                    .any(|decision| decision.chapter == chapter.chapter)
            })
            .map(|chapter| chapter.chapter)
            .unwrap_or(MainStoryChapter::ChapterComplete);
        self.selected_shop_item_index %= ECONOMY_ITEM_CATALOG.len();
        self.selected_recipe_index %= CRAFTING_RECIPES.len();
        if !self.character.inventory_items.is_empty() {
            self.selected_inventory_index %= self.character.inventory_items.len();
        }
        let defaults = Self::default();
        for member in defaults.party {
            if let Some(existing) = self
                .party
                .iter_mut()
                .find(|existing| existing.unit_id == member.unit_id)
            {
                for skill_id in member.skill_ids {
                    if !existing.skill_ids.contains(&skill_id) {
                        existing.skill_ids.push(skill_id);
                    }
                }
            } else {
                self.party.push(member);
            }
        }
        for npc in NPC_CATALOG {
            self.npc_relationships
                .entry(npc.id.to_string())
                .or_insert_with(|| NpcRelationship::new(npc.id, npc.faction_id));
        }
        for quest in REGIONAL_QUEST_CATALOG {
            self.regional_quest_states
                .entry(quest.id.to_string())
                .or_insert(QuestState::Available);
        }
        if self.progression.growth_points_awarded == 0
            && self.progression.growth_allocations.is_empty()
        {
            self.progression.growth_points_available = 1;
            self.progression.growth_points_awarded = 1;
        }
        for item in defaults.character.inventory_items {
            if !self
                .character
                .inventory_items
                .iter()
                .any(|existing| existing.item_id == item.item_id)
            {
                self.character.inventory_items.push(item);
            }
        }
        for (instance_id, condition) in character_item_conditions(&self.character) {
            self.item_conditions.entry(instance_id).or_insert(condition);
        }
        for item in ECONOMY_ITEM_CATALOG {
            self.market_stock
                .entry(item.id.to_string())
                .or_insert(if item.material { 12 } else { 4 });
            self.market_demand.entry(item.id.to_string()).or_insert(0);
        }
        let default_stock = default_regional_market_stock();
        let default_demand = default_regional_market_demand();
        for region in MARKET_REGION_IDS {
            let stock = self
                .regional_market_stock
                .entry(region.to_string())
                .or_default();
            let demand = self
                .regional_market_demand
                .entry(region.to_string())
                .or_default();
            for item in ECONOMY_ITEM_CATALOG {
                stock.entry(item.id.to_string()).or_insert_with(|| {
                    default_stock[region]
                        .get(item.id)
                        .copied()
                        .unwrap_or_default()
                });
                demand.entry(item.id.to_string()).or_insert_with(|| {
                    default_demand[region]
                        .get(item.id)
                        .copied()
                        .unwrap_or_default()
                });
            }
        }
        if let Some(mirror_stock) = self.regional_market_stock.get_mut("mirror_city") {
            for (item_id, stock) in &self.market_stock {
                mirror_stock.insert(item_id.clone(), *stock);
            }
        }
        if let Some(mirror_demand) = self.regional_market_demand.get_mut("mirror_city") {
            for (item_id, demand) in &self.market_demand {
                mirror_demand.insert(item_id.clone(), *demand);
            }
        }
        for caravan in &mut self.active_regional_caravans {
            if caravan.route_room_ids.is_empty() {
                caravan.route_room_ids =
                    Self::caravan_route(&caravan.from_region_id, &caravan.to_region_id);
            }
            caravan.route_index = caravan.route_index.min(caravan.route_room_ids.len() - 1);
            caravan.integrity = caravan.integrity.min(100);
            caravan.risk = caravan.risk.min(9);
        }
        if self.economy_mode == EconomyMode::CexConnected && self.economy_account_binding.is_none()
        {
            self.economy_mode = EconomyMode::OfflineLocal;
        }
        if let Some(binding) = &self.economy_account_binding {
            if self.wallet_snapshot.account_id.is_empty() {
                self.wallet_snapshot.account_id = binding.account_id.clone();
            }
        }
        if self.pending_economic_intents.len() > 128 {
            self.pending_economic_intents.truncate(128);
        }
        if self.verified_economic_receipts.len() > 256 {
            let keep_from = self.verified_economic_receipts.len() - 256;
            self.verified_economic_receipts.drain(..keep_from);
        }
        if self.economic_dead_letters.len() > 64 {
            let keep_from = self.economic_dead_letters.len() - 64;
            self.economic_dead_letters.drain(..keep_from);
        }
        self.economic_idempotency_keys.extend(
            self.pending_economic_intents
                .iter()
                .map(|intent| intent.idempotency_key.key.clone()),
        );
        if self.character.display_name.trim().is_empty() {
            self.apply_character_identity_name();
        }
        self.story.unlocked_room_ids.extend([
            MIRROR_SQUARE_ROOM.to_string(),
            MENTOR_HALL_ROOM.to_string(),
            CISTERN_WARD_ROOM.to_string(),
            NIGHT_WATCH_POST_ROOM.to_string(),
            WORKSHOP_GATE_ROOM.to_string(),
            MARKET_WIND_PAVILION_ROOM.to_string(),
            LANTERN_INFIRMARY_ROOM.to_string(),
            ARCHIVE_STEPS_ROOM.to_string(),
            CARAVAN_YARD_ROOM.to_string(),
        ]);
        if self.mentor_met {
            self.progression
                .world_flags
                .insert("expedition_gate_open".to_string());
            self.story
                .unlocked_room_ids
                .insert(EXPEDITION_GATE_ROOM.to_string());
        }
        if self.progression.world_flags.contains("signal_road_secured") {
            self.story
                .unlocked_room_ids
                .insert(RELAY_QUARTER_ROOM.to_string());
            self.story.current_step = StoryStepId::SignalRoadComplete;
        }
        if self
            .progression
            .world_flags
            .contains("glass_basin_wayhouse_open")
        {
            self.story.unlocked_room_ids.extend([
                GLASS_BASIN_WAYHOUSE_ROOM.to_string(),
                DEEP_RELAY_ROOM.to_string(),
                GLASS_REED_MARSH_ROOM.to_string(),
                BASIN_OBSERVATORY_ROOM.to_string(),
            ]);
        }
        if self.progression.world_flags.contains("ashen_fringe_open") {
            self.story.unlocked_room_ids.extend([
                MOON_BRIDGE_ROOM.to_string(),
                EMBER_ORCHARD_EDGE_ROOM.to_string(),
                ASH_BEACON_FIELD_ROOM.to_string(),
                CINDER_REFUGE_ROOM.to_string(),
            ]);
        }
        let mut effective_flags = self.progression.world_flags.clone();
        if self.active_title == Some(BuildTitle::RelayRunner) {
            effective_flags.insert("signal_road_secured".to_string());
        }
        if mirror_city_world_graph()
            .can_enter(self.room.id(), &effective_flags)
            .is_err()
        {
            self.room = CampaignRoom::MirrorSquare;
        }
    }

    pub fn validate(&self) -> Result<(), CampaignError> {
        if self.contract_version != CAMPAIGN_SAVE_CONTRACT {
            return Err(CampaignError::InvalidContract(
                self.contract_version.clone(),
            ));
        }
        if self.schema_revision != 10 {
            return Err(CampaignError::InvalidContract(format!(
                "unsupported campaign schema revision {}",
                self.schema_revision
            )));
        }
        if self.active_regional_quest_id.is_none() && self.active_regional_quest_step != 0 {
            return Err(CampaignError::InvalidState(
                "regional quest step exists without an active quest".to_string(),
            ));
        }
        if self.active_regional_quest_id.as_deref()
            != self
                .active_regional_quest_runtime
                .as_ref()
                .map(|runtime| runtime.quest_id.as_str())
        {
            return Err(CampaignError::InvalidState(
                "regional quest runtime does not match the active quest".to_string(),
            ));
        }
        if self.conversation_history.len() > 24 {
            return Err(CampaignError::InvalidState(
                "NPC conversation history exceeds its bounded save budget".to_string(),
            ));
        }
        if self.social_event_history.len() > 16
            || self.npc_memory.values().any(|memory| memory.len() > 8)
            || self
                .npc_bonds
                .values()
                .any(|bond| bond.unsigned_abs() > 100)
            || self.main_story_decisions.len() > 3
            || self
                .main_story_decisions
                .iter()
                .map(|decision| decision.chapter)
                .collect::<BTreeSet<_>>()
                .len()
                != self.main_story_decisions.len()
            || self.main_story_ending != resolve_main_story_ending(&self.main_story_decisions)
            || self
                .main_story_scene_progress
                .values()
                .any(|step| *step > 2)
            || self.ending_epilogue_progress > 3
            || self.ending_epilogue_complete != (self.ending_epilogue_progress >= 3)
        {
            return Err(CampaignError::InvalidState(
                "NPC social or main-story history is inconsistent or exceeds its bound".to_string(),
            ));
        }
        if self
            .technique_mastery
            .values()
            .any(|mastery| *mastery > 100)
        {
            return Err(CampaignError::InvalidState(
                "sect technique mastery exceeds its persistent cap".to_string(),
            ));
        }
        if ECONOMY_ITEM_CATALOG.iter().any(|item| {
            !self.market_stock.contains_key(item.id)
                || !self.market_demand.contains_key(item.id)
                || self.market_demand[item.id].unsigned_abs() > 20
        }) {
            return Err(CampaignError::InvalidState(
                "market stock/demand state is incomplete or out of bounds".to_string(),
            ));
        }
        if self.regional_logistics.len() > 64
            || self.active_regional_caravans.iter().any(|caravan| {
                caravan.route_room_ids.is_empty()
                    || caravan.route_index >= caravan.route_room_ids.len()
                    || caravan.integrity > 100
                    || caravan.risk > 9
            })
            || MARKET_REGION_IDS.into_iter().any(|region| {
                ECONOMY_ITEM_CATALOG.iter().any(|item| {
                    !self
                        .regional_market_stock
                        .get(region)
                        .is_some_and(|state| state.contains_key(item.id))
                        || !self
                            .regional_market_demand
                            .get(region)
                            .is_some_and(|state| {
                                state
                                    .get(item.id)
                                    .is_some_and(|demand| demand.unsigned_abs() <= 20)
                            })
                })
            })
        {
            return Err(CampaignError::InvalidState(
                "regional market or logistics state is incomplete or out of bounds".to_string(),
            ));
        }
        if self.pending_economic_intents.len() > 128
            || self.verified_economic_receipts.len() > 256
            || self.economic_dead_letters.len() > 64
            || self.pending_tradeable_purchases.len() > 32
            || self
                .pending_economic_intents
                .iter()
                .any(|intent| intent.validate().is_err())
            || self.verified_economic_receipts.iter().any(|receipt| {
                receipt.protocol_version != TERM_EXCHANGE_PROTOCOL_VERSION
                    || receipt.progression_class != receipt.status.progression_class()
            })
            || self.pending_tradeable_purchases.iter().any(|purchase| {
                purchase.quantity == 0
                    || purchase.price_wallet_credits <= 0
                    || purchase.item_id.trim().is_empty()
                    || purchase.buyer.account_id.trim().is_empty()
                    || purchase.seller.account_id.trim().is_empty()
            })
            || self.settlement_receipts.iter().any(|settlement| {
                settlement
                    .economic_receipt_id
                    .as_ref()
                    .is_some_and(|receipt_id| {
                        let Some(intent_id) = settlement.economic_intent_id.as_deref() else {
                            return true;
                        };
                        !self.verified_economic_receipts.iter().any(|receipt| {
                            receipt.receipt_id == *receipt_id && receipt.intent_id == intent_id
                        })
                    })
            })
            || self.economy_mode == EconomyMode::CexConnected
                && self.economy_account_binding.is_none()
        {
            return Err(CampaignError::InvalidState(
                "economy outbox, account binding or receipt state is invalid".to_string(),
            ));
        }
        let linked_economic_intent_ids = self
            .settlement_receipts
            .iter()
            .filter_map(|settlement| settlement.economic_intent_id.as_deref())
            .collect::<BTreeSet<_>>();
        if linked_economic_intent_ids.len()
            != self
                .settlement_receipts
                .iter()
                .filter(|settlement| settlement.economic_intent_id.is_some())
                .count()
        {
            return Err(CampaignError::InvalidState(
                "battle settlement economic intent links must be one-to-one".to_string(),
            ));
        }
        if self.active_party_ids.len() != 4 {
            return Err(CampaignError::InvalidState(
                "exactly four active party members are required".to_string(),
            ));
        }
        let hero_name = self
            .party
            .iter()
            .find(|member| member.unit_id == "hero")
            .map(|member| member.display_name.as_str());
        if self.character.display_name.trim().is_empty()
            || self.character.display_name.len() > 32
            || hero_name != Some(self.character.display_name.as_str())
            || self.character.display_name != self.character_identity.name.display_name()
        {
            return Err(CampaignError::InvalidState(
                "character identity and persistent hero name disagree".to_string(),
            ));
        }
        if self.progression.credits < 0
            || self.progression.mentor_training_sessions > MAX_MENTOR_TRAINING_SESSIONS
        {
            return Err(CampaignError::InvalidState(
                "campaign credits or mentor training count is invalid".to_string(),
            ));
        }
        if self.world_clock.day == 0
            || self.world_clock.minute_of_day >= 24 * 60
            || self.expedition_supplies.stamina > 100
            || self.expedition_supplies.rations > 12
            || self.expedition_supplies.water > 16
        {
            return Err(CampaignError::InvalidState(
                "world clock or expedition supplies are invalid".to_string(),
            ));
        }
        if let Some(chain) = &self.quest_chain {
            if chain.complete && chain.current_node != QuestChainNodeId::ReliefComplete {
                return Err(CampaignError::InvalidState(
                    "completed quest chain is not at its terminal node".to_string(),
                ));
            }
            if chain.chosen_branch.is_some()
                && chain.current_node != QuestChainNodeId::ReliefComplete
            {
                return Err(CampaignError::InvalidState(
                    "quest branch is set before the terminal decision".to_string(),
                ));
            }
        }
        let spent_growth = self
            .progression
            .growth_allocations
            .values()
            .copied()
            .sum::<u16>();
        if spent_growth.saturating_add(self.progression.growth_points_available)
            != self.progression.growth_points_awarded
            || (self.pending_growth_stat.is_some() && self.progression.growth_points_available == 0)
        {
            return Err(CampaignError::InvalidState(
                "growth point accounting is inconsistent".to_string(),
            ));
        }
        let party_ids = self
            .party
            .iter()
            .map(|member| member.unit_id.as_str())
            .collect::<BTreeSet<_>>();
        let active = self
            .active_party_ids
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let available = self
            .party
            .iter()
            .filter(|member| member.available)
            .map(|member| member.unit_id.as_str())
            .collect::<BTreeSet<_>>();
        if party_ids.len() != self.party.len()
            || active.len() != self.active_party_ids.len()
            || !active.is_subset(&party_ids)
            || !active.is_subset(&available)
        {
            return Err(CampaignError::InvalidState(
                "party ids must be unique and active ids must exist".to_string(),
            ));
        }
        match (self.phase, self.pending_battle.as_ref()) {
            (CampaignPhase::Town, None) => {}
            (CampaignPhase::BattlePending, Some(pending)) if pending.result.is_none() => {
                pending.seed.validate()?;
            }
            (CampaignPhase::PostBattlePending, Some(pending)) if pending.result.is_some() => {
                pending.seed.validate()?;
                pending
                    .result
                    .as_ref()
                    .expect("guarded result")
                    .validate_against(&pending.seed)?;
            }
            _ => {
                return Err(CampaignError::InvalidState(
                    "campaign phase and pending battle disagree".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn apply_character_identity_name(&mut self) {
        let display_name = self.character_identity.name.display_name().to_string();
        self.character.display_name = display_name.clone();
        if let Some(hero) = self
            .party
            .iter_mut()
            .find(|member| member.unit_id == "hero")
        {
            hero.display_name = display_name;
        }
    }

    pub fn cycle_character_identity(&mut self) -> Result<CharacterNamePreset, CampaignError> {
        self.require_town()?;
        if self.character_identity.confirmed {
            return Err(CampaignError::InvalidState(
                "confirmed character identity cannot be changed".to_string(),
            ));
        }
        self.character_identity.name = self.character_identity.name.next();
        self.apply_character_identity_name();
        self.revision += 1;
        Ok(self.character_identity.name)
    }

    pub fn confirm_character_identity(&mut self) -> Result<(), CampaignError> {
        self.require_town()?;
        if self.character_identity.confirmed {
            return Err(CampaignError::InvalidState(
                "character identity is already confirmed".to_string(),
            ));
        }
        self.apply_character_identity_name();
        self.character_identity.confirmed = true;
        self.revision += 1;
        self.validate()
    }

    pub fn cycle_difficulty(&mut self) -> Result<CampaignDifficulty, CampaignError> {
        self.require_town()?;
        if self.quest_state == QuestState::Accepted {
            return Err(CampaignError::InvalidState(
                "difficulty is locked after accepting a mission".to_string(),
            ));
        }
        self.difficulty = self.difficulty.next();
        self.revision += 1;
        Ok(self.difficulty)
    }

    pub fn current_guide_step(&self) -> CampaignGuideStep {
        if !self.mentor_met {
            CampaignGuideStep::MeetMentor
        } else if !self.trained_with_mentor {
            CampaignGuideStep::TrainWithMentor
        } else if !self.character.equipment_slots.contains_key("weapon") {
            CampaignGuideStep::EquipWeapon
        } else if self.room != CampaignRoom::ExpeditionGate {
            CampaignGuideStep::ReachExpeditionGate
        } else if self.quest_state == QuestState::Accepted {
            CampaignGuideStep::DeployMission
        } else if self
            .progression
            .world_flags
            .contains("mirror_siege_secured")
        {
            CampaignGuideStep::ReadJournal
        } else {
            CampaignGuideStep::AcceptMission
        }
    }

    pub fn campaign_journal(&self) -> Vec<CampaignJournalEntry> {
        let signal_state = if self
            .progression
            .world_flags
            .contains("mirror_siege_secured")
        {
            CampaignJournalState::Completed
        } else {
            match self.quest_state {
                QuestState::Locked => CampaignJournalState::Locked,
                QuestState::Available => CampaignJournalState::Available,
                QuestState::Accepted => CampaignJournalState::Active,
                QuestState::Completed => CampaignJournalState::Active,
                QuestState::Failed | QuestState::Withdrawn => CampaignJournalState::Failed,
            }
        };
        let journal_mission = if self.quest_state == QuestState::Accepted {
            self.active_mission
        } else if !self
            .progression
            .world_flags
            .contains("first_contact_secured")
        {
            CampaignMission::FirstContact
        } else if self.progression.aftershock_completions == 0 {
            CampaignMission::AftershockPatrol
        } else if !self
            .progression
            .world_flags
            .contains("convoy_exodus_secured")
        {
            CampaignMission::ConvoyExodus
        } else if !self
            .progression
            .world_flags
            .contains("mirror_siege_secured")
        {
            CampaignMission::MirrorSiege
        } else {
            CampaignMission::AftershockPatrol
        };
        let signal_objective = match journal_mission {
            CampaignMission::FirstContact => "Secure the first relay contact",
            CampaignMission::AftershockPatrol => "Break the repeatable aftershock patrol",
            CampaignMission::ConvoyExodus => "Escort, defend and extract the signal convoy",
            CampaignMission::MirrorSiege => "Break the siege and reclaim Mirror Gate",
            CampaignMission::IronDeltaSkirmish => "Win the Iron Delta score skirmish",
            CampaignMission::NightWatchCrossingSkirmish => "Escort the Night Watch patrol",
            CampaignMission::GlassBasinSkirmish => "Control the Glass Basin relay array",
            CampaignMission::EmberOrchardSkirmish => "Break the Ember Orchard base",
            CampaignMission::SaltMarshSkirmish => "Control the Salt Marsh causeway",
            CampaignMission::CinderCrownSkirmish => "Break the Cinder Crown siege",
        };
        let cistern = match self.quest_chain.as_ref() {
            None => CampaignJournalEntry {
                id: CampaignJournalId::CisternRelief,
                title: "Signal Cistern Relief".to_string(),
                state: if self
                    .progression
                    .world_flags
                    .contains("outer_signal_road_open")
                {
                    CampaignJournalState::Available
                } else {
                    CampaignJournalState::Locked
                },
                objective: "Open the outer Signal Road".to_string(),
                next_room: Some(CampaignRoom::RelayQuarter),
            },
            Some(progress) => CampaignJournalEntry {
                id: CampaignJournalId::CisternRelief,
                title: "Signal Cistern Relief".to_string(),
                state: if progress.complete {
                    CampaignJournalState::Completed
                } else {
                    CampaignJournalState::Active
                },
                objective: match progress.current_node {
                    QuestChainNodeId::SurveyDamage => "Survey the damaged cistern",
                    QuestChainNodeId::GatherSupplies => "Commit 40 credits of relief supplies",
                    QuestChainNodeId::ChooseReliefPlan => "Choose reinforce or evacuate",
                    QuestChainNodeId::ReliefComplete => "Relief plan completed",
                }
                .to_string(),
                next_room: match progress.current_node {
                    QuestChainNodeId::GatherSupplies => Some(CampaignRoom::ExpeditionGate),
                    _ => Some(CampaignRoom::RelayQuarter),
                },
            },
        };
        let mastery_state = if self.active_title.is_some() {
            CampaignJournalState::Completed
        } else if self.build_path == BuildPath::Unformed {
            CampaignJournalState::Locked
        } else {
            CampaignJournalState::Active
        };
        vec![
            CampaignJournalEntry {
                id: CampaignJournalId::SignalRoad,
                title: "Signal Road Campaign".to_string(),
                state: signal_state,
                objective: signal_objective.to_string(),
                next_room: Some(CampaignRoom::ExpeditionGate),
            },
            cistern,
            CampaignJournalEntry {
                id: CampaignJournalId::Mastery,
                title: "Path Mastery".to_string(),
                state: mastery_state,
                objective: if self.active_title.is_some() {
                    "Mastery title earned".to_string()
                } else {
                    "Choose growth, then complete the mentor challenge".to_string()
                },
                next_room: Some(CampaignRoom::MentorHall),
            },
        ]
    }

    pub fn move_to(&mut self, room: CampaignRoom) -> Result<(), CampaignError> {
        self.require_town()?;
        let mut effective_flags = self.progression.world_flags.clone();
        if self.active_title == Some(BuildTitle::RelayRunner) {
            effective_flags.insert("signal_road_secured".to_string());
        }
        mirror_city_world_graph()
            .transition(self.room.id(), room.id(), &effective_flags)
            .map_err(CampaignError::InvalidState)?;
        self.room = room;
        self.revision += 1;
        Ok(())
    }

    pub fn current_task_route_plan(&self) -> WorldRoutePlan {
        if let Some(quest_id) = self.active_regional_quest_id.as_deref() {
            if REGIONAL_QUEST_CATALOG
                .iter()
                .find(|definition| definition.id == quest_id)
                .is_some()
            {
                let remaining = self.active_regional_quest_ready_rooms();
                let mut flags = self.progression.world_flags.clone();
                if self.active_title == Some(BuildTitle::RelayRunner) {
                    flags.insert("signal_road_secured".to_string());
                }
                if let Some(route) = remaining
                    .iter()
                    .map(|room| {
                        mirror_city_world_graph().shortest_route(self.room.id(), room, &flags)
                    })
                    .filter(|route| route.reachable())
                    .min_by_key(|route| route.path.len())
                {
                    return route;
                }
                if remaining.is_empty() {
                    let runtime = self
                        .active_regional_quest_runtime
                        .as_ref()
                        .expect("active regional quest has runtime");
                    let graph = quest_condition_graph(
                        REGIONAL_QUEST_CATALOG
                            .iter()
                            .find(|definition| definition.id == quest_id)
                            .expect("active quest remains catalog bound"),
                        runtime.approach,
                    );
                    if let Some(settlement) = graph.nodes.iter().find(|node| {
                        node.kind == trnm_rpg_core::QuestConditionKind::ReturnForSettlement
                    }) {
                        return mirror_city_world_graph().shortest_route(
                            self.room.id(),
                            &settlement.subject_id,
                            &flags,
                        );
                    }
                }
                return mirror_city_world_graph().ordered_task_route(
                    self.room.id(),
                    &remaining,
                    &flags,
                );
            }
        }
        let destination = self
            .quest_chain
            .as_ref()
            .filter(|chain| !chain.complete)
            .map(|chain| match chain.current_node {
                QuestChainNodeId::SurveyDamage | QuestChainNodeId::ChooseReliefPlan => {
                    RELAY_QUARTER_ROOM
                }
                QuestChainNodeId::GatherSupplies => EXPEDITION_GATE_ROOM,
                QuestChainNodeId::ReliefComplete => RELAY_QUARTER_ROOM,
            })
            .unwrap_or_else(|| match self.story.current_step {
                StoryStepId::MeetMentor => MENTOR_HALL_ROOM,
                StoryStepId::SecureFirstContact
                | StoryStepId::BreakAftershock
                | StoryStepId::EvacuateConvoy => EXPEDITION_GATE_ROOM,
                StoryStepId::SignalRoadComplete => RELAY_QUARTER_ROOM,
            });
        let mut flags = self.progression.world_flags.clone();
        if self.active_title == Some(BuildTitle::RelayRunner) {
            flags.insert("signal_road_secured".to_string());
        }
        mirror_city_world_graph().shortest_route(self.room.id(), destination, &flags)
    }

    /// Walk one legal world edge toward the nearest currently-ready task
    /// node. This is the same authority used by manual room movement and is
    /// exposed to the client as an accessible route-follow command.
    pub fn advance_toward_current_task(&mut self) -> Result<CampaignRoom, CampaignError> {
        let route = self.current_task_route_plan();
        let next = route.path.get(1).ok_or_else(|| {
            CampaignError::InvalidState(
                route
                    .blocked_reason
                    .map(|reason| format!("task route blocked: {reason:?}"))
                    .unwrap_or_else(|| "already at the current task node".to_string()),
            )
        })?;
        let room = CampaignRoom::from_id(next).ok_or_else(|| {
            CampaignError::InvalidState(format!("task route returned unknown room {next}"))
        })?;
        self.move_to(room)?;
        Ok(room)
    }

    pub fn start_cistern_relief(&mut self) -> Result<(), CampaignError> {
        self.require_room(CampaignRoom::RelayQuarter)?;
        if !self
            .progression
            .world_flags
            .contains("outer_signal_road_open")
        {
            return Err(CampaignError::InvalidState(
                "the outer signal road must be open before cistern relief".to_string(),
            ));
        }
        if self
            .quest_chain
            .as_ref()
            .is_some_and(|chain| !chain.complete)
        {
            return Err(CampaignError::InvalidState(
                "another quest-chain step is already active".to_string(),
            ));
        }
        self.quest_chain = Some(QuestChainProgress {
            id: QuestChainId::CisternRelief,
            current_node: QuestChainNodeId::SurveyDamage,
            chosen_branch: None,
            completed_nodes: BTreeSet::new(),
            complete: false,
        });
        self.revision += 1;
        Ok(())
    }

    pub fn advance_cistern_relief(&mut self) -> Result<QuestChainNodeId, CampaignError> {
        self.require_town()?;
        let current = self
            .quest_chain
            .as_ref()
            .filter(|chain| !chain.complete)
            .map(|chain| chain.current_node)
            .ok_or_else(|| {
                CampaignError::InvalidState("cistern relief is not active".to_string())
            })?;
        let required_room = match current {
            QuestChainNodeId::SurveyDamage | QuestChainNodeId::ChooseReliefPlan => {
                CampaignRoom::RelayQuarter
            }
            QuestChainNodeId::GatherSupplies => CampaignRoom::ExpeditionGate,
            QuestChainNodeId::ReliefComplete => CampaignRoom::RelayQuarter,
        };
        if self.room != required_room {
            return Err(CampaignError::InvalidState(format!(
                "cistern relief step requires {}",
                required_room.title()
            )));
        }
        let next = match current {
            QuestChainNodeId::SurveyDamage => QuestChainNodeId::GatherSupplies,
            QuestChainNodeId::GatherSupplies => {
                if self.progression.credits < 40 {
                    return Err(CampaignError::InvalidState(
                        "gathering cistern supplies costs 40 credits".to_string(),
                    ));
                }
                self.progression.credits -= 40;
                QuestChainNodeId::ChooseReliefPlan
            }
            QuestChainNodeId::ChooseReliefPlan => {
                return Err(CampaignError::InvalidState(
                    "choose reinforce or evacuate to complete cistern relief".to_string(),
                ));
            }
            QuestChainNodeId::ReliefComplete => {
                return Err(CampaignError::InvalidState(
                    "cistern relief is already complete".to_string(),
                ));
            }
        };
        let chain = self.quest_chain.as_mut().expect("active chain was checked");
        chain.completed_nodes.insert(current);
        chain.current_node = next;
        self.revision += 1;
        Ok(next)
    }

    pub fn choose_cistern_relief_branch(
        &mut self,
        branch: QuestBranch,
    ) -> Result<(), CampaignError> {
        self.require_room(CampaignRoom::RelayQuarter)?;
        let chain = self
            .quest_chain
            .as_ref()
            .filter(|chain| {
                !chain.complete && chain.current_node == QuestChainNodeId::ChooseReliefPlan
            })
            .ok_or_else(|| {
                CampaignError::InvalidState("cistern relief is not awaiting a branch".to_string())
            })?;
        if chain.chosen_branch.is_some() {
            return Err(CampaignError::InvalidState(
                "cistern relief branch has already been chosen".to_string(),
            ));
        }
        let definition = cistern_relief_quest_chain_definition();
        let rewards = definition
            .nodes
            .iter()
            .find(|node| node.id == QuestChainNodeId::ReliefComplete && node.branch == Some(branch))
            .map(|node| node.rewards.clone())
            .ok_or_else(|| {
                CampaignError::InvalidState("missing quest branch rewards".to_string())
            })?;
        for reward in rewards {
            match reward {
                QuestChainReward::Credits { amount } => {
                    self.progression.credits = self.progression.credits.saturating_add(amount);
                }
                QuestChainReward::Reputation { amount } => {
                    self.character.attributes.reputation =
                        self.character.attributes.reputation.saturating_add(amount);
                }
                QuestChainReward::WorldFlag { flag } => {
                    self.progression.world_flags.insert(flag);
                }
                QuestChainReward::RelationshipTrust { npc_id, amount } => {
                    let relationship = self
                        .npc_relationships
                        .entry(npc_id.clone())
                        .or_insert_with(|| NpcRelationship::new(npc_id, "relay-quarter"));
                    relationship.trust = relationship.trust.saturating_add(amount);
                }
            }
        }
        let chain = self.quest_chain.as_mut().expect("active chain was checked");
        chain
            .completed_nodes
            .insert(QuestChainNodeId::ChooseReliefPlan);
        chain
            .completed_nodes
            .insert(QuestChainNodeId::ReliefComplete);
        chain.current_node = QuestChainNodeId::ReliefComplete;
        chain.chosen_branch = Some(branch);
        chain.complete = true;
        self.revision += 1;
        Ok(())
    }

    pub fn cycle_expedition_preparation(&mut self) -> Result<ExpeditionPreparation, CampaignError> {
        self.require_room(CampaignRoom::ExpeditionGate)?;
        self.selected_expedition_preparation = self.selected_expedition_preparation.next();
        self.revision += 1;
        Ok(self.selected_expedition_preparation)
    }

    fn commit_expedition_preparation(&mut self) -> Result<ExpeditionReadiness, CampaignError> {
        let preparation = self.selected_expedition_preparation;
        let mut starting_resources = 0;
        let travel_minutes = match preparation {
            ExpeditionPreparation::Immediate => {
                require_supplies(&self.expedition_supplies, 1, 2)?;
                self.expedition_supplies.stamina =
                    self.expedition_supplies.stamina.saturating_sub(20);
                self.expedition_supplies.rations -= 1;
                self.expedition_supplies.water -= 2;
                20
            }
            ExpeditionPreparation::Rested => {
                if self.progression.credits < 10 {
                    return Err(CampaignError::InvalidState(
                        "resting before departure costs 10 credits".to_string(),
                    ));
                }
                require_supplies(&self.expedition_supplies, 1, 1)?;
                self.progression.credits -= 10;
                self.expedition_supplies.stamina = 100;
                self.expedition_supplies.rations -= 1;
                self.expedition_supplies.water -= 1;
                180
            }
            ExpeditionPreparation::Supplied => {
                if self.progression.credits < 25 {
                    return Err(CampaignError::InvalidState(
                        "stocking the expedition costs 25 credits".to_string(),
                    ));
                }
                self.progression.credits -= 25;
                self.expedition_supplies.rations =
                    self.expedition_supplies.rations.saturating_add(3).min(12);
                self.expedition_supplies.water =
                    self.expedition_supplies.water.saturating_add(4).min(16);
                self.expedition_supplies.stamina =
                    self.expedition_supplies.stamina.saturating_sub(5);
                starting_resources = 50;
                35
            }
            ExpeditionPreparation::Shortcut => {
                if self.character_origin != CharacterOrigin::Scout
                    && self.active_title != Some(BuildTitle::RelayRunner)
                {
                    return Err(CampaignError::InvalidState(
                        "the shortcut requires Scout origin or Relay Runner title".to_string(),
                    ));
                }
                require_supplies(&self.expedition_supplies, 1, 2)?;
                self.expedition_supplies.stamina =
                    self.expedition_supplies.stamina.saturating_sub(10);
                self.expedition_supplies.rations -= 1;
                self.expedition_supplies.water -= 2;
                starting_resources = 20;
                10
            }
        };
        self.world_clock.advance(travel_minutes);
        Ok(ExpeditionReadiness {
            preparation,
            stamina: self.expedition_supplies.stamina,
            rations: self.expedition_supplies.rations,
            water: self.expedition_supplies.water,
            starting_resources,
            travel_minutes,
        })
    }

    pub fn cycle_character_origin(&mut self) -> Result<CharacterOrigin, CampaignError> {
        self.require_town()?;
        if self.mentor_met
            || self.progression.mentor_training_sessions > 0
            || !self.progression.growth_allocations.is_empty()
        {
            return Err(CampaignError::InvalidState(
                "character origin is fixed after mentor progression begins".to_string(),
            ));
        }
        let previous = self.character_origin;
        let next = previous.next();
        remove_origin_bonus(previous, &mut self.character.attributes);
        next.apply(&mut self.character.attributes);
        self.character_origin = next;
        self.character.skill_ids.retain(|skill| {
            !["iron_guard", "relay_overcharge", "wind_step"].contains(&skill.as_str())
        });
        self.character
            .skill_ids
            .push(next.starter_skill().to_string());
        if let Some(hero) = self
            .party
            .iter_mut()
            .find(|member| member.unit_id == "hero")
        {
            hero.attributes = self.character.attributes.clone();
            hero.skill_ids = self.character.skill_ids.clone();
        }
        self.revision += 1;
        Ok(next)
    }

    pub fn talk_to_mentor(&mut self) -> Result<(), CampaignError> {
        self.require_room(CampaignRoom::MentorHall)?;
        self.mentor_met = true;
        self.faction_rank = self.faction_rank.max(FactionRank::Initiate);
        self.npc_relationships
            .entry("street-compass-sifu".to_string())
            .or_insert_with(|| NpcRelationship::new("street-compass-sifu", "signal-road-school"))
            .apply(RelationshipAction::Talk);
        if self.quest_state == QuestState::Locked {
            self.quest_state = QuestState::Available;
        }
        self.complete_story_step(StoryStepId::MeetMentor, StoryStepId::SecureFirstContact)?;
        self.revision += 1;
        Ok(())
    }

    pub fn train_with_mentor(&mut self) -> Result<(), CampaignError> {
        self.require_room(CampaignRoom::MentorHall)?;
        if !self.mentor_met {
            return Err(CampaignError::InvalidState(
                "talk to the mentor before training".to_string(),
            ));
        }
        if self.progression.mentor_training_sessions >= MAX_MENTOR_TRAINING_SESSIONS {
            return Err(CampaignError::InvalidState(
                "mentor training cap reached; commit to the skills already learned".to_string(),
            ));
        }
        let session = self.progression.mentor_training_sessions;
        let cost = 50 + i64::from(session) * 40;
        if self.progression.credits < cost {
            return Err(CampaignError::InvalidState(format!(
                "mentor training costs {cost} credits"
            )));
        }
        self.progression.credits -= cost;
        self.progression.mentor_training_sessions += 1;
        self.trained_with_mentor = true;
        self.npc_relationships
            .get_mut("street-compass-sifu")
            .expect("mentor relationship exists")
            .apply(RelationshipAction::Train);
        let skill_id = self.selected_training_path.skill_id().to_string();
        if !self.character.skill_ids.contains(&skill_id) {
            self.character.skill_ids.push(skill_id.clone());
        }
        let progress = self
            .progression
            .skill_progress
            .entry(skill_id)
            .or_insert(SkillProgress {
                rank: 0,
                experience: 0,
            });
        progress.experience += 125;
        progress.rank = (1 + progress.experience / 250) as u16;
        self.revision += 1;
        Ok(())
    }

    pub fn cycle_training_path(&mut self) -> Result<(), CampaignError> {
        self.require_room(CampaignRoom::MentorHall)?;
        self.selected_training_path = self.selected_training_path.next();
        self.revision += 1;
        Ok(())
    }

    pub fn preview_growth_allocation(&mut self, stat: GrowthStat) -> Result<(), CampaignError> {
        self.require_town()?;
        if self.progression.growth_points_available == 0 {
            return Err(CampaignError::InvalidState(
                "no growth points are available".to_string(),
            ));
        }
        self.pending_growth_stat = Some(stat);
        self.revision += 1;
        Ok(())
    }

    pub fn cycle_growth_preview(&mut self) -> Result<GrowthStat, CampaignError> {
        let next = self.pending_growth_stat.unwrap_or_default().next();
        self.preview_growth_allocation(next)?;
        Ok(next)
    }

    pub fn cancel_growth_allocation(&mut self) -> Result<(), CampaignError> {
        self.require_town()?;
        if self.pending_growth_stat.take().is_none() {
            return Err(CampaignError::InvalidState(
                "no growth allocation is awaiting confirmation".to_string(),
            ));
        }
        self.revision += 1;
        Ok(())
    }

    pub fn confirm_growth_allocation(&mut self) -> Result<GrowthStat, CampaignError> {
        self.require_town()?;
        let stat = self.pending_growth_stat.take().ok_or_else(|| {
            CampaignError::InvalidState(
                "preview a growth allocation before confirming it".to_string(),
            )
        })?;
        if self.progression.growth_points_available == 0 {
            return Err(CampaignError::InvalidState(
                "growth point was already consumed".to_string(),
            ));
        }
        stat.apply(&mut self.character.attributes, 1);
        if let Some(hero) = self
            .party
            .iter_mut()
            .find(|member| member.unit_id == "hero")
        {
            hero.attributes = self.character.attributes.clone();
        }
        self.progression.growth_points_available -= 1;
        *self.progression.growth_allocations.entry(stat).or_default() += 1;
        let path = match stat {
            GrowthStat::Force | GrowthStat::Physique | GrowthStat::Resolve => BuildPath::Vanguard,
            GrowthStat::Agility | GrowthStat::Insight => BuildPath::Windrunner,
            GrowthStat::Craft | GrowthStat::Commerce => BuildPath::Artificer,
        };
        self.build_path = path;
        self.active_title = None;
        self.character.title = format!("{} Aspirant", path.display_name());
        self.progression.world_flags.insert(format!(
            "{}_path_chosen",
            path.display_name().to_ascii_lowercase()
        ));
        self.revision += 1;
        self.validate()?;
        Ok(stat)
    }

    pub fn attempt_mastery_challenge(&mut self) -> Result<BuildTitle, CampaignError> {
        self.require_room(CampaignRoom::MentorHall)?;
        let challenge = MasteryChallenge::for_path(self.build_path).ok_or_else(|| {
            CampaignError::InvalidState("choose a growth path before mastery".to_string())
        })?;
        if !self.trained_with_mentor {
            return Err(CampaignError::InvalidState(
                "complete mentor training before the mastery challenge".to_string(),
            ));
        }
        let success = match challenge {
            MasteryChallenge::VanguardStand => {
                self.spar_with_mentor()?.outcome == SparringOutcome::Victory
            }
            MasteryChallenge::WindrunnerCircuit => self.character.attributes.agility >= 13,
            MasteryChallenge::ArtificerCommission => {
                if self.progression.credits < 25 {
                    false
                } else {
                    self.progression.credits -= 25;
                    self.character.attributes.craft >= 11
                }
            }
        };
        if !success {
            return Err(CampaignError::InvalidState(
                "mastery challenge requirements were not met".to_string(),
            ));
        }
        let title = challenge.title();
        self.unlocked_titles.insert(title);
        self.active_title = Some(title);
        self.character.title = title.display_name().to_string();
        let flag = match title {
            BuildTitle::GateWarden => "gate_warden_route",
            BuildTitle::RelayRunner => "relay_runner_shortcut",
            BuildTitle::ForgeMaster => "forge_master_prices",
        };
        self.progression.world_flags.insert(flag.to_string());
        if title == BuildTitle::RelayRunner {
            self.story
                .unlocked_room_ids
                .insert(RELAY_QUARTER_ROOM.to_string());
        }
        self.revision += 1;
        Ok(title)
    }

    pub fn cycle_active_title(&mut self) -> Result<BuildTitle, CampaignError> {
        self.require_town()?;
        if self.unlocked_titles.is_empty() {
            return Err(CampaignError::InvalidState(
                "allocate a growth point before choosing a title".to_string(),
            ));
        }
        let titles = self.unlocked_titles.iter().copied().collect::<Vec<_>>();
        let next = self
            .active_title
            .and_then(|current| titles.iter().position(|title| *title == current))
            .map(|index| titles[(index + 1) % titles.len()])
            .unwrap_or(titles[0]);
        self.active_title = Some(next);
        self.character.title = next.display_name().to_string();
        self.revision += 1;
        Ok(next)
    }

    pub fn begin_signal_road_encounter(&mut self) -> Result<(), CampaignError> {
        self.require_town()?;
        if self.room != CampaignRoom::RelayQuarter
            && !(self.room == CampaignRoom::ExpeditionGate
                && self.active_title == Some(BuildTitle::GateWarden))
        {
            return Err(CampaignError::InvalidState(
                "the ambush is reachable from Relay Quarter or the Gate Warden route".to_string(),
            ));
        }
        if self.active_encounter.is_some() {
            return Err(CampaignError::InvalidState(
                "an RPG encounter is already active".to_string(),
            ));
        }
        self.active_encounter =
            RpgEncounterState::from_definition("signal_road_ambush", &self.character.attributes);
        let primary = self.active_technique_style();
        let secondary = self.secondary_technique_style();
        let primary_rank = self.technique_rank(primary);
        let secondary_rank = self.technique_rank(secondary);
        if let Some(encounter) = &mut self.active_encounter {
            encounter.set_technique_loadout(primary, primary_rank, secondary, secondary_rank);
        }
        self.last_encounter_outcome = None;
        self.revision += 1;
        Ok(())
    }

    pub fn begin_regional_encounter(&mut self, encounter_id: &str) -> Result<(), CampaignError> {
        self.require_town()?;
        if self.active_encounter.is_some() {
            return Err(CampaignError::InvalidState(
                "an RPG encounter is already active".to_string(),
            ));
        }
        self.active_encounter =
            RpgEncounterState::from_definition(encounter_id, &self.character.attributes);
        if self.active_encounter.is_none() {
            return Err(CampaignError::InvalidState(format!(
                "unknown regional encounter: {encounter_id}"
            )));
        }
        let primary = self.active_technique_style();
        let secondary = self.secondary_technique_style();
        let primary_rank = self.technique_rank(primary);
        let secondary_rank = self.technique_rank(secondary);
        if let Some(encounter) = &mut self.active_encounter {
            encounter.set_technique_loadout(primary, primary_rank, secondary, secondary_rank);
        }
        self.last_encounter_outcome = None;
        self.revision += 1;
        Ok(())
    }

    fn technique_style_for_slot(&self, slot: u8) -> TechniqueStyle {
        let slot = slot % 3;
        match (current_sect(&self.character), slot) {
            (Some(SectId::StreetCompass), 0) => TechniqueStyle::CompassFeint,
            (Some(SectId::StreetCompass), 1) => TechniqueStyle::CompassSpiral,
            (Some(SectId::StreetCompass), _) => TechniqueStyle::WayfinderSlip,
            (Some(SectId::IronWorkshop), 0) => TechniqueStyle::ForgeCounter,
            (Some(SectId::IronWorkshop), 1) => TechniqueStyle::RelayHammer,
            (Some(SectId::IronWorkshop), _) => TechniqueStyle::IronReversal,
            (Some(SectId::NightWatch), 0) => TechniqueStyle::NightVeil,
            (Some(SectId::NightWatch), 1) => TechniqueStyle::ShadowNeedle,
            (Some(SectId::NightWatch), _) => TechniqueStyle::LanternCut,
            (None, _) => TechniqueStyle::CenterlineBreak,
        }
    }

    fn active_technique_style(&self) -> TechniqueStyle {
        self.technique_style_for_slot(self.equipped_technique_slot)
    }

    pub fn secondary_technique_style(&self) -> TechniqueStyle {
        self.technique_style_for_slot(self.secondary_technique_slot)
    }

    fn technique_rank(&self, style: TechniqueStyle) -> u8 {
        self.technique_mastery
            .get(style.rule_id())
            .copied()
            .unwrap_or_default()
            .saturating_div(10)
            .min(10) as u8
    }

    pub fn cycle_equipped_technique(&mut self) -> Result<TechniqueStyle, CampaignError> {
        self.require_town()?;
        if current_sect(&self.character).is_none() {
            return Err(CampaignError::InvalidState(
                "join a regional sect before configuring techniques".to_string(),
            ));
        }
        self.equipped_technique_slot = (self.equipped_technique_slot + 1) % 3;
        self.revision += 1;
        Ok(self.active_technique_style())
    }

    pub fn cycle_secondary_equipped_technique(&mut self) -> Result<TechniqueStyle, CampaignError> {
        self.require_town()?;
        if current_sect(&self.character).is_none() {
            return Err(CampaignError::InvalidState(
                "join a regional sect before configuring techniques".to_string(),
            ));
        }
        self.secondary_technique_slot = (self.secondary_technique_slot + 1) % 3;
        if self.secondary_technique_slot == self.equipped_technique_slot {
            self.secondary_technique_slot = (self.secondary_technique_slot + 1) % 3;
        }
        self.revision += 1;
        Ok(self.secondary_technique_style())
    }

    pub fn act_in_signal_road_encounter(
        &mut self,
        action: EncounterAction,
    ) -> Result<Option<EncounterOutcome>, CampaignError> {
        self.require_town()?;
        let item_available = self
            .progression
            .inventory
            .iter()
            .any(|stack| stack.item_id == "field-tonic-kit" && stack.quantity > 0);
        let encounter = self
            .active_encounter
            .as_mut()
            .ok_or_else(|| CampaignError::InvalidState("no RPG encounter is active".to_string()))?;
        let technique_style = encounter.next_technique_style();
        let turn = encounter
            .advance(&self.character.attributes, action, item_available)
            .map_err(CampaignError::InvalidState)?;
        let encounter_id = encounter.encounter_id.clone();
        let encounter_round = encounter.round;
        if matches!(
            action,
            EncounterAction::Technique
                | EncounterAction::PrimaryTechnique
                | EncounterAction::SecondaryTechnique
        ) {
            let mastery = self
                .technique_mastery
                .entry(technique_style.rule_id().to_string())
                .or_default();
            *mastery = mastery.saturating_add(1).min(100);
        }
        if turn.item_consumed {
            consume_loot(&mut self.progression.inventory, "field-tonic-kit", 1)?;
        }
        if let Some(outcome) = turn.outcome {
            self.last_encounter_outcome = Some(outcome);
            match outcome {
                EncounterOutcome::Victory => {
                    self.progression.experience = self.progression.experience.saturating_add(80);
                    self.character.attributes.reputation =
                        self.character.attributes.reputation.saturating_add(2);
                    let loot = ENCOUNTER_CATALOG
                        .iter()
                        .find(|definition| definition.id == encounter_id)
                        .map(|definition| {
                            definition
                                .loot_table
                                .iter()
                                .map(|item_id| LootStack {
                                    item_id: (*item_id).to_string(),
                                    quantity: 1,
                                })
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_else(|| {
                            vec![LootStack {
                                item_id: "signal-road-emblem".to_string(),
                                quantity: 1,
                            }]
                        });
                    merge_loot(&mut self.progression.inventory, &loot);
                    self.progression
                        .world_flags
                        .insert(format!("{encounter_id}_cleared"));
                }
                EncounterOutcome::Defeat => {
                    if let Some(hero) = self
                        .party
                        .iter_mut()
                        .find(|member| member.unit_id == "hero")
                    {
                        hero.injury_level = hero.injury_level.saturating_add(1).min(4);
                    }
                    self.progression
                        .world_flags
                        .insert(format!("{encounter_id}_defeat"));
                }
                EncounterOutcome::Withdrawn => {
                    self.progression
                        .world_flags
                        .insert(format!("{encounter_id}_withdrawn"));
                }
            }
            self.combat_log = original_combat_log(
                &encounter_id,
                encounter_round,
                outcome == EncounterOutcome::Victory,
            );
            self.active_encounter = None;
        }
        self.revision += 1;
        Ok(turn.outcome)
    }

    pub fn join_regional_sect(&mut self, sect: SectId) -> Result<(), CampaignError> {
        self.require_town()?;
        let definition = SECT_CATALOG
            .iter()
            .find(|definition| definition.id == sect)
            .expect("three authored sects are static");
        if self.room.id() != definition.hall_room_id {
            return Err(CampaignError::InvalidState(format!(
                "{} requires visiting {}",
                definition.display_name, definition.hall_room_id
            )));
        }
        if self
            .character
            .sect_id
            .as_deref()
            .is_some_and(|current| current != sect.id())
        {
            return Err(CampaignError::InvalidState(
                "a character may commit to only one regional sect".to_string(),
            ));
        }
        self.character.sect_id = Some(sect.id().to_string());
        if !self
            .character
            .skill_ids
            .iter()
            .any(|skill| skill == definition.entry_skill_id)
        {
            self.character
                .skill_ids
                .push(definition.entry_skill_id.to_string());
        }
        self.progression
            .skill_progress
            .entry(definition.entry_skill_id.to_string())
            .or_insert(SkillProgress {
                rank: 1,
                experience: 0,
            });
        self.npc_relationships
            .entry(definition.mentor_id.to_string())
            .or_insert_with(|| NpcRelationship::new(definition.mentor_id, sect.id()))
            .apply(RelationshipAction::Train);
        self.revision += 1;
        Ok(())
    }

    pub fn train_next_sect_skill(&mut self) -> Result<String, CampaignError> {
        self.require_town()?;
        let sect = current_sect(&self.character).ok_or_else(|| {
            CampaignError::InvalidState(
                "join one regional sect before advanced training".to_string(),
            )
        })?;
        let known = self
            .character
            .skill_ids
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        let skill = SKILL_CATALOG
            .iter()
            .find(|skill| {
                skill.sect == Some(sect)
                    && !known.contains(skill.id)
                    && skill_unlockable(skill.id, &known, Some(sect))
            })
            .ok_or_else(|| {
                CampaignError::InvalidState("no next sect skill is unlockable".to_string())
            })?;
        if self.progression.credits < 35 {
            return Err(CampaignError::InvalidState(
                "advanced sect training costs 35 credits".to_string(),
            ));
        }
        self.progression.credits -= 35;
        self.character.skill_ids.push(skill.id.to_string());
        self.progression.skill_progress.insert(
            skill.id.to_string(),
            SkillProgress {
                rank: 1,
                experience: 0,
            },
        );
        self.revision += 1;
        Ok(skill.id.to_string())
    }

    pub fn wait_in_town(&mut self, minutes: u32) -> Result<(), CampaignError> {
        self.require_town()?;
        if !(30..=360).contains(&minutes) {
            return Err(CampaignError::InvalidState(
                "town waiting must be between 30 minutes and 6 hours".to_string(),
            ));
        }
        self.world_clock.advance(minutes);
        self.expedition_supplies.stamina = self
            .expedition_supplies
            .stamina
            .saturating_add((minutes / 30) as u8)
            .min(100);
        self.apply_current_social_event();
        self.run_regional_logistics();
        self.revision += 1;
        Ok(())
    }

    fn apply_current_social_event(&mut self) {
        let event = npc_social_event(self.world_clock.day, self.world_clock.minute_of_day);
        if self
            .social_event_history
            .last()
            .is_some_and(|record| record.event_id == event.id && record.day == self.world_clock.day)
        {
            return;
        }
        for npc_id in [event.first_npc_id, event.second_npc_id] {
            if let Some(relationship) = self.npc_relationships.get_mut(npc_id) {
                relationship.apply(RelationshipAction::Talk);
            }
            let memory = self.npc_memory.entry(npc_id.to_string()).or_default();
            memory.push(event.text.to_string());
            if memory.len() > 8 {
                memory.remove(0);
            }
            let work_output = self.npc_work_output.entry(npc_id.to_string()).or_default();
            *work_output = work_output.saturating_add(1);
        }
        let mut pair = [event.first_npc_id, event.second_npc_id];
        pair.sort_unstable();
        let bond_key = format!("{}::{}", pair[0], pair[1]);
        let bond = self.npc_bonds.entry(bond_key).or_default();
        *bond = bond.saturating_add(2).clamp(-100, 100);
        let bond_strength = *bond;
        let production = [event.first_npc_id, event.second_npc_id]
            .into_iter()
            .map(|npc_id| {
                self.npc_work_output
                    .get(npc_id)
                    .copied()
                    .unwrap_or_default()
            })
            .sum::<u32>()
            / 4
            + u32::from(bond_strength >= 8);
        let region_id = Self::region_id_for_room_id(event.room_id);
        let (current_stock, demand) = self.regional_market_state(region_id, event.market_item_id);
        let next_stock = if event.stock_delta >= 0 {
            current_stock
                .saturating_add(event.stock_delta as u16)
                .saturating_add(production.min(6) as u16)
                .min(99)
        } else {
            current_stock.saturating_sub(event.stock_delta.unsigned_abs())
        };
        self.set_regional_market_state(
            region_id,
            event.market_item_id,
            next_stock,
            (demand + event.demand_delta - i16::from(bond_strength >= 8)).clamp(-20, 20),
        );
        for (npc_id, counterpart) in [
            (event.first_npc_id, event.second_npc_id),
            (event.second_npc_id, event.first_npc_id),
        ] {
            let work = self
                .npc_work_output
                .get(npc_id)
                .copied()
                .unwrap_or_default();
            let (kind, target_id) = if bond_strength >= 12 {
                (NpcAutonomousGoalKind::FormAlliance, counterpart.to_string())
            } else if bond_strength < 0 {
                (
                    NpcAutonomousGoalKind::ResolveConflict,
                    counterpart.to_string(),
                )
            } else if work > 0 && work.is_multiple_of(7) {
                (NpcAutonomousGoalKind::Migrate, region_id.to_string())
            } else if work > 0 && work.is_multiple_of(5) {
                (
                    NpcAutonomousGoalKind::PublishTask,
                    event.market_item_id.to_string(),
                )
            } else {
                (
                    NpcAutonomousGoalKind::Produce,
                    event.market_item_id.to_string(),
                )
            };
            let goal = self
                .npc_autonomous_goals
                .entry(npc_id.to_string())
                .or_insert(NpcAutonomousGoal {
                    kind,
                    target_id: target_id.clone(),
                    region_id: region_id.to_string(),
                    progress: 0,
                });
            goal.kind = kind;
            goal.target_id = target_id;
            goal.region_id = region_id.to_string();
            goal.progress = goal.progress.saturating_add(1);
            match goal.kind {
                NpcAutonomousGoalKind::Migrate => {
                    let destination = match region_id {
                        "mirror_city" => RELAY_QUARTER_ROOM,
                        "signal_road" => GLASS_BASIN_WAYHOUSE_ROOM,
                        "glass_basin" => MOON_BRIDGE_ROOM,
                        _ => MARKET_WIND_PAVILION_ROOM,
                    };
                    self.npc_goal_rooms
                        .insert(npc_id.to_string(), destination.to_string());
                    self.progression
                        .world_flags
                        .insert(format!("npc_{npc_id}_migrated_to_{destination}"));
                }
                NpcAutonomousGoalKind::FormAlliance => {
                    self.progression.world_flags.insert(format!(
                        "npc_alliance_{}",
                        [npc_id, counterpart]
                            .into_iter()
                            .collect::<Vec<_>>()
                            .join("_")
                    ));
                }
                NpcAutonomousGoalKind::ResolveConflict => {
                    self.progression
                        .world_flags
                        .insert(format!("npc_conflict_{}_{}", npc_id, counterpart));
                }
                NpcAutonomousGoalKind::PublishTask => {
                    self.progression.world_flags.insert(format!(
                        "npc_dynamic_task_{npc_id}_{}",
                        event.market_item_id
                    ));
                }
                NpcAutonomousGoalKind::Produce => {}
            }
        }
        self.social_event_history.push(NpcSocialEventRecord {
            event_id: event.id.to_string(),
            first_npc_id: event.first_npc_id.to_string(),
            second_npc_id: event.second_npc_id.to_string(),
            room_id: event.room_id.to_string(),
            text: event.text.to_string(),
            day: self.world_clock.day,
            minute_of_day: self.world_clock.minute_of_day,
        });
        if self.social_event_history.len() > 16 {
            self.social_event_history.remove(0);
        }
    }

    pub fn cycle_main_story_choice(&mut self) -> Result<MainStoryChoice, CampaignError> {
        self.require_town()?;
        self.main_story_choice = self.main_story_choice.next();
        self.progression
            .world_flags
            .retain(|flag| !flag.starts_with("main_story_choice_"));
        self.progression
            .world_flags
            .insert(format!("main_story_choice_{:?}", self.main_story_choice).to_ascii_lowercase());
        self.revision += 1;
        Ok(self.main_story_choice)
    }

    pub fn current_regional_npc(&self) -> Option<&'static trnm_rpg_core::NpcDefinition> {
        NPC_CATALOG
            .iter()
            .find(|npc| {
                npc.room_id == self.room.id()
                    && self
                        .npc_goal_rooms
                        .get(npc.id)
                        .map(String::as_str)
                        .or_else(|| npc_room_at(npc.id, self.world_clock.minute_of_day))
                        == Some(self.room.id())
            })
            .or_else(|| {
                NPC_CATALOG.iter().find(|npc| {
                    self.npc_goal_rooms
                        .get(npc.id)
                        .map(String::as_str)
                        .or_else(|| npc_room_at(npc.id, self.world_clock.minute_of_day))
                        == Some(self.room.id())
                })
            })
    }

    pub fn current_regional_npc_summary(&self) -> Option<String> {
        let npc = self.current_regional_npc()?;
        let schedule = npc_schedule(npc.id)?;
        let relationship = self.npc_relationships.get(npc.id);
        Some(format!(
            "{} ({:?}) | {} | trust {} | interactions {} | {} now",
            npc.display_name,
            npc.role,
            schedule.activity,
            relationship.map(|value| value.trust).unwrap_or(0),
            relationship.map(|value| value.interactions).unwrap_or(0),
            "present at this scheduled location"
        ))
    }

    pub fn current_regional_npc_interactions(&self) -> u16 {
        self.current_regional_npc()
            .and_then(|npc| self.npc_relationships.get(npc.id))
            .map(|relationship| relationship.interactions)
            .unwrap_or(0)
    }

    pub fn has_current_regional_npc(&self) -> bool {
        self.current_regional_npc().is_some()
    }

    pub fn talk_to_regional_npc(&mut self) -> Result<NpcConversationRecord, CampaignError> {
        self.require_town()?;
        let npc = self.current_regional_npc().ok_or_else(|| {
            CampaignError::InvalidState("there is no regional NPC in this room".to_string())
        })?;
        let schedule = npc_schedule(npc.id).expect("catalog NPC schedules are complete");
        let completed_tasks = npc
            .task_ids
            .iter()
            .filter(|quest_id| {
                self.regional_quest_states.get(**quest_id) == Some(&QuestState::Completed)
            })
            .count();
        let relationship = self
            .npc_relationships
            .entry(npc.id.to_string())
            .or_insert_with(|| NpcRelationship::new(npc.id, npc.faction_id));
        let first_interaction = relationship.interactions == 0;
        relationship.apply(RelationshipAction::Talk);
        if npc.id == "relay-smith-brann" && first_interaction {
            relationship.apply(RelationshipAction::CompleteMission);
            self.faction_rank = self.faction_rank.max(FactionRank::Envoy);
        }
        let stage = RelationshipStage::from_trust(relationship.trust, completed_tasks);
        let baseline = npc_dialogue(npc.id, relationship.trust, completed_tasks)
            .expect("catalog NPC dialogue is complete");
        let response = npc_choice_dialogue(npc.id, stage, self.dialogue_choice);
        match self.dialogue_choice {
            DialogueChoice::AskForWork => {}
            DialogueChoice::OfferHelp => {
                relationship.apply(RelationshipAction::Train);
            }
            DialogueChoice::ShareNews if completed_tasks > 0 => {
                relationship.apply(RelationshipAction::CompleteMission);
            }
            DialogueChoice::ShareNews => {}
        }
        let remembered = self
            .npc_memory
            .get(npc.id)
            .and_then(|memory| memory.last())
            .map(|memory| format!(" I remember: {memory}"))
            .unwrap_or_default();
        let work_output = self
            .npc_work_output
            .get(npc.id)
            .copied()
            .unwrap_or_default();
        let strongest_bond = self
            .npc_bonds
            .iter()
            .filter(|(pair, _)| pair.split("::").any(|member| member == npc.id))
            .map(|(_, bond)| *bond)
            .max()
            .unwrap_or_default();
        let goal = self
            .npc_autonomous_goals
            .get(npc.id)
            .map(|goal| {
                format!(
                    "{:?} {} in {} ({})",
                    goal.kind, goal.target_id, goal.region_id, goal.progress
                )
            })
            .unwrap_or_else(|| "building trust before committing scarce stock".to_string());
        let line = format!(
            "[{stage:?} / {:?}] {baseline} {response}{remembered} Current goal: {goal} (work {work_output}, bond {strongest_bond:+}).",
            self.dialogue_choice,
        );
        let record = NpcConversationRecord {
            npc_id: npc.id.to_string(),
            line,
            activity: format!("{}; currently in {}", schedule.activity, self.room.id()),
            day: self.world_clock.day,
            minute_of_day: self.world_clock.minute_of_day,
        };
        self.last_npc_conversation = Some(record.clone());
        self.conversation_history.push(record.clone());
        if self.conversation_history.len() > 24 {
            self.conversation_history.remove(0);
        }
        self.revision += 1;
        Ok(record)
    }

    pub fn cycle_dialogue_choice(&mut self) -> Result<DialogueChoice, CampaignError> {
        self.require_town()?;
        self.dialogue_choice = self.dialogue_choice.next();
        self.revision += 1;
        Ok(self.dialogue_choice)
    }

    pub fn active_regional_quest_objective(&self) -> Option<String> {
        let quest_id = self.active_regional_quest_id.as_deref()?;
        let definition = REGIONAL_QUEST_CATALOG
            .iter()
            .find(|definition| definition.id == quest_id)?;
        let ready_rooms = self.active_regional_quest_ready_rooms();
        if !ready_rooms.is_empty() {
            let runtime = self.active_regional_quest_runtime.as_ref()?;
            return Some(format!(
                "Authored graph {}/{}: choose ready node [{}] | {:?} approach | deadline day {}",
                self.active_regional_quest_step,
                definition.waypoint_room_ids.len(),
                ready_rooms.join(" / "),
                runtime.approach,
                runtime.deadline_day,
            ));
        }
        definition
            .encounter_id
            .map(|encounter| format!("Win {encounter}, then report to the quest giver"))
            .or_else(|| Some("Return for settlement".to_string()))
    }

    pub fn active_regional_quest_ready_rooms(&self) -> Vec<String> {
        let Some(quest_id) = self.active_regional_quest_id.as_deref() else {
            return Vec::new();
        };
        let Some(definition) = REGIONAL_QUEST_CATALOG
            .iter()
            .find(|definition| definition.id == quest_id)
        else {
            return Vec::new();
        };
        let Some(runtime) = self.active_regional_quest_runtime.as_ref() else {
            return Vec::new();
        };
        let graph = quest_condition_graph(definition, runtime.approach);
        graph
            .nodes
            .iter()
            .filter(|node| node.kind == trnm_rpg_core::QuestConditionKind::VisitWaypoint)
            .filter(|node| !runtime.completed_condition_node_ids.contains(&node.id))
            .filter(|node| {
                node.exclusive_group.as_ref().is_none_or(|group| {
                    !graph.nodes.iter().any(|candidate| {
                        candidate.exclusive_group.as_ref() == Some(group)
                            && runtime.completed_condition_node_ids.contains(&candidate.id)
                    })
                })
            })
            .filter(|node| {
                quest_graph_node_ready(&graph, &node.id, &runtime.completed_condition_node_ids)
            })
            .map(|node| node.subject_id.clone())
            .collect()
    }

    pub fn start_regional_quest(&mut self, quest_id: &str) -> Result<(), CampaignError> {
        self.require_town()?;
        let definition = REGIONAL_QUEST_CATALOG
            .iter()
            .find(|definition| definition.id == quest_id)
            .ok_or_else(|| {
                CampaignError::InvalidState(format!("unknown regional quest: {quest_id}"))
            })?;
        let giver = NPC_CATALOG
            .iter()
            .find(|npc| npc.id == definition.giver_npc_id)
            .expect("quest giver is catalog validated");
        if self.room.id() != giver.room_id {
            return Err(CampaignError::InvalidState(format!(
                "{} is offered in {}",
                definition.title, giver.room_id
            )));
        }
        if self.active_regional_quest_id.is_some() {
            return Err(CampaignError::InvalidState(
                "finish the active regional quest before accepting another".to_string(),
            ));
        }
        if self
            .npc_relationships
            .get(giver.id)
            .is_none_or(|relationship| relationship.interactions == 0)
        {
            return Err(CampaignError::InvalidState(format!(
                "talk to {} before accepting {}",
                giver.display_name, definition.title
            )));
        }
        if !matches!(
            self.regional_quest_states.get(quest_id),
            Some(QuestState::Available | QuestState::Failed | QuestState::Withdrawn)
        ) {
            return Err(CampaignError::InvalidState(format!(
                "{} is not available for acceptance",
                definition.title
            )));
        }
        self.regional_quest_states
            .insert(quest_id.to_string(), QuestState::Accepted);
        self.active_regional_quest_id = Some(quest_id.to_string());
        self.active_regional_quest_step = 0;
        let failures = self
            .regional_quest_failure_counts
            .get(quest_id)
            .copied()
            .unwrap_or_default();
        let rule = quest_runtime_rule(definition.archetype);
        self.active_regional_quest_runtime = Some(RegionalQuestRuntime::new(
            quest_id,
            self.world_clock.day,
            rule.deadline_days,
            failures,
        ));
        self.revision += 1;
        Ok(())
    }

    pub fn start_first_regional_quest_here(&mut self) -> Result<String, CampaignError> {
        let quest_id = NPC_CATALOG
            .iter()
            .filter(|npc| npc.room_id == self.room.id())
            .flat_map(|npc| npc.task_ids.iter().copied())
            .find(|quest_id| {
                matches!(
                    self.regional_quest_states.get(*quest_id),
                    Some(QuestState::Available | QuestState::Failed | QuestState::Withdrawn)
                )
            })
            .ok_or_else(|| {
                CampaignError::InvalidState("no available regional quest in this room".to_string())
            })?
            .to_string();
        self.start_regional_quest(&quest_id)?;
        Ok(quest_id)
    }

    pub fn advance_active_regional_quest(&mut self) -> Result<(), CampaignError> {
        let quest_id = self.active_regional_quest_id.clone().ok_or_else(|| {
            CampaignError::InvalidState("no regional quest is active".to_string())
        })?;
        let definition = REGIONAL_QUEST_CATALOG
            .iter()
            .find(|definition| definition.id == quest_id)
            .expect("active regional quest remains catalog bound");
        if self
            .active_regional_quest_runtime
            .as_ref()
            .is_some_and(|runtime| self.world_clock.day > runtime.deadline_day)
        {
            return self.fail_active_regional_quest("the quest deadline expired");
        }
        let ready_rooms = self.active_regional_quest_ready_rooms();
        if !ready_rooms.is_empty() {
            let runtime = self.active_regional_quest_runtime.as_mut().ok_or_else(|| {
                CampaignError::InvalidState("regional quest runtime is missing".to_string())
            })?;
            let condition_graph = quest_condition_graph(definition, runtime.approach);
            let node = condition_graph
                .nodes
                .iter()
                .filter(|node| node.kind == trnm_rpg_core::QuestConditionKind::VisitWaypoint)
                .find(|node| {
                    node.subject_id == self.room.id()
                        && !runtime.completed_condition_node_ids.contains(&node.id)
                        && quest_graph_node_ready(
                            &condition_graph,
                            &node.id,
                            &runtime.completed_condition_node_ids,
                        )
                });
            let Some(node) = node else {
                return Err(CampaignError::InvalidState(format!(
                    "{} requires one of the currently ready authored nodes [{}], current room is {}",
                    definition.title,
                    ready_rooms.join(" / "),
                    self.room.id(),
                )));
            };
            let node_id = node.id.clone();
            let consequence_flag = node.consequence_flag.clone();
            runtime.completed_condition_node_ids.insert(node_id);
            self.active_regional_quest_step = runtime
                .completed_condition_node_ids
                .iter()
                .filter(|node_id| node_id.contains("_waypoint_"))
                .count();
            runtime.evidence_count = runtime.evidence_count.saturating_add(1);
            if let Some(flag) = consequence_flag {
                self.progression.world_flags.insert(flag);
            }
            self.revision += 1;
            return Ok(());
        }
        if self
            .active_regional_quest_runtime
            .as_ref()
            .is_some_and(|runtime| runtime.approach == QuestApproach::Direct)
        {
            if let Some(encounter_id) = definition.encounter_id {
                let cleared = format!("{encounter_id}_cleared");
                if !self.progression.world_flags.contains(&cleared) {
                    return self.begin_regional_encounter(encounter_id);
                }
            }
        }
        self.complete_regional_quest()
    }

    pub fn complete_regional_quest(&mut self) -> Result<(), CampaignError> {
        self.require_town()?;
        let quest_id = self.active_regional_quest_id.clone().ok_or_else(|| {
            CampaignError::InvalidState("no regional quest is active".to_string())
        })?;
        let definition = REGIONAL_QUEST_CATALOG
            .iter()
            .find(|definition| definition.id == quest_id)
            .expect("active regional quest remains catalog bound");
        let runtime = self.active_regional_quest_runtime.clone().ok_or_else(|| {
            CampaignError::InvalidState("regional quest runtime is missing".to_string())
        })?;
        let condition_graph = quest_condition_graph(definition, runtime.approach);
        let unfinished_waypoints = condition_graph
            .nodes
            .iter()
            .filter(|node| node.kind == trnm_rpg_core::QuestConditionKind::VisitWaypoint)
            .filter(|node| {
                !quest_route_node_satisfied(
                    &condition_graph,
                    node,
                    &runtime.completed_condition_node_ids,
                )
            })
            .map(|node| node.subject_id.clone())
            .collect::<Vec<_>>();
        let settlement_room = condition_graph
            .nodes
            .iter()
            .find(|node| node.kind == trnm_rpg_core::QuestConditionKind::ReturnForSettlement)
            .map(|node| node.subject_id.as_str())
            .unwrap_or("unknown");
        if !unfinished_waypoints.is_empty() || settlement_room != self.room.id() {
            return Err(CampaignError::InvalidState(format!(
                "{} still has authored nodes [{}] before settlement in {}",
                definition.title,
                unfinished_waypoints.join(" / "),
                settlement_room,
            )));
        }
        let required_plain = condition_graph
            .nodes
            .iter()
            .filter(|node| node.kind == trnm_rpg_core::QuestConditionKind::VisitWaypoint)
            .filter(|node| !node.optional && node.exclusive_group.is_none())
            .count();
        let required_groups = condition_graph
            .nodes
            .iter()
            .filter(|node| node.kind == trnm_rpg_core::QuestConditionKind::VisitWaypoint)
            .filter_map(|node| node.exclusive_group.as_deref())
            .collect::<BTreeSet<_>>()
            .len();
        let required_evidence = required_plain + required_groups;
        if usize::from(runtime.evidence_count) < required_evidence {
            return Err(CampaignError::InvalidState(format!(
                "{} lacks authored route evidence {}/{}",
                definition.title, runtime.evidence_count, required_evidence
            )));
        }
        let rule = quest_runtime_rule(definition.archetype);
        if runtime.approach == QuestApproach::Diplomatic {
            let trust = self
                .npc_relationships
                .get(definition.giver_npc_id)
                .map(|relationship| relationship.trust)
                .unwrap_or_default();
            if trust < rule.minimum_trust_for_diplomacy {
                return Err(CampaignError::InvalidState(format!(
                    "diplomatic resolution requires trust {} with {}",
                    rule.minimum_trust_for_diplomacy, definition.giver_npc_id
                )));
            }
        }
        if runtime.approach == QuestApproach::Resourceful {
            consume_loot(
                &mut self.progression.inventory,
                rule.resource_item_id,
                rule.resource_quantity,
            )?;
        }
        if runtime.approach == QuestApproach::Direct {
            if let Some(encounter_id) = definition.encounter_id {
                let flag = format!("{encounter_id}_cleared");
                if !self.progression.world_flags.contains(&flag) {
                    return Err(CampaignError::InvalidState(format!(
                        "{} requires winning encounter {}",
                        definition.title, encounter_id
                    )));
                }
            }
        }
        let mut completed_condition_node_ids = runtime.completed_condition_node_ids.clone();
        let branch_node_id = match runtime.approach {
            QuestApproach::Direct => definition
                .encounter_id
                .map(|_| format!("{}_encounter", definition.id)),
            QuestApproach::Diplomatic => Some(format!("{}_trust", definition.id)),
            QuestApproach::Resourceful => Some(format!("{}_resource", definition.id)),
        };
        if let Some(branch_node_id) = branch_node_id {
            if !quest_graph_node_ready(
                &condition_graph,
                &branch_node_id,
                &completed_condition_node_ids,
            ) {
                return Err(CampaignError::InvalidState(format!(
                    "{} branch condition {} is blocked by its authored prerequisites",
                    definition.title, branch_node_id
                )));
            }
            completed_condition_node_ids.insert(branch_node_id);
        }
        let settlement_node_id = format!("{}_settlement", definition.id);
        if !quest_graph_node_ready(
            &condition_graph,
            &settlement_node_id,
            &completed_condition_node_ids,
        ) {
            let incoming = condition_graph
                .edges
                .iter()
                .filter(|edge| edge.to == settlement_node_id)
                .map(|edge| edge.from.as_str())
                .collect::<Vec<_>>();
            return Err(CampaignError::InvalidState(format!(
                "{} settlement is blocked by unfinished authored graph branches; incoming={incoming:?}; completed={completed_condition_node_ids:?}",
                definition.title,
            )));
        }
        completed_condition_node_ids.insert(settlement_node_id);
        let (credit_bonus, reputation_bonus) = match runtime.approach {
            QuestApproach::Direct => (0, 0),
            QuestApproach::Diplomatic => (-definition.credit_reward / 5, 2),
            QuestApproach::Resourceful => (definition.credit_reward / 4, 1),
        };
        self.progression.credits += definition.credit_reward + credit_bonus;
        self.character.attributes.reputation = self
            .character
            .attributes
            .reputation
            .saturating_add(definition.reputation_reward + reputation_bonus);
        self.regional_quest_states
            .insert(quest_id.clone(), QuestState::Completed);
        self.progression
            .world_flags
            .insert(format!("regional_quest_{quest_id}_complete"));
        self.progression.world_flags.insert(
            format!("regional_quest_{quest_id}_{:?}", runtime.approach).to_ascii_lowercase(),
        );
        let region_id = Self::region_id_for_room_id(settlement_room);
        let (stock, demand) = self.regional_market_state(region_id, rule.resource_item_id);
        let (stock_delta, demand_delta) = match runtime.approach {
            QuestApproach::Direct => (1, -1),
            QuestApproach::Diplomatic => (2, -2),
            QuestApproach::Resourceful => (0, 3),
        };
        self.set_regional_market_state(
            region_id,
            rule.resource_item_id,
            stock.saturating_add(stock_delta),
            demand.saturating_add(demand_delta),
        );
        if let Some(text) = quest_resolution_text(&quest_id, runtime.approach) {
            self.combat_log.push(CombatLogBeat {
                kind: "quest_resolution".to_string(),
                text: text.to_string(),
            });
        }
        if let Some(giver) = NPC_CATALOG
            .iter()
            .find(|npc| npc.id == definition.giver_npc_id)
        {
            self.npc_relationships
                .entry(giver.id.to_string())
                .or_insert_with(|| NpcRelationship::new(giver.id, giver.faction_id))
                .apply(RelationshipAction::CompleteMission);
        }
        let completed = self
            .regional_quest_states
            .values()
            .filter(|state| **state == QuestState::Completed)
            .count();
        self.faction_rank = match completed {
            0..=1 => FactionRank::Outsider,
            2..=4 => FactionRank::Initiate,
            5..=9 => FactionRank::Disciple,
            _ => FactionRank::Envoy,
        };
        self.active_regional_quest_id = None;
        self.active_regional_quest_step = 0;
        self.active_regional_quest_runtime = None;
        if let Some(chapter) = MAIN_STORY_CHAPTERS.iter().find(|chapter| {
            !self
                .main_story_decisions
                .iter()
                .any(|decision| decision.chapter == chapter.chapter)
                && chapter.quest_ids.iter().all(|quest_id| {
                    self.regional_quest_states.get(*quest_id) == Some(&QuestState::Completed)
                })
        }) {
            self.pending_main_story_chapter = Some(chapter.chapter);
            self.progression
                .world_flags
                .insert(format!("main_story_scene_{}_available", chapter.scene_id));
            self.combat_log.push(CombatLogBeat {
                kind: "main_story_scene_available".to_string(),
                text: format!(
                    "{} — meet {} in {} and choose the chapter outcome.",
                    chapter.title, chapter.protagonist_id, chapter.room_id,
                ),
            });
        }
        self.main_story_chapter = MAIN_STORY_CHAPTERS
            .iter()
            .find(|chapter| {
                !self
                    .main_story_decisions
                    .iter()
                    .any(|decision| decision.chapter == chapter.chapter)
            })
            .map(|chapter| chapter.chapter)
            .unwrap_or(MainStoryChapter::ChapterComplete);
        self.revision += 1;
        Ok(())
    }

    pub fn advance_pending_main_story_scene(
        &mut self,
    ) -> Result<MainStorySceneAdvance, CampaignError> {
        self.require_town()?;
        let pending = self.pending_main_story_chapter.ok_or_else(|| {
            CampaignError::InvalidState(
                "no playable chapter scene is awaiting resolution".to_string(),
            )
        })?;
        let chapter = MAIN_STORY_CHAPTERS
            .iter()
            .find(|chapter| chapter.chapter == pending)
            .expect("pending chapter remains catalog bound");
        if self.room.id() != chapter.room_id {
            return Err(CampaignError::InvalidState(format!(
                "{} must be played in {}",
                chapter.title, chapter.room_id,
            )));
        }
        let step = self
            .main_story_scene_progress
            .get(chapter.scene_id)
            .copied()
            .unwrap_or_default();
        if step < 2 {
            let text = match (chapter.chapter, step) {
                (MainStoryChapter::MirrorCityOaths, 0) => {
                    "The square hears testimony from porters, smiths and ward captains before any oath is proposed."
                }
                (MainStoryChapter::MirrorCityOaths, _) => {
                    "Street Compass Sifu places the three rival charters side by side and asks who will bear their cost."
                }
                (MainStoryChapter::SignalRoadReckoning, 0) => {
                    "Captain Veyra opens the seized route ledger while witnesses identify the hands behind each missing convoy."
                }
                (MainStoryChapter::SignalRoadReckoning, _) => {
                    "The archive doors close; guards, merchants and scouts challenge the evidence before the road is judged."
                }
                (MainStoryChapter::AshenFringeCountermarch, 0) => {
                    "Scout Mako walks the beacon line with the player as refugees and patrols name what the countermarch destroyed."
                }
                (MainStoryChapter::AshenFringeCountermarch, _) => {
                    "At the final assembly every faction commits people and stores, forcing the last choice to carry a visible price."
                }
                (MainStoryChapter::ChapterComplete, _) => unreachable!(),
            }
            .to_string();
            let next = step + 1;
            self.main_story_scene_progress
                .insert(chapter.scene_id.to_string(), next);
            self.progression.world_flags.insert(format!(
                "main_story_scene_{}_beat_{}",
                chapter.scene_id, next
            ));
            self.combat_log.push(CombatLogBeat {
                kind: "main_story_scene_beat".to_string(),
                text: format!("{} — {text}", chapter.title),
            });
            self.revision += 1;
            return Ok(MainStorySceneAdvance::SceneBeat {
                chapter: chapter.chapter,
                step: next,
                text,
            });
        }
        self.finalize_pending_main_story_chapter()
            .map(MainStorySceneAdvance::ChapterResolved)
    }

    fn finalize_pending_main_story_chapter(
        &mut self,
    ) -> Result<MainStoryDecisionRecord, CampaignError> {
        self.require_town()?;
        let pending = self.pending_main_story_chapter.ok_or_else(|| {
            CampaignError::InvalidState(
                "no playable chapter scene is awaiting resolution".to_string(),
            )
        })?;
        let chapter = MAIN_STORY_CHAPTERS
            .iter()
            .find(|chapter| chapter.chapter == pending)
            .expect("pending chapter remains catalog bound");
        if self.room.id() != chapter.room_id {
            return Err(CampaignError::InvalidState(format!(
                "{} must be resolved in {}",
                chapter.title, chapter.room_id,
            )));
        }
        let (flag, credits, reputation, text) =
            main_story_chapter_outcome(chapter.chapter, self.main_story_choice);
        self.progression.world_flags.insert(flag.to_string());
        self.progression
            .world_flags
            .insert(format!("main_story_scene_{}_resolved", chapter.scene_id));
        self.progression.credits += credits;
        self.character.attributes.reputation = self
            .character
            .attributes
            .reputation
            .saturating_add(reputation);
        let decision = MainStoryDecisionRecord {
            chapter: chapter.chapter,
            choice: self.main_story_choice,
            outcome_flag: flag.to_string(),
            day: self.world_clock.day,
        };
        self.main_story_decisions.push(decision.clone());
        self.pending_main_story_chapter = None;
        self.combat_log.push(CombatLogBeat {
            kind: "main_story_chapter".to_string(),
            text: format!(
                "{} — {} and the player resolve the chapter: {}",
                chapter.title, chapter.protagonist_id, text
            ),
        });
        if !self.main_story_decisions.is_empty() {
            self.progression
                .world_flags
                .insert("glass_basin_wayhouse_open".to_string());
            self.story.unlocked_room_ids.extend([
                GLASS_BASIN_WAYHOUSE_ROOM.to_string(),
                DEEP_RELAY_ROOM.to_string(),
            ]);
        }
        if self.main_story_decisions.len() >= 2 {
            self.progression
                .world_flags
                .insert("ashen_fringe_open".to_string());
            self.story.unlocked_room_ids.extend([
                MOON_BRIDGE_ROOM.to_string(),
                EMBER_ORCHARD_EDGE_ROOM.to_string(),
            ]);
        }
        self.main_story_chapter = MAIN_STORY_CHAPTERS
            .iter()
            .find(|candidate| {
                !self
                    .main_story_decisions
                    .iter()
                    .any(|record| record.chapter == candidate.chapter)
            })
            .map(|chapter| chapter.chapter)
            .unwrap_or(MainStoryChapter::ChapterComplete);
        self.main_story_ending = resolve_main_story_ending(&self.main_story_decisions);
        if let Some(ending) = self.main_story_ending {
            let ending_id = ending.label().to_ascii_lowercase().replace([' ', '-'], "_");
            self.progression
                .world_flags
                .insert(format!("main_story_ending_{ending_id}"));
            self.progression
                .world_flags
                .insert(format!("post_ending_world_{ending_id}"));
            let post_state = match ending {
                MainStoryEnding::WayhouseLeague => {
                    "Wayhouses shelter travellers and produce route supplies."
                }
                MainStoryEnding::OpenArchiveRepublic => {
                    "Public ledgers expose shortages and lower information costs."
                }
                MainStoryEnding::FrontierAccord => {
                    "Frontier patrols and caravans share protected crossings."
                }
                MainStoryEnding::ThreeRoadCompact => {
                    "Three institutions remain independent under a rotating compact."
                }
                MainStoryEnding::ContestedMandate => {
                    "Rival mandates persist, opening post-story mediation work."
                }
            };
            self.post_ending_world_state = Some(post_state.to_string());
            self.combat_log.push(CombatLogBeat {
                kind: "main_story_ending_scene".to_string(),
                text: format!("ENDING SCENE — {}: {post_state}", ending.label()),
            });
        }
        self.revision += 1;
        Ok(decision)
    }

    pub fn resolve_pending_main_story_chapter(
        &mut self,
    ) -> Result<MainStoryDecisionRecord, CampaignError> {
        loop {
            match self.advance_pending_main_story_scene()? {
                MainStorySceneAdvance::SceneBeat { .. } => {}
                MainStorySceneAdvance::ChapterResolved(decision) => return Ok(decision),
            }
        }
    }

    pub fn advance_ending_epilogue(&mut self) -> Result<String, CampaignError> {
        self.require_town()?;
        let ending = self.main_story_ending.ok_or_else(|| {
            CampaignError::InvalidState("no resolved ending has an epilogue".to_string())
        })?;
        if self.ending_epilogue_complete {
            return Err(CampaignError::InvalidState(
                "the ending epilogue is already complete".to_string(),
            ));
        }
        let required_room = match ending {
            MainStoryEnding::WayhouseLeague => CARAVAN_YARD_ROOM,
            MainStoryEnding::OpenArchiveRepublic => ARCHIVE_STEPS_ROOM,
            MainStoryEnding::FrontierAccord => ASH_BEACON_FIELD_ROOM,
            MainStoryEnding::ThreeRoadCompact => MIRROR_SQUARE_ROOM,
            MainStoryEnding::ContestedMandate => MOON_BRIDGE_ROOM,
        };
        if self.room.id() != required_room {
            return Err(CampaignError::InvalidState(format!(
                "{} epilogue continues in {required_room}",
                ending.label()
            )));
        }
        let text = match (ending, self.ending_epilogue_progress) {
            (MainStoryEnding::WayhouseLeague, 0) => {
                "A winter caravan arrives without losing a traveller; the new league signs its first public shelter ledger."
            }
            (MainStoryEnding::OpenArchiveRepublic, 0) => {
                "Citizens compare the first open ledgers and catch a shortage before a ward goes hungry."
            }
            (MainStoryEnding::FrontierAccord, 0) => {
                "Former enemies relight the Ash Beacon together and exchange patrol routes under witness."
            }
            (MainStoryEnding::ThreeRoadCompact, 0) => {
                "Three delegations take separate seats in Mirror Square and agree to rotate the deciding voice."
            }
            (MainStoryEnding::ContestedMandate, 0) => {
                "Rival envoys meet at Moon Bridge; no banner lowers, but both sides accept the player as mediator."
            }
            (_, 1) => {
                "The player walks the changed district, hears who benefited and who still objects, then chooses to keep serving the living world."
            }
            _ => "The epilogue closes and the post-story world opens for continuing regional work.",
        }
        .to_string();
        self.combat_log.push(CombatLogBeat {
            kind: "main_story_epilogue".to_string(),
            text: format!("{} — {text}", ending.label()),
        });
        self.ending_epilogue_progress = self.ending_epilogue_progress.saturating_add(1);
        if self.ending_epilogue_progress >= 3 {
            self.ending_epilogue_complete = true;
            self.progression
                .world_flags
                .insert("main_story_epilogue_complete".to_string());
            self.progression.credits += 75;
            self.character.attributes.reputation =
                self.character.attributes.reputation.saturating_add(8);
        }
        self.revision += 1;
        Ok(text)
    }

    pub fn cycle_regional_quest_approach(&mut self) -> Result<QuestApproach, CampaignError> {
        self.require_town()?;
        let runtime = self.active_regional_quest_runtime.as_mut().ok_or_else(|| {
            CampaignError::InvalidState("no regional quest is active".to_string())
        })?;
        runtime.approach = runtime.approach.next();
        self.revision += 1;
        Ok(runtime.approach)
    }

    pub fn fail_active_regional_quest(&mut self, reason: &str) -> Result<(), CampaignError> {
        let quest_id = self.active_regional_quest_id.clone().ok_or_else(|| {
            CampaignError::InvalidState("no regional quest is active".to_string())
        })?;
        let definition = REGIONAL_QUEST_CATALOG
            .iter()
            .find(|definition| definition.id == quest_id)
            .expect("active regional quest remains catalog bound");
        let failures = self
            .regional_quest_failure_counts
            .entry(quest_id.clone())
            .or_default();
        *failures = failures.saturating_add(1);
        self.character.attributes.reputation = self
            .character
            .attributes
            .reputation
            .saturating_add(quest_runtime_rule(definition.archetype).failure_reputation);
        self.regional_quest_states
            .insert(quest_id.clone(), QuestState::Failed);
        self.progression
            .world_flags
            .insert(format!("regional_quest_{quest_id}_failed_{}", *failures));
        let authored_failure = quest_narrative(&quest_id)
            .map(|narrative| narrative.failure)
            .unwrap_or(reason);
        self.combat_log.push(CombatLogBeat {
            kind: "quest_failure".to_string(),
            text: format!("{} failed: {reason}. {authored_failure}", definition.title,),
        });
        self.active_regional_quest_id = None;
        self.active_regional_quest_step = 0;
        self.active_regional_quest_runtime = None;
        self.revision += 1;
        Ok(())
    }

    fn region_id_for_room_id(room_id: &str) -> &'static str {
        match room_id {
            GLASS_BASIN_WAYHOUSE_ROOM
            | DEEP_RELAY_ROOM
            | GLASS_REED_MARSH_ROOM
            | BASIN_OBSERVATORY_ROOM => "glass_basin",
            MOON_BRIDGE_ROOM
            | EMBER_ORCHARD_EDGE_ROOM
            | ASH_BEACON_FIELD_ROOM
            | CINDER_REFUGE_ROOM => "ashen_fringe",
            OUTER_SIGNAL_ROAD_ROOM | RELAY_QUARTER_ROOM => "signal_road",
            _ => "mirror_city",
        }
    }

    pub fn current_market_region_id(&self) -> Option<&'static str> {
        match self.room {
            CampaignRoom::MarketWindPavilion => Some("mirror_city"),
            CampaignRoom::RelayQuarter => Some("signal_road"),
            CampaignRoom::GlassBasinWayhouse => Some("glass_basin"),
            CampaignRoom::CinderRefuge => Some("ashen_fringe"),
            _ => None,
        }
    }

    fn require_regional_market(&self) -> Result<&'static str, CampaignError> {
        self.require_town()?;
        self.current_market_region_id().ok_or_else(|| {
            CampaignError::InvalidState(
                "regional trading requires a city pavilion, relay, wayhouse or refuge market"
                    .to_string(),
            )
        })
    }

    pub fn regional_market_state(&self, region_id: &str, item_id: &str) -> (u16, i16) {
        (
            self.regional_market_stock
                .get(region_id)
                .and_then(|stock| stock.get(item_id))
                .copied()
                .unwrap_or_default(),
            self.regional_market_demand
                .get(region_id)
                .and_then(|demand| demand.get(item_id))
                .copied()
                .unwrap_or_default(),
        )
    }

    fn set_regional_market_state(
        &mut self,
        region_id: &str,
        item_id: &str,
        stock: u16,
        demand: i16,
    ) {
        self.regional_market_stock
            .entry(region_id.to_string())
            .or_default()
            .insert(item_id.to_string(), stock.min(99));
        self.regional_market_demand
            .entry(region_id.to_string())
            .or_default()
            .insert(item_id.to_string(), demand.clamp(-20, 20));
        if region_id == "mirror_city" {
            self.market_stock.insert(item_id.to_string(), stock.min(99));
            self.market_demand
                .insert(item_id.to_string(), demand.clamp(-20, 20));
        }
    }

    fn caravan_route(from_region: &str, to_region: &str) -> Vec<String> {
        let hub = |region: &str| match region {
            "signal_road" => RELAY_QUARTER_ROOM,
            "glass_basin" => GLASS_BASIN_WAYHOUSE_ROOM,
            "ashen_fringe" => CINDER_REFUGE_ROOM,
            _ => CARAVAN_YARD_ROOM,
        };
        let mut route = vec![hub(from_region).to_string()];
        for waypoint in [
            OUTER_SIGNAL_ROAD_ROOM,
            MOON_BRIDGE_ROOM,
            ASH_BEACON_FIELD_ROOM,
        ] {
            if waypoint != route[0] && waypoint != hub(to_region) {
                route.push(waypoint.to_string());
            }
        }
        route.push(hub(to_region).to_string());
        route
    }

    pub fn visible_regional_caravan(&self) -> Option<&RegionalCaravanState> {
        self.active_regional_caravans
            .iter()
            .find(|caravan| caravan.current_room_id() == Some(self.room.id()))
    }

    pub fn interact_with_visible_caravan(
        &mut self,
        protect: bool,
    ) -> Result<String, CampaignError> {
        self.require_town()?;
        let room_id = self.room.id().to_string();
        let index = self
            .active_regional_caravans
            .iter()
            .position(|caravan| caravan.current_room_id() == Some(room_id.as_str()))
            .ok_or_else(|| {
                CampaignError::InvalidState("no caravan is currently visible here".to_string())
            })?;
        let caravan = &mut self.active_regional_caravans[index];
        let text = if protect {
            caravan.guarded_by_player = true;
            caravan.risk = caravan.risk.saturating_sub(4);
            caravan.integrity = caravan.integrity.saturating_add(15).min(100);
            caravan.incident = Some("player_escort".to_string());
            self.progression.world_flags.insert(format!(
                "caravan_{}_escorted",
                caravan.caravan_id.replace('-', "_")
            ));
            self.character.attributes.reputation =
                self.character.attributes.reputation.saturating_add(2);
            format!(
                "The player escorts {} toward {}; risk falls to {}.",
                caravan.caravan_id, caravan.to_region_id, caravan.risk
            )
        } else {
            let seized = caravan.quantity.min(1);
            caravan.quantity = caravan.quantity.saturating_sub(seized);
            caravan.integrity = caravan.integrity.saturating_sub(25);
            caravan.incident = Some("player_seizure".to_string());
            self.progression.credits += i64::from(seized) * 12;
            self.character.attributes.reputation =
                self.character.attributes.reputation.saturating_sub(3);
            self.progression.world_flags.insert(format!(
                "caravan_{}_seized",
                caravan.caravan_id.replace('-', "_")
            ));
            for bond in self.npc_bonds.values_mut() {
                *bond = bond.saturating_sub(1).clamp(-100, 100);
            }
            format!(
                "The player seizes {seized} {} from {}; regional witnesses remember it.",
                caravan.item_id, caravan.caravan_id
            )
        };
        self.combat_log.push(CombatLogBeat {
            kind: "caravan_encounter".to_string(),
            text: text.clone(),
        });
        self.revision += 1;
        Ok(text)
    }

    fn run_regional_logistics(&mut self) {
        let mut travelling = std::mem::take(&mut self.active_regional_caravans);
        for mut caravan in travelling.drain(..) {
            if caravan.route_room_ids.is_empty() {
                caravan.route_room_ids =
                    Self::caravan_route(&caravan.from_region_id, &caravan.to_region_id);
            }
            caravan.progress_legs = caravan.progress_legs.saturating_add(1);
            caravan.route_index = (caravan.route_index + 1).min(caravan.route_room_ids.len() - 1);
            if caravan.risk >= 7
                && !caravan.guarded_by_player
                && caravan.route_index + 1 < caravan.route_room_ids.len()
                && caravan.incident.is_none()
            {
                caravan.integrity = caravan.integrity.saturating_sub(35);
                caravan.quantity = caravan.quantity.saturating_sub(1);
                caravan.incident = Some("road_ambush".to_string());
                self.progression.world_flags.insert(format!(
                    "caravan_{}_ambushed",
                    caravan.caravan_id.replace('-', "_")
                ));
            }
            if caravan.route_index + 1 < caravan.route_room_ids.len() {
                self.active_regional_caravans.push(caravan);
                continue;
            }
            let delivered = caravan
                .quantity
                .saturating_sub(u16::from(caravan.integrity < 50));
            let (stock, demand) =
                self.regional_market_state(&caravan.to_region_id, &caravan.item_id);
            self.set_regional_market_state(
                &caravan.to_region_id,
                &caravan.item_id,
                stock.saturating_add(delivered),
                demand.saturating_sub(delivered as i16),
            );
            self.regional_logistics.push(RegionalMarketTransfer {
                item_id: caravan.item_id,
                from_region_id: caravan.from_region_id,
                to_region_id: caravan.to_region_id,
                quantity: delivered,
                day: self.world_clock.day,
            });
        }
        let item = &ECONOMY_ITEM_CATALOG[(self.world_clock.day as usize
            + usize::from(self.world_clock.minute_of_day / 120))
            % ECONOMY_ITEM_CATALOG.len()];
        let mut regions = MARKET_REGION_IDS
            .into_iter()
            .map(|region| {
                let (stock, demand) = self.regional_market_state(region, item.id);
                (region, stock, demand)
            })
            .collect::<Vec<_>>();
        regions.sort_by_key(|(_, stock, demand)| (i32::from(*stock) - i32::from(*demand), *stock));
        let Some(&(to_region, to_stock, _to_demand)) = regions.first() else {
            return;
        };
        let Some(&(from_region, from_stock, from_demand)) = regions.last() else {
            return;
        };
        if from_region == to_region || from_stock <= to_stock.saturating_add(1) {
            return;
        }
        self.set_regional_market_state(
            from_region,
            item.id,
            from_stock - 1,
            from_demand.saturating_add(1),
        );
        self.active_regional_caravans.push(RegionalCaravanState {
            caravan_id: format!(
                "caravan-{}-{}-{}-{}",
                self.world_clock.day, self.world_clock.minute_of_day, from_region, to_region
            ),
            item_id: item.id.to_string(),
            from_region_id: from_region.to_string(),
            to_region_id: to_region.to_string(),
            quantity: 1,
            progress_legs: 0,
            risk: ((self.world_clock.day
                + u32::from(self.world_clock.minute_of_day / 60)
                + item.id.len() as u32)
                % 10) as u8,
            route_room_ids: Self::caravan_route(from_region, to_region),
            route_index: 0,
            integrity: default_caravan_integrity(),
            guarded_by_player: false,
            incident: None,
        });
        if self.regional_logistics.len() > 64 {
            self.regional_logistics.remove(0);
        }
    }

    pub fn buy_regional_item(&mut self, item_id: &str) -> Result<(), CampaignError> {
        let region_id = self.require_regional_market()?;
        let definition = ECONOMY_ITEM_CATALOG
            .iter()
            .find(|definition| definition.id == item_id)
            .ok_or_else(|| CampaignError::InvalidState(format!("unknown shop item: {item_id}")))?;
        let (stock, demand) = self.regional_market_state(region_id, item_id);
        if stock == 0 {
            return Err(CampaignError::InvalidState(format!(
                "{} is out of stock until local production recovers",
                definition.display_name
            )));
        }
        let price = market_price_with_state(item_id, self.world_clock.day, stock, demand, true)
            .expect("catalog item has a market price");
        if self.progression.credits < price {
            return Err(CampaignError::InvalidState(format!(
                "{} costs {} credits",
                definition.display_name, price
            )));
        }
        self.progression.credits -= price;
        self.set_regional_market_state(region_id, item_id, stock - 1, demand + 2);
        if definition.material {
            merge_loot(
                &mut self.progression.inventory,
                &[LootStack {
                    item_id: item_id.to_string(),
                    quantity: 1,
                }],
            );
        } else if let Some(item) = trillionnium_inventory_item_for(
            &self.character.matrix_user_id,
            item_id,
            &format!("{region_id}_shop"),
            None,
            (self.world_clock.day * 24 * 60 + u32::from(self.world_clock.minute_of_day)) as i64,
        ) {
            let instance_id = item.item_instance_id.clone();
            self.character.inventory_items.push(item);
            if let Some(condition) = ItemCondition::new(item_id) {
                self.item_conditions.insert(instance_id, condition);
            }
        } else {
            merge_loot(
                &mut self.progression.inventory,
                &[LootStack {
                    item_id: item_id.to_string(),
                    quantity: 1,
                }],
            );
        }
        self.revision += 1;
        Ok(())
    }

    pub fn selected_shop_item(&self) -> &'static trnm_rpg_core::EconomyItemDefinition {
        &ECONOMY_ITEM_CATALOG[self.selected_shop_item_index % ECONOMY_ITEM_CATALOG.len()]
    }

    pub fn shop_selection_label(&self) -> String {
        let item = self.selected_shop_item();
        let region_id = self.current_market_region_id().unwrap_or("mirror_city");
        let (stock, demand) = self.regional_market_state(region_id, item.id);
        format!(
            "{} @ {} | buy {} / sell {} credits | stock {} demand {:+} | durability {}{} | day {}",
            item.display_name,
            region_id,
            market_price_with_state(item.id, self.world_clock.day, stock, demand, true)
                .unwrap_or(item.buy_price),
            market_price_with_state(item.id, self.world_clock.day, stock, demand, false)
                .unwrap_or(item.buy_price / 2),
            stock,
            demand,
            item.max_durability,
            if item.material { " | material" } else { "" },
            self.world_clock.day,
        )
    }

    pub fn cycle_shop_item(&mut self) -> Result<String, CampaignError> {
        self.require_regional_market()?;
        self.selected_shop_item_index =
            (self.selected_shop_item_index + 1) % ECONOMY_ITEM_CATALOG.len();
        self.revision += 1;
        Ok(self.selected_shop_item().id.to_string())
    }

    pub fn buy_selected_shop_item(&mut self) -> Result<String, CampaignError> {
        let item_id = self.selected_shop_item().id.to_string();
        self.buy_regional_item(&item_id)?;
        Ok(item_id)
    }

    pub fn sell_selected_shop_item(&mut self) -> Result<String, CampaignError> {
        let region_id = self.require_regional_market()?;
        let item = self.selected_shop_item();
        if item.material {
            consume_loot(&mut self.progression.inventory, item.id, 1)?;
        } else {
            let index = self
                .character
                .inventory_items
                .iter()
                .rposition(|owned| owned.item_id == item.id)
                .ok_or_else(|| {
                    CampaignError::InvalidState(format!("you do not own {}", item.display_name))
                })?;
            let removed = self.character.inventory_items.remove(index);
            self.item_conditions.remove(&removed.item_instance_id);
            self.character
                .equipment_slots
                .retain(|_, instance_id| instance_id != &removed.item_instance_id);
        }
        let (stock, demand) = self.regional_market_state(region_id, item.id);
        let price = market_price_with_state(item.id, self.world_clock.day, stock, demand, false)
            .expect("catalog item has a market price");
        self.progression.credits += price;
        self.set_regional_market_state(region_id, item.id, stock.saturating_add(1), demand - 2);
        self.progression.world_flags.insert(format!(
            "market_sale_{}_day_{}",
            item.id, self.world_clock.day
        ));
        self.revision += 1;
        Ok(item.id.to_string())
    }

    pub fn craft_regional_item(&mut self, recipe_id: &str) -> Result<(), CampaignError> {
        self.require_room(CampaignRoom::WorkshopGate)?;
        let recipe = CRAFTING_RECIPES
            .iter()
            .find(|recipe| recipe.id == recipe_id)
            .ok_or_else(|| CampaignError::InvalidState(format!("unknown recipe: {recipe_id}")))?;
        if !self
            .character
            .skill_ids
            .iter()
            .any(|skill| skill == recipe.required_skill_id)
        {
            return Err(CampaignError::InvalidState(format!(
                "recipe requires skill {}",
                recipe.required_skill_id
            )));
        }
        for (item_id, quantity) in recipe.ingredients {
            if !self
                .progression
                .inventory
                .iter()
                .any(|stack| stack.item_id == *item_id && stack.quantity >= *quantity)
            {
                return Err(CampaignError::InvalidState(format!(
                    "recipe is missing {quantity}x {item_id}"
                )));
            }
        }
        for (item_id, quantity) in recipe.ingredients {
            consume_loot(&mut self.progression.inventory, item_id, *quantity)?;
            let demand = self
                .market_demand
                .entry((*item_id).to_string())
                .or_default();
            *demand = demand
                .saturating_add(i16::try_from(*quantity).unwrap_or(i16::MAX))
                .min(20);
        }
        let item = trillionnium_inventory_item_for(
            &self.character.matrix_user_id,
            recipe.output_item_id,
            "iron_workshop_crafting",
            None,
            (self.world_clock.day * 24 * 60 + u32::from(self.world_clock.minute_of_day)) as i64,
        )
        .ok_or_else(|| {
            CampaignError::InvalidState(
                "crafted item is missing from the typed item catalog".to_string(),
            )
        })?;
        let instance_id = item.item_instance_id.clone();
        self.character.inventory_items.push(item);
        if let Some(condition) = ItemCondition::new(recipe.output_item_id) {
            self.item_conditions.insert(instance_id, condition);
        }
        self.progression
            .world_flags
            .insert(format!("crafted_{}", recipe.output_item_id));
        self.revision += 1;
        Ok(())
    }

    pub fn selected_recipe(&self) -> &'static trnm_rpg_core::CraftingRecipe {
        &CRAFTING_RECIPES[self.selected_recipe_index % CRAFTING_RECIPES.len()]
    }

    pub fn recipe_selection_label(&self) -> String {
        let recipe = self.selected_recipe();
        let ingredients = recipe
            .ingredients
            .iter()
            .map(|(item, count)| format!("{count}x {item}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "{} -> {} | {} | requires {}",
            recipe.id, recipe.output_item_id, ingredients, recipe.required_skill_id
        )
    }

    pub fn cycle_recipe(&mut self) -> Result<String, CampaignError> {
        self.require_room(CampaignRoom::WorkshopGate)?;
        self.selected_recipe_index = (self.selected_recipe_index + 1) % CRAFTING_RECIPES.len();
        self.revision += 1;
        Ok(self.selected_recipe().id.to_string())
    }

    pub fn craft_selected_recipe(&mut self) -> Result<String, CampaignError> {
        let recipe_id = self.selected_recipe().id.to_string();
        self.craft_regional_item(&recipe_id)?;
        Ok(recipe_id)
    }

    pub fn cycle_and_equip_owned_item(&mut self) -> Result<String, CampaignError> {
        self.require_town()?;
        if self.character.inventory_items.is_empty() {
            return Err(CampaignError::InvalidState(
                "no owned equipment is available".to_string(),
            ));
        }
        self.selected_inventory_index =
            (self.selected_inventory_index + 1) % self.character.inventory_items.len();
        let selected = self.character.inventory_items[self.selected_inventory_index].clone();
        let slot = selected.slot.clone();
        if slot.trim().is_empty() {
            return Err(CampaignError::InvalidState(format!(
                "{} is not equippable",
                selected.display_name
            )));
        }
        for item in &mut self.character.inventory_items {
            if item.equipped_slot.as_deref() == Some(slot.as_str()) {
                item.equipped_slot = None;
            }
            if item.item_instance_id == selected.item_instance_id {
                item.equipped_slot = Some(slot.clone());
            }
        }
        self.character
            .equipment_slots
            .insert(slot, selected.item_instance_id.clone());
        self.revision += 1;
        Ok(selected.display_name)
    }

    pub fn repair_all_equipment(&mut self) -> Result<i64, CampaignError> {
        self.require_room(CampaignRoom::WorkshopGate)?;
        let cost = self
            .item_conditions
            .values()
            .map(ItemCondition::repair_cost)
            .sum::<i64>();
        if cost == 0 {
            return Ok(0);
        }
        if self.progression.credits < cost {
            return Err(CampaignError::InvalidState(format!(
                "repairing equipment costs {cost} credits"
            )));
        }
        self.progression.credits -= cost;
        for condition in self.item_conditions.values_mut() {
            condition.repair();
        }
        self.revision += 1;
        Ok(cost)
    }

    pub fn equip_starter_weapon(&mut self) -> Result<(), CampaignError> {
        self.require_town()?;
        self.selected_loadout = LoadoutPreset::Guard;
        self.apply_selected_loadout()?;
        self.revision += 1;
        Ok(())
    }

    pub fn cycle_loadout(&mut self) -> Result<(), CampaignError> {
        self.require_town()?;
        if self.character.equipment_slots.contains_key("weapon") {
            self.selected_loadout = self.selected_loadout.next();
        }
        self.apply_selected_loadout()?;
        self.revision += 1;
        Ok(())
    }

    fn apply_selected_loadout(&mut self) -> Result<(), CampaignError> {
        self.character.equipment_slots.clear();
        for item in &mut self.character.inventory_items {
            item.equipped_slot = None;
        }
        for item_id in self.selected_loadout.item_ids() {
            self.character
                .equip_item_by_id(item_id, self.revision as i64 + 1)
                .ok_or_else(|| {
                    CampaignError::InvalidState(format!(
                        "loadout item {item_id} is missing from inventory"
                    ))
                })?;
        }
        Ok(())
    }

    pub fn select_party(&mut self, party_ids: Vec<String>) -> Result<(), CampaignError> {
        self.require_town()?;
        if party_ids.len() != 4 || party_ids.first().map(String::as_str) != Some("hero") {
            return Err(CampaignError::InvalidState(
                "party must contain the hero plus exactly three companions".to_string(),
            ));
        }
        let unique = party_ids
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let available = self
            .party
            .iter()
            .filter(|member| member.available)
            .map(|member| member.unit_id.as_str())
            .collect::<BTreeSet<_>>();
        if unique.len() != 4 || !unique.is_subset(&available) {
            return Err(CampaignError::InvalidState(
                "selected party contains duplicates or unavailable members".to_string(),
            ));
        }
        self.active_party_ids = party_ids;
        self.revision += 1;
        Ok(())
    }

    pub fn cycle_party_preset(&mut self) -> Result<(), CampaignError> {
        let presets = [
            ["hero", "aya", "mako", "tess"],
            ["hero", "aya", "nia", "sol"],
            ["hero", "mako", "brann", "tess"],
        ];
        let current = self
            .active_party_ids
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        let index = presets
            .iter()
            .position(|preset| preset.as_slice() == current.as_slice())
            .map(|index| (index + 1) % presets.len())
            .unwrap_or(0);
        for offset in 0..presets.len() {
            let candidate = presets[(index + offset) % presets.len()]
                .iter()
                .map(|id| (*id).to_string())
                .collect();
            if self.select_party(candidate).is_ok() {
                return Ok(());
            }
        }
        Err(CampaignError::InvalidState(
            "no complete party preset is currently available".to_string(),
        ))
    }

    pub fn cycle_party_member(&mut self, companion_slot: usize) -> Result<(), CampaignError> {
        self.require_town()?;
        if !(1..=3).contains(&companion_slot) {
            return Err(CampaignError::InvalidState(
                "companion slot must be 1, 2 or 3".to_string(),
            ));
        }
        let candidates = self
            .party
            .iter()
            .filter(|member| member.available && member.unit_id != "hero")
            .map(|member| member.unit_id.clone())
            .collect::<Vec<_>>();
        let current = &self.active_party_ids[companion_slot];
        let start = candidates
            .iter()
            .position(|candidate| candidate == current)
            .unwrap_or(0);
        for offset in 1..=candidates.len() {
            let candidate = &candidates[(start + offset) % candidates.len()];
            if !self
                .active_party_ids
                .iter()
                .any(|active| active == candidate)
            {
                self.active_party_ids[companion_slot] = candidate.clone();
                self.revision += 1;
                return Ok(());
            }
        }
        Err(CampaignError::InvalidState(
            "no unselected companion is available".to_string(),
        ))
    }

    pub fn spar_with_mentor(&mut self) -> Result<SparringReport, CampaignError> {
        self.require_room(CampaignRoom::MentorHall)?;
        if !self.trained_with_mentor {
            return Err(CampaignError::InvalidState(
                "complete one training session before sparring".to_string(),
            ));
        }
        let report = resolve_mentor_sparring(
            &self.character.attributes,
            &[
                SparringAction::Guard,
                SparringAction::InnerPower,
                SparringAction::Strike,
                SparringAction::InnerPower,
            ],
        );
        if !self
            .progression
            .world_flags
            .contains("mentor_sparring_completed")
        {
            self.progression
                .world_flags
                .insert("mentor_sparring_completed".to_string());
            self.progression.experience += 20;
            self.npc_relationships
                .get_mut("street-compass-sifu")
                .expect("mentor relationship exists")
                .apply(RelationshipAction::Spar);
            if report.outcome == SparringOutcome::Victory {
                self.faction_rank = FactionRank::Disciple;
                self.character.sect_id = Some("signal-road-school".to_string());
                self.character.title = "Signal Road Disciple".to_string();
            }
        }
        self.last_sparring = Some(report.clone());
        self.revision += 1;
        Ok(report)
    }

    pub fn talk_to_relay_smith(&mut self) -> Result<(), CampaignError> {
        self.require_room(CampaignRoom::RelayQuarter)?;
        let relation = self
            .npc_relationships
            .get_mut("relay-smith-brann")
            .expect("relay smith relationship exists");
        if relation.interactions == 0 {
            relation.apply(RelationshipAction::Talk);
            relation.apply(RelationshipAction::CompleteMission);
        } else {
            relation.apply(RelationshipAction::Talk);
        }
        if self.active_title == Some(BuildTitle::RelayRunner) {
            relation.apply(RelationshipAction::CompleteMission);
        }
        self.faction_rank = self.faction_rank.max(FactionRank::Envoy);
        self.revision += 1;
        Ok(())
    }

    pub fn recruit_relay_smith(&mut self) -> Result<(), CampaignError> {
        self.require_room(CampaignRoom::RelayQuarter)?;
        let relation = self
            .npc_relationships
            .get_mut("relay-smith-brann")
            .expect("relay smith relationship exists");
        if !relation.can_recruit(8) {
            return Err(CampaignError::InvalidState(
                "Brann requires 8 trust before recruitment".to_string(),
            ));
        }
        relation.recruited = true;
        self.party
            .iter_mut()
            .find(|member| member.unit_id == "brann")
            .expect("Brann roster entry exists")
            .available = true;
        self.progression
            .world_flags
            .insert("brann_recruited".to_string());
        self.revision += 1;
        Ok(())
    }

    pub fn heal_party(&mut self) -> Result<(), CampaignError> {
        self.require_town()?;
        if self.party.iter().all(|member| member.injury_level == 0) {
            return Err(CampaignError::InvalidState(
                "the active roster has no injuries to treat".to_string(),
            ));
        }
        let used_tonic = if let Some(stack) = self
            .progression
            .inventory
            .iter_mut()
            .find(|stack| stack.item_id == "field-tonic-kit" && stack.quantity > 0)
        {
            stack.quantity -= 1;
            true
        } else {
            false
        };
        self.progression
            .inventory
            .retain(|stack| stack.quantity > 0);
        if !used_tonic {
            let clinic_cost = if self.active_title == Some(BuildTitle::ForgeMaster) {
                FIELD_CLINIC_CREDIT_COST - 15
            } else {
                FIELD_CLINIC_CREDIT_COST
            };
            if self.progression.credits < clinic_cost {
                return Err(CampaignError::InvalidState(format!(
                    "field clinic costs {clinic_cost} credits"
                )));
            }
            self.progression.credits -= clinic_cost;
        }
        for member in &mut self.party {
            member.injury_level = member.injury_level.saturating_sub(1);
            if member.injury_level < 4 {
                member.available = true;
            }
        }
        self.revision += 1;
        Ok(())
    }

    pub fn equip_relay_core(&mut self) -> Result<(), CampaignError> {
        self.require_town()?;
        const ITEM_ID: &str = "relay-core-fragment";
        if !self
            .character
            .inventory_items
            .iter()
            .any(|item| item.item_id == ITEM_ID)
        {
            let stack = self
                .progression
                .inventory
                .iter_mut()
                .find(|stack| stack.item_id == ITEM_ID && stack.quantity > 0)
                .ok_or_else(|| {
                    CampaignError::InvalidState(
                        "secure the relay core before equipping its fragment".to_string(),
                    )
                })?;
            stack.quantity -= 1;
            let item = trillionnium_inventory_item_for(
                &self.character.matrix_user_id,
                ITEM_ID,
                "first_contact_victory_loot",
                None,
                self.revision as i64 + 1,
            )
            .ok_or_else(|| {
                CampaignError::InvalidState("relay core item catalog entry is missing".to_string())
            })?;
            self.character.inventory_items.push(item);
            self.progression
                .inventory
                .retain(|stack| stack.quantity > 0);
        }
        self.character
            .equip_item_by_id(ITEM_ID, self.revision as i64 + 1)
            .ok_or_else(|| {
                CampaignError::InvalidState("relay core could not be equipped".to_string())
            })?;
        self.revision += 1;
        Ok(())
    }

    pub fn accept_first_contact_quest(&mut self) -> Result<(), CampaignError> {
        self.require_room(CampaignRoom::ExpeditionGate)?;
        if !self.mentor_met || !self.trained_with_mentor {
            return Err(CampaignError::InvalidState(
                "mentor dialogue and training are required before deployment".to_string(),
            ));
        }
        if !self.character.equipment_slots.contains_key("weapon") {
            return Err(CampaignError::InvalidState(
                "equip a weapon before deployment".to_string(),
            ));
        }
        let first_contact_secured = self
            .progression
            .world_flags
            .contains("first_contact_secured");
        let convoy_secured = self
            .progression
            .world_flags
            .contains("convoy_exodus_secured");
        let mirror_siege_secured = self
            .progression
            .world_flags
            .contains("mirror_siege_secured");
        if self.quest_state == QuestState::Completed && first_contact_secured {
            self.active_mission = if self.progression.aftershock_completions == 0 {
                CampaignMission::AftershockPatrol
            } else if !convoy_secured {
                CampaignMission::ConvoyExodus
            } else if !mirror_siege_secured {
                CampaignMission::MirrorSiege
            } else {
                match self.active_mission {
                    CampaignMission::IronDeltaSkirmish
                    | CampaignMission::NightWatchCrossingSkirmish
                    | CampaignMission::GlassBasinSkirmish
                    | CampaignMission::EmberOrchardSkirmish
                    | CampaignMission::SaltMarshSkirmish
                    | CampaignMission::CinderCrownSkirmish => self.active_mission,
                    _ => CampaignMission::AftershockPatrol,
                }
            };
        } else if self.quest_state == QuestState::Available {
            self.active_mission = CampaignMission::FirstContact;
        } else if !matches!(self.quest_state, QuestState::Failed | QuestState::Withdrawn) {
            return Err(CampaignError::InvalidState(
                "no campaign mission is currently available".to_string(),
            ));
        }
        self.quest_state = QuestState::Accepted;
        self.revision += 1;
        Ok(())
    }

    /// Opens a fully independent skirmish lane without granting campaign
    /// completion flags. The battle still uses the current character, normal
    /// BattleSeed hashing and the same one-time RPG settlement path.
    pub fn prepare_standalone_skirmish(&mut self) -> Result<(), CampaignError> {
        self.require_town()?;
        if self.pending_battle.is_some() {
            return Err(CampaignError::InvalidState(
                "finish the pending battle before configuring a skirmish".to_string(),
            ));
        }
        self.room = CampaignRoom::ExpeditionGate;
        self.active_mission = CampaignMission::IronDeltaSkirmish;
        self.skirmish_setup.enabled = true;
        self.quest_state = QuestState::Accepted;
        self.progression
            .world_flags
            .insert("standalone_skirmish_accessed".to_string());
        self.progression
            .world_flags
            .insert("expedition_gate_open".to_string());
        self.revision += 1;
        Ok(())
    }

    pub fn cycle_endgame_mission(&mut self) -> Result<CampaignMission, CampaignError> {
        self.require_room(CampaignRoom::ExpeditionGate)?;
        if !self
            .progression
            .world_flags
            .contains("mirror_siege_secured")
        {
            return Err(CampaignError::InvalidState(
                "secure Mirror Siege before opening skirmish operations".to_string(),
            ));
        }
        self.active_mission = match self.active_mission {
            CampaignMission::AftershockPatrol => CampaignMission::IronDeltaSkirmish,
            CampaignMission::IronDeltaSkirmish => CampaignMission::NightWatchCrossingSkirmish,
            CampaignMission::NightWatchCrossingSkirmish => CampaignMission::GlassBasinSkirmish,
            CampaignMission::GlassBasinSkirmish => CampaignMission::EmberOrchardSkirmish,
            CampaignMission::EmberOrchardSkirmish => CampaignMission::SaltMarshSkirmish,
            CampaignMission::SaltMarshSkirmish => CampaignMission::CinderCrownSkirmish,
            _ => CampaignMission::AftershockPatrol,
        };
        self.skirmish_setup.enabled = matches!(
            self.active_mission,
            CampaignMission::IronDeltaSkirmish
                | CampaignMission::NightWatchCrossingSkirmish
                | CampaignMission::GlassBasinSkirmish
                | CampaignMission::EmberOrchardSkirmish
                | CampaignMission::SaltMarshSkirmish
                | CampaignMission::CinderCrownSkirmish
        );
        self.revision += 1;
        Ok(self.active_mission)
    }

    pub fn cycle_standalone_skirmish_map(&mut self) -> Result<CampaignMission, CampaignError> {
        self.require_room(CampaignRoom::ExpeditionGate)?;
        if !self.skirmish_setup.enabled
            || !self
                .progression
                .world_flags
                .contains("standalone_skirmish_accessed")
        {
            return Err(CampaignError::InvalidState(
                "standalone skirmish setup is not active".to_string(),
            ));
        }
        self.active_mission = match self.active_mission {
            CampaignMission::IronDeltaSkirmish => CampaignMission::NightWatchCrossingSkirmish,
            CampaignMission::NightWatchCrossingSkirmish => CampaignMission::GlassBasinSkirmish,
            CampaignMission::GlassBasinSkirmish => CampaignMission::EmberOrchardSkirmish,
            CampaignMission::EmberOrchardSkirmish => CampaignMission::SaltMarshSkirmish,
            CampaignMission::SaltMarshSkirmish => CampaignMission::CinderCrownSkirmish,
            _ => CampaignMission::IronDeltaSkirmish,
        };
        self.revision += 1;
        Ok(self.active_mission)
    }

    pub fn cycle_skirmish_faction(&mut self) -> Result<CampaignFaction, CampaignError> {
        self.require_room(CampaignRoom::ExpeditionGate)?;
        if !self.skirmish_setup.enabled {
            return Err(CampaignError::InvalidState(
                "select an endgame skirmish map before configuring factions".to_string(),
            ));
        }
        self.skirmish_setup.player_faction = self.skirmish_setup.player_faction.opponent();
        self.skirmish_setup.enemy_faction = self.skirmish_setup.player_faction.opponent();
        self.revision += 1;
        Ok(self.skirmish_setup.player_faction)
    }

    pub fn cycle_skirmish_resources(&mut self) -> Result<u32, CampaignError> {
        self.require_room(CampaignRoom::ExpeditionGate)?;
        if !self.skirmish_setup.enabled {
            return Err(CampaignError::InvalidState(
                "select an endgame skirmish map before configuring resources".to_string(),
            ));
        }
        self.skirmish_setup.starting_resources = match self.skirmish_setup.starting_resources {
            100..=299 => 300,
            300..=499 => 500,
            _ => 200,
        };
        self.revision += 1;
        Ok(self.skirmish_setup.starting_resources)
    }

    pub fn cycle_skirmish_victory_mode(&mut self) -> Result<SkirmishVictoryMode, CampaignError> {
        self.require_room(CampaignRoom::ExpeditionGate)?;
        if !self.skirmish_setup.enabled {
            return Err(CampaignError::InvalidState(
                "select an endgame skirmish map before configuring victory".to_string(),
            ));
        }
        self.skirmish_setup.victory_mode = self.skirmish_setup.victory_mode.next();
        self.revision += 1;
        Ok(self.skirmish_setup.victory_mode)
    }

    pub fn cycle_skirmish_simulation_seed(&mut self) -> Result<u64, CampaignError> {
        self.require_room(CampaignRoom::ExpeditionGate)?;
        if !self.skirmish_setup.enabled {
            return Err(CampaignError::InvalidState(
                "select an endgame skirmish map before configuring the seed".to_string(),
            ));
        }
        self.skirmish_setup.simulation_seed = match self.skirmish_setup.simulation_seed {
            1 => 2,
            2 => 3,
            _ => 1,
        };
        self.revision += 1;
        Ok(self.skirmish_setup.simulation_seed)
    }

    pub fn start_first_contact_battle(
        &mut self,
        map: BattleMapSeedV1,
    ) -> Result<BattleSeedV1, CampaignError> {
        self.require_room(CampaignRoom::ExpeditionGate)?;
        if self.quest_state != QuestState::Accepted {
            return Err(CampaignError::InvalidState(
                "accept the First Contact quest before deployment".to_string(),
            ));
        }
        map.validate()?;
        let expedition_readiness = self.commit_expedition_preparation()?;
        let next_revision = self.revision + 1;
        let equipment_ids = equipped_item_ids(&self.character);
        let campaign_level = self.progression.level;
        let reputation = self.character.attributes.reputation;
        let party = self
            .active_party_ids
            .iter()
            .enumerate()
            .map(|(index, unit_id)| {
                let member = self
                    .party
                    .iter()
                    .find(|member| &member.unit_id == unit_id)
                    .expect("validated active party member exists");
                let skills = if member.unit_id == "hero" {
                    self.character.skill_ids.clone()
                } else {
                    member.skill_ids.clone()
                };
                let member_equipment = if member.unit_id == "hero" {
                    equipment_ids.clone()
                } else {
                    Vec::new()
                };
                let skill_rank = skills
                    .iter()
                    .filter_map(|skill| self.progression.skill_progress.get(skill))
                    .map(|progress| progress.rank)
                    .max()
                    .unwrap_or(1);
                let attributes = if member.unit_id == "hero" {
                    &self.character.attributes
                } else {
                    &member.attributes
                };
                let unit_level = if member.unit_id == "hero" {
                    campaign_level
                } else {
                    1 + (member.experience / 120) as u32
                };
                let mut stats = map_rpg_to_rts_stats(
                    attributes,
                    skill_rank,
                    &member_equipment,
                    member.injury_level,
                );
                apply_conditional_equipment_affixes(
                    &mut stats,
                    &member_equipment,
                    self.character_origin,
                    self.build_path,
                    self.active_title,
                );
                apply_campaign_growth(&mut stats, unit_level, reputation);
                apply_expedition_readiness(&mut stats, &expedition_readiness);
                if member.unit_id == "hero" {
                    apply_regional_skills_and_sect(
                        &mut stats,
                        &skills,
                        current_sect(&self.character),
                    );
                }
                BattleUnitSeedV1 {
                    unit_id: member.unit_id.clone(),
                    display_name: member.display_name.clone(),
                    role: member.role.clone(),
                    spawn_slot: format!("party_{index}"),
                    persistent: member.persistent,
                    injury_level: member.injury_level,
                    skill_ids: skills,
                    equipment_ids: member_equipment.clone(),
                    veteran_rank: member.veteran_rank,
                    stats,
                }
            })
            .collect();
        let map_id = self.active_mission.map_id();
        let mission = MissionDefinition::for_mission(self.active_mission, &map);
        let mut seed = BattleSeedV1 {
            contract_version: BATTLE_SEED_CONTRACT.to_string(),
            battle_id: format!("{map_id}-{next_revision:08}"),
            campaign_revision: next_revision,
            map_id: map_id.to_string(),
            rules_version: FIRST_CONTACT_RULES_VERSION.to_string(),
            map,
            party,
            mission,
            difficulty: self.difficulty,
            character_origin: self.character_origin,
            build_path: self.build_path,
            active_title: self.active_title,
            sect_id: self.character.sect_id.clone(),
            regional_skill_bonus_permille: self
                .character
                .skill_ids
                .iter()
                .filter_map(|skill_id| SKILL_CATALOG.iter().find(|skill| skill.id == skill_id))
                .map(|skill| skill.rts_modifier_permille)
                .sum::<u16>(),
            field_build_cost_permille: if self.active_title == Some(BuildTitle::ForgeMaster) {
                800
            } else {
                1000
            },
            expedition_readiness,
            skirmish: if matches!(
                self.active_mission,
                CampaignMission::IronDeltaSkirmish
                    | CampaignMission::NightWatchCrossingSkirmish
                    | CampaignMission::GlassBasinSkirmish
                    | CampaignMission::EmberOrchardSkirmish
                    | CampaignMission::SaltMarshSkirmish
                    | CampaignMission::CinderCrownSkirmish
            ) {
                let mut setup = self.skirmish_setup.clone();
                setup.enabled = true;
                setup
            } else {
                SkirmishSetup::default()
            },
            seed_hash: String::new(),
        };
        seed.seed_hash = seed.computed_hash()?;
        seed.validate()?;
        self.revision = next_revision;
        self.phase = CampaignPhase::BattlePending;
        self.pending_battle = Some(PendingBattleV1 {
            seed: seed.clone(),
            result: None,
        });
        Ok(seed)
    }

    pub fn stage_battle_result(&mut self, result: BattleResultV1) -> Result<(), CampaignError> {
        if self.settled_battle_ids.contains(&result.battle_id) {
            return Ok(());
        }
        if self.phase != CampaignPhase::BattlePending {
            return Err(CampaignError::InvalidState(
                "no battle is awaiting a result".to_string(),
            ));
        }
        let pending = self.pending_battle.as_mut().ok_or_else(|| {
            CampaignError::InvalidState("pending battle payload is missing".to_string())
        })?;
        result.validate_against(&pending.seed)?;
        pending.result = Some(result);
        self.phase = CampaignPhase::PostBattlePending;
        self.revision += 1;
        Ok(())
    }

    pub fn submit_battle_result(
        &mut self,
        result: BattleResultV1,
    ) -> Result<SettlementReceiptV1, CampaignError> {
        if let Some(existing) = self.receipt_for(&result.battle_id) {
            if existing.seed_hash != result.seed_hash
                || existing.result_hash != result.computed_hash()?
            {
                return Err(CampaignError::Integrity(
                    "replayed battle id carries a different result payload".to_string(),
                ));
            }
            return Ok(SettlementReceiptV1::duplicate_from(existing, self.revision));
        }
        self.stage_battle_result(result)?;
        self.apply_pending_settlement()
    }

    pub fn apply_pending_settlement(&mut self) -> Result<SettlementReceiptV1, CampaignError> {
        if self.phase != CampaignPhase::PostBattlePending {
            return Err(CampaignError::InvalidState(
                "no staged battle result is ready for settlement".to_string(),
            ));
        }
        let (mission_id, seed, result) = {
            let pending = self.pending_battle.as_ref().ok_or_else(|| {
                CampaignError::InvalidState("pending settlement payload is missing".to_string())
            })?;
            let result = pending.result.clone().ok_or_else(|| {
                CampaignError::InvalidState("pending settlement result is missing".to_string())
            })?;
            (pending.seed.map_id.clone(), pending.seed.clone(), result)
        };
        if self.settled_battle_ids.contains(&result.battle_id) {
            let existing = self.receipt_for(&result.battle_id).ok_or_else(|| {
                CampaignError::Integrity("settled battle is missing its receipt".to_string())
            })?;
            return Ok(SettlementReceiptV1::duplicate_from(existing, self.revision));
        }
        result.validate_against(&seed)?;
        let revision_before = self.revision;
        let experience_delta = result
            .units
            .iter()
            .map(|unit| unit.experience_gained)
            .sum::<u64>();
        let previous_level = self.progression.level;
        self.progression.experience += experience_delta;
        self.progression.level = 1 + (self.progression.experience / 500) as u32;
        let levels_gained = self.progression.level.saturating_sub(previous_level) as u16;
        self.progression.growth_points_available = self
            .progression
            .growth_points_available
            .saturating_add(levels_gained);
        self.progression.growth_points_awarded = self
            .progression
            .growth_points_awarded
            .saturating_add(levels_gained);
        self.character.attributes.reputation = self
            .character
            .attributes
            .reputation
            .saturating_add(result.reputation_delta);
        let credit_delta = if result.outcome == BattleOutcome::Victory {
            result.resource_delta.max(0)
        } else {
            0
        };
        self.progression.credits = self.progression.credits.saturating_add(credit_delta);
        merge_loot(&mut self.progression.inventory, &result.loot);
        self.progression
            .world_flags
            .extend(result.world_flags.iter().cloned());
        self.world_clock.advance(
            result
                .elapsed_ticks
                .div_ceil(600)
                .max(1)
                .min(u64::from(u32::MAX)) as u32,
        );
        if result.outcome == BattleOutcome::Victory {
            self.expedition_supplies.stamina =
                self.expedition_supplies.stamina.saturating_add(25).min(100);
            self.expedition_supplies.rations =
                self.expedition_supplies.rations.saturating_add(1).min(12);
            self.expedition_supplies.water =
                self.expedition_supplies.water.saturating_add(2).min(16);
            self.npc_relationships
                .entry("street-compass-sifu".to_string())
                .or_insert_with(|| {
                    NpcRelationship::new("street-compass-sifu", "signal-road-school")
                })
                .apply(RelationshipAction::CompleteMission);
        }

        let mut injury_delta_by_unit = BTreeMap::new();
        for report in &result.units {
            let delta = match report.status {
                UnitBattleStatus::Healthy => 0,
                UnitBattleStatus::Wounded => 1,
                UnitBattleStatus::Incapacitated | UnitBattleStatus::Lost => 2,
            };
            if delta > 0 {
                injury_delta_by_unit.insert(report.unit_id.clone(), delta);
            }
            if let Some(member) = self
                .party
                .iter_mut()
                .find(|member| member.unit_id == report.unit_id)
            {
                member.experience = member.experience.saturating_add(report.experience_gained);
                member.veteran_rank = member.veteran_rank.max(report.veteran_rank);
                member.confirmed_kills = member
                    .confirmed_kills
                    .saturating_add(report.confirmed_kills);
                member.injury_level = member.injury_level.saturating_add(delta).min(4);
                if report.status == UnitBattleStatus::Lost && !member.persistent {
                    member.available = false;
                }
            }
        }
        if result.outcome == BattleOutcome::Defeat && injury_delta_by_unit.is_empty() {
            // Losing an objective without a recorded combat wound still has a
            // persistent expedition cost. This prevents a player from farming
            // consequence-free defeats by holding safely while the objective
            // collapses.
            if let Some(member) = self
                .party
                .iter_mut()
                .find(|member| member.persistent && member.available)
            {
                member.injury_level = member.injury_level.saturating_add(1).min(4);
                injury_delta_by_unit.insert(member.unit_id.clone(), 1);
            }
        }
        for skill_id in &self.character.skill_ids {
            let skill = self
                .progression
                .skill_progress
                .entry(skill_id.clone())
                .or_insert(SkillProgress {
                    rank: 1,
                    experience: 0,
                });
            skill.experience += experience_delta / self.character.skill_ids.len().max(1) as u64;
            skill.rank = (1 + skill.experience / 250) as u16;
        }
        if result.outcome != BattleOutcome::Withdrawal {
            let wear = if result.outcome == BattleOutcome::Victory {
                6
            } else {
                12
            };
            for instance_id in self.character.equipment_slots.values() {
                if let Some(condition) = self.item_conditions.get_mut(instance_id) {
                    condition.apply_wear(wear);
                }
            }
        }
        self.quest_state = match result.outcome {
            BattleOutcome::Victory => QuestState::Completed,
            BattleOutcome::Defeat => QuestState::Failed,
            BattleOutcome::Withdrawal => QuestState::Withdrawn,
        };
        if result.outcome == BattleOutcome::Victory
            && matches!(
                mission_id.as_str(),
                "aftershock_patrol" | "first_contact_aftershock"
            )
        {
            self.progression.aftershock_completions =
                self.progression.aftershock_completions.saturating_add(1);
        }
        if result.outcome == BattleOutcome::Victory && mission_id == "first_contact" {
            self.complete_story_step(
                StoryStepId::SecureFirstContact,
                StoryStepId::BreakAftershock,
            )?;
        } else if result.outcome == BattleOutcome::Victory
            && matches!(
                mission_id.as_str(),
                "aftershock_patrol" | "first_contact_aftershock"
            )
        {
            self.complete_story_step(StoryStepId::BreakAftershock, StoryStepId::EvacuateConvoy)?;
        } else if result.outcome == BattleOutcome::Victory && mission_id == "convoy_exodus" {
            self.complete_story_step(StoryStepId::EvacuateConvoy, StoryStepId::SignalRoadComplete)?;
        }
        self.phase = CampaignPhase::Town;
        self.room = CampaignRoom::MirrorSquare;
        self.revision += 1;
        let receipt = SettlementReceiptV1 {
            contract_version: SETTLEMENT_RECEIPT_CONTRACT.to_string(),
            battle_id: result.battle_id.clone(),
            seed_hash: result.seed_hash.clone(),
            result_hash: result.computed_hash()?,
            campaign_revision_before: revision_before,
            campaign_revision_after: self.revision,
            outcome: result.outcome,
            experience_delta,
            reputation_delta: result.reputation_delta,
            credit_delta,
            loot_delta: result.loot.clone(),
            injury_delta_by_unit,
            economic_intent_id: (credit_delta > 0)
                .then(|| format!("battle-reward:{}", result.battle_id)),
            economic_receipt_id: None,
            duplicate: false,
        };
        self.settled_battle_ids.insert(result.battle_id.clone());
        self.settlement_receipts.push(receipt.clone());
        self.pending_battle = None;
        self.queue_battle_reward_economy(&receipt)?;
        if self.economy_mode == EconomyMode::OfflineLocal {
            self.reconcile_economy(&OfflineLocalEconomyBackend, 8)?;
        }
        self.validate()?;
        Ok(self
            .receipt_for(&receipt.battle_id)
            .cloned()
            .unwrap_or(receipt))
    }

    pub fn receipt_for(&self, battle_id: &str) -> Option<&SettlementReceiptV1> {
        self.settlement_receipts
            .iter()
            .find(|receipt| receipt.battle_id == battle_id)
    }

    fn require_town(&self) -> Result<(), CampaignError> {
        if self.phase == CampaignPhase::Town {
            Ok(())
        } else {
            Err(CampaignError::InvalidState(
                "town action is unavailable during battle handoff".to_string(),
            ))
        }
    }

    fn complete_story_step(
        &mut self,
        step_id: StoryStepId,
        next_step: StoryStepId,
    ) -> Result<(), CampaignError> {
        let definition = signal_road_quest_definition();
        let step = definition
            .steps
            .into_iter()
            .find(|step| step.id == step_id)
            .ok_or_else(|| {
                CampaignError::InvalidState(format!("missing story step definition: {step_id:?}"))
            })?;
        let conditions_met = step.conditions.iter().all(|condition| match condition {
            UnlockCondition::MentorMet => self.mentor_met,
            UnlockCondition::WorldFlag { flag } => {
                self.progression.world_flags.contains(flag.as_str())
            }
            UnlockCondition::MissionVictories { mission, count } => match mission {
                CampaignMission::FirstContact => self
                    .progression
                    .world_flags
                    .contains("first_contact_secured"),
                CampaignMission::AftershockPatrol => {
                    self.progression.aftershock_completions >= *count
                }
                CampaignMission::ConvoyExodus => self
                    .progression
                    .world_flags
                    .contains("convoy_exodus_secured"),
                CampaignMission::MirrorSiege => self
                    .progression
                    .world_flags
                    .contains("mirror_siege_secured"),
                CampaignMission::IronDeltaSkirmish => {
                    self.progression.world_flags.contains("iron_delta_won")
                }
                CampaignMission::NightWatchCrossingSkirmish => self
                    .progression
                    .world_flags
                    .contains("night_watch_crossing_won"),
                CampaignMission::GlassBasinSkirmish => {
                    self.progression.world_flags.contains("glass_basin_won")
                }
                CampaignMission::EmberOrchardSkirmish => {
                    self.progression.world_flags.contains("ember_orchard_won")
                }
                CampaignMission::SaltMarshSkirmish => {
                    self.progression.world_flags.contains("salt_marsh_won")
                }
                CampaignMission::CinderCrownSkirmish => {
                    self.progression.world_flags.contains("cinder_crown_won")
                }
            },
        });
        if !conditions_met {
            return Err(CampaignError::InvalidState(format!(
                "story step conditions are not met: {step_id:?}"
            )));
        }
        for reward in step.rewards {
            match reward {
                QuestReward::WorldFlag { flag } => {
                    self.progression.world_flags.insert(flag);
                }
                QuestReward::UnlockRoom { room_id } => {
                    self.story.unlocked_room_ids.insert(room_id);
                }
            }
        }
        self.story.completed_steps.insert(step_id);
        self.story.current_step = next_step;
        Ok(())
    }

    fn require_room(&self, room: CampaignRoom) -> Result<(), CampaignError> {
        self.require_town()?;
        if self.room == room {
            Ok(())
        } else {
            Err(CampaignError::InvalidState(format!(
                "action requires {}, current room is {}",
                room.title(),
                self.room.title()
            )))
        }
    }
}

pub fn typed_equipment_modifier(item_id: &str) -> TypedEquipmentModifier {
    let mut modifier = TypedEquipmentModifier {
        item_id: item_id.to_string(),
        max_hp: 0,
        damage: 0,
        armor: 0,
        move_speed_milli: 0,
        attack_interval_ticks: 0,
        evasion_permille: 0,
        energy: 0,
        ability_range: 0,
    };
    match item_id {
        "route-guard-staff" => {
            modifier.damage = 4;
            modifier.armor = 1;
            modifier.ability_range = 1;
        }
        "street-compass-bracer" => {
            modifier.move_speed_milli = 80;
            modifier.evasion_permille = 25;
        }
        "night-watch-cloak" => {
            modifier.move_speed_milli = 120;
            modifier.evasion_permille = 45;
        }
        "iron-workshop-blade" | "market-wind-sword" => modifier.damage = 7,
        "raid-signal-drum" => {
            modifier.energy = 20;
            modifier.ability_range = 2;
        }
        "field-tonic-kit" => modifier.max_hp = 20,
        "relay-core-fragment" => {
            modifier.armor = 2;
            modifier.energy = 25;
            modifier.ability_range = 2;
        }
        "evidence-wrap-case" => {
            modifier.energy = 10;
            modifier.ability_range = 1;
        }
        "reinforced-staff" => {
            modifier.damage = 10;
            modifier.armor = 2;
            modifier.ability_range = 2;
        }
        "signal-lamellar" => {
            modifier.max_hp = 30;
            modifier.armor = 5;
        }
        "watcher-boots" => {
            modifier.move_speed_milli = 180;
            modifier.evasion_permille = 60;
        }
        "field-medic-satchel" => {
            modifier.max_hp = 15;
            modifier.energy = 35;
            modifier.ability_range = 1;
        }
        "compass-thread-coat" => {
            modifier.armor = 2;
            modifier.move_speed_milli = 140;
            modifier.evasion_permille = 50;
        }
        "emberglass-lens" => {
            modifier.energy = 30;
            modifier.ability_range = 3;
        }
        "cistern-seal-kit" => {
            modifier.max_hp = 25;
            modifier.armor = 3;
        }
        "ashward-tonic" => {
            modifier.max_hp = 18;
            modifier.evasion_permille = 20;
        }
        _ => {}
    }
    modifier
}

fn apply_conditional_equipment_affixes(
    stats: &mut RtsUnitStats,
    equipment_ids: &[String],
    origin: CharacterOrigin,
    build_path: BuildPath,
    title: Option<BuildTitle>,
) {
    for item_id in equipment_ids {
        let (condition, hp, damage, armor, speed, evasion, energy, range) = match item_id.as_str() {
            "route-guard-staff" => (
                EquipmentAffixCondition::Origin(CharacterOrigin::Balanced),
                18,
                0,
                2,
                0,
                0,
                0,
                0,
            ),
            "night-watch-cloak" => (
                EquipmentAffixCondition::BuildPath(BuildPath::Windrunner),
                0,
                0,
                0,
                90,
                35,
                0,
                0,
            ),
            "raid-signal-drum" => (
                EquipmentAffixCondition::Origin(CharacterOrigin::Artisan),
                0,
                0,
                0,
                0,
                0,
                30,
                1,
            ),
            "relay-core-fragment" => (
                EquipmentAffixCondition::MasteryTitle(BuildTitle::ForgeMaster),
                0,
                3,
                2,
                0,
                0,
                0,
                0,
            ),
            _ => continue,
        };
        if condition.active(origin, build_path, title) {
            stats.max_hp = stats.max_hp.saturating_add(hp);
            stats.damage = stats.damage.saturating_add(damage);
            stats.armor = stats.armor.saturating_add(armor);
            stats.move_speed_milli = stats.move_speed_milli.saturating_add(speed);
            stats.evasion_permille = stats.evasion_permille.saturating_add(evasion).min(500);
            stats.energy = stats.energy.saturating_add(energy);
            stats.ability_range = stats.ability_range.saturating_add(range);
        }
    }
}

fn remove_origin_bonus(origin: CharacterOrigin, attributes: &mut TrillionniumAttributes) {
    match origin {
        CharacterOrigin::Balanced => {
            attributes.physique = attributes.physique.saturating_sub(2);
            attributes.resolve = attributes.resolve.saturating_sub(2);
        }
        CharacterOrigin::Artisan => {
            attributes.craft = attributes.craft.saturating_sub(4);
            attributes.insight = attributes.insight.saturating_sub(1);
        }
        CharacterOrigin::Scout => {
            attributes.agility = attributes.agility.saturating_sub(4);
            attributes.insight = attributes.insight.saturating_sub(1);
        }
    }
}

pub fn map_rpg_to_rts_stats(
    attributes: &TrillionniumAttributes,
    skill_rank: u16,
    equipment_ids: &[String],
    injury_level: u8,
) -> RtsUnitStats {
    let derived = attributes.derived_stats();
    let mut stats = RtsUnitStats {
        max_hp: derived.max_hp as u32,
        damage: 8 + attributes.force as u32 * 2,
        armor: 2 + attributes.physique as u32 / 4 + attributes.resolve as u32 / 5,
        move_speed_milli: 850 + attributes.agility as u32 * 22,
        attack_interval_ticks: (22_i32 - attributes.agility as i32 / 2).max(8) as u32,
        evasion_permille: (attributes.agility * 8).min(300),
        energy: derived.inner_energy as u32,
        ability_range: 2 + attributes.insight as u32 / 5,
        skill_power_permille: (1000 + skill_rank as u32 * 80).min(1800) as u16,
    };
    for item_id in equipment_ids {
        let modifier = typed_equipment_modifier(item_id);
        stats.max_hp = add_signed(stats.max_hp, modifier.max_hp, 1);
        stats.damage = add_signed(stats.damage, modifier.damage, 1);
        stats.armor = add_signed(stats.armor, modifier.armor, 0);
        stats.move_speed_milli = add_signed(stats.move_speed_milli, modifier.move_speed_milli, 100);
        stats.attack_interval_ticks = add_signed(
            stats.attack_interval_ticks,
            modifier.attack_interval_ticks,
            4,
        );
        stats.evasion_permille =
            add_signed(stats.evasion_permille as u32, modifier.evasion_permille, 0).min(500) as u16;
        stats.energy = add_signed(stats.energy, modifier.energy, 0);
        stats.ability_range = add_signed(stats.ability_range, modifier.ability_range, 1);
    }
    if injury_level > 0 {
        let penalty = 100_u32.saturating_sub(injury_level as u32 * 12).max(52);
        stats.max_hp = (stats.max_hp * penalty / 100).max(1);
        stats.move_speed_milli = (stats.move_speed_milli * penalty / 100).max(100);
    }
    stats
}

fn apply_campaign_growth(stats: &mut RtsUnitStats, level: u32, reputation: i32) {
    let growth = level.saturating_sub(1).min(12);
    let morale = reputation.clamp(0, 40) as u32;
    stats.max_hp = stats
        .max_hp
        .saturating_add(growth.saturating_mul(14))
        .saturating_add(morale / 2);
    stats.damage = stats.damage.saturating_add(growth.saturating_mul(2));
    stats.armor = stats.armor.saturating_add(growth / 2);
    stats.energy = stats
        .energy
        .saturating_add(growth.saturating_mul(4))
        .saturating_add(morale / 2);
}

fn apply_expedition_readiness(stats: &mut RtsUnitStats, readiness: &ExpeditionReadiness) {
    let stamina_permille = 700_u32.saturating_add(u32::from(readiness.stamina) * 3);
    stats.max_hp = (stats.max_hp.saturating_mul(stamina_permille) / 1000).max(1);
    stats.move_speed_milli =
        (stats.move_speed_milli.saturating_mul(stamina_permille) / 1000).max(100);
    match readiness.preparation {
        ExpeditionPreparation::Supplied => {
            stats.energy = stats.energy.saturating_add(30);
        }
        ExpeditionPreparation::Shortcut => {
            stats.move_speed_milli = stats.move_speed_milli.saturating_add(120);
            stats.evasion_permille = stats.evasion_permille.saturating_add(25).min(500);
        }
        ExpeditionPreparation::Immediate | ExpeditionPreparation::Rested => {}
    }
}

fn apply_regional_skills_and_sect(
    stats: &mut RtsUnitStats,
    skill_ids: &[String],
    sect: Option<SectId>,
) {
    for skill in skill_ids
        .iter()
        .filter_map(|skill_id| SKILL_CATALOG.iter().find(|skill| skill.id == skill_id))
    {
        match skill.effect {
            trnm_rpg_core::SkillEffect::Damage => {
                stats.damage = stats
                    .damage
                    .saturating_add(u32::from(skill.rts_modifier_permille) / 25)
            }
            trnm_rpg_core::SkillEffect::Guard => {
                stats.armor = stats
                    .armor
                    .saturating_add(u32::from(skill.rts_modifier_permille) / 35)
            }
            trnm_rpg_core::SkillEffect::Mobility => {
                stats.move_speed_milli = stats
                    .move_speed_milli
                    .saturating_add(u32::from(skill.rts_modifier_permille))
            }
            trnm_rpg_core::SkillEffect::Recon => {
                stats.ability_range = stats.ability_range.saturating_add(1)
            }
            trnm_rpg_core::SkillEffect::Healing => {
                stats.energy = stats
                    .energy
                    .saturating_add(u32::from(skill.rts_modifier_permille) / 2)
            }
            trnm_rpg_core::SkillEffect::Construction => {
                stats.max_hp = stats
                    .max_hp
                    .saturating_add(u32::from(skill.rts_modifier_permille) / 3)
            }
            trnm_rpg_core::SkillEffect::Economy => {
                stats.skill_power_permille = stats
                    .skill_power_permille
                    .saturating_add(skill.rts_modifier_permille / 2)
            }
            trnm_rpg_core::SkillEffect::Diplomacy => {
                stats.energy = stats
                    .energy
                    .saturating_add(u32::from(skill.rts_modifier_permille) / 4)
            }
        }
    }
    match sect {
        Some(SectId::StreetCompass) => {
            stats.move_speed_milli = stats.move_speed_milli.saturating_add(100);
            stats.ability_range = stats.ability_range.saturating_add(1);
        }
        Some(SectId::IronWorkshop) => {
            stats.max_hp = stats.max_hp.saturating_add(24);
            stats.armor = stats.armor.saturating_add(3);
        }
        Some(SectId::NightWatch) => {
            stats.damage = stats.damage.saturating_add(3);
            stats.evasion_permille = stats.evasion_permille.saturating_add(50).min(500);
        }
        None => {}
    }
}

fn require_supplies(
    supplies: &ExpeditionSupplyState,
    rations: u8,
    water: u8,
) -> Result<(), CampaignError> {
    if supplies.rations < rations || supplies.water < water {
        Err(CampaignError::InvalidState(format!(
            "expedition requires {rations} ration(s) and {water} water"
        )))
    } else {
        Ok(())
    }
}

fn add_signed(value: u32, delta: i32, minimum: u32) -> u32 {
    (value as i64 + delta as i64).max(minimum as i64) as u32
}

fn equipped_item_ids(character: &WorldTrillionniumCharacter) -> Vec<String> {
    let equipped_instances = character
        .equipment_slots
        .values()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let mut ids = character
        .inventory_items
        .iter()
        .filter(|item| equipped_instances.contains(item.item_instance_id.as_str()))
        .map(|item| item.item_id.clone())
        .collect::<Vec<_>>();
    ids.sort();
    ids
}

fn character_item_conditions(
    character: &WorldTrillionniumCharacter,
) -> BTreeMap<String, ItemCondition> {
    character
        .inventory_items
        .iter()
        .filter_map(|item| {
            ItemCondition::new(item.item_id.clone())
                .filter(|condition| condition.max_durability > 0)
                .map(|condition| (item.item_instance_id.clone(), condition))
        })
        .collect()
}

fn current_sect(character: &WorldTrillionniumCharacter) -> Option<SectId> {
    match character.sect_id.as_deref()? {
        "signal-road-school" | "street_compass_society" => Some(SectId::StreetCompass),
        "iron_workshop_gate" => Some(SectId::IronWorkshop),
        "night_watch_alliance" => Some(SectId::NightWatch),
        _ => None,
    }
}

fn merge_loot(inventory: &mut Vec<LootStack>, loot: &[LootStack]) {
    for incoming in loot {
        if let Some(existing) = inventory
            .iter_mut()
            .find(|existing| existing.item_id == incoming.item_id)
        {
            existing.quantity = existing.quantity.saturating_add(incoming.quantity);
        } else {
            inventory.push(incoming.clone());
        }
    }
    inventory.sort_by(|left, right| left.item_id.cmp(&right.item_id));
}

fn consume_loot(
    inventory: &mut Vec<LootStack>,
    item_id: &str,
    quantity: u16,
) -> Result<(), CampaignError> {
    let stack = inventory
        .iter_mut()
        .find(|stack| stack.item_id == item_id && stack.quantity >= quantity)
        .ok_or_else(|| CampaignError::InvalidState(format!("missing loot item {item_id}")))?;
    stack.quantity -= quantity;
    inventory.retain(|stack| stack.quantity > 0);
    Ok(())
}

fn canonical_json_hash<T: Serialize>(value: &T) -> Result<String, CampaignError> {
    let bytes = serde_json::to_vec(value)?;
    let digest = Sha256::digest(bytes);
    Ok(digest.iter().map(|byte| format!("{byte:02x}")).collect())
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SaveSlotId {
    #[default]
    A,
    B,
    C,
}

impl SaveSlotId {
    pub const ALL: [Self; 3] = [Self::A, Self::B, Self::C];

    pub fn label(self) -> &'static str {
        match self {
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
        }
    }

    pub fn from_digit(digit: u8) -> Option<Self> {
        match digit {
            1 => Some(Self::A),
            2 => Some(Self::B),
            3 => Some(Self::C),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaveSlotMeta {
    pub slot: SaveSlotId,
    pub exists: bool,
    pub valid: bool,
    pub campaign_id: Option<String>,
    pub revision: Option<u64>,
    pub phase: Option<CampaignPhase>,
    pub mission: Option<CampaignMission>,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SaveSlotStore {
    root: PathBuf,
}

impl SaveSlotStore {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn path(&self, slot: SaveSlotId) -> PathBuf {
        match slot {
            SaveSlotId::A => self.root.join("campaign.json"),
            SaveSlotId::B => self.root.join("campaign-b.json"),
            SaveSlotId::C => self.root.join("campaign-c.json"),
        }
    }

    pub fn checkpoint_path(&self, slot: SaveSlotId) -> PathBuf {
        match slot {
            SaveSlotId::A => self.root.join("first-contact-battle.json"),
            SaveSlotId::B => self.root.join("campaign-b-battle.json"),
            SaveSlotId::C => self.root.join("campaign-c-battle.json"),
        }
    }

    pub fn load(&self, slot: SaveSlotId) -> Result<CampaignSaveV1, CampaignError> {
        CampaignStore::new(self.path(slot)).load()
    }

    pub fn load_or_default(&self, slot: SaveSlotId) -> Result<CampaignSaveV1, CampaignError> {
        CampaignStore::new(self.path(slot)).load_or_default()
    }

    pub fn save_atomic(
        &self,
        slot: SaveSlotId,
        save: &CampaignSaveV1,
    ) -> Result<(), CampaignError> {
        CampaignStore::new(self.path(slot)).save_atomic(save)
    }

    pub fn create_new(
        &self,
        slot: SaveSlotId,
        overwrite: bool,
    ) -> Result<CampaignSaveV1, CampaignError> {
        let path = self.path(slot);
        if path.exists() && !overwrite {
            return Err(CampaignError::InvalidState(format!(
                "slot {} requires explicit overwrite confirmation",
                slot.label()
            )));
        }
        let mut save = CampaignSaveV1 {
            campaign_id: format!("local-campaign-slot-{}", slot.label().to_ascii_lowercase()),
            ..CampaignSaveV1::default()
        };
        save.character_identity.confirmed = false;
        save.apply_character_identity_name();
        self.save_atomic(slot, &save)?;
        let checkpoint = self.checkpoint_path(slot);
        if checkpoint.exists() {
            fs::remove_file(checkpoint)?;
        }
        Ok(save)
    }

    pub fn metadata(&self, slot: SaveSlotId) -> SaveSlotMeta {
        let path = self.path(slot);
        if !path.exists() {
            return SaveSlotMeta {
                slot,
                exists: false,
                valid: false,
                campaign_id: None,
                revision: None,
                phase: None,
                mission: None,
                error: None,
            };
        }
        match self.load(slot) {
            Ok(save) => SaveSlotMeta {
                slot,
                exists: true,
                valid: true,
                campaign_id: Some(save.campaign_id),
                revision: Some(save.revision),
                phase: Some(save.phase),
                mission: Some(save.active_mission),
                error: None,
            },
            Err(error) => SaveSlotMeta {
                slot,
                exists: true,
                valid: false,
                campaign_id: None,
                revision: None,
                phase: None,
                mission: None,
                error: Some(error.to_string()),
            },
        }
    }

    pub fn list(&self) -> Vec<SaveSlotMeta> {
        SaveSlotId::ALL
            .into_iter()
            .map(|slot| self.metadata(slot))
            .collect()
    }
}

pub const PLAYER_SETTINGS_CONTRACT: &str = "trnm_player_settings_v2";

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputMode {
    #[default]
    Hybrid,
    KeyboardOnly,
    MouseOnly,
}

impl InputMode {
    pub fn next(self) -> Self {
        match self {
            Self::Hybrid => Self::KeyboardOnly,
            Self::KeyboardOnly => Self::MouseOnly,
            Self::MouseOnly => Self::Hybrid,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlScheme {
    #[default]
    Classic,
    LeftHanded,
    ArrowGrid,
}

impl ControlScheme {
    pub fn next(self) -> Self {
        match self {
            Self::Classic => Self::LeftHanded,
            Self::LeftHanded => Self::ArrowGrid,
            Self::ArrowGrid => Self::Classic,
        }
    }
}

fn default_true() -> bool {
    true
}
fn default_volume() -> u8 {
    80
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerSettings {
    pub contract_version: String,
    #[serde(default)]
    pub low_motion: bool,
    #[serde(default)]
    pub input_mode: InputMode,
    #[serde(default = "default_true")]
    pub subtitles: bool,
    #[serde(default)]
    pub high_contrast: bool,
    #[serde(default)]
    pub control_scheme: ControlScheme,
    #[serde(default = "default_volume")]
    pub master_volume_percent: u8,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            contract_version: PLAYER_SETTINGS_CONTRACT.to_string(),
            low_motion: false,
            input_mode: InputMode::Hybrid,
            subtitles: true,
            high_contrast: false,
            control_scheme: ControlScheme::Classic,
            master_volume_percent: 80,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerSettingsStore {
    path: PathBuf,
}

impl PlayerSettingsStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn load_or_default(&self) -> Result<PlayerSettings, CampaignError> {
        if !self.path.exists() {
            return Ok(PlayerSettings::default());
        }
        let bytes = fs::read(&self.path)?;
        let mut value: serde_json::Value = serde_json::from_slice(&bytes)?;
        if value
            .get("contract_version")
            .and_then(serde_json::Value::as_str)
            == Some("trnm_player_settings_v1")
        {
            value["contract_version"] =
                serde_json::Value::String(PLAYER_SETTINGS_CONTRACT.to_string());
        }
        let settings: PlayerSettings = serde_json::from_value(value)?;
        if settings.contract_version != PLAYER_SETTINGS_CONTRACT
            || settings.master_volume_percent > 100
        {
            return Err(CampaignError::InvalidContract(settings.contract_version));
        }
        Ok(settings)
    }

    pub fn save_atomic(&self, settings: &PlayerSettings) -> Result<(), CampaignError> {
        if settings.contract_version != PLAYER_SETTINGS_CONTRACT
            || settings.master_volume_percent > 100
        {
            return Err(CampaignError::InvalidContract(
                settings.contract_version.clone(),
            ));
        }
        atomic_write_json(&self.path, settings)
    }
}

fn atomic_write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), CampaignError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let temp_path = path.with_extension("json.tmp");
    let payload = serde_json::to_vec_pretty(value)?;
    let mut file = fs::File::create(&temp_path)?;
    file.write_all(&payload)?;
    file.sync_all()?;
    fs::rename(&temp_path, path)?;
    if let Some(parent) = path.parent() {
        fs::File::open(parent)?.sync_all()?;
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct CampaignStore {
    path: PathBuf,
}

impl CampaignStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn load(&self) -> Result<CampaignSaveV1, CampaignError> {
        let bytes = fs::read(&self.path)?;
        let mut save: CampaignSaveV1 = serde_json::from_slice(&bytes)?;
        save.ensure_gameplay_defaults();
        save.validate()?;
        Ok(save)
    }

    pub fn load_or_default(&self) -> Result<CampaignSaveV1, CampaignError> {
        match self.load() {
            Ok(save) => Ok(save),
            Err(CampaignError::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {
                Ok(CampaignSaveV1::default())
            }
            Err(error) => Err(error),
        }
    }

    pub fn save_atomic(&self, save: &CampaignSaveV1) -> Result<(), CampaignError> {
        save.validate()?;
        atomic_write_json(&self.path, save)
    }

    pub fn stage_result_atomic(
        &self,
        save: &mut CampaignSaveV1,
        result: BattleResultV1,
    ) -> Result<(), CampaignError> {
        let mut candidate = save.clone();
        candidate.stage_battle_result(result)?;
        self.save_atomic(&candidate)?;
        *save = candidate;
        Ok(())
    }

    pub fn settle_atomic(
        &self,
        save: &mut CampaignSaveV1,
    ) -> Result<SettlementReceiptV1, CampaignError> {
        let mut candidate = save.clone();
        let receipt = candidate.apply_pending_settlement()?;
        self.save_atomic(&candidate)?;
        *save = candidate;
        Ok(receipt)
    }

    pub fn recover_pending_settlement(
        &self,
        save: &mut CampaignSaveV1,
    ) -> Result<Option<SettlementReceiptV1>, CampaignError> {
        if save.phase == CampaignPhase::PostBattlePending {
            self.settle_atomic(save).map(Some)
        } else {
            Ok(None)
        }
    }

    pub fn submit_result_atomic(
        &self,
        save: &mut CampaignSaveV1,
        result: BattleResultV1,
    ) -> Result<SettlementReceiptV1, CampaignError> {
        if let Some(existing) = save.receipt_for(&result.battle_id) {
            if existing.seed_hash != result.seed_hash
                || existing.result_hash != result.computed_hash()?
            {
                return Err(CampaignError::Integrity(
                    "replayed battle id carries a different result payload".to_string(),
                ));
            }
            return Ok(SettlementReceiptV1::duplicate_from(existing, save.revision));
        }
        self.stage_result_atomic(save, result)?;
        self.settle_atomic(save)
    }
}

impl CampaignSaveV1 {
    fn effective_economy_binding(&self) -> EconomyAccountBinding {
        self.economy_account_binding
            .clone()
            .unwrap_or_else(|| EconomyAccountBinding {
                actor_id: self.character.character_id.clone(),
                account_id: "trnm-offline-local-account".to_string(),
                binding_revision: self.revision,
            })
    }

    pub fn bind_cex_economy_account(
        &mut self,
        actor_id: impl Into<String>,
        account_id: impl Into<String>,
    ) -> Result<(), CampaignError> {
        let actor_id = actor_id.into();
        let account_id = account_id.into();
        if actor_id.trim().is_empty() || account_id.trim().is_empty() {
            return Err(CampaignError::InvalidState(
                "CEX actor_id and account_id are required".to_string(),
            ));
        }
        self.economy_mode = EconomyMode::CexConnected;
        self.economy_account_binding = Some(EconomyAccountBinding {
            actor_id,
            account_id: account_id.clone(),
            binding_revision: self.revision,
        });
        self.wallet_snapshot.account_id = account_id;
        self.revision = self.revision.saturating_add(1);
        self.validate()
    }

    pub fn use_offline_local_economy(&mut self) {
        self.economy_mode = EconomyMode::OfflineLocal;
        self.economy_account_binding = None;
        self.wallet_snapshot.account_id = "trnm-offline-local-account".to_string();
    }

    pub fn economy_asset_semantic(item_id: &str) -> EconomyAssetSemantic {
        if item_id == "trnm-soft-credit" {
            return EconomyAssetSemantic::soft_credit();
        }
        if item_id == "cex-wallet-credit" {
            return EconomyAssetSemantic::wallet_credit();
        }
        if item_id.starts_with("rts-resource:") {
            return EconomyAssetSemantic::temporary_battle_resource(item_id);
        }
        let tradeable = ECONOMY_ITEM_CATALOG
            .iter()
            .find(|item| item.id == item_id)
            .is_some_and(|item| item.material);
        EconomyAssetSemantic {
            asset_id: item_id.to_string(),
            asset_class: if tradeable {
                EconomyAssetClass::TradeableItem
            } else {
                EconomyAssetClass::BoundGameplayItem
            },
            transferability: if tradeable {
                EconomyTransferability::Tradeable
            } else {
                EconomyTransferability::Bound
            },
            settlement_authority: if tradeable {
                CEX_SETTLEMENT_BACKEND_ID.to_string()
            } else {
                "trnm-campaign-core".to_string()
            },
        }
    }

    fn queue_economic_intent(&mut self, draft: EconomicIntentDraft) -> Result<bool, CampaignError> {
        let EconomicIntentDraft {
            kind,
            term_id,
            intent_id,
            binding,
            asset_id,
            quantity,
            amount_credits,
            metadata,
        } = draft;
        let idempotency_key = format!("{}:{}", self.campaign_id, intent_id);
        if self.economic_idempotency_keys.contains(&idempotency_key) {
            return Ok(false);
        }
        let semantic = Self::economy_asset_semantic(&asset_id);
        let intent = EconomicIntent {
            protocol_version: TERM_EXCHANGE_PROTOCOL_VERSION.to_string(),
            intent_id,
            term_id,
            term_version: "v1".to_string(),
            domain: "trnm_game".to_string(),
            kind,
            idempotency_key: EconomyIdempotencyKey {
                scope: self.campaign_id.clone(),
                key: idempotency_key.clone(),
            },
            actors: vec![EconomyActorRef {
                actor_id: binding.actor_id,
                actor_kind: "trnm_player".to_string(),
                account_id: Some(binding.account_id),
            }],
            assets: vec![EconomyAssetRef {
                asset_id,
                asset_kind: format!("{:?}", semantic.asset_class).to_ascii_lowercase(),
                quantity,
                unit: "credits".to_string(),
            }],
            amount_credits: Some(amount_credits),
            currency: Some("wallet_credits".to_string()),
            metadata,
            created_at_epoch: i64::from(self.world_clock.day) * 86_400
                + i64::from(self.world_clock.minute_of_day) * 60,
        };
        intent.validate().map_err(CampaignError::InvalidState)?;
        if self.pending_economic_intents.len() >= 128 {
            return Err(CampaignError::InvalidState(
                "economic outbox reached its bounded capacity".to_string(),
            ));
        }
        self.economic_idempotency_keys.insert(idempotency_key);
        self.pending_economic_intents.push(intent);
        Ok(true)
    }

    fn queue_battle_reward_economy(
        &mut self,
        receipt: &SettlementReceiptV1,
    ) -> Result<(), CampaignError> {
        if receipt.duplicate || receipt.credit_delta <= 0 {
            return Ok(());
        }
        let binding = self.effective_economy_binding();
        self.queue_economic_intent(EconomicIntentDraft {
            kind: EconomicIntentKind::ReleaseReward,
            term_id: "trnm_battle_reward".to_string(),
            intent_id: format!("battle-reward:{}", receipt.battle_id),
            binding,
            asset_id: "cex-wallet-credit".to_string(),
            quantity: receipt.credit_delta,
            amount_credits: receipt.credit_delta,
            metadata: json!({
                "battle_id": receipt.battle_id,
                "result_hash": receipt.result_hash,
                "local_soft_credit_delta": receipt.credit_delta,
            }),
        })?;
        Ok(())
    }

    pub fn begin_selected_tradeable_purchase(&mut self) -> Result<String, CampaignError> {
        self.begin_selected_tradeable_purchase_with_seller_account(None)
    }

    pub fn begin_selected_tradeable_purchase_with_seller_account(
        &mut self,
        connected_seller_account_id: Option<&str>,
    ) -> Result<String, CampaignError> {
        let region_id = self.require_regional_market()?;
        let item = ECONOMY_ITEM_CATALOG
            .get(self.selected_shop_item_index % ECONOMY_ITEM_CATALOG.len())
            .ok_or_else(|| CampaignError::InvalidState("shop selection is missing".to_string()))?;
        let semantic = Self::economy_asset_semantic(item.id);
        if semantic.transferability != EconomyTransferability::Tradeable {
            return Err(CampaignError::InvalidState(format!(
                "{} is bound to local gameplay and never enters CEX",
                item.display_name
            )));
        }
        let (stock, demand) = self.regional_market_state(region_id, item.id);
        if stock == 0 {
            return Err(CampaignError::InvalidState(format!(
                "{} is out of regional stock",
                item.display_name
            )));
        }
        let price = market_price_with_state(item.id, self.world_clock.day, stock, demand, true)
            .ok_or_else(|| CampaignError::InvalidState("tradeable price is missing".to_string()))?;
        let buyer = self.effective_economy_binding();
        let seller_account_id = match (self.economy_mode, connected_seller_account_id) {
            (EconomyMode::CexConnected, Some(account_id)) if !account_id.trim().is_empty() => {
                account_id.to_string()
            }
            (EconomyMode::CexConnected, _) => {
                return Err(CampaignError::InvalidState(
                    "connected trade requires a configured CEX market account".to_string(),
                ));
            }
            (EconomyMode::OfflineLocal, _) => format!("offline-market:{region_id}"),
        };
        let seller = EconomyAccountBinding {
            actor_id: format!("trnm-market:{region_id}"),
            account_id: seller_account_id,
            binding_revision: self.revision,
        };
        let purchase_id = format!(
            "trade:{}:{}:{}",
            self.campaign_id, item.id, self.reconciliation_cursor
        );
        let reserve_intent_id = format!("{purchase_id}:reserve");
        self.pending_tradeable_purchases
            .push(PendingTradeablePurchase {
                purchase_id: purchase_id.clone(),
                item_id: item.id.to_string(),
                quantity: 1,
                price_wallet_credits: price,
                buyer: buyer.clone(),
                seller,
                stage: TradeablePurchaseStage::ReservePending,
                reserve_intent_id: reserve_intent_id.clone(),
                settle_intent_id: None,
                consume_intent_id: None,
                refund_intent_id: None,
            });
        self.queue_economic_intent(EconomicIntentDraft {
            kind: EconomicIntentKind::Reserve,
            term_id: "trnm_tradeable_purchase".to_string(),
            intent_id: reserve_intent_id,
            binding: buyer,
            asset_id: item.id.to_string(),
            quantity: 1,
            amount_credits: price,
            metadata: json!({"purchase_id": purchase_id, "stage": "reserve", "region_id": region_id}),
        })?;
        Ok(purchase_id)
    }

    pub fn cancel_tradeable_purchase(&mut self, purchase_id: &str) -> Result<(), CampaignError> {
        let index = self
            .pending_tradeable_purchases
            .iter()
            .position(|purchase| purchase.purchase_id == purchase_id)
            .ok_or_else(|| {
                CampaignError::InvalidState("tradeable purchase is missing".to_string())
            })?;
        let purchase = self.pending_tradeable_purchases[index].clone();
        let (kind, binding) = if purchase.stage == TradeablePurchaseStage::Consumed {
            (EconomicIntentKind::Chargeback, purchase.seller.clone())
        } else {
            (EconomicIntentKind::Refund, purchase.buyer.clone())
        };
        let intent_id = format!("{}:recovery", purchase.purchase_id);
        self.pending_tradeable_purchases[index].stage = TradeablePurchaseStage::RefundPending;
        self.pending_tradeable_purchases[index].refund_intent_id = Some(intent_id.clone());
        self.queue_economic_intent(EconomicIntentDraft {
            kind,
            term_id: "trnm_tradeable_purchase_recovery".to_string(),
            intent_id,
            binding,
            asset_id: purchase.item_id,
            quantity: i64::from(purchase.quantity),
            amount_credits: purchase.price_wallet_credits,
            metadata: json!({"purchase_id": purchase.purchase_id, "stage": "refund_or_chargeback"}),
        })?;
        Ok(())
    }

    fn apply_verified_economic_receipt(
        &mut self,
        intent: &EconomicIntent,
        receipt: &EconomicReceipt,
    ) -> Result<(), CampaignError> {
        receipt
            .validate_for(intent)
            .map_err(CampaignError::InvalidState)?;
        let bound_account = self.effective_economy_binding().account_id;
        let applies_to_bound_wallet = intent
            .actors
            .first()
            .and_then(|actor| actor.account_id.as_deref())
            == Some(bound_account.as_str());
        let amount = intent.amount_credits.unwrap_or_default().max(0);
        if applies_to_bound_wallet && receipt.allows_progression() {
            match intent.kind {
                EconomicIntentKind::ReleaseReward | EconomicIntentKind::Settle => {
                    self.wallet_snapshot.available_credits = self
                        .wallet_snapshot
                        .available_credits
                        .saturating_add(amount);
                }
                EconomicIntentKind::Reserve => {
                    self.wallet_snapshot.available_credits = self
                        .wallet_snapshot
                        .available_credits
                        .saturating_sub(amount);
                    self.wallet_snapshot.reserved_credits =
                        self.wallet_snapshot.reserved_credits.saturating_add(amount);
                }
                EconomicIntentKind::Consume => {
                    self.wallet_snapshot.reserved_credits =
                        self.wallet_snapshot.reserved_credits.saturating_sub(amount);
                }
                EconomicIntentKind::Refund => {
                    self.wallet_snapshot.available_credits = self
                        .wallet_snapshot
                        .available_credits
                        .saturating_add(amount);
                    self.wallet_snapshot.reserved_credits =
                        self.wallet_snapshot.reserved_credits.saturating_sub(amount);
                }
                _ => {}
            }
        }

        if intent.term_id == "trnm_battle_reward" && receipt.allows_progression() {
            if let Some(settlement) = self.settlement_receipts.iter_mut().find(|settlement| {
                settlement.economic_intent_id.as_deref() == Some(intent.intent_id.as_str())
            }) {
                settlement.economic_receipt_id = Some(receipt.receipt_id.clone());
            }
        }

        let Some(index) = self
            .pending_tradeable_purchases
            .iter()
            .position(|purchase| {
                purchase.reserve_intent_id == intent.intent_id
                    || purchase.settle_intent_id.as_deref() == Some(intent.intent_id.as_str())
                    || purchase.consume_intent_id.as_deref() == Some(intent.intent_id.as_str())
                    || purchase.refund_intent_id.as_deref() == Some(intent.intent_id.as_str())
            })
        else {
            return Ok(());
        };
        if !receipt.allows_progression() {
            if receipt.progression_class == ReceiptProgressionClass::HardFail {
                self.pending_tradeable_purchases[index].stage = TradeablePurchaseStage::HardFailed;
            }
            return Ok(());
        }
        let purchase = self.pending_tradeable_purchases[index].clone();
        if purchase.reserve_intent_id == intent.intent_id {
            let next_id = format!("{}:settle", purchase.purchase_id);
            self.pending_tradeable_purchases[index].stage =
                TradeablePurchaseStage::SellerSettlementPending;
            self.pending_tradeable_purchases[index].settle_intent_id = Some(next_id.clone());
            self.queue_economic_intent(EconomicIntentDraft {
                kind: EconomicIntentKind::Settle,
                term_id: "trnm_tradeable_purchase".to_string(),
                intent_id: next_id,
                binding: purchase.seller,
                asset_id: purchase.item_id,
                quantity: i64::from(purchase.quantity),
                amount_credits: purchase.price_wallet_credits,
                metadata: json!({"purchase_id": purchase.purchase_id, "stage": "seller_settlement"}),
            })?;
        } else if purchase.settle_intent_id.as_deref() == Some(intent.intent_id.as_str()) {
            let next_id = format!("{}:consume", purchase.purchase_id);
            self.pending_tradeable_purchases[index].stage =
                TradeablePurchaseStage::BuyerConsumePending;
            self.pending_tradeable_purchases[index].consume_intent_id = Some(next_id.clone());
            self.queue_economic_intent(EconomicIntentDraft {
                kind: EconomicIntentKind::Consume,
                term_id: "trnm_tradeable_purchase".to_string(),
                intent_id: next_id,
                binding: purchase.buyer,
                asset_id: purchase.item_id,
                quantity: i64::from(purchase.quantity),
                amount_credits: purchase.price_wallet_credits,
                metadata: json!({"purchase_id": purchase.purchase_id, "stage": "buyer_consume"}),
            })?;
        } else if purchase.consume_intent_id.as_deref() == Some(intent.intent_id.as_str()) {
            self.pending_tradeable_purchases[index].stage = TradeablePurchaseStage::Consumed;
            merge_loot(
                &mut self.progression.inventory,
                &[LootStack {
                    item_id: purchase.item_id,
                    quantity: purchase.quantity,
                }],
            );
        } else if purchase.refund_intent_id.as_deref() == Some(intent.intent_id.as_str()) {
            self.pending_tradeable_purchases[index].stage = TradeablePurchaseStage::Refunded;
        }
        Ok(())
    }

    pub fn reconcile_economy<B: EconomyBackend>(
        &mut self,
        backend: &B,
        max_intents: usize,
    ) -> Result<EconomyReconciliationReport, CampaignError> {
        let mut report = EconomyReconciliationReport::default();
        while (report.attempted as usize) < max_intents {
            let Some(intent) = self.pending_economic_intents.first().cloned() else {
                break;
            };
            report.attempted = report.attempted.saturating_add(1);
            let receipt = match backend.execute(&intent) {
                Ok(receipt) => receipt,
                Err(error) => {
                    report.recoverable_holds = report.recoverable_holds.saturating_add(1);
                    report.last_error = Some(error);
                    break;
                }
            };
            if let Err(error) = receipt.validate_for(&intent) {
                report.hard_failures = report.hard_failures.saturating_add(1);
                report.last_error = Some(error);
                self.economic_dead_letters.push(intent);
                self.pending_economic_intents.remove(0);
                continue;
            }
            if let Some(existing) = self
                .verified_economic_receipts
                .iter_mut()
                .find(|existing| existing.receipt_id == receipt.receipt_id)
            {
                *existing = receipt.clone();
            } else {
                self.verified_economic_receipts.push(receipt.clone());
            }
            match receipt.progression_class {
                ReceiptProgressionClass::ProgressionAllowed
                | ReceiptProgressionClass::TerminalSkip => {
                    self.pending_economic_intents.remove(0);
                    self.apply_verified_economic_receipt(&intent, &receipt)?;
                    report.applied = report.applied.saturating_add(1);
                }
                ReceiptProgressionClass::RecoverableHold => {
                    report.recoverable_holds = report.recoverable_holds.saturating_add(1);
                    report.last_error = receipt.reason.clone();
                    break;
                }
                ReceiptProgressionClass::HardFail => {
                    self.pending_economic_intents.remove(0);
                    self.economic_dead_letters.push(intent.clone());
                    self.apply_verified_economic_receipt(&intent, &receipt)?;
                    report.hard_failures = report.hard_failures.saturating_add(1);
                }
            }
            self.reconciliation_cursor = self.reconciliation_cursor.saturating_add(1);
        }
        if let Some(binding) = self.economy_account_binding.as_ref() {
            if let Ok(Some(snapshot)) = backend.wallet_snapshot(binding, self.reconciliation_cursor)
            {
                self.wallet_snapshot = snapshot;
            }
        }
        if self.verified_economic_receipts.len() > 256 {
            let keep_from = self.verified_economic_receipts.len() - 256;
            self.verified_economic_receipts.drain(..keep_from);
        }
        report.remaining = self.pending_economic_intents.len();
        self.validate()?;
        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn map() -> BattleMapSeedV1 {
        BattleMapSeedV1 {
            width: 16,
            height: 8,
            terrain_rows: vec!["gggggggggggggggg".to_string(); 8],
            party_start: BattleGridPoint::new(1, 6),
            approach_point: BattleGridPoint::new(6, 5),
            objective: BattleGridPoint::new(14, 1),
            resource_nodes: vec![BattleMapNodeV1 {
                id: "amber_mid".to_string(),
                position: BattleGridPoint::new(7, 6),
            }],
            enemy_spawns: vec![
                BattleMapNodeV1 {
                    id: "enemy_0".to_string(),
                    position: BattleGridPoint::new(9, 4),
                },
                BattleMapNodeV1 {
                    id: "enemy_1".to_string(),
                    position: BattleGridPoint::new(11, 3),
                },
                BattleMapNodeV1 {
                    id: "enemy_2".to_string(),
                    position: BattleGridPoint::new(13, 2),
                },
            ],
        }
    }

    fn ready_campaign() -> CampaignSaveV1 {
        let mut campaign = CampaignSaveV1::default();
        campaign.move_to(CampaignRoom::MentorHall).unwrap();
        campaign.talk_to_mentor().unwrap();
        campaign.train_with_mentor().unwrap();
        campaign.equip_starter_weapon().unwrap();
        campaign.move_to(CampaignRoom::ExpeditionGate).unwrap();
        campaign.accept_first_contact_quest().unwrap();
        campaign
    }

    fn terminal_result(seed: &BattleSeedV1, outcome: BattleOutcome) -> BattleResultV1 {
        BattleResultV1 {
            contract_version: BATTLE_RESULT_CONTRACT.to_string(),
            battle_id: seed.battle_id.clone(),
            seed_hash: seed.seed_hash.clone(),
            outcome,
            units: seed
                .party
                .iter()
                .map(|unit| UnitBattleReportV1 {
                    unit_id: unit.unit_id.clone(),
                    status: UnitBattleStatus::Healthy,
                    remaining_hp: unit.stats.max_hp,
                    experience_gained: 30,
                    veteran_rank: 0,
                    confirmed_kills: 0,
                })
                .collect(),
            loot: vec![LootStack {
                item_id: "relay-core-fragment".to_string(),
                quantity: 1,
            }],
            resource_delta: 80,
            reputation_delta: 4,
            world_flags: vec!["first_contact_secured".to_string()],
            elapsed_ticks: 6_000,
            final_snapshot_hash: "a".repeat(64),
        }
    }

    #[test]
    fn mentor_training_and_loadout_are_required_before_battle() {
        let mut campaign = CampaignSaveV1::default();
        assert!(campaign.move_to(CampaignRoom::ExpeditionGate).is_err());
        let mut campaign = ready_campaign();
        let seed = campaign.start_first_contact_battle(map()).unwrap();
        assert_eq!(seed.party.len(), 4);
        assert!(seed.party[0]
            .equipment_ids
            .iter()
            .any(|item| item == "route-guard-staff"));
        seed.validate().unwrap();
    }

    #[test]
    fn seed_hash_rejects_tampered_rpg_stats() {
        let mut campaign = ready_campaign();
        let mut seed = campaign.start_first_contact_battle(map()).unwrap();
        seed.party[0].stats.damage += 999;
        assert!(matches!(seed.validate(), Err(CampaignError::Integrity(_))));
    }

    #[test]
    fn attribute_skill_equipment_mapping_is_typed_and_monotonic() {
        let attributes = TrillionniumAttributes::default();
        let base = map_rpg_to_rts_stats(&attributes, 1, &[], 0);
        let equipped = map_rpg_to_rts_stats(&attributes, 3, &["route-guard-staff".to_string()], 0);
        assert!(equipped.damage > base.damage);
        assert!(equipped.armor > base.armor);
        assert!(equipped.skill_power_permille > base.skill_power_permille);
        let wounded = map_rpg_to_rts_stats(&attributes, 3, &[], 2);
        assert!(wounded.max_hp < base.max_hp);
    }

    #[test]
    fn result_is_staged_before_settlement_and_duplicate_is_zero_delta() {
        let mut campaign = ready_campaign();
        let seed = campaign.start_first_contact_battle(map()).unwrap();
        let mut result = terminal_result(&seed, BattleOutcome::Victory);
        result.units[0].veteran_rank = 1;
        result.units[0].confirmed_kills = 2;
        campaign.stage_battle_result(result.clone()).unwrap();
        assert_eq!(campaign.phase, CampaignPhase::PostBattlePending);
        assert_eq!(campaign.progression.experience, 0);
        let receipt = campaign.apply_pending_settlement().unwrap();
        assert!(!receipt.duplicate);
        assert_eq!(receipt.experience_delta, 120);
        assert_eq!(receipt.credit_delta, 80);
        let expected_economic_intent_id = format!("battle-reward:{}", receipt.battle_id);
        assert_eq!(
            receipt.economic_intent_id.as_deref(),
            Some(expected_economic_intent_id.as_str())
        );
        let economic_receipt_id = receipt
            .economic_receipt_id
            .as_deref()
            .expect("offline battle reward receipt is linked");
        assert!(campaign.verified_economic_receipts.iter().any(|economic| {
            economic.receipt_id == economic_receipt_id
                && economic.intent_id == receipt.economic_intent_id.as_deref().unwrap()
        }));
        assert_eq!(campaign.quest_state, QuestState::Completed);
        assert_eq!(campaign.party[0].veteran_rank, 1);
        assert_eq!(campaign.party[0].confirmed_kills, 2);
        assert_eq!(campaign.room, CampaignRoom::MirrorSquare);
        let before = campaign.clone();
        let duplicate = campaign.submit_battle_result(result).unwrap();
        assert!(duplicate.duplicate);
        assert_eq!(duplicate.experience_delta, 0);
        assert_eq!(campaign, before);
    }

    #[test]
    fn atomic_store_recovers_a_post_battle_crash_once() {
        let directory = tempdir().unwrap();
        let store = CampaignStore::new(directory.path().join("campaign.json"));
        let mut campaign = ready_campaign();
        let seed = campaign.start_first_contact_battle(map()).unwrap();
        store.save_atomic(&campaign).unwrap();
        store
            .stage_result_atomic(
                &mut campaign,
                terminal_result(&seed, BattleOutcome::Victory),
            )
            .unwrap();
        let mut restarted = store.load().unwrap();
        assert_eq!(restarted.phase, CampaignPhase::PostBattlePending);
        let receipt = store
            .recover_pending_settlement(&mut restarted)
            .unwrap()
            .unwrap();
        assert!(!receipt.duplicate);
        let recovered = store.load().unwrap();
        assert_eq!(recovered.phase, CampaignPhase::Town);
        assert_eq!(recovered.progression.experience, 120);
        assert!(store
            .recover_pending_settlement(&mut restarted)
            .unwrap()
            .is_none());
    }

    #[test]
    fn corrupt_temp_file_cannot_replace_last_atomic_save() {
        let directory = tempdir().unwrap();
        let store = CampaignStore::new(directory.path().join("campaign.json"));
        let campaign = CampaignSaveV1::default();
        store.save_atomic(&campaign).unwrap();
        fs::write(store.path().with_extension("json.tmp"), b"{broken").unwrap();
        assert_eq!(store.load().unwrap(), campaign);
    }

    #[test]
    fn training_is_paid_capped_and_paths_are_real_choices() {
        let mut campaign = CampaignSaveV1::default();
        campaign.move_to(CampaignRoom::MentorHall).unwrap();
        campaign.talk_to_mentor().unwrap();
        let initial_credits = campaign.progression.credits;
        campaign.train_with_mentor().unwrap();
        campaign.cycle_training_path().unwrap();
        campaign.train_with_mentor().unwrap();
        assert_eq!(campaign.progression.mentor_training_sessions, 2);
        assert!(campaign.progression.credits < initial_credits);
        assert!(campaign.train_with_mentor().is_err());
        assert!(campaign
            .character
            .skill_ids
            .iter()
            .any(|skill| skill == "iron_guard"));
        assert!(campaign
            .character
            .skill_ids
            .iter()
            .any(|skill| skill == "wind_step"));
    }

    #[test]
    fn party_loadout_and_healing_create_persistent_tradeoffs() {
        let mut campaign = CampaignSaveV1::default();
        assert_eq!(campaign.party.len(), 7);
        campaign.cycle_party_preset().unwrap();
        assert_eq!(campaign.active_party_ids, ["hero", "aya", "nia", "sol"]);
        campaign.cycle_loadout().unwrap();
        assert_eq!(campaign.selected_loadout, LoadoutPreset::Guard);
        campaign.cycle_loadout().unwrap();
        assert_eq!(campaign.selected_loadout, LoadoutPreset::Raider);
        campaign.party[0].injury_level = 2;
        let credits = campaign.progression.credits;
        campaign.heal_party().unwrap();
        assert_eq!(campaign.party[0].injury_level, 1);
        assert_eq!(
            campaign.progression.credits,
            credits - FIELD_CLINIC_CREDIT_COST
        );
        campaign.progression.inventory.push(LootStack {
            item_id: "relay-core-fragment".to_string(),
            quantity: 1,
        });
        campaign.equip_relay_core().unwrap();
        assert!(campaign.character.equipment_slots.contains_key("relic"));
        let modifier = typed_equipment_modifier("relay-core-fragment");
        assert!(modifier.energy > 0 && modifier.ability_range > 0);
        let coat = typed_equipment_modifier("compass-thread-coat");
        assert!(coat.armor > 0 && coat.move_speed_milli > 0 && coat.evasion_permille > 0);
        let lens = typed_equipment_modifier("emberglass-lens");
        assert!(lens.energy > 0 && lens.ability_range > 0);
        for item in ECONOMY_ITEM_CATALOG.iter().filter(|item| !item.material) {
            let modifier = typed_equipment_modifier(item.id);
            assert!(
                modifier.max_hp != 0
                    || modifier.damage != 0
                    || modifier.armor != 0
                    || modifier.move_speed_milli != 0
                    || modifier.attack_interval_ticks != 0
                    || modifier.evasion_permille != 0
                    || modifier.energy != 0
                    || modifier.ability_range != 0,
                "non-material catalog item {} has no explicit BattleSeed modifier",
                item.id
            );
        }
    }

    #[test]
    fn free_party_relationship_recruitment_and_sparring_are_persistent() {
        let mut campaign = CampaignSaveV1::default();
        assert!(
            !campaign
                .party
                .iter()
                .find(|member| member.unit_id == "brann")
                .unwrap()
                .available
        );
        campaign.move_to(CampaignRoom::MentorHall).unwrap();
        campaign.talk_to_mentor().unwrap();
        campaign.train_with_mentor().unwrap();
        let report = campaign.spar_with_mentor().unwrap();
        assert_eq!(report.outcome, SparringOutcome::Victory);
        assert_eq!(campaign.faction_rank, FactionRank::Disciple);
        assert_eq!(
            campaign.character.sect_id.as_deref(),
            Some("signal-road-school")
        );

        campaign
            .progression
            .world_flags
            .insert("signal_road_secured".to_string());
        campaign.move_to(CampaignRoom::MirrorSquare).unwrap();
        campaign.move_to(CampaignRoom::RelayQuarter).unwrap();
        campaign.talk_to_relay_smith().unwrap();
        campaign.recruit_relay_smith().unwrap();
        assert!(
            campaign
                .party
                .iter()
                .find(|member| member.unit_id == "brann")
                .unwrap()
                .available
        );
        campaign
            .select_party(vec![
                "hero".to_string(),
                "aya".to_string(),
                "mako".to_string(),
                "brann".to_string(),
            ])
            .unwrap();
        assert!(campaign.validate().is_ok());
    }

    #[test]
    fn growth_preview_confirm_cancel_and_reload_are_atomic() {
        let directory = tempdir().unwrap();
        let store = CampaignStore::new(directory.path().join("growth.json"));
        let mut campaign = CampaignSaveV1::default();
        let force_before = campaign.character.attributes.force;
        campaign
            .preview_growth_allocation(GrowthStat::Force)
            .unwrap();
        assert_eq!(campaign.character.attributes.force, force_before);
        assert_eq!(campaign.progression.growth_points_available, 1);
        campaign.cancel_growth_allocation().unwrap();
        assert_eq!(campaign.progression.growth_points_available, 1);

        campaign
            .preview_growth_allocation(GrowthStat::Force)
            .unwrap();
        campaign.confirm_growth_allocation().unwrap();
        assert_eq!(campaign.character.attributes.force, force_before + 1);
        assert_eq!(campaign.progression.growth_points_available, 0);
        assert_eq!(campaign.build_path, BuildPath::Vanguard);
        assert_eq!(campaign.active_title, None);
        assert!(campaign.unlocked_titles.is_empty());
        assert!(campaign.confirm_growth_allocation().is_err());
        store.save_atomic(&campaign).unwrap();
        assert_eq!(store.load().unwrap(), campaign);
    }

    #[test]
    fn force_and_agility_builds_emit_observably_different_battle_seeds() {
        let prepare = |stat| {
            let mut campaign = CampaignSaveV1::default();
            campaign.preview_growth_allocation(stat).unwrap();
            campaign.confirm_growth_allocation().unwrap();
            campaign.move_to(CampaignRoom::MentorHall).unwrap();
            campaign.talk_to_mentor().unwrap();
            campaign.train_with_mentor().unwrap();
            campaign.equip_starter_weapon().unwrap();
            campaign.move_to(CampaignRoom::ExpeditionGate).unwrap();
            campaign.accept_first_contact_quest().unwrap();
            campaign.start_first_contact_battle(map()).unwrap()
        };
        let force = prepare(GrowthStat::Force);
        let agility = prepare(GrowthStat::Agility);
        assert!(force.party[0].stats.damage > agility.party[0].stats.damage);
        assert!(agility.party[0].stats.move_speed_milli > force.party[0].stats.move_speed_milli);
        assert_ne!(force.seed_hash, agility.seed_hash);
    }

    #[test]
    fn three_origins_by_three_paths_emit_nine_observable_builds() {
        let origins = [
            CharacterOrigin::Balanced,
            CharacterOrigin::Artisan,
            CharacterOrigin::Scout,
        ];
        let paths = [GrowthStat::Force, GrowthStat::Agility, GrowthStat::Craft];
        let mut hashes = BTreeSet::new();
        let mut stat_signatures = BTreeSet::new();
        for origin in origins {
            for stat in paths {
                let mut campaign = CampaignSaveV1::default();
                while campaign.character_origin != origin {
                    campaign.cycle_character_origin().unwrap();
                }
                campaign.preview_growth_allocation(stat).unwrap();
                campaign.confirm_growth_allocation().unwrap();
                campaign.move_to(CampaignRoom::MentorHall).unwrap();
                campaign.talk_to_mentor().unwrap();
                campaign.train_with_mentor().unwrap();
                campaign.attempt_mastery_challenge().unwrap();
                campaign.equip_starter_weapon().unwrap();
                campaign.move_to(CampaignRoom::ExpeditionGate).unwrap();
                campaign.accept_first_contact_quest().unwrap();
                let seed = campaign.start_first_contact_battle(map()).unwrap();
                assert_eq!(seed.character_origin, origin);
                assert!(seed.active_title.is_some());
                hashes.insert(seed.seed_hash.clone());
                let hero = &seed.party[0].stats;
                stat_signatures.insert((
                    hero.max_hp,
                    hero.damage,
                    hero.armor,
                    hero.move_speed_milli,
                    hero.energy,
                    hero.ability_range,
                ));
            }
        }
        assert_eq!(hashes.len(), 9);
        assert_eq!(stat_signatures.len(), 9);
    }

    #[test]
    fn task_navigation_reports_next_exit_and_locked_failure() {
        let campaign = CampaignSaveV1::default();
        let route = campaign.current_task_route_plan();
        assert_eq!(
            route.next_exit.as_ref().map(|exit| exit.to.as_str()),
            Some(MENTOR_HALL_ROOM)
        );
        let mut campaign = campaign;
        campaign.story.current_step = StoryStepId::SignalRoadComplete;
        let blocked = campaign.current_task_route_plan();
        assert!(matches!(
            blocked.blocked_reason,
            Some(trnm_rpg_core::WorldRouteBlockedReason::LockedRoom { .. })
        ));
    }

    #[test]
    fn typed_rpg_encounter_applies_item_injury_loot_and_route_consequences() {
        let mut campaign = CampaignSaveV1::default();
        campaign
            .progression
            .world_flags
            .insert("signal_road_secured".to_string());
        campaign.move_to(CampaignRoom::RelayQuarter).unwrap();
        campaign.progression.inventory.push(LootStack {
            item_id: "field-tonic-kit".to_string(),
            quantity: 1,
        });
        campaign.begin_signal_road_encounter().unwrap();
        campaign
            .act_in_signal_road_encounter(EncounterAction::Defend)
            .unwrap();
        campaign
            .act_in_signal_road_encounter(EncounterAction::UseItem)
            .unwrap();
        while campaign.active_encounter.is_some() {
            campaign
                .act_in_signal_road_encounter(EncounterAction::Attack)
                .unwrap();
        }
        assert_eq!(
            campaign.last_encounter_outcome,
            Some(EncounterOutcome::Victory)
        );
        assert!(campaign
            .progression
            .inventory
            .iter()
            .any(|stack| stack.item_id == "signal-road-emblem"));
        assert!(campaign
            .progression
            .world_flags
            .contains("signal_road_ambush_cleared"));
        assert!(!campaign
            .progression
            .inventory
            .iter()
            .any(|stack| stack.item_id == "field-tonic-kit"));

        campaign.begin_signal_road_encounter().unwrap();
        campaign
            .act_in_signal_road_encounter(EncounterAction::Withdraw)
            .unwrap();
        assert_eq!(
            campaign.last_encounter_outcome,
            Some(EncounterOutcome::Withdrawn)
        );
        assert!(campaign
            .progression
            .world_flags
            .contains("signal_road_ambush_withdrawn"));

        campaign.begin_signal_road_encounter().unwrap();
        campaign.active_encounter.as_mut().unwrap().player_hp = 1;
        campaign
            .act_in_signal_road_encounter(EncounterAction::Attack)
            .unwrap();
        assert_eq!(
            campaign.last_encounter_outcome,
            Some(EncounterOutcome::Defeat)
        );
        assert_eq!(campaign.party[0].injury_level, 1);
    }

    #[test]
    fn build_titles_unlock_a_route_encounter_and_real_price() {
        let mut runner = CampaignSaveV1::default();
        runner
            .preview_growth_allocation(GrowthStat::Agility)
            .unwrap();
        runner.confirm_growth_allocation().unwrap();
        runner.move_to(CampaignRoom::MentorHall).unwrap();
        runner.talk_to_mentor().unwrap();
        runner.train_with_mentor().unwrap();
        runner.attempt_mastery_challenge().unwrap();
        runner.move_to(CampaignRoom::MirrorSquare).unwrap();
        runner.move_to(CampaignRoom::RelayQuarter).unwrap();
        assert_eq!(runner.active_title, Some(BuildTitle::RelayRunner));

        let mut smith = CampaignSaveV1::default();
        smith.preview_growth_allocation(GrowthStat::Craft).unwrap();
        smith.confirm_growth_allocation().unwrap();
        smith.move_to(CampaignRoom::MentorHall).unwrap();
        smith.talk_to_mentor().unwrap();
        smith.train_with_mentor().unwrap();
        smith.attempt_mastery_challenge().unwrap();
        smith.move_to(CampaignRoom::MirrorSquare).unwrap();
        smith.party[0].injury_level = 1;
        let credits = smith.progression.credits;
        smith.heal_party().unwrap();
        assert_eq!(smith.progression.credits, credits - 25);

        let mut warden = CampaignSaveV1::default();
        warden.preview_growth_allocation(GrowthStat::Force).unwrap();
        warden.confirm_growth_allocation().unwrap();
        warden.move_to(CampaignRoom::MentorHall).unwrap();
        warden.talk_to_mentor().unwrap();
        warden.train_with_mentor().unwrap();
        warden.attempt_mastery_challenge().unwrap();
        warden.move_to(CampaignRoom::MirrorSquare).unwrap();
        warden.move_to(CampaignRoom::ExpeditionGate).unwrap();
        warden.begin_signal_road_encounter().unwrap();
        assert!(warden.active_encounter.is_some());
    }

    #[test]
    fn three_save_slots_are_isolated_and_settings_are_profile_scoped() {
        let directory = tempdir().unwrap();
        let slots = SaveSlotStore::new(directory.path());
        let mut first = slots.create_new(SaveSlotId::A, false).unwrap();
        first.progression.credits = 777;
        slots.save_atomic(SaveSlotId::A, &first).unwrap();
        let second = slots.create_new(SaveSlotId::B, false).unwrap();
        assert_ne!(first.campaign_id, second.campaign_id);
        assert_eq!(slots.load(SaveSlotId::A).unwrap().progression.credits, 777);
        assert_ne!(slots.load(SaveSlotId::B).unwrap().progression.credits, 777);
        assert!(slots.create_new(SaveSlotId::A, false).is_err());

        fs::write(slots.path(SaveSlotId::C), b"not-json").unwrap();
        assert!(!slots.metadata(SaveSlotId::C).valid);
        assert!(slots.metadata(SaveSlotId::A).valid);

        let settings_store =
            PlayerSettingsStore::new(directory.path().join("player-settings.json"));
        let settings = PlayerSettings {
            low_motion: true,
            input_mode: InputMode::KeyboardOnly,
            ..PlayerSettings::default()
        };
        settings_store.save_atomic(&settings).unwrap();
        slots.create_new(SaveSlotId::A, true).unwrap();
        assert_eq!(settings_store.load_or_default().unwrap(), settings);
    }

    #[test]
    fn cistern_relief_is_a_typed_branching_persistent_quest_chain() {
        let mut campaign = CampaignSaveV1::default();
        campaign
            .progression
            .world_flags
            .insert("outer_signal_road_open".to_string());
        campaign
            .progression
            .world_flags
            .insert("signal_road_secured".to_string());
        campaign
            .progression
            .world_flags
            .insert("expedition_gate_open".to_string());
        campaign.move_to(CampaignRoom::RelayQuarter).unwrap();
        campaign.start_cistern_relief().unwrap();
        assert_eq!(
            campaign.quest_chain.as_ref().unwrap().current_node,
            QuestChainNodeId::SurveyDamage
        );
        campaign.advance_cistern_relief().unwrap();
        campaign.move_to(CampaignRoom::MirrorSquare).unwrap();
        campaign.move_to(CampaignRoom::ExpeditionGate).unwrap();
        let credits_before = campaign.progression.credits;
        campaign.advance_cistern_relief().unwrap();
        assert_eq!(campaign.progression.credits, credits_before - 40);
        campaign.move_to(CampaignRoom::MirrorSquare).unwrap();
        campaign.move_to(CampaignRoom::RelayQuarter).unwrap();
        campaign.advance_cistern_relief().unwrap_err();

        let mut reinforce = campaign.clone();
        let mut evacuate = campaign;
        reinforce
            .choose_cistern_relief_branch(QuestBranch::ReinforceCistern)
            .unwrap();
        evacuate
            .choose_cistern_relief_branch(QuestBranch::EvacuateFamilies)
            .unwrap();
        assert!(reinforce
            .progression
            .world_flags
            .contains("cistern_reinforced"));
        assert!(evacuate
            .progression
            .world_flags
            .contains("cistern_families_evacuated"));
        assert!(evacuate.progression.credits > reinforce.progression.credits);
        assert!(
            reinforce.character.attributes.reputation > evacuate.character.attributes.reputation
        );

        let directory = tempdir().unwrap();
        let store = CampaignStore::new(directory.path().join("quest.json"));
        store.save_atomic(&reinforce).unwrap();
        assert_eq!(store.load().unwrap().quest_chain, reinforce.quest_chain);
    }

    #[test]
    fn expedition_preparation_changes_seed_time_supplies_and_battle_stats() {
        let immediate_campaign = ready_campaign();
        let mut supplied_campaign = immediate_campaign.clone();
        supplied_campaign.cycle_expedition_preparation().unwrap();
        supplied_campaign.cycle_expedition_preparation().unwrap();
        let mut immediate_campaign = immediate_campaign;
        let immediate = immediate_campaign
            .start_first_contact_battle(map())
            .unwrap();
        let supplied = supplied_campaign.start_first_contact_battle(map()).unwrap();
        assert_eq!(
            immediate.expedition_readiness.preparation,
            ExpeditionPreparation::Immediate
        );
        assert_eq!(
            supplied.expedition_readiness.preparation,
            ExpeditionPreparation::Supplied
        );
        assert_eq!(supplied.expedition_readiness.starting_resources, 50);
        assert!(supplied.party[0].stats.energy > immediate.party[0].stats.energy);
        assert_ne!(supplied.seed_hash, immediate.seed_hash);
        assert_ne!(
            supplied_campaign.world_clock,
            immediate_campaign.world_clock
        );
        assert_ne!(
            supplied_campaign.expedition_supplies,
            immediate_campaign.expedition_supplies
        );
    }

    #[test]
    fn new_slot_requires_identity_confirmation_and_persists_one_canonical_name() {
        let directory = tempdir().unwrap();
        let slots = SaveSlotStore::new(directory.path());
        let mut campaign = slots.create_new(SaveSlotId::A, false).unwrap();
        assert!(!campaign.character_identity.confirmed);
        assert_eq!(campaign.character.display_name, "Mirror Ranger");
        assert_eq!(
            campaign.cycle_character_identity().unwrap(),
            CharacterNamePreset::SignalRook
        );
        campaign.confirm_character_identity().unwrap();
        slots.save_atomic(SaveSlotId::A, &campaign).unwrap();
        let loaded = slots.load(SaveSlotId::A).unwrap();
        assert!(loaded.character_identity.confirmed);
        assert_eq!(loaded.character.display_name, "Signal Rook");
        assert_eq!(loaded.party[0].display_name, "Signal Rook");
        assert!(loaded.validate().is_ok());
        assert!(campaign.cycle_character_identity().is_err());
    }

    #[test]
    fn progressive_guide_and_journal_follow_authoritative_campaign_state() {
        let directory = tempdir().unwrap();
        let store = CampaignStore::new(directory.path().join("guided.json"));
        let mut campaign = CampaignSaveV1::default();
        assert_eq!(campaign.current_guide_step(), CampaignGuideStep::MeetMentor);
        campaign.move_to(CampaignRoom::MentorHall).unwrap();
        campaign.talk_to_mentor().unwrap();
        assert_eq!(
            campaign.current_guide_step(),
            CampaignGuideStep::TrainWithMentor
        );
        campaign.train_with_mentor().unwrap();
        assert_eq!(
            campaign.current_guide_step(),
            CampaignGuideStep::EquipWeapon
        );
        campaign.equip_starter_weapon().unwrap();
        assert_eq!(
            campaign.current_guide_step(),
            CampaignGuideStep::ReachExpeditionGate
        );
        campaign.move_to(CampaignRoom::ExpeditionGate).unwrap();
        assert_eq!(
            campaign.current_guide_step(),
            CampaignGuideStep::AcceptMission
        );
        campaign.accept_first_contact_quest().unwrap();
        assert_eq!(
            campaign.current_guide_step(),
            CampaignGuideStep::DeployMission
        );
        let journal = campaign.campaign_journal();
        assert_eq!(journal.len(), 3);
        assert_eq!(journal[0].state, CampaignJournalState::Active);
        store.save_atomic(&campaign).unwrap();
        assert_eq!(store.load().unwrap().campaign_journal(), journal);
    }

    #[test]
    fn difficulty_changes_seed_and_mirror_siege_follows_the_convoy() {
        let mut standard = ready_campaign();
        let standard_seed = standard.start_first_contact_battle(map()).unwrap();
        let mut veteran = CampaignSaveV1::default();
        assert_eq!(
            veteran.cycle_difficulty().unwrap(),
            CampaignDifficulty::Veteran
        );
        veteran.move_to(CampaignRoom::MentorHall).unwrap();
        veteran.talk_to_mentor().unwrap();
        veteran.train_with_mentor().unwrap();
        veteran.equip_starter_weapon().unwrap();
        veteran.move_to(CampaignRoom::ExpeditionGate).unwrap();
        veteran.accept_first_contact_quest().unwrap();
        let veteran_seed = veteran.start_first_contact_battle(map()).unwrap();
        assert_ne!(standard_seed.seed_hash, veteran_seed.seed_hash);
        assert_eq!(veteran_seed.difficulty, CampaignDifficulty::Veteran);

        let mut campaign = ready_campaign();
        campaign.quest_state = QuestState::Completed;
        campaign.progression.aftershock_completions = 1;
        campaign
            .progression
            .world_flags
            .extend(["first_contact_secured", "convoy_exodus_secured"].map(str::to_string));
        campaign.accept_first_contact_quest().unwrap();
        assert_eq!(campaign.active_mission, CampaignMission::MirrorSiege);
        assert_eq!(
            campaign.campaign_journal()[0].objective,
            "Break the siege and reclaim Mirror Gate"
        );
    }

    #[test]
    fn twenty_room_four_region_three_sect_world_and_regional_quest_route_are_live() {
        let graph = mirror_city_world_graph();
        assert_eq!(graph.rooms.len(), 20);
        assert_eq!(
            graph
                .rooms
                .values()
                .map(|room| room.region_id.as_str())
                .collect::<BTreeSet<_>>()
                .len(),
            4
        );
        let mut campaign = CampaignSaveV1::default();
        campaign.move_to(CampaignRoom::MarketWindPavilion).unwrap();
        campaign.talk_to_regional_npc().unwrap();
        assert_eq!(
            campaign.start_first_regional_quest_here().unwrap(),
            "market_debt"
        );
        campaign.advance_active_regional_quest().unwrap();
        assert_eq!(
            campaign.current_task_route_plan().destination_room_id,
            WORKSHOP_GATE_ROOM
        );
        campaign.move_to(CampaignRoom::MirrorSquare).unwrap();
        campaign.move_to(CampaignRoom::MentorHall).unwrap();
        campaign.move_to(CampaignRoom::WorkshopGate).unwrap();
        campaign.advance_active_regional_quest().unwrap();
        assert_eq!(
            campaign.regional_quest_states.get("market_debt"),
            Some(&QuestState::Completed)
        );
        campaign.join_regional_sect(SectId::IronWorkshop).unwrap();
        assert_eq!(
            campaign.character.sect_id.as_deref(),
            Some("iron_workshop_gate")
        );
        assert!(campaign.train_next_sect_skill().is_ok());
    }

    #[test]
    fn npc_hours_conversations_shop_browser_and_skirmish_setup_are_authoritative() {
        let mut campaign = CampaignSaveV1::default();
        campaign.move_to(CampaignRoom::MentorHall).unwrap();
        campaign.move_to(CampaignRoom::ArchiveSteps).unwrap();
        campaign.move_to(CampaignRoom::NightWatchPost).unwrap();
        assert!(campaign.talk_to_regional_npc().is_err());
        campaign.wait_in_town(4 * 60).unwrap();
        campaign.wait_in_town(4 * 60).unwrap();
        let conversation = campaign.talk_to_regional_npc().unwrap();
        assert_eq!(conversation.npc_id, "captain-veyra");
        assert!(conversation.activity.contains("night watch"));
        assert_eq!(campaign.conversation_history.len(), 1);

        campaign.move_to(CampaignRoom::ArchiveSteps).unwrap();
        campaign.move_to(CampaignRoom::MentorHall).unwrap();
        campaign.move_to(CampaignRoom::MirrorSquare).unwrap();
        campaign.move_to(CampaignRoom::MarketWindPavilion).unwrap();
        let first_item = campaign.selected_shop_item().id.to_string();
        let credits_before = campaign.progression.credits;
        campaign.buy_selected_shop_item().unwrap();
        assert!(campaign.progression.credits < credits_before);
        assert!(campaign
            .character
            .inventory_items
            .iter()
            .any(|item| item.item_id == first_item));
        assert_ne!(campaign.cycle_shop_item().unwrap(), first_item);
        campaign
            .progression
            .world_flags
            .insert("signal_road_secured".to_string());
        campaign.move_to(CampaignRoom::MirrorSquare).unwrap();
        campaign.move_to(CampaignRoom::RelayQuarter).unwrap();
        campaign.talk_to_regional_npc().unwrap();
        campaign.recruit_relay_smith().unwrap();
        assert!(campaign
            .party
            .iter()
            .any(|member| member.unit_id == "brann" && member.available));

        let mut skirmish = ready_campaign();
        skirmish
            .progression
            .world_flags
            .insert("mirror_siege_secured".to_string());
        skirmish.cycle_endgame_mission().unwrap();
        assert_eq!(
            skirmish.cycle_endgame_mission().unwrap(),
            CampaignMission::IronDeltaSkirmish
        );
        assert_eq!(
            skirmish.cycle_skirmish_faction().unwrap(),
            CampaignFaction::AshenCompact
        );
        assert_eq!(skirmish.cycle_skirmish_resources().unwrap(), 500);
        assert_eq!(
            skirmish.cycle_skirmish_victory_mode().unwrap(),
            SkirmishVictoryMode::Score
        );
        let seed = skirmish.start_first_contact_battle(map()).unwrap();
        assert!(seed.skirmish.enabled);
        assert_eq!(seed.skirmish.player_faction, CampaignFaction::AshenCompact);
        assert_eq!(seed.skirmish.starting_resources, 500);
        assert_eq!(seed.skirmish.victory_mode, SkirmishVictoryMode::Score);
        seed.validate().unwrap();
    }

    #[test]
    fn regional_quests_have_typed_approaches_deadlines_failure_recovery_and_market_resale() {
        let mut diplomatic = CampaignSaveV1::default();
        diplomatic
            .move_to(CampaignRoom::MarketWindPavilion)
            .unwrap();
        diplomatic.talk_to_regional_npc().unwrap();
        diplomatic.talk_to_regional_npc().unwrap();
        diplomatic.start_regional_quest("market_debt").unwrap();
        assert_eq!(
            diplomatic.cycle_regional_quest_approach().unwrap(),
            QuestApproach::Diplomatic
        );
        diplomatic.advance_active_regional_quest().unwrap();
        diplomatic.move_to(CampaignRoom::MirrorSquare).unwrap();
        diplomatic.move_to(CampaignRoom::MentorHall).unwrap();
        diplomatic.move_to(CampaignRoom::WorkshopGate).unwrap();
        diplomatic.advance_active_regional_quest().unwrap();
        assert_eq!(
            diplomatic.regional_quest_states.get("market_debt"),
            Some(&QuestState::Completed)
        );

        let mut expired = CampaignSaveV1::default();
        expired.move_to(CampaignRoom::MarketWindPavilion).unwrap();
        expired.talk_to_regional_npc().unwrap();
        expired.start_regional_quest("market_debt").unwrap();
        expired.world_clock.day += 2;
        expired.advance_active_regional_quest().unwrap();
        assert_eq!(
            expired.regional_quest_states.get("market_debt"),
            Some(&QuestState::Failed)
        );
        assert_eq!(expired.regional_quest_failure_counts["market_debt"], 1);
        expired.start_regional_quest("market_debt").unwrap();
        assert_eq!(
            expired
                .active_regional_quest_runtime
                .as_ref()
                .unwrap()
                .failure_count,
            1
        );

        let mut market = CampaignSaveV1::default();
        market.move_to(CampaignRoom::MarketWindPavilion).unwrap();
        while market.selected_shop_item().id != "salvaged-alloy" {
            market.cycle_shop_item().unwrap();
        }
        market.buy_selected_shop_item().unwrap();
        let credits_after_buy = market.progression.credits;
        market.sell_selected_shop_item().unwrap();
        assert!(market.progression.credits > credits_after_buy);
        assert!(market
            .progression
            .world_flags
            .iter()
            .any(|flag| flag.starts_with("market_sale_salvaged-alloy")));
    }

    fn test_room(room_id: &str) -> CampaignRoom {
        CampaignRoom::from_id(room_id).unwrap_or_else(|| panic!("unknown test room {room_id}"))
    }

    fn walk_to(campaign: &mut CampaignSaveV1, destination: &str) {
        while campaign.room.id() != destination {
            let mut flags = campaign.progression.world_flags.clone();
            if campaign.active_title == Some(BuildTitle::RelayRunner) {
                flags.insert("signal_road_secured".to_string());
            }
            let route =
                mirror_city_world_graph().shortest_route(campaign.room.id(), destination, &flags);
            assert!(
                route.reachable(),
                "real world route {} -> {} was blocked: {:?}",
                campaign.room.id(),
                destination,
                route.blocked_reason,
            );
            let next = route.path.get(1).expect("route must advance");
            campaign.move_to(test_room(next)).unwrap();
        }
    }

    fn finish_authored_quest_on(
        campaign: &mut CampaignSaveV1,
        quest_id: &str,
        approach: QuestApproach,
    ) {
        if !campaign.mentor_met {
            walk_to(campaign, MENTOR_HALL_ROOM);
            campaign.talk_to_mentor().unwrap();
            // The authored-quest matrix begins after the three-mission
            // campaign prologue. These prerequisite flags are fixture scope;
            // room traversal and quest/encounter state may not be mutated.
            campaign
                .progression
                .world_flags
                .extend(["signal_road_secured", "outer_signal_road_open"].map(str::to_string));
        }
        let definition = REGIONAL_QUEST_CATALOG
            .iter()
            .find(|quest| quest.id == quest_id)
            .unwrap();
        let giver = NPC_CATALOG
            .iter()
            .find(|npc| npc.id == definition.giver_npc_id)
            .unwrap();
        walk_to(campaign, giver.room_id);
        let rule = quest_runtime_rule(definition.archetype);
        let required_trust = if approach == QuestApproach::Diplomatic {
            rule.minimum_trust_for_diplomacy
        } else {
            1
        };
        for _ in 0..96 {
            let relationship = campaign.npc_relationships.get(giver.id).unwrap();
            if relationship.interactions > 0 && relationship.trust >= required_trust {
                break;
            }
            if campaign
                .current_regional_npc()
                .is_some_and(|npc| npc.id == giver.id)
            {
                campaign.talk_to_regional_npc().unwrap();
            } else {
                campaign.wait_in_town(120).unwrap();
            }
        }
        let relationship = campaign.npc_relationships.get(giver.id).unwrap();
        assert!(relationship.interactions > 0);
        assert!(relationship.trust >= required_trust);
        if approach == QuestApproach::Resourceful {
            for _ in 0..rule.resource_quantity {
                let mut purchased = false;
                for market_room in [
                    MARKET_WIND_PAVILION_ROOM,
                    RELAY_QUARTER_ROOM,
                    GLASS_BASIN_WAYHOUSE_ROOM,
                    CINDER_REFUGE_ROOM,
                ] {
                    walk_to(campaign, market_room);
                    if campaign.buy_regional_item(rule.resource_item_id).is_ok() {
                        purchased = true;
                        break;
                    }
                }
                assert!(
                    purchased,
                    "regional markets could not supply {} for {quest_id}",
                    rule.resource_item_id
                );
            }
            walk_to(campaign, giver.room_id);
        }
        campaign.start_regional_quest(quest_id).unwrap();
        let deadline = campaign
            .active_regional_quest_runtime
            .as_ref()
            .unwrap()
            .deadline_day;
        while campaign.world_clock.day <= deadline {
            campaign.wait_in_town(120).unwrap();
        }
        campaign.advance_active_regional_quest().unwrap();
        assert_eq!(
            campaign.regional_quest_states.get(quest_id),
            Some(&QuestState::Failed),
            "{quest_id} must persist a deadline failure before its retry path",
        );
        campaign.start_regional_quest(quest_id).unwrap();
        while campaign
            .active_regional_quest_runtime
            .as_ref()
            .is_some_and(|runtime| runtime.approach != approach)
        {
            campaign.cycle_regional_quest_approach().unwrap();
        }
        while let Some(waypoint) = campaign
            .active_regional_quest_ready_rooms()
            .into_iter()
            .last()
        {
            walk_to(campaign, &waypoint);
            campaign.advance_active_regional_quest().unwrap();
        }
        if let (QuestApproach::Direct, Some(encounter_id)) = (approach, definition.encounter_id) {
            if !campaign
                .progression
                .world_flags
                .contains(&format!("{encounter_id}_cleared"))
            {
                campaign.advance_active_regional_quest().unwrap();
                while campaign.active_encounter.is_some() {
                    let encounter = campaign.active_encounter.as_ref().unwrap();
                    let action = if encounter.technique_cooldown == 0 && encounter.momentum >= 2 {
                        if encounter.round.is_multiple_of(2) {
                            EncounterAction::PrimaryTechnique
                        } else {
                            EncounterAction::SecondaryTechnique
                        }
                    } else if encounter.momentum < 2 {
                        EncounterAction::Defend
                    } else {
                        EncounterAction::Attack
                    };
                    campaign.act_in_signal_road_encounter(action).unwrap();
                }
                assert_eq!(
                    campaign.last_encounter_outcome,
                    Some(EncounterOutcome::Victory)
                );
            }
        }
        if campaign.active_regional_quest_id.is_some() {
            let graph = quest_condition_graph(definition, approach);
            let settlement = graph
                .nodes
                .iter()
                .find(|node| node.kind == trnm_rpg_core::QuestConditionKind::ReturnForSettlement)
                .unwrap();
            walk_to(campaign, &settlement.subject_id);
            campaign.complete_regional_quest().unwrap();
        }
        if let Some(pending) = campaign.pending_main_story_chapter {
            let chapter = MAIN_STORY_CHAPTERS
                .iter()
                .find(|chapter| chapter.chapter == pending)
                .unwrap();
            walk_to(campaign, chapter.room_id);
            campaign.resolve_pending_main_story_chapter().unwrap();
        }
    }

    #[test]
    fn all_fifteen_authored_quests_complete_through_all_three_branches() {
        for approach in [
            QuestApproach::Direct,
            QuestApproach::Diplomatic,
            QuestApproach::Resourceful,
        ] {
            let mut campaign = CampaignSaveV1::default();
            for quest in REGIONAL_QUEST_CATALOG {
                finish_authored_quest_on(&mut campaign, quest.id, approach);
                assert_eq!(
                    campaign.regional_quest_states.get(quest.id),
                    Some(&QuestState::Completed),
                    "{} {approach:?} did not reach its terminal branch",
                    quest.id
                );
                assert!(campaign.progression.world_flags.contains(
                    &format!("regional_quest_{}_{approach:?}", quest.id).to_ascii_lowercase()
                ));
            }
        }
    }

    #[test]
    fn authored_forks_change_the_live_route_and_accept_either_ready_branch_first() {
        let mut campaign = CampaignSaveV1::default();
        campaign
            .progression
            .world_flags
            .extend(["signal_road_secured", "outer_signal_road_open"].map(str::to_string));
        let definition = REGIONAL_QUEST_CATALOG
            .iter()
            .find(|quest| quest.id == "broken_milestone")
            .unwrap();
        let giver = NPC_CATALOG
            .iter()
            .find(|npc| npc.id == definition.giver_npc_id)
            .unwrap();
        walk_to(&mut campaign, giver.room_id);
        campaign.talk_to_regional_npc().unwrap();
        campaign.start_regional_quest(definition.id).unwrap();
        let first_ready = campaign.active_regional_quest_ready_rooms();
        assert!(
            first_ready.len() >= 2,
            "the authored fork must expose route choice"
        );
        walk_to(&mut campaign, first_ready.last().unwrap());
        campaign.advance_active_regional_quest().unwrap();
        let after_branch = campaign.active_regional_quest_ready_rooms();
        assert_ne!(after_branch, first_ready);
        assert!(
            after_branch.is_empty(),
            "the mutually exclusive sibling must close"
        );
        assert!(campaign
            .progression
            .world_flags
            .iter()
            .any(|flag| flag.starts_with("quest_route_broken_milestone_")));
        let route = campaign.current_task_route_plan();
        assert_eq!(route.destination_room_id, definition.waypoint_room_ids[1]);
    }

    #[test]
    fn each_main_story_chapter_records_an_independent_irreversible_choice() {
        let mut campaign = CampaignSaveV1::default();
        let choices = [
            MainStoryChoice::ProtectWayhouses,
            MainStoryChoice::ExposeConspiracy,
            MainStoryChoice::ForgeAccord,
        ];
        for (chapter_index, choice) in choices.into_iter().enumerate() {
            let threshold = (chapter_index + 1) * 5;
            campaign.main_story_choice = choice;
            let start = chapter_index * 5;
            for quest in REGIONAL_QUEST_CATALOG.iter().skip(start).take(5) {
                finish_authored_quest_on(&mut campaign, quest.id, QuestApproach::Direct);
            }
            assert_eq!(campaign.main_story_decisions.len(), chapter_index + 1);
            assert_eq!(campaign.main_story_decisions.last().unwrap().choice, choice);
            assert_eq!(
                campaign
                    .regional_quest_states
                    .values()
                    .filter(|state| **state == QuestState::Completed)
                    .count(),
                threshold
            );
        }
        assert_eq!(campaign.main_story_decisions.len(), 3);
        assert_eq!(
            campaign
                .main_story_decisions
                .iter()
                .map(|decision| decision.choice)
                .collect::<Vec<_>>(),
            choices
        );
        assert_eq!(
            campaign
                .main_story_decisions
                .iter()
                .map(|decision| decision.chapter)
                .collect::<BTreeSet<_>>()
                .len(),
            3
        );
        assert_eq!(
            campaign.main_story_ending,
            Some(MainStoryEnding::ThreeRoadCompact)
        );
        for chapter in MAIN_STORY_CHAPTERS {
            assert!(campaign
                .progression
                .world_flags
                .contains(&format!("main_story_scene_{}_resolved", chapter.scene_id)));
        }
    }

    #[test]
    fn all_five_explicit_main_story_endings_are_resolved() {
        let decisions = |choices: [MainStoryChoice; 3]| {
            MAIN_STORY_CHAPTERS
                .iter()
                .zip(choices)
                .map(|(chapter, choice)| MainStoryDecisionRecord {
                    chapter: chapter.chapter,
                    choice,
                    outcome_flag: format!("test_{:?}_{choice:?}", chapter.chapter),
                    day: 1,
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(
            resolve_main_story_ending(&decisions([MainStoryChoice::ProtectWayhouses; 3])),
            Some(MainStoryEnding::WayhouseLeague)
        );
        assert_eq!(
            resolve_main_story_ending(&decisions([MainStoryChoice::ExposeConspiracy; 3])),
            Some(MainStoryEnding::OpenArchiveRepublic)
        );
        assert_eq!(
            resolve_main_story_ending(&decisions([MainStoryChoice::ForgeAccord; 3])),
            Some(MainStoryEnding::FrontierAccord)
        );
        assert_eq!(
            resolve_main_story_ending(&decisions([
                MainStoryChoice::ProtectWayhouses,
                MainStoryChoice::ExposeConspiracy,
                MainStoryChoice::ForgeAccord,
            ])),
            Some(MainStoryEnding::ThreeRoadCompact)
        );
        assert_eq!(
            resolve_main_story_ending(&decisions([
                MainStoryChoice::ProtectWayhouses,
                MainStoryChoice::ProtectWayhouses,
                MainStoryChoice::ForgeAccord,
            ])),
            Some(MainStoryEnding::ContestedMandate)
        );
    }

    #[test]
    fn sect_skill_tree_changes_battle_seed_and_crafting_consumes_materials() {
        let mut baseline = ready_campaign();
        let baseline_seed = baseline.start_first_contact_battle(map()).unwrap();
        let mut artisan = ready_campaign();
        artisan.move_to(CampaignRoom::MentorHall).unwrap();
        artisan.move_to(CampaignRoom::WorkshopGate).unwrap();
        artisan.join_regional_sect(SectId::IronWorkshop).unwrap();
        artisan.progression.inventory.extend([
            LootStack {
                item_id: "salvaged-alloy".to_string(),
                quantity: 3,
            },
            LootStack {
                item_id: "route-token".to_string(),
                quantity: 1,
            },
        ]);
        artisan.craft_regional_item("reinforced_staff").unwrap();
        assert!(artisan
            .character
            .inventory_items
            .iter()
            .any(|item| item.item_id == "reinforced-staff"));
        artisan.move_to(CampaignRoom::MentorHall).unwrap();
        artisan.move_to(CampaignRoom::ExpeditionGate).unwrap();
        let artisan_seed = artisan.start_first_contact_battle(map()).unwrap();
        assert_ne!(baseline_seed.seed_hash, artisan_seed.seed_hash);
        assert_eq!(artisan_seed.sect_id.as_deref(), Some("iron_workshop_gate"));
        assert!(artisan_seed.party[0].stats.armor > baseline_seed.party[0].stats.armor);
    }

    #[test]
    fn sect_technique_mastery_persists_and_changes_later_encounter_authority() {
        let mut campaign = CampaignSaveV1::default();
        campaign.character.sect_id = Some("iron_workshop_gate".to_string());
        campaign.equipped_technique_slot = 1;
        campaign
            .technique_mastery
            .insert("relay_hammer".to_string(), 50);
        campaign.room = CampaignRoom::RelayQuarter;
        campaign.begin_signal_road_encounter().unwrap();
        assert_eq!(
            campaign.active_encounter.as_ref().unwrap().technique_rank,
            5
        );
        campaign.active_encounter.as_mut().unwrap().momentum = 3;
        campaign
            .act_in_signal_road_encounter(EncounterAction::Technique)
            .unwrap();
        assert_eq!(campaign.technique_mastery["relay_hammer"], 51);
        let directory = tempdir().unwrap();
        let store = CampaignStore::new(directory.path().join("mastery.json"));
        store.save_atomic(&campaign).unwrap();
        assert_eq!(store.load().unwrap().technique_mastery["relay_hammer"], 51);
    }

    #[test]
    fn primary_secondary_techniques_chain_and_regional_logistics_persist() {
        let mut campaign = CampaignSaveV1::default();
        campaign.character.sect_id = Some("iron_workshop_gate".to_string());
        campaign.equipped_technique_slot = 0;
        campaign.secondary_technique_slot = 1;
        campaign.room = CampaignRoom::RelayQuarter;
        campaign.begin_signal_road_encounter().unwrap();
        campaign.active_encounter.as_mut().unwrap().momentum = 8;
        campaign
            .act_in_signal_road_encounter(EncounterAction::Technique)
            .unwrap();
        campaign
            .act_in_signal_road_encounter(EncounterAction::Defend)
            .unwrap();
        campaign
            .act_in_signal_road_encounter(EncounterAction::Defend)
            .unwrap();
        campaign.active_encounter.as_mut().unwrap().momentum = 8;
        campaign
            .act_in_signal_road_encounter(EncounterAction::Technique)
            .unwrap();
        assert_eq!(campaign.technique_mastery["forge_counter"], 1);
        assert_eq!(campaign.technique_mastery["relay_hammer"], 1);
        campaign.active_encounter = None;
        let before = campaign.regional_market_stock.clone();
        campaign.wait_in_town(120).unwrap();
        assert!(!campaign.active_regional_caravans.is_empty());
        for _ in 0..8 {
            campaign.wait_in_town(120).unwrap();
            if !campaign.regional_logistics.is_empty() {
                break;
            }
        }
        assert!(!campaign.regional_logistics.is_empty());
        assert_ne!(campaign.regional_market_stock, before);
        assert!(campaign.npc_work_output.values().any(|output| *output > 0));
        let directory = tempdir().unwrap();
        let store = CampaignStore::new(directory.path().join("regional-economy.json"));
        store.save_atomic(&campaign).unwrap();
        let loaded = store.load().unwrap();
        assert_eq!(loaded.schema_revision, 10);
        assert_eq!(loaded.regional_logistics, campaign.regional_logistics);
        assert_eq!(loaded.technique_mastery, campaign.technique_mastery);
    }

    #[test]
    fn chapter_scenes_require_three_player_beats_and_caravans_exist_in_rooms() {
        let mut campaign = CampaignSaveV1 {
            pending_main_story_chapter: Some(MainStoryChapter::MirrorCityOaths),
            room: CampaignRoom::MirrorSquare,
            ..CampaignSaveV1::default()
        };
        for expected_step in [1, 2] {
            let advance = campaign.advance_pending_main_story_scene().unwrap();
            assert!(matches!(
                advance,
                MainStorySceneAdvance::SceneBeat { step, .. } if step == expected_step
            ));
            assert!(campaign.main_story_decisions.is_empty());
        }
        assert!(matches!(
            campaign.advance_pending_main_story_scene().unwrap(),
            MainStorySceneAdvance::ChapterResolved(_)
        ));
        assert_eq!(campaign.main_story_decisions.len(), 1);

        campaign.wait_in_town(120).unwrap();
        let room_id = campaign.active_regional_caravans[0]
            .current_room_id()
            .unwrap()
            .to_string();
        campaign.room = CampaignRoom::from_id(&room_id).unwrap();
        let caravan_id = campaign.active_regional_caravans[0].caravan_id.clone();
        campaign.interact_with_visible_caravan(true).unwrap();
        let caravan = campaign
            .active_regional_caravans
            .iter()
            .find(|caravan| caravan.caravan_id == caravan_id)
            .unwrap();
        assert!(caravan.guarded_by_player);
        assert_eq!(caravan.incident.as_deref(), Some("player_escort"));
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum FaultBackend {
        Recoverable,
        CorruptReceipt,
    }

    impl EconomyBackend for FaultBackend {
        fn backend_id(&self) -> &str {
            "fault-backend"
        }

        fn execute(&self, intent: &EconomicIntent) -> Result<EconomicReceipt, String> {
            let mut receipt = EconomicReceipt::from_intent(
                format!("fault:{}", intent.intent_id),
                intent,
                "fault-backend",
                SettlementBackendKind::LocalTest,
                ReceiptStatus::FailedNetwork,
                intent.created_at_epoch,
            );
            receipt.reason = Some("simulated transport outage".to_string());
            if *self == Self::CorruptReceipt {
                receipt.intent_id = "wrong-intent".to_string();
            }
            Ok(receipt)
        }
    }

    #[test]
    fn revision_ten_separates_soft_wallet_bound_tradeable_and_ephemeral_assets() {
        let campaign = CampaignSaveV1::default();
        assert_eq!(campaign.schema_revision, 10);
        assert_eq!(campaign.economy_mode, EconomyMode::OfflineLocal);
        assert_eq!(
            CampaignSaveV1::economy_asset_semantic("trnm-soft-credit").transferability,
            EconomyTransferability::Bound
        );
        assert_eq!(
            CampaignSaveV1::economy_asset_semantic("salvaged-alloy").transferability,
            EconomyTransferability::Tradeable
        );
        assert_eq!(
            CampaignSaveV1::economy_asset_semantic("rts-resource:cyan").transferability,
            EconomyTransferability::Ephemeral
        );
    }

    #[test]
    fn economy_outbox_is_exactly_once_and_fail_closed_across_reload() {
        let mut campaign = CampaignSaveV1::default();
        campaign
            .bind_cex_economy_account("player-one", "account-one")
            .unwrap();
        let receipt = SettlementReceiptV1 {
            contract_version: SETTLEMENT_RECEIPT_CONTRACT.to_string(),
            battle_id: "battle-economy-1".to_string(),
            seed_hash: "seed".to_string(),
            result_hash: "result".to_string(),
            campaign_revision_before: 1,
            campaign_revision_after: 2,
            outcome: BattleOutcome::Victory,
            experience_delta: 10,
            reputation_delta: 1,
            credit_delta: 80,
            loot_delta: Vec::new(),
            injury_delta_by_unit: BTreeMap::new(),
            economic_intent_id: Some("battle-reward:battle-economy-1".to_string()),
            economic_receipt_id: None,
            duplicate: false,
        };
        campaign.queue_battle_reward_economy(&receipt).unwrap();
        campaign.queue_battle_reward_economy(&receipt).unwrap();
        assert_eq!(campaign.pending_economic_intents.len(), 1);
        let report = campaign
            .reconcile_economy(&FaultBackend::Recoverable, 4)
            .unwrap();
        assert_eq!(report.recoverable_holds, 1);
        assert_eq!(campaign.pending_economic_intents.len(), 1);
        assert_eq!(campaign.wallet_snapshot.available_credits, 0);

        let directory = tempdir().unwrap();
        let store = CampaignStore::new(directory.path().join("economy.json"));
        store.save_atomic(&campaign).unwrap();
        let mut loaded = store.load().unwrap();
        assert_eq!(loaded.pending_economic_intents.len(), 1);
        let report = loaded
            .reconcile_economy(&OfflineLocalEconomyBackend, 4)
            .unwrap();
        assert_eq!(report.applied, 1);
        assert_eq!(loaded.wallet_snapshot.available_credits, 80);
        assert!(loaded.pending_economic_intents.is_empty());
        assert_eq!(loaded.verified_economic_receipts.len(), 2);
        loaded
            .queue_battle_reward_economy(&receipt)
            .expect("duplicate queue is a no-op");
        assert!(loaded.pending_economic_intents.is_empty());
    }

    #[test]
    fn tradeable_purchase_runs_reserve_settle_consume_and_corrupt_receipts_dead_letter() {
        let selected_shop_item_index = ECONOMY_ITEM_CATALOG
            .iter()
            .position(|item| item.material)
            .unwrap();
        let mut campaign = CampaignSaveV1 {
            room: CampaignRoom::MarketWindPavilion,
            selected_shop_item_index,
            ..CampaignSaveV1::default()
        };
        let item_id = ECONOMY_ITEM_CATALOG[campaign.selected_shop_item_index]
            .id
            .to_string();
        campaign.begin_selected_tradeable_purchase().unwrap();
        let report = campaign
            .reconcile_economy(&OfflineLocalEconomyBackend, 8)
            .unwrap();
        assert_eq!(report.applied, 3);
        assert_eq!(
            campaign.pending_tradeable_purchases[0].stage,
            TradeablePurchaseStage::Consumed
        );
        assert!(campaign
            .progression
            .inventory
            .iter()
            .any(|loot| loot.item_id == item_id));

        let binding = campaign.effective_economy_binding();
        campaign
            .queue_economic_intent(EconomicIntentDraft {
                kind: EconomicIntentKind::ReleaseReward,
                term_id: "corrupt_test".to_string(),
                intent_id: "corrupt-test-intent".to_string(),
                binding,
                asset_id: "cex-wallet-credit".to_string(),
                quantity: 1,
                amount_credits: 1,
                metadata: json!({}),
            })
            .unwrap();
        let report = campaign
            .reconcile_economy(&FaultBackend::CorruptReceipt, 1)
            .unwrap();
        assert_eq!(report.hard_failures, 1);
        assert_eq!(campaign.economic_dead_letters.len(), 1);
    }

    #[test]
    fn connected_trade_requires_an_explicit_cex_market_account() {
        let selected_shop_item_index = ECONOMY_ITEM_CATALOG
            .iter()
            .position(|item| item.material)
            .unwrap();
        let mut campaign = CampaignSaveV1 {
            room: CampaignRoom::MarketWindPavilion,
            selected_shop_item_index,
            ..CampaignSaveV1::default()
        };
        campaign
            .bind_cex_economy_account("player-one", "11111111-1111-1111-1111-111111111111")
            .unwrap();
        assert!(campaign.begin_selected_tradeable_purchase().is_err());
        campaign
            .begin_selected_tradeable_purchase_with_seller_account(Some(
                "22222222-2222-2222-2222-222222222222",
            ))
            .unwrap();
        assert_eq!(
            campaign.pending_tradeable_purchases[0].seller.account_id,
            "22222222-2222-2222-2222-222222222222"
        );
    }

    #[test]
    fn v1_settings_migrate_to_subtitles_controls_and_master_volume_preference() {
        let directory = tempdir().unwrap();
        let path = directory.path().join("settings.json");
        fs::write(
            &path,
            br#"{"contract_version":"trnm_player_settings_v1","low_motion":true,"input_mode":"keyboard_only"}"#,
        )
        .unwrap();
        let settings = PlayerSettingsStore::new(path).load_or_default().unwrap();
        assert_eq!(settings.contract_version, PLAYER_SETTINGS_CONTRACT);
        assert!(settings.subtitles);
        assert_eq!(settings.master_volume_percent, 80);
        assert_eq!(settings.control_scheme, ControlScheme::Classic);
    }
}
