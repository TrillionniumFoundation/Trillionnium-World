use super::map_loader::{FirstContactMap, MissionMapCatalog};
use super::simulation_adapter::{
    FirstContactCommand, FirstContactRuntime, FirstContactSimulationAdapter,
};
use bevy::prelude::*;
use std::path::{Path, PathBuf};
use trnm_campaign_core::{
    CampaignError, CampaignPhase, CampaignRoom, CampaignSaveV1, CampaignStore, EncounterAction,
    QuestState, SettlementReceiptV1,
};
use trnm_rts_sim::{MissionSimV1, SimCheckpointStore};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CampaignMode {
    Town,
    Battle,
    Debrief,
}

#[derive(Resource, Debug, Clone)]
pub(super) struct CampaignFlow {
    pub save: CampaignSaveV1,
    pub mode: CampaignMode,
    pub mission: Option<MissionSimV1>,
    pub last_receipt: Option<SettlementReceiptV1>,
    pub status: String,
    pub last_checkpoint_tick: u64,
    store: CampaignStore,
    checkpoint_store: SimCheckpointStore,
}

impl CampaignFlow {
    pub fn load() -> Result<Self, String> {
        let save_path = campaign_save_path();
        let store = CampaignStore::new(&save_path);
        let checkpoint_store = SimCheckpointStore::new(checkpoint_path_for(&save_path));
        let mut save = store.load_or_default().map_err(|error| error.to_string())?;
        let mut last_receipt = None;
        let mut status = "Campaign ready".to_string();
        if save.phase == CampaignPhase::PostBattlePending {
            last_receipt = store
                .recover_pending_settlement(&mut save)
                .map_err(|error| error.to_string())?;
            status = "Recovered pending battle settlement after restart".to_string();
        }
        let mission = if save.phase == CampaignPhase::BattlePending {
            let seed = &save
                .pending_battle
                .as_ref()
                .ok_or_else(|| "battle phase is missing its seed".to_string())?
                .seed;
            match checkpoint_store
                .load_for_seed(seed)
                .map_err(|error| error.to_string())?
            {
                Some(sim) => {
                    status = format!("Resumed battle checkpoint at tick {}", sim.tick);
                    Some(sim)
                }
                None => {
                    status = "Resumed pending battle from its authoritative seed".to_string();
                    Some(MissionSimV1::from_seed(seed.clone()).map_err(|error| error.to_string())?)
                }
            }
        } else {
            None
        };
        let mode = if mission.is_some() {
            CampaignMode::Battle
        } else if last_receipt.is_some() {
            CampaignMode::Debrief
        } else {
            CampaignMode::Town
        };
        Ok(Self {
            save,
            mode,
            mission,
            last_receipt,
            status,
            last_checkpoint_tick: 0,
            store,
            checkpoint_store,
        })
    }

    pub fn in_battle(&self) -> bool {
        self.mode == CampaignMode::Battle
    }

    pub fn mutate_town<F>(&mut self, mutation: F) -> Result<(), CampaignError>
    where
        F: FnOnce(&mut CampaignSaveV1) -> Result<(), CampaignError>,
    {
        let mut candidate = self.save.clone();
        mutation(&mut candidate)?;
        self.store.save_atomic(&candidate)?;
        self.save = candidate;
        Ok(())
    }

    pub fn start_battle(&mut self, map: &FirstContactMap) -> Result<(), String> {
        let mut candidate = self.save.clone();
        let battle_map = map.battle_seed_map()?;
        let seed = candidate
            .start_first_contact_battle(battle_map)
            .map_err(|error| error.to_string())?;
        let mission = MissionSimV1::from_seed(seed).map_err(|error| error.to_string())?;
        self.store
            .save_atomic(&candidate)
            .map_err(|error| error.to_string())?;
        self.checkpoint_store
            .save_atomic(&mission)
            .map_err(|error| error.to_string())?;
        self.save = candidate;
        self.mission = Some(mission);
        self.last_receipt = None;
        self.last_checkpoint_tick = 0;
        self.mode = CampaignMode::Battle;
        self.status = format!(
            "BattleSeed accepted: {} deployed",
            self.save.active_mission.display_name()
        );
        Ok(())
    }

    pub fn checkpoint_if_due(&mut self) -> Result<(), String> {
        let Some(mission) = self.mission.as_ref() else {
            return Ok(());
        };
        if mission.terminal() || mission.tick < self.last_checkpoint_tick + 100 {
            return Ok(());
        }
        self.checkpoint_store
            .save_atomic(mission)
            .map_err(|error| error.to_string())?;
        self.last_checkpoint_tick = mission.tick;
        Ok(())
    }

