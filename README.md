# Trillionnium Chain (TRNM)

**TRNM** is a Rust-native Layer 1 focused on **Decentralized AI Compute** (PoCO).

- Active game workspace: `trillionnium/Cargo.toml` (6 product crates only)
- Current one-page game status: `GAME_STATUS.md`
- Platform workspace: `trillionnium/crates/platform/Cargo.toml`
- Removed legacy-game index: `docs/archive/frozen-legacy-final-index-2026-07-11.md`
- Historical status/archive docs live under `docs/archive/`

## Trillionnium World Client Boundary

- Current game status and honest open gates: `GAME_STATUS.md`
- Historical finite RPG/RTS v1 acceptance baseline (not an overall-vision 100% claim): `docs/development/trnm-deep-rpg-complete-rts-v1-dod.md`
- Canonical RPG/RTS product contract: `docs/development/trillionnium-rpg-rts-closed-loop-v1.md`
- Native playable client: `trillionnium/crates/trnm-first-contact`
- Authoritative campaign aggregate: `trillionnium/crates/trnm-campaign-core`
- Authoritative Bevy-free battle simulation: `trillionnium/crates/trnm-rts-sim`
- The old `trnm-world-bevy`/World/RTS workspace is absent from the current checkout and recoverable only from the indexed Git history.
- Manual playtest entry: `scripts/run_trnm_first_contact.sh`
- Human validation packet: `scripts/check_trnm_human_validation_packet.sh`
- CEX is an optional settlement backend behind the Trillionnium-owned `trnm-economy-protocol`; never use the CEX Web/Matrix World shell as the native game client.
- The native client owns account binding, economic outbox/reconciliation UI and gameplay state. CEX owns wallet/ledger settlement and verified receipts through `/v1/trillionnium/economy/*`.
- Current game boundary gate: `scripts/check_trnm_game_product.sh`
- Current native snapshot (2026-07-12): schema revision 11, version-pinned Term Exchange protocol v2 and RTS/checkpoint v16; a new-save four-battle-to-45-route client journey, multi-beat chapters/epilogues, visible caravan encounters, shared worker/job command authority, 64-sample balance metrics, checkpointed/streamable replay inspection, ten authored maps, animated transparent identity geometry and optional fail-closed PostgreSQL CEX wallet settlement with atomic intent/receipt persistence, escrow and priority compensation. See `GAME_STATUS.md` for exact evidence and open boundaries; public player listings remain release-gated.

The cohort/commercial collection command writes `acceptance/S6_public_launch/latest/cohort-commercial-evidence-collection.json` plus `.md`, listing first-beta participant/session/feedback/signoff evidence and payment/refund/support/legal/operator/traffic drill evidence with privacy boundaries before the strict validator is run.

---

## 1) Project Positioning

TRNM is a Rust L1 protocol for task-based AI compute settlement and verification. Its core design goals are:

- **PoCO state machine** for task lifecycle: create → commit → reveal → challenge → resolve
- **High-concurrency execution** with conflict detection and grouped scheduling
- **Auditable events + stable interfaces** for integration, replay, governance, and operations
- **Worker Agent + CLI** loop from execution to on-chain submission
- **BL09 retirement-prep wording**: the retained `trnm-pouw` crate name and any residual PoUW fields should be read as migration-era compatibility or provenance/audit evidence, not as ongoing payout authority or a default work-unit payout path

---

## 2) Repository Layout (Current)

```text
TrillionniumChain/
├── trillionnium/                    # Rust workspace (current source-of-truth lane)
│   ├── crates/
│   │   ├── trnm-first-contact       # canonical player
│   │   ├── trnm-campaign-core       # save/progression authority
│   │   ├── trnm-economy-protocol     # pure game-owned intent/receipt contract
│   │   ├── trnm-rts-sim             # 2D order-driven battle authority
│   │   ├── trnm-rts-protocol        # lightweight frame-order contract
│   │   ├── trnm-rpg-core            # lightweight RPG vocabulary
│   │   └── platform/                 # separate 12-crate chain/platform workspace
│   ├── configs/
│   ├── scripts/
│   └── run/
├── web4-frontend/                  # Web4 frontend (Next.js + Vitest + Playwright)
├── scripts/                        # Repo-level CI/automation scripts
├── docs/                           # Architecture, protocol, runbooks, reports, and historical archive docs
├── contracts/                      # Rust-native external contracts subtree (4-crate MVP, not full runtime-spec/sdk closure)
├── config/                         # Policy and alerting config
├── examples/                       # SDK and demo examples
├── OPERATIONS.md                   # Operator-facing handbook
└── RELEASE_READINESS.md            # Current release truth source
```

---

## 3) Core Modules

### Rust mainline (`trillionnium/crates`)

- `trnm-node`: node runtime loop, execution wiring, event emission
- `trnm-state`: versioned state store and `state_root`
- `trnm-pouw`: PoCO task state machine and validation logic (legacy crate name retained during migration; do not read it as current payout-authority wording)
- `trnm-executor`: conflict detection and concurrent scheduling strategy
- `trnm-mempool`: transaction pool and admission/packaging
- `trnm-rpc`: RPC service and stable query APIs
- `trnm-worker-agent`: worker execution and on-chain submission path
- `trnm-cli`: native CLI for tx/query operations
- `trnm-bench`: benchmarking and performance tooling
- `trnm-types`: shared protocol types
- `trnm-bridge-poc`: bridge proof-of-concept integration

### Web4 frontend (`web4-frontend`)

- Next.js app shell (`app/`)
- Contract/API adaptation layer (`lib/`)
- Test suites (unit/component/contract/e2e)
- Release preflight scripts in `web4-frontend/scripts/`:
  - `npm run ci:check`
  - `npm run release:preflight`
  - `npm run release:ready`

---

## 4) Quick Start

### 4.1 Environment

- Rust stable (keep aligned with `rust-toolchain`/CI)
- Node.js 20+
- Git

### 4.2 Clone

```bash
git clone https://github.com/ProfAlexQI/TrillionniumChain.git
cd TrillionniumChain
```

### 4.3 Rust mainline smoke

```bash
cd trillionnium
cargo test --workspace
```

### 4.4 Web4 frontend smoke

```bash
cd web4-frontend
npm ci
npm run ci:check
# Force e2e if needed
CI_RUN_E2E=1 npm run ci:check
```

### 4.5 External contracts subtree smoke

```bash
cargo test --manifest-path contracts/Cargo.toml
```

This validates the current `contracts/` MVP workspace only, which today contains `settlement-vault/`, `bridge-relay/`, `governance-guard/`, and `audit-events/`. It should not be read as proof that the target `sdk/`, `runtime-spec/`, `integration-tests/`, or canonical Host ABI/runtime closure already exist in-tree.

---

## 5) Common Repo Commands

### 5.1 Repo-level gates / pipeline

```bash
# Quick gate
./scripts/quick_gate_shell.sh

# Automation pipelines
./scripts/run_100step_pipeline.sh
./scripts/run_200step_pipeline.sh
./scripts/run_200step_v2_pipeline.sh
./scripts/run_codegen_pipeline.sh
```

### 5.2 Worker / Receipt gates

```bash
# Worker receipt gates
./scripts/v2/run_worker_receipt_gates.sh

# Strict real-cli mode
TRNM_TX_CLI=./trillionnium/target/debug/trnm-cli \
  ./scripts/v2/run_worker_receipt_gates_real_cli.sh
```

