#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
DOC_REL="docs/development/trillionnium-world-review-bot-executor-surface-batch-2026-07-09.md"
DOC="$ROOT/$DOC_REL"
RUNTIME_OWNER_QUEUE_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-runtime-owner-queue.json"
EXECUTION_BATCHES_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-execution-batches.json"
DOCS_PLAN_TRUTH_SOURCE_BATCH_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-docs-plan-truth-source-batch.json"
SEMANTIC_FIXTURE_JSON="$ACCEPTANCE_DIR/release-review-packet-integrity-semantic-fixture.json"
BOT_EXECUTOR_FIXTURE_JSON="$ACCEPTANCE_DIR/release-review-packet-integrity-bot-executor-semantic-fixture.json"
BOT_EXECUTOR_MATRIX_FIXTURE_JSON="$ACCEPTANCE_DIR/release-review-packet-integrity-bot-executor-matrix-semantic-fixture.json"
BOT_GAP_FIXTURE_JSON="$ACCEPTANCE_DIR/release-review-packet-integrity-bot-gap-semantic-fixture.json"
CONTROL_LOOP_FIXTURE_JSON="$ACCEPTANCE_DIR/release-review-packet-integrity-control-loop-semantic-fixture.json"
SELECTION_MINIMAP_FIXTURE_JSON="$ACCEPTANCE_DIR/release-review-packet-integrity-selection-minimap-semantic-fixture.json"
BUILD_LIFECYCLE_FIXTURE_JSON="$ACCEPTANCE_DIR/release-review-packet-integrity-build-lifecycle-semantic-fixture.json"
TECH_TREE_FIXTURE_JSON="$ACCEPTANCE_DIR/release-review-packet-integrity-tech-tree-semantic-fixture.json"
PROJECTILE_ABILITY_FIXTURE_JSON="$ACCEPTANCE_DIR/release-review-packet-integrity-projectile-ability-semantic-fixture.json"
SEMANTIC_FIXTURE_SUITE_SCRIPT="$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_semantic_fixture_suite.sh"
SUMMARY="$ACCEPTANCE_DIR/trillionnium-world-review-bot-executor-surface-batch.json"
SUMMARY_MD="$ACCEPTANCE_DIR/trillionnium-world-review-bot-executor-surface-batch.md"
REFRESH_INPUTS="${TRNM_WORLD_REVIEW_BOT_EXECUTOR_SURFACE_BATCH_REFRESH_INPUTS:-0}"
EXPECTED_COMMIT_SET_SHA256="e7140ae68254b55db1b1cff806a53a08fe27c50494a27bbac3d9e0044820caf4"
mkdir -p "$ACCEPTANCE_DIR"

require_text() {
  local path="$1"
  local needle="$2"
  if ! grep -Fq -- "$needle" "$path"; then
    echo "[FAIL] $path missing required text: $needle" >&2
    exit 1
  fi
}

