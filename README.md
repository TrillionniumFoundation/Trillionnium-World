# Trillionnium World

Trillionnium World is the **game-product** repository for the Trillionnium RPG/RTS experience. It owns deterministic game-domain behavior, authored content, the native client, campaign/save/progression logic, player-facing economy intents, and unsigned replay/outcome material.

It does **not** own canonical online admission or event ordering, Chain finality, wallet custody, or cross-repository release truth. Those responsibilities belong to Nakama, Trillionnium Chain, CEX, and Trillionnium Integration respectively.

## Current posture

| Denominator | Current posture |
| --- | --- |
| Deterministic World rules/runtime | Technical alpha; source implementation exists, exact-head and cross-language evidence remain gated |
| Native single-player client | Pre-alpha/technical alpha; Linux engineering paths exist, multi-OS signing and independent human validation remain gated |
| World-local online server | Compatibility laboratory only (`world_legacy_local_alpha`); it is not the target canonical public authority |
| Trusted CEX settlement | Source candidate only; production custody, deployed ambiguity recovery, PITR and reviewer evidence remain gated |
| Nakama closed online | Blocked until the World transition contract, Nakama shadow runner and Integration component lock converge |
| Public online / public player market | **NO-GO / disabled** |

No source change, local test, generated status file, or single-host fixture may promote public-online, custody, market, commercial, human, or cross-host claims.

## Accountable systems

| Artifact or decision | Accountable system |
| --- | --- |
| Content, deterministic rules, simulation, World outcome hash, unsigned replay material | Trillionnium World |
| Online participant admission, canonical total order, command idempotency, restart recovery, archive roots, `MatchCompletedV1` signing | Trillionnium Nakama |
| Ingress, consensus, inclusion and finality | Trillionnium Chain |
| Wallet/ledger settlement and custody | CEX |
| Exact cross-repository component locks, compatibility matrices and release evidence | Trillionnium Integration |

The existing `trillionnium/crates/trnm-game-server` is a bounded compatibility enclave. New code must not expand it into a second canonical online authority.

## Active workspace

```text
trillionnium/
  Cargo.toml                       # eight active game-product crates
  crates/
    trnm-first-contact/            # native Bevy client
    trnm-campaign-core/            # campaign/save/progression aggregate
    trnm-rpg-core/                 # RPG vocabulary and domain types
    trnm-rts-protocol/             # deterministic RTS command contract
    trnm-rts-sim/                  # Bevy-free deterministic simulation
    trnm-online-protocol/          # compatibility online wire contract
    trnm-game-server/              # World-local compatibility server
    trnm-economy-protocol/         # game-owned intent/receipt vocabulary
```

`trillionnium/crates/platform` is excluded legacy material and is not an active development workspace.

## Start here

1. `PROJECT_BOUNDARY.md` — binding repository and authority boundary.
2. `CURRENT_PLAN.md` — canonical pointer to the active execution plan.
3. `docs/status/CURRENT.md` — generated human-readable gate posture.
4. `docs/README.md` — current documentation map.
5. `GAME_STATUS.md` — native gameplay/runtime evidence and honest open boundaries.

## Development preflight

All development must happen on a lane-compliant branch, never directly on `main`.

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
```

The exact active plan defines additional workstream-specific checks. A missing, skipped, cancelled or stale check is a blocker, not a pass.

## Local Rust validation

```bash
cd trillionnium
cargo fmt --all -- --check
cargo test --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

PostgreSQL-backed settlement tests require an isolated PostgreSQL 16 test database and the explicit fail-closed environment flag documented in the settlement runbooks.

## Release truth

A capability moves through three distinct states:

```text
implemented -> independently validated -> release eligible
```

Release eligibility additionally requires exact commit/tree/binary/toolchain/environment binding, immutable artifacts, rollback or disablement rehearsal, reviewer signoff, and the evidence class appropriate to the claim. Automated evidence cannot satisfy human, public-network, cross-host, custody, legal or commercial rows.

## Repository governance

- Pull-request review is mandatory for release credit.
- CI workflows must be read-only with respect to repository source; validation may upload evidence but must not modify or push candidate code.
- Branch protection and required checks are server-side controls. Files in `.github/` cannot self-assert that those controls are active.
- Historical Chain/Web4/World-Bevy documents are provenance only unless a current document explicitly cites a bounded artifact.

## Licensing

The active Rust workspace currently uses `LicenseRef-Trillionnium-Internal`. Source, assets, audio, vendored code and third-party notices must be reviewed under the repository licensing policy before any public or commercial distribution claim.