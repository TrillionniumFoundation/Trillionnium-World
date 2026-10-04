impl BattleReplayV1 {
    pub fn replay_and_verify(&self) -> Result<MissionSimV1, SimError> {
        if self.contract_version != "trnm_battle_replay_v1" {
            return Err(SimError::InvalidState(
                "unsupported battle replay contract".to_string(),
            ));
        }
        let mut sim = MissionSimV1::from_seed(self.seed.clone())?;
        for entry in &self.entries {
            while sim.tick < entry.issued_tick {
                sim.step()?;
            }
            sim.issue_order(entry.order.clone())?;
        }
        while sim.tick < self.final_tick {
            sim.step()?;
        }
        if sim.snapshot_hash()? != self.final_snapshot_hash {
            return Err(SimError::Integrity(
                "battle replay diverged from recorded snapshot".to_string(),
            ));
        }
        Ok(sim)
    }

    pub fn save_atomic(&self, path: &Path) -> Result<(), SimError> {
        self.replay_and_verify()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let temp_path = path.with_extension("json.tmp");
        let mut file = fs::File::create(&temp_path)?;
        file.write_all(&serde_json::to_vec_pretty(self)?)?;
        file.sync_all()?;
        fs::rename(&temp_path, path)?;
        if let Some(parent) = path.parent() {
            fs::File::open(parent)?.sync_all()?;
        }
        Ok(())
    }

    pub fn load_verified(path: &Path) -> Result<Self, SimError> {
        let replay: Self = serde_json::from_slice(&fs::read(path)?)?;
        replay.replay_and_verify()?;
        Ok(replay)
    }

    pub fn migrate_to_v2(&self) -> Result<BattleReplayV2, SimError> {
        self.replay_and_verify()?;
        BattleReplayV2::from_entries(
            self.seed.clone(),
            self.entries.clone(),
            self.final_tick,
            self.final_snapshot_hash.clone(),
        )
    }
}

impl BattleReplayV2 {
    fn from_entries(
        seed: BattleSeedV1,
        entries: Vec<SimReplayEntry>,
        final_tick: u64,
        final_snapshot_hash: String,
    ) -> Result<Self, SimError> {
        let seek_checkpoints =
            Self::build_seek_checkpoints(&seed, &entries, final_tick, &final_snapshot_hash)?;
        let chunks = entries
            .chunks(REPLAY_CHUNK_ORDERS)
            .enumerate()
            .map(|(index, entries)| {
                let entries = entries.to_vec();
                Ok(BattleReplayChunkV2 {
                    index: index as u32,
                    first_tick: entries
                        .first()
                        .map(|entry| entry.issued_tick)
                        .unwrap_or_default(),
                    last_tick: entries
                        .last()
                        .map(|entry| entry.issued_tick)
                        .unwrap_or_default(),
                    chunk_hash: hash_json(&entries)?,
                    entries,
                })
            })
            .collect::<Result<Vec<_>, SimError>>()?;
        Ok(Self {
            contract_version: "trnm_battle_replay_v2".to_string(),
            seed,
            chunks,
            seek_checkpoints,
            final_tick,
            final_snapshot_hash,
        })
    }

    fn build_seek_checkpoints(
        seed: &BattleSeedV1,
        entries: &[SimReplayEntry],
        final_tick: u64,
        final_snapshot_hash: &str,
    ) -> Result<Vec<ReplaySeekCheckpointV2>, SimError> {
        let mut sim = MissionSimV1::from_seed(seed.clone())?;
        let mut checkpoints = vec![ReplaySeekCheckpointV2 {
            tick: 0,
            consumed_entry_count: 0,
            checkpoint: SimCheckpointV1::capture(&sim)?,
        }];
        let capture = |sim: &MissionSimV1,
                       consumed_entry_count: usize,
                       checkpoints: &mut Vec<ReplaySeekCheckpointV2>|
         -> Result<(), SimError> {
            if checkpoints
                .last()
                .is_some_and(|entry| entry.tick == sim.tick)
            {
                return Ok(());
            }
            checkpoints.push(ReplaySeekCheckpointV2 {
                tick: sim.tick,
                consumed_entry_count,
                checkpoint: SimCheckpointV1::capture(sim)?,
            });
            Ok(())
        };
        for (entry_index, entry) in entries.iter().enumerate() {
            while sim.tick < entry.issued_tick {
                sim.step()?;
                if sim.tick.is_multiple_of(REPLAY_SEEK_CHECKPOINT_TICKS) {
                    capture(&sim, entry_index, &mut checkpoints)?;
                }
            }
            sim.issue_order(entry.order.clone())?;
        }
        while sim.tick < final_tick {
            sim.step()?;
            if sim.tick.is_multiple_of(REPLAY_SEEK_CHECKPOINT_TICKS) {
                capture(&sim, entries.len(), &mut checkpoints)?;
            }
        }
        capture(&sim, entries.len(), &mut checkpoints)?;
        if sim.snapshot_hash()? != final_snapshot_hash {
            return Err(SimError::Integrity(
                "replay checkpoint construction diverged from final snapshot".to_string(),
            ));
        }
        Ok(checkpoints)
    }

