#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
visual="$repo_root/playtests/first_contact-visual-review.yaml"
session="$repo_root/playtests/first-contact-human-play-session.yaml"
acceptance="$repo_root/playtests/first_contact-closed-loop-acceptance.yaml"
runbook="$repo_root/docs/development/trnm-first-contact-human-validation-2026-07-11.md"

for required in "$visual" "$session" "$acceptance" "$runbook"; do
  test -s "$required" || { echo "missing validation input: $required" >&2; exit 1; }
done

test "$(grep -c 'observer: null' "$visual")" -eq 3
grep -q '^status: pending_real_observers$' "$visual"
grep -q '^status: pending_real_player$' "$session"
grep -q 'automation_counts_as_human: false' "$visual"
grep -q 'automation_counts_as_human: false' "$session"
grep -q 'status: pending_real_humans' "$acceptance"

echo 'TRNM human validation packet: ready'
echo 'Human evidence: pending (3 independent observers + 1 real 10-15 minute session)'
