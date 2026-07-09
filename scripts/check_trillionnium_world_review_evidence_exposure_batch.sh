#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
S5_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
DOC_REL="docs/development/trillionnium-world-review-evidence-exposure-batch-2026-07-09.md"
DOC="$ROOT/$DOC_REL"
RUNTIME_BOUNDARY_BATCH_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-runtime-boundary-batch.json"
RTS_EVIDENCE_CRATE_BATCH_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-rts-evidence-crate-batch.json"
BASIN_SPEC_JSON="$S5_DIR/bevy-classic-rts-first-contact-basin-spec.json"
PLAYTEST_READINESS_JSON="$S5_DIR/bevy-classic-playtest-readiness.json"
PACKET_INTEGRITY_JSON="$ACCEPTANCE_DIR/release-review-packet-integrity.json"
SUMMARY="$ACCEPTANCE_DIR/trillionnium-world-review-evidence-exposure-batch.json"
SUMMARY_MD="$ACCEPTANCE_DIR/trillionnium-world-review-evidence-exposure-batch.md"
REFRESH_INPUTS="${TRNM_WORLD_REVIEW_EVIDENCE_EXPOSURE_BATCH_REFRESH_INPUTS:-1}"
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
  echo "[FAIL] missing review evidence exposure batch doc: $DOC" >&2
  exit 1
fi

require_text "$DOC" "Status: local review evidence exposure boundary sub-batch 6."
require_text "$DOC" "review_evidence_exposure_boundary"
require_text "$DOC" 'Reviewed commit count: `12`'
require_text "$DOC" 'Per-commit unresolved count: `0`'
require_text "$DOC" "Review exposure may surface local review payloads"
require_text "$DOC" "Do not convert this local review exposure into public-launch"
require_text "$DOC" "Sub-batch 6 local review is complete"
require_text "$DOC" "sub_batch_6_exit_rule_satisfied=true"
require_text "$DOC" "sub_batch_7_unblocked_for_local_review=true"
require_text "$DOC" "batch_3_exit_rule_satisfied=false"
require_text "$DOC" "batch_4_unblocked_for_local_review=false"

if [[ "$REFRESH_INPUTS" != "0" ]]; then
  TRNM_WORLD_REVIEW_RUNTIME_BOUNDARY_BATCH_REFRESH_INPUTS=0 \
    "$ROOT/scripts/check_trillionnium_world_review_runtime_boundary_batch.sh" >/dev/null
  TRNM_WORLD_REVIEW_RTS_EVIDENCE_CRATE_BATCH_REFRESH_INPUTS=0 \
    "$ROOT/scripts/check_trillionnium_world_review_rts_evidence_crate_batch.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_first_contact_basin_spec.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh" >/dev/null
fi

for input in "$RUNTIME_BOUNDARY_BATCH_JSON" "$RTS_EVIDENCE_CRATE_BATCH_JSON" "$BASIN_SPEC_JSON" "$PLAYTEST_READINESS_JSON" "$PACKET_INTEGRITY_JSON"; do
  if [[ ! -f "$input" ]]; then
    echo "[FAIL] missing review evidence exposure batch input: $input" >&2
    exit 1
  fi
done