### 5.3 Tokenomics regression gate

```bash
./scripts/v2/run_tokenomics_r1_r14_regression_gate.sh
```

### 5.4 Historical Trillionnium World release-review surface

The former World/Bevy, OpenRA, Android, S5/S6 and public-launch evidence
scripts are frozen under `scripts/archive/world-bevy/`. They are not product
gates. The current game gate is:

```bash
./scripts/check_trnm_game_product.sh
```

The commands and notes below are retained as historical reference only.

```bash
# Refresh public-launch readiness + release signoff summary, then emit one review JSON.
./scripts/check_trillionnium_world_release_review_quickcheck.sh

# Emit the one-screen release review checklist as JSON + Markdown.
./scripts/check_trillionnium_world_release_review_status.sh

# Emit the external public-launch evidence intake checklist as JSON + Markdown.
./scripts/check_trillionnium_world_public_launch_evidence_intake.sh

# Emit the no-credit evidence template kit for all remaining public-launch blockers; packet integrity binds the JSON + Markdown as public_launch_evidence_kit_semantics and public_launch_evidence_kit_markdown_semantics.
./scripts/check_trillionnium_world_public_launch_evidence_kit.sh

# Emit the checksum-bound operator handoff for collecting real external evidence.
./scripts/check_trillionnium_world_public_launch_operator_handoff.sh

# Prove those no-credit templates fail strict field-level validators.
./scripts/check_trillionnium_world_public_launch_template_negative_fixtures.sh

# Validate a single real-evidence bundle manifest when external evidence is ready.
./scripts/check_trillionnium_world_public_launch_evidence_bundle.sh

# Prove a fake-green bundle manifest pointing at templates is rejected; packet integrity binds this as public_launch_bundle_negative_fixtures_semantics.
./scripts/check_trillionnium_world_public_launch_bundle_negative_fixtures.sh

# Verify readiness blockers match intake items and field-level validator statuses; packet integrity binds this as public_launch_blocker_consistency_semantics.
./scripts/check_trillionnium_world_public_launch_blocker_consistency.sh

# Emit a six-row public-launch blocker execution ledger from readiness, evidence intake, and blocker consistency.
./scripts/check_trillionnium_world_public_launch_blocker_execution_ledger.sh

# Build the production map-pack public evidence collection checklist.
./scripts/check_trillionnium_world_production_map_pack_public_evidence_collection.sh

# Validate production map-pack public evidence before public-launch credit.
./scripts/check_trillionnium_world_production_map_pack_public_evidence.sh

# Build the first-beta/commercial real-evidence collection checklist.
./scripts/check_trillionnium_world_cohort_commercial_evidence_collection.sh

# Validate first-beta cohort and commercial drill evidence before public-launch credit.
./scripts/check_trillionnium_world_cohort_commercial_evidence.sh

# Build the multi-node/live-traffic and public exposure evidence collection checklist.
./scripts/check_trillionnium_world_external_ops_evidence_collection.sh

# Validate multi-node/live-traffic latency and public deploy evidence before public-launch credit.
./scripts/check_trillionnium_world_external_ops_evidence.sh

# Collect S5 Android real-device evidence when an online adb device is attached.
ANDROID_SERIAL=<device-serial> ./scripts/check_trillionnium_world_s5_device_evidence.sh --require-device

# Validate collected S5 real-device evidence before public-launch credit.
./scripts/check_trillionnium_world_s5_real_device_evidence.sh

# Refresh host-side Native/Bevy local playability gates before release-review handoff.
./scripts/check_trillionnium_world_bevy_action_coach.sh
./scripts/check_trillionnium_world_bevy_player_hud_debug_layer.sh
./scripts/check_trillionnium_world_bevy_player_ui_rescue.sh
./scripts/check_trillionnium_world_bevy_live_window_screenshot_sequence.sh
./scripts/check_trillionnium_world_bevy_sprite_texture_sampling.sh
./scripts/check_trillionnium_world_bevy_live_window_sampled_texture_correlation.sh
./scripts/check_trillionnium_world_bevy_render_asset_eligibility.sh
./scripts/check_trillionnium_world_bevy_classic_playtest_handoff_packet.sh

# Refresh desktop real-machine readiness before mobile/S5 evidence.
./scripts/check_trillionnium_world_bevy_desktop_real_machine_readiness.sh

# Emit the UI / map engine / modeling full-alignment matrix.
./scripts/check_trillionnium_world_ui_map_modeling_full_alignment.sh

# Strict mode: fail unless full UI/map/modeling alignment has real external evidence.
./scripts/check_trillionnium_world_ui_map_modeling_full_alignment.sh --require-ready

# Confirm the historical CEX World adapter is retired/blocked and grants no current product credit.
./scripts/check_trillionnium_world_cex_adapter_readiness.sh

# Prove green status-only public-launch evidence fixtures are rejected.
./scripts/check_trillionnium_world_public_launch_status_only_fixtures.sh

# Verify release-review scripts, docs, workflow guards, and evidence outputs stay connected.
./scripts/check_trillionnium_world_release_review_convergence.sh

# Build a checksummed JSON + Markdown review packet for handoff.
./scripts/check_trillionnium_world_release_review_packet.sh

# Recompute packet artifact hashes/sizes and verify the packet has not drifted.
./scripts/check_trillionnium_world_release_review_packet_integrity.sh

# Run the local release-review aggregate: integrity, static guards, README links, workflow refs.
./scripts/check_trillionnium_world_release_review_ci_gate.sh

# Snapshot the current WIP tree into grouped review/commit slices without staging anything.
./scripts/check_trillionnium_world_release_review_checkpoint_manifest.sh

# Emit the post-audit risk/blocker and next-work execution plan.
./scripts/check_trillionnium_world_next_execution_plan.sh

# Emit the current origin/main..HEAD review-slice commit-range manifest.
./scripts/check_trillionnium_world_review_slice_manifest.sh

# Emit the local review triage queue for unclassified and multi-slice commits.
./scripts/check_trillionnium_world_review_triage_queue.sh

# Emit the local review primary-owner plan for triage buckets.
./scripts/check_trillionnium_world_review_primary_owner_plan.sh

# Emit the release/public-boundary owner queue from the primary-owner plan.
./scripts/check_trillionnium_world_review_release_owner_queue.sh

# Emit the RTS runtime/data-boundary owner queue from the primary-owner plan.
./scripts/check_trillionnium_world_review_runtime_owner_queue.sh

# Emit the residual owner-resolution queue for buckets not covered by the release/runtime lanes.
./scripts/check_trillionnium_world_review_residual_queue.sh

# Emit ordered reviewer execution batches across the release/runtime/residual queues.
./scripts/check_trillionnium_world_review_execution_batches.sh

# Emit batch 1 public-boundary commit-level review closure.
./scripts/check_trillionnium_world_review_public_boundary_batch.sh

# Emit batch 2 release/native handoff commit-level review closure.
./scripts/check_trillionnium_world_review_release_native_handoff_batch.sh

# Emit batch 3 runtime/data-boundary shard plan before commit-level review closure.
./scripts/check_trillionnium_world_review_runtime_boundary_batch.sh

# Emit batch 3 sub-batch 1 runtime-core semantics review with boundary follow-up.
./scripts/check_trillionnium_world_review_runtime_core_semantics_batch.sh

# Emit batch 3 sub-batch 2 runtime adapter/online boundary review.
./scripts/check_trillionnium_world_review_runtime_adapter_online_batch.sh

# Emit batch 3 sub-batch 3 OpenRA parity/claim boundary review.
./scripts/check_trillionnium_world_review_openra_parity_claim_batch.sh

# Emit batch 3 sub-batch 4 First Contact RTS data extraction review.
./scripts/check_trillionnium_world_review_first_contact_rts_data_batch.sh

# Emit batch 3 sub-batch 5 RTS evidence crate boundary review.
./scripts/check_trillionnium_world_review_rts_evidence_crate_batch.sh

# Emit batch 3 sub-batch 6 local review evidence exposure review.
./scripts/check_trillionnium_world_review_evidence_exposure_batch.sh

# Emit batch 3 sub-batch 7 Bevy runtime/renderer boundary review.
./scripts/check_trillionnium_world_review_bevy_runtime_renderer_batch.sh

# Emit batch 3 sub-batch 8 First Contact player-surface cue boundary review.
./scripts/check_trillionnium_world_review_first_contact_player_surface_cues_batch.sh

# Emit batch 4 generated count surface owner review.
./scripts/check_trillionnium_world_review_generated_count_surface_batch.sh

# Emit batch 5 docs/plan truth-source review.
./scripts/check_trillionnium_world_review_docs_plan_truth_source_batch.sh

# Emit batch 6 bot/executor surface review.
./scripts/check_trillionnium_world_review_bot_executor_surface_batch.sh

# Strict mode: fail when public-launch blockers remain.
./scripts/check_trillionnium_world_release_review_quickcheck.sh --require-ready
```

