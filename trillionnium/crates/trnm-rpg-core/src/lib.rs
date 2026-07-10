//! Minimal RPG state consumed by the playable campaign.
//!
//! Historical CEX/world fixtures intentionally live outside the game product
//! workspace. This crate owns only character attributes and typed equipment.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};

pub const MIRROR_SQUARE_ROOM: &str = "mirror_square";
pub const MENTOR_HALL_ROOM: &str = "mentor_hall";
pub const EXPEDITION_GATE_ROOM: &str = "expedition_gate";
pub const RELAY_QUARTER_ROOM: &str = "relay_quarter";

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GrowthStat {
    #[default]
    Physique,
    Force,
    Agility,
    Insight,
    Resolve,
    Craft,
    Commerce,
}

impl GrowthStat {
    pub fn next(self) -> Self {
        match self {
            Self::Physique => Self::Force,
            Self::Force => Self::Agility,
            Self::Agility => Self::Insight,
            Self::Insight => Self::Resolve,
            Self::Resolve => Self::Craft,
            Self::Craft => Self::Commerce,
            Self::Commerce => Self::Physique,
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::Physique => "Physique",
            Self::Force => "Force",
            Self::Agility => "Agility",
            Self::Insight => "Insight",
            Self::Resolve => "Resolve",
            Self::Craft => "Craft",
            Self::Commerce => "Commerce",
        }
    }

