#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
DOC_REL="docs/development/trillionnium-world-review-residual-queue-2026-07-08.md"
DOC="$ROOT/$DOC_REL"
OWNER_PLAN_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-primary-owner-plan.json"
RELEASE_QUEUE_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-release-owner-queue.json"
RUNTIME_QUEUE_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-runtime-owner-queue.json"
SUMMARY="$ACCEPTANCE_DIR/trillionnium-world-review-residual-queue.json"
SUMMARY_MD="$ACCEPTANCE_DIR/trillionnium-world-review-residual-queue.md"
mkdir -p "$ACCEPTANCE_DIR"

require_text() {
  local path="$1"
  local needle="$2"
  if ! grep -Fq -- "$needle" "$path"; then
    echo "[FAIL] $path missing required text: $needle" >&2
    exit 1
  fi
}

matches_release_truth() {
  local lower="$1"
  [[ "$lower" == *"release-review"* ||
    "$lower" == *"release review"* ||
    "$lower" == *"release packet"* ||
    "$lower" == *"packet integrity"* ||
    "$lower" == *"release_readiness"* ||
    "$lower" == *"release readiness"* ||
    "$lower" == *"signoff"* ||
    "$lower" == *"quickcheck"* ||
    "$lower" == *"public-launch"* ||
    "$lower" == *"public_launch"* ||
    "$lower" == *"no-credit"* ||
    "$lower" == *"no credit"* ||
    "$lower" == *"blocker"* ||
    "$lower" == *"readme.md"* ||
    "$lower" == *"root_readme_world_release_review"* ]]
}

matches_native_bevy() {
  local lower="$1"
  [[ "$lower" == *"trnm-world-bevy"* ||
    "$lower" == *"native bevy"* ||
    "$lower" == *"bevy-classic"* ||
    "$lower" == *"bevy classic"* ||
    "$lower" == *"playtest"* ||
    "$lower" == *"runner"* ||
    "$lower" == *"live-window"* ||
    "$lower" == *"live window"* ||
    "$lower" == *"texture sampling"* ||
    "$lower" == *"render asset"* ||
    "$lower" == *"action coach"* ||
    "$lower" == *"player hud"* ||
    "$lower" == *"s5_native_bevy_device"* ]]
}

matches_first_contact_readability() {
  local lower="$1"
  [[ "$lower" == *"first contact readability"* ||
    "$lower" == *"whole-screen first contact"* ||
    "$lower" == *"human playtest"* ||
    "$lower" == *"playtest path"* ||
    "$lower" == *"observation log"* ||
    "$lower" == *"observer runbook"* ||
    "$lower" == *"runbook"* ||
    "$lower" == *"active opening path"* ||
    "$lower" == *"opening path"* ||
    "$lower" == *"command queue"* ||
    "$lower" == *"blocked route"* ||
    "$lower" == *"selected group"* ||
    "$lower" == *"secure beacon"* ||
    "$lower" == *"first-contact-human"* ]]
}

matches_first_contact_micro_cues() {
  local lower="$1"
  [[ "$lower" == *"first contact"* ]] || return 1
  [[ "$lower" == *"fix: shrink"* ||
    "$lower" == *"fix: mute"* ||
    "$lower" == *"fix: suppress"* ||
    "$lower" == *"fix: stagger"* ||
    "$lower" == *"fix: taper"* ||
    "$lower" == *"micro cue"* ||
    "$lower" == *"micro-cue"* ||
    "$lower" == *"micro pips"* ||
    "$lower" == *"micro ticks"* ||
    "$lower" == *"micro arcs"* ||
    "$lower" == *"exact_" ||
    "$lower" == *"pixel_budget"* ||
    "$lower" == *"component_gate"* ]]
}

matches_rts_boundaries() {
  local lower="$1"
  [[ "$lower" == *"trnm-rts-data"* ||
    "$lower" == *"trnm-rts-online"* ||
    "$lower" == *"trnm-rts-bevy-runtime"* ||
    "$lower" == *"trnm-rts-evidence"* ||
    "$lower" == *"renderer-neutral"* ||
    "$lower" == *"runtime/data"* ||
    "$lower" == *"runtime data"* ||
    "$lower" == *"adapter contract"* ||
    "$lower" == *"asset-boundary"* ||
    "$lower" == *"asset boundary"* ||
    "$lower" == *"model-catalog"* ||
    "$lower" == *"model catalog"* ||
    "$lower" == *"openra"* ||
    "$lower" == *"simulation/data"* ]]
}

