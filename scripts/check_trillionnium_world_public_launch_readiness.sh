#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
SUMMARY_FILE="${TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_SUMMARY:-$ACCEPTANCE_DIR/public-launch-readiness.json}"
REQUIRE_READY=0

for arg in "$@"; do
  case "$arg" in
    --require-ready)
      REQUIRE_READY=1
      ;;
    *)
      printf 'unknown option: %s\n' "$arg" >&2
      exit 2
      ;;
  esac
done

# shellcheck source=scripts/release_review_acceptance_lock.sh
source "$ROOT/scripts/release_review_acceptance_lock.sh"
trnm_acquire_release_review_acceptance_lock "$ACCEPTANCE_DIR"

TRILLIONNIUM_PRODUCTION_MAP_PACK_PUBLIC_SUMMARY="$ROOT/acceptance/S4_map_pack_gate/latest/production-map-pack-public-evidence.json" \
  "$ROOT/scripts/check_trillionnium_world_production_map_pack_public_evidence.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_cohort_commercial_evidence.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_external_ops_evidence.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_s5_real_device_evidence.sh" >/dev/null

file_status() {
  local path="$1"
  if [[ -n "$path" && -f "$path" ]]; then
    printf 'present'
  else
    printf 'missing'
  fi
}

read_json_field() {
  local path="$1"
  local expr="$2"
  if [[ -f "$path" ]]; then
    jq -r "$expr // empty" "$path" 2>/dev/null || true
  fi
}

DEV_REPOSITORY_EVIDENCE="$ROOT/acceptance/S0_world_dev_environment/latest/dev-runtime-repository-smoke.json"
BROWSER_PARITY_EVIDENCE="$ROOT/acceptance/S3_browser_parity/latest/browser-parity.json"
REPOSITORY_ADAPTER_EVIDENCE="$ROOT/acceptance/S3_repository_adapter/latest/repository-adapter-boundary.json"
ROLLBACK_BACKUP_EVIDENCE="$ROOT/acceptance/S6_public_launch/latest/release-rollback-backup-drill.json"
COHORT_COMMERCIAL_SCHEMA_EVIDENCE="$ROOT/acceptance/S6_public_launch/latest/cohort-commercial-evidence-schema.json"
COHORT_COMMERCIAL_EVIDENCE="$ROOT/acceptance/S6_public_launch/latest/cohort-commercial-evidence.json"
EXTERNAL_OPS_EVIDENCE="$ROOT/acceptance/S6_public_launch/latest/external-ops-evidence.json"
S5_EVIDENCE="$ROOT/acceptance/S5_native_bevy_device/latest/s5-device-evidence.json"
S5_REAL_DEVICE_VALIDATION="$ROOT/acceptance/S5_native_bevy_device/latest/s5-real-device-evidence-validation.json"
NATIVE_BEVY_KEYBOARD_REPLAY_EVIDENCE="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-build-branch-title-route-all-branch-keyboard-replay.json"
NATIVE_BEVY_ACTION_COACH_EVIDENCE="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-action-coach.json"
NATIVE_BEVY_PLAYER_HUD_EVIDENCE="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-player-hud-debug-layer.json"
NATIVE_BEVY_LIVE_SCREENSHOT_EVIDENCE="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-live-window-screenshot-sequence.json"
NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_EVIDENCE="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-sprite-texture-sampling.json"
NATIVE_BEVY_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION_EVIDENCE="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-live-window-sampled-texture-correlation.json"
NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_EVIDENCE="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-render-asset-eligibility.json"
MAP_PACK_MANIFEST="$ROOT/acceptance/S4_map_pack_gate/latest/map_pack_manifest_signed.json"
MAP_PACK_SUMMARY="$ROOT/acceptance/S4_map_pack_gate/latest/map-pack-gate-summary.json"
PRODUCTION_MAP_PACK_EVIDENCE="$ROOT/acceptance/S4_map_pack_gate/latest/production-map-pack-public-evidence.json"
FIRST_BETA_EVIDENCE="$(read_json_field "$COHORT_COMMERCIAL_EVIDENCE" '.first_beta.operator_evidence.path')"
COMMERCIAL_DRILL_EVIDENCE="$(read_json_field "$COHORT_COMMERCIAL_EVIDENCE" '.commercial_launch_drill.operator_evidence.path')"
MULTI_NODE_EVIDENCE="$(read_json_field "$EXTERNAL_OPS_EVIDENCE" '.multi_node_or_live_traffic_latency.operator_evidence.path')"
PUBLIC_DEPLOY_EVIDENCE="$(read_json_field "$EXTERNAL_OPS_EVIDENCE" '.public_network_deploy.operator_evidence.path')"

