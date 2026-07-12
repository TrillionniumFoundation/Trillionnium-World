use super::map_loader::{FirstContactMap, MissionMapCatalog};
use super::simulation_adapter::{
    FirstContactCommand, FirstContactRuntime, FirstContactSimulationAdapter,
};
use bevy::prelude::*;
use serde_json::{json, Value};
use std::{
    env,
    path::{Path, PathBuf},
    time::Duration,
};
use trnm_campaign_core::{
    CampaignError, CampaignPhase, CampaignRoom, CampaignSaveV1, CampaignStore, EconomicIntent,
    EconomicIntentKind, EconomicReceipt, EconomyAccountBinding, EconomyBackend, EconomyMode,
    EncounterAction, InputMode, OfflineLocalEconomyBackend, PlayerSettings, PlayerSettingsStore,
    QuestBranch, QuestState, SaveSlotId, SaveSlotMeta, SaveSlotStore, SectId, SettlementReceiptV1,
    WalletSnapshot, SERVER_SIGNED_VALUE_ENTITLEMENT_METADATA_KEY,
};
use trnm_rpg_core::ECONOMY_ITEM_CATALOG;
use trnm_rts_sim::{BattleReplayV2, MissionSimV1, SimCheckpointStore};

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
    SkirmishSetup,
    Journal,
    ResumeGuard,
    Playing,
    Paused,
    ReplayBrowser,
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
    pub replay_cursor_tick: u64,
    pub replay_speed: u8,
    pub replay_paused: bool,
    pub replay_camera_x: i16,
    pub replay_camera_y: i16,
    pub settings: PlayerSettings,
    slot_store: SaveSlotStore,
    settings_store: PlayerSettingsStore,
    store: CampaignStore,
    checkpoint_store: SimCheckpointStore,
}

#[derive(Debug, Clone)]
struct CexHttpEconomyBackend {
    base_url: String,
    player_session: String,
    client: reqwest::blocking::Client,
}

impl CexHttpEconomyBackend {
    fn from_env() -> Result<Self, String> {
        let base_url =
            env::var("TRNM_CEX_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:8090".to_string());
        let client = reqwest::blocking::Client::builder()
            .connect_timeout(Duration::from_secs(2))
            .timeout(Duration::from_secs(5))
            .build()
            .map_err(|error| format!("CEX HTTP client: {error}"))?;
        Ok(Self {
            base_url,
            player_session: env::var("TRNM_CEX_PLAYER_SESSION")
                .ok()
                .filter(|value| !value.trim().is_empty())
                .ok_or_else(|| {
                    "TRNM_CEX_PLAYER_SESSION is required for connected player economy".to_string()
                })?,
            client,
        })
    }

    fn request(&self, path: &str) -> reqwest::blocking::RequestBuilder {
        let request = self
            .client
            .post(format!("{}{}", self.base_url.trim_end_matches('/'), path));
        request.header("x-trnm-player-session", &self.player_session)
    }
}

impl EconomyBackend for CexHttpEconomyBackend {
    fn backend_id(&self) -> &str {
        "cex-settlement-backend"
    }

    fn execute(&self, intent: &EconomicIntent) -> Result<EconomicReceipt, String> {
        let mut authorized_intent = intent.clone();
        if matches!(authorized_intent.kind, EconomicIntentKind::ReleaseReward)
            && authorized_intent.amount_credits.unwrap_or_default() > 0
        {
            let entitlement = env::var("TRNM_CEX_VALUE_ENTITLEMENTS_JSON")
                .ok()
                .and_then(|raw| raw.parse::<Value>().ok())
                .and_then(|value| value.get(&authorized_intent.intent_id).cloned())
                .or_else(|| {
                    env::var("TRNM_CEX_VALUE_ENTITLEMENT_JSON")
                        .ok()
                        .and_then(|raw| raw.parse::<Value>().ok())
                })
                .ok_or_else(|| {
                    "connected wallet reward is pending: trusted server entitlement is missing"
                        .to_string()
                })?;
            if !authorized_intent.metadata.is_object() {
                authorized_intent.metadata = json!({});
            }
            authorized_intent
                .metadata
                .as_object_mut()
                .expect("metadata was normalized to an object")
                .insert(
                    SERVER_SIGNED_VALUE_ENTITLEMENT_METADATA_KEY.to_string(),
                    entitlement,
                );
        }
        let response = self
            .request("/v1/trillionnium/economy/intents")
            .json(&serde_json::json!({"intent": authorized_intent}))
            .send()
            .map_err(|error| format!("CEX intent transport: {error}"))?;
        let status = response.status();
        let body = response
            .text()
            .map_err(|error| format!("CEX intent response: {error}"))?;
        if !status.is_success() {
            return Err(format!("CEX intent HTTP {status}: {body}"));
        }
        serde_json::from_str(&body).map_err(|error| format!("CEX receipt decode: {error}"))
    }

    fn wallet_snapshot(
        &self,
        binding: &EconomyAccountBinding,
        cursor: u64,
    ) -> Result<Option<WalletSnapshot>, String> {
        let response = self
            .request("/v1/trillionnium/economy/wallet")
            .json(&serde_json::json!({
                "actor_id": binding.actor_id,
                "account_id": binding.account_id,
                "reconciliation_cursor": cursor,
            }))
            .send()
            .map_err(|error| format!("CEX wallet transport: {error}"))?;
        let status = response.status();
        if !status.is_success() {
            return Err(format!("CEX wallet HTTP {status}"));
        }
        response
            .json::<WalletSnapshot>()
            .map(Some)
            .map_err(|error| format!("CEX wallet decode: {error}"))
    }
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
            replay_cursor_tick: 0,
            replay_speed: 1,
            replay_paused: true,
            replay_camera_x: 0,
            replay_camera_y: 0,
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

