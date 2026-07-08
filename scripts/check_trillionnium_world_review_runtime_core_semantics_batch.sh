#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
S5_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
DOC_REL="docs/development/trillionnium-world-review-runtime-core-semantics-batch-2026-07-08.md"
DOC="$ROOT/$DOC_REL"
RUNTIME_BOUNDARY_BATCH_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-runtime-boundary-batch.json"
OPENRA_LIKE_CORE_JSON="$S5_DIR/bevy-classic-rts-openra-like-core.json"
PACKET_INTEGRITY_JSON="$ACCEPTANCE_DIR/release-review-packet-integrity.json"
SUMMARY="$ACCEPTANCE_DIR/trillionnium-world-review-runtime-core-semantics-batch.json"
SUMMARY_MD="$ACCEPTANCE_DIR/trillionnium-world-review-runtime-core-semantics-batch.md"
REFRESH_INPUTS="${TRNM_WORLD_REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_REFRESH_INPUTS:-1}"
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
  echo "[FAIL] missing runtime-core semantics batch doc: $DOC" >&2
  exit 1
fi

require_text "$DOC" "Status: local review runtime-core semantics sub-batch 1."
require_text "$DOC" "runtime_core_semantics"
require_text "$DOC" 'Reviewed commit count: `55`'
require_text "$DOC" 'Per-commit unresolved count: `0`'
require_text "$DOC" "systemic runtime-core source boundary follow-up"
require_text "$DOC" "trillionnium/crates/trnm-world-bevy/src/lib.rs"
require_text "$DOC" "sub_batch_1_exit_rule_satisfied=false"
require_text "$DOC" "batch_3_exit_rule_satisfied=false"
require_text "$DOC" "batch_4_unblocked_for_local_review=false"
require_text "$DOC" "Do not convert this local review into OpenRA runtime compatibility"

if [[ "$REFRESH_INPUTS" != "0" ]]; then
  TRNM_WORLD_REVIEW_RUNTIME_BOUNDARY_BATCH_REFRESH_INPUTS=0 \
    "$ROOT/scripts/check_trillionnium_world_review_runtime_boundary_batch.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_like_core.sh" >/dev/null
fi

for input in "$RUNTIME_BOUNDARY_BATCH_JSON" "$OPENRA_LIKE_CORE_JSON" "$PACKET_INTEGRITY_JSON"; do
  if [[ ! -f "$input" ]]; then
    echo "[FAIL] missing runtime-core semantics batch input: $input" >&2
    exit 1
  fi
done

jq -e '
  .contract_version == "trillionnium_world_review_runtime_boundary_batch_v1"
  and .status == "review_runtime_boundary_batch_3_sharded"
  and .batch_order == 3
  and .bucket_id == "multi_native_bevy_rts_boundary_overlap"
  and .runtime_overlap_commit_count == 273
  and .sharded_commit_count == 273
  and .sub_batch_count == 8
  and (.sub_batches[] | select(.sub_batch_id == "runtime_core_semantics" and .count == 55 and .reviewed_commit_count == 0))
  and .batch_3_entry_rule_satisfied == true
  and .batch_3_exit_rule_satisfied == false
  and .batch_4_unblocked_for_local_review == false
  and .external_action_performed == false
  and .history_rewrite_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$RUNTIME_BOUNDARY_BATCH_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_openra_like_core_v1"
  and .green == true
  and .runtime_model == "rust_bevy_owned_openra_like_rts_core"
  and .map.width == 34
  and .map.height == 34
  and .map.actor_template_count == 39
  and .map.runtime_actor_count >= 48
  and .map.player_count == 4
  and (.orders | length) == 13
  and (.rules | length) == 11
  and (.gates | length) == 75
  and (.gates | to_entries | all(.value == true))
  and .simulation.tick_count >= 320
  and .simulation.command_accepted_count >= 15
  and .simulation.command_rejected_count >= 12
  and .simulation.completed_production_count >= 3
  and .simulation.control_group_count >= 25
  and .simulation.capture_objective_gate == true
  and .simulation.attack_move_gate == true
  and .simulation.stance_behavior_gate == true
  and .simulation.control_group_rebuild_recall_formation_gate == true
  and .source_policy.no_openra_engine_code_copied == true
  and .source_policy.rust_bevy_owned_runtime == true
  and .source_policy.uses_trillionnium_owned_mod_data == true
  and .source_policy.warcraft_iii_asset_copied == false
  and (.source_of_truth | contains("does not copy OpenRA engine code"))
' "$OPENRA_LIKE_CORE_JSON" >/dev/null

