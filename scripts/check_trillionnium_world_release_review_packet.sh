#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
PACKET_JSON="$ACCEPTANCE_DIR/release-review-packet.json"
PACKET_MD="$ACCEPTANCE_DIR/release-review-packet.md"
if [[ -v TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_JSON && -n "$TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_JSON" ]]; then
  PACKET_JSON="$TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_JSON"
fi
if [[ -v TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_MD && -n "$TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_MD" ]]; then
  PACKET_MD="$TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_MD"
fi

CONVERGENCE_JSON="$ACCEPTANCE_DIR/release-review-convergence.json"
STATUS_JSON="$ACCEPTANCE_DIR/release-review-status.json"
STATUS_MD="$ACCEPTANCE_DIR/release-review-status.md"
CONVERGENCE_LOG="$ACCEPTANCE_DIR/release-review-packet-convergence.log"
INTAKE_LOG="$ACCEPTANCE_DIR/release-review-packet-evidence-intake.log"
BLOCKER_CONSISTENCY_LOG="$ACCEPTANCE_DIR/release-review-packet-blocker-consistency.log"
EVIDENCE_KIT_LOG="$ACCEPTANCE_DIR/release-review-packet-evidence-kit.log"
OPERATOR_HANDOFF_LOG="$ACCEPTANCE_DIR/release-review-packet-operator-handoff.log"
TEMPLATE_NEGATIVE_LOG="$ACCEPTANCE_DIR/release-review-packet-template-negative-fixtures.log"
EVIDENCE_BUNDLE_LOG="$ACCEPTANCE_DIR/release-review-packet-evidence-bundle.log"
BUNDLE_NEGATIVE_LOG="$ACCEPTANCE_DIR/release-review-packet-bundle-negative-fixtures.log"
MAP_MODELING_GATE_LOG="$ACCEPTANCE_DIR/release-review-packet-map-modeling-gate.log"
CEX_ADAPTER_LOG="$ACCEPTANCE_DIR/release-review-packet-cex-adapter-readiness.log"
CHECKPOINT_MANIFEST_LOG="$ACCEPTANCE_DIR/release-review-packet-checkpoint-manifest.log"
PACKET_INTEGRITY_SEMANTIC_FIXTURE_LOG="$ACCEPTANCE_DIR/release-review-packet-integrity-semantic-fixture.log"
PACKET_INTEGRITY_BOT_EXECUTOR_SEMANTIC_FIXTURE_LOG="$ACCEPTANCE_DIR/release-review-packet-integrity-bot-executor-semantic-fixture.log"
PACKET_INTEGRITY_BOT_EXECUTOR_MATRIX_SEMANTIC_FIXTURE_LOG="$ACCEPTANCE_DIR/release-review-packet-integrity-bot-executor-matrix-semantic-fixture.log"
PACKET_INTEGRITY_BOT_GAP_SEMANTIC_FIXTURE_LOG="$ACCEPTANCE_DIR/release-review-packet-integrity-bot-gap-semantic-fixture.log"
PACKET_INTEGRITY_CONTROL_LOOP_SEMANTIC_FIXTURE_LOG="$ACCEPTANCE_DIR/release-review-packet-integrity-control-loop-semantic-fixture.log"
PACKET_INTEGRITY_SELECTION_MINIMAP_SEMANTIC_FIXTURE_LOG="$ACCEPTANCE_DIR/release-review-packet-integrity-selection-minimap-semantic-fixture.log"
PACKET_INTEGRITY_BUILD_LIFECYCLE_SEMANTIC_FIXTURE_LOG="$ACCEPTANCE_DIR/release-review-packet-integrity-build-lifecycle-semantic-fixture.log"
PACKET_INTEGRITY_TECH_TREE_SEMANTIC_FIXTURE_LOG="$ACCEPTANCE_DIR/release-review-packet-integrity-tech-tree-semantic-fixture.log"
PACKET_INTEGRITY_PROJECTILE_ABILITY_SEMANTIC_FIXTURE_LOG="$ACCEPTANCE_DIR/release-review-packet-integrity-projectile-ability-semantic-fixture.log"
SELECTION_MINIMAP_LOG="$ACCEPTANCE_DIR/release-review-packet-selection-minimap.log"
BUILD_LIFECYCLE_LOG="$ACCEPTANCE_DIR/release-review-packet-build-lifecycle.log"
TECH_TREE_LOG="$ACCEPTANCE_DIR/release-review-packet-tech-tree.log"
PROJECTILE_ABILITY_LOG="$ACCEPTANCE_DIR/release-review-packet-projectile-ability.log"
COMBAT_READABILITY_PRESSURE_READINESS_LOG="$ACCEPTANCE_DIR/release-review-packet-combat-readability-pressure-readiness.log"
CAMERA_MINIMAP_SYNC_LOG="$ACCEPTANCE_DIR/release-review-packet-camera-minimap-sync.log"
BOT_DECISION_STATE_GAP_LOG="$ACCEPTANCE_DIR/release-review-packet-bot-decision-state-gap.log"
BOT_ADAPTIVE_BUILD_ORDER_GAP_LOG="$ACCEPTANCE_DIR/release-review-packet-bot-adaptive-build-order-gap.log"
BOT_TACTICAL_MICRO_GAP_LOG="$ACCEPTANCE_DIR/release-review-packet-bot-tactical-micro-gap.log"
BOT_MAP_INTEL_GAP_LOG="$ACCEPTANCE_DIR/release-review-packet-bot-map-intel-gap.log"
FIRST_CONTACT_BASIN_SPEC_LOG="$ACCEPTANCE_DIR/release-review-packet-first-contact-basin-spec.log"
PLAYTEST_HANDOFF_PACKET_LOG="$ACCEPTANCE_DIR/release-review-packet-playtest-handoff-packet.log"
WORLD_BEVY_RELEASE_BUILD_LOG="$ACCEPTANCE_DIR/release-review-packet-world-bevy-release-build.log"
ARTIFACTS_FILE="$(mktemp)"
trap 'rm -f "$ARTIFACTS_FILE"' EXIT

