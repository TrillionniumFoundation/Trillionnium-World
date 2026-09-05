impl CampaignSaveV1 {
    pub fn cycle_active_title(&mut self) -> Result<BuildTitle, CampaignError> {
        self.apply_command_atomically(|candidate| candidate.cycle_active_title_atomic_inner())
    }

    fn cycle_active_title_atomic_inner(&mut self) -> Result<BuildTitle, CampaignError> {
        self.require_town()?;
        if self.unlocked_titles.is_empty() {
            return Err(CampaignError::InvalidState(
                "allocate a growth point before choosing a title".to_string(),
            ));
        }
        let titles = self.unlocked_titles.iter().copied().collect::<Vec<_>>();
        let next = self
            .active_title
            .and_then(|current| titles.iter().position(|title| *title == current))
            .map(|index| titles[(index + 1) % titles.len()])
            .unwrap_or(titles[0]);
        self.active_title = Some(next);
        self.character.title = next.display_name().to_string();
        self.revision += 1;
        Ok(next)
    }

    pub fn begin_signal_road_encounter(&mut self) -> Result<(), CampaignError> {
        self.apply_command_atomically(|candidate| {
            candidate.begin_signal_road_encounter_atomic_inner()
        })
    }

    fn begin_signal_road_encounter_atomic_inner(&mut self) -> Result<(), CampaignError> {
        self.require_town()?;
        if self.room != CampaignRoom::RelayQuarter
            && !(self.room == CampaignRoom::ExpeditionGate
                && self.active_title == Some(BuildTitle::GateWarden))
        {
            return Err(CampaignError::InvalidState(
                "the ambush is reachable from Relay Quarter or the Gate Warden route".to_string(),
            ));
        }
        if self.active_encounter.is_some() {
            return Err(CampaignError::InvalidState(
                "an RPG encounter is already active".to_string(),
            ));
        }
        self.active_encounter =
            RpgEncounterState::from_definition("signal_road_ambush", &self.character.attributes);
        let primary = self.active_technique_style();
        let secondary = self.secondary_technique_style();
        let primary_rank = self.technique_rank(primary);
        let secondary_rank = self.technique_rank(secondary);
        if let Some(encounter) = &mut self.active_encounter {
            encounter.set_technique_loadout(primary, primary_rank, secondary, secondary_rank);
        }
        self.last_encounter_outcome = None;
        self.revision += 1;
        Ok(())
    }

    pub fn begin_regional_encounter(&mut self, encounter_id: &str) -> Result<(), CampaignError> {
        self.apply_command_atomically(move |candidate| {
            candidate.begin_regional_encounter_atomic_inner(encounter_id)
        })
    }

    fn begin_regional_encounter_atomic_inner(
        &mut self,
        encounter_id: &str,
    ) -> Result<(), CampaignError> {
        self.require_town()?;
        if self.active_encounter.is_some() {
            return Err(CampaignError::InvalidState(
                "an RPG encounter is already active".to_string(),
            ));
        }
        self.active_encounter =
            RpgEncounterState::from_definition(encounter_id, &self.character.attributes);
        if self.active_encounter.is_none() {
            return Err(CampaignError::InvalidState(format!(
                "unknown regional encounter: {encounter_id}"
            )));
        }
        let primary = self.active_technique_style();
        let secondary = self.secondary_technique_style();
        let primary_rank = self.technique_rank(primary);
        let secondary_rank = self.technique_rank(secondary);
        if let Some(encounter) = &mut self.active_encounter {
            encounter.set_technique_loadout(primary, primary_rank, secondary, secondary_rank);
        }
        self.last_encounter_outcome = None;
        self.revision += 1;
        Ok(())
    }

    fn technique_style_for_slot(&self, slot: u8) -> TechniqueStyle {
        let slot = slot % 3;
        match (current_sect(&self.character), slot) {
            (Some(SectId::StreetCompass), 0) => TechniqueStyle::CompassFeint,
            (Some(SectId::StreetCompass), 1) => TechniqueStyle::CompassSpiral,
            (Some(SectId::StreetCompass), _) => TechniqueStyle::WayfinderSlip,
            (Some(SectId::IronWorkshop), 0) => TechniqueStyle::ForgeCounter,
            (Some(SectId::IronWorkshop), 1) => TechniqueStyle::RelayHammer,
            (Some(SectId::IronWorkshop), _) => TechniqueStyle::IronReversal,
            (Some(SectId::NightWatch), 0) => TechniqueStyle::NightVeil,
            (Some(SectId::NightWatch), 1) => TechniqueStyle::ShadowNeedle,
            (Some(SectId::NightWatch), _) => TechniqueStyle::LanternCut,
            (None, _) => TechniqueStyle::CenterlineBreak,
        }
    }

    fn active_technique_style(&self) -> TechniqueStyle {
        self.technique_style_for_slot(self.equipped_technique_slot)
    }

    pub fn secondary_technique_style(&self) -> TechniqueStyle {
        self.technique_style_for_slot(self.secondary_technique_slot)
    }

    fn technique_rank(&self, style: TechniqueStyle) -> u8 {
        self.technique_mastery
            .get(style.rule_id())
            .copied()
            .unwrap_or_default()
            .saturating_div(10)
            .min(10) as u8
    }

    pub fn cycle_equipped_technique(&mut self) -> Result<TechniqueStyle, CampaignError> {
        self.apply_command_atomically(|candidate| candidate.cycle_equipped_technique_atomic_inner())
    }

    fn cycle_equipped_technique_atomic_inner(&mut self) -> Result<TechniqueStyle, CampaignError> {
        self.require_town()?;
        if current_sect(&self.character).is_none() {
            return Err(CampaignError::InvalidState(
                "join a regional sect before configuring techniques".to_string(),
            ));
        }
        self.equipped_technique_slot = (self.equipped_technique_slot + 1) % 3;
        self.revision += 1;
        Ok(self.active_technique_style())
    }

    pub fn cycle_secondary_equipped_technique(&mut self) -> Result<TechniqueStyle, CampaignError> {
        self.apply_command_atomically(|candidate| {
            candidate.cycle_secondary_equipped_technique_atomic_inner()
        })
    }

    fn cycle_secondary_equipped_technique_atomic_inner(
        &mut self,
    ) -> Result<TechniqueStyle, CampaignError> {
        self.require_town()?;
        if current_sect(&self.character).is_none() {
            return Err(CampaignError::InvalidState(
                "join a regional sect before configuring techniques".to_string(),
            ));
        }
        self.secondary_technique_slot = (self.secondary_technique_slot + 1) % 3;
        if self.secondary_technique_slot == self.equipped_technique_slot {
            self.secondary_technique_slot = (self.secondary_technique_slot + 1) % 3;
        }
        self.revision += 1;
        Ok(self.secondary_technique_style())
    }

    pub fn act_in_signal_road_encounter(
        &mut self,
        action: EncounterAction,
    ) -> Result<Option<EncounterOutcome>, CampaignError> {
        self.apply_command_atomically(move |candidate| {
            candidate.act_in_signal_road_encounter_atomic_inner(action)
        })
    }

    fn act_in_signal_road_encounter_atomic_inner(
        &mut self,
        action: EncounterAction,
    ) -> Result<Option<EncounterOutcome>, CampaignError> {
        self.require_town()?;
        let item_available = self
            .progression
            .inventory
            .iter()
            .any(|stack| stack.item_id == "field-tonic-kit" && stack.quantity > 0);
        let encounter = self
            .active_encounter
            .as_mut()
            .ok_or_else(|| CampaignError::InvalidState("no RPG encounter is active".to_string()))?;
        let technique_style = encounter.next_technique_style();
        let turn = encounter
            .advance(&self.character.attributes, action, item_available)
            .map_err(CampaignError::InvalidState)?;
        let encounter_id = encounter.encounter_id.clone();
        let encounter_round = encounter.round;
        if matches!(
            action,
            EncounterAction::Technique
                | EncounterAction::PrimaryTechnique
                | EncounterAction::SecondaryTechnique
        ) {
            let mastery = self
                .technique_mastery
                .entry(technique_style.rule_id().to_string())
                .or_default();
            *mastery = mastery.saturating_add(1).min(100);
        }
        if turn.item_consumed {
            consume_loot(&mut self.progression.inventory, "field-tonic-kit", 1)?;
        }
        if let Some(outcome) = turn.outcome {
            self.last_encounter_outcome = Some(outcome);
            match outcome {
                EncounterOutcome::Victory => {
                    self.progression.experience = self.progression.experience.saturating_add(80);
                    self.character.attributes.reputation =
                        self.character.attributes.reputation.saturating_add(2);
                    let loot = ENCOUNTER_CATALOG
                        .iter()
                        .find(|definition| definition.id == encounter_id)
                        .map(|definition| {
                            definition
                                .loot_table
                                .iter()
                                .map(|item_id| LootStack {
                                    item_id: (*item_id).to_string(),
                                    quantity: 1,
                                })
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_else(|| {
                            vec![LootStack {
                                item_id: "signal-road-emblem".to_string(),
                                quantity: 1,
                            }]
                        });
                    merge_loot(&mut self.progression.inventory, &loot);
                    self.progression
                        .world_flags
                        .insert(format!("{encounter_id}_cleared"));
                }
                EncounterOutcome::Defeat => {
                    if let Some(hero) = self
                        .party
                        .iter_mut()
                        .find(|member| member.unit_id == "hero")
                    {
                        hero.injury_level = hero.injury_level.saturating_add(1).min(4);
                    }
                    self.progression
                        .world_flags
                        .insert(format!("{encounter_id}_defeat"));
                }
                EncounterOutcome::Withdrawn => {
                    self.progression
                        .world_flags
                        .insert(format!("{encounter_id}_withdrawn"));
                }
            }
            self.combat_log = original_combat_log(
                &encounter_id,
                encounter_round,
                outcome == EncounterOutcome::Victory,
            );
            self.active_encounter = None;
        }
        self.revision += 1;
        Ok(turn.outcome)
    }

    pub fn join_regional_sect(&mut self, sect: SectId) -> Result<(), CampaignError> {
        self.apply_command_atomically(move |candidate| {
            candidate.join_regional_sect_atomic_inner(sect)
        })
    }

    fn join_regional_sect_atomic_inner(&mut self, sect: SectId) -> Result<(), CampaignError> {
        self.require_town()?;
        let definition = SECT_CATALOG
            .iter()
            .find(|definition| definition.id == sect)
            .expect("three authored sects are static");
        if self.room.id() != definition.hall_room_id {
            return Err(CampaignError::InvalidState(format!(
                "{} requires visiting {}",
                definition.display_name, definition.hall_room_id
            )));
        }
        if self
            .character
            .sect_id
            .as_deref()
            .is_some_and(|current| current != sect.id())
        {
            return Err(CampaignError::InvalidState(
                "a character may commit to only one regional sect".to_string(),
            ));
        }
        self.character.sect_id = Some(sect.id().to_string());
        if !self
            .character
            .skill_ids
            .iter()
            .any(|skill| skill == definition.entry_skill_id)
        {
            self.character
                .skill_ids
                .push(definition.entry_skill_id.to_string());
        }
        self.progression
            .skill_progress
            .entry(definition.entry_skill_id.to_string())
            .or_insert(SkillProgress {
                rank: 1,
                experience: 0,
            });
        self.npc_relationships
            .entry(definition.mentor_id.to_string())
            .or_insert_with(|| NpcRelationship::new(definition.mentor_id, sect.id()))
            .apply(RelationshipAction::Train);
        self.revision += 1;
        Ok(())
    }

    pub fn train_next_sect_skill(&mut self) -> Result<String, CampaignError> {
        self.apply_command_atomically(|candidate| candidate.train_next_sect_skill_atomic_inner())
    }

    fn train_next_sect_skill_atomic_inner(&mut self) -> Result<String, CampaignError> {
        self.require_town()?;
        let sect = current_sect(&self.character).ok_or_else(|| {
            CampaignError::InvalidState(
                "join one regional sect before advanced training".to_string(),
            )
        })?;
        let known = self
            .character
            .skill_ids
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        let skill = SKILL_CATALOG
            .iter()
            .find(|skill| {
                skill.sect == Some(sect)
                    && !known.contains(skill.id)
                    && skill_unlockable(skill.id, &known, Some(sect))
            })
            .ok_or_else(|| {
                CampaignError::InvalidState("no next sect skill is unlockable".to_string())
            })?;
        if self.progression.credits < 35 {
            return Err(CampaignError::InvalidState(
                "advanced sect training costs 35 credits".to_string(),
            ));
        }
        self.progression.credits -= 35;
        self.character.skill_ids.push(skill.id.to_string());
        self.progression.skill_progress.insert(
            skill.id.to_string(),
            SkillProgress {
                rank: 1,
                experience: 0,
            },
        );
        self.revision += 1;
        Ok(skill.id.to_string())
    }

    pub fn wait_in_town(&mut self, minutes: u32) -> Result<(), CampaignError> {
        self.apply_command_atomically(move |candidate| candidate.wait_in_town_atomic_inner(minutes))
    }

    fn wait_in_town_atomic_inner(&mut self, minutes: u32) -> Result<(), CampaignError> {
        self.require_town()?;
        if !(30..=360).contains(&minutes) {
            return Err(CampaignError::InvalidState(
                "town waiting must be between 30 minutes and 6 hours".to_string(),
            ));
        }
        self.world_clock.advance(minutes);
        self.expedition_supplies.stamina = self
            .expedition_supplies
            .stamina
            .saturating_add((minutes / 30) as u8)
            .min(100);
        self.apply_current_social_event();
        self.run_regional_logistics();
        self.revision += 1;
        Ok(())
    }

    fn apply_current_social_event(&mut self) {
        let event = npc_social_event(self.world_clock.day, self.world_clock.minute_of_day);
        if self
            .social_event_history
            .last()
            .is_some_and(|record| record.event_id == event.id && record.day == self.world_clock.day)
        {
            return;
        }
        for npc_id in [event.first_npc_id, event.second_npc_id] {
            if let Some(relationship) = self.npc_relationships.get_mut(npc_id) {
                relationship.apply(RelationshipAction::Talk);
            }
            let memory = self.npc_memory.entry(npc_id.to_string()).or_default();
            memory.push(event.text.to_string());
            if memory.len() > 8 {
                memory.remove(0);
            }
            let work_output = self.npc_work_output.entry(npc_id.to_string()).or_default();
            *work_output = work_output.saturating_add(1);
        }
        let mut pair = [event.first_npc_id, event.second_npc_id];
        pair.sort_unstable();
        let bond_key = format!("{}::{}", pair[0], pair[1]);
        let bond = self.npc_bonds.entry(bond_key).or_default();
        *bond = bond.saturating_add(2).clamp(-100, 100);
        let bond_strength = *bond;
        let production = [event.first_npc_id, event.second_npc_id]
            .into_iter()
            .map(|npc_id| {
                self.npc_work_output
                    .get(npc_id)
                    .copied()
                    .unwrap_or_default()
            })
            .sum::<u32>()
            / 4
            + u32::from(bond_strength >= 8);
        let region_id = Self::region_id_for_room_id(event.room_id);
        let (current_stock, demand) = self.regional_market_state(region_id, event.market_item_id);
        let next_stock = if event.stock_delta >= 0 {
            current_stock
                .saturating_add(event.stock_delta as u16)
                .saturating_add(production.min(6) as u16)
                .min(99)
        } else {
            current_stock.saturating_sub(event.stock_delta.unsigned_abs())
        };
        self.set_regional_market_state(
            region_id,
            event.market_item_id,
            next_stock,
            (demand + event.demand_delta - i16::from(bond_strength >= 8)).clamp(-20, 20),
        );
        for (npc_id, counterpart) in [
            (event.first_npc_id, event.second_npc_id),
            (event.second_npc_id, event.first_npc_id),
        ] {
            let work = self
                .npc_work_output
                .get(npc_id)
                .copied()
                .unwrap_or_default();
            let (kind, target_id) = if bond_strength >= 12 {
                (NpcAutonomousGoalKind::FormAlliance, counterpart.to_string())
            } else if bond_strength < 0 {
                (
                    NpcAutonomousGoalKind::ResolveConflict,
                    counterpart.to_string(),
                )
            } else if work > 0 && work.is_multiple_of(7) {
                (NpcAutonomousGoalKind::Migrate, region_id.to_string())
            } else if work > 0 && work.is_multiple_of(5) {
                (
                    NpcAutonomousGoalKind::PublishTask,
                    event.market_item_id.to_string(),
                )
            } else {
                (
                    NpcAutonomousGoalKind::Produce,
                    event.market_item_id.to_string(),
                )
            };
            let goal = self
                .npc_autonomous_goals
                .entry(npc_id.to_string())
                .or_insert(NpcAutonomousGoal {
                    kind,
                    target_id: target_id.clone(),
                    region_id: region_id.to_string(),
                    progress: 0,
                });
            goal.kind = kind;
            goal.target_id = target_id;
            goal.region_id = region_id.to_string();
            goal.progress = goal.progress.saturating_add(1);
            match goal.kind {
                NpcAutonomousGoalKind::Migrate => {
                    let destination = match region_id {
                        "mirror_city" => RELAY_QUARTER_ROOM,
                        "signal_road" => GLASS_BASIN_WAYHOUSE_ROOM,
                        "glass_basin" => MOON_BRIDGE_ROOM,
                        _ => MARKET_WIND_PAVILION_ROOM,
                    };
                    self.npc_goal_rooms
                        .insert(npc_id.to_string(), destination.to_string());
                    self.progression
                        .world_flags
                        .insert(format!("npc_{npc_id}_migrated_to_{destination}"));
                }
                NpcAutonomousGoalKind::FormAlliance => {
                    self.progression.world_flags.insert(format!(
                        "npc_alliance_{}",
                        [npc_id, counterpart]
                            .into_iter()
                            .collect::<Vec<_>>()
                            .join("_")
                    ));
                }
                NpcAutonomousGoalKind::ResolveConflict => {
                    self.progression
                        .world_flags
                        .insert(format!("npc_conflict_{}_{}", npc_id, counterpart));
                }
                NpcAutonomousGoalKind::PublishTask => {
                    self.progression.world_flags.insert(format!(
                        "npc_dynamic_task_{npc_id}_{}",
                        event.market_item_id
                    ));
                }
                NpcAutonomousGoalKind::Produce => {}
            }
        }
        self.social_event_history.push(NpcSocialEventRecord {
            event_id: event.id.to_string(),
            first_npc_id: event.first_npc_id.to_string(),
            second_npc_id: event.second_npc_id.to_string(),
            room_id: event.room_id.to_string(),
            text: event.text.to_string(),
            day: self.world_clock.day,
            minute_of_day: self.world_clock.minute_of_day,
        });
        if self.social_event_history.len() > 16 {
            self.social_event_history.remove(0);
        }
    }

    pub fn cycle_main_story_choice(&mut self) -> Result<MainStoryChoice, CampaignError> {
        self.apply_command_atomically(|candidate| candidate.cycle_main_story_choice_atomic_inner())
    }

    fn cycle_main_story_choice_atomic_inner(&mut self) -> Result<MainStoryChoice, CampaignError> {
        self.require_town()?;
        self.main_story_choice = self.main_story_choice.next();
        self.progression
            .world_flags
            .retain(|flag| !flag.starts_with("main_story_choice_"));
        self.progression
            .world_flags
            .insert(format!("main_story_choice_{:?}", self.main_story_choice).to_ascii_lowercase());
        self.revision += 1;
        Ok(self.main_story_choice)
    }

    pub fn current_regional_npc(&self) -> Option<&'static trnm_rpg_core::NpcDefinition> {
        NPC_CATALOG
            .iter()
            .find(|npc| {
                npc.room_id == self.room.id()
                    && self
                        .npc_goal_rooms
                        .get(npc.id)
                        .map(String::as_str)
                        .or_else(|| npc_room_at(npc.id, self.world_clock.minute_of_day))
                        == Some(self.room.id())
            })
            .or_else(|| {
                NPC_CATALOG.iter().find(|npc| {
                    self.npc_goal_rooms
                        .get(npc.id)
                        .map(String::as_str)
                        .or_else(|| npc_room_at(npc.id, self.world_clock.minute_of_day))
                        == Some(self.room.id())
                })
            })
    }

    pub fn current_regional_npc_summary(&self) -> Option<String> {
        let npc = self.current_regional_npc()?;
        let schedule = npc_schedule(npc.id)?;
        let relationship = self.npc_relationships.get(npc.id);
        Some(format!(
            "{} ({:?}) | {} | trust {} | interactions {} | {} now",
            npc.display_name,
            npc.role,
            schedule.activity,
            relationship.map(|value| value.trust).unwrap_or(0),
            relationship.map(|value| value.interactions).unwrap_or(0),
            "present at this scheduled location"
        ))
    }

    pub fn current_regional_npc_interactions(&self) -> u16 {
        self.current_regional_npc()
            .and_then(|npc| self.npc_relationships.get(npc.id))
            .map(|relationship| relationship.interactions)
            .unwrap_or(0)
    }

    pub fn has_current_regional_npc(&self) -> bool {
        self.current_regional_npc().is_some()
    }

    pub fn talk_to_regional_npc(&mut self) -> Result<NpcConversationRecord, CampaignError> {
        self.apply_command_atomically(|candidate| candidate.talk_to_regional_npc_atomic_inner())
    }

    fn talk_to_regional_npc_atomic_inner(
        &mut self,
    ) -> Result<NpcConversationRecord, CampaignError> {
        self.require_town()?;
        let npc = self.current_regional_npc().ok_or_else(|| {
            CampaignError::InvalidState("there is no regional NPC in this room".to_string())
        })?;
        let schedule = npc_schedule(npc.id).expect("catalog NPC schedules are complete");
        let completed_tasks = npc
            .task_ids
            .iter()
            .filter(|quest_id| {
                self.regional_quest_states.get(**quest_id) == Some(&QuestState::Completed)
            })
            .count();
        let relationship = self
            .npc_relationships
            .entry(npc.id.to_string())
            .or_insert_with(|| NpcRelationship::new(npc.id, npc.faction_id));
        let first_interaction = relationship.interactions == 0;
        relationship.apply(RelationshipAction::Talk);
        if npc.id == "relay-smith-brann" && first_interaction {
            relationship.apply(RelationshipAction::CompleteMission);
            self.faction_rank = self.faction_rank.max(FactionRank::Envoy);
        }
        let stage = RelationshipStage::from_trust(relationship.trust, completed_tasks);
        let baseline = npc_dialogue(npc.id, relationship.trust, completed_tasks)
            .expect("catalog NPC dialogue is complete");
        let response = npc_choice_dialogue(npc.id, stage, self.dialogue_choice);
        match self.dialogue_choice {
            DialogueChoice::AskForWork => {}
            DialogueChoice::OfferHelp => {
                relationship.apply(RelationshipAction::Train);
            }
            DialogueChoice::ShareNews if completed_tasks > 0 => {
                relationship.apply(RelationshipAction::CompleteMission);
            }
            DialogueChoice::ShareNews => {}
        }
        let remembered = self
            .npc_memory
            .get(npc.id)
            .and_then(|memory| memory.last())
            .map(|memory| format!(" I remember: {memory}"))
            .unwrap_or_default();
        let work_output = self
            .npc_work_output
            .get(npc.id)
            .copied()
            .unwrap_or_default();
        let strongest_bond = self
            .npc_bonds
            .iter()
            .filter(|(pair, _)| pair.split("::").any(|member| member == npc.id))
            .map(|(_, bond)| *bond)
            .max()
            .unwrap_or_default();
        let goal = self
            .npc_autonomous_goals
            .get(npc.id)
            .map(|goal| {
                format!(
                    "{:?} {} in {} ({})",
                    goal.kind, goal.target_id, goal.region_id, goal.progress
                )
            })
            .unwrap_or_else(|| "building trust before committing scarce stock".to_string());
        let line = format!(
            "[{stage:?} / {:?}] {baseline} {response}{remembered} Current goal: {goal} (work {work_output}, bond {strongest_bond:+}).",
            self.dialogue_choice,
        );
        let record = NpcConversationRecord {
            npc_id: npc.id.to_string(),
            line,
            activity: format!("{}; currently in {}", schedule.activity, self.room.id()),
            day: self.world_clock.day,
            minute_of_day: self.world_clock.minute_of_day,
        };
        self.last_npc_conversation = Some(record.clone());
        self.conversation_history.push(record.clone());
        if self.conversation_history.len() > 24 {
            self.conversation_history.remove(0);
        }
        self.revision += 1;
        Ok(record)
    }

    pub fn cycle_dialogue_choice(&mut self) -> Result<DialogueChoice, CampaignError> {
        self.apply_command_atomically(|candidate| candidate.cycle_dialogue_choice_atomic_inner())
    }

    fn cycle_dialogue_choice_atomic_inner(&mut self) -> Result<DialogueChoice, CampaignError> {
        self.require_town()?;
        self.dialogue_choice = self.dialogue_choice.next();
        self.revision += 1;
        Ok(self.dialogue_choice)
    }

    pub fn active_regional_quest_objective(&self) -> Option<String> {
        let quest_id = self.active_regional_quest_id.as_deref()?;
        let definition = REGIONAL_QUEST_CATALOG
            .iter()
            .find(|definition| definition.id == quest_id)?;
        let ready_rooms = self.active_regional_quest_ready_rooms();
        if !ready_rooms.is_empty() {
            let runtime = self.active_regional_quest_runtime.as_ref()?;
            return Some(format!(
                "Authored graph {}/{}: choose ready node [{}] | {:?} approach | deadline day {}",
                self.active_regional_quest_step,
                definition.waypoint_room_ids.len(),
                ready_rooms.join(" / "),
                runtime.approach,
                runtime.deadline_day,
            ));
        }
        definition
            .encounter_id
            .map(|encounter| format!("Win {encounter}, then report to the quest giver"))
            .or_else(|| Some("Return for settlement".to_string()))
    }

    pub fn active_regional_quest_ready_rooms(&self) -> Vec<String> {
        let Some(quest_id) = self.active_regional_quest_id.as_deref() else {
            return Vec::new();
        };
        let Some(definition) = REGIONAL_QUEST_CATALOG
            .iter()
            .find(|definition| definition.id == quest_id)
        else {
            return Vec::new();
        };
        let Some(runtime) = self.active_regional_quest_runtime.as_ref() else {
            return Vec::new();
        };
        let graph = quest_condition_graph(definition, runtime.approach);
        graph
            .nodes
            .iter()
            .filter(|node| node.kind == trnm_rpg_core::QuestConditionKind::VisitWaypoint)
            .filter(|node| !runtime.completed_condition_node_ids.contains(&node.id))
            .filter(|node| {
                node.exclusive_group.as_ref().is_none_or(|group| {
                    !graph.nodes.iter().any(|candidate| {
                        candidate.exclusive_group.as_ref() == Some(group)
                            && runtime.completed_condition_node_ids.contains(&candidate.id)
                    })
                })
            })
            .filter(|node| {
                quest_graph_node_ready(&graph, &node.id, &runtime.completed_condition_node_ids)
            })
            .map(|node| node.subject_id.clone())
            .collect()
    }

    pub fn start_regional_quest(&mut self, quest_id: &str) -> Result<(), CampaignError> {
        self.apply_command_atomically(move |candidate| {
            candidate.start_regional_quest_atomic_inner(quest_id)
        })
    }

}

