#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MissionSimV1 {
    pub contract_version: String,
    pub seed: BattleSeedV1,
    pub tick: u64,
    pub phase: BattlePhase,
    #[serde(default)]
    pub objective_index: usize,
    #[serde(default)]
    pub objective_progress_ticks: u32,
    #[serde(default)]
    pub convoy_position: Option<BattleGridPoint>,
    #[serde(default)]
    pub convoy_hp: i64,
    #[serde(default)]
    pub move_intents: BTreeMap<String, MoveIntent>,
    #[serde(default)]
    pub tile_reservations: Vec<TileReservation>,
    pub active_order: Option<RtsFrameOrder>,
    #[serde(default)]
    pub human_enemy_authority: bool,
    #[serde(default)]
    pub enemy_active_order: Option<RtsFrameOrder>,
    #[serde(default)]
    pub queued_orders: VecDeque<RtsFrameOrder>,
    #[serde(default)]
    pub control_groups: BTreeMap<String, BTreeSet<String>>,
    pub last_order_frame: Option<u32>,
    #[serde(default)]
    pub enemy_last_order_frame: Option<u32>,
    pub order_count: u32,
    pub distinct_order_kinds: BTreeSet<String>,
    #[serde(default)]
    pub replay_orders: Vec<SimReplayEntry>,
    pub party: Vec<SimUnit>,
    pub enemies: Vec<SimUnit>,
    pub relay_guard_hp: i64,
    pub relay_guard_max_hp: i64,
    pub relay_capture_ticks: u32,
    pub resources_gathered: u32,
    pub resources_available: u32,
    pub resources_spent: u32,
    #[serde(default)]
    pub resources_generated: u32,
    #[serde(default)]
    pub player_score: u32,
    #[serde(default)]
    pub enemy_score: u32,
    #[serde(default)]
    pub resource_nodes: Vec<ResourceNodeState>,
    #[serde(default)]
    pub structures: Vec<SimStructure>,
    pub reinforcement_wave: u8,
    #[serde(default)]
    pub intel_level: u8,
    #[serde(default)]
    pub recon_bonus_ticks: u32,
    #[serde(default)]
    pub recon_focus: Option<BattleGridPoint>,
    #[serde(default)]
    pub visible_tiles: BTreeSet<BattleGridPoint>,
    #[serde(default)]
    pub explored_tiles: BTreeSet<BattleGridPoint>,
    #[serde(default)]
    pub jobs: Vec<SimJob>,
    #[serde(default)]
    pub authority_job_commands: Vec<AuthorityJobCommandRecord>,
    #[serde(default)]
    pub support_units: Vec<SupportUnit>,
    #[serde(default)]
    pub researched_techs: BTreeSet<String>,
    #[serde(default)]
    pub upgrade_level: u8,
    #[serde(default)]
    pub armor_upgrade_level: u8,
    #[serde(default)]
    pub enemy_tactics_level: u8,
    #[serde(default)]
    pub enemy_ai_goal: AiGoal,
    #[serde(default)]
    pub enemy_ai_budget: u16,
    #[serde(default)]
    pub enemy_ai_decision_index: u32,
    #[serde(default)]
    pub enemy_ai_history: Vec<AiDecision>,
    #[serde(default)]
    pub enemy_resources_available: u32,
    #[serde(default)]
    pub enemy_resources_generated: u32,
    #[serde(default)]
    pub enemy_resources_spent: u32,
    #[serde(default)]
    pub enemy_workers: u8,
    #[serde(default)]
    pub enemy_structures: Vec<SimStructure>,
    #[serde(default)]
    pub enemy_researched_techs: BTreeSet<String>,
    #[serde(default)]
    pub enemy_jobs: Vec<SimJob>,
    #[serde(default)]
    pub enemy_build_order_index: u32,
    #[serde(default)]
    pub enemy_ability_activations: BTreeMap<String, u32>,
    pub outcome: Option<BattleOutcome>,
    pub event_count: u64,
}

