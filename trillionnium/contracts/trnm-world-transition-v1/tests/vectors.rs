use trnm_world_transition_contract::{
    CanonicalPayloadV1, WorldCommandV1, WorldTransitionRequestV1,
    MAX_COMMAND_PAYLOAD_BYTES, MAX_STATE_PAYLOAD_BYTES,
};

const EMPTY_HASH: &str =
    "44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a";
const REQUEST_HASH: &str =
    "134c0257d4b51fe1061779a84680900726034a7b333fc20410bd3575f738c130";

fn request() -> WorldTransitionRequestV1 {
    WorldTransitionRequestV1::new(
        "transition-0001",
        "ruleset-v1",
        "content-v1",
        7,
        CanonicalPayloadV1::new("state-v1", "{}", MAX_STATE_PAYLOAD_BYTES).unwrap(),
        WorldCommandV1::new(
            "command-0001",
            CanonicalPayloadV1::new("command-v1", "{}", MAX_COMMAND_PAYLOAD_BYTES).unwrap(),
        )
        .unwrap(),
    )
    .unwrap()
}

#[test]
fn request_matches_published_canonical_bytes_and_hash() {
    let request = request();
    assert_eq!(request.previous_state.sha256, EMPTY_HASH);
    assert_eq!(
        request.to_canonical_json().unwrap(),
        format!(
            "{{\"command\":{{\"command_id\":\"command-0001\",\"payload\":{{\"canonical_json\":{{}},\"schema_id\":\"command-v1\",\"sha256\":\"{EMPTY_HASH}\"}}}},\"content_revision\":\"content-v1\",\"contract_version\":\"trnm_world_transition_v1\",\"expected_tick\":7,\"previous_state\":{{\"canonical_json\":{{}},\"schema_id\":\"state-v1\",\"sha256\":\"{EMPTY_HASH}\"}},\"ruleset_revision\":\"ruleset-v1\",\"transition_id\":\"transition-0001\"}}"
        )
    );
    assert_eq!(request.request_hash().unwrap(), REQUEST_HASH);
}

#[test]
fn published_negative_corpus_classes_fail_closed() {
    for raw in [
        "",
        "1",
        "{\"a\":}",
        "{\"a\":1,}",
        "[1,]",
        "{\"b\":1,\"a\":2}",
        "{\"a\":1,\"a\":2}",
        "[01]",
        "[-0]",
        "[1.0]",
        "[1e3]",
        "[NaN]",
        "[9223372036854775808]",
        "[-9223372036854775809]",
        "{\"a\":\"\\/\"}",
        "{\"a\":\"\\u0061\"}",
        "{\"a\":\"\\u001F\"}",
        "{ \"a\":1}",
        "{}x",
        "{\"nakama_\\u0070rivate_key\":\"x\"}",
        "{\"a\":{\"match_completed_v1\":{}}}",
    ] {
        assert!(
            CanonicalPayloadV1::new("negative-v1", raw, 4096).is_err(),
            "unexpected pass for {raw:?}"
        );
    }
}
