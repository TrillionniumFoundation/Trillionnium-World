#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
SUMMARY_FILE="$ACCEPTANCE_DIR/release-review-quickcheck.json"
if [[ -v TRILLIONNIUM_WORLD_RELEASE_REVIEW_QUICKCHECK_SUMMARY && -n "$TRILLIONNIUM_WORLD_RELEASE_REVIEW_QUICKCHECK_SUMMARY" ]]; then
  SUMMARY_FILE="$TRILLIONNIUM_WORLD_RELEASE_REVIEW_QUICKCHECK_SUMMARY"
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

PUBLIC_LAUNCH_SUMMARY="$ACCEPTANCE_DIR/public-launch-readiness.json"
SIGNOFF_SUMMARY="$ACCEPTANCE_DIR/release-signoff-summary.json"
PUBLIC_LAUNCH_LOG="$ACCEPTANCE_DIR/release-review-public-launch-readiness.log"
SIGNOFF_LOG="$ACCEPTANCE_DIR/release-review-signoff-summary.log"

TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_SUMMARY="$PUBLIC_LAUNCH_SUMMARY" \
  "$ROOT/scripts/check_trillionnium_world_public_launch_readiness.sh" >"$PUBLIC_LAUNCH_LOG"
TRILLIONNIUM_WORLD_RELEASE_SIGNOFF_SUMMARY="$SIGNOFF_SUMMARY" \
  "$ROOT/scripts/check_trillionnium_world_release_signoff_summary.sh" >"$SIGNOFF_LOG"

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

PUBLIC_LAUNCH_STATUS="$(read_json_field "$PUBLIC_LAUNCH_SUMMARY" '.overall_status')"
SIGNOFF_STATUS="$(read_json_field "$SIGNOFF_SUMMARY" '.status')"
NATIVE_REPLAY_READY="$(read_json_field "$SIGNOFF_SUMMARY" '.gates.native_bevy_keyboard_replay.ready_for_release_review')"
PUBLIC_LAUNCH_CONSUMES_REPLAY="$(read_json_field "$SIGNOFF_SUMMARY" '.gates.public_launch_consumes_replay.ready')"
ACTION_COACH_READY="$(read_json_field "$SIGNOFF_SUMMARY" '.gates.native_bevy_action_coach.ready_for_release_review')"
PLAYER_HUD_READY="$(read_json_field "$SIGNOFF_SUMMARY" '.gates.native_bevy_player_hud_debug_layer.ready_for_release_review')"
LIVE_SCREENSHOT_READY="$(read_json_field "$SIGNOFF_SUMMARY" '.gates.native_bevy_live_window_screenshot_sequence.ready_for_release_review')"
SPRITE_TEXTURE_SAMPLING_READY="$(read_json_field "$SIGNOFF_SUMMARY" '.gates.native_bevy_sprite_texture_sampling.ready_for_release_review')"
SAMPLED_TEXTURE_CORRELATION_READY="$(read_json_field "$SIGNOFF_SUMMARY" '.gates.native_bevy_live_window_sampled_texture_correlation.ready_for_release_review')"
RENDER_ASSET_ELIGIBILITY_READY="$(read_json_field "$SIGNOFF_SUMMARY" '.gates.native_bevy_render_asset_eligibility.ready_for_release_review')"
CEX_ADAPTER_READY="$(read_json_field "$SIGNOFF_SUMMARY" '.gates.cex_adapter_readiness.ready_for_release_review')"
PUBLIC_LAUNCH_CONSUMES_LOCAL_PLAYABILITY="$(read_json_field "$SIGNOFF_SUMMARY" '.gates.public_launch_consumes_local_playability.ready')"
S5_REAL_DEVICE_READY="$(read_json_field "$SIGNOFF_SUMMARY" '.gates.s5_real_device_matrix.ready')"
RELEASE_LATENCY_READY="$(read_json_field "$SIGNOFF_SUMMARY" '.gates.release_latency.ready')"
ROLLBACK_READY="$(read_json_field "$SIGNOFF_SUMMARY" '.gates.release_rollback_backup.ready')"
PUBLIC_DEPLOY_READY="$(read_json_field "$SIGNOFF_SUMMARY" '.gates.public_deploy.ready')"
if [[ -f "$PUBLIC_LAUNCH_SUMMARY" ]]; then
  PUBLIC_LAUNCH_BLOCKERS_JSON="$(jq -c '.blockers // []' "$PUBLIC_LAUNCH_SUMMARY")"
else
  PUBLIC_LAUNCH_BLOCKERS_JSON='["public_launch_readiness_summary"]'
fi
if [[ -f "$SIGNOFF_SUMMARY" ]]; then
  SIGNOFF_SUMMARY_BLOCKERS_JSON="$(jq -c '.summary_blockers // []' "$SIGNOFF_SUMMARY")"
