//! Authoritative RPG -> RTS -> RPG campaign contracts.
//!
//! This crate is deliberately independent from Bevy. Presentation clients may
//! request a [`BattleSeedV1`] and submit a [`BattleResultV1`], but only this
//! aggregate is allowed to mutate persistent RPG progression.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt, fs,
    io::Write,
    path::{Path, PathBuf},
};
use trnm_world_domain::{TrillionniumAttributes, WorldTrillionniumCharacter};

pub const CAMPAIGN_SAVE_CONTRACT: &str = "trnm_campaign_save_v1";
pub const BATTLE_SEED_CONTRACT: &str = "trnm_battle_seed_v1";
pub const BATTLE_RESULT_CONTRACT: &str = "trnm_battle_result_v1";
pub const SETTLEMENT_RECEIPT_CONTRACT: &str = "trnm_settlement_receipt_v1";
pub const FIRST_CONTACT_RULES_VERSION: &str = "first_contact_campaign_rules_v1";

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
}

impl CampaignRoom {
    pub fn title(self) -> &'static str {
        match self {
            Self::MirrorSquare => "镜城广场",
            Self::MentorHall => "街指南师父居",
            Self::ExpeditionGate => "First Contact 出征口",
        }
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
    pub injury_level: u8,
    pub available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CampaignProgression {
    pub level: u32,
    pub experience: u64,
    pub skill_progress: BTreeMap<String, SkillProgress>,
    pub inventory: Vec<LootStack>,
    pub world_flags: BTreeSet<String>,
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
    pub stats: RtsUnitStats,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BattleSeedV1 {
    pub contract_version: String,
    pub battle_id: String,
    pub campaign_revision: u64,
    pub map_id: String,
    pub rules_version: String,
    pub party: Vec<BattleUnitSeedV1>,
    pub seed_hash: String,
}

impl BattleSeedV1 {
    pub fn validate(&self) -> Result<(), CampaignError> {
        if self.contract_version != BATTLE_SEED_CONTRACT {
            return Err(CampaignError::InvalidContract(
                self.contract_version.clone(),
            ));
        }
        if self.map_id != "first_contact" || self.rules_version != FIRST_CONTACT_RULES_VERSION {
            return Err(CampaignError::InvalidContract(
                "unknown map or rules version".to_string(),
            ));
        }
        if self.party.len() != 4 {
            return Err(CampaignError::InvalidContract(
                "First Contact requires exactly four party units".to_string(),
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
    pub loot_delta: Vec<LootStack>,
    pub injury_delta_by_unit: BTreeMap<String, u8>,
    pub duplicate: bool,
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
            loot_delta: Vec::new(),
            injury_delta_by_unit: BTreeMap::new(),
            duplicate: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CampaignSaveV1 {
    pub contract_version: String,
    pub campaign_id: String,
    pub revision: u64,
    pub room: CampaignRoom,
    pub phase: CampaignPhase,
    pub character: WorldTrillionniumCharacter,
    pub progression: CampaignProgression,
    pub party: Vec<PartyMember>,
    pub active_party_ids: Vec<String>,
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
        character.equipment_slots.remove("weapon");
        if let Some(staff) = character
            .inventory_items
            .iter_mut()
            .find(|item| item.item_id == "route-guard-staff")
        {
            staff.equipped_slot = None;
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
        let base = TrillionniumAttributes::default();
        let mut scout = base.clone();
        scout.agility += 4;
        scout.insight += 2;
        let mut warden = base.clone();
        warden.physique += 5;
        warden.resolve += 4;
        let mut striker = base.clone();
        striker.force += 5;
        striker.agility += 2;
        Self {
            contract_version: CAMPAIGN_SAVE_CONTRACT.to_string(),
            campaign_id: "local-campaign".to_string(),
            revision: 0,
            room: CampaignRoom::MirrorSquare,
            phase: CampaignPhase::Town,
            character,
            progression: CampaignProgression {
                level: 1,
                experience: 0,
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
                    injury_level: 0,
                    available: true,
                },
                PartyMember {
                    unit_id: "aya".to_string(),
                    display_name: "Aya".to_string(),
                    role: "scout".to_string(),
                    attributes: scout,
                    skill_ids: vec!["basic_lightness".to_string(), "route_scouting".to_string()],
                    persistent: true,
                    injury_level: 0,
                    available: true,
                },
                PartyMember {
                    unit_id: "mako".to_string(),
                    display_name: "Mako".to_string(),
                    role: "warden".to_string(),
                    attributes: warden,
                    skill_ids: vec!["basic_unarmed".to_string()],
                    persistent: true,
                    injury_level: 0,
                    available: true,
                },
                PartyMember {
                    unit_id: "tess".to_string(),
                    display_name: "Tess".to_string(),
                    role: "striker".to_string(),
                    attributes: striker,
                    skill_ids: vec!["basic_blade".to_string()],
                    persistent: true,
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
    pub fn validate(&self) -> Result<(), CampaignError> {
        if self.contract_version != CAMPAIGN_SAVE_CONTRACT {
            return Err(CampaignError::InvalidContract(
                self.contract_version.clone(),
            ));
        }
        if self.active_party_ids.len() != 4 {
            return Err(CampaignError::InvalidState(
                "exactly four active party members are required".to_string(),
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
        if party_ids.len() != self.party.len()
            || active.len() != self.active_party_ids.len()
            || !active.is_subset(&party_ids)
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

    pub fn move_to(&mut self, room: CampaignRoom) -> Result<(), CampaignError> {
        self.require_town()?;
        self.room = room;
        self.revision += 1;
        Ok(())
    }

    pub fn talk_to_mentor(&mut self) -> Result<(), CampaignError> {
        self.require_room(CampaignRoom::MentorHall)?;
        self.mentor_met = true;
        if self.quest_state == QuestState::Locked {
            self.quest_state = QuestState::Available;
        }
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
        self.trained_with_mentor = true;
        if !self
            .character
            .skill_ids
            .iter()
            .any(|skill| skill == "basic_unarmed")
        {
            self.character.skill_ids.push("basic_unarmed".to_string());
        }
        let progress = self
            .progression
            .skill_progress
            .entry("basic_unarmed".to_string())
            .or_insert(SkillProgress {
                rank: 0,
                experience: 0,
            });
        progress.rank = progress.rank.max(1);
        progress.experience += 25;
        self.revision += 1;
        Ok(())
    }

    pub fn equip_starter_weapon(&mut self) -> Result<(), CampaignError> {
        self.require_town()?;
        self.character
            .equip_item_by_id("route-guard-staff", self.revision as i64 + 1)
            .ok_or_else(|| {
                CampaignError::InvalidState("starter weapon is missing from inventory".to_string())
            })?;
        self.revision += 1;
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
        if !matches!(
            self.quest_state,
            QuestState::Available | QuestState::Failed | QuestState::Withdrawn
        ) {
            return Err(CampaignError::InvalidState(
                "First Contact quest is not available".to_string(),
            ));
        }
        self.quest_state = QuestState::Accepted;
        self.revision += 1;
        Ok(())
    }

    pub fn start_first_contact_battle(&mut self) -> Result<BattleSeedV1, CampaignError> {
        self.require_room(CampaignRoom::ExpeditionGate)?;
        if self.quest_state != QuestState::Accepted {
            return Err(CampaignError::InvalidState(
                "accept the First Contact quest before deployment".to_string(),
            ));
        }
        let next_revision = self.revision + 1;
        let battle_id = format!("first-contact-{next_revision:08}");
        let equipment_ids = equipped_item_ids(&self.character);
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
                BattleUnitSeedV1 {
                    unit_id: member.unit_id.clone(),
                    display_name: member.display_name.clone(),
                    role: member.role.clone(),
                    spawn_slot: format!("party_{index}"),
                    persistent: member.persistent,
                    injury_level: member.injury_level,
                    skill_ids: skills,
                    equipment_ids: member_equipment.clone(),
                    stats: map_rpg_to_rts_stats(
                        &member.attributes,
                        skill_rank,
                        &member_equipment,
                        member.injury_level,
                    ),
                }
            })
            .collect();
        let mut seed = BattleSeedV1 {
            contract_version: BATTLE_SEED_CONTRACT.to_string(),
            battle_id,
            campaign_revision: next_revision,
            map_id: "first_contact".to_string(),
            rules_version: FIRST_CONTACT_RULES_VERSION.to_string(),
            party,
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
        let pending = self.pending_battle.as_ref().ok_or_else(|| {
            CampaignError::InvalidState("pending settlement payload is missing".to_string())
        })?;
        let result = pending.result.as_ref().ok_or_else(|| {
            CampaignError::InvalidState("pending settlement result is missing".to_string())
        })?;
        if self.settled_battle_ids.contains(&result.battle_id) {
            let existing = self.receipt_for(&result.battle_id).ok_or_else(|| {
                CampaignError::Integrity("settled battle is missing its receipt".to_string())
            })?;
            return Ok(SettlementReceiptV1::duplicate_from(existing, self.revision));
        }
        result.validate_against(&pending.seed)?;
        let revision_before = self.revision;
        let experience_delta = result
            .units
            .iter()
            .map(|unit| unit.experience_gained)
            .sum::<u64>();
        self.progression.experience += experience_delta;
        self.progression.level = 1 + (self.progression.experience / 500) as u32;
        self.character.attributes.reputation = self
            .character
            .attributes
            .reputation
            .saturating_add(result.reputation_delta);
        merge_loot(&mut self.progression.inventory, &result.loot);
        self.progression
            .world_flags
            .extend(result.world_flags.iter().cloned());

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
                member.injury_level = member.injury_level.saturating_add(delta).min(4);
                if report.status == UnitBattleStatus::Lost && !member.persistent {
                    member.available = false;
                }
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
        self.quest_state = match result.outcome {
            BattleOutcome::Victory => QuestState::Completed,
            BattleOutcome::Defeat => QuestState::Failed,
            BattleOutcome::Withdrawal => QuestState::Withdrawn,
        };
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
            loot_delta: result.loot.clone(),
            injury_delta_by_unit,
            duplicate: false,
        };
        self.settled_battle_ids.insert(result.battle_id.clone());
        self.settlement_receipts.push(receipt.clone());
        self.pending_battle = None;
        self.validate()?;
        Ok(receipt)
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
        _ => {}
    }
    modifier
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

fn canonical_json_hash<T: Serialize>(value: &T) -> Result<String, CampaignError> {
    let bytes = serde_json::to_vec(value)?;
    let digest = Sha256::digest(bytes);
    Ok(digest.iter().map(|byte| format!("{byte:02x}")).collect())
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
        let save: CampaignSaveV1 = serde_json::from_slice(&bytes)?;
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
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let temp_path = self.path.with_extension("json.tmp");
        let payload = serde_json::to_vec_pretty(save)?;
        let mut file = fs::File::create(&temp_path)?;
        file.write_all(&payload)?;
        file.sync_all()?;
        fs::rename(&temp_path, &self.path)?;
        if let Some(parent) = self.path.parent() {
            fs::File::open(parent)?.sync_all()?;
        }
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

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
        campaign.move_to(CampaignRoom::ExpeditionGate).unwrap();
        assert!(campaign.accept_first_contact_quest().is_err());
        let mut campaign = ready_campaign();
        let seed = campaign.start_first_contact_battle().unwrap();
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
        let mut seed = campaign.start_first_contact_battle().unwrap();
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
        let seed = campaign.start_first_contact_battle().unwrap();
        let result = terminal_result(&seed, BattleOutcome::Victory);
        campaign.stage_battle_result(result.clone()).unwrap();
        assert_eq!(campaign.phase, CampaignPhase::PostBattlePending);
        assert_eq!(campaign.progression.experience, 0);
        let receipt = campaign.apply_pending_settlement().unwrap();
        assert!(!receipt.duplicate);
        assert_eq!(receipt.experience_delta, 120);
        assert_eq!(campaign.quest_state, QuestState::Completed);
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
        let seed = campaign.start_first_contact_battle().unwrap();
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
}
