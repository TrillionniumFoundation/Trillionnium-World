#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-handoff-readiness.json"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_launcher.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_runner_status.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_playtest_observability_readiness.sh" >/dev/null

jq -n \
  --slurpfile readiness "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-readiness.json" \
  --slurpfile launcher "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-launcher.json" \
  --slurpfile runner "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-runner-status.json" \
  --slurpfile observability "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-playtest-observability-readiness.json" '
  def ok($x): ($x[0].green == true);
  def readiness_check($name): ($readiness[0].checks[$name] == true);
  def readiness_gate($name): ($readiness[0].gates[$name] == true);
  def launcher_gate($name): ($launcher[0].gates[$name] == true);
  def runner_gate($name): ($runner[0].gates[$name] == true);
  {
    contract_version: "trillionnium_world_bevy_classic_playtest_handoff_readiness_v1",
    green: (
      ok($readiness)
      and ok($launcher)
      and ok($runner)
      and ok($observability)
      and readiness_check("classic_rts_first_minute_readiness_green")
      and readiness_check("classic_rts_map_ui_modeling_readiness_green")
      and readiness_check("classic_rts_campaign_outcome_ui_readiness_green")
      and readiness_check("classic_rts_combat_readability_pressure_readiness_green")
      and readiness_check("classic_rts_playtest_observability_readiness_green")
      and readiness_check("client_boundary_green")
      and readiness_check("playtest_runner_status_green")
      and readiness_check("playtest_launcher_green")
      and readiness_gate("rts_campaign_handoff_open_world_resume_gate")
      and readiness_gate("rts_campaign_handoff_snapshot_round_trip_gate")
      and readiness_gate("runner_service_process_gate")
      and readiness_gate("runner_release_binary_gate")
      and readiness_gate("runner_classic_env_gate")
      and readiness_gate("runner_cex_path_gate")
      and readiness_gate("launcher_player_launch_ready_gate")
      and readiness_gate("launcher_open_world_resume_gate")
      and launcher_gate("player_launch_ready_gate")
      and launcher_gate("campaign_slot_gate")
      and launcher_gate("open_world_resume_gate")
      and launcher_gate("player_command_gate")
      and launcher_gate("service_process_gate")
      and launcher_gate("release_binary_gate")
      and launcher_gate("cex_path_gate")
      and runner_gate("service_process_gate")
      and runner_gate("release_binary_gate")
      and runner_gate("classic_env_gate")
      and runner_gate("manifest_gate")
      and runner_gate("override_dir_gate")
      and runner_gate("workdir_gate")
      and runner_gate("cex_path_gate")
      and ($launcher[0].android_s5_real_device_claimed == false)
      and ($readiness[0].status != "public_launch_ready")
    ),
    source_contracts: {
      playtest_readiness: $readiness[0].contract_version,
      playtest_launcher: $launcher[0].contract_version,
      playtest_runner_status: $runner[0].contract_version,
      playtest_observability_readiness: $observability[0].contract_version
    },
    handoff_summary: {
      playtest_readiness_green: ok($readiness),
      launcher_green: ok($launcher),
      runner_green: ok($runner),
      observability_green: ok($observability),
      runner_service: $runner[0].service.unit,
      runner_main_pid: $runner[0].service.main_pid,
      runner_binary: $runner[0].runtime.expected_binary,
      runner_process_cwd: $runner[0].runtime.process_cwd,
      campaign_slot_bytes: $launcher[0].player_entry.campaign_slot_bytes,
      title_actions: $launcher[0].player_entry.title_actions,
      resume_room_id: $launcher[0].player_entry.final_current_room_id,
      resume_map_scene: $launcher[0].player_entry.final_map_scene,
      resume_handoff_state: $launcher[0].player_entry.final_open_world_handoff_state,
      resume_primary_action: $launcher[0].player_entry.final_contextual_primary_action_label,
      observability_preview_count: $observability[0].preview_count,
      replay_elapsed_seconds: $observability[0].replay_metrics_summary.elapsed_seconds,
      endurance_elapsed_seconds: $observability[0].endurance_skirmish_summary.elapsed_seconds,
      endurance_peak_active_units: $observability[0].endurance_skirmish_summary.peak_active_units
    },
    gates: {
      playtest_readiness_gate: ok($readiness),
      launcher_gate: ok($launcher),
      runner_gate: ok($runner),
      observability_gate: ok($observability),
      first_minute_gate: readiness_check("classic_rts_first_minute_readiness_green"),
      map_ui_modeling_gate: readiness_check("classic_rts_map_ui_modeling_readiness_green"),
      campaign_outcome_ui_gate: readiness_check("classic_rts_campaign_outcome_ui_readiness_green"),
      combat_readability_pressure_gate: readiness_check("classic_rts_combat_readability_pressure_readiness_green"),
      playtest_observability_gate: readiness_check("classic_rts_playtest_observability_readiness_green"),
      client_boundary_gate: readiness_check("client_boundary_green"),
      campaign_handoff_resume_gate: readiness_gate("rts_campaign_handoff_open_world_resume_gate"),
      campaign_handoff_snapshot_gate: readiness_gate("rts_campaign_handoff_snapshot_round_trip_gate"),
      runner_service_process_gate: runner_gate("service_process_gate"),
      runner_release_binary_gate: runner_gate("release_binary_gate"),
      runner_classic_env_gate: runner_gate("classic_env_gate"),
      runner_manifest_gate: runner_gate("manifest_gate"),
      runner_override_dir_gate: runner_gate("override_dir_gate"),
      runner_workdir_gate: runner_gate("workdir_gate"),
      runner_cex_path_gate: runner_gate("cex_path_gate"),
      launcher_player_launch_ready_gate: launcher_gate("player_launch_ready_gate"),
      launcher_campaign_slot_gate: launcher_gate("campaign_slot_gate"),
      launcher_open_world_resume_gate: launcher_gate("open_world_resume_gate"),
      launcher_player_command_gate: launcher_gate("player_command_gate"),
      launcher_cex_path_gate: launcher_gate("cex_path_gate"),
      public_launch_not_claimed_gate: ($readiness[0].status != "public_launch_ready"),
      android_s5_real_device_not_claimed_gate: ($launcher[0].android_s5_real_device_claimed == false)
    },
    evidence_paths: {
      playtest_readiness: "acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-readiness.json",
      playtest_launcher: "acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-launcher.json",
      playtest_runner_status: "acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-runner-status.json",
      playtest_observability_readiness: "acceptance/S5_native_bevy_device/latest/bevy-classic-rts-playtest-observability-readiness.json"
    },
    android_s5_real_device_claimed: false,
    public_launch_ready_claimed: false,
    source_of_truth: "Classic playtest handoff readiness is the local human-playtest handoff layer for trnm-world-bevy. It requires the full Bevy classic playtest readiness chain, a live release runner, a campaign launcher that resumes into the Bevy-owned open-world RTS handoff, and observability evidence. It does not claim S5 real-device evidence, public launch readiness, or OpenRA natural replay/headless parity."
  }
' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_playtest_handoff_readiness_v1"
  and .green == true
  and .source_contracts.playtest_readiness == "trillionnium_world_bevy_classic_playtest_readiness_v1"
  and .source_contracts.playtest_launcher == "trillionnium_world_bevy_classic_playtest_launcher_v1"
  and .source_contracts.playtest_runner_status == "trillionnium_world_bevy_classic_playtest_runner_status_v1"
  and .source_contracts.playtest_observability_readiness == "trillionnium_world_bevy_classic_rts_playtest_observability_readiness_v1"
  and .handoff_summary.playtest_readiness_green == true
  and .handoff_summary.launcher_green == true
  and .handoff_summary.runner_green == true
  and .handoff_summary.observability_green == true
  and .handoff_summary.runner_main_pid > 0
  and .handoff_summary.campaign_slot_bytes > 20000
  and (.handoff_summary.title_actions | index("CAMPAIGN:START") != null)
  and (.handoff_summary.title_actions | index("CAMPAIGN:CONTINUE") != null)
  and (.handoff_summary.title_actions | index("CAMPAIGN:REPLAY") != null)
  and .handoff_summary.resume_room_id == "league-coliseum"
  and .handoff_summary.resume_map_scene == "arena_outdoor"
  and .handoff_summary.resume_handoff_state == "resumed:league-coliseum"
  and .handoff_summary.resume_primary_action == "COMBAT:attack"
  and .handoff_summary.observability_preview_count == 4
  and .handoff_summary.replay_elapsed_seconds >= 55
  and .handoff_summary.endurance_elapsed_seconds >= 120
  and .handoff_summary.endurance_peak_active_units >= 24
  and .gates.playtest_readiness_gate == true
  and .gates.launcher_gate == true
  and .gates.runner_gate == true
  and .gates.observability_gate == true
  and .gates.first_minute_gate == true
  and .gates.map_ui_modeling_gate == true
  and .gates.campaign_outcome_ui_gate == true
  and .gates.combat_readability_pressure_gate == true
  and .gates.playtest_observability_gate == true
  and .gates.client_boundary_gate == true
  and .gates.campaign_handoff_resume_gate == true
  and .gates.campaign_handoff_snapshot_gate == true
  and .gates.runner_service_process_gate == true
  and .gates.runner_release_binary_gate == true
  and .gates.runner_classic_env_gate == true
  and .gates.runner_manifest_gate == true
  and .gates.runner_override_dir_gate == true
  and .gates.runner_workdir_gate == true
  and .gates.runner_cex_path_gate == true
  and .gates.launcher_player_launch_ready_gate == true
  and .gates.launcher_campaign_slot_gate == true
  and .gates.launcher_open_world_resume_gate == true
  and .gates.launcher_player_command_gate == true
  and .gates.launcher_cex_path_gate == true
  and .gates.public_launch_not_claimed_gate == true
  and .gates.android_s5_real_device_not_claimed_gate == true
  and .android_s5_real_device_claimed == false
  and .public_launch_ready_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_PLAYTEST_HANDOFF_READINESS_GREEN %s\n' "$SUMMARY"
