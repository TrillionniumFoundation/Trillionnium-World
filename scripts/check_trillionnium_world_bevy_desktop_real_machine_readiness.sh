#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$OUT_DIR/bevy-desktop-real-machine-readiness.json"
MARKDOWN="$OUT_DIR/bevy-desktop-real-machine-readiness.md"
REFRESH="${TRNM_WORLD_DESKTOP_REAL_MACHINE_REFRESH:-1}"

RUNNER="$OUT_DIR/bevy-classic-playtest-runner-status.json"
SCREENSHOT="$OUT_DIR/bevy-live-window-screenshot-sequence.json"
MOUSE="$OUT_DIR/bevy-live-window-mouse-hit-test-sequence.json"
LAYER_PROBE="$OUT_DIR/bevy-live-window-layer-pixel-probe.json"
TEXTURE_CORRELATION="$OUT_DIR/bevy-live-window-texture-correlation.json"
SAMPLED_CORRELATION="$OUT_DIR/bevy-live-window-sampled-texture-correlation.json"
HANDOFF="$OUT_DIR/bevy-classic-playtest-handoff-packet.json"

mkdir -p "$OUT_DIR"

if [[ "$REFRESH" != "0" ]]; then
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_runner_status.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_live_window_screenshot_sequence.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_live_window_mouse_hit_test_sequence.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_live_window_layer_pixel_probe.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_live_window_texture_correlation.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_live_window_sampled_texture_correlation.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_handoff_packet.sh" >/dev/null
fi

artifact_json() {
  local label="$1"
  local path="$2"
  if [[ ! -f "$path" ]]; then
    printf 'missing desktop real-machine artifact: %s\n' "$path" >&2
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
    artifact_json classic_playtest_runner_status "$RUNNER"
    artifact_json live_window_screenshot_sequence "$SCREENSHOT"
    artifact_json live_window_mouse_hit_test_sequence "$MOUSE"
    artifact_json live_window_layer_pixel_probe "$LAYER_PROBE"
    artifact_json live_window_texture_correlation "$TEXTURE_CORRELATION"
    artifact_json live_window_sampled_texture_correlation "$SAMPLED_CORRELATION"
    artifact_json classic_playtest_handoff_packet "$HANDOFF"
  } | jq -s .
)"

