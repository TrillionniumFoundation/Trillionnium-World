const GAME_SERVER_ENTRYPOINT: &str = include_str!("../src/lib.rs");
const SETTLEMENT_WORKER_ENTRYPOINT: &str = include_str!("../src/settlement_worker.rs");
const GENERATED_GAME_SERVER_SOURCE: &str = include_str!(concat!(
    env!("OUT_DIR"),
    "/trnm_game_server_lib_generated.rs"
));
const GENERATED_SETTLEMENT_WORKER_SOURCE: &str = include_str!(concat!(
    env!("OUT_DIR"),
    "/trnm_settlement_worker_generated.rs"
));

#[test]
fn game_server_does_not_execute_terminal_economy_settlement() {
    assert!(GAME_SERVER_ENTRYPOINT.contains("trnm_game_server_lib_generated.rs"));
    assert!(!GENERATED_GAME_SERVER_SOURCE.contains("reconcile_economy(&state.cex"));
    assert!(!GENERATED_GAME_SERVER_SOURCE.contains("settle_pending_matches(&settlement_state"));
    assert!(GENERATED_GAME_SERVER_SOURCE.contains(
        "terminal settlement is owned by trnm-settlement-worker; in-process settlement is prohibited"
    ));
}

#[test]
fn both_runtime_entrypoints_register_the_complete_settlement_migration_chain() {
    assert!(SETTLEMENT_WORKER_ENTRYPOINT.contains("trnm_settlement_worker_generated.rs"));
    for marker in [
        "0016_online_settlement_outbox_v1",
        "0017_online_settlement_worker_runtime_v1",
        "0018_online_settlement_operator_controls_v1",
    ] {
        assert!(
            GENERATED_GAME_SERVER_SOURCE.contains(marker),
            "generated game server lost {marker}"
        );
        assert!(
            GENERATED_SETTLEMENT_WORKER_SOURCE.contains(marker),
            "generated settlement worker lost {marker}"
        );
    }
}