impl MissionSimV1 {
    pub fn from_seed(seed: BattleSeedV1) -> Result<Self, SimError> {
        seed.validate()?;
        let skirmish_salt = simulation_salt(&seed);
        let party_positions = formation_positions(seed.map.party_start, &seed);
        let party = seed
            .party
            .iter()
            .enumerate()
            .map(|(index, unit)| SimUnit {
                unit_id: unit.unit_id.clone(),
                role: unit.role.clone(),
                persistent: unit.persistent,
                skill_ids: unit.skill_ids.clone(),
                max_hp: unit.stats.max_hp as i64,
                hp: unit.stats.max_hp as i64,
                damage: (unit.stats.damage as i64 * unit.stats.skill_power_permille as i64 / 1000)
                    .max(1),
                armor: unit.stats.armor as i64,
                move_speed_milli: unit.stats.move_speed_milli as i32,
                movement_budget_milli: 0,
                attack_interval_ticks: unit.stats.attack_interval_ticks.max(1),
                evasion_permille: unit.stats.evasion_permille,
                energy: unit.stats.energy as i64,
                max_energy: unit.stats.energy as i64,
                ability_range: unit.stats.ability_range.max(1) as i16,
                ability_cooldown_ticks: 0,
                guard_ticks: 0,
                position: party_positions[index],
                attacks_made: 0,
                stance: RtsUnitStance::Guard,
                patrol_anchor: None,
                patrol_target: None,
                patrol_returning: false,
                cargo: 0,
                cargo_capacity: WORKER_CARGO_CAPACITY,
                confirmed_kills: 0,
                veteran_rank: unit.veteran_rank,
            })
            .collect();
        let enemy_profiles = [
            ("scout", 800, 8, 3, 1_050, 18),
            ("warden", 1_200, 10, 7, 760, 24),
            ("striker", 900, 12, 4, 900, 20),
            ("relay_guard", 1_400, 11, 8, 680, 26),
        ];
        let aftershock = is_aftershock_map(&seed.map_id);
        let siege = seed.map_id == "mirror_siege";
        let skirmish = seed.skirmish.enabled;
        let mission_scale = if siege {
            110
        } else if aftershock {
            112
        } else if skirmish {
            105
        } else {
            100
        };
        let difficulty_scale = match seed.difficulty {
            CampaignDifficulty::Story => 90,
            CampaignDifficulty::Standard => 100,
            CampaignDifficulty::Veteran => 115,
        };
        let enemy_scale = mission_scale * difficulty_scale / 100;
        let enemy_faction = if seed.skirmish.enabled {
            seed.skirmish.enemy_faction
        } else {
            RtsFaction::AshenCompact
        };
        let enemy_roster_hp_percent = match seed.difficulty {
            CampaignDifficulty::Story => 100,
            CampaignDifficulty::Standard => 150,
            CampaignDifficulty::Veteran => 300,
        };
        let mut enemies = seed
            .map
            .enemy_spawns
            .iter()
            .enumerate()
            .filter(|(index, _)| {
                if !seed.skirmish.enabled {
                    return true;
                }
                match seed.difficulty {
                    CampaignDifficulty::Story => index.is_multiple_of(2),
                    CampaignDifficulty::Standard => index % 4 != 3,
                    CampaignDifficulty::Veteran => true,
                }
            })
            .map(|(index, spawn)| {
                let roster_match = UNIT_ROSTER
                    .iter()
                    .find(|unit| unit.faction == enemy_faction && unit.id == spawn.id)
                    .or_else(|| {
                        seed.skirmish.enabled.then_some(()).and_then(|_| {
                            UNIT_ROSTER
                                .iter()
                                .filter(|unit| unit.faction == enemy_faction)
                                .nth(index % 6)
                        })
                    });
                let (role, hp, damage, armor, speed, interval) = roster_match
                    .map(|unit| {
                        let speed = match unit.ability() {
                            UnitAbility::RevealPulse | UnitAbility::SmokeDash => 1_080,
                            UnitAbility::ArcVolley | UnitAbility::PiercingCharge => 920,
                            UnitAbility::FieldRepair | UnitAbility::DemolitionCharge => 820,
                            UnitAbility::SuppressionBlast | UnitAbility::CommandSurge => 650,
                            _ => 760,
                        };
                        (
                            unit.role,
                            unit.hp as i64 * enemy_roster_hp_percent / 100,
                            unit.damage as i64,
                            2_i64 + unit.supply as i64 * 2,
                            speed,
                            17_u32 + unit.supply as u32 * 3,
                        )
                    })
                    .unwrap_or(enemy_profiles[index.min(enemy_profiles.len() - 1)]);
                SimUnit {
                    unit_id: spawn.id.clone(),
                    role: role.to_string(),
                    persistent: false,
                    skill_ids: roster_match
                        .map(|unit| vec![unit.ability().rule_id().to_string()])
                        .unwrap_or_default(),
                    max_hp: hp * enemy_scale / 100,
                    hp: hp * enemy_scale / 100,
                    damage: damage * enemy_scale / 100,
                    armor: armor
                        + if aftershock || siege { 1 } else { 0 }
                        + if seed.difficulty == CampaignDifficulty::Veteran {
                            1
                        } else {
                            0
                        },
                    move_speed_milli: speed,
                    movement_budget_milli: 0,
                    attack_interval_ticks: interval,
                    evasion_permille: 25 + index as u16 * 10,
                    energy: 0,
                    max_energy: 0,
                    ability_range: 1,
                    ability_cooldown_ticks: 0,
                    guard_ticks: 0,
                    position: nearest_passable(&seed, spawn.position).unwrap_or(spawn.position),
                    attacks_made: 0,
                    stance: RtsUnitStance::Aggressive,
                    patrol_anchor: None,
                    patrol_target: None,
                    patrol_returning: false,
                    cargo: 0,
                    cargo_capacity: WORKER_CARGO_CAPACITY,
                    confirmed_kills: 0,
                    veteran_rank: 0,
                }
            })
            .collect::<Vec<_>>();
        if seed.skirmish.enabled {
            for index in 0..3 {
                let position = nearest_passable(
                    &seed,
                    BattleGridPoint::new(
                        seed.map.objective.x - 1 - index as i16 - (skirmish_salt % 2) as i16,
                        seed.map.objective.y + 1 + (skirmish_salt % 3) as i16,
                    ),
                )
                .unwrap_or(seed.map.objective);
                enemies.push(SimUnit {
                    unit_id: format!("enemy_worker_{index}"),
                    role: "worker".to_string(),
                    persistent: false,
                    skill_ids: vec!["enemy_harvest".to_string()],
                    max_hp: 420,
                    hp: 420,
                    damage: 5,
                    armor: 1,
                    move_speed_milli: 850,
                    movement_budget_milli: 0,
                    attack_interval_ticks: 30,
                    evasion_permille: 20,
                    energy: 0,
                    max_energy: 0,
                    ability_range: 1,
                    ability_cooldown_ticks: 0,
                    guard_ticks: 0,
                    position,
                    attacks_made: 0,
                    stance: RtsUnitStance::Guard,
                    patrol_anchor: None,
                    patrol_target: None,
                    patrol_returning: false,
                    cargo: 0,
                    cargo_capacity: WORKER_CARGO_CAPACITY,
                    confirmed_kills: 0,
                    veteran_rank: 0,
                });
            }
        }
        let relay_guard_base = if siege {
            RELAY_GUARD_HP + 900
        } else if aftershock {
            RELAY_GUARD_HP + 600
        } else {
            RELAY_GUARD_HP
        };
        let relay_guard_max_hp = relay_guard_base * difficulty_scale / 100;
        let starting_resources =
            seed.expedition_readiness
                .starting_resources
                .saturating_add(if seed.skirmish.enabled {
                    seed.skirmish.starting_resources
                } else {
                    0
                });
        let mut sim = Self {
            contract_version: RTS_SIM_CONTRACT.to_string(),
            seed: seed.clone(),
            tick: 0,
            phase: if seed.mission.mission == trnm_campaign_core::CampaignMission::ConvoyExodus {
                BattlePhase::ConvoyEscort
            } else {
                BattlePhase::Approach
            },
            objective_index: 0,
            objective_progress_ticks: 0,
            convoy_position: (seed.mission.mission
                == trnm_campaign_core::CampaignMission::ConvoyExodus)
                .then(|| {
                    nearest_passable(
                        &seed,
                        BattleGridPoint::new(seed.map.party_start.x - 1, seed.map.party_start.y),
                    )
                    .unwrap_or(seed.map.party_start)
                }),
            convoy_hp: if seed.mission.mission == trnm_campaign_core::CampaignMission::ConvoyExodus
            {
                1_200
            } else {
                0
            },
            move_intents: BTreeMap::new(),
            tile_reservations: Vec::new(),
            active_order: None,
            human_enemy_authority: false,
            enemy_active_order: None,
            queued_orders: VecDeque::new(),
            control_groups: BTreeMap::new(),
            last_order_frame: None,
            enemy_last_order_frame: None,
            order_count: 0,
            distinct_order_kinds: BTreeSet::new(),
            replay_orders: Vec::new(),
            party,
            enemies,
            relay_guard_hp: relay_guard_max_hp,
            relay_guard_max_hp,
            relay_capture_ticks: 0,
            resources_gathered: starting_resources,
            resources_available: starting_resources,
            resources_spent: 0,
            resources_generated: starting_resources,
            player_score: 0,
            enemy_score: 0,
            resource_nodes: seed
                .map
                .resource_nodes
                .iter()
                .enumerate()
                .map(|(index, node)| ResourceNodeState {
                    node_id: node.id.clone(),
                    position: nearest_passable(&seed, node.position).unwrap_or(node.position),
                    remaining: if seed.skirmish.enabled {
                        RESOURCE_NODE_CAPACITY
                            .saturating_sub(((skirmish_salt + index as u64 * 73) % 121) as u32)
                    } else {
                        RESOURCE_NODE_CAPACITY
                    },
                })
                .collect(),
            structures: vec![
                SimStructure {
                    structure_id: "expedition_command_post".to_string(),
                    kind: SimStructureKind::CommandPost,
                    position: seed.map.party_start,
                    hp: 900,
                    max_hp: 900,
                },
                SimStructure {
                    structure_id: "field_workshop".to_string(),
                    kind: SimStructureKind::FieldWorkshop,
                    position: nearest_passable(
                        &seed,
                        BattleGridPoint::new(seed.map.party_start.x + 2, seed.map.party_start.y),
                    )
                    .unwrap_or(seed.map.party_start),
                    hp: 600,
                    max_hp: 600,
                },
            ],
            reinforcement_wave: 0,
            intel_level: 0,
            recon_bonus_ticks: 0,
            recon_focus: None,
            visible_tiles: BTreeSet::new(),
            explored_tiles: BTreeSet::new(),
            jobs: Vec::new(),
            authority_job_commands: Vec::new(),
            support_units: Vec::new(),
            researched_techs: BTreeSet::new(),
            upgrade_level: 0,
            armor_upgrade_level: 0,
            enemy_tactics_level: 0,
            enemy_ai_goal: AiGoal::Scout,
            enemy_ai_budget: 0,
            enemy_ai_decision_index: 0,
            enemy_ai_history: Vec::new(),
            enemy_resources_available: if seed.skirmish.enabled {
                seed.skirmish.starting_resources
            } else {
                0
            },
            enemy_resources_generated: if seed.skirmish.enabled {
                seed.skirmish.starting_resources
            } else {
                0
            },
            enemy_resources_spent: 0,
            enemy_workers: if seed.skirmish.enabled { 3 } else { 0 },
            enemy_structures: if seed.skirmish.enabled {
                vec![
                    SimStructure {
                        structure_id: "enemy_command_post".to_string(),
                        kind: SimStructureKind::CommandPost,
                        position: seed.map.objective,
                        hp: 1_200,
                        max_hp: 1_200,
                    },
                    SimStructure {
                        structure_id: "enemy_field_workshop".to_string(),
                        kind: SimStructureKind::FieldWorkshop,
                        position: nearest_passable(
                            &seed,
                            BattleGridPoint::new(seed.map.objective.x - 2, seed.map.objective.y),
                        )
                        .unwrap_or(seed.map.objective),
                        hp: 600,
                        max_hp: 600,
                    },
                    SimStructure {
                        structure_id: "enemy_supply_cache".to_string(),
                        kind: SimStructureKind::SupplyCache,
                        position: nearest_passable(
                            &seed,
                            BattleGridPoint::new(
                                seed.map.objective.x - 1,
                                seed.map.objective.y + 2,
                            ),
                        )
                        .unwrap_or(seed.map.objective),
                        hp: 420,
                        max_hp: 420,
                    },
                    SimStructure {
                        structure_id: "enemy_supply_cache_aux".to_string(),
                        kind: SimStructureKind::SupplyCache,
                        position: nearest_passable(
                            &seed,
                            BattleGridPoint::new(
                                seed.map.objective.x - 3,
                                seed.map.objective.y + 2,
                            ),
                        )
                        .unwrap_or(seed.map.objective),
                        hp: 420,
                        max_hp: 420,
                    },
                ]
            } else {
                Vec::new()
            },
            enemy_researched_techs: BTreeSet::new(),
            enemy_jobs: Vec::new(),
            enemy_build_order_index: if seed.skirmish.enabled {
                (skirmish_salt % 8) as u32
            } else {
                0
            },
            enemy_ability_activations: BTreeMap::new(),
            outcome: None,
            event_count: 0,
        };
        sim.assign_control_group(
            "1",
            sim.party.iter().map(|unit| unit.unit_id.clone()).collect(),
        );
        sim.refresh_visibility();
        sim.validate()?;
        Ok(sim)
    }

