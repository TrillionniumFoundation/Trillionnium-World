---
status: current
owner: trillionnium-world
last_reviewed: 2026-08-29
---

# Trillionnium World Gap-Closure Runbook v1

## 1. Start of every run

1. Fetch and record `main`, package branch, PR head and Git tree.
2. Read `PROJECT_BOUNDARY`, `CURRENT_PLAN`, V4 plan, gap registry and applicable
   ADRs.
3. Run `bash scripts/project-preflight.sh` in the real checkout.
4. Verify one isolated lane-prefixed branch and one PR.
5. Stop on base drift, denied path, unexpected external dependency or authority
   contradiction.

## 2. Select work

Choose the highest-priority gap that:

- is owned by this repository;
- has all source dependencies present;
- can be validated in the available environment;
- does not require production activation, external human evidence or another
  repository's private authority.

Do not start P1/P2 feature breadth while a repository-owned P0 safety gap is
open.

## 3. Implementation loop

For each gap:

1. State the invariant and failure modes.
2. Add positive and negative tests before or with the fix.
3. Make the smallest reviewable semantic change.
4. Update protocol/schema/docs/runbook and machine gap status together.
5. Run local static/unit/property/PostgreSQL checks.
6. Push exact source and wait for exact-head checks.
7. Inspect check runs, artifacts, review threads and base drift.
8. Record result as closed, failed, blocked or resume-required without partial
   credit.

## 4. Required commands

```bash
bash scripts/project-preflight.sh
./scripts/check_trnm_game_product.sh
./scripts/check_trnm_authority_boundary.sh
./scripts/check_trnm_runtime_configuration.sh
./scripts/check_trnm_settlement_outbox_contract.sh
./scripts/check_trnm_settlement_transaction_boundary.sh

cd trillionnium
cargo metadata --locked --no-deps --format-version 1 >/dev/null
cargo fmt --all -- --check
cargo test --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

For settlement source closure, require PostgreSQL-backed capture, lease,
serialization, operator and fault tests with the environment variable that
turns skipped database tests into failures.

## 5. Review and CI

- CI has `contents: read` unless an independently reviewed release publishing
  job needs narrowly scoped artifact permissions.
- CI never applies format/clippy fixes, commits, pushes or tags the candidate.
- A formatting/lint failure returns a patch artifact or diagnostic only.
- Request independent code-owner/security review.
- Do not merge your own PR.
- Revalidate the exact head/tree immediately before any release or merge action.

## 6. External blockers

Use these outcomes rather than fabricating closure:

- `BLOCKED_UPSTREAM`: Nakama, Integration, CEX or Chain owner work is absent.
- `SERVER_CONFIGURATION_REQUIRED`: branch rules, secrets or deployment controls
  require server administration.
- `EXTERNAL_EVIDENCE_REQUIRED`: human, 24-hour, public-network, cross-host,
  custody, legal or commercial proof is required.
- `BASE_DRIFT`: the reviewed base/head/tree changed.
- `STOP_CONDITION`: proceeding would violate authority, settlement, evidence or
  release boundaries.
- `RESUME_REQUIRED`: source is ready but exact CI/review/evidence is incomplete.

## 7. Closeout record

A closeout comment/record includes:

- gap/package ID;
- base/head/tree;
- changed files and invariants;
- commands and exact check runs;
- artifacts and hashes;
- reviewer decision;
- remaining limitations/dependencies;
- final outcome.
