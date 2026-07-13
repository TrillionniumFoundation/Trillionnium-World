use bevy::prelude::*;
use bevy::{input::keyboard::KeyboardInput, input::ButtonState};
use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
    time::Duration,
};
use trnm_online_protocol::{
    OnlineCampaignConnectRequest, OnlineCampaignView, OnlineLeaderboardView,
    OnlineOperationsAccessRequest, OnlineRatingView, OnlineReplayAccessRequest,
    OnlineReplayPlaybackView, OnlineSoloQueueAccessRequest, OnlineSoloQueueJoinRequest,
    OnlineSoloQueueStatus, OnlineSoloQueueView, ONLINE_AUTHORITY_BUILD, ONLINE_AUTHORITY_PROTOCOL,
    ONLINE_OPERATIONS_BUILD, ONLINE_OPERATIONS_PROTOCOL, ONLINE_PRODUCT_BUILD,
    ONLINE_PRODUCT_PROTOCOL,
};

#[derive(Debug, Clone, Deserialize)]
struct ProductSession {
    session_token: String,
    player_id: String,
    account_id: String,
    device_id: String,
    expires_at_epoch: i64,
}

#[derive(Debug, Clone, Serialize)]
struct ProductEvidence {
    contract: &'static str,
    player_id: String,
    state: String,
    account_id: Option<String>,
    campaign_id: Option<String>,
    rating: Option<i32>,
    season_id: Option<String>,
    season_rank: Option<u32>,
    queue_status: Option<String>,
    match_id: Option<String>,
    opponent_player_id: Option<String>,
    game_launched: bool,
    text_login_ready: bool,
    credential_source: String,
    replay_hash: Option<String>,
    replay_frame_count: Option<usize>,
    replay_integrity_verified: bool,
    status: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LoginField {
    PlayerId,
    Credential,
}

#[derive(Resource)]
struct ProductShell {
    cex_url: String,
    game_url: String,
    player_id: String,
    credential: String,
    credential_source: String,
    login_field: LoginField,
    device_id: String,
    slot_key: String,
    map_id: String,
    session: Option<ProductSession>,
    campaign: Option<OnlineCampaignView>,
    queue: Option<OnlineSoloQueueView>,
    rating: Option<OnlineRatingView>,
    leaderboard: Option<OnlineLeaderboardView>,
    replay_summary: Option<(String, String, usize, usize)>,
    state: String,
    status: String,
    poll_elapsed: f32,
    game_launched: bool,
    evidence_path: Option<PathBuf>,
    client: reqwest::blocking::Client,
}

#[derive(Component)]
struct ProductBody;

#[derive(Component)]
struct ProductStatus;

impl ProductShell {
    fn from_env() -> Result<Self, String> {
        let player_id = env::var("TRNM_PRODUCT_PLAYER_ID").unwrap_or_default();
        let credential = env::var("TRNM_PRODUCT_CREDENTIAL").unwrap_or_default();
        let credential_source = if credential.is_empty() {
            "not loaded"
        } else {
            "protected environment"
        };
        let mut shell = Self {
            cex_url: env::var("TRNM_CEX_LEDGER_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:7002".to_string())
                .trim_end_matches('/')
                .to_string(),
            game_url: env::var("TRNM_ONLINE_AUTHORITY_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:7005".to_string())
                .trim_end_matches('/')
                .to_string(),
            player_id,
            credential,
            credential_source: credential_source.to_string(),
            login_field: LoginField::PlayerId,
            device_id: env::var("TRNM_PRODUCT_DEVICE_ID")
                .unwrap_or_else(|_| "native-product-v2".to_string()),
            slot_key: env::var("TRNM_ONLINE_SLOT_KEY")
                .unwrap_or_else(|_| "ranked-main".to_string()),
            map_id: env::var("TRNM_PRODUCT_MAP_ID").unwrap_or_else(|_| "iron_delta".to_string()),
            session: None,
            campaign: None,
            queue: None,
            rating: None,
            leaderboard: None,
            replay_summary: None,
            state: "SIGNED OUT".to_string(),
            status: "Type player ID, TAB, credential; ENTER/F1 login. F6 saves to the Linux kernel keyring."
                .to_string(),
            poll_elapsed: 0.0,
            game_launched: false,
            evidence_path: env::var_os("TRNM_PRODUCT_EVIDENCE_PATH").map(PathBuf::from),
            client: reqwest::blocking::Client::builder()
                .connect_timeout(Duration::from_secs(2))
                .timeout(Duration::from_secs(5))
                .build()
                .map_err(|error| error.to_string())?,
        };
        if !shell.player_id.is_empty() && shell.credential.is_empty() {
            let _ = shell.load_kernel_credential();
        }
        Ok(shell)
    }

