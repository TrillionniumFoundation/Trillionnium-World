#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
GATE="$ROOT/scripts/check_trillionnium_world_halium_sidecar_runtime_dev_surface.sh"
OUT="$ROOT/acceptance/S5_native_bevy_device/latest/halium-sidecar-runtime-dev-surface.json"
TMP="$(mktemp -d)"

cleanup() {
  local rc=$?
  rm -rf "$TMP"
  "$GATE" >/dev/null 2>&1 || true
  exit "$rc"
}
trap cleanup EXIT

mkdir -p "$TMP/operator-toolchain/bin"
cat >"$TMP/operator-toolchain/bin/aarch64-linux-android-clang" <<'PAYLOAD'
#!/usr/bin/env bash
set -euo pipefail
case "${1:-}" in
  --version)
    echo "operator-arm64-toolchain aarch64-linux-android runtime-dev-surface"
    ;;
  *)
    echo "operator payload fixture accepts --version only" >&2
    exit 64
    ;;
esac
PAYLOAD
chmod +x "$TMP/operator-toolchain/bin/aarch64-linux-android-clang"

HALIUM_ARM64_TOOLCHAIN_PAYLOAD="$TMP/operator-toolchain" \
  HALIUM_ALLOW_PAYLOAD_EXECUTION=1 \
  "$GATE" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_halium_sidecar_runtime_dev_surface_v1"
  and .green == true
  and .operator_supplied_arm64_toolchain_payload.mode == "operator_supplied"
  and .operator_supplied_arm64_toolchain_payload.status == "operator_directory_validated"
  and .operator_supplied_arm64_toolchain_payload.kind == "directory"
  and .operator_supplied_arm64_toolchain_payload.sha256 != ""
  and .operator_supplied_arm64_toolchain_payload.size_bytes > 0
  and .operator_supplied_arm64_toolchain_payload.path_safety_checked == true
  and .operator_supplied_arm64_toolchain_payload.isolated_version_probe_executed == true
  and (.operator_supplied_arm64_toolchain_payload.version_probe_output | contains("operator-arm64-toolchain"))
  and .gates.property_protocol_gate == true
  and .gates.service_shim_gate == true
  and .gates.payload_intake_gate == true
  and .source_policy.operator_supplied_payload_intake_supported == true
  and .source_policy.unknown_payload_not_promoted_to_source == true
' "$OUT" >/dev/null

printf 'TRILLIONNIUM_WORLD_HALIUM_SIDECAR_OPERATOR_PAYLOAD_FIXTURE_OK %s\n' "$OUT"
