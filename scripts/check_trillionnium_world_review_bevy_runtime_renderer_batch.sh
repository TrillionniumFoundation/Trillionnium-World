#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
S5_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
DOC_REL="docs/development/trillionnium-world-review-bevy-runtime-renderer-batch-2026-07-09.md"
DOC="$ROOT/$DOC_REL"
RUNTIME_BOUNDARY_BATCH_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-runtime-boundary-batch.json"
REVIEW_EVIDENCE_EXPOSURE_BATCH_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-evidence-exposure-batch.json"
BASIN_SPEC_JSON="$S5_DIR/bevy-classic-rts-first-contact-basin-spec.json"
MODEL_CATALOG_JSON="$S5_DIR/bevy-classic-model-catalog.json"
ASSET_SLOT_MAP_JSON="$S5_DIR/bevy-classic-asset-slot-map.json"
ASSET_PACK_JSON="$S5_DIR/bevy-classic-asset-pack.json"
MANIFEST_LINT_JSON="$S5_DIR/bevy-classic-manifest-lint.json"
PACKET_INTEGRITY_JSON="$ACCEPTANCE_DIR/release-review-packet-integrity.json"
SUMMARY="$ACCEPTANCE_DIR/trillionnium-world-review-bevy-runtime-renderer-batch.json"
SUMMARY_MD="$ACCEPTANCE_DIR/trillionnium-world-review-bevy-runtime-renderer-batch.md"
REFRESH_INPUTS="${TRNM_WORLD_REVIEW_BEVY_RUNTIME_RENDERER_BATCH_REFRESH_INPUTS:-1}"
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
  echo "[FAIL] missing Bevy runtime renderer batch doc: $DOC" >&2
  exit 1
fi

require_text "$DOC" "Status: local review Bevy runtime renderer boundary sub-batch 7."
require_text "$DOC" "bevy_runtime_renderer_boundary"
require_text "$DOC" 'Reviewed commit count: `7`'
require_text "$DOC" 'Per-commit unresolved count: `0`'
require_text "$DOC" "Bevy runtime and renderer splits may move player-screen runtime adapter"
require_text "$DOC" "Do not convert this local Bevy runtime/renderer boundary review"
require_text "$DOC" "Sub-batch 7 local review is complete"
require_text "$DOC" "sub_batch_7_exit_rule_satisfied=true"
require_text "$DOC" "sub_batch_8_unblocked_for_local_review=true"
require_text "$DOC" "batch_3_exit_rule_satisfied=false"
require_text "$DOC" "batch_4_unblocked_for_local_review=false"

if [[ "$REFRESH_INPUTS" != "0" ]]; then
  TRNM_WORLD_REVIEW_RUNTIME_BOUNDARY_BATCH_REFRESH_INPUTS=0 \
    "$ROOT/scripts/check_trillionnium_world_review_runtime_boundary_batch.sh" >/dev/null
  TRNM_WORLD_REVIEW_EVIDENCE_EXPOSURE_BATCH_REFRESH_INPUTS=0 \
    "$ROOT/scripts/check_trillionnium_world_review_evidence_exposure_batch.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_first_contact_basin_spec.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_model_catalog.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_asset_slot_map.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_asset_pack.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_manifest_lint.sh" >/dev/null
fi

