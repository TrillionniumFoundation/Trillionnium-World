# `trnm-bench`

Status: **migration-only / alpha platform component**  
Owner: Trillionnium Foundation platform maintainers  
Workspace: `trillionnium/crates/platform/Cargo.toml`

## Purpose

Platform benchmark and performance-evidence harness.

It owns repeatable benchmark scenarios, metrics, comparison output, and threshold-oriented local evidence. It does not turn an unbound developer run into a public SLO or establish functional correctness by itself.

## Evidence invariants

- Every release-relevant run records commit/tree, clean state, workload, environment, hardware/network profile, command, thresholds, and raw artifacts.
- Comparisons use equivalent inputs and versions.
- Thresholds are defined before evaluation.
- Warm-up, sample count, variance, and failures are retained rather than averaged away.

## Build and test

```bash
cargo test --manifest-path trillionnium/crates/platform/Cargo.toml -p trnm-bench --locked
cargo run --manifest-path trillionnium/crates/platform/Cargo.toml -p trnm-bench -- --help
```

## Open gaps

Representative multi-host workloads, hardware normalization, retained raw artifacts, saturation/backpressure cases, and public capacity evidence remain open. See [`Gap closure board`](../../../../docs/release/GAP_CLOSURE_BOARD_2026-09-11.md).
