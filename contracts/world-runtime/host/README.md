# TRNM World Runtime Host v1

Status: **source implemented / pending remote Rust checks**  
Owner: Trillionnium World deterministic game domain  
Consumers: Trillionnium Nakama and Trillionnium Integration

This package exposes a Bevy-free execution host and a strict shadow comparator
for `trnm_world_runtime_v1`.

It intentionally has no HTTP server, socket client, database driver, signer,
participant session, canonical event-order, Chain-finality or CEX-custody
capability.

## Binaries

### `trnm-world-runtime-exec`

Reads one strict execute request from an explicit file or stdin and selects one
exact installed ruleset/content tuple:

```bash
cargo run --manifest-path contracts/world-runtime/host/Cargo.toml \
  --bin trnm-world-runtime-exec -- \
  --ruleset-id first-contact \
  --ruleset-version 1 \
  --ruleset-digest "$RULESET_SHA256" \
  --content-digest "$CONTENT_SHA256" \
  --input request.json
```

Raw mode emits an execute result or `trnm_world_runtime_error_v1`. A
deterministic rejection exits with status `2`.

Observation mode binds the exact request, implementation revision, response,
canonical response byte count and wall-clock evidence measurement:

```bash
cargo run --manifest-path contracts/world-runtime/host/Cargo.toml \
  --bin trnm-world-runtime-exec -- \
  --ruleset-id first-contact \
  --ruleset-version 1 \
  --ruleset-digest "$RULESET_SHA256" \
  --content-digest "$CONTENT_SHA256" \
  --input request.json \
  --observe \
  --implementation-id world-rust \
  --implementation-revision "$WORLD_COMMIT"
```

Observation mode exits `0` for both a deterministic success and a deterministic
rejection. This lets a shadow comparison preserve rejection parity rather than
treating every domain rejection as infrastructure failure.

### `trnm-world-runtime-shadow-diff`

Reads one `trnm_world_shadow_input_v1` packet containing a World observation,
an independent candidate observation and explicit resource budgets:

```bash
cargo run --manifest-path contracts/world-runtime/host/Cargo.toml \
  --bin trnm-world-runtime-shadow-diff -- \
  --input shadow-input.json > shadow-report.json
```

Exit status:

- `0`: equivalent deterministic game-domain observations;
- `1`: one or more typed divergences;
- `64`: invalid input, invalid claimed response/hash, forbidden authority
  material or invalid host configuration.

## Comparison contract

Before comparing two observations, the host independently verifies:

- exact observation and response shapes;
- exact implementation commit identities;
- request-hash equality;
- canonical response byte counts;
- result material hashes;
- absence of forbidden authority fields;
- positive candidate duration/response-size budgets.

For successful executions it compares ruleset/content/input bindings, final
state, outcome and replay material both by value and by hash. For deterministic
rejections it compares error code and recoverability while allowing diagnostic
message wording to differ.

A copied hash over modified material is rejected as an invalid observation
before the comparison result is produced.

## Non-claims

A green shadow report does not prove:

- authenticated participants;
- global ordering or command idempotency;
- canonical roster/event/archive roots;
- `MatchCompletedV1` construction or signature;
- Chain inclusion/finality;
- CEX wallet settlement;
- active-match takeover;
- public-online or commercial readiness.

Those remain owned by Nakama, Chain, CEX and Integration as defined by the
current authority ADR and development plan.
