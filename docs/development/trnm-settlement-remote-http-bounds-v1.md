---
status: source-candidate-rust-and-hosted-validation-pending
owner: trillionnium-world
work_items: [WORLD-P0-009, WORLD-P1-005, WORLD-P1-007]
last_reviewed: 2026-09-05
review_due: 2026-09-19
release_effect: none
---

# Settlement remote HTTP response policy v1

## Scope and source identity

This four-file tranche is based on PR #46 at
`06b19626bfa7d65128d85140919f6f7c3408f9f7`. The CEX adapter preimage is Git blob
`4b8c9d6618d8eff167a5048192f6431e0427a6f0`. Unlike the earlier local repair
packet, it applies directly to the current ordinary `src/cex.rs`; it does not
require publishing the entire v13k artifact first. The game-server semantic
library generator and full 73-write/two-deletion import remain separate open
work. This tranche must survive that later import as an explicitly requalified
successor, not be overwritten or credited using the original artifact hashes.

The changes affect the existing CEX/signer client, its private tests and this
module contract. There is no dependency, SQL, intent/signature/hash encoding,
authority, source-generation, workflow, component-lock or release-flag change.

## Transport boundary

The single `CexClient` HTTP client uses `reqwest::redirect::Policy::none()`.
Neither another-origin nor same-origin redirects may forward game-authority,
signer or player-session headers. Operators must configure the final authorized
endpoint; a redirect is not an endpoint-discovery mechanism. Existing 3-second
connect and 10-second total request timeouts remain unchanged. No automatic
retry is added here.

All eight typed successful responses use `bounded_remote_json`: signer
readiness, signer attestation, issuer-key registry, player-session verification,
signer receipt lookup, signer receipt creation, CEX receipt lookup and CEX
intent submission. CEX readiness consumes the status only and does not buffer
its body. Request serialization and the existing 64 KiB error-body reader are
unchanged. This work does not certify repository-wide logging redaction.

## Byte and decoding contract

The success-body accumulator is limited to 2,097,152 bytes. A known larger
Content-Length is rejected before accumulation. Every received chunk is checked
with overflow-safe addition before reservation or append, including responses
with no known length. An exact-limit body is allowed. A refused append leaves
the previously accumulated bytes unchanged.

Decoding occurs only after EOF. Oversized data is rejected, never truncated into
a valid-looking receipt. A valid JSON prefix followed by an oversized whitespace
tail also fails. A malformed, trailing, empty or invalid-UTF-8 document produces
a static diagnostic without reflecting peer data.

Static helper errors are `remote_success_body_too_large`,
`remote_success_body_allocation_failed`, `remote_success_body_read_failed`, and
`remote_success_json_invalid`. This bounds accumulated body length, not every
HTTP-stack buffer, allocator capacity or decoded Rust object. Larger legitimate
receipts require a separately reviewed policy, not silent fallback.

## Ambiguity versus identity rejection

At settlement boundaries, 3xx status, response-size failure, timeout and malformed
successful responses remain retryable *unknown outcomes*. None becomes 404,
`None`, proof of no remote effect, or a local success. Lookup failure returns
before a submit can start. Retrying still uses the existing immutable intent and
remote request identity and performs lookup before submit.

After a complete body decodes, all existing contract, intent-hash, account,
receipt and signer bindings still validate. A mismatched immutable receipt hash
remains **Permanent**. It is not a transport ambiguity and must not be loosened
into a retry loop. Readiness/session failures retain their existing fail-closed
caller behavior. Retry budgets, leases, quarantine and campaign application
remain owned by their existing runtime/database layers.

## Regression tests

`src/cex_response_policy_tests.rs` is an ordinary test-only module of `cex`, also
included when the existing worker binary compiles that module. Its twelve test
functions exercise the actual private adapter and loopback HTTP, not a Python
simulation. Parameterized iterations are not separate test functions.

Coverage includes exact/over-limit known and chunked bodies; accumulator error
preservation; malformed/trailing/empty/invalid-UTF-8 JSON; oversized tails; body
timeout; 301/302/303/307/308 without a connection to the redirect destination;
relative redirects; oversized signer lookup; malformed and oversized CEX lookup
without submit; and oversized post-commit success followed by exact receipt
lookup with one total submit. Test servers have abort-on-drop handles.

The pre-existing mismatched-hash Permanent assertion and signer/CEX response-loss
tests are retained. Two contradictory assertions in the earlier unexecuted
local packet were corrected before this tranche: that packet incorrectly made
the old mismatch test expect Retryable and the new redirect test expect
Permanent. Do not blindly apply the superseded packet.

## Validation and promotion

The preparation environment has no Cargo/Rust or PostgreSQL executable. These
Rust tests are authored, **not run or passed**. Local patch/hash checks and
rerunning the independent Python transition suite cannot substitute for them.
The local checkout is a labelled qualified-artifact reconstruction with exact
relevant remote preimages, not a full live-head checkout.

Run in a complete unchanged candidate:

```bash
cargo fmt --manifest-path trillionnium/Cargo.toml --all -- --check
cargo test --manifest-path trillionnium/Cargo.toml -p trnm-game-server --all-targets --locked
cargo clippy --manifest-path trillionnium/Cargo.toml -p trnm-game-server --all-targets --locked -- -D warnings
```

Also require PostgreSQL/fault, package, supply-chain and prospective-merge gates,
independent review, and CEX/Integration compatibility before promotion. The old
Rust 1.98.0 artifact and any reviewed toolchain successor have separate evidence
identities. No public-online, custody, market, deployment or production authority
is granted. Rollback must preserve no-redirect and immutable-identity guarantees;
never relax them merely to satisfy an old fixture or restore transport reachability.
