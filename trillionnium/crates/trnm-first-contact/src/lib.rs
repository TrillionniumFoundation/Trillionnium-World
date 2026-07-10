mod asset_loader;
mod campaign_flow;
mod campaign_ui;
mod evidence_adapter;
mod hud;
mod map_loader;
mod renderer;
mod simulation_adapter;

use asset_loader::{load_first_contact_atlas, FirstContactAtlasManifest};
use bevy::prelude::*;
use campaign_flow::{handle_campaign_input, settle_finished_battle, CampaignFlow};
use campaign_ui::{spawn_campaign_ui, update_campaign_ui};
use evidence_adapter::FirstContactVisualAcceptance;
use hud::{spawn_first_contact_hud, update_first_contact_hud};
use map_loader::{load_first_contact_map, FirstContactMap};
use renderer::spawn_first_contact_live_scene;
use simulation_adapter::{
    advance_first_contact_simulation, expire_first_contact_feedback, handle_first_contact_commands,
    pan_first_contact_camera, FirstContactRuntime, FirstContactSimulationAdapter,
};
use std::path::{Path, PathBuf};

pub use evidence_adapter::{FirstContactVisualAcceptance as VisualAcceptance, ObserverAnswer};

pub struct FirstContactLivePlugin {
    map: FirstContactMap,
    atlas: FirstContactAtlasManifest,
    campaign: CampaignFlow,
}

impl FirstContactLivePlugin {
    pub fn load(asset_root: &Path) -> Result<Self, String> {
        let map =
            load_first_contact_map(&asset_root.join("first_contact/maps/first_contact.yaml"))?;
        let atlas = load_first_contact_atlas(&asset_root.join("first_contact/atlas.yaml"))?;
        let campaign = CampaignFlow::load()?;
        Ok(Self {
            map,
            atlas,
            campaign,
        })
    }
}

impl Plugin for FirstContactLivePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.map.clone())
            .insert_resource(self.atlas.clone())
            .insert_resource(self.campaign.clone())
            .init_resource::<FirstContactRuntime>()
            .init_resource::<FirstContactSimulationAdapter>()
            .init_resource::<FirstContactVisualAcceptance>()
            .add_systems(
                Startup,
                (
                    spawn_first_contact_live_scene,
                    spawn_first_contact_hud,
                    spawn_campaign_ui,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    handle_campaign_input,
                    handle_first_contact_commands,
                    advance_first_contact_simulation,
                    settle_finished_battle,
                    expire_first_contact_feedback,
                    pan_first_contact_camera,
                    update_first_contact_hud,
                    update_campaign_ui,
                )
                    .chain(),
            );
    }
}

pub fn default_first_contact_asset_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../assets")
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
