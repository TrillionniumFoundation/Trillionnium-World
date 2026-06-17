#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
TMP_DIR="$(mktemp -d)"
SUCCESS=0

summary_paths=(
  "$ACCEPTANCE_DIR/release-review-packet-integrity-bot-executor-semantic-fixture.json"
  "$ACCEPTANCE_DIR/release-review-packet-integrity-bot-executor-matrix-semantic-fixture.json"
  "$ACCEPTANCE_DIR/release-review-packet-integrity-bot-gap-semantic-fixture.json"
  "$ACCEPTANCE_DIR/release-review-packet-integrity-control-loop-semantic-fixture.json"
  "$ACCEPTANCE_DIR/release-review-packet-integrity-selection-minimap-semantic-fixture.json"
  "$ACCEPTANCE_DIR/release-review-packet-integrity-build-lifecycle-semantic-fixture.json"
  "$ACCEPTANCE_DIR/release-review-packet-integrity-tech-tree-semantic-fixture.json"
  "$ACCEPTANCE_DIR/release-review-packet-integrity-projectile-ability-semantic-fixture.json"
)

backup_dir="$TMP_DIR/summary-backup"
mkdir -p "$backup_dir"
for path in "${summary_paths[@]}"; do
  if [[ -f "$path" ]]; then
    cp "$path" "$backup_dir/$(basename "$path")"
  fi
done

cleanup() {
  if [[ "$SUCCESS" != "1" ]]; then
    for path in "${summary_paths[@]}"; do
      backup="$backup_dir/$(basename "$path")"
      if [[ -f "$backup" ]]; then
        cp "$backup" "$path"
      else
        rm -f "$path"
      fi
    done
  fi
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

write_fixture_summary() {
  local output_path="$1"
  local contract_version="$2"
  local status="$3"
  local fixture_kind="$4"
  local fixture_rule="$5"
  local expected_count="$6"
  local expected_names_json="$7"

  mkdir -p "$(dirname "$output_path")"
  jq -n \
    --arg contract_version "$contract_version" \
    --arg status "$status" \
    --arg generated_at "${TRNM_RELEASE_REVIEW_PACKET_INTEGRITY_FIXTURE_GENERATED_AT:-1970-01-01T00:00:00Z}" \
    --arg fixture_kind "$fixture_kind" \
    --arg fixture_rule "$fixture_rule" \
    --argjson expected_count "$expected_count" \
    --argjson expected_names "$expected_names_json" \
    '{
      contract_version: $contract_version,
      status: $status,
      generated_at: $generated_at,
      source_of_truth: "trillionnium_world_release_review_packet_integrity_rts_semantic_fixture_suite",
      green: true,
      fixture_kind: $fixture_kind,
      fixture_rule: $fixture_rule,
      fake_packet_artifact_count: 121,
      expected_semantic_failure_count: $expected_count,
      expected_semantic_failure_names: $expected_names,
      checksum_mismatch_failure_count: 0,
      bytes_mismatch_failure_count: 0,
      contract_mismatch_failure_count: 0,
      status_mismatch_failure_count: 0,
      ready_for_release_review: true,
      public_launch_ready: false,
      android_s5_real_device_claimed: false,
      proof_scope: "host_side_bevy_runtime_replay_not_android_real_device",
      reviewer_next_action: "inspect_release_review_packet_integrity_rts_semantic_fixture_suite_before_collecting_real_external_public_launch_evidence"
    }' >"$output_path"
}

write_fixture_summary \
  "$ACCEPTANCE_DIR/release-review-packet-integrity-bot-executor-semantic-fixture.json" \
  "trillionnium_world_release_review_packet_integrity_bot_executor_semantic_fixture_v1" \
  "release_review_packet_integrity_bot_executor_semantic_fixture_green" \
  "bot_executor_source_chain_semantic_negative_fixture" \
  "packet_integrity_must_reject_semantically_invalid_bot_executor_source_chain_artifacts_even_when_sha_bytes_contract_and_status_match" \
  9 \
  '["bot_planner_action_executor_semantics","bot_planner_action_executor_log_semantics","bot_planner_action_executor_ppm_semantics","bot_planner_executor_replay_determinism_semantics","bot_planner_executor_replay_determinism_log_semantics","bot_planner_executor_replay_determinism_ppm_semantics","multi_match_bot_executor_evaluation_semantics","multi_match_bot_executor_evaluation_log_semantics","multi_match_bot_executor_evaluation_ppm_semantics"]'

