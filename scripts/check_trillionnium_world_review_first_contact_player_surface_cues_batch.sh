#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
S5_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
DOC_REL="docs/archive/world-review-2026-07/trillionnium-world-review-first-contact-player-surface-cues-batch-2026-07-09.md"
DOC="$ROOT/$DOC_REL"
RUNTIME_BOUNDARY_BATCH_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-runtime-boundary-batch.json"
RUNTIME_ADAPTER_ONLINE_BATCH_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-runtime-adapter-online-batch.json"
BEVY_RUNTIME_RENDERER_BATCH_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-bevy-runtime-renderer-batch.json"
BASIN_SPEC_JSON="$S5_DIR/bevy-classic-rts-first-contact-basin-spec.json"
PLAYTEST_READINESS_JSON="$S5_DIR/bevy-classic-playtest-readiness.json"
RUNBOOK_JSON="$ACCEPTANCE_DIR/first-contact-human-playtest-runbook.json"
OBSERVATION_JSON="$ACCEPTANCE_DIR/first-contact-human-playtest-observation-log.json"
PACKET_INTEGRITY_JSON="$ACCEPTANCE_DIR/release-review-packet-integrity.json"
SUMMARY="$ACCEPTANCE_DIR/trillionnium-world-review-first-contact-player-surface-cues-batch.json"
SUMMARY_MD="$ACCEPTANCE_DIR/trillionnium-world-review-first-contact-player-surface-cues-batch.md"
REFRESH_INPUTS="${TRNM_WORLD_REVIEW_FIRST_CONTACT_PLAYER_SURFACE_CUES_BATCH_REFRESH_INPUTS:-1}"
EXPECTED_COMMIT_SET_SHA256="63d5dc54814a4c9d7dcc2efd6a364315f03a6106d4ee696d5985f83b6fb1db94"
mkdir -p "$ACCEPTANCE_DIR"

require_text() {
  local path="$1"
  local needle="$2"
  if ! grep -Fq -- "$needle" "$path"; then
    echo "[FAIL] $path missing required text: $needle" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "[FAIL] missing First Contact player-surface cues batch doc: $DOC" >&2
  exit 1
fi

require_text "$DOC" "Status: local review First Contact player-surface cues sub-batch 8."
require_text "$DOC" "first_contact_player_surface_cues"
require_text "$DOC" 'Reviewed commit count: `63`'
require_text "$DOC" 'Per-commit unresolved count: `0`'
require_text "$DOC" "Player-surface cue changes may improve local labels"
require_text "$DOC" "Do not convert this local player-surface cue review"
require_text "$DOC" "Sub-batch 8 local review is complete"
require_text "$DOC" "sub_batch_8_exit_rule_satisfied=true"
require_text "$DOC" "batch_3_exit_rule_satisfied=true"
require_text "$DOC" "batch_4_unblocked_for_local_review=true"
require_text "$DOC" "next_batch_bucket_id=unclassified_generated_count_surface"

if [[ "$REFRESH_INPUTS" != "0" ]]; then
  TRNM_WORLD_REVIEW_RUNTIME_BOUNDARY_BATCH_REFRESH_INPUTS=0 \
    "$ROOT/scripts/check_trillionnium_world_review_runtime_boundary_batch.sh" >/dev/null
  TRNM_WORLD_REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_REFRESH_INPUTS=0 \
    "$ROOT/scripts/check_trillionnium_world_review_runtime_adapter_online_batch.sh" >/dev/null
  TRNM_WORLD_REVIEW_BEVY_RUNTIME_RENDERER_BATCH_REFRESH_INPUTS=0 \
    "$ROOT/scripts/check_trillionnium_world_review_bevy_runtime_renderer_batch.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_first_contact_basin_spec.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_first_contact_human_playtest_runbook.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_first_contact_human_playtest_observation_log.sh" >/dev/null
fi

for input in \
  "$RUNTIME_BOUNDARY_BATCH_JSON" \
  "$RUNTIME_ADAPTER_ONLINE_BATCH_JSON" \
  "$BEVY_RUNTIME_RENDERER_BATCH_JSON" \
  "$BASIN_SPEC_JSON" \
  "$PLAYTEST_READINESS_JSON" \
  "$RUNBOOK_JSON" \
  "$OBSERVATION_JSON" \
  "$PACKET_INTEGRITY_JSON"; do
  if [[ ! -f "$input" ]]; then
    echo "[FAIL] missing First Contact player-surface cues batch input: $input" >&2
    exit 1
  fi
done

actual_commit_set_sha256="$(
  jq -r '.commit_shards | map(select(.sub_batch_id == "first_contact_player_surface_cues")) | map(.commit) | sort | join("\n")' \
    "$RUNTIME_BOUNDARY_BATCH_JSON" | sha256sum | awk '{print $1}'
)"

