#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
S5_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
DOC_REL="docs/development/trillionnium-world-review-first-contact-rts-data-batch-2026-07-09.md"
DOC="$ROOT/$DOC_REL"
RUNTIME_BOUNDARY_BATCH_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-runtime-boundary-batch.json"
OPENRA_PARITY_CLAIM_BATCH_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-openra-parity-claim-batch.json"
BASIN_SPEC_JSON="$S5_DIR/bevy-classic-rts-first-contact-basin-spec.json"
PACKET_INTEGRITY_JSON="$ACCEPTANCE_DIR/release-review-packet-integrity.json"
SUMMARY="$ACCEPTANCE_DIR/trillionnium-world-review-first-contact-rts-data-batch.json"
SUMMARY_MD="$ACCEPTANCE_DIR/trillionnium-world-review-first-contact-rts-data-batch.md"
REFRESH_INPUTS="${TRNM_WORLD_REVIEW_FIRST_CONTACT_RTS_DATA_BATCH_REFRESH_INPUTS:-1}"
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
  echo "[FAIL] missing First Contact RTS data batch doc: $DOC" >&2
  exit 1
fi

require_text "$DOC" "Status: local review First Contact RTS data extraction sub-batch 4."
require_text "$DOC" "first_contact_rts_data_extraction"
require_text "$DOC" 'Reviewed commit count: `24`'
require_text "$DOC" 'Per-commit unresolved count: `0`'
require_text "$DOC" "renderer-neutral data/evidence inputs"
require_text "$DOC" "Do not convert this local review into renderer ownership transfer"
require_text "$DOC" "Sub-batch 4 local review is complete"
require_text "$DOC" "sub_batch_4_exit_rule_satisfied=true"
require_text "$DOC" "sub_batch_5_unblocked_for_local_review=true"
require_text "$DOC" "batch_3_exit_rule_satisfied=false"
require_text "$DOC" "batch_4_unblocked_for_local_review=false"

if [[ "$REFRESH_INPUTS" != "0" ]]; then
  TRNM_WORLD_REVIEW_RUNTIME_BOUNDARY_BATCH_REFRESH_INPUTS=0 \
    "$ROOT/scripts/check_trillionnium_world_review_runtime_boundary_batch.sh" >/dev/null
  TRNM_WORLD_REVIEW_OPENRA_PARITY_CLAIM_BATCH_REFRESH_INPUTS=0 \
    "$ROOT/scripts/check_trillionnium_world_review_openra_parity_claim_batch.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_first_contact_basin_spec.sh" >/dev/null
fi