    pub fn apply(self, attributes: &mut TrillionniumAttributes, points: u16) {
        let target = match self {
            Self::Physique => &mut attributes.physique,
            Self::Force => &mut attributes.force,
            Self::Agility => &mut attributes.agility,
            Self::Insight => &mut attributes.insight,
            Self::Resolve => &mut attributes.resolve,
            Self::Craft => &mut attributes.craft,
            Self::Commerce => &mut attributes.commerce,
        };
        *target = target.saturating_add(points);
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildPath {
    #[default]
    Unformed,
    Vanguard,
    Windrunner,
    Artificer,
}

impl BuildPath {
    pub fn display_name(self) -> &'static str {
        match self {
            Self::Unformed => "Unformed",
            Self::Vanguard => "Vanguard",
            Self::Windrunner => "Windrunner",
            Self::Artificer => "Artificer",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CharacterOrigin {
    #[default]
    Balanced,
    Artisan,
    Scout,
}

impl CharacterOrigin {
    pub fn next(self) -> Self {
        match self {
            Self::Balanced => Self::Artisan,
            Self::Artisan => Self::Scout,
            Self::Scout => Self::Balanced,
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::Balanced => "Mirror Ward",
            Self::Artisan => "Workshop Kin",
            Self::Scout => "Signal Runner",
        }
    }

    pub fn starter_skill(self) -> &'static str {
        match self {
            Self::Balanced => "iron_guard",
            Self::Artisan => "relay_overcharge",
            Self::Scout => "wind_step",
        }
    }

    pub fn apply(self, attributes: &mut TrillionniumAttributes) {
        match self {
            Self::Balanced => {
                attributes.physique = attributes.physique.saturating_add(2);
                attributes.resolve = attributes.resolve.saturating_add(2);
            }
            Self::Artisan => {
                attributes.craft = attributes.craft.saturating_add(4);
                attributes.insight = attributes.insight.saturating_add(1);
            }
            Self::Scout => {
                attributes.agility = attributes.agility.saturating_add(4);
                attributes.insight = attributes.insight.saturating_add(1);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MasteryChallenge {
    VanguardStand,
    WindrunnerCircuit,
    ArtificerCommission,
}

impl MasteryChallenge {
    pub fn for_path(path: BuildPath) -> Option<Self> {
        match path {
            BuildPath::Unformed => None,
            BuildPath::Vanguard => Some(Self::VanguardStand),
            BuildPath::Windrunner => Some(Self::WindrunnerCircuit),
            BuildPath::Artificer => Some(Self::ArtificerCommission),
        }
    }

    pub fn title(self) -> BuildTitle {
        match self {
            Self::VanguardStand => BuildTitle::GateWarden,
            Self::WindrunnerCircuit => BuildTitle::RelayRunner,
            Self::ArtificerCommission => BuildTitle::ForgeMaster,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind", content = "value")]
pub enum EquipmentAffixCondition {
    Origin(CharacterOrigin),
    BuildPath(BuildPath),
    MasteryTitle(BuildTitle),
}

impl EquipmentAffixCondition {
    pub fn active(
        self,
        origin: CharacterOrigin,
        build_path: BuildPath,
        title: Option<BuildTitle>,
    ) -> bool {
        match self {
            Self::Origin(expected) => origin == expected,
            Self::BuildPath(expected) => build_path == expected,
            Self::MasteryTitle(expected) => title == Some(expected),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildTitle {
    GateWarden,
    RelayRunner,
    ForgeMaster,
}

impl BuildTitle {
    pub fn display_name(self) -> &'static str {
        match self {
            Self::GateWarden => "Gate Warden",
            Self::RelayRunner => "Relay Runner",
            Self::ForgeMaster => "Forge Master",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EncounterAction {
    Attack,
    Defend,
    UseItem,
    Withdraw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EncounterOutcome {
    Victory,
    Defeat,
    Withdrawn,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RpgEncounterState {
    pub encounter_id: String,
    pub round: u8,
    pub player_hp: i64,
    pub player_max_hp: i64,
    pub enemy_hp: i64,
    pub enemy_max_hp: i64,
    pub outcome: Option<EncounterOutcome>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncounterTurn {
    pub item_consumed: bool,
    pub outcome: Option<EncounterOutcome>,
}

impl RpgEncounterState {
    pub fn signal_road_ambush(attributes: &TrillionniumAttributes) -> Self {
        let player_max_hp = attributes.derived_stats().max_hp;
        Self {
            encounter_id: "signal_road_ambush".to_string(),
            round: 0,
            player_hp: player_max_hp,
            player_max_hp,
            enemy_hp: 135,
            enemy_max_hp: 135,
            outcome: None,
        }
    }

    pub fn advance(
        &mut self,
        attributes: &TrillionniumAttributes,
        action: EncounterAction,
        item_available: bool,
    ) -> Result<EncounterTurn, String> {
        if self.outcome.is_some() {
            return Err("rpg_encounter_already_terminal".to_string());
        }
        if action == EncounterAction::UseItem && !item_available {
            return Err("rpg_encounter_item_missing".to_string());
        }
        self.round = self.round.saturating_add(1);
        let mut item_consumed = false;
        let defending = action == EncounterAction::Defend;
        match action {
            EncounterAction::Attack => {
                self.enemy_hp -= 12 + i64::from(attributes.force) * 2;
            }
            EncounterAction::Defend => {
                self.enemy_hp -= 4 + i64::from(attributes.insight / 3);
            }
            EncounterAction::UseItem => {
                item_consumed = true;
                self.player_hp = (self.player_hp + 55).min(self.player_max_hp);
            }
            EncounterAction::Withdraw => {
                self.outcome = Some(EncounterOutcome::Withdrawn);
            }
        }
        if self.outcome.is_none() && self.enemy_hp <= 0 {
            self.outcome = Some(EncounterOutcome::Victory);
        }
        if self.outcome.is_none() {
            let mitigation = if defending {
                10 + i64::from(attributes.resolve / 2)
            } else {
                i64::from(attributes.physique / 8)
            };
            let enemy_damage = 18 + i64::from(self.round % 3) * 4;
            self.player_hp -= (enemy_damage - mitigation).max(1);
            if self.player_hp <= 0 {
                self.outcome = Some(EncounterOutcome::Defeat);
            }
        }
        Ok(EncounterTurn {
            item_consumed,
            outcome: self.outcome,
        })
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FactionRank {
    #[default]
    Outsider,
    Initiate,
    Disciple,
    Envoy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationshipAction {
    Talk,
    Train,
    Spar,
    CompleteMission,
    Betray,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NpcRelationship {
    pub npc_id: String,
    pub faction_id: String,
    pub trust: i16,
    pub interactions: u16,
    pub recruited: bool,
}

impl NpcRelationship {
    pub fn new(npc_id: impl Into<String>, faction_id: impl Into<String>) -> Self {
        Self {
            npc_id: npc_id.into(),
            faction_id: faction_id.into(),
            trust: 0,
            interactions: 0,
            recruited: false,
        }
    }

    pub fn apply(&mut self, action: RelationshipAction) -> i16 {
        let delta = match action {
            RelationshipAction::Talk => 3,
            RelationshipAction::Train => 4,
            RelationshipAction::Spar => 5,
            RelationshipAction::CompleteMission => 7,
            RelationshipAction::Betray => -20,
        };
        self.trust = self.trust.saturating_add(delta).clamp(-100, 100);
        self.interactions = self.interactions.saturating_add(1);
        delta
    }

    pub fn can_recruit(&self, required_trust: i16) -> bool {
        !self.recruited && self.trust >= required_trust
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SparringAction {
    Strike,
    Guard,
    InnerPower,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SparringOutcome {
    Victory,
    Defeat,
    Draw,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SparringReport {
    pub outcome: SparringOutcome,
    pub rounds: u8,
    pub player_hp: i64,
    pub mentor_hp: i64,
    pub inner_energy: i64,
}

pub fn resolve_mentor_sparring(
    attributes: &TrillionniumAttributes,
    actions: &[SparringAction],
) -> SparringReport {
    let stats = attributes.derived_stats();
    let mut player_hp = stats.max_hp;
    let mut mentor_hp = 150_i64;
    let mut inner_energy = stats.inner_energy;
    let mut rounds = 0_u8;
    for action in actions.iter().copied().take(8) {
        if player_hp <= 0 || mentor_hp <= 0 {
            break;
        }
        rounds = rounds.saturating_add(1);
        let guarding = action == SparringAction::Guard;
        let damage = match action {
            SparringAction::Strike => 18 + i64::from(attributes.force),
            SparringAction::Guard => 7 + i64::from(attributes.insight / 2),
            SparringAction::InnerPower if inner_energy >= 24 => {
                inner_energy -= 24;
                30 + i64::from(attributes.resolve + attributes.insight)
            }
            SparringAction::InnerPower => 5,
        };
        mentor_hp -= damage;
        if mentor_hp > 0 {
            let mentor_damage = 24 + i64::from(rounds % 3) * 3;
            let mitigation = if guarding {
                15 + i64::from(attributes.resolve / 2)
            } else {
                i64::from(attributes.physique / 6)
            };
            player_hp -= (mentor_damage - mitigation).max(1);
        }
    }
    let outcome = if mentor_hp <= 0 && player_hp > 0 {
        SparringOutcome::Victory
    } else if player_hp <= 0 {
        SparringOutcome::Defeat
    } else {
        SparringOutcome::Draw
    };
    SparringReport {
        outcome,
        rounds,
        player_hp: player_hp.max(0),
        mentor_hp: mentor_hp.max(0),
        inner_energy,
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum WorldRouteBlockedReason {
    UnknownStart {
        room_id: String,
    },
    UnknownDestination {
        room_id: String,
    },
    LockedRoom {
        room_id: String,
        required_flag: String,
    },
    Unreachable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldRoutePlan {
    pub start_room_id: String,
    pub destination_room_id: String,
    pub path: Vec<String>,
    pub next_exit: Option<WorldExit>,
    pub blocked_reason: Option<WorldRouteBlockedReason>,
}

impl WorldRoutePlan {
    pub fn reachable(&self) -> bool {
        self.blocked_reason.is_none()
    }
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

    pub fn shortest_route(
        &self,
        start: &str,
        destination: &str,
        world_flags: &BTreeSet<String>,
    ) -> WorldRoutePlan {
        if !self.rooms.contains_key(start) {
            return WorldRoutePlan {
                start_room_id: start.to_string(),
                destination_room_id: destination.to_string(),
                path: Vec::new(),
                next_exit: None,
                blocked_reason: Some(WorldRouteBlockedReason::UnknownStart {
                    room_id: start.to_string(),
                }),
            };
        }
        if !self.rooms.contains_key(destination) {
            return WorldRoutePlan {
                start_room_id: start.to_string(),
                destination_room_id: destination.to_string(),
                path: Vec::new(),
                next_exit: None,
                blocked_reason: Some(WorldRouteBlockedReason::UnknownDestination {
                    room_id: destination.to_string(),
                }),
            };
        }
        if start == destination {
            return WorldRoutePlan {
                start_room_id: start.to_string(),
                destination_room_id: destination.to_string(),
                path: vec![start.to_string()],
                next_exit: None,
                blocked_reason: None,
            };
        }

        let accessible = self.bfs_path(start, destination, |room| {
            self.can_enter(room, world_flags).is_ok()
        });
        if let Some(path) = accessible {
            let next_exit = path.get(1).and_then(|next| {
                self.exits
                    .iter()
                    .find(|exit| exit.from == start && exit.to == *next)
                    .cloned()
            });
            return WorldRoutePlan {
                start_room_id: start.to_string(),
                destination_room_id: destination.to_string(),
                path,
                next_exit,
                blocked_reason: None,
            };
        }

        let structural = self.bfs_path(start, destination, |_| true);
        let blocked_reason = structural
            .as_ref()
            .and_then(|path| {
                path.iter().skip(1).find_map(|room_id| {
                    self.rooms
                        .get(room_id)
                        .and_then(|room| room.unlock_flag.as_ref().map(|flag| (room_id, flag)))
                        .filter(|(_, flag)| !world_flags.contains(*flag))
                })
            })
            .map(
                |(room_id, required_flag)| WorldRouteBlockedReason::LockedRoom {
                    room_id: room_id.clone(),
                    required_flag: required_flag.clone(),
                },
            )
            .unwrap_or(WorldRouteBlockedReason::Unreachable);
        WorldRoutePlan {
            start_room_id: start.to_string(),
            destination_room_id: destination.to_string(),
            path: Vec::new(),
            next_exit: None,
            blocked_reason: Some(blocked_reason),
        }
    }

    pub fn ordered_task_route(
        &self,
        start: &str,
        waypoints: &[String],
        world_flags: &BTreeSet<String>,
    ) -> WorldRoutePlan {
        let destination = waypoints
            .last()
            .cloned()
            .unwrap_or_else(|| start.to_string());
        let mut combined = vec![start.to_string()];
        let mut current = start.to_string();
        for waypoint in waypoints {
            let segment = self.shortest_route(&current, waypoint, world_flags);
            if !segment.reachable() {
                return WorldRoutePlan {
                    start_room_id: start.to_string(),
                    destination_room_id: destination,
                    path: Vec::new(),
                    next_exit: None,
                    blocked_reason: segment.blocked_reason,
                };
            }
            combined.extend(segment.path.into_iter().skip(1));
            current = waypoint.clone();
        }
        let next_exit = combined.get(1).and_then(|next| {
            self.exits
                .iter()
                .find(|exit| exit.from == start && exit.to == *next)
                .cloned()
        });
        WorldRoutePlan {
            start_room_id: start.to_string(),
            destination_room_id: destination,
            path: combined,
            next_exit,
            blocked_reason: None,
        }
    }

    fn bfs_path(
        &self,
        start: &str,
        destination: &str,
        can_enter: impl Fn(&str) -> bool,
    ) -> Option<Vec<String>> {
        let mut queue = VecDeque::from([start.to_string()]);
        let mut visited = BTreeSet::from([start.to_string()]);
        let mut previous = BTreeMap::<String, String>::new();
        while let Some(current) = queue.pop_front() {
            if current == destination {
                let mut path = vec![current.clone()];
                let mut cursor = current;
                while let Some(parent) = previous.get(&cursor).cloned() {
                    path.push(parent.clone());
                    cursor = parent;
                }
                path.reverse();
                return Some(path);
            }
            let mut exits = self
                .exits
                .iter()
                .filter(|exit| exit.from == current)
                .collect::<Vec<_>>();
            exits.sort_by(|left, right| {
                left.direction
                    .cmp(&right.direction)
                    .then_with(|| left.to.cmp(&right.to))
            });
            for exit in exits {
                if can_enter(&exit.to) && visited.insert(exit.to.clone()) {
                    previous.insert(exit.to.clone(), current.clone());
                    queue.push_back(exit.to.clone());
                }
            }
        }
        None
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

    #[test]
    fn route_plan_is_lock_aware_stable_and_supports_ordered_waypoints() {
        let graph = mirror_city_world_graph();
        let mut flags = BTreeSet::from(["expedition_gate_open".to_string()]);
        let blocked = graph.shortest_route(MENTOR_HALL_ROOM, RELAY_QUARTER_ROOM, &flags);
        assert_eq!(
            blocked.blocked_reason,
            Some(WorldRouteBlockedReason::LockedRoom {
                room_id: RELAY_QUARTER_ROOM.to_string(),
                required_flag: "signal_road_secured".to_string(),
            })
        );
        flags.insert("signal_road_secured".to_string());
        let waypoints = vec![
            EXPEDITION_GATE_ROOM.to_string(),
            RELAY_QUARTER_ROOM.to_string(),
        ];
        let route = graph.ordered_task_route(MENTOR_HALL_ROOM, &waypoints, &flags);
        assert_eq!(
            route.path.first().map(String::as_str),
            Some(MENTOR_HALL_ROOM)
        );
        assert_eq!(
            route.path.last().map(String::as_str),
            Some(RELAY_QUARTER_ROOM)
        );
        assert_eq!(
            route.next_exit.as_ref().map(|exit| exit.to.as_str()),
            Some(EXPEDITION_GATE_ROOM)
        );
        assert_eq!(
            route,
            graph.ordered_task_route(MENTOR_HALL_ROOM, &waypoints, &flags)
        );
        assert!(matches!(
            graph
                .shortest_route("unknown", RELAY_QUARTER_ROOM, &flags)
                .blocked_reason,
            Some(WorldRouteBlockedReason::UnknownStart { .. })
        ));
        assert!(matches!(
            graph
                .shortest_route(MIRROR_SQUARE_ROOM, "unknown", &flags)
                .blocked_reason,
            Some(WorldRouteBlockedReason::UnknownDestination { .. })
        ));

        let mut disconnected = graph.clone();
        disconnected.rooms.insert(
            "sealed_annex".to_string(),
            WorldRoom {
                id: "sealed_annex".to_string(),
                title: "Sealed Annex".to_string(),
                region_id: "mirror_city".to_string(),
                unlock_flag: None,
            },
        );
        assert_eq!(
            disconnected
                .shortest_route(MIRROR_SQUARE_ROOM, "sealed_annex", &flags)
                .blocked_reason,
            Some(WorldRouteBlockedReason::Unreachable)
        );
    }

    #[test]
    fn origins_masteries_and_affix_conditions_are_typed() {
        let mut balanced = TrillionniumAttributes::default();
        CharacterOrigin::Balanced.apply(&mut balanced);
        let mut scout = TrillionniumAttributes::default();
        CharacterOrigin::Scout.apply(&mut scout);
        assert!(balanced.physique > scout.physique);
        assert!(scout.agility > balanced.agility);
        assert_eq!(
            MasteryChallenge::for_path(BuildPath::Artificer)
                .unwrap()
                .title(),
            BuildTitle::ForgeMaster
        );
        assert!(
            EquipmentAffixCondition::Origin(CharacterOrigin::Scout).active(
                CharacterOrigin::Scout,
                BuildPath::Windrunner,
                None
            )
        );
        assert!(
            !EquipmentAffixCondition::MasteryTitle(BuildTitle::RelayRunner).active(
                CharacterOrigin::Scout,
                BuildPath::Windrunner,
                None
            )
        );
    }

    #[test]
    fn relationships_and_sparring_are_typed_and_deterministic() {
        let mut relation = NpcRelationship::new("street-compass-sifu", "signal-road-school");
        relation.apply(RelationshipAction::Talk);
        relation.apply(RelationshipAction::Train);
        assert_eq!(relation.trust, 7);
        assert!(relation.can_recruit(7));

        let actions = [
            SparringAction::Guard,
            SparringAction::InnerPower,
            SparringAction::Strike,
            SparringAction::InnerPower,
        ];
        let first = resolve_mentor_sparring(&TrillionniumAttributes::default(), &actions);
        let second = resolve_mentor_sparring(&TrillionniumAttributes::default(), &actions);
        assert_eq!(first, second);
        assert!(first.rounds >= 3);
        assert!(first.mentor_hp < 150);
    }

    #[test]
    fn growth_and_encounter_actions_are_typed_and_deterministic() {
        let mut force = TrillionniumAttributes::default();
        GrowthStat::Force.apply(&mut force, 2);
        let mut agility = TrillionniumAttributes::default();
        GrowthStat::Agility.apply(&mut agility, 2);
        assert_eq!(force.force, 13);
        assert_eq!(agility.agility, 14);

        let actions = [
            EncounterAction::Defend,
            EncounterAction::Attack,
            EncounterAction::UseItem,
            EncounterAction::Attack,
            EncounterAction::Attack,
        ];
        let run = |attributes: &TrillionniumAttributes| {
            let mut encounter = RpgEncounterState::signal_road_ambush(attributes);
            for action in actions {
                if encounter.outcome.is_none() {
                    encounter.advance(attributes, action, true).unwrap();
                }
            }
            encounter
        };
        assert_eq!(run(&force), run(&force));
        assert_ne!(run(&force).enemy_hp, run(&agility).enemy_hp);
    }
}
