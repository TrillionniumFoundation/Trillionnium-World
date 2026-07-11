#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ACCEPTANCE_DIR="$ROOT/acceptance/S6_public_launch/latest"
DOC_REL="docs/archive/world-review-2026-07/trillionnium-world-public-launch-blocker-execution-ledger-2026-07-07.md"
DOC="$ROOT/$DOC_REL"
SUMMARY="$ACCEPTANCE_DIR/trillionnium-world-public-launch-blocker-execution-ledger.json"
SUMMARY_MD="$ACCEPTANCE_DIR/trillionnium-world-public-launch-blocker-execution-ledger.md"
READINESS_JSON="$ACCEPTANCE_DIR/public-launch-readiness.json"
INTAKE_JSON="$ACCEPTANCE_DIR/public-launch-evidence-intake.json"
CONSISTENCY_JSON="$ACCEPTANCE_DIR/public-launch-blocker-consistency.json"
mkdir -p "$ACCEPTANCE_DIR"

require_text() {
  local path="$1"
  local needle="$2"
  if ! grep -Fq -- "$needle" "$path"; then
    echo "[FAIL] $path missing required text: $needle" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "[FAIL] missing public launch blocker execution ledger doc: $DOC" >&2
  exit 1
fi

require_text "$DOC" "Status: local blocker execution ledger."
require_text "$DOC" "This consumes existing readiness, evidence-intake, and blocker-consistency"
require_text "$DOC" "Do not use templates, status-only files, host-side screenshots"
require_text "$DOC" "Do not run live map ingestion, public network exposure, Android device"
require_text "$DOC" '| `s5_real_device_matrix` |'
require_text "$DOC" '| `production_map_pack_public_evidence` |'
require_text "$DOC" '| `first_beta_cohort_evidence` |'
require_text "$DOC" '| `commercial_launch_drill_evidence` |'
require_text "$DOC" '| `multi_node_or_live_traffic_latency_evidence` |'
require_text "$DOC" '| `public_network_live_exposure_evidence` |'

"$ROOT/scripts/check_trillionnium_world_public_launch_evidence_intake.sh" >/dev/null
"$ROOT/scripts/check_trillionnium_world_public_launch_blocker_consistency.sh" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_public_launch_evidence_intake_v1"
  and .status == "public_launch_evidence_intake_ready_for_operator_collection"
  and .green == true
  and .public_launch_ready == false
  and .public_launch_claimed == false
  and .android_s5_real_device_claimed == false
  and .live_map_ingestion_performed == false
  and .live_public_exposure_performed == false
  and .blocker_count == 6
  and .evidence_item_count == 6
  and .needs_collection_count == 6
  and .green_evidence_item_count == 0
' "$INTAKE_JSON" >/dev/null

jq -e '
  .contract_version == "trillionnium_world_public_launch_blocker_consistency_v1"
  and .status == "public_launch_blocker_consistency_green_with_public_launch_blockers"
  and .green == true
  and .public_launch_ready == false
  and .known_blocker_count == 6
  and .readiness_blocker_count == 6
  and .intake_needs_collection_count == 6
  and .unknown_readiness_blocker_count == 0
  and .unknown_intake_blocker_count == 0
  and .failed_check_count == 0
' "$CONSISTENCY_JSON" >/dev/null

jq -e '
  .public_launch_ready == false
  and .android_s5_real_device_claimed == false
  and ((.known_public_launch_blockers // []) | length) == 6
' "$READINESS_JSON" >/dev/null

entries_json="$(jq -c --slurpfile consistency "$CONSISTENCY_JSON" '
  .evidence_items as $items
  | $consistency[0] as $consistency_doc
  | [
      $items[] as $item
      | $item + {
          execution_status: (if $item.green == true then "external_evidence_green" else "blocked_until_real_evidence_attached" end),
          validator_present_check_status: (
            ($consistency_doc.checks[]? | select(.name == ($item.blocker_id + "_validator_present")) | .status) // "missing"
          ),
          consistency_check_status: (
            ($consistency_doc.checks[]? | select(.name == ($item.blocker_id + "_blocked_consistency")) | .status) // "missing"
          ),
          local_substitutes_rejected: true
        }
    ]
' "$INTAKE_JSON")"

blocker_count="$(jq -r '.blocker_count' "$INTAKE_JSON")"
evidence_item_count="$(jq -r '.evidence_item_count' "$INTAKE_JSON")"
needs_collection_count="$(jq -r '.needs_collection_count' "$INTAKE_JSON")"
green_evidence_item_count="$(jq -r '.green_evidence_item_count' "$INTAKE_JSON")"
blocked_evidence_item_count="$(jq -r '.blocked_evidence_item_count' "$INTAKE_JSON")"
present_evidence_item_count="$(jq -r '.present_evidence_item_count' "$INTAKE_JSON")"
missing_evidence_item_count="$(jq -r '.missing_evidence_item_count' "$INTAKE_JSON")"
consistency_failed_check_count="$(jq -r '.failed_check_count' "$CONSISTENCY_JSON")"
public_launch_ready="$(jq -r '.public_launch_ready' "$READINESS_JSON")"
android_s5_real_device_claimed="$(jq -r '.android_s5_real_device_claimed' "$READINESS_JSON")"
blockers_json="$(jq -c '.known_public_launch_blockers // []' "$READINESS_JSON")"

jq -n \
  --arg contract_version "trillionnium_world_public_launch_blocker_execution_ledger_v1" \
  --arg status "public_launch_blocker_execution_ledger_ready_for_real_evidence_collection" \
  --arg generated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg doc_path "$DOC_REL" \
  --arg readiness_path "acceptance/S6_public_launch/latest/public-launch-readiness.json" \
  --arg intake_path "acceptance/S6_public_launch/latest/public-launch-evidence-intake.json" \
  --arg consistency_path "acceptance/S6_public_launch/latest/public-launch-blocker-consistency.json" \
  --argjson public_launch_ready "$public_launch_ready" \
  --argjson android_s5_real_device_claimed "$android_s5_real_device_claimed" \
  --argjson blocker_count "$blocker_count" \
  --argjson blockers "$blockers_json" \
  --argjson evidence_item_count "$evidence_item_count" \
  --argjson needs_collection_count "$needs_collection_count" \
  --argjson green_evidence_item_count "$green_evidence_item_count" \
  --argjson blocked_evidence_item_count "$blocked_evidence_item_count" \
  --argjson present_evidence_item_count "$present_evidence_item_count" \
  --argjson missing_evidence_item_count "$missing_evidence_item_count" \
  --argjson consistency_failed_check_count "$consistency_failed_check_count" \
  --argjson blocker_entries "$entries_json" \
  '{
    contract_version: $contract_version,
    status: $status,
    green: true,
    generated_at: $generated_at,
    source_of_truth: "trillionnium_world_public_launch_blocker_execution_ledger",
    doc_path: $doc_path,
    source_inputs: {
      public_launch_readiness: $readiness_path,
      public_launch_evidence_intake: $intake_path,
      public_launch_blocker_consistency: $consistency_path
    },
    public_launch_ready: $public_launch_ready,
    android_s5_real_device_claimed: $android_s5_real_device_claimed,
    public_launch_claimed: false,
    beta_cohort_evidence_claimed: false,
    production_ready_ui_claimed: false,
    commercial_launch_evidence_claimed: false,
    multi_node_or_live_traffic_claimed: false,
    public_network_live_exposure_claimed: false,
    live_map_ingestion_performed: false,
    live_public_exposure_performed: false,
    android_device_capture_performed: false,
    local_substitutes_rejected: true,
    blocker_count: $blocker_count,
    blockers: $blockers,
    evidence_item_count: $evidence_item_count,
    needs_collection_count: $needs_collection_count,
    green_evidence_item_count: $green_evidence_item_count,
    blocked_evidence_item_count: $blocked_evidence_item_count,
    present_evidence_item_count: $present_evidence_item_count,
    missing_evidence_item_count: $missing_evidence_item_count,
    blocker_consistency_failed_check_count: $consistency_failed_check_count,
    blocker_entries: $blocker_entries,
    execution_rule: "each blocker clears only when the matching field-level validator accepts real non-template evidence",
    no_credit_boundary: "local blocker execution ledger only; no public launch, Android S5 real-device, beta, production-ready UI, commercial, multi-node, live-traffic, public-network, live-ingestion, or device-capture credit"
  }' >"$SUMMARY"

jq -e '
  .contract_version == "trillionnium_world_public_launch_blocker_execution_ledger_v1"
  and .status == "public_launch_blocker_execution_ledger_ready_for_real_evidence_collection"
  and .green == true
  and .public_launch_ready == false
  and .android_s5_real_device_claimed == false
  and .public_launch_claimed == false
  and .live_map_ingestion_performed == false
  and .live_public_exposure_performed == false
  and .android_device_capture_performed == false
  and .local_substitutes_rejected == true
  and .blocker_count == 6
  and .evidence_item_count == 6
  and .needs_collection_count == 6
  and .green_evidence_item_count == 0
  and .blocked_evidence_item_count == 6
  and .blocker_consistency_failed_check_count == 0
  and (.blocker_entries | length) == 6
  and all(.blocker_entries[]; .execution_status == "blocked_until_real_evidence_attached")
  and all(.blocker_entries[]; .validator_present_check_status == "ok")
  and all(.blocker_entries[]; .consistency_check_status == "ok")
  and all(.blocker_entries[]; .local_substitutes_rejected == true)
  and (.no_credit_boundary | contains("local blocker execution ledger only"))
' "$SUMMARY" >/dev/null

{
  printf '# Trillionnium World Public Launch Blocker Execution Ledger\n\n'
  printf -- '- status: `%s`\n' "$(jq -r '.status' "$SUMMARY")"
  printf -- '- public_launch_ready: `%s`\n' "$(jq -r '.public_launch_ready' "$SUMMARY")"
  printf -- '- Android S5 real-device claimed: `%s`\n' "$(jq -r '.android_s5_real_device_claimed' "$SUMMARY")"
  printf -- '- blockers / needs collection / green evidence: `%s` / `%s` / `%s`\n' \
    "$(jq -r '.blocker_count' "$SUMMARY")" \
    "$(jq -r '.needs_collection_count' "$SUMMARY")" \
    "$(jq -r '.green_evidence_item_count' "$SUMMARY")"
  printf -- '- blocker-consistency failed checks: `%s`\n' "$(jq -r '.blocker_consistency_failed_check_count' "$SUMMARY")"
  printf -- '- live map ingestion / public exposure / device capture performed: `%s` / `%s` / `%s`\n\n' \
    "$(jq -r '.live_map_ingestion_performed' "$SUMMARY")" \
    "$(jq -r '.live_public_exposure_performed' "$SUMMARY")" \
    "$(jq -r '.android_device_capture_performed' "$SUMMARY")"
  printf '## Execution Rows\n\n'
  jq -r '.blocker_entries[] | "- [ ] `\(.blocker_id)` / \(.label)\n  - current: `\(.current_status)` -> accepted: `\(.accepted_status)`\n  - evidence path: `\(.evidence_path // "n/a")` (`\(.file_status)`)\n  - env: `\(.evidence_env_var // "n/a")`\n  - collect: `\(.collection_command)`\n  - requirement: \(.collection_requirement)\n  - consistency: validator `\(.validator_present_check_status)`, blocker `\(.consistency_check_status)`\n"' "$SUMMARY"
  printf '## Boundary\n\n'
  printf -- '- This ledger performs no external action and grants no launch credit.\n'
  printf -- '- Templates, status-only files, local drills, and host-side evidence remain rejected as substitutes for real external evidence.\n'
} >"$SUMMARY_MD"

printf 'TRILLIONNIUM_WORLD_PUBLIC_LAUNCH_BLOCKER_EXECUTION_LEDGER_READY %s %s\n' "$SUMMARY" "$SUMMARY_MD"
