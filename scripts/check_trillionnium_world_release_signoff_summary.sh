#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
SUMMARY_FILE="$ACCEPTANCE_DIR/release-signoff-summary.json"
if [[ -v TRILLIONNIUM_WORLD_RELEASE_SIGNOFF_SUMMARY && -n "$TRILLIONNIUM_WORLD_RELEASE_SIGNOFF_SUMMARY" ]]; then
  SUMMARY_FILE="$TRILLIONNIUM_WORLD_RELEASE_SIGNOFF_SUMMARY"
fi
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

mkdir -p "$ACCEPTANCE_DIR"

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

json_bool() {
  if [[ "$1" == "true" ]]; then
    printf 'true'
  else
    printf 'false'
  fi
}

PUBLIC_LAUNCH_EVIDENCE="$ACCEPTANCE_DIR/public-launch-readiness.json"
NATIVE_BEVY_KEYBOARD_REPLAY_EVIDENCE="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-build-branch-title-route-all-branch-keyboard-replay.json"
NATIVE_BEVY_ACTION_COACH_EVIDENCE="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-action-coach.json"
NATIVE_BEVY_PLAYER_HUD_EVIDENCE="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-player-hud-debug-layer.json"
NATIVE_BEVY_LIVE_SCREENSHOT_EVIDENCE="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-live-window-screenshot-sequence.json"
NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_EVIDENCE="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-sprite-texture-sampling.json"
NATIVE_BEVY_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION_EVIDENCE="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-live-window-sampled-texture-correlation.json"
NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_EVIDENCE="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-render-asset-eligibility.json"
CEX_ADAPTER_EVIDENCE="$ROOT/acceptance/S3_repository_adapter/latest/cex-production-adapter-readiness.json"
S5_EVIDENCE="$ROOT/acceptance/S5_native_bevy_device/latest/s5-device-evidence.json"
RELEASE_LATENCY_EVIDENCE="$ACCEPTANCE_DIR/release-latency-drill.json"
ROLLBACK_BACKUP_EVIDENCE="$ACCEPTANCE_DIR/release-rollback-backup-drill.json"
PUBLIC_DEPLOY_EVIDENCE="$ACCEPTANCE_DIR/public-network-deploy-evidence.json"

PUBLIC_LAUNCH_STATUS="$(read_json_field "$PUBLIC_LAUNCH_EVIDENCE" '.overall_status')"
PUBLIC_LAUNCH_FILE_STATUS="$(file_status "$PUBLIC_LAUNCH_EVIDENCE")"
if [[ -f "$PUBLIC_LAUNCH_EVIDENCE" ]]; then
  PUBLIC_LAUNCH_BLOCKERS_JSON="$(jq -c '.blockers // []' "$PUBLIC_LAUNCH_EVIDENCE")"
else
  PUBLIC_LAUNCH_BLOCKERS_JSON='["public_launch_readiness_summary"]'
fi
PUBLIC_LAUNCH_CONSUMES_REPLAY="$(read_json_field "$PUBLIC_LAUNCH_EVIDENCE" '(.gates.native_bevy_keyboard_replay.green == true) and ((.blockers // []) | index("native_bevy_keyboard_replay_contract") | not)')"
PUBLIC_LAUNCH_CONSUMES_ACTION_COACH="$(read_json_field "$PUBLIC_LAUNCH_EVIDENCE" '(.gates.native_bevy_action_coach.green == true) and ((.blockers // []) | index("native_bevy_action_coach_contract") | not)')"
PUBLIC_LAUNCH_CONSUMES_PLAYER_HUD="$(read_json_field "$PUBLIC_LAUNCH_EVIDENCE" '(.gates.native_bevy_player_hud_debug_layer.green == true) and ((.blockers // []) | index("native_bevy_player_hud_debug_layer_contract") | not)')"
PUBLIC_LAUNCH_CONSUMES_LIVE_SCREENSHOT="$(read_json_field "$PUBLIC_LAUNCH_EVIDENCE" '(.gates.native_bevy_live_window_screenshot_sequence.green == true) and (.gates.native_bevy_live_window_screenshot_sequence.frame_sequence_gate == true) and (.gates.native_bevy_live_window_screenshot_sequence.contact_sheet_gate == true) and ((.blockers // []) | index("native_bevy_live_window_screenshot_sequence_contract") | not)')"
PUBLIC_LAUNCH_CONSUMES_SPRITE_TEXTURE_SAMPLING="$(read_json_field "$PUBLIC_LAUNCH_EVIDENCE" '(.gates.native_bevy_sprite_texture_sampling.green == true) and (.gates.native_bevy_sprite_texture_sampling.four_layer_texture_sampling_gate == true) and (.gates.native_bevy_sprite_texture_sampling.texture_sample_nonblank_gate == true) and ((.blockers // []) | index("native_bevy_sprite_texture_sampling_contract") | not)')"
PUBLIC_LAUNCH_CONSUMES_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION="$(read_json_field "$PUBLIC_LAUNCH_EVIDENCE" '(.gates.native_bevy_live_window_sampled_texture_correlation.green == true) and (.gates.native_bevy_live_window_sampled_texture_correlation.four_layer_sampled_live_correlation_gate == true) and ((.blockers // []) | index("native_bevy_live_window_sampled_texture_correlation_contract") | not)')"
PUBLIC_LAUNCH_CONSUMES_RENDER_ASSET_ELIGIBILITY="$(read_json_field "$PUBLIC_LAUNCH_EVIDENCE" '(.gates.native_bevy_render_asset_eligibility.green == true) and (.gates.native_bevy_render_asset_eligibility.render_asset_usage_gate == true) and (.gates.native_bevy_render_asset_eligibility.sprite_render_reference_gate == true) and ((.blockers // []) | index("native_bevy_render_asset_eligibility_contract") | not)')"

