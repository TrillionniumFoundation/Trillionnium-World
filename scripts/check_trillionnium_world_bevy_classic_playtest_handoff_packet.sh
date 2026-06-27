#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$OUT_DIR/bevy-classic-playtest-handoff-packet.json"
MARKDOWN="$OUT_DIR/bevy-classic-playtest-handoff-packet.md"
HANDOFF="$OUT_DIR/bevy-classic-playtest-handoff-readiness.json"
READINESS="$OUT_DIR/bevy-classic-playtest-readiness.json"
LAUNCHER="$OUT_DIR/bevy-classic-playtest-launcher.json"
RUNNER="$OUT_DIR/bevy-classic-playtest-runner-status.json"
OBSERVABILITY="$OUT_DIR/bevy-classic-rts-playtest-observability-readiness.json"
REFRESH="${TRNM_BEVY_HANDOFF_PACKET_REFRESH:-1}"

mkdir -p "$OUT_DIR"

if [[ "$REFRESH" != "0" ]]; then
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_handoff_readiness.sh" >/dev/null
fi

artifact_json() {
  local label="$1"
  local path="$2"
  if [[ ! -f "$path" ]]; then
    printf 'missing handoff packet artifact: %s\n' "$path" >&2
    return 1
  fi
  jq -n \
    --arg label "$label" \
    --arg path "${path#$ROOT/}" \
    --arg sha256 "$(sha256sum "$path" | awk '{print $1}')" \
    --argjson bytes "$(stat -c '%s' "$path")" \
    '{label: $label, path: $path, sha256: $sha256, bytes: $bytes}'
}

ARTIFACTS_JSON="$(
  {
    artifact_json playtest_handoff_readiness "$HANDOFF"
    artifact_json playtest_readiness "$READINESS"
    artifact_json playtest_launcher "$LAUNCHER"
    artifact_json playtest_runner_status "$RUNNER"
    artifact_json playtest_observability_readiness "$OBSERVABILITY"
  } | jq -s .
)"

jq -n \
  --slurpfile handoff "$HANDOFF" \
  --slurpfile readiness "$READINESS" \
  --slurpfile launcher "$LAUNCHER" \
  --slurpfile runner "$RUNNER" \
  --slurpfile observability "$OBSERVABILITY" \
  --argjson artifacts "$ARTIFACTS_JSON" '
  def ok($x): ($x[0].green == true);
  {
    contract_version: "trillionnium_world_bevy_classic_playtest_handoff_packet_v1",
    status: "classic_playtest_handoff_packet_green",
    green: (
      ok($handoff)
      and ok($readiness)
      and ok($launcher)
      and ok($runner)
      and ok($observability)
      and ($handoff[0].public_launch_ready_claimed == false)
      and ($handoff[0].android_s5_real_device_claimed == false)
      and ($handoff[0].handoff_summary.runner_main_pid > 0)
      and ($handoff[0].handoff_summary.campaign_slot_bytes > 20000)
      and ($handoff[0].handoff_summary.resume_handoff_state == "resumed:league-coliseum")
      and ($handoff[0].handoff_summary.first_contact_runtime_review_contract == "trnm_rts_evidence_bevy_runtime_adapter_v1")
      and ($handoff[0].handoff_summary.first_contact_runtime_review_after_command_queue == ["move:8,4"])
      and ($handoff[0].gates.first_contact_runtime_review_gate == true)
      and ($handoff[0].gates.first_contact_runtime_adapter_evidence_gate == true)
    ),
    source_contracts: {
      playtest_handoff_readiness: $handoff[0].contract_version,
      playtest_readiness: $readiness[0].contract_version,
      playtest_launcher: $launcher[0].contract_version,
      playtest_runner_status: $runner[0].contract_version,
      playtest_observability_readiness: $observability[0].contract_version,
      first_contact_runtime_review: $handoff[0].source_contracts.first_contact_runtime_review
    },
    handoff_summary: $handoff[0].handoff_summary,
    gates: {
      handoff_readiness_green: ok($handoff),
      playtest_readiness_green: ok($readiness),
      launcher_green: ok($launcher),
      runner_green: ok($runner),
      observability_green: ok($observability),
      public_launch_not_claimed_gate: ($handoff[0].public_launch_ready_claimed == false),
      android_s5_real_device_not_claimed_gate: ($handoff[0].android_s5_real_device_claimed == false),
      first_contact_basin_spec_gate: $handoff[0].gates.first_contact_basin_spec_gate,
      first_contact_runtime_review_gate: $handoff[0].gates.first_contact_runtime_review_gate,
      first_contact_runtime_adapter_evidence_gate: $handoff[0].gates.first_contact_runtime_adapter_evidence_gate,
      first_contact_offline_adapter_consumption_gate: $handoff[0].gates.first_contact_offline_adapter_consumption_gate,
      first_contact_offline_adapter_session_transition_gate: $handoff[0].gates.first_contact_offline_adapter_session_transition_gate,
      first_contact_offline_adapter_lobby_ready_gate: $handoff[0].gates.first_contact_offline_adapter_lobby_ready_gate,
      artifact_count_gate: ($artifacts | length == 5),
      artifact_sha_gate: ($artifacts | all((.sha256 | test("^[0-9a-f]{64}$")) and (.bytes > 0)))
    },
    run_commands: {
      refresh_handoff: "./scripts/check_trillionnium_world_bevy_classic_playtest_handoff_readiness.sh",
      refresh_packet: "./scripts/check_trillionnium_world_bevy_classic_playtest_handoff_packet.sh",
      inspect_runner: "systemctl --user status trillionnium-bevy-playtest.service",
      launch_client: "./scripts/run_trillionnium_world_bevy_client.sh"
    },
    artifact_manifest: $artifacts,
    no_credit_boundaries: {
      public_launch_ready_claimed: false,
      android_s5_real_device_claimed: false,
      openra_natural_replay_or_headless_parity_claimed: false
    },
    public_launch_ready: false,
    public_launch_ready_claimed: false,
    android_s5_real_device_claimed: false,
    openra_natural_replay_or_headless_parity_claimed: false,
    markdown_path: "acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-handoff-packet.md",
    source_of_truth: "Classic playtest handoff packet binds the local Bevy human-playtest handoff to checksummed evidence artifacts and replayable commands. It is a local host-side playtest packet only, not public launch, S5 real-device, or OpenRA natural replay/headless parity credit."
  }
  | .source_contract_count = (.source_contracts | keys | length)
  | .artifact_count = (.artifact_manifest | length)
  | .artifact_bytes_total = ([.artifact_manifest[].bytes] | add)
  | .gate_count = (.gates | keys | length)
  | .passed_gate_count = ([.gates[] | select(. == true)] | length)
  | .failed_gate_count = ([.gates[] | select(. != true)] | length)
' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_playtest_handoff_packet_v1"
  and .status == "classic_playtest_handoff_packet_green"
  and .green == true
  and .source_contracts.playtest_handoff_readiness == "trillionnium_world_bevy_classic_playtest_handoff_readiness_v1"
  and .source_contracts.playtest_readiness == "trillionnium_world_bevy_classic_playtest_readiness_v1"
  and .source_contracts.playtest_launcher == "trillionnium_world_bevy_classic_playtest_launcher_v1"
  and .source_contracts.playtest_runner_status == "trillionnium_world_bevy_classic_playtest_runner_status_v1"
  and .source_contracts.playtest_observability_readiness == "trillionnium_world_bevy_classic_rts_playtest_observability_readiness_v1"
  and .source_contracts.first_contact_runtime_review == "trnm_rts_evidence_bevy_runtime_adapter_v1"
  and .source_contract_count == (.source_contracts | keys | length)
  and .artifact_count == (.artifact_manifest | length)
  and .artifact_bytes_total == ([.artifact_manifest[].bytes] | add)
  and .gate_count == (.gates | keys | length)
  and .passed_gate_count == ([.gates[] | select(. == true)] | length)
  and .failed_gate_count == ([.gates[] | select(. != true)] | length)
  and .failed_gate_count == 0
  and .gates.handoff_readiness_green == true
  and .gates.playtest_readiness_green == true
  and .gates.launcher_green == true
  and .gates.runner_green == true
  and .gates.observability_green == true
  and .gates.public_launch_not_claimed_gate == true
  and .gates.android_s5_real_device_not_claimed_gate == true
  and .gates.first_contact_basin_spec_gate == true
  and .gates.first_contact_runtime_review_gate == true
  and .gates.first_contact_runtime_adapter_evidence_gate == true
  and .gates.first_contact_offline_adapter_consumption_gate == true
  and .gates.first_contact_offline_adapter_session_transition_gate == true
  and .gates.first_contact_offline_adapter_lobby_ready_gate == true
  and .gates.artifact_count_gate == true
  and .gates.artifact_sha_gate == true
  and .handoff_summary.runner_main_pid > 0
  and .handoff_summary.campaign_slot_bytes > 20000
  and (.handoff_summary.title_actions | index("CAMPAIGN:START") != null)
  and (.handoff_summary.title_actions | index("CAMPAIGN:CONTINUE") != null)
  and (.handoff_summary.title_actions | index("CAMPAIGN:REPLAY") != null)
  and .handoff_summary.resume_handoff_state == "resumed:league-coliseum"
  and .handoff_summary.first_contact_basin_map_id == "first_contact_basin"
  and .handoff_summary.first_contact_runtime_review_contract == "trnm_rts_evidence_bevy_runtime_adapter_v1"
  and .handoff_summary.first_contact_runtime_review_contract_count == 5
  and .handoff_summary.first_contact_runtime_review_after_command_queue == ["move:8,4"]
  and .handoff_summary.first_contact_runtime_review_command_stamp_tile == "8,4"
  and .run_commands.refresh_handoff == "./scripts/check_trillionnium_world_bevy_classic_playtest_handoff_readiness.sh"
  and .run_commands.refresh_packet == "./scripts/check_trillionnium_world_bevy_classic_playtest_handoff_packet.sh"
  and .no_credit_boundaries.public_launch_ready_claimed == false
  and .no_credit_boundaries.android_s5_real_device_claimed == false
  and .no_credit_boundaries.openra_natural_replay_or_headless_parity_claimed == false
  and .public_launch_ready == false
  and .public_launch_ready_claimed == false
  and .android_s5_real_device_claimed == false
  and .openra_natural_replay_or_headless_parity_claimed == false