DEV_REPOSITORY_STATUS="$(read_json_field "$DEV_REPOSITORY_EVIDENCE" '.status')"
BROWSER_PARITY_STATUS="$(read_json_field "$BROWSER_PARITY_EVIDENCE" '.status')"
BROWSER_PARITY_FILE_STATUS="$(file_status "$BROWSER_PARITY_EVIDENCE")"
REPOSITORY_ADAPTER_STATUS="$(read_json_field "$REPOSITORY_ADAPTER_EVIDENCE" '.status')"
REPOSITORY_ADAPTER_FILE_STATUS="$(file_status "$REPOSITORY_ADAPTER_EVIDENCE")"
ROLLBACK_BACKUP_STATUS="$(read_json_field "$ROLLBACK_BACKUP_EVIDENCE" '.status')"
ROLLBACK_BACKUP_FILE_STATUS="$(file_status "$ROLLBACK_BACKUP_EVIDENCE")"
COHORT_COMMERCIAL_SCHEMA_STATUS="$(read_json_field "$COHORT_COMMERCIAL_SCHEMA_EVIDENCE" '.status')"
COHORT_COMMERCIAL_SCHEMA_FILE_STATUS="$(file_status "$COHORT_COMMERCIAL_SCHEMA_EVIDENCE")"
COHORT_COMMERCIAL_STATUS="$(read_json_field "$COHORT_COMMERCIAL_EVIDENCE" '.status')"
COHORT_COMMERCIAL_FILE_STATUS="$(file_status "$COHORT_COMMERCIAL_EVIDENCE")"
EXTERNAL_OPS_STATUS="$(read_json_field "$EXTERNAL_OPS_EVIDENCE" '.status')"
EXTERNAL_OPS_FILE_STATUS="$(file_status "$EXTERNAL_OPS_EVIDENCE")"
S5_STATUS="$(read_json_field "$S5_REAL_DEVICE_VALIDATION" '.status')"
S5_FILE_STATUS="$(file_status "$S5_EVIDENCE")"
S5_VALIDATION_FILE_STATUS="$(file_status "$S5_REAL_DEVICE_VALIDATION")"
NATIVE_BEVY_KEYBOARD_REPLAY_CONTRACT="$(read_json_field "$NATIVE_BEVY_KEYBOARD_REPLAY_EVIDENCE" '.contract_version')"
NATIVE_BEVY_KEYBOARD_REPLAY_GREEN="$(read_json_field "$NATIVE_BEVY_KEYBOARD_REPLAY_EVIDENCE" '.green')"
NATIVE_BEVY_KEYBOARD_REPLAY_FILE_STATUS="$(file_status "$NATIVE_BEVY_KEYBOARD_REPLAY_EVIDENCE")"
NATIVE_BEVY_ACTION_COACH_CONTRACT="$(read_json_field "$NATIVE_BEVY_ACTION_COACH_EVIDENCE" '.contract_version')"
NATIVE_BEVY_ACTION_COACH_GREEN="$(read_json_field "$NATIVE_BEVY_ACTION_COACH_EVIDENCE" '.green')"
NATIVE_BEVY_ACTION_COACH_FILE_STATUS="$(file_status "$NATIVE_BEVY_ACTION_COACH_EVIDENCE")"
NATIVE_BEVY_PLAYER_HUD_CONTRACT="$(read_json_field "$NATIVE_BEVY_PLAYER_HUD_EVIDENCE" '.contract_version')"
NATIVE_BEVY_PLAYER_HUD_GREEN="$(read_json_field "$NATIVE_BEVY_PLAYER_HUD_EVIDENCE" '.green')"
NATIVE_BEVY_PLAYER_HUD_FILE_STATUS="$(file_status "$NATIVE_BEVY_PLAYER_HUD_EVIDENCE")"
NATIVE_BEVY_LIVE_SCREENSHOT_CONTRACT="$(read_json_field "$NATIVE_BEVY_LIVE_SCREENSHOT_EVIDENCE" '.contract_version')"
NATIVE_BEVY_LIVE_SCREENSHOT_GREEN="$(read_json_field "$NATIVE_BEVY_LIVE_SCREENSHOT_EVIDENCE" '.green')"
NATIVE_BEVY_LIVE_SCREENSHOT_FRAME_GATE="$(read_json_field "$NATIVE_BEVY_LIVE_SCREENSHOT_EVIDENCE" '.frame_sequence_gate')"
NATIVE_BEVY_LIVE_SCREENSHOT_CONTACT_SHEET_GATE="$(read_json_field "$NATIVE_BEVY_LIVE_SCREENSHOT_EVIDENCE" '.contact_sheet_gate')"
NATIVE_BEVY_LIVE_SCREENSHOT_FILE_STATUS="$(file_status "$NATIVE_BEVY_LIVE_SCREENSHOT_EVIDENCE")"
NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_CONTRACT="$(read_json_field "$NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_EVIDENCE" '.contract_version')"
NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_GREEN="$(read_json_field "$NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_EVIDENCE" '.green')"
NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_FOUR_LAYER_GATE="$(read_json_field "$NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_EVIDENCE" '.four_layer_texture_sampling_gate')"
NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_NONBLANK_GATE="$(read_json_field "$NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_EVIDENCE" '.texture_sample_nonblank_gate')"
NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_FILE_STATUS="$(file_status "$NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_EVIDENCE")"
NATIVE_BEVY_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION_CONTRACT="$(read_json_field "$NATIVE_BEVY_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION_EVIDENCE" '.contract_version')"
NATIVE_BEVY_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION_GREEN="$(read_json_field "$NATIVE_BEVY_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION_EVIDENCE" '.green')"
NATIVE_BEVY_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION_GATE="$(read_json_field "$NATIVE_BEVY_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION_EVIDENCE" '.gates.four_layer_sampled_live_correlation_gate')"
NATIVE_BEVY_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION_FILE_STATUS="$(file_status "$NATIVE_BEVY_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION_EVIDENCE")"
NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_CONTRACT="$(read_json_field "$NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_EVIDENCE" '.contract_version')"
NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_GREEN="$(read_json_field "$NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_EVIDENCE" '.green')"
NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_USAGE_GATE="$(read_json_field "$NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_EVIDENCE" '.render_asset_usage_gate')"
NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_REFERENCE_GATE="$(read_json_field "$NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_EVIDENCE" '.sprite_render_reference_gate')"
NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_FILE_STATUS="$(file_status "$NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_EVIDENCE")"
MAP_PACK_STATUS="$(read_json_field "$MAP_PACK_SUMMARY" '.status')"
MAP_PACK_MANIFEST_STATUS="$(file_status "$MAP_PACK_MANIFEST")"
PRODUCTION_MAP_PACK_STATUS="$(read_json_field "$PRODUCTION_MAP_PACK_EVIDENCE" '.status')"
PRODUCTION_MAP_PACK_FILE_STATUS="$(file_status "$PRODUCTION_MAP_PACK_EVIDENCE")"
FIRST_BETA_STATUS="$(read_json_field "$COHORT_COMMERCIAL_EVIDENCE" '.first_beta.status')"
FIRST_BETA_FILE_STATUS="$(read_json_field "$COHORT_COMMERCIAL_EVIDENCE" '.first_beta.operator_evidence.file_status')"
COMMERCIAL_DRILL_STATUS="$(read_json_field "$COHORT_COMMERCIAL_EVIDENCE" '.commercial_launch_drill.status')"
COMMERCIAL_DRILL_FILE_STATUS="$(read_json_field "$COHORT_COMMERCIAL_EVIDENCE" '.commercial_launch_drill.operator_evidence.file_status')"
MULTI_NODE_STATUS="$(read_json_field "$EXTERNAL_OPS_EVIDENCE" '.multi_node_or_live_traffic_latency.status')"
MULTI_NODE_FILE_STATUS="$(read_json_field "$EXTERNAL_OPS_EVIDENCE" '.multi_node_or_live_traffic_latency.operator_evidence.file_status')"
MULTI_NODE_LOCAL_STATUS="$(read_json_field "$EXTERNAL_OPS_EVIDENCE" '.multi_node_or_live_traffic_latency.local_drill.status')"
PUBLIC_DEPLOY_STATUS="$(read_json_field "$EXTERNAL_OPS_EVIDENCE" '.public_network_deploy.status')"
PUBLIC_DEPLOY_FILE_STATUS="$(read_json_field "$EXTERNAL_OPS_EVIDENCE" '.public_network_deploy.operator_evidence.file_status')"
PUBLIC_DEPLOY_LOCAL_STATUS="$(read_json_field "$EXTERNAL_OPS_EVIDENCE" '.public_network_deploy.local_drill.status')"

