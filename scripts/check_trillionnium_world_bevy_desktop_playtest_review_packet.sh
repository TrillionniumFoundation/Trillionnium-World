#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$OUT_DIR/bevy-desktop-playtest-review-packet.json"
MARKDOWN="$OUT_DIR/bevy-desktop-playtest-review-packet.md"
REFRESH="${TRNM_WORLD_DESKTOP_PLAYTEST_REVIEW_REFRESH:-1}"

DESKTOP="$OUT_DIR/bevy-desktop-real-machine-readiness.json"
SCREENSHOT="$OUT_DIR/bevy-live-window-screenshot-sequence.json"
MOUSE="$OUT_DIR/bevy-live-window-mouse-hit-test-sequence.json"
RUNNER="$OUT_DIR/bevy-classic-playtest-runner-status.json"
HANDOFF="$OUT_DIR/bevy-classic-playtest-handoff-packet.json"

mkdir -p "$OUT_DIR"

if [[ "$REFRESH" != "0" ]]; then
  "$ROOT/scripts/check_trillionnium_world_bevy_desktop_real_machine_readiness.sh" >/dev/null
fi

artifact_json() {
  local label="$1"
  local path="$2"
  if [[ ! -f "$path" ]]; then
    printf 'missing desktop playtest review artifact: %s\n' "$path" >&2
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
    artifact_json desktop_real_machine_readiness "$DESKTOP"
    artifact_json live_window_screenshot_sequence "$SCREENSHOT"
    artifact_json live_window_mouse_hit_test_sequence "$MOUSE"
    artifact_json classic_playtest_runner_status "$RUNNER"
    artifact_json classic_playtest_handoff_packet "$HANDOFF"
  } | jq -s .
)"

jq -n \
  --slurpfile desktop "$DESKTOP" \
  --slurpfile screenshot "$SCREENSHOT" \
  --slurpfile mouse "$MOUSE" \
  --slurpfile runner "$RUNNER" \
  --slurpfile handoff "$HANDOFF" \
  --argjson artifacts "$ARTIFACTS_JSON" '
  ($desktop[0]) as $desktop |
  ($screenshot[0]) as $screenshot |
  ($mouse[0]) as $mouse |
  ($runner[0]) as $runner |
  ($handoff[0]) as $handoff |
  (
    $desktop.green == true
    and $desktop.gates.live_window_screenshot_sequence_gate == true
    and $desktop.gates.live_window_mouse_hit_test_sequence_gate == true
    and $desktop.gates.android_s5_real_device_not_required_gate == true
  ) as $desktop_gate |
  (
    $screenshot.green == true
    and ($screenshot.key_events | length) >= 10
    and $screenshot.contact_sheet_gate == true
    and $screenshot.android_s5_real_device_claimed == false
  ) as $keyboard_visual_gate |
  (
    $mouse.green == true
    and ($mouse.mouse_events | length) >= 10
    and $mouse.slot_a_bytes > 512
    and $mouse.contact_sheet_gate == true
    and $mouse.android_s5_real_device_claimed == false
  ) as $mouse_visual_gate |
  (
    $runner.green == true
    and $runner.gates.service_process_gate == true
    and $runner.gates.release_binary_gate == true
  ) as $runner_gate |
  (
    $handoff.green == true
    and $handoff.no_credit_boundaries.public_launch_ready_claimed == false
    and $handoff.no_credit_boundaries.android_s5_real_device_claimed == false
  ) as $handoff_gate |
  ($artifacts | length == 5 and all($artifacts[]; (.bytes > 0) and (.sha256 | test("^[0-9a-f]{64}$")))) as $artifact_gate |
  ($desktop_gate and $keyboard_visual_gate and $mouse_visual_gate and $runner_gate and $handoff_gate and $artifact_gate) as $green |
  {
    contract_version: "trillionnium_world_bevy_desktop_playtest_review_packet_v1",
    generated_at: (now | todate),
    green: $green,
    status: (if $green then "desktop_playtest_review_packet_green" else "desktop_playtest_review_packet_blocked" end),
    source_contracts: {
      desktop_real_machine_readiness: $desktop.contract_version,
      live_window_screenshot_sequence: $screenshot.contract_version,
      live_window_mouse_hit_test_sequence: $mouse.contract_version,
      classic_playtest_runner_status: $runner.contract_version,
      classic_playtest_handoff_packet: $handoff.contract_version
    },
    gates: {
      desktop_real_machine_readiness_gate: $desktop_gate,
      keyboard_visual_review_gate: $keyboard_visual_gate,
      mouse_visual_review_gate: $mouse_visual_gate,
      release_runner_gate: $runner_gate,
      handoff_packet_gate: $handoff_gate,
      artifact_manifest_gate: $artifact_gate,
      desktop_before_mobile_gate: true,
      android_s5_real_device_not_claimed_gate: true,
      public_launch_not_claimed_gate: true
    },
    desktop_review_summary: {
      display: $desktop.desktop_runtime.display,
      release_runner_service: $desktop.desktop_runtime.release_runner_service,
      release_runner_pid: $desktop.desktop_runtime.release_runner_pid,
      screenshot_frame_count: $desktop.desktop_evidence.screenshot_frame_count,
      keyboard_event_count: $desktop.desktop_evidence.keyboard_event_count,
      mouse_event_count: $desktop.desktop_evidence.mouse_event_count,
      mouse_slot_a_bytes: $desktop.desktop_evidence.mouse_slot_a_bytes,
      screenshot_contact_sheet_path: $desktop.desktop_evidence.screenshot_contact_sheet_path,
      mouse_contact_sheet_path: $desktop.desktop_evidence.mouse_contact_sheet_path,
      handoff_resume_state: $desktop.desktop_evidence.handoff_resume_state,
      handoff_replay_elapsed_seconds: $desktop.desktop_evidence.handoff_replay_elapsed_seconds,
      handoff_endurance_elapsed_seconds: $desktop.desktop_evidence.handoff_endurance_elapsed_seconds
    },
    manual_review_checklist: [
      {
        step: "inspect_release_runner",
        command: "systemctl --user status trillionnium-bevy-playtest.service",
        expected: "release binary target/release/trnm-world-bevy run is active"
      },
      {
        step: "review_keyboard_screenshot_sequence",
        artifact: $screenshot.contact_sheet_path,
        expected: "title/create/talk/train/arena/fight/save/title/resume/complete frames are nonblank and ordered"
      },
      {
        step: "review_mouse_hit_test_sequence",
        artifact: $mouse.contact_sheet_path,
        expected: "XTest mouse clicks visible Bevy buttons and slot A is written"
      },
      {
        step: "replay_desktop_gate",
        command: "./scripts/check_trillionnium_world_bevy_desktop_real_machine_readiness.sh",
        expected: "desktop_real_machine_readiness_green"
      },
      {
        step: "confirm_boundaries",
        expected: "android_s5_real_device_claimed=false and public_launch_ready_claimed=false"
      }
    ],
    run_commands: {
      refresh_review_packet: "./scripts/check_trillionnium_world_bevy_desktop_playtest_review_packet.sh",
      fast_review_packet: "TRNM_WORLD_DESKTOP_PLAYTEST_REVIEW_REFRESH=0 ./scripts/check_trillionnium_world_bevy_desktop_playtest_review_packet.sh",
      refresh_desktop_real_machine: "./scripts/check_trillionnium_world_bevy_desktop_real_machine_readiness.sh",
      refresh_mouse_hit_test: "./scripts/check_trillionnium_world_bevy_live_window_mouse_hit_test_sequence.sh"
    },
    artifact_manifest: $artifacts,
    no_credit_boundaries: {
      android_s5_real_device_claimed: false,
      public_launch_ready_claimed: false,
      live_public_network_exposure_performed: false,
      live_osm_ingestion_performed: false,
      desktop_review_scope: "local_linux_desktop_x11_window_keyboard_mouse_visual_review_packet"
    },
    source_of_truth: "Desktop playtest review packet binds local Linux desktop runner, keyboard screenshot sequence, mouse hit-test sequence, and handoff evidence into a human-repeatable review checklist. It is desktop/local evidence only and intentionally preserves mobile/S5 and public-launch blockers."
  }' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_desktop_playtest_review_packet_v1"
  and .green == true
  and .status == "desktop_playtest_review_packet_green"
  and .gates.desktop_real_machine_readiness_gate == true
  and .gates.keyboard_visual_review_gate == true
  and .gates.mouse_visual_review_gate == true
  and .gates.release_runner_gate == true
  and .gates.handoff_packet_gate == true
  and .gates.artifact_manifest_gate == true
  and .desktop_review_summary.screenshot_frame_count >= 11
  and .desktop_review_summary.keyboard_event_count >= 10
  and .desktop_review_summary.mouse_event_count >= 10
  and .desktop_review_summary.mouse_slot_a_bytes > 512
  and (.manual_review_checklist | length == 5)
  and (.artifact_manifest | length == 5)
  and .no_credit_boundaries.android_s5_real_device_claimed == false
  and .no_credit_boundaries.public_launch_ready_claimed == false
  and .no_credit_boundaries.desktop_review_scope == "local_linux_desktop_x11_window_keyboard_mouse_visual_review_packet"