mkdir -p "$ACCEPTANCE_DIR"

if [[ "${TRNM_RELEASE_REVIEW_PACKET_USE_RELEASE_ARTIFACT_BIN:-1}" != "0" && -z "${TRNM_WORLD_BEVY_ARTIFACT_BIN:-}" ]]; then
  (
    cd "$ROOT/trillionnium"
    CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" cargo build --release -p trnm-world-bevy --bin trnm-world-bevy
  ) >"$WORLD_BEVY_RELEASE_BUILD_LOG" 2>&1
  export TRNM_WORLD_BEVY_ARTIFACT_BIN="$ROOT/target/release/trnm-world-bevy"
fi

"$ROOT/scripts/check_trillionnium_world_release_review_convergence.sh" >"$CONVERGENCE_LOG"
"$ROOT/scripts/check_trillionnium_world_public_launch_evidence_intake.sh" >"$INTAKE_LOG"
"$ROOT/scripts/check_trillionnium_world_public_launch_blocker_consistency.sh" >"$BLOCKER_CONSISTENCY_LOG"
"$ROOT/scripts/check_trillionnium_world_public_launch_evidence_kit.sh" >"$EVIDENCE_KIT_LOG"
"$ROOT/scripts/check_trillionnium_world_public_launch_operator_handoff.sh" >"$OPERATOR_HANDOFF_LOG"
"$ROOT/scripts/check_trillionnium_world_public_launch_template_negative_fixtures.sh" >"$TEMPLATE_NEGATIVE_LOG"
"$ROOT/scripts/check_trillionnium_world_public_launch_evidence_bundle.sh" >"$EVIDENCE_BUNDLE_LOG"
"$ROOT/scripts/check_trillionnium_world_public_launch_bundle_negative_fixtures.sh" >"$BUNDLE_NEGATIVE_LOG"
"$ROOT/scripts/check_trillionnium_world_map_modeling_gate.sh" >"$MAP_MODELING_GATE_LOG"
"$ROOT/scripts/check_trillionnium_world_cex_adapter_readiness.sh" >"$CEX_ADAPTER_LOG"
"$ROOT/scripts/check_trillionnium_world_release_review_checkpoint_manifest.sh" >"$CHECKPOINT_MANIFEST_LOG"
"$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_semantic_fixture.sh" >"$PACKET_INTEGRITY_SEMANTIC_FIXTURE_LOG"
TRNM_RELEASE_REVIEW_PACKET_INTEGRITY_SOURCE_CHAIN_REFRESH=0 "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_bot_executor_semantic_fixture.sh" >"$PACKET_INTEGRITY_BOT_EXECUTOR_SEMANTIC_FIXTURE_LOG"
TRNM_RELEASE_REVIEW_PACKET_INTEGRITY_SOURCE_CHAIN_REFRESH=0 "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_bot_executor_matrix_semantic_fixture.sh" >"$PACKET_INTEGRITY_BOT_EXECUTOR_MATRIX_SEMANTIC_FIXTURE_LOG"
TRNM_RELEASE_REVIEW_PACKET_INTEGRITY_SOURCE_CHAIN_REFRESH=0 "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_bot_gap_semantic_fixture.sh" >"$PACKET_INTEGRITY_BOT_GAP_SEMANTIC_FIXTURE_LOG"
TRNM_RELEASE_REVIEW_PACKET_INTEGRITY_SOURCE_CHAIN_REFRESH=0 "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_control_loop_semantic_fixture.sh" >"$PACKET_INTEGRITY_CONTROL_LOOP_SEMANTIC_FIXTURE_LOG"
TRNM_RELEASE_REVIEW_PACKET_INTEGRITY_SOURCE_CHAIN_REFRESH=0 "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_selection_minimap_semantic_fixture.sh" >"$PACKET_INTEGRITY_SELECTION_MINIMAP_SEMANTIC_FIXTURE_LOG"
TRNM_RELEASE_REVIEW_PACKET_INTEGRITY_SOURCE_CHAIN_REFRESH=0 "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_build_lifecycle_semantic_fixture.sh" >"$PACKET_INTEGRITY_BUILD_LIFECYCLE_SEMANTIC_FIXTURE_LOG"
TRNM_RELEASE_REVIEW_PACKET_INTEGRITY_SOURCE_CHAIN_REFRESH=0 "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_tech_tree_semantic_fixture.sh" >"$PACKET_INTEGRITY_TECH_TREE_SEMANTIC_FIXTURE_LOG"
TRNM_RELEASE_REVIEW_PACKET_INTEGRITY_SOURCE_CHAIN_REFRESH=0 "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_projectile_ability_semantic_fixture.sh" >"$PACKET_INTEGRITY_PROJECTILE_ABILITY_SEMANTIC_FIXTURE_LOG"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_selection_minimap.sh" >"$SELECTION_MINIMAP_LOG"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_build_lifecycle.sh" >"$BUILD_LIFECYCLE_LOG"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_tech_tree.sh" >"$TECH_TREE_LOG"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_projectile_ability.sh" >"$PROJECTILE_ABILITY_LOG"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_combat_readability_pressure_readiness.sh" >"$COMBAT_READABILITY_PRESSURE_READINESS_LOG"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_camera_minimap_sync.sh" >"$CAMERA_MINIMAP_SYNC_LOG"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_decision_state_gap.sh" >"$BOT_DECISION_STATE_GAP_LOG"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_adaptive_build_order_gap.sh" >"$BOT_ADAPTIVE_BUILD_ORDER_GAP_LOG"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_tactical_micro_gap.sh" >"$BOT_TACTICAL_MICRO_GAP_LOG"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_bot_map_intel_gap.sh" >"$BOT_MAP_INTEL_GAP_LOG"
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_first_contact_basin_spec.sh" >"$FIRST_CONTACT_BASIN_SPEC_LOG"
TRNM_BEVY_HANDOFF_READINESS_REFRESH=0 "$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_handoff_packet.sh" >"$PLAYTEST_HANDOFF_PACKET_LOG"

