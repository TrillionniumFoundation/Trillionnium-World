#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
DOC_REL="docs/development/trillionnium-world-review-slice-manifest-2026-07-07.md"
DOC="$ROOT/$DOC_REL"
SUMMARY="$ACCEPTANCE_DIR/trillionnium-world-review-slice-manifest.json"
SUMMARY_MD="$ACCEPTANCE_DIR/trillionnium-world-review-slice-manifest.md"
mkdir -p "$ACCEPTANCE_DIR"

require_text() {
  local path="$1"
  local needle="$2"
  if ! grep -Fq -- "$needle" "$path"; then
    echo "[FAIL] $path missing required text: $needle" >&2
    exit 1
  fi
}

append_slice_match() {
  local slice="$1"
  local hash="$2"
  local short_hash="$3"
  local subject="$4"
  printf '%s\t%s\t%s\n' "$hash" "$short_hash" "$subject" >>"$TMP_DIR/$slice.tsv"
}

sample_commits_json() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf '[]'
    return
  fi
  jq -R -s '
    split("\n")
    | map(select(length > 0))
    | .[:8]
    | map(split("\t") | {commit: .[0], short: .[1], subject: .[2]})
  ' "$path"
}

slice_priority() {
  case "$1" in
    release_truth_and_public_boundary) printf '1' ;;
    native_bevy_playable_client) printf '2' ;;
    first_contact_product_readability) printf '3' ;;
    first_contact_renderer_micro_cues) printf '4' ;;
    rts_runtime_data_boundaries) printf '5' ;;
    external_evidence_collection_blockers) printf '6' ;;
    *) printf '99' ;;
  esac
}

slice_question() {
  case "$1" in
    release_truth_and_public_boundary)
      printf 'Does every release/public/S5/beta/commercial claim stay tied to real evidence?'
      ;;
    native_bevy_playable_client)
      printf 'Can the local native Bevy client be reviewed and replayed without treating CEX as the product client?'
      ;;
    first_contact_product_readability)
      printf 'Can a reviewer understand selected group, objective, queue, and blocked route before more renderer tuning?'
      ;;
    first_contact_renderer_micro_cues)
      printf 'Did focused visual cleanup preserve simulation while removing status-like bars and hot overlays?'
      ;;
    rts_runtime_data_boundaries)
      printf 'Are renderer-neutral RTS contracts separated from Bevy draw math and proprietary/OpenRA-copy claims?'
      ;;
    external_evidence_collection_blockers)
      printf 'Which real external evidence rows remain blocked instead of satisfied by local templates?'
      ;;
    *)
      printf 'Manual review required.'
      ;;
  esac
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

if [[ ! -f "$DOC" ]]; then
  echo "[FAIL] missing review slice manifest doc: $DOC" >&2
  exit 1
fi

require_text "$DOC" "Status: local review-slice commit-range manifest."
require_text "$DOC" "read-only manifest over the current git range"
require_text "$DOC" "Unclassified commits remain manual-review risk"
require_text "$DOC" "Do not convert this local manifest into public-launch"
require_text "$DOC" '| `release_truth_and_public_boundary` |'
require_text "$DOC" '| `native_bevy_playable_client` |'
require_text "$DOC" '| `first_contact_product_readability` |'
require_text "$DOC" '| `first_contact_renderer_micro_cues` |'
require_text "$DOC" '| `rts_runtime_data_boundaries` |'
require_text "$DOC" '| `external_evidence_collection_blockers` |'

"$ROOT/scripts/check_trillionnium_world_review_slice_strategy.sh" >/dev/null
REVIEW_SLICE_STRATEGY_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-slice-strategy.json"
jq -e '
  .contract_version == "trillionnium_world_review_slice_strategy_v1"
  and .review_slice_count == 6
  and .external_action_performed == false
  and .push_performed == false
  and .rebase_performed == false
  and .reset_performed == false
' "$REVIEW_SLICE_STRATEGY_JSON" >/dev/null

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

slice_ids=(
  release_truth_and_public_boundary
  native_bevy_playable_client
  first_contact_product_readability
  first_contact_renderer_micro_cues
  rts_runtime_data_boundaries
  external_evidence_collection_blockers
)

for slice_id in "${slice_ids[@]}"; do
  : >"$TMP_DIR/$slice_id.tsv"
done
: >"$TMP_DIR/matched.tsv"
: >"$TMP_DIR/multi.tsv"
: >"$TMP_DIR/unclassified.tsv"

