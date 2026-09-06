#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SaveSlotId {
    #[default]
    A,
    B,
    C,
}

impl SaveSlotId {
    pub const ALL: [Self; 3] = [Self::A, Self::B, Self::C];

    pub fn label(self) -> &'static str {
        match self {
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
        }
    }

    pub fn from_digit(digit: u8) -> Option<Self> {
        match digit {
            1 => Some(Self::A),
            2 => Some(Self::B),
            3 => Some(Self::C),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaveSlotMeta {
    pub slot: SaveSlotId,
    pub exists: bool,
    pub valid: bool,
    pub campaign_id: Option<String>,
    pub revision: Option<u64>,
    pub phase: Option<CampaignPhase>,
    pub mission: Option<CampaignMission>,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SaveSlotStore {
    root: PathBuf,
}

impl SaveSlotStore {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn path(&self, slot: SaveSlotId) -> PathBuf {
        match slot {
            SaveSlotId::A => self.root.join("campaign.json"),
            SaveSlotId::B => self.root.join("campaign-b.json"),
            SaveSlotId::C => self.root.join("campaign-c.json"),
        }
    }

    pub fn checkpoint_path(&self, slot: SaveSlotId) -> PathBuf {
        match slot {
            SaveSlotId::A => self.root.join("first-contact-battle.json"),
            SaveSlotId::B => self.root.join("campaign-b-battle.json"),
            SaveSlotId::C => self.root.join("campaign-c-battle.json"),
        }
    }

    pub fn load(&self, slot: SaveSlotId) -> Result<CampaignSaveV1, CampaignError> {
        CampaignStore::new(self.path(slot)).load()
    }

    pub fn load_or_default(&self, slot: SaveSlotId) -> Result<CampaignSaveV1, CampaignError> {
        CampaignStore::new(self.path(slot)).load_or_default()
    }

    pub fn save_atomic(
        &self,
        slot: SaveSlotId,
        save: &CampaignSaveV1,
    ) -> Result<(), CampaignError> {
        CampaignStore::new(self.path(slot)).save_atomic(save)
    }

    pub fn create_new(
        &self,
        slot: SaveSlotId,
        overwrite: bool,
    ) -> Result<CampaignSaveV1, CampaignError> {
        let path = self.path(slot);
        if path.exists() && !overwrite {
            return Err(CampaignError::InvalidState(format!(
                "slot {} requires explicit overwrite confirmation",
                slot.label()
            )));
        }
        let mut save = CampaignSaveV1 {
            campaign_id: format!("local-campaign-slot-{}", slot.label().to_ascii_lowercase()),
            ..CampaignSaveV1::default()
        };
        save.character_identity.confirmed = false;
        save.apply_character_identity_name();
        self.save_atomic(slot, &save)?;
        let checkpoint = self.checkpoint_path(slot);
        if checkpoint.exists() {
            fs::remove_file(checkpoint)?;
        }
        Ok(save)
    }

    pub fn metadata(&self, slot: SaveSlotId) -> SaveSlotMeta {
        let path = self.path(slot);
        if !path.exists() {
            return SaveSlotMeta {
                slot,
                exists: false,
                valid: false,
                campaign_id: None,
                revision: None,
                phase: None,
                mission: None,
                error: None,
            };
        }
        match self.load(slot) {
            Ok(save) => SaveSlotMeta {
                slot,
                exists: true,
                valid: true,
                campaign_id: Some(save.campaign_id),
                revision: Some(save.revision),
                phase: Some(save.phase),
                mission: Some(save.active_mission),
                error: None,
            },
            Err(error) => SaveSlotMeta {
                slot,
                exists: true,
                valid: false,
                campaign_id: None,
                revision: None,
                phase: None,
                mission: None,
                error: Some(error.to_string()),
            },
        }
    }

    pub fn list(&self) -> Vec<SaveSlotMeta> {
        SaveSlotId::ALL
            .into_iter()
            .map(|slot| self.metadata(slot))
            .collect()
    }
}

