#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
DOC_REL="docs/development/trillionnium-world-review-runtime-boundary-batch-2026-07-08.md"
DOC="$ROOT/$DOC_REL"
RUNTIME_QUEUE_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-runtime-owner-queue.json"
EXECUTION_BATCHES_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-execution-batches.json"
RELEASE_NATIVE_HANDOFF_BATCH_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-release-native-handoff-batch.json"
PACKET_INTEGRITY_JSON="$ACCEPTANCE_DIR/release-review-packet-integrity.json"
SUMMARY="$ACCEPTANCE_DIR/trillionnium-world-review-runtime-boundary-batch.json"
SUMMARY_MD="$ACCEPTANCE_DIR/trillionnium-world-review-runtime-boundary-batch.md"
REFRESH_INPUTS="${TRNM_WORLD_REVIEW_RUNTIME_BOUNDARY_BATCH_REFRESH_INPUTS:-1}"
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
  echo "[FAIL] missing runtime-boundary batch doc: $DOC" >&2
  exit 1
fi

require_text "$DOC" "Status: local review runtime-boundary batch 3 shard plan."
require_text "$DOC" "multi_native_bevy_rts_boundary_overlap"
require_text "$DOC" "Runtime Sub-Batches"
require_text "$DOC" '`runtime_core_semantics`'
require_text "$DOC" '`runtime_adapter_and_online_boundary`'
require_text "$DOC" '`openra_parity_and_claim_boundary`'
require_text "$DOC" '`first_contact_player_surface_cues`'
require_text "$DOC" "It does not stage, commit, push, rebase, reset, squash"
require_text "$DOC" "Do not convert this local shard plan into public-launch"
require_text "$DOC" "batch_3_exit_rule_satisfied=false"
require_text "$DOC" "batch_4_unblocked_for_local_review=false"

if [[ "$REFRESH_INPUTS" != "0" ]]; then
  "$ROOT/scripts/check_trillionnium_world_review_runtime_owner_queue.sh" >/dev/null
  TRNM_WORLD_REVIEW_EXECUTION_BATCHES_REFRESH_INPUTS=0 \
    "$ROOT/scripts/check_trillionnium_world_review_execution_batches.sh" >/dev/null
  TRNM_WORLD_REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_REFRESH_INPUTS=0 \
    "$ROOT/scripts/check_trillionnium_world_review_release_native_handoff_batch.sh" >/dev/null
fi

for input in "$RUNTIME_QUEUE_JSON" "$EXECUTION_BATCHES_JSON" "$RELEASE_NATIVE_HANDOFF_BATCH_JSON" "$PACKET_INTEGRITY_JSON"; do
  if [[ ! -f "$input" ]]; then
    echo "[FAIL] missing runtime-boundary batch input: $input" >&2
    exit 1
  fi
done

jq -e '
  .contract_version == "trillionnium_world_review_runtime_owner_queue_v1"
  and .status == "review_runtime_owner_queue_ready"
  and .primary_owner == "rts_runtime_data_boundaries"
  and .queue_matches_owner_plan == true
  and .bucket_coverage_complete == true
  and ([.queue_items[] | select(.bucket_id == "multi_native_bevy_rts_boundary_overlap")] | length) == 273
  and .external_action_performed == false
  and .history_rewrite_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$RUNTIME_QUEUE_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_review_execution_batches_v1"
  and .status == "review_execution_batches_ready"
  and (.batches[2].batch_order == 3)
  and (.batches[2].bucket_id == "multi_native_bevy_rts_boundary_overlap")
  and (.batches[2].queue_item_count == 273)
  and (.batches[2].commit_level_primary_owner_review_required == true)
  and .external_action_performed == false
  and .history_rewrite_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$EXECUTION_BATCHES_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_review_release_native_handoff_batch_v1"
  and .status == "review_release_native_handoff_batch_2_ready"
  and .batch_2_exit_rule_satisfied == true
  and .batch_3_unblocked_for_local_review == true
  and .unresolved_release_native_handoff_review_count == 0
  and .external_action_performed == false
  and .history_rewrite_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$RELEASE_NATIVE_HANDOFF_BATCH_JSON" >/dev/null

