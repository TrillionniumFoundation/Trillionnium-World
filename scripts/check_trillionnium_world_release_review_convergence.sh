#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
SUMMARY_FILE="$ACCEPTANCE_DIR/release-review-convergence.json"
if [[ -v TRILLIONNIUM_WORLD_RELEASE_REVIEW_CONVERGENCE_SUMMARY && -n "$TRILLIONNIUM_WORLD_RELEASE_REVIEW_CONVERGENCE_SUMMARY" ]]; then
  SUMMARY_FILE="$TRILLIONNIUM_WORLD_RELEASE_REVIEW_CONVERGENCE_SUMMARY"
fi

STATUS_JSON="$ACCEPTANCE_DIR/release-review-status.json"
STATUS_MD="$ACCEPTANCE_DIR/release-review-status.md"
STATUS_LOG="$ACCEPTANCE_DIR/release-review-convergence-status.log"
CEX_ADAPTER_LOG="$ACCEPTANCE_DIR/release-review-convergence-cex-adapter-readiness.log"
CHECK_RESULTS="$(mktemp)"
trap 'rm -f "$CHECK_RESULTS"' EXIT

mkdir -p "$ACCEPTANCE_DIR"

add_check() {
  local name="$1"
  local status="$2"
  local path="$3"
  local detail="$4"
  jq -nc \
    --arg name "$name" \
    --arg status "$status" \
    --arg path "$path" \
    --arg detail "$detail" \
    '{name: $name, status: $status, path: $path, detail: $detail}' >>"$CHECK_RESULTS"
}

require_executable() {
  local name="$1"
  local path="$2"
  if [[ -x "$path" ]]; then
    add_check "$name" ok "$path" executable
  elif [[ -f "$path" ]]; then
    add_check "$name" fail "$path" not_executable
  else
    add_check "$name" fail "$path" missing
  fi
}

require_text() {
  local name="$1"
  local path="$2"
  local needle="$3"
  if [[ ! -f "$path" ]]; then
    add_check "$name" fail "$path" missing
  elif grep -Fq -- "$needle" "$path"; then
    add_check "$name" ok "$path" "contains: $needle"
  else
    add_check "$name" fail "$path" "missing: $needle"
  fi
}

require_json() {
  local name="$1"
  local path="$2"
  local expr="$3"
  local detail="$4"
  if [[ ! -f "$path" ]]; then
    add_check "$name" fail "$path" missing
  elif jq -e "$expr" "$path" >/dev/null; then
    add_check "$name" ok "$path" "$detail"
  else
    add_check "$name" fail "$path" "$detail"
  fi
}

read_json_bool() {
  local path="$1"
  local expr="$2"
  if [[ -f "$path" ]]; then
    jq -r "$expr // false" "$path" 2>/dev/null || printf 'false'
  else
    printf 'false'
  fi
}

if "$ROOT/scripts/check_trillionnium_world_cex_adapter_readiness.sh" >"$CEX_ADAPTER_LOG" 2>&1; then
  add_check cex_adapter_readiness_refresh ok "$CEX_ADAPTER_LOG" refreshed
else
  add_check cex_adapter_readiness_refresh fail "$CEX_ADAPTER_LOG" failed
fi

if "$ROOT/scripts/check_trillionnium_world_release_review_status.sh" >"$STATUS_LOG" 2>&1; then
  add_check release_review_status_refresh ok "$STATUS_LOG" refreshed
else
  add_check release_review_status_refresh fail "$STATUS_LOG" failed
fi

require_executable quickcheck_script "$ROOT/scripts/check_trillionnium_world_release_review_quickcheck.sh"
require_executable status_script "$ROOT/scripts/check_trillionnium_world_release_review_status.sh"
require_executable convergence_script "$ROOT/scripts/check_trillionnium_world_release_review_convergence.sh"
require_executable cex_adapter_readiness_script "$ROOT/scripts/check_trillionnium_world_cex_adapter_readiness.sh"
require_executable readme_guard "$ROOT/scripts/v2/root_readme_world_release_review_quickcheck_guard_test.sh"
require_executable status_guard "$ROOT/scripts/v2/release_review_status_script_contract_guard_test.sh"

doc_paths=(
  "$ROOT/README.md"
  "$ROOT/docs/development/trillionnium-world-unified-development-doc-v1.md"
  "$ROOT/docs/development/trillionnium-world-cex-full-split-plan-v1.md"
  "$ROOT/docs/development/trillionnium-world-dev-environment-v1.md"
)

