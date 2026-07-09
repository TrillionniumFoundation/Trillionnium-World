#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
DOC_REL="docs/development/trillionnium-world-review-docs-plan-truth-source-batch-2026-07-09.md"
DOC="$ROOT/$DOC_REL"
RELEASE_OWNER_QUEUE_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-release-owner-queue.json"
EXECUTION_BATCHES_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-execution-batches.json"
GENERATED_COUNT_SURFACE_BATCH_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-generated-count-surface-batch.json"
TERM_EXCHANGE_DOC="$ROOT/docs/development/trillionnium-term-exchange-kernel-v1.md"
UNIFIED_DEV_DOC="$ROOT/docs/development/trillionnium-world-unified-development-doc-v1.md"
RTS_FUSION_PLAN_DOC="$ROOT/docs/architecture/rts-fusion-engine-plan-2026-06-12.md"
PACKET_INTEGRITY_SCRIPT="$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity.sh"
PACKET_INTEGRITY_CONTRACT_GUARD="$ROOT/scripts/v2/release_review_packet_integrity_script_contract_guard_test.sh"
CHECKPOINT_MANIFEST_SCRIPT="$ROOT/scripts/check_trillionnium_world_release_review_checkpoint_manifest.sh"
CHECKPOINT_MANIFEST_CONTRACT_GUARD="$ROOT/scripts/v2/release_review_checkpoint_manifest_script_contract_guard_test.sh"
SUMMARY="$ACCEPTANCE_DIR/trillionnium-world-review-docs-plan-truth-source-batch.json"
SUMMARY_MD="$ACCEPTANCE_DIR/trillionnium-world-review-docs-plan-truth-source-batch.md"
REFRESH_INPUTS="${TRNM_WORLD_REVIEW_DOCS_PLAN_TRUTH_SOURCE_BATCH_REFRESH_INPUTS:-1}"
EXPECTED_COMMIT_SET_SHA256="ff5e709af79d9771a37aa3f5836187603433b1e3492a716ed6c5972f4ba3b858"
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
  echo "[FAIL] missing docs/plan truth-source batch doc: $DOC" >&2
  exit 1
fi

require_text "$DOC" "Status: local review docs/plan truth-source batch 5."
require_text "$DOC" "unclassified_docs_plan_truth_source"
require_text "$DOC" 'Reviewed commit count: `9`'
require_text "$DOC" 'Unresolved docs/plan truth-source route count: `0`'
require_text "$DOC" "doc_truth_source_route_complete=true"
require_text "$DOC" "term_exchange_current_truth_bound=true"
require_text "$DOC" "rts_fusion_plan_reference_bound=true"
require_text "$DOC" "packet_checkpoint_guard_truth_bound=true"
require_text "$DOC" "batch_5_exit_rule_satisfied=true"
require_text "$DOC" "batch_6_unblocked_for_local_review=true"
require_text "$DOC" "next_batch_bucket_id=unclassified_bot_executor_surface"

if [[ "$REFRESH_INPUTS" != "0" ]]; then
  "$ROOT/scripts/check_trillionnium_world_review_release_owner_queue.sh" >/dev/null
  TRNM_WORLD_REVIEW_EXECUTION_BATCHES_REFRESH_INPUTS=0 \
    "$ROOT/scripts/check_trillionnium_world_review_execution_batches.sh" >/dev/null
  TRNM_WORLD_REVIEW_GENERATED_COUNT_SURFACE_BATCH_REFRESH_INPUTS=0 \
    "$ROOT/scripts/check_trillionnium_world_review_generated_count_surface_batch.sh" >/dev/null
fi

