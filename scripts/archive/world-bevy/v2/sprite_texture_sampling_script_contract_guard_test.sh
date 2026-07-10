#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_sprite_texture_sampling.sh"
RUST="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"

required_lines=(
  'trillionnium_world_bevy_sprite_texture_sampling_v1'
  'sprite_texture_sampling_green'
  'bevy-sprite-texture-sampling.json'
  'sprite-texture-sampling'
  'runtime-texture-sprite-sampling'
  'check_trillionnium_world_bevy_sprite_asset_binding.sh'
  'Assets<Image>'
  'Assets<TextureAtlasLayout>'
  'TextureAtlasLayout'
  'texture_rect'
  'image_rgba_sample_at'
  'sampled_rgba_values'
  'image_asset_resolve_gate'
  'texture_atlas_layout_asset_resolve_gate'
  'texture_atlas_rect_resolve_gate'
  'texture_sample_nonblank_gate'
  'four_layer_texture_sampling_gate'
  'global_unique_texture_color_gate'
  'ready_for_release_review'
  'scene_layer_count'
  'material_slot_count'
  'sampled_surface_sample_count'
  'sampled_layer_counts'
  'sampled_material_slot_counts'
  'scene_layer_count == (.scene_layers | length)'
  'material_slot_count == (.material_slots | length)'
  'sampled_surface_sample_count == (.sampled_surfaces_sample | length)'
  'sampled_surface_count >= 24'
  'TRNM_WORLD_BEVY_SPRITE_TEXTURE_SAMPLING'
  'bevy_assets_image_texture_atlas_cpu_sampling_not_gpu_upload_claim'
  'external_evidence_ignored_for_current_sprite_texture_pass'
  'gpu_upload_claimed == false'
  'android_s5_real_device_claimed == false'
  'public_launch_ready == false'
  'production_ready_ui_claimed == false'
  'screen_for_screen_openra_ui_claimed == false'
  'openra_engine_port_claimed == false'
  'third_party_asset_copied == false'
  'live_osm_ingestion_claimed == false'
)

for line in "${required_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT" "$RUST" "$MAIN"; then
    echo "[FAIL] sprite texture sampling contract missing line: $line" >&2
    exit 1
  fi
done

echo "[PASS] sprite texture sampling resolves Bevy Image and TextureAtlasLayout atlas pixels without GPU/device claims"
