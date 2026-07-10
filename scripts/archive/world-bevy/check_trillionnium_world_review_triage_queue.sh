#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
DOC_REL="docs/development/trillionnium-world-review-triage-queue-2026-07-07.md"
DOC="$ROOT/$DOC_REL"
MANIFEST_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-slice-manifest.json"
SUMMARY="$ACCEPTANCE_DIR/trillionnium-world-review-triage-queue.json"
SUMMARY_MD="$ACCEPTANCE_DIR/trillionnium-world-review-triage-queue.md"
mkdir -p "$ACCEPTANCE_DIR"

require_text() {
  local path="$1"
  local needle="$2"
  if ! grep -Fq -- "$needle" "$path"; then
    echo "[FAIL] $path missing required text: $needle" >&2
    exit 1
  fi
}

sample_unclassified_json() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf '[]'
    return
  fi
  jq -R -s '
    split("\n")
    | map(select(length > 0))
    | .[:8]
    | map(split("\t") | {commit: .[0], short: .[1], subject: .[2], triage_bucket: .[3]})
  ' "$path"
}

sample_multi_json() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf '[]'
    return
  fi
  jq -R -s '
    split("\n")
    | map(select(length > 0))
    | .[:8]
    | map(split("\t") | {
        commit: .[0],
        short: .[1],
        subject: .[2],
        match_count: (.[3] | tonumber),
        matched_slices: (.[4] | split(",")),
        triage_bucket: .[5]
      })
  ' "$path"
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

bucket_description() {
  case "$1" in
    unclassified_docs_plan_truth_source) printf 'Docs and planning commits needing truth-source review.' ;;
    unclassified_generated_count_surface) printf 'Count-exposure commits needing ownership by an artifact wrapper or guard surface.' ;;
    unclassified_classic_evidence_surface) printf 'Classic UI/evidence surface commits needing primary slice assignment.' ;;
    unclassified_bot_executor_surface) printf 'Bot/executor commits needing RTS runtime/data or Bevy playable ownership.' ;;
    unclassified_map_or_modeling_surface) printf 'Map/modeling commits needing boundary and ownership review.' ;;
    unclassified_manual_other) printf 'Remaining unclassified commits requiring manual owner assignment.' ;;
    multi_public_boundary_overlap) printf 'Multi-slice commits involving public/release/external evidence boundaries.' ;;
    multi_first_contact_readability_renderer_overlap) printf 'Multi-slice commits mixing First Contact readability and renderer ownership.' ;;
    multi_native_bevy_rts_boundary_overlap) printf 'Multi-slice commits crossing native Bevy and renderer-neutral RTS boundaries.' ;;
    multi_release_native_handoff_overlap) printf 'Multi-slice commits crossing release truth and native Bevy handoff surfaces.' ;;
    multi_manual_overlap) printf 'Remaining multi-slice commits requiring manual primary-owner review.' ;;
    *) printf 'Manual review required.' ;;
  esac
}

bucket_next_action() {
  case "$1" in
    unclassified_docs_plan_truth_source) printf 'Confirm the doc is current truth or route it to archive/reference-only before review.' ;;
    unclassified_generated_count_surface) printf 'Assign each count exposure to the artifact/checker that owns the count contract.' ;;
    unclassified_classic_evidence_surface) printf 'Route each classic surface to Bevy playable, renderer, or release-truth review before push planning.' ;;
    unclassified_bot_executor_surface) printf 'Decide whether bot/executor changes belong to RTS runtime/data, Bevy integration, or release evidence.' ;;
    unclassified_map_or_modeling_surface) printf 'Verify no live ingestion or public map-pack credit is implied, then assign owner slice.' ;;
    unclassified_manual_other) printf 'Read each commit and assign a primary reviewer slice manually.' ;;
    multi_public_boundary_overlap) printf 'Review public/release/no-credit boundaries first, before product or runtime details.' ;;
    multi_first_contact_readability_renderer_overlap) printf 'Decide whether human-playtest/product readability or renderer micro-cue ownership is primary.' ;;
    multi_native_bevy_rts_boundary_overlap) printf 'Check renderer-neutral contracts before reviewing Bevy draw/runtime integration.' ;;
    multi_release_native_handoff_overlap) printf 'Check release truth and no-credit handoff boundaries before playable-client review.' ;;
    multi_manual_overlap) printf 'Read each commit and choose a primary owner or later split strategy manually.' ;;
    *) printf 'Manual review required.' ;;
  esac
}

