#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
S5_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
DOC_REL="docs/development/trillionnium-world-review-rts-evidence-crate-batch-2026-07-09.md"
DOC="$ROOT/$DOC_REL"
RUNTIME_BOUNDARY_BATCH_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-runtime-boundary-batch.json"
FIRST_CONTACT_RTS_DATA_BATCH_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-first-contact-rts-data-batch.json"
BASIN_SPEC_JSON="$S5_DIR/bevy-classic-rts-first-contact-basin-spec.json"
PACKET_INTEGRITY_JSON="$ACCEPTANCE_DIR/release-review-packet-integrity.json"
SUMMARY="$ACCEPTANCE_DIR/trillionnium-world-review-rts-evidence-crate-batch.json"
SUMMARY_MD="$ACCEPTANCE_DIR/trillionnium-world-review-rts-evidence-crate-batch.md"
REFRESH_INPUTS="${TRNM_WORLD_REVIEW_RTS_EVIDENCE_CRATE_BATCH_REFRESH_INPUTS:-1}"
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
  echo "[FAIL] missing RTS evidence crate batch doc: $DOC" >&2
  exit 1
fi

require_text "$DOC" "Status: local review RTS evidence crate boundary sub-batch 5."
require_text "$DOC" "rts_evidence_crate_boundary"
require_text "$DOC" 'Reviewed commit count: `20`'
require_text "$DOC" 'Per-commit unresolved count: `0`'
require_text "$DOC" "Evidence crates may carry review payloads"
require_text "$DOC" "Do not convert this local review into playable renderer ownership"
require_text "$DOC" "Sub-batch 5 local review is complete"
require_text "$DOC" "sub_batch_5_exit_rule_satisfied=true"
require_text "$DOC" "sub_batch_6_unblocked_for_local_review=true"
require_text "$DOC" "batch_3_exit_rule_satisfied=false"
require_text "$DOC" "batch_4_unblocked_for_local_review=false"

if [[ "$REFRESH_INPUTS" != "0" ]]; then
  TRNM_WORLD_REVIEW_RUNTIME_BOUNDARY_BATCH_REFRESH_INPUTS=0 \
    "$ROOT/scripts/check_trillionnium_world_review_runtime_boundary_batch.sh" >/dev/null
  TRNM_WORLD_REVIEW_FIRST_CONTACT_RTS_DATA_BATCH_REFRESH_INPUTS=0 \
    "$ROOT/scripts/check_trillionnium_world_review_first_contact_rts_data_batch.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_first_contact_basin_spec.sh" >/dev/null
fi

for input in "$RUNTIME_BOUNDARY_BATCH_JSON" "$FIRST_CONTACT_RTS_DATA_BATCH_JSON" "$BASIN_SPEC_JSON" "$PACKET_INTEGRITY_JSON"; do
  if [[ ! -f "$input" ]]; then
    echo "[FAIL] missing RTS evidence crate batch input: $input" >&2
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
  and (.sub_batches[] | select(.sub_batch_id == "rts_evidence_crate_boundary" and .count == 20))
  and .batch_3_entry_rule_satisfied == true
  and .batch_3_exit_rule_satisfied == false
  and .batch_4_unblocked_for_local_review == false
  and .external_action_performed == false
  and .history_rewrite_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$RUNTIME_BOUNDARY_BATCH_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_review_first_contact_rts_data_batch_v1"
  and .status == "review_first_contact_rts_data_sub_batch_4_reviewed"
  and .reviewed_commit_count == 24
  and .unresolved_commit_review_count == 0
  and .batch_3_reviewed_commit_count == 171
  and .batch_3_remaining_commit_level_review_count == 102
  and .sub_batch_4_exit_rule_satisfied == true
  and .sub_batch_5_unblocked_for_local_review == true
  and .next_sub_batch_id == "rts_evidence_crate_boundary"
  and .batch_3_exit_rule_satisfied == false
  and .batch_4_unblocked_for_local_review == false
  and .renderer_draw_math_moved_to_rts_data == false
  and .live_bevy_renderer_behavior_moved_to_rts_data == false
  and .external_action_performed == false
  and .history_rewrite_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$FIRST_CONTACT_RTS_DATA_BATCH_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_first_contact_basin_spec_v1"
  and .green == true
  and .failed_top_level_gate_count == 0
  and .rts_evidence_contract == "trnm_rts_evidence_v1"
  and .rts_evidence_bevy_runtime_adapter_gate == true
  and .first_contact_silhouette_readability_guard_gate == true
  and .first_contact_art_readability_guard_gate == true
  and .first_contact_motion_readability_guard_gate == true
  and .first_contact_visual_readability_guard_gate == true
  and .first_contact_radar_readability_guard_gate == true
  and .first_contact_marker_budget_guard_gate == true
  and .first_contact_selection_combat_focus_guard_gate == true
  and .first_contact_target_callout_guard_gate == true
  and .first_contact_atlas_readability_guard_gate == true
  and .first_contact_command_grid_readability_guard_gate == true
  and .first_contact_bottom_panel_readability_guard_gate == true
  and .first_contact_sidebar_density_guard_gate == true
  and .first_contact_player_screen_label_guard_gate == true
  and .first_contact_visual_hierarchy_guard_gate == true
  and .first_contact_central_clarity_guard_gate == true
  and .first_contact_terminal_legibility_guard_gate == true
  and (.rts_evidence_bevy_runtime_adapter.source_of_truth | contains("RTS evidence crate verifies"))
  and (.rts_evidence_bevy_runtime_adapter.source_of_truth | contains("before trnm-world-bevy includes the proof"))