    fn validate_player_id(&self) -> Result<(), String> {
        if self.player_id.is_empty()
            || self.player_id.len() > 96
            || !self
                .player_id
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
        {
            return Err("player ID must be 1-96 portable ASCII characters".to_string());
        }
        Ok(())
    }

    fn validate_credential(&self) -> Result<(), String> {
        if self.credential.len() < 24
            || self.credential.len() > 512
            || !self.credential.bytes().all(|byte| byte.is_ascii_graphic())
        {
            return Err("credential must be 24-512 non-space ASCII characters".to_string());
        }
        Ok(())
    }

    fn key_description(&self) -> Result<String, String> {
        self.validate_player_id()?;
        Ok(format!("trnm-online-product:{}", self.player_id))
    }

    fn key_id(&self) -> Result<Option<String>, String> {
        let output = Command::new("keyctl")
            .args(["search", "@u", "user", &self.key_description()?])
            .output()
            .map_err(|error| format!("open Linux kernel keyring: {error}"))?;
        if !output.status.success() {
            return Ok(None);
        }
        let id = String::from_utf8(output.stdout)
            .map_err(|_| "kernel keyring returned an invalid key ID".to_string())?;
        let id = id.trim();
        if id.is_empty() || !id.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err("kernel keyring returned an invalid key ID".to_string());
        }
        Ok(Some(id.to_string()))
    }

