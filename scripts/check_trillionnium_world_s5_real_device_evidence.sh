#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY_FILE="${TRILLIONNIUM_WORLD_S5_REAL_DEVICE_VALIDATION_SUMMARY:-$EVIDENCE_DIR/s5-real-device-evidence-validation.json}"
S5_EVIDENCE_PATH="${TRILLIONNIUM_WORLD_S5_REAL_DEVICE_EVIDENCE_PATH:-$EVIDENCE_DIR/s5-device-evidence.json}"
S5_TEMPLATE_FILE="$EVIDENCE_DIR/s5-device-evidence.template.json"
REQUIRE_READY=0

for arg in "$@"; do
  case "$arg" in
    --require-ready)
      REQUIRE_READY=1
      ;;
    *)
      printf 'unknown option: %s\n' "$arg" >&2
      exit 2
      ;;
  esac
done

mkdir -p "$EVIDENCE_DIR"

jq -n '{
  contract_version: "trillionnium_world_s5_native_bevy_device_evidence_v1",
  status: "template_requires_real_s5_device_evidence",
  acceptance_status: "s5_real_device_evidence_green",
  overall_status: "template_requires_real_device_run",
  android_target: "aarch64-linux-android",
  native_lib: {
    status: "android_native_cdylib_ready",
    path: null,
    evidence: null
  },
  apk: {
    status: "signed_debug_apk_ready",
    path: null
  },
  device_matrix: {
    status: "pending_real_device_evidence",
    device_serial: null,
    adb_devices_evidence: null,
    screenshot_evidence: null,
    gfxinfo_evidence: null,
    logcat_evidence: null,
    lifecycle_evidence: null,
    locale_evidence: null,
    input_method_evidence: null,
    weak_network_evidence: null,
    resource_pack_evidence: null,
    cjk_display_input_gate: "pending_cjk_locale_input_evidence",
    weak_network_gate: "pending_real_device_weak_network_run",
    resource_pack_gate: "pending_signed_apk_resource_pack_evidence",
    crash_free_gate: "pending_crash_free_logcat_window"
  },
  operator_signoff: {
    signed_by: null,
    signed_at: null,
    real_device_evidence_confirmed: false,
    synthetic_or_template_data_rejected: true
  },
  collection: {
    command: "ANDROID_SERIAL=<device-serial> scripts/check_trillionnium_world_s5_device_evidence.sh --require-device",
    validation_command: "TRILLIONNIUM_WORLD_S5_REAL_DEVICE_EVIDENCE_PATH=acceptance/S5_native_bevy_device/latest/s5-device-evidence.json scripts/check_trillionnium_world_s5_real_device_evidence.sh --require-ready",
    output_path: "acceptance/S5_native_bevy_device/latest/s5-device-evidence.json",
    requires_online_adb_device: true
  }
}' >"$S5_TEMPLATE_FILE"

read_json_field() {
  local path="$1"
  local expr="$2"
  if [[ -f "$path" ]]; then
    jq -r "$expr // empty" "$path" 2>/dev/null || true
  fi
}

file_status() {
  local path="$1"
  if [[ -n "$path" && -f "$path" ]]; then
    printf 'present'
  else
    printf 'missing'
  fi
}

file_nonempty() {
  local path="$1"
  if [[ -n "$path" && -s "$path" ]]; then
    printf 'true'
  else
    printf 'false'
  fi
}

json_array_from_lines() {
  jq -Rsc 'split("\n") | map(select(length > 0))'
}

