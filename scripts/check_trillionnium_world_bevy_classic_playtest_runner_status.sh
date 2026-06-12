#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SUMMARY="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-runner-status.json"
SERVICE="${TRNM_WORLD_BEVY_PLAYTEST_SERVICE:-trillionnium-bevy-playtest.service}"
EXPECTED_BINARY="$ROOT/target/release/trnm-world-bevy"
EXPECTED_REPO_ROOT="$ROOT"
EXPECTED_CWD="$ROOT/trillionnium"
EXPECTED_MANIFEST="$ROOT/assets/trnm-world/classic/manifest.json"
EXPECTED_OVERRIDE_DIR="$ROOT/assets/trnm-world/classic/art-pack-v1"

mkdir -p "$(dirname "$SUMMARY")"

systemctl_value() {
  systemctl --user show "$SERVICE" --property="$1" --value 2>/dev/null || true
}

proc_env_value() {
  local pid="$1"
  local key="$2"
  if [[ -r "/proc/$pid/environ" ]]; then
    tr '\0' '\n' <"/proc/$pid/environ" | awk -F= -v key="$key" '
      $1 == key {
        sub(/^[^=]*=/, "")
        print
        exit
      }
    '
  fi
}

ACTIVE_STATE="$(systemctl_value ActiveState)"
SUB_STATE="$(systemctl_value SubState)"
MAIN_PID_RAW="$(systemctl_value MainPID)"
EXEC_MAIN_STATUS="$(systemctl_value ExecMainStatus)"

MAIN_PID=0
if [[ "$MAIN_PID_RAW" =~ ^[0-9]+$ ]]; then
  MAIN_PID="$MAIN_PID_RAW"
fi

CMDLINE_JSON='[]'
CMDLINE_JOINED=""
PROCESS_CWD=""
LOW_SPEC_VALUE=""
CLASSIC_RENDERER_VALUE=""
CLASSIC_FPS_VALUE=""
CLASSIC_MANIFEST_VALUE=""
CLASSIC_OVERRIDE_DIR_VALUE=""
PLAYER_SCREEN_VALUE=""
WINIT_UNIX_BACKEND_VALUE=""
WAYLAND_DISPLAY_VALUE=""

if [[ "$MAIN_PID" -gt 0 && -d "/proc/$MAIN_PID" ]]; then
  if [[ -r "/proc/$MAIN_PID/cmdline" ]]; then
    CMDLINE_JSON="$(tr '\0' '\n' <"/proc/$MAIN_PID/cmdline" | awk 'NF > 0' | jq -R . | jq -s .)"
    CMDLINE_JOINED="$(jq -r 'join(" ")' <<<"$CMDLINE_JSON")"
  fi
  PROCESS_CWD="$(readlink -f "/proc/$MAIN_PID/cwd" 2>/dev/null || true)"
  LOW_SPEC_VALUE="$(proc_env_value "$MAIN_PID" TRNM_WORLD_BEVY_LOW_SPEC)"
  CLASSIC_RENDERER_VALUE="$(proc_env_value "$MAIN_PID" TRNM_WORLD_BEVY_CLASSIC_RENDERER)"
  CLASSIC_FPS_VALUE="$(proc_env_value "$MAIN_PID" TRNM_WORLD_BEVY_CLASSIC_FPS)"
  CLASSIC_MANIFEST_VALUE="$(proc_env_value "$MAIN_PID" TRNM_WORLD_BEVY_CLASSIC_ASSET_MANIFEST)"
  CLASSIC_OVERRIDE_DIR_VALUE="$(proc_env_value "$MAIN_PID" TRNM_WORLD_BEVY_CLASSIC_ASSET_OVERRIDE_DIR)"
  PLAYER_SCREEN_VALUE="$(proc_env_value "$MAIN_PID" TRNM_WORLD_BEVY_CLASSIC_PLAYER_SCREEN)"
  WINIT_UNIX_BACKEND_VALUE="$(proc_env_value "$MAIN_PID" WINIT_UNIX_BACKEND)"
  WAYLAND_DISPLAY_VALUE="$(proc_env_value "$MAIN_PID" WAYLAND_DISPLAY)"
fi

