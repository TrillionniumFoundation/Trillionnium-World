//! Deterministic, Bevy-free First Contact battle simulation.
//!
//! The simulation consumes validated [`RtsFrameOrder`] values as its only
//! player input. The authored map projection embedded in [`BattleSeedV1`]
//! drives two-dimensional pathfinding, combat, resources and objectives.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fmt, fs,
    io::Write,
    path::{Path, PathBuf},
};
use trnm_campaign_core::{
    BattleGridPoint, BattleOutcome, BattleResultV1, BattleSeedV1, CampaignDifficulty,
    CampaignError, LootStack, ObjectiveKind, SkirmishVictoryMode, UnitBattleReportV1,
    UnitBattleStatus, BATTLE_RESULT_CONTRACT,
};
use trnm_rts_protocol::{RtsFrameOrder, RtsOrderKind, RtsUnitStance};

mod content;
pub use content::*;

pub const RTS_SIM_CONTRACT: &str = "trnm_rts_sim_v13";
pub const RTS_SIM_CHECKPOINT_CONTRACT: &str = "trnm_rts_sim_checkpoint_v13";
pub const TICKS_PER_SECOND: u64 = 10;
pub const THREE_MINUTE_TICKS: u64 = 3 * 60 * TICKS_PER_SECOND;
pub const FIVE_MINUTE_TICKS: u64 = 5 * 60 * TICKS_PER_SECOND;
pub const TEN_MINUTE_TICKS: u64 = 10 * 60 * TICKS_PER_SECOND;
pub const FIFTEEN_MINUTE_TICKS: u64 = 15 * 60 * TICKS_PER_SECOND;
const MOVEMENT_TILE_COST: i32 = 10_000;
const CAPTURE_TICKS_REQUIRED: u32 = 602;
const RELAY_GUARD_HP: i64 = 5_400;
const WITHDRAWAL_MIN_TICKS: u64 = 30;
const FIELD_AID_COST: u32 = 20;
const RECON_COST: u32 = 10;
const TRAIN_SUPPORT_COST: u32 = 40;
const WORKER_CARGO_CAPACITY: u32 = 40;
const RESOURCE_NODE_CAPACITY: u32 = 800;
const MAX_REPLAY_ORDERS: usize = 4_096;

#[derive(Debug)]
pub enum SimError {
    Campaign(CampaignError),
    Order(String),
    InvalidState(String),
    Integrity(String),
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl fmt::Display for SimError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Campaign(error) => write!(formatter, "campaign contract rejected: {error}"),
            Self::Order(message) => write!(formatter, "RTS order rejected: {message}"),
            Self::InvalidState(message) => write!(formatter, "invalid RTS state: {message}"),
            Self::Integrity(message) => write!(formatter, "RTS integrity error: {message}"),
            Self::Io(error) => write!(formatter, "RTS checkpoint storage error: {error}"),
            Self::Json(error) => write!(formatter, "RTS checkpoint JSON error: {error}"),
        }
    }
}

impl std::error::Error for SimError {}

impl From<CampaignError> for SimError {
    fn from(error: CampaignError) -> Self {
        Self::Campaign(error)
    }
}

impl From<std::io::Error> for SimError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for SimError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BattlePhase {
    #[default]
    Approach,
    Contact,
    Relay,
    ConvoyEscort,
    GeneratorDefense,
    Extraction,
    Complete,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimUnit {
    pub unit_id: String,
    pub role: String,
    pub persistent: bool,
    pub skill_ids: Vec<String>,
    pub max_hp: i64,
    pub hp: i64,
    pub damage: i64,
    pub armor: i64,
    pub move_speed_milli: i32,
    pub movement_budget_milli: i32,
    pub attack_interval_ticks: u32,
    pub evasion_permille: u16,
    pub energy: i64,
    pub max_energy: i64,
    pub ability_range: i16,
    pub ability_cooldown_ticks: u32,
    pub guard_ticks: u32,
    pub position: BattleGridPoint,
    pub attacks_made: u64,
    #[serde(default)]
    pub stance: RtsUnitStance,
    #[serde(default)]
    pub patrol_anchor: Option<BattleGridPoint>,
    #[serde(default)]
    pub patrol_target: Option<BattleGridPoint>,
    #[serde(default)]
    pub patrol_returning: bool,
    #[serde(default)]
    pub cargo: u32,
    #[serde(default = "default_worker_cargo_capacity")]
    pub cargo_capacity: u32,
    #[serde(default)]
    pub confirmed_kills: u32,
    #[serde(default)]
    pub veteran_rank: u8,
}

fn default_worker_cargo_capacity() -> u32 {
    WORKER_CARGO_CAPACITY
}

impl SimUnit {
    pub fn alive(&self) -> bool {
        self.hp > 0
    }

