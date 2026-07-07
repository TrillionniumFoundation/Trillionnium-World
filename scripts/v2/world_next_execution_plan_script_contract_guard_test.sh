#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCRIPT="$ROOT/scripts/check_trillionnium_world_next_execution_plan.sh"
DOC="$ROOT/docs/development/trillionnium-world-next-execution-plan-v1.md"

required_script_lines=(
  'trillionnium_world_next_execution_plan_v1'
  'release_review_packet_integrity_green_with_public_launch_blockers'
  'next_execution_plan_green_with_public_launch_blockers'
  'public_launch_blockers_preserved'
  'whole_screen_first_contact_readability'
  'human_playtest_path'
  'real_external_evidence_collection'
  'do not shrink already-gated micro cues without a fresh screenshot-visible issue'
)

required_doc_lines=(
  'Whole-screen First Contact readability review'
  'Public launch state: blocked until real external evidence exists.'
  'Android S5 real-device state: unclaimed until device evidence is collected.'
  'Do not keep shrinking already-gated micro cues'
)

for line in "${required_script_lines[@]}"; do
  if ! grep -Fq -- "$line" "$SCRIPT"; then
    echo "[FAIL] next execution plan script missing contract line: $line" >&2
    exit 1
  fi
done

for line in "${required_doc_lines[@]}"; do
  if ! grep -Fq -- "$line" "$DOC"; then
    echo "[FAIL] next execution plan doc missing contract line: $line" >&2
    exit 1
  fi
done

echo "[PASS] next execution plan script/doc keep whole-screen product direction, public-launch blockers, and micro-cue restraint"
