// The reviewed worker body is generated from src/settlement_worker.rs.in by
// build.rs. The transform fails closed on drift and registers migrations
// 0016 through 0019 before remote work starts. The legacy unbounded run loop is
// compiled only as disabled migration evidence; runtime ownership is below.
include!(concat!(
    env!("OUT_DIR"),
    "/trnm_settlement_worker_generated.rs"
));
include!("settlement_worker_runtime_v2.rs");