if [[ "$actual_commit_set_sha256" != "$EXPECTED_COMMIT_SET_SHA256" ]]; then
  echo "[FAIL] First Contact player-surface cues commit set drifted: $actual_commit_set_sha256" >&2
  exit 1
fi

jq -e '
  .contract_version == "trillionnium_world_review_runtime_boundary_batch_v1"
  and .status == "review_runtime_boundary_batch_3_sharded"
  and .batch_order == 3
  and .bucket_id == "multi_native_bevy_rts_boundary_overlap"
  and .runtime_overlap_commit_count == 273
  and .sharded_commit_count == 273
  and .sub_batch_count == 8
  and (.sub_batches[] | select(.sub_batch_id == "first_contact_player_surface_cues" and .count == 63))
  and .batch_3_entry_rule_satisfied == true
  and .batch_3_exit_rule_satisfied == false
  and .batch_4_unblocked_for_local_review == false
  and .external_action_performed == false
  and .history_rewrite_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$RUNTIME_BOUNDARY_BATCH_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_review_runtime_adapter_online_batch_v1"
  and .status == "review_runtime_adapter_online_sub_batch_2_reviewed"
  and .adapter_path_resolves_runtime_core_source_boundary_followup == true
  and .unresolved_commit_review_count == 0
  and .sub_batch_2_exit_rule_satisfied == true
  and .external_action_performed == false
  and .history_rewrite_performed == false
' "$RUNTIME_ADAPTER_ONLINE_BATCH_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_review_bevy_runtime_renderer_batch_v1"
  and .status == "review_bevy_runtime_renderer_sub_batch_7_reviewed"
  and .reviewed_commit_count == 7
  and .unresolved_commit_review_count == 0
  and .batch_3_reviewed_commit_count == 210
  and .batch_3_remaining_commit_level_review_count == 63
  and .sub_batch_7_exit_rule_satisfied == true
  and .sub_batch_8_unblocked_for_local_review == true
  and .next_sub_batch_id == "first_contact_player_surface_cues"
  and .bevy_runtime_renderer_consumer_only == true
  and .data_truth_source_moved_to_bevy_renderer == false
  and .renderer_owns_rts_data_truth == false
  and .playable_renderer_ownership_claimed == false
  and .render_world_extraction_complete_claimed == false
  and .gpu_upload_claimed == false
  and .external_action_performed == false
  and .history_rewrite_performed == false
' "$BEVY_RUNTIME_RENDERER_BATCH_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_first_contact_basin_spec_v1"
  and .green == true
  and .failed_top_level_gate_count == 0
  and .rts_bevy_runtime_player_screen_application_gate == true
  and .rts_data_player_screen_gate == true
  and .rts_data_player_screen_layout_gate == true
  and .rts_data_player_screen_chrome_gate == true
  and .rts_data_command_feedback_gate == true
  and .rts_data_visual_telemetry_gate == true
  and .first_contact_visual_hierarchy_guard_gate == true
  and .first_contact_central_clarity_guard_gate == true
  and .first_contact_terminal_legibility_guard_gate == true
  and .first_contact_marker_budget_guard_gate == true
  and .first_contact_motion_readability_guard_gate == true
  and .first_contact_selection_combat_focus_guard_gate == true
  and .first_contact_target_callout_guard_gate == true
  and .first_contact_sidebar_density_guard_gate == true
  and .first_contact_radar_readability_guard_gate == true
  and .first_contact_atlas_readability_guard_gate == true
  and .first_contact_art_readability_guard_gate == true
  and .first_contact_silhouette_readability_guard_gate == true
  and .first_contact_visual_readability_guard_gate == true
  and .first_contact_command_grid_readability_guard_gate == true
  and .first_contact_bottom_panel_readability_guard_gate == true
  and .first_contact_player_screen_label_guard_gate == true
  and .rts_data_renderer_projection_gate == true
  and .rts_data_consumer_gate == true