    pub fn validate(&self) -> Result<(), SimError> {
        if self.contract_version != RTS_SIM_CONTRACT {
            return Err(SimError::InvalidState(format!(
                "unsupported simulation contract {}",
                self.contract_version
            )));
        }
        self.seed.validate()?;
        let expected_ids = self
            .seed
            .party
            .iter()
            .map(|unit| unit.unit_id.as_str())
            .collect::<BTreeSet<_>>();
        let party_ids = self
            .party
            .iter()
            .map(|unit| unit.unit_id.as_str())
            .collect::<BTreeSet<_>>();
        let actual_ids = if self.human_enemy_authority {
            self.party
                .iter()
                .chain(&self.enemies)
                .map(|unit| unit.unit_id.as_str())
                .collect::<BTreeSet<_>>()
        } else {
            party_ids.clone()
        };
        let generated_roster_units_are_valid = self.party.iter().all(|unit| {
            expected_ids.contains(unit.unit_id.as_str())
                || (!unit.persistent
                    && UNIT_ROSTER.iter().any(|entry| {
                        unit.unit_id.starts_with(&format!("{}_", entry.id))
                            && unit.role == entry.role
                            && unit
                                .skill_ids
                                .iter()
                                .any(|skill| skill == entry.ability().rule_id())
                    }))
        });
        if !expected_ids.is_subset(&actual_ids)
            || (!self.human_enemy_authority && actual_ids.len() != self.party.len())
            || (self.human_enemy_authority
                && actual_ids.len() != self.party.len() + self.enemies.len())
            || !generated_roster_units_are_valid
        {
            return Err(SimError::Integrity(
                "simulation party does not match BattleSeed".to_string(),
            ));
        }
        for unit in self.party.iter().chain(&self.enemies) {
            if !self.seed.map.in_bounds(unit.position) || !self.seed.map.passable(unit.position) {
                return Err(SimError::Integrity(format!(
                    "unit {} occupies an invalid map tile",
                    unit.unit_id
                )));
            }
        }
        if self.objective_index > self.seed.mission.objectives.len()
            || self.convoy_position.is_some_and(|position| {
                !self.seed.map.in_bounds(position) || !self.seed.map.passable(position)
            })
            || self.convoy_hp < 0
        {
            return Err(SimError::Integrity(
                "mission objective or convoy state is invalid".to_string(),
            ));
        }
        if self.tile_reservations.iter().any(|reservation| {
            !self.seed.map.in_bounds(reservation.tile)
                || !self.seed.map.passable(reservation.tile)
                || !self
                    .party
                    .iter()
                    .any(|unit| unit.unit_id == reservation.unit_id)
        }) {
            return Err(SimError::Integrity(
                "tile reservation references invalid traffic state".to_string(),
            ));
        }
        for support in &self.support_units {
            if !self.seed.map.in_bounds(support.position)
                || !self.seed.map.passable(support.position)
                || support.hp <= 0
            {
                return Err(SimError::Integrity(format!(
                    "support unit {} occupies an invalid state",
                    support.unit_id
                )));
            }
        }
        for structure in self.structures.iter().chain(&self.enemy_structures) {
            if !self.seed.map.in_bounds(structure.position)
                || !self.seed.map.passable(structure.position)
                || structure.max_hp <= 0
                || structure.hp > structure.max_hp
            {
                return Err(SimError::Integrity(format!(
                    "structure {} occupies an invalid state",
                    structure.structure_id
                )));
            }
        }
        if self.seed.skirmish.enabled
            && self
                .enemy_resources_available
                .saturating_add(self.enemy_resources_spent)
                != self.enemy_resources_generated
        {
            return Err(SimError::Integrity(
                "enemy resource conservation is inconsistent".to_string(),
            ));
        }
        for node in &self.resource_nodes {
            if !self.seed.map.in_bounds(node.position)
                || node.remaining > RESOURCE_NODE_CAPACITY
                || !self.seed.map.passable(node.position)
            {
                return Err(SimError::Integrity(format!(
                    "resource node {} occupies an invalid state",
                    node.node_id
                )));
            }
        }
        for job in self.jobs.iter().chain(&self.enemy_jobs) {
            if job.remaining_ticks == 0
                || !self.seed.map.in_bounds(job.target)
                || !self.seed.map.passable(job.target)
                || job.cost == 0
            {
                return Err(SimError::Integrity(format!(
                    "queued job {} is invalid",
                    job.job_id
                )));
            }
        }
        if self
            .jobs
            .iter()
            .any(|job| job.side != AuthoritySide::Player)
            || self
                .enemy_jobs
                .iter()
                .any(|job| job.side != AuthoritySide::Enemy)
        {
            return Err(SimError::Integrity(
                "job queue authority side is inconsistent".to_string(),
            ));
        }
        if self
            .resources_available
            .saturating_add(self.resources_spent)
            != self.resources_gathered
            || self.relay_guard_max_hp <= 0
            || self.relay_guard_hp > self.relay_guard_max_hp
        {
            return Err(SimError::Integrity(
                "resource or relay accounting is inconsistent".to_string(),
            ));
        }
        if let Some(order) = &self.active_order {
            order.validate().map_err(SimError::Order)?;
        }
        if let Some(order) = &self.enemy_active_order {
            order.validate().map_err(SimError::Order)?;
        }
        for order in &self.queued_orders {
            order.validate().map_err(SimError::Order)?;
            if !order.queued {
                return Err(SimError::Integrity(
                    "queued order storage contains a non-queued order".to_string(),
                ));
            }
        }
        for members in self.control_groups.values() {
            if !members
                .iter()
                .all(|member| actual_ids.contains(member.as_str()))
            {
                return Err(SimError::Integrity(
                    "control group references an unknown party unit".to_string(),
                ));
            }
        }
        if self
            .visible_tiles
            .iter()
            .chain(&self.explored_tiles)
            .any(|tile| !self.seed.map.in_bounds(*tile))
        {
            return Err(SimError::Integrity(
                "fog state contains an out-of-bounds tile".to_string(),
            ));
        }
        if self.enemy_ai_history.len() > 16
            || self
                .enemy_ai_history
                .windows(2)
                .any(|pair| pair[0].index >= pair[1].index)
        {
            return Err(SimError::Integrity(
                "enemy AI decision history is not a bounded ordered replay".to_string(),
            ));
        }
        if self.replay_orders.len() > MAX_REPLAY_ORDERS
            || self
                .replay_orders
                .windows(2)
                .any(|orders| orders[0].issued_tick > orders[1].issued_tick)
        {
            return Err(SimError::Integrity(
                "player replay is not bounded and frame ordered".to_string(),
            ));
        }
        Ok(())
    }

