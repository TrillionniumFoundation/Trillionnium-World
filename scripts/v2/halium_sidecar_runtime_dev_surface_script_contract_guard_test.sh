#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_halium_sidecar_runtime_dev_surface.sh"

test -x "$SCRIPT"
grep -q 'trillionnium_world_halium_sidecar_runtime_dev_surface_v1' "$SCRIPT"
grep -q 'target-side-lxc' "$SCRIPT"
grep -q 'real-property-protocol-shim' "$SCRIPT"
grep -q 'HALIUM_ARM64_TOOLCHAIN_PAYLOAD' "$SCRIPT"
grep -q 'HALIUM_ALLOW_PAYLOAD_EXECUTION' "$SCRIPT"
grep -q 'payload_intake_gate' "$SCRIPT"
grep -q 'readonly_ro_setprop_rejected' "$SCRIPT"
grep -q 'operator_supplied_payload_intake_supported' "$SCRIPT"
grep -q 'TRILLIONNIUM_WORLD_HALIUM_SIDECAR_RUNTIME_DEV_SURFACE_GREEN' "$SCRIPT"

printf 'TRILLIONNIUM_WORLD_HALIUM_SIDECAR_RUNTIME_DEV_SURFACE_SCRIPT_CONTRACT_GUARD_OK %s\n' "$SCRIPT"