while IFS=$'\t' read -r hash subject; do
  [[ -n "$hash" ]] || continue
  short_hash="${hash:0:10}"
  files="$(git -C "$ROOT" show --name-only --format= "$hash" | sed '/^$/d')"
  lower="$(printf '%s\n%s\n' "$subject" "$files" | tr '[:upper:]' '[:lower:]')"
  match_count=0

  if matches_release_truth "$lower"; then
    append_slice_match release_truth_and_public_boundary "$hash" "$short_hash" "$subject"
    match_count=$((match_count + 1))
  fi
  if matches_native_bevy "$lower"; then
    append_slice_match native_bevy_playable_client "$hash" "$short_hash" "$subject"
    match_count=$((match_count + 1))
  fi
  if matches_first_contact_readability "$lower"; then
    append_slice_match first_contact_product_readability "$hash" "$short_hash" "$subject"
    match_count=$((match_count + 1))
  fi
  if matches_first_contact_micro_cues "$lower"; then
    append_slice_match first_contact_renderer_micro_cues "$hash" "$short_hash" "$subject"
    match_count=$((match_count + 1))
  fi
  if matches_rts_boundaries "$lower"; then
    append_slice_match rts_runtime_data_boundaries "$hash" "$short_hash" "$subject"
    match_count=$((match_count + 1))
  fi
  if matches_external_blockers "$lower"; then
    append_slice_match external_evidence_collection_blockers "$hash" "$short_hash" "$subject"
    match_count=$((match_count + 1))
  fi

  if ((match_count == 0)); then
    printf '%s\t%s\t%s\n' "$hash" "$short_hash" "$subject" >>"$TMP_DIR/unclassified.tsv"
  else
    printf '%s\t%s\t%s\n' "$hash" "$short_hash" "$subject" >>"$TMP_DIR/matched.tsv"
  fi
  if ((match_count > 1)); then
    printf '%s\t%s\t%s\t%s\n' "$hash" "$short_hash" "$subject" "$match_count" >>"$TMP_DIR/multi.tsv"
  fi
done < <(git -C "$ROOT" log --format='%H%x09%s' origin/main..HEAD)

slice_summaries_jsonl="$TMP_DIR/slices.jsonl"
: >"$slice_summaries_jsonl"
for slice_id in "${slice_ids[@]}"; do
  slice_path="$TMP_DIR/$slice_id.tsv"
  matching_commit_count="$(wc -l <"$slice_path" | tr -d ' ')"
  sample_commits="$(sample_commits_json "$slice_path")"
  latest_commit="$(awk -F '\t' 'NR == 1 {print $1}' "$slice_path")"
  oldest_commit="$(awk -F '\t' 'END {print $1}' "$slice_path")"
  jq -n \
    --arg id "$slice_id" \
    --argjson priority "$(slice_priority "$slice_id")" \
    --arg review_question "$(slice_question "$slice_id")" \
    --argjson matching_commit_count "$matching_commit_count" \
    --arg latest_commit "$latest_commit" \
    --arg oldest_commit "$oldest_commit" \
    --argjson sample_commits "$sample_commits" \
    '{
      id: $id,
      priority: $priority,
      status: (if $matching_commit_count > 0 then "local_commit_range_manifested" else "no_matching_commits_in_current_range" end),
      review_question: $review_question,
      matching_commit_count: $matching_commit_count,
      latest_commit: $latest_commit,
      oldest_commit: $oldest_commit,
      sample_commits: $sample_commits
    }' >>"$slice_summaries_jsonl"
done

review_slices_json="$(jq -s . "$slice_summaries_jsonl")"
unclassified_sample_json="$(sample_commits_json "$TMP_DIR/unclassified.tsv")"
multi_slice_sample_json="$(sample_commits_json "$TMP_DIR/multi.tsv")"
origin_commit="$(git -C "$ROOT" rev-parse origin/main)"
head_commit="$(git -C "$ROOT" rev-parse HEAD)"
total_ahead_count="$(git -C "$ROOT" rev-list --count origin/main..HEAD)"
dirty_count="$(git -C "$ROOT" status --porcelain | wc -l | tr -d ' ')"
manifested_commit_count="$(wc -l <"$TMP_DIR/matched.tsv" | tr -d ' ')"
unclassified_commit_count="$(wc -l <"$TMP_DIR/unclassified.tsv" | tr -d ' ')"
multi_slice_commit_count="$(wc -l <"$TMP_DIR/multi.tsv" | tr -d ' ')"
slice_match_total_count="$(jq '[.[].matching_commit_count] | add' <<<"$review_slices_json")"