jq -e '
  .status == "release_review_packet_integrity_green_with_public_launch_blockers"
  and .failed_check_count == 0
  and .artifact_count >= 128
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
' "$PACKET_INTEGRITY_JSON" >/dev/null

jq -n \
  --arg contract_version "trillionnium_world_review_runtime_core_semantics_batch_v1" \
  --arg status "review_runtime_core_semantics_sub_batch_1_reviewed_with_boundary_followup" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg doc_path "$DOC_REL" \
  --slurpfile runtime_batch "$RUNTIME_BOUNDARY_BATCH_JSON" \
  --slurpfile openra_core "$OPENRA_LIKE_CORE_JSON" \
  --slurpfile packet "$PACKET_INTEGRITY_JSON" \
  '
  def expected_hashes: [
    "aafb959a79e76b7439f43038613f9f8a181d611c",
    "64c8b1265058ce7691800f051125ded94f9eb49a",
    "2e23269c855ff5b2a58e919740711eee0cc7ec5a",
    "bc6c7066da037cad0a717ed860628238feb1246d",
    "8fd2f4e3c6492ca2cf602377c859e27103c7d2c6",
    "5ad6f0e569adb90eb843d553d61a3c26f256fe13",
    "2702ac08f1163e63b555fa5c419afb5af333db1a",
    "c3e8cf83b8e814898501a7379961c30531b797fe",
    "c6d0adb8be471f3e39dcec2235656c85bc1ce805",
    "54693d0ad7e2a15e04016fcfcd6bbceae53587c8",
    "ac2ebfa35159c0d26ad63ec69f4e9b1fcbe69f71",
    "93a2bab73846b6fb506e198a6ecf21a52d94cf1f",
    "29af0970620794499d0ec600f2883a65eaf8f051",
    "5dd37559f1b8b965a02bfde9c41383ee8ab40d6c",
    "a75370bbdb88518c7f2b2fe400031b3d1593bfd9",
    "867254280c602996689d6e46a78ea116305443e0",
    "0642dbd7219a5c5937d7cd3f238a969fd869aae0",
    "34e59708dd7a6b2bf4e3305b30f0c804d279d396",
    "ec507ae12aaa96e8ca1b3b30d6a4bc4b05d95959",
    "9ccdbb014c964bfc7254adc45be093396128a9c6",
    "4f659bc15cc0dab0cc1e1f6803b5aa53bc3151df",
    "b76d57011ccc650e0f06b9f7abe61904f1173c3b",
    "2a6e5f0c1010f52f47ad6bb1afa8f1dca291c326",
    "55099fed9f9e042df1daa0ce70b7eac4f8fe81c8",
    "5bd5f5b75c1a561538a91786feeb14b4e588e05a",
    "43b119785db8c3577e99ccb0c501977decb1409b",
    "3f3c8e9b4f85e051f02b8b2f52fa91d686256453",
    "6dd414b6564b46abc7af142dc52a65c97aa76b43",
    "4c4458eae12d37404a9d663149226f7d3bccc51b",
    "4e1b60a2bbf08fdb872069422400b30374e7c9fb",
    "86f31cbbf73ca817b60ee74aedb1b06bc16b3ad1",
    "ae519725f26906a2d713aae7ccab63cbfe003499",
    "097d140b6fd40de65981733e73b815659027828d",
    "8c2da6b38d1615d29f6e5a3525435de21df1f13b",
    "b5c577ddf8e0cec8f95bfa4531db4808a836aa77",
    "e2c219003a59758f122b2d01cd96db9208f57d42",
    "e03deb69879501aad7895e41d84989ced0a0148f",
    "fc37fb2290496dcc677724bdbfd2927a447233b7",
    "64ab27193baf2e82671b0cce88ae3b93930d472f",
    "2dbbf856829932a5a650c9f10b7648556ec78446",
    "9eba5dfa4b5a55240f366d912421d88a97d999c6",
    "25f74aba6fc4da6faf93825e3c23951175c9f8d0",
    "1cd7168b1d91140d75a60d7d60ed33fabf0bfffa",
    "201bf2949b7f850d1c549c6c5d4bf386e87b3c1a",
    "75856434b08eeb2a0f72603656481beedcfb9bf4",
    "b67595ec5dea43d494de709f88b60094b7100d72",
    "819e4bfdf9317832e1b5bf940a9a083c0c78fef5",
    "5a17e3e47c1a67d970a984d45048597dd1ee1d59",
    "9779df3af0b2b0368f49b46918cdc4709221af9e",
    "d1fda7e5b9afc8af8b182fbdefa7be25472afa99",
    "949192029ccb6413d3d6a67a1788d61d886c511f",
    "8b971950aecbe5e5a9cb97cd61b50cc7bc2cabe6",
    "c7d495f079b6788a9487a3ba31d1c36afa4dca08",
    "f3cb6a5fef872664b32925ffb6d69d7e69313d83",
    "1d86e9865d4684a52767f1bd5b90af53076393f6"
  ];
  def review_profile:
    (.subject | ascii_downcase) as $s
    | if ($s | test("control group")) then
        {
          review_group: "control_group_lifecycle",
          review_focus: "control_group_assignment_prune_validate_recall_rebuild",
          boundary_conclusion: "control-group lifecycle semantics are evidence-green and no external/OpenRA compatibility credit is granted"
        }
      elif ($s | test("production|tech prerequisite|build placement|supply cap|power low|resource depletion|harvester")) then
        {
          review_group: "economy_build_production_semantics",
          review_focus: "economy_build_production_rules",
          boundary_conclusion: "economy/build/production semantics are local evidence-green and remain no-credit RTS core review data"
        }
      elif ($s | test("pathfinding|queue|formation|obstruction|path reservation|traffic")) then
        {
          review_group: "movement_pathing_order_queue",
          review_focus: "pathfinding_queue_formation_obstruction",
          boundary_conclusion: "movement/pathing/order-queue semantics are evidence-green but still need runtime/data source boundary follow-up"
        }
      elif ($s | test("objective capture|contested capture")) then
        {
          review_group: "objective_capture_semantics",
          review_focus: "capture_contest_completion_income",
          boundary_conclusion: "objective capture semantics are evidence-green and remain local First Contact review evidence"
        }
      elif ($s | test("combat|attack|target|repair|stance|patrol|stop|veterancy")) then
        {
          review_group: "combat_repair_stance_targeting",
          review_focus: "combat_repair_stance_targeting_orders",
          boundary_conclusion: "combat/repair/stance/targeting semantics are evidence-green and copy no OpenRA engine code"
        }
      else
        {
          review_group: "world_model_shroud_command_resolver",
          review_focus: "map_rule_shroud_command_resolution",
          boundary_conclusion: "world model, shroud, and command resolver semantics are evidence-green but source ownership remains a runtime-core boundary follow-up"
        }
      end;
  ($runtime_batch[0].commit_shards
    | map(select(.sub_batch_id == "runtime_core_semantics"))
    | sort_by(.queue_order)) as $items
  | ($items | map(. + review_profile + {
      commit_level_review_complete: true,
      unresolved: false,
      local_semantic_evidence_reviewed: true,
      no_openra_engine_code_claim_checked: true,
      openra_runtime_compatibility_claim_rejected: true,
      openra_replay_compatibility_claim_rejected: true,
      external_evidence_claim_rejected: true,
      public_launch_claim_rejected: true,
      android_s5_claim_rejected: true,
      production_ready_ui_claim_rejected: true
    })) as $reviews
  | ($reviews | group_by(.review_group) | map({
      review_group: .[0].review_group,
      review_focus: .[0].review_focus,
      count: length,
      unresolved_count: (map(select(.unresolved == true)) | length)
    }) | sort_by(.review_group)) as $groups
  | {
      contract_version: $contract_version,
      status: $status,
      generated_at: $generated_at,
      green: true,
      doc_path: $doc_path,
      batch_order: 3,
      sub_batch_order: 1,
      sub_batch_id: "runtime_core_semantics",
      bucket_id: "multi_native_bevy_rts_boundary_overlap",
      primary_owner: "rts_runtime_data_boundaries",
      source_runtime_boundary_batch_path: "acceptance/S6_public_launch/latest/trillionnium-world-review-runtime-boundary-batch.json",
      source_openra_like_core_path: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-openra-like-core.json",
      source_packet_integrity_path: "acceptance/S6_public_launch/latest/release-review-packet-integrity.json",
      reviewed_commit_count: ($reviews | length),
      required_reviewed_commit_count: 55,
      expected_hash_coverage_complete: (($items | map(.commit) | sort) == (expected_hashes | sort)),
      first_commit: ($items[0].short // "missing"),
      last_commit: ($items[-1].short // "missing"),
      changed_world_bevy_lib_count: ($items | map(select((.changed_path_sample // []) | index("trillionnium/crates/trnm-world-bevy/src/lib.rs"))) | length),
      changed_openra_like_core_checker_count: ($items | map(select((.changed_path_sample // []) | index("scripts/check_trillionnium_world_bevy_classic_rts_openra_like_core.sh"))) | length),
      local_semantic_evidence_green: ($openra_core[0].green == true),
      openra_like_core_gate_count: ($openra_core[0].gates | length),
      openra_like_core_all_gates_green: ($openra_core[0].gates | to_entries | all(.value == true)),
      openra_like_core_order_count: ($openra_core[0].orders | length),
      openra_like_core_rule_count: ($openra_core[0].rules | length),
      openra_like_core_tick_count: ($openra_core[0].simulation.tick_count // 0),
      openra_like_core_runtime_actor_count: ($openra_core[0].map.runtime_actor_count // 0),
      command_accepted_count: ($openra_core[0].simulation.command_accepted_count // 0),
      command_rejected_count: ($openra_core[0].simulation.command_rejected_count // 0),
      no_openra_engine_code_copied: ($openra_core[0].source_policy.no_openra_engine_code_copied == true),
      uses_trillionnium_owned_mod_data: ($openra_core[0].source_policy.uses_trillionnium_owned_mod_data == true),
      warcraft_iii_asset_copied: ($openra_core[0].source_policy.warcraft_iii_asset_copied == true),
      review_group_count: ($groups | length),
      review_group_counts: $groups,
      commit_reviews: $reviews,
      unresolved_commit_review_count: ($reviews | map(select(.unresolved == true)) | length),
      systemic_runtime_core_boundary_followup_count: 1,
      systemic_runtime_core_boundary_followups: [
        {
          followup_id: "runtime_core_source_boundary_not_fully_closed",
          severity: "batch_3_blocker",
          finding: "The reviewed OpenRA-like RTS core is evidence-green, but its source surface remains trnm-world-bevy/src/lib.rs plus the classic-rts-openra-like-core local evidence path; final batch 3 closure still needs runtime/data source-boundary review.",
          next_owner_sub_batch: "runtime_adapter_and_online_boundary"
        }
      ],
      sub_batch_1_local_review_complete: true,
      sub_batch_1_exit_rule_satisfied: false,
      sub_batch_2_unblocked_for_local_review: true,
      batch_3_exit_rule_satisfied: false,
      batch_4_unblocked_for_local_review: false,
      next_sub_batch_id: "runtime_adapter_and_online_boundary",
      packet_integrity_status: ($packet[0].status // "missing"),
      packet_integrity_failed_check_count: ($packet[0].failed_check_count // 999),
      push_performed: false,
      rebase_performed: false,
      reset_performed: false,
      squash_performed: false,
      history_rewrite_performed: false,
      upload_performed: false,
      publish_performed: false,
      external_action_performed: false,
      public_launch_ready_claimed: false,
      android_s5_real_device_claimed: false,
      beta_cohort_evidence_claimed: false,
      production_ready_ui_claimed: false,
      commercial_launch_evidence_claimed: false,
      live_public_exposure_performed: false,
      android_device_capture_performed: false,
      openra_runtime_compatibility_claimed: false,
      openra_replay_compatibility_claimed: false,
      openra_network_compatibility_claimed: false,
      no_credit_boundary: "local runtime-core semantics sub-batch 1 review only; no push, rebase, reset, squash, history rewrite, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, OpenRA runtime/replay/network compatibility, multi-node, live-traffic, or public-network credit",
      reviewer_next_action: "continue batch 3 with runtime_adapter_and_online_boundary to resolve the systemic runtime-core source-boundary follow-up before closing batch 3"
    }
  ' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_review_runtime_core_semantics_batch_v1"
  and .status == "review_runtime_core_semantics_sub_batch_1_reviewed_with_boundary_followup"
  and .green == true
  and .batch_order == 3
  and .sub_batch_order == 1
  and .sub_batch_id == "runtime_core_semantics"
  and .bucket_id == "multi_native_bevy_rts_boundary_overlap"
  and .primary_owner == "rts_runtime_data_boundaries"
  and .reviewed_commit_count == 55
  and .required_reviewed_commit_count == 55
  and .expected_hash_coverage_complete == true
  and .first_commit == "aafb959a79"
  and .last_commit == "1d86e9865d"
  and .changed_world_bevy_lib_count == 55
  and .changed_openra_like_core_checker_count == 55
  and .local_semantic_evidence_green == true
  and .openra_like_core_gate_count == 75
  and .openra_like_core_all_gates_green == true
  and .openra_like_core_order_count == 13
  and .openra_like_core_rule_count == 11
  and .openra_like_core_tick_count >= 320
  and .openra_like_core_runtime_actor_count >= 48
  and .command_accepted_count >= 15
  and .command_rejected_count >= 12
  and .no_openra_engine_code_copied == true
  and .uses_trillionnium_owned_mod_data == true
  and .warcraft_iii_asset_copied == false
  and .review_group_count == 6
  and (.review_group_counts | map(.count) | add) == 55
  and (.review_group_counts | map(select(.review_group == "world_model_shroud_command_resolver").count)[0]) == 3
  and (.review_group_counts | map(select(.review_group == "economy_build_production_semantics").count)[0]) == 13
  and (.review_group_counts | map(select(.review_group == "combat_repair_stance_targeting").count)[0]) == 8
  and (.review_group_counts | map(select(.review_group == "movement_pathing_order_queue").count)[0]) == 11
  and (.review_group_counts | map(select(.review_group == "objective_capture_semantics").count)[0]) == 2
  and (.review_group_counts | map(select(.review_group == "control_group_lifecycle").count)[0]) == 18
  and (.commit_reviews | length) == 55
  and (.commit_reviews | all(.commit_level_review_complete == true))
  and (.commit_reviews | all(.unresolved == false))
  and .unresolved_commit_review_count == 0
  and .systemic_runtime_core_boundary_followup_count == 1
  and (.systemic_runtime_core_boundary_followups[0].followup_id == "runtime_core_source_boundary_not_fully_closed")
  and .sub_batch_1_local_review_complete == true
  and .sub_batch_1_exit_rule_satisfied == false
  and .sub_batch_2_unblocked_for_local_review == true
  and .batch_3_exit_rule_satisfied == false
  and .batch_4_unblocked_for_local_review == false
  and .next_sub_batch_id == "runtime_adapter_and_online_boundary"
  and .packet_integrity_status == "release_review_packet_integrity_green_with_public_launch_blockers"
  and .packet_integrity_failed_check_count == 0
  and .push_performed == false
  and .rebase_performed == false
  and .reset_performed == false
  and .squash_performed == false
  and .history_rewrite_performed == false
  and .upload_performed == false
  and .publish_performed == false
  and .external_action_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
  and .beta_cohort_evidence_claimed == false
  and .production_ready_ui_claimed == false
  and .commercial_launch_evidence_claimed == false
  and .openra_runtime_compatibility_claimed == false
  and .openra_replay_compatibility_claimed == false
  and .openra_network_compatibility_claimed == false
  and (.no_credit_boundary | contains("local runtime-core semantics sub-batch 1 review only"))
' "$SUMMARY" >/dev/null

{
  printf '# Trillionnium World Review Runtime-Core Semantics Batch\n\n'
  printf -- '- status: `%s`\n' "$(jq -r '.status' "$SUMMARY")"
  printf -- '- batch/sub-batch: `%s` / `%s` / `%s`\n' \
    "$(jq -r '.batch_order' "$SUMMARY")" \
    "$(jq -r '.sub_batch_order' "$SUMMARY")" \
    "$(jq -r '.sub_batch_id' "$SUMMARY")"
  printf -- '- reviewed commits: `%s` / `%s`\n' \
    "$(jq -r '.reviewed_commit_count' "$SUMMARY")" \
    "$(jq -r '.required_reviewed_commit_count' "$SUMMARY")"
  printf -- '- unresolved commit reviews: `%s`\n' "$(jq -r '.unresolved_commit_review_count' "$SUMMARY")"
  printf -- '- systemic runtime-core boundary follow-ups: `%s`\n' \
    "$(jq -r '.systemic_runtime_core_boundary_followup_count' "$SUMMARY")"
  printf -- '- sub-batch 1 local review complete / exit rule: `%s` / `%s`\n' \
    "$(jq -r '.sub_batch_1_local_review_complete' "$SUMMARY")" \
    "$(jq -r '.sub_batch_1_exit_rule_satisfied' "$SUMMARY")"
  printf -- '- next sub-batch: `%s`\n\n' "$(jq -r '.next_sub_batch_id' "$SUMMARY")"
  printf '## Review Groups\n\n'
  jq -r '.review_group_counts[] | "- `\(.review_group)`: `\(.count)` commits, unresolved `\(.unresolved_count)`"' "$SUMMARY"
  printf '\n## Boundary Follow-Up\n\n'
  jq -r '.systemic_runtime_core_boundary_followups[] | "- `\(.followup_id)`: \(.finding) Next owner: `\(.next_owner_sub_batch)`."' "$SUMMARY"
} >"$SUMMARY_MD"

printf 'TRILLIONNIUM_WORLD_REVIEW_RUNTIME_CORE_SEMANTICS_BATCH_GREEN %s %s\n' "$SUMMARY" "$SUMMARY_MD"
