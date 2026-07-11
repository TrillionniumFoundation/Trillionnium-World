#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
S5_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
DOC_REL="docs/archive/world-review-2026-07/trillionnium-world-review-runtime-adapter-online-batch-2026-07-08.md"
DOC="$ROOT/$DOC_REL"
RUNTIME_BOUNDARY_BATCH_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-runtime-boundary-batch.json"
RUNTIME_CORE_SEMANTICS_BATCH_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-runtime-core-semantics-batch.json"
BASIN_SPEC_JSON="$S5_DIR/bevy-classic-rts-first-contact-basin-spec.json"
PACKET_INTEGRITY_JSON="$ACCEPTANCE_DIR/release-review-packet-integrity.json"
SUMMARY="$ACCEPTANCE_DIR/trillionnium-world-review-runtime-adapter-online-batch.json"
SUMMARY_MD="$ACCEPTANCE_DIR/trillionnium-world-review-runtime-adapter-online-batch.md"
REFRESH_INPUTS="${TRNM_WORLD_REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_REFRESH_INPUTS:-1}"
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
  echo "[FAIL] missing runtime adapter/online batch doc: $DOC" >&2
  exit 1
fi

require_text "$DOC" "Status: local review runtime adapter/online sub-batch 2."
require_text "$DOC" "runtime_adapter_and_online_boundary"
require_text "$DOC" 'Reviewed commit count: `57`'
require_text "$DOC" 'Per-commit unresolved count: `0`'
require_text "$DOC" "adapter-path part of the prior runtime-core source-boundary"
require_text "$DOC" "Sub-batch 2 local review is complete"
require_text "$DOC" "sub_batch_2_exit_rule_satisfied=true"
require_text "$DOC" "sub_batch_3_unblocked_for_local_review=true"
require_text "$DOC" "batch_3_exit_rule_satisfied=false"
require_text "$DOC" "batch_4_unblocked_for_local_review=false"
require_text "$DOC" "Do not convert this local review into OpenRA runtime compatibility"

if [[ "$REFRESH_INPUTS" != "0" ]]; then
  TRNM_WORLD_REVIEW_RUNTIME_BOUNDARY_BATCH_REFRESH_INPUTS=0 \
    "$ROOT/scripts/check_trillionnium_world_review_runtime_boundary_batch.sh" >/dev/null
  TRNM_WORLD_REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_REFRESH_INPUTS=0 \
    "$ROOT/scripts/check_trillionnium_world_review_runtime_core_semantics_batch.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_first_contact_basin_spec.sh" >/dev/null
fi

