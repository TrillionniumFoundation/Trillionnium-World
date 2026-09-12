impl MissionSimV1 {
    fn resolve_mission_objective(&mut self) {
        let Some(objective) = self
            .seed
            .mission
            .objectives
            .get(self.objective_index)
            .cloned()
        else {
            return;
        };
        if self.seed.mission.mission != trnm_campaign_core::CampaignMission::ConvoyExodus {
            if objective.kind == ObjectiveKind::Capture {
                self.objective_progress_ticks = self.relay_capture_ticks;
                if self.relay_capture_ticks >= objective.duration_ticks {
                    self.objective_index += 1;
                }
            }
            return;
        }

        match objective.kind {
            ObjectiveKind::Escort | ObjectiveKind::Extract => {
                let Some(position) = self.convoy_position else {
                    return;
                };
                let escort_ordered = self.active_order.as_ref().is_some_and(|order| {
                    matches!(order.kind, RtsOrderKind::Move | RtsOrderKind::AttackMove)
                        && order.target_tile.is_some_and(|tile| {
                            BattleGridPoint::new(tile.x as i16, tile.y as i16) == objective.target
                        })
                });
                let escorted = escort_ordered
                    || self.party.iter().filter(|unit| unit.alive()).any(|unit| {
                        distance(unit.position, position) <= 3
                            || distance(unit.position, objective.target) <= 3
                    });
                if escorted && self.tick.is_multiple_of(8) && position != objective.target {
                    let occupied = self
                        .party
                        .iter()
                        .chain(&self.enemies)
                        .filter(|unit| unit.alive())
                        .map(|unit| unit.position)
                        .chain(self.support_units.iter().map(|unit| unit.position))
                        .collect::<BTreeSet<_>>();
                    if let Some(next) =
                        next_step_toward(&self.seed, position, objective.target, 0, &occupied)
                    {
                        self.convoy_position = Some(next);
                        self.event_count += 1;
                    }
                }
                if self.convoy_position == Some(objective.target) {
                    if objective.kind == ObjectiveKind::Escort {
                        self.objective_index += 1;
                        self.objective_progress_ticks = 0;
                    } else if escorted {
                        self.objective_progress_ticks =
                            self.objective_progress_ticks.saturating_add(1);
                        if self.objective_progress_ticks >= objective.duration_ticks {
                            self.objective_index += 1;
                        }
                    }
                }
            }
            ObjectiveKind::Defend => {
                let defenders = self
                    .party
                    .iter()
                    .filter(|unit| unit.alive() && distance(unit.position, objective.target) <= 4)
                    .count();
                if defenders > 0 {
                    self.objective_progress_ticks = self.objective_progress_ticks.saturating_add(1);
                }
                if self.objective_progress_ticks == 1 || self.objective_progress_ticks == 130 {
                    self.spawn_reinforcement_wave(true);
                }
                if self.objective_progress_ticks >= objective.duration_ticks {
                    self.objective_index += 1;
                    self.objective_progress_ticks = 0;
                }
            }
            ObjectiveKind::Destroy | ObjectiveKind::Capture => {}
        }
    }

    fn move_selected_toward(
        &mut self,
        selected: &BTreeSet<String>,
        target: BattleGridPoint,
        stop_range: i16,
        formation_id: Option<&str>,
    ) {
        self.tile_reservations.clear();
        let mut occupied = self
            .party
            .iter()
            .chain(&self.enemies)
            .filter(|unit| unit.alive())
            .map(|unit| unit.position)
            .chain(self.support_units.iter().map(|unit| unit.position))
            .collect::<BTreeSet<_>>();
        let mut indices = (0..self.party.len())
            .filter(|index| {
                self.party[*index].alive() && selected.contains(&self.party[*index].unit_id)
            })
            .collect::<Vec<_>>();
        indices.sort_by(|left, right| {
            let blocked = |index: usize| {
                self.move_intents
                    .get(&self.party[index].unit_id)
                    .map(|intent| intent.blocked_ticks)
                    .unwrap_or(0)
            };
            blocked(*right)
                .cmp(&blocked(*left))
                .then_with(|| self.party[*left].unit_id.cmp(&self.party[*right].unit_id))
        });
        for index in indices {
            self.party[index].movement_budget_milli += self.party[index].move_speed_milli;
            if self.party[index].movement_budget_milli < MOVEMENT_TILE_COST {
                continue;
            }
            occupied.remove(&self.party[index].position);
            let formation_target =
                formation_target_for(target, index, formation_id.unwrap_or("none"), &self.seed);
            let previous = self.move_intents.get(&self.party[index].unit_id);
            let mut blocked_ticks = previous.map(|intent| intent.blocked_ticks).unwrap_or(0);
            let mut replan_count = previous.map(|intent| intent.replan_count).unwrap_or(0);
            let mut next = next_step_toward(
                &self.seed,
                self.party[index].position,
                formation_target,
                stop_range,
                &occupied,
            );
            if next.is_none() && distance(self.party[index].position, formation_target) > stop_range
            {
                blocked_ticks = blocked_ticks.saturating_add(1);
                if blocked_ticks >= 6 {
                    next = deterministic_yield_step(
                        &self.seed,
                        self.party[index].position,
                        formation_target,
                        &occupied,
                        &self.tile_reservations,
                    );
                    replan_count = replan_count.saturating_add(1);
                    blocked_ticks = 0;
                }
            } else if next.is_some() {
                blocked_ticks = 0;
            }
            if next.is_some_and(|candidate| {
                self.tile_reservations
                    .iter()
                    .any(|reservation| reservation.tile == candidate)
            }) {
                next = None;
                blocked_ticks = blocked_ticks.saturating_add(1);
            }
            self.move_intents.insert(
                self.party[index].unit_id.clone(),
                MoveIntent {
                    unit_id: self.party[index].unit_id.clone(),
                    target: formation_target,
                    desired_tile: next,
                    blocked_ticks,
                    replan_count,
                },
            );
            if let Some(next) = next {
                self.party[index].position = next;
                self.party[index].movement_budget_milli -= MOVEMENT_TILE_COST;
                self.tile_reservations.push(TileReservation {
                    tile: next,
                    unit_id: self.party[index].unit_id.clone(),
                });
                occupied.insert(next);
            } else {
                occupied.insert(self.party[index].position);
            }
        }
    }

    fn attack_target_position(&self, requested: Option<&str>) -> Option<BattleGridPoint> {
        if let Some(target_id) = requested {
            if let Some(enemy) = self
                .enemies
                .iter()
                .find(|enemy| enemy.unit_id == target_id && enemy.alive())
            {
                return Some(enemy.position);
            }
            if let Some(structure) = self
                .enemy_structures
                .iter()
                .find(|structure| structure.structure_id == target_id && structure.alive())
            {
                return Some(structure.position);
            }
        }
        self.enemies
            .iter()
            .find(|enemy| enemy.alive())
            .map(|enemy| enemy.position)
            .or_else(|| {
                self.enemy_structures
                    .iter()
                    .find(|structure| structure.alive())
                    .map(|structure| structure.position)
            })
            .or(Some(self.seed.map.objective))
    }

    fn set_unit_stance(&mut self, order: &RtsFrameOrder) -> Result<(), SimError> {
        let stance = order
            .target_rule_id
            .as_deref()
            .and_then(RtsUnitStance::from_rule_id)
            .ok_or_else(|| SimError::Order("unit stance is invalid".to_string()))?;
        for unit in &mut self.party {
            if order.subject_actor_ids.contains(&unit.unit_id) {
                unit.stance = stance;
            }
        }
        Ok(())
    }

    fn resolve_patrol(&mut self, selected: &BTreeSet<String>) {
        let outward = self
            .party
            .iter()
            .filter(|unit| {
                unit.alive() && selected.contains(&unit.unit_id) && !unit.patrol_returning
            })
            .map(|unit| unit.unit_id.clone())
            .collect::<BTreeSet<_>>();
        let returning = self
            .party
            .iter()
            .filter(|unit| {
                unit.alive() && selected.contains(&unit.unit_id) && unit.patrol_returning
            })
            .map(|unit| unit.unit_id.clone())
            .collect::<BTreeSet<_>>();
        let target = self
            .party
            .iter()
            .find_map(|unit| unit.patrol_target)
            .unwrap_or(self.seed.map.approach_point);
        if !outward.is_empty() {
            self.move_selected_toward(&outward, target, 0, None);
        }
        if !returning.is_empty() {
            self.move_selected_toward(&returning, self.seed.map.party_start, 0, None);
        }
        for unit in &mut self.party {
            if !selected.contains(&unit.unit_id) {
                continue;
            }
            let destination = if unit.patrol_returning {
                self.seed.map.party_start
            } else {
                unit.patrol_target.unwrap_or(target)
            };
            if distance(unit.position, destination) <= 1 {
                unit.patrol_returning = !unit.patrol_returning;
            }
        }
    }

    fn resolve_stance_fire(&mut self) {
        if matches!(
            self.current_order_kind(),
            RtsOrderKind::Attack | RtsOrderKind::FocusFire | RtsOrderKind::AttackMove
        ) {
            return;
        }
        let guard = self
            .party
            .iter()
            .filter(|unit| unit.alive() && unit.stance == RtsUnitStance::Guard)
            .filter(|unit| {
                self.enemies.iter().any(|enemy| {
                    enemy.alive()
                        && self.visible_tiles.contains(&enemy.position)
                        && distance(unit.position, enemy.position) <= unit.attack_range()
                })
            })
            .map(|unit| unit.unit_id.clone())
            .collect::<BTreeSet<_>>();
        if !guard.is_empty() {
            self.party_attack(&guard, None);
        }
        let aggressive = self
            .party
            .iter()
            .filter(|unit| unit.alive() && unit.stance == RtsUnitStance::Aggressive)
            .map(|unit| unit.unit_id.clone())
            .collect::<BTreeSet<_>>();
        let target = self
            .enemies
            .iter()
            .filter(|enemy| enemy.alive() && self.visible_tiles.contains(&enemy.position))
            .min_by_key(|enemy| {
                self.party
                    .iter()
                    .filter(|unit| aggressive.contains(&unit.unit_id))
                    .map(|unit| distance(unit.position, enemy.position))
                    .min()
                    .unwrap_or(i16::MAX)
            })
            .map(|enemy| (enemy.unit_id.clone(), enemy.position));
        if let Some((target_id, position)) = target {
            self.move_selected_toward(&aggressive, position, 1, None);
            self.party_attack(&aggressive, Some(&target_id));
        }
    }

    fn party_attack(&mut self, selected: &BTreeSet<String>, requested: Option<&str>) {
        for attacker_index in 0..self.party.len() {
            if !self.party[attacker_index].alive()
                || !selected.contains(&self.party[attacker_index].unit_id)
                || !self
                    .tick
                    .is_multiple_of(self.party[attacker_index].attack_interval_ticks as u64)
            {
                continue;
            }
            let attacker = &self.party[attacker_index];
            let requested_structure = requested
                .and_then(|id| {
                    self.enemy_structures
                        .iter()
                        .position(|structure| structure.structure_id == id && structure.alive())
                })
                .or_else(|| {
                    let siege_capable = self.party[attacker_index]
                        .skill_ids
                        .iter()
                        .find_map(|skill| UnitAbility::from_rule_id(skill))
                        .is_some_and(|ability| {
                            matches!(
                                ability,
                                UnitAbility::ArcVolley
                                    | UnitAbility::PiercingCharge
                                    | UnitAbility::DemolitionCharge
                            )
                        });
                    (self.current_order_kind() == RtsOrderKind::AttackMove && siege_capable)
                        .then(|| {
                            self.enemy_structures
                                .iter()
                                .enumerate()
                                .filter(|(_, structure)| structure.alive())
                                .filter(|(_, structure)| {
                                    distance(
                                        self.party[attacker_index].position,
                                        structure.position,
                                    ) <= self.party[attacker_index].attack_range() + 1
                                })
                                .min_by_key(|(_, structure)| {
                                    let priority = match structure.kind {
                                        SimStructureKind::CommandPost => 0,
                                        SimStructureKind::FieldWorkshop => 1,
                                        _ => 2,
                                    };
                                    (priority, structure.hp)
                                })
                                .map(|(index, _)| index)
                        })
                        .flatten()
                });
            let target_index = requested
                .and_then(|id| {
                    self.enemies
                        .iter()
                        .position(|enemy| enemy.unit_id == id && enemy.alive())
                })
                .or_else(|| {
                    if requested_structure.is_some() {
                        return None;
                    }
                    self.enemies
                        .iter()
                        .enumerate()
                        .filter(|(_, enemy)| enemy.alive())
                        .min_by_key(|(_, enemy)| distance(attacker.position, enemy.position))
                        .map(|(index, _)| index)
                });
            if let Some(target_index) = target_index {
                if distance(
                    self.party[attacker_index].position,
                    self.enemies[target_index].position,
                ) <= self.party[attacker_index].attack_range()
                {
                    let ability = self.party[attacker_index]
                        .skill_ids
                        .iter()
                        .find_map(|skill| UnitAbility::from_rule_id(skill));
                    let ability_ready =
                        ability.is_some() && self.party[attacker_index].ability_cooldown_ticks == 0;
                    let intel_bonus = if self.recon_bonus_ticks > 0 {
                        i64::from(self.intel_level) * 2
                    } else {
                        0
                    };
                    let veteran_bonus = i64::from(self.party[attacker_index].veteran_rank) * 2;
                    let damage = (self.party[attacker_index].damage + intel_bonus + veteran_bonus
                        - self.enemies[target_index].armor)
                        .max(1);
                    let was_alive = self.enemies[target_index].alive();
                    if !deterministic_evade(
                        self.tick,
                        target_index + simulation_salt(&self.seed) as usize,
                        self.enemies[target_index].evasion_permille,
                    ) {
                        self.enemies[target_index].hp -= damage;
                    }
                    self.party[attacker_index].attacks_made += 1;
                    if was_alive && !self.enemies[target_index].alive() {
                        self.player_score = self.player_score.saturating_add(100);
                        self.party[attacker_index].confirmed_kills =
                            self.party[attacker_index].confirmed_kills.saturating_add(1);
                        self.party[attacker_index].veteran_rank =
                            match self.party[attacker_index].confirmed_kills {
                                0..=1 => self.party[attacker_index].veteran_rank,
                                2..=4 => self.party[attacker_index].veteran_rank.max(1),
                                5..=8 => self.party[attacker_index].veteran_rank.max(2),
                                _ => 3,
                            };
                    }
                    if ability_ready {
                        self.activate_player_ability(
                            attacker_index,
                            target_index,
                            ability.expect("ready player ability exists"),
                        );
                    }
                    self.event_count += 1;
                }
            } else if let Some(structure_index) = requested_structure.or_else(|| {
                self.enemy_structures
                    .iter()
                    .enumerate()
                    .filter(|(_, structure)| structure.alive())
                    .min_by_key(|(_, structure)| {
                        distance(self.party[attacker_index].position, structure.position)
                    })
                    .map(|(index, _)| index)
            }) {
                if distance(
                    self.party[attacker_index].position,
                    self.enemy_structures[structure_index].position,
                ) <= self.party[attacker_index].attack_range() + 1
                {
                    let ability = self.party[attacker_index]
                        .skill_ids
                        .iter()
                        .find_map(|skill| UnitAbility::from_rule_id(skill));
                    let ability_ready =
                        ability.is_some() && self.party[attacker_index].ability_cooldown_ticks == 0;
                    let siege_bonus = match ability.filter(|_| ability_ready) {
                        Some(UnitAbility::DemolitionCharge) => 55,
                        Some(UnitAbility::PiercingCharge) => 35,
                        Some(UnitAbility::ArcVolley) => 55,
                        _ => 0,
                    };
                    self.enemy_structures[structure_index].hp -=
                        (self.party[attacker_index].damage + siege_bonus).max(1);
                    if ability_ready {
                        self.party[attacker_index].ability_cooldown_ticks = 45;
                    }
                    self.party[attacker_index].attacks_made += 1;
                    self.event_count += 1;
                }
            } else if self.current_objective_kind() == Some(ObjectiveKind::Destroy)
                && distance(self.party[attacker_index].position, self.seed.map.objective)
                    <= self.party[attacker_index].attack_range() + 1
                && self.relay_guard_hp > 0
            {
                let resource_bonus = i64::from((self.resources_gathered / 40).min(3));
                self.relay_guard_hp -= (self.party[attacker_index].damage + resource_bonus).max(1);
                self.party[attacker_index].attacks_made += 1;
                self.event_count += 1;
            }
        }
    }

    fn activate_player_ability(
        &mut self,
        attacker_index: usize,
        target_index: usize,
        ability: UnitAbility,
    ) {
        let attacker_position = self.party[attacker_index].position;
        self.party[attacker_index].ability_cooldown_ticks = 45;
        match ability {
            UnitAbility::RevealPulse => {
                self.intel_level = self.intel_level.max(3);
                self.recon_bonus_ticks = self.recon_bonus_ticks.max(120);
            }
            UnitAbility::GuardWall => {
                for unit in &mut self.party {
                    if unit.alive() && distance(unit.position, attacker_position) <= 3 {
                        unit.guard_ticks = unit.guard_ticks.max(30);
                    }
                }
            }
            UnitAbility::ArcVolley => {
                if let Some((index, _)) = self
                    .enemies
                    .iter()
                    .enumerate()
                    .filter(|(index, unit)| *index != target_index && unit.alive())
                    .min_by_key(|(_, unit)| distance(unit.position, attacker_position))
                {
                    self.enemies[index].hp -= (self.party[attacker_index].damage / 2).max(1);
                }
            }
            UnitAbility::FieldRepair => {
                if let Some(structure) = self
                    .structures
                    .iter_mut()
                    .filter(|structure| structure.alive())
                    .min_by_key(|structure| distance(structure.position, attacker_position))
                {
                    structure.hp = (structure.hp + 60).min(structure.max_hp);
                }
            }
            UnitAbility::TriageAura => {
                for unit in &mut self.party {
                    if unit.alive() && distance(unit.position, attacker_position) <= 3 {
                        unit.hp = (unit.hp + 45).min(unit.max_hp);
                    }
                }
            }
            UnitAbility::SuppressionBlast => {
                self.enemies[target_index].energy =
                    self.enemies[target_index].energy.saturating_sub(18);
                self.enemies[target_index].ability_cooldown_ticks = self.enemies[target_index]
                    .ability_cooldown_ticks
                    .saturating_add(20);
            }
            UnitAbility::SmokeDash => {
                self.party[attacker_index].evasion_permille =
                    self.party[attacker_index].evasion_permille.max(180);
                self.party[attacker_index].movement_budget_milli = self.party[attacker_index]
                    .movement_budget_milli
                    .saturating_add(MOVEMENT_TILE_COST);
            }
            UnitAbility::RetaliationPlate => {
                self.party[attacker_index].guard_ticks = 50;
                self.enemies[target_index].hp -= 8;
            }
            UnitAbility::PiercingCharge => {
                self.enemies[target_index].guard_ticks = 0;
            }
            UnitAbility::DemolitionCharge => {
                if let Some(structure) = self
                    .enemy_structures
                    .iter_mut()
                    .filter(|structure| structure.alive())
                    .min_by_key(|structure| distance(structure.position, attacker_position))
                {
                    structure.hp -= 55;
                }
            }
            UnitAbility::SignalJam => {
                self.enemy_ai_budget = self.enemy_ai_budget.saturating_sub(8);
                self.enemies[target_index].ability_cooldown_ticks = self.enemies[target_index]
                    .ability_cooldown_ticks
                    .saturating_add(35);
            }
            UnitAbility::CommandSurge => {
                for unit in &mut self.party {
                    if unit.alive() && distance(unit.position, attacker_position) <= 4 {
                        unit.movement_budget_milli = unit
                            .movement_budget_milli
                            .saturating_add(MOVEMENT_TILE_COST / 2);
                        unit.guard_ticks = unit.guard_ticks.max(15);
                    }
                }
            }
        }
        self.event_count += 1;
    }

    fn resolve_harvest(&mut self, selected: &BTreeSet<String>, order: &RtsFrameOrder) {
        let preferred_node_index = order.target_actor_id.as_deref().and_then(|id| {
            self.resource_nodes
                .iter()
                .position(|node| node.node_id == id)
        });
        advance_worker_logistics(
            &self.seed,
            self.tick,
            &mut self.party,
            &self.enemies,
            &self.jobs,
            &mut self.resource_nodes,
            self.seed.map.party_start,
            Some(selected),
            preferred_node_index,
            &mut self.resources_available,
            &mut self.resources_gathered,
            &mut self.player_score,
            &mut self.event_count,
            true,
        );
    }
    fn resolve_capture(&mut self, selected: &BTreeSet<String>) {
        if self.current_objective_kind() != Some(ObjectiveKind::Capture)
            || self.relay_guard_hp > 0
            || self.enemies.iter().any(SimUnit::alive)
        {
            return;
        }
        let holders = self
            .party
            .iter()
            .filter(|unit| {
                unit.alive()
                    && selected.contains(&unit.unit_id)
                    && distance(unit.position, self.seed.map.objective) <= 2
            })
            .count() as u32;
        if holders > 0 {
            self.relay_capture_ticks = self.relay_capture_ticks.saturating_add(holders.min(2));
            let aftershock = is_aftershock_map(&self.seed.map_id);
            let thresholds: &[u32] = &[200, 400];
            if let Some(threshold) = thresholds.get(self.reinforcement_wave as usize) {
                if self.relay_capture_ticks >= *threshold {
                    self.spawn_reinforcement_wave(aftershock);
                }
            }
        }
    }

    fn resolve_field_aid(&mut self, order: &RtsFrameOrder) -> Result<(), SimError> {
        self.spend_resources(FIELD_AID_COST, "field aid")?;
        let selected = order
            .subject_actor_ids
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        for unit in &mut self.party {
            if unit.alive() && selected.contains(unit.unit_id.as_str()) {
                unit.hp = (unit.hp + 110).min(unit.max_hp);
            }
        }
        self.event_count += 1;
        Ok(())
    }

    fn resolve_repair(&mut self, order: &RtsFrameOrder) -> Result<(), SimError> {
        if let Some(index) = order.target_actor_id.as_deref().and_then(|target| {
            self.structures
                .iter()
                .position(|structure| structure.structure_id == target && structure.alive())
        }) {
            if self.structures[index].hp >= self.structures[index].max_hp {
                return Err(SimError::Order(
                    "structure is already fully repaired".to_string(),
                ));
            }
            self.spend_resources(10, "structure repair")?;
            self.structures[index].hp =
                (self.structures[index].hp + 180).min(self.structures[index].max_hp);
            self.event_count += 1;
            Ok(())
        } else {
            self.resolve_field_aid(order)
        }
    }

}

