#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$OUT_DIR/bevy-classic-rts-production-desktop-review-packet.json"
MARKDOWN="$OUT_DIR/bevy-classic-rts-production-desktop-review-packet.md"
REFRESH="${TRNM_WORLD_RTS_PRODUCTION_DESKTOP_REVIEW_REFRESH:-1}"

PRODUCTION="$OUT_DIR/bevy-classic-rts-production-interaction-polish.json"
PRODUCTION_PREVIEW="$OUT_DIR/bevy-classic-rts-production-interaction-polish.ppm"
DESKTOP="$OUT_DIR/bevy-desktop-playtest-review-packet.json"
REAL_MACHINE="$OUT_DIR/bevy-desktop-real-machine-readiness.json"
SCREENSHOT="$OUT_DIR/bevy-live-window-screenshot-sequence.json"
MOUSE="$OUT_DIR/bevy-live-window-mouse-hit-test-sequence.json"

mkdir -p "$OUT_DIR"

if [[ "$REFRESH" != "0" ]]; then
  "$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_production_interaction_polish.sh" >/dev/null
  "$ROOT/scripts/check_trillionnium_world_bevy_desktop_playtest_review_packet.sh" >/dev/null
fi

artifact_json() {
  local label="$1"
  local path="$2"
  if [[ ! -f "$path" ]]; then
    printf 'missing production desktop review artifact: %s\n' "$path" >&2
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
    artifact_json production_interaction_polish "$PRODUCTION"
    artifact_json production_interaction_polish_preview "$PRODUCTION_PREVIEW"
    artifact_json desktop_playtest_review_packet "$DESKTOP"
    artifact_json desktop_real_machine_readiness "$REAL_MACHINE"
    artifact_json live_window_screenshot_sequence "$SCREENSHOT"
    artifact_json live_window_mouse_hit_test_sequence "$MOUSE"
  } | jq -s .
)"