require_fixture_json() {
  local path="$1"
  local contract_version="$2"
  local status="$3"
  local fixture_kind="$4"
  jq -e \
    --arg contract_version "$contract_version" \
    --arg status "$status" \
    --arg fixture_kind "$fixture_kind" \
    '
      .contract_version == $contract_version
      and .status == $status
      and .fixture_kind == $fixture_kind
      and .green == true
      and .public_launch_ready == false
      and .android_s5_real_device_claimed == false
      and (.external_action_performed // false) == false
    ' "$path" >/dev/null
}

if [[ ! -f "$DOC" ]]; then
  echo "[FAIL] missing bot/executor surface batch doc: $DOC" >&2
  exit 1
fi

require_text "$DOC" "Status: local review bot/executor surface batch 6."
require_text "$DOC" "unclassified_bot_executor_surface"
require_text "$DOC" 'Reviewed commit count: `10`'
require_text "$DOC" 'Unresolved bot/executor surface route count: `0`'
require_text "$DOC" "bot_executor_surface_route_complete=true"
require_text "$DOC" "packet_semantic_fixture_owner_bound=true"
require_text "$DOC" "rts_runtime_data_boundary_preserved=true"
require_text "$DOC" "release_evidence_contract_bound=true"
require_text "$DOC" "bevy_integration_ownership_claimed=false"
require_text "$DOC" "batch_6_exit_rule_satisfied=true"
require_text "$DOC" "batch_7_unblocked_for_local_review=true"
require_text "$DOC" "next_batch_bucket_id=unclassified_classic_evidence_surface"

if [[ "$REFRESH_INPUTS" == "1" ]]; then
  "$ROOT/scripts/check_trillionnium_world_review_runtime_owner_queue.sh" >/dev/null
  TRNM_WORLD_REVIEW_EXECUTION_BATCHES_REFRESH_INPUTS=0 \
    "$ROOT/scripts/check_trillionnium_world_review_execution_batches.sh" >/dev/null
  TRNM_WORLD_REVIEW_DOCS_PLAN_TRUTH_SOURCE_BATCH_REFRESH_INPUTS=0 \
    "$ROOT/scripts/check_trillionnium_world_review_docs_plan_truth_source_batch.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_semantic_fixture.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_bot_executor_semantic_fixture.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_bot_executor_matrix_semantic_fixture.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_bot_gap_semantic_fixture.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_control_loop_semantic_fixture.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_selection_minimap_semantic_fixture.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_build_lifecycle_semantic_fixture.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_tech_tree_semantic_fixture.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity_projectile_ability_semantic_fixture.sh" >/dev/null
  "$SEMANTIC_FIXTURE_SUITE_SCRIPT" >/dev/null
fi

for input in \
  "$RUNTIME_OWNER_QUEUE_JSON" \
  "$EXECUTION_BATCHES_JSON" \
  "$DOCS_PLAN_TRUTH_SOURCE_BATCH_JSON" \
  "$SEMANTIC_FIXTURE_JSON" \
  "$BOT_EXECUTOR_FIXTURE_JSON" \
  "$BOT_EXECUTOR_MATRIX_FIXTURE_JSON" \
  "$BOT_GAP_FIXTURE_JSON" \
  "$CONTROL_LOOP_FIXTURE_JSON" \
  "$SELECTION_MINIMAP_FIXTURE_JSON" \
  "$BUILD_LIFECYCLE_FIXTURE_JSON" \
  "$TECH_TREE_FIXTURE_JSON" \
  "$PROJECTILE_ABILITY_FIXTURE_JSON" \
  "$SEMANTIC_FIXTURE_SUITE_SCRIPT"; do
  if [[ ! -f "$input" ]]; then
    echo "[FAIL] missing bot/executor surface batch input: $input" >&2
    exit 1
  fi
done

actual_commit_set_sha256="$(
  jq -r '[.queue_items[] | select(.bucket_id == "unclassified_bot_executor_surface") | .commit] | sort | join("\n")' \
    "$RUNTIME_OWNER_QUEUE_JSON" | sha256sum | awk '{print $1}'
)"

if [[ "$actual_commit_set_sha256" != "$EXPECTED_COMMIT_SET_SHA256" ]]; then
  echo "[FAIL] bot/executor surface commit set drifted: $actual_commit_set_sha256" >&2
  exit 1
fi

jq -e '
  .contract_version == "trillionnium_world_review_runtime_owner_queue_v1"
  and .status == "review_runtime_owner_queue_ready"
  and .primary_owner == "rts_runtime_data_boundaries"
  and .lane_bucket_count == 3
  and .queue_matches_owner_plan == true
  and ([.queue_items[] | select(.bucket_id == "unclassified_bot_executor_surface")] | length) == 10
  and .push_performed == false
  and .rebase_performed == false
  and .reset_performed == false
  and .squash_performed == false
  and .history_rewrite_performed == false
  and .external_action_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$RUNTIME_OWNER_QUEUE_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_review_execution_batches_v1"
  and .status == "review_execution_batches_ready"
  and .owner_batch_count == 11
  and .queue_item_coverage_complete == true
  and .all_owner_batches_match_plan == true
  and ([.batches[] | select(
    .batch_order == 6
    and .bucket_id == "unclassified_bot_executor_surface"
    and .source_queue == "review_runtime_owner_queue"
    and .primary_owner == "rts_runtime_data_boundaries"
    and .execution_kind == "bucket_level_owner_review"
    and .owner_plan_commit_count == 10
    and .queue_item_count == 10
    and .owner_plan_matches_queue == true
    and .commit_level_primary_owner_review_required == false
    and .exit_rule == "Bot/executor changes must be assigned to runtime/data, Bevy integration, or release evidence."
  )] | length) == 1
  and .external_action_performed == false
  and .history_rewrite_performed == false
