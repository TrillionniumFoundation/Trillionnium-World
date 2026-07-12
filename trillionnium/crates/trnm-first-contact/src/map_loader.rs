use bevy::prelude::Resource;
use serde::Deserialize;
use std::{collections::BTreeMap, fs, path::Path};
use trnm_campaign_core::{BattleGridPoint, BattleMapNodeV1, BattleMapSeedV1, CampaignMission};

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

#[derive(Debug, Clone, Resource)]
pub struct MissionMapCatalog {
    pub first_contact: FirstContactMap,
    pub aftershock_patrol: FirstContactMap,
    pub convoy_exodus: FirstContactMap,
    pub mirror_siege: FirstContactMap,
    pub iron_delta: FirstContactMap,
    pub night_watch_crossing: FirstContactMap,
    pub glass_basin: FirstContactMap,
    pub ember_orchard: FirstContactMap,
    pub salt_marsh: FirstContactMap,
    pub cinder_crown: FirstContactMap,
}

impl MissionMapCatalog {
    pub fn load(asset_root: &Path) -> Result<Self, String> {
        Ok(Self {
            first_contact: load_first_contact_map(
                &asset_root.join("first_contact/maps/first_contact.yaml"),
            )?,
            aftershock_patrol: load_first_contact_map(
                &asset_root.join("first_contact/maps/aftershock_patrol.yaml"),
            )?,
            convoy_exodus: load_first_contact_map(
                &asset_root.join("first_contact/maps/convoy_exodus.yaml"),
            )?,
            mirror_siege: load_first_contact_map(
                &asset_root.join("first_contact/maps/mirror_siege.yaml"),
            )?,
            iron_delta: load_first_contact_map(
                &asset_root.join("first_contact/maps/iron_delta.yaml"),
            )?,
            night_watch_crossing: load_first_contact_map(
                &asset_root.join("first_contact/maps/night_watch_crossing.yaml"),
            )?,
            glass_basin: load_first_contact_map(
                &asset_root.join("first_contact/maps/glass_basin.yaml"),
            )?,
            ember_orchard: load_first_contact_map(
                &asset_root.join("first_contact/maps/ember_orchard.yaml"),
            )?,
            salt_marsh: load_first_contact_map(
                &asset_root.join("first_contact/maps/salt_marsh.yaml"),
            )?,
            cinder_crown: load_first_contact_map(
                &asset_root.join("first_contact/maps/cinder_crown.yaml"),
            )?,
        })
    }

