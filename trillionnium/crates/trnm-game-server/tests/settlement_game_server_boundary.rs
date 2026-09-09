mod support;

use support::{read_crate_source_bundle, read_settlement_worker_bundle};

#[test]
fn game_server_does_not_execute_terminal_economy_settlement() {
    let source = read_crate_source_bundle("src/lib.rs");
    assert!(!source.contains("trnm_game_server_lib_generated.rs"));
    assert!(!source.contains("OUT_DIR"));
    assert!(!source.contains("reconcile_economy(&state.cex"));
    assert!(!source.contains("settle_pending_matches(&settlement_state"));
    assert!(source.contains(
        "terminal settlement is owned by trnm-settlement-worker; in-process settlement is prohibited"
    ));
}

#[test]
fn both_runtime_entrypoints_register_the_complete_settlement_migration_chain() {
    let game_server = read_crate_source_bundle("src/lib.rs");
    let worker = read_settlement_worker_bundle();
    let worker_entry = read_crate_source_bundle("src/settlement_worker.rs");
    assert!(!worker_entry.contains("OUT_DIR"));
    assert!(!worker_entry.contains("trnm_settlement_worker_generated.rs"));
    assert!(worker_entry.contains("settlement_worker_legacy.rs"));
    assert!(worker_entry.contains("settlement_worker_runtime_v2.rs"));
    for marker in [
        "0016_online_settlement_outbox_v1",
        "0017_online_settlement_worker_runtime_v1",
        "0018_online_settlement_operator_controls_v1",
        "0019_online_settlement_quarantine_v1",
    ] {
        assert!(
            game_server.contains(marker),
            "direct game server lost {marker}"
        );
        assert!(
            worker.contains(marker),
            "direct settlement worker lost {marker}"
        );
    }
}

#[test]
fn direct_sources_never_restore_generated_authority() {
    for source in [
        read_crate_source_bundle("src/lib.rs"),
        read_settlement_worker_bundle(),
    ] {
        assert!(!source.contains("OUT_DIR"));
        assert!(!source.contains("trnm_game_server_lib_generated.rs"));
        assert!(!source.contains("trnm_settlement_worker_generated.rs"));
        assert!(!source.contains("src/lib.rs.in"));
        assert!(!source.contains("settlement_worker.rs.in"));
    }
}
