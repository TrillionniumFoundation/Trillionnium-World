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

