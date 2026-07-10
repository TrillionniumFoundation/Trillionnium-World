#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_mirror_city_restoration.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_mirror_city_restoration_v1'
  'bevy-classic-rts-mirror-city-restoration.json'
  'bevy-classic-rts-mirror-city-restoration.ppm'
  'classic-rts-mirror-city-restoration'
  'input_path == "apply_live_native_action_with_source(classic_rts_mirror_city_restoration_input)"'
  'RTS:QUEUE:tier2:restore_city:mirror_city@13,3'
  'RTS:QUEUE:tier2:rebuild_core:signal_core@12,3'
  'RTS:QUEUE:tier2:assign_garrison:central_keep@13,3'
  'RTS:QUEUE:tier2:victory_handoff:mirror_city@13,3'
  'victory_dependency_gate == true'
  'restore_city_gate == true'
  'rebuild_core_gate == true'
  'garrison_gate == true'
  'handoff_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS restoration script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_MIRROR_CITY_RESTORATION_CONTRACT'
  'native_classic_rts_mirror_city_restoration_evidence_json'
  'classic-rts-mirror-city-restoration'
  'classic_rts_mirror_city_restoration_input'
  'rts_restored_zone_ids'
  'rts_rebuild_structure_ids'
  'rts_garrison_unit_ids'
  'rts_victory_handoff_state'
  'CLASSIC_RTS_RESTORE_ZONE_COLOR'
  'CLASSIC_RTS_REBUILD_CORE_COLOR'
  'CLASSIC_RTS_GARRISON_COLOR'
  'CLASSIC_RTS_HANDOFF_COLOR'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS restoration source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_mirror_city_restoration.sh'
  'bevy-classic-rts-mirror-city-restoration.json'
  'classic_rts_mirror_city_restoration_green'
  'rts_mirror_city_restoration_live_input_gate'
  'rts_mirror_city_restoration_victory_dependency_gate'
  'rts_mirror_city_restoration_restore_gate'
  'rts_mirror_city_restoration_rebuild_gate'
  'rts_mirror_city_restoration_garrison_gate'
  'rts_mirror_city_restoration_handoff_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS restoration readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS mirror city restoration evidence remains connected to victory dependency, restore, rebuild, garrison, handoff, and readiness"
