use tempfile::tempdir;
use trnm_campaign_core::{
    BattleGridPoint, BattleMapNodeV1, BattleMapSeedV1, BattleOutcome, CampaignMission,
    CampaignPhase, CampaignRoom, CampaignSaveV1, CampaignStore, QuestState,
};
use trnm_rts_protocol::{RtsFrameOrder, RtsOrderKind, RtsOrderSource, RtsTile};
use trnm_rts_sim::{
    BattlePhase, MissionSimV1, SimCheckpointStore, FIVE_MINUTE_TICKS, THREE_MINUTE_TICKS,
};

fn map() -> BattleMapSeedV1 {
    BattleMapSeedV1 {
        width: 20,
        height: 10,
        terrain_rows: vec!["gggggggggggggggggggg".to_string(); 10],
        party_start: BattleGridPoint::new(1, 8),
        approach_point: BattleGridPoint::new(7, 6),
        objective: BattleGridPoint::new(18, 1),
        resource_nodes: vec![BattleMapNodeV1 {
            id: "amber_mid".to_string(),
            position: BattleGridPoint::new(8, 7),
        }],
        enemy_spawns: vec![
            BattleMapNodeV1 {
                id: "contact_scout".to_string(),
                position: BattleGridPoint::new(10, 5),
            },
            BattleMapNodeV1 {
                id: "contact_warden".to_string(),
                position: BattleGridPoint::new(12, 4),
            },
            BattleMapNodeV1 {
                id: "contact_striker".to_string(),
                position: BattleGridPoint::new(14, 3),
            },
            BattleMapNodeV1 {
                id: "relay_guard".to_string(),
                position: BattleGridPoint::new(16, 2),
            },
        ],
    }
}

fn aftershock_map() -> BattleMapSeedV1 {
    let mut authored = map();
    authored.terrain_rows[0] = "bbbbbbbbgggggggggggg".to_string();
    authored
}

fn convoy_map() -> BattleMapSeedV1 {
    let mut authored = map();
    authored.approach_point = BattleGridPoint::new(9, 6);
    authored.objective = BattleGridPoint::new(18, 2);
    authored.terrain_rows[1] = "gggggggggggggggbbbbg".to_string();
    authored
}

fn mirror_siege_map() -> BattleMapSeedV1 {
    let mut authored = map();
    authored.approach_point = BattleGridPoint::new(8, 6);
    authored.objective = BattleGridPoint::new(18, 2);
    authored.terrain_rows[2] = "ggggbbbbggggrrrrrggg".to_string();
    authored.enemy_spawns.push(BattleMapNodeV1 {
        id: "siege_commander".to_string(),
        position: BattleGridPoint::new(17, 3),
    });
    authored
}

fn ready_campaign() -> CampaignSaveV1 {
    let mut campaign = CampaignSaveV1::default();
    campaign.move_to(CampaignRoom::MentorHall).unwrap();
    campaign.talk_to_mentor().unwrap();
    campaign.train_with_mentor().unwrap();
    campaign.equip_starter_weapon().unwrap();
    campaign.cycle_party_preset().unwrap();
    campaign.move_to(CampaignRoom::ExpeditionGate).unwrap();
    campaign.accept_first_contact_quest().unwrap();
    campaign
}

fn order(sim: &MissionSimV1, kind: RtsOrderKind, target: BattleGridPoint) -> RtsFrameOrder {
    let mut order = RtsFrameOrder::new(
        sim.tick as u32,
        "player",
        sim.party
            .iter()
            .filter(|unit| unit.alive())
            .map(|unit| unit.unit_id.clone())
            .collect::<Vec<_>>(),
        kind,
        RtsOrderSource::LocalInput,
    );
    order.target_tile = Some(RtsTile::new(target.x as i32, target.y as i32));
    match kind {
        RtsOrderKind::Harvest => order.target_actor_id = Some("amber_mid".to_string()),
        RtsOrderKind::Attack => order.target_actor_id = Some("relay_beacon".to_string()),
        RtsOrderKind::Extract => order.target_actor_id = Some("expedition_gate".to_string()),
        RtsOrderKind::Ability => order.target_rule_id = Some("party_signature".to_string()),
        RtsOrderKind::Repair => order.target_actor_id = Some("party_field_aid".to_string()),
        RtsOrderKind::Build => order.target_rule_id = Some("field_barricade".to_string()),
        _ => {}
    }
    order
}

