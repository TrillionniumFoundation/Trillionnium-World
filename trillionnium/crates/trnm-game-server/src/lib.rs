#![recursion_limit = "512"]

// Async CEX transport is invoked only by the independently deployed settlement worker.
// The game-server library retains the shared type boundary but never calls these APIs.
#[rustfmt::skip]
#[allow(dead_code)]
mod cex;
mod map;
mod operations_v1;
mod product_v2;
mod production_v1;
mod published_tick_journal;
pub mod signer_protocol;
mod stream;

// The full reviewed server body is generated from src/lib.rs.in by build.rs.
// The transform fails closed on source drift, removes the in-process settlement
// loop/caller, and registers settlement migrations 16 through 18.
include!(concat!(
    env!("OUT_DIR"),
    "/trnm_game_server_lib_generated.rs"
));