    pub fn supply_cap(&self) -> u8 {
        self.structures
            .iter()
            .filter(|structure| structure.alive())
            .map(SimStructure::supply_provided)
            .fold(0_u8, u8::saturating_add)
    }

    pub fn supply_used(&self) -> u8 {
        let active_party = self.party.iter().filter(|unit| unit.alive()).count() as u8;
        let support = self
            .support_units
            .iter()
            .map(|unit| unit.supply)
            .fold(0_u8, u8::saturating_add);
        let reserved = self
            .jobs
            .iter()
            .map(|job| match job.kind {
                SimJobKind::TrainSupport | SimJobKind::TrainMedic => 1,
                SimJobKind::TrainRosterUnit => UNIT_ROSTER
                    .iter()
                    .find(|unit| unit.id == job.rule_id)
                    .map(|unit| unit.supply)
                    .unwrap_or(1),
                _ => 0,
            })
            .fold(0_u8, u8::saturating_add);
        active_party
            .saturating_add(support)
            .saturating_add(reserved)
    }

    pub fn power_provided(&self) -> u16 {
        self.structures
            .iter()
            .filter(|structure| structure.alive())
            .map(SimStructure::power_provided)
            .sum()
    }

    pub fn power_draw(&self) -> u16 {
        self.structures
            .iter()
            .filter(|structure| structure.alive())
            .map(SimStructure::power_draw)
            .sum()
    }

