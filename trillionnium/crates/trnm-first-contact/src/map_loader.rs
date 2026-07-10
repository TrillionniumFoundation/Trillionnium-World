use bevy::prelude::Resource;
use serde::Deserialize;
use std::{collections::BTreeMap, fs, path::Path};
use trnm_campaign_core::{BattleGridPoint, BattleMapNodeV1, BattleMapSeedV1};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
pub struct MapPoint {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MapObjective {
    pub id: String,
    pub label: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MapBase {
    pub id: String,
    pub owner: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub entrance_x: i32,
    pub entrance_y: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MapChokepoint {
    pub id: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub approach: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MapResourceNode {
    pub id: String,
    pub kind: String,
    pub x: i32,
    pub y: i32,
    pub route: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MapLandmark {
    pub id: String,
    pub role: String,
    pub x: i32,
    pub y: i32,
    pub frame: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MapStructure {
    pub id: String,
    pub family: String,
    pub owner: String,
    pub x: i32,
    pub y: i32,
    pub active: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MapUnit {
    pub id: String,
    #[serde(default)]
    pub spawn_slot: Option<String>,
    pub family: String,
    pub owner: String,
    pub x: i32,
    pub y: i32,
    pub selected: bool,
}

#[derive(Debug, Clone, Deserialize, Resource)]
pub struct FirstContactMap {
    pub contract_version: String,
    pub id: String,
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub tile_size: u32,
    pub player_start: MapPoint,
    pub camera_start: MapPoint,
    pub objective: MapObjective,
    pub terrain_legend: BTreeMap<char, String>,
    pub terrain_rows: Vec<String>,
    pub height_rows: Vec<String>,
    pub bases: Vec<MapBase>,
    pub chokepoints: Vec<MapChokepoint>,
    pub resources: Vec<MapResourceNode>,
    pub landmarks: Vec<MapLandmark>,
    pub structures: Vec<MapStructure>,
    pub units: Vec<MapUnit>,
}

impl FirstContactMap {
    pub fn battle_seed_map(&self) -> Result<BattleMapSeedV1, String> {
        let south_pass = self
            .chokepoints
            .iter()
            .find(|choke| choke.id == "south_pass")
            .ok_or_else(|| "authored map is missing south_pass".to_string())?;
        let map = BattleMapSeedV1 {
            width: self.width as u16,
            height: self.height as u16,
            terrain_rows: self.terrain_rows.clone(),
            party_start: BattleGridPoint::new(
                self.player_start.x as i16,
                self.player_start.y as i16,
            ),
            approach_point: BattleGridPoint::new(
                (south_pass.x + south_pass.width as i32 / 2) as i16,
                (south_pass.y + south_pass.height as i32 / 2) as i16,
            ),
            objective: BattleGridPoint::new(self.objective.x as i16, self.objective.y as i16),
            resource_nodes: self
                .resources
                .iter()
                .map(|resource| BattleMapNodeV1 {
                    id: resource.id.clone(),
                    position: BattleGridPoint::new(resource.x as i16, resource.y as i16),
                })
                .collect(),
            enemy_spawns: self
                .units
                .iter()
                .filter(|unit| unit.owner == "contact")
                .map(|unit| BattleMapNodeV1 {
                    id: unit.id.clone(),
                    position: BattleGridPoint::new(unit.x as i16, unit.y as i16),
                })
                .collect(),
        };
        map.validate().map_err(|error| error.to_string())?;
        Ok(map)
    }

    pub fn terrain_at(&self, x: usize, y: usize) -> Option<char> {
        self.terrain_rows.get(y)?.chars().nth(x)
    }

    pub fn height_at(&self, x: usize, y: usize) -> u8 {
        self.height_rows
            .get(y)
            .and_then(|row| row.as_bytes().get(x))
            .and_then(|value| char::from(*value).to_digit(10))
            .unwrap_or(0) as u8
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.contract_version != "trnm_first_contact_map_v1" {
            return Err(format!(
                "unsupported First Contact map contract: {}",
                self.contract_version
            ));
        }
        if self.width < 24 || self.height < 18 || self.tile_size < 16 {
            return Err("First Contact map dimensions are below the playable slice minimum".into());
        }
        if self.id != "first_contact" || self.title.trim().is_empty() {
            return Err("First Contact map identity/title is invalid".into());
        }
        if self.terrain_rows.len() != self.height as usize
            || self.height_rows.len() != self.height as usize
        {
            return Err("terrain_rows/height_rows must exactly match map height".into());
        }
        for (name, rows) in [
            ("terrain_rows", &self.terrain_rows),
            ("height_rows", &self.height_rows),
        ] {
            if let Some((index, row)) = rows
                .iter()
                .enumerate()
                .find(|(_, row)| row.chars().count() != self.width as usize)
            {
                return Err(format!(
                    "{name}[{index}] width {} does not match {}",
                    row.chars().count(),
                    self.width
                ));
            }
        }
        for terrain in self.terrain_rows.iter().flat_map(|row| row.chars()) {
            if !self.terrain_legend.contains_key(&terrain) {
                return Err(format!(
                    "terrain key {terrain:?} is missing from terrain_legend"
                ));
            }
        }
        let distinct_terrain = self
            .terrain_rows
            .iter()
            .flat_map(|row| row.chars())
            .collect::<std::collections::BTreeSet<_>>();
        if distinct_terrain.len() < 3 {
            return Err("First Contact requires at least three authored terrain families".into());
        }
        if self.chokepoints.len() < 2 {
            return Err("First Contact requires two authored chokepoints".into());
        }
        if self.bases.len() < 2 {
            return Err("First Contact requires authored player/contact base outlines".into());
        }
        if self.resources.len() < 3 || self.landmarks.len() < 3 {
            return Err(
                "First Contact requires resource routes and front/mid/rear landmarks".into(),
            );
        }
        if self.structures.len() < 5 {
            return Err("First Contact requires at least five structures".into());
        }
        let in_bounds =
            |x: i32, y: i32| x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32;
        if self.bases.iter().any(|base| {
            base.id.trim().is_empty()
                || !matches!(base.owner.as_str(), "player" | "contact")
                || base.width < 6
                || base.height < 6
                || !in_bounds(base.x, base.y)
                || !in_bounds(base.entrance_x, base.entrance_y)
        }) {
            return Err("authored base outline or entrance is invalid".into());
        }
        if self.chokepoints.iter().any(|choke| {
            choke.id.trim().is_empty()
                || choke.approach.trim().is_empty()
                || choke.width < 2
                || choke.height < 2
                || !in_bounds(choke.x, choke.y)
        }) {
            return Err("authored chokepoint geometry is invalid".into());
        }
        if self.resources.iter().any(|resource| {
            resource.id.trim().is_empty()
                || resource.route.trim().is_empty()
                || !in_bounds(resource.x, resource.y)
        }) {
            return Err("authored resource route is invalid".into());
        }
        let landmark_roles = self
            .landmarks
            .iter()
            .map(|landmark| landmark.role.as_str())
            .collect::<std::collections::BTreeSet<_>>();
        if !["front", "mid", "rear"]
            .iter()
            .all(|role| landmark_roles.contains(role))
        {
            return Err("front/mid/rear landmark roles are required".into());
        }
        if self
            .structures
            .iter()
            .any(|structure| structure.id.trim().is_empty() || !in_bounds(structure.x, structure.y))
        {
            return Err("authored structure placement is invalid".into());
        }
        let unit_families = self
            .units
            .iter()
            .map(|unit| unit.family.as_str())
            .collect::<std::collections::BTreeSet<_>>();
        if unit_families.len() < 6 {
            return Err("First Contact requires six visually distinct unit families".into());
        }
        let player_spawn_slots = self
            .units
            .iter()
            .filter(|unit| unit.owner == "player" && unit.selected)
            .filter_map(|unit| unit.spawn_slot.as_deref())
            .collect::<std::collections::BTreeSet<_>>();
        if player_spawn_slots.len() != 4
            || !["party_0", "party_1", "party_2", "party_3"]
                .iter()
                .all(|slot| player_spawn_slots.contains(slot))
        {
            return Err(
                "selected player units must expose four unique campaign spawn slots".into(),
            );
        }
        for (label, point) in [
            ("player_start", self.player_start),
            ("camera_start", self.camera_start),
            (
                "objective",
                MapPoint {
                    x: self.objective.x,
                    y: self.objective.y,
                },
            ),
        ] {
            if point.x < 0
                || point.y < 0
                || point.x >= self.width as i32
                || point.y >= self.height as i32
            {
                return Err(format!("{label} is outside the authored map"));
            }
        }
        Ok(())
    }
}

pub fn load_first_contact_map(path: &Path) -> Result<FirstContactMap, String> {
    let source = fs::read_to_string(path).map_err(|error| {
        format!(
            "failed to read First Contact map {}: {error}",
            path.display()
        )
    })?;
    let map: FirstContactMap = serde_yaml::from_str(&source).map_err(|error| {
        format!(
            "failed to parse First Contact map {}: {error}",
            path.display()
        )
    })?;
    map.validate()?;
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authored_first_contact_map_loads_and_meets_vertical_slice_shape() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../assets/first_contact/maps/first_contact.yaml");
        let map = load_first_contact_map(&path).expect("authored map loads");
        assert_eq!(map.id, "first_contact");
        assert_eq!((map.width, map.height), (40, 24));
        assert_eq!(map.chokepoints.len(), 2);
        assert!(map.structures.len() >= 5);
        assert_eq!(
            map.units
                .iter()
                .map(|unit| unit.family.as_str())
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            6
        );
    }
}
