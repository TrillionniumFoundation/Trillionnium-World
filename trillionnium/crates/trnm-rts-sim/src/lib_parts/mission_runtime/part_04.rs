impl MissionSimV1 {
    fn resolve_structure_functions(&mut self) {
        for side in [AuthoritySide::Player, AuthoritySide::Enemy] {
            if side == AuthoritySide::Enemy && !self.seed.skirmish.enabled {
                continue;
            }
            if self.tick.is_multiple_of(20)
                && self.side_has_structure(side, SimStructureKind::SensorTower)
            {
                match side {
                    AuthoritySide::Player => {
                        self.intel_level = 3;
                        self.recon_bonus_ticks = self.recon_bonus_ticks.max(80);
                    }
                    AuthoritySide::Enemy => {
                        self.intel_level = self.intel_level.saturating_sub(1);
                        self.recon_bonus_ticks = self.recon_bonus_ticks.min(20);
                    }
                }
            }
            if self.tick.is_multiple_of(40)
                && self.side_has_structure(side, SimStructureKind::FieldHospital)
            {
                let units = match side {
                    AuthoritySide::Player => &mut self.party,
                    AuthoritySide::Enemy => &mut self.enemies,
                };
                for unit in units {
                    if unit.alive() {
                        unit.hp = (unit.hp + 12).min(unit.max_hp);
                    }
                }
            }
            if self.side_has_structure(side, SimStructureKind::ForwardRally) {
                match side {
                    AuthoritySide::Player => {
                        if let Some(job) = self
                            .jobs
                            .iter_mut()
                            .find(|job| job.kind != SimJobKind::BuildStructure && !job.paused)
                        {
                            job.remaining_ticks = job.remaining_ticks.saturating_sub(1);
                        }
                    }
                    AuthoritySide::Enemy => {
                        if let Some(job) = self
                            .enemy_jobs
                            .first_mut()
                            .filter(|job| job.kind != SimJobKind::BuildStructure)
                        {
                            job.remaining_ticks = job.remaining_ticks.saturating_sub(1);
                        }
                    }
                }
            }
            if self.tick.is_multiple_of(50)
                && self.side_has_structure(side, SimStructureKind::AshBeacon)
            {
                let opponent = match side {
                    AuthoritySide::Player => &mut self.enemies,
                    AuthoritySide::Enemy => &mut self.party,
                };
                if let Some(unit) = opponent.iter_mut().find(|unit| unit.alive()) {
                    unit.hp -= 18;
                }
            }
            if self.tick.is_multiple_of(80)
                && self.side_has_structure(side, SimStructureKind::SiegeFoundry)
            {
                match side {
                    AuthoritySide::Player => {
                        for support in &mut self.support_units {
                            if matches!(support.role.as_str(), "siege" | "heavy") {
                                support.damage += 1;
                            }
                        }
                    }
                    AuthoritySide::Enemy => {
                        for unit in &mut self.enemies {
                            if unit.alive() && matches!(unit.role.as_str(), "siege" | "heavy") {
                                unit.damage += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    fn spend_resources(&mut self, cost: u32, label: &str) -> Result<(), SimError> {
        if self.resources_available < cost {
            return Err(SimError::Order(format!(
                "{label} requires {cost} field resources"
            )));
        }
        self.resources_available -= cost;
        self.resources_spent = self.resources_spent.saturating_add(cost);
        Ok(())
    }

    fn submit_authority_job(
        &mut self,
        source: AuthorityCommandSource,
        job: SimJob,
        label: &str,
    ) -> Result<(), SimError> {
        let side = job.side;
        if !matches!(
            (source, side),
            (AuthorityCommandSource::PlayerOrder, AuthoritySide::Player)
                | (AuthorityCommandSource::AdaptiveAi, AuthoritySide::Enemy)
        ) {
            return Err(SimError::Order(format!(
                "{label} source is not authorized for {side:?}"
            )));
        }
        if job.remaining_ticks == 0
            || !self.seed.map.in_bounds(job.target)
            || !self.seed.map.passable(job.target)
        {
            return Err(SimError::Order(format!(
                "{label} produced an invalid side job"
            )));
        }
        if job.kind == SimJobKind::BuildStructure && job.builder_id.is_none() {
            return Err(SimError::Order(format!(
                "{label} requires a living builder assignment"
            )));
        }
        let (resources_available, resources_spent, jobs) = match side {
            AuthoritySide::Player => (
                &mut self.resources_available,
                &mut self.resources_spent,
                &mut self.jobs,
            ),
            AuthoritySide::Enemy => (
                &mut self.enemy_resources_available,
                &mut self.enemy_resources_spent,
                &mut self.enemy_jobs,
            ),
        };
        if *resources_available < job.cost {
            return Err(SimError::Order(format!(
                "{label} requires {} field resources",
                job.cost
            )));
        }
        *resources_available -= job.cost;
        *resources_spent = resources_spent.saturating_add(job.cost);
        let record = AuthorityJobCommandRecord {
            tick: self.tick,
            source,
            side,
            job_id: job.job_id.clone(),
            kind: job.kind,
            rule_id: job.rule_id.clone(),
        };
        jobs.push(job);
        self.authority_job_commands.push(record);
        self.event_count += 1;
        Ok(())
    }

    fn spawn_reinforcement_wave(&mut self, aftershock: bool) {
        self.reinforcement_wave = self.reinforcement_wave.saturating_add(1);
        self.enemy_tactics_level = self.enemy_tactics_level.saturating_add(1).min(3);
        let count = if aftershock { 3 } else { 2 };
        let scale = 100 + i64::from(self.reinforcement_wave) * 8 + if aftershock { 18 } else { 0 };
        for index in 0..count {
            let spawn = &self.seed.map.enemy_spawns
                [(index + self.reinforcement_wave as usize) % self.seed.map.enemy_spawns.len()];
            let spawn_position = self.unoccupied_spawn_tile(spawn.position);
            let role = if index % 2 == 0 { "striker" } else { "warden" };
            let hp = 420 * scale / 100;
            self.enemies.push(SimUnit {
                unit_id: format!("aftershock_wave{}_{}", self.reinforcement_wave, index),
                role: role.to_string(),
                persistent: false,
                skill_ids: Vec::new(),
                max_hp: hp,
                hp,
                damage: (10 + i64::from(self.reinforcement_wave) * 2) * scale / 100,
                armor: 4 + i64::from(self.reinforcement_wave),
                move_speed_milli: 920,
                movement_budget_milli: 0,
                attack_interval_ticks: 20,
                evasion_permille: 35,
                energy: 0,
                max_energy: 0,
                ability_range: 1,
                ability_cooldown_ticks: 0,
                guard_ticks: 0,
                position: spawn_position,
                attacks_made: 0,
                stance: RtsUnitStance::Aggressive,
                patrol_anchor: None,
                patrol_target: None,
                patrol_returning: false,
                cargo: 0,
                cargo_capacity: WORKER_CARGO_CAPACITY,
                confirmed_kills: 0,
                veteran_rank: 0,
            });
        }
        self.event_count += 1;
    }

    fn resolve_party_ability(&mut self, order: &RtsFrameOrder) -> Result<(), SimError> {
        let selected = order
            .subject_actor_ids
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        let mut activated = 0;
        for index in 0..self.party.len() {
            if !self.party[index].alive()
                || !selected.contains(&self.party[index].unit_id)
                || self.party[index].ability_cooldown_ticks > 0
            {
                continue;
            }
            let skill = signature_skill(&self.party[index]);
            let cost = match skill {
                "iron_guard" => 18,
                "wind_step" => 22,
                "inner_flame" => 28,
                "relay_overcharge" => 24,
                "field_mend" => 26,
                _ => 20,
            };
            if self.party[index].energy < cost {
                continue;
            }
            self.party[index].energy -= cost;
            self.party[index].ability_cooldown_ticks = 120;
            match skill {
                "iron_guard" => self.party[index].guard_ticks = 100,
                "wind_step" => {
                    let target = order
                        .target_tile
                        .map(|tile| BattleGridPoint::new(tile.x as i16, tile.y as i16))
                        .unwrap_or(self.seed.map.approach_point);
                    let occupied = self
                        .party
                        .iter()
                        .chain(&self.enemies)
                        .filter(|unit| unit.alive() && unit.unit_id != self.party[index].unit_id)
                        .map(|unit| unit.position)
                        .collect::<BTreeSet<_>>();
                    for _ in 0..4 {
                        let Some(next) = next_step_toward(
                            &self.seed,
                            self.party[index].position,
                            target,
                            0,
                            &occupied,
                        ) else {
                            break;
                        };
                        self.party[index].position = next;
                    }
                }
                "inner_flame" => {
                    if let Some(target_index) = self
                        .enemies
                        .iter()
                        .enumerate()
                        .filter(|(_, enemy)| enemy.alive())
                        .filter(|(_, enemy)| {
                            distance(self.party[index].position, enemy.position)
                                <= self.party[index].ability_range * 2
                        })
                        .min_by_key(|(_, enemy)| {
                            distance(self.party[index].position, enemy.position)
                        })
                        .map(|(index, _)| index)
                    {
                        self.enemies[target_index].hp -= 110 + self.party[index].damage * 2;
                    } else if self.phase == BattlePhase::Relay
                        && distance(self.party[index].position, self.seed.map.objective)
                            <= self.party[index].ability_range * 2
                    {
                        self.relay_guard_hp -= 150 + self.party[index].damage * 2;
                    }
                }
                "relay_overcharge" => {
                    self.resources_generated = self.resources_generated.saturating_add(20);
                    self.resources_gathered = self.resources_gathered.saturating_add(20);
                    self.resources_available = self.resources_available.saturating_add(20);
                    if self.phase == BattlePhase::Relay {
                        self.relay_guard_hp -= 120;
                    }
                }
                "field_mend" => {
                    for unit in &mut self.party {
                        if unit.alive() {
                            unit.hp = (unit.hp + 90).min(unit.max_hp);
                        }
                    }
                }
                _ => self.party[index].guard_ticks = 60,
            }
            activated += 1;
            self.event_count += 1;
        }
        if activated == 0 {
            return Err(SimError::Order(
                "selected units have no ready signature ability or energy".to_string(),
            ));
        }
        Ok(())
    }

    fn resolve_enemy_workers(&mut self) {
        if !self.seed.skirmish.enabled {
            return;
        }
        advance_side_construction_worker(
            &self.seed,
            &mut self.enemies,
            &self.party,
            &self.enemy_jobs,
        );
        let Some(command_position) = self
            .enemy_structures
            .iter()
            .find(|structure| structure.alive() && structure.kind == SimStructureKind::CommandPost)
            .map(|structure| structure.position)
        else {
            return;
        };
        let worker_count = advance_worker_logistics(
            &self.seed,
            self.tick,
            &mut self.enemies,
            &self.party,
            &self.enemy_jobs,
            &mut self.resource_nodes,
            command_position,
            None,
            None,
            &mut self.enemy_resources_available,
            &mut self.enemy_resources_generated,
            &mut self.enemy_score,
            &mut self.event_count,
            false,
        );
        self.enemy_workers = worker_count.min(u8::MAX as usize) as u8;
    }

    fn advance_side_jobs(&mut self, side: AuthoritySide) {
        let powered = match side {
            AuthoritySide::Player => {
                !self.low_power()
                    && self
                        .side_has_structure(AuthoritySide::Player, SimStructureKind::FieldWorkshop)
            }
            AuthoritySide::Enemy => {
                !self.enemy_low_power()
                    && self
                        .side_has_structure(AuthoritySide::Enemy, SimStructureKind::FieldWorkshop)
            }
        };
        let worker_alive = match side {
            AuthoritySide::Player => self.party.iter().any(|unit| unit.alive()),
            AuthoritySide::Enemy => self
                .enemies
                .iter()
                .any(|unit| unit.alive() && unit.role == "worker"),
        };
        if !worker_alive {
            return;
        }
        let completed = match side {
            AuthoritySide::Player => {
                advance_side_job_queue(&mut self.jobs, &self.party, powered);
                let completed = self
                    .jobs
                    .iter()
                    .filter(|job| job.remaining_ticks == 0)
                    .cloned()
                    .collect::<Vec<_>>();
                self.jobs.retain(|job| job.remaining_ticks > 0);
                completed
            }
            AuthoritySide::Enemy => {
                advance_side_job_queue(&mut self.enemy_jobs, &self.enemies, powered);
                let completed = self
                    .enemy_jobs
                    .iter()
                    .filter(|job| job.remaining_ticks == 0)
                    .cloned()
                    .collect::<Vec<_>>();
                self.enemy_jobs.retain(|job| job.remaining_ticks > 0);
                completed
            }
        };
        for job in completed {
            match side {
                AuthoritySide::Player => self.apply_player_job_completion(job),
                AuthoritySide::Enemy => self.apply_enemy_job_completion(job),
            }
        }
    }

    fn apply_enemy_job_completion(&mut self, job: SimJob) {
        match job.kind {
            SimJobKind::BuildStructure => {
                if let Some(kind) = SimStructureKind::from_rule_id(&job.rule_id) {
                    let definition = kind.definition();
                    self.enemy_structures.push(SimStructure {
                        structure_id: format!(
                            "enemy_{}_{}",
                            job.rule_id,
                            self.enemy_structures.len()
                        ),
                        kind,
                        position: job.target,
                        hp: i64::from(definition.hp),
                        max_hp: i64::from(definition.hp),
                    });
                }
            }
            SimJobKind::ResearchLogistics
            | SimJobKind::ResearchOptics
            | SimJobKind::UpgradeRelayArms
            | SimJobKind::UpgradeFieldArmor
            | SimJobKind::ResearchSensorNet
            | SimJobKind::ResearchFieldMedicine
            | SimJobKind::UpgradeSiegeDrills
            | SimJobKind::UpgradeReactivePlating
            | SimJobKind::ResearchWayfinderDrills
            | SimJobKind::ResearchRapidMustering => {
                self.enemy_researched_techs.insert(job.rule_id.clone());
                match job.rule_id.as_str() {
                    "relay_arms" | "siege_drills" => {
                        for enemy in &mut self.enemies {
                            enemy.damage += 3;
                        }
                    }
                    "field_armor" | "reactive_plating" => {
                        for enemy in &mut self.enemies {
                            enemy.armor += 2;
                        }
                    }
                    "rapid_mustering" => {
                        let position = self.unoccupied_spawn_tile(self.seed.map.objective);
                        let worker_index = self.enemy_workers;
                        self.enemies.push(SimUnit {
                            unit_id: format!("enemy_worker_mustered_{worker_index}"),
                            role: "worker".to_string(),
                            persistent: false,
                            skill_ids: vec!["enemy_harvest".to_string()],
                            max_hp: 460,
                            hp: 460,
                            damage: 5,
                            armor: 2,
                            move_speed_milli: 900,
                            movement_budget_milli: 0,
                            attack_interval_ticks: 30,
                            evasion_permille: 25,
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
                        self.enemy_workers = self.enemy_workers.saturating_add(1);
                    }
                    _ => {}
                }
            }
            SimJobKind::TrainRosterUnit | SimJobKind::TrainSupport | SimJobKind::TrainMedic => {
                if let Some(unit) = UNIT_ROSTER.iter().find(|unit| unit.id == job.rule_id) {
                    let position = self.unoccupied_spawn_tile(
                        nearest_passable(
                            &self.seed,
                            BattleGridPoint::new(
                                self.seed.map.objective.x - 3,
                                self.seed.map.objective.y,
                            ),
                        )
                        .unwrap_or(self.seed.map.objective),
                    );
                    let damage_bonus = if self.enemy_researched_techs.contains("relay_arms") {
                        4
                    } else {
                        0
                    };
                    let armor_bonus = if self.enemy_researched_techs.contains("field_armor") {
                        2
                    } else {
                        0
                    };
                    let hp = i64::from(unit.hp)
                        * match self.seed.difficulty {
                            CampaignDifficulty::Story => 100,
                            CampaignDifficulty::Standard => 150,
                            CampaignDifficulty::Veteran => 300,
                        }
                        / 100;
                    self.enemies.push(SimUnit {
                        unit_id: format!("enemy_{}_{}", unit.id, self.enemy_build_order_index),
                        role: unit.role.to_string(),
                        persistent: false,
                        skill_ids: vec![unit.ability().rule_id().to_string()],
                        max_hp: hp,
                        hp,
                        damage: i64::from(unit.damage) + damage_bonus,
                        armor: i64::from(unit.supply) * 2 + armor_bonus,
                        move_speed_milli: match unit.ability() {
                            UnitAbility::SmokeDash | UnitAbility::PiercingCharge => 1_050,
                            UnitAbility::CommandSurge | UnitAbility::SuppressionBlast => 700,
                            _ => 850,
                        },
                        movement_budget_milli: 0,
                        attack_interval_ticks: 17 + u32::from(unit.supply) * 3,
                        evasion_permille: if unit.ability() == UnitAbility::SmokeDash {
                            160
                        } else {
                            45
                        },
                        energy: 100,
                        max_energy: 100,
                        ability_range: 4,
                        ability_cooldown_ticks: 0,
                        guard_ticks: 0,
                        position,
                        attacks_made: 0,
                        stance: RtsUnitStance::Aggressive,
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
        }
        self.event_count += 1;
    }

    fn resolve_enemy_economy(&mut self) {
        if !self.seed.skirmish.enabled {
            return;
        }

        let command_alive = self
            .enemy_structures
            .iter()
            .any(|structure| structure.alive() && structure.kind == SimStructureKind::CommandPost);
        if !command_alive {
            self.enemy_jobs.clear();
            return;
        }

        self.advance_side_jobs(AuthoritySide::Enemy);

        if !self.enemy_jobs.is_empty() || !self.tick.is_multiple_of(20) {
            return;
        }
        let enemy_faction = self.seed.skirmish.enemy_faction;
        let faction_units = UNIT_ROSTER
            .iter()
            .filter(|unit| unit.faction == enemy_faction)
            .collect::<Vec<_>>();
        let faction_structure = match enemy_faction {
            RtsFaction::MirrorCoalition => "field_hospital",
            RtsFaction::AshenCompact => "siege_foundry",
        };
        let faction_tech = match enemy_faction {
            RtsFaction::MirrorCoalition => "wayfinder_drills",
            RtsFaction::AshenCompact => "rapid_mustering",
        };
        let power_rule = if enemy_faction == RtsFaction::AshenCompact {
            "ash_beacon"
        } else {
            "relay_generator"
        };
        let supply_deficit = self.enemy_supply_used() >= self.enemy_supply_cap();
        let power_deficit = self.enemy_low_power();
        let workshop_missing =
            !self.side_has_structure(AuthoritySide::Enemy, SimStructureKind::FieldWorkshop);
        let choice = self.enemy_build_order_index % 8;
        let default_unit =
            faction_units[(self.enemy_build_order_index as usize) % faction_units.len()];
        let default_train = || {
            (
                SimJobKind::TrainRosterUnit,
                default_unit.id,
                default_unit.cost,
                70 + u32::from(default_unit.supply) * 10,
            )
        };
        let (kind, rule_id, cost, duration) = if workshop_missing {
            let definition = SimStructureKind::FieldWorkshop.definition();
            (
                SimJobKind::BuildStructure,
                "field_workshop",
                definition.cost,
                60,
            )
        } else if power_deficit {
            let definition = SimStructureKind::from_rule_id(power_rule)
                .expect("enemy power rule is catalogued")
                .definition();
            (SimJobKind::BuildStructure, power_rule, definition.cost, 55)
        } else if supply_deficit {
            let definition = SimStructureKind::SupplyCache.definition();
            (
                SimJobKind::BuildStructure,
                "supply_cache",
                definition.cost,
                55,
            )
        } else {
            match choice {
                0 => default_train(),
                1 if !self.enemy_researched_techs.contains("field_logistics") => (
                    research_job_kind("field_logistics"),
                    "field_logistics",
                    35,
                    70,
                ),
                2 if !self.side_has_structure(
                    AuthoritySide::Enemy,
                    SimStructureKind::from_rule_id(faction_structure)
                        .expect("enemy faction structure is catalogued"),
                ) =>
                {
                    (SimJobKind::BuildStructure, faction_structure, 55, 80)
                }
                3 if !self.enemy_researched_techs.contains("relay_arms") => {
                    (research_job_kind("relay_arms"), "relay_arms", 45, 75)
                }
                5 if !self.enemy_researched_techs.contains(faction_tech) => {
                    (research_job_kind(faction_tech), faction_tech, 45, 80)
                }
                6 if !self.enemy_researched_techs.contains("field_armor") => {
                    (research_job_kind("field_armor"), "field_armor", 45, 75)
                }
                _ => default_train(),
            }
        };
        if kind == SimJobKind::TrainRosterUnit {
            let population_cap = match self.seed.difficulty {
                CampaignDifficulty::Story => 12,
                CampaignDifficulty::Standard => 16,
                CampaignDifficulty::Veteran => 24,
            };
            if self.enemies.iter().filter(|unit| unit.alive()).count() >= population_cap {
                self.enemy_build_order_index = self.enemy_build_order_index.saturating_add(1);
                return;
            }
            let required_supply = UNIT_ROSTER
                .iter()
                .find(|unit| unit.id == rule_id)
                .map(|unit| unit.supply)
                .unwrap_or(1);
            if self.enemy_supply_used().saturating_add(required_supply) > self.enemy_supply_cap() {
                return;
            }
        }
        if self.enemy_resources_available >= cost {
            let builder_id = (kind == SimJobKind::BuildStructure)
                .then(|| {
                    self.enemies
                        .iter()
                        .filter(|unit| unit.alive() && unit.role == "worker")
                        .min_by_key(|unit| distance(unit.position, self.seed.map.objective))
                        .map(|unit| unit.unit_id.clone())
                })
                .flatten();
            let target = (kind == SimJobKind::BuildStructure).then(|| {
                let offset = self.enemy_structures.len() as i16;
                nearest_passable(
                    &self.seed,
                    BattleGridPoint::new(
                        self.seed.map.objective.x - 1 - offset % 4,
                        self.seed.map.objective.y + offset % 3 - 1,
                    ),
                )
                .unwrap_or(self.seed.map.objective)
            });
            let difficulty_duration = match self.seed.difficulty {
                CampaignDifficulty::Story => duration.saturating_mul(5),
                CampaignDifficulty::Standard => duration.saturating_mul(4),
                CampaignDifficulty::Veteran => duration,
            };
            if self
                .submit_authority_job(
                    AuthorityCommandSource::AdaptiveAi,
                    SimJob {
                        job_id: format!("enemy-{}-{}", rule_id, self.enemy_build_order_index),
                        kind,
                        rule_id: rule_id.to_string(),
                        remaining_ticks: difficulty_duration,
                        target: target.unwrap_or(self.seed.map.objective),
                        cost,
                        paused: false,
                        builder_id,
                        side: AuthoritySide::Enemy,
                    },
                    "enemy AI job command",
                )
                .is_ok()
            {
                self.enemy_build_order_index = self.enemy_build_order_index.saturating_add(1);
            }
        }
    }

    fn observe_enemy_ai(&self) -> AiObservation {
        AiObservation {
            tick: self.tick,
            phase: self.phase,
            living_party: self.party.iter().filter(|unit| unit.alive()).count() as u8,
            living_enemies: self.enemies.iter().filter(|unit| unit.alive()).count() as u8,
            wounded_party: self
                .party
                .iter()
                .filter(|unit| unit.alive() && unit.hp * 2 < unit.max_hp)
                .count() as u8,
            party_resources: self.resources_available,
            party_structures: self
                .structures
                .iter()
                .filter(|structure| structure.alive())
                .count() as u8,
            researched_tech_count: self.researched_techs.len() as u8
                + self.upgrade_level
                + self.armor_upgrade_level,
            convoy_active: self.convoy_position.is_some() && self.convoy_hp > 0,
        }
    }

}

impl MissionSimV1 {
    fn refresh_enemy_ai_plan(&mut self) {
        let (interval, budget_gain) = match self.seed.difficulty {
            CampaignDifficulty::Story => (70, 6),
            CampaignDifficulty::Standard => (50, 8),
            CampaignDifficulty::Veteran => (35, 10),
        };
        if self.tick != 1 && !self.tick.is_multiple_of(interval) {
            return;
        }
        let observation = self.observe_enemy_ai();
        let budget_before = self.enemy_ai_budget.saturating_add(budget_gain).min(40);
        let (requested_goal, cost, reason) = if observation.convoy_active {
            (AiGoal::InterdictConvoy, 8, "escort target is exposed")
        } else if observation.party_resources >= 120 || observation.party_structures >= 4 {
            (AiGoal::RaidEconomy, 7, "player economy is accelerating")
        } else if observation.researched_tech_count > 0 {
            (AiGoal::CounterTech, 6, "player technology is visible")
        } else if observation.living_enemies.saturating_mul(2) <= observation.living_party
            || self.relay_guard_hp * 2 < self.relay_guard_max_hp
        {
            (
                AiGoal::DefendObjective,
                5,
                "enemy force or objective integrity is low",
            )
        } else if self.tick < 300 {
            (AiGoal::Scout, 2, "contact picture is incomplete")
        } else {
            (AiGoal::Assault, 4, "battle line is stable enough to commit")
        };
        let (goal, spent, reason) = if budget_before >= cost {
            (requested_goal, cost, reason.to_string())
        } else {
            (
                AiGoal::Scout,
                2.min(budget_before),
                "budget is insufficient; gathering information".to_string(),
            )
        };
        self.enemy_ai_goal = goal;
        self.enemy_ai_budget = budget_before.saturating_sub(spent);
        self.enemy_ai_decision_index = self.enemy_ai_decision_index.saturating_add(1);
        self.enemy_tactics_level = (self.enemy_ai_decision_index / 2).min(3) as u8;
        self.enemy_ai_history.push(AiDecision {
            index: self.enemy_ai_decision_index,
            goal,
            budget_before,
            budget_after: self.enemy_ai_budget,
            reason,
            observation,
        });
        if self.enemy_ai_history.len() > 16 {
            self.enemy_ai_history.remove(0);
        }
        self.event_count = self.event_count.saturating_add(1);
    }

    fn resolve_enemy_ai(&mut self) {
        if self.phase == BattlePhase::Approach && self.tick < 300 {
            return;
        }
        let goal = self.enemy_ai_goal;
        let tactics_level = self.enemy_tactics_level;
        let objective = self.seed.map.objective;
        let convoy = self.convoy_position;
        let mut occupied = self
            .party
            .iter()
            .chain(&self.enemies)
            .filter(|unit| unit.alive())
            .map(|unit| unit.position)
            .collect::<BTreeSet<_>>();
        for attacker_index in 0..self.enemies.len() {
            if !self.enemies[attacker_index].alive()
                || self.enemies[attacker_index].role == "worker"
            {
                continue;
            }
            if matches!(goal, AiGoal::RaidEconomy | AiGoal::Assault) {
                if let Some(structure_index) = self
                    .structures
                    .iter()
                    .enumerate()
                    .filter(|(_, structure)| structure.alive())
                    .min_by_key(|(_, structure)| {
                        distance(self.enemies[attacker_index].position, structure.position)
                    })
                    .map(|(index, _)| index)
                {
                    let target = self.structures[structure_index].position;
                    let range = self.enemies[attacker_index].attack_range();
                    if distance(self.enemies[attacker_index].position, target) > range {
                        self.enemies[attacker_index].movement_budget_milli +=
                            self.enemies[attacker_index].move_speed_milli;
                        if self.enemies[attacker_index].movement_budget_milli >= MOVEMENT_TILE_COST
                        {
                            occupied.remove(&self.enemies[attacker_index].position);
                            if let Some(next) = next_step_toward(
                                &self.seed,
                                self.enemies[attacker_index].position,
                                target,
                                range,
                                &occupied,
                            ) {
                                self.enemies[attacker_index].position = next;
                                self.enemies[attacker_index].movement_budget_milli -=
                                    MOVEMENT_TILE_COST;
                            }
                            occupied.insert(self.enemies[attacker_index].position);
                        }
                    } else if self
                        .tick
                        .is_multiple_of(self.enemies[attacker_index].attack_interval_ticks as u64)
                    {
                        let demolition = self.enemies[attacker_index]
                            .skill_ids
                            .iter()
                            .any(|skill| skill == UnitAbility::DemolitionCharge.rule_id())
                            && self.enemies[attacker_index].ability_cooldown_ticks == 0;
                        let bonus = if demolition { 55 } else { 0 };
                        self.structures[structure_index].hp -=
                            (self.enemies[attacker_index].damage + bonus).max(1);
                        if demolition {
                            self.enemies[attacker_index].ability_cooldown_ticks = 45;
                            *self
                                .enemy_ability_activations
                                .entry(UnitAbility::DemolitionCharge.rule_id().to_string())
                                .or_default() += 1;
                        }
                        self.event_count += 1;
                    }
                    continue;
                }
            }
            let Some(target_index) = self
                .party
                .iter()
                .enumerate()
                .filter(|(_, unit)| unit.alive())
                .min_by_key(|(_, unit)| {
                    let wounded_bias = unit.hp * 100 / unit.max_hp.max(1);
                    let role_bias = match goal {
                        AiGoal::RaidEconomy
                            if matches!(unit.role.as_str(), "worker" | "engineer") =>
                        {
                            -50
                        }
                        AiGoal::CounterTech if matches!(unit.role.as_str(), "mystic" | "medic") => {
                            -45
                        }
                        AiGoal::Scout if unit.role == "scout" => -25,
                        _ if tactics_level >= 2 && unit.role == "engineer" => -3,
                        _ => 0,
                    };
                    let position_bias = match goal {
                        AiGoal::DefendObjective => distance(unit.position, objective) as i64 * 4,
                        AiGoal::InterdictConvoy => convoy
                            .map(|position| distance(unit.position, position) as i64)
                            .unwrap_or_default(),
                        _ => 0,
                    };
                    distance(self.enemies[attacker_index].position, unit.position) as i64 * 10
                        + wounded_bias
                        + role_bias
                        + position_bias
                })
                .map(|(index, _)| index)
            else {
                return;
            };
            let range = self.enemies[attacker_index].attack_range();
            let target = self.party[target_index].position;
            if distance(self.enemies[attacker_index].position, target) > range {
                self.enemies[attacker_index].movement_budget_milli +=
                    self.enemies[attacker_index].move_speed_milli;
                if self.enemies[attacker_index].movement_budget_milli >= MOVEMENT_TILE_COST {
                    occupied.remove(&self.enemies[attacker_index].position);
                    if let Some(next) = next_step_toward(
                        &self.seed,
                        self.enemies[attacker_index].position,
                        target,
                        range,
                        &occupied,
                    ) {
                        self.enemies[attacker_index].position = next;
                        self.enemies[attacker_index].movement_budget_milli -= MOVEMENT_TILE_COST;
                    }
                    occupied.insert(self.enemies[attacker_index].position);
                }
            } else if self
                .tick
                .is_multiple_of(self.enemies[attacker_index].attack_interval_ticks as u64)
            {
                let ability = self.enemies[attacker_index]
                    .skill_ids
                    .iter()
                    .find_map(|skill| UnitAbility::from_rule_id(skill));
                let ability_ready =
                    ability.is_some() && self.enemies[attacker_index].ability_cooldown_ticks == 0;
                let hold_bonus = if self.current_order_kind() == RtsOrderKind::Hold {
                    3
                } else {
                    0
                };
                let guard_bonus = if self.party[target_index].guard_ticks > 0 {
                    7
                } else {
                    0
                };
                let role_bonus = match self.enemies[attacker_index].role.as_str() {
                    "assault" => 5,
                    "siege" if self.party[target_index].role == "engineer" => 9,
                    "heavy" => 4,
                    _ => 0,
                };
                let piercing_bonus =
                    if ability_ready && matches!(ability, Some(UnitAbility::PiercingCharge)) {
                        self.party[target_index].armor / 2 + 8
                    } else {
                        0
                    };
                let damage = (self.enemies[attacker_index].damage + role_bonus + piercing_bonus
                    - self.party[target_index].armor
                    - hold_bonus
                    - guard_bonus)
                    .max(1);
                let target_was_alive = self.party[target_index].alive();
                if !deterministic_evade(
                    self.tick,
                    target_index + 31 + simulation_salt(&self.seed) as usize,
                    self.party[target_index].evasion_permille,
                ) {
                    self.party[target_index].hp -= damage;
                    if self.enemies[attacker_index].role == "disruptor" {
                        self.party[target_index].energy =
                            self.party[target_index].energy.saturating_sub(8);
                        self.party[target_index].ability_cooldown_ticks = self.party[target_index]
                            .ability_cooldown_ticks
                            .saturating_add(10);
                    }
                    if self.enemies[attacker_index].role == "frontline" {
                        self.enemies[attacker_index].guard_ticks = 20;
                    }
                }
                if ability_ready {
                    self.activate_enemy_ability(attacker_index, target_index, ability.unwrap());
                }
                if target_was_alive && !self.party[target_index].alive() {
                    self.enemy_score = self.enemy_score.saturating_add(150);
                }
                self.enemies[attacker_index].attacks_made += 1;
                self.event_count += 1;
            }
        }
    }

    fn activate_enemy_ability(
        &mut self,
        attacker_index: usize,
        target_index: usize,
        ability: UnitAbility,
    ) {
        let attacker_position = self.enemies[attacker_index].position;
        self.enemies[attacker_index].ability_cooldown_ticks = 45;
        *self
            .enemy_ability_activations
            .entry(ability.rule_id().to_string())
            .or_default() += 1;
        match ability {
            UnitAbility::RevealPulse => {
                self.intel_level = self.intel_level.saturating_sub(1);
                self.recon_bonus_ticks = self.recon_bonus_ticks.min(5);
            }
            UnitAbility::GuardWall => {
                for enemy in &mut self.enemies {
                    if enemy.alive() && distance(enemy.position, attacker_position) <= 3 {
                        enemy.guard_ticks = enemy.guard_ticks.max(30);
                    }
                }
            }
            UnitAbility::ArcVolley => {
                if let Some((index, _)) = self
                    .party
                    .iter()
                    .enumerate()
                    .filter(|(index, unit)| *index != target_index && unit.alive())
                    .min_by_key(|(_, unit)| distance(unit.position, attacker_position))
                {
                    self.party[index].hp -= (self.enemies[attacker_index].damage / 2).max(1);
                }
            }
            UnitAbility::FieldRepair => {
                if let Some(structure) = self
                    .enemy_structures
                    .iter_mut()
                    .filter(|structure| structure.alive())
                    .min_by_key(|structure| distance(structure.position, attacker_position))
                {
                    structure.hp = (structure.hp + 60).min(structure.max_hp);
                }
            }
            UnitAbility::TriageAura => {
                for enemy in &mut self.enemies {
                    if enemy.alive() && distance(enemy.position, attacker_position) <= 3 {
                        enemy.hp = (enemy.hp + 45).min(enemy.max_hp);
                    }
                }
            }
            UnitAbility::SuppressionBlast => {
                self.party[target_index].energy =
                    self.party[target_index].energy.saturating_sub(18);
                self.party[target_index].ability_cooldown_ticks = self.party[target_index]
                    .ability_cooldown_ticks
                    .saturating_add(20);
            }
            UnitAbility::SmokeDash => {
                self.enemies[attacker_index].evasion_permille =
                    self.enemies[attacker_index].evasion_permille.max(180);
                self.enemies[attacker_index].movement_budget_milli = self.enemies[attacker_index]
                    .movement_budget_milli
                    .saturating_add(MOVEMENT_TILE_COST);
            }
            UnitAbility::RetaliationPlate => {
                self.enemies[attacker_index].guard_ticks = 50;
                self.party[target_index].hp -= 8;
            }
            UnitAbility::PiercingCharge => {
                self.party[target_index].guard_ticks = 0;
            }
            UnitAbility::DemolitionCharge => {
                if let Some(structure) = self
                    .structures
                    .iter_mut()
                    .filter(|structure| structure.alive())
                    .min_by_key(|structure| distance(structure.position, attacker_position))
                {
                    structure.hp -= 55;
                }
            }
            UnitAbility::SignalJam => {
                self.party[target_index].energy =
                    self.party[target_index].energy.saturating_sub(25);
                self.party[target_index].ability_cooldown_ticks = self.party[target_index]
                    .ability_cooldown_ticks
                    .saturating_add(35);
            }
            UnitAbility::CommandSurge => {
                for enemy in &mut self.enemies {
                    if enemy.alive() && distance(enemy.position, attacker_position) <= 4 {
                        enemy.movement_budget_milli = enemy
                            .movement_budget_milli
                            .saturating_add(MOVEMENT_TILE_COST / 2);
                        enemy.guard_ticks = enemy.guard_ticks.max(15);
                    }
                }
            }
        }
        self.event_count += 1;
    }

    fn resolve_relay_pressure(&mut self) {
        if self.human_enemy_authority
            || self.current_objective_kind() != Some(ObjectiveKind::Destroy)
            || self.relay_guard_hp <= 0
            || !self.tick.is_multiple_of(24)
        {
            return;
        }
        if let Some(target_index) = self
            .party
            .iter()
            .enumerate()
            .filter(|(_, unit)| unit.alive())
            .min_by_key(|(_, unit)| distance(unit.position, self.seed.map.objective))
            .map(|(index, _)| index)
        {
            let guard_bonus = if self.party[target_index].guard_ticks > 0 {
                8
            } else {
                0
            };
            self.party[target_index].hp -=
                (14 - self.party[target_index].armor - guard_bonus).max(1);
            self.event_count += 1;
        }
    }

    pub fn party_hp_percent(&self) -> u8 {
        percent(
            self.party.iter().map(|unit| unit.hp.max(0)).sum(),
            self.party.iter().map(|unit| unit.max_hp).sum(),
        )
    }

    pub fn is_enemy_visible(&self, enemy_id: &str) -> bool {
        self.enemies
            .iter()
            .find(|enemy| enemy.unit_id == enemy_id && enemy.alive())
            .is_some_and(|enemy| self.visible_tiles.contains(&enemy.position))
    }

    pub fn visible_percent(&self) -> u8 {
        let total = u32::from(self.seed.map.width) * u32::from(self.seed.map.height);
        (self.visible_tiles.len() as u32 * 100)
            .checked_div(total)
            .unwrap_or(0)
            .min(100) as u8
    }

    pub fn visible_enemy_count(&self) -> usize {
        self.enemies
            .iter()
            .filter(|enemy| enemy.alive() && self.visible_tiles.contains(&enemy.position))
            .count()
    }

    pub fn visible_enemy_hp_percent(&self) -> u8 {
        let visible = self
            .enemies
            .iter()
            .filter(|enemy| enemy.alive() && self.visible_tiles.contains(&enemy.position))
            .collect::<Vec<_>>();
        percent(
            visible.iter().map(|enemy| enemy.hp.max(0)).sum(),
            visible.iter().map(|enemy| enemy.max_hp).sum(),
        )
    }

    fn refresh_visibility(&mut self) {
        let mut visible = BTreeSet::new();
        let base_radius = 4 + i16::from(self.intel_level.min(2));
        for unit in self.party.iter().filter(|unit| unit.alive()) {
            reveal_from(&self.seed, unit.position, base_radius, &mut visible);
        }
        for support in &self.support_units {
            reveal_from(&self.seed, support.position, 3, &mut visible);
        }
        if self.recon_bonus_ticks > 0 {
            if let Some(focus) = self.recon_focus {
                let radius = if self.researched_techs.contains("signal_optics") {
                    8
                } else {
                    6
                };
                reveal_from(&self.seed, focus, radius, &mut visible);
            }
        }
        self.visible_tiles = visible;
        self.explored_tiles
            .extend(self.visible_tiles.iter().copied());
    }

    pub fn enemy_hp_percent(&self) -> u8 {
        percent(
            self.enemies.iter().map(|unit| unit.hp.max(0)).sum(),
            self.enemies.iter().map(|unit| unit.max_hp).sum(),
        )
    }

    pub fn relay_guard_percent(&self) -> u8 {
        percent(self.relay_guard_hp.max(0), self.relay_guard_max_hp)
    }

    pub fn capture_percent(&self) -> u8 {
        (self.relay_capture_ticks as u64 * 100 / CAPTURE_TICKS_REQUIRED as u64).min(100) as u8
    }

    pub fn snapshot_hash(&self) -> Result<String, SimError> {
        json_hash(self)
    }

    pub fn export_replay(&self) -> Result<BattleReplayV1, SimError> {
        self.validate()?;
        Ok(BattleReplayV1 {
            contract_version: "trnm_battle_replay_v1".to_string(),
            seed: self.seed.clone(),
            entries: self.replay_orders.clone(),
            final_tick: self.tick,
            final_snapshot_hash: self.snapshot_hash()?,
        })
    }

    pub fn export_replay_v2(&self) -> Result<BattleReplayV2, SimError> {
        BattleReplayV2::from_entries(
            self.seed.clone(),
            self.replay_orders.clone(),
            self.tick,
            self.snapshot_hash()?,
        )
    }

    pub fn into_result(self) -> Result<BattleResultV1, SimError> {
        self.validate()?;
        let outcome = self.outcome.ok_or_else(|| {
            SimError::InvalidState("cannot emit a BattleResult before terminal state".to_string())
        })?;
        let final_snapshot_hash = self.snapshot_hash()?;
        let siege = self.seed.map_id == "mirror_siege";
        let skirmish = self.seed.skirmish.enabled;
        let experience = match outcome {
            BattleOutcome::Victory if siege => 70,
            BattleOutcome::Victory if self.seed.map_id == "convoy_exodus" => 60,
            BattleOutcome::Victory if is_aftershock_map(&self.seed.map_id) => 55,
            BattleOutcome::Victory if skirmish => 50,
            BattleOutcome::Victory => 40,
            BattleOutcome::Defeat if self.tick >= 60 * TICKS_PER_SECOND => 3,
            BattleOutcome::Defeat | BattleOutcome::Withdrawal => 0,
        };
        let seeded_party_ids = self
            .seed
            .party
            .iter()
            .map(|unit| unit.unit_id.as_str())
            .collect::<BTreeSet<_>>();
        let units = self
            .party
            .iter()
            .chain(
                self.human_enemy_authority
                    .then_some(&self.enemies)
                    .into_iter()
                    .flatten(),
            )
            .filter(|unit| seeded_party_ids.contains(unit.unit_id.as_str()))
            .map(|unit| {
                let status = if unit.hp <= 0 {
                    if unit.persistent {
                        UnitBattleStatus::Incapacitated
                    } else {
                        UnitBattleStatus::Lost
                    }
                } else if unit.hp * 100 < unit.max_hp * 60 {
                    UnitBattleStatus::Wounded
                } else {
                    UnitBattleStatus::Healthy
                };
                UnitBattleReportV1 {
                    unit_id: unit.unit_id.clone(),
                    status,
                    remaining_hp: unit.hp.max(0) as u32,
                    experience_gained: experience,
                    veteran_rank: unit.veteran_rank,
                    confirmed_kills: unit.confirmed_kills,
                }
            })
            .collect();
        let aftershock = is_aftershock_map(&self.seed.map_id);
        let convoy = self.seed.map_id == "convoy_exodus";
        let (loot, reputation_delta, world_flags) = match outcome {
            BattleOutcome::Victory if self.seed.map_id == "iron_delta" => (
                vec![LootStack {
                    item_id: "salvaged-alloy".to_string(),
                    quantity: 3,
                }],
                4,
                vec!["iron_delta_won".to_string()],
            ),
            BattleOutcome::Victory if self.seed.map_id == "night_watch_crossing" => (
                vec![LootStack {
                    item_id: "watch-cloth".to_string(),
                    quantity: 3,
                }],
                4,
                vec!["night_watch_crossing_won".to_string()],
            ),
            BattleOutcome::Victory if self.seed.map_id == "glass_basin" => (
                vec![LootStack {
                    item_id: "route-token".to_string(),
                    quantity: 3,
                }],
                5,
                vec!["glass_basin_won".to_string()],
            ),
            BattleOutcome::Victory if self.seed.map_id == "ember_orchard" => (
                vec![LootStack {
                    item_id: "ash-runner-seal".to_string(),
                    quantity: 2,
                }],
                5,
                vec!["ember_orchard_won".to_string()],
            ),
            BattleOutcome::Victory if siege => (
                vec![LootStack {
                    item_id: "mirror-gate-insignia".to_string(),
                    quantity: 1,
                }],
                8,
                vec!["mirror_siege_secured".to_string()],
            ),
            BattleOutcome::Victory if convoy => (
                vec![LootStack {
                    item_id: "signal-convoy-seal".to_string(),
                    quantity: 1,
                }],
                6,
                vec!["convoy_exodus_secured".to_string()],
            ),
            BattleOutcome::Victory if aftershock => (
                vec![LootStack {
                    item_id: "field-tonic-kit".to_string(),
                    quantity: 1,
                }],
                3,
                vec!["aftershock_patrol_secured".to_string()],
            ),
            BattleOutcome::Victory => (
                vec![
                    LootStack {
                        item_id: "relay-core-fragment".to_string(),
                        quantity: 1,
                    },
                    LootStack {
                        item_id: "field-tonic-kit".to_string(),
                        quantity: 1,
                    },
                ],
                5,
                vec!["first_contact_secured".to_string()],
            ),
            BattleOutcome::Defeat => (
                Vec::new(),
                -2,
                vec![if siege {
                    "mirror_siege_lost".to_string()
                } else if convoy {
                    "convoy_exodus_lost".to_string()
                } else if aftershock {
                    "aftershock_patrol_repulsed".to_string()
                } else {
                    "first_contact_repulsed".to_string()
                }],
            ),
            BattleOutcome::Withdrawal => (
                Vec::new(),
                0,
                vec![if siege {
                    "mirror_siege_withdrawn".to_string()
                } else if convoy {
                    "convoy_exodus_withdrawn".to_string()
                } else if aftershock {
                    "aftershock_patrol_withdrawn".to_string()
                } else {
                    "first_contact_withdrawn".to_string()
                }],
            ),
        };
        Ok(BattleResultV1 {
            contract_version: BATTLE_RESULT_CONTRACT.to_string(),
            battle_id: self.seed.battle_id.clone(),
            seed_hash: self.seed.seed_hash.clone(),
            outcome,
            units,
            loot,
            resource_delta: if outcome == BattleOutcome::Victory {
                self.resources_available as i64
            } else {
                0
            },
            reputation_delta,
            world_flags,
            elapsed_ticks: self.tick,
            final_snapshot_hash,
        })
    }
}