jq -n \
  --slurpfile runner "$RUNNER" \
  --slurpfile screenshot "$SCREENSHOT" \
  --slurpfile mouse "$MOUSE" \
  --slurpfile layer "$LAYER_PROBE" \
  --slurpfile texture "$TEXTURE_CORRELATION" \
  --slurpfile sampled "$SAMPLED_CORRELATION" \
  --slurpfile handoff "$HANDOFF" \
  --argjson artifacts "$ARTIFACTS_JSON" '
  ($runner[0]) as $runner |
  ($screenshot[0]) as $screenshot |
  ($mouse[0]) as $mouse |
  ($layer[0]) as $layer |
  ($texture[0]) as $texture |
  ($sampled[0]) as $sampled |
  ($handoff[0]) as $handoff |
  (
    $runner.green == true
    and $runner.gates.service_process_gate == true
    and $runner.gates.release_binary_gate == true
    and $runner.gates.classic_env_gate == true
    and $runner.gates.cex_path_gate == true
  ) as $runner_gate |
  (
    $screenshot.green == true
    and $screenshot.host_window_gate == true
    and $screenshot.frame_count_gate == true
    and $screenshot.frame_sequence_gate == true
    and $screenshot.frame_change_gate == true
    and $screenshot.screenshot_nonblank_gate == true
    and $screenshot.contact_sheet_gate == true
    and $screenshot.final_frame_gate == true
    and $screenshot.runtime_texture_launch_env_gate == true
    and $screenshot.android_s5_real_device_claimed == false
  ) as $screenshot_gate |
  (
    $mouse.green == true
    and $mouse.hit_test_map_gate == true
    and $mouse.host_window_gate == true
    and $mouse.mouse_event_count_gate == true
    and $mouse.frame_count_gate == true
    and $mouse.frame_sequence_gate == true
    and $mouse.screenshot_nonblank_gate == true
    and $mouse.frame_change_gate == true
    and $mouse.slot_write_gate == true
    and $mouse.contact_sheet_gate == true
    and $mouse.android_s5_real_device_claimed == false
  ) as $mouse_gate |
  (
    $layer.green == true
    and $layer.gates.live_window_sequence_gate == true
    and $layer.gates.region_probe_gate == true
    and $layer.android_s5_real_device_claimed == false
  ) as $layer_gate |
  (
    $texture.green == true
    and $texture.gates.live_window_pixel_probe_gate == true
    and $texture.gates.image_handle_gate == true
    and $texture.gates.texture_atlas_layout_gate == true
    and $texture.gates.four_layer_texture_window_correlation_gate == true
    and $texture.gates.boundary_gate == true
    and $texture.android_s5_real_device_claimed == false
  ) as $texture_gate |
  (
    $sampled.green == true
    and $sampled.gates.four_layer_sampled_live_correlation_gate == true
    and $sampled.gates.boundary_gate == true
    and $sampled.android_s5_real_device_claimed == false
  ) as $sampled_gate |
  (
    $handoff.green == true
    and $handoff.no_credit_boundaries.public_launch_ready_claimed == false
    and $handoff.no_credit_boundaries.android_s5_real_device_claimed == false
  ) as $handoff_gate |
  (
    $runner_gate
    and $screenshot_gate
    and $mouse_gate
    and $layer_gate
    and $texture_gate
    and $sampled_gate
    and $handoff_gate
  ) as $green |
  {
    contract_version: "trillionnium_world_bevy_desktop_real_machine_readiness_v1",
    generated_at: (now | todate),
    status: (if $green then "desktop_real_machine_readiness_green" else "desktop_real_machine_readiness_blocked" end),
    green: $green,
    source_of_truth: "Desktop-first real-machine gate binds the live X11 Bevy window, visible screenshots, XTest keyboard input, XTest mouse button hit-tests against visible Bevy controls, live-window pixel/texture correlation, release runner status, and human-playtest handoff packet. It intentionally excludes Android S5 real-device credit.",
    gates: {
      release_runner_gate: $runner_gate,
      live_window_screenshot_sequence_gate: $screenshot_gate,
      live_window_mouse_hit_test_sequence_gate: $mouse_gate,
      live_window_layer_pixel_probe_gate: $layer_gate,
      live_window_texture_correlation_gate: $texture_gate,
      live_window_sampled_texture_correlation_gate: $sampled_gate,
      playtest_handoff_packet_gate: $handoff_gate,
      desktop_before_mobile_gate: true,
      android_s5_real_device_not_required_gate: true
    },
    desktop_runtime: {
      display: $screenshot.display,
      screenshot_window_id: $screenshot.window_id,
      release_runner_service: $runner.service.unit,
      release_runner_pid: $runner.service.main_pid,
      release_binary: $runner.runtime.expected_binary,
      release_process_cwd: $runner.runtime.process_cwd,
      classic_manifest_sha256: $runner.runtime.manifest_sha256,
      selected_environment: $runner.runtime.selected_environment
    },
    desktop_evidence: {
      screenshot_frame_count: ($screenshot.frames | length),
      screenshot_expected_frame_ids: $screenshot.expected_frame_ids,
      screenshot_actual_frame_ids: $screenshot.actual_frame_ids,
      screenshot_contact_sheet_path: $screenshot.contact_sheet_path,
      screenshot_final_frame_path: $screenshot.final_frame_path,
      screenshot_final_frame_bytes: $screenshot.final_frame_bytes,
      keyboard_event_count: ($screenshot.key_events | length),
      mouse_event_count: ($mouse.mouse_events | length),
      mouse_slot_a_bytes: $mouse.slot_a_bytes,
      mouse_contact_sheet_path: $mouse.contact_sheet_path,
      runtime_probe_path: $screenshot.runtime_probe_path,
      layer_pixel_probe_contract: $layer.contract_version,
      texture_correlation_contract: $texture.contract_version,
      sampled_texture_correlation_contract: $sampled.contract_version,
      handoff_runner_service: $handoff.handoff_summary.runner_service,
      handoff_resume_state: $handoff.handoff_summary.resume_handoff_state,
      handoff_replay_elapsed_seconds: $handoff.handoff_summary.replay_elapsed_seconds,
      handoff_endurance_elapsed_seconds: $handoff.handoff_summary.endurance_elapsed_seconds
    },
    run_commands: {
      refresh_desktop_real_machine: "./scripts/check_trillionnium_world_bevy_desktop_real_machine_readiness.sh",
      fast_recheck_existing_artifacts: "TRNM_WORLD_DESKTOP_REAL_MACHINE_REFRESH=0 ./scripts/check_trillionnium_world_bevy_desktop_real_machine_readiness.sh",
      refresh_live_window_screenshot_sequence: "./scripts/check_trillionnium_world_bevy_live_window_screenshot_sequence.sh",
      refresh_live_window_mouse_hit_test_sequence: "./scripts/check_trillionnium_world_bevy_live_window_mouse_hit_test_sequence.sh",
      inspect_runner: "systemctl --user status trillionnium-bevy-playtest.service"
    },
    artifact_manifest: $artifacts,
    no_credit_boundaries: {
      android_s5_real_device_claimed: false,
      public_launch_ready_claimed: false,
      live_public_network_exposure_performed: false,
      live_osm_ingestion_performed: false,
      desktop_real_machine_scope: "local_linux_desktop_x11_window_and_release_runner"
    }
  }' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_desktop_real_machine_readiness_v1"
  and .green == true
  and .status == "desktop_real_machine_readiness_green"
  and .gates.release_runner_gate == true
  and .gates.live_window_screenshot_sequence_gate == true
  and .gates.live_window_mouse_hit_test_sequence_gate == true
  and .gates.live_window_layer_pixel_probe_gate == true
  and .gates.live_window_texture_correlation_gate == true
  and .gates.live_window_sampled_texture_correlation_gate == true
  and .gates.playtest_handoff_packet_gate == true
  and .gates.desktop_before_mobile_gate == true
  and .gates.android_s5_real_device_not_required_gate == true
  and .desktop_evidence.screenshot_frame_count >= 11
  and .desktop_evidence.keyboard_event_count >= 10
  and .desktop_evidence.mouse_event_count >= 10
  and .desktop_evidence.mouse_slot_a_bytes > 512
  and .desktop_runtime.release_runner_pid > 0
  and .no_credit_boundaries.android_s5_real_device_claimed == false
  and .no_credit_boundaries.desktop_real_machine_scope == "local_linux_desktop_x11_window_and_release_runner"
  and (.artifact_manifest | length == 7)