if [[ ! -f "$DOC" ]]; then
  echo "[FAIL] missing review triage queue doc: $DOC" >&2
  exit 1
fi

require_text "$DOC" "Status: local review triage queue."
require_text "$DOC" "Unclassified commits are bucketed for review"
require_text "$DOC" "Multi-slice commits remain overlap risk"
require_text "$DOC" "Do not convert this local queue into public-launch"
require_text "$DOC" '| `unclassified_docs_plan_truth_source` |'
require_text "$DOC" '| `unclassified_generated_count_surface` |'
require_text "$DOC" '| `multi_public_boundary_overlap` |'
require_text "$DOC" '| `multi_first_contact_readability_renderer_overlap` |'

"$ROOT/scripts/check_trillionnium_world_review_slice_manifest.sh" >/dev/null
jq -e '
  .contract_version == "trillionnium_world_review_slice_manifest_v1"
  and .status == "review_slice_manifest_ready"
  and .review_slice_count == 6
  and .total_ahead_count >= 1
  and ((.manifested_commit_count + .unclassified_commit_count) == .total_ahead_count)
  and .external_action_performed == false
  and .history_rewrite_performed == false
' "$MANIFEST_JSON" >/dev/null

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

unclassified_buckets=(
  unclassified_docs_plan_truth_source
  unclassified_generated_count_surface
  unclassified_classic_evidence_surface
  unclassified_bot_executor_surface
  unclassified_map_or_modeling_surface
  unclassified_manual_other
)
multi_buckets=(
  multi_public_boundary_overlap
  multi_first_contact_readability_renderer_overlap
  multi_native_bevy_rts_boundary_overlap
  multi_release_native_handoff_overlap
  multi_manual_overlap
)

: >"$TMP_DIR/unclassified_all.tsv"
: >"$TMP_DIR/multi_all.tsv"
for bucket in "${unclassified_buckets[@]}" "${multi_buckets[@]}"; do
  : >"$TMP_DIR/$bucket.tsv"
done

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
  if ((match_count == 0)); then
    bucket="$(classify_unclassified "$lower")"
    printf '%s\t%s\t%s\t%s\n' "$hash" "$short_hash" "$subject" "$bucket" >>"$TMP_DIR/$bucket.tsv"
    printf '%s\t%s\t%s\t%s\n' "$hash" "$short_hash" "$subject" "$bucket" >>"$TMP_DIR/unclassified_all.tsv"
  elif ((match_count > 1)); then
    IFS=,
    joined="${matches[*]}"
    unset IFS
    bucket="$(classify_multi "$joined")"
    printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$hash" "$short_hash" "$subject" "$match_count" "$joined" "$bucket" >>"$TMP_DIR/$bucket.tsv"
    printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$hash" "$short_hash" "$subject" "$match_count" "$joined" "$bucket" >>"$TMP_DIR/multi_all.tsv"
  fi
done < <(git -C "$ROOT" log --format='%H%x09%s' origin/main..HEAD)

bucket_jsonl="$TMP_DIR/buckets.jsonl"
: >"$bucket_jsonl"
for bucket in "${unclassified_buckets[@]}"; do
  path="$TMP_DIR/$bucket.tsv"
  count="$(wc -l <"$path" | tr -d ' ')"
  sample="$(sample_unclassified_json "$path")"
  jq -n \
    --arg type "unclassified" \
    --arg id "$bucket" \
    --arg severity "manual_review" \
    --arg description "$(bucket_description "$bucket")" \
    --arg next_action "$(bucket_next_action "$bucket")" \
    --argjson commit_count "$count" \
    --argjson sample_commits "$sample" \
    '{
      type: $type,
      id: $id,
      severity: $severity,
      commit_count: $commit_count,
      description: $description,
      next_action: $next_action,
      sample_commits: $sample_commits
    }' >>"$bucket_jsonl"
done
for bucket in "${multi_buckets[@]}"; do
  path="$TMP_DIR/$bucket.tsv"
  count="$(wc -l <"$path" | tr -d ' ')"
  sample="$(sample_multi_json "$path")"
  jq -n \
    --arg type "multi_slice_overlap" \
    --arg id "$bucket" \
    --arg severity "primary_owner_required" \
    --arg description "$(bucket_description "$bucket")" \
    --arg next_action "$(bucket_next_action "$bucket")" \
    --argjson commit_count "$count" \
    --argjson sample_commits "$sample" \
    '{
      type: $type,
      id: $id,
      severity: $severity,
      commit_count: $commit_count,
      description: $description,
      next_action: $next_action,
      sample_commits: $sample_commits
    }' >>"$bucket_jsonl"
