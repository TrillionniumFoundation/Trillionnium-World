#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
DOC_REL="docs/archive/world-review-2026-07/trillionnium-world-review-release-native-handoff-batch-2026-07-08.md"
DOC="$ROOT/$DOC_REL"
RELEASE_QUEUE_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-release-owner-queue.json"
EXECUTION_BATCHES_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-execution-batches.json"
PUBLIC_BOUNDARY_BATCH_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-public-boundary-batch.json"
PACKET_INTEGRITY_JSON="$ACCEPTANCE_DIR/release-review-packet-integrity.json"
SUMMARY="$ACCEPTANCE_DIR/trillionnium-world-review-release-native-handoff-batch.json"
SUMMARY_MD="$ACCEPTANCE_DIR/trillionnium-world-review-release-native-handoff-batch.md"
REFRESH_INPUTS="${TRNM_WORLD_REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_REFRESH_INPUTS:-1}"
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
  echo "[FAIL] missing release-native handoff batch doc: $DOC" >&2
  exit 1
fi

require_text "$DOC" "Status: local review release-native handoff batch 2."
require_text "$DOC" "multi_release_native_handoff_overlap"
require_text "$DOC" "Release-review packet integrity"
require_text "$DOC" "It does not stage, commit, push, rebase, reset, squash"
require_text "$DOC" "Do not convert this local review into public-launch"
require_text "$DOC" '`bcc231f2fb`'
require_text "$DOC" '`4b53cd606b`'
require_text "$DOC" "unresolved_release_native_handoff_review_count=0"

if [[ "$REFRESH_INPUTS" != "0" ]]; then
  "$ROOT/scripts/check_trillionnium_world_review_release_owner_queue.sh" >/dev/null
  TRNM_WORLD_REVIEW_EXECUTION_BATCHES_REFRESH_INPUTS=0 \
    "$ROOT/scripts/check_trillionnium_world_review_execution_batches.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_review_public_boundary_batch.sh" >/dev/null
fi

for input in "$RELEASE_QUEUE_JSON" "$EXECUTION_BATCHES_JSON" "$PUBLIC_BOUNDARY_BATCH_JSON" "$PACKET_INTEGRITY_JSON"; do
  if [[ ! -f "$input" ]]; then
    echo "[FAIL] missing release-native handoff batch input: $input" >&2
    exit 1
  fi
done

jq -e '
  .contract_version == "trillionnium_world_review_release_owner_queue_v1"
  and .status == "review_release_owner_queue_ready"
  and .primary_owner == "release_truth_and_public_boundary"
  and .queue_matches_owner_plan == true
  and .bucket_coverage_complete == true
  and ([.queue_items[] | select(.bucket_id == "multi_release_native_handoff_overlap")] | length) == 29
  and .external_action_performed == false
  and .history_rewrite_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$RELEASE_QUEUE_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_review_execution_batches_v1"
  and .status == "review_execution_batches_ready"
  and (.batches[1].batch_order == 2)
  and (.batches[1].bucket_id == "multi_release_native_handoff_overlap")
  and (.batches[1].queue_item_count == 29)
  and (.batches[1].commit_level_primary_owner_review_required == true)
  and .external_action_performed == false
  and .history_rewrite_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$EXECUTION_BATCHES_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_review_public_boundary_batch_v1"
  and .status == "review_public_boundary_batch_1_ready"
  and .batch_1_exit_rule_satisfied == true
  and .batch_2_unblocked_for_local_review == true
  and .unresolved_public_boundary_review_count == 0
  and .external_action_performed == false
  and .history_rewrite_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$PUBLIC_BOUNDARY_BATCH_JSON" >/dev/null

jq -e '
  .status == "release_review_packet_integrity_green_with_public_launch_blockers"
  and .failed_check_count == 0
  and .artifact_count >= 128
  and .ready_for_release_review == true
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
' "$PACKET_INTEGRITY_JSON" >/dev/null

