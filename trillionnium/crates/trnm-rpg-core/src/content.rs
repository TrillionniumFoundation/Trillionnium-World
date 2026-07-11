use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SectId {
    StreetCompass,
    IronWorkshop,
    NightWatch,
}

impl SectId {
    pub fn id(self) -> &'static str {
        match self {
            Self::StreetCompass => "street_compass_society",
            Self::IronWorkshop => "iron_workshop_gate",
            Self::NightWatch => "night_watch_alliance",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SectDefinition {
    pub id: SectId,
    pub display_name: &'static str,
    pub hall_room_id: &'static str,
    pub mentor_id: &'static str,
    pub entry_skill_id: &'static str,
    pub battle_bonus: SectBattleBonus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SectBattleBonus {
    ReconAndMobility,
    ConstructionAndArmor,
    AmbushAndEvasion,
}

pub const SECT_CATALOG: [SectDefinition; 3] = [
    SectDefinition {
        id: SectId::StreetCompass,
        display_name: "Street Compass Society",
        hall_room_id: "mentor_hall",
        mentor_id: "street-compass-sifu",
        entry_skill_id: "route_scouting",
        battle_bonus: SectBattleBonus::ReconAndMobility,
    },
    SectDefinition {
        id: SectId::IronWorkshop,
        display_name: "Iron Workshop Gate",
        hall_room_id: "workshop_gate",
        mentor_id: "master-orsen",
        entry_skill_id: "artifact_crafting",
        battle_bonus: SectBattleBonus::ConstructionAndArmor,
    },
    SectDefinition {
        id: SectId::NightWatch,
        display_name: "Night Watch Alliance",
        hall_room_id: "night_watch_post",
        mentor_id: "captain-veyra",
        entry_skill_id: "shadow_patrol",
        battle_bonus: SectBattleBonus::AmbushAndEvasion,
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NpcRole {
    Mentor,
    Smith,
    WatchCaptain,
    Healer,
    Archivist,
    Merchant,
    Courier,
    Scout,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NpcDefinition {
    pub id: &'static str,
    pub display_name: &'static str,
    pub role: NpcRole,
    pub room_id: &'static str,
    pub faction_id: &'static str,
    pub task_ids: &'static [&'static str],
}

pub const NPC_CATALOG: [NpcDefinition; 10] = [
    NpcDefinition {
        id: "street-compass-sifu",
        display_name: "Street Compass Sifu",
        role: NpcRole::Mentor,
        room_id: "mentor_hall",
        faction_id: "street_compass_society",
        task_ids: &["wayfinder_oath", "broken_milestone"],
    },
    NpcDefinition {
        id: "master-orsen",
        display_name: "Master Orsen",
        role: NpcRole::Mentor,
        room_id: "workshop_gate",
        faction_id: "iron_workshop_gate",
        task_ids: &["forge_commission", "lost_tooling"],
    },
    NpcDefinition {
        id: "captain-veyra",
        display_name: "Captain Veyra",
        role: NpcRole::Mentor,
        room_id: "night_watch_post",
        faction_id: "night_watch_alliance",
        task_ids: &["lantern_watch", "wanted_raider"],
    },
    NpcDefinition {
        id: "relay-smith-brann",
        display_name: "Brann",
        role: NpcRole::Smith,
        room_id: "relay_quarter",
        faction_id: "relay_quarter",
        task_ids: &["relay_salvage"],
    },
    NpcDefinition {
        id: "healer-nima",
        display_name: "Healer Nima",
        role: NpcRole::Healer,
        room_id: "lantern_infirmary",
        faction_id: "mirror_civic",
        task_ids: &["fever_tonic"],
    },
    NpcDefinition {
        id: "archivist-sol",
        display_name: "Archivist Sol",
        role: NpcRole::Archivist,
        room_id: "archive_steps",
        faction_id: "mirror_civic",
        task_ids: &["archive_witness"],
    },
    NpcDefinition {
        id: "merchant-aya",
        display_name: "Merchant Aya",
        role: NpcRole::Merchant,
        room_id: "market_wind_pavilion",
        faction_id: "market_wind",
        task_ids: &["market_debt", "missing_crate"],
    },
    NpcDefinition {
        id: "courier-tess",
        display_name: "Courier Tess",
        role: NpcRole::Courier,
        room_id: "caravan_yard",
        faction_id: "street_compass_society",
        task_ids: &["night_letter", "escort_manifest"],
    },
    NpcDefinition {
        id: "scout-mako",
        display_name: "Scout Mako",
        role: NpcRole::Scout,
        room_id: "outer_signal_road",
        faction_id: "night_watch_alliance",
        task_ids: &["bandit_tracks"],
    },
    NpcDefinition {
        id: "quartermaster-nia",
        display_name: "Quartermaster Nia",
        role: NpcRole::WatchCaptain,
        room_id: "cistern_ward",
        faction_id: "mirror_civic",
        task_ids: &["cistern_relief", "ration_audit"],
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestArchetype {
    Courier,
    FindItem,
    Escort,
    Hunt,
    Investigation,
    Supply,
    TrainingTrial,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RegionalQuestDefinition {
    pub id: &'static str,
    pub title: &'static str,
    pub archetype: QuestArchetype,
    pub giver_npc_id: &'static str,
    pub waypoint_room_ids: &'static [&'static str],
    pub encounter_id: Option<&'static str>,
    pub credit_reward: i64,
    pub reputation_reward: i32,
}

pub const REGIONAL_QUEST_CATALOG: [RegionalQuestDefinition; 15] = [
    RegionalQuestDefinition {
        id: "wayfinder_oath",
        title: "The Wayfinder Oath",
        archetype: QuestArchetype::TrainingTrial,
        giver_npc_id: "street-compass-sifu",
        waypoint_room_ids: &["mentor_hall", "archive_steps", "expedition_gate"],
        encounter_id: Some("milestone_duel"),
        credit_reward: 35,
        reputation_reward: 3,
    },
    RegionalQuestDefinition {
        id: "broken_milestone",
        title: "The Broken Milestone",
        archetype: QuestArchetype::Investigation,
        giver_npc_id: "street-compass-sifu",
        waypoint_room_ids: &["mirror_square", "outer_signal_road"],
        encounter_id: Some("roadside_ambush"),
        credit_reward: 45,
        reputation_reward: 4,
    },
    RegionalQuestDefinition {
        id: "forge_commission",
        title: "Forge Commission",
        archetype: QuestArchetype::FindItem,
        giver_npc_id: "master-orsen",
        waypoint_room_ids: &["workshop_gate", "relay_quarter"],
        encounter_id: None,
        credit_reward: 40,
        reputation_reward: 3,
    },
    RegionalQuestDefinition {
        id: "lost_tooling",
        title: "Lost Tooling",
        archetype: QuestArchetype::FindItem,
        giver_npc_id: "master-orsen",
        waypoint_room_ids: &["caravan_yard", "outer_signal_road"],
        encounter_id: Some("scrap_stalker"),
        credit_reward: 55,
        reputation_reward: 5,
    },
    RegionalQuestDefinition {
        id: "lantern_watch",
        title: "Lantern Watch",
        archetype: QuestArchetype::Escort,
        giver_npc_id: "captain-veyra",
        waypoint_room_ids: &["night_watch_post", "market_wind_pavilion", "cistern_ward"],
        encounter_id: Some("night_raiders"),
        credit_reward: 55,
        reputation_reward: 5,
    },
    RegionalQuestDefinition {
        id: "wanted_raider",
        title: "Wanted: Ash Runner",
        archetype: QuestArchetype::Hunt,
        giver_npc_id: "captain-veyra",
        waypoint_room_ids: &["outer_signal_road"],
        encounter_id: Some("ash_runner"),
        credit_reward: 70,
        reputation_reward: 7,
    },
    RegionalQuestDefinition {
        id: "relay_salvage",
        title: "Relay Salvage",
        archetype: QuestArchetype::FindItem,
        giver_npc_id: "relay-smith-brann",
        waypoint_room_ids: &["relay_quarter", "outer_signal_road"],
        encounter_id: Some("scrap_stalker"),
        credit_reward: 50,
        reputation_reward: 4,
    },
    RegionalQuestDefinition {
        id: "fever_tonic",
        title: "Fever Tonic",
        archetype: QuestArchetype::Supply,
        giver_npc_id: "healer-nima",
        waypoint_room_ids: &["market_wind_pavilion", "lantern_infirmary"],
        encounter_id: None,
        credit_reward: 30,
        reputation_reward: 4,
    },
    RegionalQuestDefinition {
        id: "archive_witness",
        title: "Archive Witness",
        archetype: QuestArchetype::Investigation,
        giver_npc_id: "archivist-sol",
        waypoint_room_ids: &["archive_steps", "night_watch_post"],
        encounter_id: None,
        credit_reward: 35,
        reputation_reward: 5,
    },
    RegionalQuestDefinition {
        id: "market_debt",
        title: "Market Debt",
        archetype: QuestArchetype::Courier,
        giver_npc_id: "merchant-aya",
        waypoint_room_ids: &["market_wind_pavilion", "workshop_gate"],
        encounter_id: None,
        credit_reward: 45,
        reputation_reward: 2,
    },
    RegionalQuestDefinition {
        id: "missing_crate",
        title: "The Missing Crate",
        archetype: QuestArchetype::Investigation,
        giver_npc_id: "merchant-aya",
        waypoint_room_ids: &["caravan_yard", "cistern_ward"],
        encounter_id: Some("dock_thieves"),
        credit_reward: 50,
        reputation_reward: 4,
    },
    RegionalQuestDefinition {
        id: "night_letter",
        title: "Letter After Curfew",
        archetype: QuestArchetype::Courier,
        giver_npc_id: "courier-tess",
        waypoint_room_ids: &["caravan_yard", "night_watch_post"],
        encounter_id: Some("night_raiders"),
        credit_reward: 40,
        reputation_reward: 3,
    },
    RegionalQuestDefinition {
        id: "escort_manifest",
        title: "Escort Manifest",
        archetype: QuestArchetype::Escort,
        giver_npc_id: "courier-tess",
        waypoint_room_ids: &["caravan_yard", "expedition_gate"],
        encounter_id: None,
        credit_reward: 50,
        reputation_reward: 4,
    },
    RegionalQuestDefinition {
        id: "bandit_tracks",
        title: "Tracks Beyond the Gate",
        archetype: QuestArchetype::Hunt,
        giver_npc_id: "scout-mako",
        waypoint_room_ids: &["outer_signal_road"],
        encounter_id: Some("roadside_ambush"),
        credit_reward: 65,
        reputation_reward: 6,
    },
    RegionalQuestDefinition {
        id: "ration_audit",
        title: "Ration Audit",
        archetype: QuestArchetype::Supply,
        giver_npc_id: "quartermaster-nia",
        waypoint_room_ids: &["cistern_ward", "caravan_yard", "archive_steps"],
        encounter_id: None,
        credit_reward: 35,
        reputation_reward: 4,
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillEffect {
    Damage,
    Guard,
    Mobility,
    Recon,
    Healing,
    Construction,
    Economy,
    Diplomacy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SkillDefinition {
    pub id: &'static str,
    pub display_name: &'static str,
    pub sect: Option<SectId>,
    pub prerequisite: Option<&'static str>,
    pub effect: SkillEffect,
    pub rts_modifier_permille: u16,
}

pub const SKILL_CATALOG: [SkillDefinition; 15] = [
    SkillDefinition {
        id: "basic_inner_power",
        display_name: "Basic Inner Power",
        sect: None,
        prerequisite: None,
        effect: SkillEffect::Guard,
        rts_modifier_permille: 40,
    },
    SkillDefinition {
        id: "basic_unarmed",
        display_name: "Basic Unarmed",
        sect: None,
        prerequisite: None,
        effect: SkillEffect::Damage,
        rts_modifier_permille: 30,
    },
    SkillDefinition {
        id: "basic_blade",
        display_name: "Basic Blade",
        sect: None,
        prerequisite: None,
        effect: SkillEffect::Damage,
        rts_modifier_permille: 35,
    },
    SkillDefinition {
        id: "basic_lightness",
        display_name: "Basic Lightness",
        sect: None,
        prerequisite: None,
        effect: SkillEffect::Mobility,
        rts_modifier_permille: 35,
    },
    SkillDefinition {
        id: "reading_and_contracts",
        display_name: "Reading and Contracts",
        sect: None,
        prerequisite: None,
        effect: SkillEffect::Diplomacy,
        rts_modifier_permille: 20,
    },
    SkillDefinition {
        id: "route_scouting",
        display_name: "Route Scouting",
        sect: Some(SectId::StreetCompass),
        prerequisite: Some("basic_lightness"),
        effect: SkillEffect::Recon,
        rts_modifier_permille: 70,
    },
    SkillDefinition {
        id: "wind_step",
        display_name: "Wind Step",
        sect: Some(SectId::StreetCompass),
        prerequisite: Some("route_scouting"),
        effect: SkillEffect::Mobility,
        rts_modifier_permille: 100,
    },
    SkillDefinition {
        id: "compass_feint",
        display_name: "Compass Feint",
        sect: Some(SectId::StreetCompass),
        prerequisite: Some("wind_step"),
        effect: SkillEffect::Damage,
        rts_modifier_permille: 90,
    },
    SkillDefinition {
        id: "artifact_crafting",
        display_name: "Artifact Crafting",
        sect: Some(SectId::IronWorkshop),
        prerequisite: Some("reading_and_contracts"),
        effect: SkillEffect::Construction,
        rts_modifier_permille: 80,
    },
    SkillDefinition {
        id: "iron_guard",
        display_name: "Iron Guard",
        sect: Some(SectId::IronWorkshop),
        prerequisite: Some("artifact_crafting"),
        effect: SkillEffect::Guard,
        rts_modifier_permille: 110,
    },
    SkillDefinition {
        id: "relay_overcharge",
        display_name: "Relay Overcharge",
        sect: Some(SectId::IronWorkshop),
        prerequisite: Some("iron_guard"),
        effect: SkillEffect::Construction,
        rts_modifier_permille: 120,
    },
    SkillDefinition {
        id: "shadow_patrol",
        display_name: "Shadow Patrol",
        sect: Some(SectId::NightWatch),
        prerequisite: Some("basic_lightness"),
        effect: SkillEffect::Recon,
        rts_modifier_permille: 75,
    },
    SkillDefinition {
        id: "night_veil",
        display_name: "Night Veil",
        sect: Some(SectId::NightWatch),
        prerequisite: Some("shadow_patrol"),
        effect: SkillEffect::Mobility,
        rts_modifier_permille: 95,
    },
    SkillDefinition {
        id: "inner_flame",
        display_name: "Inner Flame",
        sect: Some(SectId::NightWatch),
        prerequisite: Some("night_veil"),
        effect: SkillEffect::Damage,
        rts_modifier_permille: 115,
    },
    SkillDefinition {
        id: "field_mend",
        display_name: "Field Mend",
        sect: None,
        prerequisite: Some("basic_inner_power"),
        effect: SkillEffect::Healing,
        rts_modifier_permille: 90,
    },
];

pub fn skill_unlockable(skill_id: &str, known: &BTreeSet<String>, sect: Option<SectId>) -> bool {
    SKILL_CATALOG
        .iter()
        .find(|skill| skill.id == skill_id)
        .is_some_and(|skill| {
            skill.sect.is_none_or(|required| Some(required) == sect)
                && skill
                    .prerequisite
                    .is_none_or(|required| known.contains(required))
        })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EncounterKind {
    Duel,
    Ambush,
    Hunt,
    Defense,
    Investigation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EncounterDefinition {
    pub id: &'static str,
    pub kind: EncounterKind,
    pub enemy_name: &'static str,
    pub enemy_hp: i64,
    pub loot_table: &'static [&'static str],
}

pub const ENCOUNTER_CATALOG: [EncounterDefinition; 7] = [
    EncounterDefinition {
        id: "milestone_duel",
        kind: EncounterKind::Duel,
        enemy_name: "Milestone Challenger",
        enemy_hp: 120,
        loot_table: &["route-token"],
    },
    EncounterDefinition {
        id: "roadside_ambush",
        kind: EncounterKind::Ambush,
        enemy_name: "Roadside Cutters",
        enemy_hp: 145,
        loot_table: &["salvaged-alloy", "field-tonic-kit"],
    },
    EncounterDefinition {
        id: "scrap_stalker",
        kind: EncounterKind::Hunt,
        enemy_name: "Scrap Stalker",
        enemy_hp: 170,
        loot_table: &["salvaged-alloy", "relay-core-fragment"],
    },
    EncounterDefinition {
        id: "night_raiders",
        kind: EncounterKind::Defense,
        enemy_name: "Night Raiders",
        enemy_hp: 185,
        loot_table: &["watch-cloth", "field-tonic-kit"],
    },
    EncounterDefinition {
        id: "ash_runner",
        kind: EncounterKind::Hunt,
        enemy_name: "Ash Runner",
        enemy_hp: 210,
        loot_table: &["ash-runner-seal", "watch-cloth"],
    },
    EncounterDefinition {
        id: "dock_thieves",
        kind: EncounterKind::Investigation,
        enemy_name: "Dock Thieves",
        enemy_hp: 135,
        loot_table: &["market-ledger", "salvaged-alloy"],
    },
    EncounterDefinition {
        id: "signal_road_ambush",
        kind: EncounterKind::Ambush,
        enemy_name: "Signal Road Ambushers",
        enemy_hp: 135,
        loot_table: &["signal-road-emblem"],
    },
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatLogBeat {
    pub kind: String,
    pub text: String,
}

pub fn original_combat_log(encounter_id: &str, round: u8, player_won: bool) -> Vec<CombatLogBeat> {
    let enemy = ENCOUNTER_CATALOG
        .iter()
        .find(|entry| entry.id == encounter_id)
        .map(|entry| entry.enemy_name)
        .unwrap_or("Unknown Rival");
    vec![
        CombatLogBeat {
            kind: "stance".to_string(),
            text: format!("Lantern light bends as you square your stance against {enemy}."),
        },
        CombatLogBeat {
            kind: "exchange".to_string(),
            text: format!("Steel, breath and footwork trade measure for {round} rounds."),
        },
        CombatLogBeat {
            kind: "outcome".to_string(),
            text: if player_won {
                format!("{enemy} yields the road; your party keeps the initiative.")
            } else {
                format!("{enemy} holds the ground, forcing your party to regroup.")
            },
        },
    ]
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EconomyItemDefinition {
    pub id: &'static str,
    pub display_name: &'static str,
    pub buy_price: i64,
    pub max_durability: u16,
    pub material: bool,
}

pub const ECONOMY_ITEM_CATALOG: [EconomyItemDefinition; 18] = [
    EconomyItemDefinition {
        id: "route-guard-staff",
        display_name: "Route Guard Staff",
        buy_price: 70,
        max_durability: 80,
        material: false,
    },
    EconomyItemDefinition {
        id: "street-compass-bracer",
        display_name: "Street Compass Bracer",
        buy_price: 55,
        max_durability: 70,
        material: false,
    },
    EconomyItemDefinition {
        id: "iron-workshop-blade",
        display_name: "Iron Workshop Blade",
        buy_price: 85,
        max_durability: 100,
        material: false,
    },
    EconomyItemDefinition {
        id: "market-wind-sword",
        display_name: "Market Wind Sword",
        buy_price: 90,
        max_durability: 85,
        material: false,
    },
    EconomyItemDefinition {
        id: "night-watch-cloak",
        display_name: "Night Watch Cloak",
        buy_price: 75,
        max_durability: 65,
        material: false,
    },
    EconomyItemDefinition {
        id: "raid-signal-drum",
        display_name: "Raid Signal Drum",
        buy_price: 80,
        max_durability: 75,
        material: false,
    },
    EconomyItemDefinition {
        id: "field-tonic-kit",
        display_name: "Field Tonic Kit",
        buy_price: 25,
        max_durability: 1,
        material: false,
    },
    EconomyItemDefinition {
        id: "relay-core-fragment",
        display_name: "Relay Core Fragment",
        buy_price: 120,
        max_durability: 120,
        material: false,
    },
    EconomyItemDefinition {
        id: "evidence-wrap-case",
        display_name: "Archive Wrap Case",
        buy_price: 35,
        max_durability: 60,
        material: false,
    },
    EconomyItemDefinition {
        id: "salvaged-alloy",
        display_name: "Salvaged Alloy",
        buy_price: 12,
        max_durability: 0,
        material: true,
    },
    EconomyItemDefinition {
        id: "watch-cloth",
        display_name: "Night Watch Cloth",
        buy_price: 10,
        max_durability: 0,
        material: true,
    },
    EconomyItemDefinition {
        id: "route-token",
        display_name: "Wayfinder Route Token",
        buy_price: 15,
        max_durability: 0,
        material: true,
    },
    EconomyItemDefinition {
        id: "ash-runner-seal",
        display_name: "Ash Runner Seal",
        buy_price: 45,
        max_durability: 0,
        material: true,
    },
    EconomyItemDefinition {
        id: "market-ledger",
        display_name: "Recovered Market Ledger",
        buy_price: 30,
        max_durability: 0,
        material: true,
    },
    EconomyItemDefinition {
        id: "reinforced-staff",
        display_name: "Reinforced Route Staff",
        buy_price: 130,
        max_durability: 140,
        material: false,
    },
    EconomyItemDefinition {
        id: "signal-lamellar",
        display_name: "Signal Lamellar",
        buy_price: 150,
        max_durability: 160,
        material: false,
    },
    EconomyItemDefinition {
        id: "watcher-boots",
        display_name: "Watcher Boots",
        buy_price: 115,
        max_durability: 100,
        material: false,
    },
    EconomyItemDefinition {
        id: "field-medic-satchel",
        display_name: "Field Medic Satchel",
        buy_price: 100,
        max_durability: 90,
        material: false,
    },
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CraftingRecipe {
    pub id: &'static str,
    pub output_item_id: &'static str,
    pub ingredients: &'static [(&'static str, u16)],
    pub required_skill_id: &'static str,
}

pub const CRAFTING_RECIPES: [CraftingRecipe; 4] = [
    CraftingRecipe {
        id: "reinforced_staff",
        output_item_id: "reinforced-staff",
        ingredients: &[("salvaged-alloy", 3), ("route-token", 1)],
        required_skill_id: "artifact_crafting",
    },
    CraftingRecipe {
        id: "signal_lamellar",
        output_item_id: "signal-lamellar",
        ingredients: &[("salvaged-alloy", 5), ("watch-cloth", 2)],
        required_skill_id: "iron_guard",
    },
    CraftingRecipe {
        id: "watcher_boots",
        output_item_id: "watcher-boots",
        ingredients: &[("watch-cloth", 4), ("route-token", 1)],
        required_skill_id: "shadow_patrol",
    },
    CraftingRecipe {
        id: "medic_satchel",
        output_item_id: "field-medic-satchel",
        ingredients: &[("watch-cloth", 2), ("salvaged-alloy", 2)],
        required_skill_id: "field_mend",
    },
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemCondition {
    pub item_id: String,
    pub durability: u16,
    pub max_durability: u16,
}

impl ItemCondition {
    pub fn new(item_id: impl Into<String>) -> Option<Self> {
        let item_id = item_id.into();
        let definition = ECONOMY_ITEM_CATALOG
            .iter()
            .find(|entry| entry.id == item_id)?;
        Some(Self {
            item_id,
            durability: definition.max_durability,
            max_durability: definition.max_durability,
        })
    }

    pub fn apply_wear(&mut self, amount: u16) {
        self.durability = self.durability.saturating_sub(amount);
    }
    pub fn repair_cost(&self) -> i64 {
        let missing = i64::from(self.max_durability.saturating_sub(self.durability));
        (missing + 3) / 4
    }
    pub fn repair(&mut self) {
        self.durability = self.max_durability;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authored_region_has_three_sects_ten_npcs_fifteen_quests_and_seven_encounters() {
        assert_eq!(SECT_CATALOG.len(), 3);
        assert_eq!(NPC_CATALOG.len(), 10);
        assert_eq!(REGIONAL_QUEST_CATALOG.len(), 15);
        assert_eq!(ENCOUNTER_CATALOG.len(), 7);
        assert!(REGIONAL_QUEST_CATALOG
            .iter()
            .all(|quest| NPC_CATALOG.iter().any(|npc| npc.id == quest.giver_npc_id)));
    }

    #[test]
    fn skill_tree_requires_prerequisites_and_matching_sect() {
        let mut known = BTreeSet::from(["basic_lightness".to_string()]);
        assert!(skill_unlockable(
            "route_scouting",
            &known,
            Some(SectId::StreetCompass)
        ));
        assert!(!skill_unlockable(
            "route_scouting",
            &known,
            Some(SectId::NightWatch)
        ));
        known.insert("route_scouting".to_string());
        assert!(skill_unlockable(
            "wind_step",
            &known,
            Some(SectId::StreetCompass)
        ));
    }

    #[test]
    fn combat_logs_are_original_deterministic_and_encounter_bound() {
        let first = original_combat_log("ash_runner", 4, true);
        assert_eq!(first, original_combat_log("ash_runner", 4, true));
        assert!(first.iter().any(|beat| beat.text.contains("Ash Runner")));
    }

    #[test]
    fn item_economy_has_shop_prices_recipes_durability_and_repair() {
        assert_eq!(ECONOMY_ITEM_CATALOG.len(), 18);
        assert_eq!(CRAFTING_RECIPES.len(), 4);
        let mut item = ItemCondition::new("iron-workshop-blade").unwrap();
        item.apply_wear(17);
        assert!(item.repair_cost() > 0);
        item.repair();
        assert_eq!(item.durability, item.max_durability);
    }
}