BLOCKERS=()
[[ "$DEV_REPOSITORY_STATUS" == "file_repository_persistence_green" ]] || BLOCKERS+=("dev_runtime_file_repository")
[[ "$BROWSER_PARITY_STATUS" == "standalone_browser_parity_green" ]] || BLOCKERS+=("standalone_browser_parity_evidence")
[[ "$REPOSITORY_ADAPTER_STATUS" == "repository_adapter_boundary_green" ]] || BLOCKERS+=("repository_adapter_boundary_evidence")
[[ "$ROLLBACK_BACKUP_STATUS" == "release_rollback_backup_drill_green" ]] || BLOCKERS+=("release_rollback_backup_drill")
[[ "$COHORT_COMMERCIAL_SCHEMA_STATUS" == "cohort_commercial_evidence_schema_green" ]] || BLOCKERS+=("cohort_commercial_schema_evidence")
[[ "$S5_STATUS" == "s5_real_device_evidence_green" ]] || BLOCKERS+=("s5_real_device_matrix")
[[ "$NATIVE_BEVY_KEYBOARD_REPLAY_CONTRACT" == "trillionnium_world_bevy_build_branch_title_route_all_branch_keyboard_replay_v1" && "$NATIVE_BEVY_KEYBOARD_REPLAY_GREEN" == "true" ]] || BLOCKERS+=("native_bevy_keyboard_replay_contract")
[[ "$NATIVE_BEVY_ACTION_COACH_CONTRACT" == "trillionnium_world_bevy_action_coach_v1" && "$NATIVE_BEVY_ACTION_COACH_GREEN" == "true" ]] || BLOCKERS+=("native_bevy_action_coach_contract")
[[ "$NATIVE_BEVY_PLAYER_HUD_CONTRACT" == "trillionnium_world_bevy_player_hud_debug_layer_v1" && "$NATIVE_BEVY_PLAYER_HUD_GREEN" == "true" ]] || BLOCKERS+=("native_bevy_player_hud_debug_layer_contract")
[[ "$NATIVE_BEVY_LIVE_SCREENSHOT_CONTRACT" == "trillionnium_world_bevy_live_window_screenshot_sequence_v1" && "$NATIVE_BEVY_LIVE_SCREENSHOT_GREEN" == "true" && "$NATIVE_BEVY_LIVE_SCREENSHOT_FRAME_GATE" == "true" && "$NATIVE_BEVY_LIVE_SCREENSHOT_CONTACT_SHEET_GATE" == "true" ]] || BLOCKERS+=("native_bevy_live_window_screenshot_sequence_contract")
[[ "$NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_CONTRACT" == "trillionnium_world_bevy_sprite_texture_sampling_v1" && "$NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_GREEN" == "true" && "$NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_FOUR_LAYER_GATE" == "true" && "$NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_NONBLANK_GATE" == "true" ]] || BLOCKERS+=("native_bevy_sprite_texture_sampling_contract")
[[ "$NATIVE_BEVY_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION_CONTRACT" == "trillionnium_world_bevy_live_window_sampled_texture_correlation_v1" && "$NATIVE_BEVY_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION_GREEN" == "true" && "$NATIVE_BEVY_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION_GATE" == "true" ]] || BLOCKERS+=("native_bevy_live_window_sampled_texture_correlation_contract")
[[ "$NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_CONTRACT" == "trillionnium_world_bevy_render_asset_eligibility_v1" && "$NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_GREEN" == "true" && "$NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_USAGE_GATE" == "true" && "$NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_REFERENCE_GATE" == "true" ]] || BLOCKERS+=("native_bevy_render_asset_eligibility_contract")
[[ "$MAP_PACK_STATUS" == "fixture_signed_map_pack_gate_green" && "$MAP_PACK_MANIFEST_STATUS" == "present" ]] || BLOCKERS+=("signed_map_pack_manifest")
if [[ "$PRODUCTION_MAP_PACK_STATUS" == "production_map_pack_public_ready_green" ]]; then
  true