BLOCKERS=()
EVIDENCE_FILE_STATUS="$(file_status "$S5_EVIDENCE_PATH")"
CONTRACT="$(read_json_field "$S5_EVIDENCE_PATH" '.contract_version')"
OVERALL_STATUS="$(read_json_field "$S5_EVIDENCE_PATH" '.overall_status')"
NATIVE_LIB_STATUS="$(read_json_field "$S5_EVIDENCE_PATH" '.native_lib.status')"
NATIVE_LIB_PATH="$(read_json_field "$S5_EVIDENCE_PATH" '.native_lib.path')"
NATIVE_LIB_SYMBOLS="$(read_json_field "$S5_EVIDENCE_PATH" '.native_lib.evidence')"
APK_STATUS="$(read_json_field "$S5_EVIDENCE_PATH" '.apk.status')"
APK_PATH="$(read_json_field "$S5_EVIDENCE_PATH" '.apk.path')"
DEVICE_STATUS="$(read_json_field "$S5_EVIDENCE_PATH" '.device_matrix.status')"
DEVICE_SERIAL="$(read_json_field "$S5_EVIDENCE_PATH" '.device_matrix.device_serial')"
ADB_DEVICES_EVIDENCE="$(read_json_field "$S5_EVIDENCE_PATH" '.device_matrix.adb_devices_evidence')"
SCREENSHOT_EVIDENCE="$(read_json_field "$S5_EVIDENCE_PATH" '.device_matrix.screenshot_evidence')"
GFXINFO_EVIDENCE="$(read_json_field "$S5_EVIDENCE_PATH" '.device_matrix.gfxinfo_evidence')"
LOGCAT_EVIDENCE="$(read_json_field "$S5_EVIDENCE_PATH" '.device_matrix.logcat_evidence')"
LIFECYCLE_EVIDENCE="$(read_json_field "$S5_EVIDENCE_PATH" '.device_matrix.lifecycle_evidence')"
LOCALE_EVIDENCE="$(read_json_field "$S5_EVIDENCE_PATH" '.device_matrix.locale_evidence')"
INPUT_METHOD_EVIDENCE="$(read_json_field "$S5_EVIDENCE_PATH" '.device_matrix.input_method_evidence')"
WEAK_NETWORK_EVIDENCE="$(read_json_field "$S5_EVIDENCE_PATH" '.device_matrix.weak_network_evidence')"
RESOURCE_PACK_EVIDENCE="$(read_json_field "$S5_EVIDENCE_PATH" '.device_matrix.resource_pack_evidence')"
CJK_DISPLAY_INPUT_GATE="$(read_json_field "$S5_EVIDENCE_PATH" '.device_matrix.cjk_display_input_gate')"
WEAK_NETWORK_GATE="$(read_json_field "$S5_EVIDENCE_PATH" '.device_matrix.weak_network_gate')"
RESOURCE_PACK_GATE="$(read_json_field "$S5_EVIDENCE_PATH" '.device_matrix.resource_pack_gate')"
CRASH_FREE_GATE="$(read_json_field "$S5_EVIDENCE_PATH" '.device_matrix.crash_free_gate')"
ANDROID_TARGET="$(read_json_field "$S5_EVIDENCE_PATH" '.android_target')"

[[ "$EVIDENCE_FILE_STATUS" == "present" ]] || BLOCKERS+=("s5_device_evidence_file")
[[ "$CONTRACT" == "trillionnium_world_s5_native_bevy_device_evidence_v1" ]] || BLOCKERS+=("s5_contract")
[[ "$OVERALL_STATUS" == "ready" || "$OVERALL_STATUS" == "real_device_evidence_green" ]] || BLOCKERS+=("s5_overall_ready")
[[ "$NATIVE_LIB_STATUS" == "android_native_cdylib_ready" ]] || BLOCKERS+=("android_native_cdylib_ready")
[[ "$ANDROID_TARGET" == "aarch64-linux-android" ]] || BLOCKERS+=("android_target_aarch64")
[[ "$(file_nonempty "$NATIVE_LIB_PATH")" == "true" ]] || BLOCKERS+=("native_lib_file_present")
[[ "$(file_nonempty "$NATIVE_LIB_SYMBOLS")" == "true" ]] || BLOCKERS+=("native_lib_symbol_evidence")
if [[ -s "$NATIVE_LIB_SYMBOLS" ]]; then
  grep -q 'ANativeActivity_onCreate' "$NATIVE_LIB_SYMBOLS" || BLOCKERS+=("symbol_anativeactivity_oncreate")
  grep -q 'android_main' "$NATIVE_LIB_SYMBOLS" || BLOCKERS+=("symbol_android_main")