    pub fn open_selected_slot_skirmish(&mut self) -> Result<(), String> {
        let slot = self.selected_slot;
        let save = self
            .slot_store
            .load(slot)
            .map_err(|error| error.to_string())?;
        self.activate_slot(slot, save)?;
        self.mutate_town(CampaignSaveV1::prepare_standalone_skirmish)
            .map_err(|error| error.to_string())?;
        self.shell_mode = ShellMode::SkirmishSetup;
        self.status =
            "Standalone skirmish setup opened; campaign completion was not granted".to_string();
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

    pub fn toggle_subtitles_and_contrast(&mut self) -> Result<(), String> {
        self.settings.subtitles = !self.settings.subtitles;
        self.settings.high_contrast = self.settings.subtitles;
        self.settings_store
            .save_atomic(&self.settings)
            .map_err(|error| error.to_string())
    }

    pub fn cycle_control_scheme(&mut self) -> Result<(), String> {
        self.settings.control_scheme = self.settings.control_scheme.next();
        self.settings_store
            .save_atomic(&self.settings)
            .map_err(|error| error.to_string())
    }

    pub fn cycle_master_volume(&mut self) -> Result<(), String> {
        self.settings.master_volume_percent = match self.settings.master_volume_percent {
            0..=39 => 40,
            40..=79 => 80,
            80..=99 => 100,
            _ => 0,
        };
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

    fn bind_cex_from_env_if_present(save: &mut CampaignSaveV1) -> Result<(), String> {
        let Ok(account_id) = env::var("TRNM_CEX_ACCOUNT_ID") else {
            return Ok(());
        };
        if account_id.trim().is_empty() {
            return Ok(());
        }
        let actor_id =
            env::var("TRNM_CEX_ACTOR_ID").unwrap_or_else(|_| save.character.character_id.clone());
        save.bind_cex_economy_account(actor_id, account_id)
            .map_err(|error| error.to_string())?;
        Ok(())
    }

    pub fn reconcile_economy_now(&mut self) -> Result<String, String> {
        let mut candidate = self.save.clone();
        Self::bind_cex_from_env_if_present(&mut candidate)?;
        let report = if candidate.economy_mode == EconomyMode::CexConnected {
            let backend = CexHttpEconomyBackend::from_env()?;
            candidate
                .reconcile_economy(&backend, 16)
                .map_err(|error| error.to_string())?
        } else {
            candidate
                .reconcile_economy(&OfflineLocalEconomyBackend, 16)
                .map_err(|error| error.to_string())?
        };
        self.store
            .save_atomic(&candidate)
            .map_err(|error| error.to_string())?;
        self.save = candidate;
        Ok(format!(
            "ECON {:?}: applied {}, holds {}, hard-fail {}, pending {}{}",
            self.save.economy_mode,
            report.applied,
            report.recoverable_holds,
            report.hard_failures,
            report.remaining,
            report
                .last_error
                .as_ref()
                .map(|error| format!(" | {error}"))
                .unwrap_or_default(),
        ))
    }

    pub fn begin_tradeable_purchase_and_reconcile(&mut self) -> Result<String, String> {
        let mut candidate = self.save.clone();
        Self::bind_cex_from_env_if_present(&mut candidate)?;
        let market_account_id = env::var("TRNM_CEX_MARKET_ACCOUNT_ID")
            .ok()
            .filter(|value| !value.trim().is_empty());
        let purchase_id = candidate
            .begin_selected_tradeable_purchase_with_seller_account(market_account_id.as_deref())
            .map_err(|error| error.to_string())?;
        let report = if candidate.economy_mode == EconomyMode::CexConnected {
            let backend = CexHttpEconomyBackend::from_env()?;
            candidate
                .reconcile_economy(&backend, 8)
                .map_err(|error| error.to_string())?
        } else {
            candidate
                .reconcile_economy(&OfflineLocalEconomyBackend, 8)
                .map_err(|error| error.to_string())?
        };
        self.store
            .save_atomic(&candidate)
            .map_err(|error| error.to_string())?;
        self.save = candidate;
        Ok(format!(
            "TRADE {purchase_id}: applied {}, pending {}, wallet {}/{}",
            report.applied,
            report.remaining,
            self.save.wallet_snapshot.available_credits,
            self.save.wallet_snapshot.reserved_credits,
        ))
    }

    pub fn cancel_latest_tradeable_purchase_and_reconcile(&mut self) -> Result<String, String> {
        let mut candidate = self.save.clone();
        Self::bind_cex_from_env_if_present(&mut candidate)?;
        let purchase_id = candidate
            .pending_tradeable_purchases
            .last()
            .map(|purchase| purchase.purchase_id.clone())
            .ok_or_else(|| "no tradeable purchase exists to cancel".to_string())?;
        candidate
            .cancel_tradeable_purchase(&purchase_id)
            .map_err(|error| error.to_string())?;
        let report = if candidate.economy_mode == EconomyMode::CexConnected {
            let backend = CexHttpEconomyBackend::from_env()?;
            candidate
                .reconcile_economy(&backend, 8)
                .map_err(|error| error.to_string())?
        } else {
            candidate
                .reconcile_economy(&OfflineLocalEconomyBackend, 8)
                .map_err(|error| error.to_string())?
        };
        self.store
            .save_atomic(&candidate)
            .map_err(|error| error.to_string())?;
        self.save = candidate;
        Ok(format!(
            "CANCEL {purchase_id}: applied {}, holds {}, pending {}, wallet {}/{}",
            report.applied,
            report.recoverable_holds,
            report.remaining,
            self.save.wallet_snapshot.available_credits,
            self.save.wallet_snapshot.reserved_credits,
        ))
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
        self.shell_mode = ShellMode::Playing;
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
        mission
            .export_replay_v2()
            .and_then(|replay| replay.save_atomic(&self.replay_path()))
            .map_err(|error| error.to_string())?;
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

    fn replay_path(&self) -> PathBuf {
        self.store.path().with_file_name(format!(
            "slot-{}-last-replay-v2.json",
            self.active_slot.label()
        ))
    }

    fn open_replay_browser(&mut self) -> Result<(), String> {
        let replay = BattleReplayV2::load_verified(&self.replay_path())
            .map_err(|error| format!("No verified replay for this slot: {error}"))?;
        self.shell_mode = ShellMode::ReplayBrowser;
        self.replay_cursor_tick = 0;
        self.replay_speed = 1;
        self.replay_paused = true;
        self.replay_camera_x = replay.seed.map.party_start.x;
        self.replay_camera_y = replay.seed.map.party_start.y;
        self.status = format!(
            "REPLAY {} | {} chunks / {} orders | final tick {} | hash {}",
            replay.seed.map_id,
            replay.chunks.len(),
            replay.entry_count(),
            replay.final_tick,
            &replay.final_snapshot_hash[..12],
        );
        Ok(())
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
    if input.just_pressed(KeyCode::F5) {
        match flow.toggle_subtitles_and_contrast() {
            Ok(()) => {
                flow.status = format!(
                    "Subtitles: {} | high contrast: {}",
                    flow.settings.subtitles, flow.settings.high_contrast
                )
            }
            Err(error) => flow.status = error,
        }
        return;
    }
    if input.just_pressed(KeyCode::F7) {
        let control = input.pressed(KeyCode::ControlLeft) || input.pressed(KeyCode::ControlRight);
        let shift = input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight);
        let alt = input.pressed(KeyCode::AltLeft) || input.pressed(KeyCode::AltRight);
        let result = if control && alt {
            flow.cancel_latest_tradeable_purchase_and_reconcile()
        } else if control && shift {
            flow.begin_tradeable_purchase_and_reconcile()
        } else if control {
            flow.reconcile_economy_now()
        } else {
            flow.cycle_control_scheme()
                .map(|()| format!("Control scheme: {:?}", flow.settings.control_scheme))
                .map_err(|error| error.to_string())
        };
        flow.status = result.unwrap_or_else(|error| error);
        return;
    }
    if input.just_pressed(KeyCode::F8) {
        let control = input.pressed(KeyCode::ControlLeft) || input.pressed(KeyCode::ControlRight);
        let shift = input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight);
        if control && shift {
            flow.status = flow
                .cancel_latest_tradeable_purchase_and_reconcile()
                .unwrap_or_else(|error| error);
        } else {
            match flow.cycle_master_volume() {
                Ok(()) => {
                    flow.status = format!(
                        "Master volume preference: {}%",
                        flow.settings.master_volume_percent
                    )
                }
                Err(error) => flow.status = error,
            }
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
            } else if input.just_pressed(KeyCode::KeyK) {
                if let Err(error) = flow.open_selected_slot_skirmish() {
                    flow.status = error;
                }
            } else if input.just_pressed(KeyCode::KeyP) {
                if let Err(error) = flow.open_replay_browser() {
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
        ShellMode::SkirmishSetup => {
            if input.just_pressed(KeyCode::KeyM) {
                let result =
                    flow.mutate_town(|save| save.cycle_standalone_skirmish_map().map(|_| ()));
                set_status(&mut flow, result, "Changed standalone skirmish map");
            } else if input.just_pressed(KeyCode::KeyT) {
                let result = flow.mutate_town(|save| save.cycle_skirmish_faction().map(|_| ()));
                set_status(&mut flow, result, "Changed standalone faction matchup");
            } else if input.just_pressed(KeyCode::KeyY) {
                let result = flow.mutate_town(|save| save.cycle_skirmish_resources().map(|_| ()));
                set_status(&mut flow, result, "Changed standalone starting resources");
            } else if input.just_pressed(KeyCode::KeyU) {
                let result =
                    flow.mutate_town(|save| save.cycle_skirmish_victory_mode().map(|_| ()));
                set_status(&mut flow, result, "Changed standalone victory rule");
            } else if input.just_pressed(KeyCode::KeyI) {
                let result =
                    flow.mutate_town(|save| save.cycle_skirmish_simulation_seed().map(|_| ()));
                set_status(&mut flow, result, "Changed standalone deterministic seed");
            } else if input.just_pressed(KeyCode::Enter) {
                *map = maps.for_mission(flow.save.active_mission).clone();
                match flow.start_battle(&map) {
                    Ok(()) => {
                        runtime.reset_for_battle(&map);
                        adapter.accepted_orders.clear();
                        flow.shell_mode = ShellMode::Playing;
                    }
                    Err(error) => flow.status = error,
                }
            } else if input.just_pressed(KeyCode::Escape) {
                flow.shell_mode = ShellMode::Title;
                flow.status = "Standalone skirmish setup canceled".to_string();
            }
            return;
        }
        ShellMode::ReplayBrowser => {
            if input.just_pressed(KeyCode::KeyW) {
                flow.replay_camera_y = flow.replay_camera_y.saturating_sub(1);
            } else if input.just_pressed(KeyCode::KeyS) {
                flow.replay_camera_y = flow.replay_camera_y.saturating_add(1);
            } else if input.just_pressed(KeyCode::KeyA) {
                flow.replay_camera_x = flow.replay_camera_x.saturating_sub(1);
            } else if input.just_pressed(KeyCode::KeyD) {
                flow.replay_camera_x = flow.replay_camera_x.saturating_add(1);
            } else if input.just_pressed(KeyCode::Space) {
                flow.replay_paused = !flow.replay_paused;
            } else if input.just_pressed(KeyCode::ArrowUp) {
                flow.replay_speed = match flow.replay_speed {
                    1 => 2,
                    2 => 4,
                    4 => 8,
                    _ => 1,
                };
            } else if input.just_pressed(KeyCode::ArrowLeft) {
                flow.replay_cursor_tick = flow
                    .replay_cursor_tick
                    .saturating_sub(60 * u64::from(flow.replay_speed));
            } else if input.just_pressed(KeyCode::ArrowRight) {
                flow.replay_cursor_tick = flow
                    .replay_cursor_tick
                    .saturating_add(60 * u64::from(flow.replay_speed));
            } else if input.just_pressed(KeyCode::Enter) {
                match BattleReplayV2::load_verified(&flow.replay_path())
                    .and_then(|replay| replay.replay_and_verify())
                {
                    Ok(sim) => {
                        flow.status = format!(
                            "Replay verified through tick {} with snapshot {}",
                            sim.tick,
                            &sim.snapshot_hash().unwrap_or_default()[..12]
                        );
                    }
                    Err(error) => flow.status = error.to_string(),
                }
            } else if input.just_pressed(KeyCode::Escape) {
                flow.shell_mode = ShellMode::Title;
                flow.status = "Replay browser closed".to_string();
                return;
            }
            if !flow.replay_paused {
                flow.replay_cursor_tick = flow
                    .replay_cursor_tick
                    .saturating_add(u64::from(flow.replay_speed));
            }
            if input.just_pressed(KeyCode::Space)
                || input.just_pressed(KeyCode::ArrowUp)
                || input.just_pressed(KeyCode::ArrowLeft)
                || input.just_pressed(KeyCode::ArrowRight)
                || !flow.replay_paused
            {
                match BattleReplayV2::load_verified(&flow.replay_path()).and_then(|replay| {
                    flow.replay_cursor_tick = flow.replay_cursor_tick.min(replay.final_tick);
                    replay
                        .replay_until_tick(flow.replay_cursor_tick)
                        .map(|sim| (replay.final_tick, sim))
                }) {
                    Ok((final_tick, sim)) => {
                        flow.status = format!(
                            "Replay {} at tick {}/{} | {}x | {:?} | party {} enemy {} | resources {}/{}",
                            if flow.replay_paused { "paused" } else { "playing" },
                            sim.tick,
                            final_tick,
                            flow.replay_speed,
                            sim.phase,
                            sim.party.iter().filter(|unit| unit.hp > 0).count(),
                            sim.enemies.iter().filter(|unit| unit.hp > 0).count(),
                            sim.resources_available,
                            sim.enemy_resources_available,
                        );
                    }
                    Err(error) => flow.status = error.to_string(),
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
        } else if input.just_pressed(KeyCode::KeyK) {
            (
                Some(EncounterAction::PrimaryTechnique),
                "Released the selected primary sect technique",
            )
        } else if input.just_pressed(KeyCode::KeyL) {
            (
                Some(EncounterAction::SecondaryTechnique),
                "Released the selected secondary sect technique and checked its combo",
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

    let shift = input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight);
    let control = input.pressed(KeyCode::ControlLeft) || input.pressed(KeyCode::ControlRight);
    if shift && input.just_pressed(KeyCode::Digit1) {
        let result = flow.mutate_town(|save| save.move_to(CampaignRoom::GlassBasinWayhouse));
        set_status(&mut flow, result, "Entered Glass Basin Wayhouse");
    } else if shift && input.just_pressed(KeyCode::Digit2) {
        let result = flow.mutate_town(|save| save.move_to(CampaignRoom::DeepRelay));
        set_status(&mut flow, result, "Entered Deep Relay");
    } else if shift && input.just_pressed(KeyCode::Digit3) {
        let result = flow.mutate_town(|save| save.move_to(CampaignRoom::MoonBridge));
        set_status(&mut flow, result, "Entered Moon Bridge");
    } else if shift && input.just_pressed(KeyCode::Digit4) {
        let result = flow.mutate_town(|save| save.move_to(CampaignRoom::EmberOrchardEdge));
        set_status(&mut flow, result, "Entered Ember Orchard Edge");
    } else if shift && input.just_pressed(KeyCode::Digit5) {
        let result = flow.mutate_town(|save| save.move_to(CampaignRoom::GlassReedMarsh));
        set_status(&mut flow, result, "Entered Glass Reed Marsh");
    } else if shift && input.just_pressed(KeyCode::Digit6) {
        let result = flow.mutate_town(|save| save.move_to(CampaignRoom::BasinObservatory));
        set_status(&mut flow, result, "Entered Basin Observatory");
    } else if shift && input.just_pressed(KeyCode::Digit7) {
        let result = flow.mutate_town(|save| save.move_to(CampaignRoom::AshBeaconField));
        set_status(&mut flow, result, "Entered Ash Beacon Field");
    } else if shift && input.just_pressed(KeyCode::Digit8) {
        let result = flow.mutate_town(|save| save.move_to(CampaignRoom::CinderRefuge));
        set_status(&mut flow, result, "Entered Cinder Refuge");
    } else if input.just_pressed(KeyCode::Digit1) {
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
    } else if input.just_pressed(KeyCode::Digit5) {
        let result = flow.mutate_town(|save| save.move_to(CampaignRoom::CisternWard));
        set_status(&mut flow, result, "Entered Cistern Ward");
    } else if input.just_pressed(KeyCode::Digit6) {
        let result = flow.mutate_town(|save| save.move_to(CampaignRoom::NightWatchPost));
        set_status(&mut flow, result, "Entered the Night Watch Post");
    } else if input.just_pressed(KeyCode::Digit7) {
        let result = flow.mutate_town(|save| save.move_to(CampaignRoom::WorkshopGate));
        set_status(&mut flow, result, "Entered Iron Workshop Gate");
    } else if input.just_pressed(KeyCode::Digit8) {
        let result = flow.mutate_town(|save| save.move_to(CampaignRoom::MarketWindPavilion));
        set_status(&mut flow, result, "Entered Market Wind Pavilion");
    } else if input.just_pressed(KeyCode::Digit9) {
        let result = flow.mutate_town(|save| save.move_to(CampaignRoom::LanternInfirmary));
        set_status(&mut flow, result, "Entered Lantern Infirmary");
    } else if input.just_pressed(KeyCode::Digit0) {
        let result = flow.mutate_town(|save| save.move_to(CampaignRoom::ArchiveSteps));
        set_status(&mut flow, result, "Entered Archive Steps");
    } else if input.just_pressed(KeyCode::Minus) {
        let result = flow.mutate_town(|save| save.move_to(CampaignRoom::CaravanYard));
        set_status(&mut flow, result, "Entered Caravan Yard");
    } else if input.just_pressed(KeyCode::Equal) {
        let result = flow.mutate_town(|save| save.move_to(CampaignRoom::OuterSignalRoad));
        set_status(&mut flow, result, "Entered Outer Signal Road");
    } else if input.just_pressed(KeyCode::KeyT) {
        let shift = input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight);
        let control = input.pressed(KeyCode::ControlLeft) || input.pressed(KeyCode::ControlRight);
        if shift && flow.save.room != CampaignRoom::ExpeditionGate {
            let result = flow.mutate_town(|save| save.cycle_dialogue_choice().map(|_| ()));
            set_status(&mut flow, result, "Changed the next NPC dialogue choice");
            return;
        }
        if flow.save.room == CampaignRoom::ExpeditionGate && flow.save.skirmish_setup.enabled {
            let result = flow.mutate_town(|save| save.cycle_skirmish_faction().map(|_| ()));
            set_status(&mut flow, result, "Changed skirmish faction matchup");
            return;
        }
        if control {
            let sect = match flow.save.room {
                CampaignRoom::MentorHall => Some(SectId::StreetCompass),
                CampaignRoom::WorkshopGate => Some(SectId::IronWorkshop),
                CampaignRoom::NightWatchPost => Some(SectId::NightWatch),
                _ => None,
            };
            let result = sect
                .ok_or_else(|| {
                    CampaignError::InvalidState(
                        "Ctrl+T commits only inside one of the three sect halls".to_string(),
                    )
                })
                .and_then(|sect| flow.mutate_town(|save| save.join_regional_sect(sect)));
            set_status(&mut flow, result, "Committed to the regional sect");
        } else if flow.save.room == CampaignRoom::MentorHall && !flow.save.mentor_met {
            let result = flow.mutate_town(CampaignSaveV1::talk_to_mentor);
            set_status(
                &mut flow,
                result,
                "Street Compass Sifu offered the First Contact task",
            );
        } else if flow.save.has_current_regional_npc() {
            let result = flow.mutate_town(|save| save.talk_to_regional_npc().map(|_| ()));
            if result.is_ok() {
                flow.status = flow
                    .save
                    .last_npc_conversation
                    .as_ref()
                    .map(|record| format!("{}: {}", record.npc_id, record.line))
                    .unwrap_or_else(|| "Regional conversation completed".to_string());
            } else {
                set_status(&mut flow, result, "Regional conversation completed");
            }
        } else {
            flow.status = "No one here is available for conversation".to_string();
        }
    } else if control && shift && input.just_pressed(KeyCode::KeyK) {
        let result = flow.mutate_town(|save| save.cycle_secondary_equipped_technique().map(|_| ()));
        set_status(&mut flow, result, "Changed the secondary sect technique");
    } else if shift && input.just_pressed(KeyCode::KeyK) {
        let result = flow.mutate_town(|save| save.cycle_equipped_technique().map(|_| ()));
        set_status(&mut flow, result, "Changed the equipped sect technique");
    } else if input.just_pressed(KeyCode::KeyK) {
        let result = if flow.save.room == CampaignRoom::MentorHall {
            flow.mutate_town(CampaignSaveV1::train_with_mentor)
        } else {
            flow.mutate_town(|save| save.train_next_sect_skill().map(|_| ()))
        };
        set_status(&mut flow, result, "Completed paid sect training");
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
    } else if control && shift && input.just_pressed(KeyCode::KeyC) {
        let protect = !flow
            .save
            .visible_regional_caravan()
            .is_some_and(|caravan| caravan.guarded_by_player);
        let result =
            flow.mutate_town(|save| save.interact_with_visible_caravan(protect).map(|_| ()));
        set_status(
            &mut flow,
            result,
            if protect {
                "Escorted the visible regional caravan"
            } else {
                "Intercepted the visible regional caravan"
            },
        );
    } else if input.just_pressed(KeyCode::KeyC) {
        let result = flow.mutate_town(|save| save.cycle_party_member(3));
        set_status(&mut flow, result, "Changed companion slot three");
    } else if input.just_pressed(KeyCode::KeyY) {
        if flow.save.room == CampaignRoom::ExpeditionGate && flow.save.skirmish_setup.enabled {
            let result = flow.mutate_town(|save| save.cycle_skirmish_resources().map(|_| ()));
            set_status(&mut flow, result, "Changed skirmish starting resources");
        } else {
            let result = flow.mutate_town(|save| save.spar_with_mentor().map(|_| ()));
            set_status(
                &mut flow,
                result,
                "Completed a deterministic mentor sparring bout",
            );
        }
    } else if input.just_pressed(KeyCode::KeyU) {
        if flow.save.room == CampaignRoom::ExpeditionGate && flow.save.skirmish_setup.enabled {
            let result = flow.mutate_town(|save| save.cycle_skirmish_victory_mode().map(|_| ()));
            set_status(&mut flow, result, "Changed skirmish victory mode");
        } else {
            let result = flow.mutate_town(CampaignSaveV1::recruit_relay_smith);
            set_status(&mut flow, result, "Recruited Relay Smith Brann");
        }
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
    } else if control && shift && input.just_pressed(KeyCode::KeyB) {
        let result = if flow.save.pending_main_story_chapter.is_some() {
            flow.mutate_town(|save| save.advance_pending_main_story_scene().map(|_| ()))
        } else {
            flow.mutate_town(|save| save.advance_ending_epilogue().map(|_| ()))
        };
        set_status(&mut flow, result, "Advanced the authored story scene");
    } else if shift && input.just_pressed(KeyCode::KeyB) {
        let result = flow.mutate_town(|save| save.cycle_main_story_choice().map(|_| ()));
        set_status(
            &mut flow,
            result,
            "Changed the next main-story chapter resolution",
        );
    } else if input.just_pressed(KeyCode::KeyB) {
        let result = if flow.save.quest_chain.is_none() {
            flow.mutate_town(CampaignSaveV1::start_cistern_relief)
        } else {
            flow.mutate_town(|save| save.advance_cistern_relief().map(|_| ()))
        };
        set_status(&mut flow, result, "Advanced the Cistern Relief quest chain");
    } else if input.just_pressed(KeyCode::KeyN) {
        if flow.save.active_regional_quest_id.is_some() {
            let result = flow.mutate_town(|save| save.advance_toward_current_task().map(|_| ()));
            set_status(
                &mut flow,
                result,
                "Walked one legal edge toward the active quest",
            );
        } else {
            let result = flow.mutate_town(|save| {
                save.choose_cistern_relief_branch(QuestBranch::ReinforceCistern)
            });
            set_status(
                &mut flow,
                result,
                "Reinforced the cistern and earned Brann's trust",
            );
        }
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
    } else if input.just_pressed(KeyCode::KeyR) && flow.save.active_regional_quest_id.is_some() {
        let result = flow.mutate_town(|save| save.cycle_regional_quest_approach().map(|_| ()));
        set_status(
            &mut flow,
            result,
            "Changed the active quest resolution approach",
        );
    } else if input.just_pressed(KeyCode::F9) {
        let result = flow.mutate_town(|save| save.start_first_regional_quest_here().map(|_| ()));
        set_status(&mut flow, result, "Accepted a regional authored quest");
    } else if control && input.just_pressed(KeyCode::F10) {
        let result = flow.mutate_town(|save| {
            save.fail_active_regional_quest("the player abandoned the route")
                .map(|_| ())
        });
        set_status(
            &mut flow,
            result,
            "Abandoned the active quest; its retry remains available",
        );
    } else if input.just_pressed(KeyCode::F10) {
        let endgame = flow.save.room == CampaignRoom::ExpeditionGate
            && flow.save.active_regional_quest_id.is_none()
            && flow
                .save
                .progression
                .world_flags
                .contains("mirror_siege_secured");
        let result = if endgame {
            flow.mutate_town(|save| save.cycle_endgame_mission().map(|_| ()))
        } else {
            flow.mutate_town(CampaignSaveV1::advance_active_regional_quest)
        };
        set_status(
            &mut flow,
            result,
            if endgame {
                "Changed endgame skirmish"
            } else {
                "Advanced the active regional quest"
            },
        );
    } else if input.just_pressed(KeyCode::KeyW) {
        let result = flow.mutate_town(|save| save.wait_in_town(120));
        set_status(
            &mut flow,
            result,
            "Waited two hours; NPC schedules advanced",
        );
    } else if input.just_pressed(KeyCode::F11) {
        let shift = input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight);
        let control = input.pressed(KeyCode::ControlLeft) || input.pressed(KeyCode::ControlRight);
        let result = if flow.save.current_market_region_id().is_some() {
            if control {
                flow.mutate_town(|save| save.sell_selected_shop_item().map(|_| ()))
            } else if shift {
                flow.mutate_town(|save| save.buy_selected_shop_item().map(|_| ()))
            } else {
                flow.mutate_town(|save| save.cycle_shop_item().map(|_| ()))
            }
        } else if flow.save.room == CampaignRoom::WorkshopGate {
            if shift {
                flow.mutate_town(|save| save.craft_selected_recipe().map(|_| ()))
            } else {
                flow.mutate_town(|save| save.cycle_recipe().map(|_| ()))
            }
        } else {
            flow.mutate_town(|save| save.cycle_and_equip_owned_item().map(|_| ()))
        };
        set_status(
            &mut flow,
            result,
            if control {
                "Sold the selected catalog entry at today's demand price"
            } else if shift {
                "Purchased or crafted the selected catalog entry"
            } else {
                "Changed the shop, recipe or equipped-item selection"
            },
        );
    } else if input.just_pressed(KeyCode::F12) {
        let result = flow.mutate_town(|save| save.repair_all_equipment().map(|_| ()));
        set_status(&mut flow, result, "Repaired regional equipment durability");
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

fn native_economy_e2e_tap(app: &mut App, key: KeyCode, shift: bool, control: bool, alt: bool) {
    {
        let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        if shift {
            input.press(KeyCode::ShiftLeft);
        }
        if control {
            input.press(KeyCode::ControlLeft);
        }
        if alt {
            input.press(KeyCode::AltLeft);
        }
        input.press(key);
    }
    app.update();
    let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
    input.release(key);
    input.release(KeyCode::ShiftLeft);
    input.release(KeyCode::ControlLeft);
    input.release(KeyCode::AltLeft);
    input.clear();
}

pub(super) fn run_native_economy_e2e_phase(phase: &str) -> Result<Value, String> {
    let asset_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../assets");
    let maps = MissionMapCatalog::load(&asset_root)?;
    let map = maps.first_contact.clone();
    let flow = CampaignFlow::load()?;
    let mut app = App::new();
    app.insert_resource(ButtonInput::<KeyCode>::default())
        .insert_resource(map)
        .insert_resource(maps)
        .insert_resource(flow)
        .insert_resource(FirstContactRuntime::default())
        .insert_resource(FirstContactSimulationAdapter::default())
        .add_systems(Update, handle_campaign_input);

    if phase == "purchase" {
        {
            let mut flow = app.world_mut().resource_mut::<CampaignFlow>();
            if !flow.save.character_identity.confirmed {
                flow.create_selected_slot()?;
                flow.mutate_town(CampaignSaveV1::confirm_character_identity)
                    .map_err(|error| error.to_string())?;
            }
            flow.shell_mode = ShellMode::Playing;
            flow.mutate_town(|save| save.move_to(CampaignRoom::MarketWindPavilion))
                .map_err(|error| error.to_string())?;
            for _ in 0..ECONOMY_ITEM_CATALOG.len() {
                let item = flow.save.selected_shop_item();
                if CampaignSaveV1::economy_asset_semantic(item.id).transferability
                    == trnm_campaign_core::EconomyTransferability::Tradeable
                {
                    break;
                }
                flow.mutate_town(|save| save.cycle_shop_item().map(|_| ()))
                    .map_err(|error| error.to_string())?;
            }
        }
        native_economy_e2e_tap(&mut app, KeyCode::F7, false, true, false);
        native_economy_e2e_tap(&mut app, KeyCode::F7, true, true, false);
    } else {
        app.world_mut().resource_mut::<CampaignFlow>().shell_mode = ShellMode::Playing;
        native_economy_e2e_tap(&mut app, KeyCode::F7, false, true, false);
        if phase == "cancel" {
            native_economy_e2e_tap(&mut app, KeyCode::F8, true, true, false);
        }
    }

    let flow = app.world().resource::<CampaignFlow>();
    let purchase = flow.save.pending_tradeable_purchases.last();
    let item_id = purchase
        .map(|purchase| purchase.item_id.clone())
        .unwrap_or_default();
    let item_quantity = flow
        .save
        .progression
        .inventory
        .iter()
        .find(|stack| stack.item_id == item_id)
        .map(|stack| stack.quantity)
        .unwrap_or_default();
    let ui_text = super::campaign_ui::town_body(flow);
    Ok(json!({
        "phase": phase,
        "status": flow.status,
        "purchase_id": purchase.map(|purchase| purchase.purchase_id.clone()),
        "purchase_stage": purchase.map(|purchase| format!("{:?}", purchase.stage)),
        "item_id": item_id,
        "item_quantity": item_quantity,
        "wallet": flow.save.wallet_snapshot,
        "ui_text": ui_text,
        "native_bevy_input": true,
        "economic_actions_driven_by_ctrl_f7": true,
        "live_cex_http": flow.save.economy_mode == EconomyMode::CexConnected,
    }))
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
    use trnm_campaign_core::{
        BattleGridPoint, BattleOutcome, CampaignDifficulty, CampaignMission, MainStoryChapter,
        SkirmishVictoryMode, MAIN_STORY_CHAPTERS,
    };
    use trnm_rpg_core::{
        mirror_city_world_graph, quest_runtime_rule, QuestApproach, NPC_CATALOG,
        REGIONAL_QUEST_CATALOG,
    };
    use trnm_rts_protocol::{RtsFrameOrder, RtsOrderKind, RtsOrderSource, RtsTile};
    use trnm_rts_sim::{BattlePhase, FIVE_MINUTE_TICKS};

    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    unsafe extern "C" {
        fn malloc_trim(pad: usize) -> i32;
    }

    fn release_finished_client_test_heap() {
        #[cfg(all(target_os = "linux", target_env = "gnu"))]
        {
            // SAFETY: malloc_trim(0) is a process-local glibc allocator hint.
            // All Bevy App owners have been dropped before this helper runs.
            let _ = unsafe { malloc_trim(0) };
        }
    }

    fn tap_client_key(app: &mut App, key: KeyCode, shift: bool, control: bool) {
        {
            let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            if shift {
                input.press(KeyCode::ShiftLeft);
            }
            if control {
                input.press(KeyCode::ControlLeft);
            }
            input.press(key);
        }
        app.update();
        let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        input.release(key);
        input.release(KeyCode::ShiftLeft);
        input.release(KeyCode::ControlLeft);
        input.clear();
    }

    fn client_room_key(room: CampaignRoom) -> (KeyCode, bool) {
        match room {
            CampaignRoom::MirrorSquare => (KeyCode::Digit1, false),
            CampaignRoom::MentorHall => (KeyCode::Digit2, false),
            CampaignRoom::ExpeditionGate => (KeyCode::Digit3, false),
            CampaignRoom::RelayQuarter => (KeyCode::Digit4, false),
            CampaignRoom::CisternWard => (KeyCode::Digit5, false),
            CampaignRoom::NightWatchPost => (KeyCode::Digit6, false),
            CampaignRoom::WorkshopGate => (KeyCode::Digit7, false),
            CampaignRoom::MarketWindPavilion => (KeyCode::Digit8, false),
            CampaignRoom::LanternInfirmary => (KeyCode::Digit9, false),
            CampaignRoom::ArchiveSteps => (KeyCode::Digit0, false),
            CampaignRoom::CaravanYard => (KeyCode::Minus, false),
            CampaignRoom::OuterSignalRoad => (KeyCode::Equal, false),
            CampaignRoom::GlassBasinWayhouse => (KeyCode::Digit1, true),
            CampaignRoom::DeepRelay => (KeyCode::Digit2, true),
            CampaignRoom::MoonBridge => (KeyCode::Digit3, true),
            CampaignRoom::EmberOrchardEdge => (KeyCode::Digit4, true),
            CampaignRoom::GlassReedMarsh => (KeyCode::Digit5, true),
            CampaignRoom::BasinObservatory => (KeyCode::Digit6, true),
            CampaignRoom::AshBeaconField => (KeyCode::Digit7, true),
            CampaignRoom::CinderRefuge => (KeyCode::Digit8, true),
        }
    }

    fn walk_client_to(app: &mut App, destination: &str) {
        for _ in 0..64 {
            let flow = app.world().resource::<CampaignFlow>();
            if flow.save.room.id() == destination {
                return;
            }
            let route = mirror_city_world_graph().shortest_route(
                flow.save.room.id(),
                destination,
                &flow.save.progression.world_flags,
            );
            assert!(
                route.reachable(),
                "client route {} -> {destination} was blocked: {:?}",
                flow.save.room.id(),
                route.blocked_reason,
            );
            let next = CampaignRoom::from_id(route.path.get(1).unwrap()).unwrap();
            let (key, shift) = client_room_key(next);
            tap_client_key(app, key, shift, false);
        }
        panic!("client route did not reach {destination}");
    }

    fn journey_order(
        sim: &MissionSimV1,
        kind: RtsOrderKind,
        target: BattleGridPoint,
    ) -> RtsFrameOrder {
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
        order.target_tile = Some(RtsTile::new(i32::from(target.x), i32::from(target.y)));
        match kind {
            RtsOrderKind::Harvest => {
                order.target_actor_id = sim
                    .seed
                    .map
                    .resource_nodes
                    .first()
                    .map(|node| node.id.clone())
            }
            RtsOrderKind::Attack => order.target_actor_id = Some("relay_beacon".to_string()),
            RtsOrderKind::Repair => order.target_actor_id = Some("party_field_aid".to_string()),
            RtsOrderKind::Build => order.target_rule_id = Some("field_barricade".to_string()),
            RtsOrderKind::Ability => order.target_rule_id = Some("party_signature".to_string()),
            _ => {}
        }
        order
    }

    fn journey_step_until(
        sim: &mut MissionSimV1,
        predicate: impl Fn(&MissionSimV1) -> bool,
        limit: u64,
    ) {
        while !sim.terminal() && !predicate(sim) && sim.tick < limit {
            sim.step().unwrap();
        }
        assert!(
            predicate(sim),
            "{} journey condition missed by tick {} phase {:?} outcome {:?}",
            sim.seed.map_id,
            sim.tick,
            sim.phase,
            sim.outcome,
        );
    }

    fn drive_journey_battle_to_victory(app: &mut App) {
        let mission = app.world().resource::<CampaignFlow>().save.active_mission;
        {
            let mut flow = app.world_mut().resource_mut::<CampaignFlow>();
            let sim = flow.mission.as_mut().unwrap();
            if mission == CampaignMission::ConvoyExodus {
                let generator = sim.seed.map.approach_point;
                sim.issue_order(journey_order(sim, RtsOrderKind::Move, generator))
                    .unwrap();
                journey_step_until(sim, |sim| sim.objective_index >= 1, 1_500);
                sim.issue_order(journey_order(sim, RtsOrderKind::Hold, generator))
                    .unwrap();
                while !sim.terminal() && sim.objective_index < 2 && sim.tick < 3_000 {
                    sim.step().unwrap();
                }
                assert!(
                    sim.objective_index >= 2,
                    "convoy defense failed at tick {} with {:?}",
                    sim.tick,
                    sim.outcome
                );
                let extraction = sim.seed.map.objective;
                sim.issue_order(journey_order(sim, RtsOrderKind::Move, extraction))
                    .unwrap();
                journey_step_until(sim, MissionSimV1::terminal, FIVE_MINUTE_TICKS);
            } else {
                let approach = sim.seed.map.approach_point;
                sim.issue_order(journey_order(sim, RtsOrderKind::Move, approach))
                    .unwrap();
                journey_step_until(sim, |sim| sim.phase == BattlePhase::Contact, 1_200);
                let resource = sim.seed.map.resource_nodes[0].position;
                sim.issue_order(journey_order(sim, RtsOrderKind::Harvest, resource))
                    .unwrap();
                journey_step_until(sim, |sim| sim.resources_available >= 100, 1_600);
                let objective = sim.seed.map.objective;
                let _ = sim.issue_order(journey_order(sim, RtsOrderKind::Ability, objective));
                sim.step().unwrap();
                sim.issue_order(journey_order(sim, RtsOrderKind::Attack, objective))
                    .unwrap();
                journey_step_until(
                    sim,
                    |sim| sim.phase == BattlePhase::Relay && sim.relay_guard_hp <= 0,
                    3_200,
                );
                sim.issue_order(journey_order(sim, RtsOrderKind::Hold, objective))
                    .unwrap();
                for wave in 1..=2 {
                    journey_step_until(
                        sim,
                        |sim| sim.reinforcement_wave >= wave,
                        FIVE_MINUTE_TICKS,
                    );
                    let support_kind = if wave == 1 {
                        RtsOrderKind::Repair
                    } else {
                        RtsOrderKind::Build
                    };
                    sim.issue_order(journey_order(sim, support_kind, objective))
                        .unwrap();
                    let _ = sim.issue_order(journey_order(sim, RtsOrderKind::Ability, objective));
                    sim.issue_order(journey_order(sim, RtsOrderKind::Attack, objective))
                        .unwrap();
                    journey_step_until(
                        sim,
                        |sim| sim.enemies.iter().all(|enemy| !enemy.alive()),
                        FIVE_MINUTE_TICKS,
                    );
                    sim.issue_order(journey_order(sim, RtsOrderKind::Move, objective))
                        .unwrap();
                    journey_step_until(
                        sim,
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
                    sim.issue_order(journey_order(sim, RtsOrderKind::Hold, objective))
                        .unwrap();
                }
                while !sim.terminal() {
                    sim.step().unwrap();
                }
            }
            assert_eq!(
                sim.outcome,
                Some(BattleOutcome::Victory),
                "{} journey battle ended at tick {} with {:?}",
                mission.map_id(),
                sim.tick,
                sim.outcome,
            );
        }
        app.update();
        tap_client_key(app, KeyCode::Enter, false, false);
        assert_eq!(
            app.world().resource::<CampaignFlow>().mode,
            CampaignMode::Town,
            "{} did not settle through client debrief: {}",
            mission.map_id(),
            app.world().resource::<CampaignFlow>().status,
        );
    }

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
            replay_cursor_tick: 0,
            replay_speed: 1,
            replay_paused: true,
            replay_camera_x: 0,
            replay_camera_y: 0,
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

    #[test]
    fn title_slot_opens_independent_skirmish_setup_without_campaign_unlock_credit() {
        let root = std::env::temp_dir().join(format!(
            "trnm-standalone-skirmish-client-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        let slot_store = SaveSlotStore::new(&root);
        slot_store.create_new(SaveSlotId::A, false).unwrap();
        let save_path = slot_store.path(SaveSlotId::A);
        let mut flow = CampaignFlow {
            save: CampaignSaveV1::default(),
            mode: CampaignMode::Town,
            mission: None,
            last_receipt: None,
            status: String::new(),
            last_checkpoint_tick: 0,
            shell_mode: ShellMode::Title,
            active_slot: SaveSlotId::A,
            selected_slot: SaveSlotId::A,
            overwrite_pending: None,
            replay_cursor_tick: 0,
            replay_speed: 1,
            replay_paused: true,
            replay_camera_x: 0,
            replay_camera_y: 0,
            settings: PlayerSettings::default(),
            settings_store: PlayerSettingsStore::new(root.join("settings.json")),
            store: CampaignStore::new(&save_path),
            checkpoint_store: SimCheckpointStore::new(checkpoint_path_for(&save_path)),
            slot_store,
        };
        flow.open_selected_slot_skirmish().unwrap();
        assert_eq!(flow.shell_mode, ShellMode::SkirmishSetup);
        assert!(flow.save.skirmish_setup.enabled);
        assert!(flow
            .save
            .progression
            .world_flags
            .contains("standalone_skirmish_accessed"));
        assert!(!flow
            .save
            .progression
            .world_flags
            .contains("mirror_siege_secured"));
        for _ in 0..3 {
            flow.mutate_town(|save| save.cycle_standalone_skirmish_map().map(|_| ()))
                .unwrap();
        }
        flow.mutate_town(|save| save.cycle_skirmish_faction().map(|_| ()))
            .unwrap();
        flow.mutate_town(|save| save.cycle_skirmish_resources().map(|_| ()))
            .unwrap();
        flow.mutate_town(|save| save.cycle_skirmish_victory_mode().map(|_| ()))
            .unwrap();
        assert_eq!(
            flow.save.active_mission,
            trnm_campaign_core::CampaignMission::EmberOrchardSkirmish
        );
        assert_eq!(flow.save.skirmish_setup.starting_resources, 500);
        assert_eq!(
            flow.save.skirmish_setup.player_faction,
            trnm_campaign_core::CampaignFaction::AshenCompact
        );
        let maps =
            MissionMapCatalog::load(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../assets"))
                .unwrap();
        let authored = maps.for_mission(flow.save.active_mission).clone();
        flow.start_battle(&authored).unwrap();
        assert_eq!(flow.mode, CampaignMode::Battle);
        assert_eq!(flow.mission.as_ref().unwrap().seed.map_id, "ember_orchard");
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn complete_keyboard_skirmish_setup_chain_reaches_authored_deployment() {
        let root = std::env::temp_dir().join(format!(
            "trnm-keyboard-skirmish-chain-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        let slot_store = SaveSlotStore::new(&root);
        slot_store.create_new(SaveSlotId::A, false).unwrap();
        let save_path = slot_store.path(SaveSlotId::A);
        let maps =
            MissionMapCatalog::load(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../assets"))
                .unwrap();
        let map = maps.iron_delta.clone();
        let flow = CampaignFlow {
            save: CampaignSaveV1::default(),
            mode: CampaignMode::Town,
            mission: None,
            last_receipt: None,
            status: String::new(),
            last_checkpoint_tick: 0,
            shell_mode: ShellMode::Title,
            active_slot: SaveSlotId::A,
            selected_slot: SaveSlotId::A,
            overwrite_pending: None,
            replay_cursor_tick: 0,
            replay_speed: 1,
            replay_paused: true,
            replay_camera_x: 0,
            replay_camera_y: 0,
            settings: PlayerSettings::default(),
            settings_store: PlayerSettingsStore::new(root.join("settings.json")),
            store: CampaignStore::new(&save_path),
            checkpoint_store: SimCheckpointStore::new(checkpoint_path_for(&save_path)),
            slot_store,
        };
        let mut app = App::new();
        app.insert_resource(ButtonInput::<KeyCode>::default())
            .insert_resource(map)
            .insert_resource(maps)
            .insert_resource(flow)
            .insert_resource(FirstContactRuntime::default())
            .insert_resource(FirstContactSimulationAdapter::default())
            .add_systems(Update, handle_campaign_input);
        let tap = |app: &mut App, key: KeyCode| {
            app.world_mut()
                .resource_mut::<ButtonInput<KeyCode>>()
                .press(key);
            app.update();
            let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            input.release(key);
            input.clear();
        };
        for key in [
            KeyCode::KeyK,
            KeyCode::KeyM,
            KeyCode::KeyT,
            KeyCode::KeyY,
            KeyCode::KeyU,
            KeyCode::KeyI,
            KeyCode::Enter,
        ] {
            tap(&mut app, key);
        }
        let flow = app.world().resource::<CampaignFlow>();
        assert_eq!(flow.shell_mode, ShellMode::Playing);
        assert_eq!(flow.mode, CampaignMode::Battle);
        assert!(flow.save.skirmish_setup.enabled);
        assert_eq!(
            flow.save.active_mission,
            CampaignMission::NightWatchCrossingSkirmish
        );
        assert_eq!(flow.save.skirmish_setup.simulation_seed, 2);
        assert_eq!(
            flow.mission.as_ref().unwrap().seed.map_id,
            "night_watch_crossing"
        );
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn all_authored_quest_branches_use_client_navigation_failure_combat_and_scene_keys() {
        let prologue_root =
            std::env::temp_dir().join(format!("trnm-full-client-prologue-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&prologue_root);
        std::fs::create_dir_all(&prologue_root).unwrap();
        let prologue_slots = SaveSlotStore::new(&prologue_root);
        let prologue_save_path = prologue_slots.path(SaveSlotId::A);
        let prologue_maps =
            MissionMapCatalog::load(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../assets"))
                .unwrap();
        let prologue_flow = CampaignFlow {
            save: CampaignSaveV1::default(),
            mode: CampaignMode::Town,
            mission: None,
            last_receipt: None,
            status: String::new(),
            last_checkpoint_tick: 0,
            shell_mode: ShellMode::Playing,
            active_slot: SaveSlotId::A,
            selected_slot: SaveSlotId::A,
            overwrite_pending: None,
            replay_cursor_tick: 0,
            replay_speed: 1,
            replay_paused: true,
            replay_camera_x: 0,
            replay_camera_y: 0,
            settings: PlayerSettings::default(),
            settings_store: PlayerSettingsStore::new(prologue_root.join("settings.json")),
            store: CampaignStore::new(&prologue_save_path),
            checkpoint_store: SimCheckpointStore::new(checkpoint_path_for(&prologue_save_path)),
            slot_store: prologue_slots,
        };
        let mut prologue = App::new();
        prologue
            .insert_resource(ButtonInput::<KeyCode>::default())
            .insert_resource(prologue_maps.iron_delta.clone())
            .insert_resource(prologue_maps)
            .insert_resource(prologue_flow)
            .insert_resource(FirstContactRuntime::default())
            .insert_resource(FirstContactSimulationAdapter::default())
            .add_systems(
                Update,
                (handle_campaign_input, settle_finished_battle).chain(),
            );
        for (key, shift, control) in [
            (KeyCode::Digit2, false, false),
            (KeyCode::KeyT, false, false),
            (KeyCode::KeyK, false, false),
            (KeyCode::KeyK, false, false),
            (KeyCode::KeyK, false, false),
            (KeyCode::KeyE, false, false),
            (KeyCode::KeyP, false, false),
            (KeyCode::Digit3, false, false),
        ] {
            tap_client_key(&mut prologue, key, shift, control);
        }
        tap_client_key(&mut prologue, KeyCode::F6, false, false);
        tap_client_key(&mut prologue, KeyCode::F6, false, false);
        assert_eq!(
            prologue.world().resource::<CampaignFlow>().save.difficulty,
            CampaignDifficulty::Story
        );
        for mission_index in 0..4 {
            tap_client_key(&mut prologue, KeyCode::KeyF, false, false);
            tap_client_key(&mut prologue, KeyCode::KeyF, false, false);
            drive_journey_battle_to_victory(&mut prologue);
            for _ in 0..4 {
                tap_client_key(&mut prologue, KeyCode::KeyH, false, false);
            }
            tap_client_key(&mut prologue, KeyCode::KeyA, false, false);
            tap_client_key(&mut prologue, KeyCode::KeyS, false, false);
            if mission_index == 0 {
                tap_client_key(&mut prologue, KeyCode::KeyG, false, false);
            }
            if mission_index < 3 {
                walk_client_to(&mut prologue, "expedition_gate");
            }
        }
        let authored_rpg_start = prologue.world().resource::<CampaignFlow>().save.clone();
        assert!(authored_rpg_start
            .progression
            .world_flags
            .contains("mirror_siege_secured"));
        assert!(authored_rpg_start
            .progression
            .world_flags
            .contains("outer_signal_road_open"));
        drop(prologue);
        std::fs::remove_dir_all(&prologue_root).unwrap();
        release_finished_client_test_heap();

        for approach in [
            QuestApproach::Direct,
            QuestApproach::Diplomatic,
            QuestApproach::Resourceful,
        ] {
            let root = std::env::temp_dir().join(format!(
                "trnm-client-authored-quests-{}-{approach:?}",
                std::process::id()
            ));
            let _ = std::fs::remove_dir_all(&root);
            std::fs::create_dir_all(&root).unwrap();
            let slot_store = SaveSlotStore::new(&root);
            let save_path = slot_store.path(SaveSlotId::A);
            // Every branch forks from the save produced above by real client
            // setup/deployment keys and four authoritative campaign victories.
            // No room, attribute, trust, inventory, encounter, quest node or
            // prologue-unlock flag is written directly.
            let save = authored_rpg_start.clone();
            let maps = MissionMapCatalog::load(
                &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../assets"),
            )
            .unwrap();
            let flow = CampaignFlow {
                save,
                mode: CampaignMode::Town,
                mission: None,
                last_receipt: None,
                status: String::new(),
                last_checkpoint_tick: 0,
                shell_mode: ShellMode::Playing,
                active_slot: SaveSlotId::A,
                selected_slot: SaveSlotId::A,
                overwrite_pending: None,
                replay_cursor_tick: 0,
                replay_speed: 1,
                replay_paused: true,
                replay_camera_x: 0,
                replay_camera_y: 0,
                settings: PlayerSettings::default(),
                settings_store: PlayerSettingsStore::new(root.join("settings.json")),
                store: CampaignStore::new(&save_path),
                checkpoint_store: SimCheckpointStore::new(checkpoint_path_for(&save_path)),
                slot_store,
            };
            let mut app = App::new();
            app.insert_resource(ButtonInput::<KeyCode>::default())
                .insert_resource(maps.iron_delta.clone())
                .insert_resource(maps)
                .insert_resource(flow)
                .insert_resource(FirstContactRuntime::default())
                .insert_resource(FirstContactSimulationAdapter::default())
                .add_systems(Update, handle_campaign_input);

            for definition in REGIONAL_QUEST_CATALOG {
                let giver = NPC_CATALOG
                    .iter()
                    .find(|npc| npc.id == definition.giver_npc_id)
                    .unwrap();
                walk_client_to(&mut app, giver.room_id);
                let rule = quest_runtime_rule(definition.archetype);
                let required_trust = if approach == QuestApproach::Diplomatic {
                    rule.minimum_trust_for_diplomacy
                } else {
                    1
                };
                for _ in 0..96 {
                    let flow = app.world().resource::<CampaignFlow>();
                    let relationship = flow.save.npc_relationships.get(giver.id).unwrap();
                    if relationship.interactions > 0 && relationship.trust >= required_trust {
                        break;
                    }
                    let giver_is_present = flow
                        .save
                        .current_regional_npc()
                        .is_some_and(|npc| npc.id == giver.id);
                    tap_client_key(
                        &mut app,
                        if giver_is_present {
                            KeyCode::KeyT
                        } else {
                            KeyCode::KeyW
                        },
                        false,
                        false,
                    );
                }
                let flow = app.world().resource::<CampaignFlow>();
                let relationship = flow.save.npc_relationships.get(giver.id).unwrap();
                assert!(relationship.interactions > 0);
                assert!(relationship.trust >= required_trust);

                if approach == QuestApproach::Resourceful {
                    walk_client_to(&mut app, "market_wind_pavilion");
                    for _ in 0..64 {
                        if app
                            .world()
                            .resource::<CampaignFlow>()
                            .save
                            .selected_shop_item()
                            .id
                            == rule.resource_item_id
                        {
                            break;
                        }
                        tap_client_key(&mut app, KeyCode::F11, false, false);
                    }
                    assert_eq!(
                        app.world()
                            .resource::<CampaignFlow>()
                            .save
                            .selected_shop_item()
                            .id,
                        rule.resource_item_id
                    );
                    for _ in 0..rule.resource_quantity {
                        tap_client_key(&mut app, KeyCode::F11, true, false);
                    }
                    walk_client_to(&mut app, giver.room_id);
                }

                tap_client_key(&mut app, KeyCode::F9, false, false);
                assert_eq!(
                    app.world()
                        .resource::<CampaignFlow>()
                        .save
                        .active_regional_quest_id
                        .as_deref(),
                    Some(definition.id)
                );
                tap_client_key(&mut app, KeyCode::F10, false, true);
                assert_eq!(
                    app.world()
                        .resource::<CampaignFlow>()
                        .save
                        .regional_quest_states
                        .get(definition.id),
                    Some(&QuestState::Failed)
                );
                tap_client_key(&mut app, KeyCode::F9, false, false);
                while app
                    .world()
                    .resource::<CampaignFlow>()
                    .save
                    .active_regional_quest_runtime
                    .as_ref()
                    .is_some_and(|runtime| runtime.approach != approach)
                {
                    tap_client_key(&mut app, KeyCode::KeyR, false, false);
                }

                for _ in 0..512 {
                    let flow = app.world().resource::<CampaignFlow>();
                    if flow.save.active_regional_quest_id.is_none() {
                        break;
                    }
                    if let Some(encounter) = flow.save.active_encounter.as_ref() {
                        let key = if encounter.technique_cooldown == 0 && encounter.momentum >= 2 {
                            if encounter.round.is_multiple_of(2) {
                                KeyCode::KeyK
                            } else {
                                KeyCode::KeyL
                            }
                        } else if encounter.momentum < 2 {
                            KeyCode::KeyR
                        } else {
                            KeyCode::KeyJ
                        };
                        tap_client_key(&mut app, key, false, false);
                        continue;
                    }
                    let ready = flow.save.active_regional_quest_ready_rooms();
                    if ready.iter().any(|room| room == flow.save.room.id()) {
                        tap_client_key(&mut app, KeyCode::F10, false, false);
                        continue;
                    }
                    if ready.is_empty() {
                        let encounter_pending = approach == QuestApproach::Direct
                            && definition.encounter_id.is_some_and(|encounter_id| {
                                !flow
                                    .save
                                    .progression
                                    .world_flags
                                    .contains(&format!("{encounter_id}_cleared"))
                            });
                        if encounter_pending {
                            tap_client_key(&mut app, KeyCode::F10, false, false);
                            continue;
                        }
                    }
                    let route = flow.save.current_task_route_plan();
                    tap_client_key(
                        &mut app,
                        if route.path.len() > 1 {
                            KeyCode::KeyN
                        } else {
                            KeyCode::F10
                        },
                        false,
                        false,
                    );
                }
                let flow = app.world().resource::<CampaignFlow>();
                assert_eq!(
                    flow.save.regional_quest_states.get(definition.id),
                    Some(&QuestState::Completed),
                    "{} {approach:?} did not complete through the client keys: {}",
                    definition.id,
                    flow.status,
                );

                if let Some(pending) = flow.save.pending_main_story_chapter {
                    let chapter = MAIN_STORY_CHAPTERS
                        .iter()
                        .find(|chapter| chapter.chapter == pending)
                        .unwrap();
                    let room_id = chapter.room_id;
                    let _ = flow;
                    walk_client_to(&mut app, room_id);
                    for _ in 0..3 {
                        tap_client_key(&mut app, KeyCode::KeyB, true, true);
                    }
                }
            }
            let ending = app
                .world()
                .resource::<CampaignFlow>()
                .save
                .main_story_ending
                .expect("all three chapter scenes resolve an ending");
            let epilogue_room = match ending {
                trnm_campaign_core::MainStoryEnding::WayhouseLeague => "caravan_yard",
                trnm_campaign_core::MainStoryEnding::OpenArchiveRepublic => "archive_steps",
                trnm_campaign_core::MainStoryEnding::FrontierAccord => "ash_beacon_field",
                trnm_campaign_core::MainStoryEnding::ThreeRoadCompact => "mirror_square",
                trnm_campaign_core::MainStoryEnding::ContestedMandate => "moon_bridge",
            };
            walk_client_to(&mut app, epilogue_room);
            for _ in 0..4 {
                tap_client_key(&mut app, KeyCode::KeyB, true, true);
            }
            {
                let flow = app.world().resource::<CampaignFlow>();
                assert_eq!(
                    flow.save.main_story_chapter,
                    MainStoryChapter::ChapterComplete
                );
                assert!(flow.save.main_story_ending.is_some());
                assert!(flow.save.post_ending_world_state.is_some());
                assert!(flow.save.ending_epilogue_complete);
            }
            drop(app);
            std::fs::remove_dir_all(root).unwrap();
            release_finished_client_test_heap();
        }
    }

    #[test]
    fn authored_map_score_skirmish_wins_replays_and_settles_through_real_orders() {
        let root = std::env::temp_dir().join(format!(
            "trnm-authored-skirmish-victory-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        let slot_store = SaveSlotStore::new(&root);
        slot_store.create_new(SaveSlotId::A, false).unwrap();
        let save_path = slot_store.path(SaveSlotId::A);
        let mut flow = CampaignFlow {
            save: CampaignSaveV1::default(),
            mode: CampaignMode::Town,
            mission: None,
            last_receipt: None,
            status: String::new(),
            last_checkpoint_tick: 0,
            shell_mode: ShellMode::Title,
            active_slot: SaveSlotId::A,
            selected_slot: SaveSlotId::A,
            overwrite_pending: None,
            replay_cursor_tick: 0,
            replay_speed: 1,
            replay_paused: true,
            replay_camera_x: 0,
            replay_camera_y: 0,
            settings: PlayerSettings::default(),
            settings_store: PlayerSettingsStore::new(root.join("settings.json")),
            store: CampaignStore::new(&save_path),
            checkpoint_store: SimCheckpointStore::new(checkpoint_path_for(&save_path)),
            slot_store,
        };
        flow.open_selected_slot_skirmish().unwrap();
        flow.save.difficulty = CampaignDifficulty::Story;
        flow.save.skirmish_setup.starting_resources = 300;
        flow.save.skirmish_setup.score_target = 40;
        flow.save.skirmish_setup.victory_mode = SkirmishVictoryMode::Score;
        flow.save.active_mission = trnm_campaign_core::CampaignMission::GlassBasinSkirmish;
        flow.save.character.attributes.force = 50;
        flow.save.character.attributes.agility = 50;
        flow.save.character.attributes.insight = 50;
        for member in &mut flow.save.party {
            member.attributes.force = 50;
            member.attributes.agility = 50;
            member.attributes.insight = 50;
        }
        let maps =
            MissionMapCatalog::load(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../assets"))
                .unwrap();
        flow.start_battle(&maps.glass_basin).unwrap();
        let sim = flow.mission.as_mut().unwrap();
        while !sim.terminal() && sim.tick < 1_200 {
            if sim.tick.is_multiple_of(20) || sim.active_order.is_none() {
                let subjects = sim
                    .party
                    .iter()
                    .filter(|unit| unit.hp > 0)
                    .map(|unit| unit.unit_id.clone())
                    .collect::<Vec<_>>();
                let target = sim
                    .enemies
                    .iter()
                    .filter(|unit| unit.hp > 0)
                    .min_by_key(|unit| {
                        (unit.position.x - sim.seed.map.party_start.x).abs()
                            + (unit.position.y - sim.seed.map.party_start.y).abs()
                    })
                    .unwrap()
                    .position;
                let mut order = RtsFrameOrder::new(
                    sim.tick as u32,
                    "player",
                    subjects,
                    RtsOrderKind::AttackMove,
                    RtsOrderSource::LocalInput,
                );
                order.target_tile = Some(RtsTile::new(i32::from(target.x), i32::from(target.y)));
                sim.issue_order(order).unwrap();
            }
            sim.step().unwrap();
        }
        assert_eq!(
            sim.outcome,
            Some(BattleOutcome::Victory),
            "authored score match ended at tick {} with player score {} and enemy score {}",
            sim.tick,
            sim.player_score,
            sim.enemy_score
        );
        let replay = sim.export_replay().unwrap();
        replay.replay_and_verify().unwrap();
        let result = sim.clone().into_result().unwrap();
        let receipt = flow.save.submit_battle_result(result.clone()).unwrap();
        assert_eq!(receipt.outcome, BattleOutcome::Victory);
        assert!(!receipt.duplicate);
        assert!(flow.save.submit_battle_result(result).unwrap().duplicate);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn standard_annihilation_on_authored_map_destroys_the_real_base_and_settles() {
        let root = std::env::temp_dir().join(format!(
            "trnm-standard-annihilation-victory-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        let slot_store = SaveSlotStore::new(&root);
        slot_store.create_new(SaveSlotId::A, false).unwrap();
        let save_path = slot_store.path(SaveSlotId::A);
        let mut flow = CampaignFlow {
            save: CampaignSaveV1::default(),
            mode: CampaignMode::Town,
            mission: None,
            last_receipt: None,
            status: String::new(),
            last_checkpoint_tick: 0,
            shell_mode: ShellMode::Title,
            active_slot: SaveSlotId::A,
            selected_slot: SaveSlotId::A,
            overwrite_pending: None,
            replay_cursor_tick: 0,
            replay_speed: 1,
            replay_paused: true,
            replay_camera_x: 0,
            replay_camera_y: 0,
            settings: PlayerSettings::default(),
            settings_store: PlayerSettingsStore::new(root.join("settings.json")),
            store: CampaignStore::new(&save_path),
            checkpoint_store: SimCheckpointStore::new(checkpoint_path_for(&save_path)),
            slot_store,
        };
        flow.open_selected_slot_skirmish().unwrap();
        flow.save.difficulty = CampaignDifficulty::Standard;
        flow.save.skirmish_setup.starting_resources = 500;
        flow.save.skirmish_setup.victory_mode = SkirmishVictoryMode::Annihilation;
        flow.save.active_mission = CampaignMission::SaltMarshSkirmish;
        let maps =
            MissionMapCatalog::load(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../assets"))
                .unwrap();
        flow.start_battle(&maps.salt_marsh).unwrap();
        let sim = flow.mission.as_mut().unwrap();
        for _ in 0..3 {
            let mut recon = RtsFrameOrder::new(
                sim.tick as u32,
                "player",
                sim.party
                    .iter()
                    .map(|unit| unit.unit_id.clone())
                    .collect::<Vec<_>>(),
                RtsOrderKind::Recon,
                RtsOrderSource::LocalInput,
            );
            recon.target_tile = Some(RtsTile::new(
                i32::from(sim.seed.map.objective.x),
                i32::from(sim.seed.map.objective.y),
            ));
            sim.issue_order(recon).unwrap();
            sim.step().unwrap();
        }
        let supply_site = (-4_i16..=4)
            .flat_map(|dy| (-4_i16..=4).map(move |dx| (dx, dy)))
            .map(|(dx, dy)| {
                BattleGridPoint::new(
                    sim.seed.map.party_start.x + dx,
                    sim.seed.map.party_start.y + dy,
                )
            })
            .find(|target| {
                sim.seed.map.passable(*target)
                    && sim.party.iter().all(|unit| unit.position != *target)
                    && sim
                        .structures
                        .iter()
                        .all(|structure| structure.position != *target)
            })
            .unwrap();
        let mut supply = RtsFrameOrder::new(
            sim.tick as u32,
            "player",
            sim.party
                .iter()
                .map(|unit| unit.unit_id.clone())
                .collect::<Vec<_>>(),
            RtsOrderKind::Build,
            RtsOrderSource::LocalInput,
        );
        supply.target_rule_id = Some("supply_cache".to_string());
        supply.target_tile = Some(RtsTile::new(
            i32::from(supply_site.x),
            i32::from(supply_site.y),
        ));
        sim.issue_order(supply).unwrap();
        while !sim.jobs.is_empty() && !sim.terminal() && sim.tick < 200 {
            sim.step().unwrap();
        }
        assert!(
            sim.supply_cap() > 0,
            "shared construction must create real supply"
        );
        let mut assault_started = false;
        while !sim.terminal() && sim.tick < trnm_rts_sim::SKIRMISH_TIME_LIMIT_TICKS {
            let active_builder = sim
                .jobs
                .iter()
                .find(|job| job.kind == trnm_rts_sim::SimJobKind::BuildStructure)
                .and_then(|job| job.builder_id.as_deref());
            let subjects = sim
                .party
                .iter()
                .filter(|unit| unit.hp > 0 && Some(unit.unit_id.as_str()) != active_builder)
                .map(|unit| unit.unit_id.clone())
                .collect::<Vec<_>>();
            let mut economy_issued = false;
            if sim.tick.is_multiple_of(20) && !subjects.is_empty() {
                let workshop_exists = sim.structures.iter().any(|structure| {
                    structure.hp > 0
                        && structure.kind == trnm_rts_sim::SimStructureKind::FieldWorkshop
                });
                let workshop_queued = sim.jobs.iter().any(|job| {
                    job.kind == trnm_rts_sim::SimJobKind::BuildStructure
                        && job.rule_id == "field_workshop"
                });
                let badly_wounded = sim
                    .party
                    .iter()
                    .filter(|unit| unit.hp > 0 && unit.hp * 100 < unit.max_hp * 65)
                    .count();
                if sim.tick.is_multiple_of(300)
                    && badly_wounded >= 2
                    && sim.resources_available >= 20
                {
                    let mut aid = RtsFrameOrder::new(
                        sim.tick as u32,
                        "player",
                        subjects.clone(),
                        RtsOrderKind::Repair,
                        RtsOrderSource::LocalInput,
                    );
                    aid.target_actor_id = Some("party_field_aid".to_string());
                    economy_issued = sim.issue_order(aid).is_ok();
                } else if sim.party.iter().filter(|unit| unit.hp > 0).count() >= 8
                    && sim.researched_techs.contains("field_logistics")
                    && sim.recon_bonus_ticks == 0
                    && sim.tick.is_multiple_of(600)
                    && sim.resources_available >= 25
                {
                    let mut recon = RtsFrameOrder::new(
                        sim.tick as u32,
                        "player",
                        subjects.clone(),
                        RtsOrderKind::Recon,
                        RtsOrderSource::LocalInput,
                    );
                    recon.target_tile = Some(RtsTile::new(
                        i32::from(sim.seed.map.objective.x),
                        i32::from(sim.seed.map.objective.y),
                    ));
                    economy_issued = sim.issue_order(recon).is_ok();
                } else if assault_started
                    && sim.party.iter().filter(|unit| unit.hp > 0).count() >= 14
                {
                    // The full economy is online. Preserve resources for field
                    // aid/recon and let the command loop prosecute the base.
                } else if !workshop_exists && !workshop_queued && sim.resources_available >= 55 {
                    let target = (-4_i16..=4)
                        .flat_map(|dy| (-4_i16..=4).map(move |dx| (dx, dy)))
                        .map(|(dx, dy)| {
                            BattleGridPoint::new(
                                sim.seed.map.party_start.x + dx,
                                sim.seed.map.party_start.y + dy,
                            )
                        })
                        .find(|target| {
                            sim.seed.map.passable(*target)
                                && sim.party.iter().all(|unit| unit.position != *target)
                                && sim
                                    .structures
                                    .iter()
                                    .all(|structure| structure.position != *target)
                        })
                        .unwrap();
                    let mut build = RtsFrameOrder::new(
                        sim.tick as u32,
                        "player",
                        subjects.clone(),
                        RtsOrderKind::Build,
                        RtsOrderSource::LocalInput,
                    );
                    build.target_rule_id = Some("field_workshop".to_string());
                    build.target_tile =
                        Some(RtsTile::new(i32::from(target.x), i32::from(target.y)));
                    economy_issued = match sim.issue_order(build) {
                        Ok(()) => true,
                        Err(error) if sim.tick < 40 => {
                            panic!("normal-attribute economy failed to start: {error}")
                        }
                        Err(_) => false,
                    };
                } else if workshop_exists
                    && sim.jobs.is_empty()
                    && sim.supply_used().saturating_add(2) > sim.supply_cap()
                    && sim.resources_available >= 25
                {
                    let target = (-4_i16..=4)
                        .flat_map(|dy| (-4_i16..=4).map(move |dx| (dx, dy)))
                        .map(|(dx, dy)| {
                            BattleGridPoint::new(
                                sim.seed.map.party_start.x + dx,
                                sim.seed.map.party_start.y + dy,
                            )
                        })
                        .find(|target| {
                            sim.seed.map.passable(*target)
                                && sim.party.iter().all(|unit| unit.position != *target)
                                && sim
                                    .structures
                                    .iter()
                                    .all(|structure| structure.position != *target)
                        })
                        .unwrap();
                    let mut build = RtsFrameOrder::new(
                        sim.tick as u32,
                        "player",
                        subjects.clone(),
                        RtsOrderKind::Build,
                        RtsOrderSource::LocalInput,
                    );
                    build.target_rule_id = Some("supply_cache".to_string());
                    build.target_tile =
                        Some(RtsTile::new(i32::from(target.x), i32::from(target.y)));
                    economy_issued = match sim.issue_order(build) {
                        Ok(()) => true,
                        Err(error) if sim.tick < 80 => {
                            panic!("normal-attribute supply economy failed to start: {error}")
                        }
                        Err(_) => false,
                    };
                } else if workshop_exists
                    && sim.jobs.is_empty()
                    && sim.party.iter().filter(|unit| unit.hp > 0).count() < 12
                    && sim.resources_available >= 65
                {
                    let mut train = RtsFrameOrder::new(
                        sim.tick as u32,
                        "player",
                        subjects.clone(),
                        RtsOrderKind::Train,
                        RtsOrderSource::LocalInput,
                    );
                    train.target_rule_id = Some("mirror_warden".to_string());
                    train.queue_id = Some(format!("standard-train-{}", sim.tick));
                    economy_issued = match sim.issue_order(train) {
                        Ok(()) => true,
                        Err(error) if sim.tick < 500 => {
                            panic!("normal-attribute roster production failed: {error}")
                        }
                        Err(_) => false,
                    };
                } else if workshop_exists
                    && sim.jobs.is_empty()
                    && !sim.researched_techs.contains("field_logistics")
                {
                    let mut research = RtsFrameOrder::new(
                        sim.tick as u32,
                        "player",
                        subjects.clone(),
                        RtsOrderKind::Research,
                        RtsOrderSource::LocalInput,
                    );
                    research.target_rule_id = Some("field_logistics".to_string());
                    research.queue_id = Some(format!("standard-research-{}", sim.tick));
                    economy_issued = sim.issue_order(research).is_ok();
                } else if workshop_exists
                    && sim.jobs.is_empty()
                    && sim.supply_used().saturating_add(2) > sim.supply_cap()
                    && sim.resources_available >= 25
                {
                    let target = (-4_i16..=4)
                        .flat_map(|dy| (-4_i16..=4).map(move |dx| (dx, dy)))
                        .map(|(dx, dy)| {
                            BattleGridPoint::new(
                                sim.seed.map.party_start.x + dx,
                                sim.seed.map.party_start.y + dy,
                            )
                        })
                        .find(|target| {
                            sim.seed.map.passable(*target)
                                && sim.party.iter().all(|unit| unit.position != *target)
                                && sim
                                    .structures
                                    .iter()
                                    .all(|structure| structure.position != *target)
                        })
                        .unwrap();
                    let mut build = RtsFrameOrder::new(
                        sim.tick as u32,
                        "player",
                        subjects.clone(),
                        RtsOrderKind::Build,
                        RtsOrderSource::LocalInput,
                    );
                    build.target_rule_id = Some("supply_cache".to_string());
                    build.target_tile =
                        Some(RtsTile::new(i32::from(target.x), i32::from(target.y)));
                    economy_issued = sim.issue_order(build).is_ok();
                } else if workshop_exists
                    && sim.jobs.is_empty()
                    && sim.party.iter().filter(|unit| unit.hp > 0).count() < 14
                    && sim.resources_available >= 70
                {
                    let mut train = RtsFrameOrder::new(
                        sim.tick as u32,
                        "player",
                        subjects.clone(),
                        RtsOrderKind::Train,
                        RtsOrderSource::LocalInput,
                    );
                    train.target_rule_id = Some(
                        if sim.party.len().is_multiple_of(2) {
                            "field_medic"
                        } else {
                            "mirror_striker"
                        }
                        .to_string(),
                    );
                    train.queue_id = Some(format!("standard-train-{}", sim.tick));
                    economy_issued = sim.issue_order(train).is_ok();
                } else if sim.resources_available < 200 {
                    if let Some(node) = sim.resource_nodes.iter().find(|node| node.remaining > 0) {
                        let mut harvest = RtsFrameOrder::new(
                            sim.tick as u32,
                            "player",
                            subjects.clone(),
                            RtsOrderKind::Harvest,
                            RtsOrderSource::LocalInput,
                        );
                        harvest.target_actor_id = Some(node.node_id.clone());
                        harvest.target_tile = Some(RtsTile::new(
                            i32::from(node.position.x),
                            i32::from(node.position.y),
                        ));
                        economy_issued = sim.issue_order(harvest).is_ok();
                    }
                }
            }
            assault_started |= sim.party.iter().filter(|unit| unit.hp > 0).count() >= 12
                && sim.researched_techs.contains("field_logistics");
            let battle_ready = assault_started;
            let committed_harvest = !battle_ready
                && sim.current_order_kind() == RtsOrderKind::Harvest
                && sim.resources_available < 200;
            if !economy_issued
                && !subjects.is_empty()
                && !committed_harvest
                && (sim.tick.is_multiple_of(15) || sim.active_order.is_none())
            {
                let cleanup_target = sim.enemies.iter().all(|unit| unit.hp <= 0).then(|| {
                    sim.enemy_structures
                        .iter()
                        .filter(|structure| structure.hp > 0)
                        .min_by_key(|structure| {
                            (structure.position.x - sim.seed.map.party_start.x).abs()
                                + (structure.position.y - sim.seed.map.party_start.y).abs()
                        })
                        .map(|structure| (None, structure.position))
                });
                let strategic_target = battle_ready.then(|| {
                    sim.enemy_structures
                        .iter()
                        .filter(|structure| structure.hp > 0)
                        .min_by_key(|structure| {
                            let priority = match structure.kind {
                                trnm_rts_sim::SimStructureKind::CommandPost => 0,
                                trnm_rts_sim::SimStructureKind::FieldWorkshop => 1,
                                trnm_rts_sim::SimStructureKind::SupplyCache => 2,
                                _ => 3,
                            };
                            (priority, structure.hp)
                        })
                        .map(|structure| (None, structure.position))
                });
                let target = cleanup_target
                    .flatten()
                    .or_else(|| {
                        sim.enemies
                            .iter()
                            .filter(|unit| {
                                unit.hp > 0 && sim.visible_tiles.contains(&unit.position)
                            })
                            .min_by_key(|unit| {
                                (unit.position.x - sim.seed.map.party_start.x).abs()
                                    + (unit.position.y - sim.seed.map.party_start.y).abs()
                            })
                            .map(|unit| (Some(unit.unit_id.clone()), unit.position))
                    })
                    .or_else(|| strategic_target.flatten())
                    .or_else(|| {
                        sim.enemy_structures
                            .iter()
                            .find(|structure| {
                                structure.hp > 0 && sim.visible_tiles.contains(&structure.position)
                            })
                            .map(|structure| {
                                (Some(structure.structure_id.clone()), structure.position)
                            })
                    });
                if let Some(target) = target {
                    let mut order = RtsFrameOrder::new(
                        sim.tick as u32,
                        "player",
                        subjects,
                        if target.0.is_some() {
                            RtsOrderKind::Attack
                        } else {
                            RtsOrderKind::AttackMove
                        },
                        RtsOrderSource::LocalInput,
                    );
                    order.target_actor_id = target.0;
                    order.target_tile =
                        Some(RtsTile::new(i32::from(target.1.x), i32::from(target.1.y)));
                    sim.issue_order(order).unwrap();
                }
            }
            sim.step().unwrap();
        }
        assert_eq!(
            sim.outcome,
            Some(BattleOutcome::Victory),
            "tick {} party {}/{} enemies {}/{} structures {:?} jobs {:?} player_structures {:?} res {}/{} supply {}/{} tech {:?}",
            sim.tick,
            sim.party.iter().filter(|unit| unit.hp > 0).count(),
            sim.party.len(),
            sim.enemies.iter().filter(|unit| unit.hp > 0).count(),
            sim.enemies.len(),
            sim.enemy_structures
                .iter()
                .map(|structure| (&structure.structure_id, structure.hp))
                .collect::<Vec<_>>(),
            sim.jobs
            , sim.structures
                .iter()
                .map(|structure| (&structure.structure_id, structure.kind, structure.hp))
                .collect::<Vec<_>>()
            , sim.resources_available, sim.resources_gathered, sim.supply_used(), sim.supply_cap(), sim.researched_techs
        );
        assert!(sim
            .enemy_structures
            .iter()
            .all(|structure| structure.hp <= 0));
        let replay = sim.export_replay().unwrap();
        replay.replay_and_verify().unwrap();
        let _ = sim;
        let receipt = flow.complete_terminal_battle().unwrap();
        assert_eq!(receipt.outcome, BattleOutcome::Victory);
        assert!(!receipt.duplicate);
        assert_eq!(flow.mode, CampaignMode::Debrief);
        let mut app = App::new();
        app.insert_resource(ButtonInput::<KeyCode>::default())
            .insert_resource(maps.salt_marsh.clone())
            .insert_resource(maps)
            .insert_resource(flow)
            .insert_resource(FirstContactRuntime::default())
            .insert_resource(FirstContactSimulationAdapter::default())
            .add_systems(Update, handle_campaign_input);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Enter);
        app.update();
        assert_eq!(
            app.world().resource::<CampaignFlow>().mode,
            CampaignMode::Town,
            "the real client Enter path must return the settled victory to town",
        );
        std::fs::remove_dir_all(root).unwrap();
    }
}
