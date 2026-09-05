pub const PLAYER_SETTINGS_CONTRACT: &str = "trnm_player_settings_v2";

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputMode {
    #[default]
    Hybrid,
    KeyboardOnly,
    MouseOnly,
}

impl InputMode {
    pub fn next(self) -> Self {
        match self {
            Self::Hybrid => Self::KeyboardOnly,
            Self::KeyboardOnly => Self::MouseOnly,
            Self::MouseOnly => Self::Hybrid,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlScheme {
    #[default]
    Classic,
    LeftHanded,
    ArrowGrid,
}

impl ControlScheme {
    pub fn next(self) -> Self {
        match self {
            Self::Classic => Self::LeftHanded,
            Self::LeftHanded => Self::ArrowGrid,
            Self::ArrowGrid => Self::Classic,
        }
    }
}

fn default_true() -> bool {
    true
}
fn default_volume() -> u8 {
    80
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerSettings {
    pub contract_version: String,
    #[serde(default)]
    pub low_motion: bool,
    #[serde(default)]
    pub input_mode: InputMode,
    #[serde(default = "default_true")]
    pub subtitles: bool,
    #[serde(default)]
    pub high_contrast: bool,
    #[serde(default)]
    pub control_scheme: ControlScheme,
    #[serde(default = "default_volume")]
    pub master_volume_percent: u8,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            contract_version: PLAYER_SETTINGS_CONTRACT.to_string(),
            low_motion: false,
            input_mode: InputMode::Hybrid,
            subtitles: true,
            high_contrast: false,
            control_scheme: ControlScheme::Classic,
            master_volume_percent: 80,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerSettingsStore {
    path: PathBuf,
}

impl PlayerSettingsStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn load_or_default(&self) -> Result<PlayerSettings, CampaignError> {
        if !self.path.exists() {
            return Ok(PlayerSettings::default());
        }
        let bytes = fs::read(&self.path)?;
        let mut value: serde_json::Value = serde_json::from_slice(&bytes)?;
        if value
            .get("contract_version")
            .and_then(serde_json::Value::as_str)
            == Some("trnm_player_settings_v1")
        {
            value["contract_version"] =
                serde_json::Value::String(PLAYER_SETTINGS_CONTRACT.to_string());
        }
        let settings: PlayerSettings = serde_json::from_value(value)?;
        if settings.contract_version != PLAYER_SETTINGS_CONTRACT
            || settings.master_volume_percent > 100
        {
            return Err(CampaignError::InvalidContract(settings.contract_version));
        }
        Ok(settings)
    }

    pub fn save_atomic(&self, settings: &PlayerSettings) -> Result<(), CampaignError> {
        if settings.contract_version != PLAYER_SETTINGS_CONTRACT
            || settings.master_volume_percent > 100
        {
            return Err(CampaignError::InvalidContract(
                settings.contract_version.clone(),
            ));
        }
        atomic_write_json(&self.path, settings)
    }
}

fn atomic_write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), CampaignError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let temp_path = path.with_extension("json.tmp");
    let payload = serde_json::to_vec_pretty(value)?;
    let mut file = fs::File::create(&temp_path)?;
    file.write_all(&payload)?;
    file.sync_all()?;
    fs::rename(&temp_path, path)?;
    if let Some(parent) = path.parent() {
        fs::File::open(parent)?.sync_all()?;
    }
    Ok(())
}

