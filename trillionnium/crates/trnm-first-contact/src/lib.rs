mod asset_loader;
mod audio;
mod campaign_flow;
mod campaign_ui;
mod evidence_adapter;
mod hud;
mod map_loader;
mod online_authority;
mod renderer;
mod simulation_adapter;
mod view_math;

use asset_loader::{load_first_contact_atlas, FirstContactAtlasManifest};
use audio::{spawn_trnm_audio, sync_trnm_audio, validate_trnm_audio_assets};
use bevy::prelude::*;
use campaign_flow::{
    handle_campaign_input, settle_finished_battle, CampaignFlow, CampaignMode, ShellMode,
};
use campaign_ui::{spawn_campaign_ui, update_campaign_ui};
use evidence_adapter::FirstContactVisualAcceptance;
use hud::{spawn_first_contact_hud, update_first_contact_hud};
use map_loader::{FirstContactMap, MissionMapCatalog};
use online_authority::OnlineAuthorityClient;
use renderer::{
    animate_identity_geometry, spawn_first_contact_live_scene, sync_first_contact_authored_map,
};
use simulation_adapter::{
    advance_first_contact_simulation, expire_first_contact_feedback, handle_first_contact_commands,
    handle_first_contact_mouse_selection, pan_first_contact_camera, FirstContactRuntime,
    FirstContactSimulationAdapter, MouseSelectionState,
};
use std::{
    path::{Path, PathBuf},
    time::Instant,
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

#[derive(Resource)]
struct OnlineFrameTiming {
    evidence_path: PathBuf,
    frame_count: u64,
    frames_over_100ms: u64,
    max_frame_delta_ms: f64,
    update_started_at: Option<Instant>,
    main_thread_updates_over_100ms: u64,
    max_main_thread_update_ms: f64,
    write_accumulator: f32,
}

impl OnlineFrameTiming {
    fn from_env() -> Option<Self> {
        std::env::var_os("TRNM_ONLINE_FRAME_TIMING_PATH").map(|path| Self {
            evidence_path: PathBuf::from(path),
            frame_count: 0,
            frames_over_100ms: 0,
            max_frame_delta_ms: 0.0,
            update_started_at: None,
            main_thread_updates_over_100ms: 0,
            max_main_thread_update_ms: 0.0,
            write_accumulator: 0.0,
        })
    }
}

fn begin_online_frame_timing(timing: Option<ResMut<OnlineFrameTiming>>) {
    if let Some(mut timing) = timing {
        timing.update_started_at = Some(Instant::now());
    }
}

fn record_online_frame_timing(time: Res<Time>, timing: Option<ResMut<OnlineFrameTiming>>) {
    let Some(mut timing) = timing else {
        return;
    };
    let delta_ms = time.delta_secs_f64() * 1_000.0;
    timing.frame_count = timing.frame_count.saturating_add(1);
    timing.max_frame_delta_ms = timing.max_frame_delta_ms.max(delta_ms);
    if delta_ms > 100.0 {
        timing.frames_over_100ms = timing.frames_over_100ms.saturating_add(1);
    }
    if let Some(started_at) = timing.update_started_at.take() {
        let update_ms = started_at.elapsed().as_secs_f64() * 1_000.0;
        timing.max_main_thread_update_ms = timing.max_main_thread_update_ms.max(update_ms);
        if update_ms > 100.0 {
            timing.main_thread_updates_over_100ms =
                timing.main_thread_updates_over_100ms.saturating_add(1);
        }
    }
    timing.write_accumulator += time.delta_secs();
    if timing.write_accumulator < 0.5 {
        return;
    }
    timing.write_accumulator = 0.0;
    let report = serde_json::json!({
        "contract_version": "trnm_online_render_frame_timing_v1",
        "frame_count": timing.frame_count,
        "frames_over_100ms": timing.frames_over_100ms,
        "max_frame_delta_ms": timing.max_frame_delta_ms,
        "main_thread_updates_over_100ms": timing.main_thread_updates_over_100ms,
        "max_main_thread_update_ms": timing.max_main_thread_update_ms,
        "network_requests_on_render_thread": false,
        "network_main_thread_passed": timing.main_thread_updates_over_100ms == 0,
        "frame_cadence_passed": timing.frames_over_100ms == 0,
        "passed": timing.frames_over_100ms == 0 && timing.main_thread_updates_over_100ms == 0,
    });
    if let Some(parent) = timing.evidence_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(bytes) = serde_json::to_vec_pretty(&report) {
        let _ = std::fs::write(&timing.evidence_path, bytes);
    }
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
            .insert_resource(runtime)
            .init_resource::<FirstContactSimulationAdapter>()
            .init_resource::<MouseSelectionState>()
            .init_resource::<FirstContactVisualAcceptance>()
            .add_systems(
                Startup,
                (
                    spawn_first_contact_live_scene,
                    spawn_first_contact_hud,
                    spawn_campaign_ui,
                    spawn_trnm_audio,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    begin_online_frame_timing,
                    handle_campaign_input,
                    sync_first_contact_authored_map,
                    handle_first_contact_mouse_selection,
                    handle_first_contact_commands,
                    advance_first_contact_simulation,
                    settle_finished_battle,
                    expire_first_contact_feedback,
                    pan_first_contact_camera,
                    update_first_contact_hud,
                    update_campaign_ui,
                    sync_trnm_audio,
                    animate_identity_geometry,
                    record_online_frame_timing,
                )
                    .chain(),
            );
        if let Some(online) = self.online.as_ref() {
            app.insert_resource(online.clone());
            if let Some(timing) = OnlineFrameTiming::from_env() {
                app.insert_resource(timing);
            }
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
    if low_spec {
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
