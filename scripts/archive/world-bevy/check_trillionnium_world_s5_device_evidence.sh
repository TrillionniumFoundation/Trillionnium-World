#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
if [[ -v TRILLIONNIUM_WORLD_S5_EVIDENCE_DIR && -n "$TRILLIONNIUM_WORLD_S5_EVIDENCE_DIR" ]]; then
  EVIDENCE_DIR="$TRILLIONNIUM_WORLD_S5_EVIDENCE_DIR"
fi
PACKAGE_NAME="world.trillionnium.nativebevy"
ACTIVITY_NAME="android.app.NativeActivity"
LIB_NAME="trnm_world_bevy"
ANDROID_API_LEVEL="24"
ANDROID_TARGET_SDK="34"
ANDROID_HOST_TAG="linux-x86_64"
REQUIRE_APK=0
REQUIRE_DEVICE=0

usage() {
  cat <<'EOF_USAGE'
Usage: scripts/check_trillionnium_world_s5_device_evidence.sh [--require-apk] [--require-device]

Builds the Android aarch64 Native/Bevy artifact, prepares a signed debug APK when
the Android platform/build tools are available, then collects real-device evidence
when adb sees an online Android device.

Outputs:
  acceptance/S5_native_bevy_device/latest/s5-device-evidence.json
  acceptance/S5_native_bevy_device/latest/screenshot.png
  acceptance/S5_native_bevy_device/latest/gfxinfo.txt
  acceptance/S5_native_bevy_device/latest/logcat.txt
  acceptance/S5_native_bevy_device/latest/lifecycle.txt
  acceptance/S5_native_bevy_device/latest/locale.txt
  acceptance/S5_native_bevy_device/latest/input-method.txt
  acceptance/S5_native_bevy_device/latest/weak-network.txt
  acceptance/S5_native_bevy_device/latest/apk-package-evidence.txt

Use --require-device for public-launch S5 collection so a missing adb device fails.
Set TRILLIONNIUM_WORLD_S5_WEAK_NETWORK_EVIDENCE_PATH to attach the operator's
real weak-network run evidence; a connectivity snapshot alone is not launch credit.
Validate the collected file with:
  scripts/check_trillionnium_world_s5_real_device_evidence.sh --require-ready
EOF_USAGE
}

for arg in "$@"; do
  case "$arg" in
    --help|-h) usage; exit 0 ;;
    --require-apk) REQUIRE_APK=1 ;;
    --require-device) REQUIRE_DEVICE=1 ;;
    *)
      printf 'Unknown argument: %s\n' "$arg" >&2
      exit 2
      ;;
  esac
done

mkdir -p "$EVIDENCE_DIR"
GENERATED_AT="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

require_cmd() {
  local name="$1"
  if ! command -v "$name" >/dev/null 2>&1; then
    printf 'Missing required command: %s\n' "$name" >&2
    exit 1
  fi
}

latest_matching_file() {
  find "$@" \( -type f -o -type l \) 2>/dev/null | sort -V | tail -n1
}

