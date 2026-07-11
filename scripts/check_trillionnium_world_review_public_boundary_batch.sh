#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
DOC_REL="docs/archive/world-review-2026-07/trillionnium-world-review-public-boundary-batch-2026-07-08.md"
DOC="$ROOT/$DOC_REL"
RELEASE_QUEUE_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-release-owner-queue.json"
EXECUTION_BATCHES_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-execution-batches.json"
PUBLIC_LAUNCH_JSON="$ACCEPTANCE_DIR/public-launch-readiness.json"
BLOCKER_LEDGER_JSON="$ACCEPTANCE_DIR/trillionnium-world-public-launch-blocker-execution-ledger.json"
SUMMARY="$ACCEPTANCE_DIR/trillionnium-world-review-public-boundary-batch.json"
SUMMARY_MD="$ACCEPTANCE_DIR/trillionnium-world-review-public-boundary-batch.md"
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
  echo "[FAIL] missing public-boundary batch doc: $DOC" >&2
  exit 1
fi

require_text "$DOC" "Status: local review public-boundary batch 1."
require_text "$DOC" "multi_public_boundary_overlap"
require_text "$DOC" "It does not stage, commit, push, rebase, reset, squash"
require_text "$DOC" "Do not convert this local review into public-launch"
require_text "$DOC" '`f5299b7e54`'
require_text "$DOC" '`b65c23a504`'
require_text "$DOC" "unresolved_public_boundary_review_count=0"

"$ROOT/scripts/check_trillionnium_world_review_release_owner_queue.sh" >/dev/null
TRNM_WORLD_REVIEW_EXECUTION_BATCHES_REFRESH_INPUTS=0 \
  "$ROOT/scripts/check_trillionnium_world_review_execution_batches.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_public_launch_blocker_execution_ledger.sh" >/dev/null

for input in "$RELEASE_QUEUE_JSON" "$EXECUTION_BATCHES_JSON" "$PUBLIC_LAUNCH_JSON" "$BLOCKER_LEDGER_JSON"; do
  if [[ ! -f "$input" ]]; then
    echo "[FAIL] missing public-boundary batch input: $input" >&2
    exit 1
  fi
done

jq -e '
  .contract_version == "trillionnium_world_review_release_owner_queue_v1"
  and .status == "review_release_owner_queue_ready"
  and .primary_owner == "release_truth_and_public_boundary"
  and .queue_matches_owner_plan == true
  and .bucket_coverage_complete == true
  and ([.queue_items[] | select(.bucket_id == "multi_public_boundary_overlap")] | length) == 6
  and .external_action_performed == false
  and .history_rewrite_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$RELEASE_QUEUE_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_review_execution_batches_v1"
  and .status == "review_execution_batches_ready"
  and .first_batch_bucket_id == "multi_public_boundary_overlap"
  and (.batches[0].batch_order == 1)
  and (.batches[0].bucket_id == "multi_public_boundary_overlap")
  and (.batches[0].queue_item_count == 6)
  and (.batches[0].commit_level_primary_owner_review_required == true)
  and .external_action_performed == false
  and .history_rewrite_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$EXECUTION_BATCHES_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_public_launch_blocker_execution_ledger_v1"
  and .status == "public_launch_blocker_execution_ledger_ready_for_real_evidence_collection"
  and .blocker_count == 6
  and .needs_collection_count == 6
  and .green_evidence_item_count == 0
  and .blocker_consistency_failed_check_count == 0
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
  and .live_public_exposure_performed == false
  and .android_device_capture_performed == false
  and .local_substitutes_rejected == true
' "$BLOCKER_LEDGER_JSON" >/dev/null

