#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
S5_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
DOC_REL="docs/archive/world-review-2026-07/trillionnium-world-review-openra-parity-claim-batch-2026-07-08.md"
DOC="$ROOT/$DOC_REL"
RUNTIME_BOUNDARY_BATCH_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-runtime-boundary-batch.json"
RUNTIME_ADAPTER_ONLINE_BATCH_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-runtime-adapter-online-batch.json"
PARITY_BRIDGE_JSON="$S5_DIR/bevy-classic-rts-openra-parity-bridge.json"
PARITY_LANE_JSON="$S5_DIR/bevy-classic-rts-openra-parity-lane.json"
REPLAY_COMPAT_ADAPTER_JSON="$S5_DIR/bevy-classic-rts-openra-replay-compat-adapter.json"
COMMAND_VOCAB_ADAPTER_JSON="$S5_DIR/bevy-classic-rts-openra-command-vocab-adapter.json"
ORDER_SERIALIZER_JSON="$S5_DIR/bevy-classic-rts-openra-order-serializer-fixture.json"
ORDER_REPLAY_REDUCER_JSON="$S5_DIR/bevy-classic-rts-openra-order-replay-reducer.json"
HEADLESS_COMPARISON_JSON="$S5_DIR/bevy-classic-rts-openra-headless-comparison-harness.json"
REPLAY_IMPORTER_JSON="$S5_DIR/bevy-classic-rts-openra-replay-importer.json"
ORDER_PAYLOAD_DECODER_JSON="$S5_DIR/bevy-classic-rts-openra-order-payload-decoder.json"
IMPORTED_HEADLESS_COMPARISON_JSON="$S5_DIR/bevy-classic-rts-openra-imported-headless-comparison-harness.json"
IMPORTED_REPLAY_AUDIT_JSON="$S5_DIR/bevy-classic-rts-openra-imported-replay-audit-ledger.json"
IMPORTED_REPLAY_REPRO_JSON="$S5_DIR/bevy-classic-rts-openra-imported-replay-repro-manifest.json"
IMPORTED_REPLAY_BUNDLE_JSON="$S5_DIR/bevy-classic-rts-openra-imported-replay-artifact-bundle.json"
IMPORTED_REPLAY_CAPSULE_JSON="$S5_DIR/bevy-classic-rts-openra-imported-replay-review-capsule.json"
IMPORTED_REPLAY_RECEIPT_JSON="$S5_DIR/bevy-classic-rts-openra-imported-replay-review-receipt.json"
IMPORTED_REPLAY_DIGEST_JSON="$S5_DIR/bevy-classic-rts-openra-imported-replay-review-digest.json"
SCREEN_UI_JSON="$S5_DIR/bevy-classic-rts-openra-screen-for-screen-ui-replication.json"
ASSET_PARITY_JSON="$S5_DIR/bevy-classic-rts-openra-engine-port-asset-parity.json"
PACKET_INTEGRITY_JSON="$ACCEPTANCE_DIR/release-review-packet-integrity.json"
SUMMARY="$ACCEPTANCE_DIR/trillionnium-world-review-openra-parity-claim-batch.json"
SUMMARY_MD="$ACCEPTANCE_DIR/trillionnium-world-review-openra-parity-claim-batch.md"
REFRESH_INPUTS="${TRNM_WORLD_REVIEW_OPENRA_PARITY_CLAIM_BATCH_REFRESH_INPUTS:-1}"
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
  echo "[FAIL] missing OpenRA parity/claim batch doc: $DOC" >&2
  exit 1
fi

require_text "$DOC" "Status: local review OpenRA parity/claim sub-batch 3."
require_text "$DOC" "openra_parity_and_claim_boundary"
require_text "$DOC" 'Reviewed commit count: `35`'
require_text "$DOC" 'Per-commit unresolved count: `0`'
require_text "$DOC" "Do not convert this local review into OpenRA runtime compatibility"
require_text "$DOC" "Sub-batch 3 local review is complete"
require_text "$DOC" "sub_batch_3_exit_rule_satisfied=true"
require_text "$DOC" "sub_batch_4_unblocked_for_local_review=true"
require_text "$DOC" "batch_3_exit_rule_satisfied=false"
require_text "$DOC" "batch_4_unblocked_for_local_review=false"