write_summary() {
  local overall_status="$1"
  local native_lib_status="$2"
  local apk_status="$3"
  local device_status="$4"
  local crash_status="$5"
  local apk_path="$6"
  local apk_sha256="$7"
  local apk_size_bytes="$8"
  local device_serial="$9"
  jq -n \
    --arg contract_version "trillionnium_world_s5_native_bevy_device_evidence_v1" \
    --arg generated_at "$GENERATED_AT" \
    --arg overall_status "$overall_status" \
    --arg package_name "$PACKAGE_NAME" \
    --arg activity_name "$ACTIVITY_NAME" \
    --arg lib_name "$LIB_NAME" \
    --arg android_target "aarch64-linux-android" \
    --arg android_api_level "$ANDROID_API_LEVEL" \
    --arg native_lib_status "$native_lib_status" \
    --arg native_lib_path "$NATIVE_LIB_PATH" \
    --arg native_lib_sha256 "$NATIVE_LIB_SHA256" \
    --arg native_lib_size_bytes "$NATIVE_LIB_SIZE_BYTES" \
    --arg symbol_evidence "$SYMBOL_EVIDENCE" \
    --arg apk_status "$apk_status" \
    --arg apk_path "$apk_path" \
    --arg apk_sha256 "$apk_sha256" \
    --arg apk_size_bytes "$apk_size_bytes" \
    --arg android_platform_jar "$ANDROID_PLATFORM_JAR" \
    --arg build_tools_dir "$BUILD_TOOLS_DIR" \
    --arg device_status "$device_status" \
    --arg device_serial "$device_serial" \
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
    --arg crash_status "$crash_status" \
    '{
      contract_version: $contract_version,
      generated_at: $generated_at,
      overall_status: $overall_status,
      package_name: $package_name,
      activity_name: $activity_name,
      lib_name: $lib_name,
      android_target: $android_target,
      android_api_level: ($android_api_level | tonumber),
      native_lib: {
        status: $native_lib_status,
        path: $native_lib_path,
        sha256: $native_lib_sha256,
        size_bytes: ($native_lib_size_bytes | tonumber),
        required_symbols: ["ANativeActivity_onCreate", "android_main"],
        evidence: $symbol_evidence
      },
      apk: {
        status: $apk_status,
        path: (if $apk_path == "" then null else $apk_path end),
        sha256: (if $apk_sha256 == "" then null else $apk_sha256 end),
        size_bytes: (if $apk_size_bytes == "" then null else ($apk_size_bytes | tonumber) end),
        android_platform_jar: (if $android_platform_jar == "" then null else $android_platform_jar end),
        build_tools_dir: (if $build_tools_dir == "" then null else $build_tools_dir end)
      },
      device_matrix: {
        status: $device_status,
        device_serial: (if $device_serial == "" then null else $device_serial end),
        adb_devices_evidence: $adb_devices_evidence,
        screenshot_evidence: (if $screenshot_evidence == "" then null else $screenshot_evidence end),
        gfxinfo_evidence: (if $gfxinfo_evidence == "" then null else $gfxinfo_evidence end),
        logcat_evidence: (if $logcat_evidence == "" then null else $logcat_evidence end),
        lifecycle_evidence: (if $lifecycle_evidence == "" then null else $lifecycle_evidence end),
        locale_evidence: (if $locale_evidence == "" then null else $locale_evidence end),
        input_method_evidence: (if $input_method_evidence == "" then null else $input_method_evidence end),
        weak_network_evidence: (if $weak_network_evidence == "" then null else $weak_network_evidence end),
        resource_pack_evidence: (if $resource_pack_evidence == "" then null else $resource_pack_evidence end),
        fps_gate: "requires_real_device_gfxinfo_or_frame_stats",
        render_gate: "requires_real_device_screenshot",
        touch_input_gate: "requires_adb_input_tap_evidence",
        lifecycle_gate: "requires_background_foreground_evidence",
        cjk_display_input_gate: $cjk_display_input_gate,
        weak_network_gate: $weak_network_gate,
        resource_pack_gate: $resource_pack_gate,
        crash_free_gate: $crash_status
      }
    }' >"$EVIDENCE_DIR/s5-device-evidence.json"
}

require_cmd cargo
require_cmd jq
require_cmd sha256sum
require_cmd adb

if [[ -v ANDROID_SDK_ROOT && -n "$ANDROID_SDK_ROOT" ]]; then
  ANDROID_SDK_ROOT="$ANDROID_SDK_ROOT"
elif [[ -v ANDROID_HOME && -n "$ANDROID_HOME" ]]; then
  ANDROID_SDK_ROOT="$ANDROID_HOME"
else
  ANDROID_SDK_ROOT="/usr/lib/android-sdk"
fi
if [[ -v ANDROID_NDK_ROOT && -n "$ANDROID_NDK_ROOT" ]]; then
  ANDROID_NDK_ROOT="$ANDROID_NDK_ROOT"