matches_external_blockers() {
  local lower="$1"
  [[ "$lower" == *"s5 real"* ||
    "$lower" == *"android s5"* ||
    "$lower" == *"android_device"* ||
    "$lower" == *"real-device"* ||
    "$lower" == *"real device"* ||
    "$lower" == *"production map-pack"* ||
    "$lower" == *"production-map-pack"* ||
    "$lower" == *"map_pack"* ||
    "$lower" == *"beta cohort"* ||
    "$lower" == *"cohort"* ||
    "$lower" == *"commercial"* ||
    "$lower" == *"multi-node"* ||
    "$lower" == *"live-traffic"* ||
    "$lower" == *"latency evidence"* ||
    "$lower" == *"public-network"* ||
    "$lower" == *"public network"* ||
    "$lower" == *"public exposure"* ||
    "$lower" == *"external ops"* ||
    "$lower" == *"evidence collection"* ||
    "$lower" == *"evidence validation"* ||
    "$lower" == *"evidence bundle"* ||
    "$lower" == *"operator handoff"* ]]
}

classify_unclassified() {
  local lower="$1"
  if [[ "$lower" == *"docs:"* || "$lower" == *"/docs/"* || "$lower" == *"plan"* || "$lower" == *"roadmap"* ]]; then
    printf 'unclassified_docs_plan_truth_source'
  elif [[ "$lower" == *"expose"* && "$lower" == *"count"* ]]; then
    printf 'unclassified_generated_count_surface'
  elif [[ "$lower" == *"classic"* && ( "$lower" == *"replication"* || "$lower" == *"production"* || "$lower" == *"animation"* || "$lower" == *"modeling"* || "$lower" == *"outcome"* || "$lower" == *"ui"* ) ]]; then
    printf 'unclassified_classic_evidence_surface'
  elif [[ "$lower" == *"bot"* || "$lower" == *"executor"* ]]; then
    printf 'unclassified_bot_executor_surface'
  elif [[ "$lower" == *"map"* || "$lower" == *"modeling"* ]]; then
    printf 'unclassified_map_or_modeling_surface'
  else
    printf 'unclassified_manual_other'
  fi
}

classify_multi() {
  local slices="$1"
  if [[ "$slices" == *"external_evidence_collection_blockers"* || ( "$slices" == *"release_truth_and_public_boundary"* && "$slices" == *"external_evidence_collection_blockers"* ) ]]; then
    printf 'multi_public_boundary_overlap'
  elif [[ "$slices" == *"first_contact_product_readability"* && "$slices" == *"first_contact_renderer_micro_cues"* ]]; then
    printf 'multi_first_contact_readability_renderer_overlap'
  elif [[ "$slices" == *"native_bevy_playable_client"* && "$slices" == *"rts_runtime_data_boundaries"* ]]; then
    printf 'multi_native_bevy_rts_boundary_overlap'
  elif [[ "$slices" == *"release_truth_and_public_boundary"* && "$slices" == *"native_bevy_playable_client"* ]]; then
    printf 'multi_release_native_handoff_overlap'
  else
    printf 'multi_manual_overlap'
  fi
}

bucket_queue_role() {
  case "$1" in
    unclassified_classic_evidence_surface)
      printf 'Route each classic surface to playable-client, renderer, or release-truth review before push planning.'
      ;;
    multi_first_contact_readability_renderer_overlap)
      printf 'Retain the readability/renderer overlap lane, even when zero-count, so future human-playtest-driven items have a bound queue slot.'
      ;;
    unclassified_manual_other)
      printf 'Read each commit and assign a primary reviewer slice manually.'
      ;;
    multi_manual_overlap)
      printf 'Read each overlap and choose a primary owner or later split strategy manually.'
      ;;
    *)
      printf 'Not part of the residual owner-resolution queue.'
      ;;
  esac
}

bucket_review_order() {
  case "$1" in
    unclassified_classic_evidence_surface) printf '7' ;;
    multi_first_contact_readability_renderer_overlap) printf '9' ;;
    unclassified_manual_other) printf '10' ;;
    multi_manual_overlap) printf '11' ;;
    *) printf '99' ;;
  esac
}

