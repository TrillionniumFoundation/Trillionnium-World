use super::simulation_adapter::{
    FirstContactCommand, FirstContactRuntime, FirstContactSimulationAdapter,
};
use bevy::prelude::*;
use std::path::{Path, PathBuf};
use trnm_campaign_core::{
    CampaignError, CampaignPhase, CampaignRoom, CampaignSaveV1, CampaignStore, QuestState,
    SettlementReceiptV1,
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

    pub fn start_battle(&mut self) -> Result<(), String> {
        let mut candidate = self.save.clone();
        let seed = candidate
            .start_first_contact_battle()
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
        self.status = "BattleSeed accepted: First Contact deployed".to_string();
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

    if input.just_pressed(KeyCode::Digit1) {
        let result = flow.mutate_town(|save| save.move_to(CampaignRoom::MirrorSquare));
        set_status(&mut flow, result, "Entered Mirror Square");
    } else if input.just_pressed(KeyCode::Digit2) {
        let result = flow.mutate_town(|save| save.move_to(CampaignRoom::MentorHall));
        set_status(&mut flow, result, "Entered the mentor hall");
    } else if input.just_pressed(KeyCode::Digit3) {
        let result = flow.mutate_town(|save| save.move_to(CampaignRoom::ExpeditionGate));
        set_status(&mut flow, result, "Entered the expedition gate");
    } else if input.just_pressed(KeyCode::KeyT) {
        let result = flow.mutate_town(CampaignSaveV1::talk_to_mentor);
        set_status(
            &mut flow,
            result,
            "Street Compass Sifu offered the First Contact task",
        );
    } else if input.just_pressed(KeyCode::KeyK) {
        let result = flow.mutate_town(CampaignSaveV1::train_with_mentor);
        set_status(&mut flow, result, "Learned and practiced Basic Unarmed");
    } else if input.just_pressed(KeyCode::KeyE) {
        let result = flow.mutate_town(CampaignSaveV1::equip_starter_weapon);
        set_status(&mut flow, result, "Equipped Route Guard Staff");
    } else if input.just_pressed(KeyCode::KeyP) {
        let result = flow.mutate_town(|save| {
            save.select_party(vec![
                "hero".to_string(),
                "aya".to_string(),
                "mako".to_string(),
                "tess".to_string(),
            ])
        });
        set_status(
            &mut flow,
            result,
            "Selected hero plus three persistent companions",
        );
    } else if input.just_pressed(KeyCode::KeyF) {
        if matches!(
            flow.save.quest_state,
            QuestState::Available | QuestState::Failed | QuestState::Withdrawn
        ) {
            let result = flow.mutate_town(CampaignSaveV1::accept_first_contact_quest);
            set_status(
                &mut flow,
                result,
                "Accepted First Contact mission; press F to deploy",
            );
        } else if flow.save.quest_state == QuestState::Accepted {
            match flow.start_battle() {
                Ok(()) => {
                    runtime.reset_for_battle();
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