Historical release-review note: packets generated before the 2026-07-11 cleanup credited the old CEX World adapter and removed `trnm-world-bevy` evidence chain. Those packets are archived snapshots, not current green product evidence. Current verification is the six-crate test/Clippy/release/live-window chain plus `scripts/check_trnm_game_product.sh`; CEX credit additionally requires the current v2 protocol build and real-ledger E2E.

> **Retired World appendix:** the classic-modeling and release-review notes below document the removed pre-2026-07-11 World workspace. They remain for audit/recovery only, grant no current game credit, and must not override `GAME_STATUS.md`.

Classic modeling foundation note: `scripts/check_trillionnium_world_bevy_classic_art_pack.sh`, `scripts/check_trillionnium_world_bevy_classic_manifest_lint.sh`, and `scripts/check_trillionnium_world_bevy_classic_isometric_modeling.sh` are packet-bound through `classic_asset_pack_semantics`, `classic_manifest_lint_semantics`, `classic_isometric_modeling_semantics`, and `classic_isometric_modeling_ppm_semantics`. These checks bind the project-owned manifest/PPM atlas, frame/scene/actor/clip lint, orthographic isometric depth-sorted modeling, nonblank PPM evidence, and the low-spec renderer CEX/wgpu no-credit boundary.

Classic performance budget note: `scripts/check_trillionnium_world_bevy_classic_input_frame_budget.sh` and `scripts/check_trillionnium_world_bevy_classic_render_budget.sh` are packet-bound through `classic_input_frame_budget_semantics` and `classic_render_budget_semantics`. These checks bind accepted movement input through `NativeControlAction::Move -> apply_live_native_action -> classic_draw_scene`, p95/max input-frame and render budgets, manifest-backed frame selection, nonblank low-spec renderer samples, and the CEX/wgpu no-credit boundary.

Classic playtest runner status note: `scripts/check_trillionnium_world_bevy_classic_playtest_runner_status.sh` is packet-bound through `classic_playtest_runner_status_semantics`. This check binds the live `trillionnium-bevy-playtest.service` process to the release `trnm-world-bevy run` binary, low-spec classic renderer environment, manifest/override paths, working directory, explicit CEX path rejection, and top-level gate/pass/fail counts.

Classic playtest launcher note: `scripts/check_trillionnium_world_bevy_classic_playtest_launcher.sh` is packet-bound through `classic_playtest_launcher_semantics`. This check binds the player-facing `CAMPAIGN:START` / `CAMPAIGN:CONTINUE` / `CAMPAIGN:REPLAY` title actions, campaign slot persistence, `league-coliseum` open-world resume, live release runner, classic renderer environment, and CEX/S5 no-credit boundary.

Classic campaign UI continuity note: `scripts/check_trillionnium_world_bevy_classic_rts_campaign_ui_continuity.sh` is packet-bound through `campaign_ui_continuity_semantics` and `campaign_ui_continuity_ppm_semantics`. This check binds the 16-frame campaign handoff preview, final/restored `league-coliseum` route state, contextual combat action labels, milestone pixels, persistence gates, 1920x1080 PPM evidence, and native-client/S5 no-credit boundary.

Classic visual foundation note: `scripts/check_trillionnium_world_bevy_classic_scene_preview.sh`, `scripts/check_trillionnium_world_bevy_classic_model_catalog.sh`, and `scripts/check_trillionnium_world_bevy_classic_renderer_probe.sh` are status-bound as `classic_scene_preview_green`, `classic_model_catalog_green`, and `classic_renderer_probe_green`. Packet integrity checks `classic_scene_preview_semantics`, `classic_scene_preview_ppm_semantics`, `classic_model_catalog_semantics`, `classic_model_catalog_ppm_semantics`, `classic_renderer_probe_semantics`, and `classic_renderer_probe_ppm_semantics` against manifest-backed scene panels, model catalog frames, renderer probe pixels, PPM evidence, and no-credit boundaries.

Map modeling note: `scripts/check_trillionnium_world_map_modeling_gate.sh` writes `acceptance/S4_map_pack_gate/latest/map-modeling-gate.json`, proving buildings, roads, greenery, and terrain are derivable from deterministic map_pack data while exposing top-level green/no-credit/count fields and still keeping `fixture_only=true` and public-launch credit blocked until signed real map_pack evidence exists. Packet integrity checks `map_modeling_gate_semantics` against those fixture map_pack modeling layers, count fields, no-live-ingestion boundaries, and required production/public evidence blockers.

Checkpoint note: `scripts/check_trillionnium_world_release_review_checkpoint_manifest.sh` writes `acceptance/S6_public_launch/latest/release-review-checkpoint-manifest.json` plus `.md`, grouping the current dirty working tree into review/commit slices without staging, committing, or claiming public-launch evidence.

