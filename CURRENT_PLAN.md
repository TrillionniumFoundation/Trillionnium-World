# Current Trillionnium World Plan

The executable product foundation remains:

`docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-29.md`

The historical machine inputs retained for provenance only are:

- `docs/development/trillionnium-world-development-plan-2026-08-29.json`
- `docs/development/trnm-world-gap-closure-ledger-v4.json`

They are not execution authority and cannot select, qualify, merge, release, deploy, or authorize a current object.

## Current truth and evidence authorities

Read the current sources in this order:

1. `PROJECT_BOUNDARY.md` and `PROJECT_BOUNDARY.json` — binding repository and authority ownership.
2. `CURRENT_PLAN.md` — operative PR/branch and blocker ordering.
3. `docs/catalog.json` — machine document lifecycle and truth ordering.
4. `docs/status/world-plan-v4-execution-truth-2026-09-08.json` — selected historical execution observation for the current PR/branch identity.
5. `docs/status/CURRENT.md` — deterministic rendering of that selected observation.
6. `RELEASE_READINESS.md` — repository-level release decision.
7. `docs/development/trnm-world-gap-closure-ledger-v6.json` — current denominator, owner, and fail-closed state ledger.
8. `docs/integration/trnm-world-cex-current-pending-lock-v2.json` — coordination-only CEX observation.
9. `docs/release/trnm-world-external-execution-board-v1.json` — external execution inventory and evidence limits.

The authoritative current execution snapshot is:

- `docs/status/world-plan-v4-execution-truth-2026-09-08.json`

<!-- trnm-current-execution-snapshot: docs/status/world-plan-v4-execution-truth-2026-09-08.json -->

The selected snapshot is a dated, immutable observation. It does not certify the current branch head, current prospective merge, current checks, current review, or current external component identities.

## Operative candidate

- Repository: `TrillionniumFoundation/Trillionnium-World`
- Pull request: `#104`
- Branch: `fix/world-convergence-v6-20260908`
- Merge target: `main`
- Current candidate toolchain: `1.98.1`
- Historical qualification toolchain: `1.98.0`

### Live object rule

Tracked source cannot certify its own current Git head: committing such a value changes the head and makes the assertion stale. The current head, tree, actual GitHub prospective-merge object, checks, review state, and branch-control state must therefore be read from the live pull request and GitHub control plane, then bound by an immutable check artifact or independently retained evidence record.

The PR body may render the latest live tuple for review, but it is not execution evidence. Any base, head, tree, merge object, workflow definition, component lock, or review movement invalidates object-bound credit and requires fresh read-back and non-empty execution.

PR #104 supersedes earlier World convergence and ownership-migration review surfaces. Their source history remains provenance; their CI, review, governance, release, or production credit does not transfer.

## Binding architecture decisions

- **World** owns deterministic game-domain behavior, authored content, native client behavior, World aggregate mutation under one fenced writer epoch, player-facing economic intents, World outcome hashes, and unsigned replay/outcome material.
- **Nakama** owns target online admission, identity, canonical total order, durable online idempotency, reconnect/restart recovery, archive roots, and `MatchCompletedV1` signing.
- **CEX** owns wallet/ledger settlement and custody.
- **Chain** owns ingress, consensus, inclusion, and finality.
- **Integration** owns exact cross-repository component locks, compatibility matrices, and release evidence.
- `trillionnium/crates/trnm-game-server` remains the bounded `world_legacy_local_alpha` compatibility enclave.
- `trillionnium/crates/world-authority` remains an isolated World-domain cutover candidate. Fixture adapters and the file-backed server are non-production.
- External settlement follows capture -> transaction-free remote execution -> fenced apply. Signer, CEX, or network I/O while mutable World rows are held is prohibited.
- CI may validate and upload immutable evidence but may not move refs, self-approve, synthesize statuses, dismiss substantive review, bypass protection, tag, release, or deploy.

ADR `docs/adr/0003-world-domain-authority-and-nakama-canonical-online.md` remains binding. A World-domain service cannot become a second canonical online authority.

## Current repository interpretation

