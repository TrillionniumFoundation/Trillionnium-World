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
        return Err("Online Authority map is not in the authored allowlist".to_string());
    }
    let path = asset_root
        .join("first_contact/maps")
        .join(format!("{map_id}.yaml"));
    let source =
        fs::read_to_string(&path).map_err(|error| format!("read {}: {error}", path.display()))?;
    let mut value: serde_yaml::Value =
        serde_yaml::from_str(&source).map_err(|error| format!("decode map: {error}"))?;
    let extends = value
        .get("extends")
        .and_then(serde_yaml::Value::as_str)
        .map(str::to_string);
    if let Some(extends) = extends {
        let base_path = path.with_file_name(format!("{extends}.yaml"));
        let base_source = fs::read_to_string(&base_path)
            .map_err(|error| format!("read {}: {error}", base_path.display()))?;
        let mut base: serde_yaml::Value = serde_yaml::from_str(&base_source)
            .map_err(|error| format!("decode base map: {error}"))?;
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
    let mut authored: AuthoredMap =
        serde_yaml::from_value(value).map_err(|error| format!("decode map: {error}"))?;
    apply_terrain_transform(&mut authored.terrain_rows, transform.as_deref())?;
    if authored.contract_version != "trnm_first_contact_map_v1" || authored.id != map_id {
        return Err("authored map contract/id mismatch".to_string());
    }
    let south_pass = authored
        .chokepoints
        .iter()
        .find(|value| value.id == "south_pass")
        .ok_or_else(|| "authored map is missing south_pass".to_string())?;
    let mut map = BattleMapSeedV1 {
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
    map.party_start = nearest_passable(&map, map.party_start)
        .ok_or_else(|| "authored map has no passable player start".to_string())?;
    map.approach_point = nearest_passable(&map, map.approach_point)
        .ok_or_else(|| "authored map has no passable approach point".to_string())?;
    map.objective = nearest_passable(&map, map.objective)
        .ok_or_else(|| "authored map has no passable objective".to_string())?;
    let resource_positions = map
        .resource_nodes
        .iter()
        .map(|node| nearest_passable(&map, node.position))
        .collect::<Vec<_>>();
    for (node, position) in map.resource_nodes.iter_mut().zip(resource_positions) {
        node.position = position
            .ok_or_else(|| format!("authored resource {} has no passable tile", node.id))?;
    }
    let enemy_positions = map
        .enemy_spawns
        .iter()
        .map(|node| nearest_passable(&map, node.position))
        .collect::<Vec<_>>();
    for (node, position) in map.enemy_spawns.iter_mut().zip(enemy_positions) {
        node.position =
            position.ok_or_else(|| format!("authored enemy {} has no passable tile", node.id))?;
    }
    map.validate().map_err(|error| error.to_string())?;
    Ok(map)
}

fn nearest_passable(map: &BattleMapSeedV1, point: BattleGridPoint) -> Option<BattleGridPoint> {
    for radius in 0..=12_i16 {
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                if dx.abs() + dy.abs() != radius {
                    continue;
                }
                let candidate = BattleGridPoint::new(point.x + dx, point.y + dy);
                if map.in_bounds(candidate) && map.passable(candidate) {
                    return Some(candidate);
                }
            }
        }
    }
    None
}

fn merge_yaml(base: &mut serde_yaml::Value, overlay: serde_yaml::Value) {
    match (base, overlay) {
        (serde_yaml::Value::Mapping(base), serde_yaml::Value::Mapping(overlay)) => {
            for (key, value) in overlay {
                if key.as_str() != Some("extends") {
                    base.insert(key, value);
                }
            }
        }
        (base, overlay) => *base = overlay,
    }
}

fn apply_terrain_transform(rows: &mut Vec<String>, transform: Option<&str>) -> Result<(), String> {
    match transform {
        Some("mirror_x") => {
            for row in rows {
                *row = row.chars().rev().collect();
            }
        }
        Some("rotate_180") => {
            rows.reverse();
            for row in rows {
                *row = row.chars().rev().collect();
            }
        }
        Some("shift_3") | Some("shift_5") | Some("shift_7") | Some("shift_11") => {
            let amount = match transform {
                Some("shift_3") => 3,
                Some("shift_5") => 5,
                Some("shift_7") => 7,
                _ => 11,
            };
            for row in rows {
                let mut chars = row.chars().collect::<Vec<_>>();
                chars.rotate_left(amount);
                *row = chars.into_iter().collect();
            }
        }
        Some(other) => return Err(format!("unsupported terrain transform: {other}")),
        None => {}
    }
    Ok(())
}
