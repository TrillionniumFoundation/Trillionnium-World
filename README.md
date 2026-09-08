# Trillionnium World

Trillionnium World is the **game-product** repository for the Trillionnium RPG/RTS experience. It owns deterministic game-domain behavior, authored content, the native client, campaign/save/progression logic, player-facing economic intents, World transition/outcome hashes and unsigned replay material.

It does **not** own canonical online admission or event ordering, Chain finality, wallet custody or cross-repository release truth. Those responsibilities belong to Nakama, Trillionnium Chain, CEX and Trillionnium Integration respectively.

## Current posture

| Denominator | Posture |
|---|---|
| Deterministic World rules/runtime | Source candidate; exact-head hosted qualification is required |
| Native single-player client | Technical alpha; Linux engineering path exists, multi-OS and human evidence remain open |
| World-local online server | `world_legacy_local_alpha` compatibility enclave only |
| Trusted CEX settlement | Source candidate; selected CEX revision, custody and deployed fault evidence remain open |
| Nakama canonical online | Blocked on exact contract lock, shadow convergence and cutover evidence |
| Public online / public player market | **NO-GO / disabled** |

No source change, local test, generated status file or single-host fixture may promote public-online, custody, market, commercial, human or cross-host claims.

## Authority ownership

| Artifact or decision | Accountable system |
|---|---|
| Content, deterministic rules, simulation, World outcome hash, unsigned replay | Trillionnium World |
| Participant admission, canonical total order, idempotency, restart recovery, archive root and `MatchCompletedV1` | Trillionnium Nakama |
| Ingress, consensus, inclusion and finality | Trillionnium Chain |
| Wallet/ledger settlement and custody | CEX |
| Exact multi-repository component lock and release evidence | Trillionnium Integration |

The binding details are in [`PROJECT_BOUNDARY.md`](PROJECT_BOUNDARY.md) and [`docs/adr/0003-world-authority-service-boundary.md`](docs/adr/0003-world-authority-service-boundary.md).

## Active workspace

```text
trillionnium/
  Cargo.toml
  crates/
    trnm-economy-protocol/
    trnm-rpg-core/
    trnm-campaign-core/
    trnm-rts-protocol/
    trnm-rts-sim/
    trnm-online-protocol/
    trnm-game-server/
    trnm-first-contact/
```

`trillionnium/crates/platform` is excluded legacy material. New code must not enter it.

## Start here

1. [`PROJECT_BOUNDARY.md`](PROJECT_BOUNDARY.md) — binding repository and authority boundary.
2. [`CURRENT_PLAN.md`](CURRENT_PLAN.md) — active execution pointer and exact evidence interpretation.
3. [`docs/README.md`](docs/README.md) — current documentation map.
4. [`docs/modules/README.md`](docs/modules/README.md) — eight module summaries and detailed technical designs.
5. [`GAME_STATUS.md`](GAME_STATUS.md) — gameplay/runtime evidence and honest product limitations.
6. [`RELEASE_READINESS.md`](RELEASE_READINESS.md) — current release decision.
7. [`OPERATIONS.md`](OPERATIONS.md) — supported development, validation and compatibility-enclave operations.

## Development preflight

All development occurs on a lane-compliant branch, never directly on `main`.

```bash
bash scripts/project-preflight.sh
```

Primary source gates:

```bash
./scripts/check_trnm_game_product.sh
./scripts/check_trnm_authority_boundary.sh
./scripts/check_trnm_runtime_configuration.sh
./scripts/check_trnm_settlement_outbox_contract.sh
./scripts/check_trnm_settlement_transaction_boundary.sh
python3 scripts/check-trnm-world-module-documentation.py
python3 scripts/check-trnm-world-transition-conformance.py
```

## Local Rust validation

The repository pins its toolchain in `rust-toolchain.toml`.

```bash
cd trillionnium
cargo fmt --all -- --check
cargo test --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

PostgreSQL-backed tests require an isolated PostgreSQL 16.4 database and the explicit fail-closed environment flag documented by the database and settlement runbooks.

## Release truth

Capabilities move through distinct states:

```text
implemented -> independently validated -> release eligible
```

Release eligibility requires exact source/tree/binary/toolchain/environment binding, immutable artifacts, rollback or disablement rehearsal, appropriate independent review and the evidence class required by the claim. Automation cannot create human, public-network, custody, legal or commercial evidence.

## Governance

- Pull-request review is mandatory for release credit.
- Validation workflows use read-only repository permissions and may upload evidence but may not modify candidate source.
- Branch protection and required checks are server-side controls; files in `.github/` cannot self-assert that they are active.
- Missing, skipped, cancelled, stale or unbound checks are blockers.
- Do not merge a self-authored candidate or use administrator bypass to manufacture closure.

## Licensing

The active Rust workspace uses `LicenseRef-Trillionnium-Internal`. Source, assets, audio, vendored code and third-party notices require an explicit licensing decision before public or commercial distribution.
