use std::path::PathBuf;

const CEX_SOURCE: &str = include_str!("../src/cex.rs");
const BUILD_SCRIPT: &str = include_str!("../build.rs");
const CARGO_MANIFEST: &str = include_str!("../Cargo.toml");

#[test]
fn cex_transport_is_directly_compiled_reviewed_source() {
    assert!(CEX_SOURCE.contains("pub struct CexClient"));
    assert!(CEX_SOURCE.contains("impl CexClient"));
    assert!(CEX_SOURCE.contains("impl EconomyBackend for CexClient"));
    assert!(!CEX_SOURCE.contains("OUT_DIR"));
    assert!(!CEX_SOURCE.contains("trnm_cex_generated.rs"));
    assert!(!CEX_SOURCE.contains("include!("));

    assert!(!BUILD_SCRIPT.contains("generate_cex"));
    assert!(!BUILD_SCRIPT.contains("src/cex.rs.in"));
    assert!(!BUILD_SCRIPT.contains("trnm_cex_generated.rs"));

    let template = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/cex.rs.in");
    assert!(!template.exists(), "CEX template authority must stay removed");
}

#[test]
fn direct_cex_source_fails_closed_on_ambiguous_remote_outcomes() {
    for marker in [
        "StatusCode::CONFLICT",
        "MAX_REMOTE_ERROR_BODY_BYTES",
        "bounded_error_body",
        "decode isolated signer response after possible commit",
        "decode isolated signer receipt lookup after success status",
        "decode CEX receipt lookup after success status",
        "decode CEX receipt after possible commit",
        "lookup_signer_receipt",
        "lookup_authorized_settlement_receipt",
    ] {
        assert!(CEX_SOURCE.contains(marker), "missing direct CEX control: {marker}");
    }
}

#[test]
fn direct_cex_source_never_restores_blocking_transport() {
    assert!(!CEX_SOURCE.contains("reqwest::blocking"));
    assert!(!CEX_SOURCE.contains("blocking_client"));
    assert!(!CARGO_MANIFEST.contains("\"blocking\""));
    assert!(CEX_SOURCE.contains("synchronous EconomyBackend I/O is prohibited"));
}