for doc in "${doc_paths[@]}"; do
  label="$(basename "$doc" | tr '.-' '__')"
  require_text "doc_${label}_quickcheck" "$doc" "check_trillionnium_world_release_review_quickcheck.sh"
  require_text "doc_${label}_status" "$doc" "check_trillionnium_world_release_review_status.sh"
  require_text "doc_${label}_convergence" "$doc" "check_trillionnium_world_release_review_convergence.sh"
done

require_text workflow_readme_guard "$ROOT/.github/workflows/trnm-gate-quick-check.yml" "root_readme_world_release_review_quickcheck_guard_test.sh"
require_text workflow_status_guard "$ROOT/.github/workflows/trnm-gate-quick-check.yml" "release_review_status_script_contract_guard_test.sh"

REPLAY_JSON="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-build-branch-title-route-all-branch-keyboard-replay.json"
ACTION_COACH_JSON="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-action-coach.json"
PLAYER_HUD_JSON="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-player-hud-debug-layer.json"
LIVE_SCREENSHOT_JSON="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-live-window-screenshot-sequence.json"
SPRITE_TEXTURE_SAMPLING_JSON="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-sprite-texture-sampling.json"
SAMPLED_TEXTURE_CORRELATION_JSON="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-live-window-sampled-texture-correlation.json"
RENDER_ASSET_ELIGIBILITY_JSON="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-render-asset-eligibility.json"
CEX_ADAPTER_JSON="$ROOT/acceptance/S3_repository_adapter/latest/cex-production-adapter-readiness.json"
PUBLIC_LAUNCH_JSON="$ACCEPTANCE_DIR/public-launch-readiness.json"
SIGNOFF_JSON="$ACCEPTANCE_DIR/release-signoff-summary.json"
QUICKCHECK_JSON="$ACCEPTANCE_DIR/release-review-quickcheck.json"