Next execution plan note: `scripts/check_trillionnium_world_next_execution_plan.sh` writes `acceptance/S6_public_launch/latest/trillionnium-world-next-execution-plan.json` plus `.md`, keeping whole-screen First Contact readability, human playtest path, observer runbook, evidence-volume curation, reviewer handoff index, truth-source hygiene, review slicing, review triage queue, review primary-owner plan, release-owner queue, runtime-owner queue, residual queue, review execution batches, public-boundary batch review, release-native handoff batch review, runtime-boundary batch review, runtime-core semantics batch review, runtime-adapter/online batch review, blocker execution ledger, and real external evidence collection visible after the audit. The local five-step human playtest path is also bound in `scripts/check_trillionnium_world_bevy_classic_playtest_handoff_packet.sh` through packet integrity without granting beta cohort, Android S5 real-device, or public-launch credit. The runbook checker writes `acceptance/S6_public_launch/latest/first-contact-human-playtest-runbook.json` as a no-credit local observation protocol, not human completion evidence. The evidence-volume checker writes `acceptance/S6_public_launch/latest/trillionnium-world-evidence-volume-curation.json`, inventorying large S5 host-side evidence without deleting, compressing, moving, archiving, or granting public/S5 credit. The public-launch blocker execution ledger writes `acceptance/S6_public_launch/latest/trillionnium-world-public-launch-blocker-execution-ledger.json`, joining readiness, evidence intake, and blocker consistency into six real-evidence collection rows while rejecting templates, status-only files, local drills, host-side screenshots, live ingestion, public exposure, and device-capture credit. The reviewer handoff index writes `acceptance/S6_public_launch/latest/trillionnium-world-reviewer-handoff-index.json`, checksum-binding a small reviewer summary, live player screen, representative visuals, raw archive candidates, the primary-owner plan, the release-owner queue, the runtime-owner queue, the residual queue, the review execution batches, the public-boundary batch review, the release-native handoff batch review, the runtime-boundary batch review, the runtime-core semantics batch review, the runtime-adapter/online batch review, and the bot/executor surface batch review without deleting, archiving, uploading, publishing, or granting public/S5 credit. The review-slice checker writes `acceptance/S6_public_launch/latest/trillionnium-world-review-slice-strategy.json`, grouping the local ahead-of-origin backlog into six review topics without push, rebase, reset, public-launch, or Android S5 credit. The review-slice manifest checker writes `acceptance/S6_public_launch/latest/trillionnium-world-review-slice-manifest.json`, binding the current `origin/main..HEAD` commit range to slice counts, sample commits, multi-slice commits, and unclassified manual-review risk without push, rebase, reset, squash, history rewrite, public-launch, or Android S5 credit. The review triage queue writes `acceptance/S6_public_launch/latest/trillionnium-world-review-triage-queue.json`, bucket-covering unclassified and multi-slice commits into reviewer queues while keeping manual review, primary-owner assignment, no push, no history rewrite, and no public/S5 credit explicit. The review primary-owner plan writes `acceptance/S6_public_launch/latest/trillionnium-world-review-primary-owner-plan.json`, assigning bucket-level owners and review order for all 11 triage buckets while keeping commit-level judgment, no push, no history rewrite, and no public/S5 credit explicit. The review release-owner queue writes `acceptance/S6_public_launch/latest/trillionnium-world-review-release-owner-queue.json`, expanding the `release_truth_and_public_boundary` lane into a 58-item commit queue while keeping 35 commit-level primary-owner reviews, 23 truth-source/count reviews, no push, no history rewrite, and no public/S5 credit explicit. The review runtime-owner queue writes `acceptance/S6_public_launch/latest/trillionnium-world-review-runtime-owner-queue.json`, expanding the `rts_runtime_data_boundaries` lane into a 283-item commit queue while keeping 273 commit-level primary-owner reviews, 10 runtime/bot/map boundary reviews, one explicit zero-count map/modeling bucket, no push, no history rewrite, and no public/S5 credit explicit. The review residual queue writes `acceptance/S6_public_launch/latest/trillionnium-world-review-residual-queue.json`, covering the remaining 66 native-Bevy/manual/readability items so the release/runtime/residual queue set fully covers all owner-plan commits without push, history rewrite, upload, publish, or public/S5 credit. The review execution batches write `acceptance/S6_public_launch/latest/trillionnium-world-review-execution-batches.json`, turning those queues into 11 ordered reviewer batches while keeping queue coverage, no history rewrite, no external action, and no public/S5 credit explicit. The public-boundary batch review writes `acceptance/S6_public_launch/latest/trillionnium-world-review-public-boundary-batch.json`, closing batch 1 across six `multi_public_boundary_overlap` commits with zero unresolved public-boundary reviews while keeping all six real-evidence blockers and no public/S5/beta/commercial credit explicit. The release-native handoff batch review writes `acceptance/S6_public_launch/latest/trillionnium-world-review-release-native-handoff-batch.json`, closing batch 2 across 29 `multi_release_native_handoff_overlap` commits with zero unresolved release-native handoff reviews while deferring playable-client/runtime details and keeping no public/S5/beta/commercial credit explicit. The runtime-boundary batch review writes `acceptance/S6_public_launch/latest/trillionnium-world-review-runtime-boundary-batch.json`, sharding batch 3 across 273 `multi_native_bevy_rts_boundary_overlap` commits into eight runtime/data-boundary sub-batches while keeping batch 3 open, batch 4 blocked, and no public/S5/beta/commercial credit explicit. The runtime-core semantics batch review writes `acceptance/S6_public_launch/latest/trillionnium-world-review-runtime-core-semantics-batch.json`, reviewing sub-batch 1 across 55 OpenRA-like RTS core commits with zero per-commit unresolved reviews while retaining one systemic runtime-core source boundary follow-up and no OpenRA runtime/replay/network compatibility, public/S5/beta/commercial credit. The runtime-adapter/online batch review writes `acceptance/S6_public_launch/latest/trillionnium-world-review-runtime-adapter-online-batch.json`, reviewing sub-batch 2 across 57 Bevy runtime adapter/offline-loopback commits with zero per-commit unresolved reviews, resolving the adapter path of the prior source-boundary follow-up, and still keeping batch 3 open plus all socket/hosted-service/client-prediction/rollback/public/S5/beta/commercial/OpenRA compatibility claims false. It stays green with public-launch blockers preserved and does not claim Android S5 real-device readiness or public-launch readiness.

OpenRA parity/claim batch review note: `scripts/check_trillionnium_world_review_openra_parity_claim_batch.sh` writes `acceptance/S6_public_launch/latest/trillionnium-world-review-openra-parity-claim-batch.json`, reviewing batch 3 sub-batch 3 across 35 OpenRA-style parity/replay/UI/asset-scope commits with zero per-commit unresolved reviews. It unblocks the First Contact RTS data-extraction sub-batch while keeping batch 3 open and rejecting OpenRA runtime/replay/network/binary/headless compatibility, OpenRA engine/full-engine/pixel-perfect/Westwood asset parity, third-party asset copy, socket, hosted-service, live multiplayer, public/S5/beta/commercial, and external-action credit.

First Contact RTS data extraction batch review note: `scripts/check_trillionnium_world_review_first_contact_rts_data_batch.sh` writes `acceptance/S6_public_launch/latest/trillionnium-world-review-first-contact-rts-data-batch.json`, reviewing batch 3 sub-batch 4 across 24 First Contact RTS data-extraction commits with zero per-commit unresolved reviews. It unblocks the RTS evidence crate boundary sub-batch while keeping batch 3 open and proving First Contact terrain/opening/player-startup/actor/visual-telemetry/player-screen/chrome/preview/renderer-projection/samples/labels data is renderer-neutral RTS data/evidence input, not draw-math transfer, live Bevy renderer behavior, render-world extraction, GPU upload, public/S5/beta/commercial, or external-action credit.

RTS evidence crate batch review note: `scripts/check_trillionnium_world_review_rts_evidence_crate_batch.sh` writes `acceptance/S6_public_launch/latest/trillionnium-world-review-rts-evidence-crate-batch.json`, reviewing batch 3 sub-batch 5 across 20 First Contact RTS evidence-crate commits with zero per-commit unresolved reviews. It unblocks the local review-evidence exposure sub-batch while keeping batch 3 open and proving evidence payloads, projection checks, readability guards, panel-density guards, marker/focus/atlas guards, and labels stay local `trnm-rts-evidence` review surfaces, not playable renderer ownership, render-world extraction, GPU upload, public/S5/beta/commercial, or external-action credit.