' "$SUMMARY" >/dev/null

{
  printf '# Bevy Desktop Playtest Review Packet\n\n'
  printf -- '- status: `%s`\n' "$(jq -r '.status' "$SUMMARY")"
  printf -- '- green: `%s`\n' "$(jq -r '.green' "$SUMMARY")"
  printf -- '- runner: `%s` PID `%s`\n' \
    "$(jq -r '.desktop_review_summary.release_runner_service' "$SUMMARY")" \
    "$(jq -r '.desktop_review_summary.release_runner_pid' "$SUMMARY")"
  printf -- '- keyboard_events: `%s`\n' "$(jq -r '.desktop_review_summary.keyboard_event_count' "$SUMMARY")"
  printf -- '- mouse_events: `%s`\n' "$(jq -r '.desktop_review_summary.mouse_event_count' "$SUMMARY")"
  printf -- '- mouse_slot_a_bytes: `%s`\n' "$(jq -r '.desktop_review_summary.mouse_slot_a_bytes' "$SUMMARY")"
  printf -- '- android_s5_real_device_claimed: `false`\n'
  printf -- '- public_launch_ready_claimed: `false`\n\n'
  printf '## Manual Review Checklist\n\n'
  jq -r '.manual_review_checklist[] | "- `" + .step + "`: " + (.command // .artifact // .expected)' "$SUMMARY"
  printf '\n## Evidence\n\n'
  jq -r '.artifact_manifest[] | "- `" + .label + "`: `" + .path + "` sha256 `" + .sha256 + "`"' "$SUMMARY"
  printf '\n## Commands\n\n'
  jq -r '.run_commands | to_entries[] | "- `" + .key + "`: `" + .value + "`"' "$SUMMARY"
} >"$MARKDOWN"

grep -q 'Bevy Desktop Playtest Review Packet' "$MARKDOWN"
grep -q 'android_s5_real_device_claimed: `false`' "$MARKDOWN"
grep -q 'public_launch_ready_claimed: `false`' "$MARKDOWN"

printf 'TRILLIONNIUM_WORLD_BEVY_DESKTOP_PLAYTEST_REVIEW_PACKET_GREEN %s %s\n' "$SUMMARY" "$MARKDOWN"