jq -e '
  .status == "release_review_packet_integrity_green_with_public_launch_blockers"
  and .failed_check_count == 0
  and .artifact_count >= 128
  and .ready_for_release_review == true
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
' "$PACKET_INTEGRITY_JSON" >/dev/null

jq -n \
  --arg contract_version "trillionnium_world_review_runtime_boundary_batch_v1" \
  --arg status "review_runtime_boundary_batch_3_sharded" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg doc_path "$DOC_REL" \
  --slurpfile runtime "$RUNTIME_QUEUE_JSON" \
  --slurpfile batches "$EXECUTION_BATCHES_JSON" \
  --slurpfile handoff "$RELEASE_NATIVE_HANDOFF_BATCH_JSON" \
  --slurpfile packet "$PACKET_INTEGRITY_JSON" \
  '
  def shard_profile:
    (.subject | ascii_downcase) as $s
    | if ($s | test("^feat: add openra-like|^feat: validate openra-like")) then
        {
          sub_batch_id: "runtime_core_semantics",
          sub_batch_order: 1,
          review_focus: "renderer_neutral_rts_rule_semantics",
          boundary_gate: "RTS rule state and resolver semantics must stay renderer-neutral before Bevy consumes them."
        }
      elif ($s | test("^feat: add rts bevy runtime adapter|^feat: add rts evidence adapter|^feat: add rts online protocol|^feat: move rts .*runtime adapter|^fix: expose rts online|^fix: expose offline adapter|^fix: move offline adapter")) then
        {
          sub_batch_id: "runtime_adapter_and_online_boundary",
          sub_batch_order: 2,
          review_focus: "one_way_runtime_adapter_and_online_handoff",
          boundary_gate: "Adapters and online/offline handoffs must consume runtime data without making Bevy the data source."
        }
      elif ($s | test("openra|parity|replay|importer|adapter evidence|claim")) then
        {
          sub_batch_id: "openra_parity_and_claim_boundary",
          sub_batch_order: 3,
          review_focus: "openra_style_claim_scope",
          boundary_gate: "OpenRA-style evidence must remain semantic and must not claim asset, replay, protocol, or network compatibility."
        }
      elif ($s | test("^feat: move first contact|^fix: move first contact renderer model into rts data|^fix: derive openra preview actors from rts data|^fix: move first contact preview actors into rts data|^refactor: move first contact samples to rts data|^refactor: move first contact labels to rts data")) then
        {
          sub_batch_id: "first_contact_rts_data_extraction",
          sub_batch_order: 4,
          review_focus: "first_contact_renderer_neutral_data_moves",
          boundary_gate: "First Contact authored data may move into RTS data, but draw math and live renderer behavior must stay out."
        }
      elif ($s | test("^fix: move first contact .* evidence|^refactor: move first contact .* evidence")) then
        {
          sub_batch_id: "rts_evidence_crate_boundary",
          sub_batch_order: 5,
          review_focus: "rts_evidence_payload_boundary",
          boundary_gate: "Evidence crates may carry review payloads but must not become playable renderer ownership."
        }
      elif ($s | test("^fix: expose|^fix: carry first contact reviews|reuse bevy artifact binary in rts checks")) then
        {
          sub_batch_id: "review_evidence_exposure_boundary",
          sub_batch_order: 6,
          review_focus: "local_review_artifact_exposure",
          boundary_gate: "Exposed review artifacts remain local evidence surfaces and grant no public/S5/commercial credit."
        }
      elif ($s | test("^test: gate classic model catalog|^refactor: move first contact .* runtime to rts bevy runtime|^refactor: split classic .* renderer|^fix: expose classic .* counts")) then
        {
          sub_batch_id: "bevy_runtime_renderer_boundary",
          sub_batch_order: 7,
          review_focus: "bevy_runtime_renderer_consumer_boundary",
          boundary_gate: "Bevy runtime and renderer split surfaces must stay consumers/adapters, not data truth sources."
        }
      else
        {
          sub_batch_id: "first_contact_player_surface_cues",
          sub_batch_order: 8,
          review_focus: "downstream_first_contact_player_surface_cues",
          boundary_gate: "Player-surface cue changes remain renderer/readability work and stay human-playtest gated."
        }
      end;

  ($batches[0].batches[] | select(.batch_order == 3 and .bucket_id == "multi_native_bevy_rts_boundary_overlap")) as $batch
  | ([$runtime[0].queue_items[] | select(.bucket_id == "multi_native_bevy_rts_boundary_overlap")] | sort_by(.queue_order)) as $items
  | ($items | map(. + shard_profile + {
      runtime_boundary_batch_3_item: true,
      shard_assigned: true,
      commit_level_primary_owner_review_complete: false,
      external_evidence_claim_rejected: true,
      public_launch_claim_rejected: true,
      android_s5_claim_rejected: true,
      production_ready_ui_claim_rejected: true,
      beta_claim_rejected: true,
      commercial_claim_rejected: true
    }) | sort_by(.sub_batch_order, .queue_order)) as $shards
  | ($shards
      | group_by(.sub_batch_id)
      | map({
          sub_batch_id: .[0].sub_batch_id,
          sub_batch_order: .[0].sub_batch_order,
          review_focus: .[0].review_focus,
          boundary_gate: .[0].boundary_gate,
          count: length,
          reviewed_commit_count: 0,
          remaining_commit_level_review_count: length,
          first_commit: .[0].short,
          last_commit: .[-1].short
        })
      | sort_by(.sub_batch_order)) as $sub_batches
  | {
      contract_version: $contract_version,
      status: $status,
      generated_at: $generated_at,
      green: true,
      doc_path: $doc_path,
      batch_order: 3,
      bucket_id: "multi_native_bevy_rts_boundary_overlap",
      primary_owner: "rts_runtime_data_boundaries",
      source_execution_batches_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-execution-batches.json",
      source_runtime_owner_queue_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-runtime-owner-queue.json",
      source_release_native_handoff_batch_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-release-native-handoff-batch.json",
      source_packet_integrity_path: "acceptance/S6_public_launch/latest/release-review-packet-integrity.json",
      execution_batch_queue_item_count: ($batch.queue_item_count // 0),
      runtime_overlap_commit_count: ($items | length),
      sharded_commit_count: ($shards | length),
      sub_batch_count: ($sub_batches | length),
      nonempty_sub_batch_count: ($sub_batches | map(select(.count > 0)) | length),
      sub_batches: $sub_batches,
      first_sub_batch_id: ($sub_batches[0].sub_batch_id // "missing"),
      highest_risk_sub_batch_id: "runtime_core_semantics",
      completed_sub_batch_count: 0,
      reviewed_commit_count: 0,
      remaining_commit_level_review_count: ($shards | length),
      all_commits_assigned_to_one_sub_batch: (($shards | map(.shard_assigned) | all) and (($shards | length) == ($items | length))),
      commit_level_primary_owner_review_required: ($batch.commit_level_primary_owner_review_required // false),
      prior_release_native_handoff_batch_closed: ($handoff[0].batch_2_exit_rule_satisfied == true and $handoff[0].batch_3_unblocked_for_local_review == true),
      packet_integrity_status: ($packet[0].status // "missing"),
      packet_integrity_failed_check_count: ($packet[0].failed_check_count // 999),
      packet_integrity_artifact_count: ($packet[0].artifact_count // 0),
      batch_3_entry_rule_satisfied: true,
      batch_3_exit_rule_satisfied: false,
      batch_4_unblocked_for_local_review: false,
      next_sub_batch_id: "runtime_core_semantics",
      next_batch_bucket_id: "unclassified_generated_count_surface",
      commit_shards: $shards,
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
      no_credit_boundary: "local runtime-boundary batch 3 shard plan only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, multi-node, live-traffic, or public-network credit",
      reviewer_next_action: "review sub-batch runtime_core_semantics first, then runtime_adapter_and_online_boundary; keep batch 4 blocked until all 273 runtime/data-boundary commits have commit-level review"
    }
  ' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_review_runtime_boundary_batch_v1"
  and .status == "review_runtime_boundary_batch_3_sharded"
  and .green == true
  and .batch_order == 3
  and .bucket_id == "multi_native_bevy_rts_boundary_overlap"
  and .primary_owner == "rts_runtime_data_boundaries"
  and .execution_batch_queue_item_count == 273
  and .runtime_overlap_commit_count == 273
  and .sharded_commit_count == 273
  and .sub_batch_count == 8
  and .nonempty_sub_batch_count == 8
  and (.sub_batches | map(.count) | add) == 273
  and (.sub_batches | map(select(.sub_batch_id == "runtime_core_semantics").count)[0]) == 55
  and (.sub_batches | map(select(.sub_batch_id == "runtime_adapter_and_online_boundary").count)[0]) == 57
  and (.sub_batches | map(select(.sub_batch_id == "openra_parity_and_claim_boundary").count)[0]) == 35
  and (.sub_batches | map(select(.sub_batch_id == "first_contact_rts_data_extraction").count)[0]) == 24
  and (.sub_batches | map(select(.sub_batch_id == "rts_evidence_crate_boundary").count)[0]) == 20
  and (.sub_batches | map(select(.sub_batch_id == "review_evidence_exposure_boundary").count)[0]) == 12
  and (.sub_batches | map(select(.sub_batch_id == "bevy_runtime_renderer_boundary").count)[0]) == 7
  and (.sub_batches | map(select(.sub_batch_id == "first_contact_player_surface_cues").count)[0]) == 63
  and .first_sub_batch_id == "runtime_core_semantics"
  and .highest_risk_sub_batch_id == "runtime_core_semantics"
  and .completed_sub_batch_count == 0
  and .reviewed_commit_count == 0
  and .remaining_commit_level_review_count == 273
  and .all_commits_assigned_to_one_sub_batch == true
  and .commit_level_primary_owner_review_required == true
  and .prior_release_native_handoff_batch_closed == true
  and .packet_integrity_status == "release_review_packet_integrity_green_with_public_launch_blockers"
  and .packet_integrity_failed_check_count == 0
  and .packet_integrity_artifact_count >= 128
  and .batch_3_entry_rule_satisfied == true
  and .batch_3_exit_rule_satisfied == false
  and .batch_4_unblocked_for_local_review == false
  and .next_sub_batch_id == "runtime_core_semantics"
  and .next_batch_bucket_id == "unclassified_generated_count_surface"
  and (.commit_shards | length) == 273
  and (.commit_shards | all(.shard_assigned == true))
  and (.commit_shards | all(.commit_level_primary_owner_review_complete == false))
  and (.commit_shards | all(.external_evidence_claim_rejected == true))
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
  and .live_public_exposure_performed == false
  and .android_device_capture_performed == false
  and (.no_credit_boundary | contains("local runtime-boundary batch 3 shard plan only"))
' "$SUMMARY" >/dev/null

{
  printf '# Trillionnium World Review Runtime-Boundary Batch\n\n'
  printf -- '- status: `%s`\n' "$(jq -r '.status' "$SUMMARY")"
  printf -- '- batch: `%s` / `%s`\n' \
    "$(jq -r '.batch_order' "$SUMMARY")" \
    "$(jq -r '.bucket_id' "$SUMMARY")"
  printf -- '- sharded commits: `%s` / `%s`\n' \
    "$(jq -r '.sharded_commit_count' "$SUMMARY")" \
    "$(jq -r '.runtime_overlap_commit_count' "$SUMMARY")"
  printf -- '- sub-batches: `%s`\n' "$(jq -r '.sub_batch_count' "$SUMMARY")"
  printf -- '- remaining commit-level reviews: `%s`\n' \
    "$(jq -r '.remaining_commit_level_review_count' "$SUMMARY")"
  printf -- '- batch 3 entry / exit / batch 4 unblock: `%s` / `%s` / `%s`\n\n' \
    "$(jq -r '.batch_3_entry_rule_satisfied' "$SUMMARY")" \
    "$(jq -r '.batch_3_exit_rule_satisfied' "$SUMMARY")" \
    "$(jq -r '.batch_4_unblocked_for_local_review' "$SUMMARY")"
  printf '## Runtime Sub-Batches\n\n'
  jq -r '.sub_batches[] | "- `\(.sub_batch_order)` / `\(.sub_batch_id)`: `\(.count)` commits, next reviews `\(.remaining_commit_level_review_count)`"' "$SUMMARY"
} >"$SUMMARY_MD"

printf 'TRILLIONNIUM_WORLD_REVIEW_RUNTIME_BOUNDARY_BATCH_GREEN %s %s\n' "$SUMMARY" "$SUMMARY_MD"