Review evidence exposure batch note: `scripts/check_trillionnium_world_review_evidence_exposure_batch.sh` writes `acceptance/S6_public_launch/latest/trillionnium-world-review-evidence-exposure-batch.json`, reviewing batch 3 sub-batch 6 across 12 local review-artifact exposure commits with zero per-commit unresolved reviews. It unblocks the Bevy runtime/renderer boundary sub-batch while keeping batch 3 open and proving artifact binary reuse, First Contact runtime review exposure, packet assembly review exposure, classic flow review exposure, and visual-flow counts remain local review surfaces, not external evidence, playable renderer ownership, render-world extraction, GPU upload, public/S5/beta/commercial, or external-action credit.

Bevy runtime renderer batch note: `scripts/check_trillionnium_world_review_bevy_runtime_renderer_batch.sh` writes `acceptance/S6_public_launch/latest/trillionnium-world-review-bevy-runtime-renderer-batch.json`, reviewing batch 3 sub-batch 7 across 7 Bevy runtime/renderer boundary commits with zero per-commit unresolved reviews. It unblocks the First Contact player-surface cue sub-batch while keeping batch 3 open and proving First Contact runtime adapter helpers, classic model-catalog renderer gates, and classic asset-boundary renderer splits stay Bevy consumer/renderer surfaces, not RTS data truth sources, playable renderer ownership, render-world extraction, GPU upload, public/S5/beta/commercial, OpenRA compatibility, or external-action credit.

First Contact player-surface cues batch note: `scripts/check_trillionnium_world_review_first_contact_player_surface_cues_batch.sh` writes `acceptance/S6_public_launch/latest/trillionnium-world-review-first-contact-player-surface-cues-batch.json`, reviewing batch 3 sub-batch 8 across 63 player-surface cue commits with zero per-commit unresolved reviews. It closes batch 3 locally and unblocks batch 4 `unclassified_generated_count_surface` while proving the cue work remains downstream renderer/readability polish over established RTS data/evidence/runtime surfaces, not RTS data truth-source moves, human-playtest completion, public/S5/beta/commercial, render-world extraction, GPU upload, OpenRA compatibility, or external-action credit.

Generated count surface batch note: `scripts/check_trillionnium_world_review_generated_count_surface_batch.sh` writes `acceptance/S6_public_launch/latest/trillionnium-world-review-generated-count-surface-batch.json`, reviewing batch 4 across 14 generated count exposure commits with zero unresolved owner assignments. It assigns each count surface to the checker/artifact contract that owns it, binds release CI totals and packet semantic count guards, satisfies batch 4 locally, and unblocks batch 5 `unclassified_docs_plan_truth_source` while keeping the count fields as local generated metadata, not public/S5/beta/commercial, production-ready UI, OpenRA compatibility, external-evidence, render-world extraction, GPU upload, live-traffic, or public-network credit.

Docs/plan truth-source batch note: `scripts/check_trillionnium_world_review_docs_plan_truth_source_batch.sh` writes `acceptance/S6_public_launch/latest/trillionnium-world-review-docs-plan-truth-source-batch.json`, reviewing batch 5 across 9 docs/plan truth-source commits with zero unresolved truth-source routes. It keeps Term Exchange docs as current protocol/migration truth, routes RTS fusion planning to current architecture/reference truth with generated artifacts owning exact latest state, binds packet/checkpoint guard truth to checker/artifact contracts, satisfies batch 5 locally, and unblocks batch 6 `unclassified_bot_executor_surface` while keeping all docs/plan surfaces as local review metadata, not public/S5/beta/commercial, production-ready UI, OpenRA compatibility, external-evidence, render-world extraction, GPU upload, live-traffic, or public-network credit.

Bot/executor surface batch note: `scripts/check_trillionnium_world_review_bot_executor_surface_batch.sh` writes `acceptance/S6_public_launch/latest/trillionnium-world-review-bot-executor-surface-batch.json`, reviewing batch 6 across 10 bot/executor semantic fixture commits with zero unresolved route assignments. It routes bot executor, bot gap, classic RTS local semantic fixture, first-minute rejection, and handoff drift guard commits to packet semantic fixture/guard ownership, satisfies batch 6 locally, and unblocks batch 7 `unclassified_classic_evidence_surface` while keeping Bevy playable integration, public/S5/beta/commercial, external-evidence, OpenRA compatibility, render-world extraction, GPU upload, live-traffic, and public-network credit false.

Classic RTS production-art note: `scripts/check_trillionnium_world_bevy_classic_rts_production_art_replication.sh`, `scripts/check_trillionnium_world_bevy_classic_rts_production_asset_atlas.sh`, `scripts/check_trillionnium_world_bevy_classic_rts_production_ui_skin.sh`, `scripts/check_trillionnium_world_bevy_classic_rts_production_interaction_polish.sh`, `scripts/check_trillionnium_world_bevy_classic_rts_full_screen_ui_replication.sh`, `scripts/check_trillionnium_world_bevy_classic_rts_shell_meta_ui_replication.sh`, `scripts/check_trillionnium_world_bevy_classic_rts_match_setup_ui_replication.sh`, `scripts/check_trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness.sh`, `scripts/check_trillionnium_world_bevy_classic_rts_campaign_ui_continuity.sh`, `scripts/check_trillionnium_world_bevy_classic_rts_in_match_hud_state_replication.sh`, `scripts/check_trillionnium_world_bevy_classic_rts_session_state_continuity.sh`, `scripts/check_trillionnium_world_bevy_classic_rts_combat_readability_pressure_readiness.sh`, and `scripts/check_trillionnium_world_bevy_classic_rts_production_desktop_review_packet.sh` bind original Bevy production art, texture-atlas slices, the player runtime production HUD skin screen, the player runtime command interaction screen for drag/right-click/attack/build/queue/scroll feedback, ten Rust/Bevy full-screen runtime UI surfaces, twelve title/account/session/save/pause/settings/input shell/meta runtime UI surfaces, ten pre-match map/faction/spawn/resource/victory/minimap/start-ready setup surfaces, the first-minute/victory/base-assault/aftermath/open-world campaign outcome runtime screen, the restored campaign/open-world UI continuity handoff, the live in-match resource/selection/command/minimap/production/ability/combat/objective HUD state snapshot, the save-slot/load-resume/continue-unlock session state continuity chain, the five-surface combat readability/pressure runtime screen, and the local Linux desktop keyboard/mouse review packet into classic playtest readiness plus release-review CI while keeping no-copy, Android S5, GPU-upload, production-ready UI, and public-launch blockers explicit. Classic playtest readiness plus the playtest launcher, campaign UI continuity JSON/PPM, full-screen UI replication, shell/meta UI replication, match setup UI replication, campaign outcome UI readiness, in-match HUD/state replication, session state continuity, combat readability/pressure readiness, camera/minimap sync JSON/PPM, production desktop review packet, playtest handoff readiness/packet/Markdown, public-launch evidence intake JSON/Markdown, operator handoff JSON/Markdown, evidence kit JSON/Markdown, template-negative fixture, evidence bundle JSON/Markdown, bundle-negative fixture, and live-window mouse hit-test sequence are checksum-bound inside the release-review packet; packet integrity verifies the current packet at artifact count `128` with direct `classic_playtest_readiness_semantics`, `classic_playtest_launcher_semantics`, `classic_playtest_handoff_readiness_semantics`, `classic_playtest_handoff_packet_semantics`, `classic_playtest_handoff_packet_human_task_path_semantics`, `classic_playtest_handoff_packet_markdown_semantics`, `public_launch_evidence_intake_semantics`, `public_launch_evidence_intake_markdown_semantics`, `production_map_pack_public_evidence_collection_semantics`, `cohort_commercial_evidence_collection_semantics`, `external_ops_evidence_collection_semantics`, `production_map_pack_public_evidence_semantics`, `cohort_commercial_evidence_semantics`, `external_ops_evidence_semantics`, `s5_real_device_evidence_semantics`, `public_launch_evidence_kit_semantics`, `public_launch_evidence_kit_markdown_semantics`, `public_launch_template_negative_fixtures_semantics`, `public_launch_evidence_bundle_semantics`, `public_launch_evidence_bundle_markdown_semantics`, `public_launch_bundle_negative_fixtures_semantics`, `public_launch_status_only_fixture_guard_semantics`, `public_launch_operator_handoff_semantics`, `public_launch_operator_handoff_markdown_semantics`, `full_screen_ui_replication_semantics`, `shell_meta_ui_replication_semantics`, `match_setup_ui_replication_semantics`, `campaign_outcome_ui_readiness_semantics`, `campaign_ui_continuity_semantics`, `campaign_ui_continuity_ppm_semantics`, `in_match_hud_state_replication_semantics`, `session_state_continuity_semantics`, `combat_readability_pressure_readiness_semantics`, `classic_rts_camera_minimap_sync_semantics`, `classic_rts_camera_minimap_sync_ppm_semantics`, `production_desktop_review_packet_semantics`, and `live_window_mouse_hit_test_sequence_semantics`. The full-screen and desktop packet semantic checks now require the production HUD skin and command interaction runtime-screen fields, so those layers cannot pass as board-only evidence. The classic playtest readiness artifact is now status-bound and semantics-bound to the aggregate Bevy RTS playable slice without granting S5, public-launch, production-ready UI, or OpenRA-copy credit.

