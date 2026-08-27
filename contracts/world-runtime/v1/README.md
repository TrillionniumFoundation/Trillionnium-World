# TRNM World Deterministic Runtime Contract v1

Status: **proposed implementation contract**  
Owner: Trillionnium World deterministic game domain  
Authority consumer: Trillionnium Nakama  
Cross-repository lock/evidence owner: Trillionnium Integration  
Updated: 2026-08-27

## Scope

This contract is the language-neutral boundary between World gameplay logic and an external online match authority. It defines deterministic game-domain input/output and canonical hashing. It does **not** define or transfer online authority to World.

World owns:

- ruleset and authored-content digests;
- deterministic initial game state;
- deterministic execution of an already ordered command batch;
- final game state and game-owned outcome facts;
- replay-domain material produced by the game rules;
- canonical serialization and hashes for those World-owned values.

Nakama owns:

- authenticated match authorization and participant framing;
- global command/event ordering and command idempotency;
- restart recovery and authoritative archive construction;
- canonical roster/event/archive roots;
- `MatchCompletedV1` construction and signing.

Chain owns ingress/finality/inclusion proof. CEX owns wallet/ledger settlement and custody. Integration owns exact component locks and cross-repository conformance evidence.

## Files

- `trnm-world-runtime-v1.schema.json` — request/result JSON Schema.
- `golden-vectors.json` — canonicalization and request vectors.
- `../../../../scripts/verify-trnm-world-runtime-v1.py` — standard-library reference verifier.

## Canonical JSON profile

Before hashing, values are canonicalized as follows:

1. Input is UTF-8 JSON with no duplicate object keys.
2. Object keys and string values are normalized to Unicode NFC.
3. Object keys are ordered by normalized UTF-8 bytes.
4. Array order is preserved.
5. Only `null`, booleans, strings, signed 64-bit integers, arrays and objects are allowed.
6. Floating-point/exponent numbers are rejected, including mathematically integral forms such as `1.0`.
7. JSON output has no insignificant whitespace and uses UTF-8 characters directly.
8. Maximum depth is 64, maximum node count is 100,000 and maximum canonical byte size is 16 MiB.

A hash is:

```text
SHA-256( UTF8(domain) || 0x0a || canonical_json_bytes )
```

Domains are exact ASCII strings:

- `trnm.world.runtime.v1.initial_state`
- `trnm.world.runtime.v1.command_batch`
- `trnm.world.runtime.v1.final_state`
- `trnm.world.runtime.v1.outcome`
- `trnm.world.runtime.v1.replay_material`

## Command ordinal semantics

`batch_ordinal` starts at zero and is contiguous inside one supplied World execution request. It is only a deterministic local batch position. It is not a match-global sequence, receipt cursor, participant sequence, Nakama event number or Chain nonce.

World must reject missing, duplicated or non-contiguous ordinals. World must not create a replacement global order.

## Determinism rules

Execution must not depend on:

- wall-clock time or local timezone;
- process/thread scheduling;
- unseeded randomness;
- filesystem enumeration order;
- network/DNS/external service results;
- hash-map iteration order;
- host architecture-specific floating-point behavior;
- private keys, signatures or Chain/CEX state.

All randomness required by a ruleset must already be represented in the versioned initial state or command payload.

## Forbidden authority fields

The request/result envelope intentionally has `additionalProperties: false`. It cannot carry:

- participant roster or authenticated role authority;
- match-global sequence/cursor;
- command idempotency receipt;
- canonical roster/event/archive root;
- completion signature or authority key identifier;
- Chain finality/inclusion proof;
- CEX wallet balance or custody state.

A new field that changes authority ownership requires a replacement ADR and contract version.

## Compatibility

- Consumers select an exact `contract_version`, `ruleset.id`, `ruleset.version`, `ruleset.digest` and `content_digest`.
- Unknown contract versions fail closed.
- A digest mismatch fails before execution.
- Adding or changing a required field, canonicalization rule, hash domain or semantic invariant requires a new contract version.
- Golden vectors are immutable once a version is accepted; corrections create a new vector set/version rather than silently editing historical evidence.

## Conformance

A conforming implementation must:

1. validate the schema and semantic ordinal rules;
2. reproduce every canonicalization vector byte-for-byte;
3. reproduce every fixed SHA-256 vector;
4. reject duplicate keys, floats, out-of-range integers and non-NFC key collisions;
5. produce identical output/hashes for semantically identical object-key orderings;
6. change the relevant hash when a command/state/outcome/replay value changes;
7. avoid producing any forbidden authority field.

Passing this contract proves deterministic boundary conformance only. It does not prove online operations, security, deployment, public release readiness or economic correctness.