elif [[ "$PRODUCTION_MAP_PACK_STATUS" == "blocked_missing_production_map_pack_public_evidence" ]]; then
  BLOCKERS+=("production_map_pack_public_evidence")
else
  BLOCKERS+=("production_map_pack_public_evidence")
fi
[[ "$FIRST_BETA_STATUS" == "first_beta_cohort_evidence_green" ]] || BLOCKERS+=("first_beta_cohort_evidence")
[[ "$COMMERCIAL_DRILL_STATUS" == "commercial_launch_drill_evidence_green" ]] || BLOCKERS+=("commercial_launch_drill_evidence")
[[ "$MULTI_NODE_STATUS" == "multi_node_or_live_traffic_latency_green" ]] || BLOCKERS+=("multi_node_or_live_traffic_latency_evidence")
[[ "$PUBLIC_DEPLOY_STATUS" == "public_network_deploy_green" ]] || BLOCKERS+=("public_network_live_exposure_evidence")

OVERALL_STATUS="ready_for_public_launch_review"
if [[ "${#BLOCKERS[@]}" -gt 0 ]]; then
  OVERALL_STATUS="blocked_missing_public_launch_evidence"
fi

BLOCKERS_JSON="$(printf '%s\n' "${BLOCKERS[@]}" | jq -Rsc 'split("\n") | map(select(length > 0))')"