elif [[ -v ANDROID_NDK_HOME && -n "$ANDROID_NDK_HOME" ]]; then
  ANDROID_NDK_ROOT="$ANDROID_NDK_HOME"
else
  ANDROID_NDK_ROOT=""
fi
if [[ -z "$ANDROID_NDK_ROOT" ]]; then
  ANDROID_NDK_ROOT="$(find "$ANDROID_SDK_ROOT" /usr/lib/android-sdk /usr/lib -maxdepth 5 -type f -name ndk-build 2>/dev/null | head -n1 | xargs -r dirname || true)"
fi
if [[ "$(basename "$ANDROID_NDK_ROOT")" == "build" && -d "$(dirname "$ANDROID_NDK_ROOT")/toolchains" ]]; then
  ANDROID_NDK_ROOT="$(dirname "$ANDROID_NDK_ROOT")"
fi
if [[ -z "$ANDROID_NDK_ROOT" || ! -d "$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/$ANDROID_HOST_TAG/bin" ]]; then
  printf 'TRILLIONNIUM_WORLD_S5_DEVICE_EVIDENCE_FAILED missing Android NDK toolchain\n' >&2
  exit 1
fi

ANDROID_TOOLCHAIN_BIN="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/$ANDROID_HOST_TAG/bin"
ANDROID_AARCH64_CLANG="$ANDROID_TOOLCHAIN_BIN/aarch64-linux-android$ANDROID_API_LEVEL-clang"
ANDROID_AARCH64_CLANGXX="$ANDROID_TOOLCHAIN_BIN/aarch64-linux-android$ANDROID_API_LEVEL-clang++"
ANDROID_LLVM_AR="$ANDROID_TOOLCHAIN_BIN/llvm-ar"
ANDROID_LLVM_RANLIB="$ANDROID_TOOLCHAIN_BIN/llvm-ranlib"
ANDROID_LLVM_NM="$ANDROID_TOOLCHAIN_BIN/llvm-nm"

export ANDROID_NDK_HOME="$ANDROID_NDK_ROOT"
export CC_aarch64_linux_android="$ANDROID_AARCH64_CLANG"
export CXX_aarch64_linux_android="$ANDROID_AARCH64_CLANGXX"
export AR_aarch64_linux_android="$ANDROID_LLVM_AR"
export RANLIB_aarch64_linux_android="$ANDROID_LLVM_RANLIB"
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$ANDROID_AARCH64_CLANG"

if [[ -v ANDROID_BUILD_TOOLS_DIR && -n "$ANDROID_BUILD_TOOLS_DIR" ]]; then
  BUILD_TOOLS_DIR="$ANDROID_BUILD_TOOLS_DIR"
else
  BUILD_TOOLS_DIR=""
fi
if [[ -z "$BUILD_TOOLS_DIR" ]]; then
  BUILD_TOOLS_DIR="$(find "$ANDROID_SDK_ROOT" /usr/lib/android-sdk "$HOME/Android/Sdk" "$HOME/.android-sdk" -path '*/build-tools/*/aapt2' -type f 2>/dev/null | sed 's#/aapt2$##' | sort -V | tail -n1 || true)"
fi
AAPT2=""
APKSIGNER=""
ZIPALIGN=""
if [[ -n "$BUILD_TOOLS_DIR" ]]; then
  AAPT2="$BUILD_TOOLS_DIR/aapt2"
  APKSIGNER="$BUILD_TOOLS_DIR/apksigner"
  ZIPALIGN="$BUILD_TOOLS_DIR/zipalign"
fi

if [[ -v ANDROID_PLATFORM_JAR && -n "$ANDROID_PLATFORM_JAR" ]]; then
  ANDROID_PLATFORM_JAR="$ANDROID_PLATFORM_JAR"
else
  ANDROID_PLATFORM_JAR=""
