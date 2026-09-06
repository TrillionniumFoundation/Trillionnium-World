---
status: current
owner: trillionnium-world
as_of: 2026-08-30
applies_to_candidate: fix/world-plan-v4-convergence-2026-08-30
supersedes_no_plan: true
---

# Trillionnium World Plan V4 — execution update

This update records facts observed after the V4 convergence addendum. It does not widen World authority or grant release credit.

## Exact candidate

- canonical Draft PR: `#39`;
- branch: `fix/world-plan-v4-convergence-2026-08-30`;
- base observed: `main@efcf0420f6edabc32b7f85332467f25e291cdc63`;
- candidate head after truth-gate repair: `886c9ae9bc05a632abd36d43f85bf8a059f6fb1b`;
- PRs `#36`, `#37`, and `#38` are superseded;
- `WORLD-P0-010` is closed only for the single-current-candidate claim.

## Corrected source truth

The CEX transport and settlement worker are directly compiled reviewed source. The only remaining semantic source transform is the game-server library:

```text
src/lib.rs.in -> build.rs semantic rewrite -> OUT_DIR/trnm_game_server_lib_generated.rs
```

`WORLD-P0-009` remains `source_open` until the generated body is committed as ordinary `src/lib.rs`, `src/lib.rs.in` and the semantic build script are removed, direct-source tests replace generator-shape tests, and exact-head compilation succeeds.

`WORLD-P1-001` remains `source_open`. Documentation boundaries alone do not prove correctness-oriented module ownership; the catch-all game-server library must be decomposed without changing authority, lock order, terminal-publication, or settlement invariants.

## Gate repair completed in this tranche

`scripts/check-trnm-world-v4-convergence.py` now validates the observed machine counts instead of hard-coding the pre-convergence values. It:

- requires `WORLD-P0-010` closure evidence bound to PR `#39`;
- keeps `WORLD-P0-009` open while `build.rs` or `lib.rs.in` exists;
- supports a later directly compiled shape only when all mandatory markers exist and legacy settlement paths are absent;
- rejects source verification without non-empty exact-head runs and required checks;
- keeps upstream, human, public-network, custody, market, and commercial claims fail closed.

## Repository-control observation

After the candidate write, GitHub still returned no workflow run for the exact World head. A separate read-only source-export attempt was accepted by CEX Actions but failed at runner startup before any step on both Ubuntu and macOS; its temporary workflow was removed. No source or release credit is derived from those startup failures.

The remaining repository-control gates require server-side actions not exposed by the current connector:

- enable repository/organization Actions execution;
- install a `main` ruleset with exact required checks;
- enforce independent code-owner approval after the last push;
- execute a negative merge rehearsal.

## Ordered continuation

1. Materialize the exact game-server generated body as ordinary reviewed source.
2. Replace generator-shape tests and remove the semantic build transform.
3. Extract correctness-critical ownership modules with invariant tests.
4. Run non-empty exact-head Rust, PostgreSQL, canonical JSON, package, and supply-chain checks.
5. Apply and independently query server-side governance.
6. Hand upstream work to Nakama, Integration, and CEX owners using exact component locks.
7. Execute immutable deployed fault, PITR, endurance, public-edge, custody, multi-platform, human, and commercial evidence programs.

## Release posture

```text
technical source candidate: in progress
exact-head remote verification: repository control blocked
canonical Nakama online authority: blocked upstream
trusted deployed settlement: environment evidence required
public online: NO-GO
public player market: disabled
commercial release: NO-GO
```
