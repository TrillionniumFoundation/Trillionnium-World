# Contributing

## Before changing code

1. Read [`PROJECT_BOUNDARY.md`](PROJECT_BOUNDARY.md).
2. Locate the module in [`docs/modules.json`](docs/modules.json).
3. Read its code-adjacent README and active release/gap documents.
4. Keep game, platform, contracts, and Web4 evidence denominators separate.

## Pull-request discipline

Explain the problem and scope; authority, persistence, protocol, and security impact; compatibility and migration behavior; exact verification commands; generated artifacts and evidence scope; rollback plan; and documentation updated.

Do not claim production readiness, public capacity, human usability, multi-region behavior, or mainnet closure from local tests.

## Required verification

```bash
# Game
cargo fmt --manifest-path trillionnium/Cargo.toml --all -- --check
cargo test --manifest-path trillionnium/Cargo.toml --workspace --all-targets --locked
cargo clippy --manifest-path trillionnium/Cargo.toml --workspace --all-targets --locked -- -D warnings

# Platform
cargo test --manifest-path trillionnium/crates/platform/Cargo.toml --workspace --locked

# Contracts
cargo test --manifest-path contracts/Cargo.toml --workspace

# Documentation
python3 scripts/check_docs_quality.py
```

For Web4, run `npm ci && npm run ci:check` from `web4-frontend/`.

## Protocol and persistence changes

A public protocol change requires a version decision, compatibility matrix, invalid/crossed-version tests, rollout and rollback order, deprecation criteria, and generated schema/reference updates.

A persistence change additionally requires interrupted-migration, duplicate/retry, crash/restart, rollback, and ambiguous-commit analysis.

## Documentation rules

Update the module README whenever public behavior, configuration, authority, failure, or migration changes. Use repository-relative paths. Never use a personal workstation path as a canonical command. Move superseded material to `docs/archive/` and mark it historical. Evidence reports must be immutable and commit-bound.