' "$SUMMARY" >/dev/null

{
  printf '# Bevy Desktop Real-Machine Readiness\n\n'
  printf -- '- status: `%s`\n' "$(jq -r '.status' "$SUMMARY")"
  printf -- '- green: `%s`\n' "$(jq -r '.green' "$SUMMARY")"
  printf -- '- display: `%s`\n' "$(jq -r '.desktop_runtime.display' "$SUMMARY")"
  printf -- '- release_runner: `%s` PID `%s`\n' \
    "$(jq -r '.desktop_runtime.release_runner_service' "$SUMMARY")" \
    "$(jq -r '.desktop_runtime.release_runner_pid' "$SUMMARY")"
  printf -- '- screenshot_frames: `%s`\n' "$(jq -r '.desktop_evidence.screenshot_frame_count' "$SUMMARY")"
  printf -- '- keyboard_events: `%s`\n' "$(jq -r '.desktop_evidence.keyboard_event_count' "$SUMMARY")"
  printf -- '- mouse_events: `%s`\n' "$(jq -r '.desktop_evidence.mouse_event_count' "$SUMMARY")"
  printf -- '- mouse_slot_a_bytes: `%s`\n' "$(jq -r '.desktop_evidence.mouse_slot_a_bytes' "$SUMMARY")"
  printf -- '- android_s5_real_device_claimed: `false`\n\n'
  printf '## Commands\n\n'
  jq -r '.run_commands | to_entries[] | "- `" + .key + "`: `" + .value + "`"' "$SUMMARY"
  printf '\n## Evidence\n\n'
  jq -r '.artifact_manifest[] | "- `" + .label + "`: `" + .path + "` sha256 `" + .sha256 + "`"' "$SUMMARY"
  printf '\n## Boundary\n\n'
  printf -- '- Desktop real-machine scope is local Linux desktop X11 window plus release runner.\n'
  printf -- '- Android S5 real-device evidence remains separate and intentionally last.\n'
} >"$MARKDOWN"

grep -q 'Bevy Desktop Real-Machine Readiness' "$MARKDOWN"
grep -q 'android_s5_real_device_claimed: `false`' "$MARKDOWN"

printf 'TRILLIONNIUM_WORLD_BEVY_DESKTOP_REAL_MACHINE_READINESS_GREEN %s %s\n' "$SUMMARY" "$MARKDOWN"