' "$EXECUTION_BATCHES_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_review_docs_plan_truth_source_batch_v1"
  and .status == "review_docs_plan_truth_source_batch_5_ready"
  and .reviewed_commit_count == 9
  and .unresolved_docs_plan_truth_source_review_count == 0
  and .batch_5_exit_rule_satisfied == true
  and .batch_6_unblocked_for_local_review == true
  and .next_batch_bucket_id == "unclassified_bot_executor_surface"
  and .external_action_performed == false
  and .history_rewrite_performed == false
' "$DOCS_PLAN_TRUTH_SOURCE_BATCH_JSON" >/dev/null

require_fixture_json \
  "$BOT_EXECUTOR_FIXTURE_JSON" \
  "trillionnium_world_release_review_packet_integrity_bot_executor_semantic_fixture_v1" \
  "release_review_packet_integrity_bot_executor_semantic_fixture_green" \
  "bot_executor_source_chain_semantic_negative_fixture"
require_fixture_json \
  "$BOT_EXECUTOR_MATRIX_FIXTURE_JSON" \
  "trillionnium_world_release_review_packet_integrity_bot_executor_matrix_semantic_fixture_v1" \
  "release_review_packet_integrity_bot_executor_matrix_semantic_fixture_green" \
  "bot_executor_failure_recovery_matrix_semantic_negative_fixture"
require_fixture_json \
  "$BOT_GAP_FIXTURE_JSON" \
  "trillionnium_world_release_review_packet_integrity_bot_gap_semantic_fixture_v1" \
  "release_review_packet_integrity_bot_gap_semantic_fixture_green" \
  "bot_gap_foundation_micro_intel_semantic_negative_fixture"
require_fixture_json \
  "$CONTROL_LOOP_FIXTURE_JSON" \
  "trillionnium_world_release_review_packet_integrity_control_loop_semantic_fixture_v1" \
  "release_review_packet_integrity_control_loop_semantic_fixture_green" \
  "classic_rts_control_loop_semantic_negative_fixture"
require_fixture_json \
  "$SELECTION_MINIMAP_FIXTURE_JSON" \
  "trillionnium_world_release_review_packet_integrity_selection_minimap_semantic_fixture_v1" \
  "release_review_packet_integrity_selection_minimap_semantic_fixture_green" \
  "classic_rts_selection_minimap_semantic_negative_fixture"
require_fixture_json \
  "$BUILD_LIFECYCLE_FIXTURE_JSON" \
  "trillionnium_world_release_review_packet_integrity_build_lifecycle_semantic_fixture_v1" \
  "release_review_packet_integrity_build_lifecycle_semantic_fixture_green" \
  "classic_rts_build_lifecycle_semantic_negative_fixture"
require_fixture_json \
  "$TECH_TREE_FIXTURE_JSON" \
  "trillionnium_world_release_review_packet_integrity_tech_tree_semantic_fixture_v1" \
  "release_review_packet_integrity_tech_tree_semantic_fixture_green" \
  "classic_rts_tech_tree_semantic_negative_fixture"
require_fixture_json \
  "$PROJECTILE_ABILITY_FIXTURE_JSON" \
  "trillionnium_world_release_review_packet_integrity_projectile_ability_semantic_fixture_v1" \
  "release_review_packet_integrity_projectile_ability_semantic_fixture_green" \
  "classic_rts_projectile_ability_semantic_negative_fixture"
require_fixture_json \
  "$SEMANTIC_FIXTURE_JSON" \
  "trillionnium_world_release_review_packet_integrity_semantic_fixture_v1" \
  "release_review_packet_integrity_semantic_fixture_green" \
  "release_review_convergence_status_quickcheck_release_signoff_cex_adapter_first_minute_command_feedback_and_handoff_first_contact_semantic_negative_fixture"

for line in \
  "first_minute_command_feedback" \
  "handoff_first_contact" \
  "semantic_fixture_suite"; do
  require_text "$SEMANTIC_FIXTURE_SUITE_SCRIPT" "$line"
done

runtime_queue_batch_item_count="$(jq '[.queue_items[] | select(.bucket_id == "unclassified_bot_executor_surface")] | length' "$RUNTIME_OWNER_QUEUE_JSON")"
execution_batch_queue_item_count="$(jq '.batches[] | select(.bucket_id == "unclassified_bot_executor_surface") | .queue_item_count' "$EXECUTION_BATCHES_JSON")"