jq -n \
  --arg contract_version "trillionnium_world_public_launch_readiness_v1" \
  --arg overall_status "$OVERALL_STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg dev_repository_evidence "$DEV_REPOSITORY_EVIDENCE" \
  --arg dev_repository_status "${DEV_REPOSITORY_STATUS:-missing}" \
  --arg browser_parity_evidence "$BROWSER_PARITY_EVIDENCE" \
  --arg browser_parity_status "$BROWSER_PARITY_STATUS" \
  --arg browser_parity_file_status "$BROWSER_PARITY_FILE_STATUS" \
  --arg repository_adapter_evidence "$REPOSITORY_ADAPTER_EVIDENCE" \
  --arg repository_adapter_status "$REPOSITORY_ADAPTER_STATUS" \
  --arg repository_adapter_file_status "$REPOSITORY_ADAPTER_FILE_STATUS" \
  --arg rollback_backup_evidence "$ROLLBACK_BACKUP_EVIDENCE" \
  --arg rollback_backup_status "$ROLLBACK_BACKUP_STATUS" \
  --arg rollback_backup_file_status "$ROLLBACK_BACKUP_FILE_STATUS" \
  --arg cohort_commercial_schema_evidence "$COHORT_COMMERCIAL_SCHEMA_EVIDENCE" \
  --arg cohort_commercial_schema_status "$COHORT_COMMERCIAL_SCHEMA_STATUS" \
  --arg cohort_commercial_schema_file_status "$COHORT_COMMERCIAL_SCHEMA_FILE_STATUS" \
  --arg cohort_commercial_evidence "$COHORT_COMMERCIAL_EVIDENCE" \
  --arg cohort_commercial_status "$COHORT_COMMERCIAL_STATUS" \
  --arg cohort_commercial_file_status "$COHORT_COMMERCIAL_FILE_STATUS" \
  --arg external_ops_evidence "$EXTERNAL_OPS_EVIDENCE" \
  --arg external_ops_status "$EXTERNAL_OPS_STATUS" \
  --arg external_ops_file_status "$EXTERNAL_OPS_FILE_STATUS" \
  --arg s5_evidence "$S5_EVIDENCE" \
  --arg s5_status "${S5_STATUS:-missing}" \
  --arg s5_file_status "$S5_FILE_STATUS" \
  --arg s5_validation "$S5_REAL_DEVICE_VALIDATION" \
  --arg s5_validation_file_status "$S5_VALIDATION_FILE_STATUS" \
  --arg native_bevy_keyboard_replay_evidence "$NATIVE_BEVY_KEYBOARD_REPLAY_EVIDENCE" \
  --arg native_bevy_keyboard_replay_contract "$NATIVE_BEVY_KEYBOARD_REPLAY_CONTRACT" \
  --arg native_bevy_keyboard_replay_green "${NATIVE_BEVY_KEYBOARD_REPLAY_GREEN:-missing}" \
  --arg native_bevy_keyboard_replay_file_status "$NATIVE_BEVY_KEYBOARD_REPLAY_FILE_STATUS" \
  --arg native_bevy_action_coach_evidence "$NATIVE_BEVY_ACTION_COACH_EVIDENCE" \
  --arg native_bevy_action_coach_contract "$NATIVE_BEVY_ACTION_COACH_CONTRACT" \
  --arg native_bevy_action_coach_green "${NATIVE_BEVY_ACTION_COACH_GREEN:-missing}" \
  --arg native_bevy_action_coach_file_status "$NATIVE_BEVY_ACTION_COACH_FILE_STATUS" \
  --arg native_bevy_player_hud_evidence "$NATIVE_BEVY_PLAYER_HUD_EVIDENCE" \
  --arg native_bevy_player_hud_contract "$NATIVE_BEVY_PLAYER_HUD_CONTRACT" \
  --arg native_bevy_player_hud_green "${NATIVE_BEVY_PLAYER_HUD_GREEN:-missing}" \
  --arg native_bevy_player_hud_file_status "$NATIVE_BEVY_PLAYER_HUD_FILE_STATUS" \
  --arg native_bevy_live_screenshot_evidence "$NATIVE_BEVY_LIVE_SCREENSHOT_EVIDENCE" \
  --arg native_bevy_live_screenshot_contract "$NATIVE_BEVY_LIVE_SCREENSHOT_CONTRACT" \
  --arg native_bevy_live_screenshot_green "${NATIVE_BEVY_LIVE_SCREENSHOT_GREEN:-missing}" \
  --arg native_bevy_live_screenshot_frame_gate "${NATIVE_BEVY_LIVE_SCREENSHOT_FRAME_GATE:-missing}" \
  --arg native_bevy_live_screenshot_contact_sheet_gate "${NATIVE_BEVY_LIVE_SCREENSHOT_CONTACT_SHEET_GATE:-missing}" \
  --arg native_bevy_live_screenshot_file_status "$NATIVE_BEVY_LIVE_SCREENSHOT_FILE_STATUS" \
  --arg native_bevy_sprite_texture_sampling_evidence "$NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_EVIDENCE" \
  --arg native_bevy_sprite_texture_sampling_contract "$NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_CONTRACT" \
  --arg native_bevy_sprite_texture_sampling_green "${NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_GREEN:-missing}" \
  --arg native_bevy_sprite_texture_sampling_four_layer_gate "${NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_FOUR_LAYER_GATE:-missing}" \
  --arg native_bevy_sprite_texture_sampling_nonblank_gate "${NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_NONBLANK_GATE:-missing}" \
  --arg native_bevy_sprite_texture_sampling_file_status "$NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_FILE_STATUS" \
  --arg native_bevy_live_window_sampled_texture_correlation_evidence "$NATIVE_BEVY_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION_EVIDENCE" \
  --arg native_bevy_live_window_sampled_texture_correlation_contract "$NATIVE_BEVY_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION_CONTRACT" \
  --arg native_bevy_live_window_sampled_texture_correlation_green "${NATIVE_BEVY_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION_GREEN:-missing}" \
  --arg native_bevy_live_window_sampled_texture_correlation_gate "${NATIVE_BEVY_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION_GATE:-missing}" \
  --arg native_bevy_live_window_sampled_texture_correlation_file_status "$NATIVE_BEVY_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION_FILE_STATUS" \
  --arg native_bevy_render_asset_eligibility_evidence "$NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_EVIDENCE" \
  --arg native_bevy_render_asset_eligibility_contract "$NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_CONTRACT" \
  --arg native_bevy_render_asset_eligibility_green "${NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_GREEN:-missing}" \
  --arg native_bevy_render_asset_eligibility_usage_gate "${NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_USAGE_GATE:-missing}" \
  --arg native_bevy_render_asset_eligibility_reference_gate "${NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_REFERENCE_GATE:-missing}" \
  --arg native_bevy_render_asset_eligibility_file_status "$NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_FILE_STATUS" \
  --arg map_pack_manifest "$MAP_PACK_MANIFEST" \
  --arg map_pack_summary "$MAP_PACK_SUMMARY" \
  --arg map_pack_status "$MAP_PACK_STATUS" \
  --arg map_pack_manifest_status "$MAP_PACK_MANIFEST_STATUS" \
  --arg production_map_pack_evidence "$PRODUCTION_MAP_PACK_EVIDENCE" \
  --arg production_map_pack_status "$PRODUCTION_MAP_PACK_STATUS" \
  --arg production_map_pack_file_status "$PRODUCTION_MAP_PACK_FILE_STATUS" \
  --arg first_beta_evidence "$FIRST_BETA_EVIDENCE" \
  --arg first_beta_status "$FIRST_BETA_STATUS" \
  --arg first_beta_file_status "$FIRST_BETA_FILE_STATUS" \
  --arg commercial_drill_evidence "$COMMERCIAL_DRILL_EVIDENCE" \
  --arg commercial_drill_status "$COMMERCIAL_DRILL_STATUS" \
  --arg commercial_drill_file_status "$COMMERCIAL_DRILL_FILE_STATUS" \
  --arg multi_node_evidence "$MULTI_NODE_EVIDENCE" \
  --arg multi_node_status "$MULTI_NODE_STATUS" \
  --arg multi_node_file_status "$MULTI_NODE_FILE_STATUS" \
  --arg multi_node_local_status "$MULTI_NODE_LOCAL_STATUS" \
  --arg public_deploy_evidence "$PUBLIC_DEPLOY_EVIDENCE" \
  --arg public_deploy_status "$PUBLIC_DEPLOY_STATUS" \
  --arg public_deploy_file_status "$PUBLIC_DEPLOY_FILE_STATUS" \
  --arg public_deploy_local_status "$PUBLIC_DEPLOY_LOCAL_STATUS" \
  --argjson blockers "$BLOCKERS_JSON" \
  '{
    contract_version: $contract_version,
    overall_status: $overall_status,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_public_launch_readiness_gate",
    launch_rule: "do_not_claim_public_launch_ready_without_native_bevy_local_playability_texture_sampling_render_asset_eligibility_real_device_map_pack_cohort_commercial_multi_node_and_public_deploy_evidence",
    blockers: $blockers,
    gates: {
      dev_runtime_repository: {
        evidence_path: $dev_repository_evidence,
        status: $dev_repository_status,
        required_status: "file_repository_persistence_green"
      },
      standalone_browser_parity: {
        evidence_path: $browser_parity_evidence,
        status: $browser_parity_status,
        file_status: $browser_parity_file_status,
        accepted_status: "standalone_browser_parity_green"
      },
      repository_adapter_boundary: {
        evidence_path: $repository_adapter_evidence,
        status: $repository_adapter_status,
        file_status: $repository_adapter_file_status,
        accepted_status: "repository_adapter_boundary_green"
      },
      release_rollback_backup: {
        evidence_path: $rollback_backup_evidence,
        status: $rollback_backup_status,
        file_status: $rollback_backup_file_status,
        accepted_status: "release_rollback_backup_drill_green"
      },
      cohort_commercial_schema: {
        evidence_path: $cohort_commercial_schema_evidence,
        status: $cohort_commercial_schema_status,
        file_status: $cohort_commercial_schema_file_status,
        accepted_status: "cohort_commercial_evidence_schema_green"
      },
      cohort_commercial_evidence: {
        evidence_path: $cohort_commercial_evidence,
        status: $cohort_commercial_status,
        file_status: $cohort_commercial_file_status,
        accepted_status: "cohort_commercial_evidence_green"
      },
      external_ops_evidence: {
        evidence_path: $external_ops_evidence,
        status: $external_ops_status,
        file_status: $external_ops_file_status,
        accepted_status: "external_ops_evidence_green"
      },
      s5_real_device_matrix: {
        evidence_path: $s5_evidence,
        status: $s5_status,
        file_status: $s5_file_status,
        validator_summary: $s5_validation,
        validator_file_status: $s5_validation_file_status,
        required_status: "s5_real_device_evidence_green"
      },
      native_bevy_keyboard_replay: {
        evidence_path: $native_bevy_keyboard_replay_evidence,
        file_status: $native_bevy_keyboard_replay_file_status,
        contract_version: $native_bevy_keyboard_replay_contract,
        green: ($native_bevy_keyboard_replay_green == "true"),
        required_contract: "trillionnium_world_bevy_build_branch_title_route_all_branch_keyboard_replay_v1",
        required_green: true,
        proof_scope: "host_side_bevy_runtime_replay_not_android_real_device"
      },
      native_bevy_action_coach: {
        evidence_path: $native_bevy_action_coach_evidence,
        file_status: $native_bevy_action_coach_file_status,
        contract_version: $native_bevy_action_coach_contract,
        green: ($native_bevy_action_coach_green == "true"),
        required_contract: "trillionnium_world_bevy_action_coach_v1",
        required_green: true,
        proof_scope: "host_side_bevy_runtime_guidance_not_android_real_device"
      },
      native_bevy_player_hud_debug_layer: {
        evidence_path: $native_bevy_player_hud_evidence,
        file_status: $native_bevy_player_hud_file_status,
        contract_version: $native_bevy_player_hud_contract,
        green: ($native_bevy_player_hud_green == "true"),
        required_contract: "trillionnium_world_bevy_player_hud_debug_layer_v1",
        required_green: true,
        proof_scope: "host_side_bevy_hud_layer_not_android_real_device"
      },
      native_bevy_live_window_screenshot_sequence: {
        evidence_path: $native_bevy_live_screenshot_evidence,
        file_status: $native_bevy_live_screenshot_file_status,
        contract_version: $native_bevy_live_screenshot_contract,
        green: ($native_bevy_live_screenshot_green == "true"),
        frame_sequence_gate: ($native_bevy_live_screenshot_frame_gate == "true"),
        contact_sheet_gate: ($native_bevy_live_screenshot_contact_sheet_gate == "true"),
        required_contract: "trillionnium_world_bevy_live_window_screenshot_sequence_v1",
        required_green: true,
        proof_scope: "host_side_live_window_screenshot_sequence_not_android_real_device"
      },
      native_bevy_sprite_texture_sampling: {
        evidence_path: $native_bevy_sprite_texture_sampling_evidence,
        file_status: $native_bevy_sprite_texture_sampling_file_status,
        contract_version: $native_bevy_sprite_texture_sampling_contract,
        green: ($native_bevy_sprite_texture_sampling_green == "true"),
        four_layer_texture_sampling_gate: ($native_bevy_sprite_texture_sampling_four_layer_gate == "true"),
        texture_sample_nonblank_gate: ($native_bevy_sprite_texture_sampling_nonblank_gate == "true"),
        required_contract: "trillionnium_world_bevy_sprite_texture_sampling_v1",
        required_green: true,
        proof_scope: "host_side_cpu_texture_sampling_not_gpu_upload_or_android_real_device"
      },
      native_bevy_live_window_sampled_texture_correlation: {
        evidence_path: $native_bevy_live_window_sampled_texture_correlation_evidence,
        file_status: $native_bevy_live_window_sampled_texture_correlation_file_status,
        contract_version: $native_bevy_live_window_sampled_texture_correlation_contract,
        green: ($native_bevy_live_window_sampled_texture_correlation_green == "true"),
        four_layer_sampled_live_correlation_gate: ($native_bevy_live_window_sampled_texture_correlation_gate == "true"),
        required_contract: "trillionnium_world_bevy_live_window_sampled_texture_correlation_v1",
        required_green: true,
        proof_scope: "host_side_sampled_texture_to_live_window_correlation_not_android_real_device"
      },
      native_bevy_render_asset_eligibility: {
        evidence_path: $native_bevy_render_asset_eligibility_evidence,
        file_status: $native_bevy_render_asset_eligibility_file_status,
        contract_version: $native_bevy_render_asset_eligibility_contract,
        green: ($native_bevy_render_asset_eligibility_green == "true"),
        render_asset_usage_gate: ($native_bevy_render_asset_eligibility_usage_gate == "true"),
        sprite_render_reference_gate: ($native_bevy_render_asset_eligibility_reference_gate == "true"),
        required_contract: "trillionnium_world_bevy_render_asset_eligibility_v1",
        required_green: true,
        proof_scope: "host_side_render_asset_eligibility_not_render_world_extraction_or_gpu_upload"
      },
      signed_map_pack: {
        evidence_path: $map_pack_manifest,
        summary_path: $map_pack_summary,
        status: $map_pack_status,
        manifest_status: $map_pack_manifest_status,
        required_status: "fixture_signed_map_pack_gate_green"
      },
      production_map_pack: {
        evidence_path: $production_map_pack_evidence,
        status: $production_map_pack_status,
        file_status: $production_map_pack_file_status,
        accepted_status: "production_map_pack_public_ready_green",
        local_route_status: "production_map_pack_route_green",
        evidence_contract: "trillionnium_world_production_map_pack_public_evidence_gate_v1",
        live_ingestion_allowed: false,
        runtime_clients_fetch_public_osm_directly: false
      },
      first_beta_cohort: {
        evidence_path: $first_beta_evidence,
        status: $first_beta_status,
        file_status: $first_beta_file_status,
        accepted_status: "first_beta_cohort_evidence_green",
        validator_summary: $cohort_commercial_evidence
      },
      commercial_launch_drill: {
        evidence_path: $commercial_drill_evidence,
        status: $commercial_drill_status,
        file_status: $commercial_drill_file_status,
        accepted_status: "commercial_launch_drill_evidence_green",
        validator_summary: $cohort_commercial_evidence
      },
      multi_node_or_live_traffic_latency: {
        evidence_path: $multi_node_evidence,
        status: $multi_node_status,
        file_status: $multi_node_file_status,
        accepted_status: "multi_node_or_live_traffic_latency_green",
        local_drill_status: $multi_node_local_status,
        validator_summary: $external_ops_evidence
      },
      public_network_deploy: {
        evidence_path: $public_deploy_evidence,
        status: $public_deploy_status,
        file_status: $public_deploy_file_status,
        accepted_status: "public_network_deploy_green",
        local_drill_status: $public_deploy_local_status,
        validator_summary: $external_ops_evidence
      }
    }
  }' >"$SUMMARY_FILE"

if [[ "$OVERALL_STATUS" == "ready_for_public_launch_review" ]]; then
  printf 'TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_READY %s\n' "$SUMMARY_FILE"
  exit 0
fi

printf 'TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_BLOCKED %s %s\n' "$OVERALL_STATUS" "$SUMMARY_FILE"
if [[ "$REQUIRE_READY" -eq 1 ]]; then
  exit 1
fi
