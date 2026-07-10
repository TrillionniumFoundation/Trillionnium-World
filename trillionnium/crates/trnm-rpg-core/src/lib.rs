//! Minimal RPG state consumed by the playable campaign.
//!
//! Historical CEX/world fixtures intentionally live outside the game product
//! workspace. This crate owns only character attributes and typed equipment.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashMap};

pub const MIRROR_SQUARE_ROOM: &str = "mirror_square";
pub const MENTOR_HALL_ROOM: &str = "mentor_hall";
pub const EXPEDITION_GATE_ROOM: &str = "expedition_gate";
pub const RELAY_QUARTER_ROOM: &str = "relay_quarter";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldRoom {
    pub id: String,
    pub title: String,
    pub region_id: String,
    #[serde(default)]
    pub unlock_flag: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldExit {
    pub from: String,
    pub to: String,
    pub direction: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldGraph {
    pub rooms: BTreeMap<String, WorldRoom>,
    pub exits: Vec<WorldExit>,
}

impl WorldGraph {
    pub fn validate(&self) -> Result<(), String> {
        if self.rooms.is_empty() {
            return Err("world_graph_has_no_rooms".to_string());
        }
        for (id, room) in &self.rooms {
            if id != &room.id || room.title.trim().is_empty() || room.region_id.trim().is_empty() {
                return Err(format!("world_room_invalid:{id}"));
            }
        }
        for exit in &self.exits {
            if exit.direction.trim().is_empty()
                || !self.rooms.contains_key(&exit.from)
                || !self.rooms.contains_key(&exit.to)
                || exit.from == exit.to
            {
                return Err(format!("world_exit_invalid:{}:{}", exit.from, exit.to));
            }
        }
        Ok(())
    }

    pub fn room(&self, id: &str) -> Option<&WorldRoom> {
        self.rooms.get(id)
    }

    pub fn can_enter(&self, id: &str, world_flags: &BTreeSet<String>) -> Result<(), String> {
        let room = self
            .room(id)
            .ok_or_else(|| format!("unknown_world_room:{id}"))?;
        if let Some(flag) = &room.unlock_flag {
            if !world_flags.contains(flag) {
                return Err(format!("world_room_locked:{id}:requires:{flag}"));
            }
        }
        Ok(())
    }

    pub fn transition(
        &self,
        from: &str,
        to: &str,
        world_flags: &BTreeSet<String>,
    ) -> Result<&WorldRoom, String> {
        self.validate()?;
        if from == to {
            self.can_enter(to, world_flags)?;
            return self
                .room(to)
                .ok_or_else(|| format!("unknown_world_room:{to}"));
        }
        if !self
            .exits
            .iter()
            .any(|exit| exit.from == from && exit.to == to)
        {
            return Err(format!("world_rooms_not_adjacent:{from}:{to}"));
        }
        self.can_enter(to, world_flags)?;
        self.room(to)
            .ok_or_else(|| format!("unknown_world_room:{to}"))
    }
}

pub fn mirror_city_world_graph() -> WorldGraph {
    let rooms = [
        WorldRoom {
            id: MIRROR_SQUARE_ROOM.to_string(),
            title: "镜城广场".to_string(),
            region_id: "mirror_city".to_string(),
            unlock_flag: None,
        },
        WorldRoom {
            id: MENTOR_HALL_ROOM.to_string(),
            title: "街指南师父居".to_string(),
            region_id: "mirror_city".to_string(),
            unlock_flag: None,
        },
        WorldRoom {
            id: EXPEDITION_GATE_ROOM.to_string(),
            title: "信标出征口".to_string(),
            region_id: "mirror_city".to_string(),
            unlock_flag: Some("expedition_gate_open".to_string()),
        },
        WorldRoom {
            id: RELAY_QUARTER_ROOM.to_string(),
            title: "中继新街".to_string(),
            region_id: "signal_road".to_string(),
            unlock_flag: Some("signal_road_secured".to_string()),
        },
    ]
    .into_iter()
    .map(|room| (room.id.clone(), room))
    .collect();
    let mut exits = Vec::new();
    for (from, to, direction) in [
        (MIRROR_SQUARE_ROOM, MENTOR_HALL_ROOM, "north"),
        (MENTOR_HALL_ROOM, MIRROR_SQUARE_ROOM, "south"),
        (MIRROR_SQUARE_ROOM, EXPEDITION_GATE_ROOM, "east"),
        (EXPEDITION_GATE_ROOM, MIRROR_SQUARE_ROOM, "west"),
        (MENTOR_HALL_ROOM, EXPEDITION_GATE_ROOM, "southeast"),
        (EXPEDITION_GATE_ROOM, MENTOR_HALL_ROOM, "northwest"),
        (MIRROR_SQUARE_ROOM, RELAY_QUARTER_ROOM, "northeast"),
        (RELAY_QUARTER_ROOM, MIRROR_SQUARE_ROOM, "southwest"),
    ] {
        exits.push(WorldExit {
            from: from.to_string(),
            to: to.to_string(),
            direction: direction.to_string(),
        });
    }
    WorldGraph { rooms, exits }
}

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

    #[test]
    fn mirror_city_graph_enforces_adjacency_and_story_locks() {
        let graph = mirror_city_world_graph();
        graph.validate().unwrap();
        let mut flags = BTreeSet::new();
        assert!(graph
            .transition(MIRROR_SQUARE_ROOM, EXPEDITION_GATE_ROOM, &flags)
            .unwrap_err()
            .contains("expedition_gate_open"));
        flags.insert("expedition_gate_open".to_string());
        assert_eq!(
            graph
                .transition(MIRROR_SQUARE_ROOM, EXPEDITION_GATE_ROOM, &flags)
                .unwrap()
                .id,
            EXPEDITION_GATE_ROOM
        );
        assert!(graph
            .transition(EXPEDITION_GATE_ROOM, RELAY_QUARTER_ROOM, &flags)
            .unwrap_err()
            .contains("not_adjacent"));
        assert!(graph
            .transition(MIRROR_SQUARE_ROOM, RELAY_QUARTER_ROOM, &flags)
            .unwrap_err()
            .contains("signal_road_secured"));
    }
}