impl MissionSimV1 {
    fn resolve_fortify(&mut self, order: &RtsFrameOrder) -> Result<(), SimError> {
        let rule = order.target_rule_id.as_deref().unwrap_or("field_barricade");
        let kind = SimStructureKind::from_rule_id(rule)
            .ok_or_else(|| SimError::Order(format!("unknown structure rule {rule}")))?;
        if kind == SimStructureKind::CommandPost {
            return Err(SimError::Order(
                "additional command posts cannot be field-built".to_string(),
            ));
        }
        let definition = kind.definition();
        if definition
            .faction
            .is_some_and(|faction| faction != self.seed.skirmish.player_faction)
        {
            return Err(SimError::Order(format!(
                "{} belongs to the opposing faction",
                definition.id
            )));
        }
        let base_cost = definition.cost;
        let target = order
            .target_tile
            .map(|tile| BattleGridPoint::new(tile.x as i16, tile.y as i16))
            .ok_or_else(|| SimError::Order("structure target is missing".to_string()))?;
        let in_build_radius = self
            .structures
            .iter()
            .filter(|structure| structure.alive())
            .any(|structure| distance(structure.position, target) <= 8)
            || self.party.iter().any(|unit| {
                unit.alive()
                    && order.subject_actor_ids.contains(&unit.unit_id)
                    && distance(unit.position, target) <= 6
            });
        if !in_build_radius {
            return Err(SimError::Order(
                "structure target is outside build radius".to_string(),
            ));
        }
        if self
            .structures
            .iter()
            .any(|structure| structure.alive() && structure.position == target)
            || self
                .jobs
                .iter()
                .any(|job| job.kind == SimJobKind::BuildStructure && job.target == target)
            || self
                .enemies
                .iter()
                .any(|enemy| enemy.alive() && enemy.position == target)
        {
            return Err(SimError::Order(
                "structure target tile is occupied".to_string(),
            ));
        }
        let cost = ((base_cost * u32::from(self.seed.field_build_cost_permille)) / 1000).max(1);
        let selected = order
            .subject_actor_ids
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let builder_id = self
            .party
            .iter()
            .filter(|unit| unit.alive() && selected.contains(unit.unit_id.as_str()))
            .min_by_key(|unit| distance(unit.position, target))
            .map(|unit| unit.unit_id.clone())
            .ok_or_else(|| {
                SimError::Order("construction requires a living selected builder".to_string())
            })?;
        let duration = 45 + definition.hp / 100;
        self.submit_authority_job(
            AuthorityCommandSource::PlayerOrder,
            SimJob {
                job_id: format!("build-{rule}-{}", self.event_count + 1),
                kind: SimJobKind::BuildStructure,
                rule_id: rule.to_string(),
                remaining_ticks: duration,
                target,
                cost,
                paused: false,
                builder_id: Some(builder_id),
                side: AuthoritySide::Player,
            },
            "structure construction",
        )?;
        // Builder guard is part of the accepted construction command. Publish
        // it only after the authority job has passed validation and resource
        // admission, so a rejected build leaves cooldown/guard state untouched.
        for unit in &mut self.party {
            if unit.alive() && selected.contains(unit.unit_id.as_str()) {
                unit.guard_ticks = unit.guard_ticks.max(240);
            }
        }
        Ok(())
    }