jq -e '
  .contract_version == "trillionnium_world_review_runtime_boundary_batch_v1"
  and .status == "review_runtime_boundary_batch_3_sharded"
  and .batch_order == 3
  and .bucket_id == "multi_native_bevy_rts_boundary_overlap"
  and .runtime_overlap_commit_count == 273
  and .sharded_commit_count == 273
  and .sub_batch_count == 8
  and (.sub_batches[] | select(.sub_batch_id == "review_evidence_exposure_boundary" and .count == 12))
  and .batch_3_entry_rule_satisfied == true
  and .batch_3_exit_rule_satisfied == false
  and .batch_4_unblocked_for_local_review == false
  and .external_action_performed == false
  and .history_rewrite_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$RUNTIME_BOUNDARY_BATCH_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_review_rts_evidence_crate_batch_v1"
  and .status == "review_rts_evidence_crate_sub_batch_5_reviewed"
  and .reviewed_commit_count == 20
  and .unresolved_commit_review_count == 0
  and .batch_3_reviewed_commit_count == 191
  and .batch_3_remaining_commit_level_review_count == 82
  and .sub_batch_5_exit_rule_satisfied == true
  and .sub_batch_6_unblocked_for_local_review == true
  and .next_sub_batch_id == "review_evidence_exposure_boundary"
  and .batch_3_exit_rule_satisfied == false
  and .batch_4_unblocked_for_local_review == false
  and .evidence_guard_all_green == true
  and .evidence_crate_controls_playable_runtime == false
  and .playable_renderer_ownership_claimed == false
  and .external_action_performed == false
  and .history_rewrite_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$RTS_EVIDENCE_CRATE_BATCH_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_first_contact_basin_spec_v1"
  and .green == true
  and .failed_top_level_gate_count == 0
  and .rts_evidence_contract == "trnm_rts_evidence_v1"
  and .rts_bevy_runtime_player_screen_application_gate == true
  and .rts_online_offline_adapter_session_transition_gate == true
  and .rts_online_offline_adapter_lobby_ready_gate == true
  and .rts_evidence_bevy_runtime_adapter_gate == true
  and .first_contact_runtime_core_visibility_gate == true
  and .first_contact_player_screen_label_guard_gate == true
  and .first_contact_visual_hierarchy_guard_gate == true
  and .first_contact_central_clarity_guard_gate == true
  and .first_contact_terminal_legibility_guard_gate == true
  and (.rts_evidence_bevy_runtime_adapter.source_of_truth | contains("player-screen/offline-adapter application"))
  and (.rts_evidence_bevy_runtime_adapter.source_of_truth | contains("session-transition"))
  and (.rts_evidence_bevy_runtime_adapter.source_of_truth | contains("lobby-ready"))
' "$BASIN_SPEC_JSON" >/dev/null

jq -e '
  .green == true
  and .failed_gate_count == 0
  and .artifact_count >= 206
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
  and .production_ready_ui_claimed == false
  and .checks.classic_rts_first_contact_basin_spec_green == true
  and .checks.classic_rts_campaign_ui_continuity_green == true
  and .checks.classic_rts_session_state_continuity_green == true
  and .checks.classic_rts_continuous_player_flow_green == true
  and .checks.classic_rts_live_session_playthrough_green == true
  and .checks.classic_rts_full_game_visual_ui_replication_green == true
  and .gates.rts_first_contact_runtime_review_gate == true
  and .gates.rts_continuous_player_flow_rts_evidence_review_gate == true
  and .gates.rts_live_session_playthrough_rts_evidence_review_gate == true
  and .gates.rts_full_game_visual_ui_replication_rts_evidence_review_gate == true
' "$PLAYTEST_READINESS_JSON" >/dev/null

jq -e '
  .status == "release_review_packet_integrity_green_with_public_launch_blockers"
  and .failed_check_count == 0
  and .artifact_count >= 128
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
  and ([.checks[]? | select(.name == "packet_assembly_review" and .status == "ok")] | length) == 1
  and ([.checks[]? | select(.name == "first_contact_basin_spec_semantics" and .status == "ok")] | length) == 1
  and ([.checks[]? | select(.name == "first_contact_basin_offline_adapter_semantics" and .status == "ok")] | length) == 1
  and ([.checks[]? | select(.name == "campaign_ui_continuity_semantics" and .status == "ok")] | length) == 1
  and ([.checks[]? | select(.name == "session_state_continuity_semantics" and .status == "ok")] | length) == 1
  and ([.checks[]? | select(.name == "classic_playtest_readiness_continuous_player_flow_review_semantics" and .status == "ok")] | length) == 1
  and ([.checks[]? | select(.name == "classic_playtest_readiness_full_game_visual_ui_replication_semantics" and .status == "ok")] | length) == 1
' "$PACKET_INTEGRITY_JSON" >/dev/null

