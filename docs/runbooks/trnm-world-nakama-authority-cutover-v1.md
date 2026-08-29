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

# World deterministic rules / Nakama authority cutover v1

## Purpose

Move new online-match authority to Nakama without assigning World a second
canonical cursor, archive root, completion signature, or private key. World
continues to own deterministic game rules, authored content, unsigned outcome
facts and replay material.

The current `trnm-game-server` remains a compatibility laboratory under
`world_legacy_local_alpha_v1`. It is not a public authority and must not be
promoted by this runbook.

## Bound contract

- package: `trillionnium/contracts/trnm-world-rules-contract-v1`;
- release: `trnm_world_rules_v1@1.0.0-alpha.1`;
- canonical encoding: `trnm-canonical-lines-v1`;
- hash: SHA-256;
- candidate component lock:
  `integration/component-locks/trnm-world-rules-v1.lock.json`;
- comparison tool: `tools/trnm-world-shadow-diff/trnm_world_shadow_diff.py`.

Only the fields defined by the request and receipt schemas cross the rules
boundary. Session tokens, player admission decisions, canonical global
sequence, archive roots, signing keys, Chain credentials and wallet settlement
material are forbidden.

## State machine

```text
contract_candidate
    -> adapter_conformance
    -> shadow_observation
    -> cutover_ready
    -> draining_world_local
    -> nakama_only_new_admission
    -> enclave_laboratory_only

Any unexplained divergence, incomplete drain, missing exact revision, or absent
rollback evidence -> blocked.
```

There is no automatic cross-generation live takeover. A World-local active
match is either completed on its original compatibility generation or closed by
an explicitly reviewed failure policy. Nakama does not silently reconstruct and
claim that match as canonical.

## Phase 0 — freeze

1. Freeze `trnm_world_rules_v1` schemas, canonical field order, error catalogue,
   maximums and golden vectors.
2. Bind the exact World and Nakama repository commits in the Integration release
   lock. The candidate lock in this repository is insufficient for activation.
3. Verify World contains no Nakama match-authority private key and constructs no
   `MatchCompletedV1`.
4. Verify the compatibility enclave is named and non-public in runtime config.
5. Record rollback owner, incident channel and start/end timestamps.

**Stop condition:** any schema, vector or error code changes after the lock is
approved require a new package release and a new conformance run.

## Phase 1 — adapter conformance

The Nakama adapter must:

1. decode only the published request schema;
2. reject unknown contract/ruleset/content revisions;
3. invoke the exact World deterministic rules package or an independently
   conforming implementation;
4. reproduce canonical request, result and transition hashes;
5. map unknown errors to `internal_contract_error` and fail closed;
6. keep admission, global ordering, idempotency and completion signing inside
   Nakama.

Minimum evidence:

- all committed World golden vectors;
- malformed state/command and unknown-version negatives;
- resource budget exceedance;
- deterministic rejection;
- identical input repeated across process restart;
- candidate output with an extra authority field rejected.

## Phase 2 — shadow observation

For every selected fixture or mirrored laboratory match, export one JSONL record
from World and one from the Nakama adapter. Run:

```bash
python3 tools/trnm-world-shadow-diff/trnm_world_shadow_diff.py \
  --world /evidence/world.jsonl \
  --candidate /evidence/nakama.jsonl \
  --summary /evidence/shadow-summary.json
```

Promotion requires:

- identical fixture sets;
- exact equality of request/state/command/result/replay/transition hashes;
- exact equality of disposition and stable error code;
- exact equality of deterministic resource-use counters;
- zero missing or unexpected fixtures;
- zero unexplained divergence.

A divergent record is never averaged, retried until it disappears, or manually
marked equivalent. It receives an owner, reproduction fixture and resolution.

## Phase 3 — cutover readiness

Before changing admission:

- Integration lock binds exact World/Nakama commits and release artifacts;
- all required CI runs are present for the exact commits;
- compatibility and rollback matrices are approved;
- dashboards distinguish World-local laboratory matches from Nakama matches;
- new World-local public admission is already disabled;
- active World-local match inventory is complete and timestamped;
- no settlement/outbox backlog is hidden by the cutover;
- operator access and break-glass actions are audited.

**No-go:** an empty check collection, stale evidence, or a branch-only result is
not a passing release gate.

## Phase 4 — drain

1. Stop creating new compatibility-enclave matches.
2. Preserve existing World-local matches on their original generation.
3. Monitor active count, terminal publication, settlement backlog and failed
   closed records.
4. Wait for active count to reach zero or apply the approved explicit terminal
   policy per match.
5. Archive the drain inventory and exact terminal outcomes.

Do not redirect in-flight commands or reuse World-local sequence numbers as
Nakama canonical sequence numbers.

## Phase 5 — Nakama-only admission

1. Enable new online admission only on the locked Nakama adapter revision.
2. Keep World deterministic rules endpoints private to the adapter network.
3. Verify Nakama is the only producer of canonical `MatchCompletedV1` evidence.
4. Verify World emits unsigned deterministic outcome/replay material only.
5. Sample and shadow-compare the first production candidate cohort.
6. Keep public online flags disabled until all independent public-network,
   support, moderation and release rows are green.

## Rollback

Rollback means stopping new Nakama admission and returning to a known disabled
or laboratory-only state. It does **not** mean making the compatibility enclave
public or rewriting Nakama canonical history.

Trigger rollback when:

- any unexplained deterministic divergence appears;
- contract revision or component lock differs at runtime;
- unknown errors are accepted or normalized as success;
- Nakama signs a completion not bound to the deterministic result;
- World receives authority private material;
- active drain or settlement evidence becomes incomplete;
- required exact-commit CI/evidence disappears.

Actions:

1. disable new online admission;
2. preserve all Nakama canonical event and completion records;
3. quarantine affected ruleset/content revisions;
4. keep World-local compatibility endpoints non-public;
5. capture World/Nakama inputs, outputs and hashes;
6. open a divergence incident and add a minimized fixture;
7. resume only with a new reviewed component lock and full shadow rerun.

## Evidence packet

Every rehearsal and cutover record includes:

- claim and runbook version;
- exact World/Nakama/Integration commits and package release;
- binary and schema digests;
- environment/topology;
- start/end timestamps;
- active-match drain inventory;
- shadow input and summary hashes;
- rollback trigger tests;
- limitations;
- independent reviewer decision.

This runbook does not establish Chain finality, public-edge readiness, custody,
commercial approval or human multiplayer usability.