fi
if [[ -z "$ANDROID_PLATFORM_JAR" ]]; then
  ANDROID_PLATFORM_JAR="$(latest_matching_file "$ANDROID_SDK_ROOT" /usr/lib/android-sdk "$HOME/Android/Sdk" "$HOME/.android-sdk" "$ROOT/target/android-platform-deb" -path '*/platforms/android-*/android.jar' || true)"
fi
if [[ -z "$ANDROID_PLATFORM_JAR" ]]; then
  ANDROID_PLATFORM_JAR="$(latest_matching_file "$ANDROID_SDK_ROOT" /usr/lib/android-sdk "$HOME/Android/Sdk" "$HOME/.android-sdk" "$ROOT/target/android-platform-deb" -name 'com.android.android-*.jar' || true)"
fi

(
  cd "$ROOT/trillionnium"
  cargo check -p trnm-world-bevy --lib --target aarch64-linux-android >"$EVIDENCE_DIR/aarch64-check.log" 2>&1
  cargo build -p trnm-world-bevy --lib --target aarch64-linux-android --release >"$EVIDENCE_DIR/aarch64-release-build.log" 2>&1
)

NATIVE_LIB_PATH="$ROOT/target/aarch64-linux-android/release/lib$LIB_NAME.so"
SYMBOL_EVIDENCE="$EVIDENCE_DIR/native-lib-symbols.txt"
if [[ ! -s "$NATIVE_LIB_PATH" ]]; then
  printf 'TRILLIONNIUM_WORLD_S5_DEVICE_EVIDENCE_FAILED missing native library %s\n' "$NATIVE_LIB_PATH" >&2
  exit 1
fi
"$ANDROID_LLVM_NM" -D "$NATIVE_LIB_PATH" >"$SYMBOL_EVIDENCE"
if ! grep -q 'ANativeActivity_onCreate' "$SYMBOL_EVIDENCE" || ! grep -q 'android_main' "$SYMBOL_EVIDENCE"; then
  printf 'TRILLIONNIUM_WORLD_S5_DEVICE_EVIDENCE_FAILED missing Android activity symbols\n' >&2
  exit 1
fi
NATIVE_LIB_SHA256="$(sha256sum "$NATIVE_LIB_PATH" | awk '{print $1}')"
NATIVE_LIB_SIZE_BYTES="$(stat -c '%s' "$NATIVE_LIB_PATH")"

APK_STATUS="blocked_missing_android_platform_jar"
APK_PATH=""
APK_SHA256=""
APK_SIZE_BYTES=""
if [[ -n "$ANDROID_PLATFORM_JAR" && -f "$ANDROID_PLATFORM_JAR" && -x "$AAPT2" && -x "$APKSIGNER" && -x "$ZIPALIGN" ]]; then
  require_cmd keytool
  require_cmd zip
  APK_BUILD_DIR="$ROOT/target/s5-native-bevy-apk"
  APK_PACKAGE_ROOT="$APK_BUILD_DIR/package-root"
  APK_UNSIGNED="$APK_BUILD_DIR/trillionnium-world-bevy-unsigned.apk"
  APK_ALIGNED="$APK_BUILD_DIR/trillionnium-world-bevy-aligned.apk"
  APK_SIGNED="$APK_BUILD_DIR/trillionnium-world-bevy-debug.apk"
  KEYSTORE="$APK_BUILD_DIR/debug.keystore"
  rm -rf "$APK_PACKAGE_ROOT"
  mkdir -p "$APK_PACKAGE_ROOT/lib/arm64-v8a" "$APK_BUILD_DIR"
  cp "$NATIVE_LIB_PATH" "$APK_PACKAGE_ROOT/lib/arm64-v8a/lib$LIB_NAME.so"
  cat >"$APK_BUILD_DIR/AndroidManifest.xml" <<EOF_MANIFEST