write_fixture_summary \
  "$ACCEPTANCE_DIR/release-review-packet-integrity-bot-executor-matrix-semantic-fixture.json" \
  "trillionnium_world_release_review_packet_integrity_bot_executor_matrix_semantic_fixture_v1" \
  "release_review_packet_integrity_bot_executor_matrix_semantic_fixture_green" \
  "bot_executor_failure_recovery_matrix_semantic_negative_fixture" \
  "packet_integrity_must_reject_semantically_invalid_bot_executor_failure_recovery_matrix_artifacts_even_when_sha_bytes_contract_and_status_match" \
  3 \
  '["bot_executor_failure_recovery_matrix_semantics","bot_executor_failure_recovery_matrix_log_semantics","bot_executor_failure_recovery_matrix_ppm_semantics"]'

write_fixture_summary \
  "$ACCEPTANCE_DIR/release-review-packet-integrity-bot-gap-semantic-fixture.json" \
  "trillionnium_world_release_review_packet_integrity_bot_gap_semantic_fixture_v1" \
  "release_review_packet_integrity_bot_gap_semantic_fixture_green" \
  "bot_gap_foundation_micro_intel_semantic_negative_fixture" \
  "packet_integrity_must_reject_semantically_invalid_bot_gap_foundation_micro_intel_artifacts_even_when_sha_bytes_contract_and_status_match" \
  8 \
  '["bot_decision_state_gap_semantics","bot_decision_state_gap_ppm_semantics","bot_adaptive_build_order_gap_semantics","bot_adaptive_build_order_gap_ppm_semantics","bot_tactical_micro_gap_semantics","bot_tactical_micro_gap_ppm_semantics","bot_map_intel_gap_semantics","bot_map_intel_gap_ppm_semantics"]'

write_fixture_summary \
  "$ACCEPTANCE_DIR/release-review-packet-integrity-control-loop-semantic-fixture.json" \
  "trillionnium_world_release_review_packet_integrity_control_loop_semantic_fixture_v1" \
  "release_review_packet_integrity_control_loop_semantic_fixture_green" \
  "classic_rts_control_loop_semantic_negative_fixture" \
  "packet_integrity_must_reject_semantically_invalid_classic_rts_control_loop_summary_and_ppm_even_when_sha_bytes_contract_and_status_match" \
  2 \
  '["classic_rts_control_loop_semantics","classic_rts_control_loop_ppm_semantics"]'

write_fixture_summary \
  "$ACCEPTANCE_DIR/release-review-packet-integrity-selection-minimap-semantic-fixture.json" \
  "trillionnium_world_release_review_packet_integrity_selection_minimap_semantic_fixture_v1" \
  "release_review_packet_integrity_selection_minimap_semantic_fixture_green" \
  "classic_rts_selection_minimap_semantic_negative_fixture" \
  "packet_integrity_must_reject_semantically_invalid_classic_rts_selection_minimap_summary_and_ppm_even_when_sha_bytes_contract_and_status_match" \
  2 \
  '["classic_rts_selection_minimap_semantics","classic_rts_selection_minimap_ppm_semantics"]'

write_fixture_summary \
  "$ACCEPTANCE_DIR/release-review-packet-integrity-build-lifecycle-semantic-fixture.json" \
  "trillionnium_world_release_review_packet_integrity_build_lifecycle_semantic_fixture_v1" \
  "release_review_packet_integrity_build_lifecycle_semantic_fixture_green" \
  "classic_rts_build_lifecycle_semantic_negative_fixture" \
  "packet_integrity_must_reject_semantically_invalid_classic_rts_build_lifecycle_summary_and_ppm_even_when_sha_bytes_contract_and_status_match" \
  2 \
  '["classic_rts_build_lifecycle_semantics","classic_rts_build_lifecycle_ppm_semantics"]'

write_fixture_summary \
  "$ACCEPTANCE_DIR/release-review-packet-integrity-tech-tree-semantic-fixture.json" \
  "trillionnium_world_release_review_packet_integrity_tech_tree_semantic_fixture_v1" \
  "release_review_packet_integrity_tech_tree_semantic_fixture_green" \
  "classic_rts_tech_tree_semantic_negative_fixture" \
  "packet_integrity_must_reject_semantically_invalid_classic_rts_tech_tree_summary_and_ppm_even_when_sha_bytes_contract_and_status_match" \
  2 \
  '["classic_rts_tech_tree_semantics","classic_rts_tech_tree_ppm_semantics"]'

write_fixture_summary \
  "$ACCEPTANCE_DIR/release-review-packet-integrity-projectile-ability-semantic-fixture.json" \
  "trillionnium_world_release_review_packet_integrity_projectile_ability_semantic_fixture_v1" \
  "release_review_packet_integrity_projectile_ability_semantic_fixture_green" \
  "classic_rts_projectile_ability_semantic_negative_fixture" \
  "packet_integrity_must_reject_semantically_invalid_classic_rts_projectile_ability_summary_and_ppm_even_when_sha_bytes_contract_and_status_match" \
  2 \
  '["classic_rts_projectile_ability_semantics","classic_rts_projectile_ability_ppm_semantics"]'