    pub fn entry_count(&self) -> usize {
        self.chunks.iter().map(|chunk| chunk.entries.len()).sum()
    }

    pub fn validate_chunks(&self) -> Result<(), SimError> {
        if self.contract_version != "trnm_battle_replay_v2"
            || self.chunks.len() > MAX_REPLAY_ORDERS.div_ceil(REPLAY_CHUNK_ORDERS)
        {
            return Err(SimError::InvalidState(
                "unsupported or oversized chunked battle replay".to_string(),
            ));
        }
        let mut previous_tick = 0;
        for (index, chunk) in self.chunks.iter().enumerate() {
            if chunk.index as usize != index
                || chunk.entries.len() > REPLAY_CHUNK_ORDERS
                || chunk.chunk_hash != hash_json(&chunk.entries)?
                || chunk
                    .entries
                    .windows(2)
                    .any(|pair| pair[0].issued_tick > pair[1].issued_tick)
                || index > 0 && chunk.first_tick < previous_tick
            {
                return Err(SimError::Integrity(
                    "battle replay chunk ordering or hash is invalid".to_string(),
                ));
            }
            if let Some(first) = chunk.entries.first() {
                if chunk.first_tick != first.issued_tick {
                    return Err(SimError::Integrity(
                        "battle replay chunk first tick is invalid".to_string(),
                    ));
                }
            }
            if let Some(last) = chunk.entries.last() {
                if chunk.last_tick != last.issued_tick {
                    return Err(SimError::Integrity(
                        "battle replay chunk last tick is invalid".to_string(),
                    ));
                }
                previous_tick = last.issued_tick;
            }
        }
        let mut previous_checkpoint_tick = 0;
        let mut previous_entry_count = 0;
        for (index, checkpoint) in self.seek_checkpoints.iter().enumerate() {
            checkpoint.checkpoint.validate()?;
            if checkpoint.tick != checkpoint.checkpoint.sim.tick
                || checkpoint.tick > self.final_tick
                || checkpoint.consumed_entry_count > self.entry_count()
                || index > 0 && checkpoint.tick <= previous_checkpoint_tick
                || checkpoint.consumed_entry_count < previous_entry_count
                || checkpoint.checkpoint.sim.seed.seed_hash != self.seed.seed_hash
            {
                return Err(SimError::Integrity(
                    "replay seek checkpoint ordering or binding is invalid".to_string(),
                ));
            }
            previous_checkpoint_tick = checkpoint.tick;
            previous_entry_count = checkpoint.consumed_entry_count;
        }
        Ok(())
    }

    pub fn replay_and_verify(&self) -> Result<MissionSimV1, SimError> {
        self.validate_chunks()?;
        let mut sim = MissionSimV1::from_seed(self.seed.clone())?;
        for entry in self.chunks.iter().flat_map(|chunk| &chunk.entries) {
            while sim.tick < entry.issued_tick {
                sim.step()?;
            }
            sim.issue_order(entry.order.clone())?;
        }
        while sim.tick < self.final_tick {
            sim.step()?;
        }
        if sim.snapshot_hash()? != self.final_snapshot_hash {
            return Err(SimError::Integrity(
                "chunked battle replay diverged from recorded snapshot".to_string(),
            ));
        }
        Ok(sim)
    }

