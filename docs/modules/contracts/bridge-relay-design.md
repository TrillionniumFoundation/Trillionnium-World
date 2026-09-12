---
status: current-candidate
owner: trillionnium-world-contracts
module: bridge-relay
last_reviewed: 2026-09-09
review_due: 2026-10-08
implementation_conformance: not-implied
---

# `bridge-relay` detailed design

## Scope and authority

`bridge-relay` is the Rust MVP state machine for proof/nonce/finality-gated relay semantics and normalized audit output. It records which bounded relay messages or proof identities may advance under the current fixture rules. It is not a production bridge, light client, validator, relayer fleet, custody contract, finality oracle or deployed Chain component. Cross-chain value transfer remains disabled unless separately scoped and independently qualified.

## Public interfaces

The public surface contains typed chain/domain/message/proof/nonce identifiers, relay request and decision values, in-memory state, validation helpers, stable errors and audit-event construction. Accepted input returns an explicit bounded state transition. A caller-supplied proof label or finality number has no authority until a future host verifier validates it under a versioned source-chain contract. Exact methods, field spellings and canonical bytes require generated schema and vectors before external use.

## State model and invariants

Relay identity, source/destination domains, nonce and payload/proof hash are immutable. A nonce cannot be accepted twice for altered material; finality cannot regress; unsupported domains/proof versions fail closed; and an error preserves the full state. Any ordering/window policy is deterministic and bounded. The MVP must not infer source-chain finality from wall-clock age, local presence or an unverified caller assertion. No in-memory record proves that an external event occurred.

## Dependency direction

The crate may depend on `audit-events`, deterministic serialization/hashing and bounded collections. It must not import live Chain clients, RPC, networking, wallets, World/Nakama/CEX implementations, SQL, filesystem, clocks, secrets or deployment configuration. A future verifier/host adapter obtains headers/proofs and supplies a typed verified decision; the pure state machine never calls outward during mutation.

## Execution, concurrency and cancellation

Execution is synchronous, deterministic and single-owner. It acquires no external lock and starts no task. A future runtime serializes one relay stream or uses expected revision/CAS. Proof retrieval and verification happen before entering the mutation boundary. Cancellation drops private candidate state; an uncertain durable commit is resolved by immutable relay identity lookup. Remote calls while a mutable host transaction is held are forbidden.

## Persistence and durability

Current state is in-memory. Production adoption requires canonical durable state, verified header/finality checkpoints, immutable request/decision records, atomic state-plus-audit publication, replay/bootstrap, backup/PITR, old-writer fencing and relayer recovery. Fixture proofs, local files and unit tests are not bridge evidence. A production relay also needs an accountable source of canonical headers and reorganization policy.

## Idempotency, retry and failure taxonomy

An immutable relay ID binds source/destination, nonce, payload hash, proof/version, checkpoint/finality context and contract version. Exact retry may converge; altered reuse fails. Stable failures separate malformed/unsupported input, unknown domain, invalid proof binding, insufficient finality, nonce replay/gap/window, stale checkpoint, payload conflict, bounds/overflow and integrity failure. Transport/RPC timeout, verifier unavailability and ambiguous storage commit remain adapter/runtime errors.

## Versioning and migration

Relay semantics, source-chain verifier, finality policy, proof format, state schema, audit schema, host ABI and deployment release are separate versions. Changes to canonical proof bytes, domain identifiers, finality threshold, nonce policy, error precedence or state transition require a new contract/version, old/new readers, vectors, migration/rollback and reorganization analysis. A generic “bridge v1” label cannot cover different chains or finality models.

## Resource and performance budgets

Payload/proof/header bytes, identifiers, domains, pending nonces, replay window, metadata depth and stored checkpoint history are bounded. Validation must reject oversized or deeply nested material before expensive cryptography. Proof verification, gas, storage, RPC concurrency and relayer queues require separate budgets in the future host. Raising any ceiling requires denial-of-service and state-growth review.

## Observability and security

Audit data binds stable relay/message/proof/checkpoint IDs, contract/verifier versions and bounded outcome codes. It excludes private keys, bearer credentials, complete untrusted payloads and sensitive endpoint data. Production requires independent verifier review, key separation, source/destination allow-lists, rate/abuse controls, pause/rollback policy, monitoring for finality/reorg divergence and explicit custody analysis where value is involved.

## Test and evidence traceability

Primary source is `contracts/bridge-relay/src/`. The binding design is `docs/modules/contracts/bridge-relay-design.md`, and root status is `contracts/README.md`. Structural and hostile-fixture enforcement is executed by `scripts/check-trnm-world-contract-module-documentation.py` and `scripts/test-trnm-world-contract-module-documentation.py`. Rust tests cover exact relay, altered duplicate, nonce replay/order, proof/hash/domain mismatch, insufficient/regressing finality, bounds, state preservation and audit normalization. Real-chain, reorganization, cross-client, host and custody tests remain absent.

## Change checklist and open work

A change states affected chains, proof/finality/nonce semantics, canonical bytes, errors, state growth and security assumptions; updates schema/vectors/tests/audit docs; and obtains bridge/security/Chain review. Open work includes chain-specific verifier contracts, canonical header/proof vectors, reorg model, host ABI/runtime-spec, durable storage, deterministic WASM/gas, relayer architecture, fault/chaos tests, independent audit and explicit public bridge scope authorization.