for input in \
  "$RELEASE_OWNER_QUEUE_JSON" \
  "$EXECUTION_BATCHES_JSON" \
  "$GENERATED_COUNT_SURFACE_BATCH_JSON" \
  "$TERM_EXCHANGE_DOC" \
  "$UNIFIED_DEV_DOC" \
  "$RTS_FUSION_PLAN_DOC" \
  "$PACKET_INTEGRITY_SCRIPT" \
  "$PACKET_INTEGRITY_CONTRACT_GUARD" \
  "$CHECKPOINT_MANIFEST_SCRIPT" \
  "$CHECKPOINT_MANIFEST_CONTRACT_GUARD"; do
  if [[ ! -f "$input" ]]; then
    echo "[FAIL] missing docs/plan truth-source batch input: $input" >&2
    exit 1
  fi
done

actual_commit_set_sha256="$(
  jq -r '[.queue_items[] | select(.bucket_id == "unclassified_docs_plan_truth_source") | .commit] | sort | join("\n")' \
    "$RELEASE_OWNER_QUEUE_JSON" | sha256sum | awk '{print $1}'
)"

if [[ "$actual_commit_set_sha256" != "$EXPECTED_COMMIT_SET_SHA256" ]]; then
  echo "[FAIL] docs/plan truth-source commit set drifted: $actual_commit_set_sha256" >&2
  exit 1
fi

jq -e '
  .contract_version == "trillionnium_world_review_release_owner_queue_v1"
  and .status == "review_release_owner_queue_ready"
  and .primary_owner == "release_truth_and_public_boundary"
  and .lane_bucket_count == 4
  and .queue_matches_owner_plan == true
  and ([.queue_items[] | select(.bucket_id == "unclassified_docs_plan_truth_source")] | length) == 9
  and .push_performed == false
  and .rebase_performed == false
  and .reset_performed == false
  and .squash_performed == false
  and .history_rewrite_performed == false
  and .external_action_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$RELEASE_OWNER_QUEUE_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_review_execution_batches_v1"
  and .status == "review_execution_batches_ready"
  and .owner_batch_count == 11
  and .queue_item_coverage_complete == true
  and .all_owner_batches_match_plan == true
  and ([.batches[] | select(
    .batch_order == 5
    and .bucket_id == "unclassified_docs_plan_truth_source"
    and .source_queue == "review_release_owner_queue"
    and .primary_owner == "release_truth_and_public_boundary"
    and .execution_kind == "bucket_level_owner_review"
    and .owner_plan_commit_count == 9
    and .queue_item_count == 9
    and .owner_plan_matches_queue == true
    and .commit_level_primary_owner_review_required == false
    and .exit_rule == "Each doc must be confirmed as current truth or routed to archive/reference-only."
  )] | length) == 1
  and .external_action_performed == false
  and .history_rewrite_performed == false
' "$EXECUTION_BATCHES_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_review_generated_count_surface_batch_v1"
  and .status == "review_generated_count_surface_batch_4_ready"
  and .reviewed_commit_count == 14
  and .unresolved_generated_count_surface_review_count == 0
  and .batch_4_exit_rule_satisfied == true
  and .batch_5_unblocked_for_local_review == true
  and .next_batch_bucket_id == "unclassified_docs_plan_truth_source"
  and .external_action_performed == false
  and .history_rewrite_performed == false
' "$GENERATED_COUNT_SURFACE_BATCH_JSON" >/dev/null

for line in \
  "trillionnium_term_exchange_kernel_v1" \
  "trillionnium_term_exchange_backend_adapter_v1" \
  "TermExchangeReceiptState" \
  "normalized_sql_shadow_status=receipt_tables_shadowed" \
  "trillionnium_term_exchange_receipt_projection_v1"; do
  require_text "$TERM_EXCHANGE_DOC" "$line"
done

for line in \
  "Term Exchange Kernel" \
  "Term Exchange backend adapter" \
  "Typed receipt state" \
  "Normalized receipt persistence/projection"; do
  require_text "$UNIFIED_DEV_DOC" "$line"
done

for line in \
  "RTS Fusion Engine Plan" \
  "The first cut has landed" \
  "Current Status - 2026-06-29" \
  "Renderer Ownership / Codegen Plan - 2026-06-26" \
  "Next Local Slices" \
  "public_launch_ready=false"; do
  require_text "$RTS_FUSION_PLAN_DOC" "$line"
