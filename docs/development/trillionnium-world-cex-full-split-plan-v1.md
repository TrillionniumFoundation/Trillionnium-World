# Trillionnium World CEX full split plan v1

## Intent

Move Trillionnium World from the CEX incubator into the standalone Trillionnium development environment without losing the CEX evidence discipline.

This is a full product/runtime split, not just a documentation mirror. The new landing zone is the `trillionnium/crates/trnm-world-*` crate family plus the acceptance evidence tree.

## Landing crates

| Target crate | Responsibility | CEX source area |
| --- | --- | --- |
| `trnm-world-domain` | World state, nodes, routes, positions, NPCs, tasks, receipts, source-of-truth constants | `consumer-entry-api/src/lib.rs`, `world_indexes.rs`, `world_tactics.rs` domain structs |
| `trnm-world-command` | Intent-only commands and Rust-owned decisions | `world_routes.rs`, `world_movement.rs`, `world_commerce_routes.rs`, tactics command handlers |
| `trnm-world-projection` | Home/map/route/task/read models | `world_map_projection.rs`, `world_route_projection.rs`, `client_surfaces.rs` |
| `trnm-world-map-provider` | Fixture OSM, future map_pack manifest, attribution/compliance posture | `world_map_projection.rs`, `world_map_optimization.rs`, OSM gates |
| `trnm-world-ui-fragments` | Rust-owned `/world` fragments | `world_web_shell.rs`, `real_world_map_shell.rs` Rust fragment helpers |
| `trnm-world-api` | Stable API request/response contracts | `/v1/world/*`, `/world/web/*`, client app/feed contracts |
| `trnm-world-server` | Standalone runtime adapter/server and dev HTTP surface | future replacement for CEX-hosted `/world` runtime |
| `trnm-world-bevy` | Native Bevy client shell; consumes Rust projections and submits intent-only commands | S5 Native/Bevy Mobile Gate |
| `trnm-world-dev-env` | Environment/evidence report and future gate status | docs/runtime gates/check scripts |

## Dependency order

```text
trnm-world-domain
  -> trnm-world-command
  -> trnm-world-projection
  -> trnm-world-map-provider
  -> trnm-world-ui-fragments
  -> trnm-world-api
  -> trnm-world-server
  -> trnm-world-bevy
  -> trnm-world-dev-env
```

No crate in this family may depend on CEX service internals. CEX can depend on these crates later as an adapter, but not the reverse.

## What can be moved now

- Contract constants and ownership markers.
- Pure domain structs with serde-compatible fields.
- Pure command decision logic that does not call CEX `AppState`.
- Pure projection/read-model builders.
- Rust-owned HTML fragment builders.
- Fixture map provider posture and map-pack gate metadata.
- Unit tests that assert Rust source-of-truth and no client authority mutation.

## Direct CEX extraction map

This map is based on direct inspection of `CEX/services/consumer-entry-api/src`, not on the rejected migration-map subagent output.

