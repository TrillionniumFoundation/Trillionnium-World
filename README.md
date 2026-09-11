# Trillionnium World

Trillionnium World is the game-product repository for the native RPG → RTS → RPG loop, its deterministic simulation, online authority, settlement boundary, authored content, packaging, and release evidence.

> **Current posture:** technical alpha / player-facing pre-alpha. The repository is **not release-ready** for a public commercial launch or public mainnet claim. Current release decisions must start from [`RELEASE_READINESS.md`](RELEASE_READINESS.md), and native-game status from [`GAME_STATUS.md`](GAME_STATUS.md).

## Repository boundaries

| Surface | Path | Purpose | Release interpretation |
| --- | --- | --- | --- |
| Native game product | [`trillionnium/`](trillionnium/) | Eight-crate game workspace: RPG, campaign, RTS, client, online protocol, and game server | Primary product lane |
| Platform workspace | [`trillionnium/crates/platform/`](trillionnium/crates/platform/) | Independent chain/node/PoCO platform workspace retained during repository separation | Must be built explicitly; never use as gameplay evidence |
| External contracts | [`contracts/`](contracts/) | Rust MVP contract semantics and normalized audit events | Not a production Host ABI/WASM closure |
| Web4 frontend | [`web4-frontend/`](web4-frontend/) | Read-only query client with explicit mock fallback | Subproject gates do not imply repository release readiness |
| Authored assets | [`assets/`](assets/) | Original maps, atlases, audio, and content manifests | Data/content input, not authority |
| Deployment and packaging | [`deploy/`](deploy/), [`packaging/`](packaging/) | Service definitions and distribution metadata | Evidence must remain platform- and commit-bound |

The normative ownership rule is in [`PROJECT_BOUNDARY.md`](PROJECT_BOUNDARY.md). New game-product code must not create new dependencies on the excluded platform workspace. Cross-surface interaction should use versioned protocols or generated contracts.

## Quick start

Run commands from the repository root unless stated otherwise.

### Native game workspace

```bash
cargo test --manifest-path trillionnium/Cargo.toml --workspace --all-targets --locked
cargo clippy --manifest-path trillionnium/Cargo.toml --workspace --all-targets --locked -- -D warnings
cargo run --manifest-path trillionnium/Cargo.toml -p trnm-first-contact
```

### Platform workspace

```bash
cargo test --manifest-path trillionnium/crates/platform/Cargo.toml --workspace --locked
```

### External contracts

```bash
cargo test --manifest-path contracts/Cargo.toml --workspace
```

### Web4 frontend

```bash
cd web4-frontend
npm ci
npm run ci:check
```

## Documentation entry points

- Repository documentation index: [`docs/README.md`](docs/README.md)
- Documentation policy and truth-source hierarchy: [`docs/DOCUMENTATION_POLICY.md`](docs/DOCUMENTATION_POLICY.md)
- Machine-readable module inventory: [`docs/modules.json`](docs/modules.json)
- Architecture and boundaries: [`docs/architecture/README.md`](docs/architecture/README.md)
- Current gap-closure board: [`docs/release/GAP_CLOSURE_BOARD_2026-09-11.md`](docs/release/GAP_CLOSURE_BOARD_2026-09-11.md)
- Security reporting: [`SECURITY.md`](SECURITY.md)
- Contribution and verification rules: [`CONTRIBUTING.md`](CONTRIBUTING.md)

Validate the canonical documentation surface locally:

```bash
python3 scripts/check_docs_quality.py
```

## Truth-source hierarchy

1. **Repository release readiness** — [`RELEASE_READINESS.md`](RELEASE_READINESS.md), cited together with the exact `origin/main` commit.
2. **Native game state** — [`GAME_STATUS.md`](GAME_STATUS.md).
3. **Ownership and workspace boundaries** — [`PROJECT_BOUNDARY.md`](PROJECT_BOUNDARY.md) and [`docs/modules.json`](docs/modules.json).
4. **Open gaps and closure evidence** — [`docs/release/GAP_CLOSURE_BOARD_2026-09-11.md`](docs/release/GAP_CLOSURE_BOARD_2026-09-11.md).
5. **Historical progress** — `docs/archive/`; historical material is not current readiness evidence.

A green unit test, local smoke run, documentation checklist, or one subproject release gate must never be generalized into a public-launch claim without the exact evidence required by the relevant release gate.

## License and provenance

The Rust workspaces currently use the internal license identifier declared in their workspace manifests. Authored content and third-party materials must retain source, license, and provenance records. Do not copy proprietary source code, text, maps, NPC/task tables, art, audio, or datasets into this repository.
