---
status: current-candidate
owner: trillionnium-world-game-economy
module: trnm-economy-protocol
last_reviewed: 2026-09-08
review_due: 2026-10-08
implementation_conformance: not-implied
---

# trnm-economy-protocol detailed design

## Scope and authority

`trnm-economy-protocol` defines the game-owned vocabulary for economic intents, immutable idempotency identities, backend/account bindings, wallet snapshots, receipts, progression classes, and signed value-entitlement envelopes. World may author a game intent and project a separately authenticated receipt into campaign state. CEX remains the sole wallet/ledger and custody authority; this crate cannot create ledger success, hold keys, admit a public market, or infer a wallet balance from local soft credits.

## Public interfaces

The principal public families are `EconomicIntent`, `EconomicIntentKind`, `IdempotencyKey`, `EconomicReceipt`, `EconomyAccountBinding`, `WalletSnapshot`, asset/currency/transferability enums, settlement backend identity, receipt progression state, and server-signed entitlement V1/V2 structures. Contract names include `term_exchange_protocol_v2`, `term_exchange_backend_v2`, `trnm_economy_policy_v1`, and the entitlement versions. Constructors and validators must return typed rejection rather than partially normalized authority material.

## State model and invariants

The crate owns no mutable runtime or durable state. Its values are immutable messages interpreted by campaign, signer, settlement-worker, and CEX adapters. An idempotency identity is permanently bound to the same canonical intent bytes, account/backend identity, amount/currency, progression class, source, and entitlement context. Local soft value is non-convertible unless an explicitly versioned policy says otherwise. Unknown algorithms, issuers, versions, currencies, backends, classes, or crossed identities fail closed.

## Dependency direction

This crate remains below campaign and runtime adapters. It may depend on deterministic serialization and hashing, but must not import Bevy, SQL, HTTP clients, filesystems, wall-clock services, environment variables, credentials, campaign aggregates, or CEX implementation code. Campaign and server crates may consume it; the reverse dependency is forbidden. Cross-repository integration uses versioned schemas/vectors or exact immutable packages rather than sibling path dependencies.

## Execution, concurrency and cancellation

Validation is synchronous, bounded, deterministic, and side-effect free. The crate starts no tasks, owns no locks, performs no retries, and cannot be cancelled halfway through a mutation because it performs none. Runtime callers must validate before publishing state. Concurrent callers may validate identical bytes independently; any serialization or request deduplication belongs to the owning durable outbox/CEX service and must use the immutable identity exported by this protocol.

## Persistence and durability

Durability is owned by callers. Campaign state may persist pending intents, verified receipts, wallet snapshots, and tombstones; the settlement worker/database persists capture, lease, remote-result, and apply state; CEX persists authoritative ledger effects. Serialized protocol values must be stored with their contract/backend/issuer versions and canonical hash. A database row or save file containing a receipt is evidence only after the owning adapter verifies signature, audience, account, intent, amount, status, and hash bindings.

## Idempotency, retry and failure taxonomy

An exact retry reuses one immutable idempotency key and byte-identical intent. Altered bytes under the same key are an identity conflict. Timeout, connection loss, malformed success, and response loss are ambiguous runtime outcomes, not local success or permanent failure; recovery performs exact receipt lookup before submit. Stable failure families distinguish malformed shape, unsupported version/algorithm/backend, identity mismatch, account/audience mismatch, amount/policy violation, expiry, signature failure, duplicate-exact convergence, and duplicate-altered rejection.

## Versioning and migration

Protocol version, backend version, entitlement version, issuer key version, rules/build identity, and release provenance are separate dimensions. Additive optional fields require explicit old-reader/new-reader fixtures and unknown-field policy. Any change to signing payload order, required fields, enum meaning, hash domain, validation precedence, or economic interpretation requires a new version, canonical vectors, CEX and Integration review, an admission matrix, rollback behavior, and a dated retirement window for the predecessor.

## Resource and performance budgets

Every externally supplied string, collection, metadata map, signed payload, receipt body, and wallet snapshot must have explicit byte/count limits before promotion. Validation must be linear in bounded input size, avoid unbounded recursive structures, and never allocate according to an untrusted declared length without checking the cap. The owning HTTP adapter applies body/time/redirect limits before deserialization. Changes to limits require benchmarks, compatibility review, negative boundary vectors, and denial-of-service analysis.

## Observability and security

Logs and evidence may include stable operation/intent/backend/issuer identifiers and bounded machine error codes, but never bearer tokens, private keys, signatures presented as secrets, or unbounded remote bodies. Metrics distinguish pending, lookup, submit, exact replay, altered conflict, malformed response, signature/audience mismatch, policy rejection, and apply outcomes. Entitlements are accepted only from an explicit active issuer registry with validity windows and revocation behavior. Public-market enablement remains external and disabled by default.

## Test and evidence traceability

Primary source is `trillionnium/crates/trnm-economy-protocol/src/lib.rs`. Normative neighbors include `docs/adr/0002-transaction-free-external-settlement.md`, `docs/protocol/trnm-settlement-receipt-recovery-v1.md`, `docs/architecture/trnm-settlement-lifecycle-v2.md`, and the CEX compatibility lock. Required tests cover serialization round trips, V1/V2 vectors, altered retry, signature/algorithm/audience/account/amount/expiry mismatch, exact duplicate convergence, unsupported values, and campaign/server projection. Hosted and cross-repository evidence remains separate.

## Change checklist and open work

Before changing this crate, classify the authority effect; update Rust types, canonical schema, examples and golden vectors together; prove old/new reader behavior; update campaign/server/CEX adapters and Integration locks; add boundary and hostile tests; record resource-limit and signing-domain effects; and preserve `public_player_market=disabled`. Open work includes generated JSON Schema, a complete field dictionary, stable error catalogue alignment, canonical entitlement payload vectors, and exact World/CEX cross-repository conformance on one immutable pair.
