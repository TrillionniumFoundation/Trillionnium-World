# Trillionnium World standalone development environment v1

This repository now has a dedicated Trillionnium World development environment that can receive the CEX incubator split without pretending the native/MMO future gates are already implemented.

## Scope

Configured in-tree:

- Rust workspace crates under `trillionnium/crates/trnm-world-*`:
  - `trnm-world-domain`
  - `trnm-world-command`
  - `trnm-world-projection`
  - `trnm-world-map-provider`
  - `trnm-world-ui-fragments`
  - `trnm-world-api`
  - `trnm-world-server`
  - `trnm-world-bevy`
  - `trnm-world-dev-env`
- Rust toolchain file at repo root with rustfmt, clippy, wasm, and Android targets.
- Cargo aliases in `.cargo/config.toml`: `world-check`, `world-test`, `world-env`, `world-server`, `world-bevy`.
- S5 evidence script: `scripts/check_trillionnium_world_s5_device_evidence.sh`.
- Public-launch readiness script: `scripts/check_trillionnium_world_public_launch_readiness.sh`.
- Release signoff summary script: `scripts/check_trillionnium_world_release_signoff_summary.sh`.
- Release review quickcheck script: `scripts/check_trillionnium_world_release_review_quickcheck.sh`.
- Release review status checklist script: `scripts/check_trillionnium_world_release_review_status.sh`.
- Release review convergence script: `scripts/check_trillionnium_world_release_review_convergence.sh`.
- Release review packet script: `scripts/check_trillionnium_world_release_review_packet.sh`.
- Release review packet integrity script: `scripts/check_trillionnium_world_release_review_packet_integrity.sh`.
- Release review CI aggregate script: `scripts/check_trillionnium_world_release_review_ci_gate.sh`.
- Standalone browser parity script: `scripts/check_trillionnium_world_browser_parity.sh`.
- Repository adapter boundary script: `scripts/check_trillionnium_world_repository_adapter_boundary.sh`.
- Public deploy readiness script: `scripts/check_trillionnium_world_public_deploy_readiness.sh`.
- Release rollback/backup drill script: `scripts/check_trillionnium_world_release_rollback_backup_drill.sh`.
- Release latency drill script: `scripts/check_trillionnium_world_release_latency_drill.sh`.
- Cohort/commercial evidence schema script: `scripts/check_trillionnium_world_cohort_commercial_schema.sh`.
- Cohort/commercial evidence collection script: `scripts/check_trillionnium_world_cohort_commercial_evidence_collection.sh`.
- Production map-pack route script: `scripts/check_trillionnium_world_production_map_pack_route.sh`.
- Production map-pack public evidence collection script: `scripts/check_trillionnium_world_production_map_pack_public_evidence_collection.sh`.
- Production map-pack public evidence script: `scripts/check_trillionnium_world_production_map_pack_public_evidence.sh`.
- Standalone dev runtime mode: `cargo world-server serve --bind 127.0.0.1:8787`, with `/health`, `/world/home`, `/world/state`, `/world/command`, `/world/tactics-command`, `/world/full-split`, adapter readiness, map budget, and route-artifact endpoints.
- File-backed dev repository mode: `cargo world-server serve --state-file acceptance/S0_world_dev_environment/latest/world-dev-runtime-state.json --reset-state`, plus `cargo world-server dev-runtime-repository-smoke`.
- Local environment template: `config/trillionnium-world-dev.env.example`.
- Acceptance roots for environment and native/mobile gates under `acceptance/`.

System-level tools expected by this environment:

- Rust stable + rustfmt + clippy.
- wasm32 target for PWA/Web experiments.
- Android Rust targets, JDK, Android SDK/platform tools, adb, and build tools for the S5 Native/Bevy gate.
- clang/lld/pkg-config/CMake plus Linux graphics/audio/input development libraries for Bevy-style native builds.
- Godot executable for future scene-flow comparison only; Godot is not the current runtime.
- Node/npm for `web4-frontend`.

## Current truth

- CEX remains the proof-bearing incubator until this workspace replaces its `/world` runtime gates.
- The new `trnm-world-*` crates are the extraction target and boundary contracts.
- `trnm-world-server` now has a standalone development HTTP runtime (`trillionnium_world_dev_runtime_v1`). It keeps `WorldState` in the Rust server process, mutates it only through `WorldCommand`, and exposes development endpoints for web/native clients to consume without importing CEX internals.
- `trnm-world-server` can persist that development `WorldState` through `trillionnium_world_dev_file_repository_v1`, so restart/reload tests can prove server state is not just an in-memory smoke.
- Bevy is now represented by a native client gate (`trnm-world-bevy`) for the S5 native/mobile path; it consumes Rust world projections and submits intent only. It is not the authority runtime.
- Host-side Native/Bevy playability gates now include `scripts/check_trillionnium_world_bevy_vertical_slice.sh`, `scripts/check_trillionnium_world_bevy_first_playable.sh`, and `scripts/check_trillionnium_world_bevy_build_branch_title_route_all_branch_keyboard_replay.sh`; these prove a local game-engine first playable loop and reproducible build-title keyboard route protocol before real Android collection.
- S5 Native/Bevy evidence now separates host/Android artifact readiness from real-device proof:
  - `libtrnm_world_bevy.so` must build for `aarch64-linux-android` and export `ANativeActivity_onCreate` plus `android_main`.
  - A signed debug APK is produced when an Android platform jar is available.
  - Real-device status remains blocked until ADB sees an online device and `ANDROID_SERIAL=<device-serial> scripts/check_trillionnium_world_s5_device_evidence.sh --require-device` collects launch, screenshot, gfxinfo/frame, CJK/input, lifecycle, weak-network, APK resource/signature, and crash-free logcat evidence.
- Godot is a scene-flow/reference candidate; not the authority runtime.
- Leaflet/MapLibre live/shadow map posture remains inherited from the CEX evidence until map-pack gates move here.

## Validation

From repo root:

```bash
bash scripts/check_trillionnium_world_dev_env.sh
bash scripts/check_trillionnium_world_browser_parity.sh
bash scripts/check_trillionnium_world_repository_adapter_boundary.sh
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
# Public-launch S5 credit requires a real connected device:
ANDROID_SERIAL=<device-serial> bash scripts/check_trillionnium_world_s5_device_evidence.sh --require-device
bash scripts/check_trillionnium_world_s5_real_device_evidence.sh --require-ready
bash scripts/check_trillionnium_world_external_ops_evidence.sh --require-ready
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
cargo world-env
cargo world-check
cargo world-test
cargo world-bevy
cargo world-server home-json
cargo world-server cex-map-home-json
cargo world-server route-target '/work reject latest 重试拒收退款'
cargo world-server route-artifacts
cargo world-server map-runtime-budget
cargo world-server dev-runtime-smoke
cargo world-server dev-runtime-repository-smoke ../acceptance/S0_world_dev_environment/latest/world-dev-runtime-state.json
cargo world-server serve --bind 127.0.0.1:8787 --state-file ../acceptance/S0_world_dev_environment/latest/world-dev-runtime-state.json
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

## Split rule

Port in this order:

1. Contracts, constants, and tests.
2. Domain state and command decisions.
3. Projection/read models.
4. UI fragments.
5. API/server adapters.
6. Map-pack and native/mobile gates.

Do not move browser or native clients into authority. Clients submit intent; Rust world command/projection remains source of truth.