    pub fn replay_until_tick(&self, requested_tick: u64) -> Result<MissionSimV1, SimError> {
        self.validate_chunks()?;
        let target_tick = requested_tick.min(self.final_tick);
        let checkpoint = self
            .seek_checkpoints
            .iter()
            .rev()
            .find(|checkpoint| checkpoint.tick <= target_tick);
        let (mut sim, consumed_entry_count) = checkpoint
            .map(|checkpoint| {
                (
                    checkpoint.checkpoint.sim.clone(),
                    checkpoint.consumed_entry_count,
                )
            })
            .unwrap_or((MissionSimV1::from_seed(self.seed.clone())?, 0));
        for entry in self
            .chunks
            .iter()
            .flat_map(|chunk| &chunk.entries)
            .skip(consumed_entry_count)
            .filter(|entry| entry.issued_tick <= target_tick)
        {
            while sim.tick < entry.issued_tick {
                sim.step()?;
            }
            sim.issue_order(entry.order.clone())?;
        }
        while sim.tick < target_tick && !sim.terminal() {
            sim.step()?;
        }
        Ok(sim)
    }

    fn validate_persistable(&self) -> Result<(), SimError> {
        self.validate_chunks()?;
        let final_checkpoint = self.seek_checkpoints.last().ok_or_else(|| {
            SimError::Integrity("replay is missing its terminal seek checkpoint".to_string())
        })?;
        if final_checkpoint.tick != self.final_tick
            || final_checkpoint.consumed_entry_count != self.entry_count()
            || final_checkpoint.checkpoint.sim.snapshot_hash()? != self.final_snapshot_hash
        {
            return Err(SimError::Integrity(
                "replay terminal checkpoint diverged from its manifest".to_string(),
            ));
        }
        Ok(())
    }

    pub fn save_atomic(&self, path: &Path) -> Result<(), SimError> {
        // Replay construction already performs a full deterministic replay.
        // Persistence revalidates every chunk/checkpoint hash and binds the
        // terminal checkpoint to the manifest without redundantly simulating
        // the entire match a second time. Loading remains a full verification.
        self.validate_persistable()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let temp_path = path.with_extension("json.tmp");
        let mut file = fs::File::create(&temp_path)?;
        file.write_all(&serde_json::to_vec_pretty(self)?)?;
        file.sync_all()?;
        fs::rename(&temp_path, path)?;
        if let Some(parent) = path.parent() {
            fs::File::open(parent)?.sync_all()?;
        }
        Ok(())
    }

    pub fn load_verified(path: &Path) -> Result<Self, SimError> {
        let bytes = fs::read(path)?;
        let value: serde_json::Value = serde_json::from_slice(&bytes)?;
        match value
            .get("contract_version")
            .and_then(serde_json::Value::as_str)
        {
            Some("trnm_battle_replay_v2") => {
                let replay: Self = serde_json::from_value(value)?;
                replay.replay_and_verify()?;
                Ok(replay)
            }
            Some("trnm_battle_replay_v1") => {
                let legacy: BattleReplayV1 = serde_json::from_value(value)?;
                let migrated = legacy.migrate_to_v2()?;
                migrated.save_atomic(path)?;
                Ok(migrated)
            }
            _ => Err(SimError::InvalidState(
                "unsupported battle replay file contract".to_string(),
            )),
        }
    }

    pub fn save_chunk_directory(&self, directory: &Path) -> Result<(), SimError> {
        self.validate_persistable()?;
        fs::create_dir_all(directory)?;
        let mut chunk_files = Vec::with_capacity(self.chunks.len());
        for chunk in &self.chunks {
            let filename = format!("chunk-{:05}.json", chunk.index);
            let path = directory.join(&filename);
            let mut file = fs::File::create(&path)?;
            file.write_all(&serde_json::to_vec(chunk)?)?;
            file.sync_all()?;
            chunk_files.push(filename);
        }
        let manifest = ReplayChunkDirectoryManifestV1 {
            contract_version: "trnm_battle_replay_chunk_directory_v1".to_string(),
            seed: self.seed.clone(),
            chunk_files,
            seek_checkpoints: self.seek_checkpoints.clone(),
            final_tick: self.final_tick,
            final_snapshot_hash: self.final_snapshot_hash.clone(),
        };
        let temp_path = directory.join("manifest.json.tmp");
        let manifest_path = directory.join("manifest.json");
        let mut file = fs::File::create(&temp_path)?;
        file.write_all(&serde_json::to_vec_pretty(&manifest)?)?;
        file.sync_all()?;
        fs::rename(temp_path, manifest_path)?;
        fs::File::open(directory)?.sync_all()?;
        Ok(())
    }

