#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_ui_map_modeling_full_alignment.sh"

test -x "$SCRIPT"
grep -q 'trillionnium_world_ui_map_modeling_full_alignment_v1' "$SCRIPT"
grep -q 'host_side_ui_map_modeling_aligned_public_evidence_blocked' "$SCRIPT"
grep -q -- '--require-ready' "$SCRIPT"
grep -q 'TRILLIONNIUM_WORLD_UI_MAP_MODELING_FULL_ALIGNMENT_BLOCKED' "$SCRIPT"
grep -q 'production_map_pack_public_evidence' "$SCRIPT"
grep -q 's5_real_device_evidence' "$SCRIPT"
grep -q 'TRNM_WORLD_FULL_ALIGNMENT_REFRESH=0' "$SCRIPT"
grep -q 'no_live_overpass_or_geofabrik_ingestion_performed' "$SCRIPT"
grep -q 'fixture_map_modeling_is_not_production_public_map_pack' "$SCRIPT"
grep -q 'host_side_rendering_is_not_android_s5_real_device_evidence' "$SCRIPT"

printf 'TRILLIONNIUM_WORLD_UI_MAP_MODELING_FULL_ALIGNMENT_SCRIPT_CONTRACT_GUARD_OK %s\n' "$SCRIPT"
