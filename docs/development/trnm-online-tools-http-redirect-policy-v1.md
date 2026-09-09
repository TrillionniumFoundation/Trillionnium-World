---
status: source-implemented-pending-rust-validation
owner: trillionnium-world
work_items:
  - WORLD-P1-005
  - WORLD-P1-007
last_reviewed: 2026-09-05
review_due: 2026-09-19
release_effect: none
---

# Credentialed CLI HTTP redirect policy v1

## Defect, scope and implementation

At input commit `995fd2c2865edb9633ec516207ce7bf3201b0199`, the async
`trnm-online-e2e` and `trnm-moderation-console` HTTP clients used reqwest's
default redirect policy. They attach `x-trnm-player-session` and
`x-trnm-moderator`, respectively. The locked reqwest 0.12.28 implementation
follows redirects by default and removes a fixed set of standard authentication
headers on a host/port change; that set does not contain these custom headers.
Consequently a redirect can move a credentialed request outside its intended
endpoint, and a 307/308 can forward its body. This is a source-level finding,
not a claim that a deployed credential was leaked.

Primary implementation reference:
https://github.com/seanmonstar/reqwest/blob/v0.12.28/src/redirect.rs

Both tools now construct credentialed clients through the ordinary shared module
`trillionnium/crates/trnm-game-server/src/bin/http_policy/mod.rs`.
`client_builder(connect_timeout, request_timeout)` returns an async builder
with `reqwest::redirect::Policy::none()` and the caller's existing timeouts.
The normal E2E client retains its two-second connect/four-second request budget;
the non-idempotent create/join client retains two/thirty seconds; the console
retains two/ten seconds. The uncredentialed restart-readiness polling client
is not part of this change.

No crate, feature, dependency version, manifest, workflow, command-line argument,
credential value, server-side authorization rule or release flag is changed.
The module's directory contains no `main.rs`, and it is included explicitly by
the existing two binary targets; it is not a new executable or a build-generated
implementation. These paths do not overlap the immutable v13k 73 writes/two
deletions. Source publication of that historical set is a separate blocker.

## Behavior and compatibility

All redirects, including relative and same-origin redirects, are stopped.
The original 3xx HTTP response is returned to the existing caller. It is not a
transport error, not a decoded success and not permission to resend a mutation
at the Location target. Existing HTTP status handling and explicit retry
identities remain unchanged. The existing lost-success retry branch applies
only to successful status codes and therefore cannot turn a 3xx into a retry.

Operators must configure the final authorized service endpoint instead of
relying on a redirect to relocate it. This intentionally changes deployments
that previously depended on HTTP redirection; changing server endpoint identity
requires operator review, not automatic credential forwarding. It does not
add URL allowlisting, mTLS, token rotation/revocation, error-body redaction or
response-allocation bounds. Those controls, the signer/CEX client surfaces,
and other native/tool HTTP clients retain their independent review requirements.

## Regression source and execution requirements

Seven shared Rust test functions exercise the actual builder over ephemeral
IPv4 loopback sockets:

| Case | Required result |
| --- | --- |
| 301 and 302 | Original status preserved; neither origin nor target receives a follow-up request. |
| 303 | POST is not rewritten into a credentialed GET at another endpoint. |
| 307 and 308 | Neither credential headers nor the request body are replayed to the target. |
| Relative 307 | A same-origin path cannot silently relocate the mutation. |
| Direct 200 | Original method, synthetic credential headers and body still reach the selected endpoint. |

Fixtures use only synthetic values, bounded request reads, bounded socket I/O,
separate origin/target listeners and no environment proxy discovery. Observer
accept errors fail tests instead of being treated as absence of forwarding.
The source is compiled into both binary test targets, so seven unique test
functions would execute twice. This is not fourteen unique regression designs.
Existing E2E helper tests are retained unchanged.

Run on an exact World candidate with its selected toolchain:

```bash
cargo fmt --manifest-path trillionnium/Cargo.toml --all -- --check
cargo test --manifest-path trillionnium/Cargo.toml -p trnm-game-server \
  --bin trnm-online-e2e --bin trnm-moderation-console --locked
cargo clippy --manifest-path trillionnium/Cargo.toml -p trnm-game-server \
  --all-targets --locked -- -D warnings
```

At authoring, no Rust compiler, rustfmt or Clippy is available in the execution
container; these Rust tests have NOT run. Static source checks and patch
application cannot supply their results. Current-head and prospective-merge
hosted qualification, real role-isolation/mTLS/rotation evidence and independent
security review remain required. In particular WORLD-P1-005 and WORLD-P1-007
are not closed by this source repair.

## Rollback and evidence boundary

Keep the shared module and both call sites together. Removing only the module
breaks compilation; restoring default redirect behavior reopens the documented
credential-forwarding path. Do not accept such a rollback as a qualification
fix. No local test, PR comment or source flag grants trusted settlement,
public-online, public-market, custody, legal/commercial or production approval.