    fn store_kernel_credential(&mut self) -> Result<(), String> {
        self.validate_credential()?;
        let existing = self.key_id()?;
        let mut command = Command::new("keyctl");
        if let Some(key_id) = existing {
            command.args(["update", &key_id]);
        } else {
            command.args(["padd", "user", &self.key_description()?, "@u"]);
        }
        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|error| format!("write Linux kernel keyring: {error}"))?;
        child
            .stdin
            .take()
            .ok_or_else(|| "kernel keyring stdin is unavailable".to_string())?
            .write_all(self.credential.as_bytes())
            .map_err(|error| format!("write Linux kernel keyring: {error}"))?;
        if !child
            .wait()
            .map_err(|error| format!("wait for Linux kernel keyring: {error}"))?
            .success()
        {
            return Err("Linux kernel keyring rejected the credential".to_string());
        }
        self.credential_source = "Linux kernel user keyring".to_string();
        self.status = "Credential saved in the Linux kernel user keyring; F1 LOGIN".to_string();
        Ok(())
    }

    fn load_kernel_credential(&mut self) -> Result<(), String> {
        let key_id = self
            .key_id()?
            .ok_or_else(|| "no credential is stored for this player".to_string())?;
        let output = Command::new("keyctl")
            .args(["pipe", &key_id])
            .output()
            .map_err(|error| format!("read Linux kernel keyring: {error}"))?;
        if !output.status.success() {
            return Err("Linux kernel keyring denied credential read".to_string());
        }
        self.credential = String::from_utf8(output.stdout)
            .map_err(|_| "stored credential is not valid UTF-8".to_string())?;
        self.validate_credential()?;
        self.credential_source = "Linux kernel user keyring".to_string();
        self.status = "Credential restored from the Linux kernel keyring; F1 LOGIN".to_string();
        Ok(())
    }

    fn forget_kernel_credential(&mut self) -> Result<(), String> {
        if let Some(key_id) = self.key_id()? {
            let status = Command::new("keyctl")
                .args(["unlink", &key_id, "@u"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .map_err(|error| format!("remove Linux kernel key: {error}"))?;
            if !status.success() {
                return Err("Linux kernel keyring rejected credential removal".to_string());
            }
        }
        self.credential.clear();
        self.credential_source = "not loaded".to_string();
        self.status = "Stored credential removed; type a credential to continue".to_string();
        Ok(())
    }

    fn login(&mut self) -> Result<(), String> {
        self.validate_player_id()?;
        self.validate_credential()?;
        let response = self
            .client
            .post(format!("{}/v1/trnm/product/login", self.cex_url))
            .json(&serde_json::json!({
                "player_id": self.player_id,
                "credential": self.credential,
                "device_id": self.device_id,
                "lifetime_seconds": 3600,
            }))
            .send()
            .map_err(|error| format!("login transport: {error}"))?;
        let status = response.status();
        let body = response.text().map_err(|error| error.to_string())?;
        if !status.is_success() {
            return Err(format!("login rejected ({status}): {body}"));
        }
        let session: ProductSession =
            serde_json::from_str(&body).map_err(|error| format!("login response: {error}"))?;
        if session.player_id != self.player_id || session.device_id != self.device_id {
            return Err("login response identity mismatch".to_string());
        }
        self.session = Some(session);
        self.state = "AUTHENTICATED".to_string();
        self.status = "Login verified. F2 CONNECT CLOUD CHARACTER".to_string();
        Ok(())
    }

    fn session(&self) -> Result<&ProductSession, String> {
        self.session
            .as_ref()
            .ok_or_else(|| "login is required first".to_string())
    }

    fn connect_campaign(&mut self) -> Result<(), String> {
        let session = self.session()?.clone();
        let response = self
            .client
            .post(format!("{}/v1/online/campaigns/connect", self.game_url))
            .header("x-trnm-player-session", &session.session_token)
            .json(&OnlineCampaignConnectRequest {
                protocol_version: ONLINE_AUTHORITY_PROTOCOL.to_string(),
                build_id: ONLINE_AUTHORITY_BUILD.to_string(),
                player_id: session.player_id,
                account_id: session.account_id,
                slot_key: self.slot_key.clone(),
            })
            .send()
            .map_err(|error| format!("cloud character transport: {error}"))?;
        let status = response.status();
        let body = response.text().map_err(|error| error.to_string())?;
        if !status.is_success() {
            return Err(format!("cloud character rejected ({status}): {body}"));
        }
        self.campaign = Some(
            serde_json::from_str(&body)
                .map_err(|error| format!("cloud character response: {error}"))?,
        );
        self.refresh_rating()?;
        self.refresh_operations()?;
        self.state = "LOBBY".to_string();
        self.status = "Cloud character ready. F3 JOIN RANKED SOLO QUEUE".to_string();
        Ok(())
    }

    fn access(&self) -> Result<OnlineSoloQueueAccessRequest, String> {
        let session = self.session()?;
        Ok(OnlineSoloQueueAccessRequest {
            protocol_version: ONLINE_PRODUCT_PROTOCOL.to_string(),
            build_id: ONLINE_PRODUCT_BUILD.to_string(),
            player_id: session.player_id.clone(),
            account_id: session.account_id.clone(),
        })
    }

    fn refresh_rating(&mut self) -> Result<(), String> {
        let session = self.session()?.clone();
        let response = self
            .client
            .post(format!("{}/v1/product/rating", self.game_url))
            .header("x-trnm-player-session", &session.session_token)
            .json(&self.access()?)
            .send()
            .map_err(|error| format!("rating transport: {error}"))?;
        if !response.status().is_success() {
            return Err(format!("rating rejected: {}", response.status()));
        }
        self.rating = Some(response.json().map_err(|error| error.to_string())?);
        Ok(())
    }

    fn refresh_operations(&mut self) -> Result<(), String> {
        let session = self.session()?.clone();
        let response = self
            .client
            .post(format!("{}/v1/operations/leaderboard", self.game_url))
            .header("x-trnm-player-session", &session.session_token)
            .json(&OnlineOperationsAccessRequest {
                protocol_version: ONLINE_OPERATIONS_PROTOCOL.to_string(),
                build_id: ONLINE_OPERATIONS_BUILD.to_string(),
                player_id: session.player_id,
                account_id: session.account_id,
            })
            .send()
            .map_err(|error| format!("operations transport: {error}"))?;
        let status = response.status();
        let body = response.text().map_err(|error| error.to_string())?;
        if !status.is_success() {
            return Err(format!("operations rejected ({status}): {body}"));
        }
        self.leaderboard = Some(
            serde_json::from_str(&body).map_err(|error| format!("operations response: {error}"))?,
        );
        Ok(())
    }

    fn join_queue(&mut self) -> Result<(), String> {
        let session = self.session()?.clone();
        let campaign = self
            .campaign
            .as_ref()
            .ok_or_else(|| "cloud character is required first".to_string())?;
        let response = self
            .client
            .post(format!("{}/v1/product/solo-queue/join", self.game_url))
            .header("x-trnm-player-session", &session.session_token)
            .json(&OnlineSoloQueueJoinRequest {
                protocol_version: ONLINE_PRODUCT_PROTOCOL.to_string(),
                build_id: ONLINE_PRODUCT_BUILD.to_string(),
                player_id: session.player_id,
                account_id: session.account_id,
                campaign_id: campaign.campaign_id.clone(),
                map_id: self.map_id.clone(),
            })
            .send()
            .map_err(|error| format!("queue transport: {error}"))?;
        let status = response.status();
        let body = response.text().map_err(|error| error.to_string())?;
        if !status.is_success() {
            return Err(format!("queue rejected ({status}): {body}"));
        }
        self.queue =
            Some(serde_json::from_str(&body).map_err(|error| format!("queue response: {error}"))?);
        self.sync_queue_state();
        Ok(())
    }

    fn refresh_queue(&mut self) -> Result<(), String> {
        let session = self.session()?.clone();
        let response = self
            .client
            .post(format!("{}/v1/product/solo-queue/status", self.game_url))
            .header("x-trnm-player-session", &session.session_token)
            .json(&self.access()?)
            .send()
            .map_err(|error| format!("queue status transport: {error}"))?;
        let status = response.status();
        let body = response.text().map_err(|error| error.to_string())?;
        if !status.is_success() {
            return Err(format!("queue status rejected ({status}): {body}"));
        }
        self.queue = Some(
            serde_json::from_str(&body)
                .map_err(|error| format!("queue status response: {error}"))?,
        );
        self.sync_queue_state();
        Ok(())
    }

    fn cancel_queue(&mut self) -> Result<(), String> {
        let session = self.session()?.clone();
        let response = self
            .client
            .post(format!("{}/v1/product/solo-queue/cancel", self.game_url))
            .header("x-trnm-player-session", &session.session_token)
            .json(&self.access()?)
            .send()
            .map_err(|error| format!("queue cancel transport: {error}"))?;
        let status = response.status();
        let body = response.text().map_err(|error| error.to_string())?;
        if !status.is_success() {
            return Err(format!("queue cancel rejected ({status}): {body}"));
        }
        self.queue = Some(
            serde_json::from_str(&body)
                .map_err(|error| format!("queue cancel response: {error}"))?,
        );
        self.sync_queue_state();
        Ok(())
    }

    fn sync_queue_state(&mut self) {
        match self.queue.as_ref().map(|queue| queue.status) {
            Some(OnlineSoloQueueStatus::Queued) => {
                self.state = "MATCHMAKING".to_string();
                self.status = "Searching ranked PvP — F4 CANCEL".to_string();
            }
            Some(OnlineSoloQueueStatus::Matched) => {
                self.state = "MATCH FOUND".to_string();
                self.status = "Opponent allocated. F5 LAUNCH AUTHORITATIVE MATCH".to_string();
            }
            Some(OnlineSoloQueueStatus::Cancelled) => {
                self.state = "LOBBY".to_string();
                self.status = "Queue cancelled. F3 JOIN AGAIN".to_string();
            }
            None => {}
        }
    }

    fn launch_game(&mut self) -> Result<(), String> {
        if self.game_launched {
            return Err("authoritative game is already launched".to_string());
        }
        let match_id = self
            .queue
            .as_ref()
            .and_then(|queue| queue.match_id.clone())
            .ok_or_else(|| "matched queue ticket is required".to_string())?;
        let session = self.session()?.clone();
        let current = env::current_exe().map_err(|error| error.to_string())?;
        let game = current.with_file_name("trnm-first-contact");
        Command::new(game)
            .env("TRNM_ONLINE_AUTHORITY_URL", &self.game_url)
            .env("TRNM_ONLINE_MATCH_ID", match_id)
            .env("TRNM_CEX_ACTOR_ID", &session.player_id)
            .env("TRNM_CEX_ACCOUNT_ID", &session.account_id)
            .env("TRNM_CEX_PLAYER_SESSION", &session.session_token)
            .spawn()
            .map_err(|error| format!("launch authoritative game: {error}"))?;
        self.game_launched = true;
        self.state = "IN MATCH".to_string();
        self.status = "Authoritative PvP client launched in a separate native window".to_string();
        Ok(())
    }

    fn load_replay(&mut self) -> Result<(), String> {
        let session = self.session()?.clone();
        if self
            .queue
            .as_ref()
            .and_then(|queue| queue.match_id.as_ref())
            .is_none()
        {
            let _ = self.refresh_queue();
        }
        let match_id = self.queue.as_ref().and_then(|queue| queue.match_id.clone());
        let endpoint = if match_id.is_some() {
            "/v1/operations/replays/playback"
        } else {
            "/v1/operations/replays/latest/playback"
        };
        let request = self
            .client
            .post(format!("{}{}", self.game_url, endpoint))
            .header("x-trnm-player-session", &session.session_token);
        let response = if let Some(match_id) = match_id {
            request.json(&OnlineReplayAccessRequest {
                protocol_version: ONLINE_OPERATIONS_PROTOCOL.to_string(),
                build_id: ONLINE_OPERATIONS_BUILD.to_string(),
                player_id: session.player_id.clone(),
                account_id: session.account_id.clone(),
                match_id,
            })
        } else {
            request.json(&OnlineOperationsAccessRequest {
                protocol_version: ONLINE_OPERATIONS_PROTOCOL.to_string(),
                build_id: ONLINE_OPERATIONS_BUILD.to_string(),
                player_id: session.player_id,
                account_id: session.account_id,
            })
        }
        .send()
        .map_err(|error| format!("replay playback transport: {error}"))?;
        let status = response.status();
        let body = response.text().map_err(|error| error.to_string())?;
        if !status.is_success() {
            return Err(format!("replay playback rejected ({status}): {body}"));
        }
        let playback: OnlineReplayPlaybackView = serde_json::from_str(&body)
            .map_err(|error| format!("replay playback response: {error}"))?;
        if !playback.integrity_verified {
            return Err("replay playback package failed integrity verification".to_string());
        }
        self.replay_summary = Some((
            playback.replay.match_id,
            playback.replay.replay_hash,
            playback.frames.len(),
            playback.commands.len(),
        ));
        self.state = "REPLAY READY".to_string();
        self.status = "Authoritative replay timeline verified; F9 refreshes it".to_string();
        Ok(())
    }

    fn write_evidence(&self) {
        let Some(path) = &self.evidence_path else {
            return;
        };
        let evidence = ProductEvidence {
            contract: "trnm_native_online_operations_v2",
            player_id: self.player_id.clone(),
            state: self.state.clone(),
            account_id: self
                .session
                .as_ref()
                .map(|session| session.account_id.clone()),
            campaign_id: self
                .campaign
                .as_ref()
                .map(|campaign| campaign.campaign_id.clone()),
            rating: self.rating.as_ref().map(|rating| rating.rating),
            season_id: self
                .leaderboard
                .as_ref()
                .map(|leaderboard| leaderboard.season.season_id.clone()),
            season_rank: self
                .leaderboard
                .as_ref()
                .and_then(|leaderboard| leaderboard.requester.as_ref().map(|entry| entry.rank)),
            queue_status: self
                .queue
                .as_ref()
                .map(|queue| format!("{:?}", queue.status)),
            match_id: self
                .queue
                .as_ref()
                .and_then(|queue| queue.match_id.clone())
                .or_else(|| {
                    self.replay_summary
                        .as_ref()
                        .map(|(match_id, _, _, _)| match_id.clone())
                }),
            opponent_player_id: self
                .queue
                .as_ref()
                .and_then(|queue| queue.opponent_player_id.clone()),
            game_launched: self.game_launched,
            text_login_ready: true,
            credential_source: self.credential_source.clone(),
            replay_hash: self
                .replay_summary
                .as_ref()
                .map(|(_, hash, _, _)| hash.clone()),
            replay_frame_count: self
                .replay_summary
                .as_ref()
                .map(|(_, _, frames, _)| *frames),
            replay_integrity_verified: self.replay_summary.is_some(),
            status: self.status.clone(),
        };
        if let Ok(bytes) = serde_json::to_vec_pretty(&evidence) {
            let _ = fs::write(path, bytes);
        }
    }
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: px(18),
            padding: UiRect::all(px(44)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.012, 0.025, 0.024)),
        children![
            (
                Text::new("TRILLIONNIUM ONLINE PRODUCT v2 / OPERATIONS v2"),
                TextFont::from_font_size(34.0),
                TextColor(Color::srgb(0.95, 0.82, 0.42)),
            ),
            (
                Node {
                    width: px(900),
                    min_height: px(300),
                    padding: UiRect::all(px(30)),
                    border: UiRect::all(px(2)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.035, 0.070, 0.064, 0.98)),
                BorderColor::all(Color::srgb(0.25, 0.52, 0.42)),
                children![(
                    Text::new("Loading native product shell..."),
                    ProductBody,
                    TextFont::from_font_size(20.0),
                    TextColor(Color::srgb(0.88, 0.92, 0.78)),
                )],
            ),
            (
                Text::new("TAB FIELD | ENTER/F1 LOGIN | F2 CLOUD | F3 QUEUE | F4 CANCEL | F5 PLAY | F6 SAVE | F7 LOAD | F8 FORGET | F9 REPLAY"),
                TextFont::from_font_size(17.0),
                TextColor(Color::srgb(0.62, 0.88, 0.70)),
            ),
            (
                Text::new("Product shell ready"),
                ProductStatus,
                TextFont::from_font_size(15.0),
                TextColor(Color::srgb(0.72, 0.80, 0.76)),
            ),
        ],
    ));
}