packet_json="$TMP_DIR/release-review-packet.json"
packet_md="$TMP_DIR/release-review-packet.md"
packet_log="$TMP_DIR/release-review-packet.log"
mutated_packet_json="$TMP_DIR/release-review-packet-mutated.json"
integrity_summary="$TMP_DIR/release-review-packet-integrity-rts-semantic-fixture-suite.json"

TRNM_RELEASE_REVIEW_PACKET_REFRESH_INPUTS=0 \
TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_JSON="$packet_json" \
TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_MD="$packet_md" \
  "$ROOT/scripts/check_trillionnium_world_release_review_packet.sh" >"$packet_log"

cp "$packet_json" "$mutated_packet_json"

artifact_path_for_id() {
  local artifact_id="$1"
  jq -er --arg id "$artifact_id" '.artifacts[] | select(.id == $id) | .path' "$mutated_packet_json"
}

update_packet_artifact() {
  local artifact_id="$1"
  local artifact_path="$2"
  local artifact_sha
  local artifact_bytes
  local contract_version=""
  local status=""
  local update_tmp="$TMP_DIR/packet-update.json"

  artifact_sha="$(sha256sum "$artifact_path" | awk '{print $1}')"
  artifact_bytes="$(wc -c <"$artifact_path" | tr -d ' ')"
  if [[ "$artifact_path" == *.json ]]; then
    contract_version="$(jq -r '.contract_version // empty' "$artifact_path" 2>/dev/null || true)"
    status="$(jq -r '.status // .overall_status // empty' "$artifact_path" 2>/dev/null || true)"
  fi

  jq \
    --arg id "$artifact_id" \
    --arg path "$artifact_path" \
    --arg sha "$artifact_sha" \
    --argjson bytes "$artifact_bytes" \
    --arg contract_version "$contract_version" \
    --arg status "$status" \
    '.artifacts |= map(
      if .id == $id then
        .path = $path
        | .file_status = "present"
        | .sha256 = $sha
        | .bytes = $bytes
        | .contract_version = (if $contract_version == "" then null else $contract_version end)
        | .status = (if $status == "" then null else $status end)
      else
        .
      end
    )' "$mutated_packet_json" >"$update_tmp"
  mv "$update_tmp" "$mutated_packet_json"
}

mutate_json_artifact() {
  local artifact_id="$1"
  local jq_filter="$2"
  local source_path
  local target_path

  source_path="$(artifact_path_for_id "$artifact_id")"
  target_path="$TMP_DIR/${artifact_id}.json"
  jq "$jq_filter" "$source_path" >"$target_path"
  update_packet_artifact "$artifact_id" "$target_path"
}

mutate_ppm_artifact() {
  local artifact_id="$1"
  local min_bytes="$2"
  local target_path="$TMP_DIR/${artifact_id}.ppm"

  printf 'P3\n1 1\n255\n0 0 0\n' >"$target_path"
  truncate -s "$min_bytes" "$target_path"
  update_packet_artifact "$artifact_id" "$target_path"
}

mutate_json_artifact native_bevy_bot_planner_action_executor '.green = false'
mutate_json_artifact native_bevy_bot_planner_action_executor_log '.executor_action_count = 0'
mutate_ppm_artifact native_bevy_bot_planner_action_executor_ppm 8000001
mutate_json_artifact native_bevy_bot_planner_executor_replay_determinism '.green = false'
mutate_json_artifact native_bevy_bot_planner_executor_replay_determinism_log '.replay_action_count = 0'
mutate_ppm_artifact native_bevy_bot_planner_executor_replay_determinism_ppm 8000001
mutate_json_artifact native_bevy_multi_match_bot_executor_evaluation '.green = false'
mutate_json_artifact native_bevy_multi_match_bot_executor_evaluation_log '.variant_count = 0'
mutate_ppm_artifact native_bevy_multi_match_bot_executor_evaluation_ppm 8000001

mutate_json_artifact native_bevy_bot_executor_failure_recovery_matrix '.green = false'
mutate_json_artifact native_bevy_bot_executor_failure_recovery_matrix_log '.source_replay_action_count = 0'
mutate_ppm_artifact native_bevy_bot_executor_failure_recovery_matrix_ppm 8000001

mutate_json_artifact native_bevy_bot_decision_state_gap '.green = false'
mutate_ppm_artifact native_bevy_bot_decision_state_gap_ppm 8000001
mutate_json_artifact native_bevy_bot_adaptive_build_order_gap '.green = false'
mutate_ppm_artifact native_bevy_bot_adaptive_build_order_gap_ppm 8000001
mutate_json_artifact native_bevy_bot_tactical_micro_gap '.green = false'
mutate_ppm_artifact native_bevy_bot_tactical_micro_gap_ppm 8000001
mutate_json_artifact native_bevy_bot_map_intel_gap '.green = false'
mutate_ppm_artifact native_bevy_bot_map_intel_gap_ppm 8000001