jq -n \
  --slurpfile production "$PRODUCTION" \
  --slurpfile desktop "$DESKTOP" \
  --slurpfile real "$REAL_MACHINE" \
  --slurpfile screenshot "$SCREENSHOT" \
  --slurpfile mouse "$MOUSE" \
  --argjson artifacts "$ARTIFACTS_JSON" '
  ($production[0]) as $production |
  ($desktop[0]) as $desktop |
  ($real[0]) as $real |
  ($screenshot[0]) as $screenshot |
  ($mouse[0]) as $mouse |
  (
    $production.green == true
    and $production.production_interaction_polish_gate == true
    and $production.runtime_screen_mode == "player_runtime_command_interaction_screen"
    and $production.runtime_screen_gate == true
    and $production.evidence_board_only == false
    and $production.ui_skin_runtime_screen_mode == "player_runtime_production_hud_skin_screen"
    and $production.ui_skin_runtime_screen_gate == true
    and $production.ui_skin_evidence_board_only == false
    and $production.no_copy_boundary_gate == true
    and $production.interaction_surface_count == 6
    and ($production.interaction_surface_names | index("DRAG SELECT") != null)
    and ($production.interaction_surface_names | index("RIGHT CLICK MOVE") != null)
    and ($production.interaction_surface_names | index("ATTACK LOCK") != null)
    and ($production.interaction_surface_names | index("BUILD GHOST") != null)
    and ($production.interaction_surface_names | index("QUEUE PATH") != null)
    and ($production.interaction_surface_names | index("SCROLL MINIMAP") != null)
    and $production.drag_select_skin_pixel_count > 1000
    and $production.right_click_move_skin_pixel_count > 1000
    and $production.attack_lock_skin_pixel_count > 1000
    and $production.build_ghost_skin_pixel_count > 1000
    and $production.queue_path_skin_pixel_count > 1000
    and $production.scroll_minimap_skin_pixel_count > 1000
    and $production.public_launch_ready == false
    and $production.android_s5_real_device_claimed == false
  ) as $production_gate |
  (
    $desktop.green == true
    and $desktop.gates.desktop_real_machine_readiness_gate == true
    and $desktop.gates.keyboard_visual_review_gate == true
    and $desktop.gates.mouse_visual_review_gate == true
    and $desktop.gates.release_runner_gate == true
    and $desktop.gates.handoff_packet_gate == true
    and $desktop.gates.android_s5_real_device_not_claimed_gate == true
    and $desktop.gates.public_launch_not_claimed_gate == true
    and $desktop.no_credit_boundaries.android_s5_real_device_claimed == false
    and $desktop.no_credit_boundaries.public_launch_ready_claimed == false
  ) as $desktop_gate |
  (
    $real.green == true
    and $real.gates.live_window_screenshot_sequence_gate == true
    and $real.gates.live_window_mouse_hit_test_sequence_gate == true
    and $real.gates.desktop_before_mobile_gate == true
    and $real.gates.android_s5_real_device_not_required_gate == true
  ) as $real_gate |
  (
    $screenshot.green == true
    and ($screenshot.key_events | length) >= 10
    and $screenshot.frame_sequence_gate == true
    and $screenshot.contact_sheet_gate == true
    and $screenshot.android_s5_real_device_claimed == false
  ) as $keyboard_gate |
  (
    $mouse.green == true
    and ($mouse.mouse_events | length) >= 10
    and $mouse.mouse_event_count_gate == true
    and $mouse.contact_sheet_gate == true
    and $mouse.slot_write_gate == true
    and $mouse.android_s5_real_device_claimed == false
  ) as $mouse_gate |
  (
    ($artifacts | length) == 6
    and all($artifacts[]; (.bytes > 0) and (.sha256 | test("^[0-9a-f]{64}$")))
  ) as $artifact_gate |
  (
    $production_gate
    and $desktop_gate
    and $real_gate
    and $keyboard_gate
    and $mouse_gate
    and $artifact_gate
  ) as $green |
  ([
    $production_gate,
    $desktop_gate,
    $real_gate,
    $keyboard_gate,
    $mouse_gate,
    $artifact_gate,
    ($production_gate and $desktop_gate and $real_gate),
    true,
    true,
    true
  ]) as $gate_values |
  {
    contract_version: "trillionnium_world_bevy_classic_rts_production_desktop_review_packet_v1",
    generated_at: (now | todate),
    green: $green,
    status: (if $green then "classic_rts_production_desktop_review_packet_green" else "classic_rts_production_desktop_review_packet_blocked" end),
    source_contract_count: 5,
    artifact_count: ($artifacts | length),
    artifact_bytes_total: ([$artifacts[].bytes] | add),
    gate_count: ($gate_values | length),
    passed_gate_count: ($gate_values | map(select(. == true)) | length),
    failed_gate_count: ($gate_values | map(select(. != true)) | length),
    production_interaction_surface_count: $production.interaction_surface_count,
    production_interaction_source_contract_count: $production.source_contract_count,
    production_interaction_source_path_count: $production.source_path_count,
    production_interaction_runtime_screen_layout_count: $production.runtime_screen_layout_count,
    production_interaction_pixel_count_field_count: $production.interaction_pixel_count_field_count,
    production_interaction_surface_name_count: $production.interaction_surface_name_count,
    production_interaction_replacement_slot_count: $production.interaction_replacement_slot_count,
    production_interaction_source_surface_count: $production.interaction_source_surface_count,
    production_interaction_gate_count: $production.gate_count,
    production_interaction_passed_gate_count: $production.passed_gate_count,
    production_interaction_failed_gate_count: $production.failed_gate_count,
    desktop_screenshot_frame_count: $desktop.desktop_review_summary.screenshot_frame_count,
    desktop_keyboard_event_count: $desktop.desktop_review_summary.keyboard_event_count,
    desktop_mouse_event_count: $desktop.desktop_review_summary.mouse_event_count,
    desktop_mouse_slot_a_bytes: $desktop.desktop_review_summary.mouse_slot_a_bytes,
    production_review_summary_field_count: 30,
    desktop_review_summary_field_count: 10,
    artifact_label_count: ([$artifacts[].label] | length),
    artifact_path_count: ([$artifacts[].path] | length),
    manual_review_checklist_count: 6,
    manual_review_step_count: 6,
    run_command_count: 4,
    no_credit_boundary_count: 6,
    android_s5_real_device_claimed: false,
    public_launch_ready_claimed: false,
    live_public_network_exposure_performed: false,
    live_osm_ingestion_performed: false,
    production_ready_desktop_review_shipped: false,
    source_contracts: {
      production_interaction_polish: $production.contract_version,
      desktop_playtest_review_packet: $desktop.contract_version,
      desktop_real_machine_readiness: $real.contract_version,
      live_window_screenshot_sequence: $screenshot.contract_version,
      live_window_mouse_hit_test_sequence: $mouse.contract_version
    },
    gates: {
      production_interaction_polish_gate: $production_gate,
      desktop_playtest_review_packet_gate: $desktop_gate,
      desktop_real_machine_readiness_gate: $real_gate,
      keyboard_visual_review_gate: $keyboard_gate,
      mouse_visual_review_gate: $mouse_gate,
      artifact_manifest_gate: $artifact_gate,
      production_to_desktop_review_gate: ($production_gate and $desktop_gate and $real_gate),
      desktop_before_mobile_gate: true,
      android_s5_real_device_not_claimed_gate: true,
      public_launch_not_claimed_gate: true
    },
    production_review_summary: {
      interaction_surface_count: $production.interaction_surface_count,
      source_contract_count: $production.source_contract_count,
      source_path_count: $production.source_path_count,
      runtime_screen_layout_count: $production.runtime_screen_layout_count,
      interaction_pixel_count_field_count: $production.interaction_pixel_count_field_count,
      interaction_surface_name_count: $production.interaction_surface_name_count,
      interaction_replacement_slot_count: $production.interaction_replacement_slot_count,
      interaction_source_surface_count: $production.interaction_source_surface_count,
      gate_count: $production.gate_count,
      passed_gate_count: $production.passed_gate_count,
      failed_gate_count: $production.failed_gate_count,
      interaction_surface_names: $production.interaction_surface_names,
      runtime_screen_mode: $production.runtime_screen_mode,
      runtime_screen_gate: $production.runtime_screen_gate,
      evidence_board_only: $production.evidence_board_only,
      runtime_screen_layout: $production.runtime_screen_layout,
      ui_skin_runtime_screen_mode: $production.ui_skin_runtime_screen_mode,
      ui_skin_runtime_screen_gate: $production.ui_skin_runtime_screen_gate,
      ui_skin_evidence_board_only: $production.ui_skin_evidence_board_only,
      interaction_board_pixel_count: $production.interaction_board_pixel_count,
      drag_select_skin_pixel_count: $production.drag_select_skin_pixel_count,
      right_click_move_skin_pixel_count: $production.right_click_move_skin_pixel_count,
      attack_lock_skin_pixel_count: $production.attack_lock_skin_pixel_count,
      build_ghost_skin_pixel_count: $production.build_ghost_skin_pixel_count,
      queue_path_skin_pixel_count: $production.queue_path_skin_pixel_count,
      scroll_minimap_skin_pixel_count: $production.scroll_minimap_skin_pixel_count,
      hud_binding_pixel_count: $production.hud_binding_pixel_count,
      player_first_command_interaction_screen_gate: $production.player_first_command_interaction_screen_gate,
      interaction_pixel_counts: $production.interaction_pixel_counts,
      production_preview_path: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-interaction-polish.ppm"
    },
    desktop_review_summary: {
      display: $desktop.desktop_review_summary.display,
      release_runner_service: $desktop.desktop_review_summary.release_runner_service,
      release_runner_pid: $desktop.desktop_review_summary.release_runner_pid,
      screenshot_frame_count: $desktop.desktop_review_summary.screenshot_frame_count,
      keyboard_event_count: $desktop.desktop_review_summary.keyboard_event_count,
      mouse_event_count: $desktop.desktop_review_summary.mouse_event_count,
      mouse_slot_a_bytes: $desktop.desktop_review_summary.mouse_slot_a_bytes,
      screenshot_contact_sheet_path: $desktop.desktop_review_summary.screenshot_contact_sheet_path,
      mouse_contact_sheet_path: $desktop.desktop_review_summary.mouse_contact_sheet_path,
      handoff_resume_state: $desktop.desktop_review_summary.handoff_resume_state
    },
    manual_review_checklist: [
      {
        step: "inspect_production_interaction_polish_preview",
        artifact: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-production-interaction-polish.ppm",
        expected: "drag select, right-click move, attack lock, build ghost, queue path, and scroll/minimap feedback use original production UI skin slots"
      },
      {
        step: "review_keyboard_desktop_sequence",
        artifact: $desktop.desktop_review_summary.screenshot_contact_sheet_path,
        expected: "desktop keyboard sequence remains nonblank and ordered with release runner active"
      },
      {
        step: "review_mouse_desktop_sequence",
        artifact: $desktop.desktop_review_summary.mouse_contact_sheet_path,
        expected: "desktop XTest mouse hit tests click visible Bevy controls and write slot evidence"
      },
      {
        step: "compare_production_feedback_to_desktop_controls",
        expected: "production interaction surface names align with the desktop keyboard/mouse review packet without claiming Android or public launch"
      },
      {
        step: "fast_replay_production_desktop_packet",
        command: "TRNM_WORLD_RTS_PRODUCTION_DESKTOP_REVIEW_REFRESH=0 ./scripts/check_trillionnium_world_bevy_classic_rts_production_desktop_review_packet.sh",
        expected: "classic_rts_production_desktop_review_packet_green"
      },
      {
        step: "confirm_boundaries",
        expected: "android_s5_real_device_claimed=false, public_launch_ready_claimed=false, and production_ready_desktop_review_shipped=false"
      }
    ],
    run_commands: {
      refresh_production_desktop_review_packet: "./scripts/check_trillionnium_world_bevy_classic_rts_production_desktop_review_packet.sh",
      fast_production_desktop_review_packet: "TRNM_WORLD_RTS_PRODUCTION_DESKTOP_REVIEW_REFRESH=0 ./scripts/check_trillionnium_world_bevy_classic_rts_production_desktop_review_packet.sh",
      refresh_production_interaction_polish: "./scripts/check_trillionnium_world_bevy_classic_rts_production_interaction_polish.sh",
      refresh_desktop_playtest_review_packet: "./scripts/check_trillionnium_world_bevy_desktop_playtest_review_packet.sh"
    },
    artifact_manifest: $artifacts,
    no_credit_boundaries: {
      android_s5_real_device_claimed: false,
      public_launch_ready_claimed: false,
      live_public_network_exposure_performed: false,
      live_osm_ingestion_performed: false,
      production_ready_desktop_review_shipped: false,
      desktop_review_scope: "local_linux_desktop_x11_window_keyboard_mouse_with_production_interaction_polish"
    },
    source_of_truth: "Classic RTS production desktop review packet binds the original production interaction polish board to local Linux desktop X11 keyboard/mouse review evidence. It is a local review handoff only: no Android S5, public launch, live network, copied RTS UI/cursor art, or production-shipped UI credit."
  }' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_production_desktop_review_packet_v1"
  and .green == true
  and .status == "classic_rts_production_desktop_review_packet_green"
  and .source_contract_count == (.source_contracts | length)
  and .artifact_count == (.artifact_manifest | length)
  and .artifact_bytes_total == ([.artifact_manifest[].bytes] | add)
  and .gate_count == (.gates | length)
  and .passed_gate_count == ([.gates[]] | map(select(. == true)) | length)
  and .failed_gate_count == ([.gates[]] | map(select(. != true)) | length)
  and .failed_gate_count == 0
  and .production_interaction_surface_count == .production_review_summary.interaction_surface_count
  and .production_interaction_source_contract_count == .production_review_summary.source_contract_count
  and .production_interaction_source_path_count == .production_review_summary.source_path_count
  and .production_interaction_runtime_screen_layout_count == .production_review_summary.runtime_screen_layout_count
  and .production_interaction_pixel_count_field_count == .production_review_summary.interaction_pixel_count_field_count
  and .production_interaction_surface_name_count == .production_review_summary.interaction_surface_name_count
  and .production_interaction_replacement_slot_count == .production_review_summary.interaction_replacement_slot_count
  and .production_interaction_source_surface_count == .production_review_summary.interaction_source_surface_count
  and .production_interaction_gate_count == .production_review_summary.gate_count
  and .production_interaction_passed_gate_count == .production_review_summary.passed_gate_count
  and .production_interaction_failed_gate_count == .production_review_summary.failed_gate_count
  and .desktop_screenshot_frame_count == .desktop_review_summary.screenshot_frame_count
  and .desktop_keyboard_event_count == .desktop_review_summary.keyboard_event_count
  and .desktop_mouse_event_count == .desktop_review_summary.mouse_event_count
  and .desktop_mouse_slot_a_bytes == .desktop_review_summary.mouse_slot_a_bytes
  and .production_review_summary_field_count == (.production_review_summary | keys | length)
  and .desktop_review_summary_field_count == (.desktop_review_summary | keys | length)
  and .artifact_label_count == ([.artifact_manifest[].label] | length)
  and .artifact_path_count == ([.artifact_manifest[].path] | length)
  and .artifact_label_count == .artifact_count
  and .artifact_path_count == .artifact_count
  and .manual_review_checklist_count == (.manual_review_checklist | length)
  and .manual_review_step_count == ([.manual_review_checklist[].step] | length)
  and .run_command_count == (.run_commands | keys | length)
  and .no_credit_boundary_count == (.no_credit_boundaries | keys | length)
  and .android_s5_real_device_claimed == false
  and .public_launch_ready_claimed == false
  and .live_public_network_exposure_performed == false
  and .live_osm_ingestion_performed == false
  and .production_ready_desktop_review_shipped == false
  and .source_contracts.production_interaction_polish == "trillionnium_world_bevy_classic_rts_production_interaction_polish_v1"
  and .source_contracts.desktop_playtest_review_packet == "trillionnium_world_bevy_desktop_playtest_review_packet_v1"
  and .source_contracts.desktop_real_machine_readiness == "trillionnium_world_bevy_desktop_real_machine_readiness_v1"
  and .gates.production_interaction_polish_gate == true
  and .gates.desktop_playtest_review_packet_gate == true
  and .gates.desktop_real_machine_readiness_gate == true
  and .gates.keyboard_visual_review_gate == true
  and .gates.mouse_visual_review_gate == true
  and .gates.artifact_manifest_gate == true
  and .gates.production_to_desktop_review_gate == true
  and .production_review_summary.interaction_surface_count == 6
  and .production_review_summary.source_contract_count == 6
  and .production_review_summary.source_path_count == 6
  and .production_review_summary.runtime_screen_layout_count == 6
  and .production_review_summary.interaction_pixel_count_field_count == 5
  and .production_review_summary.interaction_surface_name_count == 6
  and .production_review_summary.interaction_replacement_slot_count == 6
  and .production_review_summary.interaction_source_surface_count == 6
  and .production_review_summary.gate_count == 12
  and .production_review_summary.passed_gate_count == 12
  and .production_review_summary.failed_gate_count == 0
  and (.production_review_summary.interaction_surface_names | index("DRAG SELECT") != null)
  and (.production_review_summary.interaction_surface_names | index("RIGHT CLICK MOVE") != null)
  and (.production_review_summary.interaction_surface_names | index("ATTACK LOCK") != null)
  and (.production_review_summary.interaction_surface_names | index("BUILD GHOST") != null)
  and (.production_review_summary.interaction_surface_names | index("QUEUE PATH") != null)
  and (.production_review_summary.interaction_surface_names | index("SCROLL MINIMAP") != null)
  and .production_review_summary.runtime_screen_mode == "player_runtime_command_interaction_screen"
  and .production_review_summary.runtime_screen_gate == true
  and .production_review_summary.evidence_board_only == false
  and .production_review_summary.runtime_screen_layout.drag_select == "visible marquee skin and selection feedback strip"
  and .production_review_summary.runtime_screen_layout.queue_path == "queued waypoint path, rally chain, reservation, and cancel/repath strip"
  and .production_review_summary.ui_skin_runtime_screen_mode == "player_runtime_production_hud_skin_screen"
  and .production_review_summary.ui_skin_runtime_screen_gate == true
  and .production_review_summary.ui_skin_evidence_board_only == false
  and .production_review_summary.interaction_board_pixel_count > 80000
  and .production_review_summary.drag_select_skin_pixel_count > 1000
  and .production_review_summary.right_click_move_skin_pixel_count > 1000
  and .production_review_summary.attack_lock_skin_pixel_count > 1000
  and .production_review_summary.build_ghost_skin_pixel_count > 1000
  and .production_review_summary.queue_path_skin_pixel_count > 1000
  and .production_review_summary.scroll_minimap_skin_pixel_count > 1000
  and .production_review_summary.player_first_command_interaction_screen_gate == true
  and .production_review_summary.interaction_pixel_counts.player_first_command_interaction_view_non_background > 120000
  and .production_review_summary.interaction_pixel_counts.player_first_command_interaction_view_frame > 8000
  and .production_review_summary.interaction_pixel_counts.player_first_command_interaction_status_strip > 10000
  and .desktop_review_summary.screenshot_frame_count >= 11
  and .desktop_review_summary.keyboard_event_count >= 10
  and .desktop_review_summary.mouse_event_count >= 10
  and .desktop_review_summary.mouse_slot_a_bytes > 512
  and (.manual_review_checklist | length == 6)
  and (.artifact_manifest | length == 6)
  and .no_credit_boundaries.android_s5_real_device_claimed == false
  and .no_credit_boundaries.public_launch_ready_claimed == false
  and .no_credit_boundaries.production_ready_desktop_review_shipped == false
  and .no_credit_boundaries.desktop_review_scope == "local_linux_desktop_x11_window_keyboard_mouse_with_production_interaction_polish"