<manifest xmlns:android="http://schemas.android.com/apk/res/android"
    package="$PACKAGE_NAME"
    android:versionCode="1"
    android:versionName="0.1.0">
    <uses-sdk android:minSdkVersion="$ANDROID_API_LEVEL" android:targetSdkVersion="$ANDROID_TARGET_SDK" />
    <uses-feature android:glEsVersion="0x00030000" android:required="false" />
    <application
        android:label="Trillionnium World"
        android:debuggable="true"
        android:hasCode="false"
        android:theme="@android:style/Theme.Material.NoActionBar.Fullscreen">
        <activity
            android:name="$ACTIVITY_NAME"
            android:configChanges="keyboardHidden|orientation|screenLayout|screenSize|smallestScreenSize|uiMode"
            android:exported="true"
            android:screenOrientation="landscape">
            <meta-data android:name="android.app.lib_name" android:value="$LIB_NAME" />
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
        </activity>
    </application>
</manifest>
EOF_MANIFEST
  "$AAPT2" link -o "$APK_UNSIGNED" -I "$ANDROID_PLATFORM_JAR" --manifest "$APK_BUILD_DIR/AndroidManifest.xml"
  (cd "$APK_PACKAGE_ROOT" && zip -q -r "$APK_UNSIGNED" lib)
  "$ZIPALIGN" -f 4 "$APK_UNSIGNED" "$APK_ALIGNED"
  if [[ ! -s "$KEYSTORE" ]]; then
    keytool -genkeypair -v -keystore "$KEYSTORE" -storepass android -keypass android -alias androiddebugkey -keyalg RSA -keysize 2048 -validity 10000 -dname "CN=Trillionnium World,O=Trillionnium,C=CN" >/dev/null 2>&1
  fi
  "$APKSIGNER" sign --ks "$KEYSTORE" --ks-pass pass:android --key-pass pass:android --out "$APK_SIGNED" "$APK_ALIGNED"
  "$APKSIGNER" verify --print-certs "$APK_SIGNED" >"$EVIDENCE_DIR/apksigner-verify.txt"
  RESOURCE_PACK_EVIDENCE="$EVIDENCE_DIR/apk-package-evidence.txt"
  {
    printf 'apk_path=%s\n' "$APK_SIGNED"
    printf 'apk_sha256=%s\n' "$(sha256sum "$APK_SIGNED" | awk '{print $1}')"
    printf 'native_lib=%s\n' "lib/arm64-v8a/lib$LIB_NAME.so"
    "$APKSIGNER" verify --print-certs "$APK_SIGNED"
    if command -v aapt >/dev/null 2>&1; then
      aapt dump badging "$APK_SIGNED"
    fi
    unzip -l "$APK_SIGNED"
  } >"$RESOURCE_PACK_EVIDENCE" 2>&1 || true
  if [[ -s "$RESOURCE_PACK_EVIDENCE" ]]; then
    RESOURCE_PACK_GATE="apk_signature_resource_pack_evidence_collected"
  fi
  APK_STATUS="signed_debug_apk_ready"
  APK_PATH="$APK_SIGNED"
  APK_SHA256="$(sha256sum "$APK_SIGNED" | awk '{print $1}')"
  APK_SIZE_BYTES="$(stat -c '%s' "$APK_SIGNED")"
fi

ADB_DEVICES_EVIDENCE="$EVIDENCE_DIR/adb-devices.txt"
adb devices -l >"$ADB_DEVICES_EVIDENCE"
DEVICE_SERIAL="$(awk 'NR > 1 && $2 == "device" {print $1; exit}' "$ADB_DEVICES_EVIDENCE")"
DEVICE_STATUS="blocked_no_connected_android_device"
SCREENSHOT_EVIDENCE=""
GFXINFO_EVIDENCE=""
LOGCAT_EVIDENCE=""
LIFECYCLE_EVIDENCE=""
LOCALE_EVIDENCE=""
INPUT_METHOD_EVIDENCE=""
WEAK_NETWORK_EVIDENCE=""
CJK_DISPLAY_INPUT_GATE="requires_real_device_cjk_locale_input_evidence"
WEAK_NETWORK_GATE="requires_real_device_weak_network_run"
RESOURCE_PACK_EVIDENCE="${RESOURCE_PACK_EVIDENCE:-}"
RESOURCE_PACK_GATE="${RESOURCE_PACK_GATE:-requires_signed_apk_resource_package_evidence}"
CRASH_STATUS="not_run_no_device"