if [[ "$REFRESH_INPUTS" != "0" ]]; then
  TRNM_WORLD_REVIEW_RUNTIME_BOUNDARY_BATCH_REFRESH_INPUTS=0 \
    "$ROOT/scripts/check_trillionnium_world_review_runtime_boundary_batch.sh" >/dev/null
  TRNM_WORLD_REVIEW_RUNTIME_ADAPTER_ONLINE_BATCH_REFRESH_INPUTS=0 \
    "$ROOT/scripts/check_trillionnium_world_review_runtime_adapter_online_batch.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_parity_bridge.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_parity_lane.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_replay_compat_adapter.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_command_vocab_adapter.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_order_serializer_fixture.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_order_replay_reducer.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_headless_comparison_harness.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_replay_importer.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_order_payload_decoder.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_imported_headless_comparison_harness.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_imported_replay_audit_ledger.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_imported_replay_repro_manifest.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_imported_replay_artifact_bundle.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_imported_replay_review_capsule.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_imported_replay_review_receipt.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_imported_replay_review_digest.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_screen_for_screen_ui_replication.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_engine_port_asset_parity.sh" >/dev/null
fi

for input in \
  "$RUNTIME_BOUNDARY_BATCH_JSON" \
  "$RUNTIME_ADAPTER_ONLINE_BATCH_JSON" \
  "$PARITY_BRIDGE_JSON" \
  "$PARITY_LANE_JSON" \
  "$REPLAY_COMPAT_ADAPTER_JSON" \
  "$COMMAND_VOCAB_ADAPTER_JSON" \
  "$ORDER_SERIALIZER_JSON" \
  "$ORDER_REPLAY_REDUCER_JSON" \
  "$HEADLESS_COMPARISON_JSON" \
  "$REPLAY_IMPORTER_JSON" \
  "$ORDER_PAYLOAD_DECODER_JSON" \
  "$IMPORTED_HEADLESS_COMPARISON_JSON" \
  "$IMPORTED_REPLAY_AUDIT_JSON" \
  "$IMPORTED_REPLAY_REPRO_JSON" \
  "$IMPORTED_REPLAY_BUNDLE_JSON" \
  "$IMPORTED_REPLAY_CAPSULE_JSON" \
  "$IMPORTED_REPLAY_RECEIPT_JSON" \
  "$IMPORTED_REPLAY_DIGEST_JSON" \
  "$SCREEN_UI_JSON" \
  "$ASSET_PARITY_JSON" \
  "$PACKET_INTEGRITY_JSON"; do
  if [[ ! -f "$input" ]]; then
    echo "[FAIL] missing OpenRA parity/claim batch input: $input" >&2
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
  and (.sub_batches[] | select(.sub_batch_id == "openra_parity_and_claim_boundary" and .count == 35))
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
  and .reviewed_commit_count == 57
  and .unresolved_commit_review_count == 0
  and .batch_3_reviewed_commit_count == 112
  and .batch_3_remaining_commit_level_review_count == 161
  and .sub_batch_2_exit_rule_satisfied == true
  and .sub_batch_3_unblocked_for_local_review == true
  and .next_sub_batch_id == "openra_parity_and_claim_boundary"
  and .batch_3_exit_rule_satisfied == false
  and .batch_4_unblocked_for_local_review == false
  and .external_action_performed == false
  and .history_rewrite_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
  and .openra_runtime_compatibility_claimed == false
  and .openra_replay_compatibility_claimed == false
  and .openra_network_compatibility_claimed == false
' "$RUNTIME_ADAPTER_ONLINE_BATCH_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_parity_bridge_v1"
  and .green == true
  and .no_parity_claim_gate == true
  and .openra_target_commit_gate == true
  and .comparison_axis_count == 4
  and .preview_count == 4
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
' "$PARITY_BRIDGE_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_parity_lane_v1"
  and .green == true
  and .no_openra_parity_claim_gate == true
  and .lane_axis_count == 6
  and .bevy_openra_parity_claimed == false
  and .bevy_openra_runtime_parity_claimed == false
  and .bevy_openra_replay_file_claimed == false
  and .bevy_openra_headless_client_match_claimed == false
  and .bevy_openra_bot_ai_parity_claimed == false
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
' "$PARITY_LANE_JSON" >/dev/null

