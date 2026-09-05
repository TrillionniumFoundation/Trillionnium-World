    #[test]
    fn progressive_guide_and_journal_follow_authoritative_campaign_state() {
        let directory = tempdir().unwrap();
        let store = CampaignStore::new(directory.path().join("guided.json"));
        let mut campaign = CampaignSaveV1::default();
        assert_eq!(campaign.current_guide_step(), CampaignGuideStep::MeetMentor);
        campaign.move_to(CampaignRoom::MentorHall).unwrap();
        campaign.talk_to_mentor().unwrap();
        assert_eq!(
            campaign.current_guide_step(),
            CampaignGuideStep::TrainWithMentor
        );
        campaign.train_with_mentor().unwrap();
        assert_eq!(
            campaign.current_guide_step(),
            CampaignGuideStep::EquipWeapon
        );
        campaign.equip_starter_weapon().unwrap();
        assert_eq!(
            campaign.current_guide_step(),
            CampaignGuideStep::ReachExpeditionGate
        );
        campaign.move_to(CampaignRoom::ExpeditionGate).unwrap();
        assert_eq!(
            campaign.current_guide_step(),
            CampaignGuideStep::AcceptMission
        );
        campaign.accept_first_contact_quest().unwrap();
        assert_eq!(
            campaign.current_guide_step(),
            CampaignGuideStep::DeployMission
        );
        let journal = campaign.campaign_journal();
        assert_eq!(journal.len(), 3);
        assert_eq!(journal[0].state, CampaignJournalState::Active);
        store.save_atomic(&campaign).unwrap();
        assert_eq!(store.load().unwrap().campaign_journal(), journal);
    }

    #[test]
    fn difficulty_changes_seed_and_mirror_siege_follows_the_convoy() {
        let mut standard = ready_campaign();
        let standard_seed = standard.start_first_contact_battle(map()).unwrap();
        let mut veteran = CampaignSaveV1::default();
        assert_eq!(
            veteran.cycle_difficulty().unwrap(),
            CampaignDifficulty::Veteran
        );
        veteran.move_to(CampaignRoom::MentorHall).unwrap();
        veteran.talk_to_mentor().unwrap();
        veteran.train_with_mentor().unwrap();
        veteran.equip_starter_weapon().unwrap();
        veteran.move_to(CampaignRoom::ExpeditionGate).unwrap();
        veteran.accept_first_contact_quest().unwrap();
        let veteran_seed = veteran.start_first_contact_battle(map()).unwrap();
        assert_ne!(standard_seed.seed_hash, veteran_seed.seed_hash);
        assert_eq!(veteran_seed.difficulty, CampaignDifficulty::Veteran);

        let mut campaign = ready_campaign();
        campaign.quest_state = QuestState::Completed;
        campaign.progression.aftershock_completions = 1;
        campaign
            .progression
            .world_flags
            .extend(["first_contact_secured", "convoy_exodus_secured"].map(str::to_string));
        campaign.accept_first_contact_quest().unwrap();
        assert_eq!(campaign.active_mission, CampaignMission::MirrorSiege);
        assert_eq!(
            campaign.campaign_journal()[0].objective,
            "Break the siege and reclaim Mirror Gate"
        );
    }

    #[test]
    fn twenty_room_four_region_three_sect_world_and_regional_quest_route_are_live() {
        let graph = mirror_city_world_graph();
        assert_eq!(graph.rooms.len(), 20);
        assert_eq!(
            graph
                .rooms
                .values()
                .map(|room| room.region_id.as_str())
                .collect::<BTreeSet<_>>()
                .len(),
            4
        );
        let mut campaign = CampaignSaveV1::default();
        campaign.move_to(CampaignRoom::MarketWindPavilion).unwrap();
        campaign.talk_to_regional_npc().unwrap();
        assert_eq!(
            campaign.start_first_regional_quest_here().unwrap(),
            "market_debt"
        );
        campaign.advance_active_regional_quest().unwrap();
        assert_eq!(
            campaign.current_task_route_plan().destination_room_id,
            WORKSHOP_GATE_ROOM
        );
        campaign.move_to(CampaignRoom::MirrorSquare).unwrap();
        campaign.move_to(CampaignRoom::MentorHall).unwrap();
        campaign.move_to(CampaignRoom::WorkshopGate).unwrap();
        campaign.advance_active_regional_quest().unwrap();
        assert_eq!(
            campaign.regional_quest_states.get("market_debt"),
            Some(&QuestState::Completed)
        );
        campaign.join_regional_sect(SectId::IronWorkshop).unwrap();
        assert_eq!(
            campaign.character.sect_id.as_deref(),
            Some("iron_workshop_gate")
        );
        assert!(campaign.train_next_sect_skill().is_ok());
    }

    #[test]
    fn npc_hours_conversations_shop_browser_and_skirmish_setup_are_authoritative() {
        let mut campaign = CampaignSaveV1::default();
        campaign.move_to(CampaignRoom::MentorHall).unwrap();
        campaign.move_to(CampaignRoom::ArchiveSteps).unwrap();
        campaign.move_to(CampaignRoom::NightWatchPost).unwrap();
        assert!(campaign.talk_to_regional_npc().is_err());
        campaign.wait_in_town(4 * 60).unwrap();
        campaign.wait_in_town(4 * 60).unwrap();
        let conversation = campaign.talk_to_regional_npc().unwrap();
        assert_eq!(conversation.npc_id, "captain-veyra");
        assert!(conversation.activity.contains("night watch"));
        assert_eq!(campaign.conversation_history.len(), 1);

        campaign.move_to(CampaignRoom::ArchiveSteps).unwrap();
        campaign.move_to(CampaignRoom::MentorHall).unwrap();
        campaign.move_to(CampaignRoom::MirrorSquare).unwrap();
        campaign.move_to(CampaignRoom::MarketWindPavilion).unwrap();
        let first_item = campaign.selected_shop_item().id.to_string();
        let credits_before = campaign.progression.credits;
        campaign.buy_selected_shop_item().unwrap();
        assert!(campaign.progression.credits < credits_before);
        assert!(campaign
            .character
            .inventory_items
            .iter()
            .any(|item| item.item_id == first_item));
        assert_ne!(campaign.cycle_shop_item().unwrap(), first_item);
        campaign
            .progression
            .world_flags
            .insert("signal_road_secured".to_string());
        campaign.move_to(CampaignRoom::MirrorSquare).unwrap();
        campaign.move_to(CampaignRoom::RelayQuarter).unwrap();
        campaign.talk_to_regional_npc().unwrap();
        campaign.recruit_relay_smith().unwrap();
        assert!(campaign
            .party
            .iter()
            .any(|member| member.unit_id == "brann" && member.available));

        let mut skirmish = ready_campaign();
        skirmish
            .progression
            .world_flags
            .insert("mirror_siege_secured".to_string());
        skirmish.cycle_endgame_mission().unwrap();
        assert_eq!(
            skirmish.cycle_endgame_mission().unwrap(),
            CampaignMission::IronDeltaSkirmish
        );
        assert_eq!(
            skirmish.cycle_skirmish_faction().unwrap(),
            CampaignFaction::AshenCompact
        );
        assert_eq!(skirmish.cycle_skirmish_resources().unwrap(), 500);
        assert_eq!(
            skirmish.cycle_skirmish_victory_mode().unwrap(),
            SkirmishVictoryMode::Score
        );
        let seed = skirmish.start_first_contact_battle(map()).unwrap();
        assert!(seed.skirmish.enabled);
        assert_eq!(seed.skirmish.player_faction, CampaignFaction::AshenCompact);
        assert_eq!(seed.skirmish.starting_resources, 500);
        assert_eq!(seed.skirmish.victory_mode, SkirmishVictoryMode::Score);
        seed.validate().unwrap();
    }

    #[test]
    fn regional_quests_have_typed_approaches_deadlines_failure_recovery_and_market_resale() {
        let mut diplomatic = CampaignSaveV1::default();
        diplomatic
            .move_to(CampaignRoom::MarketWindPavilion)
            .unwrap();
        diplomatic.talk_to_regional_npc().unwrap();
        diplomatic.talk_to_regional_npc().unwrap();
        diplomatic.start_regional_quest("market_debt").unwrap();
        assert_eq!(
            diplomatic.cycle_regional_quest_approach().unwrap(),
            QuestApproach::Diplomatic
        );
        diplomatic.advance_active_regional_quest().unwrap();
        diplomatic.move_to(CampaignRoom::MirrorSquare).unwrap();
        diplomatic.move_to(CampaignRoom::MentorHall).unwrap();
        diplomatic.move_to(CampaignRoom::WorkshopGate).unwrap();
        diplomatic.advance_active_regional_quest().unwrap();
        assert_eq!(
            diplomatic.regional_quest_states.get("market_debt"),
            Some(&QuestState::Completed)
        );

        let mut expired = CampaignSaveV1::default();
        expired.move_to(CampaignRoom::MarketWindPavilion).unwrap();
        expired.talk_to_regional_npc().unwrap();
        expired.start_regional_quest("market_debt").unwrap();
        expired.world_clock.day += 2;
        expired.advance_active_regional_quest().unwrap();
        assert_eq!(
            expired.regional_quest_states.get("market_debt"),
            Some(&QuestState::Failed)
        );
        assert_eq!(expired.regional_quest_failure_counts["market_debt"], 1);
        expired.start_regional_quest("market_debt").unwrap();
        assert_eq!(
            expired
                .active_regional_quest_runtime
                .as_ref()
                .unwrap()
                .failure_count,
            1
        );

        let mut market = CampaignSaveV1::default();
        market.move_to(CampaignRoom::MarketWindPavilion).unwrap();
        while market.selected_shop_item().id != "salvaged-alloy" {
            market.cycle_shop_item().unwrap();
        }
        market.buy_selected_shop_item().unwrap();
        let credits_after_buy = market.progression.credits;
        market.sell_selected_shop_item().unwrap();
        assert!(market.progression.credits > credits_after_buy);
        assert!(market
            .progression
            .world_flags
            .iter()
            .any(|flag| flag.starts_with("market_sale_salvaged-alloy")));
    }

    fn test_room(room_id: &str) -> CampaignRoom {
        CampaignRoom::from_id(room_id).unwrap_or_else(|| panic!("unknown test room {room_id}"))
    }

    fn walk_to(campaign: &mut CampaignSaveV1, destination: &str) {
        while campaign.room.id() != destination {
            let mut flags = campaign.progression.world_flags.clone();
            if campaign.active_title == Some(BuildTitle::RelayRunner) {
                flags.insert("signal_road_secured".to_string());
            }
            let route =
                mirror_city_world_graph().shortest_route(campaign.room.id(), destination, &flags);
            assert!(
                route.reachable(),
                "real world route {} -> {} was blocked: {:?}",
                campaign.room.id(),
                destination,
                route.blocked_reason,
            );
            let next = route.path.get(1).expect("route must advance");
            campaign.move_to(test_room(next)).unwrap();
        }
    }

    fn finish_authored_quest_on(
        campaign: &mut CampaignSaveV1,
        quest_id: &str,
        approach: QuestApproach,
    ) {
        if !campaign.mentor_met {
            walk_to(campaign, MENTOR_HALL_ROOM);
            campaign.talk_to_mentor().unwrap();
            // The authored-quest matrix begins after the three-mission
            // campaign prologue. These prerequisite flags are fixture scope;
            // room traversal and quest/encounter state may not be mutated.
            campaign
                .progression
                .world_flags
                .extend(["signal_road_secured", "outer_signal_road_open"].map(str::to_string));
        }
        let definition = REGIONAL_QUEST_CATALOG
            .iter()
            .find(|quest| quest.id == quest_id)
            .unwrap();
        let giver = NPC_CATALOG
            .iter()
            .find(|npc| npc.id == definition.giver_npc_id)
            .unwrap();
        walk_to(campaign, giver.room_id);
        let rule = quest_runtime_rule(definition.archetype);
        let required_trust = if approach == QuestApproach::Diplomatic {
            rule.minimum_trust_for_diplomacy
        } else {
            1
        };
        for _ in 0..96 {
            let relationship = campaign.npc_relationships.get(giver.id).unwrap();
            if relationship.interactions > 0 && relationship.trust >= required_trust {
                break;
            }
            if campaign
                .current_regional_npc()
                .is_some_and(|npc| npc.id == giver.id)
            {
                campaign.talk_to_regional_npc().unwrap();
            } else {
                campaign.wait_in_town(120).unwrap();
            }
        }
        let relationship = campaign.npc_relationships.get(giver.id).unwrap();
        assert!(relationship.interactions > 0);
        assert!(relationship.trust >= required_trust);
        if approach == QuestApproach::Resourceful {
            for _ in 0..rule.resource_quantity {
                let mut purchased = false;
                for market_room in [
                    MARKET_WIND_PAVILION_ROOM,
                    RELAY_QUARTER_ROOM,
                    GLASS_BASIN_WAYHOUSE_ROOM,
                    CINDER_REFUGE_ROOM,
                ] {
                    walk_to(campaign, market_room);
                    if campaign.buy_regional_item(rule.resource_item_id).is_ok() {
                        purchased = true;
                        break;
                    }
                }
                assert!(
                    purchased,
                    "regional markets could not supply {} for {quest_id}",
                    rule.resource_item_id
                );
            }
            walk_to(campaign, giver.room_id);
        }
        campaign.start_regional_quest(quest_id).unwrap();
        let deadline = campaign
            .active_regional_quest_runtime
            .as_ref()
            .unwrap()
            .deadline_day;
        while campaign.world_clock.day <= deadline {
            campaign.wait_in_town(120).unwrap();
        }
        campaign.advance_active_regional_quest().unwrap();
        assert_eq!(
            campaign.regional_quest_states.get(quest_id),
            Some(&QuestState::Failed),
            "{quest_id} must persist a deadline failure before its retry path",
        );
        campaign.start_regional_quest(quest_id).unwrap();
        while campaign
            .active_regional_quest_runtime
            .as_ref()
            .is_some_and(|runtime| runtime.approach != approach)
        {
            campaign.cycle_regional_quest_approach().unwrap();
        }
        while let Some(waypoint) = campaign
            .active_regional_quest_ready_rooms()
            .into_iter()
            .last()
        {
            walk_to(campaign, &waypoint);
            campaign.advance_active_regional_quest().unwrap();
        }
        if let (QuestApproach::Direct, Some(encounter_id)) = (approach, definition.encounter_id) {
            if !campaign
                .progression
                .world_flags
                .contains(&format!("{encounter_id}_cleared"))
            {
                campaign.advance_active_regional_quest().unwrap();
                while campaign.active_encounter.is_some() {
                    let encounter = campaign.active_encounter.as_ref().unwrap();
                    let action = if encounter.technique_cooldown == 0 && encounter.momentum >= 2 {
                        if encounter.round.is_multiple_of(2) {
                            EncounterAction::PrimaryTechnique
                        } else {
                            EncounterAction::SecondaryTechnique
                        }
                    } else if encounter.momentum < 2 {
                        EncounterAction::Defend
                    } else {
                        EncounterAction::Attack
                    };
                    campaign.act_in_signal_road_encounter(action).unwrap();
                }
                assert_eq!(
                    campaign.last_encounter_outcome,
                    Some(EncounterOutcome::Victory)
                );
            }
        }
        if campaign.active_regional_quest_id.is_some() {
            let graph = quest_condition_graph(definition, approach);
            let settlement = graph
                .nodes
                .iter()
                .find(|node| node.kind == trnm_rpg_core::QuestConditionKind::ReturnForSettlement)
                .unwrap();
            walk_to(campaign, &settlement.subject_id);
            campaign.complete_regional_quest().unwrap();
        }
        if let Some(pending) = campaign.pending_main_story_chapter {
            let chapter = MAIN_STORY_CHAPTERS
                .iter()
                .find(|chapter| chapter.chapter == pending)
                .unwrap();
            walk_to(campaign, chapter.room_id);
            campaign.resolve_pending_main_story_chapter().unwrap();
        }
    }

    #[test]
    fn all_fifteen_authored_quests_complete_through_all_three_branches() {
        for approach in [
            QuestApproach::Direct,
            QuestApproach::Diplomatic,
            QuestApproach::Resourceful,
        ] {
            let mut campaign = CampaignSaveV1::default();
            for quest in REGIONAL_QUEST_CATALOG {
                finish_authored_quest_on(&mut campaign, quest.id, approach);
                assert_eq!(
                    campaign.regional_quest_states.get(quest.id),
                    Some(&QuestState::Completed),
                    "{} {approach:?} did not reach its terminal branch",
                    quest.id
                );
                assert!(campaign.progression.world_flags.contains(
                    &format!("regional_quest_{}_{approach:?}", quest.id).to_ascii_lowercase()
                ));
            }
        }
    }

    #[test]
    fn authored_forks_change_the_live_route_and_accept_either_ready_branch_first() {
        let mut campaign = CampaignSaveV1::default();
        campaign
            .progression
            .world_flags
            .extend(["signal_road_secured", "outer_signal_road_open"].map(str::to_string));
        let definition = REGIONAL_QUEST_CATALOG
            .iter()
            .find(|quest| quest.id == "broken_milestone")
            .unwrap();
        let giver = NPC_CATALOG
            .iter()
            .find(|npc| npc.id == definition.giver_npc_id)
            .unwrap();
        walk_to(&mut campaign, giver.room_id);
        campaign.talk_to_regional_npc().unwrap();
        campaign.start_regional_quest(definition.id).unwrap();
        let first_ready = campaign.active_regional_quest_ready_rooms();
        assert!(
            first_ready.len() >= 2,
            "the authored fork must expose route choice"
        );
        walk_to(&mut campaign, first_ready.last().unwrap());
        campaign.advance_active_regional_quest().unwrap();
        let after_branch = campaign.active_regional_quest_ready_rooms();
        assert_ne!(after_branch, first_ready);
        assert!(
            after_branch.is_empty(),
            "the mutually exclusive sibling must close"
        );
        assert!(campaign
            .progression
            .world_flags
            .iter()
            .any(|flag| flag.starts_with("quest_route_broken_milestone_")));
        let route = campaign.current_task_route_plan();
        assert_eq!(route.destination_room_id, definition.waypoint_room_ids[1]);
    }

    #[test]
    fn each_main_story_chapter_records_an_independent_irreversible_choice() {
        let mut campaign = CampaignSaveV1::default();
        let choices = [
            MainStoryChoice::ProtectWayhouses,
            MainStoryChoice::ExposeConspiracy,
            MainStoryChoice::ForgeAccord,
        ];
        for (chapter_index, choice) in choices.into_iter().enumerate() {
            let threshold = (chapter_index + 1) * 5;
            campaign.main_story_choice = choice;
            let start = chapter_index * 5;
            for quest in REGIONAL_QUEST_CATALOG.iter().skip(start).take(5) {
                finish_authored_quest_on(&mut campaign, quest.id, QuestApproach::Direct);
            }
            assert_eq!(campaign.main_story_decisions.len(), chapter_index + 1);
            assert_eq!(campaign.main_story_decisions.last().unwrap().choice, choice);
            assert_eq!(
                campaign
                    .regional_quest_states
                    .values()
                    .filter(|state| **state == QuestState::Completed)
                    .count(),
                threshold
            );
        }
        assert_eq!(campaign.main_story_decisions.len(), 3);
        assert_eq!(
            campaign
                .main_story_decisions
                .iter()
                .map(|decision| decision.choice)
                .collect::<Vec<_>>(),
            choices
        );
        assert_eq!(
            campaign
                .main_story_decisions
                .iter()
                .map(|decision| decision.chapter)
                .collect::<BTreeSet<_>>()
                .len(),
            3
        );
        assert_eq!(
            campaign.main_story_ending,
            Some(MainStoryEnding::ThreeRoadCompact)
        );
        for chapter in MAIN_STORY_CHAPTERS {
            assert!(campaign
                .progression
                .world_flags
                .contains(&format!("main_story_scene_{}_resolved", chapter.scene_id)));
        }
    }

    #[test]
    fn all_five_explicit_main_story_endings_are_resolved() {
        let decisions = |choices: [MainStoryChoice; 3]| {
            MAIN_STORY_CHAPTERS
                .iter()
                .zip(choices)
                .map(|(chapter, choice)| MainStoryDecisionRecord {
                    chapter: chapter.chapter,
                    choice,
                    outcome_flag: format!("test_{:?}_{choice:?}", chapter.chapter),
                    day: 1,
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(
            resolve_main_story_ending(&decisions([MainStoryChoice::ProtectWayhouses; 3])),
            Some(MainStoryEnding::WayhouseLeague)
        );
        assert_eq!(
            resolve_main_story_ending(&decisions([MainStoryChoice::ExposeConspiracy; 3])),
            Some(MainStoryEnding::OpenArchiveRepublic)
        );
        assert_eq!(
            resolve_main_story_ending(&decisions([MainStoryChoice::ForgeAccord; 3])),
            Some(MainStoryEnding::FrontierAccord)
        );
        assert_eq!(
            resolve_main_story_ending(&decisions([
                MainStoryChoice::ProtectWayhouses,
                MainStoryChoice::ExposeConspiracy,
                MainStoryChoice::ForgeAccord,
            ])),
            Some(MainStoryEnding::ThreeRoadCompact)
        );
        assert_eq!(
            resolve_main_story_ending(&decisions([
                MainStoryChoice::ProtectWayhouses,
                MainStoryChoice::ProtectWayhouses,
                MainStoryChoice::ForgeAccord,
            ])),
            Some(MainStoryEnding::ContestedMandate)
        );
    }

    #[test]
    fn sect_skill_tree_changes_battle_seed_and_crafting_consumes_materials() {
        let mut baseline = ready_campaign();
        let baseline_seed = baseline.start_first_contact_battle(map()).unwrap();
        let mut artisan = ready_campaign();
        artisan.move_to(CampaignRoom::MentorHall).unwrap();
        artisan.move_to(CampaignRoom::WorkshopGate).unwrap();
        artisan.join_regional_sect(SectId::IronWorkshop).unwrap();
        artisan.progression.inventory.extend([
            LootStack {
                item_id: "salvaged-alloy".to_string(),
                quantity: 3,
            },
            LootStack {
                item_id: "route-token".to_string(),
                quantity: 1,
            },
        ]);
        artisan.craft_regional_item("reinforced_staff").unwrap();
        assert!(artisan
            .character
            .inventory_items
            .iter()
            .any(|item| item.item_id == "reinforced-staff"));
        artisan.move_to(CampaignRoom::MentorHall).unwrap();
        artisan.move_to(CampaignRoom::ExpeditionGate).unwrap();
        let artisan_seed = artisan.start_first_contact_battle(map()).unwrap();
        assert_ne!(baseline_seed.seed_hash, artisan_seed.seed_hash);
        assert_eq!(artisan_seed.sect_id.as_deref(), Some("iron_workshop_gate"));
        assert!(artisan_seed.party[0].stats.armor > baseline_seed.party[0].stats.armor);
    }

    #[test]
    fn sect_technique_mastery_persists_and_changes_later_encounter_authority() {
        let mut campaign = CampaignSaveV1::default();
        campaign.character.sect_id = Some("iron_workshop_gate".to_string());
        campaign.equipped_technique_slot = 1;
        campaign
            .technique_mastery
            .insert("relay_hammer".to_string(), 50);
        campaign.room = CampaignRoom::RelayQuarter;
        campaign.begin_signal_road_encounter().unwrap();
        assert_eq!(
            campaign.active_encounter.as_ref().unwrap().technique_rank,
            5
        );
        campaign.active_encounter.as_mut().unwrap().momentum = 3;
        campaign
            .act_in_signal_road_encounter(EncounterAction::Technique)
            .unwrap();
        assert_eq!(campaign.technique_mastery["relay_hammer"], 51);
        let directory = tempdir().unwrap();
        let store = CampaignStore::new(directory.path().join("mastery.json"));
        store.save_atomic(&campaign).unwrap();
        assert_eq!(store.load().unwrap().technique_mastery["relay_hammer"], 51);
    }

    #[test]
    fn primary_secondary_techniques_chain_and_regional_logistics_persist() {
        let mut campaign = CampaignSaveV1::default();
        campaign.character.sect_id = Some("iron_workshop_gate".to_string());
        campaign.equipped_technique_slot = 0;
        campaign.secondary_technique_slot = 1;
        campaign.room = CampaignRoom::RelayQuarter;
        campaign.begin_signal_road_encounter().unwrap();
        campaign.active_encounter.as_mut().unwrap().momentum = 8;
        campaign
            .act_in_signal_road_encounter(EncounterAction::Technique)
            .unwrap();
        campaign
            .act_in_signal_road_encounter(EncounterAction::Defend)
            .unwrap();
        campaign
            .act_in_signal_road_encounter(EncounterAction::Defend)
            .unwrap();
        campaign.active_encounter.as_mut().unwrap().momentum = 8;
        campaign
            .act_in_signal_road_encounter(EncounterAction::Technique)
            .unwrap();
        assert_eq!(campaign.technique_mastery["forge_counter"], 1);
        assert_eq!(campaign.technique_mastery["relay_hammer"], 1);
        campaign.active_encounter = None;
        let before = campaign.regional_market_stock.clone();
        campaign.wait_in_town(120).unwrap();
        assert!(!campaign.active_regional_caravans.is_empty());
        for _ in 0..8 {
            campaign.wait_in_town(120).unwrap();
            if !campaign.regional_logistics.is_empty() {
                break;
            }
        }
        assert!(!campaign.regional_logistics.is_empty());
        assert_ne!(campaign.regional_market_stock, before);
        assert!(campaign.npc_work_output.values().any(|output| *output > 0));
        let directory = tempdir().unwrap();
        let store = CampaignStore::new(directory.path().join("regional-economy.json"));
        store.save_atomic(&campaign).unwrap();
        let loaded = store.load().unwrap();
        assert_eq!(loaded.schema_revision, 12);
        assert_eq!(loaded.regional_logistics, campaign.regional_logistics);
        assert_eq!(loaded.technique_mastery, campaign.technique_mastery);
    }