if [[ -n "$DEVICE_SERIAL" ]]; then
  if [[ "$APK_STATUS" != "signed_debug_apk_ready" ]]; then
    DEVICE_STATUS="blocked_apk_not_ready"
    CRASH_STATUS="not_run_apk_not_ready"
  else
    INSTALL_EVIDENCE="$EVIDENCE_DIR/adb-install.txt"
    LAUNCH_EVIDENCE="$EVIDENCE_DIR/adb-launch.txt"
    SCREENSHOT_EVIDENCE="$EVIDENCE_DIR/screenshot.png"
    GFXINFO_EVIDENCE="$EVIDENCE_DIR/gfxinfo.txt"
    LOGCAT_EVIDENCE="$EVIDENCE_DIR/logcat.txt"
    LIFECYCLE_EVIDENCE="$EVIDENCE_DIR/lifecycle.txt"
    LOCALE_EVIDENCE="$EVIDENCE_DIR/locale.txt"
    INPUT_METHOD_EVIDENCE="$EVIDENCE_DIR/input-method.txt"
    WEAK_NETWORK_EVIDENCE="$EVIDENCE_DIR/weak-network.txt"
    adb -s "$DEVICE_SERIAL" install -r "$APK_PATH" >"$INSTALL_EVIDENCE" 2>&1
    adb -s "$DEVICE_SERIAL" logcat -c || true
    adb -s "$DEVICE_SERIAL" shell monkey -p "$PACKAGE_NAME" 1 >"$LAUNCH_EVIDENCE" 2>&1
    sleep 8
    adb -s "$DEVICE_SERIAL" shell input tap 480 270 >>"$LIFECYCLE_EVIDENCE" 2>&1 || true
    adb -s "$DEVICE_SERIAL" shell input swipe 260 270 700 270 300 >>"$LIFECYCLE_EVIDENCE" 2>&1 || true
    adb -s "$DEVICE_SERIAL" shell screencap -p >"$SCREENSHOT_EVIDENCE" || true
    adb -s "$DEVICE_SERIAL" shell input keyevent HOME >>"$LIFECYCLE_EVIDENCE" 2>&1 || true
    sleep 2
    adb -s "$DEVICE_SERIAL" shell monkey -p "$PACKAGE_NAME" 1 >>"$LIFECYCLE_EVIDENCE" 2>&1 || true
    sleep 5
    adb -s "$DEVICE_SERIAL" shell dumpsys gfxinfo "$PACKAGE_NAME" >"$GFXINFO_EVIDENCE" 2>&1 || true
    {
      printf '[persist.sys.locale]\n'
      adb -s "$DEVICE_SERIAL" shell getprop persist.sys.locale || true
      printf '\n[ro.product.locale]\n'
      adb -s "$DEVICE_SERIAL" shell getprop ro.product.locale || true
      printf '\n[system_locales]\n'
      adb -s "$DEVICE_SERIAL" shell settings get system system_locales || true
    } >"$LOCALE_EVIDENCE" 2>&1 || true
    {
      printf '[default_input_method]\n'
      adb -s "$DEVICE_SERIAL" shell settings get secure default_input_method || true
      printf '\n[ime_list]\n'
      adb -s "$DEVICE_SERIAL" shell ime list -s || true
    } >"$INPUT_METHOD_EVIDENCE" 2>&1 || true
    {
      printf '[connectivity_snapshot]\n'
      adb -s "$DEVICE_SERIAL" shell dumpsys connectivity || true
    } >"$WEAK_NETWORK_EVIDENCE" 2>&1 || true
    if [[ -v TRILLIONNIUM_WORLD_S5_WEAK_NETWORK_EVIDENCE_PATH && -s "$TRILLIONNIUM_WORLD_S5_WEAK_NETWORK_EVIDENCE_PATH" ]]; then
      cp "$TRILLIONNIUM_WORLD_S5_WEAK_NETWORK_EVIDENCE_PATH" "$WEAK_NETWORK_EVIDENCE"
      WEAK_NETWORK_GATE="real_device_weak_network_run"
    else
      WEAK_NETWORK_GATE="connectivity_snapshot_only_requires_operator_weak_network_run"
    fi
    if [[ -s "$LOCALE_EVIDENCE" && -s "$INPUT_METHOD_EVIDENCE" ]]; then
      CJK_DISPLAY_INPUT_GATE="cjk_locale_input_snapshot_collected"
    fi
    adb -s "$DEVICE_SERIAL" logcat -d -v time >"$LOGCAT_EVIDENCE" 2>&1 || true
    if grep -Eiq 'FATAL EXCEPTION|AndroidRuntime|ANR in|SIGSEGV|Fatal signal' "$LOGCAT_EVIDENCE"; then
      DEVICE_STATUS="failed_crash_or_anr_detected"
      CRASH_STATUS="failed_crash_or_anr_detected"
    else
      DEVICE_STATUS="real_device_evidence_collected"
      CRASH_STATUS="crash_free_logcat_window"
    fi
  fi
