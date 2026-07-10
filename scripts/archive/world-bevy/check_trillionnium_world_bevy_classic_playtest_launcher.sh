#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-launcher.json"
mkdir -p "$(dirname "$SUMMARY")"

"$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_campaign_entry.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_runner_status.sh" >/dev/null

jq -n \
  --slurpfile campaign "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-campaign-entry.json" \
  --slurpfile runner "$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-runner-status.json" '
  def has_action($a): (($campaign[0].title_actions // []) | index($a) != null);
  def gate($x): ($x == true);
  def runner_gate($name): ($runner[0].gates[$name] == true);
  {
    contract_version: "trillionnium_world_bevy_classic_playtest_launcher_v1",
    campaign_entry_contract: $campaign[0].contract_version,
    runner_status_contract: $runner[0].contract_version,
    title_menu_contract: $campaign[0].title_menu_contract,
    state_snapshot_contract: $campaign[0].state_snapshot_contract,
    status: "green",
    green: (
      gate($campaign[0].green)
      and gate($runner[0].green)
      and has_action("CAMPAIGN:START")
      and has_action("CAMPAIGN:CONTINUE")
      and has_action("CAMPAIGN:REPLAY")
      and ($campaign[0].input_action_count == 73)
      and ($campaign[0].start_input_count == 73)
      and ($campaign[0].replay_input_count == 73)
      and ($campaign[0].campaign_slot_bytes > 20000)
      and ($campaign[0].final_current_room_id == "league-coliseum")
      and ($campaign[0].final_map_scene == "arena_outdoor")
      and ($campaign[0].final_open_world_handoff_state == "resumed:league-coliseum")
      and ($campaign[0].final_contextual_primary_action_label == "COMBAT:attack")
      and runner_gate("service_process_gate")
      and runner_gate("release_binary_gate")
      and runner_gate("classic_env_gate")
      and runner_gate("manifest_gate")
      and runner_gate("override_dir_gate")
      and runner_gate("workdir_gate")
      and runner_gate("cex_path_gate")
    ),
    player_entry: {
      title_actions: $campaign[0].title_actions,
      primary_start_action: "CAMPAIGN:START",
      resume_action: "CAMPAIGN:CONTINUE",
      replay_action: "CAMPAIGN:REPLAY",
      followup_action_after_resume: "CONTINUE:SESSION",
      input_path: $campaign[0].input_path,
      input_action_count: $campaign[0].input_action_count,
      start_input_count: $campaign[0].start_input_count,
      replay_input_count: $campaign[0].replay_input_count,
      campaign_slot_path: $campaign[0].campaign_slot_path,
      campaign_slot_bytes: $campaign[0].campaign_slot_bytes,
      final_current_room_id: $campaign[0].final_current_room_id,
      final_map_scene: $campaign[0].final_map_scene,
      final_open_world_handoff_state: $campaign[0].final_open_world_handoff_state,
      final_contextual_primary_action_label: $campaign[0].final_contextual_primary_action_label
    },
    live_runner: {
      service: $runner[0].service,
      runtime: $runner[0].runtime
    },
    gates: {
      campaign_entry_gate: gate($campaign[0].green),
      runner_status_gate: gate($runner[0].green),
      title_campaign_start_action_gate: has_action("CAMPAIGN:START"),
      title_campaign_continue_action_gate: has_action("CAMPAIGN:CONTINUE"),
      title_campaign_replay_action_gate: has_action("CAMPAIGN:REPLAY"),
      campaign_start_gate: gate($campaign[0].start_gate),
      campaign_continue_gate: gate($campaign[0].continue_gate),
      campaign_continue_unlock_gate: gate($campaign[0].continue_unlock_gate),
      campaign_replay_gate: gate($campaign[0].replay_gate),
      campaign_slot_gate: (($campaign[0].campaign_slot_bytes // 0) > 20000),
      open_world_resume_gate: (
        $campaign[0].final_current_room_id == "league-coliseum"
        and $campaign[0].final_map_scene == "arena_outdoor"
        and $campaign[0].final_open_world_handoff_state == "resumed:league-coliseum"
      ),
      player_command_gate: ($campaign[0].final_contextual_primary_action_label == "COMBAT:attack"),
      service_process_gate: runner_gate("service_process_gate"),
      release_binary_gate: runner_gate("release_binary_gate"),
      classic_env_gate: runner_gate("classic_env_gate"),
      manifest_gate: runner_gate("manifest_gate"),
      override_dir_gate: runner_gate("override_dir_gate"),
      workdir_gate: runner_gate("workdir_gate"),
      cex_path_gate: runner_gate("cex_path_gate"),
      player_launch_ready_gate: (
        gate($campaign[0].green)
        and gate($runner[0].green)
        and (($campaign[0].campaign_slot_bytes // 0) > 20000)
        and ($campaign[0].final_contextual_primary_action_label == "COMBAT:attack")
      )
    },
    android_s5_real_device_claimed: false,
    public_launch_ready: false,
    public_launch_ready_claimed: false,
    source_of_truth: "A player-ready classic playtest launcher must expose CAMPAIGN title actions, persist and restore the campaign slot, resume into the Bevy-owned open-world state, and run on the live release trnm-world-bevy service with no CEX runtime path."
  }
  | .source_contract_count = ([.campaign_entry_contract, .runner_status_contract, .title_menu_contract, .state_snapshot_contract] | length)
  | .title_action_count = (.player_entry.title_actions | length)
  | .runner_cmdline_arg_count = (.live_runner.runtime.cmdline | length)
  | .runner_selected_environment_count = (.live_runner.runtime.selected_environment | keys | length)
  | .gate_count = (.gates | keys | length)
  | .passed_gate_count = ([.gates[] | select(. == true)] | length)
  | .failed_gate_count = ([.gates[] | select(. != true)] | length)' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_playtest_launcher_v1"
  and .campaign_entry_contract == "trillionnium_world_bevy_classic_rts_campaign_entry_v1"
  and .runner_status_contract == "trillionnium_world_bevy_classic_playtest_runner_status_v1"
  and .title_menu_contract == "trillionnium_world_bevy_title_menu_v1"
  and .state_snapshot_contract == "trillionnium_world_bevy_state_snapshot_v1"
  and .green == true
  and .source_contract_count == 4
  and .title_action_count == (.player_entry.title_actions | length)
  and .title_action_count == 3
  and .runner_cmdline_arg_count == (.live_runner.runtime.cmdline | length)
  and .runner_cmdline_arg_count == 2
  and .runner_selected_environment_count == (.live_runner.runtime.selected_environment | keys | length)
  and .gate_count == (.gates | keys | length)
  and .passed_gate_count == ([.gates[] | select(. == true)] | length)
  and .failed_gate_count == ([.gates[] | select(. != true)] | length)
  and .failed_gate_count == 0
  and (.player_entry.title_actions | index("CAMPAIGN:START") != null)
  and (.player_entry.title_actions | index("CAMPAIGN:CONTINUE") != null)
  and (.player_entry.title_actions | index("CAMPAIGN:REPLAY") != null)
  and .player_entry.input_action_count == 73
  and .player_entry.start_input_count == 73
  and .player_entry.replay_input_count == 73
  and .player_entry.campaign_slot_bytes > 20000
  and .player_entry.final_current_room_id == "league-coliseum"
  and .player_entry.final_map_scene == "arena_outdoor"
  and .player_entry.final_open_world_handoff_state == "resumed:league-coliseum"
  and .player_entry.final_contextual_primary_action_label == "COMBAT:attack"
  and .live_runner.service.active_state == "active"
  and .live_runner.service.sub_state == "running"
  and .live_runner.service.main_pid > 0
  and .gates.campaign_entry_gate == true
  and .gates.runner_status_gate == true
  and .gates.title_campaign_start_action_gate == true
  and .gates.title_campaign_continue_action_gate == true
  and .gates.title_campaign_replay_action_gate == true
  and .gates.campaign_slot_gate == true
  and .gates.open_world_resume_gate == true
  and .gates.player_command_gate == true
  and .gates.service_process_gate == true
  and .gates.release_binary_gate == true
  and .gates.classic_env_gate == true
  and .gates.manifest_gate == true
  and .gates.override_dir_gate == true
  and .gates.workdir_gate == true
  and .gates.cex_path_gate == true
  and .gates.player_launch_ready_gate == true
  and .android_s5_real_device_claimed == false
  and .public_launch_ready == false
  and .public_launch_ready_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_PLAYTEST_LAUNCHER_GREEN %s\n' "$SUMMARY"
