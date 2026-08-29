---
status: current
owner: trillionnium-world-release
last_reviewed: 2026-08-29
---

# Trillionnium World Release Evidence Contract v1

## 1. Purpose

Prevent source, local, deployed, human and commercial claims from being mixed.
Every release claim is a machine-readable evidence row with an explicit
required evidence kind.

## 2. Evidence row

Each row contains at least:

```json
{
  "claim_id": "WORLD-...",
  "claim": "bounded statement",
  "evidence_kind": "source|unit|local_blackbox|deployed_single_host|deployed_cross_host|public_network|human|custody_approval|commercial_approval",
  "state": "pending|failed|passed|expired|revoked",
  "repository": "owner/name",
  "commit": "40-hex",
  "tree": "40-hex",
  "artifacts": [{"name": "...", "sha256": "64-hex", "size": 1}],
  "toolchain": {"rust": "...", "postgres": "..."},
  "environment": {"topology": "...", "host_ids": ["hashed-id"]},
  "started_at": "RFC3339",
  "ended_at": "RFC3339",
  "thresholds": {},
  "measurements": {},
  "raw_evidence": [{"path": "...", "sha256": "64-hex"}],
  "limitations": [],
  "reviewer": "independent-principal",
  "reviewed_at": "RFC3339",
  "expires_at": "RFC3339|null"
}
```

## 3. Fail-closed rules

A row is not passed when:

- check collection is empty, skipped, cancelled or missing;
- commit/tree/artifact/toolchain/environment identity is missing or mismatched;
- raw evidence is absent, mutable or hash-invalid;
- thresholds were changed after the run without a new claim revision;
- required duration/topology/participant count is incomplete;
- reviewer is the sole author/operator for an independent-review row;
- evidence is stale, expired, revoked or belongs to another component lock;
- automated evidence is offered for human/public/commercial approval;
- same-host evidence is offered for cross-host recovery;
- a partial endurance attempt is offered as a complete duration pass.

## 4. Gate aggregation

A product gate is green only when every dependency row is passed and unexpired.
Aggregation cannot upgrade an evidence kind. The status renderer must show the
weakest missing dependency and preserve all limitations.

## 5. Required release groups

### Technical alpha

Exact-head source/unit/static checks, deterministic regression and package
shape. No public or settlement custody claim.

### Commercial single-player

Technical alpha plus multi-OS signed distribution, human usability,
accessibility, crash recovery, support/privacy/legal approval.

### Trusted settlement

Transaction-free source plus deployed signer/CEX ambiguity, duplicate,
process-kill, rollback/PITR, reconciliation, retention, operator and custody
approval.

### Closed/public online

Nakama canonical authority, Integration lock, multi-host recovery, endurance,
public edge/capacity, multiplayer human and moderation/support evidence.

### Public player market

All online and trusted-settlement rows plus custody, listing ownership, fraud,
dispute, chargeback, support, privacy and legal approval. Enablement requires an
explicit separately reviewed activation change.

## 6. Evidence lifecycle

- `pending` until the exact run exists;
- `passed` only after independent review;
- `failed` preserves raw artifacts and reason;
- `expired` when time/environment/component validity ends;
- `revoked` when compromise, invalid methodology or component drift is found.

Evidence deletion follows retention and legal policy; status removal never
erases audit history.
