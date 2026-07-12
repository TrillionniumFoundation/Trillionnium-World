use serde::Deserialize;
use std::{fs, path::Path};
use trnm_campaign_core::{BattleGridPoint, BattleMapNodeV1, BattleMapSeedV1};

#[derive(Debug, Deserialize)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Deserialize)]
struct Chokepoint {
    id: String,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

#[derive(Debug, Deserialize)]
struct MapNode {
    id: String,
    x: i32,
    y: i32,
    #[serde(default)]
    owner: String,
}

#[derive(Debug, Deserialize)]
struct AuthoredMap {
    contract_version: String,
    id: String,
    width: u16,
    height: u16,
    player_start: Point,
    objective: MapNode,
    terrain_rows: Vec<String>,
    chokepoints: Vec<Chokepoint>,
    resources: Vec<MapNode>,
    units: Vec<MapNode>,
}

pub fn load_authoritative_map(asset_root: &Path, map_id: &str) -> Result<BattleMapSeedV1, String> {
    if !matches!(
        map_id,
        "first_contact"
            | "iron_delta"
            | "night_watch_crossing"
            | "glass_basin"
            | "ember_orchard"
            | "salt_marsh"
            | "cinder_crown"
    ) {
        return Err("Online Authority v2 map is not in the authored allowlist".to_string());
    }
    let path = asset_root
        .join("first_contact/maps")
        .join(format!("{map_id}.yaml"));
    let bytes = fs::read(&path).map_err(|error| format!("read {}: {error}", path.display()))?;
    let authored: AuthoredMap =
        serde_yaml::from_slice(&bytes).map_err(|error| format!("decode map: {error}"))?;
    if authored.contract_version != "trnm_first_contact_map_v1" || authored.id != map_id {
        return Err("authored map contract/id mismatch".to_string());
    }
    let south_pass = authored
        .chokepoints
        .iter()
        .find(|value| value.id == "south_pass")
        .ok_or_else(|| "authored map is missing south_pass".to_string())?;
    let map = BattleMapSeedV1 {
        width: authored.width,
        height: authored.height,
        terrain_rows: authored.terrain_rows,
        party_start: BattleGridPoint::new(
            authored.player_start.x as i16,
            authored.player_start.y as i16,
        ),
        approach_point: BattleGridPoint::new(
            (south_pass.x + south_pass.width as i32 / 2) as i16,
            (south_pass.y + south_pass.height as i32 / 2) as i16,
        ),
        objective: BattleGridPoint::new(authored.objective.x as i16, authored.objective.y as i16),
        resource_nodes: authored
            .resources
            .into_iter()
            .map(|value| BattleMapNodeV1 {
                id: value.id,
                position: BattleGridPoint::new(value.x as i16, value.y as i16),
            })
            .collect(),
        enemy_spawns: authored
            .units
            .into_iter()
            .filter(|value| value.owner == "contact")
            .map(|value| BattleMapNodeV1 {
                id: value.id,
                position: BattleGridPoint::new(value.x as i16, value.y as i16),
            })
            .collect(),
    };
    map.validate().map_err(|error| error.to_string())?;
    Ok(map)
}
