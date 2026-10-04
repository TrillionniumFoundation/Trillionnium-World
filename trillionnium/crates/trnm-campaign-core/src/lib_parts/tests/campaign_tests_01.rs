    fn map() -> BattleMapSeedV1 {
        BattleMapSeedV1 {
            width: 16,
            height: 8,
            terrain_rows: vec!["gggggggggggggggg".to_string(); 8],
            party_start: BattleGridPoint::new(1, 6),
            approach_point: BattleGridPoint::new(6, 5),
            objective: BattleGridPoint::new(14, 1),
            resource_nodes: vec![BattleMapNodeV1 {
                id: "amber_mid".to_string(),
                position: BattleGridPoint::new(7, 6),
            }],
            enemy_spawns: vec![
                BattleMapNodeV1 {
                    id: "enemy_0".to_string(),
                    position: BattleGridPoint::new(9, 4),
                },
                BattleMapNodeV1 {
                    id: "enemy_1".to_string(),
                    position: BattleGridPoint::new(11, 3),
                },
                BattleMapNodeV1 {
                    id: "enemy_2".to_string(),
                    position: BattleGridPoint::new(13, 2),
                },
            ],
        }
    }

    fn ready_campaign() -> CampaignSaveV1 {
        let mut campaign = CampaignSaveV1::default();
        campaign.move_to(CampaignRoom::MentorHall).unwrap();
        campaign.talk_to_mentor().unwrap();
        campaign.train_with_mentor().unwrap();
        campaign.equip_starter_weapon().unwrap();
        campaign.move_to(CampaignRoom::ExpeditionGate).unwrap();
        campaign.accept_first_contact_quest().unwrap();
        campaign
    }

    fn terminal_result(seed: &BattleSeedV1, outcome: BattleOutcome) -> BattleResultV1 {
        BattleResultV1 {
            contract_version: BATTLE_RESULT_CONTRACT.to_string(),
            battle_id: seed.battle_id.clone(),
            seed_hash: seed.seed_hash.clone(),
            outcome,
            units: seed
                .party
                .iter()
                .map(|unit| UnitBattleReportV1 {
                    unit_id: unit.unit_id.clone(),
                    status: UnitBattleStatus::Healthy,
                    remaining_hp: unit.stats.max_hp,
                    experience_gained: 30,
                    veteran_rank: 0,
                    confirmed_kills: 0,
                })
                .collect(),
            loot: vec![LootStack {
                item_id: "relay-core-fragment".to_string(),
                quantity: 1,
            }],
            resource_delta: 80,
            reputation_delta: 4,
            world_flags: vec!["first_contact_secured".to_string()],
            elapsed_ticks: 6_000,
            final_snapshot_hash: "a".repeat(64),
        }
    }

    #[test]
    fn mentor_training_and_loadout_are_required_before_battle() {
        let mut campaign = CampaignSaveV1::default();
        assert!(campaign.move_to(CampaignRoom::ExpeditionGate).is_err());
        let mut campaign = ready_campaign();
        let seed = campaign.start_first_contact_battle(map()).unwrap();
        assert_eq!(seed.party.len(), 4);
        assert!(seed.party[0]
            .equipment_ids
            .iter()
            .any(|item| item == "route-guard-staff"));
        seed.validate().unwrap();
    }

    #[test]
    fn seed_hash_rejects_tampered_rpg_stats() {
        let mut campaign = ready_campaign();
        let mut seed = campaign.start_first_contact_battle(map()).unwrap();
        seed.party[0].stats.damage += 999;
        assert!(matches!(seed.validate(), Err(CampaignError::Integrity(_))));
    }

    #[test]
    fn attribute_skill_equipment_mapping_is_typed_and_monotonic() {
        let attributes = TrillionniumAttributes::default();
        let base = map_rpg_to_rts_stats(&attributes, 1, &[], 0);
        let equipped = map_rpg_to_rts_stats(&attributes, 3, &["route-guard-staff".to_string()], 0);
        assert!(equipped.damage > base.damage);
        assert!(equipped.armor > base.armor);
        assert!(equipped.skill_power_permille > base.skill_power_permille);
        let wounded = map_rpg_to_rts_stats(&attributes, 3, &[], 2);
        assert!(wounded.max_hp < base.max_hp);
    }

    #[test]
    fn result_is_staged_before_settlement_and_duplicate_is_zero_delta() {
        let mut campaign = ready_campaign();
        let seed = campaign.start_first_contact_battle(map()).unwrap();
        let mut result = terminal_result(&seed, BattleOutcome::Victory);
        result.units[0].veteran_rank = 1;
        result.units[0].confirmed_kills = 2;
        campaign.stage_battle_result(result.clone()).unwrap();
        assert_eq!(campaign.phase, CampaignPhase::PostBattlePending);
        assert_eq!(campaign.progression.experience, 0);
        let receipt = campaign.apply_pending_settlement().unwrap();
        assert!(!receipt.duplicate);
        assert_eq!(receipt.experience_delta, 120);
        assert_eq!(receipt.credit_delta, 80);
        let expected_economic_intent_id = format!(
            "{}:battle-reward:{}",
            campaign.campaign_id, receipt.battle_id
        );
        assert_eq!(
            receipt.economic_intent_id.as_deref(),
            Some(expected_economic_intent_id.as_str())
        );
        let economic_receipt_id = receipt
            .economic_receipt_id
            .as_deref()
            .expect("offline battle reward receipt is linked");
        assert!(campaign.verified_economic_receipts.iter().any(|economic| {
            economic.receipt_id == economic_receipt_id
                && economic.intent_id == receipt.economic_intent_id.as_deref().unwrap()
        }));
        assert_eq!(campaign.quest_state, QuestState::Completed);
        assert_eq!(campaign.party[0].veteran_rank, 1);
        assert_eq!(campaign.party[0].confirmed_kills, 2);
        assert_eq!(campaign.room, CampaignRoom::MirrorSquare);
        let before = campaign.clone();
        let duplicate = campaign.submit_battle_result(result).unwrap();
        assert!(duplicate.duplicate);
        assert_eq!(duplicate.experience_delta, 0);
        assert_eq!(campaign, before);
    }

    #[test]
    fn atomic_store_recovers_a_post_battle_crash_once() {
        let directory = tempdir().unwrap();
        let store = CampaignStore::new(directory.path().join("campaign.json"));
        let mut campaign = ready_campaign();
        let seed = campaign.start_first_contact_battle(map()).unwrap();
        store.save_atomic(&campaign).unwrap();
        store
            .stage_result_atomic(
                &mut campaign,
                terminal_result(&seed, BattleOutcome::Victory),
            )
            .unwrap();
        let mut restarted = store.load().unwrap();
        assert_eq!(restarted.phase, CampaignPhase::PostBattlePending);
        let receipt = store
            .recover_pending_settlement(&mut restarted)
            .unwrap()
            .unwrap();
        assert!(!receipt.duplicate);
        let recovered = store.load().unwrap();
        assert_eq!(recovered.phase, CampaignPhase::Town);
        assert_eq!(recovered.progression.experience, 120);
        assert!(store
            .recover_pending_settlement(&mut restarted)
            .unwrap()
            .is_none());
    }

    #[test]
    fn corrupt_temp_file_cannot_replace_last_atomic_save() {
        let directory = tempdir().unwrap();
        let store = CampaignStore::new(directory.path().join("campaign.json"));
        let campaign = CampaignSaveV1::default();
        store.save_atomic(&campaign).unwrap();
        fs::write(store.path().with_extension("json.tmp"), b"{broken").unwrap();
        assert_eq!(store.load().unwrap(), campaign);
    }

    #[test]
    fn training_is_paid_capped_and_paths_are_real_choices() {
        let mut campaign = CampaignSaveV1::default();
        campaign.move_to(CampaignRoom::MentorHall).unwrap();
        campaign.talk_to_mentor().unwrap();
        let initial_credits = campaign.progression.credits;
        campaign.train_with_mentor().unwrap();
        campaign.cycle_training_path().unwrap();
        campaign.train_with_mentor().unwrap();
        assert_eq!(campaign.progression.mentor_training_sessions, 2);
        assert!(campaign.progression.credits < initial_credits);
        assert!(campaign.train_with_mentor().is_err());
        assert!(campaign
            .character
            .skill_ids
            .iter()
            .any(|skill| skill == "iron_guard"));
        assert!(campaign
            .character
            .skill_ids
            .iter()
            .any(|skill| skill == "wind_step"));
    }

    #[test]
    fn party_loadout_and_healing_create_persistent_tradeoffs() {
        let mut campaign = CampaignSaveV1::default();
        assert_eq!(campaign.party.len(), 7);
        campaign.cycle_party_preset().unwrap();
        assert_eq!(campaign.active_party_ids, ["hero", "aya", "nia", "sol"]);
        campaign.cycle_loadout().unwrap();
        assert_eq!(campaign.selected_loadout, LoadoutPreset::Guard);
        campaign.cycle_loadout().unwrap();
        assert_eq!(campaign.selected_loadout, LoadoutPreset::Raider);
        campaign.party[0].injury_level = 2;
        let credits = campaign.progression.credits;
        campaign.heal_party().unwrap();
        assert_eq!(campaign.party[0].injury_level, 1);
        assert_eq!(
            campaign.progression.credits,
            credits - FIELD_CLINIC_CREDIT_COST
        );
        campaign.progression.inventory.push(LootStack {
            item_id: "relay-core-fragment".to_string(),
            quantity: 1,
        });
        campaign.equip_relay_core().unwrap();
        assert!(campaign.character.equipment_slots.contains_key("relic"));
        let modifier = typed_equipment_modifier("relay-core-fragment");
        assert!(modifier.energy > 0 && modifier.ability_range > 0);
        let coat = typed_equipment_modifier("compass-thread-coat");
        assert!(coat.armor > 0 && coat.move_speed_milli > 0 && coat.evasion_permille > 0);
        let lens = typed_equipment_modifier("emberglass-lens");
        assert!(lens.energy > 0 && lens.ability_range > 0);
        for item in ECONOMY_ITEM_CATALOG.iter().filter(|item| !item.material) {
            let modifier = typed_equipment_modifier(item.id);
            assert!(
                modifier.max_hp != 0
                    || modifier.damage != 0
                    || modifier.armor != 0
                    || modifier.move_speed_milli != 0
                    || modifier.attack_interval_ticks != 0
                    || modifier.evasion_permille != 0
                    || modifier.energy != 0
                    || modifier.ability_range != 0,
                "non-material catalog item {} has no explicit BattleSeed modifier",
                item.id
            );
        }
    }

    #[test]
    fn free_party_relationship_recruitment_and_sparring_are_persistent() {
        let mut campaign = CampaignSaveV1::default();
        assert!(
            !campaign
                .party
                .iter()
                .find(|member| member.unit_id == "brann")
                .unwrap()
                .available
        );
        campaign.move_to(CampaignRoom::MentorHall).unwrap();
        campaign.talk_to_mentor().unwrap();
        campaign.train_with_mentor().unwrap();
        let report = campaign.spar_with_mentor().unwrap();
        assert_eq!(report.outcome, SparringOutcome::Victory);
        assert_eq!(campaign.faction_rank, FactionRank::Disciple);
        assert_eq!(
            campaign.character.sect_id.as_deref(),
            Some("signal-road-school")
        );

        campaign
            .progression
            .world_flags
            .insert("signal_road_secured".to_string());
        campaign.move_to(CampaignRoom::MirrorSquare).unwrap();
        campaign.move_to(CampaignRoom::RelayQuarter).unwrap();
        campaign.talk_to_relay_smith().unwrap();
        campaign.recruit_relay_smith().unwrap();
        assert!(
            campaign
                .party
                .iter()
                .find(|member| member.unit_id == "brann")
                .unwrap()
                .available
        );
        campaign
            .select_party(vec![
                "hero".to_string(),
                "aya".to_string(),
                "mako".to_string(),
                "brann".to_string(),
            ])
            .unwrap();
        assert!(campaign.validate().is_ok());
    }

    #[test]
    fn growth_preview_confirm_cancel_and_reload_are_atomic() {
        let directory = tempdir().unwrap();
        let store = CampaignStore::new(directory.path().join("growth.json"));
        let mut campaign = CampaignSaveV1::default();
        let force_before = campaign.character.attributes.force;
        campaign
            .preview_growth_allocation(GrowthStat::Force)
            .unwrap();
        assert_eq!(campaign.character.attributes.force, force_before);
        assert_eq!(campaign.progression.growth_points_available, 1);
        campaign.cancel_growth_allocation().unwrap();
        assert_eq!(campaign.progression.growth_points_available, 1);

        campaign
            .preview_growth_allocation(GrowthStat::Force)
            .unwrap();
        campaign.confirm_growth_allocation().unwrap();
        assert_eq!(campaign.character.attributes.force, force_before + 1);
        assert_eq!(campaign.progression.growth_points_available, 0);
        assert_eq!(campaign.build_path, BuildPath::Vanguard);
        assert_eq!(campaign.active_title, None);
        assert!(campaign.unlocked_titles.is_empty());
        assert!(campaign.confirm_growth_allocation().is_err());
        store.save_atomic(&campaign).unwrap();
        assert_eq!(store.load().unwrap(), campaign);
    }

    #[test]
    fn force_and_agility_builds_emit_observably_different_battle_seeds() {
        let prepare = |stat| {
            let mut campaign = CampaignSaveV1::default();
            campaign.preview_growth_allocation(stat).unwrap();
            campaign.confirm_growth_allocation().unwrap();
            campaign.move_to(CampaignRoom::MentorHall).unwrap();
            campaign.talk_to_mentor().unwrap();
            campaign.train_with_mentor().unwrap();
            campaign.equip_starter_weapon().unwrap();
            campaign.move_to(CampaignRoom::ExpeditionGate).unwrap();
            campaign.accept_first_contact_quest().unwrap();
            campaign.start_first_contact_battle(map()).unwrap()
        };
        let force = prepare(GrowthStat::Force);
        let agility = prepare(GrowthStat::Agility);
        assert!(force.party[0].stats.damage > agility.party[0].stats.damage);
        assert!(agility.party[0].stats.move_speed_milli > force.party[0].stats.move_speed_milli);
        assert_ne!(force.seed_hash, agility.seed_hash);
    }

    #[test]
    fn three_origins_by_three_paths_emit_nine_observable_builds() {
        let origins = [
            CharacterOrigin::Balanced,
            CharacterOrigin::Artisan,
            CharacterOrigin::Scout,
        ];
        let paths = [GrowthStat::Force, GrowthStat::Agility, GrowthStat::Craft];
        let mut hashes = BTreeSet::new();
        let mut stat_signatures = BTreeSet::new();
        for origin in origins {
            for stat in paths {
                let mut campaign = CampaignSaveV1::default();
                while campaign.character_origin != origin {
                    campaign.cycle_character_origin().unwrap();
                }
                campaign.preview_growth_allocation(stat).unwrap();
                campaign.confirm_growth_allocation().unwrap();
                campaign.move_to(CampaignRoom::MentorHall).unwrap();
                campaign.talk_to_mentor().unwrap();
                campaign.train_with_mentor().unwrap();
                campaign.attempt_mastery_challenge().unwrap();
                campaign.equip_starter_weapon().unwrap();
                campaign.move_to(CampaignRoom::ExpeditionGate).unwrap();
                campaign.accept_first_contact_quest().unwrap();
                let seed = campaign.start_first_contact_battle(map()).unwrap();
                assert_eq!(seed.character_origin, origin);
                assert!(seed.active_title.is_some());
                hashes.insert(seed.seed_hash.clone());
                let hero = &seed.party[0].stats;
                stat_signatures.insert((
                    hero.max_hp,
                    hero.damage,
                    hero.armor,
                    hero.move_speed_milli,
                    hero.energy,
                    hero.ability_range,
                ));
            }
        }
        assert_eq!(hashes.len(), 9);
        assert_eq!(stat_signatures.len(), 9);
    }

    #[test]
    fn task_navigation_reports_next_exit_and_locked_failure() {
        let campaign = CampaignSaveV1::default();
        let route = campaign.current_task_route_plan();
        assert_eq!(
            route.next_exit.as_ref().map(|exit| exit.to.as_str()),
            Some(MENTOR_HALL_ROOM)
        );
        let mut campaign = campaign;
        campaign.story.current_step = StoryStepId::SignalRoadComplete;
        let blocked = campaign.current_task_route_plan();
        assert!(matches!(
            blocked.blocked_reason,
            Some(trnm_rpg_core::WorldRouteBlockedReason::LockedRoom { .. })
        ));
    }

    #[test]
    fn typed_rpg_encounter_applies_item_injury_loot_and_route_consequences() {
        let mut campaign = CampaignSaveV1::default();
        campaign
            .progression
            .world_flags
            .insert("signal_road_secured".to_string());
        campaign.move_to(CampaignRoom::RelayQuarter).unwrap();
        campaign.progression.inventory.push(LootStack {
            item_id: "field-tonic-kit".to_string(),
            quantity: 1,
        });
        campaign.begin_signal_road_encounter().unwrap();
        campaign
            .act_in_signal_road_encounter(EncounterAction::Defend)
            .unwrap();
        campaign
            .act_in_signal_road_encounter(EncounterAction::UseItem)
            .unwrap();
        while campaign.active_encounter.is_some() {
            campaign
                .act_in_signal_road_encounter(EncounterAction::Attack)
                .unwrap();
        }
        assert_eq!(
            campaign.last_encounter_outcome,
            Some(EncounterOutcome::Victory)
        );
        assert!(campaign
            .progression
            .inventory
            .iter()
            .any(|stack| stack.item_id == "signal-road-emblem"));
        assert!(campaign
            .progression
            .world_flags
            .contains("signal_road_ambush_cleared"));
        assert!(!campaign
            .progression
            .inventory
            .iter()
            .any(|stack| stack.item_id == "field-tonic-kit"));

        campaign.begin_signal_road_encounter().unwrap();
        campaign
            .act_in_signal_road_encounter(EncounterAction::Withdraw)
            .unwrap();
        assert_eq!(
            campaign.last_encounter_outcome,
            Some(EncounterOutcome::Withdrawn)
        );
        assert!(campaign
            .progression
            .world_flags
            .contains("signal_road_ambush_withdrawn"));

        campaign.begin_signal_road_encounter().unwrap();
        campaign.active_encounter.as_mut().unwrap().player_hp = 1;
        campaign
            .act_in_signal_road_encounter(EncounterAction::Attack)
            .unwrap();
        assert_eq!(
            campaign.last_encounter_outcome,
            Some(EncounterOutcome::Defeat)
        );
        assert_eq!(campaign.party[0].injury_level, 1);
    }

    #[test]
    fn build_titles_unlock_a_route_encounter_and_real_price() {
        let mut runner = CampaignSaveV1::default();
        runner
            .preview_growth_allocation(GrowthStat::Agility)
            .unwrap();
        runner.confirm_growth_allocation().unwrap();
        runner.move_to(CampaignRoom::MentorHall).unwrap();
        runner.talk_to_mentor().unwrap();
        runner.train_with_mentor().unwrap();
        runner.attempt_mastery_challenge().unwrap();
        runner.move_to(CampaignRoom::MirrorSquare).unwrap();
        runner.move_to(CampaignRoom::RelayQuarter).unwrap();
        assert_eq!(runner.active_title, Some(BuildTitle::RelayRunner));

        let mut smith = CampaignSaveV1::default();
        smith.preview_growth_allocation(GrowthStat::Craft).unwrap();
        smith.confirm_growth_allocation().unwrap();
        smith.move_to(CampaignRoom::MentorHall).unwrap();
        smith.talk_to_mentor().unwrap();
        smith.train_with_mentor().unwrap();
        smith.attempt_mastery_challenge().unwrap();
        smith.move_to(CampaignRoom::MirrorSquare).unwrap();
        smith.party[0].injury_level = 1;
        let credits = smith.progression.credits;
        smith.heal_party().unwrap();
        assert_eq!(smith.progression.credits, credits - 25);

        let mut warden = CampaignSaveV1::default();
        warden.preview_growth_allocation(GrowthStat::Force).unwrap();
        warden.confirm_growth_allocation().unwrap();
        warden.move_to(CampaignRoom::MentorHall).unwrap();
        warden.talk_to_mentor().unwrap();
        warden.train_with_mentor().unwrap();
        warden.attempt_mastery_challenge().unwrap();
        warden.move_to(CampaignRoom::MirrorSquare).unwrap();
        warden.move_to(CampaignRoom::ExpeditionGate).unwrap();
        warden.begin_signal_road_encounter().unwrap();
        assert!(warden.active_encounter.is_some());
    }

    #[test]
    fn three_save_slots_are_isolated_and_settings_are_profile_scoped() {
        let directory = tempdir().unwrap();
        let slots = SaveSlotStore::new(directory.path());
        let mut first = slots.create_new(SaveSlotId::A, false).unwrap();
        first.progression.credits = 777;
        slots.save_atomic(SaveSlotId::A, &first).unwrap();
        let second = slots.create_new(SaveSlotId::B, false).unwrap();
        assert_ne!(first.campaign_id, second.campaign_id);
        assert_eq!(slots.load(SaveSlotId::A).unwrap().progression.credits, 777);
        assert_ne!(slots.load(SaveSlotId::B).unwrap().progression.credits, 777);
        assert!(slots.create_new(SaveSlotId::A, false).is_err());

        fs::write(slots.path(SaveSlotId::C), b"not-json").unwrap();
        assert!(!slots.metadata(SaveSlotId::C).valid);
        assert!(slots.metadata(SaveSlotId::A).valid);

        let settings_store =
            PlayerSettingsStore::new(directory.path().join("player-settings.json"));
        let settings = PlayerSettings {
            low_motion: true,
            input_mode: InputMode::KeyboardOnly,
            ..PlayerSettings::default()
        };
        settings_store.save_atomic(&settings).unwrap();
        slots.create_new(SaveSlotId::A, true).unwrap();
        assert_eq!(settings_store.load_or_default().unwrap(), settings);
    }

    #[test]
    fn cistern_relief_is_a_typed_branching_persistent_quest_chain() {
        let mut campaign = CampaignSaveV1::default();
        campaign
            .progression
            .world_flags
            .insert("outer_signal_road_open".to_string());
        campaign
            .progression
            .world_flags
            .insert("signal_road_secured".to_string());
        campaign
            .progression
            .world_flags
            .insert("expedition_gate_open".to_string());
        campaign.move_to(CampaignRoom::RelayQuarter).unwrap();
        campaign.start_cistern_relief().unwrap();
        assert_eq!(
            campaign.quest_chain.as_ref().unwrap().current_node,
            QuestChainNodeId::SurveyDamage
        );
        campaign.advance_cistern_relief().unwrap();
        campaign.move_to(CampaignRoom::MirrorSquare).unwrap();
        campaign.move_to(CampaignRoom::ExpeditionGate).unwrap();
        let credits_before = campaign.progression.credits;
        campaign.advance_cistern_relief().unwrap();
        assert_eq!(campaign.progression.credits, credits_before - 40);
        campaign.move_to(CampaignRoom::MirrorSquare).unwrap();
        campaign.move_to(CampaignRoom::RelayQuarter).unwrap();
        campaign.advance_cistern_relief().unwrap_err();

        let mut reinforce = campaign.clone();
        let mut evacuate = campaign;
        reinforce
            .choose_cistern_relief_branch(QuestBranch::ReinforceCistern)
            .unwrap();
        evacuate
            .choose_cistern_relief_branch(QuestBranch::EvacuateFamilies)
            .unwrap();
        assert!(reinforce
            .progression
            .world_flags
            .contains("cistern_reinforced"));
        assert!(evacuate
            .progression
            .world_flags
            .contains("cistern_families_evacuated"));
        assert!(evacuate.progression.credits > reinforce.progression.credits);
        assert!(
            reinforce.character.attributes.reputation > evacuate.character.attributes.reputation
        );

        let directory = tempdir().unwrap();
        let store = CampaignStore::new(directory.path().join("quest.json"));
        store.save_atomic(&reinforce).unwrap();
        assert_eq!(store.load().unwrap().quest_chain, reinforce.quest_chain);
    }

    #[test]
    fn expedition_preparation_changes_seed_time_supplies_and_battle_stats() {
        let immediate_campaign = ready_campaign();
        let mut supplied_campaign = immediate_campaign.clone();
        supplied_campaign.cycle_expedition_preparation().unwrap();
        supplied_campaign.cycle_expedition_preparation().unwrap();
        let mut immediate_campaign = immediate_campaign;
        let immediate = immediate_campaign
            .start_first_contact_battle(map())
            .unwrap();
        let supplied = supplied_campaign.start_first_contact_battle(map()).unwrap();
        assert_eq!(
            immediate.expedition_readiness.preparation,
            ExpeditionPreparation::Immediate
        );
        assert_eq!(
            supplied.expedition_readiness.preparation,
            ExpeditionPreparation::Supplied
        );
        assert_eq!(supplied.expedition_readiness.starting_resources, 50);
        assert!(supplied.party[0].stats.energy > immediate.party[0].stats.energy);
        assert_ne!(supplied.seed_hash, immediate.seed_hash);
        assert_ne!(
            supplied_campaign.world_clock,
            immediate_campaign.world_clock
        );
        assert_ne!(
            supplied_campaign.expedition_supplies,
            immediate_campaign.expedition_supplies
        );
    }

    #[test]
    fn new_slot_requires_identity_confirmation_and_persists_one_canonical_name() {
        let directory = tempdir().unwrap();
        let slots = SaveSlotStore::new(directory.path());
        let mut campaign = slots.create_new(SaveSlotId::A, false).unwrap();
        assert!(!campaign.character_identity.confirmed);
        assert_eq!(campaign.character.display_name, "Mirror Ranger");
        assert_eq!(
            campaign.cycle_character_identity().unwrap(),
            CharacterNamePreset::SignalRook
        );
        campaign.confirm_character_identity().unwrap();
        slots.save_atomic(SaveSlotId::A, &campaign).unwrap();
        let loaded = slots.load(SaveSlotId::A).unwrap();
        assert!(loaded.character_identity.confirmed);
        assert_eq!(loaded.character.display_name, "Signal Rook");
        assert_eq!(loaded.party[0].display_name, "Signal Rook");
        assert!(loaded.validate().is_ok());
        assert!(campaign.cycle_character_identity().is_err());
    }

