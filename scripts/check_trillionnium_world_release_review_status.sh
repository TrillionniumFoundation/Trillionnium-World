#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
STATUS_JSON="$ACCEPTANCE_DIR/release-review-status.json"
STATUS_MD="$ACCEPTANCE_DIR/release-review-status.md"
QUICKCHECK_SUMMARY="$ACCEPTANCE_DIR/release-review-quickcheck.json"
SIGNOFF_SUMMARY="$ACCEPTANCE_DIR/release-signoff-summary.json"
QUICKCHECK_LOG="$ACCEPTANCE_DIR/release-review-status-quickcheck.log"
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

"$ROOT/scripts/check_trillionnium_world_release_review_quickcheck.sh" >"$QUICKCHECK_LOG"

jq -n \
  --arg contract_version "trillionnium_world_release_review_status_v1" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg quickcheck_summary "$QUICKCHECK_SUMMARY" \
  --arg signoff_summary "$SIGNOFF_SUMMARY" \
  --arg quickcheck_log "$QUICKCHECK_LOG" \
  --arg markdown_path "$STATUS_MD" \
  --slurpfile quick "$QUICKCHECK_SUMMARY" \
  --slurpfile signoff "$SIGNOFF_SUMMARY" \
  '
  def blocker_detail($id):
    if $id == "s5_real_device_matrix" then {
      id: $id,
      label: "S5 Android real-device matrix",
      needed: "Connect an Android device and collect launch, screenshot, gfxinfo/frame, CJK/input, lifecycle, weak-network, APK resource/signature, and crash-free logcat evidence."
    } elif $id == "production_map_pack_public_evidence" then {
      id: $id,
      label: "Production map-pack public evidence",
      needed: "Provide production/public map-pack ready evidence, not only the local route or fixture-signed manifest."
    } elif $id == "first_beta_cohort_evidence" then {
      id: $id,
      label: "First beta cohort evidence",
      needed: "Attach real 5-10 participant cohort evidence with status first_beta_cohort_evidence_green."
    } elif $id == "commercial_launch_drill_evidence" then {
      id: $id,
      label: "Commercial launch drill evidence",
      needed: "Attach real or sanitized payment, refund, support, legal, operator, and traffic drill evidence."
    } elif $id == "multi_node_or_live_traffic_latency_evidence" then {
      id: $id,
      label: "Multi-node or live-traffic latency evidence",
      needed: "Provide multi-node release latency or live public traffic latency evidence; local latency drill is not enough."
    } elif $id == "public_network_live_exposure_evidence" then {
      id: $id,
      label: "Public network live exposure evidence",
      needed: "Provide approved host, domain/TLS, monitoring, backup, rollback, and public URL probe evidence."
    } else {
      id: $id,
      label: $id,
      needed: "Provide the missing public-launch evidence required by public-launch readiness."
    } end;

  ($quick[0]) as $q |
  ($signoff[0]) as $s |
  ($q.public_launch_blockers // []) as $blockers |
  (if ($q.public_launch_ready == true) then
    "public_launch_ready_for_review"
  elif ($q.ready_for_release_review == true) then
    "release_review_ready_public_launch_blocked"
  else
    "blocked_native_bevy_replay_or_public_launch_consumption"
  end) as $status |
  {
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_release_review_status",
    quickcheck_summary: $quickcheck_summary,
    signoff_summary: $signoff_summary,
    quickcheck_log: $quickcheck_log,
    markdown_path: $markdown_path,
    ready_for_release_review: ($q.ready_for_release_review == true),
    public_launch_ready: ($q.public_launch_ready == true),
    android_s5_real_device_claimed: false,
    boundary: {
      native_bevy_replay_scope: "host_side_bevy_runtime_replay_not_android_real_device",
      native_bevy_texture_render_scope: "host_side_texture_sampling_correlation_and_render_asset_eligibility_not_gpu_upload_or_android_real_device",
      public_launch_claim: "blocked_until_real_external_evidence_is_attached"
    },
    ready_items: [
      {
        id: "native_bevy_keyboard_replay",
        label: "Native/Bevy keyboard replay",
        ready: ($q.gates.native_bevy_keyboard_replay_ready == true),
        evidence_path: $s.gates.native_bevy_keyboard_replay.evidence_path,
        detail: ("force=" + (($s.gates.native_bevy_keyboard_replay.branches.force.recorded_sequence_count // 0) | tostring) + ", agility=" + (($s.gates.native_bevy_keyboard_replay.branches.agility.recorded_sequence_count // 0) | tostring) + ", craft=" + (($s.gates.native_bevy_keyboard_replay.branches.craft.recorded_sequence_count // 0) | tostring) + "; force combat=" + ($s.gates.native_bevy_keyboard_replay.branches.force.combat_result_state // "unknown"))
      },
      {
        id: "native_bevy_action_coach",
        label: "Native/Bevy action coach",
        ready: ($q.gates.native_bevy_action_coach_ready == true),
        evidence_path: $s.gates.native_bevy_action_coach.evidence_path,
        detail: ("coach_stage=" + (($s.gates.native_bevy_action_coach.coach_stage_gate // false) | tostring) + ", enter_execution=" + (($s.gates.native_bevy_action_coach.enter_execution_gate // false) | tostring) + ", final_next=" + (($s.gates.native_bevy_action_coach.final_next_gate // false) | tostring))
      },
      {
        id: "native_bevy_player_hud_debug_layer",
        label: "Native/Bevy player HUD/debug layer",
        ready: ($q.gates.native_bevy_player_hud_debug_layer_ready == true),
        evidence_path: $s.gates.native_bevy_player_hud_debug_layer.evidence_path,
        detail: ("player_hud=" + (($s.gates.native_bevy_player_hud_debug_layer.player_hud_gate // false) | tostring) + ", debug_layer=" + (($s.gates.native_bevy_player_hud_debug_layer.debug_layer_gate // false) | tostring))
      },
      {
        id: "native_bevy_live_window_screenshot_sequence",
        label: "Native/Bevy live-window screenshot sequence",
        ready: ($q.gates.native_bevy_live_window_screenshot_sequence_ready == true),
        evidence_path: $s.gates.native_bevy_live_window_screenshot_sequence.evidence_path,
        detail: ("frames=" + (($s.gates.native_bevy_live_window_screenshot_sequence.actual_frame_count // 0) | tostring) + ", sequence=" + (($s.gates.native_bevy_live_window_screenshot_sequence.frame_sequence_gate // false) | tostring) + ", contact_sheet=" + (($s.gates.native_bevy_live_window_screenshot_sequence.contact_sheet_gate // false) | tostring))
      },
      {
        id: "native_bevy_sprite_texture_sampling",
        label: "Native/Bevy sprite texture sampling",
        ready: ($q.gates.native_bevy_sprite_texture_sampling_ready == true),
        evidence_path: $s.gates.native_bevy_sprite_texture_sampling.evidence_path,
        detail: ("sampled_surfaces=" + (($s.gates.native_bevy_sprite_texture_sampling.sampled_surface_count // 0) | tostring) + ", unique_rgba=" + (($s.gates.native_bevy_sprite_texture_sampling.texture_unique_rgba_color_count // 0) | tostring) + ", four_layer=" + (($s.gates.native_bevy_sprite_texture_sampling.four_layer_texture_sampling_gate // false) | tostring))
      },
      {
        id: "native_bevy_live_window_sampled_texture_correlation",
        label: "Native/Bevy sampled texture live-window correlation",
        ready: ($q.gates.native_bevy_live_window_sampled_texture_correlation_ready == true),
        evidence_path: $s.gates.native_bevy_live_window_sampled_texture_correlation.evidence_path,
        detail: ("live_frames=" + (($s.gates.native_bevy_live_window_sampled_texture_correlation.live_frame_count // 0) | tostring) + ", final_frame_colors=" + (($s.gates.native_bevy_live_window_sampled_texture_correlation.live_final_frame_colors_96x54 // 0) | tostring) + ", four_layer=" + (($s.gates.native_bevy_live_window_sampled_texture_correlation.four_layer_sampled_live_correlation_gate // false) | tostring))
      },
     {
       id: "native_bevy_render_asset_eligibility",
        label: "Native/Bevy render asset eligibility",
        ready: ($q.gates.native_bevy_render_asset_eligibility_ready == true),
        evidence_path: $s.gates.native_bevy_render_asset_eligibility.evidence_path,
       detail: ("usage=" + ($s.gates.native_bevy_render_asset_eligibility.image_asset_usage_debug // "unknown") + ", sprite_refs=" + (($s.gates.native_bevy_render_asset_eligibility.sprite_render_reference_count // 0) | tostring) + ", render_usage=" + (($s.gates.native_bevy_render_asset_eligibility.render_asset_usage_gate // false) | tostring))
     },
      {
        id: "cex_adapter_readiness",
        label: "CEX production world adapter readiness",
        ready: ($q.gates.cex_adapter_readiness_ready == true),
        evidence_path: $s.gates.cex_adapter_readiness.evidence_path,
        detail: ("routes=" + (($s.gates.cex_adapter_readiness.route_record_total // 0) | tostring) + ", nodes=" + (($s.gates.cex_adapter_readiness.world_node_count // 0) | tostring) + ", protocol=" + ($s.gates.cex_adapter_readiness.protocol_contract // "unknown"))
      },
     {
       id: "public_launch_consumes_replay",
        label: "Public launch consumes replay gate",
        ready: ($q.gates.public_launch_consumes_replay == true),
        evidence_path: $q.refreshed_evidence.public_launch_readiness.summary_path,
        detail: ($q.refreshed_evidence.public_launch_readiness.status // "unknown")
      },
      {
        id: "public_launch_consumes_local_playability",
        label: "Public launch consumes local playability gates",
        ready: ($q.gates.public_launch_consumes_local_playability == true),
        evidence_path: $q.refreshed_evidence.public_launch_readiness.summary_path,
        detail: ($q.refreshed_evidence.public_launch_readiness.status // "unknown")
      },
      {
        id: "release_latency_local_drill",
        label: "Release latency local drill",
        ready: ($q.gates.release_latency_ready == true),
        evidence_path: $s.gates.release_latency.evidence_path,
        detail: ($s.gates.release_latency.status // "unknown")
      },
      {
        id: "release_rollback_backup_drill",
        label: "Release rollback/backup drill",
        ready: ($q.gates.release_rollback_backup_ready == true),
        evidence_path: $s.gates.release_rollback_backup.evidence_path,
        detail: ($s.gates.release_rollback_backup.status // "unknown")
      },
      {
        id: "public_deploy_local_drill",
        label: "Public deploy local drill",
        ready: ($q.gates.public_deploy_ready == true),
        evidence_path: $s.gates.public_deploy.evidence_path,
        detail: ($s.gates.public_deploy.status // "unknown")
      }
    ],
    blocked_items: ($blockers | map(blocker_detail(.))),
    reviewer_next_action: (if ($q.public_launch_ready == true) then "review_public_launch_ready_evidence" elif ($q.ready_for_release_review == true) then "collect_real_external_public_launch_evidence" else "fix_native_bevy_replay_or_public_launch_consumption_chain" end)
  }
  ' >"$STATUS_JSON"

STATUS="$(jq -r '.status' "$STATUS_JSON")"
READY_FOR_REVIEW="$(jq -r '.ready_for_release_review' "$STATUS_JSON")"
PUBLIC_LAUNCH_READY="$(jq -r '.public_launch_ready' "$STATUS_JSON")"
GENERATED_AT="$(jq -r '.generated_at' "$STATUS_JSON")"

{
  printf '# Trillionnium World Release Review Status\n\n'
  printf -- '- generated_at: `%s`\n' "$GENERATED_AT"
  printf -- '- status: `%s`\n' "$STATUS"
  printf -- '- ready_for_release_review: `%s`\n' "$READY_FOR_REVIEW"
  printf -- '- public_launch_ready: `%s`\n' "$PUBLIC_LAUNCH_READY"
  printf -- '- android_s5_real_device_claimed: `false`\n\n'
  printf '## Green For Review\n\n'
  jq -r '.ready_items[] | "- [\(.ready | if . then "x" else " " end)] \(.label): \(.detail)"' "$STATUS_JSON"
  printf '\n## Still Requires Real External Evidence\n\n'
  jq -r 'if (.blocked_items | length) == 0 then "- [x] No public-launch blockers remain." else .blocked_items[] | "- [ ] \(.label): \(.needed)" end' "$STATUS_JSON"
  printf '\n## Boundary\n\n'
  printf -- '- Native/Bevy keyboard replay, classic player motion, action coach, HUD/debug layer, player UI rescue, live screenshots, sprite texture sampling, sampled texture live-window correlation, and render asset eligibility are host-side proof, not Android real-device proof.\n'
  printf -- '- CEX adapter readiness is incubator runtime adapter evidence, not real external public-launch evidence.\n'
  printf -- '- Public launch remains blocked until the external evidence above is attached.\n'
} >"$STATUS_MD"

case "$STATUS" in
  public_launch_ready_for_review)
    printf 'TRILLIONNIUM_WORLD_RELEASE_REVIEW_STATUS_PUBLIC_LAUNCH_READY %s %s\n' "$STATUS_JSON" "$STATUS_MD"
    ;;
  release_review_ready_public_launch_blocked)
    printf 'TRILLIONNIUM_WORLD_RELEASE_REVIEW_STATUS_READY_WITH_BLOCKERS %s %s\n' "$STATUS_JSON" "$STATUS_MD"
    ;;
  *)
    printf 'TRILLIONNIUM_WORLD_RELEASE_REVIEW_STATUS_BLOCKED %s %s %s\n' "$STATUS" "$STATUS_JSON" "$STATUS_MD" >&2
    exit 1
    ;;
esac

if [[ "$REQUIRE_READY" -eq 1 && "$STATUS" != "public_launch_ready_for_review" ]]; then
  exit 1
fi