NATIVE_REPLAY_CONTRACT="$(read_json_field "$NATIVE_BEVY_KEYBOARD_REPLAY_EVIDENCE" '.contract_version')"
NATIVE_REPLAY_GREEN="$(read_json_field "$NATIVE_BEVY_KEYBOARD_REPLAY_EVIDENCE" '.green')"
NATIVE_REPLAY_BRANCH_COUNT="$(read_json_field "$NATIVE_BEVY_KEYBOARD_REPLAY_EVIDENCE" '.branch_count')"
NATIVE_REPLAY_ALL_BRANCH_GATE="$(read_json_field "$NATIVE_BEVY_KEYBOARD_REPLAY_EVIDENCE" '.all_branch_replay_gate')"
NATIVE_REPLAY_FILE_STATUS="$(file_status "$NATIVE_BEVY_KEYBOARD_REPLAY_EVIDENCE")"
FORCE_SEQUENCE_COUNT="$(read_json_field "$NATIVE_BEVY_KEYBOARD_REPLAY_EVIDENCE" '.replay_results.force.recorded_sequence_count')"
AGILITY_SEQUENCE_COUNT="$(read_json_field "$NATIVE_BEVY_KEYBOARD_REPLAY_EVIDENCE" '.replay_results.agility.recorded_sequence_count')"
CRAFT_SEQUENCE_COUNT="$(read_json_field "$NATIVE_BEVY_KEYBOARD_REPLAY_EVIDENCE" '.replay_results.craft.recorded_sequence_count')"
FORCE_FINAL_STATUS="$(read_json_field "$NATIVE_BEVY_KEYBOARD_REPLAY_EVIDENCE" '.replay_results.force.replay_final_runtime.objective_status')"
AGILITY_FINAL_STATUS="$(read_json_field "$NATIVE_BEVY_KEYBOARD_REPLAY_EVIDENCE" '.replay_results.agility.replay_final_runtime.objective_status')"
CRAFT_FINAL_STATUS="$(read_json_field "$NATIVE_BEVY_KEYBOARD_REPLAY_EVIDENCE" '.replay_results.craft.replay_final_runtime.objective_status')"
FORCE_COMBAT_STATUS="$(read_json_field "$NATIVE_BEVY_KEYBOARD_REPLAY_EVIDENCE" '.replay_results.force.replay_final_runtime.combat_result_state')"
ACTION_COACH_CONTRACT="$(read_json_field "$NATIVE_BEVY_ACTION_COACH_EVIDENCE" '.contract_version')"
ACTION_COACH_GREEN="$(read_json_field "$NATIVE_BEVY_ACTION_COACH_EVIDENCE" '.green')"
ACTION_COACH_STAGE_GATE="$(read_json_field "$NATIVE_BEVY_ACTION_COACH_EVIDENCE" '.coach_stage_gate')"
ACTION_COACH_ENTER_GATE="$(read_json_field "$NATIVE_BEVY_ACTION_COACH_EVIDENCE" '.enter_execution_gate')"
ACTION_COACH_FINAL_NEXT_GATE="$(read_json_field "$NATIVE_BEVY_ACTION_COACH_EVIDENCE" '.final_next_gate')"
ACTION_COACH_FILE_STATUS="$(file_status "$NATIVE_BEVY_ACTION_COACH_EVIDENCE")"
PLAYER_HUD_CONTRACT="$(read_json_field "$NATIVE_BEVY_PLAYER_HUD_EVIDENCE" '.contract_version')"
PLAYER_HUD_GREEN="$(read_json_field "$NATIVE_BEVY_PLAYER_HUD_EVIDENCE" '.green')"
PLAYER_HUD_GATE="$(read_json_field "$NATIVE_BEVY_PLAYER_HUD_EVIDENCE" '.player_hud_gate')"
PLAYER_HUD_DEBUG_GATE="$(read_json_field "$NATIVE_BEVY_PLAYER_HUD_EVIDENCE" '.debug_layer_gate')"
PLAYER_HUD_FILE_STATUS="$(file_status "$NATIVE_BEVY_PLAYER_HUD_EVIDENCE")"
LIVE_SCREENSHOT_CONTRACT="$(read_json_field "$NATIVE_BEVY_LIVE_SCREENSHOT_EVIDENCE" '.contract_version')"
LIVE_SCREENSHOT_GREEN="$(read_json_field "$NATIVE_BEVY_LIVE_SCREENSHOT_EVIDENCE" '.green')"
LIVE_SCREENSHOT_FRAME_SEQUENCE_GATE="$(read_json_field "$NATIVE_BEVY_LIVE_SCREENSHOT_EVIDENCE" '.frame_sequence_gate')"
LIVE_SCREENSHOT_CONTACT_SHEET_GATE="$(read_json_field "$NATIVE_BEVY_LIVE_SCREENSHOT_EVIDENCE" '.contact_sheet_gate')"
LIVE_SCREENSHOT_ACTUAL_FRAME_COUNT="$(read_json_field "$NATIVE_BEVY_LIVE_SCREENSHOT_EVIDENCE" '.actual_frame_ids | length')"
LIVE_SCREENSHOT_FILE_STATUS="$(file_status "$NATIVE_BEVY_LIVE_SCREENSHOT_EVIDENCE")"
SPRITE_TEXTURE_SAMPLING_CONTRACT="$(read_json_field "$NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_EVIDENCE" '.contract_version')"
SPRITE_TEXTURE_SAMPLING_GREEN="$(read_json_field "$NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_EVIDENCE" '.green')"
SPRITE_TEXTURE_SAMPLING_FOUR_LAYER_GATE="$(read_json_field "$NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_EVIDENCE" '.four_layer_texture_sampling_gate')"
SPRITE_TEXTURE_SAMPLING_NONBLANK_GATE="$(read_json_field "$NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_EVIDENCE" '.texture_sample_nonblank_gate')"
SPRITE_TEXTURE_SAMPLING_SURFACE_COUNT="$(read_json_field "$NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_EVIDENCE" '.sampled_surface_count')"
SPRITE_TEXTURE_SAMPLING_UNIQUE_COLOR_COUNT="$(read_json_field "$NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_EVIDENCE" '.texture_unique_rgba_color_count')"
SPRITE_TEXTURE_SAMPLING_FILE_STATUS="$(file_status "$NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_EVIDENCE")"
SAMPLED_TEXTURE_CORRELATION_CONTRACT="$(read_json_field "$NATIVE_BEVY_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION_EVIDENCE" '.contract_version')"
SAMPLED_TEXTURE_CORRELATION_GREEN="$(read_json_field "$NATIVE_BEVY_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION_EVIDENCE" '.green')"
SAMPLED_TEXTURE_CORRELATION_FOUR_LAYER_GATE="$(read_json_field "$NATIVE_BEVY_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION_EVIDENCE" '.gates.four_layer_sampled_live_correlation_gate')"
SAMPLED_TEXTURE_CORRELATION_LIVE_FRAME_COUNT="$(read_json_field "$NATIVE_BEVY_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION_EVIDENCE" '.live_frame_count')"
SAMPLED_TEXTURE_CORRELATION_LIVE_FINAL_FRAME_COLOR_COUNT="$(read_json_field "$NATIVE_BEVY_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION_EVIDENCE" '.live_final_frame_colors_96x54')"
SAMPLED_TEXTURE_CORRELATION_FILE_STATUS="$(file_status "$NATIVE_BEVY_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION_EVIDENCE")"
RENDER_ASSET_ELIGIBILITY_CONTRACT="$(read_json_field "$NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_EVIDENCE" '.contract_version')"
RENDER_ASSET_ELIGIBILITY_GREEN="$(read_json_field "$NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_EVIDENCE" '.green')"
RENDER_ASSET_ELIGIBILITY_USAGE_GATE="$(read_json_field "$NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_EVIDENCE" '.render_asset_usage_gate')"
RENDER_ASSET_ELIGIBILITY_DESCRIPTOR_GATE="$(read_json_field "$NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_EVIDENCE" '.image_descriptor_render_eligibility_gate')"
RENDER_ASSET_ELIGIBILITY_ATLAS_GATE="$(read_json_field "$NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_EVIDENCE" '.atlas_layout_render_eligibility_gate')"
RENDER_ASSET_ELIGIBILITY_REFERENCE_GATE="$(read_json_field "$NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_EVIDENCE" '.sprite_render_reference_gate')"
RENDER_ASSET_ELIGIBILITY_IMAGE_USAGE_DEBUG="$(read_json_field "$NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_EVIDENCE" '.image_asset_usage_debug')"
RENDER_ASSET_ELIGIBILITY_SPRITE_REFERENCE_COUNT="$(read_json_field "$NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_EVIDENCE" '.sprite_render_reference_count')"
RENDER_ASSET_ELIGIBILITY_FILE_STATUS="$(file_status "$NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_EVIDENCE")"
CEX_ADAPTER_CONTRACT="$(read_json_field "$CEX_ADAPTER_EVIDENCE" '.contract_version')"
CEX_ADAPTER_GREEN="$(read_json_field "$CEX_ADAPTER_EVIDENCE" '.green')"
CEX_ADAPTER_STATUS="$(read_json_field "$CEX_ADAPTER_EVIDENCE" '.status')"
CEX_ADAPTER_PROTOCOL="$(read_json_field "$CEX_ADAPTER_EVIDENCE" '.observed.protocol_contract')"
CEX_ADAPTER_DOMAIN="$(read_json_field "$CEX_ADAPTER_EVIDENCE" '.observed.domain_contract')"
CEX_ADAPTER_SOURCE_CONTRACT="$(read_json_field "$CEX_ADAPTER_EVIDENCE" '.observed.contract_version')"
CEX_ADAPTER_ROUTE_RECORD_TOTAL="$(read_json_field "$CEX_ADAPTER_EVIDENCE" '.observed.route_record_total')"
CEX_ADAPTER_WORLD_NODE_COUNT="$(read_json_field "$CEX_ADAPTER_EVIDENCE" '.observed.world_node_count')"
CEX_ADAPTER_FILE_STATUS="$(file_status "$CEX_ADAPTER_EVIDENCE")"