fn step_until(sim: &mut MissionSimV1, predicate: impl Fn(&MissionSimV1) -> bool, limit: u64) {
    while !sim.terminal() && !predicate(sim) && sim.tick < limit {
        sim.step().unwrap();
    }
    assert!(
        predicate(sim),
        "condition not reached by tick {} phase {:?} outcome {:?} guard {} capture {} wave {} alive {} order {:?}",
        sim.tick,
        sim.phase,
        sim.outcome,
        sim.relay_guard_hp,
        sim.relay_capture_ticks,
        sim.reinforcement_wave,
        sim.enemies.iter().filter(|enemy| enemy.alive()).count(),
        sim.current_order_kind(),
    );
}

fn run_victory(mut sim: MissionSimV1) -> MissionSimV1 {
    let reinforcement_waves = 2;
    let approach = sim.seed.map.approach_point;
    sim.issue_order(order(&sim, RtsOrderKind::Move, approach))
        .unwrap();
    step_until(&mut sim, |sim| sim.phase == BattlePhase::Contact, 900);
    let resource = sim.seed.map.resource_nodes[0].position;
    sim.issue_order(order(&sim, RtsOrderKind::Harvest, resource))
        .unwrap();
    step_until(&mut sim, |sim| sim.resources_available >= 100, 1_300);
    let objective = sim.seed.map.objective;
    sim.issue_order(order(&sim, RtsOrderKind::Attack, objective))
        .unwrap();
    step_until(
        &mut sim,
        |sim| sim.phase == BattlePhase::Relay && sim.relay_guard_hp <= 0,
        2_700,
    );
    sim.issue_order(order(&sim, RtsOrderKind::Hold, objective))
        .unwrap();
    for wave in 1..=reinforcement_waves {
        step_until(
            &mut sim,
            |sim| sim.reinforcement_wave >= wave,
            FIVE_MINUTE_TICKS,
        );
        if wave == 1 {
            sim.issue_order(order(&sim, RtsOrderKind::Repair, objective))
                .unwrap();
        } else if wave == 2 {
            sim.issue_order(order(&sim, RtsOrderKind::Build, objective))
                .unwrap();
        } else {
            sim.issue_order(order(&sim, RtsOrderKind::Ability, objective))
                .unwrap();
        }
        sim.issue_order(order(&sim, RtsOrderKind::Attack, objective))
            .unwrap();
        step_until(
            &mut sim,
            |sim| sim.enemies.iter().all(|enemy| !enemy.alive()),
            FIVE_MINUTE_TICKS,
        );
        sim.issue_order(order(&sim, RtsOrderKind::Move, objective))
            .unwrap();
        step_until(
            &mut sim,
            |sim| {
                sim.party.iter().any(|unit| {
                    unit.alive()
                        && (unit.position.x - sim.seed.map.objective.x).abs()
                            + (unit.position.y - sim.seed.map.objective.y).abs()
                            <= 2
                })
            },
            FIVE_MINUTE_TICKS,
        );
        sim.issue_order(order(&sim, RtsOrderKind::Hold, objective))
            .unwrap();
    }
    while !sim.terminal() {
        sim.step().unwrap();
    }
    assert_eq!(sim.outcome, Some(BattleOutcome::Victory));
    assert!((THREE_MINUTE_TICKS..=FIVE_MINUTE_TICKS).contains(&sim.tick));
    sim
}

fn run_convoy_victory(mut sim: MissionSimV1) -> MissionSimV1 {
    assert_eq!(sim.phase, BattlePhase::ConvoyEscort);
    let generator = sim.seed.map.approach_point;
    sim.issue_order(order(&sim, RtsOrderKind::Move, generator))
        .unwrap();
    step_until(&mut sim, |sim| sim.objective_index >= 1, 1_200);
    assert_eq!(sim.phase, BattlePhase::GeneratorDefense);
    sim.issue_order(order(&sim, RtsOrderKind::Attack, generator))
        .unwrap();
    step_until(&mut sim, |sim| sim.objective_index >= 2, 2_400);
    assert_eq!(sim.reinforcement_wave, 2);
    assert_eq!(sim.phase, BattlePhase::Extraction);
    let extraction = sim.seed.map.objective;
    sim.issue_order(order(&sim, RtsOrderKind::Move, extraction))
        .unwrap();
    step_until(&mut sim, MissionSimV1::terminal, FIVE_MINUTE_TICKS);
    assert_eq!(sim.outcome, Some(BattleOutcome::Victory));
    sim
}

