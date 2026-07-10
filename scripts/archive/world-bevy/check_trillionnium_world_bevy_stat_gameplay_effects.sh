#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-stat-gameplay-effects.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- stat-gameplay-effects >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_stat_gameplay_effects_v1"
  and .stat_allocation_contract == "trillionnium_world_bevy_stat_allocation_v1"
  and .stat_allocation_persistence_contract == "trillionnium_world_bevy_stat_allocation_persistence_v1"
  and .green == true
  and .event_acceptance_gate == true
  and .force_damage_gate == true
  and .agility_mitigation_gate == true
  and .craft_reward_gate == true
  and .branch_isolation_gate == true
  and .baseline_rematch.enemy_hp == 38
  and .force_rematch.enemy_hp == 34
  and .force_rematch.player_hp == 94
  and .agility_rematch.enemy_hp == 38
  and .baseline_rematch.player_hp == 94
  and .agility_rematch.player_hp == 97
  and .baseline_reward.coins == 10
  and .baseline_reward.xp == 40
  and .craft_reward.coins == 14
  and .craft_reward.xp == 45
  and (.force_rematch.growth_history | index("stat_effect:force:attack_damage_bonus")) != null
  and (.agility_rematch.growth_history | index("stat_effect:agility:incoming_damage_reduced")) != null
  and (.craft_reward.growth_history | index("stat_effect:craft:task_reward_bonus")) != null
  and .android_s5_real_device_claimed == false
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_STAT_GAMEPLAY_EFFECTS_GREEN %s\n' "$SUMMARY"
