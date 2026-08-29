---
status: current-candidate
owner: trillionnium-world
work_items:
  - WORLD-P1-007
  - WORLD-P2-001
last_reviewed: 2026-08-29
review_due: 2026-09-12
---

# Trillionnium World Backup, PITR, and Restore Contract v1

## Scope

This contract covers PostgreSQL authority/settlement data, host-local authority
journals, replay/evidence artifacts, release manifests, and operator evidence.
It does not claim a deployed backup service; deployment evidence remains open.

## Data classes

| Class | Examples | Recovery requirement |
| --- | --- | --- |
| canonical target online | Nakama roots/completion | owned outside World |
| compatibility authority | match, command, checkpoint, terminal ACK | consistent DB + journal lineage |
| campaign/economy | campaigns, intents, receipts, outbox, quarantine | no duplicate value; exact receipt lineage |
| replay/evidence | replay chunks, cold witnesses, run artifacts | integrity hashes and retention |
| secrets | DB/service credentials, signer seed | never in ordinary backup; separate custody |
| release | source/tree/toolchain/binary/SBOM/signature | immutable/checksummed |

## Objectives

Production profiles declare numeric RPO/RTO per class. Source or single-host
laboratory evidence cannot claim cross-host RPO=0. Until measured and approved,
status remains blocked.

## PostgreSQL backup

Required production shape:

- encrypted full/base backups;
- continuous WAL archive with monitored success;
- immutable retention tiers;
- backup catalogue binding database system identifier and timeline;
- checksums and restoration verification;
- separate credentials and storage role;
- capacity to retain settlement/operator evidence for the approved period.

Backups exclude unnecessary plaintext credentials and private signing material.

## Restore procedure

1. Declare incident/change ticket and stop admission/writers.
2. Capture old primary identity, timeline, last known durable cursors, and artifact hashes.
3. Restore base backup and WAL to the selected recovery target.
4. Record new database system identifier/timeline/postmaster start.
5. Prevent old-primary network and credential access.
6. Apply only checksum-matching migrations; never edit applied SQL.
7. Run schema, FK, unique, procedure, privilege, and state-invariant checks.
8. Reconcile authority journal hot/cold witnesses with restored DB lineage.
9. Reconcile settlement jobs by stable remote identity using lookup-before-submit.
10. Quarantine unknown/ambiguous rows; never infer success from local state alone.
11. Start read-only probes, then one fenced writer, then bounded admission.
12. Record RPO/RTO, data loss window, limitations, and independent approval.

## Old-primary fencing

The old primary must be unable to:

- retain database advisory locks or credentials;
- route player admission;
- publish checkpoints/terminal results;
- claim settlement work;
- write archived WAL into the new lineage;
- serve stale readiness as healthy.

Host/database authority identity binds system identifier, timeline, postmaster
start, physical host, instance ID/epoch, backend PID/start, and owner nonce.

## Settlement restore invariants

- `remote_request_id` and intent hash are immutable across restore/recapture.
- A remotely committed receipt recovered by lookup is not resubmitted.
- `remote_succeeded` remains separate from `campaign_applied_at`.
- Applied campaign state has exact receipt and capture lineage.
- Dead-letter/operator/quarantine evidence is retained and append-only.
- Restore never resets attempts, leases, receipt identities, or operator history
  through direct SQL outside a reviewed procedure.

## Journal and replay

Journal/cold-witness restore requires:

- exact manifest version, sequence/count/latest sentinel;
- file type/mode/owner and checksum validation;
- database lineage/timeline binding;
- no overlap/gap outside the documented recovery algorithm;
- quarantine of historical terminal rows lacking exact full-tuple ACK;
- explicit retention proof before deleting cold evidence.

## Restore drills

Required matrices:

```text
latest successful WAL
selected point before remote receipt
selected point after remote receipt before campaign apply
timeline promotion
old primary isolated late
journal ahead of DB
DB ahead of journal
corrupt/incomplete WAL or manifest
operator evidence retention
```

Each drill uses exact immutable artifacts, records injected failure, verifies no
duplicate settlement, and emits a final checksummed summary.

## Acceptance

- scheduled backups and WAL archive are monitored;
- restore is successfully rehearsed on a separate host/environment;
- old-primary isolation is demonstrated, not assumed;
- declared RPO/RTO are measured;
- settlement ambiguity converges through exact lookup;
- evidence retention and deletion policy are approved;
- current release status references the exact restore evidence.