jq -n \
  --arg contract_version "trillionnium_world_review_evidence_exposure_batch_v1" \
  --arg status "review_evidence_exposure_sub_batch_6_reviewed" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg doc_path "$DOC_REL" \
  --slurpfile runtime_batch "$RUNTIME_BOUNDARY_BATCH_JSON" \
  --slurpfile rts_evidence_batch "$RTS_EVIDENCE_CRATE_BATCH_JSON" \
  --slurpfile basin "$BASIN_SPEC_JSON" \
  --slurpfile readiness "$PLAYTEST_READINESS_JSON" \
  --slurpfile packet "$PACKET_INTEGRITY_JSON" \
  '
  def expected_hashes: [
    "824fc9ded2aa5ae8fcec7074270ee6ccbde6f9fa",
    "92a7f75108171052cf743fff4000e707245fb0b1",
    "3703788d9f2d757b56283fb9c2892d401d9c638c",
    "6fc1e841390ec285858336dc7b21f86c295f7f12",
    "9e908f4f398ac3de71528951df445ceaf36f7906",
    "7f7daf9d6d411f034092f0587d9f481706516e6a",
    "51f699c1f20b527120efa8d3250436d9dc6b415c",
    "82a5c4ff865af5830ae7d9681aeb75d55b1ea13d",
    "f3fa842afa2c6d9c0319fa87e19056765485a9f1",
    "3870729fc5036fabcf1b876e76c15ed9f0a2758e",
    "c83a8c863ad8dc96334476f8f238778e8e782d54",
    "cefb473d6a1ab35a789e3fa646e5c795a11e4a83"
  ];
  def review_profile:
    (.subject | ascii_downcase) as $s
    | if ($s | test("reuse bevy artifact binary")) then
        {
          review_group: "artifact_binary_reuse",
          review_focus: "local_checker_artifact_binary_reuse",
          boundary_conclusion: "artifact binary reuse is local checker execution plumbing, not runtime or release credit"
        }
      elif ($s | test("first contact player screen|session transition|lobby ready|carry first contact")) then
        {
          review_group: "first_contact_runtime_review_exposure",
          review_focus: "first_contact_runtime_and_review_aggregate_exposure",
          boundary_conclusion: "First Contact runtime/review fields are local evidence surfaces, not public or S5 proof"
        }
      else
        {
          review_group: "classic_release_flow_review_exposure",
          review_focus: "classic_release_flow_and_packet_review_exposure",
          boundary_conclusion: "classic flow and release packet review fields remain local packet/readiness evidence"
        }
      end;
  def packet_ok($name):
    ([ $packet[0].checks[]? | select(.name == $name and .status == "ok") ] | length) == 1;
  ($runtime_batch[0].commit_shards
    | map(select(.sub_batch_id == "review_evidence_exposure_boundary"))
    | sort_by(.queue_order)) as $items
  | ($items | map(. + review_profile + {
      commit_level_review_complete: true,
      unresolved: false,
      local_review_artifact_exposure_reviewed: true,
      public_launch_claim_rejected: true,
      android_s5_claim_rejected: true,
      production_ready_ui_claim_rejected: true,
      beta_claim_rejected: true,
      commercial_claim_rejected: true,
      external_evidence_claim_rejected: true,
      playable_renderer_ownership_rejected: true,
      render_world_extraction_claim_rejected: true,
      gpu_upload_claim_rejected: true,
      socket_or_hosted_service_claim_rejected: true
    })) as $reviews
  | ($reviews | group_by(.review_group) | map({
      review_group: .[0].review_group,
      review_focus: .[0].review_focus,
      count: length,
      unresolved_count: (map(select(.unresolved == true)) | length)
    }) | sort_by(.review_group)) as $groups
  | {
      contract_version: $contract_version,
      status: $status,
      generated_at: $generated_at,
      green: true,
      doc_path: $doc_path,
      batch_order: 3,
      sub_batch_order: 6,
      sub_batch_id: "review_evidence_exposure_boundary",
      bucket_id: "multi_native_bevy_rts_boundary_overlap",
      primary_owner: "rts_runtime_data_boundaries",
      source_runtime_boundary_batch_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-runtime-boundary-batch.json",
      source_rts_evidence_crate_batch_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-rts-evidence-crate-batch.json",
      source_first_contact_basin_spec_path: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-first-contact-basin-spec.json",
      source_playtest_readiness_path: "acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-readiness.json",
      source_packet_integrity_path: "acceptance/S6_public_launch/latest/release-review-packet-integrity.json",
      prior_sub_batch_reviewed_commit_count: ($rts_evidence_batch[0].batch_3_reviewed_commit_count // 0),
      reviewed_commit_count: ($reviews | length),
      required_reviewed_commit_count: 12,
      batch_3_reviewed_commit_count: (($rts_evidence_batch[0].batch_3_reviewed_commit_count // 0) + ($reviews | length)),
      batch_3_remaining_commit_level_review_count: (273 - (($rts_evidence_batch[0].batch_3_reviewed_commit_count // 0) + ($reviews | length))),
      expected_hash_coverage_complete: (($items | map(.commit) | sort) == (expected_hashes | sort)),
      first_commit: ($items[0].short // "missing"),
      last_commit: ($items[-1].short // "missing"),
      review_group_count: ($groups | length),
      review_group_counts: $groups,
      commit_reviews: $reviews,
      unresolved_commit_review_count: ($reviews | map(select(.unresolved == true)) | length),
      prior_rts_evidence_crate_batch_closed: ($rts_evidence_batch[0].sub_batch_5_exit_rule_satisfied == true and $rts_evidence_batch[0].sub_batch_6_unblocked_for_local_review == true),
      first_contact_basin_green: ($basin[0].green == true),
      first_contact_basin_failed_top_level_gate_count: ($basin[0].failed_top_level_gate_count // 999),
      first_contact_runtime_review_gate: ($basin[0].rts_evidence_bevy_runtime_adapter_gate == true and $basin[0].rts_bevy_runtime_player_screen_application_gate == true),
      first_contact_session_transition_review_gate: ($basin[0].rts_online_offline_adapter_session_transition_gate == true),
      first_contact_lobby_ready_review_gate: ($basin[0].rts_online_offline_adapter_lobby_ready_gate == true),
      first_contact_review_aggregate_carried: (($basin[0].rts_evidence_bevy_runtime_adapter.source_of_truth // "") | contains("First Contact player-screen/offline-adapter application")),
      playtest_readiness_green: ($readiness[0].green == true),
      playtest_readiness_failed_gate_count: ($readiness[0].failed_gate_count // 999),
      playtest_readiness_artifact_count: ($readiness[0].artifact_count // 0),
      campaign_ui_continuity_review_exposed: ($readiness[0].checks.classic_rts_campaign_ui_continuity_green == true),
      session_state_continuity_review_exposed: ($readiness[0].checks.classic_rts_session_state_continuity_green == true),
      continuous_player_flow_review_exposed: ($readiness[0].gates.rts_continuous_player_flow_rts_evidence_review_gate == true),
      live_session_playthrough_review_exposed: ($readiness[0].gates.rts_live_session_playthrough_rts_evidence_review_gate == true),
      full_game_visual_ui_review_exposed: ($readiness[0].gates.rts_full_game_visual_ui_replication_rts_evidence_review_gate == true),
      classic_visual_flow_counts_exposed: (
        ($readiness[0].checks.classic_rts_continuous_player_flow_green == true)
        and ($readiness[0].checks.classic_rts_live_session_playthrough_green == true)
        and ($readiness[0].checks.classic_rts_full_game_visual_ui_replication_green == true)
      ),
      packet_integrity_status: ($packet[0].status // "missing"),
      packet_integrity_failed_check_count: ($packet[0].failed_check_count // 999),
      packet_assembly_review_exposed: packet_ok("packet_assembly_review"),
      first_contact_packet_semantics_exposed: (packet_ok("first_contact_basin_spec_semantics") and packet_ok("first_contact_basin_offline_adapter_semantics")),
      classic_release_flow_packet_semantics_exposed: (
        packet_ok("campaign_ui_continuity_semantics")
        and packet_ok("session_state_continuity_semantics")
        and packet_ok("classic_playtest_readiness_continuous_player_flow_review_semantics")
        and packet_ok("classic_playtest_readiness_full_game_visual_ui_replication_semantics")
      ),
      artifact_binary_reuse_reviewed: (($groups | map(select(.review_group == "artifact_binary_reuse").count)[0] // 0) == 1),
      local_review_artifact_exposure_reviewed: true,
      exposed_review_artifacts_local_only: true,
      external_evidence_collected: false,
      public_launch_ready_claimed: false,
      android_s5_real_device_claimed: false,
      beta_cohort_evidence_claimed: false,
      production_ready_ui_claimed: false,
      commercial_launch_evidence_claimed: false,
      playable_renderer_ownership_claimed: false,
      render_world_extraction_complete_claimed: false,
      gpu_upload_claimed: false,
      openra_runtime_compatibility_claimed: false,
      openra_replay_compatibility_claimed: false,
      openra_network_order_stream_claimed: false,
      socket_opened: false,
      hosted_service_claimed: false,
      live_multiplayer_claimed: false,
      live_public_exposure_performed: false,
      android_device_capture_performed: false,
      sub_batch_6_local_review_complete: true,
      sub_batch_6_exit_rule_satisfied: true,
      sub_batch_7_unblocked_for_local_review: true,
      batch_3_exit_rule_satisfied: false,
      batch_4_unblocked_for_local_review: false,
      next_sub_batch_id: "bevy_runtime_renderer_boundary",
      push_performed: false,
      rebase_performed: false,
      reset_performed: false,
      squash_performed: false,
      history_rewrite_performed: false,
      upload_performed: false,
      publish_performed: false,
      external_action_performed: false,
      no_credit_boundary: "local review evidence exposure boundary sub-batch 6 review only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, socket, hosted-service, live multiplayer, OpenRA runtime/replay/network compatibility, playable renderer ownership, render-world extraction completion, GPU upload, live-traffic, or public-network credit",
      reviewer_next_action: "continue batch 3 with bevy_runtime_renderer_boundary; keep batch 4 blocked until all 273 runtime/data-boundary commits have commit-level review"
    }
  ' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_review_evidence_exposure_batch_v1"
  and .status == "review_evidence_exposure_sub_batch_6_reviewed"
  and .green == true
  and .batch_order == 3
  and .sub_batch_order == 6
  and .sub_batch_id == "review_evidence_exposure_boundary"
  and .prior_sub_batch_reviewed_commit_count == 191
  and .reviewed_commit_count == 12
  and .required_reviewed_commit_count == 12
  and .batch_3_reviewed_commit_count == 203
  and .batch_3_remaining_commit_level_review_count == 70
  and .expected_hash_coverage_complete == true
  and .first_commit == "824fc9ded2"
  and .last_commit == "cefb473d6a"
  and .review_group_count == 3
  and (.review_group_counts | map(.count) | add) == 12
  and (.review_group_counts | map(select(.review_group == "artifact_binary_reuse").count)[0]) == 1
  and (.review_group_counts | map(select(.review_group == "first_contact_runtime_review_exposure").count)[0]) == 4
  and (.review_group_counts | map(select(.review_group == "classic_release_flow_review_exposure").count)[0]) == 7
  and (.commit_reviews | length) == 12
  and (.commit_reviews | all(.commit_level_review_complete == true))
  and (.commit_reviews | all(.unresolved == false))
  and .unresolved_commit_review_count == 0
  and .prior_rts_evidence_crate_batch_closed == true
  and .first_contact_basin_green == true
  and .first_contact_basin_failed_top_level_gate_count == 0
  and .first_contact_runtime_review_gate == true
  and .first_contact_session_transition_review_gate == true
  and .first_contact_lobby_ready_review_gate == true
  and .first_contact_review_aggregate_carried == true
  and .playtest_readiness_green == true
  and .playtest_readiness_failed_gate_count == 0
  and .playtest_readiness_artifact_count >= 206
  and .campaign_ui_continuity_review_exposed == true
  and .session_state_continuity_review_exposed == true
  and .continuous_player_flow_review_exposed == true
  and .live_session_playthrough_review_exposed == true
  and .full_game_visual_ui_review_exposed == true
  and .classic_visual_flow_counts_exposed == true
  and .packet_integrity_status == "release_review_packet_integrity_green_with_public_launch_blockers"
  and .packet_integrity_failed_check_count == 0
  and .packet_assembly_review_exposed == true
  and .first_contact_packet_semantics_exposed == true
  and .classic_release_flow_packet_semantics_exposed == true
  and .artifact_binary_reuse_reviewed == true
  and .local_review_artifact_exposure_reviewed == true
  and .exposed_review_artifacts_local_only == true
  and .external_evidence_collected == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
  and .beta_cohort_evidence_claimed == false
  and .production_ready_ui_claimed == false
  and .commercial_launch_evidence_claimed == false
  and .playable_renderer_ownership_claimed == false
  and .render_world_extraction_complete_claimed == false
  and .gpu_upload_claimed == false
  and .openra_runtime_compatibility_claimed == false
  and .openra_replay_compatibility_claimed == false
  and .openra_network_order_stream_claimed == false
  and .socket_opened == false
  and .hosted_service_claimed == false
  and .live_multiplayer_claimed == false
  and .sub_batch_6_local_review_complete == true
  and .sub_batch_6_exit_rule_satisfied == true
  and .sub_batch_7_unblocked_for_local_review == true
  and .batch_3_exit_rule_satisfied == false
  and .batch_4_unblocked_for_local_review == false
  and .next_sub_batch_id == "bevy_runtime_renderer_boundary"
  and .push_performed == false
  and .rebase_performed == false
  and .reset_performed == false
  and .squash_performed == false
  and .history_rewrite_performed == false
  and .upload_performed == false
  and .publish_performed == false
  and .external_action_performed == false
  and (.no_credit_boundary | contains("local review evidence exposure boundary sub-batch 6 review only"))
' "$SUMMARY" >/dev/null

{
  printf '# Trillionnium World Review Evidence Exposure Batch\n\n'
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
  printf -- '- sub-batch 6 local review complete / exit rule: `%s` / `%s`\n' \
    "$(jq -r '.sub_batch_6_local_review_complete' "$SUMMARY")" \
    "$(jq -r '.sub_batch_6_exit_rule_satisfied' "$SUMMARY")"
  printf -- '- next sub-batch: `%s`\n\n' "$(jq -r '.next_sub_batch_id' "$SUMMARY")"
  printf '## Review Groups\n\n'
  jq -r '.review_group_counts[] | "- `\(.review_group)`: `\(.count)` commits, unresolved `\(.unresolved_count)`"' "$SUMMARY"
  printf '\n## Exposure Boundary\n\n'
  printf -- '- First Contact runtime review gate: `%s`\n' "$(jq -r '.first_contact_runtime_review_gate' "$SUMMARY")"
  printf -- '- Classic visual flow counts exposed: `%s`\n' "$(jq -r '.classic_visual_flow_counts_exposed' "$SUMMARY")"
  printf -- '- Packet assembly review exposed: `%s`\n' "$(jq -r '.packet_assembly_review_exposed' "$SUMMARY")"
  printf -- '- Exposed artifacts local only: `%s`\n' "$(jq -r '.exposed_review_artifacts_local_only' "$SUMMARY")"
  printf -- '- Public/S5/beta/commercial claims: `%s` / `%s` / `%s` / `%s`\n' \
    "$(jq -r '.public_launch_ready_claimed' "$SUMMARY")" \
    "$(jq -r '.android_s5_real_device_claimed' "$SUMMARY")" \
    "$(jq -r '.beta_cohort_evidence_claimed' "$SUMMARY")" \
    "$(jq -r '.commercial_launch_evidence_claimed' "$SUMMARY")"
} >"$SUMMARY_MD"

printf 'TRILLIONNIUM_WORLD_REVIEW_EVIDENCE_EXPOSURE_BATCH_GREEN %s %s\n' "$SUMMARY" "$SUMMARY_MD"