done

triage_buckets_json="$(jq -s . "$bucket_jsonl")"
manifest_total_ahead_count="$(jq -r '.total_ahead_count' "$MANIFEST_JSON")"
manifest_manifested_commit_count="$(jq -r '.manifested_commit_count' "$MANIFEST_JSON")"
manifest_unclassified_commit_count="$(jq -r '.unclassified_commit_count' "$MANIFEST_JSON")"
manifest_multi_slice_commit_count="$(jq -r '.multi_slice_commit_count' "$MANIFEST_JSON")"
unclassified_bucketed_count="$(wc -l <"$TMP_DIR/unclassified_all.tsv" | tr -d ' ')"
multi_slice_bucketed_count="$(wc -l <"$TMP_DIR/multi_all.tsv" | tr -d ' ')"
triage_queue_item_count="$((unclassified_bucketed_count + multi_slice_bucketed_count))"
origin_commit="$(git -C "$ROOT" rev-parse origin/main)"
head_commit="$(git -C "$ROOT" rev-parse HEAD)"
dirty_count="$(git -C "$ROOT" status --porcelain | wc -l | tr -d ' ')"
unclassified_sample_json="$(sample_unclassified_json "$TMP_DIR/unclassified_all.tsv")"
multi_slice_sample_json="$(sample_multi_json "$TMP_DIR/multi_all.tsv")"

jq -n \
  --arg contract_version "trillionnium_world_review_triage_queue_v1" \
  --arg status "review_triage_queue_ready" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg doc_path "$DOC_REL" \
  --arg manifest_path "acceptance/S6_public_launch/latest/trillionnium-world-review-slice-manifest.json" \
  --arg origin_commit "$origin_commit" \
  --arg head_commit "$head_commit" \
  --argjson dirty_count "$dirty_count" \
  --argjson total_ahead_count "$manifest_total_ahead_count" \
  --argjson manifested_commit_count "$manifest_manifested_commit_count" \
  --argjson unclassified_commit_count "$manifest_unclassified_commit_count" \
  --argjson multi_slice_commit_count "$manifest_multi_slice_commit_count" \
  --argjson unclassified_bucketed_count "$unclassified_bucketed_count" \
  --argjson multi_slice_bucketed_count "$multi_slice_bucketed_count" \
  --argjson triage_queue_item_count "$triage_queue_item_count" \
  --argjson triage_buckets "$triage_buckets_json" \
  --argjson unclassified_sample_commits "$unclassified_sample_json" \
  --argjson multi_slice_sample_commits "$multi_slice_sample_json" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    green: true,
    doc_path: $doc_path,
    manifest_path: $manifest_path,
    origin_main_commit: $origin_commit,
    head_commit: $head_commit,
    dirty_count_at_generation: $dirty_count,
    total_ahead_count: $total_ahead_count,
    manifested_commit_count: $manifested_commit_count,
    unclassified_commit_count: $unclassified_commit_count,
    multi_slice_commit_count: $multi_slice_commit_count,
    unclassified_bucketed_count: $unclassified_bucketed_count,
    multi_slice_bucketed_count: $multi_slice_bucketed_count,
    triage_queue_item_count: $triage_queue_item_count,
    triage_bucket_count: ($triage_buckets | length),
    triage_buckets: $triage_buckets,
    unclassified_sample_commits: $unclassified_sample_commits,
    multi_slice_sample_commits: $multi_slice_sample_commits,
    unclassified_bucket_coverage_complete: ($unclassified_bucketed_count == $unclassified_commit_count),
    multi_slice_bucket_coverage_complete: ($multi_slice_bucketed_count == $multi_slice_commit_count),
    manual_review_required: true,
    primary_owner_assignment_required: ($multi_slice_commit_count > 0),
    local_backlog_risk_active: ($total_ahead_count > 0),
    push_performed: false,
    rebase_performed: false,
    reset_performed: false,
    squash_performed: false,
    history_rewrite_performed: false,
    external_action_performed: false,
    public_launch_ready_claimed: false,
    android_s5_real_device_claimed: false,
    beta_cohort_evidence_claimed: false,
    production_ready_ui_claimed: false,
    commercial_launch_evidence_claimed: false,
    public_network_live_exposure_claimed: false,
    no_credit_boundary: "local review triage queue only; no push, rebase, reset, squash, history rewrite, public launch, Android S5 real-device, beta, production-ready UI, commercial, multi-node, live-traffic, or public-network credit",
    reviewer_next_action: "review_unclassified_bucket_samples_and_multi_slice_overlap_samples_before_any_external_push_or_history_operation"
  }' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_review_triage_queue_v1"
  and .status == "review_triage_queue_ready"
  and .green == true
  and .total_ahead_count >= 1
  and .triage_bucket_count == 11
  and .unclassified_bucketed_count == .unclassified_commit_count
  and .multi_slice_bucketed_count == .multi_slice_commit_count
  and .triage_queue_item_count == (.unclassified_commit_count + .multi_slice_commit_count)
  and .unclassified_bucket_coverage_complete == true
  and .multi_slice_bucket_coverage_complete == true
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
  and .beta_cohort_evidence_claimed == false
  and .production_ready_ui_claimed == false
  and .commercial_launch_evidence_claimed == false
  and .public_network_live_exposure_claimed == false
  and (.no_credit_boundary | contains("local review triage queue only"))
  and (.reviewer_next_action | contains("before_any_external_push_or_history_operation"))
