mod asset_loader;
mod audio;
mod campaign_flow;
mod campaign_ui;
mod evidence_adapter;
mod hud;
mod map_loader;
mod renderer;
mod simulation_adapter;
mod view_math;

use asset_loader::{load_first_contact_atlas, FirstContactAtlasManifest};
use audio::{spawn_trnm_audio, sync_trnm_audio, validate_trnm_audio_assets};
use bevy::prelude::*;
use campaign_flow::{handle_campaign_input, settle_finished_battle, CampaignFlow};
use campaign_ui::{spawn_campaign_ui, update_campaign_ui};
use evidence_adapter::FirstContactVisualAcceptance;
use hud::{spawn_first_contact_hud, update_first_contact_hud};
use map_loader::{FirstContactMap, MissionMapCatalog};
use renderer::{
    animate_identity_geometry, spawn_first_contact_live_scene, sync_first_contact_authored_map,
};
use simulation_adapter::{
    advance_first_contact_simulation, expire_first_contact_feedback, handle_first_contact_commands,
    handle_first_contact_mouse_selection, pan_first_contact_camera, FirstContactRuntime,
    FirstContactSimulationAdapter, MouseSelectionState,
};
use std::path::{Path, PathBuf};

pub use evidence_adapter::{FirstContactVisualAcceptance as VisualAcceptance, ObserverAnswer};

pub fn run_native_economy_e2e_phase(phase: &str) -> Result<serde_json::Value, String> {
    campaign_flow::run_native_economy_e2e_phase(phase)
}

pub struct FirstContactLivePlugin {
    map: FirstContactMap,
    maps: MissionMapCatalog,
    atlas: FirstContactAtlasManifest,
    campaign: CampaignFlow,
}

impl FirstContactLivePlugin {
    pub fn load(asset_root: &Path) -> Result<Self, String> {
        validate_trnm_audio_assets(asset_root)?;
        let maps = MissionMapCatalog::load(asset_root)?;
        let atlas = load_first_contact_atlas(&asset_root.join("first_contact/atlas.yaml"))?;
        let campaign = CampaignFlow::load()?;
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
        })
    }
}

impl Plugin for FirstContactLivePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.map.clone())
            .insert_resource(self.maps.clone())
            .insert_resource(self.atlas.clone())
            .insert_resource(self.campaign.clone())
            .init_resource::<FirstContactRuntime>()
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
                )
                    .chain(),
            );
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
