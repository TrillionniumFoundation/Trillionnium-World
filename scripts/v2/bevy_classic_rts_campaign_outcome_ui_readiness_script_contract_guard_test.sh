#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"
CI="$ROOT/scripts/check_trillionnium_world_release_review_ci_gate.sh"
PACKET="$ROOT/scripts/check_trillionnium_world_release_review_packet.sh"
INTEGRITY="$ROOT/scripts/check_trillionnium_world_release_review_packet_integrity.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness_v1'
  'classic_rts_campaign_outcome_ui_readiness_green'
  'bevy-classic-rts-campaign-outcome-ui-readiness.json'
  'bevy-classic-rts-campaign-outcome-ui-readiness'
  'classic-rts-campaign-outcome-ui-readiness'
  'first-minute-readiness.ppm'
  'objective-victory-loop.ppm'
  'base-assault-resolution.ppm'
  'battle-aftermath.ppm'
  'open-world-after-action.ppm'
  'preview_count == 5'
  'runtime_screen_mode == "player_runtime_campaign_outcome_screen"'
  'runtime_screen_gate == true'
  'evidence_board_only == false'
  'title to victory to aftermath to open-world resume'
  'relay beacon extracted victory and defeat-risk summary'
  'league-coliseum arena_outdoor resume state'
  'first_minute_gate == true'
  'objective_victory_gate == true'
  'base_assault_gate == true'
  'battle_aftermath_gate == true'
  'open_world_return_gate == true'
  'native_boundary_gate == true'
  'preview_gate == true'
  'campaign_outcome_ui_readiness_gate == true'
  'internal_campaign_outcome_ui_readiness_claimed == true'
  'external_evidence_ignored_for_current_outcome_pass == true'
  'public_launch_ready == false'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS campaign outcome UI readiness script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CAMPAIGN_OUTCOME_UI_READINESS_CONTRACT'
  'native_classic_rts_campaign_outcome_ui_readiness_evidence_json'
  'classic-rts-campaign-outcome-ui-readiness'
  'classic_rts_campaign_outcome_ui_readiness_green'
  'player_runtime_campaign_outcome_screen'
  'runtime_screen_layout'
  'campaign_outcome_ui_readiness_gate'
  'first-minute-readiness.ppm'
  'objective-victory-loop.ppm'
  'base-assault-resolution.ppm'
  'battle-aftermath.ppm'
  'open-world-after-action.ppm'
  'TITLE campaign entry'
  'objective claim/extract victory'
  'battle aftermath rewards'
  'open-world route resume'
  'internal_campaign_outcome_ui_readiness_claimed'
  'external_evidence_ignored_for_current_outcome_pass'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS campaign outcome UI readiness source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_campaign_outcome_ui_readiness.sh'
  'bevy-classic-rts-campaign-outcome-ui-readiness.json'
  'classic_rts_campaign_outcome_ui_readiness_green'
  'rts_campaign_outcome_ui_readiness'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS campaign outcome UI readiness readiness line: $line" >&2
    exit 1
  fi
done

required_release_lines=(
  'bevy_classic_rts_campaign_outcome_ui_readiness_contract_guard'
  'bevy_classic_rts_campaign_outcome_ui_readiness_gate'
  'native_bevy_classic_rts_campaign_outcome_ui_readiness'
  'campaign_outcome_ui_readiness_semantics'
  'fake_packet_artifact_count == 113'
)

for line in "${required_release_lines[@]}"; do
  if ! grep -Fq "$line" "$CI" "$PACKET" "$INTEGRITY"; then
    echo "[FAIL] missing classic RTS campaign outcome UI readiness release line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS campaign outcome UI readiness remains connected to Bevy-native first-minute, victory, base-assault, aftermath, open-world resume, CI, packet, and integrity semantics"
