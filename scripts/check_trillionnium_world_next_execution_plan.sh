#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
DOC="$ROOT/docs/development/trillionnium-world-next-execution-plan-v1.md"
PACKET_JSON="$ACCEPTANCE_DIR/release-review-packet-integrity.json"
PUBLIC_LAUNCH_JSON="$ACCEPTANCE_DIR/public-launch-readiness.json"
RUNNER_JSON="$ROOT/acceptance/S5_native_bevy_device/latest/bevy-classic-playtest-runner-status.json"
SUMMARY_JSON="$ACCEPTANCE_DIR/trillionnium-world-next-execution-plan.json"
SUMMARY_MD="$ACCEPTANCE_DIR/trillionnium-world-next-execution-plan.md"

if [[ -v TRILLIONNIUM_WORLD_NEXT_EXECUTION_PLAN_SUMMARY && -n "$TRILLIONNIUM_WORLD_NEXT_EXECUTION_PLAN_SUMMARY" ]]; then
  SUMMARY_JSON="$TRILLIONNIUM_WORLD_NEXT_EXECUTION_PLAN_SUMMARY"
fi

mkdir -p "$ACCEPTANCE_DIR"

require_file() {
  local path="$1"
  if [[ ! -f "$path" ]]; then
    echo "[FAIL] missing required file: $path" >&2
    exit 1
  fi
}

require_text() {
  local path="$1"
  local needle="$2"
  if ! grep -Fq -- "$needle" "$path"; then
    echo "[FAIL] $path missing required text: $needle" >&2
    exit 1
  fi
}

require_file "$DOC"
require_file "$PACKET_JSON"
require_file "$PUBLIC_LAUNCH_JSON"
require_file "$RUNNER_JSON"

require_text "$DOC" "Whole-screen First Contact readability review"
require_text "$DOC" "Local review state: green with public-launch blockers."
require_text "$DOC" "Public launch state: blocked until real external evidence exists."
require_text "$DOC" "Do not keep shrinking already-gated micro cues"

packet_status="$(jq -r '.status // "missing"' "$PACKET_JSON")"
packet_green="$(jq -r '.green // false' "$PACKET_JSON")"
packet_artifact_count="$(jq -r '.artifact_count // 0' "$PACKET_JSON")"
packet_failed_check_count="$(jq -r '.failed_check_count // 999' "$PACKET_JSON")"
packet_check_count="$(jq -r '(.checks // []) | length' "$PACKET_JSON")"

runner_green="$(jq -r '.green // false' "$RUNNER_JSON")"
runner_gate_count="$(jq -r '.gate_count // 0' "$RUNNER_JSON")"
runner_failed_gate_count="$(jq -r '.failed_gate_count // 999' "$RUNNER_JSON")"
runner_pid="$(jq -r '.service.main_pid // "unknown"' "$RUNNER_JSON")"
runner_screenshot_path="$(jq -r '.live_player_screen.screenshot_path // ""' "$RUNNER_JSON")"

public_launch_ready="$(jq -r '.public_launch_ready // false' "$PUBLIC_LAUNCH_JSON")"
android_s5_real_device_claimed="$(jq -r '.android_s5_real_device_claimed // false' "$PUBLIC_LAUNCH_JSON")"
blocker_count="$(jq -r '.known_public_launch_blocker_count // ((.known_public_launch_blockers // []) | length)' "$PUBLIC_LAUNCH_JSON")"
blockers_json="$(jq -c '.known_public_launch_blockers // []' "$PUBLIC_LAUNCH_JSON")"

head_commit="$(git -C "$ROOT" rev-parse HEAD)"
origin_commit="$(git -C "$ROOT" rev-parse origin/main)"
ahead_count="$(git -C "$ROOT" rev-list --count origin/main..HEAD)"
dirty_count="$(git -C "$ROOT" status --porcelain | wc -l | tr -d ' ')"
s5_acceptance_kib="$(du -sk "$ROOT/acceptance/S5_native_bevy_device/latest" | awk '{print $1}')"

packet_gate=false
if [[ "$packet_green" == "true" && "$packet_status" == "release_review_packet_integrity_green_with_public_launch_blockers" && "$packet_artifact_count" -ge 128 && "$packet_failed_check_count" -eq 0 ]]; then
  packet_gate=true
fi

runner_gate=false
if [[ "$runner_green" == "true" && "$runner_gate_count" -ge 21 && "$runner_failed_gate_count" -eq 0 ]]; then
  runner_gate=true
fi

public_launch_blocker_gate=false
if [[ "$public_launch_ready" == "false" && "$android_s5_real_device_claimed" == "false" && "$blocker_count" -eq 6 ]]; then
  public_launch_blocker_gate=true
fi

green=false
status="next_execution_plan_blocked"
if [[ "$packet_gate" == "true" && "$runner_gate" == "true" && "$public_launch_blocker_gate" == "true" ]]; then
  green=true
  status="next_execution_plan_green_with_public_launch_blockers"
fi

risks_json="$(jq -nc '[
  {
    id: "local_commit_backlog",
    severity: "high",
    status: "active",
    next_action: "group local commits into reviewable slices before any external push"
  },
  {
    id: "external_public_launch_evidence_gap",
    severity: "blocking",
    status: "blocked_on_real_evidence",
    next_action: "collect the six real external evidence bundles without granting template or host-side credit"
  },
  {
    id: "documentation_truth_source_drift",
    severity: "medium",
    status: "mitigating",
    next_action: "keep README, RELEASE_READINESS, and development docs synchronized with packet artifacts"
  },
  {
    id: "acceptance_evidence_volume",
    severity: "medium",
    status: "active",
    next_action: "curate large S5/Bevy evidence before handoff"
  },
  {
    id: "first_contact_central_battlefield_readability",
    severity: "high",
    status: "active",
    next_action: "shift from isolated micro-cue shaving to whole-screen product readability"
  }
]')"

