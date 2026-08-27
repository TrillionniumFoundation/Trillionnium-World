---
status: current
owner: trillionnium-world
contract_version: trnm_world_transition_v1
applies_to:
  - deterministic-world-rules
  - nakama-adapter-input
  - shadow-verification
supersedes: []
last_reviewed: 2026-08-28
review_due: 2026-09-11
---

# Trillionnium World Deterministic Transition Contract v1

## 1. Purpose

`trnm_world_transition_v1` is the frozen boundary between an online authority
and deterministic World rules. It allows an authority implementation to submit
one versioned game-domain command against one exact deterministic state and to
receive deterministic next-state, replay and optional outcome material.

This contract is intentionally narrower than an online-match protocol. It does
not admit players, establish sessions, total-order network events, recover an
authority process, own a canonical archive root, sign completion evidence,
claim Chain finality or settle a wallet.

## 2. Accountable systems

| Artifact or decision | Accountable system |
| --- | --- |
| Ruleset/content interpretation | World |
| Deterministic state transition | World |
| World replay and outcome material | World |
| World request/transition/outcome hashes | World |
| Online admission and participant identity | Nakama |
| Canonical global command order and idempotency | Nakama |
| Canonical archive root and `MatchCompletedV1` | Nakama |
| Match-evidence signing key | Nakama |
| Chain ingress/finality | Chain |
| Wallet/ledger settlement | CEX |
| Cross-repository component lock | Integration |

A World hash proves only that exact deterministic material was processed under
the named World ruleset/content revisions. It does not prove admission, total
order, archive completeness, signature authority, Chain finality or settlement.

## 3. Wire artifacts

The machine-readable schema is:

`docs/protocol/schemas/trnm-world-transition-v1.schema.json`

Golden serialization and SHA-256 core vectors are:

`docs/protocol/vectors/trnm-world-transition-v1.json`

The dependency-free reference package is:

`trillionnium/contracts/trnm-world-transition-v1`

## 4. Request

A request contains:

- `contract_version` — exactly `trnm_world_transition_v1`;
- `transition_id` — deterministic caller-owned correlation ID, not an authority
  receipt;
- `ruleset_revision` and `content_revision` — independently versioned;
- `expected_tick` — the deterministic state tick before application;
- `previous_state` — canonical payload plus exact SHA-256;
- `command` — command ID and canonical payload plus exact SHA-256.

Payloads are JSON objects or arrays. Their byte representation is UTF-8,
minified and key-sorted by the producer. Duplicate object keys, non-finite
numbers and ambiguous encodings are invalid. The payload SHA-256 is over the
exact canonical JSON bytes, without a newline.

## 5. Accepted result

An accepted result contains:

- exact contract/ruleset/content/transition identities;
- request hash and prior-state hash;
- deterministic next tick and next state;
- deterministic replay material;
- optional World outcome material and World outcome hash;
- World transition hash.

The result remains unsigned. The target online authority may later bind it into
its own canonical completion evidence, but that later receipt is outside this
contract.

## 6. Rejected result

Rejections use stable codes:

| Code | Meaning | Retry by default |
| --- | --- | --- |
| `invalid_contract_version` | Unsupported contract envelope | no |
| `invalid_request` | Identifier, tick or envelope invariant failed | no |
| `unknown_ruleset_revision` | Ruleset is not supported | no |
| `unknown_content_revision` | Content revision is not supported | no |
| `payload_hash_mismatch` | Canonical bytes do not match the supplied hash | no |
| `invalid_canonical_payload` | Payload is not canonical JSON | no |
| `forbidden_authority_surface` | Payload attempts to cross an authority boundary | no |
| `resource_budget_exceeded` | Published payload budget exceeded | no |
| `invalid_command` | Game-domain command validation failed | no |
| `domain_rejected` | Validly encoded command is rejected by deterministic rules | no |
| `nondeterministic_output` | Adapter output violates deterministic invariants | no |
| `internal_unavailable` | Transient World adapter unavailability | yes |

Error detail is bounded, control-free diagnostic text. Consumers must branch on
the stable code, never free-form detail.

## 7. Canonical ordering and hashes

Top-level and nested contract objects are serialized with keys in ascending
lexicographic order, no insignificant whitespace and UTF-8 strings escaped as
JSON requires.

Hashes are lower-case hexadecimal SHA-256 with domain separation:

```text
request_hash = SHA256(
  "trnm.world.transition.request.v1\n" || canonical_request_json
)

world_transition_hash = SHA256(
  "trnm.world.transition.accepted.v1\n" || canonical_accepted_facts_json
)

world_outcome_hash = SHA256(
  "trnm.world.outcome.v1\n" || canonical_outcome_binding_json
)
```

`canonical_outcome_binding_json` binds ruleset revision, content revision,
outcome schema ID and the exact outcome payload hash.

The accepted-facts preimage excludes `world_transition_hash` itself. It includes
all remaining accepted fields, including the request hash, previous-state hash,
next-state payload, replay payload and optional outcome binding.

## 8. Resource budgets

| Payload | Maximum canonical bytes |
| --- | ---: |
| Previous/next state | 2 MiB each |
| Command | 128 KiB |
| Replay material | 2 MiB |
| Outcome material | 512 KiB |

An implementation may publish a lower ruleset-specific limit, but must never
silently accept above these contract maxima.

## 9. Forbidden surfaces

The transition boundary rejects payload keys that attempt to carry:

- Nakama session tokens or authority private keys;
- match-authority private keys;
- participant-admission receipts;
- canonical global event cursors;
- canonical archive roots;
- `MatchCompletedV1` material;
- Chain AppHash/finality claims.

Game-domain payload schemas should be allowlists. The reference package applies
an additional denylist as defense in depth.

## 10. Compatibility and retirement

- Contract version and release provenance are separate concepts.
- Unknown contract, ruleset or content revisions fail closed.
- A producer must not infer compatibility from semver alone.
- A published revision remains readable for the compatibility window declared
  by Integration's exact component lock.
- Adding optional fields requires a new schema revision and golden vectors;
  changing canonical meaning, key ordering, hashes or error semantics requires
  a new contract version.
- Retirement requires a usage inventory, shadow-diff evidence, rollback plan and
  explicit Integration approval.

## 11. Concurrency, persistence and I/O

The reference transition function is pure with respect to external systems:

- no network, signer, wallet or database operation;
- no mutable global state;
- no wall-clock or random input;
- no hidden session or authority context;
- deterministic output for identical request bytes and exact component lock.

Callers own admission, sequencing, retries, persistence and canonical
publication. Those concerns must not be reintroduced through opaque payloads.

## 12. Evidence and next dependency

`WORLD-P0-002` is complete only when schema, package, golden vectors, negative
boundary tests and exact-commit CI are green. The next dependent work item is
`WORLD-P0-003`: implement the Nakama-side adapter and shadow runner against this
contract, compare exact accept/reject outputs and fail promotion on unexplained
divergence.
