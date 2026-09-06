#[derive(Debug, Clone)]
pub struct CampaignStore {
    path: PathBuf,
}

impl CampaignStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn load(&self) -> Result<CampaignSaveV1, CampaignError> {
        let bytes = fs::read(&self.path)?;
        let mut save: CampaignSaveV1 = serde_json::from_slice(&bytes)?;
        save.ensure_gameplay_defaults();
        save.validate()?;
        Ok(save)
    }

    pub fn load_or_default(&self) -> Result<CampaignSaveV1, CampaignError> {
        match self.load() {
            Ok(save) => Ok(save),
            Err(CampaignError::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {
                Ok(CampaignSaveV1::default())
            }
            Err(error) => Err(error),
        }
    }

    pub fn save_atomic(&self, save: &CampaignSaveV1) -> Result<(), CampaignError> {
        save.validate()?;
        atomic_write_json(&self.path, save)
    }

    pub fn stage_result_atomic(
        &self,
        save: &mut CampaignSaveV1,
        result: BattleResultV1,
    ) -> Result<(), CampaignError> {
        let mut candidate = save.clone();
        candidate.stage_battle_result(result)?;
        self.save_atomic(&candidate)?;
        *save = candidate;
        Ok(())
    }

    pub fn settle_atomic(
        &self,
        save: &mut CampaignSaveV1,
    ) -> Result<SettlementReceiptV1, CampaignError> {
        let mut candidate = save.clone();
        let receipt = candidate.apply_pending_settlement()?;
        self.save_atomic(&candidate)?;
        *save = candidate;
        Ok(receipt)
    }

    pub fn recover_pending_settlement(
        &self,
        save: &mut CampaignSaveV1,
    ) -> Result<Option<SettlementReceiptV1>, CampaignError> {
        if save.phase == CampaignPhase::PostBattlePending {
            self.settle_atomic(save).map(Some)
        } else {
            Ok(None)
        }
    }

    pub fn submit_result_atomic(
        &self,
        save: &mut CampaignSaveV1,
        result: BattleResultV1,
    ) -> Result<SettlementReceiptV1, CampaignError> {
        if let Some(existing) = save.receipt_for(&result.battle_id) {
            if existing.seed_hash != result.seed_hash
                || existing.result_hash != result.computed_hash()?
            {
                return Err(CampaignError::Integrity(
                    "replayed battle id carries a different result payload".to_string(),
                ));
            }
            return Ok(SettlementReceiptV1::duplicate_from(existing, save.revision));
        }
        self.stage_result_atomic(save, result)?;
        self.settle_atomic(save)
    }
}