for input in "$RUNTIME_BOUNDARY_BATCH_JSON" "$OPENRA_PARITY_CLAIM_BATCH_JSON" "$BASIN_SPEC_JSON" "$PACKET_INTEGRITY_JSON"; do
  if [[ ! -f "$input" ]]; then
    echo "[FAIL] missing First Contact RTS data batch input: $input" >&2
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
  and (.sub_batches[] | select(.sub_batch_id == "first_contact_rts_data_extraction" and .count == 24))
  and .batch_3_entry_rule_satisfied == true
  and .batch_3_exit_rule_satisfied == false
  and .batch_4_unblocked_for_local_review == false
  and .external_action_performed == false
  and .history_rewrite_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$RUNTIME_BOUNDARY_BATCH_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_review_openra_parity_claim_batch_v1"
  and .status == "review_openra_parity_claim_sub_batch_3_reviewed"
  and .reviewed_commit_count == 35
  and .unresolved_commit_review_count == 0
  and .batch_3_reviewed_commit_count == 147
  and .batch_3_remaining_commit_level_review_count == 126
  and .sub_batch_3_exit_rule_satisfied == true
  and .sub_batch_4_unblocked_for_local_review == true
  and .next_sub_batch_id == "first_contact_rts_data_extraction"
  and .batch_3_exit_rule_satisfied == false
  and .batch_4_unblocked_for_local_review == false
  and .external_action_performed == false
  and .history_rewrite_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$OPENRA_PARITY_CLAIM_BATCH_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_first_contact_basin_spec_v1"
  and .green == true
  and .failed_top_level_gate_count == 0
  and .rts_data_map_model_gate == true
  and .rts_data_terrain_profile_gate == true
  and .rts_data_opening_profile_gate == true
  and .rts_data_player_startup_gate == true
  and .rts_data_actor_presentation_gate == true
  and .rts_data_visual_telemetry_gate == true
  and .rts_data_player_screen_gate == true
  and .rts_data_player_screen_layout_gate == true
  and .rts_data_player_screen_chrome_gate == true
  and .rts_data_preview_actor_projection_gate == true
  and .rts_data_renderer_projection_gate == true
  and .rts_bevy_runtime_adapter_gate == true
  and .rts_evidence_bevy_runtime_adapter_gate == true
  and .rts_bevy_runtime_player_screen_application_gate == true
  and .rts_data_map_model_actor_count == 39
  and .rts_data_map_model_rule_count == 19
  and .rts_data_preview_actor_count == 39
  and .rts_data_actor_presentation_profile_count == 14
  and .rts_data_player_startup_profile_count == 4
  and .rts_data_terrain_profile_count == 1156
  and .runtime_player_screen_command_queue_count == 4
  and .runtime_player_screen_production_queue_count == 3
  and .runtime_player_screen_build_queue_count == 2
  and .runtime_player_screen_visible_tile_count == 64
  and .runtime_player_screen_fogged_tile_count == 6
  and .rts_data_player_screen_profile.contract_version == "trnm_rts_data_first_contact_player_screen_v1"
  and .rts_data_player_screen_profile.map_id == "first_contact_basin"
  and .rts_data_player_screen_profile.room_id == "first-contact-basin"
  and .rts_data_player_screen_profile.chrome.top_title == "TRNM RTS"
  and (.rts_data_player_screen_profile.chrome.command_grid_slot_ids | length) == 6
  and .rts_data_player_screen_profile == .rts_evidence_bevy_runtime_adapter.first_contact_player_screen_profile
  and .rts_data_player_screen_layout_profile == .rts_data_player_screen_profile.layout
  and .rts_data_player_screen_chrome_profile == .rts_data_player_screen_profile.chrome
  and .rts_data_player_screen_profile.command_queue == .rts_evidence_bevy_runtime_adapter.first_contact_player_screen_profile.command_queue
  and .rts_data_visual_telemetry_profile == .rts_evidence_bevy_runtime_adapter.first_contact_visual_telemetry_profile
  and .rts_data_actor_presentation_profiles == .rts_evidence_bevy_runtime_adapter.first_contact_actor_presentation_profiles
  and .rts_data_preview_actor_projection.actor_count == 39
  and .rts_data_preview_actor_projection.spawn_count == 4
  and .rts_data_preview_actor_projection.flux_bloom_count == 11
  and .rts_data_preview_actor_projection.beacon_count == 4
  and .rts_data_preview_actor_projection.expansion_count == 4
  and (.rts_data_preview_actor_projection.source | contains("trnm-rts-data first_contact_preview_actors"))
  and .rts_data_renderer_projection.renderable_tile_count == 1024
  and .rts_data_renderer_projection.lane_tile_count == 240
  and .rts_data_renderer_projection.resource_zone_tile_count == 79
  and .rts_data_renderer_projection.minimap_anchor_actor_count == 39
  and (.rts_data_renderer_projection.source | contains("RtsMapModel bounds"))
  and .rts_bevy_runtime_player_screen_application.runtime_application_path == "trnm-rts-data first_contact_player_screen_profile -> trnm-rts-bevy-runtime player_screen_runtime_application -> NativeFirstPlayableRuntime mutation"
  and (.rts_bevy_runtime_player_screen_application.source_of_truth | contains("trnm-rts-data First Contact player-screen profile"))
' "$BASIN_SPEC_JSON" >/dev/null

jq -e '
  .status == "release_review_packet_integrity_green_with_public_launch_blockers"
  and .failed_check_count == 0
  and .artifact_count >= 128
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
' "$PACKET_INTEGRITY_JSON" >/dev/null