The current candidate contains ordinary Git-tracked correctness source, detailed contracts for the active game-product and isolated World-authority crates, strict bounded JSON parsing, closed-world component/document inventories, PostgreSQL fencing/CAS contracts, transition vectors, settlement boundaries, and read-only exact-head/merge qualification definitions.

These are source-side `closed_candidate` facts only. Documentation structure, workflow definitions, local fixtures, and historical artifacts do not imply implementation conformance, hosted execution, protected admission, independent acceptance, deployment, custody, human validation, commercial approval, or production authorization.

### Native Actions blocker

Repository-native job creation remains an external GitHub control-plane blocker. Canonical PR #104 and the independent protected-main bootstrap PR #124 both contain unconditional read-only pull-request workflows, but live API read-back must show non-empty runs, jobs, and steps before any check receives credit.

The accountable remediation record is issue #58. Repository or organization administrators must inspect the effective Actions enablement/allowlist, billing or quota suspension, hosted/self-hosted runner availability, event restrictions, and any enterprise or ruleset suppression. A synthetic commit status, external status injection, reduced gate, or protection bypass is prohibited.

## Current dependency interpretation

Exact external candidate identities are deliberately not duplicated in this tracked plan. Current observations live only in their versioned machine records and must be refreshed by the accountable Integration process:

- World↔CEX coordination: `docs/integration/trnm-world-cex-current-pending-lock-v2.json`
- Cross-repository execution inventory: `docs/release/trnm-world-external-execution-board-v1.json`
- Repository denominator ledger: `docs/development/trnm-world-gap-closure-ledger-v6.json`

A coordination pin is not component qualification. Any external repository movement requires a fresh immutable Integration lock and fresh contract-byte, fault, cutover, and rollback evidence.

## Ordered remaining blockers

1. Restore GitHub repository-native job creation and obtain non-empty terminal-success truth, source, Rust 1.98.1, PostgreSQL, transition, documentation, package, boundary, and supply-chain execution for one unchanged exact World head and actual prospective merge.
2. Read back and enforce protected-main required contexts, review freshness, stale-review dismissal, conversation resolution, linear history, force-push/deletion denial, and no-bypass policy; then obtain a fresh eligible non-author approval on the unchanged final tuple.
3. Complete all remaining classified `include!` ownership migrations into real Rust module/trait/visibility boundaries through compile/test-preserving review tranches.
4. Replace fixture/file-backed World authority adapters with independently reviewed production identity, session, account, PostgreSQL repository, ledger, evidence, metrics, internal-routing, and deployment-identity implementations.
5. Independently qualify World, CEX, Nakama, and Chain; have Integration pin exact accepted commits, trees, artifacts, and contract bytes and close divergence, response-loss, restart, failover, no-dual-writer, cutover, rollback, and disablement matrices.
6. Obtain deployment, public-edge, cross-host recovery/endurance, KMS/HSM custody, accessibility, privacy, licensing, legal, support, commercial, and final accountable human go/no-go evidence.

Public online operation, public player markets, trusted settlement, and commercial release remain **NO-GO / disabled**. Production authorization remains **not granted**.

## Executable truth and integrity checks

```bash
python3 scripts/check-trnm-world-execution-truth.py
python3 scripts/test-trnm-world-execution-truth.py
python3 scripts/check-trnm-world-machine-truth.py
python3 scripts/test-trnm-world-machine-truth.py
python3 scripts/check-trnm-world-active-closure.py
python3 scripts/test-trnm-world-active-closure.py
python3 scripts/check-trnm-world-ci-integrity.py
python3 scripts/test-trnm-world-ci-integrity.py
python3 scripts/check-trnm-world-current-conformance.py
python3 scripts/test-trnm-world-current-conformance.py
python3 scripts/check-trnm-world-detailed-documentation.py
python3 scripts/check-trnm-world-authority-documentation.py
python3 scripts/check-trnm-world-cex-pending-lock.py
bash scripts/run-trnm-world-v6-qualification-v2.sh truth
```

`docs/status/CURRENT.md` is a deterministic rendering of the selected dated snapshot, not a fresh GitHub query. A local operator may regenerate it with `--write` only after an authorized snapshot/pointer update; CI rejects that option. Neither generation nor a status file closes evidence.

Production authorization remains **not granted**.