bucket_routed_primary_owner() {
  case "$1" in
    unclassified_classic_evidence_surface) printf 'native_bevy_playable_client' ;;
    multi_first_contact_readability_renderer_overlap) printf 'first_contact_product_readability' ;;
    unclassified_manual_other|multi_manual_overlap) printf 'manual_triage_required' ;;
    *) printf 'unmapped' ;;
  esac
}

bucket_resolution_kind() {
  case "$1" in
    unclassified_classic_evidence_surface|unclassified_manual_other) printf 'manual_assignment' ;;
    multi_first_contact_readability_renderer_overlap|multi_manual_overlap) printf 'overlap_resolution' ;;
    *) printf 'unknown' ;;
  esac
}

is_residual_bucket() {
  case "$1" in
    unclassified_classic_evidence_surface|multi_first_contact_readability_renderer_overlap|unclassified_manual_other|multi_manual_overlap)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

if [[ ! -f "$DOC" ]]; then
  echo "[FAIL] missing review residual queue doc: $DOC" >&2
  exit 1
fi

require_text "$DOC" "Status: local residual owner-resolution queue."
require_text "$DOC" "release/public-boundary or RTS runtime/data-boundary queues"
require_text "$DOC" "It does not reassign historical authorship, stage, commit, push, rebase"
require_text "$DOC" "Do not convert this local queue into public-launch"
require_text "$DOC" '| `unclassified_classic_evidence_surface` |'
require_text "$DOC" '| `multi_first_contact_readability_renderer_overlap` |'
require_text "$DOC" '| `unclassified_manual_other` |'
require_text "$DOC" '| `multi_manual_overlap` |'

"$ROOT/scripts/check_trillionnium_world_review_primary_owner_plan.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_review_release_owner_queue.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_review_runtime_owner_queue.sh" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_review_primary_owner_plan_v1"
  and .status == "review_primary_owner_plan_ready"
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
' "$RELEASE_QUEUE_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_review_runtime_owner_queue_v1"
  and .status == "review_runtime_owner_queue_ready"
  and .queue_matches_owner_plan == true
' "$RUNTIME_QUEUE_JSON" >/dev/null

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT
queue_jsonl="$TMP_DIR/residual_queue.jsonl"
: >"$queue_jsonl"
queue_order=0

while IFS=$'\t' read -r hash subject; do
  [[ -n "$hash" ]] || continue
  short_hash="${hash:0:10}"
  files="$(git -C "$ROOT" show --name-only --format= "$hash" | sed '/^$/d')"
  lower="$(printf '%s\n%s\n' "$subject" "$files" | tr '[:upper:]' '[:lower:]')"
  matches=()

  if matches_release_truth "$lower"; then
    matches+=(release_truth_and_public_boundary)
  fi
  if matches_native_bevy "$lower"; then
    matches+=(native_bevy_playable_client)
  fi
  if matches_first_contact_readability "$lower"; then
    matches+=(first_contact_product_readability)
  fi
  if matches_first_contact_micro_cues "$lower"; then
    matches+=(first_contact_renderer_micro_cues)
  fi
  if matches_rts_boundaries "$lower"; then
    matches+=(rts_runtime_data_boundaries)
  fi
  if matches_external_blockers "$lower"; then
    matches+=(external_evidence_collection_blockers)
  fi

  match_count="${#matches[@]}"
  bucket=""
  source_type=""
  source_severity=""
  if ((match_count == 0)); then
    bucket="$(classify_unclassified "$lower")"
    source_type="unclassified"
    source_severity="manual_review"
  elif ((match_count > 1)); then
    IFS=,
    joined="${matches[*]}"
    unset IFS
    bucket="$(classify_multi "$joined")"
    source_type="multi_slice_overlap"
    source_severity="primary_owner_required"
  else
    continue
  fi

  if ! is_residual_bucket "$bucket"; then
    continue
  fi

  queue_order=$((queue_order + 1))
  matched_slices_json="$(printf '%s\n' "${matches[@]}" | jq -R -s 'split("\n") | map(select(length > 0))')"
  changed_paths_json="$(printf '%s\n' "$files" | jq -R -s 'split("\n") | map(select(length > 0))')"
  jq -n \
    --argjson queue_order "$queue_order" \
    --arg commit "$hash" \
    --arg short "$short_hash" \
    --arg subject "$subject" \
    --arg bucket_id "$bucket" \
    --arg routed_primary_owner "$(bucket_routed_primary_owner "$bucket")" \
    --arg source_type "$source_type" \
    --arg source_severity "$source_severity" \
    --arg resolution_kind "$(bucket_resolution_kind "$bucket")" \
    --argjson review_order "$(bucket_review_order "$bucket")" \
    --arg queue_role "$(bucket_queue_role "$bucket")" \
    --argjson match_count "$match_count" \
    --argjson matched_slices "$matched_slices_json" \
    --argjson changed_paths "$changed_paths_json" \
    '{
      queue_order: $queue_order,
      commit: $commit,
      short: $short,
      subject: $subject,
      bucket_id: $bucket_id,
      routed_primary_owner: $routed_primary_owner,
      source_type: $source_type,
      source_severity: $source_severity,
      resolution_kind: $resolution_kind,
      owner_review_order: $review_order,
      queue_role: $queue_role,
      match_count: $match_count,
      matched_slices: $matched_slices,
      changed_path_count: ($changed_paths | length),
      changed_path_sample: ($changed_paths[:12])
    }' >>"$queue_jsonl"