S5_STATUS="$(read_json_field "$S5_EVIDENCE" '.overall_status')"
S5_FILE_STATUS="$(file_status "$S5_EVIDENCE")"
RELEASE_LATENCY_STATUS="$(read_json_field "$RELEASE_LATENCY_EVIDENCE" '.status')"
RELEASE_LATENCY_FILE_STATUS="$(file_status "$RELEASE_LATENCY_EVIDENCE")"
ROLLBACK_BACKUP_STATUS="$(read_json_field "$ROLLBACK_BACKUP_EVIDENCE" '.status')"
ROLLBACK_BACKUP_FILE_STATUS="$(file_status "$ROLLBACK_BACKUP_EVIDENCE")"
PUBLIC_DEPLOY_STATUS="$(read_json_field "$PUBLIC_DEPLOY_EVIDENCE" '.status')"
PUBLIC_DEPLOY_FILE_STATUS="$(file_status "$PUBLIC_DEPLOY_EVIDENCE")"

NATIVE_REPLAY_READY=false
if [[ "$NATIVE_REPLAY_CONTRACT" == "trillionnium_world_bevy_build_branch_title_route_all_branch_keyboard_replay_v1" \
  && "$NATIVE_REPLAY_GREEN" == "true" \
  && "$NATIVE_REPLAY_BRANCH_COUNT" == "3" \
  && "$NATIVE_REPLAY_ALL_BRANCH_GATE" == "true" ]]; then
  NATIVE_REPLAY_READY=true
fi

PUBLIC_LAUNCH_REPLAY_READY=false
if [[ "$PUBLIC_LAUNCH_CONSUMES_REPLAY" == "true" ]]; then
  PUBLIC_LAUNCH_REPLAY_READY=true
fi

ACTION_COACH_READY=false
if [[ "$ACTION_COACH_CONTRACT" == "trillionnium_world_bevy_action_coach_v1" \
  && "$ACTION_COACH_GREEN" == "true" \
  && "$ACTION_COACH_STAGE_GATE" == "true" \
  && "$ACTION_COACH_ENTER_GATE" == "true" \
  && "$ACTION_COACH_FINAL_NEXT_GATE" == "true" ]]; then
  ACTION_COACH_READY=true
fi