require_json native_bevy_keyboard_replay "$REPLAY_JSON" '.contract_version == "trillionnium_world_bevy_build_branch_title_route_all_branch_keyboard_replay_v1" and .green == true and .branch_count == 3 and .all_branch_replay_gate == true and .replay_results.force.recorded_sequence_count == 10 and .replay_results.agility.recorded_sequence_count == 8 and .replay_results.craft.recorded_sequence_count == 7 and .replay_results.force.replay_final_runtime.combat_result_state == "victory"' "contract green, 3 branches, keyboard replay counts, force combat victory"
require_json native_bevy_action_coach "$ACTION_COACH_JSON" '.contract_version == "trillionnium_world_bevy_action_coach_v1" and .green == true and .coach_stage_gate == true and .enter_execution_gate == true and .final_next_gate == true and .android_s5_real_device_claimed == false' "action coach contract green with Android S5 no-claim boundary"
require_json native_bevy_player_hud_debug_layer "$PLAYER_HUD_JSON" '.contract_version == "trillionnium_world_bevy_player_hud_debug_layer_v1" and .green == true and .player_hud_gate == true and .debug_layer_gate == true and .android_s5_real_device_claimed == false' "player HUD/debug layer contract green with Android S5 no-claim boundary"
require_json native_bevy_live_window_screenshot_sequence "$LIVE_SCREENSHOT_JSON" '.contract_version == "trillionnium_world_bevy_live_window_screenshot_sequence_v1" and .green == true and .frame_sequence_gate == true and .contact_sheet_gate == true and .android_s5_real_device_claimed == false' "live-window screenshot sequence contract green with Android S5 no-claim boundary"
require_json native_bevy_sprite_texture_sampling "$SPRITE_TEXTURE_SAMPLING_JSON" '.contract_version == "trillionnium_world_bevy_sprite_texture_sampling_v1" and .green == true and .four_layer_texture_sampling_gate == true and .texture_sample_nonblank_gate == true and .sampled_surface_count >= 24 and .texture_unique_rgba_color_count >= 4 and .gpu_upload_claimed == false and .android_s5_real_device_claimed == false' "sprite texture sampling contract green with host-side CPU sampling boundary"
require_json native_bevy_live_window_sampled_texture_correlation "$SAMPLED_TEXTURE_CORRELATION_JSON" '.contract_version == "trillionnium_world_bevy_live_window_sampled_texture_correlation_v1" and .green == true and .gates.four_layer_sampled_live_correlation_gate == true and .sampled_surface_count >= 24 and .live_frame_count >= 3 and .live_final_frame_colors_96x54 > 0 and .gpu_upload_claimed == false and .android_s5_real_device_claimed == false' "sampled texture live-window correlation contract green with Android S5 no-claim boundary"
require_json native_bevy_render_asset_eligibility "$RENDER_ASSET_ELIGIBILITY_JSON" '.contract_version == "trillionnium_world_bevy_render_asset_eligibility_v1" and .green == true and .render_asset_usage_gate == true and .image_descriptor_render_eligibility_gate == true and .atlas_layout_render_eligibility_gate == true and .sprite_render_reference_gate == true and .image_asset_usage_main_world == true and .image_asset_usage_render_world == true and .render_world_extraction_completed_claimed == false and .gpu_upload_claimed == false and .android_s5_real_device_claimed == false' "render asset eligibility contract green without claiming extraction/GPU/Android"
require_json cex_adapter_readiness "$CEX_ADAPTER_JSON" '.contract_version == "trillionnium_world_cex_adapter_readiness_gate_v1" and .green == true and .status == "cex_adapter_readiness_green" and .observed.contract_version == "cex_trillionnium_world_production_adapter_v1" and .observed.protocol_contract == "trillionnium_world_runtime_adapter_v1" and .observed.domain_contract == "trillionnium_world_domain_v1" and .observed.route_record_total > 0 and .observed.world_node_count > 0' "CEX production adapter readiness evidence green without importing CEX internals"
require_json public_launch_consumes_replay "$PUBLIC_LAUNCH_JSON" '.gates.native_bevy_keyboard_replay.green == true and .gates.native_bevy_keyboard_replay.contract_version == "trillionnium_world_bevy_build_branch_title_route_all_branch_keyboard_replay_v1" and .gates.native_bevy_keyboard_replay.proof_scope == "host_side_bevy_runtime_replay_not_android_real_device" and ((.blockers // []) | index("native_bevy_keyboard_replay_contract") | not)' "native replay gate consumed; replay contract is not a blocker"
require_json public_launch_consumes_local_playability "$PUBLIC_LAUNCH_JSON" '.gates.native_bevy_action_coach.green == true and .gates.native_bevy_player_hud_debug_layer.green == true and .gates.native_bevy_live_window_screenshot_sequence.green == true and .gates.native_bevy_sprite_texture_sampling.green == true and .gates.native_bevy_live_window_sampled_texture_correlation.green == true and .gates.native_bevy_render_asset_eligibility.green == true and ((.blockers // []) | index("native_bevy_action_coach_contract") | not) and ((.blockers // []) | index("native_bevy_player_hud_debug_layer_contract") | not) and ((.blockers // []) | index("native_bevy_live_window_screenshot_sequence_contract") | not) and ((.blockers // []) | index("native_bevy_sprite_texture_sampling_contract") | not) and ((.blockers // []) | index("native_bevy_live_window_sampled_texture_correlation_contract") | not) and ((.blockers // []) | index("native_bevy_render_asset_eligibility_contract") | not)' "action coach, player HUD, live screenshot, texture sampling, sampled correlation, and render eligibility gates consumed; local playability contracts are not blockers"
require_json release_signoff_summary "$SIGNOFF_JSON" '.contract_version == "trillionnium_world_release_signoff_summary_v1" and .status == "release_signoff_summary_ready_with_public_launch_blockers" and .android_s5_real_device_claimed == false and .gates.native_bevy_keyboard_replay.ready_for_release_review == true and .gates.native_bevy_action_coach.ready_for_release_review == true and .gates.native_bevy_player_hud_debug_layer.ready_for_release_review == true and .gates.native_bevy_live_window_screenshot_sequence.ready_for_release_review == true and .gates.native_bevy_sprite_texture_sampling.ready_for_release_review == true and .gates.native_bevy_live_window_sampled_texture_correlation.ready_for_release_review == true and .gates.native_bevy_render_asset_eligibility.ready_for_release_review == true and .gates.cex_adapter_readiness.ready_for_release_review == true and .gates.public_launch_consumes_replay.ready == true and .gates.public_launch_consumes_local_playability.ready == true and .gates.s5_real_device_matrix.ready == false' "signoff summary keeps local Bevy playability, texture sampling, render eligibility, CEX adapter readiness ready; public-launch consumed; Android S5 unclaimed"
require_json release_review_quickcheck "$QUICKCHECK_JSON" '.contract_version == "trillionnium_world_release_review_quickcheck_v1" and .status == "release_review_quickcheck_green_with_public_launch_blockers" and .ready_for_release_review == true and .public_launch_ready == false and .android_s5_real_device_claimed == false and .gates.native_bevy_action_coach_ready == true and .gates.native_bevy_player_hud_debug_layer_ready == true and .gates.native_bevy_live_window_screenshot_sequence_ready == true and .gates.native_bevy_sprite_texture_sampling_ready == true and .gates.native_bevy_live_window_sampled_texture_correlation_ready == true and .gates.native_bevy_render_asset_eligibility_ready == true and .gates.cex_adapter_readiness_ready == true and .gates.public_launch_consumes_local_playability == true' "quickcheck green for review with public-launch blockers, CEX adapter readiness, and local Bevy texture/render playability gates"
require_json release_review_status_json "$STATUS_JSON" '.contract_version == "trillionnium_world_release_review_status_v1" and .status == "release_review_ready_public_launch_blocked" and .ready_for_release_review == true and .public_launch_ready == false and .android_s5_real_device_claimed == false and (.ready_items | length) >= 9 and (.ready_items | map(.id) | index("cex_adapter_readiness")) and (.blocked_items | length) == 6' "status checklist has expanded green review items including CEX adapter readiness and six external blockers"
require_text release_review_status_markdown_green "$STATUS_MD" "Green For Review"
require_text release_review_status_markdown_blockers "$STATUS_MD" "Still Requires Real External Evidence"
require_text release_review_status_markdown_boundary "$STATUS_MD" "Native/Bevy keyboard replay, action coach, HUD/debug layer, player UI rescue, live screenshots, sprite texture sampling, sampled texture live-window correlation, and render asset eligibility are host-side proof, not Android real-device proof."
require_text release_review_status_markdown_cex_boundary "$STATUS_MD" "CEX adapter readiness is incubator runtime adapter evidence, not real external public-launch evidence."

