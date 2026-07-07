#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_next_execution_plan.sh"
OBSERVATION_LOG_SCRIPT="$ROOT/scripts/check_trillionnium_world_first_contact_human_playtest_observation_log.sh"
RUNBOOK_SCRIPT="$ROOT/scripts/check_trillionnium_world_first_contact_human_playtest_runbook.sh"
EVIDENCE_VOLUME_SCRIPT="$ROOT/scripts/check_trillionnium_world_evidence_volume_curation.sh"
REVIEWER_HANDOFF_SCRIPT="$ROOT/scripts/check_trillionnium_world_reviewer_handoff_index.sh"
REVIEW_SLICE_SCRIPT="$ROOT/scripts/check_trillionnium_world_review_slice_strategy.sh"
DOC="$ROOT/docs/development/trillionnium-world-next-execution-plan-v1.md"
READABILITY_REVIEW_DOC="$ROOT/docs/development/trillionnium-world-first-contact-readability-review-2026-07-07.md"
PLAYTEST_OBSERVATION_LOG_DOC="$ROOT/docs/development/trillionnium-world-first-contact-human-playtest-observation-log-2026-07-07.md"
PLAYTEST_RUNBOOK_DOC="$ROOT/docs/development/trillionnium-world-first-contact-human-playtest-runbook-2026-07-07.md"
EVIDENCE_VOLUME_DOC="$ROOT/docs/development/trillionnium-world-evidence-volume-curation-2026-07-07.md"
REVIEWER_HANDOFF_DOC="$ROOT/docs/development/trillionnium-world-reviewer-handoff-index-2026-07-07.md"
REVIEW_SLICE_DOC="$ROOT/docs/development/trillionnium-world-review-slice-strategy-2026-07-07.md"

required_script_lines=(
  'trillionnium_world_next_execution_plan_v1'
  'release_review_packet_integrity_green_with_public_launch_blockers'
  'next_execution_plan_green_with_public_launch_blockers'
  'public_launch_blockers_preserved'
  'whole_screen_first_contact_readability'
  'trillionnium-world-first-contact-readability-review-2026-07-07.md'
  'readability_review'
  'trillionnium-world-first-contact-human-playtest-observation-log-2026-07-07.md'
  'human_playtest_observation'
  'first-contact-human-playtest-observation-log.json'
  'trillionnium-world-first-contact-human-playtest-runbook-2026-07-07.md'
  'human_playtest_runbook'
  'first-contact-human-playtest-runbook.json'
  'pre_human_playtest_runbook_ready'
  'runbook_prompts_bound'
  'confusion_triggers_bound'
  'recording_schema_bound'
  'trillionnium-world-evidence-volume-curation-2026-07-07.md'
  'evidence_volume_curation'
  'trillionnium-world-evidence-volume-curation.json'
  'evidence_volume_curation_ready'
  'deletion_performed'
  'archive_movement_performed'
  'trillionnium-world-reviewer-handoff-index-2026-07-07.md'
  'reviewer_handoff_index'
  'trillionnium-world-reviewer-handoff-index.json'
  'reviewer_handoff_index_green_with_public_launch_blockers'
  'representative_visual_count'
  'upload_performed'
  'publish_performed'
  'trillionnium-world-review-slice-strategy-2026-07-07.md'
  'review_slice_strategy'
  'trillionnium-world-review-slice-strategy.json'
  'review_slice_strategy_ready'
  'external_action_performed'
  '.review_slice_strategy.external_action_performed == false'
  '.human_playtest_runbook.prompts_bound == true'
  '.evidence_volume_curation.deletion_performed == false'
  '.reviewer_handoff_index.upload_performed == false'
  'ready_for_renderer_change_from_human_observation'
  'pre_human_playtest_observation_seed'
  'recorded_confusion_point_count == 0'
  'unrecorded_slot_count == 3'
  'human_playtest_evidence_claimed == false'
  'beta_cohort_evidence_claimed == false'
  'human_playtest_path'
  'bevy-classic-playtest-handoff-packet.human_playtest_task_path'
  'real_external_evidence_collection'
  'do not shrink already-gated micro cues without a fresh screenshot-visible issue'
)

