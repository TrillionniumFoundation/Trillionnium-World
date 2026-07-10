#![recursion_limit = "1024"]

pub use trnm_first_contact::{
    build_first_contact_live_bevy_app, default_first_contact_asset_root, ObserverAnswer,
    VisualAcceptance,
};

// Frozen compatibility/evidence surface. The default product path is the
// independent `trnm-first-contact` crate; legacy probes remain callable but no
// longer organize or compile into the player runner.
#[cfg(feature = "legacy")]
include!("legacy.rs");