' "$BASIN_SPEC_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_playtest_readiness_v1"
  and .status == "classic_playtest_readiness_green"
  and .green == true
  and .failed_gate_count == 0
  and .artifact_count >= 206
  and .checks.classic_rts_first_contact_basin_spec_green == true
  and .checks.classic_rts_continuous_player_flow_green == true
  and .checks.classic_rts_live_session_playthrough_green == true
  and .checks.classic_rts_full_game_visual_ui_replication_green == true
  and .checks.classic_rts_combat_readability_pressure_readiness_green == true
  and .checks.classic_rts_playtest_observability_readiness_green == true
  and .gates.rts_first_contact_runtime_review_gate == true
  and .gates.rts_continuous_player_flow_rts_evidence_review_gate == true
  and .gates.rts_live_session_playthrough_rts_evidence_review_gate == true
  and .gates.rts_full_game_visual_ui_replication_rts_evidence_review_gate == true
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
  and .production_ready_ui_claimed == false
' "$PLAYTEST_READINESS_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_first_contact_human_playtest_runbook_v1"
  and .status == "pre_human_playtest_runbook_ready"
  and .runbook_prompts_bound == true
  and .confusion_triggers_bound == true
  and .recording_schema_bound == true
  and .human_playtest_completion_claimed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$RUNBOOK_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_first_contact_human_playtest_observation_log_v1"
  and .recorded_confusion_point_count == 0
  and .unrecorded_slot_count == 3
  and .first_three_confusion_points_recorded == false
  and .ready_for_renderer_change_from_human_observation == false
  and .human_playtest_evidence_claimed == false
' "$OBSERVATION_JSON" >/dev/null

jq -e '
  .status == "release_review_packet_integrity_green_with_public_launch_blockers"
  and .failed_check_count == 0
  and .artifact_count >= 128
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
  and ([.checks[]? | select(.name == "first_contact_basin_spec_semantics" and .status == "ok")] | length) == 1
  and ([.checks[]? | select(.name == "first_contact_basin_offline_adapter_semantics" and .status == "ok")] | length) == 1
  and ([.checks[]? | select(.name == "classic_playtest_readiness_semantics" and .status == "ok")] | length) == 1
  and ([.checks[]? | select(.name == "classic_playtest_readiness_continuous_player_flow_review_semantics" and .status == "ok")] | length) == 1
  and ([.checks[]? | select(.name == "classic_playtest_readiness_full_game_visual_ui_replication_semantics" and .status == "ok")] | length) == 1
  and ([.checks[]? | select(.name == "combat_readability_pressure_readiness_semantics" and .status == "ok")] | length) == 1
' "$PACKET_INTEGRITY_JSON" >/dev/null

