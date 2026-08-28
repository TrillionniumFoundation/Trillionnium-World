// The reviewed worker body is generated from src/settlement_worker.rs.in by
// build.rs. The transform fails closed on drift and registers migrations
// 0016_online_settlement_outbox_v1, 0017_online_settlement_worker_runtime_v1,
// and 0018_online_settlement_operator_controls_v1 before remote work starts.
include!(concat!(env!("OUT_DIR"), "/trnm_settlement_worker_generated.rs"));
