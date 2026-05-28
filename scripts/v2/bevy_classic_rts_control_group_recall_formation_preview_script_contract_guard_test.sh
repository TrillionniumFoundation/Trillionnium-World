#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_control_group_recall_formation_preview.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
RELEASE="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_control_group_recall_formation_preview_v1'
  'bevy-classic-rts-control-group-recall-formation-preview.json'
  'bevy-classic-rts-control-group-recall-formation-preview.ppm'
  'classic-rts-control-group-recall-formation-preview'
  'recall_hud_gate == true'
  'formation_anchor_gate == true'
  'queued_member_gate == true'
  'filtered_member_gate == true'
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTROL_GROUP_RECALL_FORMATION_PREVIEW_GREEN'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS control group recall formation preview script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CONTROL_GROUP_RECALL_FORMATION_PREVIEW_CONTRACT'
  'native_classic_rts_control_group_recall_formation_preview_evidence_json'
  'classic_draw_rts_control_group_recall_formation_preview_overlay'
  'CLASSIC_RTS_RECALL_FORMATION_HUD_COLOR'
  'CLASSIC_RTS_RECALL_FORMATION_ANCHOR_COLOR'
  'CLASSIC_RTS_RECALL_FORMATION_FILTERED_COLOR'
  'Original Trillionnium control-group recall formation preview overlays'
  'classic-rts-control-group-recall-formation-preview'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS control group recall formation preview source line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'trillionnium_world_bevy_classic_rts_control_group_recall_formation_preview_v1'
  'bevy_classic_rts_control_group_recall_formation_preview_contract_guard'
  'bevy_classic_rts_control_group_recall_formation_preview_gate'
  'bevy_classic_rts_control_group_recall_formation_preview_script_contract_guard_test.sh'
  'check_trillionnium_world_bevy_classic_rts_control_group_recall_formation_preview.sh'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$RELEASE"; then
    echo "[FAIL] missing classic RTS control group recall formation preview release-review line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS control group recall formation preview evidence remains connected to renderer, CLI, release-review, group-28 recall focus, formation slot markers, member filtering, and original art policy"