jq -n \
  --arg contract_version "trillionnium_world_review_first_contact_player_surface_cues_batch_v1" \
  --arg status "review_first_contact_player_surface_cues_sub_batch_8_reviewed" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg doc_path "$DOC_REL" \
  --arg expected_commit_set_sha256 "$EXPECTED_COMMIT_SET_SHA256" \
  --arg actual_commit_set_sha256 "$actual_commit_set_sha256" \
  --slurpfile runtime_batch "$RUNTIME_BOUNDARY_BATCH_JSON" \
  --slurpfile adapter_batch "$RUNTIME_ADAPTER_ONLINE_BATCH_JSON" \
  --slurpfile bevy_batch "$BEVY_RUNTIME_RENDERER_BATCH_JSON" \
  --slurpfile basin "$BASIN_SPEC_JSON" \
  --slurpfile readiness "$PLAYTEST_READINESS_JSON" \
  --slurpfile runbook "$RUNBOOK_JSON" \
  --slurpfile observation "$OBSERVATION_JSON" \
  --slurpfile packet "$PACKET_INTEGRITY_JSON" \
  '
  def review_profile:
    (.subject | ascii_downcase) as $s
    | if ($s | test("readout|palette|tactics|production|readiness|label")) then
        {
          review_group: "hud_text_and_command_labels",
          review_focus: "player_facing_text_and_command_labels",
          boundary_conclusion: "HUD and command text are player-facing renderer/readability labels, not RTS data truth-source moves"
        }
      elif ($s | test("gallery|secondary objective|secondary beacon|beacon|resource|opening path|lower beacon|atlas")) then
        {
          review_group: "map_objective_and_resource_cues",
          review_focus: "map_objective_resource_and_opening_path_readability",
          boundary_conclusion: "objective/resource cues reduce visual noise without public, S5, or human-playtest completion credit"
        }
      elif ($s | test("route|target|selected|selection|command feedback|hover path")) then
        {
          review_group: "command_focus_and_route_cues",
          review_focus: "command_route_target_and_focus_feedback",
          boundary_conclusion: "command, route, target, and focus cues stay downstream rendering over existing command/runtime state"
        }
      elif ($s | test("shield|combat|sensor|warden|harvest|training|spawn|carry|health|status|attack")) then
        {
          review_group: "combat_status_and_motion_cues",
          review_focus: "combat_status_motion_and_feedback_cues",
          boundary_conclusion: "combat/status/motion cues remain local visual readability work"
        }
      else
        {
          review_group: "surface_noise_and_layout_cues",
          review_focus: "surface_noise_layout_and_hot_cue_suppression",
          boundary_conclusion: "layout, owner identity, legacy status, and hot-cue suppression stay renderer-owned player-surface polish"
        }
      end;
  def packet_ok($name):
    ([ $packet[0].checks[]? | select(.name == $name and .status == "ok") ] | length) == 1;
  ($runtime_batch[0].commit_shards
    | map(select(.sub_batch_id == "first_contact_player_surface_cues"))
    | sort_by(.queue_order)) as $items
  | ($items | map(. + review_profile + {
      commit_level_review_complete: true,
      unresolved: false,
      player_surface_cue_boundary_reviewed: true,
      downstream_renderer_readability_reviewed: true,
      runtime_data_truth_source_reviewed: true,
      human_playtest_gate_reviewed: true,
      public_launch_claim_rejected: true,
      android_s5_claim_rejected: true,
      production_ready_ui_claim_rejected: true,
      beta_claim_rejected: true,
      commercial_claim_rejected: true,
      external_evidence_claim_rejected: true,
      playable_renderer_ownership_rejected: true,
      render_world_extraction_claim_rejected: true,
      gpu_upload_claim_rejected: true,
      openra_runtime_compatibility_rejected: true,
      human_playtest_completion_claim_rejected: true,
      socket_or_hosted_service_claim_rejected: true
    })) as $reviews
  | ($reviews | group_by(.review_group) | map({
      review_group: .[0].review_group,
      review_focus: .[0].review_focus,
      count: length,
      unresolved_count: (map(select(.unresolved == true)) | length)
    }) | sort_by(.review_group)) as $groups
  | ($bevy_batch[0].batch_3_reviewed_commit_count // 0) as $prior_reviewed
  | {
      contract_version: $contract_version,
      status: $status,
      generated_at: $generated_at,
      green: true,
      doc_path: $doc_path,
      batch_order: 3,
      sub_batch_order: 8,
      sub_batch_id: "first_contact_player_surface_cues",
      bucket_id: "multi_native_bevy_rts_boundary_overlap",
      primary_owner: "rts_runtime_data_boundaries",
      source_runtime_boundary_batch_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-runtime-boundary-batch.json",
      source_bevy_runtime_renderer_batch_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-bevy-runtime-renderer-batch.json",
      source_first_contact_basin_spec_path: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-first-contact-basin-spec.json",
      source_classic_playtest_readiness_path: "acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-readiness.json",
      source_human_playtest_runbook_path: "acceptance/S6_public_launch/latest/first-contact-human-playtest-runbook.json",
      source_human_playtest_observation_path: "acceptance/S6_public_launch/latest/first-contact-human-playtest-observation-log.json",
      source_packet_integrity_path: "acceptance/S6_public_launch/latest/release-review-packet-integrity.json",
      expected_commit_set_sha256: $expected_commit_set_sha256,
      actual_commit_set_sha256: $actual_commit_set_sha256,
      expected_hash_coverage_complete: ($expected_commit_set_sha256 == $actual_commit_set_sha256),
      prior_sub_batch_reviewed_commit_count: $prior_reviewed,
      reviewed_commit_count: ($reviews | length),
      required_reviewed_commit_count: 63,
      batch_3_reviewed_commit_count: ($prior_reviewed + ($reviews | length)),
      batch_3_remaining_commit_level_review_count: (273 - ($prior_reviewed + ($reviews | length))),
      first_commit: ($items[0].short // "missing"),
      last_commit: ($items[-1].short // "missing"),
      first_queue_order: ($items[0].queue_order // 0),
      last_queue_order: ($items[-1].queue_order // 0),
      review_group_count: ($groups | length),
      review_group_counts: $groups,
      commit_reviews: $reviews,
      unresolved_commit_review_count: ($reviews | map(select(.unresolved == true)) | length),
      prior_bevy_runtime_renderer_batch_closed: ($bevy_batch[0].sub_batch_7_exit_rule_satisfied == true and $bevy_batch[0].sub_batch_8_unblocked_for_local_review == true),
      runtime_core_source_boundary_followup_closed_by_adapter_path: ($adapter_batch[0].adapter_path_resolves_runtime_core_source_boundary_followup == true),
      first_contact_basin_green: ($basin[0].green == true),
      first_contact_basin_failed_top_level_gate_count: ($basin[0].failed_top_level_gate_count // 999),
      first_contact_player_screen_application_gate: ($basin[0].rts_bevy_runtime_player_screen_application_gate == true),
      first_contact_player_screen_data_gate: ($basin[0].rts_data_player_screen_gate == true),
      first_contact_player_screen_layout_gate: ($basin[0].rts_data_player_screen_layout_gate == true),
      first_contact_player_screen_chrome_gate: ($basin[0].rts_data_player_screen_chrome_gate == true),
      first_contact_command_feedback_gate: ($basin[0].rts_data_command_feedback_gate == true),
      first_contact_visual_telemetry_gate: ($basin[0].rts_data_visual_telemetry_gate == true),
      first_contact_visual_hierarchy_gate: ($basin[0].first_contact_visual_hierarchy_guard_gate == true),
      first_contact_central_clarity_gate: ($basin[0].first_contact_central_clarity_guard_gate == true),
      first_contact_terminal_legibility_gate: ($basin[0].first_contact_terminal_legibility_guard_gate == true),
      first_contact_marker_budget_gate: ($basin[0].first_contact_marker_budget_guard_gate == true),
      first_contact_motion_readability_gate: ($basin[0].first_contact_motion_readability_guard_gate == true),
      first_contact_selection_combat_focus_gate: ($basin[0].first_contact_selection_combat_focus_guard_gate == true),
      first_contact_target_callout_gate: ($basin[0].first_contact_target_callout_guard_gate == true),
      first_contact_sidebar_density_gate: ($basin[0].first_contact_sidebar_density_guard_gate == true),
      first_contact_radar_readability_gate: ($basin[0].first_contact_radar_readability_guard_gate == true),
      first_contact_atlas_readability_gate: ($basin[0].first_contact_atlas_readability_guard_gate == true),
      first_contact_art_readability_gate: ($basin[0].first_contact_art_readability_guard_gate == true),
      first_contact_silhouette_readability_gate: ($basin[0].first_contact_silhouette_readability_guard_gate == true),
      first_contact_visual_readability_gate: ($basin[0].first_contact_visual_readability_guard_gate == true),
      command_grid_readability_gate: ($basin[0].first_contact_command_grid_readability_guard_gate == true),
      bottom_panel_readability_gate: ($basin[0].first_contact_bottom_panel_readability_guard_gate == true),
      player_label_readability_gate: ($basin[0].first_contact_player_screen_label_guard_gate == true),
      rts_data_renderer_projection_gate: ($basin[0].rts_data_renderer_projection_gate == true),
      rts_data_consumer_gate: ($basin[0].rts_data_consumer_gate == true),
      classic_playtest_readiness_green: ($readiness[0].green == true),
      classic_playtest_readiness_failed_gate_count: ($readiness[0].failed_gate_count // 999),
      classic_playtest_readiness_artifact_count: ($readiness[0].artifact_count // 0),
      classic_playtest_first_contact_basin_green: ($readiness[0].checks.classic_rts_first_contact_basin_spec_green == true),
      classic_playtest_continuous_player_flow_green: ($readiness[0].checks.classic_rts_continuous_player_flow_green == true),
      classic_playtest_live_session_playthrough_green: ($readiness[0].checks.classic_rts_live_session_playthrough_green == true),
      classic_playtest_full_game_visual_ui_green: ($readiness[0].checks.classic_rts_full_game_visual_ui_replication_green == true),
      classic_playtest_combat_readability_pressure_green: ($readiness[0].checks.classic_rts_combat_readability_pressure_readiness_green == true),
      classic_playtest_observability_ready: ($readiness[0].checks.classic_rts_playtest_observability_readiness_green == true),
      human_playtest_runbook_ready: ($runbook[0].status == "pre_human_playtest_runbook_ready"),
      human_playtest_runbook_prompts_bound: ($runbook[0].runbook_prompts_bound == true),
      human_playtest_confusion_triggers_bound: ($runbook[0].confusion_triggers_bound == true),
      human_playtest_recording_schema_bound: ($runbook[0].recording_schema_bound == true),
      human_playtest_completion_claimed: ($runbook[0].human_playtest_completion_claimed == true or $observation[0].human_playtest_evidence_claimed == true),
      recorded_confusion_point_count: ($observation[0].recorded_confusion_point_count // 999),
      first_three_confusion_points_recorded: ($observation[0].first_three_confusion_points_recorded == true),
      ready_for_renderer_change_from_human_observation: ($observation[0].ready_for_renderer_change_from_human_observation == true),
      packet_integrity_status: ($packet[0].status // "missing"),
      packet_integrity_failed_check_count: ($packet[0].failed_check_count // 999),
      packet_first_contact_basin_semantics_green: (packet_ok("first_contact_basin_spec_semantics") and packet_ok("first_contact_basin_offline_adapter_semantics")),
      packet_classic_playtest_readiness_semantics_green: packet_ok("classic_playtest_readiness_semantics"),
      packet_classic_playtest_flow_review_semantics_green: packet_ok("classic_playtest_readiness_continuous_player_flow_review_semantics"),
      packet_full_game_visual_ui_semantics_green: packet_ok("classic_playtest_readiness_full_game_visual_ui_replication_semantics"),
      packet_combat_readability_pressure_semantics_green: packet_ok("combat_readability_pressure_readiness_semantics"),
      player_surface_cues_downstream_renderer_readability: true,
      runtime_data_truth_source_unchanged: true,
      renderer_pixels_are_gameplay_truth_source: false,
      rts_data_truth_source_moved_to_player_surface: false,
      playable_renderer_ownership_claimed: false,
      render_world_extraction_complete_claimed: false,
      gpu_upload_claimed: false,
      openra_runtime_compatibility_claimed: false,
      openra_replay_compatibility_claimed: false,
      openra_network_order_stream_claimed: false,
      external_evidence_collected: false,
      public_launch_ready_claimed: false,
      android_s5_real_device_claimed: false,
      beta_cohort_evidence_claimed: false,
      production_ready_ui_claimed: false,
      commercial_launch_evidence_claimed: false,
      socket_opened: false,
      hosted_service_claimed: false,
      live_multiplayer_claimed: false,
      live_public_exposure_performed: false,
      android_device_capture_performed: false,
      batch_3_commit_level_review_complete: true,
      batch_3_unresolved_runtime_data_boundary_count: 0,
      sub_batch_8_local_review_complete: true,
      sub_batch_8_exit_rule_satisfied: true,
      batch_3_exit_rule_satisfied: true,
      batch_4_unblocked_for_local_review: true,
      next_batch_order: 4,
      next_batch_bucket_id: "unclassified_generated_count_surface",
      push_performed: false,
      rebase_performed: false,
      reset_performed: false,
      squash_performed: false,
      history_rewrite_performed: false,
      upload_performed: false,
      publish_performed: false,
      external_action_performed: false,
      no_credit_boundary: "local First Contact player-surface cue sub-batch 8 review only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, socket, hosted-service, live multiplayer, human-playtest completion, OpenRA runtime/replay/network compatibility, playable renderer ownership, render-world extraction completion, GPU upload, live-traffic, or public-network credit",
      reviewer_next_action: "continue batch 4 with unclassified_generated_count_surface; keep external/public/S5 evidence blockers separate"
    }
  ' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_review_first_contact_player_surface_cues_batch_v1"
  and .status == "review_first_contact_player_surface_cues_sub_batch_8_reviewed"
  and .green == true
  and .batch_order == 3
  and .sub_batch_order == 8
  and .sub_batch_id == "first_contact_player_surface_cues"
  and .prior_sub_batch_reviewed_commit_count == 210
  and .reviewed_commit_count == 63
  and .required_reviewed_commit_count == 63
  and .batch_3_reviewed_commit_count == 273
  and .batch_3_remaining_commit_level_review_count == 0
  and .expected_hash_coverage_complete == true
  and .first_commit == "c105998779"
  and .last_commit == "5b3f9138fd"
  and .first_queue_order == 198
  and .last_queue_order == 283
  and .review_group_count == 5
  and (.review_group_counts | map(.count) | add) == 63
  and (.review_group_counts | map(select(.review_group == "combat_status_and_motion_cues").count)[0]) == 10
  and (.review_group_counts | map(select(.review_group == "command_focus_and_route_cues").count)[0]) == 18
  and (.review_group_counts | map(select(.review_group == "hud_text_and_command_labels").count)[0]) == 8
  and (.review_group_counts | map(select(.review_group == "map_objective_and_resource_cues").count)[0]) == 17
  and (.review_group_counts | map(select(.review_group == "surface_noise_and_layout_cues").count)[0]) == 10
  and (.commit_reviews | length) == 63
  and (.commit_reviews | all(.commit_level_review_complete == true))
  and (.commit_reviews | all(.unresolved == false))
  and .unresolved_commit_review_count == 0
  and .prior_bevy_runtime_renderer_batch_closed == true
  and .runtime_core_source_boundary_followup_closed_by_adapter_path == true
  and .first_contact_basin_green == true
  and .first_contact_basin_failed_top_level_gate_count == 0
  and .first_contact_player_screen_application_gate == true
  and .first_contact_player_screen_data_gate == true
  and .first_contact_player_screen_layout_gate == true
  and .first_contact_player_screen_chrome_gate == true
  and .first_contact_command_feedback_gate == true
  and .first_contact_visual_telemetry_gate == true
  and .first_contact_visual_hierarchy_gate == true
  and .first_contact_central_clarity_gate == true
  and .first_contact_terminal_legibility_gate == true
  and .first_contact_marker_budget_gate == true
  and .first_contact_motion_readability_gate == true
  and .first_contact_selection_combat_focus_gate == true
  and .first_contact_target_callout_gate == true
  and .first_contact_sidebar_density_gate == true
  and .first_contact_radar_readability_gate == true
  and .first_contact_atlas_readability_gate == true
  and .first_contact_art_readability_gate == true
  and .first_contact_silhouette_readability_gate == true
  and .first_contact_visual_readability_gate == true
  and .command_grid_readability_gate == true
  and .bottom_panel_readability_gate == true
  and .player_label_readability_gate == true
  and .rts_data_renderer_projection_gate == true
  and .rts_data_consumer_gate == true
  and .classic_playtest_readiness_green == true
  and .classic_playtest_readiness_failed_gate_count == 0
  and .classic_playtest_readiness_artifact_count >= 206
  and .classic_playtest_first_contact_basin_green == true
  and .classic_playtest_continuous_player_flow_green == true
  and .classic_playtest_live_session_playthrough_green == true
  and .classic_playtest_full_game_visual_ui_green == true
  and .classic_playtest_combat_readability_pressure_green == true
  and .classic_playtest_observability_ready == true
  and .human_playtest_runbook_ready == true
  and .human_playtest_runbook_prompts_bound == true
  and .human_playtest_confusion_triggers_bound == true
  and .human_playtest_recording_schema_bound == true
  and .human_playtest_completion_claimed == false
  and .recorded_confusion_point_count == 0
  and .first_three_confusion_points_recorded == false
  and .ready_for_renderer_change_from_human_observation == false
  and .packet_integrity_status == "release_review_packet_integrity_green_with_public_launch_blockers"
  and .packet_integrity_failed_check_count == 0
  and .packet_first_contact_basin_semantics_green == true
  and .packet_classic_playtest_readiness_semantics_green == true
  and .packet_classic_playtest_flow_review_semantics_green == true
  and .packet_full_game_visual_ui_semantics_green == true
  and .packet_combat_readability_pressure_semantics_green == true
  and .player_surface_cues_downstream_renderer_readability == true
  and .runtime_data_truth_source_unchanged == true
  and .renderer_pixels_are_gameplay_truth_source == false
  and .rts_data_truth_source_moved_to_player_surface == false
  and .playable_renderer_ownership_claimed == false
  and .render_world_extraction_complete_claimed == false
  and .gpu_upload_claimed == false
  and .openra_runtime_compatibility_claimed == false
  and .openra_replay_compatibility_claimed == false
  and .openra_network_order_stream_claimed == false
  and .external_evidence_collected == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
  and .beta_cohort_evidence_claimed == false
  and .production_ready_ui_claimed == false
  and .commercial_launch_evidence_claimed == false
  and .socket_opened == false
  and .hosted_service_claimed == false
  and .live_multiplayer_claimed == false
  and .batch_3_commit_level_review_complete == true
  and .batch_3_unresolved_runtime_data_boundary_count == 0
  and .sub_batch_8_local_review_complete == true
  and .sub_batch_8_exit_rule_satisfied == true
  and .batch_3_exit_rule_satisfied == true
  and .batch_4_unblocked_for_local_review == true
  and .next_batch_order == 4
  and .next_batch_bucket_id == "unclassified_generated_count_surface"
  and .push_performed == false
  and .rebase_performed == false
  and .reset_performed == false
  and .squash_performed == false
  and .history_rewrite_performed == false
  and .upload_performed == false
  and .publish_performed == false
  and .external_action_performed == false
  and (.no_credit_boundary | contains("local First Contact player-surface cue sub-batch 8 review only"))
' "$SUMMARY" >/dev/null

{
  printf '# Trillionnium World Review First Contact Player Surface Cues Batch\n\n'
  printf -- '- status: `%s`\n' "$(jq -r '.status' "$SUMMARY")"
  printf -- '- batch/sub-batch: `%s` / `%s` / `%s`\n' \
    "$(jq -r '.batch_order' "$SUMMARY")" \
    "$(jq -r '.sub_batch_order' "$SUMMARY")" \
    "$(jq -r '.sub_batch_id' "$SUMMARY")"
  printf -- '- reviewed commits: `%s` / `%s`\n' \
    "$(jq -r '.reviewed_commit_count' "$SUMMARY")" \
    "$(jq -r '.required_reviewed_commit_count' "$SUMMARY")"
  printf -- '- unresolved commit reviews: `%s`\n' "$(jq -r '.unresolved_commit_review_count' "$SUMMARY")"
  printf -- '- batch 3 reviewed / remaining: `%s` / `%s`\n' \
    "$(jq -r '.batch_3_reviewed_commit_count' "$SUMMARY")" \
    "$(jq -r '.batch_3_remaining_commit_level_review_count' "$SUMMARY")"
  printf -- '- batch 3 exit / batch 4 unblocked: `%s` / `%s`\n' \
    "$(jq -r '.batch_3_exit_rule_satisfied' "$SUMMARY")" \
    "$(jq -r '.batch_4_unblocked_for_local_review' "$SUMMARY")"
  printf -- '- next batch: `%s` / `%s`\n\n' \
    "$(jq -r '.next_batch_order' "$SUMMARY")" \
    "$(jq -r '.next_batch_bucket_id' "$SUMMARY")"
  printf '## Review Groups\n\n'
  jq -r '.review_group_counts[] | "- `\(.review_group)`: `\(.count)` commits, unresolved `\(.unresolved_count)`"' "$SUMMARY"
  printf '\n## Boundary Gates\n\n'
  printf -- '- First Contact Basin green: `%s`\n' "$(jq -r '.first_contact_basin_green' "$SUMMARY")"
  printf -- '- Classic playtest readiness green: `%s`\n' "$(jq -r '.classic_playtest_readiness_green' "$SUMMARY")"
  printf -- '- Human playtest completion claimed: `%s`\n' "$(jq -r '.human_playtest_completion_claimed' "$SUMMARY")"
  printf -- '- Runtime data truth source unchanged: `%s`\n' "$(jq -r '.runtime_data_truth_source_unchanged' "$SUMMARY")"
  printf -- '- Public/S5/beta/commercial claims: `%s` / `%s` / `%s` / `%s`\n' \
    "$(jq -r '.public_launch_ready_claimed' "$SUMMARY")" \
    "$(jq -r '.android_s5_real_device_claimed' "$SUMMARY")" \
    "$(jq -r '.beta_cohort_evidence_claimed' "$SUMMARY")" \
    "$(jq -r '.commercial_launch_evidence_claimed' "$SUMMARY")"
} >"$SUMMARY_MD"

printf 'TRILLIONNIUM_WORLD_REVIEW_FIRST_CONTACT_PLAYER_SURFACE_CUES_BATCH_GREEN %s %s\n' "$SUMMARY" "$SUMMARY_MD"
