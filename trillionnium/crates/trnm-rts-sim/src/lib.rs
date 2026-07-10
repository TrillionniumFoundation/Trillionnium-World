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
    BattleGridPoint, BattleOutcome, BattleResultV1, BattleSeedV1, CampaignError, LootStack,
    UnitBattleReportV1, UnitBattleStatus, BATTLE_RESULT_CONTRACT,
};
use trnm_rts_protocol::{RtsFrameOrder, RtsOrderKind};

pub const RTS_SIM_CONTRACT: &str = "trnm_rts_sim_v4";
pub const RTS_SIM_CHECKPOINT_CONTRACT: &str = "trnm_rts_sim_checkpoint_v4";
pub const TICKS_PER_SECOND: u64 = 10;
pub const THREE_MINUTE_TICKS: u64 = 3 * 60 * TICKS_PER_SECOND;
pub const FIVE_MINUTE_TICKS: u64 = 5 * 60 * TICKS_PER_SECOND;
pub const TEN_MINUTE_TICKS: u64 = 10 * 60 * TICKS_PER_SECOND;
pub const FIFTEEN_MINUTE_TICKS: u64 = 15 * 60 * TICKS_PER_SECOND;
const MOVEMENT_TILE_COST: i32 = 10_000;
const CAPTURE_TICKS_REQUIRED: u32 = 600;
const RELAY_GUARD_HP: i64 = 5_400;
const WITHDRAWAL_MIN_TICKS: u64 = 30;
const FIELD_AID_COST: u32 = 20;
const FORTIFY_COST: u32 = 30;
const RECON_COST: u32 = 10;
const TRAIN_SUPPORT_COST: u32 = 40;
const RESEARCH_COST: u32 = 35;
const UPGRADE_COST: u32 = 45;

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
}

impl SimUnit {
    pub fn alive(&self) -> bool {
        self.hp > 0
    }