required_doc_lines=(
  'Whole-screen First Contact readability review'
  'Public launch state: blocked until real external evidence exists.'
  'Android S5 real-device state: unclaimed until device evidence is collected.'
  'packet binding: `bevy-classic-playtest-handoff-packet`'
  'trillionnium-world-first-contact-readability-review-2026-07-07.md'
  'trillionnium-world-first-contact-human-playtest-observation-log-2026-07-07.md'
  'trillionnium-world-first-contact-human-playtest-runbook-2026-07-07.md'
  'trillionnium-world-evidence-volume-curation-2026-07-07.md'
  'trillionnium-world-reviewer-handoff-index-2026-07-07.md'
  'trillionnium-world-review-slice-strategy-2026-07-07.md'
  'Do not keep shrinking already-gated micro cues'
)

required_readability_review_lines=(
  'The central beacon fight is still the dominant whole-screen readability risk.'
  'Do a product-level silhouette and composition pass around the active center'
  'Use the five-step human playtest path to log the first three confusion points'
  'trillionnium-world-first-contact-human-playtest-runbook-2026-07-07.md'
  'Do not keep shaving already-gated micro cues without a fresh screenshot-visible'
)

required_playtest_observation_log_lines=(
  'Status: pre-human-playtest observation seed.'
  'Record the first three moments where the tester hesitates'
  '| 3 | `secure_beacon` |'
  '| 5 | `recover_blocked_route` |'
  'This log has three recorded human-observed confusion points'
  'trillionnium-world-first-contact-human-playtest-runbook-2026-07-07.md'
)

required_playtest_runbook_lines=(
  'Status: pre-human-playtest runbook.'
  'One observer, one local tester, one five-step path.'
  'Read only the fixed prompt for each task'
  '| 1 | `start_campaign` |'
  '| 5 | `recover_blocked_route` |'
  'Each recorded confusion point should include:'
  'ready_for_renderer_change_from_human_observation'
)

required_review_slice_lines=(
  'Status: local review-slice strategy.'
  'This is a grouping plan over existing local commits, not a history rewrite.'
  'Do not push, rebase, force-push, reset, squash, or delete commits'
  '| `release_truth_and_public_boundary` |'
  '| `native_bevy_playable_client` |'
  '| `first_contact_product_readability` |'
  '| `external_evidence_collection_blockers` |'
)

required_evidence_volume_lines=(
  'Status: local evidence-volume curation plan.'
  'Do not delete, compress, move, archive, rewrite, or prune acceptance evidence'
  'Preserve `acceptance/S5_native_bevy_device/latest` as the source of truth.'
  '| `reviewer_summary` |'
  '| `raw_visual_archive_candidate` |'
  '| `external_evidence_blockers` |'
)

required_reviewer_handoff_lines=(
  'Status: local reviewer handoff index.'
  'This is an index over existing local evidence, not a new evidence claim.'
  'Do not delete, compress, move, archive, rewrite, upload, or publish evidence'
  '| `reviewer_summary` |'
  '| `representative_visuals` |'
  '| `raw_visual_archive_candidates` |'
)

required_observation_log_script_lines=(
  'trillionnium_world_first_contact_human_playtest_observation_log_v1'
  'first-contact-human-playtest-observation-log.json'
  'recorded_confusion_point_count'
  'unrecorded_slot_count'
  'ready_for_renderer_change_from_human_observation'
  'human_playtest_evidence_claimed == false'
  'beta_cohort_evidence_claimed == false'
  'public_launch_ready_claimed == false'
  'android_s5_real_device_claimed == false'
  'TRILLIONNIUM_WORLD_FIRST_CONTACT_HUMAN_PLAYTEST_OBSERVATION_LOG_GREEN'
)

required_runbook_script_lines=(
  'trillionnium_world_first_contact_human_playtest_runbook_v1'
  'first-contact-human-playtest-runbook.json'
  'pre_human_playtest_runbook_ready'
  'runbook_prompts_bound'
  'confusion_triggers_bound'
  'recording_schema_bound'
  'human_playtest_completion_claimed'
  'public_launch_ready_claimed == false'
  'android_s5_real_device_claimed == false'
  'TRILLIONNIUM_WORLD_FIRST_CONTACT_HUMAN_PLAYTEST_RUNBOOK_GREEN'
)

