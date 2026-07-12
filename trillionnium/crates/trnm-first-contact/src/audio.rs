use super::campaign_flow::CampaignFlow;
use bevy::prelude::*;
use rodio::{cpal::BufferSize, Decoder, DeviceSinkBuilder, Player, Source};
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
    sync::mpsc::{self, Receiver, Sender},
    time::{Duration, Instant},
};

const AUDIO_ASSETS: [(&str, usize); 2] = [
    ("first_contact/audio/mirror_city_loop.wav", 600_000),
    ("first_contact/audio/signal_battle_loop.wav", 300_000),
];
// The supported X230 service runs the renderer and audio under a shared 50%
// CPU quota. A long ambient-loop buffer is preferable to startup underruns;
// this audio baseline has no latency-sensitive voice or rhythm input.
const AUDIO_STABILITY_BUFFER_FRAMES: u32 = 32_768;

#[derive(Debug, Clone, Copy, PartialEq)]
struct AudioState {
    scene: AudioScene,
    volume: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AudioScene {
    Town,
    Battle,
    Epilogue,
}

#[derive(Debug)]
enum AudioCommand {
    Apply(AudioState),
}

#[derive(Resource)]
pub(super) struct TrnmAudioControl {
    sender: Sender<AudioCommand>,
    applied: AudioState,
}

fn live_volume(percent: u8) -> f32 {
    percent as f32 / 100.0
}

pub(super) fn validate_trnm_audio_assets(asset_root: &Path) -> Result<(), String> {
    for (relative, minimum_bytes) in AUDIO_ASSETS {
        let path = asset_root.join(relative);
        let bytes = fs::read(&path)
            .map_err(|error| format!("failed to read TRNM audio {}: {error}", path.display()))?;
        if bytes.len() < minimum_bytes
            || bytes.get(0..4) != Some(b"RIFF")
            || bytes.get(8..12) != Some(b"WAVE")
        {
            return Err(format!(
                "TRNM audio {} is not the expected authored PCM WAV",
                path.display()
            ));
        }
    }
    Ok(())
}

pub(super) fn spawn_trnm_audio(mut commands: Commands, flow: Res<CampaignFlow>) {
    let asset_root = super::default_first_contact_asset_root();
    let initial = AudioState {
        scene: audio_scene(&flow),
        volume: live_volume(flow.settings.master_volume_percent),
    };
    let (sender, receiver) = mpsc::channel();
    let thread_sender = sender.clone();
    std::thread::Builder::new()
        .name("trnm-audio".to_string())
        .spawn(move || {
            if let Err(error) = run_audio_thread(&asset_root, receiver, initial) {
                eprintln!("TRNM audio disabled after initialization failure: {error}");
            }
        })
        .expect("TRNM audio thread must spawn");
    let _ = thread_sender.send(AudioCommand::Apply(initial));
    commands.insert_resource(TrnmAudioControl {
        sender,
        applied: initial,
    });
}

fn run_audio_thread(
    asset_root: &Path,
    receiver: Receiver<AudioCommand>,
    initial: AudioState,
) -> Result<(), String> {
    let mut last_stream_warning = Instant::now() - Duration::from_secs(30);
    let stream = DeviceSinkBuilder::from_default_device()
        .map_err(|error| error.to_string())?
        .with_buffer_size(BufferSize::Fixed(AUDIO_STABILITY_BUFFER_FRAMES))
        .with_error_callback(move |error| {
            if last_stream_warning.elapsed() >= Duration::from_secs(10) {
                eprintln!("TRNM audio stream warning: {error}");
                last_stream_warning = Instant::now();
            }
        })
        .open_stream()
        .map_err(|error| error.to_string())?;
    let town = loop_player(
        &stream,
        asset_root.join("first_contact/audio/mirror_city_loop.wav"),
    )?;
    let battle = loop_player(
        &stream,
        asset_root.join("first_contact/audio/signal_battle_loop.wav"),
    )?;
    apply_audio_state(&town, &battle, initial);

    while let Ok(AudioCommand::Apply(state)) = receiver.recv() {
        apply_audio_state(&town, &battle, state);
    }
    Ok(())
}

fn loop_player(stream: &rodio::MixerDeviceSink, path: PathBuf) -> Result<Player, String> {
    let player = Player::connect_new(stream.mixer());
    let source = Decoder::try_from(
        File::open(&path)
            .map_err(|error| format!("failed to open TRNM audio {}: {error}", path.display()))?,
    )
    .map_err(|error| format!("failed to decode TRNM audio {}: {error}", path.display()))?;
    player.append(source.repeat_infinite());
    Ok(player)
}

fn apply_audio_state(town: &Player, battle: &Player, state: AudioState) {
    match state.scene {
        AudioScene::Town => {
            town.set_volume(state.volume);
            battle.pause();
            town.play();
        }
        AudioScene::Battle => {
            battle.set_volume(state.volume);
            town.pause();
            battle.play();
        }
        AudioScene::Epilogue => {
            town.set_volume(state.volume * 0.72);
            battle.set_volume(state.volume * 0.22);
            town.play();
            battle.play();
        }
    }
}

fn audio_scene(flow: &CampaignFlow) -> AudioScene {
    if flow.in_battle() {
        AudioScene::Battle
    } else if flow.save.main_story_ending.is_some() {
        AudioScene::Epilogue
    } else {
        AudioScene::Town
    }
}

pub(super) fn sync_trnm_audio(flow: Res<CampaignFlow>, mut audio: ResMut<TrnmAudioControl>) {
    let next = AudioState {
        scene: audio_scene(&flow),
        volume: live_volume(flow.settings.master_volume_percent),
    };
    if next != audio.applied && audio.sender.send(AudioCommand::Apply(next)).is_ok() {
        audio.applied = next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn master_volume_preference_maps_to_the_live_audio_player() {
        assert_eq!(live_volume(40), 0.4);
        assert_eq!(live_volume(0), 0.0);
    }

    #[test]
    fn original_town_and_battle_wav_assets_are_present_and_valid() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../assets");
        validate_trnm_audio_assets(&root).unwrap();
    }
}