#[test]
fn standalone_skirmish_configuration_runs_and_settles_back_into_rpg_once() {
    let directory = tempdir().unwrap();
    let store = CampaignStore::new(directory.path().join("standalone-skirmish.json"));
    let mut campaign = CampaignSaveV1::default();
    campaign.prepare_standalone_skirmish().unwrap();
    assert_eq!(campaign.active_mission, CampaignMission::IronDeltaSkirmish);
    campaign.cycle_skirmish_faction().unwrap();
    campaign.cycle_skirmish_resources().unwrap();
    let seed = campaign.start_first_contact_battle(map()).unwrap();
    assert!(seed.skirmish.enabled);
    let mut sim = MissionSimV1::from_seed(seed).unwrap();
    sim.objective_index = sim.seed.mission.objectives.len();
    sim.step().unwrap();
    assert_eq!(sim.outcome, Some(BattleOutcome::Victory));
    let result = sim.into_result().unwrap();
    store
        .stage_result_atomic(&mut campaign, result.clone())
        .unwrap();
    let receipt = store.settle_atomic(&mut campaign).unwrap();
    assert!(!receipt.duplicate);
    assert_eq!(campaign.phase, CampaignPhase::Town);
    assert!(campaign.progression.world_flags.contains("iron_delta_won"));
    let duplicate = campaign.submit_battle_result(result).unwrap();
    assert!(duplicate.duplicate);
    assert_eq!(duplicate.experience_delta, 0);
}

#[test]
fn e2e_first_victory_unlocks_repeatable_aftershock_and_growth_changes_next_seed() {
    let directory = tempdir().unwrap();
    let store = CampaignStore::new(directory.path().join("campaign.json"));
    let mut campaign = ready_campaign();
    let first_seed = campaign.start_first_contact_battle(map()).unwrap();
    let first_terrain_rows = first_seed.map.terrain_rows.clone();
    let first_hero = first_seed
        .party
        .iter()
        .find(|unit| unit.unit_id == "hero")
        .unwrap()
        .stats
        .clone();
    let first_result = run_victory(MissionSimV1::from_seed(first_seed).unwrap())
        .into_result()
        .unwrap();
    campaign.submit_battle_result(first_result).unwrap();
    campaign.equip_relay_core().unwrap();
    campaign.move_to(CampaignRoom::ExpeditionGate).unwrap();
    campaign.accept_first_contact_quest().unwrap();
    let aftershock_seed = campaign
        .start_first_contact_battle(aftershock_map())
        .unwrap();
    assert_eq!(aftershock_seed.map_id, "aftershock_patrol");
    assert_ne!(first_terrain_rows, aftershock_seed.map.terrain_rows);
    let aftershock_hero = aftershock_seed
        .party
        .iter()
        .find(|unit| unit.unit_id == "hero")
        .unwrap();
    assert_ne!(aftershock_hero.stats.max_hp, first_hero.max_hp);
    if aftershock_hero.stats.max_hp < first_hero.max_hp {
        assert!(aftershock_hero.injury_level > 0);
    }
    assert!(aftershock_hero.stats.energy > first_hero.energy);
    let aftershock_result = run_victory(MissionSimV1::from_seed(aftershock_seed).unwrap())
        .into_result()
        .unwrap();
    campaign.submit_battle_result(aftershock_result).unwrap();
    assert_eq!(campaign.progression.aftershock_completions, 1);
    assert!(campaign
        .progression
        .world_flags
        .contains("signal_road_secured"));
    store.save_atomic(&campaign).unwrap();
    let mut restarted = store.load().unwrap();
    restarted.move_to(CampaignRoom::RelayQuarter).unwrap();
    assert_eq!(restarted.room, CampaignRoom::RelayQuarter);
    restarted.move_to(CampaignRoom::MirrorSquare).unwrap();
    restarted.move_to(CampaignRoom::ExpeditionGate).unwrap();
    restarted.accept_first_contact_quest().unwrap();
    assert_eq!(restarted.active_mission.map_id(), "convoy_exodus");
}