' "$SUMMARY" >/dev/null

{
  printf '# Bevy Classic Playtest Handoff Packet\n\n'
  printf '%s\n' "- Status: \`$(jq -r '.green' "$SUMMARY")\`"
  printf '%s\n' "- Contract: \`$(jq -r '.contract_version' "$SUMMARY")\`"
  printf '%s\n' "- Gate count: \`$(jq -r '.passed_gate_count' "$SUMMARY")\` / \`$(jq -r '.gate_count' "$SUMMARY")\` passed"
  printf '%s\n' "- Artifact count: \`$(jq -r '.artifact_count' "$SUMMARY")\`, bytes \`$(jq -r '.artifact_bytes_total' "$SUMMARY")\`"
  printf '%s\n' \
    "- Runner: \`$(jq -r '.handoff_summary.runner_service' "$SUMMARY")\` PID \`$(jq -r '.handoff_summary.runner_main_pid' "$SUMMARY")\`"
  printf '%s\n' \
    "- Resume: \`$(jq -r '.handoff_summary.resume_room_id' "$SUMMARY")\` / \`$(jq -r '.handoff_summary.resume_map_scene' "$SUMMARY")\` / \`$(jq -r '.handoff_summary.resume_handoff_state' "$SUMMARY")\`"
  printf '%s\n' "- Campaign slot bytes: \`$(jq -r '.handoff_summary.campaign_slot_bytes' "$SUMMARY")\`"
  printf '%s\n' "- Title actions: \`$(jq -r '.handoff_summary.title_actions | join(", ")' "$SUMMARY")\`"
  printf '%s\n' "- First Contact runtime review: \`$(jq -r '.handoff_summary.first_contact_runtime_review_contract' "$SUMMARY")\` after \`$(jq -r '.handoff_summary.first_contact_runtime_review_after_command_queue | join(", ")' "$SUMMARY")\` tile \`$(jq -r '.handoff_summary.first_contact_runtime_review_command_stamp_tile' "$SUMMARY")\`"
  printf '%s\n\n' \
    "- Observability: replay \`$(jq -r '.handoff_summary.replay_elapsed_seconds' "$SUMMARY")s\`, endurance \`$(jq -r '.handoff_summary.endurance_elapsed_seconds' "$SUMMARY")s\`, peak units \`$(jq -r '.handoff_summary.endurance_peak_active_units' "$SUMMARY")\`"
  printf '## Commands\n\n'
  jq -r '.run_commands | to_entries[] | "- `" + .key + "`: `" + .value + "`"' "$SUMMARY"
  printf '\n## Evidence\n\n'
  jq -r '.artifact_manifest[] | "- `" + .label + "`: `" + .path + "` sha256 `" + .sha256 + "` bytes `" + (.bytes|tostring) + "`"' "$SUMMARY"
  printf '\n## Boundaries\n\n'
  printf '%s\n' '- Public launch ready: `false`'
  printf '%s\n' '- Android S5 real device ready: `false`'
  printf '%s\n' '- OpenRA natural replay/headless parity: `false`'
} >"$MARKDOWN"

grep -q 'Bevy Classic Playtest Handoff Packet' "$MARKDOWN"
grep -q 'Public launch ready: `false`' "$MARKDOWN"
grep -q 'Android S5 real device ready: `false`' "$MARKDOWN"
grep -q './scripts/check_trillionnium_world_bevy_classic_playtest_handoff_packet.sh' "$MARKDOWN"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_PLAYTEST_HANDOFF_PACKET_GREEN %s %s\n' "$SUMMARY" "$MARKDOWN"
