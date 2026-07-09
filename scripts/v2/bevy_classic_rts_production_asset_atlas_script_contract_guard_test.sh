#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_production_asset_atlas.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
RELEASE_CI="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

test -x "$SCRIPT"

required_script_lines=(
  'classic-rts-production-asset-atlas'
  'bevy-classic-rts-production-asset-atlas.json'
  'bevy-classic-rts-production-asset-atlas.ppm'
  'SUMMARY_RAW="$(mktemp'
  'SUMMARY_TMP="$(mktemp'
  'trillionnium_world_bevy_classic_rts_production_asset_atlas_v1'
  'trillionnium_world_bevy_runtime_texture_asset_v1'
  'source_contract_count'
  'source_path_count'
  'atlas_family_name_count'
  'binding_replacement_slot_count'
  'binding_runtime_target_count'
  'runtime_material_slot_count'
  'runtime_scene_layer_count'
  'first_contact_production_art_pack_id == "first_contact_production_art_pack_v4"'
  'first_contact_pack_atlas_slot_count == 23'
  'first_contact_pack_v2_atlas_slot_count == 5'
  'first_contact_pack_v3_atlas_slot_count == 6'
  'first_contact_pack_v4_atlas_slot_count == 6'
  'first_contact_pack_atlas_slot_pixel_count > 9500'
  'first_contact_production_art_pack_atlas_gate == true'
  'production_asset_atlas_gate == true'
  'runtime_texture_asset_gate == true'
  'gate_count == 8'
  'passed_gate_count == 8'
  'failed_gate_count == 0'
  'no_copy_boundary_gate == true'
  'warcraft_iii_asset_copied == false'
  'openra_asset_copied == false'
  'third_party_asset_copied == false'
  'final_external_bitmap_art_shipped == false'
  'production_ready_art_shipped == false'
  'gpu_upload_claimed == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_PRODUCTION_ASSET_ATLAS_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing production asset atlas script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_PRODUCTION_ASSET_ATLAS_CONTRACT'
  'native_classic_rts_production_asset_atlas_evidence_json'
  'TRNM NATIVE PRODUCTION ASSET ATLAS'
  'native_runtime_texture_asset_evidence_json'
  'runtime_texture_asset_gate'
  'texture_atlas_binding_gate'
  'production_asset_atlas_preview_gate'
  'first_contact_production_art_pack_atlas_gate'
  'first_contact_pack_v2_atlas_slot_count'
  'first_contact_pack_v3_atlas_slot_count'
  'first_contact_pack_v4_atlas_slot_count'
  'first_contact_pack_atlas_slot_names'
  'production_asset_atlas_gate'
  'no_copy_boundary_gate'
  'final_external_bitmap_art_shipped'
  'production_ready_art_shipped'
  'warcraft_iii_asset_copied'
  'openra_asset_copied'
  'third_party_asset_copied'
  'original Trillionnium replacement slots'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing production asset atlas source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_production_asset_atlas.sh'
  'rts_production_asset_atlas'
  'classic_rts_production_asset_atlas_green'
  'bevy-classic-rts-production-asset-atlas.json'
  'rts_production_asset_atlas_first_contact_pack_slot_count'
  'rts_production_asset_atlas_first_contact_pack_slot_pixel_count'
  'rts_production_asset_atlas_first_contact_pack_atlas_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing production asset atlas readiness line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'check_trillionnium_world_bevy_classic_rts_production_asset_atlas.sh'
  'bevy_classic_rts_production_asset_atlas_script_contract_guard_test.sh'
  'bevy_classic_rts_production_asset_atlas_gate'
  'trillionnium_world_bevy_classic_rts_production_asset_atlas_v1'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE_CI"; then
    echo "[FAIL] missing production asset atlas release CI line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS production asset atlas gate remains connected to Rust CLI, runtime texture asset evidence, playtest readiness, no-copy policy, and release-review CI"