    fn resolve_recon(&mut self, order: &RtsFrameOrder) -> Result<(), SimError> {
        self.spend_resources(RECON_COST, "recon sweep")?;
        self.intel_level = self.intel_level.saturating_add(1).min(3);
        self.recon_bonus_ticks = 300;
        self.recon_focus = order
            .target_tile
            .map(|tile| BattleGridPoint::new(tile.x as i16, tile.y as i16));
        self.refresh_visibility();
        self.event_count += 1;
        Ok(())
    }

    fn queue_job_from_order(&mut self, order: &RtsFrameOrder) -> Result<(), SimError> {
        let rule = order.target_rule_id.as_deref().unwrap_or_default();
        let kind = match (order.kind, rule) {
            (RtsOrderKind::Train, "field_medic") => SimJobKind::TrainMedic,
            (RtsOrderKind::Train, "field_support_drone") => SimJobKind::TrainSupport,
            (RtsOrderKind::Train, rule) if UNIT_ROSTER.iter().any(|unit| unit.id == rule) => {
                SimJobKind::TrainRosterUnit
            }
            (RtsOrderKind::Research, "signal_optics") => SimJobKind::ResearchOptics,
            (RtsOrderKind::Research, "sensor_net") => SimJobKind::ResearchSensorNet,
            (RtsOrderKind::Research, "field_medicine") => SimJobKind::ResearchFieldMedicine,
            (RtsOrderKind::Research, "field_logistics") => SimJobKind::ResearchLogistics,
            (RtsOrderKind::Research, "wayfinder_drills") => SimJobKind::ResearchWayfinderDrills,
            (RtsOrderKind::Research, "rapid_mustering") => SimJobKind::ResearchRapidMustering,
            (RtsOrderKind::Upgrade, "field_armor") => SimJobKind::UpgradeFieldArmor,
            (RtsOrderKind::Upgrade, "siege_drills") => SimJobKind::UpgradeSiegeDrills,
            (RtsOrderKind::Upgrade, "reactive_plating") => SimJobKind::UpgradeReactivePlating,
            (RtsOrderKind::Upgrade, "relay_arms") => SimJobKind::UpgradeRelayArms,
            _ => return Err(SimError::Order("unsupported job order".to_string())),
        };
        self.queue_job(order, kind)
    }