PLAYER_HUD_READY=false
if [[ "$PLAYER_HUD_CONTRACT" == "trillionnium_world_bevy_player_hud_debug_layer_v1" \
  && "$PLAYER_HUD_GREEN" == "true" \
  && "$PLAYER_HUD_GATE" == "true" \
  && "$PLAYER_HUD_DEBUG_GATE" == "true" ]]; then
  PLAYER_HUD_READY=true
fi

LIVE_SCREENSHOT_READY=false
if [[ "$LIVE_SCREENSHOT_CONTRACT" == "trillionnium_world_bevy_live_window_screenshot_sequence_v1" \
  && "$LIVE_SCREENSHOT_GREEN" == "true" \
  && "$LIVE_SCREENSHOT_FRAME_SEQUENCE_GATE" == "true" \
  && "$LIVE_SCREENSHOT_CONTACT_SHEET_GATE" == "true" ]]; then
  LIVE_SCREENSHOT_READY=true
fi

SPRITE_TEXTURE_SAMPLING_READY=false
if [[ "$SPRITE_TEXTURE_SAMPLING_CONTRACT" == "trillionnium_world_bevy_sprite_texture_sampling_v1" \
  && "$SPRITE_TEXTURE_SAMPLING_GREEN" == "true" \
  && "$SPRITE_TEXTURE_SAMPLING_FOUR_LAYER_GATE" == "true" \
  && "$SPRITE_TEXTURE_SAMPLING_NONBLANK_GATE" == "true" ]]; then
  SPRITE_TEXTURE_SAMPLING_READY=true
fi

SAMPLED_TEXTURE_CORRELATION_READY=false
if [[ "$SAMPLED_TEXTURE_CORRELATION_CONTRACT" == "trillionnium_world_bevy_live_window_sampled_texture_correlation_v1" \
  && "$SAMPLED_TEXTURE_CORRELATION_GREEN" == "true" \
  && "$SAMPLED_TEXTURE_CORRELATION_FOUR_LAYER_GATE" == "true" ]]; then
  SAMPLED_TEXTURE_CORRELATION_READY=true
fi

RENDER_ASSET_ELIGIBILITY_READY=false
if [[ "$RENDER_ASSET_ELIGIBILITY_CONTRACT" == "trillionnium_world_bevy_render_asset_eligibility_v1" \
  && "$RENDER_ASSET_ELIGIBILITY_GREEN" == "true" \
  && "$RENDER_ASSET_ELIGIBILITY_USAGE_GATE" == "true" \
  && "$RENDER_ASSET_ELIGIBILITY_DESCRIPTOR_GATE" == "true" \
  && "$RENDER_ASSET_ELIGIBILITY_ATLAS_GATE" == "true" \
  && "$RENDER_ASSET_ELIGIBILITY_REFERENCE_GATE" == "true" ]]; then
  RENDER_ASSET_ELIGIBILITY_READY=true
fi

CEX_ADAPTER_READY=false
if [[ "$CEX_ADAPTER_CONTRACT" == "trillionnium_world_cex_adapter_readiness_gate_v1" \
  && "$CEX_ADAPTER_GREEN" == "true" \
  && "$CEX_ADAPTER_STATUS" == "cex_adapter_readiness_green" \
  && "$CEX_ADAPTER_PROTOCOL" == "trillionnium_world_runtime_adapter_v1" \
  && "$CEX_ADAPTER_DOMAIN" == "trillionnium_world_domain_v1" \
  && "$CEX_ADAPTER_SOURCE_CONTRACT" == "cex_trillionnium_world_production_adapter_v1" \
  && "${CEX_ADAPTER_ROUTE_RECORD_TOTAL:-0}" -gt 0 \
  && "${CEX_ADAPTER_WORLD_NODE_COUNT:-0}" -gt 0 ]]; then
  CEX_ADAPTER_READY=true
fi

PUBLIC_LAUNCH_LOCAL_PLAYABILITY_READY=false
if [[ "$PUBLIC_LAUNCH_CONSUMES_ACTION_COACH" == "true" \
  && "$PUBLIC_LAUNCH_CONSUMES_PLAYER_HUD" == "true" \
  && "$PUBLIC_LAUNCH_CONSUMES_LIVE_SCREENSHOT" == "true" \
  && "$PUBLIC_LAUNCH_CONSUMES_SPRITE_TEXTURE_SAMPLING" == "true" \
  && "$PUBLIC_LAUNCH_CONSUMES_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION" == "true" \
  && "$PUBLIC_LAUNCH_CONSUMES_RENDER_ASSET_ELIGIBILITY" == "true" ]]; then
  PUBLIC_LAUNCH_LOCAL_PLAYABILITY_READY=true
fi

S5_REAL_DEVICE_READY=false
if [[ "$S5_STATUS" == "ready" || "$S5_STATUS" == "real_device_evidence_green" ]]; then
  S5_REAL_DEVICE_READY=true
fi

RELEASE_LATENCY_READY=false
if [[ "$RELEASE_LATENCY_STATUS" == "local_release_latency_drill_green" || "$RELEASE_LATENCY_STATUS" == "multi_node_or_live_traffic_latency_green" ]]; then
  RELEASE_LATENCY_READY=true
fi

ROLLBACK_READY=false
if [[ "$ROLLBACK_BACKUP_STATUS" == "release_rollback_backup_drill_green" ]]; then
  ROLLBACK_READY=true
fi

PUBLIC_DEPLOY_READY=false
if [[ "$PUBLIC_DEPLOY_STATUS" == "local_public_deploy_drill_green" || "$PUBLIC_DEPLOY_STATUS" == "public_network_deploy_green" ]]; then
  PUBLIC_DEPLOY_READY=true
fi

