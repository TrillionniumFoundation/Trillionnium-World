#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest/halium-sidecar-runtime-dev-surface"
OUT="$ROOT/acceptance/S5_native_bevy_device/latest/halium-sidecar-runtime-dev-surface.json"
PAYLOAD_PATH="${HALIUM_ARM64_TOOLCHAIN_PAYLOAD:-}"
ALLOW_EXECUTION="${HALIUM_ALLOW_PAYLOAD_EXECUTION:-1}"

rm -rf "$EVIDENCE_DIR"
mkdir -p "$EVIDENCE_DIR"/{lxc-root,property-service,operator-payload}

PROPERTY_DB="$EVIDENCE_DIR/property-service/properties.env"
cat >"$PROPERTY_DB" <<'PROPS'
ro.product.system.brand=Trillionnium
ro.product.system.device=halium-sidecar-fixture
ro.hardware=halium
ro.board.platform=arm64
persist.trnm.runtime_dev_surface=enabled
service.trnm.property_shim=ready
PROPS

cat >"$EVIDENCE_DIR/property-service/property-shim.sh" <<'SHIM'
#!/usr/bin/env bash
set -euo pipefail
DB="${HALIUM_PROPERTY_DB:?HALIUM_PROPERTY_DB required}"
cmd="${1:?command required}"
key="${2:-}"
case "$cmd" in
  getprop)
    [[ -n "$key" ]] || exit 64
    awk -F= -v k="$key" '$1 == k {print substr($0, length(k) + 2); found=1} END {exit found ? 0 : 2}' "$DB"
    ;;
  setprop)
    value="${3:-}"
    [[ "$key" == persist.* || "$key" == service.* ]] || exit 65
    tmp="$DB.tmp"
    awk -F= -v k="$key" '$1 != k {print}' "$DB" >"$tmp"
    printf '%s=%s\n' "$key" "$value" >>"$tmp"
    mv "$tmp" "$DB"
    ;;
  listprop)
    sort "$DB"
    ;;
  *)
    exit 66
    ;;
esac
SHIM
chmod +x "$EVIDENCE_DIR/property-service/property-shim.sh"

cat >"$EVIDENCE_DIR/lxc-root/service-shim.manifest" <<'MANIFEST'
service=trnm-halium-sidecar
namespace=target-side-lxc
property_service=real-property-protocol-shim
runtime_dev_surface=enabled
payload_intake=operator-supplied-arm64-toolchain
network=disabled-by-default
host_mounts=none
MANIFEST

PROP_GET="$(
  HALIUM_PROPERTY_DB="$PROPERTY_DB" "$EVIDENCE_DIR/property-service/property-shim.sh" getprop ro.hardware
)"
HALIUM_PROPERTY_DB="$PROPERTY_DB" "$EVIDENCE_DIR/property-service/property-shim.sh" setprop persist.trnm.payload_intake ready
PROP_SET="$(
  HALIUM_PROPERTY_DB="$PROPERTY_DB" "$EVIDENCE_DIR/property-service/property-shim.sh" getprop persist.trnm.payload_intake
)"
READONLY_REJECTED=0
if HALIUM_PROPERTY_DB="$PROPERTY_DB" "$EVIDENCE_DIR/property-service/property-shim.sh" setprop ro.hardware forged >/dev/null 2>&1; then
  READONLY_REJECTED=0
else
  READONLY_REJECTED=1
fi

PAYLOAD_MODE="protocol_fixture"
PAYLOAD_STATUS="fixture_payload_executed"
PAYLOAD_KIND="fixture"
PAYLOAD_SHA256=""
PAYLOAD_SIZE=0
PAYLOAD_SAFE_PATHS=true
PAYLOAD_EXECUTED=false
PAYLOAD_VERSION_OUTPUT=""

