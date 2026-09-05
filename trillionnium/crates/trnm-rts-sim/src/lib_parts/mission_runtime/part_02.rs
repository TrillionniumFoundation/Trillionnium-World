impl MissionSimV1 {
    pub fn issue_order(&mut self, order: RtsFrameOrder) -> Result<(), SimError> {
        self.validate()?;
        if self.terminal() {
            return Err(SimError::InvalidState(
                "cannot issue an order to a terminal battle".to_string(),
            ));
        }
        order.validate().map_err(SimError::Order)?;
        if order.player_id != "player" {
            return Err(SimError::Order(
                "only the local player may command the party".to_string(),
            ));
        }
        if self
            .last_order_frame
            .is_some_and(|previous| order.frame < previous)
        {
            return Err(SimError::Order("order frame regression".to_string()));
        }
        let living_party = self
            .party
            .iter()
            .filter(|unit| unit.alive())
            .map(|unit| unit.unit_id.as_str())
            .collect::<BTreeSet<_>>();
        let subjects = order
            .subject_actor_ids
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if subjects.is_empty() || !subjects.is_subset(&living_party) {
            return Err(SimError::Order(
                "order subjects must be living seeded party units".to_string(),
            ));
        }
        if let Some(tile) = order.target_tile {
            let target = BattleGridPoint::new(tile.x as i16, tile.y as i16);
            if !self.seed.map.in_bounds(target) || !self.seed.map.passable(target) {
                return Err(SimError::Order(
                    "order target tile is blocked or outside the map".to_string(),
                ));
            }
        }
        if matches!(order.kind, RtsOrderKind::Attack | RtsOrderKind::FocusFire) {
            if let Some(target) = order.target_actor_id.as_deref() {
                let target_position = self
                    .enemies
                    .iter()
                    .find(|enemy| enemy.unit_id == target && enemy.alive())
                    .map(|enemy| enemy.position)
                    .or_else(|| {
                        self.enemy_structures
                            .iter()
                            .find(|structure| structure.structure_id == target && structure.alive())
                            .map(|structure| structure.position)
                    });
                if self.seed.skirmish.enabled && target_position.is_none() {
                    return Err(SimError::Order("attack target is not alive".to_string()));
                }
                if target_position.is_some_and(|position| !self.visible_tiles.contains(&position)) {
                    return Err(SimError::Order(
                        "target enemy is outside current line of sight".to_string(),
                    ));
                }
            }
        }
        if matches!(order.kind, RtsOrderKind::Extract) {
            if self.tick < WITHDRAWAL_MIN_TICKS {
                return Err(SimError::Order(
                    "withdrawal requires thirty committed simulation ticks".to_string(),
                ));
            }
            self.outcome = Some(BattleOutcome::Withdrawal);
            self.phase = BattlePhase::Complete;
        } else if order.queued {
            if !is_continuous_order(order.kind) {
                return Err(SimError::Order(
                    "only movement, combat, harvest and hold orders may be shift-queued"
                        .to_string(),
                ));
            }
            self.queued_orders.push_back(order.clone());
            if self.active_order.is_none() {
                self.activate_next_queued_order();
            }
        } else {
            match order.kind {
                RtsOrderKind::Ability => self.resolve_party_ability(&order)?,
                RtsOrderKind::Repair => self.resolve_repair(&order)?,
                RtsOrderKind::Build => self.resolve_fortify(&order)?,
                RtsOrderKind::Recon => self.resolve_recon(&order)?,
                RtsOrderKind::Train | RtsOrderKind::Research | RtsOrderKind::Upgrade => {
                    self.queue_job_from_order(&order)?
                }
                RtsOrderKind::AssignGroup => self.assign_control_group_order(&order, false),
                RtsOrderKind::AppendGroup => self.assign_control_group_order(&order, true),
                RtsOrderKind::RemoveGroup => self.remove_control_group_order(&order),
                RtsOrderKind::RecallGroup => {
                    let group = order.target_rule_id.as_deref().unwrap_or_default();
                    if self.control_group_members(group).is_empty() {
                        return Err(SimError::Order("control group is empty".to_string()));
                    }
                }
                RtsOrderKind::CancelQueuedOrder => self.cancel_queued_order(&order)?,
                RtsOrderKind::CancelJob => self.cancel_job(&order)?,
                RtsOrderKind::PauseJob => self.set_job_paused(&order, true)?,
                RtsOrderKind::ResumeJob => self.set_job_paused(&order, false)?,
                RtsOrderKind::PromoteJob => self.promote_job(&order)?,
                RtsOrderKind::SetRally => self.set_rally(&order)?,
                RtsOrderKind::SetStance => self.set_unit_stance(&order)?,
                RtsOrderKind::Stop => {
                    self.queued_orders.clear();
                    self.active_order = None;
                    for unit in &mut self.party {
                        if order.subject_actor_ids.contains(&unit.unit_id) {
                            unit.patrol_anchor = None;
                            unit.patrol_target = None;
                        }
                    }
                }
                RtsOrderKind::Move
                | RtsOrderKind::AttackMove
                | RtsOrderKind::Patrol
                | RtsOrderKind::Harvest
                | RtsOrderKind::Capture
                | RtsOrderKind::Attack
                | RtsOrderKind::FocusFire
                | RtsOrderKind::Hold => {
                    self.queued_orders.clear();
                    if order.kind == RtsOrderKind::Patrol {
                        let target = order.target_tile.expect("validated patrol tile");
                        for unit in &mut self.party {
                            if order.subject_actor_ids.contains(&unit.unit_id) {
                                unit.patrol_anchor = Some(unit.position);
                                unit.patrol_target =
                                    Some(BattleGridPoint::new(target.x as i16, target.y as i16));
                                unit.patrol_returning = false;
                            }
                        }
                    }
                    self.active_order = Some(order.clone());
                }
                RtsOrderKind::Extract => unreachable!("withdrawal handled above"),
            }
        }
        self.last_order_frame = Some(order.frame);
        self.order_count = self.order_count.saturating_add(1);
        self.distinct_order_kinds
            .insert(order.kind.as_str().to_string());
        self.replay_orders.push(SimReplayEntry {
            issued_tick: self.tick,
            order,
        });
        self.event_count += 1;
        Ok(())
    }

    pub fn enable_human_enemy_authority(
        &mut self,
        enemy_unit_ids: &BTreeSet<String>,
    ) -> Result<(), SimError> {
        self.validate()?;
        if enemy_unit_ids.is_empty()
            || !enemy_unit_ids.iter().all(|unit_id| {
                self.party
                    .iter()
                    .any(|unit| &unit.unit_id == unit_id && unit.alive())
            })
        {
            return Err(SimError::Order(
                "human enemy control set must contain seeded living party units".to_string(),
            ));
        }
        // Validate both sides before draining the authoritative party. A rejected
        // partition must not move even one unit out of the player roster.
        if !self
            .party
            .iter()
            .any(|unit| unit.alive() && !enemy_unit_ids.contains(&unit.unit_id))
        {
            return Err(SimError::Order(
                "ranked PvP requires at least one unit on each side".to_string(),
            ));
        }
        let mut retained = Vec::new();
        let mut human_enemies = Vec::new();
        for unit in self.party.drain(..) {
            if enemy_unit_ids.contains(&unit.unit_id) {
                human_enemies.push(unit);
            } else {
                retained.push(unit);
            }
        }
        for (index, unit) in human_enemies.iter_mut().enumerate() {
            let requested = self
                .seed
                .map
                .enemy_spawns
                .get(index)
                .map(|spawn| spawn.position)
                .unwrap_or(self.seed.map.objective);
            unit.position = nearest_passable(&self.seed, requested).unwrap_or(requested);
            unit.stance = RtsUnitStance::Guard;
            unit.patrol_anchor = None;
            unit.patrol_target = None;
            unit.patrol_returning = false;
            unit.movement_budget_milli = 0;
        }
        self.party = retained;
        self.enemies = human_enemies;
        self.human_enemy_authority = true;
        self.enemy_active_order = None;
        self.enemy_last_order_frame = None;
        self.enemy_structures.clear();
        self.enemy_jobs.clear();
        self.enemy_ai_history.clear();
        self.control_groups.clear();
        self.assign_control_group(
            "1",
            self.party.iter().map(|unit| unit.unit_id.clone()).collect(),
        );
        self.refresh_visibility();
        self.validate()
    }

    pub fn issue_human_enemy_order(&mut self, order: RtsFrameOrder) -> Result<(), SimError> {
        self.validate()?;
        if !self.human_enemy_authority {
            return Err(SimError::Order(
                "simulation has no human enemy authority".to_string(),
            ));
        }
        if self.terminal() {
            return Err(SimError::InvalidState(
                "cannot issue an order to a terminal battle".to_string(),
            ));
        }
        order.validate().map_err(SimError::Order)?;
        if self
            .enemy_last_order_frame
            .is_some_and(|previous| order.frame < previous)
        {
            return Err(SimError::Order("enemy order frame regression".to_string()));
        }
        let living = self
            .enemies
            .iter()
            .filter(|unit| unit.alive())
            .map(|unit| unit.unit_id.as_str())
            .collect::<BTreeSet<_>>();
        let subjects = order
            .subject_actor_ids
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if subjects.is_empty() || !subjects.is_subset(&living) {
            return Err(SimError::Order(
                "enemy order subjects must be living human-controlled units".to_string(),
            ));
        }
        if !matches!(
            order.kind,
            RtsOrderKind::Move
                | RtsOrderKind::AttackMove
                | RtsOrderKind::Attack
                | RtsOrderKind::FocusFire
                | RtsOrderKind::Hold
                | RtsOrderKind::Stop
        ) {
            return Err(SimError::Order(
                "ranked PvP enemy authority currently accepts move/attack/hold/stop orders"
                    .to_string(),
            ));
        }
        if let Some(tile) = order.target_tile {
            let target = BattleGridPoint::new(tile.x as i16, tile.y as i16);
            if !self.seed.map.in_bounds(target) || !self.seed.map.passable(target) {
                return Err(SimError::Order(
                    "enemy order target tile is blocked or outside the map".to_string(),
                ));
            }
        }
        if matches!(order.kind, RtsOrderKind::Attack | RtsOrderKind::FocusFire)
            && order.target_actor_id.as_deref().is_some_and(|target| {
                !self
                    .party
                    .iter()
                    .any(|unit| unit.unit_id == target && unit.alive())
            })
        {
            return Err(SimError::Order(
                "human enemy attack target is not alive".to_string(),
            ));
        }
        if order.kind == RtsOrderKind::Stop {
            self.enemy_active_order = None;
        } else {
            self.enemy_active_order = Some(order.clone());
        }
        self.enemy_last_order_frame = Some(order.frame);
        self.order_count = self.order_count.saturating_add(1);
        self.distinct_order_kinds
            .insert(format!("enemy:{}", order.kind.as_str()));
        self.replay_orders.push(SimReplayEntry {
            issued_tick: self.tick,
            order,
        });
        self.event_count = self.event_count.saturating_add(1);
        Ok(())
    }

    pub fn control_group_members(&self, group_id: &str) -> Vec<String> {
        self.control_groups
            .get(group_id)
            .into_iter()
            .flatten()
            .filter(|member| {
                self.party
                    .iter()
                    .any(|unit| unit.unit_id == **member && unit.alive())
            })
            .cloned()
            .collect()
    }

    fn assign_control_group(&mut self, group_id: &str, members: BTreeSet<String>) {
        self.control_groups.insert(group_id.to_string(), members);
    }

    fn assign_control_group_order(&mut self, order: &RtsFrameOrder, append: bool) {
        let group = order.target_rule_id.as_deref().unwrap_or_default();
        let members = order
            .subject_actor_ids
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        if append {
            self.control_groups
                .entry(group.to_string())
                .or_default()
                .extend(members);
        } else {
            self.assign_control_group(group, members);
        }
    }

    fn remove_control_group_order(&mut self, order: &RtsFrameOrder) {
        let group = order.target_rule_id.as_deref().unwrap_or_default();
        if let Some(members) = self.control_groups.get_mut(group) {
            for member in &order.subject_actor_ids {
                members.remove(member);
            }
        }
    }

    fn prune_control_groups(&mut self) {
        let living = self
            .party
            .iter()
            .filter(|unit| unit.alive())
            .map(|unit| unit.unit_id.clone())
            .collect::<BTreeSet<_>>();
        for members in self.control_groups.values_mut() {
            members.retain(|member| living.contains(member));
        }
    }

    fn cancel_queued_order(&mut self, order: &RtsFrameOrder) -> Result<(), SimError> {
        let queue_id = order.queue_id.as_deref().unwrap_or_default();
        let before = self.queued_orders.len();
        self.queued_orders
            .retain(|queued| queued.queue_id.as_deref() != Some(queue_id));
        if before == self.queued_orders.len() {
            return Err(SimError::Order("queued order was not found".to_string()));
        }
        Ok(())
    }

    fn activate_next_queued_order(&mut self) {
        self.active_order = self.queued_orders.pop_front().map(|mut order| {
            order.queued = false;
            order
        });
    }

    fn active_order_complete(&self) -> bool {
        let Some(order) = &self.active_order else {
            return true;
        };
        match order.kind {
            RtsOrderKind::Move | RtsOrderKind::AttackMove => {
                order.target_tile.is_some_and(|tile| {
                    let target = BattleGridPoint::new(tile.x as i16, tile.y as i16);
                    self.party.iter().filter(|unit| unit.alive()).all(|unit| {
                        !order.subject_actor_ids.contains(&unit.unit_id)
                            || distance(unit.position, target) <= 1
                    })
                })
            }
            RtsOrderKind::Patrol => false,
            RtsOrderKind::Attack | RtsOrderKind::FocusFire => order
                .target_actor_id
                .as_deref()
                .and_then(|id| {
                    self.enemies
                        .iter()
                        .find(|enemy| enemy.unit_id == id)
                        .map(SimTargetRef::Unit)
                        .or_else(|| {
                            self.enemy_structures
                                .iter()
                                .find(|structure| structure.structure_id == id)
                                .map(SimTargetRef::Structure)
                        })
                })
                .map_or(
                    self.phase == BattlePhase::Relay
                        && self.relay_guard_hp <= 0
                        && self.enemies.iter().all(|enemy| !enemy.alive()),
                    SimTargetRef::destroyed,
                ),
            RtsOrderKind::Harvest => self.resources_available >= 200,
            RtsOrderKind::Capture | RtsOrderKind::Hold => self.terminal(),
            _ => true,
        }
    }

    fn move_human_enemies_toward(
        &mut self,
        selected: &BTreeSet<String>,
        target: BattleGridPoint,
        stop_range: i16,
    ) {
        let mut occupied = self
            .party
            .iter()
            .chain(&self.enemies)
            .filter(|unit| unit.alive())
            .map(|unit| unit.position)
            .collect::<BTreeSet<_>>();
        for index in 0..self.enemies.len() {
            if !self.enemies[index].alive() || !selected.contains(&self.enemies[index].unit_id) {
                continue;
            }
            self.enemies[index].movement_budget_milli += self.enemies[index].move_speed_milli;
            if self.enemies[index].movement_budget_milli < MOVEMENT_TILE_COST {
                continue;
            }
            occupied.remove(&self.enemies[index].position);
            if let Some(next) = next_step_toward(
                &self.seed,
                self.enemies[index].position,
                target,
                stop_range,
                &occupied,
            ) {
                self.enemies[index].position = next;
                self.enemies[index].movement_budget_milli -= MOVEMENT_TILE_COST;
            }
            occupied.insert(self.enemies[index].position);
        }
    }

    fn human_enemy_attack(&mut self, selected: &BTreeSet<String>, requested: Option<&str>) {
        for attacker_index in 0..self.enemies.len() {
            if !self.enemies[attacker_index].alive()
                || !selected.contains(&self.enemies[attacker_index].unit_id)
                || !self
                    .tick
                    .is_multiple_of(self.enemies[attacker_index].attack_interval_ticks as u64)
            {
                continue;
            }
            let target_index = requested
                .and_then(|id| {
                    self.party
                        .iter()
                        .position(|unit| unit.unit_id == id && unit.alive())
                })
                .or_else(|| {
                    self.party
                        .iter()
                        .enumerate()
                        .filter(|(_, unit)| unit.alive())
                        .min_by_key(|(_, unit)| {
                            distance(self.enemies[attacker_index].position, unit.position)
                        })
                        .map(|(index, _)| index)
                });
            let Some(target_index) = target_index else {
                continue;
            };
            if distance(
                self.enemies[attacker_index].position,
                self.party[target_index].position,
            ) > self.enemies[attacker_index].attack_range()
            {
                continue;
            }
            let damage =
                (self.enemies[attacker_index].damage - self.party[target_index].armor).max(1);
            let was_alive = self.party[target_index].alive();
            if !deterministic_evade(
                self.tick,
                target_index + 97 + simulation_salt(&self.seed) as usize,
                self.party[target_index].evasion_permille,
            ) {
                self.party[target_index].hp -= damage;
            }
            self.enemies[attacker_index].attacks_made += 1;
            if was_alive && !self.party[target_index].alive() {
                self.enemy_score = self.enemy_score.saturating_add(100);
                self.enemies[attacker_index].confirmed_kills = self.enemies[attacker_index]
                    .confirmed_kills
                    .saturating_add(1);
            }
            self.event_count = self.event_count.saturating_add(1);
        }
    }

    fn resolve_human_enemy_order(&mut self) {
        let Some(order) = self.enemy_active_order.clone() else {
            return;
        };
        let selected = order
            .subject_actor_ids
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        match order.kind {
            RtsOrderKind::Move | RtsOrderKind::AttackMove => {
                if let Some(tile) = order.target_tile {
                    self.move_human_enemies_toward(
                        &selected,
                        BattleGridPoint::new(tile.x as i16, tile.y as i16),
                        0,
                    );
                }
                if order.kind == RtsOrderKind::AttackMove {
                    self.human_enemy_attack(&selected, order.target_actor_id.as_deref());
                }
            }
            RtsOrderKind::Attack | RtsOrderKind::FocusFire => {
                let target = order
                    .target_actor_id
                    .as_deref()
                    .and_then(|id| {
                        self.party
                            .iter()
                            .find(|unit| unit.unit_id == id && unit.alive())
                    })
                    .or_else(|| self.party.iter().find(|unit| unit.alive()))
                    .map(|unit| unit.position);
                if let Some(target) = target {
                    self.move_human_enemies_toward(&selected, target, 1);
                }
                self.human_enemy_attack(&selected, order.target_actor_id.as_deref());
            }
            RtsOrderKind::Hold | RtsOrderKind::Stop => {}
            _ => unreachable!("human enemy order kinds are validated at submission"),
        }
    }

    pub fn step(&mut self) -> Result<(), SimError> {
        self.validate()?;
        if self.terminal() {
            return Err(SimError::InvalidState(
                "cannot advance a terminal battle".to_string(),
            ));
        }
        self.tick += 1;
        self.recon_bonus_ticks = self.recon_bonus_ticks.saturating_sub(1);
        advance_side_construction_worker(&self.seed, &mut self.party, &self.enemies, &self.jobs);
        self.advance_side_jobs(AuthoritySide::Player);
        self.resolve_structure_functions();
        for support in &mut self.support_units {
            support.ability_cooldown_ticks = support.ability_cooldown_ticks.saturating_sub(1);
        }
        for unit in &mut self.party {
            unit.ability_cooldown_ticks = unit.ability_cooldown_ticks.saturating_sub(1);
            unit.guard_ticks = unit.guard_ticks.saturating_sub(1);
            if self.tick.is_multiple_of(50) {
                unit.energy = (unit.energy + 1).min(unit.max_energy);
            }
        }
        for unit in &mut self.enemies {
            unit.ability_cooldown_ticks = unit.ability_cooldown_ticks.saturating_sub(1);
            unit.guard_ticks = unit.guard_ticks.saturating_sub(1);
        }
        self.resolve_player_order();
        self.resolve_stance_fire();
        self.update_phase();
        if self.human_enemy_authority {
            self.resolve_human_enemy_order();
        } else {
            self.refresh_enemy_ai_plan();
            self.resolve_enemy_workers();
            self.resolve_enemy_economy();
            self.resolve_enemy_ai();
        }
        self.resolve_support_fire();
        self.resolve_relay_pressure();
        self.resolve_mission_objective();
        self.update_phase();
        self.prune_control_groups();
        self.refresh_visibility();
        if self.active_order_complete() && !self.terminal() {
            self.active_order = None;
            self.activate_next_queued_order();
        }
        let time_limit = if self.seed.skirmish.enabled {
            SKIRMISH_TIME_LIMIT_TICKS
        } else {
            FIVE_MINUTE_TICKS
        };
        if self.party.iter().all(|unit| !unit.alive()) {
            self.outcome = Some(BattleOutcome::Defeat);
            self.phase = BattlePhase::Complete;
            self.event_count += 1;
        } else if self.human_enemy_authority && self.enemies.iter().all(|unit| !unit.alive()) {
            self.outcome = Some(BattleOutcome::Victory);
            self.phase = BattlePhase::Complete;
            self.event_count += 1;
        } else if self.tick >= time_limit {
            let player_hp: i64 = self.party.iter().map(|unit| unit.hp.max(0)).sum();
            let enemy_hp: i64 = self.enemies.iter().map(|unit| unit.hp.max(0)).sum();
            self.outcome = Some(
                if self.human_enemy_authority
                    && (player_hp > enemy_hp
                        || (player_hp == enemy_hp && self.player_score > self.enemy_score))
                {
                    BattleOutcome::Victory
                } else {
                    BattleOutcome::Defeat
                },
            );
            self.phase = BattlePhase::Complete;
            self.event_count += 1;
        } else if self.seed.skirmish.enabled {
            let terminal = match self.seed.skirmish.victory_mode {
                SkirmishVictoryMode::Objective => (self.objective_index
                    >= self.seed.mission.objectives.len())
                .then_some(BattleOutcome::Victory),
                SkirmishVictoryMode::Score => {
                    if self.player_score >= self.seed.skirmish.score_target {
                        Some(BattleOutcome::Victory)
                    } else if self.enemy_score >= self.seed.skirmish.score_target {
                        Some(BattleOutcome::Defeat)
                    } else {
                        None
                    }
                }
                SkirmishVictoryMode::Annihilation => {
                    (self.enemies.iter().all(|enemy| !enemy.alive())
                        && self
                            .enemy_structures
                            .iter()
                            .all(|structure| !structure.alive()))
                    .then_some(BattleOutcome::Victory)
                }
            };
            if let Some(outcome) = terminal {
                self.outcome = Some(outcome);
                self.phase = BattlePhase::Complete;
                self.event_count += 1;
            }
        } else if self.objective_index >= self.seed.mission.objectives.len() {
            self.outcome = Some(BattleOutcome::Victory);
            self.phase = BattlePhase::Complete;
            self.event_count += 1;
        }
        Ok(())
    }

    fn resolve_player_order(&mut self) {
        let Some(order) = self.active_order.clone() else {
            return;
        };
        let selected = order
            .subject_actor_ids
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        match order.kind {
            RtsOrderKind::Move | RtsOrderKind::AttackMove => {
                if let Some(tile) = order.target_tile {
                    self.move_selected_toward(
                        &selected,
                        BattleGridPoint::new(tile.x as i16, tile.y as i16),
                        0,
                        order.formation_id.as_deref(),
                    );
                }
                if order.kind == RtsOrderKind::AttackMove {
                    self.party_attack(&selected, order.target_actor_id.as_deref());
                }
            }
            RtsOrderKind::Attack | RtsOrderKind::FocusFire => {
                let target = self.attack_target_position(order.target_actor_id.as_deref());
                if let Some(target) = target {
                    self.move_selected_toward(&selected, target, 1, None);
                }
                self.party_attack(&selected, order.target_actor_id.as_deref());
            }
            RtsOrderKind::Patrol => self.resolve_patrol(&selected),
            RtsOrderKind::Harvest => self.resolve_harvest(&selected, &order),
            RtsOrderKind::Hold | RtsOrderKind::Capture => self.resolve_capture(&selected),
            RtsOrderKind::Ability
            | RtsOrderKind::Repair
            | RtsOrderKind::Build
            | RtsOrderKind::Recon
            | RtsOrderKind::Train
            | RtsOrderKind::Research
            | RtsOrderKind::Upgrade
            | RtsOrderKind::AssignGroup
            | RtsOrderKind::AppendGroup
            | RtsOrderKind::RemoveGroup
            | RtsOrderKind::RecallGroup
            | RtsOrderKind::CancelQueuedOrder
            | RtsOrderKind::CancelJob
            | RtsOrderKind::PauseJob
            | RtsOrderKind::ResumeJob
            | RtsOrderKind::PromoteJob
            | RtsOrderKind::SetRally
            | RtsOrderKind::Stop
            | RtsOrderKind::SetStance
            | RtsOrderKind::Extract => {}
        }
    }

    fn update_phase(&mut self) {
        if self.seed.mission.mission == trnm_campaign_core::CampaignMission::ConvoyExodus {
            self.phase = match self.current_objective_kind() {
                Some(ObjectiveKind::Escort) => BattlePhase::ConvoyEscort,
                Some(ObjectiveKind::Defend) => BattlePhase::GeneratorDefense,
                Some(ObjectiveKind::Extract) => BattlePhase::Extraction,
                _ if self.objective_index >= self.seed.mission.objectives.len() => {
                    BattlePhase::Complete
                }
                _ => self.phase,
            };
            return;
        }
        if self.phase == BattlePhase::Approach
            && self
                .party
                .iter()
                .filter(|unit| unit.alive())
                .any(|unit| distance(unit.position, self.seed.map.approach_point) <= 2)
        {
            self.phase = BattlePhase::Contact;
            self.objective_index = 1;
            self.event_count += 1;
        }
        if self.phase == BattlePhase::Contact && self.enemies.iter().all(|unit| !unit.alive()) {
            self.phase = BattlePhase::Relay;
            self.event_count += 1;
        }
        if self.objective_index == 1 && self.relay_guard_hp <= 0 {
            self.objective_index = 2;
            self.objective_progress_ticks = self.relay_capture_ticks;
        }
    }

    pub fn current_objective_kind(&self) -> Option<ObjectiveKind> {
        self.seed
            .mission
            .objectives
            .get(self.objective_index)
            .map(|objective| objective.kind)
    }

    pub fn current_objective_id(&self) -> Option<&str> {
        self.seed
            .mission
            .objectives
            .get(self.objective_index)
            .map(|objective| objective.id.as_str())
    }

}

