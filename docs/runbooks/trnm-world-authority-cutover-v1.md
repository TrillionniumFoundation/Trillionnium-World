---
status: current
owner: trillionnium-world
applies_to:
  - WORLD-P0-003
  - WORLD-P0-004
  - authority-cutover
last_reviewed: 2026-08-28
review_due: 2026-09-11
---

# World-local authority → Nakama cutover and rollback runbook v1

## Scope

This runbook governs the transition of new online match authority from the
World-local `legacy_local_alpha` compatibility enclave to Nakama. It does not
permit live active-match takeover without a separately reviewed takeover
matrix.

## Preconditions

All must be true before a cutover rehearsal:

1. exact World runtime contract source manifest is green;
2. independent Nakama consumer reproduces all valid and invalid vectors;
3. Integration locks exact World, Nakama, schema, binary and environment
   revisions;
4. promoted fixture and load matrices have zero unexplained divergence;
5. World and Nakama private credentials are distinct and World has no Nakama
   completion-signing key;
6. Nakama completion signing and canonical archive ownership are operational;
7. World compatibility endpoints can be disabled independently;
8. rollback does not create dual admission or dual signing.

Missing CI, an empty check set or partial evidence is a failed precondition.

## Phase 1 — source and fixture freeze

- freeze exact runtime schema, error catalogue, ruleset/content digests and
  shadow budgets;
- record World/Nakama/Integration commit, tree, binary and toolchain digests;
- reject any unreviewed vector or digest change;
- preserve the World-local server as laboratory-only.

Exit: immutable component lock and reproducible fixture packet.

## Phase 2 — unsigned shadow

- Nakama remains canonical for the test lane's admission and ordering;
- send the already ordered game-domain batch to both deterministic consumers;
- collect `trnm_world_runtime_observation_v1` from each implementation;
- compare with `trnm-world-runtime-shadow-diff`;
- persist both observations, report, environment and raw metric hashes;
- stop on the first unexplained divergence.

World output remains unsigned and cannot become canonical completion evidence.

Exit: zero unexplained fixture and approved load divergence.

## Phase 3 — drain compatibility admission

- set World compatibility admission to `draining`;
- reject creation of new canonical matches in the World-local enclave;
- allow already active compatibility matches to complete under their original
  generation only;
- enumerate active matches and verify no hidden admission path remains;
- do not migrate live actor state to Nakama by assumption.

Exit: active compatibility-match count is zero, or an independently approved
cross-generation takeover matrix exists.

## Phase 4 — canonical cutover

- enable new online admission only in Nakama;
- verify Nakama owns participant framing, global sequence, idempotency,
  recovery, canonical roots and `MatchCompletedV1` signing;
- keep World deterministic execution behind the exact contract selection;
- disable or firewall World-local public authority endpoints;
- retain explicit laboratory access only through a named non-public profile.

Exit: one and only one canonical admission/order/root/signature owner.

## Rollback

Rollback is allowed only before dual authority can occur.

1. stop new Nakama admission;
2. preserve all already admitted Nakama matches under Nakama until drained or
   explicitly abandoned by its authority rules;
3. do not reopen World admission for those match identities;
4. re-enable the World laboratory only for new isolated test identities;
5. retain all divergence and cutover evidence;
6. require a new component lock before another rehearsal.

Rollback must never move the same live match identity between two canonical
owners or allow both systems to sign completion.

## Immediate abort conditions

- World loads, proxies or derives a Nakama completion-signing key;
- both systems accept a new canonical match identity;
- active-match takeover is inferred from completed-match equality;
- shadow input bindings differ;
- material or error-code divergence is unexplained;
- candidate resource budgets fail;
- World endpoints cannot be disabled independently;
- evidence is stale, partial or not bound to exact revisions.

## Evidence packet

Record:

- claim and rehearsal ID;
- exact World/Nakama/Integration source and binary identities;
- schema/vector/error-catalog digests;
- environment/topology;
- shadow observations and reports;
- active-match inventory before/after drain;
- endpoint and key-ownership checks;
- timestamps, limitations and reviewer decisions;
- rollback result.

A successful source-only rehearsal grants no public-online, cross-host,
commercial or public-market credit.