Classic RTS continuous player-flow note: `scripts/check_trillionnium_world_bevy_classic_rts_continuous_player_flow.sh` binds the title/account shell, match setup, in-match HUD, command feedback, save/load resume, campaign outcome, and open-world continuity Rust/Bevy screens into one six-step player-flow runtime surface, then feeds that gate into classic playtest readiness and release-review CI without granting S5, public-launch, production-ready UI, or OpenRA screen-for-screen credit.

Classic RTS OpenRA-style foundation note: `scripts/check_trillionnium_world_bevy_classic_rts_openra_engine_port_asset_parity.sh` keeps the legacy artifact name but now claims only scoped Rust OpenRA-style foundation analogues plus owned-asset manifest parity. It binds ModData/ruleset/actor/world/order/chrome/widget/sprite/palette/asset-loader/replay-like surfaces, reads the canonical OpenRA screen-for-screen UI evidence as comparison context, and proves pixel-perfect parity across the full Trillionnium-owned OpenRA-compatible PPM asset manifest with zero sha or pixel mismatches. Release-review packet integrity binds `openra_engine_port_asset_parity_semantics` and `openra_engine_port_asset_parity_ppm_semantics`; completed OpenRA engine port, OpenRA game-screen parity, OpenRA/Westwood asset copying, binary replay/network parity, Android S5, and public launch remain unclaimed.

The status command also writes `acceptance/S6_public_launch/latest/release-review-status.md`, a compact checklist of what is green for review and what still needs real external evidence. The CEX adapter command now emits a retired/blocked result. Cached 2026-06 evidence was removed because the separate CEX checkout no longer resolves its old World trait manifests; it grants no current product or release-review credit. The Bevy classic playtest handoff packet writes `acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-handoff-packet.json` plus `.md`, checksum-binding the local human-playtest readiness, launcher, runner, and observability evidence without granting public-launch or S5 real-device credit. The desktop real-machine command writes `acceptance/S5_native_bevy_device/latest/bevy-desktop-real-machine-readiness.json` plus `.md`, binding the local Linux X11 Bevy window, screenshots, XTest keyboard input, pixel/texture correlation, release runner, and handoff packet before mobile/S5 work. The UI/map/modeling full-alignment command writes `acceptance/S6_public_launch/latest/trnm-world-ui-map-modeling-full-alignment.json` plus `.md`, proving host-side UI design, desktop real-machine readiness, fixture map modeling, and original low-spec modeling alignment while keeping production map-pack, S5 real-device, and public-launch blockers explicit. The public-launch readiness command now consumes sprite texture sampling, sampled texture live-window correlation, and render asset eligibility as first-class local gates before the release-review chain can stay green; packet integrity binds its local Bevy gates, exact external blockers, blocked real-evidence statuses, and no live-ingestion/public-network credit boundary. The evidence intake command writes `acceptance/S6_public_launch/latest/public-launch-evidence-intake.json` plus `.md`, turning the remaining external blockers into explicit evidence paths, collection commands, and env hooks without claiming public launch readiness; both files are checksum-bound in the release-review packet, and packet integrity binds the six collection items, present/missing split, env hooks, collection commands, Markdown checklist, the three collection artifact checklists, the three validator summaries, the S5 real-device validator summary, and no-approval boundary. The evidence kit command writes `acceptance/S6_public_launch/latest/public-launch-evidence-kit.json` plus `.md`, generating no-credit templates, collection commands, and validator commands for all six external blockers. The template negative fixtures command writes `acceptance/S6_public_launch/latest/public-launch-template-negative-fixtures.json` and proves those templates fail strict field-level validators; packet integrity binds all four template rejection cases, six template paths, blocked statuses, and no-claim boundary. The evidence bundle command writes `acceptance/S6_public_launch/latest/public-launch-evidence-bundle.json` plus `.md`, validating a single operator-supplied manifest that points to all six real evidence files; both files are checksum-bound in the release-review packet, and packet integrity binds the default blocked manifest gate, six evidence item statuses, validator failures, no-credit template boundary, and Markdown checklist.  The bundle negative fixtures command writes `acceptance/S6_public_launch/latest/public-launch-bundle-negative-fixtures.json` and proves a fake-green bundle manifest pointing at templates is rejected. The blocker consistency command writes `acceptance/S6_public_launch/latest/public-launch-blocker-consistency.json` and proves readiness blockers still match intake items plus field-level validator statuses. The production map-pack collection command writes `acceptance/S4_map_pack_gate/latest/production-map-pack-public-evidence-collection.json` plus `.md`, listing the real source, ODbL, attribution, sensitive POI, geofence, key custody, distribution/revocation, rollback, and signoff artifacts required without doing live ingestion. The production map-pack public evidence command writes `acceptance/S4_map_pack_gate/latest/production-map-pack-public-evidence.json` and validates those artifacts before public-launch credit. The cohort/commercial evidence command writes `acceptance/S6_public_launch/latest/cohort-commercial-evidence.json` and validates first-beta plus commercial drill fields instead of trusting status-only files. The external ops evidence command writes `acceptance/S6_public_launch/latest/external-ops-evidence.json` and validates multi-node/live-traffic latency plus public deploy fields instead of treating local drills as launch credit. The S5 collection command writes `acceptance/S5_native_bevy_device/latest/s5-device-evidence.json` plus adb screenshot/gfxinfo/logcat/lifecycle artifacts when an online Android device is attached; the S5 validator writes `acceptance/S5_native_bevy_device/latest/s5-real-device-evidence-validation.json` and validates real-device screenshot/gfxinfo/logcat/lifecycle/crash-free evidence instead of accepting host-side replay credit. The status-only fixture guard writes `acceptance/S6_public_launch/latest/public-launch-status-only-fixtures.json` and proves fake green evidence files are rejected by the field-level validators; packet integrity binds all four status-only rejection results, blocked statuses, and blocker-present checks. The convergence command writes `acceptance/S6_public_launch/latest/release-review-convergence.json` and catches disconnected README/docs/workflow/evidence entry points. The packet command writes `acceptance/S6_public_launch/latest/release-review-packet.json` plus `.md`, including checksums for the review evidence bundle. The integrity command writes `acceptance/S6_public_launch/latest/release-review-packet-integrity.json` and recomputes those checksums before handoff. The CI gate writes `acceptance/S6_public_launch/latest/release-review-ci-gate.json` as the local aggregate for review handoff.

