#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
DOC_REL="docs/archive/world-review-2026-07/trillionnium-world-review-generated-count-surface-batch-2026-07-09.md"
DOC="$ROOT/$DOC_REL"
RELEASE_OWNER_QUEUE_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-release-owner-queue.json"
EXECUTION_BATCHES_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-execution-batches.json"
PLAYER_SURFACE_CUES_BATCH_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-first-contact-player-surface-cues-batch.json"
RELEASE_CI_SCRIPT="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"
RELEASE_CI_CONTRACT_GUARD="$ROOT/scripts/v2/release_review_ci_gate_script_contract_guard_test.sh"
PACKET_INTEGRITY_SCRIPT="$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity.sh"
PACKET_INTEGRITY_CONTRACT_GUARD="$ROOT/scripts/v2/release_review_packet_integrity_script_contract_guard_test.sh"
PACKET_INTEGRITY_JSON="$ACCEPTANCE_DIR/release-review-packet-integrity.json"
SUMMARY="$ACCEPTANCE_DIR/trillionnium-world-review-generated-count-surface-batch.json"
SUMMARY_MD="$ACCEPTANCE_DIR/trillionnium-world-review-generated-count-surface-batch.md"
REFRESH_INPUTS="${TRNM_WORLD_REVIEW_GENERATED_COUNT_SURFACE_BATCH_REFRESH_INPUTS:-1}"
EXPECTED_COMMIT_SET_SHA256="52858a89d48b82c98f771becb4f405d8c52e794004e9ad4c3d4044eefe744af1"
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
  echo "[FAIL] missing generated count surface batch doc: $DOC" >&2
  exit 1
fi

require_text "$DOC" "Status: local review generated count surface batch 4."
require_text "$DOC" "unclassified_generated_count_surface"
require_text "$DOC" 'Reviewed commit count: `14`'
require_text "$DOC" 'Per-count unresolved owner assignment count: `0`'
require_text "$DOC" "Generated count surfaces must be assigned to the checker and artifact"
require_text "$DOC" "count_contract_owner_assignment_complete=true"
require_text "$DOC" "owning_checker_artifact_binding_complete=true"
require_text "$DOC" "release_ci_count_guard_bound=true"
require_text "$DOC" "packet_semantic_count_guard_bound=true"
require_text "$DOC" "batch_4_exit_rule_satisfied=true"
require_text "$DOC" "batch_5_unblocked_for_local_review=true"
require_text "$DOC" "next_batch_bucket_id=unclassified_docs_plan_truth_source"

if [[ "$REFRESH_INPUTS" != "0" ]]; then
  "$ROOT/scripts/check_trillionnium_world_review_release_owner_queue.sh" >/dev/null
  TRNM_WORLD_REVIEW_EXECUTION_BATCHES_REFRESH_INPUTS=0 \
    "$ROOT/scripts/check_trillionnium_world_review_execution_batches.sh" >/dev/null
  TRNM_WORLD_REVIEW_FIRST_CONTACT_PLAYER_SURFACE_CUES_BATCH_REFRESH_INPUTS=0 \
    "$ROOT/scripts/check_trillionnium_world_review_first_contact_player_surface_cues_batch.sh" >/dev/null
fi

for input in \
  "$RELEASE_OWNER_QUEUE_JSON" \
  "$EXECUTION_BATCHES_JSON" \
  "$PLAYER_SURFACE_CUES_BATCH_JSON" \
  "$RELEASE_CI_SCRIPT" \
  "$RELEASE_CI_CONTRACT_GUARD" \
  "$PACKET_INTEGRITY_SCRIPT" \
  "$PACKET_INTEGRITY_CONTRACT_GUARD" \
  "$PACKET_INTEGRITY_JSON"; do
  if [[ ! -f "$input" ]]; then
    echo "[FAIL] missing generated count surface batch input: $input" >&2
    exit 1
  fi
done

actual_commit_set_sha256="$(
  jq -r '[.queue_items[] | select(.bucket_id == "unclassified_generated_count_surface") | .commit] | sort | join("\n")' \
    "$RELEASE_OWNER_QUEUE_JSON" | sha256sum | awk '{print $1}'
)"

if [[ "$actual_commit_set_sha256" != "$EXPECTED_COMMIT_SET_SHA256" ]]; then
  echo "[FAIL] generated count surface commit set drifted: $actual_commit_set_sha256" >&2
  exit 1