required_evidence_volume_script_lines=(
  'trillionnium_world_evidence_volume_curation_v1'
  'trillionnium-world-evidence-volume-curation.json'
  'evidence_volume_curation_ready'
  'large_file_count > 100'
  'deletion_performed == false'
  'compression_performed == false'
  'archive_movement_performed == false'
  'public_launch_ready_claimed == false'
  'android_s5_real_device_claimed == false'
  'TRILLIONNIUM_WORLD_EVIDENCE_VOLUME_CURATION_GREEN'
)

required_reviewer_handoff_script_lines=(
  'trillionnium_world_reviewer_handoff_index_v1'
  'trillionnium-world-reviewer-handoff-index.json'
  'reviewer_handoff_index_green_with_public_launch_blockers'
  'artifact_count == 24'
  'representative_visual_count == 5'
  'raw_visual_archive_candidate_count == 6'
  'upload_performed == false'
  'publish_performed == false'
  'public_launch_ready == false'
  'android_s5_real_device_claimed == false'
  'TRILLIONNIUM_WORLD_REVIEWER_HANDOFF_INDEX_GREEN_WITH_PUBLIC_LAUNCH_BLOCKERS'
)

required_review_slice_script_lines=(
  'trillionnium_world_review_slice_strategy_v1'
  'trillionnium-world-review-slice-strategy.json'
  'review_slice_strategy_ready'
  'review_slice_count == 6'
  'external_action_performed == false'
  'push_performed == false'
  'rebase_performed == false'
  'reset_performed == false'
  'public_launch_ready_claimed == false'
  'android_s5_real_device_claimed == false'
  'TRILLIONNIUM_WORLD_REVIEW_SLICE_STRATEGY_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] next execution plan script missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_doc_lines[@]}"; do
  if ! grep -Fq -- "$line" "$DOC"; then
    echo "[FAIL] next execution plan doc missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_readability_review_lines[@]}"; do
  if ! grep -Fq -- "$line" "$READABILITY_REVIEW_DOC"; then
    echo "[FAIL] readability review doc missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_playtest_observation_log_lines[@]}"; do
  if ! grep -Fq -- "$line" "$PLAYTEST_OBSERVATION_LOG_DOC"; then
    echo "[FAIL] playtest observation log missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_playtest_runbook_lines[@]}"; do
  if ! grep -Fq -- "$line" "$PLAYTEST_RUNBOOK_DOC"; then
    echo "[FAIL] playtest runbook missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_review_slice_lines[@]}"; do
  if ! grep -Fq -- "$line" "$REVIEW_SLICE_DOC"; then
    echo "[FAIL] review slice strategy missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_evidence_volume_lines[@]}"; do
  if ! grep -Fq -- "$line" "$EVIDENCE_VOLUME_DOC"; then
    echo "[FAIL] evidence volume curation missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_reviewer_handoff_lines[@]}"; do
  if ! grep -Fq -- "$line" "$REVIEWER_HANDOFF_DOC"; then
    echo "[FAIL] reviewer handoff index missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_observation_log_script_lines[@]}"; do
  if ! grep -Fq -- "$line" "$OBSERVATION_LOG_SCRIPT"; then
    echo "[FAIL] playtest observation log script missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_runbook_script_lines[@]}"; do
  if ! grep -Fq -- "$line" "$RUNBOOK_SCRIPT"; then
    echo "[FAIL] playtest runbook script missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_review_slice_script_lines[@]}"; do
  if ! grep -Fq -- "$line" "$REVIEW_SLICE_SCRIPT"; then
    echo "[FAIL] review slice strategy script missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_evidence_volume_script_lines[@]}"; do
  if ! grep -Fq -- "$line" "$EVIDENCE_VOLUME_SCRIPT"; then
    echo "[FAIL] evidence volume curation script missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_reviewer_handoff_script_lines[@]}"; do
  if ! grep -Fq -- "$line" "$REVIEWER_HANDOFF_SCRIPT"; then
    echo "[FAIL] reviewer handoff index script missing contract line: $line" >&2
    exit 1
  fi
done

echo "[PASS] next execution plan script/doc keep whole-screen product direction, public-launch blockers, and micro-cue restraint"