| CEX source | Pure Trillionnium target | Extraction status |
| --- | --- | --- |
| `lib.rs` world structs (`WorldState`, `WorldMapNode`, `WorldPlayerPosition`, commerce/tactics structs) | `trnm-world-domain` | Split: standalone world state, nodes, routes, positions, NPCs, tasks, receipts, CEX snapshot compatibility, item/equipment runtime, authored quest chains, tactics sessions, simulation ticks, and reward settlements are present. |
| `league_repository.rs::default_world_map_nodes` | `trnm-world-domain` / `trnm-world-map-provider` | Split: `WorldState::cex_default_map_fixture()` preserves the 24-node CEX incubator topology; `trnm-world-map-provider` packages it via `cex_default_world_from_map_provider()`; `trnm-world-server cex-map-home-json` and `full-split-json` smoke the 24-node surface. |
| `world_movement.rs` | `trnm-world-command` + `trnm-world-ui-fragments` | Split: transition contract, alias normalization, Rust-owned transition decision, locked/interaction-required/non-adjacent/blocked classification, and keypad transition attributes have been extracted. |
| `world_indexes.rs` | `trnm-world-domain` | Split: recent-tail/sorted/indexed helpers and standalone `WorldIndexes` node/position/task/receipt maps have moved. Commerce/work-order progression now enters the split through typed `WorldRouteRecords` rather than CEX `LeagueState`. |
| `world_map_projection.rs` | `trnm-world-projection` / `trnm-world-map-provider` | Split: entity-group cursor/hash/versioning, delta payload helpers, weak ETag generation, transport-delta contract shape, OSM fail-closed provider posture, and runtime budget/density/RUM contracts live in standalone crates. Full live viewport construction is now a `WorldRepository` / session adapter concern. |
| `world_route_projection.rs` | `trnm-world-projection` / `trnm-world-ui-fragments` | Split: UI IDs, command targets, playability anchors, focus lanes, route UI contract JSON, recommendation ranker, preview parsing/JSON, task graph/story DTOs, route artifacts builder, feed item DTO conversion, route HTML fragments, typed route records, tactics route-task binding, and fixture work-order/ledger/session adapters have moved. |
| `world_tactics.rs` | `trnm-world-domain` / `trnm-world-command` / `trnm-world-projection` | Split: contract constants, skills, mentor training, task archetypes, objective roles, sect/NPC fixtures and capability mapping, NPC relationship deltas, attributes, item/equipment catalog and runtime, resource pressure/survival, region/story unlock, authored quest chains, combat numerics, board/session projection, command descriptor JSON, and adapter-free command outcomes have moved. |
| `world_web_shell.rs` | `trnm-world-ui-fragments` | Started: Rust-owned keypad movement button fragment has been extracted with the CEX transition data attributes preserved. Large tactics/status/local-action fragments remain. |
| `real_world_map_shell.rs` | `trnm-world-ui-fragments` | Pending: runtime JS is browser-shell-specific; only contract JSON/bootstrap fragments should move before a standalone web shell exists. |
| `world_routes.rs`, `world_commerce_routes.rs` | `trnm-world-api` / `trnm-world-server` + adapter traits | Split boundary ready: pure `world_action_kind`, quality-signal classification, world commands, tactics commands, and the `WorldLedgerAdapter` / `WorldRepository` / `WorldSessionGuard` / identity / evidence / metrics traits are standalone. HTTP/session/ledger/SQL production implementations stay outside these crates. |
| `client_surfaces.rs`, `client_app_shell.rs` | `trnm-world-projection` / `trnm-world-ui-fragments` | Split at the world-contract layer: `world_full_split_projection_json`, route artifacts, tactics board, runtime adapter receipts, and Rust fragments are available for Trillionnium-side client/feed shells without CEX service internals. |

## Extracted standalone slices