' "$SUMMARY" >/dev/null

{
  printf '# Trillionnium World Review Triage Queue\n\n'
  printf -- '- status: `%s`\n' "$(jq -r '.status' "$SUMMARY")"
  printf -- '- base/head: `%s` / `%s`\n' "$(jq -r '.origin_main_commit' "$SUMMARY")" "$(jq -r '.head_commit' "$SUMMARY")"
  printf -- '- ahead / manifested: `%s` / `%s`\n' "$(jq -r '.total_ahead_count' "$SUMMARY")" "$(jq -r '.manifested_commit_count' "$SUMMARY")"
  printf -- '- unclassified / bucketed: `%s` / `%s`\n' "$(jq -r '.unclassified_commit_count' "$SUMMARY")" "$(jq -r '.unclassified_bucketed_count' "$SUMMARY")"
  printf -- '- multi-slice / bucketed: `%s` / `%s`\n' "$(jq -r '.multi_slice_commit_count' "$SUMMARY")" "$(jq -r '.multi_slice_bucketed_count' "$SUMMARY")"
  printf -- '- queue items: `%s`\n' "$(jq -r '.triage_queue_item_count' "$SUMMARY")"
  printf -- '- push/rebase/reset/squash/history rewrite/external action: `%s` / `%s` / `%s` / `%s` / `%s` / `%s`\n' \
    "$(jq -r '.push_performed' "$SUMMARY")" \
    "$(jq -r '.rebase_performed' "$SUMMARY")" \
    "$(jq -r '.reset_performed' "$SUMMARY")" \
    "$(jq -r '.squash_performed' "$SUMMARY")" \
    "$(jq -r '.history_rewrite_performed' "$SUMMARY")" \
    "$(jq -r '.external_action_performed' "$SUMMARY")"
  printf -- '- public launch ready claimed: `%s`\n' "$(jq -r '.public_launch_ready_claimed' "$SUMMARY")"
  printf -- '- Android S5 real-device claimed: `%s`\n\n' "$(jq -r '.android_s5_real_device_claimed' "$SUMMARY")"
  printf '## Buckets\n\n'
  jq -r '.triage_buckets[] | "- `\(.id)`: `\(.commit_count)` commits; \(.next_action)"' "$SUMMARY"
  printf '\n## Unclassified Sample\n\n'
  jq -r '.unclassified_sample_commits[]? | "- `\(.short)`: \(.subject) (`\(.triage_bucket)`)"' "$SUMMARY"
  printf '\n## Multi-Slice Sample\n\n'
  jq -r '.multi_slice_sample_commits[]? | "- `\(.short)`: \(.subject) (`\(.triage_bucket)`, \(.match_count) slices)"' "$SUMMARY"
} >"$SUMMARY_MD"

printf 'TRILLIONNIUM_WORLD_REVIEW_TRIAGE_QUEUE_GREEN %s %s\n' "$SUMMARY" "$SUMMARY_MD"
