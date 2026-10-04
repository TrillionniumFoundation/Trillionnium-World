pub const RTS_SIM_CONTRACT: &str = "trnm_rts_sim_v16";
pub const RTS_SIM_CHECKPOINT_CONTRACT: &str = "trnm_rts_sim_checkpoint_v16";
pub const TICKS_PER_SECOND: u64 = 10;
pub const THREE_MINUTE_TICKS: u64 = 3 * 60 * TICKS_PER_SECOND;
pub const FIVE_MINUTE_TICKS: u64 = 5 * 60 * TICKS_PER_SECOND;
pub const SKIRMISH_TIME_LIMIT_TICKS: u64 = 12 * 60 * TICKS_PER_SECOND;
pub const TEN_MINUTE_TICKS: u64 = 10 * 60 * TICKS_PER_SECOND;
pub const FIFTEEN_MINUTE_TICKS: u64 = 15 * 60 * TICKS_PER_SECOND;
pub const REPLAY_SEEK_CHECKPOINT_TICKS: u64 = 300;
const MOVEMENT_TILE_COST: i32 = 10_000;
const CAPTURE_TICKS_REQUIRED: u32 = 602;
const RELAY_GUARD_HP: i64 = 5_400;
const WITHDRAWAL_MIN_TICKS: u64 = 30;
const FIELD_AID_COST: u32 = 20;
const RECON_COST: u32 = 10;
const TRAIN_SUPPORT_COST: u32 = 40;
const WORKER_CARGO_CAPACITY: u32 = 40;
const RESOURCE_NODE_CAPACITY: u32 = 800;
const MAX_REPLAY_ORDERS: usize = 65_536;
const REPLAY_CHUNK_ORDERS: usize = 512;

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

fn construction_job_ready(
    units: &[SimUnit],
    builder_id: Option<&str>,
    target: Option<BattleGridPoint>,
) -> bool {
    let (Some(builder_id), Some(target)) = (builder_id, target) else {
        return false;
    };
    units.iter().any(|unit| {
        unit.alive() && unit.unit_id == builder_id && distance(unit.position, target) <= 1
    })
}

fn advance_side_job_queue(jobs: &mut [SimJob], units: &[SimUnit], powered: bool) {
    let readiness = jobs
        .iter()
        .map(|job| {
            job.kind != SimJobKind::BuildStructure
                || construction_job_ready(units, job.builder_id.as_deref(), Some(job.target))
        })
        .collect::<Vec<_>>();
    for (job, ready) in jobs.iter_mut().zip(readiness) {
        if job.kind == SimJobKind::BuildStructure && !job.paused && ready {
            job.remaining_ticks = job.remaining_ticks.saturating_sub(1);
        }
    }
    if let Some(job) = jobs
        .iter_mut()
        .find(|job| job.kind != SimJobKind::BuildStructure && !job.paused && powered)
    {
        job.remaining_ticks = job.remaining_ticks.saturating_sub(1);
    }
}

fn advance_side_construction_worker(
    seed: &BattleSeedV1,
    units: &mut [SimUnit],
    opposing_units: &[SimUnit],
    jobs: &[SimJob],
) {
    let Some((builder_id, target)) = jobs
        .iter()
        .find(|job| job.kind == SimJobKind::BuildStructure && !job.paused)
        .and_then(|job| Some((job.builder_id.as_deref()?, job.target)))
    else {
        return;
    };
    let Some(index) = units
        .iter()
        .position(|unit| unit.alive() && unit.unit_id == builder_id)
    else {
        return;
    };
    if distance(units[index].position, target) <= 1 {
        return;
    }
    let occupied = units
        .iter()
        .chain(opposing_units)
        .filter(|unit| unit.alive() && unit.unit_id != builder_id)
        .map(|unit| unit.position)
        .collect::<BTreeSet<_>>();
    units[index].movement_budget_milli = units[index]
        .movement_budget_milli
        .saturating_add(units[index].move_speed_milli);
    if units[index].movement_budget_milli < MOVEMENT_TILE_COST {
        return;
    }
    if let Some(next) = next_step_toward(seed, units[index].position, target, 1, &occupied) {
        units[index].position = next;
        units[index].movement_budget_milli -= MOVEMENT_TILE_COST;
    }
}

