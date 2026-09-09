//! Production adapter contracts and the fail-closed composition root.
//!
//! `base` retains the stable adapter vocabulary. `composition` requires all
//! nine production roles and exact-object qualification evidence before a
//! runtime assembly can be constructed. A valid assembly is still not a
//! production release authorization.

pub(crate) use crate::{
    WorldAccountAuthDecision, WorldActorIdentity, WorldEvidenceReceipt, WorldLedgerReceipt,
    WorldMetricReceipt, WorldRepositoryReceipt, WorldSessionDecision,
};

#[path = "../production_contract_base.rs"]
mod base;
pub use base::*;

mod composition;
pub use composition::*;