done < <(git -C "$ROOT" log --reverse --format='%H%x09%s' origin/main..HEAD)

queue_items_file="$TMP_DIR/residual_queue.json"
bucket_counts_file="$TMP_DIR/residual_bucket_counts.json"
jq -s 'sort_by(.owner_review_order, .queue_order)' "$queue_jsonl" >"$queue_items_file"
jq -n --slurpfile queue_items "$queue_items_file" '
  ($queue_items[0]) as $items
  |
    [
      "unclassified_classic_evidence_surface",
      "multi_first_contact_readability_renderer_overlap",
      "unclassified_manual_other",
      "multi_manual_overlap"
    ] as $ids
    | $ids
    | map(. as $id | {
        bucket_id: $id,
        routed_primary_owner: (
          if $id == "unclassified_classic_evidence_surface" then "native_bevy_playable_client"
          elif $id == "multi_first_contact_readability_renderer_overlap" then "first_contact_product_readability"
          else "manual_triage_required"
          end
        ),
        owner_review_order: (
          if $id == "unclassified_classic_evidence_surface" then 7
          elif $id == "multi_first_contact_readability_renderer_overlap" then 9
          elif $id == "unclassified_manual_other" then 10
          else 11
          end
        ),
        queue_item_count: ([ $items[] | select(.bucket_id == $id) ] | length),
        manual_assignment_review_item_count: ([ $items[] | select(.bucket_id == $id and .resolution_kind == "manual_assignment") ] | length),
        overlap_resolution_review_item_count: ([ $items[] | select(.bucket_id == $id and .resolution_kind == "overlap_resolution") ] | length)
      })
' >"$bucket_counts_file"

owner_plan_remaining_commit_count="$(jq '
  [.owner_rows[]
   | select(
       .bucket_id == "unclassified_classic_evidence_surface"
       or .bucket_id == "multi_first_contact_readability_renderer_overlap"
       or .bucket_id == "unclassified_manual_other"
       or .bucket_id == "multi_manual_overlap"
     )
   | .commit_count] | add
' "$OWNER_PLAN_JSON")"
owner_plan_remaining_bucket_count="$(jq '
  [.owner_rows[]
   | select(
       .bucket_id == "unclassified_classic_evidence_surface"
       or .bucket_id == "multi_first_contact_readability_renderer_overlap"
       or .bucket_id == "unclassified_manual_other"
       or .bucket_id == "multi_manual_overlap"
     )] | length
' "$OWNER_PLAN_JSON")"
owner_plan_total_commit_count="$(jq '[.owner_rows[].commit_count] | add' "$OWNER_PLAN_JSON")"
release_queue_item_count="$(jq '.release_queue_item_count // 0' "$RELEASE_QUEUE_JSON")"
runtime_queue_item_count="$(jq '.runtime_queue_item_count // 0' "$RUNTIME_QUEUE_JSON")"
residual_queue_item_count="$(jq 'length' "$queue_items_file")"
manual_assignment_review_item_count="$(jq '[.[] | select(.resolution_kind == "manual_assignment")] | length' "$queue_items_file")"
overlap_resolution_review_item_count="$(jq '[.[] | select(.resolution_kind == "overlap_resolution")] | length' "$queue_items_file")"
native_bevy_evidence_review_item_count="$(jq '[.[] | select(.bucket_id == "unclassified_classic_evidence_surface")] | length' "$queue_items_file")"
zero_count_bucket_count="$(jq '[.[] | select(.queue_item_count == 0)] | length' "$bucket_counts_file")"
covered_commit_count_by_all_queues="$((release_queue_item_count + runtime_queue_item_count + residual_queue_item_count))"
origin_commit="$(git -C "$ROOT" rev-parse origin/main)"
head_commit="$(git -C "$ROOT" rev-parse HEAD)"
dirty_count="$(git -C "$ROOT" status --porcelain | wc -l | tr -d ' ')"