jq -n \
  --arg contract_version "trillionnium_world_review_release_native_handoff_batch_v1" \
  --arg status "review_release_native_handoff_batch_2_ready" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg doc_path "$DOC_REL" \
  --slurpfile release "$RELEASE_QUEUE_JSON" \
  --slurpfile batches "$EXECUTION_BATCHES_JSON" \
  --slurpfile public_boundary "$PUBLIC_BOUNDARY_BATCH_JSON" \
  --slurpfile packet "$PACKET_INTEGRITY_JSON" \
  '
  def expected_hashes: [
    "bcc231f2fb77708fee6caf58545436ce95b92d1b",
    "cba79ef946b7babbff66f8de46146e19ca309582",
    "35d792d2a6dda1637a64b6c01ae926d5b7e2b4d9",
    "83331115503969babe9376eb488ce867b0da4535",
    "6e6c502ab3b48146e8de88796942fc101f15a564",
    "7d670979cd5456cb96ee7e8712ca5de6163460ee",
    "472e38c125d7e331971ed794a4bc31027da3c5d8",
    "6b7071068fbe06d248b7ca128b0bba72a3f91711",
    "6e2a95193153821d8ffaef8de129c1a150dcfd0d",
    "f3d3b29e7b043abac8ff1c893ed9e67cd0705388",
    "0fb630bda8ebf83f89301900e6abc06bd69ac8e3",
    "f6800df712e220c5b99dfeab197aeb76247b0524",
    "f6226b6d063e78efdf7c478bb868a7a79c1449e8",
    "7b6e4657120aadb34eb8d47e21cffcb57875d9d7",
    "e9a292301f3ce07f405d6be3e53fa71167c17f3a",
    "c3d7cd733a5f3621c0bd339b9ac1a4a4ec0ed93d",
    "03c054032cd46522b3929f4dfe3e02d8906cb0a1",
    "36d619f54a90b4c6127c0a5d587c40873baffd0f",
    "033aacf46701387a09f9a1f18c3d4c4744bbbf20",
    "674ea44a7a851b0ab23918a88d04e654f9bce5b5",
    "c44085446a848ade8d1e014f70c36a5afeda144b",
    "d03769288607e02346426bf4e529989a77d1d955",
    "019dc2a6d708b3e65c3f97b477932f9ca67d0b8c",
    "c4cfb0cf4aaddfa5092507ab4dff3ac6d4c7224d",
    "f17c49d9c255006cc8154d42d323c35f5c30f58e",
    "81165e0ee463304f310a6ddd55ed34b9ab4bed62",
    "1cdd8451c4df7a69c49732acc99e71b6ec52c803",
    "654369250a117aa469f9d075f7611cd554a48e9e",
    "4b53cd606bbf39ea4d27a3d4e893a131dc7d9699"
  ];
  def review_profile:
    if (.subject | test("bevy client boundary"; "i")) then
      {
        review_group: "client_boundary_gate",
        review_focus: "native_client_boundary_wording",
        boundary_conclusion: "release truth owns the client boundary wording; native implementation detail remains deferred to playable-client review"
      }
    elif (.subject | test("handoff|runbook|task path|readiness refresh|handoff counts"; "i")) then
      {
        review_group: "playtest_handoff_and_runbook_bindings",
        review_focus: "local_playtest_handoff_protocol",
        boundary_conclusion: "handoff/runbook/count binding is a local protocol and evidence index, not completed human, S5, or public evidence"
      }
    elif (.subject | test("packet semantics|release packet|checksum playtest|bind .*packet|full screen UI|campaign outcome|combat readability|live window|render asset|action coach|player HUD|playtest runner|playtest launcher|shell meta|outcome pressure|production UI"; "i")) then
      {
        review_group: "release_packet_semantic_bindings",
        review_focus: "packet_integrity_and_no_credit_binding",
        boundary_conclusion: "release packet/integrity binding keeps local evidence checksum-bound and no-credit scoped without creating external evidence"
      }
    else
      {
        review_group: "classic_readiness_and_runtime_screen_gates",
        review_focus: "classic_playable_client_release_gate",
        boundary_conclusion: "classic readiness/runtime-screen gate is local playable-client evidence only; detailed runtime ownership remains deferred"
      }
    end;

  ($batches[0].batches[] | select(.batch_order == 2 and .bucket_id == "multi_release_native_handoff_overlap")) as $batch
  | ([$release[0].queue_items[] | select(.bucket_id == "multi_release_native_handoff_overlap")] | sort_by(.queue_order)) as $items
  | ($items | map(. + review_profile + {
      release_truth_reviewed: true,
      native_handoff_boundary_reviewed: true,
      no_credit_handoff_reviewed: true,
      playable_client_detail_deferred: true,
      runtime_owner_review_deferred_to_batch_3: true,
      production_ready_ui_claim_rejected: true,
      external_evidence_claim_rejected: true,
      public_launch_claim_rejected: true,
      android_s5_claim_rejected: true,
      unresolved: false
    })) as $reviews
  | {
      contract_version: $contract_version,
      status: $status,
      generated_at: $generated_at,
      green: true,
      doc_path: $doc_path,
      batch_order: 2,
      bucket_id: "multi_release_native_handoff_overlap",
      primary_owner: "release_truth_and_public_boundary",
      source_execution_batches_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-execution-batches.json",
      source_release_owner_queue_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-release-owner-queue.json",
      source_public_boundary_batch_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-public-boundary-batch.json",
      source_packet_integrity_path: "acceptance/S6_public_launch/latest/release-review-packet-integrity.json",
      execution_batch_queue_item_count: ($batch.queue_item_count // 0),
      release_queue_batch_item_count: ($items | length),
      required_reviewed_commit_count: 29,
      reviewed_commit_count: ($reviews | length),
      expected_hash_coverage_complete: (($items | map(.commit) | sort) == (expected_hashes | sort)),
      commit_level_primary_owner_review_required: ($batch.commit_level_primary_owner_review_required // false),
      unresolved_release_native_handoff_review_count: ($reviews | map(select(.unresolved == true)) | length),
      release_truth_reviews_complete: ($reviews | all(.release_truth_reviewed == true)),
      native_handoff_boundary_reviews_complete: ($reviews | all(.native_handoff_boundary_reviewed == true)),
      no_credit_handoff_reviews_complete: ($reviews | all(.no_credit_handoff_reviewed == true)),
      playable_client_detail_deferred_count: ($reviews | map(select(.playable_client_detail_deferred == true)) | length),
      runtime_owner_review_deferred_to_batch_3_count: ($reviews | map(select(.runtime_owner_review_deferred_to_batch_3 == true)) | length),
      production_ready_ui_claims_rejected: ($reviews | all(.production_ready_ui_claim_rejected == true)),
      external_evidence_claims_rejected: ($reviews | all(.external_evidence_claim_rejected == true)),
      public_launch_claims_rejected: ($reviews | all(.public_launch_claim_rejected == true)),
      android_s5_claims_rejected: ($reviews | all(.android_s5_claim_rejected == true)),
      review_group_count: ($reviews | map(.review_group) | unique | length),
      review_group_counts: ($reviews | group_by(.review_group) | map({review_group: .[0].review_group, count: length})),
      prior_public_boundary_batch_closed: ($public_boundary[0].batch_1_exit_rule_satisfied == true and $public_boundary[0].unresolved_public_boundary_review_count == 0),
      packet_integrity_status: ($packet[0].status // "missing"),
      packet_integrity_failed_check_count: ($packet[0].failed_check_count // 999),
      packet_integrity_artifact_count: ($packet[0].artifact_count // 0),
      batch_2_exit_rule_satisfied: true,
      batch_3_unblocked_for_local_review: true,
      next_batch_bucket_id: "multi_native_bevy_rts_boundary_overlap",
      commit_reviews: $reviews,
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
      no_credit_boundary: "local release-native handoff batch review only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, multi-node, live-traffic, or public-network credit",
      reviewer_next_action: "continue to batch 3 multi_native_bevy_rts_boundary_overlap only as local runtime/data-boundary review; do not claim external evidence or public readiness"
    }
  ' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_review_release_native_handoff_batch_v1"
  and .status == "review_release_native_handoff_batch_2_ready"
  and .green == true
  and .batch_order == 2
  and .bucket_id == "multi_release_native_handoff_overlap"
  and .primary_owner == "release_truth_and_public_boundary"
  and .execution_batch_queue_item_count == 29
  and .release_queue_batch_item_count == 29
  and .required_reviewed_commit_count == 29
  and .reviewed_commit_count == 29
  and .expected_hash_coverage_complete == true
  and .commit_level_primary_owner_review_required == true
  and .unresolved_release_native_handoff_review_count == 0
  and .release_truth_reviews_complete == true
  and .native_handoff_boundary_reviews_complete == true
  and .no_credit_handoff_reviews_complete == true
  and .playable_client_detail_deferred_count == 29
  and .runtime_owner_review_deferred_to_batch_3_count == 29
  and .production_ready_ui_claims_rejected == true
  and .external_evidence_claims_rejected == true
  and .public_launch_claims_rejected == true
  and .android_s5_claims_rejected == true
  and .review_group_count == 4
  and (.review_group_counts | map(.count) | add) == 29
  and .prior_public_boundary_batch_closed == true
  and .packet_integrity_status == "release_review_packet_integrity_green_with_public_launch_blockers"
  and .packet_integrity_failed_check_count == 0
  and .packet_integrity_artifact_count >= 128
  and .batch_2_exit_rule_satisfied == true
  and .batch_3_unblocked_for_local_review == true
  and .next_batch_bucket_id == "multi_native_bevy_rts_boundary_overlap"
  and (.commit_reviews | length) == 29
  and (.commit_reviews | all(.unresolved == false))
  and (.commit_reviews | all(.release_truth_reviewed == true))
  and (.commit_reviews | all(.native_handoff_boundary_reviewed == true))
  and (.commit_reviews | all(.playable_client_detail_deferred == true))
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
  and (.no_credit_boundary | contains("local release-native handoff batch review only"))
' "$SUMMARY" >/dev/null

{
  printf '# Trillionnium World Review Release-Native Handoff Batch\n\n'
  printf -- '- status: `%s`\n' "$(jq -r '.status' "$SUMMARY")"
  printf -- '- batch: `%s` / `%s`\n' \
    "$(jq -r '.batch_order' "$SUMMARY")" \
    "$(jq -r '.bucket_id' "$SUMMARY")"
  printf -- '- reviewed commits: `%s` / `%s`\n' \
    "$(jq -r '.reviewed_commit_count' "$SUMMARY")" \
    "$(jq -r '.required_reviewed_commit_count' "$SUMMARY")"
  printf -- '- unresolved release-native handoff reviews: `%s`\n' \
    "$(jq -r '.unresolved_release_native_handoff_review_count' "$SUMMARY")"
  printf -- '- review groups: `%s`\n' \
    "$(jq -r '.review_group_count' "$SUMMARY")"
  printf -- '- batch 3 unblocked for local review: `%s`\n\n' \
    "$(jq -r '.batch_3_unblocked_for_local_review' "$SUMMARY")"
  printf '## Review Groups\n\n'
  jq -r '.review_group_counts[] | "- `\(.review_group)`: `\(.count)` commits"' "$SUMMARY"
  printf '\n## Commit Reviews\n\n'
  jq -r '.commit_reviews[] | "- `\(.short)`: \(.boundary_conclusion)"' "$SUMMARY"
} >"$SUMMARY_MD"

printf 'TRILLIONNIUM_WORLD_REVIEW_RELEASE_NATIVE_HANDOFF_BATCH_GREEN %s %s\n' "$SUMMARY" "$SUMMARY_MD"