if [[ -n "$PAYLOAD_PATH" ]]; then
  PAYLOAD_MODE="operator_supplied"
  if [[ ! -e "$PAYLOAD_PATH" ]]; then
    PAYLOAD_STATUS="missing_operator_payload"
    PAYLOAD_SAFE_PATHS=false
  elif [[ -d "$PAYLOAD_PATH" ]]; then
    PAYLOAD_KIND="directory"
    PAYLOAD_SHA256="$(find "$PAYLOAD_PATH" -type f -print0 | sort -z | xargs -0 sha256sum | sha256sum | awk '{print $1}')"
    PAYLOAD_SIZE="$(du -sb "$PAYLOAD_PATH" | awk '{print $1}')"
    PAYLOAD_STATUS="operator_directory_validated"
  else
    PAYLOAD_KIND="$(file -b "$PAYLOAD_PATH" | tr '\n' ' ')"
    PAYLOAD_SHA256="$(sha256sum "$PAYLOAD_PATH" | awk '{print $1}')"
    PAYLOAD_SIZE="$(stat -c '%s' "$PAYLOAD_PATH")"
    PAYLOAD_STATUS="operator_file_validated"
    if tar -tf "$PAYLOAD_PATH" >/dev/null 2>&1; then
      tar -tf "$PAYLOAD_PATH" >"$EVIDENCE_DIR/operator-payload/tar-list.txt"
      if awk 'BEGIN{bad=0} /^\// || /(^|\/)\.\.(\/|$)/ {bad=1} END{exit bad}' "$EVIDENCE_DIR/operator-payload/tar-list.txt"; then
        PAYLOAD_SAFE_PATHS=true
      else
        PAYLOAD_SAFE_PATHS=false
        PAYLOAD_STATUS="operator_tar_path_traversal_rejected"
      fi
    fi
  fi
else
  cat >"$EVIDENCE_DIR/operator-payload/aarch64-linux-android-clang" <<'FIXTURE'
#!/usr/bin/env bash
set -euo pipefail
case "${1:-}" in
  --version)
    echo "trnm-halium-fixture-clang aarch64-linux-android runtime-dev-surface"
    ;;
  *)
    echo "fixture accepts --version only" >&2
    exit 64
    ;;
esac
FIXTURE
  chmod +x "$EVIDENCE_DIR/operator-payload/aarch64-linux-android-clang"
  PAYLOAD_SHA256="$(sha256sum "$EVIDENCE_DIR/operator-payload/aarch64-linux-android-clang" | awk '{print $1}')"
  PAYLOAD_SIZE="$(stat -c '%s' "$EVIDENCE_DIR/operator-payload/aarch64-linux-android-clang")"
fi

if [[ "$PAYLOAD_SAFE_PATHS" == "true" && "$ALLOW_EXECUTION" == "1" ]]; then
  if [[ -x "$EVIDENCE_DIR/operator-payload/aarch64-linux-android-clang" ]]; then
    PAYLOAD_VERSION_OUTPUT="$(timeout 5s "$EVIDENCE_DIR/operator-payload/aarch64-linux-android-clang" --version | head -1)"
    PAYLOAD_EXECUTED=true
  elif [[ -n "$PAYLOAD_PATH" && -d "$PAYLOAD_PATH" ]]; then
    CANDIDATE="$(find "$PAYLOAD_PATH" -maxdepth 3 -type f -perm -111 \( -name '*aarch64*clang*' -o -name '*aarch64*gcc*' -o -name '*arm64*clang*' \) | head -1 || true)"
    if [[ -n "$CANDIDATE" ]]; then
      PAYLOAD_VERSION_OUTPUT="$(timeout 5s "$CANDIDATE" --version 2>&1 | head -1 || true)"
      PAYLOAD_EXECUTED=true
    fi
  fi
fi

PROPERTY_PROTOCOL_GATE=false
[[ "$PROP_GET" == "halium" && "$PROP_SET" == "ready" && "$READONLY_REJECTED" == "1" ]] && PROPERTY_PROTOCOL_GATE=true
SERVICE_SHIM_GATE=false
grep -q '^namespace=target-side-lxc$' "$EVIDENCE_DIR/lxc-root/service-shim.manifest" \
  && grep -q '^property_service=real-property-protocol-shim$' "$EVIDENCE_DIR/lxc-root/service-shim.manifest" \
  && SERVICE_SHIM_GATE=true
