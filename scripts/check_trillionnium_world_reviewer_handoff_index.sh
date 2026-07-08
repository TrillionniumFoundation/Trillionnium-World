#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
DOC_REL="docs/development/trillionnium-world-reviewer-handoff-index-2026-07-07.md"
DOC="$ROOT/$DOC_REL"
SUMMARY="$ACCEPTANCE_DIR/trillionnium-world-reviewer-handoff-index.json"
SUMMARY_MD="$ACCEPTANCE_DIR/trillionnium-world-reviewer-handoff-index.md"
mkdir -p "$ACCEPTANCE_DIR"

require_text() {
  local path="$1"
  local needle="$2"
  if ! grep -Fq -- "$needle" "$path"; then
    echo "[FAIL] $path missing required text: $needle" >&2
    exit 1
  fi
}

artifact_json() {
  local label="$1"
  local section="$2"
  local path_rel="$3"
  local path="$ROOT/$path_rel"
  if [[ ! -f "$path" ]]; then
    echo "[FAIL] missing handoff index artifact: $path_rel" >&2
    exit 1
  fi
  jq -n \
    --arg label "$label" \
    --arg section "$section" \
    --arg path "$path_rel" \
    --arg sha256 "$(sha256sum "$path" | awk '{print $1}')" \
    --argjson bytes "$(stat -c '%s' "$path")" \
    '{label: $label, section: $section, path: $path, sha256: $sha256, bytes: $bytes}'
}

if [[ ! -f "$DOC" ]]; then
  echo "[FAIL] missing reviewer handoff index doc: $DOC" >&2
  exit 1
fi

require_text "$DOC" "Status: local reviewer handoff index."
require_text "$DOC" "This is an index over existing local evidence, not a new evidence claim."
require_text "$DOC" "Do not delete, compress, move, archive, rewrite, upload, or publish evidence"
require_text "$DOC" "Review-slice manifest"
require_text "$DOC" "Review triage queue"
require_text "$DOC" "Review primary-owner plan"
require_text "$DOC" "Review release-owner queue"
require_text "$DOC" "Review runtime-owner queue"
require_text "$DOC" "Public-launch blocker execution ledger"
require_text "$DOC" '| `reviewer_summary` |'
require_text "$DOC" '| `live_player_screen` |'
require_text "$DOC" '| `representative_visuals` |'
require_text "$DOC" '| `raw_visual_archive_candidates` |'

"$ROOT/scripts/check_trillionnium_world_evidence_volume_curation.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_review_slice_strategy.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_review_slice_manifest.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_review_triage_queue.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_review_primary_owner_plan.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_review_release_owner_queue.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_review_runtime_owner_queue.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_first_contact_human_playtest_runbook.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_public_launch_blocker_execution_ledger.sh" >/dev/null

CURATION_JSON="$ACCEPTANCE_DIR/trillionnium-world-evidence-volume-curation.json"
REVIEW_SLICE_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-slice-strategy.json"
REVIEW_SLICE_MANIFEST_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-slice-manifest.json"
REVIEW_TRIAGE_QUEUE_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-triage-queue.json"
REVIEW_PRIMARY_OWNER_PLAN_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-primary-owner-plan.json"
REVIEW_RELEASE_OWNER_QUEUE_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-release-owner-queue.json"
REVIEW_RUNTIME_OWNER_QUEUE_JSON="$ACCEPTANCE_DIR/trillionnium-world-review-runtime-owner-queue.json"
RUNBOOK_JSON="$ACCEPTANCE_DIR/first-contact-human-playtest-runbook.json"
OBSERVATION_JSON="$ACCEPTANCE_DIR/first-contact-human-playtest-observation-log.json"
PACKET_JSON="$ACCEPTANCE_DIR/release-review-packet-integrity.json"
PUBLIC_LAUNCH_JSON="$ACCEPTANCE_DIR/public-launch-readiness.json"
BLOCKER_LEDGER_JSON="$ACCEPTANCE_DIR/trillionnium-world-public-launch-blocker-execution-ledger.json"

jq -e '
  .contract_version == "trillionnium_world_evidence_volume_curation_v1"
  and .deletion_performed == false
  and .compression_performed == false
  and .archive_movement_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$CURATION_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_review_slice_strategy_v1"
  and .external_action_performed == false
  and .push_performed == false
  and .rebase_performed == false
  and .reset_performed == false