BLOCKERS=()
[[ "$NATIVE_REPLAY_READY" == "true" ]] || BLOCKERS+=("native_bevy_keyboard_replay_contract")
[[ "$PUBLIC_LAUNCH_REPLAY_READY" == "true" ]] || BLOCKERS+=("public_launch_replay_consumption")
[[ "$ACTION_COACH_READY" == "true" ]] || BLOCKERS+=("native_bevy_action_coach_contract")
[[ "$PLAYER_HUD_READY" == "true" ]] || BLOCKERS+=("native_bevy_player_hud_debug_layer_contract")
[[ "$LIVE_SCREENSHOT_READY" == "true" ]] || BLOCKERS+=("native_bevy_live_window_screenshot_sequence_contract")
[[ "$SPRITE_TEXTURE_SAMPLING_READY" == "true" ]] || BLOCKERS+=("native_bevy_sprite_texture_sampling_contract")
[[ "$SAMPLED_TEXTURE_CORRELATION_READY" == "true" ]] || BLOCKERS+=("native_bevy_live_window_sampled_texture_correlation_contract")
[[ "$RENDER_ASSET_ELIGIBILITY_READY" == "true" ]] || BLOCKERS+=("native_bevy_render_asset_eligibility_contract")
[[ "$CEX_ADAPTER_READY" == "true" ]] || BLOCKERS+=("cex_adapter_readiness_contract")
[[ "$PUBLIC_LAUNCH_LOCAL_PLAYABILITY_READY" == "true" ]] || BLOCKERS+=("public_launch_local_playability_consumption")

OVERALL_STATUS="release_signoff_summary_ready_with_public_launch_blockers"
if [[ "$NATIVE_REPLAY_READY" != "true" || "$PUBLIC_LAUNCH_REPLAY_READY" != "true" || "$ACTION_COACH_READY" != "true" || "$PLAYER_HUD_READY" != "true" || "$LIVE_SCREENSHOT_READY" != "true" || "$SPRITE_TEXTURE_SAMPLING_READY" != "true" || "$SAMPLED_TEXTURE_CORRELATION_READY" != "true" || "$RENDER_ASSET_ELIGIBILITY_READY" != "true" || "$CEX_ADAPTER_READY" != "true" || "$PUBLIC_LAUNCH_LOCAL_PLAYABILITY_READY" != "true" ]]; then
  OVERALL_STATUS="release_signoff_summary_blocked_native_bevy_replay"
elif [[ "$PUBLIC_LAUNCH_STATUS" == "ready_for_public_launch_review" ]]; then
  OVERALL_STATUS="release_signoff_summary_green"
fi

SUMMARY_BLOCKERS_JSON="$(printf '%s\n' "${BLOCKERS[@]}" | jq -Rsc 'split("\n") | map(select(length > 0))')"