jq -n \
  --arg contract_version "trillionnium_world_review_residual_queue_v1" \
  --arg status "review_residual_queue_ready" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg doc_path "$DOC_REL" \
  --arg origin_commit "$origin_commit" \
  --arg head_commit "$head_commit" \
  --argjson dirty_count "$dirty_count" \
  --argjson owner_plan_remaining_bucket_count "$owner_plan_remaining_bucket_count" \
  --argjson owner_plan_remaining_commit_count "$owner_plan_remaining_commit_count" \
  --argjson owner_plan_total_commit_count "$owner_plan_total_commit_count" \
  --argjson release_queue_item_count "$release_queue_item_count" \
  --argjson runtime_queue_item_count "$runtime_queue_item_count" \
  --argjson residual_queue_item_count "$residual_queue_item_count" \
  --argjson manual_assignment_review_item_count "$manual_assignment_review_item_count" \
  --argjson overlap_resolution_review_item_count "$overlap_resolution_review_item_count" \
  --argjson native_bevy_evidence_review_item_count "$native_bevy_evidence_review_item_count" \
  --argjson zero_count_bucket_count "$zero_count_bucket_count" \
  --argjson covered_commit_count_by_all_queues "$covered_commit_count_by_all_queues" \
  --slurpfile bucket_counts_file "$bucket_counts_file" \
  --slurpfile queue_items_file "$queue_items_file" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    green: true,
    doc_path: $doc_path,
    owner_plan_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-primary-owner-plan.json",
    release_owner_queue_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-release-owner-queue.json",
    runtime_owner_queue_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-runtime-owner-queue.json",
    origin_main_commit: $origin_commit,
    head_commit: $head_commit,
    dirty_count_at_generation: $dirty_count,
    queue_scope: "remaining_owner_resolution",
    remaining_bucket_count: $owner_plan_remaining_bucket_count,
    remaining_bucket_ids: [
      "unclassified_classic_evidence_surface",
      "multi_first_contact_readability_renderer_overlap",
      "unclassified_manual_other",
      "multi_manual_overlap"
    ],
    owner_plan_remaining_commit_count: $owner_plan_remaining_commit_count,
    owner_plan_total_commit_count: $owner_plan_total_commit_count,
    release_queue_item_count: $release_queue_item_count,
    runtime_queue_item_count: $runtime_queue_item_count,
    residual_queue_item_count: $residual_queue_item_count,
    queue_matches_owner_plan: ($residual_queue_item_count == $owner_plan_remaining_commit_count),
    covered_commit_count_by_all_queues: $covered_commit_count_by_all_queues,
    all_owner_queue_coverage_complete: ($covered_commit_count_by_all_queues == $owner_plan_total_commit_count),
    manual_assignment_review_item_count: $manual_assignment_review_item_count,
    overlap_resolution_review_item_count: $overlap_resolution_review_item_count,
    native_bevy_evidence_review_item_count: $native_bevy_evidence_review_item_count,
    zero_count_bucket_count: $zero_count_bucket_count,
    bucket_counts: $bucket_counts_file[0],
    queue_items: $queue_items_file[0],
    bucket_coverage_complete: (
      ($bucket_counts_file[0] | length) == $owner_plan_remaining_bucket_count
      and ($residual_queue_item_count == $owner_plan_remaining_commit_count)
      and ($covered_commit_count_by_all_queues == $owner_plan_total_commit_count)
    ),
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
    no_credit_boundary: "local residual owner-resolution queue only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, multi-node, live-traffic, or public-network credit",
    reviewer_next_action: "review queue_items in owner_review_order, routing remaining native-Bevy/readability/manual commits before any external push or history operation"
  }' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_review_residual_queue_v1"
  and .status == "review_residual_queue_ready"
  and .green == true
  and .queue_scope == "remaining_owner_resolution"
  and .remaining_bucket_count == 4
  and .residual_queue_item_count == .owner_plan_remaining_commit_count
  and .queue_matches_owner_plan == true
  and .manual_assignment_review_item_count >= 1
  and .overlap_resolution_review_item_count >= 1
  and .native_bevy_evidence_review_item_count >= 1
  and .zero_count_bucket_count >= 1
  and .covered_commit_count_by_all_queues == .owner_plan_total_commit_count
  and .all_owner_queue_coverage_complete == true
  and .bucket_coverage_complete == true
  and (.bucket_counts | length) == 4
  and ([.bucket_counts[].bucket_id] | sort) == ([
      "unclassified_classic_evidence_surface",
      "multi_first_contact_readability_renderer_overlap",
      "unclassified_manual_other",
      "multi_manual_overlap"
    ] | sort)
  and (.queue_items | length) == .residual_queue_item_count
  and (.queue_items | all(
      .routed_primary_owner == "native_bevy_playable_client"
      or .routed_primary_owner == "first_contact_product_readability"
      or .routed_primary_owner == "manual_triage_required"
    ))
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
  and (.no_credit_boundary | contains("local residual owner-resolution queue only"))
  and (.reviewer_next_action | contains("before any external push or history operation"))