- `trnm-world-domain::cex_compat` reads CEX-incubator snapshots without depending on CEX service crates.
- `trnm-world-command` owns the first extracted transition/movement semantics from CEX `world_movement.rs`, including the `trillionnium_world_transition_semantics_v1` contract, source-of-truth `rust_world_map_transition_rules`, keypad aliases, wait/open/blocked/locked/interaction/non-adjacent decisions, and Rust-owned command mutation.
- `trnm-world-ui-fragments` owns the first extracted CEX keypad button fragment: buttons remain rendered by Rust, expose transition status/kind/result/blocked reason, and keep the browser role as input-only.
- `trnm-world-command` also owns the first adapter-free CEX `world_routes.rs` action classification slice: `trillionnium_world_action_engine_v1`, action kind/base impact, and quality-signal scoring are available without CEX runtime state.
- `trnm-world-domain` owns the first adapter-free CEX `world_indexes.rs` slice: recent-tail indices, generic sorted/indexed helpers, and standalone node/location/position/task/receipt indexes.
- `trnm-world-domain` now carries the CEX default map as `WorldState::cex_default_map_fixture()` with 24 incubator nodes, preserving routes such as `mirror-city-square -> league-coliseum` north and `mirror-city-square -> starter-studio` east while keeping the smaller smoke fixture stable.
- `trnm-world-projection` now owns the first CEX `world_route_projection.rs` UI-routing slice: `trillionnium_world_route_command_target_v1`, work/commerce/contract panel constants, playability body/command anchors, route command target mapping, and focus-lane node/tag preferences. `trnm-world-api` and `trnm-world-server` expose this via a `route-target` smoke path.
- `trnm-world-projection` also owns the next adapter-free route projection slices: route UI contract JSON, commercial-quality route recommendation scoring, preview item parsing, preview JSON wrapper, task graph grouping/sorting, task descriptor outcome/feedback text, next-opportunity derivation, suggested action targeting, story DTOs, route artifacts builder, and feed item DTO conversion.
- `trnm-world-ui-fragments` now owns route preview cards, route task graph app/world-flow cards, route action-button handoff attributes, and CJK/i18n visible-text fallback helpers.
- `trnm-world-domain` now owns the first CEX `world_tactics.rs` content/runtime slices: `trillionnium_skill_v1`, `trillionnium_training_command_v1`, `trillionnium_task_archetype_v1`, objective role helpers, sect fixtures, NPC fixtures and capability-to-task mapping, NPC relationship deltas, attributes/derived stats, resource pressure/survival mutation state, region/story unlock state, and combat numerics mutation state.
- `trnm-world-map-provider` now packages the CEX default map fixture, extracted OpenStreetMap provider-mode fail-closed posture, map runtime budget/density/RUM contracts, and entity-group versioned map delta helpers; `trnm-world-server cex-map-home-json`, `route-artifacts`, and `map-runtime-budget` prove these slices through standalone runtime smokes.
- `trnm-world-domain` also owns native item/equipment catalog/runtime state, authored quest chains, story arc catalog, tactics session/tick/reward settlement records, and character projection runtime fields.
- `trnm-world-command` now owns adapter-free tactics command outcomes for `train_skill`, `equip_item`, `attack`, `talk_npc`, `offer_task`, and `complete_task`; these mutate `WorldTrillionniumCharacter` state without CEX `AppState`.
- `trnm-world-api` defines the runtime cutover trait seam: `WorldIdentityAdapter`, `WorldSessionGuard`, `WorldLedgerAdapter`, `WorldRepository`, `WorldEvidenceSink`, and `WorldMetricsSink`, plus `trillionnium_world_runtime_adapter_v1` readiness evidence.
- `trnm-world-server` provides fixture implementations for the runtime traits and exposes `adapter-readiness`, `tactics-command`, and `full-split-json` smokes.
- `trnm-world-server` now also provides a standalone development HTTP runtime contract, `trillionnium_world_dev_runtime_v1`, through `cargo world-server serve --bind 127.0.0.1:8787`. It exposes `/health`, `/world/home`, `/world/state`, `/world/command`, `/world/tactics-command`, `/world/full-split`, adapter readiness, map budget, and route-artifact endpoints while keeping world mutation inside Rust `WorldCommand`.
- `trnm-world-server` has a file-backed development repository contract, `trillionnium_world_dev_file_repository_v1`, through `--state-file` and `dev-runtime-repository-smoke`, so restart/reload state persistence is now gated before broader Web/Native/Matrix parity.
- `trnm-world-bevy` provides the first S5 game-engine client path: a Bevy app/plugin/resource/component shell for standalone world snapshots, native intent submission back into the Rust command/API layer, a host vertical playable slice, a host first-playable loop covering mentor talk, training, movement, combat, task completion, reward/equipment state, save/restore, and a three-branch title-route keyboard replay gate that reproduces force/agility/craft key-event sequences on a fresh runtime. It now also gates player-facing action coach guidance, player HUD/debug-layer separation, and an 11-frame live-window screenshot sequence for host-side playability review. This keeps Bevy as an intent-only client path rather than a second authority runtime; Android real-device S5 evidence is still separate.

## Adapter Trait Boundary

The standalone crates now define the adapter traits needed for cutover. Production implementations should live outside `trnm-world-*`, so the standalone world does not import CEX service internals:

- `WorldLedgerAdapter`
- `WorldIdentityAdapter`
- `WorldRepository`
- `WorldSessionGuard`
- `WorldEvidenceSink`
- `WorldMetricsSink`

Fixture implementations are in `trnm-world-server` and are verified by `cargo world-server adapter-readiness` plus `cargo world-server full-split-json`. CEX production adapters now live in the CEX repo as callers of these traits and export JSON evidence from `GET /v1/trillionnium/world/adapters/readiness`; Trillionnium consumes that evidence through `scripts/check_trillionnium_world_cex_adapter_readiness.sh` without importing CEX service internals.

## Environment gates now configured

- Rust stable with rustfmt/clippy.
- wasm target for Web/PWA experiments.
- Android targets for Native/Bevy gate.
- JDK/Android SDK/adb/build-tools/NDK installer path for native Android work.
- Linux native graphics/audio/input development libraries for Bevy-style native builds.
- Native Bevy client crate and host + aarch64 Android cargo checks.
- S5 Native/Bevy evidence script builds the aarch64 Android `cdylib`, checks `ANativeActivity_onCreate`/`android_main`, signs a debug APK when an Android platform jar is present, and records ADB device evidence under `acceptance/S5_native_bevy_device/latest/`; public-launch S5 credit uses `ANDROID_SERIAL=<device-serial> scripts/check_trillionnium_world_s5_device_evidence.sh --require-device` followed by `scripts/check_trillionnium_world_s5_real_device_evidence.sh --require-ready`.
- Godot tooling for comparison/reference only.
- Web4 npm dependency install and checks.