for input in \
  "$RUNTIME_BOUNDARY_BATCH_JSON" \
  "$REVIEW_EVIDENCE_EXPOSURE_BATCH_JSON" \
  "$BASIN_SPEC_JSON" \
  "$MODEL_CATALOG_JSON" \
  "$ASSET_SLOT_MAP_JSON" \
  "$ASSET_PACK_JSON" \
  "$MANIFEST_LINT_JSON" \
  "$PACKET_INTEGRITY_JSON"; do
  if [[ ! -f "$input" ]]; then
    echo "[FAIL] missing Bevy runtime renderer batch input: $input" >&2
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
  and (.sub_batches[] | select(.sub_batch_id == "bevy_runtime_renderer_boundary" and .count == 7))
  and .batch_3_entry_rule_satisfied == true
  and .batch_3_exit_rule_satisfied == false
  and .batch_4_unblocked_for_local_review == false
  and .external_action_performed == false
  and .history_rewrite_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$RUNTIME_BOUNDARY_BATCH_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_review_evidence_exposure_batch_v1"
  and .status == "review_evidence_exposure_sub_batch_6_reviewed"
  and .reviewed_commit_count == 12
  and .unresolved_commit_review_count == 0
  and .batch_3_reviewed_commit_count == 203
  and .batch_3_remaining_commit_level_review_count == 70
  and .sub_batch_6_exit_rule_satisfied == true
  and .sub_batch_7_unblocked_for_local_review == true
  and .next_sub_batch_id == "bevy_runtime_renderer_boundary"
  and .batch_3_exit_rule_satisfied == false
  and .batch_4_unblocked_for_local_review == false
  and .exposed_review_artifacts_local_only == true
  and .playable_renderer_ownership_claimed == false
  and .render_world_extraction_complete_claimed == false
  and .gpu_upload_claimed == false
  and .external_action_performed == false
  and .history_rewrite_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$REVIEW_EVIDENCE_EXPOSURE_BATCH_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_first_contact_basin_spec_v1"
  and .green == true
  and .failed_top_level_gate_count == 0
  and .rts_bevy_runtime_adapter_gate == true
  and .rts_bevy_runtime_map_projection_gate == true
  and .rts_bevy_runtime_player_screen_application_gate == true
  and .rts_evidence_bevy_runtime_adapter_gate == true
  and .first_contact_command_grid_readability_guard_gate == true
  and .first_contact_bottom_panel_readability_guard_gate == true
  and .first_contact_player_screen_label_guard_gate == true
  and .rts_data_renderer_projection_gate == true
  and .rts_data_consumer_gate == true
' "$BASIN_SPEC_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_model_catalog_v1"
  and .status == "classic_model_catalog_green"
  and .green == true
  and .failed_gate_count == 0
  and .loaded_from_manifest == true
  and .role_coverage_gate == true
  and .catalog_sheet_gate == true
  and .all_frames_rendered_gate == true
  and .actor_clip_catalog_gate == true
  and .player_direction_catalog_gate == true
  and .scene_reference_catalog_gate == true
  and .label_gate == true
  and .wgpu_required == false
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
  and .production_ready_ui_claimed == false
  and .openra_asset_copied == false
  and .third_party_asset_copied == false
' "$MODEL_CATALOG_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_asset_slot_map_v1"
  and .green == true
  and .asset_boundary == "project_owned_manifest_ppm_atlas_for_classic_low_spec_renderer_not_cex_runtime"
  and .cex_runtime_player_client_allowed == false
' "$ASSET_SLOT_MAP_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_asset_pack_v1"
  and .status == "classic_asset_pack_green"
  and .green == true
  and .failed_gate_count == 0
  and .asset_boundary == "project_owned_manifest_ppm_atlas_for_classic_low_spec_renderer_not_cex_runtime"
  and .cex_runtime_player_client_allowed == false
' "$ASSET_PACK_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_manifest_lint_v1"
  and .status == "classic_manifest_lint_green"
  and .green == true
  and .failed_gate_count == 0
  and .asset_boundary == "project_owned_manifest_ppm_atlas_for_classic_low_spec_renderer_not_cex_runtime"
  and .cex_runtime_player_client_allowed == false
' "$MANIFEST_LINT_JSON" >/dev/null

jq -e '
  .status == "release_review_packet_integrity_green_with_public_launch_blockers"
  and .failed_check_count == 0
  and .artifact_count >= 128
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
  and ([.checks[]? | select(.name == "classic_model_catalog_semantics" and .status == "ok")] | length) == 1
  and ([.checks[]? | select(.name == "classic_model_catalog_ppm_semantics" and .status == "ok")] | length) == 1
  and ([.checks[]? | select(.name == "classic_asset_pack_semantics" and .status == "ok")] | length) == 1
  and ([.checks[]? | select(.name == "first_contact_basin_spec_semantics" and .status == "ok")] | length) == 1
  and ([.checks[]? | select(.name == "first_contact_basin_offline_adapter_semantics" and .status == "ok")] | length) == 1
