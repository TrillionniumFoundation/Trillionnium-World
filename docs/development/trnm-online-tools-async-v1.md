---
status: source-implemented-unverified
owner: trillionnium-world
work_items:
  - WORLD-P0-009
  - WORLD-P1-001
last_reviewed: 2026-09-05
review_due: 2026-09-19
release_effect: none
---

# Online tool async HTTP contract v1

## Scope and source ownership

This change removes the two `reqwest::blocking` consumers compiled as binaries
of `trnm-game-server`: `trnm-online-e2e` and `trnm-moderation-console`. Both
remain ordinary source under `trillionnium/crates/trnm-game-server/src/bin`.
Their binary names, command-line arguments, environment variables, request
schemas, response types and packaging entry points are unchanged. No binary is
feature-gated away and no existing test is skipped to make a build pass.

The library, settlement worker, signer, database transactions and online
authority profile are not changed by this port. This document does not authorize
running a moderation action or an E2E scenario against a deployment.

## Why removing a Cargo feature alone is incorrect

The live PR manifest omits reqwest's `blocking` feature, but the original two
binary sources import that feature-gated namespace. The retained v13k source
artifact re-enabled the feature to compile those tools, contradicting the
service dependency policy. Removing the feature without fixing its callers
leaves an all-target compilation gap.

The tools now use the existing async `reqwest::Client`, `RequestBuilder` and
`Response` APIs. Every request send and response-body read is awaited. No new
HTTP package, hidden synchronous wrapper, `Runtime::block_on` bridge or optional
blocking feature is introduced.

The native client is a different runtime surface and still declares its own
blocking feature. Cargo can unify features when packages are built together.
Therefore qualification must compile/test the game-server targets separately
from `trnm-first-contact`; a workspace-wide build that enables blocking through
the native client is not proof of the server's nonblocking feature profile.

## E2E process and concurrency model

`trnm-online-e2e` enters an explicit multi-thread Tokio runtime with two workers.
Its HTTP call chain, polling, server-restart subprocess and retry delays are
async. Four existing parallel HTTP submission/reconnect sites use Tokio tasks;
join results are awaited. The two-participant command/reconnect rendezvous uses
`tokio::sync::Barrier`, not a thread-blocking barrier. The existing 32 race
rounds, pipeline depth of four, request identities and ordering assertions are
retained. The short metrics mutex never spans an await.

The old tungstenite smoke test remains synchronous. Its connect, stream-read
and close operations are isolated with `tokio::task::block_in_place` at the four
existing call sites. This requires the explicitly multi-thread runtime; do not
move this program into a current-thread runtime without migrating the socket
path. These socket operations are not async HTTP or new server authority.
Existing socket timing limits remain; this port does not establish a new
cancellation or total-connect-time guarantee for the legacy WebSocket path.

## HTTP and retry semantics

| Operation | Preserved behavior |
| --- | --- |
| Idempotent HTTP transport | Clone the same request; at most the first attempt plus three transport retries; async backoff of 100, 200 and 300 milliseconds. |
| Simulated lost successful response | Read/discard the first success, then resend the exact cloned request; retain the same command/intent identity. |
| HTTP error status | Return the actual response; do not reinterpret a 409 or other HTTP failure as a transport failure. |
| Create and join | One non-idempotent request with the existing 30-second timeout; no generic retry helper. |
| Regular requests | Preserve the two-second connect and four-second request timeout. |
| Snapshot/reconnect publication barrier | Retry only 503 with `recoverable=true`, under the existing 10-second polling window and one-second interval. |
| JSON decoding failure | Remains an error, not a successfully decoded receipt. |

These are tool-side request rules. They do not prove exactly-once settlement,
durable authority recovery or the correctness of a remote server.

## Moderation console

The console uses a current-thread Tokio runtime because its ten command paths
are sequential async HTTP operations. Each send and response text read is
awaited. The existing moderator header, role-specific token input, two-second
connect timeout, ten-second request timeout, exit-code behavior and typed
responses are retained. No automatic retry is added to mutation commands.
No operator action is executed by the source port or its qualification tests.

## Regression and qualification requirements

Seven loopback Rust regressions are included through
`online_e2e/async_http_tests.rs` in the E2E binary's test target. They exercise
the actual HTTP helpers, not a separate Python HTTP model:

- successful responses and HTTP conflicts are not transport-retried;
- simulated lost success and transport response loss preserve request bytes;
- exhausted transport retries stop after four attempts;
- non-idempotent creation does not retry a lost response;
- malformed successful JSON is not reported as a decoded receipt.

Each fixture binds an ephemeral loopback address, uses bounded reads/writes,
closes each response connection, and watches for unwanted extra requests.
Fixture clients disable environment proxy discovery. The fixtures contain only
synthetic identities; they neither contact production nor mutate a ledger.
The pre-existing three E2E unit tests remain in place.

Required actual Rust execution on one exact candidate:

```bash
cargo fmt --manifest-path trillionnium/Cargo.toml --all -- --check
cargo test --manifest-path trillionnium/Cargo.toml -p trnm-game-server \
  --bin trnm-online-e2e --bin trnm-moderation-console --locked
cargo clippy --manifest-path trillionnium/Cargo.toml -p trnm-game-server \
  --all-targets --locked -- -D warnings
cargo build --manifest-path trillionnium/Cargo.toml -p trnm-game-server \
  --all-targets --release --locked
```

The existing final-head workspace lane already selects the game-server package
with `--all-targets`, so the new included tests participate without a duplicate
workflow or source-writing CI action. A source check or lexical rewrite audit
is not compilation, formatting, task-scheduling or network-test evidence.

## Immutable artifact and rollback boundary

The original v13k ZIP, its member digests and qualified tree stay immutable.
The two tool files and their tests do not overlap its 73 write paths. A separate
successor-source overlay may remove its now-unused `blocking` feature after
these callers are fixed. That successor has different bytes and needs its own
exact-head qualification and review; it cannot inherit the historical artifact
result. The importer must continue to reject unexplained overlapping changes.

Rollback must keep callers and their dependency contract consistent. Restoring
old blocking imports while keeping the current no-blocking manifest recreates
the compilation gap. A failed qualification blocks admission; it is not a
reason to restore blocking HTTP in the service dependency profile.

This source candidate grants no public-online, custody, cross-host, independent
review, human/accessibility or commercial-release credit. Production
authorization remains `not_granted`.
