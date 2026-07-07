#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
DOC_REL="docs/development/trillionnium-world-first-contact-human-playtest-observation-log-2026-07-07.md"
DOC="$ROOT/$DOC_REL"
SUMMARY="$ACCEPTANCE_DIR/first-contact-human-playtest-observation-log.json"
SUMMARY_MD="$ACCEPTANCE_DIR/first-contact-human-playtest-observation-log.md"
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
  echo "[FAIL] missing observation log: $DOC" >&2
  exit 1
fi

require_text "$DOC" "Status: pre-human-playtest observation seed."
require_text "$DOC" "This file is not beta evidence, public-launch evidence, Android S5 real-device"
require_text "$DOC" "Record the first three moments where the tester hesitates"
require_text "$DOC" '| 1 | `start_campaign` |'
require_text "$DOC" '| 2 | `select_units` |'
require_text "$DOC" '| 3 | `secure_beacon` |'
require_text "$DOC" '| 4 | `read_command_queue` |'
require_text "$DOC" '| 5 | `recover_blocked_route` |'
require_text "$DOC" "1. \`unrecorded\`:"
require_text "$DOC" "2. \`unrecorded\`:"
require_text "$DOC" "3. \`unrecorded\`:"

unrecorded_slot_count="$(grep -Ec '^[0-9]+\. `unrecorded`:' "$DOC")"
recorded_confusion_point_count="$((3 - unrecorded_slot_count))"

jq -n \
  --arg contract_version "trillionnium_world_first_contact_human_playtest_observation_log_v1" \
  --arg status "pre_human_playtest_observation_seed" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg doc_path "$DOC_REL" \
  --argjson unrecorded_slot_count "$unrecorded_slot_count" \
  --argjson recorded_confusion_point_count "$recorded_confusion_point_count" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    green: true,
    doc_path: $doc_path,
    task_ids: ["start_campaign", "select_units", "secure_beacon", "read_command_queue", "recover_blocked_route"],
    task_count: 5,
    required_confusion_point_count: 3,
    recorded_confusion_point_count: $recorded_confusion_point_count,
    unrecorded_slot_count: $unrecorded_slot_count,
    first_three_confusion_points_recorded: ($recorded_confusion_point_count == 3),
    ready_for_renderer_change_from_human_observation: ($recorded_confusion_point_count == 3),
    human_playtest_evidence_claimed: false,
    beta_cohort_evidence_claimed: false,
    public_launch_ready_claimed: false,
    android_s5_real_device_claimed: false,
    production_ready_ui_claimed: false,
    commercial_launch_evidence_claimed: false,
    no_credit_boundary: "desk-review seed only; not beta, public launch, Android S5 real-device, production-ready UI, commercial launch, or human tester completion evidence",
    source_of_truth: "The First Contact observation log is a local pre-human-playtest checklist until three observer-recorded confusion points replace the unrecorded slots."
  }' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_first_contact_human_playtest_observation_log_v1"
  and .status == "pre_human_playtest_observation_seed"
  and .green == true
  and .task_count == 5
  and .required_confusion_point_count == 3
  and .recorded_confusion_point_count == 0
  and .unrecorded_slot_count == 3
  and .first_three_confusion_points_recorded == false
  and .ready_for_renderer_change_from_human_observation == false
  and .human_playtest_evidence_claimed == false
  and .beta_cohort_evidence_claimed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
  and .production_ready_ui_claimed == false
  and .commercial_launch_evidence_claimed == false
  and (.no_credit_boundary | contains("not beta"))
  and (.source_of_truth | contains("pre-human-playtest checklist"))
' "$SUMMARY" >/dev/null

{
  printf '# First Contact Human Playtest Observation Log\n\n'
  printf -- '- status: `%s`\n' "$(jq -r '.status' "$SUMMARY")"
  printf -- '- task count: `%s`\n' "$(jq -r '.task_count' "$SUMMARY")"
  printf -- '- recorded confusion points: `%s` / `%s`\n' \
    "$(jq -r '.recorded_confusion_point_count' "$SUMMARY")" \
    "$(jq -r '.required_confusion_point_count' "$SUMMARY")"
  printf -- '- ready for renderer change from human observation: `%s`\n' \
    "$(jq -r '.ready_for_renderer_change_from_human_observation' "$SUMMARY")"
  printf -- '- public launch ready claimed: `%s`\n' "$(jq -r '.public_launch_ready_claimed' "$SUMMARY")"
  printf -- '- Android S5 real-device claimed: `%s`\n' "$(jq -r '.android_s5_real_device_claimed' "$SUMMARY")"
} >"$SUMMARY_MD"

printf 'TRILLIONNIUM_WORLD_FIRST_CONTACT_HUMAN_PLAYTEST_OBSERVATION_LOG_GREEN %s %s\n' "$SUMMARY" "$SUMMARY_MD"
