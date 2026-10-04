// Generated TEST DATA from docs/protocol/vectors/trnm-world-transition-negative-v1.json.
// Regenerate with: python3 scripts/check-trnm-world-transition-conformance.py --print-negative-fixture
// No production source or runtime semantics are generated.
#[rustfmt::skip]
pub(super) const SOURCE_SHA256: &str = "7d55621a07f59ccba92b9f92e7dd215cc34742405d9be1556a2d98e97249029f";
#[rustfmt::skip]
pub(super) const CASES: &[(&str, &[u8])] = &[
    ("empty", b""),
    ("scalar_root", b"1"),
    ("missing_value", b"{\"a\":}"),
    ("trailing_comma_object", b"{\"a\":1,}"),
    ("trailing_comma_array", b"[1,]"),
    ("unsorted_keys", b"{\"b\":1,\"a\":2}"),
    ("duplicate_key", b"{\"a\":1,\"a\":2}"),
    ("leading_zero", b"[01]"),
    ("negative_zero", b"[-0]"),
    ("float", b"[1.0]"),
    ("exponent", b"[1e3]"),
    ("nan", b"[NaN]"),
    ("positive_i64_overflow", b"[9223372036854775808]"),
    ("negative_i64_overflow", b"[-9223372036854775809]"),
    ("nonminimal_slash_escape", b"{\"a\":\"\\/\"}"),
    ("nonminimal_unicode_escape", b"{\"a\":\"\\u0061\"}"),
    ("uppercase_unicode_hex", b"{\"a\":\"\\u001F\"}"),
    ("insignificant_whitespace", b"{ \"a\":1}"),
    ("trailing_bytes", b"{}x"),
    ("escaped_authority_key", b"{\"nakama_\\u0070rivate_key\":\"x\"}"),
    ("nested_authority_key", b"{\"a\":{\"match_completed_v1\":{}}}"),
    ("case_folded_nakama_key", b"{\"Nakama_Private_Key\":\"x\"}"),
    ("case_folded_completion_key", b"{\"MATCH_COMPLETED_V1\":{}}"),
    ("nested_case_folded_chain_key", b"{\"a\":{\"Chain_App_Hash\":\"x\"}}"),
];
