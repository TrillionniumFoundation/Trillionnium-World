---
status: current-candidate
owner: trillionnium-world
contract_version: trnm_world_transition_v1
applies_to:
  - deterministic-world-rules
  - nakama-adapter-input
  - shadow-verification
last_reviewed: 2026-08-29
review_due: 2026-09-12
---

# Trillionnium World Deterministic Transition Contract v1

## Purpose

`trnm_world_transition_v1` is the unsigned boundary between canonical online
match authority and deterministic World rules. It advances one exact World state
with one exact game-domain command and returns next-state, replay, and optional
outcome material.

It does **not** authenticate players, assign roles, total-order network events,
own online idempotency, recover a match process, construct canonical archive
roots, sign `MatchCompletedV1`, claim Chain finality, or settle a wallet.

## Ownership

| Decision or artifact | Accountable system |
| --- | --- |
| Ruleset/content interpretation | World |
| Deterministic state transition | World |
| Unsigned replay/outcome material | World |
| World request/transition/outcome hashes | World |
| Admission, total order, idempotency, recovery | Nakama |
| Canonical roots and `MatchCompletedV1` | Nakama |
| Chain ingress/finality | Chain |
| Wallet/ledger/custody | CEX |
| Cross-repository component lock | Integration |

## Strict canonical JSON profile

Every opaque payload is exact UTF-8 JSON and must satisfy all rules below:

1. root is an object or array;
2. no insignificant whitespace;
3. object keys are strictly ascending by decoded Unicode scalar sequence;
4. duplicate keys are rejected;
5. numbers are signed decimal `i64` integers only;
6. no leading zeros, `-0`, decimal point, exponent, `NaN`, or infinity;
7. strings are valid UTF-8 and contain no unescaped control characters;
8. the short escapes `\"`, `\\`, `\b`, `\f`, `\n`, `\r`, `\t` are used when applicable;
9. `\u` is permitted only for remaining U+0000..U+001F values, with lower-case hex;
10. nesting depth is at most 128;
11. there are no trailing bytes;
12. re-encoding the decoded value must reproduce the exact input bytes;
13. decoded object keys are recursively checked for forbidden authority fields.

This is a contract property, not a serialization-library preference. A producer
that emits a semantically equivalent but byte-different document is rejected.

## Resource budgets

| Material | Maximum bytes |
| --- | ---: |
| Previous state | 2 MiB |
| Command | 128 KiB |
| Next state | 2 MiB |
| Replay | 2 MiB |
| Outcome | 512 KiB |
| Identifier | 160 bytes |
| Error detail | 256 UTF-8 bytes |

## Request

A request contains:

- exact `contract_version`;
- caller correlation `transition_id`;
- independent `ruleset_revision` and `content_revision`;
- `expected_tick` in `0..=i64::MAX`;
- previous-state canonical payload and SHA-256;
- command ID, canonical payload, and SHA-256.

`transition_id` is correlation material, not an admission receipt or canonical
sequence.

## Results

An accepted result binds:

- request hash and previous-state hash;
- exact revisions and transition ID;
- next tick/state;
- replay material;
- optional outcome material and outcome hash;
- World transition hash.

A rejected result contains stable machine code, bounded detail, optional request
hash, and retryability. Consumers branch on code, never on detail.

Stable codes are:

```text
invalid_contract_version
invalid_request
unknown_ruleset_revision
unknown_content_revision
payload_hash_mismatch
invalid_canonical_payload
forbidden_authority_surface
resource_budget_exceeded
invalid_command
domain_rejected
nondeterministic_output
internal_unavailable
```

Only `internal_unavailable` is retryable by default.

## Domain-separated hashes

```text
request_hash = SHA256(
  "trnm.world.transition.request.v1\n" || canonical_request
)

world_transition_hash = SHA256(
  "trnm.world.transition.accepted.v1\n" || canonical_accepted_facts
)

world_outcome_hash = SHA256(
  "trnm.world.outcome.v1\n" || canonical_outcome_binding
)
```

`canonical_accepted_facts` excludes `world_transition_hash` itself. Outcome
binding includes ruleset revision, content revision, outcome schema ID, and
outcome payload hash.

## Forbidden authority material

Opaque payloads reject decoded keys including:

```text
nakama_session_token
nakama_private_key
match_authority_private_key
canonical_archive_root
chain_finality
chain_app_hash
match_completed_v1
participant_admission_receipt
global_event_cursor
```

Ruleset-specific schemas should use allowlists; this denylist is defense in
depth.

## Compatibility

Contract, ruleset, content, and build provenance are separate identifiers.
Unknown revisions fail closed. Any change to canonical bytes, hash preimages,
error semantics, required fields, or resource ceilings requires a new contract
version and new golden vectors. Retirement requires usage inventory, shadow-diff
results, drain/rollback rehearsal, and Integration approval.

## Evidence

The reference package is:

`trillionnium/contracts/trnm-world-transition-v1`

Machine schema and vectors are:

- `docs/protocol/schemas/trnm-world-transition-v1.schema.json`
- `docs/protocol/vectors/trnm-world-transition-v1.json`
- `docs/protocol/vectors/trnm-world-transition-negative-v1.json`

Independent conformance is:

`python3 scripts/check-trnm-world-transition-conformance.py`

Source presence does not grant Nakama cutover or closed-online credit. Exact-head
Rust, independent-vector, and cross-repository shadow evidence are mandatory.
