---
status: current-candidate
owner: trillionnium-world-release
contract: trnm_world_evidence_record_v1
last_reviewed: 2026-08-29
review_due: 2026-09-12
---

# Trillionnium World Evidence Record v1

## Purpose

An evidence record binds one narrowly scoped claim to exact source, artifacts, environment, observations and reviewer decision. It prevents narrative reports, stale runs and lower-class evidence from silently promoting release claims.

## Required fields

```json
{
  "schema": "trnm_world_evidence_record_v1",
  "claim_id": "...",
  "gate_id": "...",
  "evidence_class": "database_black_box",
  "scope": "...",
  "result": "pass|fail|invalid|inconclusive",
  "source": {
    "repository": "TrillionniumFoundation/Trillionnium-World",
    "commit": "40-lower-hex",
    "tree": "40-lower-hex",
    "clean": true
  },
  "artifacts": [],
  "toolchain": {},
  "component_lock": {},
  "environment": {},
  "timing": {},
  "thresholds": [],
  "observations": [],
  "faults": [],
  "raw_evidence": [],
  "limitations": [],
  "expiry": {},
  "review": {}
}
```

## Field semantics

### Identity

- `claim_id`: exact backlog/acceptance row.
- `gate_id`: release denominator gate.
- `evidence_class`: one allowed class from Plan v4.
- `scope`: exact statement proven; avoid broad terms such as “production ready”.
- `result`: only `pass` grants the scoped row, subject to reviewer acceptance.

### Source

- repository slug;
- 40-hex commit and tree;
- branch/PR for navigation only;
- clean-source assertion and source-manifest hash;
- submodule/vendor/component revisions where present.

### Artifacts

For each binary, image, package, schema or dataset:

- name/type;
- SHA-256;
- byte size;
- build ID/version;
- source manifest reference;
- signature/provenance reference;
- retention location.

### Toolchain

- Rust/Go/Python/Node versions as applicable;
- operating-system image digest;
- PostgreSQL and external service versions;
- dependency lock hashes;
- workflow/action revisions.

### Component lock

Cross-repository evidence lists exact World, Nakama, Chain, CEX and Integration revisions/artifacts. Missing components are explicit `not_applicable`, not omitted.

### Environment

- host/cloud/region identifiers in privacy-safe form;
- physical host count;
- CPU, memory, storage and network profile;
- database topology and durability settings;
- TLS/edge/security controls;
- injected latency/loss/failure profile;
- isolation from unrelated workloads.

### Timing

- UTC start/end;
- monotonic duration;
- timezone/source of wall-clock evidence;
- sample cadence;
- interruption/restart inventory.

### Thresholds and observations

Each threshold includes metric, operator, limit, unit and rationale. Observations include sample count, summary statistics, worst case and raw reference.

### Faults

Each injected fault records:

- phase and trigger;
- target component;
- expected durable state;
- observed durable state;
- recovery action/time;
- duplicate/loss invariants;
- cleanup result.

### Raw evidence

Every referenced file includes path/URI, SHA-256, size and content type. Generated summary hashes alone do not replace raw logs/data needed for review.

### Limitations and expiry

- untested topology/platform/feature;
- assumptions and exclusions;
- evidence expiry/review date;
- dependency revisions that invalidate the result when changed.

### Review

- reviewer ID and role;
- independence from author/runner where required;
- decision and timestamp;
- comments/conditions;
- approval scope;
- signature or immutable review reference.

## Validation rules

Evidence is invalid when:

- commit/tree/artifact/environment binding is missing;
- source is dirty without a complete reviewed patch manifest;
- result is partial but recorded as pass;
- a required dependency is stale or absent;
- raw artifacts are missing or hash-invalid;
- the evidence class is lower than the gate requires;
- the producer is also the only required independent reviewer;
- timestamps/duration are inconsistent;
- a failed/interrupted endurance run claims partial credit;
- automated data is used for human evidence;
- local/single-host evidence is used for public-network or cross-host claims;
- source status self-asserts server rules, custody or commercial approval.

## Recommended storage

```text
docs/evidence/<gate>/<claim>/<evidence-id>/record.json
acceptance/<gate>/<evidence-id>/raw/...
```

Large or sensitive raw evidence may live in controlled storage, but the repository record still carries immutable hashes, retention owner, access method and privacy classification.

## Privacy

Human and operational evidence follows data minimization:

- participant aliases rather than unnecessary identity;
- consent scope and revocation policy;
- no credentials, tokens, private keys or unnecessary personal data;
- redaction that preserves hash-bound original storage where review requires it;
- explicit retention/deletion owner.

## Promotion

A machine status row changes only after:

1. validator accepts the evidence schema and hashes;
2. dependencies and evidence class match the gate;
3. reviewer accepts the record;
4. exact claim/gate status is updated;
5. generated human-readable status is regenerated;
6. release owner makes any required explicit activation decision.