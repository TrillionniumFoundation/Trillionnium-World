impl CampaignSaveV1 {
    fn apply_pending_settlement_atomic_inner(
        &mut self,
    ) -> Result<SettlementReceiptV1, CampaignError> {
        if self.phase != CampaignPhase::PostBattlePending {
            return Err(CampaignError::InvalidState(
                "no staged battle result is ready for settlement".to_string(),
            ));
        }
        let (mission_id, seed, result) = {
            let pending = self.pending_battle.as_ref().ok_or_else(|| {
                CampaignError::InvalidState("pending settlement payload is missing".to_string())
            })?;
            let result = pending.result.clone().ok_or_else(|| {
                CampaignError::InvalidState("pending settlement result is missing".to_string())
            })?;
            (pending.seed.map_id.clone(), pending.seed.clone(), result)
        };
        if self.settled_battle_ids.contains(&result.battle_id) {
            let existing = self.receipt_for(&result.battle_id).ok_or_else(|| {
                CampaignError::Integrity("settled battle is missing its receipt".to_string())
            })?;
            return Ok(SettlementReceiptV1::duplicate_from(existing, self.revision));
        }
        result.validate_against(&seed)?;
        let revision_before = self.revision;
        let experience_delta = result
            .units
            .iter()
            .map(|unit| unit.experience_gained)
            .sum::<u64>();
        let previous_level = self.progression.level;
        self.progression.experience += experience_delta;
        self.progression.level = 1 + (self.progression.experience / 500) as u32;
        let levels_gained = self.progression.level.saturating_sub(previous_level) as u16;
        self.progression.growth_points_available = self
            .progression
            .growth_points_available
            .saturating_add(levels_gained);
        self.progression.growth_points_awarded = self
            .progression
            .growth_points_awarded
            .saturating_add(levels_gained);
        self.character.attributes.reputation = self
            .character
            .attributes
            .reputation
            .saturating_add(result.reputation_delta);
        let credit_delta = if result.outcome == BattleOutcome::Victory {
            result.resource_delta.max(0)
        } else {
            0
        };
        self.progression.credits = self.progression.credits.saturating_add(credit_delta);
        merge_loot(&mut self.progression.inventory, &result.loot);
        self.progression
            .world_flags
            .extend(result.world_flags.iter().cloned());
        self.world_clock.advance(
            result
                .elapsed_ticks
                .div_ceil(600)
                .max(1)
                .min(u64::from(u32::MAX)) as u32,
        );
        if result.outcome == BattleOutcome::Victory {
            self.expedition_supplies.stamina =
                self.expedition_supplies.stamina.saturating_add(25).min(100);
            self.expedition_supplies.rations =
                self.expedition_supplies.rations.saturating_add(1).min(12);
            self.expedition_supplies.water =
                self.expedition_supplies.water.saturating_add(2).min(16);
            self.npc_relationships
                .entry("street-compass-sifu".to_string())
                .or_insert_with(|| {
                    NpcRelationship::new("street-compass-sifu", "signal-road-school")
                })
                .apply(RelationshipAction::CompleteMission);
        }

        let mut injury_delta_by_unit = BTreeMap::new();
        for report in &result.units {
            let delta = match report.status {
                UnitBattleStatus::Healthy => 0,
                UnitBattleStatus::Wounded => 1,
                UnitBattleStatus::Incapacitated | UnitBattleStatus::Lost => 2,
            };
            if delta > 0 {
                injury_delta_by_unit.insert(report.unit_id.clone(), delta);
            }
            if let Some(member) = self
                .party
                .iter_mut()
                .find(|member| member.unit_id == report.unit_id)
            {
                member.experience = member.experience.saturating_add(report.experience_gained);
                member.veteran_rank = member.veteran_rank.max(report.veteran_rank);
                member.confirmed_kills = member
                    .confirmed_kills
                    .saturating_add(report.confirmed_kills);
                member.injury_level = member.injury_level.saturating_add(delta).min(4);
                if report.status == UnitBattleStatus::Lost && !member.persistent {
                    member.available = false;
                }
            }
        }
        if result.outcome == BattleOutcome::Defeat && injury_delta_by_unit.is_empty() {
            // Losing an objective without a recorded combat wound still has a
            // persistent expedition cost. This prevents a player from farming
            // consequence-free defeats by holding safely while the objective
            // collapses.
            if let Some(member) = self
                .party
                .iter_mut()
                .find(|member| member.persistent && member.available)
            {
                member.injury_level = member.injury_level.saturating_add(1).min(4);
                injury_delta_by_unit.insert(member.unit_id.clone(), 1);
            }
        }
        for skill_id in &self.character.skill_ids {
            let skill = self
                .progression
                .skill_progress
                .entry(skill_id.clone())
                .or_insert(SkillProgress {
                    rank: 1,
                    experience: 0,
                });
            skill.experience += experience_delta / self.character.skill_ids.len().max(1) as u64;
            skill.rank = (1 + skill.experience / 250) as u16;
        }
        if result.outcome != BattleOutcome::Withdrawal {
            let wear = if result.outcome == BattleOutcome::Victory {
                6
            } else {
                12
            };
            for instance_id in self.character.equipment_slots.values() {
                if let Some(condition) = self.item_conditions.get_mut(instance_id) {
                    condition.apply_wear(wear);
                }
            }
        }
        self.quest_state = match result.outcome {
            BattleOutcome::Victory => QuestState::Completed,
            BattleOutcome::Defeat => QuestState::Failed,
            BattleOutcome::Withdrawal => QuestState::Withdrawn,
        };
        if result.outcome == BattleOutcome::Victory
            && matches!(
                mission_id.as_str(),
                "aftershock_patrol" | "first_contact_aftershock"
            )
        {
            self.progression.aftershock_completions =
                self.progression.aftershock_completions.saturating_add(1);
        }
        if result.outcome == BattleOutcome::Victory && mission_id == "first_contact" {
            self.complete_story_step(
                StoryStepId::SecureFirstContact,
                StoryStepId::BreakAftershock,
            )?;
        } else if result.outcome == BattleOutcome::Victory
            && matches!(
                mission_id.as_str(),
                "aftershock_patrol" | "first_contact_aftershock"
            )
        {
            self.complete_story_step(StoryStepId::BreakAftershock, StoryStepId::EvacuateConvoy)?;
        } else if result.outcome == BattleOutcome::Victory && mission_id == "convoy_exodus" {
            self.complete_story_step(StoryStepId::EvacuateConvoy, StoryStepId::SignalRoadComplete)?;
        }
        self.phase = CampaignPhase::Town;
        self.room = CampaignRoom::MirrorSquare;
        self.revision += 1;
        let receipt = SettlementReceiptV1 {
            contract_version: SETTLEMENT_RECEIPT_CONTRACT.to_string(),
            battle_id: result.battle_id.clone(),
            seed_hash: result.seed_hash.clone(),
            result_hash: result.computed_hash()?,
            campaign_revision_before: revision_before,
            campaign_revision_after: self.revision,
            outcome: result.outcome,
            experience_delta,
            reputation_delta: result.reputation_delta,
            credit_delta,
            loot_delta: result.loot.clone(),
            injury_delta_by_unit,
            economic_intent_id: (credit_delta > 0).then(|| {
                self.scoped_economic_intent_id(&format!("battle-reward:{}", result.battle_id))
            }),
            economic_receipt_id: None,
            duplicate: false,
        };
        self.settled_battle_ids.insert(result.battle_id.clone());
        self.settlement_receipts.push(receipt.clone());
        self.pending_battle = None;
        self.queue_battle_reward_economy(&receipt)?;
        if self.economy_mode == EconomyMode::OfflineLocal {
            self.reconcile_economy(&OfflineLocalEconomyBackend, 8)?;
        }
        self.validate()?;
        Ok(self
            .receipt_for(&receipt.battle_id)
            .cloned()
            .unwrap_or(receipt))
    }

    pub fn receipt_for(&self, battle_id: &str) -> Option<&SettlementReceiptV1> {
        self.settlement_receipts
            .iter()
            .find(|receipt| receipt.battle_id == battle_id)
    }

    fn require_town(&self) -> Result<(), CampaignError> {
        if self.phase == CampaignPhase::Town {
            Ok(())
        } else {
            Err(CampaignError::InvalidState(
                "town action is unavailable during battle handoff".to_string(),
            ))
        }
    }

    fn complete_story_step(
        &mut self,
        step_id: StoryStepId,
        next_step: StoryStepId,
    ) -> Result<(), CampaignError> {
        let definition = signal_road_quest_definition();
        let step = definition
            .steps
            .into_iter()
            .find(|step| step.id == step_id)
            .ok_or_else(|| {
                CampaignError::InvalidState(format!("missing story step definition: {step_id:?}"))
            })?;
        let conditions_met = step.conditions.iter().all(|condition| match condition {
            UnlockCondition::MentorMet => self.mentor_met,
            UnlockCondition::WorldFlag { flag } => {
                self.progression.world_flags.contains(flag.as_str())
            }
            UnlockCondition::MissionVictories { mission, count } => match mission {
                CampaignMission::FirstContact => self
                    .progression
                    .world_flags
                    .contains("first_contact_secured"),
                CampaignMission::AftershockPatrol => {
                    self.progression.aftershock_completions >= *count
                }
                CampaignMission::ConvoyExodus => self
                    .progression
                    .world_flags
                    .contains("convoy_exodus_secured"),
                CampaignMission::MirrorSiege => self
                    .progression
                    .world_flags
                    .contains("mirror_siege_secured"),
                CampaignMission::IronDeltaSkirmish => {
                    self.progression.world_flags.contains("iron_delta_won")
                }
                CampaignMission::NightWatchCrossingSkirmish => self
                    .progression
                    .world_flags
                    .contains("night_watch_crossing_won"),
                CampaignMission::GlassBasinSkirmish => {
                    self.progression.world_flags.contains("glass_basin_won")
                }
                CampaignMission::EmberOrchardSkirmish => {
                    self.progression.world_flags.contains("ember_orchard_won")
                }
                CampaignMission::SaltMarshSkirmish => {
                    self.progression.world_flags.contains("salt_marsh_won")
                }
                CampaignMission::CinderCrownSkirmish => {
                    self.progression.world_flags.contains("cinder_crown_won")
                }
            },
        });
        if !conditions_met {
            return Err(CampaignError::InvalidState(format!(
                "story step conditions are not met: {step_id:?}"
            )));
        }
        for reward in step.rewards {
            match reward {
                QuestReward::WorldFlag { flag } => {
                    self.progression.world_flags.insert(flag);
                }
                QuestReward::UnlockRoom { room_id } => {
                    self.story.unlocked_room_ids.insert(room_id);
                }
            }
        }
        self.story.completed_steps.insert(step_id);
        self.story.current_step = next_step;
        Ok(())
    }

    fn require_room(&self, room: CampaignRoom) -> Result<(), CampaignError> {
        self.require_town()?;
        if self.room == room {
            Ok(())
        } else {
            Err(CampaignError::InvalidState(format!(
                "action requires {}, current room is {}",
                room.title(),
                self.room.title()
            )))
        }
    }
}