jq -n \
  --arg contract_version "trillionnium_world_release_signoff_summary_v1" \
  --arg status "$OVERALL_STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg native_replay_evidence "$NATIVE_BEVY_KEYBOARD_REPLAY_EVIDENCE" \
  --arg native_replay_file_status "$NATIVE_REPLAY_FILE_STATUS" \
  --arg native_replay_contract "$NATIVE_REPLAY_CONTRACT" \
  --argjson native_replay_green "$(json_bool "$NATIVE_REPLAY_GREEN")" \
  --arg native_replay_branch_count "$NATIVE_REPLAY_BRANCH_COUNT" \
  --argjson native_replay_ready "$(json_bool "$NATIVE_REPLAY_READY")" \
  --arg force_sequence_count "$FORCE_SEQUENCE_COUNT" \
  --arg agility_sequence_count "$AGILITY_SEQUENCE_COUNT" \
  --arg craft_sequence_count "$CRAFT_SEQUENCE_COUNT" \
  --arg force_final_status "$FORCE_FINAL_STATUS" \
  --arg agility_final_status "$AGILITY_FINAL_STATUS" \
  --arg craft_final_status "$CRAFT_FINAL_STATUS" \
  --arg force_combat_status "$FORCE_COMBAT_STATUS" \
  --arg action_coach_evidence "$NATIVE_BEVY_ACTION_COACH_EVIDENCE" \
  --arg action_coach_file_status "$ACTION_COACH_FILE_STATUS" \
  --arg action_coach_contract "$ACTION_COACH_CONTRACT" \
  --argjson action_coach_green "$(json_bool "$ACTION_COACH_GREEN")" \
  --argjson action_coach_ready "$(json_bool "$ACTION_COACH_READY")" \
  --argjson action_coach_stage_gate "$(json_bool "$ACTION_COACH_STAGE_GATE")" \
  --argjson action_coach_enter_gate "$(json_bool "$ACTION_COACH_ENTER_GATE")" \
  --argjson action_coach_final_next_gate "$(json_bool "$ACTION_COACH_FINAL_NEXT_GATE")" \
  --arg player_hud_evidence "$NATIVE_BEVY_PLAYER_HUD_EVIDENCE" \
  --arg player_hud_file_status "$PLAYER_HUD_FILE_STATUS" \
  --arg player_hud_contract "$PLAYER_HUD_CONTRACT" \
  --argjson player_hud_green "$(json_bool "$PLAYER_HUD_GREEN")" \
  --argjson player_hud_ready "$(json_bool "$PLAYER_HUD_READY")" \
  --argjson player_hud_gate "$(json_bool "$PLAYER_HUD_GATE")" \
  --argjson player_hud_debug_gate "$(json_bool "$PLAYER_HUD_DEBUG_GATE")" \
  --arg live_screenshot_evidence "$NATIVE_BEVY_LIVE_SCREENSHOT_EVIDENCE" \
  --arg live_screenshot_file_status "$LIVE_SCREENSHOT_FILE_STATUS" \
  --arg live_screenshot_contract "$LIVE_SCREENSHOT_CONTRACT" \
  --argjson live_screenshot_green "$(json_bool "$LIVE_SCREENSHOT_GREEN")" \
  --argjson live_screenshot_ready "$(json_bool "$LIVE_SCREENSHOT_READY")" \
  --argjson live_screenshot_frame_sequence_gate "$(json_bool "$LIVE_SCREENSHOT_FRAME_SEQUENCE_GATE")" \
  --argjson live_screenshot_contact_sheet_gate "$(json_bool "$LIVE_SCREENSHOT_CONTACT_SHEET_GATE")" \
  --arg live_screenshot_actual_frame_count "$LIVE_SCREENSHOT_ACTUAL_FRAME_COUNT" \
  --arg sprite_texture_sampling_evidence "$NATIVE_BEVY_SPRITE_TEXTURE_SAMPLING_EVIDENCE" \
  --arg sprite_texture_sampling_file_status "$SPRITE_TEXTURE_SAMPLING_FILE_STATUS" \
  --arg sprite_texture_sampling_contract "$SPRITE_TEXTURE_SAMPLING_CONTRACT" \
  --argjson sprite_texture_sampling_green "$(json_bool "$SPRITE_TEXTURE_SAMPLING_GREEN")" \
  --argjson sprite_texture_sampling_ready "$(json_bool "$SPRITE_TEXTURE_SAMPLING_READY")" \
  --argjson sprite_texture_sampling_four_layer_gate "$(json_bool "$SPRITE_TEXTURE_SAMPLING_FOUR_LAYER_GATE")" \
  --argjson sprite_texture_sampling_nonblank_gate "$(json_bool "$SPRITE_TEXTURE_SAMPLING_NONBLANK_GATE")" \
  --arg sprite_texture_sampling_surface_count "$SPRITE_TEXTURE_SAMPLING_SURFACE_COUNT" \
  --arg sprite_texture_sampling_unique_color_count "$SPRITE_TEXTURE_SAMPLING_UNIQUE_COLOR_COUNT" \
  --arg sampled_texture_correlation_evidence "$NATIVE_BEVY_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION_EVIDENCE" \
  --arg sampled_texture_correlation_file_status "$SAMPLED_TEXTURE_CORRELATION_FILE_STATUS" \
  --arg sampled_texture_correlation_contract "$SAMPLED_TEXTURE_CORRELATION_CONTRACT" \
  --argjson sampled_texture_correlation_green "$(json_bool "$SAMPLED_TEXTURE_CORRELATION_GREEN")" \
  --argjson sampled_texture_correlation_ready "$(json_bool "$SAMPLED_TEXTURE_CORRELATION_READY")" \
  --argjson sampled_texture_correlation_four_layer_gate "$(json_bool "$SAMPLED_TEXTURE_CORRELATION_FOUR_LAYER_GATE")" \
  --arg sampled_texture_correlation_live_frame_count "$SAMPLED_TEXTURE_CORRELATION_LIVE_FRAME_COUNT" \
  --arg sampled_texture_correlation_live_final_frame_color_count "$SAMPLED_TEXTURE_CORRELATION_LIVE_FINAL_FRAME_COLOR_COUNT" \
  --arg render_asset_eligibility_evidence "$NATIVE_BEVY_RENDER_ASSET_ELIGIBILITY_EVIDENCE" \
  --arg render_asset_eligibility_file_status "$RENDER_ASSET_ELIGIBILITY_FILE_STATUS" \
  --arg render_asset_eligibility_contract "$RENDER_ASSET_ELIGIBILITY_CONTRACT" \
  --argjson render_asset_eligibility_green "$(json_bool "$RENDER_ASSET_ELIGIBILITY_GREEN")" \
  --argjson render_asset_eligibility_ready "$(json_bool "$RENDER_ASSET_ELIGIBILITY_READY")" \
  --argjson render_asset_eligibility_usage_gate "$(json_bool "$RENDER_ASSET_ELIGIBILITY_USAGE_GATE")" \
  --argjson render_asset_eligibility_descriptor_gate "$(json_bool "$RENDER_ASSET_ELIGIBILITY_DESCRIPTOR_GATE")" \
  --argjson render_asset_eligibility_atlas_gate "$(json_bool "$RENDER_ASSET_ELIGIBILITY_ATLAS_GATE")" \
  --argjson render_asset_eligibility_reference_gate "$(json_bool "$RENDER_ASSET_ELIGIBILITY_REFERENCE_GATE")" \
  --arg render_asset_eligibility_image_usage_debug "$RENDER_ASSET_ELIGIBILITY_IMAGE_USAGE_DEBUG" \
  --arg render_asset_eligibility_sprite_reference_count "$RENDER_ASSET_ELIGIBILITY_SPRITE_REFERENCE_COUNT" \
  --arg cex_adapter_evidence "$CEX_ADAPTER_EVIDENCE" \
  --arg cex_adapter_file_status "$CEX_ADAPTER_FILE_STATUS" \
  --arg cex_adapter_contract "$CEX_ADAPTER_CONTRACT" \
  --argjson cex_adapter_green "$(json_bool "$CEX_ADAPTER_GREEN")" \
  --arg cex_adapter_status "$CEX_ADAPTER_STATUS" \
  --arg cex_adapter_protocol "$CEX_ADAPTER_PROTOCOL" \
  --arg cex_adapter_domain "$CEX_ADAPTER_DOMAIN" \
  --arg cex_adapter_source_contract "$CEX_ADAPTER_SOURCE_CONTRACT" \
  --arg cex_adapter_route_record_total "$CEX_ADAPTER_ROUTE_RECORD_TOTAL" \
  --arg cex_adapter_world_node_count "$CEX_ADAPTER_WORLD_NODE_COUNT" \
  --argjson cex_adapter_ready "$(json_bool "$CEX_ADAPTER_READY")" \
  --arg public_launch_evidence "$PUBLIC_LAUNCH_EVIDENCE" \
  --arg public_launch_file_status "$PUBLIC_LAUNCH_FILE_STATUS" \
  --arg public_launch_status "$PUBLIC_LAUNCH_STATUS" \
  --argjson public_launch_replay_ready "$(json_bool "$PUBLIC_LAUNCH_REPLAY_READY")" \
  --argjson public_launch_local_playability_ready "$(json_bool "$PUBLIC_LAUNCH_LOCAL_PLAYABILITY_READY")" \
  --argjson public_launch_consumes_action_coach "$(json_bool "$PUBLIC_LAUNCH_CONSUMES_ACTION_COACH")" \
  --argjson public_launch_consumes_player_hud "$(json_bool "$PUBLIC_LAUNCH_CONSUMES_PLAYER_HUD")" \
  --argjson public_launch_consumes_live_screenshot "$(json_bool "$PUBLIC_LAUNCH_CONSUMES_LIVE_SCREENSHOT")" \
  --argjson public_launch_consumes_sprite_texture_sampling "$(json_bool "$PUBLIC_LAUNCH_CONSUMES_SPRITE_TEXTURE_SAMPLING")" \
  --argjson public_launch_consumes_sampled_texture_correlation "$(json_bool "$PUBLIC_LAUNCH_CONSUMES_LIVE_WINDOW_SAMPLED_TEXTURE_CORRELATION")" \
  --argjson public_launch_consumes_render_asset_eligibility "$(json_bool "$PUBLIC_LAUNCH_CONSUMES_RENDER_ASSET_ELIGIBILITY")" \
  --arg s5_evidence "$S5_EVIDENCE" \
  --arg s5_file_status "$S5_FILE_STATUS" \
  --arg s5_status "$S5_STATUS" \
  --argjson s5_real_device_ready "$(json_bool "$S5_REAL_DEVICE_READY")" \
  --arg release_latency_evidence "$RELEASE_LATENCY_EVIDENCE" \
  --arg release_latency_file_status "$RELEASE_LATENCY_FILE_STATUS" \
  --arg release_latency_status "$RELEASE_LATENCY_STATUS" \
  --argjson release_latency_ready "$(json_bool "$RELEASE_LATENCY_READY")" \
  --arg rollback_backup_evidence "$ROLLBACK_BACKUP_EVIDENCE" \
  --arg rollback_backup_file_status "$ROLLBACK_BACKUP_FILE_STATUS" \
  --arg rollback_backup_status "$ROLLBACK_BACKUP_STATUS" \
  --argjson rollback_ready "$(json_bool "$ROLLBACK_READY")" \
  --arg public_deploy_evidence "$PUBLIC_DEPLOY_EVIDENCE" \
  --arg public_deploy_file_status "$PUBLIC_DEPLOY_FILE_STATUS" \
  --arg public_deploy_status "$PUBLIC_DEPLOY_STATUS" \
  --argjson public_deploy_ready "$(json_bool "$PUBLIC_DEPLOY_READY")" \
  --argjson summary_blockers "$SUMMARY_BLOCKERS_JSON" \
  --argjson public_launch_blockers "$PUBLIC_LAUNCH_BLOCKERS_JSON" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_release_signoff_summary",
    signoff_rule: "native_bevy_keyboard_replay_action_coach_player_hud_live_screenshot_texture_sampling_correlation_render_asset_eligibility_and_cex_adapter_readiness_must_be_green_and_public_launch_readiness_must_consume_local_playability_before_release_review",
    public_launch_ready: ($public_launch_status == "ready_for_public_launch_review"),
    android_s5_real_device_claimed: false,
    summary_blockers: $summary_blockers,
    public_launch_blockers: $public_launch_blockers,
    gates: {
      native_bevy_keyboard_replay: {
        evidence_path: $native_replay_evidence,
        file_status: $native_replay_file_status,
        contract_version: $native_replay_contract,
        green: $native_replay_green,
        branch_count: (if $native_replay_branch_count == "" then null else ($native_replay_branch_count | tonumber) end),
        ready_for_release_review: $native_replay_ready,
        proof_scope: "host_side_bevy_runtime_replay_not_android_real_device",
        branches: {
          force: {
            recorded_sequence_count: (if $force_sequence_count == "" then null else ($force_sequence_count | tonumber) end),
            final_objective_status: $force_final_status,
            combat_result_state: $force_combat_status
          },
          agility: {
            recorded_sequence_count: (if $agility_sequence_count == "" then null else ($agility_sequence_count | tonumber) end),
            final_objective_status: $agility_final_status
          },
          craft: {
            recorded_sequence_count: (if $craft_sequence_count == "" then null else ($craft_sequence_count | tonumber) end),
            final_objective_status: $craft_final_status
          }
        }
      },
      public_launch_consumes_replay: {
        evidence_path: $public_launch_evidence,
        file_status: $public_launch_file_status,
        public_launch_status: $public_launch_status,
        ready: $public_launch_replay_ready
      },
      native_bevy_action_coach: {
        evidence_path: $action_coach_evidence,
        file_status: $action_coach_file_status,
        contract_version: $action_coach_contract,
        green: $action_coach_green,
        coach_stage_gate: $action_coach_stage_gate,
        enter_execution_gate: $action_coach_enter_gate,
        final_next_gate: $action_coach_final_next_gate,
        ready_for_release_review: $action_coach_ready,
        proof_scope: "host_side_bevy_runtime_guidance_not_android_real_device"
      },
      native_bevy_player_hud_debug_layer: {
        evidence_path: $player_hud_evidence,
        file_status: $player_hud_file_status,
        contract_version: $player_hud_contract,
        green: $player_hud_green,
        player_hud_gate: $player_hud_gate,
        debug_layer_gate: $player_hud_debug_gate,
        ready_for_release_review: $player_hud_ready,
        proof_scope: "host_side_bevy_hud_layer_not_android_real_device"
      },
      native_bevy_live_window_screenshot_sequence: {
        evidence_path: $live_screenshot_evidence,
        file_status: $live_screenshot_file_status,
        contract_version: $live_screenshot_contract,
        green: $live_screenshot_green,
        frame_sequence_gate: $live_screenshot_frame_sequence_gate,
        contact_sheet_gate: $live_screenshot_contact_sheet_gate,
        actual_frame_count: (if $live_screenshot_actual_frame_count == "" then null else ($live_screenshot_actual_frame_count | tonumber) end),
        ready_for_release_review: $live_screenshot_ready,
        proof_scope: "host_side_live_window_screenshot_sequence_not_android_real_device"
      },
      native_bevy_sprite_texture_sampling: {
        evidence_path: $sprite_texture_sampling_evidence,
        file_status: $sprite_texture_sampling_file_status,
        contract_version: $sprite_texture_sampling_contract,
        green: $sprite_texture_sampling_green,
        four_layer_texture_sampling_gate: $sprite_texture_sampling_four_layer_gate,
        texture_sample_nonblank_gate: $sprite_texture_sampling_nonblank_gate,
        sampled_surface_count: (if $sprite_texture_sampling_surface_count == "" then null else ($sprite_texture_sampling_surface_count | tonumber) end),
        texture_unique_rgba_color_count: (if $sprite_texture_sampling_unique_color_count == "" then null else ($sprite_texture_sampling_unique_color_count | tonumber) end),
        ready_for_release_review: $sprite_texture_sampling_ready,
        proof_scope: "host_side_cpu_texture_sampling_not_gpu_upload_or_android_real_device"
      },
      native_bevy_live_window_sampled_texture_correlation: {
        evidence_path: $sampled_texture_correlation_evidence,
        file_status: $sampled_texture_correlation_file_status,
        contract_version: $sampled_texture_correlation_contract,
        green: $sampled_texture_correlation_green,
        four_layer_sampled_live_correlation_gate: $sampled_texture_correlation_four_layer_gate,
        live_frame_count: (if $sampled_texture_correlation_live_frame_count == "" then null else ($sampled_texture_correlation_live_frame_count | tonumber) end),
        live_final_frame_colors_96x54: (if $sampled_texture_correlation_live_final_frame_color_count == "" then null else ($sampled_texture_correlation_live_final_frame_color_count | tonumber) end),
        ready_for_release_review: $sampled_texture_correlation_ready,
        proof_scope: "host_side_sampled_texture_to_live_window_correlation_not_android_real_device"
      },
      native_bevy_render_asset_eligibility: {
        evidence_path: $render_asset_eligibility_evidence,
        file_status: $render_asset_eligibility_file_status,
        contract_version: $render_asset_eligibility_contract,
        green: $render_asset_eligibility_green,
        render_asset_usage_gate: $render_asset_eligibility_usage_gate,
        image_descriptor_render_eligibility_gate: $render_asset_eligibility_descriptor_gate,
        atlas_layout_render_eligibility_gate: $render_asset_eligibility_atlas_gate,
        sprite_render_reference_gate: $render_asset_eligibility_reference_gate,
        image_asset_usage_debug: $render_asset_eligibility_image_usage_debug,
        sprite_render_reference_count: (if $render_asset_eligibility_sprite_reference_count == "" then null else ($render_asset_eligibility_sprite_reference_count | tonumber) end),
        ready_for_release_review: $render_asset_eligibility_ready,
        proof_scope: "host_side_render_asset_eligibility_not_render_world_extraction_or_gpu_upload"
      },
      cex_adapter_readiness: {
        evidence_path: $cex_adapter_evidence,
        file_status: $cex_adapter_file_status,
        contract_version: $cex_adapter_contract,
        green: $cex_adapter_green,
        status: $cex_adapter_status,
        source_contract_version: $cex_adapter_source_contract,
        protocol_contract: $cex_adapter_protocol,
        domain_contract: $cex_adapter_domain,
        route_record_total: (if $cex_adapter_route_record_total == "" then null else ($cex_adapter_route_record_total | tonumber) end),
        world_node_count: (if $cex_adapter_world_node_count == "" then null else ($cex_adapter_world_node_count | tonumber) end),
        ready_for_release_review: $cex_adapter_ready,
        proof_scope: "cex_incubator_runtime_adapter_json_evidence_not_public_launch_external_evidence"
      },
      public_launch_consumes_local_playability: {
        evidence_path: $public_launch_evidence,
        file_status: $public_launch_file_status,
        public_launch_status: $public_launch_status,
        action_coach: $public_launch_consumes_action_coach,
        player_hud_debug_layer: $public_launch_consumes_player_hud,
        live_window_screenshot_sequence: $public_launch_consumes_live_screenshot,
        sprite_texture_sampling: $public_launch_consumes_sprite_texture_sampling,
        live_window_sampled_texture_correlation: $public_launch_consumes_sampled_texture_correlation,
        render_asset_eligibility: $public_launch_consumes_render_asset_eligibility,
        ready: $public_launch_local_playability_ready
      },
      s5_real_device_matrix: {
        evidence_path: $s5_evidence,
        file_status: $s5_file_status,
        status: $s5_status,
        ready: $s5_real_device_ready,
        required_before_public_launch_ready: true
      },
      release_latency: {
        evidence_path: $release_latency_evidence,
        file_status: $release_latency_file_status,
        status: $release_latency_status,
        ready: $release_latency_ready,
        local_drill_is_not_multi_node_or_live_traffic: ($release_latency_status == "local_release_latency_drill_green")
      },
      release_rollback_backup: {
        evidence_path: $rollback_backup_evidence,
        file_status: $rollback_backup_file_status,
        status: $rollback_backup_status,
        ready: $rollback_ready
      },
      public_deploy: {
        evidence_path: $public_deploy_evidence,
        file_status: $public_deploy_file_status,
        status: $public_deploy_status,
        ready: $public_deploy_ready,
        local_drill_is_not_public_network_exposure: ($public_deploy_status == "local_public_deploy_drill_green")
      }
    },
    reviewer_shortlist: [
      "native_bevy_keyboard_replay",
      "native_bevy_action_coach",
      "native_bevy_player_hud_debug_layer",
      "native_bevy_live_window_screenshot_sequence",
      "native_bevy_sprite_texture_sampling",
      "native_bevy_live_window_sampled_texture_correlation",
      "native_bevy_render_asset_eligibility",
      "cex_adapter_readiness",
      "public_launch_consumes_replay",
      "public_launch_consumes_local_playability",
      "s5_real_device_matrix",
      "release_latency",
      "release_rollback_backup",
      "public_deploy"
    ]
  }' >"$SUMMARY_FILE"

case "$OVERALL_STATUS" in
  release_signoff_summary_green)
    printf 'TRILLIONNIUM_WORLD_RELEASE_SIGNOFF_SUMMARY_GREEN %s\n' "$SUMMARY_FILE"
    ;;
  release_signoff_summary_ready_with_public_launch_blockers)
    printf 'TRILLIONNIUM_WORLD_RELEASE_SIGNOFF_SUMMARY_BLOCKED_PUBLIC_LAUNCH %s\n' "$SUMMARY_FILE"
    ;;
  *)
    printf 'TRILLIONNIUM_WORLD_RELEASE_SIGNOFF_SUMMARY_BLOCKED %s %s\n' "$OVERALL_STATUS" "$SUMMARY_FILE"
    ;;
esac

if [[ "$REQUIRE_READY" -eq 1 && "$OVERALL_STATUS" != "release_signoff_summary_green" ]]; then
  exit 1
fi
