#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EVIDENCE_DIR="$ROOT/acceptance/S5_native_bevy_device/latest"
SUMMARY="$EVIDENCE_DIR/bevy-quest-journal.json"
mkdir -p "$EVIDENCE_DIR"

(
  cd "$ROOT/trillionnium"
  cargo run -p trnm-world-bevy -- quest-journal >"$SUMMARY"
)

jq -e '
  .contract_version == "trillionnium_world_bevy_quest_journal_v1"
  and .input_affordance_contract == "trillionnium_world_bevy_input_affordance_feedback_v1"
  and .green == true
  and .event_acceptance_gate == true
  and .journal_presence_gate == true
  and .initial_journal_gate == true
  and .create_progress_gate == true
  and .train_progress_gate == true
  and .fight_progress_gate == true
  and .completion_progress_gate == true
  and .equip_progress_gate == true
  and .final_runtime_gate == true
  and .android_s5_real_device_claimed == false
  and (.quest_texts[0] | contains("QUEST JOURNAL | TRACKED:"))
  and (.quest_texts[0] | contains("NEXT: TITLE:NEW"))
  and (.quest_texts[0] | contains("[ ]CREATE"))
  and (.quest_texts[0] | contains("locked: task-arena-rematch-route"))
  and (.quest_texts[2] | contains("[x]CREATE"))
  and (.quest_texts[4] | contains("[x]TRAIN"))
  and (.quest_texts[6] | contains("[x]FIGHT"))
  and (.quest_texts[7] | contains("completed: task-fixture-first-route"))
  and (.quest_texts[7] | contains("claim claimed"))
  and (.quest_texts[8] | contains("[x]EQUIP"))
  and (.quest_texts[8] | contains("equipment ready"))
' "$SUMMARY" >/dev/null

printf 'TRILLIONNIUM_WORLD_BEVY_QUEST_JOURNAL_GREEN %s\n' "$SUMMARY"