fi
[[ "$APK_STATUS" == "signed_debug_apk_ready" ]] || BLOCKERS+=("signed_debug_apk_ready")
[[ "$(file_nonempty "$APK_PATH")" == "true" ]] || BLOCKERS+=("apk_file_present")
[[ "$DEVICE_STATUS" == "real_device_evidence_collected" || "$DEVICE_STATUS" == "real_device_evidence_green" ]] || BLOCKERS+=("real_device_evidence_collected")
[[ -n "$DEVICE_SERIAL" && "$DEVICE_SERIAL" != "null" ]] || BLOCKERS+=("real_device_serial")
[[ "$(file_nonempty "$ADB_DEVICES_EVIDENCE")" == "true" ]] || BLOCKERS+=("adb_devices_evidence")
if [[ -n "$DEVICE_SERIAL" && -s "$ADB_DEVICES_EVIDENCE" ]]; then
  grep -Fq "$DEVICE_SERIAL" "$ADB_DEVICES_EVIDENCE" || BLOCKERS+=("adb_devices_contains_serial")
fi
[[ "$(file_nonempty "$SCREENSHOT_EVIDENCE")" == "true" ]] || BLOCKERS+=("real_device_screenshot")
[[ "$(file_nonempty "$GFXINFO_EVIDENCE")" == "true" ]] || BLOCKERS+=("real_device_gfxinfo_or_frame_stats")
[[ "$(file_nonempty "$LOGCAT_EVIDENCE")" == "true" ]] || BLOCKERS+=("real_device_logcat")
[[ "$(file_nonempty "$LIFECYCLE_EVIDENCE")" == "true" ]] || BLOCKERS+=("real_device_lifecycle")
[[ "$(file_nonempty "$LOCALE_EVIDENCE")" == "true" ]] || BLOCKERS+=("real_device_cjk_locale")
[[ "$(file_nonempty "$INPUT_METHOD_EVIDENCE")" == "true" ]] || BLOCKERS+=("real_device_input_method")
[[ "$CJK_DISPLAY_INPUT_GATE" == "cjk_locale_input_snapshot_collected" || "$CJK_DISPLAY_INPUT_GATE" == "cjk_display_input_green" ]] || BLOCKERS+=("real_device_cjk_display_input")
[[ "$(file_nonempty "$WEAK_NETWORK_EVIDENCE")" == "true" ]] || BLOCKERS+=("real_device_weak_network_evidence")
[[ "$WEAK_NETWORK_GATE" == "real_device_weak_network_run" || "$WEAK_NETWORK_GATE" == "weak_network_green" ]] || BLOCKERS+=("real_device_weak_network")
[[ "$(file_nonempty "$RESOURCE_PACK_EVIDENCE")" == "true" ]] || BLOCKERS+=("android_resource_pack_evidence")
[[ "$RESOURCE_PACK_GATE" == "apk_signature_resource_pack_evidence_collected" || "$RESOURCE_PACK_GATE" == "resource_pack_green" ]] || BLOCKERS+=("android_resource_pack_gate")
[[ "$CRASH_FREE_GATE" == "crash_free_logcat_window" || "$CRASH_FREE_GATE" == "crash_free_green" || "$CRASH_FREE_GATE" == "no_crashes_detected" ]] || BLOCKERS+=("crash_free_logcat_window")
if [[ -s "$LOGCAT_EVIDENCE" ]] && grep -Eiq 'FATAL EXCEPTION|AndroidRuntime|ANR in|SIGSEGV|Fatal signal' "$LOGCAT_EVIDENCE"; then
  BLOCKERS+=("logcat_crash_or_anr_detected")
fi

BLOCKERS_JSON="$(printf '%s\n' "${BLOCKERS[@]}" | json_array_from_lines)"
STATUS="s5_real_device_evidence_green"
if [[ "$(jq 'length' <<<"$BLOCKERS_JSON")" != "0" ]]; then
  STATUS="blocked_missing_s5_real_device_evidence"
fi