done

for line in \
  "first_contact_motion_readability_contract" \
  "release_review_checkpoint_manifest_semantics" \
  "bot_planner_action_executor_count_semantics" \
  "checks_total: \$check_count"; do
  require_text "$PACKET_INTEGRITY_SCRIPT" "$line"
done

for line in \
  "first_contact_motion_readability_guard" \
  "release_review_checkpoint_manifest_semantics" \
  "bot_planner_action_executor_count_semantics"; do
  require_text "$PACKET_INTEGRITY_CONTRACT_GUARD" "$line"
done

for line in \
  "trillionnium_world_release_review_checkpoint_manifest_v1" \
  "checkpoint_manifest_only_not_public_launch_evidence" \
  "docs_planning"; do
  require_text "$CHECKPOINT_MANIFEST_SCRIPT" "$line"
done

for line in \
  "trillionnium_world_release_review_checkpoint_manifest_v1" \
  "checkpoint_manifest_only_not_public_launch_evidence"; do
  require_text "$CHECKPOINT_MANIFEST_CONTRACT_GUARD" "$line"
done

release_queue_batch_item_count="$(jq '[.queue_items[] | select(.bucket_id == "unclassified_docs_plan_truth_source")] | length' "$RELEASE_OWNER_QUEUE_JSON")"
execution_batch_queue_item_count="$(jq '.batches[] | select(.bucket_id == "unclassified_docs_plan_truth_source") | .queue_item_count' "$EXECUTION_BATCHES_JSON")"