    fn attack_range(&self) -> i16 {
        match self.role.as_str() {
            "scout" | "engineer" | "mystic" | "recon" | "raider" | "disruptor" => 3,
            "medic" | "support" | "siege" => 2,
            _ => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SimJobKind {
    TrainSupport,
    TrainMedic,
    TrainRosterUnit,
    ResearchLogistics,
    ResearchOptics,
    UpgradeRelayArms,
    UpgradeFieldArmor,
    ResearchSensorNet,
    ResearchFieldMedicine,
    UpgradeSiegeDrills,
    UpgradeReactivePlating,
    ResearchWayfinderDrills,
    ResearchRapidMustering,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimJob {
    pub job_id: String,
    pub kind: SimJobKind,
    pub rule_id: String,
    pub remaining_ticks: u32,
    pub target: BattleGridPoint,
    pub cost: u32,
    pub paused: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SimStructureKind {
    CommandPost,
    FieldWorkshop,
    RelayGenerator,
    SupplyCache,
    FieldBarricade,
    SensorTower,
    FieldHospital,
    SiegeFoundry,
    AshBeacon,
    ForwardRally,
}

impl SimStructureKind {
    fn from_rule_id(rule: &str) -> Option<Self> {
        Some(match rule {
            "command_post" => Self::CommandPost,
            "field_workshop" => Self::FieldWorkshop,
            "relay_generator" => Self::RelayGenerator,
            "supply_cache" => Self::SupplyCache,
            "field_barricade" => Self::FieldBarricade,
            "sensor_tower" => Self::SensorTower,
            "field_hospital" => Self::FieldHospital,
            "siege_foundry" => Self::SiegeFoundry,
            "ash_beacon" => Self::AshBeacon,
            "forward_rally" => Self::ForwardRally,
            _ => return None,
        })
    }

    fn rule_id(self) -> &'static str {
        match self {
            Self::CommandPost => "command_post",
            Self::FieldWorkshop => "field_workshop",
            Self::RelayGenerator => "relay_generator",
            Self::SupplyCache => "supply_cache",
            Self::FieldBarricade => "field_barricade",
            Self::SensorTower => "sensor_tower",
            Self::FieldHospital => "field_hospital",
            Self::SiegeFoundry => "siege_foundry",
            Self::AshBeacon => "ash_beacon",
            Self::ForwardRally => "forward_rally",
        }
    }

    fn definition(self) -> &'static StructureArchetype {
        STRUCTURE_ROSTER
            .iter()
            .find(|definition| definition.id == self.rule_id())
            .expect("every authoritative structure kind is catalogued")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimStructure {
    pub structure_id: String,
    pub kind: SimStructureKind,
    pub position: BattleGridPoint,
    pub hp: i64,
    pub max_hp: i64,
}

impl SimStructure {
    fn alive(&self) -> bool {
        self.hp > 0
    }

    fn supply_provided(&self) -> u8 {
        self.kind.definition().supply_delta.max(0) as u8
    }

    fn power_provided(&self) -> u16 {
        self.kind.definition().power_delta.max(0) as u16
    }

    fn power_draw(&self) -> u16 {
        self.kind.definition().power_delta.min(0).unsigned_abs()
    }
}

enum SimTargetRef<'a> {
    Unit(&'a SimUnit),
    Structure(&'a SimStructure),
}

impl SimTargetRef<'_> {
    fn destroyed(self) -> bool {
        match self {
            Self::Unit(unit) => !unit.alive(),
            Self::Structure(structure) => !structure.alive(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceNodeState {
    pub node_id: String,
    pub position: BattleGridPoint,
    pub remaining: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportUnit {
    pub unit_id: String,
    #[serde(default)]
    pub archetype_id: String,
    #[serde(default = "default_support_role")]
    pub role: String,
    pub position: BattleGridPoint,
    pub hp: i64,
    pub damage: i64,
    #[serde(default)]
    pub armor: i64,
    #[serde(default = "default_support_range")]
    pub attack_range: i16,
    #[serde(default)]
    pub ability_cooldown_ticks: u32,
    pub attack_interval_ticks: u32,
    #[serde(default = "default_support_supply")]
    pub supply: u8,
}

fn default_support_range() -> i16 {
    4
}

fn default_support_supply() -> u8 {
    1
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MoveIntent {
    pub unit_id: String,
    pub target: BattleGridPoint,
    pub desired_tile: Option<BattleGridPoint>,
    pub blocked_ticks: u16,
    pub replan_count: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TileReservation {
    pub tile: BattleGridPoint,
    pub unit_id: String,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiGoal {
    #[default]
    Scout,
    RaidEconomy,
    CounterTech,
    DefendObjective,
    InterdictConvoy,
    Assault,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiObservation {
    pub tick: u64,
    pub phase: BattlePhase,
    pub living_party: u8,
    pub living_enemies: u8,
    pub wounded_party: u8,
    pub party_resources: u32,
    pub party_structures: u8,
    pub researched_tech_count: u8,
    pub convoy_active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiDecision {
    pub index: u32,
    pub goal: AiGoal,
    pub budget_before: u16,
    pub budget_after: u16,
    pub reason: String,
    pub observation: AiObservation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnemyJobKind {
    Build,
    Train,
    Research,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnemyJob {
    pub kind: EnemyJobKind,
    pub rule_id: String,
    pub remaining_ticks: u32,
    pub cost: u32,
    #[serde(default)]
    pub builder_id: Option<String>,
    #[serde(default)]
    pub target: Option<BattleGridPoint>,
    #[serde(default)]
    pub total_ticks: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimReplayEntry {
    pub issued_tick: u64,
    pub order: RtsFrameOrder,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BattleReplayV1 {
    pub contract_version: String,
    pub seed: BattleSeedV1,
    pub entries: Vec<SimReplayEntry>,
    pub final_tick: u64,
    pub final_snapshot_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkirmishBalanceSample {
    pub map_id: String,
    pub player_faction: RtsFaction,
    pub enemy_faction: RtsFaction,
    pub seed_hash: String,
    pub map_fingerprint: String,
    pub simulation_salt: u64,
    pub final_tick: u64,
    pub outcome: Option<BattleOutcome>,
    pub player_score: u32,
    pub enemy_score: u32,
    pub player_hp_percent: u8,
    pub enemy_hp_percent: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkirmishBalanceMatrix {
    pub contract_version: String,
    pub samples: Vec<SkirmishBalanceSample>,
    pub faction_pressure_delta_permille: u16,
}

pub fn run_skirmish_balance_matrix(
    seeds: &[BattleSeedV1],
    max_ticks: u64,
) -> Result<SkirmishBalanceMatrix, SimError> {
    if seeds.is_empty() || max_ticks < 100 {
        return Err(SimError::InvalidState(
            "balance matrix requires skirmish seeds and at least 100 ticks".to_string(),
        ));
    }
    let mut samples = Vec::with_capacity(seeds.len());
    let mut pressure = [0_u64; 2];
    let mut map_fingerprints = BTreeMap::<String, String>::new();
    for seed in seeds {
        if !seed.skirmish.enabled {
            return Err(SimError::InvalidState(
                "balance matrix accepts only configured skirmish seeds".to_string(),
            ));
        }
        let map_fingerprint = hash_json(&seed.map)?;
        if let Some(existing) =
            map_fingerprints.insert(seed.map_id.clone(), map_fingerprint.clone())
        {
            if existing != map_fingerprint {
                return Err(SimError::Integrity(format!(
                    "map {} changed geometry inside one balance matrix",
                    seed.map_id
                )));
            }
        }
        let mut sim = MissionSimV1::from_seed(seed.clone())?;
        while !sim.terminal() && sim.tick < max_ticks {
            if sim.tick.is_multiple_of(20) || sim.active_order.is_none() {
                let target = sim
                    .enemies
                    .iter()
                    .find(|enemy| enemy.alive() && sim.is_enemy_visible(&enemy.unit_id))
                    .map(|enemy| (enemy.unit_id.clone(), enemy.position))
                    .or_else(|| {
                        sim.enemy_structures
                            .iter()
                            .find(|structure| {
                                structure.alive() && sim.visible_tiles.contains(&structure.position)
                            })
                            .map(|structure| (structure.structure_id.clone(), structure.position))
                    });
                let subjects = sim
                    .party
                    .iter()
                    .filter(|unit| unit.alive())
                    .map(|unit| unit.unit_id.clone())
                    .collect::<Vec<_>>();
                if !subjects.is_empty() {
                    let (kind, actor_id, position) = target
                        .map(|(id, position)| (RtsOrderKind::Attack, Some(id), position))
                        .unwrap_or((RtsOrderKind::AttackMove, None, sim.seed.map.objective));
                    let mut order = RtsFrameOrder::new(
                        sim.tick as u32,
                        "player",
                        subjects,
                        kind,
                        trnm_rts_protocol::RtsOrderSource::Replay,
                    );
                    order.target_actor_id = actor_id;
                    order.target_tile = Some(trnm_rts_protocol::RtsTile::new(
                        i32::from(position.x),
                        i32::from(position.y),
                    ));
                    sim.issue_order(order)?;
                }
            }
            sim.step()?;
        }
        let player_hp = sim.party_hp_percent();
        let enemy_hp = sim.enemy_hp_percent();
        let player_faction_index = match seed.skirmish.player_faction {
            RtsFaction::MirrorCoalition => 0,
            RtsFaction::AshenCompact => 1,
        };
        let enemy_faction_index = 1 - player_faction_index;
        pressure[player_faction_index] +=
            u64::from(100_u8.saturating_sub(enemy_hp)) * 10 + u64::from(sim.player_score);
        pressure[enemy_faction_index] +=
            u64::from(100_u8.saturating_sub(player_hp)) * 10 + u64::from(sim.enemy_score);
        samples.push(SkirmishBalanceSample {
            map_id: seed.map_id.clone(),
            player_faction: seed.skirmish.player_faction,
            enemy_faction: seed.skirmish.enemy_faction,
            seed_hash: seed.seed_hash.clone(),
            map_fingerprint,
            simulation_salt: simulation_salt(seed),
            final_tick: sim.tick,
            outcome: sim.outcome,
            player_score: sim.player_score,
            enemy_score: sim.enemy_score,
            player_hp_percent: player_hp,
            enemy_hp_percent: enemy_hp,
        });
    }
    let [mirror, ashen] = pressure;
    if map_fingerprints.values().collect::<BTreeSet<_>>().len() != map_fingerprints.len() {
        return Err(SimError::Integrity(
            "different balance-map ids resolve to duplicate authoritative geometry".to_string(),
        ));
    }
    let high = mirror.max(ashen).max(1);
    let delta = (mirror.abs_diff(ashen) * 1000 / high) as u16;
    Ok(SkirmishBalanceMatrix {
        contract_version: "trnm_skirmish_balance_matrix_v1".to_string(),
        samples,
        faction_pressure_delta_permille: delta,
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MissionSimV1 {
    pub contract_version: String,
    pub seed: BattleSeedV1,
    pub tick: u64,
    pub phase: BattlePhase,
    #[serde(default)]
    pub objective_index: usize,
    #[serde(default)]
    pub objective_progress_ticks: u32,
    #[serde(default)]
    pub convoy_position: Option<BattleGridPoint>,
    #[serde(default)]
    pub convoy_hp: i64,
    #[serde(default)]
    pub move_intents: BTreeMap<String, MoveIntent>,
    #[serde(default)]
    pub tile_reservations: Vec<TileReservation>,
    pub active_order: Option<RtsFrameOrder>,
    #[serde(default)]
    pub queued_orders: VecDeque<RtsFrameOrder>,
    #[serde(default)]
    pub control_groups: BTreeMap<String, BTreeSet<String>>,
    pub last_order_frame: Option<u32>,
    pub order_count: u32,
    pub distinct_order_kinds: BTreeSet<String>,
    #[serde(default)]
    pub replay_orders: Vec<SimReplayEntry>,
    pub party: Vec<SimUnit>,
    pub enemies: Vec<SimUnit>,
    pub relay_guard_hp: i64,
    pub relay_guard_max_hp: i64,
    pub relay_capture_ticks: u32,
    pub resources_gathered: u32,
    pub resources_available: u32,
    pub resources_spent: u32,
    #[serde(default)]
    pub resources_generated: u32,
    #[serde(default)]
    pub player_score: u32,
    #[serde(default)]
    pub enemy_score: u32,
    #[serde(default)]
    pub resource_nodes: Vec<ResourceNodeState>,
    #[serde(default)]
    pub structures: Vec<SimStructure>,
    pub reinforcement_wave: u8,
    #[serde(default)]
    pub intel_level: u8,
    #[serde(default)]
    pub recon_bonus_ticks: u32,
    #[serde(default)]
    pub recon_focus: Option<BattleGridPoint>,
    #[serde(default)]
    pub visible_tiles: BTreeSet<BattleGridPoint>,
    #[serde(default)]
    pub explored_tiles: BTreeSet<BattleGridPoint>,
    #[serde(default)]
    pub jobs: Vec<SimJob>,
    #[serde(default)]
    pub support_units: Vec<SupportUnit>,
    #[serde(default)]
    pub researched_techs: BTreeSet<String>,
    #[serde(default)]
    pub upgrade_level: u8,
    #[serde(default)]
    pub armor_upgrade_level: u8,
    #[serde(default)]
    pub enemy_tactics_level: u8,
    #[serde(default)]
    pub enemy_ai_goal: AiGoal,
    #[serde(default)]
    pub enemy_ai_budget: u16,
    #[serde(default)]
    pub enemy_ai_decision_index: u32,
    #[serde(default)]
    pub enemy_ai_history: Vec<AiDecision>,
    #[serde(default)]
    pub enemy_resources_available: u32,
    #[serde(default)]
    pub enemy_resources_generated: u32,
    #[serde(default)]
    pub enemy_resources_spent: u32,
    #[serde(default)]
    pub enemy_workers: u8,
    #[serde(default)]
    pub enemy_structures: Vec<SimStructure>,
    #[serde(default)]
    pub enemy_researched_techs: BTreeSet<String>,
    #[serde(default)]
    pub enemy_jobs: Vec<EnemyJob>,
    #[serde(default)]
    pub enemy_build_order_index: u32,
    #[serde(default)]
    pub enemy_ability_activations: BTreeMap<String, u32>,
    pub outcome: Option<BattleOutcome>,
    pub event_count: u64,
}

impl MissionSimV1 {
    pub fn from_seed(seed: BattleSeedV1) -> Result<Self, SimError> {
        seed.validate()?;
        let party_positions = formation_positions(seed.map.party_start, &seed);
        let party = seed
            .party
            .iter()
            .enumerate()
            .map(|(index, unit)| SimUnit {
                unit_id: unit.unit_id.clone(),
                role: unit.role.clone(),
                persistent: unit.persistent,
                skill_ids: unit.skill_ids.clone(),
                max_hp: unit.stats.max_hp as i64,
                hp: unit.stats.max_hp as i64,
                damage: (unit.stats.damage as i64 * unit.stats.skill_power_permille as i64 / 1000)
                    .max(1),
                armor: unit.stats.armor as i64,
                move_speed_milli: unit.stats.move_speed_milli as i32,
                movement_budget_milli: 0,
                attack_interval_ticks: unit.stats.attack_interval_ticks.max(1),
                evasion_permille: unit.stats.evasion_permille,
                energy: unit.stats.energy as i64,
                max_energy: unit.stats.energy as i64,
                ability_range: unit.stats.ability_range.max(1) as i16,
                ability_cooldown_ticks: 0,
                guard_ticks: 0,
                position: party_positions[index],
                attacks_made: 0,
                stance: RtsUnitStance::Guard,
                patrol_anchor: None,
                patrol_target: None,
                patrol_returning: false,
                cargo: 0,
                cargo_capacity: WORKER_CARGO_CAPACITY,
                confirmed_kills: 0,
                veteran_rank: unit.veteran_rank,
            })
            .collect();
        let enemy_profiles = [
            ("scout", 800, 8, 3, 1_050, 18),
            ("warden", 1_200, 10, 7, 760, 24),
            ("striker", 900, 12, 4, 900, 20),
            ("relay_guard", 1_400, 11, 8, 680, 26),
        ];
        let aftershock = is_aftershock_map(&seed.map_id);
        let siege = seed.map_id == "mirror_siege";
        let skirmish = seed.skirmish.enabled;
        let mission_scale = if siege {
            110
        } else if aftershock {
            112
        } else if skirmish {
            105
        } else {
            100
        };
        let difficulty_scale = match seed.difficulty {
            CampaignDifficulty::Story => 90,
            CampaignDifficulty::Standard => 100,
            CampaignDifficulty::Veteran => 115,
        };
        let enemy_scale = mission_scale * difficulty_scale / 100;
        let enemy_faction = if seed.skirmish.enabled {
            seed.skirmish.enemy_faction
        } else {
            RtsFaction::AshenCompact
        };
        let mut enemies = seed
            .map
            .enemy_spawns
            .iter()
            .enumerate()
            .map(|(index, spawn)| {
                let roster_match = UNIT_ROSTER
                    .iter()
                    .find(|unit| unit.faction == enemy_faction && unit.id == spawn.id)
                    .or_else(|| {
                        seed.skirmish.enabled.then_some(()).and_then(|_| {
                            UNIT_ROSTER
                                .iter()
                                .filter(|unit| unit.faction == enemy_faction)
                                .nth(index % 6)
                        })
                    });
                let (role, hp, damage, armor, speed, interval) = roster_match
                    .map(|unit| {
                        let speed = match unit.ability() {
                            UnitAbility::RevealPulse | UnitAbility::SmokeDash => 1_080,
                            UnitAbility::ArcVolley | UnitAbility::PiercingCharge => 920,
                            UnitAbility::FieldRepair | UnitAbility::DemolitionCharge => 820,
                            UnitAbility::SuppressionBlast | UnitAbility::CommandSurge => 650,
                            _ => 760,
                        };
                        (
                            unit.role,
                            unit.hp as i64 * 8,
                            unit.damage as i64,
                            2_i64 + unit.supply as i64 * 2,
                            speed,
                            17_u32 + unit.supply as u32 * 3,
                        )
                    })
                    .unwrap_or(enemy_profiles[index.min(enemy_profiles.len() - 1)]);
                SimUnit {
                    unit_id: spawn.id.clone(),
                    role: role.to_string(),
                    persistent: false,
                    skill_ids: roster_match
                        .map(|unit| vec![unit.ability().rule_id().to_string()])
                        .unwrap_or_default(),
                    max_hp: hp * enemy_scale / 100,
                    hp: hp * enemy_scale / 100,
                    damage: damage * enemy_scale / 100,
                    armor: armor
                        + if aftershock || siege { 1 } else { 0 }
                        + if seed.difficulty == CampaignDifficulty::Veteran {
                            1
                        } else {
                            0
                        },
                    move_speed_milli: speed,
                    movement_budget_milli: 0,
                    attack_interval_ticks: interval,
                    evasion_permille: 25 + index as u16 * 10,
                    energy: 0,
                    max_energy: 0,
                    ability_range: 1,
                    ability_cooldown_ticks: 0,
                    guard_ticks: 0,
                    position: nearest_passable(&seed, spawn.position).unwrap_or(spawn.position),
                    attacks_made: 0,
                    stance: RtsUnitStance::Aggressive,
                    patrol_anchor: None,
                    patrol_target: None,
                    patrol_returning: false,
                    cargo: 0,
                    cargo_capacity: WORKER_CARGO_CAPACITY,
                    confirmed_kills: 0,
                    veteran_rank: 0,
                }
            })
            .collect::<Vec<_>>();
        if seed.skirmish.enabled {
            for index in 0..3 {
                let position = nearest_passable(
                    &seed,
                    BattleGridPoint::new(
                        seed.map.objective.x - 1 - index as i16,
                        seed.map.objective.y + 2,
                    ),
                )
                .unwrap_or(seed.map.objective);
                enemies.push(SimUnit {
                    unit_id: format!("enemy_worker_{index}"),
                    role: "worker".to_string(),
                    persistent: false,
                    skill_ids: vec!["enemy_harvest".to_string()],
                    max_hp: 420,
                    hp: 420,
                    damage: 5,
                    armor: 1,
                    move_speed_milli: 850,
                    movement_budget_milli: 0,
                    attack_interval_ticks: 30,
                    evasion_permille: 20,
                    energy: 0,
                    max_energy: 0,
                    ability_range: 1,
                    ability_cooldown_ticks: 0,
                    guard_ticks: 0,
                    position,
                    attacks_made: 0,
                    stance: RtsUnitStance::Guard,
                    patrol_anchor: None,
                    patrol_target: None,
                    patrol_returning: false,
                    cargo: 0,
                    cargo_capacity: WORKER_CARGO_CAPACITY,
                    confirmed_kills: 0,
                    veteran_rank: 0,
                });
            }
        }
        let relay_guard_base = if siege {
            RELAY_GUARD_HP + 900
        } else if aftershock {
            RELAY_GUARD_HP + 600
        } else {
            RELAY_GUARD_HP
        };
        let relay_guard_max_hp = relay_guard_base * difficulty_scale / 100;
        let starting_resources =
            seed.expedition_readiness
                .starting_resources
                .saturating_add(if seed.skirmish.enabled {
                    seed.skirmish.starting_resources
                } else {
                    0
                });
        let mut sim = Self {
            contract_version: RTS_SIM_CONTRACT.to_string(),
            seed: seed.clone(),
            tick: 0,
            phase: if seed.mission.mission == trnm_campaign_core::CampaignMission::ConvoyExodus {
                BattlePhase::ConvoyEscort
            } else {
                BattlePhase::Approach
            },
            objective_index: 0,
            objective_progress_ticks: 0,
            convoy_position: (seed.mission.mission
                == trnm_campaign_core::CampaignMission::ConvoyExodus)
                .then(|| {
                    nearest_passable(
                        &seed,
                        BattleGridPoint::new(seed.map.party_start.x - 1, seed.map.party_start.y),
                    )
                    .unwrap_or(seed.map.party_start)
                }),
            convoy_hp: if seed.mission.mission == trnm_campaign_core::CampaignMission::ConvoyExodus
            {
                1_200
            } else {
                0
            },
            move_intents: BTreeMap::new(),
            tile_reservations: Vec::new(),
            active_order: None,
            queued_orders: VecDeque::new(),
            control_groups: BTreeMap::new(),
            last_order_frame: None,
            order_count: 0,
            distinct_order_kinds: BTreeSet::new(),
            replay_orders: Vec::new(),
            party,
            enemies,
            relay_guard_hp: relay_guard_max_hp,
            relay_guard_max_hp,
            relay_capture_ticks: 0,
            resources_gathered: starting_resources,
            resources_available: starting_resources,
            resources_spent: 0,
            resources_generated: starting_resources,
            player_score: 0,
            enemy_score: 0,
            resource_nodes: seed
                .map
                .resource_nodes
                .iter()
                .map(|node| ResourceNodeState {
                    node_id: node.id.clone(),
                    position: nearest_passable(&seed, node.position).unwrap_or(node.position),
                    remaining: RESOURCE_NODE_CAPACITY,
                })
                .collect(),
            structures: vec![
                SimStructure {
                    structure_id: "expedition_command_post".to_string(),
                    kind: SimStructureKind::CommandPost,
                    position: seed.map.party_start,
                    hp: 900,
                    max_hp: 900,
                },
                SimStructure {
                    structure_id: "field_workshop".to_string(),
                    kind: SimStructureKind::FieldWorkshop,
                    position: nearest_passable(
                        &seed,
                        BattleGridPoint::new(seed.map.party_start.x + 2, seed.map.party_start.y),
                    )
                    .unwrap_or(seed.map.party_start),
                    hp: 600,
                    max_hp: 600,
                },
            ],
            reinforcement_wave: 0,
            intel_level: 0,
            recon_bonus_ticks: 0,
            recon_focus: None,
            visible_tiles: BTreeSet::new(),
            explored_tiles: BTreeSet::new(),
            jobs: Vec::new(),
            support_units: Vec::new(),
            researched_techs: BTreeSet::new(),
            upgrade_level: 0,
            armor_upgrade_level: 0,
            enemy_tactics_level: 0,
            enemy_ai_goal: AiGoal::Scout,
            enemy_ai_budget: 0,
            enemy_ai_decision_index: 0,
            enemy_ai_history: Vec::new(),
            enemy_resources_available: if seed.skirmish.enabled {
                seed.skirmish.starting_resources
            } else {
                0
            },
            enemy_resources_generated: if seed.skirmish.enabled {
                seed.skirmish.starting_resources
            } else {
                0
            },
            enemy_resources_spent: 0,
            enemy_workers: if seed.skirmish.enabled { 3 } else { 0 },
            enemy_structures: if seed.skirmish.enabled {
                vec![
                    SimStructure {
                        structure_id: "enemy_command_post".to_string(),
                        kind: SimStructureKind::CommandPost,
                        position: seed.map.objective,
                        hp: 1_200,
                        max_hp: 1_200,
                    },
                    SimStructure {
                        structure_id: "enemy_field_workshop".to_string(),
                        kind: SimStructureKind::FieldWorkshop,
                        position: nearest_passable(
                            &seed,
                            BattleGridPoint::new(seed.map.objective.x - 2, seed.map.objective.y),
                        )
                        .unwrap_or(seed.map.objective),
                        hp: 600,
                        max_hp: 600,
                    },
                    SimStructure {
                        structure_id: "enemy_supply_cache".to_string(),
                        kind: SimStructureKind::SupplyCache,
                        position: nearest_passable(
                            &seed,
                            BattleGridPoint::new(
                                seed.map.objective.x - 1,
                                seed.map.objective.y + 2,
                            ),
                        )
                        .unwrap_or(seed.map.objective),
                        hp: 420,
                        max_hp: 420,
                    },
                    SimStructure {
                        structure_id: "enemy_supply_cache_aux".to_string(),
                        kind: SimStructureKind::SupplyCache,
                        position: nearest_passable(
                            &seed,
                            BattleGridPoint::new(
                                seed.map.objective.x - 3,
                                seed.map.objective.y + 2,
                            ),
                        )
                        .unwrap_or(seed.map.objective),
                        hp: 420,
                        max_hp: 420,
                    },
                ]
            } else {
                Vec::new()
            },
            enemy_researched_techs: BTreeSet::new(),
            enemy_jobs: Vec::new(),
            enemy_build_order_index: 0,
            enemy_ability_activations: BTreeMap::new(),
            outcome: None,
            event_count: 0,
        };
        sim.assign_control_group(
            "1",
            sim.party.iter().map(|unit| unit.unit_id.clone()).collect(),
        );
        sim.refresh_visibility();
        sim.validate()?;
        Ok(sim)
    }

    pub fn validate(&self) -> Result<(), SimError> {
        if self.contract_version != RTS_SIM_CONTRACT {
            return Err(SimError::InvalidState(format!(
                "unsupported simulation contract {}",
                self.contract_version
            )));
        }
        self.seed.validate()?;
        let expected_ids = self
            .seed
            .party
            .iter()
            .map(|unit| unit.unit_id.as_str())
            .collect::<BTreeSet<_>>();
        let actual_ids = self
            .party
            .iter()
            .map(|unit| unit.unit_id.as_str())
            .collect::<BTreeSet<_>>();
        if expected_ids != actual_ids || self.party.len() != self.seed.party.len() {
            return Err(SimError::Integrity(
                "simulation party does not match BattleSeed".to_string(),
            ));
        }
        for unit in self.party.iter().chain(&self.enemies) {
            if !self.seed.map.in_bounds(unit.position) || !self.seed.map.passable(unit.position) {
                return Err(SimError::Integrity(format!(
                    "unit {} occupies an invalid map tile",
                    unit.unit_id
                )));
            }
        }
        if self.objective_index > self.seed.mission.objectives.len()
            || self.convoy_position.is_some_and(|position| {
                !self.seed.map.in_bounds(position) || !self.seed.map.passable(position)
            })
            || self.convoy_hp < 0
        {
            return Err(SimError::Integrity(
                "mission objective or convoy state is invalid".to_string(),
            ));
        }
        if self.tile_reservations.iter().any(|reservation| {
            !self.seed.map.in_bounds(reservation.tile)
                || !self.seed.map.passable(reservation.tile)
                || !self
                    .party
                    .iter()
                    .any(|unit| unit.unit_id == reservation.unit_id)
        }) {
            return Err(SimError::Integrity(
                "tile reservation references invalid traffic state".to_string(),
            ));
        }
        for support in &self.support_units {
            if !self.seed.map.in_bounds(support.position)
                || !self.seed.map.passable(support.position)
                || support.hp <= 0
            {
                return Err(SimError::Integrity(format!(
                    "support unit {} occupies an invalid state",
                    support.unit_id
                )));
            }
        }
        for structure in self.structures.iter().chain(&self.enemy_structures) {
            if !self.seed.map.in_bounds(structure.position)
                || !self.seed.map.passable(structure.position)
                || structure.max_hp <= 0
                || structure.hp > structure.max_hp
            {
                return Err(SimError::Integrity(format!(
                    "structure {} occupies an invalid state",
                    structure.structure_id
                )));
            }
        }
        if self.seed.skirmish.enabled
            && self
                .enemy_resources_available
                .saturating_add(self.enemy_resources_spent)
                != self.enemy_resources_generated
        {
            return Err(SimError::Integrity(
                "enemy resource conservation is inconsistent".to_string(),
            ));
        }
        for node in &self.resource_nodes {
            if !self.seed.map.in_bounds(node.position)
                || node.remaining > RESOURCE_NODE_CAPACITY
                || !self.seed.map.passable(node.position)
            {
                return Err(SimError::Integrity(format!(
                    "resource node {} occupies an invalid state",
                    node.node_id
                )));
            }
        }
        for job in &self.jobs {
            if job.remaining_ticks == 0
                || !self.seed.map.in_bounds(job.target)
                || !self.seed.map.passable(job.target)
                || job.cost == 0
            {
                return Err(SimError::Integrity(format!(
                    "queued job {} is invalid",
                    job.job_id
                )));
            }
        }
        if self
            .resources_available
            .saturating_add(self.resources_spent)
            != self.resources_gathered
            || self.relay_guard_max_hp <= 0
            || self.relay_guard_hp > self.relay_guard_max_hp
        {
            return Err(SimError::Integrity(
                "resource or relay accounting is inconsistent".to_string(),
            ));
        }
        if let Some(order) = &self.active_order {
            order.validate().map_err(SimError::Order)?;
        }
        for order in &self.queued_orders {
            order.validate().map_err(SimError::Order)?;
            if !order.queued {
                return Err(SimError::Integrity(
                    "queued order storage contains a non-queued order".to_string(),
                ));
            }
        }
        for members in self.control_groups.values() {
            if !members
                .iter()
                .all(|member| actual_ids.contains(member.as_str()))
            {
                return Err(SimError::Integrity(
                    "control group references an unknown party unit".to_string(),
                ));
            }
        }
        if self
            .visible_tiles
            .iter()
            .chain(&self.explored_tiles)
            .any(|tile| !self.seed.map.in_bounds(*tile))
        {
            return Err(SimError::Integrity(
                "fog state contains an out-of-bounds tile".to_string(),
            ));
        }
        if self.enemy_ai_history.len() > 16
            || self
                .enemy_ai_history
                .windows(2)
                .any(|pair| pair[0].index >= pair[1].index)
        {
            return Err(SimError::Integrity(
                "enemy AI decision history is not a bounded ordered replay".to_string(),
            ));
        }
        if self.replay_orders.len() > MAX_REPLAY_ORDERS
            || self
                .replay_orders
                .windows(2)
                .any(|orders| orders[0].issued_tick > orders[1].issued_tick)
        {
            return Err(SimError::Integrity(
                "player replay is not bounded and frame ordered".to_string(),
            ));
        }
        Ok(())
    }

    pub fn supply_cap(&self) -> u8 {
        self.structures
            .iter()
            .filter(|structure| structure.alive())
            .map(SimStructure::supply_provided)
            .fold(0_u8, u8::saturating_add)
    }

    pub fn supply_used(&self) -> u8 {
        let active_party = self.party.iter().filter(|unit| unit.alive()).count() as u8;
        let support = self
            .support_units
            .iter()
            .map(|unit| unit.supply)
            .fold(0_u8, u8::saturating_add);
        let reserved = self
            .jobs
            .iter()
            .map(|job| match job.kind {
                SimJobKind::TrainSupport | SimJobKind::TrainMedic => 1,
                SimJobKind::TrainRosterUnit => UNIT_ROSTER
                    .iter()
                    .find(|unit| unit.id == job.rule_id)
                    .map(|unit| unit.supply)
                    .unwrap_or(1),
                _ => 0,
            })
            .fold(0_u8, u8::saturating_add);
        active_party
            .saturating_add(support)
            .saturating_add(reserved)
    }

    pub fn power_provided(&self) -> u16 {
        self.structures
            .iter()
            .filter(|structure| structure.alive())
            .map(SimStructure::power_provided)
            .sum()
    }

    pub fn power_draw(&self) -> u16 {
        self.structures
            .iter()
            .filter(|structure| structure.alive())
            .map(SimStructure::power_draw)
            .sum()
    }

    pub fn low_power(&self) -> bool {
        self.power_draw() > self.power_provided()
    }

    pub fn enemy_supply_cap(&self) -> u8 {
        self.enemy_structures
            .iter()
            .filter(|structure| structure.alive())
            .map(SimStructure::supply_provided)
            .fold(0_u8, u8::saturating_add)
    }

    pub fn enemy_supply_used(&self) -> u8 {
        let living = self
            .enemies
            .iter()
            .filter(|unit| unit.alive())
            .map(|unit| {
                if unit.role == "worker" {
                    1
                } else {
                    UNIT_ROSTER
                        .iter()
                        .find(|entry| unit.unit_id.contains(entry.id))
                        .map(|entry| entry.supply)
                        .unwrap_or(1)
                }
            })
            .fold(0_u8, u8::saturating_add);
        let reserved = self
            .enemy_jobs
            .iter()
            .filter(|job| job.kind == EnemyJobKind::Train)
            .filter_map(|job| UNIT_ROSTER.iter().find(|entry| entry.id == job.rule_id))
            .map(|entry| entry.supply)
            .fold(0_u8, u8::saturating_add);
        living.saturating_add(reserved)
    }

    pub fn enemy_power_provided(&self) -> u16 {
        self.enemy_structures
            .iter()
            .filter(|structure| structure.alive())
            .map(SimStructure::power_provided)
            .sum()
    }

    pub fn enemy_power_draw(&self) -> u16 {
        self.enemy_structures
            .iter()
            .filter(|structure| structure.alive())
            .map(SimStructure::power_draw)
            .sum()
    }

    pub fn enemy_low_power(&self) -> bool {
        self.enemy_power_draw() > self.enemy_power_provided()
    }

    pub fn terminal(&self) -> bool {
        self.outcome.is_some()
    }

    pub fn current_order_kind(&self) -> RtsOrderKind {
        self.active_order
            .as_ref()
            .map(|order| order.kind)
            .unwrap_or(RtsOrderKind::Hold)
    }

    pub fn issue_order(&mut self, order: RtsFrameOrder) -> Result<(), SimError> {
        self.validate()?;
        if self.terminal() {
            return Err(SimError::InvalidState(
                "cannot issue an order to a terminal battle".to_string(),
            ));
        }
        order.validate().map_err(SimError::Order)?;
        if order.player_id != "player" {
            return Err(SimError::Order(
                "only the local player may command the party".to_string(),
            ));
        }
        if self
            .last_order_frame
            .is_some_and(|previous| order.frame < previous)
        {
            return Err(SimError::Order("order frame regression".to_string()));
        }
        let living_party = self
            .party
            .iter()
            .filter(|unit| unit.alive())
            .map(|unit| unit.unit_id.as_str())
            .collect::<BTreeSet<_>>();
        let subjects = order
            .subject_actor_ids
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if subjects.is_empty() || !subjects.is_subset(&living_party) {
            return Err(SimError::Order(
                "order subjects must be living seeded party units".to_string(),
            ));
        }
        if let Some(tile) = order.target_tile {
            let target = BattleGridPoint::new(tile.x as i16, tile.y as i16);
            if !self.seed.map.in_bounds(target) || !self.seed.map.passable(target) {
                return Err(SimError::Order(
                    "order target tile is blocked or outside the map".to_string(),
                ));
            }
        }
        if matches!(order.kind, RtsOrderKind::Attack | RtsOrderKind::FocusFire) {
            if let Some(target) = order.target_actor_id.as_deref() {
                let target_position = self
                    .enemies
                    .iter()
                    .find(|enemy| enemy.unit_id == target && enemy.alive())
                    .map(|enemy| enemy.position)
                    .or_else(|| {
                        self.enemy_structures
                            .iter()
                            .find(|structure| structure.structure_id == target && structure.alive())
                            .map(|structure| structure.position)
                    });
                if self.seed.skirmish.enabled && target_position.is_none() {
                    return Err(SimError::Order("attack target is not alive".to_string()));
                }
                if target_position.is_some_and(|position| !self.visible_tiles.contains(&position)) {
                    return Err(SimError::Order(
                        "target enemy is outside current line of sight".to_string(),
                    ));
                }
            }
        }
        if matches!(order.kind, RtsOrderKind::Extract) {
            if self.tick < WITHDRAWAL_MIN_TICKS {
                return Err(SimError::Order(
                    "withdrawal requires thirty committed simulation ticks".to_string(),
                ));
            }
            self.outcome = Some(BattleOutcome::Withdrawal);
            self.phase = BattlePhase::Complete;
        } else if order.queued {
            if !is_continuous_order(order.kind) {
                return Err(SimError::Order(
                    "only movement, combat, harvest and hold orders may be shift-queued"
                        .to_string(),
                ));
            }
            self.queued_orders.push_back(order.clone());
            if self.active_order.is_none() {
                self.activate_next_queued_order();
            }
        } else {
            match order.kind {
                RtsOrderKind::Ability => self.resolve_party_ability(&order)?,
                RtsOrderKind::Repair => self.resolve_repair(&order)?,
                RtsOrderKind::Build => self.resolve_fortify(&order)?,
                RtsOrderKind::Recon => self.resolve_recon(&order)?,
                RtsOrderKind::Train | RtsOrderKind::Research | RtsOrderKind::Upgrade => {
                    self.queue_job_from_order(&order)?
                }
                RtsOrderKind::AssignGroup => self.assign_control_group_order(&order, false),
                RtsOrderKind::AppendGroup => self.assign_control_group_order(&order, true),
                RtsOrderKind::RemoveGroup => self.remove_control_group_order(&order),
                RtsOrderKind::RecallGroup => {
                    let group = order.target_rule_id.as_deref().unwrap_or_default();
                    if self.control_group_members(group).is_empty() {
                        return Err(SimError::Order("control group is empty".to_string()));
                    }
                }
                RtsOrderKind::CancelQueuedOrder => self.cancel_queued_order(&order)?,
                RtsOrderKind::CancelJob => self.cancel_job(&order)?,
                RtsOrderKind::PauseJob => self.set_job_paused(&order, true)?,
                RtsOrderKind::ResumeJob => self.set_job_paused(&order, false)?,
                RtsOrderKind::PromoteJob => self.promote_job(&order)?,
                RtsOrderKind::SetRally => self.set_rally(&order)?,
                RtsOrderKind::SetStance => self.set_unit_stance(&order)?,
                RtsOrderKind::Stop => {
                    self.queued_orders.clear();
                    self.active_order = None;
                    for unit in &mut self.party {
                        if order.subject_actor_ids.contains(&unit.unit_id) {
                            unit.patrol_anchor = None;
                            unit.patrol_target = None;
                        }
                    }
                }
                RtsOrderKind::Move
                | RtsOrderKind::AttackMove
                | RtsOrderKind::Patrol
                | RtsOrderKind::Harvest
                | RtsOrderKind::Capture
                | RtsOrderKind::Attack
                | RtsOrderKind::FocusFire
                | RtsOrderKind::Hold => {
                    self.queued_orders.clear();
                    if order.kind == RtsOrderKind::Patrol {
                        let target = order.target_tile.expect("validated patrol tile");
                        for unit in &mut self.party {
                            if order.subject_actor_ids.contains(&unit.unit_id) {
                                unit.patrol_anchor = Some(unit.position);
                                unit.patrol_target =
                                    Some(BattleGridPoint::new(target.x as i16, target.y as i16));
                                unit.patrol_returning = false;
                            }
                        }
                    }
                    self.active_order = Some(order.clone());
                }
                RtsOrderKind::Extract => unreachable!("withdrawal handled above"),
            }
        }
        self.last_order_frame = Some(order.frame);
        self.order_count = self.order_count.saturating_add(1);
        self.distinct_order_kinds
            .insert(order.kind.as_str().to_string());
        self.replay_orders.push(SimReplayEntry {
            issued_tick: self.tick,
            order,
        });
        if self.replay_orders.len() > MAX_REPLAY_ORDERS {
            self.replay_orders.remove(0);
        }
        self.event_count += 1;
        Ok(())
    }

    pub fn control_group_members(&self, group_id: &str) -> Vec<String> {
        self.control_groups
            .get(group_id)
            .into_iter()
            .flatten()
            .filter(|member| {
                self.party
                    .iter()
                    .any(|unit| unit.unit_id == **member && unit.alive())
            })
            .cloned()
            .collect()
    }

    fn assign_control_group(&mut self, group_id: &str, members: BTreeSet<String>) {
        self.control_groups.insert(group_id.to_string(), members);
    }

    fn assign_control_group_order(&mut self, order: &RtsFrameOrder, append: bool) {
        let group = order.target_rule_id.as_deref().unwrap_or_default();
        let members = order
            .subject_actor_ids
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        if append {
            self.control_groups
                .entry(group.to_string())
                .or_default()
                .extend(members);
        } else {
            self.assign_control_group(group, members);
        }
    }

    fn remove_control_group_order(&mut self, order: &RtsFrameOrder) {
        let group = order.target_rule_id.as_deref().unwrap_or_default();
        if let Some(members) = self.control_groups.get_mut(group) {
            for member in &order.subject_actor_ids {
                members.remove(member);
            }
        }
    }

    fn prune_control_groups(&mut self) {
        let living = self
            .party
            .iter()
            .filter(|unit| unit.alive())
            .map(|unit| unit.unit_id.clone())
            .collect::<BTreeSet<_>>();
        for members in self.control_groups.values_mut() {
            members.retain(|member| living.contains(member));
        }
    }

    fn cancel_queued_order(&mut self, order: &RtsFrameOrder) -> Result<(), SimError> {
        let queue_id = order.queue_id.as_deref().unwrap_or_default();
        let before = self.queued_orders.len();
        self.queued_orders
            .retain(|queued| queued.queue_id.as_deref() != Some(queue_id));
        if before == self.queued_orders.len() {
            return Err(SimError::Order("queued order was not found".to_string()));
        }
        Ok(())
    }

    fn activate_next_queued_order(&mut self) {
        self.active_order = self.queued_orders.pop_front().map(|mut order| {
            order.queued = false;
            order
        });
    }

    fn active_order_complete(&self) -> bool {
        let Some(order) = &self.active_order else {
            return true;
        };
        match order.kind {
            RtsOrderKind::Move | RtsOrderKind::AttackMove => {
                order.target_tile.is_some_and(|tile| {
                    let target = BattleGridPoint::new(tile.x as i16, tile.y as i16);
                    self.party.iter().filter(|unit| unit.alive()).all(|unit| {
                        !order.subject_actor_ids.contains(&unit.unit_id)
                            || distance(unit.position, target) <= 1
                    })
                })
            }
            RtsOrderKind::Patrol => false,
            RtsOrderKind::Attack | RtsOrderKind::FocusFire => order
                .target_actor_id
                .as_deref()
                .and_then(|id| {
                    self.enemies
                        .iter()
                        .find(|enemy| enemy.unit_id == id)
                        .map(SimTargetRef::Unit)
                        .or_else(|| {
                            self.enemy_structures
                                .iter()
                                .find(|structure| structure.structure_id == id)
                                .map(SimTargetRef::Structure)
                        })
                })
                .map_or(
                    self.phase == BattlePhase::Relay
                        && self.relay_guard_hp <= 0
                        && self.enemies.iter().all(|enemy| !enemy.alive()),
                    SimTargetRef::destroyed,
                ),
            RtsOrderKind::Harvest => self.resources_available >= 100,
            RtsOrderKind::Capture | RtsOrderKind::Hold => self.terminal(),
            _ => true,
        }
    }

    pub fn step(&mut self) -> Result<(), SimError> {
        self.validate()?;
        if self.terminal() {
            return Err(SimError::InvalidState(
                "cannot advance a terminal battle".to_string(),
            ));
        }
        self.tick += 1;
        self.recon_bonus_ticks = self.recon_bonus_ticks.saturating_sub(1);
        self.process_jobs();
        self.resolve_structure_functions();
        self.resolve_enemy_structure_functions();
        for support in &mut self.support_units {
            support.ability_cooldown_ticks = support.ability_cooldown_ticks.saturating_sub(1);
        }
        for unit in &mut self.party {
            unit.ability_cooldown_ticks = unit.ability_cooldown_ticks.saturating_sub(1);
            unit.guard_ticks = unit.guard_ticks.saturating_sub(1);
            if self.tick.is_multiple_of(50) {
                unit.energy = (unit.energy + 1).min(unit.max_energy);
            }
        }
        for unit in &mut self.enemies {
            unit.ability_cooldown_ticks = unit.ability_cooldown_ticks.saturating_sub(1);
            unit.guard_ticks = unit.guard_ticks.saturating_sub(1);
        }
        self.resolve_player_order();
        self.resolve_stance_fire();
        self.update_phase();
        self.refresh_enemy_ai_plan();
        self.resolve_enemy_workers();
        self.resolve_enemy_economy();
        self.resolve_enemy_ai();
        self.resolve_support_fire();
        self.resolve_relay_pressure();
        self.resolve_mission_objective();
        self.update_phase();
        self.prune_control_groups();
        self.refresh_visibility();
        if self.active_order_complete() && !self.terminal() {
            self.active_order = None;
            self.activate_next_queued_order();
        }
        if self.party.iter().all(|unit| !unit.alive()) || self.tick >= FIVE_MINUTE_TICKS {
            self.outcome = Some(BattleOutcome::Defeat);
            self.phase = BattlePhase::Complete;
            self.event_count += 1;
        } else if self.seed.skirmish.enabled {
            let terminal = match self.seed.skirmish.victory_mode {
                SkirmishVictoryMode::Objective => (self.objective_index
                    >= self.seed.mission.objectives.len())
                .then_some(BattleOutcome::Victory),
                SkirmishVictoryMode::Score => {
                    if self.player_score >= self.seed.skirmish.score_target {
                        Some(BattleOutcome::Victory)
                    } else if self.enemy_score >= self.seed.skirmish.score_target {
                        Some(BattleOutcome::Defeat)
                    } else {
                        None
                    }
                }
                SkirmishVictoryMode::Annihilation => {
                    (self.enemies.iter().all(|enemy| !enemy.alive())
                        && self
                            .enemy_structures
                            .iter()
                            .all(|structure| !structure.alive()))
                    .then_some(BattleOutcome::Victory)
                }
            };
            if let Some(outcome) = terminal {
                self.outcome = Some(outcome);
                self.phase = BattlePhase::Complete;
                self.event_count += 1;
            }
        } else if self.objective_index >= self.seed.mission.objectives.len() {
            self.outcome = Some(BattleOutcome::Victory);
            self.phase = BattlePhase::Complete;
            self.event_count += 1;
        }
        Ok(())
    }

    fn resolve_player_order(&mut self) {
        let Some(order) = self.active_order.clone() else {
            return;
        };
        let selected = order
            .subject_actor_ids
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        match order.kind {
            RtsOrderKind::Move | RtsOrderKind::AttackMove => {
                if let Some(tile) = order.target_tile {
                    self.move_selected_toward(
                        &selected,
                        BattleGridPoint::new(tile.x as i16, tile.y as i16),
                        0,
                        order.formation_id.as_deref(),
                    );
                }
                if order.kind == RtsOrderKind::AttackMove {
                    self.party_attack(&selected, order.target_actor_id.as_deref());
                }
            }
            RtsOrderKind::Attack | RtsOrderKind::FocusFire => {
                let target = self.attack_target_position(order.target_actor_id.as_deref());
                if let Some(target) = target {
                    self.move_selected_toward(&selected, target, 1, None);
                }
                self.party_attack(&selected, order.target_actor_id.as_deref());
            }
            RtsOrderKind::Patrol => self.resolve_patrol(&selected),
            RtsOrderKind::Harvest => self.resolve_harvest(&selected, &order),
            RtsOrderKind::Hold | RtsOrderKind::Capture => self.resolve_capture(&selected),
            RtsOrderKind::Ability
            | RtsOrderKind::Repair
            | RtsOrderKind::Build
            | RtsOrderKind::Recon
            | RtsOrderKind::Train
            | RtsOrderKind::Research
            | RtsOrderKind::Upgrade
            | RtsOrderKind::AssignGroup
            | RtsOrderKind::AppendGroup
            | RtsOrderKind::RemoveGroup
            | RtsOrderKind::RecallGroup
            | RtsOrderKind::CancelQueuedOrder
            | RtsOrderKind::CancelJob
            | RtsOrderKind::PauseJob
            | RtsOrderKind::ResumeJob
            | RtsOrderKind::PromoteJob
            | RtsOrderKind::SetRally
            | RtsOrderKind::Stop
            | RtsOrderKind::SetStance
            | RtsOrderKind::Extract => {}
        }
    }

    fn update_phase(&mut self) {
        if self.seed.mission.mission == trnm_campaign_core::CampaignMission::ConvoyExodus {
            self.phase = match self.current_objective_kind() {
                Some(ObjectiveKind::Escort) => BattlePhase::ConvoyEscort,
                Some(ObjectiveKind::Defend) => BattlePhase::GeneratorDefense,
                Some(ObjectiveKind::Extract) => BattlePhase::Extraction,
                _ if self.objective_index >= self.seed.mission.objectives.len() => {
                    BattlePhase::Complete
                }
                _ => self.phase,
            };
            return;
        }
        if self.phase == BattlePhase::Approach
            && self
                .party
                .iter()
                .filter(|unit| unit.alive())
                .any(|unit| distance(unit.position, self.seed.map.approach_point) <= 2)
        {
            self.phase = BattlePhase::Contact;
            self.objective_index = 1;
            self.event_count += 1;
        }
        if self.phase == BattlePhase::Contact && self.enemies.iter().all(|unit| !unit.alive()) {
            self.phase = BattlePhase::Relay;
            self.event_count += 1;
        }
        if self.objective_index == 1 && self.relay_guard_hp <= 0 {
            self.objective_index = 2;
            self.objective_progress_ticks = self.relay_capture_ticks;
        }
    }

    pub fn current_objective_kind(&self) -> Option<ObjectiveKind> {
        self.seed
            .mission
            .objectives
            .get(self.objective_index)
            .map(|objective| objective.kind)
    }

    pub fn current_objective_id(&self) -> Option<&str> {
        self.seed
            .mission
            .objectives
            .get(self.objective_index)
            .map(|objective| objective.id.as_str())
    }

    fn resolve_mission_objective(&mut self) {
        let Some(objective) = self
            .seed
            .mission
            .objectives
            .get(self.objective_index)
            .cloned()
        else {
            return;
        };
        if self.seed.mission.mission != trnm_campaign_core::CampaignMission::ConvoyExodus {
            if objective.kind == ObjectiveKind::Capture {
                self.objective_progress_ticks = self.relay_capture_ticks;
                if self.relay_capture_ticks >= objective.duration_ticks {
                    self.objective_index += 1;
                }
            }
            return;
        }

        match objective.kind {
            ObjectiveKind::Escort | ObjectiveKind::Extract => {
                let Some(position) = self.convoy_position else {
                    return;
                };
                let escort_ordered = self.active_order.as_ref().is_some_and(|order| {
                    matches!(order.kind, RtsOrderKind::Move | RtsOrderKind::AttackMove)
                        && order.target_tile.is_some_and(|tile| {
                            BattleGridPoint::new(tile.x as i16, tile.y as i16) == objective.target
                        })
                });
                let escorted = escort_ordered
                    || self.party.iter().filter(|unit| unit.alive()).any(|unit| {
                        distance(unit.position, position) <= 3
                            || distance(unit.position, objective.target) <= 3
                    });
                if escorted && self.tick.is_multiple_of(8) && position != objective.target {
                    let occupied = self
                        .party
                        .iter()
                        .chain(&self.enemies)
                        .filter(|unit| unit.alive())
                        .map(|unit| unit.position)
                        .chain(self.support_units.iter().map(|unit| unit.position))
                        .collect::<BTreeSet<_>>();
                    if let Some(next) =
                        next_step_toward(&self.seed, position, objective.target, 0, &occupied)
                    {
                        self.convoy_position = Some(next);
                        self.event_count += 1;
                    }
                }
                if self.convoy_position == Some(objective.target) {
                    if objective.kind == ObjectiveKind::Escort {
                        self.objective_index += 1;
                        self.objective_progress_ticks = 0;
                    } else if escorted {
                        self.objective_progress_ticks =
                            self.objective_progress_ticks.saturating_add(1);
                        if self.objective_progress_ticks >= objective.duration_ticks {
                            self.objective_index += 1;
                        }
                    }
                }
            }
            ObjectiveKind::Defend => {
                let defenders = self
                    .party
                    .iter()
                    .filter(|unit| unit.alive() && distance(unit.position, objective.target) <= 4)
                    .count();
                if defenders > 0 {
                    self.objective_progress_ticks = self.objective_progress_ticks.saturating_add(1);
                }
                if self.objective_progress_ticks == 1 || self.objective_progress_ticks == 130 {
                    self.spawn_reinforcement_wave(true);
                }
                if self.objective_progress_ticks >= objective.duration_ticks {
                    self.objective_index += 1;
                    self.objective_progress_ticks = 0;
                }
            }
            ObjectiveKind::Destroy | ObjectiveKind::Capture => {}
        }
    }

    fn move_selected_toward(
        &mut self,
        selected: &BTreeSet<String>,
        target: BattleGridPoint,
        stop_range: i16,
        formation_id: Option<&str>,
    ) {
        self.tile_reservations.clear();
        let mut occupied = self
            .party
            .iter()
            .chain(&self.enemies)
            .filter(|unit| unit.alive())
            .map(|unit| unit.position)
            .chain(self.support_units.iter().map(|unit| unit.position))
            .collect::<BTreeSet<_>>();
        let mut indices = (0..self.party.len())
            .filter(|index| {
                self.party[*index].alive() && selected.contains(&self.party[*index].unit_id)
            })
            .collect::<Vec<_>>();
        indices.sort_by(|left, right| {
            let blocked = |index: usize| {
                self.move_intents
                    .get(&self.party[index].unit_id)
                    .map(|intent| intent.blocked_ticks)
                    .unwrap_or(0)
            };
            blocked(*right)
                .cmp(&blocked(*left))
                .then_with(|| self.party[*left].unit_id.cmp(&self.party[*right].unit_id))
        });
        for index in indices {
            self.party[index].movement_budget_milli += self.party[index].move_speed_milli;
            if self.party[index].movement_budget_milli < MOVEMENT_TILE_COST {
                continue;
            }
            occupied.remove(&self.party[index].position);
            let formation_target =
                formation_target_for(target, index, formation_id.unwrap_or("none"), &self.seed);
            let previous = self.move_intents.get(&self.party[index].unit_id);
            let mut blocked_ticks = previous.map(|intent| intent.blocked_ticks).unwrap_or(0);
            let mut replan_count = previous.map(|intent| intent.replan_count).unwrap_or(0);
            let mut next = next_step_toward(
                &self.seed,
                self.party[index].position,
                formation_target,
                stop_range,
                &occupied,
            );
            if next.is_none() && distance(self.party[index].position, formation_target) > stop_range
            {
                blocked_ticks = blocked_ticks.saturating_add(1);
                if blocked_ticks >= 6 {
                    next = deterministic_yield_step(
                        &self.seed,
                        self.party[index].position,
                        formation_target,
                        &occupied,
                        &self.tile_reservations,
                    );
                    replan_count = replan_count.saturating_add(1);
                    blocked_ticks = 0;
                }
            } else if next.is_some() {
                blocked_ticks = 0;
            }
            if next.is_some_and(|candidate| {
                self.tile_reservations
                    .iter()
                    .any(|reservation| reservation.tile == candidate)
            }) {
                next = None;
                blocked_ticks = blocked_ticks.saturating_add(1);
            }
            self.move_intents.insert(
                self.party[index].unit_id.clone(),
                MoveIntent {
                    unit_id: self.party[index].unit_id.clone(),
                    target: formation_target,
                    desired_tile: next,
                    blocked_ticks,
                    replan_count,
                },
            );
            if let Some(next) = next {
                self.party[index].position = next;
                self.party[index].movement_budget_milli -= MOVEMENT_TILE_COST;
                self.tile_reservations.push(TileReservation {
                    tile: next,
                    unit_id: self.party[index].unit_id.clone(),
                });
                occupied.insert(next);
            } else {
                occupied.insert(self.party[index].position);
            }
        }
    }

    fn attack_target_position(&self, requested: Option<&str>) -> Option<BattleGridPoint> {
        if let Some(target_id) = requested {
            if let Some(enemy) = self
                .enemies
                .iter()
                .find(|enemy| enemy.unit_id == target_id && enemy.alive())
            {
                return Some(enemy.position);
            }
            if let Some(structure) = self
                .enemy_structures
                .iter()
                .find(|structure| structure.structure_id == target_id && structure.alive())
            {
                return Some(structure.position);
            }
        }
        self.enemies
            .iter()
            .find(|enemy| enemy.alive())
            .map(|enemy| enemy.position)
            .or_else(|| {
                self.enemy_structures
                    .iter()
                    .find(|structure| structure.alive())
                    .map(|structure| structure.position)
            })
            .or(Some(self.seed.map.objective))
    }

    fn set_unit_stance(&mut self, order: &RtsFrameOrder) -> Result<(), SimError> {
        let stance = order
            .target_rule_id
            .as_deref()
            .and_then(RtsUnitStance::from_rule_id)
            .ok_or_else(|| SimError::Order("unit stance is invalid".to_string()))?;
        for unit in &mut self.party {
            if order.subject_actor_ids.contains(&unit.unit_id) {
                unit.stance = stance;
            }
        }
        Ok(())
    }

    fn resolve_patrol(&mut self, selected: &BTreeSet<String>) {
        let outward = self
            .party
            .iter()
            .filter(|unit| {
                unit.alive() && selected.contains(&unit.unit_id) && !unit.patrol_returning
            })
            .map(|unit| unit.unit_id.clone())
            .collect::<BTreeSet<_>>();
        let returning = self
            .party
            .iter()
            .filter(|unit| {
                unit.alive() && selected.contains(&unit.unit_id) && unit.patrol_returning
            })
            .map(|unit| unit.unit_id.clone())
            .collect::<BTreeSet<_>>();
        let target = self
            .party
            .iter()
            .find_map(|unit| unit.patrol_target)
            .unwrap_or(self.seed.map.approach_point);
        if !outward.is_empty() {
            self.move_selected_toward(&outward, target, 0, None);
        }
        if !returning.is_empty() {
            self.move_selected_toward(&returning, self.seed.map.party_start, 0, None);
        }
        for unit in &mut self.party {
            if !selected.contains(&unit.unit_id) {
                continue;
            }
            let destination = if unit.patrol_returning {
                self.seed.map.party_start
            } else {
                unit.patrol_target.unwrap_or(target)
            };
            if distance(unit.position, destination) <= 1 {
                unit.patrol_returning = !unit.patrol_returning;
            }
        }
    }

    fn resolve_stance_fire(&mut self) {
        if matches!(
            self.current_order_kind(),
            RtsOrderKind::Attack | RtsOrderKind::FocusFire | RtsOrderKind::AttackMove
        ) {
            return;
        }
        let guard = self
            .party
            .iter()
            .filter(|unit| unit.alive() && unit.stance == RtsUnitStance::Guard)
            .filter(|unit| {
                self.enemies.iter().any(|enemy| {
                    enemy.alive()
                        && self.visible_tiles.contains(&enemy.position)
                        && distance(unit.position, enemy.position) <= unit.attack_range()
                })
            })
            .map(|unit| unit.unit_id.clone())
            .collect::<BTreeSet<_>>();
        if !guard.is_empty() {
            self.party_attack(&guard, None);
        }
        let aggressive = self
            .party
            .iter()
            .filter(|unit| unit.alive() && unit.stance == RtsUnitStance::Aggressive)
            .map(|unit| unit.unit_id.clone())
            .collect::<BTreeSet<_>>();
        let target = self
            .enemies
            .iter()
            .filter(|enemy| enemy.alive() && self.visible_tiles.contains(&enemy.position))
            .min_by_key(|enemy| {
                self.party
                    .iter()
                    .filter(|unit| aggressive.contains(&unit.unit_id))
                    .map(|unit| distance(unit.position, enemy.position))
                    .min()
                    .unwrap_or(i16::MAX)
            })
            .map(|enemy| (enemy.unit_id.clone(), enemy.position));
        if let Some((target_id, position)) = target {
            self.move_selected_toward(&aggressive, position, 1, None);
            self.party_attack(&aggressive, Some(&target_id));
        }
    }

    fn party_attack(&mut self, selected: &BTreeSet<String>, requested: Option<&str>) {
        for attacker_index in 0..self.party.len() {
            if !self.party[attacker_index].alive()
                || !selected.contains(&self.party[attacker_index].unit_id)
                || !self
                    .tick
                    .is_multiple_of(self.party[attacker_index].attack_interval_ticks as u64)
            {
                continue;
            }
            let attacker = &self.party[attacker_index];
            let requested_structure = requested.and_then(|id| {
                self.enemy_structures
                    .iter()
                    .position(|structure| structure.structure_id == id && structure.alive())
            });
            let target_index = requested
                .and_then(|id| {
                    self.enemies
                        .iter()
                        .position(|enemy| enemy.unit_id == id && enemy.alive())
                })
                .or_else(|| {
                    if requested_structure.is_some() {
                        return None;
                    }
                    self.enemies
                        .iter()
                        .enumerate()
                        .filter(|(_, enemy)| enemy.alive())
                        .min_by_key(|(_, enemy)| distance(attacker.position, enemy.position))
                        .map(|(index, _)| index)
                });
            if let Some(target_index) = target_index {
                if distance(
                    self.party[attacker_index].position,
                    self.enemies[target_index].position,
                ) <= self.party[attacker_index].attack_range()
                {
                    let intel_bonus = if self.recon_bonus_ticks > 0 {
                        i64::from(self.intel_level) * 2
                    } else {
                        0
                    };
                    let veteran_bonus = i64::from(self.party[attacker_index].veteran_rank) * 2;
                    let damage = (self.party[attacker_index].damage + intel_bonus + veteran_bonus
                        - self.enemies[target_index].armor)
                        .max(1);
                    let was_alive = self.enemies[target_index].alive();
                    if !deterministic_evade(
                        self.tick,
                        target_index + simulation_salt(&self.seed) as usize,
                        self.enemies[target_index].evasion_permille,
                    ) {
                        self.enemies[target_index].hp -= damage;
                    }
                    self.party[attacker_index].attacks_made += 1;
                    if was_alive && !self.enemies[target_index].alive() {
                        self.player_score = self.player_score.saturating_add(100);
                        self.party[attacker_index].confirmed_kills =
                            self.party[attacker_index].confirmed_kills.saturating_add(1);
                        self.party[attacker_index].veteran_rank =
                            match self.party[attacker_index].confirmed_kills {
                                0..=1 => self.party[attacker_index].veteran_rank,
                                2..=4 => self.party[attacker_index].veteran_rank.max(1),
                                5..=8 => self.party[attacker_index].veteran_rank.max(2),
                                _ => 3,
                            };
                    }
                    self.event_count += 1;
                }
            } else if let Some(structure_index) = requested_structure.or_else(|| {
                self.enemy_structures
                    .iter()
                    .enumerate()
                    .filter(|(_, structure)| structure.alive())
                    .min_by_key(|(_, structure)| {
                        distance(self.party[attacker_index].position, structure.position)
                    })
                    .map(|(index, _)| index)
            }) {
                if distance(
                    self.party[attacker_index].position,
                    self.enemy_structures[structure_index].position,
                ) <= self.party[attacker_index].attack_range() + 1
                {
                    let siege_bonus = if self.party[attacker_index]
                        .skill_ids
                        .iter()
                        .any(|skill| skill.contains("demolition") || skill.contains("siege"))
                    {
                        12
                    } else {
                        0
                    };
                    self.enemy_structures[structure_index].hp -=
                        (self.party[attacker_index].damage + siege_bonus).max(1);
                    self.party[attacker_index].attacks_made += 1;
                    self.event_count += 1;
                }
            } else if self.current_objective_kind() == Some(ObjectiveKind::Destroy)
                && distance(self.party[attacker_index].position, self.seed.map.objective)
                    <= self.party[attacker_index].attack_range() + 1
                && self.relay_guard_hp > 0
            {
                let resource_bonus = i64::from((self.resources_gathered / 40).min(3));
                self.relay_guard_hp -= (self.party[attacker_index].damage + resource_bonus).max(1);
                self.party[attacker_index].attacks_made += 1;
                self.event_count += 1;
            }
        }
    }

    fn resolve_harvest(&mut self, selected: &BTreeSet<String>, order: &RtsFrameOrder) {
        let Some(node_index) = order
            .target_actor_id
            .as_deref()
            .and_then(|id| {
                self.resource_nodes
                    .iter()
                    .position(|node| node.node_id == id)
            })
            .or(if self.resource_nodes.is_empty() {
                None
            } else {
                Some(0)
            })
        else {
            return;
        };
        let node_position = self.resource_nodes[node_index].position;
        let node_depleted = self.resource_nodes[node_index].remaining == 0;
        let returning = self
            .party
            .iter()
            .filter(|unit| {
                unit.alive()
                    && selected.contains(&unit.unit_id)
                    && (unit.cargo >= unit.cargo_capacity || (node_depleted && unit.cargo > 0))
            })
            .map(|unit| unit.unit_id.clone())
            .collect::<BTreeSet<_>>();
        let gathering = selected
            .difference(&returning)
            .cloned()
            .collect::<BTreeSet<_>>();
        if !gathering.is_empty() && !node_depleted {
            self.move_selected_toward(&gathering, node_position, 1, None);
        }
        if !returning.is_empty() {
            self.move_selected_toward(&returning, self.seed.map.party_start, 1, None);
        }
        if self.tick.is_multiple_of(10) {
            for index in 0..self.party.len() {
                if !gathering.contains(&self.party[index].unit_id)
                    || distance(self.party[index].position, node_position) > 1
                    || self.resource_nodes[node_index].remaining == 0
                {
                    continue;
                }
                let room = self.party[index]
                    .cargo_capacity
                    .saturating_sub(self.party[index].cargo);
                let harvested = 4_u32
                    .min(room)
                    .min(self.resource_nodes[node_index].remaining);
                self.party[index].cargo = self.party[index].cargo.saturating_add(harvested);
                self.resource_nodes[node_index].remaining -= harvested;
                self.party[index].energy =
                    (self.party[index].energy + 4).min(self.party[index].max_energy);
                self.event_count += u64::from(harvested > 0);
            }
        }
        for unit in &mut self.party {
            if returning.contains(&unit.unit_id)
                && distance(unit.position, self.seed.map.party_start) <= 1
                && unit.cargo > 0
            {
                let deposited = std::mem::take(&mut unit.cargo);
                self.resources_gathered = self.resources_gathered.saturating_add(deposited);
                self.resources_available = self.resources_available.saturating_add(deposited);
                self.player_score = self.player_score.saturating_add(deposited);
                self.event_count += 1;
            }
        }
    }

    fn resolve_capture(&mut self, selected: &BTreeSet<String>) {
        if self.current_objective_kind() != Some(ObjectiveKind::Capture)
            || self.relay_guard_hp > 0
            || self.enemies.iter().any(SimUnit::alive)
        {
            return;
        }
        let holders = self
            .party
            .iter()
            .filter(|unit| {
                unit.alive()
                    && selected.contains(&unit.unit_id)
                    && distance(unit.position, self.seed.map.objective) <= 2
            })
            .count() as u32;
        if holders > 0 {
            self.relay_capture_ticks = self.relay_capture_ticks.saturating_add(holders.min(2));
            let aftershock = is_aftershock_map(&self.seed.map_id);
            let thresholds: &[u32] = &[200, 400];
            if let Some(threshold) = thresholds.get(self.reinforcement_wave as usize) {
                if self.relay_capture_ticks >= *threshold {
                    self.spawn_reinforcement_wave(aftershock);
                }
            }
        }
    }

    fn resolve_field_aid(&mut self, order: &RtsFrameOrder) -> Result<(), SimError> {
        self.spend_resources(FIELD_AID_COST, "field aid")?;
        let selected = order
            .subject_actor_ids
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        for unit in &mut self.party {
            if unit.alive() && selected.contains(unit.unit_id.as_str()) {
                unit.hp = (unit.hp + 110).min(unit.max_hp);
            }
        }
        self.event_count += 1;
        Ok(())
    }

    fn resolve_repair(&mut self, order: &RtsFrameOrder) -> Result<(), SimError> {
        if let Some(index) = order.target_actor_id.as_deref().and_then(|target| {
            self.structures
                .iter()
                .position(|structure| structure.structure_id == target && structure.alive())
        }) {
            if self.structures[index].hp >= self.structures[index].max_hp {
                return Err(SimError::Order(
                    "structure is already fully repaired".to_string(),
                ));
            }
            self.spend_resources(10, "structure repair")?;
            self.structures[index].hp =
                (self.structures[index].hp + 180).min(self.structures[index].max_hp);
            self.event_count += 1;
            Ok(())
        } else {
            self.resolve_field_aid(order)
        }
    }

    fn resolve_fortify(&mut self, order: &RtsFrameOrder) -> Result<(), SimError> {
        let rule = order.target_rule_id.as_deref().unwrap_or("field_barricade");
        let kind = SimStructureKind::from_rule_id(rule)
            .ok_or_else(|| SimError::Order(format!("unknown structure rule {rule}")))?;
        if kind == SimStructureKind::CommandPost {
            return Err(SimError::Order(
                "additional command posts cannot be field-built".to_string(),
            ));
        }
        let definition = kind.definition();
        if definition
            .faction
            .is_some_and(|faction| faction != self.seed.skirmish.player_faction)
        {
            return Err(SimError::Order(format!(
                "{} belongs to the opposing faction",
                definition.id
            )));
        }
        let base_cost = definition.cost;
        let hp = i64::from(definition.hp);
        let target = order
            .target_tile
            .map(|tile| BattleGridPoint::new(tile.x as i16, tile.y as i16))
            .ok_or_else(|| SimError::Order("structure target is missing".to_string()))?;
        let in_build_radius = self
            .structures
            .iter()
            .filter(|structure| structure.alive())
            .any(|structure| distance(structure.position, target) <= 8)
            || self.party.iter().any(|unit| {
                unit.alive()
                    && order.subject_actor_ids.contains(&unit.unit_id)
                    && distance(unit.position, target) <= 6
            });
        if !in_build_radius {
            return Err(SimError::Order(
                "structure target is outside build radius".to_string(),
            ));
        }
        if self
            .structures
            .iter()
            .any(|structure| structure.alive() && structure.position == target)
            || self
                .enemies
                .iter()
                .any(|enemy| enemy.alive() && enemy.position == target)
        {
            return Err(SimError::Order(
                "structure target tile is occupied".to_string(),
            ));
        }
        let cost = ((base_cost * u32::from(self.seed.field_build_cost_permille)) / 1000).max(1);
        self.spend_resources(cost, "structure construction")?;
        self.structures.push(SimStructure {
            structure_id: format!("{rule}-{}", self.event_count + 1),
            kind,
            position: target,
            hp,
            max_hp: hp,
        });
        let selected = order
            .subject_actor_ids
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        for unit in &mut self.party {
            if unit.alive() && selected.contains(unit.unit_id.as_str()) {
                unit.guard_ticks = unit.guard_ticks.max(240);
            }
        }
        self.event_count += 1;
        Ok(())
    }

    fn resolve_recon(&mut self, order: &RtsFrameOrder) -> Result<(), SimError> {
        self.spend_resources(RECON_COST, "recon sweep")?;
        self.intel_level = self.intel_level.saturating_add(1).min(3);
        self.recon_bonus_ticks = 300;
        self.recon_focus = order
            .target_tile
            .map(|tile| BattleGridPoint::new(tile.x as i16, tile.y as i16));
        self.refresh_visibility();
        self.event_count += 1;
        Ok(())
    }

    fn queue_job_from_order(&mut self, order: &RtsFrameOrder) -> Result<(), SimError> {
        let rule = order.target_rule_id.as_deref().unwrap_or_default();
        let kind = match (order.kind, rule) {
            (RtsOrderKind::Train, "field_medic") => SimJobKind::TrainMedic,
            (RtsOrderKind::Train, "field_support_drone") => SimJobKind::TrainSupport,
            (RtsOrderKind::Train, rule) if UNIT_ROSTER.iter().any(|unit| unit.id == rule) => {
                SimJobKind::TrainRosterUnit
            }
            (RtsOrderKind::Research, "signal_optics") => SimJobKind::ResearchOptics,
            (RtsOrderKind::Research, "sensor_net") => SimJobKind::ResearchSensorNet,
            (RtsOrderKind::Research, "field_medicine") => SimJobKind::ResearchFieldMedicine,
            (RtsOrderKind::Research, "field_logistics") => SimJobKind::ResearchLogistics,
            (RtsOrderKind::Research, "wayfinder_drills") => SimJobKind::ResearchWayfinderDrills,
            (RtsOrderKind::Research, "rapid_mustering") => SimJobKind::ResearchRapidMustering,
            (RtsOrderKind::Upgrade, "field_armor") => SimJobKind::UpgradeFieldArmor,
            (RtsOrderKind::Upgrade, "siege_drills") => SimJobKind::UpgradeSiegeDrills,
            (RtsOrderKind::Upgrade, "reactive_plating") => SimJobKind::UpgradeReactivePlating,
            (RtsOrderKind::Upgrade, "relay_arms") => SimJobKind::UpgradeRelayArms,
            _ => return Err(SimError::Order("unsupported job order".to_string())),
        };
        self.queue_job(order, kind)
    }

    fn queue_job(&mut self, order: &RtsFrameOrder, kind: SimJobKind) -> Result<(), SimError> {
        let rule_id = order
            .target_rule_id
            .clone()
            .ok_or_else(|| SimError::Order("job rule is required".to_string()))?;
        if !matches!(
            kind,
            SimJobKind::TrainSupport | SimJobKind::TrainMedic | SimJobKind::TrainRosterUnit
        ) && self.jobs.iter().any(|job| job.kind == kind)
        {
            return Err(SimError::Order(
                "that production or technology job is already queued".to_string(),
            ));
        }
        let requested_supply = match kind {
            SimJobKind::TrainSupport | SimJobKind::TrainMedic => 1,
            SimJobKind::TrainRosterUnit => UNIT_ROSTER
                .iter()
                .find(|unit| unit.id == rule_id)
                .map(|unit| unit.supply)
                .unwrap_or(1),
            _ => 0,
        };
        if requested_supply > 0
            && self.supply_used().saturating_add(requested_supply) > self.supply_cap()
        {
            return Err(SimError::Order(
                "unit production is supply blocked".to_string(),
            ));
        }
        let workshop_ready = self.structures.iter().any(|structure| {
            structure.alive() && structure.kind == SimStructureKind::FieldWorkshop
        });
        if matches!(
            kind,
            SimJobKind::TrainMedic
                | SimJobKind::TrainRosterUnit
                | SimJobKind::ResearchOptics
                | SimJobKind::UpgradeRelayArms
                | SimJobKind::UpgradeFieldArmor
                | SimJobKind::ResearchSensorNet
                | SimJobKind::ResearchFieldMedicine
                | SimJobKind::UpgradeSiegeDrills
                | SimJobKind::UpgradeReactivePlating
                | SimJobKind::ResearchWayfinderDrills
                | SimJobKind::ResearchRapidMustering
        ) && !workshop_ready
        {
            return Err(SimError::Order(
                "a powered field workshop prerequisite is missing".to_string(),
            ));
        }
        let tech_definition = match kind {
            SimJobKind::ResearchLogistics
            | SimJobKind::ResearchOptics
            | SimJobKind::UpgradeRelayArms
            | SimJobKind::UpgradeFieldArmor
            | SimJobKind::ResearchSensorNet
            | SimJobKind::ResearchFieldMedicine
            | SimJobKind::UpgradeSiegeDrills
            | SimJobKind::UpgradeReactivePlating
            | SimJobKind::ResearchWayfinderDrills
            | SimJobKind::ResearchRapidMustering => Some(
                TECH_TREE
                    .iter()
                    .find(|tech| tech.id == rule_id)
                    .ok_or_else(|| SimError::Order(format!("unknown technology {rule_id}")))?,
            ),
            _ => None,
        };
        if let Some(tech) = tech_definition {
            if tech
                .faction
                .is_some_and(|faction| faction != self.seed.skirmish.player_faction)
            {
                return Err(SimError::Order(format!(
                    "{} belongs to the opposing faction",
                    tech.id
                )));
            }
            if tech
                .prerequisite
                .is_some_and(|required| !self.researched_techs.contains(required))
            {
                return Err(SimError::Order(format!(
                    "research {} before {}",
                    tech.prerequisite.unwrap_or_default(),
                    tech.id
                )));
            }
        }
        let tech_cost = tech_definition.map(|tech| tech.cost).unwrap_or_default();
        let (cost, duration, label) = match kind {
            SimJobKind::TrainSupport => (TRAIN_SUPPORT_COST, 80, "support production"),
            SimJobKind::TrainMedic => (TRAIN_SUPPORT_COST + 10, 95, "field medic production"),
            SimJobKind::TrainRosterUnit => {
                let unit = UNIT_ROSTER
                    .iter()
                    .find(|unit| unit.id == rule_id)
                    .ok_or_else(|| SimError::Order("unknown faction unit".to_string()))?;
                if unit.faction != self.seed.skirmish.player_faction {
                    return Err(SimError::Order(format!(
                        "{} belongs to the opposing faction",
                        unit.id
                    )));
                }
                let duration = if self.researched_techs.contains("rapid_mustering") {
                    60
                } else {
                    90
                };
                (unit.cost, duration, "faction roster production")
            }
            SimJobKind::ResearchLogistics => {
                if self.researched_techs.contains("field_logistics") {
                    return Err(SimError::Order(
                        "field logistics is already researched".to_string(),
                    ));
                }
                (tech_cost, 70, "field logistics research")
            }
            SimJobKind::ResearchOptics => {
                if self.researched_techs.contains("signal_optics") {
                    return Err(SimError::Order(
                        "signal optics is already researched".to_string(),
                    ));
                }
                (tech_cost, 90, "signal optics research")
            }
            SimJobKind::UpgradeRelayArms => {
                if self.upgrade_level >= 3 {
                    return Err(SimError::Order(
                        "relay arms upgrade cap reached".to_string(),
                    ));
                }
                (tech_cost, 60, "relay arms upgrade")
            }
            SimJobKind::UpgradeFieldArmor => {
                if self.armor_upgrade_level >= 3 {
                    return Err(SimError::Order(
                        "field armor upgrade cap reached".to_string(),
                    ));
                }
                (tech_cost, 75, "field armor upgrade")
            }
            SimJobKind::ResearchSensorNet => {
                if self.researched_techs.contains("sensor_net") {
                    return Err(SimError::Order(
                        "sensor net is already researched".to_string(),
                    ));
                }
                (tech_cost, 100, "sensor net research")
            }
            SimJobKind::ResearchFieldMedicine => {
                if self.researched_techs.contains("field_medicine") {
                    return Err(SimError::Order(
                        "field medicine is already researched".to_string(),
                    ));
                }
                (tech_cost, 100, "field medicine research")
            }
            SimJobKind::UpgradeSiegeDrills => (tech_cost, 90, "siege drills upgrade"),
            SimJobKind::UpgradeReactivePlating => (tech_cost, 90, "reactive plating upgrade"),
            SimJobKind::ResearchWayfinderDrills => {
                if self.researched_techs.contains("wayfinder_drills") {
                    return Err(SimError::Order(
                        "wayfinder drills are already researched".to_string(),
                    ));
                }
                (tech_cost, 80, "wayfinder drills research")
            }
            SimJobKind::ResearchRapidMustering => {
                if self.researched_techs.contains("rapid_mustering") {
                    return Err(SimError::Order(
                        "rapid mustering is already researched".to_string(),
                    ));
                }
                (tech_cost, 80, "rapid mustering research")
            }
        };
        self.spend_resources(cost, label)?;
        let target = order
            .target_tile
            .map(|tile| BattleGridPoint::new(tile.x as i16, tile.y as i16))
            .unwrap_or(self.seed.map.party_start);
        self.jobs.push(SimJob {
            job_id: format!(
                "{}-{}",
                order.queue_id.as_deref().unwrap_or("field"),
                self.tick
            ),
            kind,
            rule_id,
            remaining_ticks: duration,
            target,
            cost,
            paused: false,
        });
        self.event_count += 1;
        Ok(())
    }

    fn job_index(&self, order: &RtsFrameOrder) -> Result<usize, SimError> {
        let job_id = order.queue_id.as_deref().unwrap_or_default();
        self.jobs
            .iter()
            .position(|job| job.job_id == job_id)
            .ok_or_else(|| SimError::Order(format!("job {job_id} was not found")))
    }

    fn cancel_job(&mut self, order: &RtsFrameOrder) -> Result<(), SimError> {
        let index = self.job_index(order)?;
        let job = self.jobs.remove(index);
        let refund = job.cost / 2;
        self.resources_spent = self.resources_spent.saturating_sub(refund);
        self.resources_available = self.resources_available.saturating_add(refund);
        Ok(())
    }

    fn set_job_paused(&mut self, order: &RtsFrameOrder, paused: bool) -> Result<(), SimError> {
        let index = self.job_index(order)?;
        if self.jobs[index].paused == paused {
            return Err(SimError::Order("job pause state is unchanged".to_string()));
        }
        self.jobs[index].paused = paused;
        Ok(())
    }

    fn promote_job(&mut self, order: &RtsFrameOrder) -> Result<(), SimError> {
        let index = self.job_index(order)?;
        if index == 0 {
            return Err(SimError::Order("job is already first in queue".to_string()));
        }
        let job = self.jobs.remove(index);
        self.jobs.insert(0, job);
        Ok(())
    }

    fn set_rally(&mut self, order: &RtsFrameOrder) -> Result<(), SimError> {
        let index = self.job_index(order)?;
        let tile = order.target_tile.expect("validated rally tile");
        let target = BattleGridPoint::new(tile.x as i16, tile.y as i16);
        if !self.seed.map.passable(target) {
            return Err(SimError::Order("rally point is blocked".to_string()));
        }
        self.jobs[index].target = target;
        Ok(())
    }

    fn process_jobs(&mut self) {
        let powered = !self.low_power();
        if let Some(job) = self.jobs.first_mut().filter(|job| !job.paused && powered) {
            job.remaining_ticks = job.remaining_ticks.saturating_sub(1);
        }
        let completed = self
            .jobs
            .iter()
            .filter(|job| job.remaining_ticks == 0)
            .cloned()
            .collect::<Vec<_>>();
        self.jobs.retain(|job| job.remaining_ticks > 0);
        for job in completed {
            match job.kind {
                SimJobKind::TrainSupport => {
                    let position = self.unoccupied_spawn_tile(job.target);
                    self.support_units.push(SupportUnit {
                        unit_id: format!("field_support_{}", self.support_units.len() + 1),
                        archetype_id: "field_support_drone".to_string(),
                        role: "support".to_string(),
                        position,
                        hp: 240,
                        damage: 18 + i64::from(self.upgrade_level) * 5,
                        armor: 1,
                        attack_range: 4,
                        ability_cooldown_ticks: 0,
                        attack_interval_ticks: 18,
                        supply: 1,
                    });
                }
                SimJobKind::TrainMedic => {
                    let position = self.unoccupied_spawn_tile(job.target);
                    self.support_units.push(SupportUnit {
                        unit_id: format!("field_medic_{}", self.support_units.len() + 1),
                        archetype_id: "field_medic".to_string(),
                        role: "medic".to_string(),
                        position,
                        hp: 210,
                        damage: 6,
                        armor: 0,
                        attack_range: 3,
                        ability_cooldown_ticks: 0,
                        attack_interval_ticks: 20,
                        supply: 1,
                    });
                }
                SimJobKind::TrainRosterUnit => {
                    let unit = UNIT_ROSTER
                        .iter()
                        .find(|unit| unit.id == job.rule_id)
                        .expect("queued faction unit remains catalogued");
                    let position = self.unoccupied_spawn_tile(job.target);
                    self.support_units.push(SupportUnit {
                        unit_id: format!("{}_{}", unit.id, self.support_units.len() + 1),
                        archetype_id: unit.id.to_string(),
                        role: unit.role.to_string(),
                        position,
                        hp: i64::from(unit.hp),
                        damage: i64::from(unit.damage) + i64::from(self.upgrade_level) * 3,
                        armor: i64::from(unit.supply),
                        attack_range: match unit.ability() {
                            UnitAbility::RevealPulse
                            | UnitAbility::ArcVolley
                            | UnitAbility::SignalJam => 5,
                            UnitAbility::SuppressionBlast | UnitAbility::DemolitionCharge => 4,
                            _ => 3,
                        },
                        ability_cooldown_ticks: 0,
                        attack_interval_ticks: 16 + u32::from(unit.supply) * 3,
                        supply: unit.supply,
                    });
                }
                SimJobKind::ResearchLogistics => {
                    self.researched_techs.insert("field_logistics".to_string());
                }
                SimJobKind::ResearchOptics => {
                    self.researched_techs.insert("signal_optics".to_string());
                    self.intel_level = self.intel_level.saturating_add(1).min(3);
                }
                SimJobKind::UpgradeRelayArms => {
                    self.researched_techs.insert("relay_arms".to_string());
                    self.upgrade_level = self.upgrade_level.saturating_add(1).min(3);
                    for unit in &mut self.party {
                        unit.damage += 3;
                        unit.armor += 1;
                    }
                    for support in &mut self.support_units {
                        support.damage += 5;
                    }
                }
                SimJobKind::UpgradeFieldArmor => {
                    self.researched_techs.insert("field_armor".to_string());
                    self.armor_upgrade_level = self.armor_upgrade_level.saturating_add(1).min(3);
                    for unit in &mut self.party {
                        unit.armor += 2;
                        unit.max_hp += 25;
                        unit.hp += 25;
                    }
                }
                SimJobKind::ResearchSensorNet => {
                    self.researched_techs.insert("sensor_net".to_string());
                    self.intel_level = 3;
                }
                SimJobKind::ResearchFieldMedicine => {
                    self.researched_techs.insert("field_medicine".to_string());
                    for unit in &mut self.party {
                        unit.max_energy += 20;
                        unit.energy += 20;
                    }
                }
                SimJobKind::UpgradeSiegeDrills => {
                    self.researched_techs.insert("siege_drills".to_string());
                    for unit in &mut self.party {
                        unit.damage += 4;
                    }
                    self.relay_guard_hp = self.relay_guard_hp.saturating_sub(150);
                }
                SimJobKind::UpgradeReactivePlating => {
                    self.researched_techs.insert("reactive_plating".to_string());
                    for unit in &mut self.party {
                        unit.armor += 3;
                    }
                }
                SimJobKind::ResearchWayfinderDrills => {
                    self.researched_techs.insert("wayfinder_drills".to_string());
                    for unit in &mut self.party {
                        unit.move_speed_milli += 120;
                        unit.evasion_permille = unit.evasion_permille.saturating_add(25).min(400);
                    }
                }
                SimJobKind::ResearchRapidMustering => {
                    self.researched_techs.insert("rapid_mustering".to_string());
                }
            }
            self.event_count += 1;
        }
    }

    fn unoccupied_spawn_tile(&self, preferred: BattleGridPoint) -> BattleGridPoint {
        let occupied = self
            .party
            .iter()
            .chain(&self.enemies)
            .filter(|unit| unit.alive())
            .map(|unit| unit.position)
            .chain(self.support_units.iter().map(|unit| unit.position))
            .collect::<BTreeSet<_>>();
        let mut frontier = VecDeque::from([preferred]);
        let mut visited = BTreeSet::from([preferred]);
        while let Some(tile) = frontier.pop_front() {
            if self.seed.map.passable(tile) && !occupied.contains(&tile) {
                return tile;
            }
            for next in neighbors(tile) {
                if self.seed.map.in_bounds(next) && visited.insert(next) {
                    frontier.push_back(next);
                }
            }
        }
        self.seed.map.party_start
    }

    fn resolve_support_fire(&mut self) {
        for support_index in 0..self.support_units.len() {
            if !self
                .tick
                .is_multiple_of(self.support_units[support_index].attack_interval_ticks as u64)
            {
                continue;
            }
            let ability = UNIT_ROSTER
                .iter()
                .find(|unit| unit.id == self.support_units[support_index].archetype_id)
                .map(UnitArchetype::ability);
            if self.support_units[support_index].ability_cooldown_ticks == 0 {
                match ability {
                    Some(UnitAbility::RevealPulse) => {
                        self.recon_bonus_ticks = self.recon_bonus_ticks.max(120);
                        self.intel_level = self.intel_level.max(2);
                    }
                    Some(UnitAbility::GuardWall) => {
                        for unit in &mut self.party {
                            if distance(unit.position, self.support_units[support_index].position)
                                <= 3
                            {
                                unit.guard_ticks = unit.guard_ticks.max(50);
                            }
                        }
                    }
                    Some(UnitAbility::FieldRepair) => {
                        if let Some(structure) = self
                            .structures
                            .iter_mut()
                            .filter(|structure| {
                                structure.alive() && structure.hp < structure.max_hp
                            })
                            .min_by_key(|structure| structure.hp)
                        {
                            structure.hp = (structure.hp + 45).min(structure.max_hp);
                        }
                    }
                    Some(UnitAbility::TriageAura) => {
                        for unit in &mut self.party {
                            if unit.alive() {
                                unit.hp = (unit.hp + 18).min(unit.max_hp);
                            }
                        }
                    }
                    Some(UnitAbility::SmokeDash) => {
                        self.support_units[support_index].armor += 2;
                    }
                    Some(UnitAbility::RetaliationPlate) => {
                        self.support_units[support_index].hp += 20;
                    }
                    Some(UnitAbility::SignalJam) => {
                        self.enemy_ai_budget = self.enemy_ai_budget.saturating_sub(3);
                    }
                    Some(UnitAbility::CommandSurge) => {
                        for support in &mut self.support_units {
                            support.damage += 1;
                        }
                    }
                    Some(
                        UnitAbility::ArcVolley
                        | UnitAbility::SuppressionBlast
                        | UnitAbility::PiercingCharge
                        | UnitAbility::DemolitionCharge,
                    ) => {}
                    None => {}
                }
                if ability.is_some() {
                    self.support_units[support_index].ability_cooldown_ticks = 120;
                    self.event_count += 1;
                }
            }
            if self.support_units[support_index].role == "medic" {
                if let Some(target) = self
                    .party
                    .iter_mut()
                    .filter(|unit| unit.alive() && unit.hp < unit.max_hp)
                    .min_by_key(|unit| unit.hp * 100 / unit.max_hp.max(1))
                {
                    target.hp = (target.hp + 24).min(target.max_hp);
                    self.event_count += 1;
                }
                continue;
            }
            let target = self
                .enemies
                .iter()
                .enumerate()
                .filter(|(_, enemy)| enemy.alive())
                .filter(|(_, enemy)| {
                    distance(self.support_units[support_index].position, enemy.position)
                        <= self.support_units[support_index].attack_range
                })
                .min_by_key(|(_, enemy)| {
                    distance(self.support_units[support_index].position, enemy.position)
                })
                .map(|(index, _)| index);
            if let Some(target) = target {
                let ability_bonus = match ability {
                    Some(UnitAbility::ArcVolley) => 8,
                    Some(UnitAbility::SuppressionBlast) => 10,
                    Some(UnitAbility::PiercingCharge) => self.enemies[target].armor,
                    Some(UnitAbility::DemolitionCharge) => 14,
                    _ => 0,
                };
                self.enemies[target].hp -= self.support_units[support_index].damage + ability_bonus;
                self.event_count += 1;
            } else if self.phase == BattlePhase::Relay
                && self.relay_guard_hp > 0
                && distance(
                    self.support_units[support_index].position,
                    self.seed.map.objective,
                ) <= 5
            {
                self.relay_guard_hp -= self.support_units[support_index].damage;
                self.event_count += 1;
            }
        }
    }

    fn resolve_structure_functions(&mut self) {
        if self.tick.is_multiple_of(20)
            && self.structures.iter().any(|structure| {
                structure.alive() && structure.kind == SimStructureKind::SensorTower
            })
        {
            self.intel_level = 3;
            self.recon_bonus_ticks = self.recon_bonus_ticks.max(80);
        }
        if self.tick.is_multiple_of(40)
            && self.structures.iter().any(|structure| {
                structure.alive() && structure.kind == SimStructureKind::FieldHospital
            })
        {
            for unit in &mut self.party {
                if unit.alive() {
                    unit.hp = (unit.hp + 12).min(unit.max_hp);
                }
            }
        }
        if self
            .structures
            .iter()
            .any(|structure| structure.alive() && structure.kind == SimStructureKind::ForwardRally)
        {
            if let Some(job) = self.jobs.first_mut().filter(|job| !job.paused) {
                job.remaining_ticks = job.remaining_ticks.saturating_sub(1);
            }
        }
        if self.tick.is_multiple_of(50)
            && self
                .structures
                .iter()
                .any(|structure| structure.alive() && structure.kind == SimStructureKind::AshBeacon)
        {
            if let Some(enemy) = self.enemies.iter_mut().find(|enemy| enemy.alive()) {
                enemy.hp -= 18;
            }
        }
        if self.tick.is_multiple_of(80)
            && self.structures.iter().any(|structure| {
                structure.alive() && structure.kind == SimStructureKind::SiegeFoundry
            })
        {
            for support in &mut self.support_units {
                if matches!(support.role.as_str(), "siege" | "heavy") {
                    support.damage += 1;
                }
            }
        }
    }

    fn resolve_enemy_structure_functions(&mut self) {
        if !self.seed.skirmish.enabled {
            return;
        }
        if self.tick.is_multiple_of(20)
            && self.enemy_structures.iter().any(|structure| {
                structure.alive() && structure.kind == SimStructureKind::SensorTower
            })
        {
            self.intel_level = self.intel_level.saturating_sub(1);
            self.recon_bonus_ticks = self.recon_bonus_ticks.min(20);
        }
        if self.tick.is_multiple_of(40)
            && self.enemy_structures.iter().any(|structure| {
                structure.alive() && structure.kind == SimStructureKind::FieldHospital
            })
        {
            for unit in &mut self.enemies {
                if unit.alive() {
                    unit.hp = (unit.hp + 12).min(unit.max_hp);
                }
            }
        }
        if self
            .enemy_structures
            .iter()
            .any(|structure| structure.alive() && structure.kind == SimStructureKind::ForwardRally)
        {
            if let Some(job) = self
                .enemy_jobs
                .first_mut()
                .filter(|job| job.kind != EnemyJobKind::Build)
            {
                job.remaining_ticks = job.remaining_ticks.saturating_sub(1);
            }
        }
        if self.tick.is_multiple_of(50)
            && self
                .enemy_structures
                .iter()
                .any(|structure| structure.alive() && structure.kind == SimStructureKind::AshBeacon)
        {
            if let Some(unit) = self.party.iter_mut().find(|unit| unit.alive()) {
                unit.hp -= 18;
            }
        }
        if self.tick.is_multiple_of(80)
            && self.enemy_structures.iter().any(|structure| {
                structure.alive() && structure.kind == SimStructureKind::SiegeFoundry
            })
        {
            for unit in &mut self.enemies {
                if unit.alive() && matches!(unit.role.as_str(), "siege" | "heavy") {
                    unit.damage += 1;
                }
            }
        }
    }

    fn spend_resources(&mut self, cost: u32, label: &str) -> Result<(), SimError> {
        if self.resources_available < cost {
            return Err(SimError::Order(format!(
                "{label} requires {cost} field resources"
            )));
        }
        self.resources_available -= cost;
        self.resources_spent = self.resources_spent.saturating_add(cost);
        Ok(())
    }

    fn spawn_reinforcement_wave(&mut self, aftershock: bool) {
        self.reinforcement_wave = self.reinforcement_wave.saturating_add(1);
        self.enemy_tactics_level = self.enemy_tactics_level.saturating_add(1).min(3);
        let count = if aftershock { 3 } else { 2 };
        let scale = 100 + i64::from(self.reinforcement_wave) * 8 + if aftershock { 18 } else { 0 };
        for index in 0..count {
            let spawn = &self.seed.map.enemy_spawns
                [(index + self.reinforcement_wave as usize) % self.seed.map.enemy_spawns.len()];
            let spawn_position = self.unoccupied_spawn_tile(spawn.position);
            let role = if index % 2 == 0 { "striker" } else { "warden" };
            let hp = 420 * scale / 100;
            self.enemies.push(SimUnit {
                unit_id: format!("aftershock_wave{}_{}", self.reinforcement_wave, index),
                role: role.to_string(),
                persistent: false,
                skill_ids: Vec::new(),
                max_hp: hp,
                hp,
                damage: (10 + i64::from(self.reinforcement_wave) * 2) * scale / 100,
                armor: 4 + i64::from(self.reinforcement_wave),
                move_speed_milli: 920,
                movement_budget_milli: 0,
                attack_interval_ticks: 20,
                evasion_permille: 35,
                energy: 0,
                max_energy: 0,
                ability_range: 1,
                ability_cooldown_ticks: 0,
                guard_ticks: 0,
                position: spawn_position,
                attacks_made: 0,
                stance: RtsUnitStance::Aggressive,
                patrol_anchor: None,
                patrol_target: None,
                patrol_returning: false,
                cargo: 0,
                cargo_capacity: WORKER_CARGO_CAPACITY,
                confirmed_kills: 0,
                veteran_rank: 0,
            });
        }
        self.event_count += 1;
    }

    fn resolve_party_ability(&mut self, order: &RtsFrameOrder) -> Result<(), SimError> {
        let selected = order
            .subject_actor_ids
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        let mut activated = 0;
        for index in 0..self.party.len() {
            if !self.party[index].alive()
                || !selected.contains(&self.party[index].unit_id)
                || self.party[index].ability_cooldown_ticks > 0
            {
                continue;
            }
            let skill = signature_skill(&self.party[index]);
            let cost = match skill {
                "iron_guard" => 18,
                "wind_step" => 22,
                "inner_flame" => 28,
                "relay_overcharge" => 24,
                "field_mend" => 26,
                _ => 20,
            };
            if self.party[index].energy < cost {
                continue;
            }
            self.party[index].energy -= cost;
            self.party[index].ability_cooldown_ticks = 120;
            match skill {
                "iron_guard" => self.party[index].guard_ticks = 100,
                "wind_step" => {
                    let target = order
                        .target_tile
                        .map(|tile| BattleGridPoint::new(tile.x as i16, tile.y as i16))
                        .unwrap_or(self.seed.map.approach_point);
                    let occupied = self
                        .party
                        .iter()
                        .chain(&self.enemies)
                        .filter(|unit| unit.alive() && unit.unit_id != self.party[index].unit_id)
                        .map(|unit| unit.position)
                        .collect::<BTreeSet<_>>();
                    for _ in 0..4 {
                        let Some(next) = next_step_toward(
                            &self.seed,
                            self.party[index].position,
                            target,
                            0,
                            &occupied,
                        ) else {
                            break;
                        };
                        self.party[index].position = next;
                    }
                }
                "inner_flame" => {
                    if let Some(target_index) = self
                        .enemies
                        .iter()
                        .enumerate()
                        .filter(|(_, enemy)| enemy.alive())
                        .filter(|(_, enemy)| {
                            distance(self.party[index].position, enemy.position)
                                <= self.party[index].ability_range * 2
                        })
                        .min_by_key(|(_, enemy)| {
                            distance(self.party[index].position, enemy.position)
                        })
                        .map(|(index, _)| index)
                    {
                        self.enemies[target_index].hp -= 110 + self.party[index].damage * 2;
                    } else if self.phase == BattlePhase::Relay
                        && distance(self.party[index].position, self.seed.map.objective)
                            <= self.party[index].ability_range * 2
                    {
                        self.relay_guard_hp -= 150 + self.party[index].damage * 2;
                    }
                }
                "relay_overcharge" => {
                    self.resources_generated = self.resources_generated.saturating_add(20);
                    self.resources_gathered = self.resources_gathered.saturating_add(20);
                    self.resources_available = self.resources_available.saturating_add(20);
                    if self.phase == BattlePhase::Relay {
                        self.relay_guard_hp -= 120;
                    }
                }
                "field_mend" => {
                    for unit in &mut self.party {
                        if unit.alive() {
                            unit.hp = (unit.hp + 90).min(unit.max_hp);
                        }
                    }
                }
                _ => self.party[index].guard_ticks = 60,
            }
            activated += 1;
            self.event_count += 1;
        }
        if activated == 0 {
            return Err(SimError::Order(
                "selected units have no ready signature ability or energy".to_string(),
            ));
        }
        Ok(())
    }

    fn resolve_enemy_workers(&mut self) {
        if !self.seed.skirmish.enabled {
            return;
        }
        let Some(command_position) = self
            .enemy_structures
            .iter()
            .find(|structure| structure.alive() && structure.kind == SimStructureKind::CommandPost)
            .map(|structure| structure.position)
        else {
            return;
        };
        let worker_indices = self
            .enemies
            .iter()
            .enumerate()
            .filter(|(_, unit)| unit.alive() && unit.role == "worker")
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        self.enemy_workers = worker_indices.len().min(u8::MAX as usize) as u8;
        let mut occupied = self
            .party
            .iter()
            .chain(&self.enemies)
            .filter(|unit| unit.alive())
            .map(|unit| unit.position)
            .collect::<BTreeSet<_>>();
        let construction_assignment = self
            .enemy_jobs
            .first()
            .filter(|job| job.kind == EnemyJobKind::Build)
            .and_then(|job| Some((job.builder_id.as_deref()?, job.target?)));
        for index in worker_indices {
            let assigned_site = construction_assignment
                .filter(|(builder_id, _)| *builder_id == self.enemies[index].unit_id.as_str())
                .map(|(_, target)| target);
            let node_index = self
                .resource_nodes
                .iter()
                .enumerate()
                .filter(|(_, node)| node.remaining > 0)
                .min_by_key(|(_, node)| distance(self.enemies[index].position, node.position))
                .map(|(index, _)| index);
            let returning = assigned_site.is_none()
                && (self.enemies[index].cargo >= self.enemies[index].cargo_capacity
                    || (node_index.is_none() && self.enemies[index].cargo > 0));
            let target = if let Some(site) = assigned_site {
                site
            } else if returning {
                command_position
            } else if let Some(node_index) = node_index {
                self.resource_nodes[node_index].position
            } else {
                command_position
            };
            let stop_range = 1;
            self.enemies[index].movement_budget_milli = self.enemies[index]
                .movement_budget_milli
                .saturating_add(self.enemies[index].move_speed_milli);
            if self.enemies[index].movement_budget_milli >= MOVEMENT_TILE_COST
                && distance(self.enemies[index].position, target) > stop_range
            {
                occupied.remove(&self.enemies[index].position);
                if let Some(next) = next_step_toward(
                    &self.seed,
                    self.enemies[index].position,
                    target,
                    stop_range,
                    &occupied,
                ) {
                    self.enemies[index].position = next;
                    self.enemies[index].movement_budget_milli -= MOVEMENT_TILE_COST;
                }
                occupied.insert(self.enemies[index].position);
            }
            if assigned_site.is_some() {
                continue;
            }
            if returning
                && distance(self.enemies[index].position, command_position) <= 1
                && self.enemies[index].cargo > 0
            {
                let deposited = std::mem::take(&mut self.enemies[index].cargo);
                self.enemy_resources_available =
                    self.enemy_resources_available.saturating_add(deposited);
                self.enemy_resources_generated =
                    self.enemy_resources_generated.saturating_add(deposited);
                self.enemy_score = self.enemy_score.saturating_add(deposited);
                self.event_count += 1;
            } else if !returning && self.tick.is_multiple_of(10) {
                if let Some(node_index) = node_index {
                    if distance(
                        self.enemies[index].position,
                        self.resource_nodes[node_index].position,
                    ) <= 1
                    {
                        let room = self.enemies[index]
                            .cargo_capacity
                            .saturating_sub(self.enemies[index].cargo);
                        let gathered = 4_u32
                            .min(room)
                            .min(self.resource_nodes[node_index].remaining);
                        self.enemies[index].cargo =
                            self.enemies[index].cargo.saturating_add(gathered);
                        self.resource_nodes[node_index].remaining -= gathered;
                        self.event_count += u64::from(gathered > 0);
                    }
                }
            }
        }
    }

    fn resolve_enemy_economy(&mut self) {
        if !self.seed.skirmish.enabled {
            return;
        }

        let command_alive = self
            .enemy_structures
            .iter()
            .any(|structure| structure.alive() && structure.kind == SimStructureKind::CommandPost);
        if !command_alive {
            self.enemy_jobs.clear();
            return;
        }

        let worker_alive = self
            .enemies
            .iter()
            .any(|unit| unit.alive() && unit.role == "worker");
        let build_worker_ready = self.enemy_jobs.first().is_some_and(|job| {
            if job.kind != EnemyJobKind::Build {
                return true;
            }
            let (Some(builder_id), Some(target)) = (job.builder_id.as_deref(), job.target) else {
                return false;
            };
            self.enemies.iter().any(|unit| {
                unit.alive() && unit.unit_id == builder_id && distance(unit.position, target) <= 1
            })
        });
        let enemy_low_power = self.enemy_low_power();
        if let Some(job) = self.enemy_jobs.first_mut().filter(|job| {
            worker_alive
                && build_worker_ready
                && (!enemy_low_power
                    || job.kind == EnemyJobKind::Build
                        && matches!(job.rule_id.as_str(), "relay_generator" | "ash_beacon"))
        }) {
            job.remaining_ticks = job.remaining_ticks.saturating_sub(1);
        }
        if self
            .enemy_jobs
            .first()
            .is_some_and(|job| job.remaining_ticks == 0)
        {
            let job = self.enemy_jobs.remove(0);
            match job.kind {
                EnemyJobKind::Build => {
                    if let Some(kind) = SimStructureKind::from_rule_id(&job.rule_id) {
                        let position = job.target.unwrap_or(self.seed.map.objective);
                        let definition = kind.definition();
                        self.enemy_structures.push(SimStructure {
                            structure_id: format!(
                                "enemy_{}_{}",
                                job.rule_id,
                                self.enemy_structures.len()
                            ),
                            kind,
                            position,
                            hp: i64::from(definition.hp),
                            max_hp: i64::from(definition.hp),
                        });
                    }
                }
                EnemyJobKind::Research => {
                    self.enemy_researched_techs.insert(job.rule_id.clone());
                    match job.rule_id.as_str() {
                        "relay_arms" | "siege_drills" => {
                            for enemy in &mut self.enemies {
                                enemy.damage += 3;
                            }
                        }
                        "field_armor" | "reactive_plating" => {
                            for enemy in &mut self.enemies {
                                enemy.armor += 2;
                            }
                        }
                        "rapid_mustering" => {
                            let position = self.unoccupied_spawn_tile(self.seed.map.objective);
                            let worker_index = self.enemy_workers;
                            self.enemies.push(SimUnit {
                                unit_id: format!("enemy_worker_mustered_{worker_index}"),
                                role: "worker".to_string(),
                                persistent: false,
                                skill_ids: vec!["enemy_harvest".to_string()],
                                max_hp: 460,
                                hp: 460,
                                damage: 5,
                                armor: 2,
                                move_speed_milli: 900,
                                movement_budget_milli: 0,
                                attack_interval_ticks: 30,
                                evasion_permille: 25,
                                energy: 0,
                                max_energy: 0,
                                ability_range: 1,
                                ability_cooldown_ticks: 0,
                                guard_ticks: 0,
                                position,
                                attacks_made: 0,
                                stance: RtsUnitStance::Guard,
                                patrol_anchor: None,
                                patrol_target: None,
                                patrol_returning: false,
                                cargo: 0,
                                cargo_capacity: WORKER_CARGO_CAPACITY,
                                confirmed_kills: 0,
                                veteran_rank: 0,
                            });
                            self.enemy_workers = self.enemy_workers.saturating_add(1);
                        }
                        _ => {}
                    }
                }
                EnemyJobKind::Train => {
                    if let Some(unit) = UNIT_ROSTER.iter().find(|unit| unit.id == job.rule_id) {
                        let position = self.unoccupied_spawn_tile(
                            nearest_passable(
                                &self.seed,
                                BattleGridPoint::new(
                                    self.seed.map.objective.x - 3,
                                    self.seed.map.objective.y,
                                ),
                            )
                            .unwrap_or(self.seed.map.objective),
                        );
                        let damage_bonus = if self.enemy_researched_techs.contains("relay_arms") {
                            4
                        } else {
                            0
                        };
                        let armor_bonus = if self.enemy_researched_techs.contains("field_armor") {
                            2
                        } else {
                            0
                        };
                        self.enemies.push(SimUnit {
                            unit_id: format!("enemy_{}_{}", unit.id, self.enemy_build_order_index),
                            role: unit.role.to_string(),
                            persistent: false,
                            skill_ids: vec![unit.ability().rule_id().to_string()],
                            max_hp: i64::from(unit.hp) * 8,
                            hp: i64::from(unit.hp) * 8,
                            damage: i64::from(unit.damage) + damage_bonus,
                            armor: i64::from(unit.supply) * 2 + armor_bonus,
                            move_speed_milli: match unit.ability() {
                                UnitAbility::SmokeDash | UnitAbility::PiercingCharge => 1_050,
                                UnitAbility::CommandSurge | UnitAbility::SuppressionBlast => 700,
                                _ => 850,
                            },
                            movement_budget_milli: 0,
                            attack_interval_ticks: 17 + u32::from(unit.supply) * 3,
                            evasion_permille: if unit.ability() == UnitAbility::SmokeDash {
                                160
                            } else {
                                45
                            },
                            energy: 100,
                            max_energy: 100,
                            ability_range: 4,
                            ability_cooldown_ticks: 0,
                            guard_ticks: 0,
                            position,
                            attacks_made: 0,
                            stance: RtsUnitStance::Aggressive,
                            patrol_anchor: None,
                            patrol_target: None,
                            patrol_returning: false,
                            cargo: 0,
                            cargo_capacity: WORKER_CARGO_CAPACITY,
                            confirmed_kills: 0,
                            veteran_rank: 0,
                        });
                    }
                }
            }
            self.event_count += 1;
        }

        if !self.enemy_jobs.is_empty() || !self.tick.is_multiple_of(20) {
            return;
        }
        let enemy_faction = self.seed.skirmish.enemy_faction;
        let faction_units = UNIT_ROSTER
            .iter()
            .filter(|unit| unit.faction == enemy_faction)
            .collect::<Vec<_>>();
        let faction_structure = match enemy_faction {
            RtsFaction::MirrorCoalition => "field_hospital",
            RtsFaction::AshenCompact => "siege_foundry",
        };
        let faction_tech = match enemy_faction {
            RtsFaction::MirrorCoalition => "wayfinder_drills",
            RtsFaction::AshenCompact => "rapid_mustering",
        };
        let power_rule = if enemy_faction == RtsFaction::AshenCompact {
            "ash_beacon"
        } else {
            "relay_generator"
        };
        let supply_deficit = self.enemy_supply_used() >= self.enemy_supply_cap();
        let power_deficit = self.enemy_low_power();
        let choice = self.enemy_build_order_index % 8;
        let (kind, rule_id, cost, duration) = if power_deficit {
            let definition = SimStructureKind::from_rule_id(power_rule)
                .expect("enemy power rule is catalogued")
                .definition();
            (EnemyJobKind::Build, power_rule, definition.cost, 55)
        } else if supply_deficit {
            let definition = SimStructureKind::SupplyCache.definition();
            (EnemyJobKind::Build, "supply_cache", definition.cost, 55)
        } else {
            match choice {
                0 => (EnemyJobKind::Build, "supply_cache", 25, 55),
                1 if !self.enemy_researched_techs.contains("field_logistics") => {
                    (EnemyJobKind::Research, "field_logistics", 35, 70)
                }
                2 => (EnemyJobKind::Build, faction_structure, 55, 80),
                3 if !self.enemy_researched_techs.contains("relay_arms") => {
                    (EnemyJobKind::Research, "relay_arms", 45, 75)
                }
                5 if !self.enemy_researched_techs.contains(faction_tech) => {
                    (EnemyJobKind::Research, faction_tech, 45, 80)
                }
                6 if !self.enemy_researched_techs.contains("field_armor") => {
                    (EnemyJobKind::Research, "field_armor", 45, 75)
                }
                _ => {
                    let unit = faction_units
                        [(self.enemy_build_order_index as usize) % faction_units.len()];
                    (
                        EnemyJobKind::Train,
                        unit.id,
                        unit.cost,
                        70 + u32::from(unit.supply) * 10,
                    )
                }
            }
        };
        if kind == EnemyJobKind::Train {
            let required_supply = UNIT_ROSTER
                .iter()
                .find(|unit| unit.id == rule_id)
                .map(|unit| unit.supply)
                .unwrap_or(1);
            if self.enemy_supply_used().saturating_add(required_supply) > self.enemy_supply_cap() {
                return;
            }
        }
        if self.enemy_resources_available >= cost {
            let builder_id = (kind == EnemyJobKind::Build)
                .then(|| {
                    self.enemies
                        .iter()
                        .filter(|unit| unit.alive() && unit.role == "worker")
                        .min_by_key(|unit| distance(unit.position, self.seed.map.objective))
                        .map(|unit| unit.unit_id.clone())
                })
                .flatten();
            let target = (kind == EnemyJobKind::Build).then(|| {
                let offset = self.enemy_structures.len() as i16;
                nearest_passable(
                    &self.seed,
                    BattleGridPoint::new(
                        self.seed.map.objective.x - 1 - offset % 4,
                        self.seed.map.objective.y + offset % 3 - 1,
                    ),
                )
                .unwrap_or(self.seed.map.objective)
            });
            self.enemy_resources_available -= cost;
            self.enemy_resources_spent = self.enemy_resources_spent.saturating_add(cost);
            self.enemy_jobs.push(EnemyJob {
                kind,
                rule_id: rule_id.to_string(),
                remaining_ticks: duration,
                cost,
                builder_id,
                target,
                total_ticks: duration,
            });
            self.enemy_build_order_index = self.enemy_build_order_index.saturating_add(1);
            self.event_count += 1;
        }
    }

    fn observe_enemy_ai(&self) -> AiObservation {
        AiObservation {
            tick: self.tick,
            phase: self.phase,
            living_party: self.party.iter().filter(|unit| unit.alive()).count() as u8,
            living_enemies: self.enemies.iter().filter(|unit| unit.alive()).count() as u8,
            wounded_party: self
                .party
                .iter()
                .filter(|unit| unit.alive() && unit.hp * 2 < unit.max_hp)
                .count() as u8,
            party_resources: self.resources_available,
            party_structures: self
                .structures
                .iter()
                .filter(|structure| structure.alive())
                .count() as u8,
            researched_tech_count: self.researched_techs.len() as u8
                + self.upgrade_level
                + self.armor_upgrade_level,
            convoy_active: self.convoy_position.is_some() && self.convoy_hp > 0,
        }
    }

    fn refresh_enemy_ai_plan(&mut self) {
        let (interval, budget_gain) = match self.seed.difficulty {
            CampaignDifficulty::Story => (70, 6),
            CampaignDifficulty::Standard => (50, 8),
            CampaignDifficulty::Veteran => (35, 10),
        };
        if self.tick != 1 && !self.tick.is_multiple_of(interval) {
            return;
        }
        let observation = self.observe_enemy_ai();
        let budget_before = self.enemy_ai_budget.saturating_add(budget_gain).min(40);
        let (requested_goal, cost, reason) = if observation.convoy_active {
            (AiGoal::InterdictConvoy, 8, "escort target is exposed")
        } else if observation.party_resources >= 120 || observation.party_structures >= 4 {
            (AiGoal::RaidEconomy, 7, "player economy is accelerating")
        } else if observation.researched_tech_count > 0 {
            (AiGoal::CounterTech, 6, "player technology is visible")
        } else if observation.living_enemies.saturating_mul(2) <= observation.living_party
            || self.relay_guard_hp * 2 < self.relay_guard_max_hp
        {
            (
                AiGoal::DefendObjective,
                5,
                "enemy force or objective integrity is low",
            )
        } else if self.tick < 300 {
            (AiGoal::Scout, 2, "contact picture is incomplete")
        } else {
            (AiGoal::Assault, 4, "battle line is stable enough to commit")
        };
        let (goal, spent, reason) = if budget_before >= cost {
            (requested_goal, cost, reason.to_string())
        } else {
            (
                AiGoal::Scout,
                2.min(budget_before),
                "budget is insufficient; gathering information".to_string(),
            )
        };
        self.enemy_ai_goal = goal;
        self.enemy_ai_budget = budget_before.saturating_sub(spent);
        self.enemy_ai_decision_index = self.enemy_ai_decision_index.saturating_add(1);
        self.enemy_tactics_level = (self.enemy_ai_decision_index / 2).min(3) as u8;
        self.enemy_ai_history.push(AiDecision {
            index: self.enemy_ai_decision_index,
            goal,
            budget_before,
            budget_after: self.enemy_ai_budget,
            reason,
            observation,
        });
        if self.enemy_ai_history.len() > 16 {
            self.enemy_ai_history.remove(0);
        }
        self.event_count = self.event_count.saturating_add(1);
    }

    fn resolve_enemy_ai(&mut self) {
        if self.phase == BattlePhase::Approach && self.tick < 300 {
            return;
        }
        let goal = self.enemy_ai_goal;
        let tactics_level = self.enemy_tactics_level;
        let objective = self.seed.map.objective;
        let convoy = self.convoy_position;
        let mut occupied = self
            .party
            .iter()
            .chain(&self.enemies)
            .filter(|unit| unit.alive())
            .map(|unit| unit.position)
            .collect::<BTreeSet<_>>();
        for attacker_index in 0..self.enemies.len() {
            if !self.enemies[attacker_index].alive()
                || self.enemies[attacker_index].role == "worker"
            {
                continue;
            }
            if matches!(goal, AiGoal::RaidEconomy | AiGoal::Assault) {
                if let Some(structure_index) = self
                    .structures
                    .iter()
                    .enumerate()
                    .filter(|(_, structure)| structure.alive())
                    .min_by_key(|(_, structure)| {
                        distance(self.enemies[attacker_index].position, structure.position)
                    })
                    .map(|(index, _)| index)
                {
                    let target = self.structures[structure_index].position;
                    let range = self.enemies[attacker_index].attack_range();
                    if distance(self.enemies[attacker_index].position, target) > range {
                        self.enemies[attacker_index].movement_budget_milli +=
                            self.enemies[attacker_index].move_speed_milli;
                        if self.enemies[attacker_index].movement_budget_milli >= MOVEMENT_TILE_COST
                        {
                            occupied.remove(&self.enemies[attacker_index].position);
                            if let Some(next) = next_step_toward(
                                &self.seed,
                                self.enemies[attacker_index].position,
                                target,
                                range,
                                &occupied,
                            ) {
                                self.enemies[attacker_index].position = next;
                                self.enemies[attacker_index].movement_budget_milli -=
                                    MOVEMENT_TILE_COST;
                            }
                            occupied.insert(self.enemies[attacker_index].position);
                        }
                    } else if self
                        .tick
                        .is_multiple_of(self.enemies[attacker_index].attack_interval_ticks as u64)
                    {
                        let demolition = self.enemies[attacker_index]
                            .skill_ids
                            .iter()
                            .any(|skill| skill == UnitAbility::DemolitionCharge.rule_id())
                            && self.enemies[attacker_index].ability_cooldown_ticks == 0;
                        let bonus = if demolition { 55 } else { 0 };
                        self.structures[structure_index].hp -=
                            (self.enemies[attacker_index].damage + bonus).max(1);
                        if demolition {
                            self.enemies[attacker_index].ability_cooldown_ticks = 45;
                            *self
                                .enemy_ability_activations
                                .entry(UnitAbility::DemolitionCharge.rule_id().to_string())
                                .or_default() += 1;
                        }
                        self.event_count += 1;
                    }
                    continue;
                }
            }
            let Some(target_index) = self
                .party
                .iter()
                .enumerate()
                .filter(|(_, unit)| unit.alive())
                .min_by_key(|(_, unit)| {
                    let wounded_bias = unit.hp * 100 / unit.max_hp.max(1);
                    let role_bias = match goal {
                        AiGoal::RaidEconomy
                            if matches!(unit.role.as_str(), "worker" | "engineer") =>
                        {
                            -50
                        }
                        AiGoal::CounterTech if matches!(unit.role.as_str(), "mystic" | "medic") => {
                            -45
                        }
                        AiGoal::Scout if unit.role == "scout" => -25,
                        _ if tactics_level >= 2 && unit.role == "engineer" => -3,
                        _ => 0,
                    };
                    let position_bias = match goal {
                        AiGoal::DefendObjective => distance(unit.position, objective) as i64 * 4,
                        AiGoal::InterdictConvoy => convoy
                            .map(|position| distance(unit.position, position) as i64)
                            .unwrap_or_default(),
                        _ => 0,
                    };
                    distance(self.enemies[attacker_index].position, unit.position) as i64 * 10
                        + wounded_bias
                        + role_bias
                        + position_bias
                })
                .map(|(index, _)| index)
            else {
                return;
            };
            let range = self.enemies[attacker_index].attack_range();
            let target = self.party[target_index].position;
            if distance(self.enemies[attacker_index].position, target) > range {
                self.enemies[attacker_index].movement_budget_milli +=
                    self.enemies[attacker_index].move_speed_milli;
                if self.enemies[attacker_index].movement_budget_milli >= MOVEMENT_TILE_COST {
                    occupied.remove(&self.enemies[attacker_index].position);
                    if let Some(next) = next_step_toward(
                        &self.seed,
                        self.enemies[attacker_index].position,
                        target,
                        range,
                        &occupied,
                    ) {
                        self.enemies[attacker_index].position = next;
                        self.enemies[attacker_index].movement_budget_milli -= MOVEMENT_TILE_COST;
                    }
                    occupied.insert(self.enemies[attacker_index].position);
                }
            } else if self
                .tick
                .is_multiple_of(self.enemies[attacker_index].attack_interval_ticks as u64)
            {
                let ability = self.enemies[attacker_index]
                    .skill_ids
                    .iter()
                    .find_map(|skill| UnitAbility::from_rule_id(skill));
                let ability_ready =
                    ability.is_some() && self.enemies[attacker_index].ability_cooldown_ticks == 0;
                let hold_bonus = if self.current_order_kind() == RtsOrderKind::Hold {
                    3
                } else {
                    0
                };
                let guard_bonus = if self.party[target_index].guard_ticks > 0 {
                    7
                } else {
                    0
                };
                let role_bonus = match self.enemies[attacker_index].role.as_str() {
                    "assault" => 5,
                    "siege" if self.party[target_index].role == "engineer" => 9,
                    "heavy" => 4,
                    _ => 0,
                };
                let piercing_bonus =
                    if ability_ready && matches!(ability, Some(UnitAbility::PiercingCharge)) {
                        self.party[target_index].armor / 2 + 8
                    } else {
                        0
                    };
                let damage = (self.enemies[attacker_index].damage + role_bonus + piercing_bonus
                    - self.party[target_index].armor
                    - hold_bonus
                    - guard_bonus)
                    .max(1);
                let target_was_alive = self.party[target_index].alive();
                if !deterministic_evade(
                    self.tick,
                    target_index + 31 + simulation_salt(&self.seed) as usize,
                    self.party[target_index].evasion_permille,
                ) {
                    self.party[target_index].hp -= damage;
                    if self.enemies[attacker_index].role == "disruptor" {
                        self.party[target_index].energy =
                            self.party[target_index].energy.saturating_sub(8);
                        self.party[target_index].ability_cooldown_ticks = self.party[target_index]
                            .ability_cooldown_ticks
                            .saturating_add(10);
                    }
                    if self.enemies[attacker_index].role == "frontline" {
                        self.enemies[attacker_index].guard_ticks = 20;
                    }
                }
                if ability_ready {
                    self.activate_enemy_ability(attacker_index, target_index, ability.unwrap());
                }
                if target_was_alive && !self.party[target_index].alive() {
                    self.enemy_score = self.enemy_score.saturating_add(150);
                }
                self.enemies[attacker_index].attacks_made += 1;
                self.event_count += 1;
            }
        }
    }

    fn activate_enemy_ability(
        &mut self,
        attacker_index: usize,
        target_index: usize,
        ability: UnitAbility,
    ) {
        let attacker_position = self.enemies[attacker_index].position;
        self.enemies[attacker_index].ability_cooldown_ticks = 45;
        *self
            .enemy_ability_activations
            .entry(ability.rule_id().to_string())
            .or_default() += 1;
        match ability {
            UnitAbility::RevealPulse => {
                self.intel_level = self.intel_level.saturating_sub(1);
                self.recon_bonus_ticks = self.recon_bonus_ticks.min(5);
            }
            UnitAbility::GuardWall => {
                for enemy in &mut self.enemies {
                    if enemy.alive() && distance(enemy.position, attacker_position) <= 3 {
                        enemy.guard_ticks = enemy.guard_ticks.max(30);
                    }
                }
            }
            UnitAbility::ArcVolley => {
                if let Some((index, _)) = self
                    .party
                    .iter()
                    .enumerate()
                    .filter(|(index, unit)| *index != target_index && unit.alive())
                    .min_by_key(|(_, unit)| distance(unit.position, attacker_position))
                {
                    self.party[index].hp -= (self.enemies[attacker_index].damage / 2).max(1);
                }
            }
            UnitAbility::FieldRepair => {
                if let Some(structure) = self
                    .enemy_structures
                    .iter_mut()
                    .filter(|structure| structure.alive())
                    .min_by_key(|structure| distance(structure.position, attacker_position))
                {
                    structure.hp = (structure.hp + 60).min(structure.max_hp);
                }
            }
            UnitAbility::TriageAura => {
                for enemy in &mut self.enemies {
                    if enemy.alive() && distance(enemy.position, attacker_position) <= 3 {
                        enemy.hp = (enemy.hp + 45).min(enemy.max_hp);
                    }
                }
            }
            UnitAbility::SuppressionBlast => {
                self.party[target_index].energy =
                    self.party[target_index].energy.saturating_sub(18);
                self.party[target_index].ability_cooldown_ticks = self.party[target_index]
                    .ability_cooldown_ticks
                    .saturating_add(20);
            }
            UnitAbility::SmokeDash => {
                self.enemies[attacker_index].evasion_permille =
                    self.enemies[attacker_index].evasion_permille.max(180);
                self.enemies[attacker_index].movement_budget_milli = self.enemies[attacker_index]
                    .movement_budget_milli
                    .saturating_add(MOVEMENT_TILE_COST);
            }
            UnitAbility::RetaliationPlate => {
                self.enemies[attacker_index].guard_ticks = 50;
                self.party[target_index].hp -= 8;
            }
            UnitAbility::PiercingCharge => {
                self.party[target_index].guard_ticks = 0;
            }
            UnitAbility::DemolitionCharge => {
                if let Some(structure) = self
                    .structures
                    .iter_mut()
                    .filter(|structure| structure.alive())
                    .min_by_key(|structure| distance(structure.position, attacker_position))
                {
                    structure.hp -= 55;
                }
            }
            UnitAbility::SignalJam => {
                self.party[target_index].energy =
                    self.party[target_index].energy.saturating_sub(25);
                self.party[target_index].ability_cooldown_ticks = self.party[target_index]
                    .ability_cooldown_ticks
                    .saturating_add(35);
            }
            UnitAbility::CommandSurge => {
                for enemy in &mut self.enemies {
                    if enemy.alive() && distance(enemy.position, attacker_position) <= 4 {
                        enemy.movement_budget_milli = enemy
                            .movement_budget_milli
                            .saturating_add(MOVEMENT_TILE_COST / 2);
                        enemy.guard_ticks = enemy.guard_ticks.max(15);
                    }
                }
            }
        }
        self.event_count += 1;
    }

    fn resolve_relay_pressure(&mut self) {
        if self.current_objective_kind() != Some(ObjectiveKind::Destroy)
            || self.relay_guard_hp <= 0
            || !self.tick.is_multiple_of(24)
        {
            return;
        }
        if let Some(target_index) = self
            .party
            .iter()
            .enumerate()
            .filter(|(_, unit)| unit.alive())
            .min_by_key(|(_, unit)| distance(unit.position, self.seed.map.objective))
            .map(|(index, _)| index)
        {
            let guard_bonus = if self.party[target_index].guard_ticks > 0 {
                8
            } else {
                0
            };
            self.party[target_index].hp -=
                (14 - self.party[target_index].armor - guard_bonus).max(1);
            self.event_count += 1;
        }
    }

    pub fn party_hp_percent(&self) -> u8 {
        percent(
            self.party.iter().map(|unit| unit.hp.max(0)).sum(),
            self.party.iter().map(|unit| unit.max_hp).sum(),
        )
    }

    pub fn is_enemy_visible(&self, enemy_id: &str) -> bool {
        self.enemies
            .iter()
            .find(|enemy| enemy.unit_id == enemy_id && enemy.alive())
            .is_some_and(|enemy| self.visible_tiles.contains(&enemy.position))
    }

    pub fn visible_percent(&self) -> u8 {
        let total = u32::from(self.seed.map.width) * u32::from(self.seed.map.height);
        (self.visible_tiles.len() as u32 * 100)
            .checked_div(total)
            .unwrap_or(0)
            .min(100) as u8
    }

    pub fn visible_enemy_count(&self) -> usize {
        self.enemies
            .iter()
            .filter(|enemy| enemy.alive() && self.visible_tiles.contains(&enemy.position))
            .count()
    }

    pub fn visible_enemy_hp_percent(&self) -> u8 {
        let visible = self
            .enemies
            .iter()
            .filter(|enemy| enemy.alive() && self.visible_tiles.contains(&enemy.position))
            .collect::<Vec<_>>();
        percent(
            visible.iter().map(|enemy| enemy.hp.max(0)).sum(),
            visible.iter().map(|enemy| enemy.max_hp).sum(),
        )
    }

    fn refresh_visibility(&mut self) {
        let mut visible = BTreeSet::new();
        let base_radius = 4 + i16::from(self.intel_level.min(2));
        for unit in self.party.iter().filter(|unit| unit.alive()) {
            reveal_from(&self.seed, unit.position, base_radius, &mut visible);
        }
        for support in &self.support_units {
            reveal_from(&self.seed, support.position, 3, &mut visible);
        }
        if self.recon_bonus_ticks > 0 {
            if let Some(focus) = self.recon_focus {
                let radius = if self.researched_techs.contains("signal_optics") {
                    8
                } else {
                    6
                };
                reveal_from(&self.seed, focus, radius, &mut visible);
            }
        }
        self.visible_tiles = visible;
        self.explored_tiles
            .extend(self.visible_tiles.iter().copied());
    }

    pub fn enemy_hp_percent(&self) -> u8 {
        percent(
            self.enemies.iter().map(|unit| unit.hp.max(0)).sum(),
            self.enemies.iter().map(|unit| unit.max_hp).sum(),
        )
    }

    pub fn relay_guard_percent(&self) -> u8 {
        percent(self.relay_guard_hp.max(0), self.relay_guard_max_hp)
    }

    pub fn capture_percent(&self) -> u8 {
        (self.relay_capture_ticks as u64 * 100 / CAPTURE_TICKS_REQUIRED as u64).min(100) as u8
    }

    pub fn snapshot_hash(&self) -> Result<String, SimError> {
        json_hash(self)
    }

    pub fn export_replay(&self) -> Result<BattleReplayV1, SimError> {
        self.validate()?;
        Ok(BattleReplayV1 {
            contract_version: "trnm_battle_replay_v1".to_string(),
            seed: self.seed.clone(),
            entries: self.replay_orders.clone(),
            final_tick: self.tick,
            final_snapshot_hash: self.snapshot_hash()?,
        })
    }

    pub fn into_result(self) -> Result<BattleResultV1, SimError> {
        self.validate()?;
        let outcome = self.outcome.ok_or_else(|| {
            SimError::InvalidState("cannot emit a BattleResult before terminal state".to_string())
        })?;
        let final_snapshot_hash = self.snapshot_hash()?;
        let siege = self.seed.map_id == "mirror_siege";
        let skirmish = self.seed.skirmish.enabled;
        let experience = match outcome {
            BattleOutcome::Victory if siege => 70,
            BattleOutcome::Victory if self.seed.map_id == "convoy_exodus" => 60,
            BattleOutcome::Victory if is_aftershock_map(&self.seed.map_id) => 55,
            BattleOutcome::Victory if skirmish => 50,
            BattleOutcome::Victory => 40,
            BattleOutcome::Defeat if self.tick >= 60 * TICKS_PER_SECOND => 3,
            BattleOutcome::Defeat | BattleOutcome::Withdrawal => 0,
        };
        let units = self
            .party
            .iter()
            .map(|unit| {
                let status = if unit.hp <= 0 {
                    if unit.persistent {
                        UnitBattleStatus::Incapacitated
                    } else {
                        UnitBattleStatus::Lost
                    }
                } else if unit.hp * 100 < unit.max_hp * 60 {
                    UnitBattleStatus::Wounded
                } else {
                    UnitBattleStatus::Healthy
                };
                UnitBattleReportV1 {
                    unit_id: unit.unit_id.clone(),
                    status,
                    remaining_hp: unit.hp.max(0) as u32,
                    experience_gained: experience,
                    veteran_rank: unit.veteran_rank,
                    confirmed_kills: unit.confirmed_kills,
                }
            })
            .collect();
        let aftershock = is_aftershock_map(&self.seed.map_id);
        let convoy = self.seed.map_id == "convoy_exodus";
        let (loot, reputation_delta, world_flags) = match outcome {
            BattleOutcome::Victory if self.seed.map_id == "iron_delta" => (
                vec![LootStack {
                    item_id: "salvaged-alloy".to_string(),
                    quantity: 3,
                }],
                4,
                vec!["iron_delta_won".to_string()],
            ),
            BattleOutcome::Victory if self.seed.map_id == "night_watch_crossing" => (
                vec![LootStack {
                    item_id: "watch-cloth".to_string(),
                    quantity: 3,
                }],
                4,
                vec!["night_watch_crossing_won".to_string()],
            ),
            BattleOutcome::Victory if self.seed.map_id == "glass_basin" => (
                vec![LootStack {
                    item_id: "route-token".to_string(),
                    quantity: 3,
                }],
                5,
                vec!["glass_basin_won".to_string()],
            ),
            BattleOutcome::Victory if self.seed.map_id == "ember_orchard" => (
                vec![LootStack {
                    item_id: "ash-runner-seal".to_string(),
                    quantity: 2,
                }],
                5,
                vec!["ember_orchard_won".to_string()],
            ),
            BattleOutcome::Victory if siege => (
                vec![LootStack {
                    item_id: "mirror-gate-insignia".to_string(),
                    quantity: 1,
                }],
                8,
                vec!["mirror_siege_secured".to_string()],
            ),
            BattleOutcome::Victory if convoy => (
                vec![LootStack {
                    item_id: "signal-convoy-seal".to_string(),
                    quantity: 1,
                }],
                6,
                vec!["convoy_exodus_secured".to_string()],
            ),
            BattleOutcome::Victory if aftershock => (
                vec![LootStack {
                    item_id: "field-tonic-kit".to_string(),
                    quantity: 1,
                }],
                3,
                vec!["aftershock_patrol_secured".to_string()],
            ),
            BattleOutcome::Victory => (
                vec![
                    LootStack {
                        item_id: "relay-core-fragment".to_string(),
                        quantity: 1,
                    },
                    LootStack {
                        item_id: "field-tonic-kit".to_string(),
                        quantity: 1,
                    },
                ],
                5,
                vec!["first_contact_secured".to_string()],
            ),
            BattleOutcome::Defeat => (
                Vec::new(),
                -2,
                vec![if siege {
                    "mirror_siege_lost".to_string()
                } else if convoy {
                    "convoy_exodus_lost".to_string()
                } else if aftershock {
                    "aftershock_patrol_repulsed".to_string()
                } else {
                    "first_contact_repulsed".to_string()
                }],
            ),
            BattleOutcome::Withdrawal => (
                Vec::new(),
                0,
                vec![if siege {
                    "mirror_siege_withdrawn".to_string()
                } else if convoy {
                    "convoy_exodus_withdrawn".to_string()
                } else if aftershock {
                    "aftershock_patrol_withdrawn".to_string()
                } else {
                    "first_contact_withdrawn".to_string()
                }],
            ),
        };
        Ok(BattleResultV1 {
            contract_version: BATTLE_RESULT_CONTRACT.to_string(),
            battle_id: self.seed.battle_id.clone(),
            seed_hash: self.seed.seed_hash.clone(),
            outcome,
            units,
            loot,
            resource_delta: if outcome == BattleOutcome::Victory {
                self.resources_available as i64
            } else {
                0
            },
            reputation_delta,
            world_flags,
            elapsed_ticks: self.tick,
            final_snapshot_hash,
        })
    }
}

fn is_aftershock_map(map_id: &str) -> bool {
    matches!(map_id, "aftershock_patrol" | "first_contact_aftershock")
}

fn default_support_role() -> String {
    "support".to_string()
}

fn is_continuous_order(kind: RtsOrderKind) -> bool {
    matches!(
        kind,
        RtsOrderKind::Move
            | RtsOrderKind::AttackMove
            | RtsOrderKind::Patrol
            | RtsOrderKind::Harvest
            | RtsOrderKind::Capture
            | RtsOrderKind::Attack
            | RtsOrderKind::FocusFire
            | RtsOrderKind::Hold
    )
}

fn reveal_from(
    seed: &BattleSeedV1,
    origin: BattleGridPoint,
    radius: i16,
    visible: &mut BTreeSet<BattleGridPoint>,
) {
    let mut frontier = VecDeque::from([(origin, 0_i16)]);
    let mut visited = BTreeSet::from([origin]);
    while let Some((tile, steps)) = frontier.pop_front() {
        visible.insert(tile);
        if steps >= radius {
            continue;
        }
        for next in neighbors(tile) {
            if !seed.map.in_bounds(next) || !visited.insert(next) {
                continue;
            }
            if seed.map.passable(next) {
                frontier.push_back((next, steps + 1));
            } else {
                visible.insert(next);
            }
        }
    }
}

fn formation_target_for(
    center: BattleGridPoint,
    index: usize,
    formation_id: &str,
    seed: &BattleSeedV1,
) -> BattleGridPoint {
    let offsets: &[(i16, i16)] = match formation_id {
        "party_line" => &[(-1, 0), (0, 0), (1, 0), (2, 0)],
        "party_column" => &[(0, -1), (0, 0), (0, 1), (0, 2)],
        "party_wedge" => &[(0, 0), (-1, 1), (1, 1), (0, 2)],
        _ => &[(0, 0)],
    };
    let (x, y) = offsets[index % offsets.len()];
    let candidate = BattleGridPoint::new(center.x + x, center.y + y);
    if seed.map.passable(candidate) {
        candidate
    } else {
        center
    }
}

fn nearest_passable(seed: &BattleSeedV1, target: BattleGridPoint) -> Option<BattleGridPoint> {
    if seed.map.passable(target) {
        return Some(target);
    }
    neighbors(target)
        .into_iter()
        .find(|candidate| seed.map.passable(*candidate))
}

fn signature_skill(unit: &SimUnit) -> &'static str {
    for skill in [
        "field_mend",
        "relay_overcharge",
        "inner_flame",
        "wind_step",
        "iron_guard",
    ] {
        if unit.skill_ids.iter().any(|candidate| candidate == skill) {
            return skill;
        }
    }
    "iron_guard"
}

fn formation_positions(start: BattleGridPoint, seed: &BattleSeedV1) -> Vec<BattleGridPoint> {
    let candidates = [
        start,
        BattleGridPoint::new(start.x + 1, start.y),
        BattleGridPoint::new(start.x, start.y - 1),
        BattleGridPoint::new(start.x + 1, start.y - 1),
    ];
    candidates
        .into_iter()
        .map(|candidate| {
            if seed.map.passable(candidate) {
                candidate
            } else {
                start
            }
        })
        .collect()
}

impl BattleReplayV1 {
    pub fn replay_and_verify(&self) -> Result<MissionSimV1, SimError> {
        if self.contract_version != "trnm_battle_replay_v1" {
            return Err(SimError::InvalidState(
                "unsupported battle replay contract".to_string(),
            ));
        }
        let mut sim = MissionSimV1::from_seed(self.seed.clone())?;
        for entry in &self.entries {
            while sim.tick < entry.issued_tick {
                sim.step()?;
            }
            sim.issue_order(entry.order.clone())?;
        }
        while sim.tick < self.final_tick {
            sim.step()?;
        }
        if sim.snapshot_hash()? != self.final_snapshot_hash {
            return Err(SimError::Integrity(
                "battle replay diverged from recorded snapshot".to_string(),
            ));
        }
        Ok(sim)
    }

    pub fn save_atomic(&self, path: &Path) -> Result<(), SimError> {
        self.replay_and_verify()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let temp_path = path.with_extension("json.tmp");
        let mut file = fs::File::create(&temp_path)?;
        file.write_all(&serde_json::to_vec_pretty(self)?)?;
        file.sync_all()?;
        fs::rename(&temp_path, path)?;
        if let Some(parent) = path.parent() {
            fs::File::open(parent)?.sync_all()?;
        }
        Ok(())
    }

    pub fn load_verified(path: &Path) -> Result<Self, SimError> {
        let replay: Self = serde_json::from_slice(&fs::read(path)?)?;
        replay.replay_and_verify()?;
        Ok(replay)
    }
}

fn percent(current: i64, maximum: i64) -> u8 {
    if maximum <= 0 {
        0
    } else {
        (current * 100 / maximum).clamp(0, 100) as u8
    }
}

fn distance(left: BattleGridPoint, right: BattleGridPoint) -> i16 {
    (left.x - right.x).abs() + (left.y - right.y).abs()
}

fn neighbors(point: BattleGridPoint) -> [BattleGridPoint; 4] {
    [
        BattleGridPoint::new(point.x + 1, point.y),
        BattleGridPoint::new(point.x - 1, point.y),
        BattleGridPoint::new(point.x, point.y + 1),
        BattleGridPoint::new(point.x, point.y - 1),
    ]
}

fn next_step_toward(
    seed: &BattleSeedV1,
    start: BattleGridPoint,
    target: BattleGridPoint,
    stop_range: i16,
    occupied: &BTreeSet<BattleGridPoint>,
) -> Option<BattleGridPoint> {
    if distance(start, target) <= stop_range {
        return None;
    }
    let mut queue = VecDeque::from([start]);
    let mut previous = BTreeMap::<BattleGridPoint, BattleGridPoint>::new();
    let mut visited = BTreeSet::from([start]);
    let mut reached = None;
    while let Some(current) = queue.pop_front() {
        if distance(current, target) <= stop_range {
            reached = Some(current);
            break;
        }
        for next in neighbors(current) {
            if !seed.map.passable(next)
                || (occupied.contains(&next) && next != target)
                || !visited.insert(next)
            {
                continue;
            }
            previous.insert(next, current);
            queue.push_back(next);
        }
    }
    let mut current = reached?;
    while let Some(parent) = previous.get(&current).copied() {
        if parent == start {
            return Some(current);
        }
        current = parent;
    }
    None
}

fn deterministic_yield_step(
    seed: &BattleSeedV1,
    start: BattleGridPoint,
    target: BattleGridPoint,
    occupied: &BTreeSet<BattleGridPoint>,
    reservations: &[TileReservation],
) -> Option<BattleGridPoint> {
    let mut candidates = neighbors(start)
        .into_iter()
        .filter(|candidate| {
            seed.map.passable(*candidate)
                && !occupied.contains(candidate)
                && !reservations
                    .iter()
                    .any(|reservation| reservation.tile == *candidate)
        })
        .collect::<Vec<_>>();
    candidates.sort_by_key(|candidate| (distance(*candidate, target), candidate.y, candidate.x));
    candidates.into_iter().next()
}

fn deterministic_evade(tick: u64, unit_index: usize, evasion_permille: u16) -> bool {
    ((tick.wrapping_mul(37) + unit_index as u64 * 101) % 1000) < evasion_permille as u64
}

fn simulation_salt(seed: &BattleSeedV1) -> u64 {
    if seed.skirmish.enabled {
        return seed.skirmish.simulation_seed % 997;
    }
    // Authored campaign battles retain their established deterministic
    // cadence. Only an explicitly configured skirmish seed is allowed to
    // perturb combat sampling.
    0
}

fn hash_json<T: Serialize>(value: &T) -> Result<String, SimError> {
    json_hash(value)
}

fn json_hash<T: Serialize>(value: &T) -> Result<String, SimError> {
    let bytes = serde_json::to_vec(value)?;
    let digest = Sha256::digest(bytes);
    Ok(digest.iter().map(|byte| format!("{byte:02x}")).collect())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimCheckpointV1 {
    pub contract_version: String,
    pub sim: MissionSimV1,
    pub checkpoint_hash: String,
}

impl SimCheckpointV1 {
    pub fn capture(sim: &MissionSimV1) -> Result<Self, SimError> {
        sim.validate()?;
        let mut checkpoint = Self {
            contract_version: RTS_SIM_CHECKPOINT_CONTRACT.to_string(),
            sim: sim.clone(),
            checkpoint_hash: String::new(),
        };
        checkpoint.checkpoint_hash = checkpoint.computed_hash()?;
        Ok(checkpoint)
    }

    pub fn validate(&self) -> Result<(), SimError> {
        if self.contract_version != RTS_SIM_CHECKPOINT_CONTRACT {
            return Err(SimError::InvalidState(format!(
                "unsupported checkpoint contract {}",
                self.contract_version
            )));
        }
        self.sim.validate()?;
        if self.checkpoint_hash != self.computed_hash()? {
            return Err(SimError::Integrity(
                "simulation checkpoint hash mismatch".to_string(),
            ));
        }
        Ok(())
    }

    fn computed_hash(&self) -> Result<String, SimError> {
        let mut canonical = self.clone();
        canonical.checkpoint_hash.clear();
        json_hash(&canonical)
    }
}

#[derive(Debug, Clone)]
pub struct SimCheckpointStore {
    path: PathBuf,
}

impl SimCheckpointStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn save_atomic(&self, sim: &MissionSimV1) -> Result<(), SimError> {
        let checkpoint = SimCheckpointV1::capture(sim)?;
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let temp_path = self.path.with_extension("json.tmp");
        let mut file = fs::File::create(&temp_path)?;
        file.write_all(&serde_json::to_vec_pretty(&checkpoint)?)?;
        file.sync_all()?;
        fs::rename(&temp_path, &self.path)?;
        if let Some(parent) = self.path.parent() {
            fs::File::open(parent)?.sync_all()?;
        }
        Ok(())
    }

    pub fn load(&self) -> Result<SimCheckpointV1, SimError> {
        let checkpoint: SimCheckpointV1 = serde_json::from_slice(&fs::read(&self.path)?)?;
        checkpoint.validate()?;
        Ok(checkpoint)
    }

    pub fn load_for_seed(&self, seed: &BattleSeedV1) -> Result<Option<MissionSimV1>, SimError> {
        match self.load() {
            Ok(checkpoint)
                if checkpoint.sim.seed.battle_id == seed.battle_id
                    && checkpoint.sim.seed.seed_hash == seed.seed_hash =>
            {
                Ok(Some(checkpoint.sim))
            }
            Ok(_) => Ok(None),
            Err(SimError::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use trnm_campaign_core::{
        BattleMapNodeV1, BattleMapSeedV1, CampaignMission, CampaignRoom, CampaignSaveV1,
        MissionDefinition, QuestState,
    };
    use trnm_rts_protocol::{RtsOrderSource, RtsTile};

    fn map() -> BattleMapSeedV1 {
        BattleMapSeedV1 {
            width: 20,
            height: 10,
            terrain_rows: vec!["gggggggggggggggggggg".to_string(); 10],
            party_start: BattleGridPoint::new(1, 8),
            approach_point: BattleGridPoint::new(7, 6),
            objective: BattleGridPoint::new(18, 1),
            resource_nodes: vec![BattleMapNodeV1 {
                id: "amber_mid".to_string(),
                position: BattleGridPoint::new(8, 7),
            }],
            enemy_spawns: vec![
                BattleMapNodeV1 {
                    id: "contact_scout".to_string(),
                    position: BattleGridPoint::new(10, 5),
                },
                BattleMapNodeV1 {
                    id: "contact_warden".to_string(),
                    position: BattleGridPoint::new(12, 4),
                },
                BattleMapNodeV1 {
                    id: "contact_striker".to_string(),
                    position: BattleGridPoint::new(14, 3),
                },
                BattleMapNodeV1 {
                    id: "relay_guard".to_string(),
                    position: BattleGridPoint::new(16, 2),
                },
            ],
        }
    }

    fn seed() -> BattleSeedV1 {
        let mut campaign = CampaignSaveV1::default();
        campaign.move_to(CampaignRoom::MentorHall).unwrap();
        campaign.talk_to_mentor().unwrap();
        campaign.train_with_mentor().unwrap();
        campaign.equip_starter_weapon().unwrap();
        campaign.move_to(CampaignRoom::ExpeditionGate).unwrap();
        campaign.accept_first_contact_quest().unwrap();
        campaign.start_first_contact_battle(map()).unwrap()
    }

    fn iron_delta_seed() -> BattleSeedV1 {
        let mut campaign = CampaignSaveV1::default();
        campaign.move_to(CampaignRoom::MentorHall).unwrap();
        campaign.talk_to_mentor().unwrap();
        campaign.train_with_mentor().unwrap();
        campaign.equip_starter_weapon().unwrap();
        campaign.move_to(CampaignRoom::ExpeditionGate).unwrap();
        campaign.quest_state = QuestState::Completed;
        campaign.progression.aftershock_completions = 1;
        campaign.active_mission = CampaignMission::AftershockPatrol;
        for flag in [
            "first_contact_secured",
            "convoy_exodus_secured",
            "mirror_siege_secured",
        ] {
            campaign.progression.world_flags.insert(flag.to_string());
        }
        assert_eq!(
            campaign.cycle_endgame_mission().unwrap(),
            CampaignMission::IronDeltaSkirmish
        );
        campaign.accept_first_contact_quest().unwrap();
        let mut authored_map = map();
        authored_map.enemy_spawns = [
            "ash_runner",
            "ash_bulwark",
            "ash_lancer",
            "ash_sapper",
            "ash_commander",
        ]
        .into_iter()
        .enumerate()
        .map(|(index, id)| BattleMapNodeV1 {
            id: id.to_string(),
            position: BattleGridPoint::new(10 + index as i16, 5),
        })
        .collect();
        campaign.start_first_contact_battle(authored_map).unwrap()
    }

    #[test]
    fn iron_delta_instantiates_the_authored_ashen_compact_roster() {
        let sim = MissionSimV1::from_seed(iron_delta_seed()).unwrap();
        let runner = sim
            .enemies
            .iter()
            .find(|enemy| enemy.unit_id == "ash_runner")
            .unwrap();
        let commander = sim
            .enemies
            .iter()
            .find(|enemy| enemy.unit_id == "ash_commander")
            .unwrap();
        assert_eq!(runner.role, "raider");
        assert_eq!(commander.role, "heavy");
        assert!(commander.max_hp > runner.max_hp);
        assert!(commander.damage > runner.damage);
    }

    #[test]
    fn configurable_skirmish_drives_faction_roster_structures_tech_and_terminal_rules() {
        let mut seed = iron_delta_seed();
        seed.skirmish.player_faction = RtsFaction::AshenCompact;
        seed.skirmish.enemy_faction = RtsFaction::MirrorCoalition;
        seed.skirmish.starting_resources = 1_000;
        seed.skirmish.victory_mode = SkirmishVictoryMode::Score;
        seed.skirmish.score_target = 500;
        seed.seed_hash = seed.computed_hash().unwrap();
        seed.validate().unwrap();

        let mut sim = MissionSimV1::from_seed(seed.clone()).unwrap();
        assert!(
            sim.resources_available >= 1_000,
            "skirmish resources {} readiness {} configured {}",
            sim.resources_available,
            sim.seed.expedition_readiness.starting_resources,
            sim.seed.skirmish.starting_resources
        );
        assert!(sim
            .enemies
            .iter()
            .filter(|enemy| enemy.role != "worker")
            .all(|enemy| {
                UNIT_ROSTER.iter().any(|unit| {
                    unit.faction == RtsFaction::MirrorCoalition && unit.role == enemy.role
                })
            }));
        let start = sim.seed.map.party_start;
        let mut workshop = order(
            &sim,
            RtsOrderKind::Build,
            BattleGridPoint::new(start.x + 4, start.y),
        );
        workshop.target_rule_id = Some("field_workshop".to_string());
        sim.issue_order(workshop).unwrap();
        let mut beacon = order(
            &sim,
            RtsOrderKind::Build,
            BattleGridPoint::new(start.x + 5, start.y),
        );
        beacon.target_rule_id = Some("ash_beacon".to_string());
        sim.issue_order(beacon).unwrap();
        let mut opposing_tower = order(
            &sim,
            RtsOrderKind::Build,
            BattleGridPoint::new(start.x + 6, start.y),
        );
        opposing_tower.target_rule_id = Some("sensor_tower".to_string());
        assert!(sim.issue_order(opposing_tower).is_err());

        sim.issue_order(job_order(
            &sim,
            RtsOrderKind::Research,
            "rapid_mustering",
            start,
        ))
        .unwrap();
        for _ in 0..80 {
            sim.process_jobs();
        }
        assert!(sim.researched_techs.contains("rapid_mustering"));
        sim.issue_order(job_order(&sim, RtsOrderKind::Train, "ash_runner", start))
            .unwrap();
        for _ in 0..60 {
            sim.process_jobs();
        }
        assert!(sim.support_units.iter().any(|unit| unit.role == "raider"));
        assert!(sim
            .issue_order(job_order(
                &sim,
                RtsOrderKind::Train,
                "mirror_wayfinder",
                start,
            ))
            .is_err());

        let mut score_sim = MissionSimV1::from_seed(seed.clone()).unwrap();
        score_sim.player_score = 500;
        score_sim.step().unwrap();
        assert_eq!(score_sim.outcome, Some(BattleOutcome::Victory));

        seed.skirmish.victory_mode = SkirmishVictoryMode::Annihilation;
        seed.seed_hash = seed.computed_hash().unwrap();
        let mut annihilation = MissionSimV1::from_seed(seed).unwrap();
        for enemy in &mut annihilation.enemies {
            enemy.hp = 0;
        }
        for structure in &mut annihilation.enemy_structures {
            structure.hp = 0;
        }
        annihilation.step().unwrap();
        assert_eq!(annihilation.outcome, Some(BattleOutcome::Victory));
    }

    #[test]
    fn skirmish_enemy_runs_a_deterministic_resource_build_research_and_production_loop() {
        let seed = iron_delta_seed();
        let mut first = MissionSimV1::from_seed(seed.clone()).unwrap();
        let mut second = MissionSimV1::from_seed(seed).unwrap();
        for sim in [&mut first, &mut second] {
            assert_eq!(
                sim.enemies
                    .iter()
                    .filter(|unit| unit.role == "worker" && unit.alive())
                    .count(),
                usize::from(sim.enemy_workers)
            );
            assert!(sim.enemy_supply_used() <= sim.enemy_supply_cap());
            for unit in &mut sim.party {
                unit.max_hp = 100_000;
                unit.hp = 100_000;
            }
            for _ in 0..500 {
                if !sim.terminal() {
                    sim.step().unwrap();
                }
            }
            assert!(sim.enemy_resources_generated > sim.seed.skirmish.starting_resources);
            assert!(sim.enemy_resources_spent > 0);
            assert_eq!(
                sim.enemy_resources_available + sim.enemy_resources_spent,
                sim.enemy_resources_generated
            );
            assert!(sim.enemy_structures.len() >= 3);
            assert!(
                !sim.enemy_researched_techs.is_empty()
                    || sim.enemies.len() > sim.seed.map.enemy_spawns.len()
            );
        }
        assert_eq!(
            first.enemy_resources_generated,
            second.enemy_resources_generated
        );
        assert_eq!(first.enemy_resources_spent, second.enemy_resources_spent);
        assert_eq!(first.enemy_structures, second.enemy_structures);
        assert_eq!(first.enemy_researched_techs, second.enemy_researched_techs);
        assert_eq!(first.enemies, second.enemies);
    }

    fn order(sim: &MissionSimV1, kind: RtsOrderKind, target: BattleGridPoint) -> RtsFrameOrder {
        let mut order = RtsFrameOrder::new(
            sim.tick as u32,
            "player",
            sim.party
                .iter()
                .filter(|unit| unit.alive())
                .map(|unit| unit.unit_id.clone())
                .collect::<Vec<_>>(),
            kind,
            RtsOrderSource::LocalInput,
        );
        order.target_tile = Some(RtsTile::new(target.x as i32, target.y as i32));
        if kind == RtsOrderKind::Harvest {
            order.target_actor_id = Some("amber_mid".to_string());
        } else if kind == RtsOrderKind::Attack {
            order.target_actor_id = Some("relay_beacon".to_string());
        } else if kind == RtsOrderKind::Ability {
            order.target_rule_id = Some("party_signature".to_string());
        } else if kind == RtsOrderKind::Repair {
            order.target_actor_id = Some("party_field_aid".to_string());
        } else if kind == RtsOrderKind::Build {
            order.target_rule_id = Some("field_barricade".to_string());
        }
        order
    }

    fn job_order(
        sim: &MissionSimV1,
        kind: RtsOrderKind,
        rule: &str,
        target: BattleGridPoint,
    ) -> RtsFrameOrder {
        let mut order = order(sim, kind, target);
        order.target_rule_id = Some(rule.to_string());
        order.queue_id = Some("test_queue".to_string());
        order
    }

    fn step_until(sim: &mut MissionSimV1, predicate: impl Fn(&MissionSimV1) -> bool, limit: u64) {
        while !sim.terminal() && !predicate(sim) && sim.tick < limit {
            sim.step().unwrap();
        }
        assert!(
            predicate(sim),
            "condition not reached by tick {} phase {:?} outcome {:?} guard {} capture {} wave {} alive_enemies {} order {:?}",
            sim.tick,
            sim.phase,
            sim.outcome,
            sim.relay_guard_hp,
            sim.relay_capture_ticks,
            sim.reinforcement_wave,
            sim.enemies.iter().filter(|enemy| enemy.alive()).count(),
            sim.current_order_kind(),
        );
    }

    fn run_decision_dense_victory(mut sim: MissionSimV1, harvest: bool) -> MissionSimV1 {
        let approach = sim.seed.map.approach_point;
        sim.issue_order(order(&sim, RtsOrderKind::Move, approach))
            .unwrap();
        step_until(&mut sim, |sim| sim.phase == BattlePhase::Contact, 900);
        if harvest {
            let resource = sim.seed.map.resource_nodes[0].position;
            sim.issue_order(order(&sim, RtsOrderKind::Harvest, resource))
                .unwrap();
            step_until(&mut sim, |sim| sim.resources_available >= 100, 1_300);
        } else {
            let objective = sim.seed.map.objective;
            sim.issue_order(order(&sim, RtsOrderKind::Ability, objective))
                .unwrap();
        }
        let objective = sim.seed.map.objective;
        sim.issue_order(order(&sim, RtsOrderKind::Attack, objective))
            .unwrap();
        step_until(
            &mut sim,
            |sim| sim.phase == BattlePhase::Relay && sim.relay_guard_hp <= 0,
            2_700,
        );
        sim.issue_order(order(&sim, RtsOrderKind::Hold, objective))
            .unwrap();
        for wave in 1..=2 {
            step_until(
                &mut sim,
                |sim| sim.reinforcement_wave >= wave,
                FIVE_MINUTE_TICKS,
            );
            if harvest {
                let resource_order = if wave == 1 {
                    RtsOrderKind::Repair
                } else {
                    RtsOrderKind::Build
                };
                sim.issue_order(order(&sim, resource_order, objective))
                    .unwrap();
            } else {
                sim.issue_order(order(&sim, RtsOrderKind::Ability, objective))
                    .unwrap();
            }
            sim.issue_order(order(&sim, RtsOrderKind::Attack, objective))
                .unwrap();
            step_until(
                &mut sim,
                |sim| sim.enemies.iter().all(|enemy| !enemy.alive()),
                FIVE_MINUTE_TICKS,
            );
            sim.issue_order(order(&sim, RtsOrderKind::Move, objective))
                .unwrap();
            step_until(
                &mut sim,
                |sim| {
                    sim.party.iter().any(|unit| {
                        unit.alive() && distance(unit.position, sim.seed.map.objective) <= 2
                    })
                },
                FIVE_MINUTE_TICKS,
            );
            sim.issue_order(order(&sim, RtsOrderKind::Hold, objective))
                .unwrap();
        }
        while !sim.terminal() {
            sim.step().unwrap();
        }
        sim
    }

    #[test]
    fn three_phase_orders_produce_a_real_three_to_five_minute_victory() {
        let sim = run_decision_dense_victory(MissionSimV1::from_seed(seed()).unwrap(), true);
        assert_eq!(sim.outcome, Some(BattleOutcome::Victory));
        assert!(
            (THREE_MINUTE_TICKS..=FIVE_MINUTE_TICKS).contains(&sim.tick),
            "victory tick {} is outside the 3-5 minute target",
            sim.tick
        );
        assert!((8..=12).contains(&sim.order_count));
        let barricade_cost = SimStructureKind::FieldBarricade.definition().cost;
        assert!(sim.resources_spent >= FIELD_AID_COST + barricade_cost);
        assert_eq!(sim.enemy_tactics_level, 3);
        assert!(!sim.enemy_ai_history.is_empty());
        let result = sim.into_result().unwrap();
        assert!(!result.loot.is_empty());
        assert!(result.resource_delta > 0);
    }

    #[test]
    fn one_order_cannot_win_the_mission() {
        for kind in [
            RtsOrderKind::Move,
            RtsOrderKind::Attack,
            RtsOrderKind::Harvest,
            RtsOrderKind::Hold,
        ] {
            let mut sim = MissionSimV1::from_seed(seed()).unwrap();
            let target = match kind {
                RtsOrderKind::Harvest => sim.seed.map.resource_nodes[0].position,
                RtsOrderKind::Move => sim.seed.map.approach_point,
                _ => sim.seed.map.objective,
            };
            sim.issue_order(order(&sim, kind, target)).unwrap();
            while !sim.terminal() {
                sim.step().unwrap();
            }
            assert_ne!(
                sim.outcome,
                Some(BattleOutcome::Victory),
                "{kind:?} won alone"
            );
        }
    }

    #[test]
    fn ability_rush_is_a_second_viable_route_without_resource_payout() {
        let sim = run_decision_dense_victory(MissionSimV1::from_seed(seed()).unwrap(), false);
        assert_eq!(sim.outcome, Some(BattleOutcome::Victory));
        assert_eq!(sim.resources_gathered, 0);
        assert!((8..=12).contains(&sim.order_count));
        assert!(sim.distinct_order_kinds.contains("ability"));
        assert!((THREE_MINUTE_TICKS..=FIVE_MINUTE_TICKS).contains(&sim.tick));
    }

    #[test]
    fn withdrawal_has_zero_progression_reward() {
        let mut sim = MissionSimV1::from_seed(seed()).unwrap();
        sim.issue_order(order(&sim, RtsOrderKind::Move, sim.seed.map.approach_point))
            .unwrap();
        for _ in 0..WITHDRAWAL_MIN_TICKS {
            sim.step().unwrap();
        }
        let mut retreat = order(&sim, RtsOrderKind::Extract, sim.seed.map.party_start);
        retreat.target_actor_id = Some("expedition_gate".to_string());
        sim.issue_order(retreat).unwrap();
        let result = sim.into_result().unwrap();
        assert_eq!(result.outcome, BattleOutcome::Withdrawal);
        assert_eq!(
            result
                .units
                .iter()
                .map(|unit| unit.experience_gained)
                .sum::<u64>(),
            0
        );
        assert_eq!(result.resource_delta, 0);
    }

    #[test]
    fn checkpoint_resume_is_bit_deterministic() {
        let directory = tempdir().unwrap();
        let store = SimCheckpointStore::new(directory.path().join("battle.json"));
        let mut baseline = MissionSimV1::from_seed(seed()).unwrap();
        baseline
            .issue_order(order(
                &baseline,
                RtsOrderKind::Move,
                baseline.seed.map.approach_point,
            ))
            .unwrap();
        for _ in 0..200 {
            baseline.step().unwrap();
        }
        store.save_atomic(&baseline).unwrap();
        let resumed = store.load_for_seed(&baseline.seed).unwrap().unwrap();
        assert_eq!(resumed, baseline);
        assert_eq!(
            resumed.snapshot_hash().unwrap(),
            baseline.snapshot_hash().unwrap()
        );
    }

    #[test]
    fn tampered_checkpoint_is_rejected() {
        let mut checkpoint =
            SimCheckpointV1::capture(&MissionSimV1::from_seed(seed()).unwrap()).unwrap();
        checkpoint.sim.resources_gathered += 1;
        assert!(matches!(checkpoint.validate(), Err(SimError::Integrity(_))));
    }

    #[test]
    fn recon_production_research_upgrade_and_formations_change_authoritative_state() {
        let seed = seed();
        let mut sim = MissionSimV1::from_seed(seed.clone()).unwrap();
        sim.resources_gathered = 200;
        sim.resources_available = 200;
        let target = seed.map.approach_point;

        sim.issue_order(order(&sim, RtsOrderKind::Recon, target))
            .unwrap();
        assert_eq!(sim.intel_level, 1);
        sim.issue_order(job_order(
            &sim,
            RtsOrderKind::Train,
            "field_support_drone",
            target,
        ))
        .unwrap();
        sim.issue_order(job_order(
            &sim,
            RtsOrderKind::Research,
            "field_logistics",
            target,
        ))
        .unwrap();
        for _ in 0..150 {
            sim.step().unwrap();
        }
        assert_eq!(sim.support_units.len(), 1);
        assert!(sim.researched_techs.contains("field_logistics"));
        let damage_before = sim.party[0].damage;
        sim.issue_order(job_order(&sim, RtsOrderKind::Upgrade, "relay_arms", target))
            .unwrap();
        for _ in 0..60 {
            sim.step().unwrap();
        }
        assert_eq!(sim.upgrade_level, 1);
        assert!(sim.party[0].damage > damage_before);
        assert_eq!(sim.resources_available + sim.resources_spent, 200);

        let mut line = MissionSimV1::from_seed(seed.clone()).unwrap();
        let mut line_order = order(&line, RtsOrderKind::Move, target);
        line_order.formation_id = Some("party_line".to_string());
        line.issue_order(line_order).unwrap();
        let mut column = MissionSimV1::from_seed(seed).unwrap();
        let mut column_order = order(&column, RtsOrderKind::Move, target);
        column_order.formation_id = Some("party_column".to_string());
        column.issue_order(column_order).unwrap();
        for _ in 0..180 {
            line.step().unwrap();
            column.step().unwrap();
        }
        assert_ne!(
            line.party
                .iter()
                .map(|unit| unit.position)
                .collect::<Vec<_>>(),
            column
                .party
                .iter()
                .map(|unit| unit.position)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn control_groups_and_shift_queue_are_authoritative_and_cancelable() {
        let mut sim = MissionSimV1::from_seed(seed()).unwrap();
        let subjects = sim.party[0..2]
            .iter()
            .map(|unit| unit.unit_id.clone())
            .collect::<Vec<_>>();
        let mut assign = RtsFrameOrder::new(
            1,
            "player",
            subjects.clone(),
            RtsOrderKind::AssignGroup,
            RtsOrderSource::LocalInput,
        );
        assign.target_rule_id = Some("2".to_string());
        sim.issue_order(assign).unwrap();
        assert_eq!(
            sim.control_group_members("2")
                .into_iter()
                .collect::<BTreeSet<_>>(),
            subjects.into_iter().collect::<BTreeSet<_>>()
        );

        let mut first = order(&sim, RtsOrderKind::Move, sim.seed.map.approach_point);
        first.frame = 2;
        first.queued = true;
        first.queue_id = Some("route-1".to_string());
        sim.issue_order(first).unwrap();
        let mut second = order(&sim, RtsOrderKind::Attack, sim.seed.map.objective);
        second.frame = 2;
        second.queued = true;
        second.queue_id = Some("route-2".to_string());
        sim.issue_order(second).unwrap();
        assert_eq!(sim.queued_orders.len(), 1);

        let mut cancel = RtsFrameOrder::new(
            2,
            "player",
            sim.party
                .iter()
                .map(|unit| unit.unit_id.clone())
                .collect::<Vec<_>>(),
            RtsOrderKind::CancelQueuedOrder,
            RtsOrderSource::LocalInput,
        );
        cancel.queue_id = Some("route-2".to_string());
        sim.issue_order(cancel).unwrap();
        assert!(sim.queued_orders.is_empty());
        sim.party[0].hp = 0;
        let dead_id = sim.party[0].unit_id.clone();
        sim.step().unwrap();
        assert!(!sim.control_group_members("2").contains(&dead_id));
        assert!(sim.validate().is_ok());
    }

    #[test]
    fn production_jobs_pause_promote_rally_cancel_and_refund_once() {
        let mut sim = MissionSimV1::from_seed(seed()).unwrap();
        sim.resources_gathered = 300;
        sim.resources_available = 300;
        let target = sim.seed.map.approach_point;
        sim.issue_order(job_order(
            &sim,
            RtsOrderKind::Train,
            "field_support_drone",
            target,
        ))
        .unwrap();
        let support_id = sim.jobs[0].job_id.clone();

        let lifecycle = |sim: &MissionSimV1, kind: RtsOrderKind, job_id: &str| {
            let mut order = RtsFrameOrder::new(
                sim.tick as u32,
                "player",
                sim.party
                    .iter()
                    .map(|unit| unit.unit_id.clone())
                    .collect::<Vec<_>>(),
                kind,
                RtsOrderSource::LocalInput,
            );
            order.queue_id = Some(job_id.to_string());
            order
        };
        sim.issue_order(lifecycle(&sim, RtsOrderKind::PauseJob, &support_id))
            .unwrap();
        let remaining = sim.jobs[0].remaining_ticks;
        for _ in 0..10 {
            sim.step().unwrap();
        }
        assert_eq!(sim.jobs[0].remaining_ticks, remaining);
        sim.issue_order(lifecycle(&sim, RtsOrderKind::ResumeJob, &support_id))
            .unwrap();

        let mut blocked_rally = lifecycle(&sim, RtsOrderKind::SetRally, &support_id);
        blocked_rally.target_tile = Some(RtsTile::new(999, 999));
        assert!(sim.issue_order(blocked_rally).is_err());
        let mut rally = lifecycle(&sim, RtsOrderKind::SetRally, &support_id);
        rally.target_tile = Some(RtsTile::new(target.x as i32, target.y as i32));
        sim.issue_order(rally).unwrap();
        assert_eq!(sim.jobs[0].target, target);

        sim.issue_order(job_order(&sim, RtsOrderKind::Train, "field_medic", target))
            .unwrap();
        let medic_id = sim.jobs[1].job_id.clone();
        sim.issue_order(lifecycle(&sim, RtsOrderKind::PromoteJob, &medic_id))
            .unwrap();
        assert_eq!(sim.jobs[0].kind, SimJobKind::TrainMedic);
        let before_cancel = sim.resources_available;
        sim.issue_order(lifecycle(&sim, RtsOrderKind::CancelJob, &medic_id))
            .unwrap();
        assert_eq!(sim.resources_available, before_cancel + 25);
        assert!(sim
            .issue_order(lifecycle(&sim, RtsOrderKind::CancelJob, &medic_id))
            .is_err());
        assert_eq!(sim.resources_available + sim.resources_spent, 300);
    }

    #[test]
    fn fog_hides_targets_until_deterministic_recon_reveals_them() {
        let mut sim = MissionSimV1::from_seed(seed()).unwrap();
        let enemy_id = sim.enemies[0].unit_id.clone();
        let enemy_position = sim.enemies[0].position;
        assert!(!sim.is_enemy_visible(&enemy_id));
        let mut hidden_attack = order(&sim, RtsOrderKind::Attack, enemy_position);
        hidden_attack.target_actor_id = Some(enemy_id.clone());
        assert!(sim.issue_order(hidden_attack).is_err());

        sim.resources_gathered = 20;
        sim.resources_available = 20;
        sim.issue_order(order(&sim, RtsOrderKind::Recon, enemy_position))
            .unwrap();
        assert!(sim.is_enemy_visible(&enemy_id));
        assert!(sim.explored_tiles.is_superset(&sim.visible_tiles));
        assert!(sim.visible_percent() > 0);
        sim.issue_order({
            let mut attack = order(&sim, RtsOrderKind::Attack, enemy_position);
            attack.target_actor_id = Some(enemy_id);
            attack
        })
        .unwrap();
    }

    #[test]
    fn expanded_medic_optics_armor_tree_changes_authoritative_combat_state() {
        let mut sim = MissionSimV1::from_seed(seed()).unwrap();
        sim.resources_gathered = 600;
        sim.resources_available = 600;
        let target = sim.seed.map.party_start;
        sim.issue_order(job_order(
            &sim,
            RtsOrderKind::Research,
            "field_logistics",
            target,
        ))
        .unwrap();
        for _ in 0..70 {
            sim.step().unwrap();
        }
        sim.issue_order(job_order(
            &sim,
            RtsOrderKind::Research,
            "signal_optics",
            target,
        ))
        .unwrap();
        for _ in 0..90 {
            sim.step().unwrap();
        }
        assert!(sim.researched_techs.contains("signal_optics"));

        let armor_before = sim.party[0].armor;
        sim.issue_order(job_order(
            &sim,
            RtsOrderKind::Upgrade,
            "field_armor",
            target,
        ))
        .unwrap();
        for _ in 0..75 {
            sim.step().unwrap();
        }
        assert_eq!(sim.armor_upgrade_level, 1);
        assert!(sim.party[0].armor > armor_before);

        sim.issue_order(job_order(&sim, RtsOrderKind::Train, "field_medic", target))
            .unwrap();
        for _ in 0..95 {
            sim.step().unwrap();
        }
        assert!(sim.support_units.iter().any(|unit| unit.role == "medic"));
        assert_eq!(sim.resources_available + sim.resources_spent, 600);
    }

    #[test]
    fn workers_carry_depleting_resources_back_to_the_command_post() {
        let mut sim = MissionSimV1::from_seed(seed()).unwrap();
        let node = sim.resource_nodes[0].position;
        sim.issue_order(order(&sim, RtsOrderKind::Harvest, node))
            .unwrap();
        step_until(
            &mut sim,
            |sim| sim.party.iter().any(|unit| unit.cargo > 0),
            800,
        );
        assert_eq!(
            sim.resources_gathered, 0,
            "cargo must not teleport into storage"
        );
        step_until(&mut sim, |sim| sim.resources_gathered > 0, 1_400);
        let carried = sim.party.iter().map(|unit| unit.cargo).sum::<u32>();
        assert_eq!(
            sim.resource_nodes[0].remaining + carried + sim.resources_gathered,
            RESOURCE_NODE_CAPACITY
        );
        assert_eq!(
            sim.resources_available + sim.resources_spent,
            sim.resources_gathered
        );
    }

    #[test]
    fn structures_supply_power_prerequisites_repair_and_blocking_are_authoritative() {
        let mut sim = MissionSimV1::from_seed(seed()).unwrap();
        sim.resources_gathered = 400;
        sim.resources_available = 400;
        let start = sim.seed.map.party_start;
        let workshop_tile = BattleGridPoint::new(start.x + 4, start.y);
        let mut workshop = order(&sim, RtsOrderKind::Build, workshop_tile);
        workshop.target_rule_id = Some("field_workshop".to_string());
        sim.issue_order(workshop).unwrap();
        assert!(sim.low_power());

        let job = job_order(&sim, RtsOrderKind::Research, "field_logistics", start);
        sim.issue_order(job).unwrap();
        let remaining = sim.jobs[0].remaining_ticks;
        for _ in 0..5 {
            sim.step().unwrap();
        }
        assert_eq!(sim.jobs[0].remaining_ticks, remaining);

        let generator_tile = BattleGridPoint::new(start.x + 5, start.y);
        let mut generator = order(&sim, RtsOrderKind::Build, generator_tile);
        generator.target_rule_id = Some("relay_generator".to_string());
        sim.issue_order(generator.clone()).unwrap();
        assert!(!sim.low_power());
        sim.step().unwrap();
        assert!(sim.jobs[0].remaining_ticks < remaining);
        assert!(
            sim.issue_order(generator).is_err(),
            "occupied build must fail"
        );

        let generator_id = sim
            .structures
            .iter()
            .find(|structure| structure.kind == SimStructureKind::RelayGenerator)
            .unwrap()
            .structure_id
            .clone();
        let generator_index = sim
            .structures
            .iter()
            .position(|structure| structure.structure_id == generator_id)
            .unwrap();
        sim.structures[generator_index].hp -= 200;
        let mut repair = order(&sim, RtsOrderKind::Repair, generator_tile);
        repair.target_actor_id = Some(generator_id);
        sim.issue_order(repair).unwrap();
        assert!(sim.structures[generator_index].hp > sim.structures[generator_index].max_hp - 200);

        let cap_before = sim.supply_cap();
        let mut supply = order(
            &sim,
            RtsOrderKind::Build,
            BattleGridPoint::new(start.x + 6, start.y),
        );
        supply.target_rule_id = Some("supply_cache".to_string());
        sim.issue_order(supply).unwrap();
        assert_eq!(sim.supply_cap(), cap_before + 4);
        assert_eq!(sim.resources_available + sim.resources_spent, 400);
        sim.validate().unwrap();
        let directory = tempdir().unwrap();
        let store = SimCheckpointStore::new(directory.path().join("economy.json"));
        store.save_atomic(&sim).unwrap();
        let resumed = store.load_for_seed(&sim.seed).unwrap().unwrap();
        assert_eq!(resumed, sim);
        assert_eq!(
            resumed.snapshot_hash().unwrap(),
            sim.snapshot_hash().unwrap()
        );
    }

    #[test]
    fn stance_patrol_stop_and_veterancy_survive_the_authoritative_result() {
        let mut sim = MissionSimV1::from_seed(seed()).unwrap();
        let hero = sim.party[0].unit_id.clone();
        let subjects = vec![hero.clone()];
        let mut stance = RtsFrameOrder::new(
            0,
            "player",
            subjects.clone(),
            RtsOrderKind::SetStance,
            RtsOrderSource::LocalInput,
        );
        stance.target_rule_id = Some(RtsUnitStance::Aggressive.as_str().to_string());
        sim.issue_order(stance).unwrap();
        assert_eq!(sim.party[0].stance, RtsUnitStance::Aggressive);

        let mut patrol = RtsFrameOrder::new(
            1,
            "player",
            subjects.clone(),
            RtsOrderKind::Patrol,
            RtsOrderSource::LocalInput,
        );
        patrol.target_tile = Some(RtsTile::new(
            sim.seed.map.approach_point.x as i32,
            sim.seed.map.approach_point.y as i32,
        ));
        sim.issue_order(patrol).unwrap();
        assert!(sim.party[0].patrol_target.is_some());
        let stop = RtsFrameOrder::new(
            2,
            "player",
            subjects.clone(),
            RtsOrderKind::Stop,
            RtsOrderSource::LocalInput,
        );
        sim.issue_order(stop).unwrap();
        assert!(sim.active_order.is_none() && sim.party[0].patrol_target.is_none());

        let selected = BTreeSet::from([hero]);
        let interval = sim.party[0].attack_interval_ticks as u64;
        for target_index in 0..2 {
            sim.enemies[target_index].position = sim.party[0].position;
            sim.enemies[target_index].hp = 1;
            sim.enemies[target_index].evasion_permille = 0;
            sim.tick = sim.tick.next_multiple_of(interval);
            let target_id = sim.enemies[target_index].unit_id.clone();
            sim.party_attack(&selected, Some(&target_id));
            sim.tick += 1;
        }
        assert_eq!(sim.party[0].confirmed_kills, 2);
        assert_eq!(sim.party[0].veteran_rank, 1);
        sim.outcome = Some(BattleOutcome::Withdrawal);
        sim.phase = BattlePhase::Complete;
        let result = sim.into_result().unwrap();
        let hero_report = result
            .units
            .iter()
            .find(|unit| unit.unit_id == "hero")
            .unwrap();
        assert_eq!(hero_report.confirmed_kills, 2);
        assert_eq!(hero_report.veteran_rank, 1);
    }

    #[test]
    fn reservation_yield_and_bounded_replan_keep_eight_actors_unique() {
        let mut baseline = MissionSimV1::from_seed(seed()).unwrap();
        for enemy in &mut baseline.enemies {
            enemy.hp = 0;
        }
        baseline.party[0].position = BattleGridPoint::new(5, 5);
        baseline.party[1].position = BattleGridPoint::new(1, 1);
        baseline.party[2].position = BattleGridPoint::new(2, 1);
        baseline.party[3].position = BattleGridPoint::new(3, 1);
        baseline.support_units = [
            BattleGridPoint::new(4, 5),
            BattleGridPoint::new(6, 5),
            BattleGridPoint::new(5, 4),
            BattleGridPoint::new(5, 6),
        ]
        .into_iter()
        .enumerate()
        .map(|(index, position)| SupportUnit {
            unit_id: format!("traffic_support_{index}"),
            archetype_id: "field_support_drone".to_string(),
            role: "support".to_string(),
            position,
            hp: 100,
            damage: 1,
            armor: 0,
            attack_range: 4,
            ability_cooldown_ticks: 0,
            attack_interval_ticks: 20,
            supply: 1,
        })
        .collect();
        let hero = BTreeSet::from([baseline.party[0].unit_id.clone()]);
        let target = BattleGridPoint::new(9, 5);
        for _ in 0..7 {
            baseline.party[0].movement_budget_milli = MOVEMENT_TILE_COST;
            baseline.move_selected_toward(&hero, target, 0, None);
        }
        let intent = baseline.move_intents.get("hero").unwrap();
        assert!(
            intent.replan_count >= 1,
            "blocked actor must enter bounded replan"
        );
        assert_eq!(baseline.party[0].position, BattleGridPoint::new(5, 5));

        baseline.support_units[1].position = BattleGridPoint::new(8, 8);
        baseline.party[0].movement_budget_milli = MOVEMENT_TILE_COST;
        baseline.move_selected_toward(&hero, target, 0, None);
        assert_ne!(baseline.party[0].position, BattleGridPoint::new(5, 5));
        let positions = baseline
            .party
            .iter()
            .map(|unit| unit.position)
            .chain(baseline.support_units.iter().map(|unit| unit.position))
            .collect::<BTreeSet<_>>();
        assert_eq!(positions.len(), 8);
        assert!(baseline.tile_reservations.len() <= 1);

        let first_hash = baseline.snapshot_hash().unwrap();
        let checkpoint = SimCheckpointV1::capture(&baseline).unwrap();
        let resumed: SimCheckpointV1 =
            serde_json::from_slice(&serde_json::to_vec(&checkpoint).unwrap()).unwrap();
        resumed.validate().unwrap();
        assert_eq!(resumed.sim.snapshot_hash().unwrap(), first_hash);
    }

    #[test]
    fn opposing_traffic_uses_a_stable_side_step() {
        let seed = seed();
        let left = BattleGridPoint::new(5, 5);
        let right = BattleGridPoint::new(6, 5);
        let first = deterministic_yield_step(
            &seed,
            left,
            BattleGridPoint::new(9, 5),
            &BTreeSet::from([right]),
            &[],
        )
        .expect("left actor must find a deterministic side step");
        let repeated = deterministic_yield_step(
            &seed,
            left,
            BattleGridPoint::new(9, 5),
            &BTreeSet::from([right]),
            &[],
        )
        .unwrap();
        assert_eq!(first, repeated);
        assert_ne!(first, right);

        let second = deterministic_yield_step(
            &seed,
            right,
            BattleGridPoint::new(2, 5),
            &BTreeSet::from([left, first]),
            &[],
        )
        .expect("right actor must yield without entering the reserved side step");
        assert_ne!(second, left);
        assert_ne!(second, first);
    }

    #[test]
    fn adaptive_ai_observes_budget_selects_goals_and_replays_deterministically() {
        let mut sim = MissionSimV1::from_seed(seed()).unwrap();
        sim.step().unwrap();
        assert_eq!(sim.enemy_ai_goal, AiGoal::Scout);
        let history_before_invalid = sim.enemy_ai_history.clone();
        let mut invalid = order(&sim, RtsOrderKind::Move, sim.seed.map.approach_point);
        invalid.player_id = "intruder".to_string();
        assert!(sim.issue_order(invalid).is_err());
        assert_eq!(sim.enemy_ai_history, history_before_invalid);

        sim.resources_gathered = 200;
        sim.resources_available = 200;
        while sim.tick < 50 {
            sim.step().unwrap();
        }
        assert_eq!(sim.enemy_ai_goal, AiGoal::RaidEconomy);
        sim.resources_spent = 200;
        sim.resources_available = 0;
        sim.researched_techs.insert("signal_optics".to_string());
        while sim.tick < 100 {
            sim.step().unwrap();
        }
        assert_eq!(sim.enemy_ai_goal, AiGoal::CounterTech);
        assert!(sim
            .enemy_ai_history
            .iter()
            .any(|decision| decision.goal == AiGoal::RaidEconomy));

        let checkpoint = SimCheckpointV1::capture(&sim).unwrap();
        let mut first = checkpoint.sim.clone();
        let mut second = checkpoint.sim;
        for _ in 0..25 {
            first.step().unwrap();
            second.step().unwrap();
        }
        assert_eq!(first.enemy_ai_history, second.enemy_ai_history);
        assert_eq!(
            first.snapshot_hash().unwrap(),
            second.snapshot_hash().unwrap()
        );
    }

    #[test]
    fn supplied_expedition_resources_enter_authoritative_conservation() {
        let mut campaign = CampaignSaveV1::default();
        campaign.move_to(CampaignRoom::MentorHall).unwrap();
        campaign.talk_to_mentor().unwrap();
        campaign.train_with_mentor().unwrap();
        campaign.equip_starter_weapon().unwrap();
        campaign.move_to(CampaignRoom::ExpeditionGate).unwrap();
        campaign.accept_first_contact_quest().unwrap();
        campaign.cycle_expedition_preparation().unwrap();
        campaign.cycle_expedition_preparation().unwrap();
        let supplied_seed = campaign.start_first_contact_battle(map()).unwrap();
        let sim = MissionSimV1::from_seed(supplied_seed).unwrap();
        assert_eq!(sim.resources_available, 50);
        assert_eq!(sim.resources_generated, 50);
        assert_eq!(
            sim.resources_available + sim.resources_spent,
            sim.resources_gathered
        );
        sim.validate().unwrap();
    }

    #[test]
    fn enemy_structures_are_explicit_attack_targets_and_never_take_proximity_damage() {
        let mut sim = MissionSimV1::from_seed(iron_delta_seed()).unwrap();
        let target_id = "enemy_supply_cache".to_string();
        let target_index = sim
            .enemy_structures
            .iter()
            .position(|structure| structure.structure_id == target_id)
            .unwrap();
        let target_position = sim.enemy_structures[target_index].position;
        let starting_hp = sim.enemy_structures[target_index].hp;
        for unit in &mut sim.party {
            unit.position = target_position;
            unit.attack_interval_ticks = 1;
        }
        for enemy in &mut sim.enemies {
            enemy.hp = 0;
        }
        sim.tick = 1;
        sim.step().unwrap();
        assert_eq!(sim.enemy_structures[target_index].hp, starting_hp);
        let selected = sim
            .party
            .iter()
            .map(|unit| unit.unit_id.clone())
            .collect::<BTreeSet<_>>();
        sim.party_attack(&selected, Some(&target_id));
        assert!(sim.enemy_structures[target_index].hp < starting_hp);
    }

    #[test]
    fn all_twelve_enemy_archetype_abilities_execute_as_typed_authority() {
        let mut sim = MissionSimV1::from_seed(iron_delta_seed()).unwrap();
        for unit in &mut sim.party {
            unit.max_hp = 100_000;
            unit.hp = 100_000;
            unit.max_energy = 1_000;
            unit.energy = 1_000;
        }
        for archetype in UNIT_ROSTER {
            sim.enemies[0].skill_ids = vec![archetype.ability().rule_id().to_string()];
            sim.enemies[0].ability_cooldown_ticks = 0;
            sim.activate_enemy_ability(0, 0, archetype.ability());
        }
        assert_eq!(sim.enemy_ability_activations.len(), 12);
        assert!(UNIT_ROSTER.iter().all(|unit| {
            sim.enemy_ability_activations
                .contains_key(unit.ability().rule_id())
        }));
    }

    #[test]
    fn exported_replay_reconstructs_the_exact_authoritative_snapshot() {
        let mut sim = MissionSimV1::from_seed(seed()).unwrap();
        sim.issue_order(order(&sim, RtsOrderKind::Move, sim.seed.map.approach_point))
            .unwrap();
        for _ in 0..80 {
            sim.step().unwrap();
        }
        let replay = sim.export_replay().unwrap();
        let replayed = replay.replay_and_verify().unwrap();
        assert_eq!(replayed.tick, sim.tick);
        assert_eq!(
            replayed.snapshot_hash().unwrap(),
            sim.snapshot_hash().unwrap()
        );
        let directory = tempdir().unwrap();
        let path = directory.path().join("battle-replay.json");
        replay.save_atomic(&path).unwrap();
        let loaded = BattleReplayV1::load_verified(&path).unwrap();
        assert_eq!(loaded.final_snapshot_hash, replay.final_snapshot_hash);
    }

    #[test]
    fn long_match_replay_retains_more_than_the_legacy_512_order_window() {
        let mut durable_seed = seed();
        for unit in &mut durable_seed.party {
            unit.stats.max_hp = 100_000;
            unit.stats.armor = 100;
        }
        durable_seed.seed_hash = durable_seed.computed_hash().unwrap();
        let mut sim = MissionSimV1::from_seed(durable_seed).unwrap();
        for _ in 0..900 {
            let target = if sim.tick.is_multiple_of(2) {
                sim.seed.map.party_start
            } else {
                sim.seed.map.approach_point
            };
            sim.issue_order(order(&sim, RtsOrderKind::Move, target))
                .unwrap();
            sim.step().unwrap();
        }
        assert_eq!(sim.replay_orders.len(), 900);
        let replay = sim.export_replay().unwrap();
        let replayed = replay.replay_and_verify().unwrap();
        assert_eq!(
            replayed.snapshot_hash().unwrap(),
            sim.snapshot_hash().unwrap()
        );
    }

    #[test]
    fn enemy_builder_must_reach_the_authored_site_before_construction_progresses() {
        let mut sim = MissionSimV1::from_seed(iron_delta_seed()).unwrap();
        sim.enemy_resources_available = 1_000;
        sim.enemy_resources_generated = 1_000;
        for worker in sim.enemies.iter_mut().filter(|unit| unit.role == "worker") {
            worker.position = sim.seed.map.party_start;
        }
        while sim.enemy_jobs.is_empty() {
            sim.step().unwrap();
        }
        let job = sim.enemy_jobs[0].clone();
        assert_eq!(job.kind, EnemyJobKind::Build);
        assert!(job.builder_id.is_some());
        assert!(job.target.is_some());
        let starting_ticks = job.remaining_ticks;
        sim.step().unwrap();
        if sim.enemy_jobs[0].remaining_ticks == starting_ticks {
            let builder_id = job.builder_id.as_deref().unwrap();
            let target = job.target.unwrap();
            assert!(sim
                .enemies
                .iter()
                .any(|unit| { unit.unit_id == builder_id && distance(unit.position, target) > 1 }));
        }
        let structures_before = sim.enemy_structures.len();
        while sim.enemy_structures.len() == structures_before && sim.tick < 900 {
            sim.step().unwrap();
        }
        assert!(sim.enemy_structures.len() > structures_before);
    }

    #[test]
    fn multi_map_faction_swap_multi_seed_balance_matrix_runs_real_authority() {
        let base = iron_delta_seed();
        let missions = [
            CampaignMission::IronDeltaSkirmish,
            CampaignMission::NightWatchCrossingSkirmish,
            CampaignMission::GlassBasinSkirmish,
            CampaignMission::EmberOrchardSkirmish,
        ];
        let mut seeds = Vec::new();
        for (map_index, mission) in missions.into_iter().enumerate() {
            for swapped in [false, true] {
                for seed_index in 0..3 {
                    let mut seed = base.clone();
                    seed.map_id = mission.map_id().to_string();
                    seed.map.terrain_rows[map_index] = if map_index.is_multiple_of(2) {
                        "ggggggggggggggggbbgg".to_string()
                    } else {
                        "ggggggggbbbbgggggggg".to_string()
                    };
                    seed.mission = MissionDefinition::for_mission(mission, &seed.map);
                    seed.battle_id = format!(
                        "balance-{}-{}-{seed_index}",
                        mission.map_id(),
                        u8::from(swapped)
                    );
                    seed.skirmish.player_faction = if swapped {
                        RtsFaction::AshenCompact
                    } else {
                        RtsFaction::MirrorCoalition
                    };
                    seed.skirmish.enemy_faction = seed.skirmish.player_faction.opponent();
                    seed.seed_hash = seed.computed_hash().unwrap();
                    seed.validate().unwrap();
                    seeds.push(seed);
                }
            }
        }
        let matrix = run_skirmish_balance_matrix(&seeds, 450).unwrap();
        assert_eq!(matrix.samples.len(), 24);
        assert_eq!(
            matrix
                .samples
                .iter()
                .map(|sample| sample.map_id.as_str())
                .collect::<BTreeSet<_>>()
                .len(),
            4
        );
        assert!(
            matrix.faction_pressure_delta_permille <= 350,
            "faction pressure delta {} permille exceeds the automated alpha band",
            matrix.faction_pressure_delta_permille
        );
    }

    #[test]
    fn difficulty_scales_enemy_pressure_and_ai_cadence_deterministically() {
        let base = seed();
        let with_difficulty = |difficulty| {
            let mut value = base.clone();
            value.difficulty = difficulty;
            value.seed_hash = value.computed_hash().unwrap();
            value
        };
        let mut story =
            MissionSimV1::from_seed(with_difficulty(CampaignDifficulty::Story)).unwrap();
        let mut standard =
            MissionSimV1::from_seed(with_difficulty(CampaignDifficulty::Standard)).unwrap();
        let mut veteran =
            MissionSimV1::from_seed(with_difficulty(CampaignDifficulty::Veteran)).unwrap();
        assert!(story.enemies[0].max_hp < standard.enemies[0].max_hp);
        assert!(standard.enemies[0].max_hp < veteran.enemies[0].max_hp);
        assert!(story.relay_guard_max_hp < standard.relay_guard_max_hp);
        assert!(standard.relay_guard_max_hp < veteran.relay_guard_max_hp);
        for _ in 0..105 {
            story.step().unwrap();
            standard.step().unwrap();
            veteran.step().unwrap();
        }
        assert!(story.enemy_ai_decision_index < standard.enemy_ai_decision_index);
        assert!(standard.enemy_ai_decision_index < veteran.enemy_ai_decision_index);

        let checkpoint = SimCheckpointV1::capture(&veteran).unwrap();
        let mut first = checkpoint.sim.clone();
        let mut second = checkpoint.sim;
        for _ in 0..35 {
            first.step().unwrap();
            second.step().unwrap();
        }
        assert_eq!(
            first.snapshot_hash().unwrap(),
            second.snapshot_hash().unwrap()
        );
    }
}