fn handle_input(input: Res<ButtonInput<KeyCode>>, mut shell: ResMut<ProductShell>) {
    let result = if input.just_pressed(KeyCode::F1) || input.just_pressed(KeyCode::Enter) {
        shell.login()
    } else if input.just_pressed(KeyCode::F2) {
        shell.connect_campaign()
    } else if input.just_pressed(KeyCode::F3) {
        shell.join_queue()
    } else if input.just_pressed(KeyCode::F4) {
        shell.cancel_queue()
    } else if input.just_pressed(KeyCode::F5) {
        shell.launch_game()
    } else if input.just_pressed(KeyCode::F6) {
        shell.store_kernel_credential()
    } else if input.just_pressed(KeyCode::F7) {
        shell.load_kernel_credential()
    } else if input.just_pressed(KeyCode::F8) {
        shell.forget_kernel_credential()
    } else if input.just_pressed(KeyCode::F9) {
        shell.load_replay()
    } else {
        return;
    };
    if let Err(error) = result {
        shell.status = error;
    }
    shell.write_evidence();
}

fn handle_text_input(
    mut keyboard_inputs: MessageReader<KeyboardInput>,
    mut shell: ResMut<ProductShell>,
) {
    if shell.session.is_some() {
        return;
    }
    let mut changed = false;
    for event in keyboard_inputs.read() {
        if event.state != ButtonState::Pressed {
            continue;
        }
        match event.key_code {
            KeyCode::Tab => {
                shell.login_field = match shell.login_field {
                    LoginField::PlayerId => LoginField::Credential,
                    LoginField::Credential => LoginField::PlayerId,
                };
                changed = true;
            }
            KeyCode::Backspace => {
                match shell.login_field {
                    LoginField::PlayerId => {
                        shell.player_id.pop();
                    }
                    LoginField::Credential => {
                        shell.credential.pop();
                        shell.credential_source = "typed in native window".to_string();
                    }
                }
                changed = true;
            }
            _ => {
                let Some(text) = event.text.as_deref() else {
                    continue;
                };
                for character in text
                    .chars()
                    .filter(|character| character.is_ascii_graphic())
                {
                    match shell.login_field {
                        LoginField::PlayerId
                            if shell.player_id.len() < 96
                                && (character.is_ascii_alphanumeric()
                                    || matches!(character, '-' | '_' | '.')) =>
                        {
                            shell.player_id.push(character);
                            changed = true;
                        }
                        LoginField::Credential if shell.credential.len() < 512 => {
                            shell.credential.push(character);
                            shell.credential_source = "typed in native window".to_string();
                            changed = true;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    if changed {
        shell.status = format!(
            "Editing {} — TAB switches field; ENTER/F1 logs in",
            match shell.login_field {
                LoginField::PlayerId => "PLAYER ID",
                LoginField::Credential => "CREDENTIAL",
            }
        );
        shell.write_evidence();
    }
}

fn poll_queue(time: Res<Time>, mut shell: ResMut<ProductShell>) {
    if shell.queue.as_ref().map(|queue| queue.status) != Some(OnlineSoloQueueStatus::Queued) {
        return;
    }
    shell.poll_elapsed += time.delta_secs();
    if shell.poll_elapsed < 1.0 {
        return;
    }
    shell.poll_elapsed = 0.0;
    if let Err(error) = shell.refresh_queue() {
        shell.status = error;
    }
    shell.write_evidence();
}

fn update_ui(
    shell: Res<ProductShell>,
    mut body: Query<&mut Text, (With<ProductBody>, Without<ProductStatus>)>,
    mut status: Query<&mut Text, (With<ProductStatus>, Without<ProductBody>)>,
) {
    if !shell.is_changed() {
        return;
    }
    let account = shell
        .session
        .as_ref()
        .map(|session| session.account_id.as_str())
        .unwrap_or("not authenticated");
    let session_expiry = shell
        .session
        .as_ref()
        .map(|session| session.expires_at_epoch.to_string())
        .unwrap_or_else(|| "—".to_string());
    let campaign = shell
        .campaign
        .as_ref()
        .map(|campaign| campaign.campaign_id.as_str())
        .unwrap_or("not connected");
    let rating = shell
        .rating
        .as_ref()
        .map(|rating| format!("{}  W{} L{}", rating.rating, rating.wins, rating.losses))
        .unwrap_or_else(|| "unrated".to_string());
    let opponent = shell
        .queue
        .as_ref()
        .and_then(|queue| queue.opponent_player_id.as_deref())
        .unwrap_or("searching / none");
    let season = shell
        .leaderboard
        .as_ref()
        .map(|leaderboard| leaderboard.season.display_name.as_str())
        .unwrap_or("not loaded");
    let season_rank = shell
        .leaderboard
        .as_ref()
        .and_then(|leaderboard| leaderboard.requester.as_ref())
        .map(|entry| format!("#{} / {}", entry.rank, entry.rating))
        .unwrap_or_else(|| "unranked".to_string());
    let match_id = shell
        .queue
        .as_ref()
        .and_then(|queue| queue.match_id.as_deref())
        .or_else(|| {
            shell
                .replay_summary
                .as_ref()
                .map(|(match_id, _, _, _)| match_id.as_str())
        })
        .unwrap_or("not allocated");
    let masked_credential = if shell.credential.is_empty() {
        "<empty>".to_string()
    } else {
        format!("{} chars ********", shell.credential.len())
    };
    let replay = shell
        .replay_summary
        .as_ref()
        .map(|(_, hash, frames, commands)| {
            format!(
                "verified {} frames / {} commands / {}…",
                frames,
                commands,
                &hash[..12]
            )
        })
        .unwrap_or_else(|| "not loaded".to_string());
    body.single_mut().expect("one product body").0 = format!(
        "STATE: {}\n\nLOGIN FIELD: {}\nPLAYER: {}\nCREDENTIAL: {}\nCREDENTIAL SOURCE: {}\nACCOUNT: {}\nSESSION EXPIRY: {}\nCLOUD CHARACTER: {}\nMAP: {}\nRANKED MMR: {}\nSEASON: {}\nSEASON RANK: {}\nOPPONENT: {}\nMATCH: {}\nREPLAY: {}\n\nThe credential value is never rendered, logged or passed to the game process. The game receives only the scoped player session.",
        shell.state,
        match shell.login_field { LoginField::PlayerId => "PLAYER ID", LoginField::Credential => "CREDENTIAL" },
        if shell.player_id.is_empty() { "<type player ID>" } else { shell.player_id.as_str() },
        masked_credential,
        shell.credential_source,
        account,
        session_expiry,
        campaign,
        shell.map_id,
        rating,
        season,
        season_rank,
        opponent,
        match_id,
        replay,
    );
    status.single_mut().expect("one product status").0 = shell.status.clone();
}

fn main() {
    let shell = ProductShell::from_env().unwrap_or_else(|error| panic!("{error}"));
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::srgb(0.012, 0.025, 0.024)))
        .insert_resource(shell)
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Trillionnium — Online Product v2".to_string(),
                        resolution: (1180, 720).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, spawn_ui)
        .add_systems(
            Update,
            (handle_text_input, handle_input, poll_queue, update_ui).chain(),
        );
    app.run();
}