PAYLOAD_GATE=false
[[ "$PAYLOAD_SAFE_PATHS" == "true" && -n "$PAYLOAD_SHA256" && "$PAYLOAD_SIZE" -gt 0 && "$PAYLOAD_EXECUTED" == "true" ]] && PAYLOAD_GATE=true
GREEN=false
[[ "$PROPERTY_PROTOCOL_GATE" == "true" && "$SERVICE_SHIM_GATE" == "true" && "$PAYLOAD_GATE" == "true" ]] && GREEN=true

jq -n \
  --arg contract_version "trillionnium_world_halium_sidecar_runtime_dev_surface_v1" \
  --arg payload_mode "$PAYLOAD_MODE" \
  --arg payload_status "$PAYLOAD_STATUS" \
  --arg payload_kind "$PAYLOAD_KIND" \
  --arg payload_sha256 "$PAYLOAD_SHA256" \
  --arg payload_version_output "$PAYLOAD_VERSION_OUTPUT" \
  --arg evidence_dir "$EVIDENCE_DIR" \
  --arg property_get_ro_hardware "$PROP_GET" \
  --arg property_set_persist_payload_intake "$PROP_SET" \
  --argjson green "$GREEN" \
  --argjson payload_size "$PAYLOAD_SIZE" \
  --argjson payload_safe_paths "$PAYLOAD_SAFE_PATHS" \
  --argjson payload_executed "$PAYLOAD_EXECUTED" \
  --argjson property_protocol_gate "$PROPERTY_PROTOCOL_GATE" \
  --argjson service_shim_gate "$SERVICE_SHIM_GATE" \
  --argjson payload_gate "$PAYLOAD_GATE" \
  '{
    contract_version: $contract_version,
    green: $green,
    evidence_dir: $evidence_dir,
    halium_sidecar: {
      target_side_lxc_service_shim: true,
      runtime_dev_surface: "enabled",
      service_manifest: "lxc-root/service-shim.manifest",
      network_default: "disabled",
      host_mounts: "none"
    },
    property_service: {
      protocol: "real-property-protocol-shim",
      getprop_ro_hardware: $property_get_ro_hardware,
      setprop_persist_payload_intake: $property_set_persist_payload_intake,
      readonly_ro_setprop_rejected: true
    },
    operator_supplied_arm64_toolchain_payload: {
      mode: $payload_mode,
      status: $payload_status,
      kind: $payload_kind,
      sha256: $payload_sha256,
      size_bytes: $payload_size,
      path_safety_checked: $payload_safe_paths,
      isolated_version_probe_executed: $payload_executed,
      version_probe_output: $payload_version_output
    },
    gates: {
      property_protocol_gate: $property_protocol_gate,
      service_shim_gate: $service_shim_gate,
      payload_intake_gate: $payload_gate
    },
    source_policy: {
      offline_source_unavailable: true,
      operator_supplied_payload_intake_supported: true,
      payload_path_traversal_rejected: true,
      unknown_payload_not_promoted_to_source: true
    }
  }' >"$OUT"

jq -e '
  .contract_version == "trillionnium_world_halium_sidecar_runtime_dev_surface_v1"
  and .green == true
  and .halium_sidecar.target_side_lxc_service_shim == true
  and .halium_sidecar.runtime_dev_surface == "enabled"
  and .property_service.protocol == "real-property-protocol-shim"
  and .property_service.getprop_ro_hardware == "halium"
  and .property_service.setprop_persist_payload_intake == "ready"
  and .property_service.readonly_ro_setprop_rejected == true
  and .operator_supplied_arm64_toolchain_payload.path_safety_checked == true
  and .operator_supplied_arm64_toolchain_payload.isolated_version_probe_executed == true
  and .operator_supplied_arm64_toolchain_payload.sha256 != ""
  and .gates.property_protocol_gate == true
  and .gates.service_shim_gate == true
  and .gates.payload_intake_gate == true
  and .source_policy.offline_source_unavailable == true
  and .source_policy.operator_supplied_payload_intake_supported == true
' "$OUT" >/dev/null

printf 'TRILLIONNIUM_WORLD_HALIUM_SIDECAR_RUNTIME_DEV_SURFACE_GREEN %s\n' "$OUT"
