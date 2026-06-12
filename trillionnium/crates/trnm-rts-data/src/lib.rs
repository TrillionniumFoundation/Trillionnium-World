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
pub const TRNM_RTS_DATA_FIRST_CONTACT_OPENING_PROFILE_CONTRACT: &str =
    "trnm_rts_data_first_contact_opening_profile_v1";
pub const TRNM_RTS_DATA_FIRST_CONTACT_COMMAND_FEEDBACK_CONTRACT: &str =
    "trnm_rts_data_first_contact_command_feedback_v1";
pub const TRNM_RTS_DATA_FIRST_CONTACT_PLAYER_STARTUP_CONTRACT: &str =
    "trnm_rts_data_first_contact_player_startup_v1";
pub const TRNM_RTS_DATA_FIRST_CONTACT_ACTOR_PRESENTATION_CONTRACT: &str =
    "trnm_rts_data_first_contact_actor_presentation_v1";
pub const TRNM_RTS_DATA_FIRST_CONTACT_ACTOR_GLYPH_CONTRACT: &str =
    "trnm_rts_data_first_contact_actor_glyph_v1";
pub const TRNM_RTS_DATA_FIRST_CONTACT_VISUAL_TELEMETRY_CONTRACT: &str =
    "trnm_rts_data_first_contact_visual_telemetry_v1";

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
pub struct RtsOpeningLoopProfile {
    pub contract_version: String,
    pub map_id: String,
    pub flux_bank: u32,
    pub worker_cargo: u32,
    pub worker_capacity: u32,
    pub relay_build_progress: u8,
    pub beacon_capture_progress: u8,
    pub worker_train_progress: u8,
    pub scout_train_progress: u8,
    pub active_beacon_tile: RtsTile,
    pub active_relay_tile: RtsTile,
    pub opening_actions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsCommandFeedbackProfile {
    pub contract_version: String,
    pub map_id: String,
    pub selected_group: String,
    pub active_order: String,
    pub target_tile: RtsTile,
    pub blocked_tile: RtsTile,
    pub blocked_reason: String,
    pub queued_before: u8,
    pub queued_after: u8,
    pub command_ack_progress: u8,
    pub cooldown_progress: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsPlayerStartupProfile {
    pub contract_version: String,
    pub map_id: String,
    pub player_id: String,
    pub faction: String,
    pub actor_id_prefix: String,
    pub spawn_tile: RtsTile,
    pub command_core_rule_id: String,
    pub worker_rule_id: String,
    pub faction_unit_rule_id: String,
    pub opening_harvest_tile: RtsTile,
    pub opening_relay_tile: RtsTile,
    pub opening_beacon_tile: RtsTile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RtsActorColorRole {
    Worker,
    Scout,
    Warden,
    Striker,
    CommandCore,
    FluxRelay,
    Objective,
    Resource,
    MapDetail,
}

impl RtsActorColorRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Worker => "worker",
            Self::Scout => "scout",
            Self::Warden => "warden",
            Self::Striker => "striker",
            Self::CommandCore => "command_core",
            Self::FluxRelay => "flux_relay",
            Self::Objective => "objective",
            Self::Resource => "resource",
            Self::MapDetail => "map_detail",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RtsActorGlyphRole {
    Worker,
    Scout,
    Warden,
    Striker,
    CommandCore,
    FluxRelay,
    Beacon,
    Resource,
    MapDetail,
}

impl RtsActorGlyphRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Worker => "worker",
            Self::Scout => "scout",
            Self::Warden => "warden",
            Self::Striker => "striker",
            Self::CommandCore => "command_core",
            Self::FluxRelay => "flux_relay",
            Self::Beacon => "beacon",
            Self::Resource => "resource",
            Self::MapDetail => "map_detail",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RtsActorGlyphBody {
    Unit,
    Structure,
    SpawnPad,
    ResourceBloom,
    ObjectiveBeacon,
    TerrainRidge,
    FluxVent,
    LaneMarker,
    BeaconRing,
    ExpansionMarker,
}

impl RtsActorGlyphBody {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unit => "unit",
            Self::Structure => "structure",
            Self::SpawnPad => "spawn_pad",
            Self::ResourceBloom => "resource_bloom",
            Self::ObjectiveBeacon => "objective_beacon",
            Self::TerrainRidge => "terrain_ridge",
            Self::FluxVent => "flux_vent",
            Self::LaneMarker => "lane_marker",
            Self::BeaconRing => "beacon_ring",
            Self::ExpansionMarker => "expansion_marker",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RtsActorGlyphAccent {
    None,
    WorkerCargo,
    ScoutSensor,
    WardenShield,
    StrikerBlade,
    CommandSpire,
    RelayMast,
    BeaconCore,
    ResourceGlint,
    OwnerStripe,
    RidgeLip,
    VentGlow,
    LaneCross,
    RingFrame,
    ExpansionCross,
}

impl RtsActorGlyphAccent {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::WorkerCargo => "worker_cargo",
            Self::ScoutSensor => "scout_sensor",
            Self::WardenShield => "warden_shield",
            Self::StrikerBlade => "striker_blade",
            Self::CommandSpire => "command_spire",
            Self::RelayMast => "relay_mast",
            Self::BeaconCore => "beacon_core",
            Self::ResourceGlint => "resource_glint",
            Self::OwnerStripe => "owner_stripe",
            Self::RidgeLip => "ridge_lip",
            Self::VentGlow => "vent_glow",
            Self::LaneCross => "lane_cross",
            Self::RingFrame => "ring_frame",
            Self::ExpansionCross => "expansion_cross",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsActorGlyphProfile {
    pub contract_version: String,
    pub body: RtsActorGlyphBody,
    pub accent: RtsActorGlyphAccent,
    pub footprint_width_cells: u8,
    pub footprint_height_cells: u8,
    pub selection_ring: bool,
    pub shadow: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsActorPresentationProfile {
    pub contract_version: String,
    pub map_id: String,
    pub rule_id: String,
    pub label: String,
    pub color_role: RtsActorColorRole,
    pub glyph_role: RtsActorGlyphRole,
    pub structure: bool,
    pub selectable: bool,
    pub health_bar_width: u8,
    pub draw_priority: u8,
    pub glyph: RtsActorGlyphProfile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RtsVisualTelemetryColorRole {
    Health,
    Mana,
    Attack,
    Confirm,
    ActionTrail,
    NpcAction,
}

impl RtsVisualTelemetryColorRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Health => "health",
            Self::Mana => "mana",
            Self::Attack => "attack",
            Self::Confirm => "confirm",
            Self::ActionTrail => "action_trail",
            Self::NpcAction => "npc_action",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsUnitStatusTelemetry {
    pub tile: RtsTile,
    pub role_badge: String,
    pub health_percent: u8,
    pub shield_percent: u8,
    pub role_color: RtsVisualTelemetryColorRole,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsTacticalTrackProfile {
    pub from_tile: RtsTile,
    pub to_tile: RtsTile,
    pub color_role: RtsVisualTelemetryColorRole,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RtsFirstContactVisualTelemetryProfile {
    pub contract_version: String,
    pub map_id: String,
    pub unit_statuses: Vec<RtsUnitStatusTelemetry>,
    pub tactical_tracks: Vec<RtsTacticalTrackProfile>,
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

    pub fn rule_count_by_kind(&self, kind: RtsRuleKind) -> usize {
        self.rules.iter().filter(|rule| rule.kind == kind).count()
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

pub fn first_contact_opening_loop_profile() -> RtsOpeningLoopProfile {
    RtsOpeningLoopProfile {
        contract_version: TRNM_RTS_DATA_FIRST_CONTACT_OPENING_PROFILE_CONTRACT.to_string(),
        map_id: "first_contact_basin".to_string(),
        flux_bank: 340,
        worker_cargo: 8,
        worker_capacity: 12,
        relay_build_progress: 58,
        beacon_capture_progress: 42,
        worker_train_progress: 76,
        scout_train_progress: 34,
        active_beacon_tile: RtsTile::new(16, 9),
        active_relay_tile: RtsTile::new(11, 8),
        opening_actions: [
            "worker_harvest_flux",
            "build_flux_relay",
            "train_worker",
            "train_horizon_scout",
            "secure_flux_beacon",
        ]
        .iter()
        .map(|action| (*action).to_string())
        .collect(),
    }
}

pub fn first_contact_command_feedback_profile() -> RtsCommandFeedbackProfile {
    RtsCommandFeedbackProfile {
        contract_version: TRNM_RTS_DATA_FIRST_CONTACT_COMMAND_FEEDBACK_CONTRACT.to_string(),
        map_id: "first_contact_basin".to_string(),
        selected_group: "GROUP 1".to_string(),
        active_order: "SECURE BEACON".to_string(),
        target_tile: RtsTile::new(16, 9),
        blocked_tile: RtsTile::new(15, 16),
        blocked_reason: "MID VENT BLOCKED".to_string(),
        queued_before: 2,
        queued_after: 3,
        command_ack_progress: 86,
        cooldown_progress: 32,
    }
}

pub fn first_contact_player_startup_profiles() -> Vec<RtsPlayerStartupProfile> {
    let opening = first_contact_opening_loop_profile();
    [
        (
            "Multi0",
            "horizon",
            "multi0",
            RtsTile::new(8, 8),
            "trnm.horizon.scout",
        ),
        (
            "Multi1",
            "forge",
            "multi1",
            RtsTile::new(25, 25),
            "trnm.forge.warden",
        ),
        (
            "Multi2",
            "horizon",
            "multi2",
            RtsTile::new(25, 8),
            "trnm.horizon.scout",
        ),
        (
            "Multi3",
            "forge",
            "multi3",
            RtsTile::new(8, 25),
            "trnm.forge.warden",
        ),
    ]
    .into_iter()
    .map(
        |(player_id, faction, actor_id_prefix, spawn_tile, faction_unit_rule_id)| {
            RtsPlayerStartupProfile {
                contract_version: TRNM_RTS_DATA_FIRST_CONTACT_PLAYER_STARTUP_CONTRACT.to_string(),
                map_id: "first_contact_basin".to_string(),
                player_id: player_id.to_string(),
                faction: faction.to_string(),
                actor_id_prefix: actor_id_prefix.to_string(),
                spawn_tile,
                command_core_rule_id: "trnm.command.core".to_string(),
                worker_rule_id: "trnm.worker".to_string(),
                faction_unit_rule_id: faction_unit_rule_id.to_string(),
                opening_harvest_tile: RtsTile::new(10, 10),
                opening_relay_tile: opening.active_relay_tile,
                opening_beacon_tile: opening.active_beacon_tile,
            }
        },
    )
    .collect()
}

pub fn first_contact_actor_presentation_profiles() -> Vec<RtsActorPresentationProfile> {
    [
        (
            "mpspawn",
            RtsActorColorRole::MapDetail,
            RtsActorGlyphRole::MapDetail,
            false,
            false,
            10,
            11,
            RtsActorGlyphBody::SpawnPad,
            RtsActorGlyphAccent::OwnerStripe,
            3,
            3,
            false,
            true,
        ),
        (
            "trnm.worker",
            RtsActorColorRole::Worker,
            RtsActorGlyphRole::Worker,
            false,
            true,
            18,
            42,
            RtsActorGlyphBody::Unit,
            RtsActorGlyphAccent::WorkerCargo,
            1,
            1,
            true,
            true,
        ),
        (
            "trnm.horizon.scout",
            RtsActorColorRole::Scout,
            RtsActorGlyphRole::Scout,
            false,
            true,
            18,
            45,
            RtsActorGlyphBody::Unit,
            RtsActorGlyphAccent::ScoutSensor,
            1,
            1,
            true,
            true,
        ),
        (
            "trnm.forge.warden",
            RtsActorColorRole::Warden,
            RtsActorGlyphRole::Warden,
            false,
            true,
            20,
            46,
            RtsActorGlyphBody::Unit,
            RtsActorGlyphAccent::WardenShield,
            1,
            1,
            true,
            true,
        ),
        (
            "trnm.striker",
            RtsActorColorRole::Striker,
            RtsActorGlyphRole::Striker,
            false,
            true,
            18,
            47,
            RtsActorGlyphBody::Unit,
            RtsActorGlyphAccent::StrikerBlade,
            1,
            1,
            true,
            true,
        ),
        (
            "trnm.command.core",
            RtsActorColorRole::CommandCore,
            RtsActorGlyphRole::CommandCore,
            true,
            true,
            36,
            70,
            RtsActorGlyphBody::Structure,
            RtsActorGlyphAccent::CommandSpire,
            2,
            2,
            true,
            true,
        ),
        (
            "trnm.flux.relay",
            RtsActorColorRole::FluxRelay,
            RtsActorGlyphRole::FluxRelay,
            true,
            true,
            30,
            62,
            RtsActorGlyphBody::Structure,
            RtsActorGlyphAccent::RelayMast,
            2,
            2,
            true,
            true,
        ),
        (
            "trnm.flux.beacon",
            RtsActorColorRole::Objective,
            RtsActorGlyphRole::Beacon,
            true,
            true,
            28,
            58,
            RtsActorGlyphBody::ObjectiveBeacon,
            RtsActorGlyphAccent::BeaconCore,
            2,
            2,
            true,
            true,
        ),
        (
            "trnm.flux.bloom",
            RtsActorColorRole::Resource,
            RtsActorGlyphRole::Resource,
            false,
            false,
            12,
            20,
            RtsActorGlyphBody::ResourceBloom,
            RtsActorGlyphAccent::ResourceGlint,
            2,
            2,
            false,
            true,
        ),
        (
            "trnm.map.ridge",
            RtsActorColorRole::MapDetail,
            RtsActorGlyphRole::MapDetail,
            false,
            false,
            10,
            10,
            RtsActorGlyphBody::TerrainRidge,
            RtsActorGlyphAccent::RidgeLip,
            2,
            1,
            false,
            false,
        ),
        (
            "trnm.flux.vent",
            RtsActorColorRole::MapDetail,
            RtsActorGlyphRole::MapDetail,
            false,
            false,
            10,
            10,
            RtsActorGlyphBody::FluxVent,
            RtsActorGlyphAccent::VentGlow,
            1,
            1,
            false,
            false,
        ),
        (
            "trnm.lane.marker",
            RtsActorColorRole::MapDetail,
            RtsActorGlyphRole::MapDetail,
            false,
            false,
            10,
            10,
            RtsActorGlyphBody::LaneMarker,
            RtsActorGlyphAccent::LaneCross,
            3,
            3,
            false,
            false,
        ),
        (
            "trnm.beacon.ring",
            RtsActorColorRole::MapDetail,
            RtsActorGlyphRole::MapDetail,
            false,
            false,
            10,
            10,
            RtsActorGlyphBody::BeaconRing,
            RtsActorGlyphAccent::RingFrame,
            2,
            2,
            false,
            false,
        ),
        (
            "trnm.expansion.marker",
            RtsActorColorRole::MapDetail,
            RtsActorGlyphRole::MapDetail,
            false,
            false,
            10,
            10,
            RtsActorGlyphBody::ExpansionMarker,
            RtsActorGlyphAccent::ExpansionCross,
            1,
            1,
            false,
            false,
        ),
    ]
    .into_iter()
    .map(
        |(
            rule_id,
            color_role,
            glyph_role,
            structure,
            selectable,
            health_bar_width,
            draw_priority,
            glyph_body,
            glyph_accent,
            footprint_width_cells,
            footprint_height_cells,
            selection_ring,
            shadow,
        )| {
            let label = first_contact_rules()
                .into_iter()
                .find(|rule| rule.id == rule_id)
                .map(|rule| rule.label)
                .unwrap_or_else(|| rule_id.to_string());
            RtsActorPresentationProfile {
                contract_version: TRNM_RTS_DATA_FIRST_CONTACT_ACTOR_PRESENTATION_CONTRACT
                    .to_string(),
                map_id: "first_contact_basin".to_string(),
                rule_id: rule_id.to_string(),
                label,
                color_role,
                glyph_role,
                structure,
                selectable,
                health_bar_width,
                draw_priority,
                glyph: RtsActorGlyphProfile {
                    contract_version: TRNM_RTS_DATA_FIRST_CONTACT_ACTOR_GLYPH_CONTRACT.to_string(),
                    body: glyph_body,
                    accent: glyph_accent,
                    footprint_width_cells,
                    footprint_height_cells,
                    selection_ring,
                    shadow,
                },
            }
        },
    )
    .collect()
}

pub fn first_contact_actor_presentation_profile(
    rule_id: &str,
) -> Option<RtsActorPresentationProfile> {
    first_contact_actor_presentation_profiles()
        .into_iter()
        .find(|profile| profile.rule_id == rule_id)
}

pub fn first_contact_visual_telemetry_profile() -> RtsFirstContactVisualTelemetryProfile {
    RtsFirstContactVisualTelemetryProfile {
        contract_version: TRNM_RTS_DATA_FIRST_CONTACT_VISUAL_TELEMETRY_CONTRACT.to_string(),
        map_id: "first_contact_basin".to_string(),
        unit_statuses: vec![
            RtsUnitStatusTelemetry {
                tile: RtsTile::new(8, 8),
                role_badge: "W".to_string(),
                health_percent: 82,
                shield_percent: 44,
                role_color: RtsVisualTelemetryColorRole::Health,
            },
            RtsUnitStatusTelemetry {
                tile: RtsTile::new(25, 8),
                role_badge: "S".to_string(),
                health_percent: 76,
                shield_percent: 68,
                role_color: RtsVisualTelemetryColorRole::Mana,
            },
            RtsUnitStatusTelemetry {
                tile: RtsTile::new(25, 25),
                role_badge: "R".to_string(),
                health_percent: 64,
                shield_percent: 22,
                role_color: RtsVisualTelemetryColorRole::Attack,
            },
            RtsUnitStatusTelemetry {
                tile: RtsTile::new(8, 25),
                role_badge: "G".to_string(),
                health_percent: 91,
                shield_percent: 55,
                role_color: RtsVisualTelemetryColorRole::Confirm,
            },
        ],
        tactical_tracks: vec![
            RtsTacticalTrackProfile {
                from_tile: RtsTile::new(8, 8),
                to_tile: RtsTile::new(12, 16),
                color_role: RtsVisualTelemetryColorRole::ActionTrail,
            },
            RtsTacticalTrackProfile {
                from_tile: RtsTile::new(25, 25),
                to_tile: RtsTile::new(21, 16),
                color_role: RtsVisualTelemetryColorRole::NpcAction,
            },
            RtsTacticalTrackProfile {
                from_tile: RtsTile::new(25, 8),
                to_tile: RtsTile::new(16, 12),
                color_role: RtsVisualTelemetryColorRole::ActionTrail,
            },
            RtsTacticalTrackProfile {
                from_tile: RtsTile::new(8, 25),
                to_tile: RtsTile::new(16, 21),
                color_role: RtsVisualTelemetryColorRole::NpcAction,
            },
            RtsTacticalTrackProfile {
                from_tile: RtsTile::new(11, 8),
                to_tile: RtsTile::new(16, 9),
                color_role: RtsVisualTelemetryColorRole::ActionTrail,
            },
            RtsTacticalTrackProfile {
                from_tile: RtsTile::new(22, 25),
                to_tile: RtsTile::new(16, 24),
                color_role: RtsVisualTelemetryColorRole::NpcAction,
            },
        ],
    }
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

    #[test]
    fn first_contact_opening_profile_binds_real_map_rules() {
        let map = first_contact_basin_map();
        let opening = first_contact_opening_loop_profile();
        assert_eq!(
            opening.contract_version,
            TRNM_RTS_DATA_FIRST_CONTACT_OPENING_PROFILE_CONTRACT
        );
        assert_eq!(opening.map_id, map.map_id);
        assert!(map.bounds.contains(opening.active_beacon_tile));
        assert!(map.bounds.contains(opening.active_relay_tile));
        assert!(map.actors.iter().any(|actor| {
            actor.rule_id == "trnm.flux.beacon" && actor.tile == opening.active_beacon_tile
        }));
        assert!(map.actors.iter().any(|actor| {
            actor.rule_id == "trnm.expansion.marker" && actor.tile == opening.active_relay_tile
        }));
        assert!(map.rules.iter().any(|rule| {
            rule.id == "trnm.worker"
                && rule.cost == 200
                && opening.flux_bank >= rule.cost
                && rule.build_duration == Some(100)
        }));
        assert!(map.rules.iter().any(|rule| {
            rule.id == "trnm.horizon.scout"
                && rule.speed == Some(92)
                && opening.scout_train_progress >= 30
        }));
        assert_eq!(
            opening.opening_actions,
            vec![
                "worker_harvest_flux",
                "build_flux_relay",
                "train_worker",
                "train_horizon_scout",
                "secure_flux_beacon",
            ]
        );
    }

    #[test]
    fn first_contact_command_feedback_profile_targets_playable_tiles() {
        let map = first_contact_basin_map();
        let feedback = first_contact_command_feedback_profile();
        assert_eq!(
            feedback.contract_version,
            TRNM_RTS_DATA_FIRST_CONTACT_COMMAND_FEEDBACK_CONTRACT
        );
        assert_eq!(feedback.map_id, map.map_id);
        assert!(map.bounds.contains(feedback.target_tile));
        assert!(map.bounds.contains(feedback.blocked_tile));
        assert!(map.actors.iter().any(|actor| {
            actor.rule_id == "trnm.flux.beacon" && actor.tile == feedback.target_tile
        }));
        assert_eq!(feedback.selected_group, "GROUP 1");
        assert!(feedback.queued_after > feedback.queued_before);
        assert!(feedback.command_ack_progress > feedback.cooldown_progress);
    }

    #[test]
    fn first_contact_player_startups_bind_spawn_players_and_rules() {
        let map = first_contact_basin_map();
        let startups = first_contact_player_startup_profiles();
        assert_eq!(startups.len(), 4);
        assert!(startups.iter().all(|startup| {
            startup.contract_version == TRNM_RTS_DATA_FIRST_CONTACT_PLAYER_STARTUP_CONTRACT
                && startup.map_id == map.map_id
                && map.players.iter().any(|player| {
                    player.id == startup.player_id
                        && player.playable
                        && player.faction == startup.faction
                })
                && map.actors.iter().any(|actor| {
                    actor.rule_id == "mpspawn"
                        && actor.owner == startup.player_id
                        && actor.tile == startup.spawn_tile
                })
                && map
                    .rules
                    .iter()
                    .any(|rule| rule.id == startup.command_core_rule_id)
                && map
                    .rules
                    .iter()
                    .any(|rule| rule.id == startup.worker_rule_id)
                && map
                    .rules
                    .iter()
                    .any(|rule| rule.id == startup.faction_unit_rule_id)
        }));
        let multi0 = startups
            .iter()
            .find(|startup| startup.player_id == "Multi0")
            .expect("Multi0 startup exists");
        assert_eq!(multi0.spawn_tile, RtsTile::new(8, 8));
        assert_eq!(multi0.opening_harvest_tile, RtsTile::new(10, 10));
        assert_eq!(
            multi0.opening_beacon_tile,
            first_contact_opening_loop_profile().active_beacon_tile
        );
        assert_eq!(
            multi0.opening_relay_tile,
            first_contact_opening_loop_profile().active_relay_tile
        );
    }

    #[test]
    fn first_contact_actor_presentations_bind_visible_rules() {
        let map = first_contact_basin_map();
        let profiles = first_contact_actor_presentation_profiles();
        assert!(profiles.len() >= 8);
        assert!(profiles.iter().all(|profile| {
            profile.contract_version == TRNM_RTS_DATA_FIRST_CONTACT_ACTOR_PRESENTATION_CONTRACT
                && profile.map_id == map.map_id
                && map.rules.iter().any(|rule| rule.id == profile.rule_id)
                && profile.health_bar_width >= 10
                && profile.glyph.contract_version
                    == TRNM_RTS_DATA_FIRST_CONTACT_ACTOR_GLYPH_CONTRACT
                && profile.glyph.footprint_width_cells > 0
                && profile.glyph.footprint_height_cells > 0
        }));
        let core = first_contact_actor_presentation_profile("trnm.command.core")
            .expect("command core presentation exists");
        assert_eq!(core.color_role, RtsActorColorRole::CommandCore);
        assert_eq!(core.glyph_role, RtsActorGlyphRole::CommandCore);
        assert!(core.structure);
        assert_eq!(core.health_bar_width, 36);
        assert_eq!(core.glyph.body, RtsActorGlyphBody::Structure);
        assert_eq!(core.glyph.accent, RtsActorGlyphAccent::CommandSpire);
        assert_eq!(core.glyph.footprint_width_cells, 2);
        let worker = first_contact_actor_presentation_profile("trnm.worker")
            .expect("worker presentation exists");
        assert_eq!(worker.color_role.as_str(), "worker");
        assert_eq!(worker.glyph_role.as_str(), "worker");
        assert_eq!(worker.glyph.body.as_str(), "unit");
        assert_eq!(worker.glyph.accent.as_str(), "worker_cargo");
        assert!(worker.glyph.selection_ring);
        assert!(worker.selectable);
        assert!(!worker.structure);
        assert!(
            first_contact_actor_presentation_profile("trnm.flux.beacon").is_some_and(|profile| {
                profile.structure
                    && profile.color_role.as_str() == "objective"
                    && profile.glyph.body == RtsActorGlyphBody::ObjectiveBeacon
            })
        );
        assert!(
            first_contact_actor_presentation_profile("mpspawn").is_some_and(|profile| {
                profile.glyph.body == RtsActorGlyphBody::SpawnPad
                    && profile.glyph.accent == RtsActorGlyphAccent::OwnerStripe
            })
        );
    }

    #[test]
    fn first_contact_visual_telemetry_binds_playable_overlay_tiles() {
        let map = first_contact_basin_map();
        let profile = first_contact_visual_telemetry_profile();
        assert_eq!(
            profile.contract_version,
            TRNM_RTS_DATA_FIRST_CONTACT_VISUAL_TELEMETRY_CONTRACT
        );
        assert_eq!(profile.map_id, map.map_id);
        assert_eq!(profile.unit_statuses.len(), 4);
        assert_eq!(profile.tactical_tracks.len(), 6);
        assert!(profile.unit_statuses.iter().all(|status| {
            map.bounds.contains(status.tile)
                && !status.role_badge.is_empty()
                && status.health_percent <= 100
                && status.shield_percent <= 100
        }));
        assert!(profile.tactical_tracks.iter().all(|track| {
            map.bounds.contains(track.from_tile) && map.bounds.contains(track.to_tile)
        }));
        assert!(profile.unit_statuses.iter().any(|status| {
            status.tile == RtsTile::new(8, 8)
                && status.role_badge == "W"
                && status.role_color.as_str() == "health"
        }));
        let opening = first_contact_opening_loop_profile();
        assert!(profile.tactical_tracks.iter().any(|track| {
            track.from_tile == opening.active_relay_tile
                && track.to_tile == opening.active_beacon_tile
                && track.color_role == RtsVisualTelemetryColorRole::ActionTrail
        }));
    }
}