artifact() {
  local id="$1"
  local label="$2"
  local path="$3"
  local role="$4"
  local file_status="missing"
  local sha256=""
  local bytes=""
  local contract_version=""
  local status=""

  if [[ -f "$path" ]]; then
    file_status="present"
    sha256="$(sha256sum "$path" | awk '{print $1}')"
    bytes="$(wc -c <"$path" | tr -d ' ')"
    if [[ "$path" == *.json ]]; then
      contract_version="$(jq -r '.contract_version // empty' "$path" 2>/dev/null || true)"
      status="$(jq -r '.status // .overall_status // empty' "$path" 2>/dev/null || true)"
    fi
  fi

  jq -nc \
    --arg id "$id" \
    --arg label "$label" \
    --arg path "$path" \
    --arg role "$role" \
    --arg file_status "$file_status" \
    --arg sha256 "$sha256" \
    --arg bytes "$bytes" \
    --arg contract_version "$contract_version" \
    --arg status "$status" \
    '{
      id: $id,
      label: $label,
      path: $path,
      role: $role,
      file_status: $file_status,
      sha256: (if $sha256 == "" then null else $sha256 end),
      bytes: (if $bytes == "" then null else ($bytes | tonumber) end),
      contract_version: (if $contract_version == "" then null else $contract_version end),
      status: (if $status == "" then null else $status end)
    }' >>"$ARTIFACTS_FILE"
}