else
  SIGNOFF_SUMMARY_BLOCKERS_JSON='["release_signoff_summary"]'
fi

STATUS="release_review_quickcheck_blocked_native_bevy_replay"
if [[ "$NATIVE_REPLAY_READY" == "true" \
  && "$PUBLIC_LAUNCH_CONSUMES_REPLAY" == "true" \
  && "$ACTION_COACH_READY" == "true" \
  && "$PLAYER_HUD_READY" == "true" \
  && "$LIVE_SCREENSHOT_READY" == "true" \
  && "$SPRITE_TEXTURE_SAMPLING_READY" == "true" \
  && "$SAMPLED_TEXTURE_CORRELATION_READY" == "true" \
  && "$RENDER_ASSET_ELIGIBILITY_READY" == "true" \
  && "$CEX_ADAPTER_READY" == "true" \
  && "$PUBLIC_LAUNCH_CONSUMES_LOCAL_PLAYABILITY" == "true" ]]; then
  if [[ "$PUBLIC_LAUNCH_STATUS" == "ready_for_public_launch_review" && "$SIGNOFF_STATUS" == "release_signoff_summary_green" ]]; then
    STATUS="release_review_quickcheck_green"
  else
    STATUS="release_review_quickcheck_green_with_public_launch_blockers"
  fi
fi

GATE_COUNT=14
READY_GATE_COUNT=0
for gate_ready in \
  "$NATIVE_REPLAY_READY" \
  "$PUBLIC_LAUNCH_CONSUMES_REPLAY" \
  "$ACTION_COACH_READY" \
  "$PLAYER_HUD_READY" \
  "$LIVE_SCREENSHOT_READY" \
  "$SPRITE_TEXTURE_SAMPLING_READY" \
  "$SAMPLED_TEXTURE_CORRELATION_READY" \
  "$RENDER_ASSET_ELIGIBILITY_READY" \
  "$CEX_ADAPTER_READY" \
  "$PUBLIC_LAUNCH_CONSUMES_LOCAL_PLAYABILITY" \
  "$S5_REAL_DEVICE_READY" \
  "$RELEASE_LATENCY_READY" \
  "$ROLLBACK_READY" \
  "$PUBLIC_DEPLOY_READY"; do
  if [[ "$gate_ready" == "true" ]]; then
    READY_GATE_COUNT=$((READY_GATE_COUNT + 1))
  fi
done
BLOCKED_GATE_COUNT=$((GATE_COUNT - READY_GATE_COUNT))
SIGNOFF_SUMMARY_BLOCKER_COUNT="$(jq 'length' <<<"$SIGNOFF_SUMMARY_BLOCKERS_JSON")"
PUBLIC_LAUNCH_BLOCKER_COUNT="$(jq 'length' <<<"$PUBLIC_LAUNCH_BLOCKERS_JSON")"
GREEN=true
if [[ "$STATUS" == "release_review_quickcheck_blocked_native_bevy_replay" ]]; then
  GREEN=false
fi