#[allow(clippy::too_many_arguments)]
fn advance_worker_logistics(
    seed: &BattleSeedV1,
    tick: u64,
    units: &mut [SimUnit],
    opposing_units: &[SimUnit],
    jobs: &[SimJob],
    resource_nodes: &mut [ResourceNodeState],
    command_position: BattleGridPoint,
    selected: Option<&BTreeSet<String>>,
    preferred_node_index: Option<usize>,
    resources_available: &mut u32,
    resources_generated: &mut u32,
    score: &mut u32,
    event_count: &mut u64,
    restore_energy: bool,
) -> usize {
    let worker_indices = units
        .iter()
        .enumerate()
        .filter(|(_, unit)| {
            unit.alive()
                && selected
                    .map(|selected| selected.contains(&unit.unit_id))
                    .unwrap_or(unit.role == "worker")
        })
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    let mut occupied = units
        .iter()
        .chain(opposing_units)
        .filter(|unit| unit.alive())
        .map(|unit| unit.position)
        .collect::<BTreeSet<_>>();
    let construction_assignment = jobs
        .iter()
        .find(|job| job.kind == SimJobKind::BuildStructure && !job.paused)
        .and_then(|job| Some((job.builder_id.as_deref()?, job.target)));
    for index in worker_indices.iter().copied() {
        if construction_assignment.is_some_and(|(builder_id, _)| builder_id == units[index].unit_id)
        {
            continue;
        }
        let node_index = preferred_node_index
            .filter(|node_index| resource_nodes[*node_index].remaining > 0)
            .or_else(|| {
                resource_nodes
                    .iter()
                    .enumerate()
                    .filter(|(_, node)| node.remaining > 0)
                    .min_by_key(|(_, node)| distance(units[index].position, node.position))
                    .map(|(index, _)| index)
            });
        let returning = units[index].cargo >= units[index].cargo_capacity
            || (node_index.is_none() && units[index].cargo > 0);
        let target = if returning {
            command_position
        } else if let Some(node_index) = node_index {
            resource_nodes[node_index].position
        } else {
            command_position
        };
        units[index].movement_budget_milli = units[index]
            .movement_budget_milli
            .saturating_add(units[index].move_speed_milli);
        if units[index].movement_budget_milli >= MOVEMENT_TILE_COST
            && distance(units[index].position, target) > 1
        {
            occupied.remove(&units[index].position);
            if let Some(next) = next_step_toward(seed, units[index].position, target, 1, &occupied)
            {
                units[index].position = next;
                units[index].movement_budget_milli -= MOVEMENT_TILE_COST;
            }
            occupied.insert(units[index].position);
        }
        if returning
            && distance(units[index].position, command_position) <= 1
            && units[index].cargo > 0
        {
            let deposited = std::mem::take(&mut units[index].cargo);
            *resources_available = resources_available.saturating_add(deposited);
            *resources_generated = resources_generated.saturating_add(deposited);
            *score = score.saturating_add(deposited);
            *event_count += 1;
        } else if !returning && tick.is_multiple_of(10) {
            if let Some(node_index) = node_index {
                if distance(units[index].position, resource_nodes[node_index].position) <= 1 {
                    let room = units[index]
                        .cargo_capacity
                        .saturating_sub(units[index].cargo);
                    let gathered = 4_u32.min(room).min(resource_nodes[node_index].remaining);
                    units[index].cargo = units[index].cargo.saturating_add(gathered);
                    resource_nodes[node_index].remaining -= gathered;
                    if restore_energy {
                        units[index].energy =
                            (units[index].energy + 4).min(units[index].max_energy);
                    }
                    *event_count += u64::from(gathered > 0);
                }
            }
        }
    }
    worker_indices.len()
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
    BuildStructure,
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

fn research_job_kind(rule_id: &str) -> SimJobKind {
    match rule_id {
        "field_logistics" => SimJobKind::ResearchLogistics,
        "signal_optics" => SimJobKind::ResearchOptics,
        "relay_arms" => SimJobKind::UpgradeRelayArms,
        "field_armor" => SimJobKind::UpgradeFieldArmor,
        "sensor_net" => SimJobKind::ResearchSensorNet,
        "field_medicine" => SimJobKind::ResearchFieldMedicine,
        "siege_drills" => SimJobKind::UpgradeSiegeDrills,
        "reactive_plating" => SimJobKind::UpgradeReactivePlating,
        "wayfinder_drills" => SimJobKind::ResearchWayfinderDrills,
        "rapid_mustering" => SimJobKind::ResearchRapidMustering,
        _ => SimJobKind::ResearchLogistics,
    }
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
    #[serde(default)]
    pub builder_id: Option<String>,
    #[serde(default)]
    pub side: AuthoritySide,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthoritySide {
    #[default]
    Player,
    Enemy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityCommandSource {
    PlayerOrder,
    AdaptiveAi,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityJobCommandRecord {
    pub tick: u64,
    pub source: AuthorityCommandSource,
    pub side: AuthoritySide,
    pub job_id: String,
    pub kind: SimJobKind,
    pub rule_id: String,
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
pub struct BattleReplayChunkV2 {
    pub index: u32,
    pub first_tick: u64,
    pub last_tick: u64,
    pub entries: Vec<SimReplayEntry>,
    pub chunk_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BattleReplayV2 {
    pub contract_version: String,
    pub seed: BattleSeedV1,
    pub chunks: Vec<BattleReplayChunkV2>,
    #[serde(default)]
    pub seek_checkpoints: Vec<ReplaySeekCheckpointV2>,
    pub final_tick: u64,
    pub final_snapshot_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplaySeekCheckpointV2 {
    pub tick: u64,
    pub consumed_entry_count: usize,
    pub checkpoint: SimCheckpointV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ReplayChunkDirectoryManifestV1 {
    contract_version: String,
    seed: BattleSeedV1,
    chunk_files: Vec<String>,
    seek_checkpoints: Vec<ReplaySeekCheckpointV2>,
    final_tick: u64,
    final_snapshot_hash: String,
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
    pub winner_faction: Option<RtsFaction>,
    pub player_score: u32,
    pub enemy_score: u32,
    pub player_hp_percent: u8,
    pub enemy_hp_percent: u8,
    pub player_resources_gathered: u32,
    pub player_resources_spent: u32,
    pub enemy_resources_generated: u32,
    pub enemy_resources_spent: u32,
    pub player_tech_count: u8,
    pub enemy_tech_count: u8,
    pub player_resource_efficiency_permille: u16,
    pub enemy_resource_efficiency_permille: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkirmishBalanceMatrix {
    pub contract_version: String,
    pub samples: Vec<SkirmishBalanceSample>,
    pub faction_pressure_delta_permille: u16,
    pub terminal_sample_count: u32,
    pub mirror_wins: u32,
    pub ashen_wins: u32,
    pub average_terminal_ticks: u64,
    pub faction_win_delta_permille: u16,
    pub average_resource_efficiency_delta_permille: u16,
    pub average_tech_count_delta_permille: u16,
    pub terminal_samples_by_map: BTreeMap<String, u32>,
}

fn drive_balance_player_economy(sim: &mut MissionSimV1) -> Result<bool, SimError> {
    let subjects = sim
        .party
        .iter()
        .filter(|unit| unit.alive())
        .map(|unit| unit.unit_id.clone())
        .collect::<Vec<_>>();
    if subjects.is_empty() {
        return Ok(false);
    }
    if sim.resources_available < 200 {
        if let Some(node) = sim.resource_nodes.iter().find(|node| node.remaining > 0) {
            let mut order = RtsFrameOrder::new(
                sim.tick as u32,
                "player",
                subjects,
                RtsOrderKind::Harvest,
                trnm_rts_protocol::RtsOrderSource::Replay,
            );
            order.target_actor_id = Some(node.node_id.clone());
            order.target_tile = Some(trnm_rts_protocol::RtsTile::new(
                i32::from(node.position.x),
                i32::from(node.position.y),
            ));
            sim.issue_order(order)?;
            return Ok(true);
        }
    }
    let workshop_exists = sim
        .structures
        .iter()
        .any(|structure| structure.alive() && structure.kind == SimStructureKind::FieldWorkshop);
    let workshop_queued = sim
        .jobs
        .iter()
        .any(|job| job.kind == SimJobKind::BuildStructure && job.rule_id == "field_workshop");
    if !workshop_exists && !workshop_queued && sim.resources_available >= 55 {
        let target = nearest_passable(
            &sim.seed,
            BattleGridPoint::new(sim.seed.map.party_start.x + 2, sim.seed.map.party_start.y),
        )
        .unwrap_or(sim.seed.map.party_start);
        let mut order = RtsFrameOrder::new(
            sim.tick as u32,
            "player",
            subjects.clone(),
            RtsOrderKind::Build,
            trnm_rts_protocol::RtsOrderSource::Replay,
        );
        order.target_rule_id = Some("field_workshop".to_string());
        order.target_tile = Some(trnm_rts_protocol::RtsTile::new(
            i32::from(target.x),
            i32::from(target.y),
        ));
        if sim.issue_order(order).is_ok() {
            return Ok(true);
        }
    }
    if workshop_exists
        && !sim.researched_techs.contains("field_logistics")
        && !sim.jobs.iter().any(|job| job.rule_id == "field_logistics")
        && sim.resources_available >= 35
    {
        let mut order = RtsFrameOrder::new(
            sim.tick as u32,
            "player",
            subjects.clone(),
            RtsOrderKind::Research,
            trnm_rts_protocol::RtsOrderSource::Replay,
        );
        order.target_rule_id = Some("field_logistics".to_string());
        order.queue_id = Some(format!("balance-research-{}", sim.tick));
        if sim.issue_order(order).is_ok() {
            return Ok(true);
        }
    }
    if workshop_exists && sim.jobs.is_empty() {
        if let Some(unit) = UNIT_ROSTER
            .iter()
            .find(|unit| unit.faction == sim.seed.skirmish.player_faction)
        {
            let mut order = RtsFrameOrder::new(
                sim.tick as u32,
                "player",
                subjects,
                RtsOrderKind::Train,
                trnm_rts_protocol::RtsOrderSource::Replay,
            );
            order.target_rule_id = Some(unit.id.to_string());
            order.queue_id = Some(format!("balance-train-{}", sim.tick));
            let _ = sim.issue_order(order);
            return Ok(true);
        }
    }
    Ok(false)
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
    let mut wins = [0_u32; 2];
    let mut efficiency = [0_u64; 2];
    let mut tech_counts = [0_u64; 2];
    let mut terminal_ticks = Vec::new();
    let mut terminal_samples_by_map = BTreeMap::<String, u32>::new();
    let mut map_fingerprints = BTreeMap::<String, String>::new();
    for seed in seeds {
        if !seed.skirmish.enabled {
            return Err(SimError::InvalidState(
                "balance matrix accepts only configured skirmish seeds".to_string(),
            ));
        }
        let map_fingerprint = hash_json(&(
            seed.map.width,
            seed.map.height,
            &seed.map.terrain_rows,
            &seed.map.resource_nodes,
        ))?;
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
            let economy_issued = if sim.tick.is_multiple_of(40) {
                drive_balance_player_economy(&mut sim)?
            } else {
                false
            };
            if !economy_issued && (sim.tick.is_multiple_of(20) || sim.active_order.is_none()) {
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
        let winner_faction = if let Some(outcome) = sim.outcome {
            terminal_ticks.push(sim.tick);
            *terminal_samples_by_map
                .entry(seed.map_id.clone())
                .or_default() += 1;
            let winner = match outcome {
                BattleOutcome::Victory => player_faction_index,
                BattleOutcome::Defeat => enemy_faction_index,
                BattleOutcome::Withdrawal => 2,
            };
            if winner < 2 {
                wins[winner] = wins[winner].saturating_add(1);
            }
            (winner < 2).then_some(if winner == 0 {
                RtsFaction::MirrorCoalition
            } else {
                RtsFaction::AshenCompact
            })
        } else {
            None
        };
        let player_resource_efficiency_permille = (u64::from(sim.resources_spent) * 1000
            / u64::from(
                seed.skirmish
                    .starting_resources
                    .saturating_add(sim.resources_gathered)
                    .max(1),
            ))
        .min(1000) as u16;
        let enemy_resource_efficiency_permille = (u64::from(sim.enemy_resources_spent) * 1000
            / u64::from(sim.enemy_resources_generated.max(1)))
        .min(1000) as u16;
        efficiency[player_faction_index] += u64::from(player_resource_efficiency_permille);
        efficiency[enemy_faction_index] += u64::from(enemy_resource_efficiency_permille);
        tech_counts[player_faction_index] += sim.researched_techs.len() as u64;
        tech_counts[enemy_faction_index] += sim.enemy_researched_techs.len() as u64;
        samples.push(SkirmishBalanceSample {
            map_id: seed.map_id.clone(),
            player_faction: seed.skirmish.player_faction,
            enemy_faction: seed.skirmish.enemy_faction,
            seed_hash: seed.seed_hash.clone(),
            map_fingerprint,
            simulation_salt: simulation_salt(seed),
            final_tick: sim.tick,
            outcome: sim.outcome,
            winner_faction,
            player_score: sim.player_score,
            enemy_score: sim.enemy_score,
            player_hp_percent: player_hp,
            enemy_hp_percent: enemy_hp,
            player_resources_gathered: sim.resources_gathered,
            player_resources_spent: sim.resources_spent,
            enemy_resources_generated: sim.enemy_resources_generated,
            enemy_resources_spent: sim.enemy_resources_spent,
            player_tech_count: sim.researched_techs.len().min(u8::MAX as usize) as u8,
            enemy_tech_count: sim.enemy_researched_techs.len().min(u8::MAX as usize) as u8,
            player_resource_efficiency_permille,
            enemy_resource_efficiency_permille,
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
        contract_version: "trnm_skirmish_balance_matrix_v3".to_string(),
        samples,
        faction_pressure_delta_permille: delta,
        terminal_sample_count: terminal_ticks.len() as u32,
        mirror_wins: wins[0],
        ashen_wins: wins[1],
        average_terminal_ticks: if terminal_ticks.is_empty() {
            0
        } else {
            terminal_ticks.iter().sum::<u64>() / terminal_ticks.len() as u64
        },
        faction_win_delta_permille: {
            let total = u64::from(wins[0] + wins[1]).max(1);
            (u64::from(wins[0].abs_diff(wins[1])) * 1000 / total) as u16
        },
        average_resource_efficiency_delta_permille: {
            let high = efficiency[0].max(efficiency[1]).max(1);
            (efficiency[0].abs_diff(efficiency[1]) * 1000 / high) as u16
        },
        average_tech_count_delta_permille: {
            let high = tech_counts[0].max(tech_counts[1]).max(1);
            (tech_counts[0].abs_diff(tech_counts[1]) * 1000 / high) as u16
        },
        terminal_samples_by_map,
    })
}

