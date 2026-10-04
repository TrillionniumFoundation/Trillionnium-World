# TRNM legacy platform workspace

Status: **excluded legacy source**  
Release denominator: **none**  
Active development: **forbidden in this repository**

This nested workspace is retained only for provenance, historical comparison and explicitly requested compatibility investigation. It is not part of the active eight-crate Trillionnium World game-product workspace and must not be cited as current World, Chain, validator, PoUW, bridge, oracle or mainnet implementation evidence.

## Retained members

The workspace manifest currently lists exactly:

| Crate | Historical role |
|---|---|
| `trnm-node` | node/runtime assembly |
| `trnm-types` | shared platform types |
| `trnm-state` | state storage/model |
| `trnm-pouw` | proof-of-useful-work experiments |
| `trnm-executor` | execution experiments |
| `trnm-mempool` | transaction admission/queue experiments |
| `trnm-rpc` | RPC experiments |
| `trnm-bench` | benchmark driver |
| `trnm-worker-agent` | worker-agent experiments |
| `trnm-cli` | historical CLI |
| `trnm-bridge-poc` | bridge proof of concept |
| `trnm-oracle` | oracle proof of concept |

`docs/component-catalog.json` records the workspace and every member as `excluded-legacy` with `release_denominator=none`.

## Binding boundaries

- Current World development occurs only in `trillionnium/Cargo.toml` and the explicitly isolated `trillionnium/crates/world-authority/Cargo.toml` candidate.
- Canonical Chain development belongs in `TrillionniumFoundation/Trillionnium-Chain`, not here.
- New source, dependencies, migrations, workflow credit or release claims must not be added to this workspace.
- Active crates must not depend on this workspace by path, feature, generated source, copied artifact or runtime fallback.
- A passing nested build proves only that retained historical source still compiles under that exact environment; it grants no current product or production credit.
- Historical documents under `trillionnium/docs` do not override `PROJECT_BOUNDARY.md`, `CURRENT_PLAN.md`, `docs/catalog.json` or `RELEASE_READINESS.md`.

## Explicit historical validation

When a provenance investigation specifically requires it, the workspace may be built independently:

```bash
cargo fmt --manifest-path trillionnium/crates/platform/Cargo.toml --all -- --check
cargo test --manifest-path trillionnium/crates/platform/Cargo.toml --workspace --all-targets --locked
cargo clippy --manifest-path trillionnium/crates/platform/Cargo.toml --workspace --all-targets --locked -- -D warnings
```

Those commands are intentionally outside the active game-product gate. Their results must be labelled `historical_platform_only=true` and bound to exact source/toolchain/dependencies.

## Change and retirement policy

The preferred changes are documentation correction, archival relocation or deletion after an independently reviewed provenance inventory. Any semantic edit requires a new ADR explaining why the work cannot occur in the accountable active repository, plus explicit approval from World and Chain owners. Compatibility readers or fixtures must be extracted into a narrowly named package rather than reviving the whole workspace.

Before physical removal, retain:

- commit/tree provenance;
- license and third-party notices;
- any still-referenced protocol vectors or migration fixtures;
- replacement repository/path mapping;
- proof that active manifests, scripts and workflows no longer reference these crates.

```text
active_world_component=false
active_chain_component=false
release_denominator=none
production_authorization=not_granted
```