    fn attack_range(&self) -> i16 {
        match self.role.as_str() {
            "scout" | "engineer" | "mystic" => 3,
            "medic" => 2,
            _ => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SimJobKind {
    TrainSupport,
    TrainMedic,
    ResearchLogistics,
    ResearchOptics,
    UpgradeRelayArms,
    UpgradeFieldArmor,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportUnit {
    pub unit_id: String,
    #[serde(default = "default_support_role")]
    pub role: String,
    pub position: BattleGridPoint,
    pub hp: i64,
    pub damage: i64,
    pub attack_interval_ticks: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MissionSimV1 {
    pub contract_version: String,
    pub seed: BattleSeedV1,
    pub tick: u64,
    pub phase: BattlePhase,
    pub active_order: Option<RtsFrameOrder>,
    #[serde(default)]
    pub queued_orders: VecDeque<RtsFrameOrder>,
    #[serde(default)]
    pub control_groups: BTreeMap<String, BTreeSet<String>>,
    pub last_order_frame: Option<u32>,
    pub order_count: u32,
    pub distinct_order_kinds: BTreeSet<String>,
    pub party: Vec<SimUnit>,
    pub enemies: Vec<SimUnit>,
    pub relay_guard_hp: i64,
    pub relay_guard_max_hp: i64,
    pub relay_capture_ticks: u32,
    pub resources_gathered: u32,
    pub resources_available: u32,
    pub resources_spent: u32,
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
            })
            .collect();
        let enemy_profiles = [
            ("scout", 800, 8, 3, 1_050, 18),
            ("warden", 1_200, 10, 7, 760, 24),
            ("striker", 900, 12, 4, 900, 20),
            ("relay_guard", 1_400, 11, 8, 680, 26),
        ];
        let aftershock = is_aftershock_map(&seed.map_id);
        let enemy_scale = if aftershock { 112 } else { 100 };
        let enemies = seed
            .map
            .enemy_spawns
            .iter()
            .enumerate()
            .map(|(index, spawn)| {
                let (role, hp, damage, armor, speed, interval) =
                    enemy_profiles[index.min(enemy_profiles.len() - 1)];
                SimUnit {
                    unit_id: spawn.id.clone(),
                    role: role.to_string(),
                    persistent: false,
                    skill_ids: Vec::new(),
                    max_hp: hp * enemy_scale / 100,
                    hp: hp * enemy_scale / 100,
                    damage: damage * enemy_scale / 100,
                    armor: armor + if aftershock { 1 } else { 0 },
                    move_speed_milli: speed,
                    movement_budget_milli: 0,
                    attack_interval_ticks: interval,
                    evasion_permille: 25 + index as u16 * 10,
                    energy: 0,
                    max_energy: 0,
                    ability_range: 1,
                    ability_cooldown_ticks: 0,
                    guard_ticks: 0,
                    position: spawn.position,
                    attacks_made: 0,
                }
            })
            .collect();
        let relay_guard_max_hp = if aftershock {
            RELAY_GUARD_HP + 600
        } else {
            RELAY_GUARD_HP
        };
        let mut sim = Self {
            contract_version: RTS_SIM_CONTRACT.to_string(),
            seed,
            tick: 0,
            phase: BattlePhase::Approach,
            active_order: None,
            queued_orders: VecDeque::new(),
            control_groups: BTreeMap::new(),
            last_order_frame: None,
            order_count: 0,
            distinct_order_kinds: BTreeSet::new(),
            party,
            enemies,
            relay_guard_hp: relay_guard_max_hp,
            relay_guard_max_hp,
            relay_capture_ticks: 0,
            resources_gathered: 0,
            resources_available: 0,
            resources_spent: 0,
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
            || self.relay_guard_max_hp < RELAY_GUARD_HP
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
        Ok(())
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
        if matches!(order.kind, RtsOrderKind::Attack | RtsOrderKind::FocusFire)
            && order.target_actor_id.as_deref().is_some_and(|target| {
                self.enemies
                    .iter()
                    .any(|enemy| enemy.unit_id == target && enemy.alive())
                    && !self.is_enemy_visible(target)
            })
        {
            return Err(SimError::Order(
                "target enemy is outside current line of sight".to_string(),
            ));
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
                RtsOrderKind::Repair => self.resolve_field_aid(&order)?,
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
                RtsOrderKind::Move
                | RtsOrderKind::AttackMove
                | RtsOrderKind::Harvest
                | RtsOrderKind::Capture
                | RtsOrderKind::Attack
                | RtsOrderKind::FocusFire
                | RtsOrderKind::Hold => {
                    self.queued_orders.clear();
                    self.active_order = Some(order.clone());
                }
                RtsOrderKind::Extract => unreachable!("withdrawal handled above"),
            }
        }
        self.last_order_frame = Some(order.frame);
        self.order_count = self.order_count.saturating_add(1);
        self.distinct_order_kinds
            .insert(order.kind.as_str().to_string());
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
            RtsOrderKind::Attack | RtsOrderKind::FocusFire => order
                .target_actor_id
                .as_deref()
                .and_then(|id| self.enemies.iter().find(|enemy| enemy.unit_id == id))
                .map_or(
                    self.phase == BattlePhase::Relay
                        && self.relay_guard_hp <= 0
                        && self.enemies.iter().all(|enemy| !enemy.alive()),
                    |enemy| !enemy.alive(),
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
        for unit in &mut self.party {
            unit.ability_cooldown_ticks = unit.ability_cooldown_ticks.saturating_sub(1);
            unit.guard_ticks = unit.guard_ticks.saturating_sub(1);
            if self.tick.is_multiple_of(50) {
                unit.energy = (unit.energy + 1).min(unit.max_energy);
            }
        }
        self.resolve_player_order();
        self.update_phase();
        self.resolve_enemy_ai();
        self.resolve_support_fire();
        self.resolve_relay_pressure();
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
        } else if self.relay_capture_ticks >= CAPTURE_TICKS_REQUIRED
            && self.enemies.iter().all(|enemy| !enemy.alive())
        {
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
            | RtsOrderKind::Extract => {}
        }
    }

    fn update_phase(&mut self) {
        if self.phase == BattlePhase::Approach
            && self
                .party
                .iter()
                .filter(|unit| unit.alive())
                .any(|unit| distance(unit.position, self.seed.map.approach_point) <= 2)
        {
            self.phase = BattlePhase::Contact;
            self.event_count += 1;
        }
        if self.phase == BattlePhase::Contact && self.enemies.iter().all(|unit| !unit.alive()) {
            self.phase = BattlePhase::Relay;
            self.event_count += 1;
        }
    }

    fn move_selected_toward(
        &mut self,
        selected: &BTreeSet<String>,
        target: BattleGridPoint,
        stop_range: i16,
        formation_id: Option<&str>,
    ) {
        let mut occupied = self
            .party
            .iter()
            .chain(&self.enemies)
            .filter(|unit| unit.alive())
            .map(|unit| unit.position)
            .collect::<BTreeSet<_>>();
        for index in 0..self.party.len() {
            if !self.party[index].alive() || !selected.contains(&self.party[index].unit_id) {
                continue;
            }
            self.party[index].movement_budget_milli += self.party[index].move_speed_milli;
            if self.party[index].movement_budget_milli < MOVEMENT_TILE_COST {
                continue;
            }
            occupied.remove(&self.party[index].position);
            let formation_target =
                formation_target_for(target, index, formation_id.unwrap_or("none"), &self.seed);
            if let Some(next) = next_step_toward(
                &self.seed,
                self.party[index].position,
                formation_target,
                stop_range,
                &occupied,
            ) {
                self.party[index].position = next;
                self.party[index].movement_budget_milli -= MOVEMENT_TILE_COST;
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
        }
        self.enemies
            .iter()
            .find(|enemy| enemy.alive())
            .map(|enemy| enemy.position)
            .or(Some(self.seed.map.objective))
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
            let target_index = requested
                .and_then(|id| {
                    self.enemies
                        .iter()
                        .position(|enemy| enemy.unit_id == id && enemy.alive())
                })
                .or_else(|| {
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
                    let damage = (self.party[attacker_index].damage + intel_bonus
                        - self.enemies[target_index].armor)
                        .max(1);
                    if !deterministic_evade(
                        self.tick,
                        target_index,
                        self.enemies[target_index].evasion_permille,
                    ) {
                        self.enemies[target_index].hp -= damage;
                    }
                    self.party[attacker_index].attacks_made += 1;
                    self.event_count += 1;
                }
            } else if self.phase == BattlePhase::Relay
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
        let Some(node) = order
            .target_actor_id
            .as_deref()
            .and_then(|id| {
                self.seed
                    .map
                    .resource_nodes
                    .iter()
                    .find(|node| node.id == id)
            })
            .or_else(|| self.seed.map.resource_nodes.first())
            .cloned()
        else {
            return;
        };
        self.move_selected_toward(selected, node.position, 1, None);
        let workers = self
            .party
            .iter()
            .filter(|unit| {
                unit.alive()
                    && selected.contains(&unit.unit_id)
                    && distance(unit.position, node.position) <= 1
            })
            .count() as u32;
        if workers > 0 && self.tick.is_multiple_of(10) {
            let harvested = workers.saturating_mul(4);
            self.resources_gathered = self.resources_gathered.saturating_add(harvested);
            self.resources_available = self.resources_available.saturating_add(harvested);
            for unit in &mut self.party {
                if selected.contains(&unit.unit_id) && distance(unit.position, node.position) <= 1 {
                    unit.energy = (unit.energy + 4).min(unit.max_energy);
                }
            }
            self.event_count += 1;
        }
    }

    fn resolve_capture(&mut self, selected: &BTreeSet<String>) {
        if self.phase != BattlePhase::Relay
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

    fn resolve_fortify(&mut self, order: &RtsFrameOrder) -> Result<(), SimError> {
        self.spend_resources(FORTIFY_COST, "fortification")?;
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
            (RtsOrderKind::Train, _) => SimJobKind::TrainSupport,
            (RtsOrderKind::Research, "signal_optics") => SimJobKind::ResearchOptics,
            (RtsOrderKind::Research, _) => SimJobKind::ResearchLogistics,
            (RtsOrderKind::Upgrade, "field_armor") => SimJobKind::UpgradeFieldArmor,
            (RtsOrderKind::Upgrade, _) => SimJobKind::UpgradeRelayArms,
            _ => return Err(SimError::Order("unsupported job order".to_string())),
        };
        self.queue_job(order, kind)
    }

    fn queue_job(&mut self, order: &RtsFrameOrder, kind: SimJobKind) -> Result<(), SimError> {
        let rule_id = order
            .target_rule_id
            .clone()
            .ok_or_else(|| SimError::Order("job rule is required".to_string()))?;
        if self.jobs.iter().any(|job| job.kind == kind) {
            return Err(SimError::Order(
                "that production or technology job is already queued".to_string(),
            ));
        }
        let (cost, duration, label) = match kind {
            SimJobKind::TrainSupport => (TRAIN_SUPPORT_COST, 80, "support production"),
            SimJobKind::TrainMedic => (TRAIN_SUPPORT_COST + 10, 95, "field medic production"),
            SimJobKind::ResearchLogistics => {
                if self.researched_techs.contains("field_logistics") {
                    return Err(SimError::Order(
                        "field logistics is already researched".to_string(),
                    ));
                }
                (RESEARCH_COST, 70, "field logistics research")
            }
            SimJobKind::ResearchOptics => {
                if !self.researched_techs.contains("field_logistics") {
                    return Err(SimError::Order(
                        "research field logistics before signal optics".to_string(),
                    ));
                }
                if self.researched_techs.contains("signal_optics") {
                    return Err(SimError::Order(
                        "signal optics is already researched".to_string(),
                    ));
                }
                (RESEARCH_COST + 15, 90, "signal optics research")
            }
            SimJobKind::UpgradeRelayArms => {
                if !self.researched_techs.contains("field_logistics") {
                    return Err(SimError::Order(
                        "research field logistics before upgrading relay arms".to_string(),
                    ));
                }
                if self.upgrade_level >= 3 {
                    return Err(SimError::Order(
                        "relay arms upgrade cap reached".to_string(),
                    ));
                }
                (UPGRADE_COST, 60, "relay arms upgrade")
            }
            SimJobKind::UpgradeFieldArmor => {
                if !self.researched_techs.contains("field_logistics") {
                    return Err(SimError::Order(
                        "research field logistics before upgrading field armor".to_string(),
                    ));
                }
                if self.armor_upgrade_level >= 3 {
                    return Err(SimError::Order(
                        "field armor upgrade cap reached".to_string(),
                    ));
                }
                (UPGRADE_COST + 10, 75, "field armor upgrade")
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
        if let Some(job) = self.jobs.first_mut().filter(|job| !job.paused) {
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
                    let position = nearest_passable(&self.seed, job.target)
                        .unwrap_or(self.seed.map.party_start);
                    self.support_units.push(SupportUnit {
                        unit_id: format!("field_support_{}", self.support_units.len() + 1),
                        role: "support".to_string(),
                        position,
                        hp: 240,
                        damage: 18 + i64::from(self.upgrade_level) * 5,
                        attack_interval_ticks: 18,
                    });
                }
                SimJobKind::TrainMedic => {
                    let position = nearest_passable(&self.seed, job.target)
                        .unwrap_or(self.seed.map.party_start);
                    self.support_units.push(SupportUnit {
                        unit_id: format!("field_medic_{}", self.support_units.len() + 1),
                        role: "medic".to_string(),
                        position,
                        hp: 210,
                        damage: 6,
                        attack_interval_ticks: 20,
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
                    self.armor_upgrade_level = self.armor_upgrade_level.saturating_add(1).min(3);
                    for unit in &mut self.party {
                        unit.armor += 2;
                        unit.max_hp += 25;
                        unit.hp += 25;
                    }
                }
            }
            self.event_count += 1;
        }
    }

    fn resolve_support_fire(&mut self) {
        for support_index in 0..self.support_units.len() {
            if !self
                .tick
                .is_multiple_of(self.support_units[support_index].attack_interval_ticks as u64)
            {
                continue;
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
                    distance(self.support_units[support_index].position, enemy.position) <= 4
                })
                .min_by_key(|(_, enemy)| {
                    distance(self.support_units[support_index].position, enemy.position)
                })
                .map(|(index, _)| index);
            if let Some(target) = target {
                self.enemies[target].hp -= self.support_units[support_index].damage;
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
                position: spawn.position,
                attacks_made: 0,
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

    fn resolve_enemy_ai(&mut self) {
        if self.phase == BattlePhase::Approach && self.tick < 300 {
            return;
        }
        let mut occupied = self
            .party
            .iter()
            .chain(&self.enemies)
            .filter(|unit| unit.alive())
            .map(|unit| unit.position)
            .collect::<BTreeSet<_>>();
        for attacker_index in 0..self.enemies.len() {
            if !self.enemies[attacker_index].alive() {
                continue;
            }
            let Some(target_index) = self
                .party
                .iter()
                .enumerate()
                .filter(|(_, unit)| unit.alive())
                .min_by_key(|(_, unit)| {
                    let wounded_bias = unit.hp * 100 / unit.max_hp.max(1);
                    let role_bias = if self.enemy_tactics_level >= 2 && unit.role == "engineer" {
                        -3
                    } else {
                        0
                    };
                    distance(self.enemies[attacker_index].position, unit.position) as i64 * 10
                        + wounded_bias
                        + role_bias
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
                let damage = (self.enemies[attacker_index].damage
                    - self.party[target_index].armor
                    - hold_bonus
                    - guard_bonus)
                    .max(1);
                if !deterministic_evade(
                    self.tick,
                    target_index + 31,
                    self.party[target_index].evasion_permille,
                ) {
                    self.party[target_index].hp -= damage;
                }
                self.enemies[attacker_index].attacks_made += 1;
                self.event_count += 1;
            }
        }
    }

    fn resolve_relay_pressure(&mut self) {
        if self.phase != BattlePhase::Relay
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

    pub fn into_result(self) -> Result<BattleResultV1, SimError> {
        self.validate()?;
        let outcome = self.outcome.ok_or_else(|| {
            SimError::InvalidState("cannot emit a BattleResult before terminal state".to_string())
        })?;
        let final_snapshot_hash = self.snapshot_hash()?;
        let experience = match outcome {
            BattleOutcome::Victory if is_aftershock_map(&self.seed.map_id) => 55,
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
                }
            })
            .collect();
        let aftershock = is_aftershock_map(&self.seed.map_id);
        let (loot, reputation_delta, world_flags) = match outcome {
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
                vec![if aftershock {
                    "aftershock_patrol_repulsed".to_string()
                } else {
                    "first_contact_repulsed".to_string()
                }],
            ),
            BattleOutcome::Withdrawal => (
                Vec::new(),
                0,
                vec![if aftershock {
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

fn deterministic_evade(tick: u64, unit_index: usize, evasion_permille: u16) -> bool {
    ((tick.wrapping_mul(37) + unit_index as u64 * 101) % 1000) < evasion_permille as u64
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
    use trnm_campaign_core::{BattleMapNodeV1, BattleMapSeedV1, CampaignRoom, CampaignSaveV1};
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
        assert!(sim.resources_spent >= FIELD_AID_COST + FORTIFY_COST);
        assert_eq!(sim.enemy_tactics_level, 2);
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
}
