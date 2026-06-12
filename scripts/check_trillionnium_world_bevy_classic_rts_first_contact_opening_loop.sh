#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-rts-first-contact-opening-loop.json"
mkdir -p "$(dirname "$OUT")"

(
  cd "$ROOT/trillionnium"
  CARGO_BUILD_JOBS=1 cargo run -p trnm-world-bevy -- classic-rts-first-contact-opening-loop >"$OUT"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_rts_first_contact_opening_loop_v1"
  and .green == true
  and .map_id == "first_contact_basin"
  and .flux_bank >= 300
  and .worker_cargo > 0
  and .worker_capacity == 12
  and .relay_build_progress >= 50
  and .beacon_capture_progress >= 40
  and .worker_train_progress >= 70
  and .scout_train_progress >= 30
  and .active_beacon_tile.x == 16
  and .active_beacon_tile.y == 9
  and .active_relay_tile.x == 11
  and .active_relay_tile.y == 8
  and .rts_data_opening_profile_contract == "trnm_rts_data_first_contact_opening_profile_v1"
  and .rts_data_command_feedback_contract == "trnm_rts_data_first_contact_command_feedback_v1"
  and .rts_data_opening_profile_gate == true
  and .rts_data_command_feedback_gate == true
  and .rts_data_command_feedback_target_tile.x == 16
  and .rts_data_command_feedback_target_tile.y == 9
  and .rts_data_command_feedback_blocked_tile.x == 15
  and .rts_data_command_feedback_blocked_tile.y == 16
  and .opening_actions == [
    "worker_harvest_flux",
    "build_flux_relay",
    "train_worker",
    "train_horizon_scout",
    "secure_flux_beacon"
  ]
  and .economy_gate == true
  and .production_gate == true
  and .build_gate == true
  and .objective_gate == true
  and .runtime_gate == true
' "$OUT" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_FIRST_CONTACT_OPENING_LOOP_GREEN %s\n' "$OUT"
