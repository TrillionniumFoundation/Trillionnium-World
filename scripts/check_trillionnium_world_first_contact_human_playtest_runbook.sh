#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
DOC_REL="docs/development/trillionnium-world-first-contact-human-playtest-runbook-2026-07-07.md"
DOC="$ROOT/$DOC_REL"
HANDOFF_PACKET="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-handoff-packet.json"
OBSERVATION_LOG_JSON="$ACCEPTANCE_DIR/first-contact-human-playtest-observation-log.json"
SUMMARY="$ACCEPTANCE_DIR/first-contact-human-playtest-runbook.json"
SUMMARY_MD="$ACCEPTANCE_DIR/first-contact-human-playtest-runbook.md"
mkdir -p "$ACCEPTANCE_DIR"

require_file() {
  local path="$1"
  if [[ ! -f "$path" ]]; then
    echo "[FAIL] missing required file: $path" >&2
    exit 1
  fi
}

require_text() {
  local path="$1"
  local needle="$2"
  if ! grep -Fq -- "$needle" "$path"; then
    echo "[FAIL] $path missing required text: $needle" >&2
    exit 1
  fi
}

require_file "$DOC"
require_file "$HANDOFF_PACKET"

"$ROOT/scripts/check_trillionnium_world_first_contact_human_playtest_observation_log.sh" >/dev/null
require_file "$OBSERVATION_LOG_JSON"

require_text "$DOC" "Status: pre-human-playtest runbook."
require_text "$DOC" "This file is not beta evidence, public-launch evidence, Android S5 real-device"
require_text "$DOC" "One observer, one local tester, one five-step path."
require_text "$DOC" "Read only the fixed prompt for each task"
require_text "$DOC" "Stop after the first three confusion points are recorded."
require_text "$DOC" "| 1 | \`start_campaign\` |"
require_text "$DOC" "| 2 | \`select_units\` |"
require_text "$DOC" "| 3 | \`secure_beacon\` |"
require_text "$DOC" "| 4 | \`read_command_queue\` |"
require_text "$DOC" "| 5 | \`recover_blocked_route\` |"
require_text "$DOC" "Each recorded confusion point should include:"
require_text "$DOC" "\`ready_for_renderer_change_from_human_observation\`"

task_ids_json="$(jq -c '[.human_playtest_task_path[].id]' "$HANDOFF_PACKET")"
observation_task_ids_json="$(jq -c '.task_ids' "$OBSERVATION_LOG_JSON")"

jq -e \
  --argjson task_ids "$task_ids_json" \
  --argjson observation_task_ids "$observation_task_ids_json" '
  .contract_version == "trillionnium_world_bevy_classic_playtest_handoff_packet_v1"
  and .green == true
  and $task_ids == ["start_campaign", "select_units", "secure_beacon", "read_command_queue", "recover_blocked_route"]
  and $observation_task_ids == $task_ids
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
  and .human_playtest_task_path_public_launch_credit_claimed == false
' "$HANDOFF_PACKET" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_first_contact_human_playtest_observation_log_v1"
  and .status == "pre_human_playtest_observation_seed"
  and .recorded_confusion_point_count == 0
  and .unrecorded_slot_count == 3
  and .ready_for_renderer_change_from_human_observation == false
  and .human_playtest_evidence_claimed == false
  and .beta_cohort_evidence_claimed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
' "$OBSERVATION_LOG_JSON" >/dev/null

jq -n \
  --arg contract_version "trillionnium_world_first_contact_human_playtest_runbook_v1" \
  --arg status "pre_human_playtest_runbook_ready" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg doc_path "$DOC_REL" \
  --arg handoff_packet_path "acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-handoff-packet.json" \
  --arg observation_log_artifact_path "acceptance/S6_public_launch/latest/first-contact-human-playtest-observation-log.json" \
  --argjson task_ids "$task_ids_json" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    green: true,
    doc_path: $doc_path,
    handoff_packet_path: $handoff_packet_path,
    observation_log_artifact_path: $observation_log_artifact_path,
    task_ids: $task_ids,
    task_count: ($task_ids | length),
    observer_count_required: 1,
    tester_count_required: 1,
    required_confusion_point_count: 3,
    runbook_prompts_bound: true,
    pass_signals_bound: true,
    confusion_triggers_bound: true,
    recording_schema_bound: true,
    observation_log_required_before_renderer_change: true,
    ready_for_renderer_change_from_human_observation: false,
    human_playtest_completion_claimed: false,
    beta_cohort_evidence_claimed: false,
    public_launch_ready_claimed: false,
    android_s5_real_device_claimed: false,
    production_ready_ui_claimed: false,
    commercial_launch_evidence_claimed: false,
    no_credit_boundary: "runbook only; not beta, public launch, Android S5 real-device, production-ready UI, commercial launch, or human tester completion evidence",
    source_of_truth: "The runbook makes the local First Contact playtest repeatable, but renderer readiness still requires three recorded observer confusion points in the observation log."
  }' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_first_contact_human_playtest_runbook_v1"
  and .status == "pre_human_playtest_runbook_ready"
  and .green == true
  and .task_count == 5
  and .observer_count_required == 1
  and .tester_count_required == 1
  and .required_confusion_point_count == 3
  and .task_ids == ["start_campaign", "select_units", "secure_beacon", "read_command_queue", "recover_blocked_route"]
  and .runbook_prompts_bound == true
  and .pass_signals_bound == true
  and .confusion_triggers_bound == true
  and .recording_schema_bound == true
  and .observation_log_required_before_renderer_change == true
  and .ready_for_renderer_change_from_human_observation == false
  and .human_playtest_completion_claimed == false
  and .beta_cohort_evidence_claimed == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
  and .production_ready_ui_claimed == false
  and .commercial_launch_evidence_claimed == false
  and (.no_credit_boundary | contains("runbook only"))
  and (.source_of_truth | contains("three recorded observer confusion points"))
' "$SUMMARY" >/dev/null

{
  printf '# First Contact Human Playtest Runbook\n\n'
  printf -- '- status: `%s`\n' "$(jq -r '.status' "$SUMMARY")"
  printf -- '- tasks: `%s`\n' "$(jq -r '.task_count' "$SUMMARY")"
  printf -- '- observer/tester: `%s` / `%s`\n' \
    "$(jq -r '.observer_count_required' "$SUMMARY")" \
    "$(jq -r '.tester_count_required' "$SUMMARY")"
  printf -- '- required confusion points: `%s`\n' "$(jq -r '.required_confusion_point_count' "$SUMMARY")"
  printf -- '- ready for renderer change from human observation: `%s`\n' \
    "$(jq -r '.ready_for_renderer_change_from_human_observation' "$SUMMARY")"
  printf -- '- public launch ready claimed: `%s`\n' "$(jq -r '.public_launch_ready_claimed' "$SUMMARY")"
  printf -- '- Android S5 real-device claimed: `%s`\n' "$(jq -r '.android_s5_real_device_claimed' "$SUMMARY")"
} >"$SUMMARY_MD"

printf 'TRILLIONNIUM_WORLD_FIRST_CONTACT_HUMAN_PLAYTEST_RUNBOOK_GREEN %s %s\n' "$SUMMARY" "$SUMMARY_MD"