' "$REVIEW_SLICE_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_review_slice_manifest_v1"
  and .status == "review_slice_manifest_ready"
  and .review_slice_count == 6
  and .total_ahead_count >= 1
  and ((.manifested_commit_count + .unclassified_commit_count) == .total_ahead_count)
  and .push_performed == false
  and .rebase_performed == false
  and .reset_performed == false
  and .squash_performed == false
  and .history_rewrite_performed == false
  and .external_action_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$REVIEW_SLICE_MANIFEST_JSON" >/dev/null

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
' "$REVIEW_TRIAGE_QUEUE_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_review_primary_owner_plan_v1"
  and .status == "review_primary_owner_plan_ready"
  and .owner_bucket_count == 11
  and .bucket_primary_owner_assigned_count == 11
  and .bucket_primary_owner_assignment_complete == true
  and .commit_level_primary_owner_review_required_count >= 1
  and .review_order_complete == true
  and .push_performed == false
  and .rebase_performed == false
  and .reset_performed == false
  and .squash_performed == false
  and .history_rewrite_performed == false
  and .external_action_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$REVIEW_PRIMARY_OWNER_PLAN_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_review_release_owner_queue_v1"
  and .status == "review_release_owner_queue_ready"
  and .primary_owner == "release_truth_and_public_boundary"
  and .lane_bucket_count == 4
  and .queue_matches_owner_plan == true
  and .bucket_coverage_complete == true
  and .push_performed == false
  and .rebase_performed == false
  and .reset_performed == false
  and .squash_performed == false
  and .history_rewrite_performed == false
  and .external_action_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$REVIEW_RELEASE_OWNER_QUEUE_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_review_runtime_owner_queue_v1"
  and .status == "review_runtime_owner_queue_ready"
  and .primary_owner == "rts_runtime_data_boundaries"
  and .lane_bucket_count == 3
  and .queue_matches_owner_plan == true
  and .bucket_coverage_complete == true
  and .commit_level_primary_owner_review_required_count >= 1
  and .runtime_boundary_review_item_count >= 1
  and .push_performed == false
  and .rebase_performed == false
  and .reset_performed == false
  and .squash_performed == false
  and .history_rewrite_performed == false
  and .external_action_performed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$REVIEW_RUNTIME_OWNER_QUEUE_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_first_contact_human_playtest_runbook_v1"
  and .human_playtest_completion_claimed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$RUNBOOK_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_first_contact_human_playtest_observation_log_v1"
  and .recorded_confusion_point_count == 0
  and .ready_for_renderer_change_from_human_observation == false
  and .human_playtest_evidence_claimed == false
' "$OBSERVATION_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_public_launch_blocker_execution_ledger_v1"
  and .status == "public_launch_blocker_execution_ledger_ready_for_real_evidence_collection"
  and .needs_collection_count == 6
  and .green_evidence_item_count == 0
  and .blocker_consistency_failed_check_count == 0
  and .live_public_exposure_performed == false
  and .android_device_capture_performed == false
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
' "$BLOCKER_LEDGER_JSON" >/dev/null

TRNM_RELEASE_REVIEW_PACKET_REFRESH_INPUTS=0 \
TRNM_RELEASE_REVIEW_PACKET_USE_RELEASE_ARTIFACT_BIN=0 \
  "$ROOT/scripts/check_trillionnium_world_release_review_packet.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity.sh" --no-refresh >/dev/null

jq -e '
  .status == "release_review_packet_integrity_green_with_public_launch_blockers"
  and .green == true
  and .failed_check_count == 0
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
' "$PACKET_JSON" >/dev/null

