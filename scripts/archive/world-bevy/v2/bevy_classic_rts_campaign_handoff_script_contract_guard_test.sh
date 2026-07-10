#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_bevy_classic_rts_campaign_handoff.sh"
SOURCE="$ROOT/trillionnium/crates/trnm-world-bevy/src/lib.rs"
MAIN="$ROOT/trillionnium/crates/trnm-world-bevy/src/main.rs"
READINESS="$ROOT/scripts/check_trillionnium_world_bevy_classic_playtest_readiness.sh"

required_script_lines=(
  'trillionnium_world_bevy_classic_rts_campaign_handoff_v1'
  'bevy-classic-rts-campaign-handoff.json'
  'bevy-classic-rts-campaign-handoff.ppm'
  'classic-rts-campaign-handoff'
  'input_path == "apply_live_native_action_with_source(classic_rts_campaign_handoff_input)"'
  'RTS:QUEUE:objective:extract:relay_beacon@9,2'
  'RTS:QUEUE:camp:clear:forest_creep_camp@8,3'
  'RTS:QUEUE:recon:mark:enemy_base@10,2'
  'RTS:QUEUE:counter:fortify:watch_tower@7,4'
  'RTS:QUEUE:aftermath:next:secure_expansion@9,2'
  'RTS:QUEUE:commander:ability:rally_aura@mirror_captain'
  'RTS:QUEUE:expansion:defend:counter_wave@8,3'
  'RTS:QUEUE:tier2:push:gate_bulwark@10,3'
  'RTS:QUEUE:tier2:finish:gate_bulwark@10,3'
  'RTS:QUEUE:tier2:inner_secure:signal_core@12,3'
  'RTS:QUEUE:tier2:keep_claim:central_keep@13,3'
  'RTS:QUEUE:tier2:victory_handoff:mirror_city@13,3'
  'RTS:QUEUE:tier2:open_world_resume:league-coliseum@12,3'
  'early_campaign_gate == true'
  'mid_campaign_gate == true'
  'end_campaign_gate == true'
  'open_world_resume_gate == true'
  'snapshot_round_trip_gate == true'
  'render_milestone_gate == true'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq "$line" "$SCRIPT"; then
    echo "[FAIL] missing classic RTS campaign handoff script line: $line" >&2
    exit 1
  fi
done

required_source_lines=(
  'TRILLIONNIUM_WORLD_BEVY_CLASSIC_RTS_CAMPAIGN_HANDOFF_CONTRACT'
  'native_classic_rts_campaign_handoff_evidence_json'
  'classic-rts-campaign-handoff'
  'classic_rts_campaign_handoff_input'
  'objective_victory_seen'
  'creep_camp_seen'
  'recon_seen'
  'enemy_pressure_seen'
  'army_rally_seen'
  'base_assault_seen'
  'aftermath_seen'
  'commander_seen'
  'expansion_seen'
  'tier_two_seen'
  'breach_seen'
  'inner_seen'
  'keep_pressure_seen'
  'keep_victory_seen'
  'restoration_seen'
  'open_world_seen'
  'snapshot_round_trip_gate'
  'native_playable_save_snapshot'
  'native_restore_playable_save_snapshot'
)

for line in "${required_source_lines[@]}"; do
  if ! grep -Fq "$line" "$SOURCE" "$MAIN"; then
    echo "[FAIL] missing classic RTS campaign handoff source line: $line" >&2
    exit 1
  fi
done

required_readiness_lines=(
  'check_trillionnium_world_bevy_classic_rts_campaign_handoff.sh'
  'bevy-classic-rts-campaign-handoff.json'
  'classic_rts_campaign_handoff_green'
  'rts_campaign_handoff_live_input_gate'
  'rts_campaign_handoff_early_campaign_gate'
  'rts_campaign_handoff_mid_campaign_gate'
  'rts_campaign_handoff_end_campaign_gate'
  'rts_campaign_handoff_open_world_resume_gate'
  'rts_campaign_handoff_snapshot_round_trip_gate'
  'rts_campaign_handoff_render_milestone_gate'
)

for line in "${required_readiness_lines[@]}"; do
  if ! grep -Fq "$line" "$READINESS"; then
    echo "[FAIL] missing classic RTS campaign handoff readiness line: $line" >&2
    exit 1
  fi
done

echo "[PASS] classic RTS campaign handoff evidence remains connected to the full live-input RTS-to-open-world chain"