    fn queue_job(&mut self, order: &RtsFrameOrder, kind: SimJobKind) -> Result<(), SimError> {
        let rule_id = order
            .target_rule_id
            .clone()
            .ok_or_else(|| SimError::Order("job rule is required".to_string()))?;
        if !matches!(
            kind,
            SimJobKind::TrainSupport | SimJobKind::TrainMedic | SimJobKind::TrainRosterUnit
        ) && self.jobs.iter().any(|job| job.kind == kind)
        {
            return Err(SimError::Order(
                "that production or technology job is already queued".to_string(),
            ));
        }
        let requested_supply = match kind {
            SimJobKind::TrainSupport | SimJobKind::TrainMedic => 1,
            SimJobKind::TrainRosterUnit => UNIT_ROSTER
                .iter()
                .find(|unit| unit.id == rule_id)
                .map(|unit| unit.supply)
                .unwrap_or(1),
            _ => 0,
        };
        if requested_supply > 0
            && self.supply_used().saturating_add(requested_supply) > self.supply_cap()
        {
            return Err(SimError::Order(
                "unit production is supply blocked".to_string(),
            ));
        }
        let workshop_ready = self.structures.iter().any(|structure| {
            structure.alive() && structure.kind == SimStructureKind::FieldWorkshop
        });
        if matches!(
            kind,
            SimJobKind::TrainMedic
                | SimJobKind::TrainRosterUnit
                | SimJobKind::ResearchOptics
                | SimJobKind::UpgradeRelayArms
                | SimJobKind::UpgradeFieldArmor
                | SimJobKind::ResearchSensorNet
                | SimJobKind::ResearchFieldMedicine
                | SimJobKind::UpgradeSiegeDrills
                | SimJobKind::UpgradeReactivePlating
                | SimJobKind::ResearchWayfinderDrills
                | SimJobKind::ResearchRapidMustering
        ) && !workshop_ready
        {
            return Err(SimError::Order(
                "a powered field workshop prerequisite is missing".to_string(),
            ));
        }
        let tech_definition = match kind {
            SimJobKind::ResearchLogistics
            | SimJobKind::ResearchOptics
            | SimJobKind::UpgradeRelayArms
            | SimJobKind::UpgradeFieldArmor
            | SimJobKind::ResearchSensorNet
            | SimJobKind::ResearchFieldMedicine
            | SimJobKind::UpgradeSiegeDrills
            | SimJobKind::UpgradeReactivePlating
            | SimJobKind::ResearchWayfinderDrills
            | SimJobKind::ResearchRapidMustering => Some(
                TECH_TREE
                    .iter()
                    .find(|tech| tech.id == rule_id)
                    .ok_or_else(|| SimError::Order(format!("unknown technology {rule_id}")))?,
            ),
            _ => None,
        };
        if let Some(tech) = tech_definition {
            if tech
                .faction
                .is_some_and(|faction| faction != self.seed.skirmish.player_faction)
            {
                return Err(SimError::Order(format!(
                    "{} belongs to the opposing faction",
                    tech.id
                )));
            }
            if tech
                .prerequisite
                .is_some_and(|required| !self.researched_techs.contains(required))
            {
                return Err(SimError::Order(format!(
                    "research {} before {}",
                    tech.prerequisite.unwrap_or_default(),
                    tech.id
                )));
            }
        }
        let tech_cost = tech_definition.map(|tech| tech.cost).unwrap_or_default();
        let (cost, duration, label) = match kind {
            SimJobKind::BuildStructure => {
                return Err(SimError::Order(
                    "structures use the shared construction authority".to_string(),
                ));
            }
            SimJobKind::TrainSupport => (TRAIN_SUPPORT_COST, 80, "support production"),
            SimJobKind::TrainMedic => (TRAIN_SUPPORT_COST + 10, 95, "field medic production"),
            SimJobKind::TrainRosterUnit => {
                let unit = UNIT_ROSTER
                    .iter()
                    .find(|unit| unit.id == rule_id)
                    .ok_or_else(|| SimError::Order("unknown faction unit".to_string()))?;
                if unit.faction != self.seed.skirmish.player_faction {
                    return Err(SimError::Order(format!(
                        "{} belongs to the opposing faction",
                        unit.id
                    )));
                }
                let duration = if self.researched_techs.contains("rapid_mustering") {
                    60
                } else {
                    90
                };
                (unit.cost, duration, "faction roster production")
            }
            SimJobKind::ResearchLogistics => {
                if self.researched_techs.contains("field_logistics") {
                    return Err(SimError::Order(
                        "field logistics is already researched".to_string(),
                    ));
                }
                (tech_cost, 70, "field logistics research")
            }
            SimJobKind::ResearchOptics => {
                if self.researched_techs.contains("signal_optics") {
                    return Err(SimError::Order(
                        "signal optics is already researched".to_string(),
                    ));
                }
                (tech_cost, 90, "signal optics research")
            }
            SimJobKind::UpgradeRelayArms => {
                if self.upgrade_level >= 3 {
                    return Err(SimError::Order(
                        "relay arms upgrade cap reached".to_string(),
                    ));
                }
                (tech_cost, 60, "relay arms upgrade")
            }
            SimJobKind::UpgradeFieldArmor => {
                if self.armor_upgrade_level >= 3 {
                    return Err(SimError::Order(
                        "field armor upgrade cap reached".to_string(),
                    ));
                }
                (tech_cost, 75, "field armor upgrade")
            }
            SimJobKind::ResearchSensorNet => {
                if self.researched_techs.contains("sensor_net") {
                    return Err(SimError::Order(
                        "sensor net is already researched".to_string(),
                    ));
                }
                (tech_cost, 100, "sensor net research")
            }
            SimJobKind::ResearchFieldMedicine => {
                if self.researched_techs.contains("field_medicine") {
                    return Err(SimError::Order(
                        "field medicine is already researched".to_string(),
                    ));
                }
                (tech_cost, 100, "field medicine research")
            }
            SimJobKind::UpgradeSiegeDrills => (tech_cost, 90, "siege drills upgrade"),
            SimJobKind::UpgradeReactivePlating => (tech_cost, 90, "reactive plating upgrade"),
            SimJobKind::ResearchWayfinderDrills => {
                if self.researched_techs.contains("wayfinder_drills") {
                    return Err(SimError::Order(
                        "wayfinder drills are already researched".to_string(),
                    ));
                }
                (tech_cost, 80, "wayfinder drills research")
            }
            SimJobKind::ResearchRapidMustering => {
                if self.researched_techs.contains("rapid_mustering") {
                    return Err(SimError::Order(
                        "rapid mustering is already researched".to_string(),
                    ));
                }
                (tech_cost, 80, "rapid mustering research")
            }
        };
        let target = order
            .target_tile
            .map(|tile| BattleGridPoint::new(tile.x as i16, tile.y as i16))
            .unwrap_or(self.seed.map.party_start);
        self.submit_authority_job(
            AuthorityCommandSource::PlayerOrder,
            SimJob {
                job_id: format!(
                    "{}-{}",
                    order.queue_id.as_deref().unwrap_or("field"),
                    self.tick
                ),
                kind,
                rule_id,
                remaining_ticks: duration,
                target,
                cost,
                paused: false,
                builder_id: None,
                side: AuthoritySide::Player,
            },
            label,
        )
    }