    fn complete_terminal_battle(&mut self) -> Result<SettlementReceiptV1, String> {
        let mission = self
            .mission
            .take()
            .ok_or_else(|| "terminal battle simulation is missing".to_string())?;
        let result = mission.into_result().map_err(|error| error.to_string())?;
        self.store
            .stage_result_atomic(&mut self.save, result)
            .map_err(|error| error.to_string())?;
        let receipt = self
            .store
            .settle_atomic(&mut self.save)
            .map_err(|error| error.to_string())?;
        self.status = format!(
            "Settlement applied once: +{} XP, reputation {:+}",
            receipt.experience_delta, receipt.reputation_delta
        );
        self.last_receipt = Some(receipt.clone());
        self.mode = CampaignMode::Debrief;
        Ok(receipt)
    }
}

fn campaign_save_path() -> PathBuf {
    std::env::var_os("TRNM_CAMPAIGN_SAVE_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../run/campaign/campaign.json")
        })
}

fn checkpoint_path_for(save_path: &Path) -> PathBuf {
    save_path.with_file_name("first-contact-battle.json")
}

fn set_status(flow: &mut CampaignFlow, result: Result<(), CampaignError>, success: &str) {
    flow.status = match result {
        Ok(()) => success.to_string(),
        Err(error) => error.to_string(),
    };
}