CMD0="$(jq -r '.[0] // ""' <<<"$CMDLINE_JSON")"
HAS_RUN_ARG="$(jq -r 'index("run") != null' <<<"$CMDLINE_JSON")"
ENV_JSON="$(jq -n \
  --arg low_spec "$LOW_SPEC_VALUE" \
  --arg classic_renderer "$CLASSIC_RENDERER_VALUE" \
  --arg classic_fps "$CLASSIC_FPS_VALUE" \
  --arg classic_asset_manifest "$CLASSIC_MANIFEST_VALUE" \
  --arg classic_asset_override_dir "$CLASSIC_OVERRIDE_DIR_VALUE" \
  --arg classic_player_screen "$PLAYER_SCREEN_VALUE" \
  --arg winit_unix_backend "$WINIT_UNIX_BACKEND_VALUE" \
  --arg wayland_display "$WAYLAND_DISPLAY_VALUE" \
  '{
    TRNM_WORLD_BEVY_LOW_SPEC: $low_spec,
    TRNM_WORLD_BEVY_CLASSIC_RENDERER: $classic_renderer,
    TRNM_WORLD_BEVY_CLASSIC_FPS: $classic_fps,
    TRNM_WORLD_BEVY_CLASSIC_ASSET_MANIFEST: $classic_asset_manifest,
    TRNM_WORLD_BEVY_CLASSIC_ASSET_OVERRIDE_DIR: $classic_asset_override_dir,
    TRNM_WORLD_BEVY_CLASSIC_PLAYER_SCREEN: $classic_player_screen,
    WINIT_UNIX_BACKEND: $winit_unix_backend,
    WAYLAND_DISPLAY: $wayland_display
  }')"

SERVICE_PROCESS_GATE=false
if [[ "$ACTIVE_STATE" == "active" && "$SUB_STATE" == "running" && "$MAIN_PID" -gt 0 && -d "/proc/$MAIN_PID" ]]; then
  SERVICE_PROCESS_GATE=true
fi

RELEASE_BINARY_GATE=false
if [[ "$CMD0" == "$EXPECTED_BINARY" && -x "$EXPECTED_BINARY" && "$HAS_RUN_ARG" == "true" ]]; then
  RELEASE_BINARY_GATE=true
fi

CLASSIC_ENV_GATE=false
if [[ "$LOW_SPEC_VALUE" == "1" && "$CLASSIC_RENDERER_VALUE" == "1" && "$CLASSIC_FPS_VALUE" == "30" ]]; then
  CLASSIC_ENV_GATE=true
fi

PLAYER_SCREEN_ENV_GATE=false
if [[ "$PLAYER_SCREEN_VALUE" == "1" ]]; then
  PLAYER_SCREEN_ENV_GATE=true
fi

X11_BACKEND_GATE=false
if [[ "$WINIT_UNIX_BACKEND_VALUE" == "x11" && -z "$WAYLAND_DISPLAY_VALUE" ]]; then
  X11_BACKEND_GATE=true
fi

MANIFEST_GATE=false
if [[ "$CLASSIC_MANIFEST_VALUE" == "$EXPECTED_MANIFEST" && -f "$EXPECTED_MANIFEST" ]]; then
  MANIFEST_GATE=true
fi

OVERRIDE_DIR_GATE=false
if [[ "$CLASSIC_OVERRIDE_DIR_VALUE" == "$EXPECTED_OVERRIDE_DIR" && -d "$EXPECTED_OVERRIDE_DIR" ]]; then
  OVERRIDE_DIR_GATE=true
fi

WORKDIR_GATE=false
if [[ "$PROCESS_CWD" == "$EXPECTED_CWD" ]]; then
  WORKDIR_GATE=true
fi

COMBINED_RUNTIME_PATHS="$(printf '%s %s %s %s %s' "$CMDLINE_JOINED" "$PROCESS_CWD" "$CLASSIC_MANIFEST_VALUE" "$CLASSIC_OVERRIDE_DIR_VALUE" "$CMD0")"
CEX_PATH_GATE=true
if grep -qiE '(^|[[:space:]])/[^[:space:]]*/CEX(/|[[:space:]]|$)|(^|[[:space:]])/[^[:space:]]*/cex(/|[[:space:]]|$)' <<<"$COMBINED_RUNTIME_PATHS"; then
  CEX_PATH_GATE=false
fi

GREEN=false
if [[ "$SERVICE_PROCESS_GATE" == "true" && "$RELEASE_BINARY_GATE" == "true" && "$CLASSIC_ENV_GATE" == "true" && "$PLAYER_SCREEN_ENV_GATE" == "true" && "$X11_BACKEND_GATE" == "true" && "$MANIFEST_GATE" == "true" && "$OVERRIDE_DIR_GATE" == "true" && "$WORKDIR_GATE" == "true" && "$CEX_PATH_GATE" == "true" ]]; then
  GREEN=true
fi

STATUS=blocked
if [[ "$GREEN" == "true" ]]; then
  STATUS=green
fi

MANIFEST_SHA256=""
if [[ -f "$EXPECTED_MANIFEST" ]]; then
  MANIFEST_SHA256="$(sha256sum "$EXPECTED_MANIFEST" | awk '{print $1}')"
fi