artifact native_bevy_keyboard_replay "Native/Bevy keyboard replay" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-build-branch-title-route-all-branch-keyboard-replay.json" release_review_input
artifact native_bevy_action_coach "Native/Bevy action coach" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-action-coach.json" release_review_input
artifact native_bevy_player_hud_debug_layer "Native/Bevy player HUD/debug layer" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-player-hud-debug-layer.json" release_review_input
artifact native_bevy_player_ui_rescue "Native/Bevy player UI rescue" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-player-ui-rescue.json" release_review_input
artifact native_bevy_live_window_screenshot_sequence "Native/Bevy live-window screenshot sequence" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-live-window-screenshot-sequence.json" release_review_input
artifact native_bevy_live_window_mouse_hit_test_sequence "Native/Bevy live-window mouse hit-test sequence" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-live-window-mouse-hit-test-sequence.json" release_review_input
artifact native_bevy_sprite_texture_sampling "Native/Bevy sprite texture sampling" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-sprite-texture-sampling.json" release_review_input
artifact native_bevy_live_window_sampled_texture_correlation "Native/Bevy live-window sampled texture correlation" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-live-window-sampled-texture-correlation.json" release_review_input
artifact native_bevy_render_asset_eligibility "Native/Bevy render asset eligibility" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-render-asset-eligibility.json" release_review_input
artifact native_bevy_classic_asset_pack "Native/Bevy classic asset pack" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-asset-pack.json" release_review_input
artifact native_bevy_classic_manifest_lint "Native/Bevy classic manifest lint" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-manifest-lint.json" release_review_input
artifact native_bevy_classic_animation_preview "Native/Bevy classic animation preview" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-animation-preview.json" release_review_input
artifact native_bevy_classic_animation_preview_ppm "Native/Bevy classic animation preview PPM" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-animation-preview.ppm" release_review_visual_evidence
artifact native_bevy_classic_animation_selector "Native/Bevy classic animation selector" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-animation-selector.json" release_review_input
artifact native_bevy_classic_player_motion_probe "Native/Bevy classic player motion probe" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-player-motion-probe.json" release_review_input
artifact native_bevy_classic_player_motion_probe_ppm "Native/Bevy classic player motion probe PPM" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-player-motion-probe.ppm" release_review_visual_evidence
artifact native_bevy_classic_input_frame_budget "Native/Bevy classic input-frame budget" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-input-frame-budget.json" release_review_input
artifact native_bevy_classic_render_budget "Native/Bevy classic render budget" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-render-budget.json" release_review_input
artifact native_bevy_classic_scene_preview "Native/Bevy classic scene preview" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-scene-preview.json" release_review_input
artifact native_bevy_classic_scene_preview_ppm "Native/Bevy classic scene preview PPM" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-scene-preview.ppm" release_review_visual_evidence
artifact native_bevy_classic_model_catalog "Native/Bevy classic model catalog" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-model-catalog.json" release_review_input
artifact native_bevy_classic_model_catalog_ppm "Native/Bevy classic model catalog PPM" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-model-catalog.ppm" release_review_visual_evidence
artifact native_bevy_classic_renderer_probe "Native/Bevy classic renderer probe" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-renderer-probe.json" release_review_input
artifact native_bevy_classic_renderer_probe_ppm "Native/Bevy classic renderer probe PPM" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-renderer-probe.ppm" release_review_visual_evidence
artifact native_bevy_classic_isometric_modeling "Native/Bevy classic isometric modeling" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-isometric-modeling.json" release_review_input
artifact native_bevy_classic_isometric_modeling_ppm "Native/Bevy classic isometric modeling PPM" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-isometric-modeling.ppm" release_review_visual_evidence
artifact native_bevy_classic_rts_control_loop "Native/Bevy classic RTS control loop" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-loop.json" release_review_input
artifact native_bevy_classic_rts_control_loop_ppm "Native/Bevy classic RTS control loop PPM" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-control-loop.ppm" release_review_visual_evidence
artifact native_bevy_classic_rts_selection_minimap "Native/Bevy classic RTS selection/minimap" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-selection-minimap.json" release_review_input
artifact native_bevy_classic_rts_selection_minimap_ppm "Native/Bevy classic RTS selection/minimap PPM" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-selection-minimap.ppm" release_review_visual_evidence
artifact native_bevy_classic_rts_build_lifecycle "Native/Bevy classic RTS build lifecycle" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-build-lifecycle.json" release_review_input
artifact native_bevy_classic_rts_build_lifecycle_ppm "Native/Bevy classic RTS build lifecycle PPM" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-build-lifecycle.ppm" release_review_visual_evidence
artifact native_bevy_classic_rts_tech_tree "Native/Bevy classic RTS tech tree" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-tech-tree.json" release_review_input
artifact native_bevy_classic_rts_tech_tree_ppm "Native/Bevy classic RTS tech tree PPM" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-tech-tree.ppm" release_review_visual_evidence
artifact native_bevy_classic_rts_projectile_ability "Native/Bevy classic RTS projectile/ability" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-projectile-ability.json" release_review_input
artifact native_bevy_classic_rts_projectile_ability_ppm "Native/Bevy classic RTS projectile/ability PPM" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-projectile-ability.ppm" release_review_visual_evidence
artifact native_bevy_classic_rts_first_contact_basin_spec "Native/Bevy classic RTS First Contact Basin spec" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-first-contact-basin-spec.json" release_review_input
artifact native_bevy_first_minute_command_feedback_replay "Native/Bevy first-minute command feedback replay" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-first-minute-command-feedback-replay.json" release_review_input
artifact native_bevy_first_minute_command_feedback_source_recording "Native/Bevy first-minute command feedback source recording" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-first-minute-command-feedback-source-recording.json" release_review_recording
artifact native_bevy_first_minute_command_feedback_recording "Native/Bevy first-minute command feedback command recording" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-first-minute-command-feedback-recording.json" release_review_recording
artifact native_bevy_first_minute_command_feedback_replay_ppm "Native/Bevy first-minute command feedback replay PPM" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-first-minute-command-feedback-replay.ppm" release_review_visual_evidence
artifact native_bevy_first_minute_command_feedback_rejection_replay "Native/Bevy first-minute command feedback rejection replay" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-first-minute-command-feedback-rejection-replay.json" release_review_input
artifact native_bevy_first_minute_command_feedback_rejection_source_recording "Native/Bevy first-minute command feedback rejection source recording" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-first-minute-command-feedback-rejection-source-recording.json" release_review_recording
artifact native_bevy_first_minute_command_feedback_rejection_recording "Native/Bevy first-minute command feedback rejection recording" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-first-minute-command-feedback-rejection-recording.json" release_review_recording
artifact native_bevy_first_minute_command_feedback_rejection_replay_ppm "Native/Bevy first-minute command feedback rejection replay PPM" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-first-minute-command-feedback-rejection-replay.ppm" release_review_visual_evidence
artifact native_bevy_bot_planner_action_executor "Native/Bevy bot planner action executor" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-planner-action-executor.json" release_review_input
artifact native_bevy_bot_planner_action_executor_log "Native/Bevy bot planner action executor log" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-planner-action-executor/bot-planner-action-executor.actions.json" release_review_recording
artifact native_bevy_bot_planner_action_executor_ppm "Native/Bevy bot planner action executor PPM" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-planner-action-executor/bot-planner-action-executor.ppm" release_review_visual_evidence
artifact native_bevy_bot_planner_executor_replay_determinism "Native/Bevy bot planner executor replay determinism" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-planner-executor-replay-determinism.json" release_review_input
artifact native_bevy_bot_planner_executor_replay_determinism_log "Native/Bevy bot planner executor replay determinism log" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-planner-executor-replay-determinism/bot-planner-executor-replay-determinism.replay.json" release_review_recording
artifact native_bevy_bot_planner_executor_replay_determinism_ppm "Native/Bevy bot planner executor replay determinism PPM" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-planner-executor-replay-determinism/bot-planner-executor-replay-determinism.ppm" release_review_visual_evidence
artifact native_bevy_multi_match_bot_executor_evaluation "Native/Bevy multi-match bot executor evaluation" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-multi-match-bot-executor-evaluation.json" release_review_input
artifact native_bevy_multi_match_bot_executor_evaluation_log "Native/Bevy multi-match bot executor evaluation log" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-multi-match-bot-executor-evaluation/multi-match-bot-executor-evaluation.matches.json" release_review_recording
artifact native_bevy_multi_match_bot_executor_evaluation_ppm "Native/Bevy multi-match bot executor evaluation PPM" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-multi-match-bot-executor-evaluation/multi-match-bot-executor-evaluation.ppm" release_review_visual_evidence
artifact native_bevy_bot_executor_failure_recovery_matrix "Native/Bevy bot executor failure recovery matrix" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-executor-failure-recovery-matrix.json" release_review_input
artifact native_bevy_bot_executor_failure_recovery_matrix_log "Native/Bevy bot executor failure recovery matrix log" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-executor-failure-recovery-matrix/bot-executor-failure-recovery-matrix.matrix.json" release_review_recording
artifact native_bevy_bot_executor_failure_recovery_matrix_ppm "Native/Bevy bot executor failure recovery matrix PPM" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-executor-failure-recovery-matrix/bot-executor-failure-recovery-matrix.ppm" release_review_visual_evidence
artifact native_bevy_bot_decision_state_gap "Native/Bevy bot decision-state gap" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-decision-state-gap.json" release_review_input
artifact native_bevy_bot_decision_state_gap_ppm "Native/Bevy bot decision-state gap PPM" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-decision-state-gap.ppm" release_review_visual_evidence
artifact native_bevy_bot_adaptive_build_order_gap "Native/Bevy bot adaptive build-order gap" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-adaptive-build-order-gap.json" release_review_input
artifact native_bevy_bot_adaptive_build_order_gap_ppm "Native/Bevy bot adaptive build-order gap PPM" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-adaptive-build-order-gap.ppm" release_review_visual_evidence
artifact native_bevy_bot_tactical_micro_gap "Native/Bevy bot tactical micro gap" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-tactical-micro-gap.json" release_review_input
artifact native_bevy_bot_tactical_micro_gap_ppm "Native/Bevy bot tactical micro gap PPM" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-tactical-micro-gap.ppm" release_review_visual_evidence
artifact native_bevy_bot_map_intel_gap "Native/Bevy bot map intel gap" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-map-intel-gap.json" release_review_input
artifact native_bevy_bot_map_intel_gap_ppm "Native/Bevy bot map intel gap PPM" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-bot-map-intel-gap.ppm" release_review_visual_evidence
artifact native_bevy_classic_playtest_readiness "Native/Bevy classic playtest readiness" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-readiness.json" release_review_input
artifact native_bevy_classic_playtest_runner_status "Native/Bevy classic playtest runner status" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-runner-status.json" release_review_input
artifact native_bevy_classic_playtest_launcher "Native/Bevy classic playtest launcher" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-launcher.json" release_review_input
artifact native_bevy_classic_playtest_handoff_readiness "Native/Bevy classic playtest handoff readiness" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-handoff-readiness.json" release_review_input
artifact native_bevy_classic_playtest_handoff_packet "Native/Bevy classic playtest handoff packet" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-handoff-packet.json" release_review_input
artifact native_bevy_classic_playtest_handoff_packet_markdown "Native/Bevy classic playtest handoff packet Markdown" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-handoff-packet.md" release_review_input
artifact native_bevy_classic_rts_full_screen_ui_replication "Native/Bevy classic RTS full screen/UI replication" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-full-screen-ui-replication.json" release_review_input
artifact native_bevy_classic_rts_shell_meta_ui_replication "Native/Bevy classic RTS shell/meta UI replication" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-shell-meta-ui-replication.json" release_review_input
artifact native_bevy_classic_rts_match_setup_ui_replication "Native/Bevy classic RTS match setup UI replication" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-match-setup-ui-replication.json" release_review_input
artifact native_bevy_classic_rts_campaign_outcome_ui_readiness "Native/Bevy classic RTS campaign outcome UI readiness" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-campaign-outcome-ui-readiness.json" release_review_input
artifact native_bevy_classic_rts_campaign_ui_continuity "Native/Bevy classic RTS campaign UI continuity" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-campaign-ui-continuity.json" release_review_input
artifact native_bevy_classic_rts_campaign_ui_continuity_ppm "Native/Bevy classic RTS campaign UI continuity PPM" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-campaign-ui-continuity.ppm" release_review_visual_evidence
artifact native_bevy_classic_rts_in_match_hud_state_replication "Native/Bevy classic RTS in-match HUD/state replication" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-in-match-hud-state-replication.json" release_review_input
artifact native_bevy_classic_rts_session_state_continuity "Native/Bevy classic RTS session state continuity" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-session-state-continuity.json" release_review_input
artifact native_bevy_classic_rts_combat_readability_pressure_readiness "Native/Bevy classic RTS combat readability/pressure readiness" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-combat-readability-pressure-readiness.json" release_review_input
artifact native_bevy_classic_rts_camera_minimap_sync "Native/Bevy classic RTS camera/minimap sync" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-camera-minimap-sync.json" release_review_input
artifact native_bevy_classic_rts_camera_minimap_sync_ppm "Native/Bevy classic RTS camera/minimap sync PPM" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-camera-minimap-sync.ppm" release_review_visual_evidence
artifact native_bevy_classic_rts_full_game_visual_ui_replication "Native/Bevy classic RTS full-game visual/UI replication" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-full-game-visual-ui-replication.json" release_review_input
artifact native_bevy_classic_rts_full_game_visual_ui_replication_ppm "Native/Bevy classic RTS full-game visual/UI replication PPM" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-full-game-visual-ui-replication.ppm" release_review_visual_evidence
artifact native_bevy_classic_rts_openra_screen_for_screen_ui_replication "Native/Bevy classic RTS OpenRA screen-for-screen UI replication" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-screen-for-screen-ui-replication.json" release_review_input
artifact native_bevy_classic_rts_openra_screen_for_screen_ui_replication_ppm "Native/Bevy classic RTS OpenRA screen-for-screen UI replication PPM" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-screen-for-screen-ui-replication.ppm" release_review_visual_evidence
artifact native_bevy_classic_rts_openra_engine_port_asset_parity "Native/Bevy classic RTS OpenRA engine port asset parity" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-engine-port-asset-parity.json" release_review_input
artifact native_bevy_classic_rts_openra_engine_port_asset_parity_ppm "Native/Bevy classic RTS OpenRA engine port asset parity PPM" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-engine-port-asset-parity.ppm" release_review_visual_evidence
artifact native_bevy_classic_rts_production_desktop_review_packet "Native/Bevy classic RTS production desktop review packet" "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-desktop-review-packet.json" release_review_input
artifact cex_adapter_readiness "CEX production world adapter readiness" "$ROOT/acceptance/S3_repository_adapter/latest/cex-production-adapter-readiness.json" release_review_input
artifact s5_real_device_evidence "S5 real-device evidence validation" "$ROOT/acceptance/S5_native_bevy_device/latest/s5-real-device-evidence-validation.json" release_review_input
artifact public_launch_readiness "Public launch readiness" "$ACCEPTANCE_DIR/public-launch-readiness.json" release_review_input
artifact public_launch_evidence_intake "Public launch evidence intake" "$ACCEPTANCE_DIR/public-launch-evidence-intake.json" release_review_input
artifact public_launch_evidence_intake_markdown "Public launch evidence intake Markdown" "$ACCEPTANCE_DIR/public-launch-evidence-intake.md" release_review_input
artifact production_map_pack_public_evidence_collection "Production map-pack public evidence collection" "$ROOT/acceptance/S4_map_pack_gate/latest/production-map-pack-public-evidence-collection.json" release_review_collection
artifact map_modeling_gate "Map modeling gate" "$ROOT/acceptance/S4_map_pack_gate/latest/map-modeling-gate.json" release_review_input
artifact cohort_commercial_evidence_collection "Cohort/commercial evidence collection" "$ACCEPTANCE_DIR/cohort-commercial-evidence-collection.json" release_review_collection
artifact external_ops_evidence_collection "External ops evidence collection" "$ACCEPTANCE_DIR/external-ops-evidence-collection.json" release_review_collection
artifact public_launch_blocker_consistency "Public launch blocker consistency" "$ACCEPTANCE_DIR/public-launch-blocker-consistency.json" release_review_gate
artifact public_launch_evidence_kit "Public launch evidence kit" "$ACCEPTANCE_DIR/public-launch-evidence-kit.json" release_review_gate
artifact public_launch_evidence_kit_markdown "Public launch evidence kit Markdown" "$ACCEPTANCE_DIR/public-launch-evidence-kit.md" release_review_gate
artifact public_launch_operator_handoff "Public launch operator handoff" "$ACCEPTANCE_DIR/public-launch-operator-handoff.json" release_review_operator_handoff
artifact public_launch_operator_handoff_markdown "Public launch operator handoff Markdown" "$ACCEPTANCE_DIR/public-launch-operator-handoff.md" release_review_operator_handoff
artifact public_launch_template_negative_fixtures "Public launch template negative fixtures" "$ACCEPTANCE_DIR/public-launch-template-negative-fixtures.json" release_review_gate
artifact public_launch_evidence_bundle "Public launch evidence bundle" "$ACCEPTANCE_DIR/public-launch-evidence-bundle.json" release_review_gate
artifact public_launch_evidence_bundle_markdown "Public launch evidence bundle Markdown" "$ACCEPTANCE_DIR/public-launch-evidence-bundle.md" release_review_gate
artifact public_launch_bundle_negative_fixtures "Public launch bundle negative fixtures" "$ACCEPTANCE_DIR/public-launch-bundle-negative-fixtures.json" release_review_gate
artifact public_launch_status_only_fixture_guard "Public launch status-only fixture guard" "$ACCEPTANCE_DIR/public-launch-status-only-fixtures.json" release_review_gate
artifact production_map_pack_public_evidence "Production map-pack public evidence" "$ROOT/acceptance/S4_map_pack_gate/latest/production-map-pack-public-evidence.json" release_review_input
artifact cohort_commercial_evidence "Cohort/commercial evidence validation" "$ACCEPTANCE_DIR/cohort-commercial-evidence.json" release_review_input
artifact external_ops_evidence "External ops evidence validation" "$ACCEPTANCE_DIR/external-ops-evidence.json" release_review_input
artifact release_signoff_summary "Release signoff summary" "$ACCEPTANCE_DIR/release-signoff-summary.json" release_review_input
artifact release_review_quickcheck "Release review quickcheck" "$ACCEPTANCE_DIR/release-review-quickcheck.json" release_review_input
artifact release_review_status_json "Release review status JSON" "$STATUS_JSON" release_review_checklist
artifact release_review_status_markdown "Release review status Markdown" "$STATUS_MD" release_review_checklist
artifact release_review_convergence "Release review convergence" "$CONVERGENCE_JSON" release_review_gate
artifact release_review_checkpoint_manifest "Release review checkpoint manifest" "$ACCEPTANCE_DIR/release-review-checkpoint-manifest.json" release_review_checkpoint
artifact release_review_packet_integrity_semantic_fixture "Release review packet integrity semantic fixture" "$ACCEPTANCE_DIR/release-review-packet-integrity-semantic-fixture.json" release_review_gate
artifact release_review_packet_integrity_bot_executor_semantic_fixture "Release review packet integrity bot executor semantic fixture" "$ACCEPTANCE_DIR/release-review-packet-integrity-bot-executor-semantic-fixture.json" release_review_gate
artifact release_review_packet_integrity_bot_executor_matrix_semantic_fixture "Release review packet integrity bot executor failure/recovery matrix semantic fixture" "$ACCEPTANCE_DIR/release-review-packet-integrity-bot-executor-matrix-semantic-fixture.json" release_review_gate
artifact release_review_packet_integrity_bot_gap_semantic_fixture "Release review packet integrity bot gap semantic fixture" "$ACCEPTANCE_DIR/release-review-packet-integrity-bot-gap-semantic-fixture.json" release_review_gate
artifact release_review_packet_integrity_control_loop_semantic_fixture "Release review packet integrity control loop semantic fixture" "$ACCEPTANCE_DIR/release-review-packet-integrity-control-loop-semantic-fixture.json" release_review_gate
artifact release_review_packet_integrity_selection_minimap_semantic_fixture "Release review packet integrity selection/minimap semantic fixture" "$ACCEPTANCE_DIR/release-review-packet-integrity-selection-minimap-semantic-fixture.json" release_review_gate
artifact release_review_packet_integrity_build_lifecycle_semantic_fixture "Release review packet integrity build lifecycle semantic fixture" "$ACCEPTANCE_DIR/release-review-packet-integrity-build-lifecycle-semantic-fixture.json" release_review_gate
artifact release_review_packet_integrity_tech_tree_semantic_fixture "Release review packet integrity tech tree semantic fixture" "$ACCEPTANCE_DIR/release-review-packet-integrity-tech-tree-semantic-fixture.json" release_review_gate
artifact release_review_packet_integrity_projectile_ability_semantic_fixture "Release review packet integrity projectile/ability semantic fixture" "$ACCEPTANCE_DIR/release-review-packet-integrity-projectile-ability-semantic-fixture.json" release_review_gate
artifact release_review_packet_convergence_log "Release review packet convergence log" "$CONVERGENCE_LOG" release_review_log