jq -n \
  --arg contract_version "trillionnium_world_review_docs_plan_truth_source_batch_v1" \
  --arg status "review_docs_plan_truth_source_batch_5_ready" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg doc_path "$DOC_REL" \
  --arg bucket_id "unclassified_docs_plan_truth_source" \
  --arg primary_owner "release_truth_and_public_boundary" \
  --arg execution_kind "bucket_level_owner_review" \
  --arg expected_commit_set_sha256 "$EXPECTED_COMMIT_SET_SHA256" \
  --arg actual_commit_set_sha256 "$actual_commit_set_sha256" \
  --arg next_batch_bucket_id "unclassified_bot_executor_surface" \
  --argjson release_queue_batch_item_count "$release_queue_batch_item_count" \
  --argjson execution_batch_queue_item_count "$execution_batch_queue_item_count" \
  --argjson commit_reviews '[
    {
      "short_commit": "769379e99c",
      "commit": "769379e99c18cad6a67541e03da2fd79705ebb97",
      "subject": "docs: document term exchange backend adapter",
      "review_group": "term_exchange_docs_current_truth",
      "truth_source_route": "current_protocol_doc",
      "owning_doc": "docs/development/trillionnium-term-exchange-kernel-v1.md",
      "route_conclusion": "backend adapter contract remains current migration truth"
    },
    {
      "short_commit": "8172591e3a",
      "commit": "8172591e3a9502b735f4a452d40e905f254c28a9",
      "subject": "docs: document term exchange receipt state",
      "review_group": "term_exchange_docs_current_truth",
      "truth_source_route": "current_protocol_doc",
      "owning_doc": "docs/development/trillionnium-term-exchange-kernel-v1.md",
      "route_conclusion": "typed receipt state and normalized receipt projection remain current migration truth"
    },
    {
      "short_commit": "5a34e94254",
      "commit": "5a34e94254fe06c4f9cb4c6b32aa4fb390620763",
      "subject": "docs: refresh RTS fusion future plan",
      "review_group": "rts_fusion_architecture_plan",
      "truth_source_route": "current_architecture_reference_with_artifact_owned_counts",
      "owning_doc": "docs/architecture/rts-fusion-engine-plan-2026-06-12.md",
      "route_conclusion": "future plan remains architecture/reference truth while latest exact counts are artifact-owned"
    },
    {
      "short_commit": "9ab03c253",
      "commit": "9ab03c253c23e1772d3afe7f3bd3ba081e6e4bc0",
      "subject": "docs: refresh RTS fusion next slices",
      "review_group": "rts_fusion_architecture_plan",
      "truth_source_route": "current_architecture_reference_with_artifact_owned_counts",
      "owning_doc": "docs/architecture/rts-fusion-engine-plan-2026-06-12.md",
      "route_conclusion": "next slices remain planning/reference truth and do not override generated review artifacts"
    },
    {
      "short_commit": "991af6e0c",
      "commit": "991af6e0c4921e85036fee41d55c2ec69573c775",
      "subject": "fix: wire First Contact motion packet guard",
      "review_group": "packet_checkpoint_guard_truth_source",
      "truth_source_route": "current_packet_integrity_contract",
      "owning_checker": "scripts/check_trillionnium_world_release_review_packet_integrity.sh",
      "route_conclusion": "First Contact motion guard is packet-integrity contract truth, not public evidence"
    },
    {
      "short_commit": "bb794ecca",
      "commit": "bb794ecca8a7497143c2623b4e85d9d04df5f87e",
      "subject": "docs: record RTS renderer ownership plan",
      "review_group": "rts_fusion_architecture_plan",
      "truth_source_route": "current_architecture_reference_with_artifact_owned_counts",
      "owning_doc": "docs/architecture/rts-fusion-engine-plan-2026-06-12.md",
      "route_conclusion": "renderer ownership plan remains current boundary guidance"
    },
    {
      "short_commit": "63047b368",
      "commit": "63047b3683cd2e854f57a9e32289ad8e2ab5792a",
      "subject": "fix: bind checkpoint manifest summary counts",
      "review_group": "packet_checkpoint_guard_truth_source",
      "truth_source_route": "current_checkpoint_manifest_contract",
      "owning_checker": "scripts/check_trillionnium_world_release_review_checkpoint_manifest.sh",
      "route_conclusion": "checkpoint manifest summary counts are local review grouping metadata, not release evidence"
    },
    {
      "short_commit": "cc67934a06",
      "commit": "cc67934a06e02768c0e07b957b28ac37f41e7870",
      "subject": "fix: expose classic bot executor counts",
      "review_group": "packet_checkpoint_guard_truth_source",
      "truth_source_route": "current_packet_integrity_contract",
      "owning_checker": "scripts/check_trillionnium_world_release_review_packet_integrity.sh",
      "route_conclusion": "classic bot executor count surfaces are current checker/artifact contract fields"
    },
    {
      "short_commit": "cfaf1aad38",
      "commit": "cfaf1aad389828418f835e1a00f30b40601b2416",
      "subject": "docs: refresh RTS fusion execution plan",
      "review_group": "rts_fusion_architecture_plan",
      "truth_source_route": "current_architecture_reference_with_artifact_owned_counts",
      "owning_doc": "docs/architecture/rts-fusion-engine-plan-2026-06-12.md",
      "route_conclusion": "execution plan remains architecture/reference truth while generated next-plan owns latest local queue state"
    }
  ]' \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_review_docs_plan_truth_source_batch",
    green: true,
    doc_path: $doc_path,
    batch_order: 5,
    bucket_id: $bucket_id,
    primary_owner: $primary_owner,
    execution_kind: $execution_kind,
    release_queue_batch_item_count: $release_queue_batch_item_count,
    execution_batch_queue_item_count: $execution_batch_queue_item_count,
    required_reviewed_commit_count: 9,
    reviewed_commit_count: 9,
    expected_commit_set_sha256: $expected_commit_set_sha256,
    actual_commit_set_sha256: $actual_commit_set_sha256,
    expected_hash_coverage_complete: ($actual_commit_set_sha256 == $expected_commit_set_sha256),
    commit_level_primary_owner_review_required: false,
    doc_truth_source_route_complete: true,
    term_exchange_current_truth_bound: true,
    rts_fusion_plan_reference_bound: true,
    packet_checkpoint_guard_truth_bound: true,
    unresolved_docs_plan_truth_source_review_count: 0,
    review_group_count: 3,
    review_group_counts: {
      term_exchange_docs_current_truth: 2,
      rts_fusion_architecture_plan: 4,
      packet_checkpoint_guard_truth_source: 3
    },
    archive_reference_only_route_count: 0,
    prior_batch_4_closed: true,
    batch_5_exit_rule_satisfied: true,
    batch_6_unblocked_for_local_review: true,
    next_batch_bucket_id: $next_batch_bucket_id,
    commit_reviews: $commit_reviews,
    push_performed: false,
    rebase_performed: false,
    reset_performed: false,
    squash_performed: false,
    history_rewrite_performed: false,
    external_action_performed: false,
    upload_performed: false,
    publish_performed: false,
    public_launch_ready_claimed: false,
    android_s5_real_device_claimed: false,
    beta_claimed: false,
    commercial_claimed: false,
    production_ready_ui_claimed: false,
    openra_runtime_compatibility_claimed: false,
    render_world_extraction_complete_claimed: false,
    gpu_upload_claimed: false,
    live_traffic_performed: false,
    public_network_credit_claimed: false,
    no_credit_boundary: "local docs/plan truth-source batch 5 review only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, socket, hosted-service, live multiplayer, OpenRA runtime/replay/network compatibility, render-world extraction completion, GPU upload, live-traffic, public-network, external-evidence, or human-playtest completion credit"
  }' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_review_docs_plan_truth_source_batch_v1"
  and .status == "review_docs_plan_truth_source_batch_5_ready"
  and .batch_order == 5
  and .bucket_id == "unclassified_docs_plan_truth_source"
  and .release_queue_batch_item_count == 9
  and .execution_batch_queue_item_count == 9
  and .reviewed_commit_count == 9
  and .unresolved_docs_plan_truth_source_review_count == 0
  and .expected_hash_coverage_complete == true
  and .commit_level_primary_owner_review_required == false
  and .doc_truth_source_route_complete == true
  and .term_exchange_current_truth_bound == true
  and .rts_fusion_plan_reference_bound == true
  and .packet_checkpoint_guard_truth_bound == true
  and .review_group_count == 3
  and .review_group_counts.term_exchange_docs_current_truth == 2
  and .review_group_counts.rts_fusion_architecture_plan == 4
  and .review_group_counts.packet_checkpoint_guard_truth_source == 3
  and .archive_reference_only_route_count == 0
  and .prior_batch_4_closed == true
  and .batch_5_exit_rule_satisfied == true
  and .batch_6_unblocked_for_local_review == true
  and .next_batch_bucket_id == "unclassified_bot_executor_surface"
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
  and .external_action_performed == false
  and .history_rewrite_performed == false