CHECKS_JSON="$(jq -s '.' "$CHECK_RESULTS")"
FAILURES_JSON="$(jq -s '[.[] | select(.status != "ok")]' "$CHECK_RESULTS")"
FAILURE_COUNT="$(jq 'length' <<<"$FAILURES_JSON")"
READY_FOR_RELEASE_REVIEW="$(read_json_bool "$STATUS_JSON" '.ready_for_release_review')"
PUBLIC_LAUNCH_READY="$(read_json_bool "$STATUS_JSON" '.public_launch_ready')"

GREEN=false
STATUS=release_review_convergence_blocked
if [[ "$FAILURE_COUNT" == "0" ]]; then
  GREEN=true
  if [[ "$PUBLIC_LAUNCH_READY" == "true" ]]; then
    STATUS=release_review_convergence_green
  else
    STATUS=release_review_convergence_green_with_public_launch_blockers
  fi
fi

jq -n \
  --arg contract_version "trillionnium_world_release_review_convergence_v1" \
  --arg status "$STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg status_json "$STATUS_JSON" \
  --arg status_markdown "$STATUS_MD" \
  --arg status_log "$STATUS_LOG" \
  --argjson green "$GREEN" \
  --argjson ready_for_release_review "$READY_FOR_RELEASE_REVIEW" \
  --argjson public_launch_ready "$PUBLIC_LAUNCH_READY" \
  --argjson checks "$CHECKS_JSON" \
  --argjson failures "$FAILURES_JSON" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_release_review_convergence",
    green: $green,
    ready_for_release_review: $ready_for_release_review,
    public_launch_ready: $public_launch_ready,
    android_s5_real_device_claimed: false,
    proof_scope: "host_side_bevy_runtime_replay_not_android_real_device",
    refreshed_status: {
      json_path: $status_json,
      markdown_path: $status_markdown,
      log_path: $status_log
    },
    convergence_rule: "release_review_status_must_refresh_and_scripts_docs_workflow_guards_evidence_outputs_must_remain_connected",
    checks: $checks,
    failures: $failures,
    reviewer_next_action: (if $green and $public_launch_ready then "review_public_launch_ready_evidence" elif $green then "collect_real_external_public_launch_evidence" else "repair_release_review_entrypoint_or_evidence_chain" end)
  }' >"$SUMMARY_FILE"

case "$STATUS" in
  release_review_convergence_green)
    printf 'TRILLIONNIUM_WORLD_RELEASE_REVIEW_CONVERGENCE_GREEN %s\n' "$SUMMARY_FILE"
    ;;
  release_review_convergence_green_with_public_launch_blockers)
    printf 'TRILLIONNIUM_WORLD_RELEASE_REVIEW_CONVERGENCE_GREEN_WITH_PUBLIC_LAUNCH_BLOCKERS %s\n' "$SUMMARY_FILE"
    ;;
  *)
    printf 'TRILLIONNIUM_WORLD_RELEASE_REVIEW_CONVERGENCE_BLOCKED %s %s\n' "$STATUS" "$SUMMARY_FILE" >&2
    exit 1
    ;;
esac