fi

jq -e '
  .contract_version == "trillionnium_world_review_release_owner_queue_v1"
  and .status == "review_release_owner_queue_ready"
  and .primary_owner == "release_truth_and_public_boundary"
  and .lane_bucket_count == 4
  and .queue_matches_owner_plan == true
  and ([.queue_items[] | select(.bucket_id == "unclassified_generated_count_surface")] | length) == 14
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
    .batch_order == 4
    and .bucket_id == "unclassified_generated_count_surface"
    and .source_queue == "review_release_owner_queue"
    and .primary_owner == "release_truth_and_public_boundary"
    and .execution_kind == "bucket_level_owner_review"
    and .owner_plan_commit_count == 14
    and .queue_item_count == 14
    and .owner_plan_matches_queue == true
    and .commit_level_primary_owner_review_required == false
    and .exit_rule == "Each count exposure must have an owning artifact/checker."
  )] | length) == 1
  and .external_action_performed == false
  and .history_rewrite_performed == false
' "$EXECUTION_BATCHES_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_review_first_contact_player_surface_cues_batch_v1"
  and .status == "review_first_contact_player_surface_cues_sub_batch_8_reviewed"
  and .reviewed_commit_count == 63
  and .unresolved_commit_review_count == 0
  and .batch_3_reviewed_commit_count == 273
  and .batch_3_remaining_commit_level_review_count == 0
  and .sub_batch_8_exit_rule_satisfied == true
  and .batch_3_exit_rule_satisfied == true
  and .batch_4_unblocked_for_local_review == true
  and .next_batch_bucket_id == "unclassified_generated_count_surface"
  and .external_action_performed == false
  and .history_rewrite_performed == false
' "$PLAYER_SURFACE_CUES_BATCH_JSON" >/dev/null

for line in \
  'check_count: $check_count' \
  'checks_total: $check_count' \
  'failed_check_count: $failed_check_count' \
  'checks_failed: $failed_check_count' \
  'artifact_count: $artifact_count'; do
  require_text "$RELEASE_CI_SCRIPT" "$line"
done

for line in \
  'check_count' \
  'checks_total: $check_count' \
  'failed_check_count' \
  'checks_failed: $failed_check_count'; do
  require_text "$RELEASE_CI_CONTRACT_GUARD" "$line"
done

for line in \
  'check_count: $check_count' \
  'checks_total: $check_count' \
  'failed_check_count: $failed_check_count' \
  'checks_failed: $failed_check_count' \
  'artifact_count: $artifact_count' \
  'action_coach_count_semantics' \
  'player_hud_debug_layer_count_semantics' \
  'player_ui_rescue_count_semantics' \
  'first_contact_basin_spec_count_semantics' \
  'in_match_hud_state_replication_count_semantics' \
  'bot_planner_action_executor_count_semantics'; do
  require_text "$PACKET_INTEGRITY_SCRIPT" "$line"
done

for line in \
  'action_coach_count_semantics' \
  'player_hud_debug_layer_count_semantics' \
  'player_ui_rescue_count_semantics' \
  'first_contact_basin_spec_count_semantics' \
  'in_match_hud_state_replication_count_semantics' \
  'bot_planner_action_executor_count_semantics'; do
  require_text "$PACKET_INTEGRITY_CONTRACT_GUARD" "$line"
done

jq -e '
  .status == "release_review_packet_integrity_green_with_public_launch_blockers"
  and .green == true
  and .failed_check_count == 0
  and .artifact_count >= 128
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
  and ([.checks[]? | select(.name == "action_coach_count_semantics" and .status == "ok")] | length) == 1
  and ([.checks[]? | select(.name == "player_hud_debug_layer_count_semantics" and .status == "ok")] | length) == 1
  and ([.checks[]? | select(.name == "player_ui_rescue_count_semantics" and .status == "ok")] | length) == 1
  and ([.checks[]? | select(.name == "first_contact_basin_spec_count_semantics" and .status == "ok")] | length) == 1
  and ([.checks[]? | select(.name == "in_match_hud_state_replication_count_semantics" and .status == "ok")] | length) == 1
  and ([.checks[]? | select(.name == "bot_planner_action_executor_count_semantics" and .status == "ok")] | length) == 1
' "$PACKET_INTEGRITY_JSON" >/dev/null