ARTIFACTS_JSON="$(jq -s '.' "$ARTIFACTS_FILE")"
CONVERGENCE_GREEN="$(jq -r '.green // false' "$CONVERGENCE_JSON")"
READY_FOR_RELEASE_REVIEW="$(jq -r '.ready_for_release_review // false' "$STATUS_JSON")"
PUBLIC_LAUNCH_READY="$(jq -r '.public_launch_ready // false' "$STATUS_JSON")"
STATUS_READY="$(jq -r '.status // "unknown"' "$STATUS_JSON")"
CONVERGENCE_STATUS="$(jq -r '.status // "unknown"' "$CONVERGENCE_JSON")"
BLOCKED_ITEMS_JSON="$(jq -c '.blocked_items // []' "$STATUS_JSON")"
READY_ITEMS_JSON="$(jq -c '.ready_items // []' "$STATUS_JSON")"
MISSING_ARTIFACTS_JSON="$(jq -c '[.[] | select(.file_status != "present") | .id]' <<<"$ARTIFACTS_JSON")"
MISSING_ARTIFACT_COUNT="$(jq 'length' <<<"$MISSING_ARTIFACTS_JSON")"

PACKET_STATUS=release_review_packet_blocked
if [[ "$CONVERGENCE_GREEN" == "true" && "$READY_FOR_RELEASE_REVIEW" == "true" && "$MISSING_ARTIFACT_COUNT" == "0" ]]; then
  if [[ "$PUBLIC_LAUNCH_READY" == "true" ]]; then
    PACKET_STATUS=release_review_packet_green
  else
    PACKET_STATUS=release_review_packet_ready_with_public_launch_blockers
  fi
