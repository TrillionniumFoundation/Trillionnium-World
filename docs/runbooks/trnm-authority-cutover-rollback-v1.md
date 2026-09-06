---
status: current-candidate
owner: trillionnium-integration
contributors:
  - trillionnium-world
  - trillionnium-nakama
applies_to_plan: trillionnium-world-development-2026-08-29-v4
last_reviewed: 2026-08-29
review_due: 2026-09-12
---

# World-to-Nakama Authority Cutover and Rollback v1

## Objective

Move canonical online admission, order, idempotency, recovery, archive roots and completion signing from the World-local compatibility enclave to Nakama without dual authority or assumed live takeover.

## Non-goals

This runbook does not:

- make World a public online authority;
- prove Chain finality or CEX settlement;
- authorize public launch;
- allow one active match to be owned by both generations;
- treat completed-match compatibility as live takeover evidence.

## Required component lock

The cutover candidate binds:

- World repository commit/tree, transition contract/schema/vector revision and artifact hash;
- Nakama repository commit/tree, adapter/runtime binary/image and completion key ID;
- Integration harness revision and evidence schema;
- CEX and Chain interface revisions where exercised;
- deployment configuration, database schemas and environment/topology.

Any component change invalidates the candidate unless the lock is regenerated and all dependent evidence reruns.

## Pre-cutover gates

1. World strict canonical transition contract exact-head checks pass.
2. Nakama adapter independently reproduces all positive/negative vectors.
3. Shadow corpus has zero unexplained accept/reject/state/replay/outcome/hash divergence.
4. Representative load, restart, response-loss and rejection paths pass.
5. Nakama alone holds completion-signing private material.
6. World-local admission/completion endpoints have an explicit disable switch.
7. Active World-local match inventory and maximum drain time are known.
8. Rollback release and configuration are immutable and deployable.
9. Monitoring, incident owner and change window are approved.
10. Public-online and player-market flags remain disabled.

## Authority profiles

```text
offline_world_v1
world_legacy_local_alpha_v1
nakama_shadow_v1
nakama_canonical_v1
```

Every request/log/evidence record carries exactly one profile. `nakama_shadow_v1` output is noncanonical and cannot produce completion or settlement effects.

## Shadow phase

For each canonical World-local laboratory command corpus item:

1. normalize exact prior state and command under the World contract;
2. run local World adapter and Nakama adapter;
3. compare exact accepted/rejected bytes;
4. compare error code/retryability/detail bounds;
5. compare next tick/state/replay/outcome bytes and hashes;
6. compare CPU/time/memory/resource budgets;
7. classify divergence.

Divergence classes:

- input/component-lock mismatch;
- canonical encoding mismatch;
- rules/content mismatch;
- deterministic implementation defect;
- unsupported behavior;
- unexplained.

Any unexplained divergence blocks promotion.

## Drain strategy

Default strategy is drain, not takeover.

1. Stop admitting new canonical matches to World-local enclave.
2. Keep existing enclave matches on their original owner/generation.
3. Allow them to finish or enter a reviewed fail-close/abandonment path.
4. Preserve replay, terminal and settlement evidence.
5. Verify active enclave count reaches zero.
6. Only then disable canonical World-local admission endpoints.

A live takeover is allowed only after a separate matrix proves exact state, command gap, generation, archive and settlement continuity across every crash point.

## Cutover procedure

1. Freeze the exact component lock and evidence bundle.
2. Verify rollback artifacts and configuration.
3. Set World-local admission to drain-only.
4. Verify no new World-local match is created.
5. Wait for active count zero and terminal/settlement obligations resolved or explicitly quarantined.
6. Enable Nakama canonical admission for a bounded canary cohort.
7. Verify:
   - participant roles and controller sets;
   - canonical sequence/idempotency;
   - World transition contract use;
   - reconnect/restart recovery;
   - canonical archive roots;
   - Nakama-only `MatchCompletedV1` signature;
   - no World-local completion signature or public admission;
   - exact observability/component-lock labels.
8. Expand only under predefined error/divergence/capacity thresholds.
9. Record final authority inventory and disablement state.

## Stop conditions

Immediately halt expansion when:

- any dual admission/order/root/signature is observed;
- unexplained deterministic divergence occurs;
- Nakama cannot recover canonical state after restart;
- World receives or exposes Nakama private material;
- active World-local matches are silently reassigned;
- completion evidence omits exact World transition facts/component lock;
- settlement or Chain consumers receive crossed authority evidence;
- required monitoring/evidence is missing.

## Rollback modes

### Before Nakama canonical match creation

Disable Nakama admission and restore prior World-local laboratory configuration. No canonical match migration is involved.

### After Nakama canonical matches exist

Rollback does **not** transfer those active matches to World by default.

- stop new Nakama admission;
- keep existing Nakama matches on Nakama if safe;
- repair/redeploy Nakama or use its reviewed fail-close/abandonment path;
- restore World-local only for new laboratory/nonpublic matches when explicitly approved;
- do not create duplicate completion or settlement effects.

### Contract rollback

Nakama may select a prior compatible World contract only when:

- Integration lock declares compatibility;
- prior World artifact remains available and verified;
- schema/vector and state migration policy permit it;
- active matches are drained or version-pinned;
- no hidden fallback to sibling source exists.

## Disablement verification

After successful cutover:

- World-local public bind/admission routes are disabled or laboratory-network restricted;
- source/status/UI labels identify compatibility enclave correctly;
- no World process has match completion key access;
- Nakama is sole canonical completion issuer;
- old protocols have usage inventory and retirement date;
- rollback artifacts remain retained for the approved window.

## Required evidence

- exact component lock and signatures/hashes;
- shadow corpus and divergence ledger;
- active-match inventory/drain timeline;
- configuration snapshots before/after;
- canary and expansion metrics;
- restart/reconnect/fault raw artifacts;
- canonical completion samples and key ID;
- negative proof that World cannot sign/claim completion;
- rollback rehearsal;
- independent World/Nakama/Integration reviewer decisions.

## Release effect

Successful cutover may create a `closed_online_nakama` candidate. It does not grant public-network, cross-host, custody, human, commercial or public-market approval.