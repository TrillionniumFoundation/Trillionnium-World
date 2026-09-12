# trnm-world-command

Status: current cutover candidate  
Owner: Trillionnium World deterministic command decisions

## Purpose

This crate defines typed World-domain commands, stable decisions and rejection reasons, movement/transition resolution, and tactics/task/item/skill state transitions over a `WorldState` candidate.

## Authority and non-goals

It decides deterministic World business semantics only. It does not authenticate a caller, assign the canonical online order, persist idempotency, manage reconnect generations, hold database locks, perform remote I/O, sign completion, mutate CEX wallet state, or replace Nakama as the canonical online authority. An admitted Nakama or bounded migration adapter supplies ordering and identity separately.

## Public contracts

The principal surface includes `WorldCommand`, `WorldCommandDecision`, typed tactics request/outcome values, `apply_command`, `apply_tactics_command`, and transition-decision helpers. Each result carries stable status/reason/contract material sufficient for a caller to persist an accepted successor or expose a bounded rejection without interpreting display text.

## State and invariants

A command executes against one expected aggregate revision. It either produces a completely valid successor with a deterministic decision or leaves the input state semantically unchanged. Movement follows authored graph/transition rules; tasks, items, skills, health/resources, and character state remain bounded and referentially valid. A command cannot smuggle participant, custody, or canonical ordering authority into World fields.

## Candidate publication policy v2

`WORLD_COMMAND_PUBLICATION_CONTRACT` identifies `trillionnium_world_command_publication_v2`. `src/lib.rs` owns publication; `src/engine.rs` is the private, ordinary Rust decision module. The engine was moved byte-for-byte from the preceding library source, not regenerated or expanded by a build script.

Both public mutation entrypoints clone the input, run the decision engine on that private candidate, and replace the caller's state only when `accepted=true`. Rejection and unwinding before publication discard the candidate. A rejected tactics response includes the original character rather than defaults or helper mutations from the discarded candidate. Response diagnostic time is not a persisted-state update. Authentication, revision validation, database atomicity, durable duplicate lookup and recovery remain application/repository responsibilities; this in-memory boundary does not implement those adapters.

This corrects the earlier tactics path in which `ensure_defaults(now_epoch)` ran against caller-owned state before rejection. Command and outcome DTO layouts are unchanged; the publication policy is separately versioned. Accepted state and outcome equivalence to the previous engine must pass before promotion. Rejection-state vectors and exact World/Nakama component locks must be refreshed; historical execution credit does not transfer. Candidate cloning is O(serialized aggregate state size) in memory/work and requires representative-state benchmarking before production adoption.

## Dependencies and boundaries

The crate depends on `trnm-world-domain` and deterministic support libraries only. It must not import projections, UI, API/server, SQL, filesystem, HTTP, environment, credentials, CEX, or Nakama implementations. Application adapters validate caller identity/order/revision before invoking the command and persist only after full success.

## Failure and recovery

Unknown commands, invalid subjects/targets, blocked transitions, unmet prerequisites, stale expected state, insufficient resources, invalid tactics, and crossed versions fail closed with stable bounded reasons. Retrying identical admitted bytes against the identical revision is deterministic. Altered duplicates or stale revisions are rejected by the owning application/repository boundary.

## Testing and evidence

`publication_tests` covers private mutation discard, complete acceptance, pre-publication panic, six tactics rejection families at three supplied epochs, repeated rejection, five World rejection cases, and accepted-output equivalence to the private engine. Existing engine tests are retained. Run the actual crate, not only a source scanner:

```bash
cargo test --manifest-path trillionnium/crates/world-authority/Cargo.toml -p trnm-world-command --all-targets --locked
cargo clippy --manifest-path trillionnium/crates/world-authority/Cargo.toml -p trnm-world-command --all-targets --locked -- -D warnings
```

These tests are authored source until execution is retained for the exact candidate. Nakama shadow comparison, PostgreSQL replay, cross-host recovery and independent acceptance remain separate evidence classes. Production authorization remains `not_granted`.

## Compatibility and change control

Changing command fields, error precedence, transition rules, arithmetic, side effects, or canonical outcome material requires a version bump, vectors, migration/rollback analysis, projection/API updates, and cross-repository conformance. Adding a command does not authorize a new public route by itself.

## Detailed design

See [`../../../../docs/modules/world-authority/trnm-world-command-design.md`](../../../../docs/modules/world-authority/trnm-world-command-design.md).
