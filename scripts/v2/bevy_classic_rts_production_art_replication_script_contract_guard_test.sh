#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_production_art_replication.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
RELEASE_CI="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

test -x "$SCRIPT"

required_script_lines=(
  'classic-rts-production-art-replication'
  'bevy-classic-rts-production-art-replication.json'
  'bevy-classic-rts-production-art-replication.ppm'
  'SUMMARY_RAW="$(mktemp'
  'SUMMARY_TMP="$(mktemp'
  'trillionnium_world_bevy_classic_rts_production_art_replication_v1'
  'source_contract_count'
  'required_asset_kind_count'
  'required_gameplay_layer_count'
  'required_replacement_slot_count'
  'first_contact_production_art_pack_id == "first_contact_production_art_pack_v3"'
  'first_contact_production_pack_family_count == 6'
  'first_contact_production_pack_v2_feature_count == 5'
  'first_contact_production_pack_v3_feature_count == 6'
  'first_contact_production_art_pack_pixel_counts.terrain_material > 600'
  'first_contact_production_art_pack_pixel_counts.unit_sprite_skin > 600'
  'first_contact_production_art_pack_v2_pixel_counts.terrain_texture > 100'
  'first_contact_production_art_pack_v2_gate == true'
  'first_contact_production_art_pack_v3_pixel_counts.terrain_cluster > 100'
  'first_contact_production_art_pack_v3_gate == true'
  'first_contact_production_art_pack_gate == true'
  'production_art_replication_gate == true'
  'gate_count == 6'
  'passed_gate_count == 6'
  'failed_gate_count == 0'
  'no_copy_boundary_gate == true'
  'warcraft_iii_asset_copied == false'
  'openra_asset_copied == false'
  'third_party_asset_copied == false'
  'final_external_bitmap_art_shipped == false'
  'production_ready_art_shipped == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_PRODUCTION_ART_REPLICATION_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing production art replication script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_PRODUCTION_ART_REPLICATION_CONTRACT'
  'native_classic_rts_production_art_replication_evidence_json'
  'PRODUCTION ART REPLICATION BOARD'
  'required_asset_kinds'
  'required_replacement_slots'
  'authored_replacement_slot_gate'
  'production_preview_gate'
  'first_contact_production_art_pack_gate'
  'first_contact_production_art_pack_v2_gate'
  'first_contact_production_art_pack_v3_gate'
  'first_contact_production_art_pack_pixel_counts'
  'first_contact_production_art_pack_v2_pixel_counts'
  'first_contact_production_art_pack_v3_pixel_counts'
  'production_art_replication_gate'
  'no_copy_boundary_gate'
  'final_external_bitmap_art_shipped'
  'production_ready_art_shipped'
  'warcraft_iii_asset_copied'
  'openra_asset_copied'
  'third_party_asset_copied'
  'original Trillionnium-owned replacement slots'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing production art replication source line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'check_trillionnium_world_bevy_classic_rts_production_art_replication.sh'
  'bevy_classic_rts_production_art_replication_script_contract_guard_test.sh'
  'bevy_classic_rts_production_art_replication_gate'
  'trillionnium_world_bevy_classic_rts_production_art_replication_v1'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE_CI"; then
    echo "[FAIL] missing production art replication release CI line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS production art replication gate remains connected to Rust CLI, no-copy policy, and release-review CI"