fi

OVERALL_STATUS="blocked"
if [[ "$DEVICE_STATUS" == "real_device_evidence_collected" \
  && "$CJK_DISPLAY_INPUT_GATE" == "cjk_locale_input_snapshot_collected" \
  && "$WEAK_NETWORK_GATE" == "real_device_weak_network_run" \
  && "$RESOURCE_PACK_GATE" == "apk_signature_resource_pack_evidence_collected" ]]; then
  OVERALL_STATUS="ready"
elif [[ "$DEVICE_STATUS" == "real_device_evidence_collected" ]]; then
  OVERALL_STATUS="blocked_missing_s5_go_condition_evidence"
elif [[ "$APK_STATUS" == "signed_debug_apk_ready" && "$DEVICE_STATUS" == "blocked_no_connected_android_device" ]]; then
  OVERALL_STATUS="blocked_no_connected_android_device"
elif [[ "$APK_STATUS" != "signed_debug_apk_ready" ]]; then
  OVERALL_STATUS="$APK_STATUS"
fi

write_summary "$OVERALL_STATUS" "android_native_cdylib_ready" "$APK_STATUS" "$DEVICE_STATUS" "$CRASH_STATUS" "$APK_PATH" "$APK_SHA256" "$APK_SIZE_BYTES" "$DEVICE_SERIAL"

case "$OVERALL_STATUS" in
  ready)
    printf 'TRILLIONNIUM_WORLD_S5_DEVICE_EVIDENCE_READY\n'
    ;;
  blocked_no_connected_android_device|blocked_missing_android_platform_jar)
    if [[ "$REQUIRE_DEVICE" -eq 1 || "$REQUIRE_APK" -eq 1 && "$APK_STATUS" != "signed_debug_apk_ready" ]]; then
      printf 'TRILLIONNIUM_WORLD_S5_DEVICE_EVIDENCE_BLOCKED %s\n' "$OVERALL_STATUS"
      exit 1
    fi
    printf 'TRILLIONNIUM_WORLD_S5_DEVICE_EVIDENCE_BLOCKED %s\n' "$OVERALL_STATUS"
    ;;
  blocked_missing_s5_go_condition_evidence)
    printf 'TRILLIONNIUM_WORLD_S5_DEVICE_EVIDENCE_BLOCKED %s\n' "$OVERALL_STATUS"
    if [[ "$REQUIRE_DEVICE" -eq 1 ]]; then
      exit 1
    fi
    ;;
  *)
    printf 'TRILLIONNIUM_WORLD_S5_DEVICE_EVIDENCE_FAILED %s\n' "$OVERALL_STATUS" >&2
    exit 1
    ;;
esac