' "$SUMMARY" >/dev/null

{
  printf '# Classic RTS Production Desktop Review Packet\n\n'
  printf -- '- status: `%s`\n' "$(jq -r '.status' "$SUMMARY")"
  printf -- '- green: `%s`\n' "$(jq -r '.green' "$SUMMARY")"
  printf -- '- production_surfaces: `%s`\n' "$(jq -r '.production_review_summary.interaction_surface_count' "$SUMMARY")"
  printf -- '- source_contract_count: `%s`\n' "$(jq -r '.source_contract_count' "$SUMMARY")"
  printf -- '- artifact_count: `%s`\n' "$(jq -r '.artifact_count' "$SUMMARY")"
  printf -- '- artifact_bytes_total: `%s`\n' "$(jq -r '.artifact_bytes_total' "$SUMMARY")"
  printf -- '- gate_count: `%s`\n' "$(jq -r '.gate_count' "$SUMMARY")"
  printf -- '- passed_gate_count: `%s`\n' "$(jq -r '.passed_gate_count' "$SUMMARY")"
  printf -- '- failed_gate_count: `%s`\n' "$(jq -r '.failed_gate_count' "$SUMMARY")"
  printf -- '- desktop_runner: `%s` PID `%s`\n' \
    "$(jq -r '.desktop_review_summary.release_runner_service' "$SUMMARY")" \
    "$(jq -r '.desktop_review_summary.release_runner_pid' "$SUMMARY")"
  printf -- '- keyboard_events: `%s`\n' "$(jq -r '.desktop_review_summary.keyboard_event_count' "$SUMMARY")"
  printf -- '- mouse_events: `%s`\n' "$(jq -r '.desktop_review_summary.mouse_event_count' "$SUMMARY")"
  printf -- '- artifact_label_count: `%s`\n' "$(jq -r '.artifact_label_count' "$SUMMARY")"
  printf -- '- manual_review_checklist_count: `%s`\n' "$(jq -r '.manual_review_checklist_count' "$SUMMARY")"
  printf -- '- run_command_count: `%s`\n' "$(jq -r '.run_command_count' "$SUMMARY")"
  printf -- '- no_credit_boundary_count: `%s`\n' "$(jq -r '.no_credit_boundary_count' "$SUMMARY")"
  printf -- '- android_s5_real_device_claimed: `false`\n'
  printf -- '- public_launch_ready_claimed: `false`\n\n'
  printf '## Manual Review Checklist\n\n'
  jq -r '.manual_review_checklist[] | "- `" + .step + "`: " + (.command // .artifact // .expected)' "$SUMMARY"
  printf '\n## Evidence\n\n'
  jq -r '.artifact_manifest[] | "- `" + .label + "`: `" + .path + "` sha256 `" + .sha256 + "`"' "$SUMMARY"
  printf '\n## Commands\n\n'
  jq -r '.run_commands | to_entries[] | "- `" + .key + "`: `" + .value + "`"' "$SUMMARY"
} >"$MARKDOWN"

grep -q 'Classic RTS Production Desktop Review Packet' "$MARKDOWN"
grep -q 'android_s5_real_device_claimed: `false`' "$MARKDOWN"
grep -q 'public_launch_ready_claimed: `false`' "$MARKDOWN"

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_PRODUCTION_DESKTOP_REVIEW_PACKET_GREEN %s %s\n' "$SUMMARY" "$MARKDOWN"
