#![recursion_limit = "512"]

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