    pub fn for_mission(&self, mission: CampaignMission) -> &FirstContactMap {
        match mission {
            CampaignMission::FirstContact => &self.first_contact,
            CampaignMission::AftershockPatrol => &self.aftershock_patrol,
            CampaignMission::ConvoyExodus => &self.convoy_exodus,
            CampaignMission::MirrorSiege => &self.mirror_siege,
            CampaignMission::IronDeltaSkirmish => &self.iron_delta,
            CampaignMission::NightWatchCrossingSkirmish => &self.night_watch_crossing,
            CampaignMission::GlassBasinSkirmish => &self.glass_basin,
            CampaignMission::EmberOrchardSkirmish => &self.ember_orchard,
            CampaignMission::SaltMarshSkirmish => &self.salt_marsh,
            CampaignMission::CinderCrownSkirmish => &self.cinder_crown,
        }
    }
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
        if !matches!(
            self.id.as_str(),
            "first_contact"
                | "aftershock_patrol"
                | "convoy_exodus"
                | "mirror_siege"
                | "iron_delta"
                | "night_watch_crossing"
                | "glass_basin"
                | "ember_orchard"
                | "salt_marsh"
                | "cinder_crown"
        ) || self.title.trim().is_empty()
        {
            return Err("campaign map identity/title is invalid".into());
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
    let mut value: serde_yaml::Value = serde_yaml::from_str(&source).map_err(|error| {
        format!(
            "failed to parse First Contact map {}: {error}",
            path.display()
        )
    })?;
    let extends = value
        .get("extends")
        .and_then(serde_yaml::Value::as_str)
        .map(str::to_string);
    if let Some(extends) = extends {
        let base_path = path.with_file_name(format!("{extends}.yaml"));
        let base_source = fs::read_to_string(&base_path).map_err(|error| {
            format!(
                "failed to read extended map {}: {error}",
                base_path.display()
            )
        })?;
        let mut base: serde_yaml::Value = serde_yaml::from_str(&base_source).map_err(|error| {
            format!(
                "failed to parse extended map {}: {error}",
                base_path.display()
            )
        })?;
        merge_yaml(&mut base, value);
        value = base;
    }
    let transform = value
        .get("terrain_transform")
        .and_then(serde_yaml::Value::as_str)
        .map(str::to_string);
    if let Some(mapping) = value.as_mapping_mut() {
        mapping.remove(serde_yaml::Value::String("extends".to_string()));
        mapping.remove(serde_yaml::Value::String("terrain_transform".to_string()));
    }
    let mut map: FirstContactMap = serde_yaml::from_value(value).map_err(|error| {
        format!(
            "failed to materialize First Contact map {}: {error}",
            path.display()
        )
    })?;
    match transform.as_deref() {
        Some("mirror_x") => {
            for row in map
                .terrain_rows
                .iter_mut()
                .chain(map.height_rows.iter_mut())
            {
                *row = row.chars().rev().collect();
            }
        }
        Some("rotate_180") => {
            for rows in [&mut map.terrain_rows, &mut map.height_rows] {
                rows.reverse();
                for row in rows.iter_mut() {
                    *row = row.chars().rev().collect();
                }
            }
        }
        Some("shift_3") | Some("shift_5") | Some("shift_7") | Some("shift_11") => {
            let amount = match transform.as_deref() {
                Some("shift_3") => 3,
                Some("shift_5") => 5,
                Some("shift_7") => 7,
                _ => 11,
            };
            for row in map
                .terrain_rows
                .iter_mut()
                .chain(map.height_rows.iter_mut())
            {
                let mut chars = row.chars().collect::<Vec<_>>();
                chars.rotate_left(amount);
                *row = chars.into_iter().collect();
            }
        }
        Some(other) => return Err(format!("unsupported terrain transform: {other}")),
        None => {}
    }
    map.validate()?;
    Ok(map)
}

fn merge_yaml(base: &mut serde_yaml::Value, overlay: serde_yaml::Value) {
    match (base, overlay) {
        (serde_yaml::Value::Mapping(base), serde_yaml::Value::Mapping(overlay)) => {
            for (key, value) in overlay {
                if key.as_str() == Some("extends") {
                    continue;
                }
                base.insert(key, value);
            }
        }
        (base, overlay) => *base = overlay,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use trnm_campaign_core::{CampaignFaction, CampaignMission, CampaignSaveV1};
    use trnm_rts_sim::run_skirmish_balance_matrix;

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

    #[test]
    fn campaign_catalog_contains_ten_distinct_authored_maps() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../assets");
        let catalog = MissionMapCatalog::load(&root).expect("both authored maps load");
        assert_eq!(catalog.aftershock_patrol.id, "aftershock_patrol");
        assert_ne!(
            catalog.first_contact.terrain_rows,
            catalog.aftershock_patrol.terrain_rows
        );
        assert_ne!(
            catalog.first_contact.chokepoints[1].width,
            catalog.aftershock_patrol.chokepoints[1].width
        );
        assert_eq!(catalog.convoy_exodus.id, "convoy_exodus");
        assert_ne!(
            catalog.convoy_exodus.terrain_rows,
            catalog.aftershock_patrol.terrain_rows
        );
        assert_eq!(catalog.mirror_siege.id, "mirror_siege");
        assert_ne!(
            catalog.mirror_siege.terrain_rows,
            catalog.convoy_exodus.terrain_rows
        );
        assert_eq!(
            catalog
                .mirror_siege
                .units
                .iter()
                .filter(|unit| unit.owner == "contact")
                .count(),
            5
        );
        assert_eq!(catalog.iron_delta.id, "iron_delta");
        assert_eq!(catalog.night_watch_crossing.id, "night_watch_crossing");
        assert_eq!(catalog.glass_basin.id, "glass_basin");
        assert_eq!(catalog.ember_orchard.id, "ember_orchard");
        assert_eq!(catalog.salt_marsh.id, "salt_marsh");
        assert_eq!(catalog.cinder_crown.id, "cinder_crown");
        assert_ne!(
            catalog.iron_delta.terrain_rows,
            catalog.first_contact.terrain_rows
        );
        assert_ne!(
            catalog.night_watch_crossing.terrain_rows,
            catalog.convoy_exodus.terrain_rows
        );
        assert_ne!(
            catalog.iron_delta.terrain_rows,
            catalog.night_watch_crossing.terrain_rows
        );
        assert_ne!(
            (
                catalog.iron_delta.player_start.x,
                catalog.iron_delta.player_start.y
            ),
            (
                catalog.night_watch_crossing.player_start.x,
                catalog.night_watch_crossing.player_start.y
            )
        );
        assert_ne!(
            catalog.iron_delta.objective.id,
            catalog.night_watch_crossing.objective.id
        );
        let skirmish_terrain = [
            &catalog.iron_delta,
            &catalog.night_watch_crossing,
            &catalog.glass_basin,
            &catalog.ember_orchard,
            &catalog.salt_marsh,
            &catalog.cinder_crown,
        ]
        .into_iter()
        .map(|map| map.terrain_rows.join("\n"))
        .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(skirmish_terrain.len(), 6);
        let skirmish_objectives = [
            &catalog.iron_delta,
            &catalog.night_watch_crossing,
            &catalog.glass_basin,
            &catalog.ember_orchard,
            &catalog.salt_marsh,
            &catalog.cinder_crown,
        ]
        .into_iter()
        .map(|map| map.objective.id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(skirmish_objectives.len(), 6);
    }

    #[test]
    fn real_authored_maps_faction_swaps_and_seed_salts_drive_balance_matrix() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../assets");
        let catalog = MissionMapCatalog::load(&root).unwrap();
        let cases = [
            (CampaignMission::IronDeltaSkirmish, &catalog.iron_delta),
            (
                CampaignMission::NightWatchCrossingSkirmish,
                &catalog.night_watch_crossing,
            ),
            (CampaignMission::GlassBasinSkirmish, &catalog.glass_basin),
            (
                CampaignMission::EmberOrchardSkirmish,
                &catalog.ember_orchard,
            ),
        ];
        let mut seeds = Vec::new();
        for (mission, map) in cases {
            for player_faction in [
                CampaignFaction::MirrorCoalition,
                CampaignFaction::AshenCompact,
            ] {
                for seed_index in 0..2 {
                    let mut campaign = CampaignSaveV1 {
                        campaign_id: format!(
                            "real-balance-{}-{player_faction:?}-{seed_index}",
                            map.id
                        ),
                        ..CampaignSaveV1::default()
                    };
                    campaign.prepare_standalone_skirmish().unwrap();
                    campaign.active_mission = mission;
                    campaign.skirmish_setup.player_faction = player_faction;
                    campaign.skirmish_setup.enemy_faction = player_faction.opponent();
                    campaign.skirmish_setup.simulation_seed = seed_index + 1;
                    seeds.push(
                        campaign
                            .start_first_contact_battle(map.battle_seed_map().unwrap())
                            .unwrap(),
                    );
                }
            }
        }
        // This adapter test owns authored-map/faction/seed identity, while the
        // 64-sample terminal economy matrix lives in trnm-rts-sim. Three
        // hundred ticks are enough to expose distinct pressure/fingerprints
        // without making this client package duplicate the terminal gate.
        let matrix = run_skirmish_balance_matrix(&seeds, 300).unwrap();
        assert_eq!(matrix.samples.len(), 16);
        assert_eq!(
            matrix
                .samples
                .iter()
                .map(|sample| sample.map_fingerprint.as_str())
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            4
        );
        for map_id in [
            "iron_delta",
            "night_watch_crossing",
            "glass_basin",
            "ember_orchard",
        ] {
            assert_eq!(
                matrix
                    .samples
                    .iter()
                    .filter(|sample| sample.map_id == map_id)
                    .map(|sample| sample.simulation_salt)
                    .collect::<std::collections::BTreeSet<_>>()
                    .len(),
                2,
                "battle identity must produce real deterministic variation for {map_id}"
            );
        }
        assert!(matrix.faction_pressure_delta_permille <= 450);
    }
}
