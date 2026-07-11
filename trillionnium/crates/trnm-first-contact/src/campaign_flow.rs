use super::map_loader::{FirstContactMap, MissionMapCatalog};
use super::simulation_adapter::{
    FirstContactCommand, FirstContactRuntime, FirstContactSimulationAdapter,
};
use bevy::prelude::*;
use std::path::{Path, PathBuf};
use trnm_campaign_core::{
    CampaignError, CampaignPhase, CampaignRoom, CampaignSaveV1, CampaignStore, EncounterAction,
    InputMode, PlayerSettings, PlayerSettingsStore, QuestBranch, QuestState, SaveSlotId,
    SaveSlotMeta, SaveSlotStore, SettlementReceiptV1,
};
use trnm_rts_sim::{MissionSimV1, SimCheckpointStore};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CampaignMode {
    Town,
    Battle,
    Debrief,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ShellMode {
    Title,
    CharacterCreate,
    Journal,
    ResumeGuard,
    Playing,
    Paused,
}

#[derive(Resource, Debug, Clone)]
pub(super) struct CampaignFlow {
    pub save: CampaignSaveV1,
    pub mode: CampaignMode,
    pub mission: Option<MissionSimV1>,
    pub last_receipt: Option<SettlementReceiptV1>,
    pub status: String,
    pub last_checkpoint_tick: u64,
    pub shell_mode: ShellMode,
    pub active_slot: SaveSlotId,
    pub selected_slot: SaveSlotId,
    pub overwrite_pending: Option<SaveSlotId>,
    pub settings: PlayerSettings,
    slot_store: SaveSlotStore,
    settings_store: PlayerSettingsStore,
    store: CampaignStore,
    checkpoint_store: SimCheckpointStore,
}

impl CampaignFlow {
    pub fn load() -> Result<Self, String> {
        let save_path = campaign_save_path();
        let slot_root = save_path
            .parent()
            .ok_or_else(|| "campaign save path has no parent".to_string())?
            .to_path_buf();
        let slot_store = SaveSlotStore::new(&slot_root);
        let settings_store = PlayerSettingsStore::new(slot_root.join("player-settings.json"));
        let mut startup_warnings = Vec::new();
        let settings = settings_store.load_or_default().unwrap_or_else(|error| {
            startup_warnings.push(format!("Settings reset after validation failure: {error}"));
            PlayerSettings::default()
        });
        let store = CampaignStore::new(&save_path);
        let checkpoint_store = SimCheckpointStore::new(checkpoint_path_for(&save_path));
        let (mut save, slot_a_valid) = match store.load_or_default() {
            Ok(save) => (save, true),
            Err(error) => {
                startup_warnings.push(format!(
                    "Slot A is isolated after validation failure: {error}"
                ));
                (CampaignSaveV1::default(), false)
            }
        };
        let mut last_receipt = None;
        let mut status = if startup_warnings.is_empty() {
            "Campaign ready".to_string()
        } else {
            startup_warnings.join(" | ")
        };
        if slot_a_valid && save.phase == CampaignPhase::PostBattlePending {
            last_receipt = store
                .recover_pending_settlement(&mut save)
                .map_err(|error| error.to_string())?;
            status = "Recovered pending battle settlement after restart".to_string();
        }
        let mission = if slot_a_valid && save.phase == CampaignPhase::BattlePending {
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
            shell_mode: ShellMode::Title,
            active_slot: SaveSlotId::A,
            selected_slot: SaveSlotId::A,
            overwrite_pending: None,
            settings,
            slot_store,
            settings_store,
            store,
            checkpoint_store,
        })
    }

    pub fn in_battle(&self) -> bool {
        self.mode == CampaignMode::Battle
    }

    pub fn gameplay_running(&self) -> bool {
        self.shell_mode == ShellMode::Playing
    }

    pub fn keyboard_gameplay_enabled(&self) -> bool {
        self.gameplay_running() && self.settings.input_mode != InputMode::MouseOnly
    }

    pub fn mouse_gameplay_enabled(&self) -> bool {
        self.gameplay_running() && self.settings.input_mode != InputMode::KeyboardOnly
    }

    pub fn slot_metadata(&self) -> Vec<SaveSlotMeta> {
        self.slot_store.list()
    }

    fn activate_slot(&mut self, slot: SaveSlotId, save: CampaignSaveV1) -> Result<(), String> {
        let store = CampaignStore::new(self.slot_store.path(slot));
        let checkpoint_store = SimCheckpointStore::new(self.slot_store.checkpoint_path(slot));
        let mut save = save;
        let mut last_receipt = None;
        let mut status = format!("Loaded slot {}", slot.label());
        if save.phase == CampaignPhase::PostBattlePending {
            last_receipt = store
                .recover_pending_settlement(&mut save)
                .map_err(|error| error.to_string())?;
            status = format!("Recovered slot {} settlement", slot.label());
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
                    status = format!(
                        "Slot {} checkpoint restored at tick {}",
                        slot.label(),
                        sim.tick
                    );
                    Some(sim)
                }
                None => {
                    Some(MissionSimV1::from_seed(seed.clone()).map_err(|error| error.to_string())?)
                }
            }
        } else {
            None
        };
        self.mode = if mission.is_some() {
            CampaignMode::Battle
        } else if last_receipt.is_some() {
            CampaignMode::Debrief
        } else {
            CampaignMode::Town
        };
        self.active_slot = slot;
        self.selected_slot = slot;
        self.store = store;
        self.checkpoint_store = checkpoint_store;
        self.save = save;
        self.mission = mission;
        self.last_receipt = last_receipt;
        self.last_checkpoint_tick = 0;
        self.status = status;
        self.overwrite_pending = None;
        Ok(())
    }

    pub fn load_selected_slot(&mut self) -> Result<(), String> {
        let slot = self.selected_slot;
        let save = self
            .slot_store
            .load(slot)
            .map_err(|error| error.to_string())?;
        self.activate_slot(slot, save)?;
        self.shell_mode = if self.save.character_identity.confirmed {
            ShellMode::ResumeGuard
        } else {
            ShellMode::CharacterCreate
        };
        Ok(())
    }

    pub fn create_selected_slot(&mut self) -> Result<(), String> {
        let slot = self.selected_slot;
        let exists = self.slot_store.metadata(slot).exists;
        if exists && self.overwrite_pending != Some(slot) {
            self.overwrite_pending = Some(slot);
            return Err(format!(
                "Slot {} exists. Press N again to confirm overwrite.",
                slot.label()
            ));
        }
        let save = self
            .slot_store
            .create_new(slot, exists)
            .map_err(|error| error.to_string())?;
        self.activate_slot(slot, save)?;
        self.shell_mode = ShellMode::CharacterCreate;
        self.status = format!(
            "New campaign created in slot {}; confirm a persistent identity",
            slot.label()
        );
        Ok(())
    }

    pub fn toggle_low_motion(&mut self) -> Result<(), String> {
        self.settings.low_motion = !self.settings.low_motion;
        self.settings_store
            .save_atomic(&self.settings)
            .map_err(|error| error.to_string())
    }

    pub fn cycle_input_mode(&mut self) -> Result<(), String> {
        self.settings.input_mode = self.settings.input_mode.next();
        self.settings_store
            .save_atomic(&self.settings)
            .map_err(|error| error.to_string())
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
    if input.just_pressed(KeyCode::F1) {
        flow.shell_mode = ShellMode::Title;
        flow.status = "Title menu opened; active state remains atomically saved".to_string();
        return;
    }
    if input.just_pressed(KeyCode::F4) {
        if flow.shell_mode == ShellMode::Playing && !flow.in_battle() {
            flow.shell_mode = ShellMode::Journal;
            flow.status = "Campaign journal opened from authoritative state".to_string();
        } else if flow.shell_mode == ShellMode::Journal {
            flow.shell_mode = ShellMode::Playing;
            flow.status = "Campaign journal closed".to_string();
        }
        return;
    }
    match flow.shell_mode {
        ShellMode::Title => {
            for (key, slot) in [
                (KeyCode::Digit1, SaveSlotId::A),
                (KeyCode::Digit2, SaveSlotId::B),
                (KeyCode::Digit3, SaveSlotId::C),
            ] {
                if input.just_pressed(key) {
                    flow.selected_slot = slot;
                    flow.overwrite_pending = None;
                    flow.status = format!("Selected save slot {}", slot.label());
                }
            }
            if input.just_pressed(KeyCode::KeyN) {
                if let Err(error) = flow.create_selected_slot() {
                    flow.status = error;
                }
            } else if input.just_pressed(KeyCode::Enter) {
                if let Err(error) = flow.load_selected_slot() {
                    flow.status = error;
                }
            } else if input.just_pressed(KeyCode::F2) {
                match flow.toggle_low_motion() {
                    Ok(()) => flow.status = format!("Low motion: {}", flow.settings.low_motion),
                    Err(error) => flow.status = error,
                }
            } else if input.just_pressed(KeyCode::F3) {
                match flow.cycle_input_mode() {
                    Ok(()) => flow.status = format!("Input mode: {:?}", flow.settings.input_mode),
                    Err(error) => flow.status = error,
                }
            }
            return;
        }
        ShellMode::CharacterCreate => {
            if input.just_pressed(KeyCode::KeyC) {
                let result = flow.mutate_town(|save| save.cycle_character_identity().map(|_| ()));
                set_status(&mut flow, result, "Changed the persistent character name");
            } else if input.just_pressed(KeyCode::Enter) {
                let result = flow.mutate_town(CampaignSaveV1::confirm_character_identity);
                match result {
                    Ok(()) => {
                        flow.shell_mode = ShellMode::Playing;
                        flow.status = format!(
                            "Character identity confirmed: {}",
                            flow.save.character.display_name
                        );
                    }
                    Err(error) => flow.status = error.to_string(),
                }
            } else if input.just_pressed(KeyCode::Escape) {
                flow.shell_mode = ShellMode::Title;
                flow.status = "Character creation canceled; slot remains unconfirmed".to_string();
            }
            return;
        }
        ShellMode::Journal => {
            if input.just_pressed(KeyCode::Escape) {
                flow.shell_mode = ShellMode::Playing;
                flow.status = "Campaign journal closed".to_string();
            }
            return;
        }
        ShellMode::ResumeGuard => {
            if input.just_pressed(KeyCode::Enter) {
                flow.shell_mode = ShellMode::Playing;
                flow.status = format!("Slot {} resumed", flow.active_slot.label());
            }
            return;
        }
        ShellMode::Paused => {
            if input.just_pressed(KeyCode::Escape) {
                flow.shell_mode = ShellMode::Playing;
                flow.status = "Simulation resumed".to_string();
            } else if input.just_pressed(KeyCode::F2) {
                match flow.toggle_low_motion() {
                    Ok(()) => flow.status = format!("Low motion: {}", flow.settings.low_motion),
                    Err(error) => flow.status = error,
                }
            } else if input.just_pressed(KeyCode::F3) {
                match flow.cycle_input_mode() {
                    Ok(()) => flow.status = format!("Input mode: {:?}", flow.settings.input_mode),
                    Err(error) => flow.status = error,
                }
            }
            return;
        }
        ShellMode::Playing => {}
    }
    if input.just_pressed(KeyCode::Escape)
        && (flow.in_battle() || flow.save.active_encounter.is_none())
    {
        flow.shell_mode = ShellMode::Paused;
        flow.status = "Paused: authoritative RTS ticks and gameplay input are stopped".to_string();
        return;
    }
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
    } else if input.just_pressed(KeyCode::KeyO) {
        let result = flow.mutate_town(|save| save.cycle_character_origin().map(|_| ()));
        set_status(
            &mut flow,
            result,
            "Changed character origin before mentor commitment",
        );
    } else if input.just_pressed(KeyCode::KeyQ) {
        let result = flow.mutate_town(|save| save.attempt_mastery_challenge().map(|_| ()));
        set_status(
            &mut flow,
            result,
            "Completed the selected path mastery challenge",
        );
    } else if input.just_pressed(KeyCode::KeyB) {
        let result = if flow.save.quest_chain.is_none() {
            flow.mutate_town(CampaignSaveV1::start_cistern_relief)
        } else {
            flow.mutate_town(|save| save.advance_cistern_relief().map(|_| ()))
        };
        set_status(&mut flow, result, "Advanced the Cistern Relief quest chain");
    } else if input.just_pressed(KeyCode::KeyN) {
        let result = flow
            .mutate_town(|save| save.choose_cistern_relief_branch(QuestBranch::ReinforceCistern));
        set_status(
            &mut flow,
            result,
            "Reinforced the cistern and earned Brann's trust",
        );
    } else if input.just_pressed(KeyCode::KeyM) {
        let result = flow
            .mutate_town(|save| save.choose_cistern_relief_branch(QuestBranch::EvacuateFamilies));
        set_status(
            &mut flow,
            result,
            "Evacuated the families and secured relief credits",
        );
    } else if input.just_pressed(KeyCode::KeyR) && flow.save.room == CampaignRoom::ExpeditionGate {
        let result = flow.mutate_town(|save| save.cycle_expedition_preparation().map(|_| ()));
        set_status(&mut flow, result, "Changed expedition preparation");
    } else if input.just_pressed(KeyCode::F6) && flow.save.room == CampaignRoom::ExpeditionGate {
        let result = flow.mutate_town(|save| save.cycle_difficulty().map(|_| ()));
        set_status(&mut flow, result, "Changed campaign difficulty");
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
        let repeatable_campaign = flow.save.quest_state == QuestState::Completed
            && flow
                .save
                .progression
                .world_flags
                .contains("first_contact_secured");
        if matches!(
            flow.save.quest_state,
            QuestState::Available | QuestState::Failed | QuestState::Withdrawn
        ) || repeatable_campaign
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

    #[test]
    fn shell_modes_gate_authoritative_gameplay_and_input_modes() {
        let save_path = PathBuf::from("/tmp/trnm-shell-gate-campaign.json");
        let slot_store = SaveSlotStore::new("/tmp/trnm-shell-gate");
        let mut flow = CampaignFlow {
            save: CampaignSaveV1::default(),
            mode: CampaignMode::Battle,
            mission: None,
            last_receipt: None,
            status: String::new(),
            last_checkpoint_tick: 0,
            shell_mode: ShellMode::Paused,
            active_slot: SaveSlotId::A,
            selected_slot: SaveSlotId::A,
            overwrite_pending: None,
            settings: PlayerSettings::default(),
            slot_store,
            settings_store: PlayerSettingsStore::new("/tmp/trnm-shell-gate-settings.json"),
            store: CampaignStore::new(&save_path),
            checkpoint_store: SimCheckpointStore::new(checkpoint_path_for(&save_path)),
        };
        assert!(!flow.gameplay_running());
        assert!(!flow.keyboard_gameplay_enabled());
        assert!(!flow.mouse_gameplay_enabled());
        flow.shell_mode = ShellMode::Playing;
        assert!(flow.keyboard_gameplay_enabled());
        assert!(flow.mouse_gameplay_enabled());
        flow.settings.input_mode = InputMode::KeyboardOnly;
        assert!(flow.keyboard_gameplay_enabled());
        assert!(!flow.mouse_gameplay_enabled());
        flow.settings.input_mode = InputMode::MouseOnly;
        assert!(!flow.keyboard_gameplay_enabled());
        assert!(flow.mouse_gameplay_enabled());
    }
}