#[test]
fn e2e_third_mission_escorts_defends_extracts_and_unlocks_outer_road() {
    let directory = tempdir().unwrap();
    let store = CampaignStore::new(directory.path().join("campaign.json"));
    let mut campaign = ready_campaign();

    let first = campaign.start_first_contact_battle(map()).unwrap();
    let result = run_victory(MissionSimV1::from_seed(first).unwrap())
        .into_result()
        .unwrap();
    campaign.submit_battle_result(result).unwrap();
    campaign.equip_relay_core().unwrap();
    campaign.move_to(CampaignRoom::ExpeditionGate).unwrap();
    campaign.accept_first_contact_quest().unwrap();
    let aftershock = campaign
        .start_first_contact_battle(aftershock_map())
        .unwrap();
    let result = run_victory(MissionSimV1::from_seed(aftershock).unwrap())
        .into_result()
        .unwrap();
    campaign.submit_battle_result(result).unwrap();

    campaign.move_to(CampaignRoom::ExpeditionGate).unwrap();
    campaign.accept_first_contact_quest().unwrap();
    assert_eq!(campaign.active_mission.map_id(), "convoy_exodus");
    let convoy = campaign.start_first_contact_battle(convoy_map()).unwrap();
    assert_eq!(
        convoy
            .mission
            .objectives
            .iter()
            .map(|objective| objective.kind)
            .collect::<Vec<_>>(),
        [
            trnm_campaign_core::ObjectiveKind::Escort,
            trnm_campaign_core::ObjectiveKind::Defend,
            trnm_campaign_core::ObjectiveKind::Extract,
        ]
    );
    let result = run_convoy_victory(MissionSimV1::from_seed(convoy).unwrap())
        .into_result()
        .unwrap();
    campaign.submit_battle_result(result).unwrap();
    assert!(campaign
        .progression
        .world_flags
        .contains("outer_signal_road_open"));
    assert_eq!(
        campaign.story.current_step,
        trnm_campaign_core::StoryStepId::SignalRoadComplete
    );
    store.save_atomic(&campaign).unwrap();
    let mut restarted = store.load().unwrap();
    assert!(restarted
        .progression
        .world_flags
        .contains("convoy_exodus_secured"));
    restarted.move_to(CampaignRoom::ExpeditionGate).unwrap();
    restarted.accept_first_contact_quest().unwrap();
    assert_eq!(restarted.active_mission.map_id(), "mirror_siege");
    let siege = restarted
        .start_first_contact_battle(mirror_siege_map())
        .unwrap();
    assert_eq!(siege.mission.objectives.len(), 3);
    assert_eq!(siege.map.enemy_spawns.len(), 5);
    let result = run_victory(MissionSimV1::from_seed(siege).unwrap())
        .into_result()
        .unwrap();
    restarted.submit_battle_result(result).unwrap();
    assert!(restarted
        .progression
        .world_flags
        .contains("mirror_siege_secured"));
    store.save_atomic(&restarted).unwrap();
    assert!(store
        .load()
        .unwrap()
        .progression
        .world_flags
        .contains("mirror_siege_secured"));
}

#[test]
fn e2e_victory_returns_growth_loot_resources_reputation_and_durable_save() {
    let directory = tempdir().unwrap();
    let store = CampaignStore::new(directory.path().join("campaign.json"));
    let mut campaign = ready_campaign();
    let credits_before = campaign.progression.credits;
    let seed = campaign.start_first_contact_battle(map()).unwrap();
    store.save_atomic(&campaign).unwrap();
    let result = run_victory(MissionSimV1::from_seed(seed).unwrap())
        .into_result()
        .unwrap();
    let receipt = store.submit_result_atomic(&mut campaign, result).unwrap();
    assert_eq!(receipt.outcome, BattleOutcome::Victory);
    assert!(receipt.credit_delta > 0);
    let mut restarted = store.load().unwrap();
    assert_eq!(restarted.phase, CampaignPhase::Town);
    assert_eq!(restarted.room, CampaignRoom::MirrorSquare);
    assert_eq!(restarted.quest_state, QuestState::Completed);
    assert!(restarted.progression.experience > 0);
    assert!(restarted.progression.credits > credits_before);
    assert!(restarted.character.attributes.reputation > 0);
    assert!(restarted
        .progression
        .inventory
        .iter()
        .any(|loot| loot.item_id == "field-tonic-kit"));
    restarted.equip_relay_core().unwrap();
    store.save_atomic(&restarted).unwrap();
    assert!(store
        .load()
        .unwrap()
        .character
        .equipment_slots
        .contains_key("relic"));
}

