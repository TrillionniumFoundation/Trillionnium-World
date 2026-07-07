#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
DOC_REL="docs/development/trillionnium-world-review-primary-owner-plan-2026-07-07.md"
DOC="$ROOT/$DOC_REL"
TRIAGE_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-triage-queue.json"
SUMMARY="$ACCEPTANCE_DIR/trillionnium-world-review-primary-owner-plan.json"
SUMMARY_MD="$ACCEPTANCE_DIR/trillionnium-world-review-primary-owner-plan.md"
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
  echo "[FAIL] missing review primary-owner plan doc: $DOC" >&2
  exit 1
fi

require_text "$DOC" "Status: local review primary-owner plan."
require_text "$DOC" "Bucket primary owners are review routing defaults"
require_text "$DOC" "Multi-slice and manual buckets still need commit-level reviewer judgment"
require_text "$DOC" "Do not convert this local owner plan into public-launch"
require_text "$DOC" '| `multi_public_boundary_overlap` |'
require_text "$DOC" '| `multi_native_bevy_rts_boundary_overlap` |'
require_text "$DOC" '| `unclassified_generated_count_surface` |'
require_text "$DOC" '| `multi_manual_overlap` |'

"$ROOT/scripts/check_trillionnium_world_review_triage_queue.sh" >/dev/null
jq -e '
  .contract_version == "trillionnium_world_review_triage_queue_v1"
  and .status == "review_triage_queue_ready"
  and .triage_bucket_count == 11
  and .unclassified_bucketed_count == .unclassified_commit_count
  and .multi_slice_bucketed_count == .multi_slice_commit_count
  and .manual_review_required == true
  and .primary_owner_assignment_required == true
  and .push_performed == false
  and .rebase_performed == false
  and .reset_performed == false
  and .squash_performed == false
  and .history_rewrite_performed == false
  and .external_action_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$TRIAGE_JSON" >/dev/null

OWNER_DEFS="$(
  jq -n '[
    {
      bucket_id: "multi_public_boundary_overlap",
      primary_owner: "release_truth_and_public_boundary",
      review_order: 1,
      exit_rule: "No-credit and public/S5 boundaries must be reviewed before product/runtime details."
    },
    {
      bucket_id: "multi_release_native_handoff_overlap",
      primary_owner: "release_truth_and_public_boundary",
      review_order: 2,
      exit_rule: "Release truth and handoff language must be resolved before Bevy client review."
    },
    {
      bucket_id: "multi_native_bevy_rts_boundary_overlap",
      primary_owner: "rts_runtime_data_boundaries",
      review_order: 3,
      exit_rule: "Renderer-neutral RTS contracts must be reviewed before Bevy draw/runtime integration."
    },
    {
      bucket_id: "unclassified_generated_count_surface",
      primary_owner: "release_truth_and_public_boundary",
      review_order: 4,
      exit_rule: "Each count exposure must have an owning artifact/checker."
    },
    {
      bucket_id: "unclassified_docs_plan_truth_source",
      primary_owner: "release_truth_and_public_boundary",
      review_order: 5,
      exit_rule: "Each doc must be confirmed as current truth or routed to archive/reference-only."
    },
    {
      bucket_id: "unclassified_bot_executor_surface",
      primary_owner: "rts_runtime_data_boundaries",
      review_order: 6,
      exit_rule: "Bot/executor changes must be assigned to runtime/data, Bevy integration, or release evidence."
    },
    {
      bucket_id: "unclassified_classic_evidence_surface",
      primary_owner: "native_bevy_playable_client",
      review_order: 7,
      exit_rule: "Classic evidence surfaces must be routed to playable-client, renderer, or release-truth review."
    },
    {
      bucket_id: "unclassified_map_or_modeling_surface",
      primary_owner: "rts_runtime_data_boundaries",
      review_order: 8,
      exit_rule: "Map/modeling changes must prove no live-ingestion or public map-pack credit is implied."
    },
    {
      bucket_id: "multi_first_contact_readability_renderer_overlap",
      primary_owner: "first_contact_product_readability",
      review_order: 9,
      exit_rule: "Human-playtest/product readability owns the first pass before renderer micro-cue ownership."
    },
    {
      bucket_id: "unclassified_manual_other",
      primary_owner: "manual_triage_required",
      review_order: 10,
      exit_rule: "Read each commit and assign a primary reviewer slice manually."
    },
    {
      bucket_id: "multi_manual_overlap",
      primary_owner: "manual_triage_required",
      review_order: 11,
      exit_rule: "Read each overlap and decide primary owner or later split strategy manually."
    }
  ]'
)"