Operator handoff note: scripts/check_trillionnium_world_public_launch_operator_handoff.sh writes acceptance/S6_public_launch/latest/public-launch-operator-handoff.json plus .md; both files are checksum-bound in the release-review packet, and packet integrity verifies the six collection actions, templates, validator commands, bundle template, negative fixtures, artifact checksums, and no-credit boundary without granting public-launch credit.

---

## 6) Documentation Entry Points

- Release/truth source entry: [RELEASE_READINESS.md](RELEASE_READINESS.md)
  - When referencing this file, include the current `git rev-parse origin/main` value to avoid using stale commit hashes as current truth.
- Project status log: [docs/archive/root-history/STATUS.md](docs/archive/root-history/STATUS.md)
- Historical roadmap: [docs/archive/root-history/ROADMAP.md](docs/archive/root-history/ROADMAP.md)
- Historical backlog snapshots: [docs/archive/root-history/BACKLOG.md](docs/archive/root-history/BACKLOG.md)
- Unified development scheduling: historical planning boards have existed under archived docs, but if a referenced planning-board file is absent in this checkout, use repository docs under `docs/`, `trillionnium/docs/`, and the subproject READMEs as the live execution entrypoints instead.
- Current RPG/RTS game status: [GAME_STATUS.md](GAME_STATUS.md)
- Canonical RPG→RTS→RPG authority contract: [docs/development/trillionnium-rpg-rts-closed-loop-v1.md](docs/development/trillionnium-rpg-rts-closed-loop-v1.md)
- Frozen World/Bevy development documents: [docs/archive/world-bevy/README.md](docs/archive/world-bevy/README.md)
- External benchmark comparison: [docs/reports/TRNM_CONCURRENCY_COMPARISON_2026-03-05.md](docs/reports/TRNM_CONCURRENCY_COMPARISON_2026-03-05.md)
- Concurrency bottleneck map + 8-week roadmap: if an older report is referenced from `RELEASE_READINESS.md` but not present in this checkout, treat it as historical only and do not cite it as current local truth.
- Web4 platform overview: if an older master-planning file is absent in this checkout, treat `RELEASE_READINESS.md`, `docs/reports/TRNM_WEB4_PLATFORM_SCORECARD_2026-03-31.md`, `web4-frontend/docs/README.md`, and `web4-frontend/README.md` as the current Web4 truth-source entrypoints.
- Rust-native external contracts baseline architecture: [trillionnium/docs/protocol/external-contracts-rust/RUST_NATIVE_EXTERNAL_CONTRACTS_ARCH_2026-03-05.md](trillionnium/docs/protocol/external-contracts-rust/RUST_NATIVE_EXTERNAL_CONTRACTS_ARCH_2026-03-05.md)
- `contracts/` status and boundaries: [contracts/README.md](contracts/README.md)
- Historical path note for this perimeter: if an older prompt/doc still says `trillionnium-rust/docs/...` or `contracts-rust/...`, treat that as drift only. The current in-tree truth paths are `trillionnium/docs/...` and `contracts/...`.
- PoCO mechanism (challenge-economics / PoUW minimal packet): [trillionnium/docs/challenge-economics-minimal.md](trillionnium/docs/challenge-economics-minimal.md)
- A2A adapter contract: [docs/agent/a2a_adapter_contract_v1.md](docs/agent/a2a_adapter_contract_v1.md)
- MCP adapter contract: [docs/agent/mcp_adapter_contract_v1.md](docs/agent/mcp_adapter_contract_v1.md)
- Operations handbook: [OPERATIONS.md](OPERATIONS.md)
- OpenClaw ops micro-runbook: [docs/development/OPENCLAW_OPS_MICRO_RUNBOOK.md](docs/development/OPENCLAW_OPS_MICRO_RUNBOOK.md)
- Web4 frontend overview / quickstart: [web4-frontend/README.md](web4-frontend/README.md)
- Web4 documentation center (primary docs entrypoint for operator/developer guidance):
  - [web4-frontend/docs/README.md](web4-frontend/docs/README.md)
  - [web4-frontend/docs/developer-guide.md](web4-frontend/docs/developer-guide.md)
  - [web4-frontend/docs/api-contract.md](web4-frontend/docs/api-contract.md)
  - [web4-frontend/docs/testing-ci.md](web4-frontend/docs/testing-ci.md)
  - [web4-frontend/docs/operations-runbook.md](web4-frontend/docs/operations-runbook.md)
  - [web4-frontend/docs/release-checklist.md](web4-frontend/docs/release-checklist.md)

> Quick link check: run `./scripts/check_root_readme_local_links.sh` to verify local links.

---

## 7) CI / Workflows

The repo runs multiple chain/frontend pipelines under `.github/workflows/`, including:

- `trnm-merge-gates.yml`
- `rust-l1-nightly-health.yml`
- `trnm-gate-quick-check.yml`
- `web4-frontend-ci.yml`

Please run the local minimum gates before creating PRs to reduce CI turnarounds.

---

## 8) Current State Notes (Operational Boundaries)

- Main development entry is `trillionnium/`.
- Historical/archive material in this repo currently lives under `docs/archive/`; do not assume a top-level `legacy/` directory exists in every snapshot.
- Whether the project is currently **release-ready** is defined by [RELEASE_READINESS.md](RELEASE_READINESS.md); historical evidence documents are not automatically equivalent to live state.
- `contracts/` is an **independent Rust-native external-contract subtree / MVP contract scaffolding**. Today it contains 4 landed crates: `settlement-vault/`, `bridge-relay/`, `governance-guard/`, and `audit-events/`.
- `contracts/` is **not yet** the full `sdk / runtime-spec / integration-tests` target layout, and its current crates should not be described as completed Host ABI/runtime integration.
- `audit-events/` under `contracts/` is a shared audit-event schema-adjacent layer; it is not a proof that canonical `sdk`, `runtime-spec`, or `wasm32-unknown-unknown` Host ABI/runtime integration is complete.
- Presence of `contracts/` does **not** by itself move external contracts into Day-1 mainnet minimum scope; that boundary still follows `RELEASE_READINESS.md` plus `trillionnium/docs/release/TRNM_MAINNET_GAP_MATRIX_2026-03-26.md`.
- When validating or citing this perimeter, prefer current-tree paths and commands, for example `cargo test --manifest-path contracts/Cargo.toml`; do not treat historical `contracts-rust/Cargo.toml` references as live workspace truth.
- Web4 currently uses a read-only API client by default; it falls back to local mock snapshots only when explicitly launched with `?mode=mock`, and write paths are not exposed by default.
- If you see `/api/v0/web4/*` references in docs, treat them as historical naming only; current frontend consumption is around:
  - `query-task`
  - `query-events`
  - `query-capability-audit`
  - `query-normalized-audit-events`