jq -n \
  --arg contract_version "trillionnium_world_review_slice_manifest_v1" \
  --arg status "review_slice_manifest_ready" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg doc_path "$DOC_REL" \
  --arg base_ref "origin/main" \
  --arg origin_commit "$origin_commit" \
  --arg head_commit "$head_commit" \
  --argjson total_ahead_count "$total_ahead_count" \
  --argjson dirty_count "$dirty_count" \
  --argjson review_slices "$review_slices_json" \
  --argjson manifested_commit_count "$manifested_commit_count" \
  --argjson unclassified_commit_count "$unclassified_commit_count" \
  --argjson multi_slice_commit_count "$multi_slice_commit_count" \
  --argjson slice_match_total_count "$slice_match_total_count" \
  --argjson unclassified_sample_commits "$unclassified_sample_json" \
  --argjson multi_slice_sample_commits "$multi_slice_sample_json" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    green: true,
    doc_path: $doc_path,
    base_ref: $base_ref,
    origin_main_commit: $origin_commit,
    head_commit: $head_commit,
    total_ahead_count: $total_ahead_count,
    dirty_count_at_generation: $dirty_count,
    review_slices: $review_slices,
    review_slice_count: ($review_slices | length),
    manifested_commit_count: $manifested_commit_count,
    unclassified_commit_count: $unclassified_commit_count,
    unclassified_manual_review_required: ($unclassified_commit_count > 0),
    multi_slice_commit_count: $multi_slice_commit_count,
    slice_match_total_count: $slice_match_total_count,
    unclassified_sample_commits: $unclassified_sample_commits,
    multi_slice_sample_commits: $multi_slice_sample_commits,
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
    no_credit_boundary: "local review-slice commit-range manifest only; no push, rebase, reset, squash, history rewrite, public launch, Android S5 real-device, beta, production-ready UI, commercial, multi-node, live-traffic, or public-network credit",
    reviewer_next_action: "review_slice_samples_then_triage_unclassified_commits_before_any_external_push_or_history_operation"
  }' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_review_slice_manifest_v1"
  and .status == "review_slice_manifest_ready"
  and .green == true
  and .total_ahead_count >= 1
  and .review_slice_count == 6
  and ([.review_slices[].id] == [
    "release_truth_and_public_boundary",
    "native_bevy_playable_client",
    "first_contact_product_readability",
    "first_contact_renderer_micro_cues",
    "rts_runtime_data_boundaries",
    "external_evidence_collection_blockers"
  ])
  and (.review_slices | all(.matching_commit_count >= 0))
  and ((.manifested_commit_count + .unclassified_commit_count) == .total_ahead_count)
  and .slice_match_total_count >= .manifested_commit_count
  and .local_backlog_risk_active == true
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
  and (.no_credit_boundary | contains("local review-slice commit-range manifest only"))
  and (.reviewer_next_action | contains("triage_unclassified_commits"))
' "$SUMMARY" >/dev/null

{
  printf '# Trillionnium World Review Slice Manifest\n\n'
  printf -- '- status: `%s`\n' "$(jq -r '.status' "$SUMMARY")"
  printf -- '- base/head: `%s` / `%s`\n' "$(jq -r '.origin_main_commit' "$SUMMARY")" "$(jq -r '.head_commit' "$SUMMARY")"
  printf -- '- ahead count: `%s`\n' "$(jq -r '.total_ahead_count' "$SUMMARY")"
  printf -- '- dirty count at generation: `%s`\n' "$(jq -r '.dirty_count_at_generation' "$SUMMARY")"
  printf -- '- review slices: `%s`\n' "$(jq -r '.review_slice_count' "$SUMMARY")"
  printf -- '- manifested / unclassified / multi-slice commits: `%s` / `%s` / `%s`\n' \
    "$(jq -r '.manifested_commit_count' "$SUMMARY")" \
    "$(jq -r '.unclassified_commit_count' "$SUMMARY")" \
    "$(jq -r '.multi_slice_commit_count' "$SUMMARY")"
  printf -- '- push/rebase/reset/squash/history rewrite/external action: `%s` / `%s` / `%s` / `%s` / `%s` / `%s`\n' \
    "$(jq -r '.push_performed' "$SUMMARY")" \
    "$(jq -r '.rebase_performed' "$SUMMARY")" \
    "$(jq -r '.reset_performed' "$SUMMARY")" \
    "$(jq -r '.squash_performed' "$SUMMARY")" \
    "$(jq -r '.history_rewrite_performed' "$SUMMARY")" \
    "$(jq -r '.external_action_performed' "$SUMMARY")"
  printf -- '- public launch ready claimed: `%s`\n' "$(jq -r '.public_launch_ready_claimed' "$SUMMARY")"
  printf -- '- Android S5 real-device claimed: `%s`\n\n' "$(jq -r '.android_s5_real_device_claimed' "$SUMMARY")"
  printf '## Slices\n\n'
  jq -r '.review_slices[] | "- `\(.id)`: `\(.matching_commit_count)` commits; latest `\(.latest_commit)`; \(.review_question)"' "$SUMMARY"
  printf '\n## Unclassified Sample\n\n'
  jq -r '.unclassified_sample_commits[]? | "- `\(.short)`: \(.subject)"' "$SUMMARY"
} >"$SUMMARY_MD"

printf 'TRILLIONNIUM_WORLD_REVIEW_SLICE_MANIFEST_GREEN %s %s\n' "$SUMMARY" "$SUMMARY_MD"
