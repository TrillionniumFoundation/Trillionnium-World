# trnm-economy-protocol

Status: current candidate  
Owner: Trillionnium World game economy  
Release effect: none by itself

## Purpose

This crate defines the game-owned economic vocabulary exchanged by campaign code, the settlement outbox, signers, and CEX adapters. It owns typed asset semantics, account bindings, immutable intents, idempotency keys, receipts, wallet snapshots, progression classes, and signed value-entitlement envelopes.

## Authority and non-goals

World may create game intents and project independently verified receipts into campaign state. CEX remains authoritative for wallet and ledger state and custody. This crate never opens network connections, stores private keys, signs on behalf of CEX, or enables a public player market. A serialized receipt is evidence only after the owning backend has authenticated and validated it.

## Public contracts

The principal contract families are `term_exchange_protocol_v2`, `term_exchange_backend_v2`, `trnm_economy_policy_v1`, `trnm_server_signed_value_entitlement_v1`, and `trnm_server_signed_value_entitlement_v2`. Public structs and enums are the Rust source of truth until generated JSON Schema is published. V2 entitlements bind algorithm, issuer/key identity, actor/account, source, intent, amount/currency, validity window, match/rules/build identities, result and participant hashes, nonce, and signature.

## State and invariants

The crate itself is stateless. Callers must preserve these invariants:

- an idempotency identity is immutable and cannot be reused with different bytes;
- positive wallet value is never inferred from local soft credits;
- receipt identity, intent identity, account binding, backend identity, amount, and progression class must agree exactly;
- unknown algorithms, versions, currencies, backends, or progression classes fail closed;
- public-market enablement remains false unless a separate externally approved gate changes it.

## Dependencies and boundaries

The crate depends only on serialization and hashing support and must remain independent of Bevy, HTTP clients, databases, filesystems, wall clock access, and service credentials. Campaign code may depend on this crate; this crate must not depend on campaign or runtime adapters.

## Failure and recovery

Shape and semantic validation return bounded errors and must not mutate caller state. Timeout, conflict, malformed success, or response loss are settlement-runtime concerns and must be recovered by exact receipt lookup using the same immutable intent identity, never by manufacturing a local success.

## Testing and evidence

Required coverage includes round-trip serialization, altered-retry rejection, account/audience mismatch, amount and validity bounds, V1/V2 compatibility fixtures, signature-algorithm rejection, duplicate receipt handling, and campaign projection tests. Source/unit tests do not grant custody, deployed settlement, public-market, or commercial evidence.

## Compatibility and change control

Additive optional fields require explicit compatibility tests. Any changed signing payload, required field, hash domain, enum meaning, or validation rule requires a new contract version, migration notes, golden vectors, CEX/Integration review, and a declared retirement window for the old version.