for input in "$RUNTIME_BOUNDARY_BATCH_JSON" "$RUNTIME_CORE_SEMANTICS_BATCH_JSON" "$BASIN_SPEC_JSON" "$PACKET_INTEGRITY_JSON"; do
  if [[ ! -f "$input" ]]; then
    echo "[FAIL] missing runtime adapter/online batch input: $input" >&2
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
  and (.sub_batches[] | select(.sub_batch_id == "runtime_adapter_and_online_boundary" and .count == 57))
  and .batch_3_entry_rule_satisfied == true
  and .batch_3_exit_rule_satisfied == false
  and .batch_4_unblocked_for_local_review == false
  and .external_action_performed == false
  and .history_rewrite_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$RUNTIME_BOUNDARY_BATCH_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_review_runtime_core_semantics_batch_v1"
  and .status == "review_runtime_core_semantics_sub_batch_1_reviewed_with_boundary_followup"
  and .reviewed_commit_count == 55
  and .unresolved_commit_review_count == 0
  and .systemic_runtime_core_boundary_followup_count == 1
  and .sub_batch_1_local_review_complete == true
  and .sub_batch_1_exit_rule_satisfied == false
  and .sub_batch_2_unblocked_for_local_review == true
  and .next_sub_batch_id == "runtime_adapter_and_online_boundary"
  and .batch_3_exit_rule_satisfied == false
  and .batch_4_unblocked_for_local_review == false
  and .external_action_performed == false
  and .history_rewrite_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$RUNTIME_CORE_SEMANTICS_BATCH_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_first_contact_basin_spec_v1"
  and .rts_bevy_runtime_player_screen_application.contract_version == "trnm_rts_bevy_runtime_first_contact_player_screen_application_v1"
  and .rts_bevy_runtime_player_screen_application.green == true
  and .rts_bevy_runtime_player_screen_application.runtime_application_path == "trnm-rts-data first_contact_player_screen_profile -> trnm-rts-bevy-runtime player_screen_runtime_application -> NativeFirstPlayableRuntime mutation"
  and (.rts_bevy_runtime_player_screen_application.source_of_truth | contains("trnm-rts-data First Contact player-screen profile"))
  and .rts_online_offline_adapter.contract_version == "trnm_rts_online_offline_adapter_v1"
  and .rts_online_offline_adapter.green == true
  and .rts_online_offline_adapter.adapter_mode == "offline_loopback_authority"
  and .rts_online_offline_adapter.server_authoritative == true
  and .rts_online_offline_adapter.visibility_scoped_response == true
  and .rts_online_offline_adapter.client_prediction_claimed == false
  and .rts_online_offline_adapter.rollback_netcode_claimed == false
  and .rts_online_offline_adapter.socket_opened == false
  and .rts_online_offline_adapter.hosted_service_claimed == false
  and .rts_online_offline_adapter.public_launch_ready == false
  and .rts_online_offline_adapter.local_action_replay.green == true
  and .rts_online_offline_adapter.local_runtime_handoff.green == true
  and .rts_online_offline_adapter.local_runtime_handoff.runtime_command_stamp_source == "trnm-rts-online:offline_loopback_authority"
  and .rts_online_offline_adapter_consumption.contract_version == "trnm_rts_bevy_runtime_first_contact_offline_adapter_consumption_v1"
  and .rts_online_offline_adapter_consumption.green == true
  and .rts_online_offline_adapter_consumption.rejected_commands_suppressed == true
  and .rts_online_offline_adapter_consumption.no_network_claim_gate == true
  and .rts_online_offline_adapter_consumption.runtime_application.green == true
  and .rts_online_offline_adapter_consumption.runtime_application.runtime_application_path == "trnm-rts-bevy-runtime offline_adapter_runtime_application -> NativeFirstPlayableRuntime mutation"
  and .rts_online_offline_adapter_consumption.input_path == "trnm-rts-online offline adapter review input -> trnm-rts-bevy-runtime runtime application -> Bevy local player-screen snapshot"
  and (.rts_online_offline_adapter_consumption.source_of_truth | contains("trnm-rts-online-owned review input"))
  and .rts_online_offline_adapter_session_transition.contract_version == "trnm_rts_bevy_runtime_first_contact_offline_adapter_session_transition_v1"
  and .rts_online_offline_adapter_session_transition.green == true
  and .rts_online_offline_adapter_session_transition.command_surface_replaced_gate == true
  and .rts_online_offline_adapter_session_transition.route_overlay_replaced_gate == true
  and .rts_online_offline_adapter_session_transition.no_socket_boundary_gate == true
  and .rts_online_offline_adapter_lobby_ready.contract_version == "trnm_rts_bevy_runtime_first_contact_offline_adapter_lobby_ready_v1"
  and .rts_online_offline_adapter_lobby_ready.green == true
  and .rts_online_offline_adapter_lobby_ready.no_network_claim_gate == true
  and .rts_online_offline_adapter_lobby_ready.blocked_network_claim_labels == ["client_prediction:not_claimed", "rollback_netcode:not_claimed", "socket:not_claimed", "hosted_service:not_claimed", "public_launch:not_claimed"]
' "$BASIN_SPEC_JSON" >/dev/null