### Read-surface contract (important for integration)

- The following endpoints are the current minimal read contract:
  - `query-task/<task_id>`
  - `query-events/<task_id>?limit=<n>`
  - `query-capability-audit/<subject-or-token>`
  - `query-normalized-audit-events?source=<source>&eventType=<eventType>&cursor=<cursor>&limit=<n>`
- `query-task/<task_id>` prefers persisted state snapshots first, then replay over canonical node event history. Adapter fallback may only enrich `Committed`/`Revealed` views when persisted commit history exists.
- For `query-events/<task_id>`, adapter fallback is strictly bounded/deduplicated to recent commit/reveal tails; it must not invent pre-commit history.
- For durable indexer/archive replica planning, persist canonical node event streams rather than relying long-term on adapter fallback.
- Reference: [TRNM_DAY1_PUBLIC_READ_CONTRACT_2026-04-03.md](trillionnium/docs/release/TRNM_DAY1_PUBLIC_READ_CONTRACT_2026-04-03.md) and [TRNM_DAY1_PUBLIC_READ_CONTRACT_MATRIX_2026-04-03.md](trillionnium/docs/release/TRNM_DAY1_PUBLIC_READ_CONTRACT_MATRIX_2026-04-03.md)
- These two documents freeze the **Day-1 minimum public read surface only**, not full durable-indexer/archival read-model readiness.
- `query-events/<task_id>` defaults to `100` and hard-caps at `500`; no assumption of infinite window.
- `query-events/<task_id>` currently accepts only one `limit` key. Unknown keys, duplicated `limit`, case variants (`Limit=`), empty values, or query smuggling are fail-closed.
- `query-capability-audit/<subject-or-token>` supports both capability token and subject DID.
- `query-normalized-audit-events` currently accepts only `source / eventType / cursor / limit`; unknown keys, repeated keys, case variants (`Limit=` / `Source=` / `eventtype=` / `Cursor=`), empty values, and smuggling are fail-closed.
- Paths with a single trailing slash are accepted for:
  - `query-task/<id>/`
  - `query-events/<id>/`
  - `query-capability-audit/<subject>/`
  - but **not** for `query-normalized-audit-events/` (currently exact path only).
- For `query-events/<id>/`, the `limit` query contract is unchanged: `?limit=<n>` still parses normally, default remains `100`, and the same fail-closed rules apply.
- All read endpoints remain fail-closed for extra segments, raw/encoded slash tricks, and query/fragment smuggling.

### Explorer scaffold (operator-facing)

Current explorer service in this repo is an operator-facing scaffold, not a production durable indexer.

Typical commands (run from repo root):

```bash
./trillionnium/scripts/v2/explorer_service_up.sh
./trillionnium/scripts/v2/explorer_service_status.sh
./trillionnium/scripts/v2/explorer_service_down.sh
```

Or from inside `trillionnium/`:

```bash
./scripts/v2/explorer_service_up.sh
./scripts/v2/explorer_service_status.sh
./scripts/v2/explorer_service_down.sh
```

- Service status defaults to `http://127.0.0.1:8090/healthz`.
- Environment overrides: `EXPLORER_HOST`, `EXPLORER_PORT`, `EXPLORER_PUBLIC_BASE_URL`, `EXPLORER_HEALTH_URL`, and `EXPLORER_RPC_BASE_URL`.
- If `trillionnium/run/explorer-service/explorer-service.env` exists, the scripts load it automatically and preserve it as the operator-local source of truth across `up` / `status` / `down`.
- For external exposure, prefer loopback-bound bind + reverse proxy, and keep the emitted `local_health_url` as the local liveness target even when `public_base_url` points at a proxy-facing URL.
- `explorer_service_status.sh` reports `pid_file`, `log_file`, `public_base_url`, `health_url`, `local_health_url`, `rpc_base_url`, and explicitly marks `service_mode=operator-facing-static-scaffold`, `production_ready=false`.
- To capture one deterministic operator handoff packet for this scaffold, use:

```bash
./trillionnium/scripts/v2/capture_explorer_scaffold_handoff.sh
```

- That helper is intentionally **placeholder-only**. It preserves blocker markers such as `deployment_evidence_scope=placeholder-only`, `rank1_read_surface_blocker=still-open`, and `durable_indexer_status=not-implemented-in-this-scaffold`, and it rejects drift if fetched `index.json` no longer matches the scaffold contract.
- The emitted `summary.txt` is also a template-boundary packet, not just a file list. Reuse `template_selection`, `durable_template_allowed`, `durable_template_rejection_reason`, and every `truth_source_*` line verbatim instead of paraphrasing the scaffold into a durable-service handoff.
- Build the packet from `explorer_service_status.sh` output first, then reuse the emitted `index_url`, `health_url`, and `local_health_url` instead of hand-typing proxy/local URLs from shell memory. If you also fetch a reverse-proxy/public URL, attach it as separate evidence rather than replacing the local status-driven proof.
- When the public URL and local bind target differ, preserve the emitted `local_index_url` and `local_index_fetch_command` from `summary.txt` too. Do not reconstruct the local `/index.json` path by editing the public URL by hand.
- Current scaffold intentionally keeps durable-read anchors fail-closed:
  - `ingestion_source`
  - `checkpoint_store`
  - `replay_start_anchor`
  - `retention_scope`
  - `archive_owner`
  - `lag_slo`
- In handoff notes, include flags such as:
  - `deployment_evidence_scope=placeholder-only`
  - `rank1_read_surface_blocker=still-open`
  - `durable_indexer_status=not-implemented-in-this-scaffold`
  - `durable_read_anchor_complete=false`
- Use `trillionnium/docs/release/TRNM_EXPLORER_SCAFFOLD_HANDOFF_TEMPLATE_2026-04-04.md` for this scaffold path.
- Operator bring-up / reverse-proxy / systemd details live in `trillionnium/docs/runbooks/explorer-service-scaffold.md`; use that runbook to keep `health_url` and `local_health_url` aligned with the same deployed instance you hand off.
- Do **not** switch to `TRNM_DURABLE_READ_SERVICE_HANDOFF_TEMPLATE_2026-04-04.md` until all six durable-read anchors exist, replay/restore/lag evidence is attached in the same packet, and `summary.txt` reports `durable_template_allowed=true`.
- When the lane moves beyond placeholder scaffold work, the next implementation boundary is `trillionnium/docs/release/TRNM_RANK1_IMPLEMENTATION_DESIGN_PACKET_2026-04-05.md`, which defines the durable indexer/read-model path rather than more scaffold polish.

Only switch to durable handoff templates when all six durable-read anchors are truly implemented.

---

## License

Internal / proprietary. This repository does not grant a public open-source
license for Trillionnium-owned code.

Third-party source, assets, tools, and references remain under their own
licenses. Direct imports or derived work from OpenRA, Digital Extinction,
Kiomet, Kodiak, or other upstream projects must be recorded with their source
commit, component license, and attribution/notice requirements before release.
The current RTS audit manifest is
`docs/architecture/rts-third-party-source-manifest-2026-06-11.md`.