    fn job_index(&self, order: &RtsFrameOrder) -> Result<usize, SimError> {
        let job_id = order.queue_id.as_deref().unwrap_or_default();
        self.jobs
            .iter()
            .position(|job| job.job_id == job_id)
            .ok_or_else(|| SimError::Order(format!("job {job_id} was not found")))
    }

    fn cancel_job(&mut self, order: &RtsFrameOrder) -> Result<(), SimError> {
        let index = self.job_index(order)?;
        let job = self.jobs.remove(index);
        let refund = job.cost / 2;
        self.resources_spent = self.resources_spent.saturating_sub(refund);
        self.resources_available = self.resources_available.saturating_add(refund);
        Ok(())
    }

    fn set_job_paused(&mut self, order: &RtsFrameOrder, paused: bool) -> Result<(), SimError> {
        let index = self.job_index(order)?;
        if self.jobs[index].paused == paused {
            return Err(SimError::Order("job pause state is unchanged".to_string()));
        }
        self.jobs[index].paused = paused;
        Ok(())
    }

    fn promote_job(&mut self, order: &RtsFrameOrder) -> Result<(), SimError> {
        let index = self.job_index(order)?;
        if index == 0 {
            return Err(SimError::Order("job is already first in queue".to_string()));
        }
        let job = self.jobs.remove(index);
        self.jobs.insert(0, job);
        Ok(())
    }