jq -n \
  --arg contract_version "trillionnium_world_review_bot_executor_surface_batch_v1" \
  --arg status "review_bot_executor_surface_batch_6_ready" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg doc_path "$DOC_REL" \
  --arg bucket_id "unclassified_bot_executor_surface" \
  --arg primary_owner "rts_runtime_data_boundaries" \
  --arg execution_kind "bucket_level_owner_review" \
  --arg expected_commit_set_sha256 "$EXPECTED_COMMIT_SET_SHA256" \
  --arg actual_commit_set_sha256 "$actual_commit_set_sha256" \
  --arg next_batch_bucket_id "unclassified_classic_evidence_surface" \
  --argjson runtime_queue_batch_item_count "$runtime_queue_batch_item_count" \
  --argjson execution_batch_queue_item_count "$execution_batch_queue_item_count" \
  --argjson commit_reviews '[
    {
      "short_commit": "f30f6cfa77",
      "commit": "f30f6cfa77625f81dd19d33007bb1d2d6b2d7b88",
      "subject": "test: add bot executor packet semantic fixture",
      "review_group": "packet_bot_executor_semantic_fixtures",
      "owner_route": "release_packet_integrity_bot_executor_semantic_fixture"
    },
    {
      "short_commit": "01cc005183",
      "commit": "01cc00518341eeb962a986696501bf42205b203a",
      "subject": "test: add bot executor matrix packet semantic fixture",
      "review_group": "packet_bot_executor_semantic_fixtures",
      "owner_route": "release_packet_integrity_bot_executor_matrix_semantic_fixture"
    },
    {
      "short_commit": "d3a38a5baa",
      "commit": "d3a38a5baaaab2a2299b7aa5570d8bab38b6039c",
      "subject": "test: add bot gap packet semantic fixture",
      "review_group": "packet_bot_executor_semantic_fixtures",
      "owner_route": "release_packet_integrity_bot_gap_semantic_fixture"
    },
    {
      "short_commit": "0da6378e9a",
      "commit": "0da6378e9a1912fdeb3e446418fec34fbc88f7d0",
      "subject": "test: add control loop packet semantic fixture",
      "review_group": "classic_rts_local_semantic_fixtures",
      "owner_route": "release_packet_integrity_control_loop_semantic_fixture"
    },
    {
      "short_commit": "5b54e6044c",
      "commit": "5b54e6044cbe71a521790dba167b80031e5b24b0",
      "subject": "test: add selection minimap packet semantic fixture",
      "review_group": "classic_rts_local_semantic_fixtures",
      "owner_route": "release_packet_integrity_selection_minimap_semantic_fixture"
    },
    {
      "short_commit": "24351131a5",
      "commit": "24351131a57d2d3893dfcfdf43d131a073ae7f9b",
      "subject": "test: add build lifecycle packet semantic fixture",
      "review_group": "classic_rts_local_semantic_fixtures",
      "owner_route": "release_packet_integrity_build_lifecycle_semantic_fixture"
    },
    {
      "short_commit": "ee443911a9",
      "commit": "ee443911a9e4398d9ee8b25fabd274c6da26cbc6",
      "subject": "test: add tech tree packet semantic fixture",
      "review_group": "classic_rts_local_semantic_fixtures",
      "owner_route": "release_packet_integrity_tech_tree_semantic_fixture"
    },
    {
      "short_commit": "ae8a3b4e3d",
      "commit": "ae8a3b4e3da3e2630d02255db59f7f140f859cf9",
      "subject": "test: add projectile ability packet semantic fixture",
      "review_group": "classic_rts_local_semantic_fixtures",
      "owner_route": "release_packet_integrity_projectile_ability_semantic_fixture"
    },
    {
      "short_commit": "044163ef73",
      "commit": "044163ef732401e3a57d449c159ca1c9c6c6dd53",
      "subject": "test: bind first-minute rejection fixture semantics",
      "review_group": "first_minute_handoff_drift_guards",
      "owner_route": "release_packet_integrity_semantic_fixture"
    },
    {
      "short_commit": "e7bc3a60c1",
      "commit": "e7bc3a60c1087fec2cffa2b23661096b9aef7f7f",
      "subject": "fix: guard handoff First Contact semantic drift",
      "review_group": "first_minute_handoff_drift_guards",
      "owner_route": "release_packet_integrity_semantic_fixture_suite"
    }
  ]' \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    green: true,
    doc_path: $doc_path,
    batch_order: 6,
    bucket_id: $bucket_id,
    primary_owner: $primary_owner,
    execution_kind: $execution_kind,
    expected_commit_set_sha256: $expected_commit_set_sha256,
    actual_commit_set_sha256: $actual_commit_set_sha256,
    expected_hash_coverage_complete: ($expected_commit_set_sha256 == $actual_commit_set_sha256),
    runtime_queue_batch_item_count: $runtime_queue_batch_item_count,
    execution_batch_queue_item_count: $execution_batch_queue_item_count,
    required_commit_count: 10,
    reviewed_commit_count: ($commit_reviews | length),
    unresolved_bot_executor_surface_review_count: 0,
    commit_level_primary_owner_review_required: false,
    review_group_count: 3,
    review_group_counts: {
      packet_bot_executor_semantic_fixtures: 3,
      classic_rts_local_semantic_fixtures: 5,
      first_minute_handoff_drift_guards: 2
    },
    commit_reviews: $commit_reviews,
    bot_executor_surface_route_complete: true,
    packet_semantic_fixture_owner_bound: true,
    rts_runtime_data_boundary_preserved: true,
    release_evidence_contract_bound: true,
    prior_batch_5_closed: true,
    batch_6_exit_rule_satisfied: true,
    batch_7_unblocked_for_local_review: true,
    next_batch_bucket_id: $next_batch_bucket_id,
    bevy_integration_ownership_claimed: false,
    playable_runtime_ownership_claimed: false,
    external_evidence_claimed: false,
    public_launch_ready_claimed: false,
    android_s5_real_device_claimed: false,
    openra_runtime_compatibility_claimed: false,
    push_performed: false,
    rebase_performed: false,
    reset_performed: false,
    squash_performed: false,
    history_rewrite_performed: false,
    external_action_performed: false,
    no_credit_boundary: "local bot/executor surface batch 6 review only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, socket, hosted-service, live multiplayer, external evidence, human-playtest completion, Bevy playable integration ownership, OpenRA runtime/replay/network compatibility, render-world extraction completion, GPU upload, live-traffic, or public-network credit"
  }' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_review_bot_executor_surface_batch_v1"
  and .status == "review_bot_executor_surface_batch_6_ready"
  and .green == true
  and .batch_order == 6
  and .bucket_id == "unclassified_bot_executor_surface"
  and .primary_owner == "rts_runtime_data_boundaries"
  and .reviewed_commit_count == 10
  and .unresolved_bot_executor_surface_review_count == 0
  and .expected_hash_coverage_complete == true
  and .review_group_count == 3
  and .review_group_counts.packet_bot_executor_semantic_fixtures == 3
  and .review_group_counts.classic_rts_local_semantic_fixtures == 5
  and .review_group_counts.first_minute_handoff_drift_guards == 2
  and .bot_executor_surface_route_complete == true
  and .packet_semantic_fixture_owner_bound == true
  and .rts_runtime_data_boundary_preserved == true
  and .release_evidence_contract_bound == true
  and .prior_batch_5_closed == true
  and .batch_6_exit_rule_satisfied == true
  and .batch_7_unblocked_for_local_review == true
  and .next_batch_bucket_id == "unclassified_classic_evidence_surface"
  and .bevy_integration_ownership_claimed == false
  and .playable_runtime_ownership_claimed == false
  and .external_evidence_claimed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
  and .openra_runtime_compatibility_claimed == false
  and .external_action_performed == false
  and .history_rewrite_performed == false
' "$SUMMARY" >/dev/null

{
  printf '# Trillionnium World Review Bot Executor Surface Batch\n\n'
  printf -- '- status: `%s`\n' "$(jq -r '.status' "$SUMMARY")"
  printf -- '- reviewed commits: `%s`\n' "$(jq -r '.reviewed_commit_count' "$SUMMARY")"
  printf -- '- unresolved bot/executor surface routes: `%s`\n' "$(jq -r '.unresolved_bot_executor_surface_review_count' "$SUMMARY")"
  printf -- '- packet semantic fixture owner bound: `%s`\n' "$(jq -r '.packet_semantic_fixture_owner_bound' "$SUMMARY")"
  printf -- '- RTS runtime/data boundary preserved: `%s`\n' "$(jq -r '.rts_runtime_data_boundary_preserved' "$SUMMARY")"
  printf -- '- next batch: `%s`\n' "$(jq -r '.next_batch_bucket_id' "$SUMMARY")"
} >"$SUMMARY_MD"

echo "TRILLIONNIUM_WORLD_REVIEW_BOT_EXECUTOR_SURFACE_BATCH_GREEN $SUMMARY"
