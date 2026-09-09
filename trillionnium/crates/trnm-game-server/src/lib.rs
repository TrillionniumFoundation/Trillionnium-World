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

// Correctness-critical implementation is partitioned by ownership. Every
// included file is ordinary reviewed source; no build script rewrites runtime
// semantics and no generated Rust source participates in compilation.

// Ownership section: authority_foundation. Ordinary Git-tracked source.
include!("lib_parts/authority_foundation/part_01.rs");

// Ownership section: configuration_and_migrations. Ordinary Git-tracked source.
include!("lib_parts/configuration_and_migrations/part_01.rs");

// Ownership section: terminal_recovery. Ordinary Git-tracked source.
include!("lib_parts/terminal_recovery/part_01.rs");

// Ownership section: terminal_recovery. Ordinary Git-tracked source.
include!("lib_parts/terminal_recovery/part_02.rs");

// Ownership section: terminal_recovery. Ordinary Git-tracked source.
include!("lib_parts/terminal_recovery/part_03.rs");

// Ownership section: operations_boundary. Ordinary Git-tracked source.
include!("lib_parts/operations_boundary/part_01.rs");

// Ownership section: fleet_fencing. Ordinary Git-tracked source.
include!("lib_parts/fleet_fencing/part_01.rs");

// Ownership section: identity. Ordinary Git-tracked source.
include!("lib_parts/identity/part_01.rs");

// Ownership section: application. Ordinary Git-tracked source.
include!("lib_parts/application/part_01.rs");

// Ownership section: http_routing. Ordinary Git-tracked source.
include!("lib_parts/http_routing/part_01.rs");

// Ownership section: readiness. Ordinary Git-tracked source.
include!("lib_parts/readiness/part_01.rs");

// Ownership section: product_api. Ordinary Git-tracked source.
include!("lib_parts/product_api/part_01.rs");

// Ownership section: product_api. Ordinary Git-tracked source.
include!("lib_parts/product_api/part_02.rs");

// Ownership section: actor_runtime. Ordinary Git-tracked source.
include!("lib_parts/actor_runtime/part_01.rs");

// Ownership section: actor_runtime. Ordinary Git-tracked source.
include!("lib_parts/actor_runtime/part_02.rs");

// Ownership section: actor_runtime. Ordinary Git-tracked source.
include!("lib_parts/actor_runtime/part_03.rs");

// Ownership section: campaign_persistence. Ordinary Git-tracked source.
include!("lib_parts/campaign_persistence/part_01.rs");

// Ownership section: tests. Ordinary Git-tracked source.
include!("lib_parts/tests/part_01.rs");