    fn set_rally(&mut self, order: &RtsFrameOrder) -> Result<(), SimError> {
        let index = self.job_index(order)?;
        let tile = order.target_tile.expect("validated rally tile");
        let target = BattleGridPoint::new(tile.x as i16, tile.y as i16);
        if !self.seed.map.passable(target) {
            return Err(SimError::Order("rally point is blocked".to_string()));
        }
        self.jobs[index].target = target;
        Ok(())
    }

    fn apply_player_job_completion(&mut self, job: SimJob) {
        match job.kind {
            SimJobKind::BuildStructure => {
                let kind = SimStructureKind::from_rule_id(&job.rule_id)
                    .expect("queued player structure remains catalogued");
                let definition = kind.definition();
                self.structures.push(SimStructure {
                    structure_id: format!("{}-{}", job.rule_id, self.event_count + 1),
                    kind,
                    position: job.target,
                    hp: i64::from(definition.hp),
                    max_hp: i64::from(definition.hp),
                });
            }
            SimJobKind::TrainSupport => {
                let position = self.unoccupied_spawn_tile(job.target);
                self.support_units.push(SupportUnit {
                    unit_id: format!("field_support_{}", self.support_units.len() + 1),
                    archetype_id: "field_support_drone".to_string(),
                    role: "support".to_string(),
                    position,
                    hp: 240,
                    damage: 18 + i64::from(self.upgrade_level) * 5,
                    armor: 1,
                    attack_range: 4,
                    ability_cooldown_ticks: 0,
                    attack_interval_ticks: 18,
                    supply: 1,
                });
            }
            SimJobKind::TrainMedic => {
                let position = self.unoccupied_spawn_tile(job.target);
                self.support_units.push(SupportUnit {
                    unit_id: format!("field_medic_{}", self.support_units.len() + 1),
                    archetype_id: "field_medic".to_string(),
                    role: "medic".to_string(),
                    position,
                    hp: 210,
                    damage: 6,
                    armor: 0,
                    attack_range: 3,
                    ability_cooldown_ticks: 0,
                    attack_interval_ticks: 20,
                    supply: 1,
                });
            }
            SimJobKind::TrainRosterUnit => {
                let unit = UNIT_ROSTER
                    .iter()
                    .find(|unit| unit.id == job.rule_id)
                    .expect("queued faction unit remains catalogued");
                let position = self.unoccupied_spawn_tile(job.target);
                let max_hp = i64::from(unit.hp) * 3;
                self.party.push(SimUnit {
                    unit_id: format!("{}_{}", unit.id, self.party.len() + 1),
                    role: unit.role.to_string(),
                    persistent: false,
                    skill_ids: vec![unit.ability().rule_id().to_string()],
                    position,
                    hp: max_hp,
                    max_hp,
                    damage: i64::from(unit.damage) + i64::from(self.upgrade_level) * 3,
                    armor: i64::from(unit.supply) * 2,
                    move_speed_milli: match unit.ability() {
                        UnitAbility::SmokeDash | UnitAbility::PiercingCharge => 1_050,
                        UnitAbility::CommandSurge | UnitAbility::SuppressionBlast => 700,
                        _ => 850,
                    },
                    movement_budget_milli: 0,
                    attack_interval_ticks: 16 + u32::from(unit.supply) * 3,
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
            SimJobKind::ResearchLogistics => {
                self.researched_techs.insert("field_logistics".to_string());
            }
            SimJobKind::ResearchOptics => {
                self.researched_techs.insert("signal_optics".to_string());
                self.intel_level = self.intel_level.saturating_add(1).min(3);
            }
            SimJobKind::UpgradeRelayArms => {
                self.researched_techs.insert("relay_arms".to_string());
                self.upgrade_level = self.upgrade_level.saturating_add(1).min(3);
                for unit in &mut self.party {
                    unit.damage += 3;
                    unit.armor += 1;
                }
                for support in &mut self.support_units {
                    support.damage += 5;
                }
            }
            SimJobKind::UpgradeFieldArmor => {
                self.researched_techs.insert("field_armor".to_string());
                self.armor_upgrade_level = self.armor_upgrade_level.saturating_add(1).min(3);
                for unit in &mut self.party {
                    unit.armor += 2;
                    unit.max_hp += 25;
                    unit.hp += 25;
                }
            }
            SimJobKind::ResearchSensorNet => {
                self.researched_techs.insert("sensor_net".to_string());
                self.intel_level = 3;
            }
            SimJobKind::ResearchFieldMedicine => {
                self.researched_techs.insert("field_medicine".to_string());
                for unit in &mut self.party {
                    unit.max_energy += 20;
                    unit.energy += 20;
                }
            }
            SimJobKind::UpgradeSiegeDrills => {
                self.researched_techs.insert("siege_drills".to_string());
                for unit in &mut self.party {
                    unit.damage += 4;
                }
                self.relay_guard_hp = self.relay_guard_hp.saturating_sub(150);
            }
            SimJobKind::UpgradeReactivePlating => {
                self.researched_techs.insert("reactive_plating".to_string());
                for unit in &mut self.party {
                    unit.armor += 3;
                }
            }
            SimJobKind::ResearchWayfinderDrills => {
                self.researched_techs.insert("wayfinder_drills".to_string());
                for unit in &mut self.party {
                    unit.move_speed_milli += 120;
                    unit.evasion_permille = unit.evasion_permille.saturating_add(25).min(400);
                }
            }
            SimJobKind::ResearchRapidMustering => {
                self.researched_techs.insert("rapid_mustering".to_string());
            }
        }
        self.event_count += 1;
    }

    fn unoccupied_spawn_tile(&self, preferred: BattleGridPoint) -> BattleGridPoint {
        let occupied = self
            .party
            .iter()
            .chain(&self.enemies)
            .filter(|unit| unit.alive())
            .map(|unit| unit.position)
            .chain(self.support_units.iter().map(|unit| unit.position))
            .collect::<BTreeSet<_>>();
        let mut frontier = VecDeque::from([preferred]);
        let mut visited = BTreeSet::from([preferred]);
        while let Some(tile) = frontier.pop_front() {
            if self.seed.map.passable(tile) && !occupied.contains(&tile) {
                return tile;
            }
            for next in neighbors(tile) {
                if self.seed.map.in_bounds(next) && visited.insert(next) {
                    frontier.push_back(next);
                }
            }
        }
        self.seed.map.party_start
    }

    fn resolve_support_fire(&mut self) {
        for support_index in 0..self.support_units.len() {
            if !self
                .tick
                .is_multiple_of(self.support_units[support_index].attack_interval_ticks as u64)
            {
                continue;
            }
            let ability = UNIT_ROSTER
                .iter()
                .find(|unit| unit.id == self.support_units[support_index].archetype_id)
                .map(UnitArchetype::ability);
            if self.support_units[support_index].ability_cooldown_ticks == 0 {
                match ability {
                    Some(UnitAbility::RevealPulse) => {
                        self.recon_bonus_ticks = self.recon_bonus_ticks.max(120);
                        self.intel_level = self.intel_level.max(2);
                    }
                    Some(UnitAbility::GuardWall) => {
                        for unit in &mut self.party {
                            if distance(unit.position, self.support_units[support_index].position)
                                <= 3
                            {
                                unit.guard_ticks = unit.guard_ticks.max(50);
                            }
                        }
                    }
                    Some(UnitAbility::FieldRepair) => {
                        if let Some(structure) = self
                            .structures
                            .iter_mut()
                            .filter(|structure| {
                                structure.alive() && structure.hp < structure.max_hp
                            })
                            .min_by_key(|structure| structure.hp)
                        {
                            structure.hp = (structure.hp + 45).min(structure.max_hp);
                        }
                    }
                    Some(UnitAbility::TriageAura) => {
                        for unit in &mut self.party {
                            if unit.alive() {
                                unit.hp = (unit.hp + 18).min(unit.max_hp);
                            }
                        }
                    }
                    Some(UnitAbility::SmokeDash) => {
                        self.support_units[support_index].armor += 2;
                    }
                    Some(UnitAbility::RetaliationPlate) => {
                        self.support_units[support_index].hp += 20;
                    }
                    Some(UnitAbility::SignalJam) => {
                        self.enemy_ai_budget = self.enemy_ai_budget.saturating_sub(3);
                    }
                    Some(UnitAbility::CommandSurge) => {
                        for support in &mut self.support_units {
                            support.damage += 1;
                        }
                    }
                    Some(
                        UnitAbility::ArcVolley
                        | UnitAbility::SuppressionBlast
                        | UnitAbility::PiercingCharge
                        | UnitAbility::DemolitionCharge,
                    ) => {}
                    None => {}
                }
                if ability.is_some() {
                    self.support_units[support_index].ability_cooldown_ticks = 120;
                    self.event_count += 1;
                }
            }
            if self.support_units[support_index].role == "medic" {
                if let Some(target) = self
                    .party
                    .iter_mut()
                    .filter(|unit| unit.alive() && unit.hp < unit.max_hp)
                    .min_by_key(|unit| unit.hp * 100 / unit.max_hp.max(1))
                {
                    target.hp = (target.hp + 24).min(target.max_hp);
                    self.event_count += 1;
                }
                continue;
            }
            let target = self
                .enemies
                .iter()
                .enumerate()
                .filter(|(_, enemy)| enemy.alive())
                .filter(|(_, enemy)| {
                    distance(self.support_units[support_index].position, enemy.position)
                        <= self.support_units[support_index].attack_range
                })
                .min_by_key(|(_, enemy)| {
                    distance(self.support_units[support_index].position, enemy.position)
                })
                .map(|(index, _)| index);
            if let Some(target) = target {
                let ability_bonus = match ability {
                    Some(UnitAbility::ArcVolley) => 8,
                    Some(UnitAbility::SuppressionBlast) => 10,
                    Some(UnitAbility::PiercingCharge) => self.enemies[target].armor,
                    Some(UnitAbility::DemolitionCharge) => 14,
                    _ => 0,
                };
                self.enemies[target].hp -= self.support_units[support_index].damage + ability_bonus;
                self.event_count += 1;
            } else if self.phase == BattlePhase::Relay
                && self.relay_guard_hp > 0
                && distance(
                    self.support_units[support_index].position,
                    self.seed.map.objective,
                ) <= 5
            {
                self.relay_guard_hp -= self.support_units[support_index].damage;
                self.event_count += 1;
            }
        }
    }

    fn side_has_structure(&self, side: AuthoritySide, kind: SimStructureKind) -> bool {
        let structures = match side {
            AuthoritySide::Player => &self.structures,
            AuthoritySide::Enemy => &self.enemy_structures,
        };
        structures
            .iter()
            .any(|structure| structure.alive() && structure.kind == kind)
    }

}

