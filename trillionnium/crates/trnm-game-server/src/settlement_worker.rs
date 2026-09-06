// Directly compiled settlement implementation.
//
// `settlement_worker_legacy.rs` retains the reviewed capture/execute/apply
// primitives and historical single-loop entrypoint as migration evidence.
// `settlement_worker_runtime_v2.rs` owns the exported runtime, bounded shutdown,
// poison isolation, and migrations 18-19. Neither file is rewritten at build
// time; the only public entrypoint is runtime v2.
#[allow(dead_code)]
mod implementation {
    include!("settlement_worker_legacy.rs");
    include!("settlement_worker_runtime_v2.rs");
}

pub use implementation::{run_v2 as run, WorkerConfig};
