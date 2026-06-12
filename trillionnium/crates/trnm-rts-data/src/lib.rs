//! Typed RTS data boundary for Trillionnium maps, rules, and source manifests.
//!
//! This crate is intentionally Bevy-free. It is the landing zone for imported
//! map/rule data before the playable runtime consumes it.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use trnm_rts_core::RtsTile;

pub const TRNM_RTS_DATA_CONTRACT: &str = "trnm_rts_data_map_model_v1";
pub const TRNM_RTS_DATA_SOURCE_MANIFEST_CONTRACT: &str = "trnm_rts_data_source_manifest_v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RtsDataIntegrationMode {
    ProjectOwned,
    DirectInternalImport,
    CleanRoomReference,
    AgplInternalComponent,
    GplInternalComponent,
}

impl RtsDataIntegrationMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectOwned => "project_owned",
            Self::DirectInternalImport => "direct_internal_import",
            Self::CleanRoomReference => "clean_room_reference",
            Self::AgplInternalComponent => "agpl_internal_component",
            Self::GplInternalComponent => "gpl_internal_component",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsMapBounds {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl RtsMapBounds {
    pub const fn max_x(self) -> i32 {
        self.x + self.width as i32 - 1
    }

    pub const fn max_y(self) -> i32 {
        self.y + self.height as i32 - 1
    }

    pub const fn contains(self, tile: RtsTile) -> bool {
        tile.x >= self.x && tile.y >= self.y && tile.x <= self.max_x() && tile.y <= self.max_y()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsDataSourceManifest {
    pub contract_version: String,
    pub source_id: String,
    pub upstream: String,
    pub local_audit_path: String,
    pub audited_commit: String,
    pub license: String,
    pub integration_mode: RtsDataIntegrationMode,
    pub release_constraint: String,
    pub copied_or_derived: bool,
    pub source_paths: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsMapPlayer {
    pub id: String,
    pub name: String,
    pub playable: bool,
    pub allow_bots: bool,
    pub faction: String,
    pub owns_world: bool,
    pub non_combatant: bool,
    pub enemies: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RtsRuleKind {
    Unit,
    Structure,
    Resource,
    Objective,
    Marker,
    Spawn,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsRule {
    pub id: String,
    pub label: String,
    pub kind: RtsRuleKind,
    pub faction: String,
    pub cost: u32,
    pub hp: u32,
    #[serde(default)]
    pub speed: Option<u32>,
    #[serde(default)]
    pub build_duration: Option<u32>,
    pub queue: String,
    pub traits: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsMapActor {
    pub id: String,
    pub rule_id: String,
    pub owner: String,
    pub tile: RtsTile,
    pub kind: RtsRuleKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RtsTerrainRole {
    Border,
    Lane,
    CentralBasin,
    BasePad,
    ResourceZone,
    Field,
}

impl RtsTerrainRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Border => "border",
            Self::Lane => "lane",
            Self::CentralBasin => "central_basin",
            Self::BasePad => "base_pad",
            Self::ResourceZone => "resource_zone",
            Self::Field => "field",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsTerrainTileProfile {
    pub tile: RtsTile,
    pub role: RtsTerrainRole,
    pub playable: bool,
    pub lane: bool,
    pub base_pad: bool,
    pub resource_zone: bool,
    pub height: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsMapModel {
    pub contract_version: String,
    pub map_id: String,
    pub title: String,
    pub author: String,
    pub requires_mod: String,
    pub tileset: String,
    pub width: u32,
    pub height: u32,
    pub bounds: RtsMapBounds,
    pub players: Vec<RtsMapPlayer>,
    pub rules: Vec<RtsRule>,
    pub actors: Vec<RtsMapActor>,
    pub source_manifest: RtsDataSourceManifest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsMapSummary {
    pub contract_version: String,
    pub map_id: String,
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub playable_min_x: i32,
    pub playable_min_y: i32,
    pub playable_max_x: i32,
    pub playable_max_y: i32,
    pub player_count: usize,
    pub playable_player_count: usize,
    pub actor_count: usize,
    pub rule_count: usize,
    pub spawn_count: usize,
    pub resource_count: usize,
    pub objective_count: usize,
    pub marker_count: usize,
    pub flux_bloom_count: usize,
    pub beacon_count: usize,
    pub expansion_count: usize,
    pub source_integration_mode: String,
    pub canonical_sha256: String,
}

impl RtsMapModel {
    pub fn validate(&self) -> Result<(), String> {
        if self.contract_version != TRNM_RTS_DATA_CONTRACT {
            return Err(format!("contract_mismatch:{}", self.contract_version));
        }
        if self.width == 0 || self.height == 0 {
            return Err("map_size_empty".to_string());
        }
        if self.bounds.x < 0 || self.bounds.y < 0 {
            return Err("map_bounds_negative_origin".to_string());
        }
        if self.bounds.max_x() >= self.width as i32 || self.bounds.max_y() >= self.height as i32 {
            return Err("map_bounds_outside_map".to_string());
        }
        if self.players.is_empty() {
            return Err("players_empty".to_string());
        }
        if self.rules.is_empty() {
            return Err("rules_empty".to_string());
        }
        if self.actors.is_empty() {
            return Err("actors_empty".to_string());
        }
        let mut player_ids = BTreeSet::new();
        for player in &self.players {
            if player.id.is_empty() {
                return Err("player_id_empty".to_string());
            }
            if !player_ids.insert(player.id.as_str()) {
                return Err(format!("player_id_duplicate:{}", player.id));
            }
        }
        let rule_ids = self
            .rules
            .iter()
            .map(|rule| rule.id.as_str())
            .collect::<BTreeSet<_>>();
        let mut actor_ids = BTreeSet::new();
        for actor in &self.actors {
            if actor.id.is_empty() {
                return Err("actor_id_empty".to_string());
            }
            if !actor_ids.insert(actor.id.as_str()) {
                return Err(format!("actor_id_duplicate:{}", actor.id));
            }
            if !rule_ids.contains(actor.rule_id.as_str()) {
                return Err(format!("actor_rule_missing:{}:{}", actor.id, actor.rule_id));
            }
            if !player_ids.contains(actor.owner.as_str()) {
                return Err(format!("actor_owner_missing:{}:{}", actor.id, actor.owner));
            }
            if !self.bounds.contains(actor.tile) {
                return Err(format!("actor_outside_playable_bounds:{}", actor.id));
            }
        }
        Ok(())
    }

    pub fn actor_count_by_kind(&self, kind: RtsRuleKind) -> usize {
        self.actors
            .iter()
            .filter(|actor| actor.kind == kind)
            .count()
    }

    pub fn actor_count_by_rule(&self, rule_id: &str) -> usize {
        self.actors
            .iter()
            .filter(|actor| actor.rule_id == rule_id)
            .count()
    }

    pub fn canonical_sha256(&self) -> String {
        let bytes = serde_json::to_vec(self).expect("RtsMapModel serializes");
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        format!("{:x}", hasher.finalize())
    }

    pub fn summary(&self) -> RtsMapSummary {
        RtsMapSummary {
            contract_version: TRNM_RTS_DATA_CONTRACT.to_string(),
            map_id: self.map_id.clone(),
            title: self.title.clone(),
            width: self.width,
            height: self.height,
            playable_min_x: self.bounds.x,
            playable_min_y: self.bounds.y,
            playable_max_x: self.bounds.max_x(),
            playable_max_y: self.bounds.max_y(),
            player_count: self.players.len(),
            playable_player_count: self.players.iter().filter(|player| player.playable).count(),
            actor_count: self.actors.len(),
            rule_count: self.rules.len(),
            spawn_count: self.actor_count_by_kind(RtsRuleKind::Spawn),
            resource_count: self.actor_count_by_kind(RtsRuleKind::Resource),
            objective_count: self.actor_count_by_kind(RtsRuleKind::Objective),
            marker_count: self.actor_count_by_kind(RtsRuleKind::Marker),
            flux_bloom_count: self.actor_count_by_rule("trnm.flux.bloom"),
            beacon_count: self.actor_count_by_rule("trnm.flux.beacon"),
            expansion_count: self.actor_count_by_rule("trnm.expansion.marker"),
            source_integration_mode: self.source_manifest.integration_mode.as_str().to_string(),
            canonical_sha256: self.canonical_sha256(),
        }
    }
}

pub fn first_contact_basin_map() -> RtsMapModel {
    RtsMapModel {
        contract_version: TRNM_RTS_DATA_CONTRACT.to_string(),
        map_id: "first_contact_basin".to_string(),
        title: "First Contact Basin".to_string(),
        author: "Trillionnium Authors".to_string(),
        requires_mod: "trnm".to_string(),
        tileset: "TRNM".to_string(),
        width: 34,
        height: 34,
        bounds: RtsMapBounds {
            x: 1,
            y: 1,
            width: 32,
            height: 32,
        },
        players: first_contact_players(),
        rules: first_contact_rules(),
        actors: first_contact_actors(),
        source_manifest: first_contact_source_manifest(),
    }
}

pub fn first_contact_source_manifest() -> RtsDataSourceManifest {
    RtsDataSourceManifest {
        contract_version: TRNM_RTS_DATA_SOURCE_MANIFEST_CONTRACT.to_string(),
        source_id: "trillionnium-rts-openra-modsdk-first-contact".to_string(),
        upstream: "local:/home/qian/.openclaw/workspace/TrillionniumRTS".to_string(),
        local_audit_path: "/home/qian/.openclaw/workspace/TrillionniumRTS".to_string(),
        audited_commit: "6fd679b576a1130558cd69b4e3ab2817f819dd22".to_string(),
        license: "GPL-3.0-or-later OpenRA Mod SDK prototype boundary".to_string(),
        integration_mode: RtsDataIntegrationMode::GplInternalComponent,
        release_constraint:
            "internal_only_until_gpl_component_review_or_replacement".to_string(),
        copied_or_derived: true,
        source_paths: vec![
            "mods/trnm/maps/first-contact-basin/map.yaml".to_string(),
            "mods/trnm/rules/trnm.yaml".to_string(),
            "mods/trnm/rules/mpspawn.yaml".to_string(),
            "mods/trnm/tilesets/trnm.yaml".to_string(),
            "mods/trnm/sequences/trnm.yaml".to_string(),
        ],
        notes: vec![
            "Typed Rust data model derived from local TrillionniumRTS YAML map/rules.".to_string(),
            "No Westwood, EA, Warcraft III, or proprietary original-game assets are bundled by this crate.".to_string(),
            "This crate is the first map/rules boundary for replacing hard-coded Bevy First Contact constants.".to_string(),
        ],
    }
}

fn first_contact_players() -> Vec<RtsMapPlayer> {
    vec![
        player(
            "Neutral",
            "Neutral",
            false,
            false,
            "Random",
            true,
            true,
            &[],
        ),
        player("Creeps", "Creeps", false, false, "Random", false, true, &[]),
        player(
            "Multi0",
            "Multi0",
            true,
            true,
            "horizon",
            false,
            false,
            &["Creeps", "Multi1", "Multi2", "Multi3"],
        ),
        player(
            "Multi1",
            "Multi1",
            true,
            true,
            "forge",
            false,
            false,
            &["Creeps", "Multi0", "Multi2", "Multi3"],
        ),
        player(
            "Multi2",
            "Multi2",
            true,
            true,
            "horizon",
            false,
            false,
            &["Creeps", "Multi0", "Multi1", "Multi3"],
        ),
        player(
            "Multi3",
            "Multi3",
            true,
            true,
            "forge",
            false,
            false,
            &["Creeps", "Multi0", "Multi1", "Multi2"],
        ),
    ]
}

fn player(
    id: &str,
    name: &str,
    playable: bool,
    allow_bots: bool,
    faction: &str,
    owns_world: bool,
    non_combatant: bool,
    enemies: &[&str],
) -> RtsMapPlayer {
    RtsMapPlayer {
        id: id.to_string(),
        name: name.to_string(),
        playable,
        allow_bots,
        faction: faction.to_string(),
        owns_world,
        non_combatant,
        enemies: enemies.iter().map(|enemy| (*enemy).to_string()).collect(),
    }
}

fn first_contact_rules() -> Vec<RtsRule> {
    vec![
        rule(
            "mpspawn",
            "Spawn",
            RtsRuleKind::Spawn,
            "neutral",
            0,
            1,
            None,
            None,
            "MapDetail",
            &["spawn"],
        ),
        rule(
            "trnm.worker",
            "Worker",
            RtsRuleKind::Unit,
            "shared",
            200,
            8000,
            Some(64),
            Some(100),
            "Unit",
            &["selectable", "mobile", "harvester", "builder"],
        ),
        rule(
            "trnm.horizon.scout",
            "Horizon Scout",
            RtsRuleKind::Unit,
            "horizon",
            250,
            9000,
            Some(92),
            Some(125),
            "Unit",
            &["selectable", "mobile", "attack"],
        ),
        rule(
            "trnm.horizon.skimmer",
            "Horizon Skimmer",
            RtsRuleKind::Unit,
            "horizon",
            475,
            11000,
            Some(104),
            Some(190),
            "Unit",
            &["selectable", "mobile", "attack"],
        ),
        rule(
            "trnm.forge.warden",
            "Forge Warden",
            RtsRuleKind::Unit,
            "forge",
            300,
            18000,
            Some(56),
            Some(150),
            "Unit",
            &["selectable", "mobile", "attack"],
        ),
        rule(
            "trnm.forge.bastion",
            "Forge Bastion",
            RtsRuleKind::Unit,
            "forge",
            575,
            24500,
            Some(48),
            Some(230),
            "Unit",
            &["selectable", "mobile", "attack"],
        ),
        rule(
            "trnm.striker",
            "Striker",
            RtsRuleKind::Unit,
            "shared",
            400,
            13000,
            Some(64),
            Some(175),
            "Unit",
            &["selectable", "mobile", "attack"],
        ),
        rule(
            "trnm.command.core",
            "Command Core",
            RtsRuleKind::Structure,
            "shared",
            1600,
            70000,
            None,
            None,
            "Building/Unit",
            &["selectable", "producer", "base_provider"],
        ),
        rule(
            "trnm.flux.relay",
            "Flux Relay",
            RtsRuleKind::Structure,
            "shared",
            500,
            70000,
            None,
            Some(180),
            "Building",
            &["selectable", "refinery", "buildable"],
        ),
        rule(
            "trnm.assembly.pad",
            "Assembly Pad",
            RtsRuleKind::Structure,
            "shared",
            700,
            70000,
            None,
            Some(220),
            "Building",
            &["selectable", "producer", "buildable"],
        ),
        rule(
            "trnm.signal.array",
            "Signal Array",
            RtsRuleKind::Structure,
            "shared",
            850,
            70000,
            None,
            Some(260),
            "Building",
            &["selectable", "tech", "buildable"],
        ),
        rule(
            "trnm.sentinel.node",
            "Sentinel Node",
            RtsRuleKind::Structure,
            "shared",
            550,
            52000,
            None,
            Some(200),
            "Building",
            &["selectable", "defense", "attack"],
        ),
        rule(
            "trnm.flux.beacon",
            "Flux Beacon",
            RtsRuleKind::Objective,
            "neutral",
            650,
            46000,
            None,
            None,
            "Objective",
            &["selectable", "capturable", "income"],
        ),
        rule(
            "trnm.flux.bloom",
            "Flux Bloom",
            RtsRuleKind::Resource,
            "neutral",
            0,
            1,
            None,
            None,
            "Resource",
            &["resource_seed"],
        ),
        rule(
            "trnm.map.ridge",
            "Map Ridge",
            RtsRuleKind::Marker,
            "neutral",
            0,
            1,
            None,
            None,
            "MapDetail",
            &["map_detail"],
        ),
        rule(
            "trnm.flux.vent",
            "Flux Vent",
            RtsRuleKind::Marker,
            "neutral",
            0,
            1,
            None,
            None,
            "MapDetail",
            &["map_detail"],
        ),
        rule(
            "trnm.lane.marker",
            "Lane Marker",
            RtsRuleKind::Marker,
            "neutral",
            0,
            1,
            None,
            None,
            "MapDetail",
            &["map_detail"],
        ),
        rule(
            "trnm.beacon.ring",
            "Beacon Ring",
            RtsRuleKind::Marker,
            "neutral",
            0,
            1,
            None,
            None,
            "MapDetail",
            &["map_detail"],
        ),
        rule(
            "trnm.expansion.marker",
            "Expansion Marker",
            RtsRuleKind::Marker,
            "neutral",
            0,
            1,
            None,
            None,
            "MapDetail",
            &["map_detail"],
        ),
    ]
}

fn rule(
    id: &str,
    label: &str,
    kind: RtsRuleKind,
    faction: &str,
    cost: u32,
    hp: u32,
    speed: Option<u32>,
    build_duration: Option<u32>,
    queue: &str,
    traits: &[&str],
) -> RtsRule {
    RtsRule {
        id: id.to_string(),
        label: label.to_string(),
        kind,
        faction: faction.to_string(),
        cost,
        hp,
        speed,
        build_duration,
        queue: queue.to_string(),
        traits: traits.iter().map(|value| (*value).to_string()).collect(),
    }
}

fn first_contact_actors() -> Vec<RtsMapActor> {
    let rows = [
        ("Actor0", "mpspawn", "Multi0", 8, 8, RtsRuleKind::Spawn),
        ("Actor1", "mpspawn", "Multi1", 25, 25, RtsRuleKind::Spawn),
        (
            "Actor2",
            "trnm.flux.bloom",
            "Neutral",
            12,
            16,
            RtsRuleKind::Resource,
        ),
        (
            "Actor3",
            "trnm.flux.bloom",
            "Neutral",
            21,
            16,
            RtsRuleKind::Resource,
        ),
        (
            "Actor4",
            "trnm.flux.bloom",
            "Neutral",
            16,
            12,
            RtsRuleKind::Resource,
        ),
        ("Actor5", "mpspawn", "Multi2", 25, 8, RtsRuleKind::Spawn),
        ("Actor6", "mpspawn", "Multi3", 8, 25, RtsRuleKind::Spawn),
        (
            "Actor7",
            "trnm.flux.bloom",
            "Neutral",
            16,
            21,
            RtsRuleKind::Resource,
        ),
        (
            "Actor8",
            "trnm.flux.bloom",
            "Neutral",
            8,
            16,
            RtsRuleKind::Resource,
        ),
        (
            "Actor9",
            "trnm.flux.bloom",
            "Neutral",
            25,
            16,
            RtsRuleKind::Resource,
        ),
        (
            "Actor10",
            "trnm.flux.bloom",
            "Neutral",
            10,
            10,
            RtsRuleKind::Resource,
        ),
        (
            "Actor11",
            "trnm.flux.bloom",
            "Neutral",
            23,
            23,
            RtsRuleKind::Resource,
        ),
        (
            "Actor12",
            "trnm.flux.bloom",
            "Neutral",
            23,
            10,
            RtsRuleKind::Resource,
        ),
        (
            "Actor13",
            "trnm.flux.bloom",
            "Neutral",
            10,
            23,
            RtsRuleKind::Resource,
        ),
        (
            "Actor14",
            "trnm.flux.bloom",
            "Neutral",
            16,
            16,
            RtsRuleKind::Resource,
        ),
        (
            "Actor15",
            "trnm.flux.beacon",
            "Neutral",
            16,
            9,
            RtsRuleKind::Objective,
        ),
        (
            "Actor16",
            "trnm.flux.beacon",
            "Neutral",
            24,
            16,
            RtsRuleKind::Objective,
        ),
        (
            "Actor17",
            "trnm.flux.beacon",
            "Neutral",
            16,
            24,
            RtsRuleKind::Objective,
        ),
        (
            "Actor18",
            "trnm.flux.beacon",
            "Neutral",
            9,
            16,
            RtsRuleKind::Objective,
        ),
        (
            "Actor19",
            "trnm.map.ridge",
            "Neutral",
            6,
            13,
            RtsRuleKind::Marker,
        ),
        (
            "Actor20",
            "trnm.map.ridge",
            "Neutral",
            27,
            20,
            RtsRuleKind::Marker,
        ),
        (
            "Actor21",
            "trnm.map.ridge",
            "Neutral",
            20,
            6,
            RtsRuleKind::Marker,
        ),
        (
            "Actor22",
            "trnm.map.ridge",
            "Neutral",
            13,
            27,
            RtsRuleKind::Marker,
        ),
        (
            "Actor23",
            "trnm.flux.vent",
            "Neutral",
            14,
            14,
            RtsRuleKind::Marker,
        ),
        (
            "Actor24",
            "trnm.flux.vent",
            "Neutral",
            19,
            19,
            RtsRuleKind::Marker,
        ),
        (
            "Actor25",
            "trnm.flux.vent",
            "Neutral",
            19,
            14,
            RtsRuleKind::Marker,
        ),
        (
            "Actor26",
            "trnm.flux.vent",
            "Neutral",
            14,
            19,
            RtsRuleKind::Marker,
        ),
        (
            "Actor27",
            "trnm.lane.marker",
            "Neutral",
            8,
            12,
            RtsRuleKind::Marker,
        ),
        (
            "Actor28",
            "trnm.lane.marker",
            "Neutral",
            25,
            21,
            RtsRuleKind::Marker,
        ),
        (
            "Actor29",
            "trnm.lane.marker",
            "Neutral",
            21,
            8,
            RtsRuleKind::Marker,
        ),
        (
            "Actor30",
            "trnm.lane.marker",
            "Neutral",
            12,
            25,
            RtsRuleKind::Marker,
        ),
        (
            "Actor31",
            "trnm.beacon.ring",
            "Neutral",
            16,
            10,
            RtsRuleKind::Marker,
        ),
        (
            "Actor32",
            "trnm.beacon.ring",
            "Neutral",
            23,
            16,
            RtsRuleKind::Marker,
        ),
        (
            "Actor33",
            "trnm.beacon.ring",
            "Neutral",
            16,
            23,
            RtsRuleKind::Marker,
        ),
        (
            "Actor34",
            "trnm.beacon.ring",
            "Neutral",
            10,
            16,
            RtsRuleKind::Marker,
        ),
        (
            "Actor35",
            "trnm.expansion.marker",
            "Neutral",
            11,
            8,
            RtsRuleKind::Marker,
        ),
        (
            "Actor36",
            "trnm.expansion.marker",
            "Neutral",
            22,
            25,
            RtsRuleKind::Marker,
        ),
        (
            "Actor37",
            "trnm.expansion.marker",
            "Neutral",
            22,
            8,
            RtsRuleKind::Marker,
        ),
        (
            "Actor38",
            "trnm.expansion.marker",
            "Neutral",
            11,
            25,
            RtsRuleKind::Marker,
        ),
    ];
    rows.into_iter()
        .map(|(id, rule_id, owner, x, y, kind)| RtsMapActor {
            id: id.to_string(),
            rule_id: rule_id.to_string(),
            owner: owner.to_string(),
            tile: RtsTile::new(x, y),
            kind,
        })
        .collect()
}

pub fn first_contact_terrain_profile(tile: RtsTile) -> RtsTerrainTileProfile {
    let x = tile.x;
    let y = tile.y;
    let playable = (1..=32).contains(&x) && (1..=32).contains(&y);
    let lane = first_contact_lane_tile(tile);
    let base_pad = first_contact_base_pad(tile);
    let resource_zone = first_contact_resource_zone(tile);
    let dx = (x - 16).abs();
    let dy = (y - 16).abs();
    let role = if !playable {
        RtsTerrainRole::Border
    } else if lane {
        RtsTerrainRole::Lane
    } else if dx <= 4 && dy <= 4 {
        RtsTerrainRole::CentralBasin
    } else if base_pad {
        RtsTerrainRole::BasePad
    } else if resource_zone {
        RtsTerrainRole::ResourceZone
    } else {
        RtsTerrainRole::Field
    };
    RtsTerrainTileProfile {
        tile,
        role,
        playable,
        lane,
        base_pad,
        resource_zone,
        height: first_contact_tile_height(tile),
    }
}

pub fn first_contact_terrain_profiles() -> Vec<RtsTerrainTileProfile> {
    (0..34)
        .flat_map(|y| (0..34).map(move |x| first_contact_terrain_profile(RtsTile::new(x, y))))
        .collect()
}

fn first_contact_lane_tile(tile: RtsTile) -> bool {
    let x = tile.x;
    let y = tile.y;
    x == 16 || y == 16 || (x - y).abs() <= 1 || (x + y - 33).abs() <= 1
}

fn first_contact_base_pad(tile: RtsTile) -> bool {
    let x = tile.x;
    let y = tile.y;
    (6..=11).contains(&x) && (6..=11).contains(&y)
        || (22..=27).contains(&x) && (22..=27).contains(&y)
        || (22..=27).contains(&x) && (6..=11).contains(&y)
        || (6..=11).contains(&x) && (22..=27).contains(&y)
}

fn first_contact_resource_zone(tile: RtsTile) -> bool {
    let x = tile.x;
    let y = tile.y;
    ((11..=14).contains(&x) && (14..=18).contains(&y))
        || ((19..=22).contains(&x) && (14..=18).contains(&y))
        || ((14..=18).contains(&x) && (11..=14).contains(&y))
        || ((14..=18).contains(&x) && (19..=22).contains(&y))
}

fn first_contact_tile_height(tile: RtsTile) -> u8 {
    let x = tile.x;
    let y = tile.y;
    let dx = (x - 16).abs();
    let dy = (y - 16).abs();
    if !(1..=32).contains(&x) || !(1..=32).contains(&y) {
        0
    } else if dx <= 3 && dy <= 3 {
        2
    } else if first_contact_lane_tile(tile) {
        1
    } else if first_contact_base_pad(tile) {
        2
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_contact_basin_matches_openra_modsdk_map_contract() {
        let map = first_contact_basin_map();
        map.validate().expect("first contact map validates");
        let summary = map.summary();
        assert_eq!(summary.map_id, "first_contact_basin");
        assert_eq!(summary.width, 34);
        assert_eq!(summary.height, 34);
        assert_eq!(summary.playable_min_x, 1);
        assert_eq!(summary.playable_min_y, 1);
        assert_eq!(summary.playable_max_x, 32);
        assert_eq!(summary.playable_max_y, 32);
        assert_eq!(summary.player_count, 6);
        assert_eq!(summary.playable_player_count, 4);
        assert_eq!(summary.actor_count, 39);
        assert_eq!(summary.spawn_count, 4);
        assert_eq!(summary.flux_bloom_count, 11);
        assert_eq!(summary.beacon_count, 4);
        assert_eq!(summary.expansion_count, 4);
        assert_eq!(summary.source_integration_mode, "gpl_internal_component");
    }

    #[test]
    fn first_contact_source_manifest_tracks_direct_internal_derivation() {
        let manifest = first_contact_source_manifest();
        assert_eq!(
            manifest.contract_version,
            TRNM_RTS_DATA_SOURCE_MANIFEST_CONTRACT
        );
        assert!(manifest.copied_or_derived);
        assert_eq!(
            manifest.audited_commit,
            "6fd679b576a1130558cd69b4e3ab2817f819dd22"
        );
        assert!(manifest
            .source_paths
            .contains(&"mods/trnm/maps/first-contact-basin/map.yaml".to_string()));
        assert_eq!(
            manifest.release_constraint,
            "internal_only_until_gpl_component_review_or_replacement"
        );
    }

    #[test]
    fn first_contact_canonical_hash_is_stable() {
        let map = first_contact_basin_map();
        let hash = map.canonical_sha256();
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|ch| ch.is_ascii_hexdigit()));
        assert_eq!(hash, first_contact_basin_map().canonical_sha256());
    }

    #[test]
    fn first_contact_terrain_profiles_preserve_render_roles() {
        let profiles = first_contact_terrain_profiles();
        assert_eq!(profiles.len(), 34 * 34);
        assert_eq!(
            first_contact_terrain_profile(RtsTile::new(0, 0)).role,
            RtsTerrainRole::Border
        );
        assert_eq!(
            first_contact_terrain_profile(RtsTile::new(16, 9)).role,
            RtsTerrainRole::Lane
        );
        assert_eq!(
            first_contact_terrain_profile(RtsTile::new(16, 16)).height,
            2
        );
        assert!(first_contact_terrain_profile(RtsTile::new(10, 10)).base_pad);
        assert!(first_contact_terrain_profile(RtsTile::new(12, 16)).resource_zone);
        assert!(
            profiles
                .iter()
                .filter(|profile| profile.role == RtsTerrainRole::Border)
                .count()
                >= 120
        );
        assert!(
            profiles
                .iter()
                .filter(|profile| profile.resource_zone)
                .count()
                >= 76
        );
    }
}