mutate_json_artifact native_bevy_classic_rts_control_loop '.green = false'
mutate_ppm_artifact native_bevy_classic_rts_control_loop_ppm 8000001
mutate_json_artifact native_bevy_classic_rts_selection_minimap '.green = false'
mutate_ppm_artifact native_bevy_classic_rts_selection_minimap_ppm 8000001
mutate_json_artifact native_bevy_classic_rts_build_lifecycle '.green = false'
mutate_ppm_artifact native_bevy_classic_rts_build_lifecycle_ppm 8000001
mutate_json_artifact native_bevy_classic_rts_tech_tree '.green = false'
mutate_ppm_artifact native_bevy_classic_rts_tech_tree_ppm 8000001
mutate_json_artifact native_bevy_classic_rts_projectile_ability '.green = false'
mutate_ppm_artifact native_bevy_classic_rts_projectile_ability_ppm 8000001

expected_names="$TMP_DIR/expected-failure-names.json"
jq -n '[
  "bot_planner_action_executor_semantics",
  "bot_planner_action_executor_log_semantics",
  "bot_planner_action_executor_ppm_semantics",
  "bot_planner_executor_replay_determinism_semantics",
  "bot_planner_executor_replay_determinism_log_semantics",
  "bot_planner_executor_replay_determinism_ppm_semantics",
  "multi_match_bot_executor_evaluation_semantics",
  "multi_match_bot_executor_evaluation_log_semantics",
  "multi_match_bot_executor_evaluation_ppm_semantics",
  "bot_executor_failure_recovery_matrix_semantics",
  "bot_executor_failure_recovery_matrix_log_semantics",
  "bot_executor_failure_recovery_matrix_ppm_semantics",
  "bot_decision_state_gap_semantics",
  "bot_decision_state_gap_ppm_semantics",
  "bot_adaptive_build_order_gap_semantics",
  "bot_adaptive_build_order_gap_ppm_semantics",
  "bot_tactical_micro_gap_semantics",
  "bot_tactical_micro_gap_ppm_semantics",
  "bot_map_intel_gap_semantics",
  "bot_map_intel_gap_ppm_semantics",
  "classic_rts_control_loop_semantics",
  "classic_rts_control_loop_ppm_semantics",
  "classic_rts_selection_minimap_semantics",
  "classic_rts_selection_minimap_ppm_semantics",
  "classic_rts_build_lifecycle_semantics",
  "classic_rts_build_lifecycle_ppm_semantics",
  "classic_rts_tech_tree_semantics",
  "classic_rts_tech_tree_ppm_semantics",
  "classic_rts_projectile_ability_semantics",
  "classic_rts_projectile_ability_ppm_semantics"
]' >"$expected_names"

set +e
TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_JSON="$mutated_packet_json" \
TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_MD="$packet_md" \
TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_LOG="$packet_log" \
TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_INTEGRITY_SUMMARY="$integrity_summary" \
  "$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity.sh" --no-refresh >"$TMP_DIR/stdout.log" 2>"$TMP_DIR/stderr.log"
status=$?
set -e
if [[ "$status" -eq 0 ]]; then
  echo "[FAIL] RTS packet semantic fixture suite unexpectedly passed" >&2
  cat "$TMP_DIR/stdout.log" >&2
  cat "$TMP_DIR/stderr.log" >&2
  exit 1
fi
if [[ ! -f "$integrity_summary" ]]; then
  echo "[FAIL] RTS packet semantic fixture suite did not write integrity summary" >&2
  exit 1
fi

jq -e \
  --slurpfile expected "$expected_names" \
  '.status == "release_review_packet_integrity_blocked"
    and .green == false
    and ([.failures[].name] | sort) == ($expected[0] | sort)
    and (([.failures[].detail] | index("sha256_mismatch")) == null)
    and (([.failures[].detail] | index("bytes_mismatch")) == null)
    and (([.failures[].detail] | index("contract_mismatch")) == null)
    and (([.failures[].detail] | index("status_mismatch")) == null)' \
  "$integrity_summary" >/dev/null

for path in "${summary_paths[@]}"; do
  test -s "$path"
done

SUCCESS=1
printf 'TRILLIONNIUM_WORLD_RELEASE_REVIEW_PACKET_INTEGRITY_RTS_SEMANTIC_FIXTURE_SUITE_GREEN %s failures=30 summaries=%s\n' "$integrity_summary" "${#summary_paths[@]}"