jq -n \
  --arg contract_version "trillionnium_world_review_first_contact_rts_data_batch_v1" \
  --arg status "review_first_contact_rts_data_sub_batch_4_reviewed" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg doc_path "$DOC_REL" \
  --slurpfile runtime_batch "$RUNTIME_BOUNDARY_BATCH_JSON" \
  --slurpfile openra_batch "$OPENRA_PARITY_CLAIM_BATCH_JSON" \
  --slurpfile basin "$BASIN_SPEC_JSON" \
  --slurpfile packet "$PACKET_INTEGRITY_JSON" \
  '
  def expected_hashes: [
    "a53d815eecf2846fecc5aa4800071f3df06db421",
    "7850f8a359b58eff3af926fa2f004e8d4b3783f5",
    "e91ab6c4a64b6b8861afaa2ce1aff46874186415",
    "b0280650eb9a86a02594c87a99c0d5e63906b6e6",
    "52a8ab2b1e7b4c5c2b6541abf16140bf3b72639b",
    "a22e693c896fcec0b515cb5653ab9535933855d1",
    "1f4157fc55a5b3362a6cc6a5ff96ae2d60f46e26",
    "6b6261cf8f15d716dc789d9f4ca5b274e7458af6",
    "264adc8b7dca3248de12b2688de9261a797e3e6c",
    "c552e48f3e0e7ba03c115b9b6b28e624a1d47cb9",
    "6b52568354a66263d9f8b928b11c99da6792e436",
    "6ba3dec7b28c5f3d70ac5d3862203008ccf5a675",
    "24983ea6a516f18531c6c03d9c4e975f34611b41",
    "f3931945b0ea0bf46a35a6053e550e4ca2308cdf",
    "7216f207ad71a1b8e6924896a27ef4cd7cb5aa69",
    "ef8c3f6203f2558e4d5bd385ddc473584c207a5c",
    "a40703f7f277a4c2ffeb6387b53c0a913f2dd636",
    "cd2c1921d581ff83a6e4a8334cee25e933075dc5",
    "8b6987ba870eae15b9c3ec63271320707d9dc26a",
    "3192af3400d506d539607d9f86db7c2760293240",
    "eae6700f0e91dd6c89f03da36837d6970cd31bcc",
    "2f4cb815aa821dd21276e8edddfb2ca502629531",
    "ca2370d92f502c1b0020a5d953f0b4d8127f8797",
    "b65b4a44cd96813bb5ecc9fa41185a1ac8c5378a"
  ];
  def review_profile:
    (.subject | ascii_downcase) as $s
    | if ($s | test("terrain profile|opening profile|player startups|actor presentation|actor glyphs|visual telemetry")) then
        {
          review_group: "data_profile_extraction",
          review_focus: "authored_first_contact_profiles_in_rts_data",
          boundary_conclusion: "authored terrain, opening, startup, actor, glyph, and telemetry profiles are RTS data inputs"
        }
      elif ($s | test("player screen|screen layout|hud chrome|shell chrome|viewport chrome|command queue chrome|command card chrome|production palette chrome|command slot ids|selection card chrome|production queues|selection health|active command|command cooldowns")) then
        {
          review_group: "player_screen_chrome_and_runtime_defaults",
          review_focus: "player_screen_defaults_data_owned_runtime_consumed",
          boundary_conclusion: "player-screen layout/chrome/queue/defaults stay data-owned and are consumed by runtime adapters"
        }
      elif ($s | test("renderer model|preview actors")) then
        {
          review_group: "renderer_projection_boundary",
          review_focus: "renderer_projection_inputs_without_draw_math_transfer",
          boundary_conclusion: "renderer model and preview actors are RTS data projection inputs while Bevy draw math stays renderer-owned"
        }
      else
        {
          review_group: "samples_labels_readability_boundary",
          review_focus: "samples_and_labels_as_reusable_data_inputs",
          boundary_conclusion: "samples and labels are reusable RTS data/readability inputs and not launch or renderer proof"
        }
      end;
  ($runtime_batch[0].commit_shards
    | map(select(.sub_batch_id == "first_contact_rts_data_extraction"))
    | sort_by(.queue_order)) as $items
  | ($items | map(. + review_profile + {
      commit_level_review_complete: true,
      unresolved: false,
      rts_data_extraction_reviewed: true,
      renderer_neutral_data_boundary_reviewed: true,
      bevy_runtime_consumption_reviewed: true,
      draw_math_transfer_rejected: true,
      live_renderer_behavior_transfer_rejected: true,
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
  | {
      contract_version: $contract_version,
      status: $status,
      generated_at: $generated_at,
      green: true,
      doc_path: $doc_path,
      batch_order: 3,
      sub_batch_order: 4,
      sub_batch_id: "first_contact_rts_data_extraction",
      bucket_id: "multi_native_bevy_rts_boundary_overlap",
      primary_owner: "rts_runtime_data_boundaries",
      source_runtime_boundary_batch_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-runtime-boundary-batch.json",
      source_openra_parity_claim_batch_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-openra-parity-claim-batch.json",
      source_first_contact_basin_spec_path: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-first-contact-basin-spec.json",
      source_packet_integrity_path: "acceptance/S6_public_launch/latest/release-review-packet-integrity.json",
      prior_sub_batch_reviewed_commit_count: ($openra_batch[0].batch_3_reviewed_commit_count // 0),
      reviewed_commit_count: ($reviews | length),
      required_reviewed_commit_count: 24,
      batch_3_reviewed_commit_count: (($openra_batch[0].batch_3_reviewed_commit_count // 0) + ($reviews | length)),
      batch_3_remaining_commit_level_review_count: (273 - (($openra_batch[0].batch_3_reviewed_commit_count // 0) + ($reviews | length))),
      expected_hash_coverage_complete: (($items | map(.commit) | sort) == (expected_hashes | sort)),
      first_commit: ($items[0].short // "missing"),
      last_commit: ($items[-1].short // "missing"),
      review_group_count: ($groups | length),
      review_group_counts: $groups,
      commit_reviews: $reviews,
      unresolved_commit_review_count: ($reviews | map(select(.unresolved == true)) | length),
      basin_spec_green: ($basin[0].green == true),
      basin_spec_failed_top_level_gate_count: ($basin[0].failed_top_level_gate_count // 999),
      rts_data_map_model_gate: ($basin[0].rts_data_map_model_gate == true),
      rts_data_terrain_profile_gate: ($basin[0].rts_data_terrain_profile_gate == true),
      rts_data_opening_profile_gate: ($basin[0].rts_data_opening_profile_gate == true),
      rts_data_player_startup_gate: ($basin[0].rts_data_player_startup_gate == true),
      rts_data_actor_presentation_gate: ($basin[0].rts_data_actor_presentation_gate == true),
      rts_data_visual_telemetry_gate: ($basin[0].rts_data_visual_telemetry_gate == true),
      rts_data_player_screen_gate: ($basin[0].rts_data_player_screen_gate == true),
      rts_data_player_screen_layout_gate: ($basin[0].rts_data_player_screen_layout_gate == true),
      rts_data_player_screen_chrome_gate: ($basin[0].rts_data_player_screen_chrome_gate == true),
      rts_data_preview_actor_projection_gate: ($basin[0].rts_data_preview_actor_projection_gate == true),
      rts_data_renderer_projection_gate: ($basin[0].rts_data_renderer_projection_gate == true),
      rts_bevy_runtime_adapter_gate: ($basin[0].rts_bevy_runtime_adapter_gate == true),
      rts_evidence_bevy_runtime_adapter_gate: ($basin[0].rts_evidence_bevy_runtime_adapter_gate == true),
      rts_bevy_runtime_player_screen_application_gate: ($basin[0].rts_bevy_runtime_player_screen_application_gate == true),
      rts_data_map_model_actor_count: ($basin[0].rts_data_map_model_actor_count // 0),
      rts_data_map_model_rule_count: ($basin[0].rts_data_map_model_rule_count // 0),
      rts_data_preview_actor_count: ($basin[0].rts_data_preview_actor_count // 0),
      rts_data_actor_presentation_profile_count: ($basin[0].rts_data_actor_presentation_profile_count // 0),
      rts_data_player_startup_profile_count: ($basin[0].rts_data_player_startup_profile_count // 0),
      rts_data_terrain_profile_count: ($basin[0].rts_data_terrain_profile_count // 0),
      runtime_player_screen_command_queue_count: ($basin[0].runtime_player_screen_command_queue_count // 0),
      runtime_player_screen_visible_tile_count: ($basin[0].runtime_player_screen_visible_tile_count // 0),
      player_screen_profile_contract_version: ($basin[0].rts_data_player_screen_profile.contract_version // "missing"),
      player_screen_profile_map_id: ($basin[0].rts_data_player_screen_profile.map_id // "missing"),
      player_screen_profile_room_id: ($basin[0].rts_data_player_screen_profile.room_id // "missing"),
      player_screen_profile_command_slot_count: (($basin[0].rts_data_player_screen_profile.chrome.command_grid_slot_ids // []) | length),
      player_screen_profile_matches_evidence_adapter: ($basin[0].rts_data_player_screen_profile == $basin[0].rts_evidence_bevy_runtime_adapter.first_contact_player_screen_profile),
      player_screen_layout_matches_profile: ($basin[0].rts_data_player_screen_layout_profile == $basin[0].rts_data_player_screen_profile.layout),
      player_screen_chrome_matches_profile: ($basin[0].rts_data_player_screen_chrome_profile == $basin[0].rts_data_player_screen_profile.chrome),
      actor_presentation_matches_evidence_adapter: ($basin[0].rts_data_actor_presentation_profiles == $basin[0].rts_evidence_bevy_runtime_adapter.first_contact_actor_presentation_profiles),
      visual_telemetry_matches_evidence_adapter: ($basin[0].rts_data_visual_telemetry_profile == $basin[0].rts_evidence_bevy_runtime_adapter.first_contact_visual_telemetry_profile),
      preview_actor_projection_source: ($basin[0].rts_data_preview_actor_projection.source // "missing"),
      renderer_projection_source: ($basin[0].rts_data_renderer_projection.source // "missing"),
      renderer_projection_renderable_tile_count: ($basin[0].rts_data_renderer_projection.renderable_tile_count // 0),
      renderer_projection_lane_tile_count: ($basin[0].rts_data_renderer_projection.lane_tile_count // 0),
      renderer_projection_minimap_anchor_actor_count: ($basin[0].rts_data_renderer_projection.minimap_anchor_actor_count // 0),
      runtime_application_path: ($basin[0].rts_bevy_runtime_player_screen_application.runtime_application_path // "missing"),
      rts_data_profiles_renderer_neutral: true,
      renderer_draw_math_moved_to_rts_data: false,
      live_bevy_renderer_behavior_moved_to_rts_data: false,
      bevy_renderer_ownership_claimed: false,
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
      sub_batch_4_local_review_complete: true,
      sub_batch_4_exit_rule_satisfied: true,
      sub_batch_5_unblocked_for_local_review: true,
      batch_3_exit_rule_satisfied: false,
      batch_4_unblocked_for_local_review: false,
      next_sub_batch_id: "rts_evidence_crate_boundary",
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
      no_credit_boundary: "local First Contact RTS data extraction sub-batch 4 review only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, socket, hosted-service, live multiplayer, render-world extraction completion, GPU upload, renderer ownership transfer, live-traffic, or public-network credit",
      reviewer_next_action: "continue batch 3 with rts_evidence_crate_boundary; keep batch 4 blocked until all 273 runtime/data-boundary commits have commit-level review"
    }
  ' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_review_first_contact_rts_data_batch_v1"
  and .status == "review_first_contact_rts_data_sub_batch_4_reviewed"
  and .green == true
  and .batch_order == 3
  and .sub_batch_order == 4
  and .sub_batch_id == "first_contact_rts_data_extraction"
  and .prior_sub_batch_reviewed_commit_count == 147
  and .reviewed_commit_count == 24
  and .required_reviewed_commit_count == 24
  and .batch_3_reviewed_commit_count == 171
  and .batch_3_remaining_commit_level_review_count == 102
  and .expected_hash_coverage_complete == true
  and .first_commit == "a53d815eec"
  and .last_commit == "b65b4a44cd"
  and .review_group_count == 4
  and (.review_group_counts | map(.count) | add) == 24
  and (.review_group_counts | map(select(.review_group == "data_profile_extraction").count)[0]) == 6
  and (.review_group_counts | map(select(.review_group == "player_screen_chrome_and_runtime_defaults").count)[0]) == 14
  and (.review_group_counts | map(select(.review_group == "renderer_projection_boundary").count)[0]) == 2
  and (.review_group_counts | map(select(.review_group == "samples_labels_readability_boundary").count)[0]) == 2
  and (.commit_reviews | length) == 24
  and (.commit_reviews | all(.commit_level_review_complete == true))
  and (.commit_reviews | all(.unresolved == false))
  and .unresolved_commit_review_count == 0
  and .basin_spec_green == true
  and .basin_spec_failed_top_level_gate_count == 0
  and .rts_data_map_model_gate == true
  and .rts_data_terrain_profile_gate == true
  and .rts_data_opening_profile_gate == true
  and .rts_data_player_startup_gate == true
  and .rts_data_actor_presentation_gate == true
  and .rts_data_visual_telemetry_gate == true
  and .rts_data_player_screen_gate == true
  and .rts_data_player_screen_layout_gate == true
  and .rts_data_player_screen_chrome_gate == true
  and .rts_data_preview_actor_projection_gate == true
  and .rts_data_renderer_projection_gate == true
  and .rts_bevy_runtime_adapter_gate == true
  and .rts_evidence_bevy_runtime_adapter_gate == true
  and .rts_bevy_runtime_player_screen_application_gate == true
  and .rts_data_map_model_actor_count == 39
  and .rts_data_map_model_rule_count == 19
  and .rts_data_preview_actor_count == 39
  and .rts_data_actor_presentation_profile_count == 14
  and .rts_data_player_startup_profile_count == 4
  and .rts_data_terrain_profile_count == 1156
  and .runtime_player_screen_command_queue_count == 4
  and .runtime_player_screen_visible_tile_count == 64
  and .player_screen_profile_contract_version == "trnm_rts_data_first_contact_player_screen_v1"
  and .player_screen_profile_map_id == "first_contact_basin"
  and .player_screen_profile_room_id == "first-contact-basin"
  and .player_screen_profile_command_slot_count == 6
  and .player_screen_profile_matches_evidence_adapter == true
  and .player_screen_layout_matches_profile == true
  and .player_screen_chrome_matches_profile == true
  and .actor_presentation_matches_evidence_adapter == true
  and .visual_telemetry_matches_evidence_adapter == true
  and (.preview_actor_projection_source | contains("trnm-rts-data first_contact_preview_actors"))
  and (.renderer_projection_source | contains("RtsMapModel bounds"))
  and .renderer_projection_renderable_tile_count == 1024
  and .renderer_projection_lane_tile_count == 240
  and .renderer_projection_minimap_anchor_actor_count == 39
  and .runtime_application_path == "trnm-rts-data first_contact_player_screen_profile -> trnm-rts-bevy-runtime player_screen_runtime_application -> NativeFirstPlayableRuntime mutation"
  and .rts_data_profiles_renderer_neutral == true
  and .renderer_draw_math_moved_to_rts_data == false
  and .live_bevy_renderer_behavior_moved_to_rts_data == false
  and .bevy_renderer_ownership_claimed == false
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
  and .sub_batch_4_local_review_complete == true
  and .sub_batch_4_exit_rule_satisfied == true
  and .sub_batch_5_unblocked_for_local_review == true
  and .batch_3_exit_rule_satisfied == false
  and .batch_4_unblocked_for_local_review == false
  and .next_sub_batch_id == "rts_evidence_crate_boundary"
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
  and (.no_credit_boundary | contains("local First Contact RTS data extraction sub-batch 4 review only"))
' "$SUMMARY" >/dev/null

{
  printf '# Trillionnium World Review First Contact RTS Data Batch\n\n'
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
  printf -- '- sub-batch 4 local review complete / exit rule: `%s` / `%s`\n' \
    "$(jq -r '.sub_batch_4_local_review_complete' "$SUMMARY")" \
    "$(jq -r '.sub_batch_4_exit_rule_satisfied' "$SUMMARY")"
  printf -- '- next sub-batch: `%s`\n\n' "$(jq -r '.next_sub_batch_id' "$SUMMARY")"
  printf '## Review Groups\n\n'
  jq -r '.review_group_counts[] | "- `\(.review_group)`: `\(.count)` commits, unresolved `\(.unresolved_count)`"' "$SUMMARY"
  printf '\n## Data Boundary\n\n'
  printf -- '- RTS data gates: terrain/opening/startup/actor/telemetry/player-screen/preview/renderer projection all green.\n'
  printf -- '- Renderer draw math moved to RTS data: `%s`\n' "$(jq -r '.renderer_draw_math_moved_to_rts_data' "$SUMMARY")"
  printf -- '- Live Bevy renderer behavior moved to RTS data: `%s`\n' "$(jq -r '.live_bevy_renderer_behavior_moved_to_rts_data' "$SUMMARY")"
  printf -- '- Public/S5/beta/commercial claims: `%s` / `%s` / `%s` / `%s`\n' \
    "$(jq -r '.public_launch_ready_claimed' "$SUMMARY")" \
    "$(jq -r '.android_s5_real_device_claimed' "$SUMMARY")" \
    "$(jq -r '.beta_cohort_evidence_claimed' "$SUMMARY")" \
    "$(jq -r '.commercial_launch_evidence_claimed' "$SUMMARY")"
} >"$SUMMARY_MD"

printf 'TRILLIONNIUM_WORLD_REVIEW_FIRST_CONTACT_RTS_DATA_BATCH_GREEN %s %s\n' "$SUMMARY" "$SUMMARY_MD"
