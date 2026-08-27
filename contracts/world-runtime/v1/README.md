# TRNM World Deterministic Runtime Contract v1

Status: **source implemented / pending exact-head Rust and external-consumer evidence**  
Owner: Trillionnium World deterministic game domain  
Canonical online consumer: Trillionnium Nakama  
Cross-repository lock/evidence owner: Trillionnium Integration  
Updated: 2026-08-28

## Scope

This contract is the language-neutral boundary between World gameplay logic and
an external online match authority. It defines deterministic game-domain
input/output, canonical hashing, execution observations and unsigned shadow
comparison. It does **not** define or transfer online authority to World.

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

Chain owns ingress/finality/inclusion proof. CEX owns wallet/ledger settlement
and custody. Integration owns exact component locks and cross-repository
conformance evidence.

## Files

### Language-neutral contract

- `trnm-world-runtime-v1.schema.json` — execute request/result JSON Schema;
- `trnm-world-shadow-v1.schema.json` — runtime observation, shadow input and
  report JSON Schema;
- `golden-vectors.json` — canonicalization and request vectors;
- `shadow-vectors.json` — success, rejection, tamper, request-binding and
  resource-budget vectors;
- `error-catalog.json` — stable runtime rejection and shadow-divergence codes;
- `compatibility-matrix.json` — current World/Nakama/Integration promotion
  state and fail-closed flags.

### Implementations

- `../rust/` — strict canonical JSON and deterministic RTS rules adapter;
- `../host/` — Bevy-free execution CLI, observation builder and typed shadow
  comparator;
- `../../../../scripts/verify-trnm-world-runtime-v1.py` — independent
  standard-library runtime-vector verifier;
- `../../../../scripts/verify-trnm-world-shadow-v1.py` — independent
  standard-library shadow-vector verifier.

## Canonical JSON profile

Before hashing, values are canonicalized as follows:

1. input is UTF-8 JSON with no duplicate object keys;
2. object keys and string values are normalized to Unicode NFC;
3. object keys are ordered by normalized UTF-8 bytes;
4. array order is preserved;
5. only `null`, booleans, strings, signed 64-bit integers, arrays and objects are
   allowed;
6. floating-point/exponent numbers are rejected, including `1.0`;
7. output has no insignificant whitespace and uses UTF-8 characters directly;
8. maximum depth is 64, maximum node count is 100,000 and maximum canonical
   byte size is 16 MiB.

A hash is:

```text
SHA-256( UTF8(domain) || 0x0a || canonical_json_bytes )
```

Runtime domains are exact ASCII strings:

- `trnm.world.runtime.v1.initial_state`
- `trnm.world.runtime.v1.command_batch`
- `trnm.world.runtime.v1.final_state`
- `trnm.world.runtime.v1.outcome`
- `trnm.world.runtime.v1.replay_material`

Shadow evidence adds:

- `trnm.world.shadow.v1.request`
- `trnm.world.shadow.v1.response`
- `trnm.world.shadow.v1.divergence_value`

## Command ordinal semantics

`batch_ordinal` starts at zero and is contiguous inside one supplied World
execution request. It is only a deterministic local batch position. It is not a
match-global sequence, receipt cursor, participant sequence, Nakama event number
or Chain nonce.

World rejects missing, duplicated or non-contiguous ordinals and never creates
a replacement global order.

## Execution observation

`trnm_world_runtime_observation_v1` binds:

- implementation identifier and exact 40-hex revision;
- full canonical request hash;
- either an execute result or a stable deterministic error envelope;
- canonical response byte count;
- measured duration for an explicitly configured evidence budget.

A response observation is invalid when a claimed final-state, outcome or replay
hash does not bind its material, when the byte count is inaccurate, or when any
forbidden authority field appears.

## Shadow comparison

`trnm_world_shadow_input_v1` contains a World observation, an independent
candidate observation and positive duration/response-size budgets.

A report is equivalent only when:

- request hashes match;
- both paths either succeed or reject deterministically;
- success paths match exact ruleset/content/input bindings and final
  state/outcome/replay material by value and hash;
- rejection paths match stable error code and recoverability;
- candidate resource budgets pass.

Diagnostic error wording is not canonical. A self-inconsistent observation is
rejected before normal divergence classification.

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

All randomness required by a ruleset must already be represented in the
versioned initial state or command payload.

## Forbidden authority fields

The envelopes intentionally use `additionalProperties: false` and recursively
reject fields that imply:

- participant roster or authenticated role authority;
- match-global sequence/cursor or idempotency receipt;
- canonical roster/event/archive root;
- completion signature or authority key identifier;
- Chain finality/inclusion proof;
- CEX wallet balance or custody state;
- participant session tokens.

A field that changes ownership requires a replacement ADR and contract version.

## Compatibility and promotion

- consumers select an exact contract, ruleset version/digest and content digest;
- unknown versions and digest mismatches fail before execution;
- canonicalization, hash-domain or required-field changes require a new contract
  version;
- accepted golden vectors are immutable;
- an independent Nakama consumer and Integration component lock are mandatory;
- zero unexplained fixture and load divergence is mandatory;
- active compatibility matches must drain unless a separately approved takeover
  matrix exists;
- public online and public player markets remain disabled until all dependent
  gates pass.

## Conformance

A conforming implementation must:

1. validate schemas and semantic ordinal rules;
2. reproduce canonicalization and hash vectors byte-for-byte;
3. reject duplicate keys, floats, out-of-range integers and NFC collisions;
4. reject forbidden authority material;
5. self-validate all claimed result hashes;
6. produce typed shadow divergence without masking an invalid observation;
7. expose no Bevy, network, persistence or signing capability in the runtime
   boundary.

Passing this contract proves deterministic source/implementation conformance
only. It does not prove an independent Nakama consumer, authenticated online
operation, canonical completion, deployment, public release readiness, Chain
finality or economic correctness.
