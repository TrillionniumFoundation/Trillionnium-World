use tempfile::tempdir;
use trnm_campaign_core::{
    BattleOutcome, CampaignPhase, CampaignRoom, CampaignSaveV1, CampaignStore, QuestState,
};
use trnm_rts_sim::{
    MissionSimV1, SimCheckpointStore, SimCommand, FIFTEEN_MINUTE_TICKS, TEN_MINUTE_TICKS,
};

fn ready_campaign() -> CampaignSaveV1 {
    let mut campaign = CampaignSaveV1::default();
    campaign.move_to(CampaignRoom::MentorHall).unwrap();
    campaign.talk_to_mentor().unwrap();
    campaign.train_with_mentor().unwrap();
    campaign.equip_starter_weapon().unwrap();
    campaign
        .select_party(vec![
            "hero".to_string(),
            "aya".to_string(),
            "mako".to_string(),
            "tess".to_string(),
        ])
        .unwrap();
    campaign.move_to(CampaignRoom::ExpeditionGate).unwrap();
    campaign.accept_first_contact_quest().unwrap();
    campaign
}

fn run(mut sim: MissionSimV1, command: SimCommand) -> MissionSimV1 {
    while !sim.terminal() && sim.tick <= FIFTEEN_MINUTE_TICKS {
        sim.step(command).unwrap();
    }
    assert!(sim.terminal());
    sim
}

#[test]
fn e2e_victory_returns_growth_loot_reputation_and_durable_save() {
    let directory = tempdir().unwrap();
    let store = CampaignStore::new(directory.path().join("campaign.json"));
    let mut campaign = ready_campaign();
    let seed = campaign.start_first_contact_battle().unwrap();
    store.save_atomic(&campaign).unwrap();
    let sim = run(MissionSimV1::from_seed(seed).unwrap(), SimCommand::Assault);
    assert_eq!(sim.outcome, Some(BattleOutcome::Victory));
    assert!((TEN_MINUTE_TICKS..=FIFTEEN_MINUTE_TICKS).contains(&sim.tick));
    let result = sim.into_result().unwrap();
    let receipt = store.submit_result_atomic(&mut campaign, result).unwrap();
    assert_eq!(receipt.outcome, BattleOutcome::Victory);
    let restarted = store.load().unwrap();
    assert_eq!(restarted.phase, CampaignPhase::Town);
    assert_eq!(restarted.room, CampaignRoom::MirrorSquare);
    assert_eq!(restarted.quest_state, QuestState::Completed);
    assert!(restarted.progression.experience > 0);
    assert!(restarted.character.attributes.reputation > 0);
    assert!(restarted
        .progression
        .inventory
        .iter()
        .any(|loot| loot.item_id == "relay-core-fragment"));
}

#[test]
fn e2e_defeat_returns_injuries_and_retryable_quest() {
    let mut campaign = ready_campaign();
    let seed = campaign.start_first_contact_battle().unwrap();
    let sim = run(MissionSimV1::from_seed(seed).unwrap(), SimCommand::Hold);
    assert_eq!(sim.outcome, Some(BattleOutcome::Defeat));
    let receipt = campaign
        .submit_battle_result(sim.into_result().unwrap())
        .unwrap();
    assert_eq!(receipt.outcome, BattleOutcome::Defeat);
    assert_eq!(campaign.quest_state, QuestState::Failed);
    assert!(campaign.party.iter().any(|member| member.injury_level > 0));
    assert_eq!(campaign.room, CampaignRoom::MirrorSquare);
}

#[test]
fn e2e_withdrawal_returns_without_fake_victory_rewards() {
    let mut campaign = ready_campaign();
    let seed = campaign.start_first_contact_battle().unwrap();
    let mut sim = MissionSimV1::from_seed(seed).unwrap();
    for _ in 0..200 {
        sim.step(SimCommand::Advance).unwrap();
    }
    sim.step(SimCommand::Retreat).unwrap();
    let receipt = campaign
        .submit_battle_result(sim.into_result().unwrap())
        .unwrap();
    assert_eq!(receipt.outcome, BattleOutcome::Withdrawal);
    assert!(receipt.loot_delta.is_empty());
    assert_eq!(campaign.quest_state, QuestState::Withdrawn);
    assert!(!campaign
        .progression
        .world_flags
        .contains("first_contact_secured"));
}

#[test]
fn e2e_battle_crash_resumes_the_same_deterministic_snapshot() {
    let directory = tempdir().unwrap();
    let checkpoint_store =
        SimCheckpointStore::new(directory.path().join("first-contact-battle.json"));
    let mut campaign = ready_campaign();
    let seed = campaign.start_first_contact_battle().unwrap();
    let mut baseline = MissionSimV1::from_seed(seed.clone()).unwrap();
    for _ in 0..2_800 {
        baseline.step(SimCommand::Assault).unwrap();
    }
    checkpoint_store.save_atomic(&baseline).unwrap();
    let mut resumed = checkpoint_store.load_for_seed(&seed).unwrap().unwrap();
    while !baseline.terminal() {
        baseline.step(SimCommand::Assault).unwrap();
    }
    while !resumed.terminal() {
        resumed.step(SimCommand::Assault).unwrap();
    }
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
    let seed = campaign.start_first_contact_battle().unwrap();
    let result = run(MissionSimV1::from_seed(seed).unwrap(), SimCommand::Assault)
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
    let seed = campaign.start_first_contact_battle().unwrap();
    let result = run(MissionSimV1::from_seed(seed).unwrap(), SimCommand::Assault)
        .into_result()
        .unwrap();
    let first = campaign.submit_battle_result(result.clone()).unwrap();
    assert!(!first.duplicate);
    let after_first = campaign.clone();
    let second = campaign.submit_battle_result(result).unwrap();
    assert!(second.duplicate);
    assert_eq!(second.experience_delta, 0);
    assert_eq!(second.reputation_delta, 0);
    assert!(second.loot_delta.is_empty());
    assert_eq!(campaign, after_first);
}
