mod asset_loader;
mod audio;
mod campaign_flow;
mod campaign_ui;
mod evidence_adapter;
mod frame_timing;
mod hud;
mod map_loader;
mod online_authority;
mod online_command_journal;
mod renderer;
mod simulation_adapter;
mod ui;
mod view_math;

use asset_loader::{load_first_contact_atlas, FirstContactAtlasManifest};
use audio::{spawn_trnm_audio, sync_trnm_audio, validate_trnm_audio_assets};
use bevy::prelude::*;
use campaign_flow::{
    handle_campaign_input, handle_campaign_ui_intents, pump_campaign_economy_tasks,
    settle_finished_battle, CampaignEconomyTasks, CampaignFlow, CampaignMode, CampaignUiIntents,
    ShellMode,
};
use campaign_ui::{collect_campaign_ui_intents, spawn_campaign_ui, update_campaign_ui};
use evidence_adapter::FirstContactVisualAcceptance;
use frame_timing::{begin_online_frame_timing, record_online_frame_timing, OnlineFrameTiming};
use hud::{
    handle_first_contact_command_card_interactions, spawn_first_contact_hud,
    update_first_contact_hud,
};
use map_loader::{FirstContactMap, MissionMapCatalog};
use online_authority::OnlineAuthorityClient;
use renderer::{
    animate_identity_geometry, prepare_first_contact_live_scene, spawn_first_contact_authored_map,
    sync_first_contact_authored_map,
};
use simulation_adapter::{
    advance_first_contact_simulation, expire_first_contact_feedback, handle_first_contact_commands,
    handle_first_contact_mouse_selection, pan_first_contact_camera, FirstContactCommandIntents,
    FirstContactRuntime, FirstContactSimulationAdapter, MouseSelectionState,
};
use std::path::{Path, PathBuf};
use ui::{
    handle_world_ui_input, handle_world_ui_interactions, spawn_world_ui, sync_world_ui,
    WorldUiState,
};

pub use evidence_adapter::{FirstContactVisualAcceptance as VisualAcceptance, ObserverAnswer};

pub fn run_native_economy_e2e_phase(phase: &str) -> Result<serde_json::Value, String> {
    campaign_flow::run_native_economy_e2e_phase(phase)
}

pub struct FirstContactLivePlugin {
    map: FirstContactMap,
    maps: MissionMapCatalog,
    atlas: FirstContactAtlasManifest,
    campaign: CampaignFlow,
    online: Option<OnlineAuthorityClient>,
}

impl FirstContactLivePlugin {
    pub fn load(asset_root: &Path) -> Result<Self, String> {
        validate_trnm_audio_assets(asset_root)?;
        let maps = MissionMapCatalog::load(asset_root)?;
        let atlas = load_first_contact_atlas(&asset_root.join("first_contact/atlas.yaml"))?;
        let mut campaign = CampaignFlow::load()?;
        let online = OnlineAuthorityClient::from_env()?;
        if let Some((_, mission)) = online.as_ref() {
            campaign.mission = Some(mission.clone());
            campaign.mode = CampaignMode::Battle;
            campaign.shell_mode = ShellMode::Playing;
            campaign.status =
                "ONLINE AUTHORITY: server snapshot attached; local simulation disabled".to_string();
        }
        let active_mission = campaign
            .mission
            .as_ref()
            .map(|mission| mission.seed.map_id.as_str())
            .unwrap_or(campaign.save.active_mission.map_id());
        let map = match active_mission {
            "aftershock_patrol" | "first_contact_aftershock" => maps.aftershock_patrol.clone(),
            "convoy_exodus" => maps.convoy_exodus.clone(),
            "mirror_siege" => maps.mirror_siege.clone(),
            "iron_delta" => maps.iron_delta.clone(),
            "night_watch_crossing" => maps.night_watch_crossing.clone(),
            "glass_basin" => maps.glass_basin.clone(),
            "ember_orchard" => maps.ember_orchard.clone(),
            "salt_marsh" => maps.salt_marsh.clone(),
            "cinder_crown" => maps.cinder_crown.clone(),
            _ => maps.first_contact.clone(),
        };
        Ok(Self {
            map,
            maps,
            atlas,
            campaign,
            online: online.map(|(client, _)| client),
        })
    }
}

impl Plugin for FirstContactLivePlugin {
    fn build(&self, app: &mut App) {
        let mut runtime = FirstContactRuntime::default();
        if self.online.is_some() {
            runtime.reset_for_battle(&self.map);
            runtime.command_feedback =
                "ONLINE ATTACHED: select assigned units and issue a server command".to_string();
        }
        app.insert_resource(self.map.clone())
            .insert_resource(self.maps.clone())
            .insert_resource(self.atlas.clone())
            .insert_resource(self.campaign.clone())
            .insert_resource(CampaignEconomyTasks::default())
            .insert_resource(runtime)
            .init_resource::<FirstContactSimulationAdapter>()
            .init_resource::<FirstContactCommandIntents>()
            .init_resource::<CampaignUiIntents>()
            .init_resource::<MouseSelectionState>()
            .init_resource::<FirstContactVisualAcceptance>()
            .init_resource::<WorldUiState>()
            .add_systems(
                Startup,
                (
                    prepare_first_contact_live_scene,
                    spawn_first_contact_hud,
                    spawn_campaign_ui,
                    spawn_world_ui,
                    spawn_trnm_audio,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    begin_online_frame_timing,
                    pump_campaign_economy_tasks,
                    collect_campaign_ui_intents,
                    handle_world_ui_input,
                    handle_world_ui_interactions,
                    handle_campaign_input,
                    handle_campaign_ui_intents,
                    sync_first_contact_authored_map,
                    spawn_first_contact_authored_map,
                    handle_first_contact_mouse_selection,
                    handle_first_contact_command_card_interactions,
                    handle_first_contact_commands,
                    advance_first_contact_simulation,
                    settle_finished_battle,
                    expire_first_contact_feedback,
                    pan_first_contact_camera,
                    update_first_contact_hud,
                    update_campaign_ui,
                    sync_world_ui,
                    sync_trnm_audio,
                    animate_identity_geometry,
                    record_online_frame_timing,
                )
                    .chain(),
            );
        if let Some(online) = self.online.as_ref() {
            app.insert_resource(online.clone());
        }
        if let Some(timing) = OnlineFrameTiming::from_env() {
            app.insert_resource(timing);
        }
    }
}

pub fn default_first_contact_asset_root() -> PathBuf {
    std::env::var_os("TRNM_ASSET_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../assets"))
}

pub fn build_first_contact_live_bevy_app(low_spec: bool) -> Result<App, String> {
    let asset_root = default_first_contact_asset_root();
    let plugin = FirstContactLivePlugin::load(&asset_root)?;
    let mut app = App::new();
    if plugin.online.is_some() || std::env::var_os("TRNM_ONLINE_FRAME_TIMING_PATH").is_some() {
        // Online play must keep presenting while network work happens on its
        // worker, even on low-spec machines. Reactive desktop mode can sleep
        // for 250 ms without input and is unsuitable for a live RTS client.
        // Evidence-only offline runs use the same continuous production loop.
        app.insert_resource(bevy::winit::WinitSettings::continuous());
    } else if low_spec {
        app.insert_resource(bevy::winit::WinitSettings::desktop_app());
    }
    app.insert_resource(ClearColor(Color::srgb(0.015, 0.026, 0.024)))
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: asset_root.to_string_lossy().into_owned(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Trillionnium — First Contact".to_string(),
                        resolution: (1280, 720).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(plugin);
    Ok(app)
}