for replay_input in \
  "$REPLAY_COMPAT_ADAPTER_JSON" \
  "$COMMAND_VOCAB_ADAPTER_JSON" \
  "$ORDER_SERIALIZER_JSON" \
  "$ORDER_REPLAY_REDUCER_JSON" \
  "$HEADLESS_COMPARISON_JSON" \
  "$REPLAY_IMPORTER_JSON" \
  "$ORDER_PAYLOAD_DECODER_JSON" \
  "$IMPORTED_HEADLESS_COMPARISON_JSON" \
  "$IMPORTED_REPLAY_AUDIT_JSON" \
  "$IMPORTED_REPLAY_REPRO_JSON" \
  "$IMPORTED_REPLAY_BUNDLE_JSON" \
  "$IMPORTED_REPLAY_CAPSULE_JSON" \
  "$IMPORTED_REPLAY_RECEIPT_JSON" \
  "$IMPORTED_REPLAY_DIGEST_JSON"; do
  jq -e '
    .green == true
    and .compatibility_boundary_gate == true
    and (.bevy_openra_parity_claimed // false) == false
    and (.bevy_openra_runtime_parity_claimed // false) == false
    and (.bevy_openra_replay_file_claimed // false) == false
    and ((.bevy_openra_binary_replay_compatible // .openra_binary_replay_compatible // false) == false)
    and (.bevy_openra_network_order_stream_claimed // false) == false
    and (.bevy_openra_headless_client_match_claimed // false) == false
    and (.public_launch_ready // false) == false
    and (.android_s5_real_device_claimed // false) == false
  ' "$replay_input" >/dev/null
done

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_imported_replay_review_digest_v1"
  and .negative_case_count == 7
  and .detected_negative_case_count == 7
  and .digest_assertion_gate == true
  and .negative_corpus_gate == true
' "$IMPORTED_REPLAY_DIGEST_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_screen_for_screen_ui_replication_v1"
  and .green == true
  and .failed_gate_count == 0
  and .screen_for_screen_openra_ui_claimed == false
  and .openra_screen_for_screen_ui_replication_claimed == false
  and .openra_pixel_perfect_asset_parity_claimed == false
  and .openra_asset_copied == false
  and .third_party_asset_copied == false
  and .warcraft_iii_asset_copied == false
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
  and .openra_reference_screen_count == 8
  and .replicated_interaction_surface_count == 8
' "$SCREEN_UI_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_engine_port_asset_parity_v1"
  and .green == true
  and .failed_gate_count == 0
  and .openra_engine_port_claimed == false
  and .openra_full_engine_port_claimed == false
  and .openra_engine_port_foundation_claimed == false
  and .trillionnium_owned_asset_pack_pixel_parity_claimed == true
  and .openra_pixel_perfect_asset_parity_claimed == false
  and .openra_westwood_pixel_perfect_asset_parity_claimed == false
  and .openra_asset_copied == false
  and .openra_csharp_engine_code_copied == false
  and .third_party_asset_copied == false
  and .westwood_asset_copied == false
  and .warcraft_iii_asset_copied == false
  and .bevy_openra_binary_replay_compatible == false
  and .bevy_openra_network_order_stream_claimed == false
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
  and .ported_engine_module_count >= 11
  and .openra_widget_root_count == 4
  and .openra_chrome_screen_count == 8
' "$ASSET_PARITY_JSON" >/dev/null

jq -e '
  .status == "release_review_packet_integrity_green_with_public_launch_blockers"
  and .failed_check_count == 0
  and .artifact_count >= 128
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
' "$PACKET_INTEGRITY_JSON" >/dev/null

jq -n \
  --arg contract_version "trillionnium_world_review_openra_parity_claim_batch_v1" \
  --arg status "review_openra_parity_claim_sub_batch_3_reviewed" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg doc_path "$DOC_REL" \
  --slurpfile runtime_batch "$RUNTIME_BOUNDARY_BATCH_JSON" \
  --slurpfile runtime_adapter "$RUNTIME_ADAPTER_ONLINE_BATCH_JSON" \
  --slurpfile parity_bridge "$PARITY_BRIDGE_JSON" \
  --slurpfile parity_lane "$PARITY_LANE_JSON" \
  --slurpfile replay_compat "$REPLAY_COMPAT_ADAPTER_JSON" \
  --slurpfile imported_digest "$IMPORTED_REPLAY_DIGEST_JSON" \
  --slurpfile screen_ui "$SCREEN_UI_JSON" \
  --slurpfile asset_parity "$ASSET_PARITY_JSON" \
  --slurpfile packet "$PACKET_INTEGRITY_JSON" \
  '
  def expected_hashes: [
    "36814fcbbbb8a0ab8fdb17a9d9c86fb89be9b1b4",
    "6d0a24580bbf01073a96f29c49cfe9d67096e2a8",
    "7fec7e3f7bfa5d1d74cca09a1efc7b2c8647cffc",
    "16e5775e543b916b61e97aad061145bf70b11cd1",
    "30873c0b29a8dfb0a4d4f0f98be1536d61833110",
    "12ac68542baee96d31cfee0e115e6750a11399d6",
    "220f9337c4fd1e9fea2ccd6a0df541ff88e55728",
    "98f381aeb8d45bac5ef9ecd205179c4f074cf5bc",
    "d4287654f278338307bc8aa5b810e72c9ad7641e",
    "a649ef6d70ac1acc508e2df9aac420eef99d3c5f",
    "e7caa4c14871b07c25fb924f1af79f3fa00ca4c6",
    "ad2f2cc188fa69a853e1688f3f37ae7839125c87",
    "084654619c4ad99380cd12d5b4166448352d84f9",
    "cdc46e2ec2cc59c7bde2e6bbf9fc5414919c687b",
    "2549462458b2e41799094aef3731121a65572f72",
    "80f00b1c055f06b0d4e5392457e06058f21256a2",
    "100f5e74b83331d4996fe266228994e8ed8c1a7f",
    "e2b04b0d82a60e818469016475b0dfd4550c05fb",
    "6a92e9c43c917136b9669fc84a39d3eab8eb238e",
    "c1df2032bdc9031d1fa0aa56dab8cff311c7692f",
    "43c1c3d04c0d92061c6c842977531cfd93dab926",
    "f713fb13e0c5ec6ba4dcb9d6cb349b6d67239217",
    "09a16fc7a8627fd6bd5b8e80cb92c886a969e445",
    "8ac65744a1e07a72efe89c8f621d3ea060e2ce23",
    "ee5a21942b2cf6941704fb118d26d3b9470190e1",
    "a39e3440fb6336d1e31f9e7013ff2d92974d4c7e",
    "e17a5236bd3a475777f4b7c49674bffb4aa06ee8",
    "df7c4f660a46e5224fb99622a32b2cfd642cc60c",
    "f7f99599968df97de9d59f4e0c5aede7c14d00b2",
    "ad3032f9039abb0a00c16d08b392c0a01b29fde8",
    "7a1ae888247fea29a81ffcc5337b633197c0b6d6",
    "a22a921bcac0eaf8698a871453cd30250a25c0b0",
    "89c16b6ec0c0311719f9c4466ca76aedca4fa0ae",
    "4f052edb83a8af044135b6581d220707f5ad174a",
    "d3fb381a96a80d9943fda3a088e4f14ea90b685f"
  ];
  def review_profile:
    (.subject | ascii_downcase) as $s
    | if ($s | test("screen|ui")) then
        {
          review_group: "screen_ui_claim_boundary",
          review_focus: "openra_style_screen_set_without_openra_ui_claim",
          boundary_conclusion: "screen-set evidence stays local OpenRA-style UI review evidence without screen-for-screen OpenRA UI or asset parity credit"
        }
      elif ($s | test("asset|engine port|hud")) then
        {
          review_group: "engine_asset_claim_boundary",
          review_focus: "project_owned_asset_pack_and_engine_scope",
          boundary_conclusion: "engine and asset evidence stays local/project-owned and grants no OpenRA engine, full-engine, Westwood, or third-party asset-copy credit"
        }
      elif ($s | test("replay|command vocabulary|order serializer|order replay|payload decoder|headless comparison|imported|reuse openra")) then
        {
          review_group: "replay_order_import_boundary",
          review_focus: "local_replay_summary_and_imported_fixture_boundary",
          boundary_conclusion: "replay/order/imported evidence is local summary/fixture evidence and does not claim binary replay, headless-client, protocol, or network compatibility"
        }
      else
        {
          review_group: "semantic_parity_bridge_lane",
          review_focus: "openra_style_semantic_comparison_boundary",
          boundary_conclusion: "parity target, bridge, lane, preview actors, and parity counts remain local semantic comparison evidence"
        }
      end;
  ($runtime_batch[0].commit_shards
    | map(select(.sub_batch_id == "openra_parity_and_claim_boundary"))
    | sort_by(.queue_order)) as $items
  | ($items | map(. + review_profile + {
      commit_level_review_complete: true,
      unresolved: false,
      openra_semantic_boundary_reviewed: true,
      replay_summary_boundary_reviewed: true,
      ui_claim_boundary_reviewed: true,
      asset_claim_boundary_reviewed: true,
      openra_runtime_compatibility_claim_rejected: true,
      openra_binary_replay_compatibility_claim_rejected: true,
      openra_network_compatibility_claim_rejected: true,
      openra_engine_port_claim_rejected: true,
      openra_pixel_perfect_asset_parity_claim_rejected: true,
      third_party_asset_copy_claim_rejected: true,
      public_launch_claim_rejected: true,
      android_s5_claim_rejected: true
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
      sub_batch_order: 3,
      sub_batch_id: "openra_parity_and_claim_boundary",
      bucket_id: "multi_native_bevy_rts_boundary_overlap",
      primary_owner: "rts_runtime_data_boundaries",
      source_runtime_boundary_batch_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-runtime-boundary-batch.json",
      source_runtime_adapter_online_batch_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-runtime-adapter-online-batch.json",
      source_parity_bridge_path: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-parity-bridge.json",
      source_parity_lane_path: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-parity-lane.json",
      source_imported_replay_digest_path: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-imported-replay-review-digest.json",
      source_screen_ui_path: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-screen-for-screen-ui-replication.json",
      source_asset_parity_path: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-engine-port-asset-parity.json",
      source_packet_integrity_path: "acceptance/S6_public_launch/latest/release-review-packet-integrity.json",
      prior_sub_batch_reviewed_commit_count: ($runtime_adapter[0].batch_3_reviewed_commit_count // 0),
      reviewed_commit_count: ($reviews | length),
      required_reviewed_commit_count: 35,
      batch_3_reviewed_commit_count: (($runtime_adapter[0].batch_3_reviewed_commit_count // 0) + ($reviews | length)),
      batch_3_remaining_commit_level_review_count: (273 - (($runtime_adapter[0].batch_3_reviewed_commit_count // 0) + ($reviews | length))),
      expected_hash_coverage_complete: (($items | map(.commit) | sort) == (expected_hashes | sort)),
      first_commit: ($items[0].short // "missing"),
      last_commit: ($items[-1].short // "missing"),
      review_group_count: ($groups | length),
      review_group_counts: $groups,
      commit_reviews: $reviews,
      unresolved_commit_review_count: ($reviews | map(select(.unresolved == true)) | length),
      parity_bridge_green: ($parity_bridge[0].green == true),
      parity_bridge_no_claim_gate: ($parity_bridge[0].no_parity_claim_gate == true),
      parity_bridge_comparison_axis_count: ($parity_bridge[0].comparison_axis_count // 0),
      parity_lane_green: ($parity_lane[0].green == true),
      parity_lane_no_claim_gate: ($parity_lane[0].no_openra_parity_claim_gate == true),
      parity_lane_axis_count: ($parity_lane[0].lane_axis_count // 0),
      replay_compat_adapter_green: ($replay_compat[0].green == true),
      replay_summary_adapter_local_claimed: ($replay_compat[0].bevy_openra_replay_summary_adapter_claimed == true),
      imported_replay_digest_green: ($imported_digest[0].green == true),
      imported_replay_negative_case_count: ($imported_digest[0].negative_case_count // 0),
      imported_replay_detected_negative_case_count: ($imported_digest[0].detected_negative_case_count // 0),
      screen_ui_green: ($screen_ui[0].green == true),
      screen_ui_failed_gate_count: ($screen_ui[0].failed_gate_count // 999),
      screen_ui_reference_screen_count: ($screen_ui[0].openra_reference_screen_count // 0),
      screen_ui_interaction_surface_count: ($screen_ui[0].replicated_interaction_surface_count // 0),
      asset_parity_green: ($asset_parity[0].green == true),
      asset_parity_failed_gate_count: ($asset_parity[0].failed_gate_count // 999),
      asset_parity_project_owned_pixel_parity_claimed: ($asset_parity[0].trillionnium_owned_asset_pack_pixel_parity_claimed == true),
      asset_parity_ported_engine_module_count: ($asset_parity[0].ported_engine_module_count // 0),
      asset_parity_widget_root_count: ($asset_parity[0].openra_widget_root_count // 0),
      openra_parity_claimed: false,
      openra_runtime_compatibility_claimed: false,
      openra_replay_compatibility_claimed: false,
      openra_binary_replay_compatible: false,
      openra_network_order_stream_claimed: false,
      openra_headless_client_match_claimed: false,
      openra_engine_port_claimed: false,
      openra_full_engine_port_claimed: false,
      openra_pixel_perfect_asset_parity_claimed: false,
      openra_westwood_pixel_perfect_asset_parity_claimed: false,
      openra_asset_copied: false,
      third_party_asset_copied: false,
      sub_batch_3_local_review_complete: true,
      sub_batch_3_exit_rule_satisfied: true,
      sub_batch_4_unblocked_for_local_review: true,
      batch_3_exit_rule_satisfied: false,
      batch_4_unblocked_for_local_review: false,
      next_sub_batch_id: "first_contact_rts_data_extraction",
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
      live_multiplayer_claimed: false,
      no_credit_boundary: "local OpenRA parity/claim sub-batch 3 review only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, socket, hosted-service, live multiplayer, OpenRA runtime/replay/network/binary/headless compatibility, OpenRA engine/full-engine/pixel-perfect/Westwood asset parity, third-party asset copy, multi-node, live-traffic, or public-network credit",
      reviewer_next_action: "continue batch 3 with first_contact_rts_data_extraction; keep batch 4 blocked until all 273 runtime/data-boundary commits have commit-level review"
    }
  ' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_review_openra_parity_claim_batch_v1"
  and .status == "review_openra_parity_claim_sub_batch_3_reviewed"
  and .green == true
  and .batch_order == 3
  and .sub_batch_order == 3
  and .sub_batch_id == "openra_parity_and_claim_boundary"
  and .prior_sub_batch_reviewed_commit_count == 112
  and .reviewed_commit_count == 35
  and .required_reviewed_commit_count == 35
  and .batch_3_reviewed_commit_count == 147
  and .batch_3_remaining_commit_level_review_count == 126
  and .expected_hash_coverage_complete == true
  and .first_commit == "36814fcbbb"
  and .last_commit == "d3fb381a96"
  and .review_group_count == 4
  and (.review_group_counts | map(.count) | add) == 35
  and (.review_group_counts | map(select(.review_group == "semantic_parity_bridge_lane").count)[0]) == 5
  and (.review_group_counts | map(select(.review_group == "replay_order_import_boundary").count)[0]) == 23
  and (.review_group_counts | map(select(.review_group == "screen_ui_claim_boundary").count)[0]) == 3
  and (.review_group_counts | map(select(.review_group == "engine_asset_claim_boundary").count)[0]) == 4
  and (.commit_reviews | length) == 35
  and (.commit_reviews | all(.commit_level_review_complete == true))
  and (.commit_reviews | all(.unresolved == false))
  and .unresolved_commit_review_count == 0
  and .parity_bridge_green == true
  and .parity_bridge_no_claim_gate == true
  and .parity_bridge_comparison_axis_count == 4
  and .parity_lane_green == true
  and .parity_lane_no_claim_gate == true
  and .parity_lane_axis_count == 6
  and .replay_compat_adapter_green == true
  and .replay_summary_adapter_local_claimed == true
  and .imported_replay_digest_green == true
  and .imported_replay_negative_case_count == 7
  and .imported_replay_detected_negative_case_count == 7
  and .screen_ui_green == true
  and .screen_ui_failed_gate_count == 0
  and .screen_ui_reference_screen_count == 8
  and .screen_ui_interaction_surface_count == 8
  and .asset_parity_green == true
  and .asset_parity_failed_gate_count == 0
  and .asset_parity_project_owned_pixel_parity_claimed == true
  and .asset_parity_ported_engine_module_count >= 11
  and .asset_parity_widget_root_count == 4
  and .openra_parity_claimed == false
  and .openra_runtime_compatibility_claimed == false
  and .openra_replay_compatibility_claimed == false
  and .openra_binary_replay_compatible == false
  and .openra_network_order_stream_claimed == false
  and .openra_headless_client_match_claimed == false
  and .openra_engine_port_claimed == false
  and .openra_full_engine_port_claimed == false
  and .openra_pixel_perfect_asset_parity_claimed == false
  and .openra_westwood_pixel_perfect_asset_parity_claimed == false
  and .openra_asset_copied == false
  and .third_party_asset_copied == false
  and .sub_batch_3_local_review_complete == true
  and .sub_batch_3_exit_rule_satisfied == true
  and .sub_batch_4_unblocked_for_local_review == true
  and .batch_3_exit_rule_satisfied == false
  and .batch_4_unblocked_for_local_review == false
  and .next_sub_batch_id == "first_contact_rts_data_extraction"
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
  and .live_multiplayer_claimed == false
  and (.no_credit_boundary | contains("local OpenRA parity/claim sub-batch 3 review only"))
' "$SUMMARY" >/dev/null

{
  printf '# Trillionnium World Review OpenRA Parity/Claim Batch\n\n'
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
  printf -- '- sub-batch 3 local review complete / exit rule: `%s` / `%s`\n' \
    "$(jq -r '.sub_batch_3_local_review_complete' "$SUMMARY")" \
    "$(jq -r '.sub_batch_3_exit_rule_satisfied' "$SUMMARY")"
  printf -- '- next sub-batch: `%s`\n\n' "$(jq -r '.next_sub_batch_id' "$SUMMARY")"
  printf '## Review Groups\n\n'
  jq -r '.review_group_counts[] | "- `\(.review_group)`: `\(.count)` commits, unresolved `\(.unresolved_count)`"' "$SUMMARY"
  printf '\n## Claim Boundary\n\n'
  printf -- '- OpenRA runtime / replay / network / binary / headless claims: `%s` / `%s` / `%s` / `%s` / `%s`\n' \
    "$(jq -r '.openra_runtime_compatibility_claimed' "$SUMMARY")" \
    "$(jq -r '.openra_replay_compatibility_claimed' "$SUMMARY")" \
    "$(jq -r '.openra_network_order_stream_claimed' "$SUMMARY")" \
    "$(jq -r '.openra_binary_replay_compatible' "$SUMMARY")" \
    "$(jq -r '.openra_headless_client_match_claimed' "$SUMMARY")"
  printf -- '- OpenRA engine / full-engine / pixel-perfect / third-party asset claims: `%s` / `%s` / `%s` / `%s`\n' \
    "$(jq -r '.openra_engine_port_claimed' "$SUMMARY")" \
    "$(jq -r '.openra_full_engine_port_claimed' "$SUMMARY")" \
    "$(jq -r '.openra_pixel_perfect_asset_parity_claimed' "$SUMMARY")" \
    "$(jq -r '.third_party_asset_copied' "$SUMMARY")"
} >"$SUMMARY_MD"

printf 'TRILLIONNIUM_WORLD_REVIEW_OPENRA_PARITY_CLAIM_BATCH_GREEN %s %s\n' "$SUMMARY" "$SUMMARY_MD"