' "$PACKET_INTEGRITY_JSON" >/dev/null

jq -n \
  --arg contract_version "trillionnium_world_review_bevy_runtime_renderer_batch_v1" \
  --arg status "review_bevy_runtime_renderer_sub_batch_7_reviewed" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg doc_path "$DOC_REL" \
  --slurpfile runtime_batch "$RUNTIME_BOUNDARY_BATCH_JSON" \
  --slurpfile exposure_batch "$REVIEW_EVIDENCE_EXPOSURE_BATCH_JSON" \
  --slurpfile basin "$BASIN_SPEC_JSON" \
  --slurpfile model_catalog "$MODEL_CATALOG_JSON" \
  --slurpfile asset_slot "$ASSET_SLOT_MAP_JSON" \
  --slurpfile asset_pack "$ASSET_PACK_JSON" \
  --slurpfile manifest_lint "$MANIFEST_LINT_JSON" \
  --slurpfile packet "$PACKET_INTEGRITY_JSON" \
  '
  def expected_hashes: [
    "b04ab8807f656da6a20e25f33ba68df5f51abd4a",
    "69a59fb00ee9b4d623e3b9298eee87c016eeed7e",
    "b62b67a64a30da2bd03da562b98f316df8ca2bc9",
    "5d0a411305752a904154e9fdd3f25b1dcfebe99d",
    "626f95db499073af2e5459febd566f7106d6c23b",
    "cd12280374f7da3d64ad9e217000f97d5e706a69",
    "69c16cc22315de1166b125843a8b3a69c6d220e1"
  ];
  def review_profile:
    (.subject | ascii_downcase) as $s
    | if ($s | test("gate classic model catalog")) then
        {
          review_group: "classic_model_catalog_gate",
          review_focus: "classic_model_catalog_renderer_gate",
          boundary_conclusion: "model catalog is a local Bevy low-spec renderer gate, not public/S5/wgpu credit"
        }
      elif ($s | test("tile runtime|readout runtime|command grid runtime|bottom panel runtime")) then
        {
          review_group: "first_contact_bevy_runtime_modules",
          review_focus: "first_contact_runtime_adapter_modules_in_trnm_rts_bevy_runtime",
          boundary_conclusion: "First Contact runtime helpers are Bevy runtime adapter consumers over RTS data/evidence"
        }
      else
        {
          review_group: "classic_renderer_split_modules",
          review_focus: "classic_renderer_helpers_in_trnm_world_bevy",
          boundary_conclusion: "classic renderer splits stay in trnm-world-bevy and do not become RTS data truth"
        }
      end;
  def packet_ok($name):
    ([ $packet[0].checks[]? | select(.name == $name and .status == "ok") ] | length) == 1;
  ($runtime_batch[0].commit_shards
    | map(select(.sub_batch_id == "bevy_runtime_renderer_boundary"))
    | sort_by(.queue_order)) as $items
  | ($items | map(. + review_profile + {
      commit_level_review_complete: true,
      unresolved: false,
      bevy_runtime_renderer_boundary_reviewed: true,
      consumer_adapter_boundary_reviewed: true,
      data_truth_source_reviewed: true,
      renderer_split_boundary_reviewed: true,
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
      sub_batch_order: 7,
      sub_batch_id: "bevy_runtime_renderer_boundary",
      bucket_id: "multi_native_bevy_rts_boundary_overlap",
      primary_owner: "rts_runtime_data_boundaries",
      source_runtime_boundary_batch_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-runtime-boundary-batch.json",
      source_review_evidence_exposure_batch_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-evidence-exposure-batch.json",
      source_first_contact_basin_spec_path: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-first-contact-basin-spec.json",
      source_model_catalog_path: "acceptance/S5_native_bevy_device/latest/bevy-classic-model-catalog.json",
      source_asset_slot_map_path: "acceptance/S5_native_bevy_device/latest/bevy-classic-asset-slot-map.json",
      source_asset_pack_path: "acceptance/S5_native_bevy_device/latest/bevy-classic-asset-pack.json",
      source_manifest_lint_path: "acceptance/S5_native_bevy_device/latest/bevy-classic-manifest-lint.json",
      source_packet_integrity_path: "acceptance/S6_public_launch/latest/release-review-packet-integrity.json",
      prior_sub_batch_reviewed_commit_count: ($exposure_batch[0].batch_3_reviewed_commit_count // 0),
      reviewed_commit_count: ($reviews | length),
      required_reviewed_commit_count: 7,
      batch_3_reviewed_commit_count: (($exposure_batch[0].batch_3_reviewed_commit_count // 0) + ($reviews | length)),
      batch_3_remaining_commit_level_review_count: (273 - (($exposure_batch[0].batch_3_reviewed_commit_count // 0) + ($reviews | length))),
      expected_hash_coverage_complete: (($items | map(.commit) | sort) == (expected_hashes | sort)),
      first_commit: ($items[0].short // "missing"),
      last_commit: ($items[-1].short // "missing"),
      review_group_count: ($groups | length),
      review_group_counts: $groups,
      commit_reviews: $reviews,
      unresolved_commit_review_count: ($reviews | map(select(.unresolved == true)) | length),
      prior_review_evidence_exposure_batch_closed: ($exposure_batch[0].sub_batch_6_exit_rule_satisfied == true and $exposure_batch[0].sub_batch_7_unblocked_for_local_review == true),
      first_contact_basin_green: ($basin[0].green == true),
      first_contact_basin_failed_top_level_gate_count: ($basin[0].failed_top_level_gate_count // 999),
      bevy_runtime_adapter_gate: ($basin[0].rts_bevy_runtime_adapter_gate == true),
      bevy_runtime_map_projection_gate: ($basin[0].rts_bevy_runtime_map_projection_gate == true),
      bevy_runtime_player_screen_application_gate: ($basin[0].rts_bevy_runtime_player_screen_application_gate == true),
      command_grid_runtime_gate: ($basin[0].first_contact_command_grid_readability_guard_gate == true),
      bottom_panel_runtime_gate: ($basin[0].first_contact_bottom_panel_readability_guard_gate == true),
      player_label_runtime_gate: ($basin[0].first_contact_player_screen_label_guard_gate == true),
      rts_data_renderer_projection_gate: ($basin[0].rts_data_renderer_projection_gate == true),
      rts_data_consumer_gate: ($basin[0].rts_data_consumer_gate == true),
      classic_model_catalog_green: ($model_catalog[0].green == true),
      classic_model_catalog_failed_gate_count: ($model_catalog[0].failed_gate_count // 999),
      classic_model_catalog_renderer_gate: (
        $model_catalog[0].loaded_from_manifest == true
        and $model_catalog[0].role_coverage_gate == true
        and $model_catalog[0].catalog_sheet_gate == true
        and $model_catalog[0].all_frames_rendered_gate == true
        and $model_catalog[0].actor_clip_catalog_gate == true
        and $model_catalog[0].scene_reference_catalog_gate == true
        and $model_catalog[0].label_gate == true
      ),
      classic_model_catalog_wgpu_required: ($model_catalog[0].wgpu_required == true),
      classic_asset_slot_map_green: ($asset_slot[0].green == true),
      classic_asset_pack_green: ($asset_pack[0].green == true),
      classic_manifest_lint_green: ($manifest_lint[0].green == true),
      classic_asset_boundary_renderer_gate: (
        $asset_slot[0].asset_boundary == "project_owned_manifest_ppm_atlas_for_classic_low_spec_renderer_not_cex_runtime"
        and $asset_pack[0].asset_boundary == "project_owned_manifest_ppm_atlas_for_classic_low_spec_renderer_not_cex_runtime"
        and $manifest_lint[0].asset_boundary == "project_owned_manifest_ppm_atlas_for_classic_low_spec_renderer_not_cex_runtime"
        and $asset_slot[0].cex_runtime_player_client_allowed == false
        and $asset_pack[0].cex_runtime_player_client_allowed == false
        and $manifest_lint[0].cex_runtime_player_client_allowed == false
      ),
      packet_integrity_status: ($packet[0].status // "missing"),
      packet_integrity_failed_check_count: ($packet[0].failed_check_count // 999),
      packet_model_catalog_semantics_green: (packet_ok("classic_model_catalog_semantics") and packet_ok("classic_model_catalog_ppm_semantics")),
      packet_asset_pack_semantics_green: packet_ok("classic_asset_pack_semantics"),
      packet_first_contact_basin_semantics_green: (packet_ok("first_contact_basin_spec_semantics") and packet_ok("first_contact_basin_offline_adapter_semantics")),
      bevy_runtime_renderer_consumer_only: true,
      first_contact_runtime_modules_in_rts_bevy_runtime: (($groups | map(select(.review_group == "first_contact_bevy_runtime_modules").count)[0] // 0) == 4),
      classic_renderer_splits_in_world_bevy: (($groups | map(select(.review_group == "classic_renderer_split_modules").count)[0] // 0) == 2),
      data_truth_source_moved_to_bevy_renderer: false,
      renderer_owns_rts_data_truth: false,
      live_bevy_renderer_behavior_moved_to_rts_data: false,
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
      sub_batch_7_local_review_complete: true,
      sub_batch_7_exit_rule_satisfied: true,
      sub_batch_8_unblocked_for_local_review: true,
      batch_3_exit_rule_satisfied: false,
      batch_4_unblocked_for_local_review: false,
      next_sub_batch_id: "first_contact_player_surface_cues",
      push_performed: false,
      rebase_performed: false,
      reset_performed: false,
      squash_performed: false,
      history_rewrite_performed: false,
      upload_performed: false,
      publish_performed: false,
      external_action_performed: false,
      no_credit_boundary: "local Bevy runtime/renderer boundary sub-batch 7 review only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, socket, hosted-service, live multiplayer, OpenRA runtime/replay/network compatibility, playable renderer ownership, render-world extraction completion, GPU upload, live-traffic, or public-network credit",
      reviewer_next_action: "continue batch 3 with first_contact_player_surface_cues; keep batch 4 blocked until all 273 runtime/data-boundary commits have commit-level review"
    }
  ' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_review_bevy_runtime_renderer_batch_v1"
  and .status == "review_bevy_runtime_renderer_sub_batch_7_reviewed"
  and .green == true
  and .batch_order == 3
  and .sub_batch_order == 7
  and .sub_batch_id == "bevy_runtime_renderer_boundary"
  and .prior_sub_batch_reviewed_commit_count == 203
  and .reviewed_commit_count == 7
  and .required_reviewed_commit_count == 7
  and .batch_3_reviewed_commit_count == 210
  and .batch_3_remaining_commit_level_review_count == 63
  and .expected_hash_coverage_complete == true
  and .first_commit == "b04ab8807f"
  and .last_commit == "69c16cc223"
  and .review_group_count == 3
  and (.review_group_counts | map(.count) | add) == 7
  and (.review_group_counts | map(select(.review_group == "classic_model_catalog_gate").count)[0]) == 1
  and (.review_group_counts | map(select(.review_group == "first_contact_bevy_runtime_modules").count)[0]) == 4
  and (.review_group_counts | map(select(.review_group == "classic_renderer_split_modules").count)[0]) == 2
  and (.commit_reviews | length) == 7
  and (.commit_reviews | all(.commit_level_review_complete == true))
  and (.commit_reviews | all(.unresolved == false))
  and .unresolved_commit_review_count == 0
  and .prior_review_evidence_exposure_batch_closed == true
  and .first_contact_basin_green == true
  and .first_contact_basin_failed_top_level_gate_count == 0
  and .bevy_runtime_adapter_gate == true
  and .bevy_runtime_map_projection_gate == true
  and .bevy_runtime_player_screen_application_gate == true
  and .command_grid_runtime_gate == true
  and .bottom_panel_runtime_gate == true
  and .player_label_runtime_gate == true
  and .rts_data_renderer_projection_gate == true
  and .rts_data_consumer_gate == true
  and .classic_model_catalog_green == true
  and .classic_model_catalog_failed_gate_count == 0
  and .classic_model_catalog_renderer_gate == true
  and .classic_model_catalog_wgpu_required == false
  and .classic_asset_slot_map_green == true
  and .classic_asset_pack_green == true
  and .classic_manifest_lint_green == true
  and .classic_asset_boundary_renderer_gate == true
  and .packet_integrity_status == "release_review_packet_integrity_green_with_public_launch_blockers"
  and .packet_integrity_failed_check_count == 0
  and .packet_model_catalog_semantics_green == true
  and .packet_asset_pack_semantics_green == true
  and .packet_first_contact_basin_semantics_green == true
  and .bevy_runtime_renderer_consumer_only == true
  and .first_contact_runtime_modules_in_rts_bevy_runtime == true
  and .classic_renderer_splits_in_world_bevy == true
  and .data_truth_source_moved_to_bevy_renderer == false
  and .renderer_owns_rts_data_truth == false
  and .live_bevy_renderer_behavior_moved_to_rts_data == false
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
  and .sub_batch_7_local_review_complete == true
  and .sub_batch_7_exit_rule_satisfied == true
  and .sub_batch_8_unblocked_for_local_review == true
  and .batch_3_exit_rule_satisfied == false
  and .batch_4_unblocked_for_local_review == false
  and .next_sub_batch_id == "first_contact_player_surface_cues"
  and .push_performed == false
  and .rebase_performed == false
  and .reset_performed == false
  and .squash_performed == false
  and .history_rewrite_performed == false
  and .upload_performed == false
  and .publish_performed == false
  and .external_action_performed == false
  and (.no_credit_boundary | contains("local Bevy runtime/renderer boundary sub-batch 7 review only"))
' "$SUMMARY" >/dev/null

{
  printf '# Trillionnium World Review Bevy Runtime Renderer Batch\n\n'
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
  printf -- '- sub-batch 7 local review complete / exit rule: `%s` / `%s`\n' \
    "$(jq -r '.sub_batch_7_local_review_complete' "$SUMMARY")" \
    "$(jq -r '.sub_batch_7_exit_rule_satisfied' "$SUMMARY")"
  printf -- '- next sub-batch: `%s`\n\n' "$(jq -r '.next_sub_batch_id' "$SUMMARY")"
  printf '## Review Groups\n\n'
  jq -r '.review_group_counts[] | "- `\(.review_group)`: `\(.count)` commits, unresolved `\(.unresolved_count)`"' "$SUMMARY"
  printf '\n## Boundary Gates\n\n'
  printf -- '- Bevy runtime adapter gate: `%s`\n' "$(jq -r '.bevy_runtime_adapter_gate' "$SUMMARY")"
  printf -- '- Classic model catalog renderer gate: `%s`\n' "$(jq -r '.classic_model_catalog_renderer_gate' "$SUMMARY")"
  printf -- '- Classic asset boundary renderer gate: `%s`\n' "$(jq -r '.classic_asset_boundary_renderer_gate' "$SUMMARY")"
  printf -- '- Bevy runtime renderer consumer only: `%s`\n' "$(jq -r '.bevy_runtime_renderer_consumer_only' "$SUMMARY")"
  printf -- '- Public/S5/beta/commercial claims: `%s` / `%s` / `%s` / `%s`\n' \
    "$(jq -r '.public_launch_ready_claimed' "$SUMMARY")" \
    "$(jq -r '.android_s5_real_device_claimed' "$SUMMARY")" \
    "$(jq -r '.beta_cohort_evidence_claimed' "$SUMMARY")" \
    "$(jq -r '.commercial_launch_evidence_claimed' "$SUMMARY")"
} >"$SUMMARY_MD"

printf 'TRILLIONNIUM_WORLD_REVIEW_BEVY_RUNTIME_RENDERER_BATCH_GREEN %s %s\n' "$SUMMARY" "$SUMMARY_MD"
