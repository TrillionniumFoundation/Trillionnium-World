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
    #[serde(default)]
    pub inventory_rolled_back: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValueEventSource {
    RegionalQuest,
    Chapter,
    Ending,
    Battle,
    PlayerTrade,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValueSettlementPolicy {
    LocalSoftOnly,
    WalletOnly,
    DualTrack,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValueEventRecord {
    pub event_id: String,
    pub source: ValueEventSource,
    pub policy: ValueSettlementPolicy,
    pub local_soft_credit_delta: i64,
    pub wallet_credit_delta: i64,
    pub economic_intent_id: String,
    #[serde(default)]
    pub economic_receipt_id: Option<String>,
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
    compensation: bool,
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

