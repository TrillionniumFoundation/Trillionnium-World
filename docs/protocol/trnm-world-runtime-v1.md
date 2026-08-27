# TRNM World Deterministic Runtime Protocol v1

Status: **proposed / source implemented**  
Owner: Trillionnium World deterministic game domain  
Consumers: Trillionnium Nakama and Trillionnium Integration  
Updated: 2026-08-27  
Contract files: `../../contracts/world-runtime/v1/`

## 1. Protocol objective

`trnm_world_runtime_v1` lets an external match authority execute one exact World ruleset over an already ordered local command batch and receive unsigned deterministic game-domain output.

The protocol deliberately separates game authority from online authority:

- World is authoritative for rules, deterministic state transitions and game-owned outcome facts.
- Nakama is authoritative for authenticated participants, global ordering, idempotency, restart recovery, canonical match roots and completion signing.
- Integration locks the exact World contract and runs independent consumer vectors.
- Chain and CEX remain outside the deterministic execution request/result.

## 2. Execute request

An execute request contains only:

- exact contract version;
- exact ruleset ID/version/digest;
- exact authored-content digest;
- deterministic initial game state;
- an already ordered array of game commands.

`batch_ordinal` is contiguous from zero. It exists to make local batch reconstruction deterministic; it grants no global sequencing authority.

The request does not carry a participant roster, global cursor, completion key, Chain finality or wallet state.

## 3. Execute result

A result contains:

- the exact selected ruleset and content digests;
- initial-state and command-batch hashes;
- final game state and hash;
- game-owned outcome facts and hash;
- replay-domain material and hash.

It is unsigned. Nakama may bind these exact World outputs into its separately versioned authoritative completion evidence. World must not reconstruct or sign that evidence.

## 4. Canonical serialization and hashing

The canonical profile and domains are normative in `contracts/world-runtime/v1/README.md`. Hashes use:

```text
SHA-256(UTF8(domain) || LF || canonical_json_bytes)
```

No implementation may substitute locale-sensitive sorting, floating-point normalization, platform-native number encoding or an unversioned serializer.

## 5. Semantic validation order

A consumer or World adapter validates in this order:

1. strict UTF-8 JSON parse with duplicate-key and float rejection;
2. envelope contract version and exact field set;
3. ruleset/content digest syntax and availability;
4. canonical-value limits and normalized-key collision checks;
5. contiguous command ordinals;
6. ruleset-specific command/state validation;
7. deterministic execution;
8. canonical output serialization and hashing.

Failure at any step produces no authoritative output and no settlement/signature side effect.

## 6. Error contract

The first implementation must map failures to stable machine-readable classes:

| Error class | Retry meaning |
| --- | --- |
| `unsupported_contract` | retry only after component upgrade |
| `ruleset_unavailable` | retry only with an installed exact digest |
| `content_unavailable` | retry only with an installed exact digest |
| `invalid_canonical_json` | caller must replace malformed payload |
| `resource_limit_exceeded` | caller must submit a bounded payload |
| `ordinal_discontinuity` | authority must repair supplied batch order |
| `invalid_game_command` | caller must replace game-domain command |
| `deterministic_execution_failed` | quarantine exact input and investigate |
| `output_contract_violation` | fail closed and quarantine implementation |

A future transport binding may add request IDs and retry metadata, but it must not reinterpret a deterministic error as a successful game result.

## 7. Consumer conformance

Nakama conformance requires:

- independent strict parsing/canonicalization implementation;
- every fixed vector and negative vector;
- proof that World `batch_ordinal` is not reused as Nakama global sequence;
- proof that no World result field is treated as a participant roster, canonical match root, signature or finality proof;
- exact component lock through Integration.

Integration conformance requires:

- exact World commit/tree and contract-blob hashes;
- exact Nakama consumer commit/tree;
- independent canonical/hash report;
- tampered vector failures;
- immutable evidence record with environment and limitations.

## 8. Rollout

1. Accept schema, vectors and verifier.
2. Implement a Bevy-free World Rust adapter that emits this exact envelope.
3. Add golden vectors produced by the real ruleset implementation.
4. Add an independent Nakama consumer.
5. Add Integration exact component locks and cross-language verification.
6. Run unsigned dual comparison against legacy World-local authority.
7. Route new closed-alpha matches through one selected Nakama authority.
8. Retire legacy World authority only after drain, retention and credential revocation.

Dual comparison never signs, publishes or settles twice.

## 9. Evidence and non-claims

Passing the source verifier establishes only canonical contract conformance. It does not establish:

- production deployment;
- online availability or cross-host recovery;
- public security/edge readiness;
- CEX settlement correctness;
- Chain finality;
- public-online or player-market readiness.

Those remain separate gates in Development Plan v2 and the P0 execution registry.
