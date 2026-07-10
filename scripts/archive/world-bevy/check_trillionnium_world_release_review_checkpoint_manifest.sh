#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
SUMMARY_JSON="$ACCEPTANCE_DIR/release-review-checkpoint-manifest.json"
SUMMARY_MD="$ACCEPTANCE_DIR/release-review-checkpoint-manifest.md"
if [[ -v TRILLIONNIUM_WORLD_RELEASE_REVIEW_CHECKPOINT_MANIFEST_JSON && -n "$TRILLIONNIUM_WORLD_RELEASE_REVIEW_CHECKPOINT_MANIFEST_JSON" ]]; then
  SUMMARY_JSON="$TRILLIONNIUM_WORLD_RELEASE_REVIEW_CHECKPOINT_MANIFEST_JSON"
fi
if [[ -v TRILLIONNIUM_WORLD_RELEASE_REVIEW_CHECKPOINT_MANIFEST_MD && -n "$TRILLIONNIUM_WORLD_RELEASE_REVIEW_CHECKPOINT_MANIFEST_MD" ]]; then
  SUMMARY_MD="$TRILLIONNIUM_WORLD_RELEASE_REVIEW_CHECKPOINT_MANIFEST_MD"
fi

mkdir -p "$ACCEPTANCE_DIR"

STATUS_LINES="$(mktemp)"
ENTRIES_FILE="$(mktemp)"
trap 'rm -f "$STATUS_LINES" "$ENTRIES_FILE"' EXIT

cd "$ROOT"
git status --porcelain=v1 -uall -- >"$STATUS_LINES"