#[test]
fn e2e_defeat_returns_injuries_and_small_nonfarmable_xp() {
    let mut campaign = ready_campaign();
    let seed = campaign.start_first_contact_battle(map()).unwrap();
    let mut sim = MissionSimV1::from_seed(seed).unwrap();
    let hold = RtsFrameOrder::new(
        0,
        "player",
        sim.party
            .iter()
            .map(|unit| unit.unit_id.clone())
            .collect::<Vec<_>>(),
        RtsOrderKind::Hold,
        RtsOrderSource::LocalInput,
    );
    sim.issue_order(hold).unwrap();
    while !sim.terminal() {
        sim.step().unwrap();
    }
    let receipt = campaign
        .submit_battle_result(sim.into_result().unwrap())
        .unwrap();
    assert_eq!(receipt.outcome, BattleOutcome::Defeat);
    assert!(receipt.experience_delta <= 12);
    assert_eq!(receipt.credit_delta, 0);
    assert_eq!(campaign.quest_state, QuestState::Failed);
    assert!(campaign.party.iter().any(|member| member.injury_level > 0));
}

#[test]
fn e2e_withdrawal_returns_zero_xp_resources_and_fake_victory_rewards() {
    let mut campaign = ready_campaign();
    let seed = campaign.start_first_contact_battle(map()).unwrap();
    let mut sim = MissionSimV1::from_seed(seed).unwrap();
    sim.issue_order(order(&sim, RtsOrderKind::Move, sim.seed.map.approach_point))
        .unwrap();
    for _ in 0..30 {
        sim.step().unwrap();
    }
    sim.issue_order(order(&sim, RtsOrderKind::Extract, sim.seed.map.party_start))
        .unwrap();
    let receipt = campaign
        .submit_battle_result(sim.into_result().unwrap())
        .unwrap();
    assert_eq!(receipt.outcome, BattleOutcome::Withdrawal);
    assert_eq!(receipt.experience_delta, 0);
    assert_eq!(receipt.credit_delta, 0);
    assert!(receipt.loot_delta.is_empty());
    assert_eq!(campaign.quest_state, QuestState::Withdrawn);
}

#[test]
fn e2e_battle_crash_resumes_the_same_deterministic_snapshot() {
    let directory = tempdir().unwrap();
    let checkpoint_store =
        SimCheckpointStore::new(directory.path().join("first-contact-battle.json"));
    let mut campaign = ready_campaign();
    let seed = campaign.start_first_contact_battle(map()).unwrap();
    let mut baseline = MissionSimV1::from_seed(seed.clone()).unwrap();
    baseline
        .issue_order(order(
            &baseline,
            RtsOrderKind::Move,
            baseline.seed.map.approach_point,
        ))
        .unwrap();
    for _ in 0..280 {
        baseline.step().unwrap();
    }
    checkpoint_store.save_atomic(&baseline).unwrap();
    let resumed = checkpoint_store.load_for_seed(&seed).unwrap().unwrap();
    assert_eq!(resumed, baseline);
    assert_eq!(
        resumed.snapshot_hash().unwrap(),
        baseline.snapshot_hash().unwrap()
    );
}

#[test]
fn e2e_settlement_crash_recovers_pending_result_exactly_once() {
    let directory = tempdir().unwrap();
    let store = CampaignStore::new(directory.path().join("campaign.json"));
    let mut campaign = ready_campaign();
    let seed = campaign.start_first_contact_battle(map()).unwrap();
    let result = run_victory(MissionSimV1::from_seed(seed).unwrap())
        .into_result()
        .unwrap();
    store.stage_result_atomic(&mut campaign, result).unwrap();
    let mut restarted = store.load().unwrap();
    assert_eq!(restarted.phase, CampaignPhase::PostBattlePending);
    let receipt = store
        .recover_pending_settlement(&mut restarted)
        .unwrap()
        .unwrap();
    assert!(!receipt.duplicate);
    assert!(store
        .recover_pending_settlement(&mut restarted)
        .unwrap()
        .is_none());
    assert_eq!(store.load().unwrap().settlement_receipts.len(), 1);
}

#[test]
fn e2e_duplicate_result_has_zero_delta_and_cannot_change_the_save() {
    let mut campaign = ready_campaign();
    let seed = campaign.start_first_contact_battle(map()).unwrap();
    let result = run_victory(MissionSimV1::from_seed(seed).unwrap())
        .into_result()
        .unwrap();
    let first = campaign.submit_battle_result(result.clone()).unwrap();
    assert!(!first.duplicate);
    let after_first = campaign.clone();
    let second = campaign.submit_battle_result(result).unwrap();
    assert!(second.duplicate);
    assert_eq!(second.experience_delta, 0);
    assert_eq!(second.credit_delta, 0);
    assert_eq!(second.reputation_delta, 0);
    assert!(second.loot_delta.is_empty());
    assert_eq!(campaign, after_first);
}