## Smallest validation gates

From repo root:

```bash
bash scripts/check_trillionnium_world_dev_env.sh
bash scripts/check_trillionnium_world_browser_parity.sh
bash scripts/check_trillionnium_world_repository_adapter_boundary.sh
bash scripts/check_trillionnium_world_cex_adapter_readiness.sh
bash scripts/check_trillionnium_world_map_pack_gate.sh
bash scripts/check_trillionnium_world_public_deploy_readiness.sh
bash scripts/check_trillionnium_world_release_rollback_backup_drill.sh
bash scripts/check_trillionnium_world_release_latency_drill.sh
bash scripts/check_trillionnium_world_cohort_commercial_schema.sh
bash scripts/check_trillionnium_world_cohort_commercial_evidence_collection.sh
bash scripts/check_trillionnium_world_production_map_pack_route.sh
bash scripts/check_trillionnium_world_production_map_pack_public_evidence_collection.sh
bash scripts/check_trillionnium_world_production_map_pack_public_evidence.sh
bash scripts/check_trillionnium_world_external_ops_evidence_collection.sh
bash scripts/check_trillionnium_world_s5_device_evidence.sh
ANDROID_SERIAL=<device-serial> bash scripts/check_trillionnium_world_s5_device_evidence.sh --require-device
bash scripts/check_trillionnium_world_s5_real_device_evidence.sh --require-ready
bash scripts/check_trillionnium_world_bevy_action_coach.sh
bash scripts/check_trillionnium_world_bevy_player_hud_debug_layer.sh
bash scripts/check_trillionnium_world_bevy_live_window_screenshot_sequence.sh
bash scripts/check_trillionnium_world_public_launch_readiness.sh
bash scripts/check_trillionnium_world_release_signoff_summary.sh
bash scripts/check_trillionnium_world_release_review_quickcheck.sh
bash scripts/check_trillionnium_world_release_review_status.sh
bash scripts/check_trillionnium_world_release_review_convergence.sh
bash scripts/check_trillionnium_world_release_review_packet.sh
bash scripts/check_trillionnium_world_release_review_packet_integrity.sh
bash scripts/check_trillionnium_world_release_review_ci_gate.sh
```

From `trillionnium/`:

```bash
cargo fmt --all -- --check
cargo world-check
cargo world-test
cargo world-env
cargo world-server home-json
cargo world-server home-fragment
cargo world-server cex-map-home-json
cargo world-server route-artifacts
cargo world-server map-runtime-budget
cargo world-server tactics-command train_skill
cargo world-server tactics-command equip_item
cargo world-server adapter-readiness
cargo world-server dev-runtime-smoke
cargo world-server dev-runtime-repository-smoke ../acceptance/S0_world_dev_environment/latest/world-dev-runtime-state.json
cargo world-server full-split-json
cargo world-bevy
cargo world-server move-east
```

From `web4-frontend/`:

```bash
npm ci
npm run typecheck
npm run test:unit
```

## Cutover definition

The pure standalone split is complete when:

1. `trnm-world-server` owns the `/world` equivalent home/command/projection API.
2. Route/tactics/world commands have standalone adapter-free outcomes.
3. Ledger/identity/session/repository/evidence/metrics dependencies are trait-backed with fixture adapters and production-ready trait contracts; repository cutover must also pass `scripts/check_trillionnium_world_repository_adapter_boundary.sh`.
4. Existing CEX Web/Browser/first-human evidence has equivalent Trillionnium-side standalone gates, starting with `scripts/check_trillionnium_world_browser_parity.sh`.
5. S5/S6 claims stay honest: native/Bevy/mobile/commercial score movement requires device matrix, cohort, launch drill, local release latency drill, or multi-node/live evidence.

Current status: items 1-3 are satisfied by `full-split-json`, `tactics-command`, `adapter-readiness`, and `dev-runtime-smoke` smokes. Item 4 is covered at the standalone crate/dev-env level; browser/mobile/commercial parity remains a later product validation stage rather than a pure code extraction blocker.