pub(super) fn handle_campaign_input(
    input: Res<ButtonInput<KeyCode>>,
    mut map: ResMut<FirstContactMap>,
    maps: Res<MissionMapCatalog>,
    mut flow: ResMut<CampaignFlow>,
    mut runtime: ResMut<FirstContactRuntime>,
    mut adapter: ResMut<FirstContactSimulationAdapter>,
) {
    if flow.mode == CampaignMode::Debrief {
        if input.just_pressed(KeyCode::Enter) {
            flow.mode = CampaignMode::Town;
            flow.last_receipt = None;
            flow.status = "Returned to Mirror Square; campaign save is current".to_string();
        }
        return;
    }
    if flow.mode == CampaignMode::Battle {
        return;
    }

    if flow.save.active_encounter.is_some() {
        let (action, success) = if input.just_pressed(KeyCode::KeyJ) {
            (Some(EncounterAction::Attack), "Struck in the RPG encounter")
        } else if input.just_pressed(KeyCode::KeyR) {
            (
                Some(EncounterAction::Defend),
                "Defended in the RPG encounter",
            )
        } else if input.just_pressed(KeyCode::KeyI) {
            (
                Some(EncounterAction::UseItem),
                "Used a Field Tonic in the RPG encounter",
            )
        } else if input.just_pressed(KeyCode::Escape) {
            (
                Some(EncounterAction::Withdraw),
                "Withdrew from the RPG encounter",
            )
        } else {
            (None, "")
        };
        if let Some(action) = action {
            let result =
                flow.mutate_town(|save| save.act_in_signal_road_encounter(action).map(|_| ()));
            set_status(&mut flow, result, success);
        }
        return;
    }

    if input.just_pressed(KeyCode::Digit1) {
        let result = flow.mutate_town(|save| save.move_to(CampaignRoom::MirrorSquare));
        set_status(&mut flow, result, "Entered Mirror Square");
    } else if input.just_pressed(KeyCode::Digit2) {
        let result = flow.mutate_town(|save| save.move_to(CampaignRoom::MentorHall));
        set_status(&mut flow, result, "Entered the mentor hall");
    } else if input.just_pressed(KeyCode::Digit3) {
        let result = flow.mutate_town(|save| save.move_to(CampaignRoom::ExpeditionGate));
        set_status(&mut flow, result, "Entered the expedition gate");
    } else if input.just_pressed(KeyCode::Digit4) {
        let result = flow.mutate_town(|save| save.move_to(CampaignRoom::RelayQuarter));
        set_status(&mut flow, result, "Entered Relay Quarter");
    } else if input.just_pressed(KeyCode::KeyT) {
        if flow.save.room == CampaignRoom::RelayQuarter {
            let result = flow.mutate_town(CampaignSaveV1::talk_to_relay_smith);
            set_status(&mut flow, result, "Built trust with Relay Smith Brann");
        } else {
            let result = flow.mutate_town(CampaignSaveV1::talk_to_mentor);
            set_status(
                &mut flow,
                result,
                "Street Compass Sifu offered the First Contact task",
            );
        }
    } else if input.just_pressed(KeyCode::KeyK) {
        let result = flow.mutate_town(CampaignSaveV1::train_with_mentor);
        set_status(&mut flow, result, "Completed paid mentor training");
    } else if input.just_pressed(KeyCode::KeyL) {
        let result = flow.mutate_town(CampaignSaveV1::cycle_training_path);
        set_status(&mut flow, result, "Changed mentor training path");
    } else if input.just_pressed(KeyCode::KeyE) {
        let result = flow.mutate_town(CampaignSaveV1::cycle_loadout);
        set_status(&mut flow, result, "Changed typed equipment loadout");
    } else if input.just_pressed(KeyCode::KeyP) {
        let result = flow.mutate_town(CampaignSaveV1::cycle_party_preset);
        set_status(&mut flow, result, "Changed four-person persistent party");
    } else if input.just_pressed(KeyCode::KeyZ) {
        let result = flow.mutate_town(|save| save.cycle_party_member(1));
        set_status(&mut flow, result, "Changed companion slot one");
    } else if input.just_pressed(KeyCode::KeyX) {
        let result = flow.mutate_town(|save| save.cycle_party_member(2));
        set_status(&mut flow, result, "Changed companion slot two");
    } else if input.just_pressed(KeyCode::KeyC) {
        let result = flow.mutate_town(|save| save.cycle_party_member(3));
        set_status(&mut flow, result, "Changed companion slot three");
    } else if input.just_pressed(KeyCode::KeyY) {
        let result = flow.mutate_town(|save| save.spar_with_mentor().map(|_| ()));
        set_status(
            &mut flow,
            result,
            "Completed a deterministic mentor sparring bout",
        );
    } else if input.just_pressed(KeyCode::KeyU) {
        let result = flow.mutate_town(CampaignSaveV1::recruit_relay_smith);
        set_status(&mut flow, result, "Recruited Relay Smith Brann");
    } else if input.just_pressed(KeyCode::KeyH) {
        let result = flow.mutate_town(CampaignSaveV1::heal_party);
        set_status(
            &mut flow,
            result,
            "Treated one injury level across the roster",
        );
    } else if input.just_pressed(KeyCode::KeyG) {
        let result = flow.mutate_town(CampaignSaveV1::equip_relay_core);
        set_status(&mut flow, result, "Equipped the recovered Relay Core relic");
    } else if input.just_pressed(KeyCode::KeyA) {
        let result = flow.mutate_town(|save| save.cycle_growth_preview().map(|_| ()));
        set_status(
            &mut flow,
            result,
            "Previewed the next permanent growth allocation",
        );
    } else if input.just_pressed(KeyCode::KeyS) {
        let result = flow.mutate_town(|save| save.confirm_growth_allocation().map(|_| ()));
        set_status(&mut flow, result, "Confirmed and consumed one growth point");
    } else if input.just_pressed(KeyCode::KeyD) {
        let result = flow.mutate_town(CampaignSaveV1::cancel_growth_allocation);
        set_status(
            &mut flow,
            result,
            "Canceled the growth preview without spending",
        );
    } else if input.just_pressed(KeyCode::KeyV) {
        let result = flow.mutate_town(|save| save.cycle_active_title().map(|_| ()));
        set_status(&mut flow, result, "Changed the active build title");
    } else if input.just_pressed(KeyCode::KeyJ)
        && (flow.save.room == CampaignRoom::RelayQuarter
            || flow
                .save
                .progression
                .world_flags
                .contains("gate_warden_route"))
    {
        let result = flow.mutate_town(CampaignSaveV1::begin_signal_road_encounter);
        set_status(&mut flow, result, "Signal Road ambush began");
    } else if input.just_pressed(KeyCode::KeyF) {
        let repeatable_aftershock = flow.save.quest_state == QuestState::Completed
            && flow
                .save
                .progression
                .world_flags
                .contains("first_contact_secured");
        if matches!(
            flow.save.quest_state,
            QuestState::Available | QuestState::Failed | QuestState::Withdrawn
        ) || repeatable_aftershock
        {
            let result = flow.mutate_town(CampaignSaveV1::accept_first_contact_quest);
            set_status(
                &mut flow,
                result,
                "Accepted campaign mission; press F to deploy",
            );
        } else if flow.save.quest_state == QuestState::Accepted {
            *map = maps.for_mission(flow.save.active_mission).clone();
            match flow.start_battle(&map) {
                Ok(()) => {
                    runtime.reset_for_battle(&map);
                    adapter.accepted_orders.clear();
                }
                Err(error) => flow.status = error,
            }
        } else {
            flow.status = "The First Contact mission is not ready for deployment".to_string();
        }
    }
}

pub(super) fn settle_finished_battle(
    mut flow: ResMut<CampaignFlow>,
    mut runtime: ResMut<FirstContactRuntime>,
) {
    if !flow.in_battle() || !flow.mission.as_ref().is_some_and(MissionSimV1::terminal) {
        return;
    }
    match flow.complete_terminal_battle() {
        Ok(receipt) => {
            runtime.command = FirstContactCommand::Hold;
            runtime.command_feedback = format!(
                "Battle settled: {:?}, +{} XP",
                receipt.outcome, receipt.experience_delta
            );
        }
        Err(error) => flow.status = error,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkpoint_path_is_sibling_of_campaign_save() {
        assert_eq!(
            checkpoint_path_for(Path::new("/tmp/trnm/campaign.json")),
            PathBuf::from("/tmp/trnm/first-contact-battle.json")
        );
    }
}
