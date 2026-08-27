---
status: candidate
owner: trillionnium-world
applies_to:
  - WORLD-P0-002
  - WORLD-P0-003
  - WORLD-P0-004
contract_release: trnm_world_rules_v1@1.0.0-alpha.1
last_reviewed: 2026-08-28
review_due: 2026-09-11
---

# Trillionnium World deterministic rules contract v1

## Decision

`trnm_world_rules_v1` is the only new World-to-Nakama game-rules boundary in
this tranche. It exposes deterministic state transition facts and deliberately
cannot represent online authority.

World owns:

- versioned game rules and authored content;
- canonical game-domain state and command bytes;
- deterministic validation and transition;
- unsigned outcome and replay material;
- World request/result/transition hashes.

Nakama remains responsible for:

- participant admission and sessions;
- canonical global event order and match version;
- command idempotency and restart recovery;
- canonical archive roots;
- `MatchCompletedV1` and its signing key.

Chain, CEX and Integration responsibilities remain unchanged. A World hash does
not prove admission, ordering, archive completeness, finality, custody, or
commercial readiness.

## Package layout

```text
trillionnium/contracts/trnm-world-rules-contract-v1/
├── Cargo.toml
├── Cargo.lock
├── contract-manifest-v1.json
├── schema/
│   ├── transition-request-v1.schema.json
│   ├── transition-receipt-v1.schema.json
│   └── error-catalog-v1.json
├── src/
│   ├── canonical.rs
│   ├── digest.rs
│   ├── engine.rs
│   ├── error.rs
│   ├── model.rs
│   ├── lib.rs
│   └── bin/trnm-world-rules-vector.rs
└── vectors/first-contact-vector-0001.json
```

The Rust package is an independent, zero-third-party-dependency reference. It
uses an internal safe Rust SHA-256 implementation tested against public standard
vectors. This avoids pulling network, database, async runtime, signing, wallet,
or session dependencies into the contract package.

## Request contract

The request contains only:

- exact contract, ruleset and content revisions;
- a deterministic transition ID supplied by the authority adapter;
- canonical game state bytes;
- canonical game command bytes;
- deterministic step/output/replay budgets.

State and command payloads are opaque to the envelope but must already be
canonical under the locked ruleset/content package. The envelope commits them
by SHA-256 and length-bounded lowercase hex in the JSON projection.

Unknown fields are rejected. The request schema has no player ID, account ID,
session, admission decision, global sequence, archive root, signature, private
key, Chain finality or wallet field.

## Canonical encoding

Canonical requests use fixed ASCII lines in a fixed order:

```text
TRNM-WORLD-RULES-REQUEST/1
contract=...
ruleset=...
content=...
transition=...
max_steps=...
max_output_bytes=...
max_replay_bytes=...
state=<lowercase hex>
command=<lowercase hex>
```

Canonical results use `TRNM-WORLD-RULES-RESULT/1`, fixed field order and
lowercase SHA-256. The transition hash commits the result body before the final
`transition_hash` line.

Tokens allow only ASCII letters, digits, `.`, `_`, `:`, `@` and `-`; whitespace,
line breaks and `=` are rejected so no field can inject or reorder canonical
lines.

## Result semantics

An applied receipt commits:

- request, state-before and command hashes;
- state-after, outcome and replay hashes;
- deterministic steps/output/replay usage;
- final transition hash.

A rejected receipt commits:

- the request bindings;
- one stable error code;
- zero state-after/outcome/replay hashes;
- zero deterministic resource counters.

Free-text diagnostics are explicitly excluded from the transition commitment,
so localization or redaction cannot change deterministic facts.

## Error catalogue

The stable v1 codes are:

- `unsupported_contract_version`;
- `unknown_ruleset_revision`;
- `invalid_content_revision`;
- `invalid_transition_id`;
- `malformed_state`;
- `malformed_command`;
- `invalid_resource_budget`;
- `resource_budget_exceeded`;
- `domain_rejected`;
- `output_too_large`;
- `nondeterministic_result`;
- `internal_contract_error`.

Unknown errors fail closed as `internal_contract_error`. Error strings are wire
contract values and cannot be renamed inside v1.

## Determinism and resource budgets

`execute_transition_verified` runs the immutable request twice and compares
applied bytes/resource counters or the stable rejection code. Any difference
returns `nondeterministic_result` and commits no game output.

Contract ceilings:

- state: 4 MiB;
- command: 256 KiB;
- outcome: 1 MiB;
- replay: 16 MiB;
- steps: 10,000,000.

A request may set tighter limits. Budget violation produces a rejected receipt;
no partial state, outcome or replay hash is accepted.

The double-run helper is fixture/shadow evidence. A production adapter may run
once only after cross-runtime conformance is green and its canonical authority
rules remain in Nakama.

## Independent reference and shadow comparison

`tools/trnm-world-shadow-diff/reference_contract.py` independently reproduces
canonical requests, receipts and hashes using Python's standard library.

`trnm_world_shadow_diff.py` compares World and Nakama JSONL records. It requires
identical fixture sets and exact equality of every deterministic field. It
fails on:

- malformed or unknown fields;
- duplicate/missing/unexpected fixtures;
- any request/state/command/result/replay/transition hash difference;
- disposition or error difference;
- deterministic resource-use difference;
- rejected records that claim nonzero output;
- extra online-authority fields.

The summary records both input-file SHA-256 values and limitations. Shadow
comparison is evidence of deterministic equality only.

## Component lock

`integration/component-locks/trnm-world-rules-v1.lock.json` binds the immutable
candidate package release, artifact paths, canonical encoding and consumer
requirements. Its activation is `shadow_only`.

Integration must bind exact World and Nakama repository commits before cutover.
The candidate lock in World cannot activate public authority or substitute for
an Integration release lock.

## Cutover

The cutover and rollback procedure is
`docs/runbooks/trnm-world-nakama-authority-cutover-v1.md`.

Key invariants:

- no automatic cross-generation live takeover;
- stop new World-local admission before drain;
- preserve existing matches on their original generation;
- Nakama is the only canonical completion signer after cutover;
- World-local authority remains laboratory-only;
- any unexplained divergence blocks promotion;
- rollback disables new admission rather than rewriting canonical history or
  making the compatibility enclave public.

## Automated gates

`trnm-world-rules-contract` runs:

1. contract/schema/vector/error/lock/runbook consistency;
2. independent Python reference and shadow unit tests;
3. negative fixtures for session authority, public World-local authority,
   missing determinism verification and sibling-checkout coupling;
4. Rust formatting;
5. Rust tests;
6. Clippy with warnings denied;
7. canonical vector emission.

Absent exact-commit workflow runs remain a blocker rather than implicit success.

## Open promotion evidence

This tranche implements the World contract and comparison machinery. Promotion
still requires external artifacts that this repository cannot fabricate:

- Nakama adapter implementation and its exact commit;
- Integration exact-revision release lock;
- representative shadow fixture corpus with zero unexplained divergence;
- drain/cutover/rollback rehearsal;
- exact-commit CI evidence in all owning repositories.

Until those exist, the package remains `candidate`, the component lock remains
`shadow_only`, and public online remains disabled.
