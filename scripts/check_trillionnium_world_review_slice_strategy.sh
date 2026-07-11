#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
DOC_REL="docs/archive/world-review-2026-07/trillionnium-world-review-slice-strategy-2026-07-07.md"
DOC="$ROOT/$DOC_REL"
SUMMARY="$ACCEPTANCE_DIR/trillionnium-world-review-slice-strategy.json"
SUMMARY_MD="$ACCEPTANCE_DIR/trillionnium-world-review-slice-strategy.md"
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
  echo "[FAIL] missing review slice strategy: $DOC" >&2
  exit 1
fi

require_text "$DOC" "Status: local review-slice strategy."
require_text "$DOC" "This is a grouping plan over existing local commits, not a history rewrite."
require_text "$DOC" "Do not push, rebase, force-push, reset, squash, or delete commits"
require_text "$DOC" "Do not convert local review readiness into public-launch"
require_text "$DOC" '| `release_truth_and_public_boundary` |'
require_text "$DOC" '| `native_bevy_playable_client` |'
require_text "$DOC" '| `first_contact_product_readability` |'
require_text "$DOC" '| `first_contact_renderer_micro_cues` |'
require_text "$DOC" '| `rts_runtime_data_boundaries` |'
require_text "$DOC" '| `external_evidence_collection_blockers` |'
require_text "$DOC" 'Keep `external_evidence_collection_blockers` blocked until real evidence'

head_commit="$(git -C "$ROOT" rev-parse HEAD)"
origin_commit="$(git -C "$ROOT" rev-parse origin/main)"
ahead_count="$(git -C "$ROOT" rev-list --count origin/main..HEAD)"
dirty_count="$(git -C "$ROOT" status --porcelain | wc -l | tr -d ' ')"

slices_json="$(jq -nc '[
  {
    id: "release_truth_and_public_boundary",
    priority: 1,
    status: "local_review_slice",
    review_question: "Does every public/S5/beta/commercial claim stay tied to real evidence?"
  },
  {
    id: "native_bevy_playable_client",
    priority: 2,
    status: "local_review_slice",
    review_question: "Can the local native client be reviewed and replayed without CEX as the product client?"
  },
  {
    id: "first_contact_product_readability",
    priority: 3,
    status: "local_review_slice",
    review_question: "Can a reviewer understand selected group, objective, queue, and blocked route?"
  },
  {
    id: "first_contact_renderer_micro_cues",
    priority: 4,
    status: "local_review_slice",
    review_question: "Are the micro-cue gates preserving product readability without changing simulation?"
  },
  {
    id: "rts_runtime_data_boundaries",
    priority: 5,
    status: "local_review_slice",
    review_question: "Are simulation/data contracts independent from Bevy draw math and proprietary assets?"
  },
  {
    id: "external_evidence_collection_blockers",
    priority: 6,
    status: "blocked_on_real_external_evidence",
    review_question: "What real non-template artifacts are still missing?"
  }
]')"

jq -n \
  --arg contract_version "trillionnium_world_review_slice_strategy_v1" \
  --arg status "review_slice_strategy_ready" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg doc_path "$DOC_REL" \
  --arg head_commit "$head_commit" \
  --arg origin_commit "$origin_commit" \
  --argjson ahead_count "$ahead_count" \
  --argjson dirty_count "$dirty_count" \
  --argjson slices "$slices_json" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    green: true,
    doc_path: $doc_path,
    repository: {
      head_commit: $head_commit,
      origin_main_commit: $origin_commit,
      ahead_count: $ahead_count,
      dirty_count_at_generation: $dirty_count
    },
    review_slices: $slices,
    review_slice_count: ($slices | length),
    local_backlog_risk_active: ($ahead_count > 0),
    external_action_performed: false,
    push_performed: false,
    rebase_performed: false,
    reset_performed: false,
    public_launch_ready_claimed: false,
    android_s5_real_device_claimed: false,
    beta_cohort_evidence_claimed: false,
    production_ready_ui_claimed: false,
    commercial_launch_evidence_claimed: false,
    no_credit_boundary: "local review slicing only; no push, rebase, reset, public launch, Android S5 real-device, beta, production-ready UI, commercial, multi-node, or public-network credit",
    source_of_truth: "The review-slice strategy groups the local ahead-of-origin backlog into review topics without changing git history or external state."
  }' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_review_slice_strategy_v1"
  and .status == "review_slice_strategy_ready"
  and .green == true
  and .repository.ahead_count >= 1
  and .review_slice_count == 6
  and ([.review_slices[].id] == [
    "release_truth_and_public_boundary",
    "native_bevy_playable_client",
    "first_contact_product_readability",
    "first_contact_renderer_micro_cues",
    "rts_runtime_data_boundaries",
    "external_evidence_collection_blockers"
  ])
  and .local_backlog_risk_active == true
  and .external_action_performed == false
  and .push_performed == false
  and .rebase_performed == false
  and .reset_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
  and .beta_cohort_evidence_claimed == false
  and .production_ready_ui_claimed == false
  and .commercial_launch_evidence_claimed == false
  and (.no_credit_boundary | contains("local review slicing only"))
  and (.source_of_truth | contains("without changing git history or external state"))
' "$SUMMARY" >/dev/null

{
  printf '# Trillionnium World Review Slice Strategy\n\n'
  printf -- '- status: `%s`\n' "$(jq -r '.status' "$SUMMARY")"
  printf -- '- ahead count: `%s`\n' "$(jq -r '.repository.ahead_count' "$SUMMARY")"
  printf -- '- dirty count at generation: `%s`\n' "$(jq -r '.repository.dirty_count_at_generation' "$SUMMARY")"
  printf -- '- review slices: `%s`\n' "$(jq -r '.review_slice_count' "$SUMMARY")"
  printf -- '- external action performed: `%s`\n' "$(jq -r '.external_action_performed' "$SUMMARY")"
  printf -- '- public launch ready claimed: `%s`\n' "$(jq -r '.public_launch_ready_claimed' "$SUMMARY")"
  printf -- '- Android S5 real-device claimed: `%s`\n\n' "$(jq -r '.android_s5_real_device_claimed' "$SUMMARY")"
  printf '## Slices\n\n'
  jq -r '.review_slices[] | "- `\(.id)`: \(.review_question)"' "$SUMMARY"
} >"$SUMMARY_MD"

printf 'TRILLIONNIUM_WORLD_REVIEW_SLICE_STRATEGY_GREEN %s %s\n' "$SUMMARY" "$SUMMARY_MD"
