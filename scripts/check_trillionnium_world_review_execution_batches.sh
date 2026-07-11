#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
DOC_REL="docs/archive/world-review-2026-07/trillionnium-world-review-execution-batches-2026-07-08.md"
DOC="$ROOT/$DOC_REL"
OWNER_PLAN_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-primary-owner-plan.json"
RELEASE_QUEUE_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-release-owner-queue.json"
RUNTIME_QUEUE_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-runtime-owner-queue.json"
RESIDUAL_QUEUE_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-residual-queue.json"
SUMMARY="$ACCEPTANCE_DIR/trillionnium-world-review-execution-batches.json"
SUMMARY_MD="$ACCEPTANCE_DIR/trillionnium-world-review-execution-batches.md"
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
  echo "[FAIL] missing review execution batches doc: $DOC" >&2
  exit 1
fi

require_text "$DOC" "Status: local review execution batches."
require_text "$DOC" "release, runtime, and residual owner queues"
require_text "$DOC" "It does not stage, commit, push, rebase, reset, squash"
require_text "$DOC" "Do not convert these local batches into public-launch"
require_text "$DOC" '| 1 | `multi_public_boundary_overlap` |'
require_text "$DOC" '| 3 | `multi_native_bevy_rts_boundary_overlap` |'
require_text "$DOC" '| 11 | `multi_manual_overlap` |'

if [[ "${TRNM_WORLD_REVIEW_EXECUTION_BATCHES_REFRESH_INPUTS:-1}" != "0" ]]; then
  "$ROOT/scripts/check_trillionnium_world_review_primary_owner_plan.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_review_release_owner_queue.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_review_runtime_owner_queue.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_review_residual_queue.sh" >/dev/null
fi

for input in "$OWNER_PLAN_JSON" "$RELEASE_QUEUE_JSON" "$RUNTIME_QUEUE_JSON" "$RESIDUAL_QUEUE_JSON"; do
  if [[ ! -f "$input" ]]; then
    echo "[FAIL] missing review execution batches input: $input" >&2
    exit 1
  fi
done

jq -e '
  .contract_version == "trillionnium_world_review_primary_owner_plan_v1"
  and .status == "review_primary_owner_plan_ready"
  and .owner_bucket_count == 11
  and .bucket_primary_owner_assignment_complete == true
  and .review_order_complete == true
  and .push_performed == false
  and .rebase_performed == false
  and .reset_performed == false
  and .squash_performed == false
  and .history_rewrite_performed == false
  and .external_action_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$OWNER_PLAN_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_review_release_owner_queue_v1"
  and .status == "review_release_owner_queue_ready"
  and .queue_matches_owner_plan == true
  and .bucket_coverage_complete == true
  and .external_action_performed == false
  and .history_rewrite_performed == false
' "$RELEASE_QUEUE_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_review_runtime_owner_queue_v1"
  and .status == "review_runtime_owner_queue_ready"
  and .queue_matches_owner_plan == true
  and .bucket_coverage_complete == true
  and .external_action_performed == false
  and .history_rewrite_performed == false
' "$RUNTIME_QUEUE_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_review_residual_queue_v1"
  and .status == "review_residual_queue_ready"
  and .queue_matches_owner_plan == true
  and .all_owner_queue_coverage_complete == true
  and .external_action_performed == false
  and .history_rewrite_performed == false
' "$RESIDUAL_QUEUE_JSON" >/dev/null