fi

jq -n \
  --arg contract_version "trillionnium_world_release_review_packet_v1" \
  --arg status "$PACKET_STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg markdown_path "$PACKET_MD" \
  --arg convergence_status "$CONVERGENCE_STATUS" \
  --arg status_checklist_status "$STATUS_READY" \
  --argjson artifacts "$ARTIFACTS_JSON" \
  --argjson ready_for_release_review "$READY_FOR_RELEASE_REVIEW" \
  --argjson public_launch_ready "$PUBLIC_LAUNCH_READY" \
  --argjson blocked_items "$BLOCKED_ITEMS_JSON" \
  --argjson ready_items "$READY_ITEMS_JSON" \
  --argjson missing_artifacts "$MISSING_ARTIFACTS_JSON" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_release_review_packet",
    markdown_path: $markdown_path,
    ready_for_release_review: $ready_for_release_review,
    public_launch_ready: $public_launch_ready,
    android_s5_real_device_claimed: false,
    proof_scope: "host_side_bevy_runtime_replay_not_android_real_device",
    convergence_status: $convergence_status,
    status_checklist_status: $status_checklist_status,
    packet_rule: "refresh_release_review_convergence_then_emit_a_checksummed_review_manifest_for_operator_and_reviewer_handoff",
    artifacts: $artifacts,
    missing_artifacts: $missing_artifacts,
    ready_items: $ready_items,
    blocked_items: $blocked_items,
    reviewer_next_action: (if $public_launch_ready then "review_public_launch_ready_evidence" else "collect_real_external_public_launch_evidence" end)
  }' >"$PACKET_JSON"

