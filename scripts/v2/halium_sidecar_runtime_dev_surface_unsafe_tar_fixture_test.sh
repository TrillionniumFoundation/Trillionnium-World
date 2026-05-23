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

printf 'unsafe traversal payload\n' >"$TMP/bad"
tar --transform='s#^bad$#../bad#' -cf "$TMP/unsafe-toolchain.tar" -C "$TMP" bad

set +e
HALIUM_ARM64_TOOLCHAIN_PAYLOAD="$TMP/unsafe-toolchain.tar" \
  HALIUM_ALLOW_PAYLOAD_EXECUTION=1 \
  "$GATE" >/dev/null 2>"$TMP/gate.err"
rc=$?
set -e

if [[ "$rc" -eq 0 ]]; then
  echo "[FAIL] unsafe tar payload unexpectedly passed" >&2
  exit 1
fi

jq -e '
  .contract_version == "trillionnium_world_halium_sidecar_runtime_dev_surface_v1"
  and .green == false
  and .operator_supplied_arm64_toolchain_payload.mode == "operator_supplied"
  and .operator_supplied_arm64_toolchain_payload.status == "operator_tar_path_traversal_rejected"
  and .operator_supplied_arm64_toolchain_payload.path_safety_checked == false
  and .operator_supplied_arm64_toolchain_payload.isolated_version_probe_executed == false
  and .gates.property_protocol_gate == true
  and .gates.service_shim_gate == true
  and .gates.payload_intake_gate == false
  and .source_policy.payload_path_traversal_rejected == true
  and .source_policy.unknown_payload_not_promoted_to_source == true
' "$OUT" >/dev/null

printf 'TRILLIONNIUM_WORLD_HALIUM_SIDECAR_UNSAFE_TAR_FIXTURE_REJECTED %s\n' "$OUT"