jq -n \
  --arg contract_version "trillionnium_world_review_primary_owner_plan_v1" \
  --arg status "review_primary_owner_plan_ready" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg doc_path "$DOC_REL" \
  --slurpfile triage "$TRIAGE_JSON" \
  --argjson owner_defs "$OWNER_DEFS" \
  '($triage[0]) as $t
  | [
      $owner_defs[] as $o
      | ($t.triage_buckets[] | select(.id == $o.bucket_id)) as $b
      | $o + {
          source_type: $b.type,
          source_severity: $b.severity,
          commit_count: $b.commit_count,
          source_next_action: $b.next_action,
          bucket_primary_owner_assigned: true,
          commit_level_primary_owner_review_required: (
            $b.severity == "primary_owner_required"
            or $o.primary_owner == "manual_triage_required"
          )
        }
    ] as $owner_rows
  | {
      contract_version: $contract_version,
      status: $status,
      generated_at: $generated_at,
      green: true,
      doc_path: $doc_path,
      source_triage_queue_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-triage-queue.json",
      source_triage_queue_status: $t.status,
      source_triage_queue_item_count: $t.triage_queue_item_count,
      source_triage_bucket_count: $t.triage_bucket_count,
      source_unclassified_commit_count: $t.unclassified_commit_count,
      source_multi_slice_commit_count: $t.multi_slice_commit_count,
      owner_bucket_count: ($owner_rows | length),
      bucket_primary_owner_assigned_count: ([ $owner_rows[] | select(.bucket_primary_owner_assigned == true) ] | length),
      bucket_primary_owner_assignment_complete: (([ $owner_rows[] | select(.bucket_primary_owner_assigned == true) ] | length) == $t.triage_bucket_count),
      commit_level_primary_owner_review_required: true,
      commit_level_primary_owner_review_required_count: ([ $owner_rows[] | select(.commit_level_primary_owner_review_required == true) | .commit_count ] | add),
      zero_count_bucket_count: ([ $owner_rows[] | select(.commit_count == 0) ] | length),
      primary_owner_count: ([ $owner_rows[].primary_owner ] | unique | length),
      owner_rows: ($owner_rows | sort_by(.review_order)),
      review_order_complete: (([ $owner_rows[].review_order ] | sort) == [1,2,3,4,5,6,7,8,9,10,11]),
      local_backlog_risk_active: true,
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
      beta_cohort_evidence_claimed: false,
      production_ready_ui_claimed: false,
      commercial_launch_evidence_claimed: false,
      public_network_live_exposure_claimed: false,
      no_credit_boundary: "local review primary-owner plan only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, multi-node, live-traffic, or public-network credit",
      reviewer_next_action: "walk owner_rows in review_order and do commit-level judgment for primary_owner_required or manual_triage_required buckets before any external push or history operation"
    }' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_review_primary_owner_plan_v1"
  and .status == "review_primary_owner_plan_ready"
  and .green == true
  and .source_triage_queue_status == "review_triage_queue_ready"
  and .source_triage_queue_item_count >= 1
  and .source_triage_bucket_count == 11
  and .owner_bucket_count == 11
  and .bucket_primary_owner_assigned_count == 11
  and .bucket_primary_owner_assignment_complete == true
  and .commit_level_primary_owner_review_required == true
  and .commit_level_primary_owner_review_required_count == (.source_multi_slice_commit_count + ((.owner_rows[] | select(.bucket_id == "unclassified_manual_other") | .commit_count) // 0))
  and .primary_owner_count >= 5
  and .review_order_complete == true
  and .push_performed == false
  and .rebase_performed == false
  and .reset_performed == false
  and .squash_performed == false
  and .history_rewrite_performed == false
  and .external_action_performed == false
  and .upload_performed == false
  and .publish_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
  and .beta_cohort_evidence_claimed == false
  and .production_ready_ui_claimed == false
  and .commercial_launch_evidence_claimed == false
  and .public_network_live_exposure_claimed == false
  and (.no_credit_boundary | contains("local review primary-owner plan only"))
  and (.reviewer_next_action | contains("before any external push or history operation"))
' "$SUMMARY" >/dev/null

{
  printf '# Trillionnium World Review Primary-Owner Plan\n\n'
  printf -- '- status: `%s`\n' "$(jq -r '.status' "$SUMMARY")"
  printf -- '- triage queue items: `%s`\n' "$(jq -r '.source_triage_queue_item_count' "$SUMMARY")"
  printf -- '- owner buckets assigned: `%s` / `%s`\n' \
    "$(jq -r '.bucket_primary_owner_assigned_count' "$SUMMARY")" \
    "$(jq -r '.owner_bucket_count' "$SUMMARY")"
  printf -- '- commit-level owner review required count: `%s`\n' \
    "$(jq -r '.commit_level_primary_owner_review_required_count' "$SUMMARY")"
  printf -- '- push/rebase/reset/squash/history rewrite/external action: `%s` / `%s` / `%s` / `%s` / `%s` / `%s`\n' \
    "$(jq -r '.push_performed' "$SUMMARY")" \
    "$(jq -r '.rebase_performed' "$SUMMARY")" \
    "$(jq -r '.reset_performed' "$SUMMARY")" \
    "$(jq -r '.squash_performed' "$SUMMARY")" \
    "$(jq -r '.history_rewrite_performed' "$SUMMARY")" \
    "$(jq -r '.external_action_performed' "$SUMMARY")"
  printf -- '- public launch ready claimed: `%s`\n' "$(jq -r '.public_launch_ready_claimed' "$SUMMARY")"
  printf -- '- Android S5 real-device claimed: `%s`\n\n' "$(jq -r '.android_s5_real_device_claimed' "$SUMMARY")"
  printf '## Owner Rows\n\n'
  jq -r '.owner_rows[] | "- \(.review_order). `\(.bucket_id)` -> `\(.primary_owner)` (`\(.commit_count)` commits): \(.exit_rule)"' "$SUMMARY"
} >"$SUMMARY_MD"

printf 'TRILLIONNIUM_WORLD_REVIEW_PRIMARY_OWNER_PLAN_GREEN %s %s\n' "$SUMMARY" "$SUMMARY_MD"