jq -n \
  --arg contract_version "trillionnium_world_review_execution_batches_v1" \
  --arg status "review_execution_batches_ready" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg doc_path "$DOC_REL" \
  --slurpfile owner "$OWNER_PLAN_JSON" \
  --slurpfile release "$RELEASE_QUEUE_JSON" \
  --slurpfile runtime "$RUNTIME_QUEUE_JSON" \
  --slurpfile residual "$RESIDUAL_QUEUE_JSON" \
  '
  def all_items:
    (($release[0].queue_items // [])
      + ($runtime[0].queue_items // [])
      + ($residual[0].queue_items // []));
  def source_queue($bucket):
    if (($release[0].lane_bucket_ids // []) | index($bucket)) then
      "review_release_owner_queue"
    elif (($runtime[0].lane_bucket_ids // []) | index($bucket)) then
      "review_runtime_owner_queue"
    elif (($residual[0].remaining_bucket_ids // []) | index($bucket)) then
      "review_residual_queue"
    else
      "unmapped"
    end;
  def source_artifact($bucket):
    if source_queue($bucket) == "review_release_owner_queue" then
      "acceptance/S6_public_launch/latest/trillionnium-world-review-release-owner-queue.json"
    elif source_queue($bucket) == "review_runtime_owner_queue" then
      "acceptance/S6_public_launch/latest/trillionnium-world-review-runtime-owner-queue.json"
    elif source_queue($bucket) == "review_residual_queue" then
      "acceptance/S6_public_launch/latest/trillionnium-world-review-residual-queue.json"
    else
      "unmapped"
    end;
  def execution_kind($row):
    if $row.primary_owner == "manual_triage_required" then
      "manual_resolution"
    elif ($row.commit_count // 0) == 0 then
      "reserved_zero_count_lane"
    elif ($row.commit_level_primary_owner_review_required // false) == true then
      "commit_level_owner_review"
    else
      "bucket_level_owner_review"
    end;
  def batch_items($bucket): [ all_items[] | select(.bucket_id == $bucket) ];

  ($owner[0].owner_rows | sort_by(.review_order) | map(
    . as $row
    | (batch_items($row.bucket_id)) as $items
    | {
        batch_order: $row.review_order,
        bucket_id: $row.bucket_id,
        source_queue: source_queue($row.bucket_id),
        source_artifact: source_artifact($row.bucket_id),
        primary_owner: $row.primary_owner,
        execution_kind: execution_kind($row),
        owner_plan_commit_count: ($row.commit_count // 0),
        queue_item_count: ($items | length),
        owner_plan_matches_queue: (($items | length) == ($row.commit_count // 0)),
        commit_level_primary_owner_review_required: ($row.commit_level_primary_owner_review_required // false),
        exit_rule: ($row.exit_rule // "Reviewer must preserve owner boundaries before moving to the next batch."),
        reviewer_next_action: ($row.source_next_action // "Review this batch before external push or history planning."),
        sample_commits: [
          $items[0:3][]
          | {
              short: (.short // (.commit[0:10])),
              commit: .commit,
              subject: .subject,
              changed_path_count: (.changed_path_count // 0),
              matched_slices: (.matched_slices // []),
              source_type: (.source_type // "unknown")
            }
        ],
        no_credit_boundary: "local review execution batch only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, multi-node, live-traffic, or public-network credit"
      }
  )) as $batches
  | {
      contract_version: $contract_version,
      status: $status,
      generated_at: $generated_at,
      green: true,
      doc_path: $doc_path,
      source_owner_plan_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-primary-owner-plan.json",
      source_release_queue_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-release-owner-queue.json",
      source_runtime_queue_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-runtime-owner-queue.json",
      source_residual_queue_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-residual-queue.json",
      owner_plan_total_commit_count: ($owner[0].source_triage_queue_item_count // 0),
      release_queue_item_count: ($release[0].release_queue_item_count // 0),
      runtime_queue_item_count: ($runtime[0].runtime_queue_item_count // 0),
      residual_queue_item_count: ($residual[0].residual_queue_item_count // 0),
      total_queue_item_count: ($batches | map(.queue_item_count) | add),
      owner_batch_count: ($batches | length),
      nonempty_batch_count: ($batches | map(select(.queue_item_count > 0)) | length),
      reserved_zero_count_batch_count: ($batches | map(select(.queue_item_count == 0)) | length),
      commit_level_owner_review_batch_count: ($batches | map(select(.commit_level_primary_owner_review_required == true)) | length),
      manual_resolution_batch_count: ($batches | map(select(.execution_kind == "manual_resolution")) | length),
      all_owner_batches_match_plan: ($batches | all(.owner_plan_matches_queue == true)),
      queue_item_coverage_complete: (($batches | map(.queue_item_count) | add) == ($owner[0].source_triage_queue_item_count // 0)),
      first_batch_bucket_id: ($batches[0].bucket_id),
      final_batch_bucket_id: ($batches[-1].bucket_id),
      batches: $batches,
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
      public_network_live_exposure_claimed: false,
      no_credit_boundary: "local review execution batches only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, multi-node, live-traffic, or public-network credit",
      reviewer_next_action: "start with batch 1 public-boundary overlap, then proceed in numeric batch order; do not skip to runtime/product details before release no-credit boundaries are reviewed"
    }
  ' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_review_execution_batches_v1"
  and .status == "review_execution_batches_ready"
  and .green == true
  and .owner_batch_count == 11
  and .nonempty_batch_count >= 9
  and .reserved_zero_count_batch_count >= 1
  and .commit_level_owner_review_batch_count >= 1
  and .manual_resolution_batch_count == 2
  and .release_queue_item_count >= 1
  and .runtime_queue_item_count >= 1
  and .residual_queue_item_count >= 1
  and .total_queue_item_count == .owner_plan_total_commit_count
  and .all_owner_batches_match_plan == true
  and .queue_item_coverage_complete == true
  and .first_batch_bucket_id == "multi_public_boundary_overlap"
  and .final_batch_bucket_id == "multi_manual_overlap"
  and (.batches | length) == .owner_batch_count
  and (.batches | all(.source_queue != "unmapped"))
  and (.batches | all(.owner_plan_matches_queue == true))
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
  and .public_network_live_exposure_claimed == false
  and (.no_credit_boundary | contains("local review execution batches only"))
' "$SUMMARY" >/dev/null

{
  printf '# Trillionnium World Review Execution Batches\n\n'
  printf -- '- status: `%s`\n' "$(jq -r '.status' "$SUMMARY")"
  printf -- '- owner batches: `%s`\n' "$(jq -r '.owner_batch_count' "$SUMMARY")"
  printf -- '- nonempty / reserved zero-count: `%s` / `%s`\n' \
    "$(jq -r '.nonempty_batch_count' "$SUMMARY")" \
    "$(jq -r '.reserved_zero_count_batch_count' "$SUMMARY")"
  printf -- '- release/runtime/residual queue items: `%s` / `%s` / `%s`\n' \
    "$(jq -r '.release_queue_item_count' "$SUMMARY")" \
    "$(jq -r '.runtime_queue_item_count' "$SUMMARY")" \
    "$(jq -r '.residual_queue_item_count' "$SUMMARY")"
  printf -- '- total queue items: `%s`\n' "$(jq -r '.total_queue_item_count' "$SUMMARY")"
  printf -- '- queue coverage complete: `%s`\n\n' "$(jq -r '.queue_item_coverage_complete' "$SUMMARY")"
  printf '## Batches\n\n'
  jq -r '.batches[] | "- \(.batch_order). `\(.bucket_id)` / `\(.source_queue)` / `\(.primary_owner)`: \(.queue_item_count) items"' "$SUMMARY"
} >"$SUMMARY_MD"

printf 'TRILLIONNIUM_WORLD_REVIEW_EXECUTION_BATCHES_GREEN %s %s\n' "$SUMMARY" "$SUMMARY_MD"