' "$BASIN_SPEC_JSON" >/dev/null

jq -e '
  .status == "release_review_packet_integrity_green_with_public_launch_blockers"
  and .failed_check_count == 0
  and .artifact_count >= 128
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
' "$PACKET_INTEGRITY_JSON" >/dev/null

jq -n \
  --arg contract_version "trillionnium_world_review_rts_evidence_crate_batch_v1" \
  --arg status "review_rts_evidence_crate_sub_batch_5_reviewed" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg doc_path "$DOC_REL" \
  --slurpfile runtime_batch "$RUNTIME_BOUNDARY_BATCH_JSON" \
  --slurpfile first_contact_batch "$FIRST_CONTACT_RTS_DATA_BATCH_JSON" \
  --slurpfile basin "$BASIN_SPEC_JSON" \
  --slurpfile packet "$PACKET_INTEGRITY_JSON" \
  '
  def expected_hashes: [
    "b4b8c1246f84ca1b3d4ef1ee60fc0c8108028e70",
    "5df984ff1d10e5634372aaa0c5b4afa0e7fff5ca",
    "7cd12c25cfbca4daddbff2b9e3fe1d7a303c14a2",
    "874d372ebedba73016d2d395432d04443ee07b6a",
    "a8d9fa2dce35c5d9485187f7e26e8e8dfd2e109d",
    "33140eb86e19a97e87486f80fcb46c51c4124897",
    "d6229dda056809571d536cb8f685c8e44c73123d",
    "8bc35a9b7dba4f4064d1af275b8b3ccb5159f518",
    "f77f13b08d10857370a1a8bfef44a0d0f31f2325",
    "fb9999ae75e6164fc50d388fa8b19dc058d853c5",
    "e57ede2f2d8efccf3a5b5860ee104aa11e4e3cfe",
    "b635c72036f01cf52ebf9d280f376c4e0fe5c673",
    "83b895207e1ab0ddb84255b33b75fe0de4a83c00",
    "a8ccbe209e9154dd3ce38e63d7795a12c9d80d19",
    "877b515dd6991c390b58a6cdc432cab23e0438b7",
    "5a4f16da77b12c60540e3d7f3ddead8cc438414e",
    "b039d94f897e4243a8c94c998a48fe7c498b66b2",
    "92803585ae930c8363370e3522ff95f0f3956470",
    "021c9f92a7d204aacb9659ec21aad66edace1c16",
    "ffee475c4cdeedc0d00128209170cf05b141b175"
  ];
  def review_profile:
    (.subject | ascii_downcase) as $s
    | if ($s | test("online gates|projection|player screen gates|data gates|map review")) then
        {
          review_group: "evidence_gate_projection_payloads",
          review_focus: "first_contact_gate_projection_payloads_in_rts_evidence",
          boundary_conclusion: "online/projection/player-screen/data/map-review gates are local evidence payloads, not playable renderer ownership"
        }
      elif ($s | test("silhouette|art|motion|visual|radar|spatial|marker|focus|atlas")) then
        {
          review_group: "readability_geometry_payloads",
          review_focus: "readability_geometry_payloads_in_rts_evidence",
          boundary_conclusion: "readability geometry and marker/focus/atlas payloads are guard inputs, not live renderer behavior"
        }
      else
        {
          review_group: "ui_panel_label_payloads",
          review_focus: "ui_panel_and_label_payloads_in_rts_evidence",
          boundary_conclusion: "command-grid, bottom-panel, sidebar, and label payloads are local guard inputs consumed by Bevy"
        }
      end;
  def evidence_guard_gates($b): [
    $b.first_contact_silhouette_readability_guard_gate,
    $b.first_contact_art_readability_guard_gate,
    $b.first_contact_motion_readability_guard_gate,
    $b.first_contact_visual_readability_guard_gate,
    $b.first_contact_radar_readability_guard_gate,
    $b.first_contact_marker_budget_guard_gate,
    $b.first_contact_selection_combat_focus_guard_gate,
    $b.first_contact_target_callout_guard_gate,
    $b.first_contact_atlas_readability_guard_gate,
    $b.first_contact_command_grid_readability_guard_gate,
    $b.first_contact_bottom_panel_readability_guard_gate,
    $b.first_contact_sidebar_density_guard_gate,
    $b.first_contact_player_screen_label_guard_gate,
    $b.first_contact_visual_hierarchy_guard_gate,
    $b.first_contact_central_clarity_guard_gate,
    $b.first_contact_terminal_legibility_guard_gate
  ];
  ($runtime_batch[0].commit_shards
    | map(select(.sub_batch_id == "rts_evidence_crate_boundary"))
    | sort_by(.queue_order)) as $items
  | ($items | map(. + review_profile + {
      commit_level_review_complete: true,
      unresolved: false,
      rts_evidence_crate_payload_reviewed: true,
      evidence_guard_boundary_reviewed: true,
      bevy_runtime_consumption_reviewed: true,
      playable_renderer_ownership_rejected: true,
      live_renderer_behavior_transfer_rejected: true,
      render_world_extraction_claim_rejected: true,
      gpu_upload_claim_rejected: true,
      external_evidence_claim_rejected: true,
      public_launch_claim_rejected: true,
      android_s5_claim_rejected: true,
      production_ready_ui_claim_rejected: true,
      beta_claim_rejected: true,
      commercial_claim_rejected: true
    })) as $reviews
  | ($reviews | group_by(.review_group) | map({
      review_group: .[0].review_group,
      review_focus: .[0].review_focus,
      count: length,
      unresolved_count: (map(select(.unresolved == true)) | length)
    }) | sort_by(.review_group)) as $groups
  | (evidence_guard_gates($basin[0])) as $guard_gates
  | {
      contract_version: $contract_version,
      status: $status,
      generated_at: $generated_at,
      green: true,
      doc_path: $doc_path,
      batch_order: 3,
      sub_batch_order: 5,
      sub_batch_id: "rts_evidence_crate_boundary",
      bucket_id: "multi_native_bevy_rts_boundary_overlap",
      primary_owner: "rts_runtime_data_boundaries",
      source_runtime_boundary_batch_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-runtime-boundary-batch.json",
      source_first_contact_rts_data_batch_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-first-contact-rts-data-batch.json",
      source_first_contact_basin_spec_path: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-first-contact-basin-spec.json",
      source_packet_integrity_path: "acceptance/S6_public_launch/latest/release-review-packet-integrity.json",
      prior_sub_batch_reviewed_commit_count: ($first_contact_batch[0].batch_3_reviewed_commit_count // 0),
      reviewed_commit_count: ($reviews | length),
      required_reviewed_commit_count: 20,
      batch_3_reviewed_commit_count: (($first_contact_batch[0].batch_3_reviewed_commit_count // 0) + ($reviews | length)),
      batch_3_remaining_commit_level_review_count: (273 - (($first_contact_batch[0].batch_3_reviewed_commit_count // 0) + ($reviews | length))),
      expected_hash_coverage_complete: (($items | map(.commit) | sort) == (expected_hashes | sort)),
      first_commit: ($items[0].short // "missing"),
      last_commit: ($items[-1].short // "missing"),
      review_group_count: ($groups | length),
      review_group_counts: $groups,
      commit_reviews: $reviews,
      unresolved_commit_review_count: ($reviews | map(select(.unresolved == true)) | length),
      basin_spec_green: ($basin[0].green == true),
      basin_spec_failed_top_level_gate_count: ($basin[0].failed_top_level_gate_count // 999),
      rts_evidence_contract: ($basin[0].rts_evidence_contract // "missing"),
      rts_evidence_bevy_runtime_adapter_gate: ($basin[0].rts_evidence_bevy_runtime_adapter_gate == true),
      evidence_guard_gate_count: ($guard_gates | length),
      evidence_guard_green_count: ($guard_gates | map(select(. == true)) | length),
      evidence_guard_all_green: ($guard_gates | all(. == true)),
      evidence_guard_contracts: [
        $basin[0].first_contact_silhouette_readability_contract,
        $basin[0].first_contact_art_readability_contract,
        $basin[0].first_contact_motion_readability_contract,
        $basin[0].first_contact_visual_readability_contract,
        $basin[0].first_contact_radar_readability_contract,
        $basin[0].first_contact_marker_budget_contract,
        $basin[0].first_contact_selection_combat_focus_contract,
        $basin[0].first_contact_target_callout_contract,
        $basin[0].first_contact_atlas_readability_contract,
        $basin[0].first_contact_command_grid_readability_contract,
        $basin[0].first_contact_bottom_panel_readability_contract,
        $basin[0].first_contact_sidebar_density_contract,
        $basin[0].first_contact_player_screen_label_guard_contract,
        $basin[0].first_contact_visual_hierarchy_contract,
        $basin[0].first_contact_central_clarity_contract,
        $basin[0].first_contact_terminal_legibility_contract
      ],
      source_of_truth_mentions_rts_evidence_crate: (($basin[0].rts_evidence_bevy_runtime_adapter.source_of_truth // "") | contains("RTS evidence crate verifies")),
      source_of_truth_keeps_bevy_consumer_boundary: (($basin[0].rts_evidence_bevy_runtime_adapter.source_of_truth // "") | contains("before trnm-world-bevy includes the proof")),
      rts_evidence_crate_boundary_reviewed: true,
      rts_evidence_payloads_local_only: true,
      evidence_crate_controls_playable_runtime: false,
      playable_renderer_ownership_claimed: false,
      live_bevy_renderer_behavior_moved_to_rts_evidence: false,
      render_world_extraction_complete_claimed: false,
      gpu_upload_claimed: false,
      public_launch_ready_claimed: false,
      android_s5_real_device_claimed: false,
      beta_cohort_evidence_claimed: false,
      production_ready_ui_claimed: false,
      commercial_launch_evidence_claimed: false,
      live_public_exposure_performed: false,
      android_device_capture_performed: false,
      socket_opened: false,
      hosted_service_claimed: false,
      live_multiplayer_claimed: false,
      sub_batch_5_local_review_complete: true,
      sub_batch_5_exit_rule_satisfied: true,
      sub_batch_6_unblocked_for_local_review: true,
      batch_3_exit_rule_satisfied: false,
      batch_4_unblocked_for_local_review: false,
      next_sub_batch_id: "review_evidence_exposure_boundary",
      packet_integrity_status: ($packet[0].status // "missing"),
      packet_integrity_failed_check_count: ($packet[0].failed_check_count // 999),
      push_performed: false,
      rebase_performed: false,
      reset_performed: false,
      squash_performed: false,
      history_rewrite_performed: false,
      upload_performed: false,
      publish_performed: false,
      external_action_performed: false,
      no_credit_boundary: "local RTS evidence crate boundary sub-batch 5 review only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, socket, hosted-service, live multiplayer, playable renderer ownership, render-world extraction completion, GPU upload, live-traffic, or public-network credit",
      reviewer_next_action: "continue batch 3 with review_evidence_exposure_boundary; keep batch 4 blocked until all 273 runtime/data-boundary commits have commit-level review"
    }
  ' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_review_rts_evidence_crate_batch_v1"
  and .status == "review_rts_evidence_crate_sub_batch_5_reviewed"
  and .green == true
  and .batch_order == 3
  and .sub_batch_order == 5
  and .sub_batch_id == "rts_evidence_crate_boundary"
  and .prior_sub_batch_reviewed_commit_count == 171
  and .reviewed_commit_count == 20
  and .required_reviewed_commit_count == 20
  and .batch_3_reviewed_commit_count == 191
  and .batch_3_remaining_commit_level_review_count == 82
  and .expected_hash_coverage_complete == true
  and .first_commit == "b4b8c1246f"
  and .last_commit == "ffee475c4c"
  and .review_group_count == 3
  and (.review_group_counts | map(.count) | add) == 20
  and (.review_group_counts | map(select(.review_group == "evidence_gate_projection_payloads").count)[0]) == 7
  and (.review_group_counts | map(select(.review_group == "readability_geometry_payloads").count)[0]) == 9
  and (.review_group_counts | map(select(.review_group == "ui_panel_label_payloads").count)[0]) == 4
  and (.commit_reviews | length) == 20
  and (.commit_reviews | all(.commit_level_review_complete == true))
  and (.commit_reviews | all(.unresolved == false))
  and .unresolved_commit_review_count == 0
  and .basin_spec_green == true
  and .basin_spec_failed_top_level_gate_count == 0
  and .rts_evidence_contract == "trnm_rts_evidence_v1"
  and .rts_evidence_bevy_runtime_adapter_gate == true
  and .evidence_guard_gate_count == 16
  and .evidence_guard_green_count == 16
  and .evidence_guard_all_green == true
  and (.evidence_guard_contracts | length) == 16
  and .source_of_truth_mentions_rts_evidence_crate == true
  and .source_of_truth_keeps_bevy_consumer_boundary == true
  and .rts_evidence_crate_boundary_reviewed == true
  and .rts_evidence_payloads_local_only == true
  and .evidence_crate_controls_playable_runtime == false
  and .playable_renderer_ownership_claimed == false
  and .live_bevy_renderer_behavior_moved_to_rts_evidence == false
  and .render_world_extraction_complete_claimed == false
  and .gpu_upload_claimed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
  and .beta_cohort_evidence_claimed == false
  and .production_ready_ui_claimed == false
  and .commercial_launch_evidence_claimed == false
  and .socket_opened == false
  and .hosted_service_claimed == false
  and .live_multiplayer_claimed == false
  and .sub_batch_5_local_review_complete == true
  and .sub_batch_5_exit_rule_satisfied == true
  and .sub_batch_6_unblocked_for_local_review == true
  and .batch_3_exit_rule_satisfied == false
  and .batch_4_unblocked_for_local_review == false
  and .next_sub_batch_id == "review_evidence_exposure_boundary"
  and .packet_integrity_status == "release_review_packet_integrity_green_with_public_launch_blockers"
  and .packet_integrity_failed_check_count == 0
  and .push_performed == false
  and .rebase_performed == false
  and .reset_performed == false
  and .squash_performed == false
  and .history_rewrite_performed == false
  and .upload_performed == false
  and .publish_performed == false
  and .external_action_performed == false
  and (.no_credit_boundary | contains("local RTS evidence crate boundary sub-batch 5 review only"))
' "$SUMMARY" >/dev/null

{
  printf '# Trillionnium World Review RTS Evidence Crate Batch\n\n'
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
  printf -- '- sub-batch 5 local review complete / exit rule: `%s` / `%s`\n' \
    "$(jq -r '.sub_batch_5_local_review_complete' "$SUMMARY")" \
    "$(jq -r '.sub_batch_5_exit_rule_satisfied' "$SUMMARY")"
  printf -- '- next sub-batch: `%s`\n\n' "$(jq -r '.next_sub_batch_id' "$SUMMARY")"
  printf '## Review Groups\n\n'
  jq -r '.review_group_counts[] | "- `\(.review_group)`: `\(.count)` commits, unresolved `\(.unresolved_count)`"' "$SUMMARY"
  printf '\n## Evidence Boundary\n\n'
  printf -- '- RTS evidence contract: `%s`\n' "$(jq -r '.rts_evidence_contract' "$SUMMARY")"
  printf -- '- Evidence guard gates green: `%s` / `%s`\n' \
    "$(jq -r '.evidence_guard_green_count' "$SUMMARY")" \
    "$(jq -r '.evidence_guard_gate_count' "$SUMMARY")"
  printf -- '- Evidence crate controls playable runtime: `%s`\n' "$(jq -r '.evidence_crate_controls_playable_runtime' "$SUMMARY")"
  printf -- '- Playable renderer ownership claimed: `%s`\n' "$(jq -r '.playable_renderer_ownership_claimed' "$SUMMARY")"
  printf -- '- Public/S5/beta/commercial claims: `%s` / `%s` / `%s` / `%s`\n' \
    "$(jq -r '.public_launch_ready_claimed' "$SUMMARY")" \
    "$(jq -r '.android_s5_real_device_claimed' "$SUMMARY")" \
    "$(jq -r '.beta_cohort_evidence_claimed' "$SUMMARY")" \
    "$(jq -r '.commercial_launch_evidence_claimed' "$SUMMARY")"
} >"$SUMMARY_MD"

printf 'TRILLIONNIUM_WORLD_REVIEW_RTS_EVIDENCE_CRATE_BATCH_GREEN %s %s\n' "$SUMMARY" "$SUMMARY_MD"