classify_path() {
  local path="$1"
  case "$path" in
    scripts/check_trillionnium_world_cex_adapter_readiness.sh|scripts/v2/cex_adapter_readiness_script_contract_guard_test.sh|acceptance/S3_repository_adapter/latest/cex-production-adapter-readiness*|docs/development/trillionnium-world-cex-full-split-plan-v1.md)
      printf 'cex_adapter_readiness'
      ;;
    scripts/check_trillionnium_world_release_review_*|scripts/check_trillionnium_world_release_signoff_summary.sh|scripts/v2/release_review_*|scripts/v2/root_readme_world_release_review_quickcheck_guard_test.sh|scripts/v2/release_readiness_release_review_entry_guard_test.sh|acceptance/S6_public_launch/latest/release-review*|acceptance/S6_public_launch/latest/release-signoff-summary.json|README.md|RELEASE_READINESS.md|docs/README.md|docs/development/trillionnium-world-unified-development-doc-v1.md)
      printf 'release_review_surface'
      ;;
    scripts/check_trillionnium_world_public_launch*|scripts/check_trillionnium_world_public_deploy_readiness.sh|scripts/check_trillionnium_world_release_latency_drill.sh|scripts/check_trillionnium_world_release_rollback_backup_drill.sh|scripts/check_trillionnium_world_cohort*|scripts/check_trillionnium_world_external_ops*|scripts/check_trillionnium_world_s5*|scripts/v2/public_launch*|scripts/v2/cohort*|scripts/v2/external*|scripts/v2/s5*|acceptance/S6_public_launch/latest/public-launch*|acceptance/S6_public_launch/latest/cohort*|acceptance/S6_public_launch/latest/external*|acceptance/S5_native_bevy_device/latest/s5*)
      printf 'external_evidence_validators'
      ;;
    scripts/check_trillionnium_world_bevy_*|scripts/run_trillionnium_world_bevy_client.sh|scripts/v2/*bevy*|scripts/v2/*texture*|scripts/v2/*render_asset*|scripts/v2/*live_window*|scripts/v2/*asset_store*|scripts/v2/*player_ui_rescue*|scripts/v2/authored_*|scripts/v2/sprite_asset_binding*|acceptance/S5_native_bevy_device/latest/bevy-*|trillionnium/crates/trnm-world-bevy/*)
      printf 'native_bevy_host_playability'
      ;;
    scripts/check_trillionnium_world_map_*|scripts/check_trillionnium_world_production_map_pack*|scripts/check_trillionnium_world_repository_adapter_boundary.sh|scripts/v2/map_modeling*|scripts/v2/production_map_pack*|acceptance/S4_map_pack_gate/*|trillionnium/crates/trnm-world-map-provider/*)
      printf 'map_pack_repository_boundary'
      ;;
    trillionnium/Cargo.toml|trillionnium/Cargo.lock|trillionnium/crates/trnm-world-*|trillionnium/crates/trnm-cli/*|trillionnium/crates/trnm-node/*|trillionnium/crates/trnm-pouw/*|trillionnium/crates/trnm-rpc/*|trillionnium/crates/trnm-state/*|web4-frontend/*)
      printf 'code_surface'
      ;;
    .github/*|.gitignore|.cargo/*|config/*|rust-toolchain.toml|scripts/check_trillionnium_world_browser_parity.*|scripts/check_trillionnium_world_dev_env.sh)
      printf 'repo_infra_dev_environment'
      ;;
    acceptance/*)
      printf 'generated_acceptance_evidence'
      ;;
    docs/*)
      printf 'docs_planning'
      ;;
    *)
      printf 'uncategorized'
      ;;
  esac
}

while IFS= read -r line; do
  [[ -z "$line" ]] && continue
  status_code="$(printf '%s' "$line" | cut -c1-2)"
  path="$(printf '%s' "$line" | cut -c4-)"
  if [[ "$path" == "$SUMMARY_JSON" || "$path" == "$SUMMARY_MD" ]]; then
    continue
  fi
  category="$(classify_path "$path")"
  tracking_state="tracked"
  if [[ "$status_code" == "??" ]]; then
    tracking_state="untracked"
  fi
  jq -nc \
    --arg status_code "$status_code" \
    --arg tracking_state "$tracking_state" \
    --arg category "$category" \
    --arg path "$path" \
    '{
      status_code: $status_code,
      tracking_state: $tracking_state,
      category: $category,
      path: $path
    }' >>"$ENTRIES_FILE"
done <"$STATUS_LINES"

total_paths="$(jq -s 'length' "$ENTRIES_FILE")"
tracked_modified_count="$(jq -s '[.[] | select(.tracking_state == "tracked")] | length' "$ENTRIES_FILE")"
untracked_count="$(jq -s '[.[] | select(.tracking_state == "untracked")] | length' "$ENTRIES_FILE")"
uncategorized_count="$(jq -s '[.[] | select(.category == "uncategorized")] | length' "$ENTRIES_FILE")"
dirty_tree=false
if [[ "$total_paths" -gt 0 ]]; then
  dirty_tree=true
fi

branch="$(git rev-parse --abbrev-ref HEAD)"
head_sha="$(git rev-parse --short HEAD)"
head_subject="$(git log -1 --pretty=%s)"
upstream="$(git rev-parse --abbrev-ref --symbolic-full-name '@{u}' 2>/dev/null || true)"
ahead_count=0
behind_count=0
if [[ -n "$upstream" ]]; then
  ahead_count="$(git rev-list --count "$upstream"..HEAD 2>/dev/null || printf '0')"
  behind_count="$(git rev-list --count HEAD.."$upstream" 2>/dev/null || printf '0')"
fi

release_review_ci="$ACCEPTANCE_DIR/release-review-ci-gate.json"
release_review_status="$ACCEPTANCE_DIR/release-review-status.json"
cex_adapter_readiness="$ROOT/acceptance/S3_repository_adapter/latest/cex-production-adapter-readiness.json"

read_json_field() {
  local path="$1"
  local expr="$2"
  if [[ -f "$path" ]]; then
    jq -r "$expr // empty" "$path" 2>/dev/null || true
  fi
}

release_review_green="$(read_json_field "$release_review_ci" '.green')"
release_review_status_value="$(read_json_field "$release_review_ci" '.status')"
release_review_ready="$(read_json_field "$release_review_ci" '.ready_for_release_review')"
release_review_public_launch_ready="$(read_json_field "$release_review_ci" '.public_launch_ready')"
release_review_failures="$(read_json_field "$release_review_ci" '(.failures // []) | length')"
release_review_artifact_count="$(read_json_field "$release_review_ci" '.artifact_count')"
cex_adapter_green="$(read_json_field "$cex_adapter_readiness" '.green')"
cex_adapter_status="$(read_json_field "$cex_adapter_readiness" '.status')"
cex_adapter_protocol="$(read_json_field "$cex_adapter_readiness" '.observed.protocol_contract')"
cex_adapter_source_contract="$(read_json_field "$cex_adapter_readiness" '.observed.contract_version')"
cex_adapter_route_record_total="$(read_json_field "$cex_adapter_readiness" '.observed.route_record_total')"
cex_adapter_world_node_count="$(read_json_field "$cex_adapter_readiness" '.observed.world_node_count')"

if [[ -z "$release_review_green" ]]; then release_review_green=false; fi
if [[ -z "$release_review_ready" ]]; then release_review_ready=false; fi
if [[ -z "$release_review_public_launch_ready" ]]; then release_review_public_launch_ready=false; fi
if [[ -z "$release_review_failures" ]]; then release_review_failures=0; fi
if [[ -z "$release_review_artifact_count" ]]; then release_review_artifact_count=0; fi
if [[ -z "$cex_adapter_green" ]]; then cex_adapter_green=false; fi
if [[ -z "$cex_adapter_route_record_total" ]]; then cex_adapter_route_record_total=0; fi
if [[ -z "$cex_adapter_world_node_count" ]]; then cex_adapter_world_node_count=0; fi

jq -n \
  --arg contract_version "trillionnium_world_release_review_checkpoint_manifest_v1" \
  --arg status "checkpoint_manifest_ready_with_dirty_wip" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg branch "$branch" \
  --arg head_sha "$head_sha" \
  --arg head_subject "$head_subject" \
  --arg upstream "$upstream" \
  --arg release_review_ci "$release_review_ci" \
  --arg release_review_status "$release_review_status" \
  --arg cex_adapter_readiness "$cex_adapter_readiness" \
  --arg release_review_status_value "$release_review_status_value" \
  --arg cex_adapter_status "$cex_adapter_status" \
  --arg cex_adapter_protocol "$cex_adapter_protocol" \
  --arg cex_adapter_source_contract "$cex_adapter_source_contract" \
  --arg proof_scope "checkpoint_manifest_only_not_public_launch_evidence" \
  --argjson dirty_tree "$dirty_tree" \
  --argjson total_paths "$total_paths" \
  --argjson tracked_modified_count "$tracked_modified_count" \
  --argjson untracked_count "$untracked_count" \
  --argjson uncategorized_count "$uncategorized_count" \
  --argjson ahead_count "$ahead_count" \
  --argjson behind_count "$behind_count" \
  --argjson release_review_green "$release_review_green" \
  --argjson release_review_ready "$release_review_ready" \
  --argjson release_review_public_launch_ready "$release_review_public_launch_ready" \
  --argjson release_review_failures "$release_review_failures" \
  --argjson release_review_artifact_count "$release_review_artifact_count" \
  --argjson cex_adapter_green "$cex_adapter_green" \
  --argjson cex_adapter_route_record_total "$cex_adapter_route_record_total" \
  --argjson cex_adapter_world_node_count "$cex_adapter_world_node_count" \
  --slurpfile entries "$ENTRIES_FILE" \
  '
  def category_label($c):
    if $c == "cex_adapter_readiness" then "CEX adapter readiness bridge"
    elif $c == "release_review_surface" then "Release-review status, packet, signoff, and docs surface"
    elif $c == "external_evidence_validators" then "External evidence validators and blocker guards"
    elif $c == "native_bevy_host_playability" then "Native/Bevy host-side playability gates"
    elif $c == "map_pack_repository_boundary" then "Map-pack and repository-adapter boundary gates"
    elif $c == "code_surface" then "Rust/frontend code surface"
    elif $c == "repo_infra_dev_environment" then "Repo infrastructure and dev environment"
    elif $c == "generated_acceptance_evidence" then "Generated acceptance evidence"
    elif $c == "docs_planning" then "Planning/docs"
    else "Uncategorized"
    end;
  def category_order($c):
    if $c == "cex_adapter_readiness" then 10
    elif $c == "release_review_surface" then 20
    elif $c == "external_evidence_validators" then 30
    elif $c == "native_bevy_host_playability" then 40
    elif $c == "map_pack_repository_boundary" then 50
    elif $c == "code_surface" then 60
    elif $c == "repo_infra_dev_environment" then 70
    elif $c == "generated_acceptance_evidence" then 80
    elif $c == "docs_planning" then 90
    else 100
    end;
  ($entries
    | group_by(.category)
    | map({
        category: .[0].category,
        label: category_label(.[0].category),
        review_order: category_order(.[0].category),
        path_count: length,
        tracked_modified_count: ([.[] | select(.tracking_state == "tracked")] | length),
        untracked_count: ([.[] | select(.tracking_state == "untracked")] | length),
        paths: (map(.path) | sort)
      })
    | sort_by(.review_order, .category)) as $groups |
  {
    contract_version: $contract_version,
    status: (if $dirty_tree then $status else "checkpoint_manifest_ready_clean_tree" end),
    generated_at: $generated_at,
    source_of_truth: "git_status_porcelain_plus_release_review_evidence",
    green: true,
    dirty_tree: $dirty_tree,
    ready_for_release_review: $release_review_ready,
    public_launch_ready: $release_review_public_launch_ready,
    cex_adapter_ready: $cex_adapter_green,
    working_tree_path_count: $total_paths,
    tracked_modified_count: $tracked_modified_count,
    untracked_path_count: $untracked_count,
    uncategorized_path_count: $uncategorized_count,
    working_tree_group_count: ($groups | length),
    working_tree_entry_count: ($entries | length),
    release_review_artifact_count: $release_review_artifact_count,
    release_review_failure_count: $release_review_failures,
    repository_ahead_count: $ahead_count,
    repository_behind_count: $behind_count,
    proof_scope: $proof_scope,
    repository: {
      branch: $branch,
      head_sha: $head_sha,
      head_subject: $head_subject,
      upstream: (if $upstream == "" then null else $upstream end),
      ahead_count: $ahead_count,
      behind_count: $behind_count
    },
    release_review_snapshot: {
      ci_gate_path: $release_review_ci,
      status_path: $release_review_status,
      status: (if $release_review_status_value == "" then null else $release_review_status_value end),
      green: $release_review_green,
      ready_for_release_review: $release_review_ready,
      public_launch_ready: $release_review_public_launch_ready,
      failure_count: $release_review_failures,
      artifact_count: $release_review_artifact_count
    },
    cex_adapter_readiness_snapshot: {
      evidence_path: $cex_adapter_readiness,
      green: $cex_adapter_green,
      status: (if $cex_adapter_status == "" then null else $cex_adapter_status end),
      protocol_contract: (if $cex_adapter_protocol == "" then null else $cex_adapter_protocol end),
      source_contract: (if $cex_adapter_source_contract == "" then null else $cex_adapter_source_contract end),
      route_record_total: $cex_adapter_route_record_total,
      world_node_count: $cex_adapter_world_node_count
    },
    working_tree: {
      total_paths: $total_paths,
      tracked_modified_count: $tracked_modified_count,
      untracked_count: $untracked_count,
      uncategorized_count: $uncategorized_count,
      groups: $groups,
      entries: ($entries | sort_by(.category, .path))
    },
    recommended_review_order: [
      "cex_adapter_readiness",
      "release_review_surface",
      "external_evidence_validators",
      "native_bevy_host_playability",
      "map_pack_repository_boundary",
      "code_surface",
      "repo_infra_dev_environment",
      "generated_acceptance_evidence",
      "docs_planning",
      "uncategorized"
    ],
    reviewer_next_action: (if $dirty_tree then "review_grouped_wip_then_commit_by_slice" else "no_working_tree_checkpoint_needed" end),
    boundary: {
      checkpoint_manifest_claim: "groups_current_working_tree_only",
      public_launch_claim: "does_not_replace_real_external_evidence",
      commit_claim: "does_not_commit_or_stage_files"
    }
  }' >"$SUMMARY_JSON"

{
  printf '# Trillionnium World Release Review Checkpoint Manifest\n\n'
  printf -- '- generated_at: %s\n' "$(jq -r '.generated_at' "$SUMMARY_JSON")"
  printf -- '- status: %s\n' "$(jq -r '.status' "$SUMMARY_JSON")"
  printf -- '- dirty_tree: %s\n' "$(jq -r '.dirty_tree' "$SUMMARY_JSON")"
  printf -- '- working_tree_path_count: %s\n' "$(jq -r '.working_tree_path_count' "$SUMMARY_JSON")"
  printf -- '- tracked_modified_count: %s\n' "$(jq -r '.tracked_modified_count' "$SUMMARY_JSON")"
  printf -- '- untracked_path_count: %s\n' "$(jq -r '.untracked_path_count' "$SUMMARY_JSON")"
  printf -- '- working_tree_group_count: %s\n' "$(jq -r '.working_tree_group_count' "$SUMMARY_JSON")"
  printf -- '- release_review_ready: %s\n' "$(jq -r '.ready_for_release_review' "$SUMMARY_JSON")"
  printf -- '- public_launch_ready: %s\n' "$(jq -r '.public_launch_ready' "$SUMMARY_JSON")"
  printf -- '- release_review_artifact_count: %s\n' "$(jq -r '.release_review_artifact_count' "$SUMMARY_JSON")"
  printf -- '- release_review_failure_count: %s\n' "$(jq -r '.release_review_failure_count' "$SUMMARY_JSON")"
  printf -- '- cex_adapter_ready: %s\n\n' "$(jq -r '.cex_adapter_ready' "$SUMMARY_JSON")"

  printf '## Recommended Review Order\n\n'
  jq -r '.working_tree.groups[] | "- \(.category): \(.path_count) paths (\(.tracked_modified_count) tracked, \(.untracked_count) untracked)"' "$SUMMARY_JSON"

  printf '\n## Boundary\n\n'
  printf -- '- This manifest groups the current working tree only; it does not stage, commit, or publish anything.\n'
  printf -- '- Release-review readiness can be green while public launch remains blocked.\n'
  printf -- '- CEX adapter readiness is incubator runtime adapter evidence, not real external public-launch evidence.\n'
} >"$SUMMARY_MD"

printf 'TRILLIONNIUM_WORLD_RELEASE_REVIEW_CHECKPOINT_MANIFEST_READY %s %s\n' "$SUMMARY_JSON" "$SUMMARY_MD"
