#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_openra_engine_port_asset_parity.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
RELEASE_CI="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

test -x "$SCRIPT"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_openra_engine_port_asset_parity_v1'
  'bevy-classic-rts-openra-engine-port-asset-parity.json'
  'bevy-classic-rts-openra-engine-port-asset-parity.ppm'
  'classic-rts-openra-engine-port-asset-parity'
  'rust_reimplementation_of_openra_engine_foundation_owned_assets'
  'ported_engine_module_count >= 10'
  'pixel_parity.coverage == "full_classic_asset_manifest_frame_set"'
  'pixel_parity.sample_count == .asset_manifest.frame_count'
  'pixel_parity.manifest_frame_match_count == .asset_manifest.frame_count'
  'pixel_parity.sample_pixel_mismatch_count == 0'
  'openra_style_engine_foundation_claimed == true'
  'openra_engine_port_foundation_claimed == false'
  'openra_engine_port_claimed == false'
  'openra_full_engine_port_claimed == false'
  'openra_pixel_perfect_asset_parity_claimed == true'
  'openra_westwood_pixel_perfect_asset_parity_claimed == false'
  'openra_asset_copied == false'
  'openra_csharp_engine_code_copied == false'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_ENGINE_PORT_ASSET_PARITY_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing OpenRA engine port asset parity script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_OPENRA_ENGINE_PORT_ASSET_PARITY_CONTRACT'
  'native_classic_rts_openra_engine_port_asset_parity_evidence_json'
  'classic-rts-openra-engine-port-asset-parity'
  'ModData: owned mod manifest, package load order, rules/chrome source registry'
  'Ruleset: actor rules, prerequisites, production, weapons, terrain, and traits'
  'OrderManager: deterministic issue-order queue, validation, rejection, and replay hooks'
  'ChromeProvider: widget-root lookup, screen id binding, modal overlay routing'
  'SpriteSequence: frame id, facing set, fps, loop policy, and texture-atlas rects'
  'openra_engine_port_asset_parity_gate'
  'openra_engine_port_foundation_claimed'
  'openra_engine_port_claimed'
  'openra_full_engine_port_claimed'
  'openra_pixel_perfect_asset_parity_claimed'
  'trillionnium_owned_openra_compatible_asset_pack'
  'openra_style_engine_foundation_claimed'
  'full_classic_asset_manifest_frame_set'
  'manifest_frame_match_count'
  'manifest_frame_ids'
  'openra_csharp_engine_code_copied'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing OpenRA engine port asset parity source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_openra_engine_port_asset_parity.sh'
  'bevy-classic-rts-openra-engine-port-asset-parity.json'
  'bevy-classic-rts-openra-engine-port-asset-parity.ppm'
  'classic_rts_openra_engine_port_asset_parity_green'
  'rts_openra_engine_port_asset_parity_gate'
  'rts_openra_engine_port_asset_parity_module_count'
  'rts_openra_engine_port_asset_parity_claimed'
  'rts_openra_engine_port_asset_parity_pixel_mismatch_count'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing OpenRA engine port asset parity readiness line: $line" >&2
    exit 1
  fi
done

required_ci_lines=(
  'check_trillionnium_world_bevy_classic_rts_openra_engine_port_asset_parity.sh'
  'bevy_classic_rts_openra_engine_port_asset_parity_script_contract_guard_test.sh'
)

for line in "${required_ci_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE_CI"; then
    echo "[FAIL] missing OpenRA engine port asset parity release CI line: $line" >&2
    exit 1
  fi
done

echo "[PASS] OpenRA engine port asset parity gate remains connected to Rust CLI, readiness, release CI, and no-copy boundaries"
