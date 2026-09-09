//! Deterministic, Bevy-free First Contact battle simulation.
//!
//! The simulation consumes validated [`RtsFrameOrder`] values as its only
//! player input. The authored map projection embedded in [`BattleSeedV1`]
//! drives two-dimensional pathfinding, combat, resources and objectives.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fmt, fs,
    io::Write,
    path::{Path, PathBuf},
};
use trnm_campaign_core::{
    BattleGridPoint, BattleOutcome, BattleResultV1, BattleSeedV1, CampaignDifficulty,
    CampaignError, LootStack, ObjectiveKind, SkirmishVictoryMode, UnitBattleReportV1,
    UnitBattleStatus, BATTLE_RESULT_CONTRACT,
};
use trnm_rts_protocol::{RtsFrameOrder, RtsOrderKind, RtsUnitStance};

mod content;
pub use content::*;

// Correctness-critical implementation is partitioned by ownership. Every
// included file is ordinary reviewed source; no build script rewrites runtime
// semantics and no generated Rust source participates in compilation.

// Ownership section: contracts_and_primitives. Ordinary Git-tracked source.
include!("lib_parts/contracts_and_primitives/part_01.rs");

// Ownership section: mission_runtime. Ordinary Git-tracked source.
include!("lib_parts/mission_runtime/part_01.rs");

// Ownership section: mission_runtime. Ordinary Git-tracked source.
include!("lib_parts/mission_runtime/part_02.rs");

// Ownership section: mission_runtime. Ordinary Git-tracked source.
include!("lib_parts/mission_runtime/part_03.rs");

// Ownership section: mission_runtime. Ordinary Git-tracked source.
include!("lib_parts/mission_runtime/part_04.rs");

// Ownership section: simulation_helpers. Ordinary Git-tracked source.
include!("lib_parts/simulation_helpers/part_01.rs");

// Ownership section: replay. Ordinary Git-tracked source.
include!("lib_parts/replay/part_01.rs");

// Ownership section: checkpoint_storage. Ordinary Git-tracked source.
include!("lib_parts/checkpoint_storage/part_01.rs");

// Ownership section: tests. Ordinary Git-tracked source.
include!("lib_parts/tests/part_01.rs");