jq -e '
  .public_launch_ready == false
  and .android_s5_real_device_claimed == false
  and ((.known_public_launch_blockers // []) | length) == 6
' "$PUBLIC_LAUNCH_JSON" >/dev/null

ARTIFACTS_JSON="$(
  {
    artifact_json release_packet_integrity reviewer_summary "acceptance/S6_public_launch/latest/release-review-packet-integrity.json"
    artifact_json release_review_status_markdown reviewer_summary "acceptance/S6_public_launch/latest/release-review-status.md"
    artifact_json evidence_volume_curation reviewer_summary "acceptance/S6_public_launch/latest/trillionnium-world-evidence-volume-curation.json"
    artifact_json evidence_volume_curation_markdown reviewer_summary "acceptance/S6_public_launch/latest/trillionnium-world-evidence-volume-curation.md"
    artifact_json review_slice_strategy reviewer_summary "acceptance/S6_public_launch/latest/trillionnium-world-review-slice-strategy.json"
    artifact_json review_slice_manifest reviewer_summary "acceptance/S6_public_launch/latest/trillionnium-world-review-slice-manifest.json"
    artifact_json review_triage_queue reviewer_summary "acceptance/S6_public_launch/latest/trillionnium-world-review-triage-queue.json"
    artifact_json review_primary_owner_plan reviewer_summary "acceptance/S6_public_launch/latest/trillionnium-world-review-primary-owner-plan.json"
    artifact_json review_release_owner_queue reviewer_summary "acceptance/S6_public_launch/latest/trillionnium-world-review-release-owner-queue.json"
    artifact_json review_runtime_owner_queue reviewer_summary "acceptance/S6_public_launch/latest/trillionnium-world-review-runtime-owner-queue.json"
    artifact_json public_launch_blocker_execution_ledger reviewer_summary "acceptance/S6_public_launch/latest/trillionnium-world-public-launch-blocker-execution-ledger.json"
    artifact_json human_playtest_observation reviewer_summary "acceptance/S6_public_launch/latest/first-contact-human-playtest-observation-log.json"
    artifact_json human_playtest_runbook reviewer_summary "acceptance/S6_public_launch/latest/first-contact-human-playtest-runbook.json"
    artifact_json public_launch_readiness reviewer_summary "acceptance/S6_public_launch/latest/public-launch-readiness.json"
    artifact_json playtest_handoff_packet reviewer_summary "acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-handoff-packet.json"
    artifact_json playtest_handoff_packet_markdown reviewer_summary "acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-handoff-packet.md"
    artifact_json runner_status live_player_screen "acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-runner-status.json"
    artifact_json runner_probe live_player_screen "acceptance/S5_native_bevy_device/latest/manual_bevy/bevy-classic-player-screen-runner-status-probe.json"
    artifact_json live_player_screen_png live_player_screen "acceptance/S5_native_bevy_device/latest/manual_bevy/bevy-classic-player-screen-runner-status.png"
    artifact_json full_game_visual_ui_png representative_visuals "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-full-game-visual-ui-replication.png"
    artifact_json full_screen_ui_png representative_visuals "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-full-screen-ui-replication.png"
    artifact_json shell_meta_ui_png representative_visuals "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-shell-meta-ui-replication.png"
    artifact_json match_setup_ui_png representative_visuals "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-match-setup-ui-replication.png"
    artifact_json in_match_hud_state_png representative_visuals "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-in-match-hud-state-replication.png"
    artifact_json central_keep_breakthrough_ppm raw_visual_archive_candidates "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-central-keep-breakthrough.ppm"
    artifact_json central_keep_pressure_ppm raw_visual_archive_candidates "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-central-keep-pressure.ppm"
    artifact_json inner_lane_breakthrough_ppm raw_visual_archive_candidates "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-inner-lane-breakthrough.ppm"
    artifact_json siege_breach_counterplay_ppm raw_visual_archive_candidates "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-siege-breach-counterplay.ppm"
    artifact_json tier_two_siege_push_ppm raw_visual_archive_candidates "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-tier-two-siege-push.ppm"
    artifact_json expansion_counterattack_ppm raw_visual_archive_candidates "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-expansion-counterattack.ppm"
  } | jq -s .
)"

s5_latest_kib="$(jq -r '.s5_latest_kib' "$CURATION_JSON")"
large_file_count="$(jq -r '.large_file_count' "$CURATION_JSON")"
public_launch_ready="$(jq -r '.public_launch_ready // false' "$PUBLIC_LAUNCH_JSON")"
android_s5_real_device_claimed="$(jq -r '.android_s5_real_device_claimed // false' "$PUBLIC_LAUNCH_JSON")"
blocker_count="$(jq -r '(.known_public_launch_blockers // []) | length' "$PUBLIC_LAUNCH_JSON")"

jq -n \
  --arg contract_version "trillionnium_world_reviewer_handoff_index_v1" \
  --arg status "reviewer_handoff_index_green_with_public_launch_blockers" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg doc_path "$DOC_REL" \
  --argjson artifacts "$ARTIFACTS_JSON" \
  --argjson s5_latest_kib "$s5_latest_kib" \
  --argjson large_file_count "$large_file_count" \
  --argjson public_launch_ready "$public_launch_ready" \
  --argjson android_s5_real_device_claimed "$android_s5_real_device_claimed" \
  --argjson blocker_count "$blocker_count" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    green: true,
    doc_path: $doc_path,
    s5_latest_kib: $s5_latest_kib,
    large_file_count: $large_file_count,
    artifacts: $artifacts,
    artifact_count: ($artifacts | length),
    reviewer_summary_count: ([ $artifacts[] | select(.section == "reviewer_summary") ] | length),
    live_player_screen_count: ([ $artifacts[] | select(.section == "live_player_screen") ] | length),
    representative_visual_count: ([ $artifacts[] | select(.section == "representative_visuals") ] | length),
    raw_visual_archive_candidate_count: ([ $artifacts[] | select(.section == "raw_visual_archive_candidates") ] | length),
    artifact_bytes_total: ([ $artifacts[].bytes ] | add),
    raw_visual_archive_candidate_bytes_total: ([ $artifacts[] | select(.section == "raw_visual_archive_candidates") | .bytes ] | add),
    all_sha256_valid: ($artifacts | all((.sha256 | test("^[0-9a-f]{64}$")) and (.bytes > 0))),
    source_evidence_preserved: true,
    deletion_performed: false,
    compression_performed: false,
    archive_movement_performed: false,
    upload_performed: false,
    publish_performed: false,
    public_launch_ready: $public_launch_ready,
    android_s5_real_device_claimed: $android_s5_real_device_claimed,
    public_launch_blocker_count: $blocker_count,
    beta_cohort_evidence_claimed: false,
    production_ready_ui_claimed: false,
    commercial_launch_evidence_claimed: false,
    no_credit_boundary: "local reviewer handoff index only; no delete, compress, archive, upload, publish, public launch, Android S5 real-device, beta, production-ready UI, commercial, multi-node, or public-network credit",
    reviewer_next_action: "start_with_reviewer_summary_then_inspect_live_player_screen_and_representative_visuals_before_deep_raw_archive_audit"
  }' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_reviewer_handoff_index_v1"
  and .status == "reviewer_handoff_index_green_with_public_launch_blockers"
  and .green == true
  and .artifact_count == 30
  and .reviewer_summary_count == 16
  and .live_player_screen_count == 3
  and .representative_visual_count == 5
  and .raw_visual_archive_candidate_count == 6
  and .all_sha256_valid == true
  and .s5_latest_kib > 10000000
  and .large_file_count > 100
  and .source_evidence_preserved == true
  and .deletion_performed == false
  and .compression_performed == false
  and .archive_movement_performed == false
  and .upload_performed == false
  and .publish_performed == false
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
  and .public_launch_blocker_count == 6
  and .beta_cohort_evidence_claimed == false
  and .production_ready_ui_claimed == false
  and .commercial_launch_evidence_claimed == false
  and (.no_credit_boundary | contains("local reviewer handoff index only"))
