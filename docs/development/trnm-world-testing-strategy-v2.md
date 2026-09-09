---
status: current-candidate
owner: trillionnium-world
work_items:
  - WORLD-P1-007
  - WORLD-P1-008
  - WORLD-P2-002
last_reviewed: 2026-08-29
review_due: 2026-09-12
---

# Trillionnium World Testing Strategy v2

## Principle

Tests are organized by **claim and evidence class**, not by the number of
scripts. A lower evidence class never satisfies a higher one by implication.

## Evidence ladder

| Class | What it may prove | What it cannot prove |
| --- | --- | --- |
| `source_static` | ownership, forbidden calls, schemas, pinned config | compilation or runtime behavior |
| `unit` | pure state-machine and parser invariants | PostgreSQL/network/process behavior |
| `database_black_box` | transactions, leases, CAS, migrations, SQL privileges | deployed topology or external systems |
| `single_host_runtime` | real processes, signals, sockets, local recovery | cross-host/public network |
| `cross_repository_integration` | exact component compatibility | regional/public capacity |
| `cross_host` | fencing, failover, backup/restore across hosts | Internet edge and human usability |
| `public_network` | TLS/WAF/DDoS/capacity under public routing | player comprehension or commercial approval |
| `human` | usability, accessibility, comprehension | custody/security/legal approval |
| `custody_security` | KMS/HSM, secret roles, fraud controls | commercial/legal approval |
| `commercial_legal` | launch authorization for exact scope | technical correctness outside evidence scope |

## Required PR checks

The v4 candidate exposes five exact contexts:

```text
trnm-world-v4/docs-governance
trnm-world-v4/transition-contract
trnm-world-v4/settlement-postgres
trnm-world-v4/game-workspace-release
trnm-world-v4/supply-chain
```

All run against the exact PR head. Empty, skipped, cancelled, stale, or
base-only checks are failures to prove.

## Deterministic transition tests

Required:

- strict JSON grammar and exact re-encoding;
- duplicate/unsorted key rejection;
- signed-i64-only numeric profile;
- escape and Unicode canonicality;
- depth and byte budgets;
- recursively decoded authority-key rejection;
- fixed SHA-256 and domain-separated hashes;
- Rust and independent implementation agreement;
- changed command/state/revision changes the appropriate hash;
- unknown revisions and malformed outputs produce stable codes;
- repeated identical input produces byte-identical result.

Promotion additionally requires Nakama shadow comparison over representative
accepted/rejected/load/restart corpora with zero unexplained divergence.

## Settlement tests

### Source/unit

- game server cannot perform settlement remote I/O;
- blocking HTTP feature absent;
- capture, execute, and apply ownership separated;
- malformed successful response is ambiguous/retryable;
- shutdown stops new capture/claim admission;
- bounded in-flight and drain semantics;
- quarantine and operator APIs present;
- duplicate campaign job rejected.

### PostgreSQL black box

- capture/job invisible before transaction commit;
- two claimers obtain at most one live lease per job and serialization key;
- stale/expired lease cannot authorize, complete, retry, dead-letter, or apply;
- one `(capture_id,campaign_id)` job maximum;
- poison claimed job is fenced into dead letter and quarantine;
- quarantine retry time suppresses hot-loop scanning;
- operator resolution is privileged, audited, and exact-identity-bound;
- remote success and campaign apply remain separate;
- campaign revision/state hash CAS rejects drift;
- migrations 16–19 are checksum-bound and idempotent.

### Process/network fault matrix

Inject at every phase boundary:

```text
capture before/after commit
claim before/after lease
signer before send / after commit before response
CEX before send / after commit before response
receipt before/after durable store
apply before first campaign write / between campaigns / before commit
graceful SIGTERM
SIGKILL
future cancellation
PostgreSQL restart/failover/PITR
old-primary isolation
```

The invariant is convergence without duplicate remote value or partial local
progression.

## Campaign and RTS state preservation

Every rejected command must satisfy:

```text
hash(state_after_error) == hash(state_before_command)
```

unless the error contract explicitly returns a typed, committed state change.
Property/regression coverage includes:

- queue full and idempotency tombstones;
- purchase reserve failure;
- daily issuance limits;
- receipt mismatch;
- PvP authority partition failure;
- construction/resource failure;
- replay/event counters;
- cooldown/guard/resource/random cursors.

## Packaging and supply chain

Required:

- locked metadata, format, all-target tests and Clippy;
- release build of all declared binaries;
- package path/link/mode/hash/size verification;
- exact source/tree/toolchain manifest;
- SBOM and license inventory;
- RustSec/dependency policy;
- SHA-pinned Actions and read-only permissions;
- no validation workflow may modify, commit, push, tag, merge, or promote source.

## Endurance and capacity

A 24-hour row is valid only when:

- exact immutable binaries and environment are bound;
- the run reaches 24 hours and emits a final summary;
- no excluded workload shares the authority resource domain;
- thresholds and cleanup are predeclared;
- process/database/network/system metrics are retained;
- partial duration receives no 24-hour credit.

## Human evidence

Automation cannot satisfy human rows. Required sessions include:

- three independent five-second observers;
- one non-developer unguided 10–15 minute vertical slice;
- keyboard-only, mouse-only, high-contrast, subtitles, low-motion, compact and
  wide viewport coverage;
- multiplayer reconnect/failure comprehension after Nakama cutover.

Records bind consent, anonymous participant, exact binary, platform, timestamps,
raw artifact hashes, limitations, and independent reviewer decision.

## Evidence record

Every accepted record contains:

- claim ID and evidence class;
- exact commit/tree/binary/toolchain/component digests;
- environment/topology;
- start/end time;
- fault injection and thresholds;
- raw artifact hashes;
- limitations and expiry;
- independent reviewer.

A status file may reference evidence; it may never create evidence by assertion.