jq -n \
  --arg contract_version "trillionnium_world_release_review_quickcheck_v1" \
  --arg status "$STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --argjson green "$(json_bool "$GREEN")" \
  --argjson gate_count "$GATE_COUNT" \
  --argjson ready_gate_count "$READY_GATE_COUNT" \
  --argjson blocked_gate_count "$BLOCKED_GATE_COUNT" \
  --argjson signoff_summary_blocker_count "$SIGNOFF_SUMMARY_BLOCKER_COUNT" \
  --argjson public_launch_blocker_count "$PUBLIC_LAUNCH_BLOCKER_COUNT" \
  --arg public_launch_summary "$PUBLIC_LAUNCH_SUMMARY" \
  --arg public_launch_log "$PUBLIC_LAUNCH_LOG" \
  --arg public_launch_status "$PUBLIC_LAUNCH_STATUS" \
  --arg signoff_summary "$SIGNOFF_SUMMARY" \
  --arg signoff_log "$SIGNOFF_LOG" \
  --arg signoff_status "$SIGNOFF_STATUS" \
  --argjson native_replay_ready "$(json_bool "$NATIVE_REPLAY_READY")" \
  --argjson public_launch_consumes_replay "$(json_bool "$PUBLIC_LAUNCH_CONSUMES_REPLAY")" \
  --argjson action_coach_ready "$(json_bool "$ACTION_COACH_READY")" \
  --argjson player_hud_ready "$(json_bool "$PLAYER_HUD_READY")" \
  --argjson live_screenshot_ready "$(json_bool "$LIVE_SCREENSHOT_READY")" \
  --argjson sprite_texture_sampling_ready "$(json_bool "$SPRITE_TEXTURE_SAMPLING_READY")" \
  --argjson sampled_texture_correlation_ready "$(json_bool "$SAMPLED_TEXTURE_CORRELATION_READY")" \
  --argjson render_asset_eligibility_ready "$(json_bool "$RENDER_ASSET_ELIGIBILITY_READY")" \
  --argjson cex_adapter_ready "$(json_bool "$CEX_ADAPTER_READY")" \
  --argjson public_launch_consumes_local_playability "$(json_bool "$PUBLIC_LAUNCH_CONSUMES_LOCAL_PLAYABILITY")" \
  --argjson s5_real_device_ready "$(json_bool "$S5_REAL_DEVICE_READY")" \
  --argjson release_latency_ready "$(json_bool "$RELEASE_LATENCY_READY")" \
  --argjson rollback_ready "$(json_bool "$ROLLBACK_READY")" \
  --argjson public_deploy_ready "$(json_bool "$PUBLIC_DEPLOY_READY")" \
  --argjson public_launch_blockers "$PUBLIC_LAUNCH_BLOCKERS_JSON" \
  --argjson signoff_summary_blockers "$SIGNOFF_SUMMARY_BLOCKERS_JSON" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_release_review_quickcheck",
    quickcheck_rule: "refresh_public_launch_readiness_then_release_signoff_summary_and_fail_only_when_native_bevy_local_playability_texture_sampling_render_asset_eligibility_cex_adapter_readiness_or_consumption_is_broken_unless_require_ready_is_set",
    green: $green,
    ready_for_release_review: ($native_replay_ready and $public_launch_consumes_replay and $action_coach_ready and $player_hud_ready and $live_screenshot_ready and $sprite_texture_sampling_ready and $sampled_texture_correlation_ready and $render_asset_eligibility_ready and $cex_adapter_ready and $public_launch_consumes_local_playability),
    public_launch_ready: ($public_launch_status == "ready_for_public_launch_review"),
    android_s5_real_device_claimed: false,
    gate_count: $gate_count,
    ready_gate_count: $ready_gate_count,
    blocked_gate_count: $blocked_gate_count,
    signoff_summary_blocker_count: $signoff_summary_blocker_count,
    public_launch_blocker_count: $public_launch_blocker_count,
    refreshed_evidence: {
      public_launch_readiness: {
        summary_path: $public_launch_summary,
        log_path: $public_launch_log,
        status: $public_launch_status
      },
      release_signoff_summary: {
        summary_path: $signoff_summary,
        log_path: $signoff_log,
        status: $signoff_status
      }
    },
    gates: {
      native_bevy_keyboard_replay_ready: $native_replay_ready,
      public_launch_consumes_replay: $public_launch_consumes_replay,
      native_bevy_action_coach_ready: $action_coach_ready,
      native_bevy_player_hud_debug_layer_ready: $player_hud_ready,
      native_bevy_live_window_screenshot_sequence_ready: $live_screenshot_ready,
      native_bevy_sprite_texture_sampling_ready: $sprite_texture_sampling_ready,
     native_bevy_live_window_sampled_texture_correlation_ready: $sampled_texture_correlation_ready,
     native_bevy_render_asset_eligibility_ready: $render_asset_eligibility_ready,
      cex_adapter_readiness_ready: $cex_adapter_ready,
     public_launch_consumes_local_playability: $public_launch_consumes_local_playability,
      s5_real_device_ready: $s5_real_device_ready,
      release_latency_ready: $release_latency_ready,
      release_rollback_backup_ready: $rollback_ready,
      public_deploy_ready: $public_deploy_ready
    },
    signoff_summary_blockers: $signoff_summary_blockers,
    public_launch_blockers: $public_launch_blockers
  }' >"$SUMMARY_FILE"

case "$STATUS" in
  release_review_quickcheck_green)
    printf 'TRILLIONNIUM_WORLD_RELEASE_REVIEW_QUICKCHECK_GREEN %s\n' "$SUMMARY_FILE"
    ;;
  release_review_quickcheck_green_with_public_launch_blockers)
    printf 'TRILLIONNIUM_WORLD_RELEASE_REVIEW_QUICKCHECK_GREEN_WITH_PUBLIC_LAUNCH_BLOCKERS %s\n' "$SUMMARY_FILE"
    ;;
  *)
    printf 'TRILLIONNIUM_WORLD_RELEASE_REVIEW_QUICKCHECK_BLOCKED %s %s\n' "$STATUS" "$SUMMARY_FILE" >&2
    exit 1
    ;;
esac

if [[ "$REQUIRE_READY" -eq 1 && "$STATUS" != "release_review_quickcheck_green" ]]; then
  exit 1
fi