{
  printf '# Trillionnium World Release Review Packet\n\n'
  printf -- '- status: `%s`\n' "$PACKET_STATUS"
  printf -- '- ready_for_release_review: `%s`\n' "$READY_FOR_RELEASE_REVIEW"
  printf -- '- public_launch_ready: `%s`\n' "$PUBLIC_LAUNCH_READY"
  printf -- '- android_s5_real_device_claimed: `false`\n'
  printf -- '- proof_scope: `host_side_bevy_runtime_replay_not_android_real_device`\n\n'
  printf '## Evidence Artifacts\n\n'
  jq -r '.artifacts[] | "- `\(.id)`: \(.path)\n  - role: `\(.role)`\n  - file_status: `\(.file_status)`\n  - contract_version: `\(.contract_version // "n/a")`\n  - status: `\(.status // "n/a")`\n  - sha256: `\(.sha256 // "missing")`\n  - bytes: `\(.bytes // 0)`"' "$PACKET_JSON"
  printf '\n## Green For Review\n\n'
  jq -r '.ready_items[] | "- [\(.ready | if . then "x" else " " end)] \(.label): \(.detail)"' "$PACKET_JSON"
  printf '\n## Still Requires Real External Evidence\n\n'
  jq -r 'if (.blocked_items | length) == 0 then "- [x] No public-launch blockers remain." else .blocked_items[] | "- [ ] \(.label): \(.needed)" end' "$PACKET_JSON"
  printf '\n## Boundary\n\n'
  printf -- '- Native/Bevy keyboard replay, classic animation preview/selector, classic player motion, action coach, HUD/debug layer, player UI rescue, live screenshots, sprite texture sampling, sampled texture live-window correlation, and render asset eligibility are host-side proof, not Android real-device proof.\n'
  printf -- '- CEX adapter readiness proves the current CEX incubator exports the Trillionnium world runtime adapter contract; it is not a substitute for real external public-launch evidence.\n'
  printf -- '- Public launch operator handoff is a checksum-bound collection checklist; it does not grant public-launch credit without real external evidence.\n'
  printf -- '- The checkpoint manifest groups the current dirty working tree for review; it does not stage, commit, or publish anything.\n'
  printf -- '- Public launch remains blocked until the external evidence above is attached.\n'
} >"$PACKET_MD"

case "$PACKET_STATUS" in
  release_review_packet_green)
    printf 'TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_GREEN %s %s\n' "$PACKET_JSON" "$PACKET_MD"
    ;;
  release_review_packet_ready_with_public_launch_blockers)
    printf 'TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_READY_WITH_PUBLIC_LAUNCH_BLOCKERS %s %s\n' "$PACKET_JSON" "$PACKET_MD"
    ;;
  *)
    printf 'TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_BLOCKED %s %s\n' "$PACKET_STATUS" "$PACKET_JSON" >&2
    exit 1
    ;;
esac