work_queue_json="$(jq -nc '[
  {
    id: "whole_screen_first_contact_readability",
    priority: 1,
    scope: "local_product_quality",
    done_when: "unit silhouettes, building hierarchy, terrain grouping, objective focus, and combat flow are readable in the live player screen"
  },
  {
    id: "human_playtest_path",
    priority: 2,
    scope: "local_playtest",
    done_when: "a tester can start campaign, select units, secure beacon, read command queue, and recover from blocked route"
  },
  {
    id: "truth_source_hygiene",
    priority: 3,
    scope: "local_docs_and_guards",
    done_when: "artifact counts, readiness dates, and no-claim boundaries remain synchronized"
  },
  {
    id: "review_slice_strategy",
    priority: 4,
    scope: "repository_hygiene",
    done_when: "local backlog is grouped into reviewable slices without changing public/external state"
  },
  {
    id: "real_external_evidence_collection",
    priority: 5,
    scope: "external_evidence",
    done_when: "all six public-launch evidence validators pass on real non-template artifacts"
  }
]')"

jq -n \
  --arg contract_version "trillionnium_world_next_execution_plan_v1" \
  --arg status "$status" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg head_commit "$head_commit" \
  --arg origin_commit "$origin_commit" \
  --arg packet_status "$packet_status" \
  --arg runner_pid "$runner_pid" \
  --arg runner_screenshot_path "$runner_screenshot_path" \
  --argjson green "$green" \
  --argjson packet_gate "$packet_gate" \
  --argjson packet_artifact_count "$packet_artifact_count" \
  --argjson packet_failed_check_count "$packet_failed_check_count" \
  --argjson packet_check_count "$packet_check_count" \
  --argjson runner_gate "$runner_gate" \
  --argjson runner_gate_count "$runner_gate_count" \
  --argjson runner_failed_gate_count "$runner_failed_gate_count" \
  --argjson public_launch_blocker_gate "$public_launch_blocker_gate" \
  --argjson public_launch_ready "$public_launch_ready" \
  --argjson android_s5_real_device_claimed "$android_s5_real_device_claimed" \
  --argjson blocker_count "$blocker_count" \
  --argjson blockers "$blockers_json" \
  --argjson ahead_count "$ahead_count" \
  --argjson dirty_count "$dirty_count" \
  --argjson s5_acceptance_kib "$s5_acceptance_kib" \
  --argjson risks "$risks_json" \
  --argjson work_queue "$work_queue_json" \
  '{
    contract_version: $contract_version,
    status: $status,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_next_execution_plan",
    green: $green,
    gates: {
      release_review_packet_integrity: $packet_gate,
      live_runner: $runner_gate,
      public_launch_blockers_preserved: $public_launch_blocker_gate
    },
    repository: {
      head_commit: $head_commit,
      origin_main_commit: $origin_commit,
      ahead_count: $ahead_count,
      dirty_count_at_generation: $dirty_count
    },
    release_review_packet: {
      status: $packet_status,
      artifact_count: $packet_artifact_count,
      failed_check_count: $packet_failed_check_count,
      check_count: $packet_check_count
    },
    runner: {
      main_pid: $runner_pid,
      gate_count: $runner_gate_count,
      failed_gate_count: $runner_failed_gate_count,
      screenshot_path: $runner_screenshot_path
    },
    public_launch: {
      public_launch_ready: $public_launch_ready,
      android_s5_real_device_claimed: $android_s5_real_device_claimed,
      blocker_count: $blocker_count,
      blockers: $blockers
    },
    evidence_volume: {
      s5_native_bevy_latest_kib: $s5_acceptance_kib
    },
    risks: $risks,
    work_queue: $work_queue,
    operating_rule: "prefer whole-screen product quality and truth-source guards; do not shrink already-gated micro cues without a fresh screenshot-visible issue"
  }' >"$SUMMARY_JSON"

{
  printf '# Trillionnium World Next Execution Plan\n\n'
  printf -- '- status: `%s`\n' "$status"
  printf -- '- green: `%s`\n' "$green"
  printf -- '- local commits ahead of origin/main: `%s`\n' "$ahead_count"
  printf -- '- packet artifacts: `%s`, failed checks: `%s`\n' "$packet_artifact_count" "$packet_failed_check_count"
  printf -- '- public launch ready: `%s`\n' "$public_launch_ready"
  printf -- '- Android S5 real-device claimed: `%s`\n\n' "$android_s5_real_device_claimed"
  printf '## Risks\n\n'
  jq -r '.risks[] | "- `\(.id)`: \(.next_action)"' "$SUMMARY_JSON"
  printf '\n## Work Queue\n\n'
  jq -r '.work_queue[] | "- \(.priority). `\(.id)`: \(.done_when)"' "$SUMMARY_JSON"
  printf '\n## Public Launch Blockers\n\n'
  jq -r '.public_launch.blockers[] | "- `\(.)`"' "$SUMMARY_JSON"
} >"$SUMMARY_MD"

if [[ "$green" == "true" ]]; then
  printf 'TRILLIONNIUM_WORLD_NEXT_EXECUTION_PLAN_GREEN_WITH_PUBLIC_LAUNCH_BLOCKERS %s\n' "$SUMMARY_JSON"
else
  printf 'TRILLIONNIUM_WORLD_NEXT_EXECUTION_PLAN_BLOCKED %s\n' "$SUMMARY_JSON" >&2
  exit 1
fi