jq -n \
  --arg contract_version "trillionnium_world_bevy_classic_playtest_runner_status_v1" \
  --arg status "$STATUS" \
  --arg service "$SERVICE" \
  --arg active_state "$ACTIVE_STATE" \
  --arg sub_state "$SUB_STATE" \
  --arg main_pid "$MAIN_PID" \
  --arg exec_main_status "$EXEC_MAIN_STATUS" \
  --arg expected_binary "$EXPECTED_BINARY" \
  --arg expected_repo_root "$EXPECTED_REPO_ROOT" \
  --arg expected_cwd "$EXPECTED_CWD" \
  --arg process_cwd "$PROCESS_CWD" \
  --arg expected_manifest "$EXPECTED_MANIFEST" \
  --arg expected_override_dir "$EXPECTED_OVERRIDE_DIR" \
  --arg manifest_sha256 "$MANIFEST_SHA256" \
  --argjson green "$GREEN" \
  --argjson cmdline "$CMDLINE_JSON" \
  --argjson selected_environment "$ENV_JSON" \
  --argjson service_process_gate "$SERVICE_PROCESS_GATE" \
  --argjson release_binary_gate "$RELEASE_BINARY_GATE" \
  --argjson classic_env_gate "$CLASSIC_ENV_GATE" \
  --argjson player_screen_env_gate "$PLAYER_SCREEN_ENV_GATE" \
  --argjson x11_backend_gate "$X11_BACKEND_GATE" \
  --argjson manifest_gate "$MANIFEST_GATE" \
  --argjson override_dir_gate "$OVERRIDE_DIR_GATE" \
  --argjson workdir_gate "$WORKDIR_GATE" \
  --argjson cex_path_gate "$CEX_PATH_GATE" \
  '{
    contract_version: $contract_version,
    status: $status,
    green: $green,
    service: {
      unit: $service,
      active_state: $active_state,
      sub_state: $sub_state,
      main_pid: ($main_pid | tonumber),
      exec_main_status: $exec_main_status
    },
    runtime: {
      expected_binary: $expected_binary,
      expected_repo_root: $expected_repo_root,
      expected_cwd: $expected_cwd,
      process_cwd: $process_cwd,
      expected_manifest: $expected_manifest,
      expected_override_dir: $expected_override_dir,
      manifest_sha256: (if $manifest_sha256 == "" then null else $manifest_sha256 end),
      cmdline: $cmdline,
      selected_environment: $selected_environment
    },
    gates: {
      service_process_gate: $service_process_gate,
      release_binary_gate: $release_binary_gate,
      classic_env_gate: $classic_env_gate,
      player_screen_env_gate: $player_screen_env_gate,
      x11_backend_gate: $x11_backend_gate,
      manifest_gate: $manifest_gate,
      override_dir_gate: $override_dir_gate,
      workdir_gate: $workdir_gate,
      cex_path_gate: $cex_path_gate
    },
    source_of_truth: "The live playtest runner must be the release trnm-world-bevy binary with the low-spec classic player screen, X11 backend, and classic renderer manifest; CEX paths are explicitly rejected."
  }' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_bevy_classic_playtest_runner_status_v1"
  and .green == true
  and .service.unit == "trillionnium-bevy-playtest.service"
  and .service.active_state == "active"
  and .service.sub_state == "running"
  and .service.main_pid > 0
  and (.runtime.cmdline[0] | contains("/target/release/trnm-world-bevy"))
  and (.runtime.cmdline | index("run") != null)
  and .runtime.selected_environment.TRNM_WORLD_BEVY_LOW_SPEC == "1"
  and .runtime.selected_environment.TRNM_WORLD_BEVY_CLASSIC_RENDERER == "1"
  and .runtime.selected_environment.TRNM_WORLD_BEVY_CLASSIC_FPS == "30"
  and .runtime.selected_environment.TRNM_WORLD_BEVY_CLASSIC_PLAYER_SCREEN == "1"
  and .runtime.selected_environment.WINIT_UNIX_BACKEND == "x11"
  and .runtime.selected_environment.WAYLAND_DISPLAY == ""
  and (.runtime.selected_environment.TRNM_WORLD_BEVY_CLASSIC_ASSET_MANIFEST | contains("/assets/trnm-world/classic/manifest.json"))
  and (.runtime.selected_environment.TRNM_WORLD_BEVY_CLASSIC_ASSET_OVERRIDE_DIR | contains("/assets/trnm-world/classic/art-pack-v1"))
  and .gates.service_process_gate == true
  and .gates.release_binary_gate == true
  and .gates.classic_env_gate == true
  and .gates.player_screen_env_gate == true
  and .gates.x11_backend_gate == true
  and .gates.manifest_gate == true
  and .gates.override_dir_gate == true
  and .gates.workdir_gate == true
  and .gates.cex_path_gate == true
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_CLASSIC_PLAYTEST_RUNNER_STATUS_GREEN %s\n' "$SUMMARY"
