---
status: current-candidate
owner: trillionnium-world
work_items:
  - WORLD-P1-007
  - WORLD-P1-008
last_reviewed: 2026-08-29
review_due: 2026-09-12
---

# Trillionnium World Observability and SLO Contract v1

## Purpose

Observability must identify correctness degradation before readiness or release
claims remain green. Metrics/logs/traces are evidence inputs, not substitutes
for fault, restore, endurance, public-network, or human tests.

## Telemetry principles

- bounded labels and cardinality;
- monotonic counters for events, histograms for latency/age;
- exact build/release/instance/region profile as resource attributes;
- no token, private key, seed, password, full session, raw request body, or unnecessary personal data;
- readiness fails closed on authority, persistence, settlement, or migration uncertainty;
- alert owner, severity, threshold, runbook, and escalation are explicit.

## Core service indicators

### Native client

- real frame average and slowest-one-percent FPS;
- hard-stall duration/count;
- input-to-durable-ACK latency;
- network worker queue depth/errors;
- command journal pending/dead letters/recovery;
- save load/migration/corruption outcomes;
- economy pending/compensation/dead-letter age.

### Compatibility authority

- admission and active matches;
- actor warmup/running/terminalizing age;
- command ACK/effect latency;
- tick drift and publication lag;
- database pool/statement/lock saturation;
- checkpoint/terminal/fail-close success;
- journal hot/cold/manifest health;
- fleet lease/fence/drain state;
- WebSocket full/delta/resync/backpressure.

### Settlement

- capture candidates/success/failure/quarantine;
- pending/leased/retryable/remote-succeeded/pending-apply/applied/dead-letter;
- oldest eligible and pending-apply age;
- expired leases/takeovers;
- signer/CEX lookup/submit latency/status/ambiguity;
- malformed successful responses and 409 recoveries;
- quarantine scope/occurrences/age;
- operator policy/replay activity;
- receipt/application binding failures.

## Candidate SLOs

Numeric production targets require measured approval. Initial candidate classes:

| SLI | Candidate objective | Release effect |
| --- | --- | --- |
| readiness healthy | 99.9% over approved window | blocks admission when unhealthy |
| command ACK/effect | profile-specific p95/p99 and max | blocks online promotion |
| settlement pending-apply | no row beyond approved age | pages operator, blocks release |
| dead letters | zero unreviewed | pages immediately |
| expired active leases | zero beyond bounded takeover window | blocks settlement readiness |
| journal/ACK ambiguity | zero unresolved | blocks authority readiness |
| backup/WAL archive | zero unacknowledged failures | blocks production readiness |
| crash/OOM/restart | zero unexplained in endurance | invalidates endurance run |

Thresholds live in versioned operator policy/config and are recorded in evidence.

## Readiness versus liveness

- Liveness: process event loop can answer; never implies safe admission.
- Readiness: all required authority/database/journal/signer/CEX/profile checks are
  fresh, fenced, and within budgets.
- Draining: liveness remains, new admission/claim stops, in-flight work is bounded.
- Fatal/poisoned: readiness false until reviewed recovery; restart loops do not
  convert uncertainty into health.

## Alerts

Minimum paging alerts:

```text
authority fence lost or stale
migration checksum/name drift
terminal publication/journal ambiguity
settlement dead letter
pending-apply age breach
expired lease/takeover breach
quarantine recurrence or count breach
signer key/registry mismatch
CEX lookup unavailable during ambiguity
backup/WAL failure
old-primary activity
public-edge/capacity saturation
```

Every page links to a tested runbook. Acknowledgement without resolution does
not clear the underlying readiness condition.

## Evidence retention

Raw run artifacts include:

- metric snapshots/time series;
- structured logs with redaction validation;
- fault timeline;
- process/cgroup/database/network metadata;
- exact commit/tree/binary/toolchain/config hashes;
- final decision and limitations.

Artifacts are checksummed and retained according to evidence class and operator
policy.

## Acceptance

- instrumentation exists at every durable/public boundary;
- labels remain bounded under adversarial IDs;
- readiness changes are tested by fault injection;
- alerts fire and clear under drills;
- dashboards separate compatibility authority from target Nakama and public profiles;
- 24-hour and public evidence consume the same production telemetry paths.