' "$SUMMARY" >/dev/null

{
  printf '# Trillionnium World Reviewer Handoff Index\n\n'
  printf -- '- status: `%s`\n' "$(jq -r '.status' "$SUMMARY")"
  printf -- '- artifacts indexed: `%s`\n' "$(jq -r '.artifact_count' "$SUMMARY")"
  printf -- '- reviewer summary / live / representative / raw: `%s` / `%s` / `%s` / `%s`\n' \
    "$(jq -r '.reviewer_summary_count' "$SUMMARY")" \
    "$(jq -r '.live_player_screen_count' "$SUMMARY")" \
    "$(jq -r '.representative_visual_count' "$SUMMARY")" \
    "$(jq -r '.raw_visual_archive_candidate_count' "$SUMMARY")"
  printf -- '- S5 latest KiB: `%s`, large files: `%s`\n' \
    "$(jq -r '.s5_latest_kib' "$SUMMARY")" \
    "$(jq -r '.large_file_count' "$SUMMARY")"
  printf -- '- deletion/compression/archive/upload/publish: `%s` / `%s` / `%s` / `%s` / `%s`\n' \
    "$(jq -r '.deletion_performed' "$SUMMARY")" \
    "$(jq -r '.compression_performed' "$SUMMARY")" \
    "$(jq -r '.archive_movement_performed' "$SUMMARY")" \
    "$(jq -r '.upload_performed' "$SUMMARY")" \
    "$(jq -r '.publish_performed' "$SUMMARY")"
  printf -- '- public launch ready: `%s`\n' "$(jq -r '.public_launch_ready' "$SUMMARY")"
  printf -- '- Android S5 real-device claimed: `%s`\n\n' "$(jq -r '.android_s5_real_device_claimed' "$SUMMARY")"
  printf '## Indexed Artifacts\n\n'
  jq -r '.artifacts[] | "- `\(.section)` / `\(.label)`: `\(.path)` bytes `\(.bytes)` sha256 `\(.sha256)`"' "$SUMMARY"
} >"$SUMMARY_MD"

printf 'TRILLIONNIUM_WORLD_REVIEWER_HANDOFF_INDEX_GREEN_WITH_PUBLIC_LAUNCH_BLOCKERS %s %s\n' "$SUMMARY" "$SUMMARY_MD"