    pub fn load_chunk_directory_verified(directory: &Path) -> Result<Self, SimError> {
        let manifest: ReplayChunkDirectoryManifestV1 =
            serde_json::from_slice(&fs::read(directory.join("manifest.json"))?)?;
        if manifest.contract_version != "trnm_battle_replay_chunk_directory_v1" {
            return Err(SimError::InvalidState(
                "unsupported replay chunk directory contract".to_string(),
            ));
        }
        let chunks = manifest
            .chunk_files
            .iter()
            .map(|filename| -> Result<BattleReplayChunkV2, SimError> {
                Ok(serde_json::from_slice(&fs::read(
                    directory.join(filename),
                )?)?)
            })
            .collect::<Result<Vec<BattleReplayChunkV2>, SimError>>()?;
        let replay = Self {
            contract_version: "trnm_battle_replay_v2".to_string(),
            seed: manifest.seed,
            chunks,
            seek_checkpoints: manifest.seek_checkpoints,
            final_tick: manifest.final_tick,
            final_snapshot_hash: manifest.final_snapshot_hash,
        };
        replay.replay_and_verify()?;
        Ok(replay)
    }
}

fn percent(current: i64, maximum: i64) -> u8 {
    if maximum <= 0 {
        0
    } else {
        (current * 100 / maximum).clamp(0, 100) as u8
    }
}

fn distance(left: BattleGridPoint, right: BattleGridPoint) -> i16 {
    (left.x - right.x).abs() + (left.y - right.y).abs()
}

fn neighbors(point: BattleGridPoint) -> [BattleGridPoint; 4] {
    [
        BattleGridPoint::new(point.x + 1, point.y),
        BattleGridPoint::new(point.x - 1, point.y),
        BattleGridPoint::new(point.x, point.y + 1),
        BattleGridPoint::new(point.x, point.y - 1),
    ]
}

fn next_step_toward(
    seed: &BattleSeedV1,
    start: BattleGridPoint,
    target: BattleGridPoint,
    stop_range: i16,
    occupied: &BTreeSet<BattleGridPoint>,
) -> Option<BattleGridPoint> {
    if distance(start, target) <= stop_range {
        return None;
    }
    let mut queue = VecDeque::from([start]);
    let mut previous = BTreeMap::<BattleGridPoint, BattleGridPoint>::new();
    let mut visited = BTreeSet::from([start]);
    let mut reached = None;
    while let Some(current) = queue.pop_front() {
        if distance(current, target) <= stop_range {
            reached = Some(current);
            break;
        }
        for next in neighbors(current) {
            if !seed.map.passable(next)
                || (occupied.contains(&next) && next != target)
                || !visited.insert(next)
            {
                continue;
            }
            previous.insert(next, current);
            queue.push_back(next);
        }
    }
    let mut current = reached?;
    while let Some(parent) = previous.get(&current).copied() {
        if parent == start {
            return Some(current);
        }
        current = parent;
    }
    None
}

fn deterministic_yield_step(
    seed: &BattleSeedV1,
    start: BattleGridPoint,
    target: BattleGridPoint,
    occupied: &BTreeSet<BattleGridPoint>,
    reservations: &[TileReservation],
) -> Option<BattleGridPoint> {
    let mut candidates = neighbors(start)
        .into_iter()
        .filter(|candidate| {
            seed.map.passable(*candidate)
                && !occupied.contains(candidate)
                && !reservations
                    .iter()
                    .any(|reservation| reservation.tile == *candidate)
        })
        .collect::<Vec<_>>();
    candidates.sort_by_key(|candidate| (distance(*candidate, target), candidate.y, candidate.x));
    candidates.into_iter().next()
}

fn deterministic_evade(tick: u64, unit_index: usize, evasion_permille: u16) -> bool {
    ((tick.wrapping_mul(37) + unit_index as u64 * 101) % 1000) < evasion_permille as u64
}

fn simulation_salt(seed: &BattleSeedV1) -> u64 {
    if seed.skirmish.enabled {
        return seed.skirmish.simulation_seed % 997;
    }
    // Authored campaign battles retain their established deterministic
    // cadence. Only an explicitly configured skirmish seed is allowed to
    // perturb combat sampling.
    0
}

fn hash_json<T: Serialize>(value: &T) -> Result<String, SimError> {
    json_hash(value)
}

fn json_hash<T: Serialize>(value: &T) -> Result<String, SimError> {
    let bytes = serde_json::to_vec(value)?;
    let digest = Sha256::digest(bytes);
    Ok(digest.iter().map(|byte| format!("{byte:02x}")).collect())
}

