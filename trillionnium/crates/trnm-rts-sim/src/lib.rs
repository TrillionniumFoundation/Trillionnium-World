//! Deterministic, Bevy-free First Contact battle simulation.
//!
//! The simulation consumes only a validated campaign seed and emits only a
//! campaign result. It never mutates RPG progression directly.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fmt, fs,
    io::Write,
    path::{Path, PathBuf},
};
use trnm_campaign_core::{
    BattleOutcome, BattleResultV1, BattleSeedV1, CampaignError, LootStack, UnitBattleReportV1,
    UnitBattleStatus, BATTLE_RESULT_CONTRACT,
};

pub const RTS_SIM_CONTRACT: &str = "trnm_rts_sim_v1";
pub const RTS_SIM_CHECKPOINT_CONTRACT: &str = "trnm_rts_sim_checkpoint_v1";
pub const TICKS_PER_SECOND: u64 = 10;
pub const TEN_MINUTE_TICKS: u64 = 10 * 60 * TICKS_PER_SECOND;
pub const FIFTEEN_MINUTE_TICKS: u64 = 15 * 60 * TICKS_PER_SECOND;
const RELAY_POSITION_MILLI: i32 = 100_000;
const ENGAGEMENT_RANGE_MILLI: i32 = 5_000;
const RELAY_GUARD_HP: i64 = 26_000;
const CAPTURE_TICKS_REQUIRED: u32 = 1_800;