jq -n \
  --arg contract_version "trillionnium_world_s5_real_device_evidence_gate_v1" \
  --arg status "$STATUS" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg evidence_path "$S5_EVIDENCE_PATH" \
  --arg evidence_file_status "$EVIDENCE_FILE_STATUS" \
  --arg evidence_contract "$CONTRACT" \
  --arg overall_status "$OVERALL_STATUS" \
  --arg native_lib_status "$NATIVE_LIB_STATUS" \
  --arg native_lib_path "$NATIVE_LIB_PATH" \
  --arg native_lib_symbols "$NATIVE_LIB_SYMBOLS" \
  --arg apk_status "$APK_STATUS" \
  --arg apk_path "$APK_PATH" \
  --arg device_status "$DEVICE_STATUS" \
  --arg device_serial "$DEVICE_SERIAL" \
  --arg adb_devices_evidence "$ADB_DEVICES_EVIDENCE" \
  --arg screenshot_evidence "$SCREENSHOT_EVIDENCE" \
  --arg gfxinfo_evidence "$GFXINFO_EVIDENCE" \
  --arg logcat_evidence "$LOGCAT_EVIDENCE" \
  --arg lifecycle_evidence "$LIFECYCLE_EVIDENCE" \
  --arg locale_evidence "$LOCALE_EVIDENCE" \
  --arg input_method_evidence "$INPUT_METHOD_EVIDENCE" \
  --arg weak_network_evidence "$WEAK_NETWORK_EVIDENCE" \
  --arg resource_pack_evidence "$RESOURCE_PACK_EVIDENCE" \
  --arg cjk_display_input_gate "$CJK_DISPLAY_INPUT_GATE" \
  --arg weak_network_gate "$WEAK_NETWORK_GATE" \
  --arg resource_pack_gate "$RESOURCE_PACK_GATE" \
  --arg crash_free_gate "$CRASH_FREE_GATE" \
  --arg template_path "$S5_TEMPLATE_FILE" \
  --arg template_sha256 "$(sha256sum "$S5_TEMPLATE_FILE" | awk '{print $1}')" \
  --argjson blockers "$BLOCKERS_JSON" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_s5_real_device_evidence_gate",
    accepted_status: "s5_real_device_evidence_green",
    android_s5_real_device_claimed: ($status == "s5_real_device_evidence_green"),
    host_side_replay_credit: false,
    template: {
      path: $template_path,
      sha256: $template_sha256,
      status: "template_requires_real_s5_device_evidence",
      public_launch_credit: false
    },
    operator_evidence: {
      path: $evidence_path,
      file_status: $evidence_file_status,
      contract_version: $evidence_contract,
      overall_status: $overall_status
    },
    native_lib: {
      status: $native_lib_status,
      path: $native_lib_path,
      symbols_evidence: $native_lib_symbols
    },
    apk: {
      status: $apk_status,
      path: $apk_path
    },
    real_device_matrix: {
      status: $device_status,
      device_serial: (if $device_serial == "" then null else $device_serial end),
      adb_devices_evidence: $adb_devices_evidence,
      screenshot_evidence: $screenshot_evidence,
      gfxinfo_evidence: $gfxinfo_evidence,
      logcat_evidence: $logcat_evidence,
      lifecycle_evidence: $lifecycle_evidence,
      locale_evidence: $locale_evidence,
      input_method_evidence: $input_method_evidence,
      weak_network_evidence: $weak_network_evidence,
      resource_pack_evidence: $resource_pack_evidence,
      cjk_display_input_gate: $cjk_display_input_gate,
      weak_network_gate: $weak_network_gate,
      resource_pack_gate: $resource_pack_gate,
      crash_free_gate: $crash_free_gate
    },
    go_condition_matrix: {
      cjk_display_input_gate: $cjk_display_input_gate,
      weak_network_gate: $weak_network_gate,
      resource_pack_gate: $resource_pack_gate,
      crash_free_gate: $crash_free_gate,
      accepted_cjk_display_input_gate: "cjk_locale_input_snapshot_collected",
      accepted_weak_network_gate: "real_device_weak_network_run",
      accepted_resource_pack_gate: "apk_signature_resource_pack_evidence_collected"
    },
    blockers: $blockers
  }' >"$SUMMARY_FILE"

if [[ "$STATUS" == "s5_real_device_evidence_green" ]]; then
  printf 'TRILLIONNIUM_WORLD_S5_REAL_DEVICE_EVIDENCE_READY %s\n' "$SUMMARY_FILE"
  exit 0
fi

printf 'TRILLIONNIUM_WORLD_S5_REAL_DEVICE_EVIDENCE_BLOCKED %s %s\n' "$STATUS" "$SUMMARY_FILE"
if [[ "$REQUIRE_READY" -eq 1 ]]; then
  exit 1
fi