    pub fn low_power(&self) -> bool {
        self.power_draw() > self.power_provided()
    }

    pub fn enemy_supply_cap(&self) -> u8 {
        self.enemy_structures
            .iter()
            .filter(|structure| structure.alive())
            .map(SimStructure::supply_provided)
            .fold(0_u8, u8::saturating_add)
    }

    pub fn enemy_supply_used(&self) -> u8 {
        let living = self
            .enemies
            .iter()
            .filter(|unit| unit.alive())
            .map(|unit| {
                if unit.role == "worker" {
                    1
                } else {
                    UNIT_ROSTER
                        .iter()
                        .find(|entry| unit.unit_id.contains(entry.id))
                        .map(|entry| entry.supply)
                        .unwrap_or(1)
                }
            })
            .fold(0_u8, u8::saturating_add);
        let reserved = self
            .enemy_jobs
            .iter()
            .filter(|job| job.kind == SimJobKind::TrainRosterUnit)
            .filter_map(|job| UNIT_ROSTER.iter().find(|entry| entry.id == job.rule_id))
            .map(|entry| entry.supply)
            .fold(0_u8, u8::saturating_add);
        living.saturating_add(reserved)
    }

    pub fn enemy_power_provided(&self) -> u16 {
        self.enemy_structures
            .iter()
            .filter(|structure| structure.alive())
            .map(SimStructure::power_provided)
            .sum()
    }

    pub fn enemy_power_draw(&self) -> u16 {
        self.enemy_structures
            .iter()
            .filter(|structure| structure.alive())
            .map(SimStructure::power_draw)
            .sum()
    }

    pub fn enemy_low_power(&self) -> bool {
        self.enemy_power_draw() > self.enemy_power_provided()
    }

    pub fn terminal(&self) -> bool {
        self.outcome.is_some()
    }

    pub fn current_order_kind(&self) -> RtsOrderKind {
        self.active_order
            .as_ref()
            .map(|order| order.kind)
            .unwrap_or(RtsOrderKind::Hold)
    }

}

