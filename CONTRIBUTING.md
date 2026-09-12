# Contributing to Trillionnium World

## Read before changing source

1. `AGENTS.md`
2. `PROJECT_BOUNDARY.md`
3. `CURRENT_PLAN.md`
4. `docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-29.md`
5. relevant ADR, protocol, database, security, and runbook documents

Run:

```bash
bash scripts/project-preflight.sh --dev
```

Stop on a repository, branch, lane, remote, dependency, or boundary mismatch.

## Branch and PR rules

- use one lane-prefixed branch matching repository policy;
- never develop directly on `main`;
- keep one coherent ownership/evidence scope per PR;
- do not merge your own PR;
- request independent review for authority, value, security, protocol, migration, workflow, and release changes;
- do not mix production activation or public-market enablement into an engineering refactor;
- keep limitations and external blockers explicit.

## Authority rules

World owns deterministic game behavior and unsigned game-domain material.
Nakama owns target canonical online admission/order/recovery/completion, Chain
owns finality, CEX owns wallet/ledger/custody, and Integration owns cross-repo
component locks/evidence.

Do not create dual authority, proxy another system's private key, or describe a
World hash as admission/order/archive/finality/settlement proof.

## Correctness rules

- rejected commands are state-preserving unless a typed committed error contract says otherwise;
- external remote I/O never runs while mutable game rows are locked;
- retries reuse immutable identities;
- stale generation/lease owners cannot mutate durable state;
- canonical serialization and hashes are versioned and independently tested;
- migrations are append-only/checksum-bound;
- secret values never enter source, logs, artifacts, issues, or PR bodies;
- queues, bodies, collections, tasks, retries, diagnostics, and timeouts are bounded.

## Tests and evidence

Run the applicable local gates. The exact PR head must also pass:

```text
trnm-world-v4/docs-governance
trnm-world-v4/transition-contract
trnm-world-v4/settlement-postgres
trnm-world-v4/game-workspace-release
trnm-world-v4/supply-chain
```

CI is read-only. It may not patch, commit, push, tag, merge, deploy, or promote
source. Empty, skipped, stale, cancelled, or base-only checks are blockers.

Evidence records bind exact source/tree/binary/toolchain/environment, thresholds,
raw artifact hashes, limitations, and independent reviewer. Automated evidence
cannot satisfy human/public-network/custody/legal/commercial rows.

## Documentation

Update current docs, schemas, vectors, machine status, tests, and runbooks with
the same boundary. Current documents use metadata with status, owner, work item,
review date, and due date. Historical material must be clearly archived.

## Commit and review hygiene

- focused, signed-off commits;
- no probe, temporary, generated junk, or unexplained binary changes;
- no large semantic behavior hidden in build-time source rewriting;
- no stale comments or claimed test results;
- include rollback/disablement for migrations, authority, security, and deployment changes;
- resolve conversations and re-request review after material changes.

## Licensing and third-party material

Do not add code, art, audio, maps, fonts, data, or generated content without a
clear license/provenance record compatible with repository policy. Preserve
third-party notices and update the release license inventory/SBOM.

## Reporting security issues

Follow `SECURITY.md`; never disclose live exploit details or credentials in a
public issue.