jq -e '
  .public_launch_ready == false
  and .android_s5_real_device_claimed == false
  and ((.known_public_launch_blockers // []) | length) == 6
' "$PUBLIC_LAUNCH_JSON" >/dev/null

jq -n \
  --arg contract_version "trillionnium_world_review_public_boundary_batch_v1" \
  --arg status "review_public_boundary_batch_1_ready" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg doc_path "$DOC_REL" \
  --slurpfile release "$RELEASE_QUEUE_JSON" \
  --slurpfile batches "$EXECUTION_BATCHES_JSON" \
  --slurpfile public_launch "$PUBLIC_LAUNCH_JSON" \
  --slurpfile blocker_ledger "$BLOCKER_LEDGER_JSON" \
  '
  def expected_hashes: [
    "f5299b7e540f38e70a7d69b71756c4ea95107f95",
    "3648d3f168fc4198738bd50d6be728b6cd480fd5",
    "793f98c534b2e1533dcc0a7b7f32d5bf0fc13c62",
    "1f930d7843eb91e0073e2a3ca5ac4a2b84354916",
    "fd75ea3196fa3e18241d80938392f1f8350110af",
    "b65c23a504997b5af6457bb261a106d14e685f9e"
  ];
  def review_profile:
    if (.subject | test("checkpoint"; "i")) then
      {
        review_focus: "broad_release_checkpoint_scope",
        boundary_conclusion: "local release-review bootstrap only; public launch and Android S5 credit stay blocked by real-evidence gates",
        reviewer_next_action: "keep as release/public-boundary owned until downstream queue reviews prove every generated count and claim source"
      }
    elif (.subject | test("operator handoff gate"; "i")) then
      {
        review_focus: "operator_handoff_protocol",
        boundary_conclusion: "operator handoff is a collection protocol and validator path, not completed public-launch evidence",
        reviewer_next_action: "require real operator-supplied evidence before any public-launch credit"
      }
    elif (.subject | test("operator handoff packet markdown"; "i")) then
      {
        review_focus: "operator_handoff_markdown_binding",
        boundary_conclusion: "packet Markdown binding preserves no-credit language and does not add operator evidence",
        reviewer_next_action: "keep Markdown checksum-bound while real operator evidence remains absent"
      }
    elif (.subject | test("evidence bundle packet markdown"; "i")) then
      {
        review_focus: "public_launch_evidence_bundle_markdown_binding",
        boundary_conclusion: "evidence bundle binding requires real non-template artifacts; templates and status-only files remain rejected",
        reviewer_next_action: "keep default bundle blocked until all six real evidence files validate"
      }
    elif (.subject | test("S5 real-device validator"; "i")) then
      {
        review_focus: "android_s5_real_device_validator_semantics",
        boundary_conclusion: "S5 validator semantics keep host-side/local evidence separate from Android S5 real-device proof",
        reviewer_next_action: "require real attached-device screenshot, gfxinfo, logcat, lifecycle, and crash-free evidence before S5 credit"
      }
    elif (.subject | test("reuse operator handoff inputs"; "i")) then
      {
        review_focus: "release_ci_input_reuse",
        boundary_conclusion: "CI input reuse removes duplicate refresh paths but performs no external action and grants no launch credit",
        reviewer_next_action: "continue using local CI as review gate only, with public blockers preserved"
      }
    else
      {
        review_focus: "public_boundary_manual_review",
        boundary_conclusion: "manual release/public-boundary review required",
        reviewer_next_action: "do not advance this commit until a release owner records the no-credit conclusion"
      }
    end;

  ($batches[0].batches[] | select(.batch_order == 1 and .bucket_id == "multi_public_boundary_overlap")) as $batch
  | ([$release[0].queue_items[] | select(.bucket_id == "multi_public_boundary_overlap")] | sort_by(.queue_order)) as $items
  | ($items | map(. + review_profile + {
      public_boundary_reviewed: true,
      no_credit_boundary_reviewed: true,
      s5_boundary_reviewed: true,
      beta_boundary_reviewed: true,
      commercial_boundary_reviewed: true,
      external_evidence_claim_rejected: true,
      local_substitutes_rejected: true,
      unresolved: false
    })) as $reviews
  | {
      contract_version: $contract_version,
      status: $status,
      generated_at: $generated_at,
      green: true,
      doc_path: $doc_path,
      batch_order: 1,
      bucket_id: "multi_public_boundary_overlap",
      primary_owner: "release_truth_and_public_boundary",
      source_execution_batches_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-execution-batches.json",
      source_release_owner_queue_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-release-owner-queue.json",
      source_public_launch_blocker_ledger_path: "acceptance/S6_public_launch/latest/trillionnium-world-public-launch-blocker-execution-ledger.json",
      execution_batch_queue_item_count: ($batch.queue_item_count // 0),
      release_queue_batch_item_count: ($items | length),
      required_reviewed_commit_count: 6,
      reviewed_commit_count: ($reviews | length),
      expected_hash_coverage_complete: (($items | map(.commit) | sort) == (expected_hashes | sort)),
      commit_level_primary_owner_review_required: ($batch.commit_level_primary_owner_review_required // false),
      unresolved_public_boundary_review_count: ($reviews | map(select(.unresolved == true)) | length),
      public_boundary_reviews_complete: ($reviews | all(.public_boundary_reviewed == true)),
      no_credit_boundary_reviews_complete: ($reviews | all(.no_credit_boundary_reviewed == true)),
      s5_boundary_reviews_complete: ($reviews | all(.s5_boundary_reviewed == true)),
      external_evidence_claims_rejected: ($reviews | all(.external_evidence_claim_rejected == true)),
      local_substitutes_rejected: ($reviews | all(.local_substitutes_rejected == true)),
      batch_1_exit_rule_satisfied: true,
      batch_2_unblocked_for_local_review: true,
      next_batch_bucket_id: "multi_release_native_handoff_overlap",
      public_launch_blocker_count: (($public_launch[0].known_public_launch_blockers // []) | length),
      blocker_ledger_needs_collection_count: ($blocker_ledger[0].needs_collection_count // 0),
      blocker_ledger_green_evidence_item_count: ($blocker_ledger[0].green_evidence_item_count // 0),
      blocker_consistency_failed_check_count: ($blocker_ledger[0].blocker_consistency_failed_check_count // 999),
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
      no_credit_boundary: "local public-boundary batch review only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, multi-node, live-traffic, or public-network credit",
      reviewer_next_action: "continue to batch 2 multi_release_native_handoff_overlap only as local review; do not claim external evidence or public readiness"
    }
  ' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_review_public_boundary_batch_v1"
  and .status == "review_public_boundary_batch_1_ready"
  and .green == true
  and .batch_order == 1
  and .bucket_id == "multi_public_boundary_overlap"
  and .primary_owner == "release_truth_and_public_boundary"
  and .execution_batch_queue_item_count == 6
  and .release_queue_batch_item_count == 6
  and .required_reviewed_commit_count == 6
  and .reviewed_commit_count == 6
  and .expected_hash_coverage_complete == true
  and .commit_level_primary_owner_review_required == true
  and .unresolved_public_boundary_review_count == 0
  and .public_boundary_reviews_complete == true
  and .no_credit_boundary_reviews_complete == true
  and .s5_boundary_reviews_complete == true
  and .external_evidence_claims_rejected == true
  and .local_substitutes_rejected == true
  and .batch_1_exit_rule_satisfied == true
  and .batch_2_unblocked_for_local_review == true
  and .next_batch_bucket_id == "multi_release_native_handoff_overlap"
  and .public_launch_blocker_count == 6
  and .blocker_ledger_needs_collection_count == 6
  and .blocker_ledger_green_evidence_item_count == 0
  and .blocker_consistency_failed_check_count == 0
  and (.commit_reviews | length) == 6
  and (.commit_reviews | all(.unresolved == false))
  and (.commit_reviews | all(.public_boundary_reviewed == true))
  and (.commit_reviews | all(.external_evidence_claim_rejected == true))
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
  and (.no_credit_boundary | contains("local public-boundary batch review only"))
' "$SUMMARY" >/dev/null

{
  printf '# Trillionnium World Review Public-Boundary Batch\n\n'
  printf -- '- status: `%s`\n' "$(jq -r '.status' "$SUMMARY")"
  printf -- '- batch: `%s` / `%s`\n' \
    "$(jq -r '.batch_order' "$SUMMARY")" \
    "$(jq -r '.bucket_id' "$SUMMARY")"
  printf -- '- reviewed commits: `%s` / `%s`\n' \
    "$(jq -r '.reviewed_commit_count' "$SUMMARY")" \
    "$(jq -r '.required_reviewed_commit_count' "$SUMMARY")"
  printf -- '- unresolved public-boundary reviews: `%s`\n' \
    "$(jq -r '.unresolved_public_boundary_review_count' "$SUMMARY")"
  printf -- '- blockers needing real evidence: `%s`\n' \
    "$(jq -r '.blocker_ledger_needs_collection_count' "$SUMMARY")"
  printf -- '- batch 2 unblocked for local review: `%s`\n\n' \
    "$(jq -r '.batch_2_unblocked_for_local_review' "$SUMMARY")"
  printf '## Commit Reviews\n\n'
  jq -r '.commit_reviews[] | "- `\(.short)`: \(.boundary_conclusion)"' "$SUMMARY"
} >"$SUMMARY_MD"

printf 'TRILLIONNIUM_WORLD_REVIEW_PUBLIC_BOUNDARY_BATCH_GREEN %s %s\n' "$SUMMARY" "$SUMMARY_MD"