#[derive(Debug)]
pub enum SimError {
    Campaign(CampaignError),
    InvalidState(String),
    Integrity(String),
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl fmt::Display for SimError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Campaign(error) => write!(formatter, "campaign contract rejected: {error}"),
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
pub enum SimCommand {
    Advance,
    Assault,
    Harvest,
    #[default]
    Hold,
    Retreat,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimUnit {
    pub unit_id: String,
    pub persistent: bool,
    pub max_hp: i64,
    pub hp: i64,
    pub damage: i64,
    pub armor: i64,
    pub move_speed_milli: i32,
    pub attack_interval_ticks: u32,
    pub evasion_permille: u16,
    pub position_milli: i32,
    pub attacks_made: u64,
}

impl SimUnit {
    pub fn alive(&self) -> bool {
        self.hp > 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MissionSimV1 {
    pub contract_version: String,
    pub seed: BattleSeedV1,
    pub tick: u64,
    pub command: SimCommand,
    pub party: Vec<SimUnit>,
    pub enemies: Vec<SimUnit>,
    pub relay_guard_hp: i64,
    pub relay_capture_ticks: u32,
    pub resources_gathered: u32,
    pub outcome: Option<BattleOutcome>,
    pub event_count: u64,
}

impl MissionSimV1 {
    pub fn from_seed(seed: BattleSeedV1) -> Result<Self, SimError> {
        seed.validate()?;
        let party = seed
            .party
            .iter()
            .map(|unit| SimUnit {
                unit_id: unit.unit_id.clone(),
                persistent: unit.persistent,
                max_hp: unit.stats.max_hp as i64,
                hp: unit.stats.max_hp as i64,
                damage: (unit.stats.damage as i64 * unit.stats.skill_power_permille as i64 / 1000)
                    .max(1),
                armor: unit.stats.armor as i64,
                move_speed_milli: unit.stats.move_speed_milli as i32,
                attack_interval_ticks: unit.stats.attack_interval_ticks.max(1),
                evasion_permille: unit.stats.evasion_permille,
                position_milli: 0,
                attacks_made: 0,
            })
            .collect();
        let enemies = [
            ("contact_scout", 820, 13, 4, 980, 18),
            ("contact_warden", 1_300, 16, 8, 720, 22),
            ("contact_striker", 960, 19, 5, 840, 17),
            ("relay_guard", 1_550, 17, 9, 620, 24),
        ]
        .into_iter()
        .enumerate()
        .map(
            |(index, (unit_id, hp, damage, armor, speed, interval))| SimUnit {
                unit_id: unit_id.to_string(),
                persistent: false,
                max_hp: hp,
                hp,
                damage,
                armor,
                move_speed_milli: speed,
                attack_interval_ticks: interval,
                evasion_permille: 30 + index as u16 * 10,
                position_milli: 72_000 + index as i32 * 3_000,
                attacks_made: 0,
            },
        )
        .collect();
        Ok(Self {
            contract_version: RTS_SIM_CONTRACT.to_string(),
            seed,
            tick: 0,
            command: SimCommand::Hold,
            party,
            enemies,
            relay_guard_hp: RELAY_GUARD_HP,
            relay_capture_ticks: 0,
            resources_gathered: 0,
            outcome: None,
            event_count: 0,
        })
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
            .collect::<std::collections::BTreeSet<_>>();
        let actual_ids = self
            .party
            .iter()
            .map(|unit| unit.unit_id.as_str())
            .collect::<std::collections::BTreeSet<_>>();
        if expected_ids != actual_ids || self.party.len() != self.seed.party.len() {
            return Err(SimError::Integrity(
                "simulation party does not match BattleSeed".to_string(),
            ));
        }
        Ok(())
    }

    pub fn terminal(&self) -> bool {
        self.outcome.is_some()
    }

    pub fn party_hp_percent(&self) -> u8 {
        let current = self.party.iter().map(|unit| unit.hp.max(0)).sum::<i64>();
        let maximum = self.party.iter().map(|unit| unit.max_hp).sum::<i64>();
        if maximum == 0 {
            0
        } else {
            (current * 100 / maximum).clamp(0, 100) as u8
        }
    }

    pub fn enemy_hp_percent(&self) -> u8 {
        let current = self.enemies.iter().map(|unit| unit.hp.max(0)).sum::<i64>();
        let maximum = self.enemies.iter().map(|unit| unit.max_hp).sum::<i64>();
        if maximum == 0 {
            0
        } else {
            (current * 100 / maximum).clamp(0, 100) as u8
        }
    }

    pub fn relay_guard_percent(&self) -> u8 {
        (self.relay_guard_hp.max(0) * 100 / RELAY_GUARD_HP).clamp(0, 100) as u8
    }

    pub fn capture_percent(&self) -> u8 {
        (self.relay_capture_ticks as u64 * 100 / CAPTURE_TICKS_REQUIRED as u64).min(100) as u8
    }

    pub fn step(&mut self, command: SimCommand) -> Result<(), SimError> {
        self.validate()?;
        if self.terminal() {
            return Err(SimError::InvalidState(
                "cannot advance a terminal battle".to_string(),
            ));
        }
        self.tick += 1;
        self.command = command;
        if command == SimCommand::Retreat {
            self.outcome = Some(BattleOutcome::Withdrawal);
            self.event_count += 1;
            return Ok(());
        }

        self.move_units();
        self.resolve_party_attacks();
        self.resolve_enemy_attacks();
        if command == SimCommand::Harvest && self.tick.is_multiple_of(50) {
            self.resources_gathered = self.resources_gathered.saturating_add(3);
            self.event_count += 1;
        }
        if self.party.iter().all(|unit| !unit.alive()) {
            self.outcome = Some(BattleOutcome::Defeat);
            self.event_count += 1;
        } else if self.relay_capture_ticks >= CAPTURE_TICKS_REQUIRED {
            self.outcome = Some(BattleOutcome::Victory);
            self.event_count += 1;
        }
        Ok(())
    }

    fn move_units(&mut self) {
        for unit in self.party.iter_mut().filter(|unit| unit.alive()) {
            let advance = match self.command {
                SimCommand::Advance | SimCommand::Assault => unit.move_speed_milli / 10,
                SimCommand::Harvest => unit.move_speed_milli / 25,
                SimCommand::Hold | SimCommand::Retreat => 0,
            };
            if let Some(nearest) = self
                .enemies
                .iter()
                .filter(|enemy| enemy.alive())
                .map(|enemy| enemy.position_milli)
                .min()
            {
                if nearest - unit.position_milli > ENGAGEMENT_RANGE_MILLI {
                    unit.position_milli = (unit.position_milli + advance).min(RELAY_POSITION_MILLI);
                }
            } else {
                unit.position_milli = (unit.position_milli + advance).min(RELAY_POSITION_MILLI);
            }
        }
        let living_party_front = self
            .party
            .iter()
            .filter(|unit| unit.alive())
            .map(|unit| unit.position_milli)
            .max()
            .unwrap_or(0);
        for enemy in self.enemies.iter_mut().filter(|enemy| enemy.alive()) {
            if enemy.position_milli - living_party_front > ENGAGEMENT_RANGE_MILLI {
                enemy.position_milli = (enemy.position_milli - enemy.move_speed_milli / 28).max(0);
            }
        }
    }

    fn resolve_party_attacks(&mut self) {
        if self.command != SimCommand::Assault {
            return;
        }
        for attacker_index in 0..self.party.len() {
            if !self.party[attacker_index].alive()
                || !self
                    .tick
                    .is_multiple_of(self.party[attacker_index].attack_interval_ticks as u64)
            {
                continue;
            }
            let attacker_position = self.party[attacker_index].position_milli;
            if let Some(target_index) = self.enemies.iter().position(|enemy| {
                enemy.alive()
                    && (enemy.position_milli - attacker_position).abs() <= ENGAGEMENT_RANGE_MILLI
            }) {
                let damage =
                    (self.party[attacker_index].damage - self.enemies[target_index].armor).max(1);
                if !deterministic_evade(
                    self.tick,
                    target_index,
                    self.enemies[target_index].evasion_permille,
                ) {
                    self.enemies[target_index].hp -= damage;
                }
                self.party[attacker_index].attacks_made += 1;
                self.event_count += 1;
            } else if self.enemies.iter().all(|enemy| !enemy.alive())
                && RELAY_POSITION_MILLI - attacker_position <= ENGAGEMENT_RANGE_MILLI
            {
                if self.relay_guard_hp > 0 {
                    self.relay_guard_hp -= self.party[attacker_index].damage.max(1);
                }
                self.party[attacker_index].attacks_made += 1;
                self.event_count += 1;
            }
        }
        if self.enemies.iter().all(|enemy| !enemy.alive())
            && self.relay_guard_hp <= 0
            && self
                .party
                .iter()
                .filter(|unit| unit.alive())
                .any(|unit| RELAY_POSITION_MILLI - unit.position_milli <= ENGAGEMENT_RANGE_MILLI)
        {
            self.relay_capture_ticks = self.relay_capture_ticks.saturating_add(1);
        }
    }

    fn resolve_enemy_attacks(&mut self) {
        for attacker_index in 0..self.enemies.len() {
            if !self.enemies[attacker_index].alive()
                || !self
                    .tick
                    .is_multiple_of(self.enemies[attacker_index].attack_interval_ticks as u64)
            {
                continue;
            }
            let attacker_position = self.enemies[attacker_index].position_milli;
            if let Some(target_index) = self.party.iter().position(|unit| {
                unit.alive()
                    && (unit.position_milli - attacker_position).abs() <= ENGAGEMENT_RANGE_MILLI
            }) {
                let hold_bonus = if self.command == SimCommand::Hold {
                    3
                } else {
                    0
                };
                let damage = (self.enemies[attacker_index].damage
                    - self.party[target_index].armor
                    - hold_bonus)
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
            BattleOutcome::Victory => 50,
            BattleOutcome::Defeat => 20,
            BattleOutcome::Withdrawal => 10,
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
        let (loot, reputation_delta, world_flags) = match outcome {
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
            BattleOutcome::Defeat => (Vec::new(), -2, vec!["first_contact_repulsed".to_string()]),
            BattleOutcome::Withdrawal => {
                (Vec::new(), -1, vec!["first_contact_withdrawn".to_string()])
            }
        };
        Ok(BattleResultV1 {
            contract_version: BATTLE_RESULT_CONTRACT.to_string(),
            battle_id: self.seed.battle_id.clone(),
            seed_hash: self.seed.seed_hash.clone(),
            outcome,
            units,
            loot,
            resource_delta: self.resources_gathered as i64,
            reputation_delta,
            world_flags,
            elapsed_ticks: self.tick,
            final_snapshot_hash,
        })
    }
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

    pub fn load(&self) -> Result<MissionSimV1, SimError> {
        let checkpoint: SimCheckpointV1 = serde_json::from_slice(&fs::read(&self.path)?)?;
        checkpoint.validate()?;
        Ok(checkpoint.sim)
    }

    pub fn load_for_seed(&self, seed: &BattleSeedV1) -> Result<Option<MissionSimV1>, SimError> {
        match self.load() {
            Ok(sim)
                if sim.seed.battle_id == seed.battle_id && sim.seed.seed_hash == seed.seed_hash =>
            {
                Ok(Some(sim))
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
    use trnm_campaign_core::{CampaignRoom, CampaignSaveV1};

    fn seed() -> BattleSeedV1 {
        let mut campaign = CampaignSaveV1::default();
        campaign.move_to(CampaignRoom::MentorHall).unwrap();
        campaign.talk_to_mentor().unwrap();
        campaign.train_with_mentor().unwrap();
        campaign.equip_starter_weapon().unwrap();
        campaign.move_to(CampaignRoom::ExpeditionGate).unwrap();
        campaign.accept_first_contact_quest().unwrap();
        campaign.start_first_contact_battle().unwrap()
    }

    fn run_to_terminal(sim: &mut MissionSimV1, command: SimCommand, limit: u64) {
        while !sim.terminal() && sim.tick < limit {
            sim.step(command).unwrap();
        }
        assert!(
            sim.terminal(),
            "battle did not terminate by tick {limit}: relay_hp={} capture={} party_hp={} enemy_hp={} party_pos={:?} enemy_pos={:?}",
            sim.relay_guard_hp,
            sim.relay_capture_ticks,
            sim.party_hp_percent(),
            sim.enemy_hp_percent(),
            sim.party.iter().map(|unit| unit.position_milli).collect::<Vec<_>>(),
            sim.enemies.iter().map(|unit| unit.position_milli).collect::<Vec<_>>()
        );
    }

    #[test]
    fn assault_produces_a_real_ten_to_fifteen_minute_victory() {
        let mut sim = MissionSimV1::from_seed(seed()).unwrap();
        run_to_terminal(&mut sim, SimCommand::Assault, FIFTEEN_MINUTE_TICKS);
        assert_eq!(sim.outcome, Some(BattleOutcome::Victory));
        assert!(
            sim.tick >= TEN_MINUTE_TICKS,
            "victory was too short: {}",
            sim.tick
        );
        assert!(sim.tick <= FIFTEEN_MINUTE_TICKS);
        let result = sim.into_result().unwrap();
        assert_eq!(result.outcome, BattleOutcome::Victory);
        assert!(!result.loot.is_empty());
    }

    #[test]
    fn passive_party_can_be_defeated_by_active_enemy_ai() {
        let mut sim = MissionSimV1::from_seed(seed()).unwrap();
        run_to_terminal(&mut sim, SimCommand::Hold, FIFTEEN_MINUTE_TICKS);
        assert_eq!(sim.outcome, Some(BattleOutcome::Defeat));
        assert!(sim.enemies.iter().any(|enemy| enemy.attacks_made > 0));
    }

    #[test]
    fn retreat_emits_withdrawal_and_preserves_party_reports() {
        let mut sim = MissionSimV1::from_seed(seed()).unwrap();
        for _ in 0..50 {
            sim.step(SimCommand::Advance).unwrap();
        }
        sim.step(SimCommand::Retreat).unwrap();
        let result = sim.into_result().unwrap();
        assert_eq!(result.outcome, BattleOutcome::Withdrawal);
        assert_eq!(result.units.len(), 4);
    }

    #[test]
    fn mid_battle_checkpoint_resume_is_bit_deterministic() {
        let seed = seed();
        let mut uninterrupted = MissionSimV1::from_seed(seed.clone()).unwrap();
        for _ in 0..2_500 {
            uninterrupted.step(SimCommand::Assault).unwrap();
        }
        let directory = tempdir().unwrap();
        let store = SimCheckpointStore::new(directory.path().join("battle.json"));
        store.save_atomic(&uninterrupted).unwrap();
        let mut resumed = store.load_for_seed(&seed).unwrap().unwrap();
        while !uninterrupted.terminal() {
            uninterrupted.step(SimCommand::Assault).unwrap();
        }
        while !resumed.terminal() {
            resumed.step(SimCommand::Assault).unwrap();
        }
        assert_eq!(resumed, uninterrupted);
        assert_eq!(
            resumed.snapshot_hash().unwrap(),
            uninterrupted.snapshot_hash().unwrap()
        );
    }

    #[test]
    fn tampered_checkpoint_is_rejected() {
        let mut checkpoint =
            SimCheckpointV1::capture(&MissionSimV1::from_seed(seed()).unwrap()).unwrap();
        checkpoint.sim.party[0].hp += 10_000;
        assert!(matches!(checkpoint.validate(), Err(SimError::Integrity(_))));
    }
}