' "$SUMMARY" >/dev/null

{
  printf '# Trillionnium World Review Docs Plan Truth Source Batch\n\n'
  printf -- '- status: `%s`\n' "review_docs_plan_truth_source_batch_5_ready"
  printf -- '- bucket: `%s`\n' "unclassified_docs_plan_truth_source"
  printf -- '- reviewed docs/plan truth-source commits: `9`\n'
  printf -- '- unresolved docs/plan truth-source reviews: `0`\n'
  printf -- '- doc truth-source route complete: `true`\n'
  printf -- '- Term Exchange current truth bound: `true`\n'
  printf -- '- RTS fusion plan reference bound: `true`\n'
  printf -- '- packet/checkpoint guard truth bound: `true`\n'
  printf -- '- batch 5 exit rule satisfied: `true`\n'
  printf -- '- batch 6 unblocked for local review: `true`\n'
  printf -- '- next batch: `%s`\n' "unclassified_bot_executor_surface"
  printf -- '- public launch ready claimed: `false`\n'
  printf -- '- Android S5 real-device claimed: `false`\n'
} >"$SUMMARY_MD"

printf 'TRILLIONNIUM_WORLD_REVIEW_DOCS_PLAN_TRUTH_SOURCE_BATCH_GREEN %s\n' "$SUMMARY"
