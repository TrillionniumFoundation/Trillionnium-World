use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RtsFaction {
    MirrorCoalition,
    AshenCompact,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UnitArchetype {
    pub id: &'static str,
    pub faction: RtsFaction,
    pub visual_family: &'static str,
    pub role: &'static str,
    pub cost: u32,
    pub hp: u32,
    pub damage: u32,
    pub supply: u8,
}

pub const UNIT_ROSTER: [UnitArchetype; 12] = [
    UnitArchetype {
        id: "mirror_wayfinder",
        faction: RtsFaction::MirrorCoalition,
        visual_family: "scout",
        role: "recon",
        cost: 45,
        hp: 110,
        damage: 18,
        supply: 1,
    },
    UnitArchetype {
        id: "mirror_warden",
        faction: RtsFaction::MirrorCoalition,
        visual_family: "warden",
        role: "frontline",
        cost: 65,
        hp: 210,
        damage: 16,
        supply: 2,
    },
    UnitArchetype {
        id: "mirror_striker",
        faction: RtsFaction::MirrorCoalition,
        visual_family: "striker",
        role: "assault",
        cost: 70,
        hp: 145,
        damage: 28,
        supply: 2,
    },
    UnitArchetype {
        id: "relay_engineer",
        faction: RtsFaction::MirrorCoalition,
        visual_family: "relay_engineer",
        role: "builder",
        cost: 50,
        hp: 130,
        damage: 12,
        supply: 1,
    },
    UnitArchetype {
        id: "field_medic",
        faction: RtsFaction::MirrorCoalition,
        visual_family: "worker",
        role: "support",
        cost: 55,
        hp: 105,
        damage: 8,
        supply: 1,
    },
    UnitArchetype {
        id: "mirror_sentinel",
        faction: RtsFaction::MirrorCoalition,
        visual_family: "sentinel",
        role: "heavy",
        cost: 90,
        hp: 260,
        damage: 30,
        supply: 3,
    },
    UnitArchetype {
        id: "ash_runner",
        faction: RtsFaction::AshenCompact,
        visual_family: "scout",
        role: "raider",
        cost: 45,
        hp: 100,
        damage: 20,
        supply: 1,
    },
    UnitArchetype {
        id: "ash_bulwark",
        faction: RtsFaction::AshenCompact,
        visual_family: "warden",
        role: "frontline",
        cost: 65,
        hp: 225,
        damage: 15,
        supply: 2,
    },
    UnitArchetype {
        id: "ash_lancer",
        faction: RtsFaction::AshenCompact,
        visual_family: "striker",
        role: "assault",
        cost: 70,
        hp: 135,
        damage: 31,
        supply: 2,
    },
    UnitArchetype {
        id: "ash_sapper",
        faction: RtsFaction::AshenCompact,
        visual_family: "relay_engineer",
        role: "siege",
        cost: 60,
        hp: 120,
        damage: 24,
        supply: 2,
    },
    UnitArchetype {
        id: "ash_whisper",
        faction: RtsFaction::AshenCompact,
        visual_family: "worker",
        role: "disruptor",
        cost: 60,
        hp: 95,
        damage: 16,
        supply: 1,
    },
    UnitArchetype {
        id: "ash_commander",
        faction: RtsFaction::AshenCompact,
        visual_family: "sentinel",
        role: "heavy",
        cost: 95,
        hp: 245,
        damage: 34,
        supply: 3,
    },
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StructureArchetype {
    pub id: &'static str,
    pub faction: Option<RtsFaction>,
    pub cost: u32,
    pub hp: u32,
    pub power_delta: i16,
    pub supply_delta: i8,
}

pub const STRUCTURE_ROSTER: [StructureArchetype; 10] = [
    StructureArchetype {
        id: "command_post",
        faction: None,
        cost: 0,
        hp: 1200,
        power_delta: 50,
        supply_delta: 8,
    },
    StructureArchetype {
        id: "field_workshop",
        faction: None,
        cost: 45,
        hp: 600,
        power_delta: -30,
        supply_delta: 0,
    },
    StructureArchetype {
        id: "relay_generator",
        faction: None,
        cost: 35,
        hp: 520,
        power_delta: 50,
        supply_delta: 0,
    },
    StructureArchetype {
        id: "supply_cache",
        faction: None,
        cost: 25,
        hp: 420,
        power_delta: 0,
        supply_delta: 4,
    },
    StructureArchetype {
        id: "field_barricade",
        faction: None,
        cost: 30,
        hp: 480,
        power_delta: -4,
        supply_delta: 0,
    },
    StructureArchetype {
        id: "sensor_tower",
        faction: Some(RtsFaction::MirrorCoalition),
        cost: 40,
        hp: 430,
        power_delta: -12,
        supply_delta: 0,
    },
    StructureArchetype {
        id: "field_hospital",
        faction: Some(RtsFaction::MirrorCoalition),
        cost: 55,
        hp: 500,
        power_delta: -20,
        supply_delta: 0,
    },
    StructureArchetype {
        id: "siege_foundry",
        faction: Some(RtsFaction::AshenCompact),
        cost: 60,
        hp: 650,
        power_delta: -25,
        supply_delta: 0,
    },
    StructureArchetype {
        id: "ash_beacon",
        faction: Some(RtsFaction::AshenCompact),
        cost: 45,
        hp: 500,
        power_delta: 35,
        supply_delta: 0,
    },
    StructureArchetype {
        id: "forward_rally",
        faction: None,
        cost: 35,
        hp: 380,
        power_delta: -5,
        supply_delta: 2,
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TechEffect {
    Economy,
    Vision,
    Damage,
    Armor,
    Healing,
    Siege,
    Mobility,
    Production,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TechDefinition {
    pub id: &'static str,
    pub faction: Option<RtsFaction>,
    pub prerequisite: Option<&'static str>,
    pub cost: u32,
    pub effect: TechEffect,
}

pub const TECH_TREE: [TechDefinition; 10] = [
    TechDefinition {
        id: "field_logistics",
        faction: None,
        prerequisite: None,
        cost: 35,
        effect: TechEffect::Economy,
    },
    TechDefinition {
        id: "signal_optics",
        faction: None,
        prerequisite: Some("field_logistics"),
        cost: 35,
        effect: TechEffect::Vision,
    },
    TechDefinition {
        id: "relay_arms",
        faction: None,
        prerequisite: None,
        cost: 45,
        effect: TechEffect::Damage,
    },
    TechDefinition {
        id: "field_armor",
        faction: None,
        prerequisite: None,
        cost: 45,
        effect: TechEffect::Armor,
    },
    TechDefinition {
        id: "sensor_net",
        faction: Some(RtsFaction::MirrorCoalition),
        prerequisite: Some("signal_optics"),
        cost: 45,
        effect: TechEffect::Vision,
    },
    TechDefinition {
        id: "field_medicine",
        faction: Some(RtsFaction::MirrorCoalition),
        prerequisite: Some("field_logistics"),
        cost: 50,
        effect: TechEffect::Healing,
    },
    TechDefinition {
        id: "wayfinder_drills",
        faction: Some(RtsFaction::MirrorCoalition),
        prerequisite: None,
        cost: 45,
        effect: TechEffect::Mobility,
    },
    TechDefinition {
        id: "siege_drills",
        faction: Some(RtsFaction::AshenCompact),
        prerequisite: Some("relay_arms"),
        cost: 50,
        effect: TechEffect::Siege,
    },
    TechDefinition {
        id: "reactive_plating",
        faction: Some(RtsFaction::AshenCompact),
        prerequisite: Some("field_armor"),
        cost: 50,
        effect: TechEffect::Armor,
    },
    TechDefinition {
        id: "rapid_mustering",
        faction: Some(RtsFaction::AshenCompact),
        prerequisite: None,
        cost: 45,
        effect: TechEffect::Production,
    },
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkirmishRules {
    pub map_id: String,
    pub player_faction: RtsFaction,
    pub enemy_faction: RtsFaction,
    pub starting_resources: u32,
    pub victory_score: u32,
    pub seed: u64,
}

impl SkirmishRules {
    pub fn validate(&self) -> Result<(), String> {
        if self.player_faction == self.enemy_faction {
            return Err("skirmish factions must differ".to_string());
        }
        if !matches!(self.map_id.as_str(), "iron_delta" | "night_watch_crossing") {
            return Err("unknown authored skirmish map".to_string());
        }
        if !(100..=1000).contains(&self.starting_resources) || self.victory_score < 500 {
            return Err("skirmish economy or score target is invalid".to_string());
        }
        Ok(())
    }
}

pub fn faction_balance_delta_permille() -> u16 {
    let totals = [RtsFaction::MirrorCoalition, RtsFaction::AshenCompact].map(|faction| {
        UNIT_ROSTER
            .iter()
            .filter(|unit| unit.faction == faction)
            .map(|unit| unit.hp + unit.damage * 5 + unit.cost)
            .sum::<u32>()
    });
    let high = totals[0].max(totals[1]);
    let low = totals[0].min(totals[1]);
    ((high - low) * 1000 / high.max(1)) as u16
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn two_factions_have_six_units_each_and_bounded_baseline_delta() {
        for faction in [RtsFaction::MirrorCoalition, RtsFaction::AshenCompact] {
            assert_eq!(
                UNIT_ROSTER
                    .iter()
                    .filter(|unit| unit.faction == faction)
                    .count(),
                6
            );
        }
        assert!(faction_balance_delta_permille() <= 100);
    }

    #[test]
    fn structure_and_tech_catalogs_have_real_prerequisite_breadth() {
        assert_eq!(STRUCTURE_ROSTER.len(), 10);
        assert_eq!(TECH_TREE.len(), 10);
        let ids = TECH_TREE
            .iter()
            .map(|tech| tech.id)
            .collect::<BTreeSet<_>>();
        assert!(TECH_TREE
            .iter()
            .filter_map(|tech| tech.prerequisite)
            .all(|required| ids.contains(required)));
    }

    #[test]
    fn authored_skirmish_rules_reject_mirror_matches_and_unknown_maps() {
        let rules = SkirmishRules {
            map_id: "iron_delta".to_string(),
            player_faction: RtsFaction::MirrorCoalition,
            enemy_faction: RtsFaction::AshenCompact,
            starting_resources: 300,
            victory_score: 1000,
            seed: 7,
        };
        rules.validate().unwrap();
        let mut invalid = rules.clone();
        invalid.enemy_faction = invalid.player_faction;
        assert!(invalid.validate().is_err());
    }
}