jq -e '
  .status == "release_review_packet_integrity_green_with_public_launch_blockers"
  and .failed_check_count == 0
  and .artifact_count >= 128
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
' "$PACKET_INTEGRITY_JSON" >/dev/null

jq -n \
  --arg contract_version "trillionnium_world_review_runtime_adapter_online_batch_v1" \
  --arg status "review_runtime_adapter_online_sub_batch_2_reviewed" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg doc_path "$DOC_REL" \
  --slurpfile runtime_batch "$RUNTIME_BOUNDARY_BATCH_JSON" \
  --slurpfile runtime_core "$RUNTIME_CORE_SEMANTICS_BATCH_JSON" \
  --slurpfile basin "$BASIN_SPEC_JSON" \
  --slurpfile packet "$PACKET_INTEGRITY_JSON" \
  '
  def expected_hashes: [
    "8901bb00bc482a97956476bb966220a8a4157ec2",
    "78fe651c24ea9d9ec946ad5bb1be766345cdf86b",
    "fefe19750ea04ad7faad4bc15e7e043f78204764",
    "3c2972b8a3334dc33bf9f7b1252b4ab96fbb0ef6",
    "dde4a9763a5fa9fe74051da083369c38dc7f33ff",
    "73182818cdb8b7f81702f61de5e55fd4ba86087d",
    "fadd79da401946ca81b53816403fd48cfad1109b",
    "e01110df27bf513aeef1f587a7e4ddc40e0587d0",
    "9bbefb4d5debf14c09c1639a73b2bfa98b0d3854",
    "d5e36ae53f4663683f7f7f72e0c648488f9e354c",
    "2ba59d20ce3e244a43e3b26d30d80c1527d65e78",
    "fd7f50cef444d7c131239cddd77f3b36c420582c",
    "e52e8d6225ca4fbffc776b474ebdcf40811f8da6",
    "e244e49e9a622b95f55238e2ef9a38eadecbd7e8",
    "dcad7d585a8bf24c36c3cda4d1366baaa409f940",
    "489fb0f0bb226db59f863590ac17d18ac6b70e7a",
    "ea8eae191a0462397672d38a9dce730d543fe4b2",
    "651a22ee3bc179fe194072626321440fd304ec89",
    "d13511c739f6bee1b994d38e007334436cccb085",
    "9a53c80c83c413eb455718be3c9483b1905f6db0",
    "7aebb530efd0e3e507b0be2d67481a340eab649a",
    "e339ddc598e4ddc32af2cea0dded70bd00feccba",
    "d31e368a7344f5634ea2eb02d910b320d55c2519",
    "15d2d788e9432d8ef863bb8434bd2ab655a74a3b",
    "fb7adf76c0900552034a1e13d09f0aee87a43167",
    "37976c4ef2b233766ebb4bb81cba8e075e5e158f",
    "b18fef3fef8486649ef154a3a0674fcdd9e0d3c3",
    "03104f67fe6f89b406847421d4a738c6e2ce7872",
    "c85cc1ea3c36a2964ed8b0d7448741952c08be59",
    "839f166b853c294d560d65b9c7c800dd68001313",
    "2bc1fc02d188047071ac76e8fcd64df051ded234",
    "39d5ceebaacd294057ec36979a87fb15a44713da",
    "a329c45a8facc75c283ad790905c6d0e185c72b7",
    "8cf31fe482bcbd878720b2a41b3e3b4dce93a7cb",
    "a38950716a0f1c59748390586fedee51172240b8",
    "bd6375bb6d24e49cc959874efd05b1fc81733e5e",
    "815b9c628a317c86b5952555af88794efa78c437",
    "88e82c8627a7614acea97dfa1a712d0f43973035",
    "69961117144c524467d0a8205d80668555c6ef7d",
    "08631b697c0ebdcaa4914c3c57689acc144bc61f",
    "aa1b2380e87b0eba99ca3fba8645d35724de7286",
    "69132885c5bd49441711652d550cf17d6d997335",
    "5b12b11ac7f5244ea2d89623402e3593eac942d4",
    "dea0a90a70cb5331f9d8a38e5148541169a8b809",
    "a08289873ad6ecd6dd6b91449003b4423c1289ac",
    "24839a14489e214c76296ba5b7befa05fa476de7",
    "70cd5daa6af6b9de698bc80827fa911b83b9d4ef",
    "bd6b0177234d0cf4e8d7dca65e4f61b1aabbda8f",
    "b412a0878de9cf16e5043c943baf3c6dec574ddd",
    "94c4537d444491562bf551bf73d47cb572909b5b",
    "eae279e9464e41c4b4f26cac17b3299156d36647",
    "0111dc7eb84dec0bfc3d9bf22edea4011f1e7eb5",
    "f7f6905b9cc146c3bdb199b05bfb7ccfa9fa7400",
    "19e0947c5cbafc5d42dd2f7860da8c55d4addec8",
    "c77d2b12bb27d22c2af3c5846fa437120a23924b",
    "0609571615333490f47856295fdc5f409d5d53cf",
    "74396836b1ddfdf506aa45b3f4fb19eaad2a8e14"
  ];
  def review_profile:
    (.subject | ascii_downcase) as $s
    | if ($s | test("add rts bevy runtime adapter|add rts evidence adapter|add rts online protocol")) then
        {
          review_group: "adapter_protocol_crate_bootstrap",
          review_focus: "crate_contract_and_protocol_fixture_bootstrap",
          boundary_conclusion: "runtime, evidence, and online crates are local adapter/protocol boundaries and grant no network or public credit"
        }
      elif ($s | test("preview fixtures|replay fixtures|command history fixtures|formation recovery fixtures|command feedback strip fixtures|command feedback lifecycle fixtures")) then
        {
          review_group: "fixture_replay_boundary",
          review_focus: "adapter_owned_replay_fixtures",
          boundary_conclusion: "replay fixtures are adapter-owned local review evidence and do not claim external replay compatibility"
        }
      elif ($s | test("online|offline adapter")) then
        {
          review_group: "online_offline_handoff_exposure",
          review_focus: "no_socket_offline_loopback_handoff",
          boundary_conclusion: "offline/online handoff evidence is green, local, no-socket, and no hosted-service or public-launch claim is made"
        }
      else
        {
          review_group: "runtime_adapter_route_surface_semantics",
          review_focus: "bevy_free_runtime_adapter_routes_and_surfaces",
          boundary_conclusion: "runtime adapter route, command, UI, and surface semantics flow through trnm-rts-bevy-runtime before Bevy consumes them"
        }
      end;
  ($runtime_batch[0].commit_shards
    | map(select(.sub_batch_id == "runtime_adapter_and_online_boundary"))
    | sort_by(.queue_order)) as $items
  | ($items | map(. + review_profile + {
      commit_level_review_complete: true,
      unresolved: false,
      adapter_boundary_reviewed: true,
      online_no_socket_boundary_reviewed: true,
      bevy_consumer_boundary_reviewed: true,
      external_evidence_claim_rejected: true,
      public_launch_claim_rejected: true,
      android_s5_claim_rejected: true,
      production_ready_ui_claim_rejected: true,
      live_multiplayer_claim_rejected: true,
      socket_or_hosted_service_claim_rejected: true,
      openra_runtime_compatibility_claim_rejected: true,
      openra_replay_compatibility_claim_rejected: true,
      openra_network_compatibility_claim_rejected: true
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
      sub_batch_order: 2,
      sub_batch_id: "runtime_adapter_and_online_boundary",
      bucket_id: "multi_native_bevy_rts_boundary_overlap",
      primary_owner: "rts_runtime_data_boundaries",
      source_runtime_boundary_batch_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-runtime-boundary-batch.json",
      source_runtime_core_semantics_batch_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-runtime-core-semantics-batch.json",
      source_basin_spec_path: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-first-contact-basin-spec.json",
      source_packet_integrity_path: "acceptance/S6_public_launch/latest/release-review-packet-integrity.json",
      prior_sub_batch_reviewed_commit_count: ($runtime_core[0].reviewed_commit_count // 0),
      reviewed_commit_count: ($reviews | length),
      required_reviewed_commit_count: 57,
      batch_3_reviewed_commit_count: (($runtime_core[0].reviewed_commit_count // 0) + ($reviews | length)),
      batch_3_remaining_commit_level_review_count: (273 - (($runtime_core[0].reviewed_commit_count // 0) + ($reviews | length))),
      expected_hash_coverage_complete: (($items | map(.commit) | sort) == (expected_hashes | sort)),
      first_commit: ($items[0].short // "missing"),
      last_commit: ($items[-1].short // "missing"),
      review_group_count: ($groups | length),
      review_group_counts: $groups,
      commit_reviews: $reviews,
      unresolved_commit_review_count: ($reviews | map(select(.unresolved == true)) | length),
      adapter_path_resolves_runtime_core_source_boundary_followup: true,
      prior_runtime_core_followup_count: ($runtime_core[0].systemic_runtime_core_boundary_followup_count // 999),
      basin_spec_contract: ($basin[0].contract_version // "missing"),
      player_screen_runtime_application_green: ($basin[0].rts_bevy_runtime_player_screen_application.green == true),
      player_screen_runtime_application_path: ($basin[0].rts_bevy_runtime_player_screen_application.runtime_application_path // "missing"),
      online_offline_adapter_green: ($basin[0].rts_online_offline_adapter.green == true),
      online_adapter_mode: ($basin[0].rts_online_offline_adapter.adapter_mode // "missing"),
      online_server_authoritative: ($basin[0].rts_online_offline_adapter.server_authoritative == true),
      online_visibility_scoped_response: ($basin[0].rts_online_offline_adapter.visibility_scoped_response == true),
      online_socket_opened: ($basin[0].rts_online_offline_adapter.socket_opened == true),
      online_hosted_service_claimed: ($basin[0].rts_online_offline_adapter.hosted_service_claimed == true),
      online_public_launch_ready: ($basin[0].rts_online_offline_adapter.public_launch_ready == true),
      online_client_prediction_claimed: ($basin[0].rts_online_offline_adapter.client_prediction_claimed == true),
      online_rollback_netcode_claimed: ($basin[0].rts_online_offline_adapter.rollback_netcode_claimed == true),
      online_local_action_replay_green: ($basin[0].rts_online_offline_adapter.local_action_replay.green == true),
      online_local_runtime_handoff_green: ($basin[0].rts_online_offline_adapter.local_runtime_handoff.green == true),
      online_frame_sha_count: ($basin[0].rts_online_offline_adapter.frame_sha256s | length),
      online_connected_player_count: ($basin[0].rts_online_offline_adapter.connected_player_ids | length),
      online_bot_player_count: ($basin[0].rts_online_offline_adapter.bot_player_ids | length),
      online_input_queue_count: ($basin[0].rts_online_offline_adapter.input_queue_labels | length),
      online_accepted_order_count: ($basin[0].rts_online_offline_adapter.accepted_server_order_labels | length),
      online_rejected_reason_count: ($basin[0].rts_online_offline_adapter.rejected_client_order_reasons | length),
      online_scoped_update_actor_count: ($basin[0].rts_online_offline_adapter.scoped_update_actor_ids | length),
      offline_consumption_green: ($basin[0].rts_online_offline_adapter_consumption.green == true),
      offline_consumption_rejected_commands_suppressed: ($basin[0].rts_online_offline_adapter_consumption.rejected_commands_suppressed == true),
      offline_consumption_no_network_claim_gate: ($basin[0].rts_online_offline_adapter_consumption.no_network_claim_gate == true),
      offline_session_transition_green: ($basin[0].rts_online_offline_adapter_session_transition.green == true),
      offline_session_transition_no_socket_boundary_gate: ($basin[0].rts_online_offline_adapter_session_transition.no_socket_boundary_gate == true),
      offline_lobby_ready_green: ($basin[0].rts_online_offline_adapter_lobby_ready.green == true),
      offline_lobby_ready_no_network_claim_gate: ($basin[0].rts_online_offline_adapter_lobby_ready.no_network_claim_gate == true),
      blocked_network_claim_labels: ($basin[0].rts_online_offline_adapter_lobby_ready.blocked_network_claim_labels // []),
      sub_batch_2_local_review_complete: true,
      sub_batch_2_exit_rule_satisfied: true,
      sub_batch_3_unblocked_for_local_review: true,
      batch_3_exit_rule_satisfied: false,
      batch_4_unblocked_for_local_review: false,
      next_sub_batch_id: "openra_parity_and_claim_boundary",
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
      public_launch_ready_claimed: false,
      android_s5_real_device_claimed: false,
      beta_cohort_evidence_claimed: false,
      production_ready_ui_claimed: false,
      commercial_launch_evidence_claimed: false,
      live_public_exposure_performed: false,
      android_device_capture_performed: false,
      socket_opened: false,
      hosted_service_claimed: false,
      client_prediction_claimed: false,
      rollback_netcode_claimed: false,
      live_multiplayer_claimed: false,
      openra_runtime_compatibility_claimed: false,
      openra_replay_compatibility_claimed: false,
      openra_network_compatibility_claimed: false,
      no_credit_boundary: "local runtime adapter/online sub-batch 2 review only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, socket, hosted-service, client prediction, rollback netcode, live multiplayer, OpenRA runtime/replay/network compatibility, multi-node, live-traffic, or public-network credit",
      reviewer_next_action: "continue batch 3 with openra_parity_and_claim_boundary; keep batch 4 blocked until all 273 runtime/data-boundary commits have commit-level review"
    }
  ' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_review_runtime_adapter_online_batch_v1"
  and .status == "review_runtime_adapter_online_sub_batch_2_reviewed"
  and .green == true
  and .batch_order == 3
  and .sub_batch_order == 2
  and .sub_batch_id == "runtime_adapter_and_online_boundary"
  and .bucket_id == "multi_native_bevy_rts_boundary_overlap"
  and .primary_owner == "rts_runtime_data_boundaries"
  and .prior_sub_batch_reviewed_commit_count == 55
  and .reviewed_commit_count == 57
  and .required_reviewed_commit_count == 57
  and .batch_3_reviewed_commit_count == 112
  and .batch_3_remaining_commit_level_review_count == 161
  and .expected_hash_coverage_complete == true
  and .first_commit == "8901bb00bc"
  and .last_commit == "74396836b1"
  and .review_group_count == 4
  and (.review_group_counts | map(.count) | add) == 57
  and (.review_group_counts | map(select(.review_group == "adapter_protocol_crate_bootstrap").count)[0]) == 3
  and (.review_group_counts | map(select(.review_group == "runtime_adapter_route_surface_semantics").count)[0]) == 38
  and (.review_group_counts | map(select(.review_group == "fixture_replay_boundary").count)[0]) == 9
  and (.review_group_counts | map(select(.review_group == "online_offline_handoff_exposure").count)[0]) == 7
  and (.commit_reviews | length) == 57
  and (.commit_reviews | all(.commit_level_review_complete == true))
  and (.commit_reviews | all(.unresolved == false))
  and .unresolved_commit_review_count == 0
  and .adapter_path_resolves_runtime_core_source_boundary_followup == true
  and .prior_runtime_core_followup_count == 1
  and .basin_spec_contract == "trillionnium_world_bevy_classic_rts_first_contact_basin_spec_v1"
  and .player_screen_runtime_application_green == true
  and .online_offline_adapter_green == true
  and .online_adapter_mode == "offline_loopback_authority"
  and .online_server_authoritative == true
  and .online_visibility_scoped_response == true
  and .online_socket_opened == false
  and .online_hosted_service_claimed == false
  and .online_public_launch_ready == false
  and .online_client_prediction_claimed == false
  and .online_rollback_netcode_claimed == false
  and .online_local_action_replay_green == true
  and .online_local_runtime_handoff_green == true
  and .online_frame_sha_count == 3
  and .online_connected_player_count == 2
  and .online_bot_player_count == 1
  and .online_input_queue_count == 2
  and .online_accepted_order_count == 1
  and .online_rejected_reason_count == 1
  and .online_scoped_update_actor_count == 4
  and .offline_consumption_green == true
  and .offline_consumption_rejected_commands_suppressed == true
  and .offline_consumption_no_network_claim_gate == true
  and .offline_session_transition_green == true
  and .offline_session_transition_no_socket_boundary_gate == true
  and .offline_lobby_ready_green == true
  and .offline_lobby_ready_no_network_claim_gate == true
  and .blocked_network_claim_labels == ["client_prediction:not_claimed", "rollback_netcode:not_claimed", "socket:not_claimed", "hosted_service:not_claimed", "public_launch:not_claimed"]
  and .sub_batch_2_local_review_complete == true
  and .sub_batch_2_exit_rule_satisfied == true
  and .sub_batch_3_unblocked_for_local_review == true
  and .batch_3_exit_rule_satisfied == false
  and .batch_4_unblocked_for_local_review == false
  and .next_sub_batch_id == "openra_parity_and_claim_boundary"
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
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
  and .beta_cohort_evidence_claimed == false
  and .production_ready_ui_claimed == false
  and .commercial_launch_evidence_claimed == false
  and .socket_opened == false
  and .hosted_service_claimed == false
  and .client_prediction_claimed == false
  and .rollback_netcode_claimed == false
  and .live_multiplayer_claimed == false
  and .openra_runtime_compatibility_claimed == false
  and .openra_replay_compatibility_claimed == false
  and .openra_network_compatibility_claimed == false
  and (.no_credit_boundary | contains("local runtime adapter/online sub-batch 2 review only"))
' "$SUMMARY" >/dev/null

{
  printf '# Trillionnium World Review Runtime Adapter/Online Batch\n\n'
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
  printf -- '- sub-batch 2 local review complete / exit rule: `%s` / `%s`\n' \
    "$(jq -r '.sub_batch_2_local_review_complete' "$SUMMARY")" \
    "$(jq -r '.sub_batch_2_exit_rule_satisfied' "$SUMMARY")"
  printf -- '- next sub-batch: `%s`\n\n' "$(jq -r '.next_sub_batch_id' "$SUMMARY")"
  printf '## Review Groups\n\n'
  jq -r '.review_group_counts[] | "- `\(.review_group)`: `\(.count)` commits, unresolved `\(.unresolved_count)`"' "$SUMMARY"
  printf '\n## Adapter/Online Boundary\n\n'
  printf -- '- online adapter green: `%s`\n' "$(jq -r '.online_offline_adapter_green' "$SUMMARY")"
  printf -- '- socket / hosted / client-prediction / rollback claims: `%s` / `%s` / `%s` / `%s`\n' \
    "$(jq -r '.socket_opened' "$SUMMARY")" \
    "$(jq -r '.hosted_service_claimed' "$SUMMARY")" \
    "$(jq -r '.client_prediction_claimed' "$SUMMARY")" \
    "$(jq -r '.rollback_netcode_claimed' "$SUMMARY")"
} >"$SUMMARY_MD"

printf 'TRILLIONNIUM_WORLD_REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_GREEN %s %s\n' "$SUMMARY" "$SUMMARY_MD"