release_queue_batch_item_count="$(jq '[.queue_items[] | select(.bucket_id == "unclassified_generated_count_surface")] | length' "$RELEASE_OWNER_QUEUE_JSON")"
execution_batch_queue_item_count="$(jq '.batches[] | select(.bucket_id == "unclassified_generated_count_surface") | .queue_item_count' "$EXECUTION_BATCHES_JSON")"
packet_artifact_count="$(jq -r '.artifact_count // 0' "$PACKET_INTEGRITY_JSON")"
packet_failed_check_count="$(jq -r '.failed_check_count // 999' "$PACKET_INTEGRITY_JSON")"

jq -n \
  --arg contract_version "trillionnium_world_review_generated_count_surface_batch_v1" \
  --arg status "review_generated_count_surface_batch_4_ready" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg doc_path "$DOC_REL" \
  --arg bucket_id "unclassified_generated_count_surface" \
  --arg primary_owner "release_truth_and_public_boundary" \
  --arg execution_kind "bucket_level_owner_review" \
  --arg expected_commit_set_sha256 "$EXPECTED_COMMIT_SET_SHA256" \
  --arg actual_commit_set_sha256 "$actual_commit_set_sha256" \
  --arg next_batch_bucket_id "unclassified_docs_plan_truth_source" \
  --argjson release_queue_batch_item_count "$release_queue_batch_item_count" \
  --argjson execution_batch_queue_item_count "$execution_batch_queue_item_count" \
  --argjson packet_artifact_count "$packet_artifact_count" \
  --argjson packet_failed_check_count "$packet_failed_check_count" \
  --argjson commit_reviews '[
    {
      "short_commit": "d819cb0cc6",
      "commit": "d819cb0cc6cbc23af37bfc354d5634bc234cb04d",
      "subject": "chore: expose release CI check counts",
      "review_group": "release_ci_gate_counts",
      "owning_checker": "scripts/check_trillionnium_world_release_review_ci_gate.sh",
      "owning_artifact": "acceptance/S6_public_launch/latest/release-review-ci-gate.json",
      "owner_assignment": "release_ci_gate_check_totals"
    },
    {
      "short_commit": "f0128f155d",
      "commit": "f0128f155d757866134dc76ebc691f28546cc3cc",
      "subject": "fix: expose production desktop review counts",
      "review_group": "production_desktop_review_counts",
      "owning_checker": "scripts/check_trillionnium_world_bevy_classic_rts_production_desktop_review_packet.sh",
      "owning_artifact": "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-desktop-review-packet.json",
      "owner_assignment": "production_desktop_review_packet_counts"
    },
    {
      "short_commit": "6055659452",
      "commit": "60556594524c4f75a9aceb5008218ad3daa0efe3",
      "subject": "fix: expose keyboard replay summary counts",
      "review_group": "player_ui_keyboard_and_hud_counts",
      "owning_checker": "scripts/check_trillionnium_world_bevy_build_branch_title_route_all_branch_keyboard_replay.sh",
      "owning_artifact": "acceptance/S5_native_bevy_device/latest/bevy-build-branch-title-route-all-branch-keyboard-replay.json",
      "owner_assignment": "keyboard_replay_summary_counts"
    },
    {
      "short_commit": "1074712c45",
      "commit": "1074712c4541143947037a4a7ab9037bb0c7b833",
      "subject": "fix: expose player UI summary counts",
      "review_group": "player_ui_keyboard_and_hud_counts",
      "owning_checker": "scripts/check_trillionnium_world_bevy_player_hud_debug_layer.sh; scripts/check_trillionnium_world_bevy_player_ui_rescue.sh",
      "owning_artifact": "acceptance/S5_native_bevy_device/latest/bevy-player-hud-debug-layer.json; acceptance/S5_native_bevy_device/latest/bevy-player-ui-rescue.json",
      "owner_assignment": "player_hud_and_ui_rescue_counts"
    },
    {
      "short_commit": "145a3a8651",
      "commit": "145a3a865192df9c0861811ec6220c36305620fd",
      "subject": "fix: expose in-match HUD state counts",
      "review_group": "player_ui_keyboard_and_hud_counts",
      "owning_checker": "scripts/check_trillionnium_world_bevy_classic_rts_in_match_hud_state_replication.sh",
      "owning_artifact": "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-in-match-hud-state-replication.json",
      "owner_assignment": "in_match_hud_state_count_semantics"
    },
    {
      "short_commit": "2067141296",
      "commit": "20671412968b4de5fc7f03d0d5a9da83c820b687",
      "subject": "fix: expose classic foundation evidence counts",
      "review_group": "classic_foundation_budget_visual_modeling_counts",
      "owning_checker": "classic asset pack and manifest lint checkers",
      "owning_artifact": "classic asset pack and manifest lint artifacts",
      "owner_assignment": "classic_foundation_evidence_counts"
    },
    {
      "short_commit": "bb5e414569",
      "commit": "bb5e414569375fd57f54d1641a4836c35b012632",
      "subject": "fix: expose classic budget evidence counts",
      "review_group": "classic_foundation_budget_visual_modeling_counts",
      "owning_checker": "classic input/render budget checkers",
      "owning_artifact": "classic input/render budget artifacts",
      "owner_assignment": "classic_budget_evidence_counts"
    },
    {
      "short_commit": "58d7106266",
      "commit": "58d7106266e8c8e2446b10a767c670a7d3b4541a",
      "subject": "fix: expose classic visual evidence counts",
      "review_group": "classic_foundation_budget_visual_modeling_counts",
      "owning_checker": "classic model catalog, renderer probe, and scene preview checkers",
      "owning_artifact": "classic visual evidence artifacts",
      "owner_assignment": "classic_visual_evidence_counts"
    },
    {
      "short_commit": "8ff85c8997",
      "commit": "8ff85c89972e008e32fae407cfcc1ee3fdb22a10",
      "subject": "fix: expose classic animation and modeling counts",
      "review_group": "classic_foundation_budget_visual_modeling_counts",
      "owning_checker": "classic animation, selector, isometric, and player motion checkers",
      "owning_artifact": "classic animation and modeling artifacts",
      "owner_assignment": "classic_animation_and_modeling_counts"
    },
    {
      "short_commit": "0ec2b90158",
      "commit": "0ec2b90158708dfd100402a95b8196f48685b22b",
      "subject": "fix: expose classic outcome readiness counts",
      "review_group": "classic_rts_readiness_replication_and_sibling_counts",
      "owning_checker": "campaign outcome UI readiness and combat readability pressure readiness checkers",
      "owning_artifact": "classic outcome and combat readiness artifacts",
      "owner_assignment": "classic_outcome_readiness_counts"
    },
    {
      "short_commit": "4c2cf74ace",
      "commit": "4c2cf74acee0f875593593d3beae10dd53c2c1c8",
      "subject": "fix: expose classic replication counts",
      "review_group": "classic_rts_readiness_replication_and_sibling_counts",
      "owning_checker": "full-screen, match-setup, and shell/meta UI replication checkers",
      "owning_artifact": "classic replication artifacts",
      "owner_assignment": "classic_replication_counts"
    },
    {
      "short_commit": "f546adf25c",
      "commit": "f546adf25ccdfc9786851db7c9c6d39ddbdb21fa",
      "subject": "fix: expose classic production review counts",
      "review_group": "classic_rts_readiness_replication_and_sibling_counts",
      "owning_checker": "scripts/check_trillionnium_world_bevy_classic_rts_production_desktop_review_packet.sh",
      "owning_artifact": "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-desktop-review-packet.json",
      "owner_assignment": "classic_production_review_counts"
    },
    {
      "short_commit": "fa2a4c3c48",
      "commit": "fa2a4c3c4867a1e34c62ecee1ffa1a30cf8c7fe7",
      "subject": "fix: expose classic RTS sibling counts",
      "review_group": "classic_rts_readiness_replication_and_sibling_counts",
      "owning_checker": "classic RTS build lifecycle, control loop, projectile ability, selection minimap, and tech tree checkers",
      "owning_artifact": "classic RTS sibling artifacts",
      "owner_assignment": "classic_rts_sibling_counts"
    },
    {
      "short_commit": "a378ad9e8c",
      "commit": "a378ad9e8c189f975e1d9e65cd0c3e0d6cde2dba",
      "subject": "fix: expose player UI foundation counts",
      "review_group": "player_ui_keyboard_and_hud_counts",
      "owning_checker": "action coach, player HUD debug layer, and player UI rescue checkers",
      "owning_artifact": "action coach, player HUD debug layer, and player UI rescue artifacts",
      "owner_assignment": "player_ui_foundation_counts"
    }
  ]' \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_review_generated_count_surface_batch",
    green: true,
    doc_path: $doc_path,
    batch_order: 4,
    bucket_id: $bucket_id,
    primary_owner: $primary_owner,
    execution_kind: $execution_kind,
    release_queue_batch_item_count: $release_queue_batch_item_count,
    execution_batch_queue_item_count: $execution_batch_queue_item_count,
    required_reviewed_commit_count: 14,
    reviewed_commit_count: 14,
    expected_commit_set_sha256: $expected_commit_set_sha256,
    actual_commit_set_sha256: $actual_commit_set_sha256,
    expected_hash_coverage_complete: ($actual_commit_set_sha256 == $expected_commit_set_sha256),
    commit_level_primary_owner_review_required: false,
    count_contract_owner_assignment_complete: true,
    owning_checker_artifact_binding_complete: true,
    release_ci_count_guard_bound: true,
    packet_semantic_count_guard_bound: true,
    unresolved_generated_count_surface_review_count: 0,
    review_group_count: 5,
    review_group_counts: {
      release_ci_gate_counts: 1,
      production_desktop_review_counts: 1,
      player_ui_keyboard_and_hud_counts: 4,
      classic_foundation_budget_visual_modeling_counts: 4,
      classic_rts_readiness_replication_and_sibling_counts: 4
    },
    prior_batch_3_closed: true,
    packet_integrity_artifact_count: $packet_artifact_count,
    packet_integrity_failed_check_count: $packet_failed_check_count,
    batch_4_exit_rule_satisfied: true,
    batch_5_unblocked_for_local_review: true,
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
    no_credit_boundary: "local generated count surface batch 4 review only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, socket, hosted-service, live multiplayer, OpenRA runtime/replay/network compatibility, render-world extraction completion, GPU upload, live-traffic, public-network, external-evidence, or human-playtest completion credit"
  }' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_review_generated_count_surface_batch_v1"
  and .status == "review_generated_count_surface_batch_4_ready"
  and .batch_order == 4
  and .bucket_id == "unclassified_generated_count_surface"
  and .release_queue_batch_item_count == 14
  and .execution_batch_queue_item_count == 14
  and .reviewed_commit_count == 14
  and .unresolved_generated_count_surface_review_count == 0
  and .expected_hash_coverage_complete == true
  and .commit_level_primary_owner_review_required == false
  and .count_contract_owner_assignment_complete == true
  and .owning_checker_artifact_binding_complete == true
  and .release_ci_count_guard_bound == true
  and .packet_semantic_count_guard_bound == true
  and .review_group_count == 5
  and .review_group_counts.release_ci_gate_counts == 1
  and .review_group_counts.production_desktop_review_counts == 1
  and .review_group_counts.player_ui_keyboard_and_hud_counts == 4
  and .review_group_counts.classic_foundation_budget_visual_modeling_counts == 4
  and .review_group_counts.classic_rts_readiness_replication_and_sibling_counts == 4
  and .prior_batch_3_closed == true
  and .packet_integrity_failed_check_count == 0
  and .batch_4_exit_rule_satisfied == true
  and .batch_5_unblocked_for_local_review == true
  and .next_batch_bucket_id == "unclassified_docs_plan_truth_source"
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
  and .external_action_performed == false
  and .history_rewrite_performed == false
' "$SUMMARY" >/dev/null

{
  printf '# Trillionnium World Review Generated Count Surface Batch\n\n'
  printf -- '- status: `%s`\n' "review_generated_count_surface_batch_4_ready"
  printf -- '- bucket: `%s`\n' "unclassified_generated_count_surface"
  printf -- '- reviewed count exposure commits: `14`\n'
  printf -- '- unresolved generated count surface reviews: `0`\n'
  printf -- '- count owner assignment complete: `true`\n'
  printf -- '- owning checker/artifact binding complete: `true`\n'
  printf -- '- release CI count guard bound: `true`\n'
  printf -- '- packet semantic count guard bound: `true`\n'
  printf -- '- batch 4 exit rule satisfied: `true`\n'
  printf -- '- batch 5 unblocked for local review: `true`\n'
  printf -- '- next batch: `%s`\n' "unclassified_docs_plan_truth_source"
  printf -- '- public launch ready claimed: `false`\n'
  printf -- '- Android S5 real-device claimed: `false`\n'
} >"$SUMMARY_MD"

printf 'TRILLIONNIUM_WORLD_REVIEW_GENERATED_COUNT_SURFACE_BATCH_GREEN %s\n' "$SUMMARY"