' "$SUMMARY" >/dev/null

{
  printf '# Trillionnium World Review Residual Queue\n\n'
  printf -- '- status: `%s`\n' "$(jq -r '.status' "$SUMMARY")"
  printf -- '- queue scope: `%s`\n' "$(jq -r '.queue_scope' "$SUMMARY")"
  printf -- '- remaining buckets: `%s`\n' "$(jq -r '.remaining_bucket_count' "$SUMMARY")"
  printf -- '- residual queue items: `%s`\n' "$(jq -r '.residual_queue_item_count' "$SUMMARY")"
  printf -- '- release/runtime/residual queue items: `%s` / `%s` / `%s`\n' \
    "$(jq -r '.release_queue_item_count' "$SUMMARY")" \
    "$(jq -r '.runtime_queue_item_count' "$SUMMARY")" \
    "$(jq -r '.residual_queue_item_count' "$SUMMARY")"
  printf -- '- all-owner coverage complete: `%s`\n' "$(jq -r '.all_owner_queue_coverage_complete' "$SUMMARY")"
  printf -- '- manual-assignment review items: `%s`\n' "$(jq -r '.manual_assignment_review_item_count' "$SUMMARY")"
  printf -- '- overlap-resolution review items: `%s`\n' "$(jq -r '.overlap_resolution_review_item_count' "$SUMMARY")"
  printf -- '- native-Bevy evidence review items: `%s`\n' "$(jq -r '.native_bevy_evidence_review_item_count' "$SUMMARY")"
  printf -- '- zero-count buckets retained: `%s`\n' "$(jq -r '.zero_count_bucket_count' "$SUMMARY")"
  printf -- '- push/rebase/reset/squash/history rewrite/external action: `%s` / `%s` / `%s` / `%s` / `%s` / `%s`\n' \
    "$(jq -r '.push_performed' "$SUMMARY")" \
    "$(jq -r '.rebase_performed' "$SUMMARY")" \
    "$(jq -r '.reset_performed' "$SUMMARY")" \
    "$(jq -r '.squash_performed' "$SUMMARY")" \
    "$(jq -r '.history_rewrite_performed' "$SUMMARY")" \
    "$(jq -r '.external_action_performed' "$SUMMARY")"
  printf -- '- public launch ready claimed: `%s`\n' "$(jq -r '.public_launch_ready_claimed' "$SUMMARY")"
  printf -- '- Android S5 real-device claimed: `%s`\n\n' "$(jq -r '.android_s5_real_device_claimed' "$SUMMARY")"
  printf '## Bucket Counts\n\n'
  jq -r '.bucket_counts[] | "- `\(.bucket_id)`: `\(.queue_item_count)` commits; manual assignment `\(.manual_assignment_review_item_count)`; overlap resolution `\(.overlap_resolution_review_item_count)`"' "$SUMMARY"
  printf '\n## Queue Sample\n\n'
  jq -r '.queue_items[:20][] | "- `\(.short)`: \(.subject) (`\(.bucket_id)` -> `\(.routed_primary_owner)`)"' "$SUMMARY"
} >"$SUMMARY_MD"

printf 'TRILLIONNIUM_WORLD_REVIEW_RESIDUAL_QUEUE_GREEN %s %s\n' "$SUMMARY" "$SUMMARY_MD"
