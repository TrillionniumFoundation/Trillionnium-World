//! Minimal RPG state consumed by the playable campaign.
//!
//! Historical CEX/world fixtures intentionally live outside the game product
//! workspace. This crate owns only character attributes and typed equipment.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedStats {
    pub max_hp: i64,
    pub inner_energy: i64,
    pub move_range: u16,
    pub learning_speed: i64,
    pub combat_power_hint: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrillionniumAttributes {
    pub physique: u16,
    pub force: u16,
    pub agility: u16,
    pub insight: u16,
    pub resolve: u16,
    pub craft: u16,
    pub commerce: u16,
    pub reputation: i32,
}

impl Default for TrillionniumAttributes {
    fn default() -> Self {
        Self {
            physique: 12,
            force: 11,
            agility: 12,
            insight: 13,
            resolve: 12,
            craft: 10,
            commerce: 10,
            reputation: 0,
        }
    }
}

impl TrillionniumAttributes {
    pub fn derived_stats(&self) -> DerivedStats {
        DerivedStats {
            max_hp: (80 + self.physique as i64 * 6 + self.resolve as i64 * 2).clamp(80, 260),
            inner_energy: (40 + self.resolve as i64 * 5 + self.insight as i64 * 2).clamp(40, 220),
            move_range: 3 + (self.agility / 8).clamp(0, 3),
            learning_speed: (100 + self.insight as i64 * 4).clamp(100, 220),
            combat_power_hint: (self.force as i64 * 2 + self.agility as i64 + self.resolve as i64)
                .clamp(0, 160),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ItemDefinition {
    id: &'static str,
    slot: &'static str,
    family: &'static str,
    display_name: &'static str,
}

const ITEM_CATALOG: &[ItemDefinition] = &[
    ItemDefinition {
        id: "route-guard-staff",
        slot: "weapon",
        family: "staff",
        display_name: "Route Guard Staff",
    },
    ItemDefinition {
        id: "street-compass-bracer",
        slot: "wrist",
        family: "navigation",
        display_name: "Street Compass Bracer",
    },
    ItemDefinition {
        id: "iron-workshop-blade",
        slot: "weapon",
        family: "blade",
        display_name: "Iron Workshop Blade",
    },
    ItemDefinition {
        id: "market-wind-sword",
        slot: "weapon",
        family: "sword",
        display_name: "Market Wind Sword",
    },
    ItemDefinition {
        id: "night-watch-cloak",
        slot: "cloak",
        family: "lightness",
        display_name: "Night Watch Cloak",
    },
    ItemDefinition {
        id: "raid-signal-drum",
        slot: "party_tool",
        family: "raid_command",
        display_name: "Raid Signal Drum",
    },
    ItemDefinition {
        id: "field-tonic-kit",
        slot: "consumable",
        family: "medicine",
        display_name: "Field Tonic Kit",
    },
    ItemDefinition {
        id: "relay-core-fragment",
        slot: "relic",
        family: "relay_salvage",
        display_name: "Relay Core Fragment",
    },
    ItemDefinition {
        id: "evidence-wrap-case",
        slot: "pack",
        family: "evidence",
        display_name: "Evidence Wrap Case",
    },
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InventoryItem {
    pub item_instance_id: String,
    pub item_id: String,
    pub slot: String,
    pub family: String,
    pub display_name: String,
    pub quantity: u16,
    pub quality: String,
    pub equipped_slot: Option<String>,
    pub acquired_from: String,
    pub acquired_at_epoch: i64,
    pub updated_at_epoch: i64,
}

fn stable_id(prefix: &str, value: &str) -> String {
    let digest = Sha256::digest(value.as_bytes());
    let short = digest[..10]
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!("{prefix}-{short}")
}

pub fn inventory_item_for(
    player_id: &str,
    item_id: &str,
    acquired_from: &str,
    equipped_slot: Option<&str>,
    now_epoch: i64,
) -> Option<InventoryItem> {
    let definition = ITEM_CATALOG.iter().find(|item| item.id == item_id)?;
    Some(InventoryItem {
        item_instance_id: stable_id("rpg-item", &format!("{player_id}:{item_id}")),
        item_id: item_id.to_string(),
        slot: definition.slot.to_string(),
        family: definition.family.to_string(),
        display_name: definition.display_name.to_string(),
        quantity: 1,
        quality: "standard".to_string(),
        equipped_slot: equipped_slot.map(str::to_string),
        acquired_from: acquired_from.to_string(),
        acquired_at_epoch: now_epoch,
        updated_at_epoch: now_epoch,
    })
}

fn starter_inventory(player_id: &str) -> Vec<InventoryItem> {
    [
        ("route-guard-staff", Some("weapon")),
        ("street-compass-bracer", Some("wrist")),
        ("evidence-wrap-case", Some("pack")),
    ]
    .into_iter()
    .filter_map(|(item, slot)| inventory_item_for(player_id, item, "starter", slot, 0))
    .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Character {
    pub matrix_user_id: String,
    pub character_id: String,
    pub display_name: String,
    pub attributes: TrillionniumAttributes,
    pub sect_id: Option<String>,
    pub title: String,
    pub skill_ids: Vec<String>,
    #[serde(default)]
    pub inventory_items: Vec<InventoryItem>,
    #[serde(default)]
    pub equipment_slots: HashMap<String, String>,
    pub updated_at_epoch: i64,
}

impl Character {
    pub fn default_for(player_id: &str) -> Self {
        let inventory_items = starter_inventory(player_id);
        let equipment_slots = inventory_items
            .iter()
            .filter_map(|item| {
                item.equipped_slot
                    .as_ref()
                    .map(|slot| (slot.clone(), item.item_instance_id.clone()))
            })
            .collect();
        Self {
            matrix_user_id: player_id.to_string(),
            character_id: stable_id("rpg-character", player_id),
            display_name: "Mirror Ranger".to_string(),
            attributes: TrillionniumAttributes::default(),
            sect_id: None,
            title: "Mirror City Initiate".to_string(),
            skill_ids: vec![
                "basic_inner_power".to_string(),
                "basic_lightness".to_string(),
                "reading_and_contracts".to_string(),
            ],
            inventory_items,
            equipment_slots,
            updated_at_epoch: 0,
        }
    }

    pub fn ensure_defaults(&mut self) {
        if self.inventory_items.is_empty() {
            self.inventory_items = starter_inventory(&self.matrix_user_id);
        }
    }

    pub fn equip_item_by_id(&mut self, item_id: &str, now_epoch: i64) -> Option<(String, String)> {
        self.ensure_defaults();
        let (slot, instance) = {
            let item = self
                .inventory_items
                .iter_mut()
                .find(|candidate| candidate.item_id == item_id)?;
            item.updated_at_epoch = now_epoch;
            item.equipped_slot = Some(item.slot.clone());
            (item.slot.clone(), item.item_instance_id.clone())
        };
        self.equipment_slots.insert(slot.clone(), instance.clone());
        self.updated_at_epoch = now_epoch;
        Some((slot, instance))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attributes_and_typed_equipment_are_small_and_deterministic() {
        let attributes = TrillionniumAttributes::default();
        assert!(attributes.derived_stats().max_hp > 100);
        let first = inventory_item_for("player", "relay-core-fragment", "loot", None, 1).unwrap();
        let second = inventory_item_for("player", "relay-core-fragment", "loot", None, 2).unwrap();
        assert_eq!(first.item_instance_id, second.item_instance_id);
        assert_eq!(first.slot, "relic");
    }
}
