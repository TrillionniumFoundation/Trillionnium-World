mod support;

use support::read_crate_source_bundle;

#[test]
fn game_server_partition_is_hash_bound_and_non_generated() {
    let source = read_crate_source_bundle("src/lib.rs");
    assert!(source.contains("Ownership section: authority_foundation"));
    assert!(source.contains("Ownership section: campaign_persistence"));
    assert!(!source.contains("trnm_game_server_lib_generated.rs"));
    assert!(!source.contains("src/lib.rs.in"));
}
